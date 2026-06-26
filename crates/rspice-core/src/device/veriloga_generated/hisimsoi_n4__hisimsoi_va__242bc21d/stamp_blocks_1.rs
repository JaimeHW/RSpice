#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[733]) && s.b[1085]) {
            s.store_div_from_scalar(328, 1.0, 192);
            s.store_mul(329, 191, 328);
            s.store_sub_from_scalar(330, 1.0, 329);
            s.store_sub_from_scalar(336, 1.0, 330);
            s.store_square(49, 336);
            s.store_scalar(50, 1.0);
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1119] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1119] = if s.b[1119] { 1.0 } else { 0.0 };

        s.b[1120] = (4.0 == 1.0);
        s.v[1120] = if s.b[1120] { 1.0 } else { 0.0 };

        if ((((!s.b[733]) && s.b[1085]) && s.b[1119]) && s.b[1120]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1121] = (4.0 == 2.0);
        s.v[1121] = if s.b[1121] { 1.0 } else { 0.0 };

        if (((((!s.b[733]) && s.b[1085]) && s.b[1119]) && (!s.b[1120])) && s.b[1121]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1122] = (4.0 == 4.0);
        s.v[1122] = if s.b[1122] { 1.0 } else { 0.0 };

        if ((((((!s.b[733]) && s.b[1085]) && s.b[1119]) && (!s.b[1120])) && (!s.b[1121])) && s.b[1122]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1123] = (4.0 == 8.0);
        s.v[1123] = if s.b[1123] { 1.0 } else { 0.0 };

        if (((((((!s.b[733]) && s.b[1085]) && s.b[1119]) && (!s.b[1120])) && (!s.b[1121])) && (!s.b[1122])) && s.b[1123]) {
            s.store_scalar(55, 4.0);
        }

        if (((!s.b[733]) && s.b[1085]) && s.b[1119]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign16970_loop_guard: usize = 0;
        while {
            let assign16970_cond_e24532: f64 = if ((((!s.b[733]) && s.b[1085]) && s.b[1119]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign16970_cond_e24532 != 0.0
        } {
            assign16970_loop_guard += 1;
            assert!(assign16970_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[733]) && s.b[1085]) && s.b[1119]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((!s.b[733]) && s.b[1085]) && (!s.b[1119])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if ((!s.b[733]) && s.b[1085]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_scaled_mul(337, 336, 53, 1.0);
            s.store_sub_from_scalar(190, 1.0, 337);
            s.store_offset_ad(478, A::mul_offset_rhs(s.ad_value(190), s.ad_value(190), 1.0), 1.0);
        }

        if ((!s.b[733]) && s.b[1085]) {
            if ((1.0 + s.v[190]) >= (10.0 * 2.220446049250313e-16)) {
                s.store_offset(479, 190, 1.0);
            } else {
                s.store_scalar(479, (10.0 * 2.220446049250313e-16));
            }
        }

        if ((!s.b[733]) && s.b[1085]) {
            s.store_div_scaled_product_indices(328, 192, 478, 0.6666666666666667, 479, 1.0);
        }

        s.b[1124] = (s.v[339] <= 1.0);
        s.v[1124] = if s.b[1124] { 1.0 } else { 0.0 };

        s.b[1125] = (((s.v[164]) as f64).abs() > 1e-6);
        s.v[1125] = if s.b[1125] { 1.0 } else { 0.0 };

        if ((((!s.b[733]) && s.b[1085]) && s.b[1124]) && s.b[1125]) {
            s.store_sub_ad(436, A::add_scaled_product(A::mul3(A::add_scaled_inputs(A::square(s.ad_value(425)), 1.0, A::square(s.ad_value(427)), 0.08333333333333333), s.ad_value(225), s.ad_value(164)), 1.0, s.ad_value(425), s.ad_value(427), (-1.0)), A::div_scaled_product(A::mul3(A::add_scaled_inputs(s.ad_value(425), 2.0, A::div_scaled_product3_by_product(s.ad_value(323), s.ad_value(426), s.ad_value(426), 0.2, s.ad_value(225), s.ad_value(428), 1.0), 1.0), s.ad_value(426), s.ad_value(426)), s.ad_value(426), 0.16666666666666666, s.ad_value(428), 1.0));
            s.store_div(436, 436, 246);
        }

        if ((((!s.b[733]) && s.b[1085]) && s.b[1124]) && (!s.b[1125])) {
            s.copy_ad(436, 425);
        }

        if (((!s.b[733]) && s.b[1085]) && (!s.b[1124])) {
            s.store_scaled_add(436, 355, 358, (-0.5));
        }

        s.b[1129] = (s.v[612] == 0.0);
        s.v[1129] = if s.b[1129] { 1.0 } else { 0.0 };

        if s.b[1129] {
            s.store_offset(480, 190, 0.5);
            s.store_mul(481, 479, 478);
            s.store_div_scaled_inputs(482, s.ad_value(480), 0.4, s.ad_value(481), 1.0);
            s.store_sub_from_scalar(438, 0.6, 482);
        }

        s.b[1130] = (s.v[438] > (0.5 + 1e-8));
        s.v[1130] = if s.b[1130] { 1.0 } else { 0.0 };

        if (s.b[1129] && s.b[1130]) {
            s.store_scalar(438, 0.5);
        }

        if s.b[1129] {
            s.copy_ad(439, 438);
            s.store_scalar(438, 0.5);
        }

        s.b[1132] = (s.v[145] == 0.0);
        s.v[1132] = if s.b[1132] { 1.0 } else { 0.0 };

        s.b[1148] = ((p.p190 < (10.0 * 2.220446049250313e-16)) && (p.p191 < (10.0 * 2.220446049250313e-16)));
        s.v[1148] = if s.b[1148] { 1.0 } else { 0.0 };

        if ((s.b[1129] && s.b[1132]) && s.b[1148]) {
            s.store_scalar(316, 0.0);
            s.copy_ad(314, 162);
        }

        s.b[1149] = (s.v[314] > ((s.v[161] + s.v[173]) - (10.0 * 2.220446049250313e-16)));
        s.v[1149] = if s.b[1149] { 1.0 } else { 0.0 };

        if (((s.b[1129] && s.b[1132]) && s.b[1148]) && s.b[1149]) {
            s.store_offset_add(314, 161, 173, (-(10.0 * 2.220446049250313e-16)));
        }

        if ((s.b[1129] && s.b[1132]) && (!s.b[1148])) {
            s.store_scalar(1147, (if (p.p43 == 1.0) { p.p237 } else { s.v[402] }));
        }

        if ((s.b[1129] && s.b[1132]) && (!s.b[1148])) {
            s.store_div_from_scalar(1133, 1.0, 1147);
            s.store_mul(1134, 244, 1133);
            s.store_scale(1135, 1134, p.p191);
            s.store_add_scaled_product_indices(1138, 1135, 1.0, 80, 229, 1.0);
            s.store_div_from_scalar(1134, 1.0, 1138);
            s.store_scale(1137, 1134, 1.034943e-10);
            s.store_scalar(1134, (1.0 - p.p189));
            s.store_add_scaled_inputs_product_indices(314, 157, p.p189, 161, p.p189, 1134, 162, 1.0);
        }

        s.b[1150] = (s.v[314] > ((s.v[161] + s.v[173]) - (10.0 * 2.220446049250313e-16)));
        s.v[1150] = if s.b[1150] { 1.0 } else { 0.0 };

        if (((s.b[1129] && s.b[1132]) && (!s.b[1148])) && s.b[1150]) {
            s.store_offset_add(314, 161, 173, (-(10.0 * 2.220446049250313e-16)));
        }

        if ((s.b[1129] && s.b[1132]) && (!s.b[1148])) {
            s.store_sub(1140, 314, 162);
            s.store_sqrt_square_offset(44, 1140, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_add(1139, 1140, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1151] = (s.v[1139] < 0.0);
        s.v[1151] = if s.b[1151] { 1.0 } else { 0.0 };

        if (((s.b[1129] && s.b[1132]) && (!s.b[1148])) && s.b[1151]) {
            s.store_scalar(1139, 0.0);
        }

        if ((s.b[1129] && s.b[1132]) && (!s.b[1148])) {
            s.store_mul(1136, 225, 244);
            s.store_div_from_scalar(1134, 1.0, 1136);
            s.store_mul(1138, 246, 1134);
        }

        s.b[1152] = (s.v[1138] < s.v[227]);
        s.v[1152] = if s.b[1152] { 1.0 } else { 0.0 };

        if (((s.b[1129] && s.b[1132]) && (!s.b[1148])) && s.b[1152]) {
            s.copy_ad(1138, 227);
        }

        if ((s.b[1129] && s.b[1132]) && (!s.b[1148])) {
            s.store_scale(1144, 229, 9662367879.197212);
            s.store_scalar(1134, (100000.0 * 10000.0));
            s.store_scalar(1135, (1.0 / s.v[97]));
            s.store_mul_ad_lhs(1146, A::add_scaled_inputs_product(s.ad_value(1138), 2.0, A::mul3_scaled_output(s.ad_value(1144), s.ad_value(1139), s.ad_value(1137), 2.0), 1.0, s.ad_value(1134), s.ad_value(1137), 1.0), 1135);
            s.store_mul(1141, 1146, 1137);
            s.store_add_scaled_product_indices(1145, 1134, 4.0, 1144, 1139, (2.0 * 4.0));
            s.store_mul3_lhs(1142, 1145, 1137, 1137);
            s.store_sqrt_square_add(1143, 1141, 1142);
            s.store_mul_scale_ad_rhs(316, 326, A::sub(s.ad_value(1143), s.ad_value(1141)), 0.5);
        }

        if (s.b[1129] && s.b[1132]) {
            s.store_scale(316, 316, s.v[127]);
        }

        if s.b[1129] {
            s.store_sub_from_scalar(441, s.v[97], 316);
            s.store_sub_from_scalar(442, s.v[98], 316);
        }

        s.b[1153] = (s.v[441] < 1e-9);
        s.v[1153] = if s.b[1153] { 1.0 } else { 0.0 };

        if (s.b[1129] && s.b[1153]) {
            s.store_scalar(441, 1e-9);
        }

        if s.b[1129] {
            s.store_scale(328, 108, (-s.v[98]));
            s.store_mul(196, 328, 437);
            s.store_mul(197, 328, 436);
            s.store_mul(198, 197, 438);
        }

        s.b[1154] = (p.p43 == 0.0);
        s.v[1154] = if s.b[1154] { 1.0 } else { 0.0 };

        if (s.b[1129] && s.b[1154]) {
            s.store_scale(477, 196, 0.5);
            s.store_scale(476, 196, (1.0 - 0.5));
            s.store_mul_scale_ad_lhs(392, A::add(s.ad_value(357), s.ad_value(360)), (0.5 * s.v[98]), 108);
        }

        if s.b[1129] {
            s.store_scaled_sub(1155, 157, 164, 0.5);
            s.store_scale(44, 1155, (2.0 * 1.0 / (p.p227)));
            s.store_offset_ad(45, A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul(s.ad_value(44), A::scale_offset(s.ad_value(44), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0)), 1.0);
            s.store_div_from_scalar(177, p.p227, 45);
        }

        s.b[1156] = (s.v[177] < (10.0 * 2.220446049250313e-16));
        s.v[1156] = if s.b[1156] { 1.0 } else { 0.0 };

        if (s.b[1129] && s.b[1156]) {
            s.store_scalar(177, (10.0 * 2.220446049250313e-16));
        }

        if s.b[1129] {
            s.store_add(176, 161, 177);
            s.store_scalar(1166, (1.034943e-10 / 100.0));
            s.store_scale(1167, 437, 0.0001);
            s.store_scale(1168, 436, 0.0001);
            s.store_div_from_scalar(1157, p.p92, 1166);
            s.store_div_from_scalar(1158, p.p93, 1166);
            s.store_scalar(1159, p.p94);
            s.store_offset_mul_ad(1160, A::sub(s.ad_value(162), s.ad_value(161)), s.ad_value(1159), 1.0);
            s.store_add_scaled_products_indices(1161, 1157, 1167, 1.0, 1158, 1168, 1.0);
            s.store_div(1162, 1161, 1160);
            s.copy_ad(248, 1162);
            s.store_sqrt_square_offset(44, 248, ((4.0 * 3000.0) * 3000.0));
            s.store_offset_scaled_add(1159, 248, 44, 0.5, (1e-10 * 3000.0));
        }

        s.b[1169] = (s.v[1159] < 0.0);
        s.v[1169] = if s.b[1169] { 1.0 } else { 0.0 };

        if (s.b[1129] && s.b[1169]) {
            s.store_scalar(1159, 0.0);
        }

        if s.b[1129] {
            s.store_powf(1161, 1159, (p.p97 - 1.0));
            s.store_mul(1163, 1161, 1159);
            s.store_powf(1164, 1159, (s.v[111] - 1.0));
        }

    }

    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1129] {
            s.store_mul(1165, 1164, 1159);
            s.store_scale(249, 1168, 6.241449993689894e18);
            s.store_add_scaled_ad_lhs(1157, A::add_scaled_product(A::div_from_scalar(1.0, A::scale_offset(s.ad_value(249), (p.p96 * 1e-11), p.p95)), 1.0, s.ad_value(543), s.ad_value(1163), 1.0), 1165, 1.0 / (p.p106));
            s.store_div_from_scalar(251, 1.0, 1157);
            s.store_scale(251, 251, 0.0001);
            s.store_mul3_lhs(1170, 225, 244, 441);
            s.store_sqrt_square_offset(44, 1170, ((4.0 * 1e-50) * 1e-50));
            s.store_offset_scaled_add(1170, 1170, 44, 0.5, (1e-10 * 1e-50));
        }

        s.b[1178] = (s.v[1170] < 0.0);
        s.v[1178] = if s.b[1178] { 1.0 } else { 0.0 };

        if (s.b[1129] && s.b[1178]) {
            s.store_scalar(1170, 0.0);
        }

        if s.b[1129] {
            s.store_div_from_scalar(1171, 1.0, 1170);
            s.store_mul(1172, 246, 1171);
            s.store_div_scaled_inputs(1170, s.ad_value(253), 0.2, s.ad_value(251), 1.0);
            s.store_sqrt_square_sum(252, 1172, 1170);
            s.store_mul(1173, 251, 252);
            s.store_div(1171, 1173, 253);
        }

        s.b[1179] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1179] = if s.b[1179] { 1.0 } else { 0.0 };

        if (s.b[1129] && s.b[1179]) {
            s.store_scalar(1174, 1.0);
        }

        s.b[1180] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1180] = if s.b[1180] { 1.0 } else { 0.0 };

        if ((s.b[1129] && (!s.b[1179])) && s.b[1180]) {
            s.copy_ad(1174, 1171);
        }

        if ((s.b[1129] && (!s.b[1179])) && (!s.b[1180])) {
            s.store_powf(1174, 1171, (p.p113 - 1.0));
        }

        if s.b[1129] {
            s.store_mul(1170, 1171, 1174);
            s.store_offset(1175, 1170, 1.0);
        }

        s.b[1181] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1181] = if s.b[1181] { 1.0 } else { 0.0 };

        if (s.b[1129] && s.b[1181]) {
            s.store_div_from_scalar(1176, 1.0, 1175);
        }

        s.b[1182] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1182] = if s.b[1182] { 1.0 } else { 0.0 };

        if ((s.b[1129] && (!s.b[1181])) && s.b[1182]) {
            s.store_div_from_scalar_sqrt_ad(1176, 1.0, s.ad_value(1175));
        }

        if ((s.b[1129] && (!s.b[1181])) && (!s.b[1182])) {
            s.store_powf(1177, 1175, (((-1.0) / p.p113) - 1.0));
            s.store_mul(1176, 1175, 1177);
        }

        if s.b[1129] {
            s.store_mul(250, 251, 1176);
            s.store_div_scaled_product_denominator_ad(264, 107, 227, 1.0, A::sub_from_scalar(s.v[97], s.ad_value(316)), 1.0);
            s.store_mul3_lhs(200, 264, 246, 250);
            s.store_scalar(201, 0.0);
        }

        s.b[1192] = ((p.p281 > 0.0) && (p.p244 != 0.0));
        s.v[1192] = if s.b[1192] { 1.0 } else { 0.0 };

        if (s.b[1129] && s.b[1192]) {
            s.store_scaled_sub(1183, 157, 164, 0.5);
            s.store_scale(44, 1183, (2.0 * 100.0));
            s.store_offset_ad(45, A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul(s.ad_value(44), A::scale_offset(s.ad_value(44), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0)), 1.0);
            s.store_div_from_scalar(1189, 0.01, 45);
            s.store_sub_from_scalar_ad(1183, 1.1, A::add(s.ad_value(161), s.ad_value(1189)));
            s.store_sqrt_square_offset(44, 1183, ((4.0 * 0.05) * 0.05));
            s.store_offset_scaled_add(1191, 1183, 44, 0.5, (1e-10 * 0.05));
        }

        s.b[1193] = (s.v[1191] < 0.0);
        s.v[1193] = if s.b[1193] { 1.0 } else { 0.0 };

        if ((s.b[1129] && s.b[1192]) && s.b[1193]) {
            s.store_scalar(1191, 0.0);
        }

        if (s.b[1129] && s.b[1192]) {
            s.store_scale(1184, 225, s.v[116]);
            s.store_mul(1185, 323, 1184);
            s.store_powf(1184, 1191, p.p245);
            s.store_mul(1186, 1185, 1184);
            s.store_offset_scaled(1187, 173, p.p246, 1.0);
            s.store_scalar(1184, s.v[117]);
        }

        s.b[1194] = ((s.v[56] < 3.0) || (p.p43 == 1.0));
        s.v[1194] = if s.b[1194] { 1.0 } else { 0.0 };

        if ((s.b[1129] && s.b[1192]) && s.b[1194]) {
            s.store_add_scaled_inputs3(1188, s.ad_value(161), 1.0, s.ad_value(1189), 1.0, s.ad_value(172), -1.0);
        }

        if ((s.b[1129] && s.b[1192]) && (!s.b[1194])) {
            s.store_add_scaled_inputs3(1188, s.ad_value(161), 1.0, s.ad_value(1189), 1.0, s.ad_value(350), -1.0);
        }

        if (s.b[1129] && s.b[1192]) {
            s.store_add_ad_rhs(1187, 1187, A::mul3(s.ad_value(173), s.ad_value(1184), s.ad_value(1188)));
            s.store_mul(1189, 1186, 1187);
            s.copy_ad(1186, 1189);
        }

        if (s.b[1129] && (!s.b[1192])) {
            s.store_scalar(1186, 0.0);
        }

        s.b[1195] = (p.p248 != 0.0);
        s.v[1195] = if s.b[1195] { 1.0 } else { 0.0 };

        if (s.b[1129] && s.b[1195]) {
            s.store_scale(1183, 225, s.v[118]);
            s.store_mul(1191, 323, 1183);
            s.store_mul(1190, 1191, 173);
        }

        if (s.b[1129] && (!s.b[1195])) {
            s.store_scalar(1190, 0.0);
        }

        s.b[1196] = ((s.v[1186] + s.v[1190]) > 0.0);
        s.v[1196] = if s.b[1196] { 1.0 } else { 0.0 };

        if (s.b[1129] && s.b[1196]) {
            s.store_mul_add_rhs(247, 164, 1186, 1190);
            s.store_mul3_lhs(201, 264, 247, 250);
        }

        if s.b[1129] {
            s.store_add(199, 200, 201);
            s.copy_ad(203, 201);
        }

        s.b[1206] = (p.p33 != 0.0);
        s.v[1206] = if s.b[1206] { 1.0 } else { 0.0 };

        if (s.b[1129] && s.b[1206]) {
            s.copy_ad(1199, 549);
            s.store_scalar(1200, (s.v[124] - p.p71));
            s.store_div_from_scalar_square_ad(1201, 1.0, s.ad_value(1200));
            s.store_mul_ad_product_lhs(1202, A::mul_sub_from_scalar_lhs_scaled_output(p.p69, s.ad_value(233), s.ad_value(324), (2.0 * 1.034943e-10)), s.ad_value(1199), 1201);
            s.store_mul(186, 1202, 235);
            s.store_offset_scaled(1198, 173, p.p155, p.p154);
            s.store_mul(206, 186, 1198);
            s.store_sub_from_scalar_ad(1197, p.p156, A::scale(s.ad_value(157), p.p157));
            s.store_add_scaled_inputs3_offset(207, s.ad_value(174), 1.0, s.ad_value(1197), 1.0, s.ad_value(206), 1.0, (-s.v[123]));
            s.store_mul3_lhs(210, 205, 324, 324);
            s.store_scaled_mul(211, 210, 225, 0.5);
            s.store_scaled_mul(212, 211, 225, 2.0);
            s.store_offset_sub_ad(1203, A::offset(A::add_scaled_product(s.ad_value(227), 1.0, s.ad_value(210), s.ad_value(225), (-0.25)), ((s.v[123]) + ((-p.p156)))), s.ad_value(206), 1e-50);
            s.store_offset_sub(1197, 174, 1203, (-0.005));
        }

        if (s.b[1129] && s.b[1206]) {
            s.store_scalar(327, (if (s.v[1203] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if (s.b[1129] && s.b[1206]) {
            s.store_sqrt_ad(1199, A::add_scaled_square_product(s.ad_value(1197), 1.0, s.ad_value(327), s.ad_value(1203), (4.0 * 0.005)));
            s.store_sub_ad_lhs(1200, A::add_scaled_inputs4_offset(s.ad_value(1203), 1.0, s.ad_value(1197), 0.5, s.ad_value(1199), 0.5, s.ad_value(206), 1.0, (((-s.v[123])) + (p.p156))), 514);
            s.store_offset_mul(1201, 225, 1200, (-1.0));
            s.store_div_from_scalar(1202, 4.0, 212);
            s.store_offset_mul(1198, 1201, 1202, 1.0);
            s.store_sqrt_square_offset(44, 1198, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_add(1197, 1198, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1207] = (s.v[1197] < 0.0);
        s.v[1207] = if s.b[1207] { 1.0 } else { 0.0 };

        if ((s.b[1129] && s.b[1206]) && s.b[1207]) {
            s.store_scalar(1197, 0.0);
        }

        if (s.b[1129] && s.b[1206]) {
            s.store_sqrt_offset_input(213, 1197, 1e-50);
            s.store_add_ad_rhs(215, 207, A::mul_sub_from_scalar_rhs(s.ad_value(211), 1.0, s.ad_value(213)));
            s.store_div_from_scalar_add_ad(327, 1.0, s.ad_value(225), A::div_scalar_offset_denominator(2.0, s.ad_value(207), 1e-50, 1.0));
            s.store_mul_ln_ad_lhs(216, A::mul(A::div_scalar_by_product(1.0, s.ad_value(209), s.ad_value(210), 1.0), A::square(s.ad_value(207))), 327);
            s.store_div_scaled_value_offset_denominator(1200, s.ad_value(216), 1.0, s.ad_value(207), 1e-50, 1.0);
            s.store_offset_sub(217, 216, 215, (-0.002));
            s.store_sqrt_ad(327, A::add_scaled_inputs(A::square(s.ad_value(217)), 1.0, s.ad_value(216), (4.0 * 0.002)));
            s.store_add_scaled_inputs3(218, s.ad_value(216), 1.0, s.ad_value(217), (-0.5), s.ad_value(327), (-0.5));
            s.store_div_from_scalar(1197, 1.0, 327);
            s.store_mul_exp_ad_rhs(327, 209, A::mul(s.ad_value(225), s.ad_value(218)));
            s.store_add_ad_lhs(1198, A::offset(A::mul(s.ad_value(225), A::sub(s.ad_value(218), s.ad_value(514))), (-1.0)), 327);
            s.store_sqrt_square_offset(44, 1198, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_add(1197, 1198, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1208] = (s.v[1197] < 0.0);
        s.v[1208] = if s.b[1208] { 1.0 } else { 0.0 };

        if ((s.b[1129] && s.b[1206]) && s.b[1208]) {
            s.store_scalar(1197, 0.0);
        }

        if (s.b[1129] && s.b[1206]) {
            s.store_sqrt_offset_input(219, 1197, (10.0 * 2.220446049250313e-16));
            s.store_offset_mul_ad(1198, s.ad_value(225), A::sub(s.ad_value(218), s.ad_value(514)), (-1.0));
            s.store_sqrt_square_offset(44, 1198, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_add(1197, 1198, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1209] = (s.v[1197] < 0.0);
        s.v[1209] = if s.b[1209] { 1.0 } else { 0.0 };

        if ((s.b[1129] && s.b[1206]) && s.b[1209]) {
            s.store_scalar(1197, 0.0);
        }

        if (s.b[1129] && s.b[1206]) {
            s.store_sqrt_offset_input(220, 1197, (10.0 * 2.220446049250313e-16));
            s.store_mul_sub_rhs(221, 208, 219, 220);
            s.store_sub(1198, 215, 218);
            s.store_sqrt_square_offset(44, 1198, ((4.0 * 0.1) * 0.1));
            s.store_offset_scaled_add(1197, 1198, 44, 0.5, (1e-10 * 0.1));
        }

        s.b[1210] = (s.v[1197] < 0.0);
        s.v[1210] = if s.b[1210] { 1.0 } else { 0.0 };

        if ((s.b[1129] && s.b[1206]) && s.b[1210]) {
            s.store_scalar(1197, 0.0);
        }

        if (s.b[1129] && s.b[1206]) {
            s.store_div_scaled_value_offset_denominator(1204, s.ad_value(157), 1.0, s.ad_value(1197), (10.0 * 2.220446049250313e-16), 1.0);
            s.store_square(49, 1204);
            s.store_scalar(50, 1.0);
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
        }

    }

    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1129] && s.b[1206]) {
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1211] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1211] = if s.b[1211] { 1.0 } else { 0.0 };

        s.b[1212] = (4.0 == 1.0);
        s.v[1212] = if s.b[1212] { 1.0 } else { 0.0 };

        if (((s.b[1129] && s.b[1206]) && s.b[1211]) && s.b[1212]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1213] = (4.0 == 2.0);
        s.v[1213] = if s.b[1213] { 1.0 } else { 0.0 };

        if ((((s.b[1129] && s.b[1206]) && s.b[1211]) && (!s.b[1212])) && s.b[1213]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1214] = (4.0 == 4.0);
        s.v[1214] = if s.b[1214] { 1.0 } else { 0.0 };

        if (((((s.b[1129] && s.b[1206]) && s.b[1211]) && (!s.b[1212])) && (!s.b[1213])) && s.b[1214]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1215] = (4.0 == 8.0);
        s.v[1215] = if s.b[1215] { 1.0 } else { 0.0 };

        if ((((((s.b[1129] && s.b[1206]) && s.b[1211]) && (!s.b[1212])) && (!s.b[1213])) && (!s.b[1214])) && s.b[1215]) {
            s.store_scalar(55, 4.0);
        }

        if ((s.b[1129] && s.b[1206]) && s.b[1211]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign19450_loop_guard: usize = 0;
        while {
            let assign19450_cond_e26957: f64 = if (((s.b[1129] && s.b[1206]) && s.b[1211]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign19450_cond_e26957 != 0.0
        } {
            assign19450_loop_guard += 1;
            assert!(assign19450_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1129] && s.b[1206]) && s.b[1211]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((s.b[1129] && s.b[1206]) && (!s.b[1211])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if (s.b[1129] && s.b[1206]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_scaled_mul(1205, 1204, 53, 1.0);
            s.store_scale(214, 227, ((2.0 * s.v[126]) * p.p9));
            s.store_div_scaled_product_left_ad(222, A::mul3(s.ad_value(214), s.ad_value(250), s.ad_value(221)), 1205, 1.0, 441, 1.0);
            s.store_add(199, 199, 222);
        }

        s.b[1216] = ((p.p30 != 0.0) && (p.p32 != 0.0));
        s.v[1216] = if s.b[1216] { 1.0 } else { 0.0 };

        if (s.b[1129] && s.b[1216]) {
            s.store_square(294, 192);
            s.store_mul3_affine_lhs(295, 227, 324, 2.0, 0.0, 246);
            s.store_sub(296, 294, 295);
            s.store_sqrt_square_offset(44, 294, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_add(294, 294, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1217] = (s.v[294] < 0.0);
        s.v[1217] = if s.b[1217] { 1.0 } else { 0.0 };

        if ((s.b[1129] && s.b[1216]) && s.b[1217]) {
            s.store_scalar(294, 0.0);
        }

        if (s.b[1129] && s.b[1216]) {
            s.store_sqrt_square_offset(44, 296, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_add(296, 296, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1218] = (s.v[296] < 0.0);
        s.v[1218] = if s.b[1218] { 1.0 } else { 0.0 };

        if ((s.b[1129] && s.b[1216]) && s.b[1218]) {
            s.store_scalar(296, 0.0);
        }

        if (s.b[1129] && s.b[1216]) {
            s.store_sub(297, 294, 296);
        }

        s.b[1219] = ((s.v[244] < (10.0 * 2.220446049250313e-16)) || (s.v[297] < (10.0 * 2.220446049250313e-16)));
        s.v[1219] = if s.b[1219] { 1.0 } else { 0.0 };

        if ((s.b[1129] && s.b[1216]) && s.b[1219]) {
            s.store_scalar(146, 0.0);
        }

        if ((s.b[1129] && s.b[1216]) && (!s.b[1219])) {
            s.store_scalar(146, 1.0);
        }

        s.copy_ad(202, 199);

        s.v[204] = 0.0;

        s.b[1220] = ((p.p281 > 0.0) && (p.p285 > 0.0));
        s.v[1220] = if s.b[1220] { 1.0 } else { 0.0 };

        if s.b[1220] {
            s.store_scalar(1227, s.v[99]);
            s.store_scalar(1231, p.p237);
            s.store_offset_ad(1232, A::add_scaled_inputs3_offset(s.ad_value(158), 1.0, s.ad_value(185), 1.0, s.ad_value(320), -1.0, (-s.v[123])), (-p.p286));
            s.store_offset(1233, 182, p.p286);
            s.store_scalar(1235, p.p285);
            s.store_scalar(1234, p.p283);
            s.store_scalar(1225, s.v[70]);
            s.store_mul_ln_ad_rhs(1226, 227, A::div_scaled_product_by_product(s.ad_value(1225), s.ad_value(536), 1.0, s.ad_value(230), s.ad_value(230), 1.0));
        }

        if s.b[1220] {
            if (p.p43 == 1.0) {
                s.copy_ad(1223, 435);
            } else {
                s.copy_ad(1223, 350);
            }
        }

        if s.b[1220] {
            s.store_sqrt_ad(1228, A::div_scaled_product3(A::sub(s.ad_value(1226), s.ad_value(1223)), s.ad_value(536), s.ad_value(1225), ((2.0 * 1.6021918e-19) * 1.0 / (1.034943e-10)), A::add(s.ad_value(536), s.ad_value(1225)), 1.0));
            s.store_mul(1222, 1228, 1227);
            s.store_div_scaled_product_denominator_ad(1221, 1222, 1222, (-0.25), A::add(s.ad_value(157), s.ad_value(1222)), 1.0);
            s.copy_ad(1247, 1221);
            s.copy_ad(1248, 1233);
            s.store_offset_ad(336, A::div_scaled_offset_numerator(A::mul(s.ad_value(225), A::sub(s.ad_value(1232), s.ad_value(1247))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0), 1.0);
        }

        if s.b[1220] {
            if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(336, (10.0 * 2.220446049250313e-16));
            }
        }

        if s.b[1220] {
            s.store_add_ad_rhs(376, 1232, A::mul3_scaled_output(s.ad_value(241), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5));
        }

        s.b[1249] = (s.v[158] < ((s.v[123] + s.v[1248]) * 0.5));
        s.v[1249] = if s.b[1249] { 1.0 } else { 0.0 };

        if (s.b[1220] && s.b[1249]) {
            s.store_scalar(144, 0.0);
        }

        s.b[1250] = ((s.v[144] == 0.0) || (1.0 != 0.0));
        s.v[1250] = if s.b[1250] { 1.0 } else { 0.0 };

        if (s.b[1220] && s.b[1250]) {
            s.store_mul_sub_rhs(181, 225, 376, 1247);
        }

        s.b[1251] = (s.v[181] < 3.0);
        s.v[1251] = if s.b[1251] { 1.0 } else { 0.0 };

        if ((s.b[1220] && s.b[1250]) && s.b[1251]) {
            s.store_mul_sub_rhs(337, 225, 1232, 1247);
            s.store_div_from_scalar_ad(328, 1.0, A::mul_scaled_lhs(s.ad_value(225), (1.414213562373095 / 108.0), s.ad_value(240)));
            s.store_offset_scaled(329, 328, 3.0, 81.0);
            s.store_ad_value(330, A::add_scaled_sub_value_product((-2916.0), A::scale(s.ad_value(328), 81.0), 1.0, s.ad_value(328), s.ad_value(337), 27.0));
            s.store_ad_value(331, A::add_scaled_sub_value_product(1458.0, A::scaled_offset(s.ad_value(328), 54.0, 81.0), 1.0, s.ad_value(328), s.ad_value(337), 27.0));
            s.store_square(331, 331);
            s.store_powf_ad(332, A::add(s.ad_value(330), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(329), s.ad_value(329), s.ad_value(329), 4.0), s.ad_value(331)))), 0.3333333333333333);
            s.store_add_scaled_ad_lhs(336, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(329), 1.259921049894873, s.ad_value(332), 3.0)), 332, (1.0 / (3.0 * 1.259921049894873)));
            s.store_add_scaled_product_indices(376, 1247, 1.0, 336, 227, 1.0);
            s.copy_ad(378, 376);
        }

        s.b[1252] = ((s.v[158] - s.v[383]) <= s.v[1248]);
        s.v[1252] = if s.b[1252] { 1.0 } else { 0.0 };

        s.b[1253] = (p.p43 == 0.0);
        s.v[1253] = if s.b[1253] { 1.0 } else { 0.0 };

        if ((((s.b[1220] && s.b[1250]) && (!s.b[1251])) && s.b[1252]) && s.b[1253]) {
            s.store_div_from_scalar(327, 1.0, 323);
            s.store_scale(328, 1231, 9662367879.197212);
            s.store_scalar(329, (1.0 / s.v[93]));
            s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));
            s.store_mul_ad_rhs(331, 330, A::add_scaled_inputs_product(s.ad_value(1232), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));
            s.store_sub_ad_rhs(376, 1232, A::div(s.ad_value(331), s.ad_value(323)));
        }

        if (((s.b[1220] && s.b[1250]) && (!s.b[1251])) && s.b[1252]) {
            s.copy_ad(378, 376);
        }

        if (((s.b[1220] && s.b[1250]) && (!s.b[1251])) && (!s.b[1252])) {
            s.store_div_scalar_by_product(328, 1.0, s.ad_value(379), s.ad_value(434), 1.0);
            s.store_mul_ad_product_rhs(329, 328, A::sub(s.ad_value(1232), s.ad_value(383)), A::sub(s.ad_value(1232), s.ad_value(383)));
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(1232), s.ad_value(383))));
            s.store_offset_div_ad(377, A::ln(s.ad_value(329)), s.ad_value(330), p.p287);
            s.store_offset_sub(44, 377, 376, (-0.0008));
            s.store_scale(45, 377, (4.0 * 0.0008));
        }

        if (((s.b[1220] && s.b[1250]) && (!s.b[1251])) && (!s.b[1252])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((s.b[1220] && s.b[1250]) && (!s.b[1251])) && (!s.b[1252])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3(378, s.ad_value(377), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
        }

        s.b[1254] = (p.p43 == 0.0);
        s.v[1254] = if s.b[1254] { 1.0 } else { 0.0 };

        s.b[1255] = ((s.v[158] - s.v[383]) <= s.v[1248]);
        s.v[1255] = if s.b[1255] { 1.0 } else { 0.0 };

        if (((s.b[1220] && s.b[1250]) && s.b[1254]) && s.b[1255]) {
            s.store_div_from_scalar(327, 1.0, 323);
            s.store_scale(328, 1231, 9662367879.197212);
            s.store_scalar(329, (1.0 / s.v[93]));
            s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));
            s.store_mul_ad_rhs(331, 330, A::add_scaled_inputs_product(s.ad_value(1232), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));
            s.store_sub_ad_rhs(376, 1232, A::div(s.ad_value(331), s.ad_value(323)));
            s.copy_ad(378, 376);
        }

        if (((s.b[1220] && s.b[1250]) && s.b[1254]) && (!s.b[1255])) {
            s.store_div_from_scalar(327, 1.0, 323);
            s.store_scale(328, 1231, 9662367879.197212);
            s.store_scalar(329, (1.0 / s.v[93]));
            s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));
            s.store_mul_ad_rhs(331, 330, A::add_scaled_inputs_product(s.ad_value(1232), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));
            s.store_sub_ad_rhs(376, 1232, A::div(s.ad_value(331), s.ad_value(323)));
            s.copy_ad(378, 376);
        }

        s.b[1256] = ((s.v[1232] - s.v[383]) > 0.0);
        s.v[1256] = if s.b[1256] { 1.0 } else { 0.0 };

        if ((((s.b[1220] && s.b[1250]) && s.b[1254]) && (!s.b[1255])) && s.b[1256]) {
            s.store_div_scalar_by_product(328, 1.0, s.ad_value(379), s.ad_value(434), 1.0);
            s.store_mul_ad_product_rhs(329, 328, A::sub(s.ad_value(1232), s.ad_value(383)), A::sub(s.ad_value(1232), s.ad_value(383)));
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(1232), s.ad_value(383))));
            s.store_offset_div_ad(377, A::ln(s.ad_value(329)), s.ad_value(330), p.p287);
        }

        s.b[1257] = ((s.v[376] > ((s.v[377] * 0.98) - 0.4)) && (0.4 >= 0.0));
        s.v[1257] = if s.b[1257] { 1.0 } else { 0.0 };

        if (((((s.b[1220] && s.b[1250]) && s.b[1254]) && (!s.b[1255])) && s.b[1256]) && s.b[1257]) {
            s.store_offset_ad(44, A::sub_scaled_inputs(s.ad_value(376), 1.0, s.ad_value(377), 0.98), 0.4);
            s.store_square(49, 44);
            s.store_scalar(50, (0.4 * 0.4));
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
        }

    }

    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.b[1220] && s.b[1250]) && s.b[1254]) && (!s.b[1255])) && s.b[1256]) && s.b[1257]) {
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1258] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1258] = if s.b[1258] { 1.0 } else { 0.0 };

        s.b[1259] = (2.0 == 1.0);
        s.v[1259] = if s.b[1259] { 1.0 } else { 0.0 };

        if (((((((s.b[1220] && s.b[1250]) && s.b[1254]) && (!s.b[1255])) && s.b[1256]) && s.b[1257]) && s.b[1258]) && s.b[1259]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1260] = (2.0 == 2.0);
        s.v[1260] = if s.b[1260] { 1.0 } else { 0.0 };

        if ((((((((s.b[1220] && s.b[1250]) && s.b[1254]) && (!s.b[1255])) && s.b[1256]) && s.b[1257]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1261] = (2.0 == 4.0);
        s.v[1261] = if s.b[1261] { 1.0 } else { 0.0 };

        if (((((((((s.b[1220] && s.b[1250]) && s.b[1254]) && (!s.b[1255])) && s.b[1256]) && s.b[1257]) && s.b[1258]) && (!s.b[1259])) && (!s.b[1260])) && s.b[1261]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1262] = (2.0 == 8.0);
        s.v[1262] = if s.b[1262] { 1.0 } else { 0.0 };

        if ((((((((((s.b[1220] && s.b[1250]) && s.b[1254]) && (!s.b[1255])) && s.b[1256]) && s.b[1257]) && s.b[1258]) && (!s.b[1259])) && (!s.b[1260])) && (!s.b[1261])) && s.b[1262]) {
            s.store_scalar(55, 4.0);
        }

        if ((((((s.b[1220] && s.b[1250]) && s.b[1254]) && (!s.b[1255])) && s.b[1256]) && s.b[1257]) && s.b[1258]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign20680_loop_guard: usize = 0;
        while {
            let assign20680_cond_e28529: f64 = if (((((((s.b[1220] && s.b[1250]) && s.b[1254]) && (!s.b[1255])) && s.b[1256]) && s.b[1257]) && s.b[1258]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign20680_cond_e28529 != 0.0
        } {
            assign20680_loop_guard += 1;
            assert!(assign20680_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.b[1220] && s.b[1250]) && s.b[1254]) && (!s.b[1255])) && s.b[1256]) && s.b[1257]) && s.b[1258]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((((((s.b[1220] && s.b[1250]) && s.b[1254]) && (!s.b[1255])) && s.b[1256]) && s.b[1257]) && (!s.b[1258])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if (((((s.b[1220] && s.b[1250]) && s.b[1254]) && (!s.b[1255])) && s.b[1256]) && s.b[1257]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_scaled_mul(43, 44, 53, 0.4);
            s.store_add_ad_lhs(378, A::scale_offset(s.ad_value(377), 0.98, (-0.4)), 43);
        }

        if (((((s.b[1220] && s.b[1250]) && s.b[1254]) && (!s.b[1255])) && s.b[1256]) && (!s.b[1257])) {
            s.copy_ad(378, 376);
        }

        if s.b[1220] {
            s.store_offset(336, 1247, (5e-12 / 2.0));
        }

        s.b[1263] = (s.v[378] < s.v[336]);
        s.v[1263] = if s.b[1263] { 1.0 } else { 0.0 };

        if (s.b[1220] && s.b[1263]) {
            s.copy_ad(378, 336);
        }

        if s.b[1220] {
            s.copy_ad(1230, 378);
            s.copy_ad(163, 376);
        }

        if (s.b[1220] && (0.0 != 0.0)) {
            if ((s.v[376] - s.v[1230]) >= 0.0) {
                s.store_sub(166, 376, 1230);
            } else {
                s.store_scalar(166, 0.0);
            }
        }

        if (s.b[1220] && (0.0 != 0.0)) {
            s.store_offset_scaled(44, 166, (1.0 + 0.3), (((-p.p287)) + ((-0.03))));
            s.store_scale(45, 166, ((1.0 + 0.3) * (4.0 * 0.03)));
        }

        if (s.b[1220] && (0.0 != 0.0)) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (s.b[1220] && (0.0 != 0.0)) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3(165, s.ad_value(166), (1.0 + 0.3), s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
        }

        if (s.b[1220] && (0.0 != 0.0)) {
            if (s.v[165] <= s.v[166]) {
            } else {
                s.copy_ad(165, 166);
            }
        }

        s.b[1264] = (s.v[165] < 0.0);
        s.v[1264] = if s.b[1264] { 1.0 } else { 0.0 };

        if ((s.b[1220] && (0.0 != 0.0)) && s.b[1264]) {
            s.store_scalar(165, 0.0);
        }

        s.b[1265] = (s.v[165] > s.v[157]);
        s.v[1265] = if s.b[1265] { 1.0 } else { 0.0 };

        if (((s.b[1220] && (0.0 != 0.0)) && (!s.b[1264])) && s.b[1265]) {
            s.copy_ad(165, 157);
        }

        if (s.b[1220] && (0.0 != 0.0)) {
            s.store_add(163, 1230, 165);
        }

        s.b[1266] = (p.p282 == 1.0);
        s.v[1266] = if s.b[1266] { 1.0 } else { 0.0 };

        if (s.b[1220] && s.b[1266]) {
            s.copy_ad(378, 1230);
            s.copy_ad(1267, 1221);
            s.store_offset_ad(160, A::add_scaled_inputs3_offset(s.ad_value(185), (-1.0), s.ad_value(320), 1.0, s.ad_value(1267), 1.0, s.v[123]), p.p286);
        }

        s.b[1269] = (s.v[158] < s.v[160]);
        s.v[1269] = if s.b[1269] { 1.0 } else { 0.0 };

        if ((s.b[1220] && s.b[1266]) && s.b[1269]) {
            s.store_scalar(338, (-1.0));
            s.store_mul_scaled_ad_rhs(254, 227, 2.0, A::ln(A::div_from_scalar((-s.v[139]), s.ad_value(240))));
            s.store_mul_sub_rhs(336, 225, 1232, 1267);
            s.store_div_from_scalar_mul_ad(328, 1.0, s.ad_value(225), s.ad_value(238));
            s.store_mul(337, 328, 323);
            s.store_offset_scaled(262, 337, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(260, 262, 262, 8.0, 0.0, 262);
            s.store_offset(331, 336, (-2.0));
            s.store_scaled_mul(332, 337, 331, 9.0);
            s.store_sub_from_scalar(261, (7.0 * 1.414213562373095), 332);
            s.store_square(259, 261);
        }

        s.b[1270] = (s.v[260] < (s.v[259] * 1e-8));
        s.v[1270] = if s.b[1270] { 1.0 } else { 0.0 };

        if (((s.b[1220] && s.b[1266]) && s.b[1269]) && s.b[1270]) {
            s.store_add_scaled_inputs3_offset(257, s.ad_value(261), 1.0, A::div_scaled_inputs(s.ad_value(260), 0.5, s.ad_value(261), 1.0), 1.0, s.ad_value(332), 1.0, ((-7.0) * 1.414213562373095));
        }

        if (((s.b[1220] && s.b[1266]) && s.b[1269]) && (!s.b[1270])) {
            s.store_sqrt_add(258, 260, 259);
            s.store_add_ad_lhs(257, A::offset(s.ad_value(258), ((-7.0) * 1.414213562373095)), 332);
        }

        if ((s.b[1220] && s.b[1266]) && s.b[1269]) {
            s.store_powf(256, 257, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(255, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(337), 12.0)), 1.0, 256, 2.0, 256, 256, 1.414213562373095);
            s.store_div_from_scalar(328, 1.0, 256);
            s.store_mul(181, 255, 328);
            s.store_add_scaled_product_indices(313, 1267, 1.0, 181, 227, 1.0);
            s.store_sub(328, 313, 1267);
            s.store_div(329, 328, 254);
            s.store_sqrt_square_offset(330, 329, 1.0);
            s.store_add_ad_lhs(1230, A::div(s.ad_value(328), s.ad_value(330)), 1267);
        }

        if ((s.b[1220] && s.b[1266]) && (!s.b[1269])) {
            s.store_exp_ad(484, A::mul_offset_rhs(s.ad_value(225), s.ad_value(1267), (-p.p287)));
            s.store_scalar(430, 0.0);
            s.copy_ad(1268, 378);
            s.store_scale(419, 229, ((p.p237 * (p.p237 * 0.5)) * 9662367879.197212));
            s.store_sqrt_ad(327, A::mul_scaled_lhs(s.ad_value(225), 2.0, s.ad_value(419)));
            s.store_scaled_add_ad(328, A::exp(s.ad_value(327)), A::exp_scaled_input(s.ad_value(327), -1.0), 0.5);
            s.store_div_ad_lhs(420, A::ln(s.ad_value(328)), 419);
            s.store_scalar(167, 1.0);
        }

        let mut assign21280_loop_guard: usize = 0;
        while {
            let assign21280_cond_e29259: f64 = (s.v[57] + 1.0);
            let assign21280_cond_e29261: f64 = if (((s.b[1220] && s.b[1266]) && (!s.b[1269])) && (s.v[167] <= assign21280_cond_e29259)) { 1.0 } else { 0.0 };
            assign21280_cond_e29261 != 0.0
        } {
            assign21280_loop_guard += 1;
            assert!(assign21280_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1220] && s.b[1266]) && (!s.b[1269])) {
                s.store_sub(417, 1268, 1267);
                s.store_mul(181, 225, 417);
                s.store_mul_sub_rhs(337, 420, 417, 419);
            }
            s.b[1271] = (s.v[337] < 80.0);
            s.v[1271] = if s.b[1271] { 1.0 } else { 0.0 };
            if (((s.b[1220] && s.b[1266]) && (!s.b[1269])) && s.b[1271]) {
                s.store_exp(328, 337);
                s.store_exp_ad(327, A::mul_scaled_lhs(s.ad_value(420), -1.0, s.ad_value(419)));
                s.store_sub(329, 328, 327);
                s.store_div_ad_lhs(422, A::ln(A::offset(s.ad_value(329), 1.0)), 420);
                s.store_div_scaled_value_offset_denominator(423, s.ad_value(328), 1.0, s.ad_value(329), 1.0, 1.0);
            }
            if (((s.b[1220] && s.b[1266]) && (!s.b[1269])) && (!s.b[1271])) {
                s.store_sub(422, 417, 419);
                s.store_scalar(423, 1.0);
            }
            if ((s.b[1220] && s.b[1266]) && (!s.b[1269])) {
                s.store_mul(421, 225, 422);
            }
            s.b[1272] = (((s.v[181]) as f64).abs() < 1e-16);
            s.v[1272] = if s.b[1272] { 1.0 } else { 0.0 };
            if (((s.b[1220] && s.b[1266]) && (!s.b[1269])) && s.b[1272]) {
                s.store_sqrt_scaled_input_ad(327, A::sub_from_scalar(1.0, A::square(s.ad_value(423))), 1.0 / (2.0));
                s.store_mul(242, 181, 327);
                s.store_mul(443, 225, 327);
            }
            s.b[1273] = (s.v[181] < 0.0);
            s.v[1273] = if s.b[1273] { 1.0 } else { 0.0 };
            if ((((s.b[1220] && s.b[1266]) && (!s.b[1269])) && s.b[1272]) && s.b[1273]) {
                s.store_neg(242, 242);
                s.store_neg(443, 443);
            }
            s.b[1274] = (((s.v[181]) as f64).abs() < 0.005);
            s.v[1274] = if s.b[1274] { 1.0 } else { 0.0 };
            if ((((s.b[1220] && s.b[1266]) && (!s.b[1269])) && (!s.b[1272])) && s.b[1274]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(327, 181, 1.0, 181, 1.0, 181, 1.0, 181, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(328, 181, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::scale(s.ad_value(181), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(329, 421, 1.0, 421, 1.0, 421, 1.0, 421, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(330, 421, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::scale(s.ad_value(421), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sqrt_sub(242, 327, 329);
                s.store_div_scaled_product_right_ad(443, 225, A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(423), s.ad_value(330), (-1.0)), 0.5, 242, 1.0);
            }
            if ((((s.b[1220] && s.b[1266]) && (!s.b[1269])) && (!s.b[1272])) && (!s.b[1274])) {
                s.store_exp_neg_input(327, 181);
                s.store_exp_neg_input(328, 421);
                s.store_sqrt_ad(242, A::add_scaled_inputs4(s.ad_value(181), 1.0, s.ad_value(421), (-1.0), s.ad_value(327), 1.0, s.ad_value(328), (-1.0)));
                s.store_div_scaled_product_right_ad(443, 225, A::sub(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul_sub_from_scalar_rhs(s.ad_value(423), 1.0, s.ad_value(328))), 0.5, 242, 1.0);
            }
            s.b[1275] = ((s.v[430] == 1.0) && (s.v[181] < 0.0));
            s.v[1275] = if s.b[1275] { 1.0 } else { 0.0 };
            if (((s.b[1220] && s.b[1266]) && (!s.b[1269])) && s.b[1275]) {
                s.store_scalar(338, (-1.0));
            }
            s.b[1276] = (s.v[181] < 0.0);
            s.v[1276] = if s.b[1276] { 1.0 } else { 0.0 };
            if (((s.b[1220] && s.b[1266]) && (!s.b[1269])) && s.b[1276]) {
                s.store_neg(490, 242);
                s.store_neg(491, 443);
            }
            s.b[1277] = (s.v[181] < 1e-7);
            s.v[1277] = if s.b[1277] { 1.0 } else { 0.0 };
            if ((((s.b[1220] && s.b[1266]) && (!s.b[1269])) && (!s.b[1276])) && s.b[1277]) {
                s.copy_ad(490, 242);
                s.copy_ad(491, 443);
            }
            if ((((s.b[1220] && s.b[1266]) && (!s.b[1269])) && (!s.b[1276])) && (!s.b[1277])) {
                s.store_mul_offset_rhs(501, 225, 1268, (-p.p287));
                s.store_exp(502, 501);
                s.store_mul_ad_rhs(488, 379, A::add_scaled_offset_product_rhs(s.ad_value(502), 1.0, s.ad_value(484), s.ad_value(181), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(489, 379, s.ad_value(225), A::sub(s.ad_value(502), s.ad_value(484)));
                s.store_sqrt_square_add(490, 242, 488);
                s.store_div_scaled_add_product(491, s.ad_value(489), 0.5, s.ad_value(443), s.ad_value(242), (2.0 * 0.5), s.ad_value(490), 1.0);
            }
            if ((s.b[1220] && s.b[1266]) && (!s.b[1269])) {
                s.store_add_scaled_inputs_product_indices(492, 1268, 1.0, 1232, (-1.0), 240, 490, 1.0);
                s.store_offset_mul(493, 240, 491, 1.0);
            }
            s.b[1278] = (s.v[430] == 1.0);
            s.v[1278] = if s.b[1278] { 1.0 } else { 0.0 };
            if (((s.b[1220] && s.b[1266]) && (!s.b[1269])) && s.b[1278]) {
                s.store_scalar(167, (s.v[57] + 1.0));
            }
            if (((s.b[1220] && s.b[1266]) && (!s.b[1269])) && (!s.b[1278])) {
                s.store_div_scaled_inputs(494, s.ad_value(492), -1.0, s.ad_value(493), 1.0);
            }
            if (((s.b[1220] && s.b[1266]) && (!s.b[1269])) && (!s.b[1278])) {
                s.store_scaled_offset_ad(496, {
                    if (1.0 >= ((s.v[1268]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1268))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1279] = (((s.v[494]) as f64).abs() > s.v[496]);
            s.v[1279] = if s.b[1279] { 1.0 } else { 0.0 };
            if ((((s.b[1220] && s.b[1266]) && (!s.b[1269])) && (!s.b[1278])) && s.b[1279]) {
                s.store_scale(494, 496, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.b[1220] && s.b[1266]) && (!s.b[1269])) && (!s.b[1278])) {
                s.store_add(1268, 1268, 494);
            }
            s.b[1280] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[492]) as f64).abs() <= 1e-8));
            s.v[1280] = if s.b[1280] { 1.0 } else { 0.0 };
            if ((((s.b[1220] && s.b[1266]) && (!s.b[1269])) && (!s.b[1278])) && s.b[1280]) {
                s.store_scalar(430, 1.0);
            }
            if ((s.b[1220] && s.b[1266]) && (!s.b[1269])) {
                s.store_offset(167, 167, 1.0);
            }
        }

        if ((s.b[1220] && s.b[1266]) && (!s.b[1269])) {
            s.copy_ad(1230, 1268);
        }

        if s.b[1220] {
            s.store_mul_scaled_ad_rhs(332, 225, -1.0, A::sub(s.ad_value(1230), s.ad_value(1221)));
        }

        if s.b[1220] {
            s.store_scalar(1245, (if (s.v[332] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if s.b[1220] {
            s.store_mul(1246, 1245, 332);
            s.store_exp(333, 332);
            s.store_sub_ad_lhs(334, A::offset(s.ad_value(333), (-1.0)), 332);
        }

    }

    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1281] = (s.v[332] > 1e-7);
        s.v[1281] = if s.b[1281] { 1.0 } else { 0.0 };

        if (s.b[1220] && s.b[1281]) {
            s.store_mul_scaled_ad_rhs(437, 238, -1.0, A::sqrt(s.ad_value(334)));
        }

        s.b[1282] = (s.v[1246] > 1e-7);
        s.v[1282] = if s.b[1282] { 1.0 } else { 0.0 };

        if ((s.b[1220] && (!s.b[1281])) && s.b[1282]) {
            s.store_mul_sqrt_rhs(437, 238, 334);
        }

        if ((s.b[1220] && (!s.b[1281])) && (!s.b[1282])) {
            s.store_mul_ad_affine_product_rhs(437, 1245, s.ad_value(1246), A::sqrt(A::offset(A::mul_scaled_lhs(s.ad_value(1246), 0.3333333333333333, A::scale_offset(s.ad_value(1246), 0.25, 1.0)), 1.0)), (-0.7071067811865475), 0.0);
        }

        if s.b[1220] {
            s.store_sqrt_square_offset(44, 437, ((4.0 * 1e-6) * 1e-6));
            s.store_offset_scaled_add(1242, 437, 44, 0.5, (1e-10 * 1e-6));
        }

        s.b[1283] = (s.v[1242] < 0.0);
        s.v[1283] = if s.b[1283] { 1.0 } else { 0.0 };

        if (s.b[1220] && s.b[1283]) {
            s.store_scalar(1242, 0.0);
        }

        if s.b[1220] {
            s.store_div_scaled_inputs(1243, s.ad_value(1242), 1.0, s.ad_value(536), 1.6021918e-19);
            s.store_sub(328, 1243, 1234);
            s.store_scale(1244, 1243, 0.01);
            s.store_sqrt_ad(44, A::add_scaled_square_product(s.ad_value(328), 1.0, s.ad_value(1244), s.ad_value(1244), 4.0));
            s.store_add_scaled_inputs3(329, s.ad_value(328), 0.5, s.ad_value(44), 0.5, s.ad_value(1244), 1e-10);
        }

        s.b[1284] = (s.v[329] < 0.0);
        s.v[1284] = if s.b[1284] { 1.0 } else { 0.0 };

        if (s.b[1220] && s.b[1284]) {
            s.store_scalar(329, 0.0);
        }

        if s.b[1220] {
            s.store_div_scaled_product_by_product(1241, s.ad_value(329), s.ad_value(329), 1.0, s.ad_value(1243), s.ad_value(1243), 1.0);
            s.store_add_scaled_product_left_ad(1224, 1221, 1.0, A::sub(s.ad_value(1230), s.ad_value(1221)), 1241, 1.0);
            s.store_sub_ad(337, A::exp(A::mul(s.ad_value(225), s.ad_value(1224))), A::exp(A::mul(s.ad_value(225), A::sub(s.ad_value(1224), s.ad_value(157)))));
            s.store_sqrt_scaled_input(1237, 1225, ((2.0 * 1.6021918e-19) * 1.034943e-10));
            s.store_mul_sqrt_rhs(1238, 1237, 227);
            s.store_mul_sub_rhs(1229, 225, 1224, 1221);
        }

        s.b[1285] = ((s.v[1229] < (0.2 * s.v[225])) && ((0.2 * s.v[225]) >= 0.0));
        s.v[1285] = if s.b[1285] { 1.0 } else { 0.0 };

        if (s.b[1220] && s.b[1285]) {
            s.store_sub_scaled_inputs(44, 225, 0.2, 1229, 1.0);
            s.store_square(49, 44);
            s.store_scaled_mul(50, 225, 225, (0.2 * 0.2));
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1286] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[1286] = if s.b[1286] { 1.0 } else { 0.0 };

        s.b[1287] = (1.0 == 1.0);
        s.v[1287] = if s.b[1287] { 1.0 } else { 0.0 };

        if (((s.b[1220] && s.b[1285]) && s.b[1286]) && s.b[1287]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1288] = (1.0 == 2.0);
        s.v[1288] = if s.b[1288] { 1.0 } else { 0.0 };

        if ((((s.b[1220] && s.b[1285]) && s.b[1286]) && (!s.b[1287])) && s.b[1288]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1289] = (1.0 == 4.0);
        s.v[1289] = if s.b[1289] { 1.0 } else { 0.0 };

        if (((((s.b[1220] && s.b[1285]) && s.b[1286]) && (!s.b[1287])) && (!s.b[1288])) && s.b[1289]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1290] = (1.0 == 8.0);
        s.v[1290] = if s.b[1290] { 1.0 } else { 0.0 };

        if ((((((s.b[1220] && s.b[1285]) && s.b[1286]) && (!s.b[1287])) && (!s.b[1288])) && (!s.b[1289])) && s.b[1290]) {
            s.store_scalar(55, 4.0);
        }

        if ((s.b[1220] && s.b[1285]) && s.b[1286]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign21810_loop_guard: usize = 0;
        while {
            let assign21810_cond_e30576: f64 = if (((s.b[1220] && s.b[1285]) && s.b[1286]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign21810_cond_e30576 != 0.0
        } {
            assign21810_loop_guard += 1;
            assert!(assign21810_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1220] && s.b[1285]) && s.b[1286]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((s.b[1220] && s.b[1285]) && (!s.b[1286])) {
            s.store_powf(53, 53, (1.0 / 2.0));
        }

        if (s.b[1220] && s.b[1285]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_affine_lhs(43, 44, 225, 0.2, 0.0, 53);
            s.store_sub_scaled_inputs(328, 225, 0.2, 43, 1.0);
        }

        if (s.b[1220] && (!s.b[1285])) {
            s.copy_ad(328, 1229);
        }

        if s.b[1220] {
            s.store_sqrt_offset_input(1239, 328, (10.0 * 2.220446049250313e-16));
            s.store_mul(1240, 1238, 1239);
            s.store_mul_ad_lhs(1236, A::div_scaled_inputs(s.ad_value(227), 2.0, s.ad_value(1227), 1.0), 1240);
            s.store_mul_ad_lhs(204, A::mul3(s.ad_value(1236), s.ad_value(1235), s.ad_value(107)), 337);
            s.store_add(199, 202, 204);
        }

        s.store_add(201, 203, 204);

        s.b[1291] = ((p.p43 == 1.0) || (p.p45 == 1.0));
        s.v[1291] = if s.b[1291] { 1.0 } else { 0.0 };

        s.b[1304] = ((s.v[145] == 1.0) || (p.p25 == 0.0));
        s.v[1304] = if s.b[1304] { 1.0 } else { 0.0 };

        if (s.b[1291] && s.b[1304]) {
            s.store_scalar(263, 0.0);
        }

        s.b[1305] = ((p.p117 <= 0.0) || (s.v[73] <= 0.0));
        s.v[1305] = if s.b[1305] { 1.0 } else { 0.0 };

        if ((s.b[1291] && (!s.b[1304])) && s.b[1305]) {
            s.store_scalar(263, 0.0);
        }

        if ((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) {
            s.store_offset_ad(445, A::add_scaled_inputs3_offset(s.ad_value(174), 1.0, s.ad_value(185), 1.0, s.ad_value(320), -1.0, (-s.v[136])), p.p48);
        }

        s.b[1306] = (p.p44 <= 0.0);
        s.v[1306] = if s.b[1306] { 1.0 } else { 0.0 };

        if (((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && s.b[1306]) {
            s.copy_ad(1292, 445);
            s.store_square(1299, 323);
            s.copy_ad(1300, 545);
            s.store_div(1294, 1300, 1299);
            s.store_div_from_scalar(1301, 2.0, 1300);
            s.store_mul(1295, 1301, 1299);
            s.store_add_scaled_inputs_product_indices(1296, 1292, 1.0, 227, (-1.0), 130, 514, (-1.0));
            s.store_scale(483, 393, (p.p49 * 1.0 / (s.v[89])));
            s.store_add_scaled_product_indices(1296, 1296, 1.0, 130, 483, (-1.0));
            s.store_offset_mul(1298, 1295, 1296, 1.0);
            s.store_sqrt_square_offset(44, 1298, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_add(1297, 1298, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1307] = (s.v[1297] < 0.0);
        s.v[1307] = if s.b[1307] { 1.0 } else { 0.0 };

        if ((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && s.b[1306]) && s.b[1307]) {
            s.store_scalar(1297, 0.0);
        }

        if (((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && s.b[1306]) {
            s.store_offset(1297, 1297, 1e-50);
            s.store_sqrt(1297, 1297);
            s.store_add_scaled_product_value_ad(1302, A::mul_sub_from_scalar_rhs(s.ad_value(1294), 1.0, s.ad_value(1297)), 1.0, 1292, 137, 1.0);
            s.store_add_scaled_inputs3(1303, s.ad_value(173), p.p122, s.ad_value(176), 1.0, A::mul3(s.ad_value(131), s.ad_value(129), s.ad_value(1302)), -1.0);
            s.store_sqrt_square_offset(44, 1303, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_add(1303, 1303, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1308] = (s.v[1303] < 0.0);
        s.v[1308] = if s.b[1308] { 1.0 } else { 0.0 };

        if ((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && s.b[1306]) && s.b[1308]) {
            s.store_scalar(1303, 0.0);
        }

        if (((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) {
            s.store_mul(1292, 134, 445);
            s.store_div_ad_rhs(1294, 545, A::square(s.ad_value(323)));
            s.store_mul_ad(1295, A::div_from_scalar(2.0, s.ad_value(545)), A::square(s.ad_value(323)));
            s.store_add_scaled_inputs_product_indices(1296, 1292, 1.0, 227, (-1.0), 130, 514, (-1.0));
            s.store_scale(483, 393, (p.p49 * 1.0 / (s.v[89])));
            s.store_add_scaled_product_indices(1296, 1296, 1.0, 130, 483, (-1.0));
            s.store_offset_mul(1297, 1295, 1296, 1.0);
            s.store_scaled_offset(1299, 1295, 1.0, 2.0);
        }

        s.b[1309] = ((s.v[1297] < (1e-50 + s.v[1299])) && (s.v[1299] >= 0.0));
        s.v[1309] = if s.b[1309] { 1.0 } else { 0.0 };

        if ((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) && s.b[1309]) {
            s.store_sub_ad_lhs(44, A::offset(s.ad_value(1299), 1e-50), 1297);
            s.store_square(49, 44);
            s.store_square(50, 1299);
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1310] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1310] = if s.b[1310] { 1.0 } else { 0.0 };

        s.b[1311] = (4.0 == 1.0);
        s.v[1311] = if s.b[1311] { 1.0 } else { 0.0 };

        if ((((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) && s.b[1309]) && s.b[1310]) && s.b[1311]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1312] = (4.0 == 2.0);
        s.v[1312] = if s.b[1312] { 1.0 } else { 0.0 };

        if (((((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) && s.b[1309]) && s.b[1310]) && (!s.b[1311])) && s.b[1312]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1313] = (4.0 == 4.0);
        s.v[1313] = if s.b[1313] { 1.0 } else { 0.0 };

        if ((((((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) && s.b[1309]) && s.b[1310]) && (!s.b[1311])) && (!s.b[1312])) && s.b[1313]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1314] = (4.0 == 8.0);
        s.v[1314] = if s.b[1314] { 1.0 } else { 0.0 };

        if (((((((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) && s.b[1309]) && s.b[1310]) && (!s.b[1311])) && (!s.b[1312])) && (!s.b[1313])) && s.b[1314]) {
            s.store_scalar(55, 4.0);
        }

        if (((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) && s.b[1309]) && s.b[1310]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign22600_loop_guard: usize = 0;
        while {
            let assign22600_cond_e31695: f64 = if ((((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) && s.b[1309]) && s.b[1310]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign22600_cond_e31695 != 0.0
        } {
            assign22600_loop_guard += 1;
            assert!(assign22600_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) && s.b[1309]) && s.b[1310]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) && s.b[1309]) && (!s.b[1310])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if ((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) && s.b[1309]) {
            s.store_div_from_scalar(53, 1.0, 53);
        }

    }

    pub(super) fn stamp_transient_block_21(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) && s.b[1309]) {
            s.store_mul3_lhs(43, 44, 1299, 53);
            s.store_sub_ad_lhs(1297, A::offset(s.ad_value(1299), 1e-50), 43);
        }

        if ((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) && (!s.b[1309])) {
        }

        if (((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) {
            if (s.v[1297] <= 0.0) {
                s.store_scalar(1297, 0.0);
            } else {
                s.store_sqrt(1297, 1297);
            }
        }

        if (((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) {
            s.store_add_ad_rhs(1302, 1292, A::mul_sub_from_scalar_rhs(s.ad_value(1294), 1.0, s.ad_value(1297)));
            s.store_div_from_scalar_offset_input(1293, s.v[100], 131, s.v[100]);
            s.store_add_scaled_inputs_product_indices(1303, 173, p.p122, 176, 1.0, 1293, 1302, (-1.0));
            s.store_sqrt_square_offset(44, 1303, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_add(1303, 1303, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1315] = (s.v[1303] < 0.0);
        s.v[1315] = if s.b[1315] { 1.0 } else { 0.0 };

        if ((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) && s.b[1315]) {
            s.store_scalar(1303, 0.0);
        }

        if ((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) {
            s.store_offset(1303, 1303, 1e-50);
            s.store_exp_ad(1293, A::div_scaled_inputs(s.ad_value(133), -1.0, s.ad_value(1303), 1.0));
            s.store_mul_ad_lhs(263, A::mul3(s.ad_value(132), s.ad_value(1303), s.ad_value(199)), 1293);
        }

        s.b[1316] = (((p.p25 == 1.0) && (p.p26 == 2.0)) && (p.p43 == 1.0));
        s.v[1316] = if s.b[1316] { 1.0 } else { 0.0 };

        if s.b[1316] {
            s.store_mul_scaled_ad_rhs(1317, 107, (1.6021918e-19 * p.p237), A::exp_scaled_input(s.ad_value(225), (-p.p141)));
            s.store_scale(1320, 227, 0.0);
            s.store_add_scaled_inputs3(44, s.ad_value(231), 1.0, s.ad_value(1320), (-1.0), s.ad_value(231), (-0.01));
            s.store_scaled_mul(45, 231, 231, (4.0 * 0.01));
        }

        if s.b[1316] {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if s.b[1316] {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3(1320, s.ad_value(231), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
            s.store_sqrt_ad(1321, A::mul_scaled_lhs(s.ad_value(544), ((2.0 * 1.034943e-10) * 1.6021918e-19), s.ad_value(227)));
            s.store_mul_sub_rhs(1322, 225, 176, 1320);
        }

        if s.b[1316] {
            if (s.v[1322] > 0.0) {
                s.store_sqrt(1322, 1322);
            } else {
                s.store_neg_ad(1322, A::sqrt_scaled_input(s.ad_value(1322), -1.0));
            }
        }

        if s.b[1316] {
            s.store_sqrt_mul(1323, 225, 176);
            s.store_mul_scaled_ad_rhs(1324, 1321, -1.0, A::sub(s.ad_value(1322), s.ad_value(1323)));
            s.store_offset_sub_from_scalar_ad(44, p.p47, s.ad_value(1324), (-(p.p47 * 0.01)));
            s.store_scalar(45, ((4.0 * p.p47) * (p.p47 * 0.01)));
        }

        if s.b[1316] {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if s.b[1316] {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_sub_from_scalar_ad(393, p.p47, A::add_scaled_inputs(s.ad_value(44), 0.5, s.ad_value(45), 0.5));
        }

        if s.b[1316] {
            s.store_scalar(1317, (if (p.p138 > 0.0) { p.p138 } else { 1.0 }));
        }

        if s.b[1316] {
            s.store_div_scaled_value_offset_denominator(398, s.ad_value(1317), 1.0, s.ad_value(263), p.p139, 1.0);
            s.store_mul(397, 398, 323);
            s.copy_ad(396, 393);
            s.store_scaled_voltage(596, ctx, nodes, Some(17), None, (1e-9 / 0.0001));
            s.copy_ad(393, 596);
            s.store_div_scaled_inputs2(592, s.ad_value(596), 1.0, s.ad_value(396), (-1.0), s.ad_value(397), 1.0);
        }

        s.b[1338] = (((s.v[145] == 0.0) && (s.v[263] > 0.0)) && (p.p146 != 0.0));
        s.v[1338] = if s.b[1338] { 1.0 } else { 0.0 };

        s.b[1339] = (s.v[56] < 3.0);
        s.v[1339] = if s.b[1339] { 1.0 } else { 0.0 };

        if (s.b[1338] && s.b[1339]) {
            s.store_scalar(516, 0.0);
            s.store_scalar(517, 0.0);
        }

        if (s.b[1338] && (!s.b[1339])) {
            if (p.p43 == 1.0) {
                s.copy_ad(516, 156);
            } else {
                s.copy_ad(516, 350);
            }
        }

        if (s.b[1338] && (!s.b[1339])) {
            if (p.p43 == 1.0) {
                s.copy_ad(517, 156);
            } else {
                s.copy_ad(517, 353);
            }
        }

        if s.b[1338] {
            s.store_offset_scaled(1325, 185, p.p147, 1.0);
            s.store_scaled_mul(1326, 1325, 263, p.p146);
            s.store_offset_mul_ad(1327, s.ad_value(225), A::sub(s.ad_value(161), s.ad_value(516)), (-1.0));
            s.store_sqrt_square_offset(44, 1327, ((4.0 * 0.1) * 0.1));
            s.store_offset_scaled_add(1327, 1327, 44, 0.5, (1e-10 * 0.1));
        }

        s.b[1340] = (s.v[1327] < 0.0);
        s.v[1340] = if s.b[1340] { 1.0 } else { 0.0 };

        if (s.b[1338] && s.b[1340]) {
            s.store_scalar(1327, 0.0);
        }

        if s.b[1338] {
            s.store_sqrt(1328, 1327);
            s.store_mul(1329, 1327, 1328);
            s.store_offset_mul_ad(1330, s.ad_value(225), A::sub(s.ad_value(162), s.ad_value(517)), (-1.0));
            s.store_sqrt_square_offset(44, 1330, ((4.0 * 0.1) * 0.1));
            s.store_offset_scaled_add(1330, 1330, 44, 0.5, (1e-10 * 0.1));
        }

        s.b[1341] = (s.v[1330] < 0.0);
        s.v[1341] = if s.b[1341] { 1.0 } else { 0.0 };

        if (s.b[1338] && s.b[1341]) {
            s.store_scalar(1330, 0.0);
        }

        if s.b[1338] {
            s.store_sqrt(1331, 1330);
            s.store_mul(1332, 1330, 1331);
            s.store_div_from_scalar(1333, 1.0, 1327);
            s.store_mul3_lhs(328, 225, 1326, 1333);
            s.store_div_from_scalar(1333, 1.0, 1330);
            s.store_mul3_lhs(1334, 225, 1326, 1333);
            s.store_mul_ad_rhs(1335, 238, A::add_scaled_products(s.ad_value(1332), s.ad_value(1334), 1.0, s.ad_value(1329), s.ad_value(328), (-1.0)));
            s.store_mul_scaled_ad_rhs(1336, 238, 0.5, A::add_scaled_products(s.ad_value(1331), s.ad_value(1334), -1.0, s.ad_value(1328), s.ad_value(328), 1.0));
            s.store_add(1337, 1335, 1336);
            s.store_mul3_lhs(265, 264, 1337, 250);
        }

        s.v[1355] = (s.v[88] * 100.0);

        s.store_scale(1356, 323, 0.0001);

        s.v[1357] = (s.v[97] * 100.0);

        s.store_scale(1358, 107, 100.0);

        s.store_scale(1359, 252, 0.01);

        s.store_scale(1360, 436, 0.0001);

        s.store_scale(1361, 238, 0.0001);

        s.b[1362] = (p.p27 == 0.0);
        s.v[1362] = if s.b[1362] { 1.0 } else { 0.0 };

        if s.b[1362] {
            s.store_scalar(309, 0.0);
            s.store_scalar(306, 0.0);
            s.store_scalar(307, 0.0);
            s.store_scalar(308, 0.0);
            s.store_scalar(310, 0.0);
        }

        s.b[1363] = (s.v[145] == 0.0);
        s.v[1363] = if s.b[1363] { 1.0 } else { 0.0 };

        if ((!s.b[1362]) && s.b[1363]) {
            s.store_offset_add(1354, 176, 173, (-(10.0 * 2.220446049250313e-16)));
            s.store_add_scaled_inputs4_offset(1344, s.ad_value(174), 1.0, s.ad_value(185), (p.p216 * s.v[1357]), s.ad_value(320), (-(p.p216 * s.v[1357])), s.ad_value(1354), (-p.p215), (-s.v[123]));
            s.store_scalar(1346, (1.0 / s.v[1355]));
            s.store_mul(1345, 1344, 1346);
            s.store_scalar(1346, (1.0 / p.p217));
            s.store_offset_mul(1350, 1359, 1346, 1.0);
            s.store_mul(1353, 1345, 1350);
            s.store_sqrt_square_offset(44, 1353, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_add(1353, 1353, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1364] = (s.v[1353] < 0.0);
        s.v[1364] = if s.b[1364] { 1.0 } else { 0.0 };

        if (((!s.b[1362]) && s.b[1363]) && s.b[1364]) {
            s.store_scalar(1353, 0.0);
        }

        if ((!s.b[1362]) && s.b[1363]) {
            s.store_sqrt_square_offset(44, 174, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_add(1346, 174, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1365] = (s.v[1346] < 0.0);
        s.v[1365] = if s.b[1365] { 1.0 } else { 0.0 };

        if (((!s.b[1362]) && s.b[1363]) && s.b[1365]) {
            s.store_scalar(1346, 0.0);
        }

        if ((!s.b[1362]) && s.b[1363]) {
            s.store_offset(1346, 1346, (-p.p226));
            s.store_scale(1342, 1346, 10.0);
            s.store_offset_square(1345, 1342, 1.0);
            s.store_sub_from_scalar_ad(1344, 1.0, A::div_from_scalar(1.0, s.ad_value(1345)));
            s.store_mul(1353, 1353, 1344);
            s.store_scale(1343, 1358, s.v[1357]);
            s.store_div_from_scalar_offset_input(1350, p.p219, 1343, p.p219);
            s.store_scalar(1349, p.p218);
            s.store_div_ad_rhs(1351, 1349, A::add(s.ad_value(1349), s.ad_value(173)));
            s.store_div_from_scalar_offset_input(1347, 1.0, 1353, 1e-50);
            s.store_scaled_mul(1344, 303, 1347, (-p.p214));
        }

        s.b[1366] = (s.v[1344] < (-34.0));
        s.v[1366] = if s.b[1366] { 1.0 } else { 0.0 };

        if (((!s.b[1362]) && s.b[1363]) && s.b[1366]) {
            s.store_scalar(309, 0.0);
        }

        if (((!s.b[1362]) && s.b[1363]) && (!s.b[1366])) {
            s.store_exp(1345, 1344);
            s.store_mul_scale_ad_lhs(1346, A::div_from_scalar(p.p213, s.ad_value(302)), 1.6021918e-19, 1343);
            s.store_div_from_scalar(1348, 1.0, 1361);
            s.store_sqrt_mul_ad(1349, A::add_scaled_inputs(s.ad_value(1360), 1.0, s.ad_value(1356), 1e-12), s.ad_value(1348));
            s.store_mul3_lhs(1347, 1345, 1346, 1349);
            s.store_mul3_lhs(1352, 1347, 1353, 1353);
            s.store_mul3_lhs(309, 1350, 1351, 1352);
        }

        if ((!s.b[1362]) && (!s.b[1363])) {
            s.store_scalar(309, 0.0);
        }

        if (!s.b[1362]) {
            s.store_offset_scaled(1343, 158, (-p.p221), p.p222);
            s.store_exp_scaled_input(1345, 1343, s.v[1355]);
            s.store_scale(1343, 158, (1.0 / (s.v[1355]) * 1.0 / (s.v[1355])));
            s.store_mul(1346, 158, 1343);
            s.store_scale(1347, 1358, (p.p220 / 1000000.0));
            s.store_mul3_lhs(306, 1347, 1345, 1346);
        }

    }

    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1367] = (s.v[158] >= 0.0);
        s.v[1367] = if s.b[1367] { 1.0 } else { 0.0 };

        if ((!s.b[1362]) && s.b[1367]) {
            s.store_scale(306, 306, (-1.0));
        }

        if (!s.b[1362]) {
            s.store_sub(1344, 158, 157);
            s.store_offset_scaled(1343, 1344, (-p.p221), p.p222);
            s.store_exp_scaled_input(1345, 1343, s.v[1355]);
            s.store_scale(1343, 1344, (1.0 / (s.v[1355]) * 1.0 / (s.v[1355])));
            s.store_mul(1346, 1344, 1343);
            s.store_scale(1347, 1358, (p.p220 / 1000000.0));
            s.store_mul3_lhs(307, 1347, 1345, 1346);
        }

        s.b[1368] = (s.v[1344] >= 0.0);
        s.v[1368] = if s.b[1368] { 1.0 } else { 0.0 };

        if ((!s.b[1362]) && s.b[1368]) {
            s.store_scale(307, 307, (-1.0));
        }

        if (!s.b[1362]) {
            s.store_offset_scaled_sub(1353, 513, 158, 1.0 / (s.v[1355]), ((((s.v[123]) + (p.p225))) * (1.0 / (s.v[1355]))));
            s.store_sqrt_square_offset(44, 1353, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_add(1353, 1353, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1369] = (s.v[1353] < 0.0);
        s.v[1369] = if s.b[1369] { 1.0 } else { 0.0 };

        if ((!s.b[1362]) && s.b[1369]) {
            s.store_scalar(1353, 0.0);
        }

        if (!s.b[1362]) {
            s.store_offset(1353, 1353, 1e-50);
            s.store_div_from_scalar(1344, (-p.p224), 1353);
        }

        s.b[1370] = (s.v[1344] < (-34.0));
        s.v[1370] = if s.b[1370] { 1.0 } else { 0.0 };

        if ((!s.b[1362]) && s.b[1370]) {
            s.store_scalar(308, 0.0);
        }

        if ((!s.b[1362]) && (!s.b[1370])) {
            s.store_exp(1345, 1344);
            s.store_scale(1346, 1358, (p.p223 * s.v[1357]));
            s.store_mul_ad_lhs(308, A::mul3(s.ad_value(1346), s.ad_value(1353), s.ad_value(1353)), 1345);
        }

        if (!s.b[1362]) {
            s.store_scalar(310, 0.5);
        }

        s.b[1378] = (p.p28 == 0.0);
        s.v[1378] = if s.b[1378] { 1.0 } else { 0.0 };

        if s.b[1378] {
            s.store_scalar(311, 0.0);
        }

        if (!s.b[1378]) {
            s.store_add_scaled_inputs4_offset(1371, s.ad_value(157), p.p209, s.ad_value(158), (-1.0), s.ad_value(187), p.p211, s.ad_value(319), p.p211, (p.p210 * p.p209));
            s.store_scalar(1372, (1.0 / s.v[88]));
            s.store_mul(1373, 1371, 1372);
            s.store_sqrt_square_offset(44, 1373, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_add(304, 1373, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1379] = (s.v[304] < 0.0);
        s.v[1379] = if s.b[1379] { 1.0 } else { 0.0 };

        if ((!s.b[1378]) && s.b[1379]) {
            s.store_scalar(304, 0.0);
        }

        if (!s.b[1378]) {
            s.store_div_from_scalar_offset_input(1374, 1.0, 304, 1e-50);
            s.store_scaled_mul(1375, 303, 1374, (-p.p208));
        }

        s.b[1380] = (s.v[1375] < (-34.0));
        s.v[1380] = if s.b[1380] { 1.0 } else { 0.0 };

        if ((!s.b[1378]) && s.b[1380]) {
            s.store_scalar(311, 0.0);
        }

        if ((!s.b[1378]) && (!s.b[1380])) {
            s.store_exp(1371, 1375);
            s.store_mul_scale_ad_lhs(1372, A::div_from_scalar(p.p207, s.ad_value(302)), 1.6021918e-19, 107);
            s.store_mul_ad_lhs(311, A::mul3(s.ad_value(1372), s.ad_value(304), s.ad_value(304)), 1371);
        }

        if (!s.b[1378]) {
            s.store_sub(1377, 157, 513);
        }

        s.b[1381] = (s.v[1377] > 0.0);
        s.v[1381] = if s.b[1381] { 1.0 } else { 0.0 };

        if ((!s.b[1378]) && s.b[1381]) {
            s.store_square(1372, 1377);
            s.store_mul(331, 1372, 1377);
            s.store_offset(1375, 331, p.p212);
            s.store_div(1376, 331, 1375);
            s.store_mul(311, 311, 1376);
        }

        if ((!s.b[1378]) && (!s.b[1381])) {
            s.store_scalar(311, 0.0);
        }

        s.b[1389] = (p.p28 == 0.0);
        s.v[1389] = if s.b[1389] { 1.0 } else { 0.0 };

        if s.b[1389] {
            s.store_scalar(312, 0.0);
        }

        if (!s.b[1389]) {
            s.store_add_scaled_inputs3(1382, A::add_scaled_inputs3_offset(s.ad_value(157), (-p.p209), s.ad_value(158), -1.0, s.ad_value(157), 1.0, ((p.p210) * (p.p209))), 1.0, s.ad_value(187), p.p211, s.ad_value(319), p.p211);
            s.store_scalar(1383, (1.0 / s.v[88]));
            s.store_mul(1384, 1382, 1383);
            s.store_sqrt_square_offset(44, 1384, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_add(305, 1384, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1390] = (s.v[305] < 0.0);
        s.v[1390] = if s.b[1390] { 1.0 } else { 0.0 };

        if ((!s.b[1389]) && s.b[1390]) {
            s.store_scalar(305, 0.0);
        }

        if (!s.b[1389]) {
            s.store_div_from_scalar_offset_input(1385, 1.0, 305, 1e-50);
            s.store_scaled_mul(1386, 303, 1385, (-p.p208));
        }

        s.b[1391] = (s.v[1386] < (-34.0));
        s.v[1391] = if s.b[1391] { 1.0 } else { 0.0 };

        if ((!s.b[1389]) && s.b[1391]) {
            s.store_scalar(312, 0.0);
        }

        if ((!s.b[1389]) && (!s.b[1391])) {
            s.store_exp(1382, 1386);
            s.store_div_from_scalar(1385, 1.0, 302);
            s.store_scaled_mul(1383, 1385, 107, (p.p207 * 1.6021918e-19));
            s.store_mul_ad_lhs(312, A::mul3(s.ad_value(1383), s.ad_value(305), s.ad_value(305)), 1382);
        }

        if (!s.b[1389]) {
            s.store_neg(1388, 513);
        }

        s.b[1392] = (s.v[1388] > 0.0);
        s.v[1392] = if s.b[1392] { 1.0 } else { 0.0 };

        if ((!s.b[1389]) && s.b[1392]) {
            s.store_square(1383, 1388);
            s.store_mul(331, 1383, 1388);
            s.store_offset(1386, 331, p.p212);
            s.store_div(1387, 331, 1386);
            s.store_mul(312, 312, 1387);
        }

        if ((!s.b[1389]) && (!s.b[1392])) {
            s.store_scalar(312, 0.0);
        }

        s.b[1393] = (p.p43 == 1.0);
        s.v[1393] = if s.b[1393] { 1.0 } else { 0.0 };

        if s.b[1393] {
            s.store_scalar(1403, s.v[91]);
            s.store_div_from_scalar(1404, 1.0, 1403);
            s.store_scalar(1460, 0.0);
            s.store_scalar(1462, 0.0);
            s.store_scalar(1464, 0.0);
            s.store_neg(1396, 534);
            s.store_mul(1397, 1396, 436);
            s.store_add_scaled_product_indices(331, 1397, 1.0, 1396, 437, 1.0);
            s.store_mul(470, 1397, 438);
            s.store_sub(469, 1397, 470);
            s.store_mul(468, 331, 438);
            s.store_sub(467, 331, 468);
        }

        if (s.b[1393] && (p.p24 != 0.0)) {
            s.copy_ad(521, 536);
            s.store_scalar(528, 0.0);
        }

        s.b[1473] = (1.0 == 1.0);
        s.v[1473] = if s.b[1473] { 1.0 } else { 0.0 };

        s.b[1474] = (1.0 == 2.0);
        s.v[1474] = if s.b[1474] { 1.0 } else { 0.0 };

        if ((s.b[1393] && (p.p24 != 0.0)) && s.b[1473]) {
            s.store_scale(522, 533, 0.5);
            s.store_scalar(523, p.p292);
            s.store_scalar(528, s.v[525]);
        }

        if ((s.b[1393] && (p.p24 != 0.0)) && (s.b[1474] && (!s.b[1473]))) {
            s.store_scale(522, 534, 0.5);
            s.store_scalar(523, p.p68);
            s.store_scalar(528, s.v[524]);
            s.store_scalar(528, 1.0);
        }

        s.b[1475] = (s.v[528] == 0.0);
        s.v[1475] = if s.b[1475] { 1.0 } else { 0.0 };

        if ((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) {
            s.store_mul_sqrt_ad_rhs(1423, 238, A::div(s.ad_value(521), s.ad_value(536)));
            s.store_scalar(1405, ((1.0 - -1.0) / 2.0));
            s.store_scalar(1406, ((1.0 + -1.0) / 2.0));
            s.store_add_scaled_products_right_right_ad(1416, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);
            s.store_add_scaled_products_indices(1417, 461, 157, 1.0, 462, 157, -1.0);
            s.store_add_scaled_products_right_right_ad(1418, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
            s.store_add_scaled_products_right_right_ad(1419, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
            s.store_sub(1420, 1417, 1416);
            s.store_neg(1421, 1416);
            s.store_add_scaled_products_indices(1407, 1405, 461, 1.0, 1406, 462, 1.0);
            s.store_add_scaled_products_indices(1408, 1405, 462, 1.0, 1406, 461, 1.0);
            s.store_add_scaled_products_indices(1422, 1407, 1418, 1.0, 1408, 1419, 1.0);
            s.store_offset_ad(1414, A::add_scaled_products(s.ad_value(1407), s.ad_value(1421), 1.0, s.ad_value(1408), s.ad_value(1420), 1.0), (10.0 * 2.220446049250313e-16));
            s.store_neg(1394, 1414);
        }

        s.b[1476] = (s.v[1394] > s.v[141]);
        s.v[1476] = if s.b[1476] { 1.0 } else { 0.0 };

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1476]) {
            s.store_sub(1395, 1394, 141);
            s.store_sub(1396, 140, 141);
            s.store_div(44, 1395, 1396);
            s.store_square(45, 44);
            s.store_mul(46, 45, 44);
            s.store_square(47, 45);
            s.store_div_from_scalar_ad(1402, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));
            s.store_mul_sub_from_scalar_rhs(1402, 1396, 1.0, 1402);
            s.store_add(1399, 141, 1402);
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1476])) {
            s.copy_ad(1399, 1394);
        }

        if ((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) {
            s.store_offset_scaled(1415, 1399, -1.0, (-1e-12));
            s.store_mul(1424, 1423, 1404);
            s.store_square(1425, 1424);
            s.store_sub(1426, 1422, 523);
            s.store_div(1394, 521, 230);
            s.store_mul_ad(1427, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(s.ad_value(1394)));
            s.store_neg(1428, 1415);
        }

        s.b[1477] = (s.v[1426] < s.v[1428]);
        s.v[1477] = if s.b[1477] { 1.0 } else { 0.0 };

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1477]) {
            s.store_div_from_scalar_mul_ad(1395, 1.0, s.ad_value(225), s.ad_value(1423));
        }

    }

    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1477]) {
            s.store_mul(1402, 1395, 1403);
            s.store_offset_scaled(1429, 1402, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(1430, 1429, 1429, 8.0, 0.0, 1429);
            s.store_sub(1431, 237, 1427);
            s.store_mul_add_rhs(1401, 225, 1426, 1415);
            s.store_sub_from_scalar_ad(1432, (7.0 * 1.414213562373095), A::mul_scaled_lhs(s.ad_value(1402), 9.0, A::offset(s.ad_value(1401), (-2.0))));
            s.store_square(1433, 1432);
        }

        s.b[1478] = (s.v[1430] < (s.v[1433] * 1e-8));
        s.v[1478] = if s.b[1478] { 1.0 } else { 0.0 };

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1477]) && s.b[1478]) {
            s.store_add_scaled_inputs_product_mixed_aaia(1435, A::offset(s.ad_value(1432), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1430), 0.5, s.ad_value(1432), 1.0), 1.0, 1402, A::offset(s.ad_value(1401), (-2.0)), 9.0);
        }

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1477]) && (!s.b[1478])) {
            s.store_sqrt_add(1434, 1430, 1433);
            s.store_add_scaled_offset_product_rhs_mixed_aii(1435, A::offset(s.ad_value(1434), ((-7.0) * 1.414213562373095)), 1.0, 1402, 1401, (-2.0), 9.0);
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1477]) {
            s.store_powf(1436, 1435, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(1437, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1402), 12.0)), 1.0, 1436, 2.0, 1436, 1436, 1.414213562373095);
            s.store_div(1438, 1437, 1436);
            s.store_add_scaled_product_indices(1439, 1415, (-1.0), 1438, 227, 1.0);
            s.store_add(1395, 1439, 1415);
            s.store_div(1396, 1395, 1431);
            s.store_sqrt_square_offset(1397, 1396, 1.0);
            s.store_sub_ad_lhs(1440, A::div(s.ad_value(1395), s.ad_value(1397)), 1415);
            s.store_sub(1396, 1426, 1440);
            s.store_mul(459, 1403, 1396);
            s.copy_ad(458, 459);
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) {
            s.store_scalar(1438, 3.0);
            s.store_sub_ad_lhs(1441, A::div(s.ad_value(1438), s.ad_value(225)), 1415);
            s.store_exp_neg_input(1402, 1438);
            s.store_offset_ad(1401, A::div_scaled_inputs2(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1426), s.ad_value(1415))), (-1.0)), 4.0, s.ad_value(1402), 4.0, A::mul(s.ad_value(1425), s.ad_value(226)), 1.0), 1.0);
        }

        s.b[1479] = (s.v[1401] < (10.0 * 2.220446049250313e-16));
        s.v[1479] = if s.b[1479] { 1.0 } else { 0.0 };

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1479]) {
            s.store_scalar(1401, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) {
            s.store_add_ad_rhs(1441, 1426, A::mul3_scaled_output(s.ad_value(1425), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1401))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1438, 225, 1441, 1415);
            s.store_exp_neg_input(1402, 1438);
            s.store_offset_ad(1401, A::div_scaled_inputs2(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1426), s.ad_value(1415))), (-1.0)), 4.0, s.ad_value(1402), 4.0, A::mul(s.ad_value(1425), s.ad_value(226)), 1.0), 1.0);
        }

        s.b[1480] = (s.v[1401] < (10.0 * 2.220446049250313e-16));
        s.v[1480] = if s.b[1480] { 1.0 } else { 0.0 };

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1480]) {
            s.store_scalar(1401, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) {
            s.store_add_ad_rhs(1441, 1426, A::mul3_scaled_output(s.ad_value(1425), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1401))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1438, 225, 1441, 1415);
        }

        s.b[1481] = (s.v[1438] < 3.0);
        s.v[1481] = if s.b[1481] { 1.0 } else { 0.0 };

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1481]) {
            s.store_scalar(1442, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(1443, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
            s.store_offset_div_from_scalar_ad(1444, 1.0, A::mul(s.ad_value(225), s.ad_value(1424)), (1.0 / 1.414213562373095));
            s.store_div_scaled_inputs2(1445, s.ad_value(1426), -1.0, s.ad_value(1415), -1.0, s.ad_value(1424), 1.0);
            s.store_add_scaled_inputs3(1446, A::div_scaled_product(A::square(s.ad_value(1443)), s.ad_value(1443), 1.0, A::mul3_scaled_output(s.ad_value(1442), s.ad_value(1442), s.ad_value(1442), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1443), s.ad_value(1444), 1.0, s.ad_value(1442), s.ad_value(1442), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(1445), 1.0, s.ad_value(1442), 2.0), 1.0);
            s.store_div_ad(1447, A::add_scaled_square_product(s.ad_value(1443), (-1.0), s.ad_value(1442), s.ad_value(1444), 3.0), A::mul_scaled_lhs(s.ad_value(1442), 9.0, s.ad_value(1442)));
            s.store_sqrt_ad(1398, A::add_scaled_square_product(s.ad_value(1446), 1.0, A::square(s.ad_value(1447)), s.ad_value(1447), 1.0));
            s.store_powf_ad(1448, A::sub(s.ad_value(1398), s.ad_value(1446)), 0.3333333333333333);
            s.store_neg_ad(1449, A::powf(A::add(s.ad_value(1446), s.ad_value(1398)), 0.3333333333333333));
            s.store_add_scaled_inputs3(1401, s.ad_value(1448), 1.0, s.ad_value(1449), 1.0, A::div_scaled_inputs(s.ad_value(1443), 1.0, s.ad_value(1442), 3.0), -1.0);
            s.store_add_scaled_product_indices(1441, 1415, (-1.0), 1401, 227, 1.0);
            s.store_mul_add_rhs(1438, 225, 1441, 1415);
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) {
            s.store_offset_add(1450, 1426, 1415, 0.1);
            s.store_offset_exp_ad(1457, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1415), -1.0), 1e-50);
            s.store_div(1394, 230, 521);
            s.store_square(1451, 1394);
            s.store_mul(1452, 1451, 1457);
            s.store_mul(1394, 226, 1425);
            s.store_mul(1453, 225, 1450);
            s.store_add_scaled_inputs_product_mixed_aaii(1454, A::ln(A::add_scaled_square_product(s.ad_value(1453), 1.0, s.ad_value(1452), s.ad_value(1394), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1451), s.ad_value(1394))), (-1.0), 225, 1415, 1.0);
            s.store_offset_sub(44, 1453, 1454, (-1.0));
            s.store_scale(45, 1453, 4.0);
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1395, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1396, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3(1454, s.ad_value(1453), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
            s.store_sub(1453, 1453, 1454);
            s.store_add_scaled_inputs(1453, 1453, 1.0, 225, 0.1);
            s.store_add_scaled_inputs_product_mixed_aaii(1455, A::ln(A::add_scaled_square_product(s.ad_value(1453), 1.0, s.ad_value(1452), s.ad_value(1394), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1451), s.ad_value(1394))), (-1.0), 225, 1415, 1.0);
            s.copy_ad(1456, 1438);
            s.store_offset_sub(44, 1455, 1456, (-(0.0008 * 75.0)));
            s.store_scale(45, 1455, (4.0 * (0.0008 * 75.0)));
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1395, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1396, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3(1438, s.ad_value(1455), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
            s.store_sub_ad_lhs(1440, A::div(s.ad_value(1438), s.ad_value(225)), 1415);
            s.store_add_ad(1395, A::offset(s.ad_value(1438), (-1.0)), A::exp_scaled_input(s.ad_value(1438), -1.0));
        }

        s.b[1482] = (s.v[1395] < (10.0 * 2.220446049250313e-16));
        s.v[1482] = if s.b[1482] { 1.0 } else { 0.0 };

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1482]) {
            s.store_scalar(1395, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) {
            s.store_sqrt(1396, 1395);
            s.store_mul(458, 1423, 1396);
            s.store_mul_sub_rhs(459, 1403, 1426, 1440);
        }

        s.b[1483] = (p.p42 == 1.0);
        s.v[1483] = if s.b[1483] { 1.0 } else { 0.0 };

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) {
            s.store_exp_ad(1457, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1415), -1.0));
            s.store_div(1394, 230, 521);
            s.store_square(1451, 1394);
            s.store_mul(1466, 1451, 1457);
            s.store_scalar(1411, 0.0);
            s.store_scalar(167, 1.0);
        }

        let mut assign26200_loop_guard: usize = 0;
        while {
            let assign26200_cond_e35786: f64 = (2.0 * 20.0);
            let assign26200_cond_e35788: f64 = (assign26200_cond_e35786 + 1.0);
            let assign26200_cond_e35790: f64 = if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (s.v[167] <= assign26200_cond_e35788)) { 1.0 } else { 0.0 };
            assign26200_cond_e35790 != 0.0
        } {
            assign26200_loop_guard += 1;
            assert!(assign26200_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) {
                s.store_scalar(1462, 0.0);
                s.store_mul_add_rhs(1438, 225, 1440, 1415);
            }
            s.b[1484] = (s.v[1438] < 5.0);
            s.v[1484] = if s.b[1484] { 1.0 } else { 0.0 };
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && s.b[1484]) {
                s.store_ad_value(1458, A::mul3(A::square(s.ad_value(1438)), s.ad_value(1438), A::offset(A::mul(s.ad_value(1438), A::scale_offset(s.ad_value(1438), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771)));
                s.store_ad_value(1459, A::mul_offset_rhs(A::square(s.ad_value(1438)), A::mul(s.ad_value(1438), A::scale_offset(s.ad_value(1438), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
                s.store_mul3_lhs(1460, 1466, 1458, 1458);
                s.store_mul_ad_lhs(1461, A::mul3_scaled_output(s.ad_value(1466), s.ad_value(225), s.ad_value(1458), 2.0), 1459);
                s.store_mul_offset_ad_rhs(1462, 1438, A::mul_offset_rhs(s.ad_value(1438), A::mul_offset_rhs(s.ad_value(1438), A::mul(s.ad_value(1438), A::scale_offset(s.ad_value(1438), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_ad(1463, A::mul_offset_rhs(s.ad_value(1438), A::mul_offset_rhs(s.ad_value(1438), A::mul(s.ad_value(1438), A::scale_offset(s.ad_value(1438), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758))), 0.707106781186548);
                s.store_sqrt_offset_ad(1464, A::add(A::square(s.ad_value(1462)), s.ad_value(1460)), 1e-50);
                s.store_div_scaled_inputs2(1465, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1463), s.ad_value(1462), 2.0), 1.0, s.ad_value(1461), 1.0, s.ad_value(1464), 2.0);
            }
            s.b[1485] = (s.v[1438] < 80.0);
            s.v[1485] = if s.b[1485] { 1.0 } else { 0.0 };
            if ((((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (!s.b[1484])) && s.b[1485]) {
                s.store_exp(243, 1438);
                s.store_mul_offset_rhs(1460, 1466, 243, (-1.0));
                s.store_mul3_lhs(1461, 1466, 225, 243);
            }
            if ((((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (!s.b[1484])) && (!s.b[1485])) {
                s.store_exp_mul(1467, 225, 1440);
                s.store_mul_sub_rhs(1460, 1451, 1467, 1457);
                s.store_mul3_lhs(1461, 1451, 225, 1467);
            }
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (!s.b[1484])) {
                s.store_sqrt_add_ad(1464, A::offset(s.ad_value(1438), (-1.0)), s.ad_value(1460));
                s.store_scale_ad(1465, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1461), 1.0, s.ad_value(1464), 1.0), 0.5);
            }
            if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) {
                s.store_add_scaled_inputs_product_indices(1468, 1426, 1.0, 1440, (-1.0), 1424, 1464, (-1.0));
                s.store_sub_from_scalar_ad(1469, (-1.0), A::mul(s.ad_value(1424), s.ad_value(1465)));
            }
            s.b[1486] = (s.v[1411] == 1.0);
            s.v[1486] = if s.b[1486] { 1.0 } else { 0.0 };
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && s.b[1486]) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (!s.b[1486])) {
                s.store_div_scaled_inputs(494, s.ad_value(1468), -1.0, s.ad_value(1469), 1.0);
            }
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (!s.b[1486])) {
                s.store_scaled_offset_ad(1470, {
                    if (1.0 >= ((s.v[1440]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1440))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1487] = (((s.v[494]) as f64).abs() > s.v[1470]);
            s.v[1487] = if s.b[1487] { 1.0 } else { 0.0 };
            if ((((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (!s.b[1486])) && s.b[1487]) {
                s.store_scale(494, 1470, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (!s.b[1486])) {
                s.store_add(1440, 1440, 494);
            }
            s.b[1488] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1468]) as f64).abs() <= 1e-8));
            s.v[1488] = if s.b[1488] { 1.0 } else { 0.0 };
            if ((((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (!s.b[1486])) && s.b[1488]) {
                s.store_scalar(1411, 1.0);
            }
            if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.b[1490] = (s.v[1438] < 5.0);
        s.v[1490] = if s.b[1490] { 1.0 } else { 0.0 };

        if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && s.b[1490]) {
            s.store_offset_square(1471, 1462, (10.0 * 2.220446049250313e-16));
            s.store_offset(1472, 1462, (10.0 * 2.220446049250313e-16));
        }

        if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (!s.b[1490])) {
            s.store_offset(1471, 1438, (-1.0));
            s.store_sqrt(1472, 1471);
        }

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) {
            s.store_mul(458, 1423, 1472);
            s.store_div_from_scalar_add_ad(1395, 1.0, s.ad_value(1464), s.ad_value(1472));
            s.store_mul3_lhs(460, 1423, 1460, 1395);
            s.store_add(459, 458, 460);
        }

        if ((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) {
            s.store_sub(460, 459, 458);
        }

        s.b[1492] = (1.0 == 1.0);
        s.v[1492] = if s.b[1492] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1493] = (1.0 == 2.0);
        s.v[1493] = if s.b[1493] { 1.0 } else { 0.0 };

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1492]) && (s.v[1405] != 0.0)) {
            s.store_mul_neg_lhs(463, 522, 459);
            s.store_mul_neg_lhs(465, 522, 460);
        }

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1492]) && (s.v[1406] != 0.0)) {
            s.store_mul_neg_lhs(464, 522, 459);
            s.store_mul_neg_lhs(466, 522, 460);
        }

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (s.b[1493] && (!s.b[1492]))) && (s.v[1405] != 0.0)) {
            s.store_mul_neg_lhs(467, 522, 459);
            s.store_mul_neg_lhs(469, 522, 460);
        }

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (s.b[1493] && (!s.b[1492]))) && (s.v[1406] != 0.0)) {
            s.store_mul_neg_lhs(468, 522, 459);
            s.store_mul_neg_lhs(470, 522, 460);
        }

        if ((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) {
            s.store_scalar(1405, ((1.0 - 1.0) / 2.0));
            s.store_scalar(1406, ((1.0 + 1.0) / 2.0));
            s.store_add_scaled_products_right_right_ad(1416, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);
            s.store_add_scaled_products_indices(1417, 461, 157, 1.0, 462, 157, -1.0);
            s.store_add_scaled_products_right_right_ad(1418, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
            s.store_add_scaled_products_right_right_ad(1419, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
            s.store_sub(1420, 1417, 1416);
            s.store_neg(1421, 1416);
            s.store_add_scaled_products_indices(1407, 1405, 461, 1.0, 1406, 462, 1.0);
            s.store_add_scaled_products_indices(1408, 1405, 462, 1.0, 1406, 461, 1.0);
            s.store_add_scaled_products_indices(1422, 1407, 1418, 1.0, 1408, 1419, 1.0);
            s.store_offset_ad(1414, A::add_scaled_products(s.ad_value(1407), s.ad_value(1421), 1.0, s.ad_value(1408), s.ad_value(1420), 1.0), (10.0 * 2.220446049250313e-16));
            s.store_neg(1394, 1414);
        }

        s.b[1494] = (s.v[1394] > s.v[141]);
        s.v[1494] = if s.b[1494] { 1.0 } else { 0.0 };

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1494]) {
            s.store_sub(1395, 1394, 141);
            s.store_sub(1396, 140, 141);
            s.store_div(44, 1395, 1396);
            s.store_square(45, 44);
            s.store_mul(46, 45, 44);
            s.store_square(47, 45);
            s.store_div_from_scalar_ad(1402, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));
            s.store_mul_sub_from_scalar_rhs(1402, 1396, 1.0, 1402);
            s.store_add(1399, 141, 1402);
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1494])) {
            s.copy_ad(1399, 1394);
        }

        if ((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) {
            s.store_offset_scaled(1415, 1399, -1.0, (-1e-12));
            s.store_mul(1424, 1423, 1404);
            s.store_square(1425, 1424);
            s.store_sub(1426, 1422, 523);
            s.store_div(1394, 521, 230);
            s.store_mul_ad(1427, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(s.ad_value(1394)));
            s.store_neg(1428, 1415);
        }

        s.b[1495] = (s.v[1426] < s.v[1428]);
        s.v[1495] = if s.b[1495] { 1.0 } else { 0.0 };

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1495]) {
            s.store_div_from_scalar_mul_ad(1395, 1.0, s.ad_value(225), s.ad_value(1423));
            s.store_mul(1402, 1395, 1403);
            s.store_offset_scaled(1429, 1402, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(1430, 1429, 1429, 8.0, 0.0, 1429);
            s.store_sub(1431, 237, 1427);
            s.store_mul_add_rhs(1401, 225, 1426, 1415);
            s.store_sub_from_scalar_ad(1432, (7.0 * 1.414213562373095), A::mul_scaled_lhs(s.ad_value(1402), 9.0, A::offset(s.ad_value(1401), (-2.0))));
            s.store_square(1433, 1432);
        }

        s.b[1496] = (s.v[1430] < (s.v[1433] * 1e-8));
        s.v[1496] = if s.b[1496] { 1.0 } else { 0.0 };

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1495]) && s.b[1496]) {
            s.store_add_scaled_inputs_product_mixed_aaia(1435, A::offset(s.ad_value(1432), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1430), 0.5, s.ad_value(1432), 1.0), 1.0, 1402, A::offset(s.ad_value(1401), (-2.0)), 9.0);
        }

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1495]) && (!s.b[1496])) {
            s.store_sqrt_add(1434, 1430, 1433);
            s.store_add_scaled_offset_product_rhs_mixed_aii(1435, A::offset(s.ad_value(1434), ((-7.0) * 1.414213562373095)), 1.0, 1402, 1401, (-2.0), 9.0);
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1495]) {
            s.store_powf(1436, 1435, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(1437, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1402), 12.0)), 1.0, 1436, 2.0, 1436, 1436, 1.414213562373095);
            s.store_div(1438, 1437, 1436);
            s.store_add_scaled_product_indices(1439, 1415, (-1.0), 1438, 227, 1.0);
            s.store_add(1395, 1439, 1415);
            s.store_div(1396, 1395, 1431);
            s.store_sqrt_square_offset(1397, 1396, 1.0);
            s.store_sub_ad_lhs(1440, A::div(s.ad_value(1395), s.ad_value(1397)), 1415);
            s.store_sub(1396, 1426, 1440);
            s.store_mul(459, 1403, 1396);
            s.copy_ad(458, 459);
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) {
            s.store_scalar(1438, 3.0);
            s.store_sub_ad_lhs(1441, A::div(s.ad_value(1438), s.ad_value(225)), 1415);
            s.store_exp_neg_input(1402, 1438);
            s.store_offset_ad(1401, A::div_scaled_inputs2(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1426), s.ad_value(1415))), (-1.0)), 4.0, s.ad_value(1402), 4.0, A::mul(s.ad_value(1425), s.ad_value(226)), 1.0), 1.0);
        }

        s.b[1497] = (s.v[1401] < (10.0 * 2.220446049250313e-16));
        s.v[1497] = if s.b[1497] { 1.0 } else { 0.0 };

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1497]) {
            s.store_scalar(1401, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) {
            s.store_add_ad_rhs(1441, 1426, A::mul3_scaled_output(s.ad_value(1425), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1401))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1438, 225, 1441, 1415);
            s.store_exp_neg_input(1402, 1438);
            s.store_offset_ad(1401, A::div_scaled_inputs2(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1426), s.ad_value(1415))), (-1.0)), 4.0, s.ad_value(1402), 4.0, A::mul(s.ad_value(1425), s.ad_value(226)), 1.0), 1.0);
        }

        s.b[1498] = (s.v[1401] < (10.0 * 2.220446049250313e-16));
        s.v[1498] = if s.b[1498] { 1.0 } else { 0.0 };

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1498]) {
            s.store_scalar(1401, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) {
            s.store_add_ad_rhs(1441, 1426, A::mul3_scaled_output(s.ad_value(1425), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1401))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1438, 225, 1441, 1415);
        }

        s.b[1499] = (s.v[1438] < 3.0);
        s.v[1499] = if s.b[1499] { 1.0 } else { 0.0 };

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1499]) {
            s.store_scalar(1442, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(1443, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
            s.store_offset_div_from_scalar_ad(1444, 1.0, A::mul(s.ad_value(225), s.ad_value(1424)), (1.0 / 1.414213562373095));
            s.store_div_scaled_inputs2(1445, s.ad_value(1426), -1.0, s.ad_value(1415), -1.0, s.ad_value(1424), 1.0);
            s.store_add_scaled_inputs3(1446, A::div_scaled_product(A::square(s.ad_value(1443)), s.ad_value(1443), 1.0, A::mul3_scaled_output(s.ad_value(1442), s.ad_value(1442), s.ad_value(1442), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1443), s.ad_value(1444), 1.0, s.ad_value(1442), s.ad_value(1442), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(1445), 1.0, s.ad_value(1442), 2.0), 1.0);
            s.store_div_ad(1447, A::add_scaled_square_product(s.ad_value(1443), (-1.0), s.ad_value(1442), s.ad_value(1444), 3.0), A::mul_scaled_lhs(s.ad_value(1442), 9.0, s.ad_value(1442)));
            s.store_sqrt_ad(1398, A::add_scaled_square_product(s.ad_value(1446), 1.0, A::square(s.ad_value(1447)), s.ad_value(1447), 1.0));
            s.store_powf_ad(1448, A::sub(s.ad_value(1398), s.ad_value(1446)), 0.3333333333333333);
            s.store_neg_ad(1449, A::powf(A::add(s.ad_value(1446), s.ad_value(1398)), 0.3333333333333333));
            s.store_add_scaled_inputs3(1401, s.ad_value(1448), 1.0, s.ad_value(1449), 1.0, A::div_scaled_inputs(s.ad_value(1443), 1.0, s.ad_value(1442), 3.0), -1.0);
            s.store_add_scaled_product_indices(1441, 1415, (-1.0), 1401, 227, 1.0);
            s.store_mul_add_rhs(1438, 225, 1441, 1415);
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) {
            s.store_offset_add(1450, 1426, 1415, 0.1);
            s.store_offset_exp_ad(1457, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1415), -1.0), 1e-50);
            s.store_div(1394, 230, 521);
            s.store_square(1451, 1394);
            s.store_mul(1452, 1451, 1457);
            s.store_mul(1394, 226, 1425);
            s.store_mul(1453, 225, 1450);
            s.store_add_scaled_inputs_product_mixed_aaii(1454, A::ln(A::add_scaled_square_product(s.ad_value(1453), 1.0, s.ad_value(1452), s.ad_value(1394), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1451), s.ad_value(1394))), (-1.0), 225, 1415, 1.0);
            s.store_offset_sub(44, 1453, 1454, (-1.0));
            s.store_scale(45, 1453, 4.0);
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1395, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1396, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3(1454, s.ad_value(1453), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
            s.store_sub(1453, 1453, 1454);
            s.store_add_scaled_inputs(1453, 1453, 1.0, 225, 0.1);
            s.store_add_scaled_inputs_product_mixed_aaii(1455, A::ln(A::add_scaled_square_product(s.ad_value(1453), 1.0, s.ad_value(1452), s.ad_value(1394), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1451), s.ad_value(1394))), (-1.0), 225, 1415, 1.0);
            s.copy_ad(1456, 1438);
            s.store_offset_sub(44, 1455, 1456, (-(0.0008 * 75.0)));
            s.store_scale(45, 1455, (4.0 * (0.0008 * 75.0)));
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1395, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1396, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3(1438, s.ad_value(1455), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
            s.store_sub_ad_lhs(1440, A::div(s.ad_value(1438), s.ad_value(225)), 1415);
            s.store_add_ad(1395, A::offset(s.ad_value(1438), (-1.0)), A::exp_scaled_input(s.ad_value(1438), -1.0));
        }

        s.b[1500] = (s.v[1395] < (10.0 * 2.220446049250313e-16));
        s.v[1500] = if s.b[1500] { 1.0 } else { 0.0 };

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1500]) {
            s.store_scalar(1395, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) {
            s.store_sqrt(1396, 1395);
            s.store_mul(458, 1423, 1396);
            s.store_mul_sub_rhs(459, 1403, 1426, 1440);
        }

        s.b[1501] = (p.p42 == 1.0);
        s.v[1501] = if s.b[1501] { 1.0 } else { 0.0 };

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) {
            s.store_exp_ad(1457, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1415), -1.0));
            s.store_div(1394, 230, 521);
        }

    }

    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) {
            s.store_square(1451, 1394);
            s.store_mul(1466, 1451, 1457);
            s.store_scalar(1411, 0.0);
            s.store_scalar(167, 1.0);
        }

        let mut assign27750_loop_guard: usize = 0;
        while {
            let assign27750_cond_e38729: f64 = (2.0 * 20.0);
            let assign27750_cond_e38731: f64 = (assign27750_cond_e38729 + 1.0);
            let assign27750_cond_e38733: f64 = if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (s.v[167] <= assign27750_cond_e38731)) { 1.0 } else { 0.0 };
            assign27750_cond_e38733 != 0.0
        } {
            assign27750_loop_guard += 1;
            assert!(assign27750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) {
                s.store_scalar(1462, 0.0);
                s.store_mul_add_rhs(1438, 225, 1440, 1415);
            }
            s.b[1502] = (s.v[1438] < 5.0);
            s.v[1502] = if s.b[1502] { 1.0 } else { 0.0 };
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && s.b[1502]) {
                s.store_ad_value(1458, A::mul3(A::square(s.ad_value(1438)), s.ad_value(1438), A::offset(A::mul(s.ad_value(1438), A::scale_offset(s.ad_value(1438), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771)));
                s.store_ad_value(1459, A::mul_offset_rhs(A::square(s.ad_value(1438)), A::mul(s.ad_value(1438), A::scale_offset(s.ad_value(1438), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
                s.store_mul3_lhs(1460, 1466, 1458, 1458);
                s.store_mul_ad_lhs(1461, A::mul3_scaled_output(s.ad_value(1466), s.ad_value(225), s.ad_value(1458), 2.0), 1459);
                s.store_mul_offset_ad_rhs(1462, 1438, A::mul_offset_rhs(s.ad_value(1438), A::mul_offset_rhs(s.ad_value(1438), A::mul(s.ad_value(1438), A::scale_offset(s.ad_value(1438), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_ad(1463, A::mul_offset_rhs(s.ad_value(1438), A::mul_offset_rhs(s.ad_value(1438), A::mul(s.ad_value(1438), A::scale_offset(s.ad_value(1438), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758))), 0.707106781186548);
                s.store_sqrt_offset_ad(1464, A::add(A::square(s.ad_value(1462)), s.ad_value(1460)), 1e-50);
                s.store_div_scaled_inputs2(1465, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1463), s.ad_value(1462), 2.0), 1.0, s.ad_value(1461), 1.0, s.ad_value(1464), 2.0);
            }
            s.b[1503] = (s.v[1438] < 80.0);
            s.v[1503] = if s.b[1503] { 1.0 } else { 0.0 };
            if ((((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1502])) && s.b[1503]) {
                s.store_exp(243, 1438);
                s.store_mul_offset_rhs(1460, 1466, 243, (-1.0));
                s.store_mul3_lhs(1461, 1466, 225, 243);
            }
            if ((((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1502])) && (!s.b[1503])) {
                s.store_exp_mul(1467, 225, 1440);
                s.store_mul_sub_rhs(1460, 1451, 1467, 1457);
                s.store_mul3_lhs(1461, 1451, 225, 1467);
            }
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1502])) {
                s.store_sqrt_add_ad(1464, A::offset(s.ad_value(1438), (-1.0)), s.ad_value(1460));
                s.store_scale_ad(1465, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1461), 1.0, s.ad_value(1464), 1.0), 0.5);
            }
            if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) {
                s.store_add_scaled_inputs_product_indices(1468, 1426, 1.0, 1440, (-1.0), 1424, 1464, (-1.0));
                s.store_sub_from_scalar_ad(1469, (-1.0), A::mul(s.ad_value(1424), s.ad_value(1465)));
            }
            s.b[1504] = (s.v[1411] == 1.0);
            s.v[1504] = if s.b[1504] { 1.0 } else { 0.0 };
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && s.b[1504]) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1504])) {
                s.store_div_scaled_inputs(494, s.ad_value(1468), -1.0, s.ad_value(1469), 1.0);
            }
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1504])) {
                s.store_scaled_offset_ad(1470, {
                    if (1.0 >= ((s.v[1440]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1440))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1505] = (((s.v[494]) as f64).abs() > s.v[1470]);
            s.v[1505] = if s.b[1505] { 1.0 } else { 0.0 };
            if ((((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1504])) && s.b[1505]) {
                s.store_scale(494, 1470, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1504])) {
                s.store_add(1440, 1440, 494);
            }
            s.b[1506] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1468]) as f64).abs() <= 1e-8));
            s.v[1506] = if s.b[1506] { 1.0 } else { 0.0 };
            if ((((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1504])) && s.b[1506]) {
                s.store_scalar(1411, 1.0);
            }
            if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.b[1508] = (s.v[1438] < 5.0);
        s.v[1508] = if s.b[1508] { 1.0 } else { 0.0 };

        if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && s.b[1508]) {
            s.store_offset_square(1471, 1462, (10.0 * 2.220446049250313e-16));
            s.store_offset(1472, 1462, (10.0 * 2.220446049250313e-16));
        }

        if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1508])) {
            s.store_offset(1471, 1438, (-1.0));
            s.store_sqrt(1472, 1471);
        }

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) {
            s.store_mul(458, 1423, 1472);
            s.store_div_from_scalar_add_ad(1395, 1.0, s.ad_value(1464), s.ad_value(1472));
            s.store_mul3_lhs(460, 1423, 1460, 1395);
            s.store_add(459, 458, 460);
        }

        if ((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) {
            s.store_sub(460, 459, 458);
        }

        s.b[1510] = (1.0 == 1.0);
        s.v[1510] = if s.b[1510] { 1.0 } else { 0.0 };

        s.b[1511] = (1.0 == 2.0);
        s.v[1511] = if s.b[1511] { 1.0 } else { 0.0 };

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1510]) && (s.v[1405] != 0.0)) {
            s.store_mul_neg_lhs(463, 522, 459);
            s.store_mul_neg_lhs(465, 522, 460);
        }

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1510]) && (s.v[1406] != 0.0)) {
            s.store_mul_neg_lhs(464, 522, 459);
            s.store_mul_neg_lhs(466, 522, 460);
        }

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (s.b[1511] && (!s.b[1510]))) && (s.v[1405] != 0.0)) {
            s.store_mul_neg_lhs(467, 522, 459);
            s.store_mul_neg_lhs(469, 522, 460);
        }

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (s.b[1511] && (!s.b[1510]))) && (s.v[1406] != 0.0)) {
            s.store_mul_neg_lhs(468, 522, 459);
            s.store_mul_neg_lhs(470, 522, 460);
        }

        s.v[317] = p.p189;

        s.b[1514] = (s.v[145] != 0.0);
        s.v[1514] = if s.b[1514] { 1.0 } else { 0.0 };

        if s.b[1514] {
            s.store_add(1513, 157, 161);
            s.store_add_scaled_inputs(314, 1513, s.v[317], 162, (1.0 - s.v[317]));
        }

        s.b[1515] = (p.p64 != 0.0);
        s.v[1515] = if s.b[1515] { 1.0 } else { 0.0 };

        if (s.b[1514] && s.b[1515]) {
            s.store_scalar(315, 0.0);
        }

        s.b[1516] = (s.v[314] > ((s.v[161] + s.v[157]) - (10.0 * 2.220446049250313e-16)));
        s.v[1516] = if s.b[1516] { 1.0 } else { 0.0 };

        if (s.b[1514] && s.b[1516]) {
            s.store_offset_add(314, 161, 157, (-(10.0 * 2.220446049250313e-16)));
        }

        s.b[1517] = (p.p64 != 0.0);
        s.v[1517] = if s.b[1517] { 1.0 } else { 0.0 };

        s.b[1518] = (s.v[246] < 1e-15);
        s.v[1518] = if s.b[1518] { 1.0 } else { 0.0 };

        if (((!s.b[1514]) && s.b[1517]) && s.b[1518]) {
            s.store_scalar(315, 0.0);
        }

        if (((!s.b[1514]) && s.b[1517]) && (!s.b[1518])) {
            s.store_scale(1512, 227, 1.0 / (s.v[97]));
            s.store_div_from_scalar(1513, 1.0, 244);
            s.store_mul3_lhs(315, 246, 1512, 1513);
        }

        s.v[1530] = s.v[91];

        s.v[1531] = (1.0 / s.v[1530]);

        s.v[1551] = 0.0;

        s.v[1591] = 0.0;

        s.v[1589] = 0.0;

        s.v[1593] = 0.0;

        s.b[1602] = ((p.p29 >= 1.0) && (p.p188 > 0.0));
        s.v[1602] = if s.b[1602] { 1.0 } else { 0.0 };

        if ((p.p24 != 0.0) && s.b[1602]) {
            s.store_scalar(1533, p.p171);
            s.store_scalar(1534, p.p172);
            s.copy_ad(1535, 158);
            s.store_scalar(1532, p.p188);
        }

        s.b[1603] = ((s.v[69] == 0.0) && (p.p188 > 0.0));
        s.v[1603] = if s.b[1603] { 1.0 } else { 0.0 };

        if (((p.p24 != 0.0) && s.b[1602]) && s.b[1603]) {
            if (p.p43 == 1.0) {
                s.store_scale(1520, 287, s.v[1530]);
            } else {
                s.store_scale(1520, 108, s.v[1530]);
            }
        }

        if (((p.p24 != 0.0) && s.b[1602]) && s.b[1603]) {
            s.store_mul_ad_product_rhs(1523, 1533, s.ad_value(1520), A::add(s.ad_value(1534), s.ad_value(1535)));
            s.store_mul(1524, 1532, 1520);
            s.copy_ad(1528, 161);
            s.store_sub_from_scalar(1525, 1.2, 1528);
            s.store_add_scaled_products_indices(267, 158, 1524, 1.0, 1525, 1523, (-1.0));
            s.store_mul_ad_product_rhs(1523, 1533, s.ad_value(1520), A::add_scaled_inputs3(s.ad_value(1534), 1.0, s.ad_value(1535), 1.0, s.ad_value(157), -1.0));
            s.store_sub(1528, 162, 157);
            s.store_sub_from_scalar(1525, 1.2, 1528);
            s.store_add_scaled_products_left_left_ad(268, A::sub(s.ad_value(158), s.ad_value(157)), 1524, 1.0, 1523, 1525, (-1.0));
        }

        if (((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) {
            s.store_mul_sqrt_ad_rhs(1552, 238, A::div_from_scalar(s.v[69], s.ad_value(536)));
            s.store_scalar(1536, ((1.0 - -1.0) / 2.0));
            s.store_scalar(1537, ((1.0 + -1.0) / 2.0));
        }

        s.b[1604] = (p.p43 == 1.0);
        s.v[1604] = if s.b[1604] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1604]) {
            s.store_add_scaled_products_right_right_ad(1546, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);
            s.store_add_scaled_products_indices(1547, 461, 157, 1.0, 462, 157, -1.0);
            s.store_add_scaled_products_right_right_ad(1548, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
            s.store_sub(1549, 1547, 1546);
            s.store_sub(1551, 1548, 1546);
            s.store_neg(1550, 1546);
            s.store_add_scaled_products_indices(1538, 1536, 461, 1.0, 1537, 462, 1.0);
            s.store_add_scaled_products_indices(1539, 1536, 462, 1.0, 1537, 461, 1.0);
            s.store_offset_ad(1544, A::add_scaled_products(s.ad_value(1538), s.ad_value(1550), 1.0, s.ad_value(1539), s.ad_value(1549), 1.0), (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1604])) {
            s.store_add_scaled_products_indices(1538, 1536, 461, 1.0, 1537, 462, 1.0);
            s.store_add_scaled_products_indices(1539, 1536, 462, 1.0, 1537, 461, 1.0);
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1604])) && (s.v[1536] != 0.0)) {
            s.store_add_scaled_products_right_right_ad(1551, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1604])) && (s.v[1537] != 0.0)) {
            s.store_add_scaled_products_right_right_ad(1551, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1604])) {
            s.store_scalar(1544, 0.0);
        }

        if (((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) {
            s.store_neg(1519, 1544);
        }

        s.b[1605] = (s.v[1519] > s.v[141]);
        s.v[1605] = if s.b[1605] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1605]) {
            s.store_sub(1520, 1519, 141);
            s.store_sub(1521, 140, 141);
            s.store_div(44, 1520, 1521);
            s.store_square(45, 44);
            s.store_mul(46, 45, 44);
            s.store_square(47, 45);
            s.store_div_from_scalar_ad(1529, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));
            s.store_mul_sub_from_scalar_rhs(1529, 1521, 1.0, 1529);
            s.store_add(1526, 141, 1529);
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1605])) {
            s.copy_ad(1526, 1519);
        }

        if (((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) {
            s.store_offset_scaled(1545, 1526, -1.0, (-1e-12));
            s.store_scale(1553, 1552, s.v[1531]);
            s.store_square(1554, 1553);
            s.store_sub_from_scalar(1555, s.v[82], 1551);
            s.store_div_from_scalar(1519, s.v[69], 230);
            s.store_mul_ad(1556, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(s.ad_value(1519)));
            s.store_neg(1557, 1545);
        }

        s.b[1606] = (s.v[1555] < s.v[1557]);
        s.v[1606] = if s.b[1606] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1606]) {
            s.store_div_from_scalar_mul_ad(1520, 1.0, s.ad_value(225), s.ad_value(1552));
            s.store_scale(1529, 1520, s.v[1530]);
            s.store_offset_scaled(1558, 1529, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(1559, 1558, 1558, 8.0, 0.0, 1558);
            s.store_sub(1560, 237, 1556);
            s.store_mul_add_rhs(1528, 225, 1555, 1545);
            s.store_sub_from_scalar_ad(1561, (7.0 * 1.414213562373095), A::mul_scaled_lhs(s.ad_value(1529), 9.0, A::offset(s.ad_value(1528), (-2.0))));
            s.store_square(1562, 1561);
        }

    }

    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1607] = (s.v[1559] < (s.v[1562] * 1e-8));
        s.v[1607] = if s.b[1607] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1606]) && s.b[1607]) {
            s.store_add_scaled_inputs_product_mixed_aaia(1564, A::offset(s.ad_value(1561), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1559), 0.5, s.ad_value(1561), 1.0), 1.0, 1529, A::offset(s.ad_value(1528), (-2.0)), 9.0);
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1606]) && (!s.b[1607])) {
            s.store_sqrt_add(1563, 1559, 1562);
            s.store_add_scaled_offset_product_rhs_mixed_aii(1564, A::offset(s.ad_value(1563), ((-7.0) * 1.414213562373095)), 1.0, 1529, 1528, (-2.0), 9.0);
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1606]) {
            s.store_powf(1565, 1564, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(1566, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1529), 12.0)), 1.0, 1565, 2.0, 1565, 1565, 1.414213562373095);
            s.store_div(1567, 1566, 1565);
            s.store_add_scaled_product_indices(1568, 1545, (-1.0), 1567, 227, 1.0);
            s.store_add(1520, 1568, 1545);
            s.store_div(1521, 1520, 1560);
            s.store_sqrt_square_offset(1522, 1521, 1.0);
            s.store_sub_ad_lhs(1569, A::div(s.ad_value(1520), s.ad_value(1522)), 1545);
            s.store_sub(1521, 1555, 1569);
            s.store_scale(459, 1521, s.v[1530]);
            s.copy_ad(458, 459);
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) {
            s.store_scalar(1567, 3.0);
            s.store_sub_ad_lhs(1570, A::div(s.ad_value(1567), s.ad_value(225)), 1545);
            s.store_exp_neg_input(1529, 1567);
            s.store_offset_ad(1528, A::div_scaled_inputs2(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1555), s.ad_value(1545))), (-1.0)), 4.0, s.ad_value(1529), 4.0, A::mul(s.ad_value(1554), s.ad_value(226)), 1.0), 1.0);
        }

        s.b[1608] = (s.v[1528] < (10.0 * 2.220446049250313e-16));
        s.v[1608] = if s.b[1608] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1608]) {
            s.store_scalar(1528, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) {
            s.store_add_ad_rhs(1570, 1555, A::mul3_scaled_output(s.ad_value(1554), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1528))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1567, 225, 1570, 1545);
            s.store_exp_neg_input(1529, 1567);
            s.store_offset_ad(1528, A::div_scaled_inputs2(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1555), s.ad_value(1545))), (-1.0)), 4.0, s.ad_value(1529), 4.0, A::mul(s.ad_value(1554), s.ad_value(226)), 1.0), 1.0);
        }

        s.b[1609] = (s.v[1528] < (10.0 * 2.220446049250313e-16));
        s.v[1609] = if s.b[1609] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1609]) {
            s.store_scalar(1528, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) {
            s.store_add_ad_rhs(1570, 1555, A::mul3_scaled_output(s.ad_value(1554), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1528))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1567, 225, 1570, 1545);
        }

        s.b[1610] = (s.v[1567] < 3.0);
        s.v[1610] = if s.b[1610] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1610]) {
            s.store_scalar(1571, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(1572, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
            s.store_offset_div_from_scalar_ad(1573, 1.0, A::mul(s.ad_value(225), s.ad_value(1553)), (1.0 / 1.414213562373095));
            s.store_div_scaled_inputs2(1574, s.ad_value(1555), -1.0, s.ad_value(1545), -1.0, s.ad_value(1553), 1.0);
            s.store_add_scaled_inputs3(1575, A::div_scaled_product(A::square(s.ad_value(1572)), s.ad_value(1572), 1.0, A::mul3_scaled_output(s.ad_value(1571), s.ad_value(1571), s.ad_value(1571), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1572), s.ad_value(1573), 1.0, s.ad_value(1571), s.ad_value(1571), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(1574), 1.0, s.ad_value(1571), 2.0), 1.0);
            s.store_div_ad(1576, A::add_scaled_square_product(s.ad_value(1572), (-1.0), s.ad_value(1571), s.ad_value(1573), 3.0), A::mul_scaled_lhs(s.ad_value(1571), 9.0, s.ad_value(1571)));
            s.store_sqrt_ad(1524, A::add_scaled_square_product(s.ad_value(1575), 1.0, A::square(s.ad_value(1576)), s.ad_value(1576), 1.0));
            s.store_powf_ad(1577, A::sub(s.ad_value(1524), s.ad_value(1575)), 0.3333333333333333);
            s.store_neg_ad(1578, A::powf(A::add(s.ad_value(1575), s.ad_value(1524)), 0.3333333333333333));
            s.store_add_scaled_inputs3(1528, s.ad_value(1577), 1.0, s.ad_value(1578), 1.0, A::div_scaled_inputs(s.ad_value(1572), 1.0, s.ad_value(1571), 3.0), -1.0);
            s.store_add_scaled_product_indices(1570, 1545, (-1.0), 1528, 227, 1.0);
            s.store_mul_add_rhs(1567, 225, 1570, 1545);
        }

        s.b[1611] = (p.p41 > 0.0);
        s.v[1611] = if s.b[1611] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1611]) {
            s.store_offset_add(1579, 1555, 1545, 0.1);
            s.store_offset_exp_ad(1586, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1545), -1.0), 1e-50);
            s.store_scale(1519, 230, 1.0 / (s.v[69]));
            s.store_square(1580, 1519);
            s.store_mul(1581, 1580, 1586);
            s.store_mul(1519, 226, 1554);
            s.store_mul(1582, 225, 1579);
            s.store_add_scaled_inputs_product_mixed_aaii(1583, A::ln(A::add_scaled_square_product(s.ad_value(1582), 1.0, s.ad_value(1581), s.ad_value(1519), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1580), s.ad_value(1519))), (-1.0), 225, 1545, 1.0);
            s.store_offset_sub(44, 1582, 1583, (-1.0));
            s.store_scale(45, 1582, 4.0);
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1611]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1611]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1520, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1521, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3(1583, s.ad_value(1582), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
            s.store_sub(1582, 1582, 1583);
            s.store_add_scaled_inputs(1582, 1582, 1.0, 225, 0.1);
            s.store_add_scaled_inputs_product_mixed_aaii(1584, A::ln(A::add_scaled_square_product(s.ad_value(1582), 1.0, s.ad_value(1581), s.ad_value(1519), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1580), s.ad_value(1519))), (-1.0), 225, 1545, 1.0);
            s.copy_ad(1585, 1567);
            s.store_offset_sub(44, 1584, 1585, (-(0.0008 * 75.0)));
            s.store_scale(45, 1584, (4.0 * (0.0008 * 75.0)));
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1611]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1611]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1520, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1521, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3(1567, s.ad_value(1584), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) {
            s.store_sub_ad_lhs(1569, A::div(s.ad_value(1567), s.ad_value(225)), 1545);
            s.store_add_ad(1520, A::offset(s.ad_value(1567), (-1.0)), A::exp_scaled_input(s.ad_value(1567), -1.0));
        }

        s.b[1612] = (s.v[1520] < (10.0 * 2.220446049250313e-16));
        s.v[1612] = if s.b[1612] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1612]) {
            s.store_scalar(1520, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) {
            s.store_sqrt(1521, 1520);
            s.store_mul(458, 1552, 1521);
            s.store_scaled_sub(459, 1555, 1569, s.v[1530]);
        }

        s.b[1613] = (p.p41 == 1.0);
        s.v[1613] = if s.b[1613] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) {
            s.store_exp_ad(1586, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1545), -1.0));
            s.store_scale(1519, 230, 1.0 / (s.v[69]));
            s.store_square(1580, 1519);
            s.store_mul(1595, 1580, 1586);
            s.store_scalar(1542, 0.0);
            s.store_scalar(1589, 0.0);
            s.store_scalar(1593, 0.0);
            s.store_scalar(167, 1.0);
        }

        let mut assign29750_loop_guard: usize = 0;
        while {
            let assign29750_cond_e42262: f64 = (2.0 * 20.0);
            let assign29750_cond_e42264: f64 = (assign29750_cond_e42262 + 1.0);
            let assign29750_cond_e42266: f64 = if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (s.v[167] <= assign29750_cond_e42264)) { 1.0 } else { 0.0 };
            assign29750_cond_e42266 != 0.0
        } {
            assign29750_loop_guard += 1;
            assert!(assign29750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) {
                s.store_scalar(1591, 0.0);
                s.store_mul_add_rhs(1567, 225, 1569, 1545);
            }
            s.b[1614] = (s.v[1567] < 5.0);
            s.v[1614] = if s.b[1614] { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && s.b[1614]) {
                s.store_ad_value(1587, A::mul3(A::square(s.ad_value(1567)), s.ad_value(1567), A::offset(A::mul(s.ad_value(1567), A::scale_offset(s.ad_value(1567), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771)));
                s.store_ad_value(1588, A::mul_offset_rhs(A::square(s.ad_value(1567)), A::mul(s.ad_value(1567), A::scale_offset(s.ad_value(1567), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
                s.store_mul3_lhs(1589, 1595, 1587, 1587);
                s.store_mul_ad_lhs(1590, A::mul3_scaled_output(s.ad_value(1595), s.ad_value(225), s.ad_value(1587), 2.0), 1588);
                s.store_mul_offset_ad_rhs(1591, 1567, A::mul_offset_rhs(s.ad_value(1567), A::mul_offset_rhs(s.ad_value(1567), A::mul(s.ad_value(1567), A::scale_offset(s.ad_value(1567), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_ad(1592, A::mul_offset_rhs(s.ad_value(1567), A::mul_offset_rhs(s.ad_value(1567), A::mul(s.ad_value(1567), A::scale_offset(s.ad_value(1567), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758))), 0.707106781186548);
                s.store_sqrt_offset_ad(1593, A::add(A::square(s.ad_value(1591)), s.ad_value(1589)), 1e-50);
                s.store_div_scaled_inputs2(1594, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1592), s.ad_value(1591), 2.0), 1.0, s.ad_value(1590), 1.0, s.ad_value(1593), 2.0);
            }
            s.b[1615] = (s.v[1567] < 80.0);
            s.v[1615] = if s.b[1615] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1614])) && s.b[1615]) {
                s.store_exp(243, 1567);
                s.store_mul_offset_rhs(1589, 1595, 243, (-1.0));
                s.store_mul3_lhs(1590, 1595, 225, 243);
            }
            if (((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1614])) && (!s.b[1615])) {
                s.store_exp_mul(1596, 225, 1569);
                s.store_mul_sub_rhs(1589, 1580, 1596, 1586);
                s.store_mul3_lhs(1590, 1580, 225, 1596);
            }
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1614])) {
                s.store_sqrt_add_ad(1593, A::offset(s.ad_value(1567), (-1.0)), s.ad_value(1589));
                s.store_scale_ad(1594, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1590), 1.0, s.ad_value(1593), 1.0), 0.5);
            }
            if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) {
                s.store_add_scaled_inputs_product_indices(1597, 1555, 1.0, 1569, (-1.0), 1553, 1593, (-1.0));
                s.store_sub_from_scalar_ad(1598, (-1.0), A::mul(s.ad_value(1553), s.ad_value(1594)));
            }
            s.b[1616] = (s.v[1542] == 1.0);
            s.v[1616] = if s.b[1616] { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && s.b[1616]) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1616])) {
                s.store_div_scaled_inputs(494, s.ad_value(1597), -1.0, s.ad_value(1598), 1.0);
            }
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1616])) {
                s.store_scaled_offset_ad(1599, {
                    if (1.0 >= ((s.v[1569]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1569))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1617] = (((s.v[494]) as f64).abs() > s.v[1599]);
            s.v[1617] = if s.b[1617] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1616])) && s.b[1617]) {
                s.store_scale(494, 1599, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1616])) {
                s.store_add(1569, 1569, 494);
            }
            s.b[1618] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1597]) as f64).abs() <= 1e-8));
            s.v[1618] = if s.b[1618] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1616])) && s.b[1618]) {
                s.store_scalar(1542, 1.0);
            }
            if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.b[1620] = (s.v[1567] < 5.0);
        s.v[1620] = if s.b[1620] { 1.0 } else { 0.0 };

        if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && s.b[1620]) {
            s.store_offset_square(1600, 1591, (10.0 * 2.220446049250313e-16));
            s.store_offset(1601, 1591, (10.0 * 2.220446049250313e-16));
        }

        if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1620])) {
            s.store_offset(1600, 1567, (-1.0));
            s.store_sqrt(1601, 1600);
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) {
            s.store_mul(458, 1552, 1601);
            s.store_div_from_scalar_add_ad(1520, 1.0, s.ad_value(1593), s.ad_value(1601));
            s.store_mul3_lhs(460, 1552, 1589, 1520);
            s.store_add(459, 458, 460);
        }

        if (((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) {
            s.store_sub(460, 459, 458);
        }

        if (((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) {
            if (p.p43 == 1.0) {
                s.store_mul(1523, 287, 1532);
            } else {
                s.store_mul(1523, 108, 1532);
            }
        }

        s.b[1622] = (((s.v[1538] != 0.0) && (p.p43 == 0.0)) || ((s.v[1536] != 0.0) && (p.p43 == 1.0)));
        s.v[1622] = if s.b[1622] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1622]) {
            s.store_mul(455, 1523, 459);
        }

    }

    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1622]) {
            s.store_mul(457, 1523, 458);
        }

        s.b[1623] = (((s.v[1539] != 0.0) && (p.p43 == 0.0)) || ((s.v[1537] != 0.0) && (p.p43 == 1.0)));
        s.v[1623] = if s.b[1623] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1623]) {
            s.store_mul(454, 1523, 459);
            s.store_mul(456, 1523, 458);
        }

        if (((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) {
            s.store_scalar(1536, ((1.0 - 1.0) / 2.0));
            s.store_scalar(1537, ((1.0 + 1.0) / 2.0));
        }

        s.b[1624] = (p.p43 == 1.0);
        s.v[1624] = if s.b[1624] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1624]) {
            s.store_add_scaled_products_right_right_ad(1546, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);
            s.store_add_scaled_products_indices(1547, 461, 157, 1.0, 462, 157, -1.0);
            s.store_add_scaled_products_right_right_ad(1548, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
            s.store_sub(1549, 1547, 1546);
            s.store_sub(1551, 1548, 1546);
            s.store_neg(1550, 1546);
            s.store_add_scaled_products_indices(1538, 1536, 461, 1.0, 1537, 462, 1.0);
            s.store_add_scaled_products_indices(1539, 1536, 462, 1.0, 1537, 461, 1.0);
            s.store_offset_ad(1544, A::add_scaled_products(s.ad_value(1538), s.ad_value(1550), 1.0, s.ad_value(1539), s.ad_value(1549), 1.0), (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1624])) {
            s.store_add_scaled_products_indices(1538, 1536, 461, 1.0, 1537, 462, 1.0);
            s.store_add_scaled_products_indices(1539, 1536, 462, 1.0, 1537, 461, 1.0);
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1624])) && (s.v[1536] != 0.0)) {
            s.store_add_scaled_products_right_right_ad(1551, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1624])) && (s.v[1537] != 0.0)) {
            s.store_add_scaled_products_right_right_ad(1551, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1624])) {
            s.store_scalar(1544, 0.0);
        }

        if (((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) {
            s.store_neg(1519, 1544);
        }

        s.b[1625] = (s.v[1519] > s.v[141]);
        s.v[1625] = if s.b[1625] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1625]) {
            s.store_sub(1520, 1519, 141);
            s.store_sub(1521, 140, 141);
            s.store_div(44, 1520, 1521);
            s.store_square(45, 44);
            s.store_mul(46, 45, 44);
            s.store_square(47, 45);
            s.store_div_from_scalar_ad(1529, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));
            s.store_mul_sub_from_scalar_rhs(1529, 1521, 1.0, 1529);
            s.store_add(1526, 141, 1529);
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1625])) {
            s.copy_ad(1526, 1519);
        }

        if (((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) {
            s.store_offset_scaled(1545, 1526, -1.0, (-1e-12));
            s.store_scale(1553, 1552, s.v[1531]);
            s.store_square(1554, 1553);
            s.store_sub_from_scalar(1555, s.v[82], 1551);
            s.store_div_from_scalar(1519, s.v[69], 230);
            s.store_mul_ad(1556, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(s.ad_value(1519)));
            s.store_neg(1557, 1545);
        }

        s.b[1626] = (s.v[1555] < s.v[1557]);
        s.v[1626] = if s.b[1626] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1626]) {
            s.store_div_from_scalar_mul_ad(1520, 1.0, s.ad_value(225), s.ad_value(1552));
            s.store_scale(1529, 1520, s.v[1530]);
            s.store_offset_scaled(1558, 1529, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(1559, 1558, 1558, 8.0, 0.0, 1558);
            s.store_sub(1560, 237, 1556);
            s.store_mul_add_rhs(1528, 225, 1555, 1545);
            s.store_sub_from_scalar_ad(1561, (7.0 * 1.414213562373095), A::mul_scaled_lhs(s.ad_value(1529), 9.0, A::offset(s.ad_value(1528), (-2.0))));
            s.store_square(1562, 1561);
        }

        s.b[1627] = (s.v[1559] < (s.v[1562] * 1e-8));
        s.v[1627] = if s.b[1627] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1626]) && s.b[1627]) {
            s.store_add_scaled_inputs_product_mixed_aaia(1564, A::offset(s.ad_value(1561), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1559), 0.5, s.ad_value(1561), 1.0), 1.0, 1529, A::offset(s.ad_value(1528), (-2.0)), 9.0);
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1626]) && (!s.b[1627])) {
            s.store_sqrt_add(1563, 1559, 1562);
            s.store_add_scaled_offset_product_rhs_mixed_aii(1564, A::offset(s.ad_value(1563), ((-7.0) * 1.414213562373095)), 1.0, 1529, 1528, (-2.0), 9.0);
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1626]) {
            s.store_powf(1565, 1564, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(1566, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1529), 12.0)), 1.0, 1565, 2.0, 1565, 1565, 1.414213562373095);
            s.store_div(1567, 1566, 1565);
            s.store_add_scaled_product_indices(1568, 1545, (-1.0), 1567, 227, 1.0);
            s.store_add(1520, 1568, 1545);
            s.store_div(1521, 1520, 1560);
            s.store_sqrt_square_offset(1522, 1521, 1.0);
            s.store_sub_ad_lhs(1569, A::div(s.ad_value(1520), s.ad_value(1522)), 1545);
            s.store_sub(1521, 1555, 1569);
            s.store_scale(459, 1521, s.v[1530]);
            s.copy_ad(458, 459);
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) {
            s.store_scalar(1567, 3.0);
            s.store_sub_ad_lhs(1570, A::div(s.ad_value(1567), s.ad_value(225)), 1545);
            s.store_exp_neg_input(1529, 1567);
            s.store_offset_ad(1528, A::div_scaled_inputs2(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1555), s.ad_value(1545))), (-1.0)), 4.0, s.ad_value(1529), 4.0, A::mul(s.ad_value(1554), s.ad_value(226)), 1.0), 1.0);
        }

        s.b[1628] = (s.v[1528] < (10.0 * 2.220446049250313e-16));
        s.v[1628] = if s.b[1628] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1628]) {
            s.store_scalar(1528, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) {
            s.store_add_ad_rhs(1570, 1555, A::mul3_scaled_output(s.ad_value(1554), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1528))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1567, 225, 1570, 1545);
            s.store_exp_neg_input(1529, 1567);
            s.store_offset_ad(1528, A::div_scaled_inputs2(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1555), s.ad_value(1545))), (-1.0)), 4.0, s.ad_value(1529), 4.0, A::mul(s.ad_value(1554), s.ad_value(226)), 1.0), 1.0);
        }

        s.b[1629] = (s.v[1528] < (10.0 * 2.220446049250313e-16));
        s.v[1629] = if s.b[1629] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1629]) {
            s.store_scalar(1528, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) {
            s.store_add_ad_rhs(1570, 1555, A::mul3_scaled_output(s.ad_value(1554), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1528))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1567, 225, 1570, 1545);
        }

        s.b[1630] = (s.v[1567] < 3.0);
        s.v[1630] = if s.b[1630] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1630]) {
            s.store_scalar(1571, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(1572, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
            s.store_offset_div_from_scalar_ad(1573, 1.0, A::mul(s.ad_value(225), s.ad_value(1553)), (1.0 / 1.414213562373095));
            s.store_div_scaled_inputs2(1574, s.ad_value(1555), -1.0, s.ad_value(1545), -1.0, s.ad_value(1553), 1.0);
            s.store_add_scaled_inputs3(1575, A::div_scaled_product(A::square(s.ad_value(1572)), s.ad_value(1572), 1.0, A::mul3_scaled_output(s.ad_value(1571), s.ad_value(1571), s.ad_value(1571), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1572), s.ad_value(1573), 1.0, s.ad_value(1571), s.ad_value(1571), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(1574), 1.0, s.ad_value(1571), 2.0), 1.0);
            s.store_div_ad(1576, A::add_scaled_square_product(s.ad_value(1572), (-1.0), s.ad_value(1571), s.ad_value(1573), 3.0), A::mul_scaled_lhs(s.ad_value(1571), 9.0, s.ad_value(1571)));
            s.store_sqrt_ad(1524, A::add_scaled_square_product(s.ad_value(1575), 1.0, A::square(s.ad_value(1576)), s.ad_value(1576), 1.0));
            s.store_powf_ad(1577, A::sub(s.ad_value(1524), s.ad_value(1575)), 0.3333333333333333);
            s.store_neg_ad(1578, A::powf(A::add(s.ad_value(1575), s.ad_value(1524)), 0.3333333333333333));
            s.store_add_scaled_inputs3(1528, s.ad_value(1577), 1.0, s.ad_value(1578), 1.0, A::div_scaled_inputs(s.ad_value(1572), 1.0, s.ad_value(1571), 3.0), -1.0);
            s.store_add_scaled_product_indices(1570, 1545, (-1.0), 1528, 227, 1.0);
            s.store_mul_add_rhs(1567, 225, 1570, 1545);
        }

        s.b[1631] = (p.p41 > 0.0);
        s.v[1631] = if s.b[1631] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1631]) {
            s.store_offset_add(1579, 1555, 1545, 0.1);
            s.store_offset_exp_ad(1586, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1545), -1.0), 1e-50);
            s.store_scale(1519, 230, 1.0 / (s.v[69]));
            s.store_square(1580, 1519);
            s.store_mul(1581, 1580, 1586);
            s.store_mul(1519, 226, 1554);
            s.store_mul(1582, 225, 1579);
            s.store_add_scaled_inputs_product_mixed_aaii(1583, A::ln(A::add_scaled_square_product(s.ad_value(1582), 1.0, s.ad_value(1581), s.ad_value(1519), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1580), s.ad_value(1519))), (-1.0), 225, 1545, 1.0);
            s.store_offset_sub(44, 1582, 1583, (-1.0));
            s.store_scale(45, 1582, 4.0);
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1631]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1631]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1520, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1521, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3(1583, s.ad_value(1582), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
            s.store_sub(1582, 1582, 1583);
            s.store_add_scaled_inputs(1582, 1582, 1.0, 225, 0.1);
            s.store_add_scaled_inputs_product_mixed_aaii(1584, A::ln(A::add_scaled_square_product(s.ad_value(1582), 1.0, s.ad_value(1581), s.ad_value(1519), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1580), s.ad_value(1519))), (-1.0), 225, 1545, 1.0);
            s.copy_ad(1585, 1567);
            s.store_offset_sub(44, 1584, 1585, (-(0.0008 * 75.0)));
            s.store_scale(45, 1584, (4.0 * (0.0008 * 75.0)));
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1631]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1631]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1520, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1521, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3(1567, s.ad_value(1584), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) {
            s.store_sub_ad_lhs(1569, A::div(s.ad_value(1567), s.ad_value(225)), 1545);
            s.store_add_ad(1520, A::offset(s.ad_value(1567), (-1.0)), A::exp_scaled_input(s.ad_value(1567), -1.0));
        }

        s.b[1632] = (s.v[1520] < (10.0 * 2.220446049250313e-16));
        s.v[1632] = if s.b[1632] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1632]) {
            s.store_scalar(1520, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) {
            s.store_sqrt(1521, 1520);
            s.store_mul(458, 1552, 1521);
            s.store_scaled_sub(459, 1555, 1569, s.v[1530]);
        }

        s.b[1633] = (p.p41 == 1.0);
        s.v[1633] = if s.b[1633] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) {
            s.store_exp_ad(1586, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1545), -1.0));
        }

    }

    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) {
            s.store_scale(1519, 230, 1.0 / (s.v[69]));
            s.store_square(1580, 1519);
            s.store_mul(1595, 1580, 1586);
            s.store_scalar(1542, 0.0);
            s.store_scalar(1589, 0.0);
            s.store_scalar(1593, 0.0);
            s.store_scalar(167, 1.0);
        }

        let mut assign31350_loop_guard: usize = 0;
        while {
            let assign31350_cond_e45498: f64 = (2.0 * 20.0);
            let assign31350_cond_e45500: f64 = (assign31350_cond_e45498 + 1.0);
            let assign31350_cond_e45502: f64 = if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (s.v[167] <= assign31350_cond_e45500)) { 1.0 } else { 0.0 };
            assign31350_cond_e45502 != 0.0
        } {
            assign31350_loop_guard += 1;
            assert!(assign31350_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) {
                s.store_scalar(1591, 0.0);
                s.store_mul_add_rhs(1567, 225, 1569, 1545);
            }
            s.b[1634] = (s.v[1567] < 5.0);
            s.v[1634] = if s.b[1634] { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && s.b[1634]) {
                s.store_ad_value(1587, A::mul3(A::square(s.ad_value(1567)), s.ad_value(1567), A::offset(A::mul(s.ad_value(1567), A::scale_offset(s.ad_value(1567), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771)));
                s.store_ad_value(1588, A::mul_offset_rhs(A::square(s.ad_value(1567)), A::mul(s.ad_value(1567), A::scale_offset(s.ad_value(1567), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
                s.store_mul3_lhs(1589, 1595, 1587, 1587);
                s.store_mul_ad_lhs(1590, A::mul3_scaled_output(s.ad_value(1595), s.ad_value(225), s.ad_value(1587), 2.0), 1588);
                s.store_mul_offset_ad_rhs(1591, 1567, A::mul_offset_rhs(s.ad_value(1567), A::mul_offset_rhs(s.ad_value(1567), A::mul(s.ad_value(1567), A::scale_offset(s.ad_value(1567), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_ad(1592, A::mul_offset_rhs(s.ad_value(1567), A::mul_offset_rhs(s.ad_value(1567), A::mul(s.ad_value(1567), A::scale_offset(s.ad_value(1567), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758))), 0.707106781186548);
                s.store_sqrt_offset_ad(1593, A::add(A::square(s.ad_value(1591)), s.ad_value(1589)), 1e-50);
                s.store_div_scaled_inputs2(1594, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1592), s.ad_value(1591), 2.0), 1.0, s.ad_value(1590), 1.0, s.ad_value(1593), 2.0);
            }
            s.b[1635] = (s.v[1567] < 80.0);
            s.v[1635] = if s.b[1635] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (!s.b[1634])) && s.b[1635]) {
                s.store_exp(243, 1567);
                s.store_mul_offset_rhs(1589, 1595, 243, (-1.0));
                s.store_mul3_lhs(1590, 1595, 225, 243);
            }
            if (((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (!s.b[1634])) && (!s.b[1635])) {
                s.store_exp_mul(1596, 225, 1569);
                s.store_mul_sub_rhs(1589, 1580, 1596, 1586);
                s.store_mul3_lhs(1590, 1580, 225, 1596);
            }
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (!s.b[1634])) {
                s.store_sqrt_add_ad(1593, A::offset(s.ad_value(1567), (-1.0)), s.ad_value(1589));
                s.store_scale_ad(1594, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1590), 1.0, s.ad_value(1593), 1.0), 0.5);
            }
            if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) {
                s.store_add_scaled_inputs_product_indices(1597, 1555, 1.0, 1569, (-1.0), 1553, 1593, (-1.0));
                s.store_sub_from_scalar_ad(1598, (-1.0), A::mul(s.ad_value(1553), s.ad_value(1594)));
            }
            s.b[1636] = (s.v[1542] == 1.0);
            s.v[1636] = if s.b[1636] { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && s.b[1636]) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (!s.b[1636])) {
                s.store_div_scaled_inputs(494, s.ad_value(1597), -1.0, s.ad_value(1598), 1.0);
            }
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (!s.b[1636])) {
                s.store_scaled_offset_ad(1599, {
                    if (1.0 >= ((s.v[1569]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1569))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1637] = (((s.v[494]) as f64).abs() > s.v[1599]);
            s.v[1637] = if s.b[1637] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (!s.b[1636])) && s.b[1637]) {
                s.store_scale(494, 1599, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (!s.b[1636])) {
                s.store_add(1569, 1569, 494);
            }
            s.b[1638] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1597]) as f64).abs() <= 1e-8));
            s.v[1638] = if s.b[1638] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (!s.b[1636])) && s.b[1638]) {
                s.store_scalar(1542, 1.0);
            }
            if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.b[1640] = (s.v[1567] < 5.0);
        s.v[1640] = if s.b[1640] { 1.0 } else { 0.0 };

        if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && s.b[1640]) {
            s.store_offset_square(1600, 1591, (10.0 * 2.220446049250313e-16));
            s.store_offset(1601, 1591, (10.0 * 2.220446049250313e-16));
        }

        if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (!s.b[1640])) {
            s.store_offset(1600, 1567, (-1.0));
            s.store_sqrt(1601, 1600);
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) {
            s.store_mul(458, 1552, 1601);
            s.store_div_from_scalar_add_ad(1520, 1.0, s.ad_value(1593), s.ad_value(1601));
            s.store_mul3_lhs(460, 1552, 1589, 1520);
            s.store_add(459, 458, 460);
        }

        if (((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) {
            s.store_sub(460, 459, 458);
        }

        if (((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) {
            if (p.p43 == 1.0) {
                s.store_mul(1523, 287, 1532);
            } else {
                s.store_mul(1523, 108, 1532);
            }
        }

        s.b[1642] = (((s.v[1538] != 0.0) && (p.p43 == 0.0)) || ((s.v[1536] != 0.0) && (p.p43 == 1.0)));
        s.v[1642] = if s.b[1642] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1642]) {
            s.store_mul(455, 1523, 459);
            s.store_mul(457, 1523, 458);
        }

        s.b[1643] = (((s.v[1539] != 0.0) && (p.p43 == 0.0)) || ((s.v[1537] != 0.0) && (p.p43 == 1.0)));
        s.v[1643] = if s.b[1643] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1643]) {
            s.store_mul(454, 1523, 459);
            s.store_mul(456, 1523, 458);
        }

        if ((p.p24 != 0.0) && s.b[1602]) {
            s.store_add_scaled_inputs(266, 462, s.v[566], 461, s.v[565]);
        }

        if (((p.p24 != 0.0) && s.b[1602]) && (s.v[266] != 0.0)) {
            s.store_add_scaled_inputs(269, 462, p.p170, 461, p.p169);
        }

        s.b[1644] = (p.p43 == 1.0);
        s.v[1644] = if s.b[1644] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1602]) && (s.v[266] != 0.0)) && s.b[1644]) {
            s.store_add_scaled_products_indices(1520, 462, 287, 1.0, 461, 288, 1.0);
            s.store_mul_neg_rhs(269, 269, 1520);
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (s.v[266] != 0.0)) && (!s.b[1644])) {
            s.store_mul_neg_rhs(269, 269, 108);
        }

        if (((p.p24 != 0.0) && s.b[1602]) && (s.v[266] != 0.0)) {
            s.store_add_scaled_product_right_ad(268, 268, 1.0, 269, A::sub(s.ad_value(158), s.ad_value(157)), -1.0);
        }

        if ((p.p24 != 0.0) && s.b[1602]) {
            s.store_add_scaled_inputs(266, 461, s.v[566], 462, s.v[565]);
        }

        if (((p.p24 != 0.0) && s.b[1602]) && (s.v[266] != 0.0)) {
            s.store_add_scaled_inputs(270, 461, p.p170, 462, p.p169);
        }

        s.b[1645] = (p.p43 == 1.0);
        s.v[1645] = if s.b[1645] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1602]) && (s.v[266] != 0.0)) && s.b[1645]) {
            s.store_add_scaled_products_indices(1520, 461, 287, 1.0, 462, 288, 1.0);
            s.store_mul_neg_rhs(270, 270, 1520);
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (s.v[266] != 0.0)) && (!s.b[1645])) {
            s.store_mul_neg_rhs(270, 270, 108);
        }

        if (((p.p24 != 0.0) && s.b[1602]) && (s.v[266] != 0.0)) {
            s.store_add_scaled_product_indices(267, 267, 1.0, 270, 158, -1.0);
        }

        s.b[1646] = (((s.v[613] == 1.0) && (!s.b[565])) || ((s.v[613] != 1.0) && (!s.b[566])));
        s.v[1646] = if s.b[1646] { 1.0 } else { 0.0 };

        s.b[1647] = (p.p43 == 1.0);
        s.v[1647] = if s.b[1647] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (!s.b[1602])) && s.b[1646]) && s.b[1647]) {
            s.store_scale(269, 288, ((-s.v[1530]) * p.p188));
        }

        if ((((p.p24 != 0.0) && (!s.b[1602])) && s.b[1646]) && (!s.b[1647])) {
            s.store_scale(269, 108, ((-s.v[1530]) * p.p188));
        }

        if (((p.p24 != 0.0) && (!s.b[1602])) && (!s.b[1646])) {
            s.store_add_scaled_inputs(269, 462, p.p170, 461, p.p169);
        }

        s.b[1648] = (p.p43 == 1.0);
        s.v[1648] = if s.b[1648] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (!s.b[1602])) && (!s.b[1646])) && s.b[1648]) {
            s.store_add_scaled_products_indices(1520, 462, 287, 1.0, 461, 288, 1.0);
            s.store_mul_neg_rhs(269, 269, 1520);
        }

        if ((((p.p24 != 0.0) && (!s.b[1602])) && (!s.b[1646])) && (!s.b[1648])) {
            s.store_mul_neg_rhs(269, 269, 108);
        }

        if ((p.p24 != 0.0) && (!s.b[1602])) {
            s.store_mul_scaled_ad_rhs(268, 269, -1.0, A::sub(s.ad_value(158), s.ad_value(157)));
        }

        s.b[1649] = (((s.v[613] == 1.0) && (!s.b[566])) || ((s.v[613] != 1.0) && (!s.b[565])));
        s.v[1649] = if s.b[1649] { 1.0 } else { 0.0 };

        s.b[1650] = (p.p43 == 1.0);
        s.v[1650] = if s.b[1650] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (!s.b[1602])) && s.b[1649]) && s.b[1650]) {
            s.store_scale(270, 287, ((-s.v[1530]) * p.p188));
        }

        if ((((p.p24 != 0.0) && (!s.b[1602])) && s.b[1649]) && (!s.b[1650])) {
            s.store_scale(270, 108, ((-s.v[1530]) * p.p188));
        }

        if (((p.p24 != 0.0) && (!s.b[1602])) && (!s.b[1649])) {
            s.store_add_scaled_inputs(270, 461, p.p170, 462, p.p169);
        }

        s.b[1651] = (p.p43 == 1.0);
        s.v[1651] = if s.b[1651] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (!s.b[1602])) && (!s.b[1649])) && s.b[1651]) {
            s.store_add_scaled_products_indices(1520, 461, 287, 1.0, 462, 288, 1.0);
            s.store_mul_neg_rhs(270, 270, 1520);
        }

        if ((((p.p24 != 0.0) && (!s.b[1602])) && (!s.b[1649])) && (!s.b[1651])) {
            s.store_mul_neg_rhs(270, 270, 108);
        }

        if ((p.p24 != 0.0) && (!s.b[1602])) {
            s.store_mul_neg_lhs(267, 270, 158);
        }

        s.b[1652] = (p.p43 == 1.0);
        s.v[1652] = if s.b[1652] { 1.0 } else { 0.0 };

        if s.b[1652] {
            s.copy_ad(1668, 590);
            s.copy_ad(1669, 591);
            s.store_scale_ad(1670, A::exp_scaled_input(A::add_scaled_inputs(A::sub_from_scalar((s.v[87] * s.v[114]), A::mul(s.ad_value(237), s.ad_value(225))), 1.0, A::ln_scaled_input(s.ad_value(429), 1.0 / (s.v[81])), p.p175), 1.0 / (p.p174)), p.p173);
            s.store_scale_ad(1671, A::exp_scaled_input(A::add_scaled_inputs(A::sub_from_scalar((s.v[87] * s.v[114]), A::mul(s.ad_value(237), s.ad_value(225))), 1.0, A::ln_scaled_input(s.ad_value(429), 1.0 / (s.v[81])), p.p176), 1.0 / (p.p174)), p.p173);
            s.store_scaled_mul(1675, 286, 1670, p.p237);
            s.store_scaled_mul(1677, 286, 1671, p.p237);
            s.store_scaled_mul(1676, 285, 1670, p.p237);
            s.store_scaled_mul(1678, 285, 1671, p.p237);
            s.store_scale(1654, 429, 1.0 / (s.v[81]));
            s.store_offset(1655, 1675, 1e-50);
            s.store_scale_ad(1673, A::div_from_scalar(p.p174, s.ad_value(225)), 0.0);
            s.store_scale_ad(1674, A::div_from_scalar(p.p174, s.ad_value(225)), 0.0);
            s.store_scale(1672, 227, p.p174);
        }

        s.b[1681] = (s.v[1668] < s.v[1673]);
        s.v[1681] = if s.b[1681] { 1.0 } else { 0.0 };

        if (s.b[1652] && s.b[1681]) {
            s.store_exp_div(1654, 1668, 1672);
            s.store_mul_offset_rhs(282, 1675, 1654, (-1.0));
        }

        if (s.b[1652] && (!s.b[1681])) {
            s.store_exp_div(1654, 1673, 1672);
            s.store_add_scaled_offset_product_rhs_mixed_aii(282, A::mul3(A::div(s.ad_value(1675), s.ad_value(1672)), s.ad_value(1654), A::sub(s.ad_value(1668), s.ad_value(1673))), 1.0, 1675, 1654, (-1.0), 1.0);
        }

        if s.b[1652] {
            s.store_add_scaled_product_indices(282, 282, 1.0, 1668, 1677, p.p178);
        }

        s.b[1682] = (s.v[1669] < s.v[1674]);
        s.v[1682] = if s.b[1682] { 1.0 } else { 0.0 };

        if (s.b[1652] && s.b[1682]) {
            s.store_exp_div(1654, 1669, 1672);
            s.store_mul_offset_rhs(281, 1676, 1654, (-1.0));
        }

        if (s.b[1652] && (!s.b[1682])) {
            s.store_exp_div(1654, 1674, 1672);
            s.store_add_scaled_offset_product_rhs_mixed_aii(281, A::mul3(A::div(s.ad_value(1676), s.ad_value(1672)), s.ad_value(1654), A::sub(s.ad_value(1669), s.ad_value(1674))), 1.0, 1676, 1654, (-1.0), 1.0);
        }

        if s.b[1652] {
            s.store_add_scaled_product_indices(281, 281, 1.0, 1669, 1678, p.p178);
            s.store_add_scaled_inputs(282, 282, 1.0, 1668, s.v[142]);
            s.store_add_scaled_inputs(281, 281, 1.0, 1669, s.v[142]);
            s.store_scalar(1662, (p.p179 * p.p2));
            s.store_scalar(1663, (p.p179 * p.p3));
            s.store_scalar(1661, (p.p237 - p.p238));
        }

        s.b[1683] = (s.v[1661] <= 0.0);
        s.v[1683] = if s.b[1683] { 1.0 } else { 0.0 };

        if (s.b[1652] && s.b[1683]) {
            s.store_scalar(1662, 0.0);
            s.store_scalar(1663, 0.0);
        }

        s.b[1684] = (p.p5 > s.v[287]);
        s.v[1684] = if s.b[1684] { 1.0 } else { 0.0 };

        if (s.b[1652] && s.b[1684]) {
            s.store_offset_scaled(1665, 287, (-p.p180), ((p.p5) * (p.p180)));
            s.store_scale(1667, 287, p.p181);
        }

        s.b[1685] = (s.v[1669] < 0.0);
        s.v[1685] = if s.b[1685] { 1.0 } else { 0.0 };

        s.b[1686] = (s.v[1663] > 0.0);
        s.v[1686] = if s.b[1686] { 1.0 } else { 0.0 };

        if (((s.b[1652] && s.b[1684]) && s.b[1685]) && s.b[1686]) {
            s.store_sub_from_scalar_ad(1679, 1.0, A::scale(s.ad_value(1669), 1.0 / (p.p185)));
        }

        s.b[1687] = (p.p182 == 0.5);
        s.v[1687] = if s.b[1687] { 1.0 } else { 0.0 };

        if ((((s.b[1652] && s.b[1684]) && s.b[1685]) && s.b[1686]) && s.b[1687]) {
            s.store_div_from_scalar_sqrt_ad(1680, 1.0, s.ad_value(1679));
        }

        if ((((s.b[1652] && s.b[1684]) && s.b[1685]) && s.b[1686]) && (!s.b[1687])) {
            s.store_powf(1680, 1679, (-p.p182));
        }

        if (((s.b[1652] && s.b[1684]) && s.b[1685]) && s.b[1686]) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(283, 1663, 1.0, A::mul(s.ad_value(1679), s.ad_value(1680)), (p.p185 * 1.0 / ((1.0 - p.p182))));
        }

        if (((s.b[1652] && s.b[1684]) && s.b[1685]) && (!s.b[1686])) {
            s.store_scalar(283, 0.0);
        }

        s.b[1688] = (s.v[1665] > 0.0);
        s.v[1688] = if s.b[1688] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1652] && s.b[1684]) && s.b[1685]) && s.b[1688]) {
            s.store_sub_from_scalar_ad(1679, 1.0, A::scale(s.ad_value(1669), 1.0 / (p.p186)));
        }

        s.b[1689] = (p.p183 == 0.5);
        s.v[1689] = if s.b[1689] { 1.0 } else { 0.0 };

        if ((((s.b[1652] && s.b[1684]) && s.b[1685]) && s.b[1688]) && s.b[1689]) {
            s.store_div_from_scalar_sqrt_ad(1680, 1.0, s.ad_value(1679));
        }

        if ((((s.b[1652] && s.b[1684]) && s.b[1685]) && s.b[1688]) && (!s.b[1689])) {
            s.store_powf(1680, 1679, (-p.p183));
        }

        if (((s.b[1652] && s.b[1684]) && s.b[1685]) && s.b[1688]) {
            s.store_add_ad_rhs(283, 283, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1665), 1.0, A::mul(s.ad_value(1679), s.ad_value(1680)), (p.p186 * 1.0 / ((1.0 - p.p183)))));
        }

        s.b[1690] = (s.v[1667] > 0.0);
        s.v[1690] = if s.b[1690] { 1.0 } else { 0.0 };

        if (((s.b[1652] && s.b[1684]) && s.b[1685]) && s.b[1690]) {
            s.store_sub_from_scalar_ad(1679, 1.0, A::scale(s.ad_value(1669), 1.0 / (p.p187)));
        }

        s.b[1691] = (p.p184 == 0.5);
        s.v[1691] = if s.b[1691] { 1.0 } else { 0.0 };

        if ((((s.b[1652] && s.b[1684]) && s.b[1685]) && s.b[1690]) && s.b[1691]) {
            s.store_div_from_scalar_sqrt_ad(1680, 1.0, s.ad_value(1679));
        }

        if ((((s.b[1652] && s.b[1684]) && s.b[1685]) && s.b[1690]) && (!s.b[1691])) {
            s.store_powf(1680, 1679, (-p.p184));
        }

        if (((s.b[1652] && s.b[1684]) && s.b[1685]) && s.b[1690]) {
            s.store_add_ad_rhs(283, 283, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1667), 1.0, A::mul(s.ad_value(1679), s.ad_value(1680)), (p.p187 * 1.0 / ((1.0 - p.p184)))));
        }

        if ((s.b[1652] && s.b[1684]) && (!s.b[1685])) {
            s.store_add_scaled_inputs3(1654, s.ad_value(1663), 1.0, s.ad_value(1665), 1.0, s.ad_value(1667), 1.0);
            s.store_add_scaled_inputs3(1655, s.ad_value(1663), (p.p182 * 1.0 / (p.p185)), s.ad_value(1665), (p.p183 * 1.0 / (p.p186)), s.ad_value(1667), (p.p184 * 1.0 / (p.p187)));
            s.store_mul_ad_rhs(283, 1669, A::add_scaled_product(s.ad_value(1654), 1.0, s.ad_value(1669), s.ad_value(1655), 0.5));
        }

        if (s.b[1652] && (!s.b[1684])) {
            s.store_scalar(1667, (p.p181 * p.p5));
        }

        s.b[1692] = (s.v[1669] < 0.0);
        s.v[1692] = if s.b[1692] { 1.0 } else { 0.0 };

        s.b[1693] = (s.v[1663] > 0.0);
        s.v[1693] = if s.b[1693] { 1.0 } else { 0.0 };

        if (((s.b[1652] && (!s.b[1684])) && s.b[1692]) && s.b[1693]) {
            s.store_sub_from_scalar_ad(1679, 1.0, A::scale(s.ad_value(1669), 1.0 / (p.p185)));
        }

        s.b[1694] = (p.p182 == 0.5);
        s.v[1694] = if s.b[1694] { 1.0 } else { 0.0 };

        if ((((s.b[1652] && (!s.b[1684])) && s.b[1692]) && s.b[1693]) && s.b[1694]) {
            s.store_div_from_scalar_sqrt_ad(1680, 1.0, s.ad_value(1679));
        }

        if ((((s.b[1652] && (!s.b[1684])) && s.b[1692]) && s.b[1693]) && (!s.b[1694])) {
            s.store_powf(1680, 1679, (-p.p182));
        }

        if (((s.b[1652] && (!s.b[1684])) && s.b[1692]) && s.b[1693]) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(283, 1663, 1.0, A::mul(s.ad_value(1679), s.ad_value(1680)), (p.p185 * 1.0 / ((1.0 - p.p182))));
        }

        if (((s.b[1652] && (!s.b[1684])) && s.b[1692]) && (!s.b[1693])) {
            s.store_scalar(283, 0.0);
        }

        s.b[1695] = (s.v[1667] > 0.0);
        s.v[1695] = if s.b[1695] { 1.0 } else { 0.0 };

        if (((s.b[1652] && (!s.b[1684])) && s.b[1692]) && s.b[1695]) {
            s.store_sub_from_scalar_ad(1679, 1.0, A::scale(s.ad_value(1669), 1.0 / (p.p187)));
        }

        s.b[1696] = (p.p184 == 0.5);
        s.v[1696] = if s.b[1696] { 1.0 } else { 0.0 };

        if ((((s.b[1652] && (!s.b[1684])) && s.b[1692]) && s.b[1695]) && s.b[1696]) {
            s.store_div_from_scalar_sqrt_ad(1680, 1.0, s.ad_value(1679));
        }

        if ((((s.b[1652] && (!s.b[1684])) && s.b[1692]) && s.b[1695]) && (!s.b[1696])) {
            s.store_powf(1680, 1679, (-p.p184));
        }

        if (((s.b[1652] && (!s.b[1684])) && s.b[1692]) && s.b[1695]) {
            s.store_add_ad_rhs(283, 283, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1667), 1.0, A::mul(s.ad_value(1679), s.ad_value(1680)), (p.p187 * 1.0 / ((1.0 - p.p184)))));
        }

        if ((s.b[1652] && (!s.b[1684])) && (!s.b[1692])) {
            s.store_add(1654, 1663, 1667);
            s.store_add_scaled_inputs(1655, 1663, (p.p182 * 1.0 / (p.p185)), 1667, (p.p184 * 1.0 / (p.p187)));
            s.store_mul_ad_rhs(283, 1669, A::add_scaled_product(s.ad_value(1654), 1.0, s.ad_value(1669), s.ad_value(1655), 0.5));
        }

        s.b[1697] = (p.p4 > s.v[288]);
        s.v[1697] = if s.b[1697] { 1.0 } else { 0.0 };

        if (s.b[1652] && s.b[1697]) {
            s.store_offset_scaled(1664, 288, (-p.p180), ((p.p4) * (p.p180)));
            s.store_scale(1666, 288, p.p181);
        }

        s.b[1698] = (s.v[1668] < 0.0);
        s.v[1698] = if s.b[1698] { 1.0 } else { 0.0 };

        s.b[1699] = (s.v[1662] > 0.0);
        s.v[1699] = if s.b[1699] { 1.0 } else { 0.0 };

        if (((s.b[1652] && s.b[1697]) && s.b[1698]) && s.b[1699]) {
            s.store_sub_from_scalar_ad(1679, 1.0, A::scale(s.ad_value(1668), 1.0 / (p.p185)));
        }

        s.b[1700] = (p.p182 == 0.5);
        s.v[1700] = if s.b[1700] { 1.0 } else { 0.0 };

        if ((((s.b[1652] && s.b[1697]) && s.b[1698]) && s.b[1699]) && s.b[1700]) {
            s.store_div_from_scalar_sqrt_ad(1680, 1.0, s.ad_value(1679));
        }

        if ((((s.b[1652] && s.b[1697]) && s.b[1698]) && s.b[1699]) && (!s.b[1700])) {
            s.store_powf(1680, 1679, (-p.p182));
        }

        if (((s.b[1652] && s.b[1697]) && s.b[1698]) && s.b[1699]) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(284, 1662, 1.0, A::mul(s.ad_value(1679), s.ad_value(1680)), (p.p185 * 1.0 / ((1.0 - p.p182))));
        }

        if (((s.b[1652] && s.b[1697]) && s.b[1698]) && (!s.b[1699])) {
            s.store_scalar(284, 0.0);
        }

        s.b[1701] = (s.v[1664] > 0.0);
        s.v[1701] = if s.b[1701] { 1.0 } else { 0.0 };

        if (((s.b[1652] && s.b[1697]) && s.b[1698]) && s.b[1701]) {
            s.store_sub_from_scalar_ad(1679, 1.0, A::scale(s.ad_value(1668), 1.0 / (p.p186)));
        }

        s.b[1702] = (p.p183 == 0.5);
        s.v[1702] = if s.b[1702] { 1.0 } else { 0.0 };

        if ((((s.b[1652] && s.b[1697]) && s.b[1698]) && s.b[1701]) && s.b[1702]) {
            s.store_div_from_scalar_sqrt_ad(1680, 1.0, s.ad_value(1679));
        }

        if ((((s.b[1652] && s.b[1697]) && s.b[1698]) && s.b[1701]) && (!s.b[1702])) {
            s.store_powf(1680, 1679, (-p.p183));
        }

        if (((s.b[1652] && s.b[1697]) && s.b[1698]) && s.b[1701]) {
            s.store_add_ad_rhs(284, 284, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1664), 1.0, A::mul(s.ad_value(1679), s.ad_value(1680)), (p.p186 * 1.0 / ((1.0 - p.p183)))));
        }

        s.b[1703] = (s.v[1666] > 0.0);
        s.v[1703] = if s.b[1703] { 1.0 } else { 0.0 };

        if (((s.b[1652] && s.b[1697]) && s.b[1698]) && s.b[1703]) {
            s.store_sub_from_scalar_ad(1679, 1.0, A::scale(s.ad_value(1668), 1.0 / (p.p187)));
        }

        s.b[1704] = (p.p184 == 0.5);
        s.v[1704] = if s.b[1704] { 1.0 } else { 0.0 };

        if ((((s.b[1652] && s.b[1697]) && s.b[1698]) && s.b[1703]) && s.b[1704]) {
            s.store_div_from_scalar_sqrt_ad(1680, 1.0, s.ad_value(1679));
        }

        if ((((s.b[1652] && s.b[1697]) && s.b[1698]) && s.b[1703]) && (!s.b[1704])) {
            s.store_powf(1680, 1679, (-p.p184));
        }

        if (((s.b[1652] && s.b[1697]) && s.b[1698]) && s.b[1703]) {
            s.store_add_ad_rhs(284, 284, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1666), 1.0, A::mul(s.ad_value(1679), s.ad_value(1680)), (p.p187 * 1.0 / ((1.0 - p.p184)))));
        }

        if ((s.b[1652] && s.b[1697]) && (!s.b[1698])) {
            s.store_add_scaled_inputs3(1654, s.ad_value(1662), 1.0, s.ad_value(1664), 1.0, s.ad_value(1666), 1.0);
            s.store_add_scaled_inputs3(1655, s.ad_value(1662), (p.p182 * 1.0 / (p.p185)), s.ad_value(1664), (p.p183 * 1.0 / (p.p186)), s.ad_value(1666), (p.p184 * 1.0 / (p.p187)));
            s.store_mul_ad_rhs(284, 1668, A::add_scaled_product(s.ad_value(1654), 1.0, s.ad_value(1668), s.ad_value(1655), 0.5));
        }

        if (s.b[1652] && (!s.b[1697])) {
            s.store_scalar(1666, (p.p181 * p.p4));
        }

        s.b[1705] = (s.v[1668] < 0.0);
        s.v[1705] = if s.b[1705] { 1.0 } else { 0.0 };

        s.b[1706] = (s.v[1662] > 0.0);
        s.v[1706] = if s.b[1706] { 1.0 } else { 0.0 };

        if (((s.b[1652] && (!s.b[1697])) && s.b[1705]) && s.b[1706]) {
            s.store_sub_from_scalar_ad(1679, 1.0, A::scale(s.ad_value(1668), 1.0 / (p.p185)));
        }

        s.b[1707] = (p.p182 == 0.5);
        s.v[1707] = if s.b[1707] { 1.0 } else { 0.0 };

        if ((((s.b[1652] && (!s.b[1697])) && s.b[1705]) && s.b[1706]) && s.b[1707]) {
            s.store_div_from_scalar_sqrt_ad(1680, 1.0, s.ad_value(1679));
        }

        if ((((s.b[1652] && (!s.b[1697])) && s.b[1705]) && s.b[1706]) && (!s.b[1707])) {
            s.store_powf(1680, 1679, (-p.p182));
        }

        if (((s.b[1652] && (!s.b[1697])) && s.b[1705]) && s.b[1706]) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(284, 1662, 1.0, A::mul(s.ad_value(1679), s.ad_value(1680)), (p.p185 * 1.0 / ((1.0 - p.p182))));
        }

        if (((s.b[1652] && (!s.b[1697])) && s.b[1705]) && (!s.b[1706])) {
            s.store_scalar(284, 0.0);
        }

        s.b[1708] = (s.v[1666] > 0.0);
        s.v[1708] = if s.b[1708] { 1.0 } else { 0.0 };

        if (((s.b[1652] && (!s.b[1697])) && s.b[1705]) && s.b[1708]) {
            s.store_sub_from_scalar_ad(1679, 1.0, A::scale(s.ad_value(1668), 1.0 / (p.p187)));
        }

        s.b[1709] = (p.p184 == 0.5);
        s.v[1709] = if s.b[1709] { 1.0 } else { 0.0 };

        if ((((s.b[1652] && (!s.b[1697])) && s.b[1705]) && s.b[1708]) && s.b[1709]) {
            s.store_div_from_scalar_sqrt_ad(1680, 1.0, s.ad_value(1679));
        }

        if ((((s.b[1652] && (!s.b[1697])) && s.b[1705]) && s.b[1708]) && (!s.b[1709])) {
            s.store_powf(1680, 1679, (-p.p184));
        }

        if (((s.b[1652] && (!s.b[1697])) && s.b[1705]) && s.b[1708]) {
            s.store_add_ad_rhs(284, 284, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1666), 1.0, A::mul(s.ad_value(1679), s.ad_value(1680)), (p.p187 * 1.0 / ((1.0 - p.p184)))));
        }

        if ((s.b[1652] && (!s.b[1697])) && (!s.b[1705])) {
            s.store_add(1654, 1662, 1666);
            s.store_add_scaled_inputs(1655, 1662, (p.p182 * 1.0 / (p.p185)), 1666, (p.p184 * 1.0 / (p.p187)));
            s.store_mul_ad_rhs(284, 1668, A::add_scaled_product(s.ad_value(1654), 1.0, s.ad_value(1668), s.ad_value(1655), 0.5));
        }

        s.b[1710] = (s.v[1663] > 0.0);
        s.v[1710] = if s.b[1710] { 1.0 } else { 0.0 };

        if (s.b[1652] && s.b[1710]) {
            s.store_scaled_mul(1657, 544, 1661, ((-1.6021918e-19) * p.p3));
            s.store_scale(1659, 1657, (-0.001));
            s.store_add_scaled_inputs3(44, s.ad_value(1657), -1.0, s.ad_value(283), 1.0, s.ad_value(1659), -1.0);
            s.store_scaled_mul(45, 1657, 1659, (-4.0));
        }

        if (s.b[1652] && s.b[1710]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (s.b[1652] && s.b[1710]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3(283, s.ad_value(1657), -1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
            s.store_scale(283, 283, (-1.0));
        }

        s.b[1711] = (s.v[1662] > 0.0);
        s.v[1711] = if s.b[1711] { 1.0 } else { 0.0 };

        if (s.b[1652] && s.b[1711]) {
            s.store_scaled_mul(1658, 544, 1661, ((-1.6021918e-19) * p.p2));
            s.store_scale(1660, 1658, (-0.001));
            s.store_add_scaled_inputs3(44, s.ad_value(1658), -1.0, s.ad_value(284), 1.0, s.ad_value(1660), -1.0);
            s.store_scaled_mul(45, 1658, 1660, (-4.0));
        }

        if (s.b[1652] && s.b[1711]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (s.b[1652] && s.b[1711]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3(284, s.ad_value(1658), -1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
            s.store_scale(284, 284, (-1.0));
        }

        s.b[1717] = (s.v[145] == 0.0);
        s.v[1717] = if s.b[1717] { 1.0 } else { 0.0 };

        if ((s.v[85] != 0.0) && s.b[1717]) {
            s.store_scalar(1712, p.p233);
            s.store_scalar(1713, p.p234);
            s.copy_ad(1714, 441);
            s.store_mul_ad_lhs(1715, A::mul3(s.ad_value(1712), s.ad_value(1713), s.ad_value(1714)), 1714);
            s.store_offset_add_ad(1716, A::mul3(s.ad_value(250), s.ad_value(192), s.ad_value(1712)), A::mul3(s.ad_value(1713), s.ad_value(1714), s.ad_value(1714)), 1e-50);
            s.store_div(289, 1715, 1716);
        }

        if ((s.v[85] != 0.0) && (!s.b[1717])) {
            s.store_scalar(289, (p.p233 + 1e-50));
        }

        if (s.v[85] != 0.0) {
            s.store_scalar(1715, p.p235);
            s.store_mul(290, 1715, 323);
        }

        s.b[1725] = ((p.p31 != 0.0) && (s.v[145] == 0.0));
        s.v[1725] = if s.b[1725] { 1.0 } else { 0.0 };

        if s.b[1725] {
            s.store_scalar(1722, s.v[62]);
            s.store_scalar(1723, s.v[63]);
            s.store_scalar(1724, s.v[64]);
            s.store_scale(1718, 244, 6.241449993689894e18);
            s.store_mul_scaled_ad_lhs(1719, A::add_scaled_inputs3(s.ad_value(323), 1.0, A::div(s.ad_value(244), A::sub(s.ad_value(161), s.ad_value(435))), 1.0, s.ad_value(1724), 1.0), 227, 6.241449993689894e18);
            s.store_sub_ad_lhs(1720, A::div_scaled_value_by_product(s.ad_value(197), ((-2.0) * 6.241449993689894e18), s.ad_value(442), s.ad_value(108), 1.0), 1718);
        }

        s.b[1726] = ((((s.v[1720] - s.v[1718])) as f64).abs() > (10.0 * 2.220446049250313e-16));
        s.v[1726] = if s.b[1726] { 1.0 } else { 0.0 };

        if (s.b[1725] && s.b[1726]) {
            let assign33520_ad_e48662: A = A::add_scaled_product(A::div_scalar_by_product(1.0, A::add(s.ad_value(1718), s.ad_value(1719)), A::add(s.ad_value(1720), s.ad_value(1719)), 1.0), 1.0, A::div_scaled_product3(s.ad_value(1722), s.ad_value(252), s.ad_value(250), 2.0, A::sub(s.ad_value(1720), s.ad_value(1718)), 1.0), A::ln(A::div_scaled_inputs2(s.ad_value(1720), 1.0, s.ad_value(1719), 1.0, A::add(s.ad_value(1718), s.ad_value(1719)), 1.0)), 1.0);
            s.store_add_scaled_product_mixed_aai(1721, assign33520_ad_e48662, 1.0, A::mul3(A::mul3(s.ad_value(1722), s.ad_value(252), s.ad_value(250)), s.ad_value(1722), s.ad_value(252)), 250, 1.0);
        }

        if (s.b[1725] && (!s.b[1726])) {
            s.store_add_scaled_inputs_product_mixed_aaai(1721, A::div_scalar_by_product(1.0, A::add(s.ad_value(1718), s.ad_value(1719)), A::add(s.ad_value(1720), s.ad_value(1719)), 1.0), 1.0, A::div_scaled_product3(s.ad_value(1722), s.ad_value(252), s.ad_value(250), 2.0, A::add(s.ad_value(1718), s.ad_value(1719)), 1.0), 1.0, A::mul3(A::mul3(s.ad_value(1722), s.ad_value(252), s.ad_value(250)), s.ad_value(1722), s.ad_value(252)), 250, 1.0);
        }

        if s.b[1725] {
            s.store_mul_ad_lhs(291, A::div_scaled_product(A::square(s.ad_value(199)), s.ad_value(1723), 1.0, A::mul3(s.ad_value(441), s.ad_value(225), s.ad_value(107)), 1.0), 1721);
        }

        if (!s.b[1725]) {
            s.store_scalar(291, 0.0);
        }

        s.b[1744] = ((p.p32 != 0.0) && (s.v[145] == 0.0));
        s.v[1744] = if s.b[1744] { 1.0 } else { 0.0 };

        if s.b[1744] {
            s.store_div_scaled_inputs2(1727, s.ad_value(314), 1.0, s.ad_value(161), (-1.0), s.ad_value(441), 1.0);
            s.store_scaled_mul(1728, 251, 1727, 1e-5);
        }

        s.b[1745] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1745] = if s.b[1745] { 1.0 } else { 0.0 };

        if (s.b[1744] && s.b[1745]) {
            s.store_scalar(1729, 1.0);
        }

        s.b[1746] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1746] = if s.b[1746] { 1.0 } else { 0.0 };

        if ((s.b[1744] && (!s.b[1745])) && s.b[1746]) {
            s.copy_ad(1729, 1728);
        }

        if ((s.b[1744] && (!s.b[1745])) && (!s.b[1746])) {
            s.store_powf(1729, 1728, (p.p113 - 1.0));
        }

        if s.b[1744] {
            s.store_mul(1730, 1728, 1729);
            s.store_offset(1731, 1730, 1.0);
            s.store_powf(1732, 1731, (((-1.0) / p.p113) - 1.0));
            s.store_mul(1733, 1731, 1732);
            s.store_mul(293, 251, 1733);
            s.store_scaled_add(1735, 250, 293, 0.5);
            s.store_square(1734, 190);
        }

    }

    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1744] {
            let assign33710_ad_e48913: A = A::add_scaled_inputs3(A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(190), 3.0, 1.0), 1.0, s.ad_value(1734), 6.0), s.ad_value(293), s.ad_value(293)), 1.0, A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(190), 4.0, 3.0), 1.0, s.ad_value(1734), 3.0), s.ad_value(293), s.ad_value(250)), 1.0, A::mul3(A::add(A::scale_offset(s.ad_value(190), 3.0, 6.0), s.ad_value(1734)), s.ad_value(250), s.ad_value(250)), 1.0);
            s.store_div_scaled_product3_by_product(292, A::mul3(s.ad_value(107), s.ad_value(323), s.ad_value(192)), s.ad_value(250), assign33710_ad_e48913, 1.0, A::mul3_scaled_output(s.ad_value(441), A::offset(s.ad_value(190), 1.0), s.ad_value(1735), 15.0), s.ad_value(1735), 1.0);
        }

        if (!s.b[1744]) {
            s.store_scalar(292, 0.0);
        }

        s.b[1747] = ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (s.v[146] == 1.0)) && (s.v[145] == 0.0));
        s.v[1747] = if s.b[1747] { 1.0 } else { 0.0 };

        if s.b[1747] {
            s.store_sqrt(298, 296);
            s.store_add(1736, 192, 298);
            s.store_square(1737, 294);
            s.store_square(1738, 296);
            s.store_scaled_mul(1739, 294, 296, 42.0);
            s.store_add_scaled_inputs3(1739, s.ad_value(1739), 1.0, s.ad_value(1737), 4.0, s.ad_value(1738), 4.0);
            s.store_add_ad_rhs(1739, 1739, A::mul3_scaled_output(s.ad_value(298), s.ad_value(192), A::add(s.ad_value(294), s.ad_value(296)), 20.0));
            s.store_square(1740, 1736);
            s.store_square(1732, 1740);
            s.store_div_ad_rhs(299, 1739, A::mul(s.ad_value(1732), s.ad_value(1736)));
            s.store_mul_ad_product_lhs(300, A::div(s.ad_value(107), s.ad_value(441)), s.ad_value(250), 323);
            s.store_mul(1742, 300, 192);
            s.store_div(1743, 292, 1742);
            s.store_add_ad_lhs(1741, A::add_scaled_product(s.ad_value(294), 1.0, s.ad_value(192), s.ad_value(298), 4.0), 296);
            s.store_div_scaled_product_by_product(301, s.ad_value(297), s.ad_value(1741), 3.872983346207417, s.ad_value(1736), A::sqrt(A::mul(A::mul3(s.ad_value(1743), s.ad_value(1736), s.ad_value(192)), s.ad_value(1739))), 6.0);
        }

        s.store_add(199, 199, 265);

        s.b[1748] = (p.p43 == 1.0);
        s.v[1748] = if s.b[1748] { 1.0 } else { 0.0 };

        if s.b[1748] {
            s.store_add(271, 531, 532);
        }

        if (s.b[1748] && s.b[564]) {
            s.store_offset(271, 271, (-(p.p168 * s.v[99])));
        }

        if s.b[1748] {
            s.store_mul_scaled_ad_rhs(272, 271, -1.0, A::sub(s.ad_value(158), s.ad_value(513)));
            s.store_scalar(276, 0.0);
            s.store_mul_scaled_ad_rhs(274, 276, p.p9, A::offset(s.ad_value(518), s.v[101]));
            s.store_mul_scaled_ad_rhs(275, 276, p.p9, A::offset(s.ad_value(519), s.v[101]));
            s.store_mul_sub_rhs(277, 274, 158, 157);
            s.store_mul(278, 275, 158);
            s.store_mul_scaled_ad_rhs(279, 276, (p.p19 * p.p9), A::sub(s.ad_value(158), s.ad_value(513)));
            s.store_add(268, 268, 277);
            s.store_add(267, 267, 278);
            s.store_add(272, 272, 279);
        }

        if ((!s.b[1748]) && s.b[564]) {
            s.store_scalar(271, ((-p.p168) * s.v[99]));
            s.store_mul_scaled_ad_rhs(272, 271, -1.0, A::sub(s.ad_value(158), s.ad_value(513)));
        }

        if ((!s.b[1748]) && (!s.b[564])) {
            s.store_scalar(271, 0.0);
            s.store_scalar(272, 0.0);
        }

        if (!s.b[1748]) {
            s.store_scalar(273, 0.0);
            s.copy_ad(274, 273);
            s.copy_ad(275, 273);
            s.store_mul_sub_rhs(277, 274, 158, 157);
            s.store_mul(278, 275, 158);
            s.store_add(268, 268, 277);
            s.store_add(267, 267, 278);
        }

        s.store_scale(9, 199, s.v[451]);

        if (s.v[85] != 0.0) {
            s.store_scalar(24, 0.0);
            s.store_scalar(23, 0.0);
        }

        s.b[1749] = (p.p43 == 1.0);
        s.v[1749] = if s.b[1749] { 1.0 } else { 0.0 };

        if ((s.v[85] != 0.0) && s.b[1749]) {
            s.store_scalar(25, 0.0);
            s.copy_ad(556, 438);
            s.store_scale(588, 196, s.v[451]);
            s.store_scale(587, 197, s.v[451]);
        }

        if ((s.v[85] != 0.0) && (!s.b[1749])) {
            s.store_scalar(554, 0.0);
            s.store_scale(588, 392, s.v[451]);
            s.store_scaled_add(576, 198, 477, s.v[451]);
            s.store_add_scaled_inputs3(577, s.ad_value(197), s.v[451], s.ad_value(198), ((-1.0) * s.v[451]), s.ad_value(476), s.v[451]);
        }

        s.b[1750] = (p.p43 == 1.0);
        s.v[1750] = if s.b[1750] { 1.0 } else { 0.0 };

        if ((s.v[85] == 0.0) && s.b[1750]) {
            s.store_sub_scaled_inputs(23, 196, (-s.v[451]), 197, s.v[451]);
            s.store_scale(24, 198, s.v[451]);
            s.store_scaled_sub(25, 197, 198, s.v[451]);
        }

        if ((s.v[85] == 0.0) && (!s.b[1750])) {
            s.store_add_scaled_inputs4(23, s.ad_value(392), (-s.v[451]), s.ad_value(197), ((-1.0) * s.v[451]), s.ad_value(476), (-s.v[451]), s.ad_value(477), (-s.v[451]));
            s.store_scaled_add(24, 198, 477, s.v[451]);
            s.store_add_scaled_inputs3(25, s.ad_value(197), s.v[451], s.ad_value(198), ((-1.0) * s.v[451]), s.ad_value(476), s.v[451]);
        }

        s.b[1756] = (p.p64 == 0.0);
        s.v[1756] = if s.b[1756] { 1.0 } else { 0.0 };

        if s.b[1756] {
            s.store_scalar(280, 0.0);
        }

        if (!s.b[1756]) {
            s.store_add_scaled_inputs(1751, 315, s.v[97], 161, 1.0);
        }

        s.b[1757] = (s.v[1751] > s.v[314]);
        s.v[1757] = if s.b[1757] { 1.0 } else { 0.0 };

        if ((!s.b[1756]) && s.b[1757]) {
            s.copy_ad(1751, 314);
        }

        if (!s.b[1756]) {
            s.store_add_scaled_inputs3(1752, s.ad_value(157), s.v[317], s.ad_value(161), s.v[317], s.ad_value(1751), (1.0 - s.v[317]));
            s.store_sqrt_div_from_scalar_ad(1753, (2.0 * 1.034943e-10), s.ad_value(229));
            s.store_scale(1754, 1753, 1.3);
            s.store_scaled_mul(1755, 108, 1754, 1.034943e-10);
            s.store_mul_ad_lhs(280, A::add_scaled_inputs4(s.ad_value(161), 1.0 / (p.p64), s.ad_value(157), 1.0 / (p.p64), s.ad_value(1752), (-1.0 / (p.p64)), s.ad_value(315), -1.0), 1755);
        }

        s.b[1758] = (p.p65 != 0.0);
        s.v[1758] = if s.b[1758] { 1.0 } else { 0.0 };

        if s.b[1758] {
            s.store_add_scaled_product_indices(280, 280, 1.0, 135, 513, 1.0);
        }

        s.b[1759] = (p.p24 == 1.0);
        s.v[1759] = if s.b[1759] { 1.0 } else { 0.0 };

        s.b[1760] = (p.p43 == 1.0);
        s.v[1760] = if s.b[1760] { 1.0 } else { 0.0 };

        if (s.b[1759] && s.b[1760]) {
            s.store_add_scaled_inputs4(471, s.ad_value(463), -1.0, s.ad_value(464), (-1.0), s.ad_value(467), -1.0, s.ad_value(468), -1.0);
            s.store_add(472, 466, 470);
            s.store_add(473, 465, 469);
            s.store_add_ad_rhs(23, 23, A::add_scaled_inputs(A::sub(A::sub(A::add_scaled_inputs4(s.ad_value(268), 1.0, s.ad_value(267), 1.0, s.ad_value(272), 1.0, s.ad_value(280), -1.0), s.ad_value(455)), s.ad_value(454)), s.v[451], s.ad_value(471), s.v[451]));
            s.store_add_ad_rhs(24, 24, A::add_scaled_inputs4(s.ad_value(280), s.v[451], s.ad_value(268), ((-1.0) * s.v[451]), s.ad_value(456), s.v[451], s.ad_value(472), s.v[451]));
            s.store_add_scaled_inputs4(25, s.ad_value(25), 1.0, s.ad_value(457), s.v[451], s.ad_value(267), ((-1.0) * s.v[451]), s.ad_value(473), s.v[451]);
        }

        if (s.b[1759] && (!s.b[1760])) {
            s.store_add_ad_rhs(23, 23, A::sub_scaled_inputs(A::sub(A::add_scaled_inputs4(s.ad_value(268), 1.0, s.ad_value(267), 1.0, s.ad_value(272), 1.0, s.ad_value(280), -1.0), s.ad_value(455)), s.v[451], s.ad_value(454), s.v[451]));
            s.store_add_scaled_inputs4(24, s.ad_value(24), 1.0, s.ad_value(280), s.v[451], s.ad_value(268), ((-1.0) * s.v[451]), s.ad_value(456), s.v[451]);
            s.store_add_scaled_inputs3(25, s.ad_value(25), 1.0, s.ad_value(457), s.v[451], s.ad_value(267), (-s.v[451]));
        }

        s.b[1761] = (p.p43 == 1.0);
        s.v[1761] = if s.b[1761] { 1.0 } else { 0.0 };

        if s.b[1761] {
            s.store_scale(36, 281, s.v[451]);
            s.store_scale(35, 282, s.v[451]);
            s.store_scale(560, 284, s.v[451]);
            s.store_scale(561, 283, s.v[451]);
        }

        if (!s.b[1761]) {
            s.store_scalar(36, 0.0);
            s.store_scalar(35, 0.0);
            s.store_scalar(560, 0.0);
            s.store_scalar(561, 0.0);
        }

        s.b[1762] = (p.p25 != 1.0);
        s.v[1762] = if s.b[1762] { 1.0 } else { 0.0 };

        if s.b[1762] {
            s.store_scalar(557, 0.0);
        }

        if (!s.b[1762]) {
            s.store_scale(557, 263, s.v[451]);
        }

        s.store_scale(15, 308, (-s.v[451]));

        s.b[1763] = (s.v[613] == 1.0);
        s.v[1763] = if s.b[1763] { 1.0 } else { 0.0 };

        if s.b[1763] {
            s.store_add_scaled_product_indices(13, 307, ((-1.0) * s.v[451]), 310, 309, s.v[451]);
        }

        if (!s.b[1763]) {
            s.store_scaled_sub_ad_lhs(13, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(310), s.ad_value(309)), 306, s.v[451]);
        }

        s.b[1764] = (s.v[613] == 1.0);
        s.v[1764] = if s.b[1764] { 1.0 } else { 0.0 };

        if s.b[1764] {
            s.store_scaled_sub_ad_lhs(14, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(310), s.ad_value(309)), 306, s.v[451]);
        }

        if (!s.b[1764]) {
            s.store_add_scaled_product_indices(14, 307, ((-1.0) * s.v[451]), 310, 309, s.v[451]);
        }

        if (s.v[613] == 1.0) {
            s.store_scale(11, 311, s.v[451]);
        } else {
            s.store_scale(11, 312, s.v[451]);
        }

        if (s.v[613] == 1.0) {
            s.store_scale(12, 312, s.v[451]);
        } else {
            s.store_scale(12, 311, s.v[451]);
        }

        s.store_scale(597, 291, s.v[451]);

        s.store_scale(598, 292, s.v[451]);

        s.store_scalar(27, A::ddx_projection(&s.ad_value(23), Some(6), None));

        s.store_scale(27, 27, p.p50);

        s.store_scalar(28, A::ddx_projection(&s.ad_value(23), Some(7), None));

        s.store_scale(28, 28, p.p50);

        if (s.v[613] > 0.0) {
            s.copy_ad(555, 28);
        } else {
            s.copy_ad(555, 27);
        }

        s.b[1771] = ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (s.v[146] == 1.0)) && (s.v[145] == 0.0));
        s.v[1771] = if s.b[1771] { 1.0 } else { 0.0 };

        if s.b[1771] {
            s.store_scaled_mul(1765, 323, 108, (1e-6 * s.v[98]));
            s.store_scale(1766, 555, 1.0 / (s.v[451]));
            s.store_div_scaled_product3_indices(1767, 227, 1766, 1766, (0.1185185185185185 * 1.6021918e-19), 300, 1.0);
        }

        s.b[1772] = ((s.v[297] > (10.0 * 2.220446049250313e-16)) && (s.v[157] > (10.0 * 2.220446049250313e-16)));
        s.v[1772] = if s.b[1772] { 1.0 } else { 0.0 };

        if (s.b[1771] && s.b[1772]) {
            s.store_div(1768, 251, 250);
            s.store_div_scaled_inputs2(1769, A::div(s.ad_value(251), s.ad_value(293)), 1.0, s.ad_value(1768), (-1.0), s.ad_value(157), 1.0);
            s.store_add_ad_rhs(1770, 1768, A::div_scaled_product(s.ad_value(1769), A::add(A::add_scaled_product(s.ad_value(294), 1.0, s.ad_value(192), s.ad_value(298), 1.0), s.ad_value(296)), 0.6666666666666667, A::add(s.ad_value(192), s.ad_value(298)), 1.0));
        }

        if (s.b[1771] && (!s.b[1772])) {
            s.store_div(1770, 251, 293);
        }

        if s.b[1771] {
            s.store_mul3_affine_lhs(558, 1767, 299, s.v[451], 0.0, 1770);
            s.copy_ad(559, 301);
        }

        if s.b[1771] {
            if (((-s.v[1766]) > s.v[1765]) && (s.v[558] > 0.0)) {
            } else {
                s.store_scalar(558, 0.0);
            }
        }

        if s.b[1771] {
            if ((-s.v[1766]) > s.v[1765]) {
            } else {
                s.store_scalar(559, 0.0);
            }
        }

        if (!s.b[1771]) {
            s.store_scalar(558, 0.0);
            s.store_scalar(559, 0.0);
        }

        s.v[4] = 0.0;

        s.v[5] = 0.0;

        s.v[7] = 0.0;

        s.v[8] = 0.0;

        s.b[1773] = (p.p259 == 1.0);
        s.v[1773] = if s.b[1773] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_31(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1773] {
            s.store_scalar(3, 1.0);
        }

        s.b[1793] = (s.v[3] == 1.0);
        s.v[1793] = if s.b[1793] { 1.0 } else { 0.0 };

        if (s.b[1773] && s.b[1793]) {
            s.store_scalar(1784, (p.p264 / 1e-6));
            s.store_scalar(1777, p.p266);
            s.store_scalar(1778, p.p268);
            s.store_scalar(1779, p.p273);
        }

        if (s.b[1773] && s.b[1793]) {
            s.store_scalar(1780, (if (p.p263 > 0.0) { (p.p263 * p.p255) } else { 0.0 }));
        }

        if (s.b[1773] && s.b[1793]) {
            s.store_scalar(1783, p.p258);
            s.store_scaled_voltage(1781, ctx, nodes, Some(7), Some(2), p.p50);
        }

        if (s.b[1773] && (!s.b[1793])) {
            s.store_scalar(1784, (p.p59 / 1e-6));
            s.store_scalar(1777, p.p265);
            s.store_scalar(1778, p.p267);
            s.store_scalar(1779, p.p272);
        }

        if (s.b[1773] && (!s.b[1793])) {
            s.store_scalar(1780, (if (p.p263 > 0.0) { (p.p263 * p.p256) } else { 0.0 }));
        }

        if (s.b[1773] && (!s.b[1793])) {
            s.store_scalar(1783, p.p257);
            s.store_scaled_voltage(1781, ctx, nodes, Some(0), Some(6), p.p50);
        }

        if s.b[1773] {
            s.store_scalar(1790, ((((p.p271 * p.p271) + (p.p56 * p.p56))) as f64).sqrt());
            s.store_scale(1792, 105, p.p9);
            s.store_scale(1777, 1777, 0.0001);
            s.store_scale(1778, 1778, 0.01);
            s.store_scale(1782, 429, 1.0 / (s.v[81]));
            s.store_powf(328, 1782, p.p269);
            s.store_div(1785, 1777, 328);
            s.store_sub_ad(327, A::add_scaled_product(A::scale_offset(s.ad_value(1782), 0.4, 1.8), 1.0, s.ad_value(1782), s.ad_value(1782), 0.1), A::scale_offset(s.ad_value(1782), (-p.p270), p.p270));
            s.store_div(1786, 1778, 327);
            s.store_add_ad_rhs(1779, 1779, A::scaled_offset(s.ad_value(429), (-s.v[81]), p.p274));
            s.store_scalar(1774, (1.0 + (p.p279 / ((s.v[100]) as f64).powf(p.p280))));
            s.store_scalar(1776, (1.0 + (p.p277 / ((s.v[100]) as f64).powf(p.p278))));
            s.store_scalar(1775, (1.0 + (p.p275 / ((s.v[109]) as f64).powf(p.p276))));
            s.store_mul(1785, 1785, 1774);
            s.store_offset_ad(1786, A::mul3(s.ad_value(1786), s.ad_value(1775), s.ad_value(1776)), 1e-50);
            s.store_div(1787, 1781, 1783);
            s.store_mul(1788, 1785, 1787);
        }

        s.b[1794] = (s.v[1781] >= 0.0);
        s.v[1794] = if s.b[1794] { 1.0 } else { 0.0 };

        if (s.b[1773] && s.b[1794]) {
            s.store_div(328, 1788, 1786);
        }

        if (s.b[1773] && (!s.b[1794])) {
            s.store_div_scaled_inputs(328, s.ad_value(1788), -1.0, s.ad_value(1786), 1.0);
        }

        s.b[1795] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1779]) && (s.v[1779] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1795] = if s.b[1795] { 1.0 } else { 0.0 };

        if (s.b[1773] && s.b[1795]) {
            s.store_scalar(330, 1.0);
        }

        s.b[1796] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1779]) && (s.v[1779] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1796] = if s.b[1796] { 1.0 } else { 0.0 };

        if ((s.b[1773] && (!s.b[1795])) && s.b[1796]) {
            s.copy_ad(330, 328);
        }

        if ((s.b[1773] && (!s.b[1795])) && (!s.b[1796])) {
            s.store_pow_ad(330, s.ad_value(328), A::offset(s.ad_value(1779), (-1.0)));
        }

        if s.b[1773] {
            s.store_mul(329, 328, 330);
            s.store_offset(331, 329, 1.0);
        }

        s.b[1797] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1779]) && (s.v[1779] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1797] = if s.b[1797] { 1.0 } else { 0.0 };

        if (s.b[1773] && s.b[1797]) {
            s.store_div_from_scalar(332, 1.0, 331);
        }

        s.b[1798] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1779]) && (s.v[1779] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1798] = if s.b[1798] { 1.0 } else { 0.0 };

        if ((s.b[1773] && (!s.b[1797])) && s.b[1798]) {
            s.store_div_from_scalar_sqrt_ad(332, 1.0, s.ad_value(331));
        }

        if ((s.b[1773] && (!s.b[1797])) && (!s.b[1798])) {
            s.store_pow_ad(333, s.ad_value(331), A::offset(A::div_from_scalar((-1.0), s.ad_value(1779)), (-1.0)));
            s.store_mul(332, 331, 333);
        }

        if s.b[1773] {
            s.store_mul(1789, 1785, 332);
            s.store_div_from_scalar(328, 1.6021918e-19, 1783);
            s.store_mul_ad_lhs(1791, A::mul3(s.ad_value(328), s.ad_value(1790), s.ad_value(1789)), 1784);
        }

        s.b[1799] = (s.v[1791] <= 0.0);
        s.v[1799] = if s.b[1799] { 1.0 } else { 0.0 };

        if (s.b[1773] && s.b[1799]) {
            s.store_scalar(1791, 1e-50);
        }

        if s.b[1773] {
            s.store_div_from_scalar(1, 1.0, 1791);
            s.store_div(1, 1, 1792);
            s.store_add(1, 1, 1780);
        }

        if s.b[1773] {
            if ((s.v[1] > 0.0001) && (p.p32 != 0.0)) {
                s.store_div_from_scalar(6, s.v[451], 1);
            } else {
                s.store_scalar(6, 0.0);
            }
        }

        s.b[1800] = (s.v[1] < 0.0001);
        s.v[1800] = if s.b[1800] { 1.0 } else { 0.0 };

        if (s.b[1773] && s.b[1800]) {
            s.store_scalar(1, 0.0001);
        }

        if s.b[1773] {
            s.store_scale(5, 1, 1.0 / (s.v[451]));
            s.copy_ad(8, 6);
        }

        s.b[1801] = (p.p260 == 1.0);
        s.v[1801] = if s.b[1801] { 1.0 } else { 0.0 };

        if s.b[1801] {
            s.store_scalar(3, 2.0);
        }

        s.b[1821] = (s.v[3] == 1.0);
        s.v[1821] = if s.b[1821] { 1.0 } else { 0.0 };

        if (s.b[1801] && s.b[1821]) {
            s.store_scalar(1812, (p.p264 / 1e-6));
            s.store_scalar(1805, p.p266);
            s.store_scalar(1806, p.p268);
            s.store_scalar(1807, p.p273);
        }

        if (s.b[1801] && s.b[1821]) {
            s.store_scalar(1808, (if (p.p263 > 0.0) { (p.p263 * p.p255) } else { 0.0 }));
        }

        if (s.b[1801] && s.b[1821]) {
            s.store_scalar(1811, p.p258);
            s.store_scaled_voltage(1809, ctx, nodes, Some(7), Some(2), p.p50);
        }

        if (s.b[1801] && (!s.b[1821])) {
            s.store_scalar(1812, (p.p59 / 1e-6));
            s.store_scalar(1805, p.p265);
            s.store_scalar(1806, p.p267);
            s.store_scalar(1807, p.p272);
        }

        if (s.b[1801] && (!s.b[1821])) {
            s.store_scalar(1808, (if (p.p263 > 0.0) { (p.p263 * p.p256) } else { 0.0 }));
        }

        if (s.b[1801] && (!s.b[1821])) {
            s.store_scalar(1811, p.p257);
            s.store_scaled_voltage(1809, ctx, nodes, Some(0), Some(6), p.p50);
        }

        if s.b[1801] {
            s.store_scalar(1818, ((((p.p271 * p.p271) + (p.p56 * p.p56))) as f64).sqrt());
            s.store_scale(1820, 105, p.p9);
            s.store_scale(1805, 1805, 0.0001);
            s.store_scale(1806, 1806, 0.01);
            s.store_scale(1810, 429, 1.0 / (s.v[81]));
            s.store_powf(328, 1810, p.p269);
            s.store_div(1813, 1805, 328);
            s.store_sub_ad(327, A::add_scaled_product(A::scale_offset(s.ad_value(1810), 0.4, 1.8), 1.0, s.ad_value(1810), s.ad_value(1810), 0.1), A::scale_offset(s.ad_value(1810), (-p.p270), p.p270));
            s.store_div(1814, 1806, 327);
            s.store_add_ad_rhs(1807, 1807, A::scaled_offset(s.ad_value(429), (-s.v[81]), p.p274));
            s.store_scalar(1802, (1.0 + (p.p279 / ((s.v[100]) as f64).powf(p.p280))));
            s.store_scalar(1804, (1.0 + (p.p277 / ((s.v[100]) as f64).powf(p.p278))));
            s.store_scalar(1803, (1.0 + (p.p275 / ((s.v[109]) as f64).powf(p.p276))));
            s.store_mul(1813, 1813, 1802);
            s.store_offset_ad(1814, A::mul3(s.ad_value(1814), s.ad_value(1803), s.ad_value(1804)), 1e-50);
            s.store_div(1815, 1809, 1811);
            s.store_mul(1816, 1813, 1815);
        }

        s.b[1822] = (s.v[1809] >= 0.0);
        s.v[1822] = if s.b[1822] { 1.0 } else { 0.0 };

        if (s.b[1801] && s.b[1822]) {
            s.store_div(328, 1816, 1814);
        }

        if (s.b[1801] && (!s.b[1822])) {
            s.store_div_scaled_inputs(328, s.ad_value(1816), -1.0, s.ad_value(1814), 1.0);
        }

        s.b[1823] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1807]) && (s.v[1807] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1823] = if s.b[1823] { 1.0 } else { 0.0 };

        if (s.b[1801] && s.b[1823]) {
            s.store_scalar(330, 1.0);
        }

        s.b[1824] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1807]) && (s.v[1807] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1824] = if s.b[1824] { 1.0 } else { 0.0 };

        if ((s.b[1801] && (!s.b[1823])) && s.b[1824]) {
            s.copy_ad(330, 328);
        }

        if ((s.b[1801] && (!s.b[1823])) && (!s.b[1824])) {
            s.store_pow_ad(330, s.ad_value(328), A::offset(s.ad_value(1807), (-1.0)));
        }

        if s.b[1801] {
            s.store_mul(329, 328, 330);
            s.store_offset(331, 329, 1.0);
        }

        s.b[1825] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1807]) && (s.v[1807] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1825] = if s.b[1825] { 1.0 } else { 0.0 };

        if (s.b[1801] && s.b[1825]) {
            s.store_div_from_scalar(332, 1.0, 331);
        }

        s.b[1826] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1807]) && (s.v[1807] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1826] = if s.b[1826] { 1.0 } else { 0.0 };

        if ((s.b[1801] && (!s.b[1825])) && s.b[1826]) {
            s.store_div_from_scalar_sqrt_ad(332, 1.0, s.ad_value(331));
        }

        if ((s.b[1801] && (!s.b[1825])) && (!s.b[1826])) {
            s.store_pow_ad(333, s.ad_value(331), A::offset(A::div_from_scalar((-1.0), s.ad_value(1807)), (-1.0)));
            s.store_mul(332, 331, 333);
        }

        if s.b[1801] {
            s.store_mul(1817, 1813, 332);
            s.store_div_from_scalar(328, 1.6021918e-19, 1811);
            s.store_mul_ad_lhs(1819, A::mul3(s.ad_value(328), s.ad_value(1818), s.ad_value(1817)), 1812);
        }

        s.b[1827] = (s.v[1819] <= 0.0);
        s.v[1827] = if s.b[1827] { 1.0 } else { 0.0 };

        if (s.b[1801] && s.b[1827]) {
            s.store_scalar(1819, 1e-50);
        }

        if s.b[1801] {
            s.store_div_from_scalar(1, 1.0, 1819);
            s.store_div(1, 1, 1820);
            s.store_add(1, 1, 1808);
        }

        if s.b[1801] {
            if ((s.v[1] > 0.0001) && (p.p32 != 0.0)) {
                s.store_div_from_scalar(6, s.v[451], 1);
            } else {
                s.store_scalar(6, 0.0);
            }
        }

        s.b[1828] = (s.v[1] < 0.0001);
        s.v[1828] = if s.b[1828] { 1.0 } else { 0.0 };

        if (s.b[1801] && s.b[1828]) {
            s.store_scalar(1, 0.0001);
        }

        if s.b[1801] {
            s.store_scale(4, 1, 1.0 / (s.v[451]));
            s.copy_ad(7, 6);
        }

        s.b[1829] = (p.p43 == 1.0);
        s.v[1829] = if s.b[1829] { 1.0 } else { 0.0 };

        s.b[1830] = (s.v[289] < (1e-15 / 0.0001));
        s.v[1830] = if s.b[1830] { 1.0 } else { 0.0 };

        if ((s.b[1829] && (s.v[85] != 0.0)) && s.b[1830]) {
            s.store_scalar(289, (1e-15 / 0.0001));
        }

        s.b[1831] = (s.v[290] < (1e-15 / 0.0001));
        s.v[1831] = if s.b[1831] { 1.0 } else { 0.0 };

        if ((s.b[1829] && (s.v[85] != 0.0)) && s.b[1831]) {
            s.store_scalar(290, (1e-15 / 0.0001));
        }

    }
}
