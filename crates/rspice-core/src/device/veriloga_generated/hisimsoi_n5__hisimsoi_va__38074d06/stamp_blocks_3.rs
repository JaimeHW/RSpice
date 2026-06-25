#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_block_16(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1126] != 0.0)) && (!(s.v[1127] != 0.0))) {
            s.copy_ad(436, 425);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1126] != 0.0))) {
            s.store_scaled_add(436, 355, 358, (-0.5));
        }

        s.v[1131] = if (s.v[612] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1131] != 0.0) {
            s.store_offset(480, 190, 0.5);
        }

        if (s.v[1131] != 0.0) {
            s.store_mul(481, 479, 478);
        }

        if (s.v[1131] != 0.0) {
            s.store_div_ad_lhs(482, A::scale(s.ad_value(480), 0.4), 481);
        }

        if (s.v[1131] != 0.0) {
            s.store_sub_from_scalar(438, 0.6, 482);
        }

        s.v[1132] = if (s.v[438] > (0.5 + 1e-8)) { 1.0 } else { 0.0 };

        if ((s.v[1131] != 0.0) && (s.v[1132] != 0.0)) {
            s.store_scalar(438, 0.5);
        }

        if (s.v[1131] != 0.0) {
            s.copy_ad(439, 438);
        }

        if (s.v[1131] != 0.0) {
            s.store_scalar(438, 0.5);
        }

        s.v[1134] = if (s.v[145] == 0.0) { 1.0 } else { 0.0 };

        s.v[1150] = if ((p.p190 < (10.0 * 2.220446049250313e-16)) && (p.p191 < (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if (((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (s.v[1150] != 0.0)) {
            s.store_scalar(316, 0.0);
        }

        if (((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (s.v[1150] != 0.0)) {
            s.copy_ad(314, 162);
        }

        s.v[1151] = if (s.v[314] > ((s.v[161] + s.v[173]) - (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if ((((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (s.v[1150] != 0.0)) && (s.v[1151] != 0.0)) {
            s.store_offset_ad(314, A::add(s.ad_value(161), s.ad_value(173)), (-(10.0 * 2.220446049250313e-16)));
        }

        if (((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (!(s.v[1150] != 0.0))) {
            s.store_scalar(1149, (if (p.p43 == 1.0) { p.p237 } else { s.v[402] }));
        }

        if (((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (!(s.v[1150] != 0.0))) {
            s.store_div_from_scalar(1135, 1.0, 1149);
        }

        if (((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (!(s.v[1150] != 0.0))) {
            s.store_mul(1136, 244, 1135);
        }

        if (((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (!(s.v[1150] != 0.0))) {
            s.store_scale(1137, 1136, p.p191);
        }

        if (((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (!(s.v[1150] != 0.0))) {
            s.store_add_ad_lhs(1140, A::mul(s.ad_value(80), s.ad_value(229)), 1137);
        }

        if (((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (!(s.v[1150] != 0.0))) {
            s.store_div_from_scalar(1136, 1.0, 1140);
        }

        if (((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (!(s.v[1150] != 0.0))) {
            s.store_scale(1139, 1136, 1.034943e-10);
        }

        if (((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (!(s.v[1150] != 0.0))) {
            s.store_scalar(1136, (1.0 - p.p189));
        }

        if (((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (!(s.v[1150] != 0.0))) {
            s.store_add_ad(314, A::scale(A::add(s.ad_value(157), s.ad_value(161)), p.p189), A::mul(s.ad_value(1136), s.ad_value(162)));
        }

        s.v[1152] = if (s.v[314] > ((s.v[161] + s.v[173]) - (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if ((((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (!(s.v[1150] != 0.0))) && (s.v[1152] != 0.0)) {
            s.store_offset_ad(314, A::add(s.ad_value(161), s.ad_value(173)), (-(10.0 * 2.220446049250313e-16)));
        }

        if (((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (!(s.v[1150] != 0.0))) {
            s.store_sub(1142, 314, 162);
        }

        if (((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (!(s.v[1150] != 0.0))) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1142)), ((4.0 * 0.001) * 0.001)));
        }

        if (((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (!(s.v[1150] != 0.0))) {
            s.store_offset_ad(1141, A::scale(A::add(s.ad_value(1142), s.ad_value(44)), 0.5), (1e-10 * 0.001));
        }

        s.v[1153] = if (s.v[1141] < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (!(s.v[1150] != 0.0))) && (s.v[1153] != 0.0)) {
            s.store_scalar(1141, 0.0);
        }

        if (((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (!(s.v[1150] != 0.0))) {
            s.store_mul(1138, 225, 244);
        }

        if (((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (!(s.v[1150] != 0.0))) {
            s.store_div_from_scalar(1136, 1.0, 1138);
        }

        if (((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (!(s.v[1150] != 0.0))) {
            s.store_mul(1140, 246, 1136);
        }

        s.v[1154] = if (s.v[1140] < s.v[227]) { 1.0 } else { 0.0 };

        if ((((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (!(s.v[1150] != 0.0))) && (s.v[1154] != 0.0)) {
            s.copy_ad(1140, 227);
        }

        if (((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (!(s.v[1150] != 0.0))) {
            s.store_scale(1146, 229, 9662367879.197212);
        }

        if (((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (!(s.v[1150] != 0.0))) {
            s.store_scalar(1136, (100000.0 * 10000.0));
        }

        if (((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (!(s.v[1150] != 0.0))) {
            s.store_scalar(1137, (1.0 / s.v[97]));
        }

        if (((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (!(s.v[1150] != 0.0))) {
            s.store_mul_ad_lhs(1148, A::add(A::add(A::scale(s.ad_value(1140), 2.0), A::mul(A::mul(A::scale(s.ad_value(1146), 2.0), s.ad_value(1141)), s.ad_value(1139))), A::mul(s.ad_value(1136), s.ad_value(1139))), 1137);
        }

        if (((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (!(s.v[1150] != 0.0))) {
            s.store_mul(1143, 1148, 1139);
        }

        if (((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (!(s.v[1150] != 0.0))) {
            s.store_scale_ad(1147, A::add(A::mul(A::scale(s.ad_value(1146), 2.0), s.ad_value(1141)), s.ad_value(1136)), 4.0);
        }

        if (((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (!(s.v[1150] != 0.0))) {
            s.store_mul_ad_lhs(1144, A::mul(s.ad_value(1147), s.ad_value(1139)), 1139);
        }

        if (((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (!(s.v[1150] != 0.0))) {
            s.store_sqrt_ad(1145, A::add(A::square(s.ad_value(1143)), s.ad_value(1144)));
        }

        if (((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) && (!(s.v[1150] != 0.0))) {
            s.store_mul_ad_rhs(316, 326, A::scale(A::sub(s.ad_value(1145), s.ad_value(1143)), 0.5));
        }

        if ((s.v[1131] != 0.0) && (s.v[1134] != 0.0)) {
            s.store_scale(316, 316, s.v[127]);
        }

        if (s.v[1131] != 0.0) {
            s.store_sub_from_scalar(441, s.v[97], 316);
        }

        s.v[1155] = if (s.v[441] < 1e-9) { 1.0 } else { 0.0 };

        if ((s.v[1131] != 0.0) && (s.v[1155] != 0.0)) {
            s.store_scalar(441, 1e-9);
        }

        if (s.v[1131] != 0.0) {
            s.store_scale_ad(328, A::neg(s.ad_value(108)), s.v[98]);
        }

        if (s.v[1131] != 0.0) {
            s.store_mul(196, 328, 437);
        }

        if (s.v[1131] != 0.0) {
            s.store_mul(197, 328, 436);
        }

        if (s.v[1131] != 0.0) {
            s.store_mul(198, 197, 438);
        }

        s.v[1156] = if (p.p43 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1131] != 0.0) && (s.v[1156] != 0.0)) {
            s.store_scale(477, 196, 0.5);
        }

        if ((s.v[1131] != 0.0) && (s.v[1156] != 0.0)) {
            s.store_scale(476, 196, (1.0 - 0.5));
        }

        if ((s.v[1131] != 0.0) && (s.v[1156] != 0.0)) {
            s.store_mul_ad_lhs(392, A::scale(A::add(s.ad_value(357), s.ad_value(360)), (0.5 * s.v[98])), 108);
        }

        if (s.v[1131] != 0.0) {
            s.store_scaled_sub(1157, 157, 164, 0.5);
        }

        if (s.v[1131] != 0.0) {
            s.store_scale(44, 1157, (2.0 * 1.0 / (p.p227)));
        }

        if (s.v[1131] != 0.0) {
            s.store_offset_ad(45, A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::scale(s.ad_value(44), (1.0 / 5040.0)), (1.0 / 720.0))), (1.0 / 120.0))), (1.0 / 24.0))), (1.0 / 6.0))), (1.0 / 2.0))), 1.0);
        }

        if (s.v[1131] != 0.0) {
            s.store_div_from_scalar(177, p.p227, 45);
        }

        s.v[1158] = if (s.v[177] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if ((s.v[1131] != 0.0) && (s.v[1158] != 0.0)) {
            s.store_scalar(177, (10.0 * 2.220446049250313e-16));
        }

        if (s.v[1131] != 0.0) {
            s.store_add(176, 161, 177);
        }

        if (s.v[1131] != 0.0) {
            s.store_scalar(1168, (1.034943e-10 / 100.0));
        }

        if (s.v[1131] != 0.0) {
            s.store_scale(1169, 437, 0.0001);
        }

        if (s.v[1131] != 0.0) {
            s.store_scale(1170, 436, 0.0001);
        }

        if (s.v[1131] != 0.0) {
            s.store_div_from_scalar(1159, p.p92, 1168);
        }

        if (s.v[1131] != 0.0) {
            s.store_div_from_scalar(1160, p.p93, 1168);
        }

        if (s.v[1131] != 0.0) {
            s.store_scalar(1161, p.p94);
        }

        if (s.v[1131] != 0.0) {
            s.store_offset_ad(1162, A::mul(A::sub(s.ad_value(162), s.ad_value(161)), s.ad_value(1161)), 1.0);
        }

        if (s.v[1131] != 0.0) {
            s.store_add_ad(1163, A::mul(s.ad_value(1159), s.ad_value(1169)), A::mul(s.ad_value(1160), s.ad_value(1170)));
        }

        if (s.v[1131] != 0.0) {
            s.store_div(1164, 1163, 1162);
        }

        if (s.v[1131] != 0.0) {
            s.copy_ad(248, 1164);
        }

        if (s.v[1131] != 0.0) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(248)), ((4.0 * 3000.0) * 3000.0)));
        }

        if (s.v[1131] != 0.0) {
            s.store_offset_ad(1161, A::scale(A::add(s.ad_value(248), s.ad_value(44)), 0.5), (1e-10 * 3000.0));
        }

        s.v[1171] = if (s.v[1161] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1131] != 0.0) && (s.v[1171] != 0.0)) {
            s.store_scalar(1161, 0.0);
        }

        if (s.v[1131] != 0.0) {
            s.store_powf(1163, 1161, (p.p97 - 1.0));
        }

        if (s.v[1131] != 0.0) {
            s.store_mul(1165, 1163, 1161);
        }

        if (s.v[1131] != 0.0) {
            s.store_powf(1166, 1161, (s.v[111] - 1.0));
        }

        if (s.v[1131] != 0.0) {
            s.store_mul(1167, 1166, 1161);
        }

        if (s.v[1131] != 0.0) {
            s.store_scale(249, 1170, 6.241449993689894e18);
        }

        if (s.v[1131] != 0.0) {
            s.store_add_ad(1159, A::add(A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(249), (p.p96 * 1e-11)), p.p95)), A::mul(s.ad_value(543), s.ad_value(1165))), A::scale(s.ad_value(1167), 1.0 / (p.p106)));
        }

        if (s.v[1131] != 0.0) {
            s.store_div_from_scalar(251, 1.0, 1159);
        }

        if (s.v[1131] != 0.0) {
            s.store_scale(251, 251, 0.0001);
        }

        if (s.v[1131] != 0.0) {
            s.store_mul_ad_lhs(1172, A::mul(s.ad_value(225), s.ad_value(244)), 441);
        }

        if (s.v[1131] != 0.0) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1172)), ((4.0 * 1e-50) * 1e-50)));
        }

        if (s.v[1131] != 0.0) {
            s.store_offset_ad(1172, A::scale(A::add(s.ad_value(1172), s.ad_value(44)), 0.5), (1e-10 * 1e-50));
        }

        s.v[1180] = if (s.v[1172] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1131] != 0.0) && (s.v[1180] != 0.0)) {
            s.store_scalar(1172, 0.0);
        }

        if (s.v[1131] != 0.0) {
            s.store_div_from_scalar(1173, 1.0, 1172);
        }

        if (s.v[1131] != 0.0) {
            s.store_mul(1174, 246, 1173);
        }

        if (s.v[1131] != 0.0) {
            s.store_div_ad_lhs(1172, A::scale(s.ad_value(253), 0.2), 251);
        }

        if (s.v[1131] != 0.0) {
            s.store_sqrt_ad(252, A::add(A::square(s.ad_value(1174)), A::square(s.ad_value(1172))));
        }

        if (s.v[1131] != 0.0) {
            s.store_mul(1175, 251, 252);
        }

        if (s.v[1131] != 0.0) {
            s.store_div(1173, 1175, 253);
        }

        s.v[1181] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1131] != 0.0) && (s.v[1181] != 0.0)) {
            s.store_scalar(1176, 1.0);
        }

        s.v[1182] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1131] != 0.0) && (!(s.v[1181] != 0.0))) && (s.v[1182] != 0.0)) {
            s.copy_ad(1176, 1173);
        }

        if (((s.v[1131] != 0.0) && (!(s.v[1181] != 0.0))) && (!(s.v[1182] != 0.0))) {
            s.store_powf(1176, 1173, (p.p113 - 1.0));
        }

        if (s.v[1131] != 0.0) {
            s.store_mul(1172, 1173, 1176);
        }

        if (s.v[1131] != 0.0) {
            s.store_offset(1177, 1172, 1.0);
        }

        s.v[1183] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1131] != 0.0) && (s.v[1183] != 0.0)) {
            s.store_div_from_scalar(1178, 1.0, 1177);
        }

        s.v[1184] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1131] != 0.0) && (!(s.v[1183] != 0.0))) && (s.v[1184] != 0.0)) {
            s.store_div_from_scalar_ad(1178, 1.0, A::sqrt(s.ad_value(1177)));
        }

        if (((s.v[1131] != 0.0) && (!(s.v[1183] != 0.0))) && (!(s.v[1184] != 0.0))) {
            s.store_powf(1179, 1177, (((-1.0) / p.p113) - 1.0));
        }

        if (((s.v[1131] != 0.0) && (!(s.v[1183] != 0.0))) && (!(s.v[1184] != 0.0))) {
            s.store_mul(1178, 1177, 1179);
        }

        if (s.v[1131] != 0.0) {
            s.store_mul(250, 251, 1178);
        }

        if (s.v[1131] != 0.0) {
            s.store_div_ad(264, A::mul(s.ad_value(107), s.ad_value(227)), A::sub_from_scalar(s.v[97], s.ad_value(316)));
        }

        if (s.v[1131] != 0.0) {
            s.store_mul_ad_lhs(200, A::mul(s.ad_value(264), s.ad_value(246)), 250);
        }

        if (s.v[1131] != 0.0) {
            s.store_scalar(201, 0.0);
        }

        s.v[1194] = if ((p.p281 > 0.0) && (p.p244 != 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1131] != 0.0) && (s.v[1194] != 0.0)) {
            s.store_scaled_sub(1185, 157, 164, 0.5);
        }

        if ((s.v[1131] != 0.0) && (s.v[1194] != 0.0)) {
            s.store_scale(44, 1185, (2.0 * 100.0));
        }

        if ((s.v[1131] != 0.0) && (s.v[1194] != 0.0)) {
            s.store_offset_ad(45, A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::scale(s.ad_value(44), (1.0 / 5040.0)), (1.0 / 720.0))), (1.0 / 120.0))), (1.0 / 24.0))), (1.0 / 6.0))), (1.0 / 2.0))), 1.0);
        }

        if ((s.v[1131] != 0.0) && (s.v[1194] != 0.0)) {
            s.store_div_from_scalar(1191, 0.01, 45);
        }

        if ((s.v[1131] != 0.0) && (s.v[1194] != 0.0)) {
            s.store_sub_from_scalar_ad(1185, 1.1, A::add(s.ad_value(161), s.ad_value(1191)));
        }

        if ((s.v[1131] != 0.0) && (s.v[1194] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1185)), ((4.0 * 0.05) * 0.05)));
        }

        if ((s.v[1131] != 0.0) && (s.v[1194] != 0.0)) {
            s.store_offset_ad(1193, A::scale(A::add(s.ad_value(1185), s.ad_value(44)), 0.5), (1e-10 * 0.05));
        }

        s.v[1195] = if (s.v[1193] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1131] != 0.0) && (s.v[1194] != 0.0)) && (s.v[1195] != 0.0)) {
            s.store_scalar(1193, 0.0);
        }

        if ((s.v[1131] != 0.0) && (s.v[1194] != 0.0)) {
            s.store_scale(1186, 225, s.v[116]);
        }

        if ((s.v[1131] != 0.0) && (s.v[1194] != 0.0)) {
            s.store_mul(1187, 323, 1186);
        }

        if ((s.v[1131] != 0.0) && (s.v[1194] != 0.0)) {
            s.store_powf(1186, 1193, p.p245);
        }

        if ((s.v[1131] != 0.0) && (s.v[1194] != 0.0)) {
            s.store_mul(1188, 1187, 1186);
        }

        if ((s.v[1131] != 0.0) && (s.v[1194] != 0.0)) {
            s.store_offset_scaled(1189, 173, p.p246, 1.0);
        }

        if ((s.v[1131] != 0.0) && (s.v[1194] != 0.0)) {
            s.store_scalar(1186, s.v[117]);
        }

        s.v[1196] = if ((s.v[56] < 3.0) || (p.p43 == 1.0)) { 1.0 } else { 0.0 };

        if (((s.v[1131] != 0.0) && (s.v[1194] != 0.0)) && (s.v[1196] != 0.0)) {
            s.store_sub_ad_lhs(1190, A::add(s.ad_value(161), s.ad_value(1191)), 172);
        }

        if (((s.v[1131] != 0.0) && (s.v[1194] != 0.0)) && (!(s.v[1196] != 0.0))) {
            s.store_sub_ad_lhs(1190, A::add(s.ad_value(161), s.ad_value(1191)), 350);
        }

        if ((s.v[1131] != 0.0) && (s.v[1194] != 0.0)) {
            s.store_add_ad_rhs(1189, 1189, A::mul(A::mul(s.ad_value(173), s.ad_value(1186)), s.ad_value(1190)));
        }

        if ((s.v[1131] != 0.0) && (s.v[1194] != 0.0)) {
            s.store_mul(1191, 1188, 1189);
        }

        if ((s.v[1131] != 0.0) && (s.v[1194] != 0.0)) {
            s.copy_ad(1188, 1191);
        }

    }

    pub(super) fn stamp_reactive_block_17(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[1131] != 0.0) && (!(s.v[1194] != 0.0))) {
            s.store_scalar(1188, 0.0);
        }

        s.v[1197] = if (p.p248 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1131] != 0.0) && (s.v[1197] != 0.0)) {
            s.store_scale(1185, 225, s.v[118]);
        }

        if ((s.v[1131] != 0.0) && (s.v[1197] != 0.0)) {
            s.store_mul(1193, 323, 1185);
        }

        if ((s.v[1131] != 0.0) && (s.v[1197] != 0.0)) {
            s.store_mul(1192, 1193, 173);
        }

        if ((s.v[1131] != 0.0) && (!(s.v[1197] != 0.0))) {
            s.store_scalar(1192, 0.0);
        }

        s.v[1198] = if ((s.v[1188] + s.v[1192]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1131] != 0.0) && (s.v[1198] != 0.0)) {
            s.store_mul_ad_rhs(247, 164, A::add(s.ad_value(1188), s.ad_value(1192)));
        }

        if ((s.v[1131] != 0.0) && (s.v[1198] != 0.0)) {
            s.store_mul_ad_lhs(201, A::mul(s.ad_value(264), s.ad_value(247)), 250);
        }

        if (s.v[1131] != 0.0) {
            s.store_add(199, 200, 201);
        }

        if (s.v[1131] != 0.0) {
            s.copy_ad(203, 201);
        }

        s.v[1208] = if (p.p33 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.copy_ad(1201, 549);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_scalar(1202, (s.v[124] - p.p71));
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_div_from_scalar_ad(1203, 1.0, A::square(s.ad_value(1202)));
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_mul_ad_lhs(1204, A::mul(A::mul(A::scale(A::sub_from_scalar(p.p69, s.ad_value(233)), 2.0), A::scale(s.ad_value(324), 1.034943e-10)), s.ad_value(1201)), 1203);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_mul(186, 1204, 235);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_offset_scaled(1200, 173, p.p155, p.p154);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_mul(206, 186, 1200);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_sub_from_scalar_ad(1199, p.p156, A::scale(s.ad_value(157), p.p157));
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_add_ad_lhs(207, A::add(A::offset(s.ad_value(174), (-s.v[123])), s.ad_value(1199)), 206);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_mul_ad_lhs(210, A::mul(s.ad_value(205), s.ad_value(324)), 324);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_scaled_mul(211, 210, 225, 0.5);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_scaled_mul(212, 211, 225, 2.0);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_offset_ad(1205, A::sub(A::offset(A::offset(A::sub(s.ad_value(227), A::mul(s.ad_value(210), A::scale(s.ad_value(225), 0.25))), s.v[123]), (-p.p156)), s.ad_value(206)), 1e-50);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_offset_ad(1199, A::sub(s.ad_value(174), s.ad_value(1205)), (-0.005));
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_scalar(327, (if (s.v[1205] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_sqrt_ad(1201, A::add(A::square(s.ad_value(1199)), A::scale(A::mul(A::scale(s.ad_value(327), 4.0), s.ad_value(1205)), 0.005)));
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_sub_ad_lhs(1202, A::add(A::offset(A::offset(A::add(s.ad_value(1205), A::scale(A::add(s.ad_value(1199), s.ad_value(1201)), 0.5)), (-s.v[123])), p.p156), s.ad_value(206)), 514);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_offset_ad(1203, A::mul(s.ad_value(225), s.ad_value(1202)), (-1.0));
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_div_from_scalar(1204, 4.0, 212);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_offset_ad(1200, A::mul(s.ad_value(1203), s.ad_value(1204)), 1.0);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1200)), ((4.0 * 0.01) * 0.01)));
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_offset_ad(1199, A::scale(A::add(s.ad_value(1200), s.ad_value(44)), 0.5), (1e-10 * 0.01));
        }

        s.v[1209] = if (s.v[1199] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) && (s.v[1209] != 0.0)) {
            s.store_scalar(1199, 0.0);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_sqrt_ad(213, A::offset(s.ad_value(1199), 1e-50));
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_add_ad_rhs(215, 207, A::mul(s.ad_value(211), A::sub_from_scalar(1.0, s.ad_value(213))));
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_div_from_scalar_ad(327, 1.0, A::add(s.ad_value(225), A::div_from_scalar(2.0, A::offset(s.ad_value(207), 1e-50))));
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_mul_ad_lhs(216, A::ln(A::mul(A::div(A::div_from_scalar(1.0, s.ad_value(209)), s.ad_value(210)), A::square(s.ad_value(207)))), 327);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_div_ad_rhs(1202, 216, A::offset(s.ad_value(207), 1e-50));
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_offset_ad(217, A::sub(s.ad_value(216), s.ad_value(215)), (-0.002));
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_sqrt_ad(327, A::add(A::square(s.ad_value(217)), A::scale(s.ad_value(216), (4.0 * 0.002))));
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_sub_ad_rhs(218, 216, A::scale(A::add(s.ad_value(217), s.ad_value(327)), 0.5));
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_div_from_scalar(1199, 1.0, 327);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_mul_ad_rhs(327, 209, A::exp(A::mul(s.ad_value(225), s.ad_value(218))));
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_add_ad_lhs(1200, A::offset(A::mul(s.ad_value(225), A::sub(s.ad_value(218), s.ad_value(514))), (-1.0)), 327);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1200)), ((4.0 * 0.01) * 0.01)));
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_offset_ad(1199, A::scale(A::add(s.ad_value(1200), s.ad_value(44)), 0.5), (1e-10 * 0.01));
        }

        s.v[1210] = if (s.v[1199] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) && (s.v[1210] != 0.0)) {
            s.store_scalar(1199, 0.0);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_sqrt_ad(219, A::offset(s.ad_value(1199), (10.0 * 2.220446049250313e-16)));
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_offset_ad(1200, A::mul(s.ad_value(225), A::sub(s.ad_value(218), s.ad_value(514))), (-1.0));
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1200)), ((4.0 * 0.01) * 0.01)));
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_offset_ad(1199, A::scale(A::add(s.ad_value(1200), s.ad_value(44)), 0.5), (1e-10 * 0.01));
        }

        s.v[1211] = if (s.v[1199] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) && (s.v[1211] != 0.0)) {
            s.store_scalar(1199, 0.0);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_sqrt_ad(220, A::offset(s.ad_value(1199), (10.0 * 2.220446049250313e-16)));
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_mul_ad_rhs(221, 208, A::sub(s.ad_value(219), s.ad_value(220)));
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_sub(1200, 215, 218);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1200)), ((4.0 * 0.1) * 0.1)));
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_offset_ad(1199, A::scale(A::add(s.ad_value(1200), s.ad_value(44)), 0.5), (1e-10 * 0.1));
        }

        s.v[1212] = if (s.v[1199] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) && (s.v[1212] != 0.0)) {
            s.store_scalar(1199, 0.0);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_div_ad_rhs(1206, 157, A::offset(s.ad_value(1199), (10.0 * 2.220446049250313e-16)));
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_square(49, 1206);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_scalar(50, 1.0);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_scalar(51, 1.0);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_scalar(52, 1.0);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_scalar(55, 0.0);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_scalar(48, 0.0);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_scalar(53, 0.0);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_add(48, 51, 52);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.copy_ad(53, 48);
        }

        s.v[1213] = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1214] = if (4.0 == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) && (s.v[1213] != 0.0)) && (s.v[1214] != 0.0)) {
            s.store_scalar(55, 1.0);
        }

        s.v[1215] = if (4.0 == 2.0) { 1.0 } else { 0.0 };

        if (((((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) && (s.v[1213] != 0.0)) && (!(s.v[1214] != 0.0))) && (s.v[1215] != 0.0)) {
            s.store_scalar(55, 2.0);
        }

        s.v[1216] = if (4.0 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) && (s.v[1213] != 0.0)) && (!(s.v[1214] != 0.0))) && (!(s.v[1215] != 0.0))) && (s.v[1216] != 0.0)) {
            s.store_scalar(55, 3.0);
        }

        s.v[1217] = if (4.0 == 8.0) { 1.0 } else { 0.0 };

        if (((((((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) && (s.v[1213] != 0.0)) && (!(s.v[1214] != 0.0))) && (!(s.v[1215] != 0.0))) && (!(s.v[1216] != 0.0))) && (s.v[1217] != 0.0)) {
            s.store_scalar(55, 4.0);
        }

        if (((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) && (s.v[1213] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        let mut assign19470_loop_guard: usize = 0;
        while {
            let assign19470_cond_e26967: f64 = if ((((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) && (s.v[1213] != 0.0)) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign19470_cond_e26967 != 0.0
        } {
            assign19470_loop_guard += 1;
            assert!(assign19470_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) && (s.v[1213] != 0.0)) {
                s.store_sqrt(53, 53);
            }
            if (((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) && (s.v[1213] != 0.0)) {
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) && (!(s.v[1213] != 0.0))) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_div_from_scalar(53, 1.0, 53);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_mul(1207, 1206, 53);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_scale(214, 227, ((2.0 * s.v[126]) * p.p9));
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_div_ad_lhs(222, A::mul(A::mul(A::mul(s.ad_value(214), s.ad_value(250)), s.ad_value(221)), s.ad_value(1207)), 441);
        }

        if ((s.v[1131] != 0.0) && (s.v[1208] != 0.0)) {
            s.store_add(199, 199, 222);
        }

        s.v[1218] = if ((p.p30 != 0.0) && (p.p32 != 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1131] != 0.0) && (s.v[1218] != 0.0)) {
            s.store_square(294, 192);
        }

        if ((s.v[1131] != 0.0) && (s.v[1218] != 0.0)) {
            s.store_mul_ad_lhs(295, A::mul(A::scale(s.ad_value(227), 2.0), s.ad_value(324)), 246);
        }

        if ((s.v[1131] != 0.0) && (s.v[1218] != 0.0)) {
            s.store_sub(296, 294, 295);
        }

        if ((s.v[1131] != 0.0) && (s.v[1218] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(294)), ((4.0 * 0.001) * 0.001)));
        }

        if ((s.v[1131] != 0.0) && (s.v[1218] != 0.0)) {
            s.store_offset_ad(294, A::scale(A::add(s.ad_value(294), s.ad_value(44)), 0.5), (1e-10 * 0.001));
        }

        s.v[1219] = if (s.v[294] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1131] != 0.0) && (s.v[1218] != 0.0)) && (s.v[1219] != 0.0)) {
            s.store_scalar(294, 0.0);
        }

        if ((s.v[1131] != 0.0) && (s.v[1218] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(296)), ((4.0 * 0.001) * 0.001)));
        }

        if ((s.v[1131] != 0.0) && (s.v[1218] != 0.0)) {
            s.store_offset_ad(296, A::scale(A::add(s.ad_value(296), s.ad_value(44)), 0.5), (1e-10 * 0.001));
        }

        s.v[1220] = if (s.v[296] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1131] != 0.0) && (s.v[1218] != 0.0)) && (s.v[1220] != 0.0)) {
            s.store_scalar(296, 0.0);
        }

        if ((s.v[1131] != 0.0) && (s.v[1218] != 0.0)) {
            s.store_sub(297, 294, 296);
        }

        s.v[1221] = if ((s.v[244] < (10.0 * 2.220446049250313e-16)) || (s.v[297] < (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if (((s.v[1131] != 0.0) && (s.v[1218] != 0.0)) && (s.v[1221] != 0.0)) {
            s.store_scalar(146, 0.0);
        }

        if (((s.v[1131] != 0.0) && (s.v[1218] != 0.0)) && (!(s.v[1221] != 0.0))) {
            s.store_scalar(146, 1.0);
        }

        s.copy_ad(202, 199);

        s.v[204] = 0.0;

        s.v[1222] = if ((p.p281 > 0.0) && (p.p285 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1222] != 0.0) {
            s.store_scalar(1229, s.v[99]);
        }

        if (s.v[1222] != 0.0) {
            s.store_scalar(1233, p.p237);
        }

        if (s.v[1222] != 0.0) {
            s.store_offset_ad(1234, A::sub(A::add(A::offset(s.ad_value(158), (-s.v[123])), s.ad_value(185)), s.ad_value(320)), (-p.p286));
        }

        if (s.v[1222] != 0.0) {
            s.store_offset(1235, 182, p.p286);
        }

        if (s.v[1222] != 0.0) {
            s.store_scalar(1237, p.p285);
        }

        if (s.v[1222] != 0.0) {
            s.store_scalar(1236, p.p283);
        }

        if (s.v[1222] != 0.0) {
            s.store_scalar(1227, s.v[70]);
        }

        if (s.v[1222] != 0.0) {
            s.store_mul_ad_rhs(1228, 227, A::ln(A::div(A::mul(A::div(s.ad_value(1227), s.ad_value(230)), s.ad_value(536)), s.ad_value(230))));
        }

        if (s.v[1222] != 0.0) {
            s.store_ad(1225, &{
                if (p.p43 == 1.0) {
                    s.ad_value(435)
                } else {
                    s.ad_value(350)
                }
            });
        }

        if (s.v[1222] != 0.0) {
            s.store_sqrt_ad(1230, A::div(A::mul(A::mul(A::scale(A::sub(s.ad_value(1228), s.ad_value(1225)), ((2.0 * 1.6021918e-19) * 9662367879.197212)), s.ad_value(536)), s.ad_value(1227)), A::add(s.ad_value(536), s.ad_value(1227))));
        }

        if (s.v[1222] != 0.0) {
            s.store_mul(1224, 1230, 1229);
        }

        if (s.v[1222] != 0.0) {
            s.store_div_ad(1223, A::mul(A::scale(s.ad_value(1224), (-0.25)), s.ad_value(1224)), A::add(s.ad_value(157), s.ad_value(1224)));
        }

        if (s.v[1222] != 0.0) {
            s.copy_ad(1249, 1223);
        }

        if (s.v[1222] != 0.0) {
            s.copy_ad(1250, 1235);
        }

    }

    pub(super) fn stamp_reactive_block_18(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1222] != 0.0) {
            s.store_offset_ad(336, A::div(A::scale(A::offset(A::mul(s.ad_value(225), A::sub(s.ad_value(1234), s.ad_value(1249))), (-1.0)), 4.0), A::mul(s.ad_value(241), s.ad_value(226))), 1.0);
        }

        if (s.v[1222] != 0.0) {
            s.store_ad(336, &{
                if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(336)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if (s.v[1222] != 0.0) {
            s.store_add_ad_rhs(376, 1234, A::mul(A::scale(A::mul(s.ad_value(241), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336)))));
        }

        s.v[1251] = if (s.v[158] < ((s.v[123] + s.v[1250]) * 0.5)) { 1.0 } else { 0.0 };

        if ((s.v[1222] != 0.0) && (s.v[1251] != 0.0)) {
            s.store_scalar(144, 0.0);
        }

        s.v[1252] = if ((s.v[144] == 0.0) || (1.0 != 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) {
            s.store_mul_ad_rhs(181, 225, A::sub(s.ad_value(376), s.ad_value(1249)));
        }

        s.v[1253] = if (s.v[181] < 3.0) { 1.0 } else { 0.0 };

        if (((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1253] != 0.0)) {
            s.store_mul_ad_rhs(337, 225, A::sub(s.ad_value(1234), s.ad_value(1249)));
        }

        if (((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1253] != 0.0)) {
            s.store_div_from_scalar_ad(328, 1.0, A::mul(A::scale(s.ad_value(225), (1.414213562373095 / 108.0)), s.ad_value(240)));
        }

        if (((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1253] != 0.0)) {
            s.store_offset_scaled(329, 328, 3.0, 81.0);
        }

        if (((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1253] != 0.0)) {
            s.store_add_ad(330, A::sub_from_scalar((-2916.0), A::scale(s.ad_value(328), 81.0)), A::mul(A::scale(s.ad_value(328), 27.0), s.ad_value(337)));
        }

        if (((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1253] != 0.0)) {
            s.store_add_ad(331, A::sub_from_scalar(1458.0, A::scale(A::offset(s.ad_value(328), 54.0), 81.0)), A::mul(A::scale(s.ad_value(328), 27.0), s.ad_value(337)));
        }

        if (((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1253] != 0.0)) {
            s.store_square(331, 331);
        }

        if (((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1253] != 0.0)) {
            s.store_powf_ad(332, A::add(s.ad_value(330), A::sqrt(A::add(A::mul(A::mul(A::scale(s.ad_value(329), 4.0), s.ad_value(329)), s.ad_value(329)), s.ad_value(331)))), 0.3333333333333333);
        }

        if (((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1253] != 0.0)) {
            s.store_add_ad(336, A::sub_from_scalar(3.0, A::div(A::scale(s.ad_value(329), 1.259921049894873), A::scale(s.ad_value(332), 3.0))), A::scale(s.ad_value(332), (1.0 / (3.0 * 1.259921049894873))));
        }

        if (((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1253] != 0.0)) {
            s.store_add_ad_lhs(376, A::mul(s.ad_value(336), s.ad_value(227)), 1249);
        }

        if (((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1253] != 0.0)) {
            s.copy_ad(378, 376);
        }

        s.v[1254] = if ((s.v[158] - s.v[383]) <= s.v[1250]) { 1.0 } else { 0.0 };

        s.v[1255] = if (p.p43 == 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (!(s.v[1253] != 0.0))) && (s.v[1254] != 0.0)) && (s.v[1255] != 0.0)) {
            s.store_div_from_scalar(327, 1.0, 323);
        }

        if (((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (!(s.v[1253] != 0.0))) && (s.v[1254] != 0.0)) && (s.v[1255] != 0.0)) {
            s.store_scale(328, 1233, 9662367879.197212);
        }

        if (((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (!(s.v[1253] != 0.0))) && (s.v[1254] != 0.0)) && (s.v[1255] != 0.0)) {
            s.store_scalar(329, (1.0 / s.v[93]));
        }

        if (((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (!(s.v[1253] != 0.0))) && (s.v[1254] != 0.0)) && (s.v[1255] != 0.0)) {
            s.store_div_from_scalar_ad(330, 1.0, A::add(A::add(s.ad_value(327), s.ad_value(328)), s.ad_value(329)));
        }

        if (((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (!(s.v[1253] != 0.0))) && (s.v[1254] != 0.0)) && (s.v[1255] != 0.0)) {
            s.store_mul_ad_rhs(331, 330, A::add(A::sub(s.ad_value(1234), s.ad_value(475)), A::mul(A::add(s.ad_value(329), A::scale(s.ad_value(328), 0.5)), A::neg(s.ad_value(369)))));
        }

        if (((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (!(s.v[1253] != 0.0))) && (s.v[1254] != 0.0)) && (s.v[1255] != 0.0)) {
            s.store_sub_ad_rhs(376, 1234, A::div(s.ad_value(331), s.ad_value(323)));
        }

        if ((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (!(s.v[1253] != 0.0))) && (s.v[1254] != 0.0)) {
            s.copy_ad(378, 376);
        }

        if ((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (!(s.v[1253] != 0.0))) && (!(s.v[1254] != 0.0))) {
            s.store_div_ad_lhs(328, A::div_from_scalar(1.0, s.ad_value(379)), 434);
        }

        if ((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (!(s.v[1253] != 0.0))) && (!(s.v[1254] != 0.0))) {
            s.store_mul_ad(329, A::mul(s.ad_value(328), A::sub(s.ad_value(1234), s.ad_value(383))), A::sub(s.ad_value(1234), s.ad_value(383)));
        }

        if ((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (!(s.v[1253] != 0.0))) && (!(s.v[1254] != 0.0))) {
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(1234), s.ad_value(383))));
        }

        if ((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (!(s.v[1253] != 0.0))) && (!(s.v[1254] != 0.0))) {
            s.store_offset_ad(377, A::div(A::ln(s.ad_value(329)), s.ad_value(330)), p.p287);
        }

        if ((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (!(s.v[1253] != 0.0))) && (!(s.v[1254] != 0.0))) {
            s.store_offset_ad(44, A::sub(s.ad_value(377), s.ad_value(376)), (-0.0008));
        }

        if ((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (!(s.v[1253] != 0.0))) && (!(s.v[1254] != 0.0))) {
            s.store_scale(45, 377, (4.0 * 0.0008));
        }

        if ((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (!(s.v[1253] != 0.0))) && (!(s.v[1254] != 0.0))) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (!(s.v[1253] != 0.0))) && (!(s.v[1254] != 0.0))) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (!(s.v[1253] != 0.0))) && (!(s.v[1254] != 0.0))) {
            s.store_sub_ad_rhs(378, 377, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        s.v[1256] = if (p.p43 == 0.0) { 1.0 } else { 0.0 };

        s.v[1257] = if ((s.v[158] - s.v[383]) <= s.v[1250]) { 1.0 } else { 0.0 };

        if ((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_div_from_scalar(327, 1.0, 323);
        }

        if ((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_scale(328, 1233, 9662367879.197212);
        }

        if ((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_scalar(329, (1.0 / s.v[93]));
        }

        if ((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_div_from_scalar_ad(330, 1.0, A::add(A::add(s.ad_value(327), s.ad_value(328)), s.ad_value(329)));
        }

        if ((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_mul_ad_rhs(331, 330, A::add(A::sub(s.ad_value(1234), s.ad_value(475)), A::mul(A::add(s.ad_value(329), A::scale(s.ad_value(328), 0.5)), A::neg(s.ad_value(369)))));
        }

        if ((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_sub_ad_rhs(376, 1234, A::div(s.ad_value(331), s.ad_value(323)));
        }

        if ((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.copy_ad(378, 376);
        }

        if ((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) {
            s.store_div_from_scalar(327, 1.0, 323);
        }

        if ((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) {
            s.store_scale(328, 1233, 9662367879.197212);
        }

        if ((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) {
            s.store_scalar(329, (1.0 / s.v[93]));
        }

        if ((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) {
            s.store_div_from_scalar_ad(330, 1.0, A::add(A::add(s.ad_value(327), s.ad_value(328)), s.ad_value(329)));
        }

        if ((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) {
            s.store_mul_ad_rhs(331, 330, A::add(A::sub(s.ad_value(1234), s.ad_value(475)), A::mul(A::add(s.ad_value(329), A::scale(s.ad_value(328), 0.5)), A::neg(s.ad_value(369)))));
        }

        if ((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) {
            s.store_sub_ad_rhs(376, 1234, A::div(s.ad_value(331), s.ad_value(323)));
        }

        if ((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) {
            s.copy_ad(378, 376);
        }

        s.v[1258] = if ((s.v[1234] - s.v[383]) > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) {
            s.store_div_ad_lhs(328, A::div_from_scalar(1.0, s.ad_value(379)), 434);
        }

        if (((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) {
            s.store_mul_ad(329, A::mul(s.ad_value(328), A::sub(s.ad_value(1234), s.ad_value(383))), A::sub(s.ad_value(1234), s.ad_value(383)));
        }

        if (((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) {
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(1234), s.ad_value(383))));
        }

        if (((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) {
            s.store_offset_ad(377, A::div(A::ln(s.ad_value(329)), s.ad_value(330)), p.p287);
        }

        s.v[1259] = if ((s.v[376] > ((s.v[377] * 0.98) - 0.4)) && (0.4 >= 0.0)) { 1.0 } else { 0.0 };

        if ((((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) {
            s.store_offset_ad(44, A::sub(s.ad_value(376), A::scale(s.ad_value(377), 0.98)), 0.4);
        }

        if ((((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) {
            s.store_square(49, 44);
        }

        if ((((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) {
            s.store_scalar(50, (0.4 * 0.4));
        }

        if ((((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) {
            s.store_scalar(51, 1.0);
        }

        if ((((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) {
            s.store_scalar(52, 1.0);
        }

        if ((((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        if ((((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) {
            s.store_scalar(55, 0.0);
        }

        if ((((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) {
            s.store_scalar(48, 0.0);
        }

        if ((((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) {
            s.store_scalar(53, 0.0);
        }

        if ((((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) {
            s.store_add(48, 51, 52);
        }

        if ((((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) {
            s.copy_ad(53, 48);
        }

        s.v[1260] = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1261] = if (2.0 == 1.0) { 1.0 } else { 0.0 };

        if ((((((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) {
            s.store_scalar(55, 1.0);
        }

        s.v[1262] = if (2.0 == 2.0) { 1.0 } else { 0.0 };

        if (((((((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) && (s.v[1260] != 0.0)) && (!(s.v[1261] != 0.0))) && (s.v[1262] != 0.0)) {
            s.store_scalar(55, 2.0);
        }

        s.v[1263] = if (2.0 == 4.0) { 1.0 } else { 0.0 };

        if ((((((((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) && (s.v[1260] != 0.0)) && (!(s.v[1261] != 0.0))) && (!(s.v[1262] != 0.0))) && (s.v[1263] != 0.0)) {
            s.store_scalar(55, 3.0);
        }

        s.v[1264] = if (2.0 == 8.0) { 1.0 } else { 0.0 };

        if (((((((((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) && (s.v[1260] != 0.0)) && (!(s.v[1261] != 0.0))) && (!(s.v[1262] != 0.0))) && (!(s.v[1263] != 0.0))) && (s.v[1264] != 0.0)) {
            s.store_scalar(55, 4.0);
        }

        if (((((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) && (s.v[1260] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        let mut assign20700_loop_guard: usize = 0;
        while {
            let assign20700_cond_e28539: f64 = if ((((((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) && (s.v[1260] != 0.0)) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign20700_cond_e28539 != 0.0
        } {
            assign20700_loop_guard += 1;
            assert!(assign20700_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) && (s.v[1260] != 0.0)) {
                s.store_sqrt(53, 53);
            }
            if (((((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) && (s.v[1260] != 0.0)) {
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) && (!(s.v[1260] != 0.0))) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if ((((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) {
            s.store_div_from_scalar(53, 1.0, 53);
        }

        if ((((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) {
            s.store_mul_ad_lhs(43, A::scale(s.ad_value(44), 0.4), 53);
        }

        if ((((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) {
            s.store_add_ad_lhs(378, A::offset(A::scale(s.ad_value(377), 0.98), (-0.4)), 43);
        }

        if ((((((s.v[1222] != 0.0) && (s.v[1252] != 0.0)) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) {
            s.copy_ad(378, 376);
        }

        if (s.v[1222] != 0.0) {
            s.store_offset(336, 1249, (5e-12 / 2.0));
        }

        s.v[1265] = if (s.v[378] < s.v[336]) { 1.0 } else { 0.0 };

        if ((s.v[1222] != 0.0) && (s.v[1265] != 0.0)) {
            s.copy_ad(378, 336);
        }

        if (s.v[1222] != 0.0) {
            s.copy_ad(1232, 378);
        }

        if (s.v[1222] != 0.0) {
            s.copy_ad(163, 376);
        }

        if ((s.v[1222] != 0.0) && (0.0 != 0.0)) {
            s.store_ad(166, &{
                if ((s.v[376] - s.v[1232]) >= 0.0) {
                    A::sub(s.ad_value(376), s.ad_value(1232))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if ((s.v[1222] != 0.0) && (0.0 != 0.0)) {
            s.store_offset_ad(44, A::offset(A::scale(s.ad_value(166), (1.0 + 0.3)), (-p.p287)), (-0.03));
        }

        if ((s.v[1222] != 0.0) && (0.0 != 0.0)) {
            s.store_scale(45, 166, ((1.0 + 0.3) * (4.0 * 0.03)));
        }

        if ((s.v[1222] != 0.0) && (0.0 != 0.0)) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((s.v[1222] != 0.0) && (0.0 != 0.0)) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((s.v[1222] != 0.0) && (0.0 != 0.0)) {
            s.store_sub_ad(165, A::scale(s.ad_value(166), (1.0 + 0.3)), A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((s.v[1222] != 0.0) && (0.0 != 0.0)) {
            s.store_ad(165, &{
                if (s.v[165] <= s.v[166]) {
                    s.ad_value(165)
                } else {
                    s.ad_value(166)
                }
            });
        }

        s.v[1266] = if (s.v[165] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1222] != 0.0) && (0.0 != 0.0)) && (s.v[1266] != 0.0)) {
            s.store_scalar(165, 0.0);
        }

        s.v[1267] = if (s.v[165] > s.v[157]) { 1.0 } else { 0.0 };

        if ((((s.v[1222] != 0.0) && (0.0 != 0.0)) && (!(s.v[1266] != 0.0))) && (s.v[1267] != 0.0)) {
            s.copy_ad(165, 157);
        }

        if ((s.v[1222] != 0.0) && (0.0 != 0.0)) {
            s.store_add(163, 1232, 165);
        }

        s.v[1268] = if (p.p282 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) {
            s.copy_ad(378, 1232);
        }

        if ((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) {
            s.copy_ad(1269, 1223);
        }

        if ((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) {
            s.store_offset_ad(160, A::add(A::add(A::sub_from_scalar(s.v[123], s.ad_value(185)), s.ad_value(320)), s.ad_value(1269)), p.p286);
        }

        s.v[1271] = if (s.v[158] < s.v[160]) { 1.0 } else { 0.0 };

        if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (s.v[1271] != 0.0)) {
            s.store_scalar(338, (-1.0));
        }

        if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (s.v[1271] != 0.0)) {
            s.store_mul_ad(254, A::scale(s.ad_value(227), 2.0), A::ln(A::div_from_scalar((-s.v[139]), s.ad_value(240))));
        }

        if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (s.v[1271] != 0.0)) {
            s.store_mul_ad_rhs(336, 225, A::sub(s.ad_value(1234), s.ad_value(1269)));
        }

        if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (s.v[1271] != 0.0)) {
            s.store_div_from_scalar_ad(328, 1.0, A::mul(s.ad_value(225), s.ad_value(238)));
        }

        if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (s.v[1271] != 0.0)) {
            s.store_mul(337, 328, 323);
        }

        if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (s.v[1271] != 0.0)) {
            s.store_offset_scaled(262, 337, (3.0 * 1.414213562373095), 2.0);
        }

        if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (s.v[1271] != 0.0)) {
            s.store_mul_ad_lhs(260, A::mul(A::scale(s.ad_value(262), 8.0), s.ad_value(262)), 262);
        }

        if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (s.v[1271] != 0.0)) {
            s.store_offset(331, 336, (-2.0));
        }

        if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (s.v[1271] != 0.0)) {
            s.store_mul_ad_lhs(332, A::scale(s.ad_value(337), 9.0), 331);
        }

        if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (s.v[1271] != 0.0)) {
            s.store_sub_from_scalar(261, (7.0 * 1.414213562373095), 332);
        }

        if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (s.v[1271] != 0.0)) {
            s.store_square(259, 261);
        }

        s.v[1272] = if (s.v[260] < (s.v[259] * 1e-8)) { 1.0 } else { 0.0 };

        if ((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (s.v[1271] != 0.0)) && (s.v[1272] != 0.0)) {
            s.store_add_ad_lhs(257, A::add(A::offset(s.ad_value(261), ((-7.0) * 1.414213562373095)), A::div(A::scale(s.ad_value(260), 0.5), s.ad_value(261))), 332);
        }

        if ((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (s.v[1271] != 0.0)) && (!(s.v[1272] != 0.0))) {
            s.store_sqrt_ad(258, A::add(s.ad_value(260), s.ad_value(259)));
        }

        if ((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (s.v[1271] != 0.0)) && (!(s.v[1272] != 0.0))) {
            s.store_add_ad_lhs(257, A::offset(s.ad_value(258), ((-7.0) * 1.414213562373095)), 332);
        }

        if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (s.v[1271] != 0.0)) {
            s.store_powf(256, 257, 0.3333333333333333);
        }

    }

    pub(super) fn stamp_reactive_block_19(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (s.v[1271] != 0.0)) {
            s.store_add_ad(255, A::add(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(337), 12.0)), A::scale(s.ad_value(256), 2.0)), A::mul(A::scale(s.ad_value(256), 1.414213562373095), s.ad_value(256)));
        }

        if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (s.v[1271] != 0.0)) {
            s.store_div_from_scalar(328, 1.0, 256);
        }

        if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (s.v[1271] != 0.0)) {
            s.store_mul(181, 255, 328);
        }

        if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (s.v[1271] != 0.0)) {
            s.store_add_ad_lhs(313, A::mul(s.ad_value(181), s.ad_value(227)), 1269);
        }

        if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (s.v[1271] != 0.0)) {
            s.store_sub(328, 313, 1269);
        }

        if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (s.v[1271] != 0.0)) {
            s.store_div(329, 328, 254);
        }

        if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (s.v[1271] != 0.0)) {
            s.store_sqrt_ad(330, A::offset(A::square(s.ad_value(329)), 1.0));
        }

        if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (s.v[1271] != 0.0)) {
            s.store_add_ad_lhs(1232, A::div(s.ad_value(328), s.ad_value(330)), 1269);
        }

        if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) {
            s.store_exp_ad(484, A::mul(s.ad_value(225), A::offset(s.ad_value(1269), (-p.p287))));
        }

        if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) {
            s.store_scalar(430, 0.0);
        }

        if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) {
            s.copy_ad(1270, 378);
        }

        if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) {
            s.store_scale_ad(419, A::scale(s.ad_value(229), (p.p237 * (p.p237 * 0.5))), 9662367879.197212);
        }

        if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) {
            s.store_sqrt_ad(327, A::mul(A::scale(s.ad_value(225), 2.0), s.ad_value(419)));
        }

        if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) {
            s.store_scale_ad(328, A::add(A::exp(s.ad_value(327)), A::exp(A::neg(s.ad_value(327)))), 0.5);
        }

        if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) {
            s.store_div_ad_lhs(420, A::ln(s.ad_value(328)), 419);
        }

        if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) {
            s.store_scalar(167, 1.0);
        }

        let mut assign21300_loop_guard: usize = 0;
        while {
            let assign21300_cond_e29269: f64 = (s.v[57] + 1.0);
            let assign21300_cond_e29271: f64 = if ((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (s.v[167] <= assign21300_cond_e29269)) { 1.0 } else { 0.0 };
            assign21300_cond_e29271 != 0.0
        } {
            assign21300_loop_guard += 1;
            assert!(assign21300_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) {
                s.store_sub(417, 1270, 1269);
            }
            if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) {
                s.store_mul(181, 225, 417);
            }
            if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) {
                s.store_mul_ad_rhs(337, 420, A::sub(s.ad_value(417), s.ad_value(419)));
            }
            s.v[1273] = if (s.v[337] < 80.0) { 1.0 } else { 0.0 };
            if ((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (s.v[1273] != 0.0)) {
                s.store_exp(328, 337);
            }
            if ((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (s.v[1273] != 0.0)) {
                s.store_exp_ad(327, A::mul(A::neg(s.ad_value(420)), s.ad_value(419)));
            }
            if ((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (s.v[1273] != 0.0)) {
                s.store_sub(329, 328, 327);
            }
            if ((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (s.v[1273] != 0.0)) {
                s.store_div_ad_lhs(422, A::ln(A::offset(s.ad_value(329), 1.0)), 420);
            }
            if ((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (s.v[1273] != 0.0)) {
                s.store_div_ad_rhs(423, 328, A::offset(s.ad_value(329), 1.0));
            }
            if ((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (!(s.v[1273] != 0.0))) {
                s.store_sub(422, 417, 419);
            }
            if ((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (!(s.v[1273] != 0.0))) {
                s.store_scalar(423, 1.0);
            }
            if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) {
                s.store_mul(421, 225, 422);
            }
            s.v[1274] = if (((s.v[181]) as f64).abs() < 1e-16) { 1.0 } else { 0.0 };
            if ((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (s.v[1274] != 0.0)) {
                s.store_sqrt_ad(327, A::scale(A::sub_from_scalar(1.0, A::square(s.ad_value(423))), 0.5));
            }
            if ((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (s.v[1274] != 0.0)) {
                s.store_mul(242, 181, 327);
            }
            if ((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (s.v[1274] != 0.0)) {
                s.store_mul(443, 225, 327);
            }
            s.v[1275] = if (s.v[181] < 0.0) { 1.0 } else { 0.0 };
            if (((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (s.v[1274] != 0.0)) && (s.v[1275] != 0.0)) {
                s.store_neg(242, 242);
            }
            if (((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (s.v[1274] != 0.0)) && (s.v[1275] != 0.0)) {
                s.store_neg(443, 443);
            }
            s.v[1276] = if (((s.v[181]) as f64).abs() < 0.005) { 1.0 } else { 0.0 };
            if (((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (!(s.v[1274] != 0.0))) && (s.v[1276] != 0.0)) {
                s.store_mul_ad(327, A::scale(A::square(s.ad_value(181)), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(181), 0.3333333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(181), 0.25), A::sub_from_scalar(1.0, A::scale(s.ad_value(181), 0.2)))))));
            }
            if (((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (!(s.v[1274] != 0.0))) && (s.v[1276] != 0.0)) {
                s.store_mul_ad_rhs(328, 181, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(181), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(181), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(181), 0.25)))))));
            }
            if (((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (!(s.v[1274] != 0.0))) && (s.v[1276] != 0.0)) {
                s.store_mul_ad(329, A::scale(A::square(s.ad_value(421)), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(421), 0.3333333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(421), 0.25), A::sub_from_scalar(1.0, A::scale(s.ad_value(421), 0.2)))))));
            }
            if (((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (!(s.v[1274] != 0.0))) && (s.v[1276] != 0.0)) {
                s.store_mul_ad_rhs(330, 421, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(421), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(421), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(421), 0.25)))))));
            }
            if (((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (!(s.v[1274] != 0.0))) && (s.v[1276] != 0.0)) {
                s.store_sqrt_ad(242, A::sub(s.ad_value(327), s.ad_value(329)));
            }
            if (((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (!(s.v[1274] != 0.0))) && (s.v[1276] != 0.0)) {
                s.store_div_ad_lhs(443, A::mul(A::scale(s.ad_value(225), 0.5), A::sub(s.ad_value(328), A::mul(s.ad_value(423), s.ad_value(330)))), 242);
            }
            if (((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (!(s.v[1274] != 0.0))) && (!(s.v[1276] != 0.0))) {
                s.store_exp_ad(327, A::neg(s.ad_value(181)));
            }
            if (((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (!(s.v[1274] != 0.0))) && (!(s.v[1276] != 0.0))) {
                s.store_exp_ad(328, A::neg(s.ad_value(421)));
            }
            if (((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (!(s.v[1274] != 0.0))) && (!(s.v[1276] != 0.0))) {
                s.store_sqrt_ad(242, A::add(A::sub(s.ad_value(181), s.ad_value(421)), A::sub(s.ad_value(327), s.ad_value(328))));
            }
            if (((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (!(s.v[1274] != 0.0))) && (!(s.v[1276] != 0.0))) {
                s.store_div_ad_lhs(443, A::mul(A::scale(s.ad_value(225), 0.5), A::sub(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul(s.ad_value(423), A::sub_from_scalar(1.0, s.ad_value(328))))), 242);
            }
            s.v[1277] = if ((s.v[430] == 1.0) && (s.v[181] < 0.0)) { 1.0 } else { 0.0 };
            if ((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (s.v[1277] != 0.0)) {
                s.store_scalar(338, (-1.0));
            }
            s.v[1278] = if (s.v[181] < 0.0) { 1.0 } else { 0.0 };
            if ((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (s.v[1278] != 0.0)) {
                s.store_neg(490, 242);
            }
            if ((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (s.v[1278] != 0.0)) {
                s.store_neg(491, 443);
            }
            s.v[1279] = if (s.v[181] < 1e-7) { 1.0 } else { 0.0 };
            if (((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (!(s.v[1278] != 0.0))) && (s.v[1279] != 0.0)) {
                s.copy_ad(490, 242);
            }
            if (((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (!(s.v[1278] != 0.0))) && (s.v[1279] != 0.0)) {
                s.copy_ad(491, 443);
            }
            if (((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (!(s.v[1278] != 0.0))) && (!(s.v[1279] != 0.0))) {
                s.store_mul_ad_rhs(501, 225, A::offset(s.ad_value(1270), (-p.p287)));
            }
            if (((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (!(s.v[1278] != 0.0))) && (!(s.v[1279] != 0.0))) {
                s.store_exp(502, 501);
            }
            if (((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (!(s.v[1278] != 0.0))) && (!(s.v[1279] != 0.0))) {
                s.store_mul_ad_rhs(488, 379, A::sub(s.ad_value(502), A::mul(s.ad_value(484), A::offset(s.ad_value(181), 1.0))));
            }
            if (((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (!(s.v[1278] != 0.0))) && (!(s.v[1279] != 0.0))) {
                s.store_mul_ad(489, A::mul(s.ad_value(379), s.ad_value(225)), A::sub(s.ad_value(502), s.ad_value(484)));
            }
            if (((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (!(s.v[1278] != 0.0))) && (!(s.v[1279] != 0.0))) {
                s.store_sqrt_ad(490, A::add(A::square(s.ad_value(242)), s.ad_value(488)));
            }
            if (((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (!(s.v[1278] != 0.0))) && (!(s.v[1279] != 0.0))) {
                s.store_div_ad_lhs(491, A::scale(A::add(A::mul(A::scale(s.ad_value(443), 2.0), s.ad_value(242)), s.ad_value(489)), 0.5), 490);
            }
            if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) {
                s.store_add_ad(492, A::sub(s.ad_value(1270), s.ad_value(1234)), A::mul(s.ad_value(240), s.ad_value(490)));
            }
            if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) {
                s.store_offset_ad(493, A::mul(s.ad_value(240), s.ad_value(491)), 1.0);
            }
            s.v[1280] = if (s.v[430] == 1.0) { 1.0 } else { 0.0 };
            if ((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (s.v[1280] != 0.0)) {
                s.store_scalar(167, (s.v[57] + 1.0));
            }
            if ((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (!(s.v[1280] != 0.0))) {
                s.store_div_ad_lhs(494, A::neg(s.ad_value(492)), 493);
            }
            if ((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (!(s.v[1280] != 0.0))) {
                s.store_scale_ad(496, A::offset({
                    if (1.0 >= ((s.v[1270]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1270))
                    }
                }, 1.0), (0.5 * 0.1));
            }
            s.v[1281] = if (((s.v[494]) as f64).abs() > s.v[496]) { 1.0 } else { 0.0 };
            if (((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (!(s.v[1280] != 0.0))) && (s.v[1281] != 0.0)) {
                s.store_scale(494, 496, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (!(s.v[1280] != 0.0))) {
                s.store_add(1270, 1270, 494);
            }
            s.v[1282] = if ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[492]) as f64).abs() <= 1e-8)) { 1.0 } else { 0.0 };
            if (((((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) && (!(s.v[1280] != 0.0))) && (s.v[1282] != 0.0)) {
                s.store_scalar(430, 1.0);
            }
            if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) {
                s.store_offset(167, 167, 1.0);
            }
        }

        if (((s.v[1222] != 0.0) && (s.v[1268] != 0.0)) && (!(s.v[1271] != 0.0))) {
            s.copy_ad(1232, 1270);
        }

        if (s.v[1222] != 0.0) {
            s.store_mul_ad(332, A::neg(s.ad_value(225)), A::sub(s.ad_value(1232), s.ad_value(1223)));
        }

        if (s.v[1222] != 0.0) {
            s.store_scalar(1247, (if (s.v[332] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if (s.v[1222] != 0.0) {
            s.store_mul(1248, 1247, 332);
        }

        if (s.v[1222] != 0.0) {
            s.store_exp(333, 332);
        }

        if (s.v[1222] != 0.0) {
            s.store_sub_ad_lhs(334, A::offset(s.ad_value(333), (-1.0)), 332);
        }

        s.v[1283] = if (s.v[332] > 1e-7) { 1.0 } else { 0.0 };

        if ((s.v[1222] != 0.0) && (s.v[1283] != 0.0)) {
            s.store_mul_ad(437, A::neg(s.ad_value(238)), A::sqrt(s.ad_value(334)));
        }

        s.v[1284] = if (s.v[1248] > 1e-7) { 1.0 } else { 0.0 };

        if (((s.v[1222] != 0.0) && (!(s.v[1283] != 0.0))) && (s.v[1284] != 0.0)) {
            s.store_mul_ad_rhs(437, 238, A::sqrt(s.ad_value(334)));
        }

        if (((s.v[1222] != 0.0) && (!(s.v[1283] != 0.0))) && (!(s.v[1284] != 0.0))) {
            s.store_mul_ad(437, A::scale(A::mul(A::neg(s.ad_value(1247)), s.ad_value(1248)), 0.7071067811865475), A::sqrt(A::offset(A::mul(A::scale(s.ad_value(1248), 0.3333333333333333), A::offset(A::scale(s.ad_value(1248), 0.25), 1.0)), 1.0)));
        }

        if (s.v[1222] != 0.0) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(437)), ((4.0 * 1e-6) * 1e-6)));
        }

        if (s.v[1222] != 0.0) {
            s.store_offset_ad(1244, A::scale(A::add(s.ad_value(437), s.ad_value(44)), 0.5), (1e-10 * 1e-6));
        }

        s.v[1285] = if (s.v[1244] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1222] != 0.0) && (s.v[1285] != 0.0)) {
            s.store_scalar(1244, 0.0);
        }

        if (s.v[1222] != 0.0) {
            s.store_div_ad_rhs(1245, 1244, A::scale(s.ad_value(536), 1.6021918e-19));
        }

        if (s.v[1222] != 0.0) {
            s.store_sub(328, 1245, 1236);
        }

        if (s.v[1222] != 0.0) {
            s.store_scale(1246, 1245, 0.01);
        }

        if (s.v[1222] != 0.0) {
            s.store_sqrt_ad(44, A::add(A::square(s.ad_value(328)), A::mul(A::scale(s.ad_value(1246), 4.0), s.ad_value(1246))));
        }

        if (s.v[1222] != 0.0) {
            s.store_add_ad(329, A::scale(A::add(s.ad_value(328), s.ad_value(44)), 0.5), A::scale(s.ad_value(1246), 1e-10));
        }

        s.v[1286] = if (s.v[329] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1222] != 0.0) && (s.v[1286] != 0.0)) {
            s.store_scalar(329, 0.0);
        }

        if (s.v[1222] != 0.0) {
            s.store_div_ad_lhs(1243, A::mul(A::div(s.ad_value(329), s.ad_value(1245)), s.ad_value(329)), 1245);
        }

        if (s.v[1222] != 0.0) {
            s.store_add_ad_lhs(1226, A::mul(A::sub(s.ad_value(1232), s.ad_value(1223)), s.ad_value(1243)), 1223);
        }

        if (s.v[1222] != 0.0) {
            s.store_sub_ad(337, A::exp(A::mul(s.ad_value(225), s.ad_value(1226))), A::exp(A::mul(s.ad_value(225), A::sub(s.ad_value(1226), s.ad_value(157)))));
        }

        if (s.v[1222] != 0.0) {
            s.store_sqrt_ad(1239, A::scale(s.ad_value(1227), ((2.0 * 1.6021918e-19) * 1.034943e-10)));
        }

        if (s.v[1222] != 0.0) {
            s.store_mul_ad_rhs(1240, 1239, A::sqrt(s.ad_value(227)));
        }

        if (s.v[1222] != 0.0) {
            s.store_mul_ad_rhs(1231, 225, A::sub(s.ad_value(1226), s.ad_value(1223)));
        }

        s.v[1287] = if ((s.v[1231] < (0.2 * s.v[225])) && ((0.2 * s.v[225]) >= 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1222] != 0.0) && (s.v[1287] != 0.0)) {
            s.store_sub_ad_lhs(44, A::scale(s.ad_value(225), 0.2), 1231);
        }

        if ((s.v[1222] != 0.0) && (s.v[1287] != 0.0)) {
            s.store_square(49, 44);
        }

        if ((s.v[1222] != 0.0) && (s.v[1287] != 0.0)) {
            s.store_mul_ad(50, A::scale(s.ad_value(225), 0.2), A::scale(s.ad_value(225), 0.2));
        }

        if ((s.v[1222] != 0.0) && (s.v[1287] != 0.0)) {
            s.store_scalar(51, 1.0);
        }

        if ((s.v[1222] != 0.0) && (s.v[1287] != 0.0)) {
            s.store_scalar(52, 1.0);
        }

        if ((s.v[1222] != 0.0) && (s.v[1287] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        if ((s.v[1222] != 0.0) && (s.v[1287] != 0.0)) {
            s.store_scalar(55, 0.0);
        }

        if ((s.v[1222] != 0.0) && (s.v[1287] != 0.0)) {
            s.store_scalar(48, 0.0);
        }

        if ((s.v[1222] != 0.0) && (s.v[1287] != 0.0)) {
            s.store_scalar(53, 0.0);
        }

        if ((s.v[1222] != 0.0) && (s.v[1287] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((s.v[1222] != 0.0) && (s.v[1287] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((s.v[1222] != 0.0) && (s.v[1287] != 0.0)) {
            s.store_add(48, 51, 52);
        }

        if ((s.v[1222] != 0.0) && (s.v[1287] != 0.0)) {
            s.copy_ad(53, 48);
        }

        s.v[1288] = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1289] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[1222] != 0.0) && (s.v[1287] != 0.0)) && (s.v[1288] != 0.0)) && (s.v[1289] != 0.0)) {
            s.store_scalar(55, 1.0);
        }

        s.v[1290] = if (1.0 == 2.0) { 1.0 } else { 0.0 };

        if (((((s.v[1222] != 0.0) && (s.v[1287] != 0.0)) && (s.v[1288] != 0.0)) && (!(s.v[1289] != 0.0))) && (s.v[1290] != 0.0)) {
            s.store_scalar(55, 2.0);
        }

        s.v[1291] = if (1.0 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1222] != 0.0) && (s.v[1287] != 0.0)) && (s.v[1288] != 0.0)) && (!(s.v[1289] != 0.0))) && (!(s.v[1290] != 0.0))) && (s.v[1291] != 0.0)) {
            s.store_scalar(55, 3.0);
        }

        s.v[1292] = if (1.0 == 8.0) { 1.0 } else { 0.0 };

        if (((((((s.v[1222] != 0.0) && (s.v[1287] != 0.0)) && (s.v[1288] != 0.0)) && (!(s.v[1289] != 0.0))) && (!(s.v[1290] != 0.0))) && (!(s.v[1291] != 0.0))) && (s.v[1292] != 0.0)) {
            s.store_scalar(55, 4.0);
        }

        if (((s.v[1222] != 0.0) && (s.v[1287] != 0.0)) && (s.v[1288] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        let mut assign21830_loop_guard: usize = 0;
        while {
            let assign21830_cond_e30586: f64 = if ((((s.v[1222] != 0.0) && (s.v[1287] != 0.0)) && (s.v[1288] != 0.0)) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign21830_cond_e30586 != 0.0
        } {
            assign21830_loop_guard += 1;
            assert!(assign21830_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[1222] != 0.0) && (s.v[1287] != 0.0)) && (s.v[1288] != 0.0)) {
                s.store_sqrt(53, 53);
            }
            if (((s.v[1222] != 0.0) && (s.v[1287] != 0.0)) && (s.v[1288] != 0.0)) {
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((s.v[1222] != 0.0) && (s.v[1287] != 0.0)) && (!(s.v[1288] != 0.0))) {
            s.store_powf(53, 53, (1.0 / 2.0));
        }

        if ((s.v[1222] != 0.0) && (s.v[1287] != 0.0)) {
            s.store_div_from_scalar(53, 1.0, 53);
        }

        if ((s.v[1222] != 0.0) && (s.v[1287] != 0.0)) {
            s.store_mul_ad_lhs(43, A::mul(s.ad_value(44), A::scale(s.ad_value(225), 0.2)), 53);
        }

        if ((s.v[1222] != 0.0) && (s.v[1287] != 0.0)) {
            s.store_sub_ad_lhs(328, A::scale(s.ad_value(225), 0.2), 43);
        }

        if ((s.v[1222] != 0.0) && (!(s.v[1287] != 0.0))) {
            s.copy_ad(328, 1231);
        }

        if (s.v[1222] != 0.0) {
            s.store_sqrt_ad(1241, A::offset(s.ad_value(328), (10.0 * 2.220446049250313e-16)));
        }

        if (s.v[1222] != 0.0) {
            s.store_mul(1242, 1240, 1241);
        }

        if (s.v[1222] != 0.0) {
            s.store_mul_ad_lhs(1238, A::div(A::scale(s.ad_value(227), 2.0), s.ad_value(1229)), 1242);
        }

        if (s.v[1222] != 0.0) {
            s.store_mul_ad_lhs(204, A::mul(A::mul(s.ad_value(1238), s.ad_value(1237)), s.ad_value(107)), 337);
        }

        if (s.v[1222] != 0.0) {
            s.store_add(199, 202, 204);
        }

        s.store_add(201, 203, 204);

        s.v[1293] = if ((p.p43 == 1.0) || (p.p45 == 1.0)) { 1.0 } else { 0.0 };

        s.v[1306] = if ((s.v[145] == 1.0) || (p.p25 == 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1293] != 0.0) && (s.v[1306] != 0.0)) {
            s.store_scalar(263, 0.0);
        }

        s.v[1307] = if ((p.p117 <= 0.0) || (s.v[73] <= 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (s.v[1307] != 0.0)) {
            s.store_scalar(263, 0.0);
        }

        if (((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) {
            s.store_offset_ad(445, A::sub(A::add(A::offset(s.ad_value(174), (-s.v[136])), s.ad_value(185)), s.ad_value(320)), p.p48);
        }

        s.v[1308] = if (p.p44 <= 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (s.v[1308] != 0.0)) {
            s.copy_ad(1294, 445);
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (s.v[1308] != 0.0)) {
            s.store_square(1301, 323);
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (s.v[1308] != 0.0)) {
            s.copy_ad(1302, 545);
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (s.v[1308] != 0.0)) {
            s.store_div(1296, 1302, 1301);
        }

    }

    pub(super) fn stamp_reactive_block_20(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (s.v[1308] != 0.0)) {
            s.store_div_from_scalar(1303, 2.0, 1302);
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (s.v[1308] != 0.0)) {
            s.store_mul(1297, 1303, 1301);
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (s.v[1308] != 0.0)) {
            s.store_sub_ad(1298, A::sub(s.ad_value(1294), s.ad_value(227)), A::mul(s.ad_value(130), s.ad_value(514)));
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (s.v[1308] != 0.0)) {
            s.store_scale(483, 393, (p.p49 * 1.0 / (s.v[89])));
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (s.v[1308] != 0.0)) {
            s.store_sub_ad_rhs(1298, 1298, A::mul(s.ad_value(130), s.ad_value(483)));
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (s.v[1308] != 0.0)) {
            s.store_offset_ad(1300, A::mul(s.ad_value(1297), s.ad_value(1298)), 1.0);
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (s.v[1308] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1300)), ((4.0 * 0.001) * 0.001)));
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (s.v[1308] != 0.0)) {
            s.store_offset_ad(1299, A::scale(A::add(s.ad_value(1300), s.ad_value(44)), 0.5), (1e-10 * 0.001));
        }

        s.v[1309] = if (s.v[1299] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (s.v[1308] != 0.0)) && (s.v[1309] != 0.0)) {
            s.store_scalar(1299, 0.0);
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (s.v[1308] != 0.0)) {
            s.store_offset(1299, 1299, 1e-50);
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (s.v[1308] != 0.0)) {
            s.store_sqrt(1299, 1299);
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (s.v[1308] != 0.0)) {
            s.store_add_ad(1304, A::mul(s.ad_value(1294), s.ad_value(137)), A::mul(s.ad_value(1296), A::sub_from_scalar(1.0, s.ad_value(1299))));
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (s.v[1308] != 0.0)) {
            s.store_sub_ad(1305, A::add(A::scale(s.ad_value(173), p.p122), s.ad_value(176)), A::mul(A::mul(s.ad_value(131), s.ad_value(129)), s.ad_value(1304)));
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (s.v[1308] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1305)), ((4.0 * 0.01) * 0.01)));
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (s.v[1308] != 0.0)) {
            s.store_offset_ad(1305, A::scale(A::add(s.ad_value(1305), s.ad_value(44)), 0.5), (1e-10 * 0.01));
        }

        s.v[1310] = if (s.v[1305] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (s.v[1308] != 0.0)) && (s.v[1310] != 0.0)) {
            s.store_scalar(1305, 0.0);
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) {
            s.store_mul(1294, 134, 445);
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) {
            s.store_div_ad_rhs(1296, 545, A::square(s.ad_value(323)));
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) {
            s.store_mul_ad(1297, A::div_from_scalar(2.0, s.ad_value(545)), A::square(s.ad_value(323)));
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) {
            s.store_sub_ad(1298, A::sub(s.ad_value(1294), s.ad_value(227)), A::mul(s.ad_value(130), s.ad_value(514)));
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) {
            s.store_scale(483, 393, (p.p49 * 1.0 / (s.v[89])));
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) {
            s.store_sub_ad_rhs(1298, 1298, A::mul(s.ad_value(130), s.ad_value(483)));
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) {
            s.store_offset_ad(1299, A::mul(s.ad_value(1297), s.ad_value(1298)), 1.0);
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) {
            s.store_scaled_offset(1301, 1297, 1.0, 2.0);
        }

        s.v[1311] = if ((s.v[1299] < (1e-50 + s.v[1301])) && (s.v[1301] >= 0.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) {
            s.store_sub_ad_lhs(44, A::offset(s.ad_value(1301), 1e-50), 1299);
        }

        if (((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) {
            s.store_square(49, 44);
        }

        if (((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) {
            s.store_square(50, 1301);
        }

        if (((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) {
            s.store_scalar(51, 1.0);
        }

        if (((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) {
            s.store_scalar(52, 1.0);
        }

        if (((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        if (((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) {
            s.store_scalar(55, 0.0);
        }

        if (((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) {
            s.store_scalar(48, 0.0);
        }

        if (((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) {
            s.store_scalar(53, 0.0);
        }

        if (((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if (((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if (((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if (((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if (((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if (((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if (((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if (((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if (((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) {
            s.store_add(48, 51, 52);
        }

        if (((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) {
            s.copy_ad(53, 48);
        }

        s.v[1312] = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1313] = if (4.0 == 1.0) { 1.0 } else { 0.0 };

        if (((((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) && (s.v[1312] != 0.0)) && (s.v[1313] != 0.0)) {
            s.store_scalar(55, 1.0);
        }

        s.v[1314] = if (4.0 == 2.0) { 1.0 } else { 0.0 };

        if ((((((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) && (s.v[1312] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1314] != 0.0)) {
            s.store_scalar(55, 2.0);
        }

        s.v[1315] = if (4.0 == 4.0) { 1.0 } else { 0.0 };

        if (((((((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) && (s.v[1312] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1314] != 0.0))) && (s.v[1315] != 0.0)) {
            s.store_scalar(55, 3.0);
        }

        s.v[1316] = if (4.0 == 8.0) { 1.0 } else { 0.0 };

        if ((((((((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) && (s.v[1312] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1314] != 0.0))) && (!(s.v[1315] != 0.0))) && (s.v[1316] != 0.0)) {
            s.store_scalar(55, 4.0);
        }

        if ((((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) && (s.v[1312] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        let mut assign22620_loop_guard: usize = 0;
        while {
            let assign22620_cond_e31705: f64 = if (((((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) && (s.v[1312] != 0.0)) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign22620_cond_e31705 != 0.0
        } {
            assign22620_loop_guard += 1;
            assert!(assign22620_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) && (s.v[1312] != 0.0)) {
                s.store_sqrt(53, 53);
            }
            if ((((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) && (s.v[1312] != 0.0)) {
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) && (!(s.v[1312] != 0.0))) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if (((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) {
            s.store_div_from_scalar(53, 1.0, 53);
        }

        if (((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) {
            s.store_mul_ad_lhs(43, A::mul(s.ad_value(44), s.ad_value(1301)), 53);
        }

        if (((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1311] != 0.0)) {
            s.store_sub_ad_lhs(1299, A::offset(s.ad_value(1301), 1e-50), 43);
        }

        if (((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (!(s.v[1311] != 0.0))) {
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) {
            s.store_ad(1299, &{
                if (s.v[1299] <= 0.0) {
                    A::constant(0.0)
                } else {
                    A::sqrt(s.ad_value(1299))
                }
            });
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) {
            s.store_add_ad_rhs(1304, 1294, A::mul(s.ad_value(1296), A::sub_from_scalar(1.0, s.ad_value(1299))));
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) {
            s.store_div_from_scalar_ad(1295, s.v[100], A::offset(s.ad_value(131), s.v[100]));
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) {
            s.store_sub_ad(1305, A::add(A::scale(s.ad_value(173), p.p122), s.ad_value(176)), A::mul(s.ad_value(1295), s.ad_value(1304)));
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1305)), ((4.0 * 0.001) * 0.001)));
        }

        if ((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) {
            s.store_offset_ad(1305, A::scale(A::add(s.ad_value(1305), s.ad_value(44)), 0.5), (1e-10 * 0.001));
        }

        s.v[1317] = if (s.v[1305] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1317] != 0.0)) {
            s.store_scalar(1305, 0.0);
        }

        if (((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) {
            s.store_offset(1305, 1305, 1e-50);
        }

        if (((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) {
            s.store_exp_ad(1295, A::div(A::neg(s.ad_value(133)), s.ad_value(1305)));
        }

        if (((s.v[1293] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) {
            s.store_mul_ad_lhs(263, A::mul(A::mul(s.ad_value(132), s.ad_value(1305)), s.ad_value(199)), 1295);
        }

        s.v[1318] = if (((p.p25 == 1.0) && (p.p26 == 2.0)) && (p.p43 == 1.0)) { 1.0 } else { 0.0 };

        if (s.v[1318] != 0.0) {
            s.store_scale(1322, 227, 0.0);
        }

        if (s.v[1318] != 0.0) {
            s.store_sub_ad(44, A::sub(s.ad_value(231), s.ad_value(1322)), A::scale(s.ad_value(231), 0.01));
        }

        if (s.v[1318] != 0.0) {
            s.store_mul_ad(45, A::scale(s.ad_value(231), 4.0), A::scale(s.ad_value(231), 0.01));
        }

        if (s.v[1318] != 0.0) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (s.v[1318] != 0.0) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if (s.v[1318] != 0.0) {
            s.store_sub_ad_rhs(1322, 231, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if (s.v[1318] != 0.0) {
            s.store_sqrt_ad(1323, A::mul(A::scale(s.ad_value(544), ((2.0 * 1.034943e-10) * 1.6021918e-19)), s.ad_value(227)));
        }

        if (s.v[1318] != 0.0) {
            s.store_mul_ad_rhs(1324, 225, A::sub(s.ad_value(176), s.ad_value(1322)));
        }

        if (s.v[1318] != 0.0) {
            s.store_ad(1324, &{
                if (s.v[1324] > 0.0) {
                    A::sqrt(s.ad_value(1324))
                } else {
                    A::neg(A::sqrt(A::neg(s.ad_value(1324))))
                }
            });
        }

        if (s.v[1318] != 0.0) {
            s.store_sqrt_ad(1325, A::mul(s.ad_value(225), s.ad_value(176)));
        }

        if (s.v[1318] != 0.0) {
            s.store_mul_ad(1326, A::neg(s.ad_value(1323)), A::sub(s.ad_value(1324), s.ad_value(1325)));
        }

        if (s.v[1318] != 0.0) {
            s.store_offset_ad(44, A::sub_from_scalar(p.p47, s.ad_value(1326)), (-(p.p47 * 0.01)));
        }

        if (s.v[1318] != 0.0) {
            s.store_scalar(45, ((4.0 * p.p47) * (p.p47 * 0.01)));
        }

        if (s.v[1318] != 0.0) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (s.v[1318] != 0.0) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if (s.v[1318] != 0.0) {
            s.store_sub_from_scalar_ad(393, p.p47, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if (s.v[1318] != 0.0) {
            s.store_ad(596, &A::scale(A::voltage(ctx, &nodes, Some(17), None), (1e-9 / 0.0001)));
        }

        if (s.v[1318] != 0.0) {
            s.copy_ad(393, 596);
        }

        s.v[1340] = if (((s.v[145] == 0.0) && (s.v[263] > 0.0)) && (p.p146 != 0.0)) { 1.0 } else { 0.0 };

        s.v[1341] = if (s.v[56] < 3.0) { 1.0 } else { 0.0 };

        if ((s.v[1340] != 0.0) && (s.v[1341] != 0.0)) {
            s.store_scalar(516, 0.0);
        }

        if ((s.v[1340] != 0.0) && (s.v[1341] != 0.0)) {
            s.store_scalar(517, 0.0);
        }

        if ((s.v[1340] != 0.0) && (!(s.v[1341] != 0.0))) {
            s.store_ad(516, &{
                if (p.p43 == 1.0) {
                    s.ad_value(156)
                } else {
                    s.ad_value(350)
                }
            });
        }

        if ((s.v[1340] != 0.0) && (!(s.v[1341] != 0.0))) {
            s.store_ad(517, &{
                if (p.p43 == 1.0) {
                    s.ad_value(156)
                } else {
                    s.ad_value(353)
                }
            });
        }

        if (s.v[1340] != 0.0) {
            s.store_offset_scaled(1327, 185, p.p147, 1.0);
        }

        if (s.v[1340] != 0.0) {
            s.store_mul_ad_lhs(1328, A::scale(s.ad_value(1327), p.p146), 263);
        }

        if (s.v[1340] != 0.0) {
            s.store_offset_ad(1329, A::mul(s.ad_value(225), A::sub(s.ad_value(161), s.ad_value(516))), (-1.0));
        }

        if (s.v[1340] != 0.0) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1329)), ((4.0 * 0.1) * 0.1)));
        }

        if (s.v[1340] != 0.0) {
            s.store_offset_ad(1329, A::scale(A::add(s.ad_value(1329), s.ad_value(44)), 0.5), (1e-10 * 0.1));
        }

        s.v[1342] = if (s.v[1329] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1340] != 0.0) && (s.v[1342] != 0.0)) {
            s.store_scalar(1329, 0.0);
        }

        if (s.v[1340] != 0.0) {
            s.store_sqrt(1330, 1329);
        }

        if (s.v[1340] != 0.0) {
            s.store_mul(1331, 1329, 1330);
        }

        if (s.v[1340] != 0.0) {
            s.store_offset_ad(1332, A::mul(s.ad_value(225), A::sub(s.ad_value(162), s.ad_value(517))), (-1.0));
        }

        if (s.v[1340] != 0.0) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1332)), ((4.0 * 0.1) * 0.1)));
        }

        if (s.v[1340] != 0.0) {
            s.store_offset_ad(1332, A::scale(A::add(s.ad_value(1332), s.ad_value(44)), 0.5), (1e-10 * 0.1));
        }

        s.v[1343] = if (s.v[1332] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1340] != 0.0) && (s.v[1343] != 0.0)) {
            s.store_scalar(1332, 0.0);
        }

        if (s.v[1340] != 0.0) {
            s.store_sqrt(1333, 1332);
        }

        if (s.v[1340] != 0.0) {
            s.store_mul(1334, 1332, 1333);
        }

        if (s.v[1340] != 0.0) {
            s.store_div_from_scalar(1335, 1.0, 1329);
        }

        if (s.v[1340] != 0.0) {
            s.store_mul_ad_lhs(328, A::mul(s.ad_value(225), s.ad_value(1328)), 1335);
        }

        if (s.v[1340] != 0.0) {
            s.store_div_from_scalar(1335, 1.0, 1332);
        }

        if (s.v[1340] != 0.0) {
            s.store_mul_ad_lhs(1336, A::mul(s.ad_value(225), s.ad_value(1328)), 1335);
        }

        if (s.v[1340] != 0.0) {
            s.store_mul_ad_rhs(1337, 238, A::sub(A::mul(s.ad_value(1334), s.ad_value(1336)), A::mul(s.ad_value(1331), s.ad_value(328))));
        }

        if (s.v[1340] != 0.0) {
            s.store_mul_ad(1338, A::scale(s.ad_value(238), 0.5), A::add(A::mul(A::neg(s.ad_value(1333)), s.ad_value(1336)), A::mul(s.ad_value(1330), s.ad_value(328))));
        }

        if (s.v[1340] != 0.0) {
            s.store_add(1339, 1337, 1338);
        }

        if (s.v[1340] != 0.0) {
            s.store_mul_ad_lhs(265, A::mul(s.ad_value(264), s.ad_value(1339)), 250);
        }

        s.v[1357] = (s.v[88] * 100.0);

        s.store_scale(1358, 323, 0.0001);

        s.v[1359] = (s.v[97] * 100.0);

    }

    pub(super) fn stamp_reactive_block_21(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.store_scale(1360, 107, 100.0);

        s.store_scale(1361, 252, 0.01);

        s.store_scale(1362, 436, 0.0001);

        s.store_scale(1363, 238, 0.0001);

        s.v[1364] = if (p.p27 == 0.0) { 1.0 } else { 0.0 };

        s.v[1365] = if (s.v[145] == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1364] != 0.0)) && (s.v[1365] != 0.0)) {
            s.store_offset_ad(1356, A::add(s.ad_value(176), s.ad_value(173)), (-(10.0 * 2.220446049250313e-16)));
        }

        if ((!(s.v[1364] != 0.0)) && (s.v[1365] != 0.0)) {
            s.store_sub_ad(1346, A::add(A::offset(s.ad_value(174), (-s.v[123])), A::scale(A::sub(s.ad_value(185), s.ad_value(320)), (p.p216 * s.v[1359]))), A::scale(s.ad_value(1356), p.p215));
        }

        if ((!(s.v[1364] != 0.0)) && (s.v[1365] != 0.0)) {
            s.store_scalar(1348, (1.0 / s.v[1357]));
        }

        if ((!(s.v[1364] != 0.0)) && (s.v[1365] != 0.0)) {
            s.store_mul(1347, 1346, 1348);
        }

        if ((!(s.v[1364] != 0.0)) && (s.v[1365] != 0.0)) {
            s.store_scalar(1348, (1.0 / p.p217));
        }

        if ((!(s.v[1364] != 0.0)) && (s.v[1365] != 0.0)) {
            s.store_offset_ad(1352, A::mul(s.ad_value(1361), s.ad_value(1348)), 1.0);
        }

        if ((!(s.v[1364] != 0.0)) && (s.v[1365] != 0.0)) {
            s.store_mul(1355, 1347, 1352);
        }

        if ((!(s.v[1364] != 0.0)) && (s.v[1365] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1355)), ((4.0 * 0.01) * 0.01)));
        }

        if ((!(s.v[1364] != 0.0)) && (s.v[1365] != 0.0)) {
            s.store_offset_ad(1355, A::scale(A::add(s.ad_value(1355), s.ad_value(44)), 0.5), (1e-10 * 0.01));
        }

        s.v[1366] = if (s.v[1355] < 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1364] != 0.0)) && (s.v[1365] != 0.0)) && (s.v[1366] != 0.0)) {
            s.store_scalar(1355, 0.0);
        }

        if ((!(s.v[1364] != 0.0)) && (s.v[1365] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(174)), ((4.0 * 0.001) * 0.001)));
        }

        if ((!(s.v[1364] != 0.0)) && (s.v[1365] != 0.0)) {
            s.store_offset_ad(1348, A::scale(A::add(s.ad_value(174), s.ad_value(44)), 0.5), (1e-10 * 0.001));
        }

        s.v[1367] = if (s.v[1348] < 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1364] != 0.0)) && (s.v[1365] != 0.0)) && (s.v[1367] != 0.0)) {
            s.store_scalar(1348, 0.0);
        }

        if ((!(s.v[1364] != 0.0)) && (s.v[1365] != 0.0)) {
            s.store_offset(1348, 1348, (-p.p226));
        }

        if ((!(s.v[1364] != 0.0)) && (s.v[1365] != 0.0)) {
            s.store_scale(1344, 1348, 10.0);
        }

        if ((!(s.v[1364] != 0.0)) && (s.v[1365] != 0.0)) {
            s.store_offset_ad(1347, A::square(s.ad_value(1344)), 1.0);
        }

        if ((!(s.v[1364] != 0.0)) && (s.v[1365] != 0.0)) {
            s.store_sub_from_scalar_ad(1346, 1.0, A::div_from_scalar(1.0, s.ad_value(1347)));
        }

        if ((!(s.v[1364] != 0.0)) && (s.v[1365] != 0.0)) {
            s.store_mul(1355, 1355, 1346);
        }

        if ((!(s.v[1364] != 0.0)) && (s.v[1365] != 0.0)) {
            s.store_scale(1345, 1360, s.v[1359]);
        }

        if ((!(s.v[1364] != 0.0)) && (s.v[1365] != 0.0)) {
            s.store_div_from_scalar_ad(1352, p.p219, A::offset(s.ad_value(1345), p.p219));
        }

        if ((!(s.v[1364] != 0.0)) && (s.v[1365] != 0.0)) {
            s.store_scalar(1351, p.p218);
        }

        if ((!(s.v[1364] != 0.0)) && (s.v[1365] != 0.0)) {
            s.store_div_from_scalar_ad(1349, 1.0, A::offset(s.ad_value(1355), 1e-50));
        }

        if ((!(s.v[1364] != 0.0)) && (s.v[1365] != 0.0)) {
            s.store_mul_ad_lhs(1346, A::scale(s.ad_value(303), (-p.p214)), 1349);
        }

        s.v[1368] = if (s.v[1346] < (-34.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1364] != 0.0)) && (s.v[1365] != 0.0)) && (!(s.v[1368] != 0.0))) {
            s.store_exp(1347, 1346);
        }

        if (((!(s.v[1364] != 0.0)) && (s.v[1365] != 0.0)) && (!(s.v[1368] != 0.0))) {
            s.store_mul_ad_lhs(1348, A::scale(A::div_from_scalar(p.p213, s.ad_value(302)), 1.6021918e-19), 1345);
        }

        if (((!(s.v[1364] != 0.0)) && (s.v[1365] != 0.0)) && (!(s.v[1368] != 0.0))) {
            s.store_div_from_scalar(1350, 1.0, 1363);
        }

        if (((!(s.v[1364] != 0.0)) && (s.v[1365] != 0.0)) && (!(s.v[1368] != 0.0))) {
            s.store_sqrt_ad(1351, A::mul(A::add(s.ad_value(1362), A::scale(s.ad_value(1358), 1e-12)), s.ad_value(1350)));
        }

        if (((!(s.v[1364] != 0.0)) && (s.v[1365] != 0.0)) && (!(s.v[1368] != 0.0))) {
            s.store_mul_ad_lhs(1349, A::mul(s.ad_value(1347), s.ad_value(1348)), 1351);
        }

        if (!(s.v[1364] != 0.0)) {
            s.store_offset_scaled(1345, 158, (-p.p221), p.p222);
        }

        if (!(s.v[1364] != 0.0)) {
            s.store_exp_ad(1347, A::scale(s.ad_value(1345), s.v[1357]));
        }

        if (!(s.v[1364] != 0.0)) {
            s.store_scale_ad(1345, A::scale(s.ad_value(158), 1.0 / (s.v[1357])), 1.0 / (s.v[1357]));
        }

        if (!(s.v[1364] != 0.0)) {
            s.store_mul(1348, 158, 1345);
        }

        if (!(s.v[1364] != 0.0)) {
            s.store_scale(1349, 1360, (p.p220 / 1000000.0));
        }

        if (!(s.v[1364] != 0.0)) {
            s.store_sub(1346, 158, 157);
        }

        if (!(s.v[1364] != 0.0)) {
            s.store_offset_scaled(1345, 1346, (-p.p221), p.p222);
        }

        if (!(s.v[1364] != 0.0)) {
            s.store_exp_ad(1347, A::scale(s.ad_value(1345), s.v[1357]));
        }

        if (!(s.v[1364] != 0.0)) {
            s.store_scale_ad(1345, A::scale(s.ad_value(1346), 1.0 / (s.v[1357])), 1.0 / (s.v[1357]));
        }

        if (!(s.v[1364] != 0.0)) {
            s.store_mul(1348, 1346, 1345);
        }

        if (!(s.v[1364] != 0.0)) {
            s.store_scale(1349, 1360, (p.p220 / 1000000.0));
        }

        if (!(s.v[1364] != 0.0)) {
            s.store_scale_ad(1355, A::offset(A::offset(A::sub(s.ad_value(513), s.ad_value(158)), s.v[123]), p.p225), 1.0 / (s.v[1357]));
        }

        if (!(s.v[1364] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1355)), ((4.0 * 0.01) * 0.01)));
        }

        if (!(s.v[1364] != 0.0)) {
            s.store_offset_ad(1355, A::scale(A::add(s.ad_value(1355), s.ad_value(44)), 0.5), (1e-10 * 0.01));
        }

        s.v[1371] = if (s.v[1355] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1364] != 0.0)) && (s.v[1371] != 0.0)) {
            s.store_scalar(1355, 0.0);
        }

        if (!(s.v[1364] != 0.0)) {
            s.store_offset(1355, 1355, 1e-50);
        }

        if (!(s.v[1364] != 0.0)) {
            s.store_div_from_scalar(1346, (-p.p224), 1355);
        }

        s.v[1372] = if (s.v[1346] < (-34.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1364] != 0.0)) && (!(s.v[1372] != 0.0))) {
            s.store_exp(1347, 1346);
        }

        if ((!(s.v[1364] != 0.0)) && (!(s.v[1372] != 0.0))) {
            s.store_scale(1348, 1360, (p.p223 * s.v[1359]));
        }

        s.v[1380] = if (p.p28 == 0.0) { 1.0 } else { 0.0 };

        if (!(s.v[1380] != 0.0)) {
            s.store_add_ad(1373, A::sub(A::scale(A::offset(s.ad_value(157), p.p210), p.p209), s.ad_value(158)), A::scale(A::add(s.ad_value(187), s.ad_value(319)), p.p211));
        }

        if (!(s.v[1380] != 0.0)) {
            s.store_scalar(1374, (1.0 / s.v[88]));
        }

        if (!(s.v[1380] != 0.0)) {
            s.store_mul(1375, 1373, 1374);
        }

        if (!(s.v[1380] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1375)), ((4.0 * 0.01) * 0.01)));
        }

        if (!(s.v[1380] != 0.0)) {
            s.store_offset_ad(304, A::scale(A::add(s.ad_value(1375), s.ad_value(44)), 0.5), (1e-10 * 0.01));
        }

        s.v[1381] = if (s.v[304] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1380] != 0.0)) && (s.v[1381] != 0.0)) {
            s.store_scalar(304, 0.0);
        }

        if (!(s.v[1380] != 0.0)) {
            s.store_div_from_scalar_ad(1376, 1.0, A::offset(s.ad_value(304), 1e-50));
        }

        if (!(s.v[1380] != 0.0)) {
            s.store_mul_ad_lhs(1377, A::scale(s.ad_value(303), (-p.p208)), 1376);
        }

        s.v[1382] = if (s.v[1377] < (-34.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1380] != 0.0)) && (!(s.v[1382] != 0.0))) {
            s.store_exp(1373, 1377);
        }

        if ((!(s.v[1380] != 0.0)) && (!(s.v[1382] != 0.0))) {
            s.store_mul_ad_lhs(1374, A::scale(A::div_from_scalar(p.p207, s.ad_value(302)), 1.6021918e-19), 107);
        }

        if (!(s.v[1380] != 0.0)) {
            s.store_sub(1379, 157, 513);
        }

        s.v[1383] = if (s.v[1379] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1380] != 0.0)) && (s.v[1383] != 0.0)) {
            s.store_square(1374, 1379);
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1383] != 0.0)) {
            s.store_mul(331, 1374, 1379);
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1383] != 0.0)) {
            s.store_offset(1377, 331, p.p212);
        }

        s.v[1391] = if (p.p28 == 0.0) { 1.0 } else { 0.0 };

        if (!(s.v[1391] != 0.0)) {
            s.store_add_ad(1384, A::sub(A::scale(A::sub_from_scalar(p.p210, s.ad_value(157)), p.p209), A::sub(s.ad_value(158), s.ad_value(157))), A::scale(A::add(s.ad_value(187), s.ad_value(319)), p.p211));
        }

        if (!(s.v[1391] != 0.0)) {
            s.store_scalar(1385, (1.0 / s.v[88]));
        }

        if (!(s.v[1391] != 0.0)) {
            s.store_mul(1386, 1384, 1385);
        }

        if (!(s.v[1391] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1386)), ((4.0 * 0.01) * 0.01)));
        }

        if (!(s.v[1391] != 0.0)) {
            s.store_offset_ad(305, A::scale(A::add(s.ad_value(1386), s.ad_value(44)), 0.5), (1e-10 * 0.01));
        }

        s.v[1392] = if (s.v[305] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1391] != 0.0)) && (s.v[1392] != 0.0)) {
            s.store_scalar(305, 0.0);
        }

        if (!(s.v[1391] != 0.0)) {
            s.store_div_from_scalar_ad(1387, 1.0, A::offset(s.ad_value(305), 1e-50));
        }

        if (!(s.v[1391] != 0.0)) {
            s.store_mul_ad_lhs(1388, A::scale(s.ad_value(303), (-p.p208)), 1387);
        }

        s.v[1393] = if (s.v[1388] < (-34.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1391] != 0.0)) && (!(s.v[1393] != 0.0))) {
            s.store_exp(1384, 1388);
        }

        if ((!(s.v[1391] != 0.0)) && (!(s.v[1393] != 0.0))) {
            s.store_div_from_scalar(1387, 1.0, 302);
        }

        if ((!(s.v[1391] != 0.0)) && (!(s.v[1393] != 0.0))) {
            s.store_mul_ad_lhs(1385, A::scale(s.ad_value(1387), (p.p207 * 1.6021918e-19)), 107);
        }

        if (!(s.v[1391] != 0.0)) {
            s.store_neg(1390, 513);
        }

        s.v[1394] = if (s.v[1390] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1391] != 0.0)) && (s.v[1394] != 0.0)) {
            s.store_square(1385, 1390);
        }

        if ((!(s.v[1391] != 0.0)) && (s.v[1394] != 0.0)) {
            s.store_mul(331, 1385, 1390);
        }

        if ((!(s.v[1391] != 0.0)) && (s.v[1394] != 0.0)) {
            s.store_offset(1388, 331, p.p212);
        }

        s.v[1395] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1395] != 0.0) {
            s.store_scalar(1405, s.v[91]);
        }

        if (s.v[1395] != 0.0) {
            s.store_div_from_scalar(1406, 1.0, 1405);
        }

        if (s.v[1395] != 0.0) {
            s.store_scalar(1462, 0.0);
        }

        if (s.v[1395] != 0.0) {
            s.store_scalar(1464, 0.0);
        }

        if (s.v[1395] != 0.0) {
            s.store_scalar(1466, 0.0);
        }

        if (s.v[1395] != 0.0) {
            s.store_neg(1398, 534);
        }

        if (s.v[1395] != 0.0) {
            s.store_mul(1399, 1398, 436);
        }

        if (s.v[1395] != 0.0) {
            s.store_add_ad_rhs(331, 1399, A::mul(s.ad_value(1398), s.ad_value(437)));
        }

        if (s.v[1395] != 0.0) {
            s.store_mul(470, 1399, 438);
        }

        if (s.v[1395] != 0.0) {
            s.store_sub(469, 1399, 470);
        }

        if (s.v[1395] != 0.0) {
            s.store_mul(468, 331, 438);
        }

        if (s.v[1395] != 0.0) {
            s.store_sub(467, 331, 468);
        }

        if ((s.v[1395] != 0.0) && (p.p24 != 0.0)) {
            s.copy_ad(521, 536);
        }

        if ((s.v[1395] != 0.0) && (p.p24 != 0.0)) {
            s.store_scalar(528, 0.0);
        }

        s.v[1475] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1476] = if (1.0 == 2.0) { 1.0 } else { 0.0 };

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_scale(522, 533, 0.5);
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_scalar(523, p.p292);
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_scalar(528, s.v[525]);
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && ((s.v[1476] != 0.0) && (!(s.v[1475] != 0.0)))) {
            s.store_scale(522, 534, 0.5);
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && ((s.v[1476] != 0.0) && (!(s.v[1475] != 0.0)))) {
            s.store_scalar(523, p.p68);
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && ((s.v[1476] != 0.0) && (!(s.v[1475] != 0.0)))) {
            s.store_scalar(528, s.v[524]);
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && ((s.v[1476] != 0.0) && (!(s.v[1475] != 0.0)))) {
            s.store_scalar(528, 1.0);
        }

        s.v[1477] = if (s.v[528] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_mul_ad_rhs(1425, 238, A::sqrt(A::div(s.ad_value(521), s.ad_value(536))));
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_scalar(1407, ((1.0 - -1.0) / 2.0));
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_scalar(1408, ((1.0 + -1.0) / 2.0));
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_add_ad(1418, A::mul(s.ad_value(461), s.ad_value(156)), A::mul(s.ad_value(462), A::sub(s.ad_value(156), s.ad_value(157))));
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_add_ad(1419, A::mul(s.ad_value(461), s.ad_value(157)), A::mul(s.ad_value(462), A::neg(s.ad_value(157))));
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_add_ad(1420, A::mul(s.ad_value(461), s.ad_value(158)), A::mul(s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_add_ad(1421, A::mul(s.ad_value(462), s.ad_value(158)), A::mul(s.ad_value(461), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_sub(1422, 1419, 1418);
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_neg(1423, 1418);
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_add_ad(1409, A::mul(s.ad_value(1407), s.ad_value(461)), A::mul(s.ad_value(1408), s.ad_value(462)));
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_add_ad(1410, A::mul(s.ad_value(1407), s.ad_value(462)), A::mul(s.ad_value(1408), s.ad_value(461)));
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_add_ad(1424, A::mul(s.ad_value(1409), s.ad_value(1420)), A::mul(s.ad_value(1410), s.ad_value(1421)));
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_offset_ad(1416, A::add(A::mul(s.ad_value(1409), s.ad_value(1423)), A::mul(s.ad_value(1410), s.ad_value(1422))), (10.0 * 2.220446049250313e-16));
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_neg(1396, 1416);
        }

        s.v[1478] = if (s.v[1396] > s.v[141]) { 1.0 } else { 0.0 };

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1478] != 0.0)) {
            s.store_sub(1397, 1396, 141);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1478] != 0.0)) {
            s.store_sub(1398, 140, 141);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1478] != 0.0)) {
            s.store_div(44, 1397, 1398);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1478] != 0.0)) {
            s.store_square(45, 44);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1478] != 0.0)) {
            s.store_mul(46, 45, 44);
        }

    }

    pub(super) fn stamp_reactive_block_22(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1478] != 0.0)) {
            s.store_square(47, 45);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1478] != 0.0)) {
            s.store_div_from_scalar_ad(1404, 1.0, A::add(A::add(A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(45)), s.ad_value(46)), s.ad_value(47)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1478] != 0.0)) {
            s.store_mul_ad_rhs(1404, 1398, A::sub_from_scalar(1.0, s.ad_value(1404)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1478] != 0.0)) {
            s.store_add(1401, 141, 1404);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1478] != 0.0))) {
            s.copy_ad(1401, 1396);
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_offset_ad(1417, A::neg(s.ad_value(1401)), (-1e-12));
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_mul(1426, 1425, 1406);
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_square(1427, 1426);
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_sub(1428, 1424, 523);
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_div(1396, 521, 230);
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_mul_ad(1429, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(s.ad_value(1396)));
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_neg(1430, 1417);
        }

        s.v[1479] = if (s.v[1428] < s.v[1430]) { 1.0 } else { 0.0 };

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_div_from_scalar_ad(1397, 1.0, A::mul(s.ad_value(225), s.ad_value(1425)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_mul(1404, 1397, 1405);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_offset_scaled(1431, 1404, (3.0 * 1.414213562373095), 2.0);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_mul_ad_lhs(1432, A::mul(A::scale(s.ad_value(1431), 8.0), s.ad_value(1431)), 1431);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_sub(1433, 237, 1429);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_mul_ad_rhs(1403, 225, A::add(s.ad_value(1428), s.ad_value(1417)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_sub_from_scalar_ad(1434, (7.0 * 1.414213562373095), A::mul(A::scale(s.ad_value(1404), 9.0), A::offset(s.ad_value(1403), (-2.0))));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_square(1435, 1434);
        }

        s.v[1480] = if (s.v[1432] < (s.v[1435] * 1e-8)) { 1.0 } else { 0.0 };

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1480] != 0.0)) {
            s.store_add_ad(1437, A::add(A::offset(s.ad_value(1434), ((-7.0) * 1.414213562373095)), A::div(A::scale(s.ad_value(1432), 0.5), s.ad_value(1434))), A::mul(A::scale(s.ad_value(1404), 9.0), A::offset(s.ad_value(1403), (-2.0))));
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1480] != 0.0))) {
            s.store_sqrt_ad(1436, A::add(s.ad_value(1432), s.ad_value(1435)));
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1480] != 0.0))) {
            s.store_add_ad(1437, A::offset(s.ad_value(1436), ((-7.0) * 1.414213562373095)), A::mul(A::scale(s.ad_value(1404), 9.0), A::offset(s.ad_value(1403), (-2.0))));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_powf(1438, 1437, 0.3333333333333333);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_add_ad(1439, A::add(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1404), 12.0)), A::scale(s.ad_value(1438), 2.0)), A::mul(A::scale(s.ad_value(1438), 1.414213562373095), s.ad_value(1438)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_div(1440, 1439, 1438);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_sub_ad_lhs(1441, A::mul(s.ad_value(1440), s.ad_value(227)), 1417);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_add(1397, 1441, 1417);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_div(1398, 1397, 1433);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_sqrt_ad(1399, A::offset(A::square(s.ad_value(1398)), 1.0));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_sub_ad_lhs(1442, A::div(s.ad_value(1397), s.ad_value(1399)), 1417);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_sub(1398, 1428, 1442);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_mul(459, 1405, 1398);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) {
            s.copy_ad(458, 459);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_scalar(1440, 3.0);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_sub_ad_lhs(1443, A::div(s.ad_value(1440), s.ad_value(225)), 1417);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_exp_ad(1404, A::neg(s.ad_value(1440)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_offset_ad(1403, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1428), s.ad_value(1417))), (-1.0)), s.ad_value(1404)), 4.0), A::mul(s.ad_value(1427), s.ad_value(226))), 1.0);
        }

        s.v[1481] = if (s.v[1403] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1481] != 0.0)) {
            s.store_scalar(1403, (10.0 * 2.220446049250313e-16));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_add_ad_rhs(1443, 1428, A::mul(A::scale(A::mul(s.ad_value(1427), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1403)))));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_mul_ad_rhs(1440, 225, A::add(s.ad_value(1443), s.ad_value(1417)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_exp_ad(1404, A::neg(s.ad_value(1440)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_offset_ad(1403, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1428), s.ad_value(1417))), (-1.0)), s.ad_value(1404)), 4.0), A::mul(s.ad_value(1427), s.ad_value(226))), 1.0);
        }

        s.v[1482] = if (s.v[1403] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1482] != 0.0)) {
            s.store_scalar(1403, (10.0 * 2.220446049250313e-16));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_add_ad_rhs(1443, 1428, A::mul(A::scale(A::mul(s.ad_value(1427), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1403)))));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_mul_ad_rhs(1440, 225, A::add(s.ad_value(1443), s.ad_value(1417)));
        }

        s.v[1483] = if (s.v[1440] < 3.0) { 1.0 } else { 0.0 };

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1483] != 0.0)) {
            s.store_scalar(1444, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1483] != 0.0)) {
            s.store_scalar(1445, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1483] != 0.0)) {
            s.store_offset_ad(1446, A::div_from_scalar(1.0, A::mul(s.ad_value(225), s.ad_value(1426))), (1.0 / 1.414213562373095));
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1483] != 0.0)) {
            s.store_div_ad_lhs(1447, A::neg(A::add(s.ad_value(1428), s.ad_value(1417))), 1426);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1483] != 0.0)) {
            s.store_add_ad(1448, A::sub(A::div(A::mul(A::square(s.ad_value(1445)), s.ad_value(1445)), A::mul(A::mul(A::scale(s.ad_value(1444), 27.0), s.ad_value(1444)), s.ad_value(1444))), A::div(A::mul(s.ad_value(1445), s.ad_value(1446)), A::mul(A::scale(s.ad_value(1444), 6.0), s.ad_value(1444)))), A::div(s.ad_value(1447), A::scale(s.ad_value(1444), 2.0)));
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1483] != 0.0)) {
            s.store_div_ad(1449, A::sub(A::mul(A::scale(s.ad_value(1444), 3.0), s.ad_value(1446)), A::square(s.ad_value(1445))), A::mul(A::scale(s.ad_value(1444), 9.0), s.ad_value(1444)));
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1483] != 0.0)) {
            s.store_sqrt_ad(1400, A::add(A::square(s.ad_value(1448)), A::mul(A::square(s.ad_value(1449)), s.ad_value(1449))));
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1483] != 0.0)) {
            s.store_powf_ad(1450, A::sub(s.ad_value(1400), s.ad_value(1448)), 0.3333333333333333);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1483] != 0.0)) {
            s.store_neg_ad(1451, A::powf(A::add(s.ad_value(1448), s.ad_value(1400)), 0.3333333333333333));
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1483] != 0.0)) {
            s.store_sub_ad(1403, A::add(s.ad_value(1450), s.ad_value(1451)), A::div(s.ad_value(1445), A::scale(s.ad_value(1444), 3.0)));
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1483] != 0.0)) {
            s.store_sub_ad_lhs(1443, A::mul(s.ad_value(1403), s.ad_value(227)), 1417);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1483] != 0.0)) {
            s.store_mul_ad_rhs(1440, 225, A::add(s.ad_value(1443), s.ad_value(1417)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_offset_ad(1452, A::add(s.ad_value(1428), s.ad_value(1417)), 0.1);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_offset_ad(1459, A::exp(A::mul(s.ad_value(225), A::neg(s.ad_value(1417)))), 1e-50);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_div(1396, 230, 521);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_square(1453, 1396);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_mul(1454, 1453, 1459);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_mul(1396, 226, 1427);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_mul(1455, 225, 1452);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_add_ad(1456, A::sub(A::ln(A::add(A::mul(s.ad_value(1454), s.ad_value(1396)), A::square(s.ad_value(1455)))), A::ln(A::mul(s.ad_value(1453), s.ad_value(1396)))), A::mul(s.ad_value(225), s.ad_value(1417)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_offset_ad(44, A::sub(s.ad_value(1455), s.ad_value(1456)), (-1.0));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_scale(45, 1455, 4.0);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_scale_ad(1397, A::offset(A::div(s.ad_value(44), s.ad_value(45)), 1.0), 0.5);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_scale_ad(1398, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(44), 2.0), s.ad_value(45))), 0.5);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_sub_ad_rhs(1456, 1455, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_sub(1455, 1455, 1456);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_add_ad_rhs(1455, 1455, A::scale(s.ad_value(225), 0.1));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_add_ad(1457, A::sub(A::ln(A::add(A::mul(s.ad_value(1454), s.ad_value(1396)), A::square(s.ad_value(1455)))), A::ln(A::mul(s.ad_value(1453), s.ad_value(1396)))), A::mul(s.ad_value(225), s.ad_value(1417)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.copy_ad(1458, 1440);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_offset_ad(44, A::sub(s.ad_value(1457), s.ad_value(1458)), (-(0.0008 * 75.0)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_scale(45, 1457, (4.0 * (0.0008 * 75.0)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_scale_ad(1397, A::offset(A::div(s.ad_value(44), s.ad_value(45)), 1.0), 0.5);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_scale_ad(1398, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(44), ((2.0 * 0.0008) * 75.0)), s.ad_value(45))), 0.5);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_sub_ad_rhs(1440, 1457, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_sub_ad_lhs(1442, A::div(s.ad_value(1440), s.ad_value(225)), 1417);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_add_ad(1397, A::offset(s.ad_value(1440), (-1.0)), A::exp(A::neg(s.ad_value(1440))));
        }

        s.v[1484] = if (s.v[1397] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1484] != 0.0)) {
            s.store_scalar(1397, (10.0 * 2.220446049250313e-16));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_sqrt(1398, 1397);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_mul(458, 1425, 1398);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) {
            s.store_mul_ad_rhs(459, 1405, A::sub(s.ad_value(1428), s.ad_value(1442)));
        }

        s.v[1485] = if (p.p42 == 1.0) { 1.0 } else { 0.0 };

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) {
            s.store_exp_ad(1459, A::mul(s.ad_value(225), A::neg(s.ad_value(1417))));
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) {
            s.store_div(1396, 230, 521);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) {
            s.store_square(1453, 1396);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) {
            s.store_mul(1468, 1453, 1459);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) {
            s.store_scalar(1413, 0.0);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) {
            s.store_scalar(167, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_23(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let mut assign26220_loop_guard: usize = 0;
        while {
            let assign26220_cond_e35796: f64 = (2.0 * 20.0);
            let assign26220_cond_e35798: f64 = (assign26220_cond_e35796 + 1.0);
            let assign26220_cond_e35800: f64 = if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) && (s.v[167] <= assign26220_cond_e35798)) { 1.0 } else { 0.0 };
            assign26220_cond_e35800 != 0.0
        } {
            assign26220_loop_guard += 1;
            assert!(assign26220_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) {
                s.store_scalar(1464, 0.0);
            }
            if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) {
                s.store_mul_ad_rhs(1440, 225, A::add(s.ad_value(1442), s.ad_value(1417)));
            }
            s.v[1486] = if (s.v[1440] < 5.0) { 1.0 } else { 0.0 };
            if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) && (s.v[1486] != 0.0)) {
                s.store_mul_ad(1460, A::mul(A::square(s.ad_value(1440)), s.ad_value(1440)), A::offset(A::mul(s.ad_value(1440), A::offset(A::scale(s.ad_value(1440), 0.006115288895133179), (-0.07053654284009761))), 0.29693154855771));
            }
            if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) && (s.v[1486] != 0.0)) {
                s.store_mul_ad(1461, A::square(s.ad_value(1440)), A::offset(A::mul(s.ad_value(1440), A::offset(A::scale(s.ad_value(1440), (5.0 * 0.006115288895133179)), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
            }
            if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) && (s.v[1486] != 0.0)) {
                s.store_mul_ad_lhs(1462, A::mul(s.ad_value(1468), s.ad_value(1460)), 1460);
            }
            if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) && (s.v[1486] != 0.0)) {
                s.store_mul_ad_lhs(1463, A::mul(A::scale(A::mul(s.ad_value(1468), s.ad_value(225)), 2.0), s.ad_value(1460)), 1461);
            }
            if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) && (s.v[1486] != 0.0)) {
                s.store_mul_ad_rhs(1464, 1440, A::offset(A::mul(s.ad_value(1440), A::offset(A::mul(s.ad_value(1440), A::offset(A::mul(s.ad_value(1440), A::offset(A::scale(s.ad_value(1440), 6.36964918866352e-5), (-0.00163730162779191))), 0.0178800506338833)), (-0.117851130197758))), 0.707106781186548));
            }
            if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) && (s.v[1486] != 0.0)) {
                s.store_offset_ad(1465, A::mul(s.ad_value(1440), A::offset(A::mul(s.ad_value(1440), A::offset(A::mul(s.ad_value(1440), A::offset(A::scale(s.ad_value(1440), (5.0 * 6.36964918866352e-5)), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833))), (2.0 * (-0.117851130197758)))), 0.707106781186548);
            }
            if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) && (s.v[1486] != 0.0)) {
                s.store_sqrt_ad(1466, A::offset(A::add(A::square(s.ad_value(1464)), s.ad_value(1462)), 1e-50));
            }
            if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) && (s.v[1486] != 0.0)) {
                s.store_div_ad(1467, A::add(A::mul(A::scale(A::mul(s.ad_value(225), s.ad_value(1465)), 2.0), s.ad_value(1464)), s.ad_value(1463)), A::scale(s.ad_value(1466), 2.0));
            }
            s.v[1487] = if (s.v[1440] < 80.0) { 1.0 } else { 0.0 };
            if (((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) && (!(s.v[1486] != 0.0))) && (s.v[1487] != 0.0)) {
                s.store_exp(243, 1440);
            }
            if (((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) && (!(s.v[1486] != 0.0))) && (s.v[1487] != 0.0)) {
                s.store_mul_ad_rhs(1462, 1468, A::offset(s.ad_value(243), (-1.0)));
            }
            if (((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) && (!(s.v[1486] != 0.0))) && (s.v[1487] != 0.0)) {
                s.store_mul_ad_lhs(1463, A::mul(s.ad_value(1468), s.ad_value(225)), 243);
            }
            if (((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) && (!(s.v[1486] != 0.0))) && (!(s.v[1487] != 0.0))) {
                s.store_exp_ad(1469, A::mul(s.ad_value(225), s.ad_value(1442)));
            }
            if (((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) && (!(s.v[1486] != 0.0))) && (!(s.v[1487] != 0.0))) {
                s.store_mul_ad_rhs(1462, 1453, A::sub(s.ad_value(1469), s.ad_value(1459)));
            }
            if (((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) && (!(s.v[1486] != 0.0))) && (!(s.v[1487] != 0.0))) {
                s.store_mul_ad_lhs(1463, A::mul(s.ad_value(1453), s.ad_value(225)), 1469);
            }
            if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) && (!(s.v[1486] != 0.0))) {
                s.store_sqrt_ad(1466, A::add(A::offset(s.ad_value(1440), (-1.0)), s.ad_value(1462)));
            }
            if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) && (!(s.v[1486] != 0.0))) {
                s.store_scale_ad(1467, A::div(A::add(s.ad_value(225), s.ad_value(1463)), s.ad_value(1466)), 0.5);
            }
            if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) {
                s.store_sub_ad(1470, A::sub(s.ad_value(1428), s.ad_value(1442)), A::mul(s.ad_value(1426), s.ad_value(1466)));
            }
            if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) {
                s.store_sub_from_scalar_ad(1471, (-1.0), A::mul(s.ad_value(1426), s.ad_value(1467)));
            }
            s.v[1488] = if (s.v[1413] == 1.0) { 1.0 } else { 0.0 };
            if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) && (s.v[1488] != 0.0)) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) && (!(s.v[1488] != 0.0))) {
                s.store_div_ad_lhs(494, A::neg(s.ad_value(1470)), 1471);
            }
            if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) && (!(s.v[1488] != 0.0))) {
                s.store_scale_ad(1472, A::offset({
                    if (1.0 >= ((s.v[1442]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1442))
                    }
                }, 1.0), (0.5 * 0.1));
            }
            s.v[1489] = if (((s.v[494]) as f64).abs() > s.v[1472]) { 1.0 } else { 0.0 };
            if (((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) && (!(s.v[1488] != 0.0))) && (s.v[1489] != 0.0)) {
                s.store_scale(494, 1472, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) && (!(s.v[1488] != 0.0))) {
                s.store_add(1442, 1442, 494);
            }
            s.v[1490] = if ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1470]) as f64).abs() <= 1e-8)) { 1.0 } else { 0.0 };
            if (((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) && (!(s.v[1488] != 0.0))) && (s.v[1490] != 0.0)) {
                s.store_scalar(1413, 1.0);
            }
            if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.v[1492] = if (s.v[1440] < 5.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) && (s.v[1492] != 0.0)) {
            s.store_offset_ad(1473, A::square(s.ad_value(1464)), (10.0 * 2.220446049250313e-16));
        }

        if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) && (s.v[1492] != 0.0)) {
            s.store_offset(1474, 1464, (10.0 * 2.220446049250313e-16));
        }

        if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) && (!(s.v[1492] != 0.0))) {
            s.store_offset(1473, 1440, (-1.0));
        }

        if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) && (!(s.v[1492] != 0.0))) {
            s.store_sqrt(1474, 1473);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) {
            s.store_mul(458, 1425, 1474);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) {
            s.store_div_from_scalar_ad(1397, 1.0, A::add(s.ad_value(1466), s.ad_value(1474)));
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) {
            s.store_mul_ad_lhs(460, A::mul(s.ad_value(1425), s.ad_value(1462)), 1397);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1485] != 0.0)) {
            s.store_add(459, 458, 460);
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_sub(460, 459, 458);
        }

        s.v[1494] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1495] = if (1.0 == 2.0) { 1.0 } else { 0.0 };

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1494] != 0.0)) && (s.v[1407] != 0.0)) {
            s.store_mul_ad_lhs(463, A::neg(s.ad_value(522)), 459);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1494] != 0.0)) && (s.v[1407] != 0.0)) {
            s.store_mul_ad_lhs(465, A::neg(s.ad_value(522)), 460);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1494] != 0.0)) && (s.v[1408] != 0.0)) {
            s.store_mul_ad_lhs(464, A::neg(s.ad_value(522)), 459);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1494] != 0.0)) && (s.v[1408] != 0.0)) {
            s.store_mul_ad_lhs(466, A::neg(s.ad_value(522)), 460);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && ((s.v[1495] != 0.0) && (!(s.v[1494] != 0.0)))) && (s.v[1407] != 0.0)) {
            s.store_mul_ad_lhs(467, A::neg(s.ad_value(522)), 459);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && ((s.v[1495] != 0.0) && (!(s.v[1494] != 0.0)))) && (s.v[1407] != 0.0)) {
            s.store_mul_ad_lhs(469, A::neg(s.ad_value(522)), 460);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && ((s.v[1495] != 0.0) && (!(s.v[1494] != 0.0)))) && (s.v[1408] != 0.0)) {
            s.store_mul_ad_lhs(468, A::neg(s.ad_value(522)), 459);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && ((s.v[1495] != 0.0) && (!(s.v[1494] != 0.0)))) && (s.v[1408] != 0.0)) {
            s.store_mul_ad_lhs(470, A::neg(s.ad_value(522)), 460);
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_scalar(1407, ((1.0 - 1.0) / 2.0));
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_scalar(1408, ((1.0 + 1.0) / 2.0));
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_add_ad(1418, A::mul(s.ad_value(461), s.ad_value(156)), A::mul(s.ad_value(462), A::sub(s.ad_value(156), s.ad_value(157))));
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_add_ad(1419, A::mul(s.ad_value(461), s.ad_value(157)), A::mul(s.ad_value(462), A::neg(s.ad_value(157))));
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_add_ad(1420, A::mul(s.ad_value(461), s.ad_value(158)), A::mul(s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_add_ad(1421, A::mul(s.ad_value(462), s.ad_value(158)), A::mul(s.ad_value(461), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_sub(1422, 1419, 1418);
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_neg(1423, 1418);
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_add_ad(1409, A::mul(s.ad_value(1407), s.ad_value(461)), A::mul(s.ad_value(1408), s.ad_value(462)));
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_add_ad(1410, A::mul(s.ad_value(1407), s.ad_value(462)), A::mul(s.ad_value(1408), s.ad_value(461)));
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_add_ad(1424, A::mul(s.ad_value(1409), s.ad_value(1420)), A::mul(s.ad_value(1410), s.ad_value(1421)));
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_offset_ad(1416, A::add(A::mul(s.ad_value(1409), s.ad_value(1423)), A::mul(s.ad_value(1410), s.ad_value(1422))), (10.0 * 2.220446049250313e-16));
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_neg(1396, 1416);
        }

        s.v[1496] = if (s.v[1396] > s.v[141]) { 1.0 } else { 0.0 };

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1496] != 0.0)) {
            s.store_sub(1397, 1396, 141);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1496] != 0.0)) {
            s.store_sub(1398, 140, 141);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1496] != 0.0)) {
            s.store_div(44, 1397, 1398);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1496] != 0.0)) {
            s.store_square(45, 44);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1496] != 0.0)) {
            s.store_mul(46, 45, 44);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1496] != 0.0)) {
            s.store_square(47, 45);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1496] != 0.0)) {
            s.store_div_from_scalar_ad(1404, 1.0, A::add(A::add(A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(45)), s.ad_value(46)), s.ad_value(47)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1496] != 0.0)) {
            s.store_mul_ad_rhs(1404, 1398, A::sub_from_scalar(1.0, s.ad_value(1404)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1496] != 0.0)) {
            s.store_add(1401, 141, 1404);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1496] != 0.0))) {
            s.copy_ad(1401, 1396);
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_offset_ad(1417, A::neg(s.ad_value(1401)), (-1e-12));
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_mul(1426, 1425, 1406);
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_square(1427, 1426);
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_sub(1428, 1424, 523);
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_div(1396, 521, 230);
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_mul_ad(1429, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(s.ad_value(1396)));
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_neg(1430, 1417);
        }

        s.v[1497] = if (s.v[1428] < s.v[1430]) { 1.0 } else { 0.0 };

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1497] != 0.0)) {
            s.store_div_from_scalar_ad(1397, 1.0, A::mul(s.ad_value(225), s.ad_value(1425)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1497] != 0.0)) {
            s.store_mul(1404, 1397, 1405);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1497] != 0.0)) {
            s.store_offset_scaled(1431, 1404, (3.0 * 1.414213562373095), 2.0);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1497] != 0.0)) {
            s.store_mul_ad_lhs(1432, A::mul(A::scale(s.ad_value(1431), 8.0), s.ad_value(1431)), 1431);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1497] != 0.0)) {
            s.store_sub(1433, 237, 1429);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1497] != 0.0)) {
            s.store_mul_ad_rhs(1403, 225, A::add(s.ad_value(1428), s.ad_value(1417)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1497] != 0.0)) {
            s.store_sub_from_scalar_ad(1434, (7.0 * 1.414213562373095), A::mul(A::scale(s.ad_value(1404), 9.0), A::offset(s.ad_value(1403), (-2.0))));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1497] != 0.0)) {
            s.store_square(1435, 1434);
        }

        s.v[1498] = if (s.v[1432] < (s.v[1435] * 1e-8)) { 1.0 } else { 0.0 };

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1497] != 0.0)) && (s.v[1498] != 0.0)) {
            s.store_add_ad(1437, A::add(A::offset(s.ad_value(1434), ((-7.0) * 1.414213562373095)), A::div(A::scale(s.ad_value(1432), 0.5), s.ad_value(1434))), A::mul(A::scale(s.ad_value(1404), 9.0), A::offset(s.ad_value(1403), (-2.0))));
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1497] != 0.0)) && (!(s.v[1498] != 0.0))) {
            s.store_sqrt_ad(1436, A::add(s.ad_value(1432), s.ad_value(1435)));
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1497] != 0.0)) && (!(s.v[1498] != 0.0))) {
            s.store_add_ad(1437, A::offset(s.ad_value(1436), ((-7.0) * 1.414213562373095)), A::mul(A::scale(s.ad_value(1404), 9.0), A::offset(s.ad_value(1403), (-2.0))));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1497] != 0.0)) {
            s.store_powf(1438, 1437, 0.3333333333333333);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1497] != 0.0)) {
            s.store_add_ad(1439, A::add(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1404), 12.0)), A::scale(s.ad_value(1438), 2.0)), A::mul(A::scale(s.ad_value(1438), 1.414213562373095), s.ad_value(1438)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1497] != 0.0)) {
            s.store_div(1440, 1439, 1438);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1497] != 0.0)) {
            s.store_sub_ad_lhs(1441, A::mul(s.ad_value(1440), s.ad_value(227)), 1417);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1497] != 0.0)) {
            s.store_add(1397, 1441, 1417);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1497] != 0.0)) {
            s.store_div(1398, 1397, 1433);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1497] != 0.0)) {
            s.store_sqrt_ad(1399, A::offset(A::square(s.ad_value(1398)), 1.0));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1497] != 0.0)) {
            s.store_sub_ad_lhs(1442, A::div(s.ad_value(1397), s.ad_value(1399)), 1417);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1497] != 0.0)) {
            s.store_sub(1398, 1428, 1442);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1497] != 0.0)) {
            s.store_mul(459, 1405, 1398);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1497] != 0.0)) {
            s.copy_ad(458, 459);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_scalar(1440, 3.0);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_sub_ad_lhs(1443, A::div(s.ad_value(1440), s.ad_value(225)), 1417);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_exp_ad(1404, A::neg(s.ad_value(1440)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_offset_ad(1403, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1428), s.ad_value(1417))), (-1.0)), s.ad_value(1404)), 4.0), A::mul(s.ad_value(1427), s.ad_value(226))), 1.0);
        }

        s.v[1499] = if (s.v[1403] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1499] != 0.0)) {
            s.store_scalar(1403, (10.0 * 2.220446049250313e-16));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_add_ad_rhs(1443, 1428, A::mul(A::scale(A::mul(s.ad_value(1427), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1403)))));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_mul_ad_rhs(1440, 225, A::add(s.ad_value(1443), s.ad_value(1417)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_exp_ad(1404, A::neg(s.ad_value(1440)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_offset_ad(1403, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1428), s.ad_value(1417))), (-1.0)), s.ad_value(1404)), 4.0), A::mul(s.ad_value(1427), s.ad_value(226))), 1.0);
        }

        s.v[1500] = if (s.v[1403] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1500] != 0.0)) {
            s.store_scalar(1403, (10.0 * 2.220446049250313e-16));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_add_ad_rhs(1443, 1428, A::mul(A::scale(A::mul(s.ad_value(1427), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1403)))));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_mul_ad_rhs(1440, 225, A::add(s.ad_value(1443), s.ad_value(1417)));
        }

        s.v[1501] = if (s.v[1440] < 3.0) { 1.0 } else { 0.0 };

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1501] != 0.0)) {
            s.store_scalar(1444, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1501] != 0.0)) {
            s.store_scalar(1445, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1501] != 0.0)) {
            s.store_offset_ad(1446, A::div_from_scalar(1.0, A::mul(s.ad_value(225), s.ad_value(1426))), (1.0 / 1.414213562373095));
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1501] != 0.0)) {
            s.store_div_ad_lhs(1447, A::neg(A::add(s.ad_value(1428), s.ad_value(1417))), 1426);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1501] != 0.0)) {
            s.store_add_ad(1448, A::sub(A::div(A::mul(A::square(s.ad_value(1445)), s.ad_value(1445)), A::mul(A::mul(A::scale(s.ad_value(1444), 27.0), s.ad_value(1444)), s.ad_value(1444))), A::div(A::mul(s.ad_value(1445), s.ad_value(1446)), A::mul(A::scale(s.ad_value(1444), 6.0), s.ad_value(1444)))), A::div(s.ad_value(1447), A::scale(s.ad_value(1444), 2.0)));
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1501] != 0.0)) {
            s.store_div_ad(1449, A::sub(A::mul(A::scale(s.ad_value(1444), 3.0), s.ad_value(1446)), A::square(s.ad_value(1445))), A::mul(A::scale(s.ad_value(1444), 9.0), s.ad_value(1444)));
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1501] != 0.0)) {
            s.store_sqrt_ad(1400, A::add(A::square(s.ad_value(1448)), A::mul(A::square(s.ad_value(1449)), s.ad_value(1449))));
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1501] != 0.0)) {
            s.store_powf_ad(1450, A::sub(s.ad_value(1400), s.ad_value(1448)), 0.3333333333333333);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1501] != 0.0)) {
            s.store_neg_ad(1451, A::powf(A::add(s.ad_value(1448), s.ad_value(1400)), 0.3333333333333333));
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1501] != 0.0)) {
            s.store_sub_ad(1403, A::add(s.ad_value(1450), s.ad_value(1451)), A::div(s.ad_value(1445), A::scale(s.ad_value(1444), 3.0)));
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1501] != 0.0)) {
            s.store_sub_ad_lhs(1443, A::mul(s.ad_value(1403), s.ad_value(227)), 1417);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1501] != 0.0)) {
            s.store_mul_ad_rhs(1440, 225, A::add(s.ad_value(1443), s.ad_value(1417)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_offset_ad(1452, A::add(s.ad_value(1428), s.ad_value(1417)), 0.1);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_offset_ad(1459, A::exp(A::mul(s.ad_value(225), A::neg(s.ad_value(1417)))), 1e-50);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_div(1396, 230, 521);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_square(1453, 1396);
        }

    }

    pub(super) fn stamp_reactive_block_24(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_mul(1454, 1453, 1459);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_mul(1396, 226, 1427);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_mul(1455, 225, 1452);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_add_ad(1456, A::sub(A::ln(A::add(A::mul(s.ad_value(1454), s.ad_value(1396)), A::square(s.ad_value(1455)))), A::ln(A::mul(s.ad_value(1453), s.ad_value(1396)))), A::mul(s.ad_value(225), s.ad_value(1417)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_offset_ad(44, A::sub(s.ad_value(1455), s.ad_value(1456)), (-1.0));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_scale(45, 1455, 4.0);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_scale_ad(1397, A::offset(A::div(s.ad_value(44), s.ad_value(45)), 1.0), 0.5);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_scale_ad(1398, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(44), 2.0), s.ad_value(45))), 0.5);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_sub_ad_rhs(1456, 1455, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_sub(1455, 1455, 1456);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_add_ad_rhs(1455, 1455, A::scale(s.ad_value(225), 0.1));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_add_ad(1457, A::sub(A::ln(A::add(A::mul(s.ad_value(1454), s.ad_value(1396)), A::square(s.ad_value(1455)))), A::ln(A::mul(s.ad_value(1453), s.ad_value(1396)))), A::mul(s.ad_value(225), s.ad_value(1417)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.copy_ad(1458, 1440);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_offset_ad(44, A::sub(s.ad_value(1457), s.ad_value(1458)), (-(0.0008 * 75.0)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_scale(45, 1457, (4.0 * (0.0008 * 75.0)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_scale_ad(1397, A::offset(A::div(s.ad_value(44), s.ad_value(45)), 1.0), 0.5);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_scale_ad(1398, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(44), ((2.0 * 0.0008) * 75.0)), s.ad_value(45))), 0.5);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_sub_ad_rhs(1440, 1457, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_sub_ad_lhs(1442, A::div(s.ad_value(1440), s.ad_value(225)), 1417);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_add_ad(1397, A::offset(s.ad_value(1440), (-1.0)), A::exp(A::neg(s.ad_value(1440))));
        }

        s.v[1502] = if (s.v[1397] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1502] != 0.0)) {
            s.store_scalar(1397, (10.0 * 2.220446049250313e-16));
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_sqrt(1398, 1397);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_mul(458, 1425, 1398);
        }

        if ((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_mul_ad_rhs(459, 1405, A::sub(s.ad_value(1428), s.ad_value(1442)));
        }

        s.v[1503] = if (p.p42 == 1.0) { 1.0 } else { 0.0 };

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) {
            s.store_exp_ad(1459, A::mul(s.ad_value(225), A::neg(s.ad_value(1417))));
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) {
            s.store_div(1396, 230, 521);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) {
            s.store_square(1453, 1396);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) {
            s.store_mul(1468, 1453, 1459);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) {
            s.store_scalar(1413, 0.0);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) {
            s.store_scalar(167, 1.0);
        }

        let mut assign27770_loop_guard: usize = 0;
        while {
            let assign27770_cond_e38739: f64 = (2.0 * 20.0);
            let assign27770_cond_e38741: f64 = (assign27770_cond_e38739 + 1.0);
            let assign27770_cond_e38743: f64 = if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) && (s.v[167] <= assign27770_cond_e38741)) { 1.0 } else { 0.0 };
            assign27770_cond_e38743 != 0.0
        } {
            assign27770_loop_guard += 1;
            assert!(assign27770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) {
                s.store_scalar(1464, 0.0);
            }
            if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) {
                s.store_mul_ad_rhs(1440, 225, A::add(s.ad_value(1442), s.ad_value(1417)));
            }
            s.v[1504] = if (s.v[1440] < 5.0) { 1.0 } else { 0.0 };
            if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) && (s.v[1504] != 0.0)) {
                s.store_mul_ad(1460, A::mul(A::square(s.ad_value(1440)), s.ad_value(1440)), A::offset(A::mul(s.ad_value(1440), A::offset(A::scale(s.ad_value(1440), 0.006115288895133179), (-0.07053654284009761))), 0.29693154855771));
            }
            if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) && (s.v[1504] != 0.0)) {
                s.store_mul_ad(1461, A::square(s.ad_value(1440)), A::offset(A::mul(s.ad_value(1440), A::offset(A::scale(s.ad_value(1440), (5.0 * 0.006115288895133179)), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
            }
            if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) && (s.v[1504] != 0.0)) {
                s.store_mul_ad_lhs(1462, A::mul(s.ad_value(1468), s.ad_value(1460)), 1460);
            }
            if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) && (s.v[1504] != 0.0)) {
                s.store_mul_ad_lhs(1463, A::mul(A::scale(A::mul(s.ad_value(1468), s.ad_value(225)), 2.0), s.ad_value(1460)), 1461);
            }
            if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) && (s.v[1504] != 0.0)) {
                s.store_mul_ad_rhs(1464, 1440, A::offset(A::mul(s.ad_value(1440), A::offset(A::mul(s.ad_value(1440), A::offset(A::mul(s.ad_value(1440), A::offset(A::scale(s.ad_value(1440), 6.36964918866352e-5), (-0.00163730162779191))), 0.0178800506338833)), (-0.117851130197758))), 0.707106781186548));
            }
            if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) && (s.v[1504] != 0.0)) {
                s.store_offset_ad(1465, A::mul(s.ad_value(1440), A::offset(A::mul(s.ad_value(1440), A::offset(A::mul(s.ad_value(1440), A::offset(A::scale(s.ad_value(1440), (5.0 * 6.36964918866352e-5)), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833))), (2.0 * (-0.117851130197758)))), 0.707106781186548);
            }
            if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) && (s.v[1504] != 0.0)) {
                s.store_sqrt_ad(1466, A::offset(A::add(A::square(s.ad_value(1464)), s.ad_value(1462)), 1e-50));
            }
            if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) && (s.v[1504] != 0.0)) {
                s.store_div_ad(1467, A::add(A::mul(A::scale(A::mul(s.ad_value(225), s.ad_value(1465)), 2.0), s.ad_value(1464)), s.ad_value(1463)), A::scale(s.ad_value(1466), 2.0));
            }
            s.v[1505] = if (s.v[1440] < 80.0) { 1.0 } else { 0.0 };
            if (((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) && (!(s.v[1504] != 0.0))) && (s.v[1505] != 0.0)) {
                s.store_exp(243, 1440);
            }
            if (((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) && (!(s.v[1504] != 0.0))) && (s.v[1505] != 0.0)) {
                s.store_mul_ad_rhs(1462, 1468, A::offset(s.ad_value(243), (-1.0)));
            }
            if (((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) && (!(s.v[1504] != 0.0))) && (s.v[1505] != 0.0)) {
                s.store_mul_ad_lhs(1463, A::mul(s.ad_value(1468), s.ad_value(225)), 243);
            }
            if (((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) && (!(s.v[1504] != 0.0))) && (!(s.v[1505] != 0.0))) {
                s.store_exp_ad(1469, A::mul(s.ad_value(225), s.ad_value(1442)));
            }
            if (((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) && (!(s.v[1504] != 0.0))) && (!(s.v[1505] != 0.0))) {
                s.store_mul_ad_rhs(1462, 1453, A::sub(s.ad_value(1469), s.ad_value(1459)));
            }
            if (((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) && (!(s.v[1504] != 0.0))) && (!(s.v[1505] != 0.0))) {
                s.store_mul_ad_lhs(1463, A::mul(s.ad_value(1453), s.ad_value(225)), 1469);
            }
            if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) && (!(s.v[1504] != 0.0))) {
                s.store_sqrt_ad(1466, A::add(A::offset(s.ad_value(1440), (-1.0)), s.ad_value(1462)));
            }
            if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) && (!(s.v[1504] != 0.0))) {
                s.store_scale_ad(1467, A::div(A::add(s.ad_value(225), s.ad_value(1463)), s.ad_value(1466)), 0.5);
            }
            if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) {
                s.store_sub_ad(1470, A::sub(s.ad_value(1428), s.ad_value(1442)), A::mul(s.ad_value(1426), s.ad_value(1466)));
            }
            if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) {
                s.store_sub_from_scalar_ad(1471, (-1.0), A::mul(s.ad_value(1426), s.ad_value(1467)));
            }
            s.v[1506] = if (s.v[1413] == 1.0) { 1.0 } else { 0.0 };
            if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) && (s.v[1506] != 0.0)) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) && (!(s.v[1506] != 0.0))) {
                s.store_div_ad_lhs(494, A::neg(s.ad_value(1470)), 1471);
            }
            if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) && (!(s.v[1506] != 0.0))) {
                s.store_scale_ad(1472, A::offset({
                    if (1.0 >= ((s.v[1442]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1442))
                    }
                }, 1.0), (0.5 * 0.1));
            }
            s.v[1507] = if (((s.v[494]) as f64).abs() > s.v[1472]) { 1.0 } else { 0.0 };
            if (((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) && (!(s.v[1506] != 0.0))) && (s.v[1507] != 0.0)) {
                s.store_scale(494, 1472, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) && (!(s.v[1506] != 0.0))) {
                s.store_add(1442, 1442, 494);
            }
            s.v[1508] = if ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1470]) as f64).abs() <= 1e-8)) { 1.0 } else { 0.0 };
            if (((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) && (!(s.v[1506] != 0.0))) && (s.v[1508] != 0.0)) {
                s.store_scalar(1413, 1.0);
            }
            if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.v[1510] = if (s.v[1440] < 5.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) && (s.v[1510] != 0.0)) {
            s.store_offset_ad(1473, A::square(s.ad_value(1464)), (10.0 * 2.220446049250313e-16));
        }

        if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) && (s.v[1510] != 0.0)) {
            s.store_offset(1474, 1464, (10.0 * 2.220446049250313e-16));
        }

        if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) && (!(s.v[1510] != 0.0))) {
            s.store_offset(1473, 1440, (-1.0));
        }

        if ((((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) && (!(s.v[1510] != 0.0))) {
            s.store_sqrt(1474, 1473);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) {
            s.store_mul(458, 1425, 1474);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) {
            s.store_div_from_scalar_ad(1397, 1.0, A::add(s.ad_value(1466), s.ad_value(1474)));
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) {
            s.store_mul_ad_lhs(460, A::mul(s.ad_value(1425), s.ad_value(1462)), 1397);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1497] != 0.0))) && (s.v[1503] != 0.0)) {
            s.store_add(459, 458, 460);
        }

        if (((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_sub(460, 459, 458);
        }

        s.v[1512] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1513] = if (1.0 == 2.0) { 1.0 } else { 0.0 };

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1512] != 0.0)) && (s.v[1407] != 0.0)) {
            s.store_mul_ad_lhs(463, A::neg(s.ad_value(522)), 459);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1512] != 0.0)) && (s.v[1407] != 0.0)) {
            s.store_mul_ad_lhs(465, A::neg(s.ad_value(522)), 460);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1512] != 0.0)) && (s.v[1408] != 0.0)) {
            s.store_mul_ad_lhs(464, A::neg(s.ad_value(522)), 459);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1512] != 0.0)) && (s.v[1408] != 0.0)) {
            s.store_mul_ad_lhs(466, A::neg(s.ad_value(522)), 460);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && ((s.v[1513] != 0.0) && (!(s.v[1512] != 0.0)))) && (s.v[1407] != 0.0)) {
            s.store_mul_ad_lhs(467, A::neg(s.ad_value(522)), 459);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && ((s.v[1513] != 0.0) && (!(s.v[1512] != 0.0)))) && (s.v[1407] != 0.0)) {
            s.store_mul_ad_lhs(469, A::neg(s.ad_value(522)), 460);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && ((s.v[1513] != 0.0) && (!(s.v[1512] != 0.0)))) && (s.v[1408] != 0.0)) {
            s.store_mul_ad_lhs(468, A::neg(s.ad_value(522)), 459);
        }

        if (((((s.v[1395] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) && ((s.v[1513] != 0.0) && (!(s.v[1512] != 0.0)))) && (s.v[1408] != 0.0)) {
            s.store_mul_ad_lhs(470, A::neg(s.ad_value(522)), 460);
        }

        s.v[317] = p.p189;

        s.v[1516] = if (s.v[145] != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1516] != 0.0) {
            s.store_add(1515, 157, 161);
        }

        if (s.v[1516] != 0.0) {
            s.store_add_ad(314, A::scale(s.ad_value(1515), s.v[317]), A::scale(s.ad_value(162), (1.0 - s.v[317])));
        }

        s.v[1517] = if (p.p64 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1516] != 0.0) && (s.v[1517] != 0.0)) {
            s.store_scalar(315, 0.0);
        }

        s.v[1518] = if (s.v[314] > ((s.v[161] + s.v[157]) - (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if ((s.v[1516] != 0.0) && (s.v[1518] != 0.0)) {
            s.store_offset_ad(314, A::add(s.ad_value(161), s.ad_value(157)), (-(10.0 * 2.220446049250313e-16)));
        }

        s.v[1519] = if (p.p64 != 0.0) { 1.0 } else { 0.0 };

        s.v[1520] = if (s.v[246] < 1e-15) { 1.0 } else { 0.0 };

        if (((!(s.v[1516] != 0.0)) && (s.v[1519] != 0.0)) && (s.v[1520] != 0.0)) {
            s.store_scalar(315, 0.0);
        }

        if (((!(s.v[1516] != 0.0)) && (s.v[1519] != 0.0)) && (!(s.v[1520] != 0.0))) {
            s.store_scale(1514, 227, 1.0 / (s.v[97]));
        }

        if (((!(s.v[1516] != 0.0)) && (s.v[1519] != 0.0)) && (!(s.v[1520] != 0.0))) {
            s.store_div_from_scalar(1515, 1.0, 244);
        }

        if (((!(s.v[1516] != 0.0)) && (s.v[1519] != 0.0)) && (!(s.v[1520] != 0.0))) {
            s.store_mul_ad_lhs(315, A::mul(s.ad_value(246), s.ad_value(1514)), 1515);
        }

        s.v[1532] = s.v[91];

        s.v[1533] = (1.0 / s.v[1532]);

        s.v[1553] = 0.0;

        s.v[1593] = 0.0;

        s.v[1591] = 0.0;

        s.v[1595] = 0.0;

        s.v[1604] = if ((p.p29 >= 1.0) && (p.p188 > 0.0)) { 1.0 } else { 0.0 };

        if ((p.p24 != 0.0) && (s.v[1604] != 0.0)) {
            s.store_scalar(1535, p.p171);
        }

        if ((p.p24 != 0.0) && (s.v[1604] != 0.0)) {
            s.store_scalar(1536, p.p172);
        }

        if ((p.p24 != 0.0) && (s.v[1604] != 0.0)) {
            s.copy_ad(1537, 158);
        }

        if ((p.p24 != 0.0) && (s.v[1604] != 0.0)) {
            s.store_scalar(1534, p.p188);
        }

        s.v[1605] = if ((s.v[69] == 0.0) && (p.p188 > 0.0)) { 1.0 } else { 0.0 };

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (s.v[1605] != 0.0)) {
            s.store_ad(1522, &{
                if (p.p43 == 1.0) {
                    A::scale(s.ad_value(287), s.v[1532])
                } else {
                    A::scale(s.ad_value(108), s.v[1532])
                }
            });
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (s.v[1605] != 0.0)) {
            s.store_mul_ad(1525, A::mul(s.ad_value(1535), s.ad_value(1522)), A::add(s.ad_value(1536), s.ad_value(1537)));
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (s.v[1605] != 0.0)) {
            s.store_mul(1526, 1534, 1522);
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (s.v[1605] != 0.0)) {
            s.copy_ad(1530, 161);
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (s.v[1605] != 0.0)) {
            s.store_sub_from_scalar(1527, 1.2, 1530);
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (s.v[1605] != 0.0)) {
            s.store_sub_ad(267, A::mul(s.ad_value(158), s.ad_value(1526)), A::mul(s.ad_value(1527), s.ad_value(1525)));
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (s.v[1605] != 0.0)) {
            s.store_mul_ad(1525, A::mul(s.ad_value(1535), s.ad_value(1522)), A::sub(A::add(s.ad_value(1536), s.ad_value(1537)), s.ad_value(157)));
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (s.v[1605] != 0.0)) {
            s.store_sub(1530, 162, 157);
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (s.v[1605] != 0.0)) {
            s.store_sub_from_scalar(1527, 1.2, 1530);
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (s.v[1605] != 0.0)) {
            s.store_sub_ad(268, A::mul(A::sub(s.ad_value(158), s.ad_value(157)), s.ad_value(1526)), A::mul(s.ad_value(1525), s.ad_value(1527)));
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) {
            s.store_mul_ad_rhs(1554, 238, A::sqrt(A::div_from_scalar(s.v[69], s.ad_value(536))));
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) {
            s.store_scalar(1538, ((1.0 - -1.0) / 2.0));
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) {
            s.store_scalar(1539, ((1.0 + -1.0) / 2.0));
        }

        s.v[1606] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1606] != 0.0)) {
            s.store_add_ad(1548, A::mul(s.ad_value(461), s.ad_value(156)), A::mul(s.ad_value(462), A::sub(s.ad_value(156), s.ad_value(157))));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1606] != 0.0)) {
            s.store_add_ad(1549, A::mul(s.ad_value(461), s.ad_value(157)), A::mul(s.ad_value(462), A::neg(s.ad_value(157))));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1606] != 0.0)) {
            s.store_add_ad(1550, A::mul(s.ad_value(461), s.ad_value(158)), A::mul(s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1606] != 0.0)) {
            s.store_sub(1551, 1549, 1548);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1606] != 0.0)) {
            s.store_sub(1553, 1550, 1548);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1606] != 0.0)) {
            s.store_neg(1552, 1548);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1606] != 0.0)) {
            s.store_add_ad(1540, A::mul(s.ad_value(1538), s.ad_value(461)), A::mul(s.ad_value(1539), s.ad_value(462)));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1606] != 0.0)) {
            s.store_add_ad(1541, A::mul(s.ad_value(1538), s.ad_value(462)), A::mul(s.ad_value(1539), s.ad_value(461)));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1606] != 0.0)) {
            s.store_offset_ad(1546, A::add(A::mul(s.ad_value(1540), s.ad_value(1552)), A::mul(s.ad_value(1541), s.ad_value(1551))), (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1606] != 0.0))) {
            s.store_add_ad(1540, A::mul(s.ad_value(1538), s.ad_value(461)), A::mul(s.ad_value(1539), s.ad_value(462)));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1606] != 0.0))) {
            s.store_add_ad(1541, A::mul(s.ad_value(1538), s.ad_value(462)), A::mul(s.ad_value(1539), s.ad_value(461)));
        }

    }

    pub(super) fn stamp_reactive_block_25(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1538] != 0.0)) {
            s.store_add_ad(1553, A::mul(s.ad_value(461), s.ad_value(158)), A::mul(s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1539] != 0.0)) {
            s.store_add_ad(1553, A::mul(s.ad_value(462), s.ad_value(158)), A::mul(s.ad_value(461), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1606] != 0.0))) {
            s.store_scalar(1546, 0.0);
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) {
            s.store_neg(1521, 1546);
        }

        s.v[1607] = if (s.v[1521] > s.v[141]) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1607] != 0.0)) {
            s.store_sub(1522, 1521, 141);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1607] != 0.0)) {
            s.store_sub(1523, 140, 141);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1607] != 0.0)) {
            s.store_div(44, 1522, 1523);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1607] != 0.0)) {
            s.store_square(45, 44);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1607] != 0.0)) {
            s.store_mul(46, 45, 44);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1607] != 0.0)) {
            s.store_square(47, 45);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1607] != 0.0)) {
            s.store_div_from_scalar_ad(1531, 1.0, A::add(A::add(A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(45)), s.ad_value(46)), s.ad_value(47)));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1607] != 0.0)) {
            s.store_mul_ad_rhs(1531, 1523, A::sub_from_scalar(1.0, s.ad_value(1531)));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1607] != 0.0)) {
            s.store_add(1528, 141, 1531);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1607] != 0.0))) {
            s.copy_ad(1528, 1521);
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) {
            s.store_offset_ad(1547, A::neg(s.ad_value(1528)), (-1e-12));
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) {
            s.store_scale(1555, 1554, s.v[1533]);
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) {
            s.store_square(1556, 1555);
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) {
            s.store_sub_from_scalar(1557, s.v[82], 1553);
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) {
            s.store_div_from_scalar(1521, s.v[69], 230);
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) {
            s.store_mul_ad(1558, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(s.ad_value(1521)));
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) {
            s.store_neg(1559, 1547);
        }

        s.v[1608] = if (s.v[1557] < s.v[1559]) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1608] != 0.0)) {
            s.store_div_from_scalar_ad(1522, 1.0, A::mul(s.ad_value(225), s.ad_value(1554)));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1608] != 0.0)) {
            s.store_scale(1531, 1522, s.v[1532]);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1608] != 0.0)) {
            s.store_offset_scaled(1560, 1531, (3.0 * 1.414213562373095), 2.0);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1608] != 0.0)) {
            s.store_mul_ad_lhs(1561, A::mul(A::scale(s.ad_value(1560), 8.0), s.ad_value(1560)), 1560);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1608] != 0.0)) {
            s.store_sub(1562, 237, 1558);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1608] != 0.0)) {
            s.store_mul_ad_rhs(1530, 225, A::add(s.ad_value(1557), s.ad_value(1547)));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1608] != 0.0)) {
            s.store_sub_from_scalar_ad(1563, (7.0 * 1.414213562373095), A::mul(A::scale(s.ad_value(1531), 9.0), A::offset(s.ad_value(1530), (-2.0))));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1608] != 0.0)) {
            s.store_square(1564, 1563);
        }

        s.v[1609] = if (s.v[1561] < (s.v[1564] * 1e-8)) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1608] != 0.0)) && (s.v[1609] != 0.0)) {
            s.store_add_ad(1566, A::add(A::offset(s.ad_value(1563), ((-7.0) * 1.414213562373095)), A::div(A::scale(s.ad_value(1561), 0.5), s.ad_value(1563))), A::mul(A::scale(s.ad_value(1531), 9.0), A::offset(s.ad_value(1530), (-2.0))));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1608] != 0.0)) && (!(s.v[1609] != 0.0))) {
            s.store_sqrt_ad(1565, A::add(s.ad_value(1561), s.ad_value(1564)));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1608] != 0.0)) && (!(s.v[1609] != 0.0))) {
            s.store_add_ad(1566, A::offset(s.ad_value(1565), ((-7.0) * 1.414213562373095)), A::mul(A::scale(s.ad_value(1531), 9.0), A::offset(s.ad_value(1530), (-2.0))));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1608] != 0.0)) {
            s.store_powf(1567, 1566, 0.3333333333333333);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1608] != 0.0)) {
            s.store_add_ad(1568, A::add(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1531), 12.0)), A::scale(s.ad_value(1567), 2.0)), A::mul(A::scale(s.ad_value(1567), 1.414213562373095), s.ad_value(1567)));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1608] != 0.0)) {
            s.store_div(1569, 1568, 1567);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1608] != 0.0)) {
            s.store_sub_ad_lhs(1570, A::mul(s.ad_value(1569), s.ad_value(227)), 1547);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1608] != 0.0)) {
            s.store_add(1522, 1570, 1547);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1608] != 0.0)) {
            s.store_div(1523, 1522, 1562);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1608] != 0.0)) {
            s.store_sqrt_ad(1524, A::offset(A::square(s.ad_value(1523)), 1.0));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1608] != 0.0)) {
            s.store_sub_ad_lhs(1571, A::div(s.ad_value(1522), s.ad_value(1524)), 1547);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1608] != 0.0)) {
            s.store_sub(1523, 1557, 1571);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1608] != 0.0)) {
            s.store_scale(459, 1523, s.v[1532]);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1608] != 0.0)) {
            s.copy_ad(458, 459);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) {
            s.store_scalar(1569, 3.0);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) {
            s.store_sub_ad_lhs(1572, A::div(s.ad_value(1569), s.ad_value(225)), 1547);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) {
            s.store_exp_ad(1531, A::neg(s.ad_value(1569)));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) {
            s.store_offset_ad(1530, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1557), s.ad_value(1547))), (-1.0)), s.ad_value(1531)), 4.0), A::mul(s.ad_value(1556), s.ad_value(226))), 1.0);
        }

        s.v[1610] = if (s.v[1530] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_scalar(1530, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) {
            s.store_add_ad_rhs(1572, 1557, A::mul(A::scale(A::mul(s.ad_value(1556), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1530)))));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) {
            s.store_mul_ad_rhs(1569, 225, A::add(s.ad_value(1572), s.ad_value(1547)));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) {
            s.store_exp_ad(1531, A::neg(s.ad_value(1569)));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) {
            s.store_offset_ad(1530, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1557), s.ad_value(1547))), (-1.0)), s.ad_value(1531)), 4.0), A::mul(s.ad_value(1556), s.ad_value(226))), 1.0);
        }

        s.v[1611] = if (s.v[1530] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1611] != 0.0)) {
            s.store_scalar(1530, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) {
            s.store_add_ad_rhs(1572, 1557, A::mul(A::scale(A::mul(s.ad_value(1556), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1530)))));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) {
            s.store_mul_ad_rhs(1569, 225, A::add(s.ad_value(1572), s.ad_value(1547)));
        }

        s.v[1612] = if (s.v[1569] < 3.0) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_scalar(1573, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_scalar(1574, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_offset_ad(1575, A::div_from_scalar(1.0, A::mul(s.ad_value(225), s.ad_value(1555))), (1.0 / 1.414213562373095));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_div_ad_lhs(1576, A::neg(A::add(s.ad_value(1557), s.ad_value(1547))), 1555);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_add_ad(1577, A::sub(A::div(A::mul(A::square(s.ad_value(1574)), s.ad_value(1574)), A::mul(A::mul(A::scale(s.ad_value(1573), 27.0), s.ad_value(1573)), s.ad_value(1573))), A::div(A::mul(s.ad_value(1574), s.ad_value(1575)), A::mul(A::scale(s.ad_value(1573), 6.0), s.ad_value(1573)))), A::div(s.ad_value(1576), A::scale(s.ad_value(1573), 2.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_div_ad(1578, A::sub(A::mul(A::scale(s.ad_value(1573), 3.0), s.ad_value(1575)), A::square(s.ad_value(1574))), A::mul(A::scale(s.ad_value(1573), 9.0), s.ad_value(1573)));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_sqrt_ad(1526, A::add(A::square(s.ad_value(1577)), A::mul(A::square(s.ad_value(1578)), s.ad_value(1578))));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_powf_ad(1579, A::sub(s.ad_value(1526), s.ad_value(1577)), 0.3333333333333333);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_neg_ad(1580, A::powf(A::add(s.ad_value(1577), s.ad_value(1526)), 0.3333333333333333));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_sub_ad(1530, A::add(s.ad_value(1579), s.ad_value(1580)), A::div(s.ad_value(1574), A::scale(s.ad_value(1573), 3.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_sub_ad_lhs(1572, A::mul(s.ad_value(1530), s.ad_value(227)), 1547);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_mul_ad_rhs(1569, 225, A::add(s.ad_value(1572), s.ad_value(1547)));
        }

        s.v[1613] = if (p.p41 > 0.0) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_offset_ad(1581, A::add(s.ad_value(1557), s.ad_value(1547)), 0.1);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_offset_ad(1588, A::exp(A::mul(s.ad_value(225), A::neg(s.ad_value(1547)))), 1e-50);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_scale(1521, 230, 1.0 / (s.v[69]));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_square(1582, 1521);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_mul(1583, 1582, 1588);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_mul(1521, 226, 1556);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_mul(1584, 225, 1581);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_add_ad(1585, A::sub(A::ln(A::add(A::mul(s.ad_value(1583), s.ad_value(1521)), A::square(s.ad_value(1584)))), A::ln(A::mul(s.ad_value(1582), s.ad_value(1521)))), A::mul(s.ad_value(225), s.ad_value(1547)));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_offset_ad(44, A::sub(s.ad_value(1584), s.ad_value(1585)), (-1.0));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_scale(45, 1584, 4.0);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_scale_ad(1522, A::offset(A::div(s.ad_value(44), s.ad_value(45)), 1.0), 0.5);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_scale_ad(1523, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(44), 2.0), s.ad_value(45))), 0.5);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_sub_ad_rhs(1585, 1584, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_sub(1584, 1584, 1585);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_add_ad_rhs(1584, 1584, A::scale(s.ad_value(225), 0.1));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_add_ad(1586, A::sub(A::ln(A::add(A::mul(s.ad_value(1583), s.ad_value(1521)), A::square(s.ad_value(1584)))), A::ln(A::mul(s.ad_value(1582), s.ad_value(1521)))), A::mul(s.ad_value(225), s.ad_value(1547)));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1613] != 0.0)) {
            s.copy_ad(1587, 1569);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_offset_ad(44, A::sub(s.ad_value(1586), s.ad_value(1587)), (-(0.0008 * 75.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_scale(45, 1586, (4.0 * (0.0008 * 75.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_scale_ad(1522, A::offset(A::div(s.ad_value(44), s.ad_value(45)), 1.0), 0.5);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_scale_ad(1523, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(44), ((2.0 * 0.0008) * 75.0)), s.ad_value(45))), 0.5);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_sub_ad_rhs(1569, 1586, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) {
            s.store_sub_ad_lhs(1571, A::div(s.ad_value(1569), s.ad_value(225)), 1547);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) {
            s.store_add_ad(1522, A::offset(s.ad_value(1569), (-1.0)), A::exp(A::neg(s.ad_value(1569))));
        }

        s.v[1614] = if (s.v[1522] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1614] != 0.0)) {
            s.store_scalar(1522, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) {
            s.store_sqrt(1523, 1522);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) {
            s.store_mul(458, 1554, 1523);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) {
            s.store_scaled_sub(459, 1557, 1571, s.v[1532]);
        }

        s.v[1615] = if (p.p41 == 1.0) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_exp_ad(1588, A::mul(s.ad_value(225), A::neg(s.ad_value(1547))));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_scale(1521, 230, 1.0 / (s.v[69]));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_square(1582, 1521);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_mul(1597, 1582, 1588);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_scalar(1544, 0.0);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_scalar(1591, 0.0);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_scalar(1595, 0.0);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_scalar(167, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_26(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let mut assign29770_loop_guard: usize = 0;
        while {
            let assign29770_cond_e42272: f64 = (2.0 * 20.0);
            let assign29770_cond_e42274: f64 = (assign29770_cond_e42272 + 1.0);
            let assign29770_cond_e42276: f64 = if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) && (s.v[167] <= assign29770_cond_e42274)) { 1.0 } else { 0.0 };
            assign29770_cond_e42276 != 0.0
        } {
            assign29770_loop_guard += 1;
            assert!(assign29770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) {
                s.store_scalar(1593, 0.0);
            }
            if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) {
                s.store_mul_ad_rhs(1569, 225, A::add(s.ad_value(1571), s.ad_value(1547)));
            }
            s.v[1616] = if (s.v[1569] < 5.0) { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) && (s.v[1616] != 0.0)) {
                s.store_mul_ad(1589, A::mul(A::square(s.ad_value(1569)), s.ad_value(1569)), A::offset(A::mul(s.ad_value(1569), A::offset(A::scale(s.ad_value(1569), 0.006115288895133179), (-0.07053654284009761))), 0.29693154855771));
            }
            if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) && (s.v[1616] != 0.0)) {
                s.store_mul_ad(1590, A::square(s.ad_value(1569)), A::offset(A::mul(s.ad_value(1569), A::offset(A::scale(s.ad_value(1569), (5.0 * 0.006115288895133179)), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
            }
            if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) && (s.v[1616] != 0.0)) {
                s.store_mul_ad_lhs(1591, A::mul(s.ad_value(1597), s.ad_value(1589)), 1589);
            }
            if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) && (s.v[1616] != 0.0)) {
                s.store_mul_ad_lhs(1592, A::mul(A::scale(A::mul(s.ad_value(1597), s.ad_value(225)), 2.0), s.ad_value(1589)), 1590);
            }
            if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) && (s.v[1616] != 0.0)) {
                s.store_mul_ad_rhs(1593, 1569, A::offset(A::mul(s.ad_value(1569), A::offset(A::mul(s.ad_value(1569), A::offset(A::mul(s.ad_value(1569), A::offset(A::scale(s.ad_value(1569), 6.36964918866352e-5), (-0.00163730162779191))), 0.0178800506338833)), (-0.117851130197758))), 0.707106781186548));
            }
            if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) && (s.v[1616] != 0.0)) {
                s.store_offset_ad(1594, A::mul(s.ad_value(1569), A::offset(A::mul(s.ad_value(1569), A::offset(A::mul(s.ad_value(1569), A::offset(A::scale(s.ad_value(1569), (5.0 * 6.36964918866352e-5)), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833))), (2.0 * (-0.117851130197758)))), 0.707106781186548);
            }
            if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) && (s.v[1616] != 0.0)) {
                s.store_sqrt_ad(1595, A::offset(A::add(A::square(s.ad_value(1593)), s.ad_value(1591)), 1e-50));
            }
            if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) && (s.v[1616] != 0.0)) {
                s.store_div_ad(1596, A::add(A::mul(A::scale(A::mul(s.ad_value(225), s.ad_value(1594)), 2.0), s.ad_value(1593)), s.ad_value(1592)), A::scale(s.ad_value(1595), 2.0));
            }
            s.v[1617] = if (s.v[1569] < 80.0) { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) && (!(s.v[1616] != 0.0))) && (s.v[1617] != 0.0)) {
                s.store_exp(243, 1569);
            }
            if (((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) && (!(s.v[1616] != 0.0))) && (s.v[1617] != 0.0)) {
                s.store_mul_ad_rhs(1591, 1597, A::offset(s.ad_value(243), (-1.0)));
            }
            if (((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) && (!(s.v[1616] != 0.0))) && (s.v[1617] != 0.0)) {
                s.store_mul_ad_lhs(1592, A::mul(s.ad_value(1597), s.ad_value(225)), 243);
            }
            if (((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) && (!(s.v[1616] != 0.0))) && (!(s.v[1617] != 0.0))) {
                s.store_exp_ad(1598, A::mul(s.ad_value(225), s.ad_value(1571)));
            }
            if (((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) && (!(s.v[1616] != 0.0))) && (!(s.v[1617] != 0.0))) {
                s.store_mul_ad_rhs(1591, 1582, A::sub(s.ad_value(1598), s.ad_value(1588)));
            }
            if (((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) && (!(s.v[1616] != 0.0))) && (!(s.v[1617] != 0.0))) {
                s.store_mul_ad_lhs(1592, A::mul(s.ad_value(1582), s.ad_value(225)), 1598);
            }
            if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) && (!(s.v[1616] != 0.0))) {
                s.store_sqrt_ad(1595, A::add(A::offset(s.ad_value(1569), (-1.0)), s.ad_value(1591)));
            }
            if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) && (!(s.v[1616] != 0.0))) {
                s.store_scale_ad(1596, A::div(A::add(s.ad_value(225), s.ad_value(1592)), s.ad_value(1595)), 0.5);
            }
            if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) {
                s.store_sub_ad(1599, A::sub(s.ad_value(1557), s.ad_value(1571)), A::mul(s.ad_value(1555), s.ad_value(1595)));
            }
            if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) {
                s.store_sub_from_scalar_ad(1600, (-1.0), A::mul(s.ad_value(1555), s.ad_value(1596)));
            }
            s.v[1618] = if (s.v[1544] == 1.0) { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) && (s.v[1618] != 0.0)) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) && (!(s.v[1618] != 0.0))) {
                s.store_div_ad_lhs(494, A::neg(s.ad_value(1599)), 1600);
            }
            if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) && (!(s.v[1618] != 0.0))) {
                s.store_scale_ad(1601, A::offset({
                    if (1.0 >= ((s.v[1571]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1571))
                    }
                }, 1.0), (0.5 * 0.1));
            }
            s.v[1619] = if (((s.v[494]) as f64).abs() > s.v[1601]) { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) && (!(s.v[1618] != 0.0))) && (s.v[1619] != 0.0)) {
                s.store_scale(494, 1601, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) && (!(s.v[1618] != 0.0))) {
                s.store_add(1571, 1571, 494);
            }
            s.v[1620] = if ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1599]) as f64).abs() <= 1e-8)) { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) && (!(s.v[1618] != 0.0))) && (s.v[1620] != 0.0)) {
                s.store_scalar(1544, 1.0);
            }
            if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.v[1622] = if (s.v[1569] < 5.0) { 1.0 } else { 0.0 };

        if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) && (s.v[1622] != 0.0)) {
            s.store_offset_ad(1602, A::square(s.ad_value(1593)), (10.0 * 2.220446049250313e-16));
        }

        if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) && (s.v[1622] != 0.0)) {
            s.store_offset(1603, 1593, (10.0 * 2.220446049250313e-16));
        }

        if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) && (!(s.v[1622] != 0.0))) {
            s.store_offset(1602, 1569, (-1.0));
        }

        if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) && (!(s.v[1622] != 0.0))) {
            s.store_sqrt(1603, 1602);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_mul(458, 1554, 1603);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_div_from_scalar_ad(1522, 1.0, A::add(s.ad_value(1595), s.ad_value(1603)));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_mul_ad_lhs(460, A::mul(s.ad_value(1554), s.ad_value(1591)), 1522);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_add(459, 458, 460);
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) {
            s.store_sub(460, 459, 458);
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) {
            s.store_ad(1525, &{
                if (p.p43 == 1.0) {
                    A::mul(s.ad_value(287), s.ad_value(1534))
                } else {
                    A::mul(s.ad_value(108), s.ad_value(1534))
                }
            });
        }

        s.v[1624] = if (((s.v[1540] != 0.0) && (p.p43 == 0.0)) || ((s.v[1538] != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1624] != 0.0)) {
            s.store_mul(455, 1525, 459);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1624] != 0.0)) {
            s.store_mul(457, 1525, 458);
        }

        s.v[1625] = if (((s.v[1541] != 0.0) && (p.p43 == 0.0)) || ((s.v[1539] != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1625] != 0.0)) {
            s.store_mul(454, 1525, 459);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1625] != 0.0)) {
            s.store_mul(456, 1525, 458);
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) {
            s.store_scalar(1538, ((1.0 - 1.0) / 2.0));
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) {
            s.store_scalar(1539, ((1.0 + 1.0) / 2.0));
        }

        s.v[1626] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_add_ad(1548, A::mul(s.ad_value(461), s.ad_value(156)), A::mul(s.ad_value(462), A::sub(s.ad_value(156), s.ad_value(157))));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_add_ad(1549, A::mul(s.ad_value(461), s.ad_value(157)), A::mul(s.ad_value(462), A::neg(s.ad_value(157))));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_add_ad(1550, A::mul(s.ad_value(461), s.ad_value(158)), A::mul(s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_sub(1551, 1549, 1548);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_sub(1553, 1550, 1548);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_neg(1552, 1548);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_add_ad(1540, A::mul(s.ad_value(1538), s.ad_value(461)), A::mul(s.ad_value(1539), s.ad_value(462)));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_add_ad(1541, A::mul(s.ad_value(1538), s.ad_value(462)), A::mul(s.ad_value(1539), s.ad_value(461)));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_offset_ad(1546, A::add(A::mul(s.ad_value(1540), s.ad_value(1552)), A::mul(s.ad_value(1541), s.ad_value(1551))), (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1626] != 0.0))) {
            s.store_add_ad(1540, A::mul(s.ad_value(1538), s.ad_value(461)), A::mul(s.ad_value(1539), s.ad_value(462)));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1626] != 0.0))) {
            s.store_add_ad(1541, A::mul(s.ad_value(1538), s.ad_value(462)), A::mul(s.ad_value(1539), s.ad_value(461)));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1538] != 0.0)) {
            s.store_add_ad(1553, A::mul(s.ad_value(461), s.ad_value(158)), A::mul(s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1539] != 0.0)) {
            s.store_add_ad(1553, A::mul(s.ad_value(462), s.ad_value(158)), A::mul(s.ad_value(461), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1626] != 0.0))) {
            s.store_scalar(1546, 0.0);
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) {
            s.store_neg(1521, 1546);
        }

        s.v[1627] = if (s.v[1521] > s.v[141]) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1627] != 0.0)) {
            s.store_sub(1522, 1521, 141);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1627] != 0.0)) {
            s.store_sub(1523, 140, 141);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1627] != 0.0)) {
            s.store_div(44, 1522, 1523);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1627] != 0.0)) {
            s.store_square(45, 44);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1627] != 0.0)) {
            s.store_mul(46, 45, 44);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1627] != 0.0)) {
            s.store_square(47, 45);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1627] != 0.0)) {
            s.store_div_from_scalar_ad(1531, 1.0, A::add(A::add(A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(45)), s.ad_value(46)), s.ad_value(47)));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1627] != 0.0)) {
            s.store_mul_ad_rhs(1531, 1523, A::sub_from_scalar(1.0, s.ad_value(1531)));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1627] != 0.0)) {
            s.store_add(1528, 141, 1531);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1627] != 0.0))) {
            s.copy_ad(1528, 1521);
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) {
            s.store_offset_ad(1547, A::neg(s.ad_value(1528)), (-1e-12));
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) {
            s.store_scale(1555, 1554, s.v[1533]);
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) {
            s.store_square(1556, 1555);
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) {
            s.store_sub_from_scalar(1557, s.v[82], 1553);
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) {
            s.store_div_from_scalar(1521, s.v[69], 230);
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) {
            s.store_mul_ad(1558, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(s.ad_value(1521)));
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) {
            s.store_neg(1559, 1547);
        }

        s.v[1628] = if (s.v[1557] < s.v[1559]) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_div_from_scalar_ad(1522, 1.0, A::mul(s.ad_value(225), s.ad_value(1554)));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_scale(1531, 1522, s.v[1532]);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_offset_scaled(1560, 1531, (3.0 * 1.414213562373095), 2.0);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_mul_ad_lhs(1561, A::mul(A::scale(s.ad_value(1560), 8.0), s.ad_value(1560)), 1560);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_sub(1562, 237, 1558);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_mul_ad_rhs(1530, 225, A::add(s.ad_value(1557), s.ad_value(1547)));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_sub_from_scalar_ad(1563, (7.0 * 1.414213562373095), A::mul(A::scale(s.ad_value(1531), 9.0), A::offset(s.ad_value(1530), (-2.0))));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_square(1564, 1563);
        }

        s.v[1629] = if (s.v[1561] < (s.v[1564] * 1e-8)) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1628] != 0.0)) && (s.v[1629] != 0.0)) {
            s.store_add_ad(1566, A::add(A::offset(s.ad_value(1563), ((-7.0) * 1.414213562373095)), A::div(A::scale(s.ad_value(1561), 0.5), s.ad_value(1563))), A::mul(A::scale(s.ad_value(1531), 9.0), A::offset(s.ad_value(1530), (-2.0))));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1628] != 0.0)) && (!(s.v[1629] != 0.0))) {
            s.store_sqrt_ad(1565, A::add(s.ad_value(1561), s.ad_value(1564)));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1628] != 0.0)) && (!(s.v[1629] != 0.0))) {
            s.store_add_ad(1566, A::offset(s.ad_value(1565), ((-7.0) * 1.414213562373095)), A::mul(A::scale(s.ad_value(1531), 9.0), A::offset(s.ad_value(1530), (-2.0))));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_powf(1567, 1566, 0.3333333333333333);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_add_ad(1568, A::add(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1531), 12.0)), A::scale(s.ad_value(1567), 2.0)), A::mul(A::scale(s.ad_value(1567), 1.414213562373095), s.ad_value(1567)));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_div(1569, 1568, 1567);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_sub_ad_lhs(1570, A::mul(s.ad_value(1569), s.ad_value(227)), 1547);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_add(1522, 1570, 1547);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_div(1523, 1522, 1562);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_sqrt_ad(1524, A::offset(A::square(s.ad_value(1523)), 1.0));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_sub_ad_lhs(1571, A::div(s.ad_value(1522), s.ad_value(1524)), 1547);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_sub(1523, 1557, 1571);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_scale(459, 1523, s.v[1532]);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1628] != 0.0)) {
            s.copy_ad(458, 459);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) {
            s.store_scalar(1569, 3.0);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) {
            s.store_sub_ad_lhs(1572, A::div(s.ad_value(1569), s.ad_value(225)), 1547);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) {
            s.store_exp_ad(1531, A::neg(s.ad_value(1569)));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) {
            s.store_offset_ad(1530, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1557), s.ad_value(1547))), (-1.0)), s.ad_value(1531)), 4.0), A::mul(s.ad_value(1556), s.ad_value(226))), 1.0);
        }

        s.v[1630] = if (s.v[1530] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_scalar(1530, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) {
            s.store_add_ad_rhs(1572, 1557, A::mul(A::scale(A::mul(s.ad_value(1556), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1530)))));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) {
            s.store_mul_ad_rhs(1569, 225, A::add(s.ad_value(1572), s.ad_value(1547)));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) {
            s.store_exp_ad(1531, A::neg(s.ad_value(1569)));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) {
            s.store_offset_ad(1530, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1557), s.ad_value(1547))), (-1.0)), s.ad_value(1531)), 4.0), A::mul(s.ad_value(1556), s.ad_value(226))), 1.0);
        }

        s.v[1631] = if (s.v[1530] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1631] != 0.0)) {
            s.store_scalar(1530, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) {
            s.store_add_ad_rhs(1572, 1557, A::mul(A::scale(A::mul(s.ad_value(1556), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1530)))));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) {
            s.store_mul_ad_rhs(1569, 225, A::add(s.ad_value(1572), s.ad_value(1547)));
        }

        s.v[1632] = if (s.v[1569] < 3.0) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1632] != 0.0)) {
            s.store_scalar(1573, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1632] != 0.0)) {
            s.store_scalar(1574, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1632] != 0.0)) {
            s.store_offset_ad(1575, A::div_from_scalar(1.0, A::mul(s.ad_value(225), s.ad_value(1555))), (1.0 / 1.414213562373095));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1632] != 0.0)) {
            s.store_div_ad_lhs(1576, A::neg(A::add(s.ad_value(1557), s.ad_value(1547))), 1555);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1632] != 0.0)) {
            s.store_add_ad(1577, A::sub(A::div(A::mul(A::square(s.ad_value(1574)), s.ad_value(1574)), A::mul(A::mul(A::scale(s.ad_value(1573), 27.0), s.ad_value(1573)), s.ad_value(1573))), A::div(A::mul(s.ad_value(1574), s.ad_value(1575)), A::mul(A::scale(s.ad_value(1573), 6.0), s.ad_value(1573)))), A::div(s.ad_value(1576), A::scale(s.ad_value(1573), 2.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1632] != 0.0)) {
            s.store_div_ad(1578, A::sub(A::mul(A::scale(s.ad_value(1573), 3.0), s.ad_value(1575)), A::square(s.ad_value(1574))), A::mul(A::scale(s.ad_value(1573), 9.0), s.ad_value(1573)));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1632] != 0.0)) {
            s.store_sqrt_ad(1526, A::add(A::square(s.ad_value(1577)), A::mul(A::square(s.ad_value(1578)), s.ad_value(1578))));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1632] != 0.0)) {
            s.store_powf_ad(1579, A::sub(s.ad_value(1526), s.ad_value(1577)), 0.3333333333333333);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1632] != 0.0)) {
            s.store_neg_ad(1580, A::powf(A::add(s.ad_value(1577), s.ad_value(1526)), 0.3333333333333333));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1632] != 0.0)) {
            s.store_sub_ad(1530, A::add(s.ad_value(1579), s.ad_value(1580)), A::div(s.ad_value(1574), A::scale(s.ad_value(1573), 3.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1632] != 0.0)) {
            s.store_sub_ad_lhs(1572, A::mul(s.ad_value(1530), s.ad_value(227)), 1547);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1632] != 0.0)) {
            s.store_mul_ad_rhs(1569, 225, A::add(s.ad_value(1572), s.ad_value(1547)));
        }

        s.v[1633] = if (p.p41 > 0.0) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_offset_ad(1581, A::add(s.ad_value(1557), s.ad_value(1547)), 0.1);
        }

    }

    pub(super) fn stamp_reactive_block_27(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_offset_ad(1588, A::exp(A::mul(s.ad_value(225), A::neg(s.ad_value(1547)))), 1e-50);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_scale(1521, 230, 1.0 / (s.v[69]));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_square(1582, 1521);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_mul(1583, 1582, 1588);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_mul(1521, 226, 1556);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_mul(1584, 225, 1581);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_add_ad(1585, A::sub(A::ln(A::add(A::mul(s.ad_value(1583), s.ad_value(1521)), A::square(s.ad_value(1584)))), A::ln(A::mul(s.ad_value(1582), s.ad_value(1521)))), A::mul(s.ad_value(225), s.ad_value(1547)));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_offset_ad(44, A::sub(s.ad_value(1584), s.ad_value(1585)), (-1.0));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_scale(45, 1584, 4.0);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_scale_ad(1522, A::offset(A::div(s.ad_value(44), s.ad_value(45)), 1.0), 0.5);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_scale_ad(1523, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(44), 2.0), s.ad_value(45))), 0.5);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_sub_ad_rhs(1585, 1584, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_sub(1584, 1584, 1585);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_add_ad_rhs(1584, 1584, A::scale(s.ad_value(225), 0.1));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_add_ad(1586, A::sub(A::ln(A::add(A::mul(s.ad_value(1583), s.ad_value(1521)), A::square(s.ad_value(1584)))), A::ln(A::mul(s.ad_value(1582), s.ad_value(1521)))), A::mul(s.ad_value(225), s.ad_value(1547)));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1633] != 0.0)) {
            s.copy_ad(1587, 1569);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_offset_ad(44, A::sub(s.ad_value(1586), s.ad_value(1587)), (-(0.0008 * 75.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_scale(45, 1586, (4.0 * (0.0008 * 75.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_scale_ad(1522, A::offset(A::div(s.ad_value(44), s.ad_value(45)), 1.0), 0.5);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_scale_ad(1523, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(44), ((2.0 * 0.0008) * 75.0)), s.ad_value(45))), 0.5);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_sub_ad_rhs(1569, 1586, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) {
            s.store_sub_ad_lhs(1571, A::div(s.ad_value(1569), s.ad_value(225)), 1547);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) {
            s.store_add_ad(1522, A::offset(s.ad_value(1569), (-1.0)), A::exp(A::neg(s.ad_value(1569))));
        }

        s.v[1634] = if (s.v[1522] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1634] != 0.0)) {
            s.store_scalar(1522, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) {
            s.store_sqrt(1523, 1522);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) {
            s.store_mul(458, 1554, 1523);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) {
            s.store_scaled_sub(459, 1557, 1571, s.v[1532]);
        }

        s.v[1635] = if (p.p41 == 1.0) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_exp_ad(1588, A::mul(s.ad_value(225), A::neg(s.ad_value(1547))));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_scale(1521, 230, 1.0 / (s.v[69]));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_square(1582, 1521);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_mul(1597, 1582, 1588);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_scalar(1544, 0.0);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_scalar(1591, 0.0);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_scalar(1595, 0.0);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_scalar(167, 1.0);
        }

        let mut assign31370_loop_guard: usize = 0;
        while {
            let assign31370_cond_e45508: f64 = (2.0 * 20.0);
            let assign31370_cond_e45510: f64 = (assign31370_cond_e45508 + 1.0);
            let assign31370_cond_e45512: f64 = if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) && (s.v[167] <= assign31370_cond_e45510)) { 1.0 } else { 0.0 };
            assign31370_cond_e45512 != 0.0
        } {
            assign31370_loop_guard += 1;
            assert!(assign31370_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) {
                s.store_scalar(1593, 0.0);
            }
            if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) {
                s.store_mul_ad_rhs(1569, 225, A::add(s.ad_value(1571), s.ad_value(1547)));
            }
            s.v[1636] = if (s.v[1569] < 5.0) { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) && (s.v[1636] != 0.0)) {
                s.store_mul_ad(1589, A::mul(A::square(s.ad_value(1569)), s.ad_value(1569)), A::offset(A::mul(s.ad_value(1569), A::offset(A::scale(s.ad_value(1569), 0.006115288895133179), (-0.07053654284009761))), 0.29693154855771));
            }
            if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) && (s.v[1636] != 0.0)) {
                s.store_mul_ad(1590, A::square(s.ad_value(1569)), A::offset(A::mul(s.ad_value(1569), A::offset(A::scale(s.ad_value(1569), (5.0 * 0.006115288895133179)), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
            }
            if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) && (s.v[1636] != 0.0)) {
                s.store_mul_ad_lhs(1591, A::mul(s.ad_value(1597), s.ad_value(1589)), 1589);
            }
            if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) && (s.v[1636] != 0.0)) {
                s.store_mul_ad_lhs(1592, A::mul(A::scale(A::mul(s.ad_value(1597), s.ad_value(225)), 2.0), s.ad_value(1589)), 1590);
            }
            if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) && (s.v[1636] != 0.0)) {
                s.store_mul_ad_rhs(1593, 1569, A::offset(A::mul(s.ad_value(1569), A::offset(A::mul(s.ad_value(1569), A::offset(A::mul(s.ad_value(1569), A::offset(A::scale(s.ad_value(1569), 6.36964918866352e-5), (-0.00163730162779191))), 0.0178800506338833)), (-0.117851130197758))), 0.707106781186548));
            }
            if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) && (s.v[1636] != 0.0)) {
                s.store_offset_ad(1594, A::mul(s.ad_value(1569), A::offset(A::mul(s.ad_value(1569), A::offset(A::mul(s.ad_value(1569), A::offset(A::scale(s.ad_value(1569), (5.0 * 6.36964918866352e-5)), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833))), (2.0 * (-0.117851130197758)))), 0.707106781186548);
            }
            if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) && (s.v[1636] != 0.0)) {
                s.store_sqrt_ad(1595, A::offset(A::add(A::square(s.ad_value(1593)), s.ad_value(1591)), 1e-50));
            }
            if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) && (s.v[1636] != 0.0)) {
                s.store_div_ad(1596, A::add(A::mul(A::scale(A::mul(s.ad_value(225), s.ad_value(1594)), 2.0), s.ad_value(1593)), s.ad_value(1592)), A::scale(s.ad_value(1595), 2.0));
            }
            s.v[1637] = if (s.v[1569] < 80.0) { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) && (!(s.v[1636] != 0.0))) && (s.v[1637] != 0.0)) {
                s.store_exp(243, 1569);
            }
            if (((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) && (!(s.v[1636] != 0.0))) && (s.v[1637] != 0.0)) {
                s.store_mul_ad_rhs(1591, 1597, A::offset(s.ad_value(243), (-1.0)));
            }
            if (((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) && (!(s.v[1636] != 0.0))) && (s.v[1637] != 0.0)) {
                s.store_mul_ad_lhs(1592, A::mul(s.ad_value(1597), s.ad_value(225)), 243);
            }
            if (((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) && (!(s.v[1636] != 0.0))) && (!(s.v[1637] != 0.0))) {
                s.store_exp_ad(1598, A::mul(s.ad_value(225), s.ad_value(1571)));
            }
            if (((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) && (!(s.v[1636] != 0.0))) && (!(s.v[1637] != 0.0))) {
                s.store_mul_ad_rhs(1591, 1582, A::sub(s.ad_value(1598), s.ad_value(1588)));
            }
            if (((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) && (!(s.v[1636] != 0.0))) && (!(s.v[1637] != 0.0))) {
                s.store_mul_ad_lhs(1592, A::mul(s.ad_value(1582), s.ad_value(225)), 1598);
            }
            if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) && (!(s.v[1636] != 0.0))) {
                s.store_sqrt_ad(1595, A::add(A::offset(s.ad_value(1569), (-1.0)), s.ad_value(1591)));
            }
            if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) && (!(s.v[1636] != 0.0))) {
                s.store_scale_ad(1596, A::div(A::add(s.ad_value(225), s.ad_value(1592)), s.ad_value(1595)), 0.5);
            }
            if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) {
                s.store_sub_ad(1599, A::sub(s.ad_value(1557), s.ad_value(1571)), A::mul(s.ad_value(1555), s.ad_value(1595)));
            }
            if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) {
                s.store_sub_from_scalar_ad(1600, (-1.0), A::mul(s.ad_value(1555), s.ad_value(1596)));
            }
            s.v[1638] = if (s.v[1544] == 1.0) { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) && (s.v[1638] != 0.0)) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) && (!(s.v[1638] != 0.0))) {
                s.store_div_ad_lhs(494, A::neg(s.ad_value(1599)), 1600);
            }
            if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) && (!(s.v[1638] != 0.0))) {
                s.store_scale_ad(1601, A::offset({
                    if (1.0 >= ((s.v[1571]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1571))
                    }
                }, 1.0), (0.5 * 0.1));
            }
            s.v[1639] = if (((s.v[494]) as f64).abs() > s.v[1601]) { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) && (!(s.v[1638] != 0.0))) && (s.v[1639] != 0.0)) {
                s.store_scale(494, 1601, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) && (!(s.v[1638] != 0.0))) {
                s.store_add(1571, 1571, 494);
            }
            s.v[1640] = if ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1599]) as f64).abs() <= 1e-8)) { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) && (!(s.v[1638] != 0.0))) && (s.v[1640] != 0.0)) {
                s.store_scalar(1544, 1.0);
            }
            if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.v[1642] = if (s.v[1569] < 5.0) { 1.0 } else { 0.0 };

        if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) && (s.v[1642] != 0.0)) {
            s.store_offset_ad(1602, A::square(s.ad_value(1593)), (10.0 * 2.220446049250313e-16));
        }

        if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) && (s.v[1642] != 0.0)) {
            s.store_offset(1603, 1593, (10.0 * 2.220446049250313e-16));
        }

        if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) && (!(s.v[1642] != 0.0))) {
            s.store_offset(1602, 1569, (-1.0));
        }

        if ((((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) && (!(s.v[1642] != 0.0))) {
            s.store_sqrt(1603, 1602);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_mul(458, 1554, 1603);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_div_from_scalar_ad(1522, 1.0, A::add(s.ad_value(1595), s.ad_value(1603)));
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_mul_ad_lhs(460, A::mul(s.ad_value(1554), s.ad_value(1591)), 1522);
        }

        if (((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_add(459, 458, 460);
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) {
            s.store_sub(460, 459, 458);
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) {
            s.store_ad(1525, &{
                if (p.p43 == 1.0) {
                    A::mul(s.ad_value(287), s.ad_value(1534))
                } else {
                    A::mul(s.ad_value(108), s.ad_value(1534))
                }
            });
        }

        s.v[1644] = if (((s.v[1540] != 0.0) && (p.p43 == 0.0)) || ((s.v[1538] != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1644] != 0.0)) {
            s.store_mul(455, 1525, 459);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1644] != 0.0)) {
            s.store_mul(457, 1525, 458);
        }

        s.v[1645] = if (((s.v[1541] != 0.0) && (p.p43 == 0.0)) || ((s.v[1539] != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1645] != 0.0)) {
            s.store_mul(454, 1525, 459);
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) && (s.v[1645] != 0.0)) {
            s.store_mul(456, 1525, 458);
        }

        if ((p.p24 != 0.0) && (s.v[1604] != 0.0)) {
            s.store_add_ad(266, A::scale(s.ad_value(462), s.v[566]), A::scale(s.ad_value(461), s.v[565]));
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (s.v[266] != 0.0)) {
            s.store_add_ad(269, A::scale(s.ad_value(462), p.p170), A::scale(s.ad_value(461), p.p169));
        }

        s.v[1646] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (s.v[266] != 0.0)) && (s.v[1646] != 0.0)) {
            s.store_add_ad(1522, A::mul(s.ad_value(462), s.ad_value(287)), A::mul(s.ad_value(461), s.ad_value(288)));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (s.v[266] != 0.0)) && (s.v[1646] != 0.0)) {
            s.store_mul_ad_rhs(269, 269, A::neg(s.ad_value(1522)));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (s.v[266] != 0.0)) && (!(s.v[1646] != 0.0))) {
            s.store_mul_ad_rhs(269, 269, A::neg(s.ad_value(108)));
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (s.v[266] != 0.0)) {
            s.store_add_ad_rhs(268, 268, A::mul(A::neg(s.ad_value(269)), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if ((p.p24 != 0.0) && (s.v[1604] != 0.0)) {
            s.store_add_ad(266, A::scale(s.ad_value(461), s.v[566]), A::scale(s.ad_value(462), s.v[565]));
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (s.v[266] != 0.0)) {
            s.store_add_ad(270, A::scale(s.ad_value(461), p.p170), A::scale(s.ad_value(462), p.p169));
        }

        s.v[1647] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (s.v[266] != 0.0)) && (s.v[1647] != 0.0)) {
            s.store_add_ad(1522, A::mul(s.ad_value(461), s.ad_value(287)), A::mul(s.ad_value(462), s.ad_value(288)));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (s.v[266] != 0.0)) && (s.v[1647] != 0.0)) {
            s.store_mul_ad_rhs(270, 270, A::neg(s.ad_value(1522)));
        }

        if ((((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (s.v[266] != 0.0)) && (!(s.v[1647] != 0.0))) {
            s.store_mul_ad_rhs(270, 270, A::neg(s.ad_value(108)));
        }

        if (((p.p24 != 0.0) && (s.v[1604] != 0.0)) && (s.v[266] != 0.0)) {
            s.store_add_ad_rhs(267, 267, A::mul(A::neg(s.ad_value(270)), s.ad_value(158)));
        }

        s.v[1648] = if (((s.v[613] == 1.0) && (!(s.v[565] != 0.0))) || ((s.v[613] != 1.0) && (!(s.v[566] != 0.0)))) { 1.0 } else { 0.0 };

        s.v[1649] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (!(s.v[1604] != 0.0))) && (s.v[1648] != 0.0)) && (s.v[1649] != 0.0)) {
            s.store_scale(269, 288, ((-s.v[1532]) * p.p188));
        }

        if ((((p.p24 != 0.0) && (!(s.v[1604] != 0.0))) && (s.v[1648] != 0.0)) && (!(s.v[1649] != 0.0))) {
            s.store_scale(269, 108, ((-s.v[1532]) * p.p188));
        }

        if (((p.p24 != 0.0) && (!(s.v[1604] != 0.0))) && (!(s.v[1648] != 0.0))) {
            s.store_add_ad(269, A::scale(s.ad_value(462), p.p170), A::scale(s.ad_value(461), p.p169));
        }

        s.v[1650] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (!(s.v[1604] != 0.0))) && (!(s.v[1648] != 0.0))) && (s.v[1650] != 0.0)) {
            s.store_add_ad(1522, A::mul(s.ad_value(462), s.ad_value(287)), A::mul(s.ad_value(461), s.ad_value(288)));
        }

        if ((((p.p24 != 0.0) && (!(s.v[1604] != 0.0))) && (!(s.v[1648] != 0.0))) && (s.v[1650] != 0.0)) {
            s.store_mul_ad_rhs(269, 269, A::neg(s.ad_value(1522)));
        }

        if ((((p.p24 != 0.0) && (!(s.v[1604] != 0.0))) && (!(s.v[1648] != 0.0))) && (!(s.v[1650] != 0.0))) {
            s.store_mul_ad_rhs(269, 269, A::neg(s.ad_value(108)));
        }

        if ((p.p24 != 0.0) && (!(s.v[1604] != 0.0))) {
            s.store_mul_ad(268, A::neg(s.ad_value(269)), A::sub(s.ad_value(158), s.ad_value(157)));
        }

        s.v[1651] = if (((s.v[613] == 1.0) && (!(s.v[566] != 0.0))) || ((s.v[613] != 1.0) && (!(s.v[565] != 0.0)))) { 1.0 } else { 0.0 };

        s.v[1652] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (!(s.v[1604] != 0.0))) && (s.v[1651] != 0.0)) && (s.v[1652] != 0.0)) {
            s.store_scale(270, 287, ((-s.v[1532]) * p.p188));
        }

        if ((((p.p24 != 0.0) && (!(s.v[1604] != 0.0))) && (s.v[1651] != 0.0)) && (!(s.v[1652] != 0.0))) {
            s.store_scale(270, 108, ((-s.v[1532]) * p.p188));
        }

        if (((p.p24 != 0.0) && (!(s.v[1604] != 0.0))) && (!(s.v[1651] != 0.0))) {
            s.store_add_ad(270, A::scale(s.ad_value(461), p.p170), A::scale(s.ad_value(462), p.p169));
        }

        s.v[1653] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (!(s.v[1604] != 0.0))) && (!(s.v[1651] != 0.0))) && (s.v[1653] != 0.0)) {
            s.store_add_ad(1522, A::mul(s.ad_value(461), s.ad_value(287)), A::mul(s.ad_value(462), s.ad_value(288)));
        }

        if ((((p.p24 != 0.0) && (!(s.v[1604] != 0.0))) && (!(s.v[1651] != 0.0))) && (s.v[1653] != 0.0)) {
            s.store_mul_ad_rhs(270, 270, A::neg(s.ad_value(1522)));
        }

        if ((((p.p24 != 0.0) && (!(s.v[1604] != 0.0))) && (!(s.v[1651] != 0.0))) && (!(s.v[1653] != 0.0))) {
            s.store_mul_ad_rhs(270, 270, A::neg(s.ad_value(108)));
        }

        if ((p.p24 != 0.0) && (!(s.v[1604] != 0.0))) {
            s.store_mul_ad_lhs(267, A::neg(s.ad_value(270)), 158);
        }

        s.v[1654] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1654] != 0.0) {
            s.copy_ad(1670, 590);
        }

        if (s.v[1654] != 0.0) {
            s.copy_ad(1671, 591);
        }

        if (s.v[1654] != 0.0) {
            s.store_scale_ad(1672, A::exp(A::scale(A::add(A::sub_from_scalar((s.v[87] * s.v[114]), A::mul(s.ad_value(237), s.ad_value(225))), A::scale(A::ln(A::scale(s.ad_value(429), 1.0 / (s.v[81]))), p.p175)), 1.0 / (p.p174))), p.p173);
        }

        if (s.v[1654] != 0.0) {
            s.store_scale_ad(1673, A::exp(A::scale(A::add(A::sub_from_scalar((s.v[87] * s.v[114]), A::mul(s.ad_value(237), s.ad_value(225))), A::scale(A::ln(A::scale(s.ad_value(429), 1.0 / (s.v[81]))), p.p176)), 1.0 / (p.p174))), p.p173);
        }

        if (s.v[1654] != 0.0) {
            s.store_mul_ad_lhs(1677, A::scale(s.ad_value(286), p.p237), 1672);
        }

        if (s.v[1654] != 0.0) {
            s.store_mul_ad_lhs(1679, A::scale(s.ad_value(286), p.p237), 1673);
        }

        if (s.v[1654] != 0.0) {
            s.store_mul_ad_lhs(1678, A::scale(s.ad_value(285), p.p237), 1672);
        }

        if (s.v[1654] != 0.0) {
            s.store_mul_ad_lhs(1680, A::scale(s.ad_value(285), p.p237), 1673);
        }

        if (s.v[1654] != 0.0) {
            s.store_scale(1656, 429, 1.0 / (s.v[81]));
        }

        if (s.v[1654] != 0.0) {
            s.store_offset(1657, 1677, 1e-50);
        }

        if (s.v[1654] != 0.0) {
            s.store_scale_ad(1675, A::div_from_scalar(p.p174, s.ad_value(225)), 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_28(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1654] != 0.0) {
            s.store_scale_ad(1676, A::div_from_scalar(p.p174, s.ad_value(225)), 0.0);
        }

        if (s.v[1654] != 0.0) {
            s.store_scale(1674, 227, p.p174);
        }

        s.v[1683] = if (s.v[1670] < s.v[1675]) { 1.0 } else { 0.0 };

        if ((s.v[1654] != 0.0) && (s.v[1683] != 0.0)) {
            s.store_exp_ad(1656, A::div(s.ad_value(1670), s.ad_value(1674)));
        }

        if ((s.v[1654] != 0.0) && (s.v[1683] != 0.0)) {
            s.store_mul_ad_rhs(282, 1677, A::offset(s.ad_value(1656), (-1.0)));
        }

        if ((s.v[1654] != 0.0) && (!(s.v[1683] != 0.0))) {
            s.store_exp_ad(1656, A::div(s.ad_value(1675), s.ad_value(1674)));
        }

        if ((s.v[1654] != 0.0) && (!(s.v[1683] != 0.0))) {
            s.store_add_ad(282, A::mul(s.ad_value(1677), A::offset(s.ad_value(1656), (-1.0))), A::mul(A::mul(A::div(s.ad_value(1677), s.ad_value(1674)), s.ad_value(1656)), A::sub(s.ad_value(1670), s.ad_value(1675))));
        }

        if (s.v[1654] != 0.0) {
            s.store_add_ad_rhs(282, 282, A::mul(A::scale(s.ad_value(1670), p.p178), s.ad_value(1679)));
        }

        s.v[1684] = if (s.v[1671] < s.v[1676]) { 1.0 } else { 0.0 };

        if ((s.v[1654] != 0.0) && (s.v[1684] != 0.0)) {
            s.store_exp_ad(1656, A::div(s.ad_value(1671), s.ad_value(1674)));
        }

        if ((s.v[1654] != 0.0) && (s.v[1684] != 0.0)) {
            s.store_mul_ad_rhs(281, 1678, A::offset(s.ad_value(1656), (-1.0)));
        }

        if ((s.v[1654] != 0.0) && (!(s.v[1684] != 0.0))) {
            s.store_exp_ad(1656, A::div(s.ad_value(1676), s.ad_value(1674)));
        }

        if ((s.v[1654] != 0.0) && (!(s.v[1684] != 0.0))) {
            s.store_add_ad(281, A::mul(s.ad_value(1678), A::offset(s.ad_value(1656), (-1.0))), A::mul(A::mul(A::div(s.ad_value(1678), s.ad_value(1674)), s.ad_value(1656)), A::sub(s.ad_value(1671), s.ad_value(1676))));
        }

        if (s.v[1654] != 0.0) {
            s.store_add_ad_rhs(281, 281, A::mul(A::scale(s.ad_value(1671), p.p178), s.ad_value(1680)));
        }

        if (s.v[1654] != 0.0) {
            s.store_add_ad_rhs(282, 282, A::scale(s.ad_value(1670), s.v[142]));
        }

        if (s.v[1654] != 0.0) {
            s.store_add_ad_rhs(281, 281, A::scale(s.ad_value(1671), s.v[142]));
        }

        if (s.v[1654] != 0.0) {
            s.store_scalar(1664, (p.p179 * p.p2));
        }

        if (s.v[1654] != 0.0) {
            s.store_scalar(1665, (p.p179 * p.p3));
        }

        if (s.v[1654] != 0.0) {
            s.store_scalar(1663, (p.p237 - p.p238));
        }

        s.v[1685] = if (s.v[1663] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1654] != 0.0) && (s.v[1685] != 0.0)) {
            s.store_scalar(1664, 0.0);
        }

        if ((s.v[1654] != 0.0) && (s.v[1685] != 0.0)) {
            s.store_scalar(1665, 0.0);
        }

        s.v[1686] = if (p.p5 > s.v[287]) { 1.0 } else { 0.0 };

        if ((s.v[1654] != 0.0) && (s.v[1686] != 0.0)) {
            s.store_scale_ad(1667, A::sub_from_scalar(p.p5, s.ad_value(287)), p.p180);
        }

        if ((s.v[1654] != 0.0) && (s.v[1686] != 0.0)) {
            s.store_scale(1669, 287, p.p181);
        }

        s.v[1687] = if (s.v[1671] < 0.0) { 1.0 } else { 0.0 };

        s.v[1688] = if (s.v[1665] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1654] != 0.0) && (s.v[1686] != 0.0)) && (s.v[1687] != 0.0)) && (s.v[1688] != 0.0)) {
            s.store_sub_from_scalar_ad(1681, 1.0, A::scale(s.ad_value(1671), 1.0 / (p.p185)));
        }

        s.v[1689] = if (p.p182 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1654] != 0.0) && (s.v[1686] != 0.0)) && (s.v[1687] != 0.0)) && (s.v[1688] != 0.0)) && (s.v[1689] != 0.0)) {
            s.store_div_from_scalar_ad(1682, 1.0, A::sqrt(s.ad_value(1681)));
        }

        if (((((s.v[1654] != 0.0) && (s.v[1686] != 0.0)) && (s.v[1687] != 0.0)) && (s.v[1688] != 0.0)) && (!(s.v[1689] != 0.0))) {
            s.store_powf(1682, 1681, (-p.p182));
        }

        if ((((s.v[1654] != 0.0) && (s.v[1686] != 0.0)) && (s.v[1687] != 0.0)) && (s.v[1688] != 0.0)) {
            s.store_scale_ad(283, A::mul(A::scale(s.ad_value(1665), p.p185), A::sub_from_scalar(1.0, A::mul(s.ad_value(1681), s.ad_value(1682)))), 1.0 / ((1.0 - p.p182)));
        }

        if ((((s.v[1654] != 0.0) && (s.v[1686] != 0.0)) && (s.v[1687] != 0.0)) && (!(s.v[1688] != 0.0))) {
            s.store_scalar(283, 0.0);
        }

        s.v[1690] = if (s.v[1667] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1654] != 0.0) && (s.v[1686] != 0.0)) && (s.v[1687] != 0.0)) && (s.v[1690] != 0.0)) {
            s.store_sub_from_scalar_ad(1681, 1.0, A::scale(s.ad_value(1671), 1.0 / (p.p186)));
        }

        s.v[1691] = if (p.p183 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1654] != 0.0) && (s.v[1686] != 0.0)) && (s.v[1687] != 0.0)) && (s.v[1690] != 0.0)) && (s.v[1691] != 0.0)) {
            s.store_div_from_scalar_ad(1682, 1.0, A::sqrt(s.ad_value(1681)));
        }

        if (((((s.v[1654] != 0.0) && (s.v[1686] != 0.0)) && (s.v[1687] != 0.0)) && (s.v[1690] != 0.0)) && (!(s.v[1691] != 0.0))) {
            s.store_powf(1682, 1681, (-p.p183));
        }

        if ((((s.v[1654] != 0.0) && (s.v[1686] != 0.0)) && (s.v[1687] != 0.0)) && (s.v[1690] != 0.0)) {
            s.store_add_ad_rhs(283, 283, A::scale(A::mul(A::scale(s.ad_value(1667), p.p186), A::sub_from_scalar(1.0, A::mul(s.ad_value(1681), s.ad_value(1682)))), 1.0 / ((1.0 - p.p183))));
        }

        s.v[1692] = if (s.v[1669] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1654] != 0.0) && (s.v[1686] != 0.0)) && (s.v[1687] != 0.0)) && (s.v[1692] != 0.0)) {
            s.store_sub_from_scalar_ad(1681, 1.0, A::scale(s.ad_value(1671), 1.0 / (p.p187)));
        }

        s.v[1693] = if (p.p184 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1654] != 0.0) && (s.v[1686] != 0.0)) && (s.v[1687] != 0.0)) && (s.v[1692] != 0.0)) && (s.v[1693] != 0.0)) {
            s.store_div_from_scalar_ad(1682, 1.0, A::sqrt(s.ad_value(1681)));
        }

        if (((((s.v[1654] != 0.0) && (s.v[1686] != 0.0)) && (s.v[1687] != 0.0)) && (s.v[1692] != 0.0)) && (!(s.v[1693] != 0.0))) {
            s.store_powf(1682, 1681, (-p.p184));
        }

        if ((((s.v[1654] != 0.0) && (s.v[1686] != 0.0)) && (s.v[1687] != 0.0)) && (s.v[1692] != 0.0)) {
            s.store_add_ad_rhs(283, 283, A::scale(A::mul(A::scale(s.ad_value(1669), p.p187), A::sub_from_scalar(1.0, A::mul(s.ad_value(1681), s.ad_value(1682)))), 1.0 / ((1.0 - p.p184))));
        }

        if (((s.v[1654] != 0.0) && (s.v[1686] != 0.0)) && (!(s.v[1687] != 0.0))) {
            s.store_add_ad_lhs(1656, A::add(s.ad_value(1665), s.ad_value(1667)), 1669);
        }

        if (((s.v[1654] != 0.0) && (s.v[1686] != 0.0)) && (!(s.v[1687] != 0.0))) {
            s.store_add_ad(1657, A::add(A::scale(s.ad_value(1665), (p.p182 * 1.0 / (p.p185))), A::scale(s.ad_value(1667), (p.p183 * 1.0 / (p.p186)))), A::scale(s.ad_value(1669), (p.p184 * 1.0 / (p.p187))));
        }

        if (((s.v[1654] != 0.0) && (s.v[1686] != 0.0)) && (!(s.v[1687] != 0.0))) {
            s.store_mul_ad_rhs(283, 1671, A::add(s.ad_value(1656), A::mul(A::scale(s.ad_value(1671), 0.5), s.ad_value(1657))));
        }

        if ((s.v[1654] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_scalar(1669, (p.p181 * p.p5));
        }

        s.v[1694] = if (s.v[1671] < 0.0) { 1.0 } else { 0.0 };

        s.v[1695] = if (s.v[1665] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1654] != 0.0) && (!(s.v[1686] != 0.0))) && (s.v[1694] != 0.0)) && (s.v[1695] != 0.0)) {
            s.store_sub_from_scalar_ad(1681, 1.0, A::scale(s.ad_value(1671), 1.0 / (p.p185)));
        }

        s.v[1696] = if (p.p182 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1654] != 0.0) && (!(s.v[1686] != 0.0))) && (s.v[1694] != 0.0)) && (s.v[1695] != 0.0)) && (s.v[1696] != 0.0)) {
            s.store_div_from_scalar_ad(1682, 1.0, A::sqrt(s.ad_value(1681)));
        }

        if (((((s.v[1654] != 0.0) && (!(s.v[1686] != 0.0))) && (s.v[1694] != 0.0)) && (s.v[1695] != 0.0)) && (!(s.v[1696] != 0.0))) {
            s.store_powf(1682, 1681, (-p.p182));
        }

        if ((((s.v[1654] != 0.0) && (!(s.v[1686] != 0.0))) && (s.v[1694] != 0.0)) && (s.v[1695] != 0.0)) {
            s.store_scale_ad(283, A::mul(A::scale(s.ad_value(1665), p.p185), A::sub_from_scalar(1.0, A::mul(s.ad_value(1681), s.ad_value(1682)))), 1.0 / ((1.0 - p.p182)));
        }

        if ((((s.v[1654] != 0.0) && (!(s.v[1686] != 0.0))) && (s.v[1694] != 0.0)) && (!(s.v[1695] != 0.0))) {
            s.store_scalar(283, 0.0);
        }

        s.v[1697] = if (s.v[1669] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1654] != 0.0) && (!(s.v[1686] != 0.0))) && (s.v[1694] != 0.0)) && (s.v[1697] != 0.0)) {
            s.store_sub_from_scalar_ad(1681, 1.0, A::scale(s.ad_value(1671), 1.0 / (p.p187)));
        }

        s.v[1698] = if (p.p184 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1654] != 0.0) && (!(s.v[1686] != 0.0))) && (s.v[1694] != 0.0)) && (s.v[1697] != 0.0)) && (s.v[1698] != 0.0)) {
            s.store_div_from_scalar_ad(1682, 1.0, A::sqrt(s.ad_value(1681)));
        }

        if (((((s.v[1654] != 0.0) && (!(s.v[1686] != 0.0))) && (s.v[1694] != 0.0)) && (s.v[1697] != 0.0)) && (!(s.v[1698] != 0.0))) {
            s.store_powf(1682, 1681, (-p.p184));
        }

        if ((((s.v[1654] != 0.0) && (!(s.v[1686] != 0.0))) && (s.v[1694] != 0.0)) && (s.v[1697] != 0.0)) {
            s.store_add_ad_rhs(283, 283, A::scale(A::mul(A::scale(s.ad_value(1669), p.p187), A::sub_from_scalar(1.0, A::mul(s.ad_value(1681), s.ad_value(1682)))), 1.0 / ((1.0 - p.p184))));
        }

        if (((s.v[1654] != 0.0) && (!(s.v[1686] != 0.0))) && (!(s.v[1694] != 0.0))) {
            s.store_add(1656, 1665, 1669);
        }

        if (((s.v[1654] != 0.0) && (!(s.v[1686] != 0.0))) && (!(s.v[1694] != 0.0))) {
            s.store_add_ad(1657, A::scale(s.ad_value(1665), (p.p182 * 1.0 / (p.p185))), A::scale(s.ad_value(1669), (p.p184 * 1.0 / (p.p187))));
        }

        if (((s.v[1654] != 0.0) && (!(s.v[1686] != 0.0))) && (!(s.v[1694] != 0.0))) {
            s.store_mul_ad_rhs(283, 1671, A::add(s.ad_value(1656), A::mul(A::scale(s.ad_value(1671), 0.5), s.ad_value(1657))));
        }

        s.v[1699] = if (p.p4 > s.v[288]) { 1.0 } else { 0.0 };

        if ((s.v[1654] != 0.0) && (s.v[1699] != 0.0)) {
            s.store_scale_ad(1666, A::sub_from_scalar(p.p4, s.ad_value(288)), p.p180);
        }

        if ((s.v[1654] != 0.0) && (s.v[1699] != 0.0)) {
            s.store_scale(1668, 288, p.p181);
        }

        s.v[1700] = if (s.v[1670] < 0.0) { 1.0 } else { 0.0 };

        s.v[1701] = if (s.v[1664] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1654] != 0.0) && (s.v[1699] != 0.0)) && (s.v[1700] != 0.0)) && (s.v[1701] != 0.0)) {
            s.store_sub_from_scalar_ad(1681, 1.0, A::scale(s.ad_value(1670), 1.0 / (p.p185)));
        }

        s.v[1702] = if (p.p182 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1654] != 0.0) && (s.v[1699] != 0.0)) && (s.v[1700] != 0.0)) && (s.v[1701] != 0.0)) && (s.v[1702] != 0.0)) {
            s.store_div_from_scalar_ad(1682, 1.0, A::sqrt(s.ad_value(1681)));
        }

        if (((((s.v[1654] != 0.0) && (s.v[1699] != 0.0)) && (s.v[1700] != 0.0)) && (s.v[1701] != 0.0)) && (!(s.v[1702] != 0.0))) {
            s.store_powf(1682, 1681, (-p.p182));
        }

        if ((((s.v[1654] != 0.0) && (s.v[1699] != 0.0)) && (s.v[1700] != 0.0)) && (s.v[1701] != 0.0)) {
            s.store_scale_ad(284, A::mul(A::scale(s.ad_value(1664), p.p185), A::sub_from_scalar(1.0, A::mul(s.ad_value(1681), s.ad_value(1682)))), 1.0 / ((1.0 - p.p182)));
        }

        if ((((s.v[1654] != 0.0) && (s.v[1699] != 0.0)) && (s.v[1700] != 0.0)) && (!(s.v[1701] != 0.0))) {
            s.store_scalar(284, 0.0);
        }

        s.v[1703] = if (s.v[1666] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1654] != 0.0) && (s.v[1699] != 0.0)) && (s.v[1700] != 0.0)) && (s.v[1703] != 0.0)) {
            s.store_sub_from_scalar_ad(1681, 1.0, A::scale(s.ad_value(1670), 1.0 / (p.p186)));
        }

        s.v[1704] = if (p.p183 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1654] != 0.0) && (s.v[1699] != 0.0)) && (s.v[1700] != 0.0)) && (s.v[1703] != 0.0)) && (s.v[1704] != 0.0)) {
            s.store_div_from_scalar_ad(1682, 1.0, A::sqrt(s.ad_value(1681)));
        }

        if (((((s.v[1654] != 0.0) && (s.v[1699] != 0.0)) && (s.v[1700] != 0.0)) && (s.v[1703] != 0.0)) && (!(s.v[1704] != 0.0))) {
            s.store_powf(1682, 1681, (-p.p183));
        }

        if ((((s.v[1654] != 0.0) && (s.v[1699] != 0.0)) && (s.v[1700] != 0.0)) && (s.v[1703] != 0.0)) {
            s.store_add_ad_rhs(284, 284, A::scale(A::mul(A::scale(s.ad_value(1666), p.p186), A::sub_from_scalar(1.0, A::mul(s.ad_value(1681), s.ad_value(1682)))), 1.0 / ((1.0 - p.p183))));
        }

        s.v[1705] = if (s.v[1668] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1654] != 0.0) && (s.v[1699] != 0.0)) && (s.v[1700] != 0.0)) && (s.v[1705] != 0.0)) {
            s.store_sub_from_scalar_ad(1681, 1.0, A::scale(s.ad_value(1670), 1.0 / (p.p187)));
        }

        s.v[1706] = if (p.p184 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1654] != 0.0) && (s.v[1699] != 0.0)) && (s.v[1700] != 0.0)) && (s.v[1705] != 0.0)) && (s.v[1706] != 0.0)) {
            s.store_div_from_scalar_ad(1682, 1.0, A::sqrt(s.ad_value(1681)));
        }

        if (((((s.v[1654] != 0.0) && (s.v[1699] != 0.0)) && (s.v[1700] != 0.0)) && (s.v[1705] != 0.0)) && (!(s.v[1706] != 0.0))) {
            s.store_powf(1682, 1681, (-p.p184));
        }

        if ((((s.v[1654] != 0.0) && (s.v[1699] != 0.0)) && (s.v[1700] != 0.0)) && (s.v[1705] != 0.0)) {
            s.store_add_ad_rhs(284, 284, A::scale(A::mul(A::scale(s.ad_value(1668), p.p187), A::sub_from_scalar(1.0, A::mul(s.ad_value(1681), s.ad_value(1682)))), 1.0 / ((1.0 - p.p184))));
        }

        if (((s.v[1654] != 0.0) && (s.v[1699] != 0.0)) && (!(s.v[1700] != 0.0))) {
            s.store_add_ad_lhs(1656, A::add(s.ad_value(1664), s.ad_value(1666)), 1668);
        }

        if (((s.v[1654] != 0.0) && (s.v[1699] != 0.0)) && (!(s.v[1700] != 0.0))) {
            s.store_add_ad(1657, A::add(A::scale(s.ad_value(1664), (p.p182 * 1.0 / (p.p185))), A::scale(s.ad_value(1666), (p.p183 * 1.0 / (p.p186)))), A::scale(s.ad_value(1668), (p.p184 * 1.0 / (p.p187))));
        }

        if (((s.v[1654] != 0.0) && (s.v[1699] != 0.0)) && (!(s.v[1700] != 0.0))) {
            s.store_mul_ad_rhs(284, 1670, A::add(s.ad_value(1656), A::mul(A::scale(s.ad_value(1670), 0.5), s.ad_value(1657))));
        }

        if ((s.v[1654] != 0.0) && (!(s.v[1699] != 0.0))) {
            s.store_scalar(1668, (p.p181 * p.p4));
        }

        s.v[1707] = if (s.v[1670] < 0.0) { 1.0 } else { 0.0 };

        s.v[1708] = if (s.v[1664] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1654] != 0.0) && (!(s.v[1699] != 0.0))) && (s.v[1707] != 0.0)) && (s.v[1708] != 0.0)) {
            s.store_sub_from_scalar_ad(1681, 1.0, A::scale(s.ad_value(1670), 1.0 / (p.p185)));
        }

        s.v[1709] = if (p.p182 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1654] != 0.0) && (!(s.v[1699] != 0.0))) && (s.v[1707] != 0.0)) && (s.v[1708] != 0.0)) && (s.v[1709] != 0.0)) {
            s.store_div_from_scalar_ad(1682, 1.0, A::sqrt(s.ad_value(1681)));
        }

        if (((((s.v[1654] != 0.0) && (!(s.v[1699] != 0.0))) && (s.v[1707] != 0.0)) && (s.v[1708] != 0.0)) && (!(s.v[1709] != 0.0))) {
            s.store_powf(1682, 1681, (-p.p182));
        }

        if ((((s.v[1654] != 0.0) && (!(s.v[1699] != 0.0))) && (s.v[1707] != 0.0)) && (s.v[1708] != 0.0)) {
            s.store_scale_ad(284, A::mul(A::scale(s.ad_value(1664), p.p185), A::sub_from_scalar(1.0, A::mul(s.ad_value(1681), s.ad_value(1682)))), 1.0 / ((1.0 - p.p182)));
        }

        if ((((s.v[1654] != 0.0) && (!(s.v[1699] != 0.0))) && (s.v[1707] != 0.0)) && (!(s.v[1708] != 0.0))) {
            s.store_scalar(284, 0.0);
        }

        s.v[1710] = if (s.v[1668] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1654] != 0.0) && (!(s.v[1699] != 0.0))) && (s.v[1707] != 0.0)) && (s.v[1710] != 0.0)) {
            s.store_sub_from_scalar_ad(1681, 1.0, A::scale(s.ad_value(1670), 1.0 / (p.p187)));
        }

        s.v[1711] = if (p.p184 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1654] != 0.0) && (!(s.v[1699] != 0.0))) && (s.v[1707] != 0.0)) && (s.v[1710] != 0.0)) && (s.v[1711] != 0.0)) {
            s.store_div_from_scalar_ad(1682, 1.0, A::sqrt(s.ad_value(1681)));
        }

        if (((((s.v[1654] != 0.0) && (!(s.v[1699] != 0.0))) && (s.v[1707] != 0.0)) && (s.v[1710] != 0.0)) && (!(s.v[1711] != 0.0))) {
            s.store_powf(1682, 1681, (-p.p184));
        }

        if ((((s.v[1654] != 0.0) && (!(s.v[1699] != 0.0))) && (s.v[1707] != 0.0)) && (s.v[1710] != 0.0)) {
            s.store_add_ad_rhs(284, 284, A::scale(A::mul(A::scale(s.ad_value(1668), p.p187), A::sub_from_scalar(1.0, A::mul(s.ad_value(1681), s.ad_value(1682)))), 1.0 / ((1.0 - p.p184))));
        }

        if (((s.v[1654] != 0.0) && (!(s.v[1699] != 0.0))) && (!(s.v[1707] != 0.0))) {
            s.store_add(1656, 1664, 1668);
        }

        if (((s.v[1654] != 0.0) && (!(s.v[1699] != 0.0))) && (!(s.v[1707] != 0.0))) {
            s.store_add_ad(1657, A::scale(s.ad_value(1664), (p.p182 * 1.0 / (p.p185))), A::scale(s.ad_value(1668), (p.p184 * 1.0 / (p.p187))));
        }

        if (((s.v[1654] != 0.0) && (!(s.v[1699] != 0.0))) && (!(s.v[1707] != 0.0))) {
            s.store_mul_ad_rhs(284, 1670, A::add(s.ad_value(1656), A::mul(A::scale(s.ad_value(1670), 0.5), s.ad_value(1657))));
        }

        s.v[1712] = if (s.v[1665] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1654] != 0.0) && (s.v[1712] != 0.0)) {
            s.store_scale_ad(1659, A::mul(A::scale(s.ad_value(544), (-1.6021918e-19)), s.ad_value(1663)), p.p3);
        }

        if ((s.v[1654] != 0.0) && (s.v[1712] != 0.0)) {
            s.store_scale_ad(1661, A::neg(s.ad_value(1659)), 0.001);
        }

        if ((s.v[1654] != 0.0) && (s.v[1712] != 0.0)) {
            s.store_sub_ad_lhs(44, A::sub(A::neg(s.ad_value(1659)), A::neg(s.ad_value(283))), 1661);
        }

        if ((s.v[1654] != 0.0) && (s.v[1712] != 0.0)) {
            s.store_mul_ad_lhs(45, A::scale(A::neg(s.ad_value(1659)), 4.0), 1661);
        }

        if ((s.v[1654] != 0.0) && (s.v[1712] != 0.0)) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((s.v[1654] != 0.0) && (s.v[1712] != 0.0)) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((s.v[1654] != 0.0) && (s.v[1712] != 0.0)) {
            s.store_sub_ad(283, A::neg(s.ad_value(1659)), A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((s.v[1654] != 0.0) && (s.v[1712] != 0.0)) {
            s.store_scale(283, 283, (-1.0));
        }

        s.v[1713] = if (s.v[1664] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1654] != 0.0) && (s.v[1713] != 0.0)) {
            s.store_scale_ad(1660, A::mul(A::scale(s.ad_value(544), (-1.6021918e-19)), s.ad_value(1663)), p.p2);
        }

        if ((s.v[1654] != 0.0) && (s.v[1713] != 0.0)) {
            s.store_scale_ad(1662, A::neg(s.ad_value(1660)), 0.001);
        }

        if ((s.v[1654] != 0.0) && (s.v[1713] != 0.0)) {
            s.store_sub_ad_lhs(44, A::sub(A::neg(s.ad_value(1660)), A::neg(s.ad_value(284))), 1662);
        }

        if ((s.v[1654] != 0.0) && (s.v[1713] != 0.0)) {
            s.store_mul_ad_lhs(45, A::scale(A::neg(s.ad_value(1660)), 4.0), 1662);
        }

        if ((s.v[1654] != 0.0) && (s.v[1713] != 0.0)) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((s.v[1654] != 0.0) && (s.v[1713] != 0.0)) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((s.v[1654] != 0.0) && (s.v[1713] != 0.0)) {
            s.store_sub_ad(284, A::neg(s.ad_value(1660)), A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((s.v[1654] != 0.0) && (s.v[1713] != 0.0)) {
            s.store_scale(284, 284, (-1.0));
        }

        s.v[1746] = if ((p.p32 != 0.0) && (!(s.v[145] != 0.0))) { 1.0 } else { 0.0 };

        if (s.v[1746] != 0.0) {
            s.store_div_ad_lhs(1729, A::sub(s.ad_value(314), s.ad_value(161)), 441);
        }

        if (s.v[1746] != 0.0) {
            s.store_scaled_mul(1730, 251, 1729, 1e-5);
        }

        s.v[1747] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1746] != 0.0) && (s.v[1747] != 0.0)) {
            s.store_scalar(1731, 1.0);
        }

        s.v[1748] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1746] != 0.0) && (!(s.v[1747] != 0.0))) && (s.v[1748] != 0.0)) {
            s.copy_ad(1731, 1730);
        }

        if (((s.v[1746] != 0.0) && (!(s.v[1747] != 0.0))) && (!(s.v[1748] != 0.0))) {
            s.store_powf(1731, 1730, (p.p113 - 1.0));
        }

        if (s.v[1746] != 0.0) {
            s.store_mul(1732, 1730, 1731);
        }

        if (s.v[1746] != 0.0) {
            s.store_offset(1733, 1732, 1.0);
        }

        if (s.v[1746] != 0.0) {
            s.store_powf(1734, 1733, (((-1.0) / p.p113) - 1.0));
        }

        if (s.v[1746] != 0.0) {
            s.store_mul(1735, 1733, 1734);
        }

        if (s.v[1746] != 0.0) {
            s.store_mul(293, 251, 1735);
        }

        if (s.v[1746] != 0.0) {
            s.store_scaled_add(1737, 250, 293, 0.5);
        }

    }

    pub(super) fn stamp_reactive_block_29(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1746] != 0.0) {
            s.store_square(1736, 190);
        }

        if (s.v[1746] != 0.0) {
            let assign33730_ad_e48923: A = A::add(A::add(A::mul(A::mul(A::add(A::offset(A::scale(s.ad_value(190), 3.0), 1.0), A::scale(s.ad_value(1736), 6.0)), s.ad_value(293)), s.ad_value(293)), A::mul(A::mul(A::add(A::offset(A::scale(s.ad_value(190), 4.0), 3.0), A::scale(s.ad_value(1736), 3.0)), s.ad_value(293)), s.ad_value(250))), A::mul(A::mul(A::add(A::offset(A::scale(s.ad_value(190), 3.0), 6.0), s.ad_value(1736)), s.ad_value(250)), s.ad_value(250)));
            s.store_div_ad(292, A::mul(A::mul(A::mul(A::mul(s.ad_value(107), s.ad_value(323)), s.ad_value(192)), s.ad_value(250)), assign33730_ad_e48923), A::mul(A::mul(A::mul(A::scale(s.ad_value(441), 15.0), A::offset(s.ad_value(190), 1.0)), s.ad_value(1737)), s.ad_value(1737)));
        }

        if (!(s.v[1746] != 0.0)) {
            s.store_scalar(292, 0.0);
        }

        s.v[1749] = if ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (s.v[146] == 1.0)) && (!(s.v[145] != 0.0))) { 1.0 } else { 0.0 };

        if (s.v[1749] != 0.0) {
            s.store_sqrt(298, 296);
        }

        if (s.v[1749] != 0.0) {
            s.store_add(1738, 192, 298);
        }

        if (s.v[1749] != 0.0) {
            s.store_square(1739, 294);
        }

        if (s.v[1749] != 0.0) {
            s.store_square(1740, 296);
        }

        if (s.v[1749] != 0.0) {
            s.store_mul_ad_lhs(1741, A::scale(s.ad_value(294), 42.0), 296);
        }

        if (s.v[1749] != 0.0) {
            s.store_add_ad_rhs(1741, 1741, A::scale(A::add(s.ad_value(1739), s.ad_value(1740)), 4.0));
        }

        if (s.v[1749] != 0.0) {
            s.store_add_ad_rhs(1741, 1741, A::mul(A::mul(A::scale(s.ad_value(298), 20.0), s.ad_value(192)), A::add(s.ad_value(294), s.ad_value(296))));
        }

        if (s.v[1749] != 0.0) {
            s.store_square(1742, 1738);
        }

        if (s.v[1749] != 0.0) {
            s.store_square(1734, 1742);
        }

        if (s.v[1749] != 0.0) {
            s.store_div_ad_rhs(299, 1741, A::mul(s.ad_value(1734), s.ad_value(1738)));
        }

        if (s.v[1749] != 0.0) {
            s.store_mul_ad_lhs(300, A::mul(A::div(s.ad_value(107), s.ad_value(441)), s.ad_value(250)), 323);
        }

        s.store_add(199, 199, 265);

        s.v[1750] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1750] != 0.0) {
            s.store_add(271, 531, 532);
        }

        if ((s.v[1750] != 0.0) && (s.v[564] != 0.0)) {
            s.store_offset(271, 271, (-(p.p168 * s.v[99])));
        }

        if (s.v[1750] != 0.0) {
            s.store_mul_ad(272, A::neg(s.ad_value(271)), A::sub(s.ad_value(158), s.ad_value(513)));
        }

        if (s.v[1750] != 0.0) {
            s.store_scalar(276, 0.0);
        }

        if (s.v[1750] != 0.0) {
            s.store_mul_ad(274, A::scale(s.ad_value(276), p.p9), A::offset(s.ad_value(518), s.v[101]));
        }

        if (s.v[1750] != 0.0) {
            s.store_mul_ad(275, A::scale(s.ad_value(276), p.p9), A::offset(s.ad_value(519), s.v[101]));
        }

        if (s.v[1750] != 0.0) {
            s.store_mul_ad_rhs(277, 274, A::sub(s.ad_value(158), s.ad_value(157)));
        }

        if (s.v[1750] != 0.0) {
            s.store_mul(278, 275, 158);
        }

        if (s.v[1750] != 0.0) {
            s.store_mul_ad(279, A::scale(s.ad_value(276), (p.p19 * p.p9)), A::sub(s.ad_value(158), s.ad_value(513)));
        }

        if (s.v[1750] != 0.0) {
            s.store_add(268, 268, 277);
        }

        if (s.v[1750] != 0.0) {
            s.store_add(267, 267, 278);
        }

        if (s.v[1750] != 0.0) {
            s.store_add(272, 272, 279);
        }

        if ((!(s.v[1750] != 0.0)) && (s.v[564] != 0.0)) {
            s.store_scalar(271, ((-p.p168) * s.v[99]));
        }

        if ((!(s.v[1750] != 0.0)) && (s.v[564] != 0.0)) {
            s.store_mul_ad(272, A::neg(s.ad_value(271)), A::sub(s.ad_value(158), s.ad_value(513)));
        }

        if ((!(s.v[1750] != 0.0)) && (!(s.v[564] != 0.0))) {
            s.store_scalar(271, 0.0);
        }

        if ((!(s.v[1750] != 0.0)) && (!(s.v[564] != 0.0))) {
            s.store_scalar(272, 0.0);
        }

        if (!(s.v[1750] != 0.0)) {
            s.store_scalar(273, 0.0);
        }

        if (!(s.v[1750] != 0.0)) {
            s.copy_ad(274, 273);
        }

        if (!(s.v[1750] != 0.0)) {
            s.copy_ad(275, 273);
        }

        if (!(s.v[1750] != 0.0)) {
            s.store_mul_ad_rhs(277, 274, A::sub(s.ad_value(158), s.ad_value(157)));
        }

        if (!(s.v[1750] != 0.0)) {
            s.store_mul(278, 275, 158);
        }

        if (!(s.v[1750] != 0.0)) {
            s.store_add(268, 268, 277);
        }

        if (!(s.v[1750] != 0.0)) {
            s.store_add(267, 267, 278);
        }

        s.store_scale(9, 199, s.v[451]);

        if (s.v[85] != 0.0) {
            s.store_scalar(24, 0.0);
        }

        if (s.v[85] != 0.0) {
            s.store_scalar(23, 0.0);
        }

        s.v[1751] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[85] != 0.0) && (s.v[1751] != 0.0)) {
            s.store_scalar(25, 0.0);
        }

        if ((s.v[85] != 0.0) && (s.v[1751] != 0.0)) {
            s.copy_ad(556, 438);
        }

        if ((s.v[85] != 0.0) && (!(s.v[1751] != 0.0))) {
            s.store_scalar(554, 0.0);
        }

        s.v[1752] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[85] != 0.0)) && (s.v[1752] != 0.0)) {
            s.store_scale_ad(23, A::sub(A::neg(s.ad_value(196)), s.ad_value(197)), s.v[451]);
        }

        if ((!(s.v[85] != 0.0)) && (s.v[1752] != 0.0)) {
            s.store_scale(24, 198, s.v[451]);
        }

        if ((!(s.v[85] != 0.0)) && (s.v[1752] != 0.0)) {
            s.store_scaled_sub(25, 197, 198, s.v[451]);
        }

        if ((!(s.v[85] != 0.0)) && (!(s.v[1752] != 0.0))) {
            s.store_scale_ad(23, A::sub(A::sub(A::sub(A::neg(s.ad_value(392)), s.ad_value(197)), s.ad_value(476)), s.ad_value(477)), s.v[451]);
        }

        if ((!(s.v[85] != 0.0)) && (!(s.v[1752] != 0.0))) {
            s.store_scaled_add(24, 198, 477, s.v[451]);
        }

        if ((!(s.v[85] != 0.0)) && (!(s.v[1752] != 0.0))) {
            s.store_scale_ad(25, A::add(A::sub(s.ad_value(197), s.ad_value(198)), s.ad_value(476)), s.v[451]);
        }

        s.v[1758] = if (p.p64 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1758] != 0.0) {
            s.store_scalar(280, 0.0);
        }

        if (!(s.v[1758] != 0.0)) {
            s.store_add_ad_lhs(1753, A::scale(s.ad_value(315), s.v[97]), 161);
        }

        s.v[1759] = if (s.v[1753] > s.v[314]) { 1.0 } else { 0.0 };

        if ((!(s.v[1758] != 0.0)) && (s.v[1759] != 0.0)) {
            s.copy_ad(1753, 314);
        }

        if (!(s.v[1758] != 0.0)) {
            s.store_add_ad(1754, A::scale(A::add(s.ad_value(157), s.ad_value(161)), s.v[317]), A::scale(s.ad_value(1753), (1.0 - s.v[317])));
        }

        if (!(s.v[1758] != 0.0)) {
            s.store_sqrt_ad(1755, A::div_from_scalar((2.0 * 1.034943e-10), s.ad_value(229)));
        }

        if (!(s.v[1758] != 0.0)) {
            s.store_scale(1756, 1755, 1.3);
        }

        if (!(s.v[1758] != 0.0)) {
            s.store_mul_ad_lhs(1757, A::scale(s.ad_value(108), 1.034943e-10), 1756);
        }

        if (!(s.v[1758] != 0.0)) {
            s.store_mul_ad_lhs(280, A::sub(A::scale(A::sub(A::add(s.ad_value(161), s.ad_value(157)), s.ad_value(1754)), 1.0 / (p.p64)), s.ad_value(315)), 1757);
        }

        s.v[1760] = if (p.p65 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1760] != 0.0) {
            s.store_add_ad_rhs(280, 280, A::mul(s.ad_value(135), s.ad_value(513)));
        }

        s.v[1761] = if (p.p24 == 1.0) { 1.0 } else { 0.0 };

        s.v[1762] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1761] != 0.0) && (s.v[1762] != 0.0)) {
            s.store_sub_ad_lhs(471, A::sub(A::sub(A::neg(s.ad_value(463)), s.ad_value(464)), s.ad_value(467)), 468);
        }

        if ((s.v[1761] != 0.0) && (s.v[1762] != 0.0)) {
            s.store_add(472, 466, 470);
        }

        if ((s.v[1761] != 0.0) && (s.v[1762] != 0.0)) {
            s.store_add(473, 465, 469);
        }

        if ((s.v[1761] != 0.0) && (s.v[1762] != 0.0)) {
            s.store_add_ad_rhs(23, 23, A::scale(A::add(A::sub(A::sub(A::sub(A::add(A::add(s.ad_value(268), s.ad_value(267)), s.ad_value(272)), s.ad_value(280)), s.ad_value(455)), s.ad_value(454)), s.ad_value(471)), s.v[451]));
        }

        if ((s.v[1761] != 0.0) && (s.v[1762] != 0.0)) {
            s.store_add_ad_rhs(24, 24, A::scale(A::add(A::add(A::sub(s.ad_value(280), s.ad_value(268)), s.ad_value(456)), s.ad_value(472)), s.v[451]));
        }

        if ((s.v[1761] != 0.0) && (s.v[1762] != 0.0)) {
            s.store_add_ad_rhs(25, 25, A::scale(A::add(A::sub(s.ad_value(457), s.ad_value(267)), s.ad_value(473)), s.v[451]));
        }

        if ((s.v[1761] != 0.0) && (!(s.v[1762] != 0.0))) {
            s.store_add_ad_rhs(23, 23, A::scale(A::sub(A::sub(A::sub(A::add(A::add(s.ad_value(268), s.ad_value(267)), s.ad_value(272)), s.ad_value(280)), s.ad_value(455)), s.ad_value(454)), s.v[451]));
        }

        if ((s.v[1761] != 0.0) && (!(s.v[1762] != 0.0))) {
            s.store_add_ad_rhs(24, 24, A::scale(A::add(A::sub(s.ad_value(280), s.ad_value(268)), s.ad_value(456)), s.v[451]));
        }

        if ((s.v[1761] != 0.0) && (!(s.v[1762] != 0.0))) {
            s.store_add_ad_rhs(25, 25, A::scale(A::sub(s.ad_value(457), s.ad_value(267)), s.v[451]));
        }

        s.v[1763] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1763] != 0.0) {
            s.store_scale(36, 281, s.v[451]);
        }

        if (s.v[1763] != 0.0) {
            s.store_scale(35, 282, s.v[451]);
        }

        if (s.v[1763] != 0.0) {
            s.store_scale(560, 284, s.v[451]);
        }

        if (s.v[1763] != 0.0) {
            s.store_scale(561, 283, s.v[451]);
        }

        if (!(s.v[1763] != 0.0)) {
            s.store_scalar(36, 0.0);
        }

        if (!(s.v[1763] != 0.0)) {
            s.store_scalar(35, 0.0);
        }

        if (!(s.v[1763] != 0.0)) {
            s.store_scalar(560, 0.0);
        }

        if (!(s.v[1763] != 0.0)) {
            s.store_scalar(561, 0.0);
        }

        s.v[1764] = if (p.p25 != 1.0) { 1.0 } else { 0.0 };

        if (s.v[1764] != 0.0) {
            s.store_scalar(557, 0.0);
        }

        if (!(s.v[1764] != 0.0)) {
            s.store_scale(557, 263, s.v[451]);
        }

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

        s.v[1773] = if ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (s.v[146] == 1.0)) && (!(s.v[145] != 0.0))) { 1.0 } else { 0.0 };

        if (s.v[1773] != 0.0) {
            s.store_scale_ad(1767, A::mul(A::scale(s.ad_value(323), 1e-6), s.ad_value(108)), s.v[98]);
        }

        if (s.v[1773] != 0.0) {
            s.store_scale(1768, 555, 1.0 / (s.v[451]));
        }

        if (s.v[1773] != 0.0) {
            s.store_div_ad_lhs(1769, A::mul(A::mul(A::scale(s.ad_value(227), (0.1185185185185185 * 1.6021918e-19)), s.ad_value(1768)), s.ad_value(1768)), 300);
        }

        s.v[1774] = if ((s.v[297] > (10.0 * 2.220446049250313e-16)) && (s.v[157] > (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if ((s.v[1773] != 0.0) && (s.v[1774] != 0.0)) {
            s.store_div(1770, 251, 250);
        }

        if ((s.v[1773] != 0.0) && (s.v[1774] != 0.0)) {
            s.store_div_ad_lhs(1771, A::sub(A::div(s.ad_value(251), s.ad_value(293)), s.ad_value(1770)), 157);
        }

        if ((s.v[1773] != 0.0) && (s.v[1774] != 0.0)) {
            s.store_add_ad_rhs(1772, 1770, A::div(A::mul(A::scale(s.ad_value(1771), 0.6666666666666667), A::add(A::add(s.ad_value(294), A::mul(s.ad_value(192), s.ad_value(298))), s.ad_value(296))), A::add(s.ad_value(192), s.ad_value(298))));
        }

        if ((s.v[1773] != 0.0) && (!(s.v[1774] != 0.0))) {
            s.store_div(1772, 251, 293);
        }

        if (s.v[1773] != 0.0) {
            s.store_mul_ad_lhs(558, A::mul(A::scale(s.ad_value(1769), s.v[451]), s.ad_value(299)), 1772);
        }

        if (s.v[1773] != 0.0) {
            s.store_ad(558, &{
                if (((-s.v[1768]) > s.v[1767]) && (s.v[558] > 0.0)) {
                    s.ad_value(558)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (!(s.v[1773] != 0.0)) {
            s.store_scalar(558, 0.0);
        }

        s.v[1775] = if (p.p259 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1775] != 0.0) {
            s.store_scalar(3, 1.0);
        }

        s.v[1795] = if (s.v[3] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1775] != 0.0) && (s.v[1795] != 0.0)) {
            s.store_scalar(1779, p.p266);
        }

        if ((s.v[1775] != 0.0) && (s.v[1795] != 0.0)) {
            s.store_scalar(1780, p.p268);
        }

        if ((s.v[1775] != 0.0) && (s.v[1795] != 0.0)) {
            s.store_scalar(1781, p.p273);
        }

        if ((s.v[1775] != 0.0) && (s.v[1795] != 0.0)) {
            s.store_scalar(1785, p.p258);
        }

        if ((s.v[1775] != 0.0) && (s.v[1795] != 0.0)) {
            s.store_ad(1783, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(2)), p.p50));
        }

        if ((s.v[1775] != 0.0) && (!(s.v[1795] != 0.0))) {
            s.store_scalar(1779, p.p265);
        }

        if ((s.v[1775] != 0.0) && (!(s.v[1795] != 0.0))) {
            s.store_scalar(1780, p.p267);
        }

        if ((s.v[1775] != 0.0) && (!(s.v[1795] != 0.0))) {
            s.store_scalar(1781, p.p272);
        }

        if ((s.v[1775] != 0.0) && (!(s.v[1795] != 0.0))) {
            s.store_scalar(1785, p.p257);
        }

        if ((s.v[1775] != 0.0) && (!(s.v[1795] != 0.0))) {
            s.store_ad(1783, &A::scale(A::voltage(ctx, &nodes, Some(0), Some(6)), p.p50));
        }

        if (s.v[1775] != 0.0) {
            s.store_scale(1779, 1779, 0.0001);
        }

        if (s.v[1775] != 0.0) {
            s.store_scale(1780, 1780, 0.01);
        }

        if (s.v[1775] != 0.0) {
            s.store_scale(1784, 429, 1.0 / (s.v[81]));
        }

        if (s.v[1775] != 0.0) {
            s.store_powf(328, 1784, p.p269);
        }

        if (s.v[1775] != 0.0) {
            s.store_div(1787, 1779, 328);
        }

        if (s.v[1775] != 0.0) {
            s.store_sub_ad(327, A::add(A::offset(A::scale(s.ad_value(1784), 0.4), 1.8), A::mul(A::scale(s.ad_value(1784), 0.1), s.ad_value(1784))), A::scale(A::sub_from_scalar(1.0, s.ad_value(1784)), p.p270));
        }

        if (s.v[1775] != 0.0) {
            s.store_div(1788, 1780, 327);
        }

        if (s.v[1775] != 0.0) {
            s.store_add_ad_rhs(1781, 1781, A::scale(A::offset(s.ad_value(429), (-s.v[81])), p.p274));
        }

        if (s.v[1775] != 0.0) {
            s.store_scalar(1776, (1.0 + (p.p279 / ((s.v[100]) as f64).powf(p.p280))));
        }

        if (s.v[1775] != 0.0) {
            s.store_scalar(1778, (1.0 + (p.p277 / ((s.v[100]) as f64).powf(p.p278))));
        }

        if (s.v[1775] != 0.0) {
            s.store_scalar(1777, (1.0 + (p.p275 / ((s.v[109]) as f64).powf(p.p276))));
        }

        if (s.v[1775] != 0.0) {
            s.store_mul(1787, 1787, 1776);
        }

        if (s.v[1775] != 0.0) {
            s.store_offset_ad(1788, A::mul(A::mul(s.ad_value(1788), s.ad_value(1777)), s.ad_value(1778)), 1e-50);
        }

        if (s.v[1775] != 0.0) {
            s.store_div(1789, 1783, 1785);
        }

        if (s.v[1775] != 0.0) {
            s.store_mul(1790, 1787, 1789);
        }

        s.v[1796] = if (s.v[1783] >= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1775] != 0.0) && (s.v[1796] != 0.0)) {
            s.store_div(328, 1790, 1788);
        }

    }

    pub(super) fn stamp_reactive_block_30(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[1775] != 0.0) && (!(s.v[1796] != 0.0))) {
            s.store_div_ad_lhs(328, A::neg(s.ad_value(1790)), 1788);
        }

        s.v[1797] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1781]) && (s.v[1781] <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1775] != 0.0) && (s.v[1797] != 0.0)) {
            s.store_scalar(330, 1.0);
        }

        s.v[1798] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1781]) && (s.v[1781] <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1775] != 0.0) && (!(s.v[1797] != 0.0))) && (s.v[1798] != 0.0)) {
            s.copy_ad(330, 328);
        }

        if (((s.v[1775] != 0.0) && (!(s.v[1797] != 0.0))) && (!(s.v[1798] != 0.0))) {
            s.store_ad(330, &A::pow(s.ad_value(328), A::offset(s.ad_value(1781), (-1.0))));
        }

        if (s.v[1775] != 0.0) {
            s.store_mul(329, 328, 330);
        }

        if (s.v[1775] != 0.0) {
            s.store_offset(331, 329, 1.0);
        }

        s.v[1799] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1781]) && (s.v[1781] <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1775] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_div_from_scalar(332, 1.0, 331);
        }

        s.v[1800] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1781]) && (s.v[1781] <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1775] != 0.0) && (!(s.v[1799] != 0.0))) && (s.v[1800] != 0.0)) {
            s.store_div_from_scalar_ad(332, 1.0, A::sqrt(s.ad_value(331)));
        }

        if (((s.v[1775] != 0.0) && (!(s.v[1799] != 0.0))) && (!(s.v[1800] != 0.0))) {
            s.store_ad(333, &A::pow(s.ad_value(331), A::offset(A::div_from_scalar((-1.0), s.ad_value(1781)), (-1.0))));
        }

        if (((s.v[1775] != 0.0) && (!(s.v[1799] != 0.0))) && (!(s.v[1800] != 0.0))) {
            s.store_mul(332, 331, 333);
        }

        if (s.v[1775] != 0.0) {
            s.store_div_from_scalar(328, 1.6021918e-19, 1785);
        }

        s.v[1803] = if (p.p260 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1803] != 0.0) {
            s.store_scalar(3, 2.0);
        }

        s.v[1823] = if (s.v[3] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1803] != 0.0) && (s.v[1823] != 0.0)) {
            s.store_scalar(1807, p.p266);
        }

        if ((s.v[1803] != 0.0) && (s.v[1823] != 0.0)) {
            s.store_scalar(1808, p.p268);
        }

        if ((s.v[1803] != 0.0) && (s.v[1823] != 0.0)) {
            s.store_scalar(1809, p.p273);
        }

        if ((s.v[1803] != 0.0) && (s.v[1823] != 0.0)) {
            s.store_scalar(1813, p.p258);
        }

        if ((s.v[1803] != 0.0) && (s.v[1823] != 0.0)) {
            s.store_ad(1811, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(2)), p.p50));
        }

        if ((s.v[1803] != 0.0) && (!(s.v[1823] != 0.0))) {
            s.store_scalar(1807, p.p265);
        }

        if ((s.v[1803] != 0.0) && (!(s.v[1823] != 0.0))) {
            s.store_scalar(1808, p.p267);
        }

        if ((s.v[1803] != 0.0) && (!(s.v[1823] != 0.0))) {
            s.store_scalar(1809, p.p272);
        }

        if ((s.v[1803] != 0.0) && (!(s.v[1823] != 0.0))) {
            s.store_scalar(1813, p.p257);
        }

        if ((s.v[1803] != 0.0) && (!(s.v[1823] != 0.0))) {
            s.store_ad(1811, &A::scale(A::voltage(ctx, &nodes, Some(0), Some(6)), p.p50));
        }

        if (s.v[1803] != 0.0) {
            s.store_scale(1807, 1807, 0.0001);
        }

        if (s.v[1803] != 0.0) {
            s.store_scale(1808, 1808, 0.01);
        }

        if (s.v[1803] != 0.0) {
            s.store_scale(1812, 429, 1.0 / (s.v[81]));
        }

        if (s.v[1803] != 0.0) {
            s.store_powf(328, 1812, p.p269);
        }

        if (s.v[1803] != 0.0) {
            s.store_div(1815, 1807, 328);
        }

        if (s.v[1803] != 0.0) {
            s.store_sub_ad(327, A::add(A::offset(A::scale(s.ad_value(1812), 0.4), 1.8), A::mul(A::scale(s.ad_value(1812), 0.1), s.ad_value(1812))), A::scale(A::sub_from_scalar(1.0, s.ad_value(1812)), p.p270));
        }

        if (s.v[1803] != 0.0) {
            s.store_div(1816, 1808, 327);
        }

        if (s.v[1803] != 0.0) {
            s.store_add_ad_rhs(1809, 1809, A::scale(A::offset(s.ad_value(429), (-s.v[81])), p.p274));
        }

        if (s.v[1803] != 0.0) {
            s.store_scalar(1804, (1.0 + (p.p279 / ((s.v[100]) as f64).powf(p.p280))));
        }

        if (s.v[1803] != 0.0) {
            s.store_scalar(1806, (1.0 + (p.p277 / ((s.v[100]) as f64).powf(p.p278))));
        }

        if (s.v[1803] != 0.0) {
            s.store_scalar(1805, (1.0 + (p.p275 / ((s.v[109]) as f64).powf(p.p276))));
        }

        if (s.v[1803] != 0.0) {
            s.store_mul(1815, 1815, 1804);
        }

        if (s.v[1803] != 0.0) {
            s.store_offset_ad(1816, A::mul(A::mul(s.ad_value(1816), s.ad_value(1805)), s.ad_value(1806)), 1e-50);
        }

        if (s.v[1803] != 0.0) {
            s.store_div(1817, 1811, 1813);
        }

        if (s.v[1803] != 0.0) {
            s.store_mul(1818, 1815, 1817);
        }

        s.v[1824] = if (s.v[1811] >= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1803] != 0.0) && (s.v[1824] != 0.0)) {
            s.store_div(328, 1818, 1816);
        }

        if ((s.v[1803] != 0.0) && (!(s.v[1824] != 0.0))) {
            s.store_div_ad_lhs(328, A::neg(s.ad_value(1818)), 1816);
        }

        s.v[1825] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1809]) && (s.v[1809] <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1803] != 0.0) && (s.v[1825] != 0.0)) {
            s.store_scalar(330, 1.0);
        }

        s.v[1826] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1809]) && (s.v[1809] <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1803] != 0.0) && (!(s.v[1825] != 0.0))) && (s.v[1826] != 0.0)) {
            s.copy_ad(330, 328);
        }

        if (((s.v[1803] != 0.0) && (!(s.v[1825] != 0.0))) && (!(s.v[1826] != 0.0))) {
            s.store_ad(330, &A::pow(s.ad_value(328), A::offset(s.ad_value(1809), (-1.0))));
        }

        if (s.v[1803] != 0.0) {
            s.store_mul(329, 328, 330);
        }

        if (s.v[1803] != 0.0) {
            s.store_offset(331, 329, 1.0);
        }

        s.v[1827] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1809]) && (s.v[1809] <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1803] != 0.0) && (s.v[1827] != 0.0)) {
            s.store_div_from_scalar(332, 1.0, 331);
        }

        s.v[1828] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1809]) && (s.v[1809] <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1803] != 0.0) && (!(s.v[1827] != 0.0))) && (s.v[1828] != 0.0)) {
            s.store_div_from_scalar_ad(332, 1.0, A::sqrt(s.ad_value(331)));
        }

        if (((s.v[1803] != 0.0) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_ad(333, &A::pow(s.ad_value(331), A::offset(A::div_from_scalar((-1.0), s.ad_value(1809)), (-1.0))));
        }

        if (((s.v[1803] != 0.0) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_mul(332, 331, 333);
        }

        if (s.v[1803] != 0.0) {
            s.store_div_from_scalar(328, 1.6021918e-19, 1813);
        }

        s.v[1831] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1831] != 0.0) && (s.v[85] != 0.0)) {
            s.store_ad(438, &{
                if (s.v[613] == 1.0) {
                    s.ad_value(556)
                } else {
                    A::sub_from_scalar(1.0, s.ad_value(556))
                }
            });
        }

        if ((s.v[1831] != 0.0) && (s.v[85] != 0.0)) {
            s.store_add_ad_lhs(584, A::mul(s.ad_value(580), s.ad_value(438)), 473);
        }

        if ((s.v[1831] != 0.0) && (s.v[85] != 0.0)) {
            s.store_add_ad_lhs(585, A::mul(s.ad_value(580), A::sub_from_scalar(1.0, s.ad_value(438))), 473);
        }

        if ((s.v[1831] != 0.0) && (s.v[85] != 0.0)) {
            s.store_add_ad_lhs(586, A::sub(A::neg(s.ad_value(580)), s.ad_value(581)), 471);
        }

        if ((s.v[1831] != 0.0) && (!(s.v[85] != 0.0))) {
            s.store_scalar(584, 0.0);
        }

        if ((s.v[1831] != 0.0) && (!(s.v[85] != 0.0))) {
            s.store_scalar(585, 0.0);
        }

        if ((s.v[1831] != 0.0) && (!(s.v[85] != 0.0))) {
            s.store_scalar(586, 0.0);
        }

        if ((s.v[1831] != 0.0) && (!(s.v[85] != 0.0))) {
            s.store_scalar(581, 0.0);
        }

        if ((!(s.v[1831] != 0.0)) && (s.v[85] != 0.0)) {
            s.store_sub_ad_lhs(586, A::sub(A::neg(s.ad_value(584)), s.ad_value(585)), 581);
        }

        if ((!(s.v[1831] != 0.0)) && (!(s.v[85] != 0.0))) {
            s.store_scalar(584, 0.0);
        }

        if ((!(s.v[1831] != 0.0)) && (!(s.v[85] != 0.0))) {
            s.store_scalar(585, 0.0);
        }

        if ((!(s.v[1831] != 0.0)) && (!(s.v[85] != 0.0))) {
            s.store_scalar(586, 0.0);
        }

        if ((!(s.v[1831] != 0.0)) && (!(s.v[85] != 0.0))) {
            s.store_scalar(581, 0.0);
        }

        s.v[1836] = if (s.v[613] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1836] != 0.0) {
            s.copy_ad(199, 9);
        }

        if (s.v[1836] != 0.0) {
            s.copy_ad(263, 557);
        }

        if (s.v[1836] != 0.0) {
            s.store_add(594, 23, 586);
        }

        if (s.v[1836] != 0.0) {
            s.store_add(198, 24, 584);
        }

        if (s.v[1836] != 0.0) {
            s.store_neg_ad(554, A::add(A::add(s.ad_value(23), s.ad_value(24)), s.ad_value(25)));
        }

        if (s.v[1836] != 0.0) {
            s.store_add(196, 554, 581);
        }

        if (!(s.v[1836] != 0.0)) {
            s.store_neg(199, 9);
        }

        if (!(s.v[1836] != 0.0)) {
            s.store_scalar(263, 0.0);
        }

        if (!(s.v[1836] != 0.0)) {
            s.store_add(594, 23, 586);
        }

        if (!(s.v[1836] != 0.0)) {
            s.store_add(198, 25, 585);
        }

        if (!(s.v[1836] != 0.0)) {
            s.store_neg_ad(554, A::add(A::add(s.ad_value(23), s.ad_value(24)), s.ad_value(25)));
        }

        if (!(s.v[1836] != 0.0)) {
            s.store_add(196, 554, 581);
        }

        s.v[1837] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1837] != 0.0) {
            s.copy_ad(282, 35);
        }

        if (s.v[1837] != 0.0) {
            s.copy_ad(284, 560);
        }

        if (s.v[1837] != 0.0) {
            s.copy_ad(281, 36);
        }

        if (s.v[1837] != 0.0) {
            s.copy_ad(283, 561);
        }

        s.v[1838] = if ((p.p38 == 1.0) && (s.v[67] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1838] != 0.0) {
            s.copy_ad(563, 542);
        }

        if (!(s.v[1838] != 0.0)) {
            s.store_scalar(563, 0.0);
        }

        s.copy_ad(9, 199);

        s.store_scalar(27, A::ddx_projection(&s.ad_value(594), Some(6), None));

        s.store_scale(27, 27, p.p50);

        s.store_scalar(28, A::ddx_projection(&s.ad_value(594), Some(7), None));

        s.store_scale(28, 28, p.p50);

        s.v[1840] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1840] != 0.0) {
            s.store_scale(35, 282, p.p50);
        }

        if (s.v[1840] != 0.0) {
            s.store_scale(36, 281, p.p50);
        }

        s.store_scale(610, 429, (4.0 * 1.3806226e-23));

        s.copy_ad(438, 439);

        s.store_mul(615, 610, 598);

        if ((s.v[615] > 0.0) && (s.v[558] > 0.0)) {
            s.store_sqrt_ad(616, A::div(s.ad_value(558), s.ad_value(615)));
        } else {
            s.store_scalar(616, 0.0);
        }

        if (s.v[613] > 0.0) {
            s.store_mul_ad_rhs(617, 616, A::sub_from_scalar(1.0, s.ad_value(438)));
        } else {
            s.store_mul(617, 616, 438);
        }

        if (s.v[613] > 0.0) {
            s.store_mul(618, 616, 438);
        } else {
            s.store_mul_ad_rhs(618, 616, A::sub_from_scalar(1.0, s.ad_value(438)));
        }

        s.v[1848] = if ((p.p38 > 0.0) && (p.p242 > 0.0)) { 1.0 } else { 0.0 };

        s.v[1849] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        s.v[1850] = if ((p.p37 != 0.0) || ((p.p25 == 1.0) && (p.p26 == 2.0))) { 1.0 } else { 0.0 };

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
        let eq0_value: f64 = 0.0;
        stamper.stamp_potential(
            branches[0],
            eq0_value,
            &[
            ],
        );
    }
}
