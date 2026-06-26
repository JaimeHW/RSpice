#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.v[146] = 0.0;

        s.v[147] = 0.0;

        s.v[148] = 0.0;

        s.v[149] = 0.0;

        s.v[273] = 0.0;

        s.b[527] = (p.p12 == 1.0);
        s.v[527] = if s.b[527] { 1.0 } else { 0.0 };

        if s.b[527] {
            s.store_scalar(212, 1.0);
        }

        if (!s.b[527]) {
            s.store_scalar(212, (-1.0));
        }

        s.b[528] = (p.p13 == 1.0);
        s.v[528] = if s.b[528] { 1.0 } else { 0.0 };

        if s.b[528] {
            s.store_scalar(213, 1.0);
        }

        if (!s.b[528]) {
            s.store_scalar(213, (-1.0));
        }

        s.v[16] = (p.p59 * 8.85418e-12);

        s.b[529] = (p.p21 == 0.0);
        s.v[529] = if s.b[529] { 1.0 } else { 0.0 };

        if s.b[529] {
            s.store_scalar(5, (p.p1 / p.p2));
        }

        if (!s.b[529]) {
            s.store_scalar(5, p.p1);
        }

        s.v[0] = (p.p0 + p.p23);

        s.store_offset(5, 5, p.p24);

        s.v[6] = ((s.v[0]) as f64).powf((-p.p29));

        s.store_powf(7, 5, (-p.p30));

        s.store_scale(8, 7, s.v[6]);

        s.store_add_scaled_ad_lhs(9, A::scale_offset(s.ad_value(7), p.p27, (p.p25 + (p.p26 * s.v[6]))), 8, p.p28);

        s.v[10] = ((s.v[0]) as f64).powf((-p.p35));

        s.store_powf(11, 5, (-p.p36));

        s.store_scale(12, 11, s.v[10]);

        s.store_add_scaled_ad_lhs(13, A::scale_offset(s.ad_value(11), p.p33, (p.p31 + (p.p32 * s.v[10]))), 12, p.p34);

        s.store_sub_from_scalar_ad(2, s.v[0], A::scale(s.ad_value(9), 2.0));

        s.store_sub_scaled_inputs(3, 5, 1.0, 13, 2.0);

        s.store_add_scaled_ad_lhs(14, A::scale_offset(s.ad_value(7), p.p39, (p.p37 + (p.p38 * s.v[6]))), 8, p.p40);

        s.store_add_scaled_ad_lhs(15, A::scale_offset(s.ad_value(11), p.p43, (p.p41 + (p.p42 * s.v[10]))), 12, p.p44);

        s.store_sub_from_scalar_ad(1, s.v[0], A::scale(s.ad_value(14), 2.0));

        s.store_sub_scaled_inputs(4, 5, 1.0, 15, 2.0);

        s.store_div_from_scalar(278, 1e-6, 2);

        s.store_div_from_scalar(279, 1e-6, 3);

        s.store_mul(280, 278, 279);

        s.store_add_scaled_inputs3_offset(281, s.ad_value(278), p.p319, s.ad_value(279), p.p320, s.ad_value(280), p.p321, p.p191);

        s.store_add_scaled_inputs3_offset(282, s.ad_value(278), p.p325, s.ad_value(279), p.p326, s.ad_value(280), p.p327, p.p199);

        s.store_add_scaled_inputs3_offset(283, s.ad_value(278), p.p322, s.ad_value(279), p.p323, s.ad_value(280), p.p324, p.p195);

        s.store_add_scaled_inputs3_offset(284, s.ad_value(278), p.p328, s.ad_value(279), p.p329, s.ad_value(280), p.p330, p.p202);

        s.store_add_scaled_inputs3_offset(285, s.ad_value(278), p.p331, s.ad_value(279), p.p332, s.ad_value(280), p.p333, p.p203);

        s.store_add_scaled_inputs3_offset(286, s.ad_value(278), p.p334, s.ad_value(279), p.p335, s.ad_value(280), p.p336, p.p204);

        s.store_add_scaled_inputs3_offset(287, s.ad_value(278), p.p337, s.ad_value(279), p.p338, s.ad_value(280), p.p339, p.p57);

        s.store_add_scaled_inputs3_offset(288, s.ad_value(278), p.p340, s.ad_value(279), p.p341, s.ad_value(280), p.p342, p.p58);

        s.store_add_scaled_inputs3_offset(289, s.ad_value(278), p.p343, s.ad_value(279), p.p344, s.ad_value(280), p.p345, p.p51);

        s.store_add_scaled_inputs3_offset(290, s.ad_value(278), p.p346, s.ad_value(279), p.p347, s.ad_value(280), p.p348, p.p50);

        s.store_add_scaled_inputs3_offset(291, s.ad_value(278), p.p349, s.ad_value(279), p.p350, s.ad_value(280), p.p351, p.p63);

        s.store_add_scaled_inputs3_offset(292, s.ad_value(278), p.p352, s.ad_value(279), p.p353, s.ad_value(280), p.p354, p.p64);

        s.store_add_scaled_inputs3_offset(293, s.ad_value(278), p.p355, s.ad_value(279), p.p356, s.ad_value(280), p.p357, p.p65);

        s.store_add_scaled_inputs3_offset(294, s.ad_value(278), p.p358, s.ad_value(279), p.p359, s.ad_value(280), p.p360, p.p68);

        s.store_add_scaled_inputs3_offset(295, s.ad_value(278), p.p361, s.ad_value(279), p.p362, s.ad_value(280), p.p363, p.p276);

        s.store_add_scaled_inputs3_offset(250, s.ad_value(278), p.p751, s.ad_value(279), p.p752, s.ad_value(280), p.p753, p.p291);

        s.store_add_scaled_inputs3_offset(252, s.ad_value(278), p.p757, s.ad_value(279), p.p758, s.ad_value(280), p.p759, p.p294);

        s.store_add_scaled_inputs3_offset(251, s.ad_value(278), p.p754, s.ad_value(279), p.p755, s.ad_value(280), p.p756, p.p293);

        s.b[538] = (s.v[295] < 0.0);
        s.v[538] = if s.b[538] { 1.0 } else { 0.0 };

        if s.b[538] {
            s.store_scalar(295, 0.0);
        }

        s.b[539] = (s.v[295] > 1.0);
        s.v[539] = if s.b[539] { 1.0 } else { 0.0 };

        if ((!s.b[538]) && s.b[539]) {
            s.store_scalar(295, 1.0);
        }

        s.store_add_scaled_inputs3_offset(296, s.ad_value(278), p.p364, s.ad_value(279), p.p365, s.ad_value(280), p.p366, p.p277);

        s.store_add_scaled_inputs3_offset(297, s.ad_value(278), p.p367, s.ad_value(279), p.p368, s.ad_value(280), p.p369, p.p278);

        s.store_add_scaled_inputs3_offset(298, s.ad_value(278), p.p370, s.ad_value(279), p.p371, s.ad_value(280), p.p372, p.p275);

        s.store_add_scaled_inputs3_offset(299, s.ad_value(278), p.p373, s.ad_value(279), p.p374, s.ad_value(280), p.p375, p.p272);

        s.store_add_scaled_inputs3_offset(300, s.ad_value(278), p.p376, s.ad_value(279), p.p377, s.ad_value(280), p.p378, p.p273);

        s.store_add_scaled_inputs3_offset(301, s.ad_value(278), p.p379, s.ad_value(279), p.p380, s.ad_value(280), p.p381, p.p274);

        s.store_add_scaled_inputs3_offset(302, s.ad_value(278), p.p382, s.ad_value(279), p.p383, s.ad_value(280), p.p384, p.p283);

        s.b[540] = (s.v[302] < 0.0);
        s.v[540] = if s.b[540] { 1.0 } else { 0.0 };

        if s.b[540] {
            s.store_scalar(302, 0.0);
        }

        s.b[541] = (s.v[302] > 1.0);
        s.v[541] = if s.b[541] { 1.0 } else { 0.0 };

        if ((!s.b[540]) && s.b[541]) {
            s.store_scalar(302, 1.0);
        }

        s.store_add_scaled_inputs3_offset(303, s.ad_value(278), p.p385, s.ad_value(279), p.p386, s.ad_value(280), p.p387, p.p284);

        s.store_add_scaled_inputs3_offset(304, s.ad_value(278), p.p388, s.ad_value(279), p.p389, s.ad_value(280), p.p390, p.p285);

        s.store_add_scaled_inputs3_offset(305, s.ad_value(278), p.p391, s.ad_value(279), p.p392, s.ad_value(280), p.p393, p.p282);

        s.store_add_scaled_inputs3_offset(306, s.ad_value(278), p.p394, s.ad_value(279), p.p395, s.ad_value(280), p.p396, p.p279);

        s.store_add_scaled_inputs3_offset(307, s.ad_value(278), p.p397, s.ad_value(279), p.p398, s.ad_value(280), p.p399, p.p280);

        s.store_add_scaled_inputs3_offset(308, s.ad_value(278), p.p400, s.ad_value(279), p.p401, s.ad_value(280), p.p402, p.p281);

        s.store_add_scaled_inputs3_offset(313, s.ad_value(278), p.p403, s.ad_value(279), p.p404, s.ad_value(280), p.p405, p.p71);

        s.store_add_scaled_inputs3_offset(314, s.ad_value(278), p.p406, s.ad_value(279), p.p407, s.ad_value(280), p.p408, p.p72);

        s.store_add_scaled_inputs3_offset(315, s.ad_value(278), p.p409, s.ad_value(279), p.p410, s.ad_value(280), p.p411, p.p73);

        s.store_add_scaled_inputs3_offset(316, s.ad_value(278), p.p412, s.ad_value(279), p.p413, s.ad_value(280), p.p414, p.p74);

        s.store_add_scaled_inputs3_offset(317, s.ad_value(278), p.p415, s.ad_value(279), p.p416, s.ad_value(280), p.p417, p.p75);

        s.store_add_scaled_inputs3_offset(318, s.ad_value(278), p.p418, s.ad_value(279), p.p419, s.ad_value(280), p.p420, p.p84);

        s.store_add_scaled_inputs3_offset(319, s.ad_value(278), p.p421, s.ad_value(279), p.p422, s.ad_value(280), p.p423, p.p76);

        s.store_add_scaled_inputs3_offset(309, s.ad_value(278), p.p430, s.ad_value(279), p.p431, s.ad_value(280), p.p432, p.p87);

        s.store_add_scaled_inputs3_offset(310, s.ad_value(278), p.p433, s.ad_value(279), p.p434, s.ad_value(280), p.p435, p.p88);

        s.store_add_scaled_inputs3_offset(311, s.ad_value(278), p.p436, s.ad_value(279), p.p437, s.ad_value(280), p.p438, p.p61);

        s.store_add_scaled_inputs3_offset(312, s.ad_value(278), p.p439, s.ad_value(279), p.p440, s.ad_value(280), p.p441, p.p62);

        s.store_add_scaled_inputs3_offset(320, s.ad_value(278), p.p424, s.ad_value(279), p.p425, s.ad_value(280), p.p426, p.p85);

        s.store_add_scaled_inputs3_offset(321, s.ad_value(278), p.p427, s.ad_value(279), p.p428, s.ad_value(280), p.p429, p.p86);

        s.store_add_scaled_inputs3_offset(326, s.ad_value(278), p.p460, s.ad_value(279), p.p461, s.ad_value(280), p.p462, p.p113);

        s.store_add_scaled_inputs3_offset(322, s.ad_value(278), p.p442, s.ad_value(279), p.p443, s.ad_value(280), p.p444, p.p89);

        s.store_add_scaled_inputs3_offset(323, s.ad_value(278), p.p445, s.ad_value(279), p.p446, s.ad_value(280), p.p447, p.p90);

        s.store_add_scaled_inputs3_offset(324, s.ad_value(278), p.p448, s.ad_value(279), p.p449, s.ad_value(280), p.p450, p.p91);

        s.store_add_scaled_inputs3_offset(325, s.ad_value(278), p.p451, s.ad_value(279), p.p452, s.ad_value(280), p.p453, p.p92);

        s.store_add_scaled_inputs3_offset(417, s.ad_value(278), p.p454, s.ad_value(279), p.p455, s.ad_value(280), p.p456, p.p93);

        s.store_add_scaled_inputs3_offset(418, s.ad_value(278), p.p457, s.ad_value(279), p.p458, s.ad_value(280), p.p459, p.p94);

        s.store_add_scaled_inputs3_offset(327, s.ad_value(278), p.p463, s.ad_value(279), p.p464, s.ad_value(280), p.p465, p.p116);

        s.store_add_scaled_inputs3_offset(328, s.ad_value(278), p.p466, s.ad_value(279), p.p467, s.ad_value(280), p.p468, p.p123);

        s.store_add_scaled_inputs3_offset(329, s.ad_value(278), p.p469, s.ad_value(279), p.p470, s.ad_value(280), p.p471, p.p124);

        s.store_add_scaled_inputs3_offset(330, s.ad_value(278), p.p472, s.ad_value(279), p.p473, s.ad_value(280), p.p474, p.p122);

        s.store_add_scaled_inputs3_offset(331, s.ad_value(278), p.p475, s.ad_value(279), p.p476, s.ad_value(280), p.p477, p.p135);

        s.store_add_scaled_inputs3_offset(332, s.ad_value(278), p.p478, s.ad_value(279), p.p479, s.ad_value(280), p.p480, p.p139);

        s.store_add_scaled_inputs3_offset(333, s.ad_value(278), p.p481, s.ad_value(279), p.p482, s.ad_value(280), p.p483, p.p145);

        s.store_add_scaled_inputs3_offset(334, s.ad_value(278), p.p484, s.ad_value(279), p.p485, s.ad_value(280), p.p486, p.p148);

        s.store_add_scaled_inputs3_offset(335, s.ad_value(278), p.p487, s.ad_value(279), p.p488, s.ad_value(280), p.p489, p.p155);

        s.store_add_scaled_inputs3_offset(336, s.ad_value(278), p.p490, s.ad_value(279), p.p491, s.ad_value(280), p.p492, p.p142);

        s.store_add_scaled_inputs3_offset(342, s.ad_value(278), p.p493, s.ad_value(279), p.p494, s.ad_value(280), p.p495, p.p163);

        s.store_add_scaled_inputs3_offset(337, s.ad_value(278), p.p496, s.ad_value(279), p.p497, s.ad_value(280), p.p498, p.p157);

        s.store_add_scaled_inputs3_offset(338, s.ad_value(278), p.p499, s.ad_value(279), p.p500, s.ad_value(280), p.p501, p.p156);

        s.store_add_scaled_inputs3_offset(339, s.ad_value(278), p.p502, s.ad_value(279), p.p503, s.ad_value(280), p.p504, p.p158);

        s.store_add_scaled_inputs3_offset(340, s.ad_value(278), p.p505, s.ad_value(279), p.p506, s.ad_value(280), p.p507, p.p160);

        s.store_add_scaled_inputs3_offset(341, s.ad_value(278), p.p508, s.ad_value(279), p.p509, s.ad_value(280), p.p510, p.p161);

        s.store_add_scaled_inputs3_offset(343, s.ad_value(278), p.p511, s.ad_value(279), p.p512, s.ad_value(280), p.p513, p.p136);

        s.store_add_scaled_inputs3_offset(344, s.ad_value(278), p.p514, s.ad_value(279), p.p515, s.ad_value(280), p.p516, p.p166);

        s.store_add_scaled_inputs3_offset(345, s.ad_value(278), p.p517, s.ad_value(279), p.p518, s.ad_value(280), p.p519, p.p167);

        s.store_add_scaled_inputs3_offset(346, s.ad_value(278), p.p520, s.ad_value(279), p.p521, s.ad_value(280), p.p522, p.p173);

        s.store_add_scaled_inputs3_offset(347, s.ad_value(278), p.p523, s.ad_value(279), p.p524, s.ad_value(280), p.p525, p.p176);

        s.store_add_scaled_inputs3_offset(348, s.ad_value(278), p.p526, s.ad_value(279), p.p527, s.ad_value(280), p.p528, p.p182);

        s.store_add_scaled_inputs3_offset(349, s.ad_value(278), p.p529, s.ad_value(279), p.p530, s.ad_value(280), p.p531, p.p170);

        s.store_add_scaled_inputs3_offset(350, s.ad_value(278), p.p532, s.ad_value(279), p.p533, s.ad_value(280), p.p534, p.p183);

        s.store_add_scaled_inputs3_offset(351, s.ad_value(278), p.p535, s.ad_value(279), p.p536, s.ad_value(280), p.p537, p.p186);

        s.store_add_scaled_inputs3_offset(353, s.ad_value(278), p.p538, s.ad_value(279), p.p539, s.ad_value(280), p.p540, p.p119);

        s.store_add_scaled_inputs3_offset(354, s.ad_value(278), p.p541, s.ad_value(279), p.p542, s.ad_value(280), p.p543, p.p130);

        s.store_add_scaled_inputs3_offset(355, s.ad_value(278), p.p544, s.ad_value(279), p.p545, s.ad_value(280), p.p546, p.p205);

        s.store_add_scaled_inputs3_offset(356, s.ad_value(278), p.p547, s.ad_value(279), p.p548, s.ad_value(280), p.p549, p.p305);

        s.store_add_scaled_inputs3_offset(357, s.ad_value(278), p.p550, s.ad_value(279), p.p551, s.ad_value(280), p.p552, p.p306);

        s.store_add_scaled_inputs3_offset(358, s.ad_value(278), p.p553, s.ad_value(279), p.p554, s.ad_value(280), p.p555, p.p307);

        s.store_add_scaled_inputs3_offset(359, s.ad_value(278), p.p556, s.ad_value(279), p.p557, s.ad_value(280), p.p558, p.p308);

        s.store_add_scaled_inputs3_offset(360, s.ad_value(278), p.p559, s.ad_value(279), p.p560, s.ad_value(280), p.p561, p.p210);

        s.store_add_scaled_inputs3_offset(361, s.ad_value(278), p.p562, s.ad_value(279), p.p563, s.ad_value(280), p.p564, p.p214);

        s.store_add_scaled_inputs3_offset(362, s.ad_value(278), p.p565, s.ad_value(279), p.p566, s.ad_value(280), p.p567, p.p208);

        s.store_add_scaled_inputs3_offset(363, s.ad_value(278), p.p568, s.ad_value(279), p.p569, s.ad_value(280), p.p570, p.p206);

        s.store_add_scaled_inputs3_offset(364, s.ad_value(278), p.p571, s.ad_value(279), p.p572, s.ad_value(280), p.p573, p.p207);

        s.store_add_scaled_inputs3_offset(365, s.ad_value(278), p.p574, s.ad_value(279), p.p575, s.ad_value(280), p.p576, p.p209);

        s.store_add_scaled_inputs3_offset(366, s.ad_value(278), p.p577, s.ad_value(279), p.p578, s.ad_value(280), p.p579, p.p256);

        s.store_add_scaled_inputs3_offset(367, s.ad_value(278), p.p580, s.ad_value(279), p.p581, s.ad_value(280), p.p582, p.p257);

        s.store_add_scaled_inputs3_offset(368, s.ad_value(278), p.p583, s.ad_value(279), p.p584, s.ad_value(280), p.p585, p.p258);

        s.store_add_scaled_inputs3_offset(408, s.ad_value(278), p.p706, s.ad_value(279), p.p707, s.ad_value(280), p.p708, p.p217);

        s.store_add_scaled_inputs3_offset(409, s.ad_value(278), p.p709, s.ad_value(279), p.p710, s.ad_value(280), p.p711, p.p218);

        s.store_add_scaled_inputs3_offset(410, s.ad_value(278), p.p712, s.ad_value(279), p.p713, s.ad_value(280), p.p714, p.p219);

        s.store_add_scaled_inputs3_offset(411, s.ad_value(278), p.p715, s.ad_value(279), p.p716, s.ad_value(280), p.p717, p.p220);

        s.store_add_scaled_inputs3_offset(412, s.ad_value(278), p.p718, s.ad_value(279), p.p719, s.ad_value(280), p.p720, p.p221);

        s.store_add_scaled_inputs3_offset(413, s.ad_value(278), p.p721, s.ad_value(279), p.p722, s.ad_value(280), p.p723, p.p222);

        s.store_add_scaled_inputs3_offset(414, s.ad_value(278), p.p724, s.ad_value(279), p.p725, s.ad_value(280), p.p726, p.p223);

        s.store_add_scaled_inputs3_offset(415, s.ad_value(278), p.p727, s.ad_value(279), p.p728, s.ad_value(280), p.p729, p.p224);

        s.store_add_scaled_inputs3_offset(416, s.ad_value(278), p.p730, s.ad_value(279), p.p731, s.ad_value(280), p.p732, p.p225);

        s.store_add_scaled_inputs3_offset(369, s.ad_value(278), p.p586, s.ad_value(279), p.p587, s.ad_value(280), p.p588, p.p226);

        s.store_add_scaled_inputs3_offset(370, s.ad_value(278), p.p589, s.ad_value(279), p.p590, s.ad_value(280), p.p591, p.p227);

        s.store_add_scaled_inputs3_offset(371, s.ad_value(278), p.p592, s.ad_value(279), p.p593, s.ad_value(280), p.p594, p.p228);

        s.store_add_scaled_inputs3_offset(373, s.ad_value(278), p.p595, s.ad_value(279), p.p596, s.ad_value(280), p.p597, p.p230);

        s.store_add_scaled_inputs3_offset(372, s.ad_value(278), p.p598, s.ad_value(279), p.p599, s.ad_value(280), p.p600, p.p229);

        s.store_add_scaled_inputs3_offset(381, s.ad_value(278), p.p610, s.ad_value(279), p.p611, s.ad_value(280), p.p612, p.p247);

        s.store_add_scaled_inputs3_offset(374, s.ad_value(278), p.p619, s.ad_value(279), p.p620, s.ad_value(280), p.p621, p.p250);

        s.store_add_scaled_inputs3_offset(375, s.ad_value(278), p.p622, s.ad_value(279), p.p623, s.ad_value(280), p.p624, p.p251);

        s.store_add_scaled_inputs3_offset(376, s.ad_value(278), p.p625, s.ad_value(279), p.p626, s.ad_value(280), p.p627, p.p252);

        s.store_add_scaled_inputs3_offset(377, s.ad_value(278), p.p628, s.ad_value(279), p.p629, s.ad_value(280), p.p630, p.p253);

        s.store_add_scaled_inputs3_offset(378, s.ad_value(278), p.p601, s.ad_value(279), p.p602, s.ad_value(280), p.p603, p.p244);

        s.store_add_scaled_inputs3_offset(379, s.ad_value(278), p.p604, s.ad_value(279), p.p605, s.ad_value(280), p.p606, p.p245);

        s.store_add_scaled_inputs3_offset(380, s.ad_value(278), p.p607, s.ad_value(279), p.p608, s.ad_value(280), p.p609, p.p246);

        s.store_add_scaled_inputs3_offset(390, s.ad_value(278), p.p613, s.ad_value(279), p.p614, s.ad_value(280), p.p615, p.p248);

        s.store_add_scaled_inputs3_offset(392, s.ad_value(278), p.p631, s.ad_value(279), p.p632, s.ad_value(280), p.p633, p.p254);

        s.store_add_scaled_inputs3_offset(391, s.ad_value(278), p.p616, s.ad_value(279), p.p617, s.ad_value(280), p.p618, p.p249);

        s.store_add_scaled_inputs3_offset(393, s.ad_value(278), p.p634, s.ad_value(279), p.p635, s.ad_value(280), p.p636, p.p255);

        s.store_add_scaled_inputs3_offset(382, s.ad_value(278), p.p637, s.ad_value(279), p.p638, s.ad_value(280), p.p639, p.p231);

        s.store_add_scaled_inputs3_offset(383, s.ad_value(278), p.p643, s.ad_value(279), p.p644, s.ad_value(280), p.p645, p.p232);

        s.store_add_scaled_inputs3_offset(384, s.ad_value(278), p.p649, s.ad_value(279), p.p650, s.ad_value(280), p.p651, p.p233);

        s.store_add_scaled_inputs3_offset(385, s.ad_value(278), p.p655, s.ad_value(279), p.p656, s.ad_value(280), p.p657, p.p242);

        s.store_add_scaled_inputs3_offset(386, s.ad_value(278), p.p640, s.ad_value(279), p.p641, s.ad_value(280), p.p642, p.p236);

        s.store_add_scaled_inputs3_offset(387, s.ad_value(278), p.p646, s.ad_value(279), p.p647, s.ad_value(280), p.p648, p.p237);

        s.store_add_scaled_inputs3_offset(388, s.ad_value(278), p.p652, s.ad_value(279), p.p653, s.ad_value(280), p.p654, p.p238);

        s.store_add_scaled_inputs3_offset(389, s.ad_value(278), p.p658, s.ad_value(279), p.p659, s.ad_value(280), p.p660, p.p243);

        s.store_add_scaled_inputs3_offset(395, s.ad_value(278), p.p661, s.ad_value(279), p.p662, s.ad_value(280), p.p663, p.p240);

        s.store_add_scaled_inputs3_offset(394, s.ad_value(278), p.p664, s.ad_value(279), p.p665, s.ad_value(280), p.p666, p.p241);

        s.store_add_scaled_inputs3_offset(396, s.ad_value(278), p.p667, s.ad_value(279), p.p668, s.ad_value(280), p.p669, p.p259);

        s.store_add_scaled_inputs3_offset(397, s.ad_value(278), p.p670, s.ad_value(279), p.p671, s.ad_value(280), p.p672, p.p260);

        s.store_add_scaled_inputs3_offset(398, s.ad_value(278), p.p673, s.ad_value(279), p.p674, s.ad_value(280), p.p675, p.p261);

        s.store_add_scaled_inputs3_offset(399, s.ad_value(278), p.p676, s.ad_value(279), p.p677, s.ad_value(280), p.p678, p.p262);

        s.store_add_scaled_inputs3_offset(400, s.ad_value(278), p.p679, s.ad_value(279), p.p680, s.ad_value(280), p.p681, p.p100);

        s.store_add_scaled_inputs3_offset(401, s.ad_value(278), p.p682, s.ad_value(279), p.p683, s.ad_value(280), p.p684, p.p129);

        s.store_add_scaled_inputs3_offset(402, s.ad_value(278), p.p685, s.ad_value(279), p.p686, s.ad_value(280), p.p687, p.p103);

        s.store_add_scaled_inputs3_offset(403, s.ad_value(278), p.p688, s.ad_value(279), p.p689, s.ad_value(280), p.p690, p.p106);

        s.store_add_scaled_inputs3_offset(404, s.ad_value(278), p.p691, s.ad_value(279), p.p692, s.ad_value(280), p.p693, p.p110);

        s.store_add_scaled_inputs3_offset(405, s.ad_value(278), p.p694, s.ad_value(279), p.p695, s.ad_value(280), p.p696, p.p111);

        s.store_add_scaled_inputs3_offset(407, s.ad_value(278), p.p697, s.ad_value(279), p.p698, s.ad_value(280), p.p699, p.p112);

        s.store_add_scaled_inputs3_offset(406, s.ad_value(278), p.p700, s.ad_value(279), p.p701, s.ad_value(280), p.p702, p.p137);

        s.store_add_scaled_inputs3_offset(352, s.ad_value(278), p.p703, s.ad_value(279), p.p704, s.ad_value(280), p.p705, p.p187);

        s.store_add_scaled_inputs3_offset(62, s.ad_value(278), p.p739, s.ad_value(279), p.p740, s.ad_value(280), p.p741, p.p95);

        s.store_add_scaled_inputs3_offset(66, s.ad_value(278), p.p742, s.ad_value(279), p.p743, s.ad_value(280), p.p744, p.p96);

        s.store_add_scaled_inputs3_offset(67, s.ad_value(278), p.p745, s.ad_value(279), p.p746, s.ad_value(280), p.p747, p.p97);

        s.store_add_scaled_inputs3_offset(68, s.ad_value(278), p.p748, s.ad_value(279), p.p749, s.ad_value(280), p.p750, p.p98);

        s.b[542] = ((p.p20 == 1.0) && (p.p317 != 0.0));
        s.v[542] = if s.b[542] { 1.0 } else { 0.0 };

        if s.b[542] {
            s.store_add_scaled_inputs3_offset(275, s.ad_value(278), p.p733, s.ad_value(279), p.p734, s.ad_value(280), p.p735, p.p317);
            s.store_add_scaled_inputs3_offset(276, s.ad_value(278), p.p736, s.ad_value(279), p.p737, s.ad_value(280), p.p738, p.p318);
        }

        if (!s.b[542]) {
            s.store_scalar(275, 0.0);
            s.store_scalar(276, 0.0);
        }

        s.v[17] = ((3.9 * 8.85418e-12) / p.p45);

        s.v[18] = ((3.9 * 8.85418e-12) / p.p47);

        s.v[19] = ((3.9 * 8.85418e-12) / p.p46);

        s.v[20] = (s.v[16] / p.p49);

        s.v[21] = (p.p59 / 3.9);

        s.b[543] = (!param_given[47]);
        s.v[543] = if s.b[543] { 1.0 } else { 0.0 };

        if s.b[543] {
            s.store_scalar(221, (((p.p45 * p.p60) / 3.9) - p.p48));
        }

        if (!s.b[543]) {
            s.store_scalar(221, p.p47);
        }

        s.b[544] = (p.p138 > 0.0);
        s.v[544] = if s.b[544] { 1.0 } else { 0.0 };

        if s.b[544] {
            s.store_mul_sub_from_scalar_ad_rhs(331, 331, 1.0, A::mul(s.ad_value(406), A::powf(s.ad_value(2), (-p.p138))));
        }

        if (!s.b[544]) {
            s.store_mul_sub_from_scalar_rhs(331, 331, 1.0, 406);
        }

        s.store_ad_value(332, A::add_scaled_inputs(s.ad_value(332), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p141))), p.p140));

        s.store_ad_value(333, A::add_scaled_inputs(s.ad_value(333), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p147))), p.p146));

        s.store_offset_scaled_ad(137, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p153))), p.p152, p.p151);

        s.store_ad_value(334, A::add_scaled_inputs(s.ad_value(334), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p150))), p.p149));

        s.store_ad_value(336, A::add_scaled_inputs(s.ad_value(336), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p144))), p.p143));

        s.store_ad_value(342, A::add_scaled_inputs(s.ad_value(342), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p165))), p.p164));

        s.b[545] = (p.p188 > 0.0);
        s.v[545] = if s.b[545] { 1.0 } else { 0.0 };

        if s.b[545] {
            s.store_mul_sub_from_scalar_ad_rhs(344, 344, 1.0, A::mul(s.ad_value(352), A::powf(s.ad_value(2), (-p.p188))));
        }

        if (!s.b[545]) {
            s.store_mul_sub_from_scalar_rhs(344, 344, 1.0, 352);
        }

        s.store_ad_value(345, A::add_scaled_inputs(s.ad_value(345), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p169))), p.p168));

        s.store_ad_value(346, A::add_scaled_inputs(s.ad_value(346), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p175))), p.p174));

        s.store_offset_scaled_ad(138, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p181))), p.p180, p.p179);

        s.store_ad_value(347, A::add_scaled_inputs(s.ad_value(347), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p178))), p.p177));

        s.store_ad_value(349, A::add_scaled_inputs(s.ad_value(349), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p172))), p.p171));

        s.store_ad_value(350, A::add_scaled_inputs(s.ad_value(350), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p185))), p.p184));

        s.b[546] = (p.p14 == 1.0);
        s.v[546] = if s.b[546] { 1.0 } else { 0.0 };

        if s.b[546] {
            s.store_ad_value(283, A::add_scaled_inputs(s.ad_value(283), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p197))), p.p196));
            s.store_ad_value(282, A::add_scaled_inputs(s.ad_value(282), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p201))), p.p200));
        }

        if (!s.b[546]) {
            s.store_ad_value(281, A::add_scaled_inputs(s.ad_value(281), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p193))), p.p192));
        }

        s.store_ad_value(360, A::add_scaled_inputs(s.ad_value(360), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p212))), p.p211));

        s.store_ad_value(326, A::add_scaled_inputs(s.ad_value(326), 1.0, A::powf(A::scale(s.ad_value(2), 1000000.0), (-p.p115)), p.p114));

        s.store_ad_value(327, A::add_scaled_inputs(s.ad_value(327), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p118))), p.p117));

        s.store_ad_value(328, A::add_scaled_inputs(s.ad_value(328), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p126))), p.p125));

        s.store_ad_value(329, A::add_scaled_inputs(s.ad_value(329), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p128))), p.p127));

    }

    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.store_ad_value(400, A::add_scaled_inputs(s.ad_value(400), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p102))), p.p101));

        s.store_ad_value(401, A::add_scaled_inputs(s.ad_value(401), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p133))), p.p132));

        s.store_ad_value(402, A::add_scaled_inputs(s.ad_value(402), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p105))), p.p104));

        s.store_ad_value(403, A::add_scaled_inputs(s.ad_value(403), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p108))), p.p107));

        s.store_offset_scaled_ad(92, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p80))), p.p79, p.p77);

        s.store_offset_scaled_ad(93, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p82))), p.p81, p.p78);

        s.b[547] = (s.v[331] < 0.0);
        s.v[547] = if s.b[547] { 1.0 } else { 0.0 };

        if s.b[547] {
            s.store_scalar(331, 0.03);
        }

        s.b[548] = (s.v[332] < 0.0);
        s.v[548] = if s.b[548] { 1.0 } else { 0.0 };

        if s.b[548] {
            s.store_scalar(332, 0.0);
        }

        s.b[549] = (s.v[336] < 0.0);
        s.v[549] = if s.b[549] { 1.0 } else { 0.0 };

        if s.b[549] {
            s.store_scalar(336, 0.0);
        }

        s.b[550] = (s.v[334] < 0.0);
        s.v[550] = if s.b[550] { 1.0 } else { 0.0 };

        if s.b[550] {
            s.store_scalar(334, 0.0);
        }

        s.b[551] = (s.v[335] < 0.0);
        s.v[551] = if s.b[551] { 1.0 } else { 0.0 };

        if s.b[551] {
            s.store_scalar(335, 0.0);
        }

        s.b[552] = (s.v[401] < 0.0);
        s.v[552] = if s.b[552] { 1.0 } else { 0.0 };

        if s.b[552] {
            s.store_scalar(401, 0.0);
        }

        s.v[134] = p.p190;

        s.b[555] = (s.v[134] < 0.0);
        s.v[555] = if s.b[555] { 1.0 } else { 0.0 };

        if s.b[555] {
            s.store_scalar(134, 0.0);
        }

        s.b[556] = (s.v[281] < 0.0);
        s.v[556] = if s.b[556] { 1.0 } else { 0.0 };

        if s.b[556] {
            s.store_scalar(281, 0.0);
        }

        s.v[136] = p.p194;

        s.b[557] = (s.v[136] < 0.0);
        s.v[557] = if s.b[557] { 1.0 } else { 0.0 };

        if s.b[557] {
            s.store_scalar(136, 0.0);
        }

        s.b[558] = (s.v[283] < 0.0);
        s.v[558] = if s.b[558] { 1.0 } else { 0.0 };

        if s.b[558] {
            s.store_scalar(283, 0.0);
        }

        s.v[135] = p.p198;

        s.b[559] = (s.v[135] < 0.0);
        s.v[559] = if s.b[559] { 1.0 } else { 0.0 };

        if s.b[559] {
            s.store_scalar(135, 0.0);
        }

        s.b[560] = (s.v[282] < 0.0);
        s.v[560] = if s.b[560] { 1.0 } else { 0.0 };

        if s.b[560] {
            s.store_scalar(282, 0.0);
        }

        s.b[561] = (s.v[284] < 0.0);
        s.v[561] = if s.b[561] { 1.0 } else { 0.0 };

        if s.b[561] {
            s.store_scalar(284, 0.0);
        }

        s.b[565] = (s.v[326] < 2.0);
        s.v[565] = if s.b[565] { 1.0 } else { 0.0 };

        if s.b[565] {
            s.store_scalar(326, 2.0);
        }

        s.store_offset_sqrt_ad(89, A::offset(A::div(s.ad_value(321), s.ad_value(2)), 1.0), (-1.0));

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

        s.b[566] = (p.p12 != 1.0);
        s.v[566] = if s.b[566] { 1.0 } else { 0.0 };

        if s.b[566] {
            s.store_scale(114, 343, (1.0 / 3.0));
            s.store_scalar(115, (1.0 / 3.0));
            s.store_scale(143, 351, (1.0 / 3.0));
        }

        s.v[129] = (1e-8 / (s.v[21] * p.p45));

        s.store_div_from_scalar_scaled_ad(131, 1.0, A::pow(A::scale(s.ad_value(3), 1000000.0), s.ad_value(286)), p.p2);

        s.v[253] = ((((s.v[21] * p.p45) * p.p49)) as f64).sqrt();

        s.v[144] = (1e-8 / (s.v[21] * p.p46));

        s.b[567] = (p.p296 >= (s.v[2] / 2.0));
        s.v[567] = if s.b[567] { 1.0 } else { 0.0 };

        if s.b[567] {
            s.store_scalar(249, 0.0);
        }

        if (!s.b[567]) {
            s.store_scalar(249, p.p296);
        }

        s.b[568] = ((p.p18 != 0.0) && (p.p310 > 0.0));
        s.v[568] = if s.b[568] { 1.0 } else { 0.0 };

        if s.b[568] {
            s.store_offset_scaled(269, 3, ((p.p2) * (1.0 / (p.p310))), ((p.p312) * (1.0 / (p.p310))));
            s.store_offset_scaled(270, 3, ((p.p2) * (p.p311)), ((p.p312) * (p.p311)));
        }

        if (!s.b[568]) {
            s.store_scalar(269, 1.0);
            s.store_scalar(270, 0.0);
        }

        s.v[132] = (p.p215 * p.p7);

        s.v[133] = (p.p216 * p.p8);

        s.b[569] = (s.v[132] <= 0.001);
        s.v[569] = if s.b[569] { 1.0 } else { 0.0 };

        if s.b[569] {
            s.store_scalar(132, 0.001);
        }

        s.b[570] = (s.v[133] <= 0.001);
        s.v[570] = if s.b[570] { 1.0 } else { 0.0 };

        if s.b[570] {
            s.store_scalar(133, 0.001);
        }

        s.b[571] = (p.p14 == 1.0);
        s.v[571] = if s.b[571] { 1.0 } else { 0.0 };

        s.b[572] = (s.v[136] <= 0.0);
        s.v[572] = if s.b[572] { 1.0 } else { 0.0 };

        if (s.b[571] && s.b[572]) {
            s.store_scalar(136, 0.0);
        }

        s.b[573] = (s.v[135] <= 0.0);
        s.v[573] = if s.b[573] { 1.0 } else { 0.0 };

        if (s.b[571] && s.b[573]) {
            s.store_scalar(135, 0.0);
        }

        s.b[574] = (s.v[283] <= 0.0);
        s.v[574] = if s.b[574] { 1.0 } else { 0.0 };

        if (s.b[571] && s.b[574]) {
            s.store_scalar(283, 0.0);
        }

        s.b[575] = (s.v[282] <= 0.0);
        s.v[575] = if s.b[575] { 1.0 } else { 0.0 };

        if (s.b[571] && s.b[575]) {
            s.store_scalar(282, 0.0);
        }

        s.b[576] = (s.v[134] <= 0.0);
        s.v[576] = if s.b[576] { 1.0 } else { 0.0 };

        if ((!s.b[571]) && s.b[576]) {
            s.store_scalar(134, 0.0);
        }

        s.b[577] = (s.v[281] <= 0.0);
        s.v[577] = if s.b[577] { 1.0 } else { 0.0 };

        if ((!s.b[571]) && s.b[577]) {
            s.store_scalar(281, 0.0);
        }

        s.b[578] = (p.p297 <= 0.0);
        s.v[578] = if s.b[578] { 1.0 } else { 0.0 };

        if s.b[578] {
            s.store_scalar(95, 300.15);
        }

        if (!s.b[578]) {
            s.store_scalar(95, (p.p297 + 273.15));
        }

        s.b[579] = (p.p12 == 1.0);
        s.v[579] = if s.b[579] { 1.0 } else { 0.0 };

        if s.b[579] {
            s.store_scalar(205, 4.97232e-7);
        }

        if (!s.b[579]) {
            s.store_scalar(205, 3.42537e-7);
        }

        s.b[580] = (p.p12 == 1.0);
        s.v[580] = if s.b[580] { 1.0 } else { 0.0 };

        if s.b[580] {
            s.store_scalar(206, 745669000000.0);
        }

        if (!s.b[580]) {
            s.store_scalar(206, 1166450000000.0);
        }

        s.v[34] = (p.p99 * p.p99);

        s.store_scale(35, 394, p.p99);

        s.store_square(36, 35);

        s.store_scaled_limited_exp_scaled_input(207, 395, ((((p.p239 / p.p99)).max(1e-38)) as f64).ln(), 1.0 / (s.v[34]));

        s.store_div_ad_lhs(208, A::limited_exp(A::mul(s.ad_value(395), A::ln(A::max_with_scalar(A::div_from_scalar(p.p239, s.ad_value(35)), 1e-38)))), 36);

        s.store_mul3_lhs(186, 3, 205, 208);

        s.store_offset_scaled(273, 3, (((0.3333333333333333 * 1.0 / (p.p315))) * ((p.p316 * 1.0 / (((p.p315 * p.p2) * (s.v[0] - p.p314)))))), ((p.p313) * ((p.p316 * 1.0 / (((p.p315 * p.p2) * (s.v[0] - p.p314)))))));

        s.b[581] = (s.v[273] > 0.001);
        s.v[581] = if s.b[581] { 1.0 } else { 0.0 };

        if s.b[581] {
            s.store_div_from_scalar(273, 1.0, 273);
        }

        if (!s.b[581]) {
            s.store_scalar(273, 1000.0);
        }

        s.b[583] = ((p.p18 != 0.0) && (p.p310 > 0.0));
        s.v[583] = if s.b[583] { 1.0 } else { 0.0 };

        if s.b[583] {
            s.store_offset_voltage(271, ctx, nodes, Some(4), None, ((ctx_temp) + (p.p9)));
        }

        if (!s.b[583]) {
            s.store_scalar(271, (ctx_temp + p.p9));
        }

        s.v[272] = (p.p298 + 273.15);

        s.store_scaled_sub_ad(271, A::offset(s.ad_value(271), s.v[272]), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(271), (-s.v[272]), A::offset(s.ad_value(271), (-s.v[272]))), ((0.25 * 0.01) * 0.01))), 0.5);

        s.store_div(96, 271, 95);

        s.store_sub(97, 271, 95);

        s.store_scale(55, 271, 8.61708e-5);

        s.store_sub_from_scalar_ad(54, p.p55, A::div_scaled_product_offset_denominator(s.ad_value(271), s.ad_value(271), p.p299, s.ad_value(271), p.p300, 1.0));

        s.store_mul_scaled_ad_rhs(35, 271, 1.0 / (300.15), A::sqrt_scaled_input(s.ad_value(271), 1.0 / (300.15)));

        s.store_mul_scaled_ad_rhs(100, 35, p.p54, A::limited_exp(A::sub_from_scalar((p.p55 / ((2.0 * 8.61708e-5) * 300.15)), A::div_scaled_inputs(s.ad_value(54), 1.0, s.ad_value(55), 2.0))));

        s.store_mul_ln_ad_rhs(80, 55, A::max_with_scalar(A::div_scaled_product(s.ad_value(289), s.ad_value(290), 1.0, A::square(s.ad_value(100)), 1.0), 1e-38));

        s.store_mul_ln_ad_rhs(50, 55, A::max_with_scalar(A::div(s.ad_value(290), s.ad_value(100)), 1e-38));

        let assign3610_ad_e4240: A = A::add(A::add_scaled_product(s.ad_value(54), 0.5, s.ad_value(55), A::ln(A::max_with_scalar(A::div_from_scalar(p.p52, s.ad_value(100)), 1e-38)), (-1.0)), A::sqrt(A::offset(A::mul(A::add_scaled_product(s.ad_value(54), 0.5, s.ad_value(55), A::ln(A::max_with_scalar(A::div_from_scalar(p.p52, s.ad_value(100)), 1e-38)), (-1.0)), A::add_scaled_product(s.ad_value(54), 0.5, s.ad_value(55), A::ln(A::max_with_scalar(A::div_from_scalar(p.p52, s.ad_value(100)), 1e-38)), (-1.0))), ((4.0 * 0.0001) * 0.0001))));
        s.store_scaled_sub_ad_rhs(51, 54, assign3610_ad_e4240, 0.5);

        s.b[585] = ((p.p52 != 0.0) && (!param_given[58]));
        s.v[585] = if s.b[585] { 1.0 } else { 0.0 };

        s.b[586] = (p.p13 == (-1.0));
        s.v[586] = if s.b[586] { 1.0 } else { 0.0 };

        if (s.b[585] && s.b[586]) {
            s.store_add_ad_lhs(288, A::offset(s.ad_value(288), (-(0.5 * p.p55))), 51);
        }

        if (s.b[585] && (!s.b[586])) {
            s.store_sub_ad_lhs(288, A::offset(s.ad_value(288), (0.5 * p.p55)), 51);
        }

        s.store_offset_scaled(98, 54, 0.5, p.p53);

        s.store_mul_sub_rhs(52, 212, 287, 98);

        s.store_mul_sub_rhs(53, 212, 288, 98);

        s.store_add_scaled_product_mixed_aia(99, A::scale_offset(s.ad_value(54), 0.5, p.p53), 1.0, 212, A::min(A::scale(s.ad_value(54), 0.5), A::mul(s.ad_value(55), A::ln(A::max_with_scalar(A::div(s.ad_value(289), s.ad_value(100)), 1e-38)))), (-1.0));

        s.store_mul_sub_rhs(200, 212, 287, 99);

        s.store_mul_sub_rhs(240, 212, 288, 99);

        let assign3720_ad_e4372: A = A::mul3(s.ad_value(331), A::pow(s.ad_value(96), s.ad_value(338)), A::offset(A::add_scaled_inputs(A::offset(A::mul(s.ad_value(337), s.ad_value(97)), 0.9), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(337), s.ad_value(97)), 0.9, A::offset(A::mul(s.ad_value(337), s.ad_value(97)), 0.9)), ((4.0 * 0.001) * 0.001))), 0.5), ((1.0) + ((-(0.5 * (0.9 + ((((0.9 * 0.9) + ((4.0 * 0.001) * 0.001))) as f64).sqrt())))))));
        s.store_ad_value(126, assign3720_ad_e4372);

        s.store_mul_scale_ad_rhs(123, 333, A::add(A::scale_offset(s.ad_value(97), p.p159, ((1.0) + ((-1e-6)))), A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(97), p.p159, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(97), p.p159, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001)))), 0.5);

        s.store_mul_scale_ad_rhs(122, 332, A::add(A::offset(A::mul(s.ad_value(339), s.ad_value(97)), ((1.0) + ((-1e-6)))), A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(339), s.ad_value(97)), ((1.0) + ((-1e-6))), A::offset(A::mul(s.ad_value(339), s.ad_value(97)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001)))), 0.5);

        s.store_mul_pow_ad_rhs(125, 334, s.ad_value(96), s.ad_value(340));

        s.store_mul_pow_ad_rhs(124, 335, s.ad_value(96), s.ad_value(341));

        s.store_scaled_add_ad(150, A::offset(A::mul(s.ad_value(355), s.ad_value(97)), ((1.0) + ((-1e-6)))), A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(355), s.ad_value(97)), ((1.0) + ((-1e-6))), A::offset(A::mul(s.ad_value(355), s.ad_value(97)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5);

        s.store_mul_ad_rhs(353, 353, A::scale_offset(s.ad_value(278), p.p120, 1.0));

        s.store_mul_offset_ad_rhs(164, 400, A::add_scaled_inputs(A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97))), 0.5, A::sqrt(A::offset(A::mul_sub_from_scalar_lhs(0.9, A::mul(s.ad_value(353), s.ad_value(97)), A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97)))), ((4.0 * 0.001) * 0.001))), 0.5), ((1.0) + ((-(0.5 * (0.9 + ((((0.9 * 0.9) + ((4.0 * 0.001) * 0.001))) as f64).sqrt()))))));

        s.b[587] = (s.v[164] < 1000.0);
        s.v[587] = if s.b[587] { 1.0 } else { 0.0 };

        if s.b[587] {
            s.store_scalar(164, 1000.0);
        }

        s.store_mul_offset_ad_rhs(166, 402, A::add_scaled_inputs(A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97))), 0.5, A::sqrt(A::offset(A::mul_sub_from_scalar_lhs(0.9, A::mul(s.ad_value(353), s.ad_value(97)), A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97)))), ((4.0 * 0.001) * 0.001))), 0.5), ((1.0) + ((-(0.5 * (0.9 + ((((0.9 * 0.9) + ((4.0 * 0.001) * 0.001))) as f64).sqrt()))))));

        s.b[588] = (s.v[166] < 1000.0);
        s.v[588] = if s.b[588] { 1.0 } else { 0.0 };

        if s.b[588] {
            s.store_scalar(166, 1000.0);
        }

        s.store_mul_offset_ad_rhs(167, 403, A::add_scaled_inputs(A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97))), 0.5, A::sqrt(A::offset(A::mul_sub_from_scalar_lhs(0.9, A::mul(s.ad_value(353), s.ad_value(97)), A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97)))), ((4.0 * 0.001) * 0.001))), 0.5), ((1.0) + ((-(0.5 * (0.9 + ((((0.9 * 0.9) + ((4.0 * 0.001) * 0.001))) as f64).sqrt()))))));

        s.b[589] = (s.v[167] < 1000.0);
        s.v[589] = if s.b[589] { 1.0 } else { 0.0 };

        if s.b[589] {
            s.store_scalar(167, 1000.0);
        }

        s.store_mul_offset_ad_rhs(107, 316, A::add_scaled_inputs(A::scale_offset(s.ad_value(97), p.p309, (((-(-0.9))) + ((-0.0001)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(97), p.p309, (((-(-0.9))) + ((-0.0001)))), A::scale_offset(s.ad_value(97), p.p309, (((-(-0.9))) + ((-0.0001))))), (-((4.0 * (-0.9)) * 0.0001)))), 0.5), (((-0.9)) + (1.0)));

        s.store_mul_ad_rhs(354, 354, A::scale_offset(s.ad_value(278), p.p131, 1.0));

        s.store_mul_offset_ad_rhs(165, 401, A::add_scaled_inputs(A::sub_from_scalar(0.9, A::mul(s.ad_value(354), s.ad_value(97))), 0.5, A::sqrt(A::offset(A::mul_sub_from_scalar_lhs(0.9, A::mul(s.ad_value(354), s.ad_value(97)), A::sub_from_scalar(0.9, A::mul(s.ad_value(354), s.ad_value(97)))), ((4.0 * 0.001) * 0.001))), 0.5), ((1.0) + ((-(0.5 * (0.9 + ((((0.9 * 0.9) + ((4.0 * 0.001) * 0.001))) as f64).sqrt()))))));

        s.store_offset_ad(168, A::add_scaled_inputs(A::offset(A::mul(s.ad_value(326), A::scale_offset(s.ad_value(97), p.p121, 1.0)), (-2.0)), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(326), A::scale_offset(s.ad_value(97), p.p121, 1.0)), (-2.0), A::offset(A::mul(s.ad_value(326), A::scale_offset(s.ad_value(97), p.p121, 1.0)), (-2.0))), ((4.0 * 0.001) * 0.001))), 0.5), 2.0);

        s.store_add_scaled_product_indices(175, 322, 1.0, 323, 97, 1.0);

        let assign3930_ad_e4824: A = A::add_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(324), 1.0, s.ad_value(325), s.ad_value(97), 1.0), (-1e-6)), 0.5, A::sqrt(A::add_scaled_offset_product_lhs(s.ad_value(324), (-(-(4.0 * 1e-6))), A::add_scaled_product(s.ad_value(324), 1.0, s.ad_value(325), s.ad_value(97), 1.0), (-1e-6), A::offset(A::add_scaled_product(s.ad_value(324), 1.0, s.ad_value(325), s.ad_value(97), 1.0), (-1e-6)), 1.0)), 0.5);
        s.store_add_scaled_inputs3(176, s.ad_value(324), 1.0, assign3930_ad_e4824, 1.0, s.ad_value(324), (-1.0));

        s.store_add_scaled_product_indices(108, 417, 1.0, 418, 97, 1.0);

        s.store_mul_scale_ad_rhs(182, 327, A::add(A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(330), s.ad_value(97))), (-1e-6)), A::sqrt(A::offset(A::mul_offset_lhs(A::sub_from_scalar(1.0, A::mul(s.ad_value(330), s.ad_value(97))), (-1e-6), A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(330), s.ad_value(97))), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5);

        s.store_ad_value(102, A::mul_offset_lhs(A::div_from_scalar(p.p302, s.ad_value(2)), p.p301, A::offset(s.ad_value(96), (-1.0))));

        s.store_mul_pow_ad_rhs(103, 368, s.ad_value(96), s.ad_value(356));

        s.store_mul_scale_ad_rhs(104, 379, A::add(A::offset(A::mul(s.ad_value(357), s.ad_value(97)), ((1.0) + ((-1e-6)))), A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(357), s.ad_value(97)), ((1.0) + ((-1e-6))), A::offset(A::mul(s.ad_value(357), s.ad_value(97)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001)))), 0.5);

        s.store_mul_scale_ad_rhs(105, 375, A::add(A::offset(A::mul(s.ad_value(358), s.ad_value(97)), ((1.0) + ((-1e-6)))), A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(358), s.ad_value(97)), ((1.0) + ((-1e-6))), A::offset(A::mul(s.ad_value(358), s.ad_value(97)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001)))), 0.5);

        s.store_limited_exp_ad(106, A::mul(s.ad_value(359), A::ln(A::max_with_scalar(s.ad_value(96), 1e-38))));

        s.store_mul(185, 186, 106);

        s.store_mul_voltage_ad(29, s.ad_value(212), ctx, nodes, Some(8), Some(6));

        s.store_mul_voltage_ad(30, s.ad_value(212), ctx, nodes, Some(5), Some(6));

        s.store_mul_voltage_ad(31, s.ad_value(212), ctx, nodes, Some(8), Some(5));

        s.store_mul_voltage_ad(32, s.ad_value(212), ctx, nodes, Some(3), Some(6));

        s.store_mul_voltage_ad(33, s.ad_value(212), ctx, nodes, Some(3), Some(5));

        s.store_mul_voltage_ad(209, s.ad_value(212), ctx, nodes, Some(8), Some(3));

        s.v[27] = 1.0;

        s.b[590] = (s.v[30] < 0.0);
        s.v[590] = if s.b[590] { 1.0 } else { 0.0 };

        if s.b[590] {
            s.store_scalar(27, (-1.0));
            s.copy_ad(22, 31);
            s.store_neg(26, 30);
            s.copy_ad(23, 33);
            s.copy_ad(24, 32);
        }

        if (!s.b[590]) {
            s.copy_ad(22, 29);
            s.copy_ad(26, 30);
            s.copy_ad(23, 32);
            s.copy_ad(24, 33);
        }

        s.store_mul_voltage_ad(234, s.ad_value(212), ctx, nodes, Some(7), Some(5));

        s.store_mul_voltage_ad(235, s.ad_value(212), ctx, nodes, Some(7), Some(6));

        s.store_offset_sqrt_ad(73, A::offset(A::square(s.ad_value(26)), 0.0004), (-0.02));

        s.store_scaled_sub(74, 73, 26, 0.5);

        s.store_add(25, 23, 74);

        s.store_sub(69, 22, 52);

    }

    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_sub(70, 23, 53);

        s.v[77] = ((((s.v[21] * p.p49) * p.p45)) as f64).sqrt();

        s.v[76] = (((p.p49 * ((s.v[21] * p.p45) + (0.375 * p.p49)))) as f64).sqrt();

        s.store_add_scaled_inputs3(34, s.ad_value(69), ((p.p46 * s.v[21]) * 1.0 / (s.v[78])), s.ad_value(70), (((p.p45 * s.v[21]) + p.p49) * 1.0 / (s.v[78])), s.ad_value(74), 1.0);

        s.store_offset_scaled_ad(35, A::atan(A::add_scaled_product(s.ad_value(311), 1.0, s.ad_value(312), s.ad_value(34), 1.0)), 0.3183098861837907, 0.5);

        s.store_offset_scaled(75, 35, (s.v[77] - s.v[76]), s.v[76]);

        s.store_offset_ad(61, A::div_scaled_product(s.ad_value(314), s.ad_value(2), 1.0, s.ad_value(75), 1.0), 1e-6);

        s.b[591] = (s.v[61] < 40.0);
        s.v[591] = if s.b[591] { 1.0 } else { 0.0 };

        if s.b[591] {
            s.store_div_from_scalar_offset_ad(88, 0.5, A::cosh(s.ad_value(61)), (-1.0));
        }

        if (!s.b[591]) {
            s.store_limited_exp_neg_input(88, 61);
        }

        s.store_offset_ad(61, A::div_scaled_product(s.ad_value(319), s.ad_value(2), 1.0, s.ad_value(75), 1.0), 1e-6);

        s.b[592] = (s.v[61] < 40.0);
        s.v[592] = if s.b[592] { 1.0 } else { 0.0 };

        if s.b[592] {
            s.store_div_from_scalar_offset_ad(90, 0.5, A::cosh(s.ad_value(61)), (-1.0));
        }

        if (!s.b[592]) {
            s.store_limited_exp_neg_input(90, 61);
        }

        s.b[593] = (s.v[61] < 40.0);
        s.v[593] = if s.b[593] { 1.0 } else { 0.0 };

        if s.b[593] {
            s.store_div_from_scalar_ad(91, 1.0, A::max_with_scalar(A::scale_offset(A::cosh(s.ad_value(61)), p.p83, (((((-2.0)) * (p.p83))) + (1.0))), 1e-6));
        }

        if (!s.b[593]) {
            s.store_div_ad(91, A::limited_exp_scaled_input(s.ad_value(61), -1.0), A::max_with_scalar(A::offset(A::limited_exp_scaled_input(s.ad_value(61), -1.0), p.p83), 1e-6));
        }

        s.store_offset_ad(61, A::div_scaled_product(s.ad_value(362), s.ad_value(2), 1.0, s.ad_value(75), 1.0), 1e-6);

        s.b[594] = (s.v[61] < 40.0);
        s.v[594] = if s.b[594] { 1.0 } else { 0.0 };

        if s.b[594] {
            s.store_add_ad_lhs(153, A::div_scaled_value_offset_denominator(s.ad_value(363), 0.5, A::cosh(s.ad_value(61)), (-1.0), 1.0), 364);
        }

        if (!s.b[594]) {
            s.store_add_scaled_product_right_ad(153, 364, 1.0, 363, A::limited_exp_scaled_input(s.ad_value(61), -1.0), 1.0);
        }

        s.b[595] = (p.p13 == (-1.0));
        s.v[595] = if s.b[595] { 1.0 } else { 0.0 };

        if s.b[595] {
            s.store_div_scaled_product_indices(79, 298, 2, 1.0, 75, 1.0);
        }

        s.b[596] = (s.v[79] > 40.0);
        s.v[596] = if s.b[596] { 1.0 } else { 0.0 };

        if (s.b[595] && s.b[596]) {
            s.store_scaled_limited_exp(34, 79, 0.5);
        }

        if (s.b[595] && (!s.b[596])) {
            s.store_offset_cosh_ad(34, s.ad_value(79), (-1.0));
        }

        if s.b[595] {
            s.store_sub_ad_rhs(35, 299, A::div_scaled_inputs(s.ad_value(300), 0.5, s.ad_value(34), 1.0));
            s.copy_ad(36, 301);
            s.copy_ad(246, 296);
            s.copy_ad(247, 297);
            s.copy_ad(248, 295);
        }

        if (!s.b[595]) {
            s.store_div_scaled_product_indices(79, 305, 2, 1.0, 75, 1.0);
        }

        s.b[597] = (s.v[79] > 40.0);
        s.v[597] = if s.b[597] { 1.0 } else { 0.0 };

        if ((!s.b[595]) && s.b[597]) {
            s.store_scaled_limited_exp(34, 79, 0.5);
        }

        if ((!s.b[595]) && (!s.b[597])) {
            s.store_offset_cosh_ad(34, s.ad_value(79), (-1.0));
        }

        if (!s.b[595]) {
            s.store_sub_ad_rhs(35, 306, A::div_scaled_inputs(s.ad_value(307), 0.5, s.ad_value(34), 1.0));
            s.copy_ad(36, 308);
            s.copy_ad(246, 303);
            s.copy_ad(247, 304);
            s.copy_ad(248, 302);
        }

        s.store_sub(34, 35, 36);

        s.store_add_scaled_inputs3(241, s.ad_value(36), 1.0, s.ad_value(34), 0.5, A::sqrt(A::offset(A::square(s.ad_value(34)), 0.0001)), 0.5);

        s.v[244] = (((1.60219e-19 * p.p52) * s.v[16]) / ((2.0 * s.v[19]) * s.v[19]));

        s.b[598] = (p.p52 != 0.0);
        s.v[598] = if s.b[598] { 1.0 } else { 0.0 };

        if s.b[598] {
            let assign4690_ad_e5404: A = A::add_scaled_product(A::sqrt(A::offset(A::mul3(s.ad_value(213), A::add_scaled_product(s.ad_value(246), (-1.0), s.ad_value(212), s.ad_value(25), 1.0), A::mul(s.ad_value(213), A::add_scaled_product(s.ad_value(246), (-1.0), s.ad_value(212), s.ad_value(25), 1.0))), ((4.0 * 0.01) * 0.01))), 1.0, s.ad_value(213), A::add_scaled_product(s.ad_value(246), (-1.0), s.ad_value(212), s.ad_value(25), 1.0), 1.0);
            s.store_offset_sqrt_ad(34, A::scale_offset(assign4690_ad_e5404, (0.5 * 1.0 / (s.v[244])), 1.0), (-1.0));
        }

        if (!s.b[598]) {
            s.store_scalar(34, 0.0);
        }

        s.store_scaled_mul(245, 34, 34, s.v[244]);

        s.store_neg_ad(245, A::sub(A::add_scaled_inputs3_offset(s.ad_value(245), (-0.5), s.ad_value(247), 0.5, A::sqrt(A::add_scaled_offset_product_lhs(s.ad_value(247), (-(-(4.0 * 0.01))), A::sub_scaled_inputs(s.ad_value(245), -1.0, s.ad_value(247), -1.0), (-0.01), A::offset(A::sub_scaled_inputs(s.ad_value(245), -1.0, s.ad_value(247), -1.0), (-0.01)), 1.0)), 0.5, ((-0.01) * 0.5)), s.ad_value(247)));

        s.store_sub_from_scalar(72, (-1.2), 74);

        s.v[243] = (((-s.v[19]) * s.v[20]) / ((s.v[19] + s.v[20]) * s.v[17]));

        s.store_mul_scaled_ad_rhs(242, 241, s.v[243], A::sub(A::add_scaled_product(s.ad_value(70), 1.0, A::mul3(s.ad_value(212), s.ad_value(213), s.ad_value(248)), s.ad_value(245), (-1.0)), s.ad_value(72)));

        s.store_scaled_add_ad_rhs(28, 25, A::sqrt(A::offset(A::square(s.ad_value(25)), ((4.0 * 0.001) * 0.001))), 0.5);

        s.store_add_ad_lhs(87, A::offset(s.ad_value(50), 0.4), 315);

        s.b[599] = (s.v[87] < 0.0);
        s.v[599] = if s.b[599] { 1.0 } else { 0.0 };

        if s.b[599] {
            s.store_scalar(84, 0.0);
        }

        if (!s.b[599]) {
            s.store_mul_ad_product_rhs(84, 320, s.ad_value(89), A::sqrt(s.ad_value(87)));
        }

        s.store_mul_ad_affine_product_rhs(83, 313, s.ad_value(88), A::sub(s.ad_value(80), s.ad_value(87)), -1.0, 0.0);

        s.store_add_ad(82, A::mul3_scaled_output(A::add_scaled_product(s.ad_value(107), 1.0, s.ad_value(318), s.ad_value(25), 1.0), s.ad_value(90), A::add_scaled_product(s.ad_value(73), 1.0, s.ad_value(317), A::sqrt(A::offset(s.ad_value(73), 0.01)), 1.0), -1.0), A::mul3(s.ad_value(92), s.ad_value(91), A::pow(A::offset(s.ad_value(73), 0.01), s.ad_value(93))));

        s.store_mul_ad_lhs(85, A::div_scaled_inputs(s.ad_value(309), -1.0, A::add(s.ad_value(2), s.ad_value(310)), 1.0), 73);

        s.v[35] = ((s.v[20] * s.v[19]) / (s.v[20] + s.v[19]));

        s.store_mul_ad_lhs(36, A::add_scaled_inputs(s.ad_value(293), 1.0, s.ad_value(28), p.p70), 73);

        s.store_add_scaled_value_products(37, s.ad_value(25), p.p66, s.ad_value(25), s.ad_value(25), p.p67, s.ad_value(88), A::add(A::add_scaled_value_products(s.ad_value(292), 1.0, s.ad_value(294), s.ad_value(25), 1.0, s.ad_value(25), s.ad_value(25), p.p69), s.ad_value(36)), 1.0);

        s.store_mul_scaled_ad_rhs(81, 55, 1.0 / ((s.v[17] + s.v[35])), A::add(A::offset(s.ad_value(291), (s.v[17] + s.v[35])), s.ad_value(37)));

        s.store_scale(60, 290, ((1.60219e-19 * (p.p49 * 1.0 / (s.v[17]))) * (1.0 - ((0.5 * p.p49) / (p.p49 + (s.v[21] * p.p46))))));

        s.store_mul_offset_ad_lhs(34, A::div_from_scalar(p.p304, s.ad_value(2)), p.p303, 25);

        s.store_add_scaled_offset_product_rhs(101, 102, 1.0, 34, 96, (-1.0), 1.0);

        s.store_add_ad_lhs(86, A::add(A::add(A::add_scaled_inputs4(s.ad_value(83), 1.0, s.ad_value(82), 1.0, s.ad_value(84), 1.0, s.ad_value(85), 1.0), s.ad_value(60)), s.ad_value(101)), 242);

        s.store_offset_sub(71, 69, 86, p.p10);

        s.store_div_scaled_inputs(421, s.ad_value(100), ((2.0 * 1.60219e-19) * (p.p49 * p.p49)), s.ad_value(55), s.v[16]);

        s.v[419] = (s.v[17] / s.v[20]);

        s.v[420] = (s.v[19] / s.v[20]);

        s.store_ln(449, 421);

        s.store_sub_from_scalar(450, ((39.47841) as f64).ln(), 449);

        s.v[451] = (s.v[419] * s.v[419]);

        s.v[454] = (s.v[419] / (((s.v[420] * s.v[419]) + s.v[420]) + s.v[419]));

        s.v[460] = 1.0;

        s.store_sub_from_scalar_ad(461, ((s.v[451] * s.v[460]) * s.v[460]), A::mul(s.ad_value(421), A::limited_exp_scaled_input(s.ad_value(50), 2.0)));

        s.store_sqrt(462, 461);

        s.store_div_ad(463, A::sub_from_scalar(1.0, A::scale(s.ad_value(462), 0.125)), A::sub_from_scalar(0.5, A::scale(s.ad_value(462), 0.041666666666666664)));

        s.store_mul_sub_ad_lhs(35, A::offset(A::ln(A::max_with_scalar(A::scale_offset(s.ad_value(463), (s.v[419] * s.v[460]), (((s.v[419] * s.v[419]) * s.v[460]) * s.v[460])), 1e-38)), 1.0), A::ln(A::max_with_scalar(s.ad_value(421), 1e-38)), 55);

        s.store_div(422, 71, 81);

        s.store_div_scaled_offset_numerator(423, A::sub(s.ad_value(70), s.ad_value(86)), 1.0, p.p10, s.ad_value(81), 1.0);

        s.store_sub_ad_lhs(453, A::ln(A::offset(A::mul_scaled_lhs(A::sub(s.ad_value(422), s.ad_value(450)), s.v[451], A::sub(s.ad_value(422), s.ad_value(450))), 39.47841)), 449);

        s.store_sub_ad_lhs(424, A::ln(A::offset(A::mul_scaled_lhs(A::sub(s.ad_value(422), s.ad_value(450)), s.v[451], A::sub(s.ad_value(422), s.ad_value(450))), 39.47841)), 449);

        s.store_add_scaled_inputs(452, 424, 1.0 / ((1.0 + s.v[420])), 423, (s.v[420] * 1.0 / ((1.0 + s.v[420]))));

        s.store_add_scaled_inputs3(426, s.ad_value(423), 1.0, s.ad_value(422), s.v[454], s.ad_value(423), (-s.v[454]));

        s.store_min(430, 426, 453);

        s.store_min(430, 430, 450);

        s.store_add_scaled_inputs(448, 430, 1.0 / ((1.0 + s.v[419])), 422, (s.v[419] * 1.0 / ((1.0 + s.v[419]))));

        s.store_sub(34, 448, 430);

        s.store_div_scaled_product_offset_rhs(37, A::limited_exp(s.ad_value(430)), A::limited_exp(s.ad_value(34)), (-1.0), 1.0, s.ad_value(34), 1.0);

        s.store_sub(429, 423, 452);

        s.store_add_scaled_products_right_right_ad(442, 429, 429, (s.v[420] * s.v[420]), 421, A::exp(s.ad_value(452)), (-1.0));

        s.b[600] = (s.v[442] < 0.0);
        s.v[600] = if s.b[600] { 1.0 } else { 0.0 };

        if s.b[600] {
            s.store_scaled_sub(429, 423, 430, s.v[420]);
            s.store_scalar(440, (40.0 * s.v[419]));
            s.store_add(455, 440, 429);
            s.store_mul(37, 440, 429);
            s.store_offset_scaled(38, 455, 0.06534, 1.0);
            s.store_offset_ad(39, A::add_scaled_inputs(s.ad_value(455), 8.57973, s.ad_value(37), 1.0), 39.47841);
            s.store_add_scaled_inputs(40, 455, 78.95683, 37, 39.47841);
            s.store_div_scaled_inputs2(442, A::sqrt(A::add_scaled_square_product(s.ad_value(39), 1.0, s.ad_value(38), s.ad_value(40), (-4.0))), 1.0, s.ad_value(39), (-1.0), s.ad_value(38), 2.0);
            s.store_sub_scaled_inputs(37, 450, ((1.0 + s.v[419]) * 1.0 / (s.v[419])), 430, 1.0 / (s.v[419]));
            s.store_offset_scaled_sub(34, 422, 37, (40.0 * 0.2), ((2.0) * ((40.0 * 0.2))));
            s.store_mul_sub_from_scalar_ad_rhs(442, 442, 1.0, A::exp_scaled_input(A::offset(A::sub(s.ad_value(422), s.ad_value(37)), 2.0), (-1.0 / ((2.0 / 0.69)))));
            s.store_min_with_scalar(442, 442, 50.0);
        }

        s.store_max(422, 422, 450);

        s.store_sub_ad_lhs(424, A::ln(A::offset(A::mul_scaled_lhs(A::sub(s.ad_value(422), s.ad_value(450)), s.v[451], A::sub(s.ad_value(422), s.ad_value(450))), 39.47841)), 449);

        s.store_sub_scaled_inputs(37, 450, ((1.0 + s.v[419]) * 1.0 / (s.v[419])), 430, 1.0 / (s.v[419]));

        s.store_sub_ad_lhs(38, A::ln(A::offset(A::mul_scaled_lhs(A::sub(s.ad_value(37), s.ad_value(450)), s.v[451], A::sub(s.ad_value(37), s.ad_value(450))), 39.47841)), 449);

        s.store_sub(39, 38, 450);

        s.store_sub(424, 424, 39);

        s.store_sub(440, 422, 424);

        s.store_mul_scaled_ad_rhs(34, 421, -1.0, A::exp(s.ad_value(424)));

        s.store_scale(35, 440, s.v[451]);

        s.store_div_scaled_inputs2(425, A::add_scaled_product(s.ad_value(34), 1.0, s.ad_value(35), s.ad_value(440), 1.0), -1.0, s.ad_value(442), 1.0, A::add_scaled_inputs(s.ad_value(35), (-2.0), s.ad_value(34), 1.0), 1.0);

        s.store_add(424, 424, 425);

        s.store_sub(440, 422, 424);

        s.store_scale(36, 440, s.v[451]);

        s.store_div_from_scalar_ad(34, 1.0, A::add_scaled_product(s.ad_value(442), (-1.0), s.ad_value(36), s.ad_value(440), 1.0));

        s.store_add_scaled_inputs3(465, A::ln(A::abs(A::add_scaled_product(s.ad_value(442), (-1.0), s.ad_value(36), s.ad_value(440), 1.0))), 1.0, s.ad_value(449), (-1.0), s.ad_value(424), -1.0);

        s.store_div_from_scalar_offset_ad(466, 1.0, A::mul_scaled_lhs(s.ad_value(36), (-2.0), s.ad_value(34)), (-1.0));

        s.store_add_scaled_product_left_ad(467, 34, (2.0 * s.v[451]), A::mul3_scaled_output(s.ad_value(36), s.ad_value(36), s.ad_value(34), (-4.0)), 34, 1.0);

        s.store_mul(35, 465, 466);

        s.store_add_scaled_product_left_ad(425, 35, -1.0, A::mul3_scaled_output(s.ad_value(35), s.ad_value(35), s.ad_value(467), 0.5), 466, (-1.0));

        s.store_max_with_scalar(425, 425, (-10.0));

        s.store_min_with_scalar(425, 425, 10.0);

        s.store_add(424, 424, 425);

        s.store_sub(440, 422, 424);

        s.store_scale(36, 440, s.v[451]);

        s.store_div_from_scalar_ad(34, 1.0, A::add_scaled_product(s.ad_value(442), (-1.0), s.ad_value(36), s.ad_value(440), 1.0));

        s.store_add_scaled_inputs3(465, A::ln(A::abs(A::add_scaled_product(s.ad_value(442), (-1.0), s.ad_value(36), s.ad_value(440), 1.0))), 1.0, s.ad_value(449), (-1.0), s.ad_value(424), -1.0);

        s.store_div_from_scalar_offset_ad(466, 1.0, A::mul_scaled_lhs(s.ad_value(36), (-2.0), s.ad_value(34)), (-1.0));

        s.store_add_scaled_product_left_ad(467, 34, (2.0 * s.v[451]), A::mul3_scaled_output(s.ad_value(36), s.ad_value(36), s.ad_value(34), (-4.0)), 34, 1.0);

        s.store_mul(35, 465, 466);

        s.store_add_scaled_product_left_ad(425, 35, -1.0, A::mul3_scaled_output(s.ad_value(35), s.ad_value(35), s.ad_value(467), 0.5), 466, (-1.0));

        s.store_max_with_scalar(425, 425, (-10.0));

        s.store_min_with_scalar(425, 425, 10.0);

        s.store_add(424, 424, 425);

        s.store_max_ad(424, s.ad_value(424), A::offset(s.ad_value(450), (-4.0)));

        s.store_div(422, 71, 81);

        s.store_sub_ad_rhs(448, 448, A::ln_one_plus_exp(A::sub_scaled_inputs(s.ad_value(448), 1.0, s.ad_value(424), 1.05)));

        s.store_min(448, 448, 424);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_scaled_ad_rhs(457, 421, -1.0, A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.b[601] = (s.v[442] < 0.0);
        s.v[601] = if s.b[601] { 1.0 } else { 0.0 };

        if s.b[601] {
            s.store_sqrt_neg_input(439, 442);
            s.store_div_from_scalar_sin_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_mul_cos_ad_lhs(458, A::scale(s.ad_value(439), 0.5), 459);
            s.store_div_scaled_inputs(34, s.ad_value(458), (-0.5), s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, 0.25, 34, 1.0);
        }

        if (!s.b[601]) {
            s.store_sqrt(439, 442);
            s.store_div_from_scalar_sinh_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_sqrt_offset_input(458, 35, 1.0);
            s.store_div_scaled_inputs(34, s.ad_value(458), 0.5, s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, (-0.25), 34, 1.0);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_add_scaled_inputs4(429, s.ad_value(423), 1.0, s.ad_value(422), (-1.0), s.ad_value(440), 1.0, A::ln(A::abs(A::mul(A::mul3(s.ad_value(442), s.ad_value(35), s.ad_value(37)), s.ad_value(37)))), -1.0);

        s.store_add_scaled_product_mixed_iaa(427, 457, 1.0, A::add(s.ad_value(456), s.ad_value(446)), A::add_scaled_inputs(s.ad_value(429), s.v[420], s.ad_value(456), 1.0), 1.0);

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_scaled_inputs(443, 456, ((-2.0) * s.v[419]), 457, 1.0);

        s.store_mul(444, 445, 443);

        s.store_add_scaled_product_value_ad(441, A::offset(A::mul_scaled_output(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37), 2.0), (-1.0)), 1.0, 447, 443, (-1.0));

        s.store_add_ad(428, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(457), 1.0, s.ad_value(456), (-s.v[419]), s.ad_value(36), (-s.v[419])), 1.0, s.ad_value(456), s.ad_value(444), 1.0), A::add_scaled_products(s.ad_value(441), s.ad_value(36), s.v[420], s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])), s.v[420]));

        s.store_div_scaled_inputs(425, s.ad_value(427), -1.0, s.ad_value(428), 1.0);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_scaled_ad_rhs(457, 421, -1.0, A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.b[602] = (s.v[442] < 0.0);
        s.v[602] = if s.b[602] { 1.0 } else { 0.0 };

        if s.b[602] {
            s.store_sqrt_neg_input(439, 442);
            s.store_div_from_scalar_sin_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_mul_cos_ad_lhs(458, A::scale(s.ad_value(439), 0.5), 459);
            s.store_div_scaled_inputs(34, s.ad_value(458), (-0.5), s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, 0.25, 34, 1.0);
        }

        if (!s.b[602]) {
            s.store_sqrt(439, 442);
            s.store_div_from_scalar_sinh_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_sqrt_offset_input(458, 35, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[602]) {
            s.store_div_scaled_inputs(34, s.ad_value(458), 0.5, s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, (-0.25), 34, 1.0);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_add_scaled_inputs4(429, s.ad_value(423), 1.0, s.ad_value(422), (-1.0), s.ad_value(440), 1.0, A::ln(A::abs(A::mul(A::mul3(s.ad_value(442), s.ad_value(35), s.ad_value(37)), s.ad_value(37)))), -1.0);

        s.store_add_scaled_product_mixed_iaa(427, 457, 1.0, A::add(s.ad_value(456), s.ad_value(446)), A::add_scaled_inputs(s.ad_value(429), s.v[420], s.ad_value(456), 1.0), 1.0);

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_scaled_inputs(443, 456, ((-2.0) * s.v[419]), 457, 1.0);

        s.store_mul(444, 445, 443);

        s.store_add_scaled_product_value_ad(441, A::offset(A::mul_scaled_output(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37), 2.0), (-1.0)), 1.0, 447, 443, (-1.0));

        s.store_add_ad(428, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(457), 1.0, s.ad_value(456), (-s.v[419]), s.ad_value(36), (-s.v[419])), 1.0, s.ad_value(456), s.ad_value(444), 1.0), A::add_scaled_products(s.ad_value(441), s.ad_value(36), s.v[420], s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])), s.v[420]));

        s.store_div_scaled_inputs(425, s.ad_value(427), -1.0, s.ad_value(428), 1.0);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_scaled_ad_rhs(457, 421, -1.0, A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.b[603] = (s.v[442] < 0.0);
        s.v[603] = if s.b[603] { 1.0 } else { 0.0 };

        if s.b[603] {
            s.store_sqrt_neg_input(439, 442);
            s.store_div_from_scalar_sin_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_mul_cos_ad_lhs(458, A::scale(s.ad_value(439), 0.5), 459);
            s.store_div_scaled_inputs(34, s.ad_value(458), (-0.5), s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, 0.25, 34, 1.0);
        }

        if (!s.b[603]) {
            s.store_sqrt(439, 442);
            s.store_div_from_scalar_sinh_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_sqrt_offset_input(458, 35, 1.0);
            s.store_div_scaled_inputs(34, s.ad_value(458), 0.5, s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, (-0.25), 34, 1.0);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_add_scaled_inputs4(429, s.ad_value(423), 1.0, s.ad_value(422), (-1.0), s.ad_value(440), 1.0, A::ln(A::abs(A::mul(A::mul3(s.ad_value(442), s.ad_value(35), s.ad_value(37)), s.ad_value(37)))), -1.0);

        s.store_add_scaled_product_mixed_iaa(427, 457, 1.0, A::add(s.ad_value(456), s.ad_value(446)), A::add_scaled_inputs(s.ad_value(429), s.v[420], s.ad_value(456), 1.0), 1.0);

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_scaled_inputs(443, 456, ((-2.0) * s.v[419]), 457, 1.0);

        s.store_mul(444, 445, 443);

        s.store_add_scaled_product_value_ad(441, A::offset(A::mul_scaled_output(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37), 2.0), (-1.0)), 1.0, 447, 443, (-1.0));

        s.store_add_ad(428, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(457), 1.0, s.ad_value(456), (-s.v[419]), s.ad_value(36), (-s.v[419])), 1.0, s.ad_value(456), s.ad_value(444), 1.0), A::add_scaled_products(s.ad_value(441), s.ad_value(36), s.v[420], s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])), s.v[420]));

        s.store_div_scaled_inputs(425, s.ad_value(427), -1.0, s.ad_value(428), 1.0);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_scaled_ad_rhs(457, 421, -1.0, A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.b[604] = (s.v[442] < 0.0);
        s.v[604] = if s.b[604] { 1.0 } else { 0.0 };

        if s.b[604] {
            s.store_sqrt_neg_input(439, 442);
            s.store_div_from_scalar_sin_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_mul_cos_ad_lhs(458, A::scale(s.ad_value(439), 0.5), 459);
            s.store_div_scaled_inputs(34, s.ad_value(458), (-0.5), s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, 0.25, 34, 1.0);
        }

        if (!s.b[604]) {
            s.store_sqrt(439, 442);
            s.store_div_from_scalar_sinh_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_sqrt_offset_input(458, 35, 1.0);
            s.store_div_scaled_inputs(34, s.ad_value(458), 0.5, s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, (-0.25), 34, 1.0);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_add_scaled_inputs4(429, s.ad_value(423), 1.0, s.ad_value(422), (-1.0), s.ad_value(440), 1.0, A::ln(A::abs(A::mul(A::mul3(s.ad_value(442), s.ad_value(35), s.ad_value(37)), s.ad_value(37)))), -1.0);

        s.store_add_scaled_product_mixed_iaa(427, 457, 1.0, A::add(s.ad_value(456), s.ad_value(446)), A::add_scaled_inputs(s.ad_value(429), s.v[420], s.ad_value(456), 1.0), 1.0);

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_scaled_inputs(443, 456, ((-2.0) * s.v[419]), 457, 1.0);

        s.store_mul(444, 445, 443);

        s.store_add_scaled_product_value_ad(441, A::offset(A::mul_scaled_output(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37), 2.0), (-1.0)), 1.0, 447, 443, (-1.0));

        s.store_add_ad(428, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(457), 1.0, s.ad_value(456), (-s.v[419]), s.ad_value(36), (-s.v[419])), 1.0, s.ad_value(456), s.ad_value(444), 1.0), A::add_scaled_products(s.ad_value(441), s.ad_value(36), s.v[420], s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])), s.v[420]));

        s.store_div_scaled_inputs(425, s.ad_value(427), -1.0, s.ad_value(428), 1.0);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_scaled_ad_rhs(457, 421, -1.0, A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.b[605] = (s.v[442] < 0.0);
        s.v[605] = if s.b[605] { 1.0 } else { 0.0 };

        if s.b[605] {
            s.store_sqrt_neg_input(439, 442);
            s.store_div_from_scalar_sin_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_mul_cos_ad_lhs(458, A::scale(s.ad_value(439), 0.5), 459);
            s.store_div_scaled_inputs(34, s.ad_value(458), (-0.5), s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, 0.25, 34, 1.0);
        }

        if (!s.b[605]) {
            s.store_sqrt(439, 442);
            s.store_div_from_scalar_sinh_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_sqrt_offset_input(458, 35, 1.0);
            s.store_div_scaled_inputs(34, s.ad_value(458), 0.5, s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, (-0.25), 34, 1.0);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_add_scaled_inputs4(429, s.ad_value(423), 1.0, s.ad_value(422), (-1.0), s.ad_value(440), 1.0, A::ln(A::abs(A::mul(A::mul3(s.ad_value(442), s.ad_value(35), s.ad_value(37)), s.ad_value(37)))), -1.0);

        s.store_add_scaled_product_mixed_iaa(427, 457, 1.0, A::add(s.ad_value(456), s.ad_value(446)), A::add_scaled_inputs(s.ad_value(429), s.v[420], s.ad_value(456), 1.0), 1.0);

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_scaled_inputs(443, 456, ((-2.0) * s.v[419]), 457, 1.0);

        s.store_mul(444, 445, 443);

        s.store_add_scaled_product_value_ad(441, A::offset(A::mul_scaled_output(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37), 2.0), (-1.0)), 1.0, 447, 443, (-1.0));

        s.store_add_ad(428, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(457), 1.0, s.ad_value(456), (-s.v[419]), s.ad_value(36), (-s.v[419])), 1.0, s.ad_value(456), s.ad_value(444), 1.0), A::add_scaled_products(s.ad_value(441), s.ad_value(36), s.v[420], s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])), s.v[420]));

        s.store_div_scaled_inputs(425, s.ad_value(427), -1.0, s.ad_value(428), 1.0);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_mul_exp_rhs(34, 421, 448);

        s.store_add_scaled_product_indices(442, 34, (-1.0), 440, 440, s.v[451]);

        s.b[606] = (s.v[442] < 0.0);
        s.v[606] = if s.b[606] { 1.0 } else { 0.0 };

        if s.b[606] {
            s.store_sqrt_neg_input(439, 442);
            s.store_scale(36, 439, 0.5);
            s.store_div_ad_rhs(446, 439, A::tan(s.ad_value(36)));
            s.store_sin(40, 36);
            s.store_mul_neg_lhs(35, 40, 40);
        }

        if (!s.b[606]) {
            s.store_sqrt(439, 442);
            s.store_scale(36, 439, 0.5);
            s.store_sinh(40, 36);
            s.store_square(35, 40);
            s.store_div_ad_rhs(446, 439, A::tanh(s.ad_value(36)));
        }

        s.store_div_scaled_inputs2(437, s.ad_value(440), s.v[419], s.ad_value(446), (-1.0), A::sub_from_scalar(1.0, A::div(s.ad_value(442), A::mul(s.ad_value(35), s.ad_value(34)))), 1.0);

        s.store_scaled_mul(431, 440, 81, s.v[17]);

        s.store_scaled_mul(435, 437, 81, s.v[20]);

        s.store_sub(433, 435, 431);

        s.store_sub_ad_rhs(430, 423, A::div_scaled_inputs(s.ad_value(433), 1.0, s.ad_value(81), s.v[19]));

        s.store_mul_scaled_ad_lhs(210, A::add(s.ad_value(448), s.ad_value(430)), 81, 0.5);

        s.store_scale(109, 435, 1.0 / (s.v[17]));

        s.store_scale(111, 290, (1.60219e-19 * (p.p49 * 1.0 / (s.v[17]))));

        s.store_add_scaled_product_indices(36, 111, 1.0, 114, 431, 1.0 / (s.v[17]));

        s.store_scaled_add_ad_rhs(37, 36, A::sqrt(A::offset(A::square(s.ad_value(36)), 0.001)), 0.5);

        s.store_scale(127, 37, s.v[129]);

        s.store_add_scaled_product_indices(36, 111, 1.0, 143, 433, 1.0 / (s.v[19]));

        s.store_scaled_add_ad_rhs(37, 36, A::sqrt(A::offset(A::square(s.ad_value(36)), 0.001)), 0.5);

        s.store_scale(128, 37, s.v[144]);

        s.v[59] = (0.01 / s.v[17]);

        s.store_pow_ad(607, A::scaled_offset(A::abs_scaled_input(s.ad_value(109), 1.0 / (s.v[59])), 1.0, 0.5), s.ad_value(124));

        s.store_add_scaled_product(608, A::div(s.ad_value(125), s.ad_value(607)), 1.0, A::add_scaled_product(s.ad_value(122), 1.0, s.ad_value(23), s.ad_value(123), 1.0), A::pow(A::abs(s.ad_value(127)), A::add_scaled_product(s.ad_value(336), 1.0, s.ad_value(342), s.ad_value(23), 1.0)), 1.0);

        s.store_offset(112, 608, 1.0);

        s.store_scaled_add_ad(112, A::offset(s.ad_value(112), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(112), (-1.0), A::offset(s.ad_value(112), (-1.0))), ((0.25 * p.p154) * p.p154))), 0.5);

        s.store_scale(112, 112, 1.0 / (p.p11));

        s.store_div(141, 126, 112);

        s.store_pow_ad(609, A::scaled_offset(A::abs_scaled_input(s.ad_value(109), 1.0 / (s.v[59])), 1.0, 0.5), s.ad_value(348));

        s.store_add_scaled_product(610, A::div(s.ad_value(347), s.ad_value(609)), 1.0, A::add_scaled_product(s.ad_value(345), 1.0, s.ad_value(23), s.ad_value(346), 1.0), A::pow(A::abs(s.ad_value(128)), A::add_scaled_product(s.ad_value(349), 1.0, s.ad_value(350), s.ad_value(23), 1.0)), 1.0);

        s.store_offset(112, 610, 1.0);

        s.store_scaled_add_ad(112, A::offset(s.ad_value(112), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(112), (-1.0), A::offset(s.ad_value(112), (-1.0))), ((0.25 * p.p154) * p.p154))), 0.5);

        s.store_scale(112, 112, 1.0 / (p.p11));

        s.store_div(142, 344, 112);

        s.store_sub_scaled_inputs(34, 71, 1.0, 431, 1.0 / (s.v[17]));

        s.store_add_scaled_inputs3(35, s.ad_value(70), 1.0, s.ad_value(86), (-1.0), s.ad_value(433), (-1.0 / (s.v[19])));

        s.store_div_ad(139, A::exp(A::div(s.ad_value(34), s.ad_value(81))), A::add(A::exp(A::div(s.ad_value(34), s.ad_value(81))), A::exp(A::div(s.ad_value(35), s.ad_value(81)))));

        s.store_div_ad(140, A::exp(A::div(s.ad_value(35), s.ad_value(81))), A::add(A::exp(A::div(s.ad_value(34), s.ad_value(81))), A::exp(A::div(s.ad_value(35), s.ad_value(81)))));

        s.store_add_scaled_products_indices(121, 139, 141, 1.0, 140, 142, 1.0);

        s.b[611] = (p.p14 == 1.0);
        s.v[611] = if s.b[611] { 1.0 } else { 0.0 };

        if s.b[611] {
            s.store_scalar(152, 0.0);
        }

        s.b[612] = (p.p14 == 0.0);
        s.v[612] = if s.b[612] { 1.0 } else { 0.0 };

        if ((!s.b[611]) && s.b[612]) {
            s.store_offset_mul(38, 284, 109, 1.0);
            s.store_div_from_scalar(35, 1.0, 38);
            s.store_scaled_add_ad_rhs(34, 35, A::sqrt(A::offset(A::square(s.ad_value(35)), 0.01)), 0.5);
            s.store_mul_ad_affine_product_lhs(152, A::add_scaled_product(s.ad_value(134), 1.0, s.ad_value(281), s.ad_value(34), 1.0), s.ad_value(131), p.p2, 0.0, 150);
        }

        if ((!s.b[611]) && (!s.b[612])) {
            s.store_offset_mul(38, 284, 109, 1.0);
            s.store_div_from_scalar(35, 1.0, 38);
            s.store_scaled_add_ad_rhs(34, 35, A::sqrt(A::offset(A::square(s.ad_value(35)), 0.01)), 0.5);
            s.store_mul_ad_affine_product_lhs(152, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(132), 1.0, s.ad_value(133), 1.0, s.ad_value(134), 1.0), 1.0, s.ad_value(281), s.ad_value(34), 1.0), s.ad_value(131), p.p2, 0.0, 150);
        }

        s.store_div_scaled_inputs(169, s.ad_value(164), 2.0, s.ad_value(121), 1.0);

        s.store_mul(170, 169, 2);

        s.store_mul_ad_rhs(40, 404, A::add_scaled_value_products(s.ad_value(109), 1.0, s.ad_value(407), s.ad_value(28), 1.0, s.ad_value(55), s.ad_value(405), 2.0));

        s.b[613] = (s.v[152] == 0.0);
        s.v[613] = if s.b[613] { 1.0 } else { 0.0 };

        if s.b[613] {
            s.store_div_scaled_product_denominator_ad(162, 170, 40, 1.0, A::add(s.ad_value(170), s.ad_value(40)), 1.0);
        }

        if (!s.b[613]) {
            s.store_scaled_mul(177, 3, 164, s.v[17]);
            s.store_mul(34, 177, 152);
            s.store_scale(178, 34, 2.0);
            s.store_add_scaled_inputs_product_indices(179, 40, 1.0, 170, 1.0, 40, 34, 3.0);
            s.store_mul_ad_rhs(180, 40, A::add_scaled_product(s.ad_value(170), 1.0, s.ad_value(40), s.ad_value(34), 2.0));
            s.store_div_scaled_inputs2(162, s.ad_value(179), 1.0, A::sqrt(A::add_scaled_square_product(s.ad_value(179), 1.0, s.ad_value(178), s.ad_value(180), (-2.0))), (-1.0), s.ad_value(178), 1.0);
        }

        s.store_offset_ad(162, A::add_scaled_inputs(A::offset(s.ad_value(162), (-0.001)), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(162), (-0.001), A::offset(s.ad_value(162), (-0.001))), ((4.0 * 1e-5) * 1e-5))), 0.5), 0.001);

        s.store_pow_ad(41, A::div(s.ad_value(26), s.ad_value(162)), s.ad_value(168));

        s.store_pow_ad(42, A::offset(s.ad_value(41), 1.0), s.ad_value(163));

        s.store_div(113, 26, 42);

        s.b[614] = (s.v[113] > s.v[26]);
        s.v[614] = if s.b[614] { 1.0 } else { 0.0 };

        if s.b[614] {
            s.copy_ad(113, 26);
        }

        s.store_div_scaled_inputs2(422, s.ad_value(71), 1.0, s.ad_value(113), (-1.0), s.ad_value(81), 1.0);

        s.store_div_ad_lhs(423, A::add_scaled_inputs3_offset(s.ad_value(70), 1.0, s.ad_value(86), (-1.0), s.ad_value(113), -1.0, p.p10), 81);

        s.store_sub_ad_lhs(453, A::ln(A::offset(A::mul_scaled_lhs(A::sub(s.ad_value(422), s.ad_value(450)), s.v[451], A::sub(s.ad_value(422), s.ad_value(450))), 39.47841)), 449);

        s.store_sub_ad_lhs(424, A::ln(A::offset(A::mul_scaled_lhs(A::sub(s.ad_value(422), s.ad_value(450)), s.v[451], A::sub(s.ad_value(422), s.ad_value(450))), 39.47841)), 449);

        s.store_sub_scaled_inputs(37, 450, ((1.0 + s.v[419]) * 1.0 / (s.v[419])), 430, 1.0 / (s.v[419]));

        s.store_sub_ad_lhs(38, A::ln(A::offset(A::mul_scaled_lhs(A::sub(s.ad_value(37), s.ad_value(450)), s.v[451], A::sub(s.ad_value(37), s.ad_value(450))), 39.47841)), 449);

        s.store_sub(39, 38, 450);

        s.store_sub(424, 424, 39);

        s.store_add_scaled_inputs(452, 424, 1.0 / ((1.0 + s.v[420])), 423, (s.v[420] * 1.0 / ((1.0 + s.v[420]))));

        s.store_add_scaled_inputs3(426, s.ad_value(423), 1.0, s.ad_value(422), s.v[454], s.ad_value(423), (-s.v[454]));

        s.store_min(430, 426, 453);

        s.store_min(430, 430, 450);

    }

    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
    ) {
        s.store_add_scaled_inputs(448, 430, 1.0 / ((1.0 + s.v[419])), 422, (s.v[419] * 1.0 / ((1.0 + s.v[419]))));

        s.store_sub(34, 448, 430);

        s.store_div_scaled_product_offset_rhs(37, A::limited_exp(s.ad_value(430)), A::limited_exp(s.ad_value(34)), (-1.0), 1.0, s.ad_value(34), 1.0);

        s.store_sub(429, 423, 452);

        s.store_add_scaled_products_right_right_ad(442, 429, 429, (s.v[420] * s.v[420]), 421, A::exp(s.ad_value(452)), (-1.0));

        s.b[615] = (s.v[442] < 0.0);
        s.v[615] = if s.b[615] { 1.0 } else { 0.0 };

        if s.b[615] {
            s.store_scaled_sub(429, 423, 430, s.v[420]);
            s.store_scalar(440, (40.0 * s.v[419]));
            s.store_add(455, 440, 429);
            s.store_mul(37, 440, 429);
            s.store_offset_scaled(38, 455, 0.06534, 1.0);
            s.store_offset_ad(39, A::add_scaled_inputs(s.ad_value(455), 8.57973, s.ad_value(37), 1.0), 39.47841);
            s.store_add_scaled_inputs(40, 455, 78.95683, 37, 39.47841);
            s.store_div_scaled_inputs2(442, A::sqrt(A::add_scaled_square_product(s.ad_value(39), 1.0, s.ad_value(38), s.ad_value(40), (-4.0))), 1.0, s.ad_value(39), (-1.0), s.ad_value(38), 2.0);
            s.store_sub_scaled_inputs(37, 450, ((1.0 + s.v[419]) * 1.0 / (s.v[419])), 430, 1.0 / (s.v[419]));
            s.store_offset_scaled_sub(34, 422, 37, (40.0 * 0.2), ((2.0) * ((40.0 * 0.2))));
            s.store_mul_sub_from_scalar_ad_rhs(442, 442, 1.0, A::exp_scaled_input(A::offset(A::sub(s.ad_value(422), s.ad_value(37)), 2.0), (-1.0 / ((2.0 / 0.69)))));
            s.store_min_with_scalar(442, 442, 50.0);
        }

        s.store_max(422, 422, 450);

        s.store_sub_ad_lhs(424, A::ln(A::offset(A::mul_scaled_lhs(A::sub(s.ad_value(422), s.ad_value(450)), s.v[451], A::sub(s.ad_value(422), s.ad_value(450))), 39.47841)), 449);

        s.store_sub_scaled_inputs(37, 450, ((1.0 + s.v[419]) * 1.0 / (s.v[419])), 430, 1.0 / (s.v[419]));

        s.store_sub_ad_lhs(38, A::ln(A::offset(A::mul_scaled_lhs(A::sub(s.ad_value(37), s.ad_value(450)), s.v[451], A::sub(s.ad_value(37), s.ad_value(450))), 39.47841)), 449);

        s.store_sub(39, 38, 450);

        s.store_sub(424, 424, 39);

        s.store_sub(440, 422, 424);

        s.store_mul_scaled_ad_rhs(34, 421, -1.0, A::exp(s.ad_value(424)));

        s.store_scale(35, 440, s.v[451]);

        s.store_div_scaled_inputs2(425, A::add_scaled_product(s.ad_value(34), 1.0, s.ad_value(35), s.ad_value(440), 1.0), -1.0, s.ad_value(442), 1.0, A::add_scaled_inputs(s.ad_value(35), (-2.0), s.ad_value(34), 1.0), 1.0);

        s.store_add(424, 424, 425);

        s.store_sub(440, 422, 424);

        s.store_scale(36, 440, s.v[451]);

        s.store_div_from_scalar_ad(34, 1.0, A::add_scaled_product(s.ad_value(442), (-1.0), s.ad_value(36), s.ad_value(440), 1.0));

        s.store_add_scaled_inputs3(465, A::ln(A::abs(A::add_scaled_product(s.ad_value(442), (-1.0), s.ad_value(36), s.ad_value(440), 1.0))), 1.0, s.ad_value(449), (-1.0), s.ad_value(424), -1.0);

        s.store_div_from_scalar_offset_ad(466, 1.0, A::mul_scaled_lhs(s.ad_value(36), (-2.0), s.ad_value(34)), (-1.0));

        s.store_add_scaled_product_left_ad(467, 34, (2.0 * s.v[451]), A::mul3_scaled_output(s.ad_value(36), s.ad_value(36), s.ad_value(34), (-4.0)), 34, 1.0);

        s.store_mul(35, 465, 466);

        s.store_add_scaled_product_left_ad(425, 35, -1.0, A::mul3_scaled_output(s.ad_value(35), s.ad_value(35), s.ad_value(467), 0.5), 466, (-1.0));

        s.store_max_with_scalar(425, 425, (-10.0));

        s.store_min_with_scalar(425, 425, 10.0);

        s.store_add(424, 424, 425);

        s.store_sub(440, 422, 424);

        s.store_scale(36, 440, s.v[451]);

        s.store_div_from_scalar_ad(34, 1.0, A::add_scaled_product(s.ad_value(442), (-1.0), s.ad_value(36), s.ad_value(440), 1.0));

        s.store_add_scaled_inputs3(465, A::ln(A::abs(A::add_scaled_product(s.ad_value(442), (-1.0), s.ad_value(36), s.ad_value(440), 1.0))), 1.0, s.ad_value(449), (-1.0), s.ad_value(424), -1.0);

        s.store_div_from_scalar_offset_ad(466, 1.0, A::mul_scaled_lhs(s.ad_value(36), (-2.0), s.ad_value(34)), (-1.0));

        s.store_add_scaled_product_left_ad(467, 34, (2.0 * s.v[451]), A::mul3_scaled_output(s.ad_value(36), s.ad_value(36), s.ad_value(34), (-4.0)), 34, 1.0);

        s.store_mul(35, 465, 466);

        s.store_add_scaled_product_left_ad(425, 35, -1.0, A::mul3_scaled_output(s.ad_value(35), s.ad_value(35), s.ad_value(467), 0.5), 466, (-1.0));

        s.store_max_with_scalar(425, 425, (-10.0));

        s.store_min_with_scalar(425, 425, 10.0);

        s.store_add(424, 424, 425);

        s.store_max_ad(424, s.ad_value(424), A::offset(s.ad_value(450), (-4.0)));

        s.store_div_scaled_inputs2(422, s.ad_value(71), 1.0, s.ad_value(113), (-1.0), s.ad_value(81), 1.0);

        s.store_sub_ad_rhs(448, 448, A::ln_one_plus_exp(A::sub_scaled_inputs(s.ad_value(448), 1.0, s.ad_value(424), 1.05)));

        s.store_min(448, 448, 424);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_scaled_ad_rhs(457, 421, -1.0, A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.b[616] = (s.v[442] < 0.0);
        s.v[616] = if s.b[616] { 1.0 } else { 0.0 };

        if s.b[616] {
            s.store_sqrt_neg_input(439, 442);
            s.store_div_from_scalar_sin_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_mul_cos_ad_lhs(458, A::scale(s.ad_value(439), 0.5), 459);
            s.store_div_scaled_inputs(34, s.ad_value(458), (-0.5), s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, 0.25, 34, 1.0);
        }

        if (!s.b[616]) {
            s.store_sqrt(439, 442);
            s.store_div_from_scalar_sinh_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_sqrt_offset_input(458, 35, 1.0);
            s.store_div_scaled_inputs(34, s.ad_value(458), 0.5, s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, (-0.25), 34, 1.0);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_add_scaled_inputs4(429, s.ad_value(423), 1.0, s.ad_value(422), (-1.0), s.ad_value(440), 1.0, A::ln(A::abs(A::mul(A::mul3(s.ad_value(442), s.ad_value(35), s.ad_value(37)), s.ad_value(37)))), -1.0);

        s.store_add_scaled_product_mixed_iaa(427, 457, 1.0, A::add(s.ad_value(456), s.ad_value(446)), A::add_scaled_inputs(s.ad_value(429), s.v[420], s.ad_value(456), 1.0), 1.0);

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_scaled_inputs(443, 456, ((-2.0) * s.v[419]), 457, 1.0);

        s.store_mul(444, 445, 443);

        s.store_add_scaled_product_value_ad(441, A::offset(A::mul_scaled_output(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37), 2.0), (-1.0)), 1.0, 447, 443, (-1.0));

        s.store_add_ad(428, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(457), 1.0, s.ad_value(456), (-s.v[419]), s.ad_value(36), (-s.v[419])), 1.0, s.ad_value(456), s.ad_value(444), 1.0), A::add_scaled_products(s.ad_value(441), s.ad_value(36), s.v[420], s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])), s.v[420]));

        s.store_div_scaled_inputs(425, s.ad_value(427), -1.0, s.ad_value(428), 1.0);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_scaled_ad_rhs(457, 421, -1.0, A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.b[617] = (s.v[442] < 0.0);
        s.v[617] = if s.b[617] { 1.0 } else { 0.0 };

        if s.b[617] {
            s.store_sqrt_neg_input(439, 442);
            s.store_div_from_scalar_sin_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_mul_cos_ad_lhs(458, A::scale(s.ad_value(439), 0.5), 459);
            s.store_div_scaled_inputs(34, s.ad_value(458), (-0.5), s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, 0.25, 34, 1.0);
        }

        if (!s.b[617]) {
            s.store_sqrt(439, 442);
            s.store_div_from_scalar_sinh_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_sqrt_offset_input(458, 35, 1.0);
            s.store_div_scaled_inputs(34, s.ad_value(458), 0.5, s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, (-0.25), 34, 1.0);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_add_scaled_inputs4(429, s.ad_value(423), 1.0, s.ad_value(422), (-1.0), s.ad_value(440), 1.0, A::ln(A::abs(A::mul(A::mul3(s.ad_value(442), s.ad_value(35), s.ad_value(37)), s.ad_value(37)))), -1.0);

        s.store_add_scaled_product_mixed_iaa(427, 457, 1.0, A::add(s.ad_value(456), s.ad_value(446)), A::add_scaled_inputs(s.ad_value(429), s.v[420], s.ad_value(456), 1.0), 1.0);

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_scaled_inputs(443, 456, ((-2.0) * s.v[419]), 457, 1.0);

        s.store_mul(444, 445, 443);

        s.store_add_scaled_product_value_ad(441, A::offset(A::mul_scaled_output(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37), 2.0), (-1.0)), 1.0, 447, 443, (-1.0));

        s.store_add_ad(428, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(457), 1.0, s.ad_value(456), (-s.v[419]), s.ad_value(36), (-s.v[419])), 1.0, s.ad_value(456), s.ad_value(444), 1.0), A::add_scaled_products(s.ad_value(441), s.ad_value(36), s.v[420], s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])), s.v[420]));

        s.store_div_scaled_inputs(425, s.ad_value(427), -1.0, s.ad_value(428), 1.0);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_scaled_ad_rhs(457, 421, -1.0, A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.b[618] = (s.v[442] < 0.0);
        s.v[618] = if s.b[618] { 1.0 } else { 0.0 };

        if s.b[618] {
            s.store_sqrt_neg_input(439, 442);
            s.store_div_from_scalar_sin_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_mul_cos_ad_lhs(458, A::scale(s.ad_value(439), 0.5), 459);
            s.store_div_scaled_inputs(34, s.ad_value(458), (-0.5), s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, 0.25, 34, 1.0);
        }

        if (!s.b[618]) {
            s.store_sqrt(439, 442);
            s.store_div_from_scalar_sinh_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_sqrt_offset_input(458, 35, 1.0);
            s.store_div_scaled_inputs(34, s.ad_value(458), 0.5, s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, (-0.25), 34, 1.0);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_add_scaled_inputs4(429, s.ad_value(423), 1.0, s.ad_value(422), (-1.0), s.ad_value(440), 1.0, A::ln(A::abs(A::mul(A::mul3(s.ad_value(442), s.ad_value(35), s.ad_value(37)), s.ad_value(37)))), -1.0);

        s.store_add_scaled_product_mixed_iaa(427, 457, 1.0, A::add(s.ad_value(456), s.ad_value(446)), A::add_scaled_inputs(s.ad_value(429), s.v[420], s.ad_value(456), 1.0), 1.0);

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_scaled_inputs(443, 456, ((-2.0) * s.v[419]), 457, 1.0);

        s.store_mul(444, 445, 443);

        s.store_add_scaled_product_value_ad(441, A::offset(A::mul_scaled_output(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37), 2.0), (-1.0)), 1.0, 447, 443, (-1.0));

        s.store_add_ad(428, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(457), 1.0, s.ad_value(456), (-s.v[419]), s.ad_value(36), (-s.v[419])), 1.0, s.ad_value(456), s.ad_value(444), 1.0), A::add_scaled_products(s.ad_value(441), s.ad_value(36), s.v[420], s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])), s.v[420]));

        s.store_div_scaled_inputs(425, s.ad_value(427), -1.0, s.ad_value(428), 1.0);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_scaled_ad_rhs(457, 421, -1.0, A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.b[619] = (s.v[442] < 0.0);
        s.v[619] = if s.b[619] { 1.0 } else { 0.0 };

        if s.b[619] {
            s.store_sqrt_neg_input(439, 442);
            s.store_div_from_scalar_sin_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_mul_cos_ad_lhs(458, A::scale(s.ad_value(439), 0.5), 459);
            s.store_div_scaled_inputs(34, s.ad_value(458), (-0.5), s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, 0.25, 34, 1.0);
        }

        if (!s.b[619]) {
            s.store_sqrt(439, 442);
            s.store_div_from_scalar_sinh_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_sqrt_offset_input(458, 35, 1.0);
            s.store_div_scaled_inputs(34, s.ad_value(458), 0.5, s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, (-0.25), 34, 1.0);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_add_scaled_inputs4(429, s.ad_value(423), 1.0, s.ad_value(422), (-1.0), s.ad_value(440), 1.0, A::ln(A::abs(A::mul(A::mul3(s.ad_value(442), s.ad_value(35), s.ad_value(37)), s.ad_value(37)))), -1.0);

        s.store_add_scaled_product_mixed_iaa(427, 457, 1.0, A::add(s.ad_value(456), s.ad_value(446)), A::add_scaled_inputs(s.ad_value(429), s.v[420], s.ad_value(456), 1.0), 1.0);

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_scaled_inputs(443, 456, ((-2.0) * s.v[419]), 457, 1.0);

        s.store_mul(444, 445, 443);

        s.store_add_scaled_product_value_ad(441, A::offset(A::mul_scaled_output(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37), 2.0), (-1.0)), 1.0, 447, 443, (-1.0));

        s.store_add_ad(428, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(457), 1.0, s.ad_value(456), (-s.v[419]), s.ad_value(36), (-s.v[419])), 1.0, s.ad_value(456), s.ad_value(444), 1.0), A::add_scaled_products(s.ad_value(441), s.ad_value(36), s.v[420], s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])), s.v[420]));

        s.store_div_scaled_inputs(425, s.ad_value(427), -1.0, s.ad_value(428), 1.0);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_scaled_ad_rhs(457, 421, -1.0, A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.b[620] = (s.v[442] < 0.0);
        s.v[620] = if s.b[620] { 1.0 } else { 0.0 };

        if s.b[620] {
            s.store_sqrt_neg_input(439, 442);
            s.store_div_from_scalar_sin_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_mul_cos_ad_lhs(458, A::scale(s.ad_value(439), 0.5), 459);
            s.store_div_scaled_inputs(34, s.ad_value(458), (-0.5), s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, 0.25, 34, 1.0);
        }

        if (!s.b[620]) {
            s.store_sqrt(439, 442);
            s.store_div_from_scalar_sinh_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
        }

    }

    pub(super) fn stamp_transient_block_5(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (!s.b[620]) {
            s.store_square(35, 459);
            s.store_sqrt_offset_input(458, 35, 1.0);
            s.store_div_scaled_inputs(34, s.ad_value(458), 0.5, s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, (-0.25), 34, 1.0);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_add_scaled_inputs4(429, s.ad_value(423), 1.0, s.ad_value(422), (-1.0), s.ad_value(440), 1.0, A::ln(A::abs(A::mul(A::mul3(s.ad_value(442), s.ad_value(35), s.ad_value(37)), s.ad_value(37)))), -1.0);

        s.store_add_scaled_product_mixed_iaa(427, 457, 1.0, A::add(s.ad_value(456), s.ad_value(446)), A::add_scaled_inputs(s.ad_value(429), s.v[420], s.ad_value(456), 1.0), 1.0);

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_scaled_inputs(443, 456, ((-2.0) * s.v[419]), 457, 1.0);

        s.store_mul(444, 445, 443);

        s.store_add_scaled_product_value_ad(441, A::offset(A::mul_scaled_output(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37), 2.0), (-1.0)), 1.0, 447, 443, (-1.0));

        s.store_add_ad(428, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(457), 1.0, s.ad_value(456), (-s.v[419]), s.ad_value(36), (-s.v[419])), 1.0, s.ad_value(456), s.ad_value(444), 1.0), A::add_scaled_products(s.ad_value(441), s.ad_value(36), s.v[420], s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])), s.v[420]));

        s.store_div_scaled_inputs(425, s.ad_value(427), -1.0, s.ad_value(428), 1.0);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_mul_exp_rhs(34, 421, 448);

        s.store_add_scaled_product_indices(442, 34, (-1.0), 440, 440, s.v[451]);

        s.b[621] = (s.v[442] < 0.0);
        s.v[621] = if s.b[621] { 1.0 } else { 0.0 };

        if s.b[621] {
            s.store_sqrt_neg_input(439, 442);
            s.store_scale(36, 439, 0.5);
            s.store_div_ad_rhs(446, 439, A::tan(s.ad_value(36)));
            s.store_sin(40, 36);
            s.store_mul_neg_lhs(35, 40, 40);
        }

        if (!s.b[621]) {
            s.store_sqrt(439, 442);
            s.store_scale(36, 439, 0.5);
            s.store_sinh(40, 36);
            s.store_square(35, 40);
            s.store_div_ad_rhs(446, 439, A::tanh(s.ad_value(36)));
        }

        s.store_div_scaled_inputs2(438, s.ad_value(440), s.v[419], s.ad_value(446), (-1.0), A::sub_from_scalar(1.0, A::div(s.ad_value(442), A::mul(s.ad_value(35), s.ad_value(34)))), 1.0);

        s.store_scaled_mul(432, 440, 81, s.v[17]);

        s.store_scaled_mul(436, 438, 81, s.v[20]);

        s.store_sub(434, 436, 432);

        s.store_sub_ad_rhs(430, 423, A::div_scaled_inputs(s.ad_value(434), 1.0, s.ad_value(81), s.v[19]));

        s.store_scale(110, 436, 1.0 / (s.v[17]));

        s.store_scaled_add(46, 109, 110, 0.5);

        s.store_sub(49, 109, 110);

        s.store_scale(48, 290, (1.60219e-19 * (p.p49 * 1.0 / (s.v[17]))));

        s.store_scale_ad(34, A::powf(s.ad_value(113), 2.0), 1600.0);

        s.b[622] = (p.p162 != 0.0);
        s.v[622] = if s.b[622] { 1.0 } else { 0.0 };

        if s.b[622] {
            s.store_add_scaled_inputs3(47, s.ad_value(431), 1.0 / ((2.0 * s.v[17])), s.ad_value(432), 1.0 / ((2.0 * s.v[17])), A::mul_sub_from_scalar_lhs_scaled_output(1.0, A::limited_exp_scaled_input(s.ad_value(34), -1.0), A::sub(s.ad_value(431), s.ad_value(432)), ((p.p162 * 0.5) * 1.0 / (s.v[17]))), 1.0);
        }

        if (!s.b[622]) {
            s.store_scaled_add(47, 431, 432, 1.0 / ((2.0 * s.v[17])));
        }

        s.b[623] = (p.p189 != 0.0);
        s.v[623] = if s.b[623] { 1.0 } else { 0.0 };

        if s.b[623] {
            s.store_add_scaled_inputs3(145, s.ad_value(433), 1.0 / ((2.0 * s.v[19])), s.ad_value(434), 1.0 / ((2.0 * s.v[19])), A::mul_sub_from_scalar_lhs_scaled_output(1.0, A::limited_exp_scaled_input(s.ad_value(34), -1.0), A::sub(s.ad_value(433), s.ad_value(434)), ((p.p189 * 0.5) * 1.0 / (s.v[19]))), 1.0);
        }

        if (!s.b[623]) {
            s.store_scaled_add(145, 433, 434, 1.0 / ((2.0 * s.v[19])));
        }

        s.store_add_scaled_product_indices(36, 48, 1.0, 114, 47, 1.0);

        s.store_scaled_add_ad_rhs(37, 36, A::sqrt(A::offset(A::square(s.ad_value(36)), 0.001)), 0.5);

        s.store_scale(116, 37, s.v[129]);

        s.store_add_scaled_product_indices(36, 48, 1.0, 143, 145, 1.0);

        s.store_scaled_add_ad_rhs(37, 36, A::sqrt(A::offset(A::square(s.ad_value(36)), 0.001)), 0.5);

        s.store_scale(117, 37, s.v[144]);

        s.store_pow_ad(624, A::scaled_offset(A::abs_scaled_input(s.ad_value(46), 1.0 / (s.v[59])), 1.0, 0.5), s.ad_value(124));

        s.store_add_scaled_product(625, A::div_scaled_add_product(s.ad_value(125), 1.0, s.ad_value(25), s.ad_value(137), 1.0, s.ad_value(624), 1.0), 1.0, A::add_scaled_product(s.ad_value(122), 1.0, s.ad_value(25), s.ad_value(123), 1.0), A::pow(A::abs(s.ad_value(116)), A::add_scaled_product(s.ad_value(336), 1.0, s.ad_value(342), s.ad_value(25), 1.0)), 1.0);

        s.store_offset(119, 625, 1.0);

        s.store_scaled_add_ad(119, A::offset(s.ad_value(119), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(119), (-1.0), A::offset(s.ad_value(119), (-1.0))), ((0.25 * p.p154) * p.p154))), 0.5);

        s.store_scale(119, 119, 1.0 / (p.p11));

        s.store_div(141, 126, 119);

        s.store_pow_ad(626, A::scaled_offset(A::abs_scaled_input(s.ad_value(46), 1.0 / (s.v[59])), 1.0, 0.5), s.ad_value(348));

        s.store_add_scaled_product(627, A::div_scaled_add_product(s.ad_value(347), 1.0, s.ad_value(25), s.ad_value(138), 1.0, s.ad_value(626), 1.0), 1.0, A::add_scaled_product(s.ad_value(345), 1.0, s.ad_value(25), s.ad_value(346), 1.0), A::pow(A::abs(s.ad_value(117)), A::add_scaled_product(s.ad_value(349), 1.0, s.ad_value(350), s.ad_value(25), 1.0)), 1.0);

        s.store_offset(119, 627, 1.0);

        s.store_scaled_add_ad(119, A::offset(s.ad_value(119), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(119), (-1.0), A::offset(s.ad_value(119), (-1.0))), ((0.25 * p.p154) * p.p154))), 0.5);

        s.store_scale(119, 119, 1.0 / (p.p11));

        s.store_div(142, 344, 119);

        s.store_add_scaled_inputs3(34, s.ad_value(71), 1.0, s.ad_value(431), (-1.0 / ((2.0 * s.v[17]))), s.ad_value(432), (-1.0 / ((2.0 * s.v[17]))));

        s.store_add_scaled_inputs4(35, s.ad_value(70), 1.0, s.ad_value(86), (-1.0), s.ad_value(433), (-1.0 / ((2.0 * s.v[19]))), s.ad_value(434), (-1.0 / ((2.0 * s.v[19]))));

        s.store_div_ad(139, A::exp(A::div(s.ad_value(34), s.ad_value(81))), A::add(A::exp(A::div(s.ad_value(34), s.ad_value(81))), A::exp(A::div(s.ad_value(35), s.ad_value(81)))));

        s.store_div_ad(140, A::exp(A::div(s.ad_value(35), s.ad_value(81))), A::add(A::exp(A::div(s.ad_value(34), s.ad_value(81))), A::exp(A::div(s.ad_value(35), s.ad_value(81)))));

        s.store_add_scaled_products_indices(121, 139, 141, 1.0, 140, 142, 1.0);

        s.store_div_scaled_product_indices(56, 121, 3, s.v[17], 2, 1.0);

        s.store_add_scaled_product_indices(118, 48, s.v[129], 115, 46, s.v[129]);

        s.store_mul_pow_ad_rhs(37, 122, A::abs(s.ad_value(118)), s.ad_value(336));

        s.store_offset(120, 37, 1.0);

        s.store_scaled_add_ad(120, A::offset(s.ad_value(120), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(120), (-1.0), A::offset(s.ad_value(120), (-1.0))), ((0.25 * p.p154) * p.p154))), 0.5);

        s.store_scale(120, 120, 1.0 / (p.p11));

        s.store_div_scaled_inputs(173, s.ad_value(166), 2.0, s.ad_value(121), 1.0);

        s.store_mul(174, 173, 2);

        s.store_offset_mul(34, 165, 25, 0.8);

        s.store_offset_ad(181, A::add_scaled_inputs(s.ad_value(34), 0.5, A::sqrt(A::offset(A::square(s.ad_value(34)), 0.01)), 0.5), 0.2);

        s.store_mul_div_lhs(34, 49, 174, 181);

        s.store_scaled_offset_ad(161, A::sqrt(A::offset(A::square(s.ad_value(34)), p.p109)), 1.0, 1.0 / ((1.0 + ((p.p109) as f64).sqrt())));

        s.store_add_scaled_product_left_ad(161, 161, 1.0, A::mul3_scaled_output(A::add_scaled_value_products(s.ad_value(182), 1.0, s.ad_value(328), s.ad_value(28), (-1.0), s.ad_value(329), s.ad_value(25), (-1.0)), s.ad_value(46), s.ad_value(49), 0.5), 49, 1.0);

        s.store_scaled_add_ad(161, A::offset(s.ad_value(161), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(161), (-1.0), A::offset(s.ad_value(161), (-1.0))), ((0.25 * p.p134) * p.p134))), 0.5);

        s.store_div_scaled_product_indices(171, 167, 120, 2.0, 126, 1.0);

        s.store_mul(172, 171, 1);

        s.b[628] = (s.v[365] > 0.0);
        s.v[628] = if s.b[628] { 1.0 } else { 0.0 };

        if s.b[628] {
            s.store_offset_ad(154, A::div_scaled_product(s.ad_value(365), s.ad_value(46), 1.0, s.ad_value(170), 1.0), 1.0);
        }

        if (!s.b[628]) {
            s.store_div_from_scalar_sub_from_scalar_ad(154, 1.0, 1.0, A::div_scaled_product(s.ad_value(365), s.ad_value(46), 1.0, s.ad_value(170), 1.0));
        }

        s.store_sub(155, 26, 113);

        s.store_add_scaled_inputs(157, 46, 1.0, 55, 2.0);

        s.b[629] = (s.v[153] > 0.0);
        s.v[629] = if s.b[629] { 1.0 } else { 0.0 };

        if s.b[629] {
            s.copy_ad(35, 157);
            s.store_div_ad_rhs(37, 35, A::add(s.ad_value(162), s.ad_value(35)));
            s.store_mul_ad_product_lhs(156, A::div(s.ad_value(35), s.ad_value(153)), s.ad_value(37), 154);
            s.store_offset_div(158, 155, 156, 1.0);
        }

        if (!s.b[629]) {
            s.store_scalar(158, 1.0);
        }

        s.b[630] = (s.v[360] > 0.0);
        s.v[630] = if s.b[630] { 1.0 } else { 0.0 };

        s.b[631] = (p.p213 < 0.0);
        s.v[631] = if s.b[631] { 1.0 } else { 0.0 };

        if (s.b[630] && s.b[631]) {
            s.store_div_from_scalar_ad(35, 1.0, A::sub_scaled_inputs(A::div_from_scalar(1.0, s.ad_value(360)), 1.0, s.ad_value(46), p.p213));
        }

        if (s.b[630] && (!s.b[631])) {
            s.store_mul_ad_rhs(35, 360, A::scale_offset(s.ad_value(46), p.p213, 1.0));
        }

        if s.b[630] {
            s.store_offset_mul_ad(159, s.ad_value(35), A::ln(A::max_with_scalar(A::offset(A::div_scaled_value_by_product(s.ad_value(155), 1.0, s.ad_value(35), A::add(s.ad_value(162), s.ad_value(170)), 1.0), 1.0), 1e-38)), 1.0);
        }

        if (!s.b[630]) {
            s.store_scalar(159, 1.0);
        }

        s.store_mul(158, 158, 159);

        s.b[632] = (s.v[361] > 0.0);
        s.v[632] = if s.b[632] { 1.0 } else { 0.0 };

        if s.b[632] {
            s.store_offset_mul_ad(160, s.ad_value(361), A::ln(A::max_with_scalar(A::offset(A::div_scaled_inputs2_by_product(s.ad_value(26), 1.0, s.ad_value(113), (-1.0), s.ad_value(361), A::add(s.ad_value(162), s.ad_value(172)), 1.0), 1.0), 1e-38)), 1.0);
        }

        if (!s.b[632]) {
            s.store_scalar(160, 1.0);
        }

        s.b[633] = (s.v[175] != 0.0);
        s.v[633] = if s.b[633] { 1.0 } else { 0.0 };

        if s.b[633] {
            s.store_div_ad_rhs(35, 175, A::add_scaled_product(s.ad_value(81), 2.0, A::max_from_scalar(0.0, A::add(s.ad_value(176), A::mul3(s.ad_value(108), s.ad_value(49), s.ad_value(49)))), s.ad_value(46), 1.0));
            s.store_limited_exp_neg_input(94, 35);
        }

        if (!s.b[633]) {
            s.store_scalar(94, 1.0);
        }

        s.store_sub(34, 437, 438);

        s.store_sub_ad(35, A::square(s.ad_value(437)), A::square(s.ad_value(438)));

        s.store_add_ad(215, A::mul3_scaled_output(s.ad_value(81), s.ad_value(55), s.ad_value(34), (s.v[20] * 2.0)), A::mul3_scaled_output(s.ad_value(81), s.ad_value(81), s.ad_value(35), ((s.v[20] * s.v[20]) * (0.5 * 1.0 / (s.v[17])))));

        s.store_add_scaled_inputs3(216, s.ad_value(109), 0.5, s.ad_value(110), 0.5, s.ad_value(55), 1.0);

        s.b[640] = (p.p14 == 1.0);
        s.v[640] = if s.b[640] { 1.0 } else { 0.0 };

        if s.b[640] {
            s.store_scalar(151, 0.0);
            s.store_scalar(130, 1.0);
            s.store_sub(638, 29, 200);
            s.store_sqrt_square_offset(639, 638, 0.0001);
            s.store_scaled_add(636, 638, 639, 0.5);
            s.store_offset_mul(635, 284, 636, 1.0);
            s.store_div_from_scalar(634, 1.0, 635);
            s.store_add_scaled_product_indices(634, 634, 1.0, 32, 285, (-0.5));
            s.store_scaled_add_ad_rhs(34, 634, A::sqrt(A::offset(A::square(s.ad_value(634)), 0.01)), 0.5);
            s.store_mul_ad_rhs(147, 150, A::add_scaled_product(s.ad_value(132), 1.0, A::add_scaled_product(s.ad_value(136), 1.0, s.ad_value(283), s.ad_value(34), 1.0), s.ad_value(131), 1.0));
            s.store_sub(638, 31, 200);
            s.store_sqrt_square_offset(639, 638, 0.0001);
            s.store_scaled_add(637, 638, 639, 0.5);
            s.store_offset_mul(635, 284, 637, 1.0);
            s.store_div_from_scalar(634, 1.0, 635);
            s.store_add_scaled_product_indices(634, 634, 1.0, 33, 285, (-0.5));
            s.store_scaled_add_ad_rhs(34, 634, A::sqrt(A::offset(A::square(s.ad_value(634)), 0.01)), 0.5);
            s.store_mul_ad_rhs(146, 150, A::add_scaled_product(s.ad_value(133), 1.0, A::add_scaled_product(s.ad_value(135), 1.0, s.ad_value(282), s.ad_value(34), 1.0), s.ad_value(131), 1.0));
        }

        if (!s.b[640]) {
            s.store_offset_mul(635, 284, 46, 1.0);
            s.store_div_from_scalar(634, 1.0, 635);
            s.store_add_scaled_product_left_ad(634, 634, 1.0, A::add(s.ad_value(24), s.ad_value(23)), 285, (-0.5));
            s.store_scaled_add_ad_rhs(34, 634, A::sqrt(A::offset(A::square(s.ad_value(634)), 0.01)), 0.5);
            s.store_mul_ad_product_lhs(151, s.ad_value(150), A::add_scaled_product(s.ad_value(134), 1.0, s.ad_value(281), s.ad_value(34), 1.0), 131);
            s.store_offset_mul_ad(130, A::div_scaled_product(s.ad_value(56), s.ad_value(216), p.p2, s.ad_value(161), 1.0), s.ad_value(151), 1.0);
            s.copy_ad(146, 133);
            s.copy_ad(147, 132);
        }

        s.b[641] = (p.p14 == 2.0);
        s.v[641] = if s.b[641] { 1.0 } else { 0.0 };

        if ((!s.b[640]) && s.b[641]) {
            s.store_offset_mul(635, 284, 46, 1.0);
            s.store_div_from_scalar(634, 1.0, 635);
            s.store_add_scaled_product_left_ad(634, 634, 1.0, A::add(s.ad_value(24), s.ad_value(23)), 285, (-0.5));
            s.store_scaled_add_ad_rhs(34, 634, A::sqrt(A::offset(A::square(s.ad_value(634)), 0.01)), 0.5);
            s.store_mul_ad_product_lhs(151, s.ad_value(150), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(132), 1.0, s.ad_value(133), 1.0, s.ad_value(134), 1.0), 1.0, s.ad_value(281), s.ad_value(34), 1.0), 131);
            s.store_offset_mul_ad(130, A::div_scaled_product(s.ad_value(56), s.ad_value(216), p.p2, s.ad_value(161), 1.0), s.ad_value(151), 1.0);
            s.store_scalar(146, 0.0);
            s.store_scalar(147, 0.0);
        }

        s.store_div_scaled_product_by_product(214, A::mul3_scaled_output(s.ad_value(56), s.ad_value(215), s.ad_value(158), 1.0 / (s.v[17])), s.ad_value(94), 1.0, s.ad_value(161), s.ad_value(130), 1.0);

        s.store_scale(214, 214, p.p2);

        s.store_scaled_add(219, 432, 431, 0.5);

        s.store_add_scaled_inputs(218, 435, (1.0 / 6.0), 436, (2.0 * (1.0 / 6.0)));

        s.store_add_scaled_inputs(217, 435, (2.0 * (1.0 / 6.0)), 436, (1.0 / 6.0));

        s.store_scaled_add(220, 434, 433, 0.5);

        s.b[642] = (s.v[62] > 0.0);
        s.v[642] = if s.b[642] { 1.0 } else { 0.0 };

        if s.b[642] {
            s.store_div_scaled_add_product(38, s.ad_value(46), 1.0, s.ad_value(66), s.ad_value(48), 1.0, s.ad_value(67), 1.0);
            s.store_offset_pow_ad(39, s.ad_value(38), s.ad_value(68), 1.0);
            s.store_scalar(63, p.p49);
            s.store_div(64, 63, 39);
            s.store_div_from_scalar_ad(65, (3.9 * 8.85418e-12), A::add_scaled_product(s.ad_value(221), (3.9 * 1.0 / (p.p60)), s.ad_value(64), s.ad_value(62), 1.0 / (s.v[21])));
        }

        if (!s.b[642]) {
            s.store_scalar(65, s.v[18]);
        }

        s.store_div_scaled_product_indices(34, 4, 1, 1.0, 160, 1.0);

        s.store_mul(219, 219, 34);

        s.store_mul_neg_lhs(218, 218, 34);

        s.store_mul(220, 220, 34);

        s.store_mul_neg_lhs(217, 217, 34);

        s.store_mul_ad_affine_product_rhs(228, 4, s.ad_value(396), A::voltage(ctx, nodes, Some(7), Some(6)), s.v[17], 0.0);

        s.store_mul_ad_affine_product_rhs(230, 4, s.ad_value(397), A::voltage(ctx, nodes, Some(7), Some(5)), s.v[17], 0.0);

        s.store_mul_sub_rhs(240, 212, 288, 99);

        s.store_add_scaled_inputs4_offset(34, s.ad_value(235), 1.0, s.ad_value(200), (-1.0), s.ad_value(32), ((p.p45 / p.p46) * p.p269), s.ad_value(240), (-((p.p45 / p.p46) * p.p269)), (0.02 + ((-p.p268) * ((p.p45 / p.p46) * p.p269))));

        s.store_scaled_sub_ad_rhs(232, 34, A::sqrt(A::offset(A::square(s.ad_value(34)), (4.0 * 0.02))), 0.5);

        s.store_add_scaled_inputs3(35, s.ad_value(235), 1.0, s.ad_value(200), (-1.0), s.ad_value(232), -1.0);

        s.store_add_ad_rhs(228, 228, A::mul3_scaled_output(s.ad_value(212), s.ad_value(4), A::sub(s.ad_value(35), A::scaled_offset(A::sqrt(A::sub_from_scalar(1.0, A::scale(s.ad_value(232), (4.0 * 1.0 / (p.p265))))), (-1.0), (0.5 * p.p265))), p.p263));

        s.store_add_scaled_inputs4_offset(34, s.ad_value(234), 1.0, s.ad_value(200), (-1.0), s.ad_value(33), ((p.p45 / p.p46) * p.p271), s.ad_value(240), (-((p.p45 / p.p46) * p.p271)), (0.02 + ((-p.p270) * ((p.p45 / p.p46) * p.p271))));

    }

    pub(super) fn stamp_transient_block_6(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.store_scaled_sub_ad_rhs(233, 34, A::sqrt(A::offset(A::square(s.ad_value(34)), (4.0 * 0.02))), 0.5);

        s.store_add_scaled_inputs3(35, s.ad_value(234), 1.0, s.ad_value(200), (-1.0), s.ad_value(233), -1.0);

        s.store_add_ad_rhs(230, 230, A::mul3_scaled_output(s.ad_value(212), s.ad_value(4), A::sub(s.ad_value(35), A::scaled_offset(A::sqrt(A::sub_from_scalar(1.0, A::scale(s.ad_value(233), (4.0 * 1.0 / (p.p266))))), (-1.0), (0.5 * p.p266))), p.p264));

        s.store_mul_ad_product_rhs(229, 4, s.ad_value(398), A::voltage(ctx, nodes, Some(7), Some(6)));

        s.store_mul_ad_product_rhs(231, 4, s.ad_value(399), A::voltage(ctx, nodes, Some(7), Some(5)));

        s.store_add(226, 228, 229);

        s.store_add(227, 230, 231);

        s.store_ad_value(238, A::mul_scaled_lhs(s.ad_value(212), s.v[236], A::voltage(ctx, nodes, Some(6), Some(3))));

        s.store_ad_value(239, A::mul_scaled_lhs(s.ad_value(212), s.v[237], A::voltage(ctx, nodes, Some(5), Some(3))));

        s.store_div_scaled_add_product(34, s.ad_value(366), 1.0, s.ad_value(367), s.ad_value(2), 1.0, s.ad_value(2), 1.0);

        s.b[643] = ((s.v[34] <= 0.0) || (s.v[103] <= 0.0));
        s.v[643] = if s.b[643] { 1.0 } else { 0.0 };

        if s.b[643] {
            s.store_scalar(211, 0.0);
        }

        s.b[644] = (s.v[155] > (s.v[103] / 80.0));
        s.v[644] = if s.b[644] { 1.0 } else { 0.0 };

        if ((!s.b[643]) && s.b[644]) {
            s.store_div_scaled_inputs(35, s.ad_value(103), -1.0, s.ad_value(155), 1.0);
            s.store_mul_ad(211, A::mul3(s.ad_value(34), s.ad_value(155), s.ad_value(214)), A::limited_exp(s.ad_value(35)));
        }

        if ((!s.b[643]) && (!s.b[644])) {
            s.store_mul3_affine_lhs(211, 34, 155, 1.804851387e-35, 0.0, 214);
        }

        s.v[184] = 0.0;

        s.v[192] = 0.0;

        s.v[193] = 0.0;

        s.v[194] = 0.0;

        s.v[201] = 0.0;

        s.v[202] = 0.0;

        s.b[645] = (p.p17 != 0.0);
        s.v[645] = if s.b[645] { 1.0 } else { 0.0 };

        if s.b[645] {
            s.store_div_scaled_inputs2_by_product(35, s.ad_value(46), 1.0, s.ad_value(411), (-1.0), s.ad_value(412), s.ad_value(55), 1.0);
            s.store_mul_ad_product_rhs(183, 412, s.ad_value(55), A::ln(A::max_with_scalar(A::offset(A::limited_exp(s.ad_value(35)), 1.0), 1e-38)));
            s.store_add_scaled_product_indices(36, 408, 1.0, 409, 46, (-1.0));
            s.store_offset_mul(37, 410, 46, 1.0);
            s.store_scaled_mul(38, 36, 37, ((-982222000000.0) * p.p99));
            s.store_limited_exp(39, 38);
            s.store_scalar(40, 3.75956e-7);
            s.store_mul_ad_product_lhs(184, A::mul3(A::mul3(s.ad_value(3), s.ad_value(2), s.ad_value(40)), s.ad_value(207), s.ad_value(209)), s.ad_value(183), 39);
            s.store_mul(184, 184, 106);
            s.store_sub(191, 52, 50);
            s.store_sub(34, 191, 209);
            s.store_div_scaled_value_by_product(35, s.ad_value(34), 1.0, s.ad_value(416), s.ad_value(55), 1.0);
            s.store_mul_ad_product_rhs(190, 416, s.ad_value(55), A::ln(A::max_with_scalar(A::offset(A::limited_exp(s.ad_value(35)), 1.0), 1e-38)));
        }

        s.b[646] = (s.v[191] <= 0.0);
        s.v[646] = if s.b[646] { 1.0 } else { 0.0 };

        if (s.b[645] && s.b[646]) {
            s.store_scaled_add_ad(189, A::offset(s.ad_value(34), (-0.02)), A::sqrt(A::add_scaled_offset_product_lhs(s.ad_value(191), (-0.08), s.ad_value(34), (-0.02), A::offset(s.ad_value(34), (-0.02)), 1.0)), 0.5);
        }

        if (s.b[645] && (!s.b[646])) {
            s.store_scaled_add_ad(189, A::offset(s.ad_value(34), (-0.02)), A::sqrt(A::add_scaled_offset_product_lhs(s.ad_value(191), 0.08, s.ad_value(34), (-0.02), A::offset(s.ad_value(34), (-0.02)), 1.0)), 0.5);
        }

        if s.b[645] {
            s.store_add_scaled_product_indices(36, 413, 1.0, 414, 189, (-1.0));
            s.store_offset_mul(37, 415, 189, 1.0);
            s.store_scaled_mul(38, 36, 37, ((-745669000000.0) * p.p99));
            s.store_limited_exp(39, 38);
            s.store_scalar(40, 4.97232e-7);
            s.store_mul_ad_product_lhs(192, A::mul3(A::mul3(s.ad_value(3), s.ad_value(2), s.ad_value(40)), s.ad_value(207), s.ad_value(209)), s.ad_value(190), 39);
            s.store_mul(192, 192, 106);
        }

        s.store_tanh_ad(34, A::div_scaled_inputs(s.ad_value(30), 0.6, s.ad_value(55), 1.0));

        s.store_offset_scaled(57, 34, 0.5, 0.5);

        s.store_sub_from_scalar(58, 1.0, 57);

        s.store_mul_add_rhs(187, 57, 184, 192);

        s.store_mul_add_rhs(188, 58, 184, 192);

        s.b[647] = (p.p16 != 0.0);
        s.v[647] = if s.b[647] { 1.0 } else { 0.0 };

        if s.b[647] {
            s.store_add_scaled_product_right_ad(35, 369, 1.0, 370, A::add_scaled_product(s.ad_value(69), 1.0, s.ad_value(373), s.ad_value(210), (-1.0)), (-1.0));
            s.store_offset_mul_ad(36, s.ad_value(371), A::add_scaled_product(s.ad_value(69), 1.0, s.ad_value(373), s.ad_value(210), (-1.0)), 1.0);
            s.store_mul3_affine_lhs(37, 206, 35, (-p.p99), 0.0, 36);
            s.store_mul_limited_exp_rhs(38, 46, 37);
            s.store_add_scaled_inputs4(39, s.ad_value(209), 1.0, s.ad_value(73), 0.5, s.ad_value(32), 0.5, s.ad_value(33), 0.5);
            s.store_mul_ad_product_lhs(195, A::mul3(A::mul3(s.ad_value(3), s.ad_value(2), s.ad_value(205)), s.ad_value(207), s.ad_value(38)), s.ad_value(39), 106);
            s.store_offset_sqrt_ad(196, A::offset(A::square(s.ad_value(113)), 0.01), (-0.1));
            s.store_mul(35, 372, 196);
            s.store_limited_exp_neg_input(197, 35);
            s.store_offset_add(37, 35, 197, (((-1.0)) + (0.0001)));
            s.store_offset_sub_from_scalar_ad(38, 1.0, A::mul_offset_lhs(s.ad_value(35), 1.0, s.ad_value(197)), 0.0001);
            s.store_offset_square(39, 35, 0.0002);
            s.store_div_scaled_product_indices(194, 195, 38, 1.0, 39, 1.0);
            s.store_div_scaled_product_indices(193, 195, 37, 1.0, 39, 1.0);
            s.store_add_scaled_inputs_product_right_ad(34, 29, 1.0, 200, (-1.0), 385, A::sub(s.ad_value(23), s.ad_value(240)), s.v[243]);
            s.store_sqrt_square_offset(203, 34, 0.0001);
            s.store_add_scaled_product_indices(35, 382, 1.0, 383, 203, (-1.0));
            s.store_offset_mul(36, 384, 203, 1.0);
            s.store_mul_ad_lhs(37, A::mul3_scaled_output(s.ad_value(206), s.ad_value(394), s.ad_value(35), (-p.p99)), 36);
            s.store_limited_exp(38, 37);
        }

        s.b[648] = (s.v[27] > 0.0);
        s.v[648] = if s.b[648] { 1.0 } else { 0.0 };

        if (s.b[647] && s.b[648]) {
            s.store_mul_ad_lhs(201, A::mul3_scaled_output(s.ad_value(185), s.ad_value(29), s.ad_value(203), p.p234), 38);
        }

        if (s.b[647] && (!s.b[648])) {
            s.store_mul_ad_lhs(202, A::mul3_scaled_output(s.ad_value(185), s.ad_value(29), s.ad_value(203), p.p234), 38);
        }

        if s.b[647] {
            s.store_add_scaled_inputs_product_right_ad(34, 31, 1.0, 200, (-1.0), 389, A::sub(s.ad_value(23), s.ad_value(240)), s.v[243]);
            s.store_sqrt_square_offset(204, 34, 0.0001);
            s.store_add_scaled_product_indices(35, 386, 1.0, 387, 204, (-1.0));
            s.store_offset_mul(36, 388, 204, 1.0);
            s.store_mul_ad_lhs(37, A::mul3_scaled_output(s.ad_value(206), s.ad_value(394), s.ad_value(35), (-p.p99)), 36);
            s.store_limited_exp(38, 37);
        }

        s.b[649] = (s.v[27] > 0.0);
        s.v[649] = if s.b[649] { 1.0 } else { 0.0 };

        if (s.b[647] && s.b[649]) {
            s.store_mul_ad_lhs(202, A::mul3_scaled_output(s.ad_value(185), s.ad_value(31), s.ad_value(204), p.p235), 38);
        }

        if (s.b[647] && (!s.b[649])) {
            s.store_mul_ad_lhs(201, A::mul3_scaled_output(s.ad_value(185), s.ad_value(31), s.ad_value(204), p.p235), 38);
        }

        s.v[198] = 0.0;

        s.v[199] = 0.0;

        s.b[650] = (p.p15 != 0.0);
        s.v[650] = if s.b[650] { 1.0 } else { 0.0 };

        if s.b[650] {
            s.store_scalar(34, (s.v[21] * p.p45));
        }

        s.b[651] = ((s.v[378] <= 0.0) || (s.v[104] <= 0.0));
        s.v[651] = if s.b[651] { 1.0 } else { 0.0 };

        if (s.b[650] && s.b[651]) {
            s.store_scalar(40, 0.0);
        }

        if (s.b[650] && (!s.b[651])) {
            s.store_div_scaled_add_product(35, A::add_scaled_inputs3(s.ad_value(31), -1.0, s.ad_value(380), (-1.0), s.ad_value(200), 1.0), 1.0, s.ad_value(390), A::add_scaled_inputs3(s.ad_value(23), 1.0, s.ad_value(240), (-1.0), s.ad_value(391), -1.0), s.v[243], s.ad_value(34), 1.0);
            s.store_scaled_add_ad_rhs(35, 35, A::sqrt(A::offset(A::square(s.ad_value(35)), ((4.0 * 0.01) * 0.01))), 0.5);
            s.store_div_scaled_value_offset_denominator(36, s.ad_value(104), 1.0, s.ad_value(35), 0.001, 1.0);
            s.store_limited_exp_ad(37, A::mul(s.ad_value(381), A::ln(A::max_with_scalar(s.ad_value(35), 1e-38))));
            s.store_mul_ad_product_lhs(40, A::mul3(s.ad_value(378), s.ad_value(3), s.ad_value(37)), A::limited_exp_scaled_input(s.ad_value(36), -1.0), 30);
        }

        s.b[652] = (s.v[27] > 0.0);
        s.v[652] = if s.b[652] { 1.0 } else { 0.0 };

        if (s.b[650] && s.b[652]) {
            s.copy_ad(199, 40);
        }

        if (s.b[650] && (!s.b[652])) {
            s.copy_ad(198, 40);
        }

        s.b[653] = ((s.v[374] <= 0.0) || (s.v[105] <= 0.0));
        s.v[653] = if s.b[653] { 1.0 } else { 0.0 };

        if (s.b[650] && s.b[653]) {
            s.store_scalar(40, 0.0);
        }

        if (s.b[650] && (!s.b[653])) {
            s.store_div_scaled_add_product(35, A::add_scaled_inputs3(s.ad_value(29), -1.0, s.ad_value(376), (-1.0), s.ad_value(200), 1.0), 1.0, s.ad_value(392), A::add_scaled_inputs3(s.ad_value(23), 1.0, s.ad_value(240), (-1.0), s.ad_value(393), -1.0), s.v[243], s.ad_value(34), 1.0);
            s.store_scaled_add_ad_rhs(35, 35, A::sqrt(A::offset(A::square(s.ad_value(35)), ((4.0 * 0.01) * 0.01))), 0.5);
            s.store_div_scaled_value_offset_denominator(36, s.ad_value(105), 1.0, s.ad_value(35), 0.001, 1.0);
            s.store_limited_exp_ad(37, A::mul(s.ad_value(377), A::ln(A::max_with_scalar(s.ad_value(35), 1e-38))));
            s.store_ad_value(40, A::mul3(A::mul3_scaled_output(s.ad_value(30), s.ad_value(374), s.ad_value(3), -1.0), s.ad_value(37), A::limited_exp_scaled_input(s.ad_value(36), -1.0)));
        }

        s.b[654] = (s.v[27] > 0.0);
        s.v[654] = if s.b[654] { 1.0 } else { 0.0 };

        if (s.b[650] && s.b[654]) {
            s.copy_ad(198, 40);
        }

        if (s.b[650] && (!s.b[654])) {
            s.copy_ad(199, 40);
        }

        s.store_div_scaled_inputs(254, s.ad_value(164), 2.0, s.ad_value(121), 1.0);

        s.b[655] = (((p.p288 > 0.0) || (p.p289 > 0.0)) || (p.p290 > 0.0));
        s.v[655] = if s.b[655] { 1.0 } else { 0.0 };

        if s.b[655] {
            s.store_sub_scaled_inputs(255, 2, 1.0, 249, 2.0);
            s.store_square(256, 255);
        }

        s.b[656] = (p.p287 <= 0.0);
        s.v[656] = if s.b[656] { 1.0 } else { 0.0 };

        if (s.b[655] && s.b[656]) {
            s.store_scalar(257, 0.0);
        }

        if (s.b[655] && (!s.b[656])) {
            s.store_div_scaled_offset_numerator(34, s.ad_value(155), 1.0 / (s.v[253]), p.p287, s.ad_value(254), 1.0);
            s.store_scaled_ln_ad(257, A::max_with_scalar(s.ad_value(34), 1e-38), s.v[253]);
        }

        s.b[657] = (s.v[257] < 0.0);
        s.v[657] = if s.b[657] { 1.0 } else { 0.0 };

        if ((s.b[655] && (!s.b[656])) && s.b[657]) {
            s.store_scalar(257, 0.0);
        }

        s.b[658] = (p.p22 == 1.0);
        s.v[658] = if s.b[658] { 1.0 } else { 0.0 };

        if (s.b[655] && s.b[658]) {
            s.store_div(35, 47, 252);
            s.store_offset_pow_ad(36, s.ad_value(35), s.ad_value(251), 1.0);
            s.store_div(37, 250, 36);
            s.store_scale(38, 37, 1.0 / (p.p288));
            s.store_scaled_add_ad(39, A::offset(s.ad_value(38), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(38), (-1.0), A::offset(s.ad_value(38), (-1.0))), ((0.25 * p.p292) * p.p292))), 0.5);
            s.store_scale(258, 39, p.p288);
        }

        if (s.b[655] && (!s.b[658])) {
            s.store_scalar(258, p.p288);
        }

        if s.b[655] {
            s.store_mul_ad_affine_product_lhs(35, s.ad_value(55), A::abs(s.ad_value(214)), ((1.60219e-19 * 1.60219e-19) * 1.60219e-19), 0.0, 121);
            s.store_scaled_mul(36, 65, 256, 10000000000.0);
            s.store_scaled_mul(259, 65, 109, 6.241457005723417e18);
            s.store_scaled_mul(260, 65, 110, 6.241457005723417e18);
            s.store_mul_scaled_ad_rhs(261, 55, 1.0 / (1.60219e-19), A::add(s.ad_value(65), s.ad_value(291)));
            s.store_mul_ln_ad_rhs(37, 258, A::max_with_scalar(A::div_scaled_inputs2(s.ad_value(259), 1.0, s.ad_value(261), 1.0, A::add(s.ad_value(260), s.ad_value(261)), 1.0), 1e-38));
            s.store_scaled_sub(38, 259, 260, p.p289);
            s.store_scaled_sub_ad(39, A::square(s.ad_value(259)), A::square(s.ad_value(260)), (0.5 * p.p290));
            s.store_mul3_affine_lhs(40, 55, 214, 1.60219e-19, 0.0, 214);
            s.store_scaled_mul(41, 256, 3, (10000000000.0 * p.p2));
            s.store_add_scaled_inputs_product_indices(42, 258, 1.0, 260, p.p289, 260, 260, p.p290);
            s.store_mul_ad(43, A::add(s.ad_value(260), s.ad_value(261)), A::add(s.ad_value(260), s.ad_value(261)));
            s.store_add_scaled_product(262, A::div_scaled_product3_by_product(s.ad_value(40), s.ad_value(257), s.ad_value(42), 1.0, s.ad_value(41), s.ad_value(43), 1.0), 1.0, A::div(s.ad_value(35), s.ad_value(36)), A::add_scaled_inputs3(s.ad_value(37), 1.0, s.ad_value(38), 1.0, s.ad_value(39), 1.0), 1.0);
            s.store_scaled_mul(44, 258, 55, 1.60219e-19);
            s.store_mul_ad_lhs(45, A::mul3_scaled_output(s.ad_value(3), s.ad_value(255), s.ad_value(261), (p.p2 * 10000000000.0)), 261);
            s.store_mul_ad_product_lhs(263, A::div(s.ad_value(44), s.ad_value(45)), s.ad_value(214), 214);
            s.store_add(35, 263, 262);
        }

        s.b[659] = (s.v[35] > 0.0);
        s.v[659] = if s.b[659] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_7(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[655] && s.b[659]) {
            s.store_div_scaled_product_indices(264, 262, 263, 1.0, 35, 1.0);
        }

        if (s.b[655] && (!s.b[659])) {
            s.store_scalar(264, 0.0);
        }

        if (!s.b[655]) {
            s.store_scalar(264, 0.0);
        }

        s.store_scaled_mul(224, 212, 219, p.p2);

        s.store_scale(225, 220, p.p2);

        s.b[660] = (s.v[27] > 0.0);
        s.v[660] = if s.b[660] { 1.0 } else { 0.0 };

        if s.b[660] {
            s.store_scale(222, 217, p.p2);
            s.store_scale(223, 218, p.p2);
            s.store_add_scaled_inputs3(217, s.ad_value(217), p.p2, s.ad_value(226), (-p.p2), s.ad_value(238), 1.0);
            s.store_add_scaled_inputs3(218, s.ad_value(218), p.p2, s.ad_value(227), (-p.p2), s.ad_value(239), 1.0);
        }

        if (!s.b[660]) {
            s.store_scale(222, 218, p.p2);
            s.store_scale(223, 217, p.p2);
            s.store_add_scaled_inputs3(34, s.ad_value(218), p.p2, s.ad_value(226), (-p.p2), s.ad_value(238), 1.0);
            s.store_add_scaled_inputs3(218, s.ad_value(217), p.p2, s.ad_value(227), (-p.p2), s.ad_value(239), 1.0);
            s.copy_ad(217, 34);
        }

        s.store_add_scaled_inputs3(219, s.ad_value(224), 1.0, s.ad_value(226), p.p2, s.ad_value(227), p.p2);

        s.store_add_scaled_inputs3(220, s.ad_value(220), p.p2, s.ad_value(238), (-1.0), s.ad_value(239), -1.0);

        s.store_scale(226, 226, p.p2);

        s.store_scale(227, 227, p.p2);

        s.store_neg_ad(265, A::add(s.ad_value(222), s.ad_value(223)));

        s.store_mul(34, 121, 265);

        s.store_add_scaled_square_product(35, s.ad_value(2), 1.0, s.ad_value(34), s.ad_value(151), 1.0);

        s.store_scaled_div(266, 34, 35, p.p295);

        s.store_scale(268, 55, (4.0 * 1.60219e-19));

        s.store_mul(267, 268, 266);

        s.b[661] = ((p.p20 == 1.0) && (s.v[275] != 0.0));
        s.v[661] = if s.b[661] { 1.0 } else { 0.0 };

        if s.b[661] {
            s.store_div_scaled_product_indices(34, 121, 3, s.v[17], 2, 1.0);
            s.store_mul(277, 34, 46);
            s.store_mul_scaled_ad_rhs(274, 275, p.p2, A::add(s.ad_value(277), A::mul3(s.ad_value(276), s.ad_value(55), s.ad_value(34))));
        }

        if (!s.b[661]) {
            s.store_scalar(274, 0.0);
        }

        s.store_scale(199, 199, p.p2);

        s.store_scale(198, 198, p.p2);

        s.store_scale(194, 194, p.p2);

        s.store_scale(193, 193, p.p2);

        s.store_scale(201, 201, p.p2);

        s.store_scale(202, 202, p.p2);

        s.b[662] = (s.v[27] > 0.0);
        s.v[662] = if s.b[662] { 1.0 } else { 0.0 };

        s.b[663] = (p.p14 == 2.0);
        s.v[663] = if s.b[663] { 1.0 } else { 0.0 };

        if (!s.b[663]) {
            s.store_div_from_scalar(149, 1.0, 146);
            s.store_div_from_scalar(148, 1.0, 147);
        }

        s.b[664] = ((p.p20 == 1.0) && (s.v[275] != 0.0));
        s.v[664] = if s.b[664] { 1.0 } else { 0.0 };

        s.b[665] = (p.p19 == 0.0);
        s.v[665] = if s.b[665] { 1.0 } else { 0.0 };

        if (!s.b[665]) {
            s.copy_ad(666, 273);
            s.copy_ad(667, 273);
        }

        s.b[668] = (p.p16 != 0.0);
        s.v[668] = if s.b[668] { 1.0 } else { 0.0 };

        s.b[669] = (s.v[27] > 0.0);
        s.v[669] = if s.b[669] { 1.0 } else { 0.0 };

        s.b[670] = (p.p17 != 0.0);
        s.v[670] = if s.b[670] { 1.0 } else { 0.0 };

        s.b[671] = ((p.p18 != 0.0) && (p.p310 > 0.0));
        s.v[671] = if s.b[671] { 1.0 } else { 0.0 };

        s.b[672] = (p.p14 != 2.0);
        s.v[672] = if s.b[672] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_0(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[527] = (p.p12 == 1.0);
        s.v[527] = if s.b[527] { 1.0 } else { 0.0 };

        if s.b[527] {
            s.store_scalar(212, 1.0);
        }

        if (!s.b[527]) {
            s.store_scalar(212, (-1.0));
        }

        s.b[528] = (p.p13 == 1.0);
        s.v[528] = if s.b[528] { 1.0 } else { 0.0 };

        if s.b[528] {
            s.store_scalar(213, 1.0);
        }

        if (!s.b[528]) {
            s.store_scalar(213, (-1.0));
        }

        s.v[16] = (p.p59 * 8.85418e-12);

        s.b[529] = (p.p21 == 0.0);
        s.v[529] = if s.b[529] { 1.0 } else { 0.0 };

        if s.b[529] {
            s.store_scalar(5, (p.p1 / p.p2));
        }

        if (!s.b[529]) {
            s.store_scalar(5, p.p1);
        }

        s.v[0] = (p.p0 + p.p23);

        s.store_offset(5, 5, p.p24);

        s.v[6] = ((s.v[0]) as f64).powf((-p.p29));

        s.store_powf(7, 5, (-p.p30));

        s.store_scale(8, 7, s.v[6]);

        s.store_add_scaled_ad_lhs(9, A::scale_offset(s.ad_value(7), p.p27, (p.p25 + (p.p26 * s.v[6]))), 8, p.p28);

        s.v[10] = ((s.v[0]) as f64).powf((-p.p35));

        s.store_powf(11, 5, (-p.p36));

        s.store_scale(12, 11, s.v[10]);

        s.store_add_scaled_ad_lhs(13, A::scale_offset(s.ad_value(11), p.p33, (p.p31 + (p.p32 * s.v[10]))), 12, p.p34);

        s.store_sub_from_scalar_ad(2, s.v[0], A::scale(s.ad_value(9), 2.0));

        s.store_sub_scaled_inputs(3, 5, 1.0, 13, 2.0);

        s.store_add_scaled_ad_lhs(14, A::scale_offset(s.ad_value(7), p.p39, (p.p37 + (p.p38 * s.v[6]))), 8, p.p40);

        s.store_add_scaled_ad_lhs(15, A::scale_offset(s.ad_value(11), p.p43, (p.p41 + (p.p42 * s.v[10]))), 12, p.p44);

        s.store_sub_from_scalar_ad(1, s.v[0], A::scale(s.ad_value(14), 2.0));

        s.store_sub_scaled_inputs(4, 5, 1.0, 15, 2.0);

        s.store_div_from_scalar(278, 1e-6, 2);

        s.store_div_from_scalar(279, 1e-6, 3);

        s.store_mul(280, 278, 279);

        s.store_add_scaled_inputs3_offset(281, s.ad_value(278), p.p319, s.ad_value(279), p.p320, s.ad_value(280), p.p321, p.p191);

        s.store_add_scaled_inputs3_offset(284, s.ad_value(278), p.p328, s.ad_value(279), p.p329, s.ad_value(280), p.p330, p.p202);

        s.store_add_scaled_inputs3_offset(285, s.ad_value(278), p.p331, s.ad_value(279), p.p332, s.ad_value(280), p.p333, p.p203);

        s.store_add_scaled_inputs3_offset(286, s.ad_value(278), p.p334, s.ad_value(279), p.p335, s.ad_value(280), p.p336, p.p204);

        s.store_add_scaled_inputs3_offset(287, s.ad_value(278), p.p337, s.ad_value(279), p.p338, s.ad_value(280), p.p339, p.p57);

        s.store_add_scaled_inputs3_offset(288, s.ad_value(278), p.p340, s.ad_value(279), p.p341, s.ad_value(280), p.p342, p.p58);

        s.store_add_scaled_inputs3_offset(289, s.ad_value(278), p.p343, s.ad_value(279), p.p344, s.ad_value(280), p.p345, p.p51);

        s.store_add_scaled_inputs3_offset(290, s.ad_value(278), p.p346, s.ad_value(279), p.p347, s.ad_value(280), p.p348, p.p50);

        s.store_add_scaled_inputs3_offset(291, s.ad_value(278), p.p349, s.ad_value(279), p.p350, s.ad_value(280), p.p351, p.p63);

        s.store_add_scaled_inputs3_offset(292, s.ad_value(278), p.p352, s.ad_value(279), p.p353, s.ad_value(280), p.p354, p.p64);

        s.store_add_scaled_inputs3_offset(293, s.ad_value(278), p.p355, s.ad_value(279), p.p356, s.ad_value(280), p.p357, p.p65);

        s.store_add_scaled_inputs3_offset(294, s.ad_value(278), p.p358, s.ad_value(279), p.p359, s.ad_value(280), p.p360, p.p68);

        s.store_add_scaled_inputs3_offset(295, s.ad_value(278), p.p361, s.ad_value(279), p.p362, s.ad_value(280), p.p363, p.p276);

        s.store_add_scaled_inputs3_offset(250, s.ad_value(278), p.p751, s.ad_value(279), p.p752, s.ad_value(280), p.p753, p.p291);

        s.store_add_scaled_inputs3_offset(252, s.ad_value(278), p.p757, s.ad_value(279), p.p758, s.ad_value(280), p.p759, p.p294);

        s.store_add_scaled_inputs3_offset(251, s.ad_value(278), p.p754, s.ad_value(279), p.p755, s.ad_value(280), p.p756, p.p293);

        s.b[538] = (s.v[295] < 0.0);
        s.v[538] = if s.b[538] { 1.0 } else { 0.0 };

        if s.b[538] {
            s.store_scalar(295, 0.0);
        }

        s.b[539] = (s.v[295] > 1.0);
        s.v[539] = if s.b[539] { 1.0 } else { 0.0 };

        if ((!s.b[538]) && s.b[539]) {
            s.store_scalar(295, 1.0);
        }

        s.store_add_scaled_inputs3_offset(296, s.ad_value(278), p.p364, s.ad_value(279), p.p365, s.ad_value(280), p.p366, p.p277);

        s.store_add_scaled_inputs3_offset(297, s.ad_value(278), p.p367, s.ad_value(279), p.p368, s.ad_value(280), p.p369, p.p278);

        s.store_add_scaled_inputs3_offset(298, s.ad_value(278), p.p370, s.ad_value(279), p.p371, s.ad_value(280), p.p372, p.p275);

        s.store_add_scaled_inputs3_offset(299, s.ad_value(278), p.p373, s.ad_value(279), p.p374, s.ad_value(280), p.p375, p.p272);

        s.store_add_scaled_inputs3_offset(300, s.ad_value(278), p.p376, s.ad_value(279), p.p377, s.ad_value(280), p.p378, p.p273);

        s.store_add_scaled_inputs3_offset(301, s.ad_value(278), p.p379, s.ad_value(279), p.p380, s.ad_value(280), p.p381, p.p274);

        s.store_add_scaled_inputs3_offset(302, s.ad_value(278), p.p382, s.ad_value(279), p.p383, s.ad_value(280), p.p384, p.p283);

        s.b[540] = (s.v[302] < 0.0);
        s.v[540] = if s.b[540] { 1.0 } else { 0.0 };

        if s.b[540] {
            s.store_scalar(302, 0.0);
        }

        s.b[541] = (s.v[302] > 1.0);
        s.v[541] = if s.b[541] { 1.0 } else { 0.0 };

        if ((!s.b[540]) && s.b[541]) {
            s.store_scalar(302, 1.0);
        }

        s.store_add_scaled_inputs3_offset(303, s.ad_value(278), p.p385, s.ad_value(279), p.p386, s.ad_value(280), p.p387, p.p284);

        s.store_add_scaled_inputs3_offset(304, s.ad_value(278), p.p388, s.ad_value(279), p.p389, s.ad_value(280), p.p390, p.p285);

        s.store_add_scaled_inputs3_offset(305, s.ad_value(278), p.p391, s.ad_value(279), p.p392, s.ad_value(280), p.p393, p.p282);

        s.store_add_scaled_inputs3_offset(306, s.ad_value(278), p.p394, s.ad_value(279), p.p395, s.ad_value(280), p.p396, p.p279);

        s.store_add_scaled_inputs3_offset(307, s.ad_value(278), p.p397, s.ad_value(279), p.p398, s.ad_value(280), p.p399, p.p280);

        s.store_add_scaled_inputs3_offset(308, s.ad_value(278), p.p400, s.ad_value(279), p.p401, s.ad_value(280), p.p402, p.p281);

        s.store_add_scaled_inputs3_offset(313, s.ad_value(278), p.p403, s.ad_value(279), p.p404, s.ad_value(280), p.p405, p.p71);

        s.store_add_scaled_inputs3_offset(314, s.ad_value(278), p.p406, s.ad_value(279), p.p407, s.ad_value(280), p.p408, p.p72);

        s.store_add_scaled_inputs3_offset(315, s.ad_value(278), p.p409, s.ad_value(279), p.p410, s.ad_value(280), p.p411, p.p73);

        s.store_add_scaled_inputs3_offset(316, s.ad_value(278), p.p412, s.ad_value(279), p.p413, s.ad_value(280), p.p414, p.p74);

        s.store_add_scaled_inputs3_offset(317, s.ad_value(278), p.p415, s.ad_value(279), p.p416, s.ad_value(280), p.p417, p.p75);

        s.store_add_scaled_inputs3_offset(318, s.ad_value(278), p.p418, s.ad_value(279), p.p419, s.ad_value(280), p.p420, p.p84);

        s.store_add_scaled_inputs3_offset(319, s.ad_value(278), p.p421, s.ad_value(279), p.p422, s.ad_value(280), p.p423, p.p76);

        s.store_add_scaled_inputs3_offset(309, s.ad_value(278), p.p430, s.ad_value(279), p.p431, s.ad_value(280), p.p432, p.p87);

        s.store_add_scaled_inputs3_offset(310, s.ad_value(278), p.p433, s.ad_value(279), p.p434, s.ad_value(280), p.p435, p.p88);

        s.store_add_scaled_inputs3_offset(311, s.ad_value(278), p.p436, s.ad_value(279), p.p437, s.ad_value(280), p.p438, p.p61);

        s.store_add_scaled_inputs3_offset(312, s.ad_value(278), p.p439, s.ad_value(279), p.p440, s.ad_value(280), p.p441, p.p62);

        s.store_add_scaled_inputs3_offset(320, s.ad_value(278), p.p424, s.ad_value(279), p.p425, s.ad_value(280), p.p426, p.p85);

        s.store_add_scaled_inputs3_offset(321, s.ad_value(278), p.p427, s.ad_value(279), p.p428, s.ad_value(280), p.p429, p.p86);

        s.store_add_scaled_inputs3_offset(326, s.ad_value(278), p.p460, s.ad_value(279), p.p461, s.ad_value(280), p.p462, p.p113);

        s.store_add_scaled_inputs3_offset(322, s.ad_value(278), p.p442, s.ad_value(279), p.p443, s.ad_value(280), p.p444, p.p89);

        s.store_add_scaled_inputs3_offset(323, s.ad_value(278), p.p445, s.ad_value(279), p.p446, s.ad_value(280), p.p447, p.p90);

        s.store_add_scaled_inputs3_offset(324, s.ad_value(278), p.p448, s.ad_value(279), p.p449, s.ad_value(280), p.p450, p.p91);

        s.store_add_scaled_inputs3_offset(325, s.ad_value(278), p.p451, s.ad_value(279), p.p452, s.ad_value(280), p.p453, p.p92);

        s.store_add_scaled_inputs3_offset(417, s.ad_value(278), p.p454, s.ad_value(279), p.p455, s.ad_value(280), p.p456, p.p93);

        s.store_add_scaled_inputs3_offset(418, s.ad_value(278), p.p457, s.ad_value(279), p.p458, s.ad_value(280), p.p459, p.p94);

        s.store_add_scaled_inputs3_offset(327, s.ad_value(278), p.p463, s.ad_value(279), p.p464, s.ad_value(280), p.p465, p.p116);

        s.store_add_scaled_inputs3_offset(328, s.ad_value(278), p.p466, s.ad_value(279), p.p467, s.ad_value(280), p.p468, p.p123);

        s.store_add_scaled_inputs3_offset(329, s.ad_value(278), p.p469, s.ad_value(279), p.p470, s.ad_value(280), p.p471, p.p124);

        s.store_add_scaled_inputs3_offset(330, s.ad_value(278), p.p472, s.ad_value(279), p.p473, s.ad_value(280), p.p474, p.p122);

        s.store_add_scaled_inputs3_offset(331, s.ad_value(278), p.p475, s.ad_value(279), p.p476, s.ad_value(280), p.p477, p.p135);

        s.store_add_scaled_inputs3_offset(332, s.ad_value(278), p.p478, s.ad_value(279), p.p479, s.ad_value(280), p.p480, p.p139);

        s.store_add_scaled_inputs3_offset(333, s.ad_value(278), p.p481, s.ad_value(279), p.p482, s.ad_value(280), p.p483, p.p145);

        s.store_add_scaled_inputs3_offset(334, s.ad_value(278), p.p484, s.ad_value(279), p.p485, s.ad_value(280), p.p486, p.p148);

        s.store_add_scaled_inputs3_offset(335, s.ad_value(278), p.p487, s.ad_value(279), p.p488, s.ad_value(280), p.p489, p.p155);

        s.store_add_scaled_inputs3_offset(336, s.ad_value(278), p.p490, s.ad_value(279), p.p491, s.ad_value(280), p.p492, p.p142);

        s.store_add_scaled_inputs3_offset(342, s.ad_value(278), p.p493, s.ad_value(279), p.p494, s.ad_value(280), p.p495, p.p163);

        s.store_add_scaled_inputs3_offset(337, s.ad_value(278), p.p496, s.ad_value(279), p.p497, s.ad_value(280), p.p498, p.p157);

        s.store_add_scaled_inputs3_offset(338, s.ad_value(278), p.p499, s.ad_value(279), p.p500, s.ad_value(280), p.p501, p.p156);

        s.store_add_scaled_inputs3_offset(339, s.ad_value(278), p.p502, s.ad_value(279), p.p503, s.ad_value(280), p.p504, p.p158);

        s.store_add_scaled_inputs3_offset(340, s.ad_value(278), p.p505, s.ad_value(279), p.p506, s.ad_value(280), p.p507, p.p160);

        s.store_add_scaled_inputs3_offset(341, s.ad_value(278), p.p508, s.ad_value(279), p.p509, s.ad_value(280), p.p510, p.p161);

        s.store_add_scaled_inputs3_offset(343, s.ad_value(278), p.p511, s.ad_value(279), p.p512, s.ad_value(280), p.p513, p.p136);

        s.store_add_scaled_inputs3_offset(344, s.ad_value(278), p.p514, s.ad_value(279), p.p515, s.ad_value(280), p.p516, p.p166);

        s.store_add_scaled_inputs3_offset(345, s.ad_value(278), p.p517, s.ad_value(279), p.p518, s.ad_value(280), p.p519, p.p167);

        s.store_add_scaled_inputs3_offset(346, s.ad_value(278), p.p520, s.ad_value(279), p.p521, s.ad_value(280), p.p522, p.p173);

        s.store_add_scaled_inputs3_offset(347, s.ad_value(278), p.p523, s.ad_value(279), p.p524, s.ad_value(280), p.p525, p.p176);

        s.store_add_scaled_inputs3_offset(348, s.ad_value(278), p.p526, s.ad_value(279), p.p527, s.ad_value(280), p.p528, p.p182);

        s.store_add_scaled_inputs3_offset(349, s.ad_value(278), p.p529, s.ad_value(279), p.p530, s.ad_value(280), p.p531, p.p170);

        s.store_add_scaled_inputs3_offset(350, s.ad_value(278), p.p532, s.ad_value(279), p.p533, s.ad_value(280), p.p534, p.p183);

        s.store_add_scaled_inputs3_offset(351, s.ad_value(278), p.p535, s.ad_value(279), p.p536, s.ad_value(280), p.p537, p.p186);

        s.store_add_scaled_inputs3_offset(353, s.ad_value(278), p.p538, s.ad_value(279), p.p539, s.ad_value(280), p.p540, p.p119);

        s.store_add_scaled_inputs3_offset(354, s.ad_value(278), p.p541, s.ad_value(279), p.p542, s.ad_value(280), p.p543, p.p130);

        s.store_add_scaled_inputs3_offset(355, s.ad_value(278), p.p544, s.ad_value(279), p.p545, s.ad_value(280), p.p546, p.p205);

        s.store_add_scaled_inputs3_offset(356, s.ad_value(278), p.p547, s.ad_value(279), p.p548, s.ad_value(280), p.p549, p.p305);

        s.store_add_scaled_inputs3_offset(357, s.ad_value(278), p.p550, s.ad_value(279), p.p551, s.ad_value(280), p.p552, p.p306);

        s.store_add_scaled_inputs3_offset(358, s.ad_value(278), p.p553, s.ad_value(279), p.p554, s.ad_value(280), p.p555, p.p307);

        s.store_add_scaled_inputs3_offset(360, s.ad_value(278), p.p559, s.ad_value(279), p.p560, s.ad_value(280), p.p561, p.p210);

        s.store_add_scaled_inputs3_offset(361, s.ad_value(278), p.p562, s.ad_value(279), p.p563, s.ad_value(280), p.p564, p.p214);

        s.store_add_scaled_inputs3_offset(362, s.ad_value(278), p.p565, s.ad_value(279), p.p566, s.ad_value(280), p.p567, p.p208);

        s.store_add_scaled_inputs3_offset(363, s.ad_value(278), p.p568, s.ad_value(279), p.p569, s.ad_value(280), p.p570, p.p206);

        s.store_add_scaled_inputs3_offset(364, s.ad_value(278), p.p571, s.ad_value(279), p.p572, s.ad_value(280), p.p573, p.p207);

        s.store_add_scaled_inputs3_offset(365, s.ad_value(278), p.p574, s.ad_value(279), p.p575, s.ad_value(280), p.p576, p.p209);

        s.store_add_scaled_inputs3_offset(366, s.ad_value(278), p.p577, s.ad_value(279), p.p578, s.ad_value(280), p.p579, p.p256);

        s.store_add_scaled_inputs3_offset(367, s.ad_value(278), p.p580, s.ad_value(279), p.p581, s.ad_value(280), p.p582, p.p257);

        s.store_add_scaled_inputs3_offset(368, s.ad_value(278), p.p583, s.ad_value(279), p.p584, s.ad_value(280), p.p585, p.p258);

        s.store_add_scaled_inputs3_offset(408, s.ad_value(278), p.p706, s.ad_value(279), p.p707, s.ad_value(280), p.p708, p.p217);

        s.store_add_scaled_inputs3_offset(409, s.ad_value(278), p.p709, s.ad_value(279), p.p710, s.ad_value(280), p.p711, p.p218);

        s.store_add_scaled_inputs3_offset(410, s.ad_value(278), p.p712, s.ad_value(279), p.p713, s.ad_value(280), p.p714, p.p219);

        s.store_add_scaled_inputs3_offset(411, s.ad_value(278), p.p715, s.ad_value(279), p.p716, s.ad_value(280), p.p717, p.p220);

        s.store_add_scaled_inputs3_offset(412, s.ad_value(278), p.p718, s.ad_value(279), p.p719, s.ad_value(280), p.p720, p.p221);

        s.store_add_scaled_inputs3_offset(413, s.ad_value(278), p.p721, s.ad_value(279), p.p722, s.ad_value(280), p.p723, p.p222);

        s.store_add_scaled_inputs3_offset(414, s.ad_value(278), p.p724, s.ad_value(279), p.p725, s.ad_value(280), p.p726, p.p223);

        s.store_add_scaled_inputs3_offset(415, s.ad_value(278), p.p727, s.ad_value(279), p.p728, s.ad_value(280), p.p729, p.p224);

        s.store_add_scaled_inputs3_offset(416, s.ad_value(278), p.p730, s.ad_value(279), p.p731, s.ad_value(280), p.p732, p.p225);

        s.store_add_scaled_inputs3_offset(369, s.ad_value(278), p.p586, s.ad_value(279), p.p587, s.ad_value(280), p.p588, p.p226);

        s.store_add_scaled_inputs3_offset(370, s.ad_value(278), p.p589, s.ad_value(279), p.p590, s.ad_value(280), p.p591, p.p227);

        s.store_add_scaled_inputs3_offset(371, s.ad_value(278), p.p592, s.ad_value(279), p.p593, s.ad_value(280), p.p594, p.p228);

        s.store_add_scaled_inputs3_offset(373, s.ad_value(278), p.p595, s.ad_value(279), p.p596, s.ad_value(280), p.p597, p.p230);

        s.store_add_scaled_inputs3_offset(372, s.ad_value(278), p.p598, s.ad_value(279), p.p599, s.ad_value(280), p.p600, p.p229);

        s.store_add_scaled_inputs3_offset(381, s.ad_value(278), p.p610, s.ad_value(279), p.p611, s.ad_value(280), p.p612, p.p247);

        s.store_add_scaled_inputs3_offset(374, s.ad_value(278), p.p619, s.ad_value(279), p.p620, s.ad_value(280), p.p621, p.p250);

        s.store_add_scaled_inputs3_offset(375, s.ad_value(278), p.p622, s.ad_value(279), p.p623, s.ad_value(280), p.p624, p.p251);

        s.store_add_scaled_inputs3_offset(376, s.ad_value(278), p.p625, s.ad_value(279), p.p626, s.ad_value(280), p.p627, p.p252);

        s.store_add_scaled_inputs3_offset(377, s.ad_value(278), p.p628, s.ad_value(279), p.p629, s.ad_value(280), p.p630, p.p253);

        s.store_add_scaled_inputs3_offset(378, s.ad_value(278), p.p601, s.ad_value(279), p.p602, s.ad_value(280), p.p603, p.p244);

        s.store_add_scaled_inputs3_offset(379, s.ad_value(278), p.p604, s.ad_value(279), p.p605, s.ad_value(280), p.p606, p.p245);

        s.store_add_scaled_inputs3_offset(380, s.ad_value(278), p.p607, s.ad_value(279), p.p608, s.ad_value(280), p.p609, p.p246);

        s.store_add_scaled_inputs3_offset(390, s.ad_value(278), p.p613, s.ad_value(279), p.p614, s.ad_value(280), p.p615, p.p248);

        s.store_add_scaled_inputs3_offset(392, s.ad_value(278), p.p631, s.ad_value(279), p.p632, s.ad_value(280), p.p633, p.p254);

        s.store_add_scaled_inputs3_offset(391, s.ad_value(278), p.p616, s.ad_value(279), p.p617, s.ad_value(280), p.p618, p.p249);

        s.store_add_scaled_inputs3_offset(393, s.ad_value(278), p.p634, s.ad_value(279), p.p635, s.ad_value(280), p.p636, p.p255);

        s.store_add_scaled_inputs3_offset(382, s.ad_value(278), p.p637, s.ad_value(279), p.p638, s.ad_value(280), p.p639, p.p231);

        s.store_add_scaled_inputs3_offset(383, s.ad_value(278), p.p643, s.ad_value(279), p.p644, s.ad_value(280), p.p645, p.p232);

        s.store_add_scaled_inputs3_offset(384, s.ad_value(278), p.p649, s.ad_value(279), p.p650, s.ad_value(280), p.p651, p.p233);

        s.store_add_scaled_inputs3_offset(385, s.ad_value(278), p.p655, s.ad_value(279), p.p656, s.ad_value(280), p.p657, p.p242);

        s.store_add_scaled_inputs3_offset(386, s.ad_value(278), p.p640, s.ad_value(279), p.p641, s.ad_value(280), p.p642, p.p236);

        s.store_add_scaled_inputs3_offset(387, s.ad_value(278), p.p646, s.ad_value(279), p.p647, s.ad_value(280), p.p648, p.p237);

        s.store_add_scaled_inputs3_offset(388, s.ad_value(278), p.p652, s.ad_value(279), p.p653, s.ad_value(280), p.p654, p.p238);

        s.store_add_scaled_inputs3_offset(389, s.ad_value(278), p.p658, s.ad_value(279), p.p659, s.ad_value(280), p.p660, p.p243);

        s.store_add_scaled_inputs3_offset(394, s.ad_value(278), p.p664, s.ad_value(279), p.p665, s.ad_value(280), p.p666, p.p241);

        s.store_add_scaled_inputs3_offset(396, s.ad_value(278), p.p667, s.ad_value(279), p.p668, s.ad_value(280), p.p669, p.p259);

        s.store_add_scaled_inputs3_offset(397, s.ad_value(278), p.p670, s.ad_value(279), p.p671, s.ad_value(280), p.p672, p.p260);

        s.store_add_scaled_inputs3_offset(398, s.ad_value(278), p.p673, s.ad_value(279), p.p674, s.ad_value(280), p.p675, p.p261);

        s.store_add_scaled_inputs3_offset(399, s.ad_value(278), p.p676, s.ad_value(279), p.p677, s.ad_value(280), p.p678, p.p262);

        s.store_add_scaled_inputs3_offset(400, s.ad_value(278), p.p679, s.ad_value(279), p.p680, s.ad_value(280), p.p681, p.p100);

        s.store_add_scaled_inputs3_offset(401, s.ad_value(278), p.p682, s.ad_value(279), p.p683, s.ad_value(280), p.p684, p.p129);

        s.store_add_scaled_inputs3_offset(402, s.ad_value(278), p.p685, s.ad_value(279), p.p686, s.ad_value(280), p.p687, p.p103);

        s.store_add_scaled_inputs3_offset(403, s.ad_value(278), p.p688, s.ad_value(279), p.p689, s.ad_value(280), p.p690, p.p106);

        s.store_add_scaled_inputs3_offset(404, s.ad_value(278), p.p691, s.ad_value(279), p.p692, s.ad_value(280), p.p693, p.p110);

        s.store_add_scaled_inputs3_offset(405, s.ad_value(278), p.p694, s.ad_value(279), p.p695, s.ad_value(280), p.p696, p.p111);

        s.store_add_scaled_inputs3_offset(407, s.ad_value(278), p.p697, s.ad_value(279), p.p698, s.ad_value(280), p.p699, p.p112);

        s.store_add_scaled_inputs3_offset(406, s.ad_value(278), p.p700, s.ad_value(279), p.p701, s.ad_value(280), p.p702, p.p137);

        s.store_add_scaled_inputs3_offset(352, s.ad_value(278), p.p703, s.ad_value(279), p.p704, s.ad_value(280), p.p705, p.p187);

        s.store_add_scaled_inputs3_offset(62, s.ad_value(278), p.p739, s.ad_value(279), p.p740, s.ad_value(280), p.p741, p.p95);

        s.store_add_scaled_inputs3_offset(66, s.ad_value(278), p.p742, s.ad_value(279), p.p743, s.ad_value(280), p.p744, p.p96);

        s.store_add_scaled_inputs3_offset(67, s.ad_value(278), p.p745, s.ad_value(279), p.p746, s.ad_value(280), p.p747, p.p97);

        s.store_add_scaled_inputs3_offset(68, s.ad_value(278), p.p748, s.ad_value(279), p.p749, s.ad_value(280), p.p750, p.p98);

        s.b[542] = ((p.p20 == 1.0) && (p.p317 != 0.0));
        s.v[542] = if s.b[542] { 1.0 } else { 0.0 };

        if s.b[542] {
            s.store_add_scaled_inputs3_offset(275, s.ad_value(278), p.p733, s.ad_value(279), p.p734, s.ad_value(280), p.p735, p.p317);
        }

        if (!s.b[542]) {
            s.store_scalar(275, 0.0);
        }

        s.v[17] = ((3.9 * 8.85418e-12) / p.p45);

        s.v[18] = ((3.9 * 8.85418e-12) / p.p47);

        s.v[19] = ((3.9 * 8.85418e-12) / p.p46);

        s.v[20] = (s.v[16] / p.p49);

        s.v[21] = (p.p59 / 3.9);

        s.b[543] = (!param_given[47]);
        s.v[543] = if s.b[543] { 1.0 } else { 0.0 };

        if s.b[543] {
            s.store_scalar(221, (((p.p45 * p.p60) / 3.9) - p.p48));
        }

        if (!s.b[543]) {
            s.store_scalar(221, p.p47);
        }

        s.b[544] = (p.p138 > 0.0);
        s.v[544] = if s.b[544] { 1.0 } else { 0.0 };

        if s.b[544] {
            s.store_mul_sub_from_scalar_ad_rhs(331, 331, 1.0, A::mul(s.ad_value(406), A::powf(s.ad_value(2), (-p.p138))));
        }

        if (!s.b[544]) {
            s.store_mul_sub_from_scalar_rhs(331, 331, 1.0, 406);
        }

        s.store_ad_value(332, A::add_scaled_inputs(s.ad_value(332), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p141))), p.p140));

        s.store_ad_value(333, A::add_scaled_inputs(s.ad_value(333), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p147))), p.p146));

        s.store_offset_scaled_ad(137, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p153))), p.p152, p.p151);

        s.store_ad_value(334, A::add_scaled_inputs(s.ad_value(334), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p150))), p.p149));

        s.store_ad_value(336, A::add_scaled_inputs(s.ad_value(336), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p144))), p.p143));

        s.store_ad_value(342, A::add_scaled_inputs(s.ad_value(342), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p165))), p.p164));

        s.b[545] = (p.p188 > 0.0);
        s.v[545] = if s.b[545] { 1.0 } else { 0.0 };

        if s.b[545] {
            s.store_mul_sub_from_scalar_ad_rhs(344, 344, 1.0, A::mul(s.ad_value(352), A::powf(s.ad_value(2), (-p.p188))));
        }

        if (!s.b[545]) {
            s.store_mul_sub_from_scalar_rhs(344, 344, 1.0, 352);
        }

        s.store_ad_value(345, A::add_scaled_inputs(s.ad_value(345), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p169))), p.p168));

        s.store_ad_value(346, A::add_scaled_inputs(s.ad_value(346), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p175))), p.p174));

        s.store_offset_scaled_ad(138, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p181))), p.p180, p.p179);

        s.store_ad_value(347, A::add_scaled_inputs(s.ad_value(347), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p178))), p.p177));

        s.store_ad_value(349, A::add_scaled_inputs(s.ad_value(349), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p172))), p.p171));

        s.store_ad_value(350, A::add_scaled_inputs(s.ad_value(350), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p185))), p.p184));

        s.b[546] = (p.p14 == 1.0);
        s.v[546] = if s.b[546] { 1.0 } else { 0.0 };

        if (!s.b[546]) {
            s.store_ad_value(281, A::add_scaled_inputs(s.ad_value(281), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p193))), p.p192));
        }

        s.store_ad_value(360, A::add_scaled_inputs(s.ad_value(360), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p212))), p.p211));

        s.store_ad_value(326, A::add_scaled_inputs(s.ad_value(326), 1.0, A::powf(A::scale(s.ad_value(2), 1000000.0), (-p.p115)), p.p114));

        s.store_ad_value(327, A::add_scaled_inputs(s.ad_value(327), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p118))), p.p117));

        s.store_ad_value(328, A::add_scaled_inputs(s.ad_value(328), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p126))), p.p125));

        s.store_ad_value(329, A::add_scaled_inputs(s.ad_value(329), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p128))), p.p127));

        s.store_ad_value(400, A::add_scaled_inputs(s.ad_value(400), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p102))), p.p101));

        s.store_ad_value(401, A::add_scaled_inputs(s.ad_value(401), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p133))), p.p132));

        s.store_ad_value(402, A::add_scaled_inputs(s.ad_value(402), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p105))), p.p104));

        s.store_ad_value(403, A::add_scaled_inputs(s.ad_value(403), 1.0, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p108))), p.p107));

        s.store_offset_scaled_ad(92, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p80))), p.p79, p.p77);

        s.store_offset_scaled_ad(93, A::limited_exp_scaled_input(s.ad_value(2), (-1.0 / (p.p82))), p.p81, p.p78);

        s.b[547] = (s.v[331] < 0.0);
        s.v[547] = if s.b[547] { 1.0 } else { 0.0 };

        if s.b[547] {
            s.store_scalar(331, 0.03);
        }

        s.b[548] = (s.v[332] < 0.0);
        s.v[548] = if s.b[548] { 1.0 } else { 0.0 };

        if s.b[548] {
            s.store_scalar(332, 0.0);
        }

        s.b[549] = (s.v[336] < 0.0);
        s.v[549] = if s.b[549] { 1.0 } else { 0.0 };

        if s.b[549] {
            s.store_scalar(336, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.b[550] = (s.v[334] < 0.0);
        s.v[550] = if s.b[550] { 1.0 } else { 0.0 };

        if s.b[550] {
            s.store_scalar(334, 0.0);
        }

        s.b[551] = (s.v[335] < 0.0);
        s.v[551] = if s.b[551] { 1.0 } else { 0.0 };

        if s.b[551] {
            s.store_scalar(335, 0.0);
        }

        s.b[552] = (s.v[401] < 0.0);
        s.v[552] = if s.b[552] { 1.0 } else { 0.0 };

        if s.b[552] {
            s.store_scalar(401, 0.0);
        }

        s.v[134] = p.p190;

        s.b[555] = (s.v[134] < 0.0);
        s.v[555] = if s.b[555] { 1.0 } else { 0.0 };

        if s.b[555] {
            s.store_scalar(134, 0.0);
        }

        s.b[556] = (s.v[281] < 0.0);
        s.v[556] = if s.b[556] { 1.0 } else { 0.0 };

        if s.b[556] {
            s.store_scalar(281, 0.0);
        }

        s.b[561] = (s.v[284] < 0.0);
        s.v[561] = if s.b[561] { 1.0 } else { 0.0 };

        if s.b[561] {
            s.store_scalar(284, 0.0);
        }

        s.b[565] = (s.v[326] < 2.0);
        s.v[565] = if s.b[565] { 1.0 } else { 0.0 };

        if s.b[565] {
            s.store_scalar(326, 2.0);
        }

        s.store_offset_sqrt_ad(89, A::offset(A::div(s.ad_value(321), s.ad_value(2)), 1.0), (-1.0));

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

        s.b[566] = (p.p12 != 1.0);
        s.v[566] = if s.b[566] { 1.0 } else { 0.0 };

        if s.b[566] {
            s.store_scale(114, 343, (1.0 / 3.0));
            s.store_scalar(115, (1.0 / 3.0));
            s.store_scale(143, 351, (1.0 / 3.0));
        }

        s.v[129] = (1e-8 / (s.v[21] * p.p45));

        s.store_div_from_scalar_scaled_ad(131, 1.0, A::pow(A::scale(s.ad_value(3), 1000000.0), s.ad_value(286)), p.p2);

        s.v[253] = ((((s.v[21] * p.p45) * p.p49)) as f64).sqrt();

        s.v[144] = (1e-8 / (s.v[21] * p.p46));

        s.b[567] = (p.p296 >= (s.v[2] / 2.0));
        s.v[567] = if s.b[567] { 1.0 } else { 0.0 };

        if s.b[567] {
            s.store_scalar(249, 0.0);
        }

        if (!s.b[567]) {
            s.store_scalar(249, p.p296);
        }

        s.b[568] = ((p.p18 != 0.0) && (p.p310 > 0.0));
        s.v[568] = if s.b[568] { 1.0 } else { 0.0 };

        if s.b[568] {
            s.store_offset_scaled(270, 3, ((p.p2) * (p.p311)), ((p.p312) * (p.p311)));
        }

        if (!s.b[568]) {
            s.store_scalar(270, 0.0);
        }

        s.v[132] = (p.p215 * p.p7);

        s.v[133] = (p.p216 * p.p8);

        s.b[569] = (s.v[132] <= 0.001);
        s.v[569] = if s.b[569] { 1.0 } else { 0.0 };

        if s.b[569] {
            s.store_scalar(132, 0.001);
        }

        s.b[570] = (s.v[133] <= 0.001);
        s.v[570] = if s.b[570] { 1.0 } else { 0.0 };

        if s.b[570] {
            s.store_scalar(133, 0.001);
        }

        s.b[571] = (p.p14 == 1.0);
        s.v[571] = if s.b[571] { 1.0 } else { 0.0 };

        s.b[576] = (s.v[134] <= 0.0);
        s.v[576] = if s.b[576] { 1.0 } else { 0.0 };

        if ((!s.b[571]) && s.b[576]) {
            s.store_scalar(134, 0.0);
        }

        s.b[577] = (s.v[281] <= 0.0);
        s.v[577] = if s.b[577] { 1.0 } else { 0.0 };

        if ((!s.b[571]) && s.b[577]) {
            s.store_scalar(281, 0.0);
        }

        s.b[578] = (p.p297 <= 0.0);
        s.v[578] = if s.b[578] { 1.0 } else { 0.0 };

        if s.b[578] {
            s.store_scalar(95, 300.15);
        }

        if (!s.b[578]) {
            s.store_scalar(95, (p.p297 + 273.15));
        }

        s.b[580] = (p.p12 == 1.0);
        s.v[580] = if s.b[580] { 1.0 } else { 0.0 };

        if s.b[580] {
            s.store_scalar(206, 745669000000.0);
        }

        if (!s.b[580]) {
            s.store_scalar(206, 1166450000000.0);
        }

        s.v[34] = (p.p99 * p.p99);

        s.store_scale(35, 394, p.p99);

        s.store_square(36, 35);

        s.b[583] = ((p.p18 != 0.0) && (p.p310 > 0.0));
        s.v[583] = if s.b[583] { 1.0 } else { 0.0 };

        if s.b[583] {
            s.store_offset_voltage(271, ctx, nodes, Some(4), None, ((ctx_temp) + (p.p9)));
        }

        if (!s.b[583]) {
            s.store_scalar(271, (ctx_temp + p.p9));
        }

        s.v[272] = (p.p298 + 273.15);

        s.store_scaled_sub_ad(271, A::offset(s.ad_value(271), s.v[272]), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(271), (-s.v[272]), A::offset(s.ad_value(271), (-s.v[272]))), ((0.25 * 0.01) * 0.01))), 0.5);

        s.store_div(96, 271, 95);

        s.store_sub(97, 271, 95);

        s.store_scale(55, 271, 8.61708e-5);

        s.store_sub_from_scalar_ad(54, p.p55, A::div_scaled_product_offset_denominator(s.ad_value(271), s.ad_value(271), p.p299, s.ad_value(271), p.p300, 1.0));

        s.store_mul_scaled_ad_rhs(35, 271, 1.0 / (300.15), A::sqrt_scaled_input(s.ad_value(271), 1.0 / (300.15)));

        s.store_mul_scaled_ad_rhs(100, 35, p.p54, A::limited_exp(A::sub_from_scalar((p.p55 / ((2.0 * 8.61708e-5) * 300.15)), A::div_scaled_inputs(s.ad_value(54), 1.0, s.ad_value(55), 2.0))));

        s.store_mul_ln_ad_rhs(80, 55, A::max_with_scalar(A::div_scaled_product(s.ad_value(289), s.ad_value(290), 1.0, A::square(s.ad_value(100)), 1.0), 1e-38));

        s.store_mul_ln_ad_rhs(50, 55, A::max_with_scalar(A::div(s.ad_value(290), s.ad_value(100)), 1e-38));

        let assign3610_ad_e4240: A = A::add(A::add_scaled_product(s.ad_value(54), 0.5, s.ad_value(55), A::ln(A::max_with_scalar(A::div_from_scalar(p.p52, s.ad_value(100)), 1e-38)), (-1.0)), A::sqrt(A::offset(A::mul(A::add_scaled_product(s.ad_value(54), 0.5, s.ad_value(55), A::ln(A::max_with_scalar(A::div_from_scalar(p.p52, s.ad_value(100)), 1e-38)), (-1.0)), A::add_scaled_product(s.ad_value(54), 0.5, s.ad_value(55), A::ln(A::max_with_scalar(A::div_from_scalar(p.p52, s.ad_value(100)), 1e-38)), (-1.0))), ((4.0 * 0.0001) * 0.0001))));
        s.store_scaled_sub_ad_rhs(51, 54, assign3610_ad_e4240, 0.5);

        s.b[585] = ((p.p52 != 0.0) && (!param_given[58]));
        s.v[585] = if s.b[585] { 1.0 } else { 0.0 };

        s.b[586] = (p.p13 == (-1.0));
        s.v[586] = if s.b[586] { 1.0 } else { 0.0 };

        if (s.b[585] && s.b[586]) {
            s.store_add_ad_lhs(288, A::offset(s.ad_value(288), (-(0.5 * p.p55))), 51);
        }

        if (s.b[585] && (!s.b[586])) {
            s.store_sub_ad_lhs(288, A::offset(s.ad_value(288), (0.5 * p.p55)), 51);
        }

        s.store_offset_scaled(98, 54, 0.5, p.p53);

        s.store_mul_sub_rhs(52, 212, 287, 98);

        s.store_mul_sub_rhs(53, 212, 288, 98);

        s.store_add_scaled_product_mixed_aia(99, A::scale_offset(s.ad_value(54), 0.5, p.p53), 1.0, 212, A::min(A::scale(s.ad_value(54), 0.5), A::mul(s.ad_value(55), A::ln(A::max_with_scalar(A::div(s.ad_value(289), s.ad_value(100)), 1e-38)))), (-1.0));

        s.store_mul_sub_rhs(200, 212, 287, 99);

        s.store_mul_sub_rhs(240, 212, 288, 99);

        let assign3720_ad_e4372: A = A::mul3(s.ad_value(331), A::pow(s.ad_value(96), s.ad_value(338)), A::offset(A::add_scaled_inputs(A::offset(A::mul(s.ad_value(337), s.ad_value(97)), 0.9), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(337), s.ad_value(97)), 0.9, A::offset(A::mul(s.ad_value(337), s.ad_value(97)), 0.9)), ((4.0 * 0.001) * 0.001))), 0.5), ((1.0) + ((-(0.5 * (0.9 + ((((0.9 * 0.9) + ((4.0 * 0.001) * 0.001))) as f64).sqrt())))))));
        s.store_ad_value(126, assign3720_ad_e4372);

        s.store_mul_scale_ad_rhs(123, 333, A::add(A::scale_offset(s.ad_value(97), p.p159, ((1.0) + ((-1e-6)))), A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(97), p.p159, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(97), p.p159, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001)))), 0.5);

        s.store_mul_scale_ad_rhs(122, 332, A::add(A::offset(A::mul(s.ad_value(339), s.ad_value(97)), ((1.0) + ((-1e-6)))), A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(339), s.ad_value(97)), ((1.0) + ((-1e-6))), A::offset(A::mul(s.ad_value(339), s.ad_value(97)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001)))), 0.5);

        s.store_mul_pow_ad_rhs(125, 334, s.ad_value(96), s.ad_value(340));

        s.store_mul_pow_ad_rhs(124, 335, s.ad_value(96), s.ad_value(341));

        s.store_scaled_add_ad(150, A::offset(A::mul(s.ad_value(355), s.ad_value(97)), ((1.0) + ((-1e-6)))), A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(355), s.ad_value(97)), ((1.0) + ((-1e-6))), A::offset(A::mul(s.ad_value(355), s.ad_value(97)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5);

        s.store_mul_ad_rhs(353, 353, A::scale_offset(s.ad_value(278), p.p120, 1.0));

        s.store_mul_offset_ad_rhs(164, 400, A::add_scaled_inputs(A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97))), 0.5, A::sqrt(A::offset(A::mul_sub_from_scalar_lhs(0.9, A::mul(s.ad_value(353), s.ad_value(97)), A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97)))), ((4.0 * 0.001) * 0.001))), 0.5), ((1.0) + ((-(0.5 * (0.9 + ((((0.9 * 0.9) + ((4.0 * 0.001) * 0.001))) as f64).sqrt()))))));

        s.b[587] = (s.v[164] < 1000.0);
        s.v[587] = if s.b[587] { 1.0 } else { 0.0 };

        if s.b[587] {
            s.store_scalar(164, 1000.0);
        }

        s.store_mul_offset_ad_rhs(166, 402, A::add_scaled_inputs(A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97))), 0.5, A::sqrt(A::offset(A::mul_sub_from_scalar_lhs(0.9, A::mul(s.ad_value(353), s.ad_value(97)), A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97)))), ((4.0 * 0.001) * 0.001))), 0.5), ((1.0) + ((-(0.5 * (0.9 + ((((0.9 * 0.9) + ((4.0 * 0.001) * 0.001))) as f64).sqrt()))))));

        s.b[588] = (s.v[166] < 1000.0);
        s.v[588] = if s.b[588] { 1.0 } else { 0.0 };

        if s.b[588] {
            s.store_scalar(166, 1000.0);
        }

        s.store_mul_offset_ad_rhs(167, 403, A::add_scaled_inputs(A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97))), 0.5, A::sqrt(A::offset(A::mul_sub_from_scalar_lhs(0.9, A::mul(s.ad_value(353), s.ad_value(97)), A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97)))), ((4.0 * 0.001) * 0.001))), 0.5), ((1.0) + ((-(0.5 * (0.9 + ((((0.9 * 0.9) + ((4.0 * 0.001) * 0.001))) as f64).sqrt()))))));

        s.b[589] = (s.v[167] < 1000.0);
        s.v[589] = if s.b[589] { 1.0 } else { 0.0 };

        if s.b[589] {
            s.store_scalar(167, 1000.0);
        }

        s.store_mul_offset_ad_rhs(107, 316, A::add_scaled_inputs(A::scale_offset(s.ad_value(97), p.p309, (((-(-0.9))) + ((-0.0001)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(97), p.p309, (((-(-0.9))) + ((-0.0001)))), A::scale_offset(s.ad_value(97), p.p309, (((-(-0.9))) + ((-0.0001))))), (-((4.0 * (-0.9)) * 0.0001)))), 0.5), (((-0.9)) + (1.0)));

        s.store_mul_ad_rhs(354, 354, A::scale_offset(s.ad_value(278), p.p131, 1.0));

        s.store_mul_offset_ad_rhs(165, 401, A::add_scaled_inputs(A::sub_from_scalar(0.9, A::mul(s.ad_value(354), s.ad_value(97))), 0.5, A::sqrt(A::offset(A::mul_sub_from_scalar_lhs(0.9, A::mul(s.ad_value(354), s.ad_value(97)), A::sub_from_scalar(0.9, A::mul(s.ad_value(354), s.ad_value(97)))), ((4.0 * 0.001) * 0.001))), 0.5), ((1.0) + ((-(0.5 * (0.9 + ((((0.9 * 0.9) + ((4.0 * 0.001) * 0.001))) as f64).sqrt()))))));

        s.store_offset_ad(168, A::add_scaled_inputs(A::offset(A::mul(s.ad_value(326), A::scale_offset(s.ad_value(97), p.p121, 1.0)), (-2.0)), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(326), A::scale_offset(s.ad_value(97), p.p121, 1.0)), (-2.0), A::offset(A::mul(s.ad_value(326), A::scale_offset(s.ad_value(97), p.p121, 1.0)), (-2.0))), ((4.0 * 0.001) * 0.001))), 0.5), 2.0);

        s.store_add_scaled_product_indices(175, 322, 1.0, 323, 97, 1.0);

        let assign3930_ad_e4824: A = A::add_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(324), 1.0, s.ad_value(325), s.ad_value(97), 1.0), (-1e-6)), 0.5, A::sqrt(A::add_scaled_offset_product_lhs(s.ad_value(324), (-(-(4.0 * 1e-6))), A::add_scaled_product(s.ad_value(324), 1.0, s.ad_value(325), s.ad_value(97), 1.0), (-1e-6), A::offset(A::add_scaled_product(s.ad_value(324), 1.0, s.ad_value(325), s.ad_value(97), 1.0), (-1e-6)), 1.0)), 0.5);
        s.store_add_scaled_inputs3(176, s.ad_value(324), 1.0, assign3930_ad_e4824, 1.0, s.ad_value(324), (-1.0));

        s.store_add_scaled_product_indices(108, 417, 1.0, 418, 97, 1.0);

        s.store_mul_scale_ad_rhs(182, 327, A::add(A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(330), s.ad_value(97))), (-1e-6)), A::sqrt(A::offset(A::mul_offset_lhs(A::sub_from_scalar(1.0, A::mul(s.ad_value(330), s.ad_value(97))), (-1e-6), A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(330), s.ad_value(97))), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5);

        s.store_ad_value(102, A::mul_offset_lhs(A::div_from_scalar(p.p302, s.ad_value(2)), p.p301, A::offset(s.ad_value(96), (-1.0))));

        s.store_mul_pow_ad_rhs(103, 368, s.ad_value(96), s.ad_value(356));

        s.store_mul_scale_ad_rhs(104, 379, A::add(A::offset(A::mul(s.ad_value(357), s.ad_value(97)), ((1.0) + ((-1e-6)))), A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(357), s.ad_value(97)), ((1.0) + ((-1e-6))), A::offset(A::mul(s.ad_value(357), s.ad_value(97)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001)))), 0.5);

        s.store_mul_scale_ad_rhs(105, 375, A::add(A::offset(A::mul(s.ad_value(358), s.ad_value(97)), ((1.0) + ((-1e-6)))), A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(358), s.ad_value(97)), ((1.0) + ((-1e-6))), A::offset(A::mul(s.ad_value(358), s.ad_value(97)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001)))), 0.5);

        s.store_mul_voltage_ad(29, s.ad_value(212), ctx, nodes, Some(8), Some(6));

        s.store_mul_voltage_ad(30, s.ad_value(212), ctx, nodes, Some(5), Some(6));

        s.store_mul_voltage_ad(31, s.ad_value(212), ctx, nodes, Some(8), Some(5));

        s.store_mul_voltage_ad(32, s.ad_value(212), ctx, nodes, Some(3), Some(6));

        s.store_mul_voltage_ad(33, s.ad_value(212), ctx, nodes, Some(3), Some(5));

        s.store_mul_voltage_ad(209, s.ad_value(212), ctx, nodes, Some(8), Some(3));

        s.v[27] = 1.0;

        s.b[590] = (s.v[30] < 0.0);
        s.v[590] = if s.b[590] { 1.0 } else { 0.0 };

        if s.b[590] {
            s.store_scalar(27, (-1.0));
            s.copy_ad(22, 31);
            s.store_neg(26, 30);
            s.copy_ad(23, 33);
            s.copy_ad(24, 32);
        }

        if (!s.b[590]) {
            s.copy_ad(22, 29);
            s.copy_ad(26, 30);
            s.copy_ad(23, 32);
            s.copy_ad(24, 33);
        }

        s.store_mul_voltage_ad(234, s.ad_value(212), ctx, nodes, Some(7), Some(5));

        s.store_mul_voltage_ad(235, s.ad_value(212), ctx, nodes, Some(7), Some(6));

        s.store_offset_sqrt_ad(73, A::offset(A::square(s.ad_value(26)), 0.0004), (-0.02));

        s.store_scaled_sub(74, 73, 26, 0.5);

        s.store_add(25, 23, 74);

        s.store_sub(69, 22, 52);

        s.store_sub(70, 23, 53);

        s.v[77] = ((((s.v[21] * p.p49) * p.p45)) as f64).sqrt();

        s.v[76] = (((p.p49 * ((s.v[21] * p.p45) + (0.375 * p.p49)))) as f64).sqrt();

        s.store_add_scaled_inputs3(34, s.ad_value(69), ((p.p46 * s.v[21]) * 1.0 / (s.v[78])), s.ad_value(70), (((p.p45 * s.v[21]) + p.p49) * 1.0 / (s.v[78])), s.ad_value(74), 1.0);

        s.store_offset_scaled_ad(35, A::atan(A::add_scaled_product(s.ad_value(311), 1.0, s.ad_value(312), s.ad_value(34), 1.0)), 0.3183098861837907, 0.5);

        s.store_offset_scaled(75, 35, (s.v[77] - s.v[76]), s.v[76]);

        s.store_offset_ad(61, A::div_scaled_product(s.ad_value(314), s.ad_value(2), 1.0, s.ad_value(75), 1.0), 1e-6);

        s.b[591] = (s.v[61] < 40.0);
        s.v[591] = if s.b[591] { 1.0 } else { 0.0 };

        if s.b[591] {
            s.store_div_from_scalar_offset_ad(88, 0.5, A::cosh(s.ad_value(61)), (-1.0));
        }

        if (!s.b[591]) {
            s.store_limited_exp_neg_input(88, 61);
        }

        s.store_offset_ad(61, A::div_scaled_product(s.ad_value(319), s.ad_value(2), 1.0, s.ad_value(75), 1.0), 1e-6);

        s.b[592] = (s.v[61] < 40.0);
        s.v[592] = if s.b[592] { 1.0 } else { 0.0 };

        if s.b[592] {
            s.store_div_from_scalar_offset_ad(90, 0.5, A::cosh(s.ad_value(61)), (-1.0));
        }

        if (!s.b[592]) {
            s.store_limited_exp_neg_input(90, 61);
        }

        s.b[593] = (s.v[61] < 40.0);
        s.v[593] = if s.b[593] { 1.0 } else { 0.0 };

        if s.b[593] {
            s.store_div_from_scalar_ad(91, 1.0, A::max_with_scalar(A::scale_offset(A::cosh(s.ad_value(61)), p.p83, (((((-2.0)) * (p.p83))) + (1.0))), 1e-6));
        }

        if (!s.b[593]) {
            s.store_div_ad(91, A::limited_exp_scaled_input(s.ad_value(61), -1.0), A::max_with_scalar(A::offset(A::limited_exp_scaled_input(s.ad_value(61), -1.0), p.p83), 1e-6));
        }

        s.store_offset_ad(61, A::div_scaled_product(s.ad_value(362), s.ad_value(2), 1.0, s.ad_value(75), 1.0), 1e-6);

        s.b[594] = (s.v[61] < 40.0);
        s.v[594] = if s.b[594] { 1.0 } else { 0.0 };

        if s.b[594] {
            s.store_add_ad_lhs(153, A::div_scaled_value_offset_denominator(s.ad_value(363), 0.5, A::cosh(s.ad_value(61)), (-1.0), 1.0), 364);
        }

        if (!s.b[594]) {
            s.store_add_scaled_product_right_ad(153, 364, 1.0, 363, A::limited_exp_scaled_input(s.ad_value(61), -1.0), 1.0);
        }

        s.b[595] = (p.p13 == (-1.0));
        s.v[595] = if s.b[595] { 1.0 } else { 0.0 };

        if s.b[595] {
            s.store_div_scaled_product_indices(79, 298, 2, 1.0, 75, 1.0);
        }

        s.b[596] = (s.v[79] > 40.0);
        s.v[596] = if s.b[596] { 1.0 } else { 0.0 };

        if (s.b[595] && s.b[596]) {
            s.store_scaled_limited_exp(34, 79, 0.5);
        }

        if (s.b[595] && (!s.b[596])) {
            s.store_offset_cosh_ad(34, s.ad_value(79), (-1.0));
        }

        if s.b[595] {
            s.store_sub_ad_rhs(35, 299, A::div_scaled_inputs(s.ad_value(300), 0.5, s.ad_value(34), 1.0));
            s.copy_ad(36, 301);
            s.copy_ad(246, 296);
            s.copy_ad(247, 297);
            s.copy_ad(248, 295);
        }

        if (!s.b[595]) {
            s.store_div_scaled_product_indices(79, 305, 2, 1.0, 75, 1.0);
        }

        s.b[597] = (s.v[79] > 40.0);
        s.v[597] = if s.b[597] { 1.0 } else { 0.0 };

        if ((!s.b[595]) && s.b[597]) {
            s.store_scaled_limited_exp(34, 79, 0.5);
        }

        if ((!s.b[595]) && (!s.b[597])) {
            s.store_offset_cosh_ad(34, s.ad_value(79), (-1.0));
        }

        if (!s.b[595]) {
            s.store_sub_ad_rhs(35, 306, A::div_scaled_inputs(s.ad_value(307), 0.5, s.ad_value(34), 1.0));
            s.copy_ad(36, 308);
            s.copy_ad(246, 303);
            s.copy_ad(247, 304);
            s.copy_ad(248, 302);
        }

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_sub(34, 35, 36);

        s.store_add_scaled_inputs3(241, s.ad_value(36), 1.0, s.ad_value(34), 0.5, A::sqrt(A::offset(A::square(s.ad_value(34)), 0.0001)), 0.5);

        s.v[244] = (((1.60219e-19 * p.p52) * s.v[16]) / ((2.0 * s.v[19]) * s.v[19]));

        s.b[598] = (p.p52 != 0.0);
        s.v[598] = if s.b[598] { 1.0 } else { 0.0 };

        if s.b[598] {
            let assign4690_ad_e5404: A = A::add_scaled_product(A::sqrt(A::offset(A::mul3(s.ad_value(213), A::add_scaled_product(s.ad_value(246), (-1.0), s.ad_value(212), s.ad_value(25), 1.0), A::mul(s.ad_value(213), A::add_scaled_product(s.ad_value(246), (-1.0), s.ad_value(212), s.ad_value(25), 1.0))), ((4.0 * 0.01) * 0.01))), 1.0, s.ad_value(213), A::add_scaled_product(s.ad_value(246), (-1.0), s.ad_value(212), s.ad_value(25), 1.0), 1.0);
            s.store_offset_sqrt_ad(34, A::scale_offset(assign4690_ad_e5404, (0.5 * 1.0 / (s.v[244])), 1.0), (-1.0));
        }

        if (!s.b[598]) {
            s.store_scalar(34, 0.0);
        }

        s.store_scaled_mul(245, 34, 34, s.v[244]);

        s.store_neg_ad(245, A::sub(A::add_scaled_inputs3_offset(s.ad_value(245), (-0.5), s.ad_value(247), 0.5, A::sqrt(A::add_scaled_offset_product_lhs(s.ad_value(247), (-(-(4.0 * 0.01))), A::sub_scaled_inputs(s.ad_value(245), -1.0, s.ad_value(247), -1.0), (-0.01), A::offset(A::sub_scaled_inputs(s.ad_value(245), -1.0, s.ad_value(247), -1.0), (-0.01)), 1.0)), 0.5, ((-0.01) * 0.5)), s.ad_value(247)));

        s.store_sub_from_scalar(72, (-1.2), 74);

        s.v[243] = (((-s.v[19]) * s.v[20]) / ((s.v[19] + s.v[20]) * s.v[17]));

        s.store_mul_scaled_ad_rhs(242, 241, s.v[243], A::sub(A::add_scaled_product(s.ad_value(70), 1.0, A::mul3(s.ad_value(212), s.ad_value(213), s.ad_value(248)), s.ad_value(245), (-1.0)), s.ad_value(72)));

        s.store_scaled_add_ad_rhs(28, 25, A::sqrt(A::offset(A::square(s.ad_value(25)), ((4.0 * 0.001) * 0.001))), 0.5);

        s.store_add_ad_lhs(87, A::offset(s.ad_value(50), 0.4), 315);

        s.b[599] = (s.v[87] < 0.0);
        s.v[599] = if s.b[599] { 1.0 } else { 0.0 };

        if s.b[599] {
            s.store_scalar(84, 0.0);
        }

        if (!s.b[599]) {
            s.store_mul_ad_product_rhs(84, 320, s.ad_value(89), A::sqrt(s.ad_value(87)));
        }

        s.store_mul_ad_affine_product_rhs(83, 313, s.ad_value(88), A::sub(s.ad_value(80), s.ad_value(87)), -1.0, 0.0);

        s.store_add_ad(82, A::mul3_scaled_output(A::add_scaled_product(s.ad_value(107), 1.0, s.ad_value(318), s.ad_value(25), 1.0), s.ad_value(90), A::add_scaled_product(s.ad_value(73), 1.0, s.ad_value(317), A::sqrt(A::offset(s.ad_value(73), 0.01)), 1.0), -1.0), A::mul3(s.ad_value(92), s.ad_value(91), A::pow(A::offset(s.ad_value(73), 0.01), s.ad_value(93))));

        s.store_mul_ad_lhs(85, A::div_scaled_inputs(s.ad_value(309), -1.0, A::add(s.ad_value(2), s.ad_value(310)), 1.0), 73);

        s.v[35] = ((s.v[20] * s.v[19]) / (s.v[20] + s.v[19]));

        s.store_mul_ad_lhs(36, A::add_scaled_inputs(s.ad_value(293), 1.0, s.ad_value(28), p.p70), 73);

        s.store_add_scaled_value_products(37, s.ad_value(25), p.p66, s.ad_value(25), s.ad_value(25), p.p67, s.ad_value(88), A::add(A::add_scaled_value_products(s.ad_value(292), 1.0, s.ad_value(294), s.ad_value(25), 1.0, s.ad_value(25), s.ad_value(25), p.p69), s.ad_value(36)), 1.0);

        s.store_mul_scaled_ad_rhs(81, 55, 1.0 / ((s.v[17] + s.v[35])), A::add(A::offset(s.ad_value(291), (s.v[17] + s.v[35])), s.ad_value(37)));

        s.store_scale(60, 290, ((1.60219e-19 * (p.p49 * 1.0 / (s.v[17]))) * (1.0 - ((0.5 * p.p49) / (p.p49 + (s.v[21] * p.p46))))));

        s.store_mul_offset_ad_lhs(34, A::div_from_scalar(p.p304, s.ad_value(2)), p.p303, 25);

        s.store_add_scaled_offset_product_rhs(101, 102, 1.0, 34, 96, (-1.0), 1.0);

        s.store_add_ad_lhs(86, A::add(A::add(A::add_scaled_inputs4(s.ad_value(83), 1.0, s.ad_value(82), 1.0, s.ad_value(84), 1.0, s.ad_value(85), 1.0), s.ad_value(60)), s.ad_value(101)), 242);

        s.store_offset_sub(71, 69, 86, p.p10);

        s.store_div_scaled_inputs(421, s.ad_value(100), ((2.0 * 1.60219e-19) * (p.p49 * p.p49)), s.ad_value(55), s.v[16]);

        s.v[419] = (s.v[17] / s.v[20]);

        s.v[420] = (s.v[19] / s.v[20]);

        s.store_ln(449, 421);

        s.store_sub_from_scalar(450, ((39.47841) as f64).ln(), 449);

        s.v[451] = (s.v[419] * s.v[419]);

        s.v[454] = (s.v[419] / (((s.v[420] * s.v[419]) + s.v[420]) + s.v[419]));

        s.v[460] = 1.0;

        s.store_sub_from_scalar_ad(461, ((s.v[451] * s.v[460]) * s.v[460]), A::mul(s.ad_value(421), A::limited_exp_scaled_input(s.ad_value(50), 2.0)));

        s.store_sqrt(462, 461);

        s.store_div_ad(463, A::sub_from_scalar(1.0, A::scale(s.ad_value(462), 0.125)), A::sub_from_scalar(0.5, A::scale(s.ad_value(462), 0.041666666666666664)));

        s.store_mul_sub_ad_lhs(35, A::offset(A::ln(A::max_with_scalar(A::scale_offset(s.ad_value(463), (s.v[419] * s.v[460]), (((s.v[419] * s.v[419]) * s.v[460]) * s.v[460])), 1e-38)), 1.0), A::ln(A::max_with_scalar(s.ad_value(421), 1e-38)), 55);

        s.store_div(422, 71, 81);

        s.store_div_scaled_offset_numerator(423, A::sub(s.ad_value(70), s.ad_value(86)), 1.0, p.p10, s.ad_value(81), 1.0);

        s.store_sub_ad_lhs(453, A::ln(A::offset(A::mul_scaled_lhs(A::sub(s.ad_value(422), s.ad_value(450)), s.v[451], A::sub(s.ad_value(422), s.ad_value(450))), 39.47841)), 449);

        s.store_sub_ad_lhs(424, A::ln(A::offset(A::mul_scaled_lhs(A::sub(s.ad_value(422), s.ad_value(450)), s.v[451], A::sub(s.ad_value(422), s.ad_value(450))), 39.47841)), 449);

        s.store_add_scaled_inputs(452, 424, 1.0 / ((1.0 + s.v[420])), 423, (s.v[420] * 1.0 / ((1.0 + s.v[420]))));

        s.store_add_scaled_inputs3(426, s.ad_value(423), 1.0, s.ad_value(422), s.v[454], s.ad_value(423), (-s.v[454]));

        s.store_min(430, 426, 453);

        s.store_min(430, 430, 450);

        s.store_add_scaled_inputs(448, 430, 1.0 / ((1.0 + s.v[419])), 422, (s.v[419] * 1.0 / ((1.0 + s.v[419]))));

        s.store_sub(34, 448, 430);

        s.store_div_scaled_product_offset_rhs(37, A::limited_exp(s.ad_value(430)), A::limited_exp(s.ad_value(34)), (-1.0), 1.0, s.ad_value(34), 1.0);

        s.store_sub(429, 423, 452);

        s.store_add_scaled_products_right_right_ad(442, 429, 429, (s.v[420] * s.v[420]), 421, A::exp(s.ad_value(452)), (-1.0));

        s.b[600] = (s.v[442] < 0.0);
        s.v[600] = if s.b[600] { 1.0 } else { 0.0 };

        if s.b[600] {
            s.store_scaled_sub(429, 423, 430, s.v[420]);
            s.store_scalar(440, (40.0 * s.v[419]));
            s.store_add(455, 440, 429);
            s.store_mul(37, 440, 429);
            s.store_offset_scaled(38, 455, 0.06534, 1.0);
            s.store_offset_ad(39, A::add_scaled_inputs(s.ad_value(455), 8.57973, s.ad_value(37), 1.0), 39.47841);
            s.store_add_scaled_inputs(40, 455, 78.95683, 37, 39.47841);
            s.store_div_scaled_inputs2(442, A::sqrt(A::add_scaled_square_product(s.ad_value(39), 1.0, s.ad_value(38), s.ad_value(40), (-4.0))), 1.0, s.ad_value(39), (-1.0), s.ad_value(38), 2.0);
            s.store_sub_scaled_inputs(37, 450, ((1.0 + s.v[419]) * 1.0 / (s.v[419])), 430, 1.0 / (s.v[419]));
            s.store_offset_scaled_sub(34, 422, 37, (40.0 * 0.2), ((2.0) * ((40.0 * 0.2))));
            s.store_mul_sub_from_scalar_ad_rhs(442, 442, 1.0, A::exp_scaled_input(A::offset(A::sub(s.ad_value(422), s.ad_value(37)), 2.0), (-1.0 / ((2.0 / 0.69)))));
            s.store_min_with_scalar(442, 442, 50.0);
        }

        s.store_max(422, 422, 450);

        s.store_sub_ad_lhs(424, A::ln(A::offset(A::mul_scaled_lhs(A::sub(s.ad_value(422), s.ad_value(450)), s.v[451], A::sub(s.ad_value(422), s.ad_value(450))), 39.47841)), 449);

        s.store_sub_scaled_inputs(37, 450, ((1.0 + s.v[419]) * 1.0 / (s.v[419])), 430, 1.0 / (s.v[419]));

        s.store_sub_ad_lhs(38, A::ln(A::offset(A::mul_scaled_lhs(A::sub(s.ad_value(37), s.ad_value(450)), s.v[451], A::sub(s.ad_value(37), s.ad_value(450))), 39.47841)), 449);

        s.store_sub(39, 38, 450);

        s.store_sub(424, 424, 39);

        s.store_sub(440, 422, 424);

        s.store_mul_scaled_ad_rhs(34, 421, -1.0, A::exp(s.ad_value(424)));

        s.store_scale(35, 440, s.v[451]);

        s.store_div_scaled_inputs2(425, A::add_scaled_product(s.ad_value(34), 1.0, s.ad_value(35), s.ad_value(440), 1.0), -1.0, s.ad_value(442), 1.0, A::add_scaled_inputs(s.ad_value(35), (-2.0), s.ad_value(34), 1.0), 1.0);

        s.store_add(424, 424, 425);

        s.store_sub(440, 422, 424);

        s.store_scale(36, 440, s.v[451]);

        s.store_div_from_scalar_ad(34, 1.0, A::add_scaled_product(s.ad_value(442), (-1.0), s.ad_value(36), s.ad_value(440), 1.0));

        s.store_add_scaled_inputs3(465, A::ln(A::abs(A::add_scaled_product(s.ad_value(442), (-1.0), s.ad_value(36), s.ad_value(440), 1.0))), 1.0, s.ad_value(449), (-1.0), s.ad_value(424), -1.0);

        s.store_div_from_scalar_offset_ad(466, 1.0, A::mul_scaled_lhs(s.ad_value(36), (-2.0), s.ad_value(34)), (-1.0));

        s.store_add_scaled_product_left_ad(467, 34, (2.0 * s.v[451]), A::mul3_scaled_output(s.ad_value(36), s.ad_value(36), s.ad_value(34), (-4.0)), 34, 1.0);

        s.store_mul(35, 465, 466);

        s.store_add_scaled_product_left_ad(425, 35, -1.0, A::mul3_scaled_output(s.ad_value(35), s.ad_value(35), s.ad_value(467), 0.5), 466, (-1.0));

        s.store_max_with_scalar(425, 425, (-10.0));

        s.store_min_with_scalar(425, 425, 10.0);

        s.store_add(424, 424, 425);

        s.store_sub(440, 422, 424);

        s.store_scale(36, 440, s.v[451]);

        s.store_div_from_scalar_ad(34, 1.0, A::add_scaled_product(s.ad_value(442), (-1.0), s.ad_value(36), s.ad_value(440), 1.0));

        s.store_add_scaled_inputs3(465, A::ln(A::abs(A::add_scaled_product(s.ad_value(442), (-1.0), s.ad_value(36), s.ad_value(440), 1.0))), 1.0, s.ad_value(449), (-1.0), s.ad_value(424), -1.0);

        s.store_div_from_scalar_offset_ad(466, 1.0, A::mul_scaled_lhs(s.ad_value(36), (-2.0), s.ad_value(34)), (-1.0));

        s.store_add_scaled_product_left_ad(467, 34, (2.0 * s.v[451]), A::mul3_scaled_output(s.ad_value(36), s.ad_value(36), s.ad_value(34), (-4.0)), 34, 1.0);

        s.store_mul(35, 465, 466);

        s.store_add_scaled_product_left_ad(425, 35, -1.0, A::mul3_scaled_output(s.ad_value(35), s.ad_value(35), s.ad_value(467), 0.5), 466, (-1.0));

        s.store_max_with_scalar(425, 425, (-10.0));

        s.store_min_with_scalar(425, 425, 10.0);

        s.store_add(424, 424, 425);

        s.store_max_ad(424, s.ad_value(424), A::offset(s.ad_value(450), (-4.0)));

        s.store_div(422, 71, 81);

        s.store_sub_ad_rhs(448, 448, A::ln_one_plus_exp(A::sub_scaled_inputs(s.ad_value(448), 1.0, s.ad_value(424), 1.05)));

        s.store_min(448, 448, 424);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_scaled_ad_rhs(457, 421, -1.0, A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.b[601] = (s.v[442] < 0.0);
        s.v[601] = if s.b[601] { 1.0 } else { 0.0 };

        if s.b[601] {
            s.store_sqrt_neg_input(439, 442);
            s.store_div_from_scalar_sin_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_mul_cos_ad_lhs(458, A::scale(s.ad_value(439), 0.5), 459);
            s.store_div_scaled_inputs(34, s.ad_value(458), (-0.5), s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, 0.25, 34, 1.0);
        }

        if (!s.b[601]) {
            s.store_sqrt(439, 442);
            s.store_div_from_scalar_sinh_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_sqrt_offset_input(458, 35, 1.0);
            s.store_div_scaled_inputs(34, s.ad_value(458), 0.5, s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, (-0.25), 34, 1.0);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_add_scaled_inputs4(429, s.ad_value(423), 1.0, s.ad_value(422), (-1.0), s.ad_value(440), 1.0, A::ln(A::abs(A::mul(A::mul3(s.ad_value(442), s.ad_value(35), s.ad_value(37)), s.ad_value(37)))), -1.0);

        s.store_add_scaled_product_mixed_iaa(427, 457, 1.0, A::add(s.ad_value(456), s.ad_value(446)), A::add_scaled_inputs(s.ad_value(429), s.v[420], s.ad_value(456), 1.0), 1.0);

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_scaled_inputs(443, 456, ((-2.0) * s.v[419]), 457, 1.0);

        s.store_mul(444, 445, 443);

        s.store_add_scaled_product_value_ad(441, A::offset(A::mul_scaled_output(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37), 2.0), (-1.0)), 1.0, 447, 443, (-1.0));

        s.store_add_ad(428, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(457), 1.0, s.ad_value(456), (-s.v[419]), s.ad_value(36), (-s.v[419])), 1.0, s.ad_value(456), s.ad_value(444), 1.0), A::add_scaled_products(s.ad_value(441), s.ad_value(36), s.v[420], s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])), s.v[420]));

        s.store_div_scaled_inputs(425, s.ad_value(427), -1.0, s.ad_value(428), 1.0);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_scaled_ad_rhs(457, 421, -1.0, A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.b[602] = (s.v[442] < 0.0);
        s.v[602] = if s.b[602] { 1.0 } else { 0.0 };

        if s.b[602] {
            s.store_sqrt_neg_input(439, 442);
            s.store_div_from_scalar_sin_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_mul_cos_ad_lhs(458, A::scale(s.ad_value(439), 0.5), 459);
            s.store_div_scaled_inputs(34, s.ad_value(458), (-0.5), s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, 0.25, 34, 1.0);
        }

        if (!s.b[602]) {
            s.store_sqrt(439, 442);
            s.store_div_from_scalar_sinh_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_sqrt_offset_input(458, 35, 1.0);
            s.store_div_scaled_inputs(34, s.ad_value(458), 0.5, s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, (-0.25), 34, 1.0);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_add_scaled_inputs4(429, s.ad_value(423), 1.0, s.ad_value(422), (-1.0), s.ad_value(440), 1.0, A::ln(A::abs(A::mul(A::mul3(s.ad_value(442), s.ad_value(35), s.ad_value(37)), s.ad_value(37)))), -1.0);

        s.store_add_scaled_product_mixed_iaa(427, 457, 1.0, A::add(s.ad_value(456), s.ad_value(446)), A::add_scaled_inputs(s.ad_value(429), s.v[420], s.ad_value(456), 1.0), 1.0);

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_scaled_inputs(443, 456, ((-2.0) * s.v[419]), 457, 1.0);

        s.store_mul(444, 445, 443);

        s.store_add_scaled_product_value_ad(441, A::offset(A::mul_scaled_output(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37), 2.0), (-1.0)), 1.0, 447, 443, (-1.0));

        s.store_add_ad(428, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(457), 1.0, s.ad_value(456), (-s.v[419]), s.ad_value(36), (-s.v[419])), 1.0, s.ad_value(456), s.ad_value(444), 1.0), A::add_scaled_products(s.ad_value(441), s.ad_value(36), s.v[420], s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])), s.v[420]));

        s.store_div_scaled_inputs(425, s.ad_value(427), -1.0, s.ad_value(428), 1.0);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_scaled_ad_rhs(457, 421, -1.0, A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.b[603] = (s.v[442] < 0.0);
        s.v[603] = if s.b[603] { 1.0 } else { 0.0 };

        if s.b[603] {
            s.store_sqrt_neg_input(439, 442);
            s.store_div_from_scalar_sin_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_mul_cos_ad_lhs(458, A::scale(s.ad_value(439), 0.5), 459);
            s.store_div_scaled_inputs(34, s.ad_value(458), (-0.5), s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, 0.25, 34, 1.0);
        }

        if (!s.b[603]) {
            s.store_sqrt(439, 442);
            s.store_div_from_scalar_sinh_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_sqrt_offset_input(458, 35, 1.0);
            s.store_div_scaled_inputs(34, s.ad_value(458), 0.5, s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, (-0.25), 34, 1.0);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_add_scaled_inputs4(429, s.ad_value(423), 1.0, s.ad_value(422), (-1.0), s.ad_value(440), 1.0, A::ln(A::abs(A::mul(A::mul3(s.ad_value(442), s.ad_value(35), s.ad_value(37)), s.ad_value(37)))), -1.0);

        s.store_add_scaled_product_mixed_iaa(427, 457, 1.0, A::add(s.ad_value(456), s.ad_value(446)), A::add_scaled_inputs(s.ad_value(429), s.v[420], s.ad_value(456), 1.0), 1.0);

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_scaled_inputs(443, 456, ((-2.0) * s.v[419]), 457, 1.0);

        s.store_mul(444, 445, 443);

        s.store_add_scaled_product_value_ad(441, A::offset(A::mul_scaled_output(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37), 2.0), (-1.0)), 1.0, 447, 443, (-1.0));

        s.store_add_ad(428, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(457), 1.0, s.ad_value(456), (-s.v[419]), s.ad_value(36), (-s.v[419])), 1.0, s.ad_value(456), s.ad_value(444), 1.0), A::add_scaled_products(s.ad_value(441), s.ad_value(36), s.v[420], s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])), s.v[420]));

        s.store_div_scaled_inputs(425, s.ad_value(427), -1.0, s.ad_value(428), 1.0);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_scaled_ad_rhs(457, 421, -1.0, A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.b[604] = (s.v[442] < 0.0);
        s.v[604] = if s.b[604] { 1.0 } else { 0.0 };

        if s.b[604] {
            s.store_sqrt_neg_input(439, 442);
            s.store_div_from_scalar_sin_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
        }

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[604] {
            s.store_square(35, 459);
            s.store_mul_cos_ad_lhs(458, A::scale(s.ad_value(439), 0.5), 459);
            s.store_div_scaled_inputs(34, s.ad_value(458), (-0.5), s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, 0.25, 34, 1.0);
        }

        if (!s.b[604]) {
            s.store_sqrt(439, 442);
            s.store_div_from_scalar_sinh_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_sqrt_offset_input(458, 35, 1.0);
            s.store_div_scaled_inputs(34, s.ad_value(458), 0.5, s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, (-0.25), 34, 1.0);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_add_scaled_inputs4(429, s.ad_value(423), 1.0, s.ad_value(422), (-1.0), s.ad_value(440), 1.0, A::ln(A::abs(A::mul(A::mul3(s.ad_value(442), s.ad_value(35), s.ad_value(37)), s.ad_value(37)))), -1.0);

        s.store_add_scaled_product_mixed_iaa(427, 457, 1.0, A::add(s.ad_value(456), s.ad_value(446)), A::add_scaled_inputs(s.ad_value(429), s.v[420], s.ad_value(456), 1.0), 1.0);

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_scaled_inputs(443, 456, ((-2.0) * s.v[419]), 457, 1.0);

        s.store_mul(444, 445, 443);

        s.store_add_scaled_product_value_ad(441, A::offset(A::mul_scaled_output(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37), 2.0), (-1.0)), 1.0, 447, 443, (-1.0));

        s.store_add_ad(428, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(457), 1.0, s.ad_value(456), (-s.v[419]), s.ad_value(36), (-s.v[419])), 1.0, s.ad_value(456), s.ad_value(444), 1.0), A::add_scaled_products(s.ad_value(441), s.ad_value(36), s.v[420], s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])), s.v[420]));

        s.store_div_scaled_inputs(425, s.ad_value(427), -1.0, s.ad_value(428), 1.0);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_scaled_ad_rhs(457, 421, -1.0, A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.b[605] = (s.v[442] < 0.0);
        s.v[605] = if s.b[605] { 1.0 } else { 0.0 };

        if s.b[605] {
            s.store_sqrt_neg_input(439, 442);
            s.store_div_from_scalar_sin_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_mul_cos_ad_lhs(458, A::scale(s.ad_value(439), 0.5), 459);
            s.store_div_scaled_inputs(34, s.ad_value(458), (-0.5), s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, 0.25, 34, 1.0);
        }

        if (!s.b[605]) {
            s.store_sqrt(439, 442);
            s.store_div_from_scalar_sinh_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_sqrt_offset_input(458, 35, 1.0);
            s.store_div_scaled_inputs(34, s.ad_value(458), 0.5, s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, (-0.25), 34, 1.0);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_add_scaled_inputs4(429, s.ad_value(423), 1.0, s.ad_value(422), (-1.0), s.ad_value(440), 1.0, A::ln(A::abs(A::mul(A::mul3(s.ad_value(442), s.ad_value(35), s.ad_value(37)), s.ad_value(37)))), -1.0);

        s.store_add_scaled_product_mixed_iaa(427, 457, 1.0, A::add(s.ad_value(456), s.ad_value(446)), A::add_scaled_inputs(s.ad_value(429), s.v[420], s.ad_value(456), 1.0), 1.0);

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_scaled_inputs(443, 456, ((-2.0) * s.v[419]), 457, 1.0);

        s.store_mul(444, 445, 443);

        s.store_add_scaled_product_value_ad(441, A::offset(A::mul_scaled_output(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37), 2.0), (-1.0)), 1.0, 447, 443, (-1.0));

        s.store_add_ad(428, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(457), 1.0, s.ad_value(456), (-s.v[419]), s.ad_value(36), (-s.v[419])), 1.0, s.ad_value(456), s.ad_value(444), 1.0), A::add_scaled_products(s.ad_value(441), s.ad_value(36), s.v[420], s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])), s.v[420]));

        s.store_div_scaled_inputs(425, s.ad_value(427), -1.0, s.ad_value(428), 1.0);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_mul_exp_rhs(34, 421, 448);

        s.store_add_scaled_product_indices(442, 34, (-1.0), 440, 440, s.v[451]);

        s.b[606] = (s.v[442] < 0.0);
        s.v[606] = if s.b[606] { 1.0 } else { 0.0 };

        if s.b[606] {
            s.store_sqrt_neg_input(439, 442);
            s.store_scale(36, 439, 0.5);
            s.store_div_ad_rhs(446, 439, A::tan(s.ad_value(36)));
            s.store_sin(40, 36);
            s.store_mul_neg_lhs(35, 40, 40);
        }

        if (!s.b[606]) {
            s.store_sqrt(439, 442);
            s.store_scale(36, 439, 0.5);
            s.store_sinh(40, 36);
            s.store_square(35, 40);
            s.store_div_ad_rhs(446, 439, A::tanh(s.ad_value(36)));
        }

        s.store_div_scaled_inputs2(437, s.ad_value(440), s.v[419], s.ad_value(446), (-1.0), A::sub_from_scalar(1.0, A::div(s.ad_value(442), A::mul(s.ad_value(35), s.ad_value(34)))), 1.0);

        s.store_scaled_mul(431, 440, 81, s.v[17]);

        s.store_scaled_mul(435, 437, 81, s.v[20]);

        s.store_sub(433, 435, 431);

        s.store_sub_ad_rhs(430, 423, A::div_scaled_inputs(s.ad_value(433), 1.0, s.ad_value(81), s.v[19]));

        s.store_mul_scaled_ad_lhs(210, A::add(s.ad_value(448), s.ad_value(430)), 81, 0.5);

        s.store_scale(109, 435, 1.0 / (s.v[17]));

        s.store_scale(111, 290, (1.60219e-19 * (p.p49 * 1.0 / (s.v[17]))));

        s.store_add_scaled_product_indices(36, 111, 1.0, 114, 431, 1.0 / (s.v[17]));

        s.store_scaled_add_ad_rhs(37, 36, A::sqrt(A::offset(A::square(s.ad_value(36)), 0.001)), 0.5);

        s.store_scale(127, 37, s.v[129]);

        s.store_add_scaled_product_indices(36, 111, 1.0, 143, 433, 1.0 / (s.v[19]));

        s.store_scaled_add_ad_rhs(37, 36, A::sqrt(A::offset(A::square(s.ad_value(36)), 0.001)), 0.5);

        s.store_scale(128, 37, s.v[144]);

        s.v[59] = (0.01 / s.v[17]);

        s.store_pow_ad(607, A::scaled_offset(A::abs_scaled_input(s.ad_value(109), 1.0 / (s.v[59])), 1.0, 0.5), s.ad_value(124));

        s.store_add_scaled_product(608, A::div(s.ad_value(125), s.ad_value(607)), 1.0, A::add_scaled_product(s.ad_value(122), 1.0, s.ad_value(23), s.ad_value(123), 1.0), A::pow(A::abs(s.ad_value(127)), A::add_scaled_product(s.ad_value(336), 1.0, s.ad_value(342), s.ad_value(23), 1.0)), 1.0);

        s.store_offset(112, 608, 1.0);

        s.store_scaled_add_ad(112, A::offset(s.ad_value(112), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(112), (-1.0), A::offset(s.ad_value(112), (-1.0))), ((0.25 * p.p154) * p.p154))), 0.5);

        s.store_scale(112, 112, 1.0 / (p.p11));

        s.store_div(141, 126, 112);

        s.store_pow_ad(609, A::scaled_offset(A::abs_scaled_input(s.ad_value(109), 1.0 / (s.v[59])), 1.0, 0.5), s.ad_value(348));

        s.store_add_scaled_product(610, A::div(s.ad_value(347), s.ad_value(609)), 1.0, A::add_scaled_product(s.ad_value(345), 1.0, s.ad_value(23), s.ad_value(346), 1.0), A::pow(A::abs(s.ad_value(128)), A::add_scaled_product(s.ad_value(349), 1.0, s.ad_value(350), s.ad_value(23), 1.0)), 1.0);

        s.store_offset(112, 610, 1.0);

        s.store_scaled_add_ad(112, A::offset(s.ad_value(112), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(112), (-1.0), A::offset(s.ad_value(112), (-1.0))), ((0.25 * p.p154) * p.p154))), 0.5);

        s.store_scale(112, 112, 1.0 / (p.p11));

        s.store_div(142, 344, 112);

        s.store_sub_scaled_inputs(34, 71, 1.0, 431, 1.0 / (s.v[17]));

        s.store_add_scaled_inputs3(35, s.ad_value(70), 1.0, s.ad_value(86), (-1.0), s.ad_value(433), (-1.0 / (s.v[19])));

        s.store_div_ad(139, A::exp(A::div(s.ad_value(34), s.ad_value(81))), A::add(A::exp(A::div(s.ad_value(34), s.ad_value(81))), A::exp(A::div(s.ad_value(35), s.ad_value(81)))));

        s.store_div_ad(140, A::exp(A::div(s.ad_value(35), s.ad_value(81))), A::add(A::exp(A::div(s.ad_value(34), s.ad_value(81))), A::exp(A::div(s.ad_value(35), s.ad_value(81)))));

        s.store_add_scaled_products_indices(121, 139, 141, 1.0, 140, 142, 1.0);

        s.b[611] = (p.p14 == 1.0);
        s.v[611] = if s.b[611] { 1.0 } else { 0.0 };

        if s.b[611] {
            s.store_scalar(152, 0.0);
        }

        s.b[612] = (p.p14 == 0.0);
        s.v[612] = if s.b[612] { 1.0 } else { 0.0 };

        if ((!s.b[611]) && s.b[612]) {
            s.store_offset_mul(38, 284, 109, 1.0);
            s.store_div_from_scalar(35, 1.0, 38);
            s.store_scaled_add_ad_rhs(34, 35, A::sqrt(A::offset(A::square(s.ad_value(35)), 0.01)), 0.5);
            s.store_mul_ad_affine_product_lhs(152, A::add_scaled_product(s.ad_value(134), 1.0, s.ad_value(281), s.ad_value(34), 1.0), s.ad_value(131), p.p2, 0.0, 150);
        }

        if ((!s.b[611]) && (!s.b[612])) {
            s.store_offset_mul(38, 284, 109, 1.0);
            s.store_div_from_scalar(35, 1.0, 38);
            s.store_scaled_add_ad_rhs(34, 35, A::sqrt(A::offset(A::square(s.ad_value(35)), 0.01)), 0.5);
            s.store_mul_ad_affine_product_lhs(152, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(132), 1.0, s.ad_value(133), 1.0, s.ad_value(134), 1.0), 1.0, s.ad_value(281), s.ad_value(34), 1.0), s.ad_value(131), p.p2, 0.0, 150);
        }

        s.store_div_scaled_inputs(169, s.ad_value(164), 2.0, s.ad_value(121), 1.0);

        s.store_mul(170, 169, 2);

        s.store_mul_ad_rhs(40, 404, A::add_scaled_value_products(s.ad_value(109), 1.0, s.ad_value(407), s.ad_value(28), 1.0, s.ad_value(55), s.ad_value(405), 2.0));

        s.b[613] = (s.v[152] == 0.0);
        s.v[613] = if s.b[613] { 1.0 } else { 0.0 };

        if s.b[613] {
            s.store_div_scaled_product_denominator_ad(162, 170, 40, 1.0, A::add(s.ad_value(170), s.ad_value(40)), 1.0);
        }

        if (!s.b[613]) {
            s.store_scaled_mul(177, 3, 164, s.v[17]);
            s.store_mul(34, 177, 152);
            s.store_scale(178, 34, 2.0);
            s.store_add_scaled_inputs_product_indices(179, 40, 1.0, 170, 1.0, 40, 34, 3.0);
            s.store_mul_ad_rhs(180, 40, A::add_scaled_product(s.ad_value(170), 1.0, s.ad_value(40), s.ad_value(34), 2.0));
            s.store_div_scaled_inputs2(162, s.ad_value(179), 1.0, A::sqrt(A::add_scaled_square_product(s.ad_value(179), 1.0, s.ad_value(178), s.ad_value(180), (-2.0))), (-1.0), s.ad_value(178), 1.0);
        }

        s.store_offset_ad(162, A::add_scaled_inputs(A::offset(s.ad_value(162), (-0.001)), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(162), (-0.001), A::offset(s.ad_value(162), (-0.001))), ((4.0 * 1e-5) * 1e-5))), 0.5), 0.001);

        s.store_pow_ad(41, A::div(s.ad_value(26), s.ad_value(162)), s.ad_value(168));

        s.store_pow_ad(42, A::offset(s.ad_value(41), 1.0), s.ad_value(163));

        s.store_div(113, 26, 42);

        s.b[614] = (s.v[113] > s.v[26]);
        s.v[614] = if s.b[614] { 1.0 } else { 0.0 };

        if s.b[614] {
            s.copy_ad(113, 26);
        }

        s.store_div_scaled_inputs2(422, s.ad_value(71), 1.0, s.ad_value(113), (-1.0), s.ad_value(81), 1.0);

        s.store_div_ad_lhs(423, A::add_scaled_inputs3_offset(s.ad_value(70), 1.0, s.ad_value(86), (-1.0), s.ad_value(113), -1.0, p.p10), 81);

        s.store_sub_ad_lhs(453, A::ln(A::offset(A::mul_scaled_lhs(A::sub(s.ad_value(422), s.ad_value(450)), s.v[451], A::sub(s.ad_value(422), s.ad_value(450))), 39.47841)), 449);

        s.store_sub_ad_lhs(424, A::ln(A::offset(A::mul_scaled_lhs(A::sub(s.ad_value(422), s.ad_value(450)), s.v[451], A::sub(s.ad_value(422), s.ad_value(450))), 39.47841)), 449);

        s.store_sub_scaled_inputs(37, 450, ((1.0 + s.v[419]) * 1.0 / (s.v[419])), 430, 1.0 / (s.v[419]));

        s.store_sub_ad_lhs(38, A::ln(A::offset(A::mul_scaled_lhs(A::sub(s.ad_value(37), s.ad_value(450)), s.v[451], A::sub(s.ad_value(37), s.ad_value(450))), 39.47841)), 449);

        s.store_sub(39, 38, 450);

        s.store_sub(424, 424, 39);

        s.store_add_scaled_inputs(452, 424, 1.0 / ((1.0 + s.v[420])), 423, (s.v[420] * 1.0 / ((1.0 + s.v[420]))));

        s.store_add_scaled_inputs3(426, s.ad_value(423), 1.0, s.ad_value(422), s.v[454], s.ad_value(423), (-s.v[454]));

        s.store_min(430, 426, 453);

        s.store_min(430, 430, 450);

        s.store_add_scaled_inputs(448, 430, 1.0 / ((1.0 + s.v[419])), 422, (s.v[419] * 1.0 / ((1.0 + s.v[419]))));

        s.store_sub(34, 448, 430);

        s.store_div_scaled_product_offset_rhs(37, A::limited_exp(s.ad_value(430)), A::limited_exp(s.ad_value(34)), (-1.0), 1.0, s.ad_value(34), 1.0);

        s.store_sub(429, 423, 452);

        s.store_add_scaled_products_right_right_ad(442, 429, 429, (s.v[420] * s.v[420]), 421, A::exp(s.ad_value(452)), (-1.0));

        s.b[615] = (s.v[442] < 0.0);
        s.v[615] = if s.b[615] { 1.0 } else { 0.0 };

        if s.b[615] {
            s.store_scaled_sub(429, 423, 430, s.v[420]);
            s.store_scalar(440, (40.0 * s.v[419]));
            s.store_add(455, 440, 429);
            s.store_mul(37, 440, 429);
            s.store_offset_scaled(38, 455, 0.06534, 1.0);
            s.store_offset_ad(39, A::add_scaled_inputs(s.ad_value(455), 8.57973, s.ad_value(37), 1.0), 39.47841);
            s.store_add_scaled_inputs(40, 455, 78.95683, 37, 39.47841);
            s.store_div_scaled_inputs2(442, A::sqrt(A::add_scaled_square_product(s.ad_value(39), 1.0, s.ad_value(38), s.ad_value(40), (-4.0))), 1.0, s.ad_value(39), (-1.0), s.ad_value(38), 2.0);
            s.store_sub_scaled_inputs(37, 450, ((1.0 + s.v[419]) * 1.0 / (s.v[419])), 430, 1.0 / (s.v[419]));
            s.store_offset_scaled_sub(34, 422, 37, (40.0 * 0.2), ((2.0) * ((40.0 * 0.2))));
            s.store_mul_sub_from_scalar_ad_rhs(442, 442, 1.0, A::exp_scaled_input(A::offset(A::sub(s.ad_value(422), s.ad_value(37)), 2.0), (-1.0 / ((2.0 / 0.69)))));
            s.store_min_with_scalar(442, 442, 50.0);
        }

        s.store_max(422, 422, 450);

        s.store_sub_ad_lhs(424, A::ln(A::offset(A::mul_scaled_lhs(A::sub(s.ad_value(422), s.ad_value(450)), s.v[451], A::sub(s.ad_value(422), s.ad_value(450))), 39.47841)), 449);

        s.store_sub_scaled_inputs(37, 450, ((1.0 + s.v[419]) * 1.0 / (s.v[419])), 430, 1.0 / (s.v[419]));

        s.store_sub_ad_lhs(38, A::ln(A::offset(A::mul_scaled_lhs(A::sub(s.ad_value(37), s.ad_value(450)), s.v[451], A::sub(s.ad_value(37), s.ad_value(450))), 39.47841)), 449);

        s.store_sub(39, 38, 450);

        s.store_sub(424, 424, 39);

        s.store_sub(440, 422, 424);

        s.store_mul_scaled_ad_rhs(34, 421, -1.0, A::exp(s.ad_value(424)));

        s.store_scale(35, 440, s.v[451]);

        s.store_div_scaled_inputs2(425, A::add_scaled_product(s.ad_value(34), 1.0, s.ad_value(35), s.ad_value(440), 1.0), -1.0, s.ad_value(442), 1.0, A::add_scaled_inputs(s.ad_value(35), (-2.0), s.ad_value(34), 1.0), 1.0);

        s.store_add(424, 424, 425);

        s.store_sub(440, 422, 424);

        s.store_scale(36, 440, s.v[451]);

        s.store_div_from_scalar_ad(34, 1.0, A::add_scaled_product(s.ad_value(442), (-1.0), s.ad_value(36), s.ad_value(440), 1.0));

        s.store_add_scaled_inputs3(465, A::ln(A::abs(A::add_scaled_product(s.ad_value(442), (-1.0), s.ad_value(36), s.ad_value(440), 1.0))), 1.0, s.ad_value(449), (-1.0), s.ad_value(424), -1.0);

        s.store_div_from_scalar_offset_ad(466, 1.0, A::mul_scaled_lhs(s.ad_value(36), (-2.0), s.ad_value(34)), (-1.0));

        s.store_add_scaled_product_left_ad(467, 34, (2.0 * s.v[451]), A::mul3_scaled_output(s.ad_value(36), s.ad_value(36), s.ad_value(34), (-4.0)), 34, 1.0);

        s.store_mul(35, 465, 466);

        s.store_add_scaled_product_left_ad(425, 35, -1.0, A::mul3_scaled_output(s.ad_value(35), s.ad_value(35), s.ad_value(467), 0.5), 466, (-1.0));

        s.store_max_with_scalar(425, 425, (-10.0));

        s.store_min_with_scalar(425, 425, 10.0);

        s.store_add(424, 424, 425);

        s.store_sub(440, 422, 424);

        s.store_scale(36, 440, s.v[451]);

        s.store_div_from_scalar_ad(34, 1.0, A::add_scaled_product(s.ad_value(442), (-1.0), s.ad_value(36), s.ad_value(440), 1.0));

        s.store_add_scaled_inputs3(465, A::ln(A::abs(A::add_scaled_product(s.ad_value(442), (-1.0), s.ad_value(36), s.ad_value(440), 1.0))), 1.0, s.ad_value(449), (-1.0), s.ad_value(424), -1.0);

        s.store_div_from_scalar_offset_ad(466, 1.0, A::mul_scaled_lhs(s.ad_value(36), (-2.0), s.ad_value(34)), (-1.0));

        s.store_add_scaled_product_left_ad(467, 34, (2.0 * s.v[451]), A::mul3_scaled_output(s.ad_value(36), s.ad_value(36), s.ad_value(34), (-4.0)), 34, 1.0);

        s.store_mul(35, 465, 466);

        s.store_add_scaled_product_left_ad(425, 35, -1.0, A::mul3_scaled_output(s.ad_value(35), s.ad_value(35), s.ad_value(467), 0.5), 466, (-1.0));

        s.store_max_with_scalar(425, 425, (-10.0));

        s.store_min_with_scalar(425, 425, 10.0);

        s.store_add(424, 424, 425);

        s.store_max_ad(424, s.ad_value(424), A::offset(s.ad_value(450), (-4.0)));

        s.store_div_scaled_inputs2(422, s.ad_value(71), 1.0, s.ad_value(113), (-1.0), s.ad_value(81), 1.0);

        s.store_sub_ad_rhs(448, 448, A::ln_one_plus_exp(A::sub_scaled_inputs(s.ad_value(448), 1.0, s.ad_value(424), 1.05)));

    }

    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_min(448, 448, 424);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_scaled_ad_rhs(457, 421, -1.0, A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.b[616] = (s.v[442] < 0.0);
        s.v[616] = if s.b[616] { 1.0 } else { 0.0 };

        if s.b[616] {
            s.store_sqrt_neg_input(439, 442);
            s.store_div_from_scalar_sin_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_mul_cos_ad_lhs(458, A::scale(s.ad_value(439), 0.5), 459);
            s.store_div_scaled_inputs(34, s.ad_value(458), (-0.5), s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, 0.25, 34, 1.0);
        }

        if (!s.b[616]) {
            s.store_sqrt(439, 442);
            s.store_div_from_scalar_sinh_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_sqrt_offset_input(458, 35, 1.0);
            s.store_div_scaled_inputs(34, s.ad_value(458), 0.5, s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, (-0.25), 34, 1.0);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_add_scaled_inputs4(429, s.ad_value(423), 1.0, s.ad_value(422), (-1.0), s.ad_value(440), 1.0, A::ln(A::abs(A::mul(A::mul3(s.ad_value(442), s.ad_value(35), s.ad_value(37)), s.ad_value(37)))), -1.0);

        s.store_add_scaled_product_mixed_iaa(427, 457, 1.0, A::add(s.ad_value(456), s.ad_value(446)), A::add_scaled_inputs(s.ad_value(429), s.v[420], s.ad_value(456), 1.0), 1.0);

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_scaled_inputs(443, 456, ((-2.0) * s.v[419]), 457, 1.0);

        s.store_mul(444, 445, 443);

        s.store_add_scaled_product_value_ad(441, A::offset(A::mul_scaled_output(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37), 2.0), (-1.0)), 1.0, 447, 443, (-1.0));

        s.store_add_ad(428, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(457), 1.0, s.ad_value(456), (-s.v[419]), s.ad_value(36), (-s.v[419])), 1.0, s.ad_value(456), s.ad_value(444), 1.0), A::add_scaled_products(s.ad_value(441), s.ad_value(36), s.v[420], s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])), s.v[420]));

        s.store_div_scaled_inputs(425, s.ad_value(427), -1.0, s.ad_value(428), 1.0);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_scaled_ad_rhs(457, 421, -1.0, A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.b[617] = (s.v[442] < 0.0);
        s.v[617] = if s.b[617] { 1.0 } else { 0.0 };

        if s.b[617] {
            s.store_sqrt_neg_input(439, 442);
            s.store_div_from_scalar_sin_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_mul_cos_ad_lhs(458, A::scale(s.ad_value(439), 0.5), 459);
            s.store_div_scaled_inputs(34, s.ad_value(458), (-0.5), s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, 0.25, 34, 1.0);
        }

        if (!s.b[617]) {
            s.store_sqrt(439, 442);
            s.store_div_from_scalar_sinh_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_sqrt_offset_input(458, 35, 1.0);
            s.store_div_scaled_inputs(34, s.ad_value(458), 0.5, s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, (-0.25), 34, 1.0);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_add_scaled_inputs4(429, s.ad_value(423), 1.0, s.ad_value(422), (-1.0), s.ad_value(440), 1.0, A::ln(A::abs(A::mul(A::mul3(s.ad_value(442), s.ad_value(35), s.ad_value(37)), s.ad_value(37)))), -1.0);

        s.store_add_scaled_product_mixed_iaa(427, 457, 1.0, A::add(s.ad_value(456), s.ad_value(446)), A::add_scaled_inputs(s.ad_value(429), s.v[420], s.ad_value(456), 1.0), 1.0);

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_scaled_inputs(443, 456, ((-2.0) * s.v[419]), 457, 1.0);

        s.store_mul(444, 445, 443);

        s.store_add_scaled_product_value_ad(441, A::offset(A::mul_scaled_output(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37), 2.0), (-1.0)), 1.0, 447, 443, (-1.0));

        s.store_add_ad(428, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(457), 1.0, s.ad_value(456), (-s.v[419]), s.ad_value(36), (-s.v[419])), 1.0, s.ad_value(456), s.ad_value(444), 1.0), A::add_scaled_products(s.ad_value(441), s.ad_value(36), s.v[420], s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])), s.v[420]));

        s.store_div_scaled_inputs(425, s.ad_value(427), -1.0, s.ad_value(428), 1.0);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_scaled_ad_rhs(457, 421, -1.0, A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.b[618] = (s.v[442] < 0.0);
        s.v[618] = if s.b[618] { 1.0 } else { 0.0 };

        if s.b[618] {
            s.store_sqrt_neg_input(439, 442);
            s.store_div_from_scalar_sin_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_mul_cos_ad_lhs(458, A::scale(s.ad_value(439), 0.5), 459);
            s.store_div_scaled_inputs(34, s.ad_value(458), (-0.5), s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, 0.25, 34, 1.0);
        }

        if (!s.b[618]) {
            s.store_sqrt(439, 442);
            s.store_div_from_scalar_sinh_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_sqrt_offset_input(458, 35, 1.0);
            s.store_div_scaled_inputs(34, s.ad_value(458), 0.5, s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, (-0.25), 34, 1.0);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_add_scaled_inputs4(429, s.ad_value(423), 1.0, s.ad_value(422), (-1.0), s.ad_value(440), 1.0, A::ln(A::abs(A::mul(A::mul3(s.ad_value(442), s.ad_value(35), s.ad_value(37)), s.ad_value(37)))), -1.0);

        s.store_add_scaled_product_mixed_iaa(427, 457, 1.0, A::add(s.ad_value(456), s.ad_value(446)), A::add_scaled_inputs(s.ad_value(429), s.v[420], s.ad_value(456), 1.0), 1.0);

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_scaled_inputs(443, 456, ((-2.0) * s.v[419]), 457, 1.0);

        s.store_mul(444, 445, 443);

        s.store_add_scaled_product_value_ad(441, A::offset(A::mul_scaled_output(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37), 2.0), (-1.0)), 1.0, 447, 443, (-1.0));

        s.store_add_ad(428, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(457), 1.0, s.ad_value(456), (-s.v[419]), s.ad_value(36), (-s.v[419])), 1.0, s.ad_value(456), s.ad_value(444), 1.0), A::add_scaled_products(s.ad_value(441), s.ad_value(36), s.v[420], s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])), s.v[420]));

        s.store_div_scaled_inputs(425, s.ad_value(427), -1.0, s.ad_value(428), 1.0);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_scaled_ad_rhs(457, 421, -1.0, A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.b[619] = (s.v[442] < 0.0);
        s.v[619] = if s.b[619] { 1.0 } else { 0.0 };

        if s.b[619] {
            s.store_sqrt_neg_input(439, 442);
            s.store_div_from_scalar_sin_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_mul_cos_ad_lhs(458, A::scale(s.ad_value(439), 0.5), 459);
            s.store_div_scaled_inputs(34, s.ad_value(458), (-0.5), s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, 0.25, 34, 1.0);
        }

        if (!s.b[619]) {
            s.store_sqrt(439, 442);
            s.store_div_from_scalar_sinh_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_sqrt_offset_input(458, 35, 1.0);
            s.store_div_scaled_inputs(34, s.ad_value(458), 0.5, s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, (-0.25), 34, 1.0);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_add_scaled_inputs4(429, s.ad_value(423), 1.0, s.ad_value(422), (-1.0), s.ad_value(440), 1.0, A::ln(A::abs(A::mul(A::mul3(s.ad_value(442), s.ad_value(35), s.ad_value(37)), s.ad_value(37)))), -1.0);

        s.store_add_scaled_product_mixed_iaa(427, 457, 1.0, A::add(s.ad_value(456), s.ad_value(446)), A::add_scaled_inputs(s.ad_value(429), s.v[420], s.ad_value(456), 1.0), 1.0);

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_scaled_inputs(443, 456, ((-2.0) * s.v[419]), 457, 1.0);

        s.store_mul(444, 445, 443);

        s.store_add_scaled_product_value_ad(441, A::offset(A::mul_scaled_output(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37), 2.0), (-1.0)), 1.0, 447, 443, (-1.0));

        s.store_add_ad(428, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(457), 1.0, s.ad_value(456), (-s.v[419]), s.ad_value(36), (-s.v[419])), 1.0, s.ad_value(456), s.ad_value(444), 1.0), A::add_scaled_products(s.ad_value(441), s.ad_value(36), s.v[420], s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])), s.v[420]));

        s.store_div_scaled_inputs(425, s.ad_value(427), -1.0, s.ad_value(428), 1.0);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_scaled_ad_rhs(457, 421, -1.0, A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.b[620] = (s.v[442] < 0.0);
        s.v[620] = if s.b[620] { 1.0 } else { 0.0 };

        if s.b[620] {
            s.store_sqrt_neg_input(439, 442);
            s.store_div_from_scalar_sin_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_mul_cos_ad_lhs(458, A::scale(s.ad_value(439), 0.5), 459);
            s.store_div_scaled_inputs(34, s.ad_value(458), (-0.5), s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, 0.25, 34, 1.0);
        }

        if (!s.b[620]) {
            s.store_sqrt(439, 442);
            s.store_div_from_scalar_sinh_ad(459, 1.0, A::scale(s.ad_value(439), 0.5));
            s.store_square(35, 459);
            s.store_sqrt_offset_input(458, 35, 1.0);
            s.store_div_scaled_inputs(34, s.ad_value(458), 0.5, s.ad_value(439), 1.0);
            s.store_add_scaled_inputs(445, 35, (-0.25), 34, 1.0);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_add_scaled_inputs4(429, s.ad_value(423), 1.0, s.ad_value(422), (-1.0), s.ad_value(440), 1.0, A::ln(A::abs(A::mul(A::mul3(s.ad_value(442), s.ad_value(35), s.ad_value(37)), s.ad_value(37)))), -1.0);

        s.store_add_scaled_product_mixed_iaa(427, 457, 1.0, A::add(s.ad_value(456), s.ad_value(446)), A::add_scaled_inputs(s.ad_value(429), s.v[420], s.ad_value(456), 1.0), 1.0);

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_scaled_inputs(443, 456, ((-2.0) * s.v[419]), 457, 1.0);

        s.store_mul(444, 445, 443);

        s.store_add_scaled_product_value_ad(441, A::offset(A::mul_scaled_output(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37), 2.0), (-1.0)), 1.0, 447, 443, (-1.0));

        s.store_add_ad(428, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(457), 1.0, s.ad_value(456), (-s.v[419]), s.ad_value(36), (-s.v[419])), 1.0, s.ad_value(456), s.ad_value(444), 1.0), A::add_scaled_products(s.ad_value(441), s.ad_value(36), s.v[420], s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])), s.v[420]));

        s.store_div_scaled_inputs(425, s.ad_value(427), -1.0, s.ad_value(428), 1.0);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_mul_exp_rhs(34, 421, 448);

        s.store_add_scaled_product_indices(442, 34, (-1.0), 440, 440, s.v[451]);

        s.b[621] = (s.v[442] < 0.0);
        s.v[621] = if s.b[621] { 1.0 } else { 0.0 };

        if s.b[621] {
            s.store_sqrt_neg_input(439, 442);
            s.store_scale(36, 439, 0.5);
            s.store_div_ad_rhs(446, 439, A::tan(s.ad_value(36)));
            s.store_sin(40, 36);
            s.store_mul_neg_lhs(35, 40, 40);
        }

        if (!s.b[621]) {
            s.store_sqrt(439, 442);
            s.store_scale(36, 439, 0.5);
            s.store_sinh(40, 36);
            s.store_square(35, 40);
            s.store_div_ad_rhs(446, 439, A::tanh(s.ad_value(36)));
        }

        s.store_div_scaled_inputs2(438, s.ad_value(440), s.v[419], s.ad_value(446), (-1.0), A::sub_from_scalar(1.0, A::div(s.ad_value(442), A::mul(s.ad_value(35), s.ad_value(34)))), 1.0);

        s.store_scaled_mul(432, 440, 81, s.v[17]);

        s.store_scaled_mul(436, 438, 81, s.v[20]);

        s.store_sub(434, 436, 432);

        s.store_sub_ad_rhs(430, 423, A::div_scaled_inputs(s.ad_value(434), 1.0, s.ad_value(81), s.v[19]));

        s.store_scale(110, 436, 1.0 / (s.v[17]));

        s.store_scaled_add(46, 109, 110, 0.5);

        s.store_sub(49, 109, 110);

        s.store_scale(48, 290, (1.60219e-19 * (p.p49 * 1.0 / (s.v[17]))));

        s.store_scale_ad(34, A::powf(s.ad_value(113), 2.0), 1600.0);

        s.b[622] = (p.p162 != 0.0);
        s.v[622] = if s.b[622] { 1.0 } else { 0.0 };

        if s.b[622] {
            s.store_add_scaled_inputs3(47, s.ad_value(431), 1.0 / ((2.0 * s.v[17])), s.ad_value(432), 1.0 / ((2.0 * s.v[17])), A::mul_sub_from_scalar_lhs_scaled_output(1.0, A::limited_exp_scaled_input(s.ad_value(34), -1.0), A::sub(s.ad_value(431), s.ad_value(432)), ((p.p162 * 0.5) * 1.0 / (s.v[17]))), 1.0);
        }

        if (!s.b[622]) {
            s.store_scaled_add(47, 431, 432, 1.0 / ((2.0 * s.v[17])));
        }

        s.b[623] = (p.p189 != 0.0);
        s.v[623] = if s.b[623] { 1.0 } else { 0.0 };

        if s.b[623] {
            s.store_add_scaled_inputs3(145, s.ad_value(433), 1.0 / ((2.0 * s.v[19])), s.ad_value(434), 1.0 / ((2.0 * s.v[19])), A::mul_sub_from_scalar_lhs_scaled_output(1.0, A::limited_exp_scaled_input(s.ad_value(34), -1.0), A::sub(s.ad_value(433), s.ad_value(434)), ((p.p189 * 0.5) * 1.0 / (s.v[19]))), 1.0);
        }

        if (!s.b[623]) {
            s.store_scaled_add(145, 433, 434, 1.0 / ((2.0 * s.v[19])));
        }

        s.store_add_scaled_product_indices(36, 48, 1.0, 114, 47, 1.0);

        s.store_scaled_add_ad_rhs(37, 36, A::sqrt(A::offset(A::square(s.ad_value(36)), 0.001)), 0.5);

    }

    pub(super) fn stamp_reactive_block_5(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.store_scale(116, 37, s.v[129]);

        s.store_add_scaled_product_indices(36, 48, 1.0, 143, 145, 1.0);

        s.store_scaled_add_ad_rhs(37, 36, A::sqrt(A::offset(A::square(s.ad_value(36)), 0.001)), 0.5);

        s.store_scale(117, 37, s.v[144]);

        s.store_pow_ad(624, A::scaled_offset(A::abs_scaled_input(s.ad_value(46), 1.0 / (s.v[59])), 1.0, 0.5), s.ad_value(124));

        s.store_add_scaled_product(625, A::div_scaled_add_product(s.ad_value(125), 1.0, s.ad_value(25), s.ad_value(137), 1.0, s.ad_value(624), 1.0), 1.0, A::add_scaled_product(s.ad_value(122), 1.0, s.ad_value(25), s.ad_value(123), 1.0), A::pow(A::abs(s.ad_value(116)), A::add_scaled_product(s.ad_value(336), 1.0, s.ad_value(342), s.ad_value(25), 1.0)), 1.0);

        s.store_offset(119, 625, 1.0);

        s.store_scaled_add_ad(119, A::offset(s.ad_value(119), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(119), (-1.0), A::offset(s.ad_value(119), (-1.0))), ((0.25 * p.p154) * p.p154))), 0.5);

        s.store_scale(119, 119, 1.0 / (p.p11));

        s.store_div(141, 126, 119);

        s.store_pow_ad(626, A::scaled_offset(A::abs_scaled_input(s.ad_value(46), 1.0 / (s.v[59])), 1.0, 0.5), s.ad_value(348));

        s.store_add_scaled_product(627, A::div_scaled_add_product(s.ad_value(347), 1.0, s.ad_value(25), s.ad_value(138), 1.0, s.ad_value(626), 1.0), 1.0, A::add_scaled_product(s.ad_value(345), 1.0, s.ad_value(25), s.ad_value(346), 1.0), A::pow(A::abs(s.ad_value(117)), A::add_scaled_product(s.ad_value(349), 1.0, s.ad_value(350), s.ad_value(25), 1.0)), 1.0);

        s.store_offset(119, 627, 1.0);

        s.store_scaled_add_ad(119, A::offset(s.ad_value(119), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(119), (-1.0), A::offset(s.ad_value(119), (-1.0))), ((0.25 * p.p154) * p.p154))), 0.5);

        s.store_scale(119, 119, 1.0 / (p.p11));

        s.store_div(142, 344, 119);

        s.store_add_scaled_inputs3(34, s.ad_value(71), 1.0, s.ad_value(431), (-1.0 / ((2.0 * s.v[17]))), s.ad_value(432), (-1.0 / ((2.0 * s.v[17]))));

        s.store_add_scaled_inputs4(35, s.ad_value(70), 1.0, s.ad_value(86), (-1.0), s.ad_value(433), (-1.0 / ((2.0 * s.v[19]))), s.ad_value(434), (-1.0 / ((2.0 * s.v[19]))));

        s.store_div_ad(139, A::exp(A::div(s.ad_value(34), s.ad_value(81))), A::add(A::exp(A::div(s.ad_value(34), s.ad_value(81))), A::exp(A::div(s.ad_value(35), s.ad_value(81)))));

        s.store_div_ad(140, A::exp(A::div(s.ad_value(35), s.ad_value(81))), A::add(A::exp(A::div(s.ad_value(34), s.ad_value(81))), A::exp(A::div(s.ad_value(35), s.ad_value(81)))));

        s.store_add_scaled_products_indices(121, 139, 141, 1.0, 140, 142, 1.0);

        s.store_div_scaled_product_indices(56, 121, 3, s.v[17], 2, 1.0);

        s.store_add_scaled_product_indices(118, 48, s.v[129], 115, 46, s.v[129]);

        s.store_mul_pow_ad_rhs(37, 122, A::abs(s.ad_value(118)), s.ad_value(336));

        s.store_offset(120, 37, 1.0);

        s.store_scaled_add_ad(120, A::offset(s.ad_value(120), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(120), (-1.0), A::offset(s.ad_value(120), (-1.0))), ((0.25 * p.p154) * p.p154))), 0.5);

        s.store_scale(120, 120, 1.0 / (p.p11));

        s.store_div_scaled_inputs(173, s.ad_value(166), 2.0, s.ad_value(121), 1.0);

        s.store_mul(174, 173, 2);

        s.store_offset_mul(34, 165, 25, 0.8);

        s.store_offset_ad(181, A::add_scaled_inputs(s.ad_value(34), 0.5, A::sqrt(A::offset(A::square(s.ad_value(34)), 0.01)), 0.5), 0.2);

        s.store_mul_div_lhs(34, 49, 174, 181);

        s.store_scaled_offset_ad(161, A::sqrt(A::offset(A::square(s.ad_value(34)), p.p109)), 1.0, 1.0 / ((1.0 + ((p.p109) as f64).sqrt())));

        s.store_add_scaled_product_left_ad(161, 161, 1.0, A::mul3_scaled_output(A::add_scaled_value_products(s.ad_value(182), 1.0, s.ad_value(328), s.ad_value(28), (-1.0), s.ad_value(329), s.ad_value(25), (-1.0)), s.ad_value(46), s.ad_value(49), 0.5), 49, 1.0);

        s.store_scaled_add_ad(161, A::offset(s.ad_value(161), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(161), (-1.0), A::offset(s.ad_value(161), (-1.0))), ((0.25 * p.p134) * p.p134))), 0.5);

        s.store_div_scaled_product_indices(171, 167, 120, 2.0, 126, 1.0);

        s.store_mul(172, 171, 1);

        s.b[628] = (s.v[365] > 0.0);
        s.v[628] = if s.b[628] { 1.0 } else { 0.0 };

        if s.b[628] {
            s.store_offset_ad(154, A::div_scaled_product(s.ad_value(365), s.ad_value(46), 1.0, s.ad_value(170), 1.0), 1.0);
        }

        if (!s.b[628]) {
            s.store_div_from_scalar_sub_from_scalar_ad(154, 1.0, 1.0, A::div_scaled_product(s.ad_value(365), s.ad_value(46), 1.0, s.ad_value(170), 1.0));
        }

        s.store_sub(155, 26, 113);

        s.store_add_scaled_inputs(157, 46, 1.0, 55, 2.0);

        s.b[629] = (s.v[153] > 0.0);
        s.v[629] = if s.b[629] { 1.0 } else { 0.0 };

        if s.b[629] {
            s.copy_ad(35, 157);
            s.store_div_ad_rhs(37, 35, A::add(s.ad_value(162), s.ad_value(35)));
            s.store_mul_ad_product_lhs(156, A::div(s.ad_value(35), s.ad_value(153)), s.ad_value(37), 154);
            s.store_offset_div(158, 155, 156, 1.0);
        }

        if (!s.b[629]) {
            s.store_scalar(158, 1.0);
        }

        s.b[630] = (s.v[360] > 0.0);
        s.v[630] = if s.b[630] { 1.0 } else { 0.0 };

        s.b[631] = (p.p213 < 0.0);
        s.v[631] = if s.b[631] { 1.0 } else { 0.0 };

        if (s.b[630] && s.b[631]) {
            s.store_div_from_scalar_ad(35, 1.0, A::sub_scaled_inputs(A::div_from_scalar(1.0, s.ad_value(360)), 1.0, s.ad_value(46), p.p213));
        }

        if (s.b[630] && (!s.b[631])) {
            s.store_mul_ad_rhs(35, 360, A::scale_offset(s.ad_value(46), p.p213, 1.0));
        }

        if s.b[630] {
            s.store_offset_mul_ad(159, s.ad_value(35), A::ln(A::max_with_scalar(A::offset(A::div_scaled_value_by_product(s.ad_value(155), 1.0, s.ad_value(35), A::add(s.ad_value(162), s.ad_value(170)), 1.0), 1.0), 1e-38)), 1.0);
        }

        if (!s.b[630]) {
            s.store_scalar(159, 1.0);
        }

        s.store_mul(158, 158, 159);

        s.b[632] = (s.v[361] > 0.0);
        s.v[632] = if s.b[632] { 1.0 } else { 0.0 };

        if s.b[632] {
            s.store_offset_mul_ad(160, s.ad_value(361), A::ln(A::max_with_scalar(A::offset(A::div_scaled_inputs2_by_product(s.ad_value(26), 1.0, s.ad_value(113), (-1.0), s.ad_value(361), A::add(s.ad_value(162), s.ad_value(172)), 1.0), 1.0), 1e-38)), 1.0);
        }

        if (!s.b[632]) {
            s.store_scalar(160, 1.0);
        }

        s.b[633] = (s.v[175] != 0.0);
        s.v[633] = if s.b[633] { 1.0 } else { 0.0 };

        if s.b[633] {
            s.store_div_ad_rhs(35, 175, A::add_scaled_product(s.ad_value(81), 2.0, A::max_from_scalar(0.0, A::add(s.ad_value(176), A::mul3(s.ad_value(108), s.ad_value(49), s.ad_value(49)))), s.ad_value(46), 1.0));
            s.store_limited_exp_neg_input(94, 35);
        }

        if (!s.b[633]) {
            s.store_scalar(94, 1.0);
        }

        s.store_sub(34, 437, 438);

        s.store_sub_ad(35, A::square(s.ad_value(437)), A::square(s.ad_value(438)));

        s.store_add_ad(215, A::mul3_scaled_output(s.ad_value(81), s.ad_value(55), s.ad_value(34), (s.v[20] * 2.0)), A::mul3_scaled_output(s.ad_value(81), s.ad_value(81), s.ad_value(35), ((s.v[20] * s.v[20]) * (0.5 * 1.0 / (s.v[17])))));

        s.store_add_scaled_inputs3(216, s.ad_value(109), 0.5, s.ad_value(110), 0.5, s.ad_value(55), 1.0);

        s.b[640] = (p.p14 == 1.0);
        s.v[640] = if s.b[640] { 1.0 } else { 0.0 };

        if s.b[640] {
            s.store_scalar(151, 0.0);
            s.store_scalar(130, 1.0);
            s.store_sub(638, 29, 200);
            s.store_sqrt_square_offset(639, 638, 0.0001);
            s.store_scaled_add(636, 638, 639, 0.5);
            s.store_offset_mul(635, 284, 636, 1.0);
            s.store_div_from_scalar(634, 1.0, 635);
            s.store_add_scaled_product_indices(634, 634, 1.0, 32, 285, (-0.5));
            s.store_scaled_add_ad_rhs(34, 634, A::sqrt(A::offset(A::square(s.ad_value(634)), 0.01)), 0.5);
            s.store_sub(638, 31, 200);
            s.store_sqrt_square_offset(639, 638, 0.0001);
            s.store_scaled_add(637, 638, 639, 0.5);
            s.store_offset_mul(635, 284, 637, 1.0);
            s.store_div_from_scalar(634, 1.0, 635);
            s.store_add_scaled_product_indices(634, 634, 1.0, 33, 285, (-0.5));
            s.store_scaled_add_ad_rhs(34, 634, A::sqrt(A::offset(A::square(s.ad_value(634)), 0.01)), 0.5);
        }

        if (!s.b[640]) {
            s.store_offset_mul(635, 284, 46, 1.0);
            s.store_div_from_scalar(634, 1.0, 635);
            s.store_add_scaled_product_left_ad(634, 634, 1.0, A::add(s.ad_value(24), s.ad_value(23)), 285, (-0.5));
            s.store_scaled_add_ad_rhs(34, 634, A::sqrt(A::offset(A::square(s.ad_value(634)), 0.01)), 0.5);
            s.store_mul_ad_product_lhs(151, s.ad_value(150), A::add_scaled_product(s.ad_value(134), 1.0, s.ad_value(281), s.ad_value(34), 1.0), 131);
            s.store_offset_mul_ad(130, A::div_scaled_product(s.ad_value(56), s.ad_value(216), p.p2, s.ad_value(161), 1.0), s.ad_value(151), 1.0);
        }

        s.b[641] = (p.p14 == 2.0);
        s.v[641] = if s.b[641] { 1.0 } else { 0.0 };

        if ((!s.b[640]) && s.b[641]) {
            s.store_offset_mul(635, 284, 46, 1.0);
            s.store_div_from_scalar(634, 1.0, 635);
            s.store_add_scaled_product_left_ad(634, 634, 1.0, A::add(s.ad_value(24), s.ad_value(23)), 285, (-0.5));
            s.store_scaled_add_ad_rhs(34, 634, A::sqrt(A::offset(A::square(s.ad_value(634)), 0.01)), 0.5);
            s.store_mul_ad_product_lhs(151, s.ad_value(150), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(132), 1.0, s.ad_value(133), 1.0, s.ad_value(134), 1.0), 1.0, s.ad_value(281), s.ad_value(34), 1.0), 131);
            s.store_offset_mul_ad(130, A::div_scaled_product(s.ad_value(56), s.ad_value(216), p.p2, s.ad_value(161), 1.0), s.ad_value(151), 1.0);
        }

        s.store_div_scaled_product_by_product(214, A::mul3_scaled_output(s.ad_value(56), s.ad_value(215), s.ad_value(158), 1.0 / (s.v[17])), s.ad_value(94), 1.0, s.ad_value(161), s.ad_value(130), 1.0);

        s.store_scale(214, 214, p.p2);

        s.store_scaled_add(219, 432, 431, 0.5);

        s.store_add_scaled_inputs(218, 435, (1.0 / 6.0), 436, (2.0 * (1.0 / 6.0)));

        s.store_add_scaled_inputs(217, 435, (2.0 * (1.0 / 6.0)), 436, (1.0 / 6.0));

        s.store_scaled_add(220, 434, 433, 0.5);

        s.b[642] = (s.v[62] > 0.0);
        s.v[642] = if s.b[642] { 1.0 } else { 0.0 };

        if s.b[642] {
            s.store_div_scaled_add_product(38, s.ad_value(46), 1.0, s.ad_value(66), s.ad_value(48), 1.0, s.ad_value(67), 1.0);
            s.store_offset_pow_ad(39, s.ad_value(38), s.ad_value(68), 1.0);
            s.store_scalar(63, p.p49);
            s.store_div(64, 63, 39);
            s.store_div_from_scalar_ad(65, (3.9 * 8.85418e-12), A::add_scaled_product(s.ad_value(221), (3.9 * 1.0 / (p.p60)), s.ad_value(64), s.ad_value(62), 1.0 / (s.v[21])));
        }

        if (!s.b[642]) {
            s.store_scalar(65, s.v[18]);
        }

        s.store_div_scaled_product_indices(34, 4, 1, 1.0, 160, 1.0);

        s.store_mul(219, 219, 34);

        s.store_mul_neg_lhs(218, 218, 34);

        s.store_mul(220, 220, 34);

        s.store_mul_neg_lhs(217, 217, 34);

        s.store_mul_ad_affine_product_rhs(228, 4, s.ad_value(396), A::voltage(ctx, nodes, Some(7), Some(6)), s.v[17], 0.0);

        s.store_mul_ad_affine_product_rhs(230, 4, s.ad_value(397), A::voltage(ctx, nodes, Some(7), Some(5)), s.v[17], 0.0);

        s.store_mul_sub_rhs(240, 212, 288, 99);

        s.store_add_scaled_inputs4_offset(34, s.ad_value(235), 1.0, s.ad_value(200), (-1.0), s.ad_value(32), ((p.p45 / p.p46) * p.p269), s.ad_value(240), (-((p.p45 / p.p46) * p.p269)), (0.02 + ((-p.p268) * ((p.p45 / p.p46) * p.p269))));

        s.store_scaled_sub_ad_rhs(232, 34, A::sqrt(A::offset(A::square(s.ad_value(34)), (4.0 * 0.02))), 0.5);

        s.store_add_scaled_inputs3(35, s.ad_value(235), 1.0, s.ad_value(200), (-1.0), s.ad_value(232), -1.0);

        s.store_add_ad_rhs(228, 228, A::mul3_scaled_output(s.ad_value(212), s.ad_value(4), A::sub(s.ad_value(35), A::scaled_offset(A::sqrt(A::sub_from_scalar(1.0, A::scale(s.ad_value(232), (4.0 * 1.0 / (p.p265))))), (-1.0), (0.5 * p.p265))), p.p263));

        s.store_add_scaled_inputs4_offset(34, s.ad_value(234), 1.0, s.ad_value(200), (-1.0), s.ad_value(33), ((p.p45 / p.p46) * p.p271), s.ad_value(240), (-((p.p45 / p.p46) * p.p271)), (0.02 + ((-p.p270) * ((p.p45 / p.p46) * p.p271))));

        s.store_scaled_sub_ad_rhs(233, 34, A::sqrt(A::offset(A::square(s.ad_value(34)), (4.0 * 0.02))), 0.5);

        s.store_add_scaled_inputs3(35, s.ad_value(234), 1.0, s.ad_value(200), (-1.0), s.ad_value(233), -1.0);

        s.store_add_ad_rhs(230, 230, A::mul3_scaled_output(s.ad_value(212), s.ad_value(4), A::sub(s.ad_value(35), A::scaled_offset(A::sqrt(A::sub_from_scalar(1.0, A::scale(s.ad_value(233), (4.0 * 1.0 / (p.p266))))), (-1.0), (0.5 * p.p266))), p.p264));

        s.store_mul_ad_product_rhs(229, 4, s.ad_value(398), A::voltage(ctx, nodes, Some(7), Some(6)));

        s.store_mul_ad_product_rhs(231, 4, s.ad_value(399), A::voltage(ctx, nodes, Some(7), Some(5)));

        s.store_add(226, 228, 229);

        s.store_add(227, 230, 231);

        s.store_ad_value(238, A::mul_scaled_lhs(s.ad_value(212), s.v[236], A::voltage(ctx, nodes, Some(6), Some(3))));

        s.store_ad_value(239, A::mul_scaled_lhs(s.ad_value(212), s.v[237], A::voltage(ctx, nodes, Some(5), Some(3))));

        s.store_div_scaled_add_product(34, s.ad_value(366), 1.0, s.ad_value(367), s.ad_value(2), 1.0, s.ad_value(2), 1.0);

        s.b[643] = ((s.v[34] <= 0.0) || (s.v[103] <= 0.0));
        s.v[643] = if s.b[643] { 1.0 } else { 0.0 };

        s.b[644] = (s.v[155] > (s.v[103] / 80.0));
        s.v[644] = if s.b[644] { 1.0 } else { 0.0 };

        if ((!s.b[643]) && s.b[644]) {
            s.store_div_scaled_inputs(35, s.ad_value(103), -1.0, s.ad_value(155), 1.0);
        }

        s.b[645] = (p.p17 != 0.0);
        s.v[645] = if s.b[645] { 1.0 } else { 0.0 };

        if s.b[645] {
            s.store_div_scaled_inputs2_by_product(35, s.ad_value(46), 1.0, s.ad_value(411), (-1.0), s.ad_value(412), s.ad_value(55), 1.0);
            s.store_add_scaled_product_indices(36, 408, 1.0, 409, 46, (-1.0));
            s.store_offset_mul(37, 410, 46, 1.0);
            s.store_scaled_mul(38, 36, 37, ((-982222000000.0) * p.p99));
            s.store_limited_exp(39, 38);
            s.store_scalar(40, 3.75956e-7);
            s.store_sub(191, 52, 50);
            s.store_sub(34, 191, 209);
            s.store_div_scaled_value_by_product(35, s.ad_value(34), 1.0, s.ad_value(416), s.ad_value(55), 1.0);
        }

        s.b[646] = (s.v[191] <= 0.0);
        s.v[646] = if s.b[646] { 1.0 } else { 0.0 };

        if (s.b[645] && s.b[646]) {
            s.store_scaled_add_ad(189, A::offset(s.ad_value(34), (-0.02)), A::sqrt(A::add_scaled_offset_product_lhs(s.ad_value(191), (-0.08), s.ad_value(34), (-0.02), A::offset(s.ad_value(34), (-0.02)), 1.0)), 0.5);
        }

        if (s.b[645] && (!s.b[646])) {
            s.store_scaled_add_ad(189, A::offset(s.ad_value(34), (-0.02)), A::sqrt(A::add_scaled_offset_product_lhs(s.ad_value(191), 0.08, s.ad_value(34), (-0.02), A::offset(s.ad_value(34), (-0.02)), 1.0)), 0.5);
        }

        if s.b[645] {
            s.store_add_scaled_product_indices(36, 413, 1.0, 414, 189, (-1.0));
            s.store_offset_mul(37, 415, 189, 1.0);
            s.store_scaled_mul(38, 36, 37, ((-745669000000.0) * p.p99));
            s.store_limited_exp(39, 38);
            s.store_scalar(40, 4.97232e-7);
        }

        s.store_tanh_ad(34, A::div_scaled_inputs(s.ad_value(30), 0.6, s.ad_value(55), 1.0));

        s.b[647] = (p.p16 != 0.0);
        s.v[647] = if s.b[647] { 1.0 } else { 0.0 };

        if s.b[647] {
            s.store_add_scaled_product_right_ad(35, 369, 1.0, 370, A::add_scaled_product(s.ad_value(69), 1.0, s.ad_value(373), s.ad_value(210), (-1.0)), (-1.0));
            s.store_offset_mul_ad(36, s.ad_value(371), A::add_scaled_product(s.ad_value(69), 1.0, s.ad_value(373), s.ad_value(210), (-1.0)), 1.0);
            s.store_mul3_affine_lhs(37, 206, 35, (-p.p99), 0.0, 36);
            s.store_mul_limited_exp_rhs(38, 46, 37);
            s.store_add_scaled_inputs4(39, s.ad_value(209), 1.0, s.ad_value(73), 0.5, s.ad_value(32), 0.5, s.ad_value(33), 0.5);
            s.store_offset_sqrt_ad(196, A::offset(A::square(s.ad_value(113)), 0.01), (-0.1));
            s.store_mul(35, 372, 196);
            s.store_limited_exp_neg_input(197, 35);
            s.store_offset_add(37, 35, 197, (((-1.0)) + (0.0001)));
            s.store_offset_sub_from_scalar_ad(38, 1.0, A::mul_offset_lhs(s.ad_value(35), 1.0, s.ad_value(197)), 0.0001);
            s.store_offset_square(39, 35, 0.0002);
            s.store_add_scaled_inputs_product_right_ad(34, 29, 1.0, 200, (-1.0), 385, A::sub(s.ad_value(23), s.ad_value(240)), s.v[243]);
            s.store_sqrt_square_offset(203, 34, 0.0001);
        }

    }

    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[647] {
            s.store_add_scaled_product_indices(35, 382, 1.0, 383, 203, (-1.0));
            s.store_offset_mul(36, 384, 203, 1.0);
            s.store_mul_ad_lhs(37, A::mul3_scaled_output(s.ad_value(206), s.ad_value(394), s.ad_value(35), (-p.p99)), 36);
            s.store_limited_exp(38, 37);
            s.store_add_scaled_inputs_product_right_ad(34, 31, 1.0, 200, (-1.0), 389, A::sub(s.ad_value(23), s.ad_value(240)), s.v[243]);
            s.store_sqrt_square_offset(204, 34, 0.0001);
            s.store_add_scaled_product_indices(35, 386, 1.0, 387, 204, (-1.0));
            s.store_offset_mul(36, 388, 204, 1.0);
            s.store_mul_ad_lhs(37, A::mul3_scaled_output(s.ad_value(206), s.ad_value(394), s.ad_value(35), (-p.p99)), 36);
            s.store_limited_exp(38, 37);
        }

        s.b[650] = (p.p15 != 0.0);
        s.v[650] = if s.b[650] { 1.0 } else { 0.0 };

        if s.b[650] {
            s.store_scalar(34, (s.v[21] * p.p45));
        }

        s.b[651] = ((s.v[378] <= 0.0) || (s.v[104] <= 0.0));
        s.v[651] = if s.b[651] { 1.0 } else { 0.0 };

        if (s.b[650] && s.b[651]) {
            s.store_scalar(40, 0.0);
        }

        if (s.b[650] && (!s.b[651])) {
            s.store_div_scaled_add_product(35, A::add_scaled_inputs3(s.ad_value(31), -1.0, s.ad_value(380), (-1.0), s.ad_value(200), 1.0), 1.0, s.ad_value(390), A::add_scaled_inputs3(s.ad_value(23), 1.0, s.ad_value(240), (-1.0), s.ad_value(391), -1.0), s.v[243], s.ad_value(34), 1.0);
            s.store_scaled_add_ad_rhs(35, 35, A::sqrt(A::offset(A::square(s.ad_value(35)), ((4.0 * 0.01) * 0.01))), 0.5);
            s.store_div_scaled_value_offset_denominator(36, s.ad_value(104), 1.0, s.ad_value(35), 0.001, 1.0);
            s.store_limited_exp_ad(37, A::mul(s.ad_value(381), A::ln(A::max_with_scalar(s.ad_value(35), 1e-38))));
            s.store_mul_ad_product_lhs(40, A::mul3(s.ad_value(378), s.ad_value(3), s.ad_value(37)), A::limited_exp_scaled_input(s.ad_value(36), -1.0), 30);
        }

        s.b[653] = ((s.v[374] <= 0.0) || (s.v[105] <= 0.0));
        s.v[653] = if s.b[653] { 1.0 } else { 0.0 };

        if (s.b[650] && s.b[653]) {
            s.store_scalar(40, 0.0);
        }

        if (s.b[650] && (!s.b[653])) {
            s.store_div_scaled_add_product(35, A::add_scaled_inputs3(s.ad_value(29), -1.0, s.ad_value(376), (-1.0), s.ad_value(200), 1.0), 1.0, s.ad_value(392), A::add_scaled_inputs3(s.ad_value(23), 1.0, s.ad_value(240), (-1.0), s.ad_value(393), -1.0), s.v[243], s.ad_value(34), 1.0);
            s.store_scaled_add_ad_rhs(35, 35, A::sqrt(A::offset(A::square(s.ad_value(35)), ((4.0 * 0.01) * 0.01))), 0.5);
            s.store_div_scaled_value_offset_denominator(36, s.ad_value(105), 1.0, s.ad_value(35), 0.001, 1.0);
            s.store_limited_exp_ad(37, A::mul(s.ad_value(377), A::ln(A::max_with_scalar(s.ad_value(35), 1e-38))));
            s.store_ad_value(40, A::mul3(A::mul3_scaled_output(s.ad_value(30), s.ad_value(374), s.ad_value(3), -1.0), s.ad_value(37), A::limited_exp_scaled_input(s.ad_value(36), -1.0)));
        }

        s.store_div_scaled_inputs(254, s.ad_value(164), 2.0, s.ad_value(121), 1.0);

        s.b[655] = (((p.p288 > 0.0) || (p.p289 > 0.0)) || (p.p290 > 0.0));
        s.v[655] = if s.b[655] { 1.0 } else { 0.0 };

        if s.b[655] {
            s.store_sub_scaled_inputs(255, 2, 1.0, 249, 2.0);
            s.store_square(256, 255);
        }

        s.b[656] = (p.p287 <= 0.0);
        s.v[656] = if s.b[656] { 1.0 } else { 0.0 };

        if (s.b[655] && s.b[656]) {
            s.store_scalar(257, 0.0);
        }

        if (s.b[655] && (!s.b[656])) {
            s.store_div_scaled_offset_numerator(34, s.ad_value(155), 1.0 / (s.v[253]), p.p287, s.ad_value(254), 1.0);
            s.store_scaled_ln_ad(257, A::max_with_scalar(s.ad_value(34), 1e-38), s.v[253]);
        }

        s.b[657] = (s.v[257] < 0.0);
        s.v[657] = if s.b[657] { 1.0 } else { 0.0 };

        if ((s.b[655] && (!s.b[656])) && s.b[657]) {
            s.store_scalar(257, 0.0);
        }

        s.b[658] = (p.p22 == 1.0);
        s.v[658] = if s.b[658] { 1.0 } else { 0.0 };

        if (s.b[655] && s.b[658]) {
            s.store_div(35, 47, 252);
            s.store_offset_pow_ad(36, s.ad_value(35), s.ad_value(251), 1.0);
            s.store_div(37, 250, 36);
            s.store_scale(38, 37, 1.0 / (p.p288));
            s.store_scaled_add_ad(39, A::offset(s.ad_value(38), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(38), (-1.0), A::offset(s.ad_value(38), (-1.0))), ((0.25 * p.p292) * p.p292))), 0.5);
            s.store_scale(258, 39, p.p288);
        }

        if (s.b[655] && (!s.b[658])) {
            s.store_scalar(258, p.p288);
        }

        if s.b[655] {
            s.store_mul_ad_affine_product_lhs(35, s.ad_value(55), A::abs(s.ad_value(214)), ((1.60219e-19 * 1.60219e-19) * 1.60219e-19), 0.0, 121);
            s.store_scaled_mul(36, 65, 256, 10000000000.0);
            s.store_scaled_mul(259, 65, 109, 6.241457005723417e18);
            s.store_scaled_mul(260, 65, 110, 6.241457005723417e18);
            s.store_mul_scaled_ad_rhs(261, 55, 1.0 / (1.60219e-19), A::add(s.ad_value(65), s.ad_value(291)));
            s.store_mul_ln_ad_rhs(37, 258, A::max_with_scalar(A::div_scaled_inputs2(s.ad_value(259), 1.0, s.ad_value(261), 1.0, A::add(s.ad_value(260), s.ad_value(261)), 1.0), 1e-38));
            s.store_scaled_sub(38, 259, 260, p.p289);
            s.store_scaled_sub_ad(39, A::square(s.ad_value(259)), A::square(s.ad_value(260)), (0.5 * p.p290));
            s.store_mul3_affine_lhs(40, 55, 214, 1.60219e-19, 0.0, 214);
            s.store_scaled_mul(41, 256, 3, (10000000000.0 * p.p2));
            s.store_add_scaled_inputs_product_indices(42, 258, 1.0, 260, p.p289, 260, 260, p.p290);
            s.store_mul_ad(43, A::add(s.ad_value(260), s.ad_value(261)), A::add(s.ad_value(260), s.ad_value(261)));
            s.store_add_scaled_product(262, A::div_scaled_product3_by_product(s.ad_value(40), s.ad_value(257), s.ad_value(42), 1.0, s.ad_value(41), s.ad_value(43), 1.0), 1.0, A::div(s.ad_value(35), s.ad_value(36)), A::add_scaled_inputs3(s.ad_value(37), 1.0, s.ad_value(38), 1.0, s.ad_value(39), 1.0), 1.0);
            s.store_scaled_mul(44, 258, 55, 1.60219e-19);
            s.store_mul_ad_lhs(45, A::mul3_scaled_output(s.ad_value(3), s.ad_value(255), s.ad_value(261), (p.p2 * 10000000000.0)), 261);
            s.store_mul_ad_product_lhs(263, A::div(s.ad_value(44), s.ad_value(45)), s.ad_value(214), 214);
            s.store_add(35, 263, 262);
        }

        s.store_scaled_mul(224, 212, 219, p.p2);

        s.store_scale(225, 220, p.p2);

        s.b[660] = (s.v[27] > 0.0);
        s.v[660] = if s.b[660] { 1.0 } else { 0.0 };

        if s.b[660] {
            s.store_scale(222, 217, p.p2);
            s.store_scale(223, 218, p.p2);
            s.store_add_scaled_inputs3(217, s.ad_value(217), p.p2, s.ad_value(226), (-p.p2), s.ad_value(238), 1.0);
            s.store_add_scaled_inputs3(218, s.ad_value(218), p.p2, s.ad_value(227), (-p.p2), s.ad_value(239), 1.0);
        }

        if (!s.b[660]) {
            s.store_scale(222, 218, p.p2);
            s.store_scale(223, 217, p.p2);
            s.store_add_scaled_inputs3(34, s.ad_value(218), p.p2, s.ad_value(226), (-p.p2), s.ad_value(238), 1.0);
            s.store_add_scaled_inputs3(218, s.ad_value(217), p.p2, s.ad_value(227), (-p.p2), s.ad_value(239), 1.0);
            s.copy_ad(217, 34);
        }

        s.store_add_scaled_inputs3(219, s.ad_value(224), 1.0, s.ad_value(226), p.p2, s.ad_value(227), p.p2);

        s.store_add_scaled_inputs3(220, s.ad_value(220), p.p2, s.ad_value(238), (-1.0), s.ad_value(239), -1.0);

        s.store_scale(226, 226, p.p2);

        s.store_scale(227, 227, p.p2);

        s.store_neg_ad(265, A::add(s.ad_value(222), s.ad_value(223)));

        s.store_mul(34, 121, 265);

        s.store_add_scaled_square_product(35, s.ad_value(2), 1.0, s.ad_value(34), s.ad_value(151), 1.0);

        s.b[661] = ((p.p20 == 1.0) && (s.v[275] != 0.0));
        s.v[661] = if s.b[661] { 1.0 } else { 0.0 };

        if s.b[661] {
            s.store_div_scaled_product_indices(34, 121, 3, s.v[17], 2, 1.0);
        }

        s.b[671] = ((p.p18 != 0.0) && (p.p310 > 0.0));
        s.v[671] = if s.b[671] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq0_e787, eq0_e787_d_n0, eq0_e787_d_n1, eq0_e787_d_n2, eq0_e787_d_n3, eq0_e787_d_n4, eq0_e787_d_n5, eq0_e787_d_n6, eq0_e787_d_n7, eq0_e787_d_n8, eq0_e787_d_b0, eq0_e787_d_b1, eq0_e787_d_b2, eq0_e787_d_b3, eq0_e787_d_b4,) = {
    if s.b[662] {
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
        let eq0_e779_d_b0: f64 = ((s.db[212][0] * s.v[214]) + (s.v[212] * s.db[214][0]));
        let eq0_e779_d_b1: f64 = ((s.db[212][1] * s.v[214]) + (s.v[212] * s.db[214][1]));
        let eq0_e779_d_b2: f64 = ((s.db[212][2] * s.v[214]) + (s.v[212] * s.db[214][2]));
        let eq0_e779_d_b3: f64 = ((s.db[212][3] * s.v[214]) + (s.v[212] * s.db[214][3]));
        let eq0_e779_d_b4: f64 = ((s.db[212][4] * s.v[214]) + (s.v[212] * s.db[214][4]));
        let eq0_e782: f64 = 1e-12;
        let eq0_e784: f64 = (eq0_e782 * (nv5 - nv6));
        let eq0_e784_d_n6: f64 = (-eq0_e782);
        let eq0_e785: f64 = (eq0_e779 + eq0_e784);
        let eq0_e785_d_n5: f64 = (eq0_e779_d_n5 + eq0_e782);
        let eq0_e785_d_n6: f64 = (eq0_e779_d_n6 + eq0_e784_d_n6);
        (eq0_e785, eq0_e779_d_n0, eq0_e779_d_n1, eq0_e779_d_n2, eq0_e779_d_n3, eq0_e779_d_n4, eq0_e785_d_n5, eq0_e785_d_n6, eq0_e779_d_n7, eq0_e779_d_n8, eq0_e779_d_b0, eq0_e779_d_b1, eq0_e779_d_b2, eq0_e779_d_b3, eq0_e779_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e787;
        let eq0_node_derivatives: [f64; 9] = [eq0_e787_d_n0, eq0_e787_d_n1, eq0_e787_d_n2, eq0_e787_d_n3, eq0_e787_d_n4, eq0_e787_d_n5, eq0_e787_d_n6, eq0_e787_d_n7, eq0_e787_d_n8];
        let eq0_branch_derivatives: [f64; 5] = [eq0_e787_d_b0, eq0_e787_d_b1, eq0_e787_d_b2, eq0_e787_d_b3, eq0_e787_d_b4];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let (eq1_e795, eq1_e795_d_n0, eq1_e795_d_n1, eq1_e795_d_n2, eq1_e795_d_n3, eq1_e795_d_n4, eq1_e795_d_n5, eq1_e795_d_n6, eq1_e795_d_n7, eq1_e795_d_n8, eq1_e795_d_b0, eq1_e795_d_b1, eq1_e795_d_b2, eq1_e795_d_b3, eq1_e795_d_b4,) = {
    if s.b[662] {
        let eq1_e792: f64 = (s.v[199] + s.v[211]);
        let eq1_e792_d_n0: f64 = (s.dn[199][0] + s.dn[211][0]);
        let eq1_e792_d_n1: f64 = (s.dn[199][1] + s.dn[211][1]);
        let eq1_e792_d_n2: f64 = (s.dn[199][2] + s.dn[211][2]);
        let eq1_e792_d_n3: f64 = (s.dn[199][3] + s.dn[211][3]);
        let eq1_e792_d_n4: f64 = (s.dn[199][4] + s.dn[211][4]);
        let eq1_e792_d_n5: f64 = (s.dn[199][5] + s.dn[211][5]);
        let eq1_e792_d_n6: f64 = (s.dn[199][6] + s.dn[211][6]);
        let eq1_e792_d_n7: f64 = (s.dn[199][7] + s.dn[211][7]);
        let eq1_e792_d_n8: f64 = (s.dn[199][8] + s.dn[211][8]);
        let eq1_e792_d_b0: f64 = (s.db[199][0] + s.db[211][0]);
        let eq1_e792_d_b1: f64 = (s.db[199][1] + s.db[211][1]);
        let eq1_e792_d_b2: f64 = (s.db[199][2] + s.db[211][2]);
        let eq1_e792_d_b3: f64 = (s.db[199][3] + s.db[211][3]);
        let eq1_e792_d_b4: f64 = (s.db[199][4] + s.db[211][4]);
        let eq1_e793: f64 = (s.v[212] * eq1_e792);
        let eq1_e793_d_n0: f64 = ((s.dn[212][0] * eq1_e792) + (s.v[212] * eq1_e792_d_n0));
        let eq1_e793_d_n1: f64 = ((s.dn[212][1] * eq1_e792) + (s.v[212] * eq1_e792_d_n1));
        let eq1_e793_d_n2: f64 = ((s.dn[212][2] * eq1_e792) + (s.v[212] * eq1_e792_d_n2));
        let eq1_e793_d_n3: f64 = ((s.dn[212][3] * eq1_e792) + (s.v[212] * eq1_e792_d_n3));
        let eq1_e793_d_n4: f64 = ((s.dn[212][4] * eq1_e792) + (s.v[212] * eq1_e792_d_n4));
        let eq1_e793_d_n5: f64 = ((s.dn[212][5] * eq1_e792) + (s.v[212] * eq1_e792_d_n5));
        let eq1_e793_d_n6: f64 = ((s.dn[212][6] * eq1_e792) + (s.v[212] * eq1_e792_d_n6));
        let eq1_e793_d_n7: f64 = ((s.dn[212][7] * eq1_e792) + (s.v[212] * eq1_e792_d_n7));
        let eq1_e793_d_n8: f64 = ((s.dn[212][8] * eq1_e792) + (s.v[212] * eq1_e792_d_n8));
        let eq1_e793_d_b0: f64 = ((s.db[212][0] * eq1_e792) + (s.v[212] * eq1_e792_d_b0));
        let eq1_e793_d_b1: f64 = ((s.db[212][1] * eq1_e792) + (s.v[212] * eq1_e792_d_b1));
        let eq1_e793_d_b2: f64 = ((s.db[212][2] * eq1_e792) + (s.v[212] * eq1_e792_d_b2));
        let eq1_e793_d_b3: f64 = ((s.db[212][3] * eq1_e792) + (s.v[212] * eq1_e792_d_b3));
        let eq1_e793_d_b4: f64 = ((s.db[212][4] * eq1_e792) + (s.v[212] * eq1_e792_d_b4));
        (eq1_e793, eq1_e793_d_n0, eq1_e793_d_n1, eq1_e793_d_n2, eq1_e793_d_n3, eq1_e793_d_n4, eq1_e793_d_n5, eq1_e793_d_n6, eq1_e793_d_n7, eq1_e793_d_n8, eq1_e793_d_b0, eq1_e793_d_b1, eq1_e793_d_b2, eq1_e793_d_b3, eq1_e793_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e795;
        let eq1_node_derivatives: [f64; 9] = [eq1_e795_d_n0, eq1_e795_d_n1, eq1_e795_d_n2, eq1_e795_d_n3, eq1_e795_d_n4, eq1_e795_d_n5, eq1_e795_d_n6, eq1_e795_d_n7, eq1_e795_d_n8];
        let eq1_branch_derivatives: [f64; 5] = [eq1_e795_d_b0, eq1_e795_d_b1, eq1_e795_d_b2, eq1_e795_d_b3, eq1_e795_d_b4];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let (eq2_e801, eq2_e801_d_n0, eq2_e801_d_n1, eq2_e801_d_n2, eq2_e801_d_n3, eq2_e801_d_n4, eq2_e801_d_n5, eq2_e801_d_n6, eq2_e801_d_n7, eq2_e801_d_n8, eq2_e801_d_b0, eq2_e801_d_b1, eq2_e801_d_b2, eq2_e801_d_b3, eq2_e801_d_b4,) = {
    if s.b[662] {
        let eq2_e799: f64 = (s.v[212] * s.v[198]);
        let eq2_e799_d_n0: f64 = ((s.dn[212][0] * s.v[198]) + (s.v[212] * s.dn[198][0]));
        let eq2_e799_d_n1: f64 = ((s.dn[212][1] * s.v[198]) + (s.v[212] * s.dn[198][1]));
        let eq2_e799_d_n2: f64 = ((s.dn[212][2] * s.v[198]) + (s.v[212] * s.dn[198][2]));
        let eq2_e799_d_n3: f64 = ((s.dn[212][3] * s.v[198]) + (s.v[212] * s.dn[198][3]));
        let eq2_e799_d_n4: f64 = ((s.dn[212][4] * s.v[198]) + (s.v[212] * s.dn[198][4]));
        let eq2_e799_d_n5: f64 = ((s.dn[212][5] * s.v[198]) + (s.v[212] * s.dn[198][5]));
        let eq2_e799_d_n6: f64 = ((s.dn[212][6] * s.v[198]) + (s.v[212] * s.dn[198][6]));
        let eq2_e799_d_n7: f64 = ((s.dn[212][7] * s.v[198]) + (s.v[212] * s.dn[198][7]));
        let eq2_e799_d_n8: f64 = ((s.dn[212][8] * s.v[198]) + (s.v[212] * s.dn[198][8]));
        let eq2_e799_d_b0: f64 = ((s.db[212][0] * s.v[198]) + (s.v[212] * s.db[198][0]));
        let eq2_e799_d_b1: f64 = ((s.db[212][1] * s.v[198]) + (s.v[212] * s.db[198][1]));
        let eq2_e799_d_b2: f64 = ((s.db[212][2] * s.v[198]) + (s.v[212] * s.db[198][2]));
        let eq2_e799_d_b3: f64 = ((s.db[212][3] * s.v[198]) + (s.v[212] * s.db[198][3]));
        let eq2_e799_d_b4: f64 = ((s.db[212][4] * s.v[198]) + (s.v[212] * s.db[198][4]));
        (eq2_e799, eq2_e799_d_n0, eq2_e799_d_n1, eq2_e799_d_n2, eq2_e799_d_n3, eq2_e799_d_n4, eq2_e799_d_n5, eq2_e799_d_n6, eq2_e799_d_n7, eq2_e799_d_n8, eq2_e799_d_b0, eq2_e799_d_b1, eq2_e799_d_b2, eq2_e799_d_b3, eq2_e799_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e801;
        let eq2_node_derivatives: [f64; 9] = [eq2_e801_d_n0, eq2_e801_d_n1, eq2_e801_d_n2, eq2_e801_d_n3, eq2_e801_d_n4, eq2_e801_d_n5, eq2_e801_d_n6, eq2_e801_d_n7, eq2_e801_d_n8];
        let eq2_branch_derivatives: [f64; 5] = [eq2_e801_d_b0, eq2_e801_d_b1, eq2_e801_d_b2, eq2_e801_d_b3, eq2_e801_d_b4];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let (eq3_e809, eq3_e809_d_n0, eq3_e809_d_n1, eq3_e809_d_n2, eq3_e809_d_n3, eq3_e809_d_n4, eq3_e809_d_n5, eq3_e809_d_n6, eq3_e809_d_n7, eq3_e809_d_n8, eq3_e809_d_b0, eq3_e809_d_b1, eq3_e809_d_b2, eq3_e809_d_b3, eq3_e809_d_b4,) = {
    if s.b[662] {
        let eq3_e806: f64 = (s.v[193] + s.v[201]);
        let eq3_e806_d_n0: f64 = (s.dn[193][0] + s.dn[201][0]);
        let eq3_e806_d_n1: f64 = (s.dn[193][1] + s.dn[201][1]);
        let eq3_e806_d_n2: f64 = (s.dn[193][2] + s.dn[201][2]);
        let eq3_e806_d_n3: f64 = (s.dn[193][3] + s.dn[201][3]);
        let eq3_e806_d_n4: f64 = (s.dn[193][4] + s.dn[201][4]);
        let eq3_e806_d_n5: f64 = (s.dn[193][5] + s.dn[201][5]);
        let eq3_e806_d_n6: f64 = (s.dn[193][6] + s.dn[201][6]);
        let eq3_e806_d_n7: f64 = (s.dn[193][7] + s.dn[201][7]);
        let eq3_e806_d_n8: f64 = (s.dn[193][8] + s.dn[201][8]);
        let eq3_e806_d_b0: f64 = (s.db[193][0] + s.db[201][0]);
        let eq3_e806_d_b1: f64 = (s.db[193][1] + s.db[201][1]);
        let eq3_e806_d_b2: f64 = (s.db[193][2] + s.db[201][2]);
        let eq3_e806_d_b3: f64 = (s.db[193][3] + s.db[201][3]);
        let eq3_e806_d_b4: f64 = (s.db[193][4] + s.db[201][4]);
        let eq3_e807: f64 = (s.v[212] * eq3_e806);
        let eq3_e807_d_n0: f64 = ((s.dn[212][0] * eq3_e806) + (s.v[212] * eq3_e806_d_n0));
        let eq3_e807_d_n1: f64 = ((s.dn[212][1] * eq3_e806) + (s.v[212] * eq3_e806_d_n1));
        let eq3_e807_d_n2: f64 = ((s.dn[212][2] * eq3_e806) + (s.v[212] * eq3_e806_d_n2));
        let eq3_e807_d_n3: f64 = ((s.dn[212][3] * eq3_e806) + (s.v[212] * eq3_e806_d_n3));
        let eq3_e807_d_n4: f64 = ((s.dn[212][4] * eq3_e806) + (s.v[212] * eq3_e806_d_n4));
        let eq3_e807_d_n5: f64 = ((s.dn[212][5] * eq3_e806) + (s.v[212] * eq3_e806_d_n5));
        let eq3_e807_d_n6: f64 = ((s.dn[212][6] * eq3_e806) + (s.v[212] * eq3_e806_d_n6));
        let eq3_e807_d_n7: f64 = ((s.dn[212][7] * eq3_e806) + (s.v[212] * eq3_e806_d_n7));
        let eq3_e807_d_n8: f64 = ((s.dn[212][8] * eq3_e806) + (s.v[212] * eq3_e806_d_n8));
        let eq3_e807_d_b0: f64 = ((s.db[212][0] * eq3_e806) + (s.v[212] * eq3_e806_d_b0));
        let eq3_e807_d_b1: f64 = ((s.db[212][1] * eq3_e806) + (s.v[212] * eq3_e806_d_b1));
        let eq3_e807_d_b2: f64 = ((s.db[212][2] * eq3_e806) + (s.v[212] * eq3_e806_d_b2));
        let eq3_e807_d_b3: f64 = ((s.db[212][3] * eq3_e806) + (s.v[212] * eq3_e806_d_b3));
        let eq3_e807_d_b4: f64 = ((s.db[212][4] * eq3_e806) + (s.v[212] * eq3_e806_d_b4));
        (eq3_e807, eq3_e807_d_n0, eq3_e807_d_n1, eq3_e807_d_n2, eq3_e807_d_n3, eq3_e807_d_n4, eq3_e807_d_n5, eq3_e807_d_n6, eq3_e807_d_n7, eq3_e807_d_n8, eq3_e807_d_b0, eq3_e807_d_b1, eq3_e807_d_b2, eq3_e807_d_b3, eq3_e807_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e809;
        let eq3_node_derivatives: [f64; 9] = [eq3_e809_d_n0, eq3_e809_d_n1, eq3_e809_d_n2, eq3_e809_d_n3, eq3_e809_d_n4, eq3_e809_d_n5, eq3_e809_d_n6, eq3_e809_d_n7, eq3_e809_d_n8];
        let eq3_branch_derivatives: [f64; 5] = [eq3_e809_d_b0, eq3_e809_d_b1, eq3_e809_d_b2, eq3_e809_d_b3, eq3_e809_d_b4];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let (eq4_e817, eq4_e817_d_n0, eq4_e817_d_n1, eq4_e817_d_n2, eq4_e817_d_n3, eq4_e817_d_n4, eq4_e817_d_n5, eq4_e817_d_n6, eq4_e817_d_n7, eq4_e817_d_n8, eq4_e817_d_b0, eq4_e817_d_b1, eq4_e817_d_b2, eq4_e817_d_b3, eq4_e817_d_b4,) = {
    if s.b[662] {
        let eq4_e814: f64 = (s.v[194] + s.v[202]);
        let eq4_e814_d_n0: f64 = (s.dn[194][0] + s.dn[202][0]);
        let eq4_e814_d_n1: f64 = (s.dn[194][1] + s.dn[202][1]);
        let eq4_e814_d_n2: f64 = (s.dn[194][2] + s.dn[202][2]);
        let eq4_e814_d_n3: f64 = (s.dn[194][3] + s.dn[202][3]);
        let eq4_e814_d_n4: f64 = (s.dn[194][4] + s.dn[202][4]);
        let eq4_e814_d_n5: f64 = (s.dn[194][5] + s.dn[202][5]);
        let eq4_e814_d_n6: f64 = (s.dn[194][6] + s.dn[202][6]);
        let eq4_e814_d_n7: f64 = (s.dn[194][7] + s.dn[202][7]);
        let eq4_e814_d_n8: f64 = (s.dn[194][8] + s.dn[202][8]);
        let eq4_e814_d_b0: f64 = (s.db[194][0] + s.db[202][0]);
        let eq4_e814_d_b1: f64 = (s.db[194][1] + s.db[202][1]);
        let eq4_e814_d_b2: f64 = (s.db[194][2] + s.db[202][2]);
        let eq4_e814_d_b3: f64 = (s.db[194][3] + s.db[202][3]);
        let eq4_e814_d_b4: f64 = (s.db[194][4] + s.db[202][4]);
        let eq4_e815: f64 = (s.v[212] * eq4_e814);
        let eq4_e815_d_n0: f64 = ((s.dn[212][0] * eq4_e814) + (s.v[212] * eq4_e814_d_n0));
        let eq4_e815_d_n1: f64 = ((s.dn[212][1] * eq4_e814) + (s.v[212] * eq4_e814_d_n1));
        let eq4_e815_d_n2: f64 = ((s.dn[212][2] * eq4_e814) + (s.v[212] * eq4_e814_d_n2));
        let eq4_e815_d_n3: f64 = ((s.dn[212][3] * eq4_e814) + (s.v[212] * eq4_e814_d_n3));
        let eq4_e815_d_n4: f64 = ((s.dn[212][4] * eq4_e814) + (s.v[212] * eq4_e814_d_n4));
        let eq4_e815_d_n5: f64 = ((s.dn[212][5] * eq4_e814) + (s.v[212] * eq4_e814_d_n5));
        let eq4_e815_d_n6: f64 = ((s.dn[212][6] * eq4_e814) + (s.v[212] * eq4_e814_d_n6));
        let eq4_e815_d_n7: f64 = ((s.dn[212][7] * eq4_e814) + (s.v[212] * eq4_e814_d_n7));
        let eq4_e815_d_n8: f64 = ((s.dn[212][8] * eq4_e814) + (s.v[212] * eq4_e814_d_n8));
        let eq4_e815_d_b0: f64 = ((s.db[212][0] * eq4_e814) + (s.v[212] * eq4_e814_d_b0));
        let eq4_e815_d_b1: f64 = ((s.db[212][1] * eq4_e814) + (s.v[212] * eq4_e814_d_b1));
        let eq4_e815_d_b2: f64 = ((s.db[212][2] * eq4_e814) + (s.v[212] * eq4_e814_d_b2));
        let eq4_e815_d_b3: f64 = ((s.db[212][3] * eq4_e814) + (s.v[212] * eq4_e814_d_b3));
        let eq4_e815_d_b4: f64 = ((s.db[212][4] * eq4_e814) + (s.v[212] * eq4_e814_d_b4));
        (eq4_e815, eq4_e815_d_n0, eq4_e815_d_n1, eq4_e815_d_n2, eq4_e815_d_n3, eq4_e815_d_n4, eq4_e815_d_n5, eq4_e815_d_n6, eq4_e815_d_n7, eq4_e815_d_n8, eq4_e815_d_b0, eq4_e815_d_b1, eq4_e815_d_b2, eq4_e815_d_b3, eq4_e815_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e817;
        let eq4_node_derivatives: [f64; 9] = [eq4_e817_d_n0, eq4_e817_d_n1, eq4_e817_d_n2, eq4_e817_d_n3, eq4_e817_d_n4, eq4_e817_d_n5, eq4_e817_d_n6, eq4_e817_d_n7, eq4_e817_d_n8];
        let eq4_branch_derivatives: [f64; 5] = [eq4_e817_d_b0, eq4_e817_d_b1, eq4_e817_d_b2, eq4_e817_d_b3, eq4_e817_d_b4];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e830, eq5_e830_d_n0, eq5_e830_d_n1, eq5_e830_d_n2, eq5_e830_d_n3, eq5_e830_d_n4, eq5_e830_d_n5, eq5_e830_d_n6, eq5_e830_d_n7, eq5_e830_d_n8, eq5_e830_d_b0, eq5_e830_d_b1, eq5_e830_d_b2, eq5_e830_d_b3, eq5_e830_d_b4,) = {
    if (!s.b[662]) {
        let eq5_e822: f64 = (s.v[212] * s.v[214]);
        let eq5_e822_d_n0: f64 = ((s.dn[212][0] * s.v[214]) + (s.v[212] * s.dn[214][0]));
        let eq5_e822_d_n1: f64 = ((s.dn[212][1] * s.v[214]) + (s.v[212] * s.dn[214][1]));
        let eq5_e822_d_n2: f64 = ((s.dn[212][2] * s.v[214]) + (s.v[212] * s.dn[214][2]));
        let eq5_e822_d_n3: f64 = ((s.dn[212][3] * s.v[214]) + (s.v[212] * s.dn[214][3]));
        let eq5_e822_d_n4: f64 = ((s.dn[212][4] * s.v[214]) + (s.v[212] * s.dn[214][4]));
        let eq5_e822_d_n5: f64 = ((s.dn[212][5] * s.v[214]) + (s.v[212] * s.dn[214][5]));
        let eq5_e822_d_n6: f64 = ((s.dn[212][6] * s.v[214]) + (s.v[212] * s.dn[214][6]));
        let eq5_e822_d_n7: f64 = ((s.dn[212][7] * s.v[214]) + (s.v[212] * s.dn[214][7]));
        let eq5_e822_d_n8: f64 = ((s.dn[212][8] * s.v[214]) + (s.v[212] * s.dn[214][8]));
        let eq5_e822_d_b0: f64 = ((s.db[212][0] * s.v[214]) + (s.v[212] * s.db[214][0]));
        let eq5_e822_d_b1: f64 = ((s.db[212][1] * s.v[214]) + (s.v[212] * s.db[214][1]));
        let eq5_e822_d_b2: f64 = ((s.db[212][2] * s.v[214]) + (s.v[212] * s.db[214][2]));
        let eq5_e822_d_b3: f64 = ((s.db[212][3] * s.v[214]) + (s.v[212] * s.db[214][3]));
        let eq5_e822_d_b4: f64 = ((s.db[212][4] * s.v[214]) + (s.v[212] * s.db[214][4]));
        let eq5_e825: f64 = 1e-12;
        let eq5_e827: f64 = (eq5_e825 * (nv6 - nv5));
        let eq5_e827_d_n5: f64 = (-eq5_e825);
        let eq5_e828: f64 = (eq5_e822 + eq5_e827);
        let eq5_e828_d_n5: f64 = (eq5_e822_d_n5 + eq5_e827_d_n5);
        let eq5_e828_d_n6: f64 = (eq5_e822_d_n6 + eq5_e825);
        (eq5_e828, eq5_e822_d_n0, eq5_e822_d_n1, eq5_e822_d_n2, eq5_e822_d_n3, eq5_e822_d_n4, eq5_e828_d_n5, eq5_e828_d_n6, eq5_e822_d_n7, eq5_e822_d_n8, eq5_e822_d_b0, eq5_e822_d_b1, eq5_e822_d_b2, eq5_e822_d_b3, eq5_e822_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e830;
        let eq5_node_derivatives: [f64; 9] = [eq5_e830_d_n0, eq5_e830_d_n1, eq5_e830_d_n2, eq5_e830_d_n3, eq5_e830_d_n4, eq5_e830_d_n5, eq5_e830_d_n6, eq5_e830_d_n7, eq5_e830_d_n8];
        let eq5_branch_derivatives: [f64; 5] = [eq5_e830_d_b0, eq5_e830_d_b1, eq5_e830_d_b2, eq5_e830_d_b3, eq5_e830_d_b4];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let (eq6_e839, eq6_e839_d_n0, eq6_e839_d_n1, eq6_e839_d_n2, eq6_e839_d_n3, eq6_e839_d_n4, eq6_e839_d_n5, eq6_e839_d_n6, eq6_e839_d_n7, eq6_e839_d_n8, eq6_e839_d_b0, eq6_e839_d_b1, eq6_e839_d_b2, eq6_e839_d_b3, eq6_e839_d_b4,) = {
    if (!s.b[662]) {
        let eq6_e836: f64 = (s.v[199] + s.v[211]);
        let eq6_e836_d_n0: f64 = (s.dn[199][0] + s.dn[211][0]);
        let eq6_e836_d_n1: f64 = (s.dn[199][1] + s.dn[211][1]);
        let eq6_e836_d_n2: f64 = (s.dn[199][2] + s.dn[211][2]);
        let eq6_e836_d_n3: f64 = (s.dn[199][3] + s.dn[211][3]);
        let eq6_e836_d_n4: f64 = (s.dn[199][4] + s.dn[211][4]);
        let eq6_e836_d_n5: f64 = (s.dn[199][5] + s.dn[211][5]);
        let eq6_e836_d_n6: f64 = (s.dn[199][6] + s.dn[211][6]);
        let eq6_e836_d_n7: f64 = (s.dn[199][7] + s.dn[211][7]);
        let eq6_e836_d_n8: f64 = (s.dn[199][8] + s.dn[211][8]);
        let eq6_e836_d_b0: f64 = (s.db[199][0] + s.db[211][0]);
        let eq6_e836_d_b1: f64 = (s.db[199][1] + s.db[211][1]);
        let eq6_e836_d_b2: f64 = (s.db[199][2] + s.db[211][2]);
        let eq6_e836_d_b3: f64 = (s.db[199][3] + s.db[211][3]);
        let eq6_e836_d_b4: f64 = (s.db[199][4] + s.db[211][4]);
        let eq6_e837: f64 = (s.v[212] * eq6_e836);
        let eq6_e837_d_n0: f64 = ((s.dn[212][0] * eq6_e836) + (s.v[212] * eq6_e836_d_n0));
        let eq6_e837_d_n1: f64 = ((s.dn[212][1] * eq6_e836) + (s.v[212] * eq6_e836_d_n1));
        let eq6_e837_d_n2: f64 = ((s.dn[212][2] * eq6_e836) + (s.v[212] * eq6_e836_d_n2));
        let eq6_e837_d_n3: f64 = ((s.dn[212][3] * eq6_e836) + (s.v[212] * eq6_e836_d_n3));
        let eq6_e837_d_n4: f64 = ((s.dn[212][4] * eq6_e836) + (s.v[212] * eq6_e836_d_n4));
        let eq6_e837_d_n5: f64 = ((s.dn[212][5] * eq6_e836) + (s.v[212] * eq6_e836_d_n5));
        let eq6_e837_d_n6: f64 = ((s.dn[212][6] * eq6_e836) + (s.v[212] * eq6_e836_d_n6));
        let eq6_e837_d_n7: f64 = ((s.dn[212][7] * eq6_e836) + (s.v[212] * eq6_e836_d_n7));
        let eq6_e837_d_n8: f64 = ((s.dn[212][8] * eq6_e836) + (s.v[212] * eq6_e836_d_n8));
        let eq6_e837_d_b0: f64 = ((s.db[212][0] * eq6_e836) + (s.v[212] * eq6_e836_d_b0));
        let eq6_e837_d_b1: f64 = ((s.db[212][1] * eq6_e836) + (s.v[212] * eq6_e836_d_b1));
        let eq6_e837_d_b2: f64 = ((s.db[212][2] * eq6_e836) + (s.v[212] * eq6_e836_d_b2));
        let eq6_e837_d_b3: f64 = ((s.db[212][3] * eq6_e836) + (s.v[212] * eq6_e836_d_b3));
        let eq6_e837_d_b4: f64 = ((s.db[212][4] * eq6_e836) + (s.v[212] * eq6_e836_d_b4));
        (eq6_e837, eq6_e837_d_n0, eq6_e837_d_n1, eq6_e837_d_n2, eq6_e837_d_n3, eq6_e837_d_n4, eq6_e837_d_n5, eq6_e837_d_n6, eq6_e837_d_n7, eq6_e837_d_n8, eq6_e837_d_b0, eq6_e837_d_b1, eq6_e837_d_b2, eq6_e837_d_b3, eq6_e837_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e839;
        let eq6_node_derivatives: [f64; 9] = [eq6_e839_d_n0, eq6_e839_d_n1, eq6_e839_d_n2, eq6_e839_d_n3, eq6_e839_d_n4, eq6_e839_d_n5, eq6_e839_d_n6, eq6_e839_d_n7, eq6_e839_d_n8];
        let eq6_branch_derivatives: [f64; 5] = [eq6_e839_d_b0, eq6_e839_d_b1, eq6_e839_d_b2, eq6_e839_d_b3, eq6_e839_d_b4];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let (eq7_e846, eq7_e846_d_n0, eq7_e846_d_n1, eq7_e846_d_n2, eq7_e846_d_n3, eq7_e846_d_n4, eq7_e846_d_n5, eq7_e846_d_n6, eq7_e846_d_n7, eq7_e846_d_n8, eq7_e846_d_b0, eq7_e846_d_b1, eq7_e846_d_b2, eq7_e846_d_b3, eq7_e846_d_b4,) = {
    if (!s.b[662]) {
        let eq7_e844: f64 = (s.v[212] * s.v[198]);
        let eq7_e844_d_n0: f64 = ((s.dn[212][0] * s.v[198]) + (s.v[212] * s.dn[198][0]));
        let eq7_e844_d_n1: f64 = ((s.dn[212][1] * s.v[198]) + (s.v[212] * s.dn[198][1]));
        let eq7_e844_d_n2: f64 = ((s.dn[212][2] * s.v[198]) + (s.v[212] * s.dn[198][2]));
        let eq7_e844_d_n3: f64 = ((s.dn[212][3] * s.v[198]) + (s.v[212] * s.dn[198][3]));
        let eq7_e844_d_n4: f64 = ((s.dn[212][4] * s.v[198]) + (s.v[212] * s.dn[198][4]));
        let eq7_e844_d_n5: f64 = ((s.dn[212][5] * s.v[198]) + (s.v[212] * s.dn[198][5]));
        let eq7_e844_d_n6: f64 = ((s.dn[212][6] * s.v[198]) + (s.v[212] * s.dn[198][6]));
        let eq7_e844_d_n7: f64 = ((s.dn[212][7] * s.v[198]) + (s.v[212] * s.dn[198][7]));
        let eq7_e844_d_n8: f64 = ((s.dn[212][8] * s.v[198]) + (s.v[212] * s.dn[198][8]));
        let eq7_e844_d_b0: f64 = ((s.db[212][0] * s.v[198]) + (s.v[212] * s.db[198][0]));
        let eq7_e844_d_b1: f64 = ((s.db[212][1] * s.v[198]) + (s.v[212] * s.db[198][1]));
        let eq7_e844_d_b2: f64 = ((s.db[212][2] * s.v[198]) + (s.v[212] * s.db[198][2]));
        let eq7_e844_d_b3: f64 = ((s.db[212][3] * s.v[198]) + (s.v[212] * s.db[198][3]));
        let eq7_e844_d_b4: f64 = ((s.db[212][4] * s.v[198]) + (s.v[212] * s.db[198][4]));
        (eq7_e844, eq7_e844_d_n0, eq7_e844_d_n1, eq7_e844_d_n2, eq7_e844_d_n3, eq7_e844_d_n4, eq7_e844_d_n5, eq7_e844_d_n6, eq7_e844_d_n7, eq7_e844_d_n8, eq7_e844_d_b0, eq7_e844_d_b1, eq7_e844_d_b2, eq7_e844_d_b3, eq7_e844_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e846;
        let eq7_node_derivatives: [f64; 9] = [eq7_e846_d_n0, eq7_e846_d_n1, eq7_e846_d_n2, eq7_e846_d_n3, eq7_e846_d_n4, eq7_e846_d_n5, eq7_e846_d_n6, eq7_e846_d_n7, eq7_e846_d_n8];
        let eq7_branch_derivatives: [f64; 5] = [eq7_e846_d_b0, eq7_e846_d_b1, eq7_e846_d_b2, eq7_e846_d_b3, eq7_e846_d_b4];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let (eq8_e855, eq8_e855_d_n0, eq8_e855_d_n1, eq8_e855_d_n2, eq8_e855_d_n3, eq8_e855_d_n4, eq8_e855_d_n5, eq8_e855_d_n6, eq8_e855_d_n7, eq8_e855_d_n8, eq8_e855_d_b0, eq8_e855_d_b1, eq8_e855_d_b2, eq8_e855_d_b3, eq8_e855_d_b4,) = {
    if (!s.b[662]) {
        let eq8_e852: f64 = (s.v[193] + s.v[201]);
        let eq8_e852_d_n0: f64 = (s.dn[193][0] + s.dn[201][0]);
        let eq8_e852_d_n1: f64 = (s.dn[193][1] + s.dn[201][1]);
        let eq8_e852_d_n2: f64 = (s.dn[193][2] + s.dn[201][2]);
        let eq8_e852_d_n3: f64 = (s.dn[193][3] + s.dn[201][3]);
        let eq8_e852_d_n4: f64 = (s.dn[193][4] + s.dn[201][4]);
        let eq8_e852_d_n5: f64 = (s.dn[193][5] + s.dn[201][5]);
        let eq8_e852_d_n6: f64 = (s.dn[193][6] + s.dn[201][6]);
        let eq8_e852_d_n7: f64 = (s.dn[193][7] + s.dn[201][7]);
        let eq8_e852_d_n8: f64 = (s.dn[193][8] + s.dn[201][8]);
        let eq8_e852_d_b0: f64 = (s.db[193][0] + s.db[201][0]);
        let eq8_e852_d_b1: f64 = (s.db[193][1] + s.db[201][1]);
        let eq8_e852_d_b2: f64 = (s.db[193][2] + s.db[201][2]);
        let eq8_e852_d_b3: f64 = (s.db[193][3] + s.db[201][3]);
        let eq8_e852_d_b4: f64 = (s.db[193][4] + s.db[201][4]);
        let eq8_e853: f64 = (s.v[212] * eq8_e852);
        let eq8_e853_d_n0: f64 = ((s.dn[212][0] * eq8_e852) + (s.v[212] * eq8_e852_d_n0));
        let eq8_e853_d_n1: f64 = ((s.dn[212][1] * eq8_e852) + (s.v[212] * eq8_e852_d_n1));
        let eq8_e853_d_n2: f64 = ((s.dn[212][2] * eq8_e852) + (s.v[212] * eq8_e852_d_n2));
        let eq8_e853_d_n3: f64 = ((s.dn[212][3] * eq8_e852) + (s.v[212] * eq8_e852_d_n3));
        let eq8_e853_d_n4: f64 = ((s.dn[212][4] * eq8_e852) + (s.v[212] * eq8_e852_d_n4));
        let eq8_e853_d_n5: f64 = ((s.dn[212][5] * eq8_e852) + (s.v[212] * eq8_e852_d_n5));
        let eq8_e853_d_n6: f64 = ((s.dn[212][6] * eq8_e852) + (s.v[212] * eq8_e852_d_n6));
        let eq8_e853_d_n7: f64 = ((s.dn[212][7] * eq8_e852) + (s.v[212] * eq8_e852_d_n7));
        let eq8_e853_d_n8: f64 = ((s.dn[212][8] * eq8_e852) + (s.v[212] * eq8_e852_d_n8));
        let eq8_e853_d_b0: f64 = ((s.db[212][0] * eq8_e852) + (s.v[212] * eq8_e852_d_b0));
        let eq8_e853_d_b1: f64 = ((s.db[212][1] * eq8_e852) + (s.v[212] * eq8_e852_d_b1));
        let eq8_e853_d_b2: f64 = ((s.db[212][2] * eq8_e852) + (s.v[212] * eq8_e852_d_b2));
        let eq8_e853_d_b3: f64 = ((s.db[212][3] * eq8_e852) + (s.v[212] * eq8_e852_d_b3));
        let eq8_e853_d_b4: f64 = ((s.db[212][4] * eq8_e852) + (s.v[212] * eq8_e852_d_b4));
        (eq8_e853, eq8_e853_d_n0, eq8_e853_d_n1, eq8_e853_d_n2, eq8_e853_d_n3, eq8_e853_d_n4, eq8_e853_d_n5, eq8_e853_d_n6, eq8_e853_d_n7, eq8_e853_d_n8, eq8_e853_d_b0, eq8_e853_d_b1, eq8_e853_d_b2, eq8_e853_d_b3, eq8_e853_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e855;
        let eq8_node_derivatives: [f64; 9] = [eq8_e855_d_n0, eq8_e855_d_n1, eq8_e855_d_n2, eq8_e855_d_n3, eq8_e855_d_n4, eq8_e855_d_n5, eq8_e855_d_n6, eq8_e855_d_n7, eq8_e855_d_n8];
        let eq8_branch_derivatives: [f64; 5] = [eq8_e855_d_b0, eq8_e855_d_b1, eq8_e855_d_b2, eq8_e855_d_b3, eq8_e855_d_b4];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let (eq9_e864, eq9_e864_d_n0, eq9_e864_d_n1, eq9_e864_d_n2, eq9_e864_d_n3, eq9_e864_d_n4, eq9_e864_d_n5, eq9_e864_d_n6, eq9_e864_d_n7, eq9_e864_d_n8, eq9_e864_d_b0, eq9_e864_d_b1, eq9_e864_d_b2, eq9_e864_d_b3, eq9_e864_d_b4,) = {
    if (!s.b[662]) {
        let eq9_e861: f64 = (s.v[194] + s.v[202]);
        let eq9_e861_d_n0: f64 = (s.dn[194][0] + s.dn[202][0]);
        let eq9_e861_d_n1: f64 = (s.dn[194][1] + s.dn[202][1]);
        let eq9_e861_d_n2: f64 = (s.dn[194][2] + s.dn[202][2]);
        let eq9_e861_d_n3: f64 = (s.dn[194][3] + s.dn[202][3]);
        let eq9_e861_d_n4: f64 = (s.dn[194][4] + s.dn[202][4]);
        let eq9_e861_d_n5: f64 = (s.dn[194][5] + s.dn[202][5]);
        let eq9_e861_d_n6: f64 = (s.dn[194][6] + s.dn[202][6]);
        let eq9_e861_d_n7: f64 = (s.dn[194][7] + s.dn[202][7]);
        let eq9_e861_d_n8: f64 = (s.dn[194][8] + s.dn[202][8]);
        let eq9_e861_d_b0: f64 = (s.db[194][0] + s.db[202][0]);
        let eq9_e861_d_b1: f64 = (s.db[194][1] + s.db[202][1]);
        let eq9_e861_d_b2: f64 = (s.db[194][2] + s.db[202][2]);
        let eq9_e861_d_b3: f64 = (s.db[194][3] + s.db[202][3]);
        let eq9_e861_d_b4: f64 = (s.db[194][4] + s.db[202][4]);
        let eq9_e862: f64 = (s.v[212] * eq9_e861);
        let eq9_e862_d_n0: f64 = ((s.dn[212][0] * eq9_e861) + (s.v[212] * eq9_e861_d_n0));
        let eq9_e862_d_n1: f64 = ((s.dn[212][1] * eq9_e861) + (s.v[212] * eq9_e861_d_n1));
        let eq9_e862_d_n2: f64 = ((s.dn[212][2] * eq9_e861) + (s.v[212] * eq9_e861_d_n2));
        let eq9_e862_d_n3: f64 = ((s.dn[212][3] * eq9_e861) + (s.v[212] * eq9_e861_d_n3));
        let eq9_e862_d_n4: f64 = ((s.dn[212][4] * eq9_e861) + (s.v[212] * eq9_e861_d_n4));
        let eq9_e862_d_n5: f64 = ((s.dn[212][5] * eq9_e861) + (s.v[212] * eq9_e861_d_n5));
        let eq9_e862_d_n6: f64 = ((s.dn[212][6] * eq9_e861) + (s.v[212] * eq9_e861_d_n6));
        let eq9_e862_d_n7: f64 = ((s.dn[212][7] * eq9_e861) + (s.v[212] * eq9_e861_d_n7));
        let eq9_e862_d_n8: f64 = ((s.dn[212][8] * eq9_e861) + (s.v[212] * eq9_e861_d_n8));
        let eq9_e862_d_b0: f64 = ((s.db[212][0] * eq9_e861) + (s.v[212] * eq9_e861_d_b0));
        let eq9_e862_d_b1: f64 = ((s.db[212][1] * eq9_e861) + (s.v[212] * eq9_e861_d_b1));
        let eq9_e862_d_b2: f64 = ((s.db[212][2] * eq9_e861) + (s.v[212] * eq9_e861_d_b2));
        let eq9_e862_d_b3: f64 = ((s.db[212][3] * eq9_e861) + (s.v[212] * eq9_e861_d_b3));
        let eq9_e862_d_b4: f64 = ((s.db[212][4] * eq9_e861) + (s.v[212] * eq9_e861_d_b4));
        (eq9_e862, eq9_e862_d_n0, eq9_e862_d_n1, eq9_e862_d_n2, eq9_e862_d_n3, eq9_e862_d_n4, eq9_e862_d_n5, eq9_e862_d_n6, eq9_e862_d_n7, eq9_e862_d_n8, eq9_e862_d_b0, eq9_e862_d_b1, eq9_e862_d_b2, eq9_e862_d_b3, eq9_e862_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e864;
        let eq9_node_derivatives: [f64; 9] = [eq9_e864_d_n0, eq9_e864_d_n1, eq9_e864_d_n2, eq9_e864_d_n3, eq9_e864_d_n4, eq9_e864_d_n5, eq9_e864_d_n6, eq9_e864_d_n7, eq9_e864_d_n8];
        let eq9_branch_derivatives: [f64; 5] = [eq9_e864_d_b0, eq9_e864_d_b1, eq9_e864_d_b2, eq9_e864_d_b3, eq9_e864_d_b4];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let eq10_e867: f64 = (s.v[212] * s.v[187]);
        let eq10_e867_d_n0: f64 = ((s.dn[212][0] * s.v[187]) + (s.v[212] * s.dn[187][0]));
        let eq10_e867_d_n1: f64 = ((s.dn[212][1] * s.v[187]) + (s.v[212] * s.dn[187][1]));
        let eq10_e867_d_n2: f64 = ((s.dn[212][2] * s.v[187]) + (s.v[212] * s.dn[187][2]));
        let eq10_e867_d_n3: f64 = ((s.dn[212][3] * s.v[187]) + (s.v[212] * s.dn[187][3]));
        let eq10_e867_d_n4: f64 = ((s.dn[212][4] * s.v[187]) + (s.v[212] * s.dn[187][4]));
        let eq10_e867_d_n5: f64 = ((s.dn[212][5] * s.v[187]) + (s.v[212] * s.dn[187][5]));
        let eq10_e867_d_n6: f64 = ((s.dn[212][6] * s.v[187]) + (s.v[212] * s.dn[187][6]));
        let eq10_e867_d_n7: f64 = ((s.dn[212][7] * s.v[187]) + (s.v[212] * s.dn[187][7]));
        let eq10_e867_d_n8: f64 = ((s.dn[212][8] * s.v[187]) + (s.v[212] * s.dn[187][8]));
        let eq10_e867_d_b0: f64 = ((s.db[212][0] * s.v[187]) + (s.v[212] * s.db[187][0]));
        let eq10_e867_d_b1: f64 = ((s.db[212][1] * s.v[187]) + (s.v[212] * s.db[187][1]));
        let eq10_e867_d_b2: f64 = ((s.db[212][2] * s.v[187]) + (s.v[212] * s.db[187][2]));
        let eq10_e867_d_b3: f64 = ((s.db[212][3] * s.v[187]) + (s.v[212] * s.db[187][3]));
        let eq10_e867_d_b4: f64 = ((s.db[212][4] * s.v[187]) + (s.v[212] * s.db[187][4]));
        let eq10_value: f64 = eq10_e867;
        let eq10_node_derivatives: [f64; 9] = [eq10_e867_d_n0, eq10_e867_d_n1, eq10_e867_d_n2, eq10_e867_d_n3, eq10_e867_d_n4, eq10_e867_d_n5, eq10_e867_d_n6, eq10_e867_d_n7, eq10_e867_d_n8];
        let eq10_branch_derivatives: [f64; 5] = [eq10_e867_d_b0, eq10_e867_d_b1, eq10_e867_d_b2, eq10_e867_d_b3, eq10_e867_d_b4];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e870: f64 = (s.v[212] * s.v[188]);
        let eq11_e870_d_n0: f64 = ((s.dn[212][0] * s.v[188]) + (s.v[212] * s.dn[188][0]));
        let eq11_e870_d_n1: f64 = ((s.dn[212][1] * s.v[188]) + (s.v[212] * s.dn[188][1]));
        let eq11_e870_d_n2: f64 = ((s.dn[212][2] * s.v[188]) + (s.v[212] * s.dn[188][2]));
        let eq11_e870_d_n3: f64 = ((s.dn[212][3] * s.v[188]) + (s.v[212] * s.dn[188][3]));
        let eq11_e870_d_n4: f64 = ((s.dn[212][4] * s.v[188]) + (s.v[212] * s.dn[188][4]));
        let eq11_e870_d_n5: f64 = ((s.dn[212][5] * s.v[188]) + (s.v[212] * s.dn[188][5]));
        let eq11_e870_d_n6: f64 = ((s.dn[212][6] * s.v[188]) + (s.v[212] * s.dn[188][6]));
        let eq11_e870_d_n7: f64 = ((s.dn[212][7] * s.v[188]) + (s.v[212] * s.dn[188][7]));
        let eq11_e870_d_n8: f64 = ((s.dn[212][8] * s.v[188]) + (s.v[212] * s.dn[188][8]));
        let eq11_e870_d_b0: f64 = ((s.db[212][0] * s.v[188]) + (s.v[212] * s.db[188][0]));
        let eq11_e870_d_b1: f64 = ((s.db[212][1] * s.v[188]) + (s.v[212] * s.db[188][1]));
        let eq11_e870_d_b2: f64 = ((s.db[212][2] * s.v[188]) + (s.v[212] * s.db[188][2]));
        let eq11_e870_d_b3: f64 = ((s.db[212][3] * s.v[188]) + (s.v[212] * s.db[188][3]));
        let eq11_e870_d_b4: f64 = ((s.db[212][4] * s.v[188]) + (s.v[212] * s.db[188][4]));
        let eq11_value: f64 = eq11_e870;
        let eq11_node_derivatives: [f64; 9] = [eq11_e870_d_n0, eq11_e870_d_n1, eq11_e870_d_n2, eq11_e870_d_n3, eq11_e870_d_n4, eq11_e870_d_n5, eq11_e870_d_n6, eq11_e870_d_n7, eq11_e870_d_n8];
        let eq11_branch_derivatives: [f64; 5] = [eq11_e870_d_b0, eq11_e870_d_b1, eq11_e870_d_b2, eq11_e870_d_b3, eq11_e870_d_b4];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
    }
}
