#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_block_12(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[1275] != 0.0) && (!(s.v[1294] != 0.0))) {
            s.store_mul(846, 974, 951);
        }

        s.v[1302] = if (s.v[952] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) {
            s.store_mul_ad(970, A::scale(s.ad_value(303), p.p1043), A::offset(A::mul(s.ad_value(254), s.ad_value(430)), 1.0));
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) {
            s.store_mul_ad(971, A::scale(s.ad_value(305), p.p1043), A::offset(A::mul(s.ad_value(255), s.ad_value(430)), 1.0));
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) {
            s.store_div(843, 1088, 970);
        }

        s.v[1303] = if (s.v[843] > 100.0) { 1.0 } else { 0.0 };

        if (((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (s.v[1303] != 0.0)) {
            s.store_scale_ad(853, A::offset(A::offset(s.ad_value(843), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1304] = if (s.v[843] < (-100.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (!(s.v[1303] != 0.0))) && (s.v[1304] != 0.0)) {
            s.store_scalar(853, 3.720075976e-44);
        }

        if ((((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (!(s.v[1303] != 0.0))) && (!(s.v[1304] != 0.0))) {
            s.store_exp(853, 843);
        }

        s.v[1305] = if ((s.v[315] - s.v[1088]) < 0.001) { 1.0 } else { 0.0 };

        if (((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (s.v[1305] != 0.0)) {
            s.store_scalar(844, 1000.0);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (s.v[1305] != 0.0)) {
            s.store_mul_ad_lhs(843, A::mul(A::div(A::neg(s.ad_value(1088)), s.ad_value(971)), s.ad_value(315)), 844);
        }

        s.v[1306] = if (s.v[843] > 100.0) { 1.0 } else { 0.0 };

        if ((((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (s.v[1305] != 0.0)) && (s.v[1306] != 0.0)) {
            s.store_scale_ad(854, A::offset(A::offset(s.ad_value(843), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1307] = if (s.v[843] < (-100.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (s.v[1305] != 0.0)) && (!(s.v[1306] != 0.0))) && (s.v[1307] != 0.0)) {
            s.store_scalar(854, 3.720075976e-44);
        }

        if (((((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (s.v[1305] != 0.0)) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) {
            s.store_exp(854, 843);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (s.v[1305] != 0.0)) {
            s.store_neg(854, 854);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (!(s.v[1305] != 0.0))) {
            s.store_div_from_scalar_ad(844, 1.0, A::sub(s.ad_value(315), s.ad_value(1088)));
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (!(s.v[1305] != 0.0))) {
            s.store_mul_ad_lhs(843, A::mul(A::div(A::neg(s.ad_value(1088)), s.ad_value(971)), s.ad_value(315)), 844);
        }

        s.v[1308] = if (s.v[843] > 100.0) { 1.0 } else { 0.0 };

        if ((((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (!(s.v[1305] != 0.0))) && (s.v[1308] != 0.0)) {
            s.store_scale_ad(854, A::offset(A::offset(s.ad_value(843), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1309] = if (s.v[843] < (-100.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1309] != 0.0)) {
            s.store_scalar(854, 3.720075976e-44);
        }

        if (((((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) {
            s.store_exp(854, 843);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (!(s.v[1305] != 0.0))) {
            s.store_neg(854, 854);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) {
            s.store_mul(846, 975, 952);
        }

        if (s.v[1275] != 0.0) {
            s.store_scalar(930, ((s.v[328] / p.p23) * p.p155));
        }

        s.v[1310] = if ((s.v[949] <= 0.0) && (s.v[950] <= 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1275] != 0.0) && (s.v[1310] != 0.0)) {
            s.store_scalar(987, 0.0);
        }

        if ((s.v[1275] != 0.0) && (s.v[1310] != 0.0)) {
            s.store_scalar(988, 0.0);
        }

        if ((s.v[1275] != 0.0) && (s.v[1310] != 0.0)) {
            s.store_scalar(933, 0.0);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) {
            s.store_mul_ad_rhs(989, 972, A::offset(s.ad_value(983), (-1.0)));
        }

        s.v[1311] = if (s.v[989] < 1e-5) { 1.0 } else { 0.0 };

        if (((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (s.v[1311] != 0.0)) {
            s.store_scalar(989, 0.0);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (s.v[1311] != 0.0)) {
            s.store_scalar(991, 1.0);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (!(s.v[1311] != 0.0))) {
            s.store_div_from_scalar_ad(991, 1.0, A::sqrt(A::offset(s.ad_value(989), 1.0)));
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) {
            s.store_mul_ad_rhs(990, 973, A::offset(s.ad_value(984), (-1.0)));
        }

        s.v[1312] = if (s.v[990] < 1e-5) { 1.0 } else { 0.0 };

        if (((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (s.v[1312] != 0.0)) {
            s.store_scalar(990, 0.0);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (s.v[1312] != 0.0)) {
            s.store_scalar(992, 1.0);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (!(s.v[1312] != 0.0))) {
            s.store_div_from_scalar_ad(992, 1.0, A::sqrt(A::offset(s.ad_value(990), 1.0)));
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) {
            s.store_sub_from_scalar(843, 1.0, 351);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) {
            s.store_mul_ad_lhs(985, A::mul(s.ad_value(930), s.ad_value(949)), 352);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) {
            s.store_mul(844, 843, 985);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) {
            s.store_mul_ad_lhs(985, A::mul(s.ad_value(930), s.ad_value(950)), 352);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) {
            s.store_mul(844, 843, 985);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) {
            s.store_mul_ad_lhs(986, A::mul(s.ad_value(930), s.ad_value(949)), 353);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) {
            s.store_mul_ad_lhs(987, A::mul(s.ad_value(986), A::offset(s.ad_value(983), (-1.0))), 991);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) {
            s.store_mul_ad_lhs(986, A::mul(s.ad_value(930), s.ad_value(950)), 353);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) {
            s.store_mul_ad_lhs(988, A::mul(s.ad_value(986), A::offset(s.ad_value(984), (-1.0))), 992);
        }

        s.v[1313] = if (p.p13 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) {
            s.store_scalar(933, 0.0);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (!(s.v[1313] != 0.0))) {
            s.store_offset_ad(843, A::div(A::add(s.ad_value(1087), s.ad_value(1088)), s.ad_value(354)), 1.0);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (!(s.v[1313] != 0.0))) {
            s.store_add(844, 989, 990);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (!(s.v[1313] != 0.0))) {
            s.store_sqrt_ad(846, A::add(A::square(s.ad_value(843)), A::scale(s.ad_value(844), 4.0)));
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (!(s.v[1313] != 0.0))) {
            s.store_scaled_add(845, 843, 846, 0.5);
        }

        s.v[1314] = if (s.v[845] < 0.1) { 1.0 } else { 0.0 };

        if ((((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (!(s.v[1313] != 0.0))) && (s.v[1314] != 0.0)) {
            s.store_scalar(993, 10.0);
        }

        if ((((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (!(s.v[1313] != 0.0))) && (!(s.v[1314] != 0.0))) {
            s.store_div_from_scalar(993, 1.0, 845);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (!(s.v[1313] != 0.0))) {
            s.store_mul(843, 351, 985);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (!(s.v[1313] != 0.0))) {
            s.store_mul_ad_lhs(933, A::mul(s.ad_value(843), A::sub(s.ad_value(983), s.ad_value(984))), 993);
        }

        s.v[1315] = if ((s.v[953] <= 0.0) && (s.v[954] <= 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) {
            s.store_scale(932, 298, p.p1043);
        }

        s.v[1316] = if ((s.v[316] - s.v[1087]) < 0.001) { 1.0 } else { 0.0 };

        if (((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (s.v[1316] != 0.0)) {
            s.store_scalar(844, 1000.0);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (s.v[1316] != 0.0)) {
            s.store_mul_ad_lhs(843, A::mul(A::div(A::neg(s.ad_value(1087)), s.ad_value(932)), s.ad_value(316)), 844);
        }

        s.v[1317] = if (s.v[843] > 100.0) { 1.0 } else { 0.0 };

        if ((((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (s.v[1316] != 0.0)) && (s.v[1317] != 0.0)) {
            s.store_scale_ad(844, A::offset(A::offset(s.ad_value(843), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1318] = if (s.v[843] < (-100.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (s.v[1316] != 0.0)) && (!(s.v[1317] != 0.0))) && (s.v[1318] != 0.0)) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if (((((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (s.v[1316] != 0.0)) && (!(s.v[1317] != 0.0))) && (!(s.v[1318] != 0.0))) {
            s.store_exp(844, 843);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (s.v[1316] != 0.0)) {
            s.store_mul(846, 974, 953);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (!(s.v[1316] != 0.0))) {
            s.store_div_from_scalar_ad(844, 1.0, A::sub(s.ad_value(316), s.ad_value(1087)));
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (!(s.v[1316] != 0.0))) {
            s.store_mul_ad_lhs(843, A::mul(A::div(A::neg(s.ad_value(1087)), s.ad_value(932)), s.ad_value(316)), 844);
        }

        s.v[1319] = if (s.v[843] > 100.0) { 1.0 } else { 0.0 };

        if ((((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (!(s.v[1316] != 0.0))) && (s.v[1319] != 0.0)) {
            s.store_scale_ad(844, A::offset(A::offset(s.ad_value(843), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1320] = if (s.v[843] < (-100.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (!(s.v[1316] != 0.0))) && (!(s.v[1319] != 0.0))) && (s.v[1320] != 0.0)) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if (((((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (!(s.v[1316] != 0.0))) && (!(s.v[1319] != 0.0))) && (!(s.v[1320] != 0.0))) {
            s.store_exp(844, 843);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (!(s.v[1316] != 0.0))) {
            s.store_mul(846, 974, 953);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) {
            s.store_scale(932, 299, p.p1043);
        }

        s.v[1321] = if ((s.v[317] - s.v[1088]) < 0.001) { 1.0 } else { 0.0 };

        if (((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (s.v[1321] != 0.0)) {
            s.store_scalar(844, 1000.0);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (s.v[1321] != 0.0)) {
            s.store_mul_ad_lhs(843, A::mul(A::div(A::neg(s.ad_value(1088)), s.ad_value(932)), s.ad_value(317)), 844);
        }

        s.v[1322] = if (s.v[843] > 100.0) { 1.0 } else { 0.0 };

        if ((((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (s.v[1321] != 0.0)) && (s.v[1322] != 0.0)) {
            s.store_scale_ad(844, A::offset(A::offset(s.ad_value(843), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1323] = if (s.v[843] < (-100.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (s.v[1321] != 0.0)) && (!(s.v[1322] != 0.0))) && (s.v[1323] != 0.0)) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if (((((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (s.v[1321] != 0.0)) && (!(s.v[1322] != 0.0))) && (!(s.v[1323] != 0.0))) {
            s.store_exp(844, 843);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (s.v[1321] != 0.0)) {
            s.store_mul(846, 975, 954);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (!(s.v[1321] != 0.0))) {
            s.store_div_from_scalar_ad(844, 1.0, A::sub(s.ad_value(317), s.ad_value(1088)));
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (!(s.v[1321] != 0.0))) {
            s.store_mul_ad_lhs(843, A::mul(A::div(A::neg(s.ad_value(1088)), s.ad_value(932)), s.ad_value(317)), 844);
        }

        s.v[1324] = if (s.v[843] > 100.0) { 1.0 } else { 0.0 };

        if ((((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (!(s.v[1321] != 0.0))) && (s.v[1324] != 0.0)) {
            s.store_scale_ad(844, A::offset(A::offset(s.ad_value(843), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1325] = if (s.v[843] < (-100.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (!(s.v[1321] != 0.0))) && (!(s.v[1324] != 0.0))) && (s.v[1325] != 0.0)) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if (((((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (!(s.v[1321] != 0.0))) && (!(s.v[1324] != 0.0))) && (!(s.v[1325] != 0.0))) {
            s.store_exp(844, 843);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (!(s.v[1321] != 0.0))) {
            s.store_mul(846, 975, 954);
        }

        if (!(s.v[1275] != 0.0)) {
            s.store_scalar(987, 0.0);
        }

        if (!(s.v[1275] != 0.0)) {
            s.store_scalar(988, 0.0);
        }

        if (!(s.v[1275] != 0.0)) {
            s.store_scalar(933, 0.0);
        }

        s.store_add_ad_rhs(203, 203, A::mul(s.ad_value(204), s.ad_value(430)));

        s.store_add_ad_rhs(207, 207, A::mul(s.ad_value(208), s.ad_value(430)));

        s.store_add_ad_rhs(243, 243, A::mul(s.ad_value(244), s.ad_value(430)));

        s.store_add_ad_rhs(246, 246, A::mul(s.ad_value(247), s.ad_value(430)));

        s.store_add_ad_rhs(250, 250, A::mul(s.ad_value(248), s.ad_value(430)));

        s.v[1326] = if ((p.p374 != 0.0) || (p.p375 != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1326] != 0.0) {
            s.store_sub(1075, 825, 824);
        }

        if (s.v[1326] != 0.0) {
            s.store_sub_ad(826, A::sub(A::scale(s.ad_value(408), p.p37), s.ad_value(942)), A::mul(s.ad_value(405), s.ad_value(943)));
        }

        if (s.v[1326] != 0.0) {
            s.store_offset_ad(846, A::add(A::sub(s.ad_value(826), s.ad_value(825)), s.ad_value(824)), (-0.02));
        }

        s.v[1327] = if (s.v[826] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1326] != 0.0) && (s.v[1327] != 0.0)) {
            s.store_sqrt_ad(843, A::sub(A::square(s.ad_value(846)), A::scale(s.ad_value(826), (4.0 * 0.02))));
        }

        if ((s.v[1326] != 0.0) && (!(s.v[1327] != 0.0))) {
            s.store_sqrt_ad(843, A::add(A::square(s.ad_value(846)), A::scale(s.ad_value(826), (4.0 * 0.02))));
        }

        if (s.v[1326] != 0.0) {
            s.store_sub_ad_rhs(812, 826, A::scale(A::add(s.ad_value(846), s.ad_value(843)), 0.5));
        }

        if (s.v[1326] != 0.0) {
            s.store_sub(1081, 826, 812);
        }

        s.v[1328] = if (s.v[1081] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1326] != 0.0) && (s.v[1328] != 0.0)) {
            s.store_scalar(1081, 0.0);
        }

        s.v[1329] = if (s.v[376] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1326] != 0.0) && (s.v[1329] != 0.0)) {
            s.store_scalar(1082, 0.0);
        }

        if ((s.v[1326] != 0.0) && (!(s.v[1329] != 0.0))) {
            s.store_sub_ad_lhs(843, A::sub(A::sub(s.ad_value(825), s.ad_value(875)), s.ad_value(812)), 841);
        }

        s.v[1330] = if (s.v[843] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1326] != 0.0) && (!(s.v[1329] != 0.0))) && (s.v[1330] != 0.0)) {
            s.store_div(844, 843, 376);
        }

        if (((s.v[1326] != 0.0) && (!(s.v[1329] != 0.0))) && (!(s.v[1330] != 0.0))) {
            s.store_mul_ad(844, A::scale(s.ad_value(376), 0.5), A::offset(A::sqrt(A::offset(A::div(A::div(A::scale(s.ad_value(843), 4.0), s.ad_value(376)), s.ad_value(376)), 1.0)), (-1.0)));
        }

        if ((s.v[1326] != 0.0) && (!(s.v[1329] != 0.0))) {
            s.store_sub_ad_lhs(1082, A::sub(s.ad_value(825), A::add(A::square(s.ad_value(844)), s.ad_value(824))), 826);
        }

        if (!(s.v[1326] != 0.0)) {
            s.store_scalar(826, 0.0);
        }

        if (!(s.v[1326] != 0.0)) {
            s.store_scalar(1075, 0.0);
        }

        if (!(s.v[1326] != 0.0)) {
            s.store_scalar(1081, 0.0);
        }

        if (!(s.v[1326] != 0.0)) {
            s.store_scalar(1082, 0.0);
        }

        if (p.p375 != 0.0) {
            s.store_mul(843, 832, 211);
        }

        if (p.p375 != 0.0) {
            s.store_div_ad_lhs(1028, A::sub(s.ad_value(825), A::scale(s.ad_value(408), p.p37)), 843);
        }

        s.v[1331] = if (s.v[1028] > 100.0) { 1.0 } else { 0.0 };

        if ((p.p375 != 0.0) && (s.v[1331] != 0.0)) {
            s.store_sub_ad_rhs(1078, 825, A::scale(s.ad_value(408), p.p37));
        }

        s.v[1332] = if (s.v[1028] < (-100.0)) { 1.0 } else { 0.0 };

        if (((p.p375 != 0.0) && (!(s.v[1331] != 0.0))) && (s.v[1332] != 0.0)) {
            s.store_scale(1078, 843, (((1.0 + 3.720075976e-44)) as f64).ln());
        }

        if (((p.p375 != 0.0) && (!(s.v[1331] != 0.0))) && (!(s.v[1332] != 0.0))) {
            s.store_exp(1029, 1028);
        }

        if (((p.p375 != 0.0) && (!(s.v[1331] != 0.0))) && (!(s.v[1332] != 0.0))) {
            s.store_mul_ad_rhs(1078, 843, A::ln(A::offset(s.ad_value(1029), 1.0)));
        }

        if (p.p375 != 0.0) {
            s.store_mul(845, 825, 1078);
        }

        if (p.p375 != 0.0) {
            s.store_scalar(854, s.v[369]);
        }

        if (p.p375 != 0.0) {
            s.store_scalar(855, s.v[370]);
        }

        if (p.p375 != 0.0) {
            s.store_sub_ad_lhs(846, A::mul(s.ad_value(203), s.ad_value(206)), 205);
        }

        if (p.p375 != 0.0) {
            s.store_mul(847, 205, 206);
        }

        if (p.p375 != 0.0) {
            s.store_mul_ad_rhs(848, 855, A::sub(A::add(s.ad_value(203), A::mul(s.ad_value(846), s.ad_value(1082))), A::mul(A::mul(s.ad_value(847), s.ad_value(1082)), s.ad_value(1082))));
        }

        s.v[1333] = if (s.v[848] > 100.0) { 1.0 } else { 0.0 };

        if ((p.p375 != 0.0) && (s.v[1333] != 0.0)) {
            s.store_scalar(849, 2.688117142e43);
        }

        s.v[1334] = if (s.v[848] < (-100.0)) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_13(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((p.p375 != 0.0) && (!(s.v[1333] != 0.0))) && (s.v[1334] != 0.0)) {
            s.store_scalar(849, 3.720075976e-44);
        }

        if (((p.p375 != 0.0) && (!(s.v[1333] != 0.0))) && (!(s.v[1334] != 0.0))) {
            s.store_exp(849, 848);
        }

        if (p.p375 != 0.0) {
            s.store_mul_ad_lhs(850, A::neg(s.ad_value(212)), 822);
        }

        if (p.p375 != 0.0) {
            s.store_offset_ad(851, A::square(s.ad_value(850)), 0.0002);
        }

        s.v[1335] = if (s.v[850] > 100.0) { 1.0 } else { 0.0 };

        if ((p.p375 != 0.0) && (s.v[1335] != 0.0)) {
            s.store_scalar(852, 2.688117142e43);
        }

        s.v[1336] = if (s.v[850] < (-100.0)) { 1.0 } else { 0.0 };

        if (((p.p375 != 0.0) && (!(s.v[1335] != 0.0))) && (s.v[1336] != 0.0)) {
            s.store_scalar(852, 3.720075976e-44);
        }

        if (((p.p375 != 0.0) && (!(s.v[1335] != 0.0))) && (!(s.v[1336] != 0.0))) {
            s.store_exp(852, 850);
        }

        if (p.p375 != 0.0) {
            s.store_offset(844, 852, (((-1.0)) + (0.0001)));
        }

        if (p.p375 != 0.0) {
            s.store_div_ad_lhs(853, A::sub(s.ad_value(844), s.ad_value(850)), 851);
        }

        if (p.p375 != 0.0) {
            s.store_offset(844, 852, (((-1.0)) + ((-0.0001))));
        }

        if (p.p375 != 0.0) {
            s.store_div_ad_lhs(853, A::sub(A::mul(s.ad_value(850), s.ad_value(852)), s.ad_value(844)), 851);
        }

        if (p.p375 != 0.0) {
            s.store_sub(843, 821, 375);
        }

        if (p.p375 != 0.0) {
            s.store_sqrt_ad(1026, A::offset(A::square(s.ad_value(843)), 0.0001));
        }

        if (p.p375 != 0.0) {
            s.store_mul(845, 821, 1026);
        }

        if (p.p375 != 0.0) {
            s.copy_ad(964, 372);
        }

        if (p.p375 != 0.0) {
            s.copy_ad(965, 373);
        }

        if (p.p375 != 0.0) {
            s.copy_ad(855, 374);
        }

        if (p.p375 != 0.0) {
            s.store_sub_ad_lhs(846, A::mul(s.ad_value(207), s.ad_value(210)), 209);
        }

        if (p.p375 != 0.0) {
            s.store_mul(847, 209, 210);
        }

        if (p.p375 != 0.0) {
            s.store_mul_ad_rhs(848, 855, A::sub(A::add(s.ad_value(207), A::mul(s.ad_value(846), s.ad_value(1026))), A::mul(A::mul(s.ad_value(847), s.ad_value(1026)), s.ad_value(1026))));
        }

        s.v[1337] = if (s.v[848] > 100.0) { 1.0 } else { 0.0 };

        if ((p.p375 != 0.0) && (s.v[1337] != 0.0)) {
            s.store_scalar(849, 2.688117142e43);
        }

        s.v[1338] = if (s.v[848] < (-100.0)) { 1.0 } else { 0.0 };

        if (((p.p375 != 0.0) && (!(s.v[1337] != 0.0))) && (s.v[1338] != 0.0)) {
            s.store_scalar(849, 3.720075976e-44);
        }

        if (((p.p375 != 0.0) && (!(s.v[1337] != 0.0))) && (!(s.v[1338] != 0.0))) {
            s.store_exp(849, 848);
        }

        if (p.p375 != 0.0) {
            s.store_sub(843, 820, 375);
        }

        if (p.p375 != 0.0) {
            s.store_sqrt_ad(1027, A::offset(A::square(s.ad_value(843)), 0.0001));
        }

        if (p.p375 != 0.0) {
            s.store_mul(845, 820, 1027);
        }

        if (p.p375 != 0.0) {
            s.store_mul_ad_rhs(848, 855, A::sub(A::add(s.ad_value(207), A::mul(s.ad_value(846), s.ad_value(1027))), A::mul(A::mul(s.ad_value(847), s.ad_value(1027)), s.ad_value(1027))));
        }

        s.v[1339] = if (s.v[848] > 100.0) { 1.0 } else { 0.0 };

        if ((p.p375 != 0.0) && (s.v[1339] != 0.0)) {
            s.store_scalar(849, 2.688117142e43);
        }

        s.v[1340] = if (s.v[848] < (-100.0)) { 1.0 } else { 0.0 };

        if (((p.p375 != 0.0) && (!(s.v[1339] != 0.0))) && (s.v[1340] != 0.0)) {
            s.store_scalar(849, 3.720075976e-44);
        }

        if (((p.p375 != 0.0) && (!(s.v[1339] != 0.0))) && (!(s.v[1340] != 0.0))) {
            s.store_exp(849, 848);
        }

        s.v[1341] = if ((p.p374 != 0.0) && (s.v[37] != 2.0)) { 1.0 } else { 0.0 };

        if (s.v[1341] != 0.0) {
            s.store_scalar(1077, s.v[345]);
        }

        if (s.v[1341] != 0.0) {
            s.copy_ad(1076, 1082);
        }

        if (s.v[1341] != 0.0) {
            s.store_scalar(843, p.p396);
        }

        if (s.v[1341] != 0.0) {
            s.store_offset_ad(844, A::sub(s.ad_value(843), s.ad_value(1076)), (-p.p397));
        }

        if (s.v[1341] != 0.0) {
            s.store_sqrt_ad(846, A::add(A::square(s.ad_value(844)), A::scale(s.ad_value(843), (4.0 * p.p397))));
        }

        if (s.v[1341] != 0.0) {
            s.store_sub_ad_rhs(1080, 843, A::scale(A::add(s.ad_value(844), s.ad_value(846)), 0.5));
        }

        if (s.v[1341] != 0.0) {
            s.copy_ad(1076, 1080);
        }

        if (s.v[1341] != 0.0) {
            s.store_scaled_offset(843, 1076, (-p.p381), 1.0 / (p.p382));
        }

        s.v[1342] = if (s.v[843] > 100.0) { 1.0 } else { 0.0 };

        if ((s.v[1341] != 0.0) && (s.v[1342] != 0.0)) {
            s.store_scale_ad(844, A::offset(A::offset(s.ad_value(843), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1343] = if (s.v[843] < (-100.0)) { 1.0 } else { 0.0 };

        if (((s.v[1341] != 0.0) && (!(s.v[1342] != 0.0))) && (s.v[1343] != 0.0)) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if (((s.v[1341] != 0.0) && (!(s.v[1342] != 0.0))) && (!(s.v[1343] != 0.0))) {
            s.store_exp(844, 843);
        }

        if (s.v[1341] != 0.0) {
            s.store_scale_ad(1078, A::ln(A::offset(s.ad_value(844), 1.0)), p.p382);
        }

        s.v[1344] = if (p.p386 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1341] != 0.0) && (s.v[1344] != 0.0)) {
            s.store_sub_from_scalar_ad(843, 1.0, A::scale(s.ad_value(1076), 1.0 / (p.p386)));
        }

        if ((s.v[1341] != 0.0) && (!(s.v[1344] != 0.0))) {
            s.store_scalar(843, 1.0);
        }

        s.v[1345] = if (s.v[843] < 0.01) { 1.0 } else { 0.0 };

        if ((s.v[1341] != 0.0) && (s.v[1345] != 0.0)) {
            s.store_scalar(843, 0.01);
        }

        if (s.v[1341] != 0.0) {
            s.store_mul_ad_lhs(844, A::scale(A::offset(A::scale(s.ad_value(893), (s.v[892] * 1.0 / (p.p23))), (p.p28 / p.p3)), p.p1035), 1077);
        }

        if (s.v[1341] != 0.0) {
            s.store_scalar(845, (p.p1036 * p.p376));
        }

        if (s.v[1341] != 0.0) {
            s.copy_ad(846, 243);
        }

        if (s.v[1341] != 0.0) {
            s.copy_ad(847, 245);
        }

        if (s.v[1341] != 0.0) {
            s.store_div_ad_lhs(849, A::mul(s.ad_value(845), A::sub(s.ad_value(846), A::mul(s.ad_value(847), s.ad_value(1076)))), 843);
        }

        s.v[1346] = if (s.v[849] > 100.0) { 1.0 } else { 0.0 };

        if ((s.v[1341] != 0.0) && (s.v[1346] != 0.0)) {
            s.store_scale_ad(848, A::offset(A::offset(s.ad_value(849), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1347] = if (s.v[849] < (-100.0)) { 1.0 } else { 0.0 };

        if (((s.v[1341] != 0.0) && (!(s.v[1346] != 0.0))) && (s.v[1347] != 0.0)) {
            s.store_scalar(848, 3.720075976e-44);
        }

        if (((s.v[1341] != 0.0) && (!(s.v[1346] != 0.0))) && (!(s.v[1347] != 0.0))) {
            s.store_exp(848, 849);
        }

        if (s.v[1341] != 0.0) {
            s.copy_ad(1076, 1081);
        }

        if (s.v[1341] != 0.0) {
            s.store_scalar(843, p.p396);
        }

        if (s.v[1341] != 0.0) {
            s.store_offset_ad(844, A::sub(s.ad_value(843), s.ad_value(1076)), (-p.p397));
        }

        if (s.v[1341] != 0.0) {
            s.store_sqrt_ad(846, A::add(A::square(s.ad_value(844)), A::scale(s.ad_value(843), (4.0 * p.p397))));
        }

        if (s.v[1341] != 0.0) {
            s.store_sub_ad_rhs(1080, 843, A::scale(A::add(s.ad_value(844), s.ad_value(846)), 0.5));
        }

        if (s.v[1341] != 0.0) {
            s.copy_ad(1076, 1080);
        }

        if (s.v[1341] != 0.0) {
            s.store_scaled_sub(843, 826, 1075, 1.0 / (p.p387));
        }

        s.v[1348] = if (s.v[843] > 100.0) { 1.0 } else { 0.0 };

        if ((s.v[1341] != 0.0) && (s.v[1348] != 0.0)) {
            s.store_scale_ad(844, A::offset(A::offset(s.ad_value(843), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1349] = if (s.v[843] < (-100.0)) { 1.0 } else { 0.0 };

        if (((s.v[1341] != 0.0) && (!(s.v[1348] != 0.0))) && (s.v[1349] != 0.0)) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if (((s.v[1341] != 0.0) && (!(s.v[1348] != 0.0))) && (!(s.v[1349] != 0.0))) {
            s.store_exp(844, 843);
        }

        if (s.v[1341] != 0.0) {
            s.store_scale_ad(1078, A::ln(A::offset(s.ad_value(844), 1.0)), p.p387);
        }

        s.v[1350] = if (p.p391 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1341] != 0.0) && (s.v[1350] != 0.0)) {
            s.store_sub_from_scalar_ad(843, 1.0, A::scale(s.ad_value(1076), 1.0 / (p.p391)));
        }

        if ((s.v[1341] != 0.0) && (!(s.v[1350] != 0.0))) {
            s.store_scalar(843, 1.0);
        }

        s.v[1351] = if (s.v[843] < 0.01) { 1.0 } else { 0.0 };

        if ((s.v[1341] != 0.0) && (s.v[1351] != 0.0)) {
            s.store_scalar(843, 0.01);
        }

        if (s.v[1341] != 0.0) {
            s.store_mul_ad_lhs(844, A::scale(A::offset(A::scale(s.ad_value(893), (s.v[892] * 1.0 / (p.p23))), (p.p28 / p.p3)), p.p1037), 1077);
        }

        if (s.v[1341] != 0.0) {
            s.store_scalar(845, (p.p1038 * p.p376));
        }

        if (s.v[1341] != 0.0) {
            s.copy_ad(846, 246);
        }

        if (s.v[1341] != 0.0) {
            s.copy_ad(847, 249);
        }

        if (s.v[1341] != 0.0) {
            s.store_div_ad_lhs(849, A::mul(s.ad_value(845), A::sub(s.ad_value(846), A::mul(s.ad_value(847), s.ad_value(1076)))), 843);
        }

        s.v[1352] = if (s.v[849] > 100.0) { 1.0 } else { 0.0 };

        if ((s.v[1341] != 0.0) && (s.v[1352] != 0.0)) {
            s.store_scale_ad(848, A::offset(A::offset(s.ad_value(849), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1353] = if (s.v[849] < (-100.0)) { 1.0 } else { 0.0 };

        if (((s.v[1341] != 0.0) && (!(s.v[1352] != 0.0))) && (s.v[1353] != 0.0)) {
            s.store_scalar(848, 3.720075976e-44);
        }

        if (((s.v[1341] != 0.0) && (!(s.v[1352] != 0.0))) && (!(s.v[1353] != 0.0))) {
            s.store_exp(848, 849);
        }

        if (s.v[1341] != 0.0) {
            s.store_offset(1127, 826, p.p1033);
        }

        s.v[1355] = if (((((p.p374 != 0.0) && (s.v[37] != 2.0)) && (s.v[399] != 0.0)) && (p.p27 > 0.0)) && (s.v[1114] < s.v[1127])) { 1.0 } else { 0.0 };

        if (s.v[1355] != 0.0) {
            s.store_sub(843, 1114, 1127);
        }

        if (s.v[1355] != 0.0) {
            s.store_sqrt_ad(844, A::offset(A::square(s.ad_value(843)), 0.0001));
        }

        if (s.v[1355] != 0.0) {
            s.store_scale_ad(1113, A::offset(A::sub(s.ad_value(844), s.ad_value(843)), (-0.01)), 0.5);
        }

        if (s.v[1355] != 0.0) {
            s.store_scalar(854, (if (p.p37 == 1.0) { p.p1039 } else { p.p1040 }));
        }

        if (s.v[1355] != 0.0) {
            s.store_scalar(855, (if (p.p37 == 1.0) { p.p1041 } else { p.p1042 }));
        }

        if (s.v[1355] != 0.0) {
            s.store_mul(845, 1114, 1113);
        }

        if (s.v[1355] != 0.0) {
            s.store_sub_ad_lhs(846, A::mul(s.ad_value(250), s.ad_value(252)), 251);
        }

        if (s.v[1355] != 0.0) {
            s.store_mul(847, 251, 252);
        }

        if (s.v[1355] != 0.0) {
            s.store_mul_ad(848, A::scale(A::neg(s.ad_value(855)), p.p376), A::sub(A::add(s.ad_value(250), A::mul(s.ad_value(846), s.ad_value(1113))), A::mul(A::mul(s.ad_value(847), s.ad_value(1113)), s.ad_value(1113))));
        }

        s.v[1356] = if (s.v[848] > 100.0) { 1.0 } else { 0.0 };

        if ((s.v[1355] != 0.0) && (s.v[1356] != 0.0)) {
            s.store_scalar(849, 2.688117142e43);
        }

        s.v[1357] = if (s.v[848] < (-100.0)) { 1.0 } else { 0.0 };

        if (((s.v[1355] != 0.0) && (!(s.v[1356] != 0.0))) && (s.v[1357] != 0.0)) {
            s.store_scalar(849, 3.720075976e-44);
        }

        if (((s.v[1355] != 0.0) && (!(s.v[1356] != 0.0))) && (!(s.v[1357] != 0.0))) {
            s.store_exp(849, 848);
        }

        if (s.v[1355] != 0.0) {
            s.store_scale(854, 854, (p.p27 * s.v[345]));
        }

        s.v[1358] = if (s.v[37] != 2.0) { 1.0 } else { 0.0 };

        s.v[1359] = if (p.p44 == 0.0) { 1.0 } else { 0.0 };

        s.v[1360] = if (s.v[201] <= 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) {
            s.store_sub_ad(966, A::mul(s.ad_value(275), A::offset(A::scale(s.ad_value(430), p.p308), 1.0)), A::scale(s.ad_value(276), 1.0 / (s.v[892])));
        }

        if (((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) {
            s.store_scale(843, 277, s.v[892]);
        }

        if (((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) {
            s.store_div_ad(844, A::mul(s.ad_value(278), s.ad_value(843)), A::offset(s.ad_value(843), 1.0));
        }

        if (((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) {
            s.store_div_from_scalar_ad(843, 1.0, A::offset(A::mul(s.ad_value(279), s.ad_value(875)), 1.0));
        }

        if (((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) {
            s.store_add(846, 843, 280);
        }

        if (((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) {
            s.store_mul(845, 830, 846);
        }

        if (((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) {
            s.store_div_from_scalar_ad(846, 1.0, A::offset(A::mul(s.ad_value(281), s.ad_value(822)), 1.0));
        }

        if (((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) {
            s.store_mul_ad_lhs(967, A::mul(s.ad_value(844), s.ad_value(845)), 846);
        }

        if (((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) {
            s.store_add(921, 966, 967);
        }

        if (((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) {
            s.store_sub(969, 822, 921);
        }

        if (((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) {
            s.store_add_ad(843, A::add(s.ad_value(274), A::mul(s.ad_value(273), s.ad_value(969))), A::mul(A::mul(s.ad_value(202), s.ad_value(969)), s.ad_value(969)));
        }

        s.v[1361] = if (s.v[843] < 1e-5) { 1.0 } else { 0.0 };

        if ((((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) && (s.v[1361] != 0.0)) {
            s.store_scalar(843, 1e-5);
        }

        if (((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) {
            s.store_add_ad_rhs(843, 885, A::mul(A::mul(s.ad_value(267), s.ad_value(398)), s.ad_value(933)));
        }

        s.v[1365] = if (s.v[201] <= 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) {
            s.store_sub_ad(966, A::mul(s.ad_value(275), A::offset(A::scale(s.ad_value(430), p.p308), 1.0)), A::scale(s.ad_value(276), 1.0 / (s.v[892])));
        }

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) {
            s.store_scale(843, 277, s.v[892]);
        }

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) {
            s.store_div_ad(844, A::mul(s.ad_value(278), s.ad_value(843)), A::offset(s.ad_value(843), 1.0));
        }

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) {
            s.store_div_from_scalar_ad(843, 1.0, A::offset(A::mul(s.ad_value(279), s.ad_value(875)), 1.0));
        }

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) {
            s.store_add(846, 843, 280);
        }

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) {
            s.store_mul(845, 830, 846);
        }

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) {
            s.store_div_from_scalar_ad(846, 1.0, A::offset(A::mul(s.ad_value(281), s.ad_value(822)), 1.0));
        }

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) {
            s.store_mul_ad_lhs(967, A::mul(s.ad_value(844), s.ad_value(845)), 846);
        }

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) {
            s.store_add(921, 966, 967);
        }

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) {
            s.store_sub(969, 822, 921);
        }

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) {
            s.store_add_ad(843, A::add(s.ad_value(274), A::mul(s.ad_value(273), s.ad_value(969))), A::mul(A::mul(s.ad_value(202), s.ad_value(969)), s.ad_value(969)));
        }

        s.v[1366] = if (s.v[843] < 1e-5) { 1.0 } else { 0.0 };

        if ((((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) && (s.v[1366] != 0.0)) {
            s.store_scalar(843, 1e-5);
        }

    }

    pub(super) fn stamp_reactive_block_14(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) {
            s.copy_ad(843, 885);
        }

        if ((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) {
            s.store_scale_ad(843, A::add(s.ad_value(269), A::scale(s.ad_value(268), s.v[892])), 1.0 / (s.v[892]));
        }

        if ((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) {
            s.store_mul_ad_rhs(1105, 270, A::offset(A::scale(s.ad_value(430), p.p320), 1.0));
        }

        s.v[1370] = if (s.v[398] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (s.v[1370] != 0.0)) {
            s.store_sub(844, 1105, 1088);
        }

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1370] != 0.0))) {
            s.store_sub(844, 1105, 1087);
        }

        if ((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) {
            s.store_offset(845, 272, (-1.0));
        }

        s.v[1371] = if (s.v[844] <= 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (s.v[1371] != 0.0)) {
            s.store_scalar(846, 0.0);
        }

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1371] != 0.0))) {
            s.store_mul_ad(846, A::neg(s.ad_value(271)), A::pow(s.ad_value(844), s.ad_value(845)));
        }

        s.v[1372] = if (s.v[846] > 100.0) { 1.0 } else { 0.0 };

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (s.v[1372] != 0.0)) {
            s.store_scalar(847, 2.688117142e43);
        }

        s.v[1373] = if (s.v[846] < (-100.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1372] != 0.0))) && (s.v[1373] != 0.0)) {
            s.store_scalar(847, 3.720075976e-44);
        }

        if ((((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1372] != 0.0))) && (!(s.v[1373] != 0.0))) {
            s.store_exp(847, 846);
        }

        s.v[1374] = if ((s.v[399] == 0.0) || (s.v[399] == 2.0)) { 1.0 } else { 0.0 };

        s.v[1375] = if (s.v[156] < 0.001) { 1.0 } else { 0.0 };

        s.v[1376] = if (s.v[50] <= 0.001) { 1.0 } else { 0.0 };

        if ((((s.v[1358] != 0.0) && (!(s.v[1374] != 0.0))) && (s.v[1375] != 0.0)) && (s.v[1376] != 0.0)) {
            s.store_scalar(843, (1.0 / 0.001));
        }

        if ((((s.v[1358] != 0.0) && (!(s.v[1374] != 0.0))) && (s.v[1375] != 0.0)) && (!(s.v[1376] != 0.0))) {
            s.store_scalar(843, (1.0 / s.v[50]));
        }

        s.v[1377] = if (p.p39 > 1.0) { 1.0 } else { 0.0 };

        if (s.v[1377] != 0.0) {
            s.store_mul(852, 230, 49);
        }

        if (s.v[1377] != 0.0) {
            s.store_mul(843, 852, 880);
        }

        if (s.v[1377] != 0.0) {
            s.store_mul_ad_rhs(81, 229, A::add(s.ad_value(843), s.ad_value(1086)));
        }

        s.v[1378] = if (p.p3 != 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1377] != 0.0) && (s.v[1378] != 0.0)) {
            s.store_scale(81, 81, p.p3);
        }

        s.v[1379] = if (p.p39 == 2.0) { 1.0 } else { 0.0 };

        if ((s.v[1377] != 0.0) && (s.v[1379] != 0.0)) {
            s.store_add(854, 64, 81);
        }

        if ((s.v[1377] != 0.0) && (s.v[1379] != 0.0)) {
            s.store_div_ad_lhs(81, A::mul(s.ad_value(64), s.ad_value(81)), 854);
        }

        if (!(s.v[1377] != 0.0)) {
            s.store_scalar(81, 0.0);
        }

        s.v[1380] = if (p.p429 == 0.0) { 1.0 } else { 0.0 };

        s.v[1385] = if (p.p429 == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_scalar(887, 0.0);
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_sub(843, 821, 375);
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_sqrt_ad(844, A::offset(A::square(s.ad_value(843)), 0.0001));
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_scaled_add(1026, 843, 844, 0.5);
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_offset_ad(843, A::mul(s.ad_value(183), s.ad_value(1026)), 1.0);
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_mul_ad_lhs(844, A::neg(s.ad_value(184)), 818);
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_add_ad(845, A::add(A::div_from_scalar(1.0, s.ad_value(843)), s.ad_value(844)), A::mul(s.ad_value(185), A::sub(s.ad_value(897), s.ad_value(941))));
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_add_ad_rhs(846, 845, A::sqrt(A::offset(A::square(s.ad_value(845)), 0.01)));
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_scale(847, 1096, 0.5);
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_sub(843, 820, 375);
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_sqrt_ad(844, A::offset(A::square(s.ad_value(843)), 0.0001));
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_scaled_add(1027, 843, 844, 0.5);
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_offset_ad(843, A::mul(s.ad_value(183), s.ad_value(1027)), 1.0);
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_mul_ad_lhs(844, A::neg(s.ad_value(184)), 817);
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_add_ad(845, A::add(A::div_from_scalar(1.0, s.ad_value(843)), s.ad_value(844)), A::mul(s.ad_value(185), A::sub(s.ad_value(897), s.ad_value(941))));
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_add_ad_rhs(846, 845, A::sqrt(A::offset(A::square(s.ad_value(845)), 0.01)));
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_scale(847, 1095, 0.5);
        }

        s.store_mul_ad_rhs(844, 875, A::sub_from_scalar(1.0, A::div(A::mul(A::scale(s.ad_value(860), 0.5), s.ad_value(876)), s.ad_value(890))));

        s.v[1389] = if (p.p3 != 1.0) { 1.0 } else { 0.0 };

        if (s.v[1389] != 0.0) {
            s.store_scale(885, 885, p.p3);
        }

        if (s.v[1389] != 0.0) {
            s.store_scale(933, 933, p.p3);
        }

        if (s.v[1389] != 0.0) {
            s.store_scale(78, 78, p.p3);
        }

        s.store_ad(83, &A::scale(A::constant(A::ddx_projection(&s.ad_value(885), Some(9), None)), p.p37));

        s.v[1390] = if (s.v[398] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1390] != 0.0) {
            s.store_ad(84, &A::scale(A::constant(A::ddx_projection(&s.ad_value(885), Some(7), None)), p.p37));
        }

        if (!(s.v[1390] != 0.0)) {
            s.store_ad(84, &A::scale(A::constant(A::ddx_projection(&s.ad_value(885), Some(8), None)), p.p37));
        }

        s.store_ad(85, &A::scale(A::constant(A::ddx_projection(&s.ad_value(885), Some(5), None)), p.p37));

        s.store_scale(842, 396, ((((s.v[332] / p.p23) * p.p3) * s.v[331]) + p.p26));

        s.store_scale(981, 396, (p.p361 * ((((s.v[332] / p.p23) * p.p3) * s.v[365]) + p.p26)));

        s.store_scale(1115, 396, p.p27);

        s.store_scale(1116, 396, (p.p361 * p.p27));

        s.store_sub(830, 825, 1073);

        s.store_mul(853, 1059, 832);

        s.store_div_ad_lhs(809, A::mul(s.ad_value(384), s.ad_value(830)), 853);

        s.store_mul_ad_lhs(1016, A::mul(s.ad_value(1059), s.ad_value(363)), 832);

        s.store_mul_ad_lhs(1017, A::mul(s.ad_value(1059), s.ad_value(364)), 832);

        s.v[1391] = if (p.p42 == 0.0) { 1.0 } else { 0.0 };

        s.v[1392] = if ((s.v[809] > (-100.0)) && (s.v[809] < 100.0)) { 1.0 } else { 0.0 };

        if ((s.v[1391] != 0.0) && (s.v[1392] != 0.0)) {
            s.store_mul_ad(810, A::exp(s.ad_value(809)), A::exp(s.ad_value(809)));
        }

        if ((s.v[1391] != 0.0) && (s.v[1392] != 0.0)) {
            s.store_mul_ad_rhs(810, 810, A::exp(A::neg(A::div(s.ad_value(324), s.ad_value(1016)))));
        }

        if ((s.v[1391] != 0.0) && (s.v[1392] != 0.0)) {
            s.store_mul_ad_rhs(875, 1016, {
                if ((1.0 + s.v[810]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(810), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        s.v[1393] = if (p.p27 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1391] != 0.0) && (s.v[1392] != 0.0)) && (s.v[1393] != 0.0)) {
            s.store_mul_ad_rhs(1117, 810, A::exp(A::div(A::div_from_scalar((-p.p1033), s.ad_value(1017)), A::square(s.ad_value(832)))));
        }

        if (((s.v[1391] != 0.0) && (s.v[1392] != 0.0)) && (s.v[1393] != 0.0)) {
            s.store_mul_ad_rhs(1118, 1017, {
                if ((1.0 + s.v[1117]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(1117), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        s.v[1394] = if (p.p42 == 1.0) { 1.0 } else { 0.0 };

        s.v[1395] = if ((s.v[809] > (-100.0)) && (s.v[809] < 100.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1391] != 0.0)) && (s.v[1394] != 0.0)) && (s.v[1395] != 0.0)) {
            s.store_exp_ad(810, A::div(s.ad_value(809), A::mul(s.ad_value(384), s.ad_value(363))));
        }

        if (((!(s.v[1391] != 0.0)) && (s.v[1394] != 0.0)) && (s.v[1395] != 0.0)) {
            s.store_mul_ad_rhs(810, 810, A::exp(A::neg(A::div(s.ad_value(324), s.ad_value(1016)))));
        }

        if (((!(s.v[1391] != 0.0)) && (s.v[1394] != 0.0)) && (s.v[1395] != 0.0)) {
            s.store_mul_ad_rhs(875, 1016, {
                if ((1.0 + s.v[810]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(810), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        s.v[1396] = if (p.p27 > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1391] != 0.0)) && (s.v[1394] != 0.0)) && (s.v[1395] != 0.0)) && (s.v[1396] != 0.0)) {
            s.store_mul_ad_rhs(1117, 810, A::exp(A::div(A::div_from_scalar((-p.p1033), s.ad_value(1017)), A::square(s.ad_value(832)))));
        }

        if ((((!(s.v[1391] != 0.0)) && (s.v[1394] != 0.0)) && (s.v[1395] != 0.0)) && (s.v[1396] != 0.0)) {
            s.store_mul_ad_rhs(1118, 1017, {
                if ((1.0 + s.v[1117]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(1117), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) {
            s.store_div_ad_lhs(809, A::mul(s.ad_value(388), A::sub(s.ad_value(830), s.ad_value(324))), 1016);
        }

        if ((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) {
            s.store_div_ad_lhs(833, A::sub(s.ad_value(390), A::mul(A::sub_from_scalar(1.0, s.ad_value(388)), A::sub(s.ad_value(830), s.ad_value(324)))), 1016);
        }

        s.v[1397] = if (s.v[809] > 100.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (s.v[1397] != 0.0)) {
            s.store_sub(875, 830, 324);
        }

        s.v[1398] = if (s.v[833] > 100.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1397] != 0.0))) && (s.v[1398] != 0.0)) {
            s.store_div_ad_lhs(843, A::sub(A::sub(s.ad_value(830), s.ad_value(324)), s.ad_value(390)), 1016);
        }

        if ((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1397] != 0.0))) && (s.v[1398] != 0.0)) {
            s.store_exp(810, 843);
        }

        if ((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1397] != 0.0))) && (s.v[1398] != 0.0)) {
            s.store_mul_ad_lhs(875, A::div(A::mul(s.ad_value(832), s.ad_value(1140)), s.ad_value(396)), 810);
        }

        if ((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1397] != 0.0))) && (!(s.v[1398] != 0.0))) {
            s.store_exp(810, 809);
        }

        if ((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1397] != 0.0))) && (!(s.v[1398] != 0.0))) {
            s.store_mul_ad_rhs(844, 1016, {
                if ((1.0 + s.v[810]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(810), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1397] != 0.0))) && (!(s.v[1398] != 0.0))) {
            s.store_mul_ad(857, A::mul(A::div(A::neg(s.ad_value(396)), A::mul(s.ad_value(832), s.ad_value(1140))), A::exp(s.ad_value(833))), A::sub_from_scalar(1.0, s.ad_value(388)));
        }

        if ((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1397] != 0.0))) && (!(s.v[1398] != 0.0))) {
            s.store_sub_ad_rhs(845, 388, A::div(A::mul(s.ad_value(1016), s.ad_value(857)), A::sub_from_scalar(1.0, s.ad_value(388))));
        }

        if ((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1397] != 0.0))) && (!(s.v[1398] != 0.0))) {
            s.store_div(875, 844, 845);
        }

        s.v[1399] = if (p.p27 > 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (s.v[1399] != 0.0)) {
            s.store_div_ad_lhs(1119, A::mul(s.ad_value(388), A::offset(A::sub(s.ad_value(830), s.ad_value(324)), (-p.p1033))), 1017);
        }

        if (((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (s.v[1399] != 0.0)) {
            s.store_div_ad_lhs(1120, A::sub(s.ad_value(390), A::mul(A::sub_from_scalar(1.0, s.ad_value(388)), A::offset(A::sub(s.ad_value(830), s.ad_value(324)), (-p.p1033)))), 1017);
        }

        s.v[1400] = if (s.v[1119] > 100.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (s.v[1399] != 0.0)) && (s.v[1400] != 0.0)) {
            s.store_offset_ad(1118, A::sub(s.ad_value(830), s.ad_value(324)), (-p.p1033));
        }

        s.v[1401] = if (s.v[1120] > 100.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (s.v[1399] != 0.0)) && (!(s.v[1400] != 0.0))) && (s.v[1401] != 0.0)) {
            s.store_div_ad_lhs(843, A::offset(A::sub(A::sub(s.ad_value(830), s.ad_value(324)), s.ad_value(390)), (-p.p1033)), 1017);
        }

        if (((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (s.v[1399] != 0.0)) && (!(s.v[1400] != 0.0))) && (s.v[1401] != 0.0)) {
            s.store_exp(1117, 843);
        }

        if (((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (s.v[1399] != 0.0)) && (!(s.v[1400] != 0.0))) && (s.v[1401] != 0.0)) {
            s.store_mul_ad_lhs(1118, A::div(A::mul(s.ad_value(832), s.ad_value(1140)), s.ad_value(396)), 1117);
        }

        if (((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (s.v[1399] != 0.0)) && (!(s.v[1400] != 0.0))) && (!(s.v[1401] != 0.0))) {
            s.store_exp(1117, 1119);
        }

        if (((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (s.v[1399] != 0.0)) && (!(s.v[1400] != 0.0))) && (!(s.v[1401] != 0.0))) {
            s.store_mul_ad_rhs(844, 1017, {
                if ((1.0 + s.v[1117]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(1117), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (s.v[1399] != 0.0)) && (!(s.v[1400] != 0.0))) && (!(s.v[1401] != 0.0))) {
            s.store_mul_ad(857, A::mul(A::div(A::neg(s.ad_value(396)), A::mul(s.ad_value(832), s.ad_value(1140))), A::exp(s.ad_value(1120))), A::sub_from_scalar(1.0, s.ad_value(388)));
        }

        if (((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (s.v[1399] != 0.0)) && (!(s.v[1400] != 0.0))) && (!(s.v[1401] != 0.0))) {
            s.store_sub_ad_rhs(845, 388, A::div(A::mul(s.ad_value(1017), s.ad_value(857)), A::sub_from_scalar(1.0, s.ad_value(388))));
        }

        if (((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (s.v[1399] != 0.0)) && (!(s.v[1400] != 0.0))) && (!(s.v[1401] != 0.0))) {
            s.store_div(1118, 844, 845);
        }

        s.copy_ad(829, 1073);

        s.copy_ad(828, 1054);

        s.copy_ad(841, 1044);

        s.v[1402] = if (p.p61 == 2.0) { 1.0 } else { 0.0 };

        s.v[1403] = if (s.v[37] == 2.0) { 1.0 } else { 0.0 };

        if ((s.v[1402] != 0.0) && (s.v[1403] != 0.0)) {
            s.store_scalar(938, 0.0);
        }

        if ((s.v[1402] != 0.0) && (s.v[1403] != 0.0)) {
            s.store_scalar(937, 0.0);
        }

        if ((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) {
            s.store_add_ad_lhs(826, A::sub(A::sub(s.ad_value(829), s.ad_value(942)), A::mul(s.ad_value(405), s.ad_value(828))), 324);
        }

        if ((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) {
            s.store_offset_ad(813, A::add(A::sub(s.ad_value(826), s.ad_value(825)), s.ad_value(841)), (-0.08));
        }

        s.v[1404] = if (s.v[826] <= 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) && (s.v[1404] != 0.0)) {
            s.store_sqrt_ad(843, A::sub(A::square(s.ad_value(813)), A::scale(s.ad_value(826), (4.0 * 0.08))));
        }

        if (((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) && (!(s.v[1404] != 0.0))) {
            s.store_sqrt_ad(843, A::add(A::square(s.ad_value(813)), A::scale(s.ad_value(826), (4.0 * 0.08))));
        }

        if ((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) {
            s.store_sub_ad_rhs(812, 826, A::scale(A::add(s.ad_value(813), s.ad_value(843)), 0.5));
        }

        if ((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) {
            s.store_mul_ad_rhs(938, 981, A::sub(s.ad_value(812), s.ad_value(826)));
        }

        s.v[1405] = if (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) && (s.v[1405] != 0.0)) {
            s.store_offset(1127, 826, p.p1033);
        }

        if (((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) && (s.v[1405] != 0.0)) {
            s.store_scalar(1139, 0.08);
        }

        if (((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) && (s.v[1405] != 0.0)) {
            s.store_sub_ad_lhs(813, A::add(A::sub(s.ad_value(1127), s.ad_value(1125)), s.ad_value(841)), 1139);
        }

        s.v[1406] = if (s.v[1127] <= 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) && (s.v[1405] != 0.0)) && (s.v[1406] != 0.0)) {
            s.store_sqrt_ad(843, A::sub(A::square(s.ad_value(813)), A::mul(A::scale(s.ad_value(1139), 100.0), s.ad_value(1127))));
        }

        if ((((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) && (s.v[1405] != 0.0)) && (!(s.v[1406] != 0.0))) {
            s.store_sqrt_ad(843, A::add(A::square(s.ad_value(813)), A::mul(A::scale(s.ad_value(1139), 100.0), s.ad_value(1127))));
        }

        if (((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) && (s.v[1405] != 0.0)) {
            s.store_sub_ad_rhs(1128, 1127, A::scale(A::add(s.ad_value(813), s.ad_value(843)), 0.5));
        }

        if (((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) && (s.v[1405] != 0.0)) {
            s.store_add_ad_rhs(938, 938, A::mul(s.ad_value(1116), A::sub(s.ad_value(1128), s.ad_value(1127))));
        }

        if ((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) {
            s.store_scale(843, 376, 0.5);
        }

        if ((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) {
            s.store_sub_ad_lhs(846, A::sub(A::sub(s.ad_value(825), s.ad_value(812)), s.ad_value(841)), 875);
        }

        s.v[1407] = if (s.v[376] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) && (s.v[1407] != 0.0)) {
            s.store_scalar(844, 0.0);
        }

        s.v[1408] = if (s.v[846] < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) && (!(s.v[1407] != 0.0))) && (s.v[1408] != 0.0)) {
            s.store_add_ad_rhs(844, 843, A::div(s.ad_value(846), s.ad_value(376)));
        }

        if ((((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) && (!(s.v[1407] != 0.0))) && (!(s.v[1408] != 0.0))) {
            s.store_sqrt_ad(844, A::add(A::square(s.ad_value(843)), s.ad_value(846)));
        }

        if ((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) {
            s.store_mul_ad(937, A::mul(s.ad_value(981), s.ad_value(376)), A::sub(s.ad_value(844), s.ad_value(843)));
        }

        s.v[1409] = if (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0)) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_15(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) && (s.v[1409] != 0.0)) {
            s.store_sub_ad_lhs(846, A::sub(A::sub(s.ad_value(1125), s.ad_value(1128)), s.ad_value(841)), 1118);
        }

        s.v[1410] = if (s.v[846] < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) && (s.v[1409] != 0.0)) && (s.v[1410] != 0.0)) {
            s.store_add_ad_rhs(844, 843, A::div(s.ad_value(846), s.ad_value(376)));
        }

        if ((((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) && (s.v[1409] != 0.0)) && (!(s.v[1410] != 0.0))) {
            s.store_sqrt_ad(844, A::add(A::square(s.ad_value(843)), s.ad_value(846)));
        }

        if (((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) && (s.v[1409] != 0.0)) {
            s.store_add_ad_rhs(937, 937, A::mul(A::mul(s.ad_value(1116), s.ad_value(376)), A::sub(s.ad_value(844), s.ad_value(843))));
        }

        if (s.v[1402] != 0.0) {
            s.store_mul(894, 861, 333);
        }

        if (s.v[1402] != 0.0) {
            s.store_div(891, 875, 894);
        }

        if (s.v[1402] != 0.0) {
            s.store_offset_ad(814, A::sub(s.ad_value(891), s.ad_value(822)), (-0.02));
        }

        if (s.v[1402] != 0.0) {
            s.store_sqrt_ad(843, A::add(A::square(s.ad_value(814)), A::scale(s.ad_value(891), (4.0 * 0.02))));
        }

        if (s.v[1402] != 0.0) {
            s.store_sub_ad_rhs(877, 891, A::scale(A::add(s.ad_value(814), s.ad_value(843)), 0.5));
        }

        s.v[1411] = if (p.p27 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1402] != 0.0) && (s.v[1411] != 0.0)) {
            s.store_div(1129, 1118, 894);
        }

        if ((s.v[1402] != 0.0) && (s.v[1411] != 0.0)) {
            s.store_offset_ad(814, A::sub(s.ad_value(1129), s.ad_value(822)), (-0.02));
        }

        if ((s.v[1402] != 0.0) && (s.v[1411] != 0.0)) {
            s.store_sqrt_ad(843, A::add(A::square(s.ad_value(814)), A::scale(s.ad_value(1129), (4.0 * 0.02))));
        }

        if ((s.v[1402] != 0.0) && (s.v[1411] != 0.0)) {
            s.store_sub_ad_rhs(1130, 1129, A::scale(A::add(s.ad_value(814), s.ad_value(843)), 0.5));
        }

        s.v[1412] = if (s.v[37] == 2.0) { 1.0 } else { 0.0 };

        if ((s.v[1402] != 0.0) && (s.v[1412] != 0.0)) {
            s.store_scalar(1006, 0.0);
        }

        if ((s.v[1402] != 0.0) && (!(s.v[1412] != 0.0))) {
            s.store_mul(843, 894, 877);
        }

        if ((s.v[1402] != 0.0) && (!(s.v[1412] != 0.0))) {
            s.store_scale_ad(844, A::offset(A::sub(s.ad_value(875), A::scale(s.ad_value(843), 0.5)), 1e-20), 12.0);
        }

        if ((s.v[1402] != 0.0) && (!(s.v[1412] != 0.0))) {
            s.store_div(845, 877, 844);
        }

        if ((s.v[1402] != 0.0) && (!(s.v[1412] != 0.0))) {
            s.store_mul(846, 843, 845);
        }

        if ((s.v[1402] != 0.0) && (!(s.v[1412] != 0.0))) {
            s.store_sub_from_scalar(850, 1.0, 894);
        }

        if ((s.v[1402] != 0.0) && (!(s.v[1412] != 0.0))) {
            s.store_mul_ad(1006, A::mul(s.ad_value(981), s.ad_value(850)), A::sub(A::scale(s.ad_value(877), 0.5), s.ad_value(846)));
        }

        s.v[1413] = if (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1402] != 0.0) && (!(s.v[1412] != 0.0))) && (s.v[1413] != 0.0)) {
            s.store_mul(843, 894, 1130);
        }

        if (((s.v[1402] != 0.0) && (!(s.v[1412] != 0.0))) && (s.v[1413] != 0.0)) {
            s.store_scale_ad(844, A::offset(A::sub(s.ad_value(1118), A::scale(s.ad_value(843), 0.5)), 1e-20), 12.0);
        }

        if (((s.v[1402] != 0.0) && (!(s.v[1412] != 0.0))) && (s.v[1413] != 0.0)) {
            s.store_div(845, 1130, 844);
        }

        if (((s.v[1402] != 0.0) && (!(s.v[1412] != 0.0))) && (s.v[1413] != 0.0)) {
            s.store_mul(846, 843, 845);
        }

        if (((s.v[1402] != 0.0) && (!(s.v[1412] != 0.0))) && (s.v[1413] != 0.0)) {
            s.store_sub_from_scalar(850, 1.0, 894);
        }

        if (((s.v[1402] != 0.0) && (!(s.v[1412] != 0.0))) && (s.v[1413] != 0.0)) {
            s.store_add_ad_rhs(1006, 1006, A::mul(A::mul(s.ad_value(1116), s.ad_value(850)), A::sub(A::scale(s.ad_value(1130), 0.5), s.ad_value(846))));
        }

        if (s.v[1402] != 0.0) {
            s.store_mul(843, 894, 877);
        }

        if (s.v[1402] != 0.0) {
            s.store_scale_ad(844, A::offset(A::sub(s.ad_value(875), A::scale(s.ad_value(843), 0.5)), 1e-20), 12.0);
        }

        if (s.v[1402] != 0.0) {
            s.store_div(845, 843, 844);
        }

        if (s.v[1402] != 0.0) {
            s.store_mul(846, 843, 845);
        }

        if (s.v[1402] != 0.0) {
            s.store_mul_ad_rhs(915, 842, A::add(A::sub(s.ad_value(875), A::scale(s.ad_value(843), 0.5)), s.ad_value(846)));
        }

        s.v[1414] = if (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1402] != 0.0) && (s.v[1414] != 0.0)) {
            s.store_mul(1121, 894, 1130);
        }

        if ((s.v[1402] != 0.0) && (s.v[1414] != 0.0)) {
            s.store_scale_ad(855, A::offset(A::sub(s.ad_value(1118), A::scale(s.ad_value(1121), 0.5)), 1e-20), 12.0);
        }

        if ((s.v[1402] != 0.0) && (s.v[1414] != 0.0)) {
            s.store_div(845, 1121, 855);
        }

        if ((s.v[1402] != 0.0) && (s.v[1414] != 0.0)) {
            s.store_mul(846, 1121, 845);
        }

        if ((s.v[1402] != 0.0) && (s.v[1414] != 0.0)) {
            s.store_add_ad_rhs(915, 915, A::mul(s.ad_value(1115), A::add(A::sub(s.ad_value(1118), A::scale(s.ad_value(1121), 0.5)), s.ad_value(846))));
        }

        s.v[1415] = if (p.p129 > 0.5) { 1.0 } else { 0.0 };

        if ((s.v[1402] != 0.0) && (s.v[1415] != 0.0)) {
            s.store_scale(844, 844, 2.0);
        }

        if ((s.v[1402] != 0.0) && (s.v[1415] != 0.0)) {
            s.store_mul_ad(919, A::neg(s.ad_value(842)), A::sub(A::add(A::scale(s.ad_value(875), 0.5), A::scale(s.ad_value(843), 0.25)), A::div(A::square(s.ad_value(843)), s.ad_value(844))));
        }

        s.v[1416] = if (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1402] != 0.0) && (s.v[1415] != 0.0)) && (s.v[1416] != 0.0)) {
            s.store_scale(855, 855, 2.0);
        }

        if (((s.v[1402] != 0.0) && (s.v[1415] != 0.0)) && (s.v[1416] != 0.0)) {
            s.store_sub_ad_rhs(919, 919, A::mul(s.ad_value(1115), A::sub(A::add(A::scale(s.ad_value(1118), 0.5), A::scale(s.ad_value(1121), 0.25)), A::div(A::square(s.ad_value(1121)), s.ad_value(855)))));
        }

        s.v[1417] = if (p.p129 < 0.5) { 1.0 } else { 0.0 };

        if (((s.v[1402] != 0.0) && (!(s.v[1415] != 0.0))) && (s.v[1417] != 0.0)) {
            s.store_scale(844, 844, 0.08333333333333333);
        }

        if (((s.v[1402] != 0.0) && (!(s.v[1415] != 0.0))) && (s.v[1417] != 0.0)) {
            s.store_div_ad(845, A::scale(s.ad_value(842), 0.5), A::square(s.ad_value(844)));
        }

        if (((s.v[1402] != 0.0) && (!(s.v[1415] != 0.0))) && (s.v[1417] != 0.0)) {
            s.store_sub_ad(846, A::mul(s.ad_value(875), A::add(A::scale(A::mul(A::scale(s.ad_value(843), 2.0), s.ad_value(843)), 0.3333333333333333), A::mul(s.ad_value(875), A::sub(s.ad_value(875), A::scale(s.ad_value(843), (4.0 * 0.3333333333333333)))))), A::scale(A::mul(A::mul(A::scale(s.ad_value(843), 2.0), s.ad_value(843)), s.ad_value(843)), 0.06666666666666667));
        }

        if (((s.v[1402] != 0.0) && (!(s.v[1415] != 0.0))) && (s.v[1417] != 0.0)) {
            s.store_mul_ad_lhs(919, A::neg(s.ad_value(845)), 846);
        }

        s.v[1418] = if (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1402] != 0.0) && (!(s.v[1415] != 0.0))) && (s.v[1417] != 0.0)) && (s.v[1418] != 0.0)) {
            s.store_scale(855, 855, 0.08333333333333333);
        }

        if ((((s.v[1402] != 0.0) && (!(s.v[1415] != 0.0))) && (s.v[1417] != 0.0)) && (s.v[1418] != 0.0)) {
            s.store_div_ad(845, A::scale(s.ad_value(1115), 0.5), A::square(s.ad_value(855)));
        }

        if ((((s.v[1402] != 0.0) && (!(s.v[1415] != 0.0))) && (s.v[1417] != 0.0)) && (s.v[1418] != 0.0)) {
            s.store_sub_ad(846, A::mul(s.ad_value(1118), A::add(A::scale(A::mul(A::scale(s.ad_value(1121), 2.0), s.ad_value(1121)), 0.3333333333333333), A::mul(s.ad_value(1118), A::sub(s.ad_value(1118), A::scale(s.ad_value(1121), (4.0 * 0.3333333333333333)))))), A::scale(A::mul(A::mul(A::scale(s.ad_value(1121), 2.0), s.ad_value(1121)), s.ad_value(1121)), 0.06666666666666667));
        }

        if ((((s.v[1402] != 0.0) && (!(s.v[1415] != 0.0))) && (s.v[1417] != 0.0)) && (s.v[1418] != 0.0)) {
            s.store_mul_ad_lhs(1137, A::neg(s.ad_value(845)), 846);
        }

        if ((((s.v[1402] != 0.0) && (!(s.v[1415] != 0.0))) && (s.v[1417] != 0.0)) && (s.v[1418] != 0.0)) {
            s.store_add(919, 919, 1137);
        }

        if (((s.v[1402] != 0.0) && (!(s.v[1415] != 0.0))) && (!(s.v[1417] != 0.0))) {
            s.store_scaled_add(919, 915, 1006, (-0.5));
        }

        s.v[1419] = if (s.v[37] == 2.0) { 1.0 } else { 0.0 };

        if ((s.v[1402] != 0.0) && (s.v[1419] != 0.0)) {
            s.store_scalar(939, 0.0);
        }

        if ((s.v[1402] != 0.0) && (!(s.v[1419] != 0.0))) {
            s.store_scale(914, 263, (p.p361 * (s.v[913] * ((((s.v[332] / p.p23) * p.p3) * s.v[366]) + p.p29))));
        }

        if ((s.v[1402] != 0.0) && (!(s.v[1419] != 0.0))) {
            s.store_mul_ad_rhs(939, 914, A::sub(s.ad_value(902), s.ad_value(824)));
        }

        if (s.v[1402] != 0.0) {
            s.store_add_ad_lhs(916, A::add(s.ad_value(915), s.ad_value(938)), 937);
        }

        if (s.v[1402] != 0.0) {
            s.store_sub_ad_lhs(917, A::sub(A::sub(s.ad_value(1006), s.ad_value(938)), s.ad_value(937)), 939);
        }

        if (s.v[1402] != 0.0) {
            s.copy_ad(920, 939);
        }

        if (s.v[1402] != 0.0) {
            s.store_neg_ad(918, A::add(A::add(A::add(s.ad_value(916), s.ad_value(919)), s.ad_value(917)), s.ad_value(920)));
        }

        s.v[1420] = if (p.p61 == 3.0) { 1.0 } else { 0.0 };

        s.v[1421] = if (p.p41 == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1421] != 0.0)) {
            s.store_div_from_scalar(997, 3.453133e-11, 62);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1421] != 0.0))) {
            s.store_div_ad_lhs(997, A::scale(s.ad_value(416), 8.85418e-12), 62);
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_div_ad_lhs(842, A::mul(s.ad_value(842), s.ad_value(415)), 62);
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_div_ad_lhs(981, A::scale(s.ad_value(981), p.p66), 62);
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_scale(998, 62, 100000000.0);
        }

        s.v[1422] = if (p.p27 > 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1422] != 0.0)) {
            s.store_div_ad_lhs(1115, A::scale(s.ad_value(1115), p.p66), 62);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1422] != 0.0)) {
            s.store_div_ad_lhs(1116, A::scale(s.ad_value(1116), p.p66), 62);
        }

        s.v[1423] = if (s.v[37] == 2.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1423] != 0.0)) {
            s.store_scalar(938, 0.0);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1423] != 0.0)) {
            s.store_scalar(937, 0.0);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1423] != 0.0)) {
            s.store_scalar(1015, 0.0);
        }

        s.v[1424] = if ((p.p36 == 1.0) && (p.p14 != 0.0)) { 1.0 } else { 0.0 };

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1424] != 0.0)) {
            s.store_add_ad_lhs(1015, A::sub(A::sub(s.ad_value(1014), s.ad_value(942)), A::mul(s.ad_value(405), s.ad_value(943))), 324);
        }

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (!(s.v[1424] != 0.0))) {
            s.store_add(1015, 67, 324);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) {
            s.store_offset_ad(813, A::add(A::sub(s.ad_value(1015), s.ad_value(825)), s.ad_value(841)), (-0.02));
        }

        s.v[1425] = if (s.v[1015] <= 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1425] != 0.0)) {
            s.store_sqrt_ad(843, A::sub(A::square(s.ad_value(813)), A::scale(s.ad_value(1015), (4.0 * 0.02))));
        }

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (!(s.v[1425] != 0.0))) {
            s.store_sqrt_ad(843, A::add(A::square(s.ad_value(813)), A::scale(s.ad_value(1015), (4.0 * 0.02))));
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) {
            s.store_sub_ad_rhs(812, 1015, A::scale(A::add(s.ad_value(813), s.ad_value(843)), 0.5));
        }

        s.v[1426] = if (p.p27 > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1426] != 0.0)) {
            s.store_offset(1126, 1015, p.p1033);
        }

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1426] != 0.0)) {
            s.store_offset_ad(813, A::add(A::sub(s.ad_value(1126), s.ad_value(1125)), s.ad_value(841)), (-0.02));
        }

        s.v[1427] = if (s.v[1126] <= 0.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1426] != 0.0)) && (s.v[1427] != 0.0)) {
            s.store_sqrt_ad(843, A::sub(A::square(s.ad_value(813)), A::scale(s.ad_value(1126), (100.0 * 0.02))));
        }

        if (((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1426] != 0.0)) && (!(s.v[1427] != 0.0))) {
            s.store_sqrt_ad(843, A::add(A::square(s.ad_value(813)), A::scale(s.ad_value(1126), (100.0 * 0.02))));
        }

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1426] != 0.0)) {
            s.store_sub_ad_rhs(1128, 1126, A::scale(A::add(s.ad_value(813), s.ad_value(843)), 0.5));
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) {
            s.store_div_ad_lhs(843, A::sub(A::sub(s.ad_value(825), s.ad_value(841)), s.ad_value(1015)), 998);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) {
            s.store_mul(859, 843, 361);
        }

        s.v[1428] = if (((-100.0) < s.v[859]) && (s.v[859] < 100.0)) { 1.0 } else { 0.0 };

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1428] != 0.0)) {
            s.store_mul_ad_rhs(999, 360, A::exp(s.ad_value(859)));
        }

        s.v[1429] = if (s.v[859] <= (-100.0)) { 1.0 } else { 0.0 };

        if (((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (!(s.v[1428] != 0.0))) && (s.v[1429] != 0.0)) {
            s.store_scale(999, 360, 3.720075976e-44);
        }

        if (((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (!(s.v[1428] != 0.0))) && (!(s.v[1429] != 0.0))) {
            s.store_scale(999, 360, 2.688117142e43);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) {
            s.store_scale(1000, 62, 0.001);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) {
            s.store_sub_ad_lhs(813, A::sub(s.ad_value(360), s.ad_value(999)), 1000);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) {
            s.store_sqrt_ad(814, A::add(A::square(s.ad_value(813)), A::mul(A::scale(s.ad_value(1000), 4.0), s.ad_value(360))));
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) {
            s.store_sub_ad_rhs(999, 360, A::scale(A::add(s.ad_value(813), s.ad_value(814)), 0.5));
        }

        s.v[1430] = if (s.v[999] < 1e-15) { 1.0 } else { 0.0 };

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1430] != 0.0)) {
            s.store_scalar(999, 1e-15);
        }

        s.v[1431] = if (p.p27 > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1431] != 0.0)) {
            s.store_div_ad_lhs(843, A::sub(A::sub(s.ad_value(1125), s.ad_value(841)), s.ad_value(1126)), 998);
        }

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1431] != 0.0)) {
            s.store_mul(859, 843, 361);
        }

        s.v[1432] = if (((-100.0) < s.v[859]) && (s.v[859] < 100.0)) { 1.0 } else { 0.0 };

        if (((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1431] != 0.0)) && (s.v[1432] != 0.0)) {
            s.store_mul_ad_rhs(1131, 360, A::exp(s.ad_value(859)));
        }

        s.v[1433] = if (s.v[859] <= (-100.0)) { 1.0 } else { 0.0 };

        if ((((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1431] != 0.0)) && (!(s.v[1432] != 0.0))) && (s.v[1433] != 0.0)) {
            s.store_scale(1131, 360, 3.720075976e-44);
        }

        if ((((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1431] != 0.0)) && (!(s.v[1432] != 0.0))) && (!(s.v[1433] != 0.0))) {
            s.store_scale(1131, 360, 2.688117142e43);
        }

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1431] != 0.0)) {
            s.store_sub_ad_lhs(813, A::sub(s.ad_value(360), s.ad_value(1131)), 1000);
        }

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1431] != 0.0)) {
            s.store_sqrt_ad(814, A::add(A::square(s.ad_value(813)), A::mul(A::scale(s.ad_value(1000), 4.0), s.ad_value(360))));
        }

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1431] != 0.0)) {
            s.store_sub_ad_rhs(1131, 360, A::scale(A::add(s.ad_value(813), s.ad_value(814)), 0.5));
        }

        s.v[1434] = if (s.v[1131] < 1e-15) { 1.0 } else { 0.0 };

        if (((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1431] != 0.0)) && (s.v[1434] != 0.0)) {
            s.store_scalar(1131, 1e-15);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) {
            s.store_div(1001, 417, 999);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) {
            s.store_div_ad_rhs(845, 997, A::add(s.ad_value(997), s.ad_value(1001)));
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) {
            s.store_mul(1002, 845, 1001);
        }

        s.v[1435] = if (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0)) { 1.0 } else { 0.0 };

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1435] != 0.0)) {
            s.store_div(1132, 417, 1131);
        }

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1435] != 0.0)) {
            s.store_div_ad_rhs(845, 997, A::add(s.ad_value(997), s.ad_value(1132)));
        }

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1435] != 0.0)) {
            s.store_mul(1133, 845, 1132);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) {
            s.store_div_ad_lhs(982, A::mul(s.ad_value(981), s.ad_value(1002)), 997);
        }

        s.v[1436] = if (p.p27 > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1436] != 0.0)) {
            s.store_div_ad_lhs(1135, A::mul(s.ad_value(1116), s.ad_value(1133)), 997);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) {
            s.store_mul_ad_rhs(938, 982, A::sub(s.ad_value(812), s.ad_value(1015)));
        }

        s.v[1437] = if (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0)) { 1.0 } else { 0.0 };

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1437] != 0.0)) {
            s.store_mul_ad_rhs(1123, 1135, A::sub(s.ad_value(1128), s.ad_value(1126)));
        }

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1437] != 0.0)) {
            s.store_add(938, 938, 1123);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) {
            s.store_scale(843, 376, 0.5);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) {
            s.store_sub_ad_lhs(846, A::sub(A::sub(s.ad_value(825), s.ad_value(812)), s.ad_value(841)), 875);
        }

        s.v[1438] = if (s.v[376] == 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1438] != 0.0)) {
            s.store_scalar(844, 0.0);
        }

        s.v[1439] = if (s.v[846] < 0.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (!(s.v[1438] != 0.0))) && (s.v[1439] != 0.0)) {
            s.store_add_ad_rhs(844, 843, A::div(s.ad_value(846), s.ad_value(376)));
        }

        if (((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (!(s.v[1438] != 0.0))) && (!(s.v[1439] != 0.0))) {
            s.store_sqrt_ad(844, A::add(A::square(s.ad_value(843)), s.ad_value(846)));
        }

    }

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
        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) {
            s.store_mul_ad(937, A::mul(s.ad_value(982), s.ad_value(376)), A::sub(s.ad_value(844), s.ad_value(843)));
        }

        s.v[1440] = if (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0)) { 1.0 } else { 0.0 };

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1440] != 0.0)) {
            s.store_sub_ad_lhs(846, A::sub(A::sub(s.ad_value(1125), s.ad_value(1128)), s.ad_value(841)), 1118);
        }

        s.v[1441] = if (s.v[376] == 0.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1440] != 0.0)) && (s.v[1441] != 0.0)) {
            s.store_scalar(844, 0.0);
        }

        s.v[1442] = if (s.v[846] < 0.0) { 1.0 } else { 0.0 };

        if ((((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1440] != 0.0)) && (!(s.v[1441] != 0.0))) && (s.v[1442] != 0.0)) {
            s.store_add_ad_rhs(844, 843, A::div(s.ad_value(846), s.ad_value(376)));
        }

        if ((((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1440] != 0.0)) && (!(s.v[1441] != 0.0))) && (!(s.v[1442] != 0.0))) {
            s.store_sqrt_ad(844, A::add(A::square(s.ad_value(843)), s.ad_value(846)));
        }

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1440] != 0.0)) {
            s.store_mul_ad(1124, A::mul(s.ad_value(1135), s.ad_value(376)), A::sub(s.ad_value(844), s.ad_value(843)));
        }

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1423] != 0.0))) && (s.v[1440] != 0.0)) {
            s.store_add(937, 937, 1124);
        }

        s.v[1443] = if (s.v[376] <= 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1443] != 0.0)) {
            s.store_mul_ad_lhs(936, A::scale(s.ad_value(362), 0.25), 832);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1443] != 0.0)) {
            s.store_scale(843, 339, 0.5);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1443] != 0.0))) {
            s.store_mul_ad_lhs(936, A::mul(A::mul(s.ad_value(362), s.ad_value(832)), s.ad_value(376)), 376);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1443] != 0.0))) {
            s.store_mul(843, 376, 339);
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_add_ad_lhs(844, A::scale(s.ad_value(843), 2.0), 875);
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_mul_ad_rhs(1004, 832, {
                if ((1.0 + ((s.v[844] * s.v[875]) / s.v[936])) > 1e-38) {
                    A::ln(A::offset(A::div(A::mul(s.ad_value(844), s.ad_value(875)), s.ad_value(936)), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        s.v[1444] = if (p.p27 > 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1444] != 0.0)) {
            s.store_add_ad_lhs(844, A::scale(s.ad_value(843), 2.0), 1118);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1444] != 0.0)) {
            s.store_mul_ad_rhs(1136, 832, {
                if ((1.0 + ((s.v[844] * s.v[1118]) / s.v[936])) > 1e-38) {
                    A::ln(A::offset(A::div(A::mul(s.ad_value(844), s.ad_value(1118)), s.ad_value(936)), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_scale_ad(846, A::sub(A::sub(s.ad_value(829), s.ad_value(1015)), s.ad_value(942)), 4.0);
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_sqrt_ad(845, A::offset(A::square(s.ad_value(846)), 0.0001));
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_scaled_add(847, 846, 845, 0.5);
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_scale(998, 998, 2.0);
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_div_ad_lhs(843, A::add(s.ad_value(875), s.ad_value(847)), 998);
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_exp_ad(859, A::scale({
                if (s.v[843] > 1e-38) {
                    A::ln(s.ad_value(843))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (p.p59 * 0.7)));
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_offset(844, 859, 1.0);
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_div_from_scalar(999, (p.p58 * 1.9e-9), 844);
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_div(1001, 417, 999);
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_div_ad_rhs(843, 997, A::add(s.ad_value(997), s.ad_value(1001)));
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_mul(1002, 843, 1001);
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_div_ad_lhs(1003, A::mul(s.ad_value(842), s.ad_value(1002)), 997);
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_div_ad_lhs(982, A::mul(s.ad_value(981), s.ad_value(1002)), 997);
        }

        s.v[1445] = if (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1445] != 0.0)) {
            s.store_scale_ad(846, A::sub(A::sub(A::offset(s.ad_value(829), p.p1033), s.ad_value(1126)), s.ad_value(942)), 4.0);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1445] != 0.0)) {
            s.store_sqrt_ad(845, A::offset(A::square(s.ad_value(846)), 0.0001));
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1445] != 0.0)) {
            s.store_scaled_add(847, 846, 845, 0.5);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1445] != 0.0)) {
            s.store_div_ad_lhs(843, A::add(s.ad_value(1118), s.ad_value(847)), 998);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1445] != 0.0)) {
            s.store_exp_ad(859, A::scale({
                if (s.v[843] > 1e-38) {
                    A::ln(s.ad_value(843))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (p.p59 * 0.7)));
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1445] != 0.0)) {
            s.store_offset(844, 859, 1.0);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1445] != 0.0)) {
            s.store_div_from_scalar(1131, (p.p58 * 1.9e-9), 844);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1445] != 0.0)) {
            s.store_div(1132, 417, 1131);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1445] != 0.0)) {
            s.store_div_ad_rhs(843, 997, A::add(s.ad_value(997), s.ad_value(1132)));
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1445] != 0.0)) {
            s.store_mul(1133, 843, 1132);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1445] != 0.0)) {
            s.store_div_ad_lhs(1134, A::mul(s.ad_value(1115), s.ad_value(1133)), 997);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1445] != 0.0)) {
            s.store_div_ad_lhs(1135, A::mul(s.ad_value(1116), s.ad_value(1133)), 997);
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_sub(844, 875, 1004);
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_mul(894, 861, 333);
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_div(891, 844, 894);
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_offset_ad(814, A::sub(s.ad_value(891), s.ad_value(822)), (-0.02));
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_sqrt_ad(843, A::add(A::square(s.ad_value(814)), A::scale(s.ad_value(891), (4.0 * 0.02))));
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_sub_ad_rhs(877, 891, A::scale(A::add(s.ad_value(814), s.ad_value(843)), 0.5));
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_mul(843, 894, 877);
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_scale_ad(845, A::offset(A::sub(s.ad_value(844), A::scale(s.ad_value(843), 0.5)), 1e-20), 12.0);
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_div(846, 843, 845);
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_mul_ad_rhs(915, 1003, A::sub(s.ad_value(844), A::mul(s.ad_value(843), A::sub_from_scalar(0.5, s.ad_value(846)))));
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.copy_ad(916, 915);
        }

        s.v[1446] = if (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1446] != 0.0)) {
            s.store_sub(855, 1118, 1136);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1446] != 0.0)) {
            s.store_div(1129, 855, 894);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1446] != 0.0)) {
            s.store_offset_ad(814, A::sub(s.ad_value(1129), s.ad_value(822)), (-0.02));
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1446] != 0.0)) {
            s.store_sqrt_ad(1121, A::add(A::square(s.ad_value(814)), A::scale(s.ad_value(1129), (4.0 * 0.02))));
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1446] != 0.0)) {
            s.store_sub_ad_rhs(1130, 1129, A::scale(A::add(s.ad_value(814), s.ad_value(1121)), 0.5));
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1446] != 0.0)) {
            s.store_mul(1121, 894, 1130);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1446] != 0.0)) {
            s.store_scale_ad(1122, A::offset(A::sub(s.ad_value(855), A::scale(s.ad_value(1121), 0.5)), 1e-20), 12.0);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1446] != 0.0)) {
            s.store_div(846, 1121, 1122);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1446] != 0.0)) {
            s.store_mul_ad_rhs(850, 1134, A::sub(s.ad_value(855), A::mul(s.ad_value(1121), A::sub_from_scalar(0.5, s.ad_value(846)))));
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1446] != 0.0)) {
            s.store_add(915, 915, 850);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1446] != 0.0)) {
            s.copy_ad(916, 915);
        }

        s.v[1447] = if (s.v[37] == 2.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1447] != 0.0)) {
            s.store_scalar(1006, 0.0);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1447] != 0.0))) {
            s.store_sub_from_scalar(850, 1.0, 894);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1447] != 0.0))) {
            s.store_mul_ad(1006, A::mul(s.ad_value(982), s.ad_value(850)), A::sub(A::scale(s.ad_value(877), 0.5), A::div(A::mul(s.ad_value(843), s.ad_value(877)), s.ad_value(845))));
        }

        s.v[1448] = if (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0)) { 1.0 } else { 0.0 };

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1447] != 0.0))) && (s.v[1448] != 0.0)) {
            s.store_mul_ad(1138, A::mul(s.ad_value(1135), s.ad_value(850)), A::sub(A::scale(s.ad_value(1130), 0.5), A::div(A::mul(s.ad_value(1121), s.ad_value(1130)), s.ad_value(1122))));
        }

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1447] != 0.0))) && (s.v[1448] != 0.0)) {
            s.store_add(1006, 1006, 1138);
        }

        s.v[1449] = if (p.p129 > 0.5) { 1.0 } else { 0.0 };

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1449] != 0.0)) {
            s.store_mul_ad(919, A::neg(s.ad_value(1003)), A::sub(A::add(A::scale(s.ad_value(844), 0.5), A::scale(s.ad_value(843), 0.25)), A::div(A::mul(A::scale(s.ad_value(843), 0.5), s.ad_value(843)), s.ad_value(845))));
        }

        s.v[1450] = if (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0)) { 1.0 } else { 0.0 };

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1449] != 0.0)) && (s.v[1450] != 0.0)) {
            s.store_mul_ad(1137, A::neg(s.ad_value(1134)), A::sub(A::add(A::scale(A::sub(s.ad_value(1118), s.ad_value(1136)), 0.5), A::scale(s.ad_value(1121), 0.25)), A::div(A::mul(A::scale(s.ad_value(1121), 0.5), s.ad_value(1121)), s.ad_value(1122))));
        }

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1449] != 0.0)) && (s.v[1450] != 0.0)) {
            s.store_add(919, 919, 1137);
        }

        s.v[1451] = if (p.p129 < 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1449] != 0.0))) && (s.v[1451] != 0.0)) {
            s.store_scale(845, 845, 0.08333333333333333);
        }

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1449] != 0.0))) && (s.v[1451] != 0.0)) {
            s.store_div_ad(846, A::scale(s.ad_value(1003), 0.5), A::square(s.ad_value(845)));
        }

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1449] != 0.0))) && (s.v[1451] != 0.0)) {
            s.store_sub_ad(847, A::mul(s.ad_value(844), A::add(A::scale(A::mul(A::scale(s.ad_value(843), 2.0), s.ad_value(843)), 0.3333333333333333), A::mul(s.ad_value(844), A::sub(s.ad_value(844), A::scale(s.ad_value(843), (4.0 * 0.3333333333333333)))))), A::scale(A::mul(A::mul(A::scale(s.ad_value(843), 2.0), s.ad_value(843)), s.ad_value(843)), 0.06666666666666667));
        }

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1449] != 0.0))) && (s.v[1451] != 0.0)) {
            s.store_mul_ad_lhs(919, A::neg(s.ad_value(846)), 847);
        }

        s.v[1452] = if (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0)) { 1.0 } else { 0.0 };

        if (((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1449] != 0.0))) && (s.v[1451] != 0.0)) && (s.v[1452] != 0.0)) {
            s.store_scale(1122, 1122, 0.08333333333333333);
        }

        if (((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1449] != 0.0))) && (s.v[1451] != 0.0)) && (s.v[1452] != 0.0)) {
            s.store_div_ad(846, A::scale(s.ad_value(1134), 0.5), A::square(s.ad_value(1122)));
        }

        if (((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1449] != 0.0))) && (s.v[1451] != 0.0)) && (s.v[1452] != 0.0)) {
            s.store_sub_ad(847, A::mul(s.ad_value(855), A::add(A::scale(A::mul(A::scale(s.ad_value(1121), 2.0), s.ad_value(1121)), 0.3333333333333333), A::mul(s.ad_value(855), A::sub(s.ad_value(855), A::scale(s.ad_value(1121), (4.0 * 0.3333333333333333)))))), A::scale(A::mul(A::mul(A::scale(s.ad_value(1121), 2.0), s.ad_value(1121)), s.ad_value(1121)), 0.06666666666666667));
        }

        if (((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1449] != 0.0))) && (s.v[1451] != 0.0)) && (s.v[1452] != 0.0)) {
            s.store_mul_ad_lhs(1137, A::neg(s.ad_value(846)), 847);
        }

        if (((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1449] != 0.0))) && (s.v[1451] != 0.0)) && (s.v[1452] != 0.0)) {
            s.store_add(919, 919, 1137);
        }

        if ((((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1449] != 0.0))) && (!(s.v[1451] != 0.0))) {
            s.store_scale(919, 916, (-0.5));
        }

        s.v[1453] = if (s.v[37] == 2.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (s.v[1453] != 0.0)) {
            s.store_scalar(939, 0.0);
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1453] != 0.0))) {
            s.store_scale(914, 263, (p.p361 * (s.v[913] * ((((s.v[332] / p.p23) * p.p3) * s.v[366]) + p.p29))));
        }

        if (((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) && (!(s.v[1453] != 0.0))) {
            s.store_mul_ad_rhs(939, 914, A::sub(s.ad_value(902), s.ad_value(824)));
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_sub_ad_lhs(916, A::add(A::add(s.ad_value(916), s.ad_value(938)), s.ad_value(937)), 1006);
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_sub_ad_lhs(917, A::sub(A::sub(s.ad_value(1006), s.ad_value(938)), s.ad_value(937)), 939);
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.copy_ad(920, 939);
        }

        if ((!(s.v[1402] != 0.0)) && (s.v[1420] != 0.0)) {
            s.store_neg_ad(918, A::add(A::add(A::add(s.ad_value(916), s.ad_value(917)), s.ad_value(920)), s.ad_value(919)));
        }

        if ((!(s.v[1402] != 0.0)) && (!(s.v[1420] != 0.0))) {
            s.store_scalar(938, 0.0);
        }

        if ((!(s.v[1402] != 0.0)) && (!(s.v[1420] != 0.0))) {
            s.store_scalar(937, 0.0);
        }

        if ((!(s.v[1402] != 0.0)) && (!(s.v[1420] != 0.0))) {
            s.store_scalar(920, 0.0);
        }

        if ((!(s.v[1402] != 0.0)) && (!(s.v[1420] != 0.0))) {
            s.store_scalar(917, 0.0);
        }

        if ((!(s.v[1402] != 0.0)) && (!(s.v[1420] != 0.0))) {
            s.store_scalar(919, 0.0);
        }

        if ((!(s.v[1402] != 0.0)) && (!(s.v[1420] != 0.0))) {
            s.store_scalar(918, 0.0);
        }

        if ((!(s.v[1402] != 0.0)) && (!(s.v[1420] != 0.0))) {
            s.store_scalar(916, 0.0);
        }

        s.v[1454] = if (s.v[37] == 2.0) { 1.0 } else { 0.0 };

        if (s.v[1454] != 0.0) {
            s.store_scalar(909, 0.0);
        }

        if (s.v[1454] != 0.0) {
            s.store_scalar(910, 0.0);
        }

        if (!(s.v[1454] != 0.0)) {
            s.copy_ad(815, 48);
        }

        if (!(s.v[1454] != 0.0)) {
            s.store_scalar(980, (-p.p363));
        }

        if (!(s.v[1454] != 0.0)) {
            s.store_add_ad_rhs(815, 815, A::mul(s.ad_value(980), A::sub(s.ad_value(409), s.ad_value(429))));
        }

        if (!(s.v[1454] != 0.0)) {
            s.store_scalar(816, p.p183);
        }

        if (!(s.v[1454] != 0.0)) {
            s.store_scalar(976, ((((p.p185 * s.v[350]) * p.p155) * p.p3) / 1e-7));
        }

        if (!(s.v[1454] != 0.0)) {
            s.store_scale(979, 976, p.p362);
        }

        if (!(s.v[1454] != 0.0)) {
            s.store_add_ad_rhs(976, 976, A::mul(s.ad_value(979), A::sub(s.ad_value(409), s.ad_value(429))));
        }

        if (!(s.v[1454] != 0.0)) {
            s.store_scalar(977, ((((p.p186 * s.v[349]) * p.p155) * p.p3) / 1e-7));
        }

        if (!(s.v[1454] != 0.0)) {
            s.store_scale(978, 977, p.p364);
        }

        if (!(s.v[1454] != 0.0)) {
            s.store_add_ad_rhs(977, 977, A::mul(s.ad_value(978), A::sub(s.ad_value(409), s.ad_value(429))));
        }

        if (!(s.v[1454] != 0.0)) {
            s.store_scale(994, 815, 0.9);
        }

        if (!(s.v[1454] != 0.0)) {
            s.store_sub_from_scalar_ad(811, 1.0, A::div({
                if (s.v[1087] > s.v[994]) {
                    s.ad_value(994)
                } else {
                    s.ad_value(1087)
                }
            }, s.ad_value(815)));
        }

        s.v[1455] = if (s.v[816] == 0.5) { 1.0 } else { 0.0 };

        if ((!(s.v[1454] != 0.0)) && (s.v[1455] != 0.0)) {
            s.store_div_from_scalar_ad(858, 1.0, A::sqrt(s.ad_value(811)));
        }

        if ((!(s.v[1454] != 0.0)) && (!(s.v[1455] != 0.0))) {
            s.store_exp_ad(858, A::mul(A::neg(s.ad_value(816)), {
                if (s.v[811] > 1e-38) {
                    A::ln(s.ad_value(811))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }));
        }

        if (!(s.v[1454] != 0.0)) {
            s.store_div_ad(846, A::mul(A::sub_from_scalar(1.0, A::mul(s.ad_value(811), s.ad_value(858))), s.ad_value(815)), A::sub_from_scalar(1.0, s.ad_value(816)));
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
        s.v[1456] = if (s.v[1087] > s.v[994]) { 1.0 } else { 0.0 };

        if ((!(s.v[1454] != 0.0)) && (s.v[1456] != 0.0)) {
            s.store_add_ad_rhs(846, 846, A::mul(s.ad_value(858), A::sub(s.ad_value(1087), s.ad_value(994))));
        }

        if (!(s.v[1454] != 0.0)) {
            s.store_add_ad(910, A::mul(s.ad_value(976), s.ad_value(846)), A::scale(s.ad_value(987), (p.p351 * p.p3)));
        }

        if (!(s.v[1454] != 0.0)) {
            s.copy_ad(815, 41);
        }

        if (!(s.v[1454] != 0.0)) {
            s.store_scalar(980, (-p.p365));
        }

        if (!(s.v[1454] != 0.0)) {
            s.store_add_ad_rhs(815, 815, A::mul(s.ad_value(980), A::sub(s.ad_value(409), s.ad_value(429))));
        }

        if (!(s.v[1454] != 0.0)) {
            s.store_scalar(816, p.p184);
        }

        if (!(s.v[1454] != 0.0)) {
            s.store_scale(994, 815, 0.9);
        }

        if (!(s.v[1454] != 0.0)) {
            s.store_sub_from_scalar_ad(811, 1.0, A::div({
                if (s.v[1088] > s.v[994]) {
                    s.ad_value(994)
                } else {
                    s.ad_value(1088)
                }
            }, s.ad_value(815)));
        }

        s.v[1457] = if (s.v[816] == 0.5) { 1.0 } else { 0.0 };

        if ((!(s.v[1454] != 0.0)) && (s.v[1457] != 0.0)) {
            s.store_div_from_scalar_ad(858, 1.0, A::sqrt(s.ad_value(811)));
        }

        if ((!(s.v[1454] != 0.0)) && (!(s.v[1457] != 0.0))) {
            s.store_exp_ad(858, A::mul(A::neg(s.ad_value(816)), {
                if (s.v[811] > 1e-38) {
                    A::ln(s.ad_value(811))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }));
        }

        if (!(s.v[1454] != 0.0)) {
            s.store_div_ad(846, A::mul(A::sub_from_scalar(1.0, A::mul(s.ad_value(811), s.ad_value(858))), s.ad_value(815)), A::sub_from_scalar(1.0, s.ad_value(816)));
        }

        s.v[1458] = if (s.v[1088] > s.v[994]) { 1.0 } else { 0.0 };

        if ((!(s.v[1454] != 0.0)) && (s.v[1458] != 0.0)) {
            s.store_add_ad_rhs(846, 846, A::mul(s.ad_value(858), A::sub(s.ad_value(1088), s.ad_value(994))));
        }

        if (!(s.v[1454] != 0.0)) {
            s.store_add_ad(909, A::mul(s.ad_value(977), s.ad_value(846)), A::scale(s.ad_value(988), (p.p351 * p.p3)));
        }

        s.store_scale(853, 897, (-p.p37));

        s.store_scaled_sub(854, 819, 897, p.p37);

        s.v[1459] = if (s.v[43] != 0.0) { 1.0 } else { 0.0 };

        s.v[1460] = if (((s.v[109] > 0.0) && (p.p37 > 0.0)) || ((s.v[109] < 0.0) && (p.p37 < 0.0))) { 1.0 } else { 0.0 };

        s.v[1461] = if (s.v[853] < s.v[322]) { 1.0 } else { 0.0 };

        if (((s.v[1459] != 0.0) && (s.v[1460] != 0.0)) && (s.v[1461] != 0.0)) {
            s.store_scaled_sub(86, 853, 322, s.v[52]);
        }

        s.v[1462] = if (s.v[853] < s.v[175]) { 1.0 } else { 0.0 };

        if ((((s.v[1459] != 0.0) && (s.v[1460] != 0.0)) && (!(s.v[1461] != 0.0))) && (s.v[1462] != 0.0)) {
            s.store_sub(843, 853, 322);
        }

        if ((((s.v[1459] != 0.0) && (s.v[1460] != 0.0)) && (!(s.v[1461] != 0.0))) && (s.v[1462] != 0.0)) {
            s.store_square(844, 843);
        }

        if ((((s.v[1459] != 0.0) && (s.v[1460] != 0.0)) && (!(s.v[1461] != 0.0))) && (s.v[1462] != 0.0)) {
            s.store_mul_ad_rhs(86, 843, A::sub_from_scalar(s.v[52], A::mul(A::scale(s.ad_value(176), 0.3333333333333333), s.ad_value(844))));
        }

        s.v[1463] = if (s.v[853] < s.v[323]) { 1.0 } else { 0.0 };

        if (((((s.v[1459] != 0.0) && (s.v[1460] != 0.0)) && (!(s.v[1461] != 0.0))) && (!(s.v[1462] != 0.0))) && (s.v[1463] != 0.0)) {
            s.store_sub(843, 853, 323);
        }

        if (((((s.v[1459] != 0.0) && (s.v[1460] != 0.0)) && (!(s.v[1461] != 0.0))) && (!(s.v[1462] != 0.0))) && (s.v[1463] != 0.0)) {
            s.store_square(844, 843);
        }

        if (((((s.v[1459] != 0.0) && (s.v[1460] != 0.0)) && (!(s.v[1461] != 0.0))) && (!(s.v[1462] != 0.0))) && (s.v[1463] != 0.0)) {
            s.store_add_ad(86, A::add(A::mul(s.ad_value(53), s.ad_value(853)), s.ad_value(56)), A::mul(A::mul(A::scale(s.ad_value(177), 0.3333333333333333), s.ad_value(843)), s.ad_value(844)));
        }

        if (((((s.v[1459] != 0.0) && (s.v[1460] != 0.0)) && (!(s.v[1461] != 0.0))) && (!(s.v[1462] != 0.0))) && (!(s.v[1463] != 0.0))) {
            s.store_add_ad_lhs(86, A::mul(s.ad_value(53), s.ad_value(853)), 56);
        }

        s.v[1464] = if (s.v[853] < s.v[323]) { 1.0 } else { 0.0 };

        if (((s.v[1459] != 0.0) && (!(s.v[1460] != 0.0))) && (s.v[1464] != 0.0)) {
            s.store_mul_ad_rhs(86, 53, A::sub(s.ad_value(853), s.ad_value(323)));
        }

        s.v[1465] = if (s.v[853] < s.v[175]) { 1.0 } else { 0.0 };

        if ((((s.v[1459] != 0.0) && (!(s.v[1460] != 0.0))) && (!(s.v[1464] != 0.0))) && (s.v[1465] != 0.0)) {
            s.store_sub(843, 853, 323);
        }

        if ((((s.v[1459] != 0.0) && (!(s.v[1460] != 0.0))) && (!(s.v[1464] != 0.0))) && (s.v[1465] != 0.0)) {
            s.store_square(844, 843);
        }

        if ((((s.v[1459] != 0.0) && (!(s.v[1460] != 0.0))) && (!(s.v[1464] != 0.0))) && (s.v[1465] != 0.0)) {
            s.store_mul_ad_rhs(86, 843, A::sub(s.ad_value(53), A::mul(A::scale(s.ad_value(176), 0.3333333333333333), s.ad_value(844))));
        }

        s.v[1466] = if (s.v[853] < s.v[322]) { 1.0 } else { 0.0 };

        if (((((s.v[1459] != 0.0) && (!(s.v[1460] != 0.0))) && (!(s.v[1464] != 0.0))) && (!(s.v[1465] != 0.0))) && (s.v[1466] != 0.0)) {
            s.store_sub(843, 853, 322);
        }

        if (((((s.v[1459] != 0.0) && (!(s.v[1460] != 0.0))) && (!(s.v[1464] != 0.0))) && (!(s.v[1465] != 0.0))) && (s.v[1466] != 0.0)) {
            s.store_square(844, 843);
        }

        if (((((s.v[1459] != 0.0) && (!(s.v[1460] != 0.0))) && (!(s.v[1464] != 0.0))) && (!(s.v[1465] != 0.0))) && (s.v[1466] != 0.0)) {
            s.store_add_ad(86, A::add(A::scale(s.ad_value(853), s.v[52]), s.ad_value(56)), A::mul(A::mul(A::scale(s.ad_value(177), 0.3333333333333333), s.ad_value(843)), s.ad_value(844)));
        }

        if (((((s.v[1459] != 0.0) && (!(s.v[1460] != 0.0))) && (!(s.v[1464] != 0.0))) && (!(s.v[1465] != 0.0))) && (!(s.v[1466] != 0.0))) {
            s.store_add_ad_lhs(86, A::scale(s.ad_value(853), s.v[52]), 56);
        }

        s.v[1467] = if (((s.v[109] > 0.0) && (p.p37 > 0.0)) || ((s.v[109] < 0.0) && (p.p37 < 0.0))) { 1.0 } else { 0.0 };

        s.v[1468] = if (s.v[854] < s.v[322]) { 1.0 } else { 0.0 };

        if (((s.v[1459] != 0.0) && (s.v[1467] != 0.0)) && (s.v[1468] != 0.0)) {
            s.store_scaled_sub(87, 854, 322, s.v[54]);
        }

        s.v[1469] = if (s.v[854] < s.v[175]) { 1.0 } else { 0.0 };

        if ((((s.v[1459] != 0.0) && (s.v[1467] != 0.0)) && (!(s.v[1468] != 0.0))) && (s.v[1469] != 0.0)) {
            s.store_sub(843, 854, 322);
        }

        if ((((s.v[1459] != 0.0) && (s.v[1467] != 0.0)) && (!(s.v[1468] != 0.0))) && (s.v[1469] != 0.0)) {
            s.store_square(844, 843);
        }

        if ((((s.v[1459] != 0.0) && (s.v[1467] != 0.0)) && (!(s.v[1468] != 0.0))) && (s.v[1469] != 0.0)) {
            s.store_mul_ad_rhs(87, 843, A::sub_from_scalar(s.v[54], A::mul(A::scale(s.ad_value(178), 0.3333333333333333), s.ad_value(844))));
        }

        s.v[1470] = if (s.v[854] < s.v[323]) { 1.0 } else { 0.0 };

        if (((((s.v[1459] != 0.0) && (s.v[1467] != 0.0)) && (!(s.v[1468] != 0.0))) && (!(s.v[1469] != 0.0))) && (s.v[1470] != 0.0)) {
            s.store_sub(843, 854, 323);
        }

        if (((((s.v[1459] != 0.0) && (s.v[1467] != 0.0)) && (!(s.v[1468] != 0.0))) && (!(s.v[1469] != 0.0))) && (s.v[1470] != 0.0)) {
            s.store_square(844, 843);
        }

        if (((((s.v[1459] != 0.0) && (s.v[1467] != 0.0)) && (!(s.v[1468] != 0.0))) && (!(s.v[1469] != 0.0))) && (s.v[1470] != 0.0)) {
            s.store_add_ad(87, A::add(A::mul(s.ad_value(55), s.ad_value(854)), s.ad_value(57)), A::mul(A::mul(A::scale(s.ad_value(179), 0.3333333333333333), s.ad_value(843)), s.ad_value(844)));
        }

        if (((((s.v[1459] != 0.0) && (s.v[1467] != 0.0)) && (!(s.v[1468] != 0.0))) && (!(s.v[1469] != 0.0))) && (!(s.v[1470] != 0.0))) {
            s.store_add_ad_lhs(87, A::mul(s.ad_value(55), s.ad_value(854)), 57);
        }

        s.v[1471] = if (s.v[854] < s.v[323]) { 1.0 } else { 0.0 };

        if (((s.v[1459] != 0.0) && (!(s.v[1467] != 0.0))) && (s.v[1471] != 0.0)) {
            s.store_mul_ad_rhs(87, 55, A::sub(s.ad_value(854), s.ad_value(323)));
        }

        s.v[1472] = if (s.v[854] < s.v[175]) { 1.0 } else { 0.0 };

        if ((((s.v[1459] != 0.0) && (!(s.v[1467] != 0.0))) && (!(s.v[1471] != 0.0))) && (s.v[1472] != 0.0)) {
            s.store_sub(843, 854, 323);
        }

        if ((((s.v[1459] != 0.0) && (!(s.v[1467] != 0.0))) && (!(s.v[1471] != 0.0))) && (s.v[1472] != 0.0)) {
            s.store_square(844, 843);
        }

        if ((((s.v[1459] != 0.0) && (!(s.v[1467] != 0.0))) && (!(s.v[1471] != 0.0))) && (s.v[1472] != 0.0)) {
            s.store_mul_ad_rhs(87, 843, A::sub(s.ad_value(55), A::mul(A::scale(s.ad_value(178), 0.3333333333333333), s.ad_value(844))));
        }

        s.v[1473] = if (s.v[854] < s.v[322]) { 1.0 } else { 0.0 };

        if (((((s.v[1459] != 0.0) && (!(s.v[1467] != 0.0))) && (!(s.v[1471] != 0.0))) && (!(s.v[1472] != 0.0))) && (s.v[1473] != 0.0)) {
            s.store_sub(843, 854, 322);
        }

        if (((((s.v[1459] != 0.0) && (!(s.v[1467] != 0.0))) && (!(s.v[1471] != 0.0))) && (!(s.v[1472] != 0.0))) && (s.v[1473] != 0.0)) {
            s.store_square(844, 843);
        }

        if (((((s.v[1459] != 0.0) && (!(s.v[1467] != 0.0))) && (!(s.v[1471] != 0.0))) && (!(s.v[1472] != 0.0))) && (s.v[1473] != 0.0)) {
            s.store_add_ad(87, A::add(A::scale(s.ad_value(854), s.v[54]), s.ad_value(57)), A::mul(A::mul(A::scale(s.ad_value(179), 0.3333333333333333), s.ad_value(843)), s.ad_value(844)));
        }

        if (((((s.v[1459] != 0.0) && (!(s.v[1467] != 0.0))) && (!(s.v[1471] != 0.0))) && (!(s.v[1472] != 0.0))) && (!(s.v[1473] != 0.0))) {
            s.store_add_ad_lhs(87, A::scale(s.ad_value(854), s.v[54]), 57);
        }

        if (!(s.v[1459] != 0.0)) {
            s.store_scale(86, 853, s.v[52]);
        }

        if (!(s.v[1459] != 0.0)) {
            s.store_scale(87, 854, s.v[54]);
        }

        s.store_add_ad_rhs(86, 86, A::mul(s.ad_value(58), s.ad_value(853)));

        s.store_add_ad_rhs(87, 87, A::mul(s.ad_value(59), s.ad_value(854)));

        s.v[1474] = if (p.p39 == 3.0) { 1.0 } else { 0.0 };

        if (s.v[1474] != 0.0) {
            s.store_offset(843, 1019, 0.02);
        }

        if (!(s.v[1474] != 0.0)) {
            s.store_offset(843, 820, 0.02);
        }

        s.store_sqrt_ad(844, A::offset(A::square(s.ad_value(843)), (4.0 * 0.02)));

        s.store_scaled_sub(845, 843, 844, 0.5);

        s.store_scale(846, 237, s.v[349]);

        s.store_sqrt_ad(847, A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(845), 4.0), s.ad_value(238))));

        s.v[1475] = if (p.p39 == 3.0) { 1.0 } else { 0.0 };

        if (s.v[1475] != 0.0) {
            s.store_sub_ad(895, A::mul(A::add(s.ad_value(335), s.ad_value(846)), s.ad_value(1019)), A::mul(s.ad_value(846), A::add(s.ad_value(845), A::mul(A::scale(s.ad_value(238), 0.5), A::offset(s.ad_value(847), (-1.0))))));
        }

        if (!(s.v[1475] != 0.0)) {
            s.store_sub_ad(895, A::mul(A::add(s.ad_value(335), s.ad_value(846)), s.ad_value(820)), A::mul(s.ad_value(846), A::add(s.ad_value(845), A::mul(A::scale(s.ad_value(238), 0.5), A::offset(s.ad_value(847), (-1.0))))));
        }

        s.v[1476] = if (p.p39 == 3.0) { 1.0 } else { 0.0 };

        if (s.v[1476] != 0.0) {
            s.store_offset(843, 1018, 0.02);
        }

        if (!(s.v[1476] != 0.0)) {
            s.store_offset(843, 821, 0.02);
        }

        s.store_sqrt_ad(844, A::offset(A::square(s.ad_value(843)), (4.0 * 0.02)));

        s.store_scaled_sub(845, 843, 844, 0.5);

        s.store_scale(846, 236, s.v[350]);

        s.store_sqrt_ad(847, A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(845), 4.0), s.ad_value(238))));

        s.v[1477] = if (p.p39 == 3.0) { 1.0 } else { 0.0 };

        if (s.v[1477] != 0.0) {
            s.store_sub_ad(896, A::mul(A::add(s.ad_value(334), s.ad_value(846)), s.ad_value(1018)), A::mul(s.ad_value(846), A::add(s.ad_value(845), A::mul(A::scale(s.ad_value(238), 0.5), A::offset(s.ad_value(847), (-1.0))))));
        }

        if (!(s.v[1477] != 0.0)) {
            s.store_sub_ad(896, A::mul(A::add(s.ad_value(334), s.ad_value(846)), s.ad_value(821)), A::mul(s.ad_value(846), A::add(s.ad_value(845), A::mul(A::scale(s.ad_value(238), 0.5), A::offset(s.ad_value(847), (-1.0))))));
        }

        s.v[1478] = if (p.p3 != 1.0) { 1.0 } else { 0.0 };

        if (s.v[1478] != 0.0) {
            s.store_scale(895, 895, p.p3);
        }

        if (s.v[1478] != 0.0) {
            s.store_scale(896, 896, p.p3);
        }

        s.v[1505] = if (p.p223 == 0.0) { 1.0 } else { 0.0 };

        s.v[1506] = if (p.p223 == 1.0) { 1.0 } else { 0.0 };

        s.v[1507] = if (p.p223 == 2.0) { 1.0 } else { 0.0 };

        s.v[1508] = if (p.p223 == 3.0) { 1.0 } else { 0.0 };

        if ((s.v[1506] != 0.0) && (!(s.v[1505] != 0.0))) {
            s.store_add_ad_lhs(843, A::add(s.ad_value(83), s.ad_value(84)), 85);
        }

        if ((s.v[1506] != 0.0) && (!(s.v[1505] != 0.0))) {
            s.store_square(843, 843);
        }

        if ((s.v[1506] != 0.0) && (!(s.v[1505] != 0.0))) {
            s.store_div_ad_lhs(1486, A::scale(s.ad_value(946), 2.0), 75);
        }

        if ((s.v[1506] != 0.0) && (!(s.v[1505] != 0.0))) {
            s.store_div_ad_rhs(848, 72, A::scale(s.ad_value(1486), s.v[327]));
        }

        if ((s.v[1506] != 0.0) && (!(s.v[1505] != 0.0))) {
            s.store_square(848, 848);
        }

        if ((s.v[1506] != 0.0) && (!(s.v[1505] != 0.0))) {
            s.store_scale_ad(1487, A::offset(A::scale(s.ad_value(848), (p.p227 * s.v[327])), 1.0), p.p229);
        }

        if ((s.v[1506] != 0.0) && (!(s.v[1505] != 0.0))) {
            s.store_add_ad_lhs(844, A::mul(s.ad_value(1487), A::add(s.ad_value(83), s.ad_value(85))), 84);
        }

        if ((s.v[1506] != 0.0) && (!(s.v[1505] != 0.0))) {
            s.store_div_ad_lhs(845, A::square(s.ad_value(844)), 78);
        }

        if ((s.v[1508] != 0.0) && (!(((s.v[1505] != 0.0) || (s.v[1506] != 0.0)) || (s.v[1507] != 0.0)))) {
            s.store_sub_from_scalar_ad(1491, 1.0, A::mul(s.ad_value(77), s.ad_value(76)));
        }

        if ((s.v[1508] != 0.0) && (!(((s.v[1505] != 0.0) || (s.v[1506] != 0.0)) || (s.v[1507] != 0.0)))) {
            s.store_sub_from_scalar(843, 1.0, 1491);
        }

        if ((s.v[1508] != 0.0) && (!(((s.v[1505] != 0.0) || (s.v[1506] != 0.0)) || (s.v[1507] != 0.0)))) {
            s.store_offset(844, 1491, 1.0);
        }

        if ((s.v[1508] != 0.0) && (!(((s.v[1505] != 0.0) || (s.v[1506] != 0.0)) || (s.v[1507] != 0.0)))) {
            s.store_add_ad_rhs(845, 844, A::div(A::mul(A::scale(s.ad_value(74), 2.0), s.ad_value(49)), A::offset(s.ad_value(72), 1e-10)));
        }

        if ((s.v[1508] != 0.0) && (!(((s.v[1505] != 0.0) || (s.v[1506] != 0.0)) || (s.v[1507] != 0.0)))) {
            s.store_scale_ad(1495, A::offset(A::div(s.ad_value(77), s.ad_value(838)), 1.0), s.v[892]);
        }

        if ((s.v[1508] != 0.0) && (!(((s.v[1505] != 0.0) || (s.v[1506] != 0.0)) || (s.v[1507] != 0.0)))) {
            s.store_div_from_scalar(849, s.v[892], 1495);
        }

        if ((s.v[1508] != 0.0) && (!(((s.v[1505] != 0.0) || (s.v[1506] != 0.0)) || (s.v[1507] != 0.0)))) {
            s.store_square(846, 845);
        }

        if ((s.v[1508] != 0.0) && (!(((s.v[1505] != 0.0) || (s.v[1506] != 0.0)) || (s.v[1507] != 0.0)))) {
            s.store_square(847, 843);
        }

        if ((s.v[1508] != 0.0) && (!(((s.v[1505] != 0.0) || (s.v[1506] != 0.0)) || (s.v[1507] != 0.0)))) {
            s.store_square(848, 846);
        }

        if ((s.v[1508] != 0.0) && (!(((s.v[1505] != 0.0) || (s.v[1506] != 0.0)) || (s.v[1507] != 0.0)))) {
            s.store_div(850, 843, 845);
        }

        if ((s.v[1508] != 0.0) && (!(((s.v[1505] != 0.0) || (s.v[1506] != 0.0)) || (s.v[1507] != 0.0)))) {
            s.store_div(851, 72, 838);
        }

        if ((s.v[1508] != 0.0) && (!(((s.v[1505] != 0.0) || (s.v[1506] != 0.0)) || (s.v[1507] != 0.0)))) {
            s.store_square(851, 851);
        }

        if ((s.v[1508] != 0.0) && (!(((s.v[1505] != 0.0) || (s.v[1506] != 0.0)) || (s.v[1507] != 0.0)))) {
            s.store_scale_ad(1487, A::offset(A::scale(s.ad_value(851), (p.p227 * s.v[892])), 1.0), p.p229);
        }

        if ((s.v[1508] != 0.0) && (!(((s.v[1505] != 0.0) || (s.v[1506] != 0.0)) || (s.v[1507] != 0.0)))) {
            s.store_scale(1501, 396, (p.p3 * (s.v[332] * s.v[331])));
        }

        s.v[1548] = if (s.v[398] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1548] != 0.0) {
            s.store_scale(92, 918, p.p37);
        }

        if (s.v[1548] != 0.0) {
            s.store_scale(93, 919, p.p37);
        }

        if (!(s.v[1548] != 0.0)) {
            s.store_scale(93, 918, p.p37);
        }

        if (!(s.v[1548] != 0.0)) {
            s.store_scale(92, 919, p.p37);
        }

        s.v[1553] = if (p.p39 == 3.0) { 1.0 } else { 0.0 };

        s.v[1559] = if ((p.p36 == 1.0) && (p.p14 != 0.0)) { 1.0 } else { 0.0 };

        s.v[1560] = if ((p.p35 != 0.0) && (!(1.0 != 0.0))) { 1.0 } else { 0.0 };

        s.v[1561] = 1.0;

        s.v[1562] = 1.0;

        s.v[1563] = if (p.p430 == 2.0) { 1.0 } else { 0.0 };

        s.v[1564] = if (p.p430 == 2.0) { 1.0 } else { 0.0 };

        s.copy_ad(426, 916);

        s.copy_ad(427, 918);

        s.copy_ad(428, 919);

        s.store_add(425, 896, 895);

        s.store_sub(918, 427, 895);

        s.store_sub(919, 428, 896);

        s.store_add(916, 426, 425);

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
        let (eq0_e1156,) = {
    if ((((s.v[431] != 0.0) && (s.v[432] != 0.0)) && (s.v[433] != 0.0)) && (s.v[434] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq0_value: f64 = eq0_e1156;
        stamper.stamp_potential(
            branches[0],
            eq0_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_1_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq1_e1169,) = {
    if (((((s.v[431] != 0.0) && (s.v[432] != 0.0)) && (s.v[433] != 0.0)) && (!(s.v[434] != 0.0))) && (s.v[435] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq1_value: f64 = eq1_e1169;
        stamper.stamp_potential(
            branches[1],
            eq1_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_2_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq2_e1180,) = {
    if ((((s.v[431] != 0.0) && (s.v[432] != 0.0)) && (!(s.v[433] != 0.0))) && (s.v[436] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq2_value: f64 = eq2_e1180;
        stamper.stamp_potential(
            branches[2],
            eq2_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_3_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq3_e1191,) = {
    if ((((s.v[431] != 0.0) && (!(s.v[432] != 0.0))) && (s.v[437] != 0.0)) && (s.v[438] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq3_value: f64 = eq3_e1191;
        stamper.stamp_potential(
            branches[3],
            eq3_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_4_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq4_e1205,) = {
    if (((((s.v[431] != 0.0) && (!(s.v[432] != 0.0))) && (s.v[437] != 0.0)) && (!(s.v[438] != 0.0))) && (s.v[439] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq4_value: f64 = eq4_e1205;
        stamper.stamp_potential(
            branches[4],
            eq4_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_5_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq5_e1222,) = {
    if ((((((s.v[431] != 0.0) && (!(s.v[432] != 0.0))) && (s.v[437] != 0.0)) && (!(s.v[438] != 0.0))) && (!(s.v[439] != 0.0))) && (s.v[440] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq5_value: f64 = eq5_e1222;
        stamper.stamp_potential(
            branches[5],
            eq5_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_6_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq6_e1234,) = {
    if ((((s.v[431] != 0.0) && (!(s.v[432] != 0.0))) && (!(s.v[437] != 0.0))) && (s.v[441] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq6_value: f64 = eq6_e1234;
        stamper.stamp_potential(
            branches[6],
            eq6_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_7_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq7_e1245,) = {
    if (s.v[1505] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq7_value: f64 = eq7_e1245;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[8]),
            self.multiplicity * (eq7_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_8_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq8_e1259,) = {
    if ((s.v[1506] != 0.0) && (!(s.v[1505] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq8_value: f64 = eq8_e1259;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[8]),
            self.multiplicity * (eq8_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_9_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq9_e1275,) = {
    if ((s.v[1507] != 0.0) && (!((s.v[1505] != 0.0) || (s.v[1506] != 0.0)))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq9_value: f64 = eq9_e1275;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[8]),
            self.multiplicity * (eq9_value),
            &[
            ],
        );
    }
}
