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
        if ((((!(s.v[733] != 0.0)) && (s.v[1085] != 0.0)) && (s.v[1124] != 0.0)) && (!(s.v[1125] != 0.0))) {
            s.copy_ad(436, 425);
        }

        if (((!(s.v[733] != 0.0)) && (s.v[1085] != 0.0)) && (!(s.v[1124] != 0.0))) {
            s.store_scaled_add(436, 355, 358, (-0.5));
        }

        s.v[1129] = if (s.v[612] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1129] != 0.0) {
            s.store_offset(480, 190, 0.5);
        }

        if (s.v[1129] != 0.0) {
            s.store_mul(481, 479, 478);
        }

        if (s.v[1129] != 0.0) {
            s.store_div_ad_lhs(482, A::scale(s.ad_value(480), 0.4), 481);
        }

        if (s.v[1129] != 0.0) {
            s.store_sub_from_scalar(438, 0.6, 482);
        }

        s.v[1130] = if (s.v[438] > (0.5 + 1e-8)) { 1.0 } else { 0.0 };

        if ((s.v[1129] != 0.0) && (s.v[1130] != 0.0)) {
            s.store_scalar(438, 0.5);
        }

        if (s.v[1129] != 0.0) {
            s.copy_ad(439, 438);
        }

        if (s.v[1129] != 0.0) {
            s.store_scalar(438, 0.5);
        }

        s.v[1132] = if (s.v[145] == 0.0) { 1.0 } else { 0.0 };

        s.v[1148] = if ((p.p190 < (10.0 * 2.220446049250313e-16)) && (p.p191 < (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if (((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (s.v[1148] != 0.0)) {
            s.store_scalar(316, 0.0);
        }

        if (((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (s.v[1148] != 0.0)) {
            s.copy_ad(314, 162);
        }

        s.v[1149] = if (s.v[314] > ((s.v[161] + s.v[173]) - (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if ((((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (s.v[1148] != 0.0)) && (s.v[1149] != 0.0)) {
            s.store_offset_ad(314, A::add(s.ad_value(161), s.ad_value(173)), (-(10.0 * 2.220446049250313e-16)));
        }

        if (((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (!(s.v[1148] != 0.0))) {
            s.store_scalar(1147, (if (p.p43 == 1.0) { p.p237 } else { s.v[402] }));
        }

        if (((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (!(s.v[1148] != 0.0))) {
            s.store_div_from_scalar(1133, 1.0, 1147);
        }

        if (((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (!(s.v[1148] != 0.0))) {
            s.store_mul(1134, 244, 1133);
        }

        if (((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (!(s.v[1148] != 0.0))) {
            s.store_scale(1135, 1134, p.p191);
        }

        if (((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (!(s.v[1148] != 0.0))) {
            s.store_add_ad_lhs(1138, A::mul(s.ad_value(80), s.ad_value(229)), 1135);
        }

        if (((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (!(s.v[1148] != 0.0))) {
            s.store_div_from_scalar(1134, 1.0, 1138);
        }

        if (((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (!(s.v[1148] != 0.0))) {
            s.store_scale(1137, 1134, 1.034943e-10);
        }

        if (((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (!(s.v[1148] != 0.0))) {
            s.store_scalar(1134, (1.0 - p.p189));
        }

        if (((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (!(s.v[1148] != 0.0))) {
            s.store_add_ad(314, A::scale(A::add(s.ad_value(157), s.ad_value(161)), p.p189), A::mul(s.ad_value(1134), s.ad_value(162)));
        }

        s.v[1150] = if (s.v[314] > ((s.v[161] + s.v[173]) - (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if ((((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (!(s.v[1148] != 0.0))) && (s.v[1150] != 0.0)) {
            s.store_offset_ad(314, A::add(s.ad_value(161), s.ad_value(173)), (-(10.0 * 2.220446049250313e-16)));
        }

        if (((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (!(s.v[1148] != 0.0))) {
            s.store_sub(1140, 314, 162);
        }

        if (((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (!(s.v[1148] != 0.0))) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1140)), ((4.0 * 0.001) * 0.001)));
        }

        if (((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (!(s.v[1148] != 0.0))) {
            s.store_offset_ad(1139, A::scale(A::add(s.ad_value(1140), s.ad_value(44)), 0.5), (1e-10 * 0.001));
        }

        s.v[1151] = if (s.v[1139] < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (!(s.v[1148] != 0.0))) && (s.v[1151] != 0.0)) {
            s.store_scalar(1139, 0.0);
        }

        if (((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (!(s.v[1148] != 0.0))) {
            s.store_mul(1136, 225, 244);
        }

        if (((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (!(s.v[1148] != 0.0))) {
            s.store_div_from_scalar(1134, 1.0, 1136);
        }

        if (((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (!(s.v[1148] != 0.0))) {
            s.store_mul(1138, 246, 1134);
        }

        s.v[1152] = if (s.v[1138] < s.v[227]) { 1.0 } else { 0.0 };

        if ((((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (!(s.v[1148] != 0.0))) && (s.v[1152] != 0.0)) {
            s.copy_ad(1138, 227);
        }

        if (((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (!(s.v[1148] != 0.0))) {
            s.store_scale(1144, 229, 9662367879.197212);
        }

        if (((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (!(s.v[1148] != 0.0))) {
            s.store_scalar(1134, (100000.0 * 10000.0));
        }

        if (((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (!(s.v[1148] != 0.0))) {
            s.store_scalar(1135, (1.0 / s.v[97]));
        }

        if (((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (!(s.v[1148] != 0.0))) {
            s.store_mul_ad_lhs(1146, A::add(A::add(A::scale(s.ad_value(1138), 2.0), A::mul(A::mul(A::scale(s.ad_value(1144), 2.0), s.ad_value(1139)), s.ad_value(1137))), A::mul(s.ad_value(1134), s.ad_value(1137))), 1135);
        }

        if (((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (!(s.v[1148] != 0.0))) {
            s.store_mul(1141, 1146, 1137);
        }

        if (((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (!(s.v[1148] != 0.0))) {
            s.store_scale_ad(1145, A::add(A::mul(A::scale(s.ad_value(1144), 2.0), s.ad_value(1139)), s.ad_value(1134)), 4.0);
        }

        if (((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (!(s.v[1148] != 0.0))) {
            s.store_mul_ad_lhs(1142, A::mul(s.ad_value(1145), s.ad_value(1137)), 1137);
        }

        if (((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (!(s.v[1148] != 0.0))) {
            s.store_sqrt_ad(1143, A::add(A::square(s.ad_value(1141)), s.ad_value(1142)));
        }

        if (((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) && (!(s.v[1148] != 0.0))) {
            s.store_mul_ad_rhs(316, 326, A::scale(A::sub(s.ad_value(1143), s.ad_value(1141)), 0.5));
        }

        if ((s.v[1129] != 0.0) && (s.v[1132] != 0.0)) {
            s.store_scale(316, 316, s.v[127]);
        }

        if (s.v[1129] != 0.0) {
            s.store_sub_from_scalar(441, s.v[97], 316);
        }

        s.v[1153] = if (s.v[441] < 1e-9) { 1.0 } else { 0.0 };

        if ((s.v[1129] != 0.0) && (s.v[1153] != 0.0)) {
            s.store_scalar(441, 1e-9);
        }

        if (s.v[1129] != 0.0) {
            s.store_scale_ad(328, A::neg(s.ad_value(108)), s.v[98]);
        }

        if (s.v[1129] != 0.0) {
            s.store_mul(196, 328, 437);
        }

        if (s.v[1129] != 0.0) {
            s.store_mul(197, 328, 436);
        }

        if (s.v[1129] != 0.0) {
            s.store_mul(198, 197, 438);
        }

        s.v[1154] = if (p.p43 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1129] != 0.0) && (s.v[1154] != 0.0)) {
            s.store_scale(477, 196, 0.5);
        }

        if ((s.v[1129] != 0.0) && (s.v[1154] != 0.0)) {
            s.store_scale(476, 196, (1.0 - 0.5));
        }

        if ((s.v[1129] != 0.0) && (s.v[1154] != 0.0)) {
            s.store_mul_ad_lhs(392, A::scale(A::add(s.ad_value(357), s.ad_value(360)), (0.5 * s.v[98])), 108);
        }

        if (s.v[1129] != 0.0) {
            s.store_scaled_sub(1155, 157, 164, 0.5);
        }

        if (s.v[1129] != 0.0) {
            s.store_scale(44, 1155, (2.0 * 1.0 / (p.p227)));
        }

        if (s.v[1129] != 0.0) {
            s.store_offset_ad(45, A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::scale(s.ad_value(44), (1.0 / 5040.0)), (1.0 / 720.0))), (1.0 / 120.0))), (1.0 / 24.0))), (1.0 / 6.0))), (1.0 / 2.0))), 1.0);
        }

        if (s.v[1129] != 0.0) {
            s.store_div_from_scalar(177, p.p227, 45);
        }

        s.v[1156] = if (s.v[177] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if ((s.v[1129] != 0.0) && (s.v[1156] != 0.0)) {
            s.store_scalar(177, (10.0 * 2.220446049250313e-16));
        }

        if (s.v[1129] != 0.0) {
            s.store_add(176, 161, 177);
        }

        if (s.v[1129] != 0.0) {
            s.store_scalar(1166, (1.034943e-10 / 100.0));
        }

        if (s.v[1129] != 0.0) {
            s.store_scale(1167, 437, 0.0001);
        }

        if (s.v[1129] != 0.0) {
            s.store_scale(1168, 436, 0.0001);
        }

        if (s.v[1129] != 0.0) {
            s.store_div_from_scalar(1157, p.p92, 1166);
        }

        if (s.v[1129] != 0.0) {
            s.store_div_from_scalar(1158, p.p93, 1166);
        }

        if (s.v[1129] != 0.0) {
            s.store_scalar(1159, p.p94);
        }

        if (s.v[1129] != 0.0) {
            s.store_offset_ad(1160, A::mul(A::sub(s.ad_value(162), s.ad_value(161)), s.ad_value(1159)), 1.0);
        }

        if (s.v[1129] != 0.0) {
            s.store_add_ad(1161, A::mul(s.ad_value(1157), s.ad_value(1167)), A::mul(s.ad_value(1158), s.ad_value(1168)));
        }

        if (s.v[1129] != 0.0) {
            s.store_div(1162, 1161, 1160);
        }

        if (s.v[1129] != 0.0) {
            s.copy_ad(248, 1162);
        }

        if (s.v[1129] != 0.0) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(248)), ((4.0 * 3000.0) * 3000.0)));
        }

        if (s.v[1129] != 0.0) {
            s.store_offset_ad(1159, A::scale(A::add(s.ad_value(248), s.ad_value(44)), 0.5), (1e-10 * 3000.0));
        }

        s.v[1169] = if (s.v[1159] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1129] != 0.0) && (s.v[1169] != 0.0)) {
            s.store_scalar(1159, 0.0);
        }

        if (s.v[1129] != 0.0) {
            s.store_powf(1161, 1159, (p.p97 - 1.0));
        }

        if (s.v[1129] != 0.0) {
            s.store_mul(1163, 1161, 1159);
        }

        if (s.v[1129] != 0.0) {
            s.store_powf(1164, 1159, (s.v[111] - 1.0));
        }

        if (s.v[1129] != 0.0) {
            s.store_mul(1165, 1164, 1159);
        }

        if (s.v[1129] != 0.0) {
            s.store_scale(249, 1168, 6.241449993689894e18);
        }

        if (s.v[1129] != 0.0) {
            s.store_add_ad(1157, A::add(A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(249), (p.p96 * 1e-11)), p.p95)), A::mul(s.ad_value(543), s.ad_value(1163))), A::scale(s.ad_value(1165), 1.0 / (p.p106)));
        }

        if (s.v[1129] != 0.0) {
            s.store_div_from_scalar(251, 1.0, 1157);
        }

        if (s.v[1129] != 0.0) {
            s.store_scale(251, 251, 0.0001);
        }

        if (s.v[1129] != 0.0) {
            s.store_mul_ad_lhs(1170, A::mul(s.ad_value(225), s.ad_value(244)), 441);
        }

        if (s.v[1129] != 0.0) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1170)), ((4.0 * 1e-50) * 1e-50)));
        }

        if (s.v[1129] != 0.0) {
            s.store_offset_ad(1170, A::scale(A::add(s.ad_value(1170), s.ad_value(44)), 0.5), (1e-10 * 1e-50));
        }

        s.v[1178] = if (s.v[1170] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1129] != 0.0) && (s.v[1178] != 0.0)) {
            s.store_scalar(1170, 0.0);
        }

        if (s.v[1129] != 0.0) {
            s.store_div_from_scalar(1171, 1.0, 1170);
        }

        if (s.v[1129] != 0.0) {
            s.store_mul(1172, 246, 1171);
        }

        if (s.v[1129] != 0.0) {
            s.store_div_ad_lhs(1170, A::scale(s.ad_value(253), 0.2), 251);
        }

        if (s.v[1129] != 0.0) {
            s.store_sqrt_ad(252, A::add(A::square(s.ad_value(1172)), A::square(s.ad_value(1170))));
        }

        if (s.v[1129] != 0.0) {
            s.store_mul(1173, 251, 252);
        }

        if (s.v[1129] != 0.0) {
            s.store_div(1171, 1173, 253);
        }

        s.v[1179] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1129] != 0.0) && (s.v[1179] != 0.0)) {
            s.store_scalar(1174, 1.0);
        }

        s.v[1180] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1129] != 0.0) && (!(s.v[1179] != 0.0))) && (s.v[1180] != 0.0)) {
            s.copy_ad(1174, 1171);
        }

        if (((s.v[1129] != 0.0) && (!(s.v[1179] != 0.0))) && (!(s.v[1180] != 0.0))) {
            s.store_powf(1174, 1171, (p.p113 - 1.0));
        }

        if (s.v[1129] != 0.0) {
            s.store_mul(1170, 1171, 1174);
        }

        if (s.v[1129] != 0.0) {
            s.store_offset(1175, 1170, 1.0);
        }

        s.v[1181] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1129] != 0.0) && (s.v[1181] != 0.0)) {
            s.store_div_from_scalar(1176, 1.0, 1175);
        }

        s.v[1182] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1129] != 0.0) && (!(s.v[1181] != 0.0))) && (s.v[1182] != 0.0)) {
            s.store_div_from_scalar_ad(1176, 1.0, A::sqrt(s.ad_value(1175)));
        }

        if (((s.v[1129] != 0.0) && (!(s.v[1181] != 0.0))) && (!(s.v[1182] != 0.0))) {
            s.store_powf(1177, 1175, (((-1.0) / p.p113) - 1.0));
        }

        if (((s.v[1129] != 0.0) && (!(s.v[1181] != 0.0))) && (!(s.v[1182] != 0.0))) {
            s.store_mul(1176, 1175, 1177);
        }

        if (s.v[1129] != 0.0) {
            s.store_mul(250, 251, 1176);
        }

        if (s.v[1129] != 0.0) {
            s.store_div_ad(264, A::mul(s.ad_value(107), s.ad_value(227)), A::sub_from_scalar(s.v[97], s.ad_value(316)));
        }

        if (s.v[1129] != 0.0) {
            s.store_mul_ad_lhs(200, A::mul(s.ad_value(264), s.ad_value(246)), 250);
        }

        if (s.v[1129] != 0.0) {
            s.store_scalar(201, 0.0);
        }

        s.v[1192] = if ((p.p281 > 0.0) && (p.p244 != 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1129] != 0.0) && (s.v[1192] != 0.0)) {
            s.store_scaled_sub(1183, 157, 164, 0.5);
        }

        if ((s.v[1129] != 0.0) && (s.v[1192] != 0.0)) {
            s.store_scale(44, 1183, (2.0 * 100.0));
        }

        if ((s.v[1129] != 0.0) && (s.v[1192] != 0.0)) {
            s.store_offset_ad(45, A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::scale(s.ad_value(44), (1.0 / 5040.0)), (1.0 / 720.0))), (1.0 / 120.0))), (1.0 / 24.0))), (1.0 / 6.0))), (1.0 / 2.0))), 1.0);
        }

        if ((s.v[1129] != 0.0) && (s.v[1192] != 0.0)) {
            s.store_div_from_scalar(1189, 0.01, 45);
        }

        if ((s.v[1129] != 0.0) && (s.v[1192] != 0.0)) {
            s.store_sub_from_scalar_ad(1183, 1.1, A::add(s.ad_value(161), s.ad_value(1189)));
        }

        if ((s.v[1129] != 0.0) && (s.v[1192] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1183)), ((4.0 * 0.05) * 0.05)));
        }

        if ((s.v[1129] != 0.0) && (s.v[1192] != 0.0)) {
            s.store_offset_ad(1191, A::scale(A::add(s.ad_value(1183), s.ad_value(44)), 0.5), (1e-10 * 0.05));
        }

        s.v[1193] = if (s.v[1191] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1129] != 0.0) && (s.v[1192] != 0.0)) && (s.v[1193] != 0.0)) {
            s.store_scalar(1191, 0.0);
        }

        if ((s.v[1129] != 0.0) && (s.v[1192] != 0.0)) {
            s.store_scale(1184, 225, s.v[116]);
        }

        if ((s.v[1129] != 0.0) && (s.v[1192] != 0.0)) {
            s.store_mul(1185, 323, 1184);
        }

        if ((s.v[1129] != 0.0) && (s.v[1192] != 0.0)) {
            s.store_powf(1184, 1191, p.p245);
        }

        if ((s.v[1129] != 0.0) && (s.v[1192] != 0.0)) {
            s.store_mul(1186, 1185, 1184);
        }

        if ((s.v[1129] != 0.0) && (s.v[1192] != 0.0)) {
            s.store_offset_scaled(1187, 173, p.p246, 1.0);
        }

        if ((s.v[1129] != 0.0) && (s.v[1192] != 0.0)) {
            s.store_scalar(1184, s.v[117]);
        }

        s.v[1194] = if ((s.v[56] < 3.0) || (p.p43 == 1.0)) { 1.0 } else { 0.0 };

        if (((s.v[1129] != 0.0) && (s.v[1192] != 0.0)) && (s.v[1194] != 0.0)) {
            s.store_sub_ad_lhs(1188, A::add(s.ad_value(161), s.ad_value(1189)), 172);
        }

        if (((s.v[1129] != 0.0) && (s.v[1192] != 0.0)) && (!(s.v[1194] != 0.0))) {
            s.store_sub_ad_lhs(1188, A::add(s.ad_value(161), s.ad_value(1189)), 350);
        }

        if ((s.v[1129] != 0.0) && (s.v[1192] != 0.0)) {
            s.store_add_ad_rhs(1187, 1187, A::mul(A::mul(s.ad_value(173), s.ad_value(1184)), s.ad_value(1188)));
        }

        if ((s.v[1129] != 0.0) && (s.v[1192] != 0.0)) {
            s.store_mul(1189, 1186, 1187);
        }

        if ((s.v[1129] != 0.0) && (s.v[1192] != 0.0)) {
            s.copy_ad(1186, 1189);
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
        if ((s.v[1129] != 0.0) && (!(s.v[1192] != 0.0))) {
            s.store_scalar(1186, 0.0);
        }

        s.v[1195] = if (p.p248 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1129] != 0.0) && (s.v[1195] != 0.0)) {
            s.store_scale(1183, 225, s.v[118]);
        }

        if ((s.v[1129] != 0.0) && (s.v[1195] != 0.0)) {
            s.store_mul(1191, 323, 1183);
        }

        if ((s.v[1129] != 0.0) && (s.v[1195] != 0.0)) {
            s.store_mul(1190, 1191, 173);
        }

        if ((s.v[1129] != 0.0) && (!(s.v[1195] != 0.0))) {
            s.store_scalar(1190, 0.0);
        }

        s.v[1196] = if ((s.v[1186] + s.v[1190]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1129] != 0.0) && (s.v[1196] != 0.0)) {
            s.store_mul_ad_rhs(247, 164, A::add(s.ad_value(1186), s.ad_value(1190)));
        }

        if ((s.v[1129] != 0.0) && (s.v[1196] != 0.0)) {
            s.store_mul_ad_lhs(201, A::mul(s.ad_value(264), s.ad_value(247)), 250);
        }

        if (s.v[1129] != 0.0) {
            s.store_add(199, 200, 201);
        }

        if (s.v[1129] != 0.0) {
            s.copy_ad(203, 201);
        }

        s.v[1206] = if (p.p33 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.copy_ad(1199, 549);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_scalar(1200, (s.v[124] - p.p71));
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_div_from_scalar_ad(1201, 1.0, A::square(s.ad_value(1200)));
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_mul_ad_lhs(1202, A::mul(A::mul(A::scale(A::sub_from_scalar(p.p69, s.ad_value(233)), 2.0), A::scale(s.ad_value(324), 1.034943e-10)), s.ad_value(1199)), 1201);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_mul(186, 1202, 235);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_offset_scaled(1198, 173, p.p155, p.p154);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_mul(206, 186, 1198);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_sub_from_scalar_ad(1197, p.p156, A::scale(s.ad_value(157), p.p157));
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_add_ad_lhs(207, A::add(A::offset(s.ad_value(174), (-s.v[123])), s.ad_value(1197)), 206);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_mul_ad_lhs(210, A::mul(s.ad_value(205), s.ad_value(324)), 324);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_scaled_mul(211, 210, 225, 0.5);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_scaled_mul(212, 211, 225, 2.0);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_offset_ad(1203, A::sub(A::offset(A::offset(A::sub(s.ad_value(227), A::mul(s.ad_value(210), A::scale(s.ad_value(225), 0.25))), s.v[123]), (-p.p156)), s.ad_value(206)), 1e-50);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_offset_ad(1197, A::sub(s.ad_value(174), s.ad_value(1203)), (-0.005));
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_scalar(327, (if (s.v[1203] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_sqrt_ad(1199, A::add(A::square(s.ad_value(1197)), A::scale(A::mul(A::scale(s.ad_value(327), 4.0), s.ad_value(1203)), 0.005)));
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_sub_ad_lhs(1200, A::add(A::offset(A::offset(A::add(s.ad_value(1203), A::scale(A::add(s.ad_value(1197), s.ad_value(1199)), 0.5)), (-s.v[123])), p.p156), s.ad_value(206)), 514);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_offset_ad(1201, A::mul(s.ad_value(225), s.ad_value(1200)), (-1.0));
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_div_from_scalar(1202, 4.0, 212);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_offset_ad(1198, A::mul(s.ad_value(1201), s.ad_value(1202)), 1.0);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1198)), ((4.0 * 0.01) * 0.01)));
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_offset_ad(1197, A::scale(A::add(s.ad_value(1198), s.ad_value(44)), 0.5), (1e-10 * 0.01));
        }

        s.v[1207] = if (s.v[1197] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) {
            s.store_scalar(1197, 0.0);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_sqrt_ad(213, A::offset(s.ad_value(1197), 1e-50));
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_add_ad_rhs(215, 207, A::mul(s.ad_value(211), A::sub_from_scalar(1.0, s.ad_value(213))));
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_div_from_scalar_ad(327, 1.0, A::add(s.ad_value(225), A::div_from_scalar(2.0, A::offset(s.ad_value(207), 1e-50))));
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_mul_ad_lhs(216, A::ln(A::mul(A::div(A::div_from_scalar(1.0, s.ad_value(209)), s.ad_value(210)), A::square(s.ad_value(207)))), 327);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_div_ad_rhs(1200, 216, A::offset(s.ad_value(207), 1e-50));
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_offset_ad(217, A::sub(s.ad_value(216), s.ad_value(215)), (-0.002));
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_sqrt_ad(327, A::add(A::square(s.ad_value(217)), A::scale(s.ad_value(216), (4.0 * 0.002))));
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_sub_ad_rhs(218, 216, A::scale(A::add(s.ad_value(217), s.ad_value(327)), 0.5));
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_div_from_scalar(1197, 1.0, 327);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_mul_ad_rhs(327, 209, A::exp(A::mul(s.ad_value(225), s.ad_value(218))));
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_add_ad_lhs(1198, A::offset(A::mul(s.ad_value(225), A::sub(s.ad_value(218), s.ad_value(514))), (-1.0)), 327);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1198)), ((4.0 * 0.01) * 0.01)));
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_offset_ad(1197, A::scale(A::add(s.ad_value(1198), s.ad_value(44)), 0.5), (1e-10 * 0.01));
        }

        s.v[1208] = if (s.v[1197] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) && (s.v[1208] != 0.0)) {
            s.store_scalar(1197, 0.0);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_sqrt_ad(219, A::offset(s.ad_value(1197), (10.0 * 2.220446049250313e-16)));
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_offset_ad(1198, A::mul(s.ad_value(225), A::sub(s.ad_value(218), s.ad_value(514))), (-1.0));
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1198)), ((4.0 * 0.01) * 0.01)));
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_offset_ad(1197, A::scale(A::add(s.ad_value(1198), s.ad_value(44)), 0.5), (1e-10 * 0.01));
        }

        s.v[1209] = if (s.v[1197] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) && (s.v[1209] != 0.0)) {
            s.store_scalar(1197, 0.0);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_sqrt_ad(220, A::offset(s.ad_value(1197), (10.0 * 2.220446049250313e-16)));
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_mul_ad_rhs(221, 208, A::sub(s.ad_value(219), s.ad_value(220)));
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_sub(1198, 215, 218);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1198)), ((4.0 * 0.1) * 0.1)));
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_offset_ad(1197, A::scale(A::add(s.ad_value(1198), s.ad_value(44)), 0.5), (1e-10 * 0.1));
        }

        s.v[1210] = if (s.v[1197] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) && (s.v[1210] != 0.0)) {
            s.store_scalar(1197, 0.0);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_div_ad_rhs(1204, 157, A::offset(s.ad_value(1197), (10.0 * 2.220446049250313e-16)));
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_square(49, 1204);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_scalar(50, 1.0);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_scalar(51, 1.0);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_scalar(52, 1.0);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_scalar(55, 0.0);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_scalar(48, 0.0);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_scalar(53, 0.0);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_add(48, 51, 52);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.copy_ad(53, 48);
        }

        s.v[1211] = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1212] = if (4.0 == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) && (s.v[1211] != 0.0)) && (s.v[1212] != 0.0)) {
            s.store_scalar(55, 1.0);
        }

        s.v[1213] = if (4.0 == 2.0) { 1.0 } else { 0.0 };

        if (((((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) && (s.v[1211] != 0.0)) && (!(s.v[1212] != 0.0))) && (s.v[1213] != 0.0)) {
            s.store_scalar(55, 2.0);
        }

        s.v[1214] = if (4.0 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) && (s.v[1211] != 0.0)) && (!(s.v[1212] != 0.0))) && (!(s.v[1213] != 0.0))) && (s.v[1214] != 0.0)) {
            s.store_scalar(55, 3.0);
        }

        s.v[1215] = if (4.0 == 8.0) { 1.0 } else { 0.0 };

        if (((((((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) && (s.v[1211] != 0.0)) && (!(s.v[1212] != 0.0))) && (!(s.v[1213] != 0.0))) && (!(s.v[1214] != 0.0))) && (s.v[1215] != 0.0)) {
            s.store_scalar(55, 4.0);
        }

        if (((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) && (s.v[1211] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        let mut assign19450_loop_guard: usize = 0;
        while {
            let assign19450_cond_e26957: f64 = if ((((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) && (s.v[1211] != 0.0)) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign19450_cond_e26957 != 0.0
        } {
            assign19450_loop_guard += 1;
            assert!(assign19450_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) && (s.v[1211] != 0.0)) {
                s.store_sqrt(53, 53);
            }
            if (((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) && (s.v[1211] != 0.0)) {
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) && (!(s.v[1211] != 0.0))) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_div_from_scalar(53, 1.0, 53);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_mul(1205, 1204, 53);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_scale(214, 227, ((2.0 * s.v[126]) * p.p9));
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_div_ad_lhs(222, A::mul(A::mul(A::mul(s.ad_value(214), s.ad_value(250)), s.ad_value(221)), s.ad_value(1205)), 441);
        }

        if ((s.v[1129] != 0.0) && (s.v[1206] != 0.0)) {
            s.store_add(199, 199, 222);
        }

        s.v[1216] = if ((p.p30 != 0.0) && (p.p32 != 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1129] != 0.0) && (s.v[1216] != 0.0)) {
            s.store_square(294, 192);
        }

        if ((s.v[1129] != 0.0) && (s.v[1216] != 0.0)) {
            s.store_mul_ad_lhs(295, A::mul(A::scale(s.ad_value(227), 2.0), s.ad_value(324)), 246);
        }

        if ((s.v[1129] != 0.0) && (s.v[1216] != 0.0)) {
            s.store_sub(296, 294, 295);
        }

        if ((s.v[1129] != 0.0) && (s.v[1216] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(294)), ((4.0 * 0.001) * 0.001)));
        }

        if ((s.v[1129] != 0.0) && (s.v[1216] != 0.0)) {
            s.store_offset_ad(294, A::scale(A::add(s.ad_value(294), s.ad_value(44)), 0.5), (1e-10 * 0.001));
        }

        s.v[1217] = if (s.v[294] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1129] != 0.0) && (s.v[1216] != 0.0)) && (s.v[1217] != 0.0)) {
            s.store_scalar(294, 0.0);
        }

        if ((s.v[1129] != 0.0) && (s.v[1216] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(296)), ((4.0 * 0.001) * 0.001)));
        }

        if ((s.v[1129] != 0.0) && (s.v[1216] != 0.0)) {
            s.store_offset_ad(296, A::scale(A::add(s.ad_value(296), s.ad_value(44)), 0.5), (1e-10 * 0.001));
        }

        s.v[1218] = if (s.v[296] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1129] != 0.0) && (s.v[1216] != 0.0)) && (s.v[1218] != 0.0)) {
            s.store_scalar(296, 0.0);
        }

        if ((s.v[1129] != 0.0) && (s.v[1216] != 0.0)) {
            s.store_sub(297, 294, 296);
        }

        s.v[1219] = if ((s.v[244] < (10.0 * 2.220446049250313e-16)) || (s.v[297] < (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if (((s.v[1129] != 0.0) && (s.v[1216] != 0.0)) && (s.v[1219] != 0.0)) {
            s.store_scalar(146, 0.0);
        }

        if (((s.v[1129] != 0.0) && (s.v[1216] != 0.0)) && (!(s.v[1219] != 0.0))) {
            s.store_scalar(146, 1.0);
        }

        s.copy_ad(202, 199);

        s.v[204] = 0.0;

        s.v[1220] = if ((p.p281 > 0.0) && (p.p285 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1220] != 0.0) {
            s.store_scalar(1227, s.v[99]);
        }

        if (s.v[1220] != 0.0) {
            s.store_scalar(1231, p.p237);
        }

        if (s.v[1220] != 0.0) {
            s.store_offset_ad(1232, A::sub(A::add(A::offset(s.ad_value(158), (-s.v[123])), s.ad_value(185)), s.ad_value(320)), (-p.p286));
        }

        if (s.v[1220] != 0.0) {
            s.store_offset(1233, 182, p.p286);
        }

        if (s.v[1220] != 0.0) {
            s.store_scalar(1235, p.p285);
        }

        if (s.v[1220] != 0.0) {
            s.store_scalar(1234, p.p283);
        }

        if (s.v[1220] != 0.0) {
            s.store_scalar(1225, s.v[70]);
        }

        if (s.v[1220] != 0.0) {
            s.store_mul_ad_rhs(1226, 227, A::ln(A::div(A::mul(A::div(s.ad_value(1225), s.ad_value(230)), s.ad_value(536)), s.ad_value(230))));
        }

        if (s.v[1220] != 0.0) {
            s.store_ad(1223, &{
                if (p.p43 == 1.0) {
                    s.ad_value(435)
                } else {
                    s.ad_value(350)
                }
            });
        }

        if (s.v[1220] != 0.0) {
            s.store_sqrt_ad(1228, A::div(A::mul(A::mul(A::scale(A::sub(s.ad_value(1226), s.ad_value(1223)), ((2.0 * 1.6021918e-19) * 9662367879.197212)), s.ad_value(536)), s.ad_value(1225)), A::add(s.ad_value(536), s.ad_value(1225))));
        }

        if (s.v[1220] != 0.0) {
            s.store_mul(1222, 1228, 1227);
        }

        if (s.v[1220] != 0.0) {
            s.store_div_ad(1221, A::mul(A::scale(s.ad_value(1222), (-0.25)), s.ad_value(1222)), A::add(s.ad_value(157), s.ad_value(1222)));
        }

        if (s.v[1220] != 0.0) {
            s.copy_ad(1247, 1221);
        }

        if (s.v[1220] != 0.0) {
            s.copy_ad(1248, 1233);
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
        if (s.v[1220] != 0.0) {
            s.store_offset_ad(336, A::div(A::scale(A::offset(A::mul(s.ad_value(225), A::sub(s.ad_value(1232), s.ad_value(1247))), (-1.0)), 4.0), A::mul(s.ad_value(241), s.ad_value(226))), 1.0);
        }

        if (s.v[1220] != 0.0) {
            s.store_ad(336, &{
                if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(336)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if (s.v[1220] != 0.0) {
            s.store_add_ad_rhs(376, 1232, A::mul(A::scale(A::mul(s.ad_value(241), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336)))));
        }

        s.v[1249] = if (s.v[158] < ((s.v[123] + s.v[1248]) * 0.5)) { 1.0 } else { 0.0 };

        if ((s.v[1220] != 0.0) && (s.v[1249] != 0.0)) {
            s.store_scalar(144, 0.0);
        }

        s.v[1250] = if ((s.v[144] == 0.0) || (1.0 != 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) {
            s.store_mul_ad_rhs(181, 225, A::sub(s.ad_value(376), s.ad_value(1247)));
        }

        s.v[1251] = if (s.v[181] < 3.0) { 1.0 } else { 0.0 };

        if (((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1251] != 0.0)) {
            s.store_mul_ad_rhs(337, 225, A::sub(s.ad_value(1232), s.ad_value(1247)));
        }

        if (((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1251] != 0.0)) {
            s.store_div_from_scalar_ad(328, 1.0, A::mul(A::scale(s.ad_value(225), (1.414213562373095 / 108.0)), s.ad_value(240)));
        }

        if (((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1251] != 0.0)) {
            s.store_offset_scaled(329, 328, 3.0, 81.0);
        }

        if (((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1251] != 0.0)) {
            s.store_add_ad(330, A::sub_from_scalar((-2916.0), A::scale(s.ad_value(328), 81.0)), A::mul(A::scale(s.ad_value(328), 27.0), s.ad_value(337)));
        }

        if (((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1251] != 0.0)) {
            s.store_add_ad(331, A::sub_from_scalar(1458.0, A::scale(A::offset(s.ad_value(328), 54.0), 81.0)), A::mul(A::scale(s.ad_value(328), 27.0), s.ad_value(337)));
        }

        if (((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1251] != 0.0)) {
            s.store_square(331, 331);
        }

        if (((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1251] != 0.0)) {
            s.store_powf_ad(332, A::add(s.ad_value(330), A::sqrt(A::add(A::mul(A::mul(A::scale(s.ad_value(329), 4.0), s.ad_value(329)), s.ad_value(329)), s.ad_value(331)))), 0.3333333333333333);
        }

        if (((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1251] != 0.0)) {
            s.store_add_ad(336, A::sub_from_scalar(3.0, A::div(A::scale(s.ad_value(329), 1.259921049894873), A::scale(s.ad_value(332), 3.0))), A::scale(s.ad_value(332), (1.0 / (3.0 * 1.259921049894873))));
        }

        if (((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1251] != 0.0)) {
            s.store_add_ad_lhs(376, A::mul(s.ad_value(336), s.ad_value(227)), 1247);
        }

        if (((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1251] != 0.0)) {
            s.copy_ad(378, 376);
        }

        s.v[1252] = if ((s.v[158] - s.v[383]) <= s.v[1248]) { 1.0 } else { 0.0 };

        s.v[1253] = if (p.p43 == 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1251] != 0.0))) && (s.v[1252] != 0.0)) && (s.v[1253] != 0.0)) {
            s.store_div_from_scalar(327, 1.0, 323);
        }

        if (((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1251] != 0.0))) && (s.v[1252] != 0.0)) && (s.v[1253] != 0.0)) {
            s.store_scale(328, 1231, 9662367879.197212);
        }

        if (((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1251] != 0.0))) && (s.v[1252] != 0.0)) && (s.v[1253] != 0.0)) {
            s.store_scalar(329, (1.0 / s.v[93]));
        }

        if (((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1251] != 0.0))) && (s.v[1252] != 0.0)) && (s.v[1253] != 0.0)) {
            s.store_div_from_scalar_ad(330, 1.0, A::add(A::add(s.ad_value(327), s.ad_value(328)), s.ad_value(329)));
        }

        if (((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1251] != 0.0))) && (s.v[1252] != 0.0)) && (s.v[1253] != 0.0)) {
            s.store_mul_ad_rhs(331, 330, A::add(A::sub(s.ad_value(1232), s.ad_value(475)), A::mul(A::add(s.ad_value(329), A::scale(s.ad_value(328), 0.5)), A::neg(s.ad_value(369)))));
        }

        if (((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1251] != 0.0))) && (s.v[1252] != 0.0)) && (s.v[1253] != 0.0)) {
            s.store_sub_ad_rhs(376, 1232, A::div(s.ad_value(331), s.ad_value(323)));
        }

        if ((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1251] != 0.0))) && (s.v[1252] != 0.0)) {
            s.copy_ad(378, 376);
        }

        if ((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1251] != 0.0))) && (!(s.v[1252] != 0.0))) {
            s.store_div_ad_lhs(328, A::div_from_scalar(1.0, s.ad_value(379)), 434);
        }

        if ((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1251] != 0.0))) && (!(s.v[1252] != 0.0))) {
            s.store_mul_ad(329, A::mul(s.ad_value(328), A::sub(s.ad_value(1232), s.ad_value(383))), A::sub(s.ad_value(1232), s.ad_value(383)));
        }

        if ((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1251] != 0.0))) && (!(s.v[1252] != 0.0))) {
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(1232), s.ad_value(383))));
        }

        if ((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1251] != 0.0))) && (!(s.v[1252] != 0.0))) {
            s.store_offset_ad(377, A::div(A::ln(s.ad_value(329)), s.ad_value(330)), p.p287);
        }

        if ((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1251] != 0.0))) && (!(s.v[1252] != 0.0))) {
            s.store_offset_ad(44, A::sub(s.ad_value(377), s.ad_value(376)), (-0.0008));
        }

        if ((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1251] != 0.0))) && (!(s.v[1252] != 0.0))) {
            s.store_scale(45, 377, (4.0 * 0.0008));
        }

        if ((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1251] != 0.0))) && (!(s.v[1252] != 0.0))) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1251] != 0.0))) && (!(s.v[1252] != 0.0))) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1251] != 0.0))) && (!(s.v[1252] != 0.0))) {
            s.store_sub_ad_rhs(378, 377, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        s.v[1254] = if (p.p43 == 0.0) { 1.0 } else { 0.0 };

        s.v[1255] = if ((s.v[158] - s.v[383]) <= s.v[1248]) { 1.0 } else { 0.0 };

        if ((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (s.v[1255] != 0.0)) {
            s.store_div_from_scalar(327, 1.0, 323);
        }

        if ((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (s.v[1255] != 0.0)) {
            s.store_scale(328, 1231, 9662367879.197212);
        }

        if ((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (s.v[1255] != 0.0)) {
            s.store_scalar(329, (1.0 / s.v[93]));
        }

        if ((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (s.v[1255] != 0.0)) {
            s.store_div_from_scalar_ad(330, 1.0, A::add(A::add(s.ad_value(327), s.ad_value(328)), s.ad_value(329)));
        }

        if ((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (s.v[1255] != 0.0)) {
            s.store_mul_ad_rhs(331, 330, A::add(A::sub(s.ad_value(1232), s.ad_value(475)), A::mul(A::add(s.ad_value(329), A::scale(s.ad_value(328), 0.5)), A::neg(s.ad_value(369)))));
        }

        if ((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (s.v[1255] != 0.0)) {
            s.store_sub_ad_rhs(376, 1232, A::div(s.ad_value(331), s.ad_value(323)));
        }

        if ((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (s.v[1255] != 0.0)) {
            s.copy_ad(378, 376);
        }

        if ((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) {
            s.store_div_from_scalar(327, 1.0, 323);
        }

        if ((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) {
            s.store_scale(328, 1231, 9662367879.197212);
        }

        if ((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) {
            s.store_scalar(329, (1.0 / s.v[93]));
        }

        if ((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) {
            s.store_div_from_scalar_ad(330, 1.0, A::add(A::add(s.ad_value(327), s.ad_value(328)), s.ad_value(329)));
        }

        if ((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) {
            s.store_mul_ad_rhs(331, 330, A::add(A::sub(s.ad_value(1232), s.ad_value(475)), A::mul(A::add(s.ad_value(329), A::scale(s.ad_value(328), 0.5)), A::neg(s.ad_value(369)))));
        }

        if ((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) {
            s.store_sub_ad_rhs(376, 1232, A::div(s.ad_value(331), s.ad_value(323)));
        }

        if ((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) {
            s.copy_ad(378, 376);
        }

        s.v[1256] = if ((s.v[1232] - s.v[383]) > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) {
            s.store_div_ad_lhs(328, A::div_from_scalar(1.0, s.ad_value(379)), 434);
        }

        if (((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) {
            s.store_mul_ad(329, A::mul(s.ad_value(328), A::sub(s.ad_value(1232), s.ad_value(383))), A::sub(s.ad_value(1232), s.ad_value(383)));
        }

        if (((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) {
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(1232), s.ad_value(383))));
        }

        if (((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) {
            s.store_offset_ad(377, A::div(A::ln(s.ad_value(329)), s.ad_value(330)), p.p287);
        }

        s.v[1257] = if ((s.v[376] > ((s.v[377] * 0.98) - 0.4)) && (0.4 >= 0.0)) { 1.0 } else { 0.0 };

        if ((((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_offset_ad(44, A::sub(s.ad_value(376), A::scale(s.ad_value(377), 0.98)), 0.4);
        }

        if ((((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_square(49, 44);
        }

        if ((((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_scalar(50, (0.4 * 0.4));
        }

        if ((((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_scalar(51, 1.0);
        }

        if ((((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_scalar(52, 1.0);
        }

        if ((((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        if ((((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_scalar(55, 0.0);
        }

        if ((((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_scalar(48, 0.0);
        }

        if ((((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_scalar(53, 0.0);
        }

        if ((((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_add(48, 51, 52);
        }

        if ((((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.copy_ad(53, 48);
        }

        s.v[1258] = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1259] = if (2.0 == 1.0) { 1.0 } else { 0.0 };

        if ((((((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) {
            s.store_scalar(55, 1.0);
        }

        s.v[1260] = if (2.0 == 2.0) { 1.0 } else { 0.0 };

        if (((((((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) {
            s.store_scalar(55, 2.0);
        }

        s.v[1261] = if (2.0 == 4.0) { 1.0 } else { 0.0 };

        if ((((((((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1260] != 0.0))) && (s.v[1261] != 0.0)) {
            s.store_scalar(55, 3.0);
        }

        s.v[1262] = if (2.0 == 8.0) { 1.0 } else { 0.0 };

        if (((((((((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1260] != 0.0))) && (!(s.v[1261] != 0.0))) && (s.v[1262] != 0.0)) {
            s.store_scalar(55, 4.0);
        }

        if (((((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) && (s.v[1258] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        let mut assign20680_loop_guard: usize = 0;
        while {
            let assign20680_cond_e28529: f64 = if ((((((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) && (s.v[1258] != 0.0)) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign20680_cond_e28529 != 0.0
        } {
            assign20680_loop_guard += 1;
            assert!(assign20680_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) && (s.v[1258] != 0.0)) {
                s.store_sqrt(53, 53);
            }
            if (((((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) && (s.v[1258] != 0.0)) {
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) && (!(s.v[1258] != 0.0))) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if ((((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_div_from_scalar(53, 1.0, 53);
        }

        if ((((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_mul_ad_lhs(43, A::scale(s.ad_value(44), 0.4), 53);
        }

        if ((((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_add_ad_lhs(378, A::offset(A::scale(s.ad_value(377), 0.98), (-0.4)), 43);
        }

        if ((((((s.v[1220] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) {
            s.copy_ad(378, 376);
        }

        if (s.v[1220] != 0.0) {
            s.store_offset(336, 1247, (5e-12 / 2.0));
        }

        s.v[1263] = if (s.v[378] < s.v[336]) { 1.0 } else { 0.0 };

        if ((s.v[1220] != 0.0) && (s.v[1263] != 0.0)) {
            s.copy_ad(378, 336);
        }

        if (s.v[1220] != 0.0) {
            s.copy_ad(1230, 378);
        }

        if (s.v[1220] != 0.0) {
            s.copy_ad(163, 376);
        }

        if ((s.v[1220] != 0.0) && (0.0 != 0.0)) {
            s.store_ad(166, &{
                if ((s.v[376] - s.v[1230]) >= 0.0) {
                    A::sub(s.ad_value(376), s.ad_value(1230))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if ((s.v[1220] != 0.0) && (0.0 != 0.0)) {
            s.store_offset_ad(44, A::offset(A::scale(s.ad_value(166), (1.0 + 0.3)), (-p.p287)), (-0.03));
        }

        if ((s.v[1220] != 0.0) && (0.0 != 0.0)) {
            s.store_scale(45, 166, ((1.0 + 0.3) * (4.0 * 0.03)));
        }

        if ((s.v[1220] != 0.0) && (0.0 != 0.0)) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((s.v[1220] != 0.0) && (0.0 != 0.0)) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((s.v[1220] != 0.0) && (0.0 != 0.0)) {
            s.store_sub_ad(165, A::scale(s.ad_value(166), (1.0 + 0.3)), A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((s.v[1220] != 0.0) && (0.0 != 0.0)) {
            s.store_ad(165, &{
                if (s.v[165] <= s.v[166]) {
                    s.ad_value(165)
                } else {
                    s.ad_value(166)
                }
            });
        }

        s.v[1264] = if (s.v[165] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1220] != 0.0) && (0.0 != 0.0)) && (s.v[1264] != 0.0)) {
            s.store_scalar(165, 0.0);
        }

        s.v[1265] = if (s.v[165] > s.v[157]) { 1.0 } else { 0.0 };

        if ((((s.v[1220] != 0.0) && (0.0 != 0.0)) && (!(s.v[1264] != 0.0))) && (s.v[1265] != 0.0)) {
            s.copy_ad(165, 157);
        }

        if ((s.v[1220] != 0.0) && (0.0 != 0.0)) {
            s.store_add(163, 1230, 165);
        }

        s.v[1266] = if (p.p282 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) {
            s.copy_ad(378, 1230);
        }

        if ((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) {
            s.copy_ad(1267, 1221);
        }

        if ((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) {
            s.store_offset_ad(160, A::add(A::add(A::sub_from_scalar(s.v[123], s.ad_value(185)), s.ad_value(320)), s.ad_value(1267)), p.p286);
        }

        s.v[1269] = if (s.v[158] < s.v[160]) { 1.0 } else { 0.0 };

        if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (s.v[1269] != 0.0)) {
            s.store_scalar(338, (-1.0));
        }

        if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (s.v[1269] != 0.0)) {
            s.store_mul_ad(254, A::scale(s.ad_value(227), 2.0), A::ln(A::div_from_scalar((-s.v[139]), s.ad_value(240))));
        }

        if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (s.v[1269] != 0.0)) {
            s.store_mul_ad_rhs(336, 225, A::sub(s.ad_value(1232), s.ad_value(1267)));
        }

        if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (s.v[1269] != 0.0)) {
            s.store_div_from_scalar_ad(328, 1.0, A::mul(s.ad_value(225), s.ad_value(238)));
        }

        if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (s.v[1269] != 0.0)) {
            s.store_mul(337, 328, 323);
        }

        if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (s.v[1269] != 0.0)) {
            s.store_offset_scaled(262, 337, (3.0 * 1.414213562373095), 2.0);
        }

        if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (s.v[1269] != 0.0)) {
            s.store_mul_ad_lhs(260, A::mul(A::scale(s.ad_value(262), 8.0), s.ad_value(262)), 262);
        }

        if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (s.v[1269] != 0.0)) {
            s.store_offset(331, 336, (-2.0));
        }

        if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (s.v[1269] != 0.0)) {
            s.store_mul_ad_lhs(332, A::scale(s.ad_value(337), 9.0), 331);
        }

        if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (s.v[1269] != 0.0)) {
            s.store_sub_from_scalar(261, (7.0 * 1.414213562373095), 332);
        }

        if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (s.v[1269] != 0.0)) {
            s.store_square(259, 261);
        }

        s.v[1270] = if (s.v[260] < (s.v[259] * 1e-8)) { 1.0 } else { 0.0 };

        if ((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (s.v[1269] != 0.0)) && (s.v[1270] != 0.0)) {
            s.store_add_ad_lhs(257, A::add(A::offset(s.ad_value(261), ((-7.0) * 1.414213562373095)), A::div(A::scale(s.ad_value(260), 0.5), s.ad_value(261))), 332);
        }

        if ((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (s.v[1269] != 0.0)) && (!(s.v[1270] != 0.0))) {
            s.store_sqrt_ad(258, A::add(s.ad_value(260), s.ad_value(259)));
        }

        if ((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (s.v[1269] != 0.0)) && (!(s.v[1270] != 0.0))) {
            s.store_add_ad_lhs(257, A::offset(s.ad_value(258), ((-7.0) * 1.414213562373095)), 332);
        }

        if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (s.v[1269] != 0.0)) {
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
        if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (s.v[1269] != 0.0)) {
            s.store_add_ad(255, A::add(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(337), 12.0)), A::scale(s.ad_value(256), 2.0)), A::mul(A::scale(s.ad_value(256), 1.414213562373095), s.ad_value(256)));
        }

        if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (s.v[1269] != 0.0)) {
            s.store_div_from_scalar(328, 1.0, 256);
        }

        if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (s.v[1269] != 0.0)) {
            s.store_mul(181, 255, 328);
        }

        if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (s.v[1269] != 0.0)) {
            s.store_add_ad_lhs(313, A::mul(s.ad_value(181), s.ad_value(227)), 1267);
        }

        if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (s.v[1269] != 0.0)) {
            s.store_sub(328, 313, 1267);
        }

        if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (s.v[1269] != 0.0)) {
            s.store_div(329, 328, 254);
        }

        if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (s.v[1269] != 0.0)) {
            s.store_sqrt_ad(330, A::offset(A::square(s.ad_value(329)), 1.0));
        }

        if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (s.v[1269] != 0.0)) {
            s.store_add_ad_lhs(1230, A::div(s.ad_value(328), s.ad_value(330)), 1267);
        }

        if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) {
            s.store_exp_ad(484, A::mul(s.ad_value(225), A::offset(s.ad_value(1267), (-p.p287))));
        }

        if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) {
            s.store_scalar(430, 0.0);
        }

        if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) {
            s.copy_ad(1268, 378);
        }

        if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) {
            s.store_scale_ad(419, A::scale(s.ad_value(229), (p.p237 * (p.p237 * 0.5))), 9662367879.197212);
        }

        if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) {
            s.store_sqrt_ad(327, A::mul(A::scale(s.ad_value(225), 2.0), s.ad_value(419)));
        }

        if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) {
            s.store_scale_ad(328, A::add(A::exp(s.ad_value(327)), A::exp(A::neg(s.ad_value(327)))), 0.5);
        }

        if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) {
            s.store_div_ad_lhs(420, A::ln(s.ad_value(328)), 419);
        }

        if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) {
            s.store_scalar(167, 1.0);
        }

        let mut assign21280_loop_guard: usize = 0;
        while {
            let assign21280_cond_e29259: f64 = (s.v[57] + 1.0);
            let assign21280_cond_e29261: f64 = if ((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (s.v[167] <= assign21280_cond_e29259)) { 1.0 } else { 0.0 };
            assign21280_cond_e29261 != 0.0
        } {
            assign21280_loop_guard += 1;
            assert!(assign21280_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) {
                s.store_sub(417, 1268, 1267);
            }
            if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) {
                s.store_mul(181, 225, 417);
            }
            if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) {
                s.store_mul_ad_rhs(337, 420, A::sub(s.ad_value(417), s.ad_value(419)));
            }
            s.v[1271] = if (s.v[337] < 80.0) { 1.0 } else { 0.0 };
            if ((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (s.v[1271] != 0.0)) {
                s.store_exp(328, 337);
            }
            if ((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (s.v[1271] != 0.0)) {
                s.store_exp_ad(327, A::mul(A::neg(s.ad_value(420)), s.ad_value(419)));
            }
            if ((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (s.v[1271] != 0.0)) {
                s.store_sub(329, 328, 327);
            }
            if ((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (s.v[1271] != 0.0)) {
                s.store_div_ad_lhs(422, A::ln(A::offset(s.ad_value(329), 1.0)), 420);
            }
            if ((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (s.v[1271] != 0.0)) {
                s.store_div_ad_rhs(423, 328, A::offset(s.ad_value(329), 1.0));
            }
            if ((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (!(s.v[1271] != 0.0))) {
                s.store_sub(422, 417, 419);
            }
            if ((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (!(s.v[1271] != 0.0))) {
                s.store_scalar(423, 1.0);
            }
            if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) {
                s.store_mul(421, 225, 422);
            }
            s.v[1272] = if (((s.v[181]) as f64).abs() < 1e-16) { 1.0 } else { 0.0 };
            if ((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (s.v[1272] != 0.0)) {
                s.store_sqrt_ad(327, A::scale(A::sub_from_scalar(1.0, A::square(s.ad_value(423))), 0.5));
            }
            if ((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (s.v[1272] != 0.0)) {
                s.store_mul(242, 181, 327);
            }
            if ((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (s.v[1272] != 0.0)) {
                s.store_mul(443, 225, 327);
            }
            s.v[1273] = if (s.v[181] < 0.0) { 1.0 } else { 0.0 };
            if (((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (s.v[1272] != 0.0)) && (s.v[1273] != 0.0)) {
                s.store_neg(242, 242);
            }
            if (((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (s.v[1272] != 0.0)) && (s.v[1273] != 0.0)) {
                s.store_neg(443, 443);
            }
            s.v[1274] = if (((s.v[181]) as f64).abs() < 0.005) { 1.0 } else { 0.0 };
            if (((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (!(s.v[1272] != 0.0))) && (s.v[1274] != 0.0)) {
                s.store_mul_ad(327, A::scale(A::square(s.ad_value(181)), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(181), 0.3333333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(181), 0.25), A::sub_from_scalar(1.0, A::scale(s.ad_value(181), 0.2)))))));
            }
            if (((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (!(s.v[1272] != 0.0))) && (s.v[1274] != 0.0)) {
                s.store_mul_ad_rhs(328, 181, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(181), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(181), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(181), 0.25)))))));
            }
            if (((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (!(s.v[1272] != 0.0))) && (s.v[1274] != 0.0)) {
                s.store_mul_ad(329, A::scale(A::square(s.ad_value(421)), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(421), 0.3333333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(421), 0.25), A::sub_from_scalar(1.0, A::scale(s.ad_value(421), 0.2)))))));
            }
            if (((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (!(s.v[1272] != 0.0))) && (s.v[1274] != 0.0)) {
                s.store_mul_ad_rhs(330, 421, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(421), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(421), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(421), 0.25)))))));
            }
            if (((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (!(s.v[1272] != 0.0))) && (s.v[1274] != 0.0)) {
                s.store_sqrt_ad(242, A::sub(s.ad_value(327), s.ad_value(329)));
            }
            if (((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (!(s.v[1272] != 0.0))) && (s.v[1274] != 0.0)) {
                s.store_div_ad_lhs(443, A::mul(A::scale(s.ad_value(225), 0.5), A::sub(s.ad_value(328), A::mul(s.ad_value(423), s.ad_value(330)))), 242);
            }
            if (((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (!(s.v[1272] != 0.0))) && (!(s.v[1274] != 0.0))) {
                s.store_exp_ad(327, A::neg(s.ad_value(181)));
            }
            if (((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (!(s.v[1272] != 0.0))) && (!(s.v[1274] != 0.0))) {
                s.store_exp_ad(328, A::neg(s.ad_value(421)));
            }
            if (((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (!(s.v[1272] != 0.0))) && (!(s.v[1274] != 0.0))) {
                s.store_sqrt_ad(242, A::add(A::sub(s.ad_value(181), s.ad_value(421)), A::sub(s.ad_value(327), s.ad_value(328))));
            }
            if (((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (!(s.v[1272] != 0.0))) && (!(s.v[1274] != 0.0))) {
                s.store_div_ad_lhs(443, A::mul(A::scale(s.ad_value(225), 0.5), A::sub(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul(s.ad_value(423), A::sub_from_scalar(1.0, s.ad_value(328))))), 242);
            }
            s.v[1275] = if ((s.v[430] == 1.0) && (s.v[181] < 0.0)) { 1.0 } else { 0.0 };
            if ((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (s.v[1275] != 0.0)) {
                s.store_scalar(338, (-1.0));
            }
            s.v[1276] = if (s.v[181] < 0.0) { 1.0 } else { 0.0 };
            if ((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (s.v[1276] != 0.0)) {
                s.store_neg(490, 242);
            }
            if ((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (s.v[1276] != 0.0)) {
                s.store_neg(491, 443);
            }
            s.v[1277] = if (s.v[181] < 1e-7) { 1.0 } else { 0.0 };
            if (((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (!(s.v[1276] != 0.0))) && (s.v[1277] != 0.0)) {
                s.copy_ad(490, 242);
            }
            if (((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (!(s.v[1276] != 0.0))) && (s.v[1277] != 0.0)) {
                s.copy_ad(491, 443);
            }
            if (((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (!(s.v[1276] != 0.0))) && (!(s.v[1277] != 0.0))) {
                s.store_mul_ad_rhs(501, 225, A::offset(s.ad_value(1268), (-p.p287)));
            }
            if (((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (!(s.v[1276] != 0.0))) && (!(s.v[1277] != 0.0))) {
                s.store_exp(502, 501);
            }
            if (((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (!(s.v[1276] != 0.0))) && (!(s.v[1277] != 0.0))) {
                s.store_mul_ad_rhs(488, 379, A::sub(s.ad_value(502), A::mul(s.ad_value(484), A::offset(s.ad_value(181), 1.0))));
            }
            if (((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (!(s.v[1276] != 0.0))) && (!(s.v[1277] != 0.0))) {
                s.store_mul_ad(489, A::mul(s.ad_value(379), s.ad_value(225)), A::sub(s.ad_value(502), s.ad_value(484)));
            }
            if (((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (!(s.v[1276] != 0.0))) && (!(s.v[1277] != 0.0))) {
                s.store_sqrt_ad(490, A::add(A::square(s.ad_value(242)), s.ad_value(488)));
            }
            if (((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (!(s.v[1276] != 0.0))) && (!(s.v[1277] != 0.0))) {
                s.store_div_ad_lhs(491, A::scale(A::add(A::mul(A::scale(s.ad_value(443), 2.0), s.ad_value(242)), s.ad_value(489)), 0.5), 490);
            }
            if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) {
                s.store_add_ad(492, A::sub(s.ad_value(1268), s.ad_value(1232)), A::mul(s.ad_value(240), s.ad_value(490)));
            }
            if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) {
                s.store_offset_ad(493, A::mul(s.ad_value(240), s.ad_value(491)), 1.0);
            }
            s.v[1278] = if (s.v[430] == 1.0) { 1.0 } else { 0.0 };
            if ((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (s.v[1278] != 0.0)) {
                s.store_scalar(167, (s.v[57] + 1.0));
            }
            if ((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (!(s.v[1278] != 0.0))) {
                s.store_div_ad_lhs(494, A::neg(s.ad_value(492)), 493);
            }
            if ((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (!(s.v[1278] != 0.0))) {
                s.store_scale_ad(496, A::offset({
                    if (1.0 >= ((s.v[1268]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1268))
                    }
                }, 1.0), (0.5 * 0.1));
            }
            s.v[1279] = if (((s.v[494]) as f64).abs() > s.v[496]) { 1.0 } else { 0.0 };
            if (((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (!(s.v[1278] != 0.0))) && (s.v[1279] != 0.0)) {
                s.store_scale(494, 496, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (!(s.v[1278] != 0.0))) {
                s.store_add(1268, 1268, 494);
            }
            s.v[1280] = if ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[492]) as f64).abs() <= 1e-8)) { 1.0 } else { 0.0 };
            if (((((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) && (!(s.v[1278] != 0.0))) && (s.v[1280] != 0.0)) {
                s.store_scalar(430, 1.0);
            }
            if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) {
                s.store_offset(167, 167, 1.0);
            }
        }

        if (((s.v[1220] != 0.0) && (s.v[1266] != 0.0)) && (!(s.v[1269] != 0.0))) {
            s.copy_ad(1230, 1268);
        }

        if (s.v[1220] != 0.0) {
            s.store_mul_ad(332, A::neg(s.ad_value(225)), A::sub(s.ad_value(1230), s.ad_value(1221)));
        }

        if (s.v[1220] != 0.0) {
            s.store_scalar(1245, (if (s.v[332] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if (s.v[1220] != 0.0) {
            s.store_mul(1246, 1245, 332);
        }

        if (s.v[1220] != 0.0) {
            s.store_exp(333, 332);
        }

        if (s.v[1220] != 0.0) {
            s.store_sub_ad_lhs(334, A::offset(s.ad_value(333), (-1.0)), 332);
        }

        s.v[1281] = if (s.v[332] > 1e-7) { 1.0 } else { 0.0 };

        if ((s.v[1220] != 0.0) && (s.v[1281] != 0.0)) {
            s.store_mul_ad(437, A::neg(s.ad_value(238)), A::sqrt(s.ad_value(334)));
        }

        s.v[1282] = if (s.v[1246] > 1e-7) { 1.0 } else { 0.0 };

        if (((s.v[1220] != 0.0) && (!(s.v[1281] != 0.0))) && (s.v[1282] != 0.0)) {
            s.store_mul_ad_rhs(437, 238, A::sqrt(s.ad_value(334)));
        }

        if (((s.v[1220] != 0.0) && (!(s.v[1281] != 0.0))) && (!(s.v[1282] != 0.0))) {
            s.store_mul_ad(437, A::scale(A::mul(A::neg(s.ad_value(1245)), s.ad_value(1246)), 0.7071067811865475), A::sqrt(A::offset(A::mul(A::scale(s.ad_value(1246), 0.3333333333333333), A::offset(A::scale(s.ad_value(1246), 0.25), 1.0)), 1.0)));
        }

        if (s.v[1220] != 0.0) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(437)), ((4.0 * 1e-6) * 1e-6)));
        }

        if (s.v[1220] != 0.0) {
            s.store_offset_ad(1242, A::scale(A::add(s.ad_value(437), s.ad_value(44)), 0.5), (1e-10 * 1e-6));
        }

        s.v[1283] = if (s.v[1242] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1220] != 0.0) && (s.v[1283] != 0.0)) {
            s.store_scalar(1242, 0.0);
        }

        if (s.v[1220] != 0.0) {
            s.store_div_ad_rhs(1243, 1242, A::scale(s.ad_value(536), 1.6021918e-19));
        }

        if (s.v[1220] != 0.0) {
            s.store_sub(328, 1243, 1234);
        }

        if (s.v[1220] != 0.0) {
            s.store_scale(1244, 1243, 0.01);
        }

        if (s.v[1220] != 0.0) {
            s.store_sqrt_ad(44, A::add(A::square(s.ad_value(328)), A::mul(A::scale(s.ad_value(1244), 4.0), s.ad_value(1244))));
        }

        if (s.v[1220] != 0.0) {
            s.store_add_ad(329, A::scale(A::add(s.ad_value(328), s.ad_value(44)), 0.5), A::scale(s.ad_value(1244), 1e-10));
        }

        s.v[1284] = if (s.v[329] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1220] != 0.0) && (s.v[1284] != 0.0)) {
            s.store_scalar(329, 0.0);
        }

        if (s.v[1220] != 0.0) {
            s.store_div_ad_lhs(1241, A::mul(A::div(s.ad_value(329), s.ad_value(1243)), s.ad_value(329)), 1243);
        }

        if (s.v[1220] != 0.0) {
            s.store_add_ad_lhs(1224, A::mul(A::sub(s.ad_value(1230), s.ad_value(1221)), s.ad_value(1241)), 1221);
        }

        if (s.v[1220] != 0.0) {
            s.store_sub_ad(337, A::exp(A::mul(s.ad_value(225), s.ad_value(1224))), A::exp(A::mul(s.ad_value(225), A::sub(s.ad_value(1224), s.ad_value(157)))));
        }

        if (s.v[1220] != 0.0) {
            s.store_sqrt_ad(1237, A::scale(s.ad_value(1225), ((2.0 * 1.6021918e-19) * 1.034943e-10)));
        }

        if (s.v[1220] != 0.0) {
            s.store_mul_ad_rhs(1238, 1237, A::sqrt(s.ad_value(227)));
        }

        if (s.v[1220] != 0.0) {
            s.store_mul_ad_rhs(1229, 225, A::sub(s.ad_value(1224), s.ad_value(1221)));
        }

        s.v[1285] = if ((s.v[1229] < (0.2 * s.v[225])) && ((0.2 * s.v[225]) >= 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1220] != 0.0) && (s.v[1285] != 0.0)) {
            s.store_sub_ad_lhs(44, A::scale(s.ad_value(225), 0.2), 1229);
        }

        if ((s.v[1220] != 0.0) && (s.v[1285] != 0.0)) {
            s.store_square(49, 44);
        }

        if ((s.v[1220] != 0.0) && (s.v[1285] != 0.0)) {
            s.store_mul_ad(50, A::scale(s.ad_value(225), 0.2), A::scale(s.ad_value(225), 0.2));
        }

        if ((s.v[1220] != 0.0) && (s.v[1285] != 0.0)) {
            s.store_scalar(51, 1.0);
        }

        if ((s.v[1220] != 0.0) && (s.v[1285] != 0.0)) {
            s.store_scalar(52, 1.0);
        }

        if ((s.v[1220] != 0.0) && (s.v[1285] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        if ((s.v[1220] != 0.0) && (s.v[1285] != 0.0)) {
            s.store_scalar(55, 0.0);
        }

        if ((s.v[1220] != 0.0) && (s.v[1285] != 0.0)) {
            s.store_scalar(48, 0.0);
        }

        if ((s.v[1220] != 0.0) && (s.v[1285] != 0.0)) {
            s.store_scalar(53, 0.0);
        }

        if ((s.v[1220] != 0.0) && (s.v[1285] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((s.v[1220] != 0.0) && (s.v[1285] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((s.v[1220] != 0.0) && (s.v[1285] != 0.0)) {
            s.store_add(48, 51, 52);
        }

        if ((s.v[1220] != 0.0) && (s.v[1285] != 0.0)) {
            s.copy_ad(53, 48);
        }

        s.v[1286] = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1287] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[1220] != 0.0) && (s.v[1285] != 0.0)) && (s.v[1286] != 0.0)) && (s.v[1287] != 0.0)) {
            s.store_scalar(55, 1.0);
        }

        s.v[1288] = if (1.0 == 2.0) { 1.0 } else { 0.0 };

        if (((((s.v[1220] != 0.0) && (s.v[1285] != 0.0)) && (s.v[1286] != 0.0)) && (!(s.v[1287] != 0.0))) && (s.v[1288] != 0.0)) {
            s.store_scalar(55, 2.0);
        }

        s.v[1289] = if (1.0 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1220] != 0.0) && (s.v[1285] != 0.0)) && (s.v[1286] != 0.0)) && (!(s.v[1287] != 0.0))) && (!(s.v[1288] != 0.0))) && (s.v[1289] != 0.0)) {
            s.store_scalar(55, 3.0);
        }

        s.v[1290] = if (1.0 == 8.0) { 1.0 } else { 0.0 };

        if (((((((s.v[1220] != 0.0) && (s.v[1285] != 0.0)) && (s.v[1286] != 0.0)) && (!(s.v[1287] != 0.0))) && (!(s.v[1288] != 0.0))) && (!(s.v[1289] != 0.0))) && (s.v[1290] != 0.0)) {
            s.store_scalar(55, 4.0);
        }

        if (((s.v[1220] != 0.0) && (s.v[1285] != 0.0)) && (s.v[1286] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        let mut assign21810_loop_guard: usize = 0;
        while {
            let assign21810_cond_e30576: f64 = if ((((s.v[1220] != 0.0) && (s.v[1285] != 0.0)) && (s.v[1286] != 0.0)) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign21810_cond_e30576 != 0.0
        } {
            assign21810_loop_guard += 1;
            assert!(assign21810_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[1220] != 0.0) && (s.v[1285] != 0.0)) && (s.v[1286] != 0.0)) {
                s.store_sqrt(53, 53);
            }
            if (((s.v[1220] != 0.0) && (s.v[1285] != 0.0)) && (s.v[1286] != 0.0)) {
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((s.v[1220] != 0.0) && (s.v[1285] != 0.0)) && (!(s.v[1286] != 0.0))) {
            s.store_powf(53, 53, (1.0 / 2.0));
        }

        if ((s.v[1220] != 0.0) && (s.v[1285] != 0.0)) {
            s.store_div_from_scalar(53, 1.0, 53);
        }

        if ((s.v[1220] != 0.0) && (s.v[1285] != 0.0)) {
            s.store_mul_ad_lhs(43, A::mul(s.ad_value(44), A::scale(s.ad_value(225), 0.2)), 53);
        }

        if ((s.v[1220] != 0.0) && (s.v[1285] != 0.0)) {
            s.store_sub_ad_lhs(328, A::scale(s.ad_value(225), 0.2), 43);
        }

        if ((s.v[1220] != 0.0) && (!(s.v[1285] != 0.0))) {
            s.copy_ad(328, 1229);
        }

        if (s.v[1220] != 0.0) {
            s.store_sqrt_ad(1239, A::offset(s.ad_value(328), (10.0 * 2.220446049250313e-16)));
        }

        if (s.v[1220] != 0.0) {
            s.store_mul(1240, 1238, 1239);
        }

        if (s.v[1220] != 0.0) {
            s.store_mul_ad_lhs(1236, A::div(A::scale(s.ad_value(227), 2.0), s.ad_value(1227)), 1240);
        }

        if (s.v[1220] != 0.0) {
            s.store_mul_ad_lhs(204, A::mul(A::mul(s.ad_value(1236), s.ad_value(1235)), s.ad_value(107)), 337);
        }

        if (s.v[1220] != 0.0) {
            s.store_add(199, 202, 204);
        }

        s.store_add(201, 203, 204);

        s.v[1291] = if ((p.p43 == 1.0) || (p.p45 == 1.0)) { 1.0 } else { 0.0 };

        s.v[1304] = if ((s.v[145] == 1.0) || (p.p25 == 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1291] != 0.0) && (s.v[1304] != 0.0)) {
            s.store_scalar(263, 0.0);
        }

        s.v[1305] = if ((p.p117 <= 0.0) || (s.v[73] <= 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (s.v[1305] != 0.0)) {
            s.store_scalar(263, 0.0);
        }

        if (((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) {
            s.store_offset_ad(445, A::sub(A::add(A::offset(s.ad_value(174), (-s.v[136])), s.ad_value(185)), s.ad_value(320)), p.p48);
        }

        s.v[1306] = if (p.p44 <= 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (s.v[1306] != 0.0)) {
            s.copy_ad(1292, 445);
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (s.v[1306] != 0.0)) {
            s.store_square(1299, 323);
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (s.v[1306] != 0.0)) {
            s.copy_ad(1300, 545);
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (s.v[1306] != 0.0)) {
            s.store_div(1294, 1300, 1299);
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
        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (s.v[1306] != 0.0)) {
            s.store_div_from_scalar(1301, 2.0, 1300);
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (s.v[1306] != 0.0)) {
            s.store_mul(1295, 1301, 1299);
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (s.v[1306] != 0.0)) {
            s.store_sub_ad(1296, A::sub(s.ad_value(1292), s.ad_value(227)), A::mul(s.ad_value(130), s.ad_value(514)));
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (s.v[1306] != 0.0)) {
            s.store_scale(483, 393, (p.p49 * 1.0 / (s.v[89])));
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (s.v[1306] != 0.0)) {
            s.store_sub_ad_rhs(1296, 1296, A::mul(s.ad_value(130), s.ad_value(483)));
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (s.v[1306] != 0.0)) {
            s.store_offset_ad(1298, A::mul(s.ad_value(1295), s.ad_value(1296)), 1.0);
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (s.v[1306] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1298)), ((4.0 * 0.001) * 0.001)));
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (s.v[1306] != 0.0)) {
            s.store_offset_ad(1297, A::scale(A::add(s.ad_value(1298), s.ad_value(44)), 0.5), (1e-10 * 0.001));
        }

        s.v[1307] = if (s.v[1297] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (s.v[1306] != 0.0)) && (s.v[1307] != 0.0)) {
            s.store_scalar(1297, 0.0);
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (s.v[1306] != 0.0)) {
            s.store_offset(1297, 1297, 1e-50);
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (s.v[1306] != 0.0)) {
            s.store_sqrt(1297, 1297);
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (s.v[1306] != 0.0)) {
            s.store_add_ad(1302, A::mul(s.ad_value(1292), s.ad_value(137)), A::mul(s.ad_value(1294), A::sub_from_scalar(1.0, s.ad_value(1297))));
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (s.v[1306] != 0.0)) {
            s.store_sub_ad(1303, A::add(A::scale(s.ad_value(173), p.p122), s.ad_value(176)), A::mul(A::mul(s.ad_value(131), s.ad_value(129)), s.ad_value(1302)));
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (s.v[1306] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1303)), ((4.0 * 0.01) * 0.01)));
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (s.v[1306] != 0.0)) {
            s.store_offset_ad(1303, A::scale(A::add(s.ad_value(1303), s.ad_value(44)), 0.5), (1e-10 * 0.01));
        }

        s.v[1308] = if (s.v[1303] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (s.v[1306] != 0.0)) && (s.v[1308] != 0.0)) {
            s.store_scalar(1303, 0.0);
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) {
            s.store_mul(1292, 134, 445);
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) {
            s.store_div_ad_rhs(1294, 545, A::square(s.ad_value(323)));
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) {
            s.store_mul_ad(1295, A::div_from_scalar(2.0, s.ad_value(545)), A::square(s.ad_value(323)));
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) {
            s.store_sub_ad(1296, A::sub(s.ad_value(1292), s.ad_value(227)), A::mul(s.ad_value(130), s.ad_value(514)));
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) {
            s.store_scale(483, 393, (p.p49 * 1.0 / (s.v[89])));
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) {
            s.store_sub_ad_rhs(1296, 1296, A::mul(s.ad_value(130), s.ad_value(483)));
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) {
            s.store_offset_ad(1297, A::mul(s.ad_value(1295), s.ad_value(1296)), 1.0);
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) {
            s.store_scaled_offset(1299, 1295, 1.0, 2.0);
        }

        s.v[1309] = if ((s.v[1297] < (1e-50 + s.v[1299])) && (s.v[1299] >= 0.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) {
            s.store_sub_ad_lhs(44, A::offset(s.ad_value(1299), 1e-50), 1297);
        }

        if (((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) {
            s.store_square(49, 44);
        }

        if (((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) {
            s.store_square(50, 1299);
        }

        if (((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) {
            s.store_scalar(51, 1.0);
        }

        if (((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) {
            s.store_scalar(52, 1.0);
        }

        if (((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        if (((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) {
            s.store_scalar(55, 0.0);
        }

        if (((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) {
            s.store_scalar(48, 0.0);
        }

        if (((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) {
            s.store_scalar(53, 0.0);
        }

        if (((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if (((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if (((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if (((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if (((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if (((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if (((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if (((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if (((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) {
            s.store_add(48, 51, 52);
        }

        if (((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) {
            s.copy_ad(53, 48);
        }

        s.v[1310] = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1311] = if (4.0 == 1.0) { 1.0 } else { 0.0 };

        if (((((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) && (s.v[1310] != 0.0)) && (s.v[1311] != 0.0)) {
            s.store_scalar(55, 1.0);
        }

        s.v[1312] = if (4.0 == 2.0) { 1.0 } else { 0.0 };

        if ((((((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) && (s.v[1310] != 0.0)) && (!(s.v[1311] != 0.0))) && (s.v[1312] != 0.0)) {
            s.store_scalar(55, 2.0);
        }

        s.v[1313] = if (4.0 == 4.0) { 1.0 } else { 0.0 };

        if (((((((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) && (s.v[1310] != 0.0)) && (!(s.v[1311] != 0.0))) && (!(s.v[1312] != 0.0))) && (s.v[1313] != 0.0)) {
            s.store_scalar(55, 3.0);
        }

        s.v[1314] = if (4.0 == 8.0) { 1.0 } else { 0.0 };

        if ((((((((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) && (s.v[1310] != 0.0)) && (!(s.v[1311] != 0.0))) && (!(s.v[1312] != 0.0))) && (!(s.v[1313] != 0.0))) && (s.v[1314] != 0.0)) {
            s.store_scalar(55, 4.0);
        }

        if ((((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) && (s.v[1310] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        let mut assign22600_loop_guard: usize = 0;
        while {
            let assign22600_cond_e31695: f64 = if (((((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) && (s.v[1310] != 0.0)) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign22600_cond_e31695 != 0.0
        } {
            assign22600_loop_guard += 1;
            assert!(assign22600_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) && (s.v[1310] != 0.0)) {
                s.store_sqrt(53, 53);
            }
            if ((((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) && (s.v[1310] != 0.0)) {
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) && (!(s.v[1310] != 0.0))) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if (((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) {
            s.store_div_from_scalar(53, 1.0, 53);
        }

        if (((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) {
            s.store_mul_ad_lhs(43, A::mul(s.ad_value(44), s.ad_value(1299)), 53);
        }

        if (((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1309] != 0.0)) {
            s.store_sub_ad_lhs(1297, A::offset(s.ad_value(1299), 1e-50), 43);
        }

        if (((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (!(s.v[1309] != 0.0))) {
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) {
            s.store_ad(1297, &{
                if (s.v[1297] <= 0.0) {
                    A::constant(0.0)
                } else {
                    A::sqrt(s.ad_value(1297))
                }
            });
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) {
            s.store_add_ad_rhs(1302, 1292, A::mul(s.ad_value(1294), A::sub_from_scalar(1.0, s.ad_value(1297))));
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) {
            s.store_div_from_scalar_ad(1293, s.v[100], A::offset(s.ad_value(131), s.v[100]));
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) {
            s.store_sub_ad(1303, A::add(A::scale(s.ad_value(173), p.p122), s.ad_value(176)), A::mul(s.ad_value(1293), s.ad_value(1302)));
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1303)), ((4.0 * 0.001) * 0.001)));
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) {
            s.store_offset_ad(1303, A::scale(A::add(s.ad_value(1303), s.ad_value(44)), 0.5), (1e-10 * 0.001));
        }

        s.v[1315] = if (s.v[1303] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1315] != 0.0)) {
            s.store_scalar(1303, 0.0);
        }

        if (((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) {
            s.store_offset(1303, 1303, 1e-50);
        }

        if (((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) {
            s.store_exp_ad(1293, A::div(A::neg(s.ad_value(133)), s.ad_value(1303)));
        }

        if (((s.v[1291] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) {
            s.store_mul_ad_lhs(263, A::mul(A::mul(s.ad_value(132), s.ad_value(1303)), s.ad_value(199)), 1293);
        }

        s.v[1316] = if (((p.p25 == 1.0) && (p.p26 == 2.0)) && (p.p43 == 1.0)) { 1.0 } else { 0.0 };

        if (s.v[1316] != 0.0) {
            s.store_scale(1320, 227, 0.0);
        }

        if (s.v[1316] != 0.0) {
            s.store_sub_ad(44, A::sub(s.ad_value(231), s.ad_value(1320)), A::scale(s.ad_value(231), 0.01));
        }

        if (s.v[1316] != 0.0) {
            s.store_mul_ad(45, A::scale(s.ad_value(231), 4.0), A::scale(s.ad_value(231), 0.01));
        }

        if (s.v[1316] != 0.0) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (s.v[1316] != 0.0) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if (s.v[1316] != 0.0) {
            s.store_sub_ad_rhs(1320, 231, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if (s.v[1316] != 0.0) {
            s.store_sqrt_ad(1321, A::mul(A::scale(s.ad_value(544), ((2.0 * 1.034943e-10) * 1.6021918e-19)), s.ad_value(227)));
        }

        if (s.v[1316] != 0.0) {
            s.store_mul_ad_rhs(1322, 225, A::sub(s.ad_value(176), s.ad_value(1320)));
        }

        if (s.v[1316] != 0.0) {
            s.store_ad(1322, &{
                if (s.v[1322] > 0.0) {
                    A::sqrt(s.ad_value(1322))
                } else {
                    A::neg(A::sqrt(A::neg(s.ad_value(1322))))
                }
            });
        }

        if (s.v[1316] != 0.0) {
            s.store_sqrt_ad(1323, A::mul(s.ad_value(225), s.ad_value(176)));
        }

        if (s.v[1316] != 0.0) {
            s.store_mul_ad(1324, A::neg(s.ad_value(1321)), A::sub(s.ad_value(1322), s.ad_value(1323)));
        }

        if (s.v[1316] != 0.0) {
            s.store_offset_ad(44, A::sub_from_scalar(p.p47, s.ad_value(1324)), (-(p.p47 * 0.01)));
        }

        if (s.v[1316] != 0.0) {
            s.store_scalar(45, ((4.0 * p.p47) * (p.p47 * 0.01)));
        }

        if (s.v[1316] != 0.0) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (s.v[1316] != 0.0) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if (s.v[1316] != 0.0) {
            s.store_sub_from_scalar_ad(393, p.p47, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if (s.v[1316] != 0.0) {
            s.store_ad(596, &A::scale(A::voltage(ctx, &nodes, Some(17), None), (1e-9 / 0.0001)));
        }

        if (s.v[1316] != 0.0) {
            s.copy_ad(393, 596);
        }

        s.v[1338] = if (((s.v[145] == 0.0) && (s.v[263] > 0.0)) && (p.p146 != 0.0)) { 1.0 } else { 0.0 };

        s.v[1339] = if (s.v[56] < 3.0) { 1.0 } else { 0.0 };

        if ((s.v[1338] != 0.0) && (s.v[1339] != 0.0)) {
            s.store_scalar(516, 0.0);
        }

        if ((s.v[1338] != 0.0) && (s.v[1339] != 0.0)) {
            s.store_scalar(517, 0.0);
        }

        if ((s.v[1338] != 0.0) && (!(s.v[1339] != 0.0))) {
            s.store_ad(516, &{
                if (p.p43 == 1.0) {
                    s.ad_value(156)
                } else {
                    s.ad_value(350)
                }
            });
        }

        if ((s.v[1338] != 0.0) && (!(s.v[1339] != 0.0))) {
            s.store_ad(517, &{
                if (p.p43 == 1.0) {
                    s.ad_value(156)
                } else {
                    s.ad_value(353)
                }
            });
        }

        if (s.v[1338] != 0.0) {
            s.store_offset_scaled(1325, 185, p.p147, 1.0);
        }

        if (s.v[1338] != 0.0) {
            s.store_mul_ad_lhs(1326, A::scale(s.ad_value(1325), p.p146), 263);
        }

        if (s.v[1338] != 0.0) {
            s.store_offset_ad(1327, A::mul(s.ad_value(225), A::sub(s.ad_value(161), s.ad_value(516))), (-1.0));
        }

        if (s.v[1338] != 0.0) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1327)), ((4.0 * 0.1) * 0.1)));
        }

        if (s.v[1338] != 0.0) {
            s.store_offset_ad(1327, A::scale(A::add(s.ad_value(1327), s.ad_value(44)), 0.5), (1e-10 * 0.1));
        }

        s.v[1340] = if (s.v[1327] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1338] != 0.0) && (s.v[1340] != 0.0)) {
            s.store_scalar(1327, 0.0);
        }

        if (s.v[1338] != 0.0) {
            s.store_sqrt(1328, 1327);
        }

        if (s.v[1338] != 0.0) {
            s.store_mul(1329, 1327, 1328);
        }

        if (s.v[1338] != 0.0) {
            s.store_offset_ad(1330, A::mul(s.ad_value(225), A::sub(s.ad_value(162), s.ad_value(517))), (-1.0));
        }

        if (s.v[1338] != 0.0) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1330)), ((4.0 * 0.1) * 0.1)));
        }

        if (s.v[1338] != 0.0) {
            s.store_offset_ad(1330, A::scale(A::add(s.ad_value(1330), s.ad_value(44)), 0.5), (1e-10 * 0.1));
        }

        s.v[1341] = if (s.v[1330] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1338] != 0.0) && (s.v[1341] != 0.0)) {
            s.store_scalar(1330, 0.0);
        }

        if (s.v[1338] != 0.0) {
            s.store_sqrt(1331, 1330);
        }

        if (s.v[1338] != 0.0) {
            s.store_mul(1332, 1330, 1331);
        }

        if (s.v[1338] != 0.0) {
            s.store_div_from_scalar(1333, 1.0, 1327);
        }

        if (s.v[1338] != 0.0) {
            s.store_mul_ad_lhs(328, A::mul(s.ad_value(225), s.ad_value(1326)), 1333);
        }

        if (s.v[1338] != 0.0) {
            s.store_div_from_scalar(1333, 1.0, 1330);
        }

        if (s.v[1338] != 0.0) {
            s.store_mul_ad_lhs(1334, A::mul(s.ad_value(225), s.ad_value(1326)), 1333);
        }

        if (s.v[1338] != 0.0) {
            s.store_mul_ad_rhs(1335, 238, A::sub(A::mul(s.ad_value(1332), s.ad_value(1334)), A::mul(s.ad_value(1329), s.ad_value(328))));
        }

        if (s.v[1338] != 0.0) {
            s.store_mul_ad(1336, A::scale(s.ad_value(238), 0.5), A::add(A::mul(A::neg(s.ad_value(1331)), s.ad_value(1334)), A::mul(s.ad_value(1328), s.ad_value(328))));
        }

        if (s.v[1338] != 0.0) {
            s.store_add(1337, 1335, 1336);
        }

        if (s.v[1338] != 0.0) {
            s.store_mul_ad_lhs(265, A::mul(s.ad_value(264), s.ad_value(1337)), 250);
        }

        s.v[1355] = (s.v[88] * 100.0);

        s.store_scale(1356, 323, 0.0001);

        s.v[1357] = (s.v[97] * 100.0);

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
        s.store_scale(1358, 107, 100.0);

        s.store_scale(1359, 252, 0.01);

        s.store_scale(1360, 436, 0.0001);

        s.store_scale(1361, 238, 0.0001);

        s.v[1362] = if (p.p27 == 0.0) { 1.0 } else { 0.0 };

        s.v[1363] = if (s.v[145] == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1362] != 0.0)) && (s.v[1363] != 0.0)) {
            s.store_offset_ad(1354, A::add(s.ad_value(176), s.ad_value(173)), (-(10.0 * 2.220446049250313e-16)));
        }

        if ((!(s.v[1362] != 0.0)) && (s.v[1363] != 0.0)) {
            s.store_sub_ad(1344, A::add(A::offset(s.ad_value(174), (-s.v[123])), A::scale(A::sub(s.ad_value(185), s.ad_value(320)), (p.p216 * s.v[1357]))), A::scale(s.ad_value(1354), p.p215));
        }

        if ((!(s.v[1362] != 0.0)) && (s.v[1363] != 0.0)) {
            s.store_scalar(1346, (1.0 / s.v[1355]));
        }

        if ((!(s.v[1362] != 0.0)) && (s.v[1363] != 0.0)) {
            s.store_mul(1345, 1344, 1346);
        }

        if ((!(s.v[1362] != 0.0)) && (s.v[1363] != 0.0)) {
            s.store_scalar(1346, (1.0 / p.p217));
        }

        if ((!(s.v[1362] != 0.0)) && (s.v[1363] != 0.0)) {
            s.store_offset_ad(1350, A::mul(s.ad_value(1359), s.ad_value(1346)), 1.0);
        }

        if ((!(s.v[1362] != 0.0)) && (s.v[1363] != 0.0)) {
            s.store_mul(1353, 1345, 1350);
        }

        if ((!(s.v[1362] != 0.0)) && (s.v[1363] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1353)), ((4.0 * 0.01) * 0.01)));
        }

        if ((!(s.v[1362] != 0.0)) && (s.v[1363] != 0.0)) {
            s.store_offset_ad(1353, A::scale(A::add(s.ad_value(1353), s.ad_value(44)), 0.5), (1e-10 * 0.01));
        }

        s.v[1364] = if (s.v[1353] < 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1362] != 0.0)) && (s.v[1363] != 0.0)) && (s.v[1364] != 0.0)) {
            s.store_scalar(1353, 0.0);
        }

        if ((!(s.v[1362] != 0.0)) && (s.v[1363] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(174)), ((4.0 * 0.001) * 0.001)));
        }

        if ((!(s.v[1362] != 0.0)) && (s.v[1363] != 0.0)) {
            s.store_offset_ad(1346, A::scale(A::add(s.ad_value(174), s.ad_value(44)), 0.5), (1e-10 * 0.001));
        }

        s.v[1365] = if (s.v[1346] < 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1362] != 0.0)) && (s.v[1363] != 0.0)) && (s.v[1365] != 0.0)) {
            s.store_scalar(1346, 0.0);
        }

        if ((!(s.v[1362] != 0.0)) && (s.v[1363] != 0.0)) {
            s.store_offset(1346, 1346, (-p.p226));
        }

        if ((!(s.v[1362] != 0.0)) && (s.v[1363] != 0.0)) {
            s.store_scale(1342, 1346, 10.0);
        }

        if ((!(s.v[1362] != 0.0)) && (s.v[1363] != 0.0)) {
            s.store_offset_ad(1345, A::square(s.ad_value(1342)), 1.0);
        }

        if ((!(s.v[1362] != 0.0)) && (s.v[1363] != 0.0)) {
            s.store_sub_from_scalar_ad(1344, 1.0, A::div_from_scalar(1.0, s.ad_value(1345)));
        }

        if ((!(s.v[1362] != 0.0)) && (s.v[1363] != 0.0)) {
            s.store_mul(1353, 1353, 1344);
        }

        if ((!(s.v[1362] != 0.0)) && (s.v[1363] != 0.0)) {
            s.store_scale(1343, 1358, s.v[1357]);
        }

        if ((!(s.v[1362] != 0.0)) && (s.v[1363] != 0.0)) {
            s.store_div_from_scalar_ad(1350, p.p219, A::offset(s.ad_value(1343), p.p219));
        }

        if ((!(s.v[1362] != 0.0)) && (s.v[1363] != 0.0)) {
            s.store_scalar(1349, p.p218);
        }

        if ((!(s.v[1362] != 0.0)) && (s.v[1363] != 0.0)) {
            s.store_div_from_scalar_ad(1347, 1.0, A::offset(s.ad_value(1353), 1e-50));
        }

        if ((!(s.v[1362] != 0.0)) && (s.v[1363] != 0.0)) {
            s.store_mul_ad_lhs(1344, A::scale(s.ad_value(303), (-p.p214)), 1347);
        }

        s.v[1366] = if (s.v[1344] < (-34.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1362] != 0.0)) && (s.v[1363] != 0.0)) && (!(s.v[1366] != 0.0))) {
            s.store_exp(1345, 1344);
        }

        if (((!(s.v[1362] != 0.0)) && (s.v[1363] != 0.0)) && (!(s.v[1366] != 0.0))) {
            s.store_mul_ad_lhs(1346, A::scale(A::div_from_scalar(p.p213, s.ad_value(302)), 1.6021918e-19), 1343);
        }

        if (((!(s.v[1362] != 0.0)) && (s.v[1363] != 0.0)) && (!(s.v[1366] != 0.0))) {
            s.store_div_from_scalar(1348, 1.0, 1361);
        }

        if (((!(s.v[1362] != 0.0)) && (s.v[1363] != 0.0)) && (!(s.v[1366] != 0.0))) {
            s.store_sqrt_ad(1349, A::mul(A::add(s.ad_value(1360), A::scale(s.ad_value(1356), 1e-12)), s.ad_value(1348)));
        }

        if (((!(s.v[1362] != 0.0)) && (s.v[1363] != 0.0)) && (!(s.v[1366] != 0.0))) {
            s.store_mul_ad_lhs(1347, A::mul(s.ad_value(1345), s.ad_value(1346)), 1349);
        }

        if (!(s.v[1362] != 0.0)) {
            s.store_offset_scaled(1343, 158, (-p.p221), p.p222);
        }

        if (!(s.v[1362] != 0.0)) {
            s.store_exp_ad(1345, A::scale(s.ad_value(1343), s.v[1355]));
        }

        if (!(s.v[1362] != 0.0)) {
            s.store_scale_ad(1343, A::scale(s.ad_value(158), 1.0 / (s.v[1355])), 1.0 / (s.v[1355]));
        }

        if (!(s.v[1362] != 0.0)) {
            s.store_mul(1346, 158, 1343);
        }

        if (!(s.v[1362] != 0.0)) {
            s.store_scale(1347, 1358, (p.p220 / 1000000.0));
        }

        if (!(s.v[1362] != 0.0)) {
            s.store_sub(1344, 158, 157);
        }

        if (!(s.v[1362] != 0.0)) {
            s.store_offset_scaled(1343, 1344, (-p.p221), p.p222);
        }

        if (!(s.v[1362] != 0.0)) {
            s.store_exp_ad(1345, A::scale(s.ad_value(1343), s.v[1355]));
        }

        if (!(s.v[1362] != 0.0)) {
            s.store_scale_ad(1343, A::scale(s.ad_value(1344), 1.0 / (s.v[1355])), 1.0 / (s.v[1355]));
        }

        if (!(s.v[1362] != 0.0)) {
            s.store_mul(1346, 1344, 1343);
        }

        if (!(s.v[1362] != 0.0)) {
            s.store_scale(1347, 1358, (p.p220 / 1000000.0));
        }

        if (!(s.v[1362] != 0.0)) {
            s.store_scale_ad(1353, A::offset(A::offset(A::sub(s.ad_value(513), s.ad_value(158)), s.v[123]), p.p225), 1.0 / (s.v[1355]));
        }

        if (!(s.v[1362] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1353)), ((4.0 * 0.01) * 0.01)));
        }

        if (!(s.v[1362] != 0.0)) {
            s.store_offset_ad(1353, A::scale(A::add(s.ad_value(1353), s.ad_value(44)), 0.5), (1e-10 * 0.01));
        }

        s.v[1369] = if (s.v[1353] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1362] != 0.0)) && (s.v[1369] != 0.0)) {
            s.store_scalar(1353, 0.0);
        }

        if (!(s.v[1362] != 0.0)) {
            s.store_offset(1353, 1353, 1e-50);
        }

        if (!(s.v[1362] != 0.0)) {
            s.store_div_from_scalar(1344, (-p.p224), 1353);
        }

        s.v[1370] = if (s.v[1344] < (-34.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1362] != 0.0)) && (!(s.v[1370] != 0.0))) {
            s.store_exp(1345, 1344);
        }

        if ((!(s.v[1362] != 0.0)) && (!(s.v[1370] != 0.0))) {
            s.store_scale(1346, 1358, (p.p223 * s.v[1357]));
        }

        s.v[1378] = if (p.p28 == 0.0) { 1.0 } else { 0.0 };

        if (!(s.v[1378] != 0.0)) {
            s.store_add_ad(1371, A::sub(A::scale(A::offset(s.ad_value(157), p.p210), p.p209), s.ad_value(158)), A::scale(A::add(s.ad_value(187), s.ad_value(319)), p.p211));
        }

        if (!(s.v[1378] != 0.0)) {
            s.store_scalar(1372, (1.0 / s.v[88]));
        }

        if (!(s.v[1378] != 0.0)) {
            s.store_mul(1373, 1371, 1372);
        }

        if (!(s.v[1378] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1373)), ((4.0 * 0.01) * 0.01)));
        }

        if (!(s.v[1378] != 0.0)) {
            s.store_offset_ad(304, A::scale(A::add(s.ad_value(1373), s.ad_value(44)), 0.5), (1e-10 * 0.01));
        }

        s.v[1379] = if (s.v[304] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1378] != 0.0)) && (s.v[1379] != 0.0)) {
            s.store_scalar(304, 0.0);
        }

        if (!(s.v[1378] != 0.0)) {
            s.store_div_from_scalar_ad(1374, 1.0, A::offset(s.ad_value(304), 1e-50));
        }

        if (!(s.v[1378] != 0.0)) {
            s.store_mul_ad_lhs(1375, A::scale(s.ad_value(303), (-p.p208)), 1374);
        }

        s.v[1380] = if (s.v[1375] < (-34.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1378] != 0.0)) && (!(s.v[1380] != 0.0))) {
            s.store_exp(1371, 1375);
        }

        if ((!(s.v[1378] != 0.0)) && (!(s.v[1380] != 0.0))) {
            s.store_mul_ad_lhs(1372, A::scale(A::div_from_scalar(p.p207, s.ad_value(302)), 1.6021918e-19), 107);
        }

        if (!(s.v[1378] != 0.0)) {
            s.store_sub(1377, 157, 513);
        }

        s.v[1381] = if (s.v[1377] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1378] != 0.0)) && (s.v[1381] != 0.0)) {
            s.store_square(1372, 1377);
        }

        if ((!(s.v[1378] != 0.0)) && (s.v[1381] != 0.0)) {
            s.store_mul(331, 1372, 1377);
        }

        if ((!(s.v[1378] != 0.0)) && (s.v[1381] != 0.0)) {
            s.store_offset(1375, 331, p.p212);
        }

        s.v[1389] = if (p.p28 == 0.0) { 1.0 } else { 0.0 };

        if (!(s.v[1389] != 0.0)) {
            s.store_add_ad(1382, A::sub(A::scale(A::sub_from_scalar(p.p210, s.ad_value(157)), p.p209), A::sub(s.ad_value(158), s.ad_value(157))), A::scale(A::add(s.ad_value(187), s.ad_value(319)), p.p211));
        }

        if (!(s.v[1389] != 0.0)) {
            s.store_scalar(1383, (1.0 / s.v[88]));
        }

        if (!(s.v[1389] != 0.0)) {
            s.store_mul(1384, 1382, 1383);
        }

        if (!(s.v[1389] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1384)), ((4.0 * 0.01) * 0.01)));
        }

        if (!(s.v[1389] != 0.0)) {
            s.store_offset_ad(305, A::scale(A::add(s.ad_value(1384), s.ad_value(44)), 0.5), (1e-10 * 0.01));
        }

        s.v[1390] = if (s.v[305] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1389] != 0.0)) && (s.v[1390] != 0.0)) {
            s.store_scalar(305, 0.0);
        }

        if (!(s.v[1389] != 0.0)) {
            s.store_div_from_scalar_ad(1385, 1.0, A::offset(s.ad_value(305), 1e-50));
        }

        if (!(s.v[1389] != 0.0)) {
            s.store_mul_ad_lhs(1386, A::scale(s.ad_value(303), (-p.p208)), 1385);
        }

        s.v[1391] = if (s.v[1386] < (-34.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1389] != 0.0)) && (!(s.v[1391] != 0.0))) {
            s.store_exp(1382, 1386);
        }

        if ((!(s.v[1389] != 0.0)) && (!(s.v[1391] != 0.0))) {
            s.store_div_from_scalar(1385, 1.0, 302);
        }

        if ((!(s.v[1389] != 0.0)) && (!(s.v[1391] != 0.0))) {
            s.store_mul_ad_lhs(1383, A::scale(s.ad_value(1385), (p.p207 * 1.6021918e-19)), 107);
        }

        if (!(s.v[1389] != 0.0)) {
            s.store_neg(1388, 513);
        }

        s.v[1392] = if (s.v[1388] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1389] != 0.0)) && (s.v[1392] != 0.0)) {
            s.store_square(1383, 1388);
        }

        if ((!(s.v[1389] != 0.0)) && (s.v[1392] != 0.0)) {
            s.store_mul(331, 1383, 1388);
        }

        if ((!(s.v[1389] != 0.0)) && (s.v[1392] != 0.0)) {
            s.store_offset(1386, 331, p.p212);
        }

        s.v[1393] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1393] != 0.0) {
            s.store_scalar(1403, s.v[91]);
        }

        if (s.v[1393] != 0.0) {
            s.store_div_from_scalar(1404, 1.0, 1403);
        }

        if (s.v[1393] != 0.0) {
            s.store_scalar(1460, 0.0);
        }

        if (s.v[1393] != 0.0) {
            s.store_scalar(1462, 0.0);
        }

        if (s.v[1393] != 0.0) {
            s.store_scalar(1464, 0.0);
        }

        if (s.v[1393] != 0.0) {
            s.store_neg(1396, 534);
        }

        if (s.v[1393] != 0.0) {
            s.store_mul(1397, 1396, 436);
        }

        if (s.v[1393] != 0.0) {
            s.store_add_ad_rhs(331, 1397, A::mul(s.ad_value(1396), s.ad_value(437)));
        }

        if (s.v[1393] != 0.0) {
            s.store_mul(470, 1397, 438);
        }

        if (s.v[1393] != 0.0) {
            s.store_sub(469, 1397, 470);
        }

        if (s.v[1393] != 0.0) {
            s.store_mul(468, 331, 438);
        }

        if (s.v[1393] != 0.0) {
            s.store_sub(467, 331, 468);
        }

        if ((s.v[1393] != 0.0) && (p.p24 != 0.0)) {
            s.copy_ad(521, 536);
        }

        if ((s.v[1393] != 0.0) && (p.p24 != 0.0)) {
            s.store_scalar(528, 0.0);
        }

        s.v[1473] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1474] = if (1.0 == 2.0) { 1.0 } else { 0.0 };

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1473] != 0.0)) {
            s.store_scale(522, 533, 0.5);
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1473] != 0.0)) {
            s.store_scalar(523, p.p292);
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1473] != 0.0)) {
            s.store_scalar(528, s.v[525]);
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && ((s.v[1474] != 0.0) && (!(s.v[1473] != 0.0)))) {
            s.store_scale(522, 534, 0.5);
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && ((s.v[1474] != 0.0) && (!(s.v[1473] != 0.0)))) {
            s.store_scalar(523, p.p68);
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && ((s.v[1474] != 0.0) && (!(s.v[1473] != 0.0)))) {
            s.store_scalar(528, s.v[524]);
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && ((s.v[1474] != 0.0) && (!(s.v[1473] != 0.0)))) {
            s.store_scalar(528, 1.0);
        }

        s.v[1475] = if (s.v[528] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_mul_ad_rhs(1423, 238, A::sqrt(A::div(s.ad_value(521), s.ad_value(536))));
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_scalar(1405, ((1.0 - -1.0) / 2.0));
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_scalar(1406, ((1.0 + -1.0) / 2.0));
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_add_ad(1416, A::mul(s.ad_value(461), s.ad_value(156)), A::mul(s.ad_value(462), A::sub(s.ad_value(156), s.ad_value(157))));
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_add_ad(1417, A::mul(s.ad_value(461), s.ad_value(157)), A::mul(s.ad_value(462), A::neg(s.ad_value(157))));
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_add_ad(1418, A::mul(s.ad_value(461), s.ad_value(158)), A::mul(s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_add_ad(1419, A::mul(s.ad_value(462), s.ad_value(158)), A::mul(s.ad_value(461), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_sub(1420, 1417, 1416);
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_neg(1421, 1416);
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_add_ad(1407, A::mul(s.ad_value(1405), s.ad_value(461)), A::mul(s.ad_value(1406), s.ad_value(462)));
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_add_ad(1408, A::mul(s.ad_value(1405), s.ad_value(462)), A::mul(s.ad_value(1406), s.ad_value(461)));
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_add_ad(1422, A::mul(s.ad_value(1407), s.ad_value(1418)), A::mul(s.ad_value(1408), s.ad_value(1419)));
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_offset_ad(1414, A::add(A::mul(s.ad_value(1407), s.ad_value(1421)), A::mul(s.ad_value(1408), s.ad_value(1420))), (10.0 * 2.220446049250313e-16));
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_neg(1394, 1414);
        }

        s.v[1476] = if (s.v[1394] > s.v[141]) { 1.0 } else { 0.0 };

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1476] != 0.0)) {
            s.store_sub(1395, 1394, 141);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1476] != 0.0)) {
            s.store_sub(1396, 140, 141);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1476] != 0.0)) {
            s.store_div(44, 1395, 1396);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1476] != 0.0)) {
            s.store_square(45, 44);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1476] != 0.0)) {
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
        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1476] != 0.0)) {
            s.store_square(47, 45);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1476] != 0.0)) {
            s.store_div_from_scalar_ad(1402, 1.0, A::add(A::add(A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(45)), s.ad_value(46)), s.ad_value(47)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1476] != 0.0)) {
            s.store_mul_ad_rhs(1402, 1396, A::sub_from_scalar(1.0, s.ad_value(1402)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1476] != 0.0)) {
            s.store_add(1399, 141, 1402);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1476] != 0.0))) {
            s.copy_ad(1399, 1394);
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_offset_ad(1415, A::neg(s.ad_value(1399)), (-1e-12));
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_mul(1424, 1423, 1404);
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_square(1425, 1424);
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_sub(1426, 1422, 523);
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_div(1394, 521, 230);
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_mul_ad(1427, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(s.ad_value(1394)));
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_neg(1428, 1415);
        }

        s.v[1477] = if (s.v[1426] < s.v[1428]) { 1.0 } else { 0.0 };

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_div_from_scalar_ad(1395, 1.0, A::mul(s.ad_value(225), s.ad_value(1423)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_mul(1402, 1395, 1403);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_offset_scaled(1429, 1402, (3.0 * 1.414213562373095), 2.0);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_mul_ad_lhs(1430, A::mul(A::scale(s.ad_value(1429), 8.0), s.ad_value(1429)), 1429);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_sub(1431, 237, 1427);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_mul_ad_rhs(1401, 225, A::add(s.ad_value(1426), s.ad_value(1415)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_sub_from_scalar_ad(1432, (7.0 * 1.414213562373095), A::mul(A::scale(s.ad_value(1402), 9.0), A::offset(s.ad_value(1401), (-2.0))));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_square(1433, 1432);
        }

        s.v[1478] = if (s.v[1430] < (s.v[1433] * 1e-8)) { 1.0 } else { 0.0 };

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1477] != 0.0)) && (s.v[1478] != 0.0)) {
            s.store_add_ad(1435, A::add(A::offset(s.ad_value(1432), ((-7.0) * 1.414213562373095)), A::div(A::scale(s.ad_value(1430), 0.5), s.ad_value(1432))), A::mul(A::scale(s.ad_value(1402), 9.0), A::offset(s.ad_value(1401), (-2.0))));
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1478] != 0.0))) {
            s.store_sqrt_ad(1434, A::add(s.ad_value(1430), s.ad_value(1433)));
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1477] != 0.0)) && (!(s.v[1478] != 0.0))) {
            s.store_add_ad(1435, A::offset(s.ad_value(1434), ((-7.0) * 1.414213562373095)), A::mul(A::scale(s.ad_value(1402), 9.0), A::offset(s.ad_value(1401), (-2.0))));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_powf(1436, 1435, 0.3333333333333333);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_add_ad(1437, A::add(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1402), 12.0)), A::scale(s.ad_value(1436), 2.0)), A::mul(A::scale(s.ad_value(1436), 1.414213562373095), s.ad_value(1436)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_div(1438, 1437, 1436);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_sub_ad_lhs(1439, A::mul(s.ad_value(1438), s.ad_value(227)), 1415);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_add(1395, 1439, 1415);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_div(1396, 1395, 1431);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_sqrt_ad(1397, A::offset(A::square(s.ad_value(1396)), 1.0));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_sub_ad_lhs(1440, A::div(s.ad_value(1395), s.ad_value(1397)), 1415);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_sub(1396, 1426, 1440);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_mul(459, 1403, 1396);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1477] != 0.0)) {
            s.copy_ad(458, 459);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_scalar(1438, 3.0);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_sub_ad_lhs(1441, A::div(s.ad_value(1438), s.ad_value(225)), 1415);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_exp_ad(1402, A::neg(s.ad_value(1438)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_offset_ad(1401, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1426), s.ad_value(1415))), (-1.0)), s.ad_value(1402)), 4.0), A::mul(s.ad_value(1425), s.ad_value(226))), 1.0);
        }

        s.v[1479] = if (s.v[1401] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1479] != 0.0)) {
            s.store_scalar(1401, (10.0 * 2.220446049250313e-16));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_add_ad_rhs(1441, 1426, A::mul(A::scale(A::mul(s.ad_value(1425), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1401)))));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_mul_ad_rhs(1438, 225, A::add(s.ad_value(1441), s.ad_value(1415)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_exp_ad(1402, A::neg(s.ad_value(1438)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_offset_ad(1401, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1426), s.ad_value(1415))), (-1.0)), s.ad_value(1402)), 4.0), A::mul(s.ad_value(1425), s.ad_value(226))), 1.0);
        }

        s.v[1480] = if (s.v[1401] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1480] != 0.0)) {
            s.store_scalar(1401, (10.0 * 2.220446049250313e-16));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_add_ad_rhs(1441, 1426, A::mul(A::scale(A::mul(s.ad_value(1425), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1401)))));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_mul_ad_rhs(1438, 225, A::add(s.ad_value(1441), s.ad_value(1415)));
        }

        s.v[1481] = if (s.v[1438] < 3.0) { 1.0 } else { 0.0 };

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1481] != 0.0)) {
            s.store_scalar(1442, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1481] != 0.0)) {
            s.store_scalar(1443, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1481] != 0.0)) {
            s.store_offset_ad(1444, A::div_from_scalar(1.0, A::mul(s.ad_value(225), s.ad_value(1424))), (1.0 / 1.414213562373095));
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1481] != 0.0)) {
            s.store_div_ad_lhs(1445, A::neg(A::add(s.ad_value(1426), s.ad_value(1415))), 1424);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1481] != 0.0)) {
            s.store_add_ad(1446, A::sub(A::div(A::mul(A::square(s.ad_value(1443)), s.ad_value(1443)), A::mul(A::mul(A::scale(s.ad_value(1442), 27.0), s.ad_value(1442)), s.ad_value(1442))), A::div(A::mul(s.ad_value(1443), s.ad_value(1444)), A::mul(A::scale(s.ad_value(1442), 6.0), s.ad_value(1442)))), A::div(s.ad_value(1445), A::scale(s.ad_value(1442), 2.0)));
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1481] != 0.0)) {
            s.store_div_ad(1447, A::sub(A::mul(A::scale(s.ad_value(1442), 3.0), s.ad_value(1444)), A::square(s.ad_value(1443))), A::mul(A::scale(s.ad_value(1442), 9.0), s.ad_value(1442)));
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1481] != 0.0)) {
            s.store_sqrt_ad(1398, A::add(A::square(s.ad_value(1446)), A::mul(A::square(s.ad_value(1447)), s.ad_value(1447))));
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1481] != 0.0)) {
            s.store_powf_ad(1448, A::sub(s.ad_value(1398), s.ad_value(1446)), 0.3333333333333333);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1481] != 0.0)) {
            s.store_neg_ad(1449, A::powf(A::add(s.ad_value(1446), s.ad_value(1398)), 0.3333333333333333));
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1481] != 0.0)) {
            s.store_sub_ad(1401, A::add(s.ad_value(1448), s.ad_value(1449)), A::div(s.ad_value(1443), A::scale(s.ad_value(1442), 3.0)));
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1481] != 0.0)) {
            s.store_sub_ad_lhs(1441, A::mul(s.ad_value(1401), s.ad_value(227)), 1415);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1481] != 0.0)) {
            s.store_mul_ad_rhs(1438, 225, A::add(s.ad_value(1441), s.ad_value(1415)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_offset_ad(1450, A::add(s.ad_value(1426), s.ad_value(1415)), 0.1);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_offset_ad(1457, A::exp(A::mul(s.ad_value(225), A::neg(s.ad_value(1415)))), 1e-50);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_div(1394, 230, 521);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_square(1451, 1394);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_mul(1452, 1451, 1457);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_mul(1394, 226, 1425);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_mul(1453, 225, 1450);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_add_ad(1454, A::sub(A::ln(A::add(A::mul(s.ad_value(1452), s.ad_value(1394)), A::square(s.ad_value(1453)))), A::ln(A::mul(s.ad_value(1451), s.ad_value(1394)))), A::mul(s.ad_value(225), s.ad_value(1415)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_offset_ad(44, A::sub(s.ad_value(1453), s.ad_value(1454)), (-1.0));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_scale(45, 1453, 4.0);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_scale_ad(1395, A::offset(A::div(s.ad_value(44), s.ad_value(45)), 1.0), 0.5);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_scale_ad(1396, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(44), 2.0), s.ad_value(45))), 0.5);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_sub_ad_rhs(1454, 1453, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_sub(1453, 1453, 1454);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_add_ad_rhs(1453, 1453, A::scale(s.ad_value(225), 0.1));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_add_ad(1455, A::sub(A::ln(A::add(A::mul(s.ad_value(1452), s.ad_value(1394)), A::square(s.ad_value(1453)))), A::ln(A::mul(s.ad_value(1451), s.ad_value(1394)))), A::mul(s.ad_value(225), s.ad_value(1415)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.copy_ad(1456, 1438);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_offset_ad(44, A::sub(s.ad_value(1455), s.ad_value(1456)), (-(0.0008 * 75.0)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_scale(45, 1455, (4.0 * (0.0008 * 75.0)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_scale_ad(1395, A::offset(A::div(s.ad_value(44), s.ad_value(45)), 1.0), 0.5);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_scale_ad(1396, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(44), ((2.0 * 0.0008) * 75.0)), s.ad_value(45))), 0.5);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_sub_ad_rhs(1438, 1455, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_sub_ad_lhs(1440, A::div(s.ad_value(1438), s.ad_value(225)), 1415);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_add_ad(1395, A::offset(s.ad_value(1438), (-1.0)), A::exp(A::neg(s.ad_value(1438))));
        }

        s.v[1482] = if (s.v[1395] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1482] != 0.0)) {
            s.store_scalar(1395, (10.0 * 2.220446049250313e-16));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_sqrt(1396, 1395);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_mul(458, 1423, 1396);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) {
            s.store_mul_ad_rhs(459, 1403, A::sub(s.ad_value(1426), s.ad_value(1440)));
        }

        s.v[1483] = if (p.p42 == 1.0) { 1.0 } else { 0.0 };

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) {
            s.store_exp_ad(1457, A::mul(s.ad_value(225), A::neg(s.ad_value(1415))));
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) {
            s.store_div(1394, 230, 521);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) {
            s.store_square(1451, 1394);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) {
            s.store_mul(1466, 1451, 1457);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) {
            s.store_scalar(1411, 0.0);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) {
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
        let mut assign26200_loop_guard: usize = 0;
        while {
            let assign26200_cond_e35786: f64 = (2.0 * 20.0);
            let assign26200_cond_e35788: f64 = (assign26200_cond_e35786 + 1.0);
            let assign26200_cond_e35790: f64 = if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) && (s.v[167] <= assign26200_cond_e35788)) { 1.0 } else { 0.0 };
            assign26200_cond_e35790 != 0.0
        } {
            assign26200_loop_guard += 1;
            assert!(assign26200_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) {
                s.store_scalar(1462, 0.0);
            }
            if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) {
                s.store_mul_ad_rhs(1438, 225, A::add(s.ad_value(1440), s.ad_value(1415)));
            }
            s.v[1484] = if (s.v[1438] < 5.0) { 1.0 } else { 0.0 };
            if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) && (s.v[1484] != 0.0)) {
                s.store_mul_ad(1458, A::mul(A::square(s.ad_value(1438)), s.ad_value(1438)), A::offset(A::mul(s.ad_value(1438), A::offset(A::scale(s.ad_value(1438), 0.006115288895133179), (-0.07053654284009761))), 0.29693154855771));
            }
            if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) && (s.v[1484] != 0.0)) {
                s.store_mul_ad(1459, A::square(s.ad_value(1438)), A::offset(A::mul(s.ad_value(1438), A::offset(A::scale(s.ad_value(1438), (5.0 * 0.006115288895133179)), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
            }
            if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) && (s.v[1484] != 0.0)) {
                s.store_mul_ad_lhs(1460, A::mul(s.ad_value(1466), s.ad_value(1458)), 1458);
            }
            if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) && (s.v[1484] != 0.0)) {
                s.store_mul_ad_lhs(1461, A::mul(A::scale(A::mul(s.ad_value(1466), s.ad_value(225)), 2.0), s.ad_value(1458)), 1459);
            }
            if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) && (s.v[1484] != 0.0)) {
                s.store_mul_ad_rhs(1462, 1438, A::offset(A::mul(s.ad_value(1438), A::offset(A::mul(s.ad_value(1438), A::offset(A::mul(s.ad_value(1438), A::offset(A::scale(s.ad_value(1438), 6.36964918866352e-5), (-0.00163730162779191))), 0.0178800506338833)), (-0.117851130197758))), 0.707106781186548));
            }
            if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) && (s.v[1484] != 0.0)) {
                s.store_offset_ad(1463, A::mul(s.ad_value(1438), A::offset(A::mul(s.ad_value(1438), A::offset(A::mul(s.ad_value(1438), A::offset(A::scale(s.ad_value(1438), (5.0 * 6.36964918866352e-5)), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833))), (2.0 * (-0.117851130197758)))), 0.707106781186548);
            }
            if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) && (s.v[1484] != 0.0)) {
                s.store_sqrt_ad(1464, A::offset(A::add(A::square(s.ad_value(1462)), s.ad_value(1460)), 1e-50));
            }
            if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) && (s.v[1484] != 0.0)) {
                s.store_div_ad(1465, A::add(A::mul(A::scale(A::mul(s.ad_value(225), s.ad_value(1463)), 2.0), s.ad_value(1462)), s.ad_value(1461)), A::scale(s.ad_value(1464), 2.0));
            }
            s.v[1485] = if (s.v[1438] < 80.0) { 1.0 } else { 0.0 };
            if (((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) && (!(s.v[1484] != 0.0))) && (s.v[1485] != 0.0)) {
                s.store_exp(243, 1438);
            }
            if (((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) && (!(s.v[1484] != 0.0))) && (s.v[1485] != 0.0)) {
                s.store_mul_ad_rhs(1460, 1466, A::offset(s.ad_value(243), (-1.0)));
            }
            if (((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) && (!(s.v[1484] != 0.0))) && (s.v[1485] != 0.0)) {
                s.store_mul_ad_lhs(1461, A::mul(s.ad_value(1466), s.ad_value(225)), 243);
            }
            if (((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) && (!(s.v[1484] != 0.0))) && (!(s.v[1485] != 0.0))) {
                s.store_exp_ad(1467, A::mul(s.ad_value(225), s.ad_value(1440)));
            }
            if (((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) && (!(s.v[1484] != 0.0))) && (!(s.v[1485] != 0.0))) {
                s.store_mul_ad_rhs(1460, 1451, A::sub(s.ad_value(1467), s.ad_value(1457)));
            }
            if (((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) && (!(s.v[1484] != 0.0))) && (!(s.v[1485] != 0.0))) {
                s.store_mul_ad_lhs(1461, A::mul(s.ad_value(1451), s.ad_value(225)), 1467);
            }
            if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) && (!(s.v[1484] != 0.0))) {
                s.store_sqrt_ad(1464, A::add(A::offset(s.ad_value(1438), (-1.0)), s.ad_value(1460)));
            }
            if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) && (!(s.v[1484] != 0.0))) {
                s.store_scale_ad(1465, A::div(A::add(s.ad_value(225), s.ad_value(1461)), s.ad_value(1464)), 0.5);
            }
            if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) {
                s.store_sub_ad(1468, A::sub(s.ad_value(1426), s.ad_value(1440)), A::mul(s.ad_value(1424), s.ad_value(1464)));
            }
            if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) {
                s.store_sub_from_scalar_ad(1469, (-1.0), A::mul(s.ad_value(1424), s.ad_value(1465)));
            }
            s.v[1486] = if (s.v[1411] == 1.0) { 1.0 } else { 0.0 };
            if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) && (s.v[1486] != 0.0)) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) && (!(s.v[1486] != 0.0))) {
                s.store_div_ad_lhs(494, A::neg(s.ad_value(1468)), 1469);
            }
            if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) && (!(s.v[1486] != 0.0))) {
                s.store_scale_ad(1470, A::offset({
                    if (1.0 >= ((s.v[1440]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1440))
                    }
                }, 1.0), (0.5 * 0.1));
            }
            s.v[1487] = if (((s.v[494]) as f64).abs() > s.v[1470]) { 1.0 } else { 0.0 };
            if (((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) && (!(s.v[1486] != 0.0))) && (s.v[1487] != 0.0)) {
                s.store_scale(494, 1470, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) && (!(s.v[1486] != 0.0))) {
                s.store_add(1440, 1440, 494);
            }
            s.v[1488] = if ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1468]) as f64).abs() <= 1e-8)) { 1.0 } else { 0.0 };
            if (((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) && (!(s.v[1486] != 0.0))) && (s.v[1488] != 0.0)) {
                s.store_scalar(1411, 1.0);
            }
            if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.v[1490] = if (s.v[1438] < 5.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) && (s.v[1490] != 0.0)) {
            s.store_offset_ad(1471, A::square(s.ad_value(1462)), (10.0 * 2.220446049250313e-16));
        }

        if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) && (s.v[1490] != 0.0)) {
            s.store_offset(1472, 1462, (10.0 * 2.220446049250313e-16));
        }

        if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) && (!(s.v[1490] != 0.0))) {
            s.store_offset(1471, 1438, (-1.0));
        }

        if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) && (!(s.v[1490] != 0.0))) {
            s.store_sqrt(1472, 1471);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) {
            s.store_mul(458, 1423, 1472);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) {
            s.store_div_from_scalar_ad(1395, 1.0, A::add(s.ad_value(1464), s.ad_value(1472)));
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) {
            s.store_mul_ad_lhs(460, A::mul(s.ad_value(1423), s.ad_value(1460)), 1395);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1477] != 0.0))) && (s.v[1483] != 0.0)) {
            s.store_add(459, 458, 460);
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_sub(460, 459, 458);
        }

        s.v[1492] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1493] = if (1.0 == 2.0) { 1.0 } else { 0.0 };

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1492] != 0.0)) && (s.v[1405] != 0.0)) {
            s.store_mul_ad_lhs(463, A::neg(s.ad_value(522)), 459);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1492] != 0.0)) && (s.v[1405] != 0.0)) {
            s.store_mul_ad_lhs(465, A::neg(s.ad_value(522)), 460);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1492] != 0.0)) && (s.v[1406] != 0.0)) {
            s.store_mul_ad_lhs(464, A::neg(s.ad_value(522)), 459);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1492] != 0.0)) && (s.v[1406] != 0.0)) {
            s.store_mul_ad_lhs(466, A::neg(s.ad_value(522)), 460);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && ((s.v[1493] != 0.0) && (!(s.v[1492] != 0.0)))) && (s.v[1405] != 0.0)) {
            s.store_mul_ad_lhs(467, A::neg(s.ad_value(522)), 459);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && ((s.v[1493] != 0.0) && (!(s.v[1492] != 0.0)))) && (s.v[1405] != 0.0)) {
            s.store_mul_ad_lhs(469, A::neg(s.ad_value(522)), 460);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && ((s.v[1493] != 0.0) && (!(s.v[1492] != 0.0)))) && (s.v[1406] != 0.0)) {
            s.store_mul_ad_lhs(468, A::neg(s.ad_value(522)), 459);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && ((s.v[1493] != 0.0) && (!(s.v[1492] != 0.0)))) && (s.v[1406] != 0.0)) {
            s.store_mul_ad_lhs(470, A::neg(s.ad_value(522)), 460);
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_scalar(1405, ((1.0 - 1.0) / 2.0));
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_scalar(1406, ((1.0 + 1.0) / 2.0));
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_add_ad(1416, A::mul(s.ad_value(461), s.ad_value(156)), A::mul(s.ad_value(462), A::sub(s.ad_value(156), s.ad_value(157))));
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_add_ad(1417, A::mul(s.ad_value(461), s.ad_value(157)), A::mul(s.ad_value(462), A::neg(s.ad_value(157))));
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_add_ad(1418, A::mul(s.ad_value(461), s.ad_value(158)), A::mul(s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_add_ad(1419, A::mul(s.ad_value(462), s.ad_value(158)), A::mul(s.ad_value(461), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_sub(1420, 1417, 1416);
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_neg(1421, 1416);
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_add_ad(1407, A::mul(s.ad_value(1405), s.ad_value(461)), A::mul(s.ad_value(1406), s.ad_value(462)));
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_add_ad(1408, A::mul(s.ad_value(1405), s.ad_value(462)), A::mul(s.ad_value(1406), s.ad_value(461)));
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_add_ad(1422, A::mul(s.ad_value(1407), s.ad_value(1418)), A::mul(s.ad_value(1408), s.ad_value(1419)));
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_offset_ad(1414, A::add(A::mul(s.ad_value(1407), s.ad_value(1421)), A::mul(s.ad_value(1408), s.ad_value(1420))), (10.0 * 2.220446049250313e-16));
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_neg(1394, 1414);
        }

        s.v[1494] = if (s.v[1394] > s.v[141]) { 1.0 } else { 0.0 };

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1494] != 0.0)) {
            s.store_sub(1395, 1394, 141);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1494] != 0.0)) {
            s.store_sub(1396, 140, 141);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1494] != 0.0)) {
            s.store_div(44, 1395, 1396);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1494] != 0.0)) {
            s.store_square(45, 44);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1494] != 0.0)) {
            s.store_mul(46, 45, 44);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1494] != 0.0)) {
            s.store_square(47, 45);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1494] != 0.0)) {
            s.store_div_from_scalar_ad(1402, 1.0, A::add(A::add(A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(45)), s.ad_value(46)), s.ad_value(47)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1494] != 0.0)) {
            s.store_mul_ad_rhs(1402, 1396, A::sub_from_scalar(1.0, s.ad_value(1402)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1494] != 0.0)) {
            s.store_add(1399, 141, 1402);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1494] != 0.0))) {
            s.copy_ad(1399, 1394);
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_offset_ad(1415, A::neg(s.ad_value(1399)), (-1e-12));
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_mul(1424, 1423, 1404);
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_square(1425, 1424);
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_sub(1426, 1422, 523);
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_div(1394, 521, 230);
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_mul_ad(1427, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(s.ad_value(1394)));
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_neg(1428, 1415);
        }

        s.v[1495] = if (s.v[1426] < s.v[1428]) { 1.0 } else { 0.0 };

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1495] != 0.0)) {
            s.store_div_from_scalar_ad(1395, 1.0, A::mul(s.ad_value(225), s.ad_value(1423)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1495] != 0.0)) {
            s.store_mul(1402, 1395, 1403);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1495] != 0.0)) {
            s.store_offset_scaled(1429, 1402, (3.0 * 1.414213562373095), 2.0);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1495] != 0.0)) {
            s.store_mul_ad_lhs(1430, A::mul(A::scale(s.ad_value(1429), 8.0), s.ad_value(1429)), 1429);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1495] != 0.0)) {
            s.store_sub(1431, 237, 1427);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1495] != 0.0)) {
            s.store_mul_ad_rhs(1401, 225, A::add(s.ad_value(1426), s.ad_value(1415)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1495] != 0.0)) {
            s.store_sub_from_scalar_ad(1432, (7.0 * 1.414213562373095), A::mul(A::scale(s.ad_value(1402), 9.0), A::offset(s.ad_value(1401), (-2.0))));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1495] != 0.0)) {
            s.store_square(1433, 1432);
        }

        s.v[1496] = if (s.v[1430] < (s.v[1433] * 1e-8)) { 1.0 } else { 0.0 };

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1495] != 0.0)) && (s.v[1496] != 0.0)) {
            s.store_add_ad(1435, A::add(A::offset(s.ad_value(1432), ((-7.0) * 1.414213562373095)), A::div(A::scale(s.ad_value(1430), 0.5), s.ad_value(1432))), A::mul(A::scale(s.ad_value(1402), 9.0), A::offset(s.ad_value(1401), (-2.0))));
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1495] != 0.0)) && (!(s.v[1496] != 0.0))) {
            s.store_sqrt_ad(1434, A::add(s.ad_value(1430), s.ad_value(1433)));
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1495] != 0.0)) && (!(s.v[1496] != 0.0))) {
            s.store_add_ad(1435, A::offset(s.ad_value(1434), ((-7.0) * 1.414213562373095)), A::mul(A::scale(s.ad_value(1402), 9.0), A::offset(s.ad_value(1401), (-2.0))));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1495] != 0.0)) {
            s.store_powf(1436, 1435, 0.3333333333333333);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1495] != 0.0)) {
            s.store_add_ad(1437, A::add(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1402), 12.0)), A::scale(s.ad_value(1436), 2.0)), A::mul(A::scale(s.ad_value(1436), 1.414213562373095), s.ad_value(1436)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1495] != 0.0)) {
            s.store_div(1438, 1437, 1436);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1495] != 0.0)) {
            s.store_sub_ad_lhs(1439, A::mul(s.ad_value(1438), s.ad_value(227)), 1415);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1495] != 0.0)) {
            s.store_add(1395, 1439, 1415);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1495] != 0.0)) {
            s.store_div(1396, 1395, 1431);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1495] != 0.0)) {
            s.store_sqrt_ad(1397, A::offset(A::square(s.ad_value(1396)), 1.0));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1495] != 0.0)) {
            s.store_sub_ad_lhs(1440, A::div(s.ad_value(1395), s.ad_value(1397)), 1415);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1495] != 0.0)) {
            s.store_sub(1396, 1426, 1440);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1495] != 0.0)) {
            s.store_mul(459, 1403, 1396);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1495] != 0.0)) {
            s.copy_ad(458, 459);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_scalar(1438, 3.0);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_sub_ad_lhs(1441, A::div(s.ad_value(1438), s.ad_value(225)), 1415);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_exp_ad(1402, A::neg(s.ad_value(1438)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_offset_ad(1401, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1426), s.ad_value(1415))), (-1.0)), s.ad_value(1402)), 4.0), A::mul(s.ad_value(1425), s.ad_value(226))), 1.0);
        }

        s.v[1497] = if (s.v[1401] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1497] != 0.0)) {
            s.store_scalar(1401, (10.0 * 2.220446049250313e-16));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_add_ad_rhs(1441, 1426, A::mul(A::scale(A::mul(s.ad_value(1425), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1401)))));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_mul_ad_rhs(1438, 225, A::add(s.ad_value(1441), s.ad_value(1415)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_exp_ad(1402, A::neg(s.ad_value(1438)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_offset_ad(1401, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1426), s.ad_value(1415))), (-1.0)), s.ad_value(1402)), 4.0), A::mul(s.ad_value(1425), s.ad_value(226))), 1.0);
        }

        s.v[1498] = if (s.v[1401] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1498] != 0.0)) {
            s.store_scalar(1401, (10.0 * 2.220446049250313e-16));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_add_ad_rhs(1441, 1426, A::mul(A::scale(A::mul(s.ad_value(1425), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1401)))));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_mul_ad_rhs(1438, 225, A::add(s.ad_value(1441), s.ad_value(1415)));
        }

        s.v[1499] = if (s.v[1438] < 3.0) { 1.0 } else { 0.0 };

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1499] != 0.0)) {
            s.store_scalar(1442, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1499] != 0.0)) {
            s.store_scalar(1443, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1499] != 0.0)) {
            s.store_offset_ad(1444, A::div_from_scalar(1.0, A::mul(s.ad_value(225), s.ad_value(1424))), (1.0 / 1.414213562373095));
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1499] != 0.0)) {
            s.store_div_ad_lhs(1445, A::neg(A::add(s.ad_value(1426), s.ad_value(1415))), 1424);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1499] != 0.0)) {
            s.store_add_ad(1446, A::sub(A::div(A::mul(A::square(s.ad_value(1443)), s.ad_value(1443)), A::mul(A::mul(A::scale(s.ad_value(1442), 27.0), s.ad_value(1442)), s.ad_value(1442))), A::div(A::mul(s.ad_value(1443), s.ad_value(1444)), A::mul(A::scale(s.ad_value(1442), 6.0), s.ad_value(1442)))), A::div(s.ad_value(1445), A::scale(s.ad_value(1442), 2.0)));
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1499] != 0.0)) {
            s.store_div_ad(1447, A::sub(A::mul(A::scale(s.ad_value(1442), 3.0), s.ad_value(1444)), A::square(s.ad_value(1443))), A::mul(A::scale(s.ad_value(1442), 9.0), s.ad_value(1442)));
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1499] != 0.0)) {
            s.store_sqrt_ad(1398, A::add(A::square(s.ad_value(1446)), A::mul(A::square(s.ad_value(1447)), s.ad_value(1447))));
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1499] != 0.0)) {
            s.store_powf_ad(1448, A::sub(s.ad_value(1398), s.ad_value(1446)), 0.3333333333333333);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1499] != 0.0)) {
            s.store_neg_ad(1449, A::powf(A::add(s.ad_value(1446), s.ad_value(1398)), 0.3333333333333333));
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1499] != 0.0)) {
            s.store_sub_ad(1401, A::add(s.ad_value(1448), s.ad_value(1449)), A::div(s.ad_value(1443), A::scale(s.ad_value(1442), 3.0)));
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1499] != 0.0)) {
            s.store_sub_ad_lhs(1441, A::mul(s.ad_value(1401), s.ad_value(227)), 1415);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1499] != 0.0)) {
            s.store_mul_ad_rhs(1438, 225, A::add(s.ad_value(1441), s.ad_value(1415)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_offset_ad(1450, A::add(s.ad_value(1426), s.ad_value(1415)), 0.1);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_offset_ad(1457, A::exp(A::mul(s.ad_value(225), A::neg(s.ad_value(1415)))), 1e-50);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_div(1394, 230, 521);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_square(1451, 1394);
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
        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_mul(1452, 1451, 1457);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_mul(1394, 226, 1425);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_mul(1453, 225, 1450);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_add_ad(1454, A::sub(A::ln(A::add(A::mul(s.ad_value(1452), s.ad_value(1394)), A::square(s.ad_value(1453)))), A::ln(A::mul(s.ad_value(1451), s.ad_value(1394)))), A::mul(s.ad_value(225), s.ad_value(1415)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_offset_ad(44, A::sub(s.ad_value(1453), s.ad_value(1454)), (-1.0));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_scale(45, 1453, 4.0);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_scale_ad(1395, A::offset(A::div(s.ad_value(44), s.ad_value(45)), 1.0), 0.5);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_scale_ad(1396, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(44), 2.0), s.ad_value(45))), 0.5);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_sub_ad_rhs(1454, 1453, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_sub(1453, 1453, 1454);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_add_ad_rhs(1453, 1453, A::scale(s.ad_value(225), 0.1));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_add_ad(1455, A::sub(A::ln(A::add(A::mul(s.ad_value(1452), s.ad_value(1394)), A::square(s.ad_value(1453)))), A::ln(A::mul(s.ad_value(1451), s.ad_value(1394)))), A::mul(s.ad_value(225), s.ad_value(1415)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.copy_ad(1456, 1438);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_offset_ad(44, A::sub(s.ad_value(1455), s.ad_value(1456)), (-(0.0008 * 75.0)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_scale(45, 1455, (4.0 * (0.0008 * 75.0)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_scale_ad(1395, A::offset(A::div(s.ad_value(44), s.ad_value(45)), 1.0), 0.5);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_scale_ad(1396, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(44), ((2.0 * 0.0008) * 75.0)), s.ad_value(45))), 0.5);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_sub_ad_rhs(1438, 1455, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_sub_ad_lhs(1440, A::div(s.ad_value(1438), s.ad_value(225)), 1415);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_add_ad(1395, A::offset(s.ad_value(1438), (-1.0)), A::exp(A::neg(s.ad_value(1438))));
        }

        s.v[1500] = if (s.v[1395] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1500] != 0.0)) {
            s.store_scalar(1395, (10.0 * 2.220446049250313e-16));
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_sqrt(1396, 1395);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_mul(458, 1423, 1396);
        }

        if ((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_mul_ad_rhs(459, 1403, A::sub(s.ad_value(1426), s.ad_value(1440)));
        }

        s.v[1501] = if (p.p42 == 1.0) { 1.0 } else { 0.0 };

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) {
            s.store_exp_ad(1457, A::mul(s.ad_value(225), A::neg(s.ad_value(1415))));
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) {
            s.store_div(1394, 230, 521);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) {
            s.store_square(1451, 1394);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) {
            s.store_mul(1466, 1451, 1457);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) {
            s.store_scalar(1411, 0.0);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) {
            s.store_scalar(167, 1.0);
        }

        let mut assign27750_loop_guard: usize = 0;
        while {
            let assign27750_cond_e38729: f64 = (2.0 * 20.0);
            let assign27750_cond_e38731: f64 = (assign27750_cond_e38729 + 1.0);
            let assign27750_cond_e38733: f64 = if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) && (s.v[167] <= assign27750_cond_e38731)) { 1.0 } else { 0.0 };
            assign27750_cond_e38733 != 0.0
        } {
            assign27750_loop_guard += 1;
            assert!(assign27750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) {
                s.store_scalar(1462, 0.0);
            }
            if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) {
                s.store_mul_ad_rhs(1438, 225, A::add(s.ad_value(1440), s.ad_value(1415)));
            }
            s.v[1502] = if (s.v[1438] < 5.0) { 1.0 } else { 0.0 };
            if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) && (s.v[1502] != 0.0)) {
                s.store_mul_ad(1458, A::mul(A::square(s.ad_value(1438)), s.ad_value(1438)), A::offset(A::mul(s.ad_value(1438), A::offset(A::scale(s.ad_value(1438), 0.006115288895133179), (-0.07053654284009761))), 0.29693154855771));
            }
            if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) && (s.v[1502] != 0.0)) {
                s.store_mul_ad(1459, A::square(s.ad_value(1438)), A::offset(A::mul(s.ad_value(1438), A::offset(A::scale(s.ad_value(1438), (5.0 * 0.006115288895133179)), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
            }
            if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) && (s.v[1502] != 0.0)) {
                s.store_mul_ad_lhs(1460, A::mul(s.ad_value(1466), s.ad_value(1458)), 1458);
            }
            if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) && (s.v[1502] != 0.0)) {
                s.store_mul_ad_lhs(1461, A::mul(A::scale(A::mul(s.ad_value(1466), s.ad_value(225)), 2.0), s.ad_value(1458)), 1459);
            }
            if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) && (s.v[1502] != 0.0)) {
                s.store_mul_ad_rhs(1462, 1438, A::offset(A::mul(s.ad_value(1438), A::offset(A::mul(s.ad_value(1438), A::offset(A::mul(s.ad_value(1438), A::offset(A::scale(s.ad_value(1438), 6.36964918866352e-5), (-0.00163730162779191))), 0.0178800506338833)), (-0.117851130197758))), 0.707106781186548));
            }
            if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) && (s.v[1502] != 0.0)) {
                s.store_offset_ad(1463, A::mul(s.ad_value(1438), A::offset(A::mul(s.ad_value(1438), A::offset(A::mul(s.ad_value(1438), A::offset(A::scale(s.ad_value(1438), (5.0 * 6.36964918866352e-5)), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833))), (2.0 * (-0.117851130197758)))), 0.707106781186548);
            }
            if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) && (s.v[1502] != 0.0)) {
                s.store_sqrt_ad(1464, A::offset(A::add(A::square(s.ad_value(1462)), s.ad_value(1460)), 1e-50));
            }
            if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) && (s.v[1502] != 0.0)) {
                s.store_div_ad(1465, A::add(A::mul(A::scale(A::mul(s.ad_value(225), s.ad_value(1463)), 2.0), s.ad_value(1462)), s.ad_value(1461)), A::scale(s.ad_value(1464), 2.0));
            }
            s.v[1503] = if (s.v[1438] < 80.0) { 1.0 } else { 0.0 };
            if (((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) && (!(s.v[1502] != 0.0))) && (s.v[1503] != 0.0)) {
                s.store_exp(243, 1438);
            }
            if (((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) && (!(s.v[1502] != 0.0))) && (s.v[1503] != 0.0)) {
                s.store_mul_ad_rhs(1460, 1466, A::offset(s.ad_value(243), (-1.0)));
            }
            if (((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) && (!(s.v[1502] != 0.0))) && (s.v[1503] != 0.0)) {
                s.store_mul_ad_lhs(1461, A::mul(s.ad_value(1466), s.ad_value(225)), 243);
            }
            if (((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) && (!(s.v[1502] != 0.0))) && (!(s.v[1503] != 0.0))) {
                s.store_exp_ad(1467, A::mul(s.ad_value(225), s.ad_value(1440)));
            }
            if (((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) && (!(s.v[1502] != 0.0))) && (!(s.v[1503] != 0.0))) {
                s.store_mul_ad_rhs(1460, 1451, A::sub(s.ad_value(1467), s.ad_value(1457)));
            }
            if (((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) && (!(s.v[1502] != 0.0))) && (!(s.v[1503] != 0.0))) {
                s.store_mul_ad_lhs(1461, A::mul(s.ad_value(1451), s.ad_value(225)), 1467);
            }
            if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) && (!(s.v[1502] != 0.0))) {
                s.store_sqrt_ad(1464, A::add(A::offset(s.ad_value(1438), (-1.0)), s.ad_value(1460)));
            }
            if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) && (!(s.v[1502] != 0.0))) {
                s.store_scale_ad(1465, A::div(A::add(s.ad_value(225), s.ad_value(1461)), s.ad_value(1464)), 0.5);
            }
            if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) {
                s.store_sub_ad(1468, A::sub(s.ad_value(1426), s.ad_value(1440)), A::mul(s.ad_value(1424), s.ad_value(1464)));
            }
            if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) {
                s.store_sub_from_scalar_ad(1469, (-1.0), A::mul(s.ad_value(1424), s.ad_value(1465)));
            }
            s.v[1504] = if (s.v[1411] == 1.0) { 1.0 } else { 0.0 };
            if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) && (s.v[1504] != 0.0)) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) && (!(s.v[1504] != 0.0))) {
                s.store_div_ad_lhs(494, A::neg(s.ad_value(1468)), 1469);
            }
            if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) && (!(s.v[1504] != 0.0))) {
                s.store_scale_ad(1470, A::offset({
                    if (1.0 >= ((s.v[1440]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1440))
                    }
                }, 1.0), (0.5 * 0.1));
            }
            s.v[1505] = if (((s.v[494]) as f64).abs() > s.v[1470]) { 1.0 } else { 0.0 };
            if (((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) && (!(s.v[1504] != 0.0))) && (s.v[1505] != 0.0)) {
                s.store_scale(494, 1470, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) && (!(s.v[1504] != 0.0))) {
                s.store_add(1440, 1440, 494);
            }
            s.v[1506] = if ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1468]) as f64).abs() <= 1e-8)) { 1.0 } else { 0.0 };
            if (((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) && (!(s.v[1504] != 0.0))) && (s.v[1506] != 0.0)) {
                s.store_scalar(1411, 1.0);
            }
            if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.v[1508] = if (s.v[1438] < 5.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) && (s.v[1508] != 0.0)) {
            s.store_offset_ad(1471, A::square(s.ad_value(1462)), (10.0 * 2.220446049250313e-16));
        }

        if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) && (s.v[1508] != 0.0)) {
            s.store_offset(1472, 1462, (10.0 * 2.220446049250313e-16));
        }

        if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) && (!(s.v[1508] != 0.0))) {
            s.store_offset(1471, 1438, (-1.0));
        }

        if ((((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) && (!(s.v[1508] != 0.0))) {
            s.store_sqrt(1472, 1471);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) {
            s.store_mul(458, 1423, 1472);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) {
            s.store_div_from_scalar_ad(1395, 1.0, A::add(s.ad_value(1464), s.ad_value(1472)));
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) {
            s.store_mul_ad_lhs(460, A::mul(s.ad_value(1423), s.ad_value(1460)), 1395);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (!(s.v[1495] != 0.0))) && (s.v[1501] != 0.0)) {
            s.store_add(459, 458, 460);
        }

        if (((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) {
            s.store_sub(460, 459, 458);
        }

        s.v[1510] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1511] = if (1.0 == 2.0) { 1.0 } else { 0.0 };

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1510] != 0.0)) && (s.v[1405] != 0.0)) {
            s.store_mul_ad_lhs(463, A::neg(s.ad_value(522)), 459);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1510] != 0.0)) && (s.v[1405] != 0.0)) {
            s.store_mul_ad_lhs(465, A::neg(s.ad_value(522)), 460);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1510] != 0.0)) && (s.v[1406] != 0.0)) {
            s.store_mul_ad_lhs(464, A::neg(s.ad_value(522)), 459);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && (s.v[1510] != 0.0)) && (s.v[1406] != 0.0)) {
            s.store_mul_ad_lhs(466, A::neg(s.ad_value(522)), 460);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && ((s.v[1511] != 0.0) && (!(s.v[1510] != 0.0)))) && (s.v[1405] != 0.0)) {
            s.store_mul_ad_lhs(467, A::neg(s.ad_value(522)), 459);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && ((s.v[1511] != 0.0) && (!(s.v[1510] != 0.0)))) && (s.v[1405] != 0.0)) {
            s.store_mul_ad_lhs(469, A::neg(s.ad_value(522)), 460);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && ((s.v[1511] != 0.0) && (!(s.v[1510] != 0.0)))) && (s.v[1406] != 0.0)) {
            s.store_mul_ad_lhs(468, A::neg(s.ad_value(522)), 459);
        }

        if (((((s.v[1393] != 0.0) && (p.p24 != 0.0)) && (s.v[1475] != 0.0)) && ((s.v[1511] != 0.0) && (!(s.v[1510] != 0.0)))) && (s.v[1406] != 0.0)) {
            s.store_mul_ad_lhs(470, A::neg(s.ad_value(522)), 460);
        }

        s.v[317] = p.p189;

        s.v[1514] = if (s.v[145] != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1514] != 0.0) {
            s.store_add(1513, 157, 161);
        }

        if (s.v[1514] != 0.0) {
            s.store_add_ad(314, A::scale(s.ad_value(1513), s.v[317]), A::scale(s.ad_value(162), (1.0 - s.v[317])));
        }

        s.v[1515] = if (p.p64 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1514] != 0.0) && (s.v[1515] != 0.0)) {
            s.store_scalar(315, 0.0);
        }

        s.v[1516] = if (s.v[314] > ((s.v[161] + s.v[157]) - (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if ((s.v[1514] != 0.0) && (s.v[1516] != 0.0)) {
            s.store_offset_ad(314, A::add(s.ad_value(161), s.ad_value(157)), (-(10.0 * 2.220446049250313e-16)));
        }

        s.v[1517] = if (p.p64 != 0.0) { 1.0 } else { 0.0 };

        s.v[1518] = if (s.v[246] < 1e-15) { 1.0 } else { 0.0 };

        if (((!(s.v[1514] != 0.0)) && (s.v[1517] != 0.0)) && (s.v[1518] != 0.0)) {
            s.store_scalar(315, 0.0);
        }

        if (((!(s.v[1514] != 0.0)) && (s.v[1517] != 0.0)) && (!(s.v[1518] != 0.0))) {
            s.store_scale(1512, 227, 1.0 / (s.v[97]));
        }

        if (((!(s.v[1514] != 0.0)) && (s.v[1517] != 0.0)) && (!(s.v[1518] != 0.0))) {
            s.store_div_from_scalar(1513, 1.0, 244);
        }

        if (((!(s.v[1514] != 0.0)) && (s.v[1517] != 0.0)) && (!(s.v[1518] != 0.0))) {
            s.store_mul_ad_lhs(315, A::mul(s.ad_value(246), s.ad_value(1512)), 1513);
        }

        s.v[1530] = s.v[91];

        s.v[1531] = (1.0 / s.v[1530]);

        s.v[1551] = 0.0;

        s.v[1591] = 0.0;

        s.v[1589] = 0.0;

        s.v[1593] = 0.0;

        s.v[1602] = if ((p.p29 >= 1.0) && (p.p188 > 0.0)) { 1.0 } else { 0.0 };

        if ((p.p24 != 0.0) && (s.v[1602] != 0.0)) {
            s.store_scalar(1533, p.p171);
        }

        if ((p.p24 != 0.0) && (s.v[1602] != 0.0)) {
            s.store_scalar(1534, p.p172);
        }

        if ((p.p24 != 0.0) && (s.v[1602] != 0.0)) {
            s.copy_ad(1535, 158);
        }

        if ((p.p24 != 0.0) && (s.v[1602] != 0.0)) {
            s.store_scalar(1532, p.p188);
        }

        s.v[1603] = if ((s.v[69] == 0.0) && (p.p188 > 0.0)) { 1.0 } else { 0.0 };

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (s.v[1603] != 0.0)) {
            s.store_ad(1520, &{
                if (p.p43 == 1.0) {
                    A::scale(s.ad_value(287), s.v[1530])
                } else {
                    A::scale(s.ad_value(108), s.v[1530])
                }
            });
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (s.v[1603] != 0.0)) {
            s.store_mul_ad(1523, A::mul(s.ad_value(1533), s.ad_value(1520)), A::add(s.ad_value(1534), s.ad_value(1535)));
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (s.v[1603] != 0.0)) {
            s.store_mul(1524, 1532, 1520);
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (s.v[1603] != 0.0)) {
            s.copy_ad(1528, 161);
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (s.v[1603] != 0.0)) {
            s.store_sub_from_scalar(1525, 1.2, 1528);
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (s.v[1603] != 0.0)) {
            s.store_sub_ad(267, A::mul(s.ad_value(158), s.ad_value(1524)), A::mul(s.ad_value(1525), s.ad_value(1523)));
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (s.v[1603] != 0.0)) {
            s.store_mul_ad(1523, A::mul(s.ad_value(1533), s.ad_value(1520)), A::sub(A::add(s.ad_value(1534), s.ad_value(1535)), s.ad_value(157)));
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (s.v[1603] != 0.0)) {
            s.store_sub(1528, 162, 157);
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (s.v[1603] != 0.0)) {
            s.store_sub_from_scalar(1525, 1.2, 1528);
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (s.v[1603] != 0.0)) {
            s.store_sub_ad(268, A::mul(A::sub(s.ad_value(158), s.ad_value(157)), s.ad_value(1524)), A::mul(s.ad_value(1523), s.ad_value(1525)));
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) {
            s.store_mul_ad_rhs(1552, 238, A::sqrt(A::div_from_scalar(s.v[69], s.ad_value(536))));
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) {
            s.store_scalar(1536, ((1.0 - -1.0) / 2.0));
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) {
            s.store_scalar(1537, ((1.0 + -1.0) / 2.0));
        }

        s.v[1604] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1604] != 0.0)) {
            s.store_add_ad(1546, A::mul(s.ad_value(461), s.ad_value(156)), A::mul(s.ad_value(462), A::sub(s.ad_value(156), s.ad_value(157))));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1604] != 0.0)) {
            s.store_add_ad(1547, A::mul(s.ad_value(461), s.ad_value(157)), A::mul(s.ad_value(462), A::neg(s.ad_value(157))));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1604] != 0.0)) {
            s.store_add_ad(1548, A::mul(s.ad_value(461), s.ad_value(158)), A::mul(s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1604] != 0.0)) {
            s.store_sub(1549, 1547, 1546);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1604] != 0.0)) {
            s.store_sub(1551, 1548, 1546);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1604] != 0.0)) {
            s.store_neg(1550, 1546);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1604] != 0.0)) {
            s.store_add_ad(1538, A::mul(s.ad_value(1536), s.ad_value(461)), A::mul(s.ad_value(1537), s.ad_value(462)));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1604] != 0.0)) {
            s.store_add_ad(1539, A::mul(s.ad_value(1536), s.ad_value(462)), A::mul(s.ad_value(1537), s.ad_value(461)));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1604] != 0.0)) {
            s.store_offset_ad(1544, A::add(A::mul(s.ad_value(1538), s.ad_value(1550)), A::mul(s.ad_value(1539), s.ad_value(1549))), (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1604] != 0.0))) {
            s.store_add_ad(1538, A::mul(s.ad_value(1536), s.ad_value(461)), A::mul(s.ad_value(1537), s.ad_value(462)));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1604] != 0.0))) {
            s.store_add_ad(1539, A::mul(s.ad_value(1536), s.ad_value(462)), A::mul(s.ad_value(1537), s.ad_value(461)));
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
        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1604] != 0.0))) && (s.v[1536] != 0.0)) {
            s.store_add_ad(1551, A::mul(s.ad_value(461), s.ad_value(158)), A::mul(s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1604] != 0.0))) && (s.v[1537] != 0.0)) {
            s.store_add_ad(1551, A::mul(s.ad_value(462), s.ad_value(158)), A::mul(s.ad_value(461), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1604] != 0.0))) {
            s.store_scalar(1544, 0.0);
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) {
            s.store_neg(1519, 1544);
        }

        s.v[1605] = if (s.v[1519] > s.v[141]) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1605] != 0.0)) {
            s.store_sub(1520, 1519, 141);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1605] != 0.0)) {
            s.store_sub(1521, 140, 141);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1605] != 0.0)) {
            s.store_div(44, 1520, 1521);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1605] != 0.0)) {
            s.store_square(45, 44);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1605] != 0.0)) {
            s.store_mul(46, 45, 44);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1605] != 0.0)) {
            s.store_square(47, 45);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1605] != 0.0)) {
            s.store_div_from_scalar_ad(1529, 1.0, A::add(A::add(A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(45)), s.ad_value(46)), s.ad_value(47)));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1605] != 0.0)) {
            s.store_mul_ad_rhs(1529, 1521, A::sub_from_scalar(1.0, s.ad_value(1529)));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1605] != 0.0)) {
            s.store_add(1526, 141, 1529);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1605] != 0.0))) {
            s.copy_ad(1526, 1519);
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) {
            s.store_offset_ad(1545, A::neg(s.ad_value(1526)), (-1e-12));
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) {
            s.store_scale(1553, 1552, s.v[1531]);
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) {
            s.store_square(1554, 1553);
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) {
            s.store_sub_from_scalar(1555, s.v[82], 1551);
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) {
            s.store_div_from_scalar(1519, s.v[69], 230);
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) {
            s.store_mul_ad(1556, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(s.ad_value(1519)));
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) {
            s.store_neg(1557, 1545);
        }

        s.v[1606] = if (s.v[1555] < s.v[1557]) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1606] != 0.0)) {
            s.store_div_from_scalar_ad(1520, 1.0, A::mul(s.ad_value(225), s.ad_value(1552)));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1606] != 0.0)) {
            s.store_scale(1529, 1520, s.v[1530]);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1606] != 0.0)) {
            s.store_offset_scaled(1558, 1529, (3.0 * 1.414213562373095), 2.0);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1606] != 0.0)) {
            s.store_mul_ad_lhs(1559, A::mul(A::scale(s.ad_value(1558), 8.0), s.ad_value(1558)), 1558);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1606] != 0.0)) {
            s.store_sub(1560, 237, 1556);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1606] != 0.0)) {
            s.store_mul_ad_rhs(1528, 225, A::add(s.ad_value(1555), s.ad_value(1545)));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1606] != 0.0)) {
            s.store_sub_from_scalar_ad(1561, (7.0 * 1.414213562373095), A::mul(A::scale(s.ad_value(1529), 9.0), A::offset(s.ad_value(1528), (-2.0))));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1606] != 0.0)) {
            s.store_square(1562, 1561);
        }

        s.v[1607] = if (s.v[1559] < (s.v[1562] * 1e-8)) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1606] != 0.0)) && (s.v[1607] != 0.0)) {
            s.store_add_ad(1564, A::add(A::offset(s.ad_value(1561), ((-7.0) * 1.414213562373095)), A::div(A::scale(s.ad_value(1559), 0.5), s.ad_value(1561))), A::mul(A::scale(s.ad_value(1529), 9.0), A::offset(s.ad_value(1528), (-2.0))));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) {
            s.store_sqrt_ad(1563, A::add(s.ad_value(1559), s.ad_value(1562)));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) {
            s.store_add_ad(1564, A::offset(s.ad_value(1563), ((-7.0) * 1.414213562373095)), A::mul(A::scale(s.ad_value(1529), 9.0), A::offset(s.ad_value(1528), (-2.0))));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1606] != 0.0)) {
            s.store_powf(1565, 1564, 0.3333333333333333);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1606] != 0.0)) {
            s.store_add_ad(1566, A::add(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1529), 12.0)), A::scale(s.ad_value(1565), 2.0)), A::mul(A::scale(s.ad_value(1565), 1.414213562373095), s.ad_value(1565)));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1606] != 0.0)) {
            s.store_div(1567, 1566, 1565);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1606] != 0.0)) {
            s.store_sub_ad_lhs(1568, A::mul(s.ad_value(1567), s.ad_value(227)), 1545);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1606] != 0.0)) {
            s.store_add(1520, 1568, 1545);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1606] != 0.0)) {
            s.store_div(1521, 1520, 1560);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1606] != 0.0)) {
            s.store_sqrt_ad(1522, A::offset(A::square(s.ad_value(1521)), 1.0));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1606] != 0.0)) {
            s.store_sub_ad_lhs(1569, A::div(s.ad_value(1520), s.ad_value(1522)), 1545);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1606] != 0.0)) {
            s.store_sub(1521, 1555, 1569);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1606] != 0.0)) {
            s.store_scale(459, 1521, s.v[1530]);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1606] != 0.0)) {
            s.copy_ad(458, 459);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) {
            s.store_scalar(1567, 3.0);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) {
            s.store_sub_ad_lhs(1570, A::div(s.ad_value(1567), s.ad_value(225)), 1545);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) {
            s.store_exp_ad(1529, A::neg(s.ad_value(1567)));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) {
            s.store_offset_ad(1528, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1555), s.ad_value(1545))), (-1.0)), s.ad_value(1529)), 4.0), A::mul(s.ad_value(1554), s.ad_value(226))), 1.0);
        }

        s.v[1608] = if (s.v[1528] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1608] != 0.0)) {
            s.store_scalar(1528, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) {
            s.store_add_ad_rhs(1570, 1555, A::mul(A::scale(A::mul(s.ad_value(1554), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1528)))));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) {
            s.store_mul_ad_rhs(1567, 225, A::add(s.ad_value(1570), s.ad_value(1545)));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) {
            s.store_exp_ad(1529, A::neg(s.ad_value(1567)));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) {
            s.store_offset_ad(1528, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1555), s.ad_value(1545))), (-1.0)), s.ad_value(1529)), 4.0), A::mul(s.ad_value(1554), s.ad_value(226))), 1.0);
        }

        s.v[1609] = if (s.v[1528] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1609] != 0.0)) {
            s.store_scalar(1528, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) {
            s.store_add_ad_rhs(1570, 1555, A::mul(A::scale(A::mul(s.ad_value(1554), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1528)))));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) {
            s.store_mul_ad_rhs(1567, 225, A::add(s.ad_value(1570), s.ad_value(1545)));
        }

        s.v[1610] = if (s.v[1567] < 3.0) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_scalar(1571, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_scalar(1572, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_offset_ad(1573, A::div_from_scalar(1.0, A::mul(s.ad_value(225), s.ad_value(1553))), (1.0 / 1.414213562373095));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_div_ad_lhs(1574, A::neg(A::add(s.ad_value(1555), s.ad_value(1545))), 1553);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_add_ad(1575, A::sub(A::div(A::mul(A::square(s.ad_value(1572)), s.ad_value(1572)), A::mul(A::mul(A::scale(s.ad_value(1571), 27.0), s.ad_value(1571)), s.ad_value(1571))), A::div(A::mul(s.ad_value(1572), s.ad_value(1573)), A::mul(A::scale(s.ad_value(1571), 6.0), s.ad_value(1571)))), A::div(s.ad_value(1574), A::scale(s.ad_value(1571), 2.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_div_ad(1576, A::sub(A::mul(A::scale(s.ad_value(1571), 3.0), s.ad_value(1573)), A::square(s.ad_value(1572))), A::mul(A::scale(s.ad_value(1571), 9.0), s.ad_value(1571)));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_sqrt_ad(1524, A::add(A::square(s.ad_value(1575)), A::mul(A::square(s.ad_value(1576)), s.ad_value(1576))));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_powf_ad(1577, A::sub(s.ad_value(1524), s.ad_value(1575)), 0.3333333333333333);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_neg_ad(1578, A::powf(A::add(s.ad_value(1575), s.ad_value(1524)), 0.3333333333333333));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_sub_ad(1528, A::add(s.ad_value(1577), s.ad_value(1578)), A::div(s.ad_value(1572), A::scale(s.ad_value(1571), 3.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_sub_ad_lhs(1570, A::mul(s.ad_value(1528), s.ad_value(227)), 1545);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_mul_ad_rhs(1567, 225, A::add(s.ad_value(1570), s.ad_value(1545)));
        }

        s.v[1611] = if (p.p41 > 0.0) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1611] != 0.0)) {
            s.store_offset_ad(1579, A::add(s.ad_value(1555), s.ad_value(1545)), 0.1);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1611] != 0.0)) {
            s.store_offset_ad(1586, A::exp(A::mul(s.ad_value(225), A::neg(s.ad_value(1545)))), 1e-50);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1611] != 0.0)) {
            s.store_scale(1519, 230, 1.0 / (s.v[69]));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1611] != 0.0)) {
            s.store_square(1580, 1519);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1611] != 0.0)) {
            s.store_mul(1581, 1580, 1586);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1611] != 0.0)) {
            s.store_mul(1519, 226, 1554);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1611] != 0.0)) {
            s.store_mul(1582, 225, 1579);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1611] != 0.0)) {
            s.store_add_ad(1583, A::sub(A::ln(A::add(A::mul(s.ad_value(1581), s.ad_value(1519)), A::square(s.ad_value(1582)))), A::ln(A::mul(s.ad_value(1580), s.ad_value(1519)))), A::mul(s.ad_value(225), s.ad_value(1545)));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1611] != 0.0)) {
            s.store_offset_ad(44, A::sub(s.ad_value(1582), s.ad_value(1583)), (-1.0));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1611] != 0.0)) {
            s.store_scale(45, 1582, 4.0);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1611] != 0.0)) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1611] != 0.0)) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1611] != 0.0)) {
            s.store_scale_ad(1520, A::offset(A::div(s.ad_value(44), s.ad_value(45)), 1.0), 0.5);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1611] != 0.0)) {
            s.store_scale_ad(1521, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(44), 2.0), s.ad_value(45))), 0.5);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1611] != 0.0)) {
            s.store_sub_ad_rhs(1583, 1582, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1611] != 0.0)) {
            s.store_sub(1582, 1582, 1583);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1611] != 0.0)) {
            s.store_add_ad_rhs(1582, 1582, A::scale(s.ad_value(225), 0.1));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1611] != 0.0)) {
            s.store_add_ad(1584, A::sub(A::ln(A::add(A::mul(s.ad_value(1581), s.ad_value(1519)), A::square(s.ad_value(1582)))), A::ln(A::mul(s.ad_value(1580), s.ad_value(1519)))), A::mul(s.ad_value(225), s.ad_value(1545)));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1611] != 0.0)) {
            s.copy_ad(1585, 1567);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1611] != 0.0)) {
            s.store_offset_ad(44, A::sub(s.ad_value(1584), s.ad_value(1585)), (-(0.0008 * 75.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1611] != 0.0)) {
            s.store_scale(45, 1584, (4.0 * (0.0008 * 75.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1611] != 0.0)) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1611] != 0.0)) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1611] != 0.0)) {
            s.store_scale_ad(1520, A::offset(A::div(s.ad_value(44), s.ad_value(45)), 1.0), 0.5);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1611] != 0.0)) {
            s.store_scale_ad(1521, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(44), ((2.0 * 0.0008) * 75.0)), s.ad_value(45))), 0.5);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1611] != 0.0)) {
            s.store_sub_ad_rhs(1567, 1584, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) {
            s.store_sub_ad_lhs(1569, A::div(s.ad_value(1567), s.ad_value(225)), 1545);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) {
            s.store_add_ad(1520, A::offset(s.ad_value(1567), (-1.0)), A::exp(A::neg(s.ad_value(1567))));
        }

        s.v[1612] = if (s.v[1520] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_scalar(1520, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) {
            s.store_sqrt(1521, 1520);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) {
            s.store_mul(458, 1552, 1521);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) {
            s.store_scaled_sub(459, 1555, 1569, s.v[1530]);
        }

        s.v[1613] = if (p.p41 == 1.0) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_exp_ad(1586, A::mul(s.ad_value(225), A::neg(s.ad_value(1545))));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_scale(1519, 230, 1.0 / (s.v[69]));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_square(1580, 1519);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_mul(1595, 1580, 1586);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_scalar(1542, 0.0);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_scalar(1589, 0.0);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_scalar(1593, 0.0);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) {
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
        let mut assign29750_loop_guard: usize = 0;
        while {
            let assign29750_cond_e42262: f64 = (2.0 * 20.0);
            let assign29750_cond_e42264: f64 = (assign29750_cond_e42262 + 1.0);
            let assign29750_cond_e42266: f64 = if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) && (s.v[167] <= assign29750_cond_e42264)) { 1.0 } else { 0.0 };
            assign29750_cond_e42266 != 0.0
        } {
            assign29750_loop_guard += 1;
            assert!(assign29750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) {
                s.store_scalar(1591, 0.0);
            }
            if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) {
                s.store_mul_ad_rhs(1567, 225, A::add(s.ad_value(1569), s.ad_value(1545)));
            }
            s.v[1614] = if (s.v[1567] < 5.0) { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) && (s.v[1614] != 0.0)) {
                s.store_mul_ad(1587, A::mul(A::square(s.ad_value(1567)), s.ad_value(1567)), A::offset(A::mul(s.ad_value(1567), A::offset(A::scale(s.ad_value(1567), 0.006115288895133179), (-0.07053654284009761))), 0.29693154855771));
            }
            if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) && (s.v[1614] != 0.0)) {
                s.store_mul_ad(1588, A::square(s.ad_value(1567)), A::offset(A::mul(s.ad_value(1567), A::offset(A::scale(s.ad_value(1567), (5.0 * 0.006115288895133179)), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
            }
            if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) && (s.v[1614] != 0.0)) {
                s.store_mul_ad_lhs(1589, A::mul(s.ad_value(1595), s.ad_value(1587)), 1587);
            }
            if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) && (s.v[1614] != 0.0)) {
                s.store_mul_ad_lhs(1590, A::mul(A::scale(A::mul(s.ad_value(1595), s.ad_value(225)), 2.0), s.ad_value(1587)), 1588);
            }
            if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) && (s.v[1614] != 0.0)) {
                s.store_mul_ad_rhs(1591, 1567, A::offset(A::mul(s.ad_value(1567), A::offset(A::mul(s.ad_value(1567), A::offset(A::mul(s.ad_value(1567), A::offset(A::scale(s.ad_value(1567), 6.36964918866352e-5), (-0.00163730162779191))), 0.0178800506338833)), (-0.117851130197758))), 0.707106781186548));
            }
            if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) && (s.v[1614] != 0.0)) {
                s.store_offset_ad(1592, A::mul(s.ad_value(1567), A::offset(A::mul(s.ad_value(1567), A::offset(A::mul(s.ad_value(1567), A::offset(A::scale(s.ad_value(1567), (5.0 * 6.36964918866352e-5)), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833))), (2.0 * (-0.117851130197758)))), 0.707106781186548);
            }
            if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) && (s.v[1614] != 0.0)) {
                s.store_sqrt_ad(1593, A::offset(A::add(A::square(s.ad_value(1591)), s.ad_value(1589)), 1e-50));
            }
            if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) && (s.v[1614] != 0.0)) {
                s.store_div_ad(1594, A::add(A::mul(A::scale(A::mul(s.ad_value(225), s.ad_value(1592)), 2.0), s.ad_value(1591)), s.ad_value(1590)), A::scale(s.ad_value(1593), 2.0));
            }
            s.v[1615] = if (s.v[1567] < 80.0) { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) && (!(s.v[1614] != 0.0))) && (s.v[1615] != 0.0)) {
                s.store_exp(243, 1567);
            }
            if (((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) && (!(s.v[1614] != 0.0))) && (s.v[1615] != 0.0)) {
                s.store_mul_ad_rhs(1589, 1595, A::offset(s.ad_value(243), (-1.0)));
            }
            if (((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) && (!(s.v[1614] != 0.0))) && (s.v[1615] != 0.0)) {
                s.store_mul_ad_lhs(1590, A::mul(s.ad_value(1595), s.ad_value(225)), 243);
            }
            if (((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) && (!(s.v[1614] != 0.0))) && (!(s.v[1615] != 0.0))) {
                s.store_exp_ad(1596, A::mul(s.ad_value(225), s.ad_value(1569)));
            }
            if (((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) && (!(s.v[1614] != 0.0))) && (!(s.v[1615] != 0.0))) {
                s.store_mul_ad_rhs(1589, 1580, A::sub(s.ad_value(1596), s.ad_value(1586)));
            }
            if (((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) && (!(s.v[1614] != 0.0))) && (!(s.v[1615] != 0.0))) {
                s.store_mul_ad_lhs(1590, A::mul(s.ad_value(1580), s.ad_value(225)), 1596);
            }
            if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) && (!(s.v[1614] != 0.0))) {
                s.store_sqrt_ad(1593, A::add(A::offset(s.ad_value(1567), (-1.0)), s.ad_value(1589)));
            }
            if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) && (!(s.v[1614] != 0.0))) {
                s.store_scale_ad(1594, A::div(A::add(s.ad_value(225), s.ad_value(1590)), s.ad_value(1593)), 0.5);
            }
            if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) {
                s.store_sub_ad(1597, A::sub(s.ad_value(1555), s.ad_value(1569)), A::mul(s.ad_value(1553), s.ad_value(1593)));
            }
            if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) {
                s.store_sub_from_scalar_ad(1598, (-1.0), A::mul(s.ad_value(1553), s.ad_value(1594)));
            }
            s.v[1616] = if (s.v[1542] == 1.0) { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) && (s.v[1616] != 0.0)) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) && (!(s.v[1616] != 0.0))) {
                s.store_div_ad_lhs(494, A::neg(s.ad_value(1597)), 1598);
            }
            if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) && (!(s.v[1616] != 0.0))) {
                s.store_scale_ad(1599, A::offset({
                    if (1.0 >= ((s.v[1569]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1569))
                    }
                }, 1.0), (0.5 * 0.1));
            }
            s.v[1617] = if (((s.v[494]) as f64).abs() > s.v[1599]) { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) && (!(s.v[1616] != 0.0))) && (s.v[1617] != 0.0)) {
                s.store_scale(494, 1599, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) && (!(s.v[1616] != 0.0))) {
                s.store_add(1569, 1569, 494);
            }
            s.v[1618] = if ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1597]) as f64).abs() <= 1e-8)) { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) && (!(s.v[1616] != 0.0))) && (s.v[1618] != 0.0)) {
                s.store_scalar(1542, 1.0);
            }
            if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.v[1620] = if (s.v[1567] < 5.0) { 1.0 } else { 0.0 };

        if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) && (s.v[1620] != 0.0)) {
            s.store_offset_ad(1600, A::square(s.ad_value(1591)), (10.0 * 2.220446049250313e-16));
        }

        if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) && (s.v[1620] != 0.0)) {
            s.store_offset(1601, 1591, (10.0 * 2.220446049250313e-16));
        }

        if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) && (!(s.v[1620] != 0.0))) {
            s.store_offset(1600, 1567, (-1.0));
        }

        if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) && (!(s.v[1620] != 0.0))) {
            s.store_sqrt(1601, 1600);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_mul(458, 1552, 1601);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_div_from_scalar_ad(1520, 1.0, A::add(s.ad_value(1593), s.ad_value(1601)));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_mul_ad_lhs(460, A::mul(s.ad_value(1552), s.ad_value(1589)), 1520);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1606] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_add(459, 458, 460);
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) {
            s.store_sub(460, 459, 458);
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) {
            s.store_ad(1523, &{
                if (p.p43 == 1.0) {
                    A::mul(s.ad_value(287), s.ad_value(1532))
                } else {
                    A::mul(s.ad_value(108), s.ad_value(1532))
                }
            });
        }

        s.v[1622] = if (((s.v[1538] != 0.0) && (p.p43 == 0.0)) || ((s.v[1536] != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1622] != 0.0)) {
            s.store_mul(455, 1523, 459);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1622] != 0.0)) {
            s.store_mul(457, 1523, 458);
        }

        s.v[1623] = if (((s.v[1539] != 0.0) && (p.p43 == 0.0)) || ((s.v[1537] != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1623] != 0.0)) {
            s.store_mul(454, 1523, 459);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1623] != 0.0)) {
            s.store_mul(456, 1523, 458);
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) {
            s.store_scalar(1536, ((1.0 - 1.0) / 2.0));
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) {
            s.store_scalar(1537, ((1.0 + 1.0) / 2.0));
        }

        s.v[1624] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1624] != 0.0)) {
            s.store_add_ad(1546, A::mul(s.ad_value(461), s.ad_value(156)), A::mul(s.ad_value(462), A::sub(s.ad_value(156), s.ad_value(157))));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1624] != 0.0)) {
            s.store_add_ad(1547, A::mul(s.ad_value(461), s.ad_value(157)), A::mul(s.ad_value(462), A::neg(s.ad_value(157))));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1624] != 0.0)) {
            s.store_add_ad(1548, A::mul(s.ad_value(461), s.ad_value(158)), A::mul(s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1624] != 0.0)) {
            s.store_sub(1549, 1547, 1546);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1624] != 0.0)) {
            s.store_sub(1551, 1548, 1546);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1624] != 0.0)) {
            s.store_neg(1550, 1546);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1624] != 0.0)) {
            s.store_add_ad(1538, A::mul(s.ad_value(1536), s.ad_value(461)), A::mul(s.ad_value(1537), s.ad_value(462)));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1624] != 0.0)) {
            s.store_add_ad(1539, A::mul(s.ad_value(1536), s.ad_value(462)), A::mul(s.ad_value(1537), s.ad_value(461)));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1624] != 0.0)) {
            s.store_offset_ad(1544, A::add(A::mul(s.ad_value(1538), s.ad_value(1550)), A::mul(s.ad_value(1539), s.ad_value(1549))), (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1624] != 0.0))) {
            s.store_add_ad(1538, A::mul(s.ad_value(1536), s.ad_value(461)), A::mul(s.ad_value(1537), s.ad_value(462)));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1624] != 0.0))) {
            s.store_add_ad(1539, A::mul(s.ad_value(1536), s.ad_value(462)), A::mul(s.ad_value(1537), s.ad_value(461)));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1624] != 0.0))) && (s.v[1536] != 0.0)) {
            s.store_add_ad(1551, A::mul(s.ad_value(461), s.ad_value(158)), A::mul(s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1624] != 0.0))) && (s.v[1537] != 0.0)) {
            s.store_add_ad(1551, A::mul(s.ad_value(462), s.ad_value(158)), A::mul(s.ad_value(461), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1624] != 0.0))) {
            s.store_scalar(1544, 0.0);
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) {
            s.store_neg(1519, 1544);
        }

        s.v[1625] = if (s.v[1519] > s.v[141]) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1625] != 0.0)) {
            s.store_sub(1520, 1519, 141);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1625] != 0.0)) {
            s.store_sub(1521, 140, 141);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1625] != 0.0)) {
            s.store_div(44, 1520, 1521);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1625] != 0.0)) {
            s.store_square(45, 44);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1625] != 0.0)) {
            s.store_mul(46, 45, 44);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1625] != 0.0)) {
            s.store_square(47, 45);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1625] != 0.0)) {
            s.store_div_from_scalar_ad(1529, 1.0, A::add(A::add(A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(45)), s.ad_value(46)), s.ad_value(47)));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1625] != 0.0)) {
            s.store_mul_ad_rhs(1529, 1521, A::sub_from_scalar(1.0, s.ad_value(1529)));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1625] != 0.0)) {
            s.store_add(1526, 141, 1529);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1625] != 0.0))) {
            s.copy_ad(1526, 1519);
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) {
            s.store_offset_ad(1545, A::neg(s.ad_value(1526)), (-1e-12));
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) {
            s.store_scale(1553, 1552, s.v[1531]);
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) {
            s.store_square(1554, 1553);
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) {
            s.store_sub_from_scalar(1555, s.v[82], 1551);
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) {
            s.store_div_from_scalar(1519, s.v[69], 230);
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) {
            s.store_mul_ad(1556, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(s.ad_value(1519)));
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) {
            s.store_neg(1557, 1545);
        }

        s.v[1626] = if (s.v[1555] < s.v[1557]) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_div_from_scalar_ad(1520, 1.0, A::mul(s.ad_value(225), s.ad_value(1552)));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_scale(1529, 1520, s.v[1530]);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_offset_scaled(1558, 1529, (3.0 * 1.414213562373095), 2.0);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_mul_ad_lhs(1559, A::mul(A::scale(s.ad_value(1558), 8.0), s.ad_value(1558)), 1558);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_sub(1560, 237, 1556);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_mul_ad_rhs(1528, 225, A::add(s.ad_value(1555), s.ad_value(1545)));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_sub_from_scalar_ad(1561, (7.0 * 1.414213562373095), A::mul(A::scale(s.ad_value(1529), 9.0), A::offset(s.ad_value(1528), (-2.0))));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_square(1562, 1561);
        }

        s.v[1627] = if (s.v[1559] < (s.v[1562] * 1e-8)) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1626] != 0.0)) && (s.v[1627] != 0.0)) {
            s.store_add_ad(1564, A::add(A::offset(s.ad_value(1561), ((-7.0) * 1.414213562373095)), A::div(A::scale(s.ad_value(1559), 0.5), s.ad_value(1561))), A::mul(A::scale(s.ad_value(1529), 9.0), A::offset(s.ad_value(1528), (-2.0))));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1626] != 0.0)) && (!(s.v[1627] != 0.0))) {
            s.store_sqrt_ad(1563, A::add(s.ad_value(1559), s.ad_value(1562)));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1626] != 0.0)) && (!(s.v[1627] != 0.0))) {
            s.store_add_ad(1564, A::offset(s.ad_value(1563), ((-7.0) * 1.414213562373095)), A::mul(A::scale(s.ad_value(1529), 9.0), A::offset(s.ad_value(1528), (-2.0))));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_powf(1565, 1564, 0.3333333333333333);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_add_ad(1566, A::add(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1529), 12.0)), A::scale(s.ad_value(1565), 2.0)), A::mul(A::scale(s.ad_value(1565), 1.414213562373095), s.ad_value(1565)));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_div(1567, 1566, 1565);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_sub_ad_lhs(1568, A::mul(s.ad_value(1567), s.ad_value(227)), 1545);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_add(1520, 1568, 1545);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_div(1521, 1520, 1560);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_sqrt_ad(1522, A::offset(A::square(s.ad_value(1521)), 1.0));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_sub_ad_lhs(1569, A::div(s.ad_value(1520), s.ad_value(1522)), 1545);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_sub(1521, 1555, 1569);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_scale(459, 1521, s.v[1530]);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1626] != 0.0)) {
            s.copy_ad(458, 459);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) {
            s.store_scalar(1567, 3.0);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) {
            s.store_sub_ad_lhs(1570, A::div(s.ad_value(1567), s.ad_value(225)), 1545);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) {
            s.store_exp_ad(1529, A::neg(s.ad_value(1567)));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) {
            s.store_offset_ad(1528, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1555), s.ad_value(1545))), (-1.0)), s.ad_value(1529)), 4.0), A::mul(s.ad_value(1554), s.ad_value(226))), 1.0);
        }

        s.v[1628] = if (s.v[1528] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_scalar(1528, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) {
            s.store_add_ad_rhs(1570, 1555, A::mul(A::scale(A::mul(s.ad_value(1554), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1528)))));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) {
            s.store_mul_ad_rhs(1567, 225, A::add(s.ad_value(1570), s.ad_value(1545)));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) {
            s.store_exp_ad(1529, A::neg(s.ad_value(1567)));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) {
            s.store_offset_ad(1528, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1555), s.ad_value(1545))), (-1.0)), s.ad_value(1529)), 4.0), A::mul(s.ad_value(1554), s.ad_value(226))), 1.0);
        }

        s.v[1629] = if (s.v[1528] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1629] != 0.0)) {
            s.store_scalar(1528, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) {
            s.store_add_ad_rhs(1570, 1555, A::mul(A::scale(A::mul(s.ad_value(1554), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1528)))));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) {
            s.store_mul_ad_rhs(1567, 225, A::add(s.ad_value(1570), s.ad_value(1545)));
        }

        s.v[1630] = if (s.v[1567] < 3.0) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_scalar(1571, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_scalar(1572, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_offset_ad(1573, A::div_from_scalar(1.0, A::mul(s.ad_value(225), s.ad_value(1553))), (1.0 / 1.414213562373095));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_div_ad_lhs(1574, A::neg(A::add(s.ad_value(1555), s.ad_value(1545))), 1553);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_add_ad(1575, A::sub(A::div(A::mul(A::square(s.ad_value(1572)), s.ad_value(1572)), A::mul(A::mul(A::scale(s.ad_value(1571), 27.0), s.ad_value(1571)), s.ad_value(1571))), A::div(A::mul(s.ad_value(1572), s.ad_value(1573)), A::mul(A::scale(s.ad_value(1571), 6.0), s.ad_value(1571)))), A::div(s.ad_value(1574), A::scale(s.ad_value(1571), 2.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_div_ad(1576, A::sub(A::mul(A::scale(s.ad_value(1571), 3.0), s.ad_value(1573)), A::square(s.ad_value(1572))), A::mul(A::scale(s.ad_value(1571), 9.0), s.ad_value(1571)));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_sqrt_ad(1524, A::add(A::square(s.ad_value(1575)), A::mul(A::square(s.ad_value(1576)), s.ad_value(1576))));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_powf_ad(1577, A::sub(s.ad_value(1524), s.ad_value(1575)), 0.3333333333333333);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_neg_ad(1578, A::powf(A::add(s.ad_value(1575), s.ad_value(1524)), 0.3333333333333333));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_sub_ad(1528, A::add(s.ad_value(1577), s.ad_value(1578)), A::div(s.ad_value(1572), A::scale(s.ad_value(1571), 3.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_sub_ad_lhs(1570, A::mul(s.ad_value(1528), s.ad_value(227)), 1545);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_mul_ad_rhs(1567, 225, A::add(s.ad_value(1570), s.ad_value(1545)));
        }

        s.v[1631] = if (p.p41 > 0.0) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1631] != 0.0)) {
            s.store_offset_ad(1579, A::add(s.ad_value(1555), s.ad_value(1545)), 0.1);
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
        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1631] != 0.0)) {
            s.store_offset_ad(1586, A::exp(A::mul(s.ad_value(225), A::neg(s.ad_value(1545)))), 1e-50);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1631] != 0.0)) {
            s.store_scale(1519, 230, 1.0 / (s.v[69]));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1631] != 0.0)) {
            s.store_square(1580, 1519);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1631] != 0.0)) {
            s.store_mul(1581, 1580, 1586);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1631] != 0.0)) {
            s.store_mul(1519, 226, 1554);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1631] != 0.0)) {
            s.store_mul(1582, 225, 1579);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1631] != 0.0)) {
            s.store_add_ad(1583, A::sub(A::ln(A::add(A::mul(s.ad_value(1581), s.ad_value(1519)), A::square(s.ad_value(1582)))), A::ln(A::mul(s.ad_value(1580), s.ad_value(1519)))), A::mul(s.ad_value(225), s.ad_value(1545)));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1631] != 0.0)) {
            s.store_offset_ad(44, A::sub(s.ad_value(1582), s.ad_value(1583)), (-1.0));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1631] != 0.0)) {
            s.store_scale(45, 1582, 4.0);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1631] != 0.0)) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1631] != 0.0)) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1631] != 0.0)) {
            s.store_scale_ad(1520, A::offset(A::div(s.ad_value(44), s.ad_value(45)), 1.0), 0.5);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1631] != 0.0)) {
            s.store_scale_ad(1521, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(44), 2.0), s.ad_value(45))), 0.5);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1631] != 0.0)) {
            s.store_sub_ad_rhs(1583, 1582, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1631] != 0.0)) {
            s.store_sub(1582, 1582, 1583);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1631] != 0.0)) {
            s.store_add_ad_rhs(1582, 1582, A::scale(s.ad_value(225), 0.1));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1631] != 0.0)) {
            s.store_add_ad(1584, A::sub(A::ln(A::add(A::mul(s.ad_value(1581), s.ad_value(1519)), A::square(s.ad_value(1582)))), A::ln(A::mul(s.ad_value(1580), s.ad_value(1519)))), A::mul(s.ad_value(225), s.ad_value(1545)));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1631] != 0.0)) {
            s.copy_ad(1585, 1567);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1631] != 0.0)) {
            s.store_offset_ad(44, A::sub(s.ad_value(1584), s.ad_value(1585)), (-(0.0008 * 75.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1631] != 0.0)) {
            s.store_scale(45, 1584, (4.0 * (0.0008 * 75.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1631] != 0.0)) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1631] != 0.0)) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1631] != 0.0)) {
            s.store_scale_ad(1520, A::offset(A::div(s.ad_value(44), s.ad_value(45)), 1.0), 0.5);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1631] != 0.0)) {
            s.store_scale_ad(1521, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(44), ((2.0 * 0.0008) * 75.0)), s.ad_value(45))), 0.5);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1631] != 0.0)) {
            s.store_sub_ad_rhs(1567, 1584, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) {
            s.store_sub_ad_lhs(1569, A::div(s.ad_value(1567), s.ad_value(225)), 1545);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) {
            s.store_add_ad(1520, A::offset(s.ad_value(1567), (-1.0)), A::exp(A::neg(s.ad_value(1567))));
        }

        s.v[1632] = if (s.v[1520] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1632] != 0.0)) {
            s.store_scalar(1520, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) {
            s.store_sqrt(1521, 1520);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) {
            s.store_mul(458, 1552, 1521);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) {
            s.store_scaled_sub(459, 1555, 1569, s.v[1530]);
        }

        s.v[1633] = if (p.p41 == 1.0) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_exp_ad(1586, A::mul(s.ad_value(225), A::neg(s.ad_value(1545))));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_scale(1519, 230, 1.0 / (s.v[69]));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_square(1580, 1519);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_mul(1595, 1580, 1586);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_scalar(1542, 0.0);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_scalar(1589, 0.0);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_scalar(1593, 0.0);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_scalar(167, 1.0);
        }

        let mut assign31350_loop_guard: usize = 0;
        while {
            let assign31350_cond_e45498: f64 = (2.0 * 20.0);
            let assign31350_cond_e45500: f64 = (assign31350_cond_e45498 + 1.0);
            let assign31350_cond_e45502: f64 = if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) && (s.v[167] <= assign31350_cond_e45500)) { 1.0 } else { 0.0 };
            assign31350_cond_e45502 != 0.0
        } {
            assign31350_loop_guard += 1;
            assert!(assign31350_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) {
                s.store_scalar(1591, 0.0);
            }
            if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) {
                s.store_mul_ad_rhs(1567, 225, A::add(s.ad_value(1569), s.ad_value(1545)));
            }
            s.v[1634] = if (s.v[1567] < 5.0) { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) && (s.v[1634] != 0.0)) {
                s.store_mul_ad(1587, A::mul(A::square(s.ad_value(1567)), s.ad_value(1567)), A::offset(A::mul(s.ad_value(1567), A::offset(A::scale(s.ad_value(1567), 0.006115288895133179), (-0.07053654284009761))), 0.29693154855771));
            }
            if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) && (s.v[1634] != 0.0)) {
                s.store_mul_ad(1588, A::square(s.ad_value(1567)), A::offset(A::mul(s.ad_value(1567), A::offset(A::scale(s.ad_value(1567), (5.0 * 0.006115288895133179)), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
            }
            if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) && (s.v[1634] != 0.0)) {
                s.store_mul_ad_lhs(1589, A::mul(s.ad_value(1595), s.ad_value(1587)), 1587);
            }
            if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) && (s.v[1634] != 0.0)) {
                s.store_mul_ad_lhs(1590, A::mul(A::scale(A::mul(s.ad_value(1595), s.ad_value(225)), 2.0), s.ad_value(1587)), 1588);
            }
            if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) && (s.v[1634] != 0.0)) {
                s.store_mul_ad_rhs(1591, 1567, A::offset(A::mul(s.ad_value(1567), A::offset(A::mul(s.ad_value(1567), A::offset(A::mul(s.ad_value(1567), A::offset(A::scale(s.ad_value(1567), 6.36964918866352e-5), (-0.00163730162779191))), 0.0178800506338833)), (-0.117851130197758))), 0.707106781186548));
            }
            if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) && (s.v[1634] != 0.0)) {
                s.store_offset_ad(1592, A::mul(s.ad_value(1567), A::offset(A::mul(s.ad_value(1567), A::offset(A::mul(s.ad_value(1567), A::offset(A::scale(s.ad_value(1567), (5.0 * 6.36964918866352e-5)), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833))), (2.0 * (-0.117851130197758)))), 0.707106781186548);
            }
            if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) && (s.v[1634] != 0.0)) {
                s.store_sqrt_ad(1593, A::offset(A::add(A::square(s.ad_value(1591)), s.ad_value(1589)), 1e-50));
            }
            if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) && (s.v[1634] != 0.0)) {
                s.store_div_ad(1594, A::add(A::mul(A::scale(A::mul(s.ad_value(225), s.ad_value(1592)), 2.0), s.ad_value(1591)), s.ad_value(1590)), A::scale(s.ad_value(1593), 2.0));
            }
            s.v[1635] = if (s.v[1567] < 80.0) { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) && (!(s.v[1634] != 0.0))) && (s.v[1635] != 0.0)) {
                s.store_exp(243, 1567);
            }
            if (((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) && (!(s.v[1634] != 0.0))) && (s.v[1635] != 0.0)) {
                s.store_mul_ad_rhs(1589, 1595, A::offset(s.ad_value(243), (-1.0)));
            }
            if (((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) && (!(s.v[1634] != 0.0))) && (s.v[1635] != 0.0)) {
                s.store_mul_ad_lhs(1590, A::mul(s.ad_value(1595), s.ad_value(225)), 243);
            }
            if (((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) && (!(s.v[1634] != 0.0))) && (!(s.v[1635] != 0.0))) {
                s.store_exp_ad(1596, A::mul(s.ad_value(225), s.ad_value(1569)));
            }
            if (((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) && (!(s.v[1634] != 0.0))) && (!(s.v[1635] != 0.0))) {
                s.store_mul_ad_rhs(1589, 1580, A::sub(s.ad_value(1596), s.ad_value(1586)));
            }
            if (((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) && (!(s.v[1634] != 0.0))) && (!(s.v[1635] != 0.0))) {
                s.store_mul_ad_lhs(1590, A::mul(s.ad_value(1580), s.ad_value(225)), 1596);
            }
            if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) && (!(s.v[1634] != 0.0))) {
                s.store_sqrt_ad(1593, A::add(A::offset(s.ad_value(1567), (-1.0)), s.ad_value(1589)));
            }
            if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) && (!(s.v[1634] != 0.0))) {
                s.store_scale_ad(1594, A::div(A::add(s.ad_value(225), s.ad_value(1590)), s.ad_value(1593)), 0.5);
            }
            if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) {
                s.store_sub_ad(1597, A::sub(s.ad_value(1555), s.ad_value(1569)), A::mul(s.ad_value(1553), s.ad_value(1593)));
            }
            if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) {
                s.store_sub_from_scalar_ad(1598, (-1.0), A::mul(s.ad_value(1553), s.ad_value(1594)));
            }
            s.v[1636] = if (s.v[1542] == 1.0) { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) && (s.v[1636] != 0.0)) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) && (!(s.v[1636] != 0.0))) {
                s.store_div_ad_lhs(494, A::neg(s.ad_value(1597)), 1598);
            }
            if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) && (!(s.v[1636] != 0.0))) {
                s.store_scale_ad(1599, A::offset({
                    if (1.0 >= ((s.v[1569]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1569))
                    }
                }, 1.0), (0.5 * 0.1));
            }
            s.v[1637] = if (((s.v[494]) as f64).abs() > s.v[1599]) { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) && (!(s.v[1636] != 0.0))) && (s.v[1637] != 0.0)) {
                s.store_scale(494, 1599, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) && (!(s.v[1636] != 0.0))) {
                s.store_add(1569, 1569, 494);
            }
            s.v[1638] = if ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1597]) as f64).abs() <= 1e-8)) { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) && (!(s.v[1636] != 0.0))) && (s.v[1638] != 0.0)) {
                s.store_scalar(1542, 1.0);
            }
            if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.v[1640] = if (s.v[1567] < 5.0) { 1.0 } else { 0.0 };

        if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) && (s.v[1640] != 0.0)) {
            s.store_offset_ad(1600, A::square(s.ad_value(1591)), (10.0 * 2.220446049250313e-16));
        }

        if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) && (s.v[1640] != 0.0)) {
            s.store_offset(1601, 1591, (10.0 * 2.220446049250313e-16));
        }

        if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) && (!(s.v[1640] != 0.0))) {
            s.store_offset(1600, 1567, (-1.0));
        }

        if ((((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) && (!(s.v[1640] != 0.0))) {
            s.store_sqrt(1601, 1600);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_mul(458, 1552, 1601);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_div_from_scalar_ad(1520, 1.0, A::add(s.ad_value(1593), s.ad_value(1601)));
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_mul_ad_lhs(460, A::mul(s.ad_value(1552), s.ad_value(1589)), 1520);
        }

        if (((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (!(s.v[1626] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_add(459, 458, 460);
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) {
            s.store_sub(460, 459, 458);
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) {
            s.store_ad(1523, &{
                if (p.p43 == 1.0) {
                    A::mul(s.ad_value(287), s.ad_value(1532))
                } else {
                    A::mul(s.ad_value(108), s.ad_value(1532))
                }
            });
        }

        s.v[1642] = if (((s.v[1538] != 0.0) && (p.p43 == 0.0)) || ((s.v[1536] != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1642] != 0.0)) {
            s.store_mul(455, 1523, 459);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1642] != 0.0)) {
            s.store_mul(457, 1523, 458);
        }

        s.v[1643] = if (((s.v[1539] != 0.0) && (p.p43 == 0.0)) || ((s.v[1537] != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1643] != 0.0)) {
            s.store_mul(454, 1523, 459);
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) && (s.v[1643] != 0.0)) {
            s.store_mul(456, 1523, 458);
        }

        if ((p.p24 != 0.0) && (s.v[1602] != 0.0)) {
            s.store_add_ad(266, A::scale(s.ad_value(462), s.v[566]), A::scale(s.ad_value(461), s.v[565]));
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (s.v[266] != 0.0)) {
            s.store_add_ad(269, A::scale(s.ad_value(462), p.p170), A::scale(s.ad_value(461), p.p169));
        }

        s.v[1644] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (s.v[266] != 0.0)) && (s.v[1644] != 0.0)) {
            s.store_add_ad(1520, A::mul(s.ad_value(462), s.ad_value(287)), A::mul(s.ad_value(461), s.ad_value(288)));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (s.v[266] != 0.0)) && (s.v[1644] != 0.0)) {
            s.store_mul_ad_rhs(269, 269, A::neg(s.ad_value(1520)));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (s.v[266] != 0.0)) && (!(s.v[1644] != 0.0))) {
            s.store_mul_ad_rhs(269, 269, A::neg(s.ad_value(108)));
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (s.v[266] != 0.0)) {
            s.store_add_ad_rhs(268, 268, A::mul(A::neg(s.ad_value(269)), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if ((p.p24 != 0.0) && (s.v[1602] != 0.0)) {
            s.store_add_ad(266, A::scale(s.ad_value(461), s.v[566]), A::scale(s.ad_value(462), s.v[565]));
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (s.v[266] != 0.0)) {
            s.store_add_ad(270, A::scale(s.ad_value(461), p.p170), A::scale(s.ad_value(462), p.p169));
        }

        s.v[1645] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (s.v[266] != 0.0)) && (s.v[1645] != 0.0)) {
            s.store_add_ad(1520, A::mul(s.ad_value(461), s.ad_value(287)), A::mul(s.ad_value(462), s.ad_value(288)));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (s.v[266] != 0.0)) && (s.v[1645] != 0.0)) {
            s.store_mul_ad_rhs(270, 270, A::neg(s.ad_value(1520)));
        }

        if ((((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (s.v[266] != 0.0)) && (!(s.v[1645] != 0.0))) {
            s.store_mul_ad_rhs(270, 270, A::neg(s.ad_value(108)));
        }

        if (((p.p24 != 0.0) && (s.v[1602] != 0.0)) && (s.v[266] != 0.0)) {
            s.store_add_ad_rhs(267, 267, A::mul(A::neg(s.ad_value(270)), s.ad_value(158)));
        }

        s.v[1646] = if (((s.v[613] == 1.0) && (!(s.v[565] != 0.0))) || ((s.v[613] != 1.0) && (!(s.v[566] != 0.0)))) { 1.0 } else { 0.0 };

        s.v[1647] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (!(s.v[1602] != 0.0))) && (s.v[1646] != 0.0)) && (s.v[1647] != 0.0)) {
            s.store_scale(269, 288, ((-s.v[1530]) * p.p188));
        }

        if ((((p.p24 != 0.0) && (!(s.v[1602] != 0.0))) && (s.v[1646] != 0.0)) && (!(s.v[1647] != 0.0))) {
            s.store_scale(269, 108, ((-s.v[1530]) * p.p188));
        }

        if (((p.p24 != 0.0) && (!(s.v[1602] != 0.0))) && (!(s.v[1646] != 0.0))) {
            s.store_add_ad(269, A::scale(s.ad_value(462), p.p170), A::scale(s.ad_value(461), p.p169));
        }

        s.v[1648] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (!(s.v[1602] != 0.0))) && (!(s.v[1646] != 0.0))) && (s.v[1648] != 0.0)) {
            s.store_add_ad(1520, A::mul(s.ad_value(462), s.ad_value(287)), A::mul(s.ad_value(461), s.ad_value(288)));
        }

        if ((((p.p24 != 0.0) && (!(s.v[1602] != 0.0))) && (!(s.v[1646] != 0.0))) && (s.v[1648] != 0.0)) {
            s.store_mul_ad_rhs(269, 269, A::neg(s.ad_value(1520)));
        }

        if ((((p.p24 != 0.0) && (!(s.v[1602] != 0.0))) && (!(s.v[1646] != 0.0))) && (!(s.v[1648] != 0.0))) {
            s.store_mul_ad_rhs(269, 269, A::neg(s.ad_value(108)));
        }

        if ((p.p24 != 0.0) && (!(s.v[1602] != 0.0))) {
            s.store_mul_ad(268, A::neg(s.ad_value(269)), A::sub(s.ad_value(158), s.ad_value(157)));
        }

        s.v[1649] = if (((s.v[613] == 1.0) && (!(s.v[566] != 0.0))) || ((s.v[613] != 1.0) && (!(s.v[565] != 0.0)))) { 1.0 } else { 0.0 };

        s.v[1650] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (!(s.v[1602] != 0.0))) && (s.v[1649] != 0.0)) && (s.v[1650] != 0.0)) {
            s.store_scale(270, 287, ((-s.v[1530]) * p.p188));
        }

        if ((((p.p24 != 0.0) && (!(s.v[1602] != 0.0))) && (s.v[1649] != 0.0)) && (!(s.v[1650] != 0.0))) {
            s.store_scale(270, 108, ((-s.v[1530]) * p.p188));
        }

        if (((p.p24 != 0.0) && (!(s.v[1602] != 0.0))) && (!(s.v[1649] != 0.0))) {
            s.store_add_ad(270, A::scale(s.ad_value(461), p.p170), A::scale(s.ad_value(462), p.p169));
        }

        s.v[1651] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (!(s.v[1602] != 0.0))) && (!(s.v[1649] != 0.0))) && (s.v[1651] != 0.0)) {
            s.store_add_ad(1520, A::mul(s.ad_value(461), s.ad_value(287)), A::mul(s.ad_value(462), s.ad_value(288)));
        }

        if ((((p.p24 != 0.0) && (!(s.v[1602] != 0.0))) && (!(s.v[1649] != 0.0))) && (s.v[1651] != 0.0)) {
            s.store_mul_ad_rhs(270, 270, A::neg(s.ad_value(1520)));
        }

        if ((((p.p24 != 0.0) && (!(s.v[1602] != 0.0))) && (!(s.v[1649] != 0.0))) && (!(s.v[1651] != 0.0))) {
            s.store_mul_ad_rhs(270, 270, A::neg(s.ad_value(108)));
        }

        if ((p.p24 != 0.0) && (!(s.v[1602] != 0.0))) {
            s.store_mul_ad_lhs(267, A::neg(s.ad_value(270)), 158);
        }

        s.v[1652] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1652] != 0.0) {
            s.copy_ad(1668, 590);
        }

        if (s.v[1652] != 0.0) {
            s.copy_ad(1669, 591);
        }

        if (s.v[1652] != 0.0) {
            s.store_scale_ad(1670, A::exp(A::scale(A::add(A::sub_from_scalar((s.v[87] * s.v[114]), A::mul(s.ad_value(237), s.ad_value(225))), A::scale(A::ln(A::scale(s.ad_value(429), 1.0 / (s.v[81]))), p.p175)), 1.0 / (p.p174))), p.p173);
        }

        if (s.v[1652] != 0.0) {
            s.store_scale_ad(1671, A::exp(A::scale(A::add(A::sub_from_scalar((s.v[87] * s.v[114]), A::mul(s.ad_value(237), s.ad_value(225))), A::scale(A::ln(A::scale(s.ad_value(429), 1.0 / (s.v[81]))), p.p176)), 1.0 / (p.p174))), p.p173);
        }

        if (s.v[1652] != 0.0) {
            s.store_mul_ad_lhs(1675, A::scale(s.ad_value(286), p.p237), 1670);
        }

        if (s.v[1652] != 0.0) {
            s.store_mul_ad_lhs(1677, A::scale(s.ad_value(286), p.p237), 1671);
        }

        if (s.v[1652] != 0.0) {
            s.store_mul_ad_lhs(1676, A::scale(s.ad_value(285), p.p237), 1670);
        }

        if (s.v[1652] != 0.0) {
            s.store_mul_ad_lhs(1678, A::scale(s.ad_value(285), p.p237), 1671);
        }

        if (s.v[1652] != 0.0) {
            s.store_scale(1654, 429, 1.0 / (s.v[81]));
        }

        if (s.v[1652] != 0.0) {
            s.store_offset(1655, 1675, 1e-50);
        }

        if (s.v[1652] != 0.0) {
            s.store_scale_ad(1673, A::div_from_scalar(p.p174, s.ad_value(225)), 0.0);
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
        if (s.v[1652] != 0.0) {
            s.store_scale_ad(1674, A::div_from_scalar(p.p174, s.ad_value(225)), 0.0);
        }

        if (s.v[1652] != 0.0) {
            s.store_scale(1672, 227, p.p174);
        }

        s.v[1681] = if (s.v[1668] < s.v[1673]) { 1.0 } else { 0.0 };

        if ((s.v[1652] != 0.0) && (s.v[1681] != 0.0)) {
            s.store_exp_ad(1654, A::div(s.ad_value(1668), s.ad_value(1672)));
        }

        if ((s.v[1652] != 0.0) && (s.v[1681] != 0.0)) {
            s.store_mul_ad_rhs(282, 1675, A::offset(s.ad_value(1654), (-1.0)));
        }

        if ((s.v[1652] != 0.0) && (!(s.v[1681] != 0.0))) {
            s.store_exp_ad(1654, A::div(s.ad_value(1673), s.ad_value(1672)));
        }

        if ((s.v[1652] != 0.0) && (!(s.v[1681] != 0.0))) {
            s.store_add_ad(282, A::mul(s.ad_value(1675), A::offset(s.ad_value(1654), (-1.0))), A::mul(A::mul(A::div(s.ad_value(1675), s.ad_value(1672)), s.ad_value(1654)), A::sub(s.ad_value(1668), s.ad_value(1673))));
        }

        if (s.v[1652] != 0.0) {
            s.store_add_ad_rhs(282, 282, A::mul(A::scale(s.ad_value(1668), p.p178), s.ad_value(1677)));
        }

        s.v[1682] = if (s.v[1669] < s.v[1674]) { 1.0 } else { 0.0 };

        if ((s.v[1652] != 0.0) && (s.v[1682] != 0.0)) {
            s.store_exp_ad(1654, A::div(s.ad_value(1669), s.ad_value(1672)));
        }

        if ((s.v[1652] != 0.0) && (s.v[1682] != 0.0)) {
            s.store_mul_ad_rhs(281, 1676, A::offset(s.ad_value(1654), (-1.0)));
        }

        if ((s.v[1652] != 0.0) && (!(s.v[1682] != 0.0))) {
            s.store_exp_ad(1654, A::div(s.ad_value(1674), s.ad_value(1672)));
        }

        if ((s.v[1652] != 0.0) && (!(s.v[1682] != 0.0))) {
            s.store_add_ad(281, A::mul(s.ad_value(1676), A::offset(s.ad_value(1654), (-1.0))), A::mul(A::mul(A::div(s.ad_value(1676), s.ad_value(1672)), s.ad_value(1654)), A::sub(s.ad_value(1669), s.ad_value(1674))));
        }

        if (s.v[1652] != 0.0) {
            s.store_add_ad_rhs(281, 281, A::mul(A::scale(s.ad_value(1669), p.p178), s.ad_value(1678)));
        }

        if (s.v[1652] != 0.0) {
            s.store_add_ad_rhs(282, 282, A::scale(s.ad_value(1668), s.v[142]));
        }

        if (s.v[1652] != 0.0) {
            s.store_add_ad_rhs(281, 281, A::scale(s.ad_value(1669), s.v[142]));
        }

        if (s.v[1652] != 0.0) {
            s.store_scalar(1662, (p.p179 * p.p2));
        }

        if (s.v[1652] != 0.0) {
            s.store_scalar(1663, (p.p179 * p.p3));
        }

        if (s.v[1652] != 0.0) {
            s.store_scalar(1661, (p.p237 - p.p238));
        }

        s.v[1683] = if (s.v[1661] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1652] != 0.0) && (s.v[1683] != 0.0)) {
            s.store_scalar(1662, 0.0);
        }

        if ((s.v[1652] != 0.0) && (s.v[1683] != 0.0)) {
            s.store_scalar(1663, 0.0);
        }

        s.v[1684] = if (p.p5 > s.v[287]) { 1.0 } else { 0.0 };

        if ((s.v[1652] != 0.0) && (s.v[1684] != 0.0)) {
            s.store_scale_ad(1665, A::sub_from_scalar(p.p5, s.ad_value(287)), p.p180);
        }

        if ((s.v[1652] != 0.0) && (s.v[1684] != 0.0)) {
            s.store_scale(1667, 287, p.p181);
        }

        s.v[1685] = if (s.v[1669] < 0.0) { 1.0 } else { 0.0 };

        s.v[1686] = if (s.v[1663] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1652] != 0.0) && (s.v[1684] != 0.0)) && (s.v[1685] != 0.0)) && (s.v[1686] != 0.0)) {
            s.store_sub_from_scalar_ad(1679, 1.0, A::scale(s.ad_value(1669), 1.0 / (p.p185)));
        }

        s.v[1687] = if (p.p182 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1652] != 0.0) && (s.v[1684] != 0.0)) && (s.v[1685] != 0.0)) && (s.v[1686] != 0.0)) && (s.v[1687] != 0.0)) {
            s.store_div_from_scalar_ad(1680, 1.0, A::sqrt(s.ad_value(1679)));
        }

        if (((((s.v[1652] != 0.0) && (s.v[1684] != 0.0)) && (s.v[1685] != 0.0)) && (s.v[1686] != 0.0)) && (!(s.v[1687] != 0.0))) {
            s.store_powf(1680, 1679, (-p.p182));
        }

        if ((((s.v[1652] != 0.0) && (s.v[1684] != 0.0)) && (s.v[1685] != 0.0)) && (s.v[1686] != 0.0)) {
            s.store_scale_ad(283, A::mul(A::scale(s.ad_value(1663), p.p185), A::sub_from_scalar(1.0, A::mul(s.ad_value(1679), s.ad_value(1680)))), 1.0 / ((1.0 - p.p182)));
        }

        if ((((s.v[1652] != 0.0) && (s.v[1684] != 0.0)) && (s.v[1685] != 0.0)) && (!(s.v[1686] != 0.0))) {
            s.store_scalar(283, 0.0);
        }

        s.v[1688] = if (s.v[1665] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1652] != 0.0) && (s.v[1684] != 0.0)) && (s.v[1685] != 0.0)) && (s.v[1688] != 0.0)) {
            s.store_sub_from_scalar_ad(1679, 1.0, A::scale(s.ad_value(1669), 1.0 / (p.p186)));
        }

        s.v[1689] = if (p.p183 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1652] != 0.0) && (s.v[1684] != 0.0)) && (s.v[1685] != 0.0)) && (s.v[1688] != 0.0)) && (s.v[1689] != 0.0)) {
            s.store_div_from_scalar_ad(1680, 1.0, A::sqrt(s.ad_value(1679)));
        }

        if (((((s.v[1652] != 0.0) && (s.v[1684] != 0.0)) && (s.v[1685] != 0.0)) && (s.v[1688] != 0.0)) && (!(s.v[1689] != 0.0))) {
            s.store_powf(1680, 1679, (-p.p183));
        }

        if ((((s.v[1652] != 0.0) && (s.v[1684] != 0.0)) && (s.v[1685] != 0.0)) && (s.v[1688] != 0.0)) {
            s.store_add_ad_rhs(283, 283, A::scale(A::mul(A::scale(s.ad_value(1665), p.p186), A::sub_from_scalar(1.0, A::mul(s.ad_value(1679), s.ad_value(1680)))), 1.0 / ((1.0 - p.p183))));
        }

        s.v[1690] = if (s.v[1667] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1652] != 0.0) && (s.v[1684] != 0.0)) && (s.v[1685] != 0.0)) && (s.v[1690] != 0.0)) {
            s.store_sub_from_scalar_ad(1679, 1.0, A::scale(s.ad_value(1669), 1.0 / (p.p187)));
        }

        s.v[1691] = if (p.p184 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1652] != 0.0) && (s.v[1684] != 0.0)) && (s.v[1685] != 0.0)) && (s.v[1690] != 0.0)) && (s.v[1691] != 0.0)) {
            s.store_div_from_scalar_ad(1680, 1.0, A::sqrt(s.ad_value(1679)));
        }

        if (((((s.v[1652] != 0.0) && (s.v[1684] != 0.0)) && (s.v[1685] != 0.0)) && (s.v[1690] != 0.0)) && (!(s.v[1691] != 0.0))) {
            s.store_powf(1680, 1679, (-p.p184));
        }

        if ((((s.v[1652] != 0.0) && (s.v[1684] != 0.0)) && (s.v[1685] != 0.0)) && (s.v[1690] != 0.0)) {
            s.store_add_ad_rhs(283, 283, A::scale(A::mul(A::scale(s.ad_value(1667), p.p187), A::sub_from_scalar(1.0, A::mul(s.ad_value(1679), s.ad_value(1680)))), 1.0 / ((1.0 - p.p184))));
        }

        if (((s.v[1652] != 0.0) && (s.v[1684] != 0.0)) && (!(s.v[1685] != 0.0))) {
            s.store_add_ad_lhs(1654, A::add(s.ad_value(1663), s.ad_value(1665)), 1667);
        }

        if (((s.v[1652] != 0.0) && (s.v[1684] != 0.0)) && (!(s.v[1685] != 0.0))) {
            s.store_add_ad(1655, A::add(A::scale(s.ad_value(1663), (p.p182 * 1.0 / (p.p185))), A::scale(s.ad_value(1665), (p.p183 * 1.0 / (p.p186)))), A::scale(s.ad_value(1667), (p.p184 * 1.0 / (p.p187))));
        }

        if (((s.v[1652] != 0.0) && (s.v[1684] != 0.0)) && (!(s.v[1685] != 0.0))) {
            s.store_mul_ad_rhs(283, 1669, A::add(s.ad_value(1654), A::mul(A::scale(s.ad_value(1669), 0.5), s.ad_value(1655))));
        }

        if ((s.v[1652] != 0.0) && (!(s.v[1684] != 0.0))) {
            s.store_scalar(1667, (p.p181 * p.p5));
        }

        s.v[1692] = if (s.v[1669] < 0.0) { 1.0 } else { 0.0 };

        s.v[1693] = if (s.v[1663] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1652] != 0.0) && (!(s.v[1684] != 0.0))) && (s.v[1692] != 0.0)) && (s.v[1693] != 0.0)) {
            s.store_sub_from_scalar_ad(1679, 1.0, A::scale(s.ad_value(1669), 1.0 / (p.p185)));
        }

        s.v[1694] = if (p.p182 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1652] != 0.0) && (!(s.v[1684] != 0.0))) && (s.v[1692] != 0.0)) && (s.v[1693] != 0.0)) && (s.v[1694] != 0.0)) {
            s.store_div_from_scalar_ad(1680, 1.0, A::sqrt(s.ad_value(1679)));
        }

        if (((((s.v[1652] != 0.0) && (!(s.v[1684] != 0.0))) && (s.v[1692] != 0.0)) && (s.v[1693] != 0.0)) && (!(s.v[1694] != 0.0))) {
            s.store_powf(1680, 1679, (-p.p182));
        }

        if ((((s.v[1652] != 0.0) && (!(s.v[1684] != 0.0))) && (s.v[1692] != 0.0)) && (s.v[1693] != 0.0)) {
            s.store_scale_ad(283, A::mul(A::scale(s.ad_value(1663), p.p185), A::sub_from_scalar(1.0, A::mul(s.ad_value(1679), s.ad_value(1680)))), 1.0 / ((1.0 - p.p182)));
        }

        if ((((s.v[1652] != 0.0) && (!(s.v[1684] != 0.0))) && (s.v[1692] != 0.0)) && (!(s.v[1693] != 0.0))) {
            s.store_scalar(283, 0.0);
        }

        s.v[1695] = if (s.v[1667] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1652] != 0.0) && (!(s.v[1684] != 0.0))) && (s.v[1692] != 0.0)) && (s.v[1695] != 0.0)) {
            s.store_sub_from_scalar_ad(1679, 1.0, A::scale(s.ad_value(1669), 1.0 / (p.p187)));
        }

        s.v[1696] = if (p.p184 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1652] != 0.0) && (!(s.v[1684] != 0.0))) && (s.v[1692] != 0.0)) && (s.v[1695] != 0.0)) && (s.v[1696] != 0.0)) {
            s.store_div_from_scalar_ad(1680, 1.0, A::sqrt(s.ad_value(1679)));
        }

        if (((((s.v[1652] != 0.0) && (!(s.v[1684] != 0.0))) && (s.v[1692] != 0.0)) && (s.v[1695] != 0.0)) && (!(s.v[1696] != 0.0))) {
            s.store_powf(1680, 1679, (-p.p184));
        }

        if ((((s.v[1652] != 0.0) && (!(s.v[1684] != 0.0))) && (s.v[1692] != 0.0)) && (s.v[1695] != 0.0)) {
            s.store_add_ad_rhs(283, 283, A::scale(A::mul(A::scale(s.ad_value(1667), p.p187), A::sub_from_scalar(1.0, A::mul(s.ad_value(1679), s.ad_value(1680)))), 1.0 / ((1.0 - p.p184))));
        }

        if (((s.v[1652] != 0.0) && (!(s.v[1684] != 0.0))) && (!(s.v[1692] != 0.0))) {
            s.store_add(1654, 1663, 1667);
        }

        if (((s.v[1652] != 0.0) && (!(s.v[1684] != 0.0))) && (!(s.v[1692] != 0.0))) {
            s.store_add_ad(1655, A::scale(s.ad_value(1663), (p.p182 * 1.0 / (p.p185))), A::scale(s.ad_value(1667), (p.p184 * 1.0 / (p.p187))));
        }

        if (((s.v[1652] != 0.0) && (!(s.v[1684] != 0.0))) && (!(s.v[1692] != 0.0))) {
            s.store_mul_ad_rhs(283, 1669, A::add(s.ad_value(1654), A::mul(A::scale(s.ad_value(1669), 0.5), s.ad_value(1655))));
        }

        s.v[1697] = if (p.p4 > s.v[288]) { 1.0 } else { 0.0 };

        if ((s.v[1652] != 0.0) && (s.v[1697] != 0.0)) {
            s.store_scale_ad(1664, A::sub_from_scalar(p.p4, s.ad_value(288)), p.p180);
        }

        if ((s.v[1652] != 0.0) && (s.v[1697] != 0.0)) {
            s.store_scale(1666, 288, p.p181);
        }

        s.v[1698] = if (s.v[1668] < 0.0) { 1.0 } else { 0.0 };

        s.v[1699] = if (s.v[1662] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1652] != 0.0) && (s.v[1697] != 0.0)) && (s.v[1698] != 0.0)) && (s.v[1699] != 0.0)) {
            s.store_sub_from_scalar_ad(1679, 1.0, A::scale(s.ad_value(1668), 1.0 / (p.p185)));
        }

        s.v[1700] = if (p.p182 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1652] != 0.0) && (s.v[1697] != 0.0)) && (s.v[1698] != 0.0)) && (s.v[1699] != 0.0)) && (s.v[1700] != 0.0)) {
            s.store_div_from_scalar_ad(1680, 1.0, A::sqrt(s.ad_value(1679)));
        }

        if (((((s.v[1652] != 0.0) && (s.v[1697] != 0.0)) && (s.v[1698] != 0.0)) && (s.v[1699] != 0.0)) && (!(s.v[1700] != 0.0))) {
            s.store_powf(1680, 1679, (-p.p182));
        }

        if ((((s.v[1652] != 0.0) && (s.v[1697] != 0.0)) && (s.v[1698] != 0.0)) && (s.v[1699] != 0.0)) {
            s.store_scale_ad(284, A::mul(A::scale(s.ad_value(1662), p.p185), A::sub_from_scalar(1.0, A::mul(s.ad_value(1679), s.ad_value(1680)))), 1.0 / ((1.0 - p.p182)));
        }

        if ((((s.v[1652] != 0.0) && (s.v[1697] != 0.0)) && (s.v[1698] != 0.0)) && (!(s.v[1699] != 0.0))) {
            s.store_scalar(284, 0.0);
        }

        s.v[1701] = if (s.v[1664] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1652] != 0.0) && (s.v[1697] != 0.0)) && (s.v[1698] != 0.0)) && (s.v[1701] != 0.0)) {
            s.store_sub_from_scalar_ad(1679, 1.0, A::scale(s.ad_value(1668), 1.0 / (p.p186)));
        }

        s.v[1702] = if (p.p183 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1652] != 0.0) && (s.v[1697] != 0.0)) && (s.v[1698] != 0.0)) && (s.v[1701] != 0.0)) && (s.v[1702] != 0.0)) {
            s.store_div_from_scalar_ad(1680, 1.0, A::sqrt(s.ad_value(1679)));
        }

        if (((((s.v[1652] != 0.0) && (s.v[1697] != 0.0)) && (s.v[1698] != 0.0)) && (s.v[1701] != 0.0)) && (!(s.v[1702] != 0.0))) {
            s.store_powf(1680, 1679, (-p.p183));
        }

        if ((((s.v[1652] != 0.0) && (s.v[1697] != 0.0)) && (s.v[1698] != 0.0)) && (s.v[1701] != 0.0)) {
            s.store_add_ad_rhs(284, 284, A::scale(A::mul(A::scale(s.ad_value(1664), p.p186), A::sub_from_scalar(1.0, A::mul(s.ad_value(1679), s.ad_value(1680)))), 1.0 / ((1.0 - p.p183))));
        }

        s.v[1703] = if (s.v[1666] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1652] != 0.0) && (s.v[1697] != 0.0)) && (s.v[1698] != 0.0)) && (s.v[1703] != 0.0)) {
            s.store_sub_from_scalar_ad(1679, 1.0, A::scale(s.ad_value(1668), 1.0 / (p.p187)));
        }

        s.v[1704] = if (p.p184 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1652] != 0.0) && (s.v[1697] != 0.0)) && (s.v[1698] != 0.0)) && (s.v[1703] != 0.0)) && (s.v[1704] != 0.0)) {
            s.store_div_from_scalar_ad(1680, 1.0, A::sqrt(s.ad_value(1679)));
        }

        if (((((s.v[1652] != 0.0) && (s.v[1697] != 0.0)) && (s.v[1698] != 0.0)) && (s.v[1703] != 0.0)) && (!(s.v[1704] != 0.0))) {
            s.store_powf(1680, 1679, (-p.p184));
        }

        if ((((s.v[1652] != 0.0) && (s.v[1697] != 0.0)) && (s.v[1698] != 0.0)) && (s.v[1703] != 0.0)) {
            s.store_add_ad_rhs(284, 284, A::scale(A::mul(A::scale(s.ad_value(1666), p.p187), A::sub_from_scalar(1.0, A::mul(s.ad_value(1679), s.ad_value(1680)))), 1.0 / ((1.0 - p.p184))));
        }

        if (((s.v[1652] != 0.0) && (s.v[1697] != 0.0)) && (!(s.v[1698] != 0.0))) {
            s.store_add_ad_lhs(1654, A::add(s.ad_value(1662), s.ad_value(1664)), 1666);
        }

        if (((s.v[1652] != 0.0) && (s.v[1697] != 0.0)) && (!(s.v[1698] != 0.0))) {
            s.store_add_ad(1655, A::add(A::scale(s.ad_value(1662), (p.p182 * 1.0 / (p.p185))), A::scale(s.ad_value(1664), (p.p183 * 1.0 / (p.p186)))), A::scale(s.ad_value(1666), (p.p184 * 1.0 / (p.p187))));
        }

        if (((s.v[1652] != 0.0) && (s.v[1697] != 0.0)) && (!(s.v[1698] != 0.0))) {
            s.store_mul_ad_rhs(284, 1668, A::add(s.ad_value(1654), A::mul(A::scale(s.ad_value(1668), 0.5), s.ad_value(1655))));
        }

        if ((s.v[1652] != 0.0) && (!(s.v[1697] != 0.0))) {
            s.store_scalar(1666, (p.p181 * p.p4));
        }

        s.v[1705] = if (s.v[1668] < 0.0) { 1.0 } else { 0.0 };

        s.v[1706] = if (s.v[1662] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1652] != 0.0) && (!(s.v[1697] != 0.0))) && (s.v[1705] != 0.0)) && (s.v[1706] != 0.0)) {
            s.store_sub_from_scalar_ad(1679, 1.0, A::scale(s.ad_value(1668), 1.0 / (p.p185)));
        }

        s.v[1707] = if (p.p182 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1652] != 0.0) && (!(s.v[1697] != 0.0))) && (s.v[1705] != 0.0)) && (s.v[1706] != 0.0)) && (s.v[1707] != 0.0)) {
            s.store_div_from_scalar_ad(1680, 1.0, A::sqrt(s.ad_value(1679)));
        }

        if (((((s.v[1652] != 0.0) && (!(s.v[1697] != 0.0))) && (s.v[1705] != 0.0)) && (s.v[1706] != 0.0)) && (!(s.v[1707] != 0.0))) {
            s.store_powf(1680, 1679, (-p.p182));
        }

        if ((((s.v[1652] != 0.0) && (!(s.v[1697] != 0.0))) && (s.v[1705] != 0.0)) && (s.v[1706] != 0.0)) {
            s.store_scale_ad(284, A::mul(A::scale(s.ad_value(1662), p.p185), A::sub_from_scalar(1.0, A::mul(s.ad_value(1679), s.ad_value(1680)))), 1.0 / ((1.0 - p.p182)));
        }

        if ((((s.v[1652] != 0.0) && (!(s.v[1697] != 0.0))) && (s.v[1705] != 0.0)) && (!(s.v[1706] != 0.0))) {
            s.store_scalar(284, 0.0);
        }

        s.v[1708] = if (s.v[1666] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1652] != 0.0) && (!(s.v[1697] != 0.0))) && (s.v[1705] != 0.0)) && (s.v[1708] != 0.0)) {
            s.store_sub_from_scalar_ad(1679, 1.0, A::scale(s.ad_value(1668), 1.0 / (p.p187)));
        }

        s.v[1709] = if (p.p184 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1652] != 0.0) && (!(s.v[1697] != 0.0))) && (s.v[1705] != 0.0)) && (s.v[1708] != 0.0)) && (s.v[1709] != 0.0)) {
            s.store_div_from_scalar_ad(1680, 1.0, A::sqrt(s.ad_value(1679)));
        }

        if (((((s.v[1652] != 0.0) && (!(s.v[1697] != 0.0))) && (s.v[1705] != 0.0)) && (s.v[1708] != 0.0)) && (!(s.v[1709] != 0.0))) {
            s.store_powf(1680, 1679, (-p.p184));
        }

        if ((((s.v[1652] != 0.0) && (!(s.v[1697] != 0.0))) && (s.v[1705] != 0.0)) && (s.v[1708] != 0.0)) {
            s.store_add_ad_rhs(284, 284, A::scale(A::mul(A::scale(s.ad_value(1666), p.p187), A::sub_from_scalar(1.0, A::mul(s.ad_value(1679), s.ad_value(1680)))), 1.0 / ((1.0 - p.p184))));
        }

        if (((s.v[1652] != 0.0) && (!(s.v[1697] != 0.0))) && (!(s.v[1705] != 0.0))) {
            s.store_add(1654, 1662, 1666);
        }

        if (((s.v[1652] != 0.0) && (!(s.v[1697] != 0.0))) && (!(s.v[1705] != 0.0))) {
            s.store_add_ad(1655, A::scale(s.ad_value(1662), (p.p182 * 1.0 / (p.p185))), A::scale(s.ad_value(1666), (p.p184 * 1.0 / (p.p187))));
        }

        if (((s.v[1652] != 0.0) && (!(s.v[1697] != 0.0))) && (!(s.v[1705] != 0.0))) {
            s.store_mul_ad_rhs(284, 1668, A::add(s.ad_value(1654), A::mul(A::scale(s.ad_value(1668), 0.5), s.ad_value(1655))));
        }

        s.v[1710] = if (s.v[1663] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1652] != 0.0) && (s.v[1710] != 0.0)) {
            s.store_scale_ad(1657, A::mul(A::scale(s.ad_value(544), (-1.6021918e-19)), s.ad_value(1661)), p.p3);
        }

        if ((s.v[1652] != 0.0) && (s.v[1710] != 0.0)) {
            s.store_scale_ad(1659, A::neg(s.ad_value(1657)), 0.001);
        }

        if ((s.v[1652] != 0.0) && (s.v[1710] != 0.0)) {
            s.store_sub_ad_lhs(44, A::sub(A::neg(s.ad_value(1657)), A::neg(s.ad_value(283))), 1659);
        }

        if ((s.v[1652] != 0.0) && (s.v[1710] != 0.0)) {
            s.store_mul_ad_lhs(45, A::scale(A::neg(s.ad_value(1657)), 4.0), 1659);
        }

        if ((s.v[1652] != 0.0) && (s.v[1710] != 0.0)) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((s.v[1652] != 0.0) && (s.v[1710] != 0.0)) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((s.v[1652] != 0.0) && (s.v[1710] != 0.0)) {
            s.store_sub_ad(283, A::neg(s.ad_value(1657)), A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((s.v[1652] != 0.0) && (s.v[1710] != 0.0)) {
            s.store_scale(283, 283, (-1.0));
        }

        s.v[1711] = if (s.v[1662] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1652] != 0.0) && (s.v[1711] != 0.0)) {
            s.store_scale_ad(1658, A::mul(A::scale(s.ad_value(544), (-1.6021918e-19)), s.ad_value(1661)), p.p2);
        }

        if ((s.v[1652] != 0.0) && (s.v[1711] != 0.0)) {
            s.store_scale_ad(1660, A::neg(s.ad_value(1658)), 0.001);
        }

        if ((s.v[1652] != 0.0) && (s.v[1711] != 0.0)) {
            s.store_sub_ad_lhs(44, A::sub(A::neg(s.ad_value(1658)), A::neg(s.ad_value(284))), 1660);
        }

        if ((s.v[1652] != 0.0) && (s.v[1711] != 0.0)) {
            s.store_mul_ad_lhs(45, A::scale(A::neg(s.ad_value(1658)), 4.0), 1660);
        }

        if ((s.v[1652] != 0.0) && (s.v[1711] != 0.0)) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((s.v[1652] != 0.0) && (s.v[1711] != 0.0)) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((s.v[1652] != 0.0) && (s.v[1711] != 0.0)) {
            s.store_sub_ad(284, A::neg(s.ad_value(1658)), A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((s.v[1652] != 0.0) && (s.v[1711] != 0.0)) {
            s.store_scale(284, 284, (-1.0));
        }

        s.v[1744] = if ((p.p32 != 0.0) && (!(s.v[145] != 0.0))) { 1.0 } else { 0.0 };

        if (s.v[1744] != 0.0) {
            s.store_div_ad_lhs(1727, A::sub(s.ad_value(314), s.ad_value(161)), 441);
        }

        if (s.v[1744] != 0.0) {
            s.store_scaled_mul(1728, 251, 1727, 1e-5);
        }

        s.v[1745] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1744] != 0.0) && (s.v[1745] != 0.0)) {
            s.store_scalar(1729, 1.0);
        }

        s.v[1746] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1744] != 0.0) && (!(s.v[1745] != 0.0))) && (s.v[1746] != 0.0)) {
            s.copy_ad(1729, 1728);
        }

        if (((s.v[1744] != 0.0) && (!(s.v[1745] != 0.0))) && (!(s.v[1746] != 0.0))) {
            s.store_powf(1729, 1728, (p.p113 - 1.0));
        }

        if (s.v[1744] != 0.0) {
            s.store_mul(1730, 1728, 1729);
        }

        if (s.v[1744] != 0.0) {
            s.store_offset(1731, 1730, 1.0);
        }

        if (s.v[1744] != 0.0) {
            s.store_powf(1732, 1731, (((-1.0) / p.p113) - 1.0));
        }

        if (s.v[1744] != 0.0) {
            s.store_mul(1733, 1731, 1732);
        }

        if (s.v[1744] != 0.0) {
            s.store_mul(293, 251, 1733);
        }

        if (s.v[1744] != 0.0) {
            s.store_scaled_add(1735, 250, 293, 0.5);
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
        if (s.v[1744] != 0.0) {
            s.store_square(1734, 190);
        }

        if (s.v[1744] != 0.0) {
            let assign33710_ad_e48913: A = A::add(A::add(A::mul(A::mul(A::add(A::offset(A::scale(s.ad_value(190), 3.0), 1.0), A::scale(s.ad_value(1734), 6.0)), s.ad_value(293)), s.ad_value(293)), A::mul(A::mul(A::add(A::offset(A::scale(s.ad_value(190), 4.0), 3.0), A::scale(s.ad_value(1734), 3.0)), s.ad_value(293)), s.ad_value(250))), A::mul(A::mul(A::add(A::offset(A::scale(s.ad_value(190), 3.0), 6.0), s.ad_value(1734)), s.ad_value(250)), s.ad_value(250)));
            s.store_div_ad(292, A::mul(A::mul(A::mul(A::mul(s.ad_value(107), s.ad_value(323)), s.ad_value(192)), s.ad_value(250)), assign33710_ad_e48913), A::mul(A::mul(A::mul(A::scale(s.ad_value(441), 15.0), A::offset(s.ad_value(190), 1.0)), s.ad_value(1735)), s.ad_value(1735)));
        }

        if (!(s.v[1744] != 0.0)) {
            s.store_scalar(292, 0.0);
        }

        s.v[1747] = if ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (s.v[146] == 1.0)) && (!(s.v[145] != 0.0))) { 1.0 } else { 0.0 };

        if (s.v[1747] != 0.0) {
            s.store_sqrt(298, 296);
        }

        if (s.v[1747] != 0.0) {
            s.store_add(1736, 192, 298);
        }

        if (s.v[1747] != 0.0) {
            s.store_square(1737, 294);
        }

        if (s.v[1747] != 0.0) {
            s.store_square(1738, 296);
        }

        if (s.v[1747] != 0.0) {
            s.store_mul_ad_lhs(1739, A::scale(s.ad_value(294), 42.0), 296);
        }

        if (s.v[1747] != 0.0) {
            s.store_add_ad_rhs(1739, 1739, A::scale(A::add(s.ad_value(1737), s.ad_value(1738)), 4.0));
        }

        if (s.v[1747] != 0.0) {
            s.store_add_ad_rhs(1739, 1739, A::mul(A::mul(A::scale(s.ad_value(298), 20.0), s.ad_value(192)), A::add(s.ad_value(294), s.ad_value(296))));
        }

        if (s.v[1747] != 0.0) {
            s.store_square(1740, 1736);
        }

        if (s.v[1747] != 0.0) {
            s.store_square(1732, 1740);
        }

        if (s.v[1747] != 0.0) {
            s.store_div_ad_rhs(299, 1739, A::mul(s.ad_value(1732), s.ad_value(1736)));
        }

        if (s.v[1747] != 0.0) {
            s.store_mul_ad_lhs(300, A::mul(A::div(s.ad_value(107), s.ad_value(441)), s.ad_value(250)), 323);
        }

        s.store_add(199, 199, 265);

        s.v[1748] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1748] != 0.0) {
            s.store_add(271, 531, 532);
        }

        if ((s.v[1748] != 0.0) && (s.v[564] != 0.0)) {
            s.store_offset(271, 271, (-(p.p168 * s.v[99])));
        }

        if (s.v[1748] != 0.0) {
            s.store_mul_ad(272, A::neg(s.ad_value(271)), A::sub(s.ad_value(158), s.ad_value(513)));
        }

        if (s.v[1748] != 0.0) {
            s.store_scalar(276, 0.0);
        }

        if (s.v[1748] != 0.0) {
            s.store_mul_ad(274, A::scale(s.ad_value(276), p.p9), A::offset(s.ad_value(518), s.v[101]));
        }

        if (s.v[1748] != 0.0) {
            s.store_mul_ad(275, A::scale(s.ad_value(276), p.p9), A::offset(s.ad_value(519), s.v[101]));
        }

        if (s.v[1748] != 0.0) {
            s.store_mul_ad_rhs(277, 274, A::sub(s.ad_value(158), s.ad_value(157)));
        }

        if (s.v[1748] != 0.0) {
            s.store_mul(278, 275, 158);
        }

        if (s.v[1748] != 0.0) {
            s.store_mul_ad(279, A::scale(s.ad_value(276), (p.p19 * p.p9)), A::sub(s.ad_value(158), s.ad_value(513)));
        }

        if (s.v[1748] != 0.0) {
            s.store_add(268, 268, 277);
        }

        if (s.v[1748] != 0.0) {
            s.store_add(267, 267, 278);
        }

        if (s.v[1748] != 0.0) {
            s.store_add(272, 272, 279);
        }

        if ((!(s.v[1748] != 0.0)) && (s.v[564] != 0.0)) {
            s.store_scalar(271, ((-p.p168) * s.v[99]));
        }

        if ((!(s.v[1748] != 0.0)) && (s.v[564] != 0.0)) {
            s.store_mul_ad(272, A::neg(s.ad_value(271)), A::sub(s.ad_value(158), s.ad_value(513)));
        }

        if ((!(s.v[1748] != 0.0)) && (!(s.v[564] != 0.0))) {
            s.store_scalar(271, 0.0);
        }

        if ((!(s.v[1748] != 0.0)) && (!(s.v[564] != 0.0))) {
            s.store_scalar(272, 0.0);
        }

        if (!(s.v[1748] != 0.0)) {
            s.store_scalar(273, 0.0);
        }

        if (!(s.v[1748] != 0.0)) {
            s.copy_ad(274, 273);
        }

        if (!(s.v[1748] != 0.0)) {
            s.copy_ad(275, 273);
        }

        if (!(s.v[1748] != 0.0)) {
            s.store_mul_ad_rhs(277, 274, A::sub(s.ad_value(158), s.ad_value(157)));
        }

        if (!(s.v[1748] != 0.0)) {
            s.store_mul(278, 275, 158);
        }

        if (!(s.v[1748] != 0.0)) {
            s.store_add(268, 268, 277);
        }

        if (!(s.v[1748] != 0.0)) {
            s.store_add(267, 267, 278);
        }

        s.store_scale(9, 199, s.v[451]);

        if (s.v[85] != 0.0) {
            s.store_scalar(24, 0.0);
        }

        if (s.v[85] != 0.0) {
            s.store_scalar(23, 0.0);
        }

        s.v[1749] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[85] != 0.0) && (s.v[1749] != 0.0)) {
            s.store_scalar(25, 0.0);
        }

        if ((s.v[85] != 0.0) && (s.v[1749] != 0.0)) {
            s.copy_ad(556, 438);
        }

        if ((s.v[85] != 0.0) && (!(s.v[1749] != 0.0))) {
            s.store_scalar(554, 0.0);
        }

        s.v[1750] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[85] != 0.0)) && (s.v[1750] != 0.0)) {
            s.store_scale_ad(23, A::sub(A::neg(s.ad_value(196)), s.ad_value(197)), s.v[451]);
        }

        if ((!(s.v[85] != 0.0)) && (s.v[1750] != 0.0)) {
            s.store_scale(24, 198, s.v[451]);
        }

        if ((!(s.v[85] != 0.0)) && (s.v[1750] != 0.0)) {
            s.store_scaled_sub(25, 197, 198, s.v[451]);
        }

        if ((!(s.v[85] != 0.0)) && (!(s.v[1750] != 0.0))) {
            s.store_scale_ad(23, A::sub(A::sub(A::sub(A::neg(s.ad_value(392)), s.ad_value(197)), s.ad_value(476)), s.ad_value(477)), s.v[451]);
        }

        if ((!(s.v[85] != 0.0)) && (!(s.v[1750] != 0.0))) {
            s.store_scaled_add(24, 198, 477, s.v[451]);
        }

        if ((!(s.v[85] != 0.0)) && (!(s.v[1750] != 0.0))) {
            s.store_scale_ad(25, A::add(A::sub(s.ad_value(197), s.ad_value(198)), s.ad_value(476)), s.v[451]);
        }

        s.v[1756] = if (p.p64 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1756] != 0.0) {
            s.store_scalar(280, 0.0);
        }

        if (!(s.v[1756] != 0.0)) {
            s.store_add_ad_lhs(1751, A::scale(s.ad_value(315), s.v[97]), 161);
        }

        s.v[1757] = if (s.v[1751] > s.v[314]) { 1.0 } else { 0.0 };

        if ((!(s.v[1756] != 0.0)) && (s.v[1757] != 0.0)) {
            s.copy_ad(1751, 314);
        }

        if (!(s.v[1756] != 0.0)) {
            s.store_add_ad(1752, A::scale(A::add(s.ad_value(157), s.ad_value(161)), s.v[317]), A::scale(s.ad_value(1751), (1.0 - s.v[317])));
        }

        if (!(s.v[1756] != 0.0)) {
            s.store_sqrt_ad(1753, A::div_from_scalar((2.0 * 1.034943e-10), s.ad_value(229)));
        }

        if (!(s.v[1756] != 0.0)) {
            s.store_scale(1754, 1753, 1.3);
        }

        if (!(s.v[1756] != 0.0)) {
            s.store_mul_ad_lhs(1755, A::scale(s.ad_value(108), 1.034943e-10), 1754);
        }

        if (!(s.v[1756] != 0.0)) {
            s.store_mul_ad_lhs(280, A::sub(A::scale(A::sub(A::add(s.ad_value(161), s.ad_value(157)), s.ad_value(1752)), 1.0 / (p.p64)), s.ad_value(315)), 1755);
        }

        s.v[1758] = if (p.p65 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1758] != 0.0) {
            s.store_add_ad_rhs(280, 280, A::mul(s.ad_value(135), s.ad_value(513)));
        }

        s.v[1759] = if (p.p24 == 1.0) { 1.0 } else { 0.0 };

        s.v[1760] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1759] != 0.0) && (s.v[1760] != 0.0)) {
            s.store_sub_ad_lhs(471, A::sub(A::sub(A::neg(s.ad_value(463)), s.ad_value(464)), s.ad_value(467)), 468);
        }

        if ((s.v[1759] != 0.0) && (s.v[1760] != 0.0)) {
            s.store_add(472, 466, 470);
        }

        if ((s.v[1759] != 0.0) && (s.v[1760] != 0.0)) {
            s.store_add(473, 465, 469);
        }

        if ((s.v[1759] != 0.0) && (s.v[1760] != 0.0)) {
            s.store_add_ad_rhs(23, 23, A::scale(A::add(A::sub(A::sub(A::sub(A::add(A::add(s.ad_value(268), s.ad_value(267)), s.ad_value(272)), s.ad_value(280)), s.ad_value(455)), s.ad_value(454)), s.ad_value(471)), s.v[451]));
        }

        if ((s.v[1759] != 0.0) && (s.v[1760] != 0.0)) {
            s.store_add_ad_rhs(24, 24, A::scale(A::add(A::add(A::sub(s.ad_value(280), s.ad_value(268)), s.ad_value(456)), s.ad_value(472)), s.v[451]));
        }

        if ((s.v[1759] != 0.0) && (s.v[1760] != 0.0)) {
            s.store_add_ad_rhs(25, 25, A::scale(A::add(A::sub(s.ad_value(457), s.ad_value(267)), s.ad_value(473)), s.v[451]));
        }

        if ((s.v[1759] != 0.0) && (!(s.v[1760] != 0.0))) {
            s.store_add_ad_rhs(23, 23, A::scale(A::sub(A::sub(A::sub(A::add(A::add(s.ad_value(268), s.ad_value(267)), s.ad_value(272)), s.ad_value(280)), s.ad_value(455)), s.ad_value(454)), s.v[451]));
        }

        if ((s.v[1759] != 0.0) && (!(s.v[1760] != 0.0))) {
            s.store_add_ad_rhs(24, 24, A::scale(A::add(A::sub(s.ad_value(280), s.ad_value(268)), s.ad_value(456)), s.v[451]));
        }

        if ((s.v[1759] != 0.0) && (!(s.v[1760] != 0.0))) {
            s.store_add_ad_rhs(25, 25, A::scale(A::sub(s.ad_value(457), s.ad_value(267)), s.v[451]));
        }

        s.v[1761] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1761] != 0.0) {
            s.store_scale(36, 281, s.v[451]);
        }

        if (s.v[1761] != 0.0) {
            s.store_scale(35, 282, s.v[451]);
        }

        if (s.v[1761] != 0.0) {
            s.store_scale(560, 284, s.v[451]);
        }

        if (s.v[1761] != 0.0) {
            s.store_scale(561, 283, s.v[451]);
        }

        if (!(s.v[1761] != 0.0)) {
            s.store_scalar(36, 0.0);
        }

        if (!(s.v[1761] != 0.0)) {
            s.store_scalar(35, 0.0);
        }

        if (!(s.v[1761] != 0.0)) {
            s.store_scalar(560, 0.0);
        }

        if (!(s.v[1761] != 0.0)) {
            s.store_scalar(561, 0.0);
        }

        s.v[1762] = if (p.p25 != 1.0) { 1.0 } else { 0.0 };

        if (s.v[1762] != 0.0) {
            s.store_scalar(557, 0.0);
        }

        if (!(s.v[1762] != 0.0)) {
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

        s.v[1771] = if ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (s.v[146] == 1.0)) && (!(s.v[145] != 0.0))) { 1.0 } else { 0.0 };

        if (s.v[1771] != 0.0) {
            s.store_scale_ad(1765, A::mul(A::scale(s.ad_value(323), 1e-6), s.ad_value(108)), s.v[98]);
        }

        if (s.v[1771] != 0.0) {
            s.store_scale(1766, 555, 1.0 / (s.v[451]));
        }

        if (s.v[1771] != 0.0) {
            s.store_div_ad_lhs(1767, A::mul(A::mul(A::scale(s.ad_value(227), (0.1185185185185185 * 1.6021918e-19)), s.ad_value(1766)), s.ad_value(1766)), 300);
        }

        s.v[1772] = if ((s.v[297] > (10.0 * 2.220446049250313e-16)) && (s.v[157] > (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if ((s.v[1771] != 0.0) && (s.v[1772] != 0.0)) {
            s.store_div(1768, 251, 250);
        }

        if ((s.v[1771] != 0.0) && (s.v[1772] != 0.0)) {
            s.store_div_ad_lhs(1769, A::sub(A::div(s.ad_value(251), s.ad_value(293)), s.ad_value(1768)), 157);
        }

        if ((s.v[1771] != 0.0) && (s.v[1772] != 0.0)) {
            s.store_add_ad_rhs(1770, 1768, A::div(A::mul(A::scale(s.ad_value(1769), 0.6666666666666667), A::add(A::add(s.ad_value(294), A::mul(s.ad_value(192), s.ad_value(298))), s.ad_value(296))), A::add(s.ad_value(192), s.ad_value(298))));
        }

        if ((s.v[1771] != 0.0) && (!(s.v[1772] != 0.0))) {
            s.store_div(1770, 251, 293);
        }

        if (s.v[1771] != 0.0) {
            s.store_mul_ad_lhs(558, A::mul(A::scale(s.ad_value(1767), s.v[451]), s.ad_value(299)), 1770);
        }

        if (s.v[1771] != 0.0) {
            s.store_ad(558, &{
                if (((-s.v[1766]) > s.v[1765]) && (s.v[558] > 0.0)) {
                    s.ad_value(558)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (!(s.v[1771] != 0.0)) {
            s.store_scalar(558, 0.0);
        }

        s.v[1773] = if (p.p259 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1773] != 0.0) {
            s.store_scalar(3, 1.0);
        }

        s.v[1793] = if (s.v[3] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1773] != 0.0) && (s.v[1793] != 0.0)) {
            s.store_scalar(1777, p.p266);
        }

        if ((s.v[1773] != 0.0) && (s.v[1793] != 0.0)) {
            s.store_scalar(1778, p.p268);
        }

        if ((s.v[1773] != 0.0) && (s.v[1793] != 0.0)) {
            s.store_scalar(1779, p.p273);
        }

        if ((s.v[1773] != 0.0) && (s.v[1793] != 0.0)) {
            s.store_scalar(1783, p.p258);
        }

        if ((s.v[1773] != 0.0) && (s.v[1793] != 0.0)) {
            s.store_ad(1781, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(2)), p.p50));
        }

        if ((s.v[1773] != 0.0) && (!(s.v[1793] != 0.0))) {
            s.store_scalar(1777, p.p265);
        }

        if ((s.v[1773] != 0.0) && (!(s.v[1793] != 0.0))) {
            s.store_scalar(1778, p.p267);
        }

        if ((s.v[1773] != 0.0) && (!(s.v[1793] != 0.0))) {
            s.store_scalar(1779, p.p272);
        }

        if ((s.v[1773] != 0.0) && (!(s.v[1793] != 0.0))) {
            s.store_scalar(1783, p.p257);
        }

        if ((s.v[1773] != 0.0) && (!(s.v[1793] != 0.0))) {
            s.store_ad(1781, &A::scale(A::voltage(ctx, &nodes, Some(0), Some(6)), p.p50));
        }

        if (s.v[1773] != 0.0) {
            s.store_scale(1777, 1777, 0.0001);
        }

        if (s.v[1773] != 0.0) {
            s.store_scale(1778, 1778, 0.01);
        }

        if (s.v[1773] != 0.0) {
            s.store_scale(1782, 429, 1.0 / (s.v[81]));
        }

        if (s.v[1773] != 0.0) {
            s.store_powf(328, 1782, p.p269);
        }

        if (s.v[1773] != 0.0) {
            s.store_div(1785, 1777, 328);
        }

        if (s.v[1773] != 0.0) {
            s.store_sub_ad(327, A::add(A::offset(A::scale(s.ad_value(1782), 0.4), 1.8), A::mul(A::scale(s.ad_value(1782), 0.1), s.ad_value(1782))), A::scale(A::sub_from_scalar(1.0, s.ad_value(1782)), p.p270));
        }

        if (s.v[1773] != 0.0) {
            s.store_div(1786, 1778, 327);
        }

        if (s.v[1773] != 0.0) {
            s.store_add_ad_rhs(1779, 1779, A::scale(A::offset(s.ad_value(429), (-s.v[81])), p.p274));
        }

        if (s.v[1773] != 0.0) {
            s.store_scalar(1774, (1.0 + (p.p279 / ((s.v[100]) as f64).powf(p.p280))));
        }

        if (s.v[1773] != 0.0) {
            s.store_scalar(1776, (1.0 + (p.p277 / ((s.v[100]) as f64).powf(p.p278))));
        }

        if (s.v[1773] != 0.0) {
            s.store_scalar(1775, (1.0 + (p.p275 / ((s.v[109]) as f64).powf(p.p276))));
        }

        if (s.v[1773] != 0.0) {
            s.store_mul(1785, 1785, 1774);
        }

        if (s.v[1773] != 0.0) {
            s.store_offset_ad(1786, A::mul(A::mul(s.ad_value(1786), s.ad_value(1775)), s.ad_value(1776)), 1e-50);
        }

        if (s.v[1773] != 0.0) {
            s.store_div(1787, 1781, 1783);
        }

        if (s.v[1773] != 0.0) {
            s.store_mul(1788, 1785, 1787);
        }

        s.v[1794] = if (s.v[1781] >= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1773] != 0.0) && (s.v[1794] != 0.0)) {
            s.store_div(328, 1788, 1786);
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
        if ((s.v[1773] != 0.0) && (!(s.v[1794] != 0.0))) {
            s.store_div_ad_lhs(328, A::neg(s.ad_value(1788)), 1786);
        }

        s.v[1795] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1779]) && (s.v[1779] <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1773] != 0.0) && (s.v[1795] != 0.0)) {
            s.store_scalar(330, 1.0);
        }

        s.v[1796] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1779]) && (s.v[1779] <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1773] != 0.0) && (!(s.v[1795] != 0.0))) && (s.v[1796] != 0.0)) {
            s.copy_ad(330, 328);
        }

        if (((s.v[1773] != 0.0) && (!(s.v[1795] != 0.0))) && (!(s.v[1796] != 0.0))) {
            s.store_ad(330, &A::pow(s.ad_value(328), A::offset(s.ad_value(1779), (-1.0))));
        }

        if (s.v[1773] != 0.0) {
            s.store_mul(329, 328, 330);
        }

        if (s.v[1773] != 0.0) {
            s.store_offset(331, 329, 1.0);
        }

        s.v[1797] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1779]) && (s.v[1779] <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1773] != 0.0) && (s.v[1797] != 0.0)) {
            s.store_div_from_scalar(332, 1.0, 331);
        }

        s.v[1798] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1779]) && (s.v[1779] <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1773] != 0.0) && (!(s.v[1797] != 0.0))) && (s.v[1798] != 0.0)) {
            s.store_div_from_scalar_ad(332, 1.0, A::sqrt(s.ad_value(331)));
        }

        if (((s.v[1773] != 0.0) && (!(s.v[1797] != 0.0))) && (!(s.v[1798] != 0.0))) {
            s.store_ad(333, &A::pow(s.ad_value(331), A::offset(A::div_from_scalar((-1.0), s.ad_value(1779)), (-1.0))));
        }

        if (((s.v[1773] != 0.0) && (!(s.v[1797] != 0.0))) && (!(s.v[1798] != 0.0))) {
            s.store_mul(332, 331, 333);
        }

        if (s.v[1773] != 0.0) {
            s.store_div_from_scalar(328, 1.6021918e-19, 1783);
        }

        s.v[1801] = if (p.p260 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1801] != 0.0) {
            s.store_scalar(3, 2.0);
        }

        s.v[1821] = if (s.v[3] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1801] != 0.0) && (s.v[1821] != 0.0)) {
            s.store_scalar(1805, p.p266);
        }

        if ((s.v[1801] != 0.0) && (s.v[1821] != 0.0)) {
            s.store_scalar(1806, p.p268);
        }

        if ((s.v[1801] != 0.0) && (s.v[1821] != 0.0)) {
            s.store_scalar(1807, p.p273);
        }

        if ((s.v[1801] != 0.0) && (s.v[1821] != 0.0)) {
            s.store_scalar(1811, p.p258);
        }

        if ((s.v[1801] != 0.0) && (s.v[1821] != 0.0)) {
            s.store_ad(1809, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(2)), p.p50));
        }

        if ((s.v[1801] != 0.0) && (!(s.v[1821] != 0.0))) {
            s.store_scalar(1805, p.p265);
        }

        if ((s.v[1801] != 0.0) && (!(s.v[1821] != 0.0))) {
            s.store_scalar(1806, p.p267);
        }

        if ((s.v[1801] != 0.0) && (!(s.v[1821] != 0.0))) {
            s.store_scalar(1807, p.p272);
        }

        if ((s.v[1801] != 0.0) && (!(s.v[1821] != 0.0))) {
            s.store_scalar(1811, p.p257);
        }

        if ((s.v[1801] != 0.0) && (!(s.v[1821] != 0.0))) {
            s.store_ad(1809, &A::scale(A::voltage(ctx, &nodes, Some(0), Some(6)), p.p50));
        }

        if (s.v[1801] != 0.0) {
            s.store_scale(1805, 1805, 0.0001);
        }

        if (s.v[1801] != 0.0) {
            s.store_scale(1806, 1806, 0.01);
        }

        if (s.v[1801] != 0.0) {
            s.store_scale(1810, 429, 1.0 / (s.v[81]));
        }

        if (s.v[1801] != 0.0) {
            s.store_powf(328, 1810, p.p269);
        }

        if (s.v[1801] != 0.0) {
            s.store_div(1813, 1805, 328);
        }

        if (s.v[1801] != 0.0) {
            s.store_sub_ad(327, A::add(A::offset(A::scale(s.ad_value(1810), 0.4), 1.8), A::mul(A::scale(s.ad_value(1810), 0.1), s.ad_value(1810))), A::scale(A::sub_from_scalar(1.0, s.ad_value(1810)), p.p270));
        }

        if (s.v[1801] != 0.0) {
            s.store_div(1814, 1806, 327);
        }

        if (s.v[1801] != 0.0) {
            s.store_add_ad_rhs(1807, 1807, A::scale(A::offset(s.ad_value(429), (-s.v[81])), p.p274));
        }

        if (s.v[1801] != 0.0) {
            s.store_scalar(1802, (1.0 + (p.p279 / ((s.v[100]) as f64).powf(p.p280))));
        }

        if (s.v[1801] != 0.0) {
            s.store_scalar(1804, (1.0 + (p.p277 / ((s.v[100]) as f64).powf(p.p278))));
        }

        if (s.v[1801] != 0.0) {
            s.store_scalar(1803, (1.0 + (p.p275 / ((s.v[109]) as f64).powf(p.p276))));
        }

        if (s.v[1801] != 0.0) {
            s.store_mul(1813, 1813, 1802);
        }

        if (s.v[1801] != 0.0) {
            s.store_offset_ad(1814, A::mul(A::mul(s.ad_value(1814), s.ad_value(1803)), s.ad_value(1804)), 1e-50);
        }

        if (s.v[1801] != 0.0) {
            s.store_div(1815, 1809, 1811);
        }

        if (s.v[1801] != 0.0) {
            s.store_mul(1816, 1813, 1815);
        }

        s.v[1822] = if (s.v[1809] >= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1801] != 0.0) && (s.v[1822] != 0.0)) {
            s.store_div(328, 1816, 1814);
        }

        if ((s.v[1801] != 0.0) && (!(s.v[1822] != 0.0))) {
            s.store_div_ad_lhs(328, A::neg(s.ad_value(1816)), 1814);
        }

        s.v[1823] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1807]) && (s.v[1807] <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1801] != 0.0) && (s.v[1823] != 0.0)) {
            s.store_scalar(330, 1.0);
        }

        s.v[1824] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1807]) && (s.v[1807] <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1801] != 0.0) && (!(s.v[1823] != 0.0))) && (s.v[1824] != 0.0)) {
            s.copy_ad(330, 328);
        }

        if (((s.v[1801] != 0.0) && (!(s.v[1823] != 0.0))) && (!(s.v[1824] != 0.0))) {
            s.store_ad(330, &A::pow(s.ad_value(328), A::offset(s.ad_value(1807), (-1.0))));
        }

        if (s.v[1801] != 0.0) {
            s.store_mul(329, 328, 330);
        }

        if (s.v[1801] != 0.0) {
            s.store_offset(331, 329, 1.0);
        }

        s.v[1825] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1807]) && (s.v[1807] <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1801] != 0.0) && (s.v[1825] != 0.0)) {
            s.store_div_from_scalar(332, 1.0, 331);
        }

        s.v[1826] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1807]) && (s.v[1807] <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1801] != 0.0) && (!(s.v[1825] != 0.0))) && (s.v[1826] != 0.0)) {
            s.store_div_from_scalar_ad(332, 1.0, A::sqrt(s.ad_value(331)));
        }

        if (((s.v[1801] != 0.0) && (!(s.v[1825] != 0.0))) && (!(s.v[1826] != 0.0))) {
            s.store_ad(333, &A::pow(s.ad_value(331), A::offset(A::div_from_scalar((-1.0), s.ad_value(1807)), (-1.0))));
        }

        if (((s.v[1801] != 0.0) && (!(s.v[1825] != 0.0))) && (!(s.v[1826] != 0.0))) {
            s.store_mul(332, 331, 333);
        }

        if (s.v[1801] != 0.0) {
            s.store_div_from_scalar(328, 1.6021918e-19, 1811);
        }

        s.v[1829] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1829] != 0.0) && (s.v[85] != 0.0)) {
            s.store_ad(438, &{
                if (s.v[613] == 1.0) {
                    s.ad_value(556)
                } else {
                    A::sub_from_scalar(1.0, s.ad_value(556))
                }
            });
        }

        if ((s.v[1829] != 0.0) && (s.v[85] != 0.0)) {
            s.store_add_ad_lhs(584, A::mul(s.ad_value(580), s.ad_value(438)), 473);
        }

        if ((s.v[1829] != 0.0) && (s.v[85] != 0.0)) {
            s.store_add_ad_lhs(585, A::mul(s.ad_value(580), A::sub_from_scalar(1.0, s.ad_value(438))), 473);
        }

        if ((s.v[1829] != 0.0) && (s.v[85] != 0.0)) {
            s.store_add_ad_lhs(586, A::sub(A::neg(s.ad_value(580)), s.ad_value(581)), 471);
        }

        if ((s.v[1829] != 0.0) && (!(s.v[85] != 0.0))) {
            s.store_scalar(584, 0.0);
        }

        if ((s.v[1829] != 0.0) && (!(s.v[85] != 0.0))) {
            s.store_scalar(585, 0.0);
        }

        if ((s.v[1829] != 0.0) && (!(s.v[85] != 0.0))) {
            s.store_scalar(586, 0.0);
        }

        if ((s.v[1829] != 0.0) && (!(s.v[85] != 0.0))) {
            s.store_scalar(581, 0.0);
        }

        if ((!(s.v[1829] != 0.0)) && (s.v[85] != 0.0)) {
            s.store_sub_ad_lhs(586, A::sub(A::neg(s.ad_value(584)), s.ad_value(585)), 581);
        }

        if ((!(s.v[1829] != 0.0)) && (!(s.v[85] != 0.0))) {
            s.store_scalar(584, 0.0);
        }

        if ((!(s.v[1829] != 0.0)) && (!(s.v[85] != 0.0))) {
            s.store_scalar(585, 0.0);
        }

        if ((!(s.v[1829] != 0.0)) && (!(s.v[85] != 0.0))) {
            s.store_scalar(586, 0.0);
        }

        if ((!(s.v[1829] != 0.0)) && (!(s.v[85] != 0.0))) {
            s.store_scalar(581, 0.0);
        }

        s.v[1834] = if (s.v[613] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1834] != 0.0) {
            s.copy_ad(199, 9);
        }

        if (s.v[1834] != 0.0) {
            s.copy_ad(263, 557);
        }

        if (s.v[1834] != 0.0) {
            s.store_add(594, 23, 586);
        }

        if (s.v[1834] != 0.0) {
            s.store_add(198, 24, 584);
        }

        if (s.v[1834] != 0.0) {
            s.store_neg_ad(554, A::add(A::add(s.ad_value(23), s.ad_value(24)), s.ad_value(25)));
        }

        if (s.v[1834] != 0.0) {
            s.store_add(196, 554, 581);
        }

        if (!(s.v[1834] != 0.0)) {
            s.store_neg(199, 9);
        }

        if (!(s.v[1834] != 0.0)) {
            s.store_scalar(263, 0.0);
        }

        if (!(s.v[1834] != 0.0)) {
            s.store_add(594, 23, 586);
        }

        if (!(s.v[1834] != 0.0)) {
            s.store_add(198, 25, 585);
        }

        if (!(s.v[1834] != 0.0)) {
            s.store_neg_ad(554, A::add(A::add(s.ad_value(23), s.ad_value(24)), s.ad_value(25)));
        }

        if (!(s.v[1834] != 0.0)) {
            s.store_add(196, 554, 581);
        }

        s.v[1835] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1835] != 0.0) {
            s.copy_ad(282, 35);
        }

        if (s.v[1835] != 0.0) {
            s.copy_ad(284, 560);
        }

        if (s.v[1835] != 0.0) {
            s.copy_ad(281, 36);
        }

        if (s.v[1835] != 0.0) {
            s.copy_ad(283, 561);
        }

        s.v[1836] = if ((p.p38 == 1.0) && (s.v[67] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1836] != 0.0) {
            s.copy_ad(563, 542);
        }

        if (!(s.v[1836] != 0.0)) {
            s.store_scalar(563, 0.0);
        }

        s.copy_ad(9, 199);

        s.store_scalar(27, A::ddx_projection(&s.ad_value(594), Some(6), None));

        s.store_scale(27, 27, p.p50);

        s.store_scalar(28, A::ddx_projection(&s.ad_value(594), Some(7), None));

        s.store_scale(28, 28, p.p50);

        s.v[1838] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1838] != 0.0) {
            s.store_scale(35, 282, p.p50);
        }

        if (s.v[1838] != 0.0) {
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

        s.v[1846] = if ((p.p38 > 0.0) && (p.p242 > 0.0)) { 1.0 } else { 0.0 };

        s.v[1847] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        s.v[1848] = if ((p.p37 != 0.0) || ((p.p25 == 1.0) && (p.p26 == 2.0))) { 1.0 } else { 0.0 };

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
