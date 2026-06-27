#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.v[45] = 0.0;

        s.v[40] = 0.0;

        s.v[210] = 0.0;

        s.v[254] = 0.0;

        s.v[295] = 0.0;

        s.v[316] = 0.0;

        s.v[373] = 0.0;

        s.v[478] = 0.0;

        s.v[1021] = 0.0;

        s.v[839] = 0.0;

        s.v[717] = 0.0;

        s.v[691] = 0.0;

        s.v[779] = 0.0;

        s.v[749] = 0.0;

        s.v[756] = 0.0;

        s.v[754] = 0.0;

        s.v[692] = 0.0;

        s.v[916] = 0.0;

        s.v[928] = 0.0;

        s.v[829] = 0.0;

        s.v[833] = 0.0;

        s.v[841] = 0.0;

        s.v[845] = 0.0;

        s.v[849] = 0.0;

        s.v[853] = 0.0;

        s.v[859] = 0.0;

        s.v[863] = 0.0;

        s.v[1018] = 0.0;

        s.v[731] = 0.0;

        s.v[784] = 0.0;

        s.v[658] = 0.0;

        s.v[644] = 0.0;

        s.v[650] = 0.0;

        s.v[745] = 0.0;

        s.v[936] = 0.0;

        s.v[917] = 0.0;

        s.v[830] = 0.0;

        s.v[836] = 0.0;

        s.v[842] = 0.0;

        s.v[846] = 0.0;

        s.v[850] = 0.0;

        s.v[856] = 0.0;

        s.v[860] = 0.0;

        s.v[864] = 0.0;

        s.v[664] = 0.0;

        s.v[762] = 0.0;

        s.v[739] = 0.0;

        s.v[759] = 0.0;

        s.v[753] = 0.0;

        s.v[654] = 0.0;

        s.v[937] = 0.0;

        s.v[956] = 0.0;

        s.v[958] = 0.0;

        s.v[831] = 0.0;

        s.v[837] = 0.0;

        s.v[843] = 0.0;

        s.v[847] = 0.0;

        s.v[851] = 0.0;

        s.v[857] = 0.0;

        s.v[861] = 0.0;

        s.v[685] = 0.0;

        s.v[347] = 0.0;

        s.v[642] = 0.0;

        s.v[646] = 0.0;

        s.v[648] = 0.0;

        s.v[686] = 0.0;

        s.v[938] = 0.0;

        s.v[957] = 0.0;

        s.v[828] = 0.0;

        s.v[832] = 0.0;

        s.v[840] = 0.0;

        s.v[844] = 0.0;

        s.v[848] = 0.0;

        s.v[852] = 0.0;

        s.v[858] = 0.0;

        s.v[862] = 0.0;

        s.v[854] = 0.0;

        s.v[855] = 0.0;

        s.v[460] = 0.0;

        s.v[459] = 0.0;

        s.v[462] = 0.0;

        s.v[461] = 0.0;

        s.v[1019] = 1.0;

        s.v[1020] = 1.0;

        s.v[618] = 0.0;

        s.v[617] = 0.0;

        s.v[87] = 1.0;

        s.v[354] = 0.0;

        s.v[339] = 0.0;

        s.v[458] = 0.0;

        s.v[343] = 0.0;

        s.v[344] = 0.0;

        s.v[534] = 0.0;

        s.v[533] = 0.0;

        s.v[376] = 0.0;

        s.v[834] = 0.0;

        s.v[363] = 0.0;

        s.v[365] = 0.0;

        s.v[334] = 0.0;

        s.v[455] = 0.0;

        s.v[454] = 0.0;

        s.v[315] = 0.0;

        s.v[355] = 0.0;

        s.v[250] = 0.0;

        s.v[243] = 0.0;

        s.v[73] = 0.0;

        s.v[81] = 0.0;

        s.v[457] = 0.0;

        s.v[1048] = (1.3806503e-23 / 1.602176462e-19);

        s.v[320] = 0.0;

        s.v[400] = 0.0;

        s.v[23] = 0.0;

        s.v[22] = 0.0;

        s.v[323] = 0.0;

        s.v[74] = 0.0;

        s.v[80] = 0.0;

        s.v[84] = 0.0;

        s.v[959] = 0.0;

        s.v[960] = 0.0;

        s.v[961] = 0.0;

        s.b[1129] = (p.p30 == 1.0);
        s.v[1129] = if s.b[1129] { 1.0 } else { 0.0 };

        if s.b[1129] {
            s.store_scalar(379, 1.0);
        }

        if (!s.b[1129]) {
            s.store_scalar(379, (-1.0));
        }

        s.v[180] = (p.p109 * 8.8541878128e-12);

        s.v[181] = (p.p110 * 8.8541878128e-12);

        s.v[199] = ((p.p110 * 8.8541878128e-12) / p.p76);

        s.v[200] = (p.p109 / p.p110);

        s.b[1130] = (!param_given[77]);
        s.v[1130] = if s.b[1130] { 1.0 } else { 0.0 };

        if s.b[1130] {
            s.store_scalar(429, (((p.p76 * p.p110) / 3.9) - p.p78));
        }

        if (!s.b[1130]) {
            s.store_scalar(429, p.p77);
        }

        s.v[262] = (p.p0 * p.p49);

        s.v[264] = (p.p1 * p.p50);

        s.v[261] = (s.v[262] + p.p51);

        s.v[681] = (s.v[264] / p.p2);

        s.v[263] = (s.v[681] + p.p53);

        s.v[682] = ((s.v[261]) as f64).powf((-p.p58));

        s.v[683] = ((s.v[263]) as f64).powf((-p.p59));

        s.v[684] = (s.v[682] * s.v[683]);

        s.v[192] = (((p.p54 + (p.p55 * s.v[682])) + (p.p56 * s.v[683])) + (p.p57 * s.v[684]));

        s.v[688] = ((s.v[261]) as f64).powf((-p.p64));

        s.v[689] = ((s.v[263]) as f64).powf((-p.p65));

        s.v[690] = (s.v[688] * s.v[689]);

        s.v[193] = (((p.p60 + (p.p61 * s.v[688])) + (p.p62 * s.v[689])) + (p.p63 * s.v[690]));

        s.v[184] = (s.v[261] - (2.0 * s.v[192]));

        s.v[183] = ((s.v[263] - (p.p1375 * p.p1376)) - ((2.0 - p.p1375) * s.v[193]));

        s.v[196] = (((p.p66 + (p.p67 * s.v[682])) + (p.p68 * s.v[683])) + (p.p69 * s.v[684]));

        s.v[197] = (((p.p70 + (p.p71 * s.v[688])) + (p.p72 * s.v[689])) + (p.p73 * s.v[690]));

        s.v[188] = (s.v[261] - (2.0 * s.v[196]));

        s.v[187] = ((s.v[263] - (p.p1375 * p.p1376)) - ((2.0 - p.p1375) * s.v[197]));

        s.v[198] = (((p.p927 + (p.p71 / ((s.v[261]) as f64).powf(p.p64))) + (p.p72 / ((s.v[263]) as f64).powf(p.p65))) + ((p.p73 / ((s.v[261]) as f64).powf(p.p64)) / ((s.v[263]) as f64).powf(p.p65)));

        s.v[189] = (s.v[263] - (2.0 * s.v[198]));

        s.v[694] = (1e-6 / s.v[184]);

        s.v[695] = (1e-6 / s.v[183]);

        s.v[697] = (1e-6 / s.v[188]);

        s.v[698] = (1e-6 / s.v[187]);

        s.v[699] = (1e-6 / p.p48);

        s.v[700] = (1e-6 / p.p52);

        s.v[696] = (s.v[694] * s.v[695]);

        s.v[685] = s.v[682];

        s.v[691] = s.v[688];

        s.b[1142] = (p.p1026 != 0.0);
        s.v[1142] = if s.b[1142] { 1.0 } else { 0.0 };

        s.b[1143] = (p.p1026 <= (-s.v[261]));
        s.v[1143] = if s.b[1143] { 1.0 } else { 0.0 };

        if (s.b[1142] && (!s.b[1143])) {
            s.store_scalar(685, (((s.v[261] + p.p1026)) as f64).powf((-p.p58)));
            s.store_scalar(691, (((s.v[261] + p.p1026)) as f64).powf((-p.p64)));
        }

        s.v[686] = s.v[683];

        s.v[692] = s.v[689];

        s.b[1144] = (p.p1027 != 0.0);
        s.v[1144] = if s.b[1144] { 1.0 } else { 0.0 };

        s.b[1145] = (p.p1027 <= (-s.v[263]));
        s.v[1145] = if s.b[1145] { 1.0 } else { 0.0 };

        if (s.b[1144] && (!s.b[1145])) {
            s.store_scalar(686, (((s.v[263] + p.p1027)) as f64).powf((-p.p59)));
            s.store_scalar(692, (((s.v[263] + p.p1027)) as f64).powf((-p.p65)));
        }

        s.store_mul(687, 685, 686);

        s.store_add_scaled_inputs3_offset_indices(194, 685, p.p55, 686, p.p56, 687, p.p57, p.p54);

        s.store_mul(693, 691, 692);

        s.store_add_scaled_inputs3_offset_indices(195, 691, p.p61, 692, p.p62, 693, p.p63, p.p60);

        s.store_offset_sub_from_scalar_ad(186, s.v[261], A::scale(s.ad_value(194), 2.0), p.p1026);

        s.store_offset_sub_from_scalar_ad(185, s.v[263], A::scale(s.ad_value(195), 2.0), p.p1027);

        s.b[1148] = (p.p1025 == 1.0);
        s.v[1148] = if s.b[1148] { 1.0 } else { 0.0 };

        if s.b[1148] {
            s.store_div_from_scalar(701, 1e-6, 186);
            s.store_div_from_scalar(702, 1e-6, 185);
        }

        if (!s.b[1148]) {
            s.store_div_from_scalar(701, 1.0, 186);
            s.store_div_from_scalar(702, 1.0, 185);
        }

        s.store_mul(703, 701, 702);

        s.store_add_scaled_inputs3_offset_indices(707, 701, p.p116, 702, p.p117, 703, p.p118, p.p115);

        s.store_add_scaled_inputs3_offset_indices(708, 701, p.p120, 702, p.p121, 703, p.p122, p.p119);

        s.store_add_scaled_inputs3_offset_indices(793, 701, p.p130, 702, p.p131, 703, p.p132, p.p129);

        s.store_add_scaled_inputs3_offset_indices(705, 701, p.p143, 702, p.p144, 703, p.p145, p.p142);

        s.store_add_scaled_inputs3_offset_indices(706, 701, p.p88, 702, p.p89, 703, p.p90, p.p79);

        s.store_add_scaled_inputs3_offset_indices(794, 701, p.p100, 702, p.p101, 703, p.p102, p.p91);

        s.store_add_scaled_inputs3_offset_indices(704, 701, p.p104, 702, p.p105, 703, p.p106, p.p103);

        s.store_add_scaled_inputs3_offset_indices(709, 701, p.p233, 702, p.p234, 703, p.p235, p.p232);

        s.store_add_scaled_inputs3_offset_indices(720, 701, p.p243, 702, p.p244, 703, p.p245, p.p236);

        s.store_add_scaled_inputs3_offset_indices(721, 701, p.p247, 702, p.p248, 703, p.p249, p.p246);

        s.store_add_scaled_inputs3_offset_indices(722, 701, p.p251, 702, p.p252, 703, p.p253, p.p250);

        s.store_add_scaled_inputs3_offset_indices(725, 701, p.p171, 702, p.p172, 703, p.p173, p.p170);

        s.store_add_scaled_inputs3_offset_indices(726, 701, p.p175, 702, p.p176, 703, p.p177, p.p174);

        s.store_add_scaled_inputs3_offset_indices(724, 701, p.p179, 702, p.p180, 703, p.p181, p.p178);

        s.store_add_scaled_inputs3_offset_indices(728, 701, p.p187, 702, p.p188, 703, p.p189, p.p186);

        s.store_add_scaled_inputs3_offset_indices(727, 701, p.p183, 702, p.p184, 703, p.p185, p.p182);

        s.store_add_scaled_inputs3_offset_indices(723, 701, p.p255, 702, p.p256, 703, p.p257, p.p254);

        s.store_add_scaled_inputs3_offset_indices(710, 701, p.p259, 702, p.p260, 703, p.p261, p.p258);

        s.store_add_scaled_inputs3_offset_indices(714, 701, p.p263, 702, p.p264, 703, p.p265, p.p262);

        s.store_add_scaled_inputs3_offset_indices(715, 701, p.p1165, 702, p.p1166, 703, p.p1167, p.p1164);

        s.store_add_scaled_inputs3_offset_indices(716, 701, p.p1192, 702, p.p1193, 703, p.p1194, p.p1191);

        s.store_add_scaled_inputs3_offset_indices(719, 701, p.p291, 702, p.p292, 703, p.p293, p.p288);

        s.store_add_scaled_inputs3_offset_indices(711, 701, p.p271, 702, p.p272, 703, p.p273, p.p270);

        s.store_add_scaled_inputs3_offset_indices(712, 701, p.p1177, 702, p.p1178, 703, p.p1179, p.p1176);

        s.store_add_scaled_inputs3_offset_indices(713, 701, p.p276, 702, p.p277, 703, p.p278, p.p275);

        s.store_add_scaled_inputs3_offset_indices(279, 701, p.p147, 702, p.p148, 703, p.p149, p.p146);

        s.store_add_scaled_inputs3_offset_indices(280, 701, p.p1239, 702, p.p1240, 703, p.p1241, p.p1238);

        s.store_add_scaled_inputs3_offset_indices(281, 701, p.p151, 702, p.p152, 703, p.p153, p.p150);

        s.store_add_scaled_inputs3_offset_indices(282, 701, p.p1243, 702, p.p1244, 703, p.p1245, p.p1242);

        s.store_add_scaled_inputs3_offset_indices(283, 701, p.p155, 702, p.p156, 703, p.p157, p.p154);

        s.store_add_scaled_inputs3_offset_indices(285, 701, p.p159, 702, p.p160, 703, p.p161, p.p158);

        s.store_add_scaled_inputs3_offset_indices(287, 701, p.p163, 702, p.p164, 703, p.p165, p.p162);

        s.store_add_scaled_inputs3_offset_indices(289, 701, p.p167, 702, p.p168, 703, p.p169, p.p166);

        s.store_add_scaled_inputs3_offset_indices(284, 701, p.p1247, 702, p.p1248, 703, p.p1249, p.p1246);

        s.store_add_scaled_inputs3_offset_indices(286, 701, p.p1251, 702, p.p1252, 703, p.p1253, p.p1250);

        s.store_add_scaled_inputs3_offset_indices(288, 701, p.p1255, 702, p.p1256, 703, p.p1257, p.p1254);

        s.store_add_scaled_inputs3_offset_indices(290, 701, p.p1259, 702, p.p1260, 703, p.p1261, p.p1258);

        s.store_add_scaled_inputs3_offset_indices(734, 701, p.p225, 702, p.p226, 703, p.p227, p.p218);

        s.store_add_scaled_inputs3_offset_indices(735, 701, p.p215, 702, p.p216, 703, p.p217, p.p208);

        s.store_add_scaled_inputs3_offset_indices(736, 701, p.p1203, 702, p.p1204, 703, p.p1205, p.p1196);

        s.store_add_scaled_inputs3_offset_indices(782, 701, p.p112, 702, p.p113, 703, p.p114, p.p111);

        s.store_add_scaled_inputs3_offset_indices(729, 701, p.p191, 702, p.p192, 703, p.p193, p.p190);

        s.store_add_scaled_inputs3_offset_indices(730, 701, p.p195, 702, p.p196, 703, p.p197, p.p194);

        s.store_add_scaled_inputs3_offset_indices(733, 701, p.p205, 702, p.p206, 703, p.p207, p.p203);

        s.store_add_scaled_inputs3_offset_indices(737, 701, p.p310, 702, p.p311, 703, p.p312, p.p309);

        s.store_add_scaled_inputs3_offset_indices(738, 701, p.p340, 702, p.p341, 703, p.p342, p.p337);

        s.store_add_scaled_inputs3_offset_indices(748, 701, p.p355, 702, p.p356, 703, p.p357, p.p348);

        s.store_add_scaled_inputs3_offset_indices(752, 701, p.p375, 702, p.p376, 703, p.p377, p.p372);

        s.store_add_scaled_inputs3_offset_indices(751, 701, p.p363, 702, p.p364, 703, p.p365, p.p362);

        s.store_add_scaled_inputs3_offset_indices(755, 701, p.p383, 702, p.p384, 703, p.p385, p.p382);

        s.store_add_scaled_inputs3_offset_indices(758, 701, p.p397, 702, p.p398, 703, p.p399, p.p390);

        s.store_add_scaled_inputs3_offset_indices(783, 701, p.p407, 702, p.p408, 703, p.p409, p.p404);

        s.store_add_scaled_inputs3_offset_indices(786, 701, p.p418, 702, p.p419, 703, p.p420, p.p415);

        s.store_add_scaled_inputs3_offset_indices(775, 701, p.p458, 702, p.p459, 703, p.p460, p.p457);

        s.store_add_scaled_inputs3_offset_indices(774, 701, p.p468, 702, p.p469, 703, p.p470, p.p467);

        s.store_add_scaled_inputs3_offset_indices(770, 701, p.p440, 702, p.p441, 703, p.p442, p.p439);

        s.store_add_scaled_inputs3_offset_indices(787, 701, p.p444, 702, p.p445, 703, p.p446, p.p443);

        s.store_add_scaled_inputs3_offset_indices(771, 701, p.p450, 702, p.p451, 703, p.p452, p.p449);

    }

    pub(super) fn stamp_transient_block_1(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_add_scaled_inputs3_offset_indices(773, 701, p.p454, 702, p.p455, 703, p.p456, p.p453);

        s.store_add_scaled_inputs3_offset_indices(772, 701, p.p464, 702, p.p465, 703, p.p466, p.p463);

        s.store_add_scaled_inputs3_offset_indices(776, 701, p.p480, 702, p.p481, 703, p.p482, p.p477);

        s.store_add_scaled_inputs3_offset_indices(777, 701, p.p474, 702, p.p475, 703, p.p476, p.p473);

        s.store_add_scaled_inputs3_offset_indices(778, 701, p.p499, 702, p.p500, 703, p.p501, p.p498);

        s.store_add_scaled_inputs3_offset_indices(761, 701, p.p533, 702, p.p534, 703, p.p535, p.p530);

        s.store_add_scaled_inputs3_offset_indices(764, 701, p.p541, 702, p.p542, 703, p.p543, p.p540);

        s.store_add_scaled_inputs3_offset_indices(765, 701, p.p422, 702, p.p423, 703, p.p424, p.p421);

        s.store_add_scaled_inputs3_offset_indices(766, 701, p.p426, 702, p.p427, 703, p.p428, p.p425);

        s.store_add_scaled_inputs3_offset_indices(767, 701, p.p430, 702, p.p431, 703, p.p432, p.p429);

        s.store_add_scaled_inputs3_offset_indices(768, 701, p.p435, 702, p.p436, 703, p.p437, p.p434);

        s.store_add_scaled_inputs3_offset_indices(769, 701, p.p551, 702, p.p552, 703, p.p553, p.p548);

        s.store_add_scaled_inputs3_offset_indices(781, 701, p.p545, 702, p.p546, 703, p.p547, p.p544);

        s.store_add_scaled_inputs3_offset_indices(741, 701, p.p296, 702, p.p297, 703, p.p298, p.p295);

        s.store_add_scaled_inputs3_offset_indices(742, 701, p.p511, 702, p.p512, 703, p.p513, p.p510);

        s.store_add_scaled_inputs3_offset_indices(744, 701, p.p326, 702, p.p327, 703, p.p328, p.p325);

        s.store_add_scaled_inputs3_offset_indices(743, 701, p.p330, 702, p.p331, 703, p.p332, p.p329);

        s.store_add_scaled_inputs3_offset_indices(346, 701, p.p484, 702, p.p485, 703, p.p486, p.p483);

        s.store_add_scaled_inputs3_offset_indices(747, 701, p.p316, 702, p.p317, 703, p.p318, p.p315);

        s.store_add_scaled_inputs3_offset_indices(788, 701, p.p868, 702, p.p869, 703, p.p870, p.p867);

        s.store_add_scaled_inputs3_offset_indices(789, 701, p.p876, 702, p.p877, 703, p.p878, p.p875);

        s.store_add_scaled_inputs3_offset_indices(790, 701, p.p880, 702, p.p881, 703, p.p882, p.p879);

        s.store_add_scaled_inputs3_offset_indices(791, 701, p.p884, 702, p.p885, 703, p.p886, p.p883);

        s.store_add_scaled_inputs3_offset_indices(792, 701, p.p888, 702, p.p889, 703, p.p890, p.p887);

        s.store_add_scaled_inputs3_offset_indices(865, 701, p.p604, 702, p.p605, 703, p.p606, p.p601);

        s.store_add_scaled_inputs3_offset_indices(866, 701, p.p608, 702, p.p609, 703, p.p610, p.p607);

        s.store_add_scaled_inputs3_offset_indices(867, 701, p.p612, 702, p.p613, 703, p.p614, p.p611);

        s.store_add_scaled_inputs3_offset_indices(868, 701, p.p616, 702, p.p617, 703, p.p618, p.p615);

        s.store_add_scaled_inputs3_offset_indices(869, 701, p.p620, 702, p.p621, 703, p.p622, p.p619);

        s.store_add_scaled_inputs3_offset_indices(870, 701, p.p624, 702, p.p625, 703, p.p626, p.p623);

        s.store_add_scaled_inputs3_offset_indices(871, 701, p.p628, 702, p.p629, 703, p.p630, p.p627);

        s.store_add_scaled_inputs3_offset_indices(872, 701, p.p632, 702, p.p633, 703, p.p634, p.p631);

        s.store_add_scaled_inputs3_offset_indices(873, 701, p.p636, 702, p.p637, 703, p.p638, p.p635);

        s.store_add_scaled_inputs3_offset_indices(874, 701, p.p597, 702, p.p598, 703, p.p599, p.p596);

        s.store_add_scaled_inputs3_offset_indices(875, 701, p.p640, 702, p.p641, 703, p.p642, p.p639);

        s.store_add_scaled_inputs3_offset_indices(876, 701, p.p647, 702, p.p648, 703, p.p649, p.p646);

        s.store_add_scaled_inputs3_offset_indices(877, 701, p.p655, 702, p.p658, 703, p.p661, p.p650);

        s.store_add_scaled_inputs3_offset_indices(878, 701, p.p654, 702, p.p657, 703, p.p660, p.p651);

        s.store_add_scaled_inputs3_offset_indices(879, 701, p.p653, 702, p.p656, 703, p.p659, p.p652);

        s.store_add_scaled_inputs3_offset_indices(880, 701, p.p663, 702, p.p664, 703, p.p665, p.p662);

        s.store_add_scaled_inputs3_offset_indices(881, 701, p.p668, 702, p.p669, 703, p.p670, p.p667);

        s.store_add_scaled_inputs3_offset_indices(1028, 701, p.p1362, 702, p.p1363, 703, p.p1364, p.p1361);

        s.store_add_scaled_inputs3_offset_indices(1029, 701, p.p1366, 702, p.p1367, 703, p.p1368, p.p1365);

        s.store_add_scaled_inputs3_offset_indices(1030, 701, p.p1370, 702, p.p1371, 703, p.p1372, p.p1369);

        s.store_add_scaled_inputs3_offset_indices(547, 701, p.p929, 702, p.p930, 703, p.p931, p.p928);

        s.store_add_scaled_inputs3_offset_indices(550, 701, p.p934, 702, p.p936, 703, p.p938, p.p932);

        s.store_add_scaled_inputs3_offset_indices(551, 701, p.p935, 702, p.p937, 703, p.p939, p.p933);

        s.store_add_scaled_inputs3_offset_indices(557, 701, p.p941, 702, p.p942, 703, p.p943, p.p940);

        s.store_add_scaled_inputs3_offset_indices(564, 701, p.p945, 702, p.p946, 703, p.p947, p.p944);

        s.store_add_scaled_inputs3_offset_indices(556, 701, p.p949, 702, p.p950, 703, p.p951, p.p948);

        s.store_add_scaled_inputs3_offset_indices(552, 701, p.p954, 702, p.p956, 703, p.p958, p.p952);

        s.store_add_scaled_inputs3_offset_indices(553, 701, p.p955, 702, p.p957, 703, p.p959, p.p953);

        s.store_add_scaled_inputs3_offset_indices(565, 701, p.p962, 702, p.p964, 703, p.p966, p.p960);

        s.store_add_scaled_inputs3_offset_indices(566, 701, p.p963, 702, p.p965, 703, p.p967, p.p961);

        s.store_add_scaled_inputs3_offset_indices(567, 701, p.p970, 702, p.p972, 703, p.p974, p.p968);

        s.store_add_scaled_inputs3_offset_indices(568, 701, p.p971, 702, p.p973, 703, p.p975, p.p969);

        s.store_add_scaled_inputs3_offset_indices(569, 701, p.p978, 702, p.p980, 703, p.p982, p.p976);

        s.store_add_scaled_inputs3_offset_indices(570, 701, p.p979, 702, p.p981, 703, p.p983, p.p977);

        s.store_add_scaled_inputs3_offset_indices(573, 701, p.p986, 702, p.p988, 703, p.p990, p.p984);

        s.store_add_scaled_inputs3_offset_indices(574, 701, p.p987, 702, p.p989, 703, p.p991, p.p985);

        s.store_add_scaled_inputs3_offset_indices(575, 701, p.p994, 702, p.p996, 703, p.p998, p.p992);

        s.store_add_scaled_inputs3_offset_indices(576, 701, p.p995, 702, p.p997, 703, p.p999, p.p993);

        s.store_add_scaled_inputs3_offset_indices(558, 701, p.p1002, 702, p.p1004, 703, p.p1006, p.p1000);

        s.store_add_scaled_inputs3_offset_indices(559, 701, p.p1003, 702, p.p1005, 703, p.p1007, p.p1001);

        s.store_add_scaled_inputs3_offset_indices(581, 701, p.p556, 702, p.p557, 703, p.p558, p.p555);

        s.store_add_scaled_inputs3_offset_indices(582, 701, p.p560, 702, p.p561, 703, p.p562, p.p559);

        s.store_add_scaled_inputs3_offset_indices(587, 701, p.p565, 702, p.p567, 703, p.p569, p.p563);

        s.store_add_scaled_inputs3_offset_indices(588, 701, p.p566, 702, p.p568, 703, p.p570, p.p564);

        s.store_add_scaled_inputs3_offset_indices(589, 701, p.p572, 702, p.p573, 703, p.p574, p.p571);

        s.store_add_scaled_inputs3_offset_indices(590, 701, p.p576, 702, p.p577, 703, p.p578, p.p575);

        s.store_add_scaled_inputs3_offset_indices(598, 701, p.p582, 702, p.p581, 703, p.p580, p.p579);

        s.store_add_scaled_inputs3_offset_indices(597, 701, p.p584, 702, p.p585, 703, p.p586, p.p583);

        s.store_add_scaled_inputs3_offset_indices(600, 701, p.p588, 702, p.p590, 703, p.p592, p.p587);

        s.store_add_scaled_inputs3_offset_indices(601, 701, p.p589, 702, p.p591, 703, p.p593, p.p594);

        s.store_add_scaled_inputs3_offset_indices(530, 701, p.p922, 702, p.p923, 703, p.p924, p.p921);

        s.store_add_scaled_inputs3_offset_indices(806, 701, p.p1126, 702, p.p1127, 703, p.p1128, p.p1125);

        s.store_add_scaled_inputs3_offset_indices(807, 701, p.p1130, 702, p.p1131, 703, p.p1132, p.p1129);

        s.store_add_scaled_inputs3_offset_indices(808, 701, p.p1134, 702, p.p1135, 703, p.p1136, p.p1133);

        s.store_add_scaled_inputs3_offset_indices(892, 701, p.p802, 702, p.p803, 703, p.p804, p.p799);

        s.store_add_scaled_inputs3_offset_indices(893, 701, p.p807, 702, p.p808, 703, p.p809, p.p805);

        s.store_add_scaled_inputs3_offset_indices(900, 701, p.p810, 702, p.p811, 703, p.p812, p.p806);

        s.store_add_scaled_inputs3_offset_indices(894, 701, p.p814, 702, p.p815, 703, p.p816, p.p813);

        s.store_add_scaled_inputs3_offset_indices(895, 701, p.p818, 702, p.p819, 703, p.p820, p.p817);

        s.store_add_scaled_inputs3_offset_indices(896, 701, p.p824, 702, p.p825, 703, p.p826, p.p821);

        s.store_add_scaled_inputs3_offset_indices(897, 701, p.p829, 702, p.p830, 703, p.p831, p.p827);

        s.store_add_scaled_inputs3_offset_indices(901, 701, p.p832, 702, p.p833, 703, p.p834, p.p828);

        s.store_add_scaled_inputs3_offset_indices(898, 701, p.p836, 702, p.p837, 703, p.p838, p.p835);

        s.store_add_scaled_inputs3_offset_indices(899, 701, p.p840, 702, p.p841, 703, p.p842, p.p839);

        s.store_add_scaled_inputs3_offset_indices(905, 701, p.p856, 702, p.p857, 703, p.p858, p.p855);

        s.store_add_scaled_inputs3_offset_indices(902, 701, p.p844, 702, p.p845, 703, p.p846, p.p843);

        s.store_add_scaled_inputs3_offset_indices(906, 701, p.p860, 702, p.p861, 703, p.p862, p.p859);

        s.store_add_scaled_inputs3_offset_indices(903, 701, p.p848, 702, p.p849, 703, p.p850, p.p847);

        s.store_add_scaled_inputs3_offset_indices(907, 701, p.p864, 702, p.p865, 703, p.p866, p.p863);

        s.store_add_scaled_inputs3_offset_indices(904, 701, p.p852, 702, p.p853, 703, p.p854, p.p851);

        s.store_add_scaled_inputs3_offset_indices(796, 701, p.p1033, 702, p.p1034, 703, p.p1035, p.p1032);

        s.store_add_scaled_inputs3_offset_indices(797, 701, p.p1038, 702, p.p1039, 703, p.p1040, p.p1037);

        s.store_add_scaled_inputs3_offset_indices(798, 701, p.p1043, 702, p.p1044, 703, p.p1045, p.p1042);

        s.store_add_scaled_inputs3_offset_indices(799, 701, p.p1047, 702, p.p1048, 703, p.p1049, p.p1046);

        s.store_add_scaled_inputs3_offset_indices(805, 701, p.p1052, 702, p.p1053, 703, p.p1054, p.p1051);

        s.store_add_scaled_inputs3_offset_indices(800, 701, p.p1056, 702, p.p1057, 703, p.p1058, p.p1055);

        s.store_add_scaled_inputs3_offset_indices(801, 701, p.p1061, 702, p.p1062, 703, p.p1063, p.p1060);

        s.store_add_scaled_inputs3_offset_indices(802, 701, p.p1065, 702, p.p1066, 703, p.p1067, p.p1064);

        s.store_add_scaled_inputs3_offset_indices(803, 701, p.p1071, 702, p.p1072, 703, p.p1073, p.p1070);

        s.store_add_scaled_inputs3_offset_indices(804, 701, p.p1086, 702, p.p1087, 703, p.p1088, p.p1085);

        s.store_add_scaled_inputs3_offset_indices(827, 701, p.p1090, 702, p.p1091, 703, p.p1092, p.p1089);

        s.store_add_scaled_inputs3_offset_indices(809, 701, p.p732, 702, p.p733, 703, p.p734, p.p706);

        s.store_add_scaled_inputs3_offset_indices(882, 701, p.p685, 702, p.p686, 703, p.p687, p.p684);

        s.store_add_scaled_inputs3_offset_indices(887, 701, p.p689, 702, p.p690, 703, p.p691, p.p688);

        s.store_add_scaled_inputs3_offset_indices(883, 701, p.p693, 702, p.p694, 703, p.p695, p.p692);

        s.store_add_scaled_inputs3_offset_indices(884, 701, p.p673, 702, p.p674, 703, p.p675, p.p672);

        s.store_add_scaled_inputs3_offset_indices(886, 701, p.p677, 702, p.p678, 703, p.p679, p.p676);

        s.store_add_scaled_inputs3_offset_indices(885, 701, p.p681, 702, p.p682, 703, p.p683, p.p680);

        s.store_add_scaled_inputs3_offset_indices(810, 701, p.p735, 702, p.p737, 703, p.p739, p.p707);

        s.store_add_scaled_inputs3_offset_indices(813, 701, p.p736, 702, p.p738, 703, p.p740, p.p726);

        s.store_add_scaled_inputs3_offset_indices(811, 701, p.p741, 702, p.p742, 703, p.p743, p.p708);

        s.store_add_scaled_inputs3_offset_indices(812, 701, p.p744, 702, p.p745, 703, p.p746, p.p709);

        s.store_add_scaled_inputs3_offset_indices(816, 701, p.p747, 702, p.p749, 703, p.p751, p.p710);

        s.store_add_scaled_inputs3_offset_indices(814, 701, p.p748, 702, p.p750, 703, p.p752, p.p711);

        s.store_add_scaled_inputs3_offset_indices(817, 701, p.p753, 702, p.p754, 703, p.p755, p.p712);

        s.store_add_scaled_inputs3_offset_indices(818, 701, p.p756, 702, p.p757, 703, p.p758, p.p713);

        s.store_add_scaled_inputs3_offset_indices(819, 701, p.p759, 702, p.p761, 703, p.p763, p.p714);

        s.store_add_scaled_inputs3_offset_indices(815, 701, p.p760, 702, p.p762, 703, p.p764, p.p715);

        s.store_add_scaled_inputs3_offset_indices(820, 701, p.p765, 702, p.p766, 703, p.p767, p.p716);

        s.store_add_scaled_inputs3_offset_indices(821, 701, p.p768, 702, p.p769, 703, p.p770, p.p717);

        s.store_add_scaled_inputs3_offset_indices(822, 701, p.p771, 702, p.p772, 703, p.p773, p.p720);

        s.store_add_scaled_inputs3_offset_indices(824, 701, p.p774, 702, p.p775, 703, p.p776, p.p718);

        s.store_add_scaled_inputs3_offset_indices(825, 701, p.p777, 702, p.p778, 703, p.p779, p.p719);

        s.store_add_scaled_inputs3_offset_indices(826, 701, p.p780, 702, p.p781, 703, p.p782, p.p721);

        s.store_add_scaled_inputs3_offset_indices(679, 701, p.p1078, 702, p.p1079, 703, p.p1080, p.p1075);

        s.store_add_scaled_inputs3_offset_indices(680, 701, p.p1082, 702, p.p1083, 703, p.p1084, p.p1081);

        s.store_add_scaled_inputs3_offset_indices(678, 701, p.p494, 702, p.p495, 703, p.p496, p.p489);

        s.store_add_scaled_inputs3_offset_indices(328, 701, p.p515, 702, p.p516, 703, p.p517, p.p514);

        s.store_add_scaled_inputs3_offset_indices(329, 701, p.p519, 702, p.p520, 703, p.p521, p.p518);

        s.store_add_scaled_inputs3_offset_indices(331, 701, p.p523, 702, p.p524, 703, p.p525, p.p522);

        s.store_add_scaled_inputs3_offset_indices(332, 701, p.p527, 702, p.p528, 703, p.p529, p.p526);

        s.store_add_scaled_inputs3_offset_indices(828, 701, p.p1301, 702, p.p1302, 703, p.p1303, p.p1300);

        s.store_add_scaled_inputs3_offset_indices(829, 701, p.p1309, 702, p.p1310, 703, p.p1311, p.p1308);

        s.store_add_scaled_inputs3_offset_indices(830, 701, p.p1305, 702, p.p1306, 703, p.p1307, p.p1304);

        s.store_add_scaled_inputs3_offset_indices(831, 701, p.p1313, 702, p.p1314, 703, p.p1315, p.p1312);

        s.store_add_scaled_inputs3_offset_indices(835, 701, p.p1157, 702, p.p1158, 703, p.p1159, p.p1156);

        s.store_add_scaled_inputs3_offset_indices(953, 701, p.p1153, 702, p.p1154, 703, p.p1155, p.p1152);

        s.store_add_scaled_inputs3_offset_indices(836, 701, p.p1161, 702, p.p1162, 703, p.p1163, p.p1160);

        s.store_add_scaled_inputs3_offset_indices(837, 701, p.p1169, 702, p.p1170, 703, p.p1171, p.p1168);

        s.store_add_scaled_inputs3_offset_indices(840, 701, p.p1187, 702, p.p1188, 703, p.p1189, p.p1186);

        s.store_add_scaled_inputs3_offset_indices(841, 701, p.p1207, 702, p.p1208, 703, p.p1209, p.p1206);

        s.store_add_scaled_inputs3_offset_indices(842, 701, p.p1211, 702, p.p1212, 703, p.p1213, p.p1210);

        s.store_add_scaled_inputs3_offset_indices(843, 701, p.p1215, 702, p.p1216, 703, p.p1217, p.p1214);

        s.store_add_scaled_inputs3_offset_indices(844, 701, p.p1219, 702, p.p1220, 703, p.p1221, p.p1218);

        s.store_add_scaled_inputs3_offset_indices(845, 701, p.p1223, 702, p.p1224, 703, p.p1225, p.p1222);

        s.store_add_scaled_inputs3_offset_indices(846, 701, p.p1227, 702, p.p1228, 703, p.p1229, p.p1226);

        s.store_add_scaled_inputs3_offset_indices(847, 701, p.p1231, 702, p.p1232, 703, p.p1233, p.p1230);

        s.store_add_scaled_inputs3_offset_indices(848, 701, p.p1235, 702, p.p1236, 703, p.p1237, p.p1234);

        s.store_add_scaled_inputs3_offset_indices(849, 701, p.p1272, 702, p.p1273, 703, p.p1274, p.p1265);

        s.store_add_scaled_inputs3_offset_indices(850, 701, p.p1276, 702, p.p1277, 703, p.p1278, p.p1275);

        s.store_add_scaled_inputs3_offset_indices(854, 701, p.p1284, 702, p.p1285, 703, p.p1286, p.p1283);

        s.store_add_scaled_inputs3_offset_indices(855, 701, p.p1280, 702, p.p1281, 703, p.p1282, p.p1279);

        s.store_add_scaled_inputs3_offset_indices(851, 701, p.p1288, 702, p.p1289, 703, p.p1290, p.p1287);

        s.store_add_scaled_inputs3_offset_indices(852, 701, p.p1292, 702, p.p1293, 703, p.p1294, p.p1291);

        s.store_add_scaled_inputs3_offset_indices(856, 701, p.p1324, 702, p.p1325, 703, p.p1326, p.p1323);

        s.store_add_scaled_inputs3_offset_indices(857, 701, p.p1328, 702, p.p1329, 703, p.p1330, p.p1327);

        s.store_add_scaled_inputs3_offset_indices(859, 701, p.p1332, 702, p.p1333, 703, p.p1334, p.p1331);

        s.store_add_scaled_inputs3_offset_indices(860, 701, p.p1336, 702, p.p1337, 703, p.p1338, p.p1335);

        s.store_add_scaled_inputs3_offset_indices(862, 701, p.p1340, 702, p.p1341, 703, p.p1342, p.p1339);

        s.store_add_scaled_inputs3_offset_indices(863, 701, p.p1344, 702, p.p1345, 703, p.p1346, p.p1343);

        s.store_add_scaled_inputs3_offset_indices(888, 701, p.p787, 702, p.p791, 703, p.p795, p.p783);

        s.store_add_scaled_inputs3_offset_indices(891, 701, p.p788, 702, p.p792, 703, p.p796, p.p784);

        s.store_add_scaled_inputs3_offset_indices(889, 701, p.p789, 702, p.p793, 703, p.p797, p.p785);

        s.store_add_scaled_inputs3_offset_indices(890, 701, p.p790, 702, p.p794, 703, p.p798, p.p786);

        s.store_add_scaled_inputs3_offset_indices(908, 701, p.p1385, 702, p.p1386, 703, p.p1387, p.p1384);

        s.store_add_scaled_inputs3_offset_indices(909, 701, p.p1390, 702, p.p1391, 703, p.p1392, p.p1389);

        s.b[1149] = (p.p35 != 0.0);
        s.v[1149] = if s.b[1149] { 1.0 } else { 0.0 };

        if s.b[1149] {
            s.store_add_scaled_inputs3_offset_indices(839, 701, p.p1173, 702, p.p1174, 703, p.p1175, p.p1172);
            s.store_add_scaled_inputs3_offset_indices(717, 701, p.p285, 702, p.p286, 703, p.p287, p.p284);
            s.store_add_scaled_inputs3_offset_indices(731, 701, p.p199, 702, p.p200, 703, p.p201, p.p198);
            s.store_add_scaled_inputs3_offset_indices(739, 701, p.p344, 702, p.p345, 703, p.p346, p.p343);
            s.store_add_scaled_inputs3_offset_indices(749, 701, p.p359, 702, p.p360, 703, p.p361, p.p358);
            s.store_add_scaled_inputs3_offset_indices(753, 701, p.p379, 702, p.p380, 703, p.p381, p.p378);
            s.store_add_scaled_inputs3_offset_indices(756, 701, p.p387, 702, p.p388, 703, p.p389, p.p386);
            s.store_add_scaled_inputs3_offset_indices(759, 701, p.p401, 702, p.p402, 703, p.p403, p.p400);
            s.store_add_scaled_inputs3_offset_indices(784, 701, p.p411, 702, p.p412, 703, p.p413, p.p410);
            s.store_add_scaled_inputs3_offset_indices(762, 701, p.p537, 702, p.p538, 703, p.p539, p.p536);
            s.store_add_scaled_inputs3_offset_indices(745, 701, p.p306, 702, p.p307, 703, p.p308, p.p305);
            s.store_add_scaled_inputs3_offset_indices(347, 701, p.p491, 702, p.p492, 703, p.p493, p.p490);
            s.store_add_scaled_inputs3_offset_indices(779, 701, p.p507, 702, p.p508, 703, p.p509, p.p506);
        }

        s.v[167] = ((p.p80 * ((((s.v[694]) as f64).powf(p.p81) - ((s.v[699]) as f64).powf(p.p81))).max(0.0)) + (p.p82 * ((((s.v[694]) as f64).powf(p.p83) - ((s.v[699]) as f64).powf(p.p83))).max(0.0)));

        s.v[168] = ((p.p84 * ((((s.v[695]) as f64).powf(p.p85) - ((s.v[700]) as f64).powf(p.p85))).max(0.0)) + (p.p86 * (((s.v[695] * s.v[694])) as f64).powf(p.p87)));

        s.store_scale(706, 706, ((1.0 + s.v[167]) + s.v[168]));

        s.v[167] = (p.p237 * ((((s.v[694]) as f64).powf(p.p238) - ((s.v[699]) as f64).powf(p.p238))).max(0.0));

        s.v[168] = ((p.p239 * ((((s.v[695]) as f64).powf(p.p240) - ((s.v[700]) as f64).powf(p.p240))).max(0.0)) + (p.p241 * ((s.v[696]) as f64).powf(p.p242)));

        s.store_scale(720, 720, ((1.0 + s.v[167]) + s.v[168]));

        s.v[167] = (1.0 + (p.p282 * ((((s.v[694]) as f64).powf(p.p283) - ((s.v[699]) as f64).powf(p.p283))).max(0.0)));

        s.store_scale(710, 710, s.v[167]);

        s.b[1150] = (p.p35 != 0.0);
        s.v[1150] = if s.b[1150] { 1.0 } else { 0.0 };

        if s.b[1150] {
            s.store_scale(839, 839, s.v[167]);
            s.store_scale(717, 717, s.v[167]);
        }

        s.store_scale(719, 719, (1.0 + (p.p289 * ((((s.v[694]) as f64).powf(p.p290) - ((s.v[699]) as f64).powf(p.p290))).max(0.0))));

        s.store_scale(738, 738, p.p24);

        s.b[1151] = (p.p42 != 1.0);
        s.v[1151] = if s.b[1151] { 1.0 } else { 0.0 };

        s.b[1152] = (p.p339 > 0.0);
        s.v[1152] = if s.b[1152] { 1.0 } else { 0.0 };

        if (s.b[1151] && s.b[1152]) {
            s.store_scale(738, 738, (1.0 - (p.p338 * ((((s.v[694]) as f64).powf(p.p339) - ((s.v[699]) as f64).powf(p.p339))).max(0.0))));
        }

        s.b[1153] = (p.p35 != 0.0);
        s.v[1153] = if s.b[1153] { 1.0 } else { 0.0 };

        if ((s.b[1151] && s.b[1152]) && s.b[1153]) {
            s.store_scale(739, 739, (1.0 - (p.p338 * ((((s.v[694]) as f64).powf(p.p339) - ((s.v[699]) as f64).powf(p.p339))).max(0.0))));
        }

        if (s.b[1151] && (!s.b[1152])) {
            s.store_scale(738, 738, (1.0 - p.p338));
        }

        s.b[1154] = (p.p35 != 0.0);
        s.v[1154] = if s.b[1154] { 1.0 } else { 0.0 };

        if ((s.b[1151] && (!s.b[1152])) && s.b[1154]) {
            s.store_scale(739, 739, (1.0 - p.p338));
        }

        if (!s.b[1151]) {
            s.store_scale(738, 738, ((1.0 - (p.p333 * { let limited_exp_arg = ((-s.v[184]) / p.p334); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })) - (p.p335 * { let limited_exp_arg = ((-s.v[184]) / p.p336); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })));
        }

        s.b[1155] = (p.p35 != 0.0);
        s.v[1155] = if s.b[1155] { 1.0 } else { 0.0 };

        if ((!s.b[1151]) && s.b[1155]) {
            s.store_scale(739, 739, ((1.0 - (p.p333 * { let limited_exp_arg = ((-s.v[184]) / p.p334); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })) - (p.p335 * { let limited_exp_arg = ((-s.v[184]) / p.p336); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })));
        }

        s.v[167] = (p.p349 * ((((s.v[694]) as f64).powf(p.p350) - ((s.v[699]) as f64).powf(p.p350))).max(0.0));

        s.v[168] = ((p.p351 * ((((s.v[695]) as f64).powf(p.p352) - ((s.v[700]) as f64).powf(p.p352))).max(0.0)) + (p.p353 * ((s.v[696]) as f64).powf(p.p354)));

        s.store_scale(748, 748, ((1.0 + s.v[167]) + s.v[168]));

        s.b[1156] = (p.p35 != 0.0);
        s.v[1156] = if s.b[1156] { 1.0 } else { 0.0 };

        if s.b[1156] {
            s.store_scale(749, 749, ((1.0 + s.v[167]) + s.v[168]));
        }

        s.v[167] = (p.p366 * ((((s.v[694]) as f64).powf(p.p367) - ((s.v[699]) as f64).powf(p.p367))).max(0.0));

        s.v[168] = ((p.p368 * ((((s.v[695]) as f64).powf(p.p369) - ((s.v[700]) as f64).powf(p.p369))).max(0.0)) + (p.p370 * ((s.v[696]) as f64).powf(p.p371)));

        s.store_scale(751, 751, ((1.0 + s.v[167]) + s.v[168]));

        s.v[167] = (1.0 + (p.p373 * ((((s.v[694]) as f64).powf(p.p374) - ((s.v[699]) as f64).powf(p.p374))).max(0.0)));

        s.store_scale(752, 752, s.v[167]);

        s.b[1157] = (p.p35 != 0.0);
        s.v[1157] = if s.b[1157] { 1.0 } else { 0.0 };

        if s.b[1157] {
            s.store_scale(753, 753, s.v[167]);
        }

        s.v[167] = (p.p391 * ((((s.v[694]) as f64).powf(p.p392) - ((s.v[699]) as f64).powf(p.p392))).max(0.0));

        s.v[168] = ((p.p393 * ((((s.v[695]) as f64).powf(p.p394) - ((s.v[700]) as f64).powf(p.p394))).max(0.0)) + (p.p395 * ((s.v[696]) as f64).powf(p.p396)));

        s.store_scale(758, 758, ((1.0 + s.v[167]) + s.v[168]));

        s.b[1158] = (p.p35 != 0.0);
        s.v[1158] = if s.b[1158] { 1.0 } else { 0.0 };

        if s.b[1158] {
            s.store_scale(759, 759, ((1.0 + s.v[167]) + s.v[168]));
        }

        s.v[167] = ((((s.v[694]) as f64).powf(p.p202) - ((s.v[699]) as f64).powf(p.p202))).max(0.0);

    }

    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_scale(730, 730, s.v[167]);

        s.b[1159] = (p.p35 != 0.0);
        s.v[1159] = if s.b[1159] { 1.0 } else { 0.0 };

        if s.b[1159] {
            s.store_scale(731, 731, s.v[167]);
        }

        s.store_scale(733, 733, ((((s.v[694]) as f64).powf(p.p204) - ((s.v[699]) as f64).powf(p.p204))).max(0.0));

        s.v[167] = (1.0 + (p.p531 * ((((s.v[694]) as f64).powf(p.p532) - ((s.v[699]) as f64).powf(p.p532))).max(0.0)));

        s.store_scale(761, 761, s.v[167]);

        s.b[1160] = (p.p35 != 0.0);
        s.v[1160] = if s.b[1160] { 1.0 } else { 0.0 };

        if s.b[1160] {
            s.store_scale(762, 762, s.v[167]);
        }

        s.store_scale(167, 737, (1.0 + (p.p313 * ((((s.v[694]) as f64).powf(p.p314) - ((s.v[699]) as f64).powf(p.p314))).max(0.0))));

        s.store_min_with_scalar(737, 167, 0.5);

        s.store_scale(769, 769, (1.0 + (p.p549 * ((((s.v[694]) as f64).powf(p.p550) - ((s.v[699]) as f64).powf(p.p550))).max(0.0))));

        s.v[167] = (1.0 + (p.p405 * ((((s.v[694]) as f64).powf(p.p406) - ((s.v[699]) as f64).powf(p.p406))).max(0.0)));

        s.store_scale(783, 783, s.v[167]);

        s.store_max_with_scalar(783, 783, 0.0);

        s.b[1161] = (p.p35 != 0.0);
        s.v[1161] = if s.b[1161] { 1.0 } else { 0.0 };

        if s.b[1161] {
            s.store_scale(784, 784, s.v[167]);
            s.store_max_with_scalar(784, 784, 0.0);
        }

        s.v[167] = (p.p299 * ((((s.v[694]) as f64).powf(p.p300) - ((s.v[699]) as f64).powf(p.p300))).max(0.0));

        s.v[168] = ((p.p301 * ((((s.v[695]) as f64).powf(p.p302) - ((s.v[700]) as f64).powf(p.p302))).max(0.0)) + (p.p303 * ((s.v[696]) as f64).powf(p.p304)));

        s.store_scale(741, 741, ((1.0 + s.v[167]) + s.v[168]));

        s.b[1162] = (p.p35 != 0.0);
        s.v[1162] = if s.b[1162] { 1.0 } else { 0.0 };

        if s.b[1162] {
            s.store_scale(745, 745, ((1.0 + s.v[167]) + s.v[168]));
        }

        s.store_max_with_scalar_ad(346, A::scale(s.ad_value(346), (1.0 + (p.p487 * ((((s.v[694]) as f64).powf(p.p488) - ((s.v[699]) as f64).powf(p.p488))).max(0.0)))), 0.25);

        s.b[1163] = (p.p35 != 0.0);
        s.v[1163] = if s.b[1163] { 1.0 } else { 0.0 };

        if s.b[1163] {
            s.store_max_with_scalar_ad(347, A::scale(s.ad_value(347), (1.0 + (p.p487 * ((((s.v[694]) as f64).powf(p.p488) - ((s.v[699]) as f64).powf(p.p488))).max(0.0)))), 0.25);
        }

        s.v[167] = (1.0 + (p.p502 * ((((s.v[694]) as f64).powf(p.p505) - ((s.v[699]) as f64).powf(p.p505))).max(0.0)));

        s.store_scale(778, 778, s.v[167]);

        s.b[1164] = (p.p35 != 0.0);
        s.v[1164] = if s.b[1164] { 1.0 } else { 0.0 };

        if s.b[1164] {
            s.store_scale(779, 779, s.v[167]);
        }

        s.store_scale(865, 865, (1.0 + (p.p602 * ((((s.v[694]) as f64).powf(p.p603) - ((s.v[699]) as f64).powf(p.p603))).max(0.0))));

        s.store_scale(892, 892, ((1.0 + (p.p800 * s.v[694])) + (p.p801 * s.v[695])));

        s.store_scale(896, 896, ((1.0 + (p.p822 * s.v[694])) + (p.p823 * s.v[695])));

        s.store_scale(810, 810, ((1.0 + (p.p724 * s.v[694])) + (p.p725 * s.v[695])));

        s.store_scale(816, 816, ((1.0 + (p.p727 * s.v[694])) + (p.p728 * s.v[695])));

        s.store_scale(819, 819, ((1.0 + (p.p729 * s.v[694])) + (p.p730 * s.v[695])));

        s.v[823] = (p.p723 * (1.0 + (p.p731 * s.v[694])));

        s.v[167] = ((p.p92 * ((((s.v[697]) as f64).powf(p.p93) - ((s.v[699]) as f64).powf(p.p93))).max(0.0)) + (p.p94 * ((((s.v[697]) as f64).powf(p.p95) - ((s.v[699]) as f64).powf(p.p95))).max(0.0)));

        s.v[168] = ((p.p96 * ((((s.v[698]) as f64).powf(p.p97) - ((s.v[700]) as f64).powf(p.p97))).max(0.0)) + (p.p98 * (((s.v[698] * s.v[697])) as f64).powf(p.p99)));

        s.store_scale(794, 794, ((1.0 + s.v[167]) + s.v[168]));

        s.b[1165] = (p.p29 == 1.0);
        s.v[1165] = if s.b[1165] { 1.0 } else { 0.0 };

        if s.b[1165] {
            s.copy_ad(794, 706);
        }

        if (!s.b[1165]) {
        }

        s.v[167] = (p.p123 * ((((s.v[694]) as f64).powf(p.p124) - ((s.v[699]) as f64).powf(p.p124))).max(0.0));

        s.v[168] = ((p.p125 * ((((s.v[695]) as f64).powf(p.p126) - ((s.v[700]) as f64).powf(p.p126))).max(0.0)) + (p.p127 * ((s.v[696]) as f64).powf(p.p128)));

        s.store_scale(707, 707, ((1.0 + s.v[167]) + s.v[168]));

        s.v[167] = (p.p133 * ((((s.v[697]) as f64).powf(p.p134) - ((s.v[699]) as f64).powf(p.p134))).max(0.0));

        s.v[168] = ((p.p135 * ((((s.v[698]) as f64).powf(p.p136) - ((s.v[700]) as f64).powf(p.p136))).max(0.0)) + (p.p137 * (((s.v[698] * s.v[697])) as f64).powf(p.p138)));

        s.store_scale(793, 793, ((1.0 + s.v[167]) + s.v[168]));

        s.v[167] = (p.p319 * ((((s.v[697]) as f64).powf(p.p320) - ((s.v[699]) as f64).powf(p.p320))).max(0.0));

        s.v[168] = ((p.p321 * ((((s.v[698]) as f64).powf(p.p322) - ((s.v[700]) as f64).powf(p.p322))).max(0.0)) + (p.p323 * (((s.v[698] * s.v[697])) as f64).powf(p.p324)));

        s.store_scale(747, 747, ((1.0 + s.v[167]) + s.v[168]));

        s.store_scale(786, 786, (1.0 + (p.p416 * ((((s.v[697]) as f64).powf(p.p417) - ((s.v[699]) as f64).powf(p.p417))).max(0.0))));

        s.store_max_with_scalar(786, 786, 0.0);

        s.v[167] = (p.p209 * ((((s.v[694]) as f64).powf(p.p210) - ((s.v[699]) as f64).powf(p.p210))).max(0.0));

        s.v[168] = ((p.p211 * ((((s.v[695]) as f64).powf(p.p212) - ((s.v[700]) as f64).powf(p.p212))).max(0.0)) + (p.p213 * ((s.v[696]) as f64).powf(p.p214)));

        s.store_scale(735, 735, ((1.0 + s.v[167]) + s.v[168]));

        s.v[167] = (p.p1197 * ((((s.v[694]) as f64).powf(p.p1198) - ((s.v[699]) as f64).powf(p.p1198))).max(0.0));

        s.v[168] = ((p.p1199 * ((((s.v[695]) as f64).powf(p.p1200) - ((s.v[700]) as f64).powf(p.p1200))).max(0.0)) + (p.p1201 * ((s.v[696]) as f64).powf(p.p1202)));

        s.store_scale(736, 736, ((1.0 + s.v[167]) + s.v[168]));

        s.v[167] = (p.p219 * ((((s.v[694]) as f64).powf(p.p220) - ((s.v[699]) as f64).powf(p.p220))).max(0.0));

        s.v[168] = ((p.p221 * ((((s.v[695]) as f64).powf(p.p222) - ((s.v[700]) as f64).powf(p.p222))).max(0.0)) + (p.p223 * ((s.v[696]) as f64).powf(p.p224)));

        s.store_scale(734, 734, ((1.0 + s.v[167]) + s.v[168]));

        s.v[167] = (p.p1266 * ((((s.v[694]) as f64).powf(p.p1267) - ((s.v[699]) as f64).powf(p.p1267))).max(0.0));

        s.v[168] = ((p.p1268 * ((((s.v[695]) as f64).powf(p.p1269) - ((s.v[700]) as f64).powf(p.p1269))).max(0.0)) + (p.p1270 * ((s.v[696]) as f64).powf(p.p1271)));

        s.store_scale(849, 849, ((1.0 + s.v[167]) + s.v[168]));

        s.store_scale(787, 787, (1.0 + (p.p447 * ((((s.v[694]) as f64).powf(p.p448) - ((s.v[699]) as f64).powf(p.p448))).max(0.0))));

        s.store_scale(796, 796, (1.0 + (s.v[694] * p.p1036)));

        s.store_scale(797, 797, (1.0 + (s.v[694] * p.p1041)));

        s.store_scale(799, 799, (1.0 + (s.v[694] * p.p1050)));

        s.store_scale(802, 802, (1.0 + (s.v[694] * p.p1068)));

        s.store_scale(803, 803, (1.0 + (s.v[694] * p.p1074)));

        s.b[1166] = (p.p33 == 1.0);
        s.v[1166] = if s.b[1166] { 1.0 } else { 0.0 };

        if s.b[1166] {
            s.store_scale(775, 775, (1.0 + (p.p461 * ((((s.v[694]) as f64).powf(p.p462) - ((s.v[699]) as f64).powf(p.p462))).max(0.0))));
            s.store_scale(774, 774, (1.0 + (p.p471 * ((((s.v[694]) as f64).powf(p.p472) - ((s.v[699]) as f64).powf(p.p472))).max(0.0))));
        }

        if (!s.b[1166]) {
            s.store_scale(776, 776, (1.0 + (p.p478 * ((((s.v[694]) as f64).powf(p.p479) - ((s.v[699]) as f64).powf(p.p479))).max(0.0))));
        }

        s.b[1167] = (s.v[755] < 1.0);
        s.v[1167] = if s.b[1167] { 1.0 } else { 0.0 };

        if s.b[1167] {
            s.store_scalar(755, 1.0);
        }

        s.b[1168] = (s.v[755] > 2.0);
        s.v[1168] = if s.b[1168] { 1.0 } else { 0.0 };

        if ((!s.b[1167]) && s.b[1168]) {
            s.store_scalar(755, 2.0);
        }

        s.b[1169] = (p.p35 != 0.0);
        s.v[1169] = if s.b[1169] { 1.0 } else { 0.0 };

        s.b[1170] = (s.v[756] < 1.0);
        s.v[1170] = if s.b[1170] { 1.0 } else { 0.0 };

        if (s.b[1169] && s.b[1170]) {
            s.store_scalar(756, 1.0);
        }

        s.b[1171] = (s.v[756] > 2.0);
        s.v[1171] = if s.b[1171] { 1.0 } else { 0.0 };

        if ((s.b[1169] && (!s.b[1170])) && s.b[1171]) {
            s.store_scalar(756, 2.0);
        }

        s.b[1194] = (s.v[824] < 0.0);
        s.v[1194] = if s.b[1194] { 1.0 } else { 0.0 };

        if s.b[1194] {
            s.store_scalar(824, 0.0);
        }

        s.b[1195] = (s.v[825] < 0.0);
        s.v[1195] = if s.b[1195] { 1.0 } else { 0.0 };

        if s.b[1195] {
            s.store_scalar(825, 0.0);
        }

        s.b[1196] = (s.v[829] < 0.0);
        s.v[1196] = if s.b[1196] { 1.0 } else { 0.0 };

        if s.b[1196] {
            s.store_scalar(829, 0.0);
        }

        s.b[1197] = (s.v[738] <= 0.0);
        s.v[1197] = if s.b[1197] { 1.0 } else { 0.0 };

        if s.b[1197] {
            s.store_scalar(738, 0.067);
        }

        s.b[1198] = (s.v[748] < 0.0);
        s.v[1198] = if s.b[1198] { 1.0 } else { 0.0 };

        if s.b[1198] {
            s.store_scalar(748, 0.0);
        }

        s.b[1199] = (s.v[751] < 0.0);
        s.v[1199] = if s.b[1199] { 1.0 } else { 0.0 };

        if s.b[1199] {
            s.store_scalar(751, 0.0);
        }

        s.b[1200] = (s.v[752] < 0.0);
        s.v[1200] = if s.b[1200] { 1.0 } else { 0.0 };

        if s.b[1200] {
            s.store_scalar(752, 0.0);
        }

        s.b[1201] = (s.v[755] < 0.0);
        s.v[1201] = if s.b[1201] { 1.0 } else { 0.0 };

        if s.b[1201] {
            s.store_scalar(755, 0.0);
        }

        s.b[1202] = (s.v[590] <= 0.0);
        s.v[1202] = if s.b[1202] { 1.0 } else { 0.0 };

        if s.b[1202] {
            s.store_scalar(590, 1.0);
        }

        s.b[1203] = (s.v[564] <= 0.0);
        s.v[1203] = if s.b[1203] { 1.0 } else { 0.0 };

        if s.b[1203] {
            s.store_scalar(564, 10.0);
        }

        s.b[1204] = (s.v[557] <= 0.0);
        s.v[1204] = if s.b[1204] { 1.0 } else { 0.0 };

        if s.b[1204] {
            s.store_scalar(557, 2.0);
        }

        s.v[969] = 0.0;

        s.v[971] = 0.0;

        s.v[968] = 0.0;

        s.v[970] = 0.0;

        s.v[973] = 0.0;

        s.v[972] = 0.0;

        s.v[449] = (p.p895 - p.p898);

        s.v[451] = p.p896;

        s.v[450] = (p.p897 - p.p898);

        s.b[1206] = param_given[3];
        s.v[1206] = if s.b[1206] { 1.0 } else { 0.0 };

        if s.b[1206] {
            s.store_scalar(452, (p.p438 * p.p3));
        }

        s.b[1207] = ((p.p9 > 0.0) && (p.p438 > 0.0));
        s.v[1207] = if s.b[1207] { 1.0 } else { 0.0 };

        s.b[1208] = (p.p8 < 9.0);
        s.v[1208] = if s.b[1208] { 1.0 } else { 0.0 };

        s.b[1209] = ((p.p2 % 2.0) != 0.0);
        s.v[1209] = if s.b[1209] { 1.0 } else { 0.0 };

        if ((((!s.b[1206]) && s.b[1207]) && s.b[1208]) && s.b[1209]) {
            s.store_scalar(969, 1.0);
            s.store_scalar(971, 1.0);
            s.store_scalar(968, (2.0 * (((p.p2 - 1.0) / 2.0)).max(0.0)));
            s.copy_ad(970, 968);
        }

        s.b[1210] = (p.p6 == 1.0);
        s.v[1210] = if s.b[1210] { 1.0 } else { 0.0 };

        if (((((!s.b[1206]) && s.b[1207]) && s.b[1208]) && (!s.b[1209])) && s.b[1210]) {
            s.store_scalar(969, 2.0);
            s.store_scalar(968, (2.0 * (((p.p2 / 2.0) - 1.0)).max(0.0)));
            s.store_scalar(971, 0.0);
            s.store_scalar(970, p.p2);
        }

        if (((((!s.b[1206]) && s.b[1207]) && s.b[1208]) && (!s.b[1209])) && (!s.b[1210])) {
            s.store_scalar(969, 0.0);
            s.store_scalar(968, p.p2);
            s.store_scalar(971, 2.0);
            s.store_scalar(970, (2.0 * (((p.p2 / 2.0) - 1.0)).max(0.0)));
        }

        s.b[1211] = (1.0 == 1.0);
        s.v[1211] = if s.b[1211] { 1.0 } else { 0.0 };

        s.b[1212] = (s.v[970] == 0.0);
        s.v[1212] = if s.b[1212] { 1.0 } else { 0.0 };

        if (((((!s.b[1206]) && s.b[1207]) && s.b[1208]) && s.b[1211]) && s.b[1212]) {
            s.store_scalar(972, 0.0);
        }

        if (((((!s.b[1206]) && s.b[1207]) && s.b[1208]) && s.b[1211]) && (!s.b[1212])) {
            s.store_div_from_scalar_scaled_input(972, (p.p438 * s.v[449]), 970, s.v[183]);
        }

        s.b[1213] = (s.v[968] == 0.0);
        s.v[1213] = if s.b[1213] { 1.0 } else { 0.0 };

        if (((((!s.b[1206]) && s.b[1207]) && s.b[1208]) && (!s.b[1211])) && s.b[1213]) {
            s.store_scalar(972, 0.0);
        }

        if (((((!s.b[1206]) && s.b[1207]) && s.b[1208]) && (!s.b[1211])) && (!s.b[1213])) {
            s.store_div_from_scalar_scaled_input(972, (p.p438 * s.v[449]), 968, s.v[183]);
        }

        s.b[1214] = (p.p8 == 0.0);
        s.v[1214] = if s.b[1214] { 1.0 } else { 0.0 };

        s.b[1215] = (p.p8 == 1.0);
        s.v[1215] = if s.b[1215] { 1.0 } else { 0.0 };

        s.b[1216] = (p.p8 == 2.0);
        s.v[1216] = if s.b[1216] { 1.0 } else { 0.0 };

        s.b[1217] = (p.p8 == 3.0);
        s.v[1217] = if s.b[1217] { 1.0 } else { 0.0 };

        s.b[1218] = (p.p8 == 4.0);
        s.v[1218] = if s.b[1218] { 1.0 } else { 0.0 };

        s.b[1219] = (p.p8 == 5.0);
        s.v[1219] = if s.b[1219] { 1.0 } else { 0.0 };

        s.b[1220] = (p.p8 == 6.0);
        s.v[1220] = if s.b[1220] { 1.0 } else { 0.0 };

        s.b[1221] = (p.p8 == 7.0);
        s.v[1221] = if s.b[1221] { 1.0 } else { 0.0 };

        s.b[1222] = (p.p8 == 8.0);
        s.v[1222] = if s.b[1222] { 1.0 } else { 0.0 };

        s.b[1223] = (p.p8 == 9.0);
        s.v[1223] = if s.b[1223] { 1.0 } else { 0.0 };

        s.b[1224] = (p.p8 == 10.0);
        s.v[1224] = if s.b[1224] { 1.0 } else { 0.0 };

        s.b[1225] = (1.0 == 1.0);
        s.v[1225] = if s.b[1225] { 1.0 } else { 0.0 };

        s.b[1226] = (1.0 == 1.0);
        s.v[1226] = if s.b[1226] { 1.0 } else { 0.0 };

        s.b[1227] = (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0));
        s.v[1227] = if s.b[1227] { 1.0 } else { 0.0 };

        s.b[1228] = (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0));
        s.v[1228] = if s.b[1228] { 1.0 } else { 0.0 };

        s.b[1229] = (s.v[971] == 0.0);
        s.v[1229] = if s.b[1229] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && s.b[1225]) && s.b[1226]) && s.b[1227]) && s.b[1229]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && s.b[1225]) && s.b[1226]) && s.b[1227]) && (!s.b[1229])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 971, s.v[183]);
        }

        s.b[1231] = ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));
        s.v[1231] = if s.b[1231] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && s.b[1225]) && s.b[1226]) && (s.b[1228] && (!s.b[1227]))) && s.b[1231]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && s.b[1225]) && s.b[1226]) && (s.b[1228] && (!s.b[1227]))) && (!s.b[1231])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 971, (3.0 * (s.v[449] + s.v[451])));
        }

        if ((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && s.b[1225]) && s.b[1226]) && (!(s.b[1227] || s.b[1228]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1232] = (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0));
        s.v[1232] = if s.b[1232] { 1.0 } else { 0.0 };

        s.b[1233] = (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0));
        s.v[1233] = if s.b[1233] { 1.0 } else { 0.0 };

        s.b[1234] = (s.v[971] == 0.0);
        s.v[1234] = if s.b[1234] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && s.b[1225]) && (!s.b[1226])) && s.b[1232]) && s.b[1234]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && s.b[1225]) && (!s.b[1226])) && s.b[1232]) && (!s.b[1234])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 971, s.v[183]);
        }

        s.b[1236] = ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));
        s.v[1236] = if s.b[1236] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && s.b[1225]) && (!s.b[1226])) && (s.b[1233] && (!s.b[1232]))) && s.b[1236]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && s.b[1225]) && (!s.b[1226])) && (s.b[1233] && (!s.b[1232]))) && (!s.b[1236])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 971, (3.0 * (s.v[449] + s.v[451])));
        }

        if ((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && s.b[1225]) && (!s.b[1226])) && (!(s.b[1232] || s.b[1233]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1237] = (0.0 == 1.0);
        s.v[1237] = if s.b[1237] { 1.0 } else { 0.0 };

        s.b[1238] = (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0));
        s.v[1238] = if s.b[1238] { 1.0 } else { 0.0 };

        s.b[1239] = (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0));
        s.v[1239] = if s.b[1239] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1240] = (s.v[969] == 0.0);
        s.v[1240] = if s.b[1240] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && (!s.b[1225])) && s.b[1237]) && s.b[1238]) && s.b[1240]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && (!s.b[1225])) && s.b[1237]) && s.b[1238]) && (!s.b[1240])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 969, s.v[183]);
        }

        s.b[1242] = ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));
        s.v[1242] = if s.b[1242] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && (!s.b[1225])) && s.b[1237]) && (s.b[1239] && (!s.b[1238]))) && s.b[1242]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && (!s.b[1225])) && s.b[1237]) && (s.b[1239] && (!s.b[1238]))) && (!s.b[1242])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 969, (3.0 * (s.v[449] + s.v[451])));
        }

        if ((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && (!s.b[1225])) && s.b[1237]) && (!(s.b[1238] || s.b[1239]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1243] = (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0));
        s.v[1243] = if s.b[1243] { 1.0 } else { 0.0 };

        s.b[1244] = (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0));
        s.v[1244] = if s.b[1244] { 1.0 } else { 0.0 };

        s.b[1245] = (s.v[969] == 0.0);
        s.v[1245] = if s.b[1245] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && (!s.b[1225])) && (!s.b[1237])) && s.b[1243]) && s.b[1245]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && (!s.b[1225])) && (!s.b[1237])) && s.b[1243]) && (!s.b[1245])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 969, s.v[183]);
        }

        s.b[1247] = ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));
        s.v[1247] = if s.b[1247] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && (!s.b[1225])) && (!s.b[1237])) && (s.b[1244] && (!s.b[1243]))) && s.b[1247]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && (!s.b[1225])) && (!s.b[1237])) && (s.b[1244] && (!s.b[1243]))) && (!s.b[1247])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 969, (3.0 * (s.v[449] + s.v[451])));
        }

        if ((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && (!s.b[1225])) && (!s.b[1237])) && (!(s.b[1243] || s.b[1244]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1248] = (1.0 == 1.0);
        s.v[1248] = if s.b[1248] { 1.0 } else { 0.0 };

        s.b[1249] = (1.0 == 1.0);
        s.v[1249] = if s.b[1249] { 1.0 } else { 0.0 };

        s.b[1250] = (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0));
        s.v[1250] = if s.b[1250] { 1.0 } else { 0.0 };

        s.b[1251] = (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0));
        s.v[1251] = if s.b[1251] { 1.0 } else { 0.0 };

        s.b[1252] = (s.v[971] == 0.0);
        s.v[1252] = if s.b[1252] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && s.b[1248]) && s.b[1249]) && s.b[1250]) && s.b[1252]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && s.b[1248]) && s.b[1249]) && s.b[1250]) && (!s.b[1252])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 971, s.v[183]);
        }

        s.b[1254] = ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));
        s.v[1254] = if s.b[1254] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && s.b[1248]) && s.b[1249]) && (s.b[1251] && (!s.b[1250]))) && s.b[1254]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && s.b[1248]) && s.b[1249]) && (s.b[1251] && (!s.b[1250]))) && (!s.b[1254])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 971, (3.0 * (s.v[449] + s.v[451])));
        }

        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && s.b[1248]) && s.b[1249]) && (!(s.b[1250] || s.b[1251]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1255] = (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0));
        s.v[1255] = if s.b[1255] { 1.0 } else { 0.0 };

        s.b[1256] = (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0));
        s.v[1256] = if s.b[1256] { 1.0 } else { 0.0 };

        s.b[1257] = (s.v[971] == 0.0);
        s.v[1257] = if s.b[1257] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && s.b[1248]) && (!s.b[1249])) && s.b[1255]) && s.b[1257]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && s.b[1248]) && (!s.b[1249])) && s.b[1255]) && (!s.b[1257])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 971, s.v[183]);
        }

        s.b[1259] = ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));
        s.v[1259] = if s.b[1259] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && s.b[1248]) && (!s.b[1249])) && (s.b[1256] && (!s.b[1255]))) && s.b[1259]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && s.b[1248]) && (!s.b[1249])) && (s.b[1256] && (!s.b[1255]))) && (!s.b[1259])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 971, (3.0 * (s.v[449] + s.v[451])));
        }

        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && s.b[1248]) && (!s.b[1249])) && (!(s.b[1255] || s.b[1256]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1260] = (0.0 == 1.0);
        s.v[1260] = if s.b[1260] { 1.0 } else { 0.0 };

        s.b[1261] = (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0));
        s.v[1261] = if s.b[1261] { 1.0 } else { 0.0 };

        s.b[1262] = (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0));
        s.v[1262] = if s.b[1262] { 1.0 } else { 0.0 };

        s.b[1263] = (s.v[969] == 0.0);
        s.v[1263] = if s.b[1263] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && (!s.b[1248])) && s.b[1260]) && s.b[1261]) && s.b[1263]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && (!s.b[1248])) && s.b[1260]) && s.b[1261]) && (!s.b[1263])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 969, s.v[183]);
        }

        s.b[1265] = ((s.v[969] == 0.0) || (s.v[449] == 0.0));
        s.v[1265] = if s.b[1265] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && (!s.b[1248])) && s.b[1260]) && (s.b[1262] && (!s.b[1261]))) && s.b[1265]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && (!s.b[1248])) && s.b[1260]) && (s.b[1262] && (!s.b[1261]))) && (!s.b[1265])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 969, (6.0 * s.v[449]));
        }

        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && (!s.b[1248])) && s.b[1260]) && (!(s.b[1261] || s.b[1262]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1266] = (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0));
        s.v[1266] = if s.b[1266] { 1.0 } else { 0.0 };

        s.b[1267] = (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0));
        s.v[1267] = if s.b[1267] { 1.0 } else { 0.0 };

        s.b[1268] = (s.v[969] == 0.0);
        s.v[1268] = if s.b[1268] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && (!s.b[1248])) && (!s.b[1260])) && s.b[1266]) && s.b[1268]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && (!s.b[1248])) && (!s.b[1260])) && s.b[1266]) && (!s.b[1268])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 969, s.v[183]);
        }

        s.b[1270] = ((s.v[969] == 0.0) || (s.v[449] == 0.0));
        s.v[1270] = if s.b[1270] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && (!s.b[1248])) && (!s.b[1260])) && (s.b[1267] && (!s.b[1266]))) && s.b[1270]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && (!s.b[1248])) && (!s.b[1260])) && (s.b[1267] && (!s.b[1266]))) && (!s.b[1270])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 969, (6.0 * s.v[449]));
        }

        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && (!s.b[1248])) && (!s.b[1260])) && (!(s.b[1266] || s.b[1267]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1271] = (1.0 == 1.0);
        s.v[1271] = if s.b[1271] { 1.0 } else { 0.0 };

        s.b[1272] = (1.0 == 1.0);
        s.v[1272] = if s.b[1272] { 1.0 } else { 0.0 };

        s.b[1273] = (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0));
        s.v[1273] = if s.b[1273] { 1.0 } else { 0.0 };

        s.b[1274] = (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0));
        s.v[1274] = if s.b[1274] { 1.0 } else { 0.0 };

        s.b[1275] = (s.v[971] == 0.0);
        s.v[1275] = if s.b[1275] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && s.b[1271]) && s.b[1272]) && s.b[1273]) && s.b[1275]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && s.b[1271]) && s.b[1272]) && s.b[1273]) && (!s.b[1275])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 971, s.v[183]);
        }

        s.b[1277] = ((s.v[971] == 0.0) || (s.v[449] == 0.0));
        s.v[1277] = if s.b[1277] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && s.b[1271]) && s.b[1272]) && (s.b[1274] && (!s.b[1273]))) && s.b[1277]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && s.b[1271]) && s.b[1272]) && (s.b[1274] && (!s.b[1273]))) && (!s.b[1277])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 971, (6.0 * s.v[449]));
        }

        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && s.b[1271]) && s.b[1272]) && (!(s.b[1273] || s.b[1274]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1278] = (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0));
        s.v[1278] = if s.b[1278] { 1.0 } else { 0.0 };

        s.b[1279] = (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0));
        s.v[1279] = if s.b[1279] { 1.0 } else { 0.0 };

        s.b[1280] = (s.v[971] == 0.0);
        s.v[1280] = if s.b[1280] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && s.b[1271]) && (!s.b[1272])) && s.b[1278]) && s.b[1280]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && s.b[1271]) && (!s.b[1272])) && s.b[1278]) && (!s.b[1280])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 971, s.v[183]);
        }

        s.b[1282] = ((s.v[971] == 0.0) || (s.v[449] == 0.0));
        s.v[1282] = if s.b[1282] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && s.b[1271]) && (!s.b[1272])) && (s.b[1279] && (!s.b[1278]))) && s.b[1282]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && s.b[1271]) && (!s.b[1272])) && (s.b[1279] && (!s.b[1278]))) && (!s.b[1282])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 971, (6.0 * s.v[449]));
        }

        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && s.b[1271]) && (!s.b[1272])) && (!(s.b[1278] || s.b[1279]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1283] = (0.0 == 1.0);
        s.v[1283] = if s.b[1283] { 1.0 } else { 0.0 };

        s.b[1284] = (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0));
        s.v[1284] = if s.b[1284] { 1.0 } else { 0.0 };

        s.b[1285] = (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0));
        s.v[1285] = if s.b[1285] { 1.0 } else { 0.0 };

        s.b[1286] = (s.v[969] == 0.0);
        s.v[1286] = if s.b[1286] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && (!s.b[1271])) && s.b[1283]) && s.b[1284]) && s.b[1286]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && (!s.b[1271])) && s.b[1283]) && s.b[1284]) && (!s.b[1286])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 969, s.v[183]);
        }

        s.b[1288] = ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));
        s.v[1288] = if s.b[1288] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && (!s.b[1271])) && s.b[1283]) && (s.b[1285] && (!s.b[1284]))) && s.b[1288]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && (!s.b[1271])) && s.b[1283]) && (s.b[1285] && (!s.b[1284]))) && (!s.b[1288])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 969, (3.0 * (s.v[449] + s.v[451])));
        }

        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && (!s.b[1271])) && s.b[1283]) && (!(s.b[1284] || s.b[1285]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1289] = (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0));
        s.v[1289] = if s.b[1289] { 1.0 } else { 0.0 };

        s.b[1290] = (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0));
        s.v[1290] = if s.b[1290] { 1.0 } else { 0.0 };

        s.b[1291] = (s.v[969] == 0.0);
        s.v[1291] = if s.b[1291] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && (!s.b[1271])) && (!s.b[1283])) && s.b[1289]) && s.b[1291]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && (!s.b[1271])) && (!s.b[1283])) && s.b[1289]) && (!s.b[1291])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 969, s.v[183]);
        }

        s.b[1293] = ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));
        s.v[1293] = if s.b[1293] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && (!s.b[1271])) && (!s.b[1283])) && (s.b[1290] && (!s.b[1289]))) && s.b[1293]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && (!s.b[1271])) && (!s.b[1283])) && (s.b[1290] && (!s.b[1289]))) && (!s.b[1293])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 969, (3.0 * (s.v[449] + s.v[451])));
        }

        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && (!s.b[1271])) && (!s.b[1283])) && (!(s.b[1289] || s.b[1290]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1294] = (1.0 == 1.0);
        s.v[1294] = if s.b[1294] { 1.0 } else { 0.0 };

        s.b[1295] = (1.0 == 1.0);
        s.v[1295] = if s.b[1295] { 1.0 } else { 0.0 };

        s.b[1296] = (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0));
        s.v[1296] = if s.b[1296] { 1.0 } else { 0.0 };

        s.b[1297] = (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0));
        s.v[1297] = if s.b[1297] { 1.0 } else { 0.0 };

        s.b[1298] = (s.v[971] == 0.0);
        s.v[1298] = if s.b[1298] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && s.b[1294]) && s.b[1295]) && s.b[1296]) && s.b[1298]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && s.b[1294]) && s.b[1295]) && s.b[1296]) && (!s.b[1298])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 971, s.v[183]);
        }

        s.b[1300] = ((s.v[971] == 0.0) || (s.v[449] == 0.0));
        s.v[1300] = if s.b[1300] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && s.b[1294]) && s.b[1295]) && (s.b[1297] && (!s.b[1296]))) && s.b[1300]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && s.b[1294]) && s.b[1295]) && (s.b[1297] && (!s.b[1296]))) && (!s.b[1300])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 971, (6.0 * s.v[449]));
        }

        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && s.b[1294]) && s.b[1295]) && (!(s.b[1296] || s.b[1297]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1301] = (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0));
        s.v[1301] = if s.b[1301] { 1.0 } else { 0.0 };

        s.b[1302] = (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0));
        s.v[1302] = if s.b[1302] { 1.0 } else { 0.0 };

        s.b[1303] = (s.v[971] == 0.0);
        s.v[1303] = if s.b[1303] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && s.b[1294]) && (!s.b[1295])) && s.b[1301]) && s.b[1303]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && s.b[1294]) && (!s.b[1295])) && s.b[1301]) && (!s.b[1303])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 971, s.v[183]);
        }

        s.b[1305] = ((s.v[971] == 0.0) || (s.v[449] == 0.0));
        s.v[1305] = if s.b[1305] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && s.b[1294]) && (!s.b[1295])) && (s.b[1302] && (!s.b[1301]))) && s.b[1305]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && s.b[1294]) && (!s.b[1295])) && (s.b[1302] && (!s.b[1301]))) && (!s.b[1305])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 971, (6.0 * s.v[449]));
        }

        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && s.b[1294]) && (!s.b[1295])) && (!(s.b[1301] || s.b[1302]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1306] = (0.0 == 1.0);
        s.v[1306] = if s.b[1306] { 1.0 } else { 0.0 };

        s.b[1307] = (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0));
        s.v[1307] = if s.b[1307] { 1.0 } else { 0.0 };

        s.b[1308] = (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0));
        s.v[1308] = if s.b[1308] { 1.0 } else { 0.0 };

        s.b[1309] = (s.v[969] == 0.0);
        s.v[1309] = if s.b[1309] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && (!s.b[1294])) && s.b[1306]) && s.b[1307]) && s.b[1309]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && (!s.b[1294])) && s.b[1306]) && s.b[1307]) && (!s.b[1309])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 969, s.v[183]);
        }

        s.b[1311] = ((s.v[969] == 0.0) || (s.v[449] == 0.0));
        s.v[1311] = if s.b[1311] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && (!s.b[1294])) && s.b[1306]) && (s.b[1308] && (!s.b[1307]))) && s.b[1311]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && (!s.b[1294])) && s.b[1306]) && (s.b[1308] && (!s.b[1307]))) && (!s.b[1311])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 969, (6.0 * s.v[449]));
        }

        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && (!s.b[1294])) && s.b[1306]) && (!(s.b[1307] || s.b[1308]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1312] = (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0));
        s.v[1312] = if s.b[1312] { 1.0 } else { 0.0 };

        s.b[1313] = (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0));
        s.v[1313] = if s.b[1313] { 1.0 } else { 0.0 };

        s.b[1314] = (s.v[969] == 0.0);
        s.v[1314] = if s.b[1314] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && (!s.b[1294])) && (!s.b[1306])) && s.b[1312]) && s.b[1314]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && (!s.b[1294])) && (!s.b[1306])) && s.b[1312]) && (!s.b[1314])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 969, s.v[183]);
        }

        s.b[1316] = ((s.v[969] == 0.0) || (s.v[449] == 0.0));
        s.v[1316] = if s.b[1316] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && (!s.b[1294])) && (!s.b[1306])) && (s.b[1313] && (!s.b[1312]))) && s.b[1316]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && (!s.b[1294])) && (!s.b[1306])) && (s.b[1313] && (!s.b[1312]))) && (!s.b[1316])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 969, (6.0 * s.v[449]));
        }

        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && (!s.b[1294])) && (!s.b[1306])) && (!(s.b[1312] || s.b[1313]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1317] = (1.0 == 1.0);
        s.v[1317] = if s.b[1317] { 1.0 } else { 0.0 };

        s.b[1318] = (1.0 == 1.0);
        s.v[1318] = if s.b[1318] { 1.0 } else { 0.0 };

        s.b[1319] = (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0));
        s.v[1319] = if s.b[1319] { 1.0 } else { 0.0 };

        s.b[1320] = (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0));
        s.v[1320] = if s.b[1320] { 1.0 } else { 0.0 };

        s.b[1321] = (s.v[971] == 0.0);
        s.v[1321] = if s.b[1321] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1218] && (!(((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217])))) && s.b[1317]) && s.b[1318]) && s.b[1319]) && s.b[1321]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1218] && (!(((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217])))) && s.b[1317]) && s.b[1318]) && s.b[1319]) && (!s.b[1321])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 971, s.v[183]);
        }

        s.b[1323] = ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));
        s.v[1323] = if s.b[1323] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1218] && (!(((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217])))) && s.b[1317]) && s.b[1318]) && (s.b[1320] && (!s.b[1319]))) && s.b[1323]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1218] && (!(((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217])))) && s.b[1317]) && s.b[1318]) && (s.b[1320] && (!s.b[1319]))) && (!s.b[1323])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 971, (3.0 * (s.v[449] + s.v[451])));
        }

        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1218] && (!(((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217])))) && s.b[1317]) && s.b[1318]) && (!(s.b[1319] || s.b[1320]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1324] = (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0));
        s.v[1324] = if s.b[1324] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1325] = (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0));
        s.v[1325] = if s.b[1325] { 1.0 } else { 0.0 };

        s.b[1326] = (s.v[971] == 0.0);
        s.v[1326] = if s.b[1326] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1218] && (!(((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217])))) && s.b[1317]) && (!s.b[1318])) && s.b[1324]) && s.b[1326]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1218] && (!(((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217])))) && s.b[1317]) && (!s.b[1318])) && s.b[1324]) && (!s.b[1326])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 971, s.v[183]);
        }

        s.b[1328] = ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));
        s.v[1328] = if s.b[1328] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1218] && (!(((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217])))) && s.b[1317]) && (!s.b[1318])) && (s.b[1325] && (!s.b[1324]))) && s.b[1328]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1218] && (!(((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217])))) && s.b[1317]) && (!s.b[1318])) && (s.b[1325] && (!s.b[1324]))) && (!s.b[1328])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 971, (3.0 * (s.v[449] + s.v[451])));
        }

        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1218] && (!(((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217])))) && s.b[1317]) && (!s.b[1318])) && (!(s.b[1324] || s.b[1325]))) {
            s.store_scalar(973, 0.0);
        }

        if ((((!s.b[1206]) && s.b[1207]) && (s.b[1218] && (!(((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217])))) && (!s.b[1317])) {
            s.store_scalar(973, ((p.p438 * s.v[450]) / s.v[183]));
        }

        s.b[1329] = (1.0 == 1.0);
        s.v[1329] = if s.b[1329] { 1.0 } else { 0.0 };

        s.b[1330] = (1.0 == 1.0);
        s.v[1330] = if s.b[1330] { 1.0 } else { 0.0 };

        s.b[1331] = (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0));
        s.v[1331] = if s.b[1331] { 1.0 } else { 0.0 };

        s.b[1332] = (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0));
        s.v[1332] = if s.b[1332] { 1.0 } else { 0.0 };

        s.b[1333] = (s.v[971] == 0.0);
        s.v[1333] = if s.b[1333] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1219] && (!((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218])))) && s.b[1329]) && s.b[1330]) && s.b[1331]) && s.b[1333]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1219] && (!((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218])))) && s.b[1329]) && s.b[1330]) && s.b[1331]) && (!s.b[1333])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 971, s.v[183]);
        }

        s.b[1335] = ((s.v[971] == 0.0) || (s.v[449] == 0.0));
        s.v[1335] = if s.b[1335] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1219] && (!((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218])))) && s.b[1329]) && s.b[1330]) && (s.b[1332] && (!s.b[1331]))) && s.b[1335]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1219] && (!((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218])))) && s.b[1329]) && s.b[1330]) && (s.b[1332] && (!s.b[1331]))) && (!s.b[1335])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 971, (6.0 * s.v[449]));
        }

        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1219] && (!((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218])))) && s.b[1329]) && s.b[1330]) && (!(s.b[1331] || s.b[1332]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1336] = (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0));
        s.v[1336] = if s.b[1336] { 1.0 } else { 0.0 };

        s.b[1337] = (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0));
        s.v[1337] = if s.b[1337] { 1.0 } else { 0.0 };

        s.b[1338] = (s.v[971] == 0.0);
        s.v[1338] = if s.b[1338] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1219] && (!((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218])))) && s.b[1329]) && (!s.b[1330])) && s.b[1336]) && s.b[1338]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1219] && (!((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218])))) && s.b[1329]) && (!s.b[1330])) && s.b[1336]) && (!s.b[1338])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 971, s.v[183]);
        }

        s.b[1340] = ((s.v[971] == 0.0) || (s.v[449] == 0.0));
        s.v[1340] = if s.b[1340] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1219] && (!((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218])))) && s.b[1329]) && (!s.b[1330])) && (s.b[1337] && (!s.b[1336]))) && s.b[1340]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1219] && (!((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218])))) && s.b[1329]) && (!s.b[1330])) && (s.b[1337] && (!s.b[1336]))) && (!s.b[1340])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 971, (6.0 * s.v[449]));
        }

        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1219] && (!((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218])))) && s.b[1329]) && (!s.b[1330])) && (!(s.b[1336] || s.b[1337]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1341] = (s.v[969] == 0.0);
        s.v[1341] = if s.b[1341] { 1.0 } else { 0.0 };

        if (((((!s.b[1206]) && s.b[1207]) && (s.b[1219] && (!((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218])))) && (!s.b[1329])) && s.b[1341]) {
            s.store_scalar(973, 0.0);
        }

        if (((((!s.b[1206]) && s.b[1207]) && (s.b[1219] && (!((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218])))) && (!s.b[1329])) && (!s.b[1341])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[450]), 969, s.v[183]);
        }

        s.b[1342] = (1.0 == 1.0);
        s.v[1342] = if s.b[1342] { 1.0 } else { 0.0 };

        if ((((!s.b[1206]) && s.b[1207]) && (s.b[1220] && (!(((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219])))) && s.b[1342]) {
            s.store_scalar(973, ((p.p438 * s.v[450]) / s.v[183]));
        }

        s.b[1343] = (0.0 == 1.0);
        s.v[1343] = if s.b[1343] { 1.0 } else { 0.0 };

        s.b[1344] = (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0));
        s.v[1344] = if s.b[1344] { 1.0 } else { 0.0 };

        s.b[1345] = (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0));
        s.v[1345] = if s.b[1345] { 1.0 } else { 0.0 };

        s.b[1346] = (s.v[969] == 0.0);
        s.v[1346] = if s.b[1346] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1220] && (!(((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219])))) && (!s.b[1342])) && s.b[1343]) && s.b[1344]) && s.b[1346]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1220] && (!(((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219])))) && (!s.b[1342])) && s.b[1343]) && s.b[1344]) && (!s.b[1346])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 969, s.v[183]);
        }

        s.b[1348] = ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));
        s.v[1348] = if s.b[1348] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1220] && (!(((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219])))) && (!s.b[1342])) && s.b[1343]) && (s.b[1345] && (!s.b[1344]))) && s.b[1348]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1220] && (!(((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219])))) && (!s.b[1342])) && s.b[1343]) && (s.b[1345] && (!s.b[1344]))) && (!s.b[1348])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 969, (3.0 * (s.v[449] + s.v[451])));
        }

        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1220] && (!(((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219])))) && (!s.b[1342])) && s.b[1343]) && (!(s.b[1344] || s.b[1345]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1349] = (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0));
        s.v[1349] = if s.b[1349] { 1.0 } else { 0.0 };

        s.b[1350] = (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0));
        s.v[1350] = if s.b[1350] { 1.0 } else { 0.0 };

        s.b[1351] = (s.v[969] == 0.0);
        s.v[1351] = if s.b[1351] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1220] && (!(((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219])))) && (!s.b[1342])) && (!s.b[1343])) && s.b[1349]) && s.b[1351]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1220] && (!(((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219])))) && (!s.b[1342])) && (!s.b[1343])) && s.b[1349]) && (!s.b[1351])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 969, s.v[183]);
        }

        s.b[1353] = ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));
        s.v[1353] = if s.b[1353] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1220] && (!(((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219])))) && (!s.b[1342])) && (!s.b[1343])) && (s.b[1350] && (!s.b[1349]))) && s.b[1353]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1220] && (!(((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219])))) && (!s.b[1342])) && (!s.b[1343])) && (s.b[1350] && (!s.b[1349]))) && (!s.b[1353])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 969, (3.0 * (s.v[449] + s.v[451])));
        }

        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1220] && (!(((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219])))) && (!s.b[1342])) && (!s.b[1343])) && (!(s.b[1349] || s.b[1350]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1354] = (1.0 == 1.0);
        s.v[1354] = if s.b[1354] { 1.0 } else { 0.0 };

        s.b[1355] = (s.v[971] == 0.0);
        s.v[1355] = if s.b[1355] { 1.0 } else { 0.0 };

        if (((((!s.b[1206]) && s.b[1207]) && (s.b[1221] && (!((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220])))) && s.b[1354]) && s.b[1355]) {
            s.store_scalar(973, 0.0);
        }

        if (((((!s.b[1206]) && s.b[1207]) && (s.b[1221] && (!((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220])))) && s.b[1354]) && (!s.b[1355])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[450]), 971, s.v[183]);
        }

        s.b[1356] = (0.0 == 1.0);
        s.v[1356] = if s.b[1356] { 1.0 } else { 0.0 };

        s.b[1357] = (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0));
        s.v[1357] = if s.b[1357] { 1.0 } else { 0.0 };

        s.b[1358] = (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0));
        s.v[1358] = if s.b[1358] { 1.0 } else { 0.0 };

        s.b[1359] = (s.v[969] == 0.0);
        s.v[1359] = if s.b[1359] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1221] && (!((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220])))) && (!s.b[1354])) && s.b[1356]) && s.b[1357]) && s.b[1359]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1221] && (!((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220])))) && (!s.b[1354])) && s.b[1356]) && s.b[1357]) && (!s.b[1359])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 969, s.v[183]);
        }

        s.b[1361] = ((s.v[969] == 0.0) || (s.v[449] == 0.0));
        s.v[1361] = if s.b[1361] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1221] && (!((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220])))) && (!s.b[1354])) && s.b[1356]) && (s.b[1358] && (!s.b[1357]))) && s.b[1361]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1221] && (!((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220])))) && (!s.b[1354])) && s.b[1356]) && (s.b[1358] && (!s.b[1357]))) && (!s.b[1361])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 969, (6.0 * s.v[449]));
        }

        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1221] && (!((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220])))) && (!s.b[1354])) && s.b[1356]) && (!(s.b[1357] || s.b[1358]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1362] = (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0));
        s.v[1362] = if s.b[1362] { 1.0 } else { 0.0 };

        s.b[1363] = (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0));
        s.v[1363] = if s.b[1363] { 1.0 } else { 0.0 };

        s.b[1364] = (s.v[969] == 0.0);
        s.v[1364] = if s.b[1364] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1221] && (!((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220])))) && (!s.b[1354])) && (!s.b[1356])) && s.b[1362]) && s.b[1364]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1221] && (!((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220])))) && (!s.b[1354])) && (!s.b[1356])) && s.b[1362]) && (!s.b[1364])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 969, s.v[183]);
        }

        s.b[1366] = ((s.v[969] == 0.0) || (s.v[449] == 0.0));
        s.v[1366] = if s.b[1366] { 1.0 } else { 0.0 };

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1221] && (!((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220])))) && (!s.b[1354])) && (!s.b[1356])) && (s.b[1363] && (!s.b[1362]))) && s.b[1366]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1221] && (!((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220])))) && (!s.b[1354])) && (!s.b[1356])) && (s.b[1363] && (!s.b[1362]))) && (!s.b[1366])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 969, (6.0 * s.v[449]));
        }

        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1221] && (!((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220])))) && (!s.b[1354])) && (!s.b[1356])) && (!(s.b[1362] || s.b[1363]))) {
            s.store_scalar(973, 0.0);
        }

        if (((!s.b[1206]) && s.b[1207]) && (s.b[1222] && (!(((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220]) || s.b[1221])))) {
            s.store_scalar(973, ((p.p438 * s.v[450]) / s.v[183]));
        }

        s.b[1367] = (1.0 == 1.0);
        s.v[1367] = if s.b[1367] { 1.0 } else { 0.0 };

        if ((((!s.b[1206]) && s.b[1207]) && (s.b[1223] && (!((((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220]) || s.b[1221]) || s.b[1222])))) && s.b[1367]) {
            s.store_scalar(973, (((0.5 * p.p438) * s.v[449]) / s.v[183]));
        }

        s.b[1368] = (p.p2 == 2.0);
        s.v[1368] = if s.b[1368] { 1.0 } else { 0.0 };

        if (((((!s.b[1206]) && s.b[1207]) && (s.b[1223] && (!((((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220]) || s.b[1221]) || s.b[1222])))) && s.b[1367]) && s.b[1368]) {
            s.store_scalar(972, 0.0);
        }

        if (((((!s.b[1206]) && s.b[1207]) && (s.b[1223] && (!((((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220]) || s.b[1221]) || s.b[1222])))) && s.b[1367]) && (!s.b[1368])) {
            s.store_scalar(972, ((p.p438 * s.v[449]) / (s.v[183] * (p.p2 - 2.0))));
        }

        if ((((!s.b[1206]) && s.b[1207]) && (s.b[1223] && (!((((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220]) || s.b[1221]) || s.b[1222])))) && (!s.b[1367])) {
            s.store_scalar(973, 0.0);
            s.store_scalar(972, ((p.p438 * s.v[449]) / (s.v[183] * p.p2)));
        }

        s.b[1369] = (1.0 == 1.0);
        s.v[1369] = if s.b[1369] { 1.0 } else { 0.0 };

        if ((((!s.b[1206]) && s.b[1207]) && (s.b[1224] && (!(((((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220]) || s.b[1221]) || s.b[1222]) || s.b[1223])))) && s.b[1369]) {
            s.store_scalar(973, 0.0);
            s.store_scalar(972, ((p.p438 * s.v[449]) / (s.v[183] * p.p2)));
        }

        if ((((!s.b[1206]) && s.b[1207]) && (s.b[1224] && (!(((((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220]) || s.b[1221]) || s.b[1222]) || s.b[1223])))) && (!s.b[1369])) {
            s.store_scalar(973, (((0.5 * p.p438) * s.v[449]) / s.v[183]));
        }

        s.b[1370] = (p.p2 == 2.0);
        s.v[1370] = if s.b[1370] { 1.0 } else { 0.0 };

        if (((((!s.b[1206]) && s.b[1207]) && (s.b[1224] && (!(((((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220]) || s.b[1221]) || s.b[1222]) || s.b[1223])))) && (!s.b[1369])) && s.b[1370]) {
            s.store_scalar(972, 0.0);
        }

        if (((((!s.b[1206]) && s.b[1207]) && (s.b[1224] && (!(((((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220]) || s.b[1221]) || s.b[1222]) || s.b[1223])))) && (!s.b[1369])) && (!s.b[1370])) {
            s.store_scalar(972, ((p.p438 * s.v[449]) / (s.v[183] * (p.p2 - 2.0))));
        }

        if (((!s.b[1206]) && s.b[1207]) && (!((((((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220]) || s.b[1221]) || s.b[1222]) || s.b[1223]) || s.b[1224]))) {
            s.store_scalar(972, 0.0);
        }

        s.b[1371] = (s.v[972] <= 0.0);
        s.v[1371] = if s.b[1371] { 1.0 } else { 0.0 };

        if (((!s.b[1206]) && s.b[1207]) && s.b[1371]) {
            s.copy_ad(452, 973);
        }

        s.b[1372] = (s.v[973] <= 0.0);
        s.v[1372] = if s.b[1372] { 1.0 } else { 0.0 };

        if ((((!s.b[1206]) && s.b[1207]) && (!s.b[1371])) && s.b[1372]) {
            s.copy_ad(452, 972);
        }

        if ((((!s.b[1206]) && s.b[1207]) && (!s.b[1371])) && (!s.b[1372])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(452, 972, 973, 1.0, 972, 1.0, 973, 1.0, 1.0);
        }

        if ((!s.b[1206]) && (!s.b[1207])) {
            s.store_scalar(452, 0.0);
        }

        s.b[1374] = param_given[4];
        s.v[1374] = if s.b[1374] { 1.0 } else { 0.0 };

        if s.b[1374] {
            s.store_scalar(453, (p.p438 * p.p4));
        }

        s.b[1375] = ((p.p9 > 0.0) && (p.p438 > 0.0));
        s.v[1375] = if s.b[1375] { 1.0 } else { 0.0 };

        s.b[1376] = (p.p8 < 9.0);
        s.v[1376] = if s.b[1376] { 1.0 } else { 0.0 };

        s.b[1377] = ((p.p2 % 2.0) != 0.0);
        s.v[1377] = if s.b[1377] { 1.0 } else { 0.0 };

        if ((((!s.b[1374]) && s.b[1375]) && s.b[1376]) && s.b[1377]) {
            s.store_scalar(969, 1.0);
            s.store_scalar(971, 1.0);
            s.store_scalar(968, (2.0 * (((p.p2 - 1.0) / 2.0)).max(0.0)));
            s.copy_ad(970, 968);
        }

        s.b[1378] = (p.p6 == 1.0);
        s.v[1378] = if s.b[1378] { 1.0 } else { 0.0 };

        if (((((!s.b[1374]) && s.b[1375]) && s.b[1376]) && (!s.b[1377])) && s.b[1378]) {
            s.store_scalar(969, 2.0);
            s.store_scalar(968, (2.0 * (((p.p2 / 2.0) - 1.0)).max(0.0)));
            s.store_scalar(971, 0.0);
            s.store_scalar(970, p.p2);
        }

        if (((((!s.b[1374]) && s.b[1375]) && s.b[1376]) && (!s.b[1377])) && (!s.b[1378])) {
            s.store_scalar(969, 0.0);
            s.store_scalar(968, p.p2);
            s.store_scalar(971, 2.0);
            s.store_scalar(970, (2.0 * (((p.p2 / 2.0) - 1.0)).max(0.0)));
        }

        s.b[1379] = (0.0 == 1.0);
        s.v[1379] = if s.b[1379] { 1.0 } else { 0.0 };

        s.b[1380] = (s.v[970] == 0.0);
        s.v[1380] = if s.b[1380] { 1.0 } else { 0.0 };

        if (((((!s.b[1374]) && s.b[1375]) && s.b[1376]) && s.b[1379]) && s.b[1380]) {
            s.store_scalar(972, 0.0);
        }

        if (((((!s.b[1374]) && s.b[1375]) && s.b[1376]) && s.b[1379]) && (!s.b[1380])) {
            s.store_div_from_scalar_scaled_input(972, (p.p438 * s.v[449]), 970, s.v[183]);
        }

        s.b[1381] = (s.v[968] == 0.0);
        s.v[1381] = if s.b[1381] { 1.0 } else { 0.0 };

        if (((((!s.b[1374]) && s.b[1375]) && s.b[1376]) && (!s.b[1379])) && s.b[1381]) {
            s.store_scalar(972, 0.0);
        }

        if (((((!s.b[1374]) && s.b[1375]) && s.b[1376]) && (!s.b[1379])) && (!s.b[1381])) {
            s.store_div_from_scalar_scaled_input(972, (p.p438 * s.v[449]), 968, s.v[183]);
        }

        s.b[1382] = (p.p8 == 0.0);
        s.v[1382] = if s.b[1382] { 1.0 } else { 0.0 };

        s.b[1383] = (p.p8 == 1.0);
        s.v[1383] = if s.b[1383] { 1.0 } else { 0.0 };

        s.b[1384] = (p.p8 == 2.0);
        s.v[1384] = if s.b[1384] { 1.0 } else { 0.0 };

        s.b[1385] = (p.p8 == 3.0);
        s.v[1385] = if s.b[1385] { 1.0 } else { 0.0 };

        s.b[1386] = (p.p8 == 4.0);
        s.v[1386] = if s.b[1386] { 1.0 } else { 0.0 };

        s.b[1387] = (p.p8 == 5.0);
        s.v[1387] = if s.b[1387] { 1.0 } else { 0.0 };

        s.b[1388] = (p.p8 == 6.0);
        s.v[1388] = if s.b[1388] { 1.0 } else { 0.0 };

        s.b[1389] = (p.p8 == 7.0);
        s.v[1389] = if s.b[1389] { 1.0 } else { 0.0 };

        s.b[1390] = (p.p8 == 8.0);
        s.v[1390] = if s.b[1390] { 1.0 } else { 0.0 };

        s.b[1391] = (p.p8 == 9.0);
        s.v[1391] = if s.b[1391] { 1.0 } else { 0.0 };

        s.b[1392] = (p.p8 == 10.0);
        s.v[1392] = if s.b[1392] { 1.0 } else { 0.0 };

        s.b[1393] = (0.0 == 1.0);
        s.v[1393] = if s.b[1393] { 1.0 } else { 0.0 };

        s.b[1394] = (1.0 == 1.0);
        s.v[1394] = if s.b[1394] { 1.0 } else { 0.0 };

        s.b[1395] = (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0));
        s.v[1395] = if s.b[1395] { 1.0 } else { 0.0 };

        s.b[1396] = (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0));
        s.v[1396] = if s.b[1396] { 1.0 } else { 0.0 };

        s.b[1397] = (s.v[971] == 0.0);
        s.v[1397] = if s.b[1397] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && s.b[1393]) && s.b[1394]) && s.b[1395]) && s.b[1397]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && s.b[1393]) && s.b[1394]) && s.b[1395]) && (!s.b[1397])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 971, s.v[183]);
        }

        s.b[1399] = ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));
        s.v[1399] = if s.b[1399] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && s.b[1393]) && s.b[1394]) && (s.b[1396] && (!s.b[1395]))) && s.b[1399]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && s.b[1393]) && s.b[1394]) && (s.b[1396] && (!s.b[1395]))) && (!s.b[1399])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 971, (3.0 * (s.v[449] + s.v[451])));
        }

    }

    pub(super) fn stamp_transient_block_5(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && s.b[1393]) && s.b[1394]) && (!(s.b[1395] || s.b[1396]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1400] = (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0));
        s.v[1400] = if s.b[1400] { 1.0 } else { 0.0 };

        s.b[1401] = (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0));
        s.v[1401] = if s.b[1401] { 1.0 } else { 0.0 };

        s.b[1402] = (s.v[971] == 0.0);
        s.v[1402] = if s.b[1402] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && s.b[1393]) && (!s.b[1394])) && s.b[1400]) && s.b[1402]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && s.b[1393]) && (!s.b[1394])) && s.b[1400]) && (!s.b[1402])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 971, s.v[183]);
        }

        s.b[1404] = ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));
        s.v[1404] = if s.b[1404] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && s.b[1393]) && (!s.b[1394])) && (s.b[1401] && (!s.b[1400]))) && s.b[1404]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && s.b[1393]) && (!s.b[1394])) && (s.b[1401] && (!s.b[1400]))) && (!s.b[1404])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 971, (3.0 * (s.v[449] + s.v[451])));
        }

        if ((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && s.b[1393]) && (!s.b[1394])) && (!(s.b[1400] || s.b[1401]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1405] = (0.0 == 1.0);
        s.v[1405] = if s.b[1405] { 1.0 } else { 0.0 };

        s.b[1406] = (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0));
        s.v[1406] = if s.b[1406] { 1.0 } else { 0.0 };

        s.b[1407] = (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0));
        s.v[1407] = if s.b[1407] { 1.0 } else { 0.0 };

        s.b[1408] = (s.v[969] == 0.0);
        s.v[1408] = if s.b[1408] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && (!s.b[1393])) && s.b[1405]) && s.b[1406]) && s.b[1408]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && (!s.b[1393])) && s.b[1405]) && s.b[1406]) && (!s.b[1408])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 969, s.v[183]);
        }

        s.b[1410] = ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));
        s.v[1410] = if s.b[1410] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && (!s.b[1393])) && s.b[1405]) && (s.b[1407] && (!s.b[1406]))) && s.b[1410]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && (!s.b[1393])) && s.b[1405]) && (s.b[1407] && (!s.b[1406]))) && (!s.b[1410])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 969, (3.0 * (s.v[449] + s.v[451])));
        }

        if ((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && (!s.b[1393])) && s.b[1405]) && (!(s.b[1406] || s.b[1407]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1411] = (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0));
        s.v[1411] = if s.b[1411] { 1.0 } else { 0.0 };

        s.b[1412] = (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0));
        s.v[1412] = if s.b[1412] { 1.0 } else { 0.0 };

        s.b[1413] = (s.v[969] == 0.0);
        s.v[1413] = if s.b[1413] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && (!s.b[1393])) && (!s.b[1405])) && s.b[1411]) && s.b[1413]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && (!s.b[1393])) && (!s.b[1405])) && s.b[1411]) && (!s.b[1413])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 969, s.v[183]);
        }

        s.b[1415] = ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));
        s.v[1415] = if s.b[1415] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && (!s.b[1393])) && (!s.b[1405])) && (s.b[1412] && (!s.b[1411]))) && s.b[1415]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && (!s.b[1393])) && (!s.b[1405])) && (s.b[1412] && (!s.b[1411]))) && (!s.b[1415])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 969, (3.0 * (s.v[449] + s.v[451])));
        }

        if ((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && (!s.b[1393])) && (!s.b[1405])) && (!(s.b[1411] || s.b[1412]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1416] = (0.0 == 1.0);
        s.v[1416] = if s.b[1416] { 1.0 } else { 0.0 };

        s.b[1417] = (1.0 == 1.0);
        s.v[1417] = if s.b[1417] { 1.0 } else { 0.0 };

        s.b[1418] = (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0));
        s.v[1418] = if s.b[1418] { 1.0 } else { 0.0 };

        s.b[1419] = (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0));
        s.v[1419] = if s.b[1419] { 1.0 } else { 0.0 };

        s.b[1420] = (s.v[971] == 0.0);
        s.v[1420] = if s.b[1420] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && s.b[1416]) && s.b[1417]) && s.b[1418]) && s.b[1420]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && s.b[1416]) && s.b[1417]) && s.b[1418]) && (!s.b[1420])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 971, s.v[183]);
        }

        s.b[1422] = ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));
        s.v[1422] = if s.b[1422] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && s.b[1416]) && s.b[1417]) && (s.b[1419] && (!s.b[1418]))) && s.b[1422]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && s.b[1416]) && s.b[1417]) && (s.b[1419] && (!s.b[1418]))) && (!s.b[1422])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 971, (3.0 * (s.v[449] + s.v[451])));
        }

        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && s.b[1416]) && s.b[1417]) && (!(s.b[1418] || s.b[1419]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1423] = (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0));
        s.v[1423] = if s.b[1423] { 1.0 } else { 0.0 };

        s.b[1424] = (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0));
        s.v[1424] = if s.b[1424] { 1.0 } else { 0.0 };

        s.b[1425] = (s.v[971] == 0.0);
        s.v[1425] = if s.b[1425] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && s.b[1416]) && (!s.b[1417])) && s.b[1423]) && s.b[1425]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && s.b[1416]) && (!s.b[1417])) && s.b[1423]) && (!s.b[1425])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 971, s.v[183]);
        }

        s.b[1427] = ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));
        s.v[1427] = if s.b[1427] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && s.b[1416]) && (!s.b[1417])) && (s.b[1424] && (!s.b[1423]))) && s.b[1427]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && s.b[1416]) && (!s.b[1417])) && (s.b[1424] && (!s.b[1423]))) && (!s.b[1427])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 971, (3.0 * (s.v[449] + s.v[451])));
        }

        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && s.b[1416]) && (!s.b[1417])) && (!(s.b[1423] || s.b[1424]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1428] = (0.0 == 1.0);
        s.v[1428] = if s.b[1428] { 1.0 } else { 0.0 };

        s.b[1429] = (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0));
        s.v[1429] = if s.b[1429] { 1.0 } else { 0.0 };

        s.b[1430] = (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0));
        s.v[1430] = if s.b[1430] { 1.0 } else { 0.0 };

        s.b[1431] = (s.v[969] == 0.0);
        s.v[1431] = if s.b[1431] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && (!s.b[1416])) && s.b[1428]) && s.b[1429]) && s.b[1431]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && (!s.b[1416])) && s.b[1428]) && s.b[1429]) && (!s.b[1431])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 969, s.v[183]);
        }

        s.b[1433] = ((s.v[969] == 0.0) || (s.v[449] == 0.0));
        s.v[1433] = if s.b[1433] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && (!s.b[1416])) && s.b[1428]) && (s.b[1430] && (!s.b[1429]))) && s.b[1433]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && (!s.b[1416])) && s.b[1428]) && (s.b[1430] && (!s.b[1429]))) && (!s.b[1433])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 969, (6.0 * s.v[449]));
        }

        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && (!s.b[1416])) && s.b[1428]) && (!(s.b[1429] || s.b[1430]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1434] = (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0));
        s.v[1434] = if s.b[1434] { 1.0 } else { 0.0 };

        s.b[1435] = (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0));
        s.v[1435] = if s.b[1435] { 1.0 } else { 0.0 };

        s.b[1436] = (s.v[969] == 0.0);
        s.v[1436] = if s.b[1436] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && (!s.b[1416])) && (!s.b[1428])) && s.b[1434]) && s.b[1436]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && (!s.b[1416])) && (!s.b[1428])) && s.b[1434]) && (!s.b[1436])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 969, s.v[183]);
        }

        s.b[1438] = ((s.v[969] == 0.0) || (s.v[449] == 0.0));
        s.v[1438] = if s.b[1438] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && (!s.b[1416])) && (!s.b[1428])) && (s.b[1435] && (!s.b[1434]))) && s.b[1438]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && (!s.b[1416])) && (!s.b[1428])) && (s.b[1435] && (!s.b[1434]))) && (!s.b[1438])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 969, (6.0 * s.v[449]));
        }

        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && (!s.b[1416])) && (!s.b[1428])) && (!(s.b[1434] || s.b[1435]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1439] = (0.0 == 1.0);
        s.v[1439] = if s.b[1439] { 1.0 } else { 0.0 };

        s.b[1440] = (1.0 == 1.0);
        s.v[1440] = if s.b[1440] { 1.0 } else { 0.0 };

        s.b[1441] = (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0));
        s.v[1441] = if s.b[1441] { 1.0 } else { 0.0 };

        s.b[1442] = (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0));
        s.v[1442] = if s.b[1442] { 1.0 } else { 0.0 };

        s.b[1443] = (s.v[971] == 0.0);
        s.v[1443] = if s.b[1443] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && s.b[1439]) && s.b[1440]) && s.b[1441]) && s.b[1443]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && s.b[1439]) && s.b[1440]) && s.b[1441]) && (!s.b[1443])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 971, s.v[183]);
        }

        s.b[1445] = ((s.v[971] == 0.0) || (s.v[449] == 0.0));
        s.v[1445] = if s.b[1445] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && s.b[1439]) && s.b[1440]) && (s.b[1442] && (!s.b[1441]))) && s.b[1445]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && s.b[1439]) && s.b[1440]) && (s.b[1442] && (!s.b[1441]))) && (!s.b[1445])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 971, (6.0 * s.v[449]));
        }

        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && s.b[1439]) && s.b[1440]) && (!(s.b[1441] || s.b[1442]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1446] = (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0));
        s.v[1446] = if s.b[1446] { 1.0 } else { 0.0 };

        s.b[1447] = (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0));
        s.v[1447] = if s.b[1447] { 1.0 } else { 0.0 };

        s.b[1448] = (s.v[971] == 0.0);
        s.v[1448] = if s.b[1448] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && s.b[1439]) && (!s.b[1440])) && s.b[1446]) && s.b[1448]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && s.b[1439]) && (!s.b[1440])) && s.b[1446]) && (!s.b[1448])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 971, s.v[183]);
        }

        s.b[1450] = ((s.v[971] == 0.0) || (s.v[449] == 0.0));
        s.v[1450] = if s.b[1450] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && s.b[1439]) && (!s.b[1440])) && (s.b[1447] && (!s.b[1446]))) && s.b[1450]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && s.b[1439]) && (!s.b[1440])) && (s.b[1447] && (!s.b[1446]))) && (!s.b[1450])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 971, (6.0 * s.v[449]));
        }

        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && s.b[1439]) && (!s.b[1440])) && (!(s.b[1446] || s.b[1447]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1451] = (0.0 == 1.0);
        s.v[1451] = if s.b[1451] { 1.0 } else { 0.0 };

        s.b[1452] = (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0));
        s.v[1452] = if s.b[1452] { 1.0 } else { 0.0 };

        s.b[1453] = (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0));
        s.v[1453] = if s.b[1453] { 1.0 } else { 0.0 };

        s.b[1454] = (s.v[969] == 0.0);
        s.v[1454] = if s.b[1454] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && (!s.b[1439])) && s.b[1451]) && s.b[1452]) && s.b[1454]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && (!s.b[1439])) && s.b[1451]) && s.b[1452]) && (!s.b[1454])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 969, s.v[183]);
        }

        s.b[1456] = ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));
        s.v[1456] = if s.b[1456] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && (!s.b[1439])) && s.b[1451]) && (s.b[1453] && (!s.b[1452]))) && s.b[1456]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && (!s.b[1439])) && s.b[1451]) && (s.b[1453] && (!s.b[1452]))) && (!s.b[1456])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 969, (3.0 * (s.v[449] + s.v[451])));
        }

        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && (!s.b[1439])) && s.b[1451]) && (!(s.b[1452] || s.b[1453]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1457] = (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0));
        s.v[1457] = if s.b[1457] { 1.0 } else { 0.0 };

        s.b[1458] = (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0));
        s.v[1458] = if s.b[1458] { 1.0 } else { 0.0 };

        s.b[1459] = (s.v[969] == 0.0);
        s.v[1459] = if s.b[1459] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && (!s.b[1439])) && (!s.b[1451])) && s.b[1457]) && s.b[1459]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && (!s.b[1439])) && (!s.b[1451])) && s.b[1457]) && (!s.b[1459])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 969, s.v[183]);
        }

        s.b[1461] = ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));
        s.v[1461] = if s.b[1461] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && (!s.b[1439])) && (!s.b[1451])) && (s.b[1458] && (!s.b[1457]))) && s.b[1461]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && (!s.b[1439])) && (!s.b[1451])) && (s.b[1458] && (!s.b[1457]))) && (!s.b[1461])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 969, (3.0 * (s.v[449] + s.v[451])));
        }

        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && (!s.b[1439])) && (!s.b[1451])) && (!(s.b[1457] || s.b[1458]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1462] = (0.0 == 1.0);
        s.v[1462] = if s.b[1462] { 1.0 } else { 0.0 };

        s.b[1463] = (1.0 == 1.0);
        s.v[1463] = if s.b[1463] { 1.0 } else { 0.0 };

        s.b[1464] = (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0));
        s.v[1464] = if s.b[1464] { 1.0 } else { 0.0 };

        s.b[1465] = (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0));
        s.v[1465] = if s.b[1465] { 1.0 } else { 0.0 };

        s.b[1466] = (s.v[971] == 0.0);
        s.v[1466] = if s.b[1466] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && s.b[1462]) && s.b[1463]) && s.b[1464]) && s.b[1466]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && s.b[1462]) && s.b[1463]) && s.b[1464]) && (!s.b[1466])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 971, s.v[183]);
        }

        s.b[1468] = ((s.v[971] == 0.0) || (s.v[449] == 0.0));
        s.v[1468] = if s.b[1468] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && s.b[1462]) && s.b[1463]) && (s.b[1465] && (!s.b[1464]))) && s.b[1468]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && s.b[1462]) && s.b[1463]) && (s.b[1465] && (!s.b[1464]))) && (!s.b[1468])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 971, (6.0 * s.v[449]));
        }

        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && s.b[1462]) && s.b[1463]) && (!(s.b[1464] || s.b[1465]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1469] = (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0));
        s.v[1469] = if s.b[1469] { 1.0 } else { 0.0 };

        s.b[1470] = (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0));
        s.v[1470] = if s.b[1470] { 1.0 } else { 0.0 };

        s.b[1471] = (s.v[971] == 0.0);
        s.v[1471] = if s.b[1471] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && s.b[1462]) && (!s.b[1463])) && s.b[1469]) && s.b[1471]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && s.b[1462]) && (!s.b[1463])) && s.b[1469]) && (!s.b[1471])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 971, s.v[183]);
        }

        s.b[1473] = ((s.v[971] == 0.0) || (s.v[449] == 0.0));
        s.v[1473] = if s.b[1473] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && s.b[1462]) && (!s.b[1463])) && (s.b[1470] && (!s.b[1469]))) && s.b[1473]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && s.b[1462]) && (!s.b[1463])) && (s.b[1470] && (!s.b[1469]))) && (!s.b[1473])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 971, (6.0 * s.v[449]));
        }

        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && s.b[1462]) && (!s.b[1463])) && (!(s.b[1469] || s.b[1470]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1474] = (0.0 == 1.0);
        s.v[1474] = if s.b[1474] { 1.0 } else { 0.0 };

        s.b[1475] = (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0));
        s.v[1475] = if s.b[1475] { 1.0 } else { 0.0 };

        s.b[1476] = (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0));
        s.v[1476] = if s.b[1476] { 1.0 } else { 0.0 };

        s.b[1477] = (s.v[969] == 0.0);
        s.v[1477] = if s.b[1477] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && (!s.b[1462])) && s.b[1474]) && s.b[1475]) && s.b[1477]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && (!s.b[1462])) && s.b[1474]) && s.b[1475]) && (!s.b[1477])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 969, s.v[183]);
        }

        s.b[1479] = ((s.v[969] == 0.0) || (s.v[449] == 0.0));
        s.v[1479] = if s.b[1479] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && (!s.b[1462])) && s.b[1474]) && (s.b[1476] && (!s.b[1475]))) && s.b[1479]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && (!s.b[1462])) && s.b[1474]) && (s.b[1476] && (!s.b[1475]))) && (!s.b[1479])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 969, (6.0 * s.v[449]));
        }

        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && (!s.b[1462])) && s.b[1474]) && (!(s.b[1475] || s.b[1476]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1480] = (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0));
        s.v[1480] = if s.b[1480] { 1.0 } else { 0.0 };

        s.b[1481] = (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0));
        s.v[1481] = if s.b[1481] { 1.0 } else { 0.0 };

        s.b[1482] = (s.v[969] == 0.0);
        s.v[1482] = if s.b[1482] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && (!s.b[1462])) && (!s.b[1474])) && s.b[1480]) && s.b[1482]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && (!s.b[1462])) && (!s.b[1474])) && s.b[1480]) && (!s.b[1482])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 969, s.v[183]);
        }

        s.b[1484] = ((s.v[969] == 0.0) || (s.v[449] == 0.0));
        s.v[1484] = if s.b[1484] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && (!s.b[1462])) && (!s.b[1474])) && (s.b[1481] && (!s.b[1480]))) && s.b[1484]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && (!s.b[1462])) && (!s.b[1474])) && (s.b[1481] && (!s.b[1480]))) && (!s.b[1484])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 969, (6.0 * s.v[449]));
        }

    }

    pub(super) fn stamp_transient_block_6(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && (!s.b[1462])) && (!s.b[1474])) && (!(s.b[1480] || s.b[1481]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1485] = (0.0 == 1.0);
        s.v[1485] = if s.b[1485] { 1.0 } else { 0.0 };

        s.b[1486] = (1.0 == 1.0);
        s.v[1486] = if s.b[1486] { 1.0 } else { 0.0 };

        s.b[1487] = (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0));
        s.v[1487] = if s.b[1487] { 1.0 } else { 0.0 };

        s.b[1488] = (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0));
        s.v[1488] = if s.b[1488] { 1.0 } else { 0.0 };

        s.b[1489] = (s.v[971] == 0.0);
        s.v[1489] = if s.b[1489] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1386] && (!(((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385])))) && s.b[1485]) && s.b[1486]) && s.b[1487]) && s.b[1489]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1386] && (!(((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385])))) && s.b[1485]) && s.b[1486]) && s.b[1487]) && (!s.b[1489])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 971, s.v[183]);
        }

        s.b[1491] = ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));
        s.v[1491] = if s.b[1491] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1386] && (!(((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385])))) && s.b[1485]) && s.b[1486]) && (s.b[1488] && (!s.b[1487]))) && s.b[1491]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1386] && (!(((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385])))) && s.b[1485]) && s.b[1486]) && (s.b[1488] && (!s.b[1487]))) && (!s.b[1491])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 971, (3.0 * (s.v[449] + s.v[451])));
        }

        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1386] && (!(((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385])))) && s.b[1485]) && s.b[1486]) && (!(s.b[1487] || s.b[1488]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1492] = (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0));
        s.v[1492] = if s.b[1492] { 1.0 } else { 0.0 };

        s.b[1493] = (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0));
        s.v[1493] = if s.b[1493] { 1.0 } else { 0.0 };

        s.b[1494] = (s.v[971] == 0.0);
        s.v[1494] = if s.b[1494] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1386] && (!(((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385])))) && s.b[1485]) && (!s.b[1486])) && s.b[1492]) && s.b[1494]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1386] && (!(((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385])))) && s.b[1485]) && (!s.b[1486])) && s.b[1492]) && (!s.b[1494])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 971, s.v[183]);
        }

        s.b[1496] = ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));
        s.v[1496] = if s.b[1496] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1386] && (!(((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385])))) && s.b[1485]) && (!s.b[1486])) && (s.b[1493] && (!s.b[1492]))) && s.b[1496]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1386] && (!(((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385])))) && s.b[1485]) && (!s.b[1486])) && (s.b[1493] && (!s.b[1492]))) && (!s.b[1496])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 971, (3.0 * (s.v[449] + s.v[451])));
        }

        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1386] && (!(((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385])))) && s.b[1485]) && (!s.b[1486])) && (!(s.b[1492] || s.b[1493]))) {
            s.store_scalar(973, 0.0);
        }

        if ((((!s.b[1374]) && s.b[1375]) && (s.b[1386] && (!(((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385])))) && (!s.b[1485])) {
            s.store_scalar(973, ((p.p438 * s.v[450]) / s.v[183]));
        }

        s.b[1497] = (0.0 == 1.0);
        s.v[1497] = if s.b[1497] { 1.0 } else { 0.0 };

        s.b[1498] = (1.0 == 1.0);
        s.v[1498] = if s.b[1498] { 1.0 } else { 0.0 };

        s.b[1499] = (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0));
        s.v[1499] = if s.b[1499] { 1.0 } else { 0.0 };

        s.b[1500] = (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0));
        s.v[1500] = if s.b[1500] { 1.0 } else { 0.0 };

        s.b[1501] = (s.v[971] == 0.0);
        s.v[1501] = if s.b[1501] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1387] && (!((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386])))) && s.b[1497]) && s.b[1498]) && s.b[1499]) && s.b[1501]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1387] && (!((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386])))) && s.b[1497]) && s.b[1498]) && s.b[1499]) && (!s.b[1501])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 971, s.v[183]);
        }

        s.b[1503] = ((s.v[971] == 0.0) || (s.v[449] == 0.0));
        s.v[1503] = if s.b[1503] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1387] && (!((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386])))) && s.b[1497]) && s.b[1498]) && (s.b[1500] && (!s.b[1499]))) && s.b[1503]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1387] && (!((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386])))) && s.b[1497]) && s.b[1498]) && (s.b[1500] && (!s.b[1499]))) && (!s.b[1503])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 971, (6.0 * s.v[449]));
        }

        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1387] && (!((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386])))) && s.b[1497]) && s.b[1498]) && (!(s.b[1499] || s.b[1500]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1504] = (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0));
        s.v[1504] = if s.b[1504] { 1.0 } else { 0.0 };

        s.b[1505] = (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0));
        s.v[1505] = if s.b[1505] { 1.0 } else { 0.0 };

        s.b[1506] = (s.v[971] == 0.0);
        s.v[1506] = if s.b[1506] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1387] && (!((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386])))) && s.b[1497]) && (!s.b[1498])) && s.b[1504]) && s.b[1506]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1387] && (!((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386])))) && s.b[1497]) && (!s.b[1498])) && s.b[1504]) && (!s.b[1506])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 971, s.v[183]);
        }

        s.b[1508] = ((s.v[971] == 0.0) || (s.v[449] == 0.0));
        s.v[1508] = if s.b[1508] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1387] && (!((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386])))) && s.b[1497]) && (!s.b[1498])) && (s.b[1505] && (!s.b[1504]))) && s.b[1508]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1387] && (!((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386])))) && s.b[1497]) && (!s.b[1498])) && (s.b[1505] && (!s.b[1504]))) && (!s.b[1508])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 971, (6.0 * s.v[449]));
        }

        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1387] && (!((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386])))) && s.b[1497]) && (!s.b[1498])) && (!(s.b[1504] || s.b[1505]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1509] = (s.v[969] == 0.0);
        s.v[1509] = if s.b[1509] { 1.0 } else { 0.0 };

        if (((((!s.b[1374]) && s.b[1375]) && (s.b[1387] && (!((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386])))) && (!s.b[1497])) && s.b[1509]) {
            s.store_scalar(973, 0.0);
        }

        if (((((!s.b[1374]) && s.b[1375]) && (s.b[1387] && (!((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386])))) && (!s.b[1497])) && (!s.b[1509])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[450]), 969, s.v[183]);
        }

        s.b[1510] = (0.0 == 1.0);
        s.v[1510] = if s.b[1510] { 1.0 } else { 0.0 };

        if ((((!s.b[1374]) && s.b[1375]) && (s.b[1388] && (!(((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387])))) && s.b[1510]) {
            s.store_scalar(973, ((p.p438 * s.v[450]) / s.v[183]));
        }

        s.b[1511] = (0.0 == 1.0);
        s.v[1511] = if s.b[1511] { 1.0 } else { 0.0 };

        s.b[1512] = (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0));
        s.v[1512] = if s.b[1512] { 1.0 } else { 0.0 };

        s.b[1513] = (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0));
        s.v[1513] = if s.b[1513] { 1.0 } else { 0.0 };

        s.b[1514] = (s.v[969] == 0.0);
        s.v[1514] = if s.b[1514] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1388] && (!(((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387])))) && (!s.b[1510])) && s.b[1511]) && s.b[1512]) && s.b[1514]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1388] && (!(((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387])))) && (!s.b[1510])) && s.b[1511]) && s.b[1512]) && (!s.b[1514])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 969, s.v[183]);
        }

        s.b[1516] = ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));
        s.v[1516] = if s.b[1516] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1388] && (!(((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387])))) && (!s.b[1510])) && s.b[1511]) && (s.b[1513] && (!s.b[1512]))) && s.b[1516]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1388] && (!(((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387])))) && (!s.b[1510])) && s.b[1511]) && (s.b[1513] && (!s.b[1512]))) && (!s.b[1516])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 969, (3.0 * (s.v[449] + s.v[451])));
        }

        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1388] && (!(((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387])))) && (!s.b[1510])) && s.b[1511]) && (!(s.b[1512] || s.b[1513]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1517] = (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0));
        s.v[1517] = if s.b[1517] { 1.0 } else { 0.0 };

        s.b[1518] = (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0));
        s.v[1518] = if s.b[1518] { 1.0 } else { 0.0 };

        s.b[1519] = (s.v[969] == 0.0);
        s.v[1519] = if s.b[1519] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1388] && (!(((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387])))) && (!s.b[1510])) && (!s.b[1511])) && s.b[1517]) && s.b[1519]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1388] && (!(((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387])))) && (!s.b[1510])) && (!s.b[1511])) && s.b[1517]) && (!s.b[1519])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 969, s.v[183]);
        }

        s.b[1521] = ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));
        s.v[1521] = if s.b[1521] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1388] && (!(((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387])))) && (!s.b[1510])) && (!s.b[1511])) && (s.b[1518] && (!s.b[1517]))) && s.b[1521]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1388] && (!(((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387])))) && (!s.b[1510])) && (!s.b[1511])) && (s.b[1518] && (!s.b[1517]))) && (!s.b[1521])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 969, (3.0 * (s.v[449] + s.v[451])));
        }

        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1388] && (!(((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387])))) && (!s.b[1510])) && (!s.b[1511])) && (!(s.b[1517] || s.b[1518]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1522] = (0.0 == 1.0);
        s.v[1522] = if s.b[1522] { 1.0 } else { 0.0 };

        s.b[1523] = (s.v[971] == 0.0);
        s.v[1523] = if s.b[1523] { 1.0 } else { 0.0 };

        if (((((!s.b[1374]) && s.b[1375]) && (s.b[1389] && (!((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388])))) && s.b[1522]) && s.b[1523]) {
            s.store_scalar(973, 0.0);
        }

        if (((((!s.b[1374]) && s.b[1375]) && (s.b[1389] && (!((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388])))) && s.b[1522]) && (!s.b[1523])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[450]), 971, s.v[183]);
        }

        s.b[1524] = (0.0 == 1.0);
        s.v[1524] = if s.b[1524] { 1.0 } else { 0.0 };

        s.b[1525] = (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0));
        s.v[1525] = if s.b[1525] { 1.0 } else { 0.0 };

        s.b[1526] = (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0));
        s.v[1526] = if s.b[1526] { 1.0 } else { 0.0 };

        s.b[1527] = (s.v[969] == 0.0);
        s.v[1527] = if s.b[1527] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1389] && (!((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388])))) && (!s.b[1522])) && s.b[1524]) && s.b[1525]) && s.b[1527]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1389] && (!((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388])))) && (!s.b[1522])) && s.b[1524]) && s.b[1525]) && (!s.b[1527])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 969, s.v[183]);
        }

        s.b[1529] = ((s.v[969] == 0.0) || (s.v[449] == 0.0));
        s.v[1529] = if s.b[1529] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1389] && (!((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388])))) && (!s.b[1522])) && s.b[1524]) && (s.b[1526] && (!s.b[1525]))) && s.b[1529]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1389] && (!((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388])))) && (!s.b[1522])) && s.b[1524]) && (s.b[1526] && (!s.b[1525]))) && (!s.b[1529])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 969, (6.0 * s.v[449]));
        }

        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1389] && (!((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388])))) && (!s.b[1522])) && s.b[1524]) && (!(s.b[1525] || s.b[1526]))) {
            s.store_scalar(973, 0.0);
        }

        s.b[1530] = (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0));
        s.v[1530] = if s.b[1530] { 1.0 } else { 0.0 };

        s.b[1531] = (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0));
        s.v[1531] = if s.b[1531] { 1.0 } else { 0.0 };

        s.b[1532] = (s.v[969] == 0.0);
        s.v[1532] = if s.b[1532] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1389] && (!((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388])))) && (!s.b[1522])) && (!s.b[1524])) && s.b[1530]) && s.b[1532]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1389] && (!((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388])))) && (!s.b[1522])) && (!s.b[1524])) && s.b[1530]) && (!s.b[1532])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[449]), 969, s.v[183]);
        }

        s.b[1534] = ((s.v[969] == 0.0) || (s.v[449] == 0.0));
        s.v[1534] = if s.b[1534] { 1.0 } else { 0.0 };

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1389] && (!((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388])))) && (!s.b[1522])) && (!s.b[1524])) && (s.b[1531] && (!s.b[1530]))) && s.b[1534]) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1389] && (!((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388])))) && (!s.b[1522])) && (!s.b[1524])) && (s.b[1531] && (!s.b[1530]))) && (!s.b[1534])) {
            s.store_div_from_scalar_scaled_input(973, (p.p438 * s.v[183]), 969, (6.0 * s.v[449]));
        }

        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1389] && (!((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388])))) && (!s.b[1522])) && (!s.b[1524])) && (!(s.b[1530] || s.b[1531]))) {
            s.store_scalar(973, 0.0);
        }

        if (((!s.b[1374]) && s.b[1375]) && (s.b[1390] && (!(((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388]) || s.b[1389])))) {
            s.store_scalar(973, ((p.p438 * s.v[450]) / s.v[183]));
        }

        s.b[1535] = (0.0 == 1.0);
        s.v[1535] = if s.b[1535] { 1.0 } else { 0.0 };

        if ((((!s.b[1374]) && s.b[1375]) && (s.b[1391] && (!((((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388]) || s.b[1389]) || s.b[1390])))) && s.b[1535]) {
            s.store_scalar(973, (((0.5 * p.p438) * s.v[449]) / s.v[183]));
        }

        s.b[1536] = (p.p2 == 2.0);
        s.v[1536] = if s.b[1536] { 1.0 } else { 0.0 };

        if (((((!s.b[1374]) && s.b[1375]) && (s.b[1391] && (!((((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388]) || s.b[1389]) || s.b[1390])))) && s.b[1535]) && s.b[1536]) {
            s.store_scalar(972, 0.0);
        }

        if (((((!s.b[1374]) && s.b[1375]) && (s.b[1391] && (!((((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388]) || s.b[1389]) || s.b[1390])))) && s.b[1535]) && (!s.b[1536])) {
            s.store_scalar(972, ((p.p438 * s.v[449]) / (s.v[183] * (p.p2 - 2.0))));
        }

        if ((((!s.b[1374]) && s.b[1375]) && (s.b[1391] && (!((((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388]) || s.b[1389]) || s.b[1390])))) && (!s.b[1535])) {
            s.store_scalar(973, 0.0);
            s.store_scalar(972, ((p.p438 * s.v[449]) / (s.v[183] * p.p2)));
        }

        s.b[1537] = (0.0 == 1.0);
        s.v[1537] = if s.b[1537] { 1.0 } else { 0.0 };

        if ((((!s.b[1374]) && s.b[1375]) && (s.b[1392] && (!(((((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388]) || s.b[1389]) || s.b[1390]) || s.b[1391])))) && s.b[1537]) {
            s.store_scalar(973, 0.0);
            s.store_scalar(972, ((p.p438 * s.v[449]) / (s.v[183] * p.p2)));
        }

        if ((((!s.b[1374]) && s.b[1375]) && (s.b[1392] && (!(((((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388]) || s.b[1389]) || s.b[1390]) || s.b[1391])))) && (!s.b[1537])) {
            s.store_scalar(973, (((0.5 * p.p438) * s.v[449]) / s.v[183]));
        }

        s.b[1538] = (p.p2 == 2.0);
        s.v[1538] = if s.b[1538] { 1.0 } else { 0.0 };

        if (((((!s.b[1374]) && s.b[1375]) && (s.b[1392] && (!(((((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388]) || s.b[1389]) || s.b[1390]) || s.b[1391])))) && (!s.b[1537])) && s.b[1538]) {
            s.store_scalar(972, 0.0);
        }

        if (((((!s.b[1374]) && s.b[1375]) && (s.b[1392] && (!(((((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388]) || s.b[1389]) || s.b[1390]) || s.b[1391])))) && (!s.b[1537])) && (!s.b[1538])) {
            s.store_scalar(972, ((p.p438 * s.v[449]) / (s.v[183] * (p.p2 - 2.0))));
        }

        if (((!s.b[1374]) && s.b[1375]) && (!((((((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388]) || s.b[1389]) || s.b[1390]) || s.b[1391]) || s.b[1392]))) {
            s.store_scalar(972, 0.0);
        }

        s.b[1539] = (s.v[972] <= 0.0);
        s.v[1539] = if s.b[1539] { 1.0 } else { 0.0 };

        if (((!s.b[1374]) && s.b[1375]) && s.b[1539]) {
            s.copy_ad(453, 973);
        }

        s.b[1540] = (s.v[973] <= 0.0);
        s.v[1540] = if s.b[1540] { 1.0 } else { 0.0 };

        if ((((!s.b[1374]) && s.b[1375]) && (!s.b[1539])) && s.b[1540]) {
            s.copy_ad(453, 972);
        }

        if ((((!s.b[1374]) && s.b[1375]) && (!s.b[1539])) && (!s.b[1540])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(453, 972, 973, 1.0, 972, 1.0, 973, 1.0, 1.0);
        }

        if ((!s.b[1374]) && (!s.b[1375])) {
            s.store_scalar(453, 0.0);
        }

        s.b[1542] = (p.p33 == 0.0);
        s.v[1542] = if s.b[1542] { 1.0 } else { 0.0 };

        s.b[1543] = (s.v[452] < p.p1347);
        s.v[1543] = if s.b[1543] { 1.0 } else { 0.0 };

        if (s.b[1542] && s.b[1543]) {
            s.store_scalar(452, 0.0);
        }

        s.b[1544] = (s.v[453] < p.p1347);
        s.v[1544] = if s.b[1544] { 1.0 } else { 0.0 };

        if (s.b[1542] && s.b[1544]) {
            s.store_scalar(453, 0.0);
        }

        s.b[1545] = (s.v[452] <= p.p1347);
        s.v[1545] = if s.b[1545] { 1.0 } else { 0.0 };

        if ((!s.b[1542]) && s.b[1545]) {
            s.store_scalar(452, p.p1347);
        }

        s.b[1546] = (s.v[453] <= p.p1347);
        s.v[1546] = if s.b[1546] { 1.0 } else { 0.0 };

        if ((!s.b[1542]) && s.b[1546]) {
            s.store_scalar(453, p.p1347);
        }

        s.b[1547] = (p.p33 == 1.0);
        s.v[1547] = if s.b[1547] { 1.0 } else { 0.0 };

        s.b[1548] = (s.v[773] <= 0.0);
        s.v[1548] = if s.b[1548] { 1.0 } else { 0.0 };

        if (s.b[1547] && s.b[1548]) {
            s.store_scalar(773, 0.0);
        }

        s.b[1549] = (s.v[772] <= 0.0);
        s.v[1549] = if s.b[1549] { 1.0 } else { 0.0 };

        if (s.b[1547] && s.b[1549]) {
            s.store_scalar(772, 0.0);
        }

        s.b[1550] = (s.v[775] <= 0.0);
        s.v[1550] = if s.b[1550] { 1.0 } else { 0.0 };

        if (s.b[1547] && s.b[1550]) {
            s.store_scalar(775, 0.0);
        }

        s.b[1551] = (s.v[774] <= 0.0);
        s.v[1551] = if s.b[1551] { 1.0 } else { 0.0 };

        if (s.b[1547] && s.b[1551]) {
            s.store_scalar(774, 0.0);
        }

        s.b[1552] = (s.v[777] <= 0.0);
        s.v[1552] = if s.b[1552] { 1.0 } else { 0.0 };

        if ((!s.b[1547]) && s.b[1552]) {
            s.store_scalar(777, 0.0);
        }

        s.b[1553] = (s.v[776] <= 0.0);
        s.v[1553] = if s.b[1553] { 1.0 } else { 0.0 };

        if ((!s.b[1547]) && s.b[1553]) {
            s.store_scalar(776, 0.0);
        }

        s.v[465] = ((p.p900 * (p.p21 + ((s.v[189] / 3.0) / p.p22))) / ((p.p22 * p.p2) * (s.v[261] - p.p899)));

        s.b[1554] = (s.v[465] > 0.0);
        s.v[1554] = if s.b[1554] { 1.0 } else { 0.0 };

        if s.b[1554] {
            s.store_scalar(465, (1.0 / s.v[465]));
        }

        if (!s.b[1554]) {
            s.store_scalar(465, 1000.0);
        }

        s.v[167] = (p.p76 * p.p76);

        s.store_scale(168, 822, p.p76);

        s.store_square(169, 168);

        s.store_scaled_limited_exp_scaled_input(492, 826, ((((p.p722 / p.p76)).max(1e-38)) as f64).ln(), 1.0 / (s.v[167]));

        s.store_div_ad_lhs(493, A::limited_exp(A::mul(s.ad_value(826), A::ln(A::max_with_scalar(A::div_from_scalar(p.p722, s.ad_value(168)), 1e-38)))), 169);

        s.v[487] = (if (p.p30 == 1.0) { p.p703 } else { p.p702 });

        s.v[488] = (if (p.p30 == 1.0) { p.p705 } else { p.p704 });

        s.store_scale(489, 493, (s.v[487] * ((s.v[183] / p.p1373) + p.p1378)));

        s.store_scale(490, 493, (s.v[487] * ((s.v[183] / p.p1373) + p.p1377)));

        s.store_scale(491, 822, ((-s.v[488]) * p.p76));

        s.store_scale(487, 492, (s.v[487] * (((s.v[183] / p.p1373) * s.v[184]) + (p.p1381 / p.p2))));

        s.v[488] = ((-s.v[488]) * p.p76);

        s.v[191] = (p.p1101 + s.v[183]);

    }

    pub(super) fn stamp_transient_block_7(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.b[1559] = (((p.p41 != 0.0) && (p.p1099 > 0.0)) && (s.v[191] > 0.0));
        s.v[1559] = if s.b[1559] { 1.0 } else { 0.0 };

        if s.b[1559] {
            s.store_scalar(1015, ((s.v[191] * p.p2) / p.p1099));
            s.store_scalar(1016, ((p.p1100 * s.v[191]) * p.p2));
        }

        if (!s.b[1559]) {
            s.store_scalar(1015, 1.0);
            s.store_scalar(1016, 0.0);
        }

        s.b[1560] = (p.p1028 <= (-273.15));
        s.v[1560] = if s.b[1560] { 1.0 } else { 0.0 };

        if s.b[1560] {
            s.store_scalar(167, (300.15 - 273.15));
            s.store_scalar(636, 300.15);
        }

        if (!s.b[1560]) {
            s.store_scalar(636, (p.p1028 + 273.15));
        }

        s.v[635] = (ctx_temp + p.p23);

        s.b[1561] = ((p.p41 != 0.0) && (p.p1099 > 0.0));
        s.v[1561] = if s.b[1561] { 1.0 } else { 0.0 };

        s.b[1562] = ((p.p40 != 0.0) && (!true));
        s.v[1562] = if s.b[1562] { 1.0 } else { 0.0 };

        s.b[1563] = true;
        s.v[1563] = if s.b[1563] { 1.0 } else { 0.0 };

        if ((s.b[1561] && s.b[1562]) && s.b[1563]) {
            s.store_voltage(634, ctx, nodes, Some(4), None);
        }

        if ((s.b[1561] && s.b[1562]) && (!s.b[1563])) {
            s.store_voltage(634, ctx, nodes, Some(5), None);
        }

        if (s.b[1561] && (!s.b[1562])) {
            s.store_voltage(634, ctx, nodes, Some(5), None);
        }

        if (!s.b[1561]) {
            s.store_scalar(634, 0.0);
        }

        s.store_offset(635, 634, s.v[635]);

        s.store_scale(271, 635, s.v[1048]);

        s.store_div_from_scalar(272, 1.0, 271);

        s.store_div(639, 635, 636);

        s.store_sub(640, 635, 636);

        s.store_scale(637, 635, s.v[1048]);

        s.store_scale(638, 636, s.v[1048]);

        s.store_sub_from_scalar_ad(190, p.p108, A::div_scaled_product_offset_denominator(s.ad_value(635), s.ad_value(635), p.p1029, s.ad_value(635), p.p1030, 1.0));

        s.store_mul_ad(168, A::div(s.ad_value(635), s.ad_value(636)), A::sqrt(A::div(s.ad_value(635), s.ad_value(636))));

        s.store_mul_scaled_limited_exp_ad_rhs(182, 168, p.p107, A::sub(A::div_scaled_inputs(s.ad_value(190), 1.0, s.ad_value(638), 2.0), A::div_scaled_inputs(s.ad_value(190), 1.0, s.ad_value(637), 2.0)));

        s.b[1564] = (((p.p41 != 0.0) && (p.p1099 > 0.0)) && (s.v[191] > 0.0));
        s.v[1564] = if s.b[1564] { 1.0 } else { 0.0 };

        if s.b[1564] {
            s.store_ln_ad(167, A::max_with_scalar(A::div(s.ad_value(706), s.ad_value(182)), 1e-38));
            s.store_sqrt_square_offset(251, 167, 1e-6);
        }

        if (!s.b[1564]) {
            s.store_ln_ad(251, A::max_with_scalar(A::div(s.ad_value(706), s.ad_value(182)), 1e-38));
        }

        s.b[1565] = (((p.p41 != 0.0) && (p.p1099 > 0.0)) && (s.v[191] > 0.0));
        s.v[1565] = if s.b[1565] { 1.0 } else { 0.0 };

        if s.b[1565] {
            s.store_ln_ad(167, A::max_with_scalar(A::div_scaled_product(s.ad_value(953), s.ad_value(705), 1.0, A::square(s.ad_value(182)), 1.0), 1e-38));
            s.store_sqrt_square_offset(942, 167, 1e-6);
        }

        if (!s.b[1565]) {
            s.store_ln_ad(942, A::max_with_scalar(A::div_scaled_product(s.ad_value(953), s.ad_value(705), 1.0, A::square(s.ad_value(182)), 1.0), 1e-38));
        }

        s.b[1566] = (s.v[704] > 0.0);
        s.v[1566] = if s.b[1566] { 1.0 } else { 0.0 };

        if s.b[1566] {
            s.store_offset_product3(219, s.ad_value(379), s.ad_value(271), A::ln(A::max_with_scalar(A::div(s.ad_value(704), s.ad_value(705)), 1e-38)), -1.0, p.p5);
        }

        if (!s.b[1566]) {
            s.store_scalar(219, 0.0);
        }

        s.store_max_with_scalar_ad(298, A::add(A::offset(A::mul(s.ad_value(271), s.ad_value(251)), 0.4), s.ad_value(729)), 0.4);

        s.store_sqrt(299, 298);

        s.store_sqrt_div_from_scalar_ad(277, (2.0 * s.v[180]), A::scale(s.ad_value(706), 1.602176462e-19));

        s.store_sqrt_scaled_input(300, 782, ((s.v[180] / s.v[181]) * p.p76));

        s.store_mul_add_scaled_inputs_rhs(665, 720, A::scale_offset(s.ad_value(639), p.p1031, (((((-1.0)) * (p.p1031))) + (1.0))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(639), p.p1031, (((((-1.0)) * (p.p1031))) + (1.0))), ((4.0 * 0.001) * 0.001)), 0.5);

        s.store_mul_scale_offset_rhs(663, 730, 639, p.p1059, (((((-1.0)) * (p.p1059))) + (1.0)));

        s.b[1577] = (p.p35 != 0.0);
        s.v[1577] = if s.b[1577] { 1.0 } else { 0.0 };

        if s.b[1577] {
            s.store_mul_scale_offset_rhs(664, 731, 639, p.p1059, (((((-1.0)) * (p.p1059))) + (1.0)));
        }

        s.v[338] = (if (p.p30 != 1.0) { (0.3333333333333333 * p.p347) } else { (0.5 * p.p347) });

        s.store_mul_pow_ad_rhs(641, 738, s.ad_value(639), s.ad_value(796));

        s.store_mul_add_scaled_inputs_rhs(643, 748, A::offset(A::mul(s.ad_value(797), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_offset_square_offset(A::mul(s.ad_value(797), s.ad_value(640)), ((1.0) + ((-1e-6))), ((4.0 * 0.001) * 0.001)), 0.5);

        s.store_mul_add_scaled_inputs_rhs(645, 758, A::offset(A::mul(s.ad_value(798), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_offset_square_offset(A::mul(s.ad_value(798), s.ad_value(640)), ((1.0) + ((-1e-6))), ((4.0 * 0.001) * 0.001)), 0.5);

        s.store_mul_pow_ad_rhs(647, 752, s.ad_value(639), s.ad_value(799));

        s.store_mul_pow_ad_rhs(649, 755, s.ad_value(639), s.ad_value(800));

        s.store_mul_add_scaled_inputs_rhs(651, 751, A::offset(A::mul_offset_rhs(s.ad_value(805), s.ad_value(639), (-1.0)), 1.0), 0.5, A::sqrt_offset_square_offset(A::mul_offset_rhs(s.ad_value(805), s.ad_value(639), (-1.0)), 1.0, ((4.0 * 0.001) * 0.001)), 0.5);

        s.b[1578] = (p.p35 != 0.0);
        s.v[1578] = if s.b[1578] { 1.0 } else { 0.0 };

        if s.b[1578] {
            s.store_mul_pow_ad_rhs(642, 739, s.ad_value(639), s.ad_value(796));
            s.store_mul_add_scaled_inputs_rhs(644, 749, A::offset(A::mul(s.ad_value(797), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_offset_square_offset(A::mul(s.ad_value(797), s.ad_value(640)), ((1.0) + ((-1e-6))), ((4.0 * 0.001) * 0.001)), 0.5);
            s.store_mul_add_scaled_inputs_rhs(646, 759, A::offset(A::mul(s.ad_value(798), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_offset_square_offset(A::mul(s.ad_value(798), s.ad_value(640)), ((1.0) + ((-1e-6))), ((4.0 * 0.001) * 0.001)), 0.5);
            s.store_mul_pow_ad_rhs(648, 753, s.ad_value(639), s.ad_value(799));
            s.store_mul_pow_ad_rhs(650, 756, s.ad_value(639), s.ad_value(800));
        }

        s.store_pow_ad(652, s.ad_value(639), s.ad_value(801));

        s.store_mul_pow_ad_rhs(653, 741, s.ad_value(639), A::neg(s.ad_value(802)));

        s.b[1579] = (s.v[653] < 100.0);
        s.v[1579] = if s.b[1579] { 1.0 } else { 0.0 };

        if s.b[1579] {
            s.store_scalar(653, 100.0);
        }

        s.b[1580] = (p.p35 != 0.0);
        s.v[1580] = if s.b[1580] { 1.0 } else { 0.0 };

        if s.b[1580] {
            s.store_mul_pow_ad_rhs(654, 745, s.ad_value(639), A::neg(s.ad_value(802)));
        }

        s.b[1581] = (s.v[654] < 100.0);
        s.v[1581] = if s.b[1581] { 1.0 } else { 0.0 };

        if (s.b[1580] && s.b[1581]) {
            s.store_scalar(654, 100.0);
        }

        s.store_mul_pow_ad_rhs(655, 747, s.ad_value(639), A::neg(s.ad_value(802)));

        s.b[1582] = (s.v[655] < 100.0);
        s.v[1582] = if s.b[1582] { 1.0 } else { 0.0 };

        if s.b[1582] {
            s.store_scalar(655, 100.0);
        }

        let assign13980_ad_e19180: A = A::add(A::offset(A::mul(A::div_from_scalar(1.0, s.ad_value(737)), A::scale_offset(s.ad_value(640), p.p1069, 1.0)), (-2.0)), A::sqrt_offset_square_offset(A::mul(A::div_from_scalar(1.0, s.ad_value(737)), A::scale_offset(s.ad_value(640), p.p1069, 1.0)), (-2.0), ((4.0 * 0.001) * 0.001)));
        s.store_div_from_scalar_ad(656, 1.0, A::scale_offset(assign13980_ad_e19180, 0.5, 2.0));

        s.store_mul_add_scaled_inputs_rhs(657, 778, A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(803), s.ad_value(640))), (-1e-6)), 0.5, A::sqrt_offset_square_offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(803), s.ad_value(640))), (-1e-6), ((4.0 * 0.001) * 0.001)), 0.5);

        s.b[1583] = (p.p35 != 0.0);
        s.v[1583] = if s.b[1583] { 1.0 } else { 0.0 };

        if s.b[1583] {
            s.store_mul_add_scaled_inputs_rhs(658, 779, A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(803), s.ad_value(640))), (-1e-6)), 0.5, A::sqrt_offset_square_offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(803), s.ad_value(640))), (-1e-6), ((4.0 * 0.001) * 0.001)), 0.5);
        }

        s.store_mul_add_scaled_inputs_rhs(330, 328, A::offset(A::mul(s.ad_value(329), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_offset_square_offset(A::mul(s.ad_value(329), s.ad_value(640)), ((1.0) + ((-1e-6))), ((4.0 * 0.001) * 0.001)), 0.5);

        s.store_mul_add_scaled_inputs_rhs(333, 331, A::offset(A::mul(s.ad_value(332), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_offset_square_offset(A::mul(s.ad_value(332), s.ad_value(640)), ((1.0) + ((-1e-6))), ((4.0 * 0.001) * 0.001)), 0.5);

        s.store_mul_pow_ad_rhs(659, 866, s.ad_value(639), s.ad_value(804));

        s.store_add_scaled_offset_product_rhs(660, 893, 1.0, 900, 639, (-1.0), 1.0);

        s.store_add_scaled_offset_product_rhs(661, 897, 1.0, 901, 639, (-1.0), 1.0);

        s.store_limited_exp_ad(662, A::mul(s.ad_value(827), A::ln(A::max_with_scalar(s.ad_value(639), 1e-38))));

        s.store_mul_add_scaled_inputs_rhs(832, 828, A::offset(A::mul(s.ad_value(830), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_offset_square_offset(A::mul(s.ad_value(830), s.ad_value(640)), ((1.0) + ((-1e-6))), ((4.0 * 0.001) * 0.001)), 0.5);

        s.store_mul_add_scaled_inputs_rhs(833, 829, A::offset(A::mul(s.ad_value(831), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_offset_square_offset(A::mul(s.ad_value(831), s.ad_value(640)), ((1.0) + ((-1e-6))), ((4.0 * 0.001) * 0.001)), 0.5);

        s.store_mul_add_scaled_inputs_rhs(858, 856, A::offset(A::mul(s.ad_value(857), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_offset_square_offset(A::mul(s.ad_value(857), s.ad_value(640)), ((1.0) + ((-1e-6))), ((4.0 * 0.001) * 0.001)), 0.5);

        s.store_mul_add_scaled_inputs_rhs(861, 859, A::offset(A::mul(s.ad_value(860), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_offset_square_offset(A::mul(s.ad_value(860), s.ad_value(640)), ((1.0) + ((-1e-6))), ((4.0 * 0.001) * 0.001)), 0.5);

        s.store_mul_add_scaled_inputs_rhs(864, 862, A::offset(A::mul(s.ad_value(863), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_offset_square_offset(A::mul(s.ad_value(863), s.ad_value(640)), ((1.0) + ((-1e-6))), ((4.0 * 0.001) * 0.001)), 0.5);

        s.store_scaled_add_sqrt_square_offset_ad(666, A::scale_offset(s.ad_value(640), p.p1093, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001), (0.5 * p.p901));

        s.store_scaled_add_sqrt_square_offset_ad(669, A::scale_offset(s.ad_value(640), p.p1093, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001), (0.5 * p.p902));

        s.store_scaled_add_sqrt_square_offset_ad(667, A::scale_offset(s.ad_value(640), p.p1094, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001), (0.5 * p.p903));

        s.store_scaled_add_sqrt_square_offset_ad(670, A::scale_offset(s.ad_value(640), p.p1094, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001), (0.5 * p.p904));

        s.store_scaled_add_sqrt_square_offset_ad(671, A::scale_offset(s.ad_value(640), p.p1095, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001), (0.5 * p.p905));

        s.store_scaled_add_sqrt_square_offset_ad(668, A::scale_offset(s.ad_value(640), p.p1095, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001), (0.5 * p.p906));

        s.store_offset_add_scaled_inputs(672, A::offset(A::sub_from_scalar(p.p907, A::scale(s.ad_value(640), p.p1096)), (-0.01)), 0.5, A::sqrt_offset_square_offset(A::sub_from_scalar(p.p907, A::scale(s.ad_value(640), p.p1096)), (-0.01), ((4.0 * 0.001) * 0.001)), 0.5, 0.01);

        s.store_offset_add_scaled_inputs(675, A::offset(A::sub_from_scalar(p.p908, A::scale(s.ad_value(640), p.p1096)), (-0.01)), 0.5, A::sqrt_offset_square_offset(A::sub_from_scalar(p.p908, A::scale(s.ad_value(640), p.p1096)), (-0.01), ((4.0 * 0.001) * 0.001)), 0.5, 0.01);

        s.store_offset_add_scaled_inputs(673, A::offset(A::sub_from_scalar(p.p909, A::scale(s.ad_value(640), p.p1097)), (-0.01)), 0.5, A::sqrt_offset_square_offset(A::sub_from_scalar(p.p909, A::scale(s.ad_value(640), p.p1097)), (-0.01), ((4.0 * 0.001) * 0.001)), 0.5, 0.01);

        s.store_offset_add_scaled_inputs(676, A::offset(A::sub_from_scalar(p.p910, A::scale(s.ad_value(640), p.p1097)), (-0.01)), 0.5, A::sqrt_offset_square_offset(A::sub_from_scalar(p.p910, A::scale(s.ad_value(640), p.p1097)), (-0.01), ((4.0 * 0.001) * 0.001)), 0.5, 0.01);

        s.store_offset_add_scaled_inputs(674, A::offset(A::sub_from_scalar(p.p911, A::scale(s.ad_value(640), p.p1098)), (-0.01)), 0.5, A::sqrt_offset_square_offset(A::sub_from_scalar(p.p911, A::scale(s.ad_value(640), p.p1098)), (-0.01), ((4.0 * 0.001) * 0.001)), 0.5, 0.01);

        s.store_offset_add_scaled_inputs(677, A::offset(A::sub_from_scalar(p.p912, A::scale(s.ad_value(640), p.p1098)), (-0.01)), 0.5, A::sqrt_offset_square_offset(A::sub_from_scalar(p.p912, A::scale(s.ad_value(640), p.p1098)), (-0.01), ((4.0 * 0.001) * 0.001)), 0.5, 0.01);

        s.b[1584] = (p.p8 < 9.0);
        s.v[1584] = if s.b[1584] { 1.0 } else { 0.0 };

        s.b[1585] = ((p.p2 % 2.0) != 0.0);
        s.v[1585] = if s.b[1585] { 1.0 } else { 0.0 };

        if (s.b[1584] && s.b[1585]) {
            s.store_scalar(969, 1.0);
            s.store_scalar(971, 1.0);
            s.store_scalar(968, (2.0 * (((p.p2 - 1.0) / 2.0)).max(0.0)));
            s.copy_ad(970, 968);
        }

        s.b[1586] = (p.p6 == 1.0);
        s.v[1586] = if s.b[1586] { 1.0 } else { 0.0 };

        if ((s.b[1584] && (!s.b[1585])) && s.b[1586]) {
            s.store_scalar(969, 2.0);
            s.store_scalar(968, (2.0 * (((p.p2 / 2.0) - 1.0)).max(0.0)));
            s.store_scalar(971, 0.0);
            s.store_scalar(970, p.p2);
        }

        if ((s.b[1584] && (!s.b[1585])) && (!s.b[1586])) {
            s.store_scalar(969, 0.0);
            s.store_scalar(968, p.p2);
            s.store_scalar(971, 2.0);
            s.store_scalar(970, (2.0 * (((p.p2 / 2.0) - 1.0)).max(0.0)));
        }

        s.v[167] = (s.v[449] + s.v[451]);

        s.v[168] = (s.v[449] + s.v[449]);

        s.v[169] = (s.v[450] + s.v[450]);

        s.v[155] = ((s.v[167] + s.v[167]) + s.v[189]);

        s.v[156] = ((s.v[167] + s.v[167]) + s.v[189]);

        s.v[157] = s.v[168];

        s.v[158] = s.v[168];

        s.v[159] = s.v[169];

        s.v[160] = s.v[169];

        s.v[161] = (s.v[167] * s.v[189]);

        s.v[162] = (s.v[167] * s.v[189]);

        s.v[163] = (s.v[449] * s.v[189]);

        s.v[164] = (s.v[449] * s.v[189]);

        s.v[165] = (s.v[450] * s.v[189]);

        s.v[166] = (s.v[450] * s.v[189]);

        s.b[1587] = (p.p8 == 0.0);
        s.v[1587] = if s.b[1587] { 1.0 } else { 0.0 };

        s.b[1588] = (p.p8 == 1.0);
        s.v[1588] = if s.b[1588] { 1.0 } else { 0.0 };

        s.b[1589] = (p.p8 == 2.0);
        s.v[1589] = if s.b[1589] { 1.0 } else { 0.0 };

        s.b[1590] = (p.p8 == 3.0);
        s.v[1590] = if s.b[1590] { 1.0 } else { 0.0 };

        s.b[1591] = (p.p8 == 4.0);
        s.v[1591] = if s.b[1591] { 1.0 } else { 0.0 };

        s.b[1592] = (p.p8 == 5.0);
        s.v[1592] = if s.b[1592] { 1.0 } else { 0.0 };

        s.b[1593] = (p.p8 == 6.0);
        s.v[1593] = if s.b[1593] { 1.0 } else { 0.0 };

        s.b[1594] = (p.p8 == 7.0);
        s.v[1594] = if s.b[1594] { 1.0 } else { 0.0 };

        s.b[1595] = (p.p8 == 8.0);
        s.v[1595] = if s.b[1595] { 1.0 } else { 0.0 };

        s.b[1596] = (p.p8 == 9.0);
        s.v[1596] = if s.b[1596] { 1.0 } else { 0.0 };

        s.b[1597] = (p.p8 == 10.0);
        s.v[1597] = if s.b[1597] { 1.0 } else { 0.0 };

        if s.b[1587] {
            s.store_add_scaled_inputs(461, 971, s.v[155], 970, s.v[157]);
            s.store_add_scaled_inputs(462, 969, s.v[156], 968, s.v[158]);
            s.store_add_scaled_inputs(459, 971, s.v[161], 970, s.v[163]);
            s.store_add_scaled_inputs(460, 969, s.v[162], 968, s.v[164]);
        }

        if (s.b[1588] && (!s.b[1587])) {
            s.store_add_scaled_inputs(461, 971, s.v[155], 970, s.v[157]);
            s.store_scaled_add(462, 969, 968, s.v[158]);
            s.store_add_scaled_inputs(459, 971, s.v[161], 970, s.v[163]);
            s.store_scaled_add(460, 969, 968, s.v[164]);
        }

        if (s.b[1589] && (!(s.b[1587] || s.b[1588]))) {
            s.store_scaled_add(461, 971, 970, s.v[157]);
            s.store_add_scaled_inputs(462, 969, s.v[156], 968, s.v[158]);
            s.store_scaled_add(459, 971, 970, s.v[163]);
            s.store_add_scaled_inputs(460, 969, s.v[162], 968, s.v[164]);
        }

        if (s.b[1590] && (!((s.b[1587] || s.b[1588]) || s.b[1589]))) {
            s.store_scaled_add(461, 971, 970, s.v[157]);
            s.store_scaled_add(462, 969, 968, s.v[158]);
            s.store_scaled_add(459, 971, 970, s.v[163]);
            s.store_scaled_add(460, 969, 968, s.v[164]);
        }

        if (s.b[1591] && (!(((s.b[1587] || s.b[1588]) || s.b[1589]) || s.b[1590]))) {
            s.store_add_scaled_inputs(461, 971, s.v[155], 970, s.v[157]);
            s.store_add_scaled_inputs(462, 969, s.v[160], 968, s.v[158]);
            s.store_add_scaled_inputs(459, 971, s.v[161], 970, s.v[163]);
            s.store_add_scaled_inputs(460, 969, s.v[166], 968, s.v[164]);
        }

        if (s.b[1592] && (!((((s.b[1587] || s.b[1588]) || s.b[1589]) || s.b[1590]) || s.b[1591]))) {
            s.store_scaled_add(461, 971, 970, s.v[157]);
            s.store_add_scaled_inputs(462, 969, s.v[160], 968, s.v[158]);
            s.store_scaled_add(459, 971, 970, s.v[163]);
            s.store_add_scaled_inputs(460, 969, s.v[166], 968, s.v[164]);
        }

        if (s.b[1593] && (!(((((s.b[1587] || s.b[1588]) || s.b[1589]) || s.b[1590]) || s.b[1591]) || s.b[1592]))) {
            s.store_add_scaled_inputs(461, 971, s.v[159], 970, s.v[157]);
            s.store_add_scaled_inputs(462, 969, s.v[156], 968, s.v[158]);
            s.store_add_scaled_inputs(459, 971, s.v[165], 970, s.v[163]);
            s.store_add_scaled_inputs(460, 969, s.v[162], 968, s.v[164]);
        }

        if (s.b[1594] && (!((((((s.b[1587] || s.b[1588]) || s.b[1589]) || s.b[1590]) || s.b[1591]) || s.b[1592]) || s.b[1593]))) {
            s.store_add_scaled_inputs(461, 971, s.v[159], 970, s.v[157]);
            s.store_scaled_add(462, 969, 968, s.v[158]);
        }

    }

    pub(super) fn stamp_transient_block_8(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1594] && (!((((((s.b[1587] || s.b[1588]) || s.b[1589]) || s.b[1590]) || s.b[1591]) || s.b[1592]) || s.b[1593]))) {
            s.store_add_scaled_inputs(459, 971, s.v[165], 970, s.v[163]);
            s.store_scaled_add(460, 969, 968, s.v[164]);
        }

        if (s.b[1595] && (!(((((((s.b[1587] || s.b[1588]) || s.b[1589]) || s.b[1590]) || s.b[1591]) || s.b[1592]) || s.b[1593]) || s.b[1594]))) {
            s.store_add_scaled_inputs(461, 971, s.v[159], 970, s.v[157]);
            s.store_add_scaled_inputs(462, 969, s.v[160], 968, s.v[158]);
            s.store_add_scaled_inputs(459, 971, s.v[165], 970, s.v[163]);
            s.store_add_scaled_inputs(460, 969, s.v[166], 968, s.v[164]);
        }

        if (s.b[1596] && (!((((((((s.b[1587] || s.b[1588]) || s.b[1589]) || s.b[1590]) || s.b[1591]) || s.b[1592]) || s.b[1593]) || s.b[1594]) || s.b[1595]))) {
            s.store_scalar(461, (s.v[155] + ((p.p2 - 1.0) * s.v[157])));
            s.store_scalar(462, (p.p2 * s.v[158]));
            s.store_scalar(459, (s.v[161] + ((p.p2 - 1.0) * s.v[163])));
            s.store_scalar(460, (p.p2 * s.v[164]));
        }

        if (s.b[1597] && (!(((((((((s.b[1587] || s.b[1588]) || s.b[1589]) || s.b[1590]) || s.b[1591]) || s.b[1592]) || s.b[1593]) || s.b[1594]) || s.b[1595]) || s.b[1596]))) {
            s.store_scalar(461, (p.p2 * s.v[157]));
            s.store_scalar(462, (s.v[156] + ((p.p2 - 1.0) * s.v[158])));
            s.store_scalar(459, (p.p2 * s.v[163]));
            s.store_scalar(460, (s.v[162] + ((p.p2 - 1.0) * s.v[164])));
        }

        if (!((((((((((s.b[1587] || s.b[1588]) || s.b[1589]) || s.b[1590]) || s.b[1591]) || s.b[1592]) || s.b[1593]) || s.b[1594]) || s.b[1595]) || s.b[1596]) || s.b[1597])) {
            s.store_scalar(461, 0.0);
            s.store_scalar(462, 0.0);
            s.store_scalar(459, 0.0);
            s.store_scalar(460, 0.0);
        }

        s.b[1598] = param_given[17];
        s.v[1598] = if s.b[1598] { 1.0 } else { 0.0 };

        if s.b[1598] {
            s.store_scalar(463, ((p.p17 * p.p50) * p.p49));
        }

        if (!s.b[1598]) {
            s.copy_ad(463, 459);
        }

        s.b[1599] = (s.v[463] < 0.0);
        s.v[1599] = if s.b[1599] { 1.0 } else { 0.0 };

        if s.b[1599] {
            s.store_scalar(463, 0.0);
        }

        s.b[1600] = param_given[18];
        s.v[1600] = if s.b[1600] { 1.0 } else { 0.0 };

        if s.b[1600] {
            s.store_scalar(464, ((p.p18 * p.p50) * p.p49));
        }

        if (!s.b[1600]) {
            s.copy_ad(464, 460);
        }

        s.b[1601] = (s.v[464] < 0.0);
        s.v[1601] = if s.b[1601] { 1.0 } else { 0.0 };

        if s.b[1601] {
            s.store_scalar(464, 0.0);
        }

        s.b[1602] = param_given[19];
        s.v[1602] = if s.b[1602] { 1.0 } else { 0.0 };

        s.b[1603] = (p.p926 == 0.0);
        s.v[1603] = if s.b[1603] { 1.0 } else { 0.0 };

        if (s.b[1602] && s.b[1603]) {
            s.store_scalar(494, (p.p19 * p.p50));
        }

        if (s.b[1602] && (!s.b[1603])) {
            s.store_scalar(494, (((p.p19 * p.p50) - (s.v[189] * p.p2))).max(0.0));
        }

        if (!s.b[1602]) {
            s.copy_ad(494, 461);
        }

        s.b[1604] = (s.v[494] < 0.0);
        s.v[1604] = if s.b[1604] { 1.0 } else { 0.0 };

        if ((!s.b[1602]) && s.b[1604]) {
            s.store_scalar(494, 0.0);
        }

        s.b[1605] = param_given[20];
        s.v[1605] = if s.b[1605] { 1.0 } else { 0.0 };

        s.b[1606] = (p.p926 == 0.0);
        s.v[1606] = if s.b[1606] { 1.0 } else { 0.0 };

        if (s.b[1605] && s.b[1606]) {
            s.store_scalar(495, (p.p20 * p.p50));
        }

        if (s.b[1605] && (!s.b[1606])) {
            s.store_scalar(495, (((p.p20 * p.p50) - (s.v[189] * p.p2))).max(0.0));
        }

        if (!s.b[1605]) {
            s.copy_ad(495, 462);
        }

        s.b[1607] = (s.v[495] < 0.0);
        s.v[1607] = if s.b[1607] { 1.0 } else { 0.0 };

        if ((!s.b[1605]) && s.b[1607]) {
            s.store_scalar(495, 0.0);
        }

        s.b[1608] = (((p.p10 > 0.0) && (p.p11 > 0.0)) && ((p.p2 == 1.0) || ((p.p2 > 1.0) && (p.p12 > 0.0))));
        s.v[1608] = if s.b[1608] { 1.0 } else { 0.0 };

        if s.b[1608] {
            s.store_scalar(167, ((s.v[261]) as f64).powf(p.p1111));
            s.store_scalar(910, (s.v[263] + p.p1104));
            s.store_powf(168, 910, p.p1112);
            s.store_add_scaled_inputs3(911, A::div_from_scalar(p.p1108, s.ad_value(167)), 1.0, A::div_from_scalar(p.p1109, s.ad_value(168)), 1.0, A::div_from_scalar(p.p1110, A::mul(s.ad_value(167), s.ad_value(168))), 1.0);
            s.store_offset(912, 911, 1.0);
            s.store_scalar(167, ((s.v[261]) as f64).powf(p.p1117));
            s.store_powf(168, 910, p.p1118);
            s.store_add_scaled_inputs3(913, A::div_from_scalar(p.p1114, s.ad_value(167)), 1.0, A::div_from_scalar(p.p1115, s.ad_value(168)), 1.0, A::div_from_scalar(p.p1116, A::mul(s.ad_value(167), s.ad_value(168))), 1.0);
            s.store_offset(914, 913, 1.0);
            s.store_offset(167, 639, (-1.0));
            s.store_offset_mul_ad(915, s.ad_value(912), A::scale_offset(s.ad_value(167), p.p1107, 1.0), 1e-9);
            s.store_scalar(929, 0.0);
        }

        let mut assign15510_loop_guard: usize = 0;
        while {
            let assign15510_cond_e21369: f64 = if (s.b[1608] && (s.v[929] < p.p2)) { 1.0 } else { 0.0 };
            assign15510_cond_e21369 != 0.0
        } {
            assign15510_loop_guard += 1;
            assert!(assign15510_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[1608] {
                s.store_div_from_scalar_offset_scaled_input(167, (1.0 / p.p2), 929, (p.p12 + s.v[262]), (p.p10 + (0.5 * s.v[262])));
                s.store_div_from_scalar_offset_scaled_input(168, (1.0 / p.p2), 929, (p.p12 + s.v[262]), (p.p11 + (0.5 * s.v[262])));
                s.store_offset(916, 167, s.v[916]);
                s.store_offset(917, 168, s.v[917]);
                s.store_offset(929, 929, 1.0);
            }
        }

        if s.b[1608] {
            s.store_scalar(918, (1.0 / (p.p1102 + (0.5 * s.v[262]))));
            s.store_scalar(919, (1.0 / (p.p1103 + (0.5 * s.v[262]))));
            s.store_add(920, 918, 919);
            s.store_mul_div_from_scalar_lhs(921, p.p1105, 915, 920);
            s.store_add(922, 916, 917);
            s.store_mul_div_from_scalar_lhs(923, p.p1105, 915, 922);
            s.store_div_scaled_offset_numerator(924, s.ad_value(923), 1.0, 1.0, A::offset(s.ad_value(921), 1.0), 1.0);
            s.store_div_scaled_offset_numerator(925, s.ad_value(923), p.p1106, 1.0, A::scale_offset(s.ad_value(921), p.p1106, 1.0), 1.0);
            s.store_mul_ad(926, A::div_from_scalar(p.p1113, s.ad_value(914)), A::sub(s.ad_value(922), s.ad_value(920)));
            s.store_mul_ad(927, A::div_from_scalar(p.p1119, A::powf(s.ad_value(914), p.p1120)), A::sub(s.ad_value(922), s.ad_value(920)));
            s.store_mul_ad(928, A::div_from_scalar(p.p1121, A::powf(s.ad_value(914), p.p1122)), A::sub(s.ad_value(922), s.ad_value(920)));
            s.store_mul(641, 641, 924);
            s.store_mul(653, 653, 925);
            s.store_add(734, 734, 927);
            s.store_add(663, 663, 928);
        }

        s.b[1609] = (p.p27 == 1.0);
        s.v[1609] = if s.b[1609] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1609]) {
            s.store_mul_ad(956, A::div(s.ad_value(850), s.ad_value(914)), A::sub(s.ad_value(922), s.ad_value(920)));
            s.store_mul_ad(957, A::div(s.ad_value(851), A::powf(s.ad_value(914), p.p1120)), A::sub(s.ad_value(922), s.ad_value(920)));
            s.store_mul_ad(958, A::div(s.ad_value(852), A::powf(s.ad_value(914), p.p1122)), A::sub(s.ad_value(922), s.ad_value(920)));
        }

        if s.b[1608] {
            s.store_add(849, 849, 957);
            s.store_add(841, 841, 958);
        }

        if (!s.b[1608]) {
            s.store_scalar(926, 0.0);
            s.store_scalar(956, 0.0);
        }

        s.b[1610] = (p.p34 == 1.0);
        s.v[1610] = if s.b[1610] { 1.0 } else { 0.0 };

        if s.b[1610] {
            s.store_scalar(935, (p.p1 / p.p2));
            s.store_scalar(936, p.p13);
            s.store_scalar(937, p.p14);
            s.store_scalar(938, p.p15);
        }

        s.b[1611] = (((!param_given[13]) && (!param_given[14])) && (!param_given[15]));
        s.v[1611] = if s.b[1611] { 1.0 } else { 0.0 };

        s.b[1612] = (param_given[16] && (p.p16 > 0.0));
        s.v[1612] = if s.b[1612] { 1.0 } else { 0.0 };

        if ((s.b[1610] && s.b[1611]) && s.b[1612]) {
            s.store_offset(168, 935, p.p16);
            s.store_scalar(169, (1.0 / p.p1137));
            s.store_div_from_scalar_scaled_input(936, (p.p1137 * p.p1137), 168, p.p16);
            s.store_div_scaled_add_product(937, A::limited_exp_scaled_input(s.ad_value(169), ((-10.0) * p.p16)), ((0.1 * p.p16) + (0.01 * p.p1137)), A::scale_offset(s.ad_value(168), 0.1, (0.01 * p.p1137)), A::limited_exp(A::mul_scaled_lhs(s.ad_value(168), (-10.0), s.ad_value(169))), (-1.0), s.ad_value(935), 1.0);
            s.store_div_scaled_add_product(938, A::limited_exp_scaled_input(s.ad_value(169), ((-20.0) * p.p16)), ((0.05 * p.p16) + (0.0025 * p.p1137)), A::scale_offset(s.ad_value(168), 0.05, (0.0025 * p.p1137)), A::limited_exp(A::mul_scaled_lhs(s.ad_value(168), (-20.0), s.ad_value(169))), (-1.0), s.ad_value(935), 1.0);
        }

        s.store_mul_add_scaled_inputs3_offset_rhs(930, 806, s.ad_value(936), 1.0, s.ad_value(937), p.p1123, s.ad_value(938), p.p1124, 0.0);

        s.store_mul_add_scaled_inputs3_offset_rhs(931, 807, s.ad_value(936), 1.0, s.ad_value(937), p.p1123, s.ad_value(938), p.p1124, 0.0);

        s.store_mul_add_scaled_inputs3_offset_rhs(932, 855, s.ad_value(936), 1.0, s.ad_value(937), p.p1123, s.ad_value(938), p.p1124, 0.0);

        s.store_mul_add_scaled_inputs3_offset_rhs(933, 854, s.ad_value(936), 1.0, s.ad_value(937), p.p1123, s.ad_value(938), p.p1124, 0.0);

        s.store_offset_mul_ad(934, s.ad_value(808), A::add_scaled_inputs3(s.ad_value(936), 1.0, s.ad_value(937), p.p1123, s.ad_value(938), p.p1124), 1.0);

        s.store_mul(641, 641, 934);

        s.store_add(734, 734, 931);

        s.store_add(849, 849, 933);

        s.store_mul_voltage_ad(221, s.ad_value(379), ctx, nodes, Some(8), Some(10));

        s.store_mul_voltage_ad(230, s.ad_value(379), ctx, nodes, Some(8), Some(11));

        s.store_mul_voltage_ad(223, s.ad_value(379), ctx, nodes, Some(6), Some(10));

        s.store_mul_voltage_ad(224, s.ad_value(379), ctx, nodes, Some(7), Some(10));

        s.store_mul_voltage_ad(232, s.ad_value(379), ctx, nodes, Some(7), Some(11));

        s.store_sub(226, 223, 224);

        s.copy_ad(205, 226);

        s.copy_ad(202, 224);

        s.copy_ad(201, 223);

        s.store_mul_voltage_ad(498, s.ad_value(379), ctx, nodes, Some(10), Some(7));

        s.store_mul_voltage_ad(499, s.ad_value(379), ctx, nodes, Some(10), Some(6));

        s.store_sub(204, 221, 223);

        s.store_sub(203, 221, 224);

        s.store_mul_voltage_ad(430, s.ad_value(379), ctx, nodes, Some(9), Some(6));

        s.store_mul_voltage_ad(431, s.ad_value(379), ctx, nodes, Some(9), Some(7));

        s.store_mul_voltage_ad(234, s.ad_value(379), ctx, nodes, Some(3), Some(10));

        s.store_mul_voltage_ad(239, s.ad_value(379), ctx, nodes, Some(3), Some(11));

        s.store_sub(235, 234, 224);

        s.store_sub(236, 234, 224);

        s.store_sub(237, 234, 223);

        s.store_mul_scaled_voltage(1033, 379, -1.0, ctx, nodes, Some(7), Some(10));

        s.v[211] = 1.0;

        s.b[1613] = (s.v[226] < 0.0);
        s.v[1613] = if s.b[1613] { 1.0 } else { 0.0 };

        if s.b[1613] {
            s.store_scalar(211, (-1.0));
            s.store_mul_voltage_ad(223, s.ad_value(379), ctx, nodes, Some(7), Some(10));
            s.store_mul_voltage_ad(224, s.ad_value(379), ctx, nodes, Some(6), Some(10));
            s.store_mul_voltage_ad(232, s.ad_value(379), ctx, nodes, Some(6), Some(11));
            s.store_sub(235, 234, 224);
            s.store_mul_scaled_voltage(1033, 379, -1.0, ctx, nodes, Some(6), Some(10));
        }

        s.store_sub(226, 223, 224);

        s.store_scale(167, 226, p.p1146);

        s.b[1614] = (s.v[167] > 80.0);
        s.v[1614] = if s.b[1614] { 1.0 } else { 0.0 };

        if s.b[1614] {
            s.copy_ad(168, 167);
        }

        if (!s.b[1614]) {
            s.store_ln_ad(168, A::offset(A::limited_exp(s.ad_value(167)), 1.0));
        }

        s.store_offset_sub_scaled_inputs(227, s.ad_value(168), (2.0 / p.p1146), s.ad_value(226), 1.0, (-((2.0 / p.p1146) * ((2.0) as f64).ln())));

        s.store_neg_ad(218, A::add_scaled_inputs3(s.ad_value(224), 1.0, s.ad_value(226), 0.5, s.ad_value(227), (-0.5)));

        s.store_neg_ad(241, A::add_scaled_inputs3(s.ad_value(232), 1.0, s.ad_value(226), 0.5, s.ad_value(227), (-0.5)));

        s.store_add_scaled_inputs3_indices(220, 234, 1.0, 227, 0.5, 226, (-0.5));

        s.store_tanh_ad(167, A::div_scaled_inputs(s.ad_value(205), 0.6, s.ad_value(637), 1.0));

        s.store_offset_scaled(265, 167, 0.5, 0.5);

        s.store_sub_from_scalar(266, 1.0, 265);

        s.b[1615] = (p.p35 != 0.0);
        s.v[1615] = if s.b[1615] { 1.0 } else { 0.0 };

        if s.b[1615] {
            s.store_add_scaled_products_indices(838, 839, 266, 1.0, 837, 265, 1.0);
            s.store_add_scaled_products_indices(718, 717, 266, 1.0, 710, 265, 1.0);
            s.store_add_scaled_products_indices(732, 664, 266, 1.0, 663, 265, 1.0);
            s.store_add_scaled_products_indices(763, 762, 266, 1.0, 761, 265, 1.0);
            s.store_add_scaled_products_indices(785, 784, 266, 1.0, 783, 265, 1.0);
            s.store_add_scaled_products_indices(348, 347, 266, 1.0, 346, 265, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_9(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1615] {
            s.store_add_scaled_products_indices(746, 654, 266, 1.0, 653, 265, 1.0);
            s.store_add_scaled_products_indices(780, 658, 266, 1.0, 657, 265, 1.0);
            s.store_add_scaled_products_indices(740, 642, 266, 1.0, 641, 265, 1.0);
            s.store_add_scaled_products_indices(750, 644, 266, 1.0, 643, 265, 1.0);
            s.store_add_scaled_products_indices(760, 646, 266, 1.0, 645, 265, 1.0);
            s.store_add_scaled_products_indices(754, 648, 266, 1.0, 647, 265, 1.0);
            s.store_add_scaled_products_indices(757, 650, 266, 1.0, 649, 265, 1.0);
        }

        if (!s.b[1615]) {
            s.copy_ad(838, 837);
            s.copy_ad(718, 710);
            s.copy_ad(732, 663);
            s.copy_ad(763, 761);
            s.copy_ad(785, 783);
            s.copy_ad(348, 346);
            s.copy_ad(746, 653);
            s.copy_ad(780, 657);
            s.copy_ad(740, 641);
            s.copy_ad(750, 643);
            s.copy_ad(760, 645);
            s.copy_ad(754, 647);
            s.copy_ad(757, 649);
        }

        s.v[301] = ((((s.v[200] * p.p74) * p.p76)) as f64).sqrt();

        s.v[303] = (((p.p74 * ((s.v[200] * p.p76) + (0.375 * p.p74)))) as f64).sqrt();

        s.v[304] = (p.p74 + (s.v[200] * (p.p76 + p.p75)));

        s.store_add_scaled_inputs4_indices(167, 221, ((p.p75 * s.v[200]) * 1.0 / (s.v[304])), 707, ((-(p.p75 * s.v[200])) * 1.0 / (s.v[304])), 234, (((p.p76 * s.v[200]) + p.p74) * 1.0 / (s.v[304])), 708, ((-((p.p76 * s.v[200]) + p.p74)) * 1.0 / (s.v[304])));

        s.store_offset_scaled_ad(168, A::atan(A::add_scaled_product(s.ad_value(721), 1.0, s.ad_value(722), s.ad_value(167), 1.0)), 0.3183098861837907, 0.5);

        s.store_offset_scaled(302, 168, (s.v[301] - s.v[303]), s.v[303]);

        s.store_offset_div_scaled_inputs_indices(305, 723, s.v[184], 302, 1.0, 1e-6);

        s.b[1616] = (s.v[305] < 40.0);
        s.v[1616] = if s.b[1616] { 1.0 } else { 0.0 };

        if s.b[1616] {
            s.store_div_from_scalar_offset_ad(955, 0.5, A::cosh(s.ad_value(305)), (-1.0));
        }

        if (!s.b[1616]) {
            s.store_limited_exp_neg_input(955, 305);
        }

        s.v[5] = (s.v[180] / p.p74);

        s.v[7] = (s.v[181] / p.p75);

        s.store_add_scaled_inputs3_offset_mixed_iia(273, 298, 0.5, 218, ((-1.0) * 0.5), A::sqrt_offset_square_offset(A::sub(s.ad_value(298), s.ad_value(218)), (-0.05), ((0.25 * 0.1) * 0.1)), 0.5, (0.05 * 0.5));

        s.store_sqrt(274, 273);

        s.store_mul(275, 277, 274);

        s.store_div_from_scalar(260, s.v[180], 275);

        s.store_add_scaled_inputs_products_indices(276, 709, 1.0, 665, 1.0, 718, 227, 1.0, 719, 218, (-1.0));

        s.store_offset_scaled(168, 276, 1.0 / (s.v[199]), 1.0);

        s.b[1617] = (p.p29 == 1.0);
        s.v[1617] = if s.b[1617] { 1.0 } else { 0.0 };

        if s.b[1617] {
            s.store_scalar(169, ((s.v[5] * s.v[7]) / (s.v[5] + s.v[7])));
            s.store_mul_add_scaled_inputs3_offset_rhs(170, 227, s.ad_value(718), 1.0, s.ad_value(220), p.p266, s.ad_value(218), (-p.p267), 0.0);
            s.store_add_scaled_inputs_products_mixed_aiiiia(171, A::add_scaled_product(s.ad_value(220), p.p268, s.ad_value(220), s.ad_value(220), p.p269), 1.0, 218, (-p.p280), 218, 218, (-p.p281), 955, A::add(A::add_scaled_product(A::add_scaled_value_products3(s.ad_value(714), 1.0, s.ad_value(711), s.ad_value(220), 1.0, s.ad_value(220), s.ad_value(220), p.p274, s.ad_value(713), s.ad_value(218), 1.0), 1.0, s.ad_value(218), s.ad_value(218), p.p279), s.ad_value(170)), 1.0);
            s.store_div_ad(168, A::add_scaled_inputs4_offset(s.ad_value(169), 1.0, s.ad_value(709), 1.0, s.ad_value(665), 1.0, s.ad_value(171), 1.0, s.v[199]), A::offset(s.ad_value(169), s.v[199]));
        }

        s.store_scaled_add_offset_sqrt_square_offset(267, 168, 1.0, (-1.0), ((0.25 * 0.05) * 0.05), 0.5);

        s.store_mul(269, 267, 271);

        s.store_div_from_scalar(270, 1.0, 269);

        s.store_mul_neg_ad_lhs(292, A::add_scaled_product(s.ad_value(732), 1.0, s.ad_value(733), s.ad_value(218), 1.0), 227);

        s.store_offset_sub_scaled_inputs(292, s.ad_value(292), 0.5, A::sqrt_square_offset(s.ad_value(292), ((0.25 * 5e-5) * 5e-5)), 0.5, (0.25 * 5e-5));

        s.store_mul_offset_rhs_ad(293, A::add_scaled_product(A::offset(s.ad_value(679), (p.p1077 / s.v[184])), 1.0, s.ad_value(680), s.ad_value(218), 1.0), A::powf(s.ad_value(639), p.p1076), (-1.0));

        s.b[1618] = (s.v[279] > 0.0);
        s.v[1618] = if s.b[1618] { 1.0 } else { 0.0 };

        if s.b[1618] {
            s.store_mul_neg_lhs(167, 281, 227);
        }

        s.b[1619] = (s.v[167] < (-80.0));
        s.v[1619] = if s.b[1619] { 1.0 } else { 0.0 };

        if (s.b[1618] && s.b[1619]) {
            s.store_scalar(169, 1.804851387e-35);
        }

        if (s.b[1618] && (!s.b[1619])) {
            s.store_limited_exp(169, 167);
        }

        if s.b[1618] {
            s.store_offset_mul_offset_rhs(170, 279, 169, 1.0, s.v[184]);
            s.store_mul_scaled_ln_ad_rhs(278, 269, -1.0, A::max_with_scalar(A::div_from_scalar(s.v[184], s.ad_value(170)), 1e-38));
        }

        if (!s.b[1618]) {
            s.store_scalar(278, 0.0);
        }

        s.store_add_ad_rhs(171, 289, A::div(s.ad_value(283), A::pow_from_scalar(s.v[184], s.ad_value(285))));

        s.store_add_scaled_product_right_ad(278, 278, 1.0, 171, A::tanh(A::mul(s.ad_value(287), s.ad_value(227))), (-1.0));

        s.store_offset(707, 707, p.p25);

        s.store_mul(222, 221, 270);

        s.store_mul(225, 224, 270);

        s.store_mul(212, 707, 270);

        s.store_mul(215, 708, 270);

        s.store_mul(238, 234, 270);

        s.store_add_scaled_products_left_right_ad(291, 735, A::sub(s.ad_value(274), s.ad_value(299)), 1.0, 734, 218, (-1.0));

        s.store_add_ad_lhs(242, A::add(A::add_scaled_inputs4(s.ad_value(292), 1.0, s.ad_value(278), 1.0, s.ad_value(291), 1.0, s.ad_value(293), -1.0), s.ad_value(926)), 930);

        s.store_add_scaled_inputs_product_indices(213, 222, 1.0, 212, (-1.0), 242, 270, (-1.0));

        s.store_add_scaled_inputs_product_first_ad(367, A::add_scaled_product(s.ad_value(222), 1.0, s.ad_value(218), s.ad_value(270), (-1.0)), 1.0, 212, (-1.0), 242, 270, (-1.0));

        s.store_add_scaled_inputs_product_indices(214, 222, 1.0, 212, (-1.0), 242, 270, (-1.0));

        s.store_sub(216, 238, 215);

        s.store_scaled_sqrt_ad(294, A::mul_scaled_lhs(s.ad_value(706), ((2.0 * 1.602176462e-19) * s.v[180]), s.ad_value(272)), 1.0 / (s.v[199]));

        s.v[978] = 0.5;

        s.store_scaled_add_sqrt_square_offset_ad(167, A::add_scaled_product(s.ad_value(251), 2.0, s.ad_value(224), s.ad_value(272), 1.0), ((4.0 * 0.001) * 0.001), 0.5);

        s.store_offset_div_scaled_inputs_mixed_ia(253, 294, 1.0, A::sqrt(s.ad_value(167)), 2.0, 1.0);

        let assign17180_ad_e22738: A = A::add(A::offset(A::add_scaled_product(s.ad_value(251), 2.0, s.ad_value(224), s.ad_value(272), 1.0), (((((s.v[978]).max(1e-38)) as f64).ln()) + ((2.0 * s.v[978])))), A::ln(A::max_with_scalar(A::mul(A::div_scaled_inputs(s.ad_value(253), 2.0, s.ad_value(294), 1.0), A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(253), (2.0 * s.v[978]), s.ad_value(294), 1.0), 1.0, A::sqrt(s.ad_value(167)), 2.0)), 1e-38)));
        let assign17180_ad_e22776: A = A::add(A::offset(A::add_scaled_product(s.ad_value(251), 2.0, s.ad_value(224), s.ad_value(272), 1.0), (((((s.v[978]).max(1e-38)) as f64).ln()) + ((2.0 * s.v[978])))), A::ln(A::max_with_scalar(A::mul(A::div_scaled_inputs(s.ad_value(253), 2.0, s.ad_value(294), 1.0), A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(253), (2.0 * s.v[978]), s.ad_value(294), 1.0), 1.0, A::sqrt(s.ad_value(167)), 2.0)), 1e-38)));
        let assign17180_ad_e22814: A = A::add(A::offset(A::add_scaled_product(s.ad_value(251), 2.0, s.ad_value(224), s.ad_value(272), 1.0), (((((s.v[978]).max(1e-38)) as f64).ln()) + ((2.0 * s.v[978])))), A::ln(A::max_with_scalar(A::mul(A::div_scaled_inputs(s.ad_value(253), 2.0, s.ad_value(294), 1.0), A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(253), (2.0 * s.v[978]), s.ad_value(294), 1.0), 1.0, A::sqrt(s.ad_value(167)), 2.0)), 1e-38)));
        s.store_scaled_add_ad(979, assign17180_ad_e22738, A::sqrt_product_offset(assign17180_ad_e22776, assign17180_ad_e22814, ((4.0 * 0.001) * 0.001)), 0.5);

        s.store_mul_add_scaled_inputs3_offset_rhs(1128, 379, A::add_scaled_product(s.ad_value(707), 1.0, A::add_scaled_product(s.ad_value(979), 1.0, s.ad_value(224), s.ad_value(272), (-1.0)), s.ad_value(271), 1.0), 1.0, A::mul3(s.ad_value(271), s.ad_value(294), A::sqrt(s.ad_value(979))), 1.0, s.ad_value(242), 1.0, 0.0);

        s.store_div_from_scalar(295, 1.0, 294);

        s.b[1620] = (p.p29 == 1.0);
        s.v[1620] = if s.b[1620] { 1.0 } else { 0.0 };

        if s.b[1620] {
            s.store_scaled_sqrt_ad(294, A::mul_scaled_lhs(s.ad_value(706), ((2.0 * 1.602176462e-19) * s.v[180]), s.ad_value(270)), 1.0 / (s.v[199]));
            s.store_div_from_scalar(295, 1.0, 294);
            s.store_square(296, 294);
            s.store_div_from_scalar(297, 1.0, 296);
            s.store_scalar(5, (s.v[180] / p.p74));
            s.store_scalar(7, (s.v[181] / p.p75));
            s.store_div_scaled_inputs2_indices(3, 7, 1.0, 728, 1.0, 5, 1.0);
            s.store_scalar(2, (p.p76 / p.p75));
            s.store_div(124, 294, 2);
            s.store_offset_scaled(125, 124, 0.7071067811865475, 1.0);
        }

        let (assign17320_e22929,) = {
    if s.b[1620] {
        let assign17320_e22927: f64 = (1e-7 * s.v[125]);
        (assign17320_e22927,)
    } else {
        (s.v[126],)
    }
};
        s.v[126] = assign17320_e22929;

        if s.b[1620] {
            s.store_scalar(127, (5.0 / 4.0));
            s.store_div_from_scalar(128, 1.0, 124);
            s.store_square(129, 124);
            s.store_div_from_scalar_ad(130, 1.0, A::add_scaled_inputs(s.ad_value(127), 1.0, s.ad_value(124), 0.7324648775608221));
        }

        s.b[1621] = (((s.v[216]) as f64).abs() <= s.v[126]);
        s.v[1621] = if s.b[1621] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1621]) {
            s.store_mul_ad_affine_product_rhs(131, 216, s.ad_value(128), A::offset(A::mul(s.ad_value(124), A::div_scaled_inputs(s.ad_value(216), -1.0, A::mul_scaled_lhs(s.ad_value(125), (6.0 * ((2.0) as f64).sqrt()), s.ad_value(125)), 1.0)), 1.0), -1.0, 0.0);
        }

        s.b[1622] = (s.v[216] < (-s.v[126]));
        s.v[1622] = if s.b[1622] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1621])) && s.b[1622]) {
            s.store_neg(132, 216);
            s.store_mul3_lhs(133, 127, 132, 128);
            s.store_scaled_sub_offset_sqrt_square_offset(134, 133, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_products_mixed_aaia(135, A::sub(s.ad_value(132), s.ad_value(134)), A::sub(s.ad_value(132), s.ad_value(134)), 1.0, 129, A::offset(s.ad_value(134), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(137, 132, 2.0, 134, (-2.0), 129, -1.0);
            s.store_sub_ad_lhs(138, A::ln(A::max_with_scalar(A::div(s.ad_value(135), s.ad_value(129)), 1e-38)), 134);
            s.store_add(0, 135, 137);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 138, A::sub_scaled_inputs(A::square(s.ad_value(137)), 0.5, s.ad_value(135), 1.0), 1.0);
            s.store_add_ad_rhs(140, 134, A::div_scaled_product3(s.ad_value(135), s.ad_value(0), s.ad_value(138), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(138), s.ad_value(138)), s.ad_value(137), A::sub_scaled_inputs(A::square(s.ad_value(137)), 0.3333333333333333, s.ad_value(135), 1.0))), 1.0));
            s.store_limited_exp(141, 140);
            s.store_sub(142, 132, 140);
            s.store_add_scaled_offset_product_rhs(143, 142, 2.0, 129, 141, (-1.0), 1.0);
            s.store_add_scaled_square_product_mixed_iia(136, 142, 1.0, 129, A::sub(A::offset(s.ad_value(140), 1.0), s.ad_value(141)), 1.0);
            s.store_sub_from_scalar_scaled_mul(144, 1.0, 129, 141, 0.5);
            s.store_add_scaled_square_product_indices(142, 143, 1.0, 144, 136, (-4.0));
            s.store_scaled_div_ad_rhs(145, 136, A::add(s.ad_value(143), A::sqrt(s.ad_value(142))), 2.0);
            s.store_neg_ad(131, A::add(s.ad_value(140), s.ad_value(145)));
        }

        if ((s.b[1620] && (!s.b[1621])) && (!s.b[1622])) {
            s.store_mul_offset_ad_lhs(146, A::mul3(s.ad_value(125), s.ad_value(127), s.ad_value(130)), (-1.0), 130);
            s.store_mul_ad_product_rhs(147, 216, s.ad_value(128), A::offset(A::mul(s.ad_value(146), s.ad_value(216)), 1.0));
            s.store_limited_exp_neg_input(150, 147);
            s.store_sub_from_scalar(149, 1.0, 150);
            s.store_add_scaled_inputs_product_right_ad(148, 216, 1.0, 129, 0.5, 124, A::sqrt(A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(129), 0.25, s.ad_value(149), -1.0)), (-1.0));
            s.store_limited_exp_neg_input(151, 148);
            s.store_add_scaled_inputs3_mixed_iia(152, 216, 2.0, 148, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(129), 1.0, s.ad_value(151)), 1.0);
            s.store_add_scaled_products_mixed_aaia(153, A::sub(s.ad_value(216), s.ad_value(148)), A::sub(s.ad_value(216), s.ad_value(148)), 1.0, 129, A::add(A::offset(s.ad_value(148), (-1.0)), s.ad_value(151)), (-1.0));
            s.store_sub_from_scalar_scaled_mul(154, 1.0, 129, 151, 0.5);
            s.store_add_scaled_square_product_indices(150, 152, 1.0, 154, 153, (-4.0));
            s.store_scaled_div_ad_rhs(139, 153, A::add(s.ad_value(152), A::sqrt(s.ad_value(150))), 2.0);
            s.store_add(131, 148, 139);
        }

        s.b[1623] = (((s.v[216]) as f64).abs() < s.v[126]);
        s.v[1623] = if s.b[1623] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1623]) {
            s.store_mul_ad_affine_product_rhs(46, 216, s.ad_value(128), A::offset(A::mul(s.ad_value(124), A::div_scaled_inputs(s.ad_value(216), -1.0, A::mul_scaled_lhs(s.ad_value(125), (6.0 * ((2.0) as f64).sqrt()), s.ad_value(125)), 1.0)), 1.0), -1.0, 0.0);
            s.store_mul_ad_affine_product_rhs(131, 216, s.ad_value(128), A::offset(A::mul(s.ad_value(124), A::div_scaled_inputs(s.ad_value(216), -1.0, A::mul_scaled_lhs(s.ad_value(125), (6.0 * ((2.0) as f64).sqrt()), s.ad_value(125)), 1.0)), 1.0), -1.0, 0.0);
        }

        if (s.b[1620] && (!s.b[1623])) {
            s.store_add_scaled_inputs3_offset_mixed_aai(19, A::mul3(A::mul3(A::square(s.ad_value(2)), A::sub(s.ad_value(216), s.ad_value(131)), A::sub(s.ad_value(216), s.ad_value(131))), A::div_from_scalar(1.0, s.ad_value(294)), A::div_from_scalar(1.0, s.ad_value(294))), 1.0, A::limited_exp_scaled_input(s.ad_value(131), -1.0), -1.0, 131, -1.0, (-(-1.0)));
            s.store_offset_add_ad(20, A::limited_exp_scaled_input(s.ad_value(131), -1.0), A::div_scaled_product(A::square(s.ad_value(2)), A::sub_scaled_inputs(s.ad_value(131), 2.0, s.ad_value(216), 2.0), 1.0, A::square(s.ad_value(294)), 1.0), (-1.0));
            s.store_sub_div_rhs_indices(46, 131, 19, 20);
        }

        if s.b[1620] {
            s.store_mul(46, 46, 269);
            s.store_offset_scaled(95, 294, 0.7071067811865475, 1.0);
            s.store_div_from_scalar(96, 1.0, 95);
            s.store_add_ad_lhs(97, A::div_scaled_inputs(s.ad_value(251), 2.0, s.ad_value(267), 1.0), 225);
            s.store_limited_exp_neg_input(99, 97);
        }

        let (assign17800_e23644,) = {
    if s.b[1620] {
        let assign17800_e23642: f64 = (0.001 * s.v[95]);
        (assign17800_e23642,)
    } else {
        (s.v[101],)
    }
};
        s.v[101] = assign17800_e23644;

        if s.b[1620] {
            s.store_div_scaled_inputs_indices(167, 726, (-s.v[184]), 300, 1.0);
            s.store_mul_ad_product_lhs(168, s.ad_value(725), A::add_scaled_inputs(A::limited_exp_scaled_input(s.ad_value(167), 0.5), 1.0, A::limited_exp(s.ad_value(167)), 2.0), 724);
            s.store_add_scaled_inputs_product_mixed_aaii(4, A::div_scaled_product(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * p.p74) * p.p74), s.ad_value(269), (2.0 * s.v[180])), 1.0, A::div_from_scalar(p.p294, s.ad_value(269)), 1.0, 3, 216, (-1.0));
        }

        let (assign17840_e23716,) = {
    if s.b[1620] {
        let assign17840_e23706: f64 = (-s.v[4]);
        let assign17840_e23707: f64 = { let limited_exp_arg = assign17840_e23706; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign17840_e23709: f64 = (assign17840_e23707 + s.v[4]);
        let assign17840_e23711: f64 = (assign17840_e23709 - 1.0);
        let assign17840_e23712: f64 = (assign17840_e23711).sqrt();
        let assign17840_e23713: f64 = (s.v[294] * assign17840_e23712);
        let assign17840_e23714: f64 = (s.v[4] + assign17840_e23713);
        (assign17840_e23714,)
    } else {
        (s.v[104],)
    }
};
        s.v[104] = assign17840_e23716;

        s.b[1624] = (s.v[4] < s.v[97]);
        s.v[1624] = if s.b[1624] { 1.0 } else { 0.0 };

        s.b[1625] = (s.v[214] < s.v[104]);
        s.v[1625] = if s.b[1625] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_10(
        s: &mut Scratch,
    ) {
        s.b[1626] = (((s.v[214]) as f64).abs() <= s.v[101]);
        s.v[1626] = if s.b[1626] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1624]) && s.b[1625]) && s.b[1626]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(9, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        s.b[1627] = (s.v[214] < (-s.v[101]));
        s.v[1627] = if s.b[1627] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && s.b[1624]) && s.b[1625]) && (!s.b[1626])) && s.b[1627]) {
            s.store_neg(10, 214);
            s.store_scaled_mul(11, 10, 96, 1.25);
            s.store_scaled_sub_offset_sqrt_square_offset(12, 11, 10.0, (-6.0), 64.0, 0.5);
            s.store_sub(13, 10, 12);
            s.store_add_scaled_square_product_mixed_iia(14, 13, 1.0, 296, A::offset(s.ad_value(12), 1.0), 1.0);
            s.store_sub_scaled_inputs(16, 13, 2.0, 296, 1.0);
            s.store_sub_ad_lhs(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.5, s.ad_value(14), 1.0), 1.0);
            s.store_add_ad_rhs(18, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.3333333333333333, s.ad_value(14), 1.0))), 1.0));
            s.store_limited_exp(28, 18);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_div_from_scalar_offset_ad(13, 1.0, A::square(s.ad_value(18)), 2.0);
            s.store_mul_square_lhs(30, 18, 13);
            s.store_mul3_affine_lhs(31, 18, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), s.ad_value(13), 13);
            s.store_sub(13, 10, 18);
            s.store_mul(33, 99, 29);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(33), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(99), 1.0, s.ad_value(31)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(18), (-1.0), s.ad_value(33), 1.0, (-1.0)), 1.0, s.ad_value(99), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30)), 1.0), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(28), 1.0, s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(9, 18, -1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if ((((s.b[1620] && s.b[1624]) && s.b[1625]) && (!s.b[1626])) && (!s.b[1627])) {
            s.store_div_from_scalar_offset_scaled_input(38, 1.0, 294, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(39, A::mul_scaled_lhs(s.ad_value(95), 1.25, s.ad_value(38)), (-1.0), 38);
            s.store_mul_ad_product_rhs(40, 214, s.ad_value(96), A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));
            s.store_limited_exp_neg_input(13, 40);
            s.store_sub_from_scalar(41, 1.0, 13);
            s.store_add_scaled_inputs_product_right_ad(42, 214, 1.0, 296, 0.5, 294, A::sqrt(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(296), 0.25, s.ad_value(41), -1.0)), (-1.0));
            s.store_offset(43, 97, 3.0);
            s.store_sub_ad(12, A::add_scaled_inputs3(s.ad_value(42), 0.5, s.ad_value(43), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(42), s.ad_value(43)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(43), 0.5, A::sqrt_square_offset(s.ad_value(43), 5.0), 0.5));
            s.store_sub(13, 214, 12);
            s.store_limited_exp_neg_input(33, 12);
            s.store_div_from_scalar_offset_ad(34, 1.0, A::square(s.ad_value(12)), 2.0);
            s.store_mul_square_lhs(30, 12, 34);
            s.store_mul3_affine_lhs(31, 12, 34, 4.0, 0.0, 34);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(34), 8.0, s.ad_value(30), 12.0), s.ad_value(34), 34);
            s.store_max_from_scalar_ad(14, 1e-40, A::add_scaled_square_product(s.ad_value(13), 1.0, s.ad_value(296), A::add_scaled_product(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), 1.0, s.ad_value(99), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(15, 1.0, 296, A::add_scaled_product(s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(16, 13, 2.0, 296, A::add_scaled_sub_value_product(1.0, s.ad_value(33), 1.0, s.ad_value(99), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(17, 97, 1.0, 12, (-1.0), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)), 1.0);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::add_scaled_square_product(s.ad_value(16), 0.5, s.ad_value(14), s.ad_value(15), (-1.0)), 1.0);
            s.store_add_ad_rhs(44, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::add_scaled_square_product(s.ad_value(16), 0.3333333333333333, s.ad_value(14), s.ad_value(15), (-1.0)))), 1.0));
            s.store_limited_exp(28, 44);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_limited_exp_sub(28, 44, 97);
            s.store_div_from_scalar_offset_ad(13, 1.0, A::square(s.ad_value(44)), 2.0);
            s.store_mul_square_lhs(30, 44, 13);
            s.store_mul3_affine_lhs(31, 44, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), s.ad_value(13), 13);
            s.store_sub(13, 214, 44);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(29)), 1.0, s.ad_value(28), 1.0, s.ad_value(99), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(29), 1.0, s.ad_value(44), 1.0, s.ad_value(28), 1.0, (-1.0)), 1.0, s.ad_value(99), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(29), 1.0, s.ad_value(28), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(9, 44, 1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if ((s.b[1620] && s.b[1624]) && (!s.b[1625])) {
            s.copy_ad(47, 2);
            s.store_square(48, 47);
            s.store_add_scaled_product_indices(8, 4, 1.0, 46, 270, (-1.0));
            s.store_add_scaled_product_right_ad(105, 214, 1.0, 294, A::sqrt(A::offset(A::add(A::limited_exp_scaled_input(s.ad_value(8), -1.0), s.ad_value(8)), (-1.0))), (-1.0));
            s.store_offset(43, 97, 3.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(106, 105, 0.5, 43, 0.5, A::offset(A::mul(A::sub(s.ad_value(105), s.ad_value(43)), A::sub(s.ad_value(105), s.ad_value(43))), 40.0), (-0.5));
            s.store_add_scaled_value_products(107, A::mul3(s.ad_value(48), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0)), (-1.0), A::sub(s.ad_value(214), s.ad_value(106)), A::sub(s.ad_value(214), s.ad_value(106)), 1.0, s.ad_value(296), s.ad_value(4), (-1.0));
            s.store_add_scaled_inputs_product_right_ad(108, 214, 2.0, 106, (-2.0), 48, A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0), (-2.0));
            s.store_square(109, 108);
            s.store_sub_from_scalar(110, 1.0, 48);
        }

        s.b[1628] = (s.v[107] < 0.0);
        s.v[1628] = if s.b[1628] { 1.0 } else { 0.0 };

        if (((s.b[1620] && s.b[1624]) && (!s.b[1625])) && s.b[1628]) {
            s.store_scalar(107, 0.0);
        }

        if ((s.b[1620] && s.b[1624]) && (!s.b[1625])) {
            s.store_add_scaled_inputs3_mixed_iia(49, 97, 1.0, 106, (-1.0), A::ln(A::max_with_scalar(A::mul(s.ad_value(107), s.ad_value(297)), 1e-38)), 1.0);
            s.store_add(111, 107, 108);
            s.store_square(112, 111);
            s.store_add_scaled_inputs_product_first_ad(113, A::div(s.ad_value(112), s.ad_value(49)), 1.0, 109, 0.5, 107, 110, (-1.0));
            s.store_div_scaled_product_indices(114, 108, 111, 1.0, 113, 1.0);
            s.store_add_scaled_product_indices(115, 109, 0.3333333333333333, 107, 110, (-1.0));
            s.store_div_scaled_product_denominator_ad(116, 111, 107, 1.0, A::add_scaled_product(s.ad_value(113), 1.0, s.ad_value(114), s.ad_value(115), 1.0), 1.0);
            s.store_add(117, 106, 116);
            s.store_limited_exp_sub(118, 117, 97);
            s.store_add_scaled_inputs_products_mixed_iiiaii(119, 214, 2.0, 117, (-2.0), 48, A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0), (-2.0), 296, 118, 1.0);
            s.store_add_scaled_value_products(120, A::mul3(s.ad_value(48), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0)), (-1.0), A::sub(s.ad_value(214), s.ad_value(117)), A::sub(s.ad_value(214), s.ad_value(117)), 1.0, s.ad_value(296), A::add(s.ad_value(4), s.ad_value(118)), (-1.0));
            s.store_mul_add_scaled_sub_value_product_rhs(121, 120, 2.0, A::scale(s.ad_value(48), 2.0), 2.0, s.ad_value(296), s.ad_value(118), (((-1.0)) * (2.0)));
            s.store_div_scaled_inputs_mixed_ia(122, 120, 2.0, A::add(s.ad_value(119), A::sqrt(A::sub(A::square(s.ad_value(119)), s.ad_value(121)))), 1.0);
            s.store_add(9, 117, 122);
        }

        s.b[1629] = (((s.v[214]) as f64).abs() <= s.v[101]);
        s.v[1629] = if s.b[1629] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1624])) && s.b[1629]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(9, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        s.b[1630] = (s.v[214] < (-s.v[101]));
        s.v[1630] = if s.b[1630] { 1.0 } else { 0.0 };

        if (((s.b[1620] && (!s.b[1624])) && (!s.b[1629])) && s.b[1630]) {
            s.store_neg(10, 214);
            s.store_scaled_mul(11, 10, 96, 1.25);
            s.store_scaled_sub_offset_sqrt_square_offset(12, 11, 10.0, (-6.0), 64.0, 0.5);
            s.store_sub(13, 10, 12);
            s.store_add_scaled_square_product_mixed_iia(14, 13, 1.0, 296, A::offset(s.ad_value(12), 1.0), 1.0);
            s.store_sub_scaled_inputs(16, 13, 2.0, 296, 1.0);
            s.store_sub_ad_lhs(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.5, s.ad_value(14), 1.0), 1.0);
            s.store_add_ad_rhs(18, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.3333333333333333, s.ad_value(14), 1.0))), 1.0));
            s.store_limited_exp(28, 18);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_div_from_scalar_offset_ad(13, 1.0, A::square(s.ad_value(18)), 2.0);
            s.store_mul_square_lhs(30, 18, 13);
            s.store_mul3_affine_lhs(31, 18, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), s.ad_value(13), 13);
            s.store_sub(13, 10, 18);
            s.store_mul(33, 99, 29);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(33), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(99), 1.0, s.ad_value(31)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(18), (-1.0), s.ad_value(33), 1.0, (-1.0)), 1.0, s.ad_value(99), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30)), 1.0), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(28), 1.0, s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_offset_sqrt_ad(13, A::offset(A::square(s.ad_value(13)), 6.4e-7), (-0.0008));
            s.store_sub_scaled_inputs_ad_rhs(9, 18, -1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if (((s.b[1620] && (!s.b[1624])) && (!s.b[1629])) && (!s.b[1630])) {
            s.store_div_from_scalar_offset_scaled_input(38, 1.0, 294, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(39, A::mul_scaled_lhs(s.ad_value(95), 1.25, s.ad_value(38)), (-1.0), 38);
            s.store_mul_ad_product_rhs(40, 214, s.ad_value(96), A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));
            s.store_limited_exp_neg_input(13, 40);
            s.store_sub_from_scalar(41, 1.0, 13);
            s.store_add_scaled_inputs_product_right_ad(42, 214, 1.0, 296, 0.5, 294, A::sqrt(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(296), 0.25, s.ad_value(41), -1.0)), (-1.0));
            s.store_offset(43, 97, 3.0);
            s.store_sub_ad(12, A::add_scaled_inputs3(s.ad_value(42), 0.5, s.ad_value(43), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(42), s.ad_value(43)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(43), 0.5, A::sqrt_square_offset(s.ad_value(43), 5.0), 0.5));
            s.store_sub(13, 214, 12);
            s.store_limited_exp_neg_input(33, 12);
            s.store_div_from_scalar_offset_ad(34, 1.0, A::square(s.ad_value(12)), 2.0);
            s.store_mul_square_lhs(30, 12, 34);
            s.store_mul3_affine_lhs(31, 12, 34, 4.0, 0.0, 34);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(34), 8.0, s.ad_value(30), 12.0), s.ad_value(34), 34);
        }

    }

    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1620] && (!s.b[1624])) && (!s.b[1629])) && (!s.b[1630])) {
            s.store_max_from_scalar_ad(14, 1e-40, A::add_scaled_square_product(s.ad_value(13), 1.0, s.ad_value(296), A::add_scaled_product(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), 1.0, s.ad_value(99), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(15, 1.0, 296, A::add_scaled_product(s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(16, 13, 2.0, 296, A::add_scaled_sub_value_product(1.0, s.ad_value(33), 1.0, s.ad_value(99), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(17, 97, 1.0, 12, (-1.0), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)), 1.0);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::add_scaled_square_product(s.ad_value(16), 0.5, s.ad_value(14), s.ad_value(15), (-1.0)), 1.0);
            s.store_add_ad_rhs(44, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::add_scaled_square_product(s.ad_value(16), 0.3333333333333333, s.ad_value(14), s.ad_value(15), (-1.0)))), 1.0));
            s.store_limited_exp(28, 44);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_limited_exp_sub(28, 44, 97);
            s.store_div_from_scalar_offset_ad(13, 1.0, A::square(s.ad_value(44)), 2.0);
            s.store_mul_square_lhs(30, 44, 13);
            s.store_mul3_affine_lhs(31, 44, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), s.ad_value(13), 13);
            s.store_sub(13, 214, 44);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(29)), 1.0, s.ad_value(28), 1.0, s.ad_value(99), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(29), 1.0, s.ad_value(44), 1.0, s.ad_value(28), 1.0, (-1.0)), 1.0, s.ad_value(99), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(29), 1.0, s.ad_value(28), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_offset_sqrt_ad(13, A::offset(A::square(s.ad_value(13)), 6.4e-7), (-0.0008));
            s.store_add_scaled_inputs_ad_rhs(9, 44, 1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if s.b[1620] {
            s.copy_ad(123, 9);
        }

        let (assign19380_e26721,) = {
    if s.b[1620] {
        (1e-7,)
    } else {
        (s.v[102],)
    }
};
        s.v[102] = assign19380_e26721;

        if s.b[1620] {
            s.store_scalar(103, 2.0);
            s.store_scaled_square(35, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_div_scaled_inputs_indices(167, 726, (-s.v[184]), 300, 1.0);
            s.store_mul_ad_product_lhs(168, s.ad_value(725), A::add_scaled_inputs(A::limited_exp_scaled_input(s.ad_value(167), 0.5), 1.0, A::limited_exp(s.ad_value(167)), 2.0), 724);
            s.store_offset_ad(24, A::mul_scaled_output(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * (p.p74 * p.p74)) * 1.0 / ((2.0 * s.v[180])))), p.p294);
            s.store_add_scaled_value_products(6, s.ad_value(24), 1.0, A::mul3(s.ad_value(3), s.ad_value(216), s.ad_value(269)), s.ad_value(727), (-1.0), A::offset(s.ad_value(3), 1.0), s.ad_value(46), 1.0);
        }

        s.b[1631] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1631] = if s.b[1631] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1631]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(22, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if (s.b[1620] && (!s.b[1631])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_product3_rhs(45, 167, s.ad_value(214), s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
            s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
        }

        if (s.b[1620] && (!s.b[1631])) {
            let assign19520_ad_e26997: A = {
                if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                } else {
                    let assign19520_ad_e26996: A = {
                        if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                        } else {
                            {
                                if ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign19520_ad_e26996
                }
            };
            let assign19520_ad_e27079: A = {
                if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                } else {
                    let assign19520_ad_e27078: A = {
                        if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                        } else {
                            {
                                if ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign19520_ad_e27078
                }
            };
            s.store_sub_ad(169, assign19520_ad_e26997, assign19520_ad_e27079);
        }

        if (s.b[1620] && (!s.b[1631])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_neg_input(176, 97);
            s.store_limited_exp_sub(177, 123, 97);
            s.store_add_scaled_value_products(19, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123)), 1.0, s.ad_value(296), A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
        }

        if (s.b[1620] && (!s.b[1631])) {
            let assign19610_ad_e27269: A = A::add_scaled_inputs3(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0);
            s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add(assign19610_ad_e27269, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0)), 1.0, (-1.0));
        }

        if (s.b[1620] && (!s.b[1631])) {
            let assign19620_ad_e27336: A = A::sub(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0));
            let assign19620_ad_e27395: A = A::mul(A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0));
            let assign19620_ad_e27424: A = A::sub(A::add_scaled_product(assign19620_ad_e27336, 1.0, s.ad_value(296), A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0, assign19620_ad_e27395, -1.0), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign19620_ad_e27424, 2.0);
        }

        if (s.b[1620] && (!s.b[1631])) {
            s.store_add_scaled_offset_product_rhs_mixed_iaa(22, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));
        }

        if s.b[1620] {
            s.copy_ad(123, 22);
        }

        s.b[1632] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1632] = if s.b[1632] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1632]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(22, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if (s.b[1620] && (!s.b[1632])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_product3_rhs(45, 167, s.ad_value(214), s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
            s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
        }

        if (s.b[1620] && (!s.b[1632])) {
            let assign19720_ad_e27650: A = {
                if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                } else {
                    let assign19720_ad_e27649: A = {
                        if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                        } else {
                            {
                                if ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign19720_ad_e27649
                }
            };
            let assign19720_ad_e27732: A = {
                if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                } else {
                    let assign19720_ad_e27731: A = {
                        if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                        } else {
                            {
                                if ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign19720_ad_e27731
                }
            };
            s.store_sub_ad(169, assign19720_ad_e27650, assign19720_ad_e27732);
        }

        if (s.b[1620] && (!s.b[1632])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_neg_input(176, 97);
            s.store_limited_exp_sub(177, 123, 97);
            s.store_add_scaled_value_products(19, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123)), 1.0, s.ad_value(296), A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
        }

        if (s.b[1620] && (!s.b[1632])) {
            let assign19810_ad_e27922: A = A::add_scaled_inputs3(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0);
            s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add(assign19810_ad_e27922, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0)), 1.0, (-1.0));
        }

        if (s.b[1620] && (!s.b[1632])) {
            let assign19820_ad_e27989: A = A::sub(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0));
            let assign19820_ad_e28048: A = A::mul(A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0));
            let assign19820_ad_e28077: A = A::sub(A::add_scaled_product(assign19820_ad_e27989, 1.0, s.ad_value(296), A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0, assign19820_ad_e28048, -1.0), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign19820_ad_e28077, 2.0);
        }

        if (s.b[1620] && (!s.b[1632])) {
            s.store_add_scaled_offset_product_rhs_mixed_iaa(22, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));
        }

        if s.b[1620] {
            s.copy_ad(123, 22);
        }

        s.b[1633] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1633] = if s.b[1633] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1633]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(22, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if (s.b[1620] && (!s.b[1633])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_product3_rhs(45, 167, s.ad_value(214), s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
            s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
        }

        if (s.b[1620] && (!s.b[1633])) {
            let assign19920_ad_e28303: A = {
                if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                } else {
                    let assign19920_ad_e28302: A = {
                        if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                        } else {
                            {
                                if ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign19920_ad_e28302
                }
            };
            let assign19920_ad_e28385: A = {
                if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                } else {
                    let assign19920_ad_e28384: A = {
                        if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                        } else {
                            {
                                if ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign19920_ad_e28384
                }
            };
            s.store_sub_ad(169, assign19920_ad_e28303, assign19920_ad_e28385);
        }

        if (s.b[1620] && (!s.b[1633])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_neg_input(176, 97);
            s.store_limited_exp_sub(177, 123, 97);
            s.store_add_scaled_value_products(19, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123)), 1.0, s.ad_value(296), A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
        }

        if (s.b[1620] && (!s.b[1633])) {
            let assign20010_ad_e28575: A = A::add_scaled_inputs3(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0);
            s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add(assign20010_ad_e28575, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0)), 1.0, (-1.0));
        }

        if (s.b[1620] && (!s.b[1633])) {
            let assign20020_ad_e28642: A = A::sub(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0));
            let assign20020_ad_e28701: A = A::mul(A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0));
            let assign20020_ad_e28730: A = A::sub(A::add_scaled_product(assign20020_ad_e28642, 1.0, s.ad_value(296), A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0, assign20020_ad_e28701, -1.0), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign20020_ad_e28730, 2.0);
        }

        if (s.b[1620] && (!s.b[1633])) {
            s.store_add_scaled_offset_product_rhs_mixed_iaa(22, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));
        }

        if s.b[1620] {
            s.store_scale(50, 269, 3.912023005);
        }

        s.b[1634] = (s.v[22] <= 0.0);
        s.v[1634] = if s.b[1634] { 1.0 } else { 0.0 };

        if (s.b[1620] && s.b[1634]) {
            s.store_scalar(306, 0.0);
            s.store_sub(51, 214, 22);
            s.store_mul(52, 51, 269);
        }

    }

    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1620] && s.b[1634]) {
            s.copy_ad(312, 50);
            s.store_scalar(458, 1.0);
            s.store_scalar(334, 1.0);
            s.store_scalar(834, 1.0);
            s.store_scalar(853, 1.0);
            s.store_scalar(343, 1.0);
            s.store_scalar(339, 1.0);
            s.store_scalar(363, 1.0);
            s.store_scalar(365, 1.0);
            s.copy_ad(455, 453);
            s.copy_ad(454, 452);
        }

        if (s.b[1620] && (!s.b[1634])) {
            s.store_div_from_scalar_offset_ad(54, 1.0, A::square(s.ad_value(22)), 2.0);
            s.store_mul_square_lhs(55, 22, 54);
            s.store_limited_exp(53, 22);
            s.store_div_from_scalar(56, 1.0, 53);
            s.store_limited_exp_sub(53, 22, 97);
            s.store_add_scaled_product_mixed_iaa(57, 53, 1.0, A::limited_exp_scaled_input(s.ad_value(97), -1.0), A::add(A::offset(s.ad_value(22), 1.0), s.ad_value(55)), (-1.0));
            s.store_sub_ad_lhs(58, A::mul3(A::sub(s.ad_value(214), s.ad_value(22)), A::sub(s.ad_value(214), s.ad_value(22)), A::div_from_scalar(1.0, s.ad_value(296))), 57);
            s.store_offset_add_scaled_inputs(58, A::offset(s.ad_value(58), (-0.001)), 0.5, A::sqrt_offset_square_offset(s.ad_value(58), (-0.001), ((4.0 * 1e-5) * 1e-5)), 0.5, 0.001);
            s.store_sqrt(59, 58);
            s.store_mul_sqrt_ad_rhs(61, 294, A::add(s.ad_value(58), s.ad_value(57)));
            s.store_div_scaled_product3_mixed_iiia(306, 296, 57, 269, 1.0, A::add_scaled_product(s.ad_value(61), 1.0, s.ad_value(294), s.ad_value(59), 1.0), 1.0);
            s.store_mul3_lhs(247, 59, 294, 269);
            s.copy_ad(76, 56);
            s.copy_ad(78, 57);
            s.store_scalar(335, (1e-8 / (s.v[200] * p.p76)));
            s.store_mul_add_scaled_inputs_rhs(308, 335, s.ad_value(247), 1.0, s.ad_value(306), s.v[338]);
            s.store_limited_exp_ad(169, A::mul(s.ad_value(757), A::ln(A::max_with_scalar(A::scaled_offset(A::div(s.ad_value(306), s.ad_value(247)), 1.0, 0.5), 1e-38))));
            s.store_add_scaled_product(170, A::div(s.ad_value(754), s.ad_value(169)), 1.0, A::add_scaled_product(s.ad_value(750), 1.0, s.ad_value(760), s.ad_value(218), 1.0), A::pow(s.ad_value(308), s.ad_value(651)), 1.0);
            s.store_offset(171, 170, 1.0);
            s.store_scaled_add_offset_sqrt_square_offset(309, 171, 1.0, (-1.0), ((0.25 * 0.0015) * 0.0015), 0.5);
            s.store_div_from_scalar_scaled_ad(448, 1.0, A::pow_from_scalar((s.v[183] * 1000000.0), s.ad_value(771)), p.p2);
        }

        s.b[1635] = (p.p33 == 1.0);
        s.v[1635] = if s.b[1635] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1634])) && s.b[1635]) {
            s.store_scalar(456, 0.0);
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1635])) {
            s.store_offset_mul(167, 770, 306, 1.0);
            s.store_mul_sub_rhs(168, 787, 274, 299);
            s.store_add_ad_lhs(169, A::div_from_scalar(1.0, s.ad_value(167)), 168);
            s.store_add_ad_rhs(170, 169, A::sqrt_square_offset(s.ad_value(169), 0.01));
            s.store_mul_ad_affine_product_lhs(456, A::add_scaled_product(s.ad_value(777), 1.0, s.ad_value(776), s.ad_value(170), 1.0), s.ad_value(448), p.p2, 0.0, 652);
        }

        s.b[1636] = (p.p33 == 2.0);
        s.v[1636] = if s.b[1636] { 1.0 } else { 0.0 };

        if (((s.b[1620] && (!s.b[1634])) && (!s.b[1635])) && s.b[1636]) {
            s.store_mul_add_ad_lhs(456, A::add_scaled_product(s.ad_value(452), 1.0, A::add_scaled_product(s.ad_value(777), 1.0, s.ad_value(776), s.ad_value(170), 1.0), s.ad_value(448), p.p2), s.ad_value(453), 652);
        }

        if (s.b[1620] && (!s.b[1634])) {
            s.store_mul_div_scaled_inputs_rhs(310, 309, s.ad_value(746), 2.0, s.ad_value(740), 1.0);
            s.store_scale(311, 310, s.v[184]);
            s.store_mul_add_scaled_inputs_rhs(173, 742, s.ad_value(306), 1.0, s.ad_value(269), 2.0);
        }

        s.b[1637] = (s.v[456] > 0.0);
        s.v[1637] = if s.b[1637] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1634])) && s.b[1637]) {
            s.store_scale(324, 746, (s.v[183] * s.v[199]));
            s.store_mul(167, 324, 456);
            s.store_scale(325, 167, 2.0);
            s.store_add_scaled_inputs_product_indices(326, 173, 1.0, 311, 1.0, 173, 167, 3.0);
            s.store_mul_add_scaled_product_rhs(327, 173, s.ad_value(311), 1.0, s.ad_value(173), s.ad_value(167), 2.0);
            s.store_div_scaled_inputs2_mixed_iai(312, 326, 1.0, A::sqrt(A::add_scaled_square_product(s.ad_value(326), 1.0, s.ad_value(325), s.ad_value(327), (-2.0))), (-1.0), 325, 1.0);
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1637])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(312, 311, 173, 1.0, 311, 1.0, 173, 1.0, 1.0);
        }

        s.b[1638] = ((p.p1349 == 0.0) && (p.p1350 == 0.0));
        s.v[1638] = if s.b[1638] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1634])) && s.b[1638]) {
            s.store_scalar(1019, 1.0);
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1638])) {
            s.store_div_from_scalar_offset_ad(168, s.v[184], A::sqrt(A::mul(s.ad_value(782), s.ad_value(275))), s.v[184]);
            s.store_offset_div_scaled_inputs2_mixed_iaa(1019, 168, p.p1349, A::mul3_scaled_output(s.ad_value(168), A::powf(s.ad_value(306), p.p1351), s.ad_value(269), p.p1350), (-1.0), A::scale_offset(s.ad_value(218), p.p1352, 1.0), 1.0, 1.0);
            s.store_scaled_add_offset_sqrt_square_offset(1019, 1019, 0.1, (-0.1), ((0.25 * 0.0005) * 0.0005), 0.5);
        }

        if (s.b[1620] && (!s.b[1634])) {
            s.store_offset_add_scaled_inputs(312, A::offset(s.ad_value(312), (-0.001)), 0.5, A::sqrt_offset_square_offset(s.ad_value(312), (-0.001), ((4.0 * 1e-5) * 1e-5)), 0.5, 0.001);
            s.store_div(312, 312, 1019);
            s.store_pow_ad(174, A::offset(A::div(s.ad_value(226), s.ad_value(312)), 1e-6), A::div_from_scalar(1.0, s.ad_value(656)));
            s.store_pow_ad(175, A::offset(s.ad_value(174), 1.0), A::neg(s.ad_value(656)));
            s.store_mul(315, 226, 175);
            s.store_mul_add_lhs(318, 315, 224, 270);
            s.store_add_ad_lhs(98, A::div_scaled_inputs(s.ad_value(251), 2.0, s.ad_value(267), 1.0), 318);
            s.store_limited_exp_neg_input(100, 98);
        }

        let (assign20740_e29615,) = {
    if (s.b[1620] && (!s.b[1634])) {
        let assign20740_e29613: f64 = (0.001 * s.v[95]);
        (assign20740_e29613,)
    } else {
        (s.v[101],)
    }
};
        s.v[101] = assign20740_e29615;

        if (s.b[1620] && (!s.b[1634])) {
            s.store_div_scaled_inputs_indices(167, 726, (-s.v[184]), 300, 1.0);
            s.store_mul_ad_product_rhs(168, 725, A::add_scaled_inputs(A::limited_exp_scaled_input(s.ad_value(167), 0.5), 1.0, A::limited_exp(s.ad_value(167)), 2.0), A::add_scaled_products(s.ad_value(226), s.ad_value(270), 1.0, s.ad_value(724), s.ad_value(270), 1.0));
            s.store_add_scaled_offset_product_lhs_mixed_aii(4, A::add_scaled_inputs3(A::div_scaled_product(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * p.p74) * p.p74), s.ad_value(269), (2.0 * s.v[180])), 1.0, A::div_from_scalar(p.p294, s.ad_value(269)), 1.0, A::mul3(s.ad_value(3), s.ad_value(216), s.ad_value(727)), -1.0), 1.0, 3, 1.0, 168, 1.0);
        }

        let (assign20780_e29711,) = {
    if (s.b[1620] && (!s.b[1634])) {
        let assign20780_e29701: f64 = (-s.v[4]);
        let assign20780_e29702: f64 = { let limited_exp_arg = assign20780_e29701; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign20780_e29704: f64 = (assign20780_e29702 + s.v[4]);
        let assign20780_e29706: f64 = (assign20780_e29704 - 1.0);
        let assign20780_e29707: f64 = (assign20780_e29706).sqrt();
        let assign20780_e29708: f64 = (s.v[294] * assign20780_e29707);
        let assign20780_e29709: f64 = (s.v[4] + assign20780_e29708);
        (assign20780_e29709,)
    } else {
        (s.v[104],)
    }
};
        s.v[104] = assign20780_e29711;

        s.b[1639] = (s.v[4] < s.v[98]);
        s.v[1639] = if s.b[1639] { 1.0 } else { 0.0 };

        s.b[1640] = (s.v[214] < s.v[104]);
        s.v[1640] = if s.b[1640] { 1.0 } else { 0.0 };

        s.b[1641] = (((s.v[214]) as f64).abs() <= s.v[101]);
        s.v[1641] = if s.b[1641] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && (!s.b[1634])) && s.b[1639]) && s.b[1640]) && s.b[1641]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(9, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(100)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        s.b[1642] = (s.v[214] < (-s.v[101]));
        s.v[1642] = if s.b[1642] { 1.0 } else { 0.0 };

        if (((((s.b[1620] && (!s.b[1634])) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1642]) {
            s.store_neg(10, 214);
            s.store_scaled_mul(11, 10, 96, 1.25);
            s.store_scaled_sub_offset_sqrt_square_offset(12, 11, 10.0, (-6.0), 64.0, 0.5);
            s.store_sub(13, 10, 12);
            s.store_add_scaled_square_product_mixed_iia(14, 13, 1.0, 296, A::offset(s.ad_value(12), 1.0), 1.0);
            s.store_sub_scaled_inputs(16, 13, 2.0, 296, 1.0);
            s.store_sub_ad_lhs(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.5, s.ad_value(14), 1.0), 1.0);
            s.store_add_ad_rhs(18, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.3333333333333333, s.ad_value(14), 1.0))), 1.0));
            s.store_limited_exp(28, 18);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_div_from_scalar_offset_ad(13, 1.0, A::square(s.ad_value(18)), 2.0);
            s.store_mul_square_lhs(30, 18, 13);
            s.store_mul3_affine_lhs(31, 18, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), s.ad_value(13), 13);
            s.store_sub(13, 10, 18);
            s.store_mul(33, 100, 29);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(33), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(100), 1.0, s.ad_value(31)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(18), (-1.0), s.ad_value(33), 1.0, (-1.0)), 1.0, s.ad_value(100), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30)), 1.0), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(28), 1.0, s.ad_value(33), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(9, 18, -1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if (((((s.b[1620] && (!s.b[1634])) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && (!s.b[1642])) {
            s.store_div_from_scalar_offset_scaled_input(38, 1.0, 294, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(39, A::mul_scaled_lhs(s.ad_value(95), 1.25, s.ad_value(38)), (-1.0), 38);
            s.store_mul_ad_product_rhs(40, 214, s.ad_value(96), A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));
            s.store_limited_exp_neg_input(13, 40);
            s.store_sub_from_scalar(41, 1.0, 13);
            s.store_add_scaled_inputs_product_right_ad(42, 214, 1.0, 296, 0.5, 294, A::sqrt(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(296), 0.25, s.ad_value(41), -1.0)), (-1.0));
            s.store_offset(43, 98, 3.0);
            s.store_sub_ad(12, A::add_scaled_inputs3(s.ad_value(42), 0.5, s.ad_value(43), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(42), s.ad_value(43)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(43), 0.5, A::sqrt_square_offset(s.ad_value(43), 5.0), 0.5));
            s.store_sub(13, 214, 12);
            s.store_limited_exp_neg_input(33, 12);
            s.store_div_from_scalar_offset_ad(34, 1.0, A::square(s.ad_value(12)), 2.0);
            s.store_mul_square_lhs(30, 12, 34);
            s.store_mul3_affine_lhs(31, 12, 34, 4.0, 0.0, 34);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(34), 8.0, s.ad_value(30), 12.0), s.ad_value(34), 34);
            s.store_max_from_scalar_ad(14, 1e-40, A::add_scaled_square_product(s.ad_value(13), 1.0, s.ad_value(296), A::add_scaled_product(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), 1.0, s.ad_value(100), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(15, 1.0, 296, A::add_scaled_product(s.ad_value(33), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(16, 13, 2.0, 296, A::add_scaled_sub_value_product(1.0, s.ad_value(33), 1.0, s.ad_value(100), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(17, 98, 1.0, 12, (-1.0), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)), 1.0);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::add_scaled_square_product(s.ad_value(16), 0.5, s.ad_value(14), s.ad_value(15), (-1.0)), 1.0);
            s.store_add_ad_rhs(44, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::add_scaled_square_product(s.ad_value(16), 0.3333333333333333, s.ad_value(14), s.ad_value(15), (-1.0)))), 1.0));
            s.store_limited_exp(28, 44);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_limited_exp_sub(28, 44, 98);
            s.store_div_from_scalar_offset_ad(13, 1.0, A::square(s.ad_value(44)), 2.0);
            s.store_mul_square_lhs(30, 44, 13);
        }

    }

    pub(super) fn stamp_transient_block_13(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.b[1620] && (!s.b[1634])) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && (!s.b[1642])) {
            s.store_mul3_affine_lhs(31, 44, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), s.ad_value(13), 13);
            s.store_sub(13, 214, 44);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(29)), 1.0, s.ad_value(28), 1.0, s.ad_value(100), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(29), 1.0, s.ad_value(44), 1.0, s.ad_value(28), 1.0, (-1.0)), 1.0, s.ad_value(100), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(29), 1.0, s.ad_value(28), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(9, 44, 1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if (((s.b[1620] && (!s.b[1634])) && s.b[1639]) && (!s.b[1640])) {
            s.copy_ad(47, 2);
            s.store_square(48, 47);
            s.store_add_scaled_product_indices(8, 4, 1.0, 46, 270, (-1.0));
            s.store_add_scaled_product_right_ad(105, 214, 1.0, 294, A::sqrt(A::offset(A::add(A::limited_exp_scaled_input(s.ad_value(8), -1.0), s.ad_value(8)), (-1.0))), (-1.0));
            s.store_offset(43, 98, 3.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(106, 105, 0.5, 43, 0.5, A::offset(A::mul(A::sub(s.ad_value(105), s.ad_value(43)), A::sub(s.ad_value(105), s.ad_value(43))), 40.0), (-0.5));
            s.store_add_scaled_value_products(107, A::mul3(s.ad_value(48), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0)), (-1.0), A::sub(s.ad_value(214), s.ad_value(106)), A::sub(s.ad_value(214), s.ad_value(106)), 1.0, s.ad_value(296), s.ad_value(4), (-1.0));
            s.store_add_scaled_inputs_product_right_ad(108, 214, 2.0, 106, (-2.0), 48, A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0), (-2.0));
            s.store_square(109, 108);
            s.store_sub_from_scalar(110, 1.0, 48);
        }

        s.b[1643] = (s.v[107] < 0.0);
        s.v[1643] = if s.b[1643] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && (!s.b[1634])) && s.b[1639]) && (!s.b[1640])) && s.b[1643]) {
            s.store_scalar(107, 0.0);
        }

        if (((s.b[1620] && (!s.b[1634])) && s.b[1639]) && (!s.b[1640])) {
            s.store_add_scaled_inputs3_mixed_iia(49, 98, 1.0, 106, (-1.0), A::ln(A::max_with_scalar(A::mul(s.ad_value(107), s.ad_value(297)), 1e-38)), 1.0);
            s.store_add(111, 107, 108);
            s.store_square(112, 111);
            s.store_add_scaled_inputs_product_first_ad(113, A::div(s.ad_value(112), s.ad_value(49)), 1.0, 109, 0.5, 107, 110, (-1.0));
            s.store_div_scaled_product_indices(114, 108, 111, 1.0, 113, 1.0);
            s.store_add_scaled_product_indices(115, 109, 0.3333333333333333, 107, 110, (-1.0));
            s.store_div_scaled_product_denominator_ad(116, 111, 107, 1.0, A::add_scaled_product(s.ad_value(113), 1.0, s.ad_value(114), s.ad_value(115), 1.0), 1.0);
            s.store_add(117, 106, 116);
            s.store_limited_exp_sub(118, 117, 98);
            s.store_add_scaled_inputs_products_mixed_iiiaii(119, 214, 2.0, 117, (-2.0), 48, A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0), (-2.0), 296, 118, 1.0);
            s.store_add_scaled_value_products(120, A::mul3(s.ad_value(48), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0)), (-1.0), A::sub(s.ad_value(214), s.ad_value(117)), A::sub(s.ad_value(214), s.ad_value(117)), 1.0, s.ad_value(296), A::add(s.ad_value(4), s.ad_value(118)), (-1.0));
            s.store_mul_add_scaled_sub_value_product_rhs(121, 120, 2.0, A::scale(s.ad_value(48), 2.0), 2.0, s.ad_value(296), s.ad_value(118), (((-1.0)) * (2.0)));
            s.store_div_scaled_inputs_mixed_ia(122, 120, 2.0, A::add(s.ad_value(119), A::sqrt(A::sub(A::square(s.ad_value(119)), s.ad_value(121)))), 1.0);
            s.store_add(9, 117, 122);
        }

        s.b[1644] = (((s.v[214]) as f64).abs() <= s.v[101]);
        s.v[1644] = if s.b[1644] { 1.0 } else { 0.0 };

        if (((s.b[1620] && (!s.b[1634])) && (!s.b[1639])) && s.b[1644]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(9, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(100)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        s.b[1645] = (s.v[214] < (-s.v[101]));
        s.v[1645] = if s.b[1645] { 1.0 } else { 0.0 };

        if ((((s.b[1620] && (!s.b[1634])) && (!s.b[1639])) && (!s.b[1644])) && s.b[1645]) {
            s.store_neg(10, 214);
            s.store_scaled_mul(11, 10, 96, 1.25);
            s.store_scaled_sub_offset_sqrt_square_offset(12, 11, 10.0, (-6.0), 64.0, 0.5);
            s.store_sub(13, 10, 12);
            s.store_add_scaled_square_product_mixed_iia(14, 13, 1.0, 296, A::offset(s.ad_value(12), 1.0), 1.0);
            s.store_sub_scaled_inputs(16, 13, 2.0, 296, 1.0);
            s.store_sub_ad_lhs(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.5, s.ad_value(14), 1.0), 1.0);
            s.store_add_ad_rhs(18, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.3333333333333333, s.ad_value(14), 1.0))), 1.0));
            s.store_limited_exp(28, 18);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_div_from_scalar_offset_ad(13, 1.0, A::square(s.ad_value(18)), 2.0);
            s.store_mul_square_lhs(30, 18, 13);
            s.store_mul3_affine_lhs(31, 18, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), s.ad_value(13), 13);
            s.store_sub(13, 10, 18);
            s.store_mul(33, 100, 29);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(33), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(100), 1.0, s.ad_value(31)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(18), (-1.0), s.ad_value(33), 1.0, (-1.0)), 1.0, s.ad_value(100), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30)), 1.0), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(28), 1.0, s.ad_value(33), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(9, 18, -1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if ((((s.b[1620] && (!s.b[1634])) && (!s.b[1639])) && (!s.b[1644])) && (!s.b[1645])) {
            s.store_div_from_scalar_offset_scaled_input(38, 1.0, 294, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(39, A::mul_scaled_lhs(s.ad_value(95), 1.25, s.ad_value(38)), (-1.0), 38);
            s.store_mul_ad_product_rhs(40, 214, s.ad_value(96), A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));
            s.store_limited_exp_neg_input(13, 40);
            s.store_sub_from_scalar(41, 1.0, 13);
            s.store_add_scaled_inputs_product_right_ad(42, 214, 1.0, 296, 0.5, 294, A::sqrt(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(296), 0.25, s.ad_value(41), -1.0)), (-1.0));
            s.store_offset(43, 98, 3.0);
            s.store_sub_ad(12, A::add_scaled_inputs3(s.ad_value(42), 0.5, s.ad_value(43), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(42), s.ad_value(43)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(43), 0.5, A::sqrt_square_offset(s.ad_value(43), 5.0), 0.5));
            s.store_sub(13, 214, 12);
            s.store_limited_exp_neg_input(33, 12);
            s.store_div_from_scalar_offset_ad(34, 1.0, A::square(s.ad_value(12)), 2.0);
            s.store_mul_square_lhs(30, 12, 34);
            s.store_mul3_affine_lhs(31, 12, 34, 4.0, 0.0, 34);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(34), 8.0, s.ad_value(30), 12.0), s.ad_value(34), 34);
            s.store_max_from_scalar_ad(14, 1e-40, A::add_scaled_square_product(s.ad_value(13), 1.0, s.ad_value(296), A::add_scaled_product(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), 1.0, s.ad_value(100), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(15, 1.0, 296, A::add_scaled_product(s.ad_value(33), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(16, 13, 2.0, 296, A::add_scaled_sub_value_product(1.0, s.ad_value(33), 1.0, s.ad_value(100), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(17, 98, 1.0, 12, (-1.0), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)), 1.0);
            s.store_add(0, 14, 16);
            s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::add_scaled_square_product(s.ad_value(16), 0.5, s.ad_value(14), s.ad_value(15), (-1.0)), 1.0);
            s.store_add_ad_rhs(44, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::add_scaled_square_product(s.ad_value(16), 0.3333333333333333, s.ad_value(14), s.ad_value(15), (-1.0)))), 1.0));
            s.store_limited_exp(28, 44);
            s.store_div_from_scalar(29, 1.0, 28);
            s.store_limited_exp_sub(28, 44, 98);
            s.store_div_from_scalar_offset_ad(13, 1.0, A::square(s.ad_value(44)), 2.0);
            s.store_mul_square_lhs(30, 44, 13);
            s.store_mul3_affine_lhs(31, 44, 13, 4.0, 0.0, 13);
            s.store_mul_ad_product_lhs(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), s.ad_value(13), 13);
            s.store_sub(13, 214, 44);
            s.store_add_scaled_product_right_ad(36, 13, 2.0, 296, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(29)), 1.0, s.ad_value(28), 1.0, s.ad_value(100), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(29), 1.0, s.ad_value(44), 1.0, s.ad_value(28), 1.0, (-1.0)), 1.0, s.ad_value(100), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(29), 1.0, s.ad_value(28), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(9, 44, 1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);
        }

        if (s.b[1620] && (!s.b[1634])) {
            s.copy_ad(123, 9);
        }

        let (assign22300_e33112,) = {
    if (s.b[1620] && (!s.b[1634])) {
        (1e-7,)
    } else {
        (s.v[102],)
    }
};
        s.v[102] = assign22300_e33112;

        if (s.b[1620] && (!s.b[1634])) {
            s.store_scalar(103, 2.0);
            s.store_scaled_square(35, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_offset_ad(24, A::mul_scaled_output(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * (p.p74 * p.p74)) * 1.0 / ((2.0 * s.v[180])))), p.p294);
            s.store_div_scaled_inputs_indices(167, 726, (-s.v[184]), 300, 1.0);
            s.store_mul_ad_product_rhs(168, 725, A::add_scaled_inputs(A::limited_exp_scaled_input(s.ad_value(167), 0.5), 1.0, A::limited_exp(s.ad_value(167)), 2.0), A::add_scaled_products(s.ad_value(226), s.ad_value(270), 1.0, s.ad_value(724), s.ad_value(270), 1.0));
            s.store_add_scaled_inputs_product_mixed_aaai(6, A::add_scaled_product(s.ad_value(24), 1.0, A::mul3(s.ad_value(3), s.ad_value(216), s.ad_value(269)), s.ad_value(727), (-1.0)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), s.ad_value(168), s.ad_value(269)), 1.0, A::offset(s.ad_value(3), 1.0), 46, 1.0);
        }

        s.b[1646] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1646] = if s.b[1646] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1634])) && s.b[1646]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(23, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1646])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_product3_rhs(45, 167, s.ad_value(214), s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, A::limited_exp_scaled_input(s.ad_value(98), -1.0)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
            s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1646])) {
            let assign22440_ad_e33441: A = {
                if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                } else {
                    let assign22440_ad_e33440: A = {
                        if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                        } else {
                            {
                                if ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign22440_ad_e33440
                }
            };
            let assign22440_ad_e33523: A = {
                if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                } else {
                    let assign22440_ad_e33522: A = {
                        if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                        } else {
                            {
                                if ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign22440_ad_e33522
                }
            };
            s.store_sub_ad(169, assign22440_ad_e33441, assign22440_ad_e33523);
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1646])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_neg_input(176, 98);
        }

    }

    pub(super) fn stamp_transient_block_14(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1646])) {
            s.store_limited_exp_sub(177, 123, 98);
            s.store_add_scaled_value_products(19, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123)), 1.0, s.ad_value(296), A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1646])) {
            let assign22530_ad_e33740: A = A::add_scaled_inputs3(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0);
            s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add(assign22530_ad_e33740, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0)), 1.0, (-1.0));
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1646])) {
            let assign22540_ad_e33810: A = A::sub(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0));
            let assign22540_ad_e33869: A = A::mul(A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0));
            let assign22540_ad_e33898: A = A::sub(A::add_scaled_product(assign22540_ad_e33810, 1.0, s.ad_value(296), A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0, assign22540_ad_e33869, -1.0), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign22540_ad_e33898, 2.0);
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1646])) {
            s.store_add_scaled_offset_product_rhs_mixed_iaa(23, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));
        }

        if (s.b[1620] && (!s.b[1634])) {
            s.copy_ad(123, 23);
        }

        s.b[1647] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1647] = if s.b[1647] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1634])) && s.b[1647]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(23, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1647])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_product3_rhs(45, 167, s.ad_value(214), s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, A::limited_exp_scaled_input(s.ad_value(98), -1.0)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
            s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1647])) {
            let assign22640_ad_e34153: A = {
                if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                } else {
                    let assign22640_ad_e34152: A = {
                        if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                        } else {
                            {
                                if ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign22640_ad_e34152
                }
            };
            let assign22640_ad_e34235: A = {
                if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                } else {
                    let assign22640_ad_e34234: A = {
                        if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                        } else {
                            {
                                if ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign22640_ad_e34234
                }
            };
            s.store_sub_ad(169, assign22640_ad_e34153, assign22640_ad_e34235);
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1647])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_neg_input(176, 98);
            s.store_limited_exp_sub(177, 123, 98);
            s.store_add_scaled_value_products(19, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123)), 1.0, s.ad_value(296), A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1647])) {
            let assign22730_ad_e34452: A = A::add_scaled_inputs3(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0);
            s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add(assign22730_ad_e34452, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0)), 1.0, (-1.0));
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1647])) {
            let assign22740_ad_e34522: A = A::sub(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0));
            let assign22740_ad_e34581: A = A::mul(A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0));
            let assign22740_ad_e34610: A = A::sub(A::add_scaled_product(assign22740_ad_e34522, 1.0, s.ad_value(296), A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0, assign22740_ad_e34581, -1.0), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign22740_ad_e34610, 2.0);
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1647])) {
            s.store_add_scaled_offset_product_rhs_mixed_iaa(23, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));
        }

        if (s.b[1620] && (!s.b[1634])) {
            s.copy_ad(123, 23);
        }

        s.b[1648] = (((s.v[214]) as f64).abs() <= s.v[102]);
        s.v[1648] = if s.b[1648] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1634])) && s.b[1648]) {
            s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(23, 214, s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1648])) {
            s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);
            s.store_mul_product3_rhs(45, 167, s.ad_value(214), s.ad_value(96), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, A::limited_exp_scaled_input(s.ad_value(98), -1.0)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
            s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1648])) {
            let assign22840_ad_e34865: A = {
                if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                } else {
                    let assign22840_ad_e34864: A = {
                        if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                        } else {
                            {
                                if ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign22840_ad_e34864
                }
            };
            let assign22840_ad_e34947: A = {
                if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                } else {
                    let assign22840_ad_e34946: A = {
                        if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                        } else {
                            {
                                if ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign22840_ad_e34946
                }
            };
            s.store_sub_ad(169, assign22840_ad_e34865, assign22840_ad_e34947);
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1648])) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_neg_input(176, 98);
            s.store_limited_exp_sub(177, 123, 98);
            s.store_add_scaled_value_products(19, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123)), 1.0, s.ad_value(296), A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1648])) {
            let assign22930_ad_e35164: A = A::add_scaled_inputs3(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0);
            s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add(assign22930_ad_e35164, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0)), 1.0, (-1.0));
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1648])) {
            let assign22940_ad_e35234: A = A::sub(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0));
            let assign22940_ad_e35293: A = A::mul(A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0));
            let assign22940_ad_e35322: A = A::sub(A::add_scaled_product(assign22940_ad_e35234, 1.0, s.ad_value(296), A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0, assign22940_ad_e35293, -1.0), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));
            s.store_offset_ad(21, assign22940_ad_e35322, 2.0);
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1648])) {
            s.store_add_scaled_offset_product_rhs_mixed_iaa(23, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));
        }

        if (s.b[1620] && (!s.b[1634])) {
            s.store_sub(62, 23, 22);
            s.store_mul(63, 226, 270);
            s.store_limited_exp_neg_input(64, 63);
        }

        s.b[1649] = (s.v[62] < 1e-10);
        s.v[1649] = if s.b[1649] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1634])) && s.b[1649]) {
            s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
            s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));
        }

        if ((s.b[1620] && (!s.b[1634])) && s.b[1649]) {
            let assign23020_ad_e35502: A = {
                if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                } else {
                    let assign23020_ad_e35501: A = {
                        if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                        } else {
                            {
                                if ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign23020_ad_e35501
                }
            };
            let assign23020_ad_e35584: A = {
                if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                } else {
                    let assign23020_ad_e35583: A = {
                        if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                        } else {
                            {
                                if ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign23020_ad_e35583
                }
            };
            s.store_sub_ad(169, assign23020_ad_e35502, assign23020_ad_e35584);
        }

        if ((s.b[1620] && (!s.b[1634])) && s.b[1649]) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));
            s.store_limited_exp(171, 170);
            s.store_limited_exp_ad(172, A::add(s.ad_value(170), A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0)));
            s.store_limited_exp_neg_input(173, 123);
            s.store_square(174, 123);
            s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);
            s.store_limited_exp_sub(177, 123, 98);
            s.store_limited_exp_ad(178, A::div_scaled_add_product(s.ad_value(6), ((-1.0) * 2.0), s.ad_value(123), s.ad_value(269), 2.0, s.ad_value(269), 1.0));
            s.store_limited_exp_ad(179, A::add(A::div_scaled_add_product(s.ad_value(6), ((-1.0) * 2.0), s.ad_value(123), s.ad_value(269), 2.0, s.ad_value(269), 1.0), s.ad_value(170)));
        }

        if ((s.b[1620] && (!s.b[1634])) && s.b[1649]) {
            let assign23120_ad_e35785: A = A::add_scaled_offset_product_rhs(A::limited_exp(A::add_scaled_inputs3(s.ad_value(123), 1.0, s.ad_value(63), (-1.0), s.ad_value(98), -1.0)), 1.0, A::limited_exp(A::sub_scaled_inputs(s.ad_value(63), -1.0, s.ad_value(98), 1.0)), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0);
            let assign23120_ad_e35811: A = A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, s.ad_value(296), A::add_scaled_inputs4(assign23120_ad_e35785, 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div(s.ad_value(172), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), 1.0), 1.0, (-1.0));
            s.store_neg_ad(65, assign23120_ad_e35811);
        }

        if ((s.b[1620] && (!s.b[1634])) && s.b[1649]) {
            s.store_mul_ad_lhs(66, A::mul_sub_from_scalar_rhs(s.ad_value(296), 1.0, s.ad_value(64)), 57);
        }

        if ((s.b[1620] && (!s.b[1634])) && s.b[1649]) {
            let assign23140_ad_e35878: A = A::sub(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), A::div_scaled_product3(s.ad_value(2), s.ad_value(2), s.ad_value(178), 2.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(3), 1.0), A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178))), 1.0));
            let assign23140_ad_e35923: A = A::mul(A::limited_exp(A::sub_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(63), 1.0)), A::sub(A::add_scaled_product(s.ad_value(175), (-2.0), A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(175), 10.0), s.ad_value(175), 1.0), A::mul3(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 8.0), s.ad_value(123), s.ad_value(175)), s.ad_value(175), s.ad_value(175))));
            let assign23140_ad_e35948: A = A::add(A::add_scaled_inputs4(s.ad_value(173), 1.0, A::limited_exp(A::add_scaled_inputs3(s.ad_value(123), 1.0, s.ad_value(98), (-1.0), s.ad_value(63), -1.0)), 1.0, assign23140_ad_e35923, 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0), A::div(s.ad_value(178), A::mul_offset_lhs(s.ad_value(3), 1.0, A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178)))));
            let assign23140_ad_e35990: A = A::add_scaled_inputs4(assign23140_ad_e35948, 1.0, A::div(s.ad_value(172), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), 1.0, A::div(s.ad_value(179), A::mul_offset_lhs(s.ad_value(3), 1.0, A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178)))), -1.0, A::div(s.ad_value(179), A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(3), 1.0), A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178)))), -1.0);
            s.store_offset_sub_ad(54, A::add_scaled_product(assign23140_ad_e35878, 1.0, s.ad_value(296), assign23140_ad_e35990, (-1.0)), A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(178), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::add(A::scale_offset(s.ad_value(167), 2.0, 1.0), s.ad_value(178)), 1.0), 2.0);
        }

        if ((s.b[1620] && (!s.b[1634])) && s.b[1649]) {
            s.store_add_scaled_square_product_indices(54, 65, 1.0, 54, 66, (-2.0));
        }

        s.b[1650] = (s.v[54] >= 0.0);
        s.v[1650] = if s.b[1650] { 1.0 } else { 0.0 };

        if (((s.b[1620] && (!s.b[1634])) && s.b[1649]) && s.b[1650]) {
            s.store_scaled_div_ad_rhs(62, 66, A::add(s.ad_value(65), A::sqrt(s.ad_value(54))), 2.0);
        }

        if ((s.b[1620] && (!s.b[1634])) && s.b[1649]) {
            s.store_add(23, 22, 62);
        }

        if (s.b[1620] && (!s.b[1634])) {
            s.store_mul(250, 62, 269);
            s.store_div_scaled_product_offset_denominator(67, s.ad_value(23), s.ad_value(23), 1.0, A::square(s.ad_value(23)), 2.0, 1.0);
            s.store_limited_exp_neg_input(68, 23);
            s.store_add_scaled_product(69, A::limited_exp(A::sub(s.ad_value(23), s.ad_value(98))), 1.0, A::limited_exp_scaled_input(s.ad_value(98), -1.0), A::add(A::offset(s.ad_value(23), 1.0), s.ad_value(67)), (-1.0));
            s.store_sub_ad_lhs(70, A::mul3(A::sub(s.ad_value(214), s.ad_value(23)), A::sub(s.ad_value(214), s.ad_value(23)), A::div_from_scalar(1.0, s.ad_value(296))), 69);
            s.store_offset_add_scaled_inputs(70, A::offset(s.ad_value(70), (-0.001)), 0.5, A::sqrt_offset_square_offset(s.ad_value(70), (-0.001), ((4.0 * 1e-5) * 1e-5)), 0.5, 0.001);
            s.store_sqrt(60, 70);
            s.store_mul_sqrt_ad_rhs(72, 294, A::add(s.ad_value(70), s.ad_value(69)));
            s.store_div_scaled_product3_mixed_iiia(73, 296, 69, 269, 1.0, A::add_scaled_product(s.ad_value(72), 1.0, s.ad_value(294), s.ad_value(60), 1.0), 1.0);
            s.store_scaled_add(75, 22, 23, 0.5);
            s.store_abs_ad(54, A::mul(s.ad_value(68), s.ad_value(56)));
            s.store_sqrt(76, 54);
            s.store_scaled_add(77, 57, 69, 0.5);
            s.store_add_scaled_product_mixed_iaa(78, 77, 1.0, A::square(s.ad_value(62)), A::sub_scaled_inputs(s.ad_value(76), 1.0, s.ad_value(297), 2.0), 0.125);
            s.store_sub_ad_lhs(79, A::mul3(A::sub(s.ad_value(214), s.ad_value(75)), A::sub(s.ad_value(214), s.ad_value(75)), A::div_from_scalar(1.0, s.ad_value(296))), 78);
            s.store_mul_sqrt_ad_rhs(51, 294, A::add(s.ad_value(78), s.ad_value(79)));
            s.store_offset_add_scaled_inputs(79, A::offset(s.ad_value(79), (-0.001)), 0.5, A::sqrt_offset_square_offset(s.ad_value(79), (-0.001), ((4.0 * 1e-5) * 1e-5)), 0.5, 0.001);
            s.store_sqrt(71, 79);
        }

        s.b[1651] = (p.p46 == 1.0);
        s.v[1651] = if s.b[1651] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1634])) && s.b[1651]) {
            s.store_div_scaled_inputs_indices(85, 269, ((2.0 * s.v[199]) * s.v[199]), 704, (1.602176462e-19 * s.v[180]));
            s.store_add_scaled_sub_value_product_mixed_iia(86, 1.0, 76, 1.0, 51, A::div_from_scalar(1.0, s.ad_value(296)), 2.0);
            s.store_div_from_scalar_sqrt_ad(87, 1.0, A::offset(A::mul(s.ad_value(85), s.ad_value(51)), 1.0));
            s.store_div_scaled_value_offset_denominator(54, s.ad_value(87), 1.0, s.ad_value(87), 1.0, 1.0);
            s.store_mul_ad_product_rhs(88, 85, A::mul3(A::square(s.ad_value(54)), s.ad_value(51), s.ad_value(51)), A::div(s.ad_value(78), A::add(s.ad_value(78), s.ad_value(79))));
            s.store_add_scaled_inputs_product_right_ad(89, 51, 2.0, 88, (-2.0), 296, A::add(A::sub_from_scalar(1.0, s.ad_value(76)), s.ad_value(78)), 1.0);
        }

    }

    pub(super) fn stamp_transient_block_15(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1620] && (!s.b[1634])) && s.b[1651]) {
            s.store_mul_sub_scaled_inputs_rhs(90, 88, s.ad_value(88), 1.0, s.ad_value(51), 2.0);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(91, 1.0, 296, A::add(s.ad_value(76), s.ad_value(78)), 0.5);
            s.store_div_scaled_product_denominator_ad(92, 90, 89, 1.0, A::add_scaled_square_product(s.ad_value(89), 1.0, s.ad_value(91), s.ad_value(90), (-1.0)), 1.0);
            s.store_add(75, 75, 92);
            s.store_limited_exp(93, 92);
            s.store_div(76, 76, 93);
            s.store_mul(78, 78, 93);
            s.store_sub_ad(79, A::mul3(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(75), (-1.0), s.ad_value(92), 1.0), A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(75), (-1.0), s.ad_value(92), 1.0), A::div_from_scalar(1.0, s.ad_value(296))), A::div(s.ad_value(78), s.ad_value(93)));
            s.store_mul_sqrt_ad_rhs(51, 294, A::add(s.ad_value(78), s.ad_value(79)));
            s.store_add_ad(94, A::sub_from_scalar(1.0, s.ad_value(76)), A::mul3_scaled_output(s.ad_value(51), s.ad_value(87), s.ad_value(297), 2.0));
            s.store_div_scaled_product3_mixed_iiaa(62, 62, 93, A::add(s.ad_value(86), s.ad_value(77)), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(93), s.ad_value(77), 1.0), 1.0);
            s.store_mul(250, 62, 269);
            s.store_offset_add_scaled_inputs(79, A::offset(s.ad_value(79), (-0.001)), 0.5, A::sqrt_offset_square_offset(s.ad_value(79), (-0.001), ((4.0 * 1e-5) * 1e-5)), 0.5, 0.001);
            s.store_sqrt(71, 79);
        }

        s.b[1652] = (((s.v[250]) as f64).abs() > 1e-35);
        s.v[1652] = if s.b[1652] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1634])) && s.b[1652]) {
            s.store_div_scaled_inputs2_indices(74, 306, 1.0, 73, (-1.0), 250, 1.0);
        }

        if (s.b[1620] && (!s.b[1634])) {
            s.store_mul_div_scaled_product_rhs(80, 269, s.ad_value(296), s.ad_value(78), 1.0, A::add_scaled_product(s.ad_value(51), 1.0, s.ad_value(294), s.ad_value(71), 1.0), 1.0);
            s.store_mul3_lhs(82, 71, 294, 269);
            s.store_mul(52, 51, 269);
            s.store_mul_add_scaled_inputs_rhs(336, 335, s.ad_value(82), 1.0, s.ad_value(80), s.v[338]);
            s.store_limited_exp_ad(169, A::mul(s.ad_value(757), A::ln(A::max_with_scalar(A::scaled_offset(A::div(s.ad_value(80), s.ad_value(82)), 1.0, 0.5), 1e-38))));
            s.store_add_scaled_product(170, A::div(s.ad_value(754), s.ad_value(169)), 1.0, A::add_scaled_product(s.ad_value(750), 1.0, s.ad_value(760), s.ad_value(218), 1.0), A::pow(s.ad_value(336), s.ad_value(651)), 1.0);
            s.store_offset(171, 170, 1.0);
            s.store_scaled_add_offset_sqrt_square_offset(339, 171, 1.0, (-1.0), ((0.25 * 0.0015) * 0.0015), 0.5);
            s.store_div_scaled_inputs_mixed_ia(310, 746, 2.0, A::div(s.ad_value(740), s.ad_value(339)), 1.0);
            s.store_scale(311, 310, s.v[184]);
        }

        s.b[1653] = (s.v[781] > 0.0);
        s.v[1653] = if s.b[1653] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1634])) && s.b[1653]) {
            s.store_offset_div_scaled_product(360, s.ad_value(781), s.ad_value(80), 1.0, s.ad_value(311), 1.0, 1.0);
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1653])) {
            s.store_div_from_scalar_sub_from_scalar_ad(360, 1.0, 1.0, A::div_scaled_product(s.ad_value(781), s.ad_value(80), 1.0, s.ad_value(311), 1.0));
        }

        if (s.b[1620] && (!s.b[1634])) {
            s.copy_ad(359, 763);
            s.store_sub(355, 226, 315);
            s.store_add_scaled_inputs(362, 80, 1.0, 269, 2.0);
        }

        s.b[1654] = (s.v[359] > 0.0);
        s.v[1654] = if s.b[1654] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1634])) && s.b[1654]) {
            s.store_div_add_scaled_inputs_rhs_indices(170, 362, 312, 1.0, 362, 1.0);
            s.store_scaled_add_offset_sqrt_square_offset_ad(171, A::mul(s.ad_value(764), s.ad_value(218)), 1.0, 1.0, ((4.0 * 0.001) * 0.001), 0.5);
            s.store_div_from_scalar(172, 1.0, 171);
            s.store_mul_product3_rhs(361, 172, A::div(s.ad_value(362), s.ad_value(359)), s.ad_value(170), s.ad_value(360), 1.0);
            s.store_offset_div(363, 355, 361, 1.0);
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1654])) {
            s.store_scalar(363, 1.0);
        }

        s.b[1655] = (s.v[769] <= 0.0);
        s.v[1655] = if s.b[1655] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1634])) && s.b[1655]) {
            s.store_scalar(268, 1.0);
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1655])) {
            s.store_div_scaled_inputs_indices(176, 769, ((s.v[184]) as f64).sqrt(), 362, 1.0);
            s.store_div_from_scalar_offset_input(268, 1.0, 176, 1.0);
        }

        if (s.b[1620] && (!s.b[1634])) {
            s.store_add(358, 312, 311);
        }

        s.b[1656] = (s.v[785] > 0.0);
        s.v[1656] = if s.b[1656] { 1.0 } else { 0.0 };

        s.b[1657] = (p.p414 < 0.0);
        s.v[1657] = if s.b[1657] { 1.0 } else { 0.0 };

        if (((s.b[1620] && (!s.b[1634])) && s.b[1656]) && s.b[1657]) {
            s.store_div_scaled_value_by_product(168, s.ad_value(785), 1.0, A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(80), p.p414, s.ad_value(311), 1.0)), s.ad_value(268), 1.0);
        }

        if (((s.b[1620] && (!s.b[1634])) && s.b[1656]) && (!s.b[1657])) {
            s.store_div_scaled_product_offset_rhs(168, s.ad_value(785), A::div_scaled_inputs(s.ad_value(80), p.p414, s.ad_value(311), 1.0), 1.0, 1.0, s.ad_value(268), 1.0);
        }

        if ((s.b[1620] && (!s.b[1634])) && s.b[1656]) {
            s.store_offset_mul_ad(364, s.ad_value(168), A::ln(A::max_with_scalar(A::offset(A::div_scaled_value_by_product(s.ad_value(355), 1.0, s.ad_value(168), s.ad_value(358), 1.0), 1.0), 1e-38)), 1.0);
        }

        s.b[1658] = (p.p414 < 0.0);
        s.v[1658] = if s.b[1658] { 1.0 } else { 0.0 };

        if (((s.b[1620] && (!s.b[1634])) && (!s.b[1656])) && s.b[1658]) {
            s.store_div_scaled_value_by_product(168, s.ad_value(785), 1.0, A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(80), p.p414, s.ad_value(311), 1.0)), s.ad_value(268), 1.0);
        }

        if (((s.b[1620] && (!s.b[1634])) && (!s.b[1656])) && (!s.b[1658])) {
            s.store_div_scaled_product_offset_rhs(168, s.ad_value(785), A::div_scaled_inputs(s.ad_value(80), p.p414, s.ad_value(311), 1.0), 1.0, 1.0, s.ad_value(268), 1.0);
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1656])) {
            s.store_offset(364, 168, 1.0);
        }

        if (s.b[1620] && (!s.b[1634])) {
            s.store_mul(363, 363, 364);
            s.store_limited_exp_mul(168, 768, 226);
        }

        s.b[1659] = (s.v[767] > 0.0);
        s.v[1659] = if s.b[1659] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1634])) && s.b[1659]) {
            s.store_scalar(169, (1.0 + (p.p433 * s.v[184])));
            s.store_div_scaled_offset_numerator(356, A::mul(s.ad_value(169), s.ad_value(168)), 1.0, 1.0, s.ad_value(767), 1.0);
            s.store_mul(356, 356, 268);
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1659])) {
            s.store_scalar(356, 5.540622384e34);
        }

        if (s.b[1620] && (!s.b[1634])) {
            s.store_div(171, 355, 356);
            s.store_offset(167, 171, 1.0);
            s.store_mul(363, 363, 167);
        }

        s.b[1660] = (s.v[766] > 0.0);
        s.v[1660] = if s.b[1660] { 1.0 } else { 0.0 };

        s.b[1661] = (s.v[355] > ((s.v[765] * s.v[300]) / 80.0));
        s.v[1661] = if s.b[1661] { 1.0 } else { 0.0 };

        if (((s.b[1620] && (!s.b[1634])) && s.b[1660]) && s.b[1661]) {
            s.store_div_scaled_product_indices(167, 765, 300, 1.0, 355, 1.0);
            s.store_div_scaled_inputs_mixed_ai(357, A::limited_exp(s.ad_value(167)), s.v[184], 766, 1.0);
        }

        if (((s.b[1620] && (!s.b[1634])) && s.b[1660]) && (!s.b[1661])) {
            s.store_div_from_scalar(357, (5.540622384e34 * s.v[184]), 766);
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1660])) {
            s.store_scalar(357, 5.540622384e34);
        }

        if (s.b[1620] && (!s.b[1634])) {
            s.store_offset_div(365, 355, 357, 1.0);
            s.store_mul(363, 363, 365);
        }

        s.b[1662] = (s.v[678] < 0.0);
        s.v[1662] = if s.b[1662] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1634])) && s.b[1662]) {
            s.store_div_from_scalar_sub_from_scalar_ad(349, 1.0, 1.0, A::mul(s.ad_value(678), s.ad_value(218)));
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1662])) {
            s.store_offset_mul(349, 678, 218, 1.0);
        }

        if (s.b[1620] && (!s.b[1634])) {
            s.store_mul(167, 80, 349);
            s.store_div_scaled_value_offset_denominator(350, s.ad_value(167), 100.0, s.ad_value(167), 100.0, 1.0);
            s.store_scalar(352, (1.0 / p.p503));
            s.store_ln_ad(167, A::div_scaled_offset_numerator(A::mul(A::sub(s.ad_value(226), s.ad_value(250)), s.ad_value(352)), 1.0, 1.0, A::offset(A::mul(A::sub(s.ad_value(315), s.ad_value(250)), s.ad_value(352)), 1.0), 1.0));
            s.store_scale(353, 167, p.p504);
            s.store_div_from_scalar_add_ad(354, 1.0, A::offset(s.ad_value(353), 1.0), A::square(s.ad_value(353)));
            s.store_mul(341, 339, 354);
        }

        s.b[1663] = (s.v[346] < 0.0);
        s.v[1663] = if s.b[1663] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1634])) && s.b[1663]) {
            s.store_div_from_scalar_sub_from_scalar_ad(168, 1.0, 1.0, A::mul(s.ad_value(346), s.ad_value(350)));
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1663])) {
            s.store_offset_mul(168, 346, 350, 1.0);
        }

        if (s.b[1620] && (!s.b[1634])) {
            s.store_mul_div_rhs(351, 744, 168, 341);
            s.store_mul_ad_product_lhs(342, A::square(s.ad_value(351)), s.ad_value(250), 250);
        }

        s.b[1664] = (p.p30 == (-1.0));
        s.v[1664] = if s.b[1664] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1634])) && s.b[1664]) {
            s.store_div_scaled_value_offset_denominator(342, s.ad_value(342), 1.0, A::mul(s.ad_value(351), s.ad_value(250)), 1.0, 1.0);
        }

        if (s.b[1620] && (!s.b[1634])) {
            s.store_mul_offset_rhs_scaled_ad_rhs(343, 341, A::sqrt(A::scale_offset(s.ad_value(342), 2.0, 1.0)), 1.0, 0.5);
            s.store_div_from_scalar(344, 1.0, 343);
            s.store_scalar(454, 0.0);
            s.store_scalar(455, 0.0);
            s.store_add(243, 306, 73);
        }

        s.b[1665] = (p.p33 == 1.0);
        s.v[1665] = if s.b[1665] { 1.0 } else { 0.0 };

        if ((s.b[1620] && (!s.b[1634])) && s.b[1665]) {
            s.store_scalar(457, 0.0);
            s.store_scalar(458, 1.0);
            s.store_sub(169, 203, 219);
            s.store_sqrt_square_offset(170, 169, 0.01);
            s.store_scaled_add(228, 169, 170, 0.5);
            s.store_offset_mul(172, 770, 228, 1.0);
            s.store_add_scaled_product_value_ad(173, A::div_from_scalar(1.0, s.ad_value(172)), 1.0, 787, 202, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(171, 173, 173, 0.01, 0.5);
            s.store_mul_add_scaled_product_rhs(454, 652, s.ad_value(452), 1.0, A::add_scaled_product(s.ad_value(773), 1.0, s.ad_value(775), s.ad_value(171), 1.0), s.ad_value(448), 1.0);
            s.store_sub(169, 204, 219);
            s.store_sqrt_square_offset(170, 169, 0.01);
            s.store_scaled_add(229, 169, 170, 0.5);
            s.store_offset_mul(172, 770, 229, 1.0);
            s.store_add_scaled_product_value_ad(173, A::div_from_scalar(1.0, s.ad_value(172)), 1.0, 787, 201, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(171, 173, 173, 0.01, 0.5);
            s.store_mul_add_scaled_product_rhs(455, 652, s.ad_value(453), 1.0, A::add_scaled_product(s.ad_value(772), 1.0, s.ad_value(774), s.ad_value(171), 1.0), s.ad_value(448), 1.0);
        }

        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1665])) {
            s.store_offset_mul(167, 770, 243, 1.0);
            s.store_mul_sub_rhs(168, 787, 274, 299);
            s.store_add_ad_lhs(169, A::div_from_scalar(1.0, s.ad_value(167)), 168);
            s.store_scaled_add_sqrt_square_offset_rhs(170, 169, 169, 0.01, 0.5);
            s.store_mul_ad_affine_product_lhs(457, s.ad_value(652), A::add_scaled_product(s.ad_value(777), 1.0, s.ad_value(776), s.ad_value(170), 1.0), p.p2, 0.0, 448);
            s.copy_ad(455, 453);
            s.copy_ad(454, 452);
            s.store_offset_product3(458, A::div(s.ad_value(740), s.ad_value(343)), s.ad_value(243), s.ad_value(457), ((s.v[199] * s.v[183]) * 1.0 / (s.v[184])), 1.0);
        }

        s.b[1666] = (p.p33 == 2.0);
        s.v[1666] = if s.b[1666] { 1.0 } else { 0.0 };

        if (((s.b[1620] && (!s.b[1634])) && (!s.b[1665])) && s.b[1666]) {
            s.store_mul_add_ad_rhs(457, 652, A::add_scaled_product(s.ad_value(452), 1.0, A::add_scaled_product(s.ad_value(777), 1.0, s.ad_value(776), s.ad_value(170), 1.0), s.ad_value(448), p.p2), s.ad_value(453));
            s.store_scalar(455, 0.0);
            s.store_scalar(454, 0.0);
            s.store_offset_product3(458, A::div(s.ad_value(740), s.ad_value(343)), s.ad_value(243), s.ad_value(457), ((s.v[199] * s.v[183]) * 1.0 / (s.v[184])), 1.0);
        }

        if (s.b[1620] && (!s.b[1634])) {
            s.store_add_ad_rhs(167, 330, A::div(s.ad_value(333), A::add_scaled_product(s.ad_value(243), 1.0, s.ad_value(267), s.ad_value(637), 2.0)));
            s.store_sub(416, 306, 73);
            s.store_mul3_lhs(168, 167, 416, 416);
            s.store_offset(169, 168, ((1.0) + ((-0.001))));
            s.store_offset_add_scaled_inputs_mixed_ia(170, 169, 0.5, A::sqrt_square_offset(s.ad_value(169), 0.004), 0.5, (-1.0));
            s.store_scaled_offset_ad(334, A::sqrt(A::offset(s.ad_value(170), 1.0)), 1.0, 0.5);
            s.store_offset_sub_scaled_inputs(334, A::offset(s.ad_value(334), 1.0), 0.5, A::sqrt_offset_square_offset(s.ad_value(334), (-1.0), ((0.25 * 0.01) * 0.01)), 0.5, (0.25 * 0.01));
            s.store_add(167, 306, 73);
        }

    }
}
