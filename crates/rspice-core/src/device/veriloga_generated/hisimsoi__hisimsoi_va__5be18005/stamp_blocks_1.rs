#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_block_16(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((!(s.v[737] != 0.0)) && (s.v[1089] != 0.0)) && (s.v[1128] != 0.0)) && (!(s.v[1129] != 0.0))) {
            s.copy_ad(436, 425);
        }

        if (((!(s.v[737] != 0.0)) && (s.v[1089] != 0.0)) && (!(s.v[1128] != 0.0))) {
            s.store_scaled_add(436, 355, 358, (-0.5));
        }

        s.v[1133] = if (s.v[612] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1133] != 0.0) {
            s.store_offset(480, 190, 0.5);
        }

        if (s.v[1133] != 0.0) {
            s.store_mul(481, 479, 478);
        }

        if (s.v[1133] != 0.0) {
            s.store_div_ad_lhs(482, A::scale(s.ad_value(480), 0.4), 481);
        }

        if (s.v[1133] != 0.0) {
            s.store_sub_from_scalar(438, 0.6, 482);
        }

        s.v[1134] = if (s.v[438] > (0.5 + 1e-8)) { 1.0 } else { 0.0 };

        if ((s.v[1133] != 0.0) && (s.v[1134] != 0.0)) {
            s.store_scalar(438, 0.5);
        }

        if (s.v[1133] != 0.0) {
            s.copy_ad(439, 438);
        }

        if (s.v[1133] != 0.0) {
            s.store_scalar(438, 0.5);
        }

        s.v[1136] = if (s.v[145] == 0.0) { 1.0 } else { 0.0 };

        s.v[1152] = if ((p.p190 < (10.0 * 2.220446049250313e-16)) && (p.p191 < (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if (((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (s.v[1152] != 0.0)) {
            s.store_scalar(316, 0.0);
        }

        if (((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (s.v[1152] != 0.0)) {
            s.copy_ad(314, 162);
        }

        s.v[1153] = if (s.v[314] > ((s.v[161] + s.v[173]) - (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if ((((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (s.v[1152] != 0.0)) && (s.v[1153] != 0.0)) {
            s.store_offset_ad(314, A::add(s.ad_value(161), s.ad_value(173)), (-(10.0 * 2.220446049250313e-16)));
        }

        if (((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (!(s.v[1152] != 0.0))) {
            s.store_scalar(1151, (if (p.p43 == 1.0) { p.p237 } else { s.v[402] }));
        }

        if (((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (!(s.v[1152] != 0.0))) {
            s.store_div_from_scalar(1137, 1.0, 1151);
        }

        if (((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (!(s.v[1152] != 0.0))) {
            s.store_mul(1138, 244, 1137);
        }

        if (((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (!(s.v[1152] != 0.0))) {
            s.store_scale(1139, 1138, p.p191);
        }

        if (((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (!(s.v[1152] != 0.0))) {
            s.store_add_ad_lhs(1142, A::mul(s.ad_value(80), s.ad_value(229)), 1139);
        }

        if (((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (!(s.v[1152] != 0.0))) {
            s.store_div_from_scalar(1138, 1.0, 1142);
        }

        if (((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (!(s.v[1152] != 0.0))) {
            s.store_scale(1141, 1138, 1.034943e-10);
        }

        if (((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (!(s.v[1152] != 0.0))) {
            s.store_scalar(1138, (1.0 - p.p189));
        }

        if (((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (!(s.v[1152] != 0.0))) {
            s.store_add_ad(314, A::scale(A::add(s.ad_value(157), s.ad_value(161)), p.p189), A::mul(s.ad_value(1138), s.ad_value(162)));
        }

        s.v[1154] = if (s.v[314] > ((s.v[161] + s.v[173]) - (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if ((((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (!(s.v[1152] != 0.0))) && (s.v[1154] != 0.0)) {
            s.store_offset_ad(314, A::add(s.ad_value(161), s.ad_value(173)), (-(10.0 * 2.220446049250313e-16)));
        }

        if (((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (!(s.v[1152] != 0.0))) {
            s.store_sub(1144, 314, 162);
        }

        if (((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (!(s.v[1152] != 0.0))) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1144)), ((4.0 * 0.001) * 0.001)));
        }

        if (((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (!(s.v[1152] != 0.0))) {
            s.store_offset_ad(1143, A::scale(A::add(s.ad_value(1144), s.ad_value(44)), 0.5), (1e-10 * 0.001));
        }

        s.v[1155] = if (s.v[1143] < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (!(s.v[1152] != 0.0))) && (s.v[1155] != 0.0)) {
            s.store_scalar(1143, 0.0);
        }

        if (((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (!(s.v[1152] != 0.0))) {
            s.store_mul(1140, 225, 244);
        }

        if (((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (!(s.v[1152] != 0.0))) {
            s.store_div_from_scalar(1138, 1.0, 1140);
        }

        if (((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (!(s.v[1152] != 0.0))) {
            s.store_mul(1142, 246, 1138);
        }

        s.v[1156] = if (s.v[1142] < s.v[227]) { 1.0 } else { 0.0 };

        if ((((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (!(s.v[1152] != 0.0))) && (s.v[1156] != 0.0)) {
            s.copy_ad(1142, 227);
        }

        if (((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (!(s.v[1152] != 0.0))) {
            s.store_scale(1148, 229, 9662367879.197212);
        }

        if (((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (!(s.v[1152] != 0.0))) {
            s.store_scalar(1138, (100000.0 * 10000.0));
        }

        if (((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (!(s.v[1152] != 0.0))) {
            s.store_scalar(1139, (1.0 / s.v[97]));
        }

        if (((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (!(s.v[1152] != 0.0))) {
            s.store_mul_ad_lhs(1150, A::add(A::add(A::scale(s.ad_value(1142), 2.0), A::mul(A::mul(A::scale(s.ad_value(1148), 2.0), s.ad_value(1143)), s.ad_value(1141))), A::mul(s.ad_value(1138), s.ad_value(1141))), 1139);
        }

        if (((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (!(s.v[1152] != 0.0))) {
            s.store_mul(1145, 1150, 1141);
        }

        if (((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (!(s.v[1152] != 0.0))) {
            s.store_scale_ad(1149, A::add(A::mul(A::scale(s.ad_value(1148), 2.0), s.ad_value(1143)), s.ad_value(1138)), 4.0);
        }

        if (((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (!(s.v[1152] != 0.0))) {
            s.store_mul_ad_lhs(1146, A::mul(s.ad_value(1149), s.ad_value(1141)), 1141);
        }

        if (((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (!(s.v[1152] != 0.0))) {
            s.store_sqrt_ad(1147, A::add(A::square(s.ad_value(1145)), s.ad_value(1146)));
        }

        if (((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) && (!(s.v[1152] != 0.0))) {
            s.store_mul_ad_rhs(316, 326, A::scale(A::sub(s.ad_value(1147), s.ad_value(1145)), 0.5));
        }

        if ((s.v[1133] != 0.0) && (s.v[1136] != 0.0)) {
            s.store_scale(316, 316, s.v[127]);
        }

        if (s.v[1133] != 0.0) {
            s.store_sub_from_scalar(441, s.v[97], 316);
        }

        if (s.v[1133] != 0.0) {
            s.store_sub_from_scalar(442, s.v[98], 316);
        }

        s.v[1157] = if (s.v[441] < 1e-9) { 1.0 } else { 0.0 };

        if ((s.v[1133] != 0.0) && (s.v[1157] != 0.0)) {
            s.store_scalar(441, 1e-9);
        }

        if (s.v[1133] != 0.0) {
            s.store_scale_ad(328, A::neg(s.ad_value(108)), s.v[98]);
        }

        if (s.v[1133] != 0.0) {
            s.store_mul(196, 328, 437);
        }

        if (s.v[1133] != 0.0) {
            s.store_mul(197, 328, 436);
        }

        if (s.v[1133] != 0.0) {
            s.store_mul(198, 197, 438);
        }

        s.v[1158] = if (p.p43 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1133] != 0.0) && (s.v[1158] != 0.0)) {
            s.store_scale(477, 196, 0.5);
        }

        if ((s.v[1133] != 0.0) && (s.v[1158] != 0.0)) {
            s.store_scale(476, 196, (1.0 - 0.5));
        }

        if ((s.v[1133] != 0.0) && (s.v[1158] != 0.0)) {
            s.store_mul_ad_lhs(392, A::scale(A::add(s.ad_value(357), s.ad_value(360)), (0.5 * s.v[98])), 108);
        }

        if (s.v[1133] != 0.0) {
            s.store_scaled_sub(1159, 157, 164, 0.5);
        }

        if (s.v[1133] != 0.0) {
            s.store_scale(44, 1159, (2.0 * 1.0 / (p.p227)));
        }

        if (s.v[1133] != 0.0) {
            s.store_offset_ad(45, A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::scale(s.ad_value(44), (1.0 / 5040.0)), (1.0 / 720.0))), (1.0 / 120.0))), (1.0 / 24.0))), (1.0 / 6.0))), (1.0 / 2.0))), 1.0);
        }

        if (s.v[1133] != 0.0) {
            s.store_div_from_scalar(177, p.p227, 45);
        }

        s.v[1160] = if (s.v[177] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if ((s.v[1133] != 0.0) && (s.v[1160] != 0.0)) {
            s.store_scalar(177, (10.0 * 2.220446049250313e-16));
        }

        if (s.v[1133] != 0.0) {
            s.store_add(176, 161, 177);
        }

        if (s.v[1133] != 0.0) {
            s.store_scalar(1170, (1.034943e-10 / 100.0));
        }

        if (s.v[1133] != 0.0) {
            s.store_scale(1171, 437, 0.0001);
        }

        if (s.v[1133] != 0.0) {
            s.store_scale(1172, 436, 0.0001);
        }

        if (s.v[1133] != 0.0) {
            s.store_div_from_scalar(1161, p.p92, 1170);
        }

        if (s.v[1133] != 0.0) {
            s.store_div_from_scalar(1162, p.p93, 1170);
        }

        if (s.v[1133] != 0.0) {
            s.store_scalar(1163, p.p94);
        }

        if (s.v[1133] != 0.0) {
            s.store_offset_ad(1164, A::mul(A::sub(s.ad_value(162), s.ad_value(161)), s.ad_value(1163)), 1.0);
        }

        if (s.v[1133] != 0.0) {
            s.store_add_ad(1165, A::mul(s.ad_value(1161), s.ad_value(1171)), A::mul(s.ad_value(1162), s.ad_value(1172)));
        }

        if (s.v[1133] != 0.0) {
            s.store_div(1166, 1165, 1164);
        }

        if (s.v[1133] != 0.0) {
            s.copy_ad(248, 1166);
        }

        if (s.v[1133] != 0.0) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(248)), ((4.0 * 3000.0) * 3000.0)));
        }

        if (s.v[1133] != 0.0) {
            s.store_offset_ad(1163, A::scale(A::add(s.ad_value(248), s.ad_value(44)), 0.5), (1e-10 * 3000.0));
        }

        s.v[1173] = if (s.v[1163] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1133] != 0.0) && (s.v[1173] != 0.0)) {
            s.store_scalar(1163, 0.0);
        }

        if (s.v[1133] != 0.0) {
            s.store_powf(1165, 1163, (p.p97 - 1.0));
        }

        if (s.v[1133] != 0.0) {
            s.store_mul(1167, 1165, 1163);
        }

        if (s.v[1133] != 0.0) {
            s.store_powf(1168, 1163, (s.v[111] - 1.0));
        }

        if (s.v[1133] != 0.0) {
            s.store_mul(1169, 1168, 1163);
        }

        if (s.v[1133] != 0.0) {
            s.store_scale(249, 1172, 6.241449993689894e18);
        }

        if (s.v[1133] != 0.0) {
            s.store_add_ad(1161, A::add(A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(249), (p.p96 * 1e-11)), p.p95)), A::mul(s.ad_value(543), s.ad_value(1167))), A::scale(s.ad_value(1169), 1.0 / (p.p106)));
        }

        if (s.v[1133] != 0.0) {
            s.store_div_from_scalar(251, 1.0, 1161);
        }

        if (s.v[1133] != 0.0) {
            s.store_scale(251, 251, 0.0001);
        }

        if (s.v[1133] != 0.0) {
            s.store_mul_ad_lhs(1174, A::mul(s.ad_value(225), s.ad_value(244)), 441);
        }

        if (s.v[1133] != 0.0) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1174)), ((4.0 * 1e-50) * 1e-50)));
        }

        if (s.v[1133] != 0.0) {
            s.store_offset_ad(1174, A::scale(A::add(s.ad_value(1174), s.ad_value(44)), 0.5), (1e-10 * 1e-50));
        }

        s.v[1182] = if (s.v[1174] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1133] != 0.0) && (s.v[1182] != 0.0)) {
            s.store_scalar(1174, 0.0);
        }

        if (s.v[1133] != 0.0) {
            s.store_div_from_scalar(1175, 1.0, 1174);
        }

        if (s.v[1133] != 0.0) {
            s.store_mul(1176, 246, 1175);
        }

        if (s.v[1133] != 0.0) {
            s.store_div_ad_lhs(1174, A::scale(s.ad_value(253), 0.2), 251);
        }

        if (s.v[1133] != 0.0) {
            s.store_sqrt_ad(252, A::add(A::square(s.ad_value(1176)), A::square(s.ad_value(1174))));
        }

        if (s.v[1133] != 0.0) {
            s.store_mul(1177, 251, 252);
        }

        if (s.v[1133] != 0.0) {
            s.store_div(1175, 1177, 253);
        }

        s.v[1183] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1133] != 0.0) && (s.v[1183] != 0.0)) {
            s.store_scalar(1178, 1.0);
        }

        s.v[1184] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1133] != 0.0) && (!(s.v[1183] != 0.0))) && (s.v[1184] != 0.0)) {
            s.copy_ad(1178, 1175);
        }

        if (((s.v[1133] != 0.0) && (!(s.v[1183] != 0.0))) && (!(s.v[1184] != 0.0))) {
            s.store_powf(1178, 1175, (p.p113 - 1.0));
        }

        if (s.v[1133] != 0.0) {
            s.store_mul(1174, 1175, 1178);
        }

        if (s.v[1133] != 0.0) {
            s.store_offset(1179, 1174, 1.0);
        }

        s.v[1185] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1133] != 0.0) && (s.v[1185] != 0.0)) {
            s.store_div_from_scalar(1180, 1.0, 1179);
        }

        s.v[1186] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1133] != 0.0) && (!(s.v[1185] != 0.0))) && (s.v[1186] != 0.0)) {
            s.store_div_from_scalar_ad(1180, 1.0, A::sqrt(s.ad_value(1179)));
        }

        if (((s.v[1133] != 0.0) && (!(s.v[1185] != 0.0))) && (!(s.v[1186] != 0.0))) {
            s.store_powf(1181, 1179, (((-1.0) / p.p113) - 1.0));
        }

        if (((s.v[1133] != 0.0) && (!(s.v[1185] != 0.0))) && (!(s.v[1186] != 0.0))) {
            s.store_mul(1180, 1179, 1181);
        }

        if (s.v[1133] != 0.0) {
            s.store_mul(250, 251, 1180);
        }

        if (s.v[1133] != 0.0) {
            s.store_div_ad(264, A::mul(s.ad_value(107), s.ad_value(227)), A::sub_from_scalar(s.v[97], s.ad_value(316)));
        }

        if (s.v[1133] != 0.0) {
            s.store_mul_ad_lhs(200, A::mul(s.ad_value(264), s.ad_value(246)), 250);
        }

        if (s.v[1133] != 0.0) {
            s.store_scalar(201, 0.0);
        }

        s.v[1196] = if ((p.p281 > 0.0) && (p.p244 != 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1133] != 0.0) && (s.v[1196] != 0.0)) {
            s.store_scaled_sub(1187, 157, 164, 0.5);
        }

        if ((s.v[1133] != 0.0) && (s.v[1196] != 0.0)) {
            s.store_scale(44, 1187, (2.0 * 100.0));
        }

        if ((s.v[1133] != 0.0) && (s.v[1196] != 0.0)) {
            s.store_offset_ad(45, A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::scale(s.ad_value(44), (1.0 / 5040.0)), (1.0 / 720.0))), (1.0 / 120.0))), (1.0 / 24.0))), (1.0 / 6.0))), (1.0 / 2.0))), 1.0);
        }

        if ((s.v[1133] != 0.0) && (s.v[1196] != 0.0)) {
            s.store_div_from_scalar(1193, 0.01, 45);
        }

        if ((s.v[1133] != 0.0) && (s.v[1196] != 0.0)) {
            s.store_sub_from_scalar_ad(1187, 1.1, A::add(s.ad_value(161), s.ad_value(1193)));
        }

        if ((s.v[1133] != 0.0) && (s.v[1196] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1187)), ((4.0 * 0.05) * 0.05)));
        }

        if ((s.v[1133] != 0.0) && (s.v[1196] != 0.0)) {
            s.store_offset_ad(1195, A::scale(A::add(s.ad_value(1187), s.ad_value(44)), 0.5), (1e-10 * 0.05));
        }

        s.v[1197] = if (s.v[1195] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1133] != 0.0) && (s.v[1196] != 0.0)) && (s.v[1197] != 0.0)) {
            s.store_scalar(1195, 0.0);
        }

        if ((s.v[1133] != 0.0) && (s.v[1196] != 0.0)) {
            s.store_scale(1188, 225, s.v[116]);
        }

        if ((s.v[1133] != 0.0) && (s.v[1196] != 0.0)) {
            s.store_mul(1189, 323, 1188);
        }

        if ((s.v[1133] != 0.0) && (s.v[1196] != 0.0)) {
            s.store_powf(1188, 1195, p.p245);
        }

        if ((s.v[1133] != 0.0) && (s.v[1196] != 0.0)) {
            s.store_mul(1190, 1189, 1188);
        }

        if ((s.v[1133] != 0.0) && (s.v[1196] != 0.0)) {
            s.store_offset_scaled(1191, 173, p.p246, 1.0);
        }

        if ((s.v[1133] != 0.0) && (s.v[1196] != 0.0)) {
            s.store_scalar(1188, s.v[117]);
        }

        s.v[1198] = if ((s.v[56] < 3.0) || (p.p43 == 1.0)) { 1.0 } else { 0.0 };

        if (((s.v[1133] != 0.0) && (s.v[1196] != 0.0)) && (s.v[1198] != 0.0)) {
            s.store_sub_ad_lhs(1192, A::add(s.ad_value(161), s.ad_value(1193)), 172);
        }

        if (((s.v[1133] != 0.0) && (s.v[1196] != 0.0)) && (!(s.v[1198] != 0.0))) {
            s.store_sub_ad_lhs(1192, A::add(s.ad_value(161), s.ad_value(1193)), 350);
        }

        if ((s.v[1133] != 0.0) && (s.v[1196] != 0.0)) {
            s.store_add_ad_rhs(1191, 1191, A::mul(A::mul(s.ad_value(173), s.ad_value(1188)), s.ad_value(1192)));
        }

        if ((s.v[1133] != 0.0) && (s.v[1196] != 0.0)) {
            s.store_mul(1193, 1190, 1191);
        }

    }

    pub(super) fn stamp_transient_block_17(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[1133] != 0.0) && (s.v[1196] != 0.0)) {
            s.copy_ad(1190, 1193);
        }

        if ((s.v[1133] != 0.0) && (!(s.v[1196] != 0.0))) {
            s.store_scalar(1190, 0.0);
        }

        s.v[1199] = if (p.p248 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1133] != 0.0) && (s.v[1199] != 0.0)) {
            s.store_scale(1187, 225, s.v[118]);
        }

        if ((s.v[1133] != 0.0) && (s.v[1199] != 0.0)) {
            s.store_mul(1195, 323, 1187);
        }

        if ((s.v[1133] != 0.0) && (s.v[1199] != 0.0)) {
            s.store_mul(1194, 1195, 173);
        }

        if ((s.v[1133] != 0.0) && (!(s.v[1199] != 0.0))) {
            s.store_scalar(1194, 0.0);
        }

        s.v[1200] = if ((s.v[1190] + s.v[1194]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1133] != 0.0) && (s.v[1200] != 0.0)) {
            s.store_mul_ad_rhs(247, 164, A::add(s.ad_value(1190), s.ad_value(1194)));
        }

        if ((s.v[1133] != 0.0) && (s.v[1200] != 0.0)) {
            s.store_mul_ad_lhs(201, A::mul(s.ad_value(264), s.ad_value(247)), 250);
        }

        if (s.v[1133] != 0.0) {
            s.store_add(199, 200, 201);
        }

        if (s.v[1133] != 0.0) {
            s.copy_ad(203, 201);
        }

        s.v[1210] = if (p.p33 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.copy_ad(1203, 549);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_scalar(1204, (s.v[124] - p.p71));
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_div_from_scalar_ad(1205, 1.0, A::square(s.ad_value(1204)));
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_mul_ad_lhs(1206, A::mul(A::mul(A::scale(A::sub_from_scalar(p.p69, s.ad_value(233)), 2.0), A::scale(s.ad_value(324), 1.034943e-10)), s.ad_value(1203)), 1205);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_mul(186, 1206, 235);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_offset_scaled(1202, 173, p.p155, p.p154);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_mul(206, 186, 1202);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_sub_from_scalar_ad(1201, p.p156, A::scale(s.ad_value(157), p.p157));
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_add_ad_lhs(207, A::add(A::offset(s.ad_value(174), (-s.v[123])), s.ad_value(1201)), 206);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_mul_ad_lhs(210, A::mul(s.ad_value(205), s.ad_value(324)), 324);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_scaled_mul(211, 210, 225, 0.5);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_scaled_mul(212, 211, 225, 2.0);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_offset_ad(1207, A::sub(A::offset(A::offset(A::sub(s.ad_value(227), A::mul(s.ad_value(210), A::scale(s.ad_value(225), 0.25))), s.v[123]), (-p.p156)), s.ad_value(206)), 1e-50);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_offset_ad(1201, A::sub(s.ad_value(174), s.ad_value(1207)), (-0.005));
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_scalar(327, (if (s.v[1207] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_sqrt_ad(1203, A::add(A::square(s.ad_value(1201)), A::scale(A::mul(A::scale(s.ad_value(327), 4.0), s.ad_value(1207)), 0.005)));
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_sub_ad_lhs(1204, A::add(A::offset(A::offset(A::add(s.ad_value(1207), A::scale(A::add(s.ad_value(1201), s.ad_value(1203)), 0.5)), (-s.v[123])), p.p156), s.ad_value(206)), 514);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_offset_ad(1205, A::mul(s.ad_value(225), s.ad_value(1204)), (-1.0));
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_div_from_scalar(1206, 4.0, 212);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_offset_ad(1202, A::mul(s.ad_value(1205), s.ad_value(1206)), 1.0);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1202)), ((4.0 * 0.01) * 0.01)));
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_offset_ad(1201, A::scale(A::add(s.ad_value(1202), s.ad_value(44)), 0.5), (1e-10 * 0.01));
        }

        s.v[1211] = if (s.v[1201] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) && (s.v[1211] != 0.0)) {
            s.store_scalar(1201, 0.0);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_sqrt_ad(213, A::offset(s.ad_value(1201), 1e-50));
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_add_ad_rhs(215, 207, A::mul(s.ad_value(211), A::sub_from_scalar(1.0, s.ad_value(213))));
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_div_from_scalar_ad(327, 1.0, A::add(s.ad_value(225), A::div_from_scalar(2.0, A::offset(s.ad_value(207), 1e-50))));
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_mul_ad_lhs(216, A::ln(A::mul(A::div(A::div_from_scalar(1.0, s.ad_value(209)), s.ad_value(210)), A::square(s.ad_value(207)))), 327);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_div_ad_rhs(1204, 216, A::offset(s.ad_value(207), 1e-50));
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_offset_ad(217, A::sub(s.ad_value(216), s.ad_value(215)), (-0.002));
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_sqrt_ad(327, A::add(A::square(s.ad_value(217)), A::scale(s.ad_value(216), (4.0 * 0.002))));
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_sub_ad_rhs(218, 216, A::scale(A::add(s.ad_value(217), s.ad_value(327)), 0.5));
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_div_from_scalar(1201, 1.0, 327);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_mul_ad_rhs(327, 209, A::exp(A::mul(s.ad_value(225), s.ad_value(218))));
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_add_ad_lhs(1202, A::offset(A::mul(s.ad_value(225), A::sub(s.ad_value(218), s.ad_value(514))), (-1.0)), 327);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1202)), ((4.0 * 0.01) * 0.01)));
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_offset_ad(1201, A::scale(A::add(s.ad_value(1202), s.ad_value(44)), 0.5), (1e-10 * 0.01));
        }

        s.v[1212] = if (s.v[1201] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) && (s.v[1212] != 0.0)) {
            s.store_scalar(1201, 0.0);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_sqrt_ad(219, A::offset(s.ad_value(1201), (10.0 * 2.220446049250313e-16)));
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_offset_ad(1202, A::mul(s.ad_value(225), A::sub(s.ad_value(218), s.ad_value(514))), (-1.0));
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1202)), ((4.0 * 0.01) * 0.01)));
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_offset_ad(1201, A::scale(A::add(s.ad_value(1202), s.ad_value(44)), 0.5), (1e-10 * 0.01));
        }

        s.v[1213] = if (s.v[1201] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) && (s.v[1213] != 0.0)) {
            s.store_scalar(1201, 0.0);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_sqrt_ad(220, A::offset(s.ad_value(1201), (10.0 * 2.220446049250313e-16)));
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_mul_ad_rhs(221, 208, A::sub(s.ad_value(219), s.ad_value(220)));
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_sub(1202, 215, 218);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1202)), ((4.0 * 0.1) * 0.1)));
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_offset_ad(1201, A::scale(A::add(s.ad_value(1202), s.ad_value(44)), 0.5), (1e-10 * 0.1));
        }

        s.v[1214] = if (s.v[1201] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) && (s.v[1214] != 0.0)) {
            s.store_scalar(1201, 0.0);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_div_ad_rhs(1208, 157, A::offset(s.ad_value(1201), (10.0 * 2.220446049250313e-16)));
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_square(49, 1208);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_scalar(50, 1.0);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_scalar(51, 1.0);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_scalar(52, 1.0);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_scalar(55, 0.0);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_scalar(48, 0.0);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_scalar(53, 0.0);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_add(48, 51, 52);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.copy_ad(53, 48);
        }

        s.v[1215] = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1216] = if (4.0 == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) && (s.v[1215] != 0.0)) && (s.v[1216] != 0.0)) {
            s.store_scalar(55, 1.0);
        }

        s.v[1217] = if (4.0 == 2.0) { 1.0 } else { 0.0 };

        if (((((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) && (s.v[1215] != 0.0)) && (!(s.v[1216] != 0.0))) && (s.v[1217] != 0.0)) {
            s.store_scalar(55, 2.0);
        }

        s.v[1218] = if (4.0 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) && (s.v[1215] != 0.0)) && (!(s.v[1216] != 0.0))) && (!(s.v[1217] != 0.0))) && (s.v[1218] != 0.0)) {
            s.store_scalar(55, 3.0);
        }

        s.v[1219] = if (4.0 == 8.0) { 1.0 } else { 0.0 };

        if (((((((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) && (s.v[1215] != 0.0)) && (!(s.v[1216] != 0.0))) && (!(s.v[1217] != 0.0))) && (!(s.v[1218] != 0.0))) && (s.v[1219] != 0.0)) {
            s.store_scalar(55, 4.0);
        }

        if (((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) && (s.v[1215] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        let mut assign19490_loop_guard: usize = 0;
        while {
            let assign19490_cond_e26982: f64 = if ((((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) && (s.v[1215] != 0.0)) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign19490_cond_e26982 != 0.0
        } {
            assign19490_loop_guard += 1;
            assert!(assign19490_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) && (s.v[1215] != 0.0)) {
                s.store_sqrt(53, 53);
            }
            if (((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) && (s.v[1215] != 0.0)) {
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) && (!(s.v[1215] != 0.0))) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_div_from_scalar(53, 1.0, 53);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_mul(1209, 1208, 53);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_scale(214, 227, ((2.0 * s.v[126]) * p.p9));
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_div_ad_lhs(222, A::mul(A::mul(A::mul(s.ad_value(214), s.ad_value(250)), s.ad_value(221)), s.ad_value(1209)), 441);
        }

        if ((s.v[1133] != 0.0) && (s.v[1210] != 0.0)) {
            s.store_add(199, 199, 222);
        }

        s.v[1220] = if ((p.p30 != 0.0) && (p.p32 != 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1133] != 0.0) && (s.v[1220] != 0.0)) {
            s.store_square(294, 192);
        }

        if ((s.v[1133] != 0.0) && (s.v[1220] != 0.0)) {
            s.store_mul_ad_lhs(295, A::mul(A::scale(s.ad_value(227), 2.0), s.ad_value(324)), 246);
        }

        if ((s.v[1133] != 0.0) && (s.v[1220] != 0.0)) {
            s.store_sub(296, 294, 295);
        }

        if ((s.v[1133] != 0.0) && (s.v[1220] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(294)), ((4.0 * 0.001) * 0.001)));
        }

        if ((s.v[1133] != 0.0) && (s.v[1220] != 0.0)) {
            s.store_offset_ad(294, A::scale(A::add(s.ad_value(294), s.ad_value(44)), 0.5), (1e-10 * 0.001));
        }

        s.v[1221] = if (s.v[294] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1133] != 0.0) && (s.v[1220] != 0.0)) && (s.v[1221] != 0.0)) {
            s.store_scalar(294, 0.0);
        }

        if ((s.v[1133] != 0.0) && (s.v[1220] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(296)), ((4.0 * 0.001) * 0.001)));
        }

        if ((s.v[1133] != 0.0) && (s.v[1220] != 0.0)) {
            s.store_offset_ad(296, A::scale(A::add(s.ad_value(296), s.ad_value(44)), 0.5), (1e-10 * 0.001));
        }

        s.v[1222] = if (s.v[296] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1133] != 0.0) && (s.v[1220] != 0.0)) && (s.v[1222] != 0.0)) {
            s.store_scalar(296, 0.0);
        }

        if ((s.v[1133] != 0.0) && (s.v[1220] != 0.0)) {
            s.store_sub(297, 294, 296);
        }

        s.v[1223] = if ((s.v[244] < (10.0 * 2.220446049250313e-16)) || (s.v[297] < (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if (((s.v[1133] != 0.0) && (s.v[1220] != 0.0)) && (s.v[1223] != 0.0)) {
            s.store_scalar(146, 0.0);
        }

        if (((s.v[1133] != 0.0) && (s.v[1220] != 0.0)) && (!(s.v[1223] != 0.0))) {
            s.store_scalar(146, 1.0);
        }

        s.copy_ad(202, 199);

        s.v[204] = 0.0;

        s.v[1224] = if ((p.p281 > 0.0) && (p.p285 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1224] != 0.0) {
            s.store_scalar(1231, s.v[99]);
        }

        if (s.v[1224] != 0.0) {
            s.store_scalar(1235, p.p237);
        }

        if (s.v[1224] != 0.0) {
            s.store_offset_ad(1236, A::sub(A::add(A::offset(s.ad_value(158), (-s.v[123])), s.ad_value(185)), s.ad_value(320)), (-p.p286));
        }

        if (s.v[1224] != 0.0) {
            s.store_offset(1237, 182, p.p286);
        }

        if (s.v[1224] != 0.0) {
            s.store_scalar(1239, p.p285);
        }

        if (s.v[1224] != 0.0) {
            s.store_scalar(1238, p.p283);
        }

        if (s.v[1224] != 0.0) {
            s.store_scalar(1229, s.v[70]);
        }

        if (s.v[1224] != 0.0) {
            s.store_mul_ad_rhs(1230, 227, A::ln(A::div(A::mul(A::div(s.ad_value(1229), s.ad_value(230)), s.ad_value(536)), s.ad_value(230))));
        }

        if (s.v[1224] != 0.0) {
            s.store_ad(1227, &{
                if (p.p43 == 1.0) {
                    s.ad_value(435)
                } else {
                    s.ad_value(350)
                }
            });
        }

        if (s.v[1224] != 0.0) {
            s.store_sqrt_ad(1232, A::div(A::mul(A::mul(A::scale(A::sub(s.ad_value(1230), s.ad_value(1227)), ((2.0 * 1.6021918e-19) * 9662367879.197212)), s.ad_value(536)), s.ad_value(1229)), A::add(s.ad_value(536), s.ad_value(1229))));
        }

        if (s.v[1224] != 0.0) {
            s.store_mul(1226, 1232, 1231);
        }

        if (s.v[1224] != 0.0) {
            s.store_div_ad(1225, A::mul(A::scale(s.ad_value(1226), (-0.25)), s.ad_value(1226)), A::add(s.ad_value(157), s.ad_value(1226)));
        }

        if (s.v[1224] != 0.0) {
            s.copy_ad(1251, 1225);
        }

    }

    pub(super) fn stamp_transient_block_18(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1224] != 0.0) {
            s.copy_ad(1252, 1237);
        }

        if (s.v[1224] != 0.0) {
            s.store_offset_ad(336, A::div(A::scale(A::offset(A::mul(s.ad_value(225), A::sub(s.ad_value(1236), s.ad_value(1251))), (-1.0)), 4.0), A::mul(s.ad_value(241), s.ad_value(226))), 1.0);
        }

        if (s.v[1224] != 0.0) {
            s.store_ad(336, &{
                if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(336)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if (s.v[1224] != 0.0) {
            s.store_add_ad_rhs(376, 1236, A::mul(A::scale(A::mul(s.ad_value(241), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336)))));
        }

        s.v[1253] = if (s.v[158] < ((s.v[123] + s.v[1252]) * 0.5)) { 1.0 } else { 0.0 };

        if ((s.v[1224] != 0.0) && (s.v[1253] != 0.0)) {
            s.store_scalar(144, 0.0);
        }

        s.v[1254] = if ((s.v[144] == 0.0) || (1.0 != 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) {
            s.store_mul_ad_rhs(181, 225, A::sub(s.ad_value(376), s.ad_value(1251)));
        }

        s.v[1255] = if (s.v[181] < 3.0) { 1.0 } else { 0.0 };

        if (((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1255] != 0.0)) {
            s.store_mul_ad_rhs(337, 225, A::sub(s.ad_value(1236), s.ad_value(1251)));
        }

        if (((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1255] != 0.0)) {
            s.store_div_from_scalar_ad(328, 1.0, A::mul(A::scale(s.ad_value(225), (1.414213562373095 / 108.0)), s.ad_value(240)));
        }

        if (((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1255] != 0.0)) {
            s.store_offset_scaled(329, 328, 3.0, 81.0);
        }

        if (((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1255] != 0.0)) {
            s.store_add_ad(330, A::sub_from_scalar((-2916.0), A::scale(s.ad_value(328), 81.0)), A::mul(A::scale(s.ad_value(328), 27.0), s.ad_value(337)));
        }

        if (((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1255] != 0.0)) {
            s.store_add_ad(331, A::sub_from_scalar(1458.0, A::scale(A::offset(s.ad_value(328), 54.0), 81.0)), A::mul(A::scale(s.ad_value(328), 27.0), s.ad_value(337)));
        }

        if (((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1255] != 0.0)) {
            s.store_square(331, 331);
        }

        if (((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1255] != 0.0)) {
            s.store_powf_ad(332, A::add(s.ad_value(330), A::sqrt(A::add(A::mul(A::mul(A::scale(s.ad_value(329), 4.0), s.ad_value(329)), s.ad_value(329)), s.ad_value(331)))), 0.3333333333333333);
        }

        if (((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1255] != 0.0)) {
            s.store_add_ad(336, A::sub_from_scalar(3.0, A::div(A::scale(s.ad_value(329), 1.259921049894873), A::scale(s.ad_value(332), 3.0))), A::scale(s.ad_value(332), (1.0 / (3.0 * 1.259921049894873))));
        }

        if (((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1255] != 0.0)) {
            s.store_add_ad_lhs(376, A::mul(s.ad_value(336), s.ad_value(227)), 1251);
        }

        if (((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1255] != 0.0)) {
            s.copy_ad(378, 376);
        }

        s.v[1256] = if ((s.v[158] - s.v[383]) <= s.v[1252]) { 1.0 } else { 0.0 };

        s.v[1257] = if (p.p43 == 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_div_from_scalar(327, 1.0, 323);
        }

        if (((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_scale(328, 1235, 9662367879.197212);
        }

        if (((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_scalar(329, (1.0 / s.v[93]));
        }

        if (((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_div_from_scalar_ad(330, 1.0, A::add(A::add(s.ad_value(327), s.ad_value(328)), s.ad_value(329)));
        }

        if (((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_mul_ad_rhs(331, 330, A::add(A::sub(s.ad_value(1236), s.ad_value(475)), A::mul(A::add(s.ad_value(329), A::scale(s.ad_value(328), 0.5)), A::neg(s.ad_value(369)))));
        }

        if (((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_sub_ad_rhs(376, 1236, A::div(s.ad_value(331), s.ad_value(323)));
        }

        if ((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) {
            s.copy_ad(378, 376);
        }

        if ((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (!(s.v[1256] != 0.0))) {
            s.store_div_ad_lhs(328, A::div_from_scalar(1.0, s.ad_value(379)), 434);
        }

        if ((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (!(s.v[1256] != 0.0))) {
            s.store_mul_ad(329, A::mul(s.ad_value(328), A::sub(s.ad_value(1236), s.ad_value(383))), A::sub(s.ad_value(1236), s.ad_value(383)));
        }

        if ((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (!(s.v[1256] != 0.0))) {
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(1236), s.ad_value(383))));
        }

        if ((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (!(s.v[1256] != 0.0))) {
            s.store_offset_ad(377, A::div(A::ln(s.ad_value(329)), s.ad_value(330)), p.p287);
        }

        if ((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (!(s.v[1256] != 0.0))) {
            s.store_offset_ad(44, A::sub(s.ad_value(377), s.ad_value(376)), (-0.0008));
        }

        if ((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (!(s.v[1256] != 0.0))) {
            s.store_scale(45, 377, (4.0 * 0.0008));
        }

        if ((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (!(s.v[1256] != 0.0))) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (!(s.v[1256] != 0.0))) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (!(s.v[1255] != 0.0))) && (!(s.v[1256] != 0.0))) {
            s.store_sub_ad_rhs(378, 377, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        s.v[1258] = if (p.p43 == 0.0) { 1.0 } else { 0.0 };

        s.v[1259] = if ((s.v[158] - s.v[383]) <= s.v[1252]) { 1.0 } else { 0.0 };

        if ((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) {
            s.store_div_from_scalar(327, 1.0, 323);
        }

        if ((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) {
            s.store_scale(328, 1235, 9662367879.197212);
        }

        if ((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) {
            s.store_scalar(329, (1.0 / s.v[93]));
        }

        if ((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) {
            s.store_div_from_scalar_ad(330, 1.0, A::add(A::add(s.ad_value(327), s.ad_value(328)), s.ad_value(329)));
        }

        if ((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) {
            s.store_mul_ad_rhs(331, 330, A::add(A::sub(s.ad_value(1236), s.ad_value(475)), A::mul(A::add(s.ad_value(329), A::scale(s.ad_value(328), 0.5)), A::neg(s.ad_value(369)))));
        }

        if ((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) {
            s.store_sub_ad_rhs(376, 1236, A::div(s.ad_value(331), s.ad_value(323)));
        }

        if ((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) {
            s.copy_ad(378, 376);
        }

        if ((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) {
            s.store_div_from_scalar(327, 1.0, 323);
        }

        if ((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) {
            s.store_scale(328, 1235, 9662367879.197212);
        }

        if ((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) {
            s.store_scalar(329, (1.0 / s.v[93]));
        }

        if ((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) {
            s.store_div_from_scalar_ad(330, 1.0, A::add(A::add(s.ad_value(327), s.ad_value(328)), s.ad_value(329)));
        }

        if ((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) {
            s.store_mul_ad_rhs(331, 330, A::add(A::sub(s.ad_value(1236), s.ad_value(475)), A::mul(A::add(s.ad_value(329), A::scale(s.ad_value(328), 0.5)), A::neg(s.ad_value(369)))));
        }

        if ((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) {
            s.store_sub_ad_rhs(376, 1236, A::div(s.ad_value(331), s.ad_value(323)));
        }

        if ((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) {
            s.copy_ad(378, 376);
        }

        s.v[1260] = if ((s.v[1236] - s.v[383]) > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) {
            s.store_div_ad_lhs(328, A::div_from_scalar(1.0, s.ad_value(379)), 434);
        }

        if (((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) {
            s.store_mul_ad(329, A::mul(s.ad_value(328), A::sub(s.ad_value(1236), s.ad_value(383))), A::sub(s.ad_value(1236), s.ad_value(383)));
        }

        if (((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) {
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(1236), s.ad_value(383))));
        }

        if (((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) {
            s.store_offset_ad(377, A::div(A::ln(s.ad_value(329)), s.ad_value(330)), p.p287);
        }

        s.v[1261] = if ((s.v[376] > ((s.v[377] * 0.98) - 0.4)) && (0.4 >= 0.0)) { 1.0 } else { 0.0 };

        if ((((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) {
            s.store_offset_ad(44, A::sub(s.ad_value(376), A::scale(s.ad_value(377), 0.98)), 0.4);
        }

        if ((((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) {
            s.store_square(49, 44);
        }

        if ((((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) {
            s.store_scalar(50, (0.4 * 0.4));
        }

        if ((((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) {
            s.store_scalar(51, 1.0);
        }

        if ((((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) {
            s.store_scalar(52, 1.0);
        }

        if ((((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        if ((((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) {
            s.store_scalar(55, 0.0);
        }

        if ((((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) {
            s.store_scalar(48, 0.0);
        }

        if ((((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) {
            s.store_scalar(53, 0.0);
        }

        if ((((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) {
            s.store_add(48, 51, 52);
        }

        if ((((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) {
            s.copy_ad(53, 48);
        }

        s.v[1262] = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1263] = if (2.0 == 1.0) { 1.0 } else { 0.0 };

        if ((((((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) && (s.v[1262] != 0.0)) && (s.v[1263] != 0.0)) {
            s.store_scalar(55, 1.0);
        }

        s.v[1264] = if (2.0 == 2.0) { 1.0 } else { 0.0 };

        if (((((((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) && (s.v[1262] != 0.0)) && (!(s.v[1263] != 0.0))) && (s.v[1264] != 0.0)) {
            s.store_scalar(55, 2.0);
        }

        s.v[1265] = if (2.0 == 4.0) { 1.0 } else { 0.0 };

        if ((((((((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) && (s.v[1262] != 0.0)) && (!(s.v[1263] != 0.0))) && (!(s.v[1264] != 0.0))) && (s.v[1265] != 0.0)) {
            s.store_scalar(55, 3.0);
        }

        s.v[1266] = if (2.0 == 8.0) { 1.0 } else { 0.0 };

        if (((((((((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) && (s.v[1262] != 0.0)) && (!(s.v[1263] != 0.0))) && (!(s.v[1264] != 0.0))) && (!(s.v[1265] != 0.0))) && (s.v[1266] != 0.0)) {
            s.store_scalar(55, 4.0);
        }

        if (((((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) && (s.v[1262] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        let mut assign20720_loop_guard: usize = 0;
        while {
            let assign20720_cond_e28554: f64 = if ((((((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) && (s.v[1262] != 0.0)) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign20720_cond_e28554 != 0.0
        } {
            assign20720_loop_guard += 1;
            assert!(assign20720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) && (s.v[1262] != 0.0)) {
                s.store_sqrt(53, 53);
            }
            if (((((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) && (s.v[1262] != 0.0)) {
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) && (!(s.v[1262] != 0.0))) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if ((((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) {
            s.store_div_from_scalar(53, 1.0, 53);
        }

        if ((((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) {
            s.store_mul_ad_lhs(43, A::scale(s.ad_value(44), 0.4), 53);
        }

        if ((((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) {
            s.store_add_ad_lhs(378, A::offset(A::scale(s.ad_value(377), 0.98), (-0.4)), 43);
        }

        if ((((((s.v[1224] != 0.0) && (s.v[1254] != 0.0)) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) && (!(s.v[1261] != 0.0))) {
            s.copy_ad(378, 376);
        }

        if (s.v[1224] != 0.0) {
            s.store_offset(336, 1251, (5e-12 / 2.0));
        }

        s.v[1267] = if (s.v[378] < s.v[336]) { 1.0 } else { 0.0 };

        if ((s.v[1224] != 0.0) && (s.v[1267] != 0.0)) {
            s.copy_ad(378, 336);
        }

        if (s.v[1224] != 0.0) {
            s.copy_ad(1234, 378);
        }

        if (s.v[1224] != 0.0) {
            s.copy_ad(163, 376);
        }

        if ((s.v[1224] != 0.0) && (0.0 != 0.0)) {
            s.store_ad(166, &{
                if ((s.v[376] - s.v[1234]) >= 0.0) {
                    A::sub(s.ad_value(376), s.ad_value(1234))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if ((s.v[1224] != 0.0) && (0.0 != 0.0)) {
            s.store_offset_ad(44, A::offset(A::scale(s.ad_value(166), (1.0 + 0.3)), (-p.p287)), (-0.03));
        }

        if ((s.v[1224] != 0.0) && (0.0 != 0.0)) {
            s.store_scale(45, 166, ((1.0 + 0.3) * (4.0 * 0.03)));
        }

        if ((s.v[1224] != 0.0) && (0.0 != 0.0)) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((s.v[1224] != 0.0) && (0.0 != 0.0)) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((s.v[1224] != 0.0) && (0.0 != 0.0)) {
            s.store_sub_ad(165, A::scale(s.ad_value(166), (1.0 + 0.3)), A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((s.v[1224] != 0.0) && (0.0 != 0.0)) {
            s.store_ad(165, &{
                if (s.v[165] <= s.v[166]) {
                    s.ad_value(165)
                } else {
                    s.ad_value(166)
                }
            });
        }

        s.v[1268] = if (s.v[165] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1224] != 0.0) && (0.0 != 0.0)) && (s.v[1268] != 0.0)) {
            s.store_scalar(165, 0.0);
        }

        s.v[1269] = if (s.v[165] > s.v[157]) { 1.0 } else { 0.0 };

        if ((((s.v[1224] != 0.0) && (0.0 != 0.0)) && (!(s.v[1268] != 0.0))) && (s.v[1269] != 0.0)) {
            s.copy_ad(165, 157);
        }

        if ((s.v[1224] != 0.0) && (0.0 != 0.0)) {
            s.store_add(163, 1234, 165);
        }

        s.v[1270] = if (p.p282 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) {
            s.copy_ad(378, 1234);
        }

        if ((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) {
            s.copy_ad(1271, 1225);
        }

        if ((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) {
            s.store_offset_ad(160, A::add(A::add(A::sub_from_scalar(s.v[123], s.ad_value(185)), s.ad_value(320)), s.ad_value(1271)), p.p286);
        }

        s.v[1273] = if (s.v[158] < s.v[160]) { 1.0 } else { 0.0 };

        if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (s.v[1273] != 0.0)) {
            s.store_scalar(338, (-1.0));
        }

        if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (s.v[1273] != 0.0)) {
            s.store_mul_ad(254, A::scale(s.ad_value(227), 2.0), A::ln(A::div_from_scalar((-s.v[139]), s.ad_value(240))));
        }

        if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (s.v[1273] != 0.0)) {
            s.store_mul_ad_rhs(336, 225, A::sub(s.ad_value(1236), s.ad_value(1271)));
        }

        if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (s.v[1273] != 0.0)) {
            s.store_div_from_scalar_ad(328, 1.0, A::mul(s.ad_value(225), s.ad_value(238)));
        }

        if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (s.v[1273] != 0.0)) {
            s.store_mul(337, 328, 323);
        }

        if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (s.v[1273] != 0.0)) {
            s.store_offset_scaled(262, 337, (3.0 * 1.414213562373095), 2.0);
        }

        if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (s.v[1273] != 0.0)) {
            s.store_mul_ad_lhs(260, A::mul(A::scale(s.ad_value(262), 8.0), s.ad_value(262)), 262);
        }

        if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (s.v[1273] != 0.0)) {
            s.store_offset(331, 336, (-2.0));
        }

        if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (s.v[1273] != 0.0)) {
            s.store_mul_ad_lhs(332, A::scale(s.ad_value(337), 9.0), 331);
        }

        if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (s.v[1273] != 0.0)) {
            s.store_sub_from_scalar(261, (7.0 * 1.414213562373095), 332);
        }

        if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (s.v[1273] != 0.0)) {
            s.store_square(259, 261);
        }

        s.v[1274] = if (s.v[260] < (s.v[259] * 1e-8)) { 1.0 } else { 0.0 };

        if ((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (s.v[1273] != 0.0)) && (s.v[1274] != 0.0)) {
            s.store_add_ad_lhs(257, A::add(A::offset(s.ad_value(261), ((-7.0) * 1.414213562373095)), A::div(A::scale(s.ad_value(260), 0.5), s.ad_value(261))), 332);
        }

        if ((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (s.v[1273] != 0.0)) && (!(s.v[1274] != 0.0))) {
            s.store_sqrt_ad(258, A::add(s.ad_value(260), s.ad_value(259)));
        }

        if ((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (s.v[1273] != 0.0)) && (!(s.v[1274] != 0.0))) {
            s.store_add_ad_lhs(257, A::offset(s.ad_value(258), ((-7.0) * 1.414213562373095)), 332);
        }

    }

    pub(super) fn stamp_transient_block_19(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (s.v[1273] != 0.0)) {
            s.store_powf(256, 257, 0.3333333333333333);
        }

        if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (s.v[1273] != 0.0)) {
            s.store_add_ad(255, A::add(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(337), 12.0)), A::scale(s.ad_value(256), 2.0)), A::mul(A::scale(s.ad_value(256), 1.414213562373095), s.ad_value(256)));
        }

        if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (s.v[1273] != 0.0)) {
            s.store_div_from_scalar(328, 1.0, 256);
        }

        if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (s.v[1273] != 0.0)) {
            s.store_mul(181, 255, 328);
        }

        if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (s.v[1273] != 0.0)) {
            s.store_add_ad_lhs(313, A::mul(s.ad_value(181), s.ad_value(227)), 1271);
        }

        if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (s.v[1273] != 0.0)) {
            s.store_sub(328, 313, 1271);
        }

        if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (s.v[1273] != 0.0)) {
            s.store_div(329, 328, 254);
        }

        if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (s.v[1273] != 0.0)) {
            s.store_sqrt_ad(330, A::offset(A::square(s.ad_value(329)), 1.0));
        }

        if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (s.v[1273] != 0.0)) {
            s.store_add_ad_lhs(1234, A::div(s.ad_value(328), s.ad_value(330)), 1271);
        }

        if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) {
            s.store_exp_ad(484, A::mul(s.ad_value(225), A::offset(s.ad_value(1271), (-p.p287))));
        }

        if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) {
            s.store_scalar(430, 0.0);
        }

        if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) {
            s.copy_ad(1272, 378);
        }

        if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) {
            s.store_scale_ad(419, A::scale(s.ad_value(229), (p.p237 * (p.p237 * 0.5))), 9662367879.197212);
        }

        if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) {
            s.store_sqrt_ad(327, A::mul(A::scale(s.ad_value(225), 2.0), s.ad_value(419)));
        }

        if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) {
            s.store_scale_ad(328, A::add(A::exp(s.ad_value(327)), A::exp(A::neg(s.ad_value(327)))), 0.5);
        }

        if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) {
            s.store_div_ad_lhs(420, A::ln(s.ad_value(328)), 419);
        }

        if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) {
            s.store_scalar(167, 1.0);
        }

        let mut assign21320_loop_guard: usize = 0;
        while {
            let assign21320_cond_e29284: f64 = (s.v[57] + 1.0);
            let assign21320_cond_e29286: f64 = if ((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (s.v[167] <= assign21320_cond_e29284)) { 1.0 } else { 0.0 };
            assign21320_cond_e29286 != 0.0
        } {
            assign21320_loop_guard += 1;
            assert!(assign21320_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) {
                s.store_sub(417, 1272, 1271);
            }
            if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) {
                s.store_mul(181, 225, 417);
            }
            if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) {
                s.store_mul_ad_rhs(337, 420, A::sub(s.ad_value(417), s.ad_value(419)));
            }
            s.v[1275] = if (s.v[337] < 80.0) { 1.0 } else { 0.0 };
            if ((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (s.v[1275] != 0.0)) {
                s.store_exp(328, 337);
            }
            if ((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (s.v[1275] != 0.0)) {
                s.store_exp_ad(327, A::mul(A::neg(s.ad_value(420)), s.ad_value(419)));
            }
            if ((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (s.v[1275] != 0.0)) {
                s.store_sub(329, 328, 327);
            }
            if ((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (s.v[1275] != 0.0)) {
                s.store_div_ad_lhs(422, A::ln(A::offset(s.ad_value(329), 1.0)), 420);
            }
            if ((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (s.v[1275] != 0.0)) {
                s.store_div_ad_rhs(423, 328, A::offset(s.ad_value(329), 1.0));
            }
            if ((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (!(s.v[1275] != 0.0))) {
                s.store_sub(422, 417, 419);
            }
            if ((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (!(s.v[1275] != 0.0))) {
                s.store_scalar(423, 1.0);
            }
            if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) {
                s.store_mul(421, 225, 422);
            }
            s.v[1276] = if (((s.v[181]) as f64).abs() < 1e-16) { 1.0 } else { 0.0 };
            if ((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (s.v[1276] != 0.0)) {
                s.store_sqrt_ad(327, A::scale(A::sub_from_scalar(1.0, A::square(s.ad_value(423))), 0.5));
            }
            if ((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (s.v[1276] != 0.0)) {
                s.store_mul(242, 181, 327);
            }
            if ((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (s.v[1276] != 0.0)) {
                s.store_mul(443, 225, 327);
            }
            s.v[1277] = if (s.v[181] < 0.0) { 1.0 } else { 0.0 };
            if (((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (s.v[1276] != 0.0)) && (s.v[1277] != 0.0)) {
                s.store_neg(242, 242);
            }
            if (((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (s.v[1276] != 0.0)) && (s.v[1277] != 0.0)) {
                s.store_neg(443, 443);
            }
            s.v[1278] = if (((s.v[181]) as f64).abs() < 0.005) { 1.0 } else { 0.0 };
            if (((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (!(s.v[1276] != 0.0))) && (s.v[1278] != 0.0)) {
                s.store_mul_ad(327, A::scale(A::square(s.ad_value(181)), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(181), 0.3333333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(181), 0.25), A::sub_from_scalar(1.0, A::scale(s.ad_value(181), 0.2)))))));
            }
            if (((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (!(s.v[1276] != 0.0))) && (s.v[1278] != 0.0)) {
                s.store_mul_ad_rhs(328, 181, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(181), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(181), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(181), 0.25)))))));
            }
            if (((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (!(s.v[1276] != 0.0))) && (s.v[1278] != 0.0)) {
                s.store_mul_ad(329, A::scale(A::square(s.ad_value(421)), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(421), 0.3333333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(421), 0.25), A::sub_from_scalar(1.0, A::scale(s.ad_value(421), 0.2)))))));
            }
            if (((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (!(s.v[1276] != 0.0))) && (s.v[1278] != 0.0)) {
                s.store_mul_ad_rhs(330, 421, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(421), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(421), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(421), 0.25)))))));
            }
            if (((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (!(s.v[1276] != 0.0))) && (s.v[1278] != 0.0)) {
                s.store_sqrt_ad(242, A::sub(s.ad_value(327), s.ad_value(329)));
            }
            if (((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (!(s.v[1276] != 0.0))) && (s.v[1278] != 0.0)) {
                s.store_div_ad_lhs(443, A::mul(A::scale(s.ad_value(225), 0.5), A::sub(s.ad_value(328), A::mul(s.ad_value(423), s.ad_value(330)))), 242);
            }
            if (((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (!(s.v[1276] != 0.0))) && (!(s.v[1278] != 0.0))) {
                s.store_exp_ad(327, A::neg(s.ad_value(181)));
            }
            if (((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (!(s.v[1276] != 0.0))) && (!(s.v[1278] != 0.0))) {
                s.store_exp_ad(328, A::neg(s.ad_value(421)));
            }
            if (((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (!(s.v[1276] != 0.0))) && (!(s.v[1278] != 0.0))) {
                s.store_sqrt_ad(242, A::add(A::sub(s.ad_value(181), s.ad_value(421)), A::sub(s.ad_value(327), s.ad_value(328))));
            }
            if (((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (!(s.v[1276] != 0.0))) && (!(s.v[1278] != 0.0))) {
                s.store_div_ad_lhs(443, A::mul(A::scale(s.ad_value(225), 0.5), A::sub(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul(s.ad_value(423), A::sub_from_scalar(1.0, s.ad_value(328))))), 242);
            }
            s.v[1279] = if ((s.v[430] == 1.0) && (s.v[181] < 0.0)) { 1.0 } else { 0.0 };
            if ((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (s.v[1279] != 0.0)) {
                s.store_scalar(338, (-1.0));
            }
            s.v[1280] = if (s.v[181] < 0.0) { 1.0 } else { 0.0 };
            if ((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (s.v[1280] != 0.0)) {
                s.store_neg(490, 242);
            }
            if ((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (s.v[1280] != 0.0)) {
                s.store_neg(491, 443);
            }
            s.v[1281] = if (s.v[181] < 1e-7) { 1.0 } else { 0.0 };
            if (((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (!(s.v[1280] != 0.0))) && (s.v[1281] != 0.0)) {
                s.copy_ad(490, 242);
            }
            if (((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (!(s.v[1280] != 0.0))) && (s.v[1281] != 0.0)) {
                s.copy_ad(491, 443);
            }
            if (((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (!(s.v[1280] != 0.0))) && (!(s.v[1281] != 0.0))) {
                s.store_mul_ad_rhs(501, 225, A::offset(s.ad_value(1272), (-p.p287)));
            }
            if (((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (!(s.v[1280] != 0.0))) && (!(s.v[1281] != 0.0))) {
                s.store_exp(502, 501);
            }
            if (((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (!(s.v[1280] != 0.0))) && (!(s.v[1281] != 0.0))) {
                s.store_mul_ad_rhs(488, 379, A::sub(s.ad_value(502), A::mul(s.ad_value(484), A::offset(s.ad_value(181), 1.0))));
            }
            if (((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (!(s.v[1280] != 0.0))) && (!(s.v[1281] != 0.0))) {
                s.store_mul_ad(489, A::mul(s.ad_value(379), s.ad_value(225)), A::sub(s.ad_value(502), s.ad_value(484)));
            }
            if (((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (!(s.v[1280] != 0.0))) && (!(s.v[1281] != 0.0))) {
                s.store_sqrt_ad(490, A::add(A::square(s.ad_value(242)), s.ad_value(488)));
            }
            if (((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (!(s.v[1280] != 0.0))) && (!(s.v[1281] != 0.0))) {
                s.store_div_ad_lhs(491, A::scale(A::add(A::mul(A::scale(s.ad_value(443), 2.0), s.ad_value(242)), s.ad_value(489)), 0.5), 490);
            }
            if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) {
                s.store_add_ad(492, A::sub(s.ad_value(1272), s.ad_value(1236)), A::mul(s.ad_value(240), s.ad_value(490)));
            }
            if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) {
                s.store_offset_ad(493, A::mul(s.ad_value(240), s.ad_value(491)), 1.0);
            }
            s.v[1282] = if (s.v[430] == 1.0) { 1.0 } else { 0.0 };
            if ((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (s.v[1282] != 0.0)) {
                s.store_scalar(167, (s.v[57] + 1.0));
            }
            if ((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (!(s.v[1282] != 0.0))) {
                s.store_div_ad_lhs(494, A::neg(s.ad_value(492)), 493);
            }
            if ((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (!(s.v[1282] != 0.0))) {
                s.store_scale_ad(496, A::offset({
                    if (1.0 >= ((s.v[1272]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1272))
                    }
                }, 1.0), (0.5 * 0.1));
            }
            s.v[1283] = if (((s.v[494]) as f64).abs() > s.v[496]) { 1.0 } else { 0.0 };
            if (((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (!(s.v[1282] != 0.0))) && (s.v[1283] != 0.0)) {
                s.store_scale(494, 496, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (!(s.v[1282] != 0.0))) {
                s.store_add(1272, 1272, 494);
            }
            s.v[1284] = if ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[492]) as f64).abs() <= 1e-8)) { 1.0 } else { 0.0 };
            if (((((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) && (!(s.v[1282] != 0.0))) && (s.v[1284] != 0.0)) {
                s.store_scalar(430, 1.0);
            }
            if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) {
                s.store_offset(167, 167, 1.0);
            }
        }

        if (((s.v[1224] != 0.0) && (s.v[1270] != 0.0)) && (!(s.v[1273] != 0.0))) {
            s.copy_ad(1234, 1272);
        }

        if (s.v[1224] != 0.0) {
            s.store_mul_ad(332, A::neg(s.ad_value(225)), A::sub(s.ad_value(1234), s.ad_value(1225)));
        }

        if (s.v[1224] != 0.0) {
            s.store_scalar(1249, (if (s.v[332] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if (s.v[1224] != 0.0) {
            s.store_mul(1250, 1249, 332);
        }

        if (s.v[1224] != 0.0) {
            s.store_exp(333, 332);
        }

        if (s.v[1224] != 0.0) {
            s.store_sub_ad_lhs(334, A::offset(s.ad_value(333), (-1.0)), 332);
        }

        s.v[1285] = if (s.v[332] > 1e-7) { 1.0 } else { 0.0 };

        if ((s.v[1224] != 0.0) && (s.v[1285] != 0.0)) {
            s.store_mul_ad(437, A::neg(s.ad_value(238)), A::sqrt(s.ad_value(334)));
        }

        s.v[1286] = if (s.v[1250] > 1e-7) { 1.0 } else { 0.0 };

        if (((s.v[1224] != 0.0) && (!(s.v[1285] != 0.0))) && (s.v[1286] != 0.0)) {
            s.store_mul_ad_rhs(437, 238, A::sqrt(s.ad_value(334)));
        }

        if (((s.v[1224] != 0.0) && (!(s.v[1285] != 0.0))) && (!(s.v[1286] != 0.0))) {
            s.store_mul_ad(437, A::scale(A::mul(A::neg(s.ad_value(1249)), s.ad_value(1250)), 0.7071067811865475), A::sqrt(A::offset(A::mul(A::scale(s.ad_value(1250), 0.3333333333333333), A::offset(A::scale(s.ad_value(1250), 0.25), 1.0)), 1.0)));
        }

        if (s.v[1224] != 0.0) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(437)), ((4.0 * 1e-6) * 1e-6)));
        }

        if (s.v[1224] != 0.0) {
            s.store_offset_ad(1246, A::scale(A::add(s.ad_value(437), s.ad_value(44)), 0.5), (1e-10 * 1e-6));
        }

        s.v[1287] = if (s.v[1246] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1224] != 0.0) && (s.v[1287] != 0.0)) {
            s.store_scalar(1246, 0.0);
        }

        if (s.v[1224] != 0.0) {
            s.store_div_ad_rhs(1247, 1246, A::scale(s.ad_value(536), 1.6021918e-19));
        }

        if (s.v[1224] != 0.0) {
            s.store_sub(328, 1247, 1238);
        }

        if (s.v[1224] != 0.0) {
            s.store_scale(1248, 1247, 0.01);
        }

        if (s.v[1224] != 0.0) {
            s.store_sqrt_ad(44, A::add(A::square(s.ad_value(328)), A::mul(A::scale(s.ad_value(1248), 4.0), s.ad_value(1248))));
        }

        if (s.v[1224] != 0.0) {
            s.store_add_ad(329, A::scale(A::add(s.ad_value(328), s.ad_value(44)), 0.5), A::scale(s.ad_value(1248), 1e-10));
        }

        s.v[1288] = if (s.v[329] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1224] != 0.0) && (s.v[1288] != 0.0)) {
            s.store_scalar(329, 0.0);
        }

        if (s.v[1224] != 0.0) {
            s.store_div_ad_lhs(1245, A::mul(A::div(s.ad_value(329), s.ad_value(1247)), s.ad_value(329)), 1247);
        }

        if (s.v[1224] != 0.0) {
            s.store_add_ad_lhs(1228, A::mul(A::sub(s.ad_value(1234), s.ad_value(1225)), s.ad_value(1245)), 1225);
        }

        if (s.v[1224] != 0.0) {
            s.store_sub_ad(337, A::exp(A::mul(s.ad_value(225), s.ad_value(1228))), A::exp(A::mul(s.ad_value(225), A::sub(s.ad_value(1228), s.ad_value(157)))));
        }

        if (s.v[1224] != 0.0) {
            s.store_sqrt_ad(1241, A::scale(s.ad_value(1229), ((2.0 * 1.6021918e-19) * 1.034943e-10)));
        }

        if (s.v[1224] != 0.0) {
            s.store_mul_ad_rhs(1242, 1241, A::sqrt(s.ad_value(227)));
        }

        if (s.v[1224] != 0.0) {
            s.store_mul_ad_rhs(1233, 225, A::sub(s.ad_value(1228), s.ad_value(1225)));
        }

        s.v[1289] = if ((s.v[1233] < (0.2 * s.v[225])) && ((0.2 * s.v[225]) >= 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1224] != 0.0) && (s.v[1289] != 0.0)) {
            s.store_sub_ad_lhs(44, A::scale(s.ad_value(225), 0.2), 1233);
        }

        if ((s.v[1224] != 0.0) && (s.v[1289] != 0.0)) {
            s.store_square(49, 44);
        }

        if ((s.v[1224] != 0.0) && (s.v[1289] != 0.0)) {
            s.store_mul_ad(50, A::scale(s.ad_value(225), 0.2), A::scale(s.ad_value(225), 0.2));
        }

        if ((s.v[1224] != 0.0) && (s.v[1289] != 0.0)) {
            s.store_scalar(51, 1.0);
        }

        if ((s.v[1224] != 0.0) && (s.v[1289] != 0.0)) {
            s.store_scalar(52, 1.0);
        }

        if ((s.v[1224] != 0.0) && (s.v[1289] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        if ((s.v[1224] != 0.0) && (s.v[1289] != 0.0)) {
            s.store_scalar(55, 0.0);
        }

        if ((s.v[1224] != 0.0) && (s.v[1289] != 0.0)) {
            s.store_scalar(48, 0.0);
        }

        if ((s.v[1224] != 0.0) && (s.v[1289] != 0.0)) {
            s.store_scalar(53, 0.0);
        }

        if ((s.v[1224] != 0.0) && (s.v[1289] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((s.v[1224] != 0.0) && (s.v[1289] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((s.v[1224] != 0.0) && (s.v[1289] != 0.0)) {
            s.store_add(48, 51, 52);
        }

        if ((s.v[1224] != 0.0) && (s.v[1289] != 0.0)) {
            s.copy_ad(53, 48);
        }

        s.v[1290] = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1291] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[1224] != 0.0) && (s.v[1289] != 0.0)) && (s.v[1290] != 0.0)) && (s.v[1291] != 0.0)) {
            s.store_scalar(55, 1.0);
        }

        s.v[1292] = if (1.0 == 2.0) { 1.0 } else { 0.0 };

        if (((((s.v[1224] != 0.0) && (s.v[1289] != 0.0)) && (s.v[1290] != 0.0)) && (!(s.v[1291] != 0.0))) && (s.v[1292] != 0.0)) {
            s.store_scalar(55, 2.0);
        }

        s.v[1293] = if (1.0 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1224] != 0.0) && (s.v[1289] != 0.0)) && (s.v[1290] != 0.0)) && (!(s.v[1291] != 0.0))) && (!(s.v[1292] != 0.0))) && (s.v[1293] != 0.0)) {
            s.store_scalar(55, 3.0);
        }

        s.v[1294] = if (1.0 == 8.0) { 1.0 } else { 0.0 };

        if (((((((s.v[1224] != 0.0) && (s.v[1289] != 0.0)) && (s.v[1290] != 0.0)) && (!(s.v[1291] != 0.0))) && (!(s.v[1292] != 0.0))) && (!(s.v[1293] != 0.0))) && (s.v[1294] != 0.0)) {
            s.store_scalar(55, 4.0);
        }

        if (((s.v[1224] != 0.0) && (s.v[1289] != 0.0)) && (s.v[1290] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        let mut assign21850_loop_guard: usize = 0;
        while {
            let assign21850_cond_e30601: f64 = if ((((s.v[1224] != 0.0) && (s.v[1289] != 0.0)) && (s.v[1290] != 0.0)) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign21850_cond_e30601 != 0.0
        } {
            assign21850_loop_guard += 1;
            assert!(assign21850_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[1224] != 0.0) && (s.v[1289] != 0.0)) && (s.v[1290] != 0.0)) {
                s.store_sqrt(53, 53);
            }
            if (((s.v[1224] != 0.0) && (s.v[1289] != 0.0)) && (s.v[1290] != 0.0)) {
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((s.v[1224] != 0.0) && (s.v[1289] != 0.0)) && (!(s.v[1290] != 0.0))) {
            s.store_powf(53, 53, (1.0 / 2.0));
        }

        if ((s.v[1224] != 0.0) && (s.v[1289] != 0.0)) {
            s.store_div_from_scalar(53, 1.0, 53);
        }

        if ((s.v[1224] != 0.0) && (s.v[1289] != 0.0)) {
            s.store_mul_ad_lhs(43, A::mul(s.ad_value(44), A::scale(s.ad_value(225), 0.2)), 53);
        }

        if ((s.v[1224] != 0.0) && (s.v[1289] != 0.0)) {
            s.store_sub_ad_lhs(328, A::scale(s.ad_value(225), 0.2), 43);
        }

        if ((s.v[1224] != 0.0) && (!(s.v[1289] != 0.0))) {
            s.copy_ad(328, 1233);
        }

        if (s.v[1224] != 0.0) {
            s.store_sqrt_ad(1243, A::offset(s.ad_value(328), (10.0 * 2.220446049250313e-16)));
        }

        if (s.v[1224] != 0.0) {
            s.store_mul(1244, 1242, 1243);
        }

        if (s.v[1224] != 0.0) {
            s.store_mul_ad_lhs(1240, A::div(A::scale(s.ad_value(227), 2.0), s.ad_value(1231)), 1244);
        }

        if (s.v[1224] != 0.0) {
            s.store_mul_ad_lhs(204, A::mul(A::mul(s.ad_value(1240), s.ad_value(1239)), s.ad_value(107)), 337);
        }

        if (s.v[1224] != 0.0) {
            s.store_add(199, 202, 204);
        }

        s.store_add(201, 203, 204);

        s.v[1295] = if ((p.p43 == 1.0) || (p.p45 == 1.0)) { 1.0 } else { 0.0 };

        s.v[1308] = if ((s.v[145] == 1.0) || (p.p25 == 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1295] != 0.0) && (s.v[1308] != 0.0)) {
            s.store_scalar(263, 0.0);
        }

        s.v[1309] = if ((p.p117 <= 0.0) || (s.v[73] <= 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (s.v[1309] != 0.0)) {
            s.store_scalar(263, 0.0);
        }

        if (((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) {
            s.store_offset_ad(445, A::sub(A::add(A::offset(s.ad_value(174), (-s.v[136])), s.ad_value(185)), s.ad_value(320)), p.p48);
        }

        s.v[1310] = if (p.p44 <= 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (s.v[1310] != 0.0)) {
            s.copy_ad(1296, 445);
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (s.v[1310] != 0.0)) {
            s.store_square(1303, 323);
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (s.v[1310] != 0.0)) {
            s.copy_ad(1304, 545);
        }

    }

    pub(super) fn stamp_transient_block_20(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (s.v[1310] != 0.0)) {
            s.store_div(1298, 1304, 1303);
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (s.v[1310] != 0.0)) {
            s.store_div_from_scalar(1305, 2.0, 1304);
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (s.v[1310] != 0.0)) {
            s.store_mul(1299, 1305, 1303);
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (s.v[1310] != 0.0)) {
            s.store_sub_ad(1300, A::sub(s.ad_value(1296), s.ad_value(227)), A::mul(s.ad_value(130), s.ad_value(514)));
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (s.v[1310] != 0.0)) {
            s.store_scale(483, 393, (p.p49 * 1.0 / (s.v[89])));
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (s.v[1310] != 0.0)) {
            s.store_sub_ad_rhs(1300, 1300, A::mul(s.ad_value(130), s.ad_value(483)));
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (s.v[1310] != 0.0)) {
            s.store_offset_ad(1302, A::mul(s.ad_value(1299), s.ad_value(1300)), 1.0);
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (s.v[1310] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1302)), ((4.0 * 0.001) * 0.001)));
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (s.v[1310] != 0.0)) {
            s.store_offset_ad(1301, A::scale(A::add(s.ad_value(1302), s.ad_value(44)), 0.5), (1e-10 * 0.001));
        }

        s.v[1311] = if (s.v[1301] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (s.v[1310] != 0.0)) && (s.v[1311] != 0.0)) {
            s.store_scalar(1301, 0.0);
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (s.v[1310] != 0.0)) {
            s.store_offset(1301, 1301, 1e-50);
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (s.v[1310] != 0.0)) {
            s.store_sqrt(1301, 1301);
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (s.v[1310] != 0.0)) {
            s.store_add_ad(1306, A::mul(s.ad_value(1296), s.ad_value(137)), A::mul(s.ad_value(1298), A::sub_from_scalar(1.0, s.ad_value(1301))));
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (s.v[1310] != 0.0)) {
            s.store_sub_ad(1307, A::add(A::scale(s.ad_value(173), p.p122), s.ad_value(176)), A::mul(A::mul(s.ad_value(131), s.ad_value(129)), s.ad_value(1306)));
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (s.v[1310] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1307)), ((4.0 * 0.01) * 0.01)));
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (s.v[1310] != 0.0)) {
            s.store_offset_ad(1307, A::scale(A::add(s.ad_value(1307), s.ad_value(44)), 0.5), (1e-10 * 0.01));
        }

        s.v[1312] = if (s.v[1307] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (s.v[1310] != 0.0)) && (s.v[1312] != 0.0)) {
            s.store_scalar(1307, 0.0);
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) {
            s.store_mul(1296, 134, 445);
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) {
            s.store_div_ad_rhs(1298, 545, A::square(s.ad_value(323)));
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) {
            s.store_mul_ad(1299, A::div_from_scalar(2.0, s.ad_value(545)), A::square(s.ad_value(323)));
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) {
            s.store_sub_ad(1300, A::sub(s.ad_value(1296), s.ad_value(227)), A::mul(s.ad_value(130), s.ad_value(514)));
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) {
            s.store_scale(483, 393, (p.p49 * 1.0 / (s.v[89])));
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) {
            s.store_sub_ad_rhs(1300, 1300, A::mul(s.ad_value(130), s.ad_value(483)));
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) {
            s.store_offset_ad(1301, A::mul(s.ad_value(1299), s.ad_value(1300)), 1.0);
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) {
            s.store_scaled_offset(1303, 1299, 1.0, 2.0);
        }

        s.v[1313] = if ((s.v[1301] < (1e-50 + s.v[1303])) && (s.v[1303] >= 0.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) {
            s.store_sub_ad_lhs(44, A::offset(s.ad_value(1303), 1e-50), 1301);
        }

        if (((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) {
            s.store_square(49, 44);
        }

        if (((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) {
            s.store_square(50, 1303);
        }

        if (((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) {
            s.store_scalar(51, 1.0);
        }

        if (((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) {
            s.store_scalar(52, 1.0);
        }

        if (((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        if (((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) {
            s.store_scalar(55, 0.0);
        }

        if (((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) {
            s.store_scalar(48, 0.0);
        }

        if (((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) {
            s.store_scalar(53, 0.0);
        }

        if (((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if (((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if (((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if (((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if (((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if (((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if (((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if (((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if (((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) {
            s.store_add(48, 51, 52);
        }

        if (((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) {
            s.copy_ad(53, 48);
        }

        s.v[1314] = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1315] = if (4.0 == 1.0) { 1.0 } else { 0.0 };

        if (((((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) && (s.v[1314] != 0.0)) && (s.v[1315] != 0.0)) {
            s.store_scalar(55, 1.0);
        }

        s.v[1316] = if (4.0 == 2.0) { 1.0 } else { 0.0 };

        if ((((((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) && (s.v[1314] != 0.0)) && (!(s.v[1315] != 0.0))) && (s.v[1316] != 0.0)) {
            s.store_scalar(55, 2.0);
        }

        s.v[1317] = if (4.0 == 4.0) { 1.0 } else { 0.0 };

        if (((((((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) && (s.v[1314] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1316] != 0.0))) && (s.v[1317] != 0.0)) {
            s.store_scalar(55, 3.0);
        }

        s.v[1318] = if (4.0 == 8.0) { 1.0 } else { 0.0 };

        if ((((((((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) && (s.v[1314] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1316] != 0.0))) && (!(s.v[1317] != 0.0))) && (s.v[1318] != 0.0)) {
            s.store_scalar(55, 4.0);
        }

        if ((((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) && (s.v[1314] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        let mut assign22640_loop_guard: usize = 0;
        while {
            let assign22640_cond_e31720: f64 = if (((((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) && (s.v[1314] != 0.0)) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign22640_cond_e31720 != 0.0
        } {
            assign22640_loop_guard += 1;
            assert!(assign22640_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) && (s.v[1314] != 0.0)) {
                s.store_sqrt(53, 53);
            }
            if ((((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) && (s.v[1314] != 0.0)) {
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) && (!(s.v[1314] != 0.0))) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if (((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) {
            s.store_div_from_scalar(53, 1.0, 53);
        }

        if (((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) {
            s.store_mul_ad_lhs(43, A::mul(s.ad_value(44), s.ad_value(1303)), 53);
        }

        if (((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) {
            s.store_sub_ad_lhs(1301, A::offset(s.ad_value(1303), 1e-50), 43);
        }

        if (((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (!(s.v[1313] != 0.0))) {
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) {
            s.store_ad(1301, &{
                if (s.v[1301] <= 0.0) {
                    A::constant(0.0)
                } else {
                    A::sqrt(s.ad_value(1301))
                }
            });
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) {
            s.store_add_ad_rhs(1306, 1296, A::mul(s.ad_value(1298), A::sub_from_scalar(1.0, s.ad_value(1301))));
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) {
            s.store_div_from_scalar_ad(1297, s.v[100], A::offset(s.ad_value(131), s.v[100]));
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) {
            s.store_sub_ad(1307, A::add(A::scale(s.ad_value(173), p.p122), s.ad_value(176)), A::mul(s.ad_value(1297), s.ad_value(1306)));
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1307)), ((4.0 * 0.001) * 0.001)));
        }

        if ((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) {
            s.store_offset_ad(1307, A::scale(A::add(s.ad_value(1307), s.ad_value(44)), 0.5), (1e-10 * 0.001));
        }

        s.v[1319] = if (s.v[1307] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1319] != 0.0)) {
            s.store_scalar(1307, 0.0);
        }

        if (((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) {
            s.store_offset(1307, 1307, 1e-50);
        }

        if (((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) {
            s.store_exp_ad(1297, A::div(A::neg(s.ad_value(133)), s.ad_value(1307)));
        }

        if (((s.v[1295] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) {
            s.store_mul_ad_lhs(263, A::mul(A::mul(s.ad_value(132), s.ad_value(1307)), s.ad_value(199)), 1297);
        }

        s.v[1320] = if (((p.p25 == 1.0) && (p.p26 == 2.0)) && (p.p43 == 1.0)) { 1.0 } else { 0.0 };

        if (s.v[1320] != 0.0) {
            s.store_mul_ad(1321, A::scale(s.ad_value(107), (1.6021918e-19 * p.p237)), A::exp(A::scale(A::neg(s.ad_value(225)), p.p141)));
        }

        if (s.v[1320] != 0.0) {
            s.store_scale(1324, 227, 0.0);
        }

        if (s.v[1320] != 0.0) {
            s.store_sub_ad(44, A::sub(s.ad_value(231), s.ad_value(1324)), A::scale(s.ad_value(231), 0.01));
        }

        if (s.v[1320] != 0.0) {
            s.store_mul_ad(45, A::scale(s.ad_value(231), 4.0), A::scale(s.ad_value(231), 0.01));
        }

        if (s.v[1320] != 0.0) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (s.v[1320] != 0.0) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if (s.v[1320] != 0.0) {
            s.store_sub_ad_rhs(1324, 231, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if (s.v[1320] != 0.0) {
            s.store_sqrt_ad(1325, A::mul(A::scale(s.ad_value(544), ((2.0 * 1.034943e-10) * 1.6021918e-19)), s.ad_value(227)));
        }

        if (s.v[1320] != 0.0) {
            s.store_mul_ad_rhs(1326, 225, A::sub(s.ad_value(176), s.ad_value(1324)));
        }

        if (s.v[1320] != 0.0) {
            s.store_ad(1326, &{
                if (s.v[1326] > 0.0) {
                    A::sqrt(s.ad_value(1326))
                } else {
                    A::neg(A::sqrt(A::neg(s.ad_value(1326))))
                }
            });
        }

        if (s.v[1320] != 0.0) {
            s.store_sqrt_ad(1327, A::mul(s.ad_value(225), s.ad_value(176)));
        }

        if (s.v[1320] != 0.0) {
            s.store_mul_ad(1328, A::neg(s.ad_value(1325)), A::sub(s.ad_value(1326), s.ad_value(1327)));
        }

        if (s.v[1320] != 0.0) {
            s.store_offset_ad(44, A::sub_from_scalar(p.p47, s.ad_value(1328)), (-(p.p47 * 0.01)));
        }

        if (s.v[1320] != 0.0) {
            s.store_scalar(45, ((4.0 * p.p47) * (p.p47 * 0.01)));
        }

        if (s.v[1320] != 0.0) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (s.v[1320] != 0.0) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if (s.v[1320] != 0.0) {
            s.store_sub_from_scalar_ad(393, p.p47, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if (s.v[1320] != 0.0) {
            s.store_scalar(1321, (if (p.p138 > 0.0) { p.p138 } else { 1.0 }));
        }

        if (s.v[1320] != 0.0) {
            s.store_div_ad_rhs(398, 1321, A::offset(s.ad_value(263), p.p139));
        }

        if (s.v[1320] != 0.0) {
            s.store_mul(397, 398, 323);
        }

        if (s.v[1320] != 0.0) {
            s.copy_ad(396, 393);
        }

        if (s.v[1320] != 0.0) {
            s.store_ad(596, &A::scale(A::voltage(ctx, &nodes, Some(17), None), (1e-9 / 0.0001)));
        }

        if (s.v[1320] != 0.0) {
            s.copy_ad(393, 596);
        }

        if (s.v[1320] != 0.0) {
            s.store_div_ad_lhs(592, A::sub(s.ad_value(596), s.ad_value(396)), 397);
        }

        s.v[1342] = if (((s.v[145] == 0.0) && (s.v[263] > 0.0)) && (p.p146 != 0.0)) { 1.0 } else { 0.0 };

        s.v[1343] = if (s.v[56] < 3.0) { 1.0 } else { 0.0 };

        if ((s.v[1342] != 0.0) && (s.v[1343] != 0.0)) {
            s.store_scalar(516, 0.0);
        }

        if ((s.v[1342] != 0.0) && (s.v[1343] != 0.0)) {
            s.store_scalar(517, 0.0);
        }

        if ((s.v[1342] != 0.0) && (!(s.v[1343] != 0.0))) {
            s.store_ad(516, &{
                if (p.p43 == 1.0) {
                    s.ad_value(156)
                } else {
                    s.ad_value(350)
                }
            });
        }

        if ((s.v[1342] != 0.0) && (!(s.v[1343] != 0.0))) {
            s.store_ad(517, &{
                if (p.p43 == 1.0) {
                    s.ad_value(156)
                } else {
                    s.ad_value(353)
                }
            });
        }

        if (s.v[1342] != 0.0) {
            s.store_offset_scaled(1329, 185, p.p147, 1.0);
        }

        if (s.v[1342] != 0.0) {
            s.store_mul_ad_lhs(1330, A::scale(s.ad_value(1329), p.p146), 263);
        }

        if (s.v[1342] != 0.0) {
            s.store_offset_ad(1331, A::mul(s.ad_value(225), A::sub(s.ad_value(161), s.ad_value(516))), (-1.0));
        }

        if (s.v[1342] != 0.0) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1331)), ((4.0 * 0.1) * 0.1)));
        }

        if (s.v[1342] != 0.0) {
            s.store_offset_ad(1331, A::scale(A::add(s.ad_value(1331), s.ad_value(44)), 0.5), (1e-10 * 0.1));
        }

        s.v[1344] = if (s.v[1331] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1342] != 0.0) && (s.v[1344] != 0.0)) {
            s.store_scalar(1331, 0.0);
        }

        if (s.v[1342] != 0.0) {
            s.store_sqrt(1332, 1331);
        }

        if (s.v[1342] != 0.0) {
            s.store_mul(1333, 1331, 1332);
        }

        if (s.v[1342] != 0.0) {
            s.store_offset_ad(1334, A::mul(s.ad_value(225), A::sub(s.ad_value(162), s.ad_value(517))), (-1.0));
        }

        if (s.v[1342] != 0.0) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1334)), ((4.0 * 0.1) * 0.1)));
        }

        if (s.v[1342] != 0.0) {
            s.store_offset_ad(1334, A::scale(A::add(s.ad_value(1334), s.ad_value(44)), 0.5), (1e-10 * 0.1));
        }

        s.v[1345] = if (s.v[1334] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1342] != 0.0) && (s.v[1345] != 0.0)) {
            s.store_scalar(1334, 0.0);
        }

        if (s.v[1342] != 0.0) {
            s.store_sqrt(1335, 1334);
        }

        if (s.v[1342] != 0.0) {
            s.store_mul(1336, 1334, 1335);
        }

        if (s.v[1342] != 0.0) {
            s.store_div_from_scalar(1337, 1.0, 1331);
        }

        if (s.v[1342] != 0.0) {
            s.store_mul_ad_lhs(328, A::mul(s.ad_value(225), s.ad_value(1330)), 1337);
        }

    }

    pub(super) fn stamp_transient_block_21(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1342] != 0.0) {
            s.store_div_from_scalar(1337, 1.0, 1334);
        }

        if (s.v[1342] != 0.0) {
            s.store_mul_ad_lhs(1338, A::mul(s.ad_value(225), s.ad_value(1330)), 1337);
        }

        if (s.v[1342] != 0.0) {
            s.store_mul_ad_rhs(1339, 238, A::sub(A::mul(s.ad_value(1336), s.ad_value(1338)), A::mul(s.ad_value(1333), s.ad_value(328))));
        }

        if (s.v[1342] != 0.0) {
            s.store_mul_ad(1340, A::scale(s.ad_value(238), 0.5), A::add(A::mul(A::neg(s.ad_value(1335)), s.ad_value(1338)), A::mul(s.ad_value(1332), s.ad_value(328))));
        }

        if (s.v[1342] != 0.0) {
            s.store_add(1341, 1339, 1340);
        }

        if (s.v[1342] != 0.0) {
            s.store_mul_ad_lhs(265, A::mul(s.ad_value(264), s.ad_value(1341)), 250);
        }

        s.v[1359] = (s.v[88] * 100.0);

        s.store_scale(1360, 323, 0.0001);

        s.v[1361] = (s.v[97] * 100.0);

        s.store_scale(1362, 107, 100.0);

        s.store_scale(1363, 252, 0.01);

        s.store_scale(1364, 436, 0.0001);

        s.store_scale(1365, 238, 0.0001);

        s.v[1366] = if (p.p27 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1366] != 0.0) {
            s.store_scalar(309, 0.0);
        }

        if (s.v[1366] != 0.0) {
            s.store_scalar(306, 0.0);
        }

        if (s.v[1366] != 0.0) {
            s.store_scalar(307, 0.0);
        }

        if (s.v[1366] != 0.0) {
            s.store_scalar(308, 0.0);
        }

        if (s.v[1366] != 0.0) {
            s.store_scalar(310, 0.0);
        }

        s.v[1367] = if (s.v[145] == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) {
            s.store_offset_ad(1358, A::add(s.ad_value(176), s.ad_value(173)), (-(10.0 * 2.220446049250313e-16)));
        }

        if ((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) {
            s.store_sub_ad(1348, A::add(A::offset(s.ad_value(174), (-s.v[123])), A::scale(A::sub(s.ad_value(185), s.ad_value(320)), (p.p216 * s.v[1361]))), A::scale(s.ad_value(1358), p.p215));
        }

        if ((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) {
            s.store_scalar(1350, (1.0 / s.v[1359]));
        }

        if ((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) {
            s.store_mul(1349, 1348, 1350);
        }

        if ((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) {
            s.store_scalar(1350, (1.0 / p.p217));
        }

        if ((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) {
            s.store_offset_ad(1354, A::mul(s.ad_value(1363), s.ad_value(1350)), 1.0);
        }

        if ((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) {
            s.store_mul(1357, 1349, 1354);
        }

        if ((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1357)), ((4.0 * 0.01) * 0.01)));
        }

        if ((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) {
            s.store_offset_ad(1357, A::scale(A::add(s.ad_value(1357), s.ad_value(44)), 0.5), (1e-10 * 0.01));
        }

        s.v[1368] = if (s.v[1357] < 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) && (s.v[1368] != 0.0)) {
            s.store_scalar(1357, 0.0);
        }

        if ((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(174)), ((4.0 * 0.001) * 0.001)));
        }

        if ((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) {
            s.store_offset_ad(1350, A::scale(A::add(s.ad_value(174), s.ad_value(44)), 0.5), (1e-10 * 0.001));
        }

        s.v[1369] = if (s.v[1350] < 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) && (s.v[1369] != 0.0)) {
            s.store_scalar(1350, 0.0);
        }

        if ((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) {
            s.store_offset(1350, 1350, (-p.p226));
        }

        if ((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) {
            s.store_scale(1346, 1350, 10.0);
        }

        if ((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) {
            s.store_offset_ad(1349, A::square(s.ad_value(1346)), 1.0);
        }

        if ((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) {
            s.store_sub_from_scalar_ad(1348, 1.0, A::div_from_scalar(1.0, s.ad_value(1349)));
        }

        if ((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) {
            s.store_mul(1357, 1357, 1348);
        }

        if ((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) {
            s.store_scale(1347, 1362, s.v[1361]);
        }

        if ((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) {
            s.store_div_from_scalar_ad(1354, p.p219, A::offset(s.ad_value(1347), p.p219));
        }

        if ((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) {
            s.store_scalar(1353, p.p218);
        }

        if ((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) {
            s.store_div_ad_rhs(1355, 1353, A::add(s.ad_value(1353), s.ad_value(173)));
        }

        if ((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) {
            s.store_div_from_scalar_ad(1351, 1.0, A::offset(s.ad_value(1357), 1e-50));
        }

        if ((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) {
            s.store_mul_ad_lhs(1348, A::scale(s.ad_value(303), (-p.p214)), 1351);
        }

        s.v[1370] = if (s.v[1348] < (-34.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) && (s.v[1370] != 0.0)) {
            s.store_scalar(309, 0.0);
        }

        if (((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) && (!(s.v[1370] != 0.0))) {
            s.store_exp(1349, 1348);
        }

        if (((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) && (!(s.v[1370] != 0.0))) {
            s.store_mul_ad_lhs(1350, A::scale(A::div_from_scalar(p.p213, s.ad_value(302)), 1.6021918e-19), 1347);
        }

        if (((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) && (!(s.v[1370] != 0.0))) {
            s.store_div_from_scalar(1352, 1.0, 1365);
        }

        if (((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) && (!(s.v[1370] != 0.0))) {
            s.store_sqrt_ad(1353, A::mul(A::add(s.ad_value(1364), A::scale(s.ad_value(1360), 1e-12)), s.ad_value(1352)));
        }

        if (((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) && (!(s.v[1370] != 0.0))) {
            s.store_mul_ad_lhs(1351, A::mul(s.ad_value(1349), s.ad_value(1350)), 1353);
        }

        if (((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) && (!(s.v[1370] != 0.0))) {
            s.store_mul_ad_lhs(1356, A::mul(s.ad_value(1351), s.ad_value(1357)), 1357);
        }

        if (((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) && (!(s.v[1370] != 0.0))) {
            s.store_mul_ad_lhs(309, A::mul(s.ad_value(1354), s.ad_value(1355)), 1356);
        }

        if ((!(s.v[1366] != 0.0)) && (!(s.v[1367] != 0.0))) {
            s.store_scalar(309, 0.0);
        }

        if (!(s.v[1366] != 0.0)) {
            s.store_offset_scaled(1347, 158, (-p.p221), p.p222);
        }

        if (!(s.v[1366] != 0.0)) {
            s.store_exp_ad(1349, A::scale(s.ad_value(1347), s.v[1359]));
        }

        if (!(s.v[1366] != 0.0)) {
            s.store_scale_ad(1347, A::scale(s.ad_value(158), 1.0 / (s.v[1359])), 1.0 / (s.v[1359]));
        }

        if (!(s.v[1366] != 0.0)) {
            s.store_mul(1350, 158, 1347);
        }

        if (!(s.v[1366] != 0.0)) {
            s.store_scale(1351, 1362, (p.p220 / 1000000.0));
        }

        if (!(s.v[1366] != 0.0)) {
            s.store_mul_ad_lhs(306, A::mul(s.ad_value(1351), s.ad_value(1349)), 1350);
        }

        s.v[1371] = if (s.v[158] >= 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1366] != 0.0)) && (s.v[1371] != 0.0)) {
            s.store_scale(306, 306, (-1.0));
        }

        if (!(s.v[1366] != 0.0)) {
            s.store_sub(1348, 158, 157);
        }

        if (!(s.v[1366] != 0.0)) {
            s.store_offset_scaled(1347, 1348, (-p.p221), p.p222);
        }

        if (!(s.v[1366] != 0.0)) {
            s.store_exp_ad(1349, A::scale(s.ad_value(1347), s.v[1359]));
        }

        if (!(s.v[1366] != 0.0)) {
            s.store_scale_ad(1347, A::scale(s.ad_value(1348), 1.0 / (s.v[1359])), 1.0 / (s.v[1359]));
        }

        if (!(s.v[1366] != 0.0)) {
            s.store_mul(1350, 1348, 1347);
        }

        if (!(s.v[1366] != 0.0)) {
            s.store_scale(1351, 1362, (p.p220 / 1000000.0));
        }

        if (!(s.v[1366] != 0.0)) {
            s.store_mul_ad_lhs(307, A::mul(s.ad_value(1351), s.ad_value(1349)), 1350);
        }

        s.v[1372] = if (s.v[1348] >= 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1366] != 0.0)) && (s.v[1372] != 0.0)) {
            s.store_scale(307, 307, (-1.0));
        }

        if (!(s.v[1366] != 0.0)) {
            s.store_scale_ad(1357, A::offset(A::offset(A::sub(s.ad_value(513), s.ad_value(158)), s.v[123]), p.p225), 1.0 / (s.v[1359]));
        }

        if (!(s.v[1366] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1357)), ((4.0 * 0.01) * 0.01)));
        }

        if (!(s.v[1366] != 0.0)) {
            s.store_offset_ad(1357, A::scale(A::add(s.ad_value(1357), s.ad_value(44)), 0.5), (1e-10 * 0.01));
        }

        s.v[1373] = if (s.v[1357] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1366] != 0.0)) && (s.v[1373] != 0.0)) {
            s.store_scalar(1357, 0.0);
        }

        if (!(s.v[1366] != 0.0)) {
            s.store_offset(1357, 1357, 1e-50);
        }

        if (!(s.v[1366] != 0.0)) {
            s.store_div_from_scalar(1348, (-p.p224), 1357);
        }

        s.v[1374] = if (s.v[1348] < (-34.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1366] != 0.0)) && (s.v[1374] != 0.0)) {
            s.store_scalar(308, 0.0);
        }

        if ((!(s.v[1366] != 0.0)) && (!(s.v[1374] != 0.0))) {
            s.store_exp(1349, 1348);
        }

        if ((!(s.v[1366] != 0.0)) && (!(s.v[1374] != 0.0))) {
            s.store_scale(1350, 1362, (p.p223 * s.v[1361]));
        }

        if ((!(s.v[1366] != 0.0)) && (!(s.v[1374] != 0.0))) {
            s.store_mul_ad_lhs(308, A::mul(A::mul(s.ad_value(1350), s.ad_value(1357)), s.ad_value(1357)), 1349);
        }

        if (!(s.v[1366] != 0.0)) {
            s.store_scalar(310, 0.5);
        }

        s.v[1382] = if (p.p28 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1382] != 0.0) {
            s.store_scalar(311, 0.0);
        }

        if (!(s.v[1382] != 0.0)) {
            s.store_add_ad(1375, A::sub(A::scale(A::offset(s.ad_value(157), p.p210), p.p209), s.ad_value(158)), A::scale(A::add(s.ad_value(187), s.ad_value(319)), p.p211));
        }

        if (!(s.v[1382] != 0.0)) {
            s.store_scalar(1376, (1.0 / s.v[88]));
        }

        if (!(s.v[1382] != 0.0)) {
            s.store_mul(1377, 1375, 1376);
        }

        if (!(s.v[1382] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1377)), ((4.0 * 0.01) * 0.01)));
        }

        if (!(s.v[1382] != 0.0)) {
            s.store_offset_ad(304, A::scale(A::add(s.ad_value(1377), s.ad_value(44)), 0.5), (1e-10 * 0.01));
        }

        s.v[1383] = if (s.v[304] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1382] != 0.0)) && (s.v[1383] != 0.0)) {
            s.store_scalar(304, 0.0);
        }

        if (!(s.v[1382] != 0.0)) {
            s.store_div_from_scalar_ad(1378, 1.0, A::offset(s.ad_value(304), 1e-50));
        }

        if (!(s.v[1382] != 0.0)) {
            s.store_mul_ad_lhs(1379, A::scale(s.ad_value(303), (-p.p208)), 1378);
        }

        s.v[1384] = if (s.v[1379] < (-34.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1382] != 0.0)) && (s.v[1384] != 0.0)) {
            s.store_scalar(311, 0.0);
        }

        if ((!(s.v[1382] != 0.0)) && (!(s.v[1384] != 0.0))) {
            s.store_exp(1375, 1379);
        }

        if ((!(s.v[1382] != 0.0)) && (!(s.v[1384] != 0.0))) {
            s.store_mul_ad_lhs(1376, A::scale(A::div_from_scalar(p.p207, s.ad_value(302)), 1.6021918e-19), 107);
        }

        if ((!(s.v[1382] != 0.0)) && (!(s.v[1384] != 0.0))) {
            s.store_mul_ad_lhs(311, A::mul(A::mul(s.ad_value(1376), s.ad_value(304)), s.ad_value(304)), 1375);
        }

        if (!(s.v[1382] != 0.0)) {
            s.store_sub(1381, 157, 513);
        }

        s.v[1385] = if (s.v[1381] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1382] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_square(1376, 1381);
        }

        if ((!(s.v[1382] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_mul(331, 1376, 1381);
        }

        if ((!(s.v[1382] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_offset(1379, 331, p.p212);
        }

        if ((!(s.v[1382] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_div(1380, 331, 1379);
        }

        if ((!(s.v[1382] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_mul(311, 311, 1380);
        }

        if ((!(s.v[1382] != 0.0)) && (!(s.v[1385] != 0.0))) {
            s.store_scalar(311, 0.0);
        }

        s.v[1393] = if (p.p28 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1393] != 0.0) {
            s.store_scalar(312, 0.0);
        }

        if (!(s.v[1393] != 0.0)) {
            s.store_add_ad(1386, A::sub(A::scale(A::sub_from_scalar(p.p210, s.ad_value(157)), p.p209), A::sub(s.ad_value(158), s.ad_value(157))), A::scale(A::add(s.ad_value(187), s.ad_value(319)), p.p211));
        }

        if (!(s.v[1393] != 0.0)) {
            s.store_scalar(1387, (1.0 / s.v[88]));
        }

        if (!(s.v[1393] != 0.0)) {
            s.store_mul(1388, 1386, 1387);
        }

        if (!(s.v[1393] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1388)), ((4.0 * 0.01) * 0.01)));
        }

        if (!(s.v[1393] != 0.0)) {
            s.store_offset_ad(305, A::scale(A::add(s.ad_value(1388), s.ad_value(44)), 0.5), (1e-10 * 0.01));
        }

        s.v[1394] = if (s.v[305] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1393] != 0.0)) && (s.v[1394] != 0.0)) {
            s.store_scalar(305, 0.0);
        }

        if (!(s.v[1393] != 0.0)) {
            s.store_div_from_scalar_ad(1389, 1.0, A::offset(s.ad_value(305), 1e-50));
        }

        if (!(s.v[1393] != 0.0)) {
            s.store_mul_ad_lhs(1390, A::scale(s.ad_value(303), (-p.p208)), 1389);
        }

        s.v[1395] = if (s.v[1390] < (-34.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1393] != 0.0)) && (s.v[1395] != 0.0)) {
            s.store_scalar(312, 0.0);
        }

        if ((!(s.v[1393] != 0.0)) && (!(s.v[1395] != 0.0))) {
            s.store_exp(1386, 1390);
        }

        if ((!(s.v[1393] != 0.0)) && (!(s.v[1395] != 0.0))) {
            s.store_div_from_scalar(1389, 1.0, 302);
        }

        if ((!(s.v[1393] != 0.0)) && (!(s.v[1395] != 0.0))) {
            s.store_mul_ad_lhs(1387, A::scale(s.ad_value(1389), (p.p207 * 1.6021918e-19)), 107);
        }

        if ((!(s.v[1393] != 0.0)) && (!(s.v[1395] != 0.0))) {
            s.store_mul_ad_lhs(312, A::mul(A::mul(s.ad_value(1387), s.ad_value(305)), s.ad_value(305)), 1386);
        }

        if (!(s.v[1393] != 0.0)) {
            s.store_neg(1392, 513);
        }

        s.v[1396] = if (s.v[1392] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1393] != 0.0)) && (s.v[1396] != 0.0)) {
            s.store_square(1387, 1392);
        }

        if ((!(s.v[1393] != 0.0)) && (s.v[1396] != 0.0)) {
            s.store_mul(331, 1387, 1392);
        }

        if ((!(s.v[1393] != 0.0)) && (s.v[1396] != 0.0)) {
            s.store_offset(1390, 331, p.p212);
        }

        if ((!(s.v[1393] != 0.0)) && (s.v[1396] != 0.0)) {
            s.store_div(1391, 331, 1390);
        }

        if ((!(s.v[1393] != 0.0)) && (s.v[1396] != 0.0)) {
            s.store_mul(312, 312, 1391);
        }

        if ((!(s.v[1393] != 0.0)) && (!(s.v[1396] != 0.0))) {
            s.store_scalar(312, 0.0);
        }

        s.v[1397] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1397] != 0.0) {
            s.store_scalar(1407, s.v[91]);
        }

        if (s.v[1397] != 0.0) {
            s.store_div_from_scalar(1408, 1.0, 1407);
        }

        if (s.v[1397] != 0.0) {
            s.store_scalar(1464, 0.0);
        }

        if (s.v[1397] != 0.0) {
            s.store_scalar(1466, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_22(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1397] != 0.0) {
            s.store_scalar(1468, 0.0);
        }

        if (s.v[1397] != 0.0) {
            s.store_neg(1400, 534);
        }

        if (s.v[1397] != 0.0) {
            s.store_mul(1401, 1400, 436);
        }

        if (s.v[1397] != 0.0) {
            s.store_add_ad_rhs(331, 1401, A::mul(s.ad_value(1400), s.ad_value(437)));
        }

        if (s.v[1397] != 0.0) {
            s.store_mul(470, 1401, 438);
        }

        if (s.v[1397] != 0.0) {
            s.store_sub(469, 1401, 470);
        }

        if (s.v[1397] != 0.0) {
            s.store_mul(468, 331, 438);
        }

        if (s.v[1397] != 0.0) {
            s.store_sub(467, 331, 468);
        }

        if ((s.v[1397] != 0.0) && (p.p24 != 0.0)) {
            s.copy_ad(521, 536);
        }

        if ((s.v[1397] != 0.0) && (p.p24 != 0.0)) {
            s.store_scalar(528, 0.0);
        }

        s.v[1477] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1478] = if (1.0 == 2.0) { 1.0 } else { 0.0 };

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_scale(522, 533, 0.5);
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_scalar(523, p.p292);
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1477] != 0.0)) {
            s.store_scalar(528, s.v[525]);
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && ((s.v[1478] != 0.0) && (!(s.v[1477] != 0.0)))) {
            s.store_scale(522, 534, 0.5);
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && ((s.v[1478] != 0.0) && (!(s.v[1477] != 0.0)))) {
            s.store_scalar(523, p.p68);
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && ((s.v[1478] != 0.0) && (!(s.v[1477] != 0.0)))) {
            s.store_scalar(528, s.v[524]);
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && ((s.v[1478] != 0.0) && (!(s.v[1477] != 0.0)))) {
            s.store_scalar(528, 1.0);
        }

        s.v[1479] = if (s.v[528] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_mul_ad_rhs(1427, 238, A::sqrt(A::div(s.ad_value(521), s.ad_value(536))));
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_scalar(1409, ((1.0 - -1.0) / 2.0));
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_scalar(1410, ((1.0 + -1.0) / 2.0));
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_add_ad(1420, A::mul(s.ad_value(461), s.ad_value(156)), A::mul(s.ad_value(462), A::sub(s.ad_value(156), s.ad_value(157))));
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_add_ad(1421, A::mul(s.ad_value(461), s.ad_value(157)), A::mul(s.ad_value(462), A::neg(s.ad_value(157))));
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_add_ad(1422, A::mul(s.ad_value(461), s.ad_value(158)), A::mul(s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_add_ad(1423, A::mul(s.ad_value(462), s.ad_value(158)), A::mul(s.ad_value(461), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_sub(1424, 1421, 1420);
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_neg(1425, 1420);
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_add_ad(1411, A::mul(s.ad_value(1409), s.ad_value(461)), A::mul(s.ad_value(1410), s.ad_value(462)));
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_add_ad(1412, A::mul(s.ad_value(1409), s.ad_value(462)), A::mul(s.ad_value(1410), s.ad_value(461)));
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_add_ad(1426, A::mul(s.ad_value(1411), s.ad_value(1422)), A::mul(s.ad_value(1412), s.ad_value(1423)));
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_offset_ad(1418, A::add(A::mul(s.ad_value(1411), s.ad_value(1425)), A::mul(s.ad_value(1412), s.ad_value(1424))), (10.0 * 2.220446049250313e-16));
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_neg(1398, 1418);
        }

        s.v[1480] = if (s.v[1398] > s.v[141]) { 1.0 } else { 0.0 };

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1480] != 0.0)) {
            s.store_sub(1399, 1398, 141);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1480] != 0.0)) {
            s.store_sub(1400, 140, 141);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1480] != 0.0)) {
            s.store_div(44, 1399, 1400);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1480] != 0.0)) {
            s.store_square(45, 44);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1480] != 0.0)) {
            s.store_mul(46, 45, 44);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1480] != 0.0)) {
            s.store_square(47, 45);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1480] != 0.0)) {
            s.store_div_from_scalar_ad(1406, 1.0, A::add(A::add(A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(45)), s.ad_value(46)), s.ad_value(47)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1480] != 0.0)) {
            s.store_mul_ad_rhs(1406, 1400, A::sub_from_scalar(1.0, s.ad_value(1406)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1480] != 0.0)) {
            s.store_add(1403, 141, 1406);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1480] != 0.0))) {
            s.copy_ad(1403, 1398);
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_offset_ad(1419, A::neg(s.ad_value(1403)), (-1e-12));
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_mul(1428, 1427, 1408);
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_square(1429, 1428);
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_sub(1430, 1426, 523);
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_div(1398, 521, 230);
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_mul_ad(1431, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(s.ad_value(1398)));
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_neg(1432, 1419);
        }

        s.v[1481] = if (s.v[1430] < s.v[1432]) { 1.0 } else { 0.0 };

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1481] != 0.0)) {
            s.store_div_from_scalar_ad(1399, 1.0, A::mul(s.ad_value(225), s.ad_value(1427)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1481] != 0.0)) {
            s.store_mul(1406, 1399, 1407);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1481] != 0.0)) {
            s.store_offset_scaled(1433, 1406, (3.0 * 1.414213562373095), 2.0);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1481] != 0.0)) {
            s.store_mul_ad_lhs(1434, A::mul(A::scale(s.ad_value(1433), 8.0), s.ad_value(1433)), 1433);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1481] != 0.0)) {
            s.store_sub(1435, 237, 1431);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1481] != 0.0)) {
            s.store_mul_ad_rhs(1405, 225, A::add(s.ad_value(1430), s.ad_value(1419)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1481] != 0.0)) {
            s.store_sub_from_scalar_ad(1436, (7.0 * 1.414213562373095), A::mul(A::scale(s.ad_value(1406), 9.0), A::offset(s.ad_value(1405), (-2.0))));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1481] != 0.0)) {
            s.store_square(1437, 1436);
        }

        s.v[1482] = if (s.v[1434] < (s.v[1437] * 1e-8)) { 1.0 } else { 0.0 };

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1481] != 0.0)) && (s.v[1482] != 0.0)) {
            s.store_add_ad(1439, A::add(A::offset(s.ad_value(1436), ((-7.0) * 1.414213562373095)), A::div(A::scale(s.ad_value(1434), 0.5), s.ad_value(1436))), A::mul(A::scale(s.ad_value(1406), 9.0), A::offset(s.ad_value(1405), (-2.0))));
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1481] != 0.0)) && (!(s.v[1482] != 0.0))) {
            s.store_sqrt_ad(1438, A::add(s.ad_value(1434), s.ad_value(1437)));
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1481] != 0.0)) && (!(s.v[1482] != 0.0))) {
            s.store_add_ad(1439, A::offset(s.ad_value(1438), ((-7.0) * 1.414213562373095)), A::mul(A::scale(s.ad_value(1406), 9.0), A::offset(s.ad_value(1405), (-2.0))));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1481] != 0.0)) {
            s.store_powf(1440, 1439, 0.3333333333333333);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1481] != 0.0)) {
            s.store_add_ad(1441, A::add(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1406), 12.0)), A::scale(s.ad_value(1440), 2.0)), A::mul(A::scale(s.ad_value(1440), 1.414213562373095), s.ad_value(1440)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1481] != 0.0)) {
            s.store_div(1442, 1441, 1440);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1481] != 0.0)) {
            s.store_sub_ad_lhs(1443, A::mul(s.ad_value(1442), s.ad_value(227)), 1419);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1481] != 0.0)) {
            s.store_add(1399, 1443, 1419);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1481] != 0.0)) {
            s.store_div(1400, 1399, 1435);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1481] != 0.0)) {
            s.store_sqrt_ad(1401, A::offset(A::square(s.ad_value(1400)), 1.0));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1481] != 0.0)) {
            s.store_sub_ad_lhs(1444, A::div(s.ad_value(1399), s.ad_value(1401)), 1419);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1481] != 0.0)) {
            s.store_sub(1400, 1430, 1444);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1481] != 0.0)) {
            s.store_mul(459, 1407, 1400);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1481] != 0.0)) {
            s.copy_ad(458, 459);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_scalar(1442, 3.0);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_sub_ad_lhs(1445, A::div(s.ad_value(1442), s.ad_value(225)), 1419);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_exp_ad(1406, A::neg(s.ad_value(1442)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_offset_ad(1405, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1430), s.ad_value(1419))), (-1.0)), s.ad_value(1406)), 4.0), A::mul(s.ad_value(1429), s.ad_value(226))), 1.0);
        }

        s.v[1483] = if (s.v[1405] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1483] != 0.0)) {
            s.store_scalar(1405, (10.0 * 2.220446049250313e-16));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_add_ad_rhs(1445, 1430, A::mul(A::scale(A::mul(s.ad_value(1429), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1405)))));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_mul_ad_rhs(1442, 225, A::add(s.ad_value(1445), s.ad_value(1419)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_exp_ad(1406, A::neg(s.ad_value(1442)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_offset_ad(1405, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1430), s.ad_value(1419))), (-1.0)), s.ad_value(1406)), 4.0), A::mul(s.ad_value(1429), s.ad_value(226))), 1.0);
        }

        s.v[1484] = if (s.v[1405] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1484] != 0.0)) {
            s.store_scalar(1405, (10.0 * 2.220446049250313e-16));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_add_ad_rhs(1445, 1430, A::mul(A::scale(A::mul(s.ad_value(1429), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1405)))));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_mul_ad_rhs(1442, 225, A::add(s.ad_value(1445), s.ad_value(1419)));
        }

        s.v[1485] = if (s.v[1442] < 3.0) { 1.0 } else { 0.0 };

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1485] != 0.0)) {
            s.store_scalar(1446, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1485] != 0.0)) {
            s.store_scalar(1447, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1485] != 0.0)) {
            s.store_offset_ad(1448, A::div_from_scalar(1.0, A::mul(s.ad_value(225), s.ad_value(1428))), (1.0 / 1.414213562373095));
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1485] != 0.0)) {
            s.store_div_ad_lhs(1449, A::neg(A::add(s.ad_value(1430), s.ad_value(1419))), 1428);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1485] != 0.0)) {
            s.store_add_ad(1450, A::sub(A::div(A::mul(A::square(s.ad_value(1447)), s.ad_value(1447)), A::mul(A::mul(A::scale(s.ad_value(1446), 27.0), s.ad_value(1446)), s.ad_value(1446))), A::div(A::mul(s.ad_value(1447), s.ad_value(1448)), A::mul(A::scale(s.ad_value(1446), 6.0), s.ad_value(1446)))), A::div(s.ad_value(1449), A::scale(s.ad_value(1446), 2.0)));
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1485] != 0.0)) {
            s.store_div_ad(1451, A::sub(A::mul(A::scale(s.ad_value(1446), 3.0), s.ad_value(1448)), A::square(s.ad_value(1447))), A::mul(A::scale(s.ad_value(1446), 9.0), s.ad_value(1446)));
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1485] != 0.0)) {
            s.store_sqrt_ad(1402, A::add(A::square(s.ad_value(1450)), A::mul(A::square(s.ad_value(1451)), s.ad_value(1451))));
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1485] != 0.0)) {
            s.store_powf_ad(1452, A::sub(s.ad_value(1402), s.ad_value(1450)), 0.3333333333333333);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1485] != 0.0)) {
            s.store_neg_ad(1453, A::powf(A::add(s.ad_value(1450), s.ad_value(1402)), 0.3333333333333333));
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1485] != 0.0)) {
            s.store_sub_ad(1405, A::add(s.ad_value(1452), s.ad_value(1453)), A::div(s.ad_value(1447), A::scale(s.ad_value(1446), 3.0)));
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1485] != 0.0)) {
            s.store_sub_ad_lhs(1445, A::mul(s.ad_value(1405), s.ad_value(227)), 1419);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1485] != 0.0)) {
            s.store_mul_ad_rhs(1442, 225, A::add(s.ad_value(1445), s.ad_value(1419)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_offset_ad(1454, A::add(s.ad_value(1430), s.ad_value(1419)), 0.1);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_offset_ad(1461, A::exp(A::mul(s.ad_value(225), A::neg(s.ad_value(1419)))), 1e-50);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_div(1398, 230, 521);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_square(1455, 1398);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_mul(1456, 1455, 1461);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_mul(1398, 226, 1429);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_mul(1457, 225, 1454);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_add_ad(1458, A::sub(A::ln(A::add(A::mul(s.ad_value(1456), s.ad_value(1398)), A::square(s.ad_value(1457)))), A::ln(A::mul(s.ad_value(1455), s.ad_value(1398)))), A::mul(s.ad_value(225), s.ad_value(1419)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_offset_ad(44, A::sub(s.ad_value(1457), s.ad_value(1458)), (-1.0));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_scale(45, 1457, 4.0);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_scale_ad(1399, A::offset(A::div(s.ad_value(44), s.ad_value(45)), 1.0), 0.5);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_scale_ad(1400, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(44), 2.0), s.ad_value(45))), 0.5);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_sub_ad_rhs(1458, 1457, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_sub(1457, 1457, 1458);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_add_ad_rhs(1457, 1457, A::scale(s.ad_value(225), 0.1));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_add_ad(1459, A::sub(A::ln(A::add(A::mul(s.ad_value(1456), s.ad_value(1398)), A::square(s.ad_value(1457)))), A::ln(A::mul(s.ad_value(1455), s.ad_value(1398)))), A::mul(s.ad_value(225), s.ad_value(1419)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.copy_ad(1460, 1442);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_offset_ad(44, A::sub(s.ad_value(1459), s.ad_value(1460)), (-(0.0008 * 75.0)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_scale(45, 1459, (4.0 * (0.0008 * 75.0)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_scale_ad(1399, A::offset(A::div(s.ad_value(44), s.ad_value(45)), 1.0), 0.5);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_scale_ad(1400, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(44), ((2.0 * 0.0008) * 75.0)), s.ad_value(45))), 0.5);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_sub_ad_rhs(1442, 1459, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

    }

    pub(super) fn stamp_transient_block_23(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_sub_ad_lhs(1444, A::div(s.ad_value(1442), s.ad_value(225)), 1419);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_add_ad(1399, A::offset(s.ad_value(1442), (-1.0)), A::exp(A::neg(s.ad_value(1442))));
        }

        s.v[1486] = if (s.v[1399] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1486] != 0.0)) {
            s.store_scalar(1399, (10.0 * 2.220446049250313e-16));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_sqrt(1400, 1399);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_mul(458, 1427, 1400);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_mul_ad_rhs(459, 1407, A::sub(s.ad_value(1430), s.ad_value(1444)));
        }

        s.v[1487] = if (p.p42 == 1.0) { 1.0 } else { 0.0 };

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) {
            s.store_exp_ad(1461, A::mul(s.ad_value(225), A::neg(s.ad_value(1419))));
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) {
            s.store_div(1398, 230, 521);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) {
            s.store_square(1455, 1398);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) {
            s.store_mul(1470, 1455, 1461);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) {
            s.store_scalar(1415, 0.0);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) {
            s.store_scalar(167, 1.0);
        }

        let mut assign26240_loop_guard: usize = 0;
        while {
            let assign26240_cond_e35811: f64 = (2.0 * 20.0);
            let assign26240_cond_e35813: f64 = (assign26240_cond_e35811 + 1.0);
            let assign26240_cond_e35815: f64 = if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) && (s.v[167] <= assign26240_cond_e35813)) { 1.0 } else { 0.0 };
            assign26240_cond_e35815 != 0.0
        } {
            assign26240_loop_guard += 1;
            assert!(assign26240_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) {
                s.store_scalar(1466, 0.0);
            }
            if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) {
                s.store_mul_ad_rhs(1442, 225, A::add(s.ad_value(1444), s.ad_value(1419)));
            }
            s.v[1488] = if (s.v[1442] < 5.0) { 1.0 } else { 0.0 };
            if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) && (s.v[1488] != 0.0)) {
                s.store_mul_ad(1462, A::mul(A::square(s.ad_value(1442)), s.ad_value(1442)), A::offset(A::mul(s.ad_value(1442), A::offset(A::scale(s.ad_value(1442), 0.006115288895133179), (-0.07053654284009761))), 0.29693154855771));
            }
            if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) && (s.v[1488] != 0.0)) {
                s.store_mul_ad(1463, A::square(s.ad_value(1442)), A::offset(A::mul(s.ad_value(1442), A::offset(A::scale(s.ad_value(1442), (5.0 * 0.006115288895133179)), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
            }
            if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) && (s.v[1488] != 0.0)) {
                s.store_mul_ad_lhs(1464, A::mul(s.ad_value(1470), s.ad_value(1462)), 1462);
            }
            if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) && (s.v[1488] != 0.0)) {
                s.store_mul_ad_lhs(1465, A::mul(A::scale(A::mul(s.ad_value(1470), s.ad_value(225)), 2.0), s.ad_value(1462)), 1463);
            }
            if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) && (s.v[1488] != 0.0)) {
                s.store_mul_ad_rhs(1466, 1442, A::offset(A::mul(s.ad_value(1442), A::offset(A::mul(s.ad_value(1442), A::offset(A::mul(s.ad_value(1442), A::offset(A::scale(s.ad_value(1442), 6.36964918866352e-5), (-0.00163730162779191))), 0.0178800506338833)), (-0.117851130197758))), 0.707106781186548));
            }
            if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) && (s.v[1488] != 0.0)) {
                s.store_offset_ad(1467, A::mul(s.ad_value(1442), A::offset(A::mul(s.ad_value(1442), A::offset(A::mul(s.ad_value(1442), A::offset(A::scale(s.ad_value(1442), (5.0 * 6.36964918866352e-5)), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833))), (2.0 * (-0.117851130197758)))), 0.707106781186548);
            }
            if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) && (s.v[1488] != 0.0)) {
                s.store_sqrt_ad(1468, A::offset(A::add(A::square(s.ad_value(1466)), s.ad_value(1464)), 1e-50));
            }
            if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) && (s.v[1488] != 0.0)) {
                s.store_div_ad(1469, A::add(A::mul(A::scale(A::mul(s.ad_value(225), s.ad_value(1467)), 2.0), s.ad_value(1466)), s.ad_value(1465)), A::scale(s.ad_value(1468), 2.0));
            }
            s.v[1489] = if (s.v[1442] < 80.0) { 1.0 } else { 0.0 };
            if (((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) && (!(s.v[1488] != 0.0))) && (s.v[1489] != 0.0)) {
                s.store_exp(243, 1442);
            }
            if (((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) && (!(s.v[1488] != 0.0))) && (s.v[1489] != 0.0)) {
                s.store_mul_ad_rhs(1464, 1470, A::offset(s.ad_value(243), (-1.0)));
            }
            if (((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) && (!(s.v[1488] != 0.0))) && (s.v[1489] != 0.0)) {
                s.store_mul_ad_lhs(1465, A::mul(s.ad_value(1470), s.ad_value(225)), 243);
            }
            if (((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) && (!(s.v[1488] != 0.0))) && (!(s.v[1489] != 0.0))) {
                s.store_exp_ad(1471, A::mul(s.ad_value(225), s.ad_value(1444)));
            }
            if (((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) && (!(s.v[1488] != 0.0))) && (!(s.v[1489] != 0.0))) {
                s.store_mul_ad_rhs(1464, 1455, A::sub(s.ad_value(1471), s.ad_value(1461)));
            }
            if (((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) && (!(s.v[1488] != 0.0))) && (!(s.v[1489] != 0.0))) {
                s.store_mul_ad_lhs(1465, A::mul(s.ad_value(1455), s.ad_value(225)), 1471);
            }
            if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) && (!(s.v[1488] != 0.0))) {
                s.store_sqrt_ad(1468, A::add(A::offset(s.ad_value(1442), (-1.0)), s.ad_value(1464)));
            }
            if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) && (!(s.v[1488] != 0.0))) {
                s.store_scale_ad(1469, A::div(A::add(s.ad_value(225), s.ad_value(1465)), s.ad_value(1468)), 0.5);
            }
            if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) {
                s.store_sub_ad(1472, A::sub(s.ad_value(1430), s.ad_value(1444)), A::mul(s.ad_value(1428), s.ad_value(1468)));
            }
            if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) {
                s.store_sub_from_scalar_ad(1473, (-1.0), A::mul(s.ad_value(1428), s.ad_value(1469)));
            }
            s.v[1490] = if (s.v[1415] == 1.0) { 1.0 } else { 0.0 };
            if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) && (s.v[1490] != 0.0)) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) && (!(s.v[1490] != 0.0))) {
                s.store_div_ad_lhs(494, A::neg(s.ad_value(1472)), 1473);
            }
            if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) && (!(s.v[1490] != 0.0))) {
                s.store_scale_ad(1474, A::offset({
                    if (1.0 >= ((s.v[1444]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1444))
                    }
                }, 1.0), (0.5 * 0.1));
            }
            s.v[1491] = if (((s.v[494]) as f64).abs() > s.v[1474]) { 1.0 } else { 0.0 };
            if (((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) && (!(s.v[1490] != 0.0))) && (s.v[1491] != 0.0)) {
                s.store_scale(494, 1474, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) && (!(s.v[1490] != 0.0))) {
                s.store_add(1444, 1444, 494);
            }
            s.v[1492] = if ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1472]) as f64).abs() <= 1e-8)) { 1.0 } else { 0.0 };
            if (((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) && (!(s.v[1490] != 0.0))) && (s.v[1492] != 0.0)) {
                s.store_scalar(1415, 1.0);
            }
            if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.v[1494] = if (s.v[1442] < 5.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) && (s.v[1494] != 0.0)) {
            s.store_offset_ad(1475, A::square(s.ad_value(1466)), (10.0 * 2.220446049250313e-16));
        }

        if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) && (s.v[1494] != 0.0)) {
            s.store_offset(1476, 1466, (10.0 * 2.220446049250313e-16));
        }

        if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) && (!(s.v[1494] != 0.0))) {
            s.store_offset(1475, 1442, (-1.0));
        }

        if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) && (!(s.v[1494] != 0.0))) {
            s.store_sqrt(1476, 1475);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) {
            s.store_mul(458, 1427, 1476);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) {
            s.store_div_from_scalar_ad(1399, 1.0, A::add(s.ad_value(1468), s.ad_value(1476)));
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) {
            s.store_mul_ad_lhs(460, A::mul(s.ad_value(1427), s.ad_value(1464)), 1399);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1481] != 0.0))) && (s.v[1487] != 0.0)) {
            s.store_add(459, 458, 460);
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_sub(460, 459, 458);
        }

        s.v[1496] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1497] = if (1.0 == 2.0) { 1.0 } else { 0.0 };

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1496] != 0.0)) && (s.v[1409] != 0.0)) {
            s.store_mul_ad_lhs(463, A::neg(s.ad_value(522)), 459);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1496] != 0.0)) && (s.v[1409] != 0.0)) {
            s.store_mul_ad_lhs(465, A::neg(s.ad_value(522)), 460);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1496] != 0.0)) && (s.v[1410] != 0.0)) {
            s.store_mul_ad_lhs(464, A::neg(s.ad_value(522)), 459);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1496] != 0.0)) && (s.v[1410] != 0.0)) {
            s.store_mul_ad_lhs(466, A::neg(s.ad_value(522)), 460);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && ((s.v[1497] != 0.0) && (!(s.v[1496] != 0.0)))) && (s.v[1409] != 0.0)) {
            s.store_mul_ad_lhs(467, A::neg(s.ad_value(522)), 459);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && ((s.v[1497] != 0.0) && (!(s.v[1496] != 0.0)))) && (s.v[1409] != 0.0)) {
            s.store_mul_ad_lhs(469, A::neg(s.ad_value(522)), 460);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && ((s.v[1497] != 0.0) && (!(s.v[1496] != 0.0)))) && (s.v[1410] != 0.0)) {
            s.store_mul_ad_lhs(468, A::neg(s.ad_value(522)), 459);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && ((s.v[1497] != 0.0) && (!(s.v[1496] != 0.0)))) && (s.v[1410] != 0.0)) {
            s.store_mul_ad_lhs(470, A::neg(s.ad_value(522)), 460);
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_scalar(1409, ((1.0 - 1.0) / 2.0));
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_scalar(1410, ((1.0 + 1.0) / 2.0));
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_add_ad(1420, A::mul(s.ad_value(461), s.ad_value(156)), A::mul(s.ad_value(462), A::sub(s.ad_value(156), s.ad_value(157))));
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_add_ad(1421, A::mul(s.ad_value(461), s.ad_value(157)), A::mul(s.ad_value(462), A::neg(s.ad_value(157))));
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_add_ad(1422, A::mul(s.ad_value(461), s.ad_value(158)), A::mul(s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_add_ad(1423, A::mul(s.ad_value(462), s.ad_value(158)), A::mul(s.ad_value(461), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_sub(1424, 1421, 1420);
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_neg(1425, 1420);
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_add_ad(1411, A::mul(s.ad_value(1409), s.ad_value(461)), A::mul(s.ad_value(1410), s.ad_value(462)));
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_add_ad(1412, A::mul(s.ad_value(1409), s.ad_value(462)), A::mul(s.ad_value(1410), s.ad_value(461)));
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_add_ad(1426, A::mul(s.ad_value(1411), s.ad_value(1422)), A::mul(s.ad_value(1412), s.ad_value(1423)));
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_offset_ad(1418, A::add(A::mul(s.ad_value(1411), s.ad_value(1425)), A::mul(s.ad_value(1412), s.ad_value(1424))), (10.0 * 2.220446049250313e-16));
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_neg(1398, 1418);
        }

        s.v[1498] = if (s.v[1398] > s.v[141]) { 1.0 } else { 0.0 };

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1498] != 0.0)) {
            s.store_sub(1399, 1398, 141);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1498] != 0.0)) {
            s.store_sub(1400, 140, 141);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1498] != 0.0)) {
            s.store_div(44, 1399, 1400);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1498] != 0.0)) {
            s.store_square(45, 44);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1498] != 0.0)) {
            s.store_mul(46, 45, 44);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1498] != 0.0)) {
            s.store_square(47, 45);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1498] != 0.0)) {
            s.store_div_from_scalar_ad(1406, 1.0, A::add(A::add(A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(45)), s.ad_value(46)), s.ad_value(47)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1498] != 0.0)) {
            s.store_mul_ad_rhs(1406, 1400, A::sub_from_scalar(1.0, s.ad_value(1406)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1498] != 0.0)) {
            s.store_add(1403, 141, 1406);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1498] != 0.0))) {
            s.copy_ad(1403, 1398);
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_offset_ad(1419, A::neg(s.ad_value(1403)), (-1e-12));
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_mul(1428, 1427, 1408);
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_square(1429, 1428);
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_sub(1430, 1426, 523);
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_div(1398, 521, 230);
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_mul_ad(1431, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(s.ad_value(1398)));
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_neg(1432, 1419);
        }

        s.v[1499] = if (s.v[1430] < s.v[1432]) { 1.0 } else { 0.0 };

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1499] != 0.0)) {
            s.store_div_from_scalar_ad(1399, 1.0, A::mul(s.ad_value(225), s.ad_value(1427)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1499] != 0.0)) {
            s.store_mul(1406, 1399, 1407);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1499] != 0.0)) {
            s.store_offset_scaled(1433, 1406, (3.0 * 1.414213562373095), 2.0);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1499] != 0.0)) {
            s.store_mul_ad_lhs(1434, A::mul(A::scale(s.ad_value(1433), 8.0), s.ad_value(1433)), 1433);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1499] != 0.0)) {
            s.store_sub(1435, 237, 1431);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1499] != 0.0)) {
            s.store_mul_ad_rhs(1405, 225, A::add(s.ad_value(1430), s.ad_value(1419)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1499] != 0.0)) {
            s.store_sub_from_scalar_ad(1436, (7.0 * 1.414213562373095), A::mul(A::scale(s.ad_value(1406), 9.0), A::offset(s.ad_value(1405), (-2.0))));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1499] != 0.0)) {
            s.store_square(1437, 1436);
        }

        s.v[1500] = if (s.v[1434] < (s.v[1437] * 1e-8)) { 1.0 } else { 0.0 };

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1499] != 0.0)) && (s.v[1500] != 0.0)) {
            s.store_add_ad(1439, A::add(A::offset(s.ad_value(1436), ((-7.0) * 1.414213562373095)), A::div(A::scale(s.ad_value(1434), 0.5), s.ad_value(1436))), A::mul(A::scale(s.ad_value(1406), 9.0), A::offset(s.ad_value(1405), (-2.0))));
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1499] != 0.0)) && (!(s.v[1500] != 0.0))) {
            s.store_sqrt_ad(1438, A::add(s.ad_value(1434), s.ad_value(1437)));
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1499] != 0.0)) && (!(s.v[1500] != 0.0))) {
            s.store_add_ad(1439, A::offset(s.ad_value(1438), ((-7.0) * 1.414213562373095)), A::mul(A::scale(s.ad_value(1406), 9.0), A::offset(s.ad_value(1405), (-2.0))));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1499] != 0.0)) {
            s.store_powf(1440, 1439, 0.3333333333333333);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1499] != 0.0)) {
            s.store_add_ad(1441, A::add(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1406), 12.0)), A::scale(s.ad_value(1440), 2.0)), A::mul(A::scale(s.ad_value(1440), 1.414213562373095), s.ad_value(1440)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1499] != 0.0)) {
            s.store_div(1442, 1441, 1440);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1499] != 0.0)) {
            s.store_sub_ad_lhs(1443, A::mul(s.ad_value(1442), s.ad_value(227)), 1419);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1499] != 0.0)) {
            s.store_add(1399, 1443, 1419);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1499] != 0.0)) {
            s.store_div(1400, 1399, 1435);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1499] != 0.0)) {
            s.store_sqrt_ad(1401, A::offset(A::square(s.ad_value(1400)), 1.0));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1499] != 0.0)) {
            s.store_sub_ad_lhs(1444, A::div(s.ad_value(1399), s.ad_value(1401)), 1419);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1499] != 0.0)) {
            s.store_sub(1400, 1430, 1444);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1499] != 0.0)) {
            s.store_mul(459, 1407, 1400);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1499] != 0.0)) {
            s.copy_ad(458, 459);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_scalar(1442, 3.0);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_sub_ad_lhs(1445, A::div(s.ad_value(1442), s.ad_value(225)), 1419);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_exp_ad(1406, A::neg(s.ad_value(1442)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_offset_ad(1405, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1430), s.ad_value(1419))), (-1.0)), s.ad_value(1406)), 4.0), A::mul(s.ad_value(1429), s.ad_value(226))), 1.0);
        }

        s.v[1501] = if (s.v[1405] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1501] != 0.0)) {
            s.store_scalar(1405, (10.0 * 2.220446049250313e-16));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_add_ad_rhs(1445, 1430, A::mul(A::scale(A::mul(s.ad_value(1429), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1405)))));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_mul_ad_rhs(1442, 225, A::add(s.ad_value(1445), s.ad_value(1419)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_exp_ad(1406, A::neg(s.ad_value(1442)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_offset_ad(1405, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1430), s.ad_value(1419))), (-1.0)), s.ad_value(1406)), 4.0), A::mul(s.ad_value(1429), s.ad_value(226))), 1.0);
        }

        s.v[1502] = if (s.v[1405] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1502] != 0.0)) {
            s.store_scalar(1405, (10.0 * 2.220446049250313e-16));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_add_ad_rhs(1445, 1430, A::mul(A::scale(A::mul(s.ad_value(1429), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1405)))));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_mul_ad_rhs(1442, 225, A::add(s.ad_value(1445), s.ad_value(1419)));
        }

        s.v[1503] = if (s.v[1442] < 3.0) { 1.0 } else { 0.0 };

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1503] != 0.0)) {
            s.store_scalar(1446, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1503] != 0.0)) {
            s.store_scalar(1447, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1503] != 0.0)) {
            s.store_offset_ad(1448, A::div_from_scalar(1.0, A::mul(s.ad_value(225), s.ad_value(1428))), (1.0 / 1.414213562373095));
        }

    }

    pub(super) fn stamp_transient_block_24(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1503] != 0.0)) {
            s.store_div_ad_lhs(1449, A::neg(A::add(s.ad_value(1430), s.ad_value(1419))), 1428);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1503] != 0.0)) {
            s.store_add_ad(1450, A::sub(A::div(A::mul(A::square(s.ad_value(1447)), s.ad_value(1447)), A::mul(A::mul(A::scale(s.ad_value(1446), 27.0), s.ad_value(1446)), s.ad_value(1446))), A::div(A::mul(s.ad_value(1447), s.ad_value(1448)), A::mul(A::scale(s.ad_value(1446), 6.0), s.ad_value(1446)))), A::div(s.ad_value(1449), A::scale(s.ad_value(1446), 2.0)));
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1503] != 0.0)) {
            s.store_div_ad(1451, A::sub(A::mul(A::scale(s.ad_value(1446), 3.0), s.ad_value(1448)), A::square(s.ad_value(1447))), A::mul(A::scale(s.ad_value(1446), 9.0), s.ad_value(1446)));
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1503] != 0.0)) {
            s.store_sqrt_ad(1402, A::add(A::square(s.ad_value(1450)), A::mul(A::square(s.ad_value(1451)), s.ad_value(1451))));
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1503] != 0.0)) {
            s.store_powf_ad(1452, A::sub(s.ad_value(1402), s.ad_value(1450)), 0.3333333333333333);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1503] != 0.0)) {
            s.store_neg_ad(1453, A::powf(A::add(s.ad_value(1450), s.ad_value(1402)), 0.3333333333333333));
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1503] != 0.0)) {
            s.store_sub_ad(1405, A::add(s.ad_value(1452), s.ad_value(1453)), A::div(s.ad_value(1447), A::scale(s.ad_value(1446), 3.0)));
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1503] != 0.0)) {
            s.store_sub_ad_lhs(1445, A::mul(s.ad_value(1405), s.ad_value(227)), 1419);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1503] != 0.0)) {
            s.store_mul_ad_rhs(1442, 225, A::add(s.ad_value(1445), s.ad_value(1419)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_offset_ad(1454, A::add(s.ad_value(1430), s.ad_value(1419)), 0.1);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_offset_ad(1461, A::exp(A::mul(s.ad_value(225), A::neg(s.ad_value(1419)))), 1e-50);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_div(1398, 230, 521);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_square(1455, 1398);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_mul(1456, 1455, 1461);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_mul(1398, 226, 1429);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_mul(1457, 225, 1454);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_add_ad(1458, A::sub(A::ln(A::add(A::mul(s.ad_value(1456), s.ad_value(1398)), A::square(s.ad_value(1457)))), A::ln(A::mul(s.ad_value(1455), s.ad_value(1398)))), A::mul(s.ad_value(225), s.ad_value(1419)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_offset_ad(44, A::sub(s.ad_value(1457), s.ad_value(1458)), (-1.0));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_scale(45, 1457, 4.0);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_scale_ad(1399, A::offset(A::div(s.ad_value(44), s.ad_value(45)), 1.0), 0.5);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_scale_ad(1400, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(44), 2.0), s.ad_value(45))), 0.5);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_sub_ad_rhs(1458, 1457, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_sub(1457, 1457, 1458);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_add_ad_rhs(1457, 1457, A::scale(s.ad_value(225), 0.1));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_add_ad(1459, A::sub(A::ln(A::add(A::mul(s.ad_value(1456), s.ad_value(1398)), A::square(s.ad_value(1457)))), A::ln(A::mul(s.ad_value(1455), s.ad_value(1398)))), A::mul(s.ad_value(225), s.ad_value(1419)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.copy_ad(1460, 1442);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_offset_ad(44, A::sub(s.ad_value(1459), s.ad_value(1460)), (-(0.0008 * 75.0)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_scale(45, 1459, (4.0 * (0.0008 * 75.0)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_scale_ad(1399, A::offset(A::div(s.ad_value(44), s.ad_value(45)), 1.0), 0.5);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_scale_ad(1400, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(44), ((2.0 * 0.0008) * 75.0)), s.ad_value(45))), 0.5);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_sub_ad_rhs(1442, 1459, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_sub_ad_lhs(1444, A::div(s.ad_value(1442), s.ad_value(225)), 1419);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_add_ad(1399, A::offset(s.ad_value(1442), (-1.0)), A::exp(A::neg(s.ad_value(1442))));
        }

        s.v[1504] = if (s.v[1399] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1504] != 0.0)) {
            s.store_scalar(1399, (10.0 * 2.220446049250313e-16));
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_sqrt(1400, 1399);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_mul(458, 1427, 1400);
        }

        if ((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) {
            s.store_mul_ad_rhs(459, 1407, A::sub(s.ad_value(1430), s.ad_value(1444)));
        }

        s.v[1505] = if (p.p42 == 1.0) { 1.0 } else { 0.0 };

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) {
            s.store_exp_ad(1461, A::mul(s.ad_value(225), A::neg(s.ad_value(1419))));
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) {
            s.store_div(1398, 230, 521);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) {
            s.store_square(1455, 1398);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) {
            s.store_mul(1470, 1455, 1461);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) {
            s.store_scalar(1415, 0.0);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) {
            s.store_scalar(167, 1.0);
        }

        let mut assign27790_loop_guard: usize = 0;
        while {
            let assign27790_cond_e38754: f64 = (2.0 * 20.0);
            let assign27790_cond_e38756: f64 = (assign27790_cond_e38754 + 1.0);
            let assign27790_cond_e38758: f64 = if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) && (s.v[167] <= assign27790_cond_e38756)) { 1.0 } else { 0.0 };
            assign27790_cond_e38758 != 0.0
        } {
            assign27790_loop_guard += 1;
            assert!(assign27790_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) {
                s.store_scalar(1466, 0.0);
            }
            if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) {
                s.store_mul_ad_rhs(1442, 225, A::add(s.ad_value(1444), s.ad_value(1419)));
            }
            s.v[1506] = if (s.v[1442] < 5.0) { 1.0 } else { 0.0 };
            if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) && (s.v[1506] != 0.0)) {
                s.store_mul_ad(1462, A::mul(A::square(s.ad_value(1442)), s.ad_value(1442)), A::offset(A::mul(s.ad_value(1442), A::offset(A::scale(s.ad_value(1442), 0.006115288895133179), (-0.07053654284009761))), 0.29693154855771));
            }
            if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) && (s.v[1506] != 0.0)) {
                s.store_mul_ad(1463, A::square(s.ad_value(1442)), A::offset(A::mul(s.ad_value(1442), A::offset(A::scale(s.ad_value(1442), (5.0 * 0.006115288895133179)), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
            }
            if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) && (s.v[1506] != 0.0)) {
                s.store_mul_ad_lhs(1464, A::mul(s.ad_value(1470), s.ad_value(1462)), 1462);
            }
            if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) && (s.v[1506] != 0.0)) {
                s.store_mul_ad_lhs(1465, A::mul(A::scale(A::mul(s.ad_value(1470), s.ad_value(225)), 2.0), s.ad_value(1462)), 1463);
            }
            if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) && (s.v[1506] != 0.0)) {
                s.store_mul_ad_rhs(1466, 1442, A::offset(A::mul(s.ad_value(1442), A::offset(A::mul(s.ad_value(1442), A::offset(A::mul(s.ad_value(1442), A::offset(A::scale(s.ad_value(1442), 6.36964918866352e-5), (-0.00163730162779191))), 0.0178800506338833)), (-0.117851130197758))), 0.707106781186548));
            }
            if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) && (s.v[1506] != 0.0)) {
                s.store_offset_ad(1467, A::mul(s.ad_value(1442), A::offset(A::mul(s.ad_value(1442), A::offset(A::mul(s.ad_value(1442), A::offset(A::scale(s.ad_value(1442), (5.0 * 6.36964918866352e-5)), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833))), (2.0 * (-0.117851130197758)))), 0.707106781186548);
            }
            if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) && (s.v[1506] != 0.0)) {
                s.store_sqrt_ad(1468, A::offset(A::add(A::square(s.ad_value(1466)), s.ad_value(1464)), 1e-50));
            }
            if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) && (s.v[1506] != 0.0)) {
                s.store_div_ad(1469, A::add(A::mul(A::scale(A::mul(s.ad_value(225), s.ad_value(1467)), 2.0), s.ad_value(1466)), s.ad_value(1465)), A::scale(s.ad_value(1468), 2.0));
            }
            s.v[1507] = if (s.v[1442] < 80.0) { 1.0 } else { 0.0 };
            if (((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) && (!(s.v[1506] != 0.0))) && (s.v[1507] != 0.0)) {
                s.store_exp(243, 1442);
            }
            if (((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) && (!(s.v[1506] != 0.0))) && (s.v[1507] != 0.0)) {
                s.store_mul_ad_rhs(1464, 1470, A::offset(s.ad_value(243), (-1.0)));
            }
            if (((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) && (!(s.v[1506] != 0.0))) && (s.v[1507] != 0.0)) {
                s.store_mul_ad_lhs(1465, A::mul(s.ad_value(1470), s.ad_value(225)), 243);
            }
            if (((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) && (!(s.v[1506] != 0.0))) && (!(s.v[1507] != 0.0))) {
                s.store_exp_ad(1471, A::mul(s.ad_value(225), s.ad_value(1444)));
            }
            if (((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) && (!(s.v[1506] != 0.0))) && (!(s.v[1507] != 0.0))) {
                s.store_mul_ad_rhs(1464, 1455, A::sub(s.ad_value(1471), s.ad_value(1461)));
            }
            if (((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) && (!(s.v[1506] != 0.0))) && (!(s.v[1507] != 0.0))) {
                s.store_mul_ad_lhs(1465, A::mul(s.ad_value(1455), s.ad_value(225)), 1471);
            }
            if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) && (!(s.v[1506] != 0.0))) {
                s.store_sqrt_ad(1468, A::add(A::offset(s.ad_value(1442), (-1.0)), s.ad_value(1464)));
            }
            if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) && (!(s.v[1506] != 0.0))) {
                s.store_scale_ad(1469, A::div(A::add(s.ad_value(225), s.ad_value(1465)), s.ad_value(1468)), 0.5);
            }
            if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) {
                s.store_sub_ad(1472, A::sub(s.ad_value(1430), s.ad_value(1444)), A::mul(s.ad_value(1428), s.ad_value(1468)));
            }
            if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) {
                s.store_sub_from_scalar_ad(1473, (-1.0), A::mul(s.ad_value(1428), s.ad_value(1469)));
            }
            s.v[1508] = if (s.v[1415] == 1.0) { 1.0 } else { 0.0 };
            if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) && (s.v[1508] != 0.0)) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) && (!(s.v[1508] != 0.0))) {
                s.store_div_ad_lhs(494, A::neg(s.ad_value(1472)), 1473);
            }
            if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) && (!(s.v[1508] != 0.0))) {
                s.store_scale_ad(1474, A::offset({
                    if (1.0 >= ((s.v[1444]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1444))
                    }
                }, 1.0), (0.5 * 0.1));
            }
            s.v[1509] = if (((s.v[494]) as f64).abs() > s.v[1474]) { 1.0 } else { 0.0 };
            if (((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) && (!(s.v[1508] != 0.0))) && (s.v[1509] != 0.0)) {
                s.store_scale(494, 1474, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) && (!(s.v[1508] != 0.0))) {
                s.store_add(1444, 1444, 494);
            }
            s.v[1510] = if ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1472]) as f64).abs() <= 1e-8)) { 1.0 } else { 0.0 };
            if (((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) && (!(s.v[1508] != 0.0))) && (s.v[1510] != 0.0)) {
                s.store_scalar(1415, 1.0);
            }
            if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.v[1512] = if (s.v[1442] < 5.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) && (s.v[1512] != 0.0)) {
            s.store_offset_ad(1475, A::square(s.ad_value(1466)), (10.0 * 2.220446049250313e-16));
        }

        if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) && (s.v[1512] != 0.0)) {
            s.store_offset(1476, 1466, (10.0 * 2.220446049250313e-16));
        }

        if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) && (!(s.v[1512] != 0.0))) {
            s.store_offset(1475, 1442, (-1.0));
        }

        if ((((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) && (!(s.v[1512] != 0.0))) {
            s.store_sqrt(1476, 1475);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) {
            s.store_mul(458, 1427, 1476);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) {
            s.store_div_from_scalar_ad(1399, 1.0, A::add(s.ad_value(1468), s.ad_value(1476)));
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) {
            s.store_mul_ad_lhs(460, A::mul(s.ad_value(1427), s.ad_value(1464)), 1399);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1499] != 0.0))) && (s.v[1505] != 0.0)) {
            s.store_add(459, 458, 460);
        }

        if (((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) {
            s.store_sub(460, 459, 458);
        }

        s.v[1514] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1515] = if (1.0 == 2.0) { 1.0 } else { 0.0 };

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1514] != 0.0)) && (s.v[1409] != 0.0)) {
            s.store_mul_ad_lhs(463, A::neg(s.ad_value(522)), 459);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1514] != 0.0)) && (s.v[1409] != 0.0)) {
            s.store_mul_ad_lhs(465, A::neg(s.ad_value(522)), 460);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1514] != 0.0)) && (s.v[1410] != 0.0)) {
            s.store_mul_ad_lhs(464, A::neg(s.ad_value(522)), 459);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1514] != 0.0)) && (s.v[1410] != 0.0)) {
            s.store_mul_ad_lhs(466, A::neg(s.ad_value(522)), 460);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && ((s.v[1515] != 0.0) && (!(s.v[1514] != 0.0)))) && (s.v[1409] != 0.0)) {
            s.store_mul_ad_lhs(467, A::neg(s.ad_value(522)), 459);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && ((s.v[1515] != 0.0) && (!(s.v[1514] != 0.0)))) && (s.v[1409] != 0.0)) {
            s.store_mul_ad_lhs(469, A::neg(s.ad_value(522)), 460);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && ((s.v[1515] != 0.0) && (!(s.v[1514] != 0.0)))) && (s.v[1410] != 0.0)) {
            s.store_mul_ad_lhs(468, A::neg(s.ad_value(522)), 459);
        }

        if (((((s.v[1397] != 0.0) && (p.p24 != 0.0)) && (s.v[1479] != 0.0)) && ((s.v[1515] != 0.0) && (!(s.v[1514] != 0.0)))) && (s.v[1410] != 0.0)) {
            s.store_mul_ad_lhs(470, A::neg(s.ad_value(522)), 460);
        }

        s.v[317] = p.p189;

        s.v[1518] = if (s.v[145] != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1518] != 0.0) {
            s.store_add(1517, 157, 161);
        }

        if (s.v[1518] != 0.0) {
            s.store_add_ad(314, A::scale(s.ad_value(1517), s.v[317]), A::scale(s.ad_value(162), (1.0 - s.v[317])));
        }

        s.v[1519] = if (p.p64 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1518] != 0.0) && (s.v[1519] != 0.0)) {
            s.store_scalar(315, 0.0);
        }

        s.v[1520] = if (s.v[314] > ((s.v[161] + s.v[157]) - (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if ((s.v[1518] != 0.0) && (s.v[1520] != 0.0)) {
            s.store_offset_ad(314, A::add(s.ad_value(161), s.ad_value(157)), (-(10.0 * 2.220446049250313e-16)));
        }

        s.v[1521] = if (p.p64 != 0.0) { 1.0 } else { 0.0 };

        s.v[1522] = if (s.v[246] < 1e-15) { 1.0 } else { 0.0 };

        if (((!(s.v[1518] != 0.0)) && (s.v[1521] != 0.0)) && (s.v[1522] != 0.0)) {
            s.store_scalar(315, 0.0);
        }

        if (((!(s.v[1518] != 0.0)) && (s.v[1521] != 0.0)) && (!(s.v[1522] != 0.0))) {
            s.store_scale(1516, 227, 1.0 / (s.v[97]));
        }

        if (((!(s.v[1518] != 0.0)) && (s.v[1521] != 0.0)) && (!(s.v[1522] != 0.0))) {
            s.store_div_from_scalar(1517, 1.0, 244);
        }

        if (((!(s.v[1518] != 0.0)) && (s.v[1521] != 0.0)) && (!(s.v[1522] != 0.0))) {
            s.store_mul_ad_lhs(315, A::mul(s.ad_value(246), s.ad_value(1516)), 1517);
        }

        s.v[1534] = s.v[91];

        s.v[1535] = (1.0 / s.v[1534]);

        s.v[1555] = 0.0;

        s.v[1595] = 0.0;

        s.v[1593] = 0.0;

        s.v[1597] = 0.0;

        s.v[1606] = if ((p.p29 >= 1.0) && (p.p188 > 0.0)) { 1.0 } else { 0.0 };

        if ((p.p24 != 0.0) && (s.v[1606] != 0.0)) {
            s.store_scalar(1537, p.p171);
        }

        if ((p.p24 != 0.0) && (s.v[1606] != 0.0)) {
            s.store_scalar(1538, p.p172);
        }

        if ((p.p24 != 0.0) && (s.v[1606] != 0.0)) {
            s.copy_ad(1539, 158);
        }

        if ((p.p24 != 0.0) && (s.v[1606] != 0.0)) {
            s.store_scalar(1536, p.p188);
        }

        s.v[1607] = if ((s.v[69] == 0.0) && (p.p188 > 0.0)) { 1.0 } else { 0.0 };

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (s.v[1607] != 0.0)) {
            s.store_ad(1524, &{
                if (p.p43 == 1.0) {
                    A::scale(s.ad_value(287), s.v[1534])
                } else {
                    A::scale(s.ad_value(108), s.v[1534])
                }
            });
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (s.v[1607] != 0.0)) {
            s.store_mul_ad(1527, A::mul(s.ad_value(1537), s.ad_value(1524)), A::add(s.ad_value(1538), s.ad_value(1539)));
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (s.v[1607] != 0.0)) {
            s.store_mul(1528, 1536, 1524);
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (s.v[1607] != 0.0)) {
            s.copy_ad(1532, 161);
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (s.v[1607] != 0.0)) {
            s.store_sub_from_scalar(1529, 1.2, 1532);
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (s.v[1607] != 0.0)) {
            s.store_sub_ad(267, A::mul(s.ad_value(158), s.ad_value(1528)), A::mul(s.ad_value(1529), s.ad_value(1527)));
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (s.v[1607] != 0.0)) {
            s.store_mul_ad(1527, A::mul(s.ad_value(1537), s.ad_value(1524)), A::sub(A::add(s.ad_value(1538), s.ad_value(1539)), s.ad_value(157)));
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (s.v[1607] != 0.0)) {
            s.store_sub(1532, 162, 157);
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (s.v[1607] != 0.0)) {
            s.store_sub_from_scalar(1529, 1.2, 1532);
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (s.v[1607] != 0.0)) {
            s.store_sub_ad(268, A::mul(A::sub(s.ad_value(158), s.ad_value(157)), s.ad_value(1528)), A::mul(s.ad_value(1527), s.ad_value(1529)));
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) {
            s.store_mul_ad_rhs(1556, 238, A::sqrt(A::div_from_scalar(s.v[69], s.ad_value(536))));
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) {
            s.store_scalar(1540, ((1.0 - -1.0) / 2.0));
        }

    }

    pub(super) fn stamp_transient_block_25(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) {
            s.store_scalar(1541, ((1.0 + -1.0) / 2.0));
        }

        s.v[1608] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1608] != 0.0)) {
            s.store_add_ad(1550, A::mul(s.ad_value(461), s.ad_value(156)), A::mul(s.ad_value(462), A::sub(s.ad_value(156), s.ad_value(157))));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1608] != 0.0)) {
            s.store_add_ad(1551, A::mul(s.ad_value(461), s.ad_value(157)), A::mul(s.ad_value(462), A::neg(s.ad_value(157))));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1608] != 0.0)) {
            s.store_add_ad(1552, A::mul(s.ad_value(461), s.ad_value(158)), A::mul(s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1608] != 0.0)) {
            s.store_sub(1553, 1551, 1550);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1608] != 0.0)) {
            s.store_sub(1555, 1552, 1550);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1608] != 0.0)) {
            s.store_neg(1554, 1550);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1608] != 0.0)) {
            s.store_add_ad(1542, A::mul(s.ad_value(1540), s.ad_value(461)), A::mul(s.ad_value(1541), s.ad_value(462)));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1608] != 0.0)) {
            s.store_add_ad(1543, A::mul(s.ad_value(1540), s.ad_value(462)), A::mul(s.ad_value(1541), s.ad_value(461)));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1608] != 0.0)) {
            s.store_offset_ad(1548, A::add(A::mul(s.ad_value(1542), s.ad_value(1554)), A::mul(s.ad_value(1543), s.ad_value(1553))), (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1608] != 0.0))) {
            s.store_add_ad(1542, A::mul(s.ad_value(1540), s.ad_value(461)), A::mul(s.ad_value(1541), s.ad_value(462)));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1608] != 0.0))) {
            s.store_add_ad(1543, A::mul(s.ad_value(1540), s.ad_value(462)), A::mul(s.ad_value(1541), s.ad_value(461)));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1540] != 0.0)) {
            s.store_add_ad(1555, A::mul(s.ad_value(461), s.ad_value(158)), A::mul(s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1608] != 0.0))) && (s.v[1541] != 0.0)) {
            s.store_add_ad(1555, A::mul(s.ad_value(462), s.ad_value(158)), A::mul(s.ad_value(461), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1608] != 0.0))) {
            s.store_scalar(1548, 0.0);
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) {
            s.store_neg(1523, 1548);
        }

        s.v[1609] = if (s.v[1523] > s.v[141]) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1609] != 0.0)) {
            s.store_sub(1524, 1523, 141);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1609] != 0.0)) {
            s.store_sub(1525, 140, 141);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1609] != 0.0)) {
            s.store_div(44, 1524, 1525);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1609] != 0.0)) {
            s.store_square(45, 44);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1609] != 0.0)) {
            s.store_mul(46, 45, 44);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1609] != 0.0)) {
            s.store_square(47, 45);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1609] != 0.0)) {
            s.store_div_from_scalar_ad(1533, 1.0, A::add(A::add(A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(45)), s.ad_value(46)), s.ad_value(47)));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1609] != 0.0)) {
            s.store_mul_ad_rhs(1533, 1525, A::sub_from_scalar(1.0, s.ad_value(1533)));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1609] != 0.0)) {
            s.store_add(1530, 141, 1533);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1609] != 0.0))) {
            s.copy_ad(1530, 1523);
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) {
            s.store_offset_ad(1549, A::neg(s.ad_value(1530)), (-1e-12));
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) {
            s.store_scale(1557, 1556, s.v[1535]);
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) {
            s.store_square(1558, 1557);
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) {
            s.store_sub_from_scalar(1559, s.v[82], 1555);
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) {
            s.store_div_from_scalar(1523, s.v[69], 230);
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) {
            s.store_mul_ad(1560, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(s.ad_value(1523)));
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) {
            s.store_neg(1561, 1549);
        }

        s.v[1610] = if (s.v[1559] < s.v[1561]) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_div_from_scalar_ad(1524, 1.0, A::mul(s.ad_value(225), s.ad_value(1556)));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_scale(1533, 1524, s.v[1534]);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_offset_scaled(1562, 1533, (3.0 * 1.414213562373095), 2.0);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_mul_ad_lhs(1563, A::mul(A::scale(s.ad_value(1562), 8.0), s.ad_value(1562)), 1562);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_sub(1564, 237, 1560);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_mul_ad_rhs(1532, 225, A::add(s.ad_value(1559), s.ad_value(1549)));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_sub_from_scalar_ad(1565, (7.0 * 1.414213562373095), A::mul(A::scale(s.ad_value(1533), 9.0), A::offset(s.ad_value(1532), (-2.0))));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_square(1566, 1565);
        }

        s.v[1611] = if (s.v[1563] < (s.v[1566] * 1e-8)) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1610] != 0.0)) && (s.v[1611] != 0.0)) {
            s.store_add_ad(1568, A::add(A::offset(s.ad_value(1565), ((-7.0) * 1.414213562373095)), A::div(A::scale(s.ad_value(1563), 0.5), s.ad_value(1565))), A::mul(A::scale(s.ad_value(1533), 9.0), A::offset(s.ad_value(1532), (-2.0))));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1610] != 0.0)) && (!(s.v[1611] != 0.0))) {
            s.store_sqrt_ad(1567, A::add(s.ad_value(1563), s.ad_value(1566)));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1610] != 0.0)) && (!(s.v[1611] != 0.0))) {
            s.store_add_ad(1568, A::offset(s.ad_value(1567), ((-7.0) * 1.414213562373095)), A::mul(A::scale(s.ad_value(1533), 9.0), A::offset(s.ad_value(1532), (-2.0))));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_powf(1569, 1568, 0.3333333333333333);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_add_ad(1570, A::add(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1533), 12.0)), A::scale(s.ad_value(1569), 2.0)), A::mul(A::scale(s.ad_value(1569), 1.414213562373095), s.ad_value(1569)));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_div(1571, 1570, 1569);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_sub_ad_lhs(1572, A::mul(s.ad_value(1571), s.ad_value(227)), 1549);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_add(1524, 1572, 1549);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_div(1525, 1524, 1564);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_sqrt_ad(1526, A::offset(A::square(s.ad_value(1525)), 1.0));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_sub_ad_lhs(1573, A::div(s.ad_value(1524), s.ad_value(1526)), 1549);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_sub(1525, 1559, 1573);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_scale(459, 1525, s.v[1534]);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1610] != 0.0)) {
            s.copy_ad(458, 459);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) {
            s.store_scalar(1571, 3.0);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) {
            s.store_sub_ad_lhs(1574, A::div(s.ad_value(1571), s.ad_value(225)), 1549);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) {
            s.store_exp_ad(1533, A::neg(s.ad_value(1571)));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) {
            s.store_offset_ad(1532, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1559), s.ad_value(1549))), (-1.0)), s.ad_value(1533)), 4.0), A::mul(s.ad_value(1558), s.ad_value(226))), 1.0);
        }

        s.v[1612] = if (s.v[1532] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_scalar(1532, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) {
            s.store_add_ad_rhs(1574, 1559, A::mul(A::scale(A::mul(s.ad_value(1558), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1532)))));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) {
            s.store_mul_ad_rhs(1571, 225, A::add(s.ad_value(1574), s.ad_value(1549)));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) {
            s.store_exp_ad(1533, A::neg(s.ad_value(1571)));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) {
            s.store_offset_ad(1532, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1559), s.ad_value(1549))), (-1.0)), s.ad_value(1533)), 4.0), A::mul(s.ad_value(1558), s.ad_value(226))), 1.0);
        }

        s.v[1613] = if (s.v[1532] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_scalar(1532, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) {
            s.store_add_ad_rhs(1574, 1559, A::mul(A::scale(A::mul(s.ad_value(1558), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1532)))));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) {
            s.store_mul_ad_rhs(1571, 225, A::add(s.ad_value(1574), s.ad_value(1549)));
        }

        s.v[1614] = if (s.v[1571] < 3.0) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1614] != 0.0)) {
            s.store_scalar(1575, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1614] != 0.0)) {
            s.store_scalar(1576, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1614] != 0.0)) {
            s.store_offset_ad(1577, A::div_from_scalar(1.0, A::mul(s.ad_value(225), s.ad_value(1557))), (1.0 / 1.414213562373095));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1614] != 0.0)) {
            s.store_div_ad_lhs(1578, A::neg(A::add(s.ad_value(1559), s.ad_value(1549))), 1557);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1614] != 0.0)) {
            s.store_add_ad(1579, A::sub(A::div(A::mul(A::square(s.ad_value(1576)), s.ad_value(1576)), A::mul(A::mul(A::scale(s.ad_value(1575), 27.0), s.ad_value(1575)), s.ad_value(1575))), A::div(A::mul(s.ad_value(1576), s.ad_value(1577)), A::mul(A::scale(s.ad_value(1575), 6.0), s.ad_value(1575)))), A::div(s.ad_value(1578), A::scale(s.ad_value(1575), 2.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1614] != 0.0)) {
            s.store_div_ad(1580, A::sub(A::mul(A::scale(s.ad_value(1575), 3.0), s.ad_value(1577)), A::square(s.ad_value(1576))), A::mul(A::scale(s.ad_value(1575), 9.0), s.ad_value(1575)));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1614] != 0.0)) {
            s.store_sqrt_ad(1528, A::add(A::square(s.ad_value(1579)), A::mul(A::square(s.ad_value(1580)), s.ad_value(1580))));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1614] != 0.0)) {
            s.store_powf_ad(1581, A::sub(s.ad_value(1528), s.ad_value(1579)), 0.3333333333333333);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1614] != 0.0)) {
            s.store_neg_ad(1582, A::powf(A::add(s.ad_value(1579), s.ad_value(1528)), 0.3333333333333333));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1614] != 0.0)) {
            s.store_sub_ad(1532, A::add(s.ad_value(1581), s.ad_value(1582)), A::div(s.ad_value(1576), A::scale(s.ad_value(1575), 3.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1614] != 0.0)) {
            s.store_sub_ad_lhs(1574, A::mul(s.ad_value(1532), s.ad_value(227)), 1549);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1614] != 0.0)) {
            s.store_mul_ad_rhs(1571, 225, A::add(s.ad_value(1574), s.ad_value(1549)));
        }

        s.v[1615] = if (p.p41 > 0.0) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_offset_ad(1583, A::add(s.ad_value(1559), s.ad_value(1549)), 0.1);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_offset_ad(1590, A::exp(A::mul(s.ad_value(225), A::neg(s.ad_value(1549)))), 1e-50);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_scale(1523, 230, 1.0 / (s.v[69]));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_square(1584, 1523);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_mul(1585, 1584, 1590);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_mul(1523, 226, 1558);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_mul(1586, 225, 1583);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_add_ad(1587, A::sub(A::ln(A::add(A::mul(s.ad_value(1585), s.ad_value(1523)), A::square(s.ad_value(1586)))), A::ln(A::mul(s.ad_value(1584), s.ad_value(1523)))), A::mul(s.ad_value(225), s.ad_value(1549)));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_offset_ad(44, A::sub(s.ad_value(1586), s.ad_value(1587)), (-1.0));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_scale(45, 1586, 4.0);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_scale_ad(1524, A::offset(A::div(s.ad_value(44), s.ad_value(45)), 1.0), 0.5);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_scale_ad(1525, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(44), 2.0), s.ad_value(45))), 0.5);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_sub_ad_rhs(1587, 1586, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_sub(1586, 1586, 1587);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_add_ad_rhs(1586, 1586, A::scale(s.ad_value(225), 0.1));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_add_ad(1588, A::sub(A::ln(A::add(A::mul(s.ad_value(1585), s.ad_value(1523)), A::square(s.ad_value(1586)))), A::ln(A::mul(s.ad_value(1584), s.ad_value(1523)))), A::mul(s.ad_value(225), s.ad_value(1549)));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1615] != 0.0)) {
            s.copy_ad(1589, 1571);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_offset_ad(44, A::sub(s.ad_value(1588), s.ad_value(1589)), (-(0.0008 * 75.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_scale(45, 1588, (4.0 * (0.0008 * 75.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_scale_ad(1524, A::offset(A::div(s.ad_value(44), s.ad_value(45)), 1.0), 0.5);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_scale_ad(1525, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(44), ((2.0 * 0.0008) * 75.0)), s.ad_value(45))), 0.5);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_sub_ad_rhs(1571, 1588, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) {
            s.store_sub_ad_lhs(1573, A::div(s.ad_value(1571), s.ad_value(225)), 1549);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) {
            s.store_add_ad(1524, A::offset(s.ad_value(1571), (-1.0)), A::exp(A::neg(s.ad_value(1571))));
        }

        s.v[1616] = if (s.v[1524] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1616] != 0.0)) {
            s.store_scalar(1524, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) {
            s.store_sqrt(1525, 1524);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) {
            s.store_mul(458, 1556, 1525);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) {
            s.store_scaled_sub(459, 1559, 1573, s.v[1534]);
        }

        s.v[1617] = if (p.p41 == 1.0) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) {
            s.store_exp_ad(1590, A::mul(s.ad_value(225), A::neg(s.ad_value(1549))));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) {
            s.store_scale(1523, 230, 1.0 / (s.v[69]));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) {
            s.store_square(1584, 1523);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) {
            s.store_mul(1599, 1584, 1590);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) {
            s.store_scalar(1546, 0.0);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) {
            s.store_scalar(1593, 0.0);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) {
            s.store_scalar(1597, 0.0);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) {
            s.store_scalar(167, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_26(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let mut assign29790_loop_guard: usize = 0;
        while {
            let assign29790_cond_e42287: f64 = (2.0 * 20.0);
            let assign29790_cond_e42289: f64 = (assign29790_cond_e42287 + 1.0);
            let assign29790_cond_e42291: f64 = if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) && (s.v[167] <= assign29790_cond_e42289)) { 1.0 } else { 0.0 };
            assign29790_cond_e42291 != 0.0
        } {
            assign29790_loop_guard += 1;
            assert!(assign29790_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) {
                s.store_scalar(1595, 0.0);
            }
            if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) {
                s.store_mul_ad_rhs(1571, 225, A::add(s.ad_value(1573), s.ad_value(1549)));
            }
            s.v[1618] = if (s.v[1571] < 5.0) { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) && (s.v[1618] != 0.0)) {
                s.store_mul_ad(1591, A::mul(A::square(s.ad_value(1571)), s.ad_value(1571)), A::offset(A::mul(s.ad_value(1571), A::offset(A::scale(s.ad_value(1571), 0.006115288895133179), (-0.07053654284009761))), 0.29693154855771));
            }
            if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) && (s.v[1618] != 0.0)) {
                s.store_mul_ad(1592, A::square(s.ad_value(1571)), A::offset(A::mul(s.ad_value(1571), A::offset(A::scale(s.ad_value(1571), (5.0 * 0.006115288895133179)), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
            }
            if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) && (s.v[1618] != 0.0)) {
                s.store_mul_ad_lhs(1593, A::mul(s.ad_value(1599), s.ad_value(1591)), 1591);
            }
            if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) && (s.v[1618] != 0.0)) {
                s.store_mul_ad_lhs(1594, A::mul(A::scale(A::mul(s.ad_value(1599), s.ad_value(225)), 2.0), s.ad_value(1591)), 1592);
            }
            if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) && (s.v[1618] != 0.0)) {
                s.store_mul_ad_rhs(1595, 1571, A::offset(A::mul(s.ad_value(1571), A::offset(A::mul(s.ad_value(1571), A::offset(A::mul(s.ad_value(1571), A::offset(A::scale(s.ad_value(1571), 6.36964918866352e-5), (-0.00163730162779191))), 0.0178800506338833)), (-0.117851130197758))), 0.707106781186548));
            }
            if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) && (s.v[1618] != 0.0)) {
                s.store_offset_ad(1596, A::mul(s.ad_value(1571), A::offset(A::mul(s.ad_value(1571), A::offset(A::mul(s.ad_value(1571), A::offset(A::scale(s.ad_value(1571), (5.0 * 6.36964918866352e-5)), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833))), (2.0 * (-0.117851130197758)))), 0.707106781186548);
            }
            if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) && (s.v[1618] != 0.0)) {
                s.store_sqrt_ad(1597, A::offset(A::add(A::square(s.ad_value(1595)), s.ad_value(1593)), 1e-50));
            }
            if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) && (s.v[1618] != 0.0)) {
                s.store_div_ad(1598, A::add(A::mul(A::scale(A::mul(s.ad_value(225), s.ad_value(1596)), 2.0), s.ad_value(1595)), s.ad_value(1594)), A::scale(s.ad_value(1597), 2.0));
            }
            s.v[1619] = if (s.v[1571] < 80.0) { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) && (!(s.v[1618] != 0.0))) && (s.v[1619] != 0.0)) {
                s.store_exp(243, 1571);
            }
            if (((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) && (!(s.v[1618] != 0.0))) && (s.v[1619] != 0.0)) {
                s.store_mul_ad_rhs(1593, 1599, A::offset(s.ad_value(243), (-1.0)));
            }
            if (((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) && (!(s.v[1618] != 0.0))) && (s.v[1619] != 0.0)) {
                s.store_mul_ad_lhs(1594, A::mul(s.ad_value(1599), s.ad_value(225)), 243);
            }
            if (((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) && (!(s.v[1618] != 0.0))) && (!(s.v[1619] != 0.0))) {
                s.store_exp_ad(1600, A::mul(s.ad_value(225), s.ad_value(1573)));
            }
            if (((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) && (!(s.v[1618] != 0.0))) && (!(s.v[1619] != 0.0))) {
                s.store_mul_ad_rhs(1593, 1584, A::sub(s.ad_value(1600), s.ad_value(1590)));
            }
            if (((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) && (!(s.v[1618] != 0.0))) && (!(s.v[1619] != 0.0))) {
                s.store_mul_ad_lhs(1594, A::mul(s.ad_value(1584), s.ad_value(225)), 1600);
            }
            if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) && (!(s.v[1618] != 0.0))) {
                s.store_sqrt_ad(1597, A::add(A::offset(s.ad_value(1571), (-1.0)), s.ad_value(1593)));
            }
            if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) && (!(s.v[1618] != 0.0))) {
                s.store_scale_ad(1598, A::div(A::add(s.ad_value(225), s.ad_value(1594)), s.ad_value(1597)), 0.5);
            }
            if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) {
                s.store_sub_ad(1601, A::sub(s.ad_value(1559), s.ad_value(1573)), A::mul(s.ad_value(1557), s.ad_value(1597)));
            }
            if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) {
                s.store_sub_from_scalar_ad(1602, (-1.0), A::mul(s.ad_value(1557), s.ad_value(1598)));
            }
            s.v[1620] = if (s.v[1546] == 1.0) { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) && (s.v[1620] != 0.0)) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) && (!(s.v[1620] != 0.0))) {
                s.store_div_ad_lhs(494, A::neg(s.ad_value(1601)), 1602);
            }
            if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) && (!(s.v[1620] != 0.0))) {
                s.store_scale_ad(1603, A::offset({
                    if (1.0 >= ((s.v[1573]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1573))
                    }
                }, 1.0), (0.5 * 0.1));
            }
            s.v[1621] = if (((s.v[494]) as f64).abs() > s.v[1603]) { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) && (!(s.v[1620] != 0.0))) && (s.v[1621] != 0.0)) {
                s.store_scale(494, 1603, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) && (!(s.v[1620] != 0.0))) {
                s.store_add(1573, 1573, 494);
            }
            s.v[1622] = if ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1601]) as f64).abs() <= 1e-8)) { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) && (!(s.v[1620] != 0.0))) && (s.v[1622] != 0.0)) {
                s.store_scalar(1546, 1.0);
            }
            if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.v[1624] = if (s.v[1571] < 5.0) { 1.0 } else { 0.0 };

        if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) && (s.v[1624] != 0.0)) {
            s.store_offset_ad(1604, A::square(s.ad_value(1595)), (10.0 * 2.220446049250313e-16));
        }

        if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) && (s.v[1624] != 0.0)) {
            s.store_offset(1605, 1595, (10.0 * 2.220446049250313e-16));
        }

        if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) && (!(s.v[1624] != 0.0))) {
            s.store_offset(1604, 1571, (-1.0));
        }

        if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) && (!(s.v[1624] != 0.0))) {
            s.store_sqrt(1605, 1604);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) {
            s.store_mul(458, 1556, 1605);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) {
            s.store_div_from_scalar_ad(1524, 1.0, A::add(s.ad_value(1597), s.ad_value(1605)));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) {
            s.store_mul_ad_lhs(460, A::mul(s.ad_value(1556), s.ad_value(1593)), 1524);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1610] != 0.0))) && (s.v[1617] != 0.0)) {
            s.store_add(459, 458, 460);
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) {
            s.store_sub(460, 459, 458);
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) {
            s.store_ad(1527, &{
                if (p.p43 == 1.0) {
                    A::mul(s.ad_value(287), s.ad_value(1536))
                } else {
                    A::mul(s.ad_value(108), s.ad_value(1536))
                }
            });
        }

        s.v[1626] = if (((s.v[1542] != 0.0) && (p.p43 == 0.0)) || ((s.v[1540] != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_mul(455, 1527, 459);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_mul(457, 1527, 458);
        }

        s.v[1627] = if (((s.v[1543] != 0.0) && (p.p43 == 0.0)) || ((s.v[1541] != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1627] != 0.0)) {
            s.store_mul(454, 1527, 459);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1627] != 0.0)) {
            s.store_mul(456, 1527, 458);
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) {
            s.store_scalar(1540, ((1.0 - 1.0) / 2.0));
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) {
            s.store_scalar(1541, ((1.0 + 1.0) / 2.0));
        }

        s.v[1628] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_add_ad(1550, A::mul(s.ad_value(461), s.ad_value(156)), A::mul(s.ad_value(462), A::sub(s.ad_value(156), s.ad_value(157))));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_add_ad(1551, A::mul(s.ad_value(461), s.ad_value(157)), A::mul(s.ad_value(462), A::neg(s.ad_value(157))));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_add_ad(1552, A::mul(s.ad_value(461), s.ad_value(158)), A::mul(s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_sub(1553, 1551, 1550);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_sub(1555, 1552, 1550);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_neg(1554, 1550);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_add_ad(1542, A::mul(s.ad_value(1540), s.ad_value(461)), A::mul(s.ad_value(1541), s.ad_value(462)));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_add_ad(1543, A::mul(s.ad_value(1540), s.ad_value(462)), A::mul(s.ad_value(1541), s.ad_value(461)));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_offset_ad(1548, A::add(A::mul(s.ad_value(1542), s.ad_value(1554)), A::mul(s.ad_value(1543), s.ad_value(1553))), (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1628] != 0.0))) {
            s.store_add_ad(1542, A::mul(s.ad_value(1540), s.ad_value(461)), A::mul(s.ad_value(1541), s.ad_value(462)));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1628] != 0.0))) {
            s.store_add_ad(1543, A::mul(s.ad_value(1540), s.ad_value(462)), A::mul(s.ad_value(1541), s.ad_value(461)));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1540] != 0.0)) {
            s.store_add_ad(1555, A::mul(s.ad_value(461), s.ad_value(158)), A::mul(s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1628] != 0.0))) && (s.v[1541] != 0.0)) {
            s.store_add_ad(1555, A::mul(s.ad_value(462), s.ad_value(158)), A::mul(s.ad_value(461), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1628] != 0.0))) {
            s.store_scalar(1548, 0.0);
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) {
            s.store_neg(1523, 1548);
        }

        s.v[1629] = if (s.v[1523] > s.v[141]) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1629] != 0.0)) {
            s.store_sub(1524, 1523, 141);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1629] != 0.0)) {
            s.store_sub(1525, 140, 141);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1629] != 0.0)) {
            s.store_div(44, 1524, 1525);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1629] != 0.0)) {
            s.store_square(45, 44);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1629] != 0.0)) {
            s.store_mul(46, 45, 44);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1629] != 0.0)) {
            s.store_square(47, 45);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1629] != 0.0)) {
            s.store_div_from_scalar_ad(1533, 1.0, A::add(A::add(A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(45)), s.ad_value(46)), s.ad_value(47)));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1629] != 0.0)) {
            s.store_mul_ad_rhs(1533, 1525, A::sub_from_scalar(1.0, s.ad_value(1533)));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1629] != 0.0)) {
            s.store_add(1530, 141, 1533);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1629] != 0.0))) {
            s.copy_ad(1530, 1523);
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) {
            s.store_offset_ad(1549, A::neg(s.ad_value(1530)), (-1e-12));
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) {
            s.store_scale(1557, 1556, s.v[1535]);
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) {
            s.store_square(1558, 1557);
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) {
            s.store_sub_from_scalar(1559, s.v[82], 1555);
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) {
            s.store_div_from_scalar(1523, s.v[69], 230);
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) {
            s.store_mul_ad(1560, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(s.ad_value(1523)));
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) {
            s.store_neg(1561, 1549);
        }

        s.v[1630] = if (s.v[1559] < s.v[1561]) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_div_from_scalar_ad(1524, 1.0, A::mul(s.ad_value(225), s.ad_value(1556)));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_scale(1533, 1524, s.v[1534]);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_offset_scaled(1562, 1533, (3.0 * 1.414213562373095), 2.0);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_mul_ad_lhs(1563, A::mul(A::scale(s.ad_value(1562), 8.0), s.ad_value(1562)), 1562);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_sub(1564, 237, 1560);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_mul_ad_rhs(1532, 225, A::add(s.ad_value(1559), s.ad_value(1549)));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_sub_from_scalar_ad(1565, (7.0 * 1.414213562373095), A::mul(A::scale(s.ad_value(1533), 9.0), A::offset(s.ad_value(1532), (-2.0))));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_square(1566, 1565);
        }

        s.v[1631] = if (s.v[1563] < (s.v[1566] * 1e-8)) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1630] != 0.0)) && (s.v[1631] != 0.0)) {
            s.store_add_ad(1568, A::add(A::offset(s.ad_value(1565), ((-7.0) * 1.414213562373095)), A::div(A::scale(s.ad_value(1563), 0.5), s.ad_value(1565))), A::mul(A::scale(s.ad_value(1533), 9.0), A::offset(s.ad_value(1532), (-2.0))));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1630] != 0.0)) && (!(s.v[1631] != 0.0))) {
            s.store_sqrt_ad(1567, A::add(s.ad_value(1563), s.ad_value(1566)));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1630] != 0.0)) && (!(s.v[1631] != 0.0))) {
            s.store_add_ad(1568, A::offset(s.ad_value(1567), ((-7.0) * 1.414213562373095)), A::mul(A::scale(s.ad_value(1533), 9.0), A::offset(s.ad_value(1532), (-2.0))));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_powf(1569, 1568, 0.3333333333333333);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_add_ad(1570, A::add(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1533), 12.0)), A::scale(s.ad_value(1569), 2.0)), A::mul(A::scale(s.ad_value(1569), 1.414213562373095), s.ad_value(1569)));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_div(1571, 1570, 1569);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_sub_ad_lhs(1572, A::mul(s.ad_value(1571), s.ad_value(227)), 1549);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_add(1524, 1572, 1549);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_div(1525, 1524, 1564);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_sqrt_ad(1526, A::offset(A::square(s.ad_value(1525)), 1.0));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_sub_ad_lhs(1573, A::div(s.ad_value(1524), s.ad_value(1526)), 1549);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_sub(1525, 1559, 1573);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_scale(459, 1525, s.v[1534]);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1630] != 0.0)) {
            s.copy_ad(458, 459);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) {
            s.store_scalar(1571, 3.0);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) {
            s.store_sub_ad_lhs(1574, A::div(s.ad_value(1571), s.ad_value(225)), 1549);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) {
            s.store_exp_ad(1533, A::neg(s.ad_value(1571)));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) {
            s.store_offset_ad(1532, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1559), s.ad_value(1549))), (-1.0)), s.ad_value(1533)), 4.0), A::mul(s.ad_value(1558), s.ad_value(226))), 1.0);
        }

        s.v[1632] = if (s.v[1532] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1632] != 0.0)) {
            s.store_scalar(1532, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) {
            s.store_add_ad_rhs(1574, 1559, A::mul(A::scale(A::mul(s.ad_value(1558), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1532)))));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) {
            s.store_mul_ad_rhs(1571, 225, A::add(s.ad_value(1574), s.ad_value(1549)));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) {
            s.store_exp_ad(1533, A::neg(s.ad_value(1571)));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) {
            s.store_offset_ad(1532, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1559), s.ad_value(1549))), (-1.0)), s.ad_value(1533)), 4.0), A::mul(s.ad_value(1558), s.ad_value(226))), 1.0);
        }

        s.v[1633] = if (s.v[1532] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1633] != 0.0)) {
            s.store_scalar(1532, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) {
            s.store_add_ad_rhs(1574, 1559, A::mul(A::scale(A::mul(s.ad_value(1558), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1532)))));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) {
            s.store_mul_ad_rhs(1571, 225, A::add(s.ad_value(1574), s.ad_value(1549)));
        }

        s.v[1634] = if (s.v[1571] < 3.0) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1634] != 0.0)) {
            s.store_scalar(1575, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1634] != 0.0)) {
            s.store_scalar(1576, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1634] != 0.0)) {
            s.store_offset_ad(1577, A::div_from_scalar(1.0, A::mul(s.ad_value(225), s.ad_value(1557))), (1.0 / 1.414213562373095));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1634] != 0.0)) {
            s.store_div_ad_lhs(1578, A::neg(A::add(s.ad_value(1559), s.ad_value(1549))), 1557);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1634] != 0.0)) {
            s.store_add_ad(1579, A::sub(A::div(A::mul(A::square(s.ad_value(1576)), s.ad_value(1576)), A::mul(A::mul(A::scale(s.ad_value(1575), 27.0), s.ad_value(1575)), s.ad_value(1575))), A::div(A::mul(s.ad_value(1576), s.ad_value(1577)), A::mul(A::scale(s.ad_value(1575), 6.0), s.ad_value(1575)))), A::div(s.ad_value(1578), A::scale(s.ad_value(1575), 2.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1634] != 0.0)) {
            s.store_div_ad(1580, A::sub(A::mul(A::scale(s.ad_value(1575), 3.0), s.ad_value(1577)), A::square(s.ad_value(1576))), A::mul(A::scale(s.ad_value(1575), 9.0), s.ad_value(1575)));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1634] != 0.0)) {
            s.store_sqrt_ad(1528, A::add(A::square(s.ad_value(1579)), A::mul(A::square(s.ad_value(1580)), s.ad_value(1580))));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1634] != 0.0)) {
            s.store_powf_ad(1581, A::sub(s.ad_value(1528), s.ad_value(1579)), 0.3333333333333333);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1634] != 0.0)) {
            s.store_neg_ad(1582, A::powf(A::add(s.ad_value(1579), s.ad_value(1528)), 0.3333333333333333));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1634] != 0.0)) {
            s.store_sub_ad(1532, A::add(s.ad_value(1581), s.ad_value(1582)), A::div(s.ad_value(1576), A::scale(s.ad_value(1575), 3.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1634] != 0.0)) {
            s.store_sub_ad_lhs(1574, A::mul(s.ad_value(1532), s.ad_value(227)), 1549);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1634] != 0.0)) {
            s.store_mul_ad_rhs(1571, 225, A::add(s.ad_value(1574), s.ad_value(1549)));
        }

        s.v[1635] = if (p.p41 > 0.0) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_offset_ad(1583, A::add(s.ad_value(1559), s.ad_value(1549)), 0.1);
        }

    }

    pub(super) fn stamp_transient_block_27(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_offset_ad(1590, A::exp(A::mul(s.ad_value(225), A::neg(s.ad_value(1549)))), 1e-50);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_scale(1523, 230, 1.0 / (s.v[69]));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_square(1584, 1523);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_mul(1585, 1584, 1590);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_mul(1523, 226, 1558);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_mul(1586, 225, 1583);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_add_ad(1587, A::sub(A::ln(A::add(A::mul(s.ad_value(1585), s.ad_value(1523)), A::square(s.ad_value(1586)))), A::ln(A::mul(s.ad_value(1584), s.ad_value(1523)))), A::mul(s.ad_value(225), s.ad_value(1549)));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_offset_ad(44, A::sub(s.ad_value(1586), s.ad_value(1587)), (-1.0));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_scale(45, 1586, 4.0);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_scale_ad(1524, A::offset(A::div(s.ad_value(44), s.ad_value(45)), 1.0), 0.5);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_scale_ad(1525, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(44), 2.0), s.ad_value(45))), 0.5);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_sub_ad_rhs(1587, 1586, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_sub(1586, 1586, 1587);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_add_ad_rhs(1586, 1586, A::scale(s.ad_value(225), 0.1));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_add_ad(1588, A::sub(A::ln(A::add(A::mul(s.ad_value(1585), s.ad_value(1523)), A::square(s.ad_value(1586)))), A::ln(A::mul(s.ad_value(1584), s.ad_value(1523)))), A::mul(s.ad_value(225), s.ad_value(1549)));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1635] != 0.0)) {
            s.copy_ad(1589, 1571);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_offset_ad(44, A::sub(s.ad_value(1588), s.ad_value(1589)), (-(0.0008 * 75.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_scale(45, 1588, (4.0 * (0.0008 * 75.0)));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_scale_ad(1524, A::offset(A::div(s.ad_value(44), s.ad_value(45)), 1.0), 0.5);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_scale_ad(1525, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(44), ((2.0 * 0.0008) * 75.0)), s.ad_value(45))), 0.5);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1635] != 0.0)) {
            s.store_sub_ad_rhs(1571, 1588, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) {
            s.store_sub_ad_lhs(1573, A::div(s.ad_value(1571), s.ad_value(225)), 1549);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) {
            s.store_add_ad(1524, A::offset(s.ad_value(1571), (-1.0)), A::exp(A::neg(s.ad_value(1571))));
        }

        s.v[1636] = if (s.v[1524] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1636] != 0.0)) {
            s.store_scalar(1524, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) {
            s.store_sqrt(1525, 1524);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) {
            s.store_mul(458, 1556, 1525);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) {
            s.store_scaled_sub(459, 1559, 1573, s.v[1534]);
        }

        s.v[1637] = if (p.p41 == 1.0) { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) {
            s.store_exp_ad(1590, A::mul(s.ad_value(225), A::neg(s.ad_value(1549))));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) {
            s.store_scale(1523, 230, 1.0 / (s.v[69]));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) {
            s.store_square(1584, 1523);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) {
            s.store_mul(1599, 1584, 1590);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) {
            s.store_scalar(1546, 0.0);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) {
            s.store_scalar(1593, 0.0);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) {
            s.store_scalar(1597, 0.0);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) {
            s.store_scalar(167, 1.0);
        }

        let mut assign31390_loop_guard: usize = 0;
        while {
            let assign31390_cond_e45523: f64 = (2.0 * 20.0);
            let assign31390_cond_e45525: f64 = (assign31390_cond_e45523 + 1.0);
            let assign31390_cond_e45527: f64 = if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) && (s.v[167] <= assign31390_cond_e45525)) { 1.0 } else { 0.0 };
            assign31390_cond_e45527 != 0.0
        } {
            assign31390_loop_guard += 1;
            assert!(assign31390_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) {
                s.store_scalar(1595, 0.0);
            }
            if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) {
                s.store_mul_ad_rhs(1571, 225, A::add(s.ad_value(1573), s.ad_value(1549)));
            }
            s.v[1638] = if (s.v[1571] < 5.0) { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) && (s.v[1638] != 0.0)) {
                s.store_mul_ad(1591, A::mul(A::square(s.ad_value(1571)), s.ad_value(1571)), A::offset(A::mul(s.ad_value(1571), A::offset(A::scale(s.ad_value(1571), 0.006115288895133179), (-0.07053654284009761))), 0.29693154855771));
            }
            if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) && (s.v[1638] != 0.0)) {
                s.store_mul_ad(1592, A::square(s.ad_value(1571)), A::offset(A::mul(s.ad_value(1571), A::offset(A::scale(s.ad_value(1571), (5.0 * 0.006115288895133179)), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
            }
            if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) && (s.v[1638] != 0.0)) {
                s.store_mul_ad_lhs(1593, A::mul(s.ad_value(1599), s.ad_value(1591)), 1591);
            }
            if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) && (s.v[1638] != 0.0)) {
                s.store_mul_ad_lhs(1594, A::mul(A::scale(A::mul(s.ad_value(1599), s.ad_value(225)), 2.0), s.ad_value(1591)), 1592);
            }
            if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) && (s.v[1638] != 0.0)) {
                s.store_mul_ad_rhs(1595, 1571, A::offset(A::mul(s.ad_value(1571), A::offset(A::mul(s.ad_value(1571), A::offset(A::mul(s.ad_value(1571), A::offset(A::scale(s.ad_value(1571), 6.36964918866352e-5), (-0.00163730162779191))), 0.0178800506338833)), (-0.117851130197758))), 0.707106781186548));
            }
            if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) && (s.v[1638] != 0.0)) {
                s.store_offset_ad(1596, A::mul(s.ad_value(1571), A::offset(A::mul(s.ad_value(1571), A::offset(A::mul(s.ad_value(1571), A::offset(A::scale(s.ad_value(1571), (5.0 * 6.36964918866352e-5)), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833))), (2.0 * (-0.117851130197758)))), 0.707106781186548);
            }
            if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) && (s.v[1638] != 0.0)) {
                s.store_sqrt_ad(1597, A::offset(A::add(A::square(s.ad_value(1595)), s.ad_value(1593)), 1e-50));
            }
            if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) && (s.v[1638] != 0.0)) {
                s.store_div_ad(1598, A::add(A::mul(A::scale(A::mul(s.ad_value(225), s.ad_value(1596)), 2.0), s.ad_value(1595)), s.ad_value(1594)), A::scale(s.ad_value(1597), 2.0));
            }
            s.v[1639] = if (s.v[1571] < 80.0) { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) && (!(s.v[1638] != 0.0))) && (s.v[1639] != 0.0)) {
                s.store_exp(243, 1571);
            }
            if (((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) && (!(s.v[1638] != 0.0))) && (s.v[1639] != 0.0)) {
                s.store_mul_ad_rhs(1593, 1599, A::offset(s.ad_value(243), (-1.0)));
            }
            if (((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) && (!(s.v[1638] != 0.0))) && (s.v[1639] != 0.0)) {
                s.store_mul_ad_lhs(1594, A::mul(s.ad_value(1599), s.ad_value(225)), 243);
            }
            if (((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) && (!(s.v[1638] != 0.0))) && (!(s.v[1639] != 0.0))) {
                s.store_exp_ad(1600, A::mul(s.ad_value(225), s.ad_value(1573)));
            }
            if (((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) && (!(s.v[1638] != 0.0))) && (!(s.v[1639] != 0.0))) {
                s.store_mul_ad_rhs(1593, 1584, A::sub(s.ad_value(1600), s.ad_value(1590)));
            }
            if (((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) && (!(s.v[1638] != 0.0))) && (!(s.v[1639] != 0.0))) {
                s.store_mul_ad_lhs(1594, A::mul(s.ad_value(1584), s.ad_value(225)), 1600);
            }
            if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) && (!(s.v[1638] != 0.0))) {
                s.store_sqrt_ad(1597, A::add(A::offset(s.ad_value(1571), (-1.0)), s.ad_value(1593)));
            }
            if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) && (!(s.v[1638] != 0.0))) {
                s.store_scale_ad(1598, A::div(A::add(s.ad_value(225), s.ad_value(1594)), s.ad_value(1597)), 0.5);
            }
            if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) {
                s.store_sub_ad(1601, A::sub(s.ad_value(1559), s.ad_value(1573)), A::mul(s.ad_value(1557), s.ad_value(1597)));
            }
            if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) {
                s.store_sub_from_scalar_ad(1602, (-1.0), A::mul(s.ad_value(1557), s.ad_value(1598)));
            }
            s.v[1640] = if (s.v[1546] == 1.0) { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) && (s.v[1640] != 0.0)) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) && (!(s.v[1640] != 0.0))) {
                s.store_div_ad_lhs(494, A::neg(s.ad_value(1601)), 1602);
            }
            if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) && (!(s.v[1640] != 0.0))) {
                s.store_scale_ad(1603, A::offset({
                    if (1.0 >= ((s.v[1573]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1573))
                    }
                }, 1.0), (0.5 * 0.1));
            }
            s.v[1641] = if (((s.v[494]) as f64).abs() > s.v[1603]) { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) && (!(s.v[1640] != 0.0))) && (s.v[1641] != 0.0)) {
                s.store_scale(494, 1603, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) && (!(s.v[1640] != 0.0))) {
                s.store_add(1573, 1573, 494);
            }
            s.v[1642] = if ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1601]) as f64).abs() <= 1e-8)) { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) && (!(s.v[1640] != 0.0))) && (s.v[1642] != 0.0)) {
                s.store_scalar(1546, 1.0);
            }
            if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.v[1644] = if (s.v[1571] < 5.0) { 1.0 } else { 0.0 };

        if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) && (s.v[1644] != 0.0)) {
            s.store_offset_ad(1604, A::square(s.ad_value(1595)), (10.0 * 2.220446049250313e-16));
        }

        if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) && (s.v[1644] != 0.0)) {
            s.store_offset(1605, 1595, (10.0 * 2.220446049250313e-16));
        }

        if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) && (!(s.v[1644] != 0.0))) {
            s.store_offset(1604, 1571, (-1.0));
        }

        if ((((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) && (!(s.v[1644] != 0.0))) {
            s.store_sqrt(1605, 1604);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) {
            s.store_mul(458, 1556, 1605);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) {
            s.store_div_from_scalar_ad(1524, 1.0, A::add(s.ad_value(1597), s.ad_value(1605)));
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) {
            s.store_mul_ad_lhs(460, A::mul(s.ad_value(1556), s.ad_value(1593)), 1524);
        }

        if (((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (!(s.v[1630] != 0.0))) && (s.v[1637] != 0.0)) {
            s.store_add(459, 458, 460);
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) {
            s.store_sub(460, 459, 458);
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) {
            s.store_ad(1527, &{
                if (p.p43 == 1.0) {
                    A::mul(s.ad_value(287), s.ad_value(1536))
                } else {
                    A::mul(s.ad_value(108), s.ad_value(1536))
                }
            });
        }

        s.v[1646] = if (((s.v[1542] != 0.0) && (p.p43 == 0.0)) || ((s.v[1540] != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1646] != 0.0)) {
            s.store_mul(455, 1527, 459);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1646] != 0.0)) {
            s.store_mul(457, 1527, 458);
        }

        s.v[1647] = if (((s.v[1543] != 0.0) && (p.p43 == 0.0)) || ((s.v[1541] != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1647] != 0.0)) {
            s.store_mul(454, 1527, 459);
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) && (s.v[1647] != 0.0)) {
            s.store_mul(456, 1527, 458);
        }

        if ((p.p24 != 0.0) && (s.v[1606] != 0.0)) {
            s.store_add_ad(266, A::scale(s.ad_value(462), s.v[566]), A::scale(s.ad_value(461), s.v[565]));
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (s.v[266] != 0.0)) {
            s.store_add_ad(269, A::scale(s.ad_value(462), p.p170), A::scale(s.ad_value(461), p.p169));
        }

        s.v[1648] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (s.v[266] != 0.0)) && (s.v[1648] != 0.0)) {
            s.store_add_ad(1524, A::mul(s.ad_value(462), s.ad_value(287)), A::mul(s.ad_value(461), s.ad_value(288)));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (s.v[266] != 0.0)) && (s.v[1648] != 0.0)) {
            s.store_mul_ad_rhs(269, 269, A::neg(s.ad_value(1524)));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (s.v[266] != 0.0)) && (!(s.v[1648] != 0.0))) {
            s.store_mul_ad_rhs(269, 269, A::neg(s.ad_value(108)));
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (s.v[266] != 0.0)) {
            s.store_add_ad_rhs(268, 268, A::mul(A::neg(s.ad_value(269)), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if ((p.p24 != 0.0) && (s.v[1606] != 0.0)) {
            s.store_add_ad(266, A::scale(s.ad_value(461), s.v[566]), A::scale(s.ad_value(462), s.v[565]));
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (s.v[266] != 0.0)) {
            s.store_add_ad(270, A::scale(s.ad_value(461), p.p170), A::scale(s.ad_value(462), p.p169));
        }

        s.v[1649] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (s.v[266] != 0.0)) && (s.v[1649] != 0.0)) {
            s.store_add_ad(1524, A::mul(s.ad_value(461), s.ad_value(287)), A::mul(s.ad_value(462), s.ad_value(288)));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (s.v[266] != 0.0)) && (s.v[1649] != 0.0)) {
            s.store_mul_ad_rhs(270, 270, A::neg(s.ad_value(1524)));
        }

        if ((((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (s.v[266] != 0.0)) && (!(s.v[1649] != 0.0))) {
            s.store_mul_ad_rhs(270, 270, A::neg(s.ad_value(108)));
        }

        if (((p.p24 != 0.0) && (s.v[1606] != 0.0)) && (s.v[266] != 0.0)) {
            s.store_add_ad_rhs(267, 267, A::mul(A::neg(s.ad_value(270)), s.ad_value(158)));
        }

        s.v[1650] = if (((s.v[613] == 1.0) && (!(s.v[565] != 0.0))) || ((s.v[613] != 1.0) && (!(s.v[566] != 0.0)))) { 1.0 } else { 0.0 };

        s.v[1651] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (!(s.v[1606] != 0.0))) && (s.v[1650] != 0.0)) && (s.v[1651] != 0.0)) {
            s.store_scale(269, 288, ((-s.v[1534]) * p.p188));
        }

        if ((((p.p24 != 0.0) && (!(s.v[1606] != 0.0))) && (s.v[1650] != 0.0)) && (!(s.v[1651] != 0.0))) {
            s.store_scale(269, 108, ((-s.v[1534]) * p.p188));
        }

        if (((p.p24 != 0.0) && (!(s.v[1606] != 0.0))) && (!(s.v[1650] != 0.0))) {
            s.store_add_ad(269, A::scale(s.ad_value(462), p.p170), A::scale(s.ad_value(461), p.p169));
        }

        s.v[1652] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (!(s.v[1606] != 0.0))) && (!(s.v[1650] != 0.0))) && (s.v[1652] != 0.0)) {
            s.store_add_ad(1524, A::mul(s.ad_value(462), s.ad_value(287)), A::mul(s.ad_value(461), s.ad_value(288)));
        }

        if ((((p.p24 != 0.0) && (!(s.v[1606] != 0.0))) && (!(s.v[1650] != 0.0))) && (s.v[1652] != 0.0)) {
            s.store_mul_ad_rhs(269, 269, A::neg(s.ad_value(1524)));
        }

        if ((((p.p24 != 0.0) && (!(s.v[1606] != 0.0))) && (!(s.v[1650] != 0.0))) && (!(s.v[1652] != 0.0))) {
            s.store_mul_ad_rhs(269, 269, A::neg(s.ad_value(108)));
        }

        if ((p.p24 != 0.0) && (!(s.v[1606] != 0.0))) {
            s.store_mul_ad(268, A::neg(s.ad_value(269)), A::sub(s.ad_value(158), s.ad_value(157)));
        }

        s.v[1653] = if (((s.v[613] == 1.0) && (!(s.v[566] != 0.0))) || ((s.v[613] != 1.0) && (!(s.v[565] != 0.0)))) { 1.0 } else { 0.0 };

        s.v[1654] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (!(s.v[1606] != 0.0))) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) {
            s.store_scale(270, 287, ((-s.v[1534]) * p.p188));
        }

        if ((((p.p24 != 0.0) && (!(s.v[1606] != 0.0))) && (s.v[1653] != 0.0)) && (!(s.v[1654] != 0.0))) {
            s.store_scale(270, 108, ((-s.v[1534]) * p.p188));
        }

        if (((p.p24 != 0.0) && (!(s.v[1606] != 0.0))) && (!(s.v[1653] != 0.0))) {
            s.store_add_ad(270, A::scale(s.ad_value(461), p.p170), A::scale(s.ad_value(462), p.p169));
        }

        s.v[1655] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (!(s.v[1606] != 0.0))) && (!(s.v[1653] != 0.0))) && (s.v[1655] != 0.0)) {
            s.store_add_ad(1524, A::mul(s.ad_value(461), s.ad_value(287)), A::mul(s.ad_value(462), s.ad_value(288)));
        }

        if ((((p.p24 != 0.0) && (!(s.v[1606] != 0.0))) && (!(s.v[1653] != 0.0))) && (s.v[1655] != 0.0)) {
            s.store_mul_ad_rhs(270, 270, A::neg(s.ad_value(1524)));
        }

        if ((((p.p24 != 0.0) && (!(s.v[1606] != 0.0))) && (!(s.v[1653] != 0.0))) && (!(s.v[1655] != 0.0))) {
            s.store_mul_ad_rhs(270, 270, A::neg(s.ad_value(108)));
        }

        if ((p.p24 != 0.0) && (!(s.v[1606] != 0.0))) {
            s.store_mul_ad_lhs(267, A::neg(s.ad_value(270)), 158);
        }

        s.v[1656] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1656] != 0.0) {
            s.copy_ad(1672, 590);
        }

        if (s.v[1656] != 0.0) {
            s.copy_ad(1673, 591);
        }

        if (s.v[1656] != 0.0) {
            s.store_scale_ad(1674, A::exp(A::scale(A::add(A::sub_from_scalar((s.v[87] * s.v[114]), A::mul(s.ad_value(237), s.ad_value(225))), A::scale(A::ln(A::scale(s.ad_value(429), 1.0 / (s.v[81]))), p.p175)), 1.0 / (p.p174))), p.p173);
        }

        if (s.v[1656] != 0.0) {
            s.store_scale_ad(1675, A::exp(A::scale(A::add(A::sub_from_scalar((s.v[87] * s.v[114]), A::mul(s.ad_value(237), s.ad_value(225))), A::scale(A::ln(A::scale(s.ad_value(429), 1.0 / (s.v[81]))), p.p176)), 1.0 / (p.p174))), p.p173);
        }

        if (s.v[1656] != 0.0) {
            s.store_mul_ad_lhs(1679, A::scale(s.ad_value(286), p.p237), 1674);
        }

        if (s.v[1656] != 0.0) {
            s.store_mul_ad_lhs(1681, A::scale(s.ad_value(286), p.p237), 1675);
        }

        if (s.v[1656] != 0.0) {
            s.store_mul_ad_lhs(1680, A::scale(s.ad_value(285), p.p237), 1674);
        }

        if (s.v[1656] != 0.0) {
            s.store_mul_ad_lhs(1682, A::scale(s.ad_value(285), p.p237), 1675);
        }

        if (s.v[1656] != 0.0) {
            s.store_scale(1658, 429, 1.0 / (s.v[81]));
        }

        if (s.v[1656] != 0.0) {
            s.store_offset(1659, 1679, 1e-50);
        }

        if (s.v[1656] != 0.0) {
            s.store_scale_ad(1677, A::div_from_scalar(p.p174, s.ad_value(225)), 0.0);
        }

    }

    pub(super) fn stamp_transient_block_28(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1656] != 0.0) {
            s.store_scale_ad(1678, A::div_from_scalar(p.p174, s.ad_value(225)), 0.0);
        }

        if (s.v[1656] != 0.0) {
            s.store_scale(1676, 227, p.p174);
        }

        s.v[1685] = if (s.v[1672] < s.v[1677]) { 1.0 } else { 0.0 };

        if ((s.v[1656] != 0.0) && (s.v[1685] != 0.0)) {
            s.store_exp_ad(1658, A::div(s.ad_value(1672), s.ad_value(1676)));
        }

        if ((s.v[1656] != 0.0) && (s.v[1685] != 0.0)) {
            s.store_mul_ad_rhs(282, 1679, A::offset(s.ad_value(1658), (-1.0)));
        }

        if ((s.v[1656] != 0.0) && (!(s.v[1685] != 0.0))) {
            s.store_exp_ad(1658, A::div(s.ad_value(1677), s.ad_value(1676)));
        }

        if ((s.v[1656] != 0.0) && (!(s.v[1685] != 0.0))) {
            s.store_add_ad(282, A::mul(s.ad_value(1679), A::offset(s.ad_value(1658), (-1.0))), A::mul(A::mul(A::div(s.ad_value(1679), s.ad_value(1676)), s.ad_value(1658)), A::sub(s.ad_value(1672), s.ad_value(1677))));
        }

        if (s.v[1656] != 0.0) {
            s.store_add_ad_rhs(282, 282, A::mul(A::scale(s.ad_value(1672), p.p178), s.ad_value(1681)));
        }

        s.v[1686] = if (s.v[1673] < s.v[1678]) { 1.0 } else { 0.0 };

        if ((s.v[1656] != 0.0) && (s.v[1686] != 0.0)) {
            s.store_exp_ad(1658, A::div(s.ad_value(1673), s.ad_value(1676)));
        }

        if ((s.v[1656] != 0.0) && (s.v[1686] != 0.0)) {
            s.store_mul_ad_rhs(281, 1680, A::offset(s.ad_value(1658), (-1.0)));
        }

        if ((s.v[1656] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_exp_ad(1658, A::div(s.ad_value(1678), s.ad_value(1676)));
        }

        if ((s.v[1656] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_add_ad(281, A::mul(s.ad_value(1680), A::offset(s.ad_value(1658), (-1.0))), A::mul(A::mul(A::div(s.ad_value(1680), s.ad_value(1676)), s.ad_value(1658)), A::sub(s.ad_value(1673), s.ad_value(1678))));
        }

        if (s.v[1656] != 0.0) {
            s.store_add_ad_rhs(281, 281, A::mul(A::scale(s.ad_value(1673), p.p178), s.ad_value(1682)));
        }

        if (s.v[1656] != 0.0) {
            s.store_add_ad_rhs(282, 282, A::scale(s.ad_value(1672), s.v[142]));
        }

        if (s.v[1656] != 0.0) {
            s.store_add_ad_rhs(281, 281, A::scale(s.ad_value(1673), s.v[142]));
        }

        if (s.v[1656] != 0.0) {
            s.store_scalar(1666, (p.p179 * p.p2));
        }

        if (s.v[1656] != 0.0) {
            s.store_scalar(1667, (p.p179 * p.p3));
        }

        if (s.v[1656] != 0.0) {
            s.store_scalar(1665, (p.p237 - p.p238));
        }

        s.v[1687] = if (s.v[1665] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1656] != 0.0) && (s.v[1687] != 0.0)) {
            s.store_scalar(1666, 0.0);
        }

        if ((s.v[1656] != 0.0) && (s.v[1687] != 0.0)) {
            s.store_scalar(1667, 0.0);
        }

        s.v[1688] = if (p.p5 > s.v[287]) { 1.0 } else { 0.0 };

        if ((s.v[1656] != 0.0) && (s.v[1688] != 0.0)) {
            s.store_scale_ad(1669, A::sub_from_scalar(p.p5, s.ad_value(287)), p.p180);
        }

        if ((s.v[1656] != 0.0) && (s.v[1688] != 0.0)) {
            s.store_scale(1671, 287, p.p181);
        }

        s.v[1689] = if (s.v[1673] < 0.0) { 1.0 } else { 0.0 };

        s.v[1690] = if (s.v[1667] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1656] != 0.0) && (s.v[1688] != 0.0)) && (s.v[1689] != 0.0)) && (s.v[1690] != 0.0)) {
            s.store_sub_from_scalar_ad(1683, 1.0, A::scale(s.ad_value(1673), 1.0 / (p.p185)));
        }

        s.v[1691] = if (p.p182 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1656] != 0.0) && (s.v[1688] != 0.0)) && (s.v[1689] != 0.0)) && (s.v[1690] != 0.0)) && (s.v[1691] != 0.0)) {
            s.store_div_from_scalar_ad(1684, 1.0, A::sqrt(s.ad_value(1683)));
        }

        if (((((s.v[1656] != 0.0) && (s.v[1688] != 0.0)) && (s.v[1689] != 0.0)) && (s.v[1690] != 0.0)) && (!(s.v[1691] != 0.0))) {
            s.store_powf(1684, 1683, (-p.p182));
        }

        if ((((s.v[1656] != 0.0) && (s.v[1688] != 0.0)) && (s.v[1689] != 0.0)) && (s.v[1690] != 0.0)) {
            s.store_scale_ad(283, A::mul(A::scale(s.ad_value(1667), p.p185), A::sub_from_scalar(1.0, A::mul(s.ad_value(1683), s.ad_value(1684)))), 1.0 / ((1.0 - p.p182)));
        }

        if ((((s.v[1656] != 0.0) && (s.v[1688] != 0.0)) && (s.v[1689] != 0.0)) && (!(s.v[1690] != 0.0))) {
            s.store_scalar(283, 0.0);
        }

        s.v[1692] = if (s.v[1669] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1656] != 0.0) && (s.v[1688] != 0.0)) && (s.v[1689] != 0.0)) && (s.v[1692] != 0.0)) {
            s.store_sub_from_scalar_ad(1683, 1.0, A::scale(s.ad_value(1673), 1.0 / (p.p186)));
        }

        s.v[1693] = if (p.p183 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1656] != 0.0) && (s.v[1688] != 0.0)) && (s.v[1689] != 0.0)) && (s.v[1692] != 0.0)) && (s.v[1693] != 0.0)) {
            s.store_div_from_scalar_ad(1684, 1.0, A::sqrt(s.ad_value(1683)));
        }

        if (((((s.v[1656] != 0.0) && (s.v[1688] != 0.0)) && (s.v[1689] != 0.0)) && (s.v[1692] != 0.0)) && (!(s.v[1693] != 0.0))) {
            s.store_powf(1684, 1683, (-p.p183));
        }

        if ((((s.v[1656] != 0.0) && (s.v[1688] != 0.0)) && (s.v[1689] != 0.0)) && (s.v[1692] != 0.0)) {
            s.store_add_ad_rhs(283, 283, A::scale(A::mul(A::scale(s.ad_value(1669), p.p186), A::sub_from_scalar(1.0, A::mul(s.ad_value(1683), s.ad_value(1684)))), 1.0 / ((1.0 - p.p183))));
        }

        s.v[1694] = if (s.v[1671] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1656] != 0.0) && (s.v[1688] != 0.0)) && (s.v[1689] != 0.0)) && (s.v[1694] != 0.0)) {
            s.store_sub_from_scalar_ad(1683, 1.0, A::scale(s.ad_value(1673), 1.0 / (p.p187)));
        }

        s.v[1695] = if (p.p184 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1656] != 0.0) && (s.v[1688] != 0.0)) && (s.v[1689] != 0.0)) && (s.v[1694] != 0.0)) && (s.v[1695] != 0.0)) {
            s.store_div_from_scalar_ad(1684, 1.0, A::sqrt(s.ad_value(1683)));
        }

        if (((((s.v[1656] != 0.0) && (s.v[1688] != 0.0)) && (s.v[1689] != 0.0)) && (s.v[1694] != 0.0)) && (!(s.v[1695] != 0.0))) {
            s.store_powf(1684, 1683, (-p.p184));
        }

        if ((((s.v[1656] != 0.0) && (s.v[1688] != 0.0)) && (s.v[1689] != 0.0)) && (s.v[1694] != 0.0)) {
            s.store_add_ad_rhs(283, 283, A::scale(A::mul(A::scale(s.ad_value(1671), p.p187), A::sub_from_scalar(1.0, A::mul(s.ad_value(1683), s.ad_value(1684)))), 1.0 / ((1.0 - p.p184))));
        }

        if (((s.v[1656] != 0.0) && (s.v[1688] != 0.0)) && (!(s.v[1689] != 0.0))) {
            s.store_add_ad_lhs(1658, A::add(s.ad_value(1667), s.ad_value(1669)), 1671);
        }

        if (((s.v[1656] != 0.0) && (s.v[1688] != 0.0)) && (!(s.v[1689] != 0.0))) {
            s.store_add_ad(1659, A::add(A::scale(s.ad_value(1667), (p.p182 * 1.0 / (p.p185))), A::scale(s.ad_value(1669), (p.p183 * 1.0 / (p.p186)))), A::scale(s.ad_value(1671), (p.p184 * 1.0 / (p.p187))));
        }

        if (((s.v[1656] != 0.0) && (s.v[1688] != 0.0)) && (!(s.v[1689] != 0.0))) {
            s.store_mul_ad_rhs(283, 1673, A::add(s.ad_value(1658), A::mul(A::scale(s.ad_value(1673), 0.5), s.ad_value(1659))));
        }

        if ((s.v[1656] != 0.0) && (!(s.v[1688] != 0.0))) {
            s.store_scalar(1671, (p.p181 * p.p5));
        }

        s.v[1696] = if (s.v[1673] < 0.0) { 1.0 } else { 0.0 };

        s.v[1697] = if (s.v[1667] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1656] != 0.0) && (!(s.v[1688] != 0.0))) && (s.v[1696] != 0.0)) && (s.v[1697] != 0.0)) {
            s.store_sub_from_scalar_ad(1683, 1.0, A::scale(s.ad_value(1673), 1.0 / (p.p185)));
        }

        s.v[1698] = if (p.p182 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1656] != 0.0) && (!(s.v[1688] != 0.0))) && (s.v[1696] != 0.0)) && (s.v[1697] != 0.0)) && (s.v[1698] != 0.0)) {
            s.store_div_from_scalar_ad(1684, 1.0, A::sqrt(s.ad_value(1683)));
        }

        if (((((s.v[1656] != 0.0) && (!(s.v[1688] != 0.0))) && (s.v[1696] != 0.0)) && (s.v[1697] != 0.0)) && (!(s.v[1698] != 0.0))) {
            s.store_powf(1684, 1683, (-p.p182));
        }

        if ((((s.v[1656] != 0.0) && (!(s.v[1688] != 0.0))) && (s.v[1696] != 0.0)) && (s.v[1697] != 0.0)) {
            s.store_scale_ad(283, A::mul(A::scale(s.ad_value(1667), p.p185), A::sub_from_scalar(1.0, A::mul(s.ad_value(1683), s.ad_value(1684)))), 1.0 / ((1.0 - p.p182)));
        }

        if ((((s.v[1656] != 0.0) && (!(s.v[1688] != 0.0))) && (s.v[1696] != 0.0)) && (!(s.v[1697] != 0.0))) {
            s.store_scalar(283, 0.0);
        }

        s.v[1699] = if (s.v[1671] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1656] != 0.0) && (!(s.v[1688] != 0.0))) && (s.v[1696] != 0.0)) && (s.v[1699] != 0.0)) {
            s.store_sub_from_scalar_ad(1683, 1.0, A::scale(s.ad_value(1673), 1.0 / (p.p187)));
        }

        s.v[1700] = if (p.p184 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1656] != 0.0) && (!(s.v[1688] != 0.0))) && (s.v[1696] != 0.0)) && (s.v[1699] != 0.0)) && (s.v[1700] != 0.0)) {
            s.store_div_from_scalar_ad(1684, 1.0, A::sqrt(s.ad_value(1683)));
        }

        if (((((s.v[1656] != 0.0) && (!(s.v[1688] != 0.0))) && (s.v[1696] != 0.0)) && (s.v[1699] != 0.0)) && (!(s.v[1700] != 0.0))) {
            s.store_powf(1684, 1683, (-p.p184));
        }

        if ((((s.v[1656] != 0.0) && (!(s.v[1688] != 0.0))) && (s.v[1696] != 0.0)) && (s.v[1699] != 0.0)) {
            s.store_add_ad_rhs(283, 283, A::scale(A::mul(A::scale(s.ad_value(1671), p.p187), A::sub_from_scalar(1.0, A::mul(s.ad_value(1683), s.ad_value(1684)))), 1.0 / ((1.0 - p.p184))));
        }

        if (((s.v[1656] != 0.0) && (!(s.v[1688] != 0.0))) && (!(s.v[1696] != 0.0))) {
            s.store_add(1658, 1667, 1671);
        }

        if (((s.v[1656] != 0.0) && (!(s.v[1688] != 0.0))) && (!(s.v[1696] != 0.0))) {
            s.store_add_ad(1659, A::scale(s.ad_value(1667), (p.p182 * 1.0 / (p.p185))), A::scale(s.ad_value(1671), (p.p184 * 1.0 / (p.p187))));
        }

        if (((s.v[1656] != 0.0) && (!(s.v[1688] != 0.0))) && (!(s.v[1696] != 0.0))) {
            s.store_mul_ad_rhs(283, 1673, A::add(s.ad_value(1658), A::mul(A::scale(s.ad_value(1673), 0.5), s.ad_value(1659))));
        }

        s.v[1701] = if (p.p4 > s.v[288]) { 1.0 } else { 0.0 };

        if ((s.v[1656] != 0.0) && (s.v[1701] != 0.0)) {
            s.store_scale_ad(1668, A::sub_from_scalar(p.p4, s.ad_value(288)), p.p180);
        }

        if ((s.v[1656] != 0.0) && (s.v[1701] != 0.0)) {
            s.store_scale(1670, 288, p.p181);
        }

        s.v[1702] = if (s.v[1672] < 0.0) { 1.0 } else { 0.0 };

        s.v[1703] = if (s.v[1666] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1656] != 0.0) && (s.v[1701] != 0.0)) && (s.v[1702] != 0.0)) && (s.v[1703] != 0.0)) {
            s.store_sub_from_scalar_ad(1683, 1.0, A::scale(s.ad_value(1672), 1.0 / (p.p185)));
        }

        s.v[1704] = if (p.p182 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1656] != 0.0) && (s.v[1701] != 0.0)) && (s.v[1702] != 0.0)) && (s.v[1703] != 0.0)) && (s.v[1704] != 0.0)) {
            s.store_div_from_scalar_ad(1684, 1.0, A::sqrt(s.ad_value(1683)));
        }

        if (((((s.v[1656] != 0.0) && (s.v[1701] != 0.0)) && (s.v[1702] != 0.0)) && (s.v[1703] != 0.0)) && (!(s.v[1704] != 0.0))) {
            s.store_powf(1684, 1683, (-p.p182));
        }

        if ((((s.v[1656] != 0.0) && (s.v[1701] != 0.0)) && (s.v[1702] != 0.0)) && (s.v[1703] != 0.0)) {
            s.store_scale_ad(284, A::mul(A::scale(s.ad_value(1666), p.p185), A::sub_from_scalar(1.0, A::mul(s.ad_value(1683), s.ad_value(1684)))), 1.0 / ((1.0 - p.p182)));
        }

        if ((((s.v[1656] != 0.0) && (s.v[1701] != 0.0)) && (s.v[1702] != 0.0)) && (!(s.v[1703] != 0.0))) {
            s.store_scalar(284, 0.0);
        }

        s.v[1705] = if (s.v[1668] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1656] != 0.0) && (s.v[1701] != 0.0)) && (s.v[1702] != 0.0)) && (s.v[1705] != 0.0)) {
            s.store_sub_from_scalar_ad(1683, 1.0, A::scale(s.ad_value(1672), 1.0 / (p.p186)));
        }

        s.v[1706] = if (p.p183 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1656] != 0.0) && (s.v[1701] != 0.0)) && (s.v[1702] != 0.0)) && (s.v[1705] != 0.0)) && (s.v[1706] != 0.0)) {
            s.store_div_from_scalar_ad(1684, 1.0, A::sqrt(s.ad_value(1683)));
        }

        if (((((s.v[1656] != 0.0) && (s.v[1701] != 0.0)) && (s.v[1702] != 0.0)) && (s.v[1705] != 0.0)) && (!(s.v[1706] != 0.0))) {
            s.store_powf(1684, 1683, (-p.p183));
        }

        if ((((s.v[1656] != 0.0) && (s.v[1701] != 0.0)) && (s.v[1702] != 0.0)) && (s.v[1705] != 0.0)) {
            s.store_add_ad_rhs(284, 284, A::scale(A::mul(A::scale(s.ad_value(1668), p.p186), A::sub_from_scalar(1.0, A::mul(s.ad_value(1683), s.ad_value(1684)))), 1.0 / ((1.0 - p.p183))));
        }

        s.v[1707] = if (s.v[1670] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1656] != 0.0) && (s.v[1701] != 0.0)) && (s.v[1702] != 0.0)) && (s.v[1707] != 0.0)) {
            s.store_sub_from_scalar_ad(1683, 1.0, A::scale(s.ad_value(1672), 1.0 / (p.p187)));
        }

        s.v[1708] = if (p.p184 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1656] != 0.0) && (s.v[1701] != 0.0)) && (s.v[1702] != 0.0)) && (s.v[1707] != 0.0)) && (s.v[1708] != 0.0)) {
            s.store_div_from_scalar_ad(1684, 1.0, A::sqrt(s.ad_value(1683)));
        }

        if (((((s.v[1656] != 0.0) && (s.v[1701] != 0.0)) && (s.v[1702] != 0.0)) && (s.v[1707] != 0.0)) && (!(s.v[1708] != 0.0))) {
            s.store_powf(1684, 1683, (-p.p184));
        }

        if ((((s.v[1656] != 0.0) && (s.v[1701] != 0.0)) && (s.v[1702] != 0.0)) && (s.v[1707] != 0.0)) {
            s.store_add_ad_rhs(284, 284, A::scale(A::mul(A::scale(s.ad_value(1670), p.p187), A::sub_from_scalar(1.0, A::mul(s.ad_value(1683), s.ad_value(1684)))), 1.0 / ((1.0 - p.p184))));
        }

        if (((s.v[1656] != 0.0) && (s.v[1701] != 0.0)) && (!(s.v[1702] != 0.0))) {
            s.store_add_ad_lhs(1658, A::add(s.ad_value(1666), s.ad_value(1668)), 1670);
        }

        if (((s.v[1656] != 0.0) && (s.v[1701] != 0.0)) && (!(s.v[1702] != 0.0))) {
            s.store_add_ad(1659, A::add(A::scale(s.ad_value(1666), (p.p182 * 1.0 / (p.p185))), A::scale(s.ad_value(1668), (p.p183 * 1.0 / (p.p186)))), A::scale(s.ad_value(1670), (p.p184 * 1.0 / (p.p187))));
        }

        if (((s.v[1656] != 0.0) && (s.v[1701] != 0.0)) && (!(s.v[1702] != 0.0))) {
            s.store_mul_ad_rhs(284, 1672, A::add(s.ad_value(1658), A::mul(A::scale(s.ad_value(1672), 0.5), s.ad_value(1659))));
        }

        if ((s.v[1656] != 0.0) && (!(s.v[1701] != 0.0))) {
            s.store_scalar(1670, (p.p181 * p.p4));
        }

        s.v[1709] = if (s.v[1672] < 0.0) { 1.0 } else { 0.0 };

        s.v[1710] = if (s.v[1666] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1656] != 0.0) && (!(s.v[1701] != 0.0))) && (s.v[1709] != 0.0)) && (s.v[1710] != 0.0)) {
            s.store_sub_from_scalar_ad(1683, 1.0, A::scale(s.ad_value(1672), 1.0 / (p.p185)));
        }

        s.v[1711] = if (p.p182 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1656] != 0.0) && (!(s.v[1701] != 0.0))) && (s.v[1709] != 0.0)) && (s.v[1710] != 0.0)) && (s.v[1711] != 0.0)) {
            s.store_div_from_scalar_ad(1684, 1.0, A::sqrt(s.ad_value(1683)));
        }

        if (((((s.v[1656] != 0.0) && (!(s.v[1701] != 0.0))) && (s.v[1709] != 0.0)) && (s.v[1710] != 0.0)) && (!(s.v[1711] != 0.0))) {
            s.store_powf(1684, 1683, (-p.p182));
        }

        if ((((s.v[1656] != 0.0) && (!(s.v[1701] != 0.0))) && (s.v[1709] != 0.0)) && (s.v[1710] != 0.0)) {
            s.store_scale_ad(284, A::mul(A::scale(s.ad_value(1666), p.p185), A::sub_from_scalar(1.0, A::mul(s.ad_value(1683), s.ad_value(1684)))), 1.0 / ((1.0 - p.p182)));
        }

        if ((((s.v[1656] != 0.0) && (!(s.v[1701] != 0.0))) && (s.v[1709] != 0.0)) && (!(s.v[1710] != 0.0))) {
            s.store_scalar(284, 0.0);
        }

        s.v[1712] = if (s.v[1670] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1656] != 0.0) && (!(s.v[1701] != 0.0))) && (s.v[1709] != 0.0)) && (s.v[1712] != 0.0)) {
            s.store_sub_from_scalar_ad(1683, 1.0, A::scale(s.ad_value(1672), 1.0 / (p.p187)));
        }

        s.v[1713] = if (p.p184 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1656] != 0.0) && (!(s.v[1701] != 0.0))) && (s.v[1709] != 0.0)) && (s.v[1712] != 0.0)) && (s.v[1713] != 0.0)) {
            s.store_div_from_scalar_ad(1684, 1.0, A::sqrt(s.ad_value(1683)));
        }

        if (((((s.v[1656] != 0.0) && (!(s.v[1701] != 0.0))) && (s.v[1709] != 0.0)) && (s.v[1712] != 0.0)) && (!(s.v[1713] != 0.0))) {
            s.store_powf(1684, 1683, (-p.p184));
        }

        if ((((s.v[1656] != 0.0) && (!(s.v[1701] != 0.0))) && (s.v[1709] != 0.0)) && (s.v[1712] != 0.0)) {
            s.store_add_ad_rhs(284, 284, A::scale(A::mul(A::scale(s.ad_value(1670), p.p187), A::sub_from_scalar(1.0, A::mul(s.ad_value(1683), s.ad_value(1684)))), 1.0 / ((1.0 - p.p184))));
        }

        if (((s.v[1656] != 0.0) && (!(s.v[1701] != 0.0))) && (!(s.v[1709] != 0.0))) {
            s.store_add(1658, 1666, 1670);
        }

        if (((s.v[1656] != 0.0) && (!(s.v[1701] != 0.0))) && (!(s.v[1709] != 0.0))) {
            s.store_add_ad(1659, A::scale(s.ad_value(1666), (p.p182 * 1.0 / (p.p185))), A::scale(s.ad_value(1670), (p.p184 * 1.0 / (p.p187))));
        }

        if (((s.v[1656] != 0.0) && (!(s.v[1701] != 0.0))) && (!(s.v[1709] != 0.0))) {
            s.store_mul_ad_rhs(284, 1672, A::add(s.ad_value(1658), A::mul(A::scale(s.ad_value(1672), 0.5), s.ad_value(1659))));
        }

        s.v[1714] = if (s.v[1667] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1656] != 0.0) && (s.v[1714] != 0.0)) {
            s.store_scale_ad(1661, A::mul(A::scale(s.ad_value(544), (-1.6021918e-19)), s.ad_value(1665)), p.p3);
        }

        if ((s.v[1656] != 0.0) && (s.v[1714] != 0.0)) {
            s.store_scale_ad(1663, A::neg(s.ad_value(1661)), 0.001);
        }

        if ((s.v[1656] != 0.0) && (s.v[1714] != 0.0)) {
            s.store_sub_ad_lhs(44, A::sub(A::neg(s.ad_value(1661)), A::neg(s.ad_value(283))), 1663);
        }

        if ((s.v[1656] != 0.0) && (s.v[1714] != 0.0)) {
            s.store_mul_ad_lhs(45, A::scale(A::neg(s.ad_value(1661)), 4.0), 1663);
        }

        if ((s.v[1656] != 0.0) && (s.v[1714] != 0.0)) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((s.v[1656] != 0.0) && (s.v[1714] != 0.0)) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((s.v[1656] != 0.0) && (s.v[1714] != 0.0)) {
            s.store_sub_ad(283, A::neg(s.ad_value(1661)), A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((s.v[1656] != 0.0) && (s.v[1714] != 0.0)) {
            s.store_scale(283, 283, (-1.0));
        }

        s.v[1715] = if (s.v[1666] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1656] != 0.0) && (s.v[1715] != 0.0)) {
            s.store_scale_ad(1662, A::mul(A::scale(s.ad_value(544), (-1.6021918e-19)), s.ad_value(1665)), p.p2);
        }

        if ((s.v[1656] != 0.0) && (s.v[1715] != 0.0)) {
            s.store_scale_ad(1664, A::neg(s.ad_value(1662)), 0.001);
        }

        if ((s.v[1656] != 0.0) && (s.v[1715] != 0.0)) {
            s.store_sub_ad_lhs(44, A::sub(A::neg(s.ad_value(1662)), A::neg(s.ad_value(284))), 1664);
        }

        if ((s.v[1656] != 0.0) && (s.v[1715] != 0.0)) {
            s.store_mul_ad_lhs(45, A::scale(A::neg(s.ad_value(1662)), 4.0), 1664);
        }

        if ((s.v[1656] != 0.0) && (s.v[1715] != 0.0)) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((s.v[1656] != 0.0) && (s.v[1715] != 0.0)) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((s.v[1656] != 0.0) && (s.v[1715] != 0.0)) {
            s.store_sub_ad(284, A::neg(s.ad_value(1662)), A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((s.v[1656] != 0.0) && (s.v[1715] != 0.0)) {
            s.store_scale(284, 284, (-1.0));
        }

        s.v[1721] = if (s.v[145] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[85] != 0.0) && (s.v[1721] != 0.0)) {
            s.store_scalar(1716, p.p233);
        }

        if ((s.v[85] != 0.0) && (s.v[1721] != 0.0)) {
            s.store_scalar(1717, p.p234);
        }

        if ((s.v[85] != 0.0) && (s.v[1721] != 0.0)) {
            s.copy_ad(1718, 441);
        }

        if ((s.v[85] != 0.0) && (s.v[1721] != 0.0)) {
            s.store_mul_ad_lhs(1719, A::mul(A::mul(s.ad_value(1716), s.ad_value(1717)), s.ad_value(1718)), 1718);
        }

        if ((s.v[85] != 0.0) && (s.v[1721] != 0.0)) {
            s.store_offset_ad(1720, A::add(A::mul(A::mul(s.ad_value(250), s.ad_value(192)), s.ad_value(1716)), A::mul(A::mul(s.ad_value(1717), s.ad_value(1718)), s.ad_value(1718))), 1e-50);
        }

        if ((s.v[85] != 0.0) && (s.v[1721] != 0.0)) {
            s.store_div(289, 1719, 1720);
        }

        if ((s.v[85] != 0.0) && (!(s.v[1721] != 0.0))) {
            s.store_scalar(289, (p.p233 + 1e-50));
        }

        if (s.v[85] != 0.0) {
            s.store_scalar(1719, p.p235);
        }

        if (s.v[85] != 0.0) {
            s.store_mul(290, 1719, 323);
        }

        s.v[1729] = if ((p.p31 != 0.0) && (!(s.v[145] != 0.0))) { 1.0 } else { 0.0 };

        if (s.v[1729] != 0.0) {
            s.store_scalar(1726, s.v[62]);
        }

        if (s.v[1729] != 0.0) {
            s.store_scalar(1727, s.v[63]);
        }

    }

    pub(super) fn stamp_transient_block_29(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1729] != 0.0) {
            s.store_scalar(1728, s.v[64]);
        }

        if (s.v[1729] != 0.0) {
            s.store_scale(1722, 244, 6.241449993689894e18);
        }

        if (s.v[1729] != 0.0) {
            s.store_scale_ad(1723, A::mul(A::add(A::add(s.ad_value(323), A::div(s.ad_value(244), A::sub(s.ad_value(161), s.ad_value(435)))), s.ad_value(1728)), s.ad_value(227)), 6.241449993689894e18);
        }

        if (s.v[1729] != 0.0) {
            s.store_sub_ad_lhs(1724, A::div(A::div(A::scale(s.ad_value(197), ((-2.0) * 6.241449993689894e18)), s.ad_value(442)), s.ad_value(108)), 1722);
        }

        s.v[1730] = if ((((s.v[1724] - s.v[1722])) as f64).abs() > (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if ((s.v[1729] != 0.0) && (s.v[1730] != 0.0)) {
            let assign33560_ad_e48687: A = A::add(A::div(A::div_from_scalar(1.0, A::add(s.ad_value(1722), s.ad_value(1723))), A::add(s.ad_value(1724), s.ad_value(1723))), A::mul(A::div(A::mul(A::mul(A::scale(s.ad_value(1726), 2.0), s.ad_value(252)), s.ad_value(250)), A::sub(s.ad_value(1724), s.ad_value(1722))), A::ln(A::div(A::add(s.ad_value(1724), s.ad_value(1723)), A::add(s.ad_value(1722), s.ad_value(1723))))));
            s.store_add_ad(1725, assign33560_ad_e48687, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(1726), s.ad_value(252)), s.ad_value(250)), s.ad_value(1726)), s.ad_value(252)), s.ad_value(250)));
        }

        if ((s.v[1729] != 0.0) && (!(s.v[1730] != 0.0))) {
            let assign33570_ad_e48738: A = A::add(A::add(A::div(A::div_from_scalar(1.0, A::add(s.ad_value(1722), s.ad_value(1723))), A::add(s.ad_value(1724), s.ad_value(1723))), A::div(A::mul(A::mul(A::scale(s.ad_value(1726), 2.0), s.ad_value(252)), s.ad_value(250)), A::add(s.ad_value(1722), s.ad_value(1723)))), A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(1726), s.ad_value(252)), s.ad_value(250)), s.ad_value(1726)), s.ad_value(252)), s.ad_value(250)));
            s.store_ad(1725, &assign33570_ad_e48738);
        }

        if (s.v[1729] != 0.0) {
            s.store_mul_ad_lhs(291, A::div(A::mul(A::square(s.ad_value(199)), s.ad_value(1727)), A::mul(A::mul(s.ad_value(441), s.ad_value(225)), s.ad_value(107))), 1725);
        }

        if (!(s.v[1729] != 0.0)) {
            s.store_scalar(291, 0.0);
        }

        s.v[1748] = if ((p.p32 != 0.0) && (!(s.v[145] != 0.0))) { 1.0 } else { 0.0 };

        if (s.v[1748] != 0.0) {
            s.store_div_ad_lhs(1731, A::sub(s.ad_value(314), s.ad_value(161)), 441);
        }

        if (s.v[1748] != 0.0) {
            s.store_scaled_mul(1732, 251, 1731, 1e-5);
        }

        s.v[1749] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1748] != 0.0) && (s.v[1749] != 0.0)) {
            s.store_scalar(1733, 1.0);
        }

        s.v[1750] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1748] != 0.0) && (!(s.v[1749] != 0.0))) && (s.v[1750] != 0.0)) {
            s.copy_ad(1733, 1732);
        }

        if (((s.v[1748] != 0.0) && (!(s.v[1749] != 0.0))) && (!(s.v[1750] != 0.0))) {
            s.store_powf(1733, 1732, (p.p113 - 1.0));
        }

        if (s.v[1748] != 0.0) {
            s.store_mul(1734, 1732, 1733);
        }

        if (s.v[1748] != 0.0) {
            s.store_offset(1735, 1734, 1.0);
        }

        if (s.v[1748] != 0.0) {
            s.store_powf(1736, 1735, (((-1.0) / p.p113) - 1.0));
        }

        if (s.v[1748] != 0.0) {
            s.store_mul(1737, 1735, 1736);
        }

        if (s.v[1748] != 0.0) {
            s.store_mul(293, 251, 1737);
        }

        if (s.v[1748] != 0.0) {
            s.store_scaled_add(1739, 250, 293, 0.5);
        }

        if (s.v[1748] != 0.0) {
            s.store_square(1738, 190);
        }

        if (s.v[1748] != 0.0) {
            let assign33750_ad_e48938: A = A::add(A::add(A::mul(A::mul(A::add(A::offset(A::scale(s.ad_value(190), 3.0), 1.0), A::scale(s.ad_value(1738), 6.0)), s.ad_value(293)), s.ad_value(293)), A::mul(A::mul(A::add(A::offset(A::scale(s.ad_value(190), 4.0), 3.0), A::scale(s.ad_value(1738), 3.0)), s.ad_value(293)), s.ad_value(250))), A::mul(A::mul(A::add(A::offset(A::scale(s.ad_value(190), 3.0), 6.0), s.ad_value(1738)), s.ad_value(250)), s.ad_value(250)));
            s.store_div_ad(292, A::mul(A::mul(A::mul(A::mul(s.ad_value(107), s.ad_value(323)), s.ad_value(192)), s.ad_value(250)), assign33750_ad_e48938), A::mul(A::mul(A::mul(A::scale(s.ad_value(441), 15.0), A::offset(s.ad_value(190), 1.0)), s.ad_value(1739)), s.ad_value(1739)));
        }

        if (!(s.v[1748] != 0.0)) {
            s.store_scalar(292, 0.0);
        }

        s.v[1751] = if ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (s.v[146] == 1.0)) && (!(s.v[145] != 0.0))) { 1.0 } else { 0.0 };

        if (s.v[1751] != 0.0) {
            s.store_sqrt(298, 296);
        }

        if (s.v[1751] != 0.0) {
            s.store_add(1740, 192, 298);
        }

        if (s.v[1751] != 0.0) {
            s.store_square(1741, 294);
        }

        if (s.v[1751] != 0.0) {
            s.store_square(1742, 296);
        }

        if (s.v[1751] != 0.0) {
            s.store_mul_ad_lhs(1743, A::scale(s.ad_value(294), 42.0), 296);
        }

        if (s.v[1751] != 0.0) {
            s.store_add_ad_rhs(1743, 1743, A::scale(A::add(s.ad_value(1741), s.ad_value(1742)), 4.0));
        }

        if (s.v[1751] != 0.0) {
            s.store_add_ad_rhs(1743, 1743, A::mul(A::mul(A::scale(s.ad_value(298), 20.0), s.ad_value(192)), A::add(s.ad_value(294), s.ad_value(296))));
        }

        if (s.v[1751] != 0.0) {
            s.store_square(1744, 1740);
        }

        if (s.v[1751] != 0.0) {
            s.store_square(1736, 1744);
        }

        if (s.v[1751] != 0.0) {
            s.store_div_ad_rhs(299, 1743, A::mul(s.ad_value(1736), s.ad_value(1740)));
        }

        if (s.v[1751] != 0.0) {
            s.store_mul_ad_lhs(300, A::mul(A::div(s.ad_value(107), s.ad_value(441)), s.ad_value(250)), 323);
        }

        if (s.v[1751] != 0.0) {
            s.store_mul(1746, 300, 192);
        }

        if (s.v[1751] != 0.0) {
            s.store_div(1747, 292, 1746);
        }

        if (s.v[1751] != 0.0) {
            s.store_add_ad_lhs(1745, A::add(s.ad_value(294), A::mul(A::scale(s.ad_value(192), 4.0), s.ad_value(298))), 296);
        }

        if (s.v[1751] != 0.0) {
            s.store_div_ad(301, A::mul(A::scale(s.ad_value(297), 3.872983346207417), s.ad_value(1745)), A::mul(A::scale(s.ad_value(1740), 6.0), A::sqrt(A::mul(A::mul(A::mul(s.ad_value(1747), s.ad_value(1740)), s.ad_value(192)), s.ad_value(1743)))));
        }

        s.store_add(199, 199, 265);

        s.v[1752] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1752] != 0.0) {
            s.store_add(271, 531, 532);
        }

        if ((s.v[1752] != 0.0) && (s.v[564] != 0.0)) {
            s.store_offset(271, 271, (-(p.p168 * s.v[99])));
        }

        if (s.v[1752] != 0.0) {
            s.store_mul_ad(272, A::neg(s.ad_value(271)), A::sub(s.ad_value(158), s.ad_value(513)));
        }

        if (s.v[1752] != 0.0) {
            s.store_scalar(276, 0.0);
        }

        if (s.v[1752] != 0.0) {
            s.store_mul_ad(274, A::scale(s.ad_value(276), p.p9), A::offset(s.ad_value(518), s.v[101]));
        }

        if (s.v[1752] != 0.0) {
            s.store_mul_ad(275, A::scale(s.ad_value(276), p.p9), A::offset(s.ad_value(519), s.v[101]));
        }

        if (s.v[1752] != 0.0) {
            s.store_mul_ad_rhs(277, 274, A::sub(s.ad_value(158), s.ad_value(157)));
        }

        if (s.v[1752] != 0.0) {
            s.store_mul(278, 275, 158);
        }

        if (s.v[1752] != 0.0) {
            s.store_mul_ad(279, A::scale(s.ad_value(276), (p.p19 * p.p9)), A::sub(s.ad_value(158), s.ad_value(513)));
        }

        if (s.v[1752] != 0.0) {
            s.store_add(268, 268, 277);
        }

        if (s.v[1752] != 0.0) {
            s.store_add(267, 267, 278);
        }

        if (s.v[1752] != 0.0) {
            s.store_add(272, 272, 279);
        }

        if ((!(s.v[1752] != 0.0)) && (s.v[564] != 0.0)) {
            s.store_scalar(271, ((-p.p168) * s.v[99]));
        }

        if ((!(s.v[1752] != 0.0)) && (s.v[564] != 0.0)) {
            s.store_mul_ad(272, A::neg(s.ad_value(271)), A::sub(s.ad_value(158), s.ad_value(513)));
        }

        if ((!(s.v[1752] != 0.0)) && (!(s.v[564] != 0.0))) {
            s.store_scalar(271, 0.0);
        }

        if ((!(s.v[1752] != 0.0)) && (!(s.v[564] != 0.0))) {
            s.store_scalar(272, 0.0);
        }

        if (!(s.v[1752] != 0.0)) {
            s.store_scalar(273, 0.0);
        }

        if (!(s.v[1752] != 0.0)) {
            s.copy_ad(274, 273);
        }

        if (!(s.v[1752] != 0.0)) {
            s.copy_ad(275, 273);
        }

        if (!(s.v[1752] != 0.0)) {
            s.store_mul_ad_rhs(277, 274, A::sub(s.ad_value(158), s.ad_value(157)));
        }

        if (!(s.v[1752] != 0.0)) {
            s.store_mul(278, 275, 158);
        }

        if (!(s.v[1752] != 0.0)) {
            s.store_add(268, 268, 277);
        }

        if (!(s.v[1752] != 0.0)) {
            s.store_add(267, 267, 278);
        }

        s.store_scale(9, 199, s.v[451]);

        if (s.v[85] != 0.0) {
            s.store_scalar(24, 0.0);
        }

        if (s.v[85] != 0.0) {
            s.store_scalar(23, 0.0);
        }

        s.v[1753] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[85] != 0.0) && (s.v[1753] != 0.0)) {
            s.store_scalar(25, 0.0);
        }

        if ((s.v[85] != 0.0) && (s.v[1753] != 0.0)) {
            s.copy_ad(556, 438);
        }

        if ((s.v[85] != 0.0) && (s.v[1753] != 0.0)) {
            s.store_scale(588, 196, s.v[451]);
        }

        if ((s.v[85] != 0.0) && (s.v[1753] != 0.0)) {
            s.store_scale(587, 197, s.v[451]);
        }

        if ((s.v[85] != 0.0) && (!(s.v[1753] != 0.0))) {
            s.store_scalar(554, 0.0);
        }

        if ((s.v[85] != 0.0) && (!(s.v[1753] != 0.0))) {
            s.store_scale(588, 392, s.v[451]);
        }

        if ((s.v[85] != 0.0) && (!(s.v[1753] != 0.0))) {
            s.store_scaled_add(576, 198, 477, s.v[451]);
        }

        if ((s.v[85] != 0.0) && (!(s.v[1753] != 0.0))) {
            s.store_scale_ad(577, A::add(A::sub(s.ad_value(197), s.ad_value(198)), s.ad_value(476)), s.v[451]);
        }

        s.v[1754] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[85] != 0.0)) && (s.v[1754] != 0.0)) {
            s.store_scale_ad(23, A::sub(A::neg(s.ad_value(196)), s.ad_value(197)), s.v[451]);
        }

        if ((!(s.v[85] != 0.0)) && (s.v[1754] != 0.0)) {
            s.store_scale(24, 198, s.v[451]);
        }

        if ((!(s.v[85] != 0.0)) && (s.v[1754] != 0.0)) {
            s.store_scaled_sub(25, 197, 198, s.v[451]);
        }

        if ((!(s.v[85] != 0.0)) && (!(s.v[1754] != 0.0))) {
            s.store_scale_ad(23, A::sub(A::sub(A::sub(A::neg(s.ad_value(392)), s.ad_value(197)), s.ad_value(476)), s.ad_value(477)), s.v[451]);
        }

        if ((!(s.v[85] != 0.0)) && (!(s.v[1754] != 0.0))) {
            s.store_scaled_add(24, 198, 477, s.v[451]);
        }

        if ((!(s.v[85] != 0.0)) && (!(s.v[1754] != 0.0))) {
            s.store_scale_ad(25, A::add(A::sub(s.ad_value(197), s.ad_value(198)), s.ad_value(476)), s.v[451]);
        }

        s.v[1760] = if (p.p64 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1760] != 0.0) {
            s.store_scalar(280, 0.0);
        }

        if (!(s.v[1760] != 0.0)) {
            s.store_add_ad_lhs(1755, A::scale(s.ad_value(315), s.v[97]), 161);
        }

        s.v[1761] = if (s.v[1755] > s.v[314]) { 1.0 } else { 0.0 };

        if ((!(s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) {
            s.copy_ad(1755, 314);
        }

        if (!(s.v[1760] != 0.0)) {
            s.store_add_ad(1756, A::scale(A::add(s.ad_value(157), s.ad_value(161)), s.v[317]), A::scale(s.ad_value(1755), (1.0 - s.v[317])));
        }

        if (!(s.v[1760] != 0.0)) {
            s.store_sqrt_ad(1757, A::div_from_scalar((2.0 * 1.034943e-10), s.ad_value(229)));
        }

        if (!(s.v[1760] != 0.0)) {
            s.store_scale(1758, 1757, 1.3);
        }

        if (!(s.v[1760] != 0.0)) {
            s.store_mul_ad_lhs(1759, A::scale(s.ad_value(108), 1.034943e-10), 1758);
        }

        if (!(s.v[1760] != 0.0)) {
            s.store_mul_ad_lhs(280, A::sub(A::scale(A::sub(A::add(s.ad_value(161), s.ad_value(157)), s.ad_value(1756)), 1.0 / (p.p64)), s.ad_value(315)), 1759);
        }

        s.v[1762] = if (p.p65 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1762] != 0.0) {
            s.store_add_ad_rhs(280, 280, A::mul(s.ad_value(135), s.ad_value(513)));
        }

        s.v[1763] = if (p.p24 == 1.0) { 1.0 } else { 0.0 };

        s.v[1764] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1763] != 0.0) && (s.v[1764] != 0.0)) {
            s.store_sub_ad_lhs(471, A::sub(A::sub(A::neg(s.ad_value(463)), s.ad_value(464)), s.ad_value(467)), 468);
        }

        if ((s.v[1763] != 0.0) && (s.v[1764] != 0.0)) {
            s.store_add(472, 466, 470);
        }

        if ((s.v[1763] != 0.0) && (s.v[1764] != 0.0)) {
            s.store_add(473, 465, 469);
        }

        if ((s.v[1763] != 0.0) && (s.v[1764] != 0.0)) {
            s.store_add_ad_rhs(23, 23, A::scale(A::add(A::sub(A::sub(A::sub(A::add(A::add(s.ad_value(268), s.ad_value(267)), s.ad_value(272)), s.ad_value(280)), s.ad_value(455)), s.ad_value(454)), s.ad_value(471)), s.v[451]));
        }

        if ((s.v[1763] != 0.0) && (s.v[1764] != 0.0)) {
            s.store_add_ad_rhs(24, 24, A::scale(A::add(A::add(A::sub(s.ad_value(280), s.ad_value(268)), s.ad_value(456)), s.ad_value(472)), s.v[451]));
        }

        if ((s.v[1763] != 0.0) && (s.v[1764] != 0.0)) {
            s.store_add_ad_rhs(25, 25, A::scale(A::add(A::sub(s.ad_value(457), s.ad_value(267)), s.ad_value(473)), s.v[451]));
        }

        if ((s.v[1763] != 0.0) && (!(s.v[1764] != 0.0))) {
            s.store_add_ad_rhs(23, 23, A::scale(A::sub(A::sub(A::sub(A::add(A::add(s.ad_value(268), s.ad_value(267)), s.ad_value(272)), s.ad_value(280)), s.ad_value(455)), s.ad_value(454)), s.v[451]));
        }

        if ((s.v[1763] != 0.0) && (!(s.v[1764] != 0.0))) {
            s.store_add_ad_rhs(24, 24, A::scale(A::add(A::sub(s.ad_value(280), s.ad_value(268)), s.ad_value(456)), s.v[451]));
        }

        if ((s.v[1763] != 0.0) && (!(s.v[1764] != 0.0))) {
            s.store_add_ad_rhs(25, 25, A::scale(A::sub(s.ad_value(457), s.ad_value(267)), s.v[451]));
        }

        s.v[1765] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1765] != 0.0) {
            s.store_scale(36, 281, s.v[451]);
        }

        if (s.v[1765] != 0.0) {
            s.store_scale(35, 282, s.v[451]);
        }

        if (s.v[1765] != 0.0) {
            s.store_scale(560, 284, s.v[451]);
        }

        if (s.v[1765] != 0.0) {
            s.store_scale(561, 283, s.v[451]);
        }

        if (!(s.v[1765] != 0.0)) {
            s.store_scalar(36, 0.0);
        }

        if (!(s.v[1765] != 0.0)) {
            s.store_scalar(35, 0.0);
        }

        if (!(s.v[1765] != 0.0)) {
            s.store_scalar(560, 0.0);
        }

        if (!(s.v[1765] != 0.0)) {
            s.store_scalar(561, 0.0);
        }

        s.v[1766] = if (p.p25 != 1.0) { 1.0 } else { 0.0 };

        if (s.v[1766] != 0.0) {
            s.store_scalar(557, 0.0);
        }

        if (!(s.v[1766] != 0.0)) {
            s.store_scale(557, 263, s.v[451]);
        }

        s.store_scale_ad(15, A::neg(s.ad_value(308)), s.v[451]);

        s.v[1767] = if (s.v[613] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1767] != 0.0) {
            s.store_scale_ad(13, A::sub(A::mul(s.ad_value(310), s.ad_value(309)), s.ad_value(307)), s.v[451]);
        }

        if (!(s.v[1767] != 0.0)) {
            s.store_scale_ad(13, A::sub(A::mul(A::sub_from_scalar(1.0, s.ad_value(310)), s.ad_value(309)), s.ad_value(306)), s.v[451]);
        }

        s.v[1768] = if (s.v[613] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1768] != 0.0) {
            s.store_scale_ad(14, A::sub(A::mul(A::sub_from_scalar(1.0, s.ad_value(310)), s.ad_value(309)), s.ad_value(306)), s.v[451]);
        }

        if (!(s.v[1768] != 0.0)) {
            s.store_scale_ad(14, A::sub(A::mul(s.ad_value(310), s.ad_value(309)), s.ad_value(307)), s.v[451]);
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

        s.v[1775] = if ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (s.v[146] == 1.0)) && (!(s.v[145] != 0.0))) { 1.0 } else { 0.0 };

        if (s.v[1775] != 0.0) {
            s.store_scale_ad(1769, A::mul(A::scale(s.ad_value(323), 1e-6), s.ad_value(108)), s.v[98]);
        }

    }

    pub(super) fn stamp_transient_block_30(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1775] != 0.0) {
            s.store_scale(1770, 555, 1.0 / (s.v[451]));
        }

        if (s.v[1775] != 0.0) {
            s.store_div_ad_lhs(1771, A::mul(A::mul(A::scale(s.ad_value(227), (0.1185185185185185 * 1.6021918e-19)), s.ad_value(1770)), s.ad_value(1770)), 300);
        }

        s.v[1776] = if ((s.v[297] > (10.0 * 2.220446049250313e-16)) && (s.v[157] > (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if ((s.v[1775] != 0.0) && (s.v[1776] != 0.0)) {
            s.store_div(1772, 251, 250);
        }

        if ((s.v[1775] != 0.0) && (s.v[1776] != 0.0)) {
            s.store_div_ad_lhs(1773, A::sub(A::div(s.ad_value(251), s.ad_value(293)), s.ad_value(1772)), 157);
        }

        if ((s.v[1775] != 0.0) && (s.v[1776] != 0.0)) {
            s.store_add_ad_rhs(1774, 1772, A::div(A::mul(A::scale(s.ad_value(1773), 0.6666666666666667), A::add(A::add(s.ad_value(294), A::mul(s.ad_value(192), s.ad_value(298))), s.ad_value(296))), A::add(s.ad_value(192), s.ad_value(298))));
        }

        if ((s.v[1775] != 0.0) && (!(s.v[1776] != 0.0))) {
            s.store_div(1774, 251, 293);
        }

        if (s.v[1775] != 0.0) {
            s.store_mul_ad_lhs(558, A::mul(A::scale(s.ad_value(1771), s.v[451]), s.ad_value(299)), 1774);
        }

        if (s.v[1775] != 0.0) {
            s.copy_ad(559, 301);
        }

        if (s.v[1775] != 0.0) {
            s.store_ad(558, &{
                if (((-s.v[1770]) > s.v[1769]) && (s.v[558] > 0.0)) {
                    s.ad_value(558)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1775] != 0.0) {
            s.store_ad(559, &{
                if ((-s.v[1770]) > s.v[1769]) {
                    s.ad_value(559)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (!(s.v[1775] != 0.0)) {
            s.store_scalar(558, 0.0);
        }

        if (!(s.v[1775] != 0.0)) {
            s.store_scalar(559, 0.0);
        }

        s.v[4] = 0.0;

        s.v[5] = 0.0;

        s.v[7] = 0.0;

        s.v[8] = 0.0;

        s.v[1777] = if (p.p259 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1777] != 0.0) {
            s.store_scalar(3, 1.0);
        }

        s.v[1797] = if (s.v[3] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1777] != 0.0) && (s.v[1797] != 0.0)) {
            s.store_scalar(1788, (p.p264 / 1e-6));
        }

        if ((s.v[1777] != 0.0) && (s.v[1797] != 0.0)) {
            s.store_scalar(1781, p.p266);
        }

        if ((s.v[1777] != 0.0) && (s.v[1797] != 0.0)) {
            s.store_scalar(1782, p.p268);
        }

        if ((s.v[1777] != 0.0) && (s.v[1797] != 0.0)) {
            s.store_scalar(1783, p.p273);
        }

        if ((s.v[1777] != 0.0) && (s.v[1797] != 0.0)) {
            s.store_scalar(1784, (if (p.p263 > 0.0) { (p.p263 * p.p255) } else { 0.0 }));
        }

        if ((s.v[1777] != 0.0) && (s.v[1797] != 0.0)) {
            s.store_scalar(1787, p.p258);
        }

        if ((s.v[1777] != 0.0) && (s.v[1797] != 0.0)) {
            s.store_ad(1785, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(2)), p.p50));
        }

        if ((s.v[1777] != 0.0) && (!(s.v[1797] != 0.0))) {
            s.store_scalar(1788, (p.p59 / 1e-6));
        }

        if ((s.v[1777] != 0.0) && (!(s.v[1797] != 0.0))) {
            s.store_scalar(1781, p.p265);
        }

        if ((s.v[1777] != 0.0) && (!(s.v[1797] != 0.0))) {
            s.store_scalar(1782, p.p267);
        }

        if ((s.v[1777] != 0.0) && (!(s.v[1797] != 0.0))) {
            s.store_scalar(1783, p.p272);
        }

        if ((s.v[1777] != 0.0) && (!(s.v[1797] != 0.0))) {
            s.store_scalar(1784, (if (p.p263 > 0.0) { (p.p263 * p.p256) } else { 0.0 }));
        }

        if ((s.v[1777] != 0.0) && (!(s.v[1797] != 0.0))) {
            s.store_scalar(1787, p.p257);
        }

        if ((s.v[1777] != 0.0) && (!(s.v[1797] != 0.0))) {
            s.store_ad(1785, &A::scale(A::voltage(ctx, &nodes, Some(0), Some(6)), p.p50));
        }

        if (s.v[1777] != 0.0) {
            s.store_scalar(1794, ((((p.p271 * p.p271) + (p.p56 * p.p56))) as f64).sqrt());
        }

        if (s.v[1777] != 0.0) {
            s.store_scale(1796, 105, p.p9);
        }

        if (s.v[1777] != 0.0) {
            s.store_scale(1781, 1781, 0.0001);
        }

        if (s.v[1777] != 0.0) {
            s.store_scale(1782, 1782, 0.01);
        }

        if (s.v[1777] != 0.0) {
            s.store_scale(1786, 429, 1.0 / (s.v[81]));
        }

        if (s.v[1777] != 0.0) {
            s.store_powf(328, 1786, p.p269);
        }

        if (s.v[1777] != 0.0) {
            s.store_div(1789, 1781, 328);
        }

        if (s.v[1777] != 0.0) {
            s.store_sub_ad(327, A::add(A::offset(A::scale(s.ad_value(1786), 0.4), 1.8), A::mul(A::scale(s.ad_value(1786), 0.1), s.ad_value(1786))), A::scale(A::sub_from_scalar(1.0, s.ad_value(1786)), p.p270));
        }

        if (s.v[1777] != 0.0) {
            s.store_div(1790, 1782, 327);
        }

        if (s.v[1777] != 0.0) {
            s.store_add_ad_rhs(1783, 1783, A::scale(A::offset(s.ad_value(429), (-s.v[81])), p.p274));
        }

        if (s.v[1777] != 0.0) {
            s.store_scalar(1778, (1.0 + (p.p279 / ((s.v[100]) as f64).powf(p.p280))));
        }

        if (s.v[1777] != 0.0) {
            s.store_scalar(1780, (1.0 + (p.p277 / ((s.v[100]) as f64).powf(p.p278))));
        }

        if (s.v[1777] != 0.0) {
            s.store_scalar(1779, (1.0 + (p.p275 / ((s.v[109]) as f64).powf(p.p276))));
        }

        if (s.v[1777] != 0.0) {
            s.store_mul(1789, 1789, 1778);
        }

        if (s.v[1777] != 0.0) {
            s.store_offset_ad(1790, A::mul(A::mul(s.ad_value(1790), s.ad_value(1779)), s.ad_value(1780)), 1e-50);
        }

        if (s.v[1777] != 0.0) {
            s.store_div(1791, 1785, 1787);
        }

        if (s.v[1777] != 0.0) {
            s.store_mul(1792, 1789, 1791);
        }

        s.v[1798] = if (s.v[1785] >= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1777] != 0.0) && (s.v[1798] != 0.0)) {
            s.store_div(328, 1792, 1790);
        }

        if ((s.v[1777] != 0.0) && (!(s.v[1798] != 0.0))) {
            s.store_div_ad_lhs(328, A::neg(s.ad_value(1792)), 1790);
        }

        s.v[1799] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1783]) && (s.v[1783] <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1777] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_scalar(330, 1.0);
        }

        s.v[1800] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1783]) && (s.v[1783] <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1777] != 0.0) && (!(s.v[1799] != 0.0))) && (s.v[1800] != 0.0)) {
            s.copy_ad(330, 328);
        }

        if (((s.v[1777] != 0.0) && (!(s.v[1799] != 0.0))) && (!(s.v[1800] != 0.0))) {
            s.store_ad(330, &A::pow(s.ad_value(328), A::offset(s.ad_value(1783), (-1.0))));
        }

        if (s.v[1777] != 0.0) {
            s.store_mul(329, 328, 330);
        }

        if (s.v[1777] != 0.0) {
            s.store_offset(331, 329, 1.0);
        }

        s.v[1801] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1783]) && (s.v[1783] <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1777] != 0.0) && (s.v[1801] != 0.0)) {
            s.store_div_from_scalar(332, 1.0, 331);
        }

        s.v[1802] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1783]) && (s.v[1783] <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1777] != 0.0) && (!(s.v[1801] != 0.0))) && (s.v[1802] != 0.0)) {
            s.store_div_from_scalar_ad(332, 1.0, A::sqrt(s.ad_value(331)));
        }

        if (((s.v[1777] != 0.0) && (!(s.v[1801] != 0.0))) && (!(s.v[1802] != 0.0))) {
            s.store_ad(333, &A::pow(s.ad_value(331), A::offset(A::div_from_scalar((-1.0), s.ad_value(1783)), (-1.0))));
        }

        if (((s.v[1777] != 0.0) && (!(s.v[1801] != 0.0))) && (!(s.v[1802] != 0.0))) {
            s.store_mul(332, 331, 333);
        }

        if (s.v[1777] != 0.0) {
            s.store_mul(1793, 1789, 332);
        }

        if (s.v[1777] != 0.0) {
            s.store_div_from_scalar(328, 1.6021918e-19, 1787);
        }

        if (s.v[1777] != 0.0) {
            s.store_mul_ad_lhs(1795, A::mul(A::mul(s.ad_value(328), s.ad_value(1794)), s.ad_value(1793)), 1788);
        }

        s.v[1803] = if (s.v[1795] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1777] != 0.0) && (s.v[1803] != 0.0)) {
            s.store_scalar(1795, 1e-50);
        }

        if (s.v[1777] != 0.0) {
            s.store_div_from_scalar(1, 1.0, 1795);
        }

        if (s.v[1777] != 0.0) {
            s.store_div(1, 1, 1796);
        }

        if (s.v[1777] != 0.0) {
            s.store_add(1, 1, 1784);
        }

        if (s.v[1777] != 0.0) {
            s.store_ad(6, &{
                if ((s.v[1] > 0.0001) && (p.p32 != 0.0)) {
                    A::div_from_scalar(s.v[451], s.ad_value(1))
                } else {
                    A::constant(0.0)
                }
            });
        }

        s.v[1804] = if (s.v[1] < 0.0001) { 1.0 } else { 0.0 };

        if ((s.v[1777] != 0.0) && (s.v[1804] != 0.0)) {
            s.store_scalar(1, 0.0001);
        }

        if (s.v[1777] != 0.0) {
            s.store_scale(5, 1, 1.0 / (s.v[451]));
        }

        if (s.v[1777] != 0.0) {
            s.copy_ad(8, 6);
        }

        s.v[1805] = if (p.p260 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1805] != 0.0) {
            s.store_scalar(3, 2.0);
        }

        s.v[1825] = if (s.v[3] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1805] != 0.0) && (s.v[1825] != 0.0)) {
            s.store_scalar(1816, (p.p264 / 1e-6));
        }

        if ((s.v[1805] != 0.0) && (s.v[1825] != 0.0)) {
            s.store_scalar(1809, p.p266);
        }

        if ((s.v[1805] != 0.0) && (s.v[1825] != 0.0)) {
            s.store_scalar(1810, p.p268);
        }

        if ((s.v[1805] != 0.0) && (s.v[1825] != 0.0)) {
            s.store_scalar(1811, p.p273);
        }

        if ((s.v[1805] != 0.0) && (s.v[1825] != 0.0)) {
            s.store_scalar(1812, (if (p.p263 > 0.0) { (p.p263 * p.p255) } else { 0.0 }));
        }

        if ((s.v[1805] != 0.0) && (s.v[1825] != 0.0)) {
            s.store_scalar(1815, p.p258);
        }

        if ((s.v[1805] != 0.0) && (s.v[1825] != 0.0)) {
            s.store_ad(1813, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(2)), p.p50));
        }

        if ((s.v[1805] != 0.0) && (!(s.v[1825] != 0.0))) {
            s.store_scalar(1816, (p.p59 / 1e-6));
        }

        if ((s.v[1805] != 0.0) && (!(s.v[1825] != 0.0))) {
            s.store_scalar(1809, p.p265);
        }

        if ((s.v[1805] != 0.0) && (!(s.v[1825] != 0.0))) {
            s.store_scalar(1810, p.p267);
        }

        if ((s.v[1805] != 0.0) && (!(s.v[1825] != 0.0))) {
            s.store_scalar(1811, p.p272);
        }

        if ((s.v[1805] != 0.0) && (!(s.v[1825] != 0.0))) {
            s.store_scalar(1812, (if (p.p263 > 0.0) { (p.p263 * p.p256) } else { 0.0 }));
        }

        if ((s.v[1805] != 0.0) && (!(s.v[1825] != 0.0))) {
            s.store_scalar(1815, p.p257);
        }

        if ((s.v[1805] != 0.0) && (!(s.v[1825] != 0.0))) {
            s.store_ad(1813, &A::scale(A::voltage(ctx, &nodes, Some(0), Some(6)), p.p50));
        }

        if (s.v[1805] != 0.0) {
            s.store_scalar(1822, ((((p.p271 * p.p271) + (p.p56 * p.p56))) as f64).sqrt());
        }

        if (s.v[1805] != 0.0) {
            s.store_scale(1824, 105, p.p9);
        }

        if (s.v[1805] != 0.0) {
            s.store_scale(1809, 1809, 0.0001);
        }

        if (s.v[1805] != 0.0) {
            s.store_scale(1810, 1810, 0.01);
        }

        if (s.v[1805] != 0.0) {
            s.store_scale(1814, 429, 1.0 / (s.v[81]));
        }

        if (s.v[1805] != 0.0) {
            s.store_powf(328, 1814, p.p269);
        }

        if (s.v[1805] != 0.0) {
            s.store_div(1817, 1809, 328);
        }

        if (s.v[1805] != 0.0) {
            s.store_sub_ad(327, A::add(A::offset(A::scale(s.ad_value(1814), 0.4), 1.8), A::mul(A::scale(s.ad_value(1814), 0.1), s.ad_value(1814))), A::scale(A::sub_from_scalar(1.0, s.ad_value(1814)), p.p270));
        }

        if (s.v[1805] != 0.0) {
            s.store_div(1818, 1810, 327);
        }

        if (s.v[1805] != 0.0) {
            s.store_add_ad_rhs(1811, 1811, A::scale(A::offset(s.ad_value(429), (-s.v[81])), p.p274));
        }

        if (s.v[1805] != 0.0) {
            s.store_scalar(1806, (1.0 + (p.p279 / ((s.v[100]) as f64).powf(p.p280))));
        }

        if (s.v[1805] != 0.0) {
            s.store_scalar(1808, (1.0 + (p.p277 / ((s.v[100]) as f64).powf(p.p278))));
        }

        if (s.v[1805] != 0.0) {
            s.store_scalar(1807, (1.0 + (p.p275 / ((s.v[109]) as f64).powf(p.p276))));
        }

        if (s.v[1805] != 0.0) {
            s.store_mul(1817, 1817, 1806);
        }

        if (s.v[1805] != 0.0) {
            s.store_offset_ad(1818, A::mul(A::mul(s.ad_value(1818), s.ad_value(1807)), s.ad_value(1808)), 1e-50);
        }

        if (s.v[1805] != 0.0) {
            s.store_div(1819, 1813, 1815);
        }

        if (s.v[1805] != 0.0) {
            s.store_mul(1820, 1817, 1819);
        }

        s.v[1826] = if (s.v[1813] >= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1805] != 0.0) && (s.v[1826] != 0.0)) {
            s.store_div(328, 1820, 1818);
        }

        if ((s.v[1805] != 0.0) && (!(s.v[1826] != 0.0))) {
            s.store_div_ad_lhs(328, A::neg(s.ad_value(1820)), 1818);
        }

        s.v[1827] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1811]) && (s.v[1811] <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1805] != 0.0) && (s.v[1827] != 0.0)) {
            s.store_scalar(330, 1.0);
        }

        s.v[1828] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1811]) && (s.v[1811] <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1805] != 0.0) && (!(s.v[1827] != 0.0))) && (s.v[1828] != 0.0)) {
            s.copy_ad(330, 328);
        }

        if (((s.v[1805] != 0.0) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_ad(330, &A::pow(s.ad_value(328), A::offset(s.ad_value(1811), (-1.0))));
        }

        if (s.v[1805] != 0.0) {
            s.store_mul(329, 328, 330);
        }

        if (s.v[1805] != 0.0) {
            s.store_offset(331, 329, 1.0);
        }

        s.v[1829] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1811]) && (s.v[1811] <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1805] != 0.0) && (s.v[1829] != 0.0)) {
            s.store_div_from_scalar(332, 1.0, 331);
        }

        s.v[1830] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1811]) && (s.v[1811] <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1805] != 0.0) && (!(s.v[1829] != 0.0))) && (s.v[1830] != 0.0)) {
            s.store_div_from_scalar_ad(332, 1.0, A::sqrt(s.ad_value(331)));
        }

        if (((s.v[1805] != 0.0) && (!(s.v[1829] != 0.0))) && (!(s.v[1830] != 0.0))) {
            s.store_ad(333, &A::pow(s.ad_value(331), A::offset(A::div_from_scalar((-1.0), s.ad_value(1811)), (-1.0))));
        }

        if (((s.v[1805] != 0.0) && (!(s.v[1829] != 0.0))) && (!(s.v[1830] != 0.0))) {
            s.store_mul(332, 331, 333);
        }

        if (s.v[1805] != 0.0) {
            s.store_mul(1821, 1817, 332);
        }

        if (s.v[1805] != 0.0) {
            s.store_div_from_scalar(328, 1.6021918e-19, 1815);
        }

        if (s.v[1805] != 0.0) {
            s.store_mul_ad_lhs(1823, A::mul(A::mul(s.ad_value(328), s.ad_value(1822)), s.ad_value(1821)), 1816);
        }

        s.v[1831] = if (s.v[1823] <= 0.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_31(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[1805] != 0.0) && (s.v[1831] != 0.0)) {
            s.store_scalar(1823, 1e-50);
        }

        if (s.v[1805] != 0.0) {
            s.store_div_from_scalar(1, 1.0, 1823);
        }

        if (s.v[1805] != 0.0) {
            s.store_div(1, 1, 1824);
        }

        if (s.v[1805] != 0.0) {
            s.store_add(1, 1, 1812);
        }

        if (s.v[1805] != 0.0) {
            s.store_ad(6, &{
                if ((s.v[1] > 0.0001) && (p.p32 != 0.0)) {
                    A::div_from_scalar(s.v[451], s.ad_value(1))
                } else {
                    A::constant(0.0)
                }
            });
        }

        s.v[1832] = if (s.v[1] < 0.0001) { 1.0 } else { 0.0 };

        if ((s.v[1805] != 0.0) && (s.v[1832] != 0.0)) {
            s.store_scalar(1, 0.0001);
        }

        if (s.v[1805] != 0.0) {
            s.store_scale(4, 1, 1.0 / (s.v[451]));
        }

        if (s.v[1805] != 0.0) {
            s.copy_ad(7, 6);
        }

        s.v[1833] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        s.v[1834] = if (s.v[289] < (1e-15 / 0.0001)) { 1.0 } else { 0.0 };

        if (((s.v[1833] != 0.0) && (s.v[85] != 0.0)) && (s.v[1834] != 0.0)) {
            s.store_scalar(289, (1e-15 / 0.0001));
        }

        s.v[1835] = if (s.v[290] < (1e-15 / 0.0001)) { 1.0 } else { 0.0 };

        if (((s.v[1833] != 0.0) && (s.v[85] != 0.0)) && (s.v[1835] != 0.0)) {
            s.store_scalar(290, (1e-15 / 0.0001));
        }

        if ((s.v[1833] != 0.0) && (s.v[85] != 0.0)) {
            s.store_ad(438, &{
                if (s.v[613] == 1.0) {
                    s.ad_value(556)
                } else {
                    A::sub_from_scalar(1.0, s.ad_value(556))
                }
            });
        }

        if ((s.v[1833] != 0.0) && (s.v[85] != 0.0)) {
            s.store_div_ad_lhs(582, A::sub(s.ad_value(580), s.ad_value(587)), 289);
        }

        if ((s.v[1833] != 0.0) && (s.v[85] != 0.0)) {
            s.store_div_ad_lhs(583, A::sub(s.ad_value(581), s.ad_value(588)), 290);
        }

        if ((s.v[1833] != 0.0) && (s.v[85] != 0.0)) {
            s.store_add_ad_lhs(584, A::mul(s.ad_value(580), s.ad_value(438)), 473);
        }

        if ((s.v[1833] != 0.0) && (s.v[85] != 0.0)) {
            s.store_add_ad_lhs(585, A::mul(s.ad_value(580), A::sub_from_scalar(1.0, s.ad_value(438))), 473);
        }

        if ((s.v[1833] != 0.0) && (s.v[85] != 0.0)) {
            s.store_add_ad_lhs(586, A::sub(A::neg(s.ad_value(580)), s.ad_value(581)), 471);
        }

        if ((s.v[1833] != 0.0) && (!(s.v[85] != 0.0))) {
            s.store_scalar(582, 0.0);
        }

        if ((s.v[1833] != 0.0) && (!(s.v[85] != 0.0))) {
            s.store_scalar(583, 0.0);
        }

        if ((s.v[1833] != 0.0) && (!(s.v[85] != 0.0))) {
            s.store_scalar(584, 0.0);
        }

        if ((s.v[1833] != 0.0) && (!(s.v[85] != 0.0))) {
            s.store_scalar(585, 0.0);
        }

        if ((s.v[1833] != 0.0) && (!(s.v[85] != 0.0))) {
            s.store_scalar(586, 0.0);
        }

        if ((s.v[1833] != 0.0) && (!(s.v[85] != 0.0))) {
            s.store_scalar(581, 0.0);
        }

        s.v[1836] = if (s.v[289] < (1e-15 / 0.0001)) { 1.0 } else { 0.0 };

        if (((!(s.v[1833] != 0.0)) && (s.v[85] != 0.0)) && (s.v[1836] != 0.0)) {
            s.store_scalar(289, (1e-15 / 0.0001));
        }

        s.v[1837] = if (s.v[290] < (1e-15 / 0.0001)) { 1.0 } else { 0.0 };

        if (((!(s.v[1833] != 0.0)) && (s.v[85] != 0.0)) && (s.v[1837] != 0.0)) {
            s.store_scalar(290, (1e-15 / 0.0001));
        }

        if ((!(s.v[1833] != 0.0)) && (s.v[85] != 0.0)) {
            s.store_div_ad_lhs(574, A::sub(s.ad_value(584), s.ad_value(576)), 289);
        }

        if ((!(s.v[1833] != 0.0)) && (s.v[85] != 0.0)) {
            s.store_div_ad_lhs(575, A::sub(s.ad_value(585), s.ad_value(577)), 289);
        }

        if ((!(s.v[1833] != 0.0)) && (s.v[85] != 0.0)) {
            s.store_div_ad_lhs(583, A::sub(s.ad_value(581), s.ad_value(588)), 290);
        }

        if ((!(s.v[1833] != 0.0)) && (s.v[85] != 0.0)) {
            s.store_scalar(583, 0.0);
        }

        if ((!(s.v[1833] != 0.0)) && (s.v[85] != 0.0)) {
            s.store_sub_ad_lhs(586, A::sub(A::neg(s.ad_value(584)), s.ad_value(585)), 581);
        }

        if ((!(s.v[1833] != 0.0)) && (!(s.v[85] != 0.0))) {
            s.store_scalar(574, 0.0);
        }

        if ((!(s.v[1833] != 0.0)) && (!(s.v[85] != 0.0))) {
            s.store_scalar(575, 0.0);
        }

        if ((!(s.v[1833] != 0.0)) && (!(s.v[85] != 0.0))) {
            s.store_scalar(583, 0.0);
        }

        if ((!(s.v[1833] != 0.0)) && (!(s.v[85] != 0.0))) {
            s.store_scalar(584, 0.0);
        }

        if ((!(s.v[1833] != 0.0)) && (!(s.v[85] != 0.0))) {
            s.store_scalar(585, 0.0);
        }

        if ((!(s.v[1833] != 0.0)) && (!(s.v[85] != 0.0))) {
            s.store_scalar(586, 0.0);
        }

        if ((!(s.v[1833] != 0.0)) && (!(s.v[85] != 0.0))) {
            s.store_scalar(581, 0.0);
        }

        s.copy_ad(0, 4);

        s.copy_ad(1, 5);

        s.v[1838] = if (s.v[613] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1838] != 0.0) {
            s.copy_ad(199, 9);
        }

        if (s.v[1838] != 0.0) {
            s.copy_ad(263, 557);
        }

        if (s.v[1838] != 0.0) {
            s.store_scalar(573, 0.0);
        }

        if (s.v[1838] != 0.0) {
            s.store_add(594, 23, 586);
        }

        if (s.v[1838] != 0.0) {
            s.store_add(198, 24, 584);
        }

        if (s.v[1838] != 0.0) {
            s.store_neg_ad(554, A::add(A::add(s.ad_value(23), s.ad_value(24)), s.ad_value(25)));
        }

        if (s.v[1838] != 0.0) {
            s.store_add(196, 554, 581);
        }

        if (!(s.v[1838] != 0.0)) {
            s.store_neg(199, 9);
        }

        if (!(s.v[1838] != 0.0)) {
            s.copy_ad(573, 557);
        }

        if (!(s.v[1838] != 0.0)) {
            s.store_scalar(263, 0.0);
        }

        if (!(s.v[1838] != 0.0)) {
            s.store_add(594, 23, 586);
        }

        if (!(s.v[1838] != 0.0)) {
            s.store_add(198, 25, 585);
        }

        if (!(s.v[1838] != 0.0)) {
            s.store_neg_ad(554, A::add(A::add(s.ad_value(23), s.ad_value(24)), s.ad_value(25)));
        }

        if (!(s.v[1838] != 0.0)) {
            s.store_add(196, 554, 581);
        }

        s.copy_ad(307, 13);

        s.copy_ad(306, 14);

        s.copy_ad(308, 15);

        s.copy_ad(311, 11);

        s.copy_ad(312, 12);

        s.v[1839] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1839] != 0.0) {
            s.copy_ad(282, 35);
        }

        if (s.v[1839] != 0.0) {
            s.copy_ad(284, 560);
        }

        if (s.v[1839] != 0.0) {
            s.copy_ad(281, 36);
        }

        if (s.v[1839] != 0.0) {
            s.copy_ad(283, 561);
        }

        s.v[1840] = if ((p.p38 == 1.0) && (s.v[67] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1840] != 0.0) {
            s.store_mul(578, 199, 157);
        }

        if (s.v[1840] != 0.0) {
            s.copy_ad(563, 542);
        }

        if (s.v[1840] != 0.0) {
            s.store_div_from_scalar(589, 1.0, 541);
        }

        if (!(s.v[1840] != 0.0)) {
            s.store_scalar(578, 0.0);
        }

        if (!(s.v[1840] != 0.0)) {
            s.store_scalar(563, 0.0);
        }

        if (!(s.v[1840] != 0.0)) {
            s.store_scalar(589, 0.0);
        }

        s.copy_ad(9, 199);

        s.store_scalar(27, A::ddx_projection(&s.ad_value(594), Some(6), None));

        s.store_scale(27, 27, p.p50);

        s.store_scalar(28, A::ddx_projection(&s.ad_value(594), Some(7), None));

        s.store_scale(28, 28, p.p50);

        s.v[1842] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1842] != 0.0) {
            s.store_scale(35, 282, p.p50);
        }

        if (s.v[1842] != 0.0) {
            s.store_scale(36, 281, p.p50);
        }

        s.store_scale(610, 429, (4.0 * 1.3806226e-23));

        s.v[1848] = if (p.p27 == 1.0) { 1.0 } else { 0.0 };

        s.copy_ad(438, 439);

        s.store_mul(615, 610, 598);

        s.copy_ad(614, 559);

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

        s.v[1849] = if (p.p27 == 1.0) { 1.0 } else { 0.0 };

        s.v[1850] = if ((p.p38 > 0.0) && (p.p242 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1850] != 0.0) {
            s.copy_ad(595, 578);
        }

        s.v[1851] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        s.v[1852] = if ((p.p37 != 0.0) || ((p.p25 == 1.0) && (p.p26 == 2.0))) { 1.0 } else { 0.0 };

        s.v[1853] = if (p.p43 == 0.0) { 1.0 } else { 0.0 };

    }
}
