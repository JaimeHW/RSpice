#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_48(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1319] = (s.v[843] > 100.0);s.store_scalar(1319, if s.b[1319] { 1.0 } else { 0.0 });
        if (((s.b[1275] && (!s.b[1315])) && (!s.b[1316])) && s.b[1319]) {s.store_scaled_offset(844, 843, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1320] = (s.v[843] < (-100.0));s.store_scalar(1320, if s.b[1320] { 1.0 } else { 0.0 });
        if ((((s.b[1275] && (!s.b[1315])) && (!s.b[1316])) && (!s.b[1319])) && s.b[1320]) {s.store_scalar(844, 3.720075976e-44);}
        if ((((s.b[1275] && (!s.b[1315])) && (!s.b[1316])) && (!s.b[1319])) && (!s.b[1320])) {s.store_exp(844, 843);}
        if ((s.b[1275] && (!s.b[1315])) && (!s.b[1316])) {s.store_mul(846, 974, 953);s.store_mul_scale_offset_indices(929, 846, 844, -1.0, 1.0);}
        if (s.b[1275] && (!s.b[1315])) {s.store_scale(932, 299, p[1043]);}
        s.b[1321] = ((s.v[317] - s.v[1088]) < 0.001);s.store_scalar(1321, if s.b[1321] { 1.0 } else { 0.0 });
        if ((s.b[1275] && (!s.b[1315])) && s.b[1321]) {s.store_scalar(844, 1000.0);s.store_mul_div_scaled_inputs_product_lhs(843, 1088, -1.0, 932, 1.0, 317, 844);}
        s.b[1322] = (s.v[843] > 100.0);s.store_scalar(1322, if s.b[1322] { 1.0 } else { 0.0 });
        if (((s.b[1275] && (!s.b[1315])) && s.b[1321]) && s.b[1322]) {s.store_scaled_offset(844, 843, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1323] = (s.v[843] < (-100.0));s.store_scalar(1323, if s.b[1323] { 1.0 } else { 0.0 });
        if ((((s.b[1275] && (!s.b[1315])) && s.b[1321]) && (!s.b[1322])) && s.b[1323]) {s.store_scalar(844, 3.720075976e-44);}
        if ((((s.b[1275] && (!s.b[1315])) && s.b[1321]) && (!s.b[1322])) && (!s.b[1323])) {s.store_exp(844, 843);}
        if ((s.b[1275] && (!s.b[1315])) && s.b[1321]) {s.store_mul(846, 975, 954);s.store_mul_scale_offset_indices(925, 846, 844, -1.0, 1.0);}
        if ((s.b[1275] && (!s.b[1315])) && (!s.b[1321])) {s.store_div_from_scalar_sub_ad(844, 1.0, s.ad_value(317), s.ad_value(1088));s.store_mul_div_scaled_inputs_product_lhs(843, 1088, -1.0, 932, 1.0, 317, 844);}
        s.b[1324] = (s.v[843] > 100.0);s.store_scalar(1324, if s.b[1324] { 1.0 } else { 0.0 });
        if (((s.b[1275] && (!s.b[1315])) && (!s.b[1321])) && s.b[1324]) {s.store_scaled_offset(844, 843, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1325] = (s.v[843] < (-100.0));s.store_scalar(1325, if s.b[1325] { 1.0 } else { 0.0 });
        if ((((s.b[1275] && (!s.b[1315])) && (!s.b[1321])) && (!s.b[1324])) && s.b[1325]) {s.store_scalar(844, 3.720075976e-44);}
        if ((((s.b[1275] && (!s.b[1315])) && (!s.b[1321])) && (!s.b[1324])) && (!s.b[1325])) {s.store_exp(844, 843);}
        if ((s.b[1275] && (!s.b[1315])) && (!s.b[1321])) {s.store_mul(846, 975, 954);s.store_mul_scale_offset_indices(925, 846, 844, -1.0, 1.0);}
        if s.b[1275] {s.store_add_scaled_inputs4_indices(934, 926, 1.0, 927, 1.0, 928, 1.0, 929, 1.0);s.store_add_scaled_inputs4_indices(935, 922, 1.0, 923, 1.0, 924, 1.0, 925, 1.0);}
        if (!s.b[1275]) {s.store_scalar(905, 0.0);s.store_scalar(906, 0.0);s.store_scalar(934, 0.0);s.store_scalar(935, 0.0);s.store_scalar(987, 0.0);s.store_scalar(988, 0.0);s.store_scalar(933, 0.0);}
        s.store_exp_ad(1025, A::mul(s.ad_value(214), {
            if (s.v[411] > 1e-38) {
                A::ln(s.ad_value(411))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }));s.store_add_scaled_product_indices(203, 203, 1.0, 204, 430, 1.0);s.store_add_scaled_product_indices(207, 207, 1.0, 208, 430, 1.0);s.store_add_scaled_product_indices(243, 243, 1.0, 244, 430, 1.0);s.store_add_scaled_product_indices(246, 246, 1.0, 247, 430, 1.0);s.store_add_scaled_product_indices(250, 250, 1.0, 248, 430, 1.0);s.b[1326] = ((p[374] != 0.0) || (p[375] != 0.0));s.store_scalar(1326, if s.b[1326] { 1.0 } else { 0.0 });
        if s.b[1326] {s.store_sub(1075, 825, 824);s.store_add_scaled_inputs_product_indices(826, 408, p[37], 942, (-1.0), 405, 943, (-1.0));s.store_add_scaled_inputs3_offset_indices(846, 826, 1.0, 825, (-1.0), 824, 1.0, (-0.02));}
        s.b[1327] = (s.v[826] <= 0.0);s.store_scalar(1327, if s.b[1327] { 1.0 } else { 0.0 });
        if (s.b[1326] && s.b[1327]) {s.store_sqrt_add_scaled_square_input(843, 846, 1.0, 826, (-(4.0 * 0.02)));}
        if (s.b[1326] && (!s.b[1327])) {s.store_sqrt_add_scaled_square_input(843, 846, 1.0, 826, (4.0 * 0.02));}
        if s.b[1326] {s.store_add_scaled_inputs3_indices(812, 826, 1.0, 846, (-0.5), 843, (-0.5));s.store_sub(1081, 826, 812);}
        s.b[1328] = (s.v[1081] < 0.0);s.store_scalar(1328, if s.b[1328] { 1.0 } else { 0.0 });
        if (s.b[1326] && s.b[1328]) {s.store_scalar(1081, 0.0);}
        s.b[1329] = (s.v[376] == 0.0);s.store_scalar(1329, if s.b[1329] { 1.0 } else { 0.0 });
        if (s.b[1326] && s.b[1329]) {s.store_scalar(1082, 0.0);}
        if (s.b[1326] && (!s.b[1329])) {s.store_add_scaled_inputs4_indices(843, 825, 1.0, 875, (-1.0), 812, -1.0, 841, -1.0);}
        s.b[1330] = (s.v[843] < 0.0);s.store_scalar(1330, if s.b[1330] { 1.0 } else { 0.0 });
        if ((s.b[1326] && (!s.b[1329])) && s.b[1330]) {s.store_div(844, 843, 376);}
        if ((s.b[1326] && (!s.b[1329])) && (!s.b[1330])) {s.store_mul_scaled_offset_ad_rhs(844, 376, 1.0 / (2.0), A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(843), 4.0, s.ad_value(376), s.ad_value(376), 1.0), 1.0)), (-1.0));}
        if (s.b[1326] && (!s.b[1329])) {s.store_add_scaled_inputs4_mixed_iaii(1082, 825, 1.0, A::square(s.ad_value(844)), -1.0, 824, -1.0, 826, -1.0);}
        if (!s.b[1326]) {s.store_scalar(826, 0.0);s.store_scalar(1075, 0.0);s.store_scalar(1081, 0.0);s.store_scalar(1082, 0.0);}
        if (p[375] != 0.0) {s.store_mul(843, 832, 211);s.store_div_scaled_inputs2_indices(1028, 825, 1.0, 408, (-p[37]), 843, 1.0);}
        s.b[1331] = (s.v[1028] > 100.0);s.store_scalar(1331, if s.b[1331] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_49(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((p[375] != 0.0) && s.b[1331]) {s.store_sub_scaled_inputs(1078, 825, 1.0, 408, p[37]);}
        s.b[1332] = (s.v[1028] < (-100.0));s.store_scalar(1332, if s.b[1332] { 1.0 } else { 0.0 });
        if (((p[375] != 0.0) && (!s.b[1331])) && s.b[1332]) {s.store_scale(1078, 843, (((1.0 + 3.720075976e-44)) as f64).ln());}
        if (((p[375] != 0.0) && (!s.b[1331])) && (!s.b[1332])) {s.store_exp(1029, 1028);s.store_mul_ln_mixed_ia(1078, 843, A::offset(s.ad_value(1029), 1.0));}
        if (p[375] != 0.0) {s.store_mul(845, 825, 1078);s.store_scalar(854, s.v[369]);s.store_scalar(855, s.v[370]);s.store_add_scaled_product_indices(846, 205, (-1.0), 203, 206, 1.0);s.store_mul(847, 205, 206);s.store_mul_sub_mixed_iaa(848, 855, A::add_scaled_product(s.ad_value(203), 1.0, s.ad_value(846), s.ad_value(1082), 1.0), A::mul3(s.ad_value(847), s.ad_value(1082), s.ad_value(1082)));}
        s.b[1333] = (s.v[848] > 100.0);s.store_scalar(1333, if s.b[1333] { 1.0 } else { 0.0 });
        if ((p[375] != 0.0) && s.b[1333]) {s.store_scalar(849, 2.688117142e43);}
        s.b[1334] = (s.v[848] < (-100.0));s.store_scalar(1334, if s.b[1334] { 1.0 } else { 0.0 });
        if (((p[375] != 0.0) && (!s.b[1333])) && s.b[1334]) {s.store_scalar(849, 3.720075976e-44);}
        if (((p[375] != 0.0) && (!s.b[1333])) && (!s.b[1334])) {s.store_exp(849, 848);}
        if (p[375] != 0.0) {s.store_mul_product3_indices(1020, 1025, 854, 845, 849, 1.0);s.store_mul_scale_offset_indices(850, 822, 212, -1.0, 0.0);s.store_offset_square(851, 850, 0.0002);}
        s.b[1335] = (s.v[850] > 100.0);s.store_scalar(1335, if s.b[1335] { 1.0 } else { 0.0 });
        if ((p[375] != 0.0) && s.b[1335]) {s.store_scalar(852, 2.688117142e43);}
        s.b[1336] = (s.v[850] < (-100.0));s.store_scalar(1336, if s.b[1336] { 1.0 } else { 0.0 });
        if (((p[375] != 0.0) && (!s.b[1335])) && s.b[1336]) {s.store_scalar(852, 3.720075976e-44);}
        if (((p[375] != 0.0) && (!s.b[1335])) && (!s.b[1336])) {s.store_exp(852, 850);}
        if (p[375] != 0.0) {s.store_offset(844, 852, (((-1.0)) + (0.0001)));s.store_div_scaled_inputs2_indices(853, 844, 1.0, 850, (-1.0), 851, 1.0);s.store_mul(1023, 1020, 853);s.store_offset(844, 852, (((-1.0)) + ((-0.0001))));s.store_div_scaled_add_product_indices(853, 844, (-1.0), 850, 852, 1.0, 851, 1.0);s.store_mul(1024, 1020, 853);s.store_sub(843, 821, 375);s.store_sqrt_square_offset(1026, 843, 0.0001);s.store_mul(845, 821, 1026);s.copy_ad(964, 372);s.copy_ad(965, 373);s.copy_ad(855, 374);s.store_add_scaled_product_indices(846, 209, (-1.0), 207, 210, 1.0);s.store_mul(847, 209, 210);s.store_mul_sub_mixed_iaa(848, 855, A::add_scaled_product(s.ad_value(207), 1.0, s.ad_value(846), s.ad_value(1026), 1.0), A::mul3(s.ad_value(847), s.ad_value(1026), s.ad_value(1026)));}
        s.b[1337] = (s.v[848] > 100.0);s.store_scalar(1337, if s.b[1337] { 1.0 } else { 0.0 });
        if ((p[375] != 0.0) && s.b[1337]) {s.store_scalar(849, 2.688117142e43);}
        s.b[1338] = (s.v[848] < (-100.0));s.store_scalar(1338, if s.b[1338] { 1.0 } else { 0.0 });
        if (((p[375] != 0.0) && (!s.b[1337])) && s.b[1338]) {s.store_scalar(849, 3.720075976e-44);}
        if (((p[375] != 0.0) && (!s.b[1337])) && (!s.b[1338])) {s.store_exp(849, 848);}
        if (p[375] != 0.0) {s.store_mul_product3_indices(1021, 1025, 964, 845, 849, 1.0);s.store_sub(843, 820, 375);s.store_sqrt_square_offset(1027, 843, 0.0001);s.store_mul(845, 820, 1027);s.store_mul_sub_mixed_iaa(848, 855, A::add_scaled_product(s.ad_value(207), 1.0, s.ad_value(846), s.ad_value(1027), 1.0), A::mul3(s.ad_value(847), s.ad_value(1027), s.ad_value(1027)));}
        s.b[1339] = (s.v[848] > 100.0);s.store_scalar(1339, if s.b[1339] { 1.0 } else { 0.0 });
        if ((p[375] != 0.0) && s.b[1339]) {s.store_scalar(849, 2.688117142e43);}
        s.b[1340] = (s.v[848] < (-100.0));s.store_scalar(1340, if s.b[1340] { 1.0 } else { 0.0 });
        if (((p[375] != 0.0) && (!s.b[1339])) && s.b[1340]) {s.store_scalar(849, 3.720075976e-44);}
        if (((p[375] != 0.0) && (!s.b[1339])) && (!s.b[1340])) {s.store_exp(849, 848);}
        if (p[375] != 0.0) {s.store_mul_product3_indices(1022, 1025, 965, 845, 849, 1.0);}
        if (p[375] == 0.0) {s.store_scalar(1022, 0.0);s.store_scalar(1021, 0.0);s.store_scalar(1024, 0.0);s.store_scalar(1023, 0.0);}
        s.b[1341] = ((p[374] != 0.0) && (s.v[37] != 2.0));s.store_scalar(1341, if s.b[1341] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_50(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1341] {s.store_scalar(1077, s.v[345]);s.copy_ad(1076, 1082);s.store_scalar(843, p[396]);s.store_offset_sub(844, 843, 1076, (-p[397]));s.store_sqrt_add_scaled_square_input(846, 844, 1.0, 843, (4.0 * p[397]));s.store_add_scaled_inputs3_indices(1080, 843, 1.0, 844, (-0.5), 846, (-0.5));s.copy_ad(1076, 1080);s.store_scaled_offset(843, 1076, (-p[381]), 1.0 / (p[382]));}
        s.b[1342] = (s.v[843] > 100.0);s.store_scalar(1342, if s.b[1342] { 1.0 } else { 0.0 });
        if (s.b[1341] && s.b[1342]) {s.store_scaled_offset(844, 843, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1343] = (s.v[843] < (-100.0));s.store_scalar(1343, if s.b[1343] { 1.0 } else { 0.0 });
        if ((s.b[1341] && (!s.b[1342])) && s.b[1343]) {s.store_scalar(844, 3.720075976e-44);}
        if ((s.b[1341] && (!s.b[1342])) && (!s.b[1343])) {s.store_exp(844, 843);}
        if s.b[1341] {s.store_scaled_ln_ad(1078, A::offset(s.ad_value(844), 1.0), p[382]);}
        s.b[1344] = (p[386] != 0.0);s.store_scalar(1344, if s.b[1344] { 1.0 } else { 0.0 });
        if (s.b[1341] && s.b[1344]) {s.store_sub_from_scalar_scaled_input(843, 1.0, 1076, 1.0 / (p[386]));}
        if (s.b[1341] && (!s.b[1344])) {s.store_scalar(843, 1.0);}
        s.b[1345] = (s.v[843] < 0.01);s.store_scalar(1345, if s.b[1345] { 1.0 } else { 0.0 });
        if (s.b[1341] && s.b[1345]) {s.store_scalar(843, 0.01);}
        if s.b[1341] {s.store_mul_scale_offset_mixed_ia(844, 1077, A::scale_offset(s.ad_value(893), (s.v[892] * 1.0 / (p[23])), (p[28] / p[3])), p[1035], 0.0);s.store_scalar(845, (p[1036] * p[376]));s.copy_ad(846, 243);s.copy_ad(847, 245);s.store_div_scaled_product_mixed_iai(849, 845, A::add_scaled_product(s.ad_value(846), 1.0, s.ad_value(847), s.ad_value(1076), (-1.0)), 1.0, 843, 1.0);}
        s.b[1346] = (s.v[849] > 100.0);s.store_scalar(1346, if s.b[1346] { 1.0 } else { 0.0 });
        if (s.b[1341] && s.b[1346]) {s.store_scaled_offset(848, 849, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1347] = (s.v[849] < (-100.0));s.store_scalar(1347, if s.b[1347] { 1.0 } else { 0.0 });
        if ((s.b[1341] && (!s.b[1346])) && s.b[1347]) {s.store_scalar(848, 3.720075976e-44);}
        if ((s.b[1341] && (!s.b[1346])) && (!s.b[1347])) {s.store_exp(848, 849);}
        if s.b[1341] {s.store_mul_ad_product_lhs_mixed_ai(1083, A::mul3(s.ad_value(844), s.ad_value(1075), s.ad_value(1078)), 848, 1025);s.copy_ad(1076, 1081);s.store_scalar(843, p[396]);s.store_offset_sub(844, 843, 1076, (-p[397]));s.store_sqrt_add_scaled_square_input(846, 844, 1.0, 843, (4.0 * p[397]));s.store_add_scaled_inputs3_indices(1080, 843, 1.0, 844, (-0.5), 846, (-0.5));s.copy_ad(1076, 1080);s.store_scaled_sub(843, 826, 1075, 1.0 / (p[387]));}
        s.b[1348] = (s.v[843] > 100.0);s.store_scalar(1348, if s.b[1348] { 1.0 } else { 0.0 });
        if (s.b[1341] && s.b[1348]) {s.store_scaled_offset(844, 843, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1349] = (s.v[843] < (-100.0));s.store_scalar(1349, if s.b[1349] { 1.0 } else { 0.0 });
        if ((s.b[1341] && (!s.b[1348])) && s.b[1349]) {s.store_scalar(844, 3.720075976e-44);}
        if ((s.b[1341] && (!s.b[1348])) && (!s.b[1349])) {s.store_exp(844, 843);}
        if s.b[1341] {s.store_scaled_ln_ad(1078, A::offset(s.ad_value(844), 1.0), p[387]);}
        s.b[1350] = (p[391] != 0.0);s.store_scalar(1350, if s.b[1350] { 1.0 } else { 0.0 });
        if (s.b[1341] && s.b[1350]) {s.store_sub_from_scalar_scaled_input(843, 1.0, 1076, 1.0 / (p[391]));}
        if (s.b[1341] && (!s.b[1350])) {s.store_scalar(843, 1.0);}
        s.b[1351] = (s.v[843] < 0.01);s.store_scalar(1351, if s.b[1351] { 1.0 } else { 0.0 });
        if (s.b[1341] && s.b[1351]) {s.store_scalar(843, 0.01);}
        if s.b[1341] {s.store_mul_scale_offset_mixed_ia(844, 1077, A::scale_offset(s.ad_value(893), (s.v[892] * 1.0 / (p[23])), (p[28] / p[3])), p[1037], 0.0);s.store_scalar(845, (p[1038] * p[376]));s.copy_ad(846, 246);s.copy_ad(847, 249);s.store_div_scaled_product_mixed_iai(849, 845, A::add_scaled_product(s.ad_value(846), 1.0, s.ad_value(847), s.ad_value(1076), (-1.0)), 1.0, 843, 1.0);}
        s.b[1352] = (s.v[849] > 100.0);s.store_scalar(1352, if s.b[1352] { 1.0 } else { 0.0 });
        if (s.b[1341] && s.b[1352]) {s.store_scaled_offset(848, 849, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1353] = (s.v[849] < (-100.0));s.store_scalar(1353, if s.b[1353] { 1.0 } else { 0.0 });
        if ((s.b[1341] && (!s.b[1352])) && s.b[1353]) {s.store_scalar(848, 3.720075976e-44);}
        if ((s.b[1341] && (!s.b[1352])) && (!s.b[1353])) {s.store_exp(848, 849);}
        if s.b[1341] {s.store_mul_ad_product_lhs_mixed_ai(1084, A::mul3(s.ad_value(844), s.ad_value(1075), s.ad_value(1078)), 848, 1025);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_51(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1354] = (s.v[1075] >= 0.0);s.store_scalar(1354, if s.b[1354] { 1.0 } else { 0.0 });
        if (s.b[1341] && s.b[1354]) {s.copy_ad(1079, 1083);}
        if (s.b[1341] && (!s.b[1354])) {s.copy_ad(1079, 1084);}
        if s.b[1341] {s.store_offset(1127, 826, p[1033]);}
        if (!s.b[1341]) {s.store_scalar(1079, 0.0);}
        s.store_scale(79, 1079, p[37]);s.b[1355] = (((((p[374] != 0.0) && (s.v[37] != 2.0)) && (s.v[399] != 0.0)) && (p[27] > 0.0)) && (s.v[1114] < s.v[1127]));s.store_scalar(1355, if s.b[1355] { 1.0 } else { 0.0 });
        if s.b[1355] {s.store_sub(843, 1114, 1127);s.store_sqrt_square_offset(844, 843, 0.0001);s.store_offset_scaled_sub(1113, 844, 843, 0.5, (((-0.01)) * (0.5)));}
        if s.b[1355] {s.store_scalar(854, (if (p[37] == 1.0) { p[1039] } else { p[1040] }));}
        if s.b[1355] {s.store_scalar(855, (if (p[37] == 1.0) { p[1041] } else { p[1042] }));}
        if s.b[1355] {s.store_mul(845, 1114, 1113);s.store_add_scaled_product_indices(846, 251, (-1.0), 250, 252, 1.0);s.store_mul(847, 251, 252);s.store_mul_sub_scaled_inputs_rhs(848, 855, A::add_scaled_product(s.ad_value(250), 1.0, s.ad_value(846), s.ad_value(1113), 1.0), (-p[376]), A::mul3(s.ad_value(847), s.ad_value(1113), s.ad_value(1113)), (-p[376]));}
        s.b[1356] = (s.v[848] > 100.0);s.store_scalar(1356, if s.b[1356] { 1.0 } else { 0.0 });
        if (s.b[1355] && s.b[1356]) {s.store_scalar(849, 2.688117142e43);}
        s.b[1357] = (s.v[848] < (-100.0));s.store_scalar(1357, if s.b[1357] { 1.0 } else { 0.0 });
        if ((s.b[1355] && (!s.b[1356])) && s.b[1357]) {s.store_scalar(849, 3.720075976e-44);}
        if ((s.b[1355] && (!s.b[1356])) && (!s.b[1357])) {s.store_exp(849, 848);}
        if s.b[1355] {s.store_scale(854, 854, (p[27] * s.v[345]));s.store_mul_product3_indices(1112, 1025, 854, 845, 849, 1.0);}
        if (!s.b[1355]) {s.store_scalar(1112, 0.0);}
        s.store_scale(80, 1112, p[37]);s.b[1358] = (s.v[37] != 2.0);s.store_scalar(1358, if s.b[1358] { 1.0 } else { 0.0 });s.b[1359] = (p[44] == 0.0);s.store_scalar(1359, if s.b[1359] { 1.0 } else { 0.0 });s.b[1360] = (s.v[201] <= 0.0);s.store_scalar(1360, if s.b[1360] { 1.0 } else { 0.0 });
        if ((s.b[1358] && s.b[1359]) && s.b[1360]) {s.store_scalar(908, 0.0);}
        if ((s.b[1358] && s.b[1359]) && (!s.b[1360])) {s.store_add_scaled_product_mixed_iia(966, 276, (-1.0 / (s.v[892])), 275, A::scale_offset(s.ad_value(430), p[308], 1.0), 1.0);s.store_scale(843, 277, s.v[892]);s.store_div_scaled_product_offset_denominator_indices(844, 278, 843, 1.0, 843, 1.0, 1.0);s.store_div_from_scalar_offset_product(843, 1.0, 279, 875, 1.0);s.store_add(846, 843, 280);s.store_mul(845, 830, 846);s.store_div_from_scalar_offset_product(846, 1.0, 281, 822, 1.0);s.store_mul3_lhs(967, 844, 845, 846);s.store_add(921, 966, 967);s.store_sub(969, 822, 921);s.store_add_ad(843, A::add_scaled_product(s.ad_value(274), 1.0, s.ad_value(273), s.ad_value(969), 1.0), A::mul3(s.ad_value(202), s.ad_value(969), s.ad_value(969)));}
        s.b[1361] = (s.v[843] < 1e-5);s.store_scalar(1361, if s.b[1361] { 1.0 } else { 0.0 });
        if (((s.b[1358] && s.b[1359]) && (!s.b[1360])) && s.b[1361]) {s.store_scalar(843, 1e-5);}
        s.b[1362] = ((s.v[843] < (s.v[969] / 100.0)) && (s.v[969] > 0.0));s.store_scalar(1362, if s.b[1362] { 1.0 } else { 0.0 });
        if (((s.b[1358] && s.b[1359]) && (!s.b[1360])) && s.b[1362]) {s.store_scale(968, 201, 2.688117142e43);}
        s.b[1363] = ((s.v[843] < ((-s.v[969]) / 100.0)) && (s.v[969] < 0.0));s.store_scalar(1363, if s.b[1363] { 1.0 } else { 0.0 });
        if ((((s.b[1358] && s.b[1359]) && (!s.b[1360])) && (!s.b[1362])) && s.b[1363]) {s.store_scale(968, 201, 3.720075976e-44);}
        if ((((s.b[1358] && s.b[1359]) && (!s.b[1360])) && (!s.b[1362])) && (!s.b[1363])) {s.store_mul_exp_mixed_ia(968, 201, A::div(s.ad_value(969), s.ad_value(843)));}
        s.b[1364] = (s.v[968] > 10.0);s.store_scalar(1364, if s.b[1364] { 1.0 } else { 0.0 });
        if (((s.b[1358] && s.b[1359]) && (!s.b[1360])) && s.b[1364]) {s.store_scalar(968, 10.0);}
        if ((s.b[1358] && s.b[1359]) && (!s.b[1360])) {s.store_add_product3_rhs_indices(843, 885, 267, 398, 933, 1.0);s.store_mul(908, 968, 843);}
        s.b[1365] = (s.v[201] <= 0.0);s.store_scalar(1365, if s.b[1365] { 1.0 } else { 0.0 });
        if ((s.b[1358] && (!s.b[1359])) && s.b[1365]) {s.store_scalar(1106, 0.0);}
        if ((s.b[1358] && (!s.b[1359])) && (!s.b[1365])) {s.store_add_scaled_product_mixed_iia(966, 276, (-1.0 / (s.v[892])), 275, A::scale_offset(s.ad_value(430), p[308], 1.0), 1.0);s.store_scale(843, 277, s.v[892]);s.store_div_scaled_product_offset_denominator_indices(844, 278, 843, 1.0, 843, 1.0, 1.0);s.store_div_from_scalar_offset_product(843, 1.0, 279, 875, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_52(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1358] && (!s.b[1359])) && (!s.b[1365])) {s.store_add(846, 843, 280);s.store_mul(845, 830, 846);s.store_div_from_scalar_offset_product(846, 1.0, 281, 822, 1.0);s.store_mul3_lhs(967, 844, 845, 846);s.store_add(921, 966, 967);s.store_sub(969, 822, 921);s.store_add_ad(843, A::add_scaled_product(s.ad_value(274), 1.0, s.ad_value(273), s.ad_value(969), 1.0), A::mul3(s.ad_value(202), s.ad_value(969), s.ad_value(969)));}
        s.b[1366] = (s.v[843] < 1e-5);s.store_scalar(1366, if s.b[1366] { 1.0 } else { 0.0 });
        if (((s.b[1358] && (!s.b[1359])) && (!s.b[1365])) && s.b[1366]) {s.store_scalar(843, 1e-5);}
        s.b[1367] = ((s.v[843] < (s.v[969] / 100.0)) && (s.v[969] > 0.0));s.store_scalar(1367, if s.b[1367] { 1.0 } else { 0.0 });
        if (((s.b[1358] && (!s.b[1359])) && (!s.b[1365])) && s.b[1367]) {s.store_scale(968, 201, 2.688117142e43);}
        s.b[1368] = ((s.v[843] < ((-s.v[969]) / 100.0)) && (s.v[969] < 0.0));s.store_scalar(1368, if s.b[1368] { 1.0 } else { 0.0 });
        if ((((s.b[1358] && (!s.b[1359])) && (!s.b[1365])) && (!s.b[1367])) && s.b[1368]) {s.store_scale(968, 201, 3.720075976e-44);}
        if ((((s.b[1358] && (!s.b[1359])) && (!s.b[1365])) && (!s.b[1367])) && (!s.b[1368])) {s.store_mul_exp_mixed_ia(968, 201, A::div(s.ad_value(969), s.ad_value(843)));}
        s.b[1369] = (s.v[968] > 10.0);s.store_scalar(1369, if s.b[1369] { 1.0 } else { 0.0 });
        if (((s.b[1358] && (!s.b[1359])) && (!s.b[1365])) && s.b[1369]) {s.store_scalar(968, 10.0);}
        if ((s.b[1358] && (!s.b[1359])) && (!s.b[1365])) {s.copy_ad(843, 885);s.store_mul(1106, 968, 843);}
        if (s.b[1358] && (!s.b[1359])) {s.store_add_scaled_inputs(843, 269, 1.0 / (s.v[892]), 268, (s.v[892] * 1.0 / (s.v[892])));s.store_mul_scale_offset_rhs(1105, 270, 430, p[320], 1.0);}
        s.b[1370] = (s.v[398] > 0.0);s.store_scalar(1370, if s.b[1370] { 1.0 } else { 0.0 });
        if ((s.b[1358] && (!s.b[1359])) && s.b[1370]) {s.store_sub(844, 1105, 1088);}
        if ((s.b[1358] && (!s.b[1359])) && (!s.b[1370])) {s.store_sub(844, 1105, 1087);}
        if (s.b[1358] && (!s.b[1359])) {s.store_offset(845, 272, (-1.0));}
        s.b[1371] = (s.v[844] <= 0.0);s.store_scalar(1371, if s.b[1371] { 1.0 } else { 0.0 });
        if ((s.b[1358] && (!s.b[1359])) && s.b[1371]) {s.store_scalar(846, 0.0);}
        if ((s.b[1358] && (!s.b[1359])) && (!s.b[1371])) {s.store_mul_scaled_pow_ad_rhs(846, 271, -1.0, s.ad_value(844), s.ad_value(845));}
        s.b[1372] = (s.v[846] > 100.0);s.store_scalar(1372, if s.b[1372] { 1.0 } else { 0.0 });
        if ((s.b[1358] && (!s.b[1359])) && s.b[1372]) {s.store_scalar(847, 2.688117142e43);}
        s.b[1373] = (s.v[846] < (-100.0));s.store_scalar(1373, if s.b[1373] { 1.0 } else { 0.0 });
        if (((s.b[1358] && (!s.b[1359])) && (!s.b[1372])) && s.b[1373]) {s.store_scalar(847, 3.720075976e-44);}
        if (((s.b[1358] && (!s.b[1359])) && (!s.b[1372])) && (!s.b[1373])) {s.store_exp(847, 846);}
        if (s.b[1358] && (!s.b[1359])) {s.store_mul_ad_product_lhs_mixed_ai(1107, A::mul3(s.ad_value(843), s.ad_value(398), s.ad_value(933)), 844, 847);s.store_add(908, 1106, 1107);}
        s.b[1374] = ((s.v[399] == 0.0) || (s.v[399] == 2.0));s.store_scalar(1374, if s.b[1374] { 1.0 } else { 0.0 });
        if (s.b[1358] && s.b[1374]) {s.store_scalar(907, 0.0);}
        s.b[1375] = (s.v[156] < 0.001);s.store_scalar(1375, if s.b[1375] { 1.0 } else { 0.0 });s.b[1376] = (s.v[50] <= 0.001);s.store_scalar(1376, if s.b[1376] { 1.0 } else { 0.0 });
        if (((s.b[1358] && (!s.b[1374])) && s.b[1375]) && s.b[1376]) {s.store_scalar(843, (1.0 / 0.001));}
        if (((s.b[1358] && (!s.b[1374])) && s.b[1375]) && (!s.b[1376])) {s.store_scalar(843, (1.0 / s.v[50]));}
        if ((s.b[1358] && (!s.b[1374])) && s.b[1375]) {s.store_mul(907, 899, 843);}
        if ((s.b[1358] && (!s.b[1374])) && (!s.b[1375])) {s.store_div_scaled_value_offset_denominator(907, s.ad_value(899), 1.0, s.ad_value(156), s.v[50], 1.0);}
        if (!s.b[1358]) {s.store_scalar(908, 0.0);s.store_scalar(907, 0.0);}
        s.b[1377] = (p[39] > 1.0);s.store_scalar(1377, if s.b[1377] { 1.0 } else { 0.0 });
        if s.b[1377] {s.store_mul(852, 230, 49);s.store_mul(843, 852, 880);s.store_mul_add_rhs(81, 229, 843, 1086);}
        s.b[1378] = (p[3] != 1.0);s.store_scalar(1378, if s.b[1378] { 1.0 } else { 0.0 });
        if (s.b[1377] && s.b[1378]) {s.store_scale(81, 81, p[3]);}
        s.b[1379] = (p[39] == 2.0);s.store_scalar(1379, if s.b[1379] { 1.0 } else { 0.0 });
        if (s.b[1377] && s.b[1379]) {s.store_add(854, 64, 81);s.store_div_scaled_product_indices(81, 64, 81, 1.0, 854, 1.0);}
        if (!s.b[1377]) {s.store_scalar(81, 0.0);}
        s.b[1380] = (p[429] == 0.0);s.store_scalar(1380, if s.b[1380] { 1.0 } else { 0.0 });s.b[1381] = ((s.v[60] + p[135]) > p[431]);s.store_scalar(1381, if s.b[1381] { 1.0 } else { 0.0 });
        if (s.b[1380] && s.b[1381]) {s.store_add(1100, 60, 1101);}
        s.b[1382] = (s.v[1100] < p[431]);s.store_scalar(1382, if s.b[1382] { 1.0 } else { 0.0 });
        if ((s.b[1380] && s.b[1381]) && s.b[1382]) {s.store_scalar(1100, p[431]);}
        if (s.b[1380] && (!s.b[1381])) {s.store_scalar(1100, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_53(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1383] = ((s.v[61] + p[136]) > p[431]);s.store_scalar(1383, if s.b[1383] { 1.0 } else { 0.0 });
        if (s.b[1380] && s.b[1383]) {s.store_add(1099, 61, 1102);}
        s.b[1384] = (s.v[1099] < p[431]);s.store_scalar(1384, if s.b[1384] { 1.0 } else { 0.0 });
        if ((s.b[1380] && s.b[1383]) && s.b[1384]) {s.store_scalar(1099, p[431]);}
        if (s.b[1380] && (!s.b[1383])) {s.store_scalar(1099, 0.0);}
        s.b[1385] = (p[429] == 1.0);s.store_scalar(1385, if s.b[1385] { 1.0 } else { 0.0 });
        if ((!s.b[1380]) && s.b[1385]) {s.store_scalar(887, 0.0);s.store_sub(843, 821, 375);s.store_sqrt_square_offset(844, 843, 0.0001);s.store_scaled_add(1026, 843, 844, 0.5);s.store_offset_mul(843, 183, 1026, 1.0);s.store_mul_scale_offset_indices(844, 818, 184, -1.0, 0.0);s.store_add_scaled_inputs_product_mixed_aiia(845, A::div_from_scalar(1.0, s.ad_value(843)), 1.0, 844, 1.0, 185, A::sub(s.ad_value(897), s.ad_value(941)), 1.0);s.store_add_mixed_ia(846, 845, A::sqrt_square_offset(s.ad_value(845), 0.01));s.store_scale(847, 1096, 0.5);s.store_add_scaled_inputs3_mixed_aii(1100, A::add_scaled_product(s.ad_value(1098), 1.0, s.ad_value(846), s.ad_value(847), 1.0), 1.0, 60, 1.0, 1101, 1.0);}
        s.b[1386] = (s.v[1100] < p[431]);s.store_scalar(1386, if s.b[1386] { 1.0 } else { 0.0 });
        if (((!s.b[1380]) && s.b[1385]) && s.b[1386]) {s.store_scalar(1100, p[431]);}
        if ((!s.b[1380]) && s.b[1385]) {s.store_sub(843, 820, 375);s.store_sqrt_square_offset(844, 843, 0.0001);s.store_scaled_add(1027, 843, 844, 0.5);s.store_offset_mul(843, 183, 1027, 1.0);s.store_mul_scale_offset_indices(844, 817, 184, -1.0, 0.0);s.store_add_scaled_inputs_product_mixed_aiia(845, A::div_from_scalar(1.0, s.ad_value(843)), 1.0, 844, 1.0, 185, A::sub(s.ad_value(897), s.ad_value(941)), 1.0);s.store_add_mixed_ia(846, 845, A::sqrt_square_offset(s.ad_value(845), 0.01));s.store_scale(847, 1095, 0.5);s.store_add_scaled_inputs3_mixed_aii(1099, A::add_scaled_product(s.ad_value(1097), 1.0, s.ad_value(846), s.ad_value(847), 1.0), 1.0, 61, 1.0, 1102, 1.0);}
        s.b[1387] = (s.v[1099] < p[431]);s.store_scalar(1387, if s.b[1387] { 1.0 } else { 0.0 });
        if (((!s.b[1380]) && s.b[1385]) && s.b[1387]) {s.store_scalar(1099, p[431]);}
        if ((!s.b[1380]) && (!s.b[1385])) {s.store_scalar(1100, 0.0);s.store_scalar(1099, 0.0);}
        s.b[1388] = (p[430] != 0.0);s.store_scalar(1388, if s.b[1388] { 1.0 } else { 0.0 });
        if s.b[1388] {s.store_scale(1100, 1100, 1.0 / (p[30]));s.store_scale(1099, 1099, 1.0 / (p[30]));}
        s.store_mul_scale_offset_mixed_ia(844, 875, A::div_scaled_product(s.ad_value(860), s.ad_value(876), 0.5, s.ad_value(890), 1.0), -1.0, 1.0);s.b[1389] = (p[3] != 1.0);s.store_scalar(1389, if s.b[1389] { 1.0 } else { 0.0 });
        if s.b[1389] {s.store_scale(885, 885, p[3]);s.store_scale(933, 933, p[3]);s.store_scale(78, 78, p[3]);s.store_scale(934, 934, p[3]);s.store_scale(935, 935, p[3]);s.store_scale(1023, 1023, p[3]);s.store_scale(1024, 1024, p[3]);s.store_scale(1021, 1021, p[3]);s.store_scale(1022, 1022, p[3]);s.store_scale(908, 908, p[3]);s.store_scale(79, 79, p[3]);s.store_scale(905, 905, p[3]);s.store_scale(906, 906, p[3]);}
        s.store_scalar(83, (A::ddx_projection(&s.ad_value(885), Some(9), None) * p[37]));s.b[1390] = (s.v[398] > 0.0);s.store_scalar(1390, if s.b[1390] { 1.0 } else { 0.0 });
        if s.b[1390] {s.store_scalar(84, (A::ddx_projection(&s.ad_value(885), Some(7), None) * p[37]));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_54(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[1390]) {s.store_scalar(84, (A::ddx_projection(&s.ad_value(885), Some(8), None) * p[37]));}
        s.store_scalar(85, (A::ddx_projection(&s.ad_value(885), Some(5), None) * p[37]));s.store_scale(842, 396, ((((s.v[332] / p[23]) * p[3]) * s.v[331]) + p[26]));s.store_scale(981, 396, (p[361] * ((((s.v[332] / p[23]) * p[3]) * s.v[365]) + p[26])));s.store_scale(1115, 396, p[27]);s.store_scale(1116, 396, (p[361] * p[27]));s.store_sub(830, 825, 1073);s.store_mul(853, 1059, 832);s.store_div_scaled_product_indices(809, 384, 830, 1.0, 853, 1.0);s.store_mul3_lhs(1016, 1059, 363, 832);s.store_mul3_lhs(1017, 1059, 364, 832);s.b[1391] = (p[42] == 0.0);s.store_scalar(1391, if s.b[1391] { 1.0 } else { 0.0 });s.b[1392] = ((s.v[809] > (-100.0)) && (s.v[809] < 100.0));s.store_scalar(1392, if s.b[1392] { 1.0 } else { 0.0 });
        if (s.b[1391] && s.b[1392]) {let t0: A = A::exp(s.ad_value(809));s.store_square_ad(810, t0);s.store_mul_mixed_ia(810, 810, A::exp_scaled_input(A::div(s.ad_value(324), s.ad_value(1016)), -1.0));}
        if (s.b[1391] && s.b[1392]) {
            s.store_mul_mixed_ia(875, 1016, {
                            if ((1.0 + s.v[810]) > 1e-38) {
                                A::ln(A::offset(s.ad_value(810), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        s.b[1393] = (p[27] > 0.0);s.store_scalar(1393, if s.b[1393] { 1.0 } else { 0.0 });
        if ((s.b[1391] && s.b[1392]) && s.b[1393]) {s.store_mul_exp_mixed_ia(1117, 810, A::div_scalar_by_product((-p[1033]), s.ad_value(1017), A::square(s.ad_value(832)), 1.0));}
        if ((s.b[1391] && s.b[1392]) && s.b[1393]) {
            s.store_mul_mixed_ia(1118, 1017, {
                            if ((1.0 + s.v[1117]) > 1e-38) {
                                A::ln(A::offset(s.ad_value(1117), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        s.b[1394] = (p[42] == 1.0);s.store_scalar(1394, if s.b[1394] { 1.0 } else { 0.0 });s.b[1395] = ((s.v[809] > (-100.0)) && (s.v[809] < 100.0));s.store_scalar(1395, if s.b[1395] { 1.0 } else { 0.0 });
        if (((!s.b[1391]) && s.b[1394]) && s.b[1395]) {s.store_exp_ad(810, A::div(s.ad_value(809), A::mul(s.ad_value(384), s.ad_value(363))));s.store_mul_mixed_ia(810, 810, A::exp_scaled_input(A::div(s.ad_value(324), s.ad_value(1016)), -1.0));}
        if (((!s.b[1391]) && s.b[1394]) && s.b[1395]) {
            s.store_mul_mixed_ia(875, 1016, {
                            if ((1.0 + s.v[810]) > 1e-38) {
                                A::ln(A::offset(s.ad_value(810), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        s.b[1396] = (p[27] > 0.0);s.store_scalar(1396, if s.b[1396] { 1.0 } else { 0.0 });
        if ((((!s.b[1391]) && s.b[1394]) && s.b[1395]) && s.b[1396]) {s.store_mul_exp_mixed_ia(1117, 810, A::div_scalar_by_product((-p[1033]), s.ad_value(1017), A::square(s.ad_value(832)), 1.0));}
        if ((((!s.b[1391]) && s.b[1394]) && s.b[1395]) && s.b[1396]) {
            s.store_mul_mixed_ia(1118, 1017, {
                            if ((1.0 + s.v[1117]) > 1e-38) {
                                A::ln(A::offset(s.ad_value(1117), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if ((!s.b[1391]) && (!s.b[1394])) {s.store_div_scaled_product_mixed_iai(809, 388, A::sub(s.ad_value(830), s.ad_value(324)), 1.0, 1016, 1.0);s.store_div_scaled_inputs2_mixed_iai(833, 390, 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(388), A::sub(s.ad_value(830), s.ad_value(324))), (-1.0), 1016, 1.0);}
        s.b[1397] = (s.v[809] > 100.0);s.store_scalar(1397, if s.b[1397] { 1.0 } else { 0.0 });
        if (((!s.b[1391]) && (!s.b[1394])) && s.b[1397]) {s.store_sub(875, 830, 324);}
        s.b[1398] = (s.v[833] > 100.0);s.store_scalar(1398, if s.b[1398] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_55(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((!s.b[1391]) && (!s.b[1394])) && (!s.b[1397])) && s.b[1398]) {s.store_div_scaled_inputs3_indices(843, 830, 1.0, 324, (-1.0), 390, -1.0, 1016, 1.0);s.store_exp(810, 843);s.store_mul_div_scaled_product_indices(875, 810, 832, 1140, 1.0, 396, 1.0);}
        if ((((!s.b[1391]) && (!s.b[1394])) && (!s.b[1397])) && (!s.b[1398])) {s.store_exp(810, 809);}
        if ((((!s.b[1391]) && (!s.b[1394])) && (!s.b[1397])) && (!s.b[1398])) {
            s.store_mul_mixed_ia(844, 1016, {
                            if ((1.0 + s.v[810]) > 1e-38) {
                                A::ln(A::offset(s.ad_value(810), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if ((((!s.b[1391]) && (!s.b[1394])) && (!s.b[1397])) && (!s.b[1398])) {s.store_mul3_ad(857, A::div_scaled_inputs(s.ad_value(396), -1.0, A::mul(s.ad_value(832), s.ad_value(1140)), 1.0), A::exp(s.ad_value(833)), A::sub_from_scalar(1.0, s.ad_value(388)));s.store_sub_mixed_ia(845, 388, A::div_scaled_product(s.ad_value(1016), s.ad_value(857), 1.0, A::sub_from_scalar(1.0, s.ad_value(388)), 1.0));s.store_div(875, 844, 845);}
        s.b[1399] = (p[27] > 0.0);s.store_scalar(1399, if s.b[1399] { 1.0 } else { 0.0 });
        if (((!s.b[1391]) && (!s.b[1394])) && s.b[1399]) {s.store_div_scaled_product_offset_rhs_mixed_iai(1119, 388, A::sub(s.ad_value(830), s.ad_value(324)), (-p[1033]), 1.0, 1017, 1.0);s.store_div_scaled_inputs2_mixed_iai(1120, 390, 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(388), A::offset(A::sub(s.ad_value(830), s.ad_value(324)), (-p[1033]))), (-1.0), 1017, 1.0);}
        s.b[1400] = (s.v[1119] > 100.0);s.store_scalar(1400, if s.b[1400] { 1.0 } else { 0.0 });
        if ((((!s.b[1391]) && (!s.b[1394])) && s.b[1399]) && s.b[1400]) {s.store_offset_sub(1118, 830, 324, (-p[1033]));}
        s.b[1401] = (s.v[1120] > 100.0);s.store_scalar(1401, if s.b[1401] { 1.0 } else { 0.0 });
        if (((((!s.b[1391]) && (!s.b[1394])) && s.b[1399]) && (!s.b[1400])) && s.b[1401]) {s.store_div_scaled_offset_numerator_mixed_ai(843, A::add_scaled_inputs3(s.ad_value(830), 1.0, s.ad_value(324), (-1.0), s.ad_value(390), -1.0), 1.0, (-p[1033]), 1017, 1.0);s.store_exp(1117, 843);s.store_mul_div_scaled_product_indices(1118, 1117, 832, 1140, 1.0, 396, 1.0);}
        if (((((!s.b[1391]) && (!s.b[1394])) && s.b[1399]) && (!s.b[1400])) && (!s.b[1401])) {s.store_exp(1117, 1119);}
        if (((((!s.b[1391]) && (!s.b[1394])) && s.b[1399]) && (!s.b[1400])) && (!s.b[1401])) {
            s.store_mul_mixed_ia(844, 1017, {
                            if ((1.0 + s.v[1117]) > 1e-38) {
                                A::ln(A::offset(s.ad_value(1117), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if (((((!s.b[1391]) && (!s.b[1394])) && s.b[1399]) && (!s.b[1400])) && (!s.b[1401])) {s.store_mul3_ad(857, A::div_scaled_inputs(s.ad_value(396), -1.0, A::mul(s.ad_value(832), s.ad_value(1140)), 1.0), A::exp(s.ad_value(1120)), A::sub_from_scalar(1.0, s.ad_value(388)));s.store_sub_mixed_ia(845, 388, A::div_scaled_product(s.ad_value(1017), s.ad_value(857), 1.0, A::sub_from_scalar(1.0, s.ad_value(388)), 1.0));s.store_div(1118, 844, 845);}
        s.copy_ad(829, 1073);s.copy_ad(828, 1054);s.copy_ad(841, 1044);s.b[1402] = (p[61] == 2.0);s.store_scalar(1402, if s.b[1402] { 1.0 } else { 0.0 });s.b[1403] = (s.v[37] == 2.0);s.store_scalar(1403, if s.b[1403] { 1.0 } else { 0.0 });
        if (s.b[1402] && s.b[1403]) {s.store_scalar(938, 0.0);s.store_scalar(937, 0.0);}
        if (s.b[1402] && (!s.b[1403])) {s.store_add_mixed_ai(826, A::add_scaled_inputs_product(s.ad_value(829), 1.0, s.ad_value(942), (-1.0), s.ad_value(405), s.ad_value(828), (-1.0)), 324);s.store_add_scaled_inputs3_offset_indices(813, 826, 1.0, 825, (-1.0), 841, 1.0, (-0.08));}
        s.b[1404] = (s.v[826] <= 0.0);s.store_scalar(1404, if s.b[1404] { 1.0 } else { 0.0 });
        if ((s.b[1402] && (!s.b[1403])) && s.b[1404]) {s.store_sqrt_add_scaled_square_input(843, 813, 1.0, 826, (-(4.0 * 0.08)));}
        if ((s.b[1402] && (!s.b[1403])) && (!s.b[1404])) {s.store_sqrt_add_scaled_square_input(843, 813, 1.0, 826, (4.0 * 0.08));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_56(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1402] && (!s.b[1403])) {s.store_add_scaled_inputs3_indices(812, 826, 1.0, 813, (-0.5), 843, (-0.5));s.store_mul_sub_rhs(938, 981, 812, 826);}
        s.b[1405] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p[27] > 0.0));s.store_scalar(1405, if s.b[1405] { 1.0 } else { 0.0 });
        if ((s.b[1402] && (!s.b[1403])) && s.b[1405]) {s.store_offset(1127, 826, p[1033]);s.store_scalar(1139, 0.08);s.store_add_scaled_inputs4_indices(813, 1127, 1.0, 1125, (-1.0), 841, 1.0, 1139, -1.0);}
        s.b[1406] = (s.v[1127] <= 0.0);s.store_scalar(1406, if s.b[1406] { 1.0 } else { 0.0 });
        if (((s.b[1402] && (!s.b[1403])) && s.b[1405]) && s.b[1406]) {s.store_sqrt_add_scaled_square_product(843, 813, 1.0, 1139, 1127, (-100.0));}
        if (((s.b[1402] && (!s.b[1403])) && s.b[1405]) && (!s.b[1406])) {s.store_sqrt_add_scaled_square_product(843, 813, 1.0, 1139, 1127, 100.0);}
        if ((s.b[1402] && (!s.b[1403])) && s.b[1405]) {s.store_add_scaled_inputs3_indices(1128, 1127, 1.0, 813, (-0.5), 843, (-0.5));s.store_add_scaled_product_right_sub(938, 938, 1.0, 1116, 1128, 1127, 1.0);}
        if (s.b[1402] && (!s.b[1403])) {s.store_scale(843, 376, 0.5);s.store_add_scaled_inputs4_indices(846, 825, 1.0, 812, (-1.0), 841, -1.0, 875, -1.0);}
        s.b[1407] = (s.v[376] == 0.0);s.store_scalar(1407, if s.b[1407] { 1.0 } else { 0.0 });
        if ((s.b[1402] && (!s.b[1403])) && s.b[1407]) {s.store_scalar(844, 0.0);}
        s.b[1408] = (s.v[846] < 0.0);s.store_scalar(1408, if s.b[1408] { 1.0 } else { 0.0 });
        if (((s.b[1402] && (!s.b[1403])) && (!s.b[1407])) && s.b[1408]) {s.store_add_div_rhs_indices(844, 843, 846, 376);}
        if (((s.b[1402] && (!s.b[1403])) && (!s.b[1407])) && (!s.b[1408])) {s.store_sqrt_square_add(844, 843, 846);}
        if (s.b[1402] && (!s.b[1403])) {s.store_mul_ad_product_rhs_mixed_ia(937, 981, 376, A::sub(s.ad_value(844), s.ad_value(843)));}
        s.b[1409] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p[27] > 0.0));s.store_scalar(1409, if s.b[1409] { 1.0 } else { 0.0 });
        if ((s.b[1402] && (!s.b[1403])) && s.b[1409]) {s.store_add_scaled_inputs4_indices(846, 1125, 1.0, 1128, (-1.0), 841, -1.0, 1118, -1.0);}
        s.b[1410] = (s.v[846] < 0.0);s.store_scalar(1410, if s.b[1410] { 1.0 } else { 0.0 });
        if (((s.b[1402] && (!s.b[1403])) && s.b[1409]) && s.b[1410]) {s.store_add_div_rhs_indices(844, 843, 846, 376);}
        if (((s.b[1402] && (!s.b[1403])) && s.b[1409]) && (!s.b[1410])) {s.store_sqrt_square_add(844, 843, 846);}
        if ((s.b[1402] && (!s.b[1403])) && s.b[1409]) {s.store_add_product3_rhs_mixed_iia(937, 937, 1116, 376, A::sub(s.ad_value(844), s.ad_value(843)), 1.0);}
        if s.b[1402] {s.store_mul(894, 861, 333);s.store_div(891, 875, 894);s.store_offset_sub(814, 891, 822, (-0.02));s.store_sqrt_add_scaled_square_input(843, 814, 1.0, 891, (4.0 * 0.02));s.store_add_scaled_inputs3_indices(877, 891, 1.0, 814, (-0.5), 843, (-0.5));}
        s.b[1411] = (p[27] > 0.0);s.store_scalar(1411, if s.b[1411] { 1.0 } else { 0.0 });
        if (s.b[1402] && s.b[1411]) {s.store_div(1129, 1118, 894);s.store_offset_sub(814, 1129, 822, (-0.02));s.store_sqrt_add_scaled_square_input(843, 814, 1.0, 1129, (4.0 * 0.02));s.store_add_scaled_inputs3_indices(1130, 1129, 1.0, 814, (-0.5), 843, (-0.5));}
        s.b[1412] = (s.v[37] == 2.0);s.store_scalar(1412, if s.b[1412] { 1.0 } else { 0.0 });
        if (s.b[1402] && s.b[1412]) {s.store_scalar(1006, 0.0);}
        if (s.b[1402] && (!s.b[1412])) {s.store_mul(843, 894, 877);s.store_scaled_offset_ad(844, A::sub_scaled_inputs(s.ad_value(875), 1.0, s.ad_value(843), 0.5), 1e-20, 12.0);s.store_div(845, 877, 844);s.store_mul(846, 843, 845);s.store_sub_from_scalar(850, 1.0, 894);s.store_mul_ad_product_rhs_mixed_ia(1006, 981, 850, A::sub_scaled_inputs(s.ad_value(877), 0.5, s.ad_value(846), 1.0));}
        s.b[1413] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p[27] > 0.0));s.store_scalar(1413, if s.b[1413] { 1.0 } else { 0.0 });
        if ((s.b[1402] && (!s.b[1412])) && s.b[1413]) {s.store_mul(843, 894, 1130);s.store_scaled_offset_ad(844, A::sub_scaled_inputs(s.ad_value(1118), 1.0, s.ad_value(843), 0.5), 1e-20, 12.0);s.store_div(845, 1130, 844);s.store_mul(846, 843, 845);s.store_sub_from_scalar(850, 1.0, 894);s.store_add_product3_rhs_mixed_iia(1006, 1006, 1116, 850, A::sub_scaled_inputs(s.ad_value(1130), 0.5, s.ad_value(846), 1.0), 1.0);}
        if s.b[1402] {s.store_mul(843, 894, 877);s.store_scaled_offset_ad(844, A::sub_scaled_inputs(s.ad_value(875), 1.0, s.ad_value(843), 0.5), 1e-20, 12.0);s.store_div(845, 843, 844);s.store_mul(846, 843, 845);s.store_mul_add_scaled_inputs3_offset_rhs_indices(915, 842, 875, 1.0, 843, (-0.5), 846, 1.0, 0.0);}
        s.b[1414] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p[27] > 0.0));s.store_scalar(1414, if s.b[1414] { 1.0 } else { 0.0 });
        if (s.b[1402] && s.b[1414]) {s.store_mul(1121, 894, 1130);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_57(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1402] && s.b[1414]) {s.store_scaled_offset_ad(855, A::sub_scaled_inputs(s.ad_value(1118), 1.0, s.ad_value(1121), 0.5), 1e-20, 12.0);s.store_div(845, 1121, 855);s.store_mul(846, 1121, 845);s.store_add_scaled_product_mixed_iia(915, 915, 1.0, 1115, A::add_scaled_inputs3(s.ad_value(1118), 1.0, s.ad_value(1121), (-0.5), s.ad_value(846), 1.0), 1.0);}
        s.b[1415] = (p[129] > 0.5);s.store_scalar(1415, if s.b[1415] { 1.0 } else { 0.0 });
        if (s.b[1402] && s.b[1415]) {s.store_scale(844, 844, 2.0);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(919, 842, 875, ((0.5) * (-1.0)), 843, ((0.25) * (-1.0)), A::div_scaled_product(s.ad_value(843), s.ad_value(843), 1.0, s.ad_value(844), 1.0), ((-1.0) * (-1.0)), 0.0);}
        s.b[1416] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p[27] > 0.0));s.store_scalar(1416, if s.b[1416] { 1.0 } else { 0.0 });
        if ((s.b[1402] && s.b[1415]) && s.b[1416]) {s.store_scale(855, 855, 2.0);s.store_add_scaled_product_mixed_iia(919, 919, 1.0, 1115, A::add_scaled_inputs3(s.ad_value(1118), 0.5, s.ad_value(1121), 0.25, A::div_scaled_product(s.ad_value(1121), s.ad_value(1121), 1.0, s.ad_value(855), 1.0), -1.0), (-1.0));}
        s.b[1417] = (p[129] < 0.5);s.store_scalar(1417, if s.b[1417] { 1.0 } else { 0.0 });
        if ((s.b[1402] && (!s.b[1415])) && s.b[1417]) {s.store_scale(844, 844, 0.08333333333333333);s.store_div_scaled_inputs_square_rhs(845, 842, 0.5, 844, 1.0);s.store_add_scaled_product_mixed_aia(846, A::mul3_scaled_output(s.ad_value(843), s.ad_value(843), s.ad_value(843), (2.0 * 0.06666666666666667)), (-1.0), 875, A::add_scaled_products(s.ad_value(843), s.ad_value(843), (2.0 * 0.3333333333333333), s.ad_value(875), A::sub_scaled_inputs(s.ad_value(875), 1.0, s.ad_value(843), (4.0 * 0.3333333333333333)), 1.0), 1.0);s.store_mul_scale_offset_indices(919, 846, 845, -1.0, 0.0);}
        s.b[1418] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p[27] > 0.0));s.store_scalar(1418, if s.b[1418] { 1.0 } else { 0.0 });
        if (((s.b[1402] && (!s.b[1415])) && s.b[1417]) && s.b[1418]) {s.store_scale(855, 855, 0.08333333333333333);s.store_div_scaled_inputs_square_rhs(845, 1115, 0.5, 855, 1.0);s.store_add_scaled_product_mixed_aia(846, A::mul3_scaled_output(s.ad_value(1121), s.ad_value(1121), s.ad_value(1121), (2.0 * 0.06666666666666667)), (-1.0), 1118, A::add_scaled_products(s.ad_value(1121), s.ad_value(1121), (2.0 * 0.3333333333333333), s.ad_value(1118), A::sub_scaled_inputs(s.ad_value(1118), 1.0, s.ad_value(1121), (4.0 * 0.3333333333333333)), 1.0), 1.0);s.store_mul_scale_offset_indices(1137, 846, 845, -1.0, 0.0);s.store_add(919, 919, 1137);}
        if ((s.b[1402] && (!s.b[1415])) && (!s.b[1417])) {s.store_scaled_add(919, 915, 1006, (-0.5));}
        s.b[1419] = (s.v[37] == 2.0);s.store_scalar(1419, if s.b[1419] { 1.0 } else { 0.0 });
        if (s.b[1402] && s.b[1419]) {s.store_scalar(939, 0.0);}
        if (s.b[1402] && (!s.b[1419])) {s.store_scale(914, 263, (p[361] * (s.v[913] * ((((s.v[332] / p[23]) * p[3]) * s.v[366]) + p[29]))));s.store_mul_sub_rhs(939, 914, 902, 824);}
        if s.b[1402] {s.store_add_scaled_inputs3_indices(916, 915, 1.0, 938, 1.0, 937, 1.0);s.store_add_scaled_inputs4_indices(917, 1006, 1.0, 938, (-1.0), 937, -1.0, 939, -1.0);s.copy_ad(920, 939);s.store_add_scaled_inputs4_indices(918, 916, (-1.0), 919, (-1.0), 917, (-1.0), 920, (-1.0));}
        s.b[1420] = (p[61] == 3.0);s.store_scalar(1420, if s.b[1420] { 1.0 } else { 0.0 });s.b[1421] = (p[41] == 0.0);s.store_scalar(1421, if s.b[1421] { 1.0 } else { 0.0 });
        if (((!s.b[1402]) && s.b[1420]) && s.b[1421]) {s.store_div_from_scalar(997, 3.453133e-11, 62);}
        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1421])) {s.store_div_scaled_inputs_indices(997, 416, 8.85418e-12, 62, 1.0);}
        if ((!s.b[1402]) && s.b[1420]) {s.store_div_scaled_product_indices(842, 842, 415, 1.0, 62, 1.0);s.store_div_scaled_inputs_indices(981, 981, p[66], 62, 1.0);s.store_scale(998, 62, 100000000.0);}
        s.b[1422] = (p[27] > 0.0);s.store_scalar(1422, if s.b[1422] { 1.0 } else { 0.0 });
        if (((!s.b[1402]) && s.b[1420]) && s.b[1422]) {s.store_div_scaled_inputs_indices(1115, 1115, p[66], 62, 1.0);s.store_div_scaled_inputs_indices(1116, 1116, p[66], 62, 1.0);}
        s.b[1423] = (s.v[37] == 2.0);s.store_scalar(1423, if s.b[1423] { 1.0 } else { 0.0 });
        if (((!s.b[1402]) && s.b[1420]) && s.b[1423]) {s.store_scalar(938, 0.0);s.store_scalar(937, 0.0);s.store_scalar(1015, 0.0);}
        s.b[1424] = ((p[36] == 1.0) && (p[14] != 0.0));s.store_scalar(1424, if s.b[1424] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_58(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1424]) {s.store_add_mixed_ai(1015, A::add_scaled_inputs_product(s.ad_value(1014), 1.0, s.ad_value(942), (-1.0), s.ad_value(405), s.ad_value(943), (-1.0)), 324);}
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && (!s.b[1424])) {s.store_add(1015, 67, 324);}
        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {s.store_add_scaled_inputs3_offset_indices(813, 1015, 1.0, 825, (-1.0), 841, 1.0, (-0.02));}
        s.b[1425] = (s.v[1015] <= 0.0);s.store_scalar(1425, if s.b[1425] { 1.0 } else { 0.0 });
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1425]) {s.store_sqrt_add_scaled_square_input(843, 813, 1.0, 1015, (-(4.0 * 0.02)));}
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && (!s.b[1425])) {s.store_sqrt_add_scaled_square_input(843, 813, 1.0, 1015, (4.0 * 0.02));}
        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {s.store_add_scaled_inputs3_indices(812, 1015, 1.0, 813, (-0.5), 843, (-0.5));}
        s.b[1426] = (p[27] > 0.0);s.store_scalar(1426, if s.b[1426] { 1.0 } else { 0.0 });
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1426]) {s.store_offset(1126, 1015, p[1033]);s.store_add_scaled_inputs3_offset_indices(813, 1126, 1.0, 1125, (-1.0), 841, 1.0, (-0.02));}
        s.b[1427] = (s.v[1126] <= 0.0);s.store_scalar(1427, if s.b[1427] { 1.0 } else { 0.0 });
        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1426]) && s.b[1427]) {s.store_sqrt_add_scaled_square_input(843, 813, 1.0, 1126, (-(100.0 * 0.02)));}
        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1426]) && (!s.b[1427])) {s.store_sqrt_add_scaled_square_input(843, 813, 1.0, 1126, (100.0 * 0.02));}
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1426]) {s.store_add_scaled_inputs3_indices(1128, 1126, 1.0, 813, (-0.5), 843, (-0.5));}
        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {s.store_div_scaled_inputs3_indices(843, 825, 1.0, 841, (-1.0), 1015, -1.0, 998, 1.0);s.store_mul(859, 843, 361);}
        s.b[1428] = (((-100.0) < s.v[859]) && (s.v[859] < 100.0));s.store_scalar(1428, if s.b[1428] { 1.0 } else { 0.0 });
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1428]) {s.store_mul_exp_rhs(999, 360, 859);}
        s.b[1429] = (s.v[859] <= (-100.0));s.store_scalar(1429, if s.b[1429] { 1.0 } else { 0.0 });
        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && (!s.b[1428])) && s.b[1429]) {s.store_scale(999, 360, 3.720075976e-44);}
        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && (!s.b[1428])) && (!s.b[1429])) {s.store_scale(999, 360, 2.688117142e43);}
        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {s.store_scale(1000, 62, 0.001);s.store_add_scaled_inputs3_indices(813, 360, 1.0, 999, (-1.0), 1000, -1.0);s.store_sqrt_add_scaled_square_product(814, 813, 1.0, 1000, 360, 4.0);s.store_add_scaled_inputs3_indices(999, 360, 1.0, 813, (-0.5), 814, (-0.5));}
        s.b[1430] = (s.v[999] < 1e-15);s.store_scalar(1430, if s.b[1430] { 1.0 } else { 0.0 });
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1430]) {s.store_scalar(999, 1e-15);}
        s.b[1431] = (p[27] > 0.0);s.store_scalar(1431, if s.b[1431] { 1.0 } else { 0.0 });
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) {s.store_div_scaled_inputs3_indices(843, 1125, 1.0, 841, (-1.0), 1126, -1.0, 998, 1.0);s.store_mul(859, 843, 361);}
        s.b[1432] = (((-100.0) < s.v[859]) && (s.v[859] < 100.0));s.store_scalar(1432, if s.b[1432] { 1.0 } else { 0.0 });
        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) && s.b[1432]) {s.store_mul_exp_rhs(1131, 360, 859);}
        s.b[1433] = (s.v[859] <= (-100.0));s.store_scalar(1433, if s.b[1433] { 1.0 } else { 0.0 });
        if ((((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) && (!s.b[1432])) && s.b[1433]) {s.store_scale(1131, 360, 3.720075976e-44);}
        if ((((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) && (!s.b[1432])) && (!s.b[1433])) {s.store_scale(1131, 360, 2.688117142e43);}
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) {s.store_add_scaled_inputs3_indices(813, 360, 1.0, 1131, (-1.0), 1000, -1.0);s.store_sqrt_add_scaled_square_product(814, 813, 1.0, 1000, 360, 4.0);s.store_add_scaled_inputs3_indices(1131, 360, 1.0, 813, (-0.5), 814, (-0.5));}
        s.b[1434] = (s.v[1131] < 1e-15);s.store_scalar(1434, if s.b[1434] { 1.0 } else { 0.0 });
        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) && s.b[1434]) {s.store_scalar(1131, 1e-15);}
        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {s.store_div(1001, 417, 999);s.store_div_add_scaled_inputs_rhs_indices(845, 997, 997, 1.0, 1001, 1.0);s.store_mul(1002, 845, 1001);}
        s.b[1435] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p[27] > 0.0));s.store_scalar(1435, if s.b[1435] { 1.0 } else { 0.0 });
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1435]) {s.store_div(1132, 417, 1131);s.store_div_add_scaled_inputs_rhs_indices(845, 997, 997, 1.0, 1132, 1.0);s.store_mul(1133, 845, 1132);}
        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {s.store_div_scaled_product_indices(982, 981, 1002, 1.0, 997, 1.0);}
        s.b[1436] = (p[27] > 0.0);s.store_scalar(1436, if s.b[1436] { 1.0 } else { 0.0 });
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1436]) {s.store_div_scaled_product_indices(1135, 1116, 1133, 1.0, 997, 1.0);}
        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {s.store_mul_sub_rhs(938, 982, 812, 1015);}
        s.b[1437] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p[27] > 0.0));s.store_scalar(1437, if s.b[1437] { 1.0 } else { 0.0 });
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1437]) {s.store_mul_sub_rhs(1123, 1135, 1128, 1126);s.store_add(938, 938, 1123);}
        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {s.store_scale(843, 376, 0.5);s.store_add_scaled_inputs4_indices(846, 825, 1.0, 812, (-1.0), 841, -1.0, 875, -1.0);}
        s.b[1438] = (s.v[376] == 0.0);s.store_scalar(1438, if s.b[1438] { 1.0 } else { 0.0 });
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1438]) {s.store_scalar(844, 0.0);}
        s.b[1439] = (s.v[846] < 0.0);s.store_scalar(1439, if s.b[1439] { 1.0 } else { 0.0 });
        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && (!s.b[1438])) && s.b[1439]) {s.store_add_div_rhs_indices(844, 843, 846, 376);}
        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && (!s.b[1438])) && (!s.b[1439])) {s.store_sqrt_square_add(844, 843, 846);}
        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {s.store_mul_ad_product_rhs_mixed_ia(937, 982, 376, A::sub(s.ad_value(844), s.ad_value(843)));}
        s.b[1440] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p[27] > 0.0));s.store_scalar(1440, if s.b[1440] { 1.0 } else { 0.0 });
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1440]) {s.store_add_scaled_inputs4_indices(846, 1125, 1.0, 1128, (-1.0), 841, -1.0, 1118, -1.0);}
        s.b[1441] = (s.v[376] == 0.0);s.store_scalar(1441, if s.b[1441] { 1.0 } else { 0.0 });
        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1440]) && s.b[1441]) {s.store_scalar(844, 0.0);}
        s.b[1442] = (s.v[846] < 0.0);s.store_scalar(1442, if s.b[1442] { 1.0 } else { 0.0 });
        if ((((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1440]) && (!s.b[1441])) && s.b[1442]) {s.store_add_div_rhs_indices(844, 843, 846, 376);}
        if ((((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1440]) && (!s.b[1441])) && (!s.b[1442])) {s.store_sqrt_square_add(844, 843, 846);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_59(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1440]) {s.store_mul_ad_product_rhs_mixed_ia(1124, 1135, 376, A::sub(s.ad_value(844), s.ad_value(843)));s.store_add(937, 937, 1124);}
        s.b[1443] = (s.v[376] <= 0.0);s.store_scalar(1443, if s.b[1443] { 1.0 } else { 0.0 });
        if (((!s.b[1402]) && s.b[1420]) && s.b[1443]) {s.store_scaled_mul(936, 362, 832, 0.25);s.store_scale(843, 339, 0.5);}
        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1443])) {s.store_mul_product3_indices(936, 376, 362, 832, 376, 1.0);s.store_mul(843, 376, 339);}
        if ((!s.b[1402]) && s.b[1420]) {s.store_add_scaled_inputs(844, 843, 2.0, 875, 1.0);}
        if ((!s.b[1402]) && s.b[1420]) {
            s.store_mul_mixed_ia(1004, 832, {
                            if ((1.0 + ((s.v[844] * s.v[875]) / s.v[936])) > 1e-38) {
                                A::ln(A::offset(A::div_scaled_product(s.ad_value(844), s.ad_value(875), 1.0, s.ad_value(936), 1.0), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        s.b[1444] = (p[27] > 0.0);s.store_scalar(1444, if s.b[1444] { 1.0 } else { 0.0 });
        if (((!s.b[1402]) && s.b[1420]) && s.b[1444]) {s.store_add_scaled_inputs(844, 843, 2.0, 1118, 1.0);}
        if (((!s.b[1402]) && s.b[1420]) && s.b[1444]) {
            s.store_mul_mixed_ia(1136, 832, {
                            if ((1.0 + ((s.v[844] * s.v[1118]) / s.v[936])) > 1e-38) {
                                A::ln(A::offset(A::div_scaled_product(s.ad_value(844), s.ad_value(1118), 1.0, s.ad_value(936), 1.0), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if ((!s.b[1402]) && s.b[1420]) {s.store_add_scaled_inputs3_indices(846, 829, 4.0, 1015, ((-1.0) * 4.0), 942, (-4.0));s.store_sqrt_square_offset(845, 846, 0.0001);s.store_scaled_add(847, 846, 845, 0.5);s.store_scale(998, 998, 2.0);s.store_div_scaled_inputs2_indices(843, 875, 1.0, 847, 1.0, 998, 1.0);}
        if ((!s.b[1402]) && s.b[1420]) {
            s.store_exp_scaled_input_ad(859, {
                if (s.v[843] > 1e-38) {
                    A::ln(s.ad_value(843))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (p[59] * 0.7));
        }
        if ((!s.b[1402]) && s.b[1420]) {s.store_offset(844, 859, 1.0);s.store_div_from_scalar(999, (p[58] * 1.9e-9), 844);s.store_div(1001, 417, 999);s.store_div_add_scaled_inputs_rhs_indices(843, 997, 997, 1.0, 1001, 1.0);s.store_mul(1002, 843, 1001);s.store_div_scaled_product_indices(1003, 842, 1002, 1.0, 997, 1.0);s.store_div_scaled_product_indices(982, 981, 1002, 1.0, 997, 1.0);}
        s.b[1445] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p[27] > 0.0));s.store_scalar(1445, if s.b[1445] { 1.0 } else { 0.0 });
        if (((!s.b[1402]) && s.b[1420]) && s.b[1445]) {s.store_add_scaled_inputs3_offset_indices(846, 829, 4.0, 1126, ((-1.0) * 4.0), 942, (-4.0), (p[1033] * 4.0));s.store_sqrt_square_offset(845, 846, 0.0001);s.store_scaled_add(847, 846, 845, 0.5);s.store_div_scaled_inputs2_indices(843, 1118, 1.0, 847, 1.0, 998, 1.0);}
        if (((!s.b[1402]) && s.b[1420]) && s.b[1445]) {
            s.store_exp_scaled_input_ad(859, {
                if (s.v[843] > 1e-38) {
                    A::ln(s.ad_value(843))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (p[59] * 0.7));
        }
        if (((!s.b[1402]) && s.b[1420]) && s.b[1445]) {s.store_offset(844, 859, 1.0);s.store_div_from_scalar(1131, (p[58] * 1.9e-9), 844);s.store_div(1132, 417, 1131);s.store_div_add_scaled_inputs_rhs_indices(843, 997, 997, 1.0, 1132, 1.0);s.store_mul(1133, 843, 1132);s.store_div_scaled_product_indices(1134, 1115, 1133, 1.0, 997, 1.0);s.store_div_scaled_product_indices(1135, 1116, 1133, 1.0, 997, 1.0);}
        if ((!s.b[1402]) && s.b[1420]) {s.store_sub(844, 875, 1004);s.store_mul(894, 861, 333);s.store_div(891, 844, 894);s.store_offset_sub(814, 891, 822, (-0.02));s.store_sqrt_add_scaled_square_input(843, 814, 1.0, 891, (4.0 * 0.02));s.store_add_scaled_inputs3_indices(877, 891, 1.0, 814, (-0.5), 843, (-0.5));s.store_mul(843, 894, 877);s.store_scaled_offset_ad(845, A::sub_scaled_inputs(s.ad_value(844), 1.0, s.ad_value(843), 0.5), 1e-20, 12.0);s.store_div(846, 843, 845);s.store_mul_sub_mixed_iia(915, 1003, 844, A::mul_sub_from_scalar_rhs(s.ad_value(843), 0.5, s.ad_value(846)));s.copy_ad(916, 915);}
        s.b[1446] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p[27] > 0.0));s.store_scalar(1446, if s.b[1446] { 1.0 } else { 0.0 });
        if (((!s.b[1402]) && s.b[1420]) && s.b[1446]) {s.store_sub(855, 1118, 1136);s.store_div(1129, 855, 894);s.store_offset_sub(814, 1129, 822, (-0.02));s.store_sqrt_add_scaled_square_input(1121, 814, 1.0, 1129, (4.0 * 0.02));s.store_add_scaled_inputs3_indices(1130, 1129, 1.0, 814, (-0.5), 1121, (-0.5));s.store_mul(1121, 894, 1130);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_60(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[1402]) && s.b[1420]) && s.b[1446]) {s.store_scaled_offset_ad(1122, A::sub_scaled_inputs(s.ad_value(855), 1.0, s.ad_value(1121), 0.5), 1e-20, 12.0);s.store_div(846, 1121, 1122);s.store_mul_sub_mixed_iia(850, 1134, 855, A::mul_sub_from_scalar_rhs(s.ad_value(1121), 0.5, s.ad_value(846)));s.store_add(915, 915, 850);s.copy_ad(916, 915);}
        s.b[1447] = (s.v[37] == 2.0);s.store_scalar(1447, if s.b[1447] { 1.0 } else { 0.0 });
        if (((!s.b[1402]) && s.b[1420]) && s.b[1447]) {s.store_scalar(1006, 0.0);}
        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1447])) {s.store_sub_from_scalar(850, 1.0, 894);s.store_mul_ad_product_rhs_mixed_ia(1006, 982, 850, A::sub_scaled_inputs(s.ad_value(877), 0.5, A::div_scaled_product(s.ad_value(843), s.ad_value(877), 1.0, s.ad_value(845), 1.0), 1.0));}
        s.b[1448] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p[27] > 0.0));s.store_scalar(1448, if s.b[1448] { 1.0 } else { 0.0 });
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1447])) && s.b[1448]) {s.store_mul_ad_product_rhs_mixed_ia(1138, 1135, 850, A::sub_scaled_inputs(s.ad_value(1130), 0.5, A::div_scaled_product(s.ad_value(1121), s.ad_value(1130), 1.0, s.ad_value(1122), 1.0), 1.0));s.store_add(1006, 1006, 1138);}
        s.b[1449] = (p[129] > 0.5);s.store_scalar(1449, if s.b[1449] { 1.0 } else { 0.0 });
        if (((!s.b[1402]) && s.b[1420]) && s.b[1449]) {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(919, 1003, 844, ((0.5) * (-1.0)), 843, ((0.25) * (-1.0)), A::div_scaled_product(s.ad_value(843), s.ad_value(843), 0.5, s.ad_value(845), 1.0), ((-1.0) * (-1.0)), 0.0);}
        s.b[1450] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p[27] > 0.0));s.store_scalar(1450, if s.b[1450] { 1.0 } else { 0.0 });
        if ((((!s.b[1402]) && s.b[1420]) && s.b[1449]) && s.b[1450]) {s.store_mul_add_scaled_inputs4_rhs_mixed_iiia(1137, 1134, 1118, ((0.5) * (-1.0)), 1136, (((-0.5)) * (-1.0)), 1121, ((0.25) * (-1.0)), A::div_scaled_product(s.ad_value(1121), s.ad_value(1121), 0.5, s.ad_value(1122), 1.0), ((-1.0) * (-1.0)));s.store_add(919, 919, 1137);}
        s.b[1451] = (p[129] < 0.5);s.store_scalar(1451, if s.b[1451] { 1.0 } else { 0.0 });
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1449])) && s.b[1451]) {s.store_scale(845, 845, 0.08333333333333333);s.store_div_scaled_inputs_square_rhs(846, 1003, 0.5, 845, 1.0);s.store_add_scaled_product_mixed_aia(847, A::mul3_scaled_output(s.ad_value(843), s.ad_value(843), s.ad_value(843), (2.0 * 0.06666666666666667)), (-1.0), 844, A::add_scaled_products(s.ad_value(843), s.ad_value(843), (2.0 * 0.3333333333333333), s.ad_value(844), A::sub_scaled_inputs(s.ad_value(844), 1.0, s.ad_value(843), (4.0 * 0.3333333333333333)), 1.0), 1.0);s.store_mul_scale_offset_indices(919, 847, 846, -1.0, 0.0);}
        s.b[1452] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p[27] > 0.0));s.store_scalar(1452, if s.b[1452] { 1.0 } else { 0.0 });
        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1449])) && s.b[1451]) && s.b[1452]) {s.store_scale(1122, 1122, 0.08333333333333333);s.store_div_scaled_inputs_square_rhs(846, 1134, 0.5, 1122, 1.0);s.store_add_scaled_product_mixed_aia(847, A::mul3_scaled_output(s.ad_value(1121), s.ad_value(1121), s.ad_value(1121), (2.0 * 0.06666666666666667)), (-1.0), 855, A::add_scaled_products(s.ad_value(1121), s.ad_value(1121), (2.0 * 0.3333333333333333), s.ad_value(855), A::sub_scaled_inputs(s.ad_value(855), 1.0, s.ad_value(1121), (4.0 * 0.3333333333333333)), 1.0), 1.0);s.store_mul_scale_offset_indices(1137, 847, 846, -1.0, 0.0);s.store_add(919, 919, 1137);}
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1449])) && (!s.b[1451])) {s.store_scale(919, 916, (-0.5));}
        s.b[1453] = (s.v[37] == 2.0);s.store_scalar(1453, if s.b[1453] { 1.0 } else { 0.0 });
        if (((!s.b[1402]) && s.b[1420]) && s.b[1453]) {s.store_scalar(939, 0.0);}
        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1453])) {s.store_scale(914, 263, (p[361] * (s.v[913] * ((((s.v[332] / p[23]) * p[3]) * s.v[366]) + p[29]))));s.store_mul_sub_rhs(939, 914, 902, 824);}
        if ((!s.b[1402]) && s.b[1420]) {s.store_add_scaled_inputs4_indices(916, 916, 1.0, 938, 1.0, 937, 1.0, 1006, -1.0);s.store_add_scaled_inputs4_indices(917, 1006, 1.0, 938, (-1.0), 937, -1.0, 939, -1.0);s.copy_ad(920, 939);s.store_add_scaled_inputs4_indices(918, 916, (-1.0), 917, (-1.0), 920, (-1.0), 919, (-1.0));}
        if ((!s.b[1402]) && (!s.b[1420])) {s.store_scalar(938, 0.0);s.store_scalar(937, 0.0);s.store_scalar(920, 0.0);s.store_scalar(917, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_61(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[1402]) && (!s.b[1420])) {s.store_scalar(919, 0.0);s.store_scalar(918, 0.0);s.store_scalar(916, 0.0);}
        s.b[1454] = (s.v[37] == 2.0);s.store_scalar(1454, if s.b[1454] { 1.0 } else { 0.0 });
        if s.b[1454] {s.store_scalar(909, 0.0);s.store_scalar(910, 0.0);}
        if (!s.b[1454]) {s.copy_ad(815, 48);s.store_scalar(980, (-p[363]));s.store_add_scaled_product_right_sub(815, 815, 1.0, 980, 409, 429, 1.0);s.store_scalar(816, p[183]);s.store_scalar(976, ((((p[185] * s.v[350]) * p[155]) * p[3]) / 1e-7));s.store_scale(979, 976, p[362]);s.store_add_scaled_product_right_sub(976, 976, 1.0, 979, 409, 429, 1.0);s.store_scalar(977, ((((p[186] * s.v[349]) * p[155]) * p[3]) / 1e-7));s.store_scale(978, 977, p[364]);s.store_add_scaled_product_right_sub(977, 977, 1.0, 978, 409, 429, 1.0);s.store_scale(994, 815, 0.9);}
        if (!s.b[1454]) {
            s.store_sub_from_scalar_div_mixed_ai(811, 1.0, {
                if (s.v[1087] > s.v[994]) {
                    s.ad_value(994)
                } else {
                    s.ad_value(1087)
                }
            }, 815);
        }
        s.b[1455] = (s.v[816] == 0.5);s.store_scalar(1455, if s.b[1455] { 1.0 } else { 0.0 });
        if ((!s.b[1454]) && s.b[1455]) {s.store_div_from_scalar_sqrt_ad(858, 1.0, s.ad_value(811));}
        if ((!s.b[1454]) && (!s.b[1455])) {
            s.store_exp_mul_scaled_lhs_mixed_ia(858, 816, -1.0, {
                if (s.v[811] > 1e-38) {
                    A::ln(s.ad_value(811))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }
        if (!s.b[1454]) {s.store_div_ad(846, A::mul_sub_from_scalar_lhs(1.0, A::mul(s.ad_value(811), s.ad_value(858)), s.ad_value(815)), A::sub_from_scalar(1.0, s.ad_value(816)));}
        s.b[1456] = (s.v[1087] > s.v[994]);s.store_scalar(1456, if s.b[1456] { 1.0 } else { 0.0 });
        if ((!s.b[1454]) && s.b[1456]) {s.store_add_scaled_product_right_sub(846, 846, 1.0, 858, 1087, 994, 1.0);}
        if (!s.b[1454]) {s.store_add_scaled_product_indices(910, 987, (p[351] * p[3]), 976, 846, 1.0);s.copy_ad(815, 41);s.store_scalar(980, (-p[365]));s.store_add_scaled_product_right_sub(815, 815, 1.0, 980, 409, 429, 1.0);s.store_scalar(816, p[184]);s.store_scale(994, 815, 0.9);}
        if (!s.b[1454]) {
            s.store_sub_from_scalar_div_mixed_ai(811, 1.0, {
                if (s.v[1088] > s.v[994]) {
                    s.ad_value(994)
                } else {
                    s.ad_value(1088)
                }
            }, 815);
        }
        s.b[1457] = (s.v[816] == 0.5);s.store_scalar(1457, if s.b[1457] { 1.0 } else { 0.0 });
        if ((!s.b[1454]) && s.b[1457]) {s.store_div_from_scalar_sqrt_ad(858, 1.0, s.ad_value(811));}
        if ((!s.b[1454]) && (!s.b[1457])) {
            s.store_exp_mul_scaled_lhs_mixed_ia(858, 816, -1.0, {
                if (s.v[811] > 1e-38) {
                    A::ln(s.ad_value(811))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }
        if (!s.b[1454]) {s.store_div_ad(846, A::mul_sub_from_scalar_lhs(1.0, A::mul(s.ad_value(811), s.ad_value(858)), s.ad_value(815)), A::sub_from_scalar(1.0, s.ad_value(816)));}
        s.b[1458] = (s.v[1088] > s.v[994]);s.store_scalar(1458, if s.b[1458] { 1.0 } else { 0.0 });
        if ((!s.b[1454]) && s.b[1458]) {s.store_add_scaled_product_right_sub(846, 846, 1.0, 858, 1088, 994, 1.0);}
        if (!s.b[1454]) {s.store_add_scaled_product_indices(909, 988, (p[351] * p[3]), 977, 846, 1.0);}
        s.store_scale(853, 897, (-p[37]));s.store_scaled_sub(854, 819, 897, p[37]);s.b[1459] = (s.v[43] != 0.0);s.store_scalar(1459, if s.b[1459] { 1.0 } else { 0.0 });s.b[1460] = (((s.v[109] > 0.0) && (p[37] > 0.0)) || ((s.v[109] < 0.0) && (p[37] < 0.0)));s.store_scalar(1460, if s.b[1460] { 1.0 } else { 0.0 });s.b[1461] = (s.v[853] < s.v[322]);s.store_scalar(1461, if s.b[1461] { 1.0 } else { 0.0 });
        if ((s.b[1459] && s.b[1460]) && s.b[1461]) {s.store_scaled_sub(86, 853, 322, s.v[52]);}
        s.b[1462] = (s.v[853] < s.v[175]);s.store_scalar(1462, if s.b[1462] { 1.0 } else { 0.0 });
        if (((s.b[1459] && s.b[1460]) && (!s.b[1461])) && s.b[1462]) {s.store_sub(843, 853, 322);s.store_square(844, 843);s.store_mul_scale_offset_mixed_ia(86, 843, A::mul_scaled_lhs(s.ad_value(176), 1.0 / (3.0), s.ad_value(844)), -1.0, s.v[52]);}
        s.b[1463] = (s.v[853] < s.v[323]);s.store_scalar(1463, if s.b[1463] { 1.0 } else { 0.0 });
        if ((((s.b[1459] && s.b[1460]) && (!s.b[1461])) && (!s.b[1462])) && s.b[1463]) {s.store_sub(843, 853, 323);s.store_square(844, 843);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_62(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1459] && s.b[1460]) && (!s.b[1461])) && (!s.b[1462])) && s.b[1463]) {s.store_add_ad(86, A::add_scaled_product(s.ad_value(56), 1.0, s.ad_value(53), s.ad_value(853), 1.0), A::mul3_scaled_output(s.ad_value(177), s.ad_value(843), s.ad_value(844), 1.0 / (3.0)));}
        if ((((s.b[1459] && s.b[1460]) && (!s.b[1461])) && (!s.b[1462])) && (!s.b[1463])) {s.store_add_scaled_product_indices(86, 56, 1.0, 53, 853, 1.0);}
        s.b[1464] = (s.v[853] < s.v[323]);s.store_scalar(1464, if s.b[1464] { 1.0 } else { 0.0 });
        if ((s.b[1459] && (!s.b[1460])) && s.b[1464]) {s.store_mul_sub_rhs(86, 53, 853, 323);}
        s.b[1465] = (s.v[853] < s.v[175]);s.store_scalar(1465, if s.b[1465] { 1.0 } else { 0.0 });
        if (((s.b[1459] && (!s.b[1460])) && (!s.b[1464])) && s.b[1465]) {s.store_sub(843, 853, 323);s.store_square(844, 843);s.store_mul_add_scaled_product_rhs_indices(86, 843, 53, 1.0, 176, 844, (-1.0 / (3.0)));}
        s.b[1466] = (s.v[853] < s.v[322]);s.store_scalar(1466, if s.b[1466] { 1.0 } else { 0.0 });
        if ((((s.b[1459] && (!s.b[1460])) && (!s.b[1464])) && (!s.b[1465])) && s.b[1466]) {s.store_sub(843, 853, 322);s.store_square(844, 843);s.store_add_scaled_inputs3_mixed_iia(86, 853, s.v[52], 56, 1.0, A::mul3_scaled_output(s.ad_value(177), s.ad_value(843), s.ad_value(844), 1.0 / (3.0)), 1.0);}
        if ((((s.b[1459] && (!s.b[1460])) && (!s.b[1464])) && (!s.b[1465])) && (!s.b[1466])) {s.store_add_scaled_inputs(86, 853, s.v[52], 56, 1.0);}
        s.b[1467] = (((s.v[109] > 0.0) && (p[37] > 0.0)) || ((s.v[109] < 0.0) && (p[37] < 0.0)));s.store_scalar(1467, if s.b[1467] { 1.0 } else { 0.0 });s.b[1468] = (s.v[854] < s.v[322]);s.store_scalar(1468, if s.b[1468] { 1.0 } else { 0.0 });
        if ((s.b[1459] && s.b[1467]) && s.b[1468]) {s.store_scaled_sub(87, 854, 322, s.v[54]);}
        s.b[1469] = (s.v[854] < s.v[175]);s.store_scalar(1469, if s.b[1469] { 1.0 } else { 0.0 });
        if (((s.b[1459] && s.b[1467]) && (!s.b[1468])) && s.b[1469]) {s.store_sub(843, 854, 322);s.store_square(844, 843);s.store_mul_scale_offset_mixed_ia(87, 843, A::mul_scaled_lhs(s.ad_value(178), 1.0 / (3.0), s.ad_value(844)), -1.0, s.v[54]);}
        s.b[1470] = (s.v[854] < s.v[323]);s.store_scalar(1470, if s.b[1470] { 1.0 } else { 0.0 });
        if ((((s.b[1459] && s.b[1467]) && (!s.b[1468])) && (!s.b[1469])) && s.b[1470]) {s.store_sub(843, 854, 323);s.store_square(844, 843);s.store_add_ad(87, A::add_scaled_product(s.ad_value(57), 1.0, s.ad_value(55), s.ad_value(854), 1.0), A::mul3_scaled_output(s.ad_value(179), s.ad_value(843), s.ad_value(844), 1.0 / (3.0)));}
        if ((((s.b[1459] && s.b[1467]) && (!s.b[1468])) && (!s.b[1469])) && (!s.b[1470])) {s.store_add_scaled_product_indices(87, 57, 1.0, 55, 854, 1.0);}
        s.b[1471] = (s.v[854] < s.v[323]);s.store_scalar(1471, if s.b[1471] { 1.0 } else { 0.0 });
        if ((s.b[1459] && (!s.b[1467])) && s.b[1471]) {s.store_mul_sub_rhs(87, 55, 854, 323);}
        s.b[1472] = (s.v[854] < s.v[175]);s.store_scalar(1472, if s.b[1472] { 1.0 } else { 0.0 });
        if (((s.b[1459] && (!s.b[1467])) && (!s.b[1471])) && s.b[1472]) {s.store_sub(843, 854, 323);s.store_square(844, 843);s.store_mul_add_scaled_product_rhs_indices(87, 843, 55, 1.0, 178, 844, (-1.0 / (3.0)));}
        s.b[1473] = (s.v[854] < s.v[322]);s.store_scalar(1473, if s.b[1473] { 1.0 } else { 0.0 });
        if ((((s.b[1459] && (!s.b[1467])) && (!s.b[1471])) && (!s.b[1472])) && s.b[1473]) {s.store_sub(843, 854, 322);s.store_square(844, 843);s.store_add_scaled_inputs3_mixed_iia(87, 854, s.v[54], 57, 1.0, A::mul3_scaled_output(s.ad_value(179), s.ad_value(843), s.ad_value(844), 1.0 / (3.0)), 1.0);}
        if ((((s.b[1459] && (!s.b[1467])) && (!s.b[1471])) && (!s.b[1472])) && (!s.b[1473])) {s.store_add_scaled_inputs(87, 854, s.v[54], 57, 1.0);}
        if (!s.b[1459]) {s.store_scale(86, 853, s.v[52]);s.store_scale(87, 854, s.v[54]);}
        s.store_add_scaled_product_indices(86, 86, 1.0, 58, 853, 1.0);s.store_add_scaled_product_indices(87, 87, 1.0, 59, 854, 1.0);s.b[1474] = (p[39] == 3.0);s.store_scalar(1474, if s.b[1474] { 1.0 } else { 0.0 });
        if s.b[1474] {s.store_offset(843, 1019, 0.02);}
        if (!s.b[1474]) {s.store_offset(843, 820, 0.02);}
        s.store_sqrt_square_offset(844, 843, (4.0 * 0.02));s.store_scaled_sub(845, 843, 844, 0.5);s.store_scale(846, 237, s.v[349]);s.store_sqrt_sub_from_scalar_ad(847, 1.0, A::div_scaled_inputs(s.ad_value(845), 4.0, s.ad_value(238), 1.0));s.b[1475] = (p[39] == 3.0);s.store_scalar(1475, if s.b[1475] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_63(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1475] {s.store_add_scaled_products_mixed_aiia(895, A::add(s.ad_value(335), s.ad_value(846)), 1019, 1.0, 846, A::add_scaled_offset_product_rhs(s.ad_value(845), 1.0, s.ad_value(238), s.ad_value(847), (-1.0), 0.5), (-1.0));}
        if (!s.b[1475]) {s.store_add_scaled_products_mixed_aiia(895, A::add(s.ad_value(335), s.ad_value(846)), 820, 1.0, 846, A::add_scaled_offset_product_rhs(s.ad_value(845), 1.0, s.ad_value(238), s.ad_value(847), (-1.0), 0.5), (-1.0));}
        s.b[1476] = (p[39] == 3.0);s.store_scalar(1476, if s.b[1476] { 1.0 } else { 0.0 });
        if s.b[1476] {s.store_offset(843, 1018, 0.02);}
        if (!s.b[1476]) {s.store_offset(843, 821, 0.02);}
        s.store_sqrt_square_offset(844, 843, (4.0 * 0.02));s.store_scaled_sub(845, 843, 844, 0.5);s.store_scale(846, 236, s.v[350]);s.store_sqrt_sub_from_scalar_ad(847, 1.0, A::div_scaled_inputs(s.ad_value(845), 4.0, s.ad_value(238), 1.0));s.b[1477] = (p[39] == 3.0);s.store_scalar(1477, if s.b[1477] { 1.0 } else { 0.0 });
        if s.b[1477] {s.store_add_scaled_products_mixed_aiia(896, A::add(s.ad_value(334), s.ad_value(846)), 1018, 1.0, 846, A::add_scaled_offset_product_rhs(s.ad_value(845), 1.0, s.ad_value(238), s.ad_value(847), (-1.0), 0.5), (-1.0));}
        if (!s.b[1477]) {s.store_add_scaled_products_mixed_aiia(896, A::add(s.ad_value(334), s.ad_value(846)), 821, 1.0, 846, A::add_scaled_offset_product_rhs(s.ad_value(845), 1.0, s.ad_value(238), s.ad_value(847), (-1.0), 0.5), (-1.0));}
        s.b[1478] = (p[3] != 1.0);s.store_scalar(1478, if s.b[1478] { 1.0 } else { 0.0 });
        if s.b[1478] {s.store_scale(895, 895, p[3]);s.store_scale(896, 896, p[3]);}
        s.b[1505] = (p[223] == 0.0);s.store_scalar(1505, if s.b[1505] { 1.0 } else { 0.0 });s.b[1506] = (p[223] == 1.0);s.store_scalar(1506, if s.b[1506] { 1.0 } else { 0.0 });s.b[1507] = (p[223] == 2.0);s.store_scalar(1507, if s.b[1507] { 1.0 } else { 0.0 });s.b[1508] = (p[223] == 3.0);s.store_scalar(1508, if s.b[1508] { 1.0 } else { 0.0 });
        if (s.b[1506] && (!s.b[1505])) {s.store_add_scaled_inputs3_indices(843, 83, 1.0, 84, 1.0, 85, 1.0);s.store_square(843, 843);s.store_div_scaled_inputs_indices(1486, 946, 2.0, 75, 1.0);s.store_div_scaled_inputs_indices(848, 72, 1.0, 1486, s.v[327]);s.store_square(848, 848);s.store_offset_scaled(1487, 848, (((p[227] * s.v[327])) * (p[229])), p[229]);s.store_offset_scaled(1488, 848, (((p[228] * s.v[327])) * (p[230])), p[230]);}
        s.b[1509] = (s.v[1488] > 0.9);s.store_scalar(1509, if s.b[1509] { 1.0 } else { 0.0 });
        if ((s.b[1506] && (!s.b[1505])) && s.b[1509]) {s.store_scalar(1488, 0.9);}
        s.b[1510] = (s.v[1488] > (0.9 * s.v[1487]));s.store_scalar(1510, if s.b[1510] { 1.0 } else { 0.0 });
        if ((s.b[1506] && (!s.b[1505])) && s.b[1510]) {s.store_scale(1488, 1487, 0.9);}
        if (s.b[1506] && (!s.b[1505])) {s.store_add_scaled_product_mixed_iia(844, 84, 1.0, 1487, A::add(s.ad_value(83), s.ad_value(85)), 1.0);s.store_div_scaled_product_indices(845, 844, 844, 1.0, 78, 1.0);}
        if (s.b[1508] && (!((s.b[1505] || s.b[1506]) || s.b[1507]))) {s.store_sub_from_scalar_scaled_mul(1491, 1.0, 77, 76, 1.0);s.store_sub_from_scalar(843, 1.0, 1491);s.store_offset(844, 1491, 1.0);s.store_add_mixed_ia(845, 844, A::div_scaled_product_offset_denominator(s.ad_value(74), s.ad_value(49), 2.0, s.ad_value(72), 1e-10, 1.0));s.store_offset_scaled_div(1495, 77, 838, s.v[892], s.v[892]);s.store_div_from_scalar(849, s.v[892], 1495);s.store_mul_add_scaled_inputs_rhs_mixed_ia(1492, 849, 844, 0.5, A::div_scaled_product(s.ad_value(843), s.ad_value(843), 1.0, s.ad_value(845), 6.0), 1.0);s.store_square(846, 845);s.store_square(847, 843);s.store_square(848, 846);}
    }
}
