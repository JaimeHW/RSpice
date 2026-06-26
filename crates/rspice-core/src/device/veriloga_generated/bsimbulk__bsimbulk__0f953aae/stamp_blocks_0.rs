#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.v[485] = 0.0;

        s.v[466] = 0.0;

        s.v[535] = 0.0;

        s.v[505] = 0.0;

        s.v[512] = 0.0;

        s.v[510] = 0.0;

        s.v[467] = 0.0;

        s.v[649] = 0.0;

        s.v[661] = 0.0;

        s.v[669] = 0.0;

        s.v[606] = 0.0;

        s.v[610] = 0.0;

        s.v[616] = 0.0;

        s.v[620] = 0.0;

        s.v[624] = 0.0;

        s.v[628] = 0.0;

        s.v[634] = 0.0;

        s.v[638] = 0.0;

        s.v[780] = 0.0;

        s.v[781] = 0.0;

        s.v[491] = 0.0;

        s.v[540] = 0.0;

        s.v[414] = 0.0;

        s.v[400] = 0.0;

        s.v[406] = 0.0;

        s.v[501] = 0.0;

        s.v[650] = 0.0;

        s.v[699] = 0.0;

        s.v[670] = 0.0;

        s.v[607] = 0.0;

        s.v[613] = 0.0;

        s.v[617] = 0.0;

        s.v[621] = 0.0;

        s.v[625] = 0.0;

        s.v[631] = 0.0;

        s.v[635] = 0.0;

        s.v[639] = 0.0;

        s.v[762] = 1.0;

        s.v[421] = 0.0;

        s.v[518] = 0.0;

        s.v[498] = 0.0;

        s.v[515] = 0.0;

        s.v[509] = 0.0;

        s.v[410] = 0.0;

        s.v[688] = 0.0;

        s.v[690] = 0.0;

        s.v[671] = 0.0;

        s.v[608] = 0.0;

        s.v[614] = 0.0;

        s.v[618] = 0.0;

        s.v[622] = 0.0;

        s.v[626] = 0.0;

        s.v[632] = 0.0;

        s.v[636] = 0.0;

        s.v[759] = 0.0;

        s.v[763] = 1.0;

        s.v[460] = 0.0;

        s.v[165] = 0.0;

        s.v[398] = 0.0;

        s.v[402] = 0.0;

        s.v[404] = 0.0;

        s.v[461] = 0.0;

        s.v[689] = 0.0;

        s.v[605] = 0.0;

        s.v[609] = 0.0;

        s.v[615] = 0.0;

        s.v[619] = 0.0;

        s.v[623] = 0.0;

        s.v[627] = 0.0;

        s.v[633] = 0.0;

        s.v[637] = 0.0;

        s.v[761] = 0.0;

        s.v[629] = 0.0;

        s.v[630] = 0.0;

        s.v[247] = 0.0;

        s.v[246] = 0.0;

        s.v[249] = 0.0;

        s.v[248] = 0.0;

        s.v[782] = 1.0;

        s.v[783] = 1.0;

        s.v[372] = 0.0;

        s.v[371] = 0.0;

        s.v[374] = 0.0;

        s.v[373] = 0.0;

        s.v[67] = 0.0;

        s.v[71] = 0.0;

        s.v[750] = 0.0;

        s.v[147] = 0.0;

        s.v[178] = 0.0;

        s.v[183] = 0.0;

        s.v[416] = 0.0;

        s.v[552] = 0.0;

        s.v[557] = 0.0;

        s.v[760] = 0.0;

        s.b[859] = (p.p39 == 1.0);
        s.v[859] = if s.b[859] { 1.0 } else { 0.0 };

        if s.b[859] {
            s.store_scalar(187, 1.0);
        }

        if (!s.b[859]) {
            s.store_scalar(187, (-1.0));
        }

        s.v[26] = (p.p110 * 8.85418e-12);

        s.v[27] = (p.p111 * 8.85418e-12);

        s.v[46] = ((p.p111 * 8.85418e-12) / p.p77);

        s.v[47] = (p.p110 / p.p111);

        s.b[860] = (!param_given[78]);
        s.v[860] = if s.b[860] { 1.0 } else { 0.0 };

        if s.b[860] {
            s.store_scalar(229, (((p.p77 * p.p111) / 3.9) - p.p79));
        }

        if (!s.b[860]) {
            s.store_scalar(229, p.p78);
        }

        s.v[99] = (p.p0 * p.p52);

        s.v[101] = (p.p1 * p.p53);

        s.v[98] = (s.v[99] + p.p54);

        s.v[456] = (s.v[101] / p.p2);

        s.v[100] = (s.v[456] + p.p56);

        s.v[457] = ((s.v[98]) as f64).powf((-p.p61));

        s.v[458] = ((s.v[100]) as f64).powf((-p.p62));

        s.v[459] = (s.v[457] * s.v[458]);

        s.v[39] = (((p.p57 + (p.p58 * s.v[457])) + (p.p59 * s.v[458])) + (p.p60 * s.v[459]));

        s.v[463] = ((s.v[98]) as f64).powf((-p.p67));

        s.v[464] = ((s.v[100]) as f64).powf((-p.p68));

        s.v[465] = (s.v[463] * s.v[464]);

        s.v[40] = (((p.p63 + (p.p64 * s.v[463])) + (p.p65 * s.v[464])) + (p.p66 * s.v[465]));

        s.v[30] = (s.v[98] - (2.0 * s.v[39]));

        s.v[29] = (s.v[100] - (2.0 * s.v[40]));

        s.v[43] = (((p.p69 + (p.p70 * s.v[457])) + (p.p71 * s.v[458])) + (p.p72 * s.v[459]));

        s.v[44] = (((p.p73 + (p.p74 * s.v[463])) + (p.p75 * s.v[464])) + (p.p76 * s.v[465]));

        s.v[34] = (s.v[98] - (2.0 * s.v[43]));

        s.v[33] = (s.v[100] - (2.0 * s.v[44]));

        s.v[45] = (((p.p138 + (p.p74 / ((s.v[98]) as f64).powf(p.p67))) + (p.p75 / ((s.v[100]) as f64).powf(p.p68))) + ((p.p76 / ((s.v[98]) as f64).powf(p.p67)) / ((s.v[100]) as f64).powf(p.p68)));

        s.v[35] = (s.v[100] - (2.0 * s.v[45]));

        s.v[469] = (1e-6 / s.v[30]);

        s.v[470] = (1e-6 / s.v[29]);

        s.v[472] = (1e-6 / s.v[34]);

        s.v[473] = (1e-6 / s.v[33]);

        s.v[474] = (1e-6 / p.p51);

        s.v[475] = (1e-6 / p.p55);

        s.v[471] = (s.v[469] * s.v[470]);

        s.v[460] = s.v[457];

        s.v[466] = s.v[463];

        s.b[872] = (p.p818 != 0.0);
        s.v[872] = if s.b[872] { 1.0 } else { 0.0 };

        s.b[873] = (p.p818 <= (-s.v[98]));
        s.v[873] = if s.b[873] { 1.0 } else { 0.0 };

        if (s.b[872] && (!s.b[873])) {
            s.store_scalar(460, (((s.v[98] + p.p818)) as f64).powf((-p.p61)));
            s.store_scalar(466, (((s.v[98] + p.p818)) as f64).powf((-p.p67)));
        }

        s.v[461] = s.v[458];

        s.v[467] = s.v[464];

        s.b[874] = (p.p819 != 0.0);
        s.v[874] = if s.b[874] { 1.0 } else { 0.0 };

        s.b[875] = (p.p819 <= (-s.v[100]));
        s.v[875] = if s.b[875] { 1.0 } else { 0.0 };

        if (s.b[874] && (!s.b[875])) {
            s.store_scalar(461, (((s.v[100] + p.p819)) as f64).powf((-p.p62)));
            s.store_scalar(467, (((s.v[100] + p.p819)) as f64).powf((-p.p68)));
        }

        s.store_mul(462, 460, 461);

        s.store_ad_value(41, A::add_scaled_inputs3(A::scale_offset(s.ad_value(460), p.p58, p.p57), 1.0, s.ad_value(461), p.p59, s.ad_value(462), p.p60));

        s.store_mul(468, 466, 467);

        s.store_ad_value(42, A::add_scaled_inputs3(A::scale_offset(s.ad_value(466), p.p64, p.p63), 1.0, s.ad_value(467), p.p65, s.ad_value(468), p.p66));

        s.store_offset_sub_from_scalar_ad(32, s.v[98], A::scale(s.ad_value(41), 2.0), p.p818);

        s.store_offset_sub_from_scalar_ad(31, s.v[100], A::scale(s.ad_value(42), 2.0), p.p819);

        s.b[878] = (p.p817 == 1.0);
        s.v[878] = if s.b[878] { 1.0 } else { 0.0 };

        if s.b[878] {
            s.store_div_from_scalar(476, 1e-6, 32);
            s.store_div_from_scalar(477, 1e-6, 31);
        }

        if (!s.b[878]) {
            s.store_div_from_scalar(476, 1.0, 32);
            s.store_div_from_scalar(477, 1.0, 31);
        }

        s.store_mul(478, 476, 477);

        s.store_ad_value(482, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p117, p.p116), 1.0, s.ad_value(477), p.p118, s.ad_value(478), p.p119));

        s.store_ad_value(549, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p127, p.p126), 1.0, s.ad_value(477), p.p128, s.ad_value(478), p.p129));

        s.store_ad_value(480, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p140, p.p139), 1.0, s.ad_value(477), p.p141, s.ad_value(478), p.p142));

        s.store_ad_value(481, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p89, p.p80), 1.0, s.ad_value(477), p.p90, s.ad_value(478), p.p91));

        s.store_ad_value(550, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p101, p.p92), 1.0, s.ad_value(477), p.p102, s.ad_value(478), p.p103));

        s.store_ad_value(479, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p105, p.p104), 1.0, s.ad_value(477), p.p106, s.ad_value(478), p.p107));

        s.store_ad_value(483, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p210, p.p209), 1.0, s.ad_value(477), p.p211, s.ad_value(478), p.p212));

        s.store_ad_value(488, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p220, p.p213), 1.0, s.ad_value(477), p.p221, s.ad_value(478), p.p222));

        s.store_ad_value(484, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p226, p.p223), 1.0, s.ad_value(477), p.p227, s.ad_value(478), p.p228));

        s.store_ad_value(487, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p236, p.p233), 1.0, s.ad_value(477), p.p237, s.ad_value(478), p.p238));

        s.store_ad_value(116, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p144, p.p143), 1.0, s.ad_value(477), p.p145, s.ad_value(478), p.p146));

        s.store_ad_value(117, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p148, p.p147), 1.0, s.ad_value(477), p.p149, s.ad_value(478), p.p150));

        s.store_ad_value(118, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p152, p.p151), 1.0, s.ad_value(477), p.p153, s.ad_value(478), p.p154));

        s.store_ad_value(119, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p156, p.p155), 1.0, s.ad_value(477), p.p157, s.ad_value(478), p.p158));

        s.store_ad_value(120, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p160, p.p159), 1.0, s.ad_value(477), p.p161, s.ad_value(478), p.p162));

        s.store_ad_value(121, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p164, p.p163), 1.0, s.ad_value(477), p.p165, s.ad_value(478), p.p166));

        s.store_ad_value(494, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p202, p.p195), 1.0, s.ad_value(477), p.p203, s.ad_value(478), p.p204));

        s.store_ad_value(495, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p192, p.p185), 1.0, s.ad_value(477), p.p193, s.ad_value(478), p.p194));

        s.store_ad_value(538, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p113, p.p112), 1.0, s.ad_value(477), p.p114, s.ad_value(478), p.p115));

        s.store_ad_value(489, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p168, p.p167), 1.0, s.ad_value(477), p.p169, s.ad_value(478), p.p170));

        s.store_ad_value(490, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p172, p.p171), 1.0, s.ad_value(477), p.p173, s.ad_value(478), p.p174));

        s.store_ad_value(493, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p182, p.p180), 1.0, s.ad_value(477), p.p183, s.ad_value(478), p.p184));

        s.store_ad_value(496, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p254, p.p253), 1.0, s.ad_value(477), p.p255, s.ad_value(478), p.p256));

        s.store_ad_value(497, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p276, p.p273), 1.0, s.ad_value(477), p.p277, s.ad_value(478), p.p278));

        s.store_ad_value(504, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p291, p.p284), 1.0, s.ad_value(477), p.p292, s.ad_value(478), p.p293));

        s.store_ad_value(508, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p311, p.p308), 1.0, s.ad_value(477), p.p312, s.ad_value(478), p.p313));

        s.store_ad_value(507, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p299, p.p298), 1.0, s.ad_value(477), p.p300, s.ad_value(478), p.p301));

        s.store_ad_value(511, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p319, p.p318), 1.0, s.ad_value(477), p.p320, s.ad_value(478), p.p321));

        s.store_ad_value(514, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p333, p.p326), 1.0, s.ad_value(477), p.p334, s.ad_value(478), p.p335));

        s.store_ad_value(539, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p343, p.p340), 1.0, s.ad_value(477), p.p344, s.ad_value(478), p.p345));

        s.store_ad_value(542, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p354, p.p351), 1.0, s.ad_value(477), p.p355, s.ad_value(478), p.p356));

        s.store_ad_value(531, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p394, p.p393), 1.0, s.ad_value(477), p.p395, s.ad_value(478), p.p396));

        s.store_ad_value(530, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p404, p.p403), 1.0, s.ad_value(477), p.p405, s.ad_value(478), p.p406));

        s.store_ad_value(526, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p376, p.p375), 1.0, s.ad_value(477), p.p377, s.ad_value(478), p.p378));

        s.store_ad_value(543, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p380, p.p379), 1.0, s.ad_value(477), p.p381, s.ad_value(478), p.p382));

        s.store_ad_value(527, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p386, p.p385), 1.0, s.ad_value(477), p.p387, s.ad_value(478), p.p388));

        s.store_ad_value(529, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p390, p.p389), 1.0, s.ad_value(477), p.p391, s.ad_value(478), p.p392));

        s.store_ad_value(528, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p400, p.p399), 1.0, s.ad_value(477), p.p401, s.ad_value(478), p.p402));

        s.store_ad_value(532, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p416, p.p413), 1.0, s.ad_value(477), p.p417, s.ad_value(478), p.p418));

        s.store_ad_value(533, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p410, p.p409), 1.0, s.ad_value(477), p.p411, s.ad_value(478), p.p412));

        s.store_ad_value(534, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p435, p.p434), 1.0, s.ad_value(477), p.p436, s.ad_value(478), p.p437));

        s.store_ad_value(517, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p463, p.p460), 1.0, s.ad_value(477), p.p464, s.ad_value(478), p.p465));

        s.store_ad_value(520, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p471, p.p470), 1.0, s.ad_value(477), p.p472, s.ad_value(478), p.p473));

        s.store_ad_value(521, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p358, p.p357), 1.0, s.ad_value(477), p.p359, s.ad_value(478), p.p360));

        s.store_ad_value(522, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p362, p.p361), 1.0, s.ad_value(477), p.p363, s.ad_value(478), p.p364));

        s.store_ad_value(523, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p366, p.p365), 1.0, s.ad_value(477), p.p367, s.ad_value(478), p.p368));

        s.store_ad_value(524, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p371, p.p370), 1.0, s.ad_value(477), p.p372, s.ad_value(478), p.p373));

        s.store_ad_value(525, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p481, p.p478), 1.0, s.ad_value(477), p.p482, s.ad_value(478), p.p483));

        s.store_ad_value(537, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p475, p.p474), 1.0, s.ad_value(477), p.p476, s.ad_value(478), p.p477));

        s.store_ad_value(500, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p240, p.p239), 1.0, s.ad_value(477), p.p241, s.ad_value(478), p.p242));

        s.store_ad_value(164, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p420, p.p419), 1.0, s.ad_value(477), p.p421, s.ad_value(478), p.p422));

        s.store_ad_value(503, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p260, p.p259), 1.0, s.ad_value(477), p.p261, s.ad_value(478), p.p262));

        s.store_ad_value(544, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p667, p.p666), 1.0, s.ad_value(477), p.p668, s.ad_value(478), p.p669));

        s.store_ad_value(545, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p675, p.p674), 1.0, s.ad_value(477), p.p676, s.ad_value(478), p.p677));

        s.store_ad_value(546, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p679, p.p678), 1.0, s.ad_value(477), p.p680, s.ad_value(478), p.p681));

        s.store_ad_value(547, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p683, p.p682), 1.0, s.ad_value(477), p.p684, s.ad_value(478), p.p685));

        s.store_ad_value(548, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p687, p.p686), 1.0, s.ad_value(477), p.p688, s.ad_value(478), p.p689));

        s.store_ad_value(551, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p489, p.p484), 1.0, s.ad_value(477), p.p490, s.ad_value(478), p.p491));

        s.store_ad_value(554, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p497, p.p494), 1.0, s.ad_value(477), p.p498, s.ad_value(478), p.p499));

        s.store_ad_value(578, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p936, p.p935), 1.0, s.ad_value(477), p.p937, s.ad_value(478), p.p938));

        s.store_ad_value(579, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p940, p.p939), 1.0, s.ad_value(477), p.p941, s.ad_value(478), p.p942));

        s.store_ad_value(580, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p944, p.p943), 1.0, s.ad_value(477), p.p945, s.ad_value(478), p.p946));

        s.store_ad_value(559, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p633, p.p630), 1.0, s.ad_value(477), p.p634, s.ad_value(478), p.p635));

        s.store_ad_value(560, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p637, p.p636), 1.0, s.ad_value(477), p.p638, s.ad_value(478), p.p639));

        s.store_ad_value(561, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p641, p.p640), 1.0, s.ad_value(477), p.p642, s.ad_value(478), p.p643));

        s.store_ad_value(562, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p645, p.p644), 1.0, s.ad_value(477), p.p646, s.ad_value(478), p.p647));

        s.store_ad_value(563, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p651, p.p648), 1.0, s.ad_value(477), p.p652, s.ad_value(478), p.p653));

        s.store_ad_value(564, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p655, p.p654), 1.0, s.ad_value(477), p.p656, s.ad_value(478), p.p657));

        s.store_ad_value(565, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p659, p.p658), 1.0, s.ad_value(477), p.p660, s.ad_value(478), p.p661));

        s.store_ad_value(566, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p663, p.p662), 1.0, s.ad_value(477), p.p664, s.ad_value(478), p.p665));

        s.store_ad_value(567, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p825, p.p824), 1.0, s.ad_value(477), p.p826, s.ad_value(478), p.p827));

        s.store_ad_value(568, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p830, p.p829), 1.0, s.ad_value(477), p.p831, s.ad_value(478), p.p832));

        s.store_ad_value(569, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p835, p.p834), 1.0, s.ad_value(477), p.p836, s.ad_value(478), p.p837));

        s.store_ad_value(570, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p839, p.p838), 1.0, s.ad_value(477), p.p840, s.ad_value(478), p.p841));

        s.store_ad_value(577, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p844, p.p843), 1.0, s.ad_value(477), p.p845, s.ad_value(478), p.p846));

        s.store_ad_value(571, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p848, p.p847), 1.0, s.ad_value(477), p.p849, s.ad_value(478), p.p850));

        s.store_ad_value(572, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p853, p.p852), 1.0, s.ad_value(477), p.p854, s.ad_value(478), p.p855));

        s.store_ad_value(573, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p857, p.p856), 1.0, s.ad_value(477), p.p858, s.ad_value(478), p.p859));

        s.store_ad_value(574, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p863, p.p862), 1.0, s.ad_value(477), p.p864, s.ad_value(478), p.p865));

        s.store_ad_value(575, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p878, p.p877), 1.0, s.ad_value(477), p.p879, s.ad_value(478), p.p880));

        s.store_ad_value(576, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p886, p.p885), 1.0, s.ad_value(477), p.p887, s.ad_value(478), p.p888));

        s.store_ad_value(604, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p882, p.p881), 1.0, s.ad_value(477), p.p883, s.ad_value(478), p.p884));

        s.store_ad_value(581, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p564, p.p537), 1.0, s.ad_value(477), p.p565, s.ad_value(478), p.p566));

        s.store_ad_value(582, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p567, p.p538), 1.0, s.ad_value(477), p.p568, s.ad_value(478), p.p569));

    }

    pub(super) fn stamp_transient_block_1(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_ad_value(583, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p570, p.p539), 1.0, s.ad_value(477), p.p571, s.ad_value(478), p.p572));

        s.store_ad_value(584, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p573, p.p540), 1.0, s.ad_value(477), p.p574, s.ad_value(478), p.p575));

        s.store_ad_value(585, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p576, p.p541), 1.0, s.ad_value(477), p.p577, s.ad_value(478), p.p578));

        s.store_ad_value(586, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p579, p.p533), 1.0, s.ad_value(477), p.p580, s.ad_value(478), p.p581));

        s.store_ad_value(587, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p582, p.p534), 1.0, s.ad_value(477), p.p583, s.ad_value(478), p.p584));

        s.store_ad_value(588, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p585, p.p535), 1.0, s.ad_value(477), p.p586, s.ad_value(478), p.p587));

        s.store_ad_value(589, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p588, p.p536), 1.0, s.ad_value(477), p.p589, s.ad_value(478), p.p590));

        s.store_ad_value(590, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p591, p.p542), 1.0, s.ad_value(477), p.p592, s.ad_value(478), p.p593));

        s.store_ad_value(591, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p594, p.p543), 1.0, s.ad_value(477), p.p595, s.ad_value(478), p.p596));

        s.store_ad_value(592, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p597, p.p544), 1.0, s.ad_value(477), p.p598, s.ad_value(478), p.p599));

        s.store_ad_value(593, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p600, p.p545), 1.0, s.ad_value(477), p.p601, s.ad_value(478), p.p602));

        s.store_ad_value(594, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p603, p.p546), 1.0, s.ad_value(477), p.p604, s.ad_value(478), p.p605));

        s.store_ad_value(595, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p606, p.p547), 1.0, s.ad_value(477), p.p607, s.ad_value(478), p.p608));

        s.store_ad_value(596, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p609, p.p548), 1.0, s.ad_value(477), p.p610, s.ad_value(478), p.p611));

        s.store_ad_value(597, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p612, p.p549), 1.0, s.ad_value(477), p.p613, s.ad_value(478), p.p614));

        s.store_ad_value(598, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p615, p.p550), 1.0, s.ad_value(477), p.p616, s.ad_value(478), p.p617));

        s.store_ad_value(599, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p618, p.p553), 1.0, s.ad_value(477), p.p619, s.ad_value(478), p.p620));

        s.store_ad_value(601, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p621, p.p551), 1.0, s.ad_value(477), p.p622, s.ad_value(478), p.p623));

        s.store_ad_value(602, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p624, p.p552), 1.0, s.ad_value(477), p.p625, s.ad_value(478), p.p626));

        s.store_ad_value(603, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p627, p.p554), 1.0, s.ad_value(477), p.p628, s.ad_value(478), p.p629));

        s.store_ad_value(454, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p870, p.p867), 1.0, s.ad_value(477), p.p871, s.ad_value(478), p.p872));

        s.store_ad_value(455, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p874, p.p873), 1.0, s.ad_value(477), p.p875, s.ad_value(478), p.p876));

        s.store_ad_value(453, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p430, p.p425), 1.0, s.ad_value(477), p.p431, s.ad_value(478), p.p432));

        s.store_ad_value(148, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p445, p.p444), 1.0, s.ad_value(477), p.p446, s.ad_value(478), p.p447));

        s.store_ad_value(149, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p449, p.p448), 1.0, s.ad_value(477), p.p450, s.ad_value(478), p.p451));

        s.store_ad_value(151, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p453, p.p452), 1.0, s.ad_value(477), p.p454, s.ad_value(478), p.p455));

        s.store_ad_value(152, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p457, p.p456), 1.0, s.ad_value(477), p.p458, s.ad_value(478), p.p459));

        s.store_ad_value(605, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p1047, p.p1046), 1.0, s.ad_value(477), p.p1048, s.ad_value(478), p.p1049));

        s.store_ad_value(606, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p1055, p.p1054), 1.0, s.ad_value(477), p.p1056, s.ad_value(478), p.p1057));

        s.store_ad_value(607, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p1051, p.p1050), 1.0, s.ad_value(477), p.p1052, s.ad_value(478), p.p1053));

        s.store_ad_value(608, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p1059, p.p1058), 1.0, s.ad_value(477), p.p1060, s.ad_value(478), p.p1061));

        s.store_ad_value(612, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p967, p.p966), 1.0, s.ad_value(477), p.p968, s.ad_value(478), p.p969));

        s.store_ad_value(686, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p963, p.p962), 1.0, s.ad_value(477), p.p964, s.ad_value(478), p.p965));

        s.store_ad_value(613, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p971, p.p970), 1.0, s.ad_value(477), p.p972, s.ad_value(478), p.p973));

        s.store_ad_value(614, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p975, p.p974), 1.0, s.ad_value(477), p.p976, s.ad_value(478), p.p977));

        s.store_ad_value(615, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p979, p.p978), 1.0, s.ad_value(477), p.p980, s.ad_value(478), p.p981));

        s.store_ad_value(616, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p983, p.p982), 1.0, s.ad_value(477), p.p984, s.ad_value(478), p.p985));

        s.store_ad_value(617, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p987, p.p986), 1.0, s.ad_value(477), p.p988, s.ad_value(478), p.p989));

        s.store_ad_value(618, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p991, p.p990), 1.0, s.ad_value(477), p.p992, s.ad_value(478), p.p993));

        s.store_ad_value(619, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p995, p.p994), 1.0, s.ad_value(477), p.p996, s.ad_value(478), p.p997));

        s.store_ad_value(620, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p999, p.p998), 1.0, s.ad_value(477), p.p1000, s.ad_value(478), p.p1001));

        s.store_ad_value(621, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p1003, p.p1002), 1.0, s.ad_value(477), p.p1004, s.ad_value(478), p.p1005));

        s.store_ad_value(622, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p1007, p.p1006), 1.0, s.ad_value(477), p.p1008, s.ad_value(478), p.p1009));

        s.store_ad_value(623, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p1011, p.p1010), 1.0, s.ad_value(477), p.p1012, s.ad_value(478), p.p1013));

        s.store_ad_value(624, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p1018, p.p1017), 1.0, s.ad_value(477), p.p1019, s.ad_value(478), p.p1020));

        s.store_ad_value(625, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p1022, p.p1021), 1.0, s.ad_value(477), p.p1023, s.ad_value(478), p.p1024));

        s.store_ad_value(629, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p1030, p.p1029), 1.0, s.ad_value(477), p.p1031, s.ad_value(478), p.p1032));

        s.store_ad_value(630, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p1026, p.p1025), 1.0, s.ad_value(477), p.p1027, s.ad_value(478), p.p1028));

        s.store_ad_value(626, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p1034, p.p1033), 1.0, s.ad_value(477), p.p1035, s.ad_value(478), p.p1036));

        s.store_ad_value(627, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p1038, p.p1037), 1.0, s.ad_value(477), p.p1039, s.ad_value(478), p.p1040));

        s.store_ad_value(631, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p1070, p.p1069), 1.0, s.ad_value(477), p.p1071, s.ad_value(478), p.p1072));

        s.store_ad_value(632, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p1074, p.p1073), 1.0, s.ad_value(477), p.p1075, s.ad_value(478), p.p1076));

        s.store_ad_value(634, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p1078, p.p1077), 1.0, s.ad_value(477), p.p1079, s.ad_value(478), p.p1080));

        s.store_ad_value(635, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p1082, p.p1081), 1.0, s.ad_value(477), p.p1083, s.ad_value(478), p.p1084));

        s.store_ad_value(637, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p1086, p.p1085), 1.0, s.ad_value(477), p.p1087, s.ad_value(478), p.p1088));

        s.store_ad_value(638, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p1090, p.p1089), 1.0, s.ad_value(477), p.p1091, s.ad_value(478), p.p1092));

        s.store_ad_value(640, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p787, p.p786), 1.0, s.ad_value(477), p.p788, s.ad_value(478), p.p789));

        s.store_ad_value(641, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p795, p.p794), 1.0, s.ad_value(477), p.p796, s.ad_value(478), p.p797));

        s.store_ad_value(642, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p791, p.p790), 1.0, s.ad_value(477), p.p792, s.ad_value(478), p.p793));

        s.b[879] = (p.p44 != 0.0);
        s.v[879] = if s.b[879] { 1.0 } else { 0.0 };

        if s.b[879] {
            s.store_ad_value(485, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p230, p.p229), 1.0, s.ad_value(477), p.p231, s.ad_value(478), p.p232));
            s.store_ad_value(491, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p176, p.p175), 1.0, s.ad_value(477), p.p177, s.ad_value(478), p.p178));
            s.store_ad_value(498, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p280, p.p279), 1.0, s.ad_value(477), p.p281, s.ad_value(478), p.p282));
            s.store_ad_value(505, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p295, p.p294), 1.0, s.ad_value(477), p.p296, s.ad_value(478), p.p297));
            s.store_ad_value(509, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p315, p.p314), 1.0, s.ad_value(477), p.p316, s.ad_value(478), p.p317));
            s.store_ad_value(512, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p323, p.p322), 1.0, s.ad_value(477), p.p324, s.ad_value(478), p.p325));
            s.store_ad_value(515, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p337, p.p336), 1.0, s.ad_value(477), p.p338, s.ad_value(478), p.p339));
            s.store_ad_value(540, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p347, p.p346), 1.0, s.ad_value(477), p.p348, s.ad_value(478), p.p349));
            s.store_ad_value(518, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p467, p.p466), 1.0, s.ad_value(477), p.p468, s.ad_value(478), p.p469));
            s.store_ad_value(501, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p250, p.p249), 1.0, s.ad_value(477), p.p251, s.ad_value(478), p.p252));
            s.store_ad_value(165, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p427, p.p426), 1.0, s.ad_value(477), p.p428, s.ad_value(478), p.p429));
            s.store_ad_value(535, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p441, p.p440), 1.0, s.ad_value(477), p.p442, s.ad_value(478), p.p443));
            s.store_ad_value(552, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p526, p.p525), 1.0, s.ad_value(477), p.p527, s.ad_value(478), p.p528));
            s.store_ad_value(557, A::add_scaled_inputs3(A::scale_offset(s.ad_value(476), p.p530, p.p529), 1.0, s.ad_value(477), p.p531, s.ad_value(478), p.p532));
        }

        s.v[12] = ((p.p81 * ((((s.v[469]) as f64).powf(p.p82) - ((s.v[474]) as f64).powf(p.p82))).max(0.0)) + (p.p83 * ((((s.v[469]) as f64).powf(p.p84) - ((s.v[474]) as f64).powf(p.p84))).max(0.0)));

        s.v[13] = ((p.p85 * ((((s.v[470]) as f64).powf(p.p86) - ((s.v[475]) as f64).powf(p.p86))).max(0.0)) + (p.p87 * (((s.v[470] * s.v[469])) as f64).powf(p.p88)));

        s.store_scale(481, 481, ((1.0 + s.v[12]) + s.v[13]));

        s.v[12] = (p.p214 * ((((s.v[469]) as f64).powf(p.p215) - ((s.v[474]) as f64).powf(p.p215))).max(0.0));

        s.v[13] = ((p.p216 * ((((s.v[470]) as f64).powf(p.p217) - ((s.v[475]) as f64).powf(p.p217))).max(0.0)) + (p.p218 * ((s.v[471]) as f64).powf(p.p219)));

        s.store_scale(488, 488, ((1.0 + s.v[12]) + s.v[13]));

        s.v[12] = (1.0 + (p.p224 * ((((s.v[469]) as f64).powf(p.p225) - ((s.v[474]) as f64).powf(p.p225))).max(0.0)));

        s.store_scale(484, 484, s.v[12]);

        s.b[880] = (p.p44 != 0.0);
        s.v[880] = if s.b[880] { 1.0 } else { 0.0 };

        if s.b[880] {
            s.store_scale(485, 485, s.v[12]);
        }

        s.store_scale(487, 487, (1.0 + (p.p234 * ((((s.v[469]) as f64).powf(p.p235) - ((s.v[474]) as f64).powf(p.p235))).max(0.0))));

        s.store_scale(497, 497, p.p34);

        s.b[881] = (p.p50 != 1.0);
        s.v[881] = if s.b[881] { 1.0 } else { 0.0 };

        s.b[882] = (p.p275 > 0.0);
        s.v[882] = if s.b[882] { 1.0 } else { 0.0 };

        if (s.b[881] && s.b[882]) {
            s.store_scale(497, 497, (1.0 - (p.p274 * ((((s.v[469]) as f64).powf(p.p275) - ((s.v[474]) as f64).powf(p.p275))).max(0.0))));
        }

        s.b[883] = (p.p44 != 0.0);
        s.v[883] = if s.b[883] { 1.0 } else { 0.0 };

        if ((s.b[881] && s.b[882]) && s.b[883]) {
            s.store_scale(498, 498, (1.0 - (p.p274 * ((((s.v[469]) as f64).powf(p.p275) - ((s.v[474]) as f64).powf(p.p275))).max(0.0))));
        }

        if (s.b[881] && (!s.b[882])) {
            s.store_scale(497, 497, (1.0 - p.p274));
        }

        s.b[884] = (p.p44 != 0.0);
        s.v[884] = if s.b[884] { 1.0 } else { 0.0 };

        if ((s.b[881] && (!s.b[882])) && s.b[884]) {
            s.store_scale(498, 498, (1.0 - p.p274));
        }

        if (!s.b[881]) {
            let assign3470_ad_e4787: A = A::scale(s.ad_value(497), ((1.0 - (p.p269 * { let limited_exp_arg = ((-s.v[30]) / p.p270); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })) - (p.p271 * { let limited_exp_arg = ((-s.v[30]) / p.p272); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })));
            s.store_ad_value(497, assign3470_ad_e4787);
        }

        s.b[885] = (p.p44 != 0.0);
        s.v[885] = if s.b[885] { 1.0 } else { 0.0 };

        if ((!s.b[881]) && s.b[885]) {
            let assign3490_ad_e4815: A = A::scale(s.ad_value(498), ((1.0 - (p.p269 * { let limited_exp_arg = ((-s.v[30]) / p.p270); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })) - (p.p271 * { let limited_exp_arg = ((-s.v[30]) / p.p272); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })));
            s.store_ad_value(498, assign3490_ad_e4815);
        }

        s.v[12] = (p.p285 * ((((s.v[469]) as f64).powf(p.p286) - ((s.v[474]) as f64).powf(p.p286))).max(0.0));

        s.v[13] = ((p.p287 * ((((s.v[470]) as f64).powf(p.p288) - ((s.v[475]) as f64).powf(p.p288))).max(0.0)) + (p.p289 * ((s.v[471]) as f64).powf(p.p290)));

        s.store_scale(504, 504, ((1.0 + s.v[12]) + s.v[13]));

        s.b[886] = (p.p44 != 0.0);
        s.v[886] = if s.b[886] { 1.0 } else { 0.0 };

        if s.b[886] {
            s.store_scale(505, 505, ((1.0 + s.v[12]) + s.v[13]));
        }

        s.v[12] = (p.p302 * ((((s.v[469]) as f64).powf(p.p303) - ((s.v[474]) as f64).powf(p.p303))).max(0.0));

        s.v[13] = ((p.p304 * ((((s.v[470]) as f64).powf(p.p305) - ((s.v[475]) as f64).powf(p.p305))).max(0.0)) + (p.p306 * ((s.v[471]) as f64).powf(p.p307)));

        s.store_scale(507, 507, ((1.0 + s.v[12]) + s.v[13]));

        s.v[12] = (1.0 + (p.p309 * ((((s.v[469]) as f64).powf(p.p310) - ((s.v[474]) as f64).powf(p.p310))).max(0.0)));

        s.store_scale(508, 508, s.v[12]);

        s.b[887] = (p.p44 != 0.0);
        s.v[887] = if s.b[887] { 1.0 } else { 0.0 };

        if s.b[887] {
            s.store_scale(509, 509, s.v[12]);
        }

        s.v[12] = (p.p327 * ((((s.v[469]) as f64).powf(p.p328) - ((s.v[474]) as f64).powf(p.p328))).max(0.0));

        s.v[13] = ((p.p329 * ((((s.v[470]) as f64).powf(p.p330) - ((s.v[475]) as f64).powf(p.p330))).max(0.0)) + (p.p331 * ((s.v[471]) as f64).powf(p.p332)));

        s.store_scale(514, 514, ((1.0 + s.v[12]) + s.v[13]));

        s.b[888] = (p.p44 != 0.0);
        s.v[888] = if s.b[888] { 1.0 } else { 0.0 };

        if s.b[888] {
            s.store_scale(515, 515, ((1.0 + s.v[12]) + s.v[13]));
        }

        s.v[12] = ((((s.v[469]) as f64).powf(p.p179) - ((s.v[474]) as f64).powf(p.p179))).max(0.0);

        s.store_scale(490, 490, s.v[12]);

        s.b[889] = (p.p44 != 0.0);
        s.v[889] = if s.b[889] { 1.0 } else { 0.0 };

        if s.b[889] {
            s.store_scale(491, 491, s.v[12]);
        }

        s.store_scale(493, 493, ((((s.v[469]) as f64).powf(p.p181) - ((s.v[474]) as f64).powf(p.p181))).max(0.0));

        s.v[12] = (1.0 + (p.p461 * ((((s.v[469]) as f64).powf(p.p462) - ((s.v[474]) as f64).powf(p.p462))).max(0.0)));

        s.store_scale(517, 517, s.v[12]);

        s.b[890] = (p.p44 != 0.0);
        s.v[890] = if s.b[890] { 1.0 } else { 0.0 };

        if s.b[890] {
            s.store_scale(518, 518, s.v[12]);
        }

        s.store_scale(12, 496, (1.0 + (p.p257 * ((((s.v[469]) as f64).powf(p.p258) - ((s.v[474]) as f64).powf(p.p258))).max(0.0))));

        s.store_min_with_scalar(496, 12, 0.5);

        s.store_scale(525, 525, (1.0 + (p.p479 * ((((s.v[469]) as f64).powf(p.p480) - ((s.v[474]) as f64).powf(p.p480))).max(0.0))));

        s.v[12] = (1.0 + (p.p341 * ((((s.v[469]) as f64).powf(p.p342) - ((s.v[474]) as f64).powf(p.p342))).max(0.0)));

        s.store_scale(539, 539, s.v[12]);

        s.store_max_with_scalar(539, 539, 0.0);

        s.b[891] = (p.p44 != 0.0);
        s.v[891] = if s.b[891] { 1.0 } else { 0.0 };

        if s.b[891] {
            s.store_scale(540, 540, s.v[12]);
            s.store_max_with_scalar(540, 540, 0.0);
        }

        s.v[12] = (p.p243 * ((((s.v[469]) as f64).powf(p.p244) - ((s.v[474]) as f64).powf(p.p244))).max(0.0));

        s.v[13] = ((p.p245 * ((((s.v[470]) as f64).powf(p.p246) - ((s.v[475]) as f64).powf(p.p246))).max(0.0)) + (p.p247 * ((s.v[471]) as f64).powf(p.p248)));

        s.store_scale(500, 500, ((1.0 + s.v[12]) + s.v[13]));

        s.b[892] = (p.p44 != 0.0);
        s.v[892] = if s.b[892] { 1.0 } else { 0.0 };

        if s.b[892] {
            s.store_scale(501, 501, ((1.0 + s.v[12]) + s.v[13]));
        }

        s.store_max_with_scalar_ad(164, A::scale(s.ad_value(164), (1.0 + (p.p423 * ((((s.v[469]) as f64).powf(p.p424) - ((s.v[474]) as f64).powf(p.p424))).max(0.0)))), 0.25);

        s.b[893] = (p.p44 != 0.0);
        s.v[893] = if s.b[893] { 1.0 } else { 0.0 };

        if s.b[893] {
            s.store_max_with_scalar_ad(165, A::scale(s.ad_value(165), (1.0 + (p.p423 * ((((s.v[469]) as f64).powf(p.p424) - ((s.v[474]) as f64).powf(p.p424))).max(0.0)))), 0.25);
        }

        s.v[12] = (1.0 + (p.p438 * ((((s.v[469]) as f64).powf(p.p439) - ((s.v[474]) as f64).powf(p.p439))).max(0.0)));

        s.store_scale(534, 534, s.v[12]);

        s.b[894] = (p.p44 != 0.0);
        s.v[894] = if s.b[894] { 1.0 } else { 0.0 };

        if s.b[894] {
            s.store_scale(535, 535, s.v[12]);
        }

        s.v[12] = (p.p485 * ((((s.v[469]) as f64).powf(p.p486) - ((s.v[474]) as f64).powf(p.p486))).max(0.0));

        s.v[13] = (p.p487 * ((((s.v[470]) as f64).powf(p.p488) - ((s.v[475]) as f64).powf(p.p488))).max(0.0));

        s.store_scale(551, 551, ((1.0 + s.v[12]) + s.v[13]));

        s.b[895] = (p.p44 != 0.0);
        s.v[895] = if s.b[895] { 1.0 } else { 0.0 };

        if s.b[895] {
            s.store_scale(552, 552, ((1.0 + s.v[12]) + s.v[13]));
        }

        s.v[13] = (p.p495 * ((((s.v[470]) as f64).powf(p.p496) - ((s.v[475]) as f64).powf(p.p496))).max(0.0));

        s.store_scale(554, 554, (1.0 + s.v[13]));

        s.v[13] = (p.p519 * ((((s.v[470]) as f64).powf(p.p520) - ((s.v[475]) as f64).powf(p.p520))).max(0.0));

        s.v[555] = p.p518;

        s.v[555] = (s.v[555] * (1.0 + s.v[13]));

        s.v[13] = (p.p522 * ((((s.v[470]) as f64).powf(p.p523) - ((s.v[475]) as f64).powf(p.p523))).max(0.0));

        s.v[556] = p.p521;

        s.v[556] = (s.v[556] * (1.0 + s.v[13]));

        s.store_scale(559, 559, ((1.0 + (p.p631 * s.v[469])) + (p.p632 * s.v[470])));

        s.store_scale(563, 563, ((1.0 + (p.p649 * s.v[469])) + (p.p650 * s.v[470])));

        s.store_scale(590, 590, ((1.0 + (p.p557 * s.v[469])) + (p.p558 * s.v[470])));

        s.store_scale(593, 593, ((1.0 + (p.p559 * s.v[469])) + (p.p560 * s.v[470])));

        s.store_scale(596, 596, ((1.0 + (p.p561 * s.v[469])) + (p.p562 * s.v[470])));

        s.v[600] = (p.p556 * (1.0 + (p.p563 * s.v[469])));

        s.v[12] = ((p.p93 * ((((s.v[472]) as f64).powf(p.p94) - ((s.v[474]) as f64).powf(p.p94))).max(0.0)) + (p.p95 * ((((s.v[472]) as f64).powf(p.p96) - ((s.v[474]) as f64).powf(p.p96))).max(0.0)));

        s.v[13] = ((p.p97 * ((((s.v[473]) as f64).powf(p.p98) - ((s.v[475]) as f64).powf(p.p98))).max(0.0)) + (p.p99 * (((s.v[473] * s.v[472])) as f64).powf(p.p100)));

        s.store_scale(550, 550, ((1.0 + s.v[12]) + s.v[13]));

        s.v[12] = (p.p120 * ((((s.v[472]) as f64).powf(p.p121) - ((s.v[474]) as f64).powf(p.p121))).max(0.0));

        s.v[13] = ((p.p122 * ((((s.v[473]) as f64).powf(p.p123) - ((s.v[475]) as f64).powf(p.p123))).max(0.0)) + (p.p124 * ((s.v[471]) as f64).powf(p.p125)));

        s.store_scale(482, 482, ((1.0 + s.v[12]) + s.v[13]));

        s.v[12] = (p.p130 * ((((s.v[472]) as f64).powf(p.p131) - ((s.v[474]) as f64).powf(p.p131))).max(0.0));

        s.v[13] = ((p.p132 * ((((s.v[473]) as f64).powf(p.p133) - ((s.v[475]) as f64).powf(p.p133))).max(0.0)) + (p.p134 * ((s.v[471]) as f64).powf(p.p135)));

        s.store_scale(549, 549, ((1.0 + s.v[12]) + s.v[13]));

        s.v[12] = (p.p263 * ((((s.v[472]) as f64).powf(p.p264) - ((s.v[474]) as f64).powf(p.p264))).max(0.0));

        s.v[13] = ((p.p265 * ((((s.v[470]) as f64).powf(p.p266) - ((s.v[475]) as f64).powf(p.p266))).max(0.0)) + (p.p267 * ((s.v[471]) as f64).powf(p.p268)));

        s.store_scale(503, 503, ((1.0 + s.v[12]) + s.v[13]));

        s.store_scale(542, 542, (1.0 + (p.p352 * ((((s.v[472]) as f64).powf(p.p353) - ((s.v[474]) as f64).powf(p.p353))).max(0.0))));

        s.store_max_with_scalar(542, 542, 0.0);

        s.v[12] = (p.p186 * ((((s.v[469]) as f64).powf(p.p187) - ((s.v[474]) as f64).powf(p.p187))).max(0.0));

        s.v[13] = ((p.p188 * ((((s.v[470]) as f64).powf(p.p189) - ((s.v[475]) as f64).powf(p.p189))).max(0.0)) + (p.p190 * ((s.v[471]) as f64).powf(p.p191)));

        s.store_scale(495, 495, ((1.0 + s.v[12]) + s.v[13]));

        s.v[12] = (p.p196 * ((((s.v[469]) as f64).powf(p.p197) - ((s.v[474]) as f64).powf(p.p197))).max(0.0));

        s.v[13] = ((p.p198 * ((((s.v[470]) as f64).powf(p.p199) - ((s.v[475]) as f64).powf(p.p199))).max(0.0)) + (p.p200 * ((s.v[471]) as f64).powf(p.p201)));

        s.store_scale(494, 494, ((1.0 + s.v[12]) + s.v[13]));

        s.store_scale(543, 543, (1.0 + (p.p383 * ((((s.v[469]) as f64).powf(p.p384) - ((s.v[474]) as f64).powf(p.p384))).max(0.0))));

        s.store_scale(567, 567, (1.0 + (s.v[469] * p.p828)));

        s.store_scale(568, 568, (1.0 + (s.v[469] * p.p833)));

        s.store_scale(570, 570, (1.0 + (s.v[469] * p.p842)));

        s.store_scale(573, 573, (1.0 + (s.v[469] * p.p860)));

        s.store_scale(574, 574, (1.0 + (s.v[469] * p.p866)));

        s.b[896] = (!true);
        s.v[896] = if s.b[896] { 1.0 } else { 0.0 };

        s.b[897] = ((p.p49 == 0.0) || (p.p909 == 0.0));
        s.v[897] = if s.b[897] { 1.0 } else { 0.0 };

        s.b[898] = (p.p42 == 1.0);
        s.v[898] = if s.b[898] { 1.0 } else { 0.0 };

        if s.b[898] {
            s.store_scale(531, 531, (1.0 + (p.p397 * ((((s.v[469]) as f64).powf(p.p398) - ((s.v[474]) as f64).powf(p.p398))).max(0.0))));
            s.store_scale(530, 530, (1.0 + (p.p407 * ((((s.v[469]) as f64).powf(p.p408) - ((s.v[474]) as f64).powf(p.p408))).max(0.0))));
        }

        if (!s.b[898]) {
            s.store_scale(532, 532, (1.0 + (p.p414 * ((((s.v[469]) as f64).powf(p.p415) - ((s.v[474]) as f64).powf(p.p415))).max(0.0))));
        }

        s.b[899] = (s.v[511] < 1.0);
        s.v[899] = if s.b[899] { 1.0 } else { 0.0 };

        if s.b[899] {
            s.store_scalar(511, 1.0);
        }

        s.b[900] = (s.v[511] > 2.0);
        s.v[900] = if s.b[900] { 1.0 } else { 0.0 };

        if ((!s.b[899]) && s.b[900]) {
            s.store_scalar(511, 2.0);
        }

        s.b[901] = (p.p44 != 0.0);
        s.v[901] = if s.b[901] { 1.0 } else { 0.0 };

        s.b[902] = (s.v[512] < 1.0);
        s.v[902] = if s.b[902] { 1.0 } else { 0.0 };

        if (s.b[901] && s.b[902]) {
            s.store_scalar(512, 1.0);
        }

        s.b[903] = (s.v[512] > 2.0);
        s.v[903] = if s.b[903] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((s.b[901] && (!s.b[902])) && s.b[903]) {
            s.store_scalar(512, 2.0);
        }

        s.b[923] = (s.v[601] < 0.0);
        s.v[923] = if s.b[923] { 1.0 } else { 0.0 };

        if s.b[923] {
            s.store_scalar(601, 0.0);
        }

        s.b[924] = (s.v[602] < 0.0);
        s.v[924] = if s.b[924] { 1.0 } else { 0.0 };

        if s.b[924] {
            s.store_scalar(602, 0.0);
        }

        s.b[925] = (s.v[606] < 0.0);
        s.v[925] = if s.b[925] { 1.0 } else { 0.0 };

        if s.b[925] {
            s.store_scalar(606, 0.0);
        }

        s.b[926] = (s.v[497] <= 0.0);
        s.v[926] = if s.b[926] { 1.0 } else { 0.0 };

        if s.b[926] {
            s.store_scalar(497, 0.067);
        }

        s.b[927] = (s.v[504] < 0.0);
        s.v[927] = if s.b[927] { 1.0 } else { 0.0 };

        if s.b[927] {
            s.store_scalar(504, 0.0);
        }

        s.b[928] = (s.v[507] < 0.0);
        s.v[928] = if s.b[928] { 1.0 } else { 0.0 };

        if s.b[928] {
            s.store_scalar(507, 0.0);
        }

        s.b[929] = (s.v[508] < 0.0);
        s.v[929] = if s.b[929] { 1.0 } else { 0.0 };

        if s.b[929] {
            s.store_scalar(508, 0.0);
        }

        s.b[930] = (s.v[511] < 0.0);
        s.v[930] = if s.b[930] { 1.0 } else { 0.0 };

        if s.b[930] {
            s.store_scalar(511, 0.0);
        }

        s.b[931] = (s.v[555] < 0.0);
        s.v[931] = if s.b[931] { 1.0 } else { 0.0 };

        if s.b[931] {
            s.store_scalar(555, 0.0);
        }

        s.b[932] = (p.p1065 == 1.0);
        s.v[932] = if s.b[932] { 1.0 } else { 0.0 };

        if s.b[932] {
            s.store_scalar(746, p.p1066);
        }

        s.b[933] = (s.v[30] > s.v[746]);
        s.v[933] = if s.b[933] { 1.0 } else { 0.0 };

        if (s.b[932] && s.b[933]) {
            s.store_sub_from_scalar(12, s.v[30], 746);
        }

        if (s.b[932] && (!s.b[933])) {
            s.store_scalar(746, s.v[30]);
            s.copy_ad(12, 746);
        }

        s.b[934] = (p.p801 >= (s.v[12] / 2.0));
        s.v[934] = if s.b[934] { 1.0 } else { 0.0 };

        if (s.b[932] && s.b[934]) {
            s.store_scalar(359, 0.0);
        }

        if (s.b[932] && (!s.b[934])) {
            s.store_scalar(359, p.p801);
        }

        s.v[701] = 0.0;

        s.v[703] = 0.0;

        s.v[700] = 0.0;

        s.v[702] = 0.0;

        s.v[705] = 0.0;

        s.v[704] = 0.0;

        s.v[236] = (p.p695 - p.p698);

        s.v[238] = p.p696;

        s.v[237] = (p.p697 - p.p698);

        s.b[935] = param_given[3];
        s.v[935] = if s.b[935] { 1.0 } else { 0.0 };

        if s.b[935] {
            s.store_scalar(239, (p.p374 * p.p3));
        }

        s.b[936] = ((p.p10 > 0.0) && (p.p374 > 0.0));
        s.v[936] = if s.b[936] { 1.0 } else { 0.0 };

        s.b[937] = (p.p9 < 9.0);
        s.v[937] = if s.b[937] { 1.0 } else { 0.0 };

        s.b[938] = ((p.p2 % 2.0) != 0.0);
        s.v[938] = if s.b[938] { 1.0 } else { 0.0 };

        if ((((!s.b[935]) && s.b[936]) && s.b[937]) && s.b[938]) {
            s.store_scalar(701, 1.0);
            s.store_scalar(703, 1.0);
            s.store_scalar(700, (2.0 * (((p.p2 - 1.0) / 2.0)).max(0.0)));
            s.copy_ad(702, 700);
        }

        s.b[939] = (p.p6 == 1.0);
        s.v[939] = if s.b[939] { 1.0 } else { 0.0 };

        if (((((!s.b[935]) && s.b[936]) && s.b[937]) && (!s.b[938])) && s.b[939]) {
            s.store_scalar(701, 2.0);
            s.store_scalar(700, (2.0 * (((p.p2 / 2.0) - 1.0)).max(0.0)));
            s.store_scalar(703, 0.0);
            s.store_scalar(702, p.p2);
        }

        if (((((!s.b[935]) && s.b[936]) && s.b[937]) && (!s.b[938])) && (!s.b[939])) {
            s.store_scalar(701, 0.0);
            s.store_scalar(700, p.p2);
            s.store_scalar(703, 2.0);
            s.store_scalar(702, (2.0 * (((p.p2 / 2.0) - 1.0)).max(0.0)));
        }

        s.b[940] = (1.0 == 1.0);
        s.v[940] = if s.b[940] { 1.0 } else { 0.0 };

        s.b[941] = (s.v[702] == 0.0);
        s.v[941] = if s.b[941] { 1.0 } else { 0.0 };

        if (((((!s.b[935]) && s.b[936]) && s.b[937]) && s.b[940]) && s.b[941]) {
            s.store_scalar(704, 0.0);
        }

        if (((((!s.b[935]) && s.b[936]) && s.b[937]) && s.b[940]) && (!s.b[941])) {
            s.store_div_from_scalar_scaled_input(704, (p.p374 * s.v[236]), 702, s.v[29]);
        }

        s.b[942] = (s.v[700] == 0.0);
        s.v[942] = if s.b[942] { 1.0 } else { 0.0 };

        if (((((!s.b[935]) && s.b[936]) && s.b[937]) && (!s.b[940])) && s.b[942]) {
            s.store_scalar(704, 0.0);
        }

        if (((((!s.b[935]) && s.b[936]) && s.b[937]) && (!s.b[940])) && (!s.b[942])) {
            s.store_div_from_scalar_scaled_input(704, (p.p374 * s.v[236]), 700, s.v[29]);
        }

        s.b[943] = (p.p9 == 0.0);
        s.v[943] = if s.b[943] { 1.0 } else { 0.0 };

        s.b[944] = (p.p9 == 1.0);
        s.v[944] = if s.b[944] { 1.0 } else { 0.0 };

        s.b[945] = (p.p9 == 2.0);
        s.v[945] = if s.b[945] { 1.0 } else { 0.0 };

        s.b[946] = (p.p9 == 3.0);
        s.v[946] = if s.b[946] { 1.0 } else { 0.0 };

        s.b[947] = (p.p9 == 4.0);
        s.v[947] = if s.b[947] { 1.0 } else { 0.0 };

        s.b[948] = (p.p9 == 5.0);
        s.v[948] = if s.b[948] { 1.0 } else { 0.0 };

        s.b[949] = (p.p9 == 6.0);
        s.v[949] = if s.b[949] { 1.0 } else { 0.0 };

        s.b[950] = (p.p9 == 7.0);
        s.v[950] = if s.b[950] { 1.0 } else { 0.0 };

        s.b[951] = (p.p9 == 8.0);
        s.v[951] = if s.b[951] { 1.0 } else { 0.0 };

        s.b[952] = (p.p9 == 9.0);
        s.v[952] = if s.b[952] { 1.0 } else { 0.0 };

        s.b[953] = (p.p9 == 10.0);
        s.v[953] = if s.b[953] { 1.0 } else { 0.0 };

        s.b[954] = (1.0 == 1.0);
        s.v[954] = if s.b[954] { 1.0 } else { 0.0 };

        s.b[955] = (1.0 == 1.0);
        s.v[955] = if s.b[955] { 1.0 } else { 0.0 };

        s.b[956] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[956] = if s.b[956] { 1.0 } else { 0.0 };

        s.b[957] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[957] = if s.b[957] { 1.0 } else { 0.0 };

        s.b[958] = (s.v[703] == 0.0);
        s.v[958] = if s.b[958] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && s.b[955]) && s.b[956]) && s.b[958]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && s.b[955]) && s.b[956]) && (!s.b[958])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[960] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[960] = if s.b[960] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && s.b[955]) && (s.b[957] && (!s.b[956]))) && s.b[960]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && s.b[955]) && (s.b[957] && (!s.b[956]))) && (!s.b[960])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && s.b[955]) && (!(s.b[956] || s.b[957]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[961] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[961] = if s.b[961] { 1.0 } else { 0.0 };

        s.b[962] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[962] = if s.b[962] { 1.0 } else { 0.0 };

        s.b[963] = (s.v[703] == 0.0);
        s.v[963] = if s.b[963] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && (!s.b[955])) && s.b[961]) && s.b[963]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && (!s.b[955])) && s.b[961]) && (!s.b[963])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[965] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[965] = if s.b[965] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && (!s.b[955])) && (s.b[962] && (!s.b[961]))) && s.b[965]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && (!s.b[955])) && (s.b[962] && (!s.b[961]))) && (!s.b[965])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && (!s.b[955])) && (!(s.b[961] || s.b[962]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[966] = (0.0 == 1.0);
        s.v[966] = if s.b[966] { 1.0 } else { 0.0 };

        s.b[967] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[967] = if s.b[967] { 1.0 } else { 0.0 };

        s.b[968] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[968] = if s.b[968] { 1.0 } else { 0.0 };

        s.b[969] = (s.v[701] == 0.0);
        s.v[969] = if s.b[969] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && s.b[966]) && s.b[967]) && s.b[969]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && s.b[966]) && s.b[967]) && (!s.b[969])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[971] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[971] = if s.b[971] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && s.b[966]) && (s.b[968] && (!s.b[967]))) && s.b[971]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && s.b[966]) && (s.b[968] && (!s.b[967]))) && (!s.b[971])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && s.b[966]) && (!(s.b[967] || s.b[968]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[972] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[972] = if s.b[972] { 1.0 } else { 0.0 };

        s.b[973] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[973] = if s.b[973] { 1.0 } else { 0.0 };

        s.b[974] = (s.v[701] == 0.0);
        s.v[974] = if s.b[974] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && (!s.b[966])) && s.b[972]) && s.b[974]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && (!s.b[966])) && s.b[972]) && (!s.b[974])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[976] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[976] = if s.b[976] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && (!s.b[966])) && (s.b[973] && (!s.b[972]))) && s.b[976]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && (!s.b[966])) && (s.b[973] && (!s.b[972]))) && (!s.b[976])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && (!s.b[966])) && (!(s.b[972] || s.b[973]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[977] = (1.0 == 1.0);
        s.v[977] = if s.b[977] { 1.0 } else { 0.0 };

        s.b[978] = (1.0 == 1.0);
        s.v[978] = if s.b[978] { 1.0 } else { 0.0 };

        s.b[979] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[979] = if s.b[979] { 1.0 } else { 0.0 };

        s.b[980] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[980] = if s.b[980] { 1.0 } else { 0.0 };

        s.b[981] = (s.v[703] == 0.0);
        s.v[981] = if s.b[981] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && s.b[978]) && s.b[979]) && s.b[981]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && s.b[978]) && s.b[979]) && (!s.b[981])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[983] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[983] = if s.b[983] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && s.b[978]) && (s.b[980] && (!s.b[979]))) && s.b[983]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && s.b[978]) && (s.b[980] && (!s.b[979]))) && (!s.b[983])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && s.b[978]) && (!(s.b[979] || s.b[980]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[984] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[984] = if s.b[984] { 1.0 } else { 0.0 };

        s.b[985] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[985] = if s.b[985] { 1.0 } else { 0.0 };

        s.b[986] = (s.v[703] == 0.0);
        s.v[986] = if s.b[986] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && (!s.b[978])) && s.b[984]) && s.b[986]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && (!s.b[978])) && s.b[984]) && (!s.b[986])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[988] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[988] = if s.b[988] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && (!s.b[978])) && (s.b[985] && (!s.b[984]))) && s.b[988]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && (!s.b[978])) && (s.b[985] && (!s.b[984]))) && (!s.b[988])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && (!s.b[978])) && (!(s.b[984] || s.b[985]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[989] = (0.0 == 1.0);
        s.v[989] = if s.b[989] { 1.0 } else { 0.0 };

        s.b[990] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[990] = if s.b[990] { 1.0 } else { 0.0 };

        s.b[991] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[991] = if s.b[991] { 1.0 } else { 0.0 };

        s.b[992] = (s.v[701] == 0.0);
        s.v[992] = if s.b[992] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && s.b[989]) && s.b[990]) && s.b[992]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && s.b[989]) && s.b[990]) && (!s.b[992])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[994] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));
        s.v[994] = if s.b[994] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && s.b[989]) && (s.b[991] && (!s.b[990]))) && s.b[994]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && s.b[989]) && (s.b[991] && (!s.b[990]))) && (!s.b[994])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (6.0 * s.v[236]));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && s.b[989]) && (!(s.b[990] || s.b[991]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[995] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[995] = if s.b[995] { 1.0 } else { 0.0 };

        s.b[996] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[996] = if s.b[996] { 1.0 } else { 0.0 };

        s.b[997] = (s.v[701] == 0.0);
        s.v[997] = if s.b[997] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && (!s.b[989])) && s.b[995]) && s.b[997]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && (!s.b[989])) && s.b[995]) && (!s.b[997])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[999] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));
        s.v[999] = if s.b[999] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && (!s.b[989])) && (s.b[996] && (!s.b[995]))) && s.b[999]) {
            s.store_scalar(705, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && (!s.b[989])) && (s.b[996] && (!s.b[995]))) && (!s.b[999])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (6.0 * s.v[236]));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && (!s.b[989])) && (!(s.b[995] || s.b[996]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1000] = (1.0 == 1.0);
        s.v[1000] = if s.b[1000] { 1.0 } else { 0.0 };

        s.b[1001] = (1.0 == 1.0);
        s.v[1001] = if s.b[1001] { 1.0 } else { 0.0 };

        s.b[1002] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1002] = if s.b[1002] { 1.0 } else { 0.0 };

        s.b[1003] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1003] = if s.b[1003] { 1.0 } else { 0.0 };

        s.b[1004] = (s.v[703] == 0.0);
        s.v[1004] = if s.b[1004] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && s.b[1001]) && s.b[1002]) && s.b[1004]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && s.b[1001]) && s.b[1002]) && (!s.b[1004])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1006] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));
        s.v[1006] = if s.b[1006] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && s.b[1001]) && (s.b[1003] && (!s.b[1002]))) && s.b[1006]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && s.b[1001]) && (s.b[1003] && (!s.b[1002]))) && (!s.b[1006])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (6.0 * s.v[236]));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && s.b[1001]) && (!(s.b[1002] || s.b[1003]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1007] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1007] = if s.b[1007] { 1.0 } else { 0.0 };

        s.b[1008] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1008] = if s.b[1008] { 1.0 } else { 0.0 };

        s.b[1009] = (s.v[703] == 0.0);
        s.v[1009] = if s.b[1009] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && (!s.b[1001])) && s.b[1007]) && s.b[1009]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && (!s.b[1001])) && s.b[1007]) && (!s.b[1009])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1011] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));
        s.v[1011] = if s.b[1011] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && (!s.b[1001])) && (s.b[1008] && (!s.b[1007]))) && s.b[1011]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && (!s.b[1001])) && (s.b[1008] && (!s.b[1007]))) && (!s.b[1011])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (6.0 * s.v[236]));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && (!s.b[1001])) && (!(s.b[1007] || s.b[1008]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1012] = (0.0 == 1.0);
        s.v[1012] = if s.b[1012] { 1.0 } else { 0.0 };

        s.b[1013] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1013] = if s.b[1013] { 1.0 } else { 0.0 };

        s.b[1014] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1014] = if s.b[1014] { 1.0 } else { 0.0 };

        s.b[1015] = (s.v[701] == 0.0);
        s.v[1015] = if s.b[1015] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && s.b[1012]) && s.b[1013]) && s.b[1015]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && s.b[1012]) && s.b[1013]) && (!s.b[1015])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1017] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1017] = if s.b[1017] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && s.b[1012]) && (s.b[1014] && (!s.b[1013]))) && s.b[1017]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && s.b[1012]) && (s.b[1014] && (!s.b[1013]))) && (!s.b[1017])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && s.b[1012]) && (!(s.b[1013] || s.b[1014]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1018] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1018] = if s.b[1018] { 1.0 } else { 0.0 };

        s.b[1019] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1019] = if s.b[1019] { 1.0 } else { 0.0 };

        s.b[1020] = (s.v[701] == 0.0);
        s.v[1020] = if s.b[1020] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && (!s.b[1012])) && s.b[1018]) && s.b[1020]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && (!s.b[1012])) && s.b[1018]) && (!s.b[1020])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1022] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1022] = if s.b[1022] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && (!s.b[1012])) && (s.b[1019] && (!s.b[1018]))) && s.b[1022]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && (!s.b[1012])) && (s.b[1019] && (!s.b[1018]))) && (!s.b[1022])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && (!s.b[1012])) && (!(s.b[1018] || s.b[1019]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1023] = (1.0 == 1.0);
        s.v[1023] = if s.b[1023] { 1.0 } else { 0.0 };

        s.b[1024] = (1.0 == 1.0);
        s.v[1024] = if s.b[1024] { 1.0 } else { 0.0 };

        s.b[1025] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1025] = if s.b[1025] { 1.0 } else { 0.0 };

        s.b[1026] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1026] = if s.b[1026] { 1.0 } else { 0.0 };

        s.b[1027] = (s.v[703] == 0.0);
        s.v[1027] = if s.b[1027] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && s.b[1024]) && s.b[1025]) && s.b[1027]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && s.b[1024]) && s.b[1025]) && (!s.b[1027])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1029] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));
        s.v[1029] = if s.b[1029] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && s.b[1024]) && (s.b[1026] && (!s.b[1025]))) && s.b[1029]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && s.b[1024]) && (s.b[1026] && (!s.b[1025]))) && (!s.b[1029])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (6.0 * s.v[236]));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && s.b[1024]) && (!(s.b[1025] || s.b[1026]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1030] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1030] = if s.b[1030] { 1.0 } else { 0.0 };

        s.b[1031] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1031] = if s.b[1031] { 1.0 } else { 0.0 };

        s.b[1032] = (s.v[703] == 0.0);
        s.v[1032] = if s.b[1032] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && (!s.b[1024])) && s.b[1030]) && s.b[1032]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && (!s.b[1024])) && s.b[1030]) && (!s.b[1032])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1034] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));
        s.v[1034] = if s.b[1034] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && (!s.b[1024])) && (s.b[1031] && (!s.b[1030]))) && s.b[1034]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && (!s.b[1024])) && (s.b[1031] && (!s.b[1030]))) && (!s.b[1034])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (6.0 * s.v[236]));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && (!s.b[1024])) && (!(s.b[1030] || s.b[1031]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1035] = (0.0 == 1.0);
        s.v[1035] = if s.b[1035] { 1.0 } else { 0.0 };

        s.b[1036] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1036] = if s.b[1036] { 1.0 } else { 0.0 };

        s.b[1037] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1037] = if s.b[1037] { 1.0 } else { 0.0 };

        s.b[1038] = (s.v[701] == 0.0);
        s.v[1038] = if s.b[1038] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && s.b[1035]) && s.b[1036]) && s.b[1038]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && s.b[1035]) && s.b[1036]) && (!s.b[1038])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1040] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));
        s.v[1040] = if s.b[1040] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && s.b[1035]) && (s.b[1037] && (!s.b[1036]))) && s.b[1040]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && s.b[1035]) && (s.b[1037] && (!s.b[1036]))) && (!s.b[1040])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (6.0 * s.v[236]));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && s.b[1035]) && (!(s.b[1036] || s.b[1037]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1041] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1041] = if s.b[1041] { 1.0 } else { 0.0 };

        s.b[1042] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1042] = if s.b[1042] { 1.0 } else { 0.0 };

        s.b[1043] = (s.v[701] == 0.0);
        s.v[1043] = if s.b[1043] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && (!s.b[1035])) && s.b[1041]) && s.b[1043]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && (!s.b[1035])) && s.b[1041]) && (!s.b[1043])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1045] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));
        s.v[1045] = if s.b[1045] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && (!s.b[1035])) && (s.b[1042] && (!s.b[1041]))) && s.b[1045]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && (!s.b[1035])) && (s.b[1042] && (!s.b[1041]))) && (!s.b[1045])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (6.0 * s.v[236]));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && (!s.b[1035])) && (!(s.b[1041] || s.b[1042]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1046] = (1.0 == 1.0);
        s.v[1046] = if s.b[1046] { 1.0 } else { 0.0 };

        s.b[1047] = (1.0 == 1.0);
        s.v[1047] = if s.b[1047] { 1.0 } else { 0.0 };

        s.b[1048] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1048] = if s.b[1048] { 1.0 } else { 0.0 };

        s.b[1049] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1049] = if s.b[1049] { 1.0 } else { 0.0 };

        s.b[1050] = (s.v[703] == 0.0);
        s.v[1050] = if s.b[1050] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && s.b[1047]) && s.b[1048]) && s.b[1050]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && s.b[1047]) && s.b[1048]) && (!s.b[1050])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1052] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1052] = if s.b[1052] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && s.b[1047]) && (s.b[1049] && (!s.b[1048]))) && s.b[1052]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && s.b[1047]) && (s.b[1049] && (!s.b[1048]))) && (!s.b[1052])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && s.b[1047]) && (!(s.b[1048] || s.b[1049]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1053] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1053] = if s.b[1053] { 1.0 } else { 0.0 };

        s.b[1054] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1054] = if s.b[1054] { 1.0 } else { 0.0 };

        s.b[1055] = (s.v[703] == 0.0);
        s.v[1055] = if s.b[1055] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && (!s.b[1047])) && s.b[1053]) && s.b[1055]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && (!s.b[1047])) && s.b[1053]) && (!s.b[1055])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1057] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1057] = if s.b[1057] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && (!s.b[1047])) && (s.b[1054] && (!s.b[1053]))) && s.b[1057]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && (!s.b[1047])) && (s.b[1054] && (!s.b[1053]))) && (!s.b[1057])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && (!s.b[1047])) && (!(s.b[1053] || s.b[1054]))) {
            s.store_scalar(705, 0.0);
        }

        if ((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && (!s.b[1046])) {
            s.store_scalar(705, ((p.p374 * s.v[237]) / s.v[29]));
        }

        s.b[1058] = (1.0 == 1.0);
        s.v[1058] = if s.b[1058] { 1.0 } else { 0.0 };

        s.b[1059] = (1.0 == 1.0);
        s.v[1059] = if s.b[1059] { 1.0 } else { 0.0 };

        s.b[1060] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1060] = if s.b[1060] { 1.0 } else { 0.0 };

        s.b[1061] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1061] = if s.b[1061] { 1.0 } else { 0.0 };

        s.b[1062] = (s.v[703] == 0.0);
        s.v[1062] = if s.b[1062] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && s.b[1059]) && s.b[1060]) && s.b[1062]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && s.b[1059]) && s.b[1060]) && (!s.b[1062])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1064] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));
        s.v[1064] = if s.b[1064] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && s.b[1059]) && (s.b[1061] && (!s.b[1060]))) && s.b[1064]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && s.b[1059]) && (s.b[1061] && (!s.b[1060]))) && (!s.b[1064])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (6.0 * s.v[236]));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && s.b[1059]) && (!(s.b[1060] || s.b[1061]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1065] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1065] = if s.b[1065] { 1.0 } else { 0.0 };

        s.b[1066] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1066] = if s.b[1066] { 1.0 } else { 0.0 };

        s.b[1067] = (s.v[703] == 0.0);
        s.v[1067] = if s.b[1067] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && (!s.b[1059])) && s.b[1065]) && s.b[1067]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && (!s.b[1059])) && s.b[1065]) && (!s.b[1067])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1069] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));
        s.v[1069] = if s.b[1069] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && (!s.b[1059])) && (s.b[1066] && (!s.b[1065]))) && s.b[1069]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && (!s.b[1059])) && (s.b[1066] && (!s.b[1065]))) && (!s.b[1069])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (6.0 * s.v[236]));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && (!s.b[1059])) && (!(s.b[1065] || s.b[1066]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1070] = (s.v[701] == 0.0);
        s.v[1070] = if s.b[1070] { 1.0 } else { 0.0 };

        if (((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && (!s.b[1058])) && s.b[1070]) {
            s.store_scalar(705, 0.0);
        }

        if (((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && (!s.b[1058])) && (!s.b[1070])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[237]), 701, s.v[29]);
        }

        s.b[1071] = (1.0 == 1.0);
        s.v[1071] = if s.b[1071] { 1.0 } else { 0.0 };

        if ((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && s.b[1071]) {
            s.store_scalar(705, ((p.p374 * s.v[237]) / s.v[29]));
        }

        s.b[1072] = (0.0 == 1.0);
        s.v[1072] = if s.b[1072] { 1.0 } else { 0.0 };

        s.b[1073] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1073] = if s.b[1073] { 1.0 } else { 0.0 };

        s.b[1074] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1074] = if s.b[1074] { 1.0 } else { 0.0 };

        s.b[1075] = (s.v[701] == 0.0);
        s.v[1075] = if s.b[1075] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && s.b[1072]) && s.b[1073]) && s.b[1075]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && s.b[1072]) && s.b[1073]) && (!s.b[1075])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1077] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1077] = if s.b[1077] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && s.b[1072]) && (s.b[1074] && (!s.b[1073]))) && s.b[1077]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && s.b[1072]) && (s.b[1074] && (!s.b[1073]))) && (!s.b[1077])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && s.b[1072]) && (!(s.b[1073] || s.b[1074]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1078] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1078] = if s.b[1078] { 1.0 } else { 0.0 };

        s.b[1079] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1079] = if s.b[1079] { 1.0 } else { 0.0 };

        s.b[1080] = (s.v[701] == 0.0);
        s.v[1080] = if s.b[1080] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && (!s.b[1072])) && s.b[1078]) && s.b[1080]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && (!s.b[1072])) && s.b[1078]) && (!s.b[1080])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1082] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1082] = if s.b[1082] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && (!s.b[1072])) && (s.b[1079] && (!s.b[1078]))) && s.b[1082]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && (!s.b[1072])) && (s.b[1079] && (!s.b[1078]))) && (!s.b[1082])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && (!s.b[1072])) && (!(s.b[1078] || s.b[1079]))) {
            s.store_scalar(705, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1083] = (1.0 == 1.0);
        s.v[1083] = if s.b[1083] { 1.0 } else { 0.0 };

        s.b[1084] = (s.v[703] == 0.0);
        s.v[1084] = if s.b[1084] { 1.0 } else { 0.0 };

        if (((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && s.b[1083]) && s.b[1084]) {
            s.store_scalar(705, 0.0);
        }

        if (((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && s.b[1083]) && (!s.b[1084])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[237]), 703, s.v[29]);
        }

        s.b[1085] = (0.0 == 1.0);
        s.v[1085] = if s.b[1085] { 1.0 } else { 0.0 };

        s.b[1086] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1086] = if s.b[1086] { 1.0 } else { 0.0 };

        s.b[1087] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1087] = if s.b[1087] { 1.0 } else { 0.0 };

        s.b[1088] = (s.v[701] == 0.0);
        s.v[1088] = if s.b[1088] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && s.b[1085]) && s.b[1086]) && s.b[1088]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && s.b[1085]) && s.b[1086]) && (!s.b[1088])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1090] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));
        s.v[1090] = if s.b[1090] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && s.b[1085]) && (s.b[1087] && (!s.b[1086]))) && s.b[1090]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && s.b[1085]) && (s.b[1087] && (!s.b[1086]))) && (!s.b[1090])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (6.0 * s.v[236]));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && s.b[1085]) && (!(s.b[1086] || s.b[1087]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1091] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1091] = if s.b[1091] { 1.0 } else { 0.0 };

        s.b[1092] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1092] = if s.b[1092] { 1.0 } else { 0.0 };

        s.b[1093] = (s.v[701] == 0.0);
        s.v[1093] = if s.b[1093] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && (!s.b[1085])) && s.b[1091]) && s.b[1093]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && (!s.b[1085])) && s.b[1091]) && (!s.b[1093])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1095] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));
        s.v[1095] = if s.b[1095] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && (!s.b[1085])) && (s.b[1092] && (!s.b[1091]))) && s.b[1095]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && (!s.b[1085])) && (s.b[1092] && (!s.b[1091]))) && (!s.b[1095])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (6.0 * s.v[236]));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && (!s.b[1085])) && (!(s.b[1091] || s.b[1092]))) {
            s.store_scalar(705, 0.0);
        }

        if (((!s.b[935]) && s.b[936]) && (s.b[951] && (!(((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950])))) {
            s.store_scalar(705, ((p.p374 * s.v[237]) / s.v[29]));
        }

        s.b[1096] = (1.0 == 1.0);
        s.v[1096] = if s.b[1096] { 1.0 } else { 0.0 };

        if ((((!s.b[935]) && s.b[936]) && (s.b[952] && (!((((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950]) || s.b[951])))) && s.b[1096]) {
            s.store_scalar(705, (((0.5 * p.p374) * s.v[236]) / s.v[29]));
        }

        s.b[1097] = (p.p2 == 2.0);
        s.v[1097] = if s.b[1097] { 1.0 } else { 0.0 };

        if (((((!s.b[935]) && s.b[936]) && (s.b[952] && (!((((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950]) || s.b[951])))) && s.b[1096]) && s.b[1097]) {
            s.store_scalar(704, 0.0);
        }

        if (((((!s.b[935]) && s.b[936]) && (s.b[952] && (!((((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950]) || s.b[951])))) && s.b[1096]) && (!s.b[1097])) {
            s.store_scalar(704, ((p.p374 * s.v[236]) / (s.v[29] * (p.p2 - 2.0))));
        }

        if ((((!s.b[935]) && s.b[936]) && (s.b[952] && (!((((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950]) || s.b[951])))) && (!s.b[1096])) {
            s.store_scalar(705, 0.0);
            s.store_scalar(704, ((p.p374 * s.v[236]) / (s.v[29] * p.p2)));
        }

        s.b[1098] = (1.0 == 1.0);
        s.v[1098] = if s.b[1098] { 1.0 } else { 0.0 };

        if ((((!s.b[935]) && s.b[936]) && (s.b[953] && (!(((((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950]) || s.b[951]) || s.b[952])))) && s.b[1098]) {
            s.store_scalar(705, 0.0);
            s.store_scalar(704, ((p.p374 * s.v[236]) / (s.v[29] * p.p2)));
        }

        if ((((!s.b[935]) && s.b[936]) && (s.b[953] && (!(((((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950]) || s.b[951]) || s.b[952])))) && (!s.b[1098])) {
            s.store_scalar(705, (((0.5 * p.p374) * s.v[236]) / s.v[29]));
        }

        s.b[1099] = (p.p2 == 2.0);
        s.v[1099] = if s.b[1099] { 1.0 } else { 0.0 };

        if (((((!s.b[935]) && s.b[936]) && (s.b[953] && (!(((((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950]) || s.b[951]) || s.b[952])))) && (!s.b[1098])) && s.b[1099]) {
            s.store_scalar(704, 0.0);
        }

        if (((((!s.b[935]) && s.b[936]) && (s.b[953] && (!(((((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950]) || s.b[951]) || s.b[952])))) && (!s.b[1098])) && (!s.b[1099])) {
            s.store_scalar(704, ((p.p374 * s.v[236]) / (s.v[29] * (p.p2 - 2.0))));
        }

        if (((!s.b[935]) && s.b[936]) && (!((((((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950]) || s.b[951]) || s.b[952]) || s.b[953]))) {
            s.store_scalar(704, 0.0);
        }

        s.b[1100] = (s.v[704] <= 0.0);
        s.v[1100] = if s.b[1100] { 1.0 } else { 0.0 };

        if (((!s.b[935]) && s.b[936]) && s.b[1100]) {
            s.copy_ad(239, 705);
        }

        s.b[1101] = (s.v[705] <= 0.0);
        s.v[1101] = if s.b[1101] { 1.0 } else { 0.0 };

        if ((((!s.b[935]) && s.b[936]) && (!s.b[1100])) && s.b[1101]) {
            s.copy_ad(239, 704);
        }

        if ((((!s.b[935]) && s.b[936]) && (!s.b[1100])) && (!s.b[1101])) {
            s.store_div_ad(239, A::mul(s.ad_value(704), s.ad_value(705)), A::add(s.ad_value(704), s.ad_value(705)));
        }

        if ((!s.b[935]) && (!s.b[936])) {
            s.store_scalar(239, 0.0);
        }

        s.b[1103] = param_given[4];
        s.v[1103] = if s.b[1103] { 1.0 } else { 0.0 };

        if s.b[1103] {
            s.store_scalar(240, (p.p374 * p.p4));
        }

        s.b[1104] = ((p.p10 > 0.0) && (p.p374 > 0.0));
        s.v[1104] = if s.b[1104] { 1.0 } else { 0.0 };

        s.b[1105] = (p.p9 < 9.0);
        s.v[1105] = if s.b[1105] { 1.0 } else { 0.0 };

        s.b[1106] = ((p.p2 % 2.0) != 0.0);
        s.v[1106] = if s.b[1106] { 1.0 } else { 0.0 };

        if ((((!s.b[1103]) && s.b[1104]) && s.b[1105]) && s.b[1106]) {
            s.store_scalar(701, 1.0);
            s.store_scalar(703, 1.0);
            s.store_scalar(700, (2.0 * (((p.p2 - 1.0) / 2.0)).max(0.0)));
            s.copy_ad(702, 700);
        }

        s.b[1107] = (p.p6 == 1.0);
        s.v[1107] = if s.b[1107] { 1.0 } else { 0.0 };

        if (((((!s.b[1103]) && s.b[1104]) && s.b[1105]) && (!s.b[1106])) && s.b[1107]) {
            s.store_scalar(701, 2.0);
            s.store_scalar(700, (2.0 * (((p.p2 / 2.0) - 1.0)).max(0.0)));
            s.store_scalar(703, 0.0);
            s.store_scalar(702, p.p2);
        }

        if (((((!s.b[1103]) && s.b[1104]) && s.b[1105]) && (!s.b[1106])) && (!s.b[1107])) {
            s.store_scalar(701, 0.0);
            s.store_scalar(700, p.p2);
            s.store_scalar(703, 2.0);
            s.store_scalar(702, (2.0 * (((p.p2 / 2.0) - 1.0)).max(0.0)));
        }

        s.b[1108] = (0.0 == 1.0);
        s.v[1108] = if s.b[1108] { 1.0 } else { 0.0 };

        s.b[1109] = (s.v[702] == 0.0);
        s.v[1109] = if s.b[1109] { 1.0 } else { 0.0 };

        if (((((!s.b[1103]) && s.b[1104]) && s.b[1105]) && s.b[1108]) && s.b[1109]) {
            s.store_scalar(704, 0.0);
        }

        if (((((!s.b[1103]) && s.b[1104]) && s.b[1105]) && s.b[1108]) && (!s.b[1109])) {
            s.store_div_from_scalar_scaled_input(704, (p.p374 * s.v[236]), 702, s.v[29]);
        }

        s.b[1110] = (s.v[700] == 0.0);
        s.v[1110] = if s.b[1110] { 1.0 } else { 0.0 };

        if (((((!s.b[1103]) && s.b[1104]) && s.b[1105]) && (!s.b[1108])) && s.b[1110]) {
            s.store_scalar(704, 0.0);
        }

        if (((((!s.b[1103]) && s.b[1104]) && s.b[1105]) && (!s.b[1108])) && (!s.b[1110])) {
            s.store_div_from_scalar_scaled_input(704, (p.p374 * s.v[236]), 700, s.v[29]);
        }

        s.b[1111] = (p.p9 == 0.0);
        s.v[1111] = if s.b[1111] { 1.0 } else { 0.0 };

        s.b[1112] = (p.p9 == 1.0);
        s.v[1112] = if s.b[1112] { 1.0 } else { 0.0 };

        s.b[1113] = (p.p9 == 2.0);
        s.v[1113] = if s.b[1113] { 1.0 } else { 0.0 };

        s.b[1114] = (p.p9 == 3.0);
        s.v[1114] = if s.b[1114] { 1.0 } else { 0.0 };

        s.b[1115] = (p.p9 == 4.0);
        s.v[1115] = if s.b[1115] { 1.0 } else { 0.0 };

        s.b[1116] = (p.p9 == 5.0);
        s.v[1116] = if s.b[1116] { 1.0 } else { 0.0 };

        s.b[1117] = (p.p9 == 6.0);
        s.v[1117] = if s.b[1117] { 1.0 } else { 0.0 };

        s.b[1118] = (p.p9 == 7.0);
        s.v[1118] = if s.b[1118] { 1.0 } else { 0.0 };

        s.b[1119] = (p.p9 == 8.0);
        s.v[1119] = if s.b[1119] { 1.0 } else { 0.0 };

        s.b[1120] = (p.p9 == 9.0);
        s.v[1120] = if s.b[1120] { 1.0 } else { 0.0 };

        s.b[1121] = (p.p9 == 10.0);
        s.v[1121] = if s.b[1121] { 1.0 } else { 0.0 };

        s.b[1122] = (0.0 == 1.0);
        s.v[1122] = if s.b[1122] { 1.0 } else { 0.0 };

        s.b[1123] = (1.0 == 1.0);
        s.v[1123] = if s.b[1123] { 1.0 } else { 0.0 };

        s.b[1124] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1124] = if s.b[1124] { 1.0 } else { 0.0 };

        s.b[1125] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1125] = if s.b[1125] { 1.0 } else { 0.0 };

        s.b[1126] = (s.v[703] == 0.0);
        s.v[1126] = if s.b[1126] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && s.b[1123]) && s.b[1124]) && s.b[1126]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && s.b[1123]) && s.b[1124]) && (!s.b[1126])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1128] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1128] = if s.b[1128] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && s.b[1123]) && (s.b[1125] && (!s.b[1124]))) && s.b[1128]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && s.b[1123]) && (s.b[1125] && (!s.b[1124]))) && (!s.b[1128])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && s.b[1123]) && (!(s.b[1124] || s.b[1125]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1129] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1129] = if s.b[1129] { 1.0 } else { 0.0 };

        s.b[1130] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1130] = if s.b[1130] { 1.0 } else { 0.0 };

        s.b[1131] = (s.v[703] == 0.0);
        s.v[1131] = if s.b[1131] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && (!s.b[1123])) && s.b[1129]) && s.b[1131]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && (!s.b[1123])) && s.b[1129]) && (!s.b[1131])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1133] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1133] = if s.b[1133] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && (!s.b[1123])) && (s.b[1130] && (!s.b[1129]))) && s.b[1133]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && (!s.b[1123])) && (s.b[1130] && (!s.b[1129]))) && (!s.b[1133])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && (!s.b[1123])) && (!(s.b[1129] || s.b[1130]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1134] = (0.0 == 1.0);
        s.v[1134] = if s.b[1134] { 1.0 } else { 0.0 };

        s.b[1135] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1135] = if s.b[1135] { 1.0 } else { 0.0 };

        s.b[1136] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1136] = if s.b[1136] { 1.0 } else { 0.0 };

        s.b[1137] = (s.v[701] == 0.0);
        s.v[1137] = if s.b[1137] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && s.b[1134]) && s.b[1135]) && s.b[1137]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && s.b[1134]) && s.b[1135]) && (!s.b[1137])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1139] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1139] = if s.b[1139] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && s.b[1134]) && (s.b[1136] && (!s.b[1135]))) && s.b[1139]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && s.b[1134]) && (s.b[1136] && (!s.b[1135]))) && (!s.b[1139])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && s.b[1134]) && (!(s.b[1135] || s.b[1136]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1140] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1140] = if s.b[1140] { 1.0 } else { 0.0 };

        s.b[1141] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1141] = if s.b[1141] { 1.0 } else { 0.0 };

        s.b[1142] = (s.v[701] == 0.0);
        s.v[1142] = if s.b[1142] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && (!s.b[1134])) && s.b[1140]) && s.b[1142]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && (!s.b[1134])) && s.b[1140]) && (!s.b[1142])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1144] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1144] = if s.b[1144] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && (!s.b[1134])) && (s.b[1141] && (!s.b[1140]))) && s.b[1144]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && (!s.b[1134])) && (s.b[1141] && (!s.b[1140]))) && (!s.b[1144])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && (!s.b[1134])) && (!(s.b[1140] || s.b[1141]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1145] = (0.0 == 1.0);
        s.v[1145] = if s.b[1145] { 1.0 } else { 0.0 };

        s.b[1146] = (1.0 == 1.0);
        s.v[1146] = if s.b[1146] { 1.0 } else { 0.0 };

        s.b[1147] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1147] = if s.b[1147] { 1.0 } else { 0.0 };

        s.b[1148] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1148] = if s.b[1148] { 1.0 } else { 0.0 };

        s.b[1149] = (s.v[703] == 0.0);
        s.v[1149] = if s.b[1149] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && s.b[1146]) && s.b[1147]) && s.b[1149]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && s.b[1146]) && s.b[1147]) && (!s.b[1149])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1151] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1151] = if s.b[1151] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && s.b[1146]) && (s.b[1148] && (!s.b[1147]))) && s.b[1151]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && s.b[1146]) && (s.b[1148] && (!s.b[1147]))) && (!s.b[1151])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && s.b[1146]) && (!(s.b[1147] || s.b[1148]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1152] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1152] = if s.b[1152] { 1.0 } else { 0.0 };

        s.b[1153] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1153] = if s.b[1153] { 1.0 } else { 0.0 };

        s.b[1154] = (s.v[703] == 0.0);
        s.v[1154] = if s.b[1154] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && (!s.b[1146])) && s.b[1152]) && s.b[1154]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && (!s.b[1146])) && s.b[1152]) && (!s.b[1154])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1156] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1156] = if s.b[1156] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && (!s.b[1146])) && (s.b[1153] && (!s.b[1152]))) && s.b[1156]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && (!s.b[1146])) && (s.b[1153] && (!s.b[1152]))) && (!s.b[1156])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && (!s.b[1146])) && (!(s.b[1152] || s.b[1153]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1157] = (0.0 == 1.0);
        s.v[1157] = if s.b[1157] { 1.0 } else { 0.0 };

        s.b[1158] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1158] = if s.b[1158] { 1.0 } else { 0.0 };

        s.b[1159] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1159] = if s.b[1159] { 1.0 } else { 0.0 };

        s.b[1160] = (s.v[701] == 0.0);
        s.v[1160] = if s.b[1160] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && s.b[1157]) && s.b[1158]) && s.b[1160]) {
            s.store_scalar(705, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_5(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && s.b[1157]) && s.b[1158]) && (!s.b[1160])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1162] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));
        s.v[1162] = if s.b[1162] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && s.b[1157]) && (s.b[1159] && (!s.b[1158]))) && s.b[1162]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && s.b[1157]) && (s.b[1159] && (!s.b[1158]))) && (!s.b[1162])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (6.0 * s.v[236]));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && s.b[1157]) && (!(s.b[1158] || s.b[1159]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1163] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1163] = if s.b[1163] { 1.0 } else { 0.0 };

        s.b[1164] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1164] = if s.b[1164] { 1.0 } else { 0.0 };

        s.b[1165] = (s.v[701] == 0.0);
        s.v[1165] = if s.b[1165] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && (!s.b[1157])) && s.b[1163]) && s.b[1165]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && (!s.b[1157])) && s.b[1163]) && (!s.b[1165])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1167] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));
        s.v[1167] = if s.b[1167] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && (!s.b[1157])) && (s.b[1164] && (!s.b[1163]))) && s.b[1167]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && (!s.b[1157])) && (s.b[1164] && (!s.b[1163]))) && (!s.b[1167])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (6.0 * s.v[236]));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && (!s.b[1157])) && (!(s.b[1163] || s.b[1164]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1168] = (0.0 == 1.0);
        s.v[1168] = if s.b[1168] { 1.0 } else { 0.0 };

        s.b[1169] = (1.0 == 1.0);
        s.v[1169] = if s.b[1169] { 1.0 } else { 0.0 };

        s.b[1170] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1170] = if s.b[1170] { 1.0 } else { 0.0 };

        s.b[1171] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1171] = if s.b[1171] { 1.0 } else { 0.0 };

        s.b[1172] = (s.v[703] == 0.0);
        s.v[1172] = if s.b[1172] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && s.b[1169]) && s.b[1170]) && s.b[1172]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && s.b[1169]) && s.b[1170]) && (!s.b[1172])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1174] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));
        s.v[1174] = if s.b[1174] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && s.b[1169]) && (s.b[1171] && (!s.b[1170]))) && s.b[1174]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && s.b[1169]) && (s.b[1171] && (!s.b[1170]))) && (!s.b[1174])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (6.0 * s.v[236]));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && s.b[1169]) && (!(s.b[1170] || s.b[1171]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1175] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1175] = if s.b[1175] { 1.0 } else { 0.0 };

        s.b[1176] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1176] = if s.b[1176] { 1.0 } else { 0.0 };

        s.b[1177] = (s.v[703] == 0.0);
        s.v[1177] = if s.b[1177] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && (!s.b[1169])) && s.b[1175]) && s.b[1177]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && (!s.b[1169])) && s.b[1175]) && (!s.b[1177])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1179] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));
        s.v[1179] = if s.b[1179] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && (!s.b[1169])) && (s.b[1176] && (!s.b[1175]))) && s.b[1179]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && (!s.b[1169])) && (s.b[1176] && (!s.b[1175]))) && (!s.b[1179])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (6.0 * s.v[236]));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && (!s.b[1169])) && (!(s.b[1175] || s.b[1176]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1180] = (0.0 == 1.0);
        s.v[1180] = if s.b[1180] { 1.0 } else { 0.0 };

        s.b[1181] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1181] = if s.b[1181] { 1.0 } else { 0.0 };

        s.b[1182] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1182] = if s.b[1182] { 1.0 } else { 0.0 };

        s.b[1183] = (s.v[701] == 0.0);
        s.v[1183] = if s.b[1183] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && s.b[1180]) && s.b[1181]) && s.b[1183]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && s.b[1180]) && s.b[1181]) && (!s.b[1183])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1185] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1185] = if s.b[1185] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && s.b[1180]) && (s.b[1182] && (!s.b[1181]))) && s.b[1185]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && s.b[1180]) && (s.b[1182] && (!s.b[1181]))) && (!s.b[1185])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && s.b[1180]) && (!(s.b[1181] || s.b[1182]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1186] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1186] = if s.b[1186] { 1.0 } else { 0.0 };

        s.b[1187] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1187] = if s.b[1187] { 1.0 } else { 0.0 };

        s.b[1188] = (s.v[701] == 0.0);
        s.v[1188] = if s.b[1188] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && (!s.b[1180])) && s.b[1186]) && s.b[1188]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && (!s.b[1180])) && s.b[1186]) && (!s.b[1188])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1190] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1190] = if s.b[1190] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && (!s.b[1180])) && (s.b[1187] && (!s.b[1186]))) && s.b[1190]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && (!s.b[1180])) && (s.b[1187] && (!s.b[1186]))) && (!s.b[1190])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && (!s.b[1180])) && (!(s.b[1186] || s.b[1187]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1191] = (0.0 == 1.0);
        s.v[1191] = if s.b[1191] { 1.0 } else { 0.0 };

        s.b[1192] = (1.0 == 1.0);
        s.v[1192] = if s.b[1192] { 1.0 } else { 0.0 };

        s.b[1193] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1193] = if s.b[1193] { 1.0 } else { 0.0 };

        s.b[1194] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1194] = if s.b[1194] { 1.0 } else { 0.0 };

        s.b[1195] = (s.v[703] == 0.0);
        s.v[1195] = if s.b[1195] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && s.b[1192]) && s.b[1193]) && s.b[1195]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && s.b[1192]) && s.b[1193]) && (!s.b[1195])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1197] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));
        s.v[1197] = if s.b[1197] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && s.b[1192]) && (s.b[1194] && (!s.b[1193]))) && s.b[1197]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && s.b[1192]) && (s.b[1194] && (!s.b[1193]))) && (!s.b[1197])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (6.0 * s.v[236]));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && s.b[1192]) && (!(s.b[1193] || s.b[1194]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1198] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1198] = if s.b[1198] { 1.0 } else { 0.0 };

        s.b[1199] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1199] = if s.b[1199] { 1.0 } else { 0.0 };

        s.b[1200] = (s.v[703] == 0.0);
        s.v[1200] = if s.b[1200] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && (!s.b[1192])) && s.b[1198]) && s.b[1200]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && (!s.b[1192])) && s.b[1198]) && (!s.b[1200])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1202] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));
        s.v[1202] = if s.b[1202] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && (!s.b[1192])) && (s.b[1199] && (!s.b[1198]))) && s.b[1202]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && (!s.b[1192])) && (s.b[1199] && (!s.b[1198]))) && (!s.b[1202])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (6.0 * s.v[236]));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && (!s.b[1192])) && (!(s.b[1198] || s.b[1199]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1203] = (0.0 == 1.0);
        s.v[1203] = if s.b[1203] { 1.0 } else { 0.0 };

        s.b[1204] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1204] = if s.b[1204] { 1.0 } else { 0.0 };

        s.b[1205] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1205] = if s.b[1205] { 1.0 } else { 0.0 };

        s.b[1206] = (s.v[701] == 0.0);
        s.v[1206] = if s.b[1206] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && s.b[1203]) && s.b[1204]) && s.b[1206]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && s.b[1203]) && s.b[1204]) && (!s.b[1206])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1208] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));
        s.v[1208] = if s.b[1208] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && s.b[1203]) && (s.b[1205] && (!s.b[1204]))) && s.b[1208]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && s.b[1203]) && (s.b[1205] && (!s.b[1204]))) && (!s.b[1208])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (6.0 * s.v[236]));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && s.b[1203]) && (!(s.b[1204] || s.b[1205]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1209] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1209] = if s.b[1209] { 1.0 } else { 0.0 };

        s.b[1210] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1210] = if s.b[1210] { 1.0 } else { 0.0 };

        s.b[1211] = (s.v[701] == 0.0);
        s.v[1211] = if s.b[1211] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && (!s.b[1203])) && s.b[1209]) && s.b[1211]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && (!s.b[1203])) && s.b[1209]) && (!s.b[1211])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1213] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));
        s.v[1213] = if s.b[1213] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && (!s.b[1203])) && (s.b[1210] && (!s.b[1209]))) && s.b[1213]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && (!s.b[1203])) && (s.b[1210] && (!s.b[1209]))) && (!s.b[1213])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (6.0 * s.v[236]));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && (!s.b[1203])) && (!(s.b[1209] || s.b[1210]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1214] = (0.0 == 1.0);
        s.v[1214] = if s.b[1214] { 1.0 } else { 0.0 };

        s.b[1215] = (1.0 == 1.0);
        s.v[1215] = if s.b[1215] { 1.0 } else { 0.0 };

        s.b[1216] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1216] = if s.b[1216] { 1.0 } else { 0.0 };

        s.b[1217] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1217] = if s.b[1217] { 1.0 } else { 0.0 };

        s.b[1218] = (s.v[703] == 0.0);
        s.v[1218] = if s.b[1218] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && s.b[1215]) && s.b[1216]) && s.b[1218]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && s.b[1215]) && s.b[1216]) && (!s.b[1218])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1220] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1220] = if s.b[1220] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && s.b[1215]) && (s.b[1217] && (!s.b[1216]))) && s.b[1220]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && s.b[1215]) && (s.b[1217] && (!s.b[1216]))) && (!s.b[1220])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && s.b[1215]) && (!(s.b[1216] || s.b[1217]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1221] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1221] = if s.b[1221] { 1.0 } else { 0.0 };

        s.b[1222] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1222] = if s.b[1222] { 1.0 } else { 0.0 };

        s.b[1223] = (s.v[703] == 0.0);
        s.v[1223] = if s.b[1223] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && (!s.b[1215])) && s.b[1221]) && s.b[1223]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && (!s.b[1215])) && s.b[1221]) && (!s.b[1223])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1225] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1225] = if s.b[1225] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && (!s.b[1215])) && (s.b[1222] && (!s.b[1221]))) && s.b[1225]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && (!s.b[1215])) && (s.b[1222] && (!s.b[1221]))) && (!s.b[1225])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && (!s.b[1215])) && (!(s.b[1221] || s.b[1222]))) {
            s.store_scalar(705, 0.0);
        }

        if ((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && (!s.b[1214])) {
            s.store_scalar(705, ((p.p374 * s.v[237]) / s.v[29]));
        }

        s.b[1226] = (0.0 == 1.0);
        s.v[1226] = if s.b[1226] { 1.0 } else { 0.0 };

        s.b[1227] = (1.0 == 1.0);
        s.v[1227] = if s.b[1227] { 1.0 } else { 0.0 };

        s.b[1228] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1228] = if s.b[1228] { 1.0 } else { 0.0 };

        s.b[1229] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1229] = if s.b[1229] { 1.0 } else { 0.0 };

        s.b[1230] = (s.v[703] == 0.0);
        s.v[1230] = if s.b[1230] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && s.b[1227]) && s.b[1228]) && s.b[1230]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && s.b[1227]) && s.b[1228]) && (!s.b[1230])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1232] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));
        s.v[1232] = if s.b[1232] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && s.b[1227]) && (s.b[1229] && (!s.b[1228]))) && s.b[1232]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && s.b[1227]) && (s.b[1229] && (!s.b[1228]))) && (!s.b[1232])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (6.0 * s.v[236]));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && s.b[1227]) && (!(s.b[1228] || s.b[1229]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1233] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1233] = if s.b[1233] { 1.0 } else { 0.0 };

        s.b[1234] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1234] = if s.b[1234] { 1.0 } else { 0.0 };

        s.b[1235] = (s.v[703] == 0.0);
        s.v[1235] = if s.b[1235] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && (!s.b[1227])) && s.b[1233]) && s.b[1235]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && (!s.b[1227])) && s.b[1233]) && (!s.b[1235])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1237] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));
        s.v[1237] = if s.b[1237] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && (!s.b[1227])) && (s.b[1234] && (!s.b[1233]))) && s.b[1237]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && (!s.b[1227])) && (s.b[1234] && (!s.b[1233]))) && (!s.b[1237])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (6.0 * s.v[236]));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && (!s.b[1227])) && (!(s.b[1233] || s.b[1234]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1238] = (s.v[701] == 0.0);
        s.v[1238] = if s.b[1238] { 1.0 } else { 0.0 };

        if (((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && (!s.b[1226])) && s.b[1238]) {
            s.store_scalar(705, 0.0);
        }

        if (((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && (!s.b[1226])) && (!s.b[1238])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[237]), 701, s.v[29]);
        }

        s.b[1239] = (0.0 == 1.0);
        s.v[1239] = if s.b[1239] { 1.0 } else { 0.0 };

        if ((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && s.b[1239]) {
            s.store_scalar(705, ((p.p374 * s.v[237]) / s.v[29]));
        }

        s.b[1240] = (0.0 == 1.0);
        s.v[1240] = if s.b[1240] { 1.0 } else { 0.0 };

        s.b[1241] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1241] = if s.b[1241] { 1.0 } else { 0.0 };

        s.b[1242] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1242] = if s.b[1242] { 1.0 } else { 0.0 };

        s.b[1243] = (s.v[701] == 0.0);
        s.v[1243] = if s.b[1243] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && s.b[1240]) && s.b[1241]) && s.b[1243]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && s.b[1240]) && s.b[1241]) && (!s.b[1243])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1245] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1245] = if s.b[1245] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_6(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && s.b[1240]) && (s.b[1242] && (!s.b[1241]))) && s.b[1245]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && s.b[1240]) && (s.b[1242] && (!s.b[1241]))) && (!s.b[1245])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && s.b[1240]) && (!(s.b[1241] || s.b[1242]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1246] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1246] = if s.b[1246] { 1.0 } else { 0.0 };

        s.b[1247] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1247] = if s.b[1247] { 1.0 } else { 0.0 };

        s.b[1248] = (s.v[701] == 0.0);
        s.v[1248] = if s.b[1248] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && (!s.b[1240])) && s.b[1246]) && s.b[1248]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && (!s.b[1240])) && s.b[1246]) && (!s.b[1248])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1250] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1250] = if s.b[1250] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && (!s.b[1240])) && (s.b[1247] && (!s.b[1246]))) && s.b[1250]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && (!s.b[1240])) && (s.b[1247] && (!s.b[1246]))) && (!s.b[1250])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && (!s.b[1240])) && (!(s.b[1246] || s.b[1247]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1251] = (0.0 == 1.0);
        s.v[1251] = if s.b[1251] { 1.0 } else { 0.0 };

        s.b[1252] = (s.v[703] == 0.0);
        s.v[1252] = if s.b[1252] { 1.0 } else { 0.0 };

        if (((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && s.b[1251]) && s.b[1252]) {
            s.store_scalar(705, 0.0);
        }

        if (((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && s.b[1251]) && (!s.b[1252])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[237]), 703, s.v[29]);
        }

        s.b[1253] = (0.0 == 1.0);
        s.v[1253] = if s.b[1253] { 1.0 } else { 0.0 };

        s.b[1254] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1254] = if s.b[1254] { 1.0 } else { 0.0 };

        s.b[1255] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1255] = if s.b[1255] { 1.0 } else { 0.0 };

        s.b[1256] = (s.v[701] == 0.0);
        s.v[1256] = if s.b[1256] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && s.b[1253]) && s.b[1254]) && s.b[1256]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && s.b[1253]) && s.b[1254]) && (!s.b[1256])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1258] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));
        s.v[1258] = if s.b[1258] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && s.b[1253]) && (s.b[1255] && (!s.b[1254]))) && s.b[1258]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && s.b[1253]) && (s.b[1255] && (!s.b[1254]))) && (!s.b[1258])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (6.0 * s.v[236]));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && s.b[1253]) && (!(s.b[1254] || s.b[1255]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1259] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1259] = if s.b[1259] { 1.0 } else { 0.0 };

        s.b[1260] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1260] = if s.b[1260] { 1.0 } else { 0.0 };

        s.b[1261] = (s.v[701] == 0.0);
        s.v[1261] = if s.b[1261] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && (!s.b[1253])) && s.b[1259]) && s.b[1261]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && (!s.b[1253])) && s.b[1259]) && (!s.b[1261])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1263] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));
        s.v[1263] = if s.b[1263] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && (!s.b[1253])) && (s.b[1260] && (!s.b[1259]))) && s.b[1263]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && (!s.b[1253])) && (s.b[1260] && (!s.b[1259]))) && (!s.b[1263])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (6.0 * s.v[236]));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && (!s.b[1253])) && (!(s.b[1259] || s.b[1260]))) {
            s.store_scalar(705, 0.0);
        }

        if (((!s.b[1103]) && s.b[1104]) && (s.b[1119] && (!(((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118])))) {
            s.store_scalar(705, ((p.p374 * s.v[237]) / s.v[29]));
        }

        s.b[1264] = (0.0 == 1.0);
        s.v[1264] = if s.b[1264] { 1.0 } else { 0.0 };

        if ((((!s.b[1103]) && s.b[1104]) && (s.b[1120] && (!((((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118]) || s.b[1119])))) && s.b[1264]) {
            s.store_scalar(705, (((0.5 * p.p374) * s.v[236]) / s.v[29]));
        }

        s.b[1265] = (p.p2 == 2.0);
        s.v[1265] = if s.b[1265] { 1.0 } else { 0.0 };

        if (((((!s.b[1103]) && s.b[1104]) && (s.b[1120] && (!((((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118]) || s.b[1119])))) && s.b[1264]) && s.b[1265]) {
            s.store_scalar(704, 0.0);
        }

        if (((((!s.b[1103]) && s.b[1104]) && (s.b[1120] && (!((((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118]) || s.b[1119])))) && s.b[1264]) && (!s.b[1265])) {
            s.store_scalar(704, ((p.p374 * s.v[236]) / (s.v[29] * (p.p2 - 2.0))));
        }

        if ((((!s.b[1103]) && s.b[1104]) && (s.b[1120] && (!((((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118]) || s.b[1119])))) && (!s.b[1264])) {
            s.store_scalar(705, 0.0);
            s.store_scalar(704, ((p.p374 * s.v[236]) / (s.v[29] * p.p2)));
        }

        s.b[1266] = (0.0 == 1.0);
        s.v[1266] = if s.b[1266] { 1.0 } else { 0.0 };

        if ((((!s.b[1103]) && s.b[1104]) && (s.b[1121] && (!(((((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118]) || s.b[1119]) || s.b[1120])))) && s.b[1266]) {
            s.store_scalar(705, 0.0);
            s.store_scalar(704, ((p.p374 * s.v[236]) / (s.v[29] * p.p2)));
        }

        if ((((!s.b[1103]) && s.b[1104]) && (s.b[1121] && (!(((((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118]) || s.b[1119]) || s.b[1120])))) && (!s.b[1266])) {
            s.store_scalar(705, (((0.5 * p.p374) * s.v[236]) / s.v[29]));
        }

        s.b[1267] = (p.p2 == 2.0);
        s.v[1267] = if s.b[1267] { 1.0 } else { 0.0 };

        if (((((!s.b[1103]) && s.b[1104]) && (s.b[1121] && (!(((((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118]) || s.b[1119]) || s.b[1120])))) && (!s.b[1266])) && s.b[1267]) {
            s.store_scalar(704, 0.0);
        }

        if (((((!s.b[1103]) && s.b[1104]) && (s.b[1121] && (!(((((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118]) || s.b[1119]) || s.b[1120])))) && (!s.b[1266])) && (!s.b[1267])) {
            s.store_scalar(704, ((p.p374 * s.v[236]) / (s.v[29] * (p.p2 - 2.0))));
        }

        if (((!s.b[1103]) && s.b[1104]) && (!((((((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118]) || s.b[1119]) || s.b[1120]) || s.b[1121]))) {
            s.store_scalar(704, 0.0);
        }

        s.b[1268] = (s.v[704] <= 0.0);
        s.v[1268] = if s.b[1268] { 1.0 } else { 0.0 };

        if (((!s.b[1103]) && s.b[1104]) && s.b[1268]) {
            s.copy_ad(240, 705);
        }

        s.b[1269] = (s.v[705] <= 0.0);
        s.v[1269] = if s.b[1269] { 1.0 } else { 0.0 };

        if ((((!s.b[1103]) && s.b[1104]) && (!s.b[1268])) && s.b[1269]) {
            s.copy_ad(240, 704);
        }

        if ((((!s.b[1103]) && s.b[1104]) && (!s.b[1268])) && (!s.b[1269])) {
            s.store_div_ad(240, A::mul(s.ad_value(704), s.ad_value(705)), A::add(s.ad_value(704), s.ad_value(705)));
        }

        if ((!s.b[1103]) && (!s.b[1104])) {
            s.store_scalar(240, 0.0);
        }

        s.b[1271] = (p.p42 == 0.0);
        s.v[1271] = if s.b[1271] { 1.0 } else { 0.0 };

        s.b[1272] = (s.v[239] < p.p1093);
        s.v[1272] = if s.b[1272] { 1.0 } else { 0.0 };

        if (s.b[1271] && s.b[1272]) {
            s.store_scalar(239, 0.0);
        }

        s.b[1273] = (s.v[240] < p.p1093);
        s.v[1273] = if s.b[1273] { 1.0 } else { 0.0 };

        if (s.b[1271] && s.b[1273]) {
            s.store_scalar(240, 0.0);
        }

        s.b[1274] = (s.v[239] <= p.p1093);
        s.v[1274] = if s.b[1274] { 1.0 } else { 0.0 };

        if ((!s.b[1271]) && s.b[1274]) {
            s.store_scalar(239, p.p1093);
        }

        s.b[1275] = (s.v[240] <= p.p1093);
        s.v[1275] = if s.b[1275] { 1.0 } else { 0.0 };

        if ((!s.b[1271]) && s.b[1275]) {
            s.store_scalar(240, p.p1093);
        }

        s.b[1276] = (p.p42 == 1.0);
        s.v[1276] = if s.b[1276] { 1.0 } else { 0.0 };

        s.b[1277] = (s.v[529] <= 0.0);
        s.v[1277] = if s.b[1277] { 1.0 } else { 0.0 };

        if (s.b[1276] && s.b[1277]) {
            s.store_scalar(529, 0.0);
        }

        s.b[1278] = (s.v[528] <= 0.0);
        s.v[1278] = if s.b[1278] { 1.0 } else { 0.0 };

        if (s.b[1276] && s.b[1278]) {
            s.store_scalar(528, 0.0);
        }

        s.b[1279] = (s.v[531] <= 0.0);
        s.v[1279] = if s.b[1279] { 1.0 } else { 0.0 };

        if (s.b[1276] && s.b[1279]) {
            s.store_scalar(531, 0.0);
        }

        s.b[1280] = (s.v[530] <= 0.0);
        s.v[1280] = if s.b[1280] { 1.0 } else { 0.0 };

        if (s.b[1276] && s.b[1280]) {
            s.store_scalar(530, 0.0);
        }

        s.b[1281] = (s.v[533] <= 0.0);
        s.v[1281] = if s.b[1281] { 1.0 } else { 0.0 };

        if ((!s.b[1276]) && s.b[1281]) {
            s.store_scalar(533, 0.0);
        }

        s.b[1282] = (s.v[532] <= 0.0);
        s.v[1282] = if s.b[1282] { 1.0 } else { 0.0 };

        if ((!s.b[1276]) && s.b[1282]) {
            s.store_scalar(532, 0.0);
        }

        s.v[271] = 0.0;

        s.v[272] = 0.0;

        s.v[273] = 0.0;

        s.v[274] = 0.0;

        s.v[275] = 0.0;

        s.v[276] = 0.0;

        s.b[1283] = (p.p8 != 0.0);
        s.v[1283] = if s.b[1283] { 1.0 } else { 0.0 };

        if s.b[1283] {
            s.store_scalar(256, ((((s.v[30] * 1000000.0)).max(1e-38)) as f64).ln());
            s.store_scalar(257, ((((s.v[29] * 1000000.0)).max(1e-38)) as f64).ln());
            s.store_scalar(258, (((p.p2).max(1e-38)) as f64).ln());
            s.store_scalar(259, 5.0);
            s.store_scalar(268, p.p11);
            s.store_scalar(270, p.p12);
            s.store_scalar(269, p.p13);
            s.store_scalar(266, p.p14);
            s.store_scalar(267, p.p15);
        }

        s.b[1284] = ((!param_given[757]) || (!param_given[761]));
        s.v[1284] = if s.b[1284] { 1.0 } else { 0.0 };

        if (s.b[1283] && s.b[1284]) {
            s.store_scalar(259, 1.0);
        }

        s.b[1285] = (((!param_given[773]) && (!param_given[774])) || ((!param_given[775]) && (!param_given[776])));
        s.v[1285] = if s.b[1285] { 1.0 } else { 0.0 };

        if ((s.b[1283] && (!s.b[1284])) && s.b[1285]) {
            s.store_scalar(259, 3.0);
        }

        s.b[1286] = (p.p8 == 2.0);
        s.v[1286] = if s.b[1286] { 1.0 } else { 0.0 };

        s.b[1287] = (s.v[259] == 5.0);
        s.v[1287] = if s.b[1287] { 1.0 } else { 0.0 };

        if ((s.b[1283] && s.b[1286]) && s.b[1287]) {
            s.store_scaled_limited_exp_ad(262, A::add_scaled_inputs3(s.ad_value(256), p.p777, s.ad_value(257), p.p778, s.ad_value(258), p.p779), p.p773);
            s.store_scaled_limited_exp_ad(263, A::add_scaled_inputs3(s.ad_value(256), p.p780, s.ad_value(257), p.p781, s.ad_value(258), p.p782), p.p774);
            s.store_div_ad(267, A::mul(s.ad_value(262), s.ad_value(263)), A::add(s.ad_value(262), s.ad_value(263)));
            s.store_scaled_limited_exp_ad(264, A::add_scaled_inputs3(s.ad_value(256), p.p777, s.ad_value(257), p.p778, s.ad_value(258), p.p779), p.p775);
            s.store_scaled_limited_exp_ad(265, A::add_scaled_inputs3(s.ad_value(256), p.p780, s.ad_value(257), p.p781, s.ad_value(258), p.p782), p.p776);
            s.store_div_ad(266, A::mul(s.ad_value(264), s.ad_value(265)), A::add(s.ad_value(264), s.ad_value(265)));
        }

        s.b[1288] = ((s.v[259] == 3.0) || (s.v[259] == 5.0));
        s.v[1288] = if s.b[1288] { 1.0 } else { 0.0 };

        if ((s.b[1283] && s.b[1286]) && s.b[1288]) {
            s.store_scaled_limited_exp_ad(269, A::add_scaled_inputs3(s.ad_value(256), p.p758, s.ad_value(257), p.p759, s.ad_value(258), p.p760), p.p757);
            s.store_scaled_limited_exp_ad(270, A::add_scaled_inputs3(s.ad_value(256), p.p762, s.ad_value(257), p.p763, s.ad_value(258), p.p764), p.p761);
        }

        if (s.b[1283] && s.b[1286]) {
            s.store_scaled_limited_exp_ad(260, A::add_scaled_inputs3(s.ad_value(256), p.p766, s.ad_value(257), p.p767, s.ad_value(258), p.p768), p.p765);
            s.store_scaled_limited_exp_ad(261, A::add_scaled_inputs3(s.ad_value(256), p.p770, s.ad_value(257), p.p771, s.ad_value(258), p.p772), p.p769);
            s.store_div_ad(268, A::mul(s.ad_value(260), s.ad_value(261)), A::add(s.ad_value(260), s.ad_value(261)));
        }

        s.b[1289] = ((p.p8 == 1.0) || ((p.p8 == 2.0) && (s.v[259] == 5.0)));
        s.v[1289] = if s.b[1289] { 1.0 } else { 0.0 };

        s.b[1290] = (s.v[266] < 0.001);
        s.v[1290] = if s.b[1290] { 1.0 } else { 0.0 };

        if ((s.b[1283] && s.b[1289]) && s.b[1290]) {
            s.store_scalar(272, 1000.0);
        }

        if ((s.b[1283] && s.b[1289]) && (!s.b[1290])) {
            s.store_offset_div_from_scalar_ad(272, 1.0, s.ad_value(266), p.p756);
        }

        s.b[1291] = (s.v[268] < 0.001);
        s.v[1291] = if s.b[1291] { 1.0 } else { 0.0 };

        if ((s.b[1283] && s.b[1289]) && s.b[1291]) {
            s.store_scalar(273, 1000.0);
        }

        if ((s.b[1283] && s.b[1289]) && (!s.b[1291])) {
            s.store_offset_div_from_scalar_ad(273, 1.0, s.ad_value(268), p.p756);
        }

        s.b[1292] = (s.v[269] < 0.001);
        s.v[1292] = if s.b[1292] { 1.0 } else { 0.0 };

        if ((s.b[1283] && s.b[1289]) && s.b[1292]) {
            s.store_scalar(274, 1000.0);
        }

        if ((s.b[1283] && s.b[1289]) && (!s.b[1292])) {
            s.store_offset_div_from_scalar_ad(274, 1.0, s.ad_value(269), p.p756);
        }

        s.b[1293] = (s.v[267] < 0.001);
        s.v[1293] = if s.b[1293] { 1.0 } else { 0.0 };

        if ((s.b[1283] && s.b[1289]) && s.b[1293]) {
            s.store_scalar(271, 1000.0);
        }

        if ((s.b[1283] && s.b[1289]) && (!s.b[1293])) {
            s.store_offset_div_from_scalar_ad(271, 1.0, s.ad_value(267), p.p756);
        }

        s.b[1294] = (s.v[270] < 0.001);
        s.v[1294] = if s.b[1294] { 1.0 } else { 0.0 };

        if ((s.b[1283] && s.b[1289]) && s.b[1294]) {
            s.store_scalar(275, 1000.0);
        }

        if ((s.b[1283] && s.b[1289]) && (!s.b[1294])) {
            s.store_offset_div_from_scalar_ad(275, 1.0, s.ad_value(270), p.p756);
        }

        s.b[1295] = ((p.p8 == 2.0) && (s.v[259] == 3.0));
        s.v[1295] = if s.b[1295] { 1.0 } else { 0.0 };

        if ((s.b[1283] && (!s.b[1289])) && s.b[1295]) {
            s.store_scalar(272, p.p756);
            s.store_scalar(271, p.p756);
        }

        s.b[1296] = (s.v[268] < 0.001);
        s.v[1296] = if s.b[1296] { 1.0 } else { 0.0 };

        if (((s.b[1283] && (!s.b[1289])) && s.b[1295]) && s.b[1296]) {
            s.store_scalar(273, 1000.0);
        }

        if (((s.b[1283] && (!s.b[1289])) && s.b[1295]) && (!s.b[1296])) {
            s.store_offset_div_from_scalar_ad(273, 1.0, s.ad_value(268), p.p756);
        }

        s.b[1297] = (s.v[269] < 0.001);
        s.v[1297] = if s.b[1297] { 1.0 } else { 0.0 };

        if (((s.b[1283] && (!s.b[1289])) && s.b[1295]) && s.b[1297]) {
            s.store_scalar(274, 1000.0);
        }

        if (((s.b[1283] && (!s.b[1289])) && s.b[1295]) && (!s.b[1297])) {
            s.store_offset_div_from_scalar_ad(274, 1.0, s.ad_value(269), p.p756);
        }

        s.b[1298] = (s.v[270] < 0.001);
        s.v[1298] = if s.b[1298] { 1.0 } else { 0.0 };

        if (((s.b[1283] && (!s.b[1289])) && s.b[1295]) && s.b[1298]) {
            s.store_scalar(275, 1000.0);
        }

        if (((s.b[1283] && (!s.b[1289])) && s.b[1295]) && (!s.b[1298])) {
            s.store_offset_div_from_scalar_ad(275, 1.0, s.ad_value(270), p.p756);
        }

        s.b[1299] = ((p.p8 == 2.0) && (s.v[259] == 1.0));
        s.v[1299] = if s.b[1299] { 1.0 } else { 0.0 };

        if (((s.b[1283] && (!s.b[1289])) && (!s.b[1295])) && s.b[1299]) {
            s.store_scalar(272, p.p756);
        }

    }

    pub(super) fn stamp_transient_block_7(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        if (((s.b[1283] && (!s.b[1289])) && (!s.b[1295])) && s.b[1299]) {
            s.store_scalar(271, p.p756);
            s.store_scalar(274, 1000.0);
            s.store_scalar(275, 1000.0);
        }

        s.b[1300] = (s.v[268] < 0.001);
        s.v[1300] = if s.b[1300] { 1.0 } else { 0.0 };

        if ((((s.b[1283] && (!s.b[1289])) && (!s.b[1295])) && s.b[1299]) && s.b[1300]) {
            s.store_scalar(273, 1000.0);
        }

        if ((((s.b[1283] && (!s.b[1289])) && (!s.b[1295])) && s.b[1299]) && (!s.b[1300])) {
            s.store_offset_div_from_scalar_ad(273, 1.0, s.ad_value(268), p.p756);
        }

        s.b[1301] = (p.p1097 == 1.0);
        s.v[1301] = if s.b[1301] { 1.0 } else { 0.0 };

        s.b[1302] = (p.p16 < 0.001);
        s.v[1302] = if s.b[1302] { 1.0 } else { 0.0 };

        if (s.b[1301] && s.b[1302]) {
            s.store_scalar(276, 1000.0);
        }

        if (s.b[1301] && (!s.b[1302])) {
            s.store_scalar(276, (p.p756 + (1.0 / p.p16)));
        }

        if s.b[1301] {
            s.store_scalar(302, (1.0 - p.p1128));
        }

        if (!s.b[1301]) {
            s.store_scalar(302, 1.0);
        }

        s.v[252] = ((p.p700 * (p.p31 + ((s.v[35] / 3.0) / p.p32))) / ((p.p32 * p.p2) * (s.v[98] - p.p699)));

        s.b[1303] = (s.v[252] > 0.0);
        s.v[1303] = if s.b[1303] { 1.0 } else { 0.0 };

        if s.b[1303] {
            s.store_scalar(252, (1.0 / s.v[252]));
        }

        if (!s.b[1303]) {
            s.store_scalar(252, 1000.0);
        }

        s.v[12] = (p.p77 * p.p77);

        s.store_scale(13, 599, p.p77);

        s.store_square(14, 13);

        s.store_scaled_limited_exp_scaled_input(298, 603, ((((p.p555 / p.p77)).max(1e-38)) as f64).ln(), 1.0 / (s.v[12]));

        s.store_div_ad_lhs(299, A::limited_exp(A::mul(s.ad_value(603), A::ln(A::max_with_scalar(A::div_from_scalar(p.p555, s.ad_value(13)), 1e-38)))), 14);

        s.v[294] = (if (p.p39 == 1.0) { 4.97232e-7 } else { 3.42537e-7 });

        s.v[295] = (if (p.p39 == 1.0) { 745669000000.0 } else { 1166450000000.0 });

        s.store_scale(296, 299, (s.v[294] * s.v[29]));

        s.store_scale(297, 599, ((-s.v[295]) * p.p77));

        s.store_scale(294, 298, ((s.v[29] * s.v[30]) * s.v[294]));

        s.v[295] = ((-s.v[295]) * p.p77);

        s.v[38] = (p.p911 + s.v[29]);

        s.b[1305] = (((p.p49 != 0.0) && (p.p909 > 0.0)) && (s.v[38] > 0.0));
        s.v[1305] = if s.b[1305] { 1.0 } else { 0.0 };

        if s.b[1305] {
            s.store_scalar(747, ((s.v[38] * p.p2) / p.p909));
            s.store_scalar(748, ((p.p910 * s.v[38]) * p.p2));
        }

        if (!s.b[1305]) {
            s.store_scalar(747, 1.0);
            s.store_scalar(748, 0.0);
        }

        s.b[1306] = (p.p820 <= (-273.15));
        s.v[1306] = if s.b[1306] { 1.0 } else { 0.0 };

        if s.b[1306] {
            s.store_scalar(12, (300.15 - 273.15));
            s.store_scalar(392, 300.15);
        }

        if (!s.b[1306]) {
            s.store_scalar(392, (p.p820 + 273.15));
        }

        s.v[391] = (ctx_temp + p.p33);

        s.b[1307] = (((p.p49 != 0.0) && (p.p909 > 0.0)) && (s.v[38] > 0.0));
        s.v[1307] = if s.b[1307] { 1.0 } else { 0.0 };

        if s.b[1307] {
            s.store_voltage(390, ctx, nodes, Some(4), None);
        }

        if (!s.b[1307]) {
            s.store_scalar(390, 0.0);
        }

        s.store_offset(391, 390, s.v[391]);

        s.store_scale(108, 391, 8.617087e-5);

        s.store_div_from_scalar(109, 1.0, 108);

        s.store_div(395, 391, 392);

        s.store_sub(396, 391, 392);

        s.store_scale(393, 391, 8.617087e-5);

        s.store_scale(394, 392, 8.617087e-5);

        s.store_sub_from_scalar_ad(36, p.p109, A::div(A::mul_scaled_lhs(s.ad_value(391), p.p821, s.ad_value(391)), A::offset(s.ad_value(391), p.p822)));

        s.store_sub_from_scalar_ad(37, p.p109, A::div(A::mul_scaled_lhs(s.ad_value(392), p.p821, s.ad_value(392)), A::offset(s.ad_value(392), p.p822)));

        s.store_mul_ad(13, A::div(s.ad_value(391), s.ad_value(392)), A::sqrt(A::div(s.ad_value(391), s.ad_value(392))));

        s.store_mul_scaled_ad_rhs(28, 13, p.p108, A::limited_exp(A::sub(A::div_scaled_inputs(s.ad_value(36), 1.0, s.ad_value(394), 2.0), A::div_scaled_inputs(s.ad_value(36), 1.0, s.ad_value(393), 2.0))));

        s.b[1308] = (((p.p49 != 0.0) && (p.p909 > 0.0)) && (s.v[38] > 0.0));
        s.v[1308] = if s.b[1308] { 1.0 } else { 0.0 };

        if s.b[1308] {
            s.store_ln_ad(12, A::max_with_scalar(A::div(s.ad_value(481), s.ad_value(28)), 1e-38));
            s.store_sqrt_square_offset(88, 12, 1e-6);
        }

        if (!s.b[1308]) {
            s.store_ln_ad(88, A::max_with_scalar(A::div(s.ad_value(481), s.ad_value(28)), 1e-38));
        }

        s.b[1309] = (((p.p49 != 0.0) && (p.p909 > 0.0)) && (s.v[38] > 0.0));
        s.v[1309] = if s.b[1309] { 1.0 } else { 0.0 };

        if s.b[1309] {
            s.store_ln_ad(12, A::max_with_scalar(A::div(A::mul(s.ad_value(686), s.ad_value(480)), A::square(s.ad_value(28))), 1e-38));
            s.store_sqrt_square_offset(675, 12, 1e-6);
        }

        if (!s.b[1309]) {
            s.store_ln_ad(675, A::max_with_scalar(A::div(A::mul(s.ad_value(686), s.ad_value(480)), A::square(s.ad_value(28))), 1e-38));
        }

        s.b[1310] = (s.v[479] > 0.0);
        s.v[1310] = if s.b[1310] { 1.0 } else { 0.0 };

        if s.b[1310] {
            s.store_offset_ad(63, A::mul3_scaled_output(s.ad_value(187), s.ad_value(108), A::ln(A::max_with_scalar(A::div(s.ad_value(479), s.ad_value(480)), 1e-38)), -1.0), p.p5);
        }

        if (!s.b[1310]) {
            s.store_scalar(63, 0.0);
        }

        s.store_max_with_scalar_ad(127, A::add(A::offset(A::mul(s.ad_value(108), s.ad_value(88)), 0.4), s.ad_value(489)), 0.4);

        s.store_sqrt(128, 127);

        s.store_sqrt_div_from_scalar_ad(114, (2.0 * s.v[26]), A::scale(s.ad_value(481), 1.60219e-19));

        s.store_sqrt_scaled_input(129, 538, ((s.v[26] / s.v[27]) * p.p77));

        let assign13230_ad_e18111: A = {
    if (!((1.0 + (p.p823 * (s.v[395] - 1.0))) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::scale_offset(s.ad_value(395), p.p823, (((((-1.0)) * (p.p823))) + (1.0))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(395), p.p823, (((((-1.0)) * (p.p823))) + (1.0))), A::scale_offset(s.ad_value(395), p.p823, (((((-1.0)) * (p.p823))) + (1.0)))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if ((1.0 + (p.p823 * (s.v[395] - 1.0))) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(395), p.p823, (((((-1.0)) * (p.p823))) + (1.0))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(422, 488, assign13230_ad_e18111);

        s.store_mul_ad_rhs(420, 490, A::scale_offset(s.ad_value(395), p.p851, (((((-1.0)) * (p.p851))) + (1.0))));

        s.b[1311] = (p.p44 != 0.0);
        s.v[1311] = if s.b[1311] { 1.0 } else { 0.0 };

        if s.b[1311] {
            s.store_mul_ad_rhs(421, 491, A::scale_offset(s.ad_value(395), p.p851, (((((-1.0)) * (p.p851))) + (1.0))));
        }

        s.v[158] = (if (p.p39 != 1.0) { (0.3333333333333333 * p.p283) } else { (0.5 * p.p283) });

        s.store_mul_pow_ad_rhs(397, 497, s.ad_value(395), s.ad_value(567));

        let assign13290_ad_e18224: A = {
    if (!(((1.0 + (s.v[568] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(568), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(568), s.ad_value(396)), ((1.0) + ((-1e-6)))), A::offset(A::mul(s.ad_value(568), s.ad_value(396)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[568] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(568), s.ad_value(396)), ((1.0) + ((-1e-6)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(399, 504, assign13290_ad_e18224);

        let assign13300_ad_e18298: A = {
    if (!(((1.0 + (s.v[569] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(569), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(569), s.ad_value(396)), ((1.0) + ((-1e-6)))), A::offset(A::mul(s.ad_value(569), s.ad_value(396)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[569] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(569), s.ad_value(396)), ((1.0) + ((-1e-6)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(401, 514, assign13300_ad_e18298);

        s.store_mul_pow_ad_rhs(403, 508, s.ad_value(395), s.ad_value(570));

        s.store_mul_pow_ad_rhs(405, 511, s.ad_value(395), s.ad_value(571));

        let assign13330_ad_e18382: A = {
    if (!((1.0 + (s.v[577] * (s.v[395] - 1.0))) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(577), A::offset(s.ad_value(395), (-1.0))), 1.0), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(577), A::offset(s.ad_value(395), (-1.0))), 1.0), A::offset(A::mul(s.ad_value(577), A::offset(s.ad_value(395), (-1.0))), 1.0)), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if ((1.0 + (s.v[577] * (s.v[395] - 1.0))) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(577), A::offset(s.ad_value(395), (-1.0))), 1.0))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(407, 507, assign13330_ad_e18382);

        s.b[1312] = (p.p44 != 0.0);
        s.v[1312] = if s.b[1312] { 1.0 } else { 0.0 };

        if s.b[1312] {
            s.store_mul_pow_ad_rhs(398, 498, s.ad_value(395), s.ad_value(567));
        }

        if s.b[1312] {
            let assign13360_ad_e18468: A = {
                if (!(((1.0 + (s.v[568] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(568), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(568), s.ad_value(396)), ((1.0) + ((-1e-6)))), A::offset(A::mul(s.ad_value(568), s.ad_value(396)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[568] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(568), s.ad_value(396)), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(400, 505, assign13360_ad_e18468);
        }

        if s.b[1312] {
            let assign13370_ad_e18545: A = {
                if (!(((1.0 + (s.v[569] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(569), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(569), s.ad_value(396)), ((1.0) + ((-1e-6)))), A::offset(A::mul(s.ad_value(569), s.ad_value(396)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[569] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(569), s.ad_value(396)), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(402, 515, assign13370_ad_e18545);
        }

        if s.b[1312] {
            s.store_mul_pow_ad_rhs(404, 509, s.ad_value(395), s.ad_value(570));
            s.store_mul_pow_ad_rhs(406, 512, s.ad_value(395), s.ad_value(571));
        }

        s.store_pow_ad(408, s.ad_value(395), s.ad_value(572));

        s.store_mul_pow_ad_rhs(409, 500, s.ad_value(395), A::neg(s.ad_value(573)));

        s.b[1313] = (s.v[409] < 100.0);
        s.v[1313] = if s.b[1313] { 1.0 } else { 0.0 };

        if s.b[1313] {
            s.store_scalar(409, 100.0);
        }

        s.b[1314] = (p.p1094 == 1.0);
        s.v[1314] = if s.b[1314] { 1.0 } else { 0.0 };

        if s.b[1314] {
            s.store_powf(762, 395, p.p1120);
            s.store_scale_ad(763, A::powf(s.ad_value(395), (-p.p1121)), p.p1100);
        }

        s.b[1315] = (p.p44 != 0.0);
        s.v[1315] = if s.b[1315] { 1.0 } else { 0.0 };

        if s.b[1315] {
            s.store_mul_pow_ad_rhs(410, 501, s.ad_value(395), A::neg(s.ad_value(573)));
        }

        s.b[1316] = (s.v[410] < 100.0);
        s.v[1316] = if s.b[1316] { 1.0 } else { 0.0 };

        if (s.b[1315] && s.b[1316]) {
            s.store_scalar(410, 100.0);
        }

        s.store_mul_pow_ad_rhs(411, 503, s.ad_value(395), A::neg(s.ad_value(573)));

        s.b[1317] = (s.v[411] < 100.0);
        s.v[1317] = if s.b[1317] { 1.0 } else { 0.0 };

        if s.b[1317] {
            s.store_scalar(411, 100.0);
        }

        let assign13540_ad_e18729: A = {
    if (!((((1.0 / s.v[496]) * (1.0 + (p.p861 * s.v[396]))) - 2.0) < ((-10000.0) * 0.001))) {
        let assign13540_ad_e18693: A = A::add(A::offset(A::mul(A::div_from_scalar(1.0, s.ad_value(496)), A::scale_offset(s.ad_value(396), p.p861, 1.0)), (-2.0)), A::sqrt(A::offset(A::mul(A::offset(A::mul(A::div_from_scalar(1.0, s.ad_value(496)), A::scale_offset(s.ad_value(396), p.p861, 1.0)), (-2.0)), A::offset(A::mul(A::div_from_scalar(1.0, s.ad_value(496)), A::scale_offset(s.ad_value(396), p.p861, 1.0)), (-2.0))), ((4.0 * 0.001) * 0.001))));
        A::scale(assign13540_ad_e18693, 0.5)
    } else {
        {
            if ((((1.0 / s.v[496]) * (1.0 + (p.p861 * s.v[396]))) - 2.0) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(A::div_from_scalar(1.0, s.ad_value(496)), A::scale_offset(s.ad_value(396), p.p861, 1.0)), (-2.0)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_div_from_scalar_offset_ad(412, 1.0, assign13540_ad_e18729, 2.0);

        let assign13550_ad_e18805: A = {
    if (!(((1.0 - (s.v[574] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6)), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6)), A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 - (s.v[574] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(413, 534, assign13550_ad_e18805);

        s.b[1318] = (p.p44 != 0.0);
        s.v[1318] = if s.b[1318] { 1.0 } else { 0.0 };

        if s.b[1318] {
            let assign13570_ad_e18883: A = {
                if (!(((1.0 - (s.v[574] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6)), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6)), A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 - (s.v[574] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(414, 535, assign13570_ad_e18883);
        }

        let assign13580_ad_e18959: A = {
    if (!(((1.0 + (s.v[149] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(149), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(149), s.ad_value(396)), ((1.0) + ((-1e-6)))), A::offset(A::mul(s.ad_value(149), s.ad_value(396)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[149] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(149), s.ad_value(396)), ((1.0) + ((-1e-6)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(150, 148, assign13580_ad_e18959);

        let assign13590_ad_e19033: A = {
    if (!(((1.0 + (s.v[152] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(152), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(152), s.ad_value(396)), ((1.0) + ((-1e-6)))), A::offset(A::mul(s.ad_value(152), s.ad_value(396)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[152] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(152), s.ad_value(396)), ((1.0) + ((-1e-6)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(153, 151, assign13590_ad_e19033);

        s.store_mul_pow_ad_rhs(415, 554, s.ad_value(395), s.ad_value(575));

        s.b[1319] = (p.p44 != 0.0);
        s.v[1319] = if s.b[1319] { 1.0 } else { 0.0 };

        if s.b[1319] {
            s.store_mul_pow_ad_rhs(416, 557, s.ad_value(395), s.ad_value(575));
        }

        let assign13630_ad_e19123: A = {
    if (!(((1.0 + (s.v[576] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(576), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(576), s.ad_value(396)), ((1.0) + ((-1e-6)))), A::offset(A::mul(s.ad_value(576), s.ad_value(396)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[576] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(576), s.ad_value(396)), ((1.0) + ((-1e-6)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(417, 560, assign13630_ad_e19123);

        let assign13640_ad_e19197: A = {
    if (!(((1.0 + (s.v[576] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(576), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(576), s.ad_value(396)), ((1.0) + ((-1e-6)))), A::offset(A::mul(s.ad_value(576), s.ad_value(396)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[576] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(576), s.ad_value(396)), ((1.0) + ((-1e-6)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(418, 564, assign13640_ad_e19197);

        s.store_limited_exp_ad(419, A::mul(s.ad_value(604), A::ln(A::max_with_scalar(s.ad_value(395), 1e-38))));

        let assign13660_ad_e19278: A = {
    if (!(((1.0 + (s.v[607] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(607), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(607), s.ad_value(396)), ((1.0) + ((-1e-6)))), A::offset(A::mul(s.ad_value(607), s.ad_value(396)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[607] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(607), s.ad_value(396)), ((1.0) + ((-1e-6)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(609, 605, assign13660_ad_e19278);

    }

    pub(super) fn stamp_transient_block_8(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let assign13670_ad_e19352: A = {
    if (!(((1.0 + (s.v[608] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(608), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(608), s.ad_value(396)), ((1.0) + ((-1e-6)))), A::offset(A::mul(s.ad_value(608), s.ad_value(396)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[608] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(608), s.ad_value(396)), ((1.0) + ((-1e-6)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(610, 606, assign13670_ad_e19352);

        let assign13680_ad_e19426: A = {
    if (!(((1.0 + (s.v[632] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(632), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(632), s.ad_value(396)), ((1.0) + ((-1e-6)))), A::offset(A::mul(s.ad_value(632), s.ad_value(396)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[632] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(632), s.ad_value(396)), ((1.0) + ((-1e-6)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(633, 631, assign13680_ad_e19426);

        let assign13690_ad_e19500: A = {
    if (!(((1.0 + (s.v[635] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(635), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(635), s.ad_value(396)), ((1.0) + ((-1e-6)))), A::offset(A::mul(s.ad_value(635), s.ad_value(396)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[635] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(635), s.ad_value(396)), ((1.0) + ((-1e-6)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(636, 634, assign13690_ad_e19500);

        let assign13700_ad_e19574: A = {
    if (!(((1.0 + (s.v[638] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(638), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(638), s.ad_value(396)), ((1.0) + ((-1e-6)))), A::offset(A::mul(s.ad_value(638), s.ad_value(396)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[638] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(638), s.ad_value(396)), ((1.0) + ((-1e-6)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(639, 637, assign13700_ad_e19574);

        let assign13710_ad_e19648: A = {
    if (!(((1.0 + (p.p889 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::scale_offset(s.ad_value(396), p.p889, ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(396), p.p889, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(396), p.p889, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (p.p889 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(396), p.p889, ((1.0) + ((-1e-6)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_scale_ad(423, assign13710_ad_e19648, p.p701);

        let assign13720_ad_e19722: A = {
    if (!(((1.0 + (p.p889 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::scale_offset(s.ad_value(396), p.p889, ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(396), p.p889, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(396), p.p889, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (p.p889 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(396), p.p889, ((1.0) + ((-1e-6)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_scale_ad(426, assign13720_ad_e19722, p.p702);

        let assign13730_ad_e19796: A = {
    if (!(((1.0 + (p.p890 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::scale_offset(s.ad_value(396), p.p890, ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(396), p.p890, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(396), p.p890, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (p.p890 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(396), p.p890, ((1.0) + ((-1e-6)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_scale_ad(424, assign13730_ad_e19796, p.p703);

        let assign13740_ad_e19870: A = {
    if (!(((1.0 + (p.p890 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::scale_offset(s.ad_value(396), p.p890, ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(396), p.p890, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(396), p.p890, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (p.p890 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(396), p.p890, ((1.0) + ((-1e-6)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_scale_ad(427, assign13740_ad_e19870, p.p704);

        let assign13750_ad_e19944: A = {
    if (!(((1.0 + (p.p891 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::scale_offset(s.ad_value(396), p.p891, ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(396), p.p891, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(396), p.p891, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (p.p891 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(396), p.p891, ((1.0) + ((-1e-6)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_scale_ad(428, assign13750_ad_e19944, p.p705);

        let assign13760_ad_e20018: A = {
    if (!(((1.0 + (p.p891 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::scale_offset(s.ad_value(396), p.p891, ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(396), p.p891, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(396), p.p891, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (p.p891 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(396), p.p891, ((1.0) + ((-1e-6)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_scale_ad(425, assign13760_ad_e20018, p.p706);

        let assign13770_ad_e20091: A = {
    if (!(((p.p707 - (p.p892 * s.v[396])) - 0.01) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p707, A::scale(s.ad_value(396), p.p892)), (-0.01)), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(p.p707, A::scale(s.ad_value(396), p.p892)), (-0.01)), A::offset(A::sub_from_scalar(p.p707, A::scale(s.ad_value(396), p.p892)), (-0.01))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((p.p707 - (p.p892 * s.v[396])) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::sub_from_scalar(p.p707, A::scale(s.ad_value(396), p.p892)), (-0.01)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(429, assign13770_ad_e20091, 0.01);

        let assign13780_ad_e20165: A = {
    if (!(((p.p708 - (p.p892 * s.v[396])) - 0.01) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p708, A::scale(s.ad_value(396), p.p892)), (-0.01)), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(p.p708, A::scale(s.ad_value(396), p.p892)), (-0.01)), A::offset(A::sub_from_scalar(p.p708, A::scale(s.ad_value(396), p.p892)), (-0.01))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((p.p708 - (p.p892 * s.v[396])) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::sub_from_scalar(p.p708, A::scale(s.ad_value(396), p.p892)), (-0.01)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(432, assign13780_ad_e20165, 0.01);

        let assign13790_ad_e20239: A = {
    if (!(((p.p709 - (p.p893 * s.v[396])) - 0.01) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p709, A::scale(s.ad_value(396), p.p893)), (-0.01)), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(p.p709, A::scale(s.ad_value(396), p.p893)), (-0.01)), A::offset(A::sub_from_scalar(p.p709, A::scale(s.ad_value(396), p.p893)), (-0.01))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((p.p709 - (p.p893 * s.v[396])) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::sub_from_scalar(p.p709, A::scale(s.ad_value(396), p.p893)), (-0.01)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(430, assign13790_ad_e20239, 0.01);

        let assign13800_ad_e20313: A = {
    if (!(((p.p710 - (p.p893 * s.v[396])) - 0.01) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p710, A::scale(s.ad_value(396), p.p893)), (-0.01)), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(p.p710, A::scale(s.ad_value(396), p.p893)), (-0.01)), A::offset(A::sub_from_scalar(p.p710, A::scale(s.ad_value(396), p.p893)), (-0.01))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((p.p710 - (p.p893 * s.v[396])) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::sub_from_scalar(p.p710, A::scale(s.ad_value(396), p.p893)), (-0.01)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(433, assign13800_ad_e20313, 0.01);

        let assign13810_ad_e20387: A = {
    if (!(((p.p711 - (p.p894 * s.v[396])) - 0.01) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p711, A::scale(s.ad_value(396), p.p894)), (-0.01)), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(p.p711, A::scale(s.ad_value(396), p.p894)), (-0.01)), A::offset(A::sub_from_scalar(p.p711, A::scale(s.ad_value(396), p.p894)), (-0.01))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((p.p711 - (p.p894 * s.v[396])) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::sub_from_scalar(p.p711, A::scale(s.ad_value(396), p.p894)), (-0.01)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(431, assign13810_ad_e20387, 0.01);

        let assign13820_ad_e20461: A = {
    if (!(((p.p712 - (p.p894 * s.v[396])) - 0.01) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p712, A::scale(s.ad_value(396), p.p894)), (-0.01)), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(p.p712, A::scale(s.ad_value(396), p.p894)), (-0.01)), A::offset(A::sub_from_scalar(p.p712, A::scale(s.ad_value(396), p.p894)), (-0.01))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((p.p712 - (p.p894 * s.v[396])) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::sub_from_scalar(p.p712, A::scale(s.ad_value(396), p.p894)), (-0.01)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(434, assign13820_ad_e20461, 0.01);

        s.store_sub_ad(12, A::div(s.ad_value(37), s.ad_value(394)), A::div(s.ad_value(36), s.ad_value(393)));

        s.store_ln_ad(13, A::max_with_scalar(s.ad_value(395), 1e-38));

        s.store_ad_value(15, A::limited_exp_scaled_input(A::add_scaled_inputs(s.ad_value(12), 1.0, s.ad_value(13), p.p895), 1.0 / (p.p725)));

        s.store_scale(435, 15, p.p719);

        s.store_scale(436, 15, p.p721);

        s.store_scale(437, 15, p.p723);

        s.store_ad_value(15, A::limited_exp_scaled_input(A::add_scaled_inputs(s.ad_value(12), 1.0, s.ad_value(13), p.p896), 1.0 / (p.p726)));

        s.store_scale(438, 15, p.p720);

        s.store_scale(439, 15, p.p722);

        s.store_scale(440, 15, p.p724);

        s.store_scaled_limited_exp_ad(441, A::div(A::mul_scaled_lhs(s.ad_value(37), p.p897, A::offset(s.ad_value(395), (-1.0))), s.ad_value(393)), p.p735);

        s.store_scaled_limited_exp_ad(443, A::div(A::mul_scaled_lhs(s.ad_value(37), p.p899, A::offset(s.ad_value(395), (-1.0))), s.ad_value(393)), p.p737);

        s.store_scaled_limited_exp_ad(445, A::div(A::mul_scaled_lhs(s.ad_value(37), p.p901, A::offset(s.ad_value(395), (-1.0))), s.ad_value(393)), (p.p739 * ((((p.p741 / s.v[35])) as f64).sqrt() + 1.0)));

        s.store_scaled_limited_exp_ad(442, A::div(A::mul_scaled_lhs(s.ad_value(37), p.p898, A::offset(s.ad_value(395), (-1.0))), s.ad_value(393)), p.p736);

        s.store_scaled_limited_exp_ad(444, A::div(A::mul_scaled_lhs(s.ad_value(37), p.p900, A::offset(s.ad_value(395), (-1.0))), s.ad_value(393)), p.p738);

        s.store_scaled_limited_exp_ad(446, A::div(A::mul_scaled_lhs(s.ad_value(37), p.p902, A::offset(s.ad_value(395), (-1.0))), s.ad_value(393)), (p.p740 * ((((p.p741 / s.v[35])) as f64).sqrt() + 1.0)));

        let assign13990_ad_e20690: A = {
    if (!(((p.p742 * (1.0 + (p.p903 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001))) {
        let assign13990_ad_e20654: A = A::add(A::scale_offset(s.ad_value(395), ((p.p903) * (p.p742)), (((((((((-1.0)) * (p.p903))) + (1.0))) * (p.p742))) + ((-0.01)))), A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(395), ((p.p903) * (p.p742)), (((((((((-1.0)) * (p.p903))) + (1.0))) * (p.p742))) + ((-0.01)))), A::scale_offset(s.ad_value(395), ((p.p903) * (p.p742)), (((((((((-1.0)) * (p.p903))) + (1.0))) * (p.p742))) + ((-0.01))))), ((4.0 * 0.001) * 0.001))));
        A::scale(assign13990_ad_e20654, 0.5)
    } else {
        {
            if (((p.p742 * (1.0 + (p.p903 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(395), ((p.p903) * (p.p742)), (((((((((-1.0)) * (p.p903))) + (1.0))) * (p.p742))) + ((-0.01)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(447, assign13990_ad_e20690, 0.01);

        let assign14000_ad_e20788: A = {
    if (!(((p.p744 * (1.0 + (p.p905 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001))) {
        let assign14000_ad_e20752: A = A::add(A::scale_offset(s.ad_value(395), ((p.p905) * (p.p744)), (((((((((-1.0)) * (p.p905))) + (1.0))) * (p.p744))) + ((-0.01)))), A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(395), ((p.p905) * (p.p744)), (((((((((-1.0)) * (p.p905))) + (1.0))) * (p.p744))) + ((-0.01)))), A::scale_offset(s.ad_value(395), ((p.p905) * (p.p744)), (((((((((-1.0)) * (p.p905))) + (1.0))) * (p.p744))) + ((-0.01))))), ((4.0 * 0.001) * 0.001))));
        A::scale(assign14000_ad_e20752, 0.5)
    } else {
        {
            if (((p.p744 * (1.0 + (p.p905 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(395), ((p.p905) * (p.p744)), (((((((((-1.0)) * (p.p905))) + (1.0))) * (p.p744))) + ((-0.01)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(449, assign14000_ad_e20788, 0.01);

        let assign14010_ad_e20886: A = {
    if (!(((p.p746 * (1.0 + (p.p907 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001))) {
        let assign14010_ad_e20850: A = A::add(A::scale_offset(s.ad_value(395), ((p.p907) * (p.p746)), (((((((((-1.0)) * (p.p907))) + (1.0))) * (p.p746))) + ((-0.01)))), A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(395), ((p.p907) * (p.p746)), (((((((((-1.0)) * (p.p907))) + (1.0))) * (p.p746))) + ((-0.01)))), A::scale_offset(s.ad_value(395), ((p.p907) * (p.p746)), (((((((((-1.0)) * (p.p907))) + (1.0))) * (p.p746))) + ((-0.01))))), ((4.0 * 0.001) * 0.001))));
        A::scale(assign14010_ad_e20850, 0.5)
    } else {
        {
            if (((p.p746 * (1.0 + (p.p907 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(395), ((p.p907) * (p.p746)), (((((((((-1.0)) * (p.p907))) + (1.0))) * (p.p746))) + ((-0.01)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(451, assign14010_ad_e20886, 0.01);

        let assign14020_ad_e20984: A = {
    if (!(((p.p743 * (1.0 + (p.p904 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001))) {
        let assign14020_ad_e20948: A = A::add(A::scale_offset(s.ad_value(395), ((p.p904) * (p.p743)), (((((((((-1.0)) * (p.p904))) + (1.0))) * (p.p743))) + ((-0.01)))), A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(395), ((p.p904) * (p.p743)), (((((((((-1.0)) * (p.p904))) + (1.0))) * (p.p743))) + ((-0.01)))), A::scale_offset(s.ad_value(395), ((p.p904) * (p.p743)), (((((((((-1.0)) * (p.p904))) + (1.0))) * (p.p743))) + ((-0.01))))), ((4.0 * 0.001) * 0.001))));
        A::scale(assign14020_ad_e20948, 0.5)
    } else {
        {
            if (((p.p743 * (1.0 + (p.p904 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(395), ((p.p904) * (p.p743)), (((((((((-1.0)) * (p.p904))) + (1.0))) * (p.p743))) + ((-0.01)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(448, assign14020_ad_e20984, 0.01);

        let assign14030_ad_e21082: A = {
    if (!(((p.p745 * (1.0 + (p.p906 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001))) {
        let assign14030_ad_e21046: A = A::add(A::scale_offset(s.ad_value(395), ((p.p906) * (p.p745)), (((((((((-1.0)) * (p.p906))) + (1.0))) * (p.p745))) + ((-0.01)))), A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(395), ((p.p906) * (p.p745)), (((((((((-1.0)) * (p.p906))) + (1.0))) * (p.p745))) + ((-0.01)))), A::scale_offset(s.ad_value(395), ((p.p906) * (p.p745)), (((((((((-1.0)) * (p.p906))) + (1.0))) * (p.p745))) + ((-0.01))))), ((4.0 * 0.001) * 0.001))));
        A::scale(assign14030_ad_e21046, 0.5)
    } else {
        {
            if (((p.p745 * (1.0 + (p.p906 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(395), ((p.p906) * (p.p745)), (((((((((-1.0)) * (p.p906))) + (1.0))) * (p.p745))) + ((-0.01)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(450, assign14030_ad_e21082, 0.01);

        let assign14040_ad_e21180: A = {
    if (!(((p.p747 * (1.0 + (p.p908 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001))) {
        let assign14040_ad_e21144: A = A::add(A::scale_offset(s.ad_value(395), ((p.p908) * (p.p747)), (((((((((-1.0)) * (p.p908))) + (1.0))) * (p.p747))) + ((-0.01)))), A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(395), ((p.p908) * (p.p747)), (((((((((-1.0)) * (p.p908))) + (1.0))) * (p.p747))) + ((-0.01)))), A::scale_offset(s.ad_value(395), ((p.p908) * (p.p747)), (((((((((-1.0)) * (p.p908))) + (1.0))) * (p.p747))) + ((-0.01))))), ((4.0 * 0.001) * 0.001))));
        A::scale(assign14040_ad_e21144, 0.5)
    } else {
        {
            if (((p.p747 * (1.0 + (p.p908 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(395), ((p.p908) * (p.p747)), (((((((((-1.0)) * (p.p908))) + (1.0))) * (p.p747))) + ((-0.01)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(452, assign14040_ad_e21180, 0.01);

        s.b[1320] = (p.p9 < 9.0);
        s.v[1320] = if s.b[1320] { 1.0 } else { 0.0 };

        s.b[1321] = ((p.p2 % 2.0) != 0.0);
        s.v[1321] = if s.b[1321] { 1.0 } else { 0.0 };

        if (s.b[1320] && s.b[1321]) {
            s.store_scalar(701, 1.0);
            s.store_scalar(703, 1.0);
            s.store_scalar(700, (2.0 * (((p.p2 - 1.0) / 2.0)).max(0.0)));
            s.copy_ad(702, 700);
        }

        s.b[1322] = (p.p6 == 1.0);
        s.v[1322] = if s.b[1322] { 1.0 } else { 0.0 };

        if ((s.b[1320] && (!s.b[1321])) && s.b[1322]) {
            s.store_scalar(701, 2.0);
            s.store_scalar(700, (2.0 * (((p.p2 / 2.0) - 1.0)).max(0.0)));
            s.store_scalar(703, 0.0);
            s.store_scalar(702, p.p2);
        }

        if ((s.b[1320] && (!s.b[1321])) && (!s.b[1322])) {
            s.store_scalar(701, 0.0);
            s.store_scalar(700, p.p2);
            s.store_scalar(703, 2.0);
            s.store_scalar(702, (2.0 * (((p.p2 / 2.0) - 1.0)).max(0.0)));
        }

        s.v[12] = (s.v[236] + s.v[238]);

        s.v[13] = (s.v[236] + s.v[236]);

        s.v[14] = (s.v[237] + s.v[237]);

        s.v[0] = ((s.v[12] + s.v[12]) + s.v[35]);

        s.v[1] = ((s.v[12] + s.v[12]) + s.v[35]);

        s.v[2] = s.v[13];

        s.v[3] = s.v[13];

        s.v[4] = s.v[14];

        s.v[5] = s.v[14];

        s.v[6] = (s.v[12] * s.v[35]);

        s.v[7] = (s.v[12] * s.v[35]);

        s.v[8] = (s.v[236] * s.v[35]);

        s.v[9] = (s.v[236] * s.v[35]);

        s.v[10] = (s.v[237] * s.v[35]);

        s.v[11] = (s.v[237] * s.v[35]);

        s.b[1323] = (p.p9 == 0.0);
        s.v[1323] = if s.b[1323] { 1.0 } else { 0.0 };

        s.b[1324] = (p.p9 == 1.0);
        s.v[1324] = if s.b[1324] { 1.0 } else { 0.0 };

        s.b[1325] = (p.p9 == 2.0);
        s.v[1325] = if s.b[1325] { 1.0 } else { 0.0 };

        s.b[1326] = (p.p9 == 3.0);
        s.v[1326] = if s.b[1326] { 1.0 } else { 0.0 };

        s.b[1327] = (p.p9 == 4.0);
        s.v[1327] = if s.b[1327] { 1.0 } else { 0.0 };

        s.b[1328] = (p.p9 == 5.0);
        s.v[1328] = if s.b[1328] { 1.0 } else { 0.0 };

        s.b[1329] = (p.p9 == 6.0);
        s.v[1329] = if s.b[1329] { 1.0 } else { 0.0 };

        s.b[1330] = (p.p9 == 7.0);
        s.v[1330] = if s.b[1330] { 1.0 } else { 0.0 };

        s.b[1331] = (p.p9 == 8.0);
        s.v[1331] = if s.b[1331] { 1.0 } else { 0.0 };

        s.b[1332] = (p.p9 == 9.0);
        s.v[1332] = if s.b[1332] { 1.0 } else { 0.0 };

        s.b[1333] = (p.p9 == 10.0);
        s.v[1333] = if s.b[1333] { 1.0 } else { 0.0 };

        if s.b[1323] {
            s.store_add_scaled_inputs(248, 703, s.v[0], 702, s.v[2]);
            s.store_add_scaled_inputs(249, 701, s.v[1], 700, s.v[3]);
            s.store_add_scaled_inputs(246, 703, s.v[6], 702, s.v[8]);
            s.store_add_scaled_inputs(247, 701, s.v[7], 700, s.v[9]);
        }

        if (s.b[1324] && (!s.b[1323])) {
            s.store_add_scaled_inputs(248, 703, s.v[0], 702, s.v[2]);
            s.store_scaled_add(249, 701, 700, s.v[3]);
        }

    }

    pub(super) fn stamp_transient_block_9(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1324] && (!s.b[1323])) {
            s.store_add_scaled_inputs(246, 703, s.v[6], 702, s.v[8]);
            s.store_scaled_add(247, 701, 700, s.v[9]);
        }

        if (s.b[1325] && (!(s.b[1323] || s.b[1324]))) {
            s.store_scaled_add(248, 703, 702, s.v[2]);
            s.store_add_scaled_inputs(249, 701, s.v[1], 700, s.v[3]);
            s.store_scaled_add(246, 703, 702, s.v[8]);
            s.store_add_scaled_inputs(247, 701, s.v[7], 700, s.v[9]);
        }

        if (s.b[1326] && (!((s.b[1323] || s.b[1324]) || s.b[1325]))) {
            s.store_scaled_add(248, 703, 702, s.v[2]);
            s.store_scaled_add(249, 701, 700, s.v[3]);
            s.store_scaled_add(246, 703, 702, s.v[8]);
            s.store_scaled_add(247, 701, 700, s.v[9]);
        }

        if (s.b[1327] && (!(((s.b[1323] || s.b[1324]) || s.b[1325]) || s.b[1326]))) {
            s.store_add_scaled_inputs(248, 703, s.v[0], 702, s.v[2]);
            s.store_add_scaled_inputs(249, 701, s.v[5], 700, s.v[3]);
            s.store_add_scaled_inputs(246, 703, s.v[6], 702, s.v[8]);
            s.store_add_scaled_inputs(247, 701, s.v[11], 700, s.v[9]);
        }

        if (s.b[1328] && (!((((s.b[1323] || s.b[1324]) || s.b[1325]) || s.b[1326]) || s.b[1327]))) {
            s.store_scaled_add(248, 703, 702, s.v[2]);
            s.store_add_scaled_inputs(249, 701, s.v[5], 700, s.v[3]);
            s.store_scaled_add(246, 703, 702, s.v[8]);
            s.store_add_scaled_inputs(247, 701, s.v[11], 700, s.v[9]);
        }

        if (s.b[1329] && (!(((((s.b[1323] || s.b[1324]) || s.b[1325]) || s.b[1326]) || s.b[1327]) || s.b[1328]))) {
            s.store_add_scaled_inputs(248, 703, s.v[4], 702, s.v[2]);
            s.store_add_scaled_inputs(249, 701, s.v[1], 700, s.v[3]);
            s.store_add_scaled_inputs(246, 703, s.v[10], 702, s.v[8]);
            s.store_add_scaled_inputs(247, 701, s.v[7], 700, s.v[9]);
        }

        if (s.b[1330] && (!((((((s.b[1323] || s.b[1324]) || s.b[1325]) || s.b[1326]) || s.b[1327]) || s.b[1328]) || s.b[1329]))) {
            s.store_add_scaled_inputs(248, 703, s.v[4], 702, s.v[2]);
            s.store_scaled_add(249, 701, 700, s.v[3]);
            s.store_add_scaled_inputs(246, 703, s.v[10], 702, s.v[8]);
            s.store_scaled_add(247, 701, 700, s.v[9]);
        }

        if (s.b[1331] && (!(((((((s.b[1323] || s.b[1324]) || s.b[1325]) || s.b[1326]) || s.b[1327]) || s.b[1328]) || s.b[1329]) || s.b[1330]))) {
            s.store_add_scaled_inputs(248, 703, s.v[4], 702, s.v[2]);
            s.store_add_scaled_inputs(249, 701, s.v[5], 700, s.v[3]);
            s.store_add_scaled_inputs(246, 703, s.v[10], 702, s.v[8]);
            s.store_add_scaled_inputs(247, 701, s.v[11], 700, s.v[9]);
        }

        if (s.b[1332] && (!((((((((s.b[1323] || s.b[1324]) || s.b[1325]) || s.b[1326]) || s.b[1327]) || s.b[1328]) || s.b[1329]) || s.b[1330]) || s.b[1331]))) {
            s.store_scalar(248, (s.v[0] + ((p.p2 - 1.0) * s.v[2])));
            s.store_scalar(249, (p.p2 * s.v[3]));
            s.store_scalar(246, (s.v[6] + ((p.p2 - 1.0) * s.v[8])));
            s.store_scalar(247, (p.p2 * s.v[9]));
        }

        if (s.b[1333] && (!(((((((((s.b[1323] || s.b[1324]) || s.b[1325]) || s.b[1326]) || s.b[1327]) || s.b[1328]) || s.b[1329]) || s.b[1330]) || s.b[1331]) || s.b[1332]))) {
            s.store_scalar(248, (p.p2 * s.v[2]));
            s.store_scalar(249, (s.v[1] + ((p.p2 - 1.0) * s.v[3])));
            s.store_scalar(246, (p.p2 * s.v[8]));
            s.store_scalar(247, (s.v[7] + ((p.p2 - 1.0) * s.v[9])));
        }

        if (!((((((((((s.b[1323] || s.b[1324]) || s.b[1325]) || s.b[1326]) || s.b[1327]) || s.b[1328]) || s.b[1329]) || s.b[1330]) || s.b[1331]) || s.b[1332]) || s.b[1333])) {
            s.store_scalar(248, 0.0);
            s.store_scalar(249, 0.0);
            s.store_scalar(246, 0.0);
            s.store_scalar(247, 0.0);
        }

        s.b[1334] = param_given[24];
        s.v[1334] = if s.b[1334] { 1.0 } else { 0.0 };

        if s.b[1334] {
            s.store_scalar(250, ((p.p24 * p.p53) * p.p52));
        }

        if (!s.b[1334]) {
            s.copy_ad(250, 246);
        }

        s.b[1335] = (s.v[250] < 0.0);
        s.v[1335] = if s.b[1335] { 1.0 } else { 0.0 };

        if s.b[1335] {
            s.store_scalar(250, 0.0);
        }

        s.b[1336] = param_given[25];
        s.v[1336] = if s.b[1336] { 1.0 } else { 0.0 };

        if s.b[1336] {
            s.store_scalar(251, ((p.p25 * p.p53) * p.p52));
        }

        if (!s.b[1336]) {
            s.copy_ad(251, 247);
        }

        s.b[1337] = (s.v[251] < 0.0);
        s.v[1337] = if s.b[1337] { 1.0 } else { 0.0 };

        if s.b[1337] {
            s.store_scalar(251, 0.0);
        }

        s.b[1338] = param_given[26];
        s.v[1338] = if s.b[1338] { 1.0 } else { 0.0 };

        s.b[1339] = (p.p137 == 0.0);
        s.v[1339] = if s.b[1339] { 1.0 } else { 0.0 };

        if (s.b[1338] && s.b[1339]) {
            s.store_scalar(300, (p.p26 * p.p53));
        }

        if (s.b[1338] && (!s.b[1339])) {
            s.store_scalar(300, (((p.p26 * p.p53) - (s.v[35] * p.p2))).max(0.0));
        }

        if (!s.b[1338]) {
            s.copy_ad(300, 248);
        }

        s.b[1340] = (s.v[300] < 0.0);
        s.v[1340] = if s.b[1340] { 1.0 } else { 0.0 };

        if ((!s.b[1338]) && s.b[1340]) {
            s.store_scalar(300, 0.0);
        }

        s.b[1341] = param_given[27];
        s.v[1341] = if s.b[1341] { 1.0 } else { 0.0 };

        s.b[1342] = (p.p137 == 0.0);
        s.v[1342] = if s.b[1342] { 1.0 } else { 0.0 };

        if (s.b[1341] && s.b[1342]) {
            s.store_scalar(301, (p.p27 * p.p53));
        }

        if (s.b[1341] && (!s.b[1342])) {
            s.store_scalar(301, (((p.p27 * p.p53) - (s.v[35] * p.p2))).max(0.0));
        }

        if (!s.b[1341]) {
            s.copy_ad(301, 249);
        }

        s.b[1343] = (s.v[301] < 0.0);
        s.v[1343] = if s.b[1343] { 1.0 } else { 0.0 };

        if ((!s.b[1341]) && s.b[1343]) {
            s.store_scalar(301, 0.0);
        }

        s.store_add_scaled_ad_lhs(341, A::add_scaled_products(s.ad_value(250), s.ad_value(435), 1.0, s.ad_value(300), s.ad_value(436), 1.0), 437, (s.v[35] * p.p2));

        s.b[1344] = (s.v[341] > 0.0);
        s.v[1344] = if s.b[1344] { 1.0 } else { 0.0 };

        if s.b[1344] {
            s.store_scale(343, 393, p.p725);
            s.store_scaled_limited_exp_ad(351, A::div_from_scalar((-p.p731), s.ad_value(343)), p.p733);
            s.store_max_with_scalar_ad(14, A::div_from_scalar(p.p727, s.ad_value(341)), 10.0);
            s.store_sub_ad_lhs(25, A::offset(s.ad_value(14), 1.0), 351);
            s.store_mul_ln_ad_rhs(350, 343, A::max_with_scalar(A::add_scaled_inputs(s.ad_value(25), 0.5, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(25)), 1.0, s.ad_value(351), 4.0)), 0.5), 1e-38));
            s.store_limited_exp_div(12, 350, 343);
            s.store_mul_offset_ad_rhs(349, 341, A::add_scaled_inputs3(s.ad_value(12), 1.0, A::div(s.ad_value(351), s.ad_value(12)), (-1.0), s.ad_value(351), 1.0), (-1.0));
            s.store_div_ad_lhs(348, A::mul(s.ad_value(341), A::add(s.ad_value(12), A::div(s.ad_value(351), s.ad_value(12)))), 343);
        }

        if s.b[1344] {
            let assign15280_ad_e22663: A = {
                if (!(((p.p729 / s.v[341]) - 10.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::div_from_scalar(p.p729, s.ad_value(341)), (-10.0)), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::div_from_scalar(p.p729, s.ad_value(341)), (-10.0)), A::offset(A::div_from_scalar(p.p729, s.ad_value(341)), (-10.0))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((p.p729 / s.v[341]) - 10.0) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::div_from_scalar(p.p729, s.ad_value(341)), (-10.0)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(14, assign15280_ad_e22663, 10.0);
        }

        if s.b[1344] {
            s.store_sub_from_scalar_ad(347, (-p.p731), A::mul(s.ad_value(343), A::ln(A::max_with_scalar(A::scaled_offset(s.ad_value(14), (-1.0), 1.0 / (p.p733)), 1e-38))));
            s.store_scaled_limited_exp_ad(13, A::div_scaled_inputs(A::offset(s.ad_value(347), p.p731), -1.0, s.ad_value(343), 1.0), p.p733);
            s.store_mul_offset_rhs(346, 341, 13, 1.0);
            s.store_div_ad_lhs(345, A::mul_scaled_lhs(s.ad_value(341), -1.0, s.ad_value(13)), 343);
        }

        if (!s.b[1344]) {
            s.store_scalar(343, 0.0);
            s.store_scalar(351, 0.0);
            s.store_scalar(350, 0.0);
            s.store_scalar(349, 0.0);
            s.store_scalar(348, 0.0);
            s.store_scalar(347, 0.0);
            s.store_scalar(346, 0.0);
            s.store_scalar(345, 0.0);
        }

        s.store_add_scaled_ad_lhs(342, A::add_scaled_products(s.ad_value(251), s.ad_value(438), 1.0, s.ad_value(301), s.ad_value(439), 1.0), 440, (s.v[35] * p.p2));

        s.b[1345] = (s.v[342] > 0.0);
        s.v[1345] = if s.b[1345] { 1.0 } else { 0.0 };

        if s.b[1345] {
            s.store_scale(344, 393, p.p726);
            s.store_scaled_limited_exp_ad(358, A::div_from_scalar((-p.p732), s.ad_value(344)), p.p734);
            s.store_max_with_scalar_ad(14, A::div_from_scalar(p.p728, s.ad_value(342)), 10.0);
            s.store_sub_ad_lhs(25, A::offset(s.ad_value(14), 1.0), 358);
            s.store_mul_ln_ad_rhs(357, 344, A::max_with_scalar(A::add_scaled_inputs(s.ad_value(25), 0.5, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(25)), 1.0, s.ad_value(358), 4.0)), 0.5), 1e-38));
            s.store_limited_exp_div(12, 357, 344);
            s.store_mul_offset_ad_rhs(356, 342, A::add_scaled_inputs3(s.ad_value(12), 1.0, A::div(s.ad_value(358), s.ad_value(12)), (-1.0), s.ad_value(358), 1.0), (-1.0));
            s.store_div_ad_lhs(355, A::mul(s.ad_value(342), A::add(s.ad_value(12), A::div(s.ad_value(358), s.ad_value(12)))), 344);
        }

        if s.b[1345] {
            let assign15510_ad_e22914: A = {
                if (!(((p.p730 / s.v[342]) - 10.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::div_from_scalar(p.p730, s.ad_value(342)), (-10.0)), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::div_from_scalar(p.p730, s.ad_value(342)), (-10.0)), A::offset(A::div_from_scalar(p.p730, s.ad_value(342)), (-10.0))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((p.p730 / s.v[342]) - 10.0) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::div_from_scalar(p.p730, s.ad_value(342)), (-10.0)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(14, assign15510_ad_e22914, 10.0);
        }

        if s.b[1345] {
            s.store_sub_from_scalar_ad(354, (-p.p732), A::mul(s.ad_value(344), A::ln(A::max_with_scalar(A::scaled_offset(s.ad_value(14), (-1.0), 1.0 / (p.p734)), 1e-38))));
            s.store_scaled_limited_exp_ad(13, A::div_scaled_inputs(A::offset(s.ad_value(354), p.p732), -1.0, s.ad_value(344), 1.0), p.p734);
            s.store_mul_offset_rhs(353, 342, 13, 1.0);
            s.store_div_ad_lhs(352, A::mul_scaled_lhs(s.ad_value(342), -1.0, s.ad_value(13)), 344);
        }

        if (!s.b[1345]) {
            s.store_scalar(344, 0.0);
            s.store_scalar(358, 0.0);
            s.store_scalar(357, 0.0);
            s.store_scalar(356, 0.0);
            s.store_scalar(355, 0.0);
            s.store_scalar(354, 0.0);
            s.store_scalar(353, 0.0);
            s.store_scalar(352, 0.0);
        }

        s.b[1346] = (((p.p17 > 0.0) && (p.p18 > 0.0)) && ((p.p2 == 1.0) || ((p.p2 > 1.0) && (p.p19 > 0.0))));
        s.v[1346] = if s.b[1346] { 1.0 } else { 0.0 };

        if s.b[1346] {
            s.store_scalar(12, ((s.v[98]) as f64).powf(p.p921));
            s.store_scalar(643, (s.v[100] + p.p914));
            s.store_powf(13, 643, p.p922);
            s.store_ad_value(644, A::add_scaled_inputs3(A::div_from_scalar(p.p918, s.ad_value(12)), 1.0, A::div_from_scalar(p.p919, s.ad_value(13)), 1.0, A::div_from_scalar(p.p920, A::mul(s.ad_value(12), s.ad_value(13))), 1.0));
            s.store_offset(645, 644, 1.0);
            s.store_scalar(12, ((s.v[98]) as f64).powf(p.p927));
            s.store_powf(13, 643, p.p928);
            s.store_ad_value(646, A::add_scaled_inputs3(A::div_from_scalar(p.p924, s.ad_value(12)), 1.0, A::div_from_scalar(p.p925, s.ad_value(13)), 1.0, A::div_from_scalar(p.p926, A::mul(s.ad_value(12), s.ad_value(13))), 1.0));
            s.store_offset(647, 646, 1.0);
            s.store_offset(12, 395, (-1.0));
            s.store_offset_mul_ad(648, s.ad_value(645), A::scale_offset(s.ad_value(12), p.p917, 1.0), 1e-9);
            s.store_scalar(662, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_10(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let mut assign15770_loop_guard: usize = 0;
        while {
            let assign15770_cond_e23123: f64 = if (s.b[1346] && (s.v[662] < p.p2)) { 1.0 } else { 0.0 };
            assign15770_cond_e23123 != 0.0
        } {
            assign15770_loop_guard += 1;
            assert!(assign15770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[1346] {
                s.store_div_from_scalar_offset_scaled_input(12, (1.0 / p.p2), 662, (p.p19 + s.v[99]), (p.p17 + (0.5 * s.v[99])));
                s.store_div_from_scalar_offset_scaled_input(13, (1.0 / p.p2), 662, (p.p19 + s.v[99]), (p.p18 + (0.5 * s.v[99])));
                s.store_offset(649, 12, s.v[649]);
                s.store_offset(650, 13, s.v[650]);
                s.store_offset(662, 662, 1.0);
            }
        }

        if s.b[1346] {
            s.store_scalar(651, (1.0 / (p.p912 + (0.5 * s.v[99]))));
            s.store_scalar(652, (1.0 / (p.p913 + (0.5 * s.v[99]))));
            s.store_add(653, 651, 652);
            s.store_mul_div_from_scalar_lhs(654, p.p915, 648, 653);
            s.store_add(655, 649, 650);
            s.store_mul_div_from_scalar_lhs(656, p.p915, 648, 655);
            s.store_div_ad(657, A::offset(s.ad_value(656), 1.0), A::offset(s.ad_value(654), 1.0));
            s.store_div_ad(658, A::scale_offset(s.ad_value(656), p.p916, 1.0), A::scale_offset(s.ad_value(654), p.p916, 1.0));
            s.store_mul_ad(659, A::div_from_scalar(p.p923, s.ad_value(647)), A::sub(s.ad_value(655), s.ad_value(653)));
            s.store_mul_ad(660, A::div_from_scalar(p.p929, A::powf(s.ad_value(647), p.p930)), A::sub(s.ad_value(655), s.ad_value(653)));
            s.store_mul_ad(661, A::div_from_scalar(p.p931, A::powf(s.ad_value(647), p.p932)), A::sub(s.ad_value(655), s.ad_value(653)));
            s.store_mul(397, 397, 657);
            s.store_mul(409, 409, 658);
            s.store_add(494, 494, 660);
            s.store_add(420, 420, 661);
        }

        s.b[1347] = (p.p37 == 1.0);
        s.v[1347] = if s.b[1347] { 1.0 } else { 0.0 };

        if (s.b[1346] && s.b[1347]) {
            s.store_mul_ad(688, A::div(s.ad_value(625), s.ad_value(647)), A::sub(s.ad_value(655), s.ad_value(653)));
            s.store_mul_ad(689, A::div(s.ad_value(626), A::powf(s.ad_value(647), p.p930)), A::sub(s.ad_value(655), s.ad_value(653)));
            s.store_mul_ad(690, A::div(s.ad_value(627), A::powf(s.ad_value(647), p.p932)), A::sub(s.ad_value(655), s.ad_value(653)));
        }

        if s.b[1346] {
            s.store_add(624, 624, 689);
            s.store_add(616, 616, 690);
        }

        if (!s.b[1346]) {
            s.store_scalar(659, 0.0);
            s.store_scalar(688, 0.0);
        }

        s.b[1348] = (p.p43 == 1.0);
        s.v[1348] = if s.b[1348] { 1.0 } else { 0.0 };

        if s.b[1348] {
            s.store_scalar(668, (p.p1 / p.p2));
            s.store_scalar(669, p.p20);
            s.store_scalar(670, p.p21);
            s.store_scalar(671, p.p22);
        }

        s.b[1349] = (((!param_given[20]) && (!param_given[21])) && (!param_given[22]));
        s.v[1349] = if s.b[1349] { 1.0 } else { 0.0 };

        s.b[1350] = (param_given[23] && (p.p23 > 0.0));
        s.v[1350] = if s.b[1350] { 1.0 } else { 0.0 };

        if ((s.b[1348] && s.b[1349]) && s.b[1350]) {
            s.store_offset(13, 668, p.p23);
            s.store_scalar(14, (1.0 / p.p947));
            s.store_div_from_scalar_scaled_input(669, (p.p947 * p.p947), 13, p.p23);
            s.store_div_ad_lhs(670, A::add_scaled_product(A::limited_exp_scaled_input(s.ad_value(14), ((-10.0) * p.p23)), ((0.1 * p.p23) + (0.01 * p.p947)), A::scale_offset(s.ad_value(13), 0.1, (0.01 * p.p947)), A::limited_exp(A::mul_scaled_lhs(s.ad_value(13), (-10.0), s.ad_value(14))), (-1.0)), 668);
            s.store_div_ad_lhs(671, A::add_scaled_product(A::limited_exp_scaled_input(s.ad_value(14), ((-20.0) * p.p23)), ((0.05 * p.p23) + (0.0025 * p.p947)), A::scale_offset(s.ad_value(13), 0.05, (0.0025 * p.p947)), A::limited_exp(A::mul_scaled_lhs(s.ad_value(13), (-20.0), s.ad_value(14))), (-1.0)), 668);
        }

        s.store_mul_ad_rhs(663, 578, A::add_scaled_inputs3(s.ad_value(669), 1.0, s.ad_value(670), p.p933, s.ad_value(671), p.p934));

        s.store_mul_ad_rhs(664, 579, A::add_scaled_inputs3(s.ad_value(669), 1.0, s.ad_value(670), p.p933, s.ad_value(671), p.p934));

        s.store_mul_ad_rhs(665, 630, A::add_scaled_inputs3(s.ad_value(669), 1.0, s.ad_value(670), p.p933, s.ad_value(671), p.p934));

        s.store_mul_ad_rhs(666, 629, A::add_scaled_inputs3(s.ad_value(669), 1.0, s.ad_value(670), p.p933, s.ad_value(671), p.p934));

        s.store_offset_mul_ad(667, s.ad_value(580), A::add_scaled_inputs3(s.ad_value(669), 1.0, s.ad_value(670), p.p933, s.ad_value(671), p.p934), 1.0);

        s.store_mul(397, 397, 667);

        s.store_add(494, 494, 664);

        s.store_mul_voltage_ad(64, s.ad_value(187), ctx, nodes, Some(9), Some(11));

        s.store_mul_voltage_ad(66, s.ad_value(187), ctx, nodes, Some(5), Some(11));

        s.store_mul_voltage_ad(70, s.ad_value(187), ctx, nodes, Some(7), Some(11));

        s.store_sub(74, 66, 70);

        s.copy_ad(68, 66);

        s.copy_ad(56, 74);

        s.copy_ad(50, 70);

        s.copy_ad(48, 66);

        s.store_mul_voltage_ad(306, s.ad_value(187), ctx, nodes, Some(12), Some(7));

        s.store_mul_voltage_ad(307, s.ad_value(187), ctx, nodes, Some(13), Some(5));

        s.store_mul_voltage_ad(308, s.ad_value(187), ctx, nodes, Some(13), Some(5));

        s.store_mul_voltage_ad(309, s.ad_value(187), ctx, nodes, Some(13), Some(14));

        s.store_sub(54, 64, 66);

        s.store_sub(52, 64, 70);

        s.store_mul_voltage_ad(230, s.ad_value(187), ctx, nodes, Some(10), Some(5));

        s.store_mul_voltage_ad(231, s.ad_value(187), ctx, nodes, Some(10), Some(7));

        s.copy_ad(232, 230);

        s.b[1351] = ((((p.p1110 != 0.0) && (p.p42 == 1.0)) && (p.p1095 == 1.0)) && (p.p1094 == 1.0));
        s.v[1351] = if s.b[1351] { 1.0 } else { 0.0 };

        if s.b[1351] {
            s.store_ad_value(68, A::add_scaled_product(s.ad_value(66), 1.0, s.ad_value(187), A::voltage(ctx, nodes, Some(6), Some(5)), (1.0 - (p.p1111 / p.p1110))));
            s.store_ad_value(308, A::add_scaled_inputs3(s.ad_value(307), 1.0, s.ad_value(66), 1.0, s.ad_value(68), -1.0));
            s.store_ad_value(232, A::add_scaled_inputs3(s.ad_value(230), 1.0, s.ad_value(66), 1.0, s.ad_value(68), -1.0));
        }

        s.copy_ad(69, 68);

        s.store_mul_voltage_ad(72, s.ad_value(187), ctx, nodes, Some(7), Some(11));

        s.v[57] = 1.0;

        s.b[1352] = (s.v[74] < 0.0);
        s.v[1352] = if s.b[1352] { 1.0 } else { 0.0 };

        if s.b[1352] {
            s.store_scalar(57, (-1.0));
            s.store_mul_voltage_ad(66, s.ad_value(187), ctx, nodes, Some(7), Some(11));
            s.store_mul_voltage_ad(70, s.ad_value(187), ctx, nodes, Some(5), Some(11));
            s.copy_ad(72, 69);
            s.store_mul_voltage_ad(68, s.ad_value(187), ctx, nodes, Some(7), Some(11));
        }

        s.store_sub(74, 66, 70);

        s.store_sub(75, 68, 72);

        s.store_scale(12, 75, p.p956);

        if ((!(s.v[12] > 37.0)) && (!(s.v[12] < (-37.0)))) {
            s.store_ln_one_plus_exp(13, 12);
        } else {
            if ((!(s.v[12] > 37.0)) && (s.v[12] < (-37.0))) {
                s.store_exp(13, 12);
            } else {
                if (s.v[12] > 37.0) {
                    s.copy_ad(13, 12);
                } else {
                    s.store_scalar(13, 0.0);
                }
            }
        }

        s.store_offset_ad(76, A::sub_scaled_inputs(s.ad_value(13), (2.0 / p.p956), s.ad_value(75), 1.0), (-((2.0 / p.p956) * ((2.0) as f64).ln())));

        s.store_neg_ad(62, A::add_scaled_inputs3(s.ad_value(72), 1.0, s.ad_value(75), 0.5, s.ad_value(76), (-0.5)));

        s.store_scale(12, 74, p.p956);

        if ((!(s.v[12] > 37.0)) && (!(s.v[12] < (-37.0)))) {
            s.store_ln_one_plus_exp(13, 12);
        } else {
            if ((!(s.v[12] > 37.0)) && (s.v[12] < (-37.0))) {
                s.store_exp(13, 12);
            } else {
                if (s.v[12] > 37.0) {
                    s.copy_ad(13, 12);
                } else {
                    s.store_scalar(13, 0.0);
                }
            }
        }

        s.store_offset_ad(76, A::sub_scaled_inputs(s.ad_value(13), (2.0 / p.p956), s.ad_value(74), 1.0), (-((2.0 / p.p956) * ((2.0) as f64).ln())));

        s.store_neg_ad(61, A::add_scaled_inputs3(s.ad_value(70), 1.0, s.ad_value(74), 0.5, s.ad_value(76), (-0.5)));

        s.store_tanh_ad(12, A::div_scaled_inputs(s.ad_value(56), p.p1123, s.ad_value(393), 1.0));

        s.store_offset_scaled(102, 12, 0.5, 0.5);

        s.store_sub_from_scalar(103, 1.0, 102);

        s.b[1353] = (p.p44 != 0.0);
        s.v[1353] = if s.b[1353] { 1.0 } else { 0.0 };

        if s.b[1353] {
            s.store_ad_value(486, A::add_scaled_products(s.ad_value(485), s.ad_value(103), 1.0, s.ad_value(484), s.ad_value(102), 1.0));
            s.store_ad_value(492, A::add_scaled_products(s.ad_value(421), s.ad_value(103), 1.0, s.ad_value(420), s.ad_value(102), 1.0));
            s.store_ad_value(519, A::add_scaled_products(s.ad_value(518), s.ad_value(103), 1.0, s.ad_value(517), s.ad_value(102), 1.0));
            s.store_ad_value(541, A::add_scaled_products(s.ad_value(540), s.ad_value(103), 1.0, s.ad_value(539), s.ad_value(102), 1.0));
            s.store_ad_value(166, A::add_scaled_products(s.ad_value(165), s.ad_value(103), 1.0, s.ad_value(164), s.ad_value(102), 1.0));
            s.store_ad_value(502, A::add_scaled_products(s.ad_value(410), s.ad_value(103), 1.0, s.ad_value(409), s.ad_value(102), 1.0));
            s.store_ad_value(536, A::add_scaled_products(s.ad_value(414), s.ad_value(103), 1.0, s.ad_value(413), s.ad_value(102), 1.0));
            s.store_ad_value(499, A::add_scaled_products(s.ad_value(398), s.ad_value(103), 1.0, s.ad_value(397), s.ad_value(102), 1.0));
            s.store_ad_value(506, A::add_scaled_products(s.ad_value(400), s.ad_value(103), 1.0, s.ad_value(399), s.ad_value(102), 1.0));
            s.store_ad_value(516, A::add_scaled_products(s.ad_value(402), s.ad_value(103), 1.0, s.ad_value(401), s.ad_value(102), 1.0));
            s.store_ad_value(510, A::add_scaled_products(s.ad_value(404), s.ad_value(103), 1.0, s.ad_value(403), s.ad_value(102), 1.0));
            s.store_ad_value(513, A::add_scaled_products(s.ad_value(406), s.ad_value(103), 1.0, s.ad_value(405), s.ad_value(102), 1.0));
            s.store_ad_value(553, A::add_scaled_products(s.ad_value(552), s.ad_value(103), 1.0, s.ad_value(551), s.ad_value(102), 1.0));
            s.store_ad_value(558, A::add_scaled_products(s.ad_value(416), s.ad_value(103), 1.0, s.ad_value(415), s.ad_value(102), 1.0));
        }

        if (!s.b[1353]) {
            s.copy_ad(486, 484);
            s.copy_ad(492, 420);
            s.copy_ad(519, 517);
            s.copy_ad(541, 539);
            s.copy_ad(166, 164);
            s.copy_ad(502, 409);
            s.copy_ad(536, 413);
            s.copy_ad(499, 397);
            s.copy_ad(506, 399);
            s.copy_ad(516, 401);
            s.copy_ad(510, 403);
            s.copy_ad(513, 405);
            s.copy_ad(553, 551);
            s.copy_ad(558, 415);
        }

        s.b[1354] = ((0.05 == 0.0) && ((s.v[127] - s.v[61]) < ((-2500.0) * 0.1)));
        s.v[1354] = if s.b[1354] { 1.0 } else { 0.0 };

        if s.b[1354] {
            s.store_div_from_scalar_ad(110, ((-0.1) * 0.1), A::sub_scaled_inputs(s.ad_value(127), 16.0, s.ad_value(61), 16.0));
        }

        if (!s.b[1354]) {
            s.store_scaled_add_ad(110, A::offset(A::sub(s.ad_value(127), s.ad_value(61)), 0.05), A::sqrt(A::offset(A::mul(A::offset(A::sub(s.ad_value(127), s.ad_value(61)), (-0.05)), A::offset(A::sub(s.ad_value(127), s.ad_value(61)), (-0.05))), ((0.25 * 0.1) * 0.1))), 0.5);
        }

        s.store_sqrt(111, 110);

        s.store_mul(112, 114, 111);

        s.store_div_from_scalar(97, s.v[26], 112);

        s.store_ad_value(113, A::add_scaled_product(A::add_scaled_product(A::add(s.ad_value(483), s.ad_value(422)), 1.0, s.ad_value(486), s.ad_value(76), 1.0), 1.0, s.ad_value(487), s.ad_value(61), (-1.0)));

        s.store_offset_scaled(13, 113, 1.0 / (s.v[46]), 1.0);

        s.b[1355] = ((1.0 == 0.0) && (s.v[13] < ((-2500.0) * 0.05)));
        s.v[1355] = if s.b[1355] { 1.0 } else { 0.0 };

        if s.b[1355] {
            s.store_div_from_scalar_scaled_input(104, ((-0.05) * 0.05), 13, 16.0);
        }

        if (!s.b[1355]) {
            s.store_scaled_add_ad(104, A::offset(s.ad_value(13), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(13), (-1.0)), A::offset(s.ad_value(13), (-1.0))), ((0.25 * 0.05) * 0.05))), 0.5);
        }

        s.store_mul(106, 104, 108);

        s.store_div_from_scalar(107, 1.0, 106);

        s.store_mul_neg_ad_lhs(123, A::add_scaled_product(s.ad_value(492), 1.0, s.ad_value(493), s.ad_value(61), 1.0), 76);

        s.store_offset_ad(123, A::sub_scaled_inputs(s.ad_value(123), 0.5, A::sqrt(A::offset(A::mul(s.ad_value(123), s.ad_value(123)), ((0.25 * 0.005) * 0.005))), 0.5), (0.25 * 0.005));

        s.store_mul_ad(124, A::add_scaled_product(A::offset(s.ad_value(454), (p.p869 / s.v[30])), 1.0, s.ad_value(455), s.ad_value(61), 1.0), A::offset(A::powf(s.ad_value(395), p.p868), (-1.0)));

        s.b[1356] = (s.v[116] > 0.0);
        s.v[1356] = if s.b[1356] { 1.0 } else { 0.0 };

        if s.b[1356] {
            s.store_mul_neg_lhs(12, 117, 76);
        }

        s.b[1357] = (s.v[12] < (-80.0));
        s.v[1357] = if s.b[1357] { 1.0 } else { 0.0 };

        if (s.b[1356] && s.b[1357]) {
            s.store_scalar(14, 1.804851387e-35);
        }

        if (s.b[1356] && (!s.b[1357])) {
            s.store_limited_exp(14, 12);
        }

        if s.b[1356] {
            s.store_offset_mul_ad(15, s.ad_value(116), A::offset(s.ad_value(14), 1.0), s.v[30]);
            s.store_mul_scaled_ad_rhs(115, 106, -1.0, A::ln(A::max_with_scalar(A::div_from_scalar(s.v[30], s.ad_value(15)), 1e-38)));
        }

        if (!s.b[1356]) {
            s.store_scalar(115, 0.0);
        }

        s.store_add_ad_rhs(16, 121, A::div(s.ad_value(118), A::pow_from_scalar(s.v[30], s.ad_value(119))));

        s.store_ad_value(115, A::add_scaled_product(s.ad_value(115), 1.0, s.ad_value(16), A::tanh(A::mul(s.ad_value(120), s.ad_value(76))), (-1.0)));

        s.store_offset(482, 482, p.p35);

        s.store_mul(65, 64, 107);

        s.store_mul(73, 70, 107);

        s.store_mul(58, 482, 107);

        s.store_ad_value(122, A::add_scaled_products(s.ad_value(495), A::sub(s.ad_value(111), s.ad_value(128)), 1.0, s.ad_value(494), s.ad_value(61), (-1.0)));

        s.store_add_ad_lhs(79, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(123), 1.0, s.ad_value(115), 1.0, s.ad_value(122), 1.0), 1.0, s.ad_value(124), (-1.0), s.ad_value(659), 1.0), 663);

        s.store_ad_value(59, A::add_scaled_product(A::sub(s.ad_value(65), s.ad_value(58)), 1.0, s.ad_value(79), s.ad_value(107), (-1.0)));

        s.store_scaled_sqrt_ad(125, A::mul_scaled_lhs(s.ad_value(481), ((2.0 * 1.60219e-19) * s.v[26]), s.ad_value(109)), 1.0 / (s.v[46]));

        s.v[710] = 0.5;

    }

    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!(((2.0 * s.v[88]) + (s.v[70] * s.v[109])) < ((-10000.0) * 0.001))) {
            s.store_scaled_add_ad(12, A::add_scaled_product(s.ad_value(88), 2.0, s.ad_value(70), s.ad_value(109), 1.0), A::sqrt(A::offset(A::mul(A::add_scaled_product(s.ad_value(88), 2.0, s.ad_value(70), s.ad_value(109), 1.0), A::add_scaled_product(s.ad_value(88), 2.0, s.ad_value(70), s.ad_value(109), 1.0)), ((4.0 * 0.001) * 0.001))), 0.5);
        } else {
            if (((2.0 * s.v[88]) + (s.v[70] * s.v[109])) < ((-10000.0) * 0.001)) {
                s.store_div_from_scalar_ad(12, ((-0.001) * 0.001), A::add_scaled_product(s.ad_value(88), 2.0, s.ad_value(70), s.ad_value(109), 1.0));
            } else {
                s.store_scalar(12, 0.0);
            }
        }

        s.store_offset_ad(90, A::div_scaled_inputs(s.ad_value(125), 1.0, A::sqrt(s.ad_value(12)), 2.0), 1.0);

        if (!((((((s.v[70] * s.v[109]) + (2.0 * s.v[88])) + (((s.v[710]).max(1e-38)) as f64).ln()) + (2.0 * s.v[710])) + ((((((2.0 * s.v[90]) / s.v[125]) * ((((2.0 * s.v[710]) * s.v[90]) / s.v[125]) + (2.0 * ((s.v[12]) as f64).sqrt())))).max(1e-38)) as f64).ln()) < ((-10000.0) * 0.001))) {
            let assign17290_ad_e24521: A = A::add(A::offset(A::add_scaled_product(s.ad_value(88), 2.0, s.ad_value(70), s.ad_value(109), 1.0), (((((s.v[710]).max(1e-38)) as f64).ln()) + ((2.0 * s.v[710])))), A::ln(A::max_with_scalar(A::mul(A::div_scaled_inputs(s.ad_value(90), 2.0, s.ad_value(125), 1.0), A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(90), (2.0 * s.v[710]), s.ad_value(125), 1.0), 1.0, A::sqrt(s.ad_value(12)), 2.0)), 1e-38)));
            let assign17290_ad_e24559: A = A::add(A::offset(A::add_scaled_product(s.ad_value(88), 2.0, s.ad_value(70), s.ad_value(109), 1.0), (((((s.v[710]).max(1e-38)) as f64).ln()) + ((2.0 * s.v[710])))), A::ln(A::max_with_scalar(A::mul(A::div_scaled_inputs(s.ad_value(90), 2.0, s.ad_value(125), 1.0), A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(90), (2.0 * s.v[710]), s.ad_value(125), 1.0), 1.0, A::sqrt(s.ad_value(12)), 2.0)), 1e-38)));
            let assign17290_ad_e24597: A = A::add(A::offset(A::add_scaled_product(s.ad_value(88), 2.0, s.ad_value(70), s.ad_value(109), 1.0), (((((s.v[710]).max(1e-38)) as f64).ln()) + ((2.0 * s.v[710])))), A::ln(A::max_with_scalar(A::mul(A::div_scaled_inputs(s.ad_value(90), 2.0, s.ad_value(125), 1.0), A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(90), (2.0 * s.v[710]), s.ad_value(125), 1.0), 1.0, A::sqrt(s.ad_value(12)), 2.0)), 1e-38)));
            s.store_scaled_add_ad(711, assign17290_ad_e24521, A::sqrt(A::offset(A::mul(assign17290_ad_e24559, assign17290_ad_e24597), ((4.0 * 0.001) * 0.001))), 0.5);
        } else {
            if ((((((s.v[70] * s.v[109]) + (2.0 * s.v[88])) + (((s.v[710]).max(1e-38)) as f64).ln()) + (2.0 * s.v[710])) + ((((((2.0 * s.v[90]) / s.v[125]) * ((((2.0 * s.v[710]) * s.v[90]) / s.v[125]) + (2.0 * ((s.v[12]) as f64).sqrt())))).max(1e-38)) as f64).ln()) < ((-10000.0) * 0.001)) {
                let assign17290_ad_e24692: A = A::add(A::offset(A::add_scaled_product(s.ad_value(88), 2.0, s.ad_value(70), s.ad_value(109), 1.0), (((((s.v[710]).max(1e-38)) as f64).ln()) + ((2.0 * s.v[710])))), A::ln(A::max_with_scalar(A::mul(A::div_scaled_inputs(s.ad_value(90), 2.0, s.ad_value(125), 1.0), A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(90), (2.0 * s.v[710]), s.ad_value(125), 1.0), 1.0, A::sqrt(s.ad_value(12)), 2.0)), 1e-38)));
                s.store_div_from_scalar_ad(711, ((-0.001) * 0.001), assign17290_ad_e24692);
            } else {
                s.store_scalar(711, 0.0);
            }
        }

        s.store_mul_ad_rhs(857, 187, A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(482), 1.0, A::add_scaled_product(s.ad_value(711), 1.0, s.ad_value(70), s.ad_value(109), (-1.0)), s.ad_value(108), 1.0), 1.0, A::mul3(s.ad_value(108), s.ad_value(125), A::sqrt(s.ad_value(711))), 1.0, s.ad_value(79), 1.0));

        s.store_scaled_sqrt_ad(125, A::mul_scaled_lhs(s.ad_value(481), ((2.0 * 1.60219e-19) * s.v[26]), s.ad_value(107)), 1.0 / (s.v[46]));

        s.store_div_from_scalar(126, 1.0, 125);

        s.store_div(89, 88, 104);

        s.v[13] = 1.0;

        s.store_scale(204, 59, 1.0 / (s.v[13]));

        s.store_scale(205, 125, 1.0 / (s.v[13]));

        s.store_sub_scaled_ad_rhs(13, 204, 0.5, A::scale_offset(s.ad_value(205), ((0.7071067811865475) * (3.0)), 3.0));

        s.store_add_ad_rhs(14, 13, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(13)), 1.0, s.ad_value(204), 6.0)));

        s.b[1358] = (s.v[204] < 0.0);
        s.v[1358] = if s.b[1358] { 1.0 } else { 0.0 };

        if s.b[1358] {
            s.store_div_ad_lhs(15, A::sub(s.ad_value(204), s.ad_value(14)), 205);
            s.store_neg_ad(91, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));
        }

        if (!s.b[1358]) {
            s.store_limited_exp_neg_input(15, 14);
            s.store_scale(13, 205, 0.5);
            s.store_sub_ad_lhs(14, A::sqrt(A::add_scaled_inputs3(A::offset(s.ad_value(204), (-1.0)), 1.0, s.ad_value(15), 1.0, A::square(s.ad_value(13)), 1.0)), 13);
            s.store_sub_ad_lhs(91, A::offset(A::square(s.ad_value(14)), 1.0), 15);
        }

        s.store_scaled_add_ad(20, A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(91), (-1.0)), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);

        s.store_sqrt(96, 20);

        s.store_div_ad_lhs(12, A::offset(A::div_scaled_inputs(s.ad_value(125), 1.0, s.ad_value(96), 2.0), 1.0), 125);

        s.store_ad_value(13, A::add_scaled_inputs3(s.ad_value(91), 1.0, s.ad_value(89), (-2.0), s.ad_value(73), -1.0));

        s.store_sub_ad_rhs(14, 13, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));

        s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul(s.ad_value(14), A::offset(s.ad_value(14), 0.402982)), 2.446562)), 0.5);

        s.copy_ad(94, 96);

        s.b[1359] = (s.v[20] <= (-68.0));
        s.v[1359] = if s.b[1359] { 1.0 } else { 0.0 };

        if s.b[1359] {
            s.store_scalar(16, (-100.0));
            s.store_scalar(17, 20.0);
        }

        s.b[1360] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));
        s.v[1360] = if s.b[1360] { 1.0 } else { 0.0 };

        if (s.b[1359] && s.b[1360]) {
            s.store_limited_exp(15, 16);
        }

        s.b[1361] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));
        s.v[1361] = if s.b[1361] { 1.0 } else { 0.0 };

        if ((s.b[1359] && (!s.b[1360])) && s.b[1361]) {
            s.store_limited_exp(15, 20);
        }

        if ((s.b[1359] && (!s.b[1360])) && (!s.b[1361])) {
            s.store_div_ad_lhs(14, A::sub(s.ad_value(20), s.ad_value(16)), 17);
            s.store_square(18, 14);
            s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add_scaled_product(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), 1.0, s.ad_value(18), A::sub_from_scalar((15.0 / 16.0), A::mul(s.ad_value(18), A::sub_from_scalar(1.25, s.ad_value(18)))), 1.0), 1.0));
        }

        if s.b[1359] {
            s.store_mul_ad_rhs(200, 15, A::add_scaled_inputs3(A::offset(s.ad_value(13), 1.0), 1.0, s.ad_value(20), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), -1.0));
        }

        if (!s.b[1359]) {
            s.store_limited_exp(15, 20);
            s.store_div_from_scalar(95, 1.0, 94);
            s.store_ad_value(16, A::add_scaled_inputs3(s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(13), -1.0));
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0)));
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
            s.store_ad_value(16, A::add_scaled_inputs3(s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(13), -1.0));
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0)));
            s.store_scaled_mul_ad(18, A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0)), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0)), 1.0);
            s.store_sub_ad_lhs(19, A::add_scaled_product(A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-1.0)), 18);
            s.store_ad_value(200, A::add_scaled_product(s.ad_value(15), 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::offset(A::div(A::mul(s.ad_value(16), s.ad_value(19)), A::mul_scaled_lhs(s.ad_value(17), 2.0, s.ad_value(17))), 1.0), (-1.0)));
        }

        s.b[1362] = ((1.0 == 0.0) && (s.v[91] < ((-2500.0) * 2.0)));
        s.v[1362] = if s.b[1362] { 1.0 } else { 0.0 };

        if s.b[1362] {
            s.store_div_from_scalar_scaled_input(93, ((-2.0) * 2.0), 91, 16.0);
        }

        if (!s.b[1362]) {
            s.store_scaled_add_ad(93, A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(91), (-1.0)), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        s.store_sqrt(96, 93);

        s.store_sub_scaled_inputs(92, 91, 1.0, 200, 2.0);

        s.b[1363] = ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0)));
        s.v[1363] = if s.b[1363] { 1.0 } else { 0.0 };

        if s.b[1363] {
            s.store_div_from_scalar_scaled_input(12, ((-2.0) * 2.0), 92, 16.0);
        }

        if (!s.b[1363]) {
            s.store_scaled_add_ad(12, A::offset(s.ad_value(92), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(92), (-1.0)), A::offset(s.ad_value(92), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        s.store_offset_div_ad(90, s.ad_value(125), A::add(s.ad_value(96), A::sqrt(s.ad_value(12))), 1.0);

        s.v[155] = (1e-8 / (s.v[47] * p.p77));

        s.store_mul_ad_rhs(12, 106, A::add_scaled_product(A::sub(s.ad_value(59), s.ad_value(91)), 1.0, s.ad_value(200), A::offset(s.ad_value(90), (-1.0)), (-2.0)));

        s.b[1364] = ((0.0 == 0.0) && (s.v[12] < ((-2500.0) * 0.1)));
        s.v[1364] = if s.b[1364] { 1.0 } else { 0.0 };

        if s.b[1364] {
            s.store_div_from_scalar_scaled_input(84, ((-0.1) * 0.1), 12, 16.0);
        }

        if (!s.b[1364]) {
            s.store_scaled_add_ad_rhs(84, 12, A::sqrt(A::offset(A::mul(s.ad_value(12), s.ad_value(12)), ((0.25 * 0.1) * 0.1))), 0.5);
        }

        s.store_mul3_affine_lhs(130, 90, 106, 2.0, 0.0, 200);

        s.store_add_scaled_inputs(132, 84, s.v[155], 130, (s.v[158] * s.v[155]));

        s.store_pow_ad(14, A::scaled_offset(A::div(s.ad_value(130), s.ad_value(84)), 1.0, 0.5), s.ad_value(513));

        s.store_ad_value(15, A::add_scaled_product(A::div(s.ad_value(510), s.ad_value(14)), 1.0, A::add_scaled_product(s.ad_value(506), 1.0, s.ad_value(516), s.ad_value(61), 1.0), A::pow(s.ad_value(132), s.ad_value(407)), 1.0));

        s.store_offset(16, 15, 1.0);

        s.b[1365] = ((1.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.0015)));
        s.v[1365] = if s.b[1365] { 1.0 } else { 0.0 };

        if s.b[1365] {
            s.store_div_from_scalar_scaled_input(133, ((-0.0015) * 0.0015), 16, 16.0);
        }

        if (!s.b[1365]) {
            s.store_scaled_add_ad(133, A::offset(s.ad_value(16), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(16), (-1.0)), A::offset(s.ad_value(16), (-1.0))), ((0.25 * 0.0015) * 0.0015))), 0.5);
        }

        s.store_div_from_scalar_scaled_ad(235, 1.0, A::pow_from_scalar((s.v[29] * 1000000.0), s.ad_value(527)), p.p2);

        s.b[1366] = (p.p42 == 1.0);
        s.v[1366] = if s.b[1366] { 1.0 } else { 0.0 };

        if s.b[1366] {
            s.store_scalar(243, 0.0);
        }

        if (!s.b[1366]) {
            s.store_offset_mul(12, 526, 130, 1.0);
            s.store_mul_sub_rhs(13, 543, 111, 128);
            s.store_add_ad_lhs(14, A::div_from_scalar(1.0, s.ad_value(12)), 13);
            s.store_add_ad_rhs(15, 14, A::sqrt(A::offset(A::square(s.ad_value(14)), 0.01)));
        }

        s.b[1367] = (p.p42 == 0.0);
        s.v[1367] = if s.b[1367] { 1.0 } else { 0.0 };

        if ((!s.b[1366]) && s.b[1367]) {
            s.store_mul_ad_affine_product_lhs(243, A::add_scaled_product(s.ad_value(533), 1.0, s.ad_value(532), s.ad_value(15), 1.0), s.ad_value(235), p.p2, 0.0, 408);
        }

        if ((!s.b[1366]) && (!s.b[1367])) {
            s.store_mul_add_ad_lhs(243, A::add_scaled_product(s.ad_value(239), 1.0, A::add_scaled_product(s.ad_value(533), 1.0, s.ad_value(532), s.ad_value(15), 1.0), s.ad_value(235), p.p2), s.ad_value(240), 408);
        }

        s.store_pow_ad(12, s.ad_value(133), A::div_from_scalar(1.0, s.ad_value(166)));

        s.store_mul(23, 453, 61);

        s.store_sqrt_square_offset(24, 23, 0.1);

        s.store_scaled_add_ad(13, A::sub_from_scalar(1.0, s.ad_value(23)), A::sqrt(A::add_scaled_product(s.ad_value(24), 1.0, A::sub_from_scalar(1.0, s.ad_value(23)), A::sub_from_scalar(1.0, s.ad_value(23)), 1.0)), 0.5);

        s.store_div_ad(14, A::mul_scaled_lhs(s.ad_value(200), (10.0 * p.p433), s.ad_value(13)), A::offset(A::mul(s.ad_value(200), s.ad_value(13)), (10.0 * p.p433)));

        s.b[1368] = (s.v[536] < 0.0);
        s.v[1368] = if s.b[1368] { 1.0 } else { 0.0 };

        if s.b[1368] {
            s.store_scaled_mul_ad(138, A::div_scaled_inputs(A::mul(A::div(s.ad_value(499), s.ad_value(12)), s.ad_value(106)), 1.0, s.ad_value(502), s.v[30]), A::div_from_scalar(1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(536), s.ad_value(14)))), 2.0);
        }

        if (!s.b[1368]) {
            s.store_scaled_mul_ad(138, A::div_scaled_inputs(A::mul(A::div(s.ad_value(499), s.ad_value(12)), s.ad_value(106)), 1.0, s.ad_value(502), s.v[30]), A::offset(A::mul(s.ad_value(536), s.ad_value(14)), 1.0), 2.0);
        }

        s.b[1369] = (s.v[243] > 0.0);
        s.v[1369] = if s.b[1369] { 1.0 } else { 0.0 };

        if s.b[1369] {
            s.store_mul3_affine_lhs(23, 90, 106, ((s.v[29] * 2.0) * s.v[46]), 0.0, 502);
            s.store_ad_value(24, A::div_scaled_inputs(A::mul3(s.ad_value(23), s.ad_value(138), s.ad_value(243)), 1.0, s.ad_value(106), 2.0));
            s.store_div_ad(12, A::mul_scaled_lhs(s.ad_value(138), 0.5, A::add(A::square(s.ad_value(200)), s.ad_value(200))), A::offset(A::mul_scaled_lhs(s.ad_value(138), 0.5, A::offset(s.ad_value(200), 1.0)), 1.0));
            s.store_mul_scaled_ad_rhs(13, 138, 2.0, A::sub(s.ad_value(200), s.ad_value(12)));
            s.store_sqrt_square_offset(14, 13, 1.0);
        }

        s.b[1370] = (s.v[13] != 0.0);
        s.v[1370] = if s.b[1370] { 1.0 } else { 0.0 };

        if (s.b[1369] && s.b[1370]) {
            s.store_asinh(147, 13);
            s.store_ad_value(15, A::add_scaled_product(s.ad_value(14), 1.0, A::div_from_scalar(1.0, s.ad_value(13)), s.ad_value(147), 1.0));
        }

        if (s.b[1369] && (!s.b[1370])) {
            s.store_add_ad_rhs(15, 14, A::div_from_scalar(1.0, s.ad_value(14)));
        }

        if s.b[1369] {
            s.store_ad_value(16, A::add_scaled_product(A::add_scaled_product(A::mul3(s.ad_value(24), s.ad_value(12), A::offset(A::add(s.ad_value(200), s.ad_value(12)), 1.0)), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0, s.ad_value(138), A::sub(A::add(A::square(s.ad_value(200)), s.ad_value(200)), A::add(A::square(s.ad_value(12)), s.ad_value(12))), (-1.0)));
        }

        s.b[1371] = (s.v[13] != 0.0);
        s.v[1371] = if s.b[1371] { 1.0 } else { 0.0 };

        if (s.b[1369] && s.b[1371]) {
            s.store_div_ad(17, A::mul_scaled_lhs(s.ad_value(138), (-2.0), A::add_scaled_product(s.ad_value(147), (-1.0), s.ad_value(13), s.ad_value(14), 1.0)), A::square(s.ad_value(13)));
        }

        if (s.b[1369] && (!s.b[1371])) {
            s.store_mul_scaled_ad_rhs(17, 138, (-2.0), A::div(s.ad_value(13), s.ad_value(14)));
        }

        if s.b[1369] {
            s.store_ad_value(18, A::add_scaled_product(A::add_scaled_product(A::add_scaled_product(s.ad_value(15), 1.0, s.ad_value(12), s.ad_value(17), 1.0), 1.0, s.ad_value(24), A::offset(A::add_scaled_inputs(s.ad_value(200), 1.0, s.ad_value(12), 2.0), 1.0), 1.0), 1.0, s.ad_value(138), A::scale_offset(s.ad_value(12), 2.0, 1.0), 1.0));
            s.store_sub_ad_rhs(12, 12, A::div(s.ad_value(16), s.ad_value(18)));
            s.store_mul_scaled_ad_rhs(13, 138, 2.0, A::sub(s.ad_value(200), s.ad_value(12)));
            s.store_sqrt_square_offset(14, 13, 1.0);
        }

        s.b[1372] = (s.v[13] != 0.0);
        s.v[1372] = if s.b[1372] { 1.0 } else { 0.0 };

        if (s.b[1369] && s.b[1372]) {
            s.store_asinh(147, 13);
            s.store_ad_value(15, A::add_scaled_product(s.ad_value(14), 1.0, A::div_from_scalar(1.0, s.ad_value(13)), s.ad_value(147), 1.0));
        }

        if (s.b[1369] && (!s.b[1372])) {
            s.store_add_ad_rhs(15, 14, A::div_from_scalar(1.0, s.ad_value(14)));
        }

        if s.b[1369] {
            s.store_ad_value(16, A::add_scaled_product(A::add_scaled_product(A::mul3(s.ad_value(24), s.ad_value(12), A::offset(A::add(s.ad_value(200), s.ad_value(12)), 1.0)), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0, s.ad_value(138), A::sub(A::add(A::square(s.ad_value(200)), s.ad_value(200)), A::add(A::square(s.ad_value(12)), s.ad_value(12))), (-1.0)));
        }

        s.b[1373] = (s.v[13] != 0.0);
        s.v[1373] = if s.b[1373] { 1.0 } else { 0.0 };

        if (s.b[1369] && s.b[1373]) {
            s.store_div_ad(17, A::mul_scaled_lhs(s.ad_value(138), (-2.0), A::add_scaled_product(s.ad_value(147), (-1.0), s.ad_value(13), s.ad_value(14), 1.0)), A::square(s.ad_value(13)));
        }

        if (s.b[1369] && (!s.b[1373])) {
            s.store_mul_scaled_ad_rhs(17, 138, (-2.0), A::div(s.ad_value(13), s.ad_value(14)));
        }

        if s.b[1369] {
            s.store_ad_value(18, A::add_scaled_product(A::add_scaled_product(A::add_scaled_product(s.ad_value(15), 1.0, s.ad_value(12), s.ad_value(17), 1.0), 1.0, s.ad_value(24), A::offset(A::add_scaled_inputs(s.ad_value(200), 1.0, s.ad_value(12), 2.0), 1.0), 1.0), 1.0, s.ad_value(138), A::scale_offset(s.ad_value(12), 2.0, 1.0), 1.0));
            s.store_sub_ad_rhs(131, 12, A::div(s.ad_value(16), s.ad_value(18)));
        }

        if (!s.b[1369]) {
            s.store_div_ad(12, A::mul_scaled_lhs(s.ad_value(138), 0.5, A::add(A::square(s.ad_value(200)), s.ad_value(200))), A::offset(A::mul_scaled_lhs(s.ad_value(138), 0.5, A::offset(s.ad_value(200), 1.0)), 1.0));
            s.store_mul_scaled_ad_rhs(13, 138, 2.0, A::sub(s.ad_value(200), s.ad_value(12)));
            s.store_sqrt_square_offset(14, 13, 1.0);
        }

        s.b[1374] = (s.v[13] != 0.0);
        s.v[1374] = if s.b[1374] { 1.0 } else { 0.0 };

        if ((!s.b[1369]) && s.b[1374]) {
            s.store_asinh(147, 13);
            s.store_ad_value(15, A::add_scaled_product(s.ad_value(14), 1.0, A::div_from_scalar(1.0, s.ad_value(13)), s.ad_value(147), 1.0));
        }

        if ((!s.b[1369]) && (!s.b[1374])) {
            s.store_add_ad_rhs(15, 14, A::div_from_scalar(1.0, s.ad_value(14)));
        }

        if (!s.b[1369]) {
            s.store_ad_value(16, A::add_scaled_products(s.ad_value(12), s.ad_value(15), 1.0, s.ad_value(138), A::sub(A::add(A::square(s.ad_value(200)), s.ad_value(200)), A::add(A::square(s.ad_value(12)), s.ad_value(12))), (-1.0)));
        }

        s.b[1375] = (s.v[13] != 0.0);
        s.v[1375] = if s.b[1375] { 1.0 } else { 0.0 };

        if ((!s.b[1369]) && s.b[1375]) {
            s.store_div_ad(17, A::mul_scaled_lhs(s.ad_value(138), (-2.0), A::add_scaled_product(s.ad_value(147), (-1.0), s.ad_value(13), s.ad_value(14), 1.0)), A::square(s.ad_value(13)));
        }

        if ((!s.b[1369]) && (!s.b[1375])) {
            s.store_mul_scaled_ad_rhs(17, 138, (-2.0), A::div(s.ad_value(13), s.ad_value(14)));
        }

        if (!s.b[1369]) {
            s.store_ad_value(18, A::add_scaled_product(A::add_scaled_product(s.ad_value(15), 1.0, s.ad_value(12), s.ad_value(17), 1.0), 1.0, s.ad_value(138), A::scale_offset(s.ad_value(12), 2.0, 1.0), 1.0));
            s.store_sub_ad_rhs(12, 12, A::div(s.ad_value(16), s.ad_value(18)));
            s.store_mul_scaled_ad_rhs(13, 138, 2.0, A::sub(s.ad_value(200), s.ad_value(12)));
            s.store_sqrt_square_offset(14, 13, 1.0);
        }

        s.b[1376] = (s.v[13] != 0.0);
        s.v[1376] = if s.b[1376] { 1.0 } else { 0.0 };

        if ((!s.b[1369]) && s.b[1376]) {
            s.store_asinh(147, 13);
            s.store_ad_value(15, A::add_scaled_product(s.ad_value(14), 1.0, A::div_from_scalar(1.0, s.ad_value(13)), s.ad_value(147), 1.0));
        }

        if ((!s.b[1369]) && (!s.b[1376])) {
            s.store_add_ad_rhs(15, 14, A::div_from_scalar(1.0, s.ad_value(14)));
        }

        if (!s.b[1369]) {
            s.store_ad_value(16, A::add_scaled_products(s.ad_value(12), s.ad_value(15), 1.0, s.ad_value(138), A::sub(A::add(A::square(s.ad_value(200)), s.ad_value(200)), A::add(A::square(s.ad_value(12)), s.ad_value(12))), (-1.0)));
        }

        s.b[1377] = (s.v[13] != 0.0);
        s.v[1377] = if s.b[1377] { 1.0 } else { 0.0 };

        if ((!s.b[1369]) && s.b[1377]) {
            s.store_div_ad(17, A::mul_scaled_lhs(s.ad_value(138), (-2.0), A::add_scaled_product(s.ad_value(147), (-1.0), s.ad_value(13), s.ad_value(14), 1.0)), A::square(s.ad_value(13)));
        }

        if ((!s.b[1369]) && (!s.b[1377])) {
            s.store_mul_scaled_ad_rhs(17, 138, (-2.0), A::div(s.ad_value(13), s.ad_value(14)));
        }

        if (!s.b[1369]) {
            s.store_ad_value(18, A::add_scaled_product(A::add_scaled_product(s.ad_value(15), 1.0, s.ad_value(12), s.ad_value(17), 1.0), 1.0, s.ad_value(138), A::scale_offset(s.ad_value(12), 2.0, 1.0), 1.0));
            s.store_sub_ad_rhs(131, 12, A::div(s.ad_value(16), s.ad_value(18)));
        }

        let assign18670_ad_e26371: A = A::sub(A::sub_scaled_inputs(s.ad_value(91), 1.0, s.ad_value(89), 2.0), A::add_scaled_inputs(s.ad_value(131), 2.0, A::ln(A::max_with_scalar(A::mul(A::mul3_scaled_output(s.ad_value(131), s.ad_value(90), s.ad_value(126), 2.0), A::add(A::mul3_scaled_output(s.ad_value(131), s.ad_value(90), s.ad_value(126), 2.0), A::div(s.ad_value(125), A::offset(s.ad_value(90), (-1.0))))), 1e-38)), 1.0));
        s.store_ad_value(143, assign18670_ad_e26371);

        s.store_mul(136, 143, 106);

        s.b[1378] = ((p.p1130 == 0.0) && (p.p1131 == 0.0));
        s.v[1378] = if s.b[1378] { 1.0 } else { 0.0 };

        if s.b[1378] {
            s.store_scalar(782, 1.0);
        }

        if (!s.b[1378]) {
            s.store_div_from_scalar_offset_ad(13, s.v[30], A::sqrt(A::mul(s.ad_value(538), s.ad_value(112))), s.v[30]);
            s.store_offset_div_ad(782, A::sub_scaled_inputs(s.ad_value(13), p.p1130, A::mul3_scaled_output(s.ad_value(13), A::powf(s.ad_value(200), p.p1132), s.ad_value(106), p.p1131), 1.0), A::scale_offset(s.ad_value(61), p.p1133, 1.0), 1.0);
        }

    }

    pub(super) fn stamp_transient_block_12(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[1379] = ((0.1 == 0.0) && (s.v[782] < ((-2500.0) * 0.0005)));
        s.v[1379] = if s.b[1379] { 1.0 } else { 0.0 };

        if ((!s.b[1378]) && s.b[1379]) {
            s.store_div_from_scalar_scaled_input(782, ((-0.0005) * 0.0005), 782, 16.0);
        }

        if ((!s.b[1378]) && (!s.b[1379])) {
            s.store_scaled_add_ad(782, A::offset(s.ad_value(782), 0.1), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(782), (-0.1)), A::offset(s.ad_value(782), (-0.1))), ((0.25 * 0.0005) * 0.0005))), 0.5);
        }

        s.b[1380] = ((0.0 == 0.0) && ((s.v[136] - s.v[70]) < ((-2500.0) * 0.001)));
        s.v[1380] = if s.b[1380] { 1.0 } else { 0.0 };

        if s.b[1380] {
            s.store_div_from_scalar_ad(140, ((-0.001) * 0.001), A::sub_scaled_inputs(s.ad_value(136), 16.0, s.ad_value(70), 16.0));
        }

        if (!s.b[1380]) {
            s.store_ad_value(140, A::add_scaled_inputs3(s.ad_value(136), 0.5, s.ad_value(70), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(136), s.ad_value(70)), A::sub(s.ad_value(136), s.ad_value(70))), ((0.25 * 0.001) * 0.001))), 0.5));
        }

        s.store_div(140, 140, 782);

        s.store_pow_ad(19, A::offset(A::div(s.ad_value(74), s.ad_value(140)), 1e-6), A::div_from_scalar(1.0, s.ad_value(412)));

        s.store_pow_ad(20, A::offset(s.ad_value(19), 1.0), A::neg(s.ad_value(412)));

        s.store_mul(139, 74, 20);

        s.store_mul_add_lhs(142, 139, 70, 107);

        s.store_scaled_add_ad(20, A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(91), (-1.0)), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);

        s.store_sqrt(96, 20);

        s.store_div_ad_lhs(12, A::offset(A::div_scaled_inputs(s.ad_value(125), 1.0, s.ad_value(96), 2.0), 1.0), 125);

        s.store_ad_value(13, A::add_scaled_inputs3(s.ad_value(91), 1.0, s.ad_value(89), (-2.0), s.ad_value(142), -1.0));

        s.store_sub_ad_rhs(14, 13, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));

        s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul(s.ad_value(14), A::offset(s.ad_value(14), 0.402982)), 2.446562)), 0.5);

        s.copy_ad(94, 96);

        s.b[1381] = (s.v[20] <= (-68.0));
        s.v[1381] = if s.b[1381] { 1.0 } else { 0.0 };

        if s.b[1381] {
            s.store_scalar(16, (-100.0));
            s.store_scalar(17, 20.0);
        }

        s.b[1382] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));
        s.v[1382] = if s.b[1382] { 1.0 } else { 0.0 };

        if (s.b[1381] && s.b[1382]) {
            s.store_limited_exp(15, 16);
        }

        s.b[1383] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));
        s.v[1383] = if s.b[1383] { 1.0 } else { 0.0 };

        if ((s.b[1381] && (!s.b[1382])) && s.b[1383]) {
            s.store_limited_exp(15, 20);
        }

        if ((s.b[1381] && (!s.b[1382])) && (!s.b[1383])) {
            s.store_div_ad_lhs(14, A::sub(s.ad_value(20), s.ad_value(16)), 17);
            s.store_square(18, 14);
            s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add_scaled_product(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), 1.0, s.ad_value(18), A::sub_from_scalar((15.0 / 16.0), A::mul(s.ad_value(18), A::sub_from_scalar(1.25, s.ad_value(18)))), 1.0), 1.0));
        }

        if s.b[1381] {
            s.store_mul_ad_rhs(144, 15, A::add_scaled_inputs3(A::offset(s.ad_value(13), 1.0), 1.0, s.ad_value(20), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), -1.0));
        }

        if (!s.b[1381]) {
            s.store_limited_exp(15, 20);
            s.store_div_from_scalar(95, 1.0, 94);
            s.store_ad_value(16, A::add_scaled_inputs3(s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(13), -1.0));
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0)));
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
            s.store_ad_value(16, A::add_scaled_inputs3(s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(13), -1.0));
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0)));
            s.store_scaled_mul_ad(18, A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0)), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0)), 1.0);
            s.store_sub_ad_lhs(19, A::add_scaled_product(A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-1.0)), 18);
            s.store_ad_value(144, A::add_scaled_product(s.ad_value(15), 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::offset(A::div(A::mul(s.ad_value(16), s.ad_value(19)), A::mul_scaled_lhs(s.ad_value(17), 2.0, s.ad_value(17))), 1.0), (-1.0)));
        }

        s.store_offset_ad(92, A::add_scaled_inputs3(s.ad_value(91), 1.0, s.ad_value(200), (-1.0), s.ad_value(144), -1.0), (-1.0));

        s.b[1384] = ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0)));
        s.v[1384] = if s.b[1384] { 1.0 } else { 0.0 };

        if s.b[1384] {
            s.store_div_from_scalar_scaled_input(12, ((-2.0) * 2.0), 92, 16.0);
        }

        if (!s.b[1384]) {
            s.store_scaled_add_ad(12, A::offset(s.ad_value(92), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(92), (-1.0)), A::offset(s.ad_value(92), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        s.store_sqrt(14, 12);

        s.store_offset_div_ad(90, s.ad_value(125), A::add(s.ad_value(96), s.ad_value(14)), 1.0);

        s.store_mul_ad(217, A::sub(s.ad_value(200), s.ad_value(144)), A::sub(s.ad_value(200), s.ad_value(144)));

        s.store_div_from_scalar_add_ad(12, 1.0, A::offset(s.ad_value(200), 1.0), s.ad_value(144));

        s.store_mul(13, 217, 12);

        s.store_ad_value(189, A::add_scaled_product(A::sub(s.ad_value(59), s.ad_value(91)), 1.0, A::offset(s.ad_value(90), (-1.0)), A::add_scaled_inputs3(s.ad_value(200), 1.0, s.ad_value(144), 1.0, s.ad_value(13), 0.3333333333333333), (-1.0)));

        s.store_scale(14, 90, 0.3333333333333333);

        s.store_mul(15, 13, 12);

        s.store_mul_ad_rhs(190, 14, A::add_scaled_product(A::add_scaled_inputs(s.ad_value(200), 2.0, s.ad_value(144), 1.0), 1.0, A::add_scaled_inputs(A::scale_offset(s.ad_value(200), 0.8, 1.0), 1.0, s.ad_value(144), 1.2), s.ad_value(15), 0.5));

        s.store_mul_ad_rhs(193, 14, A::add_scaled_product(A::add_scaled_inputs(s.ad_value(200), 1.0, s.ad_value(144), 2.0), 1.0, A::add_scaled_inputs(A::scale_offset(s.ad_value(200), 1.2, 1.0), 1.0, s.ad_value(144), 0.8), s.ad_value(15), 0.5));

        s.b[1385] = ((0.0 == 0.0) && ((s.v[106] * s.v[189]) < ((-2500.0) * 0.1)));
        s.v[1385] = if s.b[1385] { 1.0 } else { 0.0 };

        if s.b[1385] {
            s.store_div_from_scalar_ad(81, ((-0.1) * 0.1), A::mul_scaled_output(s.ad_value(106), s.ad_value(189), 16.0));
        }

        if (!s.b[1385]) {
            s.store_ad_value(81, A::add_scaled_product(A::sqrt(A::offset(A::mul3(s.ad_value(106), s.ad_value(189), A::mul(s.ad_value(106), s.ad_value(189))), ((0.25 * 0.1) * 0.1))), 0.5, s.ad_value(106), s.ad_value(189), 0.5));
        }

        s.store_mul_add_rhs(80, 106, 190, 193);

        s.store_add_scaled_inputs(156, 81, s.v[155], 80, (s.v[158] * s.v[155]));

        s.store_pow_ad(14, A::scaled_offset(A::div(s.ad_value(80), s.ad_value(81)), 1.0, 0.5), s.ad_value(513));

        s.store_ad_value(15, A::add_scaled_product(A::div(s.ad_value(510), s.ad_value(14)), 1.0, A::add_scaled_product(s.ad_value(506), 1.0, s.ad_value(516), s.ad_value(61), 1.0), A::pow(s.ad_value(156), s.ad_value(407)), 1.0));

        s.store_offset(16, 15, 1.0);

        s.b[1386] = ((1.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.0015)));
        s.v[1386] = if s.b[1386] { 1.0 } else { 0.0 };

        if s.b[1386] {
            s.store_div_from_scalar_scaled_input(159, ((-0.0015) * 0.0015), 16, 16.0);
        }

        if (!s.b[1386]) {
            s.store_scaled_add_ad(159, A::offset(s.ad_value(16), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(16), (-1.0)), A::offset(s.ad_value(16), (-1.0))), ((0.25 * 0.0015) * 0.0015))), 0.5);
        }

        s.store_ad_value(134, A::div_scaled_inputs(s.ad_value(502), 2.0, A::div(s.ad_value(499), s.ad_value(159)), 1.0));

        s.store_scale(135, 134, s.v[30]);

        s.b[1387] = (s.v[537] > 0.0);
        s.v[1387] = if s.b[1387] { 1.0 } else { 0.0 };

        if s.b[1387] {
            s.store_offset_div_ad(172, A::mul(s.ad_value(537), s.ad_value(80)), s.ad_value(135), 1.0);
        }

        if (!s.b[1387]) {
            s.store_div_from_scalar_sub_from_scalar_ad(172, 1.0, 1.0, A::div(A::mul(s.ad_value(537), s.ad_value(80)), s.ad_value(135)));
        }

        s.copy_ad(171, 519);

        s.store_sub(167, 74, 139);

        s.store_add_scaled_inputs(174, 80, 1.0, 106, 2.0);

        s.b[1388] = (s.v[171] > 0.0);
        s.v[1388] = if s.b[1388] { 1.0 } else { 0.0 };

        if s.b[1388] {
            s.store_div_ad_rhs(15, 174, A::add(s.ad_value(140), s.ad_value(174)));
        }

        if s.b[1388] {
            let assign19470_ad_e27354: A = {
                if (!((1.0 + (s.v[520] * s.v[61])) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(520), s.ad_value(61)), 1.0), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(520), s.ad_value(61)), 1.0), A::offset(A::mul(s.ad_value(520), s.ad_value(61)), 1.0)), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if ((1.0 + (s.v[520] * s.v[61])) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(520), s.ad_value(61)), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad_value(16, assign19470_ad_e27354);
        }

        if s.b[1388] {
            s.store_div_from_scalar(17, 1.0, 16);
            s.store_mul_ad_lhs(173, A::mul3(A::div(s.ad_value(174), s.ad_value(171)), s.ad_value(15), s.ad_value(172)), 17);
            s.store_offset_div(175, 167, 173, 1.0);
        }

        if (!s.b[1388]) {
            s.store_scalar(175, 1.0);
        }

        s.b[1389] = (s.v[525] <= 0.0);
        s.v[1389] = if s.b[1389] { 1.0 } else { 0.0 };

        if s.b[1389] {
            s.store_scalar(105, 1.0);
        }

        if (!s.b[1389]) {
            s.store_scaled_div(21, 525, 174, ((s.v[30]) as f64).sqrt());
            s.store_div_from_scalar_offset_input(105, 1.0, 21, 1.0);
        }

        s.store_add(170, 140, 135);

        s.b[1390] = (s.v[541] > 0.0);
        s.v[1390] = if s.b[1390] { 1.0 } else { 0.0 };

        s.b[1391] = (p.p350 < 0.0);
        s.v[1391] = if s.b[1391] { 1.0 } else { 0.0 };

        if (s.b[1390] && s.b[1391]) {
            s.store_div_ad_lhs(13, A::div(s.ad_value(541), A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(80), p.p350, s.ad_value(135), 1.0))), 105);
        }

        if (s.b[1390] && (!s.b[1391])) {
            s.store_div_ad_lhs(13, A::mul(s.ad_value(541), A::offset(A::div_scaled_inputs(s.ad_value(80), p.p350, s.ad_value(135), 1.0), 1.0)), 105);
        }

        if s.b[1390] {
            s.store_offset_mul_ad(176, s.ad_value(13), A::ln(A::max_with_scalar(A::offset(A::div(A::div(s.ad_value(167), s.ad_value(13)), s.ad_value(170)), 1.0), 1e-38)), 1.0);
        }

        s.b[1392] = (p.p350 < 0.0);
        s.v[1392] = if s.b[1392] { 1.0 } else { 0.0 };

        if ((!s.b[1390]) && s.b[1392]) {
            s.store_div_ad_lhs(13, A::div(s.ad_value(541), A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(80), p.p350, s.ad_value(135), 1.0))), 105);
        }

        if ((!s.b[1390]) && (!s.b[1392])) {
            s.store_div_ad_lhs(13, A::mul(s.ad_value(541), A::offset(A::div_scaled_inputs(s.ad_value(80), p.p350, s.ad_value(135), 1.0), 1.0)), 105);
        }

        if (!s.b[1390]) {
            s.store_offset(176, 13, 1.0);
        }

        s.store_mul(175, 175, 176);

        s.store_limited_exp_mul(13, 524, 74);

        s.b[1393] = (s.v[523] > 0.0);
        s.v[1393] = if s.b[1393] { 1.0 } else { 0.0 };

        if s.b[1393] {
            s.store_scalar(14, (1.0 + (p.p369 * s.v[30])));
            s.store_div_ad_lhs(168, A::offset(A::mul(s.ad_value(14), s.ad_value(13)), 1.0), 523);
            s.store_mul(168, 168, 105);
        }

        if (!s.b[1393]) {
            s.store_scalar(168, 5.540622384e34);
        }

        s.store_div(16, 167, 168);

        s.store_offset(12, 16, 1.0);

        s.store_mul(175, 175, 12);

        s.b[1394] = (s.v[522] > 0.0);
        s.v[1394] = if s.b[1394] { 1.0 } else { 0.0 };

        s.b[1395] = (s.v[167] > ((s.v[521] * s.v[129]) / 80.0));
        s.v[1395] = if s.b[1395] { 1.0 } else { 0.0 };

        if (s.b[1394] && s.b[1395]) {
            s.store_div_ad_lhs(12, A::mul(s.ad_value(521), s.ad_value(129)), 167);
            s.store_ad_value(169, A::div_scaled_inputs(A::limited_exp(s.ad_value(12)), s.v[30], s.ad_value(522), 1.0));
        }

        if (s.b[1394] && (!s.b[1395])) {
            s.store_div_from_scalar(169, (5.540622384e34 * s.v[30]), 522);
        }

        if (!s.b[1394]) {
            s.store_scalar(169, 5.540622384e34);
        }

        s.store_offset_div(177, 167, 169, 1.0);

        s.store_mul(175, 175, 177);

        s.store_pow_ad(12, s.ad_value(159), A::div_from_scalar(1.0, s.ad_value(166)));

        s.store_mul(23, 453, 61);

        s.store_sqrt_square_offset(24, 23, 0.1);

        s.store_scaled_add_ad(13, A::sub_from_scalar(1.0, s.ad_value(23)), A::sqrt(A::add_scaled_product(s.ad_value(24), 1.0, A::sub_from_scalar(1.0, s.ad_value(23)), A::sub_from_scalar(1.0, s.ad_value(23)), 1.0)), 0.5);

        s.store_div_ad(14, A::mul_scaled_lhs(s.ad_value(80), (10.0 * p.p433), s.ad_value(13)), A::offset(A::mul(s.ad_value(80), s.ad_value(13)), (10.0 * p.p433)));

        s.b[1396] = (s.v[536] < 0.0);
        s.v[1396] = if s.b[1396] { 1.0 } else { 0.0 };

        if s.b[1396] {
            s.store_scaled_mul_ad(138, A::div_scaled_inputs(A::mul(A::div(s.ad_value(499), s.ad_value(12)), s.ad_value(106)), 1.0, s.ad_value(502), s.v[30]), A::div_from_scalar(1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(536), s.ad_value(14)))), 2.0);
        }

        if (!s.b[1396]) {
            s.store_scaled_mul_ad(138, A::div_scaled_inputs(A::mul(A::div(s.ad_value(499), s.ad_value(12)), s.ad_value(106)), 1.0, s.ad_value(502), s.v[30]), A::offset(A::mul(s.ad_value(536), s.ad_value(14)), 1.0), 2.0);
        }

        s.store_mul_scaled_ad_rhs(13, 138, 2.0, A::sub(s.ad_value(200), s.ad_value(144)));

        s.store_sqrt_square_offset(14, 13, 1.0);

        s.b[1397] = (s.v[13] != 0.0);
        s.v[1397] = if s.b[1397] { 1.0 } else { 0.0 };

        if s.b[1397] {
            s.store_ad_value(162, A::add_scaled_product(s.ad_value(14), 0.5, A::div_from_scalar(1.0, s.ad_value(13)), A::asinh(s.ad_value(13)), 0.5));
        }

        if (!s.b[1397]) {
            s.store_scaled_add_ad_rhs(162, 14, A::div_from_scalar(1.0, s.ad_value(14)), 0.5);
        }

        s.copy_ad(163, 162);

        s.v[241] = 0.0;

        s.v[242] = 0.0;

        s.b[1398] = (p.p42 == 1.0);
        s.v[1398] = if s.b[1398] { 1.0 } else { 0.0 };

        if s.b[1398] {
            s.store_scalar(244, 0.0);
            s.store_scalar(245, 1.0);
            s.store_mul_voltage_ad(71, s.ad_value(187), ctx, nodes, Some(8), Some(11));
            s.store_sub(53, 64, 71);
            s.store_sub(14, 53, 63);
            s.store_sqrt_square_offset(15, 14, 0.01);
            s.store_scaled_add(77, 14, 15, 0.5);
            s.store_offset_mul(17, 526, 77, 1.0);
            s.copy_ad(51, 71);
            s.store_ad_value(18, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(17)), 1.0, s.ad_value(543), s.ad_value(51), 1.0));
            s.store_scaled_add_ad_rhs(16, 18, A::sqrt(A::offset(A::square(s.ad_value(18)), 0.01)), 0.5);
            s.store_mul_ad_rhs(241, 408, A::add_scaled_product(s.ad_value(239), 1.0, A::add_scaled_product(s.ad_value(529), 1.0, s.ad_value(531), s.ad_value(16), 1.0), s.ad_value(235), 1.0));
            s.store_mul_voltage_ad(67, s.ad_value(187), ctx, nodes, Some(6), Some(11));
            s.store_sub(55, 64, 67);
            s.store_sub(14, 55, 63);
            s.store_sqrt_square_offset(15, 14, 0.01);
            s.store_scaled_add(78, 14, 15, 0.5);
            s.store_offset_mul(17, 526, 78, 1.0);
            s.copy_ad(49, 67);
            s.store_ad_value(18, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(17)), 1.0, s.ad_value(543), s.ad_value(49), 1.0));
            s.store_scaled_add_ad_rhs(16, 18, A::sqrt(A::offset(A::square(s.ad_value(18)), 0.01)), 0.5);
            s.store_mul_ad_rhs(242, 408, A::add_scaled_product(s.ad_value(240), 1.0, A::add_scaled_product(s.ad_value(528), 1.0, s.ad_value(530), s.ad_value(16), 1.0), s.ad_value(235), 1.0));
        }

        if (!s.b[1398]) {
            s.store_offset_mul(12, 526, 80, 1.0);
            s.store_mul_sub_rhs(13, 543, 111, 128);
            s.store_add_ad_lhs(14, A::div_from_scalar(1.0, s.ad_value(12)), 13);
            s.store_scaled_add_ad_rhs(15, 14, A::sqrt(A::offset(A::square(s.ad_value(14)), 0.01)), 0.5);
            s.store_mul_ad_affine_product_lhs(244, s.ad_value(408), A::add_scaled_product(s.ad_value(533), 1.0, s.ad_value(532), s.ad_value(15), 1.0), p.p2, 0.0, 235);
        }

    }

    pub(super) fn stamp_transient_block_13(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (!s.b[1398]) {
            s.copy_ad(242, 240);
            s.copy_ad(241, 239);
            s.store_offset_ad(245, A::mul3_scaled_output(A::div(s.ad_value(499), A::mul(s.ad_value(162), s.ad_value(159))), s.ad_value(80), s.ad_value(244), ((s.v[46] * s.v[29]) * 1.0 / (s.v[30]))), 1.0);
        }

        s.b[1399] = (p.p42 == 2.0);
        s.v[1399] = if s.b[1399] { 1.0 } else { 0.0 };

        if ((!s.b[1398]) && s.b[1399]) {
            s.store_mul_add_ad_rhs(244, 408, A::add_scaled_product(s.ad_value(239), 1.0, A::add_scaled_product(s.ad_value(533), 1.0, s.ad_value(532), s.ad_value(15), 1.0), s.ad_value(235), p.p2), s.ad_value(240));
            s.store_scalar(242, 0.0);
            s.store_scalar(241, 0.0);
            s.store_offset_ad(245, A::mul3_scaled_output(A::div(s.ad_value(499), A::mul(s.ad_value(162), s.ad_value(159))), s.ad_value(80), s.ad_value(244), ((s.v[46] * s.v[29]) * 1.0 / (s.v[30]))), 1.0);
        }

        s.store_add_ad_rhs(12, 150, A::div(s.ad_value(153), A::add_scaled_product(s.ad_value(80), 1.0, s.ad_value(104), s.ad_value(393), 2.0)));

        s.store_sub(216, 200, 144);

        s.store_mul3_lhs(13, 12, 216, 216);

        s.store_offset(14, 13, ((1.0) + ((-0.001))));

        s.store_offset_ad(15, A::add_scaled_inputs(s.ad_value(14), 0.5, A::sqrt(A::offset(A::square(s.ad_value(14)), 0.004)), 0.5), (-1.0));

        s.store_scaled_offset_ad(154, A::sqrt(A::offset(s.ad_value(15), 1.0)), 1.0, 0.5);

        s.store_offset_ad(154, A::sub_scaled_inputs(A::offset(s.ad_value(154), 1.0), 0.5, A::sqrt(A::offset(A::mul(A::offset(s.ad_value(154), (-1.0)), A::offset(s.ad_value(154), (-1.0))), ((0.25 * 0.01) * 0.01))), 0.5), (0.25 * 0.01));

        s.store_add(12, 200, 144);

        s.store_sub(13, 200, 144);

        s.store_div_ad_rhs(14, 13, A::add(s.ad_value(12), s.ad_value(610)));

        s.store_mul3_lhs(15, 609, 14, 14);

        s.store_offset(611, 15, 1.0);

        s.store_div_ad_rhs(21, 633, A::add_scaled_products(A::max_from_scalar(0.0, A::add(s.ad_value(636), A::mul3(s.ad_value(639), s.ad_value(13), s.ad_value(13)))), s.ad_value(12), 1.0, s.ad_value(104), s.ad_value(393), 2.0));

        s.store_limited_exp_neg_input(628, 21);

        s.store_mul3_lhs(160, 159, 162, 245);

        s.store_div(157, 499, 160);

        let assign20520_ad_e28234: A = A::mul3(A::div(A::mul(A::mul3(A::mul3_scaled_output(s.ad_value(90), s.ad_value(157), s.ad_value(106), ((2.0 * p.p2) * ((s.v[29] * 1.0 / (s.v[30])) * s.v[46]))), s.ad_value(106), A::mul(A::sub(s.ad_value(200), s.ad_value(144)), A::add(A::offset(s.ad_value(200), 1.0), s.ad_value(144)))), s.ad_value(175)), s.ad_value(154)), s.ad_value(611), s.ad_value(628));
        s.store_ad_value(188, assign20520_ad_e28234);

        s.store_scale(188, 188, p.p36);

        s.b[1400] = ((p.p42 == 1.0) && (p.p1094 == 1.0));
        s.v[1400] = if s.b[1400] { 1.0 } else { 0.0 };

        if s.b[1400] {
            s.store_mul_ln_ad_rhs(753, 108, A::div_scaled_inputs(s.ad_value(481), p.p1117, A::powf(s.ad_value(28), 2.0), 1.0));
        }

        s.b[1401] = (((p.p49 != 0.0) && (p.p909 > 0.0)) && (s.v[38] > 0.0));
        s.v[1401] = if s.b[1401] { 1.0 } else { 0.0 };

        if (s.b[1400] && s.b[1401]) {
            s.store_mul_sqrt_ad_rhs(753, 108, A::offset(A::square(s.ad_value(753)), 1e-6));
        }

        if s.b[1400] {
            s.store_sub_from_scalar_ad(16, 1.0, A::scale(s.ad_value(50), p.p1113));
        }

        s.b[1402] = ((0.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.001)));
        s.v[1402] = if s.b[1402] { 1.0 } else { 0.0 };

        if (s.b[1400] && s.b[1402]) {
            s.store_div_from_scalar_scaled_input(16, ((-0.001) * 0.001), 16, 16.0);
        }

        if (s.b[1400] && (!s.b[1402])) {
            s.store_scaled_add_ad_rhs(16, 16, A::sqrt(A::offset(A::mul(s.ad_value(16), s.ad_value(16)), ((0.25 * 0.001) * 0.001))), 0.5);
        }

        if s.b[1400] {
            s.store_offset(13, 200, (-p.p1102));
        }

        s.b[1403] = ((0.1 == 0.0) && (s.v[13] < ((-2500.0) * 2.0)));
        s.v[1403] = if s.b[1403] { 1.0 } else { 0.0 };

        if (s.b[1400] && s.b[1403]) {
            s.store_div_from_scalar_scaled_input(13, ((-2.0) * 2.0), 13, 16.0);
        }

        if (s.b[1400] && (!s.b[1403])) {
            s.store_scaled_add_ad(13, A::offset(s.ad_value(13), 0.1), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(13), (-0.1)), A::offset(s.ad_value(13), (-0.1))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        if s.b[1400] {
            s.store_ad_value(14, A::div_scaled_inputs(s.ad_value(13), (10.0 * p.p1103), A::offset(s.ad_value(13), (10.0 * p.p1103)), 1.0));
            s.store_mul_ad_rhs(754, 763, A::scale_offset(s.ad_value(14), p.p1101, 1.0));
            s.store_scale(23, 754, ((p.p2 * s.v[29]) * 1.60219e-19));
        }

        s.b[1404] = (p.p1110 != 0.0);
        s.v[1404] = if s.b[1404] { 1.0 } else { 0.0 };

        if (s.b[1400] && s.b[1404]) {
            s.store_abs_voltage(757, ctx, nodes, Some(6), Some(5));
        }

        s.b[1405] = (p.p1127 == 0.0);
        s.v[1405] = if s.b[1405] { 1.0 } else { 0.0 };

        if ((s.b[1400] && s.b[1404]) && s.b[1405]) {
            s.store_scalar(21, 1.0);
        }

        s.b[1406] = ((0.0 == 0.0) && ((s.v[757] - p.p1126) < ((-2500.0) * 0.5)));
        s.v[1406] = if s.b[1406] { 1.0 } else { 0.0 };

        if (((s.b[1400] && s.b[1404]) && (!s.b[1405])) && s.b[1406]) {
            s.store_div_from_scalar_offset_scaled_input(22, ((-0.5) * 0.5), 757, 16.0, (((-p.p1126)) * (16.0)));
        }

        if (((s.b[1400] && s.b[1404]) && (!s.b[1405])) && (!s.b[1406])) {
            s.store_scaled_add_ad(22, A::offset(s.ad_value(757), (-p.p1126)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(757), (-p.p1126)), A::offset(s.ad_value(757), (-p.p1126))), ((0.25 * 0.5) * 0.5))), 0.5);
        }

        if ((s.b[1400] && s.b[1404]) && (!s.b[1405])) {
            s.store_offset_scaled(21, 22, p.p1127, 1.0);
        }

        s.b[1408] = ((p.p1098 != 0.0) && (p.p514 > 0.0));
        s.v[1408] = if s.b[1408] { 1.0 } else { 0.0 };

        if ((s.b[1400] && s.b[1404]) && s.b[1408]) {
            s.store_sqrt_offset_ad(760, A::square(A::voltage(ctx, nodes, Some(11), Some(3))), ((10.0) as f64).powf(((2.0 * ((-3.0) - ((p.p514) as f64).ln())) / p.p515)));
            s.store_mul_ad_affine_product_rhs(750, 23, s.ad_value(21), A::scale_offset(A::powf(s.ad_value(760), p.p515), p.p514, 1.0), p.p1099, 0.0);
        }

        if ((s.b[1400] && s.b[1404]) && (!s.b[1408])) {
            s.store_scaled_mul(750, 23, 21, p.p1099);
        }

        if (s.b[1400] && s.b[1404]) {
            s.store_offset_div(14, 50, 753, 1.0);
        }

        s.b[1409] = ((0.0 == 0.0) && (s.v[14] < ((-2500.0) * 0.05)));
        s.v[1409] = if s.b[1409] { 1.0 } else { 0.0 };

        if ((s.b[1400] && s.b[1404]) && s.b[1409]) {
            s.store_div_from_scalar_scaled_input(14, ((-0.05) * 0.05), 14, 16.0);
        }

        if ((s.b[1400] && s.b[1404]) && (!s.b[1409])) {
            s.store_scaled_add_ad_rhs(14, 14, A::sqrt(A::offset(A::mul(s.ad_value(14), s.ad_value(14)), ((0.25 * 0.05) * 0.05))), 0.5);
        }

        if (s.b[1400] && s.b[1404]) {
            s.store_sub_scaled_ad_lhs(18, A::sub_from_scalar(1.0, A::scaled_offset(A::sqrt(s.ad_value(14)), (-1.0), p.p1124)), 50, p.p1125);
        }

        s.b[1410] = ((0.0 == 0.0) && (s.v[18] < ((-2500.0) * 0.05)));
        s.v[1410] = if s.b[1410] { 1.0 } else { 0.0 };

        if ((s.b[1400] && s.b[1404]) && s.b[1410]) {
            s.store_div_from_scalar_scaled_input(18, ((-0.05) * 0.05), 18, 16.0);
        }

        if ((s.b[1400] && s.b[1404]) && (!s.b[1410])) {
            s.store_scaled_add_ad_rhs(18, 18, A::sqrt(A::offset(A::mul(s.ad_value(18), s.ad_value(18)), ((0.25 * 0.05) * 0.05))), 0.5);
        }

        if (s.b[1400] && s.b[1404]) {
            s.store_mul(750, 18, 750);
            s.store_mul3_affine_lhs(19, 762, 235, p.p1110, 0.0, 16);
            s.store_mul(755, 750, 19);
            s.store_div_ad(752, A::powf(s.ad_value(757), (4.0 - p.p1107)), A::add_scaled_inputs(A::powf(s.ad_value(757), (4.0 - p.p1107)), 1.0, A::powf(s.ad_value(755), (4.0 - p.p1107)), p.p1122));
            s.store_powf(17, 752, (1.0 / p.p1107));
            s.store_div_ad_lhs(20, A::mul(s.ad_value(17), s.ad_value(757)), 755);
        }

        s.b[1411] = ((0.0 == 0.0) && (s.v[20] < ((-2500.0) * 0.001)));
        s.v[1411] = if s.b[1411] { 1.0 } else { 0.0 };

        if ((s.b[1400] && s.b[1404]) && s.b[1411]) {
            s.store_div_from_scalar_scaled_input(20, ((-0.001) * 0.001), 20, 16.0);
        }

        if ((s.b[1400] && s.b[1404]) && (!s.b[1411])) {
            s.store_scaled_add_ad_rhs(20, 20, A::sqrt(A::offset(A::mul(s.ad_value(20), s.ad_value(20)), ((0.25 * 0.001) * 0.001))), 0.5);
        }

        if (s.b[1400] && s.b[1404]) {
            s.store_mul_powf_ad_rhs(759, 19, A::offset(A::powf(s.ad_value(20), p.p1107), 1.0), (1.0 / p.p1107));
        }

        s.b[1412] = (p.p1112 != 0.0);
        s.v[1412] = if s.b[1412] { 1.0 } else { 0.0 };

        if (s.b[1400] && s.b[1412]) {
            s.store_abs_voltage(758, ctx, nodes, Some(7), Some(8));
        }

        s.b[1414] = ((p.p1098 != 0.0) && (p.p516 > 0.0));
        s.v[1414] = if s.b[1414] { 1.0 } else { 0.0 };

        if ((s.b[1400] && s.b[1412]) && s.b[1414]) {
            s.store_sqrt_offset_ad(760, A::square(A::voltage(ctx, nodes, Some(11), Some(3))), ((10.0) as f64).powf(((2.0 * ((-3.0) - ((p.p516) as f64).ln())) / p.p517)));
            s.store_mul_scaled_ad_rhs(751, 23, p.p1109, A::scale_offset(A::powf(s.ad_value(760), p.p517), p.p516, 1.0));
        }

        if ((s.b[1400] && s.b[1412]) && (!s.b[1414])) {
            s.store_scale(751, 23, p.p1109);
        }

        if (s.b[1400] && s.b[1412]) {
            s.store_offset_div(14, 50, 753, 1.0);
        }

        s.b[1415] = ((0.0 == 0.0) && (s.v[14] < ((-2500.0) * 0.05)));
        s.v[1415] = if s.b[1415] { 1.0 } else { 0.0 };

        if ((s.b[1400] && s.b[1412]) && s.b[1415]) {
            s.store_div_from_scalar_scaled_input(14, ((-0.05) * 0.05), 14, 16.0);
        }

        if ((s.b[1400] && s.b[1412]) && (!s.b[1415])) {
            s.store_scaled_add_ad_rhs(14, 14, A::sqrt(A::offset(A::mul(s.ad_value(14), s.ad_value(14)), ((0.25 * 0.05) * 0.05))), 0.5);
        }

        if (s.b[1400] && s.b[1412]) {
            s.store_sub_scaled_ad_lhs(18, A::sub_from_scalar(1.0, A::scaled_offset(A::sqrt(s.ad_value(14)), (-1.0), p.p1124)), 50, p.p1125);
        }

        s.b[1416] = ((0.0 == 0.0) && (s.v[18] < ((-2500.0) * 0.05)));
        s.v[1416] = if s.b[1416] { 1.0 } else { 0.0 };

        if ((s.b[1400] && s.b[1412]) && s.b[1416]) {
            s.store_div_from_scalar_scaled_input(18, ((-0.05) * 0.05), 18, 16.0);
        }

        if ((s.b[1400] && s.b[1412]) && (!s.b[1416])) {
            s.store_scaled_add_ad_rhs(18, 18, A::sqrt(A::offset(A::mul(s.ad_value(18), s.ad_value(18)), ((0.25 * 0.05) * 0.05))), 0.5);
        }

        if (s.b[1400] && s.b[1412]) {
            s.store_mul(751, 18, 751);
            s.store_mul3_affine_lhs(19, 762, 235, p.p1112, 0.0, 16);
            s.store_mul(756, 751, 19);
            s.store_div_ad(752, A::powf(s.ad_value(758), (4.0 - p.p1107)), A::add_scaled_inputs(A::powf(s.ad_value(758), (4.0 - p.p1107)), 1.0, A::powf(s.ad_value(756), (4.0 - p.p1107)), p.p1122));
            s.store_powf(17, 752, (1.0 / p.p1107));
            s.store_div_ad_lhs(20, A::mul(s.ad_value(17), s.ad_value(758)), 756);
        }

        s.b[1417] = ((0.0 == 0.0) && (s.v[20] < ((-2500.0) * 0.001)));
        s.v[1417] = if s.b[1417] { 1.0 } else { 0.0 };

        if ((s.b[1400] && s.b[1412]) && s.b[1417]) {
            s.store_div_from_scalar_scaled_input(20, ((-0.001) * 0.001), 20, 16.0);
        }

        if ((s.b[1400] && s.b[1412]) && (!s.b[1417])) {
            s.store_scaled_add_ad_rhs(20, 20, A::sqrt(A::offset(A::mul(s.ad_value(20), s.ad_value(20)), ((0.25 * 0.001) * 0.001))), 0.5);
        }

        if (s.b[1400] && s.b[1412]) {
            s.store_mul_powf_ad_rhs(761, 19, A::offset(A::powf(s.ad_value(20), p.p1107), 1.0), (1.0 / p.p1107));
        }

        s.b[1418] = ((p.p1110 != 0.0) && (p.p1112 != 0.0));
        s.v[1418] = if s.b[1418] { 1.0 } else { 0.0 };

        if (s.b[1400] && s.b[1418]) {
            s.store_div_ad(17, A::mul(s.ad_value(57), s.ad_value(188)), A::min(s.ad_value(750), s.ad_value(751)));
            s.store_offset_ad(17, A::sub_scaled_inputs(A::offset(s.ad_value(17), 1.0), 0.5, A::sqrt(A::offset(A::mul(A::offset(s.ad_value(17), (-1.0)), A::offset(s.ad_value(17), (-1.0))), ((0.25 * p.p1108) * p.p1108))), 0.5), (0.25 * p.p1108));
            s.store_offset(17, 17, (((((0.5 * (((1.0 + ((0.25 * p.p1108) * p.p1108))) as f64).sqrt())) + ((-0.5)))) + ((-(0.25 * p.p1108)))));
        }

        s.b[1419] = (((-1.0) == 0.0) && (s.v[17] < ((-2500.0) * p.p1108)));
        s.v[1419] = if s.b[1419] { 1.0 } else { 0.0 };

        if ((s.b[1400] && s.b[1418]) && s.b[1419]) {
            s.store_div_from_scalar_scaled_input(17, ((-p.p1108) * p.p1108), 17, 16.0);
        }

        if ((s.b[1400] && s.b[1418]) && (!s.b[1419])) {
            s.store_scaled_add_ad(17, A::offset(s.ad_value(17), (-1.0)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(17), (-(-1.0))), A::offset(s.ad_value(17), (-(-1.0)))), ((0.25 * p.p1108) * p.p1108))), 0.5);
        }

        if (s.b[1400] && s.b[1418]) {
            s.store_offset(17, 17, (((-(0.5 * (((1.0 + ((0.25 * p.p1108) * p.p1108))) as f64).sqrt()))) + (0.5)));
            s.store_mul_ad_product_lhs(188, s.ad_value(57), A::min(s.ad_value(750), s.ad_value(751)), 17);
        }

        s.b[1420] = (p.p1110 != 0.0);
        s.v[1420] = if s.b[1420] { 1.0 } else { 0.0 };

        if ((s.b[1400] && (!s.b[1418])) && s.b[1420]) {
            s.store_div_ad_lhs(17, A::mul(s.ad_value(57), s.ad_value(188)), 750);
            s.store_offset_ad(17, A::sub_scaled_inputs(A::offset(s.ad_value(17), 1.0), 0.5, A::sqrt(A::offset(A::mul(A::offset(s.ad_value(17), (-1.0)), A::offset(s.ad_value(17), (-1.0))), ((0.25 * p.p1108) * p.p1108))), 0.5), (0.25 * p.p1108));
            s.store_offset(17, 17, (((((0.5 * (((1.0 + ((0.25 * p.p1108) * p.p1108))) as f64).sqrt())) + ((-0.5)))) + ((-(0.25 * p.p1108)))));
        }

        s.b[1421] = (((-1.0) == 0.0) && (s.v[17] < ((-2500.0) * p.p1108)));
        s.v[1421] = if s.b[1421] { 1.0 } else { 0.0 };

        if (((s.b[1400] && (!s.b[1418])) && s.b[1420]) && s.b[1421]) {
            s.store_div_from_scalar_scaled_input(17, ((-p.p1108) * p.p1108), 17, 16.0);
        }

        if (((s.b[1400] && (!s.b[1418])) && s.b[1420]) && (!s.b[1421])) {
            s.store_scaled_add_ad(17, A::offset(s.ad_value(17), (-1.0)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(17), (-(-1.0))), A::offset(s.ad_value(17), (-(-1.0)))), ((0.25 * p.p1108) * p.p1108))), 0.5);
        }

        if ((s.b[1400] && (!s.b[1418])) && s.b[1420]) {
            s.store_offset(17, 17, (((-(0.5 * (((1.0 + ((0.25 * p.p1108) * p.p1108))) as f64).sqrt()))) + (0.5)));
            s.store_mul3_lhs(188, 57, 750, 17);
        }

        s.b[1422] = (p.p1112 != 0.0);
        s.v[1422] = if s.b[1422] { 1.0 } else { 0.0 };

        if ((s.b[1400] && (!s.b[1418])) && s.b[1422]) {
            s.store_div_ad_lhs(17, A::mul(s.ad_value(57), s.ad_value(188)), 751);
            s.store_offset_ad(17, A::sub_scaled_inputs(A::offset(s.ad_value(17), 1.0), 0.5, A::sqrt(A::offset(A::mul(A::offset(s.ad_value(17), (-1.0)), A::offset(s.ad_value(17), (-1.0))), ((0.25 * p.p1108) * p.p1108))), 0.5), (0.25 * p.p1108));
            s.store_offset(17, 17, (((((0.5 * (((1.0 + ((0.25 * p.p1108) * p.p1108))) as f64).sqrt())) + ((-0.5)))) + ((-(0.25 * p.p1108)))));
        }

        s.b[1423] = (((-1.0) == 0.0) && (s.v[17] < ((-2500.0) * p.p1108)));
        s.v[1423] = if s.b[1423] { 1.0 } else { 0.0 };

        if (((s.b[1400] && (!s.b[1418])) && s.b[1422]) && s.b[1423]) {
            s.store_div_from_scalar_scaled_input(17, ((-p.p1108) * p.p1108), 17, 16.0);
        }

        if (((s.b[1400] && (!s.b[1418])) && s.b[1422]) && (!s.b[1423])) {
            s.store_scaled_add_ad(17, A::offset(s.ad_value(17), (-1.0)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(17), (-(-1.0))), A::offset(s.ad_value(17), (-(-1.0)))), ((0.25 * p.p1108) * p.p1108))), 0.5);
        }

        if ((s.b[1400] && (!s.b[1418])) && s.b[1422]) {
            s.store_offset(17, 17, (((-(0.5 * (((1.0 + ((0.25 * p.p1108) * p.p1108))) as f64).sqrt()))) + (0.5)));
            s.store_mul3_lhs(188, 57, 751, 17);
        }

        s.v[774] = 0.0;

        s.v[775] = 0.0;

        s.v[776] = 0.0;

        s.v[777] = 0.0;

        s.b[1424] = (((p.p42 == 1.0) && (p.p1095 == 1.0)) && (p.p1094 == 1.0));
        s.v[1424] = if s.b[1424] { 1.0 } else { 0.0 };

        if s.b[1424] {
            s.store_offset_scaled(764, 232, -1.0, (-p.p1114));
            s.store_div(764, 764, 108);
            s.store_scaled_sqrt_scaled_input(765, 109, (((2.0 * 1.60219e-19) * s.v[26]) * p.p1117), 1.0 / (s.v[46]));
            s.store_ln_ad(766, A::max_with_scalar(A::div_from_scalar(p.p1117, s.ad_value(28)), 1e-38));
            s.store_scalar(13, 1.0);
            s.store_div(204, 764, 13);
            s.store_div(205, 765, 13);
            s.store_sub_scaled_ad_rhs(13, 204, 0.5, A::scale_offset(s.ad_value(205), ((0.7071067811865475) * (3.0)), 3.0));
            s.store_add_ad_rhs(14, 13, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(13)), 1.0, s.ad_value(204), 6.0)));
        }

        s.b[1425] = (s.v[204] < 0.0);
        s.v[1425] = if s.b[1425] { 1.0 } else { 0.0 };

        if (s.b[1424] && s.b[1425]) {
            s.store_div_ad_lhs(15, A::sub(s.ad_value(204), s.ad_value(14)), 205);
            s.store_neg_ad(767, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));
        }

        if (s.b[1424] && (!s.b[1425])) {
            s.store_limited_exp_neg_input(15, 14);
            s.store_scale(13, 205, 0.5);
            s.store_sub_ad_lhs(14, A::sqrt(A::add_scaled_inputs3(A::offset(s.ad_value(204), (-1.0)), 1.0, s.ad_value(15), 1.0, A::square(s.ad_value(13)), 1.0)), 13);
            s.store_sub_ad_lhs(767, A::offset(A::square(s.ad_value(14)), 1.0), 15);
        }

        if s.b[1424] {
            s.store_scaled_add_ad(20, A::offset(s.ad_value(767), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(767), (-1.0)), A::offset(s.ad_value(767), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(96, 20);
        }

    }

    pub(super) fn stamp_transient_block_14(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1424] {
            s.store_div_ad_lhs(12, A::offset(A::div_scaled_inputs(s.ad_value(765), 1.0, s.ad_value(96), 2.0), 1.0), 765);
            s.store_ad_value(13, A::add_scaled_inputs3(s.ad_value(767), 1.0, s.ad_value(766), (-2.0), A::div(s.ad_value(69), s.ad_value(108)), -1.0));
            s.store_sub_ad_rhs(14, 13, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));
            s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul(s.ad_value(14), A::offset(s.ad_value(14), 0.402982)), 2.446562)), 0.5);
            s.copy_ad(94, 96);
        }

        s.b[1426] = (s.v[20] <= (-68.0));
        s.v[1426] = if s.b[1426] { 1.0 } else { 0.0 };

        if (s.b[1424] && s.b[1426]) {
            s.store_scalar(16, (-100.0));
            s.store_scalar(17, 20.0);
        }

        s.b[1427] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));
        s.v[1427] = if s.b[1427] { 1.0 } else { 0.0 };

        if ((s.b[1424] && s.b[1426]) && s.b[1427]) {
            s.store_limited_exp(15, 16);
        }

        s.b[1428] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));
        s.v[1428] = if s.b[1428] { 1.0 } else { 0.0 };

        if (((s.b[1424] && s.b[1426]) && (!s.b[1427])) && s.b[1428]) {
            s.store_limited_exp(15, 20);
        }

        if (((s.b[1424] && s.b[1426]) && (!s.b[1427])) && (!s.b[1428])) {
            s.store_div_ad_lhs(14, A::sub(s.ad_value(20), s.ad_value(16)), 17);
            s.store_square(18, 14);
            s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add_scaled_product(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), 1.0, s.ad_value(18), A::sub_from_scalar((15.0 / 16.0), A::mul(s.ad_value(18), A::sub_from_scalar(1.25, s.ad_value(18)))), 1.0), 1.0));
        }

        if (s.b[1424] && s.b[1426]) {
            s.store_mul_ad_rhs(768, 15, A::add_scaled_inputs3(A::offset(s.ad_value(13), 1.0), 1.0, s.ad_value(20), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), -1.0));
        }

        if (s.b[1424] && (!s.b[1426])) {
            s.store_limited_exp(15, 20);
            s.store_div_from_scalar(95, 1.0, 94);
            s.store_ad_value(16, A::add_scaled_inputs3(s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(13), -1.0));
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0)));
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
            s.store_ad_value(16, A::add_scaled_inputs3(s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(13), -1.0));
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0)));
            s.store_scaled_mul_ad(18, A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0)), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0)), 1.0);
            s.store_sub_ad_lhs(19, A::add_scaled_product(A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-1.0)), 18);
            s.store_ad_value(768, A::add_scaled_product(s.ad_value(15), 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::offset(A::div(A::mul(s.ad_value(16), s.ad_value(19)), A::mul_scaled_lhs(s.ad_value(17), 2.0, s.ad_value(17))), 1.0), (-1.0)));
        }

        s.b[1429] = ((1.0 == 0.0) && (s.v[767] < ((-2500.0) * 2.0)));
        s.v[1429] = if s.b[1429] { 1.0 } else { 0.0 };

        if (s.b[1424] && s.b[1429]) {
            s.store_div_from_scalar_scaled_input(769, ((-2.0) * 2.0), 767, 16.0);
        }

        if (s.b[1424] && (!s.b[1429])) {
            s.store_scaled_add_ad(769, A::offset(s.ad_value(767), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(767), (-1.0)), A::offset(s.ad_value(767), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        if s.b[1424] {
            s.store_sqrt(770, 769);
            s.store_sub_scaled_inputs(771, 767, 1.0, 768, 2.0);
        }

        s.b[1430] = ((1.0 == 0.0) && (s.v[771] < ((-2500.0) * 2.0)));
        s.v[1430] = if s.b[1430] { 1.0 } else { 0.0 };

        if (s.b[1424] && s.b[1430]) {
            s.store_div_from_scalar_scaled_input(12, ((-2.0) * 2.0), 771, 16.0);
        }

        if (s.b[1424] && (!s.b[1430])) {
            s.store_scaled_add_ad(12, A::offset(s.ad_value(771), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(771), (-1.0)), A::offset(s.ad_value(771), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        if s.b[1424] {
            s.store_offset_div_ad(772, s.ad_value(765), A::add(s.ad_value(770), A::sqrt(s.ad_value(12))), 1.0);
            s.store_sub_scaled_inputs(773, 767, 1.0, 768, 2.0);
            s.store_ad_value(775, A::mul3(A::div_from_scalar(((((p.p2 * s.v[33]) * p.p1115) * 8.85418e-12) * p.p111), s.ad_value(229)), s.ad_value(108), A::add_scaled_product(A::sub(s.ad_value(764), s.ad_value(773)), 1.0, s.ad_value(772), s.ad_value(768), (-2.0))));
        }

        s.b[1431] = (p.p1118 > 0.0);
        s.v[1431] = if s.b[1431] { 1.0 } else { 0.0 };

        if (s.b[1424] && s.b[1431]) {
            s.store_offset_scaled(13, 768, 1.0 / (p.p1119), 1.0);
            s.store_div_from_scalar(14, (p.p1118 * 1.9e-9), 13);
            s.store_div_from_scalar_ad(12, (3.9 * 8.85418e-12), A::add_scaled_inputs(s.ad_value(229), (3.9 * 1.0 / (p.p111)), s.ad_value(14), 1.0 / (s.v[47])));
        }

        if (s.b[1424] && (!s.b[1431])) {
            s.store_div_from_scalar(12, (8.85418e-12 * p.p111), 229);
        }

        if s.b[1424] {
            s.store_mul_ad_lhs(774, A::mul3_scaled_output(s.ad_value(772), s.ad_value(108), s.ad_value(12), (((p.p2 * s.v[33]) * p.p1116) * 2.0)), 768);
        }

        s.b[1432] = (p.p1096 == 1.0);
        s.v[1432] = if s.b[1432] { 1.0 } else { 0.0 };

        if (s.b[1424] && s.b[1432]) {
            s.store_offset_ad(764, A::mul_scaled_lhs(s.ad_value(187), -1.0, A::voltage(ctx, nodes, Some(10), Some(7))), (-p.p1114));
            s.store_div(764, 764, 108);
            s.store_scalar(13, 1.0);
            s.store_div(204, 764, 13);
            s.store_div(205, 765, 13);
            s.store_sub_scaled_ad_rhs(13, 204, 0.5, A::scale_offset(s.ad_value(205), ((0.7071067811865475) * (3.0)), 3.0));
            s.store_add_ad_rhs(14, 13, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(13)), 1.0, s.ad_value(204), 6.0)));
        }

        s.b[1433] = (s.v[204] < 0.0);
        s.v[1433] = if s.b[1433] { 1.0 } else { 0.0 };

        if ((s.b[1424] && s.b[1432]) && s.b[1433]) {
            s.store_div_ad_lhs(15, A::sub(s.ad_value(204), s.ad_value(14)), 205);
            s.store_neg_ad(767, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));
        }

        if ((s.b[1424] && s.b[1432]) && (!s.b[1433])) {
            s.store_limited_exp_neg_input(15, 14);
            s.store_scale(13, 205, 0.5);
            s.store_sub_ad_lhs(14, A::sqrt(A::add_scaled_inputs3(A::offset(s.ad_value(204), (-1.0)), 1.0, s.ad_value(15), 1.0, A::square(s.ad_value(13)), 1.0)), 13);
            s.store_sub_ad_lhs(767, A::offset(A::square(s.ad_value(14)), 1.0), 15);
        }

        if (s.b[1424] && s.b[1432]) {
            s.store_scaled_add_ad(20, A::offset(s.ad_value(767), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(767), (-1.0)), A::offset(s.ad_value(767), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(96, 20);
            s.store_div_ad_lhs(12, A::offset(A::div_scaled_inputs(s.ad_value(765), 1.0, s.ad_value(96), 2.0), 1.0), 765);
            s.store_ad_value(13, A::add_scaled_inputs3(s.ad_value(767), 1.0, s.ad_value(766), (-2.0), A::div(A::mul(s.ad_value(187), A::voltage(ctx, nodes, Some(7), Some(11))), s.ad_value(108)), -1.0));
            s.store_sub_ad_rhs(14, 13, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));
            s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul(s.ad_value(14), A::offset(s.ad_value(14), 0.402982)), 2.446562)), 0.5);
            s.copy_ad(94, 96);
        }

        s.b[1434] = (s.v[20] <= (-68.0));
        s.v[1434] = if s.b[1434] { 1.0 } else { 0.0 };

        if ((s.b[1424] && s.b[1432]) && s.b[1434]) {
            s.store_scalar(16, (-100.0));
            s.store_scalar(17, 20.0);
        }

        s.b[1435] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));
        s.v[1435] = if s.b[1435] { 1.0 } else { 0.0 };

        if (((s.b[1424] && s.b[1432]) && s.b[1434]) && s.b[1435]) {
            s.store_limited_exp(15, 16);
        }

        s.b[1436] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));
        s.v[1436] = if s.b[1436] { 1.0 } else { 0.0 };

        if ((((s.b[1424] && s.b[1432]) && s.b[1434]) && (!s.b[1435])) && s.b[1436]) {
            s.store_limited_exp(15, 20);
        }

        if ((((s.b[1424] && s.b[1432]) && s.b[1434]) && (!s.b[1435])) && (!s.b[1436])) {
            s.store_div_ad_lhs(14, A::sub(s.ad_value(20), s.ad_value(16)), 17);
            s.store_square(18, 14);
            s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add_scaled_product(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), 1.0, s.ad_value(18), A::sub_from_scalar((15.0 / 16.0), A::mul(s.ad_value(18), A::sub_from_scalar(1.25, s.ad_value(18)))), 1.0), 1.0));
        }

        if ((s.b[1424] && s.b[1432]) && s.b[1434]) {
            s.store_mul_ad_rhs(768, 15, A::add_scaled_inputs3(A::offset(s.ad_value(13), 1.0), 1.0, s.ad_value(20), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), -1.0));
        }

        if ((s.b[1424] && s.b[1432]) && (!s.b[1434])) {
            s.store_limited_exp(15, 20);
            s.store_div_from_scalar(95, 1.0, 94);
            s.store_ad_value(16, A::add_scaled_inputs3(s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(13), -1.0));
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0)));
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
            s.store_ad_value(16, A::add_scaled_inputs3(s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(13), -1.0));
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0)));
            s.store_scaled_mul_ad(18, A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0)), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0)), 1.0);
            s.store_sub_ad_lhs(19, A::add_scaled_product(A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-1.0)), 18);
            s.store_ad_value(768, A::add_scaled_product(s.ad_value(15), 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::offset(A::div(A::mul(s.ad_value(16), s.ad_value(19)), A::mul_scaled_lhs(s.ad_value(17), 2.0, s.ad_value(17))), 1.0), (-1.0)));
        }

        s.b[1437] = ((1.0 == 0.0) && (s.v[767] < ((-2500.0) * 2.0)));
        s.v[1437] = if s.b[1437] { 1.0 } else { 0.0 };

        if ((s.b[1424] && s.b[1432]) && s.b[1437]) {
            s.store_div_from_scalar_scaled_input(769, ((-2.0) * 2.0), 767, 16.0);
        }

        if ((s.b[1424] && s.b[1432]) && (!s.b[1437])) {
            s.store_scaled_add_ad(769, A::offset(s.ad_value(767), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(767), (-1.0)), A::offset(s.ad_value(767), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        if (s.b[1424] && s.b[1432]) {
            s.store_sqrt(770, 769);
            s.store_sub_scaled_inputs(771, 767, 1.0, 768, 2.0);
        }

        s.b[1438] = ((1.0 == 0.0) && (s.v[771] < ((-2500.0) * 2.0)));
        s.v[1438] = if s.b[1438] { 1.0 } else { 0.0 };

        if ((s.b[1424] && s.b[1432]) && s.b[1438]) {
            s.store_div_from_scalar_scaled_input(12, ((-2.0) * 2.0), 771, 16.0);
        }

        if ((s.b[1424] && s.b[1432]) && (!s.b[1438])) {
            s.store_scaled_add_ad(12, A::offset(s.ad_value(771), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(771), (-1.0)), A::offset(s.ad_value(771), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        if (s.b[1424] && s.b[1432]) {
            s.store_offset_div_ad(772, s.ad_value(765), A::add(s.ad_value(770), A::sqrt(s.ad_value(12))), 1.0);
            s.store_sub_scaled_inputs(773, 767, 1.0, 768, 2.0);
            s.store_ad_value(777, A::mul3(A::div_from_scalar(((((p.p2 * s.v[33]) * p.p1115) * 8.85418e-12) * p.p111), s.ad_value(229)), s.ad_value(108), A::add_scaled_product(A::sub(s.ad_value(764), s.ad_value(773)), 1.0, s.ad_value(772), s.ad_value(768), (-2.0))));
        }

        s.b[1439] = (p.p1118 > 0.0);
        s.v[1439] = if s.b[1439] { 1.0 } else { 0.0 };

        if ((s.b[1424] && s.b[1432]) && s.b[1439]) {
            s.store_offset_scaled(13, 768, 1.0 / (p.p1119), 1.0);
            s.store_div_from_scalar(14, (p.p1118 * 1.9e-9), 13);
            s.store_div_from_scalar_ad(12, (3.9 * 8.85418e-12), A::add_scaled_inputs(s.ad_value(229), (3.9 * 1.0 / (p.p111)), s.ad_value(14), 1.0 / (s.v[47])));
        }

        if ((s.b[1424] && s.b[1432]) && (!s.b[1439])) {
            s.store_div_from_scalar(12, (8.85418e-12 * p.p111), 229);
        }

        if (s.b[1424] && s.b[1432]) {
            s.store_mul_ad_lhs(776, A::mul3_scaled_output(s.ad_value(772), s.ad_value(108), s.ad_value(12), (((p.p2 * s.v[33]) * p.p1116) * 2.0)), 768);
        }

        s.v[254] = 0.0;

        s.b[1440] = (p.p7 > 1.0);
        s.v[1440] = if s.b[1440] { 1.0 } else { 0.0 };

        if s.b[1440] {
            s.store_scaled_mul(255, 157, 80, ((s.v[29] * 1.0 / (s.v[30])) * s.v[46]));
            s.store_scale(21, 108, p.p755);
            s.store_scaled_mul(12, 21, 157, ((s.v[29] * 1.0 / (s.v[30])) * s.v[46]));
            s.store_scaled_add(254, 12, 255, (p.p754 * p.p2));
        }

        s.b[1441] = (p.p7 == 2.0);
        s.v[1441] = if s.b[1441] { 1.0 } else { 0.0 };

        if (s.b[1440] && s.b[1441]) {
            s.store_div_from_scalar(253, 1.0, 252);
        }

        s.b[1442] = (s.v[253] < p.p1093);
        s.v[1442] = if s.b[1442] { 1.0 } else { 0.0 };

        if ((s.b[1440] && s.b[1441]) && s.b[1442]) {
            s.store_scalar(253, p.p1093);
            s.store_div_from_scalar(252, 1.0, 253);
        }

        if (s.b[1440] && s.b[1441]) {
            s.store_add(23, 252, 254);
            s.store_div_ad_lhs(254, A::mul(s.ad_value(252), s.ad_value(254)), 23);
        }

        s.b[1443] = (p.p1094 == 0.0);
        s.v[1443] = if s.b[1443] { 1.0 } else { 0.0 };

        s.b[1444] = ((s.v[553] <= 0.0) || (s.v[558] <= 0.0));
        s.v[1444] = if s.b[1444] { 1.0 } else { 0.0 };

        if (s.b[1443] && s.b[1444]) {
            s.store_scalar(178, 0.0);
        }

        s.b[1445] = (s.v[167] > (s.v[558] / 80.0));
        s.v[1445] = if s.b[1445] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (!s.b[1444])) && s.b[1445]) {
            s.store_scaled_div(13, 558, 167, -1.0);
            s.store_div_ad_lhs(178, A::mul(A::mul3(s.ad_value(553), s.ad_value(167), s.ad_value(188)), A::limited_exp(s.ad_value(13))), 177);
        }

        if ((s.b[1443] && (!s.b[1444])) && (!s.b[1445])) {
            s.store_div_ad_lhs(178, A::mul3_scaled_output(s.ad_value(553), s.ad_value(167), s.ad_value(188), 1.804851387e-35), 177);
        }

        s.b[1446] = (p.p1094 == 1.0);
        s.v[1446] = if s.b[1446] { 1.0 } else { 0.0 };

        if ((!s.b[1443]) && s.b[1446]) {
            s.store_mul3_affine_lhs(184, 555, 74, 1.0, 1.0, 140);
            s.store_pow_ad(19, A::offset(A::div(s.ad_value(74), s.ad_value(184)), 1e-6), A::div_from_scalar(1.0, s.ad_value(412)));
            s.store_pow_ad(20, A::offset(s.ad_value(19), 1.0), A::neg(s.ad_value(412)));
            s.store_mul(183, 74, 20);
            s.store_sub(185, 74, 183);
        }

        s.b[1447] = ((0.0 == 0.0) && (s.v[185] < ((-2500.0) * 0.001)));
        s.v[1447] = if s.b[1447] { 1.0 } else { 0.0 };

        if (((!s.b[1443]) && s.b[1446]) && s.b[1447]) {
            s.store_div_from_scalar_scaled_input(185, ((-0.001) * 0.001), 185, 16.0);
        }

        if (((!s.b[1443]) && s.b[1446]) && (!s.b[1447])) {
            s.store_scaled_add_ad_rhs(185, 185, A::sqrt(A::offset(A::mul(s.ad_value(185), s.ad_value(185)), ((0.25 * 0.001) * 0.001))), 0.5);
        }

        if ((!s.b[1443]) && s.b[1446]) {
            s.store_mul_scaled_ad_rhs(181, 558, 0.5, A::offset(A::powf(s.ad_value(183), s.v[556]), 1.0));
            s.store_offset_scaled_ad(13, A::limited_exp_scaled_input(s.ad_value(76), p.p492), p.p493, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_15(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        if ((!s.b[1443]) && s.b[1446]) {
            s.store_div(182, 553, 13);
            s.store_mul_ad_rhs(14, 182, A::add_scaled_product(A::scale_offset(s.ad_value(61), p.p505, 1.0), 1.0, s.ad_value(61), s.ad_value(61), p.p506));
        }

        s.b[1448] = ((0.0 == 0.0) && (s.v[14] < ((-2500.0) * 1e-12)));
        s.v[1448] = if s.b[1448] { 1.0 } else { 0.0 };

        if (((!s.b[1443]) && s.b[1446]) && s.b[1448]) {
            s.store_div_from_scalar_scaled_input(182, ((-1e-12) * 1e-12), 14, 16.0);
        }

        if (((!s.b[1443]) && s.b[1446]) && (!s.b[1448])) {
            s.store_scaled_add_ad_rhs(182, 14, A::sqrt(A::offset(A::mul(s.ad_value(14), s.ad_value(14)), ((0.25 * 1e-12) * 1e-12))), 0.5);
        }

        s.b[1449] = ((s.v[553] <= 0.0) || (s.v[558] <= 0.0));
        s.v[1449] = if s.b[1449] { 1.0 } else { 0.0 };

        if (((!s.b[1443]) && s.b[1446]) && s.b[1449]) {
            s.store_scalar(178, 0.0);
        }

        s.b[1450] = (s.v[185] > (s.v[181] / 80.0));
        s.v[1450] = if s.b[1450] { 1.0 } else { 0.0 };

        if ((((!s.b[1443]) && s.b[1446]) && (!s.b[1449])) && s.b[1450]) {
            s.store_ad_value(13, A::div_scaled_inputs(s.ad_value(181), -1.0, A::powf(s.ad_value(185), p.p524), 1.0));
            s.store_div_ad_lhs(178, A::mul(A::mul3(s.ad_value(182), s.ad_value(185), s.ad_value(188)), A::limited_exp(s.ad_value(13))), 177);
        }

        if ((((!s.b[1443]) && s.b[1446]) && (!s.b[1449])) && (!s.b[1450])) {
            s.store_div_ad_lhs(178, A::mul3_scaled_output(s.ad_value(182), s.ad_value(185), s.ad_value(188), 1.804851387e-35), 177);
        }

        s.b[1451] = ((p.p1094 == 1.0) && (p.p1098 == 1.0));
        s.v[1451] = if s.b[1451] { 1.0 } else { 0.0 };

        if s.b[1451] {
            s.store_offset(13, 200, (-p.p1105));
        }

        s.b[1452] = ((0.1 == 0.0) && (s.v[13] < ((-2500.0) * 2.0)));
        s.v[1452] = if s.b[1452] { 1.0 } else { 0.0 };

        if (s.b[1451] && s.b[1452]) {
            s.store_div_from_scalar_scaled_input(13, ((-2.0) * 2.0), 13, 16.0);
        }

        if (s.b[1451] && (!s.b[1452])) {
            s.store_scaled_add_ad(13, A::offset(s.ad_value(13), 0.1), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(13), (-0.1)), A::offset(s.ad_value(13), (-0.1))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        if s.b[1451] {
            s.store_ad_value(14, A::div_scaled_inputs(s.ad_value(13), (10.0 * p.p1106), A::offset(s.ad_value(13), (10.0 * p.p1106)), 1.0));
            s.store_mul_ad_rhs(754, 763, A::scale_offset(s.ad_value(14), p.p1104, 1.0));
            s.store_scaled_div(778, 188, 754, ((p.p502) * 1.0 / (((p.p2 * s.v[29]) * 1.60219e-19))));
            s.store_offset_scaled(779, 778, 1.0 / (p.p1099), (-1.0));
        }

        s.b[1453] = ((0.0 == 0.0) && (s.v[779] < ((-2500.0) * p.p504)));
        s.v[1453] = if s.b[1453] { 1.0 } else { 0.0 };

        if (s.b[1451] && s.b[1453]) {
            s.store_div_from_scalar_scaled_input(779, ((-p.p504) * p.p504), 779, 16.0);
        }

        if (s.b[1451] && (!s.b[1453])) {
            s.store_scaled_add_ad_rhs(779, 779, A::sqrt(A::offset(A::mul(s.ad_value(779), s.ad_value(779)), ((0.25 * p.p504) * p.p504))), 0.5);
        }

        if s.b[1451] {
            s.store_scale(779, 779, p.p1099);
        }

        s.b[1454] = (p.p514 > 0.0);
        s.v[1454] = if s.b[1454] { 1.0 } else { 0.0 };

        s.b[1455] = ((0.0 == 0.0) && (((((s.v[187] * (nv0 - nv2)) - (p.p512 * s.v[183])) - p.p503) - (p.p514 * ((s.v[760]) as f64).powf(p.p513))) < ((-2500.0) * 0.05)));
        s.v[1455] = if s.b[1455] { 1.0 } else { 0.0 };

        if ((s.b[1451] && s.b[1454]) && s.b[1455]) {
            s.store_div_from_scalar_ad(14, ((-0.05) * 0.05), A::sub_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(183), (-p.p512), s.ad_value(187), A::voltage(ctx, nodes, Some(0), Some(2)), 1.0), (-p.p503)), 16.0, A::powf(s.ad_value(760), p.p513), (p.p514 * 16.0)));
        }

        if ((s.b[1451] && s.b[1454]) && (!s.b[1455])) {
            let assign23370_ad_e32293: A = A::mul(A::sub_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(183), (-p.p512), s.ad_value(187), A::voltage(ctx, nodes, Some(0), Some(2)), 1.0), (-p.p503)), 1.0, A::powf(s.ad_value(760), p.p513), p.p514), A::sub_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(183), (-p.p512), s.ad_value(187), A::voltage(ctx, nodes, Some(0), Some(2)), 1.0), (-p.p503)), 1.0, A::powf(s.ad_value(760), p.p513), p.p514));
            s.store_ad_value(14, A::add_scaled_inputs3(A::offset(A::add_scaled_product(s.ad_value(183), (-p.p512), s.ad_value(187), A::voltage(ctx, nodes, Some(0), Some(2)), 1.0), (-p.p503)), 0.5, A::powf(s.ad_value(760), p.p513), ((-p.p514) * 0.5), A::sqrt(A::offset(assign23370_ad_e32293, ((0.25 * 0.05) * 0.05))), 0.5));
        }

        s.b[1456] = ((0.0 == 0.0) && ((((s.v[187] * (nv0 - nv2)) - (p.p512 * s.v[183])) - p.p503) < ((-2500.0) * 0.05)));
        s.v[1456] = if s.b[1456] { 1.0 } else { 0.0 };

        if ((s.b[1451] && (!s.b[1454])) && s.b[1456]) {
            s.store_div_from_scalar_ad(14, ((-0.05) * 0.05), A::scaled_offset(A::add_scaled_product(s.ad_value(183), (-p.p512), s.ad_value(187), A::voltage(ctx, nodes, Some(0), Some(2)), 1.0), (-p.p503), 16.0));
        }

        if ((s.b[1451] && (!s.b[1454])) && (!s.b[1456])) {
            let assign23400_ad_e32396: A = A::add(A::offset(A::add_scaled_product(s.ad_value(183), (-p.p512), s.ad_value(187), A::voltage(ctx, nodes, Some(0), Some(2)), 1.0), (-p.p503)), A::sqrt(A::offset(A::mul(A::offset(A::add_scaled_product(s.ad_value(183), (-p.p512), s.ad_value(187), A::voltage(ctx, nodes, Some(0), Some(2)), 1.0), (-p.p503)), A::offset(A::add_scaled_product(s.ad_value(183), (-p.p512), s.ad_value(187), A::voltage(ctx, nodes, Some(0), Some(2)), 1.0), (-p.p503))), ((0.25 * 0.05) * 0.05))));
            s.store_scale_ad(14, assign23400_ad_e32396, 0.5);
        }

        if s.b[1451] {
            s.store_scale(15, 779, ((2.0 * 1.60219e-19) / (p.p110 * 8.85418e-12)));
            s.store_powf_ad(15, A::mul(s.ad_value(15), s.ad_value(14)), 0.5);
            s.store_ad_value(16, A::add_scaled_product(s.ad_value(61), p.p507, s.ad_value(61), s.ad_value(61), p.p508));
            s.store_ad_value(17, A::add_scaled_inputs(s.ad_value(14), p.p509, A::powf(s.ad_value(14), p.p511), p.p510));
            s.store_scaled_add_ad_lhs(18, A::offset(s.ad_value(16), 1.0), 17, p.p500);
        }

        s.b[1457] = ((0.0 == 0.0) && (s.v[18] < ((-2500.0) * 1e-12)));
        s.v[1457] = if s.b[1457] { 1.0 } else { 0.0 };

        if (s.b[1451] && s.b[1457]) {
            s.store_div_from_scalar_scaled_input(186, ((-1e-12) * 1e-12), 18, 16.0);
        }

        if (s.b[1451] && (!s.b[1457])) {
            s.store_scaled_add_ad_rhs(186, 18, A::sqrt(A::offset(A::mul(s.ad_value(18), s.ad_value(18)), ((0.25 * 1e-12) * 1e-12))), 0.5);
        }

        s.b[1458] = (s.v[15] > (p.p501 / 80.0));
        s.v[1458] = if s.b[1458] { 1.0 } else { 0.0 };

        if (s.b[1451] && s.b[1458]) {
            s.store_div_from_scalar(13, (-p.p501), 15);
            s.store_mul_ad(780, A::mul3(s.ad_value(186), s.ad_value(15), s.ad_value(188)), A::limited_exp(s.ad_value(13)));
        }

        if (s.b[1451] && (!s.b[1458])) {
            s.store_mul3_affine_lhs(780, 186, 15, 1.804851387e-35, 0.0, 188);
        }

        s.store_scaled_mul(824, 178, 187, p.p28);

        s.v[283] = 0.0;

        s.v[284] = 0.0;

        s.v[285] = 0.0;

        s.v[286] = 0.0;

        s.v[287] = 0.0;

        s.v[290] = 0.0;

        s.v[291] = 0.0;

        s.b[1459] = ((p.p46 != 0.0) || (p.p47 != 0.0));
        s.v[1459] = if s.b[1459] { 1.0 } else { 0.0 };

        if s.b[1459] {
            s.store_mul_add_ad_rhs(277, 106, A::add_scaled_inputs3(s.ad_value(59), 1.0, s.ad_value(91), (-1.0), s.ad_value(200), 1.0), s.ad_value(144));
            s.store_sqrt_square_offset(13, 277, 0.0001);
            s.store_scaled_sub(279, 13, 277, 0.5);
            s.store_scaled_add(278, 277, 13, 0.5);
        }

        s.b[1460] = (p.p47 != 0.0);
        s.v[1460] = if s.b[1460] { 1.0 } else { 0.0 };

        if (s.b[1459] && s.b[1460]) {
            s.store_div_ad_lhs(13, A::div(s.ad_value(277), s.ad_value(589)), 108);
        }

        if (s.b[1459] && s.b[1460]) {
            let assign23680_ad_e32660: A = {
                if ((!((-s.v[13]) > 37.0)) && (!((-s.v[13]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::neg(s.ad_value(13)))
                } else {
                    {
                        if ((!((-s.v[13]) > 37.0)) && ((-s.v[13]) < (-37.0))) {
                            A::exp_scaled_input(s.ad_value(13), -1.0)
                        } else {
                            {
                                if ((-s.v[13]) > 37.0) {
                                    A::neg(s.ad_value(13))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_mul_ad_product_rhs(282, 589, s.ad_value(108), assign23680_ad_e32660);
        }

        if (s.b[1459] && s.b[1460]) {
            s.store_ad_value(14, A::add_scaled_product(s.ad_value(586), 1.0, s.ad_value(587), s.ad_value(279), (-1.0)));
            s.store_offset_mul(15, 588, 279, 1.0);
            s.store_scaled_mul(16, 14, 15, ((-745669000000.0) * p.p77));
            s.store_limited_exp(17, 16);
            s.store_scalar(18, 4.97232e-7);
            s.store_mul_ad_product_lhs(284, A::mul3_scaled_output(s.ad_value(18), s.ad_value(298), s.ad_value(64), ((p.p2 * s.v[29]) * s.v[30])), s.ad_value(282), 17);
            s.store_mul(284, 284, 419);
            s.store_div_ad_lhs(13, A::div(A::sub(s.ad_value(277), s.ad_value(584)), s.ad_value(585)), 108);
        }

        if (s.b[1459] && s.b[1460]) {
            let assign23770_ad_e32789: A = {
                if ((!(s.v[13] > 37.0)) && (!(s.v[13] < (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(13))
                } else {
                    {
                        if ((!(s.v[13] > 37.0)) && (s.v[13] < (-37.0))) {
                            A::exp(s.ad_value(13))
                        } else {
                            {
                                if (s.v[13] > 37.0) {
                                    s.ad_value(13)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_mul_ad_product_rhs(281, 585, s.ad_value(108), assign23770_ad_e32789);
        }

        if (s.b[1459] && s.b[1460]) {
            s.store_ad_value(14, A::add_scaled_product(s.ad_value(581), 1.0, s.ad_value(582), s.ad_value(278), (-1.0)));
            s.store_offset_mul(15, 583, 278, 1.0);
            s.store_scaled_mul(16, 14, 15, ((-982222000000.0) * p.p77));
            s.store_limited_exp(17, 16);
            s.store_scalar(18, 3.75956e-7);
            s.store_mul_ad_product_lhs(283, A::mul3_scaled_output(s.ad_value(18), s.ad_value(298), s.ad_value(64), ((p.p2 * s.v[29]) * s.v[30])), s.ad_value(281), 17);
            s.store_mul(283, 283, 419);
            s.store_add(285, 284, 283);
        }

        s.b[1461] = (p.p46 != 0.0);
        s.v[1461] = if s.b[1461] { 1.0 } else { 0.0 };

        if (s.b[1459] && s.b[1461]) {
            s.store_ad_value(13, A::add_scaled_product(s.ad_value(590), 1.0, s.ad_value(591), s.ad_value(278), (-1.0)));
            s.store_offset_mul(14, 592, 278, 1.0);
            s.store_scaled_mul(15, 13, 14, s.v[295]);
            s.store_mul_ad(16, A::mul3(s.ad_value(90), s.ad_value(106), A::add(s.ad_value(200), s.ad_value(144))), A::limited_exp(s.ad_value(15)));
            s.store_mul_ad_lhs(288, A::mul3_scaled_output(s.ad_value(294), s.ad_value(16), A::sub(A::add_scaled_inputs(s.ad_value(64), 1.0, s.ad_value(76), 0.5), A::add_scaled_inputs(s.ad_value(70), 0.5, s.ad_value(66), 0.5)), p.p2), 419);
            s.store_offset_sqrt_ad(280, A::offset(A::square(s.ad_value(139)), 0.01), (-0.1));
            s.store_scale(13, 280, s.v[600]);
            s.store_limited_exp_neg_input(289, 13);
            s.store_offset_add(15, 13, 289, (((-1.0)) + (0.0001)));
            s.store_offset_sub_from_scalar_ad(16, 1.0, A::mul(A::offset(s.ad_value(13), 1.0), s.ad_value(289)), 0.0001);
            s.store_offset_square(17, 13, 0.0002);
        }

        s.b[1462] = (s.v[57] > 0.0);
        s.v[1462] = if s.b[1462] { 1.0 } else { 0.0 };

        if ((s.b[1459] && s.b[1461]) && s.b[1462]) {
            s.store_div_ad_lhs(287, A::mul(s.ad_value(288), s.ad_value(16)), 17);
            s.store_div_ad_lhs(286, A::mul(s.ad_value(288), s.ad_value(15)), 17);
        }

        if ((s.b[1459] && s.b[1461]) && (!s.b[1462])) {
            s.store_div_ad_lhs(286, A::mul(s.ad_value(288), s.ad_value(16)), 17);
            s.store_div_ad_lhs(287, A::mul(s.ad_value(288), s.ad_value(15)), 17);
        }

        if (s.b[1459] && s.b[1461]) {
            s.store_sub(14, 52, 63);
            s.store_sqrt_square_offset(77, 14, 0.0001);
        }

        s.b[1463] = (p.p1041 == 1.0);
        s.v[1463] = if s.b[1463] { 1.0 } else { 0.0 };

        if ((s.b[1459] && s.b[1461]) && s.b[1463]) {
            let assign24060_ad_e33151: A = {
                if (!((s.v[593] - (s.v[594] * s.v[77])) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::add_scaled_product(s.ad_value(593), 1.0, s.ad_value(594), s.ad_value(77), (-1.0)), 0.5, A::sqrt(A::offset(A::mul(A::add_scaled_product(s.ad_value(593), 1.0, s.ad_value(594), s.ad_value(77), (-1.0)), A::add_scaled_product(s.ad_value(593), 1.0, s.ad_value(594), s.ad_value(77), (-1.0))), ((4.0 * 1e-6) * 1e-6))), 0.5)
                } else {
                    {
                        if ((s.v[593] - (s.v[594] * s.v[77])) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::add_scaled_product(s.ad_value(593), 1.0, s.ad_value(594), s.ad_value(77), (-1.0)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad_value(13, assign24060_ad_e33151);
        }

        s.b[1464] = (s.v[595] < 0.01);
        s.v[1464] = if s.b[1464] { 1.0 } else { 0.0 };

        if (((s.b[1459] && s.b[1461]) && s.b[1463]) && s.b[1464]) {
            s.store_scalar(595, 0.01);
        }

        if ((s.b[1459] && s.b[1461]) && (!s.b[1463])) {
            s.store_ad_value(13, A::add_scaled_product(s.ad_value(593), 1.0, s.ad_value(594), s.ad_value(77), (-1.0)));
        }

        if (s.b[1459] && s.b[1461]) {
            s.store_offset_mul(14, 595, 77, 1.0);
            s.store_mul3_lhs(15, 297, 13, 14);
            s.store_limited_exp(16, 15);
            s.store_mul3_affine_lhs(292, 419, 296, p.p2, 0.0, 601);
            s.store_mul_ad_lhs(290, A::mul3(s.ad_value(292), s.ad_value(52), s.ad_value(77)), 16);
            s.store_sub(14, 54, 63);
            s.store_sqrt_square_offset(78, 14, 0.0001);
        }

        s.b[1465] = (p.p1041 == 1.0);
        s.v[1465] = if s.b[1465] { 1.0 } else { 0.0 };

        if ((s.b[1459] && s.b[1461]) && s.b[1465]) {
            let assign24180_ad_e33317: A = {
                if (!((s.v[596] - (s.v[597] * s.v[78])) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::add_scaled_product(s.ad_value(596), 1.0, s.ad_value(597), s.ad_value(78), (-1.0)), 0.5, A::sqrt(A::offset(A::mul(A::add_scaled_product(s.ad_value(596), 1.0, s.ad_value(597), s.ad_value(78), (-1.0)), A::add_scaled_product(s.ad_value(596), 1.0, s.ad_value(597), s.ad_value(78), (-1.0))), ((4.0 * 1e-6) * 1e-6))), 0.5)
                } else {
                    {
                        if ((s.v[596] - (s.v[597] * s.v[78])) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::add_scaled_product(s.ad_value(596), 1.0, s.ad_value(597), s.ad_value(78), (-1.0)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad_value(13, assign24180_ad_e33317);
        }

        s.b[1466] = (s.v[598] < 0.01);
        s.v[1466] = if s.b[1466] { 1.0 } else { 0.0 };

        if (((s.b[1459] && s.b[1461]) && s.b[1465]) && s.b[1466]) {
            s.store_scalar(598, 0.01);
        }

        if ((s.b[1459] && s.b[1461]) && (!s.b[1465])) {
            s.store_ad_value(13, A::add_scaled_product(s.ad_value(596), 1.0, s.ad_value(597), s.ad_value(78), (-1.0)));
        }

        if (s.b[1459] && s.b[1461]) {
            s.store_offset_mul(14, 598, 78, 1.0);
            s.store_mul3_lhs(15, 297, 13, 14);
            s.store_limited_exp(16, 15);
            s.store_mul3_affine_lhs(293, 419, 296, p.p2, 0.0, 602);
            s.store_mul_ad_lhs(291, A::mul3(s.ad_value(293), s.ad_value(54), s.ad_value(78)), 16);
        }

        s.store_scaled_mul(827, 187, 290, p.p28);

        s.store_scaled_mul(828, 187, 291, p.p28);

        s.store_scaled_mul(831, 187, 285, p.p28);

        s.store_scaled_mul(829, 187, 286, p.p28);

        s.store_scaled_mul(830, 187, 287, p.p28);

        s.v[180] = 0.0;

    }
}
