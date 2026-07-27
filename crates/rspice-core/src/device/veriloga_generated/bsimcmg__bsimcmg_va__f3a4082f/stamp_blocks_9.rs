#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_9(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1146] = ((p[66] != 0.0) && (s.v[699] <= 0.0));s.store_scalar(1146, if s.b[1146] { 1.0 } else { 0.0 });
        if s.b[1146] {s.store_scalar(699, 85000.0);}
        s.b[1147] = (s.v[670] <= 0.0);s.store_scalar(1147, if s.b[1147] { 1.0 } else { 0.0 });
        if s.b[1147] {s.store_scalar(670, 0.6);}
        s.b[1148] = (s.v[671] <= 0.0);s.store_scalar(1148, if s.b[1148] { 1.0 } else { 0.0 });
        if s.b[1148] {s.store_scalar(671, 0.6);}
        s.b[1152] = (s.v[678] <= 0.0);s.store_scalar(1152, if s.b[1152] { 1.0 } else { 0.0 });
        if s.b[1152] {s.store_scalar(678, 1.06);}
        s.b[1153] = (s.v[673] < 0.0);s.store_scalar(1153, if s.b[1153] { 1.0 } else { 0.0 });
        if s.b[1153] {s.store_scalar(673, 0.0);}
        s.b[1154] = (s.v[677] < 0.0);s.store_scalar(1154, if s.b[1154] { 1.0 } else { 0.0 });
        if s.b[1154] {s.store_scalar(677, 0.0);}
        s.b[1155] = (s.v[803] < (-s.v[153]));s.store_scalar(1155, if s.b[1155] { 1.0 } else { 0.0 });
        if s.b[1155] {s.store_scalar(803, 0.0);}
        s.b[1156] = (s.v[685] < 0.0);s.store_scalar(1156, if s.b[1156] { 1.0 } else { 0.0 });
        if s.b[1156] {s.store_scalar(685, 0.0);}
        s.b[1157] = (s.v[687] < 0.0);s.store_scalar(1157, if s.b[1157] { 1.0 } else { 0.0 });
        if s.b[1157] {s.store_scalar(687, 0.0);}
        s.b[1158] = ((p[61] != 0.0) && (s.v[689] < 0.2));s.store_scalar(1158, if s.b[1158] { 1.0 } else { 0.0 });
        if s.b[1158] {s.store_scalar(689, 0.2);}
        s.b[1159] = ((p[61] != 0.0) && (s.v[689] > 1.2));s.store_scalar(1159, if s.b[1159] { 1.0 } else { 0.0 });
        if s.b[1159] {s.store_scalar(689, 1.2);}
        s.b[1160] = (s.v[695] < 2.0);s.store_scalar(1160, if s.b[1160] { 1.0 } else { 0.0 });
        if s.b[1160] {s.store_scalar(695, 2.0);}
        s.b[1161] = (s.v[697] < 2.0);s.store_scalar(1161, if s.b[1161] { 1.0 } else { 0.0 });
        if s.b[1161] {s.store_scalar(697, 2.0);}
        s.b[1162] = (s.v[704] < 0.0);s.store_scalar(1162, if s.b[1162] { 1.0 } else { 0.0 });
        if s.b[1162] {s.store_scalar(704, 0.03);}
        s.b[1163] = (s.v[807] < 0.0);s.store_scalar(1163, if s.b[1163] { 1.0 } else { 0.0 });
        if s.b[1163] {s.store_scalar(807, 0.0);}
        s.b[1164] = (s.v[811] < 0.0);s.store_scalar(1164, if s.b[1164] { 1.0 } else { 0.0 });
        if s.b[1164] {s.store_scalar(811, 0.0);}
        s.b[1165] = (s.v[812] < 0.0);s.store_scalar(1165, if s.b[1165] { 1.0 } else { 0.0 });
        if s.b[1165] {s.store_scalar(812, 0.0);}
        s.b[1166] = (s.v[814] < 0.0);s.store_scalar(1166, if s.b[1166] { 1.0 } else { 0.0 });
        if s.b[1166] {s.store_scalar(814, 0.0);}
        s.b[1167] = (s.v[707] < 0.0);s.store_scalar(1167, if s.b[1167] { 1.0 } else { 0.0 });
        if s.b[1167] {s.store_scalar(707, 0.0);}
        s.b[1168] = (s.v[709] < 0.0);s.store_scalar(1168, if s.b[1168] { 1.0 } else { 0.0 });
        if s.b[1168] {s.store_scalar(709, 0.0);}
        s.b[1169] = (s.v[853] < 0.0);s.store_scalar(1169, if s.b[1169] { 1.0 } else { 0.0 });
        if s.b[1169] {s.store_scalar(853, 0.0);}
        s.b[1170] = (s.v[852] < 0.0);s.store_scalar(1170, if s.b[1170] { 1.0 } else { 0.0 });
        if s.b[1170] {s.store_scalar(852, 0.0);}
        s.b[1171] = (s.v[712] < 0.0);s.store_scalar(1171, if s.b[1171] { 1.0 } else { 0.0 });
        if s.b[1171] {s.store_scalar(712, 0.0);}
        s.b[1172] = (s.v[711] < 0.0);s.store_scalar(1172, if s.b[1172] { 1.0 } else { 0.0 });
        if s.b[1172] {s.store_scalar(711, 0.0);}
        s.b[1175] = (p[66] != 0.0);s.store_scalar(1175, if s.b[1175] { 1.0 } else { 0.0 });s.b[1178] = (s.v[706] < 0.0);s.store_scalar(1178, if s.b[1178] { 1.0 } else { 0.0 });
        if (s.b[1175] && s.b[1178]) {s.store_scalar(706, 0.0);}
        s.b[1179] = (s.v[815] < 0.0);s.store_scalar(1179, if s.b[1179] { 1.0 } else { 0.0 });
        if (s.b[1175] && s.b[1179]) {s.store_scalar(815, 0.0);}
        s.b[1180] = (s.v[816] < 0.0);s.store_scalar(1180, if s.b[1180] { 1.0 } else { 0.0 });
        if (s.b[1175] && s.b[1180]) {s.store_scalar(816, 0.0);}
        s.b[1181] = (s.v[818] < 0.0);s.store_scalar(1181, if s.b[1181] { 1.0 } else { 0.0 });
        if (s.b[1175] && s.b[1181]) {s.store_scalar(818, 0.0);}
        s.b[1183] = (s.v[719] <= 0.0);s.store_scalar(1183, if s.b[1183] { 1.0 } else { 0.0 });
        if s.b[1183] {s.store_scalar(719, 1.06);}
        s.b[1184] = (s.v[790] < 2.0);s.store_scalar(1184, if s.b[1184] { 1.0 } else { 0.0 });
        if s.b[1184] {s.store_scalar(790, 2.0);}
        s.b[1185] = (p[66] != 0.0);s.store_scalar(1185, if s.b[1185] { 1.0 } else { 0.0 });s.b[1186] = (s.v[791] < 2.0);s.store_scalar(1186, if s.b[1186] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1185] && s.b[1186]) {s.store_scalar(791, 2.0);}
        s.b[1187] = (s.v[700] < 0.0);s.store_scalar(1187, if s.b[1187] { 1.0 } else { 0.0 });
        if s.b[1187] {s.store_scalar(700, 0.0);}
        s.b[1188] = (s.v[749] < 0.0);s.store_scalar(1188, if s.b[1188] { 1.0 } else { 0.0 });
        if s.b[1188] {s.store_scalar(749, 0.0);}
        s.b[1189] = (s.v[763] < 0.0);s.store_scalar(1189, if s.b[1189] { 1.0 } else { 0.0 });
        if s.b[1189] {s.store_scalar(763, 0.0);}
        s.b[1190] = (p[69] != 0.0);s.store_scalar(1190, if s.b[1190] { 1.0 } else { 0.0 });s.b[1191] = (s.v[726] <= 0.0);s.store_scalar(1191, if s.b[1191] { 1.0 } else { 0.0 });
        if (s.b[1190] && s.b[1191]) {s.store_scalar(726, 3.0);}
        s.b[1192] = (s.v[731] <= 0.0);s.store_scalar(1192, if s.b[1192] { 1.0 } else { 0.0 });
        if (s.b[1190] && s.b[1192]) {s.store_scalar(731, 1.0);}
        s.b[1193] = (p[68] != 0.0);s.store_scalar(1193, if s.b[1193] { 1.0 } else { 0.0 });s.b[1194] = (s.v[742] <= 0.0);s.store_scalar(1194, if s.b[1194] { 1.0 } else { 0.0 });
        if (s.b[1193] && s.b[1194]) {s.store_scalar(742, 1.0);}
        s.b[1195] = (s.v[736] <= 0.0);s.store_scalar(1195, if s.b[1195] { 1.0 } else { 0.0 });
        if (s.b[1193] && s.b[1195]) {s.store_scalar(736, 1.0);}
        s.b[1213] = (s.v[648] < 0.0);s.store_scalar(1213, if s.b[1213] { 1.0 } else { 0.0 });
        if s.b[1213] {s.store_scalar(648, 0.0);}
        s.b[1214] = (s.v[649] < 0.0);s.store_scalar(1214, if s.b[1214] { 1.0 } else { 0.0 });
        if s.b[1214] {s.store_scalar(649, 0.0);}
        s.b[1215] = (s.v[643] < 0.0);s.store_scalar(1215, if s.b[1215] { 1.0 } else { 0.0 });
        if s.b[1215] {s.store_scalar(643, 0.0);}
        s.b[1216] = (s.v[642] < 0.0);s.store_scalar(1216, if s.b[1216] { 1.0 } else { 0.0 });
        if s.b[1216] {s.store_scalar(642, 0.0);}
        s.b[1217] = (s.v[650] < 0.0);s.store_scalar(1217, if s.b[1217] { 1.0 } else { 0.0 });
        if s.b[1217] {s.store_scalar(650, 0.0);}
        s.b[1218] = (s.v[651] <= 0.02);s.store_scalar(1218, if s.b[1218] { 1.0 } else { 0.0 });
        if s.b[1218] {s.store_scalar(651, 0.02);}
        s.b[1219] = (s.v[652] <= 0.02);s.store_scalar(1219, if s.b[1219] { 1.0 } else { 0.0 });
        if s.b[1219] {s.store_scalar(652, 0.02);}
        s.b[1220] = (s.v[653] <= 0.02);s.store_scalar(1220, if s.b[1220] { 1.0 } else { 0.0 });
        if s.b[1220] {s.store_scalar(653, 0.02);}
        s.b[1221] = (s.v[446] < (-p[4]));s.store_scalar(1221, if s.b[1221] { 1.0 } else { 0.0 });
        if s.b[1221] {s.store_scalar(446, 0.0);}
        s.b[1222] = (p[57] == 1.0);s.store_scalar(1222, if s.b[1222] { 1.0 } else { 0.0 });s.b[1223] = ((s.v[882] < 1.0) || (s.v[882] > 3.0));s.store_scalar(1223, if s.b[1223] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1223]) {s.store_scalar(882, 2.0);}
        s.b[1224] = ((s.v[883] < 1.0) || (s.v[883] > 3.0));s.store_scalar(1224, if s.b[1224] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1224]) {s.store_scalar(883, 2.6);}
        s.b[1225] = ((s.v[884] < 1.0) || (s.v[884] > 3.0));s.store_scalar(1225, if s.b[1225] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1225]) {s.store_scalar(884, 2.6);}
        s.b[1226] = (s.v[885] < 0.0);s.store_scalar(1226, if s.b[1226] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1226]) {s.store_scalar(885, 14.0);}
        s.b[1227] = (s.v[886] < 0.0);s.store_scalar(1227, if s.b[1227] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1227]) {s.store_scalar(886, 24.0);}
        s.b[1228] = (s.v[887] < 0.0);s.store_scalar(1228, if s.b[1228] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1228]) {s.store_scalar(887, 24.0);}
        s.b[1229] = (s.v[888] < 0.0);s.store_scalar(1229, if s.b[1229] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1229]) {s.store_scalar(888, 0.139);}
        s.b[1230] = (s.v[889] < 0.0);s.store_scalar(1230, if s.b[1230] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1230]) {s.store_scalar(889, 2.0);}
        s.b[1231] = (s.v[890] < 0.0);s.store_scalar(1231, if s.b[1231] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1231]) {s.store_scalar(890, 11.2);}
        s.b[1232] = (s.v[891] < 0.0);s.store_scalar(1232, if s.b[1232] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1232]) {s.store_scalar(891, 8.02);}
        s.b[1233] = (s.v[892] < 0.0);s.store_scalar(1233, if s.b[1233] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1233]) {s.store_scalar(892, 6.18);}
        s.b[1234] = ((p[74] != 0.0) && (p[1791] > 0.0));s.store_scalar(1234, if s.b[1234] { 1.0 } else { 0.0 });s.b[1235] = (p[1795] != 0.0);s.store_scalar(1235, if s.b[1235] { 1.0 } else { 0.0 });
        if (s.b[1234] && s.b[1235]) {s.store_scalar(169, (p[1793] * ((p[59]) as f64).powf(p[1795])));}
        if (s.b[1234] && (!s.b[1235])) {s.store_scalar(169, p[1793]);}
        s.b[1236] = (p[1794] != 0.0);s.store_scalar(1236, if s.b[1236] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1234] && s.b[1236]) {s.store_scalar(170, ((p[1797] * p[4]) * ((s.v[115]) as f64).powf(p[1794])));}
        if (s.b[1234] && (!s.b[1236])) {s.store_scalar(170, (p[1797] * p[4]));}
        s.b[1237] = (p[62] == 5.0);s.store_scalar(1237, if s.b[1237] { 1.0 } else { 0.0 });s.b[1238] = (p[1796] != 0.0);s.store_scalar(1238, if s.b[1238] { 1.0 } else { 0.0 });
        if ((s.b[1234] && s.b[1237]) && s.b[1238]) {s.store_scalar(171, (((p[1798] * p[59]) * p[43]) * ((p[56]) as f64).powf(p[1796])));}
        if ((s.b[1234] && s.b[1237]) && (!s.b[1238])) {s.store_scalar(171, ((p[1798] * p[59]) * p[43]));}
        if (s.b[1234] && (!s.b[1237])) {s.store_scalar(171, 0.0);}
        if s.b[1234] {s.store_add_scaled_inputs3_indices(634, 169, p[1792], 170, p[1792], 171, p[1792]);}
        s.b[1241] = (p[77] == 0.0);s.store_scalar(1241, if s.b[1241] { 1.0 } else { 0.0 });
        if s.b[1241] {s.store_scalar(190, (p[1078] * p[18]));s.store_scalar(191, (p[1079] * p[19]));}
        s.b[1242] = (p[1080] > 0.0);s.store_scalar(1242, if s.b[1242] { 1.0 } else { 0.0 });
        if ((!s.b[1241]) && s.b[1242]) {s.store_scalar(444, ((p[4] * p[92]) + ((p[3] + ((p[4] - p[3]) * p[1084])) * p[1080])));}
        if ((!s.b[1241]) && (!s.b[1242])) {s.store_scalar(444, (p[4] * (1e-9_f64).max((p[92] + p[1080]))));}
        if (!s.b[1241]) {s.store_primal_offset(445, 446, p[4]);}
        s.b[1243] = param_given[1083];s.store_scalar(1243, if s.b[1243] { 1.0 } else { 0.0 });
        if ((!s.b[1241]) && s.b[1243]) {s.store_scalar(431, p[1083]);}
        if ((!s.b[1241]) && (!s.b[1243])) {s.store_scalar(429, (if (p[60] == 1.0) { 1417.0 } else { 470.5 }));}
        s.b[1244] = (p[60] == 1.0);s.store_scalar(1244, if s.b[1244] { 1.0 } else { 0.0 });
        if (((!s.b[1241]) && (!s.b[1243])) && s.b[1244]) {s.store_scalar(168, (((p[97] / 9.68e22)) as f64).powf(0.68));s.store_scalar(169, (3.43e26 / p[97]));s.store_scaled_sub_ad(430, A::offset(A::div_scaled_offset_numerator(s.ad_value(429), 1.0, (-52.2), A::offset(s.ad_value(168), 1.0), 1.0), 52.2), A::div_scalar_offset_denominator(43.4, A::square(s.ad_value(169)), 1.0, 1.0), 0.0001);}
        if (((!s.b[1241]) && (!s.b[1243])) && (!s.b[1244])) {s.store_scalar(168, (((p[97] / 2.23e22)) as f64).powf(0.719));s.store_scalar(169, (6.1e26 / p[97]));s.store_scaled_sub_ad(430, A::offset(A::div_scaled_offset_numerator(s.ad_value(429), 1.0, (-44.9), A::offset(s.ad_value(168), 1.0), 1.0), 44.9), A::div_scalar_offset_denominator(29.0, A::square(s.ad_value(169)), 1.0, 1.0), 0.0001);}
        if ((!s.b[1241]) && (!s.b[1243])) {s.store_div_from_scalar_scaled_input(431, 1.0, 430, (1.60219e-19 * p[97]));}
        if (!s.b[1241]) {s.store_scalar(433, ((55.0 * 3.141592653589793) / 180.0));s.store_primal_min_with_scalar(432, 444, (1e-18_f64).max((p[3] * (p[92] + (0.0_f64).min(p[1080])))));s.store_scaled_mul_ad(434, A::div(s.ad_value(431), A::tan(s.ad_value(433))), A::add_scaled_inputs3(A::div_from_scalar(1.0, A::sqrt(s.ad_value(432))), 1.0, A::div_from_scalar(2.0, A::sqrt(s.ad_value(444))), (-1.0), A::sqrt(A::div(s.ad_value(432), A::square(s.ad_value(444)))), 1.0), 1.0 / ((((3.141592653589793) as f64).sqrt() * p[5])));s.store_primal_offset_scaled(436, 444, p[5], p[1092]);s.store_primal_offset_scaled(437, 445, p[5], p[1093]);s.store_sqrt_ad(435, A::div_scaled_inputs(s.ad_value(436), p[1082], A::mul(s.ad_value(431), s.ad_value(437)), 1.0));s.store_div_from_scalar(438, p[20], 435);s.store_limited_exp_scaled_input(168, 438, 2.0);}
        s.b[1245] = (p[1086] == 1.0);s.store_scalar(1245, if s.b[1245] { 1.0 } else { 0.0 });
        if ((!s.b[1241]) && s.b[1245]) {s.store_scaled_mul(439, 431, 435, 1.0 / (p[1082]));s.store_mul_scale_offset_indices(169, 168, 439, 1.0, 1.0);s.store_sub_offset_lhs(170, 169, 1.0, 439);s.store_add_offset_lhs(171, 169, (-1.0), 439);}
        if ((!s.b[1241]) && (!s.b[1245])) {s.store_offset(170, 168, 1.0);s.store_offset(171, 168, (-1.0));}
        if (!s.b[1241]) {s.store_div_scaled_product3_by_product_indices(440, 431, 435, 170, 1.0, 436, 171, 1.0);}
        s.b[1246] = (p[1080] < (-1e-10));s.store_scalar(1246, if s.b[1246] { 1.0 } else { 0.0 });
        if ((!s.b[1241]) && s.b[1246]) {s.store_scalar(441, (p[1082] / (((-p[1080]) * p[3]) * p[5])));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((!s.b[1241]) && s.b[1246]) {s.store_div_scaled_product_mixed_aia(442, A::add(s.ad_value(440), s.ad_value(434)), 441, 1.0, A::add_scaled_inputs3(s.ad_value(440), 1.0, s.ad_value(434), 1.0, s.ad_value(441), 1.0), 1.0);}
        if ((!s.b[1241]) && (!s.b[1246])) {s.store_add(442, 440, 434);}
        if (!s.b[1241]) {s.store_scale(443, 442, (1.0 / (p[59]) * (0.0_f64).max(((((p[1094] + (p[1095] * p[3])) + (p[1096] * p[4])) + (p[1097] * p[20])) + (p[1098] * p[1080])))));s.copy_ad(190, 443);s.copy_ad(191, 443);}
        s.b[1247] = (p[64] == 0.0);s.store_scalar(1247, if s.b[1247] { 1.0 } else { 0.0 });s.b[1248] = (s.v[190] < p[151]);s.store_scalar(1248, if s.b[1248] { 1.0 } else { 0.0 });
        if (s.b[1247] && s.b[1248]) {s.store_scalar(190, 0.0);}
        s.b[1249] = (s.v[191] < p[151]);s.store_scalar(1249, if s.b[1249] { 1.0 } else { 0.0 });
        if (s.b[1247] && s.b[1249]) {s.store_scalar(191, 0.0);}
        s.b[1250] = (s.v[190] <= p[151]);s.store_scalar(1250, if s.b[1250] { 1.0 } else { 0.0 });
        if ((!s.b[1247]) && s.b[1250]) {s.store_scalar(190, p[151]);}
        s.b[1251] = (s.v[191] <= p[151]);s.store_scalar(1251, if s.b[1251] { 1.0 } else { 0.0 });
        if ((!s.b[1247]) && s.b[1251]) {s.store_scalar(191, p[151]);}
        s.b[1252] = (p[78] != 1.0);s.store_scalar(1252, if s.b[1252] { 1.0 } else { 0.0 });s.b[1253] = param_given[1542];s.store_scalar(1253, if s.b[1253] { 1.0 } else { 0.0 });
        if (s.b[1252] && s.b[1253]) {s.store_scalar(646, p[1542]);}
        s.b[1254] = (param_given[85] && (p[85] > 0.0));s.store_scalar(1254, if s.b[1254] { 1.0 } else { 0.0 });
        if ((s.b[1252] && (!s.b[1253])) && s.b[1254]) {s.store_primal_max_from_scalar_ad(646, 0.0, A::sub_scaled_inputs(s.ad_value(163), p[85], s.ad_value(648), 1.0));}
        s.b[1255] = (p[78] == 3.0);s.store_scalar(1255, if s.b[1255] { 1.0 } else { 0.0 });
        if (((s.b[1252] && (!s.b[1253])) && (!s.b[1254])) && s.b[1255]) {s.store_primal_scale(646, 163, (0.3 * p[43]));}
        if (((s.b[1252] && (!s.b[1253])) && (!s.b[1254])) && (!s.b[1255])) {s.store_primal_scale(646, 163, (0.3 * p[3]));}
        s.b[1256] = param_given[1543];s.store_scalar(1256, if s.b[1256] { 1.0 } else { 0.0 });
        if (s.b[1252] && s.b[1256]) {s.store_scalar(647, p[1543]);}
        s.b[1257] = (param_given[85] && (p[85] > 0.0));s.store_scalar(1257, if s.b[1257] { 1.0 } else { 0.0 });
        if ((s.b[1252] && (!s.b[1256])) && s.b[1257]) {s.store_primal_max_from_scalar_ad(647, 0.0, A::sub_scaled_inputs(s.ad_value(163), p[85], s.ad_value(649), 1.0));}
        s.b[1258] = (p[78] == 3.0);s.store_scalar(1258, if s.b[1258] { 1.0 } else { 0.0 });
        if (((s.b[1252] && (!s.b[1256])) && (!s.b[1257])) && s.b[1258]) {s.store_primal_scale(647, 163, (0.3 * p[43]));}
        if (((s.b[1252] && (!s.b[1256])) && (!s.b[1257])) && (!s.b[1258])) {s.store_primal_scale(647, 163, (0.3 * p[3]));}
        s.b[1259] = (p[78] == 2.0);s.store_scalar(1259, if s.b[1259] { 1.0 } else { 0.0 });
        if s.b[1259] {s.store_scalar(447, (p[1089] + p[1090]));s.store_scalar(449, (0.5 * (p[4] - p[3])));s.store_primal_max_from_scalar_ad(448, 0.0, A::offset(s.ad_value(449), (-p[90])));s.store_scalar(450, (0.0_f64).max((p[1080] + p[1081])));}
        s.b[1260] = (p[1090] > 0.0);s.store_scalar(1260, if s.b[1260] { 1.0 } else { 0.0 });
        if (s.b[1259] && s.b[1260]) {s.store_scalar(168, (3.467e-11 * (if (!(((1e-7 * p[1088]) / (3.9 * p[1087])) > 1e-38)) { (-87.498233534) } else { (if (((1e-7 * p[1088]) / (3.9 * p[1087])) > 1e-38) { ((((1e-7 * p[1088]) / (3.9 * p[1087]))) as f64).ln() } else { 0.0 }) })));}
        if (s.b[1259] && s.b[1260]) {s.store_scale(169, 450, (0.942 * (s.v[144] * 1.0 / (p[1087]))));s.store_scaled_add(451, 168, 169, (p[3] + ((p[4] - p[3]) * p[1084])));}
        if (s.b[1259] && (!s.b[1260])) {s.store_primal_offset_div_scaled_offset_numerator_indices(925, 447, 0.2, (p[90] * 0.2), 450, 1.0, 2.3);s.store_scalar(926, 1.05);s.store_primal_abs_ad(927, A::sub(A::offset(s.ad_value(447), p[90]), s.ad_value(450)));s.store_primal_scale(928, 926, p[1087]);s.store_primal_min_offset_rhs(929, 450, 447, p[90]);s.store_primal_div_from_scalar_offset_input(930, p[1087], 925, 1.0);s.store_scalar(931, 1700000000000.0);s.store_primal_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p[1087])));s.store_mul(933, 931, 932);}
        s.b[1261] = (s.v[933] > 80.0);s.store_scalar(1261, if s.b[1261] { 1.0 } else { 0.0 });
        if ((s.b[1259] && (!s.b[1260])) && s.b[1261]) {s.copy_ad(934, 932);}
        if ((s.b[1259] && (!s.b[1260])) && (!s.b[1261])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_13(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1259] && (!s.b[1260])) {s.store_primal_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(450), 1.0, s.ad_value(447), p[90], 1.0), A::div_scaled_offset_numerator(s.ad_value(447), 1.0, p[90], s.ad_value(450), 1.0)), 0.5);s.store_primal_mul(936, 927, 935);}
        if (s.b[1259] && (!s.b[1260])) {
            s.store_primal_scale_ad(937, {
                if (!(((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p[1087]), 1.0 / (p[1087]))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }
        if (s.b[1259] && (!s.b[1260])) {s.store_scaled_add(938, 934, 937, p[3]);s.store_primal_div(930, 928, 447);s.store_primal_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);s.store_primal_add_mixed_ai(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(447), (2.0 * p[90]), (p[90] * p[90])), 1.0, A::square(s.ad_value(447)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p[90]), 1.0, s.ad_value(447), s.ad_value(930), 1.0), 447);s.store_primal_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p[90], A::scaled_offset(s.ad_value(930), 2.0, p[90]), 1.0);}
        if (s.b[1259] && (!s.b[1260])) {
            s.store_primal_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }
        if (s.b[1259] && (!s.b[1260])) {s.store_primal_mul(943, 925, 926);s.store_primal_sqrt_square_offset(944, 943, 1.0);s.store_add_mixed_ai(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p[90] * p[90]), s.ad_value(943), s.ad_value(928), (2.0 * p[90])), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p[90], A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);s.store_primal_mul_scale_offset_mixed_ai(945, A::offset(s.ad_value(944), 1.0), 943, p[90], 0.0);}
        if (s.b[1259] && (!s.b[1260])) {
            s.store_mul_div_scaled_inputs_mixed_aii(946, {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 943, ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85), 944, 1.0);
        }
        if (s.b[1259] && (!s.b[1260])) {s.store_scalar(627, 1.2e-12);s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1259] && (!s.b[1260])) {s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p[3], 933, ((-0.5) * p[3]), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p[3]));s.store_add(451, 938, 947);}
        s.b[1262] = (p[1090] > 0.0);s.store_scalar(1262, if s.b[1262] { 1.0 } else { 0.0 });
        if (s.b[1259] && s.b[1262]) {s.store_primal_offset_div_scaled_offset_numerator_indices(925, 448, 0.2, (p[90] * 0.2), 449, 1.0, 2.3);s.store_scalar(926, 1.05);s.store_primal_abs_ad(927, A::sub(A::offset(s.ad_value(448), p[90]), s.ad_value(449)));s.store_primal_scale(928, 926, p[1087]);s.store_primal_min_offset_rhs(929, 449, 448, p[90]);s.store_primal_div_from_scalar_offset_input(930, p[1087], 925, 1.0);s.store_scalar(931, 1700000000000.0);s.store_primal_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p[1087])));s.store_mul(933, 931, 932);}
        s.b[1263] = (s.v[933] > 80.0);s.store_scalar(1263, if s.b[1263] { 1.0 } else { 0.0 });
        if ((s.b[1259] && s.b[1262]) && s.b[1263]) {s.copy_ad(934, 932);}
        if ((s.b[1259] && s.b[1262]) && (!s.b[1263])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }
        if (s.b[1259] && s.b[1262]) {s.store_primal_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(449), 1.0, s.ad_value(448), p[90], 1.0), A::div_scaled_offset_numerator(s.ad_value(448), 1.0, p[90], s.ad_value(449), 1.0)), 0.5);s.store_primal_mul(936, 927, 935);}
        if (s.b[1259] && s.b[1262]) {
            s.store_primal_scale_ad(937, {
                if (!(((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p[1087]), 1.0 / (p[1087]))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }
        if (s.b[1259] && s.b[1262]) {s.store_scaled_add(938, 934, 937, p[92]);s.store_primal_div(930, 928, 448);s.store_primal_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);s.store_primal_add_mixed_ai(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(448), (2.0 * p[90]), (p[90] * p[90])), 1.0, A::square(s.ad_value(448)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p[90]), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);s.store_primal_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p[90], A::scaled_offset(s.ad_value(930), 2.0, p[90]), 1.0);}
        if (s.b[1259] && s.b[1262]) {
            s.store_primal_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }
        if (s.b[1259] && s.b[1262]) {s.store_primal_mul(943, 925, 926);s.store_primal_sqrt_square_offset(944, 943, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1259] && s.b[1262]) {s.store_add_mixed_ai(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p[90] * p[90]), s.ad_value(943), s.ad_value(928), (2.0 * p[90])), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p[90], A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);s.store_primal_mul_scale_offset_mixed_ai(945, A::offset(s.ad_value(944), 1.0), 943, p[90], 0.0);}
        if (s.b[1259] && s.b[1262]) {
            s.store_mul_div_scaled_inputs_mixed_aii(946, {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 943, ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.7), 944, 1.0);
        }
        if (s.b[1259] && s.b[1262]) {s.store_scalar(627, 1.2e-12);s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p[92], 933, ((-0.5) * p[92]), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p[92]));s.store_add(452, 938, 947);}
        if (s.b[1259] && (!s.b[1262])) {s.store_primal_offset_div_scaled_offset_numerator_indices(925, 448, 0.2, (p[90] * 0.2), 449, 1.0, 2.3);s.store_scalar(926, 1.05);s.store_primal_abs_ad(927, A::sub(A::offset(s.ad_value(448), p[90]), s.ad_value(449)));s.store_primal_scale(928, 926, p[1087]);s.store_primal_min_offset_rhs(929, 449, 448, p[90]);s.store_primal_div_from_scalar_offset_input(930, p[1087], 925, 1.0);s.store_scalar(931, 1700000000000.0);s.store_primal_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p[1087])));s.store_mul(933, 931, 932);}
        s.b[1264] = (s.v[933] > 80.0);s.store_scalar(1264, if s.b[1264] { 1.0 } else { 0.0 });
        if ((s.b[1259] && (!s.b[1262])) && s.b[1264]) {s.copy_ad(934, 932);}
        if ((s.b[1259] && (!s.b[1262])) && (!s.b[1264])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }
        if (s.b[1259] && (!s.b[1262])) {s.store_primal_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(449), 1.0, s.ad_value(448), p[90], 1.0), A::div_scaled_offset_numerator(s.ad_value(448), 1.0, p[90], s.ad_value(449), 1.0)), 0.5);s.store_primal_mul(936, 927, 935);}
        if (s.b[1259] && (!s.b[1262])) {
            s.store_primal_scale_ad(937, {
                if (!(((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p[1087]), 1.0 / (p[1087]))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }
        if (s.b[1259] && (!s.b[1262])) {s.store_scaled_add(938, 934, 937, p[92]);s.store_primal_div(930, 928, 448);s.store_primal_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1259] && (!s.b[1262])) {s.store_primal_add_mixed_ai(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(448), (2.0 * p[90]), (p[90] * p[90])), 1.0, A::square(s.ad_value(448)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p[90]), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);s.store_primal_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p[90], A::scaled_offset(s.ad_value(930), 2.0, p[90]), 1.0);}
        if (s.b[1259] && (!s.b[1262])) {
            s.store_primal_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }
        if (s.b[1259] && (!s.b[1262])) {s.store_primal_mul(943, 925, 926);s.store_primal_sqrt_square_offset(944, 943, 1.0);s.store_add_mixed_ai(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p[90] * p[90]), s.ad_value(943), s.ad_value(928), (2.0 * p[90])), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p[90], A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);s.store_primal_mul_scale_offset_mixed_ai(945, A::offset(s.ad_value(944), 1.0), 943, p[90], 0.0);}
        if (s.b[1259] && (!s.b[1262])) {
            s.store_mul_div_scaled_inputs_mixed_aii(946, {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 943, ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85), 944, 1.0);
        }
        if (s.b[1259] && (!s.b[1262])) {s.store_scalar(627, 1.2e-12);s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p[92], 933, ((-0.5) * p[92]), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p[92]));s.store_add(452, 938, 947);}
        s.b[1265] = (p[1090] > 0.0);s.store_scalar(1265, if s.b[1265] { 1.0 } else { 0.0 });
        if (s.b[1259] && s.b[1265]) {s.store_scalar(454, 0.0);}
        s.b[1266] = (p[1080] > 0.0);s.store_scalar(1266, if s.b[1266] { 1.0 } else { 0.0 });
        if ((s.b[1259] && (!s.b[1265])) && s.b[1266]) {s.store_scalar(454, ((p[4] - p[3]) * ((p[1080] * p[1084]) + p[1081])));}
        if ((s.b[1259] && (!s.b[1265])) && (!s.b[1266])) {s.store_primal_scale(454, 450, (p[4] - p[3]));}
        if s.b[1259] {s.store_primal_offset_scaled(455, 454, ((p[5]) * ((s.v[144] * 1.0 / (p[1087])))), ((((p[1092]) + (p[1091]))) * ((s.v[144] * 1.0 / (p[1087])))));s.store_add_scaled_inputs3_indices(453, 455, p[59], 451, (p[5] * p[59]), 452, ((p[1103] * (p[5] * 2.0)) * p[59]));s.store_scale(453, 453, (0.0_f64).max((((p[1099] + (p[1100] * p[3])) + (p[1101] * p[4])) + (p[1102] * p[20]))));}
        s.b[1267] = (p[78] == 3.0);s.store_scalar(1267, if s.b[1267] { 1.0 } else { 0.0 });
        if s.b[1267] {s.store_scalar(447, (p[1089] + p[1090]));s.store_scalar(449, (0.5 * (p[4] - p[43])));s.store_primal_max_from_scalar_ad(448, 0.0, A::offset(s.ad_value(449), (-p[90])));s.store_scalar(450, (0.0_f64).max((p[1080] + p[1081])));s.store_scalar(1031, (0.5 * p[41]));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1268] = (p[1090] > 0.0);s.store_scalar(1268, if s.b[1268] { 1.0 } else { 0.0 });
        if (s.b[1267] && s.b[1268]) {s.store_scalar(168, (3.467e-11 * (if (!(((1e-7 * p[1088]) / (3.9 * p[1087])) > 1e-38)) { (-87.498233534) } else { (if (((1e-7 * p[1088]) / (3.9 * p[1087])) > 1e-38) { ((((1e-7 * p[1088]) / (3.9 * p[1087]))) as f64).ln() } else { 0.0 }) })));}
        if (s.b[1267] && s.b[1268]) {s.store_scale(169, 450, (0.942 * (s.v[144] * 1.0 / (p[1087]))));s.store_scaled_add(1034, 168, 169, (p[43] + ((p[4] - p[43]) * p[1084])));}
        if (s.b[1267] && (!s.b[1268])) {s.store_primal_offset_div_scaled_offset_numerator_indices(925, 447, 0.2, (p[90] * 0.2), 450, 1.0, 2.3);s.store_scalar(926, 1.05);s.store_primal_abs_ad(927, A::sub(A::offset(s.ad_value(447), p[90]), s.ad_value(450)));s.store_primal_scale(928, 926, p[1087]);s.store_primal_min_offset_rhs(929, 450, 447, p[90]);s.store_primal_div_from_scalar_offset_input(930, p[1087], 925, 1.0);s.store_scalar(931, 1700000000000.0);s.store_primal_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p[1087])));s.store_mul(933, 931, 932);}
        s.b[1269] = (s.v[933] > 80.0);s.store_scalar(1269, if s.b[1269] { 1.0 } else { 0.0 });
        if ((s.b[1267] && (!s.b[1268])) && s.b[1269]) {s.copy_ad(934, 932);}
        if ((s.b[1267] && (!s.b[1268])) && (!s.b[1269])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }
        if (s.b[1267] && (!s.b[1268])) {s.store_primal_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(450), 1.0, s.ad_value(447), p[90], 1.0), A::div_scaled_offset_numerator(s.ad_value(447), 1.0, p[90], s.ad_value(450), 1.0)), 0.5);s.store_primal_mul(936, 927, 935);}
        if (s.b[1267] && (!s.b[1268])) {
            s.store_primal_scale_ad(937, {
                if (!(((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p[1087]), 1.0 / (p[1087]))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }
        if (s.b[1267] && (!s.b[1268])) {s.store_scaled_add(938, 934, 937, p[43]);s.store_primal_div(930, 928, 447);s.store_primal_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);s.store_primal_add_mixed_ai(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(447), (2.0 * p[90]), (p[90] * p[90])), 1.0, A::square(s.ad_value(447)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p[90]), 1.0, s.ad_value(447), s.ad_value(930), 1.0), 447);s.store_primal_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p[90], A::scaled_offset(s.ad_value(930), 2.0, p[90]), 1.0);}
        if (s.b[1267] && (!s.b[1268])) {
            s.store_primal_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }
        if (s.b[1267] && (!s.b[1268])) {s.store_primal_mul(943, 925, 926);s.store_primal_sqrt_square_offset(944, 943, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1267] && (!s.b[1268])) {s.store_add_mixed_ai(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p[90] * p[90]), s.ad_value(943), s.ad_value(928), (2.0 * p[90])), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p[90], A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);s.store_primal_mul_scale_offset_mixed_ai(945, A::offset(s.ad_value(944), 1.0), 943, p[90], 0.0);}
        if (s.b[1267] && (!s.b[1268])) {
            s.store_mul_div_scaled_inputs_mixed_aii(946, {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 943, ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85), 944, 1.0);
        }
        if (s.b[1267] && (!s.b[1268])) {s.store_scalar(627, 1.2e-12);s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p[43], 933, ((-0.5) * p[43]), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p[43]));s.store_add(1034, 938, 947);}
        if s.b[1267] {s.store_primal_offset_div_from_scalar_ad(925, (0.2 * (p[1089] + p[90])), s.ad_value(1031), 2.3);s.store_scalar(926, 1.05);s.store_primal_abs_ad(927, A::sub_from_scalar((p[1089] + p[90]), s.ad_value(1031)));s.store_primal_scale(928, 926, p[1087]);s.store_primal_min_with_scalar(929, 1031, (p[1089] + p[90]));s.store_primal_div_from_scalar_offset_input(930, p[1087], 925, 1.0);s.store_scalar(931, 1700000000000.0);s.store_primal_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p[1087])));s.store_mul(933, 931, 932);}
        s.b[1270] = (s.v[933] > 80.0);s.store_scalar(1270, if s.b[1270] { 1.0 } else { 0.0 });
        if (s.b[1267] && s.b[1270]) {s.copy_ad(934, 932);}
        if (s.b[1267] && (!s.b[1270])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }
        if s.b[1267] {s.store_primal_scale_ad(935, A::min(A::scale(s.ad_value(1031), 1.0 / ((p[1089] + p[90]))), A::div_from_scalar((p[1089] + p[90]), s.ad_value(1031))), 0.5);s.store_primal_mul(936, 927, 935);}
        if s.b[1267] {
            s.store_primal_scale_ad(937, {
                if (!(((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p[1087]), 1.0 / (p[1087]))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }
        if s.b[1267] {s.store_scaled_add(938, 934, 937, p[43]);s.store_primal_scale(930, 928, 1.0 / (p[1089]));s.store_primal_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);s.store_primal_offset_add_scaled_inputs_mixed_ai(940, A::offset(A::mul(A::sqrt(A::scale_offset(s.ad_value(930), (p[1089] * p[1089]), (((p[1089] * p[1089])) + (((p[90] * p[90]) + ((2.0 * p[1089]) * p[90])))))), A::sqrt(A::offset(s.ad_value(930), 1.0))), p[90]), 1.0, 930, p[1089], p[1089]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1267] {s.store_primal_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p[90], A::scaled_offset(s.ad_value(930), 2.0, p[90]), 1.0);}
        if s.b[1267] {
            s.store_primal_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }
        if s.b[1267] {s.store_primal_mul(943, 925, 926);s.store_primal_sqrt_square_offset(944, 943, 1.0);s.store_add_mixed_ai(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p[90] * p[90]), s.ad_value(943), s.ad_value(928), (2.0 * p[90])), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p[90], A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);s.store_primal_mul_scale_offset_mixed_ai(945, A::offset(s.ad_value(944), 1.0), 943, p[90], 0.0);}
        if s.b[1267] {
            s.store_mul_div_scaled_inputs_mixed_aii(946, {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 943, ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85), 944, 1.0);
        }
        if s.b[1267] {s.store_scalar(627, 1.2e-12);s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p[43], 933, ((-0.5) * p[43]), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p[43]));s.store_add(1035, 938, 947);}
        s.b[1271] = (p[1090] > 0.0);s.store_scalar(1271, if s.b[1271] { 1.0 } else { 0.0 });
        if (s.b[1267] && s.b[1271]) {s.store_primal_offset_div_scaled_offset_numerator_indices(925, 448, 0.2, (p[90] * 0.2), 449, 1.0, 2.3);s.store_scalar(926, 1.05);s.store_primal_abs_ad(927, A::sub(A::offset(s.ad_value(448), p[90]), s.ad_value(449)));s.store_primal_scale(928, 926, p[1087]);s.store_primal_min_offset_rhs(929, 449, 448, p[90]);s.store_primal_div_from_scalar_offset_input(930, p[1087], 925, 1.0);s.store_scalar(931, 1700000000000.0);s.store_primal_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p[1087])));s.store_mul(933, 931, 932);}
        s.b[1272] = (s.v[933] > 80.0);s.store_scalar(1272, if s.b[1272] { 1.0 } else { 0.0 });
        if ((s.b[1267] && s.b[1271]) && s.b[1272]) {s.copy_ad(934, 932);}
        if ((s.b[1267] && s.b[1271]) && (!s.b[1272])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }
        if (s.b[1267] && s.b[1271]) {s.store_primal_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(449), 1.0, s.ad_value(448), p[90], 1.0), A::div_scaled_offset_numerator(s.ad_value(448), 1.0, p[90], s.ad_value(449), 1.0)), 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_20(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1267] && s.b[1271]) {s.store_primal_mul(936, 927, 935);}
        if (s.b[1267] && s.b[1271]) {
            s.store_primal_scale_ad(937, {
                if (!(((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p[1087]), 1.0 / (p[1087]))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }
        if (s.b[1267] && s.b[1271]) {s.store_scaled_add(938, 934, 937, p[40]);s.store_primal_div(930, 928, 448);s.store_primal_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);s.store_primal_add_mixed_ai(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(448), (2.0 * p[90]), (p[90] * p[90])), 1.0, A::square(s.ad_value(448)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p[90]), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);s.store_primal_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p[90], A::scaled_offset(s.ad_value(930), 2.0, p[90]), 1.0);}
        if (s.b[1267] && s.b[1271]) {
            s.store_primal_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }
        if (s.b[1267] && s.b[1271]) {s.store_primal_mul(943, 925, 926);s.store_primal_sqrt_square_offset(944, 943, 1.0);s.store_add_mixed_ai(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p[90] * p[90]), s.ad_value(943), s.ad_value(928), (2.0 * p[90])), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p[90], A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);s.store_primal_mul_scale_offset_mixed_ai(945, A::offset(s.ad_value(944), 1.0), 943, p[90], 0.0);}
        if (s.b[1267] && s.b[1271]) {
            s.store_mul_div_scaled_inputs_mixed_aii(946, {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 943, ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.7), 944, 1.0);
        }
        if (s.b[1267] && s.b[1271]) {s.store_scalar(627, 1.2e-12);s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p[40], 933, ((-0.5) * p[40]), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p[40]));s.store_add(1036, 938, 947);}
        if (s.b[1267] && (!s.b[1271])) {s.store_primal_offset_div_scaled_offset_numerator_indices(925, 448, 0.2, (p[90] * 0.2), 449, 1.0, 2.3);s.store_scalar(926, 1.05);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_21(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1267] && (!s.b[1271])) {s.store_primal_abs_ad(927, A::sub(A::offset(s.ad_value(448), p[90]), s.ad_value(449)));s.store_primal_scale(928, 926, p[1087]);s.store_primal_min_offset_rhs(929, 449, 448, p[90]);s.store_primal_div_from_scalar_offset_input(930, p[1087], 925, 1.0);s.store_scalar(931, 1700000000000.0);s.store_primal_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p[1087])));s.store_mul(933, 931, 932);}
        s.b[1273] = (s.v[933] > 80.0);s.store_scalar(1273, if s.b[1273] { 1.0 } else { 0.0 });
        if ((s.b[1267] && (!s.b[1271])) && s.b[1273]) {s.copy_ad(934, 932);}
        if ((s.b[1267] && (!s.b[1271])) && (!s.b[1273])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }
        if (s.b[1267] && (!s.b[1271])) {s.store_primal_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(449), 1.0, s.ad_value(448), p[90], 1.0), A::div_scaled_offset_numerator(s.ad_value(448), 1.0, p[90], s.ad_value(449), 1.0)), 0.5);s.store_primal_mul(936, 927, 935);}
        if (s.b[1267] && (!s.b[1271])) {
            s.store_primal_scale_ad(937, {
                if (!(((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p[1087]), 1.0 / (p[1087]))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }
        if (s.b[1267] && (!s.b[1271])) {s.store_scaled_add(938, 934, 937, p[40]);s.store_primal_div(930, 928, 448);s.store_primal_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);s.store_primal_add_mixed_ai(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(448), (2.0 * p[90]), (p[90] * p[90])), 1.0, A::square(s.ad_value(448)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p[90]), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);s.store_primal_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p[90], A::scaled_offset(s.ad_value(930), 2.0, p[90]), 1.0);}
        if (s.b[1267] && (!s.b[1271])) {
            s.store_primal_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }
        if (s.b[1267] && (!s.b[1271])) {s.store_primal_mul(943, 925, 926);s.store_primal_sqrt_square_offset(944, 943, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_22(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1267] && (!s.b[1271])) {s.store_add_mixed_ai(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p[90] * p[90]), s.ad_value(943), s.ad_value(928), (2.0 * p[90])), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p[90], A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);s.store_primal_mul_scale_offset_mixed_ai(945, A::offset(s.ad_value(944), 1.0), 943, p[90], 0.0);}
        if (s.b[1267] && (!s.b[1271])) {
            s.store_mul_div_scaled_inputs_mixed_aii(946, {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 943, ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85), 944, 1.0);
        }
        if (s.b[1267] && (!s.b[1271])) {s.store_scalar(627, 1.2e-12);s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p[40], 933, ((-0.5) * p[40]), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p[40]));s.store_add(1036, 938, 947);}
        if s.b[1267] {s.store_primal_offset_div_scaled_offset_numerator_indices(925, 448, 0.2, (p[90] * 0.2), 449, 1.0, 2.3);s.store_scalar(926, 1.05);s.store_primal_abs_ad(927, A::sub(A::offset(s.ad_value(448), p[90]), s.ad_value(449)));s.store_primal_scale(928, 926, p[1087]);s.store_primal_min_offset_rhs(929, 449, 448, p[90]);s.store_primal_div_from_scalar_offset_input(930, p[1087], 925, 1.0);s.store_scalar(931, 1700000000000.0);s.store_primal_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p[1087])));s.store_mul(933, 931, 932);}
        s.b[1274] = (s.v[933] > 80.0);s.store_scalar(1274, if s.b[1274] { 1.0 } else { 0.0 });
        if (s.b[1267] && s.b[1274]) {s.copy_ad(934, 932);}
        if (s.b[1267] && (!s.b[1274])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }
        if s.b[1267] {s.store_primal_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(449), 1.0, s.ad_value(448), p[90], 1.0), A::div_scaled_offset_numerator(s.ad_value(448), 1.0, p[90], s.ad_value(449), 1.0)), 0.5);s.store_primal_mul(936, 927, 935);}
        if s.b[1267] {
            s.store_primal_scale_ad(937, {
                if (!(((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p[1087]), 1.0 / (p[1087]))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }
        if s.b[1267] {s.store_scaled_add(938, 934, 937, p[40]);s.store_primal_div(930, 928, 448);s.store_primal_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_23(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1267] {s.store_primal_add_mixed_ai(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(448), (2.0 * p[90]), (p[90] * p[90])), 1.0, A::square(s.ad_value(448)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p[90]), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);s.store_primal_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p[90], A::scaled_offset(s.ad_value(930), 2.0, p[90]), 1.0);}
        if s.b[1267] {
            s.store_primal_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }
        if s.b[1267] {s.store_primal_mul(943, 925, 926);s.store_primal_sqrt_square_offset(944, 943, 1.0);s.store_add_mixed_ai(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p[90] * p[90]), s.ad_value(943), s.ad_value(928), (2.0 * p[90])), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p[90], A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);s.store_primal_mul_scale_offset_mixed_ai(945, A::offset(s.ad_value(944), 1.0), 943, p[90], 0.0);}
        if s.b[1267] {
            s.store_mul_div_scaled_inputs_mixed_aii(946, {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 943, ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85), 944, 1.0);
        }
        if s.b[1267] {s.store_scalar(627, 1.2e-12);s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p[40], 933, ((-0.5) * p[40]), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p[40]));s.store_add(1037, 938, 947);s.store_primal_offset_div_scaled_offset_numerator_indices(925, 448, 0.2, (p[90] * 0.2), 449, 1.0, 2.3);s.store_scalar(926, 1.05);s.store_primal_abs_ad(927, A::sub(A::offset(s.ad_value(448), p[90]), s.ad_value(449)));s.store_primal_scale(928, 926, p[1087]);s.store_primal_min_offset_rhs(929, 449, 448, p[90]);s.store_primal_div_from_scalar_offset_input(930, p[1087], 925, 1.0);s.store_scalar(931, 1700000000000.0);s.store_primal_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p[1087])));s.store_mul(933, 931, 932);}
        s.b[1275] = (s.v[933] > 80.0);s.store_scalar(1275, if s.b[1275] { 1.0 } else { 0.0 });
        if (s.b[1267] && s.b[1275]) {s.copy_ad(934, 932);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_24(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1267] && (!s.b[1275])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }
        if s.b[1267] {s.store_primal_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(449), 1.0, s.ad_value(448), p[90], 1.0), A::div_scaled_offset_numerator(s.ad_value(448), 1.0, p[90], s.ad_value(449), 1.0)), 0.5);s.store_primal_mul(936, 927, 935);}
        if s.b[1267] {
            s.store_primal_scale_ad(937, {
                if (!(((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p[1087]), 1.0 / (p[1087]))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }
        if s.b[1267] {s.store_scaled_add(938, 934, 937, p[42]);s.store_primal_div(930, 928, 448);s.store_primal_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);s.store_primal_add_mixed_ai(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(448), (2.0 * p[90]), (p[90] * p[90])), 1.0, A::square(s.ad_value(448)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p[90]), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);s.store_primal_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p[90], A::scaled_offset(s.ad_value(930), 2.0, p[90]), 1.0);}
        if s.b[1267] {
            s.store_primal_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }
        if s.b[1267] {s.store_primal_mul(943, 925, 926);s.store_primal_sqrt_square_offset(944, 943, 1.0);s.store_add_mixed_ai(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p[90] * p[90]), s.ad_value(943), s.ad_value(928), (2.0 * p[90])), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p[90], A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);s.store_primal_mul_scale_offset_mixed_ai(945, A::offset(s.ad_value(944), 1.0), 943, p[90], 0.0);}
    }
}
