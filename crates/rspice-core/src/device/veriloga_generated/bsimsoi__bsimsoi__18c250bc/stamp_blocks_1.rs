#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1159] {s.store_add_scaled_product_indices(845, 843, 1.0, 843, 843, 2.0);s.store_add_scaled_product_indices(1142, 193, 1.0, 192, 845, 1.0);s.copy_ad(49, 832);s.store_mul_div_from_scalar_lhs_ad_indices(847, 1.115, 832, 430);s.store_div_scaled_product_indices(850, 256, 847, 1.0, 300, 1.0);}
        s.b[1163] = (s.v[850] > 100.0);s.store_scalar(1163, if s.b[1163] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1163]) {s.store_scaled_offset(843, 850, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1164] = (s.v[850] < (-100.0));s.store_scalar(1164, if s.b[1164] { 1.0 } else { 0.0 });
        if ((s.b[1159] && (!s.b[1163])) && s.b[1164]) {s.store_scalar(843, 3.720075976e-44);}
        if ((s.b[1159] && (!s.b[1163])) && (!s.b[1164])) {s.store_exp(843, 850);}
        s.b[1165] = (s.v[256] == s.v[257]);s.store_scalar(1165, if s.b[1165] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1165]) {s.copy_ad(844, 843);}
        if (s.b[1159] && (!s.b[1165])) {s.store_div_scaled_product_indices(850, 257, 847, 1.0, 300, 1.0);}
        s.b[1166] = (s.v[850] > 100.0);s.store_scalar(1166, if s.b[1166] { 1.0 } else { 0.0 });
        if ((s.b[1159] && (!s.b[1165])) && s.b[1166]) {s.store_scaled_offset(844, 850, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1167] = (s.v[850] < (-100.0));s.store_scalar(1167, if s.b[1167] { 1.0 } else { 0.0 });
        if (((s.b[1159] && (!s.b[1165])) && (!s.b[1166])) && s.b[1167]) {s.store_scalar(844, 3.720075976e-44);}
        if (((s.b[1159] && (!s.b[1165])) && (!s.b[1166])) && (!s.b[1167])) {s.store_exp(844, 850);}
        if s.b[1159] {s.store_div_scaled_product_indices(850, 258, 847, 1.0, 302, 1.0);}
        s.b[1168] = (s.v[850] > 100.0);s.store_scalar(1168, if s.b[1168] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1168]) {s.store_scaled_offset(845, 850, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1169] = (s.v[850] < (-100.0));s.store_scalar(1169, if s.b[1169] { 1.0 } else { 0.0 });
        if ((s.b[1159] && (!s.b[1168])) && s.b[1169]) {s.store_scalar(845, 3.720075976e-44);}
        if ((s.b[1159] && (!s.b[1168])) && (!s.b[1169])) {s.store_exp(845, 850);}
        if s.b[1159] {s.store_mul(972, 355, 843);s.store_mul(949, 306, 843);s.store_mul(947, 308, 844);s.store_mul(951, 310, 845);s.store_mul(850, 259, 430);}
        s.b[1170] = (s.v[850] > 100.0);s.store_scalar(1170, if s.b[1170] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1170]) {s.store_scaled_offset(843, 850, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1171] = (s.v[850] < (-100.0));s.store_scalar(1171, if s.b[1171] { 1.0 } else { 0.0 });
        if ((s.b[1159] && (!s.b[1170])) && s.b[1171]) {s.store_scalar(843, 3.720075976e-44);}
        if ((s.b[1159] && (!s.b[1170])) && (!s.b[1171])) {s.store_exp(843, 850);}
        if s.b[1159] {s.store_mul(953, 312, 843);s.store_div_scaled_product_indices(850, 256, 847, 1.0, 301, 1.0);}
        s.b[1172] = (s.v[850] > 100.0);s.store_scalar(1172, if s.b[1172] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1172]) {s.store_scaled_offset(843, 850, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1173] = (s.v[850] < (-100.0));s.store_scalar(1173, if s.b[1173] { 1.0 } else { 0.0 });
        if ((s.b[1159] && (!s.b[1172])) && s.b[1173]) {s.store_scalar(843, 3.720075976e-44);}
        if ((s.b[1159] && (!s.b[1172])) && (!s.b[1173])) {s.store_exp(843, 850);}
        s.b[1174] = (s.v[256] == s.v[260]);s.store_scalar(1174, if s.b[1174] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1174]) {s.copy_ad(844, 843);}
        if (s.b[1159] && (!s.b[1174])) {s.store_div_scaled_product_indices(850, 260, 847, 1.0, 301, 1.0);}
        s.b[1175] = (s.v[850] > 100.0);s.store_scalar(1175, if s.b[1175] { 1.0 } else { 0.0 });
        if ((s.b[1159] && (!s.b[1174])) && s.b[1175]) {s.store_scaled_offset(844, 850, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1176] = (s.v[850] < (-100.0));s.store_scalar(1176, if s.b[1176] { 1.0 } else { 0.0 });
        if (((s.b[1159] && (!s.b[1174])) && (!s.b[1175])) && s.b[1176]) {s.store_scalar(844, 3.720075976e-44);}
        if (((s.b[1159] && (!s.b[1174])) && (!s.b[1175])) && (!s.b[1176])) {s.store_exp(844, 850);}
        if s.b[1159] {s.store_div_scaled_product_indices(850, 261, 847, 1.0, 303, 1.0);}
        s.b[1177] = (s.v[850] > 100.0);s.store_scalar(1177, if s.b[1177] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1177]) {s.store_scaled_offset(845, 850, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1178] = (s.v[850] < (-100.0));s.store_scalar(1178, if s.b[1178] { 1.0 } else { 0.0 });
        if ((s.b[1159] && (!s.b[1177])) && s.b[1178]) {s.store_scalar(845, 3.720075976e-44);}
        if ((s.b[1159] && (!s.b[1177])) && (!s.b[1178])) {s.store_exp(845, 850);}
        if s.b[1159] {s.store_mul(973, 356, 843);s.store_mul(950, 307, 843);s.store_mul(948, 309, 844);s.store_mul(952, 311, 845);s.store_mul(850, 262, 430);}
        s.b[1179] = (s.v[850] > 100.0);s.store_scalar(1179, if s.b[1179] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1179]) {s.store_scaled_offset(843, 850, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1180] = (s.v[850] < (-100.0));s.store_scalar(1180, if s.b[1180] { 1.0 } else { 0.0 });
        if ((s.b[1159] && (!s.b[1179])) && s.b[1180]) {s.store_scalar(843, 3.720075976e-44);}
        if ((s.b[1159] && (!s.b[1179])) && (!s.b[1180])) {s.store_exp(843, 850);}
        if s.b[1159] {s.store_mul(954, 313, 843);s.store_mul_pow_indices(945, 144, 411, 145);}
        s.b[1181] = (p.p38 < 4.2);s.store_scalar(1181, if s.b[1181] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1181]) {s.store_offset_mul_ad(961, s.ad_value(231), A::scale_offset(s.ad_value(411), p.p238, 1.0), 1e-9);}
        if (s.b[1159] && (!s.b[1181])) {s.store_offset_mul_ad(961, s.ad_value(231), A::scale_offset(s.ad_value(430), p.p238, 1.0), 1e-9);}
        if s.b[1159] {s.store_scale(850, 235, p.p235);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1159] {s.store_div(960, 850, 961);s.store_scale(847, 51, p.p235);s.store_div(959, 847, 961);s.store_offset(845, 959, 1.0);s.store_offset(850, 960, 1.0);s.store_div(843, 845, 850);s.store_mul(945, 945, 843);s.store_add_scaled_product_indices(946, 101, 1.0, 102, 430, (-1.0));s.store_offset_mul(845, 45, 959, 1.0);s.store_offset_mul(850, 45, 960, 1.0);s.store_div(843, 845, 850);s.store_mul(946, 946, 843);}
        s.b[1182] = (p.p429 != 1.0);s.store_scalar(1182, if s.b[1182] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1182]) {s.store_div_scaled_add_product_indices(955, 181, 1.0, 186, 430, 1.0, 159, 1.0);s.store_scalar(1095, 0.0);s.store_scalar(1096, 0.0);}
        if (s.b[1159] && (!s.b[1182])) {s.store_scalar(955, 0.0);s.store_scale(1094, 159, p.p3);s.store_mul(853, 186, 430);s.store_add(844, 169, 853);s.store_offset(845, 853, p.p140);s.store_div(1095, 844, 1094);s.store_div(1097, 845, 1094);s.store_add(850, 170, 853);s.store_offset(847, 853, p.p139);s.store_div(1096, 850, 1094);s.store_div(1098, 847, 1094);}
        if s.b[1159] {s.store_add_scaled_product_indices(956, 153, 1.0, 139, 430, 1.0);s.store_add_scaled_product_indices(957, 154, 1.0, 141, 430, 1.0);s.store_add_scaled_product_indices(958, 155, 1.0, 143, 430, 1.0);}
        if (!s.b[1159]) {s.copy_ad(940, 115);s.copy_ad(941, 160);s.copy_ad(942, 118);s.copy_ad(943, 339);s.copy_ad(944, 340);s.copy_ad(912, 395);s.copy_ad(1140, 367);s.copy_ad(1141, 342);s.copy_ad(1142, 343);s.copy_ad(949, 161);s.copy_ad(950, 162);s.copy_ad(947, 163);s.copy_ad(948, 164);s.copy_ad(951, 165);s.copy_ad(952, 166);s.copy_ad(953, 167);s.copy_ad(954, 168);s.copy_ad(972, 357);s.copy_ad(973, 358);s.copy_ad(945, 404);s.copy_ad(946, 407);s.copy_ad(956, 138);s.copy_ad(957, 140);s.copy_ad(958, 142);}
        s.b[1183] = (param_given[90] || param_given[94]);s.store_scalar(1183, if s.b[1183] { 1.0 } else { 0.0 });s.b[1184] = (!param_given[90]);s.store_scalar(1184, if s.b[1184] { 1.0 } else { 0.0 });
        if (s.b[1183] && s.b[1184]) {s.store_scalar(120, 0.53);}
        s.b[1185] = (!param_given[94]);s.store_scalar(1185, if s.b[1185] { 1.0 } else { 0.0 });
        if (s.b[1183] && s.b[1185]) {s.store_scalar(124, (-0.0186));}
        s.b[1186] = (!param_given[87]);s.store_scalar(1186, if s.b[1186] { 1.0 } else { 0.0 });
        if (((!s.b[1183]) && s.b[1186]) && (p.p41 != 0.0)) {s.store_scaled_div_from_scalar_ad(843, 1.602176462e-19, A::scale(s.ad_value(417), 2.0), 1000000.0);}
        if (((!s.b[1183]) && s.b[1186]) && (p.p41 == 0.0)) {s.store_scalar(843, 0.00077348);}
        if ((!s.b[1183]) && s.b[1186]) {s.store_add_scaled_product_indices(114, 942, 1.0, 843, 108, (-(s.v[117] * s.v[117])));}
        s.b[1187] = (s.v[114] > 0.0);s.store_scalar(1187, if s.b[1187] { 1.0 } else { 0.0 });
        if ((!s.b[1183]) && s.b[1187]) {s.store_neg(114, 114);}
        s.b[1188] = (s.v[116] > 0.0);s.store_scalar(1188, if s.b[1188] { 1.0 } else { 0.0 });
        if ((!s.b[1183]) && s.b[1188]) {s.store_primal_neg(116, 116);}
        s.b[1189] = (!param_given[85]);s.store_scalar(1189, if s.b[1189] { 1.0 } else { 0.0 });
        if ((!s.b[1183]) && s.b[1189]) {s.store_div_scaled_product_mixed_iai(112, 419, A::sqrt(s.ad_value(108)), 1.0, 396, 1.0);}
        s.b[1190] = (!param_given[86]);s.store_scalar(1190, if s.b[1190] { 1.0 } else { 0.0 });
        if ((!s.b[1183]) && s.b[1190]) {s.store_div_scaled_product_mixed_iai(113, 419, A::sqrt(s.ad_value(109)), 1.0, 396, 1.0);}
        if (!s.b[1183]) {s.store_sub(843, 112, 113);s.store_sub_mixed_ai(844, A::sqrt(A::sub(s.ad_value(942), s.ad_value(114))), 943);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_18(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (!s.b[1183]) {s.store_mul_sub_mixed_iai(845, 943, A::sqrt(A::sub(s.ad_value(942), s.ad_value(116))), 943);s.store_div_scaled_product_add_scaled_denominator_indices(846, 843, 844, 1.0, 845, 2.0, 116, 1.0, 1.0);s.store_add_scaled_inputs3_indices(402, 402, 1.0, 124, (-1.0), 846, 1.0);s.store_add_scaled_product_mixed_iia(120, 113, 1.0, 402, A::sqrt(A::sub(s.ad_value(942), s.ad_value(116))), (-2.0));}
        s.store_offset(843, 265, s.v[328]);s.b[1191] = (s.v[843] < 1e-8);s.store_scalar(1191, if s.b[1191] { 1.0 } else { 0.0 });
        if s.b[1191] {s.store_scalar(843, 1e-8);}
        s.store_mul_scale_offset_mixed_ia(405, 120, A::div(s.ad_value(264), s.ad_value(843)), 1.0, 1.0);s.store_scale(376, 405, (p.p66 * 1.0 / (p.p67)));s.store_scale(403, 402, (p.p66 * 1.0 / (p.p67)));s.b[1192] = (!param_given[109]);s.store_scalar(1192, if s.b[1192] { 1.0 } else { 0.0 });s.b[1193] = (param_given[108] || param_given[107]);s.store_scalar(1193, if s.b[1193] { 1.0 } else { 0.0 });
        if (s.b[1192] && s.b[1193]) {s.store_add_scaled_product_mixed_aii(406, A::add_scaled_inputs4(s.ad_value(406), 1.0, s.ad_value(152), (-1.0), s.ad_value(408), p.p37, s.ad_value(942), -1.0), 1.0, 405, 943, (-1.0));}
        if (s.b[1192] && (!s.b[1193])) {
        }
        s.b[1194] = (!param_given[108]);s.store_scalar(1194, if s.b[1194] { 1.0 } else { 0.0 });
        if s.b[1194] {s.store_add_scaled_inputs_product_indices(408, 406, p.p37, 942, p.p37, 405, 943, p.p37);}
        s.b[1195] = (p.p38 < 4.2);s.store_scalar(1195, if s.b[1195] { 1.0 } else { 0.0 });
        if s.b[1195] {s.copy_ad(1095, 173);s.copy_ad(1097, 171);s.copy_ad(1140, 367);s.copy_ad(1141, 342);s.copy_ad(1142, 343);}
        s.b[1196] = (p.p62 == 4.0);s.store_scalar(1196, if s.b[1196] { 1.0 } else { 0.0 });
        if (s.b[1195] && s.b[1196]) {s.copy_ad(956, 138);s.copy_ad(958, 142);}
        s.store_scaled_voltage(819, ctx, nodes, Some(7), Some(8), p.p37);s.store_scaled_voltage(818, ctx, nodes, Some(5), Some(8), p.p37);s.store_scaled_voltage(821, ctx, nodes, Some(9), Some(8), p.p37);s.store_scaled_voltage(897, ctx, nodes, Some(3), Some(8), p.p37);s.store_scaled_voltage(899, ctx, nodes, Some(5), Some(4), p.p37);s.store_scaled_voltage(1114, ctx, nodes, Some(9), Some(4), p.p37);s.store_scaled_voltage(1087, ctx, nodes, Some(11), Some(8), p.p37);s.store_scaled_voltage(1088, ctx, nodes, Some(12), Some(7), p.p37);s.store_scaled_voltage(1018, ctx, nodes, Some(10), Some(8), p.p37);s.store_sub(817, 818, 819);s.store_sub(820, 821, 819);s.store_sub(898, 897, 819);s.store_sub(1019, 1018, 819);s.b[1197] = (s.v[819] >= 0.0);s.store_scalar(1197, if s.b[1197] { 1.0 } else { 0.0 });
        if s.b[1197] {s.store_scalar(398, 1.0);s.copy_ad(822, 819);s.copy_ad(823, 821);s.copy_ad(824, 818);s.copy_ad(900, 817);s.copy_ad(901, 897);s.copy_ad(1110, 820);s.store_scalar(995, s.v[347]);s.store_scalar(996, s.v[348]);s.copy_ad(1143, 282);s.store_add_scaled_product_indices(1144, 283, 1.0, 284, 430, 1.0);s.copy_ad(1145, 285);s.copy_ad(1146, 286);s.copy_ad(1147, 287);s.copy_ad(1148, 288);s.copy_ad(1149, 289);s.copy_ad(1150, 290);s.store_add_scaled_product_indices(1151, 291, 1.0, 292, 430, 1.0);s.copy_ad(1152, 293);s.copy_ad(1153, 294);s.copy_ad(1154, 295);s.copy_ad(1155, 296);s.copy_ad(1156, 297);}
        if (!s.b[1197]) {s.store_scalar(398, (-1.0));s.store_neg(822, 819);s.copy_ad(823, 820);s.copy_ad(824, 817);s.copy_ad(900, 818);s.copy_ad(901, 898);s.copy_ad(1110, 821);s.store_scalar(995, s.v[348]);s.store_scalar(996, s.v[347]);s.copy_ad(1143, 290);s.store_add_scaled_product_indices(1144, 291, 1.0, 292, 430, 1.0);s.copy_ad(1145, 293);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[1197]) {s.copy_ad(1146, 294);s.copy_ad(1147, 295);s.copy_ad(1148, 296);s.copy_ad(1149, 297);s.copy_ad(1150, 282);s.store_add_scaled_product_indices(1151, 283, 1.0, 284, 430, 1.0);s.copy_ad(1152, 285);s.copy_ad(1153, 286);s.copy_ad(1154, 287);s.copy_ad(1155, 288);s.copy_ad(1156, 289);}
        s.store_sub(902, 901, 941);s.store_scalar(913, s.v[392]);s.store_add(843, 406, 942);s.b[1198] = (p.p41 == 0.0);s.store_scalar(1198, if s.b[1198] { 1.0 } else { 0.0 });
        if s.b[1198] {s.copy_ad(418, 417);}
        if (!s.b[1198]) {s.store_scalar(418, (p.p60 * 8.85418e-12));}
        s.b[1199] = ((((s.v[110] > 1e18) && (s.v[110] < 1e25)) && (s.v[823] > s.v[843])) && (s.v[418] != 0.0));s.store_scalar(1199, if s.b[1199] { 1.0 } else { 0.0 });
        if s.b[1199] {s.store_div_scaled_product_mixed_iia(844, 418, 110, (1000000.0 * 1.602176462e-19), A::square(s.ad_value(396)), 1.0);s.store_sqrt_offset_ad(847, A::div_scaled_inputs2(s.ad_value(823), 2.0, s.ad_value(843), (-2.0), s.ad_value(844), 1.0), 1.0);s.store_mul_scale_offset_indices(845, 844, 847, 1.0, (-1.0));s.store_div_scaled_product_indices(846, 845, 845, 0.5, 844, 1.0);s.store_offset_sub_from_scalar_ad(850, p.p1034, s.ad_value(846), (-0.05));s.store_sqrt_square_offset(849, 850, 0.224);s.store_offset_add_scaled_inputs_indices(848, 850, (-0.5), 849, (-0.5), p.p1034);s.store_sub(825, 823, 848);}
        if (!s.b[1199]) {s.copy_ad(825, 823);}
        s.b[1200] = ((((s.v[110] > 1e18) && (s.v[110] < 1e25)) && (s.v[1110] > s.v[843])) && (s.v[418] != 0.0));s.store_scalar(1200, if s.b[1200] { 1.0 } else { 0.0 });
        if s.b[1200] {s.store_div_scaled_product_mixed_iia(844, 418, 110, (1000000.0 * 1.602176462e-19), A::square(s.ad_value(396)), 1.0);s.store_sqrt_offset_ad(847, A::div_scaled_inputs2(s.ad_value(1110), 2.0, s.ad_value(843), (-2.0), s.ad_value(844), 1.0), 1.0);s.store_mul_scale_offset_indices(845, 844, 847, 1.0, (-1.0));s.store_div_scaled_product_indices(846, 845, 845, 0.5, 844, 1.0);s.store_offset_sub_from_scalar_ad(850, p.p1034, s.ad_value(846), (-0.05));s.store_sqrt_square_offset(849, 850, 0.224);s.store_offset_add_scaled_inputs_indices(848, 850, (-0.5), 849, (-0.5), p.p1034);s.store_sub(1111, 1110, 848);}
        if (!s.b[1200]) {s.copy_ad(1111, 1110);}
        s.copy_ad(1125, 823);s.store_scalar(892, s.v[327]);s.b[1201] = ((p.p36 == 1.0) && (p.p14 != 0.0));s.store_scalar(1201, if s.b[1201] { 1.0 } else { 0.0 });
        if s.b[1201] {s.store_scale(832, 409, 8.617087e-5);}
        if (!s.b[1201]) {s.copy_ad(832, 49);}
        s.store_sub(834, 940, 942);s.b[1202] = (s.v[37] == 0.0);s.store_scalar(1202, if s.b[1202] { 1.0 } else { 0.0 });
        if s.b[1202] {s.copy_ad(1033, 824);s.copy_ad(1048, 824);}
        s.b[1203] = (p.p432 == 0.0);s.store_scalar(1203, if s.b[1203] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1203]) {s.store_div_scaled_inputs_indices(843, 225, (-s.v[327]), 119, 1.0);s.store_mul_add_scaled_inputs_rhs(844, 224, A::exp_scaled_input(s.ad_value(843), 0.5), 1.0, A::exp(s.ad_value(843)), 2.0);s.store_mul_sub_rhs(845, 844, 940, 942);s.store_div_scaled_inputs_indices(846, 344, 0.5, 393, 1.0);s.store_add_scaled_inputs4_indices(1036, 942, 1.0, 846, (-1.0), 216, 1.0, 845, 1.0);s.store_offset_scaled(843, 393, 1.0 / (s.v[913]), 1.0);s.store_div_scaled_inputs_indices(846, 223, (-s.v[327]), 119, 1.0);s.store_mul_add_scaled_inputs_rhs(848, 222, A::exp_scaled_input(s.ad_value(846), 0.5), 1.0, A::exp(s.ad_value(846)), 2.0);s.store_div_scaled_inputs2_indices(844, 221, 1.0, 848, (-1.0), 843, 1.0);s.store_mul(845, 844, 902);s.store_div_from_scalar_offset_ad(847, 1.0, A::div_from_scalar(s.v[913], s.ad_value(393)), 1.0);s.store_add_scaled_product_indices(1031, 845, 1.0, 847, 1036, 1.0);}
        if ((!s.b[1202]) && (!s.b[1203])) {s.store_div_from_scalar_add_ad(843, 1.0, A::offset(s.ad_value(393), s.v[913]), s.ad_value(218));s.store_div_scaled_inputs_indices(844, 225, (-s.v[327]), 119, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
    ) {
        if ((!s.b[1202]) && (!s.b[1203])) {s.store_mul_add_scaled_inputs_rhs(845, 224, A::exp_scaled_input(s.ad_value(844), 0.5), 1.0, A::exp(s.ad_value(844)), 2.0);s.store_mul_add_rhs(846, 845, 822, 217);s.store_div_scaled_inputs_indices(847, 344, 0.5, 393, 1.0);s.store_mul_ad_product_rhs_mixed_ia(848, 393, 843, A::add_scaled_inputs3(s.ad_value(942), 1.0, s.ad_value(847), (-1.0), s.ad_value(216), 1.0));s.store_mul3_lhs(849, 218, 843, 846);s.store_add(1036, 848, 849);s.store_scaled_mul(850, 843, 902, s.v[913]);s.store_add(1031, 1036, 850);}
        if (!s.b[1202]) {s.store_offset_sub(844, 1036, 1031, (-0.005));s.store_sqrt_square_offset(845, 844, 2.5e-5);s.store_scaled_add(846, 844, 845, 0.5);s.store_div_scaled_product_indices(847, 846, 393, 1.0, 344, 1.0);s.store_add_scaled_product_indices(1032, 1031, 1.0, 846, 847, (-0.5));s.store_offset(844, 942, (-0.02));s.store_offset_sub(845, 844, 1032, (-0.005));s.store_sqrt_square_offset(846, 845, (4.0 * 0.005));s.store_add_scaled_inputs3_indices(1032, 844, 1.0, 845, (-0.5), 846, (-0.5));s.store_sub(827, 942, 1032);s.store_sqrt(828, 827);s.store_div_scaled_product_indices(864, 944, 828, 1.0, 943, 1.0);s.store_sqrt(846, 864);s.store_mul(843, 131, 1032);}
        s.b[1204] = (s.v[843] >= (-0.5));s.store_scalar(1204, if s.b[1204] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1204]) {s.store_offset(844, 843, 1.0);}
        if ((!s.b[1202]) && (!s.b[1204])) {s.store_div_from_scalar_offset_scaled_input(847, 1.0, 843, 8.0, 3.0);s.store_mul_scale_offset_rhs(844, 847, 843, 3.0, 1.0);}
        if (!s.b[1202]) {s.store_mul3_lhs(865, 397, 846, 844);s.store_mul(843, 134, 1032);}
        s.b[1205] = (s.v[843] >= (-0.5));s.store_scalar(1205, if s.b[1205] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1205]) {s.store_offset(844, 843, 1.0);}
        if ((!s.b[1202]) && (!s.b[1205])) {s.store_div_from_scalar_offset_scaled_input(847, 1.0, 843, 8.0, 3.0);s.store_mul_scale_offset_rhs(844, 847, 843, 3.0, 1.0);}
        if (!s.b[1202]) {s.store_mul3_lhs(866, 397, 846, 844);s.store_div_scaled_inputs_indices(843, 130, ((-0.5) * s.v[892]), 865, 1.0);}
        s.b[1206] = (s.v[843] > (-100.0));s.store_scalar(1206, if s.b[1206] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1206]) {s.store_exp(844, 843);s.store_mul_scale_offset_rhs(868, 844, 844, 2.0, 1.0);}
        if ((!s.b[1202]) && (!s.b[1206])) {s.store_scalar(844, 3.720075976e-44);s.store_mul_scale_offset_rhs(868, 844, 844, 2.0, 1.0);}
        if (!s.b[1202]) {s.store_div_scaled_product_indices(845, 100, 417, 1.0, 864, 1.0);s.store_add_scaled_value_products_indices(846, 96, 1.0, 97, 1032, 1.0, 98, 822, 1.0);s.store_div_scaled_inputs2_mixed_aii(847, A::add_scaled_product(s.ad_value(845), 1.0, s.ad_value(846), s.ad_value(868), 1.0), 1.0, 99, 1.0, 396, 1.0);}
        s.b[1207] = (s.v[847] >= (-0.5));s.store_scalar(1207, if s.b[1207] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1207]) {s.store_offset(831, 847, 1.0);}
        if ((!s.b[1202]) && (!s.b[1207])) {s.store_div_from_scalar_offset_scaled_input(843, 1.0, 847, 8.0, 3.0);s.store_mul_scale_offset_rhs(831, 843, 847, 3.0, 1.0);}
        s.b[1208] = (s.v[378] > 0.0);s.store_scalar(1208, if s.b[1208] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1208]) {s.store_mul_scale_offset_indices(843, 822, 379, -1.0, 0.0);}
        s.b[1209] = (s.v[843] < (-100.0));s.store_scalar(1209, if s.b[1209] { 1.0 } else { 0.0 });
        if (((!s.b[1202]) && s.b[1208]) && s.b[1209]) {s.store_scalar(845, 3.720075976e-44);}
        if (((!s.b[1202]) && s.b[1208]) && (!s.b[1209])) {s.store_exp(845, 843);}
        if ((!s.b[1202]) && s.b[1208]) {s.store_offset_mul_offset_rhs(846, 378, 845, 1.0, s.v[892]);}
        if ((!s.b[1202]) && s.b[1208]) {
            s.store_mul_mixed_ia(847, 832, {
                            if ((s.v[892] / s.v[846]) > 1e-38) {
                                A::ln(A::div_from_scalar(s.v[892], s.ad_value(846)))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if ((!s.b[1202]) && s.b[1208]) {s.store_mul(1090, 831, 847);}
        if ((!s.b[1202]) && (!s.b[1208])) {s.store_scalar(1090, 0.0);}
        if (!s.b[1202]) {s.store_mul(63, 129, 868);s.store_mul(867, 63, 834);s.store_div_scaled_inputs_indices(843, 133, ((-0.5) * (s.v[328] * s.v[892])), 866, 1.0);}
        s.b[1210] = (s.v[843] > (-100.0));s.store_scalar(1210, if s.b[1210] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1210]) {s.store_exp(844, 843);s.store_mul_scale_offset_rhs(845, 844, 844, 2.0, 1.0);}
        if ((!s.b[1202]) && (!s.b[1210])) {s.store_scalar(844, 3.720075976e-44);s.store_mul_scale_offset_rhs(845, 844, 844, 2.0, 1.0);}
        if (!s.b[1202]) {s.store_mul(843, 132, 845);s.store_mul(904, 843, 834);s.store_sqrt_offset_scaled_input(843, 128, 1.0 / (s.v[892]), 1.0);s.store_add_scaled_inputs_product_indices(844, 121, 1.0, 122, 1.0 / (s.v[892]), 123, 1032, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[1202]) {s.store_add_scaled_product_mixed_aii(903, A::mul3(s.ad_value(376), A::offset(s.ad_value(843), (-1.0)), s.ad_value(943)), 1.0, 844, 430, 1.0);s.store_div_scaled_product_offset_denominator_indices(870, 415, 942, 1.0, 127, s.v[328], 1.0);s.store_add_scaled_product_indices(846, 400, 1.0, 188, 1032, 1.0);}
        s.b[1211] = (s.v[846] < 0.0001);s.store_scalar(1211, if s.b[1211] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1211]) {s.store_div_from_scalar_sub_from_scalar_ad(852, 1.0, 3.0, A::scale(s.ad_value(846), 20000.0));s.store_mul_scale_offset_indices(846, 852, 846, -1.0, 0.0002);}
        if (!s.b[1202]) {s.store_mul3_lhs(873, 846, 1141, 822);s.store_add_scaled_product_indices(846, 401, 1.0, 190, 1032, 1.0);}
        s.b[1212] = (s.v[846] < 0.0001);s.store_scalar(1212, if s.b[1212] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1212]) {s.store_div_from_scalar_sub_from_scalar_ad(852, 1.0, 3.0, A::scale(s.ad_value(846), 20000.0));s.store_mul_scale_offset_indices(846, 852, 846, -1.0, 0.0002);}
        if (!s.b[1202]) {s.store_mul3_lhs(1070, 846, 1141, 822);s.store_sqrt_offset_scaled_input(1089, 377, 1.0 / (s.v[892]), 1.0);s.store_exp_mul_scaled_lhs_indices(843, 382, 2.0, 822);s.store_div_scaled_product_offset_denominator_mixed_iai(1091, 391, A::offset(s.ad_value(843), (-1.0)), 1.0, 843, 1.0, 1.0);s.store_sub_add_scaled_inputs4_lhs_mixed_aiii(1037, A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_value_products(s.ad_value(408), p.p37, A::add_scaled_products(s.ad_value(376), s.ad_value(828), 1.0, s.ad_value(405), s.ad_value(943), (-1.0)), s.ad_value(1089), 1.0, s.ad_value(403), s.ad_value(1032), (-1.0)), 1.0, s.ad_value(867), (-1.0), s.ad_value(904), -1.0), 1.0, A::add_scaled_product(s.ad_value(125), 1.0, s.ad_value(126), s.ad_value(1032), 1.0), s.ad_value(870), 1.0), 1.0, 903, 1.0, 873, -1.0, 1090, -1.0, 1091);s.store_sub_add_scaled_inputs4_lhs_mixed_aiii(1052, A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_value_products(s.ad_value(408), p.p37, A::add_scaled_products(s.ad_value(376), s.ad_value(828), 1.0, s.ad_value(405), s.ad_value(943), (-1.0)), s.ad_value(1089), 1.0, s.ad_value(403), s.ad_value(1032), (-1.0)), 1.0, s.ad_value(867), (-1.0), s.ad_value(904), -1.0), 1.0, A::add_scaled_product(s.ad_value(125), 1.0, s.ad_value(126), s.ad_value(1032), 1.0), s.ad_value(870), 1.0), 1.0, 903, 1.0, 1070, -1.0, 1090, -1.0, 1091);s.store_sub(1038, 1037, 825);s.store_mul(853, 219, 832);}
        s.b[1213] = (((s.v[1038] - s.v[220]) / s.v[853]) > 100.0);s.store_scalar(1213, if s.b[1213] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1213]) {s.store_scaled_offset_ad(1039, A::div_scaled_inputs2(s.ad_value(1038), 1.0, s.ad_value(220), (-1.0), s.ad_value(853), 1.0), ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1214] = (((s.v[1038] - s.v[220]) / s.v[853]) < (-100.0));s.store_scalar(1214, if s.b[1214] { 1.0 } else { 0.0 });
        if (((!s.b[1202]) && (!s.b[1213])) && s.b[1214]) {s.store_scalar(1039, 3.720075976e-44);}
        if (((!s.b[1202]) && (!s.b[1213])) && (!s.b[1214])) {s.store_exp_ad(1039, A::div_scaled_inputs2(s.ad_value(1038), 1.0, s.ad_value(220), (-1.0), s.ad_value(853), 1.0));}
        if (!s.b[1202]) {s.store_mul_ln_mixed_ia(1042, 853, A::offset(s.ad_value(1039), 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[1202]) {s.store_sub(1040, 825, 1037);}
        s.b[1215] = (((s.v[1040] - s.v[220]) / s.v[853]) > 100.0);s.store_scalar(1215, if s.b[1215] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1215]) {s.store_scaled_offset_ad(1041, A::div_scaled_inputs2(s.ad_value(1040), 1.0, s.ad_value(220), (-1.0), s.ad_value(853), 1.0), ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1216] = (((s.v[1040] - s.v[220]) / s.v[853]) < (-100.0));s.store_scalar(1216, if s.b[1216] { 1.0 } else { 0.0 });
        if (((!s.b[1202]) && (!s.b[1215])) && s.b[1216]) {s.store_scalar(1041, 3.720075976e-44);}
        if (((!s.b[1202]) && (!s.b[1215])) && (!s.b[1216])) {s.store_exp_ad(1041, A::div_scaled_inputs2(s.ad_value(1040), 1.0, s.ad_value(220), (-1.0), s.ad_value(853), 1.0));}
        if (!s.b[1202]) {s.store_mul_ln_mixed_ia(1043, 853, A::offset(s.ad_value(1041), 1.0));s.store_mul_product3_indices(844, 832, 226, 376, 832, 1.0);s.store_add_scaled_product_mixed_iia(845, 1043, 1.0, 405, A::sqrt(s.ad_value(942)), 2.0);s.store_offset_div_scaled_product_indices(843, 1043, 845, 1.0, 844, 1.0, 1.0);}
        if (!s.b[1202]) {
            s.store_add_scaled_product_mixed_iia(1034, 942, 1.0, 832, {
                if (s.v[843] > 1e-38) {
                    A::ln(s.ad_value(843))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 1.0);
        }
        if (!s.b[1202]) {s.store_div_add_scaled_inputs_rhs_mixed_ia(843, 396, 396, 1.0, A::div_scalar_offset_denominator(1.0, A::div_from_scalar(1.0, s.ad_value(393)), (1.0 / s.v[913]), 1.0), 1.0);s.store_add_scaled_product_indices(1035, 1034, 1.0, 843, 1042, (-1.0));}
        s.b[1217] = (p.p432 == 0.0);s.store_scalar(1217, if s.b[1217] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1217]) {s.store_div_scaled_inputs_indices(843, 225, (-s.v[327]), 119, 1.0);s.store_mul_add_scaled_inputs_rhs(844, 224, A::exp_scaled_input(s.ad_value(843), 0.5), 1.0, A::exp(s.ad_value(843)), 2.0);s.store_mul_sub_rhs(845, 844, 940, 942);s.store_div_scaled_inputs_indices(846, 344, 0.5, 393, 1.0);s.store_add_scaled_inputs4_indices(1036, 1035, 1.0, 846, (-1.0), 216, 1.0, 845, 1.0);s.store_offset_scaled(843, 393, 1.0 / (s.v[913]), 1.0);s.store_div_scaled_inputs_indices(846, 223, (-s.v[327]), 119, 1.0);s.store_mul_add_scaled_inputs_rhs(848, 222, A::exp_scaled_input(s.ad_value(846), 0.5), 1.0, A::exp(s.ad_value(846)), 2.0);s.store_div_scaled_inputs2_indices(844, 221, 1.0, 848, (-1.0), 843, 1.0);s.store_mul(845, 844, 902);s.store_div_from_scalar_offset_ad(843, 1.0, A::div_from_scalar(s.v[913], s.ad_value(393)), 1.0);s.store_add_scaled_product_indices(1031, 845, 1.0, 843, 1036, 1.0);}
        if ((!s.b[1202]) && (!s.b[1217])) {s.store_div_from_scalar_add_ad(843, 1.0, A::offset(s.ad_value(393), s.v[913]), s.ad_value(218));s.store_div_scaled_inputs_indices(844, 225, (-s.v[327]), 119, 1.0);s.store_mul_add_scaled_inputs_rhs(845, 224, A::exp_scaled_input(s.ad_value(844), 0.5), 1.0, A::exp(s.ad_value(844)), 2.0);s.store_mul_add_rhs(846, 845, 822, 217);s.store_div_scaled_inputs_indices(847, 344, 0.5, 393, 1.0);s.store_mul_ad_product_rhs_mixed_ia(848, 393, 843, A::add_scaled_inputs3(s.ad_value(1035), 1.0, s.ad_value(847), (-1.0), s.ad_value(216), 1.0));s.store_mul3_lhs(849, 218, 843, 846);s.store_add(1036, 848, 849);s.store_scaled_mul(850, 843, 902, s.v[913]);s.store_add(1031, 1036, 850);}
        s.b[1218] = (s.v[37] == 2.0);s.store_scalar(1218, if s.b[1218] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1218]) {s.store_offset(1030, 1031, 0.02);s.store_offset(824, 1031, 0.02);}
        if ((!s.b[1202]) && (!s.b[1218])) {s.store_offset_sub_ad(844, s.ad_value(824), A::offset(s.ad_value(1031), 0.02), (-0.01));s.store_sqrt_square_offset(845, 844, 0.0001);s.store_add_scaled_inputs3_offset_indices(1030, 1031, 1.0, 844, 0.5, 845, 0.5, 0.02);}
        if (!s.b[1202]) {s.store_offset_sub(844, 1036, 1030, (-0.005));s.store_sqrt_square_offset(845, 844, 2.5e-5);s.store_scaled_add(846, 844, 845, 0.5);s.store_div_scaled_product_indices(847, 846, 393, 1.0, 344, 1.0);s.store_add_scaled_product_indices(1033, 1030, 1.0, 846, 847, (-0.5));s.store_sub(1060, 1052, 825);s.store_mul(853, 219, 832);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1219] = (((s.v[1060] - s.v[220]) / s.v[853]) > 100.0);s.store_scalar(1219, if s.b[1219] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1219]) {s.store_scaled_offset_ad(1061, A::div_scaled_inputs2(s.ad_value(1060), 1.0, s.ad_value(220), (-1.0), s.ad_value(853), 1.0), ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1220] = (((s.v[1060] - s.v[220]) / s.v[853]) < (-100.0));s.store_scalar(1220, if s.b[1220] { 1.0 } else { 0.0 });
        if (((!s.b[1202]) && (!s.b[1219])) && s.b[1220]) {s.store_scalar(1061, 3.720075976e-44);}
        if (((!s.b[1202]) && (!s.b[1219])) && (!s.b[1220])) {s.store_exp_ad(1061, A::div_scaled_inputs2(s.ad_value(1060), 1.0, s.ad_value(220), (-1.0), s.ad_value(853), 1.0));}
        if (!s.b[1202]) {s.store_mul_ln_mixed_ia(1064, 853, A::offset(s.ad_value(1061), 1.0));s.store_sub(1062, 825, 1052);}
        s.b[1221] = (((s.v[1062] - s.v[220]) / s.v[853]) > 100.0);s.store_scalar(1221, if s.b[1221] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1221]) {s.store_scaled_offset_ad(1063, A::div_scaled_inputs2(s.ad_value(1062), 1.0, s.ad_value(220), (-1.0), s.ad_value(853), 1.0), ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1222] = (((s.v[1062] - s.v[220]) / s.v[853]) < (-100.0));s.store_scalar(1222, if s.b[1222] { 1.0 } else { 0.0 });
        if (((!s.b[1202]) && (!s.b[1221])) && s.b[1222]) {s.store_scalar(1063, 3.720075976e-44);}
        if (((!s.b[1202]) && (!s.b[1221])) && (!s.b[1222])) {s.store_exp_ad(1063, A::div_scaled_inputs2(s.ad_value(1062), 1.0, s.ad_value(220), (-1.0), s.ad_value(853), 1.0));}
        if (!s.b[1202]) {s.store_mul_ln_mixed_ia(1065, 853, A::offset(s.ad_value(1063), 1.0));s.store_mul_product3_indices(844, 832, 226, 376, 832, 1.0);s.store_add_scaled_product_mixed_iia(845, 1065, 1.0, 405, A::sqrt(s.ad_value(942)), 2.0);s.store_offset_div_scaled_product_indices(843, 1065, 845, 1.0, 844, 1.0, 1.0);}
        if (!s.b[1202]) {
            s.store_add_scaled_product_mixed_iia(1049, 942, 1.0, 832, {
                if (s.v[843] > 1e-38) {
                    A::ln(s.ad_value(843))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 1.0);
        }
        if (!s.b[1202]) {s.store_div_add_scaled_inputs_rhs_mixed_ia(843, 396, 396, 1.0, A::div_scalar_offset_denominator(1.0, A::div_from_scalar(1.0, s.ad_value(393)), (1.0 / s.v[913]), 1.0), 1.0);s.store_add_scaled_product_indices(1050, 1049, 1.0, 843, 1064, (-1.0));}
        s.b[1223] = (p.p432 == 0.0);s.store_scalar(1223, if s.b[1223] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1223]) {s.store_div_scaled_inputs_indices(843, 225, (-s.v[327]), 119, 1.0);s.store_mul_add_scaled_inputs_rhs(844, 224, A::exp_scaled_input(s.ad_value(843), 0.5), 1.0, A::exp(s.ad_value(843)), 2.0);s.store_mul_sub_rhs(845, 844, 940, 942);s.store_div_scaled_inputs_indices(846, 344, 0.5, 393, 1.0);s.store_add_scaled_inputs4_indices(1051, 1050, 1.0, 846, (-1.0), 216, 1.0, 845, 1.0);s.store_offset_scaled(843, 393, 1.0 / (s.v[913]), 1.0);s.store_div_scaled_inputs_indices(846, 223, (-s.v[327]), 119, 1.0);s.store_mul_add_scaled_inputs_rhs(848, 222, A::exp_scaled_input(s.ad_value(846), 0.5), 1.0, A::exp(s.ad_value(846)), 2.0);s.store_div_scaled_inputs2_indices(844, 221, 1.0, 848, (-1.0), 843, 1.0);s.store_mul(845, 844, 902);s.store_div_from_scalar_offset_ad(843, 1.0, A::div_from_scalar(s.v[913], s.ad_value(393)), 1.0);s.store_add_scaled_product_indices(1047, 845, 1.0, 843, 1051, 1.0);}
        if ((!s.b[1202]) && (!s.b[1223])) {s.store_div_from_scalar_add_ad(843, 1.0, A::offset(s.ad_value(393), s.v[913]), s.ad_value(218));s.store_div_scaled_inputs_indices(844, 225, (-s.v[327]), 119, 1.0);s.store_mul_add_scaled_inputs_rhs(845, 224, A::exp_scaled_input(s.ad_value(844), 0.5), 1.0, A::exp(s.ad_value(844)), 2.0);s.store_mul_add_rhs(846, 845, 822, 217);s.store_div_scaled_inputs_indices(847, 344, 0.5, 393, 1.0);s.store_mul_ad_product_rhs_mixed_ia(848, 393, 843, A::add_scaled_inputs3(s.ad_value(1050), 1.0, s.ad_value(847), (-1.0), s.ad_value(216), 1.0));s.store_mul3_lhs(849, 218, 843, 846);s.store_add(1051, 848, 849);s.store_scaled_mul(850, 843, 902, s.v[913]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
    ) {
        if ((!s.b[1202]) && (!s.b[1223])) {s.store_add(1047, 1051, 850);}
        s.b[1224] = (s.v[37] == 2.0);s.store_scalar(1224, if s.b[1224] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1224]) {s.store_offset(1046, 1047, 0.02);s.store_offset(824, 1047, 0.02);}
        if ((!s.b[1202]) && (!s.b[1224])) {s.store_offset_sub_ad(844, s.ad_value(824), A::offset(s.ad_value(1047), 0.02), (-0.01));s.store_sqrt_square_offset(845, 844, 0.0001);s.store_add_scaled_inputs3_offset_indices(1046, 1047, 1.0, 844, 0.5, 845, 0.5, 0.02);}
        if (!s.b[1202]) {s.store_offset_sub(844, 1051, 1046, (-0.005));s.store_sqrt_square_offset(845, 844, 2.5e-5);s.store_scaled_add(846, 844, 845, 0.5);s.store_div_scaled_product_indices(847, 846, 393, 1.0, 344, 1.0);s.store_add_scaled_product_indices(1048, 1046, 1.0, 846, 847, (-0.5));}
        s.store_offset(843, 1033, ((5.0) + ((-0.001))));s.store_sqrt_square_offset(844, 843, (-(0.004 * (-5.0))));s.store_offset_add_scaled_inputs_indices(845, 843, 0.5, 844, 0.5, (-5.0));s.store_scalar(843, 1.5);s.store_offset_sub_from_scalar_ad(844, s.v[843], s.ad_value(845), (-0.002));s.store_sqrt_square_offset(846, 844, (0.008 * s.v[843]));s.store_offset_add_scaled_inputs_indices(962, 844, (-0.5), 846, (-0.5), s.v[843]);s.store_scale(843, 942, 0.95);s.store_offset_sub(844, 843, 962, (-0.002));s.store_sqrt_add_scaled_square_input(845, 844, 1.0, 843, 0.008);s.store_add_scaled_inputs3_indices(841, 843, 1.0, 844, (-0.5), 845, (-0.5));s.store_offset(843, 1048, ((5.0) + ((-0.001))));s.store_sqrt_square_offset(844, 843, (-(0.004 * (-5.0))));s.store_offset_add_scaled_inputs_indices(845, 843, 0.5, 844, 0.5, (-5.0));s.store_scalar(843, 1.5);s.store_offset_sub_from_scalar_ad(844, s.v[843], s.ad_value(845), (-0.002));s.store_sqrt_square_offset(846, 844, (0.008 * s.v[843]));s.store_offset_add_scaled_inputs_indices(1045, 844, (-0.5), 846, (-0.5), s.v[843]);s.store_scale(843, 942, 0.95);s.store_offset_sub(844, 843, 1045, (-0.002));s.store_sqrt_add_scaled_square_input(845, 844, 1.0, 843, 0.008);s.store_add_scaled_inputs3_indices(1044, 843, 1.0, 844, (-0.5), 845, (-0.5));s.store_sub(827, 942, 841);s.store_sqrt(828, 827);s.store_div_scaled_product_indices(864, 944, 828, 1.0, 943, 1.0);s.store_sqrt(846, 864);s.store_mul(843, 131, 841);s.b[1225] = (s.v[843] >= (-0.5));s.store_scalar(1225, if s.b[1225] { 1.0 } else { 0.0 });
        if s.b[1225] {s.store_offset(844, 843, 1.0);}
        if (!s.b[1225]) {s.store_div_from_scalar_offset_scaled_input(847, 1.0, 843, 8.0, 3.0);s.store_mul_scale_offset_rhs(844, 847, 843, 3.0, 1.0);}
        s.store_mul3_lhs(865, 397, 846, 844);s.store_mul(843, 134, 841);s.b[1226] = (s.v[843] >= (-0.5));s.store_scalar(1226, if s.b[1226] { 1.0 } else { 0.0 });
        if s.b[1226] {s.store_offset(844, 843, 1.0);}
        if (!s.b[1226]) {s.store_div_from_scalar_offset_scaled_input(847, 1.0, 843, 8.0, 3.0);s.store_mul_scale_offset_rhs(844, 847, 843, 3.0, 1.0);}
        s.store_mul3_lhs(866, 397, 846, 844);s.store_div_scaled_inputs_indices(843, 130, ((-0.5) * s.v[892]), 865, 1.0);s.b[1227] = (s.v[843] > (-100.0));s.store_scalar(1227, if s.b[1227] { 1.0 } else { 0.0 });
        if s.b[1227] {s.store_exp(844, 843);s.store_mul_scale_offset_rhs(868, 844, 844, 2.0, 1.0);}
        if (!s.b[1227]) {s.store_scalar(844, 3.720075976e-44);s.store_mul_scale_offset_rhs(868, 844, 844, 2.0, 1.0);}
        s.store_div_scaled_product_indices(845, 100, 417, 1.0, 864, 1.0);s.store_add_scaled_value_products_indices(846, 96, 1.0, 97, 841, 1.0, 98, 822, 1.0);s.store_div_scaled_inputs2_mixed_aii(847, A::add_scaled_product(s.ad_value(845), 1.0, s.ad_value(846), s.ad_value(868), 1.0), 1.0, 99, 1.0, 396, 1.0);s.b[1228] = (s.v[847] >= (-0.5));s.store_scalar(1228, if s.b[1228] { 1.0 } else { 0.0 });
        if s.b[1228] {s.store_offset(831, 847, 1.0);}
        if (!s.b[1228]) {s.store_div_from_scalar_offset_scaled_input(843, 1.0, 847, 8.0, 3.0);s.store_mul_scale_offset_rhs(831, 843, 847, 3.0, 1.0);}
        s.b[1229] = (s.v[378] > 0.0);s.store_scalar(1229, if s.b[1229] { 1.0 } else { 0.0 });
        if s.b[1229] {s.store_mul_scale_offset_indices(843, 822, 379, -1.0, 0.0);}
        s.b[1230] = (s.v[843] < (-100.0));s.store_scalar(1230, if s.b[1230] { 1.0 } else { 0.0 });
        if (s.b[1229] && s.b[1230]) {s.store_scalar(845, 3.720075976e-44);}
        if (s.b[1229] && (!s.b[1230])) {s.store_exp(845, 843);}
        if s.b[1229] {s.store_offset_mul_offset_rhs(846, 378, 845, 1.0, s.v[892]);}
        if s.b[1229] {
            s.store_mul_mixed_ia(847, 832, {
                            if ((s.v[892] / s.v[846]) > 1e-38) {
                                A::ln(A::div_from_scalar(s.v[892], s.ad_value(846)))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if s.b[1229] {s.store_mul(1090, 831, 847);}
        if (!s.b[1229]) {s.store_scalar(1090, 0.0);}
        s.store_mul(63, 129, 868);s.store_mul(867, 63, 834);s.store_div_scaled_inputs_indices(843, 133, ((-0.5) * (s.v[328] * s.v[892])), 866, 1.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1231] = (s.v[843] > (-100.0));s.store_scalar(1231, if s.b[1231] { 1.0 } else { 0.0 });
        if s.b[1231] {s.store_exp(844, 843);s.store_mul_scale_offset_rhs(845, 844, 844, 2.0, 1.0);}
        if (!s.b[1231]) {s.store_scalar(844, 3.720075976e-44);s.store_mul_scale_offset_rhs(845, 844, 844, 2.0, 1.0);}
        s.store_mul(843, 132, 845);s.store_mul(904, 843, 834);s.store_sqrt_offset_scaled_input(843, 128, 1.0 / (s.v[892]), 1.0);s.store_add_scaled_inputs_product_indices(844, 121, 1.0, 122, 1.0 / (s.v[892]), 123, 841, 1.0);s.store_add_scaled_product_mixed_aii(903, A::mul3(s.ad_value(376), A::offset(s.ad_value(843), (-1.0)), s.ad_value(943)), 1.0, 844, 430, 1.0);s.store_div_scaled_product_offset_denominator_indices(870, 415, 942, 1.0, 127, s.v[328], 1.0);s.store_add_scaled_product_indices(846, 400, 1.0, 188, 841, 1.0);s.b[1232] = (s.v[846] < 0.0001);s.store_scalar(1232, if s.b[1232] { 1.0 } else { 0.0 });
        if s.b[1232] {s.store_div_from_scalar_sub_from_scalar_ad(852, 1.0, 3.0, A::scale(s.ad_value(846), 20000.0));s.store_mul_scale_offset_indices(846, 852, 846, -1.0, 0.0002);}
        s.store_mul3_lhs(873, 846, 1141, 822);s.store_sqrt_offset_scaled_input(1089, 377, 1.0 / (s.v[892]), 1.0);s.store_div_from_scalar(852, 2.2361, 943);s.store_add_scaled_product_right_sub(963, 828, 1.0, 852, 962, 841, (-1.0));s.store_exp_mul_scaled_lhs_indices(843, 382, 2.0, 822);s.store_div_scaled_product_offset_denominator_mixed_iai(1091, 391, A::offset(s.ad_value(843), (-1.0)), 1.0, 843, 1.0, 1.0);s.store_sub_add_scaled_inputs4_lhs_mixed_aiii(829, A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_value_products(s.ad_value(408), p.p37, A::add_scaled_products(s.ad_value(376), s.ad_value(963), 1.0, s.ad_value(405), s.ad_value(943), (-1.0)), s.ad_value(1089), 1.0, s.ad_value(403), s.ad_value(841), (-1.0)), 1.0, s.ad_value(867), (-1.0), s.ad_value(904), -1.0), 1.0, A::add_scaled_product(s.ad_value(125), 1.0, s.ad_value(126), s.ad_value(841), 1.0), s.ad_value(870), 1.0), 1.0, 903, 1.0, 873, -1.0, 1090, -1.0, 1091);s.store_sub(1053, 942, 1044);s.store_sqrt(1054, 1053);s.store_div_scaled_product_indices(1055, 944, 1054, 1.0, 943, 1.0);s.store_sqrt(846, 1055);s.store_mul(843, 131, 1044);s.b[1233] = (s.v[843] >= (-0.5));s.store_scalar(1233, if s.b[1233] { 1.0 } else { 0.0 });
        if s.b[1233] {s.store_offset(844, 843, 1.0);}
        if (!s.b[1233]) {s.store_div_from_scalar_offset_scaled_input(847, 1.0, 843, 8.0, 3.0);s.store_mul_scale_offset_rhs(844, 847, 843, 3.0, 1.0);}
        s.store_mul3_lhs(1056, 397, 846, 844);s.store_mul(843, 134, 1044);s.b[1234] = (s.v[843] >= (-0.5));s.store_scalar(1234, if s.b[1234] { 1.0 } else { 0.0 });
        if s.b[1234] {s.store_offset(844, 843, 1.0);}
        if (!s.b[1234]) {s.store_div_from_scalar_offset_scaled_input(847, 1.0, 843, 8.0, 3.0);s.store_mul_scale_offset_rhs(844, 847, 843, 3.0, 1.0);}
        s.store_mul3_lhs(1057, 397, 846, 844);s.store_div_scaled_inputs_indices(843, 130, ((-0.5) * s.v[892]), 1056, 1.0);s.b[1235] = (s.v[843] > (-100.0));s.store_scalar(1235, if s.b[1235] { 1.0 } else { 0.0 });
        if s.b[1235] {s.store_exp(844, 843);s.store_mul_scale_offset_rhs(1058, 844, 844, 2.0, 1.0);}
        if (!s.b[1235]) {s.store_scalar(844, 3.720075976e-44);s.store_mul_scale_offset_rhs(1058, 844, 844, 2.0, 1.0);}
        s.store_div_scaled_product_indices(845, 100, 417, 1.0, 1055, 1.0);s.store_add_scaled_value_products_indices(846, 96, 1.0, 97, 1044, 1.0, 98, 822, 1.0);s.store_div_scaled_inputs2_mixed_aii(847, A::add_scaled_product(s.ad_value(845), 1.0, s.ad_value(846), s.ad_value(1058), 1.0), 1.0, 99, 1.0, 396, 1.0);s.b[1236] = (s.v[847] >= (-0.5));s.store_scalar(1236, if s.b[1236] { 1.0 } else { 0.0 });
        if s.b[1236] {s.store_offset(1059, 847, 1.0);}
        if (!s.b[1236]) {s.store_div_from_scalar_offset_scaled_input(843, 1.0, 847, 8.0, 3.0);s.store_mul_scale_offset_rhs(1059, 843, 847, 3.0, 1.0);}
        s.b[1237] = (s.v[378] > 0.0);s.store_scalar(1237, if s.b[1237] { 1.0 } else { 0.0 });
        if s.b[1237] {s.store_mul_scale_offset_indices(843, 822, 379, -1.0, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1238] = (s.v[843] < (-100.0));s.store_scalar(1238, if s.b[1238] { 1.0 } else { 0.0 });
        if (s.b[1237] && s.b[1238]) {s.store_scalar(845, 3.720075976e-44);}
        if (s.b[1237] && (!s.b[1238])) {s.store_exp(845, 843);}
        if s.b[1237] {s.store_offset_mul_offset_rhs(846, 378, 845, 1.0, s.v[892]);}
        if s.b[1237] {
            s.store_mul_mixed_ia(847, 832, {
                            if ((s.v[892] / s.v[846]) > 1e-38) {
                                A::ln(A::div_from_scalar(s.v[892], s.ad_value(846)))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if s.b[1237] {s.store_mul(1071, 1059, 847);}
        if (!s.b[1237]) {s.store_scalar(1071, 0.0);}
        s.store_mul(63, 129, 1058);s.store_mul(1067, 63, 834);s.store_div_scaled_inputs_indices(843, 133, ((-0.5) * (s.v[328] * s.v[892])), 1057, 1.0);s.b[1239] = (s.v[843] > (-100.0));s.store_scalar(1239, if s.b[1239] { 1.0 } else { 0.0 });
        if s.b[1239] {s.store_exp(844, 843);s.store_mul_scale_offset_rhs(845, 844, 844, 2.0, 1.0);}
        if (!s.b[1239]) {s.store_scalar(844, 3.720075976e-44);s.store_mul_scale_offset_rhs(845, 844, 844, 2.0, 1.0);}
        s.store_mul(843, 132, 845);s.store_mul(1068, 843, 834);s.store_sqrt_offset_scaled_input(843, 128, 1.0 / (s.v[892]), 1.0);s.store_add_scaled_inputs_product_indices(844, 121, 1.0, 122, 1.0 / (s.v[892]), 123, 1044, 1.0);s.store_add_scaled_product_mixed_aii(1069, A::mul3(s.ad_value(376), A::offset(s.ad_value(843), (-1.0)), s.ad_value(943)), 1.0, 844, 430, 1.0);s.store_div_scaled_product_offset_denominator_indices(1066, 415, 942, 1.0, 127, s.v[328], 1.0);s.store_add_scaled_product_indices(846, 401, 1.0, 190, 1044, 1.0);s.b[1240] = (s.v[846] < 0.0001);s.store_scalar(1240, if s.b[1240] { 1.0 } else { 0.0 });
        if s.b[1240] {s.store_div_from_scalar_sub_from_scalar_ad(852, 1.0, 3.0, A::scale(s.ad_value(846), 20000.0));s.store_mul_scale_offset_indices(846, 852, 846, -1.0, 0.0002);}
        s.store_mul3_lhs(1070, 846, 1141, 822);s.store_sqrt_offset_scaled_input(1089, 377, 1.0 / (s.v[892]), 1.0);s.store_div_from_scalar(852, 2.2361, 943);s.store_add_scaled_product_right_sub(1072, 1054, 1.0, 852, 1045, 1044, (-1.0));s.store_exp_mul_scaled_lhs_indices(843, 382, 2.0, 822);s.store_div_scaled_product_offset_denominator_mixed_iai(1091, 391, A::offset(s.ad_value(843), (-1.0)), 1.0, 843, 1.0, 1.0);s.store_sub_add_scaled_inputs4_lhs_mixed_aiii(1073, A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_value_products(s.ad_value(408), p.p37, A::add_scaled_products(s.ad_value(376), s.ad_value(1072), 1.0, s.ad_value(405), s.ad_value(943), (-1.0)), s.ad_value(1089), 1.0, s.ad_value(403), s.ad_value(1044), (-1.0)), 1.0, s.ad_value(1067), (-1.0), s.ad_value(1068), -1.0), 1.0, A::add_scaled_product(s.ad_value(125), 1.0, s.ad_value(126), s.ad_value(1044), 1.0), s.ad_value(1066), 1.0), 1.0, 1069, 1.0, 1070, -1.0, 1071, -1.0, 1091);s.b[1241] = (((p.p61 == 3.0) && (p.p36 == 1.0)) && (p.p14 != 0.0));s.store_scalar(1241, if s.b[1241] { 1.0 } else { 0.0 });
        if s.b[1241] {s.store_sqrt(1007, 944);s.store_mul(1008, 397, 1007);s.store_mul(1009, 397, 1007);s.store_div_scaled_inputs_indices(843, 130, ((-0.5) * s.v[892]), 1008, 1.0);}
        s.b[1242] = (s.v[843] > (-100.0));s.store_scalar(1242, if s.b[1242] { 1.0 } else { 0.0 });
        if (s.b[1241] && s.b[1242]) {s.store_exp(844, 843);s.store_mul_scale_offset_rhs(1010, 844, 844, 2.0, 1.0);}
        if (s.b[1241] && (!s.b[1242])) {s.store_scalar(844, 3.720075976e-44);s.store_mul_scale_offset_rhs(1010, 844, 844, 2.0, 1.0);}
        if s.b[1241] {s.store_mul3_lhs(1011, 129, 1010, 834);s.store_div_scaled_inputs_indices(843, 133, ((-0.5) * (s.v[328] * s.v[892])), 1009, 1.0);}
        s.b[1243] = (s.v[843] > (-100.0));s.store_scalar(1243, if s.b[1243] { 1.0 } else { 0.0 });
        if (s.b[1241] && s.b[1243]) {s.store_exp(844, 843);s.store_mul_scale_offset_rhs(845, 844, 844, 2.0, 1.0);}
        if (s.b[1241] && (!s.b[1243])) {s.store_scalar(844, 3.720075976e-44);s.store_mul_scale_offset_rhs(845, 844, 844, 2.0, 1.0);}
        if s.b[1241] {s.store_mul(843, 132, 845);s.store_mul(1012, 843, 834);s.store_sqrt_offset_scaled_input(843, 128, 1.0 / (s.v[892]), 1.0);s.store_add_scaled_inputs(844, 121, 1.0, 122, 1.0 / (s.v[892]));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1241] {s.store_add_scaled_product_mixed_aii(1013, A::mul3(s.ad_value(376), A::offset(s.ad_value(843), (-1.0)), s.ad_value(943)), 1.0, 844, 430, 1.0);s.store_add_mixed_ai(1014, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(408), p.p37, s.ad_value(1011), (-1.0), s.ad_value(1012), -1.0), 1.0, s.ad_value(125), s.ad_value(1066), 1.0), 1013);}
        if (!s.b[1241]) {s.store_scalar(1014, 0.0);}
        s.store_sub(830, 825, 829);s.store_mul(853, 831, 832);s.store_div_scaled_product_indices(809, 384, 830, 1.0, 853, 1.0);s.store_div_scaled_inputs2_mixed_iai(833, 151, 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(384), s.ad_value(830)), (-1.0), 853, 1.0);s.b[1244] = (s.v[809] > 100.0);s.store_scalar(1244, if s.b[1244] { 1.0 } else { 0.0 });
        if s.b[1244] {s.copy_ad(875, 830);s.store_scalar(810, 0.0);}
        s.b[1245] = (s.v[833] > 100.0);s.store_scalar(1245, if s.b[1245] { 1.0 } else { 0.0 });
        if ((!s.b[1244]) && s.b[1245]) {s.store_div_scaled_inputs2_by_product_indices(843, 830, 1.0, 151, (-1.0), 831, 832, 1.0);s.store_exp(810, 843);s.store_mul_div_scaled_product_indices(875, 810, 832, 1140, 1.0, 396, 1.0);}
        if ((!s.b[1244]) && (!s.b[1245])) {s.store_exp(810, 809);s.store_mul_ln_mixed_ia(844, 853, A::offset(s.ad_value(810), 1.0));s.store_mul3_ad(857, A::div_scaled_inputs(s.ad_value(396), -1.0, A::mul(s.ad_value(832), s.ad_value(1140)), 1.0), A::exp(s.ad_value(833)), A::sub_from_scalar(1.0, s.ad_value(384)));s.store_sub_mixed_ia(845, 384, A::div_scaled_product(s.ad_value(853), s.ad_value(857), 1.0, A::sub_from_scalar(1.0, s.ad_value(384)), 1.0));s.store_div(875, 844, 845);}
        s.store_add_scaled_inputs(890, 875, 1.0, 832, 2.0);s.copy_ad(72, 875);s.b[1246] = (s.v[385] <= 0.0);s.store_scalar(1246, if s.b[1246] { 1.0 } else { 0.0 });
        if s.b[1246] {s.store_scalar(1092, 1.0);}
        if (!s.b[1246]) {s.store_div_scaled_inputs_indices(852, 385, ((s.v[892]) as f64).sqrt(), 890, 1.0);s.store_div_from_scalar_offset_input(1092, 1.0, 852, 1.0);}
        s.store_sub(852, 828, 943);s.store_sub_from_scalar_ad(893, s.v[328], A::add_scaled_products(s.ad_value(197), s.ad_value(875), (2.0 - p.p22), s.ad_value(198), s.ad_value(852), (2.0 - p.p22)));s.b[1247] = (s.v[893] < 2e-8);s.store_scalar(1247, if s.b[1247] { 1.0 } else { 0.0 });
        if s.b[1247] {s.store_div_from_scalar_sub_from_scalar_ad(843, 1.0, 6e-8, A::scale(s.ad_value(893), 2.0));s.store_mul_scale_offset_indices(893, 843, 893, -(2e-8), (4e-8) * (2e-8));}
        s.b[1248] = (p.p429 == 1.0);s.store_scalar(1248, if s.b[1248] { 1.0 } else { 0.0 });
        if s.b[1248] {s.store_scalar(887, 0.0);}
        if (!s.b[1248]) {s.store_add_scaled_products_indices(843, 183, 875, 1.0, 184, 852, 1.0);}
        s.b[1249] = (s.v[843] >= (-0.9));s.store_scalar(1249, if s.b[1249] { 1.0 } else { 0.0 });
        if ((!s.b[1248]) && s.b[1249]) {s.store_mul_scale_offset_indices(887, 955, 843, 1.0, 1.0);}
        if ((!s.b[1248]) && (!s.b[1249])) {s.store_div_from_scalar_offset_scaled_input(844, 1.0, 843, 20.0, 17.0);s.store_mul_ad_product_lhs_mixed_ia(887, 955, A::offset(s.ad_value(843), 0.8), 844);}
        s.store_offset_scaled(1101, 430, p.p137, p.p135);s.store_offset_scaled(1102, 430, p.p138, p.p136);s.b[1250] = (p.p429 == 2.0);s.store_scalar(1250, if s.b[1250] { 1.0 } else { 0.0 });
        if s.b[1250] {s.store_add_mixed_ai(887, A::add_scaled_inputs4(s.ad_value(61), 1.0, s.ad_value(887), 1.0, s.ad_value(60), 1.0, s.ad_value(1102), 1.0), 1101);}
        s.b[1251] = (s.v[103] == 0.0);s.store_scalar(1251, if s.b[1251] { 1.0 } else { 0.0 });
        if s.b[1251] {s.store_scalar(860, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1251] {s.store_scalar(861, 1.0);}
        if (!s.b[1251]) {s.store_mul(853, 107, 962);}
        s.b[1252] = (s.v[853] >= (-0.5));s.store_scalar(1252, if s.b[1252] { 1.0 } else { 0.0 });
        if ((!s.b[1251]) && s.b[1252]) {s.store_div_from_scalar_offset_input(854, 1.0, 853, 1.0);}
        if ((!s.b[1251]) && (!s.b[1252])) {s.store_scalar(855, ((-1.0) / ((1.0 - 0.5) * (1.0 - 0.5))));s.store_offset_scaled(964, 855, 0.5, (1.0 / (1.0 - 0.5)));s.store_add_scaled_product_indices(854, 964, 1.0, 855, 853, 1.0);}
        if (!s.b[1251]) {s.store_add(853, 942, 266);s.store_div_scaled_product_indices(964, 962, 854, 1.0, 853, 1.0);}
        s.b[1253] = (s.v[964] < 0.5);s.store_scalar(1253, if s.b[1253] { 1.0 } else { 0.0 });
        if ((!s.b[1251]) && s.b[1253]) {s.store_div_from_scalar_sqrt_ad(965, 1.0, A::sub_from_scalar(1.0, s.ad_value(964)));}
        if ((!s.b[1251]) && (!s.b[1253])) {s.store_scalar(854, (1.0 / ((2.0 * (1.0 - 0.5)) * (((1.0 - 0.5)) as f64).sqrt())));s.store_sub_from_scalar_scaled_input(855, (1.0 / (((1.0 - 0.5)) as f64).sqrt()), 854, 0.5);s.store_add_scaled_product_indices(965, 855, 1.0, 854, 964, 1.0);}
        if (!s.b[1251]) {s.store_div_scaled_product_mixed_iia(853, 376, 1089, 0.5, A::sqrt(A::add(s.ad_value(942), s.ad_value(266))), 1.0);s.store_mul(844, 853, 965);s.store_sqrt_mul(852, 242, 864);s.store_offset_scaled(869, 852, 2.0, s.v[892]);s.store_div_from_scalar(848, s.v[892], 869);s.store_mul(870, 103, 848);s.store_offset(871, 200, s.v[328]);s.store_div(872, 199, 871);s.store_add(845, 870, 872);s.store_square(849, 848);s.store_mul(850, 848, 849);s.store_offset_mul(861, 844, 845, 1.0);s.store_mul3_lhs(851, 104, 103, 850);s.store_mul_scale_offset_indices(879, 851, 844, -1.0, 0.0);s.store_add_scaled_product_indices(860, 861, 1.0, 879, 875, 1.0);}
        s.b[1254] = (s.v[861] < 0.01);s.store_scalar(1254, if s.b[1254] { 1.0 } else { 0.0 });
        if s.b[1254] {s.store_div_from_scalar_sub_from_scalar_ad(852, 1.0, 3.0, A::scale(s.ad_value(861), 200.0));s.store_mul_scale_offset_indices(861, 852, 861, -1.0, 0.02);}
        s.b[1255] = (s.v[860] < 0.01);s.store_scalar(1255, if s.b[1255] { 1.0 } else { 0.0 });
        if s.b[1255] {s.store_div_from_scalar_sub_from_scalar_ad(852, 1.0, 3.0, A::scale(s.ad_value(860), 200.0));s.store_mul_scale_offset_indices(860, 852, 860, -1.0, 0.02);}
        s.copy_ad(74, 860);s.b[1256] = (s.v[103] == 0.0);s.store_scalar(1256, if s.b[1256] { 1.0 } else { 0.0 });
        if s.b[1256] {s.store_scalar(1074, 1.0);}
        if (!s.b[1256]) {s.store_mul(853, 107, 1045);}
        s.b[1257] = (s.v[853] >= (-0.5));s.store_scalar(1257, if s.b[1257] { 1.0 } else { 0.0 });
        if ((!s.b[1256]) && s.b[1257]) {s.store_div_from_scalar_offset_input(854, 1.0, 853, 1.0);}
        if ((!s.b[1256]) && (!s.b[1257])) {s.store_scalar(855, ((-1.0) / ((1.0 - 0.5) * (1.0 - 0.5))));s.store_offset_scaled(964, 855, 0.5, (1.0 / (1.0 - 0.5)));s.store_add_scaled_product_indices(854, 964, 1.0, 855, 853, 1.0);}
        if (!s.b[1256]) {s.store_add(853, 942, 266);s.store_div_scaled_product_indices(964, 1045, 854, 1.0, 853, 1.0);}
        s.b[1258] = (s.v[964] < 0.5);s.store_scalar(1258, if s.b[1258] { 1.0 } else { 0.0 });
        if ((!s.b[1256]) && s.b[1258]) {s.store_div_from_scalar_sqrt_ad(965, 1.0, A::sub_from_scalar(1.0, s.ad_value(964)));}
        if ((!s.b[1256]) && (!s.b[1258])) {s.store_scalar(854, (1.0 / ((2.0 * (1.0 - 0.5)) * (((1.0 - 0.5)) as f64).sqrt())));s.store_sub_from_scalar_scaled_input(855, (1.0 / (((1.0 - 0.5)) as f64).sqrt()), 854, 0.5);s.store_add_scaled_product_indices(965, 855, 1.0, 854, 964, 1.0);}
        if (!s.b[1256]) {s.store_div_scaled_product_mixed_iia(853, 376, 1089, 0.5, A::sqrt(A::add(s.ad_value(942), s.ad_value(266))), 1.0);s.store_mul(844, 853, 965);s.store_sqrt_mul(852, 242, 1055);s.store_offset_scaled(869, 852, 2.0, s.v[892]);s.store_div_from_scalar(848, s.v[892], 869);s.store_mul(870, 103, 848);s.store_offset(871, 200, s.v[328]);s.store_div(872, 199, 871);s.store_add(845, 870, 872);s.store_square(849, 848);s.store_mul(850, 848, 849);s.store_offset_mul(1074, 844, 845, 1.0);}
        s.b[1259] = (s.v[1074] < 0.01);s.store_scalar(1259, if s.b[1259] { 1.0 } else { 0.0 });
        if s.b[1259] {s.store_div_from_scalar_sub_from_scalar_ad(852, 1.0, 3.0, A::scale(s.ad_value(1074), 200.0));s.store_mul_scale_offset_indices(1074, 852, 1074, -1.0, 0.02);}
        if (p.p41 != 0.0) {s.store_scaled_offset_ad(965, A::sub_from_scalar((p.p52 - p.p53), A::scale(s.ad_value(912), 0.5)), 0.45, (2.0 * p.p37));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (p.p41 != 0.0) {s.store_scalar(1109, ((p.p45 * p.p47) / 3.9));s.store_scaled_sub(856, 897, 941, p.p123);}
        if (p.p41 == 0.0) {s.store_scalar(965, 0.0);s.store_scalar(1109, p.p66);s.store_scaled_sub(856, 897, 941, p.p123);}
        s.b[1260] = (p.p62 == 1.0);s.store_scalar(1260, if s.b[1260] { 1.0 } else { 0.0 });
        if s.b[1260] {s.store_add_scaled_inputs4_indices(843, 875, 1.0, 829, 1.0, 829, 1.0, 965, -1.0);s.store_add_scaled_product_indices(845, 956, 1.0, 958, 841, 1.0);s.store_div(846, 843, 1109);s.store_mul_mixed_ia(848, 846, A::add_scaled_inputs_product(s.ad_value(845), 1.0, s.ad_value(856), 1.0, s.ad_value(957), s.ad_value(846), 1.0));}
        s.b[1261] = (p.p62 == 2.0);s.store_scalar(1261, if s.b[1261] { 1.0 } else { 0.0 });
        if ((!s.b[1260]) && s.b[1261]) {s.store_mul_shared_diff_quotient_add_product_input_product_quotient(848, 875, 965, 415, 956, 958, 841, 856, 957);}
        s.b[1262] = (p.p62 == 3.0);s.store_scalar(1262, if s.b[1262] { 1.0 } else { 0.0 });
        if (((!s.b[1260]) && (!s.b[1261])) && s.b[1262]) {s.store_add_scaled_inputs4_indices(843, 875, 1.0, 829, 1.0, 829, 1.0, 965, -1.0);s.store_offset_mul(845, 958, 841, 1.0);s.store_div(846, 843, 1109);s.store_mul_add_scaled_product_rhs_indices(847, 846, 956, 1.0, 957, 846, 1.0);s.store_mul(848, 847, 845);}
        if (((!s.b[1260]) && (!s.b[1261])) && (!s.b[1262])) {s.store_scale_ad(843, A::div_scaled_inputs2(s.ad_value(875), 1e-8, s.ad_value(68), 1e-8, s.ad_value(415), 1.0), 0.16666666666666666);}
        if (((!s.b[1260]) && (!s.b[1261])) && (!s.b[1262])) {
            s.store_exp_ad(844, A::mul(s.ad_value(148), {
                if (s.v[843] > 1e-38) {
                    A::ln(s.ad_value(843))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }));
        }
        if (((!s.b[1260]) && (!s.b[1261])) && (!s.b[1262])) {s.store_add_scaled_product_indices(845, 956, 1.0, 958, 841, 1.0);s.store_mul_pow_indices(1157, 149, 411, 150);s.store_mul_pow_indices(1158, 146, 411, 147);s.copy_ad(1108, 69);}
        if (((!s.b[1260]) && (!s.b[1261])) && (!s.b[1262])) {
            s.store_exp_ad(853, A::mul(s.ad_value(1157), {
                if ((1.0 + (s.v[875] / s.v[1108])) > 1e-38) {
                    A::ln(A::offset(A::div(s.ad_value(875), s.ad_value(1108)), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }));
        }
        if (((!s.b[1260]) && (!s.b[1261])) && (!s.b[1262])) {s.store_div(854, 1158, 853);s.store_add_scaled_product_indices(848, 854, 1.0, 844, 845, 1.0);}
        s.b[1263] = (s.v[848] >= (-0.8));s.store_scalar(1263, if s.b[1263] { 1.0 } else { 0.0 });
        if s.b[1263] {s.store_offset(936, 848, 1.0);}
        if (!s.b[1263]) {s.store_div_from_scalar_offset_scaled_input(852, 1.0, 848, 10.0, 7.0);s.store_mul_scale_offset_indices(936, 852, 848, 1.0, 0.6);}
        s.store_div_scaled_inputs3_indices(835, 945, 1.0, 897, p.p124, 941, (-p.p124), 936, 1.0);s.store_scale(835, 835, p.p31);s.copy_ad(75, 835);s.store_mul3_lhs(888, 893, 946, 396);s.store_mul(889, 888, 887);s.store_div_scaled_inputs_indices(836, 946, 2.0, 835, 1.0);s.store_scale(838, 836, s.v[892]);s.b[1264] = (s.v[105] == 0.0);s.store_scalar(1264, if s.b[1264] { 1.0 } else { 0.0 });
        if s.b[1264] {s.copy_ad(874, 106);}
        s.b[1265] = (s.v[105] > 0.0);s.store_scalar(1265, if s.b[1265] { 1.0 } else { 0.0 });
        if ((!s.b[1264]) && s.b[1265]) {s.store_sub_from_scalar(843, 1.0, 106);s.store_offset_add_scaled_product_indices(844, 843, 1.0, 105, 875, (-1.0), (-0.0001));s.store_sqrt_add_scaled_square_input(845, 844, 1.0, 843, 0.0004);s.store_add_scaled_inputs4_indices(874, 106, 1.0, 843, 1.0, 844, (-0.5), 845, (-0.5));}
        if ((!s.b[1264]) && (!s.b[1265])) {s.store_offset_add_scaled_product_indices(844, 106, 1.0, 105, 875, 1.0, (-0.0001));s.store_sqrt_add_scaled_square_input(845, 844, 1.0, 106, 0.0004);s.store_scaled_add(874, 844, 845, 0.5);}
        s.store_div(76, 860, 890);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1266] = ((s.v[887] == 0.0) && (s.v[874] == 1.0));s.store_scalar(1266, if s.b[1266] { 1.0 } else { 0.0 });
        if s.b[1266] {s.store_div_from_scalar_ad(843, 1.0, A::add_scaled_product(s.ad_value(890), 1.0, s.ad_value(860), s.ad_value(838), 1.0));s.store_mul(846, 838, 890);s.store_mul(837, 846, 843);}
        if (!s.b[1266]) {s.store_mul(852, 860, 889);s.store_mul(850, 890, 852);s.store_mul(849, 890, 889);s.store_mul_add_scaled_inputs_rhs(843, 860, A::offset(s.ad_value(852), (-1.0)), 2.0, A::div_from_scalar(1.0, s.ad_value(874)), 2.0);s.store_add_scaled_inputs_mixed_ai(844, A::add_scaled_products(s.ad_value(890), A::offset(A::div_from_scalar(2.0, s.ad_value(874)), (-1.0)), 1.0, s.ad_value(860), s.ad_value(838), 1.0), 1.0, 850, 3.0);s.store_mul_add_scaled_inputs_rhs_indices(845, 890, 838, 1.0, 849, 2.0);s.store_sqrt_add_scaled_square_product(846, 844, 1.0, 843, 845, (-2.0));s.store_div_scaled_inputs2_indices(837, 844, 1.0, 846, (-1.0), 843, 1.0);}
        s.store_add_scaled_inputs3_indices(844, 837, 1.0, 822, (-1.0), 180, -1.0);s.store_sqrt_add_scaled_square_product(845, 844, 1.0, 180, 837, 4.0);s.store_add_scaled_inputs3_indices(876, 837, 1.0, 844, (-0.5), 845, (-0.5));s.b[1267] = (s.v[876] > s.v[822]);s.store_scalar(1267, if s.b[1267] { 1.0 } else { 0.0 });
        if s.b[1267] {s.copy_ad(876, 822);}
        s.store_sub(878, 822, 876);s.copy_ad(77, 876);s.store_sub_from_scalar_ad(872, 1.0, A::div_scaled_product(s.ad_value(860), s.ad_value(837), 0.5, s.ad_value(890), 1.0));s.store_mul(852, 889, 875);s.store_add_scaled_inputs_product_indices(843, 838, 1.0, 837, 1.0, 852, 872, 2.0);s.store_mul(852, 889, 860);s.store_add_offset_lhs_mixed_ai(844, A::div_from_scalar(2.0, s.ad_value(874)), (-1.0), 852);s.store_div(840, 843, 844);s.b[1268] = ((s.v[191] > 0.0) && (s.v[878] > 1e-10));s.store_scalar(1268, if s.b[1268] { 1.0 } else { 0.0 });
        if s.b[1268] {s.store_div_from_scalar_ad(843, 1.0, A::mul3(s.ad_value(191), s.ad_value(860), s.ad_value(119)));s.store_div(845, 875, 838);s.store_scaled_add(844, 860, 845, s.v[892]);s.store_mul(852, 843, 844);s.store_mul(862, 852, 878);}
        if (!s.b[1268]) {s.store_scalar(862, 2.688117142e43);}
        s.b[1269] = (s.v[1142] > 0.0);s.store_scalar(1269, if s.b[1269] { 1.0 } else { 0.0 });
        if s.b[1269] {s.store_mul(851, 860, 837);s.store_mul(843, 890, 851);s.store_add(844, 890, 851);s.copy_ad(845, 1142);s.store_div_scaled_inputs2_mixed_iai(863, 890, 1.0, A::div(s.ad_value(843), s.ad_value(844)), (-1.0), 845, 1.0);s.store_mul(850, 194, 841);}
        s.b[1270] = (s.v[850] >= (-0.9));s.store_scalar(1270, if s.b[1270] { 1.0 } else { 0.0 });
        if (s.b[1269] && s.b[1270]) {s.store_div_from_scalar_offset_input(846, 1.0, 850, 1.0);s.store_mul(863, 863, 846);}
        if (s.b[1269] && (!s.b[1270])) {s.store_div_from_scalar_offset_input(847, 1.0, 850, 0.8);s.store_mul_scale_offset_rhs(846, 847, 850, 20.0, 17.0);s.store_mul(863, 863, 846);}
        if (!s.b[1269]) {s.store_scalar(863, 2.688117142e43);}
        s.store_mul(843, 387, 822);s.b[1271] = (s.v[843] > 100.0);s.store_scalar(1271, if s.b[1271] { 1.0 } else { 0.0 });
        if s.b[1271] {s.store_scalar(844, 2.688117142e43);}
        if (!s.b[1271]) {s.store_exp(844, 843);}
        s.b[1272] = (s.v[386] > 3.720075976e-44);s.store_scalar(1272, if s.b[1272] { 1.0 } else { 0.0 });
        if s.b[1272] {s.store_scalar(845, (1.0 + (p.p283 * s.v[892])));s.store_div_scaled_offset_numerator_mixed_ai(1093, A::mul(s.ad_value(845), s.ad_value(844)), 1.0, 1.0, 386, 1.0);s.store_mul(1093, 1093, 1092);}
        if (!s.b[1272]) {s.store_scalar(1093, 2.688117142e43);}
        s.store_div(851, 195, 838);s.store_mul(852, 851, 875);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1273] = (s.v[852] > (-0.9));s.store_scalar(1273, if s.b[1273] { 1.0 } else { 0.0 });
        if s.b[1273] {s.store_offset(843, 852, 1.0);}
        if (!s.b[1273]) {s.store_div_from_scalar_offset_scaled_input(844, 1.0, 852, 20.0, 17.0);s.store_mul_scale_offset_indices(843, 844, 852, 1.0, 0.8);}
        s.store_add(871, 862, 863);s.store_div_scaled_product_indices(844, 862, 863, 1.0, 871, 1.0);s.store_add(871, 844, 1093);s.store_div_scaled_product_indices(845, 844, 1093, 1.0, 871, 1.0);s.store_add_scaled_product_indices(839, 840, 1.0, 843, 845, 1.0);s.store_scaled_mul(886, 396, 893, 1.0 / (s.v[892]));s.store_mul(880, 835, 886);s.store_sub_from_scalar_ad(843, 1.0, A::div_scaled_product(s.ad_value(860), s.ad_value(876), 0.5, s.ad_value(890), 1.0));s.store_mul(882, 875, 843);s.store_div(852, 876, 838);s.store_offset(883, 852, 1.0);s.store_div_scaled_product_indices(881, 880, 882, 1.0, 883, 1.0);s.store_offset_mul(843, 881, 887, 1.0);s.store_div(852, 876, 843);s.store_mul(884, 881, 852);s.store_div(1085, 881, 843);s.store_div(852, 878, 839);s.store_offset(843, 852, 1.0);s.store_scaled_mul(885, 884, 843, 1.0 / (p.p23));s.store_scale(885, 885, p.p30);s.store_scaled_mul(78, 1085, 843, 1.0 / (p.p23));s.b[1274] = (s.v[78] < 1e-9);s.store_scalar(1274, if s.b[1274] { 1.0 } else { 0.0 });
        if s.b[1274] {s.store_scalar(78, 1e-9);}
        s.store_scaled_mul(1086, 1085, 843, 1.0 / (p.p23));s.b[1275] = (s.v[37] != 2.0);s.store_scalar(1275, if s.b[1275] { 1.0 } else { 0.0 });s.b[1276] = (p.p41 == 0.0);s.store_scalar(1276, if s.b[1276] { 1.0 } else { 0.0 });
        if (s.b[1275] && s.b[1276]) {s.store_mul_div_from_scalar_lhs_ad_indices(843, (3.0 * 3.9), 416, 415);}
        if (s.b[1275] && (!s.b[1276])) {s.store_div_scaled_inputs_indices(843, 415, p.p47, 416, 1.0);}
        s.b[1277] = (p.p43 == 0.0);s.store_scalar(1277, if s.b[1277] { 1.0 } else { 0.0 });s.b[1278] = (p.p41 == 0.0);s.store_scalar(1278, if s.b[1278] { 1.0 } else { 0.0 });
        if ((s.b[1275] && s.b[1277]) && s.b[1278]) {s.store_div_scaled_inputs3_indices(844, 822, -1.0, 1111, (-1.0), 1153, -1.0, 843, 1.0);}
        if ((s.b[1275] && s.b[1277]) && (!s.b[1278])) {s.store_div_scaled_inputs4_indices(844, 822, -1.0, 1111, (-1.0), 1153, -1.0, 375, 1.0, 843, 1.0);}
        s.b[1279] = (((s.v[1150] <= 0.0) || (s.v[1151] <= 0.0)) || (s.v[1152] < 0.0));s.store_scalar(1279, if s.b[1279] { 1.0 } else { 0.0 });
        if ((s.b[1275] && s.b[1277]) && s.b[1279]) {s.store_scalar(906, 0.0);}
        if ((s.b[1275] && s.b[1277]) && (!s.b[1279])) {s.store_scaled_add_mixed_ia(844, 844, A::sqrt_square_offset(s.ad_value(844), ((4.0 * 0.01) * 0.01)), 0.5);s.store_div_scaled_value_offset_denominator(845, s.ad_value(1151), 1.0, s.ad_value(844), 0.001, 1.0);s.store_mul_product3_mixed_aiii(906, A::exp_scaled_input(s.ad_value(845), -1.0), 995, 1150, 844, 1.0);s.store_square(847, 824);s.store_mul_scale_offset_indices(848, 847, 824, -1.0, 0.0);s.store_offset_add_ad(849, s.ad_value(1152), A::abs(s.ad_value(848)), 1e-9);s.store_offset_add_scaled_inputs(850, A::div(s.ad_value(848), s.ad_value(849)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(848), s.ad_value(849)), ((4.0 * 1e-6) * 1e-6)), 0.5, (-1e-6));s.store_mul(906, 906, 850);}
        s.b[1280] = (p.p41 == 0.0);s.store_scalar(1280, if s.b[1280] { 1.0 } else { 0.0 });
        if ((s.b[1275] && s.b[1277]) && s.b[1280]) {s.store_div_scaled_inputs3_indices(844, 822, 1.0, 825, (-1.0), 1146, -1.0, 843, 1.0);}
        if ((s.b[1275] && s.b[1277]) && (!s.b[1280])) {s.store_div_scaled_inputs4_indices(844, 822, 1.0, 825, (-1.0), 1146, -1.0, 375, 1.0, 843, 1.0);}
        s.b[1281] = (((s.v[1143] <= 0.0) || (s.v[1144] <= 0.0)) || (s.v[1145] < 0.0));s.store_scalar(1281, if s.b[1281] { 1.0 } else { 0.0 });
        if ((s.b[1275] && s.b[1277]) && s.b[1281]) {s.store_scalar(905, 0.0);}
        if ((s.b[1275] && s.b[1277]) && (!s.b[1281])) {s.store_scaled_add_mixed_ia(844, 844, A::sqrt_square_offset(s.ad_value(844), ((4.0 * 0.01) * 0.01)), 0.5);s.store_div_scaled_value_offset_denominator(845, s.ad_value(1144), 1.0, s.ad_value(844), 0.001, 1.0);}
    }
}
