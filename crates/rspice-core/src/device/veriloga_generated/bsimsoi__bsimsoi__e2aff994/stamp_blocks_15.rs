#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_add_scaled_inputs3_offset_indices(841, 701, p[1207], 702, p[1208], 703, p[1209], p[1206]);s.store_primal_add_scaled_inputs3_offset_indices(842, 701, p[1211], 702, p[1212], 703, p[1213], p[1210]);s.store_primal_add_scaled_inputs3_offset_indices(843, 701, p[1215], 702, p[1216], 703, p[1217], p[1214]);s.store_primal_add_scaled_inputs3_offset_indices(844, 701, p[1219], 702, p[1220], 703, p[1221], p[1218]);s.store_primal_add_scaled_inputs3_offset_indices(845, 701, p[1223], 702, p[1224], 703, p[1225], p[1222]);s.store_primal_add_scaled_inputs3_offset_indices(846, 701, p[1227], 702, p[1228], 703, p[1229], p[1226]);s.store_primal_add_scaled_inputs3_offset_indices(847, 701, p[1231], 702, p[1232], 703, p[1233], p[1230]);s.store_primal_add_scaled_inputs3_offset_indices(848, 701, p[1235], 702, p[1236], 703, p[1237], p[1234]);s.store_add_scaled_inputs3_offset_indices(849, 701, p[1272], 702, p[1273], 703, p[1274], p[1265]);s.store_primal_add_scaled_inputs3_offset_indices(850, 701, p[1276], 702, p[1277], 703, p[1278], p[1275]);s.store_primal_add_scaled_inputs3_offset_indices(854, 701, p[1284], 702, p[1285], 703, p[1286], p[1283]);s.store_primal_add_scaled_inputs3_offset_indices(855, 701, p[1280], 702, p[1281], 703, p[1282], p[1279]);s.store_primal_add_scaled_inputs3_offset_indices(851, 701, p[1288], 702, p[1289], 703, p[1290], p[1287]);s.store_primal_add_scaled_inputs3_offset_indices(852, 701, p[1292], 702, p[1293], 703, p[1294], p[1291]);s.store_primal_add_scaled_inputs3_offset_indices(856, 701, p[1324], 702, p[1325], 703, p[1326], p[1323]);s.store_primal_add_scaled_inputs3_offset_indices(857, 701, p[1328], 702, p[1329], 703, p[1330], p[1327]);s.store_primal_add_scaled_inputs3_offset_indices(859, 701, p[1332], 702, p[1333], 703, p[1334], p[1331]);s.store_primal_add_scaled_inputs3_offset_indices(860, 701, p[1336], 702, p[1337], 703, p[1338], p[1335]);s.store_primal_add_scaled_inputs3_offset_indices(862, 701, p[1340], 702, p[1341], 703, p[1342], p[1339]);s.store_primal_add_scaled_inputs3_offset_indices(863, 701, p[1344], 702, p[1345], 703, p[1346], p[1343]);s.store_add_scaled_inputs3_offset_indices(888, 701, p[787], 702, p[791], 703, p[795], p[783]);s.store_primal_add_scaled_inputs3_offset_indices(891, 701, p[788], 702, p[792], 703, p[796], p[784]);s.store_primal_add_scaled_inputs3_offset_indices(889, 701, p[789], 702, p[793], 703, p[797], p[785]);s.store_primal_add_scaled_inputs3_offset_indices(890, 701, p[790], 702, p[794], 703, p[798], p[786]);s.store_primal_add_scaled_inputs3_offset_indices(908, 701, p[1385], 702, p[1386], 703, p[1387], p[1384]);s.store_primal_add_scaled_inputs3_offset_indices(909, 701, p[1390], 702, p[1391], 703, p[1392], p[1389]);s.b[1149] = (p[35] != 0.0);s.store_scalar(1149, if s.b[1149] { 1.0 } else { 0.0 });
        if s.b[1149] {s.store_add_scaled_inputs3_offset_indices(839, 701, p[1173], 702, p[1174], 703, p[1175], p[1172]);s.store_add_scaled_inputs3_offset_indices(717, 701, p[285], 702, p[286], 703, p[287], p[284]);s.store_add_scaled_inputs3_offset_indices(731, 701, p[199], 702, p[200], 703, p[201], p[198]);s.store_primal_add_scaled_inputs3_offset_indices(739, 701, p[344], 702, p[345], 703, p[346], p[343]);s.store_add_scaled_inputs3_offset_indices(749, 701, p[359], 702, p[360], 703, p[361], p[358]);s.store_add_scaled_inputs3_offset_indices(753, 701, p[379], 702, p[380], 703, p[381], p[378]);s.store_primal_add_scaled_inputs3_offset_indices(756, 701, p[387], 702, p[388], 703, p[389], p[386]);s.store_add_scaled_inputs3_offset_indices(759, 701, p[401], 702, p[402], 703, p[403], p[400]);s.store_add_scaled_inputs3_offset_indices(784, 701, p[411], 702, p[412], 703, p[413], p[410]);s.store_add_scaled_inputs3_offset_indices(762, 701, p[537], 702, p[538], 703, p[539], p[536]);s.store_add_scaled_inputs3_offset_indices(745, 701, p[306], 702, p[307], 703, p[308], p[305]);s.store_primal_add_scaled_inputs3_offset_indices(347, 701, p[491], 702, p[492], 703, p[493], p[490]);s.store_add_scaled_inputs3_offset_indices(779, 701, p[507], 702, p[508], 703, p[509], p[506]);}
        s.store_scalar(167, ((p[80] * ((((s.v[694]) as f64).powf(p[81]) - ((s.v[699]) as f64).powf(p[81]))).max(0.0)) + (p[82] * ((((s.v[694]) as f64).powf(p[83]) - ((s.v[699]) as f64).powf(p[83]))).max(0.0))));s.store_scalar(168, ((p[84] * ((((s.v[695]) as f64).powf(p[85]) - ((s.v[700]) as f64).powf(p[85]))).max(0.0)) + (p[86] * (((s.v[695] * s.v[694])) as f64).powf(p[87]))));s.store_scale(706, 706, ((1.0 + s.v[167]) + s.v[168]));s.store_scalar(167, (p[237] * ((((s.v[694]) as f64).powf(p[238]) - ((s.v[699]) as f64).powf(p[238]))).max(0.0)));s.store_scalar(168, ((p[239] * ((((s.v[695]) as f64).powf(p[240]) - ((s.v[700]) as f64).powf(p[240]))).max(0.0)) + (p[241] * ((s.v[696]) as f64).powf(p[242]))));s.store_scale(720, 720, ((1.0 + s.v[167]) + s.v[168]));s.store_scalar(167, (1.0 + (p[282] * ((((s.v[694]) as f64).powf(p[283]) - ((s.v[699]) as f64).powf(p[283]))).max(0.0))));s.store_scale(710, 710, s.v[167]);s.b[1150] = (p[35] != 0.0);s.store_scalar(1150, if s.b[1150] { 1.0 } else { 0.0 });
        if s.b[1150] {s.store_scale(839, 839, s.v[167]);s.store_scale(717, 717, s.v[167]);}
        s.store_primal_scale(719, 719, (1.0 + (p[289] * ((((s.v[694]) as f64).powf(p[290]) - ((s.v[699]) as f64).powf(p[290]))).max(0.0))));s.store_primal_scale(738, 738, p[24]);s.b[1151] = (p[42] != 1.0);s.store_scalar(1151, if s.b[1151] { 1.0 } else { 0.0 });s.b[1152] = (p[339] > 0.0);s.store_scalar(1152, if s.b[1152] { 1.0 } else { 0.0 });
        if (s.b[1151] && s.b[1152]) {s.store_primal_scale(738, 738, (1.0 - (p[338] * ((((s.v[694]) as f64).powf(p[339]) - ((s.v[699]) as f64).powf(p[339]))).max(0.0))));}
        s.b[1153] = (p[35] != 0.0);s.store_scalar(1153, if s.b[1153] { 1.0 } else { 0.0 });
        if ((s.b[1151] && s.b[1152]) && s.b[1153]) {s.store_primal_scale(739, 739, (1.0 - (p[338] * ((((s.v[694]) as f64).powf(p[339]) - ((s.v[699]) as f64).powf(p[339]))).max(0.0))));}
        if (s.b[1151] && (!s.b[1152])) {s.store_primal_scale(738, 738, (1.0 - p[338]));}
        s.b[1154] = (p[35] != 0.0);s.store_scalar(1154, if s.b[1154] { 1.0 } else { 0.0 });
        if ((s.b[1151] && (!s.b[1152])) && s.b[1154]) {s.store_primal_scale(739, 739, (1.0 - p[338]));}
        if (!s.b[1151]) {s.store_primal_scale(738, 738, ((1.0 - (p[333] * { let limited_exp_arg = ((-s.v[184]) / p[334]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })) - (p[335] * { let limited_exp_arg = ((-s.v[184]) / p[336]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })));}
        s.b[1155] = (p[35] != 0.0);s.store_scalar(1155, if s.b[1155] { 1.0 } else { 0.0 });
        if ((!s.b[1151]) && s.b[1155]) {s.store_primal_scale(739, 739, ((1.0 - (p[333] * { let limited_exp_arg = ((-s.v[184]) / p[334]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })) - (p[335] * { let limited_exp_arg = ((-s.v[184]) / p[336]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })));}
        s.store_scalar(167, (p[349] * ((((s.v[694]) as f64).powf(p[350]) - ((s.v[699]) as f64).powf(p[350]))).max(0.0)));s.store_scalar(168, ((p[351] * ((((s.v[695]) as f64).powf(p[352]) - ((s.v[700]) as f64).powf(p[352]))).max(0.0)) + (p[353] * ((s.v[696]) as f64).powf(p[354]))));s.store_scale(748, 748, ((1.0 + s.v[167]) + s.v[168]));s.b[1156] = (p[35] != 0.0);s.store_scalar(1156, if s.b[1156] { 1.0 } else { 0.0 });
        if s.b[1156] {s.store_scale(749, 749, ((1.0 + s.v[167]) + s.v[168]));}
        s.store_scalar(167, (p[366] * ((((s.v[694]) as f64).powf(p[367]) - ((s.v[699]) as f64).powf(p[367]))).max(0.0)));s.store_scalar(168, ((p[368] * ((((s.v[695]) as f64).powf(p[369]) - ((s.v[700]) as f64).powf(p[369]))).max(0.0)) + (p[370] * ((s.v[696]) as f64).powf(p[371]))));s.store_scale(751, 751, ((1.0 + s.v[167]) + s.v[168]));s.store_scalar(167, (1.0 + (p[373] * ((((s.v[694]) as f64).powf(p[374]) - ((s.v[699]) as f64).powf(p[374]))).max(0.0))));s.store_scale(752, 752, s.v[167]);s.b[1157] = (p[35] != 0.0);s.store_scalar(1157, if s.b[1157] { 1.0 } else { 0.0 });
        if s.b[1157] {s.store_scale(753, 753, s.v[167]);}
        s.store_scalar(167, (p[391] * ((((s.v[694]) as f64).powf(p[392]) - ((s.v[699]) as f64).powf(p[392]))).max(0.0)));s.store_scalar(168, ((p[393] * ((((s.v[695]) as f64).powf(p[394]) - ((s.v[700]) as f64).powf(p[394]))).max(0.0)) + (p[395] * ((s.v[696]) as f64).powf(p[396]))));s.store_scale(758, 758, ((1.0 + s.v[167]) + s.v[168]));s.b[1158] = (p[35] != 0.0);s.store_scalar(1158, if s.b[1158] { 1.0 } else { 0.0 });
        if s.b[1158] {s.store_scale(759, 759, ((1.0 + s.v[167]) + s.v[168]));}
        s.store_scalar(167, ((((s.v[694]) as f64).powf(p[202]) - ((s.v[699]) as f64).powf(p[202]))).max(0.0));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_scale(730, 730, s.v[167]);s.b[1159] = (p[35] != 0.0);s.store_scalar(1159, if s.b[1159] { 1.0 } else { 0.0 });
        if s.b[1159] {s.store_scale(731, 731, s.v[167]);}
        s.store_primal_scale(733, 733, ((((s.v[694]) as f64).powf(p[204]) - ((s.v[699]) as f64).powf(p[204]))).max(0.0));s.store_scalar(167, (1.0 + (p[531] * ((((s.v[694]) as f64).powf(p[532]) - ((s.v[699]) as f64).powf(p[532]))).max(0.0))));s.store_scale(761, 761, s.v[167]);s.b[1160] = (p[35] != 0.0);s.store_scalar(1160, if s.b[1160] { 1.0 } else { 0.0 });
        if s.b[1160] {s.store_scale(762, 762, s.v[167]);}
        s.store_scale(167, 737, (1.0 + (p[313] * ((((s.v[694]) as f64).powf(p[314]) - ((s.v[699]) as f64).powf(p[314]))).max(0.0))));s.store_min_with_scalar(737, 167, 0.5);s.store_primal_scale(769, 769, (1.0 + (p[549] * ((((s.v[694]) as f64).powf(p[550]) - ((s.v[699]) as f64).powf(p[550]))).max(0.0))));s.store_scalar(167, (1.0 + (p[405] * ((((s.v[694]) as f64).powf(p[406]) - ((s.v[699]) as f64).powf(p[406]))).max(0.0))));s.store_scale(783, 783, s.v[167]);s.store_max_with_scalar(783, 783, 0.0);s.b[1161] = (p[35] != 0.0);s.store_scalar(1161, if s.b[1161] { 1.0 } else { 0.0 });
        if s.b[1161] {s.store_scale(784, 784, s.v[167]);s.store_max_with_scalar(784, 784, 0.0);}
        s.store_scalar(167, (p[299] * ((((s.v[694]) as f64).powf(p[300]) - ((s.v[699]) as f64).powf(p[300]))).max(0.0)));s.store_scalar(168, ((p[301] * ((((s.v[695]) as f64).powf(p[302]) - ((s.v[700]) as f64).powf(p[302]))).max(0.0)) + (p[303] * ((s.v[696]) as f64).powf(p[304]))));s.store_scale(741, 741, ((1.0 + s.v[167]) + s.v[168]));s.b[1162] = (p[35] != 0.0);s.store_scalar(1162, if s.b[1162] { 1.0 } else { 0.0 });
        if s.b[1162] {s.store_scale(745, 745, ((1.0 + s.v[167]) + s.v[168]));}
        s.store_primal_max_with_scalar_ad(346, A::scale(s.ad_value(346), (1.0 + (p[487] * ((((s.v[694]) as f64).powf(p[488]) - ((s.v[699]) as f64).powf(p[488]))).max(0.0)))), 0.25);s.b[1163] = (p[35] != 0.0);s.store_scalar(1163, if s.b[1163] { 1.0 } else { 0.0 });
        if s.b[1163] {s.store_primal_max_with_scalar_ad(347, A::scale(s.ad_value(347), (1.0 + (p[487] * ((((s.v[694]) as f64).powf(p[488]) - ((s.v[699]) as f64).powf(p[488]))).max(0.0)))), 0.25);}
        s.store_scalar(167, (1.0 + (p[502] * ((((s.v[694]) as f64).powf(p[505]) - ((s.v[699]) as f64).powf(p[505]))).max(0.0))));s.store_scale(778, 778, s.v[167]);s.b[1164] = (p[35] != 0.0);s.store_scalar(1164, if s.b[1164] { 1.0 } else { 0.0 });
        if s.b[1164] {s.store_scale(779, 779, s.v[167]);}
        s.store_primal_scale(865, 865, (1.0 + (p[602] * ((((s.v[694]) as f64).powf(p[603]) - ((s.v[699]) as f64).powf(p[603]))).max(0.0))));s.store_primal_scale(892, 892, ((1.0 + (p[800] * s.v[694])) + (p[801] * s.v[695])));s.store_primal_scale(896, 896, ((1.0 + (p[822] * s.v[694])) + (p[823] * s.v[695])));s.store_scale(810, 810, ((1.0 + (p[724] * s.v[694])) + (p[725] * s.v[695])));s.store_scale(816, 816, ((1.0 + (p[727] * s.v[694])) + (p[728] * s.v[695])));s.store_scale(819, 819, ((1.0 + (p[729] * s.v[694])) + (p[730] * s.v[695])));s.store_scalar(823, (p[723] * (1.0 + (p[731] * s.v[694]))));s.store_scalar(167, ((p[92] * ((((s.v[697]) as f64).powf(p[93]) - ((s.v[699]) as f64).powf(p[93]))).max(0.0)) + (p[94] * ((((s.v[697]) as f64).powf(p[95]) - ((s.v[699]) as f64).powf(p[95]))).max(0.0))));s.store_scalar(168, ((p[96] * ((((s.v[698]) as f64).powf(p[97]) - ((s.v[700]) as f64).powf(p[97]))).max(0.0)) + (p[98] * (((s.v[698] * s.v[697])) as f64).powf(p[99]))));s.store_scale(794, 794, ((1.0 + s.v[167]) + s.v[168]));s.b[1165] = (p[29] == 1.0);s.store_scalar(1165, if s.b[1165] { 1.0 } else { 0.0 });
        if s.b[1165] {s.copy_ad(794, 706);}
        if (!s.b[1165]) {
        }
        s.store_scalar(167, (p[123] * ((((s.v[694]) as f64).powf(p[124]) - ((s.v[699]) as f64).powf(p[124]))).max(0.0)));s.store_scalar(168, ((p[125] * ((((s.v[695]) as f64).powf(p[126]) - ((s.v[700]) as f64).powf(p[126]))).max(0.0)) + (p[127] * ((s.v[696]) as f64).powf(p[128]))));s.store_scale(707, 707, ((1.0 + s.v[167]) + s.v[168]));s.store_scalar(167, (p[133] * ((((s.v[697]) as f64).powf(p[134]) - ((s.v[699]) as f64).powf(p[134]))).max(0.0)));s.store_scalar(168, ((p[135] * ((((s.v[698]) as f64).powf(p[136]) - ((s.v[700]) as f64).powf(p[136]))).max(0.0)) + (p[137] * (((s.v[698] * s.v[697])) as f64).powf(p[138]))));s.store_scale(793, 793, ((1.0 + s.v[167]) + s.v[168]));s.store_scalar(167, (p[319] * ((((s.v[697]) as f64).powf(p[320]) - ((s.v[699]) as f64).powf(p[320]))).max(0.0)));s.store_scalar(168, ((p[321] * ((((s.v[698]) as f64).powf(p[322]) - ((s.v[700]) as f64).powf(p[322]))).max(0.0)) + (p[323] * (((s.v[698] * s.v[697])) as f64).powf(p[324]))));s.store_scale(747, 747, ((1.0 + s.v[167]) + s.v[168]));s.store_primal_scale(786, 786, (1.0 + (p[416] * ((((s.v[697]) as f64).powf(p[417]) - ((s.v[699]) as f64).powf(p[417]))).max(0.0))));s.store_primal_max_with_scalar(786, 786, 0.0);s.store_scalar(167, (p[209] * ((((s.v[694]) as f64).powf(p[210]) - ((s.v[699]) as f64).powf(p[210]))).max(0.0)));s.store_scalar(168, ((p[211] * ((((s.v[695]) as f64).powf(p[212]) - ((s.v[700]) as f64).powf(p[212]))).max(0.0)) + (p[213] * ((s.v[696]) as f64).powf(p[214]))));s.store_scale(735, 735, ((1.0 + s.v[167]) + s.v[168]));s.store_scalar(167, (p[1197] * ((((s.v[694]) as f64).powf(p[1198]) - ((s.v[699]) as f64).powf(p[1198]))).max(0.0)));s.store_scalar(168, ((p[1199] * ((((s.v[695]) as f64).powf(p[1200]) - ((s.v[700]) as f64).powf(p[1200]))).max(0.0)) + (p[1201] * ((s.v[696]) as f64).powf(p[1202]))));s.store_scale(736, 736, ((1.0 + s.v[167]) + s.v[168]));s.store_scalar(167, (p[219] * ((((s.v[694]) as f64).powf(p[220]) - ((s.v[699]) as f64).powf(p[220]))).max(0.0)));s.store_scalar(168, ((p[221] * ((((s.v[695]) as f64).powf(p[222]) - ((s.v[700]) as f64).powf(p[222]))).max(0.0)) + (p[223] * ((s.v[696]) as f64).powf(p[224]))));s.store_scale(734, 734, ((1.0 + s.v[167]) + s.v[168]));s.store_scalar(167, (p[1266] * ((((s.v[694]) as f64).powf(p[1267]) - ((s.v[699]) as f64).powf(p[1267]))).max(0.0)));s.store_scalar(168, ((p[1268] * ((((s.v[695]) as f64).powf(p[1269]) - ((s.v[700]) as f64).powf(p[1269]))).max(0.0)) + (p[1270] * ((s.v[696]) as f64).powf(p[1271]))));s.store_scale(849, 849, ((1.0 + s.v[167]) + s.v[168]));s.store_primal_scale(787, 787, (1.0 + (p[447] * ((((s.v[694]) as f64).powf(p[448]) - ((s.v[699]) as f64).powf(p[448]))).max(0.0))));s.store_primal_scale(796, 796, (1.0 + (s.v[694] * p[1036])));s.store_primal_scale(797, 797, (1.0 + (s.v[694] * p[1041])));s.store_primal_scale(799, 799, (1.0 + (s.v[694] * p[1050])));s.store_primal_scale(802, 802, (1.0 + (s.v[694] * p[1068])));s.store_primal_scale(803, 803, (1.0 + (s.v[694] * p[1074])));s.b[1166] = (p[33] == 1.0);s.store_scalar(1166, if s.b[1166] { 1.0 } else { 0.0 });
        if s.b[1166] {s.store_primal_scale(775, 775, (1.0 + (p[461] * ((((s.v[694]) as f64).powf(p[462]) - ((s.v[699]) as f64).powf(p[462]))).max(0.0))));s.store_primal_scale(774, 774, (1.0 + (p[471] * ((((s.v[694]) as f64).powf(p[472]) - ((s.v[699]) as f64).powf(p[472]))).max(0.0))));}
        if (!s.b[1166]) {s.store_primal_scale(776, 776, (1.0 + (p[478] * ((((s.v[694]) as f64).powf(p[479]) - ((s.v[699]) as f64).powf(p[479]))).max(0.0))));}
        s.b[1167] = (s.v[755] < 1.0);s.store_scalar(1167, if s.b[1167] { 1.0 } else { 0.0 });
        if s.b[1167] {s.store_scalar(755, 1.0);}
        s.b[1168] = (s.v[755] > 2.0);s.store_scalar(1168, if s.b[1168] { 1.0 } else { 0.0 });
        if ((!s.b[1167]) && s.b[1168]) {s.store_scalar(755, 2.0);}
        s.b[1169] = (p[35] != 0.0);s.store_scalar(1169, if s.b[1169] { 1.0 } else { 0.0 });s.b[1170] = (s.v[756] < 1.0);s.store_scalar(1170, if s.b[1170] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1169] && s.b[1170]) {s.store_scalar(756, 1.0);}
        s.b[1171] = (s.v[756] > 2.0);s.store_scalar(1171, if s.b[1171] { 1.0 } else { 0.0 });
        if ((s.b[1169] && (!s.b[1170])) && s.b[1171]) {s.store_scalar(756, 2.0);}
        s.b[1196] = (s.v[829] < 0.0);s.store_scalar(1196, if s.b[1196] { 1.0 } else { 0.0 });
        if s.b[1196] {s.store_scalar(829, 0.0);}
        s.b[1197] = (s.v[738] <= 0.0);s.store_scalar(1197, if s.b[1197] { 1.0 } else { 0.0 });
        if s.b[1197] {s.store_scalar(738, 0.067);}
        s.b[1198] = (s.v[748] < 0.0);s.store_scalar(1198, if s.b[1198] { 1.0 } else { 0.0 });
        if s.b[1198] {s.store_scalar(748, 0.0);}
        s.b[1199] = (s.v[751] < 0.0);s.store_scalar(1199, if s.b[1199] { 1.0 } else { 0.0 });
        if s.b[1199] {s.store_scalar(751, 0.0);}
        s.b[1200] = (s.v[752] < 0.0);s.store_scalar(1200, if s.b[1200] { 1.0 } else { 0.0 });
        if s.b[1200] {s.store_scalar(752, 0.0);}
        s.b[1201] = (s.v[755] < 0.0);s.store_scalar(1201, if s.b[1201] { 1.0 } else { 0.0 });
        if s.b[1201] {s.store_scalar(755, 0.0);}
        s.b[1202] = (s.v[590] <= 0.0);s.store_scalar(1202, if s.b[1202] { 1.0 } else { 0.0 });
        if s.b[1202] {s.store_scalar(590, 1.0);}
        s.b[1203] = (s.v[564] <= 0.0);s.store_scalar(1203, if s.b[1203] { 1.0 } else { 0.0 });
        if s.b[1203] {s.store_scalar(564, 10.0);}
        s.b[1204] = (s.v[557] <= 0.0);s.store_scalar(1204, if s.b[1204] { 1.0 } else { 0.0 });
        if s.b[1204] {s.store_scalar(557, 2.0);}
        s.store_scalar(969, 0.0);s.store_scalar(971, 0.0);s.store_scalar(968, 0.0);s.store_scalar(970, 0.0);s.store_scalar(973, 0.0);s.store_scalar(972, 0.0);s.store_scalar(449, (p[895] - p[898]));s.store_scalar(451, p[896]);s.store_scalar(450, (p[897] - p[898]));s.b[1206] = param_given[3];s.store_scalar(1206, if s.b[1206] { 1.0 } else { 0.0 });
        if s.b[1206] {s.store_scalar(452, (p[438] * p[3]));}
        s.b[1207] = ((p[9] > 0.0) && (p[438] > 0.0));s.store_scalar(1207, if s.b[1207] { 1.0 } else { 0.0 });s.b[1208] = (p[8] < 9.0);s.store_scalar(1208, if s.b[1208] { 1.0 } else { 0.0 });s.b[1209] = ((p[2] % 2.0) != 0.0);s.store_scalar(1209, if s.b[1209] { 1.0 } else { 0.0 });
        if ((((!s.b[1206]) && s.b[1207]) && s.b[1208]) && s.b[1209]) {s.store_scalar(969, 1.0);s.store_scalar(971, 1.0);s.store_scalar(968, (2.0 * (((p[2] - 1.0) / 2.0)).max(0.0)));s.copy_ad(970, 968);}
        s.b[1210] = (p[6] == 1.0);s.store_scalar(1210, if s.b[1210] { 1.0 } else { 0.0 });
        if (((((!s.b[1206]) && s.b[1207]) && s.b[1208]) && (!s.b[1209])) && s.b[1210]) {s.store_scalar(969, 2.0);s.store_scalar(968, (2.0 * (((p[2] / 2.0) - 1.0)).max(0.0)));s.store_scalar(971, 0.0);s.store_scalar(970, p[2]);}
        if (((((!s.b[1206]) && s.b[1207]) && s.b[1208]) && (!s.b[1209])) && (!s.b[1210])) {s.store_scalar(969, 0.0);s.store_scalar(968, p[2]);s.store_scalar(971, 2.0);s.store_scalar(970, (2.0 * (((p[2] / 2.0) - 1.0)).max(0.0)));}
        s.b[1211] = (1.0 == 1.0);s.store_scalar(1211, if s.b[1211] { 1.0 } else { 0.0 });s.b[1212] = (s.v[970] == 0.0);s.store_scalar(1212, if s.b[1212] { 1.0 } else { 0.0 });
        if (((((!s.b[1206]) && s.b[1207]) && s.b[1208]) && s.b[1211]) && s.b[1212]) {s.store_scalar(972, 0.0);}
        if (((((!s.b[1206]) && s.b[1207]) && s.b[1208]) && s.b[1211]) && (!s.b[1212])) {s.store_primal_div_from_scalar_scaled_input(972, (p[438] * s.v[449]), 970, s.v[183]);}
        s.b[1213] = (s.v[968] == 0.0);s.store_scalar(1213, if s.b[1213] { 1.0 } else { 0.0 });
        if (((((!s.b[1206]) && s.b[1207]) && s.b[1208]) && (!s.b[1211])) && s.b[1213]) {s.store_scalar(972, 0.0);}
        if (((((!s.b[1206]) && s.b[1207]) && s.b[1208]) && (!s.b[1211])) && (!s.b[1213])) {s.store_primal_div_from_scalar_scaled_input(972, (p[438] * s.v[449]), 968, s.v[183]);}
        s.b[1214] = (p[8] == 0.0);s.store_scalar(1214, if s.b[1214] { 1.0 } else { 0.0 });s.b[1215] = (p[8] == 1.0);s.store_scalar(1215, if s.b[1215] { 1.0 } else { 0.0 });s.b[1216] = (p[8] == 2.0);s.store_scalar(1216, if s.b[1216] { 1.0 } else { 0.0 });s.b[1217] = (p[8] == 3.0);s.store_scalar(1217, if s.b[1217] { 1.0 } else { 0.0 });s.b[1218] = (p[8] == 4.0);s.store_scalar(1218, if s.b[1218] { 1.0 } else { 0.0 });s.b[1219] = (p[8] == 5.0);s.store_scalar(1219, if s.b[1219] { 1.0 } else { 0.0 });s.b[1220] = (p[8] == 6.0);s.store_scalar(1220, if s.b[1220] { 1.0 } else { 0.0 });s.b[1221] = (p[8] == 7.0);s.store_scalar(1221, if s.b[1221] { 1.0 } else { 0.0 });s.b[1222] = (p[8] == 8.0);s.store_scalar(1222, if s.b[1222] { 1.0 } else { 0.0 });s.b[1223] = (p[8] == 9.0);s.store_scalar(1223, if s.b[1223] { 1.0 } else { 0.0 });s.b[1224] = (p[8] == 10.0);s.store_scalar(1224, if s.b[1224] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1225] = (1.0 == 1.0);s.store_scalar(1225, if s.b[1225] { 1.0 } else { 0.0 });s.b[1226] = (1.0 == 1.0);s.store_scalar(1226, if s.b[1226] { 1.0 } else { 0.0 });s.b[1227] = (((p[9] == 1.0) || (p[9] == 2.0)) || (p[9] == 5.0));s.store_scalar(1227, if s.b[1227] { 1.0 } else { 0.0 });s.b[1228] = (((p[9] == 3.0) || (p[9] == 4.0)) || (p[9] == 6.0));s.store_scalar(1228, if s.b[1228] { 1.0 } else { 0.0 });s.b[1229] = (s.v[971] == 0.0);s.store_scalar(1229, if s.b[1229] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && s.b[1225]) && s.b[1226]) && s.b[1227]) && s.b[1229]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && s.b[1225]) && s.b[1226]) && s.b[1227]) && (!s.b[1229])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 971, s.v[183]);}
        s.b[1231] = ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));s.store_scalar(1231, if s.b[1231] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && s.b[1225]) && s.b[1226]) && (s.b[1228] && (!s.b[1227]))) && s.b[1231]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && s.b[1225]) && s.b[1226]) && (s.b[1228] && (!s.b[1227]))) && (!s.b[1231])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 971, (3.0 * (s.v[449] + s.v[451])));}
        if ((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && s.b[1225]) && s.b[1226]) && (!(s.b[1227] || s.b[1228]))) {s.store_scalar(973, 0.0);}
        s.b[1232] = (((p[9] == 1.0) || (p[9] == 3.0)) || (p[9] == 7.0));s.store_scalar(1232, if s.b[1232] { 1.0 } else { 0.0 });s.b[1233] = (((p[9] == 2.0) || (p[9] == 4.0)) || (p[9] == 8.0));s.store_scalar(1233, if s.b[1233] { 1.0 } else { 0.0 });s.b[1234] = (s.v[971] == 0.0);s.store_scalar(1234, if s.b[1234] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && s.b[1225]) && (!s.b[1226])) && s.b[1232]) && s.b[1234]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && s.b[1225]) && (!s.b[1226])) && s.b[1232]) && (!s.b[1234])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 971, s.v[183]);}
        s.b[1236] = ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));s.store_scalar(1236, if s.b[1236] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && s.b[1225]) && (!s.b[1226])) && (s.b[1233] && (!s.b[1232]))) && s.b[1236]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && s.b[1225]) && (!s.b[1226])) && (s.b[1233] && (!s.b[1232]))) && (!s.b[1236])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 971, (3.0 * (s.v[449] + s.v[451])));}
        if ((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && s.b[1225]) && (!s.b[1226])) && (!(s.b[1232] || s.b[1233]))) {s.store_scalar(973, 0.0);}
        s.b[1237] = (0.0 == 1.0);s.store_scalar(1237, if s.b[1237] { 1.0 } else { 0.0 });s.b[1238] = (((p[9] == 1.0) || (p[9] == 2.0)) || (p[9] == 5.0));s.store_scalar(1238, if s.b[1238] { 1.0 } else { 0.0 });s.b[1239] = (((p[9] == 3.0) || (p[9] == 4.0)) || (p[9] == 6.0));s.store_scalar(1239, if s.b[1239] { 1.0 } else { 0.0 });s.b[1240] = (s.v[969] == 0.0);s.store_scalar(1240, if s.b[1240] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && (!s.b[1225])) && s.b[1237]) && s.b[1238]) && s.b[1240]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && (!s.b[1225])) && s.b[1237]) && s.b[1238]) && (!s.b[1240])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 969, s.v[183]);}
        s.b[1242] = ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));s.store_scalar(1242, if s.b[1242] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && (!s.b[1225])) && s.b[1237]) && (s.b[1239] && (!s.b[1238]))) && s.b[1242]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && (!s.b[1225])) && s.b[1237]) && (s.b[1239] && (!s.b[1238]))) && (!s.b[1242])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 969, (3.0 * (s.v[449] + s.v[451])));}
        if ((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && (!s.b[1225])) && s.b[1237]) && (!(s.b[1238] || s.b[1239]))) {s.store_scalar(973, 0.0);}
        s.b[1243] = (((p[9] == 1.0) || (p[9] == 3.0)) || (p[9] == 7.0));s.store_scalar(1243, if s.b[1243] { 1.0 } else { 0.0 });s.b[1244] = (((p[9] == 2.0) || (p[9] == 4.0)) || (p[9] == 8.0));s.store_scalar(1244, if s.b[1244] { 1.0 } else { 0.0 });s.b[1245] = (s.v[969] == 0.0);s.store_scalar(1245, if s.b[1245] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && (!s.b[1225])) && (!s.b[1237])) && s.b[1243]) && s.b[1245]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && (!s.b[1225])) && (!s.b[1237])) && s.b[1243]) && (!s.b[1245])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 969, s.v[183]);}
        s.b[1247] = ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));s.store_scalar(1247, if s.b[1247] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && (!s.b[1225])) && (!s.b[1237])) && (s.b[1244] && (!s.b[1243]))) && s.b[1247]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && (!s.b[1225])) && (!s.b[1237])) && (s.b[1244] && (!s.b[1243]))) && (!s.b[1247])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 969, (3.0 * (s.v[449] + s.v[451])));}
        if ((((((!s.b[1206]) && s.b[1207]) && s.b[1214]) && (!s.b[1225])) && (!s.b[1237])) && (!(s.b[1243] || s.b[1244]))) {s.store_scalar(973, 0.0);}
        s.b[1248] = (1.0 == 1.0);s.store_scalar(1248, if s.b[1248] { 1.0 } else { 0.0 });s.b[1249] = (1.0 == 1.0);s.store_scalar(1249, if s.b[1249] { 1.0 } else { 0.0 });s.b[1250] = (((p[9] == 1.0) || (p[9] == 2.0)) || (p[9] == 5.0));s.store_scalar(1250, if s.b[1250] { 1.0 } else { 0.0 });s.b[1251] = (((p[9] == 3.0) || (p[9] == 4.0)) || (p[9] == 6.0));s.store_scalar(1251, if s.b[1251] { 1.0 } else { 0.0 });s.b[1252] = (s.v[971] == 0.0);s.store_scalar(1252, if s.b[1252] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && s.b[1248]) && s.b[1249]) && s.b[1250]) && s.b[1252]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && s.b[1248]) && s.b[1249]) && s.b[1250]) && (!s.b[1252])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 971, s.v[183]);}
        s.b[1254] = ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));s.store_scalar(1254, if s.b[1254] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && s.b[1248]) && s.b[1249]) && (s.b[1251] && (!s.b[1250]))) && s.b[1254]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && s.b[1248]) && s.b[1249]) && (s.b[1251] && (!s.b[1250]))) && (!s.b[1254])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 971, (3.0 * (s.v[449] + s.v[451])));}
        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && s.b[1248]) && s.b[1249]) && (!(s.b[1250] || s.b[1251]))) {s.store_scalar(973, 0.0);}
        s.b[1255] = (((p[9] == 1.0) || (p[9] == 3.0)) || (p[9] == 7.0));s.store_scalar(1255, if s.b[1255] { 1.0 } else { 0.0 });s.b[1256] = (((p[9] == 2.0) || (p[9] == 4.0)) || (p[9] == 8.0));s.store_scalar(1256, if s.b[1256] { 1.0 } else { 0.0 });s.b[1257] = (s.v[971] == 0.0);s.store_scalar(1257, if s.b[1257] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && s.b[1248]) && (!s.b[1249])) && s.b[1255]) && s.b[1257]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && s.b[1248]) && (!s.b[1249])) && s.b[1255]) && (!s.b[1257])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 971, s.v[183]);}
        s.b[1259] = ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));s.store_scalar(1259, if s.b[1259] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && s.b[1248]) && (!s.b[1249])) && (s.b[1256] && (!s.b[1255]))) && s.b[1259]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && s.b[1248]) && (!s.b[1249])) && (s.b[1256] && (!s.b[1255]))) && (!s.b[1259])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 971, (3.0 * (s.v[449] + s.v[451])));}
        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && s.b[1248]) && (!s.b[1249])) && (!(s.b[1255] || s.b[1256]))) {s.store_scalar(973, 0.0);}
        s.b[1260] = (0.0 == 1.0);s.store_scalar(1260, if s.b[1260] { 1.0 } else { 0.0 });s.b[1261] = (((p[9] == 1.0) || (p[9] == 2.0)) || (p[9] == 5.0));s.store_scalar(1261, if s.b[1261] { 1.0 } else { 0.0 });s.b[1262] = (((p[9] == 3.0) || (p[9] == 4.0)) || (p[9] == 6.0));s.store_scalar(1262, if s.b[1262] { 1.0 } else { 0.0 });s.b[1263] = (s.v[969] == 0.0);s.store_scalar(1263, if s.b[1263] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && (!s.b[1248])) && s.b[1260]) && s.b[1261]) && s.b[1263]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && (!s.b[1248])) && s.b[1260]) && s.b[1261]) && (!s.b[1263])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 969, s.v[183]);}
        s.b[1265] = ((s.v[969] == 0.0) || (s.v[449] == 0.0));s.store_scalar(1265, if s.b[1265] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && (!s.b[1248])) && s.b[1260]) && (s.b[1262] && (!s.b[1261]))) && s.b[1265]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && (!s.b[1248])) && s.b[1260]) && (s.b[1262] && (!s.b[1261]))) && (!s.b[1265])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 969, (6.0 * s.v[449]));}
        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && (!s.b[1248])) && s.b[1260]) && (!(s.b[1261] || s.b[1262]))) {s.store_scalar(973, 0.0);}
        s.b[1266] = (((p[9] == 1.0) || (p[9] == 3.0)) || (p[9] == 7.0));s.store_scalar(1266, if s.b[1266] { 1.0 } else { 0.0 });s.b[1267] = (((p[9] == 2.0) || (p[9] == 4.0)) || (p[9] == 8.0));s.store_scalar(1267, if s.b[1267] { 1.0 } else { 0.0 });s.b[1268] = (s.v[969] == 0.0);s.store_scalar(1268, if s.b[1268] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && (!s.b[1248])) && (!s.b[1260])) && s.b[1266]) && s.b[1268]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && (!s.b[1248])) && (!s.b[1260])) && s.b[1266]) && (!s.b[1268])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 969, s.v[183]);}
        s.b[1270] = ((s.v[969] == 0.0) || (s.v[449] == 0.0));s.store_scalar(1270, if s.b[1270] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && (!s.b[1248])) && (!s.b[1260])) && (s.b[1267] && (!s.b[1266]))) && s.b[1270]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && (!s.b[1248])) && (!s.b[1260])) && (s.b[1267] && (!s.b[1266]))) && (!s.b[1270])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 969, (6.0 * s.v[449]));}
        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1215] && (!s.b[1214]))) && (!s.b[1248])) && (!s.b[1260])) && (!(s.b[1266] || s.b[1267]))) {s.store_scalar(973, 0.0);}
        s.b[1271] = (1.0 == 1.0);s.store_scalar(1271, if s.b[1271] { 1.0 } else { 0.0 });s.b[1272] = (1.0 == 1.0);s.store_scalar(1272, if s.b[1272] { 1.0 } else { 0.0 });s.b[1273] = (((p[9] == 1.0) || (p[9] == 2.0)) || (p[9] == 5.0));s.store_scalar(1273, if s.b[1273] { 1.0 } else { 0.0 });s.b[1274] = (((p[9] == 3.0) || (p[9] == 4.0)) || (p[9] == 6.0));s.store_scalar(1274, if s.b[1274] { 1.0 } else { 0.0 });s.b[1275] = (s.v[971] == 0.0);s.store_scalar(1275, if s.b[1275] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && s.b[1271]) && s.b[1272]) && s.b[1273]) && s.b[1275]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && s.b[1271]) && s.b[1272]) && s.b[1273]) && (!s.b[1275])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 971, s.v[183]);}
        s.b[1277] = ((s.v[971] == 0.0) || (s.v[449] == 0.0));s.store_scalar(1277, if s.b[1277] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && s.b[1271]) && s.b[1272]) && (s.b[1274] && (!s.b[1273]))) && s.b[1277]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && s.b[1271]) && s.b[1272]) && (s.b[1274] && (!s.b[1273]))) && (!s.b[1277])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 971, (6.0 * s.v[449]));}
        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && s.b[1271]) && s.b[1272]) && (!(s.b[1273] || s.b[1274]))) {s.store_scalar(973, 0.0);}
        s.b[1278] = (((p[9] == 1.0) || (p[9] == 3.0)) || (p[9] == 7.0));s.store_scalar(1278, if s.b[1278] { 1.0 } else { 0.0 });s.b[1279] = (((p[9] == 2.0) || (p[9] == 4.0)) || (p[9] == 8.0));s.store_scalar(1279, if s.b[1279] { 1.0 } else { 0.0 });s.b[1280] = (s.v[971] == 0.0);s.store_scalar(1280, if s.b[1280] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && s.b[1271]) && (!s.b[1272])) && s.b[1278]) && s.b[1280]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && s.b[1271]) && (!s.b[1272])) && s.b[1278]) && (!s.b[1280])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 971, s.v[183]);}
        s.b[1282] = ((s.v[971] == 0.0) || (s.v[449] == 0.0));s.store_scalar(1282, if s.b[1282] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && s.b[1271]) && (!s.b[1272])) && (s.b[1279] && (!s.b[1278]))) && s.b[1282]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && s.b[1271]) && (!s.b[1272])) && (s.b[1279] && (!s.b[1278]))) && (!s.b[1282])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 971, (6.0 * s.v[449]));}
        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && s.b[1271]) && (!s.b[1272])) && (!(s.b[1278] || s.b[1279]))) {s.store_scalar(973, 0.0);}
        s.b[1283] = (0.0 == 1.0);s.store_scalar(1283, if s.b[1283] { 1.0 } else { 0.0 });s.b[1284] = (((p[9] == 1.0) || (p[9] == 2.0)) || (p[9] == 5.0));s.store_scalar(1284, if s.b[1284] { 1.0 } else { 0.0 });s.b[1285] = (((p[9] == 3.0) || (p[9] == 4.0)) || (p[9] == 6.0));s.store_scalar(1285, if s.b[1285] { 1.0 } else { 0.0 });s.b[1286] = (s.v[969] == 0.0);s.store_scalar(1286, if s.b[1286] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && (!s.b[1271])) && s.b[1283]) && s.b[1284]) && s.b[1286]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && (!s.b[1271])) && s.b[1283]) && s.b[1284]) && (!s.b[1286])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 969, s.v[183]);}
        s.b[1288] = ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));s.store_scalar(1288, if s.b[1288] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && (!s.b[1271])) && s.b[1283]) && (s.b[1285] && (!s.b[1284]))) && s.b[1288]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && (!s.b[1271])) && s.b[1283]) && (s.b[1285] && (!s.b[1284]))) && (!s.b[1288])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 969, (3.0 * (s.v[449] + s.v[451])));}
        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && (!s.b[1271])) && s.b[1283]) && (!(s.b[1284] || s.b[1285]))) {s.store_scalar(973, 0.0);}
        s.b[1289] = (((p[9] == 1.0) || (p[9] == 3.0)) || (p[9] == 7.0));s.store_scalar(1289, if s.b[1289] { 1.0 } else { 0.0 });s.b[1290] = (((p[9] == 2.0) || (p[9] == 4.0)) || (p[9] == 8.0));s.store_scalar(1290, if s.b[1290] { 1.0 } else { 0.0 });s.b[1291] = (s.v[969] == 0.0);s.store_scalar(1291, if s.b[1291] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && (!s.b[1271])) && (!s.b[1283])) && s.b[1289]) && s.b[1291]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && (!s.b[1271])) && (!s.b[1283])) && s.b[1289]) && (!s.b[1291])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 969, s.v[183]);}
        s.b[1293] = ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));s.store_scalar(1293, if s.b[1293] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && (!s.b[1271])) && (!s.b[1283])) && (s.b[1290] && (!s.b[1289]))) && s.b[1293]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && (!s.b[1271])) && (!s.b[1283])) && (s.b[1290] && (!s.b[1289]))) && (!s.b[1293])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 969, (3.0 * (s.v[449] + s.v[451])));}
        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1216] && (!(s.b[1214] || s.b[1215])))) && (!s.b[1271])) && (!s.b[1283])) && (!(s.b[1289] || s.b[1290]))) {s.store_scalar(973, 0.0);}
        s.b[1294] = (1.0 == 1.0);s.store_scalar(1294, if s.b[1294] { 1.0 } else { 0.0 });s.b[1295] = (1.0 == 1.0);s.store_scalar(1295, if s.b[1295] { 1.0 } else { 0.0 });s.b[1296] = (((p[9] == 1.0) || (p[9] == 2.0)) || (p[9] == 5.0));s.store_scalar(1296, if s.b[1296] { 1.0 } else { 0.0 });s.b[1297] = (((p[9] == 3.0) || (p[9] == 4.0)) || (p[9] == 6.0));s.store_scalar(1297, if s.b[1297] { 1.0 } else { 0.0 });s.b[1298] = (s.v[971] == 0.0);s.store_scalar(1298, if s.b[1298] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && s.b[1294]) && s.b[1295]) && s.b[1296]) && s.b[1298]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && s.b[1294]) && s.b[1295]) && s.b[1296]) && (!s.b[1298])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 971, s.v[183]);}
        s.b[1300] = ((s.v[971] == 0.0) || (s.v[449] == 0.0));s.store_scalar(1300, if s.b[1300] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && s.b[1294]) && s.b[1295]) && (s.b[1297] && (!s.b[1296]))) && s.b[1300]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && s.b[1294]) && s.b[1295]) && (s.b[1297] && (!s.b[1296]))) && (!s.b[1300])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 971, (6.0 * s.v[449]));}
        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && s.b[1294]) && s.b[1295]) && (!(s.b[1296] || s.b[1297]))) {s.store_scalar(973, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_9(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1301] = (((p[9] == 1.0) || (p[9] == 3.0)) || (p[9] == 7.0));s.store_scalar(1301, if s.b[1301] { 1.0 } else { 0.0 });s.b[1302] = (((p[9] == 2.0) || (p[9] == 4.0)) || (p[9] == 8.0));s.store_scalar(1302, if s.b[1302] { 1.0 } else { 0.0 });s.b[1303] = (s.v[971] == 0.0);s.store_scalar(1303, if s.b[1303] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && s.b[1294]) && (!s.b[1295])) && s.b[1301]) && s.b[1303]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && s.b[1294]) && (!s.b[1295])) && s.b[1301]) && (!s.b[1303])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 971, s.v[183]);}
        s.b[1305] = ((s.v[971] == 0.0) || (s.v[449] == 0.0));s.store_scalar(1305, if s.b[1305] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && s.b[1294]) && (!s.b[1295])) && (s.b[1302] && (!s.b[1301]))) && s.b[1305]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && s.b[1294]) && (!s.b[1295])) && (s.b[1302] && (!s.b[1301]))) && (!s.b[1305])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 971, (6.0 * s.v[449]));}
        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && s.b[1294]) && (!s.b[1295])) && (!(s.b[1301] || s.b[1302]))) {s.store_scalar(973, 0.0);}
        s.b[1306] = (0.0 == 1.0);s.store_scalar(1306, if s.b[1306] { 1.0 } else { 0.0 });s.b[1307] = (((p[9] == 1.0) || (p[9] == 2.0)) || (p[9] == 5.0));s.store_scalar(1307, if s.b[1307] { 1.0 } else { 0.0 });s.b[1308] = (((p[9] == 3.0) || (p[9] == 4.0)) || (p[9] == 6.0));s.store_scalar(1308, if s.b[1308] { 1.0 } else { 0.0 });s.b[1309] = (s.v[969] == 0.0);s.store_scalar(1309, if s.b[1309] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && (!s.b[1294])) && s.b[1306]) && s.b[1307]) && s.b[1309]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && (!s.b[1294])) && s.b[1306]) && s.b[1307]) && (!s.b[1309])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 969, s.v[183]);}
        s.b[1311] = ((s.v[969] == 0.0) || (s.v[449] == 0.0));s.store_scalar(1311, if s.b[1311] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && (!s.b[1294])) && s.b[1306]) && (s.b[1308] && (!s.b[1307]))) && s.b[1311]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && (!s.b[1294])) && s.b[1306]) && (s.b[1308] && (!s.b[1307]))) && (!s.b[1311])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 969, (6.0 * s.v[449]));}
        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && (!s.b[1294])) && s.b[1306]) && (!(s.b[1307] || s.b[1308]))) {s.store_scalar(973, 0.0);}
        s.b[1312] = (((p[9] == 1.0) || (p[9] == 3.0)) || (p[9] == 7.0));s.store_scalar(1312, if s.b[1312] { 1.0 } else { 0.0 });s.b[1313] = (((p[9] == 2.0) || (p[9] == 4.0)) || (p[9] == 8.0));s.store_scalar(1313, if s.b[1313] { 1.0 } else { 0.0 });s.b[1314] = (s.v[969] == 0.0);s.store_scalar(1314, if s.b[1314] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && (!s.b[1294])) && (!s.b[1306])) && s.b[1312]) && s.b[1314]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && (!s.b[1294])) && (!s.b[1306])) && s.b[1312]) && (!s.b[1314])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 969, s.v[183]);}
        s.b[1316] = ((s.v[969] == 0.0) || (s.v[449] == 0.0));s.store_scalar(1316, if s.b[1316] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && (!s.b[1294])) && (!s.b[1306])) && (s.b[1313] && (!s.b[1312]))) && s.b[1316]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && (!s.b[1294])) && (!s.b[1306])) && (s.b[1313] && (!s.b[1312]))) && (!s.b[1316])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 969, (6.0 * s.v[449]));}
        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1217] && (!((s.b[1214] || s.b[1215]) || s.b[1216])))) && (!s.b[1294])) && (!s.b[1306])) && (!(s.b[1312] || s.b[1313]))) {s.store_scalar(973, 0.0);}
        s.b[1317] = (1.0 == 1.0);s.store_scalar(1317, if s.b[1317] { 1.0 } else { 0.0 });s.b[1318] = (1.0 == 1.0);s.store_scalar(1318, if s.b[1318] { 1.0 } else { 0.0 });s.b[1319] = (((p[9] == 1.0) || (p[9] == 2.0)) || (p[9] == 5.0));s.store_scalar(1319, if s.b[1319] { 1.0 } else { 0.0 });s.b[1320] = (((p[9] == 3.0) || (p[9] == 4.0)) || (p[9] == 6.0));s.store_scalar(1320, if s.b[1320] { 1.0 } else { 0.0 });s.b[1321] = (s.v[971] == 0.0);s.store_scalar(1321, if s.b[1321] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1218] && (!(((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217])))) && s.b[1317]) && s.b[1318]) && s.b[1319]) && s.b[1321]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1218] && (!(((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217])))) && s.b[1317]) && s.b[1318]) && s.b[1319]) && (!s.b[1321])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 971, s.v[183]);}
        s.b[1323] = ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));s.store_scalar(1323, if s.b[1323] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1218] && (!(((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217])))) && s.b[1317]) && s.b[1318]) && (s.b[1320] && (!s.b[1319]))) && s.b[1323]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1218] && (!(((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217])))) && s.b[1317]) && s.b[1318]) && (s.b[1320] && (!s.b[1319]))) && (!s.b[1323])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 971, (3.0 * (s.v[449] + s.v[451])));}
        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1218] && (!(((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217])))) && s.b[1317]) && s.b[1318]) && (!(s.b[1319] || s.b[1320]))) {s.store_scalar(973, 0.0);}
        s.b[1324] = (((p[9] == 1.0) || (p[9] == 3.0)) || (p[9] == 7.0));s.store_scalar(1324, if s.b[1324] { 1.0 } else { 0.0 });s.b[1325] = (((p[9] == 2.0) || (p[9] == 4.0)) || (p[9] == 8.0));s.store_scalar(1325, if s.b[1325] { 1.0 } else { 0.0 });s.b[1326] = (s.v[971] == 0.0);s.store_scalar(1326, if s.b[1326] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1218] && (!(((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217])))) && s.b[1317]) && (!s.b[1318])) && s.b[1324]) && s.b[1326]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1218] && (!(((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217])))) && s.b[1317]) && (!s.b[1318])) && s.b[1324]) && (!s.b[1326])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 971, s.v[183]);}
        s.b[1328] = ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));s.store_scalar(1328, if s.b[1328] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1218] && (!(((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217])))) && s.b[1317]) && (!s.b[1318])) && (s.b[1325] && (!s.b[1324]))) && s.b[1328]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1218] && (!(((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217])))) && s.b[1317]) && (!s.b[1318])) && (s.b[1325] && (!s.b[1324]))) && (!s.b[1328])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 971, (3.0 * (s.v[449] + s.v[451])));}
        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1218] && (!(((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217])))) && s.b[1317]) && (!s.b[1318])) && (!(s.b[1324] || s.b[1325]))) {s.store_scalar(973, 0.0);}
        if ((((!s.b[1206]) && s.b[1207]) && (s.b[1218] && (!(((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217])))) && (!s.b[1317])) {s.store_scalar(973, ((p[438] * s.v[450]) / s.v[183]));}
        s.b[1329] = (1.0 == 1.0);s.store_scalar(1329, if s.b[1329] { 1.0 } else { 0.0 });s.b[1330] = (1.0 == 1.0);s.store_scalar(1330, if s.b[1330] { 1.0 } else { 0.0 });s.b[1331] = (((p[9] == 1.0) || (p[9] == 2.0)) || (p[9] == 5.0));s.store_scalar(1331, if s.b[1331] { 1.0 } else { 0.0 });s.b[1332] = (((p[9] == 3.0) || (p[9] == 4.0)) || (p[9] == 6.0));s.store_scalar(1332, if s.b[1332] { 1.0 } else { 0.0 });s.b[1333] = (s.v[971] == 0.0);s.store_scalar(1333, if s.b[1333] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1219] && (!((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218])))) && s.b[1329]) && s.b[1330]) && s.b[1331]) && s.b[1333]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1219] && (!((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218])))) && s.b[1329]) && s.b[1330]) && s.b[1331]) && (!s.b[1333])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 971, s.v[183]);}
        s.b[1335] = ((s.v[971] == 0.0) || (s.v[449] == 0.0));s.store_scalar(1335, if s.b[1335] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1219] && (!((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218])))) && s.b[1329]) && s.b[1330]) && (s.b[1332] && (!s.b[1331]))) && s.b[1335]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1219] && (!((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218])))) && s.b[1329]) && s.b[1330]) && (s.b[1332] && (!s.b[1331]))) && (!s.b[1335])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 971, (6.0 * s.v[449]));}
        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1219] && (!((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218])))) && s.b[1329]) && s.b[1330]) && (!(s.b[1331] || s.b[1332]))) {s.store_scalar(973, 0.0);}
        s.b[1336] = (((p[9] == 1.0) || (p[9] == 3.0)) || (p[9] == 7.0));s.store_scalar(1336, if s.b[1336] { 1.0 } else { 0.0 });s.b[1337] = (((p[9] == 2.0) || (p[9] == 4.0)) || (p[9] == 8.0));s.store_scalar(1337, if s.b[1337] { 1.0 } else { 0.0 });s.b[1338] = (s.v[971] == 0.0);s.store_scalar(1338, if s.b[1338] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1219] && (!((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218])))) && s.b[1329]) && (!s.b[1330])) && s.b[1336]) && s.b[1338]) {s.store_scalar(973, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1219] && (!((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218])))) && s.b[1329]) && (!s.b[1330])) && s.b[1336]) && (!s.b[1338])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 971, s.v[183]);}
        s.b[1340] = ((s.v[971] == 0.0) || (s.v[449] == 0.0));s.store_scalar(1340, if s.b[1340] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1219] && (!((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218])))) && s.b[1329]) && (!s.b[1330])) && (s.b[1337] && (!s.b[1336]))) && s.b[1340]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1219] && (!((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218])))) && s.b[1329]) && (!s.b[1330])) && (s.b[1337] && (!s.b[1336]))) && (!s.b[1340])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 971, (6.0 * s.v[449]));}
        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1219] && (!((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218])))) && s.b[1329]) && (!s.b[1330])) && (!(s.b[1336] || s.b[1337]))) {s.store_scalar(973, 0.0);}
        s.b[1341] = (s.v[969] == 0.0);s.store_scalar(1341, if s.b[1341] { 1.0 } else { 0.0 });
        if (((((!s.b[1206]) && s.b[1207]) && (s.b[1219] && (!((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218])))) && (!s.b[1329])) && s.b[1341]) {s.store_scalar(973, 0.0);}
        if (((((!s.b[1206]) && s.b[1207]) && (s.b[1219] && (!((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218])))) && (!s.b[1329])) && (!s.b[1341])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[450]), 969, s.v[183]);}
        s.b[1342] = (1.0 == 1.0);s.store_scalar(1342, if s.b[1342] { 1.0 } else { 0.0 });
        if ((((!s.b[1206]) && s.b[1207]) && (s.b[1220] && (!(((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219])))) && s.b[1342]) {s.store_scalar(973, ((p[438] * s.v[450]) / s.v[183]));}
        s.b[1343] = (0.0 == 1.0);s.store_scalar(1343, if s.b[1343] { 1.0 } else { 0.0 });s.b[1344] = (((p[9] == 1.0) || (p[9] == 2.0)) || (p[9] == 5.0));s.store_scalar(1344, if s.b[1344] { 1.0 } else { 0.0 });s.b[1345] = (((p[9] == 3.0) || (p[9] == 4.0)) || (p[9] == 6.0));s.store_scalar(1345, if s.b[1345] { 1.0 } else { 0.0 });s.b[1346] = (s.v[969] == 0.0);s.store_scalar(1346, if s.b[1346] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1220] && (!(((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219])))) && (!s.b[1342])) && s.b[1343]) && s.b[1344]) && s.b[1346]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1220] && (!(((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219])))) && (!s.b[1342])) && s.b[1343]) && s.b[1344]) && (!s.b[1346])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 969, s.v[183]);}
        s.b[1348] = ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));s.store_scalar(1348, if s.b[1348] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1220] && (!(((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219])))) && (!s.b[1342])) && s.b[1343]) && (s.b[1345] && (!s.b[1344]))) && s.b[1348]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1220] && (!(((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219])))) && (!s.b[1342])) && s.b[1343]) && (s.b[1345] && (!s.b[1344]))) && (!s.b[1348])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 969, (3.0 * (s.v[449] + s.v[451])));}
        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1220] && (!(((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219])))) && (!s.b[1342])) && s.b[1343]) && (!(s.b[1344] || s.b[1345]))) {s.store_scalar(973, 0.0);}
        s.b[1349] = (((p[9] == 1.0) || (p[9] == 3.0)) || (p[9] == 7.0));s.store_scalar(1349, if s.b[1349] { 1.0 } else { 0.0 });s.b[1350] = (((p[9] == 2.0) || (p[9] == 4.0)) || (p[9] == 8.0));s.store_scalar(1350, if s.b[1350] { 1.0 } else { 0.0 });s.b[1351] = (s.v[969] == 0.0);s.store_scalar(1351, if s.b[1351] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1220] && (!(((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219])))) && (!s.b[1342])) && (!s.b[1343])) && s.b[1349]) && s.b[1351]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1220] && (!(((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219])))) && (!s.b[1342])) && (!s.b[1343])) && s.b[1349]) && (!s.b[1351])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 969, s.v[183]);}
        s.b[1353] = ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));s.store_scalar(1353, if s.b[1353] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1220] && (!(((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219])))) && (!s.b[1342])) && (!s.b[1343])) && (s.b[1350] && (!s.b[1349]))) && s.b[1353]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1220] && (!(((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219])))) && (!s.b[1342])) && (!s.b[1343])) && (s.b[1350] && (!s.b[1349]))) && (!s.b[1353])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 969, (3.0 * (s.v[449] + s.v[451])));}
        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1220] && (!(((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219])))) && (!s.b[1342])) && (!s.b[1343])) && (!(s.b[1349] || s.b[1350]))) {s.store_scalar(973, 0.0);}
        s.b[1354] = (1.0 == 1.0);s.store_scalar(1354, if s.b[1354] { 1.0 } else { 0.0 });s.b[1355] = (s.v[971] == 0.0);s.store_scalar(1355, if s.b[1355] { 1.0 } else { 0.0 });
        if (((((!s.b[1206]) && s.b[1207]) && (s.b[1221] && (!((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220])))) && s.b[1354]) && s.b[1355]) {s.store_scalar(973, 0.0);}
        if (((((!s.b[1206]) && s.b[1207]) && (s.b[1221] && (!((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220])))) && s.b[1354]) && (!s.b[1355])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[450]), 971, s.v[183]);}
        s.b[1356] = (0.0 == 1.0);s.store_scalar(1356, if s.b[1356] { 1.0 } else { 0.0 });s.b[1357] = (((p[9] == 1.0) || (p[9] == 2.0)) || (p[9] == 5.0));s.store_scalar(1357, if s.b[1357] { 1.0 } else { 0.0 });s.b[1358] = (((p[9] == 3.0) || (p[9] == 4.0)) || (p[9] == 6.0));s.store_scalar(1358, if s.b[1358] { 1.0 } else { 0.0 });s.b[1359] = (s.v[969] == 0.0);s.store_scalar(1359, if s.b[1359] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1221] && (!((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220])))) && (!s.b[1354])) && s.b[1356]) && s.b[1357]) && s.b[1359]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1221] && (!((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220])))) && (!s.b[1354])) && s.b[1356]) && s.b[1357]) && (!s.b[1359])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 969, s.v[183]);}
        s.b[1361] = ((s.v[969] == 0.0) || (s.v[449] == 0.0));s.store_scalar(1361, if s.b[1361] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1221] && (!((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220])))) && (!s.b[1354])) && s.b[1356]) && (s.b[1358] && (!s.b[1357]))) && s.b[1361]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1221] && (!((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220])))) && (!s.b[1354])) && s.b[1356]) && (s.b[1358] && (!s.b[1357]))) && (!s.b[1361])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 969, (6.0 * s.v[449]));}
        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1221] && (!((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220])))) && (!s.b[1354])) && s.b[1356]) && (!(s.b[1357] || s.b[1358]))) {s.store_scalar(973, 0.0);}
        s.b[1362] = (((p[9] == 1.0) || (p[9] == 3.0)) || (p[9] == 7.0));s.store_scalar(1362, if s.b[1362] { 1.0 } else { 0.0 });s.b[1363] = (((p[9] == 2.0) || (p[9] == 4.0)) || (p[9] == 8.0));s.store_scalar(1363, if s.b[1363] { 1.0 } else { 0.0 });s.b[1364] = (s.v[969] == 0.0);s.store_scalar(1364, if s.b[1364] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1221] && (!((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220])))) && (!s.b[1354])) && (!s.b[1356])) && s.b[1362]) && s.b[1364]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1221] && (!((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220])))) && (!s.b[1354])) && (!s.b[1356])) && s.b[1362]) && (!s.b[1364])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 969, s.v[183]);}
        s.b[1366] = ((s.v[969] == 0.0) || (s.v[449] == 0.0));s.store_scalar(1366, if s.b[1366] { 1.0 } else { 0.0 });
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1221] && (!((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220])))) && (!s.b[1354])) && (!s.b[1356])) && (s.b[1363] && (!s.b[1362]))) && s.b[1366]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1206]) && s.b[1207]) && (s.b[1221] && (!((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220])))) && (!s.b[1354])) && (!s.b[1356])) && (s.b[1363] && (!s.b[1362]))) && (!s.b[1366])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 969, (6.0 * s.v[449]));}
        if ((((((!s.b[1206]) && s.b[1207]) && (s.b[1221] && (!((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220])))) && (!s.b[1354])) && (!s.b[1356])) && (!(s.b[1362] || s.b[1363]))) {s.store_scalar(973, 0.0);}
        if (((!s.b[1206]) && s.b[1207]) && (s.b[1222] && (!(((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220]) || s.b[1221])))) {s.store_scalar(973, ((p[438] * s.v[450]) / s.v[183]));}
        s.b[1367] = (1.0 == 1.0);s.store_scalar(1367, if s.b[1367] { 1.0 } else { 0.0 });
        if ((((!s.b[1206]) && s.b[1207]) && (s.b[1223] && (!((((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220]) || s.b[1221]) || s.b[1222])))) && s.b[1367]) {s.store_scalar(973, (((0.5 * p[438]) * s.v[449]) / s.v[183]));}
        s.b[1368] = (p[2] == 2.0);s.store_scalar(1368, if s.b[1368] { 1.0 } else { 0.0 });
        if (((((!s.b[1206]) && s.b[1207]) && (s.b[1223] && (!((((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220]) || s.b[1221]) || s.b[1222])))) && s.b[1367]) && s.b[1368]) {s.store_scalar(972, 0.0);}
        if (((((!s.b[1206]) && s.b[1207]) && (s.b[1223] && (!((((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220]) || s.b[1221]) || s.b[1222])))) && s.b[1367]) && (!s.b[1368])) {s.store_scalar(972, ((p[438] * s.v[449]) / (s.v[183] * (p[2] - 2.0))));}
        if ((((!s.b[1206]) && s.b[1207]) && (s.b[1223] && (!((((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220]) || s.b[1221]) || s.b[1222])))) && (!s.b[1367])) {s.store_scalar(973, 0.0);s.store_scalar(972, ((p[438] * s.v[449]) / (s.v[183] * p[2])));}
        s.b[1369] = (1.0 == 1.0);s.store_scalar(1369, if s.b[1369] { 1.0 } else { 0.0 });
        if ((((!s.b[1206]) && s.b[1207]) && (s.b[1224] && (!(((((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220]) || s.b[1221]) || s.b[1222]) || s.b[1223])))) && s.b[1369]) {s.store_scalar(973, 0.0);s.store_scalar(972, ((p[438] * s.v[449]) / (s.v[183] * p[2])));}
        if ((((!s.b[1206]) && s.b[1207]) && (s.b[1224] && (!(((((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220]) || s.b[1221]) || s.b[1222]) || s.b[1223])))) && (!s.b[1369])) {s.store_scalar(973, (((0.5 * p[438]) * s.v[449]) / s.v[183]));}
        s.b[1370] = (p[2] == 2.0);s.store_scalar(1370, if s.b[1370] { 1.0 } else { 0.0 });
        if (((((!s.b[1206]) && s.b[1207]) && (s.b[1224] && (!(((((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220]) || s.b[1221]) || s.b[1222]) || s.b[1223])))) && (!s.b[1369])) && s.b[1370]) {s.store_scalar(972, 0.0);}
        if (((((!s.b[1206]) && s.b[1207]) && (s.b[1224] && (!(((((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220]) || s.b[1221]) || s.b[1222]) || s.b[1223])))) && (!s.b[1369])) && (!s.b[1370])) {s.store_scalar(972, ((p[438] * s.v[449]) / (s.v[183] * (p[2] - 2.0))));}
        if (((!s.b[1206]) && s.b[1207]) && (!((((((((((s.b[1214] || s.b[1215]) || s.b[1216]) || s.b[1217]) || s.b[1218]) || s.b[1219]) || s.b[1220]) || s.b[1221]) || s.b[1222]) || s.b[1223]) || s.b[1224]))) {s.store_scalar(972, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1371] = (s.v[972] <= 0.0);s.store_scalar(1371, if s.b[1371] { 1.0 } else { 0.0 });
        if (((!s.b[1206]) && s.b[1207]) && s.b[1371]) {s.copy_ad(452, 973);}
        s.b[1372] = (s.v[973] <= 0.0);s.store_scalar(1372, if s.b[1372] { 1.0 } else { 0.0 });
        if ((((!s.b[1206]) && s.b[1207]) && (!s.b[1371])) && s.b[1372]) {s.copy_ad(452, 972);}
        if ((((!s.b[1206]) && s.b[1207]) && (!s.b[1371])) && (!s.b[1372])) {s.store_primal_div_scaled_product_add_scaled_denominator_indices(452, 972, 973, 1.0, 972, 1.0, 973, 1.0, 1.0);}
        if ((!s.b[1206]) && (!s.b[1207])) {s.store_scalar(452, 0.0);}
        s.b[1374] = param_given[4];s.store_scalar(1374, if s.b[1374] { 1.0 } else { 0.0 });
        if s.b[1374] {s.store_scalar(453, (p[438] * p[4]));}
        s.b[1375] = ((p[9] > 0.0) && (p[438] > 0.0));s.store_scalar(1375, if s.b[1375] { 1.0 } else { 0.0 });s.b[1376] = (p[8] < 9.0);s.store_scalar(1376, if s.b[1376] { 1.0 } else { 0.0 });s.b[1377] = ((p[2] % 2.0) != 0.0);s.store_scalar(1377, if s.b[1377] { 1.0 } else { 0.0 });
        if ((((!s.b[1374]) && s.b[1375]) && s.b[1376]) && s.b[1377]) {s.store_scalar(969, 1.0);s.store_scalar(971, 1.0);s.store_scalar(968, (2.0 * (((p[2] - 1.0) / 2.0)).max(0.0)));s.copy_ad(970, 968);}
        s.b[1378] = (p[6] == 1.0);s.store_scalar(1378, if s.b[1378] { 1.0 } else { 0.0 });
        if (((((!s.b[1374]) && s.b[1375]) && s.b[1376]) && (!s.b[1377])) && s.b[1378]) {s.store_scalar(969, 2.0);s.store_scalar(968, (2.0 * (((p[2] / 2.0) - 1.0)).max(0.0)));s.store_scalar(971, 0.0);s.store_scalar(970, p[2]);}
        if (((((!s.b[1374]) && s.b[1375]) && s.b[1376]) && (!s.b[1377])) && (!s.b[1378])) {s.store_scalar(969, 0.0);s.store_scalar(968, p[2]);s.store_scalar(971, 2.0);s.store_scalar(970, (2.0 * (((p[2] / 2.0) - 1.0)).max(0.0)));}
        s.b[1379] = (0.0 == 1.0);s.store_scalar(1379, if s.b[1379] { 1.0 } else { 0.0 });s.b[1380] = (s.v[970] == 0.0);s.store_scalar(1380, if s.b[1380] { 1.0 } else { 0.0 });
        if (((((!s.b[1374]) && s.b[1375]) && s.b[1376]) && s.b[1379]) && s.b[1380]) {s.store_scalar(972, 0.0);}
        if (((((!s.b[1374]) && s.b[1375]) && s.b[1376]) && s.b[1379]) && (!s.b[1380])) {s.store_primal_div_from_scalar_scaled_input(972, (p[438] * s.v[449]), 970, s.v[183]);}
        s.b[1381] = (s.v[968] == 0.0);s.store_scalar(1381, if s.b[1381] { 1.0 } else { 0.0 });
        if (((((!s.b[1374]) && s.b[1375]) && s.b[1376]) && (!s.b[1379])) && s.b[1381]) {s.store_scalar(972, 0.0);}
        if (((((!s.b[1374]) && s.b[1375]) && s.b[1376]) && (!s.b[1379])) && (!s.b[1381])) {s.store_primal_div_from_scalar_scaled_input(972, (p[438] * s.v[449]), 968, s.v[183]);}
        s.b[1382] = (p[8] == 0.0);s.store_scalar(1382, if s.b[1382] { 1.0 } else { 0.0 });s.b[1383] = (p[8] == 1.0);s.store_scalar(1383, if s.b[1383] { 1.0 } else { 0.0 });s.b[1384] = (p[8] == 2.0);s.store_scalar(1384, if s.b[1384] { 1.0 } else { 0.0 });s.b[1385] = (p[8] == 3.0);s.store_scalar(1385, if s.b[1385] { 1.0 } else { 0.0 });s.b[1386] = (p[8] == 4.0);s.store_scalar(1386, if s.b[1386] { 1.0 } else { 0.0 });s.b[1387] = (p[8] == 5.0);s.store_scalar(1387, if s.b[1387] { 1.0 } else { 0.0 });s.b[1388] = (p[8] == 6.0);s.store_scalar(1388, if s.b[1388] { 1.0 } else { 0.0 });s.b[1389] = (p[8] == 7.0);s.store_scalar(1389, if s.b[1389] { 1.0 } else { 0.0 });s.b[1390] = (p[8] == 8.0);s.store_scalar(1390, if s.b[1390] { 1.0 } else { 0.0 });s.b[1391] = (p[8] == 9.0);s.store_scalar(1391, if s.b[1391] { 1.0 } else { 0.0 });s.b[1392] = (p[8] == 10.0);s.store_scalar(1392, if s.b[1392] { 1.0 } else { 0.0 });s.b[1393] = (0.0 == 1.0);s.store_scalar(1393, if s.b[1393] { 1.0 } else { 0.0 });s.b[1394] = (1.0 == 1.0);s.store_scalar(1394, if s.b[1394] { 1.0 } else { 0.0 });s.b[1395] = (((p[9] == 1.0) || (p[9] == 2.0)) || (p[9] == 5.0));s.store_scalar(1395, if s.b[1395] { 1.0 } else { 0.0 });s.b[1396] = (((p[9] == 3.0) || (p[9] == 4.0)) || (p[9] == 6.0));s.store_scalar(1396, if s.b[1396] { 1.0 } else { 0.0 });s.b[1397] = (s.v[971] == 0.0);s.store_scalar(1397, if s.b[1397] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && s.b[1393]) && s.b[1394]) && s.b[1395]) && s.b[1397]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && s.b[1393]) && s.b[1394]) && s.b[1395]) && (!s.b[1397])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 971, s.v[183]);}
        s.b[1399] = ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));s.store_scalar(1399, if s.b[1399] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && s.b[1393]) && s.b[1394]) && (s.b[1396] && (!s.b[1395]))) && s.b[1399]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && s.b[1393]) && s.b[1394]) && (s.b[1396] && (!s.b[1395]))) && (!s.b[1399])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 971, (3.0 * (s.v[449] + s.v[451])));}
        if ((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && s.b[1393]) && s.b[1394]) && (!(s.b[1395] || s.b[1396]))) {s.store_scalar(973, 0.0);}
        s.b[1400] = (((p[9] == 1.0) || (p[9] == 3.0)) || (p[9] == 7.0));s.store_scalar(1400, if s.b[1400] { 1.0 } else { 0.0 });s.b[1401] = (((p[9] == 2.0) || (p[9] == 4.0)) || (p[9] == 8.0));s.store_scalar(1401, if s.b[1401] { 1.0 } else { 0.0 });s.b[1402] = (s.v[971] == 0.0);s.store_scalar(1402, if s.b[1402] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && s.b[1393]) && (!s.b[1394])) && s.b[1400]) && s.b[1402]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && s.b[1393]) && (!s.b[1394])) && s.b[1400]) && (!s.b[1402])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 971, s.v[183]);}
        s.b[1404] = ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));s.store_scalar(1404, if s.b[1404] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && s.b[1393]) && (!s.b[1394])) && (s.b[1401] && (!s.b[1400]))) && s.b[1404]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && s.b[1393]) && (!s.b[1394])) && (s.b[1401] && (!s.b[1400]))) && (!s.b[1404])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 971, (3.0 * (s.v[449] + s.v[451])));}
        if ((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && s.b[1393]) && (!s.b[1394])) && (!(s.b[1400] || s.b[1401]))) {s.store_scalar(973, 0.0);}
        s.b[1405] = (0.0 == 1.0);s.store_scalar(1405, if s.b[1405] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1406] = (((p[9] == 1.0) || (p[9] == 2.0)) || (p[9] == 5.0));s.store_scalar(1406, if s.b[1406] { 1.0 } else { 0.0 });s.b[1407] = (((p[9] == 3.0) || (p[9] == 4.0)) || (p[9] == 6.0));s.store_scalar(1407, if s.b[1407] { 1.0 } else { 0.0 });s.b[1408] = (s.v[969] == 0.0);s.store_scalar(1408, if s.b[1408] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && (!s.b[1393])) && s.b[1405]) && s.b[1406]) && s.b[1408]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && (!s.b[1393])) && s.b[1405]) && s.b[1406]) && (!s.b[1408])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 969, s.v[183]);}
        s.b[1410] = ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));s.store_scalar(1410, if s.b[1410] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && (!s.b[1393])) && s.b[1405]) && (s.b[1407] && (!s.b[1406]))) && s.b[1410]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && (!s.b[1393])) && s.b[1405]) && (s.b[1407] && (!s.b[1406]))) && (!s.b[1410])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 969, (3.0 * (s.v[449] + s.v[451])));}
        if ((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && (!s.b[1393])) && s.b[1405]) && (!(s.b[1406] || s.b[1407]))) {s.store_scalar(973, 0.0);}
        s.b[1411] = (((p[9] == 1.0) || (p[9] == 3.0)) || (p[9] == 7.0));s.store_scalar(1411, if s.b[1411] { 1.0 } else { 0.0 });s.b[1412] = (((p[9] == 2.0) || (p[9] == 4.0)) || (p[9] == 8.0));s.store_scalar(1412, if s.b[1412] { 1.0 } else { 0.0 });s.b[1413] = (s.v[969] == 0.0);s.store_scalar(1413, if s.b[1413] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && (!s.b[1393])) && (!s.b[1405])) && s.b[1411]) && s.b[1413]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && (!s.b[1393])) && (!s.b[1405])) && s.b[1411]) && (!s.b[1413])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 969, s.v[183]);}
        s.b[1415] = ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));s.store_scalar(1415, if s.b[1415] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && (!s.b[1393])) && (!s.b[1405])) && (s.b[1412] && (!s.b[1411]))) && s.b[1415]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && (!s.b[1393])) && (!s.b[1405])) && (s.b[1412] && (!s.b[1411]))) && (!s.b[1415])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 969, (3.0 * (s.v[449] + s.v[451])));}
        if ((((((!s.b[1374]) && s.b[1375]) && s.b[1382]) && (!s.b[1393])) && (!s.b[1405])) && (!(s.b[1411] || s.b[1412]))) {s.store_scalar(973, 0.0);}
        s.b[1416] = (0.0 == 1.0);s.store_scalar(1416, if s.b[1416] { 1.0 } else { 0.0 });s.b[1417] = (1.0 == 1.0);s.store_scalar(1417, if s.b[1417] { 1.0 } else { 0.0 });s.b[1418] = (((p[9] == 1.0) || (p[9] == 2.0)) || (p[9] == 5.0));s.store_scalar(1418, if s.b[1418] { 1.0 } else { 0.0 });s.b[1419] = (((p[9] == 3.0) || (p[9] == 4.0)) || (p[9] == 6.0));s.store_scalar(1419, if s.b[1419] { 1.0 } else { 0.0 });s.b[1420] = (s.v[971] == 0.0);s.store_scalar(1420, if s.b[1420] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && s.b[1416]) && s.b[1417]) && s.b[1418]) && s.b[1420]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && s.b[1416]) && s.b[1417]) && s.b[1418]) && (!s.b[1420])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 971, s.v[183]);}
        s.b[1422] = ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));s.store_scalar(1422, if s.b[1422] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && s.b[1416]) && s.b[1417]) && (s.b[1419] && (!s.b[1418]))) && s.b[1422]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && s.b[1416]) && s.b[1417]) && (s.b[1419] && (!s.b[1418]))) && (!s.b[1422])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 971, (3.0 * (s.v[449] + s.v[451])));}
        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && s.b[1416]) && s.b[1417]) && (!(s.b[1418] || s.b[1419]))) {s.store_scalar(973, 0.0);}
        s.b[1423] = (((p[9] == 1.0) || (p[9] == 3.0)) || (p[9] == 7.0));s.store_scalar(1423, if s.b[1423] { 1.0 } else { 0.0 });s.b[1424] = (((p[9] == 2.0) || (p[9] == 4.0)) || (p[9] == 8.0));s.store_scalar(1424, if s.b[1424] { 1.0 } else { 0.0 });s.b[1425] = (s.v[971] == 0.0);s.store_scalar(1425, if s.b[1425] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && s.b[1416]) && (!s.b[1417])) && s.b[1423]) && s.b[1425]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && s.b[1416]) && (!s.b[1417])) && s.b[1423]) && (!s.b[1425])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 971, s.v[183]);}
        s.b[1427] = ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));s.store_scalar(1427, if s.b[1427] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && s.b[1416]) && (!s.b[1417])) && (s.b[1424] && (!s.b[1423]))) && s.b[1427]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && s.b[1416]) && (!s.b[1417])) && (s.b[1424] && (!s.b[1423]))) && (!s.b[1427])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 971, (3.0 * (s.v[449] + s.v[451])));}
        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && s.b[1416]) && (!s.b[1417])) && (!(s.b[1423] || s.b[1424]))) {s.store_scalar(973, 0.0);}
        s.b[1428] = (0.0 == 1.0);s.store_scalar(1428, if s.b[1428] { 1.0 } else { 0.0 });s.b[1429] = (((p[9] == 1.0) || (p[9] == 2.0)) || (p[9] == 5.0));s.store_scalar(1429, if s.b[1429] { 1.0 } else { 0.0 });s.b[1430] = (((p[9] == 3.0) || (p[9] == 4.0)) || (p[9] == 6.0));s.store_scalar(1430, if s.b[1430] { 1.0 } else { 0.0 });s.b[1431] = (s.v[969] == 0.0);s.store_scalar(1431, if s.b[1431] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && (!s.b[1416])) && s.b[1428]) && s.b[1429]) && s.b[1431]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && (!s.b[1416])) && s.b[1428]) && s.b[1429]) && (!s.b[1431])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 969, s.v[183]);}
        s.b[1433] = ((s.v[969] == 0.0) || (s.v[449] == 0.0));s.store_scalar(1433, if s.b[1433] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && (!s.b[1416])) && s.b[1428]) && (s.b[1430] && (!s.b[1429]))) && s.b[1433]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && (!s.b[1416])) && s.b[1428]) && (s.b[1430] && (!s.b[1429]))) && (!s.b[1433])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 969, (6.0 * s.v[449]));}
        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && (!s.b[1416])) && s.b[1428]) && (!(s.b[1429] || s.b[1430]))) {s.store_scalar(973, 0.0);}
        s.b[1434] = (((p[9] == 1.0) || (p[9] == 3.0)) || (p[9] == 7.0));s.store_scalar(1434, if s.b[1434] { 1.0 } else { 0.0 });s.b[1435] = (((p[9] == 2.0) || (p[9] == 4.0)) || (p[9] == 8.0));s.store_scalar(1435, if s.b[1435] { 1.0 } else { 0.0 });s.b[1436] = (s.v[969] == 0.0);s.store_scalar(1436, if s.b[1436] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && (!s.b[1416])) && (!s.b[1428])) && s.b[1434]) && s.b[1436]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && (!s.b[1416])) && (!s.b[1428])) && s.b[1434]) && (!s.b[1436])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 969, s.v[183]);}
        s.b[1438] = ((s.v[969] == 0.0) || (s.v[449] == 0.0));s.store_scalar(1438, if s.b[1438] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && (!s.b[1416])) && (!s.b[1428])) && (s.b[1435] && (!s.b[1434]))) && s.b[1438]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && (!s.b[1416])) && (!s.b[1428])) && (s.b[1435] && (!s.b[1434]))) && (!s.b[1438])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 969, (6.0 * s.v[449]));}
        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1383] && (!s.b[1382]))) && (!s.b[1416])) && (!s.b[1428])) && (!(s.b[1434] || s.b[1435]))) {s.store_scalar(973, 0.0);}
        s.b[1439] = (0.0 == 1.0);s.store_scalar(1439, if s.b[1439] { 1.0 } else { 0.0 });s.b[1440] = (1.0 == 1.0);s.store_scalar(1440, if s.b[1440] { 1.0 } else { 0.0 });s.b[1441] = (((p[9] == 1.0) || (p[9] == 2.0)) || (p[9] == 5.0));s.store_scalar(1441, if s.b[1441] { 1.0 } else { 0.0 });s.b[1442] = (((p[9] == 3.0) || (p[9] == 4.0)) || (p[9] == 6.0));s.store_scalar(1442, if s.b[1442] { 1.0 } else { 0.0 });s.b[1443] = (s.v[971] == 0.0);s.store_scalar(1443, if s.b[1443] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && s.b[1439]) && s.b[1440]) && s.b[1441]) && s.b[1443]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && s.b[1439]) && s.b[1440]) && s.b[1441]) && (!s.b[1443])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 971, s.v[183]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_13(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1445] = ((s.v[971] == 0.0) || (s.v[449] == 0.0));s.store_scalar(1445, if s.b[1445] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && s.b[1439]) && s.b[1440]) && (s.b[1442] && (!s.b[1441]))) && s.b[1445]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && s.b[1439]) && s.b[1440]) && (s.b[1442] && (!s.b[1441]))) && (!s.b[1445])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 971, (6.0 * s.v[449]));}
        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && s.b[1439]) && s.b[1440]) && (!(s.b[1441] || s.b[1442]))) {s.store_scalar(973, 0.0);}
        s.b[1446] = (((p[9] == 1.0) || (p[9] == 3.0)) || (p[9] == 7.0));s.store_scalar(1446, if s.b[1446] { 1.0 } else { 0.0 });s.b[1447] = (((p[9] == 2.0) || (p[9] == 4.0)) || (p[9] == 8.0));s.store_scalar(1447, if s.b[1447] { 1.0 } else { 0.0 });s.b[1448] = (s.v[971] == 0.0);s.store_scalar(1448, if s.b[1448] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && s.b[1439]) && (!s.b[1440])) && s.b[1446]) && s.b[1448]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && s.b[1439]) && (!s.b[1440])) && s.b[1446]) && (!s.b[1448])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 971, s.v[183]);}
        s.b[1450] = ((s.v[971] == 0.0) || (s.v[449] == 0.0));s.store_scalar(1450, if s.b[1450] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && s.b[1439]) && (!s.b[1440])) && (s.b[1447] && (!s.b[1446]))) && s.b[1450]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && s.b[1439]) && (!s.b[1440])) && (s.b[1447] && (!s.b[1446]))) && (!s.b[1450])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 971, (6.0 * s.v[449]));}
        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && s.b[1439]) && (!s.b[1440])) && (!(s.b[1446] || s.b[1447]))) {s.store_scalar(973, 0.0);}
        s.b[1451] = (0.0 == 1.0);s.store_scalar(1451, if s.b[1451] { 1.0 } else { 0.0 });s.b[1452] = (((p[9] == 1.0) || (p[9] == 2.0)) || (p[9] == 5.0));s.store_scalar(1452, if s.b[1452] { 1.0 } else { 0.0 });s.b[1453] = (((p[9] == 3.0) || (p[9] == 4.0)) || (p[9] == 6.0));s.store_scalar(1453, if s.b[1453] { 1.0 } else { 0.0 });s.b[1454] = (s.v[969] == 0.0);s.store_scalar(1454, if s.b[1454] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && (!s.b[1439])) && s.b[1451]) && s.b[1452]) && s.b[1454]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && (!s.b[1439])) && s.b[1451]) && s.b[1452]) && (!s.b[1454])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 969, s.v[183]);}
        s.b[1456] = ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));s.store_scalar(1456, if s.b[1456] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && (!s.b[1439])) && s.b[1451]) && (s.b[1453] && (!s.b[1452]))) && s.b[1456]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && (!s.b[1439])) && s.b[1451]) && (s.b[1453] && (!s.b[1452]))) && (!s.b[1456])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 969, (3.0 * (s.v[449] + s.v[451])));}
        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && (!s.b[1439])) && s.b[1451]) && (!(s.b[1452] || s.b[1453]))) {s.store_scalar(973, 0.0);}
        s.b[1457] = (((p[9] == 1.0) || (p[9] == 3.0)) || (p[9] == 7.0));s.store_scalar(1457, if s.b[1457] { 1.0 } else { 0.0 });s.b[1458] = (((p[9] == 2.0) || (p[9] == 4.0)) || (p[9] == 8.0));s.store_scalar(1458, if s.b[1458] { 1.0 } else { 0.0 });s.b[1459] = (s.v[969] == 0.0);s.store_scalar(1459, if s.b[1459] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && (!s.b[1439])) && (!s.b[1451])) && s.b[1457]) && s.b[1459]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && (!s.b[1439])) && (!s.b[1451])) && s.b[1457]) && (!s.b[1459])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 969, s.v[183]);}
        s.b[1461] = ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));s.store_scalar(1461, if s.b[1461] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && (!s.b[1439])) && (!s.b[1451])) && (s.b[1458] && (!s.b[1457]))) && s.b[1461]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && (!s.b[1439])) && (!s.b[1451])) && (s.b[1458] && (!s.b[1457]))) && (!s.b[1461])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 969, (3.0 * (s.v[449] + s.v[451])));}
        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1384] && (!(s.b[1382] || s.b[1383])))) && (!s.b[1439])) && (!s.b[1451])) && (!(s.b[1457] || s.b[1458]))) {s.store_scalar(973, 0.0);}
        s.b[1462] = (0.0 == 1.0);s.store_scalar(1462, if s.b[1462] { 1.0 } else { 0.0 });s.b[1463] = (1.0 == 1.0);s.store_scalar(1463, if s.b[1463] { 1.0 } else { 0.0 });s.b[1464] = (((p[9] == 1.0) || (p[9] == 2.0)) || (p[9] == 5.0));s.store_scalar(1464, if s.b[1464] { 1.0 } else { 0.0 });s.b[1465] = (((p[9] == 3.0) || (p[9] == 4.0)) || (p[9] == 6.0));s.store_scalar(1465, if s.b[1465] { 1.0 } else { 0.0 });s.b[1466] = (s.v[971] == 0.0);s.store_scalar(1466, if s.b[1466] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && s.b[1462]) && s.b[1463]) && s.b[1464]) && s.b[1466]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && s.b[1462]) && s.b[1463]) && s.b[1464]) && (!s.b[1466])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 971, s.v[183]);}
        s.b[1468] = ((s.v[971] == 0.0) || (s.v[449] == 0.0));s.store_scalar(1468, if s.b[1468] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && s.b[1462]) && s.b[1463]) && (s.b[1465] && (!s.b[1464]))) && s.b[1468]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && s.b[1462]) && s.b[1463]) && (s.b[1465] && (!s.b[1464]))) && (!s.b[1468])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 971, (6.0 * s.v[449]));}
        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && s.b[1462]) && s.b[1463]) && (!(s.b[1464] || s.b[1465]))) {s.store_scalar(973, 0.0);}
        s.b[1469] = (((p[9] == 1.0) || (p[9] == 3.0)) || (p[9] == 7.0));s.store_scalar(1469, if s.b[1469] { 1.0 } else { 0.0 });s.b[1470] = (((p[9] == 2.0) || (p[9] == 4.0)) || (p[9] == 8.0));s.store_scalar(1470, if s.b[1470] { 1.0 } else { 0.0 });s.b[1471] = (s.v[971] == 0.0);s.store_scalar(1471, if s.b[1471] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && s.b[1462]) && (!s.b[1463])) && s.b[1469]) && s.b[1471]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && s.b[1462]) && (!s.b[1463])) && s.b[1469]) && (!s.b[1471])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 971, s.v[183]);}
        s.b[1473] = ((s.v[971] == 0.0) || (s.v[449] == 0.0));s.store_scalar(1473, if s.b[1473] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && s.b[1462]) && (!s.b[1463])) && (s.b[1470] && (!s.b[1469]))) && s.b[1473]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && s.b[1462]) && (!s.b[1463])) && (s.b[1470] && (!s.b[1469]))) && (!s.b[1473])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 971, (6.0 * s.v[449]));}
        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && s.b[1462]) && (!s.b[1463])) && (!(s.b[1469] || s.b[1470]))) {s.store_scalar(973, 0.0);}
        s.b[1474] = (0.0 == 1.0);s.store_scalar(1474, if s.b[1474] { 1.0 } else { 0.0 });s.b[1475] = (((p[9] == 1.0) || (p[9] == 2.0)) || (p[9] == 5.0));s.store_scalar(1475, if s.b[1475] { 1.0 } else { 0.0 });s.b[1476] = (((p[9] == 3.0) || (p[9] == 4.0)) || (p[9] == 6.0));s.store_scalar(1476, if s.b[1476] { 1.0 } else { 0.0 });s.b[1477] = (s.v[969] == 0.0);s.store_scalar(1477, if s.b[1477] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && (!s.b[1462])) && s.b[1474]) && s.b[1475]) && s.b[1477]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && (!s.b[1462])) && s.b[1474]) && s.b[1475]) && (!s.b[1477])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 969, s.v[183]);}
        s.b[1479] = ((s.v[969] == 0.0) || (s.v[449] == 0.0));s.store_scalar(1479, if s.b[1479] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && (!s.b[1462])) && s.b[1474]) && (s.b[1476] && (!s.b[1475]))) && s.b[1479]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && (!s.b[1462])) && s.b[1474]) && (s.b[1476] && (!s.b[1475]))) && (!s.b[1479])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 969, (6.0 * s.v[449]));}
        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && (!s.b[1462])) && s.b[1474]) && (!(s.b[1475] || s.b[1476]))) {s.store_scalar(973, 0.0);}
        s.b[1480] = (((p[9] == 1.0) || (p[9] == 3.0)) || (p[9] == 7.0));s.store_scalar(1480, if s.b[1480] { 1.0 } else { 0.0 });s.b[1481] = (((p[9] == 2.0) || (p[9] == 4.0)) || (p[9] == 8.0));s.store_scalar(1481, if s.b[1481] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1482] = (s.v[969] == 0.0);s.store_scalar(1482, if s.b[1482] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && (!s.b[1462])) && (!s.b[1474])) && s.b[1480]) && s.b[1482]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && (!s.b[1462])) && (!s.b[1474])) && s.b[1480]) && (!s.b[1482])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 969, s.v[183]);}
        s.b[1484] = ((s.v[969] == 0.0) || (s.v[449] == 0.0));s.store_scalar(1484, if s.b[1484] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && (!s.b[1462])) && (!s.b[1474])) && (s.b[1481] && (!s.b[1480]))) && s.b[1484]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && (!s.b[1462])) && (!s.b[1474])) && (s.b[1481] && (!s.b[1480]))) && (!s.b[1484])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 969, (6.0 * s.v[449]));}
        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1385] && (!((s.b[1382] || s.b[1383]) || s.b[1384])))) && (!s.b[1462])) && (!s.b[1474])) && (!(s.b[1480] || s.b[1481]))) {s.store_scalar(973, 0.0);}
        s.b[1485] = (0.0 == 1.0);s.store_scalar(1485, if s.b[1485] { 1.0 } else { 0.0 });s.b[1486] = (1.0 == 1.0);s.store_scalar(1486, if s.b[1486] { 1.0 } else { 0.0 });s.b[1487] = (((p[9] == 1.0) || (p[9] == 2.0)) || (p[9] == 5.0));s.store_scalar(1487, if s.b[1487] { 1.0 } else { 0.0 });s.b[1488] = (((p[9] == 3.0) || (p[9] == 4.0)) || (p[9] == 6.0));s.store_scalar(1488, if s.b[1488] { 1.0 } else { 0.0 });s.b[1489] = (s.v[971] == 0.0);s.store_scalar(1489, if s.b[1489] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1386] && (!(((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385])))) && s.b[1485]) && s.b[1486]) && s.b[1487]) && s.b[1489]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1386] && (!(((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385])))) && s.b[1485]) && s.b[1486]) && s.b[1487]) && (!s.b[1489])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 971, s.v[183]);}
        s.b[1491] = ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));s.store_scalar(1491, if s.b[1491] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1386] && (!(((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385])))) && s.b[1485]) && s.b[1486]) && (s.b[1488] && (!s.b[1487]))) && s.b[1491]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1386] && (!(((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385])))) && s.b[1485]) && s.b[1486]) && (s.b[1488] && (!s.b[1487]))) && (!s.b[1491])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 971, (3.0 * (s.v[449] + s.v[451])));}
        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1386] && (!(((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385])))) && s.b[1485]) && s.b[1486]) && (!(s.b[1487] || s.b[1488]))) {s.store_scalar(973, 0.0);}
        s.b[1492] = (((p[9] == 1.0) || (p[9] == 3.0)) || (p[9] == 7.0));s.store_scalar(1492, if s.b[1492] { 1.0 } else { 0.0 });s.b[1493] = (((p[9] == 2.0) || (p[9] == 4.0)) || (p[9] == 8.0));s.store_scalar(1493, if s.b[1493] { 1.0 } else { 0.0 });s.b[1494] = (s.v[971] == 0.0);s.store_scalar(1494, if s.b[1494] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1386] && (!(((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385])))) && s.b[1485]) && (!s.b[1486])) && s.b[1492]) && s.b[1494]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1386] && (!(((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385])))) && s.b[1485]) && (!s.b[1486])) && s.b[1492]) && (!s.b[1494])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 971, s.v[183]);}
        s.b[1496] = ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));s.store_scalar(1496, if s.b[1496] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1386] && (!(((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385])))) && s.b[1485]) && (!s.b[1486])) && (s.b[1493] && (!s.b[1492]))) && s.b[1496]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1386] && (!(((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385])))) && s.b[1485]) && (!s.b[1486])) && (s.b[1493] && (!s.b[1492]))) && (!s.b[1496])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 971, (3.0 * (s.v[449] + s.v[451])));}
        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1386] && (!(((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385])))) && s.b[1485]) && (!s.b[1486])) && (!(s.b[1492] || s.b[1493]))) {s.store_scalar(973, 0.0);}
        if ((((!s.b[1374]) && s.b[1375]) && (s.b[1386] && (!(((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385])))) && (!s.b[1485])) {s.store_scalar(973, ((p[438] * s.v[450]) / s.v[183]));}
        s.b[1497] = (0.0 == 1.0);s.store_scalar(1497, if s.b[1497] { 1.0 } else { 0.0 });s.b[1498] = (1.0 == 1.0);s.store_scalar(1498, if s.b[1498] { 1.0 } else { 0.0 });s.b[1499] = (((p[9] == 1.0) || (p[9] == 2.0)) || (p[9] == 5.0));s.store_scalar(1499, if s.b[1499] { 1.0 } else { 0.0 });s.b[1500] = (((p[9] == 3.0) || (p[9] == 4.0)) || (p[9] == 6.0));s.store_scalar(1500, if s.b[1500] { 1.0 } else { 0.0 });s.b[1501] = (s.v[971] == 0.0);s.store_scalar(1501, if s.b[1501] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1387] && (!((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386])))) && s.b[1497]) && s.b[1498]) && s.b[1499]) && s.b[1501]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1387] && (!((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386])))) && s.b[1497]) && s.b[1498]) && s.b[1499]) && (!s.b[1501])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 971, s.v[183]);}
        s.b[1503] = ((s.v[971] == 0.0) || (s.v[449] == 0.0));s.store_scalar(1503, if s.b[1503] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1387] && (!((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386])))) && s.b[1497]) && s.b[1498]) && (s.b[1500] && (!s.b[1499]))) && s.b[1503]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1387] && (!((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386])))) && s.b[1497]) && s.b[1498]) && (s.b[1500] && (!s.b[1499]))) && (!s.b[1503])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 971, (6.0 * s.v[449]));}
        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1387] && (!((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386])))) && s.b[1497]) && s.b[1498]) && (!(s.b[1499] || s.b[1500]))) {s.store_scalar(973, 0.0);}
        s.b[1504] = (((p[9] == 1.0) || (p[9] == 3.0)) || (p[9] == 7.0));s.store_scalar(1504, if s.b[1504] { 1.0 } else { 0.0 });s.b[1505] = (((p[9] == 2.0) || (p[9] == 4.0)) || (p[9] == 8.0));s.store_scalar(1505, if s.b[1505] { 1.0 } else { 0.0 });s.b[1506] = (s.v[971] == 0.0);s.store_scalar(1506, if s.b[1506] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1387] && (!((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386])))) && s.b[1497]) && (!s.b[1498])) && s.b[1504]) && s.b[1506]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1387] && (!((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386])))) && s.b[1497]) && (!s.b[1498])) && s.b[1504]) && (!s.b[1506])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 971, s.v[183]);}
        s.b[1508] = ((s.v[971] == 0.0) || (s.v[449] == 0.0));s.store_scalar(1508, if s.b[1508] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1387] && (!((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386])))) && s.b[1497]) && (!s.b[1498])) && (s.b[1505] && (!s.b[1504]))) && s.b[1508]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1387] && (!((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386])))) && s.b[1497]) && (!s.b[1498])) && (s.b[1505] && (!s.b[1504]))) && (!s.b[1508])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 971, (6.0 * s.v[449]));}
        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1387] && (!((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386])))) && s.b[1497]) && (!s.b[1498])) && (!(s.b[1504] || s.b[1505]))) {s.store_scalar(973, 0.0);}
        s.b[1509] = (s.v[969] == 0.0);s.store_scalar(1509, if s.b[1509] { 1.0 } else { 0.0 });
        if (((((!s.b[1374]) && s.b[1375]) && (s.b[1387] && (!((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386])))) && (!s.b[1497])) && s.b[1509]) {s.store_scalar(973, 0.0);}
        if (((((!s.b[1374]) && s.b[1375]) && (s.b[1387] && (!((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386])))) && (!s.b[1497])) && (!s.b[1509])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[450]), 969, s.v[183]);}
        s.b[1510] = (0.0 == 1.0);s.store_scalar(1510, if s.b[1510] { 1.0 } else { 0.0 });
        if ((((!s.b[1374]) && s.b[1375]) && (s.b[1388] && (!(((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387])))) && s.b[1510]) {s.store_scalar(973, ((p[438] * s.v[450]) / s.v[183]));}
        s.b[1511] = (0.0 == 1.0);s.store_scalar(1511, if s.b[1511] { 1.0 } else { 0.0 });s.b[1512] = (((p[9] == 1.0) || (p[9] == 2.0)) || (p[9] == 5.0));s.store_scalar(1512, if s.b[1512] { 1.0 } else { 0.0 });s.b[1513] = (((p[9] == 3.0) || (p[9] == 4.0)) || (p[9] == 6.0));s.store_scalar(1513, if s.b[1513] { 1.0 } else { 0.0 });s.b[1514] = (s.v[969] == 0.0);s.store_scalar(1514, if s.b[1514] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1388] && (!(((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387])))) && (!s.b[1510])) && s.b[1511]) && s.b[1512]) && s.b[1514]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1388] && (!(((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387])))) && (!s.b[1510])) && s.b[1511]) && s.b[1512]) && (!s.b[1514])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 969, s.v[183]);}
        s.b[1516] = ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));s.store_scalar(1516, if s.b[1516] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1388] && (!(((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387])))) && (!s.b[1510])) && s.b[1511]) && (s.b[1513] && (!s.b[1512]))) && s.b[1516]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1388] && (!(((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387])))) && (!s.b[1510])) && s.b[1511]) && (s.b[1513] && (!s.b[1512]))) && (!s.b[1516])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 969, (3.0 * (s.v[449] + s.v[451])));}
        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1388] && (!(((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387])))) && (!s.b[1510])) && s.b[1511]) && (!(s.b[1512] || s.b[1513]))) {s.store_scalar(973, 0.0);}
        s.b[1517] = (((p[9] == 1.0) || (p[9] == 3.0)) || (p[9] == 7.0));s.store_scalar(1517, if s.b[1517] { 1.0 } else { 0.0 });s.b[1518] = (((p[9] == 2.0) || (p[9] == 4.0)) || (p[9] == 8.0));s.store_scalar(1518, if s.b[1518] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1519] = (s.v[969] == 0.0);s.store_scalar(1519, if s.b[1519] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1388] && (!(((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387])))) && (!s.b[1510])) && (!s.b[1511])) && s.b[1517]) && s.b[1519]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1388] && (!(((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387])))) && (!s.b[1510])) && (!s.b[1511])) && s.b[1517]) && (!s.b[1519])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 969, s.v[183]);}
        s.b[1521] = ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0));s.store_scalar(1521, if s.b[1521] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1388] && (!(((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387])))) && (!s.b[1510])) && (!s.b[1511])) && (s.b[1518] && (!s.b[1517]))) && s.b[1521]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1388] && (!(((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387])))) && (!s.b[1510])) && (!s.b[1511])) && (s.b[1518] && (!s.b[1517]))) && (!s.b[1521])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 969, (3.0 * (s.v[449] + s.v[451])));}
        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1388] && (!(((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387])))) && (!s.b[1510])) && (!s.b[1511])) && (!(s.b[1517] || s.b[1518]))) {s.store_scalar(973, 0.0);}
        s.b[1522] = (0.0 == 1.0);s.store_scalar(1522, if s.b[1522] { 1.0 } else { 0.0 });s.b[1523] = (s.v[971] == 0.0);s.store_scalar(1523, if s.b[1523] { 1.0 } else { 0.0 });
        if (((((!s.b[1374]) && s.b[1375]) && (s.b[1389] && (!((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388])))) && s.b[1522]) && s.b[1523]) {s.store_scalar(973, 0.0);}
        if (((((!s.b[1374]) && s.b[1375]) && (s.b[1389] && (!((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388])))) && s.b[1522]) && (!s.b[1523])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[450]), 971, s.v[183]);}
        s.b[1524] = (0.0 == 1.0);s.store_scalar(1524, if s.b[1524] { 1.0 } else { 0.0 });s.b[1525] = (((p[9] == 1.0) || (p[9] == 2.0)) || (p[9] == 5.0));s.store_scalar(1525, if s.b[1525] { 1.0 } else { 0.0 });s.b[1526] = (((p[9] == 3.0) || (p[9] == 4.0)) || (p[9] == 6.0));s.store_scalar(1526, if s.b[1526] { 1.0 } else { 0.0 });s.b[1527] = (s.v[969] == 0.0);s.store_scalar(1527, if s.b[1527] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1389] && (!((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388])))) && (!s.b[1522])) && s.b[1524]) && s.b[1525]) && s.b[1527]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1389] && (!((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388])))) && (!s.b[1522])) && s.b[1524]) && s.b[1525]) && (!s.b[1527])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 969, s.v[183]);}
        s.b[1529] = ((s.v[969] == 0.0) || (s.v[449] == 0.0));s.store_scalar(1529, if s.b[1529] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1389] && (!((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388])))) && (!s.b[1522])) && s.b[1524]) && (s.b[1526] && (!s.b[1525]))) && s.b[1529]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1389] && (!((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388])))) && (!s.b[1522])) && s.b[1524]) && (s.b[1526] && (!s.b[1525]))) && (!s.b[1529])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 969, (6.0 * s.v[449]));}
        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1389] && (!((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388])))) && (!s.b[1522])) && s.b[1524]) && (!(s.b[1525] || s.b[1526]))) {s.store_scalar(973, 0.0);}
        s.b[1530] = (((p[9] == 1.0) || (p[9] == 3.0)) || (p[9] == 7.0));s.store_scalar(1530, if s.b[1530] { 1.0 } else { 0.0 });s.b[1531] = (((p[9] == 2.0) || (p[9] == 4.0)) || (p[9] == 8.0));s.store_scalar(1531, if s.b[1531] { 1.0 } else { 0.0 });s.b[1532] = (s.v[969] == 0.0);s.store_scalar(1532, if s.b[1532] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1389] && (!((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388])))) && (!s.b[1522])) && (!s.b[1524])) && s.b[1530]) && s.b[1532]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1389] && (!((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388])))) && (!s.b[1522])) && (!s.b[1524])) && s.b[1530]) && (!s.b[1532])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[449]), 969, s.v[183]);}
        s.b[1534] = ((s.v[969] == 0.0) || (s.v[449] == 0.0));s.store_scalar(1534, if s.b[1534] { 1.0 } else { 0.0 });
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1389] && (!((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388])))) && (!s.b[1522])) && (!s.b[1524])) && (s.b[1531] && (!s.b[1530]))) && s.b[1534]) {s.store_scalar(973, 0.0);}
        if (((((((!s.b[1374]) && s.b[1375]) && (s.b[1389] && (!((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388])))) && (!s.b[1522])) && (!s.b[1524])) && (s.b[1531] && (!s.b[1530]))) && (!s.b[1534])) {s.store_primal_div_from_scalar_scaled_input(973, (p[438] * s.v[183]), 969, (6.0 * s.v[449]));}
        if ((((((!s.b[1374]) && s.b[1375]) && (s.b[1389] && (!((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388])))) && (!s.b[1522])) && (!s.b[1524])) && (!(s.b[1530] || s.b[1531]))) {s.store_scalar(973, 0.0);}
        if (((!s.b[1374]) && s.b[1375]) && (s.b[1390] && (!(((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388]) || s.b[1389])))) {s.store_scalar(973, ((p[438] * s.v[450]) / s.v[183]));}
        s.b[1535] = (0.0 == 1.0);s.store_scalar(1535, if s.b[1535] { 1.0 } else { 0.0 });
        if ((((!s.b[1374]) && s.b[1375]) && (s.b[1391] && (!((((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388]) || s.b[1389]) || s.b[1390])))) && s.b[1535]) {s.store_scalar(973, (((0.5 * p[438]) * s.v[449]) / s.v[183]));}
        s.b[1536] = (p[2] == 2.0);s.store_scalar(1536, if s.b[1536] { 1.0 } else { 0.0 });
        if (((((!s.b[1374]) && s.b[1375]) && (s.b[1391] && (!((((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388]) || s.b[1389]) || s.b[1390])))) && s.b[1535]) && s.b[1536]) {s.store_scalar(972, 0.0);}
        if (((((!s.b[1374]) && s.b[1375]) && (s.b[1391] && (!((((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388]) || s.b[1389]) || s.b[1390])))) && s.b[1535]) && (!s.b[1536])) {s.store_scalar(972, ((p[438] * s.v[449]) / (s.v[183] * (p[2] - 2.0))));}
        if ((((!s.b[1374]) && s.b[1375]) && (s.b[1391] && (!((((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388]) || s.b[1389]) || s.b[1390])))) && (!s.b[1535])) {s.store_scalar(973, 0.0);s.store_scalar(972, ((p[438] * s.v[449]) / (s.v[183] * p[2])));}
        s.b[1537] = (0.0 == 1.0);s.store_scalar(1537, if s.b[1537] { 1.0 } else { 0.0 });
        if ((((!s.b[1374]) && s.b[1375]) && (s.b[1392] && (!(((((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388]) || s.b[1389]) || s.b[1390]) || s.b[1391])))) && s.b[1537]) {s.store_scalar(973, 0.0);s.store_scalar(972, ((p[438] * s.v[449]) / (s.v[183] * p[2])));}
        if ((((!s.b[1374]) && s.b[1375]) && (s.b[1392] && (!(((((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388]) || s.b[1389]) || s.b[1390]) || s.b[1391])))) && (!s.b[1537])) {s.store_scalar(973, (((0.5 * p[438]) * s.v[449]) / s.v[183]));}
        s.b[1538] = (p[2] == 2.0);s.store_scalar(1538, if s.b[1538] { 1.0 } else { 0.0 });
        if (((((!s.b[1374]) && s.b[1375]) && (s.b[1392] && (!(((((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388]) || s.b[1389]) || s.b[1390]) || s.b[1391])))) && (!s.b[1537])) && s.b[1538]) {s.store_scalar(972, 0.0);}
        if (((((!s.b[1374]) && s.b[1375]) && (s.b[1392] && (!(((((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388]) || s.b[1389]) || s.b[1390]) || s.b[1391])))) && (!s.b[1537])) && (!s.b[1538])) {s.store_scalar(972, ((p[438] * s.v[449]) / (s.v[183] * (p[2] - 2.0))));}
        if (((!s.b[1374]) && s.b[1375]) && (!((((((((((s.b[1382] || s.b[1383]) || s.b[1384]) || s.b[1385]) || s.b[1386]) || s.b[1387]) || s.b[1388]) || s.b[1389]) || s.b[1390]) || s.b[1391]) || s.b[1392]))) {s.store_scalar(972, 0.0);}
        s.b[1539] = (s.v[972] <= 0.0);s.store_scalar(1539, if s.b[1539] { 1.0 } else { 0.0 });
        if (((!s.b[1374]) && s.b[1375]) && s.b[1539]) {s.copy_ad(453, 973);}
        s.b[1540] = (s.v[973] <= 0.0);s.store_scalar(1540, if s.b[1540] { 1.0 } else { 0.0 });
        if ((((!s.b[1374]) && s.b[1375]) && (!s.b[1539])) && s.b[1540]) {s.copy_ad(453, 972);}
        if ((((!s.b[1374]) && s.b[1375]) && (!s.b[1539])) && (!s.b[1540])) {s.store_primal_div_scaled_product_add_scaled_denominator_indices(453, 972, 973, 1.0, 972, 1.0, 973, 1.0, 1.0);}
        if ((!s.b[1374]) && (!s.b[1375])) {s.store_scalar(453, 0.0);}
        s.b[1542] = (p[33] == 0.0);s.store_scalar(1542, if s.b[1542] { 1.0 } else { 0.0 });s.b[1543] = (s.v[452] < p[1347]);s.store_scalar(1543, if s.b[1543] { 1.0 } else { 0.0 });
        if (s.b[1542] && s.b[1543]) {s.store_scalar(452, 0.0);}
        s.b[1544] = (s.v[453] < p[1347]);s.store_scalar(1544, if s.b[1544] { 1.0 } else { 0.0 });
        if (s.b[1542] && s.b[1544]) {s.store_scalar(453, 0.0);}
        s.b[1545] = (s.v[452] <= p[1347]);s.store_scalar(1545, if s.b[1545] { 1.0 } else { 0.0 });
        if ((!s.b[1542]) && s.b[1545]) {s.store_scalar(452, p[1347]);}
        s.b[1546] = (s.v[453] <= p[1347]);s.store_scalar(1546, if s.b[1546] { 1.0 } else { 0.0 });
        if ((!s.b[1542]) && s.b[1546]) {s.store_scalar(453, p[1347]);}
        s.b[1547] = (p[33] == 1.0);s.store_scalar(1547, if s.b[1547] { 1.0 } else { 0.0 });s.b[1548] = (s.v[773] <= 0.0);s.store_scalar(1548, if s.b[1548] { 1.0 } else { 0.0 });
        if (s.b[1547] && s.b[1548]) {s.store_scalar(773, 0.0);}
        s.b[1549] = (s.v[772] <= 0.0);s.store_scalar(1549, if s.b[1549] { 1.0 } else { 0.0 });
        if (s.b[1547] && s.b[1549]) {s.store_scalar(772, 0.0);}
        s.b[1550] = (s.v[775] <= 0.0);s.store_scalar(1550, if s.b[1550] { 1.0 } else { 0.0 });
        if (s.b[1547] && s.b[1550]) {s.store_scalar(775, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_16(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();s.b[1551] = (s.v[774] <= 0.0);s.store_scalar(1551, if s.b[1551] { 1.0 } else { 0.0 });
        if (s.b[1547] && s.b[1551]) {s.store_scalar(774, 0.0);}
        s.b[1552] = (s.v[777] <= 0.0);s.store_scalar(1552, if s.b[1552] { 1.0 } else { 0.0 });
        if ((!s.b[1547]) && s.b[1552]) {s.store_scalar(777, 0.0);}
        s.b[1553] = (s.v[776] <= 0.0);s.store_scalar(1553, if s.b[1553] { 1.0 } else { 0.0 });
        if ((!s.b[1547]) && s.b[1553]) {s.store_scalar(776, 0.0);}
        s.store_scalar(465, ((p[900] * (p[21] + ((s.v[189] / 3.0) / p[22]))) / ((p[22] * p[2]) * (s.v[261] - p[899]))));s.b[1554] = (s.v[465] > 0.0);s.store_scalar(1554, if s.b[1554] { 1.0 } else { 0.0 });
        if s.b[1554] {s.store_scalar(465, (1.0 / s.v[465]));}
        if (!s.b[1554]) {s.store_scalar(465, 1000.0);}
        s.store_scalar(167, (p[76] * p[76]));s.store_scale(168, 822, p[76]);s.store_square(169, 168);s.store_scaled_limited_exp_scaled_input(492, 826, ((((p[722] / p[76])).max(1e-38)) as f64).ln(), 1.0 / (s.v[167]));s.store_scalar(488, (if (p[30] == 1.0) { p[705] } else { p[704] }));s.store_primal_scale(491, 822, ((-s.v[488]) * p[76]));s.store_scalar(488, ((-s.v[488]) * p[76]));s.store_scalar(191, (p[1101] + s.v[183]));s.b[1559] = (((p[41] != 0.0) && (p[1099] > 0.0)) && (s.v[191] > 0.0));s.store_scalar(1559, if s.b[1559] { 1.0 } else { 0.0 });
        if s.b[1559] {s.store_scalar(1015, ((s.v[191] * p[2]) / p[1099]));s.store_scalar(1016, ((p[1100] * s.v[191]) * p[2]));}
        if (!s.b[1559]) {s.store_scalar(1015, 1.0);s.store_scalar(1016, 0.0);}
        s.b[1560] = (p[1028] <= (-273.15));s.store_scalar(1560, if s.b[1560] { 1.0 } else { 0.0 });
        if s.b[1560] {s.store_scalar(167, (300.15 - 273.15));s.store_scalar(636, 300.15);}
        if (!s.b[1560]) {s.store_scalar(636, (p[1028] + 273.15));}
        s.store_scalar(635, (ctx_temp + p[23]));s.b[1561] = ((p[41] != 0.0) && (p[1099] > 0.0));s.store_scalar(1561, if s.b[1561] { 1.0 } else { 0.0 });s.b[1562] = ((p[40] != 0.0) && (!true));s.store_scalar(1562, if s.b[1562] { 1.0 } else { 0.0 });s.b[1563] = true;s.store_scalar(1563, if s.b[1563] { 1.0 } else { 0.0 });
        if ((s.b[1561] && s.b[1562]) && s.b[1563]) {s.store_voltage(634, ctx, nodes, Some(4), None);}
        if ((s.b[1561] && s.b[1562]) && (!s.b[1563])) {s.store_voltage(634, ctx, nodes, Some(5), None);}
        if (s.b[1561] && (!s.b[1562])) {s.store_voltage(634, ctx, nodes, Some(5), None);}
        if (!s.b[1561]) {s.store_scalar(634, 0.0);}
        s.store_offset(635, 634, s.v[635]);s.store_scale(271, 635, s.v[1048]);s.store_div_from_scalar(272, 1.0, 271);s.store_div(639, 635, 636);s.store_sub(640, 635, 636);s.store_scale(637, 635, s.v[1048]);s.store_primal_scale(638, 636, s.v[1048]);s.store_sub_from_scalar_ad(190, p[108], A::div_scaled_product_offset_denominator(s.ad_value(635), s.ad_value(635), p[1029], s.ad_value(635), p[1030], 1.0));s.store_mul_div_scaled_inputs_mixed_aii(168, A::sqrt(A::div(s.ad_value(635), s.ad_value(636))), 635, 1.0, 636, 1.0);s.store_mul_scaled_limited_exp_ad_rhs(182, 168, p[107], A::sub(A::div_scaled_inputs(s.ad_value(190), 1.0, s.ad_value(638), 2.0), A::div_scaled_inputs(s.ad_value(190), 1.0, s.ad_value(637), 2.0)));s.b[1564] = (((p[41] != 0.0) && (p[1099] > 0.0)) && (s.v[191] > 0.0));s.store_scalar(1564, if s.b[1564] { 1.0 } else { 0.0 });
        if s.b[1564] {s.store_ln_ad(167, A::max_with_scalar(A::div(s.ad_value(706), s.ad_value(182)), 1e-38));s.store_sqrt_square_offset(251, 167, 1e-6);}
        if (!s.b[1564]) {s.store_ln_ad(251, A::max_with_scalar(A::div(s.ad_value(706), s.ad_value(182)), 1e-38));}
        s.b[1565] = (((p[41] != 0.0) && (p[1099] > 0.0)) && (s.v[191] > 0.0));s.store_scalar(1565, if s.b[1565] { 1.0 } else { 0.0 });
        if s.b[1565] {s.store_ln_ad(167, A::max_with_scalar(A::div_scaled_product(s.ad_value(953), s.ad_value(705), 1.0, A::square(s.ad_value(182)), 1.0), 1e-38));s.store_sqrt_square_offset(942, 167, 1e-6);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1565]) {s.store_ln_ad(942, A::max_with_scalar(A::div_scaled_product(s.ad_value(953), s.ad_value(705), 1.0, A::square(s.ad_value(182)), 1.0), 1e-38));}
        s.b[1566] = (s.v[704] > 0.0);s.store_scalar(1566, if s.b[1566] { 1.0 } else { 0.0 });
        if s.b[1566] {s.store_offset_product3(219, s.ad_value(379), s.ad_value(271), A::ln(A::max_with_scalar(A::div(s.ad_value(704), s.ad_value(705)), 1e-38)), -1.0, p[5]);}
        if (!s.b[1566]) {s.store_scalar(219, 0.0);}
        s.store_max_with_scalar_ad(298, A::add(A::offset(A::mul(s.ad_value(271), s.ad_value(251)), 0.4), s.ad_value(729)), 0.4);s.store_sqrt(299, 298);s.store_sqrt_div_from_scalar_ad(277, (2.0 * s.v[180]), A::scale(s.ad_value(706), 1.602176462e-19));s.store_primal_sqrt_scaled_input(300, 782, ((s.v[180] / s.v[181]) * p[76]));s.store_mul_add_scaled_inputs_rhs(665, 720, A::scale_offset(s.ad_value(639), p[1031], (((((-1.0)) * (p[1031]))) + (1.0))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(639), p[1031], (((((-1.0)) * (p[1031]))) + (1.0))), ((4.0 * 0.001) * 0.001)), 0.5);s.store_mul_scale_offset_rhs(663, 730, 639, p[1059], (((((-1.0)) * (p[1059]))) + (1.0)));s.b[1577] = (p[35] != 0.0);s.store_scalar(1577, if s.b[1577] { 1.0 } else { 0.0 });
        if s.b[1577] {s.store_mul_scale_offset_rhs(664, 731, 639, p[1059], (((((-1.0)) * (p[1059]))) + (1.0)));}
        s.store_scalar(338, (if (p[30] != 1.0) { (0.3333333333333333 * p[347]) } else { (0.5 * p[347]) }));s.store_mul_pow_indices(641, 738, 639, 796);s.store_mul_add_scaled_inputs_rhs(643, 748, A::offset(A::mul(s.ad_value(797), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(797), s.ad_value(640)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5);s.store_mul_add_scaled_inputs_rhs(645, 758, A::offset(A::mul(s.ad_value(798), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(798), s.ad_value(640)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5);s.store_mul_pow_indices(647, 752, 639, 799);s.store_mul_pow_indices(649, 755, 639, 800);s.store_mul_add_scaled_inputs_rhs(651, 751, A::offset(A::mul_offset_rhs(s.ad_value(805), s.ad_value(639), (-1.0)), 1.0), 0.5, A::sqrt_square_offset(A::offset(A::mul_offset_rhs(s.ad_value(805), s.ad_value(639), (-1.0)), 1.0), ((4.0 * 0.001) * 0.001)), 0.5);s.b[1578] = (p[35] != 0.0);s.store_scalar(1578, if s.b[1578] { 1.0 } else { 0.0 });
        if s.b[1578] {s.store_mul_pow_indices(642, 739, 639, 796);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1578] {s.store_mul_add_scaled_inputs_rhs(644, 749, A::offset(A::mul(s.ad_value(797), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(797), s.ad_value(640)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5);s.store_mul_add_scaled_inputs_rhs(646, 759, A::offset(A::mul(s.ad_value(798), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(798), s.ad_value(640)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5);s.store_mul_pow_indices(648, 753, 639, 799);s.store_mul_pow_indices(650, 756, 639, 800);}
        s.store_pow_indices(652, 639, 801);s.store_mul_pow_mixed_iia(653, 741, 639, A::neg(s.ad_value(802)));s.b[1579] = (s.v[653] < 100.0);s.store_scalar(1579, if s.b[1579] { 1.0 } else { 0.0 });
        if s.b[1579] {s.store_scalar(653, 100.0);}
        s.b[1580] = (p[35] != 0.0);s.store_scalar(1580, if s.b[1580] { 1.0 } else { 0.0 });
        if s.b[1580] {s.store_mul_pow_mixed_iia(654, 745, 639, A::neg(s.ad_value(802)));}
        s.b[1581] = (s.v[654] < 100.0);s.store_scalar(1581, if s.b[1581] { 1.0 } else { 0.0 });
        if (s.b[1580] && s.b[1581]) {s.store_scalar(654, 100.0);}
        s.store_mul_pow_mixed_iia(655, 747, 639, A::neg(s.ad_value(802)));s.b[1582] = (s.v[655] < 100.0);s.store_scalar(1582, if s.b[1582] { 1.0 } else { 0.0 });
        if s.b[1582] {s.store_scalar(655, 100.0);}
        s.store_div_from_scalar_offset_ad(656, 1.0, A::add_scaled_inputs(A::offset(A::mul(A::div_from_scalar(1.0, s.ad_value(737)), A::scale_offset(s.ad_value(640), p[1069], 1.0)), (-2.0)), 0.5, A::sqrt_square_offset(A::offset(A::mul(A::div_from_scalar(1.0, s.ad_value(737)), A::scale_offset(s.ad_value(640), p[1069], 1.0)), (-2.0)), ((4.0 * 0.001) * 0.001)), 0.5), 2.0);s.store_mul_add_scaled_inputs_rhs(657, 778, A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(803), s.ad_value(640))), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(803), s.ad_value(640))), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5);s.b[1583] = (p[35] != 0.0);s.store_scalar(1583, if s.b[1583] { 1.0 } else { 0.0 });
        if s.b[1583] {s.store_mul_add_scaled_inputs_rhs(658, 779, A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(803), s.ad_value(640))), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(803), s.ad_value(640))), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_mul_add_scaled_inputs_rhs(330, 328, A::offset(A::mul(s.ad_value(329), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(329), s.ad_value(640)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5);s.store_mul_add_scaled_inputs_rhs(333, 331, A::offset(A::mul(s.ad_value(332), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(332), s.ad_value(640)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5);s.store_mul_pow_indices(659, 866, 639, 804);s.store_add_scaled_offset_product_rhs(660, 893, 1.0, 900, 639, (-1.0), 1.0);s.store_add_scaled_offset_product_rhs(661, 897, 1.0, 901, 639, (-1.0), 1.0);s.store_mul_add_scaled_inputs_rhs(832, 828, A::offset(A::mul(s.ad_value(830), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(830), s.ad_value(640)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5);s.store_mul_add_scaled_inputs_rhs(833, 829, A::offset(A::mul(s.ad_value(831), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(831), s.ad_value(640)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5);s.store_mul_add_scaled_inputs_rhs(858, 856, A::offset(A::mul(s.ad_value(857), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(857), s.ad_value(640)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5);s.store_mul_add_scaled_inputs_rhs(861, 859, A::offset(A::mul(s.ad_value(860), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(860), s.ad_value(640)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5);s.store_mul_add_scaled_inputs_rhs(864, 862, A::offset(A::mul(s.ad_value(863), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(863), s.ad_value(640)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5);s.store_scaled_add_sqrt_square_offset_ad(666, A::scale_offset(s.ad_value(640), p[1093], ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001), (0.5 * p[901]));
    }
}
