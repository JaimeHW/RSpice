#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[735]) && s.b[1087]) {
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

        s.b[1121] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1121] = if s.b[1121] { 1.0 } else { 0.0 };

        s.b[1122] = (4.0 == 1.0);
        s.v[1122] = if s.b[1122] { 1.0 } else { 0.0 };

        if ((((!s.b[735]) && s.b[1087]) && s.b[1121]) && s.b[1122]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1123] = (4.0 == 2.0);
        s.v[1123] = if s.b[1123] { 1.0 } else { 0.0 };

        if (((((!s.b[735]) && s.b[1087]) && s.b[1121]) && (!s.b[1122])) && s.b[1123]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1124] = (4.0 == 4.0);
        s.v[1124] = if s.b[1124] { 1.0 } else { 0.0 };

        if ((((((!s.b[735]) && s.b[1087]) && s.b[1121]) && (!s.b[1122])) && (!s.b[1123])) && s.b[1124]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1125] = (4.0 == 8.0);
        s.v[1125] = if s.b[1125] { 1.0 } else { 0.0 };

        if (((((((!s.b[735]) && s.b[1087]) && s.b[1121]) && (!s.b[1122])) && (!s.b[1123])) && (!s.b[1124])) && s.b[1125]) {
            s.store_scalar(55, 4.0);
        }

        if (((!s.b[735]) && s.b[1087]) && s.b[1121]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign16990_loop_guard: usize = 0;
        while {
            let assign16990_cond_e24542: f64 = if ((((!s.b[735]) && s.b[1087]) && s.b[1121]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign16990_cond_e24542 != 0.0
        } {
            assign16990_loop_guard += 1;
            assert!(assign16990_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[735]) && s.b[1087]) && s.b[1121]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((!s.b[735]) && s.b[1087]) && (!s.b[1121])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if ((!s.b[735]) && s.b[1087]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_scaled_mul(337, 336, 53, 1.0);
            s.store_sub_from_scalar(190, 1.0, 337);
            s.store_offset_ad(478, A::mul_offset_rhs(s.ad_value(190), s.ad_value(190), 1.0), 1.0);
        }

        if ((!s.b[735]) && s.b[1087]) {
            s.store_ad_value(479, {
                if ((1.0 + s.v[190]) >= (10.0 * 2.220446049250313e-16)) {
                    A::offset(s.ad_value(190), 1.0)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if ((!s.b[735]) && s.b[1087]) {
            s.store_ad_value(328, A::div_scaled_product(s.ad_value(192), s.ad_value(478), 0.6666666666666667, s.ad_value(479), 1.0));
        }

        s.b[1126] = (s.v[339] <= 1.0);
        s.v[1126] = if s.b[1126] { 1.0 } else { 0.0 };

        s.b[1127] = (((s.v[164]) as f64).abs() > 1e-6);
        s.v[1127] = if s.b[1127] { 1.0 } else { 0.0 };

        if ((((!s.b[735]) && s.b[1087]) && s.b[1126]) && s.b[1127]) {
            let assign17090_ad_e24712: A = A::sub(A::add_scaled_product(A::mul3(A::add_scaled_inputs(A::square(s.ad_value(425)), 1.0, A::square(s.ad_value(427)), 0.08333333333333333), s.ad_value(225), s.ad_value(164)), 1.0, s.ad_value(425), s.ad_value(427), (-1.0)), A::div_scaled_product(A::mul3(A::add_scaled_inputs(s.ad_value(425), 2.0, A::div_scaled_product3_by_product(s.ad_value(323), s.ad_value(426), s.ad_value(426), 0.2, s.ad_value(225), s.ad_value(428), 1.0), 1.0), s.ad_value(426), s.ad_value(426)), s.ad_value(426), 0.16666666666666666, s.ad_value(428), 1.0));
            s.store_ad_value(436, assign17090_ad_e24712);
        }

        if ((((!s.b[735]) && s.b[1087]) && s.b[1126]) && s.b[1127]) {
            s.store_div(436, 436, 246);
        }

        if ((((!s.b[735]) && s.b[1087]) && s.b[1126]) && (!s.b[1127])) {
            s.copy_ad(436, 425);
        }

        if (((!s.b[735]) && s.b[1087]) && (!s.b[1126])) {
            s.store_scaled_add(436, 355, 358, (-0.5));
        }

        s.b[1131] = (s.v[612] == 0.0);
        s.v[1131] = if s.b[1131] { 1.0 } else { 0.0 };

        if s.b[1131] {
            s.store_offset(480, 190, 0.5);
            s.store_mul(481, 479, 478);
            s.store_scaled_div(482, 480, 481, 0.4);
            s.store_sub_from_scalar(438, 0.6, 482);
        }

        s.b[1132] = (s.v[438] > (0.5 + 1e-8));
        s.v[1132] = if s.b[1132] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1132]) {
            s.store_scalar(438, 0.5);
        }

        if s.b[1131] {
            s.copy_ad(439, 438);
            s.store_scalar(438, 0.5);
        }

        s.b[1134] = (s.v[145] == 0.0);
        s.v[1134] = if s.b[1134] { 1.0 } else { 0.0 };

        s.b[1150] = ((p.p190 < (10.0 * 2.220446049250313e-16)) && (p.p191 < (10.0 * 2.220446049250313e-16)));
        s.v[1150] = if s.b[1150] { 1.0 } else { 0.0 };

        if ((s.b[1131] && s.b[1134]) && s.b[1150]) {
            s.store_scalar(316, 0.0);
            s.copy_ad(314, 162);
        }

        s.b[1151] = (s.v[314] > ((s.v[161] + s.v[173]) - (10.0 * 2.220446049250313e-16)));
        s.v[1151] = if s.b[1151] { 1.0 } else { 0.0 };

        if (((s.b[1131] && s.b[1134]) && s.b[1150]) && s.b[1151]) {
            s.store_offset_add(314, 161, 173, (-(10.0 * 2.220446049250313e-16)));
        }

        if ((s.b[1131] && s.b[1134]) && (!s.b[1150])) {
            s.store_scalar(1149, (if (p.p43 == 1.0) { p.p237 } else { s.v[402] }));
        }

        if ((s.b[1131] && s.b[1134]) && (!s.b[1150])) {
            s.store_div_from_scalar(1135, 1.0, 1149);
            s.store_mul(1136, 244, 1135);
            s.store_scale(1137, 1136, p.p191);
            s.store_ad_value(1140, A::add_scaled_product(s.ad_value(1137), 1.0, s.ad_value(80), s.ad_value(229), 1.0));
            s.store_div_from_scalar(1136, 1.0, 1140);
            s.store_scale(1139, 1136, 1.034943e-10);
            s.store_scalar(1136, (1.0 - p.p189));
            s.store_ad_value(314, A::add_scaled_inputs_product(s.ad_value(157), p.p189, s.ad_value(161), p.p189, s.ad_value(1136), s.ad_value(162), 1.0));
        }

        s.b[1152] = (s.v[314] > ((s.v[161] + s.v[173]) - (10.0 * 2.220446049250313e-16)));
        s.v[1152] = if s.b[1152] { 1.0 } else { 0.0 };

        if (((s.b[1131] && s.b[1134]) && (!s.b[1150])) && s.b[1152]) {
            s.store_offset_add(314, 161, 173, (-(10.0 * 2.220446049250313e-16)));
        }

        if ((s.b[1131] && s.b[1134]) && (!s.b[1150])) {
            s.store_sub(1142, 314, 162);
            s.store_sqrt_square_offset(44, 1142, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_add(1141, 1142, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1153] = (s.v[1141] < 0.0);
        s.v[1153] = if s.b[1153] { 1.0 } else { 0.0 };

        if (((s.b[1131] && s.b[1134]) && (!s.b[1150])) && s.b[1153]) {
            s.store_scalar(1141, 0.0);
        }

        if ((s.b[1131] && s.b[1134]) && (!s.b[1150])) {
            s.store_mul(1138, 225, 244);
            s.store_div_from_scalar(1136, 1.0, 1138);
            s.store_mul(1140, 246, 1136);
        }

        s.b[1154] = (s.v[1140] < s.v[227]);
        s.v[1154] = if s.b[1154] { 1.0 } else { 0.0 };

        if (((s.b[1131] && s.b[1134]) && (!s.b[1150])) && s.b[1154]) {
            s.copy_ad(1140, 227);
        }

        if ((s.b[1131] && s.b[1134]) && (!s.b[1150])) {
            s.store_scale(1146, 229, 9662367879.197212);
            s.store_scalar(1136, (100000.0 * 10000.0));
            s.store_scalar(1137, (1.0 / s.v[97]));
            s.store_mul_ad_lhs(1148, A::add_scaled_inputs_product(s.ad_value(1140), 2.0, A::mul3_scaled_output(s.ad_value(1146), s.ad_value(1141), s.ad_value(1139), 2.0), 1.0, s.ad_value(1136), s.ad_value(1139), 1.0), 1137);
            s.store_mul(1143, 1148, 1139);
            s.store_ad_value(1147, A::add_scaled_product(s.ad_value(1136), 4.0, s.ad_value(1146), s.ad_value(1141), (2.0 * 4.0)));
            s.store_mul3_lhs(1144, 1147, 1139, 1139);
            s.store_sqrt_square_add(1145, 1143, 1144);
            s.store_mul_scale_ad_rhs(316, 326, A::sub(s.ad_value(1145), s.ad_value(1143)), 0.5);
        }

        if (s.b[1131] && s.b[1134]) {
            s.store_scale(316, 316, s.v[127]);
        }

        if s.b[1131] {
            s.store_sub_from_scalar(441, s.v[97], 316);
            s.store_sub_from_scalar(442, s.v[98], 316);
        }

        s.b[1155] = (s.v[441] < 1e-9);
        s.v[1155] = if s.b[1155] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1155]) {
            s.store_scalar(441, 1e-9);
        }

        if s.b[1131] {
            s.store_scale(328, 108, (-s.v[98]));
            s.store_mul(196, 328, 437);
            s.store_mul(197, 328, 436);
            s.store_mul(198, 197, 438);
        }

        s.b[1156] = (p.p43 == 0.0);
        s.v[1156] = if s.b[1156] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1156]) {
            s.store_scale(477, 196, 0.5);
            s.store_scale(476, 196, (1.0 - 0.5));
            s.store_mul_scale_ad_lhs(392, A::add(s.ad_value(357), s.ad_value(360)), (0.5 * s.v[98]), 108);
        }

        if s.b[1131] {
            s.store_scaled_sub(1157, 157, 164, 0.5);
            s.store_scale(44, 1157, (2.0 * 1.0 / (p.p227)));
            s.store_offset_ad(45, A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul(s.ad_value(44), A::scale_offset(s.ad_value(44), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0)), 1.0);
            s.store_div_from_scalar(177, p.p227, 45);
        }

        s.b[1158] = (s.v[177] < (10.0 * 2.220446049250313e-16));
        s.v[1158] = if s.b[1158] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1158]) {
            s.store_scalar(177, (10.0 * 2.220446049250313e-16));
        }

        if s.b[1131] {
            s.store_add(176, 161, 177);
            s.store_scalar(1168, (1.034943e-10 / 100.0));
            s.store_scale(1169, 437, 0.0001);
            s.store_scale(1170, 436, 0.0001);
            s.store_div_from_scalar(1159, p.p92, 1168);
            s.store_div_from_scalar(1160, p.p93, 1168);
            s.store_scalar(1161, p.p94);
            s.store_offset_mul_ad(1162, A::sub(s.ad_value(162), s.ad_value(161)), s.ad_value(1161), 1.0);
            s.store_ad_value(1163, A::add_scaled_products(s.ad_value(1159), s.ad_value(1169), 1.0, s.ad_value(1160), s.ad_value(1170), 1.0));
            s.store_div(1164, 1163, 1162);
            s.copy_ad(248, 1164);
            s.store_sqrt_square_offset(44, 248, ((4.0 * 3000.0) * 3000.0));
            s.store_offset_scaled_add(1161, 248, 44, 0.5, (1e-10 * 3000.0));
        }

        s.b[1171] = (s.v[1161] < 0.0);
        s.v[1171] = if s.b[1171] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1171]) {
            s.store_scalar(1161, 0.0);
        }

        if s.b[1131] {
            s.store_powf(1163, 1161, (p.p97 - 1.0));
            s.store_mul(1165, 1163, 1161);
            s.store_powf(1166, 1161, (s.v[111] - 1.0));
        }

    }

    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1131] {
            s.store_mul(1167, 1166, 1161);
            s.store_scale(249, 1170, 6.241449993689894e18);
            s.store_add_scaled_ad_lhs(1159, A::add_scaled_product(A::div_from_scalar(1.0, A::scale_offset(s.ad_value(249), (p.p96 * 1e-11), p.p95)), 1.0, s.ad_value(543), s.ad_value(1165), 1.0), 1167, 1.0 / (p.p106));
            s.store_div_from_scalar(251, 1.0, 1159);
            s.store_scale(251, 251, 0.0001);
            s.store_mul3_lhs(1172, 225, 244, 441);
            s.store_sqrt_square_offset(44, 1172, ((4.0 * 1e-50) * 1e-50));
            s.store_offset_scaled_add(1172, 1172, 44, 0.5, (1e-10 * 1e-50));
        }

        s.b[1180] = (s.v[1172] < 0.0);
        s.v[1180] = if s.b[1180] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1180]) {
            s.store_scalar(1172, 0.0);
        }

        if s.b[1131] {
            s.store_div_from_scalar(1173, 1.0, 1172);
            s.store_mul(1174, 246, 1173);
            s.store_scaled_div(1172, 253, 251, 0.2);
            s.store_sqrt_square_sum(252, 1174, 1172);
            s.store_mul(1175, 251, 252);
            s.store_div(1173, 1175, 253);
        }

        s.b[1181] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1181] = if s.b[1181] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1181]) {
            s.store_scalar(1176, 1.0);
        }

        s.b[1182] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1182] = if s.b[1182] { 1.0 } else { 0.0 };

        if ((s.b[1131] && (!s.b[1181])) && s.b[1182]) {
            s.copy_ad(1176, 1173);
        }

        if ((s.b[1131] && (!s.b[1181])) && (!s.b[1182])) {
            s.store_powf(1176, 1173, (p.p113 - 1.0));
        }

        if s.b[1131] {
            s.store_mul(1172, 1173, 1176);
            s.store_offset(1177, 1172, 1.0);
        }

        s.b[1183] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1183] = if s.b[1183] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1183]) {
            s.store_div_from_scalar(1178, 1.0, 1177);
        }

        s.b[1184] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1184] = if s.b[1184] { 1.0 } else { 0.0 };

        if ((s.b[1131] && (!s.b[1183])) && s.b[1184]) {
            s.store_div_from_scalar_sqrt_ad(1178, 1.0, s.ad_value(1177));
        }

        if ((s.b[1131] && (!s.b[1183])) && (!s.b[1184])) {
            s.store_powf(1179, 1177, (((-1.0) / p.p113) - 1.0));
            s.store_mul(1178, 1177, 1179);
        }

        if s.b[1131] {
            s.store_mul(250, 251, 1178);
            s.store_ad_value(264, A::div_scaled_product(s.ad_value(107), s.ad_value(227), 1.0, A::sub_from_scalar(s.v[97], s.ad_value(316)), 1.0));
            s.store_mul3_lhs(200, 264, 246, 250);
            s.store_scalar(201, 0.0);
        }

        s.b[1194] = ((p.p281 > 0.0) && (p.p244 != 0.0));
        s.v[1194] = if s.b[1194] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1194]) {
            s.store_scaled_sub(1185, 157, 164, 0.5);
            s.store_scale(44, 1185, (2.0 * 100.0));
            s.store_offset_ad(45, A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul(s.ad_value(44), A::scale_offset(s.ad_value(44), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0)), 1.0);
            s.store_div_from_scalar(1191, 0.01, 45);
            s.store_sub_from_scalar_ad(1185, 1.1, A::add(s.ad_value(161), s.ad_value(1191)));
            s.store_sqrt_square_offset(44, 1185, ((4.0 * 0.05) * 0.05));
            s.store_offset_scaled_add(1193, 1185, 44, 0.5, (1e-10 * 0.05));
        }

        s.b[1195] = (s.v[1193] < 0.0);
        s.v[1195] = if s.b[1195] { 1.0 } else { 0.0 };

        if ((s.b[1131] && s.b[1194]) && s.b[1195]) {
            s.store_scalar(1193, 0.0);
        }

        if (s.b[1131] && s.b[1194]) {
            s.store_scale(1186, 225, s.v[116]);
            s.store_mul(1187, 323, 1186);
            s.store_powf(1186, 1193, p.p245);
            s.store_mul(1188, 1187, 1186);
            s.store_offset_scaled(1189, 173, p.p246, 1.0);
            s.store_scalar(1186, s.v[117]);
        }

        s.b[1196] = ((s.v[56] < 3.0) || (p.p43 == 1.0));
        s.v[1196] = if s.b[1196] { 1.0 } else { 0.0 };

        if ((s.b[1131] && s.b[1194]) && s.b[1196]) {
            s.store_ad_value(1190, A::add_scaled_inputs3(s.ad_value(161), 1.0, s.ad_value(1191), 1.0, s.ad_value(172), -1.0));
        }

        if ((s.b[1131] && s.b[1194]) && (!s.b[1196])) {
            s.store_ad_value(1190, A::add_scaled_inputs3(s.ad_value(161), 1.0, s.ad_value(1191), 1.0, s.ad_value(350), -1.0));
        }

        if (s.b[1131] && s.b[1194]) {
            s.store_add_ad_rhs(1189, 1189, A::mul3(s.ad_value(173), s.ad_value(1186), s.ad_value(1190)));
            s.store_mul(1191, 1188, 1189);
            s.copy_ad(1188, 1191);
        }

        if (s.b[1131] && (!s.b[1194])) {
            s.store_scalar(1188, 0.0);
        }

        s.b[1197] = (p.p248 != 0.0);
        s.v[1197] = if s.b[1197] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1197]) {
            s.store_scale(1185, 225, s.v[118]);
            s.store_mul(1193, 323, 1185);
            s.store_mul(1192, 1193, 173);
        }

        if (s.b[1131] && (!s.b[1197])) {
            s.store_scalar(1192, 0.0);
        }

        s.b[1198] = ((s.v[1188] + s.v[1192]) > 0.0);
        s.v[1198] = if s.b[1198] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1198]) {
            s.store_mul_add_rhs(247, 164, 1188, 1192);
            s.store_mul3_lhs(201, 264, 247, 250);
        }

        if s.b[1131] {
            s.store_add(199, 200, 201);
            s.copy_ad(203, 201);
        }

        s.b[1208] = (p.p33 != 0.0);
        s.v[1208] = if s.b[1208] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1208]) {
            s.copy_ad(1201, 549);
            s.store_scalar(1202, (s.v[124] - p.p71));
            s.store_div_from_scalar_square_ad(1203, 1.0, s.ad_value(1202));
            s.store_mul_ad_product_lhs(1204, A::mul_sub_from_scalar_lhs_scaled_output(p.p69, s.ad_value(233), s.ad_value(324), (2.0 * 1.034943e-10)), s.ad_value(1201), 1203);
            s.store_mul(186, 1204, 235);
            s.store_offset_scaled(1200, 173, p.p155, p.p154);
            s.store_mul(206, 186, 1200);
            s.store_sub_from_scalar_ad(1199, p.p156, A::scale(s.ad_value(157), p.p157));
            s.store_ad_value(207, A::add_scaled_inputs3_offset(s.ad_value(174), 1.0, s.ad_value(1199), 1.0, s.ad_value(206), 1.0, (-s.v[123])));
            s.store_mul3_lhs(210, 205, 324, 324);
            s.store_scaled_mul(211, 210, 225, 0.5);
            s.store_scaled_mul(212, 211, 225, 2.0);
            s.store_offset_sub_ad(1205, A::offset(A::add_scaled_product(s.ad_value(227), 1.0, s.ad_value(210), s.ad_value(225), (-0.25)), ((s.v[123]) + ((-p.p156)))), s.ad_value(206), 1e-50);
            s.store_offset_sub(1199, 174, 1205, (-0.005));
        }

        if (s.b[1131] && s.b[1208]) {
            s.store_scalar(327, (if (s.v[1205] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if (s.b[1131] && s.b[1208]) {
            s.store_sqrt_ad(1201, A::add_scaled_square_product(s.ad_value(1199), 1.0, s.ad_value(327), s.ad_value(1205), (4.0 * 0.005)));
            s.store_ad_value(1202, A::add_scaled_inputs3_offset(A::add_scaled_inputs3(s.ad_value(1205), 1.0, s.ad_value(1199), 0.5, s.ad_value(1201), 0.5), 1.0, s.ad_value(206), 1.0, s.ad_value(514), -1.0, (((-s.v[123])) + (p.p156))));
            s.store_offset_mul(1203, 225, 1202, (-1.0));
            s.store_div_from_scalar(1204, 4.0, 212);
            s.store_offset_mul(1200, 1203, 1204, 1.0);
            s.store_sqrt_square_offset(44, 1200, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_add(1199, 1200, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1209] = (s.v[1199] < 0.0);
        s.v[1209] = if s.b[1209] { 1.0 } else { 0.0 };

        if ((s.b[1131] && s.b[1208]) && s.b[1209]) {
            s.store_scalar(1199, 0.0);
        }

        if (s.b[1131] && s.b[1208]) {
            s.store_sqrt_offset_input(213, 1199, 1e-50);
            s.store_add_ad_rhs(215, 207, A::mul_sub_from_scalar_rhs(s.ad_value(211), 1.0, s.ad_value(213)));
            s.store_div_from_scalar_add_ad(327, 1.0, s.ad_value(225), A::div_from_scalar(2.0, A::offset(s.ad_value(207), 1e-50)));
            s.store_mul_ln_ad_lhs(216, A::mul(A::div(A::div_from_scalar(1.0, s.ad_value(209)), s.ad_value(210)), A::square(s.ad_value(207))), 327);
            s.store_div_ad_rhs(1202, 216, A::offset(s.ad_value(207), 1e-50));
            s.store_offset_sub(217, 216, 215, (-0.002));
            s.store_sqrt_ad(327, A::add_scaled_inputs(A::square(s.ad_value(217)), 1.0, s.ad_value(216), (4.0 * 0.002)));
            s.store_ad_value(218, A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(217), (-0.5), s.ad_value(327), (-0.5)));
            s.store_div_from_scalar(1199, 1.0, 327);
            s.store_mul_exp_ad_rhs(327, 209, A::mul(s.ad_value(225), s.ad_value(218)));
            s.store_add_ad_lhs(1200, A::offset(A::mul(s.ad_value(225), A::sub(s.ad_value(218), s.ad_value(514))), (-1.0)), 327);
            s.store_sqrt_square_offset(44, 1200, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_add(1199, 1200, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1210] = (s.v[1199] < 0.0);
        s.v[1210] = if s.b[1210] { 1.0 } else { 0.0 };

        if ((s.b[1131] && s.b[1208]) && s.b[1210]) {
            s.store_scalar(1199, 0.0);
        }

        if (s.b[1131] && s.b[1208]) {
            s.store_sqrt_offset_input(219, 1199, (10.0 * 2.220446049250313e-16));
            s.store_offset_mul_ad(1200, s.ad_value(225), A::sub(s.ad_value(218), s.ad_value(514)), (-1.0));
            s.store_sqrt_square_offset(44, 1200, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_add(1199, 1200, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1211] = (s.v[1199] < 0.0);
        s.v[1211] = if s.b[1211] { 1.0 } else { 0.0 };

        if ((s.b[1131] && s.b[1208]) && s.b[1211]) {
            s.store_scalar(1199, 0.0);
        }

        if (s.b[1131] && s.b[1208]) {
            s.store_sqrt_offset_input(220, 1199, (10.0 * 2.220446049250313e-16));
            s.store_mul_sub_rhs(221, 208, 219, 220);
            s.store_sub(1200, 215, 218);
            s.store_sqrt_square_offset(44, 1200, ((4.0 * 0.1) * 0.1));
            s.store_offset_scaled_add(1199, 1200, 44, 0.5, (1e-10 * 0.1));
        }

        s.b[1212] = (s.v[1199] < 0.0);
        s.v[1212] = if s.b[1212] { 1.0 } else { 0.0 };

        if ((s.b[1131] && s.b[1208]) && s.b[1212]) {
            s.store_scalar(1199, 0.0);
        }

        if (s.b[1131] && s.b[1208]) {
            s.store_div_ad_rhs(1206, 157, A::offset(s.ad_value(1199), (10.0 * 2.220446049250313e-16)));
            s.store_square(49, 1206);
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
        if (s.b[1131] && s.b[1208]) {
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1213] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1213] = if s.b[1213] { 1.0 } else { 0.0 };

        s.b[1214] = (4.0 == 1.0);
        s.v[1214] = if s.b[1214] { 1.0 } else { 0.0 };

        if (((s.b[1131] && s.b[1208]) && s.b[1213]) && s.b[1214]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1215] = (4.0 == 2.0);
        s.v[1215] = if s.b[1215] { 1.0 } else { 0.0 };

        if ((((s.b[1131] && s.b[1208]) && s.b[1213]) && (!s.b[1214])) && s.b[1215]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1216] = (4.0 == 4.0);
        s.v[1216] = if s.b[1216] { 1.0 } else { 0.0 };

        if (((((s.b[1131] && s.b[1208]) && s.b[1213]) && (!s.b[1214])) && (!s.b[1215])) && s.b[1216]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1217] = (4.0 == 8.0);
        s.v[1217] = if s.b[1217] { 1.0 } else { 0.0 };

        if ((((((s.b[1131] && s.b[1208]) && s.b[1213]) && (!s.b[1214])) && (!s.b[1215])) && (!s.b[1216])) && s.b[1217]) {
            s.store_scalar(55, 4.0);
        }

        if ((s.b[1131] && s.b[1208]) && s.b[1213]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign19470_loop_guard: usize = 0;
        while {
            let assign19470_cond_e26967: f64 = if (((s.b[1131] && s.b[1208]) && s.b[1213]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign19470_cond_e26967 != 0.0
        } {
            assign19470_loop_guard += 1;
            assert!(assign19470_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1131] && s.b[1208]) && s.b[1213]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((s.b[1131] && s.b[1208]) && (!s.b[1213])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if (s.b[1131] && s.b[1208]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_scaled_mul(1207, 1206, 53, 1.0);
            s.store_scale(214, 227, ((2.0 * s.v[126]) * p.p9));
            s.store_ad_value(222, A::div_scaled_product(A::mul3(s.ad_value(214), s.ad_value(250), s.ad_value(221)), s.ad_value(1207), 1.0, s.ad_value(441), 1.0));
            s.store_add(199, 199, 222);
        }

        s.b[1218] = ((p.p30 != 0.0) && (p.p32 != 0.0));
        s.v[1218] = if s.b[1218] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1218]) {
            s.store_square(294, 192);
            s.store_mul3_affine_lhs(295, 227, 324, 2.0, 0.0, 246);
            s.store_sub(296, 294, 295);
            s.store_sqrt_square_offset(44, 294, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_add(294, 294, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1219] = (s.v[294] < 0.0);
        s.v[1219] = if s.b[1219] { 1.0 } else { 0.0 };

        if ((s.b[1131] && s.b[1218]) && s.b[1219]) {
            s.store_scalar(294, 0.0);
        }

        if (s.b[1131] && s.b[1218]) {
            s.store_sqrt_square_offset(44, 296, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_add(296, 296, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1220] = (s.v[296] < 0.0);
        s.v[1220] = if s.b[1220] { 1.0 } else { 0.0 };

        if ((s.b[1131] && s.b[1218]) && s.b[1220]) {
            s.store_scalar(296, 0.0);
        }

        if (s.b[1131] && s.b[1218]) {
            s.store_sub(297, 294, 296);
        }

        s.b[1221] = ((s.v[244] < (10.0 * 2.220446049250313e-16)) || (s.v[297] < (10.0 * 2.220446049250313e-16)));
        s.v[1221] = if s.b[1221] { 1.0 } else { 0.0 };

        if ((s.b[1131] && s.b[1218]) && s.b[1221]) {
            s.store_scalar(146, 0.0);
        }

        if ((s.b[1131] && s.b[1218]) && (!s.b[1221])) {
            s.store_scalar(146, 1.0);
        }

        s.copy_ad(202, 199);

        s.v[204] = 0.0;

        s.b[1222] = ((p.p281 > 0.0) && (p.p285 > 0.0));
        s.v[1222] = if s.b[1222] { 1.0 } else { 0.0 };

        if s.b[1222] {
            s.store_scalar(1229, s.v[99]);
            s.store_scalar(1233, p.p237);
            s.store_offset_ad(1234, A::add_scaled_inputs3_offset(s.ad_value(158), 1.0, s.ad_value(185), 1.0, s.ad_value(320), -1.0, (-s.v[123])), (-p.p286));
            s.store_offset(1235, 182, p.p286);
            s.store_scalar(1237, p.p285);
            s.store_scalar(1236, p.p283);
            s.store_scalar(1227, s.v[70]);
            s.store_mul_ln_ad_rhs(1228, 227, A::div_scaled_product_by_product(s.ad_value(1227), s.ad_value(536), 1.0, s.ad_value(230), s.ad_value(230), 1.0));
        }

        if s.b[1222] {
            s.store_ad_value(1225, {
                if (p.p43 == 1.0) {
                    s.ad_value(435)
                } else {
                    s.ad_value(350)
                }
            });
        }

        if s.b[1222] {
            s.store_sqrt_ad(1230, A::div_scaled_product3(A::sub(s.ad_value(1228), s.ad_value(1225)), s.ad_value(536), s.ad_value(1227), ((2.0 * 1.6021918e-19) * 1.0 / (1.034943e-10)), A::add(s.ad_value(536), s.ad_value(1227)), 1.0));
            s.store_mul(1224, 1230, 1229);
            s.store_ad_value(1223, A::div_scaled_product(s.ad_value(1224), s.ad_value(1224), (-0.25), A::add(s.ad_value(157), s.ad_value(1224)), 1.0));
            s.copy_ad(1249, 1223);
            s.copy_ad(1250, 1235);
            s.store_offset_div_ad(336, A::scaled_offset(A::mul(s.ad_value(225), A::sub(s.ad_value(1234), s.ad_value(1249))), (-1.0), 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0);
        }

        if s.b[1222] {
            s.store_ad_value(336, {
                if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(336)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if s.b[1222] {
            s.store_add_ad_rhs(376, 1234, A::mul3_scaled_output(s.ad_value(241), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5));
        }

        s.b[1251] = (s.v[158] < ((s.v[123] + s.v[1250]) * 0.5));
        s.v[1251] = if s.b[1251] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1251]) {
            s.store_scalar(144, 0.0);
        }

        s.b[1252] = ((s.v[144] == 0.0) || (1.0 != 0.0));
        s.v[1252] = if s.b[1252] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1252]) {
            s.store_mul_sub_rhs(181, 225, 376, 1249);
        }

        s.b[1253] = (s.v[181] < 3.0);
        s.v[1253] = if s.b[1253] { 1.0 } else { 0.0 };

        if ((s.b[1222] && s.b[1252]) && s.b[1253]) {
            s.store_mul_sub_rhs(337, 225, 1234, 1249);
            s.store_div_from_scalar_ad(328, 1.0, A::mul_scaled_lhs(s.ad_value(225), (1.414213562373095 / 108.0), s.ad_value(240)));
            s.store_offset_scaled(329, 328, 3.0, 81.0);
            s.store_ad_value(330, A::add_scaled_sub_value_product((-2916.0), A::scale(s.ad_value(328), 81.0), 1.0, s.ad_value(328), s.ad_value(337), 27.0));
            s.store_ad_value(331, A::add_scaled_sub_value_product(1458.0, A::scaled_offset(s.ad_value(328), 54.0, 81.0), 1.0, s.ad_value(328), s.ad_value(337), 27.0));
            s.store_square(331, 331);
            s.store_powf_ad(332, A::add(s.ad_value(330), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(329), s.ad_value(329), s.ad_value(329), 4.0), s.ad_value(331)))), 0.3333333333333333);
            s.store_add_scaled_ad_lhs(336, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(329), 1.259921049894873, s.ad_value(332), 3.0)), 332, (1.0 / (3.0 * 1.259921049894873)));
            s.store_ad_value(376, A::add_scaled_product(s.ad_value(1249), 1.0, s.ad_value(336), s.ad_value(227), 1.0));
            s.copy_ad(378, 376);
        }

        s.b[1254] = ((s.v[158] - s.v[383]) <= s.v[1250]);
        s.v[1254] = if s.b[1254] { 1.0 } else { 0.0 };

        s.b[1255] = (p.p43 == 0.0);
        s.v[1255] = if s.b[1255] { 1.0 } else { 0.0 };

        if ((((s.b[1222] && s.b[1252]) && (!s.b[1253])) && s.b[1254]) && s.b[1255]) {
            s.store_div_from_scalar(327, 1.0, 323);
            s.store_scale(328, 1233, 9662367879.197212);
            s.store_scalar(329, (1.0 / s.v[93]));
            s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));
            s.store_mul_ad_rhs(331, 330, A::add_scaled_inputs_product(s.ad_value(1234), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));
            s.store_sub_ad_rhs(376, 1234, A::div(s.ad_value(331), s.ad_value(323)));
        }

        if (((s.b[1222] && s.b[1252]) && (!s.b[1253])) && s.b[1254]) {
            s.copy_ad(378, 376);
        }

        if (((s.b[1222] && s.b[1252]) && (!s.b[1253])) && (!s.b[1254])) {
            s.store_div_ad_lhs(328, A::div_from_scalar(1.0, s.ad_value(379)), 434);
            s.store_mul_ad_product_rhs(329, 328, A::sub(s.ad_value(1234), s.ad_value(383)), A::sub(s.ad_value(1234), s.ad_value(383)));
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(1234), s.ad_value(383))));
            s.store_offset_div_ad(377, A::ln(s.ad_value(329)), s.ad_value(330), p.p287);
            s.store_offset_sub(44, 377, 376, (-0.0008));
            s.store_scale(45, 377, (4.0 * 0.0008));
        }

        if (((s.b[1222] && s.b[1252]) && (!s.b[1253])) && (!s.b[1254])) {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((s.b[1222] && s.b[1252]) && (!s.b[1253])) && (!s.b[1254])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_ad_value(378, A::add_scaled_inputs3(s.ad_value(377), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5)));
        }

        s.b[1256] = (p.p43 == 0.0);
        s.v[1256] = if s.b[1256] { 1.0 } else { 0.0 };

        s.b[1257] = ((s.v[158] - s.v[383]) <= s.v[1250]);
        s.v[1257] = if s.b[1257] { 1.0 } else { 0.0 };

        if (((s.b[1222] && s.b[1252]) && s.b[1256]) && s.b[1257]) {
            s.store_div_from_scalar(327, 1.0, 323);
            s.store_scale(328, 1233, 9662367879.197212);
            s.store_scalar(329, (1.0 / s.v[93]));
            s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));
            s.store_mul_ad_rhs(331, 330, A::add_scaled_inputs_product(s.ad_value(1234), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));
            s.store_sub_ad_rhs(376, 1234, A::div(s.ad_value(331), s.ad_value(323)));
            s.copy_ad(378, 376);
        }

        if (((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) {
            s.store_div_from_scalar(327, 1.0, 323);
            s.store_scale(328, 1233, 9662367879.197212);
            s.store_scalar(329, (1.0 / s.v[93]));
            s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));
            s.store_mul_ad_rhs(331, 330, A::add_scaled_inputs_product(s.ad_value(1234), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));
            s.store_sub_ad_rhs(376, 1234, A::div(s.ad_value(331), s.ad_value(323)));
            s.copy_ad(378, 376);
        }

        s.b[1258] = ((s.v[1234] - s.v[383]) > 0.0);
        s.v[1258] = if s.b[1258] { 1.0 } else { 0.0 };

        if ((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) {
            s.store_div_ad_lhs(328, A::div_from_scalar(1.0, s.ad_value(379)), 434);
            s.store_mul_ad_product_rhs(329, 328, A::sub(s.ad_value(1234), s.ad_value(383)), A::sub(s.ad_value(1234), s.ad_value(383)));
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(1234), s.ad_value(383))));
            s.store_offset_div_ad(377, A::ln(s.ad_value(329)), s.ad_value(330), p.p287);
        }

        s.b[1259] = ((s.v[376] > ((s.v[377] * 0.98) - 0.4)) && (0.4 >= 0.0));
        s.v[1259] = if s.b[1259] { 1.0 } else { 0.0 };

        if (((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) {
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
        if (((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) {
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1260] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1260] = if s.b[1260] { 1.0 } else { 0.0 };

        s.b[1261] = (2.0 == 1.0);
        s.v[1261] = if s.b[1261] { 1.0 } else { 0.0 };

        if (((((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) && s.b[1260]) && s.b[1261]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1262] = (2.0 == 2.0);
        s.v[1262] = if s.b[1262] { 1.0 } else { 0.0 };

        if ((((((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) && s.b[1260]) && (!s.b[1261])) && s.b[1262]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1263] = (2.0 == 4.0);
        s.v[1263] = if s.b[1263] { 1.0 } else { 0.0 };

        if (((((((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) && s.b[1260]) && (!s.b[1261])) && (!s.b[1262])) && s.b[1263]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1264] = (2.0 == 8.0);
        s.v[1264] = if s.b[1264] { 1.0 } else { 0.0 };

        if ((((((((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) && s.b[1260]) && (!s.b[1261])) && (!s.b[1262])) && (!s.b[1263])) && s.b[1264]) {
            s.store_scalar(55, 4.0);
        }

        if ((((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) && s.b[1260]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign20700_loop_guard: usize = 0;
        while {
            let assign20700_cond_e28539: f64 = if (((((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) && s.b[1260]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign20700_cond_e28539 != 0.0
        } {
            assign20700_loop_guard += 1;
            assert!(assign20700_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) && s.b[1260]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) && (!s.b[1260])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if (((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_scaled_mul(43, 44, 53, 0.4);
            s.store_add_ad_lhs(378, A::scale_offset(s.ad_value(377), 0.98, (-0.4)), 43);
        }

        if (((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && (!s.b[1259])) {
            s.copy_ad(378, 376);
        }

        if s.b[1222] {
            s.store_offset(336, 1249, (5e-12 / 2.0));
        }

        s.b[1265] = (s.v[378] < s.v[336]);
        s.v[1265] = if s.b[1265] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1265]) {
            s.copy_ad(378, 336);
        }

        if s.b[1222] {
            s.copy_ad(1232, 378);
            s.copy_ad(163, 376);
        }

        if (s.b[1222] && (0.0 != 0.0)) {
            s.store_ad_value(166, {
                if ((s.v[376] - s.v[1232]) >= 0.0) {
                    A::sub(s.ad_value(376), s.ad_value(1232))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.b[1222] && (0.0 != 0.0)) {
            s.store_offset_scaled(44, 166, (1.0 + 0.3), (((-p.p287)) + ((-0.03))));
            s.store_scale(45, 166, ((1.0 + 0.3) * (4.0 * 0.03)));
        }

        if (s.b[1222] && (0.0 != 0.0)) {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (s.b[1222] && (0.0 != 0.0)) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_ad_value(165, A::add_scaled_inputs3(s.ad_value(166), (1.0 + 0.3), s.ad_value(44), (-0.5), s.ad_value(45), (-0.5)));
        }

        if (s.b[1222] && (0.0 != 0.0)) {
            s.store_ad_value(165, {
                if (s.v[165] <= s.v[166]) {
                    s.ad_value(165)
                } else {
                    s.ad_value(166)
                }
            });
        }

        s.b[1266] = (s.v[165] < 0.0);
        s.v[1266] = if s.b[1266] { 1.0 } else { 0.0 };

        if ((s.b[1222] && (0.0 != 0.0)) && s.b[1266]) {
            s.store_scalar(165, 0.0);
        }

        s.b[1267] = (s.v[165] > s.v[157]);
        s.v[1267] = if s.b[1267] { 1.0 } else { 0.0 };

        if (((s.b[1222] && (0.0 != 0.0)) && (!s.b[1266])) && s.b[1267]) {
            s.copy_ad(165, 157);
        }

        if (s.b[1222] && (0.0 != 0.0)) {
            s.store_add(163, 1232, 165);
        }

        s.b[1268] = (p.p282 == 1.0);
        s.v[1268] = if s.b[1268] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1268]) {
            s.copy_ad(378, 1232);
            s.copy_ad(1269, 1223);
            s.store_offset_ad(160, A::add_scaled_inputs3_offset(s.ad_value(185), (-1.0), s.ad_value(320), 1.0, s.ad_value(1269), 1.0, s.v[123]), p.p286);
        }

        s.b[1271] = (s.v[158] < s.v[160]);
        s.v[1271] = if s.b[1271] { 1.0 } else { 0.0 };

        if ((s.b[1222] && s.b[1268]) && s.b[1271]) {
            s.store_scalar(338, (-1.0));
            s.store_mul_scaled_ad_rhs(254, 227, 2.0, A::ln(A::div_from_scalar((-s.v[139]), s.ad_value(240))));
            s.store_mul_sub_rhs(336, 225, 1234, 1269);
            s.store_div_from_scalar_mul_ad(328, 1.0, s.ad_value(225), s.ad_value(238));
            s.store_mul(337, 328, 323);
            s.store_offset_scaled(262, 337, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(260, 262, 262, 8.0, 0.0, 262);
            s.store_offset(331, 336, (-2.0));
            s.store_scaled_mul(332, 337, 331, 9.0);
            s.store_sub_from_scalar(261, (7.0 * 1.414213562373095), 332);
            s.store_square(259, 261);
        }

        s.b[1272] = (s.v[260] < (s.v[259] * 1e-8));
        s.v[1272] = if s.b[1272] { 1.0 } else { 0.0 };

        if (((s.b[1222] && s.b[1268]) && s.b[1271]) && s.b[1272]) {
            s.store_ad_value(257, A::add_scaled_inputs3_offset(s.ad_value(261), 1.0, A::div_scaled_inputs(s.ad_value(260), 0.5, s.ad_value(261), 1.0), 1.0, s.ad_value(332), 1.0, ((-7.0) * 1.414213562373095)));
        }

        if (((s.b[1222] && s.b[1268]) && s.b[1271]) && (!s.b[1272])) {
            s.store_sqrt_add(258, 260, 259);
            s.store_add_ad_lhs(257, A::offset(s.ad_value(258), ((-7.0) * 1.414213562373095)), 332);
        }

        if ((s.b[1222] && s.b[1268]) && s.b[1271]) {
            s.store_powf(256, 257, 0.3333333333333333);
            s.store_ad_value(255, A::add_scaled_inputs_product(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(337), 12.0)), 1.0, s.ad_value(256), 2.0, s.ad_value(256), s.ad_value(256), 1.414213562373095));
            s.store_div_from_scalar(328, 1.0, 256);
            s.store_mul(181, 255, 328);
            s.store_ad_value(313, A::add_scaled_product(s.ad_value(1269), 1.0, s.ad_value(181), s.ad_value(227), 1.0));
            s.store_sub(328, 313, 1269);
            s.store_div(329, 328, 254);
            s.store_sqrt_square_offset(330, 329, 1.0);
            s.store_add_ad_lhs(1232, A::div(s.ad_value(328), s.ad_value(330)), 1269);
        }

        if ((s.b[1222] && s.b[1268]) && (!s.b[1271])) {
            s.store_exp_ad(484, A::mul_offset_rhs(s.ad_value(225), s.ad_value(1269), (-p.p287)));
            s.store_scalar(430, 0.0);
            s.copy_ad(1270, 378);
            s.store_scale(419, 229, ((p.p237 * (p.p237 * 0.5)) * 9662367879.197212));
            s.store_sqrt_ad(327, A::mul_scaled_lhs(s.ad_value(225), 2.0, s.ad_value(419)));
            s.store_scaled_add_ad(328, A::exp(s.ad_value(327)), A::exp_scaled_input(s.ad_value(327), -1.0), 0.5);
            s.store_div_ad_lhs(420, A::ln(s.ad_value(328)), 419);
            s.store_scalar(167, 1.0);
        }

        let mut assign21300_loop_guard: usize = 0;
        while {
            let assign21300_cond_e29269: f64 = (s.v[57] + 1.0);
            let assign21300_cond_e29271: f64 = if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (s.v[167] <= assign21300_cond_e29269)) { 1.0 } else { 0.0 };
            assign21300_cond_e29271 != 0.0
        } {
            assign21300_loop_guard += 1;
            assert!(assign21300_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1222] && s.b[1268]) && (!s.b[1271])) {
                s.store_sub(417, 1270, 1269);
                s.store_mul(181, 225, 417);
                s.store_mul_sub_rhs(337, 420, 417, 419);
            }
            s.b[1273] = (s.v[337] < 80.0);
            s.v[1273] = if s.b[1273] { 1.0 } else { 0.0 };
            if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && s.b[1273]) {
                s.store_exp(328, 337);
                s.store_exp_ad(327, A::mul_scaled_lhs(s.ad_value(420), -1.0, s.ad_value(419)));
                s.store_sub(329, 328, 327);
                s.store_div_ad_lhs(422, A::ln(A::offset(s.ad_value(329), 1.0)), 420);
                s.store_div_ad_rhs(423, 328, A::offset(s.ad_value(329), 1.0));
            }
            if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1273])) {
                s.store_sub(422, 417, 419);
                s.store_scalar(423, 1.0);
            }
            if ((s.b[1222] && s.b[1268]) && (!s.b[1271])) {
                s.store_mul(421, 225, 422);
            }
            s.b[1274] = (((s.v[181]) as f64).abs() < 1e-16);
            s.v[1274] = if s.b[1274] { 1.0 } else { 0.0 };
            if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && s.b[1274]) {
                s.store_sqrt_scaled_ad(327, A::sub_from_scalar(1.0, A::square(s.ad_value(423))), 1.0 / (2.0));
                s.store_mul(242, 181, 327);
                s.store_mul(443, 225, 327);
            }
            s.b[1275] = (s.v[181] < 0.0);
            s.v[1275] = if s.b[1275] { 1.0 } else { 0.0 };
            if ((((s.b[1222] && s.b[1268]) && (!s.b[1271])) && s.b[1274]) && s.b[1275]) {
                s.store_neg(242, 242);
                s.store_neg(443, 443);
            }
            s.b[1276] = (((s.v[181]) as f64).abs() < 0.005);
            s.v[1276] = if s.b[1276] { 1.0 } else { 0.0 };
            if ((((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1274])) && s.b[1276]) {
                s.store_ad_value(327, A::mul_sub_from_scalar_rhs_scaled_output(A::square(s.ad_value(181)), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::scale(s.ad_value(181), 0.2), 1.0 / (4.0)), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_mul_sub_from_scalar_ad_rhs(328, 181, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::scale(s.ad_value(181), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_ad_value(329, A::mul_sub_from_scalar_rhs_scaled_output(A::square(s.ad_value(421)), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::scale(s.ad_value(421), 0.2), 1.0 / (4.0)), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_mul_sub_from_scalar_ad_rhs(330, 421, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::scale(s.ad_value(421), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sqrt_sub(242, 327, 329);
                s.store_ad_value(443, A::div_scaled_product(s.ad_value(225), A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(423), s.ad_value(330), (-1.0)), 0.5, s.ad_value(242), 1.0));
            }
            if ((((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1274])) && (!s.b[1276])) {
                s.store_exp_neg_input(327, 181);
                s.store_exp_neg_input(328, 421);
                s.store_sqrt_add_ad(242, A::sub(s.ad_value(181), s.ad_value(421)), A::sub(s.ad_value(327), s.ad_value(328)));
                s.store_ad_value(443, A::div_scaled_product(s.ad_value(225), A::sub(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul_sub_from_scalar_rhs(s.ad_value(423), 1.0, s.ad_value(328))), 0.5, s.ad_value(242), 1.0));
            }
            s.b[1277] = ((s.v[430] == 1.0) && (s.v[181] < 0.0));
            s.v[1277] = if s.b[1277] { 1.0 } else { 0.0 };
            if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && s.b[1277]) {
                s.store_scalar(338, (-1.0));
            }
            s.b[1278] = (s.v[181] < 0.0);
            s.v[1278] = if s.b[1278] { 1.0 } else { 0.0 };
            if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && s.b[1278]) {
                s.store_neg(490, 242);
                s.store_neg(491, 443);
            }
            s.b[1279] = (s.v[181] < 1e-7);
            s.v[1279] = if s.b[1279] { 1.0 } else { 0.0 };
            if ((((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1278])) && s.b[1279]) {
                s.copy_ad(490, 242);
                s.copy_ad(491, 443);
            }
            if ((((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1278])) && (!s.b[1279])) {
                s.store_mul_offset_rhs(501, 225, 1270, (-p.p287));
                s.store_exp(502, 501);
                s.store_mul_ad_rhs(488, 379, A::add_scaled_offset_product_rhs(s.ad_value(502), 1.0, s.ad_value(484), s.ad_value(181), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(489, 379, s.ad_value(225), A::sub(s.ad_value(502), s.ad_value(484)));
                s.store_sqrt_square_add(490, 242, 488);
                s.store_div_ad_lhs(491, A::add_scaled_product(s.ad_value(489), 0.5, s.ad_value(443), s.ad_value(242), (2.0 * 0.5)), 490);
            }
            if ((s.b[1222] && s.b[1268]) && (!s.b[1271])) {
                s.store_ad_value(492, A::add_scaled_inputs_product(s.ad_value(1270), 1.0, s.ad_value(1234), (-1.0), s.ad_value(240), s.ad_value(490), 1.0));
                s.store_offset_mul(493, 240, 491, 1.0);
            }
            s.b[1280] = (s.v[430] == 1.0);
            s.v[1280] = if s.b[1280] { 1.0 } else { 0.0 };
            if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && s.b[1280]) {
                s.store_scalar(167, (s.v[57] + 1.0));
            }
            if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1280])) {
                s.store_scaled_div(494, 492, 493, -1.0);
            }
            if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1280])) {
                s.store_scaled_offset_ad(496, {
                    if (1.0 >= ((s.v[1270]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1270))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1281] = (((s.v[494]) as f64).abs() > s.v[496]);
            s.v[1281] = if s.b[1281] { 1.0 } else { 0.0 };
            if ((((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1280])) && s.b[1281]) {
                s.store_scale(494, 496, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1280])) {
                s.store_add(1270, 1270, 494);
            }
            s.b[1282] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[492]) as f64).abs() <= 1e-8));
            s.v[1282] = if s.b[1282] { 1.0 } else { 0.0 };
            if ((((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1280])) && s.b[1282]) {
                s.store_scalar(430, 1.0);
            }
            if ((s.b[1222] && s.b[1268]) && (!s.b[1271])) {
                s.store_offset(167, 167, 1.0);
            }
        }

        if ((s.b[1222] && s.b[1268]) && (!s.b[1271])) {
            s.copy_ad(1232, 1270);
        }

        if s.b[1222] {
            s.store_mul_scaled_ad_rhs(332, 225, -1.0, A::sub(s.ad_value(1232), s.ad_value(1223)));
        }

        if s.b[1222] {
            s.store_scalar(1247, (if (s.v[332] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if s.b[1222] {
            s.store_mul(1248, 1247, 332);
            s.store_exp(333, 332);
            s.store_sub_ad_lhs(334, A::offset(s.ad_value(333), (-1.0)), 332);
        }

    }

    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1283] = (s.v[332] > 1e-7);
        s.v[1283] = if s.b[1283] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1283]) {
            s.store_mul_scaled_ad_rhs(437, 238, -1.0, A::sqrt(s.ad_value(334)));
        }

        s.b[1284] = (s.v[1248] > 1e-7);
        s.v[1284] = if s.b[1284] { 1.0 } else { 0.0 };

        if ((s.b[1222] && (!s.b[1283])) && s.b[1284]) {
            s.store_mul_sqrt_rhs(437, 238, 334);
        }

        if ((s.b[1222] && (!s.b[1283])) && (!s.b[1284])) {
            s.store_mul_ad_affine_product_rhs(437, 1247, s.ad_value(1248), A::sqrt(A::offset(A::mul_scaled_lhs(s.ad_value(1248), 0.3333333333333333, A::scale_offset(s.ad_value(1248), 0.25, 1.0)), 1.0)), (-0.7071067811865475), 0.0);
        }

        if s.b[1222] {
            s.store_sqrt_square_offset(44, 437, ((4.0 * 1e-6) * 1e-6));
            s.store_offset_scaled_add(1244, 437, 44, 0.5, (1e-10 * 1e-6));
        }

        s.b[1285] = (s.v[1244] < 0.0);
        s.v[1285] = if s.b[1285] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1285]) {
            s.store_scalar(1244, 0.0);
        }

        if s.b[1222] {
            s.store_scaled_div(1245, 1244, 536, (1.0 / (1.6021918e-19)));
            s.store_sub(328, 1245, 1236);
            s.store_scale(1246, 1245, 0.01);
            s.store_sqrt_ad(44, A::add_scaled_square_product(s.ad_value(328), 1.0, s.ad_value(1246), s.ad_value(1246), 4.0));
            s.store_ad_value(329, A::add_scaled_inputs3(s.ad_value(328), 0.5, s.ad_value(44), 0.5, s.ad_value(1246), 1e-10));
        }

        s.b[1286] = (s.v[329] < 0.0);
        s.v[1286] = if s.b[1286] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1286]) {
            s.store_scalar(329, 0.0);
        }

        if s.b[1222] {
            s.store_ad_value(1243, A::div_scaled_product_by_product(s.ad_value(329), s.ad_value(329), 1.0, s.ad_value(1245), s.ad_value(1245), 1.0));
            s.store_ad_value(1226, A::add_scaled_product(s.ad_value(1223), 1.0, A::sub(s.ad_value(1232), s.ad_value(1223)), s.ad_value(1243), 1.0));
            s.store_sub_ad(337, A::exp(A::mul(s.ad_value(225), s.ad_value(1226))), A::exp(A::mul(s.ad_value(225), A::sub(s.ad_value(1226), s.ad_value(157)))));
            s.store_sqrt_scaled_input(1239, 1227, ((2.0 * 1.6021918e-19) * 1.034943e-10));
            s.store_mul_sqrt_rhs(1240, 1239, 227);
            s.store_mul_sub_rhs(1231, 225, 1226, 1223);
        }

        s.b[1287] = ((s.v[1231] < (0.2 * s.v[225])) && ((0.2 * s.v[225]) >= 0.0));
        s.v[1287] = if s.b[1287] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1287]) {
            s.store_sub_scaled_inputs(44, 225, 0.2, 1231, 1.0);
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

        s.b[1288] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[1288] = if s.b[1288] { 1.0 } else { 0.0 };

        s.b[1289] = (1.0 == 1.0);
        s.v[1289] = if s.b[1289] { 1.0 } else { 0.0 };

        if (((s.b[1222] && s.b[1287]) && s.b[1288]) && s.b[1289]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1290] = (1.0 == 2.0);
        s.v[1290] = if s.b[1290] { 1.0 } else { 0.0 };

        if ((((s.b[1222] && s.b[1287]) && s.b[1288]) && (!s.b[1289])) && s.b[1290]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1291] = (1.0 == 4.0);
        s.v[1291] = if s.b[1291] { 1.0 } else { 0.0 };

        if (((((s.b[1222] && s.b[1287]) && s.b[1288]) && (!s.b[1289])) && (!s.b[1290])) && s.b[1291]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1292] = (1.0 == 8.0);
        s.v[1292] = if s.b[1292] { 1.0 } else { 0.0 };

        if ((((((s.b[1222] && s.b[1287]) && s.b[1288]) && (!s.b[1289])) && (!s.b[1290])) && (!s.b[1291])) && s.b[1292]) {
            s.store_scalar(55, 4.0);
        }

        if ((s.b[1222] && s.b[1287]) && s.b[1288]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign21830_loop_guard: usize = 0;
        while {
            let assign21830_cond_e30586: f64 = if (((s.b[1222] && s.b[1287]) && s.b[1288]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign21830_cond_e30586 != 0.0
        } {
            assign21830_loop_guard += 1;
            assert!(assign21830_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1222] && s.b[1287]) && s.b[1288]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((s.b[1222] && s.b[1287]) && (!s.b[1288])) {
            s.store_powf(53, 53, (1.0 / 2.0));
        }

        if (s.b[1222] && s.b[1287]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_affine_lhs(43, 44, 225, 0.2, 0.0, 53);
            s.store_sub_scaled_inputs(328, 225, 0.2, 43, 1.0);
        }

        if (s.b[1222] && (!s.b[1287])) {
            s.copy_ad(328, 1231);
        }

        if s.b[1222] {
            s.store_sqrt_offset_input(1241, 328, (10.0 * 2.220446049250313e-16));
            s.store_mul(1242, 1240, 1241);
            s.store_mul_ad_lhs(1238, A::div_scaled_inputs(s.ad_value(227), 2.0, s.ad_value(1229), 1.0), 1242);
            s.store_mul_ad_lhs(204, A::mul3(s.ad_value(1238), s.ad_value(1237), s.ad_value(107)), 337);
            s.store_add(199, 202, 204);
        }

        s.store_add(201, 203, 204);

        s.b[1293] = ((p.p43 == 1.0) || (p.p45 == 1.0));
        s.v[1293] = if s.b[1293] { 1.0 } else { 0.0 };

        s.b[1306] = ((s.v[145] == 1.0) || (p.p25 == 0.0));
        s.v[1306] = if s.b[1306] { 1.0 } else { 0.0 };

        if (s.b[1293] && s.b[1306]) {
            s.store_scalar(263, 0.0);
        }

        s.b[1307] = ((p.p117 <= 0.0) || (s.v[73] <= 0.0));
        s.v[1307] = if s.b[1307] { 1.0 } else { 0.0 };

        if ((s.b[1293] && (!s.b[1306])) && s.b[1307]) {
            s.store_scalar(263, 0.0);
        }

        if ((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) {
            s.store_offset_ad(445, A::add_scaled_inputs3_offset(s.ad_value(174), 1.0, s.ad_value(185), 1.0, s.ad_value(320), -1.0, (-s.v[136])), p.p48);
        }

        s.b[1308] = (p.p44 <= 0.0);
        s.v[1308] = if s.b[1308] { 1.0 } else { 0.0 };

        if (((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && s.b[1308]) {
            s.copy_ad(1294, 445);
            s.store_square(1301, 323);
            s.copy_ad(1302, 545);
            s.store_div(1296, 1302, 1301);
            s.store_div_from_scalar(1303, 2.0, 1302);
            s.store_mul(1297, 1303, 1301);
            s.store_ad_value(1298, A::add_scaled_inputs_product(s.ad_value(1294), 1.0, s.ad_value(227), (-1.0), s.ad_value(130), s.ad_value(514), (-1.0)));
            s.store_scale(483, 393, (p.p49 * 1.0 / (s.v[89])));
            s.store_ad_value(1298, A::add_scaled_product(s.ad_value(1298), 1.0, s.ad_value(130), s.ad_value(483), (-1.0)));
            s.store_offset_mul(1300, 1297, 1298, 1.0);
            s.store_sqrt_square_offset(44, 1300, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_add(1299, 1300, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1309] = (s.v[1299] < 0.0);
        s.v[1309] = if s.b[1309] { 1.0 } else { 0.0 };

        if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && s.b[1308]) && s.b[1309]) {
            s.store_scalar(1299, 0.0);
        }

        if (((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && s.b[1308]) {
            s.store_offset(1299, 1299, 1e-50);
            s.store_sqrt(1299, 1299);
            s.store_ad_value(1304, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(1296), 1.0, s.ad_value(1299)), 1.0, s.ad_value(1294), s.ad_value(137), 1.0));
            s.store_ad_value(1305, A::add_scaled_inputs3(s.ad_value(173), p.p122, s.ad_value(176), 1.0, A::mul3(s.ad_value(131), s.ad_value(129), s.ad_value(1304)), -1.0));
            s.store_sqrt_square_offset(44, 1305, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_add(1305, 1305, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1310] = (s.v[1305] < 0.0);
        s.v[1310] = if s.b[1310] { 1.0 } else { 0.0 };

        if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && s.b[1308]) && s.b[1310]) {
            s.store_scalar(1305, 0.0);
        }

        if (((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) {
            s.store_mul(1294, 134, 445);
            s.store_div_ad_rhs(1296, 545, A::square(s.ad_value(323)));
            s.store_mul_ad(1297, A::div_from_scalar(2.0, s.ad_value(545)), A::square(s.ad_value(323)));
            s.store_ad_value(1298, A::add_scaled_inputs_product(s.ad_value(1294), 1.0, s.ad_value(227), (-1.0), s.ad_value(130), s.ad_value(514), (-1.0)));
            s.store_scale(483, 393, (p.p49 * 1.0 / (s.v[89])));
            s.store_ad_value(1298, A::add_scaled_product(s.ad_value(1298), 1.0, s.ad_value(130), s.ad_value(483), (-1.0)));
            s.store_offset_mul(1299, 1297, 1298, 1.0);
            s.store_scaled_offset(1301, 1297, 1.0, 2.0);
        }

        s.b[1311] = ((s.v[1299] < (1e-50 + s.v[1301])) && (s.v[1301] >= 0.0));
        s.v[1311] = if s.b[1311] { 1.0 } else { 0.0 };

        if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) {
            s.store_sub_ad_lhs(44, A::offset(s.ad_value(1301), 1e-50), 1299);
            s.store_square(49, 44);
            s.store_square(50, 1301);
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

        s.b[1312] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1312] = if s.b[1312] { 1.0 } else { 0.0 };

        s.b[1313] = (4.0 == 1.0);
        s.v[1313] = if s.b[1313] { 1.0 } else { 0.0 };

        if ((((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) && s.b[1313]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1314] = (4.0 == 2.0);
        s.v[1314] = if s.b[1314] { 1.0 } else { 0.0 };

        if (((((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) && (!s.b[1313])) && s.b[1314]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1315] = (4.0 == 4.0);
        s.v[1315] = if s.b[1315] { 1.0 } else { 0.0 };

        if ((((((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) && (!s.b[1313])) && (!s.b[1314])) && s.b[1315]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1316] = (4.0 == 8.0);
        s.v[1316] = if s.b[1316] { 1.0 } else { 0.0 };

        if (((((((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) && (!s.b[1313])) && (!s.b[1314])) && (!s.b[1315])) && s.b[1316]) {
            s.store_scalar(55, 4.0);
        }

        if (((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign22620_loop_guard: usize = 0;
        while {
            let assign22620_cond_e31705: f64 = if ((((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign22620_cond_e31705 != 0.0
        } {
            assign22620_loop_guard += 1;
            assert!(assign22620_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && (!s.b[1312])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) {
            s.store_div_from_scalar(53, 1.0, 53);
        }

    }

    pub(super) fn stamp_transient_block_21(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) {
            s.store_mul3_lhs(43, 44, 1301, 53);
            s.store_sub_ad_lhs(1299, A::offset(s.ad_value(1301), 1e-50), 43);
        }

        if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && (!s.b[1311])) {
        }

        if (((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) {
            s.store_ad_value(1299, {
                if (s.v[1299] <= 0.0) {
                    A::constant(0.0)
                } else {
                    A::sqrt(s.ad_value(1299))
                }
            });
        }

        if (((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) {
            s.store_add_ad_rhs(1304, 1294, A::mul_sub_from_scalar_rhs(s.ad_value(1296), 1.0, s.ad_value(1299)));
            s.store_div_from_scalar_offset_input(1295, s.v[100], 131, s.v[100]);
            s.store_ad_value(1305, A::add_scaled_inputs_product(s.ad_value(173), p.p122, s.ad_value(176), 1.0, s.ad_value(1295), s.ad_value(1304), (-1.0)));
            s.store_sqrt_square_offset(44, 1305, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_add(1305, 1305, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1317] = (s.v[1305] < 0.0);
        s.v[1317] = if s.b[1317] { 1.0 } else { 0.0 };

        if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1317]) {
            s.store_scalar(1305, 0.0);
        }

        if ((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) {
            s.store_offset(1305, 1305, 1e-50);
            s.store_exp_ad(1295, A::div_scaled_inputs(s.ad_value(133), -1.0, s.ad_value(1305), 1.0));
            s.store_mul_ad_lhs(263, A::mul3(s.ad_value(132), s.ad_value(1305), s.ad_value(199)), 1295);
        }

        s.b[1318] = (((p.p25 == 1.0) && (p.p26 == 2.0)) && (p.p43 == 1.0));
        s.v[1318] = if s.b[1318] { 1.0 } else { 0.0 };

        if s.b[1318] {
            s.store_mul_scaled_ad_rhs(1319, 107, (1.6021918e-19 * p.p237), A::exp_scaled_input(s.ad_value(225), (-p.p141)));
            s.store_scale(1322, 227, 0.0);
            s.store_ad_value(44, A::add_scaled_inputs3(s.ad_value(231), 1.0, s.ad_value(1322), (-1.0), s.ad_value(231), (-0.01)));
            s.store_scaled_mul(45, 231, 231, (4.0 * 0.01));
        }

        if s.b[1318] {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if s.b[1318] {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_ad_value(1322, A::add_scaled_inputs3(s.ad_value(231), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5)));
            s.store_sqrt_ad(1323, A::mul_scaled_lhs(s.ad_value(544), ((2.0 * 1.034943e-10) * 1.6021918e-19), s.ad_value(227)));
            s.store_mul_sub_rhs(1324, 225, 176, 1322);
        }

        if s.b[1318] {
            s.store_ad_value(1324, {
                if (s.v[1324] > 0.0) {
                    A::sqrt(s.ad_value(1324))
                } else {
                    A::neg(A::sqrt_scaled_input(s.ad_value(1324), -1.0))
                }
            });
        }

        if s.b[1318] {
            s.store_sqrt_mul(1325, 225, 176);
            s.store_mul_scaled_ad_rhs(1326, 1323, -1.0, A::sub(s.ad_value(1324), s.ad_value(1325)));
            s.store_offset_sub_from_scalar_ad(44, p.p47, s.ad_value(1326), (-(p.p47 * 0.01)));
            s.store_scalar(45, ((4.0 * p.p47) * (p.p47 * 0.01)));
        }

        if s.b[1318] {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if s.b[1318] {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_sub_from_scalar_ad(393, p.p47, A::add_scaled_inputs(s.ad_value(44), 0.5, s.ad_value(45), 0.5));
        }

        if s.b[1318] {
            s.store_scalar(1319, (if (p.p138 > 0.0) { p.p138 } else { 1.0 }));
        }

        if s.b[1318] {
            s.store_div_ad_rhs(398, 1319, A::offset(s.ad_value(263), p.p139));
            s.store_mul(397, 398, 323);
            s.copy_ad(396, 393);
            s.store_scaled_voltage(596, ctx, nodes, Some(17), None, (1e-9 / 0.0001));
            s.copy_ad(393, 596);
            s.store_div_ad_lhs(592, A::sub(s.ad_value(596), s.ad_value(396)), 397);
        }

        s.b[1340] = (((s.v[145] == 0.0) && (s.v[263] > 0.0)) && (p.p146 != 0.0));
        s.v[1340] = if s.b[1340] { 1.0 } else { 0.0 };

        s.b[1341] = (s.v[56] < 3.0);
        s.v[1341] = if s.b[1341] { 1.0 } else { 0.0 };

        if (s.b[1340] && s.b[1341]) {
            s.store_scalar(516, 0.0);
            s.store_scalar(517, 0.0);
        }

        if (s.b[1340] && (!s.b[1341])) {
            s.store_ad_value(516, {
                if (p.p43 == 1.0) {
                    s.ad_value(156)
                } else {
                    s.ad_value(350)
                }
            });
        }

        if (s.b[1340] && (!s.b[1341])) {
            s.store_ad_value(517, {
                if (p.p43 == 1.0) {
                    s.ad_value(156)
                } else {
                    s.ad_value(353)
                }
            });
        }

        if s.b[1340] {
            s.store_offset_scaled(1327, 185, p.p147, 1.0);
            s.store_scaled_mul(1328, 1327, 263, p.p146);
            s.store_offset_mul_ad(1329, s.ad_value(225), A::sub(s.ad_value(161), s.ad_value(516)), (-1.0));
            s.store_sqrt_square_offset(44, 1329, ((4.0 * 0.1) * 0.1));
            s.store_offset_scaled_add(1329, 1329, 44, 0.5, (1e-10 * 0.1));
        }

        s.b[1342] = (s.v[1329] < 0.0);
        s.v[1342] = if s.b[1342] { 1.0 } else { 0.0 };

        if (s.b[1340] && s.b[1342]) {
            s.store_scalar(1329, 0.0);
        }

        if s.b[1340] {
            s.store_sqrt(1330, 1329);
            s.store_mul(1331, 1329, 1330);
            s.store_offset_mul_ad(1332, s.ad_value(225), A::sub(s.ad_value(162), s.ad_value(517)), (-1.0));
            s.store_sqrt_square_offset(44, 1332, ((4.0 * 0.1) * 0.1));
            s.store_offset_scaled_add(1332, 1332, 44, 0.5, (1e-10 * 0.1));
        }

        s.b[1343] = (s.v[1332] < 0.0);
        s.v[1343] = if s.b[1343] { 1.0 } else { 0.0 };

        if (s.b[1340] && s.b[1343]) {
            s.store_scalar(1332, 0.0);
        }

        if s.b[1340] {
            s.store_sqrt(1333, 1332);
            s.store_mul(1334, 1332, 1333);
            s.store_div_from_scalar(1335, 1.0, 1329);
            s.store_mul3_lhs(328, 225, 1328, 1335);
            s.store_div_from_scalar(1335, 1.0, 1332);
            s.store_mul3_lhs(1336, 225, 1328, 1335);
            s.store_mul_ad_rhs(1337, 238, A::add_scaled_products(s.ad_value(1334), s.ad_value(1336), 1.0, s.ad_value(1331), s.ad_value(328), (-1.0)));
            s.store_mul_scaled_ad_rhs(1338, 238, 0.5, A::add_scaled_products(s.ad_value(1333), s.ad_value(1336), -1.0, s.ad_value(1330), s.ad_value(328), 1.0));
            s.store_add(1339, 1337, 1338);
            s.store_mul3_lhs(265, 264, 1339, 250);
        }

        s.v[1357] = (s.v[88] * 100.0);

        s.store_scale(1358, 323, 0.0001);

        s.v[1359] = (s.v[97] * 100.0);

        s.store_scale(1360, 107, 100.0);

        s.store_scale(1361, 252, 0.01);

        s.store_scale(1362, 436, 0.0001);

        s.store_scale(1363, 238, 0.0001);

        s.b[1364] = (p.p27 == 0.0);
        s.v[1364] = if s.b[1364] { 1.0 } else { 0.0 };

        if s.b[1364] {
            s.store_scalar(309, 0.0);
            s.store_scalar(306, 0.0);
            s.store_scalar(307, 0.0);
            s.store_scalar(308, 0.0);
            s.store_scalar(310, 0.0);
        }

        s.b[1365] = (s.v[145] == 0.0);
        s.v[1365] = if s.b[1365] { 1.0 } else { 0.0 };

        if ((!s.b[1364]) && s.b[1365]) {
            s.store_offset_add(1356, 176, 173, (-(10.0 * 2.220446049250313e-16)));
            s.store_sub_scaled_ad_lhs(1346, A::add_scaled_inputs3_offset(s.ad_value(174), 1.0, s.ad_value(185), (p.p216 * s.v[1359]), s.ad_value(320), (-(p.p216 * s.v[1359])), (-s.v[123])), 1356, p.p215);
            s.store_scalar(1348, (1.0 / s.v[1357]));
            s.store_mul(1347, 1346, 1348);
            s.store_scalar(1348, (1.0 / p.p217));
            s.store_offset_mul(1352, 1361, 1348, 1.0);
            s.store_mul(1355, 1347, 1352);
            s.store_sqrt_square_offset(44, 1355, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_add(1355, 1355, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1366] = (s.v[1355] < 0.0);
        s.v[1366] = if s.b[1366] { 1.0 } else { 0.0 };

        if (((!s.b[1364]) && s.b[1365]) && s.b[1366]) {
            s.store_scalar(1355, 0.0);
        }

        if ((!s.b[1364]) && s.b[1365]) {
            s.store_sqrt_square_offset(44, 174, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_add(1348, 174, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1367] = (s.v[1348] < 0.0);
        s.v[1367] = if s.b[1367] { 1.0 } else { 0.0 };

        if (((!s.b[1364]) && s.b[1365]) && s.b[1367]) {
            s.store_scalar(1348, 0.0);
        }

        if ((!s.b[1364]) && s.b[1365]) {
            s.store_offset(1348, 1348, (-p.p226));
            s.store_scale(1344, 1348, 10.0);
            s.store_offset_square(1347, 1344, 1.0);
            s.store_sub_from_scalar_ad(1346, 1.0, A::div_from_scalar(1.0, s.ad_value(1347)));
            s.store_mul(1355, 1355, 1346);
            s.store_scale(1345, 1360, s.v[1359]);
            s.store_div_from_scalar_offset_input(1352, p.p219, 1345, p.p219);
            s.store_scalar(1351, p.p218);
            s.store_div_ad_rhs(1353, 1351, A::add(s.ad_value(1351), s.ad_value(173)));
            s.store_div_from_scalar_offset_input(1349, 1.0, 1355, 1e-50);
            s.store_scaled_mul(1346, 303, 1349, (-p.p214));
        }

        s.b[1368] = (s.v[1346] < (-34.0));
        s.v[1368] = if s.b[1368] { 1.0 } else { 0.0 };

        if (((!s.b[1364]) && s.b[1365]) && s.b[1368]) {
            s.store_scalar(309, 0.0);
        }

        if (((!s.b[1364]) && s.b[1365]) && (!s.b[1368])) {
            s.store_exp(1347, 1346);
            s.store_mul_scale_ad_lhs(1348, A::div_from_scalar(p.p213, s.ad_value(302)), 1.6021918e-19, 1345);
            s.store_div_from_scalar(1350, 1.0, 1363);
            s.store_sqrt_mul_ad(1351, A::add_scaled_inputs(s.ad_value(1362), 1.0, s.ad_value(1358), 1e-12), s.ad_value(1350));
            s.store_mul3_lhs(1349, 1347, 1348, 1351);
            s.store_mul3_lhs(1354, 1349, 1355, 1355);
            s.store_mul3_lhs(309, 1352, 1353, 1354);
        }

        if ((!s.b[1364]) && (!s.b[1365])) {
            s.store_scalar(309, 0.0);
        }

        if (!s.b[1364]) {
            s.store_offset_scaled(1345, 158, (-p.p221), p.p222);
            s.store_exp_scaled_input(1347, 1345, s.v[1357]);
            s.store_scale(1345, 158, (1.0 / (s.v[1357]) * 1.0 / (s.v[1357])));
            s.store_mul(1348, 158, 1345);
            s.store_scale(1349, 1360, (p.p220 / 1000000.0));
            s.store_mul3_lhs(306, 1349, 1347, 1348);
        }

    }

    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1369] = (s.v[158] >= 0.0);
        s.v[1369] = if s.b[1369] { 1.0 } else { 0.0 };

        if ((!s.b[1364]) && s.b[1369]) {
            s.store_scale(306, 306, (-1.0));
        }

        if (!s.b[1364]) {
            s.store_sub(1346, 158, 157);
            s.store_offset_scaled(1345, 1346, (-p.p221), p.p222);
            s.store_exp_scaled_input(1347, 1345, s.v[1357]);
            s.store_scale(1345, 1346, (1.0 / (s.v[1357]) * 1.0 / (s.v[1357])));
            s.store_mul(1348, 1346, 1345);
            s.store_scale(1349, 1360, (p.p220 / 1000000.0));
            s.store_mul3_lhs(307, 1349, 1347, 1348);
        }

        s.b[1370] = (s.v[1346] >= 0.0);
        s.v[1370] = if s.b[1370] { 1.0 } else { 0.0 };

        if ((!s.b[1364]) && s.b[1370]) {
            s.store_scale(307, 307, (-1.0));
        }

        if (!s.b[1364]) {
            s.store_offset_scaled_sub(1355, 513, 158, 1.0 / (s.v[1357]), ((((s.v[123]) + (p.p225))) * (1.0 / (s.v[1357]))));
            s.store_sqrt_square_offset(44, 1355, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_add(1355, 1355, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1371] = (s.v[1355] < 0.0);
        s.v[1371] = if s.b[1371] { 1.0 } else { 0.0 };

        if ((!s.b[1364]) && s.b[1371]) {
            s.store_scalar(1355, 0.0);
        }

        if (!s.b[1364]) {
            s.store_offset(1355, 1355, 1e-50);
            s.store_div_from_scalar(1346, (-p.p224), 1355);
        }

        s.b[1372] = (s.v[1346] < (-34.0));
        s.v[1372] = if s.b[1372] { 1.0 } else { 0.0 };

        if ((!s.b[1364]) && s.b[1372]) {
            s.store_scalar(308, 0.0);
        }

        if ((!s.b[1364]) && (!s.b[1372])) {
            s.store_exp(1347, 1346);
            s.store_scale(1348, 1360, (p.p223 * s.v[1359]));
            s.store_mul_ad_lhs(308, A::mul3(s.ad_value(1348), s.ad_value(1355), s.ad_value(1355)), 1347);
        }

        if (!s.b[1364]) {
            s.store_scalar(310, 0.5);
        }

        s.b[1380] = (p.p28 == 0.0);
        s.v[1380] = if s.b[1380] { 1.0 } else { 0.0 };

        if s.b[1380] {
            s.store_scalar(311, 0.0);
        }

        if (!s.b[1380]) {
            s.store_add_ad(1373, A::sub(A::scaled_offset(s.ad_value(157), p.p210, p.p209), s.ad_value(158)), A::add_scaled_inputs(s.ad_value(187), p.p211, s.ad_value(319), p.p211));
            s.store_scalar(1374, (1.0 / s.v[88]));
            s.store_mul(1375, 1373, 1374);
            s.store_sqrt_square_offset(44, 1375, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_add(304, 1375, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1381] = (s.v[304] < 0.0);
        s.v[1381] = if s.b[1381] { 1.0 } else { 0.0 };

        if ((!s.b[1380]) && s.b[1381]) {
            s.store_scalar(304, 0.0);
        }

        if (!s.b[1380]) {
            s.store_div_from_scalar_offset_input(1376, 1.0, 304, 1e-50);
            s.store_scaled_mul(1377, 303, 1376, (-p.p208));
        }

        s.b[1382] = (s.v[1377] < (-34.0));
        s.v[1382] = if s.b[1382] { 1.0 } else { 0.0 };

        if ((!s.b[1380]) && s.b[1382]) {
            s.store_scalar(311, 0.0);
        }

        if ((!s.b[1380]) && (!s.b[1382])) {
            s.store_exp(1373, 1377);
            s.store_mul_scale_ad_lhs(1374, A::div_from_scalar(p.p207, s.ad_value(302)), 1.6021918e-19, 107);
            s.store_mul_ad_lhs(311, A::mul3(s.ad_value(1374), s.ad_value(304), s.ad_value(304)), 1373);
        }

        if (!s.b[1380]) {
            s.store_sub(1379, 157, 513);
        }

        s.b[1383] = (s.v[1379] > 0.0);
        s.v[1383] = if s.b[1383] { 1.0 } else { 0.0 };

        if ((!s.b[1380]) && s.b[1383]) {
            s.store_square(1374, 1379);
            s.store_mul(331, 1374, 1379);
            s.store_offset(1377, 331, p.p212);
            s.store_div(1378, 331, 1377);
            s.store_mul(311, 311, 1378);
        }

        if ((!s.b[1380]) && (!s.b[1383])) {
            s.store_scalar(311, 0.0);
        }

        s.b[1391] = (p.p28 == 0.0);
        s.v[1391] = if s.b[1391] { 1.0 } else { 0.0 };

        if s.b[1391] {
            s.store_scalar(312, 0.0);
        }

        if (!s.b[1391]) {
            s.store_ad_value(1384, A::add_scaled_inputs3(A::add_scaled_inputs3_offset(s.ad_value(157), (-p.p209), s.ad_value(158), -1.0, s.ad_value(157), 1.0, (p.p209 * p.p210)), 1.0, s.ad_value(187), p.p211, s.ad_value(319), p.p211));
            s.store_scalar(1385, (1.0 / s.v[88]));
            s.store_mul(1386, 1384, 1385);
            s.store_sqrt_square_offset(44, 1386, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_add(305, 1386, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1392] = (s.v[305] < 0.0);
        s.v[1392] = if s.b[1392] { 1.0 } else { 0.0 };

        if ((!s.b[1391]) && s.b[1392]) {
            s.store_scalar(305, 0.0);
        }

        if (!s.b[1391]) {
            s.store_div_from_scalar_offset_input(1387, 1.0, 305, 1e-50);
            s.store_scaled_mul(1388, 303, 1387, (-p.p208));
        }

        s.b[1393] = (s.v[1388] < (-34.0));
        s.v[1393] = if s.b[1393] { 1.0 } else { 0.0 };

        if ((!s.b[1391]) && s.b[1393]) {
            s.store_scalar(312, 0.0);
        }

        if ((!s.b[1391]) && (!s.b[1393])) {
            s.store_exp(1384, 1388);
            s.store_div_from_scalar(1387, 1.0, 302);
            s.store_scaled_mul(1385, 1387, 107, (p.p207 * 1.6021918e-19));
            s.store_mul_ad_lhs(312, A::mul3(s.ad_value(1385), s.ad_value(305), s.ad_value(305)), 1384);
        }

        if (!s.b[1391]) {
            s.store_neg(1390, 513);
        }

        s.b[1394] = (s.v[1390] > 0.0);
        s.v[1394] = if s.b[1394] { 1.0 } else { 0.0 };

        if ((!s.b[1391]) && s.b[1394]) {
            s.store_square(1385, 1390);
            s.store_mul(331, 1385, 1390);
            s.store_offset(1388, 331, p.p212);
            s.store_div(1389, 331, 1388);
            s.store_mul(312, 312, 1389);
        }

        if ((!s.b[1391]) && (!s.b[1394])) {
            s.store_scalar(312, 0.0);
        }

        s.b[1395] = (p.p43 == 1.0);
        s.v[1395] = if s.b[1395] { 1.0 } else { 0.0 };

        if s.b[1395] {
            s.store_scalar(1405, s.v[91]);
            s.store_div_from_scalar(1406, 1.0, 1405);
            s.store_scalar(1462, 0.0);
            s.store_scalar(1464, 0.0);
            s.store_scalar(1466, 0.0);
            s.store_neg(1398, 534);
            s.store_mul(1399, 1398, 436);
            s.store_ad_value(331, A::add_scaled_product(s.ad_value(1399), 1.0, s.ad_value(1398), s.ad_value(437), 1.0));
            s.store_mul(470, 1399, 438);
            s.store_sub(469, 1399, 470);
            s.store_mul(468, 331, 438);
            s.store_sub(467, 331, 468);
        }

        if (s.b[1395] && (p.p24 != 0.0)) {
            s.copy_ad(521, 536);
            s.store_scalar(528, 0.0);
        }

        s.b[1475] = (1.0 == 1.0);
        s.v[1475] = if s.b[1475] { 1.0 } else { 0.0 };

        s.b[1476] = (1.0 == 2.0);
        s.v[1476] = if s.b[1476] { 1.0 } else { 0.0 };

        if ((s.b[1395] && (p.p24 != 0.0)) && s.b[1475]) {
            s.store_scale(522, 533, 0.5);
            s.store_scalar(523, p.p292);
            s.store_scalar(528, s.v[525]);
        }

        if ((s.b[1395] && (p.p24 != 0.0)) && (s.b[1476] && (!s.b[1475]))) {
            s.store_scale(522, 534, 0.5);
            s.store_scalar(523, p.p68);
            s.store_scalar(528, s.v[524]);
            s.store_scalar(528, 1.0);
        }

        s.b[1477] = (s.v[528] == 0.0);
        s.v[1477] = if s.b[1477] { 1.0 } else { 0.0 };

        if ((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) {
            s.store_mul_sqrt_ad_rhs(1425, 238, A::div(s.ad_value(521), s.ad_value(536)));
            s.store_scalar(1407, ((1.0 - -1.0) / 2.0));
            s.store_scalar(1408, ((1.0 + -1.0) / 2.0));
            s.store_ad_value(1418, A::add_scaled_products(s.ad_value(461), s.ad_value(156), 1.0, s.ad_value(462), A::sub(s.ad_value(156), s.ad_value(157)), 1.0));
            s.store_ad_value(1419, A::add_scaled_products(s.ad_value(461), s.ad_value(157), 1.0, s.ad_value(462), s.ad_value(157), -1.0));
            s.store_ad_value(1420, A::add_scaled_products(s.ad_value(461), s.ad_value(158), 1.0, s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157)), 1.0));
            s.store_ad_value(1421, A::add_scaled_products(s.ad_value(462), s.ad_value(158), 1.0, s.ad_value(461), A::sub(s.ad_value(158), s.ad_value(157)), 1.0));
            s.store_sub(1422, 1419, 1418);
            s.store_neg(1423, 1418);
            s.store_ad_value(1409, A::add_scaled_products(s.ad_value(1407), s.ad_value(461), 1.0, s.ad_value(1408), s.ad_value(462), 1.0));
            s.store_ad_value(1410, A::add_scaled_products(s.ad_value(1407), s.ad_value(462), 1.0, s.ad_value(1408), s.ad_value(461), 1.0));
            s.store_ad_value(1424, A::add_scaled_products(s.ad_value(1409), s.ad_value(1420), 1.0, s.ad_value(1410), s.ad_value(1421), 1.0));
            s.store_offset_ad(1416, A::add_scaled_products(s.ad_value(1409), s.ad_value(1423), 1.0, s.ad_value(1410), s.ad_value(1422), 1.0), (10.0 * 2.220446049250313e-16));
            s.store_neg(1396, 1416);
        }

        s.b[1478] = (s.v[1396] > s.v[141]);
        s.v[1478] = if s.b[1478] { 1.0 } else { 0.0 };

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1478]) {
            s.store_sub(1397, 1396, 141);
            s.store_sub(1398, 140, 141);
            s.store_div(44, 1397, 1398);
            s.store_square(45, 44);
            s.store_mul(46, 45, 44);
            s.store_square(47, 45);
            s.store_div_from_scalar_add_ad(1404, 1.0, A::add_scaled_inputs3_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, 1.0), s.ad_value(47));
            s.store_mul_sub_from_scalar_rhs(1404, 1398, 1.0, 1404);
            s.store_add(1401, 141, 1404);
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1478])) {
            s.copy_ad(1401, 1396);
        }

        if ((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) {
            s.store_offset_scaled(1417, 1401, -1.0, (-1e-12));
            s.store_mul(1426, 1425, 1406);
            s.store_square(1427, 1426);
            s.store_sub(1428, 1424, 523);
            s.store_div(1396, 521, 230);
            s.store_mul_ad(1429, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(s.ad_value(1396)));
            s.store_neg(1430, 1417);
        }

        s.b[1479] = (s.v[1428] < s.v[1430]);
        s.v[1479] = if s.b[1479] { 1.0 } else { 0.0 };

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1479]) {
            s.store_div_from_scalar_mul_ad(1397, 1.0, s.ad_value(225), s.ad_value(1425));
        }

    }

    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1479]) {
            s.store_mul(1404, 1397, 1405);
            s.store_offset_scaled(1431, 1404, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(1432, 1431, 1431, 8.0, 0.0, 1431);
            s.store_sub(1433, 237, 1429);
            s.store_mul_add_rhs(1403, 225, 1428, 1417);
            s.store_sub_from_scalar_ad(1434, (7.0 * 1.414213562373095), A::mul_scaled_lhs(s.ad_value(1404), 9.0, A::offset(s.ad_value(1403), (-2.0))));
            s.store_square(1435, 1434);
        }

        s.b[1480] = (s.v[1432] < (s.v[1435] * 1e-8));
        s.v[1480] = if s.b[1480] { 1.0 } else { 0.0 };

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1479]) && s.b[1480]) {
            s.store_ad_value(1437, A::add_scaled_inputs_product(A::offset(s.ad_value(1434), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1432), 0.5, s.ad_value(1434), 1.0), 1.0, s.ad_value(1404), A::offset(s.ad_value(1403), (-2.0)), 9.0));
        }

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1479]) && (!s.b[1480])) {
            s.store_sqrt_add(1436, 1432, 1435);
            s.store_ad_value(1437, A::add_scaled_offset_product_rhs(A::offset(s.ad_value(1436), ((-7.0) * 1.414213562373095)), 1.0, s.ad_value(1404), s.ad_value(1403), (-2.0), 9.0));
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1479]) {
            s.store_powf(1438, 1437, 0.3333333333333333);
            s.store_ad_value(1439, A::add_scaled_inputs_product(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1404), 12.0)), 1.0, s.ad_value(1438), 2.0, s.ad_value(1438), s.ad_value(1438), 1.414213562373095));
            s.store_div(1440, 1439, 1438);
            s.store_ad_value(1441, A::add_scaled_product(s.ad_value(1417), (-1.0), s.ad_value(1440), s.ad_value(227), 1.0));
            s.store_add(1397, 1441, 1417);
            s.store_div(1398, 1397, 1433);
            s.store_sqrt_square_offset(1399, 1398, 1.0);
            s.store_sub_ad_lhs(1442, A::div(s.ad_value(1397), s.ad_value(1399)), 1417);
            s.store_sub(1398, 1428, 1442);
            s.store_mul(459, 1405, 1398);
            s.copy_ad(458, 459);
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) {
            s.store_scalar(1440, 3.0);
            s.store_sub_ad_lhs(1443, A::div(s.ad_value(1440), s.ad_value(225)), 1417);
            s.store_exp_neg_input(1404, 1440);
            s.store_offset_div_ad(1403, A::add_scaled_inputs(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1428), s.ad_value(1417))), (-1.0)), 4.0, s.ad_value(1404), 4.0), A::mul(s.ad_value(1427), s.ad_value(226)), 1.0);
        }

        s.b[1481] = (s.v[1403] < (10.0 * 2.220446049250313e-16));
        s.v[1481] = if s.b[1481] { 1.0 } else { 0.0 };

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1481]) {
            s.store_scalar(1403, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) {
            s.store_add_ad_rhs(1443, 1428, A::mul3_scaled_output(s.ad_value(1427), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1403))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1440, 225, 1443, 1417);
            s.store_exp_neg_input(1404, 1440);
            s.store_offset_div_ad(1403, A::add_scaled_inputs(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1428), s.ad_value(1417))), (-1.0)), 4.0, s.ad_value(1404), 4.0), A::mul(s.ad_value(1427), s.ad_value(226)), 1.0);
        }

        s.b[1482] = (s.v[1403] < (10.0 * 2.220446049250313e-16));
        s.v[1482] = if s.b[1482] { 1.0 } else { 0.0 };

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1482]) {
            s.store_scalar(1403, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) {
            s.store_add_ad_rhs(1443, 1428, A::mul3_scaled_output(s.ad_value(1427), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1403))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1440, 225, 1443, 1417);
        }

        s.b[1483] = (s.v[1440] < 3.0);
        s.v[1483] = if s.b[1483] { 1.0 } else { 0.0 };

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1483]) {
            s.store_scalar(1444, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(1445, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
            s.store_offset_div_from_scalar_ad(1446, 1.0, A::mul(s.ad_value(225), s.ad_value(1426)), (1.0 / 1.414213562373095));
            s.store_ad_value(1447, A::div_scaled_inputs(A::add(s.ad_value(1428), s.ad_value(1417)), -1.0, s.ad_value(1426), 1.0));
            s.store_ad_value(1448, A::add_scaled_inputs3(A::div_scaled_product(A::square(s.ad_value(1445)), s.ad_value(1445), 1.0, A::mul3_scaled_output(s.ad_value(1444), s.ad_value(1444), s.ad_value(1444), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1445), s.ad_value(1446), 1.0, s.ad_value(1444), s.ad_value(1444), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(1447), 1.0, s.ad_value(1444), 2.0), 1.0));
            s.store_div_ad(1449, A::add_scaled_square_product(s.ad_value(1445), (-1.0), s.ad_value(1444), s.ad_value(1446), 3.0), A::mul_scaled_lhs(s.ad_value(1444), 9.0, s.ad_value(1444)));
            s.store_sqrt_ad(1400, A::add_scaled_square_product(s.ad_value(1448), 1.0, A::square(s.ad_value(1449)), s.ad_value(1449), 1.0));
            s.store_powf_ad(1450, A::sub(s.ad_value(1400), s.ad_value(1448)), 0.3333333333333333);
            s.store_neg_ad(1451, A::powf(A::add(s.ad_value(1448), s.ad_value(1400)), 0.3333333333333333));
            s.store_ad_value(1403, A::add_scaled_inputs3(s.ad_value(1450), 1.0, s.ad_value(1451), 1.0, A::div_scaled_inputs(s.ad_value(1445), 1.0, s.ad_value(1444), 3.0), -1.0));
            s.store_ad_value(1443, A::add_scaled_product(s.ad_value(1417), (-1.0), s.ad_value(1403), s.ad_value(227), 1.0));
            s.store_mul_add_rhs(1440, 225, 1443, 1417);
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) {
            s.store_offset_add(1452, 1428, 1417, 0.1);
            s.store_offset_exp_ad(1459, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1417), -1.0), 1e-50);
            s.store_div(1396, 230, 521);
            s.store_square(1453, 1396);
            s.store_mul(1454, 1453, 1459);
            s.store_mul(1396, 226, 1427);
            s.store_mul(1455, 225, 1452);
            s.store_ad_value(1456, A::add_scaled_inputs_product(A::ln(A::add_scaled_square_product(s.ad_value(1455), 1.0, s.ad_value(1454), s.ad_value(1396), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1453), s.ad_value(1396))), (-1.0), s.ad_value(225), s.ad_value(1417), 1.0));
            s.store_offset_sub(44, 1455, 1456, (-1.0));
            s.store_scale(45, 1455, 4.0);
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1397, 44, 45, 0.5, 0.5);
            s.store_scaled_sub_from_scalar_ad(1398, 1.0, A::div(A::offset(s.ad_value(44), 2.0), s.ad_value(45)), 0.5);
            s.store_ad_value(1456, A::add_scaled_inputs3(s.ad_value(1455), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5)));
            s.store_sub(1455, 1455, 1456);
            s.store_add_scaled_inputs(1455, 1455, 1.0, 225, 0.1);
            s.store_ad_value(1457, A::add_scaled_inputs_product(A::ln(A::add_scaled_square_product(s.ad_value(1455), 1.0, s.ad_value(1454), s.ad_value(1396), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1453), s.ad_value(1396))), (-1.0), s.ad_value(225), s.ad_value(1417), 1.0));
            s.copy_ad(1458, 1440);
            s.store_offset_sub(44, 1457, 1458, (-(0.0008 * 75.0)));
            s.store_scale(45, 1457, (4.0 * (0.0008 * 75.0)));
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1397, 44, 45, 0.5, 0.5);
            s.store_scaled_sub_from_scalar_ad(1398, 1.0, A::div(A::offset(s.ad_value(44), ((2.0 * 0.0008) * 75.0)), s.ad_value(45)), 0.5);
            s.store_ad_value(1440, A::add_scaled_inputs3(s.ad_value(1457), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5)));
            s.store_sub_ad_lhs(1442, A::div(s.ad_value(1440), s.ad_value(225)), 1417);
            s.store_add_ad(1397, A::offset(s.ad_value(1440), (-1.0)), A::exp_scaled_input(s.ad_value(1440), -1.0));
        }

        s.b[1484] = (s.v[1397] < (10.0 * 2.220446049250313e-16));
        s.v[1484] = if s.b[1484] { 1.0 } else { 0.0 };

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1484]) {
            s.store_scalar(1397, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) {
            s.store_sqrt(1398, 1397);
            s.store_mul(458, 1425, 1398);
            s.store_mul_sub_rhs(459, 1405, 1428, 1442);
        }

        s.b[1485] = (p.p42 == 1.0);
        s.v[1485] = if s.b[1485] { 1.0 } else { 0.0 };

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) {
            s.store_exp_ad(1459, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1417), -1.0));
            s.store_div(1396, 230, 521);
            s.store_square(1453, 1396);
            s.store_mul(1468, 1453, 1459);
            s.store_scalar(1413, 0.0);
            s.store_scalar(167, 1.0);
        }

        let mut assign26220_loop_guard: usize = 0;
        while {
            let assign26220_cond_e35796: f64 = (2.0 * 20.0);
            let assign26220_cond_e35798: f64 = (assign26220_cond_e35796 + 1.0);
            let assign26220_cond_e35800: f64 = if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (s.v[167] <= assign26220_cond_e35798)) { 1.0 } else { 0.0 };
            assign26220_cond_e35800 != 0.0
        } {
            assign26220_loop_guard += 1;
            assert!(assign26220_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) {
                s.store_scalar(1464, 0.0);
                s.store_mul_add_rhs(1440, 225, 1442, 1417);
            }
            s.b[1486] = (s.v[1440] < 5.0);
            s.v[1486] = if s.b[1486] { 1.0 } else { 0.0 };
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && s.b[1486]) {
                s.store_ad_value(1460, A::mul3(A::square(s.ad_value(1440)), s.ad_value(1440), A::offset(A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771)));
                s.store_ad_value(1461, A::mul_offset_rhs(A::square(s.ad_value(1440)), A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
                s.store_mul3_lhs(1462, 1468, 1460, 1460);
                s.store_mul_ad_lhs(1463, A::mul3_scaled_output(s.ad_value(1468), s.ad_value(225), s.ad_value(1460), 2.0), 1461);
                s.store_mul_offset_ad_rhs(1464, 1440, A::mul_offset_rhs(s.ad_value(1440), A::mul_offset_rhs(s.ad_value(1440), A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_ad(1465, A::mul_offset_rhs(s.ad_value(1440), A::mul_offset_rhs(s.ad_value(1440), A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758))), 0.707106781186548);
                s.store_sqrt_offset_ad(1466, A::add(A::square(s.ad_value(1464)), s.ad_value(1462)), 1e-50);
                s.store_ad_value(1467, A::div_scaled_inputs(A::add(A::mul3_scaled_output(s.ad_value(225), s.ad_value(1465), s.ad_value(1464), 2.0), s.ad_value(1463)), 1.0, s.ad_value(1466), 2.0));
            }
            s.b[1487] = (s.v[1440] < 80.0);
            s.v[1487] = if s.b[1487] { 1.0 } else { 0.0 };
            if ((((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1486])) && s.b[1487]) {
                s.store_exp(243, 1440);
                s.store_mul_offset_rhs(1462, 1468, 243, (-1.0));
                s.store_mul3_lhs(1463, 1468, 225, 243);
            }
            if ((((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1486])) && (!s.b[1487])) {
                s.store_exp_mul(1469, 225, 1442);
                s.store_mul_sub_rhs(1462, 1453, 1469, 1459);
                s.store_mul3_lhs(1463, 1453, 225, 1469);
            }
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1486])) {
                s.store_sqrt_add_ad(1466, A::offset(s.ad_value(1440), (-1.0)), s.ad_value(1462));
                s.store_scaled_div_ad_lhs(1467, A::add(s.ad_value(225), s.ad_value(1463)), 1466, 0.5);
            }
            if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) {
                s.store_ad_value(1470, A::add_scaled_inputs_product(s.ad_value(1428), 1.0, s.ad_value(1442), (-1.0), s.ad_value(1426), s.ad_value(1466), (-1.0)));
                s.store_sub_from_scalar_ad(1471, (-1.0), A::mul(s.ad_value(1426), s.ad_value(1467)));
            }
            s.b[1488] = (s.v[1413] == 1.0);
            s.v[1488] = if s.b[1488] { 1.0 } else { 0.0 };
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && s.b[1488]) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1488])) {
                s.store_scaled_div(494, 1470, 1471, -1.0);
            }
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1488])) {
                s.store_scaled_offset_ad(1472, {
                    if (1.0 >= ((s.v[1442]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1442))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1489] = (((s.v[494]) as f64).abs() > s.v[1472]);
            s.v[1489] = if s.b[1489] { 1.0 } else { 0.0 };
            if ((((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1488])) && s.b[1489]) {
                s.store_scale(494, 1472, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1488])) {
                s.store_add(1442, 1442, 494);
            }
            s.b[1490] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1470]) as f64).abs() <= 1e-8));
            s.v[1490] = if s.b[1490] { 1.0 } else { 0.0 };
            if ((((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1488])) && s.b[1490]) {
                s.store_scalar(1413, 1.0);
            }
            if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.b[1492] = (s.v[1440] < 5.0);
        s.v[1492] = if s.b[1492] { 1.0 } else { 0.0 };

        if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && s.b[1492]) {
            s.store_offset_square(1473, 1464, (10.0 * 2.220446049250313e-16));
            s.store_offset(1474, 1464, (10.0 * 2.220446049250313e-16));
        }

        if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1492])) {
            s.store_offset(1473, 1440, (-1.0));
            s.store_sqrt(1474, 1473);
        }

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) {
            s.store_mul(458, 1425, 1474);
            s.store_div_from_scalar_add_ad(1397, 1.0, s.ad_value(1466), s.ad_value(1474));
            s.store_mul3_lhs(460, 1425, 1462, 1397);
            s.store_add(459, 458, 460);
        }

        if ((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) {
            s.store_sub(460, 459, 458);
        }

        s.b[1494] = (1.0 == 1.0);
        s.v[1494] = if s.b[1494] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1495] = (1.0 == 2.0);
        s.v[1495] = if s.b[1495] { 1.0 } else { 0.0 };

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1494]) && (s.v[1407] != 0.0)) {
            s.store_mul_neg_lhs(463, 522, 459);
            s.store_mul_neg_lhs(465, 522, 460);
        }

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1494]) && (s.v[1408] != 0.0)) {
            s.store_mul_neg_lhs(464, 522, 459);
            s.store_mul_neg_lhs(466, 522, 460);
        }

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (s.b[1495] && (!s.b[1494]))) && (s.v[1407] != 0.0)) {
            s.store_mul_neg_lhs(467, 522, 459);
            s.store_mul_neg_lhs(469, 522, 460);
        }

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (s.b[1495] && (!s.b[1494]))) && (s.v[1408] != 0.0)) {
            s.store_mul_neg_lhs(468, 522, 459);
            s.store_mul_neg_lhs(470, 522, 460);
        }

        if ((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) {
            s.store_scalar(1407, ((1.0 - 1.0) / 2.0));
            s.store_scalar(1408, ((1.0 + 1.0) / 2.0));
            s.store_ad_value(1418, A::add_scaled_products(s.ad_value(461), s.ad_value(156), 1.0, s.ad_value(462), A::sub(s.ad_value(156), s.ad_value(157)), 1.0));
            s.store_ad_value(1419, A::add_scaled_products(s.ad_value(461), s.ad_value(157), 1.0, s.ad_value(462), s.ad_value(157), -1.0));
            s.store_ad_value(1420, A::add_scaled_products(s.ad_value(461), s.ad_value(158), 1.0, s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157)), 1.0));
            s.store_ad_value(1421, A::add_scaled_products(s.ad_value(462), s.ad_value(158), 1.0, s.ad_value(461), A::sub(s.ad_value(158), s.ad_value(157)), 1.0));
            s.store_sub(1422, 1419, 1418);
            s.store_neg(1423, 1418);
            s.store_ad_value(1409, A::add_scaled_products(s.ad_value(1407), s.ad_value(461), 1.0, s.ad_value(1408), s.ad_value(462), 1.0));
            s.store_ad_value(1410, A::add_scaled_products(s.ad_value(1407), s.ad_value(462), 1.0, s.ad_value(1408), s.ad_value(461), 1.0));
            s.store_ad_value(1424, A::add_scaled_products(s.ad_value(1409), s.ad_value(1420), 1.0, s.ad_value(1410), s.ad_value(1421), 1.0));
            s.store_offset_ad(1416, A::add_scaled_products(s.ad_value(1409), s.ad_value(1423), 1.0, s.ad_value(1410), s.ad_value(1422), 1.0), (10.0 * 2.220446049250313e-16));
            s.store_neg(1396, 1416);
        }

        s.b[1496] = (s.v[1396] > s.v[141]);
        s.v[1496] = if s.b[1496] { 1.0 } else { 0.0 };

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1496]) {
            s.store_sub(1397, 1396, 141);
            s.store_sub(1398, 140, 141);
            s.store_div(44, 1397, 1398);
            s.store_square(45, 44);
            s.store_mul(46, 45, 44);
            s.store_square(47, 45);
            s.store_div_from_scalar_add_ad(1404, 1.0, A::add_scaled_inputs3_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, 1.0), s.ad_value(47));
            s.store_mul_sub_from_scalar_rhs(1404, 1398, 1.0, 1404);
            s.store_add(1401, 141, 1404);
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1496])) {
            s.copy_ad(1401, 1396);
        }

        if ((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) {
            s.store_offset_scaled(1417, 1401, -1.0, (-1e-12));
            s.store_mul(1426, 1425, 1406);
            s.store_square(1427, 1426);
            s.store_sub(1428, 1424, 523);
            s.store_div(1396, 521, 230);
            s.store_mul_ad(1429, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(s.ad_value(1396)));
            s.store_neg(1430, 1417);
        }

        s.b[1497] = (s.v[1428] < s.v[1430]);
        s.v[1497] = if s.b[1497] { 1.0 } else { 0.0 };

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1497]) {
            s.store_div_from_scalar_mul_ad(1397, 1.0, s.ad_value(225), s.ad_value(1425));
            s.store_mul(1404, 1397, 1405);
            s.store_offset_scaled(1431, 1404, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(1432, 1431, 1431, 8.0, 0.0, 1431);
            s.store_sub(1433, 237, 1429);
            s.store_mul_add_rhs(1403, 225, 1428, 1417);
            s.store_sub_from_scalar_ad(1434, (7.0 * 1.414213562373095), A::mul_scaled_lhs(s.ad_value(1404), 9.0, A::offset(s.ad_value(1403), (-2.0))));
            s.store_square(1435, 1434);
        }

        s.b[1498] = (s.v[1432] < (s.v[1435] * 1e-8));
        s.v[1498] = if s.b[1498] { 1.0 } else { 0.0 };

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1497]) && s.b[1498]) {
            s.store_ad_value(1437, A::add_scaled_inputs_product(A::offset(s.ad_value(1434), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1432), 0.5, s.ad_value(1434), 1.0), 1.0, s.ad_value(1404), A::offset(s.ad_value(1403), (-2.0)), 9.0));
        }

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1497]) && (!s.b[1498])) {
            s.store_sqrt_add(1436, 1432, 1435);
            s.store_ad_value(1437, A::add_scaled_offset_product_rhs(A::offset(s.ad_value(1436), ((-7.0) * 1.414213562373095)), 1.0, s.ad_value(1404), s.ad_value(1403), (-2.0), 9.0));
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1497]) {
            s.store_powf(1438, 1437, 0.3333333333333333);
            s.store_ad_value(1439, A::add_scaled_inputs_product(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1404), 12.0)), 1.0, s.ad_value(1438), 2.0, s.ad_value(1438), s.ad_value(1438), 1.414213562373095));
            s.store_div(1440, 1439, 1438);
            s.store_ad_value(1441, A::add_scaled_product(s.ad_value(1417), (-1.0), s.ad_value(1440), s.ad_value(227), 1.0));
            s.store_add(1397, 1441, 1417);
            s.store_div(1398, 1397, 1433);
            s.store_sqrt_square_offset(1399, 1398, 1.0);
            s.store_sub_ad_lhs(1442, A::div(s.ad_value(1397), s.ad_value(1399)), 1417);
            s.store_sub(1398, 1428, 1442);
            s.store_mul(459, 1405, 1398);
            s.copy_ad(458, 459);
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) {
            s.store_scalar(1440, 3.0);
            s.store_sub_ad_lhs(1443, A::div(s.ad_value(1440), s.ad_value(225)), 1417);
            s.store_exp_neg_input(1404, 1440);
            s.store_offset_div_ad(1403, A::add_scaled_inputs(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1428), s.ad_value(1417))), (-1.0)), 4.0, s.ad_value(1404), 4.0), A::mul(s.ad_value(1427), s.ad_value(226)), 1.0);
        }

        s.b[1499] = (s.v[1403] < (10.0 * 2.220446049250313e-16));
        s.v[1499] = if s.b[1499] { 1.0 } else { 0.0 };

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1499]) {
            s.store_scalar(1403, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) {
            s.store_add_ad_rhs(1443, 1428, A::mul3_scaled_output(s.ad_value(1427), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1403))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1440, 225, 1443, 1417);
            s.store_exp_neg_input(1404, 1440);
            s.store_offset_div_ad(1403, A::add_scaled_inputs(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1428), s.ad_value(1417))), (-1.0)), 4.0, s.ad_value(1404), 4.0), A::mul(s.ad_value(1427), s.ad_value(226)), 1.0);
        }

        s.b[1500] = (s.v[1403] < (10.0 * 2.220446049250313e-16));
        s.v[1500] = if s.b[1500] { 1.0 } else { 0.0 };

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1500]) {
            s.store_scalar(1403, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) {
            s.store_add_ad_rhs(1443, 1428, A::mul3_scaled_output(s.ad_value(1427), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1403))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1440, 225, 1443, 1417);
        }

        s.b[1501] = (s.v[1440] < 3.0);
        s.v[1501] = if s.b[1501] { 1.0 } else { 0.0 };

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1501]) {
            s.store_scalar(1444, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(1445, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
            s.store_offset_div_from_scalar_ad(1446, 1.0, A::mul(s.ad_value(225), s.ad_value(1426)), (1.0 / 1.414213562373095));
            s.store_ad_value(1447, A::div_scaled_inputs(A::add(s.ad_value(1428), s.ad_value(1417)), -1.0, s.ad_value(1426), 1.0));
            s.store_ad_value(1448, A::add_scaled_inputs3(A::div_scaled_product(A::square(s.ad_value(1445)), s.ad_value(1445), 1.0, A::mul3_scaled_output(s.ad_value(1444), s.ad_value(1444), s.ad_value(1444), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1445), s.ad_value(1446), 1.0, s.ad_value(1444), s.ad_value(1444), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(1447), 1.0, s.ad_value(1444), 2.0), 1.0));
            s.store_div_ad(1449, A::add_scaled_square_product(s.ad_value(1445), (-1.0), s.ad_value(1444), s.ad_value(1446), 3.0), A::mul_scaled_lhs(s.ad_value(1444), 9.0, s.ad_value(1444)));
            s.store_sqrt_ad(1400, A::add_scaled_square_product(s.ad_value(1448), 1.0, A::square(s.ad_value(1449)), s.ad_value(1449), 1.0));
            s.store_powf_ad(1450, A::sub(s.ad_value(1400), s.ad_value(1448)), 0.3333333333333333);
            s.store_neg_ad(1451, A::powf(A::add(s.ad_value(1448), s.ad_value(1400)), 0.3333333333333333));
            s.store_ad_value(1403, A::add_scaled_inputs3(s.ad_value(1450), 1.0, s.ad_value(1451), 1.0, A::div_scaled_inputs(s.ad_value(1445), 1.0, s.ad_value(1444), 3.0), -1.0));
            s.store_ad_value(1443, A::add_scaled_product(s.ad_value(1417), (-1.0), s.ad_value(1403), s.ad_value(227), 1.0));
            s.store_mul_add_rhs(1440, 225, 1443, 1417);
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) {
            s.store_offset_add(1452, 1428, 1417, 0.1);
            s.store_offset_exp_ad(1459, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1417), -1.0), 1e-50);
            s.store_div(1396, 230, 521);
            s.store_square(1453, 1396);
            s.store_mul(1454, 1453, 1459);
            s.store_mul(1396, 226, 1427);
            s.store_mul(1455, 225, 1452);
            s.store_ad_value(1456, A::add_scaled_inputs_product(A::ln(A::add_scaled_square_product(s.ad_value(1455), 1.0, s.ad_value(1454), s.ad_value(1396), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1453), s.ad_value(1396))), (-1.0), s.ad_value(225), s.ad_value(1417), 1.0));
            s.store_offset_sub(44, 1455, 1456, (-1.0));
            s.store_scale(45, 1455, 4.0);
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1397, 44, 45, 0.5, 0.5);
            s.store_scaled_sub_from_scalar_ad(1398, 1.0, A::div(A::offset(s.ad_value(44), 2.0), s.ad_value(45)), 0.5);
            s.store_ad_value(1456, A::add_scaled_inputs3(s.ad_value(1455), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5)));
            s.store_sub(1455, 1455, 1456);
            s.store_add_scaled_inputs(1455, 1455, 1.0, 225, 0.1);
            s.store_ad_value(1457, A::add_scaled_inputs_product(A::ln(A::add_scaled_square_product(s.ad_value(1455), 1.0, s.ad_value(1454), s.ad_value(1396), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1453), s.ad_value(1396))), (-1.0), s.ad_value(225), s.ad_value(1417), 1.0));
            s.copy_ad(1458, 1440);
            s.store_offset_sub(44, 1457, 1458, (-(0.0008 * 75.0)));
            s.store_scale(45, 1457, (4.0 * (0.0008 * 75.0)));
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1397, 44, 45, 0.5, 0.5);
            s.store_scaled_sub_from_scalar_ad(1398, 1.0, A::div(A::offset(s.ad_value(44), ((2.0 * 0.0008) * 75.0)), s.ad_value(45)), 0.5);
            s.store_ad_value(1440, A::add_scaled_inputs3(s.ad_value(1457), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5)));
            s.store_sub_ad_lhs(1442, A::div(s.ad_value(1440), s.ad_value(225)), 1417);
            s.store_add_ad(1397, A::offset(s.ad_value(1440), (-1.0)), A::exp_scaled_input(s.ad_value(1440), -1.0));
        }

        s.b[1502] = (s.v[1397] < (10.0 * 2.220446049250313e-16));
        s.v[1502] = if s.b[1502] { 1.0 } else { 0.0 };

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1502]) {
            s.store_scalar(1397, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) {
            s.store_sqrt(1398, 1397);
            s.store_mul(458, 1425, 1398);
            s.store_mul_sub_rhs(459, 1405, 1428, 1442);
        }

        s.b[1503] = (p.p42 == 1.0);
        s.v[1503] = if s.b[1503] { 1.0 } else { 0.0 };

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {
            s.store_exp_ad(1459, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1417), -1.0));
            s.store_div(1396, 230, 521);
        }

    }

    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {
            s.store_square(1453, 1396);
            s.store_mul(1468, 1453, 1459);
            s.store_scalar(1413, 0.0);
            s.store_scalar(167, 1.0);
        }

        let mut assign27770_loop_guard: usize = 0;
        while {
            let assign27770_cond_e38739: f64 = (2.0 * 20.0);
            let assign27770_cond_e38741: f64 = (assign27770_cond_e38739 + 1.0);
            let assign27770_cond_e38743: f64 = if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (s.v[167] <= assign27770_cond_e38741)) { 1.0 } else { 0.0 };
            assign27770_cond_e38743 != 0.0
        } {
            assign27770_loop_guard += 1;
            assert!(assign27770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {
                s.store_scalar(1464, 0.0);
                s.store_mul_add_rhs(1440, 225, 1442, 1417);
            }
            s.b[1504] = (s.v[1440] < 5.0);
            s.v[1504] = if s.b[1504] { 1.0 } else { 0.0 };
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && s.b[1504]) {
                s.store_ad_value(1460, A::mul3(A::square(s.ad_value(1440)), s.ad_value(1440), A::offset(A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771)));
                s.store_ad_value(1461, A::mul_offset_rhs(A::square(s.ad_value(1440)), A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
                s.store_mul3_lhs(1462, 1468, 1460, 1460);
                s.store_mul_ad_lhs(1463, A::mul3_scaled_output(s.ad_value(1468), s.ad_value(225), s.ad_value(1460), 2.0), 1461);
                s.store_mul_offset_ad_rhs(1464, 1440, A::mul_offset_rhs(s.ad_value(1440), A::mul_offset_rhs(s.ad_value(1440), A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_ad(1465, A::mul_offset_rhs(s.ad_value(1440), A::mul_offset_rhs(s.ad_value(1440), A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758))), 0.707106781186548);
                s.store_sqrt_offset_ad(1466, A::add(A::square(s.ad_value(1464)), s.ad_value(1462)), 1e-50);
                s.store_ad_value(1467, A::div_scaled_inputs(A::add(A::mul3_scaled_output(s.ad_value(225), s.ad_value(1465), s.ad_value(1464), 2.0), s.ad_value(1463)), 1.0, s.ad_value(1466), 2.0));
            }
            s.b[1505] = (s.v[1440] < 80.0);
            s.v[1505] = if s.b[1505] { 1.0 } else { 0.0 };
            if ((((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1504])) && s.b[1505]) {
                s.store_exp(243, 1440);
                s.store_mul_offset_rhs(1462, 1468, 243, (-1.0));
                s.store_mul3_lhs(1463, 1468, 225, 243);
            }
            if ((((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1504])) && (!s.b[1505])) {
                s.store_exp_mul(1469, 225, 1442);
                s.store_mul_sub_rhs(1462, 1453, 1469, 1459);
                s.store_mul3_lhs(1463, 1453, 225, 1469);
            }
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1504])) {
                s.store_sqrt_add_ad(1466, A::offset(s.ad_value(1440), (-1.0)), s.ad_value(1462));
                s.store_scaled_div_ad_lhs(1467, A::add(s.ad_value(225), s.ad_value(1463)), 1466, 0.5);
            }
            if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {
                s.store_ad_value(1470, A::add_scaled_inputs_product(s.ad_value(1428), 1.0, s.ad_value(1442), (-1.0), s.ad_value(1426), s.ad_value(1466), (-1.0)));
                s.store_sub_from_scalar_ad(1471, (-1.0), A::mul(s.ad_value(1426), s.ad_value(1467)));
            }
            s.b[1506] = (s.v[1413] == 1.0);
            s.v[1506] = if s.b[1506] { 1.0 } else { 0.0 };
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && s.b[1506]) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1506])) {
                s.store_scaled_div(494, 1470, 1471, -1.0);
            }
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1506])) {
                s.store_scaled_offset_ad(1472, {
                    if (1.0 >= ((s.v[1442]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1442))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1507] = (((s.v[494]) as f64).abs() > s.v[1472]);
            s.v[1507] = if s.b[1507] { 1.0 } else { 0.0 };
            if ((((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1506])) && s.b[1507]) {
                s.store_scale(494, 1472, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1506])) {
                s.store_add(1442, 1442, 494);
            }
            s.b[1508] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1470]) as f64).abs() <= 1e-8));
            s.v[1508] = if s.b[1508] { 1.0 } else { 0.0 };
            if ((((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1506])) && s.b[1508]) {
                s.store_scalar(1413, 1.0);
            }
            if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.b[1510] = (s.v[1440] < 5.0);
        s.v[1510] = if s.b[1510] { 1.0 } else { 0.0 };

        if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && s.b[1510]) {
            s.store_offset_square(1473, 1464, (10.0 * 2.220446049250313e-16));
            s.store_offset(1474, 1464, (10.0 * 2.220446049250313e-16));
        }

        if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1510])) {
            s.store_offset(1473, 1440, (-1.0));
            s.store_sqrt(1474, 1473);
        }

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {
            s.store_mul(458, 1425, 1474);
            s.store_div_from_scalar_add_ad(1397, 1.0, s.ad_value(1466), s.ad_value(1474));
            s.store_mul3_lhs(460, 1425, 1462, 1397);
            s.store_add(459, 458, 460);
        }

        if ((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) {
            s.store_sub(460, 459, 458);
        }

        s.b[1512] = (1.0 == 1.0);
        s.v[1512] = if s.b[1512] { 1.0 } else { 0.0 };

        s.b[1513] = (1.0 == 2.0);
        s.v[1513] = if s.b[1513] { 1.0 } else { 0.0 };

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1512]) && (s.v[1407] != 0.0)) {
            s.store_mul_neg_lhs(463, 522, 459);
            s.store_mul_neg_lhs(465, 522, 460);
        }

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1512]) && (s.v[1408] != 0.0)) {
            s.store_mul_neg_lhs(464, 522, 459);
            s.store_mul_neg_lhs(466, 522, 460);
        }

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (s.b[1513] && (!s.b[1512]))) && (s.v[1407] != 0.0)) {
            s.store_mul_neg_lhs(467, 522, 459);
            s.store_mul_neg_lhs(469, 522, 460);
        }

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (s.b[1513] && (!s.b[1512]))) && (s.v[1408] != 0.0)) {
            s.store_mul_neg_lhs(468, 522, 459);
            s.store_mul_neg_lhs(470, 522, 460);
        }

        s.v[317] = p.p189;

        s.b[1516] = (s.v[145] != 0.0);
        s.v[1516] = if s.b[1516] { 1.0 } else { 0.0 };

        if s.b[1516] {
            s.store_add(1515, 157, 161);
            s.store_add_scaled_inputs(314, 1515, s.v[317], 162, (1.0 - s.v[317]));
        }

        s.b[1517] = (p.p64 != 0.0);
        s.v[1517] = if s.b[1517] { 1.0 } else { 0.0 };

        if (s.b[1516] && s.b[1517]) {
            s.store_scalar(315, 0.0);
        }

        s.b[1518] = (s.v[314] > ((s.v[161] + s.v[157]) - (10.0 * 2.220446049250313e-16)));
        s.v[1518] = if s.b[1518] { 1.0 } else { 0.0 };

        if (s.b[1516] && s.b[1518]) {
            s.store_offset_add(314, 161, 157, (-(10.0 * 2.220446049250313e-16)));
        }

        s.b[1519] = (p.p64 != 0.0);
        s.v[1519] = if s.b[1519] { 1.0 } else { 0.0 };

        s.b[1520] = (s.v[246] < 1e-15);
        s.v[1520] = if s.b[1520] { 1.0 } else { 0.0 };

        if (((!s.b[1516]) && s.b[1519]) && s.b[1520]) {
            s.store_scalar(315, 0.0);
        }

        if (((!s.b[1516]) && s.b[1519]) && (!s.b[1520])) {
            s.store_scale(1514, 227, 1.0 / (s.v[97]));
            s.store_div_from_scalar(1515, 1.0, 244);
            s.store_mul3_lhs(315, 246, 1514, 1515);
        }

        s.v[1532] = s.v[91];

        s.v[1533] = (1.0 / s.v[1532]);

        s.v[1553] = 0.0;

        s.v[1593] = 0.0;

        s.v[1591] = 0.0;

        s.v[1595] = 0.0;

        s.b[1604] = ((p.p29 >= 1.0) && (p.p188 > 0.0));
        s.v[1604] = if s.b[1604] { 1.0 } else { 0.0 };

        if ((p.p24 != 0.0) && s.b[1604]) {
            s.store_scalar(1535, p.p171);
            s.store_scalar(1536, p.p172);
            s.copy_ad(1537, 158);
            s.store_scalar(1534, p.p188);
        }

        s.b[1605] = ((s.v[69] == 0.0) && (p.p188 > 0.0));
        s.v[1605] = if s.b[1605] { 1.0 } else { 0.0 };

        if (((p.p24 != 0.0) && s.b[1604]) && s.b[1605]) {
            s.store_ad_value(1522, {
                if (p.p43 == 1.0) {
                    A::scale(s.ad_value(287), s.v[1532])
                } else {
                    A::scale(s.ad_value(108), s.v[1532])
                }
            });
        }

        if (((p.p24 != 0.0) && s.b[1604]) && s.b[1605]) {
            s.store_mul_ad_product_rhs(1525, 1535, s.ad_value(1522), A::add(s.ad_value(1536), s.ad_value(1537)));
            s.store_mul(1526, 1534, 1522);
            s.copy_ad(1530, 161);
            s.store_sub_from_scalar(1527, 1.2, 1530);
            s.store_ad_value(267, A::add_scaled_products(s.ad_value(158), s.ad_value(1526), 1.0, s.ad_value(1527), s.ad_value(1525), (-1.0)));
            s.store_mul_ad_product_rhs(1525, 1535, s.ad_value(1522), A::add_scaled_inputs3(s.ad_value(1536), 1.0, s.ad_value(1537), 1.0, s.ad_value(157), -1.0));
            s.store_sub(1530, 162, 157);
            s.store_sub_from_scalar(1527, 1.2, 1530);
            s.store_ad_value(268, A::add_scaled_products(A::sub(s.ad_value(158), s.ad_value(157)), s.ad_value(1526), 1.0, s.ad_value(1525), s.ad_value(1527), (-1.0)));
        }

        if (((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) {
            s.store_mul_sqrt_ad_rhs(1554, 238, A::div_from_scalar(s.v[69], s.ad_value(536)));
            s.store_scalar(1538, ((1.0 - -1.0) / 2.0));
            s.store_scalar(1539, ((1.0 + -1.0) / 2.0));
        }

        s.b[1606] = (p.p43 == 1.0);
        s.v[1606] = if s.b[1606] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1606]) {
            s.store_ad_value(1548, A::add_scaled_products(s.ad_value(461), s.ad_value(156), 1.0, s.ad_value(462), A::sub(s.ad_value(156), s.ad_value(157)), 1.0));
            s.store_ad_value(1549, A::add_scaled_products(s.ad_value(461), s.ad_value(157), 1.0, s.ad_value(462), s.ad_value(157), -1.0));
            s.store_ad_value(1550, A::add_scaled_products(s.ad_value(461), s.ad_value(158), 1.0, s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157)), 1.0));
            s.store_sub(1551, 1549, 1548);
            s.store_sub(1553, 1550, 1548);
            s.store_neg(1552, 1548);
            s.store_ad_value(1540, A::add_scaled_products(s.ad_value(1538), s.ad_value(461), 1.0, s.ad_value(1539), s.ad_value(462), 1.0));
            s.store_ad_value(1541, A::add_scaled_products(s.ad_value(1538), s.ad_value(462), 1.0, s.ad_value(1539), s.ad_value(461), 1.0));
            s.store_offset_ad(1546, A::add_scaled_products(s.ad_value(1540), s.ad_value(1552), 1.0, s.ad_value(1541), s.ad_value(1551), 1.0), (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1606])) {
            s.store_ad_value(1540, A::add_scaled_products(s.ad_value(1538), s.ad_value(461), 1.0, s.ad_value(1539), s.ad_value(462), 1.0));
            s.store_ad_value(1541, A::add_scaled_products(s.ad_value(1538), s.ad_value(462), 1.0, s.ad_value(1539), s.ad_value(461), 1.0));
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1606])) && (s.v[1538] != 0.0)) {
            s.store_ad_value(1553, A::add_scaled_products(s.ad_value(461), s.ad_value(158), 1.0, s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157)), 1.0));
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1606])) && (s.v[1539] != 0.0)) {
            s.store_ad_value(1553, A::add_scaled_products(s.ad_value(462), s.ad_value(158), 1.0, s.ad_value(461), A::sub(s.ad_value(158), s.ad_value(157)), 1.0));
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1606])) {
            s.store_scalar(1546, 0.0);
        }

        if (((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) {
            s.store_neg(1521, 1546);
        }

        s.b[1607] = (s.v[1521] > s.v[141]);
        s.v[1607] = if s.b[1607] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1607]) {
            s.store_sub(1522, 1521, 141);
            s.store_sub(1523, 140, 141);
            s.store_div(44, 1522, 1523);
            s.store_square(45, 44);
            s.store_mul(46, 45, 44);
            s.store_square(47, 45);
            s.store_div_from_scalar_add_ad(1531, 1.0, A::add_scaled_inputs3_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, 1.0), s.ad_value(47));
            s.store_mul_sub_from_scalar_rhs(1531, 1523, 1.0, 1531);
            s.store_add(1528, 141, 1531);
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1607])) {
            s.copy_ad(1528, 1521);
        }

        if (((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) {
            s.store_offset_scaled(1547, 1528, -1.0, (-1e-12));
            s.store_scale(1555, 1554, s.v[1533]);
            s.store_square(1556, 1555);
            s.store_sub_from_scalar(1557, s.v[82], 1553);
            s.store_div_from_scalar(1521, s.v[69], 230);
            s.store_mul_ad(1558, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(s.ad_value(1521)));
            s.store_neg(1559, 1547);
        }

        s.b[1608] = (s.v[1557] < s.v[1559]);
        s.v[1608] = if s.b[1608] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1608]) {
            s.store_div_from_scalar_mul_ad(1522, 1.0, s.ad_value(225), s.ad_value(1554));
            s.store_scale(1531, 1522, s.v[1532]);
            s.store_offset_scaled(1560, 1531, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(1561, 1560, 1560, 8.0, 0.0, 1560);
            s.store_sub(1562, 237, 1558);
            s.store_mul_add_rhs(1530, 225, 1557, 1547);
            s.store_sub_from_scalar_ad(1563, (7.0 * 1.414213562373095), A::mul_scaled_lhs(s.ad_value(1531), 9.0, A::offset(s.ad_value(1530), (-2.0))));
            s.store_square(1564, 1563);
        }

    }

    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1609] = (s.v[1561] < (s.v[1564] * 1e-8));
        s.v[1609] = if s.b[1609] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1608]) && s.b[1609]) {
            s.store_ad_value(1566, A::add_scaled_inputs_product(A::offset(s.ad_value(1563), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1561), 0.5, s.ad_value(1563), 1.0), 1.0, s.ad_value(1531), A::offset(s.ad_value(1530), (-2.0)), 9.0));
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1608]) && (!s.b[1609])) {
            s.store_sqrt_add(1565, 1561, 1564);
            s.store_ad_value(1566, A::add_scaled_offset_product_rhs(A::offset(s.ad_value(1565), ((-7.0) * 1.414213562373095)), 1.0, s.ad_value(1531), s.ad_value(1530), (-2.0), 9.0));
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1608]) {
            s.store_powf(1567, 1566, 0.3333333333333333);
            s.store_ad_value(1568, A::add_scaled_inputs_product(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1531), 12.0)), 1.0, s.ad_value(1567), 2.0, s.ad_value(1567), s.ad_value(1567), 1.414213562373095));
            s.store_div(1569, 1568, 1567);
            s.store_ad_value(1570, A::add_scaled_product(s.ad_value(1547), (-1.0), s.ad_value(1569), s.ad_value(227), 1.0));
            s.store_add(1522, 1570, 1547);
            s.store_div(1523, 1522, 1562);
            s.store_sqrt_square_offset(1524, 1523, 1.0);
            s.store_sub_ad_lhs(1571, A::div(s.ad_value(1522), s.ad_value(1524)), 1547);
            s.store_sub(1523, 1557, 1571);
            s.store_scale(459, 1523, s.v[1532]);
            s.copy_ad(458, 459);
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) {
            s.store_scalar(1569, 3.0);
            s.store_sub_ad_lhs(1572, A::div(s.ad_value(1569), s.ad_value(225)), 1547);
            s.store_exp_neg_input(1531, 1569);
            s.store_offset_div_ad(1530, A::add_scaled_inputs(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1557), s.ad_value(1547))), (-1.0)), 4.0, s.ad_value(1531), 4.0), A::mul(s.ad_value(1556), s.ad_value(226)), 1.0);
        }

        s.b[1610] = (s.v[1530] < (10.0 * 2.220446049250313e-16));
        s.v[1610] = if s.b[1610] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1610]) {
            s.store_scalar(1530, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) {
            s.store_add_ad_rhs(1572, 1557, A::mul3_scaled_output(s.ad_value(1556), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1530))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1569, 225, 1572, 1547);
            s.store_exp_neg_input(1531, 1569);
            s.store_offset_div_ad(1530, A::add_scaled_inputs(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1557), s.ad_value(1547))), (-1.0)), 4.0, s.ad_value(1531), 4.0), A::mul(s.ad_value(1556), s.ad_value(226)), 1.0);
        }

        s.b[1611] = (s.v[1530] < (10.0 * 2.220446049250313e-16));
        s.v[1611] = if s.b[1611] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1611]) {
            s.store_scalar(1530, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) {
            s.store_add_ad_rhs(1572, 1557, A::mul3_scaled_output(s.ad_value(1556), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1530))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1569, 225, 1572, 1547);
        }

        s.b[1612] = (s.v[1569] < 3.0);
        s.v[1612] = if s.b[1612] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1612]) {
            s.store_scalar(1573, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(1574, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
            s.store_offset_div_from_scalar_ad(1575, 1.0, A::mul(s.ad_value(225), s.ad_value(1555)), (1.0 / 1.414213562373095));
            s.store_ad_value(1576, A::div_scaled_inputs(A::add(s.ad_value(1557), s.ad_value(1547)), -1.0, s.ad_value(1555), 1.0));
            s.store_ad_value(1577, A::add_scaled_inputs3(A::div_scaled_product(A::square(s.ad_value(1574)), s.ad_value(1574), 1.0, A::mul3_scaled_output(s.ad_value(1573), s.ad_value(1573), s.ad_value(1573), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1574), s.ad_value(1575), 1.0, s.ad_value(1573), s.ad_value(1573), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(1576), 1.0, s.ad_value(1573), 2.0), 1.0));
            s.store_div_ad(1578, A::add_scaled_square_product(s.ad_value(1574), (-1.0), s.ad_value(1573), s.ad_value(1575), 3.0), A::mul_scaled_lhs(s.ad_value(1573), 9.0, s.ad_value(1573)));
            s.store_sqrt_ad(1526, A::add_scaled_square_product(s.ad_value(1577), 1.0, A::square(s.ad_value(1578)), s.ad_value(1578), 1.0));
            s.store_powf_ad(1579, A::sub(s.ad_value(1526), s.ad_value(1577)), 0.3333333333333333);
            s.store_neg_ad(1580, A::powf(A::add(s.ad_value(1577), s.ad_value(1526)), 0.3333333333333333));
            s.store_ad_value(1530, A::add_scaled_inputs3(s.ad_value(1579), 1.0, s.ad_value(1580), 1.0, A::div_scaled_inputs(s.ad_value(1574), 1.0, s.ad_value(1573), 3.0), -1.0));
            s.store_ad_value(1572, A::add_scaled_product(s.ad_value(1547), (-1.0), s.ad_value(1530), s.ad_value(227), 1.0));
            s.store_mul_add_rhs(1569, 225, 1572, 1547);
        }

        s.b[1613] = (p.p41 > 0.0);
        s.v[1613] = if s.b[1613] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1613]) {
            s.store_offset_add(1581, 1557, 1547, 0.1);
            s.store_offset_exp_ad(1588, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1547), -1.0), 1e-50);
            s.store_scale(1521, 230, 1.0 / (s.v[69]));
            s.store_square(1582, 1521);
            s.store_mul(1583, 1582, 1588);
            s.store_mul(1521, 226, 1556);
            s.store_mul(1584, 225, 1581);
            s.store_ad_value(1585, A::add_scaled_inputs_product(A::ln(A::add_scaled_square_product(s.ad_value(1584), 1.0, s.ad_value(1583), s.ad_value(1521), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1582), s.ad_value(1521))), (-1.0), s.ad_value(225), s.ad_value(1547), 1.0));
            s.store_offset_sub(44, 1584, 1585, (-1.0));
            s.store_scale(45, 1584, 4.0);
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1613]) {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1613]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1522, 44, 45, 0.5, 0.5);
            s.store_scaled_sub_from_scalar_ad(1523, 1.0, A::div(A::offset(s.ad_value(44), 2.0), s.ad_value(45)), 0.5);
            s.store_ad_value(1585, A::add_scaled_inputs3(s.ad_value(1584), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5)));
            s.store_sub(1584, 1584, 1585);
            s.store_add_scaled_inputs(1584, 1584, 1.0, 225, 0.1);
            s.store_ad_value(1586, A::add_scaled_inputs_product(A::ln(A::add_scaled_square_product(s.ad_value(1584), 1.0, s.ad_value(1583), s.ad_value(1521), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1582), s.ad_value(1521))), (-1.0), s.ad_value(225), s.ad_value(1547), 1.0));
            s.copy_ad(1587, 1569);
            s.store_offset_sub(44, 1586, 1587, (-(0.0008 * 75.0)));
            s.store_scale(45, 1586, (4.0 * (0.0008 * 75.0)));
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1613]) {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1613]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1522, 44, 45, 0.5, 0.5);
            s.store_scaled_sub_from_scalar_ad(1523, 1.0, A::div(A::offset(s.ad_value(44), ((2.0 * 0.0008) * 75.0)), s.ad_value(45)), 0.5);
            s.store_ad_value(1569, A::add_scaled_inputs3(s.ad_value(1586), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5)));
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) {
            s.store_sub_ad_lhs(1571, A::div(s.ad_value(1569), s.ad_value(225)), 1547);
            s.store_add_ad(1522, A::offset(s.ad_value(1569), (-1.0)), A::exp_scaled_input(s.ad_value(1569), -1.0));
        }

        s.b[1614] = (s.v[1522] < (10.0 * 2.220446049250313e-16));
        s.v[1614] = if s.b[1614] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1614]) {
            s.store_scalar(1522, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) {
            s.store_sqrt(1523, 1522);
            s.store_mul(458, 1554, 1523);
            s.store_scaled_sub(459, 1557, 1571, s.v[1532]);
        }

        s.b[1615] = (p.p41 == 1.0);
        s.v[1615] = if s.b[1615] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) {
            s.store_exp_ad(1588, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1547), -1.0));
            s.store_scale(1521, 230, 1.0 / (s.v[69]));
            s.store_square(1582, 1521);
            s.store_mul(1597, 1582, 1588);
            s.store_scalar(1544, 0.0);
            s.store_scalar(1591, 0.0);
            s.store_scalar(1595, 0.0);
            s.store_scalar(167, 1.0);
        }

        let mut assign29770_loop_guard: usize = 0;
        while {
            let assign29770_cond_e42272: f64 = (2.0 * 20.0);
            let assign29770_cond_e42274: f64 = (assign29770_cond_e42272 + 1.0);
            let assign29770_cond_e42276: f64 = if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (s.v[167] <= assign29770_cond_e42274)) { 1.0 } else { 0.0 };
            assign29770_cond_e42276 != 0.0
        } {
            assign29770_loop_guard += 1;
            assert!(assign29770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) {
                s.store_scalar(1593, 0.0);
                s.store_mul_add_rhs(1569, 225, 1571, 1547);
            }
            s.b[1616] = (s.v[1569] < 5.0);
            s.v[1616] = if s.b[1616] { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && s.b[1616]) {
                s.store_ad_value(1589, A::mul3(A::square(s.ad_value(1569)), s.ad_value(1569), A::offset(A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771)));
                s.store_ad_value(1590, A::mul_offset_rhs(A::square(s.ad_value(1569)), A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
                s.store_mul3_lhs(1591, 1597, 1589, 1589);
                s.store_mul_ad_lhs(1592, A::mul3_scaled_output(s.ad_value(1597), s.ad_value(225), s.ad_value(1589), 2.0), 1590);
                s.store_mul_offset_ad_rhs(1593, 1569, A::mul_offset_rhs(s.ad_value(1569), A::mul_offset_rhs(s.ad_value(1569), A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_ad(1594, A::mul_offset_rhs(s.ad_value(1569), A::mul_offset_rhs(s.ad_value(1569), A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758))), 0.707106781186548);
                s.store_sqrt_offset_ad(1595, A::add(A::square(s.ad_value(1593)), s.ad_value(1591)), 1e-50);
                s.store_ad_value(1596, A::div_scaled_inputs(A::add(A::mul3_scaled_output(s.ad_value(225), s.ad_value(1594), s.ad_value(1593), 2.0), s.ad_value(1592)), 1.0, s.ad_value(1595), 2.0));
            }
            s.b[1617] = (s.v[1569] < 80.0);
            s.v[1617] = if s.b[1617] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1616])) && s.b[1617]) {
                s.store_exp(243, 1569);
                s.store_mul_offset_rhs(1591, 1597, 243, (-1.0));
                s.store_mul3_lhs(1592, 1597, 225, 243);
            }
            if (((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1616])) && (!s.b[1617])) {
                s.store_exp_mul(1598, 225, 1571);
                s.store_mul_sub_rhs(1591, 1582, 1598, 1588);
                s.store_mul3_lhs(1592, 1582, 225, 1598);
            }
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1616])) {
                s.store_sqrt_add_ad(1595, A::offset(s.ad_value(1569), (-1.0)), s.ad_value(1591));
                s.store_scaled_div_ad_lhs(1596, A::add(s.ad_value(225), s.ad_value(1592)), 1595, 0.5);
            }
            if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) {
                s.store_ad_value(1599, A::add_scaled_inputs_product(s.ad_value(1557), 1.0, s.ad_value(1571), (-1.0), s.ad_value(1555), s.ad_value(1595), (-1.0)));
                s.store_sub_from_scalar_ad(1600, (-1.0), A::mul(s.ad_value(1555), s.ad_value(1596)));
            }
            s.b[1618] = (s.v[1544] == 1.0);
            s.v[1618] = if s.b[1618] { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && s.b[1618]) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1618])) {
                s.store_scaled_div(494, 1599, 1600, -1.0);
            }
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1618])) {
                s.store_scaled_offset_ad(1601, {
                    if (1.0 >= ((s.v[1571]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1571))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1619] = (((s.v[494]) as f64).abs() > s.v[1601]);
            s.v[1619] = if s.b[1619] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1618])) && s.b[1619]) {
                s.store_scale(494, 1601, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1618])) {
                s.store_add(1571, 1571, 494);
            }
            s.b[1620] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1599]) as f64).abs() <= 1e-8));
            s.v[1620] = if s.b[1620] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1618])) && s.b[1620]) {
                s.store_scalar(1544, 1.0);
            }
            if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.b[1622] = (s.v[1569] < 5.0);
        s.v[1622] = if s.b[1622] { 1.0 } else { 0.0 };

        if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && s.b[1622]) {
            s.store_offset_square(1602, 1593, (10.0 * 2.220446049250313e-16));
            s.store_offset(1603, 1593, (10.0 * 2.220446049250313e-16));
        }

        if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1622])) {
            s.store_offset(1602, 1569, (-1.0));
            s.store_sqrt(1603, 1602);
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) {
            s.store_mul(458, 1554, 1603);
            s.store_div_from_scalar_add_ad(1522, 1.0, s.ad_value(1595), s.ad_value(1603));
            s.store_mul3_lhs(460, 1554, 1591, 1522);
            s.store_add(459, 458, 460);
        }

        if (((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) {
            s.store_sub(460, 459, 458);
        }

        if (((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) {
            s.store_ad_value(1525, {
                if (p.p43 == 1.0) {
                    A::mul(s.ad_value(287), s.ad_value(1534))
                } else {
                    A::mul(s.ad_value(108), s.ad_value(1534))
                }
            });
        }

        s.b[1624] = (((s.v[1540] != 0.0) && (p.p43 == 0.0)) || ((s.v[1538] != 0.0) && (p.p43 == 1.0)));
        s.v[1624] = if s.b[1624] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1624]) {
            s.store_mul(455, 1525, 459);
        }

    }

    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1624]) {
            s.store_mul(457, 1525, 458);
        }

        s.b[1625] = (((s.v[1541] != 0.0) && (p.p43 == 0.0)) || ((s.v[1539] != 0.0) && (p.p43 == 1.0)));
        s.v[1625] = if s.b[1625] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1625]) {
            s.store_mul(454, 1525, 459);
            s.store_mul(456, 1525, 458);
        }

        if (((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) {
            s.store_scalar(1538, ((1.0 - 1.0) / 2.0));
            s.store_scalar(1539, ((1.0 + 1.0) / 2.0));
        }

        s.b[1626] = (p.p43 == 1.0);
        s.v[1626] = if s.b[1626] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1626]) {
            s.store_ad_value(1548, A::add_scaled_products(s.ad_value(461), s.ad_value(156), 1.0, s.ad_value(462), A::sub(s.ad_value(156), s.ad_value(157)), 1.0));
            s.store_ad_value(1549, A::add_scaled_products(s.ad_value(461), s.ad_value(157), 1.0, s.ad_value(462), s.ad_value(157), -1.0));
            s.store_ad_value(1550, A::add_scaled_products(s.ad_value(461), s.ad_value(158), 1.0, s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157)), 1.0));
            s.store_sub(1551, 1549, 1548);
            s.store_sub(1553, 1550, 1548);
            s.store_neg(1552, 1548);
            s.store_ad_value(1540, A::add_scaled_products(s.ad_value(1538), s.ad_value(461), 1.0, s.ad_value(1539), s.ad_value(462), 1.0));
            s.store_ad_value(1541, A::add_scaled_products(s.ad_value(1538), s.ad_value(462), 1.0, s.ad_value(1539), s.ad_value(461), 1.0));
            s.store_offset_ad(1546, A::add_scaled_products(s.ad_value(1540), s.ad_value(1552), 1.0, s.ad_value(1541), s.ad_value(1551), 1.0), (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1626])) {
            s.store_ad_value(1540, A::add_scaled_products(s.ad_value(1538), s.ad_value(461), 1.0, s.ad_value(1539), s.ad_value(462), 1.0));
            s.store_ad_value(1541, A::add_scaled_products(s.ad_value(1538), s.ad_value(462), 1.0, s.ad_value(1539), s.ad_value(461), 1.0));
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1626])) && (s.v[1538] != 0.0)) {
            s.store_ad_value(1553, A::add_scaled_products(s.ad_value(461), s.ad_value(158), 1.0, s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157)), 1.0));
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1626])) && (s.v[1539] != 0.0)) {
            s.store_ad_value(1553, A::add_scaled_products(s.ad_value(462), s.ad_value(158), 1.0, s.ad_value(461), A::sub(s.ad_value(158), s.ad_value(157)), 1.0));
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1626])) {
            s.store_scalar(1546, 0.0);
        }

        if (((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) {
            s.store_neg(1521, 1546);
        }

        s.b[1627] = (s.v[1521] > s.v[141]);
        s.v[1627] = if s.b[1627] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1627]) {
            s.store_sub(1522, 1521, 141);
            s.store_sub(1523, 140, 141);
            s.store_div(44, 1522, 1523);
            s.store_square(45, 44);
            s.store_mul(46, 45, 44);
            s.store_square(47, 45);
            s.store_div_from_scalar_add_ad(1531, 1.0, A::add_scaled_inputs3_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, 1.0), s.ad_value(47));
            s.store_mul_sub_from_scalar_rhs(1531, 1523, 1.0, 1531);
            s.store_add(1528, 141, 1531);
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1627])) {
            s.copy_ad(1528, 1521);
        }

        if (((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) {
            s.store_offset_scaled(1547, 1528, -1.0, (-1e-12));
            s.store_scale(1555, 1554, s.v[1533]);
            s.store_square(1556, 1555);
            s.store_sub_from_scalar(1557, s.v[82], 1553);
            s.store_div_from_scalar(1521, s.v[69], 230);
            s.store_mul_ad(1558, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(s.ad_value(1521)));
            s.store_neg(1559, 1547);
        }

        s.b[1628] = (s.v[1557] < s.v[1559]);
        s.v[1628] = if s.b[1628] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1628]) {
            s.store_div_from_scalar_mul_ad(1522, 1.0, s.ad_value(225), s.ad_value(1554));
            s.store_scale(1531, 1522, s.v[1532]);
            s.store_offset_scaled(1560, 1531, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(1561, 1560, 1560, 8.0, 0.0, 1560);
            s.store_sub(1562, 237, 1558);
            s.store_mul_add_rhs(1530, 225, 1557, 1547);
            s.store_sub_from_scalar_ad(1563, (7.0 * 1.414213562373095), A::mul_scaled_lhs(s.ad_value(1531), 9.0, A::offset(s.ad_value(1530), (-2.0))));
            s.store_square(1564, 1563);
        }

        s.b[1629] = (s.v[1561] < (s.v[1564] * 1e-8));
        s.v[1629] = if s.b[1629] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1628]) && s.b[1629]) {
            s.store_ad_value(1566, A::add_scaled_inputs_product(A::offset(s.ad_value(1563), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1561), 0.5, s.ad_value(1563), 1.0), 1.0, s.ad_value(1531), A::offset(s.ad_value(1530), (-2.0)), 9.0));
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1628]) && (!s.b[1629])) {
            s.store_sqrt_add(1565, 1561, 1564);
            s.store_ad_value(1566, A::add_scaled_offset_product_rhs(A::offset(s.ad_value(1565), ((-7.0) * 1.414213562373095)), 1.0, s.ad_value(1531), s.ad_value(1530), (-2.0), 9.0));
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1628]) {
            s.store_powf(1567, 1566, 0.3333333333333333);
            s.store_ad_value(1568, A::add_scaled_inputs_product(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1531), 12.0)), 1.0, s.ad_value(1567), 2.0, s.ad_value(1567), s.ad_value(1567), 1.414213562373095));
            s.store_div(1569, 1568, 1567);
            s.store_ad_value(1570, A::add_scaled_product(s.ad_value(1547), (-1.0), s.ad_value(1569), s.ad_value(227), 1.0));
            s.store_add(1522, 1570, 1547);
            s.store_div(1523, 1522, 1562);
            s.store_sqrt_square_offset(1524, 1523, 1.0);
            s.store_sub_ad_lhs(1571, A::div(s.ad_value(1522), s.ad_value(1524)), 1547);
            s.store_sub(1523, 1557, 1571);
            s.store_scale(459, 1523, s.v[1532]);
            s.copy_ad(458, 459);
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) {
            s.store_scalar(1569, 3.0);
            s.store_sub_ad_lhs(1572, A::div(s.ad_value(1569), s.ad_value(225)), 1547);
            s.store_exp_neg_input(1531, 1569);
            s.store_offset_div_ad(1530, A::add_scaled_inputs(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1557), s.ad_value(1547))), (-1.0)), 4.0, s.ad_value(1531), 4.0), A::mul(s.ad_value(1556), s.ad_value(226)), 1.0);
        }

        s.b[1630] = (s.v[1530] < (10.0 * 2.220446049250313e-16));
        s.v[1630] = if s.b[1630] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1630]) {
            s.store_scalar(1530, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) {
            s.store_add_ad_rhs(1572, 1557, A::mul3_scaled_output(s.ad_value(1556), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1530))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1569, 225, 1572, 1547);
            s.store_exp_neg_input(1531, 1569);
            s.store_offset_div_ad(1530, A::add_scaled_inputs(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1557), s.ad_value(1547))), (-1.0)), 4.0, s.ad_value(1531), 4.0), A::mul(s.ad_value(1556), s.ad_value(226)), 1.0);
        }

        s.b[1631] = (s.v[1530] < (10.0 * 2.220446049250313e-16));
        s.v[1631] = if s.b[1631] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1631]) {
            s.store_scalar(1530, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) {
            s.store_add_ad_rhs(1572, 1557, A::mul3_scaled_output(s.ad_value(1556), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1530))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1569, 225, 1572, 1547);
        }

        s.b[1632] = (s.v[1569] < 3.0);
        s.v[1632] = if s.b[1632] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1632]) {
            s.store_scalar(1573, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(1574, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
            s.store_offset_div_from_scalar_ad(1575, 1.0, A::mul(s.ad_value(225), s.ad_value(1555)), (1.0 / 1.414213562373095));
            s.store_ad_value(1576, A::div_scaled_inputs(A::add(s.ad_value(1557), s.ad_value(1547)), -1.0, s.ad_value(1555), 1.0));
            s.store_ad_value(1577, A::add_scaled_inputs3(A::div_scaled_product(A::square(s.ad_value(1574)), s.ad_value(1574), 1.0, A::mul3_scaled_output(s.ad_value(1573), s.ad_value(1573), s.ad_value(1573), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1574), s.ad_value(1575), 1.0, s.ad_value(1573), s.ad_value(1573), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(1576), 1.0, s.ad_value(1573), 2.0), 1.0));
            s.store_div_ad(1578, A::add_scaled_square_product(s.ad_value(1574), (-1.0), s.ad_value(1573), s.ad_value(1575), 3.0), A::mul_scaled_lhs(s.ad_value(1573), 9.0, s.ad_value(1573)));
            s.store_sqrt_ad(1526, A::add_scaled_square_product(s.ad_value(1577), 1.0, A::square(s.ad_value(1578)), s.ad_value(1578), 1.0));
            s.store_powf_ad(1579, A::sub(s.ad_value(1526), s.ad_value(1577)), 0.3333333333333333);
            s.store_neg_ad(1580, A::powf(A::add(s.ad_value(1577), s.ad_value(1526)), 0.3333333333333333));
            s.store_ad_value(1530, A::add_scaled_inputs3(s.ad_value(1579), 1.0, s.ad_value(1580), 1.0, A::div_scaled_inputs(s.ad_value(1574), 1.0, s.ad_value(1573), 3.0), -1.0));
            s.store_ad_value(1572, A::add_scaled_product(s.ad_value(1547), (-1.0), s.ad_value(1530), s.ad_value(227), 1.0));
            s.store_mul_add_rhs(1569, 225, 1572, 1547);
        }

        s.b[1633] = (p.p41 > 0.0);
        s.v[1633] = if s.b[1633] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1633]) {
            s.store_offset_add(1581, 1557, 1547, 0.1);
            s.store_offset_exp_ad(1588, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1547), -1.0), 1e-50);
            s.store_scale(1521, 230, 1.0 / (s.v[69]));
            s.store_square(1582, 1521);
            s.store_mul(1583, 1582, 1588);
            s.store_mul(1521, 226, 1556);
            s.store_mul(1584, 225, 1581);
            s.store_ad_value(1585, A::add_scaled_inputs_product(A::ln(A::add_scaled_square_product(s.ad_value(1584), 1.0, s.ad_value(1583), s.ad_value(1521), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1582), s.ad_value(1521))), (-1.0), s.ad_value(225), s.ad_value(1547), 1.0));
            s.store_offset_sub(44, 1584, 1585, (-1.0));
            s.store_scale(45, 1584, 4.0);
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1633]) {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1633]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1522, 44, 45, 0.5, 0.5);
            s.store_scaled_sub_from_scalar_ad(1523, 1.0, A::div(A::offset(s.ad_value(44), 2.0), s.ad_value(45)), 0.5);
            s.store_ad_value(1585, A::add_scaled_inputs3(s.ad_value(1584), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5)));
            s.store_sub(1584, 1584, 1585);
            s.store_add_scaled_inputs(1584, 1584, 1.0, 225, 0.1);
            s.store_ad_value(1586, A::add_scaled_inputs_product(A::ln(A::add_scaled_square_product(s.ad_value(1584), 1.0, s.ad_value(1583), s.ad_value(1521), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1582), s.ad_value(1521))), (-1.0), s.ad_value(225), s.ad_value(1547), 1.0));
            s.copy_ad(1587, 1569);
            s.store_offset_sub(44, 1586, 1587, (-(0.0008 * 75.0)));
            s.store_scale(45, 1586, (4.0 * (0.0008 * 75.0)));
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1633]) {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1633]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1522, 44, 45, 0.5, 0.5);
            s.store_scaled_sub_from_scalar_ad(1523, 1.0, A::div(A::offset(s.ad_value(44), ((2.0 * 0.0008) * 75.0)), s.ad_value(45)), 0.5);
            s.store_ad_value(1569, A::add_scaled_inputs3(s.ad_value(1586), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5)));
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) {
            s.store_sub_ad_lhs(1571, A::div(s.ad_value(1569), s.ad_value(225)), 1547);
            s.store_add_ad(1522, A::offset(s.ad_value(1569), (-1.0)), A::exp_scaled_input(s.ad_value(1569), -1.0));
        }

        s.b[1634] = (s.v[1522] < (10.0 * 2.220446049250313e-16));
        s.v[1634] = if s.b[1634] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1634]) {
            s.store_scalar(1522, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) {
            s.store_sqrt(1523, 1522);
            s.store_mul(458, 1554, 1523);
            s.store_scaled_sub(459, 1557, 1571, s.v[1532]);
        }

        s.b[1635] = (p.p41 == 1.0);
        s.v[1635] = if s.b[1635] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) {
            s.store_exp_ad(1588, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1547), -1.0));
        }

    }

    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) {
            s.store_scale(1521, 230, 1.0 / (s.v[69]));
            s.store_square(1582, 1521);
            s.store_mul(1597, 1582, 1588);
            s.store_scalar(1544, 0.0);
            s.store_scalar(1591, 0.0);
            s.store_scalar(1595, 0.0);
            s.store_scalar(167, 1.0);
        }

        let mut assign31370_loop_guard: usize = 0;
        while {
            let assign31370_cond_e45508: f64 = (2.0 * 20.0);
            let assign31370_cond_e45510: f64 = (assign31370_cond_e45508 + 1.0);
            let assign31370_cond_e45512: f64 = if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (s.v[167] <= assign31370_cond_e45510)) { 1.0 } else { 0.0 };
            assign31370_cond_e45512 != 0.0
        } {
            assign31370_loop_guard += 1;
            assert!(assign31370_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) {
                s.store_scalar(1593, 0.0);
                s.store_mul_add_rhs(1569, 225, 1571, 1547);
            }
            s.b[1636] = (s.v[1569] < 5.0);
            s.v[1636] = if s.b[1636] { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && s.b[1636]) {
                s.store_ad_value(1589, A::mul3(A::square(s.ad_value(1569)), s.ad_value(1569), A::offset(A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771)));
                s.store_ad_value(1590, A::mul_offset_rhs(A::square(s.ad_value(1569)), A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
                s.store_mul3_lhs(1591, 1597, 1589, 1589);
                s.store_mul_ad_lhs(1592, A::mul3_scaled_output(s.ad_value(1597), s.ad_value(225), s.ad_value(1589), 2.0), 1590);
                s.store_mul_offset_ad_rhs(1593, 1569, A::mul_offset_rhs(s.ad_value(1569), A::mul_offset_rhs(s.ad_value(1569), A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_ad(1594, A::mul_offset_rhs(s.ad_value(1569), A::mul_offset_rhs(s.ad_value(1569), A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758))), 0.707106781186548);
                s.store_sqrt_offset_ad(1595, A::add(A::square(s.ad_value(1593)), s.ad_value(1591)), 1e-50);
                s.store_ad_value(1596, A::div_scaled_inputs(A::add(A::mul3_scaled_output(s.ad_value(225), s.ad_value(1594), s.ad_value(1593), 2.0), s.ad_value(1592)), 1.0, s.ad_value(1595), 2.0));
            }
            s.b[1637] = (s.v[1569] < 80.0);
            s.v[1637] = if s.b[1637] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1636])) && s.b[1637]) {
                s.store_exp(243, 1569);
                s.store_mul_offset_rhs(1591, 1597, 243, (-1.0));
                s.store_mul3_lhs(1592, 1597, 225, 243);
            }
            if (((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1636])) && (!s.b[1637])) {
                s.store_exp_mul(1598, 225, 1571);
                s.store_mul_sub_rhs(1591, 1582, 1598, 1588);
                s.store_mul3_lhs(1592, 1582, 225, 1598);
            }
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1636])) {
                s.store_sqrt_add_ad(1595, A::offset(s.ad_value(1569), (-1.0)), s.ad_value(1591));
                s.store_scaled_div_ad_lhs(1596, A::add(s.ad_value(225), s.ad_value(1592)), 1595, 0.5);
            }
            if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) {
                s.store_ad_value(1599, A::add_scaled_inputs_product(s.ad_value(1557), 1.0, s.ad_value(1571), (-1.0), s.ad_value(1555), s.ad_value(1595), (-1.0)));
                s.store_sub_from_scalar_ad(1600, (-1.0), A::mul(s.ad_value(1555), s.ad_value(1596)));
            }
            s.b[1638] = (s.v[1544] == 1.0);
            s.v[1638] = if s.b[1638] { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && s.b[1638]) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1638])) {
                s.store_scaled_div(494, 1599, 1600, -1.0);
            }
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1638])) {
                s.store_scaled_offset_ad(1601, {
                    if (1.0 >= ((s.v[1571]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1571))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1639] = (((s.v[494]) as f64).abs() > s.v[1601]);
            s.v[1639] = if s.b[1639] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1638])) && s.b[1639]) {
                s.store_scale(494, 1601, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1638])) {
                s.store_add(1571, 1571, 494);
            }
            s.b[1640] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1599]) as f64).abs() <= 1e-8));
            s.v[1640] = if s.b[1640] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1638])) && s.b[1640]) {
                s.store_scalar(1544, 1.0);
            }
            if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.b[1642] = (s.v[1569] < 5.0);
        s.v[1642] = if s.b[1642] { 1.0 } else { 0.0 };

        if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && s.b[1642]) {
            s.store_offset_square(1602, 1593, (10.0 * 2.220446049250313e-16));
            s.store_offset(1603, 1593, (10.0 * 2.220446049250313e-16));
        }

        if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1642])) {
            s.store_offset(1602, 1569, (-1.0));
            s.store_sqrt(1603, 1602);
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) {
            s.store_mul(458, 1554, 1603);
            s.store_div_from_scalar_add_ad(1522, 1.0, s.ad_value(1595), s.ad_value(1603));
            s.store_mul3_lhs(460, 1554, 1591, 1522);
            s.store_add(459, 458, 460);
        }

        if (((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) {
            s.store_sub(460, 459, 458);
        }

        if (((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) {
            s.store_ad_value(1525, {
                if (p.p43 == 1.0) {
                    A::mul(s.ad_value(287), s.ad_value(1534))
                } else {
                    A::mul(s.ad_value(108), s.ad_value(1534))
                }
            });
        }

        s.b[1644] = (((s.v[1540] != 0.0) && (p.p43 == 0.0)) || ((s.v[1538] != 0.0) && (p.p43 == 1.0)));
        s.v[1644] = if s.b[1644] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1644]) {
            s.store_mul(455, 1525, 459);
            s.store_mul(457, 1525, 458);
        }

        s.b[1645] = (((s.v[1541] != 0.0) && (p.p43 == 0.0)) || ((s.v[1539] != 0.0) && (p.p43 == 1.0)));
        s.v[1645] = if s.b[1645] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1645]) {
            s.store_mul(454, 1525, 459);
            s.store_mul(456, 1525, 458);
        }

        if ((p.p24 != 0.0) && s.b[1604]) {
            s.store_add_scaled_inputs(266, 462, s.v[566], 461, s.v[565]);
        }

        if (((p.p24 != 0.0) && s.b[1604]) && (s.v[266] != 0.0)) {
            s.store_add_scaled_inputs(269, 462, p.p170, 461, p.p169);
        }

        s.b[1646] = (p.p43 == 1.0);
        s.v[1646] = if s.b[1646] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1604]) && (s.v[266] != 0.0)) && s.b[1646]) {
            s.store_ad_value(1522, A::add_scaled_products(s.ad_value(462), s.ad_value(287), 1.0, s.ad_value(461), s.ad_value(288), 1.0));
            s.store_mul_neg_rhs(269, 269, 1522);
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (s.v[266] != 0.0)) && (!s.b[1646])) {
            s.store_mul_neg_rhs(269, 269, 108);
        }

        if (((p.p24 != 0.0) && s.b[1604]) && (s.v[266] != 0.0)) {
            s.store_ad_value(268, A::add_scaled_product(s.ad_value(268), 1.0, s.ad_value(269), A::sub(s.ad_value(158), s.ad_value(157)), -1.0));
        }

        if ((p.p24 != 0.0) && s.b[1604]) {
            s.store_add_scaled_inputs(266, 461, s.v[566], 462, s.v[565]);
        }

        if (((p.p24 != 0.0) && s.b[1604]) && (s.v[266] != 0.0)) {
            s.store_add_scaled_inputs(270, 461, p.p170, 462, p.p169);
        }

        s.b[1647] = (p.p43 == 1.0);
        s.v[1647] = if s.b[1647] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1604]) && (s.v[266] != 0.0)) && s.b[1647]) {
            s.store_ad_value(1522, A::add_scaled_products(s.ad_value(461), s.ad_value(287), 1.0, s.ad_value(462), s.ad_value(288), 1.0));
            s.store_mul_neg_rhs(270, 270, 1522);
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (s.v[266] != 0.0)) && (!s.b[1647])) {
            s.store_mul_neg_rhs(270, 270, 108);
        }

        if (((p.p24 != 0.0) && s.b[1604]) && (s.v[266] != 0.0)) {
            s.store_ad_value(267, A::add_scaled_product(s.ad_value(267), 1.0, s.ad_value(270), s.ad_value(158), -1.0));
        }

        s.b[1648] = (((s.v[613] == 1.0) && (!s.b[565])) || ((s.v[613] != 1.0) && (!s.b[566])));
        s.v[1648] = if s.b[1648] { 1.0 } else { 0.0 };

        s.b[1649] = (p.p43 == 1.0);
        s.v[1649] = if s.b[1649] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (!s.b[1604])) && s.b[1648]) && s.b[1649]) {
            s.store_scale(269, 288, ((-s.v[1532]) * p.p188));
        }

        if ((((p.p24 != 0.0) && (!s.b[1604])) && s.b[1648]) && (!s.b[1649])) {
            s.store_scale(269, 108, ((-s.v[1532]) * p.p188));
        }

        if (((p.p24 != 0.0) && (!s.b[1604])) && (!s.b[1648])) {
            s.store_add_scaled_inputs(269, 462, p.p170, 461, p.p169);
        }

        s.b[1650] = (p.p43 == 1.0);
        s.v[1650] = if s.b[1650] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (!s.b[1604])) && (!s.b[1648])) && s.b[1650]) {
            s.store_ad_value(1522, A::add_scaled_products(s.ad_value(462), s.ad_value(287), 1.0, s.ad_value(461), s.ad_value(288), 1.0));
            s.store_mul_neg_rhs(269, 269, 1522);
        }

        if ((((p.p24 != 0.0) && (!s.b[1604])) && (!s.b[1648])) && (!s.b[1650])) {
            s.store_mul_neg_rhs(269, 269, 108);
        }

        if ((p.p24 != 0.0) && (!s.b[1604])) {
            s.store_mul_scaled_ad_rhs(268, 269, -1.0, A::sub(s.ad_value(158), s.ad_value(157)));
        }

        s.b[1651] = (((s.v[613] == 1.0) && (!s.b[566])) || ((s.v[613] != 1.0) && (!s.b[565])));
        s.v[1651] = if s.b[1651] { 1.0 } else { 0.0 };

        s.b[1652] = (p.p43 == 1.0);
        s.v[1652] = if s.b[1652] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (!s.b[1604])) && s.b[1651]) && s.b[1652]) {
            s.store_scale(270, 287, ((-s.v[1532]) * p.p188));
        }

        if ((((p.p24 != 0.0) && (!s.b[1604])) && s.b[1651]) && (!s.b[1652])) {
            s.store_scale(270, 108, ((-s.v[1532]) * p.p188));
        }

        if (((p.p24 != 0.0) && (!s.b[1604])) && (!s.b[1651])) {
            s.store_add_scaled_inputs(270, 461, p.p170, 462, p.p169);
        }

        s.b[1653] = (p.p43 == 1.0);
        s.v[1653] = if s.b[1653] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (!s.b[1604])) && (!s.b[1651])) && s.b[1653]) {
            s.store_ad_value(1522, A::add_scaled_products(s.ad_value(461), s.ad_value(287), 1.0, s.ad_value(462), s.ad_value(288), 1.0));
            s.store_mul_neg_rhs(270, 270, 1522);
        }

        if ((((p.p24 != 0.0) && (!s.b[1604])) && (!s.b[1651])) && (!s.b[1653])) {
            s.store_mul_neg_rhs(270, 270, 108);
        }

        if ((p.p24 != 0.0) && (!s.b[1604])) {
            s.store_mul_neg_lhs(267, 270, 158);
        }

        s.b[1654] = (p.p43 == 1.0);
        s.v[1654] = if s.b[1654] { 1.0 } else { 0.0 };

        if s.b[1654] {
            s.copy_ad(1670, 590);
            s.copy_ad(1671, 591);
            s.store_scale_ad(1672, A::exp_scaled_input(A::add_scaled_inputs(A::sub_from_scalar((s.v[87] * s.v[114]), A::mul(s.ad_value(237), s.ad_value(225))), 1.0, A::ln_scaled_input(s.ad_value(429), 1.0 / (s.v[81])), p.p175), 1.0 / (p.p174)), p.p173);
            s.store_scale_ad(1673, A::exp_scaled_input(A::add_scaled_inputs(A::sub_from_scalar((s.v[87] * s.v[114]), A::mul(s.ad_value(237), s.ad_value(225))), 1.0, A::ln_scaled_input(s.ad_value(429), 1.0 / (s.v[81])), p.p176), 1.0 / (p.p174)), p.p173);
            s.store_scaled_mul(1677, 286, 1672, p.p237);
            s.store_scaled_mul(1679, 286, 1673, p.p237);
            s.store_scaled_mul(1678, 285, 1672, p.p237);
            s.store_scaled_mul(1680, 285, 1673, p.p237);
            s.store_scale(1656, 429, 1.0 / (s.v[81]));
            s.store_offset(1657, 1677, 1e-50);
            s.store_scale_ad(1675, A::div_from_scalar(p.p174, s.ad_value(225)), 0.0);
            s.store_scale_ad(1676, A::div_from_scalar(p.p174, s.ad_value(225)), 0.0);
            s.store_scale(1674, 227, p.p174);
        }

        s.b[1683] = (s.v[1670] < s.v[1675]);
        s.v[1683] = if s.b[1683] { 1.0 } else { 0.0 };

        if (s.b[1654] && s.b[1683]) {
            s.store_exp_div(1656, 1670, 1674);
            s.store_mul_offset_rhs(282, 1677, 1656, (-1.0));
        }

        if (s.b[1654] && (!s.b[1683])) {
            s.store_exp_div(1656, 1675, 1674);
            s.store_ad_value(282, A::add_scaled_offset_product_rhs(A::mul3(A::div(s.ad_value(1677), s.ad_value(1674)), s.ad_value(1656), A::sub(s.ad_value(1670), s.ad_value(1675))), 1.0, s.ad_value(1677), s.ad_value(1656), (-1.0), 1.0));
        }

        if s.b[1654] {
            s.store_ad_value(282, A::add_scaled_product(s.ad_value(282), 1.0, s.ad_value(1670), s.ad_value(1679), p.p178));
        }

        s.b[1684] = (s.v[1671] < s.v[1676]);
        s.v[1684] = if s.b[1684] { 1.0 } else { 0.0 };

        if (s.b[1654] && s.b[1684]) {
            s.store_exp_div(1656, 1671, 1674);
            s.store_mul_offset_rhs(281, 1678, 1656, (-1.0));
        }

        if (s.b[1654] && (!s.b[1684])) {
            s.store_exp_div(1656, 1676, 1674);
            s.store_ad_value(281, A::add_scaled_offset_product_rhs(A::mul3(A::div(s.ad_value(1678), s.ad_value(1674)), s.ad_value(1656), A::sub(s.ad_value(1671), s.ad_value(1676))), 1.0, s.ad_value(1678), s.ad_value(1656), (-1.0), 1.0));
        }

        if s.b[1654] {
            s.store_ad_value(281, A::add_scaled_product(s.ad_value(281), 1.0, s.ad_value(1671), s.ad_value(1680), p.p178));
            s.store_add_scaled_inputs(282, 282, 1.0, 1670, s.v[142]);
            s.store_add_scaled_inputs(281, 281, 1.0, 1671, s.v[142]);
            s.store_scalar(1664, (p.p179 * p.p2));
            s.store_scalar(1665, (p.p179 * p.p3));
            s.store_scalar(1663, (p.p237 - p.p238));
        }

        s.b[1685] = (s.v[1663] <= 0.0);
        s.v[1685] = if s.b[1685] { 1.0 } else { 0.0 };

        if (s.b[1654] && s.b[1685]) {
            s.store_scalar(1664, 0.0);
            s.store_scalar(1665, 0.0);
        }

        s.b[1686] = (p.p5 > s.v[287]);
        s.v[1686] = if s.b[1686] { 1.0 } else { 0.0 };

        if (s.b[1654] && s.b[1686]) {
            s.store_scale_ad(1667, A::sub_from_scalar(p.p5, s.ad_value(287)), p.p180);
            s.store_scale(1669, 287, p.p181);
        }

        s.b[1687] = (s.v[1671] < 0.0);
        s.v[1687] = if s.b[1687] { 1.0 } else { 0.0 };

        s.b[1688] = (s.v[1665] > 0.0);
        s.v[1688] = if s.b[1688] { 1.0 } else { 0.0 };

        if (((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1688]) {
            s.store_sub_from_scalar_ad(1681, 1.0, A::scale(s.ad_value(1671), 1.0 / (p.p185)));
        }

        s.b[1689] = (p.p182 == 0.5);
        s.v[1689] = if s.b[1689] { 1.0 } else { 0.0 };

        if ((((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1688]) && s.b[1689]) {
            s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));
        }

        if ((((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1688]) && (!s.b[1689])) {
            s.store_powf(1682, 1681, (-p.p182));
        }

        if (((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1688]) {
            s.store_ad_value(283, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1665), 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), (p.p185 * 1.0 / ((1.0 - p.p182)))));
        }

        if (((s.b[1654] && s.b[1686]) && s.b[1687]) && (!s.b[1688])) {
            s.store_scalar(283, 0.0);
        }

        s.b[1690] = (s.v[1667] > 0.0);
        s.v[1690] = if s.b[1690] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1690]) {
            s.store_sub_from_scalar_ad(1681, 1.0, A::scale(s.ad_value(1671), 1.0 / (p.p186)));
        }

        s.b[1691] = (p.p183 == 0.5);
        s.v[1691] = if s.b[1691] { 1.0 } else { 0.0 };

        if ((((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1690]) && s.b[1691]) {
            s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));
        }

        if ((((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1690]) && (!s.b[1691])) {
            s.store_powf(1682, 1681, (-p.p183));
        }

        if (((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1690]) {
            s.store_add_ad_rhs(283, 283, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1667), 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), (p.p186 * 1.0 / ((1.0 - p.p183)))));
        }

        s.b[1692] = (s.v[1669] > 0.0);
        s.v[1692] = if s.b[1692] { 1.0 } else { 0.0 };

        if (((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1692]) {
            s.store_sub_from_scalar_ad(1681, 1.0, A::scale(s.ad_value(1671), 1.0 / (p.p187)));
        }

        s.b[1693] = (p.p184 == 0.5);
        s.v[1693] = if s.b[1693] { 1.0 } else { 0.0 };

        if ((((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1692]) && s.b[1693]) {
            s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));
        }

        if ((((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1692]) && (!s.b[1693])) {
            s.store_powf(1682, 1681, (-p.p184));
        }

        if (((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1692]) {
            s.store_add_ad_rhs(283, 283, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1669), 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), (p.p187 * 1.0 / ((1.0 - p.p184)))));
        }

        if ((s.b[1654] && s.b[1686]) && (!s.b[1687])) {
            s.store_ad_value(1656, A::add_scaled_inputs3(s.ad_value(1665), 1.0, s.ad_value(1667), 1.0, s.ad_value(1669), 1.0));
            s.store_ad_value(1657, A::add_scaled_inputs3(s.ad_value(1665), (p.p182 * 1.0 / (p.p185)), s.ad_value(1667), (p.p183 * 1.0 / (p.p186)), s.ad_value(1669), (p.p184 * 1.0 / (p.p187))));
            s.store_mul_ad_rhs(283, 1671, A::add_scaled_product(s.ad_value(1656), 1.0, s.ad_value(1671), s.ad_value(1657), 0.5));
        }

        if (s.b[1654] && (!s.b[1686])) {
            s.store_scalar(1669, (p.p181 * p.p5));
        }

        s.b[1694] = (s.v[1671] < 0.0);
        s.v[1694] = if s.b[1694] { 1.0 } else { 0.0 };

        s.b[1695] = (s.v[1665] > 0.0);
        s.v[1695] = if s.b[1695] { 1.0 } else { 0.0 };

        if (((s.b[1654] && (!s.b[1686])) && s.b[1694]) && s.b[1695]) {
            s.store_sub_from_scalar_ad(1681, 1.0, A::scale(s.ad_value(1671), 1.0 / (p.p185)));
        }

        s.b[1696] = (p.p182 == 0.5);
        s.v[1696] = if s.b[1696] { 1.0 } else { 0.0 };

        if ((((s.b[1654] && (!s.b[1686])) && s.b[1694]) && s.b[1695]) && s.b[1696]) {
            s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));
        }

        if ((((s.b[1654] && (!s.b[1686])) && s.b[1694]) && s.b[1695]) && (!s.b[1696])) {
            s.store_powf(1682, 1681, (-p.p182));
        }

        if (((s.b[1654] && (!s.b[1686])) && s.b[1694]) && s.b[1695]) {
            s.store_ad_value(283, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1665), 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), (p.p185 * 1.0 / ((1.0 - p.p182)))));
        }

        if (((s.b[1654] && (!s.b[1686])) && s.b[1694]) && (!s.b[1695])) {
            s.store_scalar(283, 0.0);
        }

        s.b[1697] = (s.v[1669] > 0.0);
        s.v[1697] = if s.b[1697] { 1.0 } else { 0.0 };

        if (((s.b[1654] && (!s.b[1686])) && s.b[1694]) && s.b[1697]) {
            s.store_sub_from_scalar_ad(1681, 1.0, A::scale(s.ad_value(1671), 1.0 / (p.p187)));
        }

        s.b[1698] = (p.p184 == 0.5);
        s.v[1698] = if s.b[1698] { 1.0 } else { 0.0 };

        if ((((s.b[1654] && (!s.b[1686])) && s.b[1694]) && s.b[1697]) && s.b[1698]) {
            s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));
        }

        if ((((s.b[1654] && (!s.b[1686])) && s.b[1694]) && s.b[1697]) && (!s.b[1698])) {
            s.store_powf(1682, 1681, (-p.p184));
        }

        if (((s.b[1654] && (!s.b[1686])) && s.b[1694]) && s.b[1697]) {
            s.store_add_ad_rhs(283, 283, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1669), 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), (p.p187 * 1.0 / ((1.0 - p.p184)))));
        }

        if ((s.b[1654] && (!s.b[1686])) && (!s.b[1694])) {
            s.store_add(1656, 1665, 1669);
            s.store_add_scaled_inputs(1657, 1665, (p.p182 * 1.0 / (p.p185)), 1669, (p.p184 * 1.0 / (p.p187)));
            s.store_mul_ad_rhs(283, 1671, A::add_scaled_product(s.ad_value(1656), 1.0, s.ad_value(1671), s.ad_value(1657), 0.5));
        }

        s.b[1699] = (p.p4 > s.v[288]);
        s.v[1699] = if s.b[1699] { 1.0 } else { 0.0 };

        if (s.b[1654] && s.b[1699]) {
            s.store_scale_ad(1666, A::sub_from_scalar(p.p4, s.ad_value(288)), p.p180);
            s.store_scale(1668, 288, p.p181);
        }

        s.b[1700] = (s.v[1670] < 0.0);
        s.v[1700] = if s.b[1700] { 1.0 } else { 0.0 };

        s.b[1701] = (s.v[1664] > 0.0);
        s.v[1701] = if s.b[1701] { 1.0 } else { 0.0 };

        if (((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1701]) {
            s.store_sub_from_scalar_ad(1681, 1.0, A::scale(s.ad_value(1670), 1.0 / (p.p185)));
        }

        s.b[1702] = (p.p182 == 0.5);
        s.v[1702] = if s.b[1702] { 1.0 } else { 0.0 };

        if ((((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1701]) && s.b[1702]) {
            s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));
        }

        if ((((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1701]) && (!s.b[1702])) {
            s.store_powf(1682, 1681, (-p.p182));
        }

        if (((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1701]) {
            s.store_ad_value(284, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1664), 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), (p.p185 * 1.0 / ((1.0 - p.p182)))));
        }

        if (((s.b[1654] && s.b[1699]) && s.b[1700]) && (!s.b[1701])) {
            s.store_scalar(284, 0.0);
        }

        s.b[1703] = (s.v[1666] > 0.0);
        s.v[1703] = if s.b[1703] { 1.0 } else { 0.0 };

        if (((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1703]) {
            s.store_sub_from_scalar_ad(1681, 1.0, A::scale(s.ad_value(1670), 1.0 / (p.p186)));
        }

        s.b[1704] = (p.p183 == 0.5);
        s.v[1704] = if s.b[1704] { 1.0 } else { 0.0 };

        if ((((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1703]) && s.b[1704]) {
            s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));
        }

        if ((((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1703]) && (!s.b[1704])) {
            s.store_powf(1682, 1681, (-p.p183));
        }

        if (((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1703]) {
            s.store_add_ad_rhs(284, 284, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1666), 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), (p.p186 * 1.0 / ((1.0 - p.p183)))));
        }

        s.b[1705] = (s.v[1668] > 0.0);
        s.v[1705] = if s.b[1705] { 1.0 } else { 0.0 };

        if (((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1705]) {
            s.store_sub_from_scalar_ad(1681, 1.0, A::scale(s.ad_value(1670), 1.0 / (p.p187)));
        }

        s.b[1706] = (p.p184 == 0.5);
        s.v[1706] = if s.b[1706] { 1.0 } else { 0.0 };

        if ((((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1705]) && s.b[1706]) {
            s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));
        }

        if ((((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1705]) && (!s.b[1706])) {
            s.store_powf(1682, 1681, (-p.p184));
        }

        if (((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1705]) {
            s.store_add_ad_rhs(284, 284, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1668), 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), (p.p187 * 1.0 / ((1.0 - p.p184)))));
        }

        if ((s.b[1654] && s.b[1699]) && (!s.b[1700])) {
            s.store_ad_value(1656, A::add_scaled_inputs3(s.ad_value(1664), 1.0, s.ad_value(1666), 1.0, s.ad_value(1668), 1.0));
            s.store_ad_value(1657, A::add_scaled_inputs3(s.ad_value(1664), (p.p182 * 1.0 / (p.p185)), s.ad_value(1666), (p.p183 * 1.0 / (p.p186)), s.ad_value(1668), (p.p184 * 1.0 / (p.p187))));
            s.store_mul_ad_rhs(284, 1670, A::add_scaled_product(s.ad_value(1656), 1.0, s.ad_value(1670), s.ad_value(1657), 0.5));
        }

        if (s.b[1654] && (!s.b[1699])) {
            s.store_scalar(1668, (p.p181 * p.p4));
        }

        s.b[1707] = (s.v[1670] < 0.0);
        s.v[1707] = if s.b[1707] { 1.0 } else { 0.0 };

        s.b[1708] = (s.v[1664] > 0.0);
        s.v[1708] = if s.b[1708] { 1.0 } else { 0.0 };

        if (((s.b[1654] && (!s.b[1699])) && s.b[1707]) && s.b[1708]) {
            s.store_sub_from_scalar_ad(1681, 1.0, A::scale(s.ad_value(1670), 1.0 / (p.p185)));
        }

        s.b[1709] = (p.p182 == 0.5);
        s.v[1709] = if s.b[1709] { 1.0 } else { 0.0 };

        if ((((s.b[1654] && (!s.b[1699])) && s.b[1707]) && s.b[1708]) && s.b[1709]) {
            s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));
        }

        if ((((s.b[1654] && (!s.b[1699])) && s.b[1707]) && s.b[1708]) && (!s.b[1709])) {
            s.store_powf(1682, 1681, (-p.p182));
        }

        if (((s.b[1654] && (!s.b[1699])) && s.b[1707]) && s.b[1708]) {
            s.store_ad_value(284, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1664), 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), (p.p185 * 1.0 / ((1.0 - p.p182)))));
        }

        if (((s.b[1654] && (!s.b[1699])) && s.b[1707]) && (!s.b[1708])) {
            s.store_scalar(284, 0.0);
        }

        s.b[1710] = (s.v[1668] > 0.0);
        s.v[1710] = if s.b[1710] { 1.0 } else { 0.0 };

        if (((s.b[1654] && (!s.b[1699])) && s.b[1707]) && s.b[1710]) {
            s.store_sub_from_scalar_ad(1681, 1.0, A::scale(s.ad_value(1670), 1.0 / (p.p187)));
        }

        s.b[1711] = (p.p184 == 0.5);
        s.v[1711] = if s.b[1711] { 1.0 } else { 0.0 };

        if ((((s.b[1654] && (!s.b[1699])) && s.b[1707]) && s.b[1710]) && s.b[1711]) {
            s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));
        }

        if ((((s.b[1654] && (!s.b[1699])) && s.b[1707]) && s.b[1710]) && (!s.b[1711])) {
            s.store_powf(1682, 1681, (-p.p184));
        }

        if (((s.b[1654] && (!s.b[1699])) && s.b[1707]) && s.b[1710]) {
            s.store_add_ad_rhs(284, 284, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1668), 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), (p.p187 * 1.0 / ((1.0 - p.p184)))));
        }

        if ((s.b[1654] && (!s.b[1699])) && (!s.b[1707])) {
            s.store_add(1656, 1664, 1668);
            s.store_add_scaled_inputs(1657, 1664, (p.p182 * 1.0 / (p.p185)), 1668, (p.p184 * 1.0 / (p.p187)));
            s.store_mul_ad_rhs(284, 1670, A::add_scaled_product(s.ad_value(1656), 1.0, s.ad_value(1670), s.ad_value(1657), 0.5));
        }

        s.b[1712] = (s.v[1665] > 0.0);
        s.v[1712] = if s.b[1712] { 1.0 } else { 0.0 };

        if (s.b[1654] && s.b[1712]) {
            s.store_scaled_mul(1659, 544, 1663, ((-1.6021918e-19) * p.p3));
            s.store_scale(1661, 1659, (-0.001));
            s.store_ad_value(44, A::add_scaled_inputs3(s.ad_value(1659), -1.0, s.ad_value(283), 1.0, s.ad_value(1661), -1.0));
            s.store_scaled_mul(45, 1659, 1661, (-4.0));
        }

        if (s.b[1654] && s.b[1712]) {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (s.b[1654] && s.b[1712]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_ad_value(283, A::add_scaled_inputs3(s.ad_value(1659), -1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5)));
            s.store_scale(283, 283, (-1.0));
        }

        s.b[1713] = (s.v[1664] > 0.0);
        s.v[1713] = if s.b[1713] { 1.0 } else { 0.0 };

        if (s.b[1654] && s.b[1713]) {
            s.store_scaled_mul(1660, 544, 1663, ((-1.6021918e-19) * p.p2));
            s.store_scale(1662, 1660, (-0.001));
            s.store_ad_value(44, A::add_scaled_inputs3(s.ad_value(1660), -1.0, s.ad_value(284), 1.0, s.ad_value(1662), -1.0));
            s.store_scaled_mul(45, 1660, 1662, (-4.0));
        }

        if (s.b[1654] && s.b[1713]) {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (s.b[1654] && s.b[1713]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_ad_value(284, A::add_scaled_inputs3(s.ad_value(1660), -1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5)));
            s.store_scale(284, 284, (-1.0));
        }

        s.b[1719] = (s.v[145] == 0.0);
        s.v[1719] = if s.b[1719] { 1.0 } else { 0.0 };

        if ((s.v[85] != 0.0) && s.b[1719]) {
            s.store_scalar(1714, p.p233);
            s.store_scalar(1715, p.p234);
            s.copy_ad(1716, 441);
            s.store_mul_ad_lhs(1717, A::mul3(s.ad_value(1714), s.ad_value(1715), s.ad_value(1716)), 1716);
            s.store_offset_add_ad(1718, A::mul3(s.ad_value(250), s.ad_value(192), s.ad_value(1714)), A::mul3(s.ad_value(1715), s.ad_value(1716), s.ad_value(1716)), 1e-50);
            s.store_div(289, 1717, 1718);
        }

        if ((s.v[85] != 0.0) && (!s.b[1719])) {
            s.store_scalar(289, (p.p233 + 1e-50));
        }

        if (s.v[85] != 0.0) {
            s.store_scalar(1717, p.p235);
            s.store_mul(290, 1717, 323);
        }

        s.b[1727] = ((p.p31 != 0.0) && (s.v[145] == 0.0));
        s.v[1727] = if s.b[1727] { 1.0 } else { 0.0 };

        if s.b[1727] {
            s.store_scalar(1724, s.v[62]);
            s.store_scalar(1725, s.v[63]);
            s.store_scalar(1726, s.v[64]);
            s.store_scale(1720, 244, 6.241449993689894e18);
            s.store_mul_scaled_ad_lhs(1721, A::add_scaled_inputs3(s.ad_value(323), 1.0, A::div(s.ad_value(244), A::sub(s.ad_value(161), s.ad_value(435))), 1.0, s.ad_value(1726), 1.0), 227, 6.241449993689894e18);
            s.store_sub_ad_lhs(1722, A::div(A::div_scaled_inputs(s.ad_value(197), ((-2.0) * 6.241449993689894e18), s.ad_value(442), 1.0), s.ad_value(108)), 1720);
        }

        s.b[1728] = ((((s.v[1722] - s.v[1720])) as f64).abs() > (10.0 * 2.220446049250313e-16));
        s.v[1728] = if s.b[1728] { 1.0 } else { 0.0 };

        if (s.b[1727] && s.b[1728]) {
            let assign33540_ad_e48672: A = A::add_scaled_product(A::div(A::div_from_scalar(1.0, A::add(s.ad_value(1720), s.ad_value(1721))), A::add(s.ad_value(1722), s.ad_value(1721))), 1.0, A::div_scaled_product3(s.ad_value(1724), s.ad_value(252), s.ad_value(250), 2.0, A::sub(s.ad_value(1722), s.ad_value(1720)), 1.0), A::ln(A::div(A::add(s.ad_value(1722), s.ad_value(1721)), A::add(s.ad_value(1720), s.ad_value(1721)))), 1.0);
            s.store_ad_value(1723, A::add_scaled_product(assign33540_ad_e48672, 1.0, A::mul3(A::mul3(s.ad_value(1724), s.ad_value(252), s.ad_value(250)), s.ad_value(1724), s.ad_value(252)), s.ad_value(250), 1.0));
        }

        if (s.b[1727] && (!s.b[1728])) {
            let assign33550_ad_e48723: A = A::add_scaled_inputs_product(A::div(A::div_from_scalar(1.0, A::add(s.ad_value(1720), s.ad_value(1721))), A::add(s.ad_value(1722), s.ad_value(1721))), 1.0, A::div_scaled_product3(s.ad_value(1724), s.ad_value(252), s.ad_value(250), 2.0, A::add(s.ad_value(1720), s.ad_value(1721)), 1.0), 1.0, A::mul3(A::mul3(s.ad_value(1724), s.ad_value(252), s.ad_value(250)), s.ad_value(1724), s.ad_value(252)), s.ad_value(250), 1.0);
            s.store_ad_value(1723, assign33550_ad_e48723);
        }

        if s.b[1727] {
            s.store_mul_ad_lhs(291, A::div_scaled_product(A::square(s.ad_value(199)), s.ad_value(1725), 1.0, A::mul3(s.ad_value(441), s.ad_value(225), s.ad_value(107)), 1.0), 1723);
        }

        if (!s.b[1727]) {
            s.store_scalar(291, 0.0);
        }

        s.b[1746] = ((p.p32 != 0.0) && (s.v[145] == 0.0));
        s.v[1746] = if s.b[1746] { 1.0 } else { 0.0 };

        if s.b[1746] {
            s.store_div_ad_lhs(1729, A::sub(s.ad_value(314), s.ad_value(161)), 441);
            s.store_scaled_mul(1730, 251, 1729, 1e-5);
        }

        s.b[1747] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1747] = if s.b[1747] { 1.0 } else { 0.0 };

        if (s.b[1746] && s.b[1747]) {
            s.store_scalar(1731, 1.0);
        }

        s.b[1748] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1748] = if s.b[1748] { 1.0 } else { 0.0 };

        if ((s.b[1746] && (!s.b[1747])) && s.b[1748]) {
            s.copy_ad(1731, 1730);
        }

        if ((s.b[1746] && (!s.b[1747])) && (!s.b[1748])) {
            s.store_powf(1731, 1730, (p.p113 - 1.0));
        }

        if s.b[1746] {
            s.store_mul(1732, 1730, 1731);
            s.store_offset(1733, 1732, 1.0);
            s.store_powf(1734, 1733, (((-1.0) / p.p113) - 1.0));
            s.store_mul(1735, 1733, 1734);
            s.store_mul(293, 251, 1735);
            s.store_scaled_add(1737, 250, 293, 0.5);
            s.store_square(1736, 190);
        }

    }

    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1746] {
            let assign33730_ad_e48923: A = A::add_scaled_inputs3(A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(190), 3.0, 1.0), 1.0, s.ad_value(1736), 6.0), s.ad_value(293), s.ad_value(293)), 1.0, A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(190), 4.0, 3.0), 1.0, s.ad_value(1736), 3.0), s.ad_value(293), s.ad_value(250)), 1.0, A::mul3(A::add(A::scale_offset(s.ad_value(190), 3.0, 6.0), s.ad_value(1736)), s.ad_value(250), s.ad_value(250)), 1.0);
            s.store_ad_value(292, A::div_scaled_product3_by_product(A::mul3(s.ad_value(107), s.ad_value(323), s.ad_value(192)), s.ad_value(250), assign33730_ad_e48923, 1.0, A::mul3_scaled_output(s.ad_value(441), A::offset(s.ad_value(190), 1.0), s.ad_value(1737), 15.0), s.ad_value(1737), 1.0));
        }

        if (!s.b[1746]) {
            s.store_scalar(292, 0.0);
        }

        s.b[1749] = ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (s.v[146] == 1.0)) && (s.v[145] == 0.0));
        s.v[1749] = if s.b[1749] { 1.0 } else { 0.0 };

        if s.b[1749] {
            s.store_sqrt(298, 296);
            s.store_add(1738, 192, 298);
            s.store_square(1739, 294);
            s.store_square(1740, 296);
            s.store_scaled_mul(1741, 294, 296, 42.0);
            s.store_ad_value(1741, A::add_scaled_inputs3(s.ad_value(1741), 1.0, s.ad_value(1739), 4.0, s.ad_value(1740), 4.0));
            s.store_add_ad_rhs(1741, 1741, A::mul3_scaled_output(s.ad_value(298), s.ad_value(192), A::add(s.ad_value(294), s.ad_value(296)), 20.0));
            s.store_square(1742, 1738);
            s.store_square(1734, 1742);
            s.store_div_ad_rhs(299, 1741, A::mul(s.ad_value(1734), s.ad_value(1738)));
            s.store_mul_ad_product_lhs(300, A::div(s.ad_value(107), s.ad_value(441)), s.ad_value(250), 323);
            s.store_mul(1744, 300, 192);
            s.store_div(1745, 292, 1744);
            s.store_add_ad_lhs(1743, A::add_scaled_product(s.ad_value(294), 1.0, s.ad_value(192), s.ad_value(298), 4.0), 296);
            s.store_ad_value(301, A::div_scaled_product_by_product(s.ad_value(297), s.ad_value(1743), 3.872983346207417, s.ad_value(1738), A::sqrt(A::mul(A::mul3(s.ad_value(1745), s.ad_value(1738), s.ad_value(192)), s.ad_value(1741))), 6.0));
        }

        s.store_add(199, 199, 265);

        s.b[1750] = (p.p43 == 1.0);
        s.v[1750] = if s.b[1750] { 1.0 } else { 0.0 };

        if s.b[1750] {
            s.store_add(271, 531, 532);
        }

        if (s.b[1750] && s.b[564]) {
            s.store_offset(271, 271, (-(p.p168 * s.v[99])));
        }

        if s.b[1750] {
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

        if ((!s.b[1750]) && s.b[564]) {
            s.store_scalar(271, ((-p.p168) * s.v[99]));
            s.store_mul_scaled_ad_rhs(272, 271, -1.0, A::sub(s.ad_value(158), s.ad_value(513)));
        }

        if ((!s.b[1750]) && (!s.b[564])) {
            s.store_scalar(271, 0.0);
            s.store_scalar(272, 0.0);
        }

        if (!s.b[1750]) {
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

        s.b[1751] = (p.p43 == 1.0);
        s.v[1751] = if s.b[1751] { 1.0 } else { 0.0 };

        if ((s.v[85] != 0.0) && s.b[1751]) {
            s.store_scalar(25, 0.0);
            s.copy_ad(556, 438);
            s.store_scale(588, 196, s.v[451]);
            s.store_scale(587, 197, s.v[451]);
        }

        if ((s.v[85] != 0.0) && (!s.b[1751])) {
            s.store_scalar(554, 0.0);
            s.store_scale(588, 392, s.v[451]);
            s.store_scaled_add(576, 198, 477, s.v[451]);
            s.store_ad_value(577, A::add_scaled_inputs3(s.ad_value(197), s.v[451], s.ad_value(198), ((-1.0) * s.v[451]), s.ad_value(476), s.v[451]));
        }

        s.b[1752] = (p.p43 == 1.0);
        s.v[1752] = if s.b[1752] { 1.0 } else { 0.0 };

        if ((s.v[85] == 0.0) && s.b[1752]) {
            s.store_sub_scaled_inputs(23, 196, (-s.v[451]), 197, s.v[451]);
            s.store_scale(24, 198, s.v[451]);
            s.store_scaled_sub(25, 197, 198, s.v[451]);
        }

        if ((s.v[85] == 0.0) && (!s.b[1752])) {
            s.store_scaled_sub_ad_lhs(23, A::add_scaled_inputs3(s.ad_value(392), -1.0, s.ad_value(197), (-1.0), s.ad_value(476), -1.0), 477, s.v[451]);
            s.store_scaled_add(24, 198, 477, s.v[451]);
            s.store_ad_value(25, A::add_scaled_inputs3(s.ad_value(197), s.v[451], s.ad_value(198), ((-1.0) * s.v[451]), s.ad_value(476), s.v[451]));
        }

        s.b[1758] = (p.p64 == 0.0);
        s.v[1758] = if s.b[1758] { 1.0 } else { 0.0 };

        if s.b[1758] {
            s.store_scalar(280, 0.0);
        }

        if (!s.b[1758]) {
            s.store_add_scaled_inputs(1753, 315, s.v[97], 161, 1.0);
        }

        s.b[1759] = (s.v[1753] > s.v[314]);
        s.v[1759] = if s.b[1759] { 1.0 } else { 0.0 };

        if ((!s.b[1758]) && s.b[1759]) {
            s.copy_ad(1753, 314);
        }

        if (!s.b[1758]) {
            s.store_ad_value(1754, A::add_scaled_inputs3(s.ad_value(157), s.v[317], s.ad_value(161), s.v[317], s.ad_value(1753), (1.0 - s.v[317])));
            s.store_sqrt_div_from_scalar_ad(1755, (2.0 * 1.034943e-10), s.ad_value(229));
            s.store_scale(1756, 1755, 1.3);
            s.store_scaled_mul(1757, 108, 1756, 1.034943e-10);
            s.store_mul_sub_ad_lhs(280, A::add_scaled_inputs3(s.ad_value(161), 1.0 / (p.p64), s.ad_value(157), 1.0 / (p.p64), s.ad_value(1754), (-1.0 / (p.p64))), s.ad_value(315), 1757);
        }

        s.b[1760] = (p.p65 != 0.0);
        s.v[1760] = if s.b[1760] { 1.0 } else { 0.0 };

        if s.b[1760] {
            s.store_ad_value(280, A::add_scaled_product(s.ad_value(280), 1.0, s.ad_value(135), s.ad_value(513), 1.0));
        }

        s.b[1761] = (p.p24 == 1.0);
        s.v[1761] = if s.b[1761] { 1.0 } else { 0.0 };

        s.b[1762] = (p.p43 == 1.0);
        s.v[1762] = if s.b[1762] { 1.0 } else { 0.0 };

        if (s.b[1761] && s.b[1762]) {
            s.store_sub_ad_lhs(471, A::add_scaled_inputs3(s.ad_value(463), -1.0, s.ad_value(464), (-1.0), s.ad_value(467), -1.0), 468);
            s.store_add(472, 466, 470);
            s.store_add(473, 465, 469);
            s.store_add_ad_rhs(23, 23, A::add_scaled_inputs3(A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(268), 1.0, s.ad_value(267), 1.0, s.ad_value(272), 1.0), 1.0, s.ad_value(280), (-1.0), s.ad_value(455), -1.0), s.v[451], s.ad_value(454), ((-1.0) * s.v[451]), s.ad_value(471), s.v[451]));
            s.store_ad_value(24, A::add_scaled_inputs3(s.ad_value(24), 1.0, A::add_scaled_inputs3(s.ad_value(280), 1.0, s.ad_value(268), (-1.0), s.ad_value(456), 1.0), s.v[451], s.ad_value(472), s.v[451]));
            s.store_add_ad_rhs(25, 25, A::add_scaled_inputs3(s.ad_value(457), s.v[451], s.ad_value(267), ((-1.0) * s.v[451]), s.ad_value(473), s.v[451]));
        }

        if (s.b[1761] && (!s.b[1762])) {
            s.store_ad_value(23, A::add_scaled_inputs3(s.ad_value(23), 1.0, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(268), 1.0, s.ad_value(267), 1.0, s.ad_value(272), 1.0), 1.0, s.ad_value(280), (-1.0), s.ad_value(455), -1.0), s.v[451], s.ad_value(454), (-s.v[451])));
            s.store_add_ad_rhs(24, 24, A::add_scaled_inputs3(s.ad_value(280), s.v[451], s.ad_value(268), ((-1.0) * s.v[451]), s.ad_value(456), s.v[451]));
            s.store_ad_value(25, A::add_scaled_inputs3(s.ad_value(25), 1.0, s.ad_value(457), s.v[451], s.ad_value(267), (-s.v[451])));
        }

        s.b[1763] = (p.p43 == 1.0);
        s.v[1763] = if s.b[1763] { 1.0 } else { 0.0 };

        if s.b[1763] {
            s.store_scale(36, 281, s.v[451]);
            s.store_scale(35, 282, s.v[451]);
            s.store_scale(560, 284, s.v[451]);
            s.store_scale(561, 283, s.v[451]);
        }

        if (!s.b[1763]) {
            s.store_scalar(36, 0.0);
            s.store_scalar(35, 0.0);
            s.store_scalar(560, 0.0);
            s.store_scalar(561, 0.0);
        }

        s.b[1764] = (p.p25 != 1.0);
        s.v[1764] = if s.b[1764] { 1.0 } else { 0.0 };

        if s.b[1764] {
            s.store_scalar(557, 0.0);
        }

        if (!s.b[1764]) {
            s.store_scale(557, 263, s.v[451]);
        }

        s.store_scale(15, 308, (-s.v[451]));

        s.b[1765] = (s.v[613] == 1.0);
        s.v[1765] = if s.b[1765] { 1.0 } else { 0.0 };

        if s.b[1765] {
            s.store_ad_value(13, A::add_scaled_product(s.ad_value(307), ((-1.0) * s.v[451]), s.ad_value(310), s.ad_value(309), s.v[451]));
        }

        if (!s.b[1765]) {
            s.store_scaled_sub_ad_lhs(13, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(310), s.ad_value(309)), 306, s.v[451]);
        }

        s.b[1766] = (s.v[613] == 1.0);
        s.v[1766] = if s.b[1766] { 1.0 } else { 0.0 };

        if s.b[1766] {
            s.store_scaled_sub_ad_lhs(14, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(310), s.ad_value(309)), 306, s.v[451]);
        }

        if (!s.b[1766]) {
            s.store_ad_value(14, A::add_scaled_product(s.ad_value(307), ((-1.0) * s.v[451]), s.ad_value(310), s.ad_value(309), s.v[451]));
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

        s.b[1773] = ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (s.v[146] == 1.0)) && (s.v[145] == 0.0));
        s.v[1773] = if s.b[1773] { 1.0 } else { 0.0 };

        if s.b[1773] {
            s.store_scaled_mul(1767, 323, 108, (1e-6 * s.v[98]));
            s.store_scale(1768, 555, 1.0 / (s.v[451]));
            s.store_ad_value(1769, A::div_scaled_product3(s.ad_value(227), s.ad_value(1768), s.ad_value(1768), (0.1185185185185185 * 1.6021918e-19), s.ad_value(300), 1.0));
        }

        s.b[1774] = ((s.v[297] > (10.0 * 2.220446049250313e-16)) && (s.v[157] > (10.0 * 2.220446049250313e-16)));
        s.v[1774] = if s.b[1774] { 1.0 } else { 0.0 };

        if (s.b[1773] && s.b[1774]) {
            s.store_div(1770, 251, 250);
            s.store_div_ad_lhs(1771, A::sub(A::div(s.ad_value(251), s.ad_value(293)), s.ad_value(1770)), 157);
            s.store_add_ad_rhs(1772, 1770, A::div_scaled_product(s.ad_value(1771), A::add(A::add_scaled_product(s.ad_value(294), 1.0, s.ad_value(192), s.ad_value(298), 1.0), s.ad_value(296)), 0.6666666666666667, A::add(s.ad_value(192), s.ad_value(298)), 1.0));
        }

        if (s.b[1773] && (!s.b[1774])) {
            s.store_div(1772, 251, 293);
        }

        if s.b[1773] {
            s.store_mul3_affine_lhs(558, 1769, 299, s.v[451], 0.0, 1772);
            s.copy_ad(559, 301);
        }

        if s.b[1773] {
            s.store_ad_value(558, {
                if (((-s.v[1768]) > s.v[1767]) && (s.v[558] > 0.0)) {
                    s.ad_value(558)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if s.b[1773] {
            s.store_ad_value(559, {
                if ((-s.v[1768]) > s.v[1767]) {
                    s.ad_value(559)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (!s.b[1773]) {
            s.store_scalar(558, 0.0);
            s.store_scalar(559, 0.0);
        }

        s.v[4] = 0.0;

        s.v[5] = 0.0;

        s.v[7] = 0.0;

        s.v[8] = 0.0;

        s.b[1775] = (p.p259 == 1.0);
        s.v[1775] = if s.b[1775] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_31(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1775] {
            s.store_scalar(3, 1.0);
        }

        s.b[1795] = (s.v[3] == 1.0);
        s.v[1795] = if s.b[1795] { 1.0 } else { 0.0 };

        if (s.b[1775] && s.b[1795]) {
            s.store_scalar(1786, (p.p264 / 1e-6));
            s.store_scalar(1779, p.p266);
            s.store_scalar(1780, p.p268);
            s.store_scalar(1781, p.p273);
        }

        if (s.b[1775] && s.b[1795]) {
            s.store_scalar(1782, (if (p.p263 > 0.0) { (p.p263 * p.p255) } else { 0.0 }));
        }

        if (s.b[1775] && s.b[1795]) {
            s.store_scalar(1785, p.p258);
            s.store_scaled_voltage(1783, ctx, nodes, Some(7), Some(2), p.p50);
        }

        if (s.b[1775] && (!s.b[1795])) {
            s.store_scalar(1786, (p.p59 / 1e-6));
            s.store_scalar(1779, p.p265);
            s.store_scalar(1780, p.p267);
            s.store_scalar(1781, p.p272);
        }

        if (s.b[1775] && (!s.b[1795])) {
            s.store_scalar(1782, (if (p.p263 > 0.0) { (p.p263 * p.p256) } else { 0.0 }));
        }

        if (s.b[1775] && (!s.b[1795])) {
            s.store_scalar(1785, p.p257);
            s.store_scaled_voltage(1783, ctx, nodes, Some(0), Some(6), p.p50);
        }

        if s.b[1775] {
            s.store_scalar(1792, ((((p.p271 * p.p271) + (p.p56 * p.p56))) as f64).sqrt());
            s.store_scale(1794, 105, p.p9);
            s.store_scale(1779, 1779, 0.0001);
            s.store_scale(1780, 1780, 0.01);
            s.store_scale(1784, 429, 1.0 / (s.v[81]));
            s.store_powf(328, 1784, p.p269);
            s.store_div(1787, 1779, 328);
            s.store_ad_value(327, A::sub_scaled_inputs(A::add_scaled_product(A::scale_offset(s.ad_value(1784), 0.4, 1.8), 1.0, s.ad_value(1784), s.ad_value(1784), 0.1), 1.0, A::sub_from_scalar(1.0, s.ad_value(1784)), p.p270));
            s.store_div(1788, 1780, 327);
            s.store_add_ad_rhs(1781, 1781, A::scaled_offset(s.ad_value(429), (-s.v[81]), p.p274));
            s.store_scalar(1776, (1.0 + (p.p279 / ((s.v[100]) as f64).powf(p.p280))));
            s.store_scalar(1778, (1.0 + (p.p277 / ((s.v[100]) as f64).powf(p.p278))));
            s.store_scalar(1777, (1.0 + (p.p275 / ((s.v[109]) as f64).powf(p.p276))));
            s.store_mul(1787, 1787, 1776);
            s.store_offset_ad(1788, A::mul3(s.ad_value(1788), s.ad_value(1777), s.ad_value(1778)), 1e-50);
            s.store_div(1789, 1783, 1785);
            s.store_mul(1790, 1787, 1789);
        }

        s.b[1796] = (s.v[1783] >= 0.0);
        s.v[1796] = if s.b[1796] { 1.0 } else { 0.0 };

        if (s.b[1775] && s.b[1796]) {
            s.store_div(328, 1790, 1788);
        }

        if (s.b[1775] && (!s.b[1796])) {
            s.store_scaled_div(328, 1790, 1788, -1.0);
        }

        s.b[1797] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1781]) && (s.v[1781] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1797] = if s.b[1797] { 1.0 } else { 0.0 };

        if (s.b[1775] && s.b[1797]) {
            s.store_scalar(330, 1.0);
        }

        s.b[1798] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1781]) && (s.v[1781] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1798] = if s.b[1798] { 1.0 } else { 0.0 };

        if ((s.b[1775] && (!s.b[1797])) && s.b[1798]) {
            s.copy_ad(330, 328);
        }

        if ((s.b[1775] && (!s.b[1797])) && (!s.b[1798])) {
            s.store_pow_ad(330, s.ad_value(328), A::offset(s.ad_value(1781), (-1.0)));
        }

        if s.b[1775] {
            s.store_mul(329, 328, 330);
            s.store_offset(331, 329, 1.0);
        }

        s.b[1799] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1781]) && (s.v[1781] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1799] = if s.b[1799] { 1.0 } else { 0.0 };

        if (s.b[1775] && s.b[1799]) {
            s.store_div_from_scalar(332, 1.0, 331);
        }

        s.b[1800] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1781]) && (s.v[1781] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1800] = if s.b[1800] { 1.0 } else { 0.0 };

        if ((s.b[1775] && (!s.b[1799])) && s.b[1800]) {
            s.store_div_from_scalar_sqrt_ad(332, 1.0, s.ad_value(331));
        }

        if ((s.b[1775] && (!s.b[1799])) && (!s.b[1800])) {
            s.store_pow_ad(333, s.ad_value(331), A::offset(A::div_from_scalar((-1.0), s.ad_value(1781)), (-1.0)));
            s.store_mul(332, 331, 333);
        }

        if s.b[1775] {
            s.store_mul(1791, 1787, 332);
            s.store_div_from_scalar(328, 1.6021918e-19, 1785);
            s.store_mul_ad_lhs(1793, A::mul3(s.ad_value(328), s.ad_value(1792), s.ad_value(1791)), 1786);
        }

        s.b[1801] = (s.v[1793] <= 0.0);
        s.v[1801] = if s.b[1801] { 1.0 } else { 0.0 };

        if (s.b[1775] && s.b[1801]) {
            s.store_scalar(1793, 1e-50);
        }

        if s.b[1775] {
            s.store_div_from_scalar(1, 1.0, 1793);
            s.store_div(1, 1, 1794);
            s.store_add(1, 1, 1782);
        }

        if s.b[1775] {
            s.store_ad_value(6, {
                if ((s.v[1] > 0.0001) && (p.p32 != 0.0)) {
                    A::div_from_scalar(s.v[451], s.ad_value(1))
                } else {
                    A::constant(0.0)
                }
            });
        }

        s.b[1802] = (s.v[1] < 0.0001);
        s.v[1802] = if s.b[1802] { 1.0 } else { 0.0 };

        if (s.b[1775] && s.b[1802]) {
            s.store_scalar(1, 0.0001);
        }

        if s.b[1775] {
            s.store_scale(5, 1, 1.0 / (s.v[451]));
            s.copy_ad(8, 6);
        }

        s.b[1803] = (p.p260 == 1.0);
        s.v[1803] = if s.b[1803] { 1.0 } else { 0.0 };

        if s.b[1803] {
            s.store_scalar(3, 2.0);
        }

        s.b[1823] = (s.v[3] == 1.0);
        s.v[1823] = if s.b[1823] { 1.0 } else { 0.0 };

        if (s.b[1803] && s.b[1823]) {
            s.store_scalar(1814, (p.p264 / 1e-6));
            s.store_scalar(1807, p.p266);
            s.store_scalar(1808, p.p268);
            s.store_scalar(1809, p.p273);
        }

        if (s.b[1803] && s.b[1823]) {
            s.store_scalar(1810, (if (p.p263 > 0.0) { (p.p263 * p.p255) } else { 0.0 }));
        }

        if (s.b[1803] && s.b[1823]) {
            s.store_scalar(1813, p.p258);
            s.store_scaled_voltage(1811, ctx, nodes, Some(7), Some(2), p.p50);
        }

        if (s.b[1803] && (!s.b[1823])) {
            s.store_scalar(1814, (p.p59 / 1e-6));
            s.store_scalar(1807, p.p265);
            s.store_scalar(1808, p.p267);
            s.store_scalar(1809, p.p272);
        }

        if (s.b[1803] && (!s.b[1823])) {
            s.store_scalar(1810, (if (p.p263 > 0.0) { (p.p263 * p.p256) } else { 0.0 }));
        }

        if (s.b[1803] && (!s.b[1823])) {
            s.store_scalar(1813, p.p257);
            s.store_scaled_voltage(1811, ctx, nodes, Some(0), Some(6), p.p50);
        }

        if s.b[1803] {
            s.store_scalar(1820, ((((p.p271 * p.p271) + (p.p56 * p.p56))) as f64).sqrt());
            s.store_scale(1822, 105, p.p9);
            s.store_scale(1807, 1807, 0.0001);
            s.store_scale(1808, 1808, 0.01);
            s.store_scale(1812, 429, 1.0 / (s.v[81]));
            s.store_powf(328, 1812, p.p269);
            s.store_div(1815, 1807, 328);
            s.store_ad_value(327, A::sub_scaled_inputs(A::add_scaled_product(A::scale_offset(s.ad_value(1812), 0.4, 1.8), 1.0, s.ad_value(1812), s.ad_value(1812), 0.1), 1.0, A::sub_from_scalar(1.0, s.ad_value(1812)), p.p270));
            s.store_div(1816, 1808, 327);
            s.store_add_ad_rhs(1809, 1809, A::scaled_offset(s.ad_value(429), (-s.v[81]), p.p274));
            s.store_scalar(1804, (1.0 + (p.p279 / ((s.v[100]) as f64).powf(p.p280))));
            s.store_scalar(1806, (1.0 + (p.p277 / ((s.v[100]) as f64).powf(p.p278))));
            s.store_scalar(1805, (1.0 + (p.p275 / ((s.v[109]) as f64).powf(p.p276))));
            s.store_mul(1815, 1815, 1804);
            s.store_offset_ad(1816, A::mul3(s.ad_value(1816), s.ad_value(1805), s.ad_value(1806)), 1e-50);
            s.store_div(1817, 1811, 1813);
            s.store_mul(1818, 1815, 1817);
        }

        s.b[1824] = (s.v[1811] >= 0.0);
        s.v[1824] = if s.b[1824] { 1.0 } else { 0.0 };

        if (s.b[1803] && s.b[1824]) {
            s.store_div(328, 1818, 1816);
        }

        if (s.b[1803] && (!s.b[1824])) {
            s.store_scaled_div(328, 1818, 1816, -1.0);
        }

        s.b[1825] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1809]) && (s.v[1809] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1825] = if s.b[1825] { 1.0 } else { 0.0 };

        if (s.b[1803] && s.b[1825]) {
            s.store_scalar(330, 1.0);
        }

        s.b[1826] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1809]) && (s.v[1809] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1826] = if s.b[1826] { 1.0 } else { 0.0 };

        if ((s.b[1803] && (!s.b[1825])) && s.b[1826]) {
            s.copy_ad(330, 328);
        }

        if ((s.b[1803] && (!s.b[1825])) && (!s.b[1826])) {
            s.store_pow_ad(330, s.ad_value(328), A::offset(s.ad_value(1809), (-1.0)));
        }

        if s.b[1803] {
            s.store_mul(329, 328, 330);
            s.store_offset(331, 329, 1.0);
        }

        s.b[1827] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1809]) && (s.v[1809] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1827] = if s.b[1827] { 1.0 } else { 0.0 };

        if (s.b[1803] && s.b[1827]) {
            s.store_div_from_scalar(332, 1.0, 331);
        }

        s.b[1828] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1809]) && (s.v[1809] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1828] = if s.b[1828] { 1.0 } else { 0.0 };

        if ((s.b[1803] && (!s.b[1827])) && s.b[1828]) {
            s.store_div_from_scalar_sqrt_ad(332, 1.0, s.ad_value(331));
        }

        if ((s.b[1803] && (!s.b[1827])) && (!s.b[1828])) {
            s.store_pow_ad(333, s.ad_value(331), A::offset(A::div_from_scalar((-1.0), s.ad_value(1809)), (-1.0)));
            s.store_mul(332, 331, 333);
        }

        if s.b[1803] {
            s.store_mul(1819, 1815, 332);
            s.store_div_from_scalar(328, 1.6021918e-19, 1813);
            s.store_mul_ad_lhs(1821, A::mul3(s.ad_value(328), s.ad_value(1820), s.ad_value(1819)), 1814);
        }

        s.b[1829] = (s.v[1821] <= 0.0);
        s.v[1829] = if s.b[1829] { 1.0 } else { 0.0 };

        if (s.b[1803] && s.b[1829]) {
            s.store_scalar(1821, 1e-50);
        }

        if s.b[1803] {
            s.store_div_from_scalar(1, 1.0, 1821);
            s.store_div(1, 1, 1822);
            s.store_add(1, 1, 1810);
        }

        if s.b[1803] {
            s.store_ad_value(6, {
                if ((s.v[1] > 0.0001) && (p.p32 != 0.0)) {
                    A::div_from_scalar(s.v[451], s.ad_value(1))
                } else {
                    A::constant(0.0)
                }
            });
        }

        s.b[1830] = (s.v[1] < 0.0001);
        s.v[1830] = if s.b[1830] { 1.0 } else { 0.0 };

        if (s.b[1803] && s.b[1830]) {
            s.store_scalar(1, 0.0001);
        }

        if s.b[1803] {
            s.store_scale(4, 1, 1.0 / (s.v[451]));
            s.copy_ad(7, 6);
        }

        s.b[1831] = (p.p43 == 1.0);
        s.v[1831] = if s.b[1831] { 1.0 } else { 0.0 };

        s.b[1832] = (s.v[289] < (1e-15 / 0.0001));
        s.v[1832] = if s.b[1832] { 1.0 } else { 0.0 };

        if ((s.b[1831] && (s.v[85] != 0.0)) && s.b[1832]) {
            s.store_scalar(289, (1e-15 / 0.0001));
        }

        s.b[1833] = (s.v[290] < (1e-15 / 0.0001));
        s.v[1833] = if s.b[1833] { 1.0 } else { 0.0 };

        if ((s.b[1831] && (s.v[85] != 0.0)) && s.b[1833]) {
            s.store_scalar(290, (1e-15 / 0.0001));
        }

    }
}
