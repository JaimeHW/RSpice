#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_add_scaled_inputs3_offset_indices(539, 476, p[343], 477, p[344], 478, p[345], p[340]);s.store_primal_add_scaled_inputs3_offset_indices(542, 476, p[354], 477, p[355], 478, p[356], p[351]);s.store_primal_add_scaled_inputs3_offset_indices(531, 476, p[394], 477, p[395], 478, p[396], p[393]);s.store_primal_add_scaled_inputs3_offset_indices(530, 476, p[404], 477, p[405], 478, p[406], p[403]);s.store_primal_add_scaled_inputs3_offset_indices(526, 476, p[376], 477, p[377], 478, p[378], p[375]);s.store_primal_add_scaled_inputs3_offset_indices(543, 476, p[380], 477, p[381], 478, p[382], p[379]);s.store_primal_add_scaled_inputs3_offset_indices(527, 476, p[386], 477, p[387], 478, p[388], p[385]);s.store_primal_add_scaled_inputs3_offset_indices(529, 476, p[390], 477, p[391], 478, p[392], p[389]);s.store_primal_add_scaled_inputs3_offset_indices(528, 476, p[400], 477, p[401], 478, p[402], p[399]);s.store_primal_add_scaled_inputs3_offset_indices(532, 476, p[416], 477, p[417], 478, p[418], p[413]);s.store_primal_add_scaled_inputs3_offset_indices(533, 476, p[410], 477, p[411], 478, p[412], p[409]);s.store_add_scaled_inputs3_offset_indices(534, 476, p[435], 477, p[436], 478, p[437], p[434]);s.store_add_scaled_inputs3_offset_indices(517, 476, p[463], 477, p[464], 478, p[465], p[460]);s.store_primal_add_scaled_inputs3_offset_indices(520, 476, p[471], 477, p[472], 478, p[473], p[470]);s.store_primal_add_scaled_inputs3_offset_indices(521, 476, p[358], 477, p[359], 478, p[360], p[357]);s.store_primal_add_scaled_inputs3_offset_indices(522, 476, p[362], 477, p[363], 478, p[364], p[361]);s.store_primal_add_scaled_inputs3_offset_indices(523, 476, p[366], 477, p[367], 478, p[368], p[365]);s.store_primal_add_scaled_inputs3_offset_indices(524, 476, p[371], 477, p[372], 478, p[373], p[370]);s.store_primal_add_scaled_inputs3_offset_indices(525, 476, p[481], 477, p[482], 478, p[483], p[478]);s.store_primal_add_scaled_inputs3_offset_indices(537, 476, p[475], 477, p[476], 478, p[477], p[474]);s.store_add_scaled_inputs3_offset_indices(500, 476, p[240], 477, p[241], 478, p[242], p[239]);s.store_primal_add_scaled_inputs3_offset_indices(164, 476, p[420], 477, p[421], 478, p[422], p[419]);s.store_add_scaled_inputs3_offset_indices(503, 476, p[260], 477, p[261], 478, p[262], p[259]);s.store_primal_add_scaled_inputs3_offset_indices(544, 476, p[667], 477, p[668], 478, p[669], p[666]);s.store_primal_add_scaled_inputs3_offset_indices(545, 476, p[675], 477, p[676], 478, p[677], p[674]);s.store_primal_add_scaled_inputs3_offset_indices(546, 476, p[679], 477, p[680], 478, p[681], p[678]);s.store_primal_add_scaled_inputs3_offset_indices(547, 476, p[683], 477, p[684], 478, p[685], p[682]);s.store_primal_add_scaled_inputs3_offset_indices(548, 476, p[687], 477, p[688], 478, p[689], p[686]);s.store_add_scaled_inputs3_offset_indices(551, 476, p[489], 477, p[490], 478, p[491], p[484]);s.store_add_scaled_inputs3_offset_indices(554, 476, p[497], 477, p[498], 478, p[499], p[494]);s.store_primal_add_scaled_inputs3_offset_indices(578, 476, p[936], 477, p[937], 478, p[938], p[935]);s.store_primal_add_scaled_inputs3_offset_indices(579, 476, p[940], 477, p[941], 478, p[942], p[939]);s.store_primal_add_scaled_inputs3_offset_indices(580, 476, p[944], 477, p[945], 478, p[946], p[943]);s.store_primal_add_scaled_inputs3_offset_indices(559, 476, p[633], 477, p[634], 478, p[635], p[630]);s.store_primal_add_scaled_inputs3_offset_indices(560, 476, p[637], 477, p[638], 478, p[639], p[636]);s.store_primal_add_scaled_inputs3_offset_indices(561, 476, p[641], 477, p[642], 478, p[643], p[640]);s.store_primal_add_scaled_inputs3_offset_indices(562, 476, p[645], 477, p[646], 478, p[647], p[644]);s.store_primal_add_scaled_inputs3_offset_indices(563, 476, p[651], 477, p[652], 478, p[653], p[648]);s.store_primal_add_scaled_inputs3_offset_indices(564, 476, p[655], 477, p[656], 478, p[657], p[654]);s.store_primal_add_scaled_inputs3_offset_indices(565, 476, p[659], 477, p[660], 478, p[661], p[658]);
        s.store_primal_add_scaled_inputs3_offset_indices(566, 476, p[663], 477, p[664], 478, p[665], p[662]);s.store_primal_add_scaled_inputs3_offset_indices(567, 476, p[825], 477, p[826], 478, p[827], p[824]);s.store_primal_add_scaled_inputs3_offset_indices(568, 476, p[830], 477, p[831], 478, p[832], p[829]);s.store_primal_add_scaled_inputs3_offset_indices(569, 476, p[835], 477, p[836], 478, p[837], p[834]);s.store_primal_add_scaled_inputs3_offset_indices(570, 476, p[839], 477, p[840], 478, p[841], p[838]);s.store_primal_add_scaled_inputs3_offset_indices(577, 476, p[844], 477, p[845], 478, p[846], p[843]);s.store_primal_add_scaled_inputs3_offset_indices(571, 476, p[848], 477, p[849], 478, p[850], p[847]);s.store_primal_add_scaled_inputs3_offset_indices(572, 476, p[853], 477, p[854], 478, p[855], p[852]);s.store_primal_add_scaled_inputs3_offset_indices(573, 476, p[857], 477, p[858], 478, p[859], p[856]);s.store_primal_add_scaled_inputs3_offset_indices(574, 476, p[863], 477, p[864], 478, p[865], p[862]);s.store_primal_add_scaled_inputs3_offset_indices(575, 476, p[878], 477, p[879], 478, p[880], p[877]);s.store_primal_add_scaled_inputs3_offset_indices(576, 476, p[886], 477, p[887], 478, p[888], p[885]);s.store_primal_add_scaled_inputs3_offset_indices(581, 476, p[564], 477, p[565], 478, p[566], p[537]);s.store_primal_add_scaled_inputs3_offset_indices(582, 476, p[567], 477, p[568], 478, p[569], p[538]);s.store_primal_add_scaled_inputs3_offset_indices(583, 476, p[570], 477, p[571], 478, p[572], p[539]);s.store_primal_add_scaled_inputs3_offset_indices(584, 476, p[573], 477, p[574], 478, p[575], p[540]);s.store_primal_add_scaled_inputs3_offset_indices(585, 476, p[576], 477, p[577], 478, p[578], p[541]);s.store_primal_add_scaled_inputs3_offset_indices(586, 476, p[579], 477, p[580], 478, p[581], p[533]);s.store_primal_add_scaled_inputs3_offset_indices(587, 476, p[582], 477, p[583], 478, p[584], p[534]);s.store_primal_add_scaled_inputs3_offset_indices(588, 476, p[585], 477, p[586], 478, p[587], p[535]);s.store_primal_add_scaled_inputs3_offset_indices(589, 476, p[588], 477, p[589], 478, p[590], p[536]);s.store_primal_add_scaled_inputs3_offset_indices(590, 476, p[591], 477, p[592], 478, p[593], p[542]);s.store_primal_add_scaled_inputs3_offset_indices(591, 476, p[594], 477, p[595], 478, p[596], p[543]);s.store_primal_add_scaled_inputs3_offset_indices(592, 476, p[597], 477, p[598], 478, p[599], p[544]);s.store_primal_add_scaled_inputs3_offset_indices(593, 476, p[600], 477, p[601], 478, p[602], p[545]);s.store_primal_add_scaled_inputs3_offset_indices(594, 476, p[603], 477, p[604], 478, p[605], p[546]);s.store_primal_add_scaled_inputs3_offset_indices(595, 476, p[606], 477, p[607], 478, p[608], p[547]);s.store_primal_add_scaled_inputs3_offset_indices(596, 476, p[609], 477, p[610], 478, p[611], p[548]);s.store_primal_add_scaled_inputs3_offset_indices(597, 476, p[612], 477, p[613], 478, p[614], p[549]);s.store_primal_add_scaled_inputs3_offset_indices(598, 476, p[615], 477, p[616], 478, p[617], p[550]);s.store_primal_add_scaled_inputs3_offset_indices(599, 476, p[618], 477, p[619], 478, p[620], p[553]);s.store_primal_add_scaled_inputs3_offset_indices(454, 476, p[870], 477, p[871], 478, p[872], p[867]);s.store_primal_add_scaled_inputs3_offset_indices(455, 476, p[874], 477, p[875], 478, p[876], p[873]);s.store_primal_add_scaled_inputs3_offset_indices(453, 476, p[430], 477, p[431], 478, p[432], p[425]);s.store_primal_add_scaled_inputs3_offset_indices(148, 476, p[445], 477, p[446], 478, p[447], p[444]);s.store_primal_add_scaled_inputs3_offset_indices(149, 476, p[449], 477, p[450], 478, p[451], p[448]);s.store_primal_add_scaled_inputs3_offset_indices(151, 476, p[453], 477, p[454], 478, p[455], p[452]);s.store_primal_add_scaled_inputs3_offset_indices(152, 476, p[457], 477, p[458], 478, p[459], p[456]);s.store_primal_add_scaled_inputs3_offset_indices(605, 476, p[1047], 477, p[1048], 478, p[1049], p[1046]);s.store_primal_add_scaled_inputs3_offset_indices(606, 476, p[1055], 477, p[1056], 478, p[1057], p[1054]);
        s.store_primal_add_scaled_inputs3_offset_indices(607, 476, p[1051], 477, p[1052], 478, p[1053], p[1050]);s.store_primal_add_scaled_inputs3_offset_indices(608, 476, p[1059], 477, p[1060], 478, p[1061], p[1058]);s.store_primal_add_scaled_inputs3_offset_indices(612, 476, p[967], 477, p[968], 478, p[969], p[966]);s.store_primal_add_scaled_inputs3_offset_indices(686, 476, p[963], 477, p[964], 478, p[965], p[962]);s.store_primal_add_scaled_inputs3_offset_indices(613, 476, p[971], 477, p[972], 478, p[973], p[970]);s.store_primal_add_scaled_inputs3_offset_indices(614, 476, p[975], 477, p[976], 478, p[977], p[974]);s.store_primal_add_scaled_inputs3_offset_indices(615, 476, p[979], 477, p[980], 478, p[981], p[978]);s.store_add_scaled_inputs3_offset_indices(616, 476, p[983], 477, p[984], 478, p[985], p[982]);s.store_primal_add_scaled_inputs3_offset_indices(617, 476, p[987], 477, p[988], 478, p[989], p[986]);s.store_primal_add_scaled_inputs3_offset_indices(618, 476, p[991], 477, p[992], 478, p[993], p[990]);s.store_primal_add_scaled_inputs3_offset_indices(619, 476, p[995], 477, p[996], 478, p[997], p[994]);s.store_primal_add_scaled_inputs3_offset_indices(620, 476, p[999], 477, p[1000], 478, p[1001], p[998]);s.store_primal_add_scaled_inputs3_offset_indices(621, 476, p[1003], 477, p[1004], 478, p[1005], p[1002]);s.store_primal_add_scaled_inputs3_offset_indices(622, 476, p[1007], 477, p[1008], 478, p[1009], p[1006]);s.store_primal_add_scaled_inputs3_offset_indices(623, 476, p[1011], 477, p[1012], 478, p[1013], p[1010]);s.store_add_scaled_inputs3_offset_indices(624, 476, p[1018], 477, p[1019], 478, p[1020], p[1017]);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_primal_add_scaled_inputs3_offset_indices(625, 476, p[1022], 477, p[1023], 478, p[1024], p[1021]);s.store_primal_add_scaled_inputs3_offset_indices(629, 476, p[1030], 477, p[1031], 478, p[1032], p[1029]);s.store_primal_add_scaled_inputs3_offset_indices(630, 476, p[1026], 477, p[1027], 478, p[1028], p[1025]);s.store_primal_add_scaled_inputs3_offset_indices(626, 476, p[1034], 477, p[1035], 478, p[1036], p[1033]);s.store_primal_add_scaled_inputs3_offset_indices(627, 476, p[1038], 477, p[1039], 478, p[1040], p[1037]);s.store_primal_add_scaled_inputs3_offset_indices(631, 476, p[1070], 477, p[1071], 478, p[1072], p[1069]);s.store_primal_add_scaled_inputs3_offset_indices(632, 476, p[1074], 477, p[1075], 478, p[1076], p[1073]);s.store_primal_add_scaled_inputs3_offset_indices(634, 476, p[1078], 477, p[1079], 478, p[1080], p[1077]);s.store_primal_add_scaled_inputs3_offset_indices(635, 476, p[1082], 477, p[1083], 478, p[1084], p[1081]);s.store_primal_add_scaled_inputs3_offset_indices(637, 476, p[1086], 477, p[1087], 478, p[1088], p[1085]);s.store_primal_add_scaled_inputs3_offset_indices(638, 476, p[1090], 477, p[1091], 478, p[1092], p[1089]);s.store_primal_add_scaled_inputs3_offset_indices(640, 476, p[787], 477, p[788], 478, p[789], p[786]);s.store_primal_add_scaled_inputs3_offset_indices(641, 476, p[795], 477, p[796], 478, p[797], p[794]);s.store_primal_add_scaled_inputs3_offset_indices(642, 476, p[791], 477, p[792], 478, p[793], p[790]);s.b[879] = (p[44] != 0.0);s.store_scalar(879, if s.b[879] { 1.0 } else { 0.0 });
        if s.b[879] {s.store_add_scaled_inputs3_offset_indices(485, 476, p[230], 477, p[231], 478, p[232], p[229]);s.store_add_scaled_inputs3_offset_indices(491, 476, p[176], 477, p[177], 478, p[178], p[175]);s.store_primal_add_scaled_inputs3_offset_indices(498, 476, p[280], 477, p[281], 478, p[282], p[279]);s.store_add_scaled_inputs3_offset_indices(505, 476, p[295], 477, p[296], 478, p[297], p[294]);s.store_add_scaled_inputs3_offset_indices(509, 476, p[315], 477, p[316], 478, p[317], p[314]);s.store_primal_add_scaled_inputs3_offset_indices(512, 476, p[323], 477, p[324], 478, p[325], p[322]);s.store_add_scaled_inputs3_offset_indices(515, 476, p[337], 477, p[338], 478, p[339], p[336]);s.store_add_scaled_inputs3_offset_indices(540, 476, p[347], 477, p[348], 478, p[349], p[346]);s.store_add_scaled_inputs3_offset_indices(518, 476, p[467], 477, p[468], 478, p[469], p[466]);s.store_add_scaled_inputs3_offset_indices(501, 476, p[250], 477, p[251], 478, p[252], p[249]);s.store_primal_add_scaled_inputs3_offset_indices(165, 476, p[427], 477, p[428], 478, p[429], p[426]);s.store_add_scaled_inputs3_offset_indices(535, 476, p[441], 477, p[442], 478, p[443], p[440]);s.store_add_scaled_inputs3_offset_indices(552, 476, p[526], 477, p[527], 478, p[528], p[525]);s.store_primal_add_scaled_inputs3_offset_indices(557, 476, p[530], 477, p[531], 478, p[532], p[529]);}
        s.store_scalar(12, ((p[81] * ((((s.v[469]) as f64).powf(p[82]) - ((s.v[474]) as f64).powf(p[82]))).max(0.0)) + (p[83] * ((((s.v[469]) as f64).powf(p[84]) - ((s.v[474]) as f64).powf(p[84]))).max(0.0))));s.store_scalar(13, ((p[85] * ((((s.v[470]) as f64).powf(p[86]) - ((s.v[475]) as f64).powf(p[86]))).max(0.0)) + (p[87] * (((s.v[470] * s.v[469])) as f64).powf(p[88]))));s.store_scale(481, 481, ((1.0 + s.v[12]) + s.v[13]));s.store_scalar(12, (p[214] * ((((s.v[469]) as f64).powf(p[215]) - ((s.v[474]) as f64).powf(p[215]))).max(0.0)));s.store_scalar(13, ((p[216] * ((((s.v[470]) as f64).powf(p[217]) - ((s.v[475]) as f64).powf(p[217]))).max(0.0)) + (p[218] * ((s.v[471]) as f64).powf(p[219]))));s.store_scale(488, 488, ((1.0 + s.v[12]) + s.v[13]));s.store_scalar(12, (1.0 + (p[224] * ((((s.v[469]) as f64).powf(p[225]) - ((s.v[474]) as f64).powf(p[225]))).max(0.0))));s.store_scale(484, 484, s.v[12]);s.b[880] = (p[44] != 0.0);s.store_scalar(880, if s.b[880] { 1.0 } else { 0.0 });
        if s.b[880] {s.store_scale(485, 485, s.v[12]);}
        s.store_primal_scale(487, 487, (1.0 + (p[234] * ((((s.v[469]) as f64).powf(p[235]) - ((s.v[474]) as f64).powf(p[235]))).max(0.0))));s.store_primal_scale(497, 497, p[34]);s.b[881] = (p[50] != 1.0);s.store_scalar(881, if s.b[881] { 1.0 } else { 0.0 });s.b[882] = (p[275] > 0.0);s.store_scalar(882, if s.b[882] { 1.0 } else { 0.0 });
        if (s.b[881] && s.b[882]) {s.store_primal_scale(497, 497, (1.0 - (p[274] * ((((s.v[469]) as f64).powf(p[275]) - ((s.v[474]) as f64).powf(p[275]))).max(0.0))));}
        s.b[883] = (p[44] != 0.0);s.store_scalar(883, if s.b[883] { 1.0 } else { 0.0 });
        if ((s.b[881] && s.b[882]) && s.b[883]) {s.store_primal_scale(498, 498, (1.0 - (p[274] * ((((s.v[469]) as f64).powf(p[275]) - ((s.v[474]) as f64).powf(p[275]))).max(0.0))));}
        if (s.b[881] && (!s.b[882])) {s.store_primal_scale(497, 497, (1.0 - p[274]));}
        s.b[884] = (p[44] != 0.0);s.store_scalar(884, if s.b[884] { 1.0 } else { 0.0 });
        if ((s.b[881] && (!s.b[882])) && s.b[884]) {s.store_primal_scale(498, 498, (1.0 - p[274]));}
        if (!s.b[881]) {s.store_primal_scale(497, 497, ((1.0 - (p[269] * { let limited_exp_arg = ((-s.v[30]) / p[270]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })) - (p[271] * { let limited_exp_arg = ((-s.v[30]) / p[272]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })));}
        s.b[885] = (p[44] != 0.0);s.store_scalar(885, if s.b[885] { 1.0 } else { 0.0 });
        if ((!s.b[881]) && s.b[885]) {s.store_primal_scale(498, 498, ((1.0 - (p[269] * { let limited_exp_arg = ((-s.v[30]) / p[270]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })) - (p[271] * { let limited_exp_arg = ((-s.v[30]) / p[272]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })));}
        s.store_scalar(12, (p[285] * ((((s.v[469]) as f64).powf(p[286]) - ((s.v[474]) as f64).powf(p[286]))).max(0.0)));s.store_scalar(13, ((p[287] * ((((s.v[470]) as f64).powf(p[288]) - ((s.v[475]) as f64).powf(p[288]))).max(0.0)) + (p[289] * ((s.v[471]) as f64).powf(p[290]))));s.store_scale(504, 504, ((1.0 + s.v[12]) + s.v[13]));s.b[886] = (p[44] != 0.0);s.store_scalar(886, if s.b[886] { 1.0 } else { 0.0 });
        if s.b[886] {s.store_scale(505, 505, ((1.0 + s.v[12]) + s.v[13]));}
        s.store_scalar(12, (p[302] * ((((s.v[469]) as f64).powf(p[303]) - ((s.v[474]) as f64).powf(p[303]))).max(0.0)));s.store_scalar(13, ((p[304] * ((((s.v[470]) as f64).powf(p[305]) - ((s.v[475]) as f64).powf(p[305]))).max(0.0)) + (p[306] * ((s.v[471]) as f64).powf(p[307]))));s.store_scale(507, 507, ((1.0 + s.v[12]) + s.v[13]));s.store_scalar(12, (1.0 + (p[309] * ((((s.v[469]) as f64).powf(p[310]) - ((s.v[474]) as f64).powf(p[310]))).max(0.0))));s.store_scale(508, 508, s.v[12]);s.b[887] = (p[44] != 0.0);s.store_scalar(887, if s.b[887] { 1.0 } else { 0.0 });
        if s.b[887] {s.store_scale(509, 509, s.v[12]);}
        s.store_scalar(12, (p[327] * ((((s.v[469]) as f64).powf(p[328]) - ((s.v[474]) as f64).powf(p[328]))).max(0.0)));s.store_scalar(13, ((p[329] * ((((s.v[470]) as f64).powf(p[330]) - ((s.v[475]) as f64).powf(p[330]))).max(0.0)) + (p[331] * ((s.v[471]) as f64).powf(p[332]))));s.store_scale(514, 514, ((1.0 + s.v[12]) + s.v[13]));s.b[888] = (p[44] != 0.0);s.store_scalar(888, if s.b[888] { 1.0 } else { 0.0 });
        if s.b[888] {s.store_scale(515, 515, ((1.0 + s.v[12]) + s.v[13]));}
        s.store_scalar(12, ((((s.v[469]) as f64).powf(p[179]) - ((s.v[474]) as f64).powf(p[179]))).max(0.0));s.store_scale(490, 490, s.v[12]);s.b[889] = (p[44] != 0.0);s.store_scalar(889, if s.b[889] { 1.0 } else { 0.0 });
        if s.b[889] {s.store_scale(491, 491, s.v[12]);}
        s.store_primal_scale(493, 493, ((((s.v[469]) as f64).powf(p[181]) - ((s.v[474]) as f64).powf(p[181]))).max(0.0));s.store_scalar(12, (1.0 + (p[461] * ((((s.v[469]) as f64).powf(p[462]) - ((s.v[474]) as f64).powf(p[462]))).max(0.0))));s.store_scale(517, 517, s.v[12]);s.b[890] = (p[44] != 0.0);s.store_scalar(890, if s.b[890] { 1.0 } else { 0.0 });
        if s.b[890] {s.store_scale(518, 518, s.v[12]);}
        s.store_scale(12, 496, (1.0 + (p[257] * ((((s.v[469]) as f64).powf(p[258]) - ((s.v[474]) as f64).powf(p[258]))).max(0.0))));s.store_min_with_scalar(496, 12, 0.5);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_primal_scale(525, 525, (1.0 + (p[479] * ((((s.v[469]) as f64).powf(p[480]) - ((s.v[474]) as f64).powf(p[480]))).max(0.0))));s.store_scalar(12, (1.0 + (p[341] * ((((s.v[469]) as f64).powf(p[342]) - ((s.v[474]) as f64).powf(p[342]))).max(0.0))));s.store_scale(539, 539, s.v[12]);s.store_max_with_scalar(539, 539, 0.0);s.b[891] = (p[44] != 0.0);s.store_scalar(891, if s.b[891] { 1.0 } else { 0.0 });
        if s.b[891] {s.store_scale(540, 540, s.v[12]);s.store_max_with_scalar(540, 540, 0.0);}
        s.store_scalar(12, (p[243] * ((((s.v[469]) as f64).powf(p[244]) - ((s.v[474]) as f64).powf(p[244]))).max(0.0)));s.store_scalar(13, ((p[245] * ((((s.v[470]) as f64).powf(p[246]) - ((s.v[475]) as f64).powf(p[246]))).max(0.0)) + (p[247] * ((s.v[471]) as f64).powf(p[248]))));s.store_scale(500, 500, ((1.0 + s.v[12]) + s.v[13]));s.b[892] = (p[44] != 0.0);s.store_scalar(892, if s.b[892] { 1.0 } else { 0.0 });
        if s.b[892] {s.store_scale(501, 501, ((1.0 + s.v[12]) + s.v[13]));}
        s.store_primal_max_with_scalar_ad(164, A::scale(s.ad_value(164), (1.0 + (p[423] * ((((s.v[469]) as f64).powf(p[424]) - ((s.v[474]) as f64).powf(p[424]))).max(0.0)))), 0.25);s.b[893] = (p[44] != 0.0);s.store_scalar(893, if s.b[893] { 1.0 } else { 0.0 });
        if s.b[893] {s.store_primal_max_with_scalar_ad(165, A::scale(s.ad_value(165), (1.0 + (p[423] * ((((s.v[469]) as f64).powf(p[424]) - ((s.v[474]) as f64).powf(p[424]))).max(0.0)))), 0.25);}
        s.store_scalar(12, (1.0 + (p[438] * ((((s.v[469]) as f64).powf(p[439]) - ((s.v[474]) as f64).powf(p[439]))).max(0.0))));s.store_scale(534, 534, s.v[12]);s.b[894] = (p[44] != 0.0);s.store_scalar(894, if s.b[894] { 1.0 } else { 0.0 });
        if s.b[894] {s.store_scale(535, 535, s.v[12]);}
        s.store_scalar(12, (p[485] * ((((s.v[469]) as f64).powf(p[486]) - ((s.v[474]) as f64).powf(p[486]))).max(0.0)));s.store_scalar(13, (p[487] * ((((s.v[470]) as f64).powf(p[488]) - ((s.v[475]) as f64).powf(p[488]))).max(0.0)));s.store_scale(551, 551, ((1.0 + s.v[12]) + s.v[13]));s.b[895] = (p[44] != 0.0);s.store_scalar(895, if s.b[895] { 1.0 } else { 0.0 });
        if s.b[895] {s.store_scale(552, 552, ((1.0 + s.v[12]) + s.v[13]));}
        s.store_scalar(13, (p[495] * ((((s.v[470]) as f64).powf(p[496]) - ((s.v[475]) as f64).powf(p[496]))).max(0.0)));s.store_scale(554, 554, (1.0 + s.v[13]));s.store_scalar(13, (p[519] * ((((s.v[470]) as f64).powf(p[520]) - ((s.v[475]) as f64).powf(p[520]))).max(0.0)));s.store_scalar(555, p[518]);s.store_scalar(555, (s.v[555] * (1.0 + s.v[13])));s.store_scalar(13, (p[522] * ((((s.v[470]) as f64).powf(p[523]) - ((s.v[475]) as f64).powf(p[523]))).max(0.0)));s.store_scalar(556, p[521]);s.store_scalar(556, (s.v[556] * (1.0 + s.v[13])));s.store_primal_scale(559, 559, ((1.0 + (p[631] * s.v[469])) + (p[632] * s.v[470])));s.store_primal_scale(563, 563, ((1.0 + (p[649] * s.v[469])) + (p[650] * s.v[470])));s.store_primal_scale(590, 590, ((1.0 + (p[557] * s.v[469])) + (p[558] * s.v[470])));s.store_primal_scale(593, 593, ((1.0 + (p[559] * s.v[469])) + (p[560] * s.v[470])));s.store_primal_scale(596, 596, ((1.0 + (p[561] * s.v[469])) + (p[562] * s.v[470])));s.store_scalar(600, (p[556] * (1.0 + (p[563] * s.v[469]))));s.store_scalar(12, ((p[93] * ((((s.v[472]) as f64).powf(p[94]) - ((s.v[474]) as f64).powf(p[94]))).max(0.0)) + (p[95] * ((((s.v[472]) as f64).powf(p[96]) - ((s.v[474]) as f64).powf(p[96]))).max(0.0))));s.store_scalar(13, ((p[97] * ((((s.v[473]) as f64).powf(p[98]) - ((s.v[475]) as f64).powf(p[98]))).max(0.0)) + (p[99] * (((s.v[473] * s.v[472])) as f64).powf(p[100]))));s.store_scale(550, 550, ((1.0 + s.v[12]) + s.v[13]));s.store_scalar(12, (p[120] * ((((s.v[472]) as f64).powf(p[121]) - ((s.v[474]) as f64).powf(p[121]))).max(0.0)));s.store_scalar(13, ((p[122] * ((((s.v[473]) as f64).powf(p[123]) - ((s.v[475]) as f64).powf(p[123]))).max(0.0)) + (p[124] * ((s.v[471]) as f64).powf(p[125]))));s.store_scale(482, 482, ((1.0 + s.v[12]) + s.v[13]));s.store_scalar(12, (p[130] * ((((s.v[472]) as f64).powf(p[131]) - ((s.v[474]) as f64).powf(p[131]))).max(0.0)));s.store_scalar(13, ((p[132] * ((((s.v[473]) as f64).powf(p[133]) - ((s.v[475]) as f64).powf(p[133]))).max(0.0)) + (p[134] * ((s.v[471]) as f64).powf(p[135]))));s.store_scale(549, 549, ((1.0 + s.v[12]) + s.v[13]));s.store_scalar(12, (p[263] * ((((s.v[472]) as f64).powf(p[264]) - ((s.v[474]) as f64).powf(p[264]))).max(0.0)));s.store_scalar(13, ((p[265] * ((((s.v[470]) as f64).powf(p[266]) - ((s.v[475]) as f64).powf(p[266]))).max(0.0)) + (p[267] * ((s.v[471]) as f64).powf(p[268]))));s.store_scale(503, 503, ((1.0 + s.v[12]) + s.v[13]));s.store_primal_scale(542, 542, (1.0 + (p[352] * ((((s.v[472]) as f64).powf(p[353]) - ((s.v[474]) as f64).powf(p[353]))).max(0.0))));s.store_primal_max_with_scalar(542, 542, 0.0);s.store_scalar(12, (p[186] * ((((s.v[469]) as f64).powf(p[187]) - ((s.v[474]) as f64).powf(p[187]))).max(0.0)));s.store_scalar(13, ((p[188] * ((((s.v[470]) as f64).powf(p[189]) - ((s.v[475]) as f64).powf(p[189]))).max(0.0)) + (p[190] * ((s.v[471]) as f64).powf(p[191]))));s.store_scale(495, 495, ((1.0 + s.v[12]) + s.v[13]));s.store_scalar(12, (p[196] * ((((s.v[469]) as f64).powf(p[197]) - ((s.v[474]) as f64).powf(p[197]))).max(0.0)));s.store_scalar(13, ((p[198] * ((((s.v[470]) as f64).powf(p[199]) - ((s.v[475]) as f64).powf(p[199]))).max(0.0)) + (p[200] * ((s.v[471]) as f64).powf(p[201]))));s.store_scale(494, 494, ((1.0 + s.v[12]) + s.v[13]));s.store_primal_scale(543, 543, (1.0 + (p[383] * ((((s.v[469]) as f64).powf(p[384]) - ((s.v[474]) as f64).powf(p[384]))).max(0.0))));s.store_primal_scale(567, 567, (1.0 + (s.v[469] * p[828])));s.store_primal_scale(568, 568, (1.0 + (s.v[469] * p[833])));s.store_primal_scale(570, 570, (1.0 + (s.v[469] * p[842])));s.store_primal_scale(573, 573, (1.0 + (s.v[469] * p[860])));s.store_primal_scale(574, 574, (1.0 + (s.v[469] * p[866])));s.b[898] = (p[42] == 1.0);s.store_scalar(898, if s.b[898] { 1.0 } else { 0.0 });
        if s.b[898] {s.store_primal_scale(531, 531, (1.0 + (p[397] * ((((s.v[469]) as f64).powf(p[398]) - ((s.v[474]) as f64).powf(p[398]))).max(0.0))));s.store_primal_scale(530, 530, (1.0 + (p[407] * ((((s.v[469]) as f64).powf(p[408]) - ((s.v[474]) as f64).powf(p[408]))).max(0.0))));}
        if (!s.b[898]) {s.store_primal_scale(532, 532, (1.0 + (p[414] * ((((s.v[469]) as f64).powf(p[415]) - ((s.v[474]) as f64).powf(p[415]))).max(0.0))));}
        s.b[899] = (s.v[511] < 1.0);s.store_scalar(899, if s.b[899] { 1.0 } else { 0.0 });
        if s.b[899] {s.store_scalar(511, 1.0);}
        s.b[900] = (s.v[511] > 2.0);s.store_scalar(900, if s.b[900] { 1.0 } else { 0.0 });
        if ((!s.b[899]) && s.b[900]) {s.store_scalar(511, 2.0);}
        s.b[901] = (p[44] != 0.0);s.store_scalar(901, if s.b[901] { 1.0 } else { 0.0 });s.b[902] = (s.v[512] < 1.0);s.store_scalar(902, if s.b[902] { 1.0 } else { 0.0 });
        if (s.b[901] && s.b[902]) {s.store_scalar(512, 1.0);}
        s.b[903] = (s.v[512] > 2.0);s.store_scalar(903, if s.b[903] { 1.0 } else { 0.0 });
        if ((s.b[901] && (!s.b[902])) && s.b[903]) {s.store_scalar(512, 2.0);}
        s.b[925] = (s.v[606] < 0.0);s.store_scalar(925, if s.b[925] { 1.0 } else { 0.0 });
        if s.b[925] {s.store_scalar(606, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[926] = (s.v[497] <= 0.0);s.store_scalar(926, if s.b[926] { 1.0 } else { 0.0 });
        if s.b[926] {s.store_scalar(497, 0.067);}
        s.b[927] = (s.v[504] < 0.0);s.store_scalar(927, if s.b[927] { 1.0 } else { 0.0 });
        if s.b[927] {s.store_scalar(504, 0.0);}
        s.b[928] = (s.v[507] < 0.0);s.store_scalar(928, if s.b[928] { 1.0 } else { 0.0 });
        if s.b[928] {s.store_scalar(507, 0.0);}
        s.b[929] = (s.v[508] < 0.0);s.store_scalar(929, if s.b[929] { 1.0 } else { 0.0 });
        if s.b[929] {s.store_scalar(508, 0.0);}
        s.b[930] = (s.v[511] < 0.0);s.store_scalar(930, if s.b[930] { 1.0 } else { 0.0 });
        if s.b[930] {s.store_scalar(511, 0.0);}
        s.b[931] = (s.v[555] < 0.0);s.store_scalar(931, if s.b[931] { 1.0 } else { 0.0 });
        if s.b[931] {s.store_scalar(555, 0.0);}
        s.b[932] = (p[1065] == 1.0);s.store_scalar(932, if s.b[932] { 1.0 } else { 0.0 });
        if s.b[932] {s.store_scalar(746, p[1066]);}
        s.b[933] = (s.v[30] > s.v[746]);s.store_scalar(933, if s.b[933] { 1.0 } else { 0.0 });
        if (s.b[932] && s.b[933]) {s.store_sub_from_scalar(12, s.v[30], 746);}
        if (s.b[932] && (!s.b[933])) {s.store_scalar(746, s.v[30]);s.copy_ad(12, 746);}
        s.b[934] = (p[801] >= (s.v[12] / 2.0));s.store_scalar(934, if s.b[934] { 1.0 } else { 0.0 });
        if (s.b[932] && s.b[934]) {s.store_scalar(359, 0.0);}
        if (s.b[932] && (!s.b[934])) {s.store_scalar(359, p[801]);}
        s.store_scalar(701, 0.0);s.store_scalar(703, 0.0);s.store_scalar(700, 0.0);s.store_scalar(702, 0.0);s.store_scalar(705, 0.0);s.store_scalar(704, 0.0);s.store_scalar(236, (p[695] - p[698]));s.store_scalar(238, p[696]);s.store_scalar(237, (p[697] - p[698]));s.b[935] = param_given[3];s.store_scalar(935, if s.b[935] { 1.0 } else { 0.0 });
        if s.b[935] {s.store_scalar(239, (p[374] * p[3]));}
        s.b[936] = ((p[10] > 0.0) && (p[374] > 0.0));s.store_scalar(936, if s.b[936] { 1.0 } else { 0.0 });s.b[937] = (p[9] < 9.0);s.store_scalar(937, if s.b[937] { 1.0 } else { 0.0 });s.b[938] = ((p[2] % 2.0) != 0.0);s.store_scalar(938, if s.b[938] { 1.0 } else { 0.0 });
        if ((((!s.b[935]) && s.b[936]) && s.b[937]) && s.b[938]) {s.store_scalar(701, 1.0);s.store_scalar(703, 1.0);s.store_scalar(700, (2.0 * (((p[2] - 1.0) / 2.0)).max(0.0)));s.copy_ad(702, 700);}
        s.b[939] = (p[6] == 1.0);s.store_scalar(939, if s.b[939] { 1.0 } else { 0.0 });
        if (((((!s.b[935]) && s.b[936]) && s.b[937]) && (!s.b[938])) && s.b[939]) {s.store_scalar(701, 2.0);s.store_scalar(700, (2.0 * (((p[2] / 2.0) - 1.0)).max(0.0)));s.store_scalar(703, 0.0);s.store_scalar(702, p[2]);}
        if (((((!s.b[935]) && s.b[936]) && s.b[937]) && (!s.b[938])) && (!s.b[939])) {s.store_scalar(701, 0.0);s.store_scalar(700, p[2]);s.store_scalar(703, 2.0);s.store_scalar(702, (2.0 * (((p[2] / 2.0) - 1.0)).max(0.0)));}
        s.b[940] = (1.0 == 1.0);s.store_scalar(940, if s.b[940] { 1.0 } else { 0.0 });s.b[941] = (s.v[702] == 0.0);s.store_scalar(941, if s.b[941] { 1.0 } else { 0.0 });
        if (((((!s.b[935]) && s.b[936]) && s.b[937]) && s.b[940]) && s.b[941]) {s.store_scalar(704, 0.0);}
        if (((((!s.b[935]) && s.b[936]) && s.b[937]) && s.b[940]) && (!s.b[941])) {s.store_primal_div_from_scalar_scaled_input(704, (p[374] * s.v[236]), 702, s.v[29]);}
        s.b[942] = (s.v[700] == 0.0);s.store_scalar(942, if s.b[942] { 1.0 } else { 0.0 });
        if (((((!s.b[935]) && s.b[936]) && s.b[937]) && (!s.b[940])) && s.b[942]) {s.store_scalar(704, 0.0);}
        if (((((!s.b[935]) && s.b[936]) && s.b[937]) && (!s.b[940])) && (!s.b[942])) {s.store_primal_div_from_scalar_scaled_input(704, (p[374] * s.v[236]), 700, s.v[29]);}
        s.b[943] = (p[9] == 0.0);s.store_scalar(943, if s.b[943] { 1.0 } else { 0.0 });s.b[944] = (p[9] == 1.0);s.store_scalar(944, if s.b[944] { 1.0 } else { 0.0 });s.b[945] = (p[9] == 2.0);s.store_scalar(945, if s.b[945] { 1.0 } else { 0.0 });s.b[946] = (p[9] == 3.0);s.store_scalar(946, if s.b[946] { 1.0 } else { 0.0 });s.b[947] = (p[9] == 4.0);s.store_scalar(947, if s.b[947] { 1.0 } else { 0.0 });s.b[948] = (p[9] == 5.0);s.store_scalar(948, if s.b[948] { 1.0 } else { 0.0 });s.b[949] = (p[9] == 6.0);s.store_scalar(949, if s.b[949] { 1.0 } else { 0.0 });s.b[950] = (p[9] == 7.0);s.store_scalar(950, if s.b[950] { 1.0 } else { 0.0 });s.b[951] = (p[9] == 8.0);s.store_scalar(951, if s.b[951] { 1.0 } else { 0.0 });s.b[952] = (p[9] == 9.0);s.store_scalar(952, if s.b[952] { 1.0 } else { 0.0 });s.b[953] = (p[9] == 10.0);s.store_scalar(953, if s.b[953] { 1.0 } else { 0.0 });s.b[954] = (1.0 == 1.0);s.store_scalar(954, if s.b[954] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[955] = (1.0 == 1.0);s.store_scalar(955, if s.b[955] { 1.0 } else { 0.0 });s.b[956] = (((p[10] == 1.0) || (p[10] == 2.0)) || (p[10] == 5.0));s.store_scalar(956, if s.b[956] { 1.0 } else { 0.0 });s.b[957] = (((p[10] == 3.0) || (p[10] == 4.0)) || (p[10] == 6.0));s.store_scalar(957, if s.b[957] { 1.0 } else { 0.0 });s.b[958] = (s.v[703] == 0.0);s.store_scalar(958, if s.b[958] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && s.b[955]) && s.b[956]) && s.b[958]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && s.b[955]) && s.b[956]) && (!s.b[958])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 703, s.v[29]);}
        s.b[960] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));s.store_scalar(960, if s.b[960] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && s.b[955]) && (s.b[957] && (!s.b[956]))) && s.b[960]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && s.b[955]) && (s.b[957] && (!s.b[956]))) && (!s.b[960])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));}
        if ((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && s.b[955]) && (!(s.b[956] || s.b[957]))) {s.store_scalar(705, 0.0);}
        s.b[961] = (((p[10] == 1.0) || (p[10] == 3.0)) || (p[10] == 7.0));s.store_scalar(961, if s.b[961] { 1.0 } else { 0.0 });s.b[962] = (((p[10] == 2.0) || (p[10] == 4.0)) || (p[10] == 8.0));s.store_scalar(962, if s.b[962] { 1.0 } else { 0.0 });s.b[963] = (s.v[703] == 0.0);s.store_scalar(963, if s.b[963] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && (!s.b[955])) && s.b[961]) && s.b[963]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && (!s.b[955])) && s.b[961]) && (!s.b[963])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 703, s.v[29]);}
        s.b[965] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));s.store_scalar(965, if s.b[965] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && (!s.b[955])) && (s.b[962] && (!s.b[961]))) && s.b[965]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && (!s.b[955])) && (s.b[962] && (!s.b[961]))) && (!s.b[965])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));}
        if ((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && (!s.b[955])) && (!(s.b[961] || s.b[962]))) {s.store_scalar(705, 0.0);}
        s.b[966] = (0.0 == 1.0);s.store_scalar(966, if s.b[966] { 1.0 } else { 0.0 });s.b[967] = (((p[10] == 1.0) || (p[10] == 2.0)) || (p[10] == 5.0));s.store_scalar(967, if s.b[967] { 1.0 } else { 0.0 });s.b[968] = (((p[10] == 3.0) || (p[10] == 4.0)) || (p[10] == 6.0));s.store_scalar(968, if s.b[968] { 1.0 } else { 0.0 });s.b[969] = (s.v[701] == 0.0);s.store_scalar(969, if s.b[969] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && s.b[966]) && s.b[967]) && s.b[969]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && s.b[966]) && s.b[967]) && (!s.b[969])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 701, s.v[29]);}
        s.b[971] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));s.store_scalar(971, if s.b[971] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && s.b[966]) && (s.b[968] && (!s.b[967]))) && s.b[971]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && s.b[966]) && (s.b[968] && (!s.b[967]))) && (!s.b[971])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));}
        if ((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && s.b[966]) && (!(s.b[967] || s.b[968]))) {s.store_scalar(705, 0.0);}
        s.b[972] = (((p[10] == 1.0) || (p[10] == 3.0)) || (p[10] == 7.0));s.store_scalar(972, if s.b[972] { 1.0 } else { 0.0 });s.b[973] = (((p[10] == 2.0) || (p[10] == 4.0)) || (p[10] == 8.0));s.store_scalar(973, if s.b[973] { 1.0 } else { 0.0 });s.b[974] = (s.v[701] == 0.0);s.store_scalar(974, if s.b[974] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && (!s.b[966])) && s.b[972]) && s.b[974]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && (!s.b[966])) && s.b[972]) && (!s.b[974])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 701, s.v[29]);}
        s.b[976] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));s.store_scalar(976, if s.b[976] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && (!s.b[966])) && (s.b[973] && (!s.b[972]))) && s.b[976]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && (!s.b[966])) && (s.b[973] && (!s.b[972]))) && (!s.b[976])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));}
        if ((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && (!s.b[966])) && (!(s.b[972] || s.b[973]))) {s.store_scalar(705, 0.0);}
        s.b[977] = (1.0 == 1.0);s.store_scalar(977, if s.b[977] { 1.0 } else { 0.0 });s.b[978] = (1.0 == 1.0);s.store_scalar(978, if s.b[978] { 1.0 } else { 0.0 });s.b[979] = (((p[10] == 1.0) || (p[10] == 2.0)) || (p[10] == 5.0));s.store_scalar(979, if s.b[979] { 1.0 } else { 0.0 });s.b[980] = (((p[10] == 3.0) || (p[10] == 4.0)) || (p[10] == 6.0));s.store_scalar(980, if s.b[980] { 1.0 } else { 0.0 });s.b[981] = (s.v[703] == 0.0);s.store_scalar(981, if s.b[981] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && s.b[978]) && s.b[979]) && s.b[981]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && s.b[978]) && s.b[979]) && (!s.b[981])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 703, s.v[29]);}
        s.b[983] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));s.store_scalar(983, if s.b[983] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && s.b[978]) && (s.b[980] && (!s.b[979]))) && s.b[983]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && s.b[978]) && (s.b[980] && (!s.b[979]))) && (!s.b[983])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));}
        if ((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && s.b[978]) && (!(s.b[979] || s.b[980]))) {s.store_scalar(705, 0.0);}
        s.b[984] = (((p[10] == 1.0) || (p[10] == 3.0)) || (p[10] == 7.0));s.store_scalar(984, if s.b[984] { 1.0 } else { 0.0 });s.b[985] = (((p[10] == 2.0) || (p[10] == 4.0)) || (p[10] == 8.0));s.store_scalar(985, if s.b[985] { 1.0 } else { 0.0 });s.b[986] = (s.v[703] == 0.0);s.store_scalar(986, if s.b[986] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && (!s.b[978])) && s.b[984]) && s.b[986]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && (!s.b[978])) && s.b[984]) && (!s.b[986])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 703, s.v[29]);}
        s.b[988] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));s.store_scalar(988, if s.b[988] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && (!s.b[978])) && (s.b[985] && (!s.b[984]))) && s.b[988]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && (!s.b[978])) && (s.b[985] && (!s.b[984]))) && (!s.b[988])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));}
        if ((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && (!s.b[978])) && (!(s.b[984] || s.b[985]))) {s.store_scalar(705, 0.0);}
        s.b[989] = (0.0 == 1.0);s.store_scalar(989, if s.b[989] { 1.0 } else { 0.0 });s.b[990] = (((p[10] == 1.0) || (p[10] == 2.0)) || (p[10] == 5.0));s.store_scalar(990, if s.b[990] { 1.0 } else { 0.0 });s.b[991] = (((p[10] == 3.0) || (p[10] == 4.0)) || (p[10] == 6.0));s.store_scalar(991, if s.b[991] { 1.0 } else { 0.0 });s.b[992] = (s.v[701] == 0.0);s.store_scalar(992, if s.b[992] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && s.b[989]) && s.b[990]) && s.b[992]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && s.b[989]) && s.b[990]) && (!s.b[992])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 701, s.v[29]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[994] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));s.store_scalar(994, if s.b[994] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && s.b[989]) && (s.b[991] && (!s.b[990]))) && s.b[994]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && s.b[989]) && (s.b[991] && (!s.b[990]))) && (!s.b[994])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 701, (6.0 * s.v[236]));}
        if ((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && s.b[989]) && (!(s.b[990] || s.b[991]))) {s.store_scalar(705, 0.0);}
        s.b[995] = (((p[10] == 1.0) || (p[10] == 3.0)) || (p[10] == 7.0));s.store_scalar(995, if s.b[995] { 1.0 } else { 0.0 });s.b[996] = (((p[10] == 2.0) || (p[10] == 4.0)) || (p[10] == 8.0));s.store_scalar(996, if s.b[996] { 1.0 } else { 0.0 });s.b[997] = (s.v[701] == 0.0);s.store_scalar(997, if s.b[997] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && (!s.b[989])) && s.b[995]) && s.b[997]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && (!s.b[989])) && s.b[995]) && (!s.b[997])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 701, s.v[29]);}
        s.b[999] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));s.store_scalar(999, if s.b[999] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && (!s.b[989])) && (s.b[996] && (!s.b[995]))) && s.b[999]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && (!s.b[989])) && (s.b[996] && (!s.b[995]))) && (!s.b[999])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 701, (6.0 * s.v[236]));}
        if ((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && (!s.b[989])) && (!(s.b[995] || s.b[996]))) {s.store_scalar(705, 0.0);}
        s.b[1000] = (1.0 == 1.0);s.store_scalar(1000, if s.b[1000] { 1.0 } else { 0.0 });s.b[1001] = (1.0 == 1.0);s.store_scalar(1001, if s.b[1001] { 1.0 } else { 0.0 });s.b[1002] = (((p[10] == 1.0) || (p[10] == 2.0)) || (p[10] == 5.0));s.store_scalar(1002, if s.b[1002] { 1.0 } else { 0.0 });s.b[1003] = (((p[10] == 3.0) || (p[10] == 4.0)) || (p[10] == 6.0));s.store_scalar(1003, if s.b[1003] { 1.0 } else { 0.0 });s.b[1004] = (s.v[703] == 0.0);s.store_scalar(1004, if s.b[1004] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && s.b[1001]) && s.b[1002]) && s.b[1004]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && s.b[1001]) && s.b[1002]) && (!s.b[1004])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 703, s.v[29]);}
        s.b[1006] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));s.store_scalar(1006, if s.b[1006] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && s.b[1001]) && (s.b[1003] && (!s.b[1002]))) && s.b[1006]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && s.b[1001]) && (s.b[1003] && (!s.b[1002]))) && (!s.b[1006])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 703, (6.0 * s.v[236]));}
        if ((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && s.b[1001]) && (!(s.b[1002] || s.b[1003]))) {s.store_scalar(705, 0.0);}
        s.b[1007] = (((p[10] == 1.0) || (p[10] == 3.0)) || (p[10] == 7.0));s.store_scalar(1007, if s.b[1007] { 1.0 } else { 0.0 });s.b[1008] = (((p[10] == 2.0) || (p[10] == 4.0)) || (p[10] == 8.0));s.store_scalar(1008, if s.b[1008] { 1.0 } else { 0.0 });s.b[1009] = (s.v[703] == 0.0);s.store_scalar(1009, if s.b[1009] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && (!s.b[1001])) && s.b[1007]) && s.b[1009]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && (!s.b[1001])) && s.b[1007]) && (!s.b[1009])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 703, s.v[29]);}
        s.b[1011] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));s.store_scalar(1011, if s.b[1011] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && (!s.b[1001])) && (s.b[1008] && (!s.b[1007]))) && s.b[1011]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && (!s.b[1001])) && (s.b[1008] && (!s.b[1007]))) && (!s.b[1011])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 703, (6.0 * s.v[236]));}
        if ((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && (!s.b[1001])) && (!(s.b[1007] || s.b[1008]))) {s.store_scalar(705, 0.0);}
        s.b[1012] = (0.0 == 1.0);s.store_scalar(1012, if s.b[1012] { 1.0 } else { 0.0 });s.b[1013] = (((p[10] == 1.0) || (p[10] == 2.0)) || (p[10] == 5.0));s.store_scalar(1013, if s.b[1013] { 1.0 } else { 0.0 });s.b[1014] = (((p[10] == 3.0) || (p[10] == 4.0)) || (p[10] == 6.0));s.store_scalar(1014, if s.b[1014] { 1.0 } else { 0.0 });s.b[1015] = (s.v[701] == 0.0);s.store_scalar(1015, if s.b[1015] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && s.b[1012]) && s.b[1013]) && s.b[1015]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && s.b[1012]) && s.b[1013]) && (!s.b[1015])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 701, s.v[29]);}
        s.b[1017] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));s.store_scalar(1017, if s.b[1017] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && s.b[1012]) && (s.b[1014] && (!s.b[1013]))) && s.b[1017]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && s.b[1012]) && (s.b[1014] && (!s.b[1013]))) && (!s.b[1017])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));}
        if ((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && s.b[1012]) && (!(s.b[1013] || s.b[1014]))) {s.store_scalar(705, 0.0);}
        s.b[1018] = (((p[10] == 1.0) || (p[10] == 3.0)) || (p[10] == 7.0));s.store_scalar(1018, if s.b[1018] { 1.0 } else { 0.0 });s.b[1019] = (((p[10] == 2.0) || (p[10] == 4.0)) || (p[10] == 8.0));s.store_scalar(1019, if s.b[1019] { 1.0 } else { 0.0 });s.b[1020] = (s.v[701] == 0.0);s.store_scalar(1020, if s.b[1020] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && (!s.b[1012])) && s.b[1018]) && s.b[1020]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && (!s.b[1012])) && s.b[1018]) && (!s.b[1020])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 701, s.v[29]);}
        s.b[1022] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));s.store_scalar(1022, if s.b[1022] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && (!s.b[1012])) && (s.b[1019] && (!s.b[1018]))) && s.b[1022]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && (!s.b[1012])) && (s.b[1019] && (!s.b[1018]))) && (!s.b[1022])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));}
        if ((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && (!s.b[1012])) && (!(s.b[1018] || s.b[1019]))) {s.store_scalar(705, 0.0);}
        s.b[1023] = (1.0 == 1.0);s.store_scalar(1023, if s.b[1023] { 1.0 } else { 0.0 });s.b[1024] = (1.0 == 1.0);s.store_scalar(1024, if s.b[1024] { 1.0 } else { 0.0 });s.b[1025] = (((p[10] == 1.0) || (p[10] == 2.0)) || (p[10] == 5.0));s.store_scalar(1025, if s.b[1025] { 1.0 } else { 0.0 });s.b[1026] = (((p[10] == 3.0) || (p[10] == 4.0)) || (p[10] == 6.0));s.store_scalar(1026, if s.b[1026] { 1.0 } else { 0.0 });s.b[1027] = (s.v[703] == 0.0);s.store_scalar(1027, if s.b[1027] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && s.b[1024]) && s.b[1025]) && s.b[1027]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && s.b[1024]) && s.b[1025]) && (!s.b[1027])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 703, s.v[29]);}
        s.b[1029] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));s.store_scalar(1029, if s.b[1029] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && s.b[1024]) && (s.b[1026] && (!s.b[1025]))) && s.b[1029]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && s.b[1024]) && (s.b[1026] && (!s.b[1025]))) && (!s.b[1029])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 703, (6.0 * s.v[236]));}
        if ((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && s.b[1024]) && (!(s.b[1025] || s.b[1026]))) {s.store_scalar(705, 0.0);}
        s.b[1030] = (((p[10] == 1.0) || (p[10] == 3.0)) || (p[10] == 7.0));s.store_scalar(1030, if s.b[1030] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1031] = (((p[10] == 2.0) || (p[10] == 4.0)) || (p[10] == 8.0));s.store_scalar(1031, if s.b[1031] { 1.0 } else { 0.0 });s.b[1032] = (s.v[703] == 0.0);s.store_scalar(1032, if s.b[1032] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && (!s.b[1024])) && s.b[1030]) && s.b[1032]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && (!s.b[1024])) && s.b[1030]) && (!s.b[1032])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 703, s.v[29]);}
        s.b[1034] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));s.store_scalar(1034, if s.b[1034] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && (!s.b[1024])) && (s.b[1031] && (!s.b[1030]))) && s.b[1034]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && (!s.b[1024])) && (s.b[1031] && (!s.b[1030]))) && (!s.b[1034])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 703, (6.0 * s.v[236]));}
        if ((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && (!s.b[1024])) && (!(s.b[1030] || s.b[1031]))) {s.store_scalar(705, 0.0);}
        s.b[1035] = (0.0 == 1.0);s.store_scalar(1035, if s.b[1035] { 1.0 } else { 0.0 });s.b[1036] = (((p[10] == 1.0) || (p[10] == 2.0)) || (p[10] == 5.0));s.store_scalar(1036, if s.b[1036] { 1.0 } else { 0.0 });s.b[1037] = (((p[10] == 3.0) || (p[10] == 4.0)) || (p[10] == 6.0));s.store_scalar(1037, if s.b[1037] { 1.0 } else { 0.0 });s.b[1038] = (s.v[701] == 0.0);s.store_scalar(1038, if s.b[1038] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && s.b[1035]) && s.b[1036]) && s.b[1038]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && s.b[1035]) && s.b[1036]) && (!s.b[1038])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 701, s.v[29]);}
        s.b[1040] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));s.store_scalar(1040, if s.b[1040] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && s.b[1035]) && (s.b[1037] && (!s.b[1036]))) && s.b[1040]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && s.b[1035]) && (s.b[1037] && (!s.b[1036]))) && (!s.b[1040])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 701, (6.0 * s.v[236]));}
        if ((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && s.b[1035]) && (!(s.b[1036] || s.b[1037]))) {s.store_scalar(705, 0.0);}
        s.b[1041] = (((p[10] == 1.0) || (p[10] == 3.0)) || (p[10] == 7.0));s.store_scalar(1041, if s.b[1041] { 1.0 } else { 0.0 });s.b[1042] = (((p[10] == 2.0) || (p[10] == 4.0)) || (p[10] == 8.0));s.store_scalar(1042, if s.b[1042] { 1.0 } else { 0.0 });s.b[1043] = (s.v[701] == 0.0);s.store_scalar(1043, if s.b[1043] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && (!s.b[1035])) && s.b[1041]) && s.b[1043]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && (!s.b[1035])) && s.b[1041]) && (!s.b[1043])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 701, s.v[29]);}
        s.b[1045] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));s.store_scalar(1045, if s.b[1045] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && (!s.b[1035])) && (s.b[1042] && (!s.b[1041]))) && s.b[1045]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && (!s.b[1035])) && (s.b[1042] && (!s.b[1041]))) && (!s.b[1045])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 701, (6.0 * s.v[236]));}
        if ((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && (!s.b[1035])) && (!(s.b[1041] || s.b[1042]))) {s.store_scalar(705, 0.0);}
        s.b[1046] = (1.0 == 1.0);s.store_scalar(1046, if s.b[1046] { 1.0 } else { 0.0 });s.b[1047] = (1.0 == 1.0);s.store_scalar(1047, if s.b[1047] { 1.0 } else { 0.0 });s.b[1048] = (((p[10] == 1.0) || (p[10] == 2.0)) || (p[10] == 5.0));s.store_scalar(1048, if s.b[1048] { 1.0 } else { 0.0 });s.b[1049] = (((p[10] == 3.0) || (p[10] == 4.0)) || (p[10] == 6.0));s.store_scalar(1049, if s.b[1049] { 1.0 } else { 0.0 });s.b[1050] = (s.v[703] == 0.0);s.store_scalar(1050, if s.b[1050] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && s.b[1047]) && s.b[1048]) && s.b[1050]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && s.b[1047]) && s.b[1048]) && (!s.b[1050])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 703, s.v[29]);}
        s.b[1052] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));s.store_scalar(1052, if s.b[1052] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && s.b[1047]) && (s.b[1049] && (!s.b[1048]))) && s.b[1052]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && s.b[1047]) && (s.b[1049] && (!s.b[1048]))) && (!s.b[1052])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));}
        if ((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && s.b[1047]) && (!(s.b[1048] || s.b[1049]))) {s.store_scalar(705, 0.0);}
        s.b[1053] = (((p[10] == 1.0) || (p[10] == 3.0)) || (p[10] == 7.0));s.store_scalar(1053, if s.b[1053] { 1.0 } else { 0.0 });s.b[1054] = (((p[10] == 2.0) || (p[10] == 4.0)) || (p[10] == 8.0));s.store_scalar(1054, if s.b[1054] { 1.0 } else { 0.0 });s.b[1055] = (s.v[703] == 0.0);s.store_scalar(1055, if s.b[1055] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && (!s.b[1047])) && s.b[1053]) && s.b[1055]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && (!s.b[1047])) && s.b[1053]) && (!s.b[1055])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 703, s.v[29]);}
        s.b[1057] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));s.store_scalar(1057, if s.b[1057] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && (!s.b[1047])) && (s.b[1054] && (!s.b[1053]))) && s.b[1057]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && (!s.b[1047])) && (s.b[1054] && (!s.b[1053]))) && (!s.b[1057])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));}
        if ((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && (!s.b[1047])) && (!(s.b[1053] || s.b[1054]))) {s.store_scalar(705, 0.0);}
        if ((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && (!s.b[1046])) {s.store_scalar(705, ((p[374] * s.v[237]) / s.v[29]));}
        s.b[1058] = (1.0 == 1.0);s.store_scalar(1058, if s.b[1058] { 1.0 } else { 0.0 });s.b[1059] = (1.0 == 1.0);s.store_scalar(1059, if s.b[1059] { 1.0 } else { 0.0 });s.b[1060] = (((p[10] == 1.0) || (p[10] == 2.0)) || (p[10] == 5.0));s.store_scalar(1060, if s.b[1060] { 1.0 } else { 0.0 });s.b[1061] = (((p[10] == 3.0) || (p[10] == 4.0)) || (p[10] == 6.0));s.store_scalar(1061, if s.b[1061] { 1.0 } else { 0.0 });s.b[1062] = (s.v[703] == 0.0);s.store_scalar(1062, if s.b[1062] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && s.b[1059]) && s.b[1060]) && s.b[1062]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && s.b[1059]) && s.b[1060]) && (!s.b[1062])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 703, s.v[29]);}
        s.b[1064] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));s.store_scalar(1064, if s.b[1064] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && s.b[1059]) && (s.b[1061] && (!s.b[1060]))) && s.b[1064]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && s.b[1059]) && (s.b[1061] && (!s.b[1060]))) && (!s.b[1064])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 703, (6.0 * s.v[236]));}
        if ((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && s.b[1059]) && (!(s.b[1060] || s.b[1061]))) {s.store_scalar(705, 0.0);}
        s.b[1065] = (((p[10] == 1.0) || (p[10] == 3.0)) || (p[10] == 7.0));s.store_scalar(1065, if s.b[1065] { 1.0 } else { 0.0 });s.b[1066] = (((p[10] == 2.0) || (p[10] == 4.0)) || (p[10] == 8.0));s.store_scalar(1066, if s.b[1066] { 1.0 } else { 0.0 });s.b[1067] = (s.v[703] == 0.0);s.store_scalar(1067, if s.b[1067] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && (!s.b[1059])) && s.b[1065]) && s.b[1067]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && (!s.b[1059])) && s.b[1065]) && (!s.b[1067])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 703, s.v[29]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_9(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1069] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));s.store_scalar(1069, if s.b[1069] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && (!s.b[1059])) && (s.b[1066] && (!s.b[1065]))) && s.b[1069]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && (!s.b[1059])) && (s.b[1066] && (!s.b[1065]))) && (!s.b[1069])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 703, (6.0 * s.v[236]));}
        if ((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && (!s.b[1059])) && (!(s.b[1065] || s.b[1066]))) {s.store_scalar(705, 0.0);}
        s.b[1070] = (s.v[701] == 0.0);s.store_scalar(1070, if s.b[1070] { 1.0 } else { 0.0 });
        if (((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && (!s.b[1058])) && s.b[1070]) {s.store_scalar(705, 0.0);}
        if (((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && (!s.b[1058])) && (!s.b[1070])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[237]), 701, s.v[29]);}
        s.b[1071] = (1.0 == 1.0);s.store_scalar(1071, if s.b[1071] { 1.0 } else { 0.0 });
        if ((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && s.b[1071]) {s.store_scalar(705, ((p[374] * s.v[237]) / s.v[29]));}
        s.b[1072] = (0.0 == 1.0);s.store_scalar(1072, if s.b[1072] { 1.0 } else { 0.0 });s.b[1073] = (((p[10] == 1.0) || (p[10] == 2.0)) || (p[10] == 5.0));s.store_scalar(1073, if s.b[1073] { 1.0 } else { 0.0 });s.b[1074] = (((p[10] == 3.0) || (p[10] == 4.0)) || (p[10] == 6.0));s.store_scalar(1074, if s.b[1074] { 1.0 } else { 0.0 });s.b[1075] = (s.v[701] == 0.0);s.store_scalar(1075, if s.b[1075] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && s.b[1072]) && s.b[1073]) && s.b[1075]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && s.b[1072]) && s.b[1073]) && (!s.b[1075])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 701, s.v[29]);}
        s.b[1077] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));s.store_scalar(1077, if s.b[1077] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && s.b[1072]) && (s.b[1074] && (!s.b[1073]))) && s.b[1077]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && s.b[1072]) && (s.b[1074] && (!s.b[1073]))) && (!s.b[1077])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));}
        if ((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && s.b[1072]) && (!(s.b[1073] || s.b[1074]))) {s.store_scalar(705, 0.0);}
        s.b[1078] = (((p[10] == 1.0) || (p[10] == 3.0)) || (p[10] == 7.0));s.store_scalar(1078, if s.b[1078] { 1.0 } else { 0.0 });s.b[1079] = (((p[10] == 2.0) || (p[10] == 4.0)) || (p[10] == 8.0));s.store_scalar(1079, if s.b[1079] { 1.0 } else { 0.0 });s.b[1080] = (s.v[701] == 0.0);s.store_scalar(1080, if s.b[1080] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && (!s.b[1072])) && s.b[1078]) && s.b[1080]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && (!s.b[1072])) && s.b[1078]) && (!s.b[1080])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 701, s.v[29]);}
        s.b[1082] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));s.store_scalar(1082, if s.b[1082] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && (!s.b[1072])) && (s.b[1079] && (!s.b[1078]))) && s.b[1082]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && (!s.b[1072])) && (s.b[1079] && (!s.b[1078]))) && (!s.b[1082])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));}
        if ((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && (!s.b[1072])) && (!(s.b[1078] || s.b[1079]))) {s.store_scalar(705, 0.0);}
        s.b[1083] = (1.0 == 1.0);s.store_scalar(1083, if s.b[1083] { 1.0 } else { 0.0 });s.b[1084] = (s.v[703] == 0.0);s.store_scalar(1084, if s.b[1084] { 1.0 } else { 0.0 });
        if (((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && s.b[1083]) && s.b[1084]) {s.store_scalar(705, 0.0);}
        if (((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && s.b[1083]) && (!s.b[1084])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[237]), 703, s.v[29]);}
        s.b[1085] = (0.0 == 1.0);s.store_scalar(1085, if s.b[1085] { 1.0 } else { 0.0 });s.b[1086] = (((p[10] == 1.0) || (p[10] == 2.0)) || (p[10] == 5.0));s.store_scalar(1086, if s.b[1086] { 1.0 } else { 0.0 });s.b[1087] = (((p[10] == 3.0) || (p[10] == 4.0)) || (p[10] == 6.0));s.store_scalar(1087, if s.b[1087] { 1.0 } else { 0.0 });s.b[1088] = (s.v[701] == 0.0);s.store_scalar(1088, if s.b[1088] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && s.b[1085]) && s.b[1086]) && s.b[1088]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && s.b[1085]) && s.b[1086]) && (!s.b[1088])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 701, s.v[29]);}
        s.b[1090] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));s.store_scalar(1090, if s.b[1090] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && s.b[1085]) && (s.b[1087] && (!s.b[1086]))) && s.b[1090]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && s.b[1085]) && (s.b[1087] && (!s.b[1086]))) && (!s.b[1090])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 701, (6.0 * s.v[236]));}
        if ((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && s.b[1085]) && (!(s.b[1086] || s.b[1087]))) {s.store_scalar(705, 0.0);}
        s.b[1091] = (((p[10] == 1.0) || (p[10] == 3.0)) || (p[10] == 7.0));s.store_scalar(1091, if s.b[1091] { 1.0 } else { 0.0 });s.b[1092] = (((p[10] == 2.0) || (p[10] == 4.0)) || (p[10] == 8.0));s.store_scalar(1092, if s.b[1092] { 1.0 } else { 0.0 });s.b[1093] = (s.v[701] == 0.0);s.store_scalar(1093, if s.b[1093] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && (!s.b[1085])) && s.b[1091]) && s.b[1093]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && (!s.b[1085])) && s.b[1091]) && (!s.b[1093])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 701, s.v[29]);}
        s.b[1095] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));s.store_scalar(1095, if s.b[1095] { 1.0 } else { 0.0 });
        if (((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && (!s.b[1085])) && (s.b[1092] && (!s.b[1091]))) && s.b[1095]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && (!s.b[1085])) && (s.b[1092] && (!s.b[1091]))) && (!s.b[1095])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 701, (6.0 * s.v[236]));}
        if ((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && (!s.b[1085])) && (!(s.b[1091] || s.b[1092]))) {s.store_scalar(705, 0.0);}
        if (((!s.b[935]) && s.b[936]) && (s.b[951] && (!(((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950])))) {s.store_scalar(705, ((p[374] * s.v[237]) / s.v[29]));}
        s.b[1096] = (1.0 == 1.0);s.store_scalar(1096, if s.b[1096] { 1.0 } else { 0.0 });
        if ((((!s.b[935]) && s.b[936]) && (s.b[952] && (!((((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950]) || s.b[951])))) && s.b[1096]) {s.store_scalar(705, (((0.5 * p[374]) * s.v[236]) / s.v[29]));}
        s.b[1097] = (p[2] == 2.0);s.store_scalar(1097, if s.b[1097] { 1.0 } else { 0.0 });
        if (((((!s.b[935]) && s.b[936]) && (s.b[952] && (!((((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950]) || s.b[951])))) && s.b[1096]) && s.b[1097]) {s.store_scalar(704, 0.0);}
        if (((((!s.b[935]) && s.b[936]) && (s.b[952] && (!((((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950]) || s.b[951])))) && s.b[1096]) && (!s.b[1097])) {s.store_scalar(704, ((p[374] * s.v[236]) / (s.v[29] * (p[2] - 2.0))));}
        if ((((!s.b[935]) && s.b[936]) && (s.b[952] && (!((((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950]) || s.b[951])))) && (!s.b[1096])) {s.store_scalar(705, 0.0);s.store_scalar(704, ((p[374] * s.v[236]) / (s.v[29] * p[2])));}
        s.b[1098] = (1.0 == 1.0);s.store_scalar(1098, if s.b[1098] { 1.0 } else { 0.0 });
        if ((((!s.b[935]) && s.b[936]) && (s.b[953] && (!(((((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950]) || s.b[951]) || s.b[952])))) && s.b[1098]) {s.store_scalar(705, 0.0);s.store_scalar(704, ((p[374] * s.v[236]) / (s.v[29] * p[2])));}
        if ((((!s.b[935]) && s.b[936]) && (s.b[953] && (!(((((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950]) || s.b[951]) || s.b[952])))) && (!s.b[1098])) {s.store_scalar(705, (((0.5 * p[374]) * s.v[236]) / s.v[29]));}
        s.b[1099] = (p[2] == 2.0);s.store_scalar(1099, if s.b[1099] { 1.0 } else { 0.0 });
        if (((((!s.b[935]) && s.b[936]) && (s.b[953] && (!(((((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950]) || s.b[951]) || s.b[952])))) && (!s.b[1098])) && s.b[1099]) {s.store_scalar(704, 0.0);}
        if (((((!s.b[935]) && s.b[936]) && (s.b[953] && (!(((((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950]) || s.b[951]) || s.b[952])))) && (!s.b[1098])) && (!s.b[1099])) {s.store_scalar(704, ((p[374] * s.v[236]) / (s.v[29] * (p[2] - 2.0))));}
        if (((!s.b[935]) && s.b[936]) && (!((((((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950]) || s.b[951]) || s.b[952]) || s.b[953]))) {s.store_scalar(704, 0.0);}
        s.b[1100] = (s.v[704] <= 0.0);s.store_scalar(1100, if s.b[1100] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (((!s.b[935]) && s.b[936]) && s.b[1100]) {s.copy_ad(239, 705);}
        s.b[1101] = (s.v[705] <= 0.0);s.store_scalar(1101, if s.b[1101] { 1.0 } else { 0.0 });
        if ((((!s.b[935]) && s.b[936]) && (!s.b[1100])) && s.b[1101]) {s.copy_ad(239, 704);}
        if ((((!s.b[935]) && s.b[936]) && (!s.b[1100])) && (!s.b[1101])) {s.store_primal_div_scaled_product_add_scaled_denominator_indices(239, 704, 705, 1.0, 704, 1.0, 705, 1.0, 1.0);}
        if ((!s.b[935]) && (!s.b[936])) {s.store_scalar(239, 0.0);}
        s.b[1103] = param_given[4];s.store_scalar(1103, if s.b[1103] { 1.0 } else { 0.0 });
        if s.b[1103] {s.store_scalar(240, (p[374] * p[4]));}
        s.b[1104] = ((p[10] > 0.0) && (p[374] > 0.0));s.store_scalar(1104, if s.b[1104] { 1.0 } else { 0.0 });s.b[1105] = (p[9] < 9.0);s.store_scalar(1105, if s.b[1105] { 1.0 } else { 0.0 });s.b[1106] = ((p[2] % 2.0) != 0.0);s.store_scalar(1106, if s.b[1106] { 1.0 } else { 0.0 });
        if ((((!s.b[1103]) && s.b[1104]) && s.b[1105]) && s.b[1106]) {s.store_scalar(701, 1.0);s.store_scalar(703, 1.0);s.store_scalar(700, (2.0 * (((p[2] - 1.0) / 2.0)).max(0.0)));s.copy_ad(702, 700);}
        s.b[1107] = (p[6] == 1.0);s.store_scalar(1107, if s.b[1107] { 1.0 } else { 0.0 });
        if (((((!s.b[1103]) && s.b[1104]) && s.b[1105]) && (!s.b[1106])) && s.b[1107]) {s.store_scalar(701, 2.0);s.store_scalar(700, (2.0 * (((p[2] / 2.0) - 1.0)).max(0.0)));s.store_scalar(703, 0.0);s.store_scalar(702, p[2]);}
        if (((((!s.b[1103]) && s.b[1104]) && s.b[1105]) && (!s.b[1106])) && (!s.b[1107])) {s.store_scalar(701, 0.0);s.store_scalar(700, p[2]);s.store_scalar(703, 2.0);s.store_scalar(702, (2.0 * (((p[2] / 2.0) - 1.0)).max(0.0)));}
        s.b[1108] = (0.0 == 1.0);s.store_scalar(1108, if s.b[1108] { 1.0 } else { 0.0 });s.b[1109] = (s.v[702] == 0.0);s.store_scalar(1109, if s.b[1109] { 1.0 } else { 0.0 });
        if (((((!s.b[1103]) && s.b[1104]) && s.b[1105]) && s.b[1108]) && s.b[1109]) {s.store_scalar(704, 0.0);}
        if (((((!s.b[1103]) && s.b[1104]) && s.b[1105]) && s.b[1108]) && (!s.b[1109])) {s.store_primal_div_from_scalar_scaled_input(704, (p[374] * s.v[236]), 702, s.v[29]);}
        s.b[1110] = (s.v[700] == 0.0);s.store_scalar(1110, if s.b[1110] { 1.0 } else { 0.0 });
        if (((((!s.b[1103]) && s.b[1104]) && s.b[1105]) && (!s.b[1108])) && s.b[1110]) {s.store_scalar(704, 0.0);}
        if (((((!s.b[1103]) && s.b[1104]) && s.b[1105]) && (!s.b[1108])) && (!s.b[1110])) {s.store_primal_div_from_scalar_scaled_input(704, (p[374] * s.v[236]), 700, s.v[29]);}
        s.b[1111] = (p[9] == 0.0);s.store_scalar(1111, if s.b[1111] { 1.0 } else { 0.0 });s.b[1112] = (p[9] == 1.0);s.store_scalar(1112, if s.b[1112] { 1.0 } else { 0.0 });s.b[1113] = (p[9] == 2.0);s.store_scalar(1113, if s.b[1113] { 1.0 } else { 0.0 });s.b[1114] = (p[9] == 3.0);s.store_scalar(1114, if s.b[1114] { 1.0 } else { 0.0 });s.b[1115] = (p[9] == 4.0);s.store_scalar(1115, if s.b[1115] { 1.0 } else { 0.0 });s.b[1116] = (p[9] == 5.0);s.store_scalar(1116, if s.b[1116] { 1.0 } else { 0.0 });s.b[1117] = (p[9] == 6.0);s.store_scalar(1117, if s.b[1117] { 1.0 } else { 0.0 });s.b[1118] = (p[9] == 7.0);s.store_scalar(1118, if s.b[1118] { 1.0 } else { 0.0 });s.b[1119] = (p[9] == 8.0);s.store_scalar(1119, if s.b[1119] { 1.0 } else { 0.0 });s.b[1120] = (p[9] == 9.0);s.store_scalar(1120, if s.b[1120] { 1.0 } else { 0.0 });s.b[1121] = (p[9] == 10.0);s.store_scalar(1121, if s.b[1121] { 1.0 } else { 0.0 });s.b[1122] = (0.0 == 1.0);s.store_scalar(1122, if s.b[1122] { 1.0 } else { 0.0 });s.b[1123] = (1.0 == 1.0);s.store_scalar(1123, if s.b[1123] { 1.0 } else { 0.0 });s.b[1124] = (((p[10] == 1.0) || (p[10] == 2.0)) || (p[10] == 5.0));s.store_scalar(1124, if s.b[1124] { 1.0 } else { 0.0 });s.b[1125] = (((p[10] == 3.0) || (p[10] == 4.0)) || (p[10] == 6.0));s.store_scalar(1125, if s.b[1125] { 1.0 } else { 0.0 });s.b[1126] = (s.v[703] == 0.0);s.store_scalar(1126, if s.b[1126] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && s.b[1123]) && s.b[1124]) && s.b[1126]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && s.b[1123]) && s.b[1124]) && (!s.b[1126])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 703, s.v[29]);}
        s.b[1128] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));s.store_scalar(1128, if s.b[1128] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && s.b[1123]) && (s.b[1125] && (!s.b[1124]))) && s.b[1128]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && s.b[1123]) && (s.b[1125] && (!s.b[1124]))) && (!s.b[1128])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));}
        if ((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && s.b[1123]) && (!(s.b[1124] || s.b[1125]))) {s.store_scalar(705, 0.0);}
        s.b[1129] = (((p[10] == 1.0) || (p[10] == 3.0)) || (p[10] == 7.0));s.store_scalar(1129, if s.b[1129] { 1.0 } else { 0.0 });s.b[1130] = (((p[10] == 2.0) || (p[10] == 4.0)) || (p[10] == 8.0));s.store_scalar(1130, if s.b[1130] { 1.0 } else { 0.0 });s.b[1131] = (s.v[703] == 0.0);s.store_scalar(1131, if s.b[1131] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && (!s.b[1123])) && s.b[1129]) && s.b[1131]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && (!s.b[1123])) && s.b[1129]) && (!s.b[1131])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 703, s.v[29]);}
        s.b[1133] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));s.store_scalar(1133, if s.b[1133] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && (!s.b[1123])) && (s.b[1130] && (!s.b[1129]))) && s.b[1133]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && (!s.b[1123])) && (s.b[1130] && (!s.b[1129]))) && (!s.b[1133])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));}
        if ((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && (!s.b[1123])) && (!(s.b[1129] || s.b[1130]))) {s.store_scalar(705, 0.0);}
        s.b[1134] = (0.0 == 1.0);s.store_scalar(1134, if s.b[1134] { 1.0 } else { 0.0 });s.b[1135] = (((p[10] == 1.0) || (p[10] == 2.0)) || (p[10] == 5.0));s.store_scalar(1135, if s.b[1135] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1136] = (((p[10] == 3.0) || (p[10] == 4.0)) || (p[10] == 6.0));s.store_scalar(1136, if s.b[1136] { 1.0 } else { 0.0 });s.b[1137] = (s.v[701] == 0.0);s.store_scalar(1137, if s.b[1137] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && s.b[1134]) && s.b[1135]) && s.b[1137]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && s.b[1134]) && s.b[1135]) && (!s.b[1137])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 701, s.v[29]);}
        s.b[1139] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));s.store_scalar(1139, if s.b[1139] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && s.b[1134]) && (s.b[1136] && (!s.b[1135]))) && s.b[1139]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && s.b[1134]) && (s.b[1136] && (!s.b[1135]))) && (!s.b[1139])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));}
        if ((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && s.b[1134]) && (!(s.b[1135] || s.b[1136]))) {s.store_scalar(705, 0.0);}
        s.b[1140] = (((p[10] == 1.0) || (p[10] == 3.0)) || (p[10] == 7.0));s.store_scalar(1140, if s.b[1140] { 1.0 } else { 0.0 });s.b[1141] = (((p[10] == 2.0) || (p[10] == 4.0)) || (p[10] == 8.0));s.store_scalar(1141, if s.b[1141] { 1.0 } else { 0.0 });s.b[1142] = (s.v[701] == 0.0);s.store_scalar(1142, if s.b[1142] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && (!s.b[1134])) && s.b[1140]) && s.b[1142]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && (!s.b[1134])) && s.b[1140]) && (!s.b[1142])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 701, s.v[29]);}
        s.b[1144] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));s.store_scalar(1144, if s.b[1144] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && (!s.b[1134])) && (s.b[1141] && (!s.b[1140]))) && s.b[1144]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && (!s.b[1134])) && (s.b[1141] && (!s.b[1140]))) && (!s.b[1144])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));}
        if ((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && (!s.b[1134])) && (!(s.b[1140] || s.b[1141]))) {s.store_scalar(705, 0.0);}
        s.b[1145] = (0.0 == 1.0);s.store_scalar(1145, if s.b[1145] { 1.0 } else { 0.0 });s.b[1146] = (1.0 == 1.0);s.store_scalar(1146, if s.b[1146] { 1.0 } else { 0.0 });s.b[1147] = (((p[10] == 1.0) || (p[10] == 2.0)) || (p[10] == 5.0));s.store_scalar(1147, if s.b[1147] { 1.0 } else { 0.0 });s.b[1148] = (((p[10] == 3.0) || (p[10] == 4.0)) || (p[10] == 6.0));s.store_scalar(1148, if s.b[1148] { 1.0 } else { 0.0 });s.b[1149] = (s.v[703] == 0.0);s.store_scalar(1149, if s.b[1149] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && s.b[1146]) && s.b[1147]) && s.b[1149]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && s.b[1146]) && s.b[1147]) && (!s.b[1149])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 703, s.v[29]);}
        s.b[1151] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));s.store_scalar(1151, if s.b[1151] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && s.b[1146]) && (s.b[1148] && (!s.b[1147]))) && s.b[1151]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && s.b[1146]) && (s.b[1148] && (!s.b[1147]))) && (!s.b[1151])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));}
        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && s.b[1146]) && (!(s.b[1147] || s.b[1148]))) {s.store_scalar(705, 0.0);}
        s.b[1152] = (((p[10] == 1.0) || (p[10] == 3.0)) || (p[10] == 7.0));s.store_scalar(1152, if s.b[1152] { 1.0 } else { 0.0 });s.b[1153] = (((p[10] == 2.0) || (p[10] == 4.0)) || (p[10] == 8.0));s.store_scalar(1153, if s.b[1153] { 1.0 } else { 0.0 });s.b[1154] = (s.v[703] == 0.0);s.store_scalar(1154, if s.b[1154] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && (!s.b[1146])) && s.b[1152]) && s.b[1154]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && (!s.b[1146])) && s.b[1152]) && (!s.b[1154])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 703, s.v[29]);}
        s.b[1156] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));s.store_scalar(1156, if s.b[1156] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && (!s.b[1146])) && (s.b[1153] && (!s.b[1152]))) && s.b[1156]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && (!s.b[1146])) && (s.b[1153] && (!s.b[1152]))) && (!s.b[1156])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));}
        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && (!s.b[1146])) && (!(s.b[1152] || s.b[1153]))) {s.store_scalar(705, 0.0);}
        s.b[1157] = (0.0 == 1.0);s.store_scalar(1157, if s.b[1157] { 1.0 } else { 0.0 });s.b[1158] = (((p[10] == 1.0) || (p[10] == 2.0)) || (p[10] == 5.0));s.store_scalar(1158, if s.b[1158] { 1.0 } else { 0.0 });s.b[1159] = (((p[10] == 3.0) || (p[10] == 4.0)) || (p[10] == 6.0));s.store_scalar(1159, if s.b[1159] { 1.0 } else { 0.0 });s.b[1160] = (s.v[701] == 0.0);s.store_scalar(1160, if s.b[1160] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && s.b[1157]) && s.b[1158]) && s.b[1160]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && s.b[1157]) && s.b[1158]) && (!s.b[1160])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 701, s.v[29]);}
        s.b[1162] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));s.store_scalar(1162, if s.b[1162] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && s.b[1157]) && (s.b[1159] && (!s.b[1158]))) && s.b[1162]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && s.b[1157]) && (s.b[1159] && (!s.b[1158]))) && (!s.b[1162])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 701, (6.0 * s.v[236]));}
        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && s.b[1157]) && (!(s.b[1158] || s.b[1159]))) {s.store_scalar(705, 0.0);}
        s.b[1163] = (((p[10] == 1.0) || (p[10] == 3.0)) || (p[10] == 7.0));s.store_scalar(1163, if s.b[1163] { 1.0 } else { 0.0 });s.b[1164] = (((p[10] == 2.0) || (p[10] == 4.0)) || (p[10] == 8.0));s.store_scalar(1164, if s.b[1164] { 1.0 } else { 0.0 });s.b[1165] = (s.v[701] == 0.0);s.store_scalar(1165, if s.b[1165] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && (!s.b[1157])) && s.b[1163]) && s.b[1165]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && (!s.b[1157])) && s.b[1163]) && (!s.b[1165])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 701, s.v[29]);}
        s.b[1167] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));s.store_scalar(1167, if s.b[1167] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && (!s.b[1157])) && (s.b[1164] && (!s.b[1163]))) && s.b[1167]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && (!s.b[1157])) && (s.b[1164] && (!s.b[1163]))) && (!s.b[1167])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 701, (6.0 * s.v[236]));}
        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && (!s.b[1157])) && (!(s.b[1163] || s.b[1164]))) {s.store_scalar(705, 0.0);}
        s.b[1168] = (0.0 == 1.0);s.store_scalar(1168, if s.b[1168] { 1.0 } else { 0.0 });s.b[1169] = (1.0 == 1.0);s.store_scalar(1169, if s.b[1169] { 1.0 } else { 0.0 });s.b[1170] = (((p[10] == 1.0) || (p[10] == 2.0)) || (p[10] == 5.0));s.store_scalar(1170, if s.b[1170] { 1.0 } else { 0.0 });s.b[1171] = (((p[10] == 3.0) || (p[10] == 4.0)) || (p[10] == 6.0));s.store_scalar(1171, if s.b[1171] { 1.0 } else { 0.0 });s.b[1172] = (s.v[703] == 0.0);s.store_scalar(1172, if s.b[1172] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && s.b[1169]) && s.b[1170]) && s.b[1172]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && s.b[1169]) && s.b[1170]) && (!s.b[1172])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 703, s.v[29]);}
        s.b[1174] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));s.store_scalar(1174, if s.b[1174] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && s.b[1169]) && (s.b[1171] && (!s.b[1170]))) && s.b[1174]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && s.b[1169]) && (s.b[1171] && (!s.b[1170]))) && (!s.b[1174])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 703, (6.0 * s.v[236]));}
        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && s.b[1169]) && (!(s.b[1170] || s.b[1171]))) {s.store_scalar(705, 0.0);}
        s.b[1175] = (((p[10] == 1.0) || (p[10] == 3.0)) || (p[10] == 7.0));s.store_scalar(1175, if s.b[1175] { 1.0 } else { 0.0 });s.b[1176] = (((p[10] == 2.0) || (p[10] == 4.0)) || (p[10] == 8.0));s.store_scalar(1176, if s.b[1176] { 1.0 } else { 0.0 });s.b[1177] = (s.v[703] == 0.0);s.store_scalar(1177, if s.b[1177] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && (!s.b[1169])) && s.b[1175]) && s.b[1177]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && (!s.b[1169])) && s.b[1175]) && (!s.b[1177])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 703, s.v[29]);}
        s.b[1179] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));s.store_scalar(1179, if s.b[1179] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && (!s.b[1169])) && (s.b[1176] && (!s.b[1175]))) && s.b[1179]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && (!s.b[1169])) && (s.b[1176] && (!s.b[1175]))) && (!s.b[1179])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 703, (6.0 * s.v[236]));}
        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && (!s.b[1169])) && (!(s.b[1175] || s.b[1176]))) {s.store_scalar(705, 0.0);}
        s.b[1180] = (0.0 == 1.0);s.store_scalar(1180, if s.b[1180] { 1.0 } else { 0.0 });s.b[1181] = (((p[10] == 1.0) || (p[10] == 2.0)) || (p[10] == 5.0));s.store_scalar(1181, if s.b[1181] { 1.0 } else { 0.0 });s.b[1182] = (((p[10] == 3.0) || (p[10] == 4.0)) || (p[10] == 6.0));s.store_scalar(1182, if s.b[1182] { 1.0 } else { 0.0 });s.b[1183] = (s.v[701] == 0.0);s.store_scalar(1183, if s.b[1183] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && s.b[1180]) && s.b[1181]) && s.b[1183]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && s.b[1180]) && s.b[1181]) && (!s.b[1183])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 701, s.v[29]);}
        s.b[1185] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));s.store_scalar(1185, if s.b[1185] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && s.b[1180]) && (s.b[1182] && (!s.b[1181]))) && s.b[1185]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && s.b[1180]) && (s.b[1182] && (!s.b[1181]))) && (!s.b[1185])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));}
        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && s.b[1180]) && (!(s.b[1181] || s.b[1182]))) {s.store_scalar(705, 0.0);}
        s.b[1186] = (((p[10] == 1.0) || (p[10] == 3.0)) || (p[10] == 7.0));s.store_scalar(1186, if s.b[1186] { 1.0 } else { 0.0 });s.b[1187] = (((p[10] == 2.0) || (p[10] == 4.0)) || (p[10] == 8.0));s.store_scalar(1187, if s.b[1187] { 1.0 } else { 0.0 });s.b[1188] = (s.v[701] == 0.0);s.store_scalar(1188, if s.b[1188] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && (!s.b[1180])) && s.b[1186]) && s.b[1188]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && (!s.b[1180])) && s.b[1186]) && (!s.b[1188])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 701, s.v[29]);}
        s.b[1190] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));s.store_scalar(1190, if s.b[1190] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && (!s.b[1180])) && (s.b[1187] && (!s.b[1186]))) && s.b[1190]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && (!s.b[1180])) && (s.b[1187] && (!s.b[1186]))) && (!s.b[1190])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));}
        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && (!s.b[1180])) && (!(s.b[1186] || s.b[1187]))) {s.store_scalar(705, 0.0);}
        s.b[1191] = (0.0 == 1.0);s.store_scalar(1191, if s.b[1191] { 1.0 } else { 0.0 });s.b[1192] = (1.0 == 1.0);s.store_scalar(1192, if s.b[1192] { 1.0 } else { 0.0 });s.b[1193] = (((p[10] == 1.0) || (p[10] == 2.0)) || (p[10] == 5.0));s.store_scalar(1193, if s.b[1193] { 1.0 } else { 0.0 });s.b[1194] = (((p[10] == 3.0) || (p[10] == 4.0)) || (p[10] == 6.0));s.store_scalar(1194, if s.b[1194] { 1.0 } else { 0.0 });s.b[1195] = (s.v[703] == 0.0);s.store_scalar(1195, if s.b[1195] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && s.b[1192]) && s.b[1193]) && s.b[1195]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && s.b[1192]) && s.b[1193]) && (!s.b[1195])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 703, s.v[29]);}
        s.b[1197] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));s.store_scalar(1197, if s.b[1197] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && s.b[1192]) && (s.b[1194] && (!s.b[1193]))) && s.b[1197]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && s.b[1192]) && (s.b[1194] && (!s.b[1193]))) && (!s.b[1197])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 703, (6.0 * s.v[236]));}
        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && s.b[1192]) && (!(s.b[1193] || s.b[1194]))) {s.store_scalar(705, 0.0);}
        s.b[1198] = (((p[10] == 1.0) || (p[10] == 3.0)) || (p[10] == 7.0));s.store_scalar(1198, if s.b[1198] { 1.0 } else { 0.0 });s.b[1199] = (((p[10] == 2.0) || (p[10] == 4.0)) || (p[10] == 8.0));s.store_scalar(1199, if s.b[1199] { 1.0 } else { 0.0 });s.b[1200] = (s.v[703] == 0.0);s.store_scalar(1200, if s.b[1200] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && (!s.b[1192])) && s.b[1198]) && s.b[1200]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && (!s.b[1192])) && s.b[1198]) && (!s.b[1200])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 703, s.v[29]);}
        s.b[1202] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));s.store_scalar(1202, if s.b[1202] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && (!s.b[1192])) && (s.b[1199] && (!s.b[1198]))) && s.b[1202]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && (!s.b[1192])) && (s.b[1199] && (!s.b[1198]))) && (!s.b[1202])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 703, (6.0 * s.v[236]));}
        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && (!s.b[1192])) && (!(s.b[1198] || s.b[1199]))) {s.store_scalar(705, 0.0);}
        s.b[1203] = (0.0 == 1.0);s.store_scalar(1203, if s.b[1203] { 1.0 } else { 0.0 });s.b[1204] = (((p[10] == 1.0) || (p[10] == 2.0)) || (p[10] == 5.0));s.store_scalar(1204, if s.b[1204] { 1.0 } else { 0.0 });s.b[1205] = (((p[10] == 3.0) || (p[10] == 4.0)) || (p[10] == 6.0));s.store_scalar(1205, if s.b[1205] { 1.0 } else { 0.0 });s.b[1206] = (s.v[701] == 0.0);s.store_scalar(1206, if s.b[1206] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && s.b[1203]) && s.b[1204]) && s.b[1206]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && s.b[1203]) && s.b[1204]) && (!s.b[1206])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 701, s.v[29]);}
        s.b[1208] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));s.store_scalar(1208, if s.b[1208] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && s.b[1203]) && (s.b[1205] && (!s.b[1204]))) && s.b[1208]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && s.b[1203]) && (s.b[1205] && (!s.b[1204]))) && (!s.b[1208])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 701, (6.0 * s.v[236]));}
        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && s.b[1203]) && (!(s.b[1204] || s.b[1205]))) {s.store_scalar(705, 0.0);}
        s.b[1209] = (((p[10] == 1.0) || (p[10] == 3.0)) || (p[10] == 7.0));s.store_scalar(1209, if s.b[1209] { 1.0 } else { 0.0 });s.b[1210] = (((p[10] == 2.0) || (p[10] == 4.0)) || (p[10] == 8.0));s.store_scalar(1210, if s.b[1210] { 1.0 } else { 0.0 });s.b[1211] = (s.v[701] == 0.0);s.store_scalar(1211, if s.b[1211] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && (!s.b[1203])) && s.b[1209]) && s.b[1211]) {s.store_scalar(705, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_13(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && (!s.b[1203])) && s.b[1209]) && (!s.b[1211])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 701, s.v[29]);}
        s.b[1213] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));s.store_scalar(1213, if s.b[1213] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && (!s.b[1203])) && (s.b[1210] && (!s.b[1209]))) && s.b[1213]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && (!s.b[1203])) && (s.b[1210] && (!s.b[1209]))) && (!s.b[1213])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 701, (6.0 * s.v[236]));}
        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && (!s.b[1203])) && (!(s.b[1209] || s.b[1210]))) {s.store_scalar(705, 0.0);}
        s.b[1214] = (0.0 == 1.0);s.store_scalar(1214, if s.b[1214] { 1.0 } else { 0.0 });s.b[1215] = (1.0 == 1.0);s.store_scalar(1215, if s.b[1215] { 1.0 } else { 0.0 });s.b[1216] = (((p[10] == 1.0) || (p[10] == 2.0)) || (p[10] == 5.0));s.store_scalar(1216, if s.b[1216] { 1.0 } else { 0.0 });s.b[1217] = (((p[10] == 3.0) || (p[10] == 4.0)) || (p[10] == 6.0));s.store_scalar(1217, if s.b[1217] { 1.0 } else { 0.0 });s.b[1218] = (s.v[703] == 0.0);s.store_scalar(1218, if s.b[1218] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && s.b[1215]) && s.b[1216]) && s.b[1218]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && s.b[1215]) && s.b[1216]) && (!s.b[1218])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 703, s.v[29]);}
        s.b[1220] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));s.store_scalar(1220, if s.b[1220] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && s.b[1215]) && (s.b[1217] && (!s.b[1216]))) && s.b[1220]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && s.b[1215]) && (s.b[1217] && (!s.b[1216]))) && (!s.b[1220])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));}
        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && s.b[1215]) && (!(s.b[1216] || s.b[1217]))) {s.store_scalar(705, 0.0);}
        s.b[1221] = (((p[10] == 1.0) || (p[10] == 3.0)) || (p[10] == 7.0));s.store_scalar(1221, if s.b[1221] { 1.0 } else { 0.0 });s.b[1222] = (((p[10] == 2.0) || (p[10] == 4.0)) || (p[10] == 8.0));s.store_scalar(1222, if s.b[1222] { 1.0 } else { 0.0 });s.b[1223] = (s.v[703] == 0.0);s.store_scalar(1223, if s.b[1223] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && (!s.b[1215])) && s.b[1221]) && s.b[1223]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && (!s.b[1215])) && s.b[1221]) && (!s.b[1223])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 703, s.v[29]);}
        s.b[1225] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));s.store_scalar(1225, if s.b[1225] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && (!s.b[1215])) && (s.b[1222] && (!s.b[1221]))) && s.b[1225]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && (!s.b[1215])) && (s.b[1222] && (!s.b[1221]))) && (!s.b[1225])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));}
        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && (!s.b[1215])) && (!(s.b[1221] || s.b[1222]))) {s.store_scalar(705, 0.0);}
        if ((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && (!s.b[1214])) {s.store_scalar(705, ((p[374] * s.v[237]) / s.v[29]));}
        s.b[1226] = (0.0 == 1.0);s.store_scalar(1226, if s.b[1226] { 1.0 } else { 0.0 });s.b[1227] = (1.0 == 1.0);s.store_scalar(1227, if s.b[1227] { 1.0 } else { 0.0 });s.b[1228] = (((p[10] == 1.0) || (p[10] == 2.0)) || (p[10] == 5.0));s.store_scalar(1228, if s.b[1228] { 1.0 } else { 0.0 });s.b[1229] = (((p[10] == 3.0) || (p[10] == 4.0)) || (p[10] == 6.0));s.store_scalar(1229, if s.b[1229] { 1.0 } else { 0.0 });s.b[1230] = (s.v[703] == 0.0);s.store_scalar(1230, if s.b[1230] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && s.b[1227]) && s.b[1228]) && s.b[1230]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && s.b[1227]) && s.b[1228]) && (!s.b[1230])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 703, s.v[29]);}
        s.b[1232] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));s.store_scalar(1232, if s.b[1232] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && s.b[1227]) && (s.b[1229] && (!s.b[1228]))) && s.b[1232]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && s.b[1227]) && (s.b[1229] && (!s.b[1228]))) && (!s.b[1232])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 703, (6.0 * s.v[236]));}
        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && s.b[1227]) && (!(s.b[1228] || s.b[1229]))) {s.store_scalar(705, 0.0);}
        s.b[1233] = (((p[10] == 1.0) || (p[10] == 3.0)) || (p[10] == 7.0));s.store_scalar(1233, if s.b[1233] { 1.0 } else { 0.0 });s.b[1234] = (((p[10] == 2.0) || (p[10] == 4.0)) || (p[10] == 8.0));s.store_scalar(1234, if s.b[1234] { 1.0 } else { 0.0 });s.b[1235] = (s.v[703] == 0.0);s.store_scalar(1235, if s.b[1235] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && (!s.b[1227])) && s.b[1233]) && s.b[1235]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && (!s.b[1227])) && s.b[1233]) && (!s.b[1235])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 703, s.v[29]);}
        s.b[1237] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));s.store_scalar(1237, if s.b[1237] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && (!s.b[1227])) && (s.b[1234] && (!s.b[1233]))) && s.b[1237]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && (!s.b[1227])) && (s.b[1234] && (!s.b[1233]))) && (!s.b[1237])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 703, (6.0 * s.v[236]));}
        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && (!s.b[1227])) && (!(s.b[1233] || s.b[1234]))) {s.store_scalar(705, 0.0);}
        s.b[1238] = (s.v[701] == 0.0);s.store_scalar(1238, if s.b[1238] { 1.0 } else { 0.0 });
        if (((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && (!s.b[1226])) && s.b[1238]) {s.store_scalar(705, 0.0);}
        if (((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && (!s.b[1226])) && (!s.b[1238])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[237]), 701, s.v[29]);}
        s.b[1239] = (0.0 == 1.0);s.store_scalar(1239, if s.b[1239] { 1.0 } else { 0.0 });
        if ((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && s.b[1239]) {s.store_scalar(705, ((p[374] * s.v[237]) / s.v[29]));}
        s.b[1240] = (0.0 == 1.0);s.store_scalar(1240, if s.b[1240] { 1.0 } else { 0.0 });s.b[1241] = (((p[10] == 1.0) || (p[10] == 2.0)) || (p[10] == 5.0));s.store_scalar(1241, if s.b[1241] { 1.0 } else { 0.0 });s.b[1242] = (((p[10] == 3.0) || (p[10] == 4.0)) || (p[10] == 6.0));s.store_scalar(1242, if s.b[1242] { 1.0 } else { 0.0 });s.b[1243] = (s.v[701] == 0.0);s.store_scalar(1243, if s.b[1243] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && s.b[1240]) && s.b[1241]) && s.b[1243]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && s.b[1240]) && s.b[1241]) && (!s.b[1243])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 701, s.v[29]);}
        s.b[1245] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));s.store_scalar(1245, if s.b[1245] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && s.b[1240]) && (s.b[1242] && (!s.b[1241]))) && s.b[1245]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && s.b[1240]) && (s.b[1242] && (!s.b[1241]))) && (!s.b[1245])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));}
        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && s.b[1240]) && (!(s.b[1241] || s.b[1242]))) {s.store_scalar(705, 0.0);}
        s.b[1246] = (((p[10] == 1.0) || (p[10] == 3.0)) || (p[10] == 7.0));s.store_scalar(1246, if s.b[1246] { 1.0 } else { 0.0 });s.b[1247] = (((p[10] == 2.0) || (p[10] == 4.0)) || (p[10] == 8.0));s.store_scalar(1247, if s.b[1247] { 1.0 } else { 0.0 });s.b[1248] = (s.v[701] == 0.0);s.store_scalar(1248, if s.b[1248] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && (!s.b[1240])) && s.b[1246]) && s.b[1248]) {s.store_scalar(705, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && (!s.b[1240])) && s.b[1246]) && (!s.b[1248])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 701, s.v[29]);}
        s.b[1250] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));s.store_scalar(1250, if s.b[1250] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && (!s.b[1240])) && (s.b[1247] && (!s.b[1246]))) && s.b[1250]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && (!s.b[1240])) && (s.b[1247] && (!s.b[1246]))) && (!s.b[1250])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));}
        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && (!s.b[1240])) && (!(s.b[1246] || s.b[1247]))) {s.store_scalar(705, 0.0);}
        s.b[1251] = (0.0 == 1.0);s.store_scalar(1251, if s.b[1251] { 1.0 } else { 0.0 });s.b[1252] = (s.v[703] == 0.0);s.store_scalar(1252, if s.b[1252] { 1.0 } else { 0.0 });
        if (((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && s.b[1251]) && s.b[1252]) {s.store_scalar(705, 0.0);}
        if (((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && s.b[1251]) && (!s.b[1252])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[237]), 703, s.v[29]);}
        s.b[1253] = (0.0 == 1.0);s.store_scalar(1253, if s.b[1253] { 1.0 } else { 0.0 });s.b[1254] = (((p[10] == 1.0) || (p[10] == 2.0)) || (p[10] == 5.0));s.store_scalar(1254, if s.b[1254] { 1.0 } else { 0.0 });s.b[1255] = (((p[10] == 3.0) || (p[10] == 4.0)) || (p[10] == 6.0));s.store_scalar(1255, if s.b[1255] { 1.0 } else { 0.0 });s.b[1256] = (s.v[701] == 0.0);s.store_scalar(1256, if s.b[1256] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && s.b[1253]) && s.b[1254]) && s.b[1256]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && s.b[1253]) && s.b[1254]) && (!s.b[1256])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 701, s.v[29]);}
        s.b[1258] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));s.store_scalar(1258, if s.b[1258] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && s.b[1253]) && (s.b[1255] && (!s.b[1254]))) && s.b[1258]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && s.b[1253]) && (s.b[1255] && (!s.b[1254]))) && (!s.b[1258])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 701, (6.0 * s.v[236]));}
        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && s.b[1253]) && (!(s.b[1254] || s.b[1255]))) {s.store_scalar(705, 0.0);}
        s.b[1259] = (((p[10] == 1.0) || (p[10] == 3.0)) || (p[10] == 7.0));s.store_scalar(1259, if s.b[1259] { 1.0 } else { 0.0 });s.b[1260] = (((p[10] == 2.0) || (p[10] == 4.0)) || (p[10] == 8.0));s.store_scalar(1260, if s.b[1260] { 1.0 } else { 0.0 });s.b[1261] = (s.v[701] == 0.0);s.store_scalar(1261, if s.b[1261] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && (!s.b[1253])) && s.b[1259]) && s.b[1261]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && (!s.b[1253])) && s.b[1259]) && (!s.b[1261])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[236]), 701, s.v[29]);}
        s.b[1263] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));s.store_scalar(1263, if s.b[1263] { 1.0 } else { 0.0 });
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && (!s.b[1253])) && (s.b[1260] && (!s.b[1259]))) && s.b[1263]) {s.store_scalar(705, 0.0);}
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && (!s.b[1253])) && (s.b[1260] && (!s.b[1259]))) && (!s.b[1263])) {s.store_primal_div_from_scalar_scaled_input(705, (p[374] * s.v[29]), 701, (6.0 * s.v[236]));}
        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && (!s.b[1253])) && (!(s.b[1259] || s.b[1260]))) {s.store_scalar(705, 0.0);}
        if (((!s.b[1103]) && s.b[1104]) && (s.b[1119] && (!(((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118])))) {s.store_scalar(705, ((p[374] * s.v[237]) / s.v[29]));}
        s.b[1264] = (0.0 == 1.0);s.store_scalar(1264, if s.b[1264] { 1.0 } else { 0.0 });
        if ((((!s.b[1103]) && s.b[1104]) && (s.b[1120] && (!((((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118]) || s.b[1119])))) && s.b[1264]) {s.store_scalar(705, (((0.5 * p[374]) * s.v[236]) / s.v[29]));}
        s.b[1265] = (p[2] == 2.0);s.store_scalar(1265, if s.b[1265] { 1.0 } else { 0.0 });
        if (((((!s.b[1103]) && s.b[1104]) && (s.b[1120] && (!((((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118]) || s.b[1119])))) && s.b[1264]) && s.b[1265]) {s.store_scalar(704, 0.0);}
        if (((((!s.b[1103]) && s.b[1104]) && (s.b[1120] && (!((((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118]) || s.b[1119])))) && s.b[1264]) && (!s.b[1265])) {s.store_scalar(704, ((p[374] * s.v[236]) / (s.v[29] * (p[2] - 2.0))));}
        if ((((!s.b[1103]) && s.b[1104]) && (s.b[1120] && (!((((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118]) || s.b[1119])))) && (!s.b[1264])) {s.store_scalar(705, 0.0);s.store_scalar(704, ((p[374] * s.v[236]) / (s.v[29] * p[2])));}
        s.b[1266] = (0.0 == 1.0);s.store_scalar(1266, if s.b[1266] { 1.0 } else { 0.0 });
        if ((((!s.b[1103]) && s.b[1104]) && (s.b[1121] && (!(((((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118]) || s.b[1119]) || s.b[1120])))) && s.b[1266]) {s.store_scalar(705, 0.0);s.store_scalar(704, ((p[374] * s.v[236]) / (s.v[29] * p[2])));}
        if ((((!s.b[1103]) && s.b[1104]) && (s.b[1121] && (!(((((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118]) || s.b[1119]) || s.b[1120])))) && (!s.b[1266])) {s.store_scalar(705, (((0.5 * p[374]) * s.v[236]) / s.v[29]));}
        s.b[1267] = (p[2] == 2.0);s.store_scalar(1267, if s.b[1267] { 1.0 } else { 0.0 });
        if (((((!s.b[1103]) && s.b[1104]) && (s.b[1121] && (!(((((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118]) || s.b[1119]) || s.b[1120])))) && (!s.b[1266])) && s.b[1267]) {s.store_scalar(704, 0.0);}
        if (((((!s.b[1103]) && s.b[1104]) && (s.b[1121] && (!(((((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118]) || s.b[1119]) || s.b[1120])))) && (!s.b[1266])) && (!s.b[1267])) {s.store_scalar(704, ((p[374] * s.v[236]) / (s.v[29] * (p[2] - 2.0))));}
        if (((!s.b[1103]) && s.b[1104]) && (!((((((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118]) || s.b[1119]) || s.b[1120]) || s.b[1121]))) {s.store_scalar(704, 0.0);}
        s.b[1268] = (s.v[704] <= 0.0);s.store_scalar(1268, if s.b[1268] { 1.0 } else { 0.0 });
        if (((!s.b[1103]) && s.b[1104]) && s.b[1268]) {s.copy_ad(240, 705);}
        s.b[1269] = (s.v[705] <= 0.0);s.store_scalar(1269, if s.b[1269] { 1.0 } else { 0.0 });
        if ((((!s.b[1103]) && s.b[1104]) && (!s.b[1268])) && s.b[1269]) {s.copy_ad(240, 704);}
        if ((((!s.b[1103]) && s.b[1104]) && (!s.b[1268])) && (!s.b[1269])) {s.store_primal_div_scaled_product_add_scaled_denominator_indices(240, 704, 705, 1.0, 704, 1.0, 705, 1.0, 1.0);}
        if ((!s.b[1103]) && (!s.b[1104])) {s.store_scalar(240, 0.0);}
        s.b[1271] = (p[42] == 0.0);s.store_scalar(1271, if s.b[1271] { 1.0 } else { 0.0 });s.b[1272] = (s.v[239] < p[1093]);s.store_scalar(1272, if s.b[1272] { 1.0 } else { 0.0 });
        if (s.b[1271] && s.b[1272]) {s.store_scalar(239, 0.0);}
        s.b[1273] = (s.v[240] < p[1093]);s.store_scalar(1273, if s.b[1273] { 1.0 } else { 0.0 });
        if (s.b[1271] && s.b[1273]) {s.store_scalar(240, 0.0);}
        s.b[1274] = (s.v[239] <= p[1093]);s.store_scalar(1274, if s.b[1274] { 1.0 } else { 0.0 });
        if ((!s.b[1271]) && s.b[1274]) {s.store_scalar(239, p[1093]);}
        s.b[1275] = (s.v[240] <= p[1093]);s.store_scalar(1275, if s.b[1275] { 1.0 } else { 0.0 });
        if ((!s.b[1271]) && s.b[1275]) {s.store_scalar(240, p[1093]);}
        s.b[1276] = (p[42] == 1.0);s.store_scalar(1276, if s.b[1276] { 1.0 } else { 0.0 });s.b[1277] = (s.v[529] <= 0.0);s.store_scalar(1277, if s.b[1277] { 1.0 } else { 0.0 });
        if (s.b[1276] && s.b[1277]) {s.store_scalar(529, 0.0);}
        s.b[1278] = (s.v[528] <= 0.0);s.store_scalar(1278, if s.b[1278] { 1.0 } else { 0.0 });
        if (s.b[1276] && s.b[1278]) {s.store_scalar(528, 0.0);}
        s.b[1279] = (s.v[531] <= 0.0);s.store_scalar(1279, if s.b[1279] { 1.0 } else { 0.0 });
        if (s.b[1276] && s.b[1279]) {s.store_scalar(531, 0.0);}
        s.b[1280] = (s.v[530] <= 0.0);s.store_scalar(1280, if s.b[1280] { 1.0 } else { 0.0 });
        if (s.b[1276] && s.b[1280]) {s.store_scalar(530, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_15(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();s.b[1281] = (s.v[533] <= 0.0);s.store_scalar(1281, if s.b[1281] { 1.0 } else { 0.0 });
        if ((!s.b[1276]) && s.b[1281]) {s.store_scalar(533, 0.0);}
        s.b[1282] = (s.v[532] <= 0.0);s.store_scalar(1282, if s.b[1282] { 1.0 } else { 0.0 });
        if ((!s.b[1276]) && s.b[1282]) {s.store_scalar(532, 0.0);}
        s.b[1301] = (p[1097] == 1.0);s.store_scalar(1301, if s.b[1301] { 1.0 } else { 0.0 });
        if s.b[1301] {s.store_scalar(302, (1.0 - p[1128]));}
        if (!s.b[1301]) {s.store_scalar(302, 1.0);}
        s.store_scalar(252, ((p[700] * (p[31] + ((s.v[35] / 3.0) / p[32]))) / ((p[32] * p[2]) * (s.v[98] - p[699]))));s.b[1303] = (s.v[252] > 0.0);s.store_scalar(1303, if s.b[1303] { 1.0 } else { 0.0 });
        if s.b[1303] {s.store_scalar(252, (1.0 / s.v[252]));}
        if (!s.b[1303]) {s.store_scalar(252, 1000.0);}
        s.store_scalar(12, (p[77] * p[77]));s.store_scale(13, 599, p[77]);s.store_square(14, 13);s.store_scalar(295, (if (p[39] == 1.0) { 745669000000.0 } else { 1166450000000.0 }));s.store_primal_scale(297, 599, ((-s.v[295]) * p[77]));s.store_scalar(295, ((-s.v[295]) * p[77]));s.store_scalar(38, (p[911] + s.v[29]));s.b[1305] = (((p[49] != 0.0) && (p[909] > 0.0)) && (s.v[38] > 0.0));s.store_scalar(1305, if s.b[1305] { 1.0 } else { 0.0 });
        if s.b[1305] {s.store_scalar(747, ((s.v[38] * p[2]) / p[909]));s.store_scalar(748, ((p[910] * s.v[38]) * p[2]));}
        if (!s.b[1305]) {s.store_scalar(747, 1.0);s.store_scalar(748, 0.0);}
        s.b[1306] = (p[820] <= (-273.15));s.store_scalar(1306, if s.b[1306] { 1.0 } else { 0.0 });
        if s.b[1306] {s.store_scalar(12, (300.15 - 273.15));s.store_scalar(392, 300.15);}
        if (!s.b[1306]) {s.store_scalar(392, (p[820] + 273.15));}
        s.store_scalar(391, (ctx_temp + p[33]));s.b[1307] = (((p[49] != 0.0) && (p[909] > 0.0)) && (s.v[38] > 0.0));s.store_scalar(1307, if s.b[1307] { 1.0 } else { 0.0 });
        if s.b[1307] {s.store_voltage(390, ctx, nodes, Some(4), None);}
        if (!s.b[1307]) {s.store_scalar(390, 0.0);}
        s.store_offset(391, 390, s.v[391]);s.store_scale(108, 391, 8.617087e-5);s.store_div_from_scalar(109, 1.0, 108);s.store_div(395, 391, 392);s.store_sub(396, 391, 392);s.store_scale(393, 391, 8.617087e-5);s.store_primal_scale(394, 392, 8.617087e-5);s.store_sub_from_scalar_ad(36, p[109], A::div_scaled_product_offset_denominator(s.ad_value(391), s.ad_value(391), p[821], s.ad_value(391), p[822], 1.0));s.store_primal_sub_from_scalar_ad(37, p[109], A::div_scaled_product_offset_denominator(s.ad_value(392), s.ad_value(392), p[821], s.ad_value(392), p[822], 1.0));s.store_mul_div_scaled_inputs_mixed_aii(13, A::sqrt(A::div(s.ad_value(391), s.ad_value(392))), 391, 1.0, 392, 1.0);s.store_mul_scaled_limited_exp_ad_rhs(28, 13, p[108], A::sub(A::div_scaled_inputs(s.ad_value(36), 1.0, s.ad_value(394), 2.0), A::div_scaled_inputs(s.ad_value(36), 1.0, s.ad_value(393), 2.0)));s.b[1308] = (((p[49] != 0.0) && (p[909] > 0.0)) && (s.v[38] > 0.0));s.store_scalar(1308, if s.b[1308] { 1.0 } else { 0.0 });
        if s.b[1308] {s.store_ln_ad(12, A::max_with_scalar(A::div(s.ad_value(481), s.ad_value(28)), 1e-38));s.store_sqrt_square_offset(88, 12, 1e-6);}
        if (!s.b[1308]) {s.store_ln_ad(88, A::max_with_scalar(A::div(s.ad_value(481), s.ad_value(28)), 1e-38));}
        s.b[1309] = (((p[49] != 0.0) && (p[909] > 0.0)) && (s.v[38] > 0.0));s.store_scalar(1309, if s.b[1309] { 1.0 } else { 0.0 });
        if s.b[1309] {s.store_ln_ad(12, A::max_with_scalar(A::div_scaled_product(s.ad_value(686), s.ad_value(480), 1.0, A::square(s.ad_value(28)), 1.0), 1e-38));s.store_sqrt_square_offset(675, 12, 1e-6);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1309]) {s.store_ln_ad(675, A::max_with_scalar(A::div_scaled_product(s.ad_value(686), s.ad_value(480), 1.0, A::square(s.ad_value(28)), 1.0), 1e-38));}
        s.b[1310] = (s.v[479] > 0.0);s.store_scalar(1310, if s.b[1310] { 1.0 } else { 0.0 });
        if s.b[1310] {s.store_offset_product3(63, s.ad_value(187), s.ad_value(108), A::ln(A::max_with_scalar(A::div(s.ad_value(479), s.ad_value(480)), 1e-38)), -1.0, p[5]);}
        if (!s.b[1310]) {s.store_scalar(63, 0.0);}
        s.store_max_with_scalar_ad(127, A::add(A::offset(A::mul(s.ad_value(108), s.ad_value(88)), 0.4), s.ad_value(489)), 0.4);s.store_sqrt(128, 127);s.store_sqrt_div_from_scalar_ad(114, (2.0 * s.v[26]), A::scale(s.ad_value(481), 1.60219e-19));s.store_primal_sqrt_scaled_input(129, 538, ((s.v[26] / s.v[27]) * p[77]));
        s.store_mul_mixed_ia(422, 488, {
                    if (!((1.0 + (p[823] * (s.v[395] - 1.0))) < ((-10000.0) * 0.001))) {
                        A::add_scaled_inputs(A::scale_offset(s.ad_value(395), p[823], (((((-1.0)) * (p[823]))) + (1.0))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(395), p[823], (((((-1.0)) * (p[823]))) + (1.0))), ((4.0 * 0.001) * 0.001)), 0.5)
                    } else {
                        {
                            if ((1.0 + (p[823] * (s.v[395] - 1.0))) < ((-10000.0) * 0.001)) {
                                A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(395), p[823], (((((-1.0)) * (p[823]))) + (1.0))))
                            } else {
                                A::constant(0.0)
                            }
                        }
                    }
                });
        s.store_mul_scale_offset_rhs(420, 490, 395, p[851], (((((-1.0)) * (p[851]))) + (1.0)));s.b[1311] = (p[44] != 0.0);s.store_scalar(1311, if s.b[1311] { 1.0 } else { 0.0 });
        if s.b[1311] {s.store_mul_scale_offset_rhs(421, 491, 395, p[851], (((((-1.0)) * (p[851]))) + (1.0)));}
        s.store_scalar(158, (if (p[39] != 1.0) { (0.3333333333333333 * p[283]) } else { (0.5 * p[283]) }));s.store_mul_pow_indices(397, 497, 395, 567);
        s.store_mul_mixed_ia(399, 504, {
                    if (!(((1.0 + (s.v[568] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(568), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(568), s.ad_value(396)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                    } else {
                        {
                            if (((1.0 + (s.v[568] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(568), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
                            } else {
                                A::constant(0.0)
                            }
                        }
                    }
                });
        s.store_mul_mixed_ia(401, 514, {
                    if (!(((1.0 + (s.v[569] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(569), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(569), s.ad_value(396)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                    } else {
                        {
                            if (((1.0 + (s.v[569] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(569), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
                            } else {
                                A::constant(0.0)
                            }
                        }
                    }
                });
        s.store_mul_pow_indices(403, 508, 395, 570);s.store_mul_pow_indices(405, 511, 395, 571);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_mul_mixed_ia(407, 507, {
                    if (!((1.0 + (s.v[577] * (s.v[395] - 1.0))) < ((-10000.0) * 0.001))) {
                        A::add_scaled_inputs(A::offset(A::mul_offset_rhs(s.ad_value(577), s.ad_value(395), (-1.0)), 1.0), 0.5, A::sqrt_square_offset(A::offset(A::mul_offset_rhs(s.ad_value(577), s.ad_value(395), (-1.0)), 1.0), ((4.0 * 0.001) * 0.001)), 0.5)
                    } else {
                        {
                            if ((1.0 + (s.v[577] * (s.v[395] - 1.0))) < ((-10000.0) * 0.001)) {
                                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul_offset_rhs(s.ad_value(577), s.ad_value(395), (-1.0)), 1.0, 1.0)
                            } else {
                                A::constant(0.0)
                            }
                        }
                    }
                });
        s.b[1312] = (p[44] != 0.0);s.store_scalar(1312, if s.b[1312] { 1.0 } else { 0.0 });
        if s.b[1312] {s.store_mul_pow_indices(398, 498, 395, 567);}
        if s.b[1312] {
            s.store_mul_mixed_ia(400, 505, {
                            if (!(((1.0 + (s.v[568] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::mul(s.ad_value(568), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(568), s.ad_value(396)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + (s.v[568] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(568), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        if s.b[1312] {
            s.store_mul_mixed_ia(402, 515, {
                            if (!(((1.0 + (s.v[569] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::mul(s.ad_value(569), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(569), s.ad_value(396)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + (s.v[569] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(569), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        if s.b[1312] {s.store_mul_pow_indices(404, 509, 395, 570);s.store_mul_pow_indices(406, 512, 395, 571);}
        s.store_pow_indices(408, 395, 572);s.store_mul_pow_mixed_iia(409, 500, 395, A::neg(s.ad_value(573)));s.b[1313] = (s.v[409] < 100.0);s.store_scalar(1313, if s.b[1313] { 1.0 } else { 0.0 });
        if s.b[1313] {s.store_scalar(409, 100.0);}
        s.b[1314] = (p[1094] == 1.0);s.store_scalar(1314, if s.b[1314] { 1.0 } else { 0.0 });
        if s.b[1314] {s.store_powf(762, 395, p[1120]);s.store_scale_ad(763, A::powf(s.ad_value(395), (-p[1121])), p[1100]);}
        s.b[1315] = (p[44] != 0.0);s.store_scalar(1315, if s.b[1315] { 1.0 } else { 0.0 });
        if s.b[1315] {s.store_mul_pow_mixed_iia(410, 501, 395, A::neg(s.ad_value(573)));}
        s.b[1316] = (s.v[410] < 100.0);s.store_scalar(1316, if s.b[1316] { 1.0 } else { 0.0 });
        if (s.b[1315] && s.b[1316]) {s.store_scalar(410, 100.0);}
        s.store_mul_pow_mixed_iia(411, 503, 395, A::neg(s.ad_value(573)));s.b[1317] = (s.v[411] < 100.0);s.store_scalar(1317, if s.b[1317] { 1.0 } else { 0.0 });
        if s.b[1317] {s.store_scalar(411, 100.0);}
    }
}
