#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_50(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1224] {s.store_mul_div_scaled_inputs_indices(1240, 1244, 227, 2.0, 1231, 1.0);s.store_mul_product3_indices(204, 337, 1240, 1239, 107, 1.0);s.store_add(199, 202, 204);}
        s.store_add(201, 203, 204);s.b[1295] = ((p[43] == 1.0) || (p[45] == 1.0));s.store_scalar(1295, if s.b[1295] { 1.0 } else { 0.0 });s.b[1308] = ((s.v[145] == 1.0) || (p[25] == 0.0));s.store_scalar(1308, if s.b[1308] { 1.0 } else { 0.0 });
        if (s.b[1295] && s.b[1308]) {s.store_scalar(263, 0.0);}
        s.b[1309] = ((p[117] <= 0.0) || (s.v[73] <= 0.0));s.store_scalar(1309, if s.b[1309] { 1.0 } else { 0.0 });
        if ((s.b[1295] && (!s.b[1308])) && s.b[1309]) {s.store_scalar(263, 0.0);}
        if ((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) {s.store_offset_add_scaled_inputs3_offset_indices(445, 174, 1.0, 185, 1.0, 320, -1.0, (-s.v[136]), p[48]);}
        s.b[1310] = (p[44] <= 0.0);s.store_scalar(1310, if s.b[1310] { 1.0 } else { 0.0 });
        if (((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && s.b[1310]) {s.copy_ad(1296, 445);s.store_square(1303, 323);s.copy_ad(1304, 545);s.store_div(1298, 1304, 1303);s.store_div_from_scalar(1305, 2.0, 1304);s.store_mul(1299, 1305, 1303);s.store_add_scaled_inputs_product_indices(1300, 1296, 1.0, 227, (-1.0), 130, 514, (-1.0));s.store_scale(483, 393, (p[49] * 1.0 / (s.v[89])));s.store_add_scaled_product_indices(1300, 1300, 1.0, 130, 483, (-1.0));s.store_offset_mul(1302, 1299, 1300, 1.0);s.store_sqrt_square_offset(44, 1302, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(1301, 1302, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[1311] = (s.v[1301] < 0.0);s.store_scalar(1311, if s.b[1311] { 1.0 } else { 0.0 });
        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && s.b[1310]) && s.b[1311]) {s.store_scalar(1301, 0.0);}
        if (((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && s.b[1310]) {s.store_offset(1301, 1301, 1e-50);s.store_sqrt(1301, 1301);s.store_add_scaled_product_mixed_aii(1306, A::mul_sub_from_scalar_rhs(s.ad_value(1298), 1.0, s.ad_value(1301)), 1.0, 1296, 137, 1.0);s.store_add_scaled_inputs3_mixed_iia(1307, 173, p[122], 176, 1.0, A::mul3(s.ad_value(131), s.ad_value(129), s.ad_value(1306)), -1.0);s.store_sqrt_square_offset(44, 1307, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1307, 1307, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1312] = (s.v[1307] < 0.0);s.store_scalar(1312, if s.b[1312] { 1.0 } else { 0.0 });
        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && s.b[1310]) && s.b[1312]) {s.store_scalar(1307, 0.0);}
        if (((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) {s.store_mul(1296, 134, 445);s.store_div_square_rhs(1298, 545, 323);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1299, 2.0, 545, A::square(s.ad_value(323)));s.store_add_scaled_inputs_product_indices(1300, 1296, 1.0, 227, (-1.0), 130, 514, (-1.0));s.store_scale(483, 393, (p[49] * 1.0 / (s.v[89])));s.store_add_scaled_product_indices(1300, 1300, 1.0, 130, 483, (-1.0));s.store_offset_mul(1301, 1299, 1300, 1.0);s.store_scaled_offset(1303, 1299, 1.0, 2.0);}
        s.b[1313] = ((s.v[1301] < (1e-50 + s.v[1303])) && (s.v[1303] >= 0.0));s.store_scalar(1313, if s.b[1313] { 1.0 } else { 0.0 });
        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) {s.store_sub_offset_lhs(44, 1303, 1e-50, 1301);s.store_square(49, 44);s.store_square(50, 1303);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);s.store_scalar(54, 0.0);s.store_scalar(55, 0.0);s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[1314] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1314, if s.b[1314] { 1.0 } else { 0.0 });s.b[1315] = (4.0 == 1.0);s.store_scalar(1315, if s.b[1315] { 1.0 } else { 0.0 });
        if ((((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) && s.b[1315]) {s.store_scalar(55, 1.0);}
        s.b[1316] = (4.0 == 2.0);s.store_scalar(1316, if s.b[1316] { 1.0 } else { 0.0 });
        if (((((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) && (!s.b[1315])) && s.b[1316]) {s.store_scalar(55, 2.0);}
        s.b[1317] = (4.0 == 4.0);s.store_scalar(1317, if s.b[1317] { 1.0 } else { 0.0 });
        if ((((((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) && (!s.b[1315])) && (!s.b[1316])) && s.b[1317]) {s.store_scalar(55, 3.0);}
        s.b[1318] = (4.0 == 8.0);s.store_scalar(1318, if s.b[1318] { 1.0 } else { 0.0 });
        if (((((((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) && (!s.b[1315])) && (!s.b[1316])) && (!s.b[1317])) && s.b[1318]) {s.store_scalar(55, 4.0);}
        if (((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) {s.store_scalar(54, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_51(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let mut t1: usize = 0;
        while {
            let t0: f64 = if ((((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t0 != 0.0
        } {
            t1 += 1;
            if t1 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) {s.store_sqrt(53, 53);s.store_primal_offset(54, 54, 1.0);}
        }
        if (((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && (!s.b[1314])) {s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));}
        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_lhs(43, 44, 1303, 53);s.store_sub_offset_lhs(1301, 1303, 1e-50, 43);}
        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && (!s.b[1313])) {
        }
        if (((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) {
            if (s.v[1301] <= 0.0) {
                s.store_scalar(1301, 0.0);
            } else {
                s.store_sqrt(1301, 1301);
            }
        }
        if (((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) {s.store_add_mul_sub_from_scalar_rhs_indices(1306, 1296, 1298, 1.0, 1301);s.store_div_from_scalar_offset_input(1297, s.v[100], 131, s.v[100]);s.store_add_scaled_inputs_product_indices(1307, 173, p[122], 176, 1.0, 1297, 1306, (-1.0));s.store_sqrt_square_offset(44, 1307, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(1307, 1307, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[1319] = (s.v[1307] < 0.0);s.store_scalar(1319, if s.b[1319] { 1.0 } else { 0.0 });
        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1319]) {s.store_scalar(1307, 0.0);}
        if ((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) {s.store_offset(1307, 1307, 1e-50);s.store_ad_value(1297, A::exp_div_scaled_inputs(s.ad_value(133), -1.0, s.ad_value(1307), 1.0));s.store_mul_product3_indices(263, 1297, 132, 1307, 199, 1.0);}
        s.b[1320] = (((p[25] == 1.0) && (p[26] == 2.0)) && (p[43] == 1.0));s.store_scalar(1320, if s.b[1320] { 1.0 } else { 0.0 });
        if s.b[1320] {s.store_mul_scaled_exp_scaled_input_rhs(1321, 107, (1.6021918e-19 * p[237]), 225, (-p[141]));s.store_offset_scaled(1322, 544, (((((36.0 * 1e-7) / 0.0001)) as f64).sqrt() * 13.0), ((((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * 36.0) * (1e20 / 1e-6)));s.store_div_scalar_by_product_indices(1323, (((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * ((((36.0 * 1e-7) / 0.0001)) as f64).sqrt()), 1321, 1322, 1.0);s.store_mul(567, 263, 1323);s.store_mul_scaled_ln_offset_rhs(1324, 227, p[140], 567, 1.0);s.store_add_scaled_inputs3_indices(44, 231, 1.0, 1324, (-1.0), 231, (-0.01));s.store_scaled_mul(45, 231, 231, (4.0 * 0.01));}
        if s.b[1320] {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if s.b[1320] {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(1324, 231, 1.0, 44, (-0.5), 45, (-0.5));s.store_sqrt_mul_scaled_lhs(1325, 544, ((2.0 * 1.034943e-10) * 1.6021918e-19), 227);s.store_add_scaled_product_mixed_aia(1326, A::offset(A::exp(A::mul_scaled_lhs(s.ad_value(225), -1.0, A::sub(s.ad_value(176), s.ad_value(1324)))), (-1.0)), 1.0, 225, A::sub(s.ad_value(176), s.ad_value(1324)), 1.0);}
        if s.b[1320] {
            if (s.v[1326] > 0.0) {
                s.store_sqrt(1326, 1326);
            } else {
                s.store_scaled_sqrt_scaled_input(1326, 1326, -1.0, -1.0);
            }
        }
        if s.b[1320] {s.store_sqrt_ad(1327, A::add_scaled_product(A::offset(A::exp(A::mul_scaled_lhs(s.ad_value(225), -1.0, s.ad_value(176))), (-1.0)), 1.0, s.ad_value(225), s.ad_value(176), 1.0));s.store_mul_sub_scaled_inputs_rhs_indices(1328, 1325, 1326, -1.0, 1327, -1.0);s.store_offset_sub_from_scalar_ad(44, p[47], s.ad_value(1328), (-(p[47] * 0.01)));s.store_scalar(45, ((4.0 * p[47]) * (p[47] * 0.01)));}
        if s.b[1320] {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if s.b[1320] {s.store_sqrt_square_add(45, 44, 45);s.store_offset_add_scaled_inputs_indices(393, 44, (-0.5), 45, (-0.5), p[47]);}
        if s.b[1320] {s.store_scalar(1321, (if (p[138] > 0.0) { p[138] } else { 1.0 }));}
        if s.b[1320] {s.store_scaled_voltage(596, ctx, nodes, Some(17), None, (1e-9 / 0.0001));s.copy_ad(393, 596);}
        s.b[1342] = (((s.v[145] == 0.0) && (s.v[263] > 0.0)) && (p[146] != 0.0));s.store_scalar(1342, if s.b[1342] { 1.0 } else { 0.0 });s.b[1343] = (s.v[56] < 3.0);s.store_scalar(1343, if s.b[1343] { 1.0 } else { 0.0 });
        if (s.b[1342] && s.b[1343]) {s.store_scalar(516, 0.0);s.store_scalar(517, 0.0);}
        if (s.b[1342] && (!s.b[1343])) {
            if (p[43] == 1.0) {
                s.copy_ad(516, 156);
            } else {
                s.copy_ad(516, 350);
            }
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_52(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1342] && (!s.b[1343])) {
            if (p[43] == 1.0) {
                s.copy_ad(517, 156);
            } else {
                s.copy_ad(517, 353);
            }
        }
        if s.b[1342] {s.store_offset_scaled(1329, 185, p[147], 1.0);s.store_scaled_mul(1330, 1329, 263, p[146]);s.store_offset_mul_ad(1331, s.ad_value(225), A::sub(s.ad_value(161), s.ad_value(516)), (-1.0));s.store_sqrt_square_offset(44, 1331, ((4.0 * 0.1) * 0.1));s.store_offset_add_scaled_inputs_indices(1331, 1331, 0.5, 44, 0.5, (1e-10 * 0.1));}
        s.b[1344] = (s.v[1331] < 0.0);s.store_scalar(1344, if s.b[1344] { 1.0 } else { 0.0 });
        if (s.b[1342] && s.b[1344]) {s.store_scalar(1331, 0.0);}
        if s.b[1342] {s.store_sqrt(1332, 1331);s.store_mul(1333, 1331, 1332);s.store_offset_mul_ad(1334, s.ad_value(225), A::sub(s.ad_value(162), s.ad_value(517)), (-1.0));s.store_sqrt_square_offset(44, 1334, ((4.0 * 0.1) * 0.1));s.store_offset_add_scaled_inputs_indices(1334, 1334, 0.5, 44, 0.5, (1e-10 * 0.1));}
        s.b[1345] = (s.v[1334] < 0.0);s.store_scalar(1345, if s.b[1345] { 1.0 } else { 0.0 });
        if (s.b[1342] && s.b[1345]) {s.store_scalar(1334, 0.0);}
        if s.b[1342] {s.store_sqrt(1335, 1334);s.store_mul(1336, 1334, 1335);s.store_div_from_scalar(1337, 1.0, 1331);s.store_mul3_lhs(328, 225, 1330, 1337);s.store_div_from_scalar(1337, 1.0, 1334);s.store_mul3_lhs(1338, 225, 1330, 1337);s.store_mul_mixed_ia(1339, 238, A::add_scaled_products(s.ad_value(1336), s.ad_value(1338), 1.0, s.ad_value(1333), s.ad_value(328), (-1.0)));s.store_mul_add_scaled_products_indices_rhs(1340, 238, 1335, 1338, ((-1.0) * (0.5)), 1332, 328, 0.5);s.store_add(1341, 1339, 1340);s.store_mul3_lhs(265, 264, 1341, 250);}
        s.store_scalar(1359, (s.v[88] * 100.0));s.store_scale(1360, 323, 0.0001);s.store_scalar(1361, (s.v[97] * 100.0));s.store_primal_scale(1362, 107, 100.0);s.store_scale(1363, 252, 0.01);s.store_scale(1364, 436, 0.0001);s.store_scale(1365, 238, 0.0001);s.b[1366] = (p[27] == 0.0);s.store_scalar(1366, if s.b[1366] { 1.0 } else { 0.0 });s.b[1367] = (s.v[145] == 0.0);s.store_scalar(1367, if s.b[1367] { 1.0 } else { 0.0 });
        if ((!s.b[1366]) && s.b[1367]) {s.store_offset_add(1358, 176, 173, (-(10.0 * 2.220446049250313e-16)));s.store_add_scaled_inputs4_offset_indices(1348, 174, 1.0, 185, (p[216] * s.v[1361]), 320, (-(p[216] * s.v[1361])), 1358, (-p[215]), (-s.v[123]));s.store_scalar(1350, (1.0 / s.v[1359]));s.store_mul(1349, 1348, 1350);s.store_scalar(1350, (1.0 / p[217]));s.store_offset_mul(1354, 1363, 1350, 1.0);s.store_mul(1357, 1349, 1354);s.store_sqrt_square_offset(44, 1357, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1357, 1357, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1368] = (s.v[1357] < 0.0);s.store_scalar(1368, if s.b[1368] { 1.0 } else { 0.0 });
        if (((!s.b[1366]) && s.b[1367]) && s.b[1368]) {s.store_scalar(1357, 0.0);}
        if ((!s.b[1366]) && s.b[1367]) {s.store_sqrt_square_offset(44, 174, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(1350, 174, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[1369] = (s.v[1350] < 0.0);s.store_scalar(1369, if s.b[1369] { 1.0 } else { 0.0 });
        if (((!s.b[1366]) && s.b[1367]) && s.b[1369]) {s.store_scalar(1350, 0.0);}
        if ((!s.b[1366]) && s.b[1367]) {s.store_offset(1350, 1350, (-p[226]));s.store_scale(1346, 1350, 10.0);s.store_offset_square(1349, 1346, 1.0);s.store_sub_from_scalar_ad(1348, 1.0, A::div_from_scalar(1.0, s.ad_value(1349)));s.store_mul(1357, 1357, 1348);s.store_scale(1347, 1362, s.v[1361]);s.store_div_from_scalar_offset_input(1354, p[219], 1347, p[219]);s.store_scalar(1353, p[218]);s.store_div_from_scalar_offset_input(1351, 1.0, 1357, 1e-50);s.store_scaled_mul(1348, 303, 1351, (-p[214]));}
        s.b[1370] = (s.v[1348] < (-34.0));s.store_scalar(1370, if s.b[1370] { 1.0 } else { 0.0 });
        if (((!s.b[1366]) && s.b[1367]) && (!s.b[1370])) {s.store_exp(1349, 1348);s.store_mul_scale_offset_mixed_ia(1350, 1347, A::div_from_scalar(p[213], s.ad_value(302)), 1.6021918e-19, 0.0);s.store_div_from_scalar(1352, 1.0, 1365);s.store_sqrt_mul_ad(1353, A::add_scaled_inputs(s.ad_value(1364), 1.0, s.ad_value(1360), 1e-12), s.ad_value(1352));s.store_mul3_lhs(1351, 1349, 1350, 1353);}
        if (!s.b[1366]) {s.store_offset_scaled(1347, 158, (-p[221]), p[222]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_53(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1366]) {s.store_exp_scaled_input(1349, 1347, s.v[1359]);s.store_scale(1347, 158, (1.0 / (s.v[1359]) * 1.0 / (s.v[1359])));s.store_mul(1350, 158, 1347);s.store_scale(1351, 1362, (p[220] / 1000000.0));s.store_sub(1348, 158, 157);s.store_offset_scaled(1347, 1348, (-p[221]), p[222]);s.store_exp_scaled_input(1349, 1347, s.v[1359]);s.store_scale(1347, 1348, (1.0 / (s.v[1359]) * 1.0 / (s.v[1359])));s.store_mul(1350, 1348, 1347);s.store_scale(1351, 1362, (p[220] / 1000000.0));s.store_offset_scaled_sub(1357, 513, 158, 1.0 / (s.v[1359]), ((((s.v[123]) + (p[225]))) * (1.0 / (s.v[1359]))));s.store_sqrt_square_offset(44, 1357, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1357, 1357, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1373] = (s.v[1357] < 0.0);s.store_scalar(1373, if s.b[1373] { 1.0 } else { 0.0 });
        if ((!s.b[1366]) && s.b[1373]) {s.store_scalar(1357, 0.0);}
        if (!s.b[1366]) {s.store_offset(1357, 1357, 1e-50);s.store_div_from_scalar(1348, (-p[224]), 1357);}
        s.b[1374] = (s.v[1348] < (-34.0));s.store_scalar(1374, if s.b[1374] { 1.0 } else { 0.0 });
        if ((!s.b[1366]) && (!s.b[1374])) {s.store_exp(1349, 1348);s.store_scale(1350, 1362, (p[223] * s.v[1361]));}
        s.b[1382] = (p[28] == 0.0);s.store_scalar(1382, if s.b[1382] { 1.0 } else { 0.0 });
        if (!s.b[1382]) {s.store_add_scaled_inputs4_offset_indices(1375, 157, p[209], 158, (-1.0), 187, p[211], 319, p[211], (p[210] * p[209]));s.store_scalar(1376, (1.0 / s.v[88]));s.store_mul(1377, 1375, 1376);s.store_sqrt_square_offset(44, 1377, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(304, 1377, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1383] = (s.v[304] < 0.0);s.store_scalar(1383, if s.b[1383] { 1.0 } else { 0.0 });
        if ((!s.b[1382]) && s.b[1383]) {s.store_scalar(304, 0.0);}
        if (!s.b[1382]) {s.store_div_from_scalar_offset_input(1378, 1.0, 304, 1e-50);s.store_scaled_mul(1379, 303, 1378, (-p[208]));}
        s.b[1384] = (s.v[1379] < (-34.0));s.store_scalar(1384, if s.b[1384] { 1.0 } else { 0.0 });
        if ((!s.b[1382]) && (!s.b[1384])) {s.store_exp(1375, 1379);s.store_mul_scale_offset_mixed_ia(1376, 107, A::div_from_scalar(p[207], s.ad_value(302)), 1.6021918e-19, 0.0);}
        if (!s.b[1382]) {s.store_sub(1381, 157, 513);}
        s.b[1385] = (s.v[1381] > 0.0);s.store_scalar(1385, if s.b[1385] { 1.0 } else { 0.0 });
        if ((!s.b[1382]) && s.b[1385]) {s.store_square(1376, 1381);s.store_mul(331, 1376, 1381);s.store_offset(1379, 331, p[212]);}
        s.b[1393] = (p[28] == 0.0);s.store_scalar(1393, if s.b[1393] { 1.0 } else { 0.0 });
        if (!s.b[1393]) {s.store_add_scaled_inputs3_mixed_aii(1386, A::add_scaled_inputs3_offset(s.ad_value(157), (-p[209]), s.ad_value(158), -1.0, s.ad_value(157), 1.0, ((p[210]) * (p[209]))), 1.0, 187, p[211], 319, p[211]);s.store_scalar(1387, (1.0 / s.v[88]));s.store_mul(1388, 1386, 1387);s.store_sqrt_square_offset(44, 1388, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(305, 1388, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1394] = (s.v[305] < 0.0);s.store_scalar(1394, if s.b[1394] { 1.0 } else { 0.0 });
        if ((!s.b[1393]) && s.b[1394]) {s.store_scalar(305, 0.0);}
        if (!s.b[1393]) {s.store_div_from_scalar_offset_input(1389, 1.0, 305, 1e-50);s.store_scaled_mul(1390, 303, 1389, (-p[208]));}
        s.b[1395] = (s.v[1390] < (-34.0));s.store_scalar(1395, if s.b[1395] { 1.0 } else { 0.0 });
        if ((!s.b[1393]) && (!s.b[1395])) {s.store_exp(1386, 1390);s.store_div_from_scalar(1389, 1.0, 302);s.store_scaled_mul(1387, 1389, 107, (p[207] * 1.6021918e-19));}
        if (!s.b[1393]) {s.store_neg(1392, 513);}
        s.b[1396] = (s.v[1392] > 0.0);s.store_scalar(1396, if s.b[1396] { 1.0 } else { 0.0 });
        if ((!s.b[1393]) && s.b[1396]) {s.store_square(1387, 1392);s.store_mul(331, 1387, 1392);s.store_offset(1390, 331, p[212]);}
        s.b[1397] = (p[43] == 1.0);s.store_scalar(1397, if s.b[1397] { 1.0 } else { 0.0 });
        if s.b[1397] {s.store_scalar(1407, s.v[91]);s.store_primal_div_from_scalar(1408, 1.0, 1407);s.store_scalar(1464, 0.0);s.store_scalar(1466, 0.0);s.store_scalar(1468, 0.0);s.store_neg(1400, 534);s.store_mul(1401, 1400, 436);s.store_add_scaled_product_indices(331, 1401, 1.0, 1400, 437, 1.0);s.store_mul(470, 1401, 438);s.store_sub(469, 1401, 470);s.store_mul(468, 331, 438);s.store_sub(467, 331, 468);}
        if (s.b[1397] && (p[24] != 0.0)) {s.copy_ad(521, 536);s.store_scalar(528, 0.0);}
        s.b[1477] = (1.0 == 1.0);s.store_scalar(1477, if s.b[1477] { 1.0 } else { 0.0 });s.b[1478] = (1.0 == 2.0);s.store_scalar(1478, if s.b[1478] { 1.0 } else { 0.0 });
        if ((s.b[1397] && (p[24] != 0.0)) && s.b[1477]) {s.store_primal_scale(522, 533, 0.5);s.store_scalar(523, p[292]);s.store_scalar(528, s.v[525]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_54(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1397] && (p[24] != 0.0)) && (s.b[1478] && (!s.b[1477]))) {s.store_primal_scale(522, 534, 0.5);s.store_scalar(523, p[68]);s.store_scalar(528, s.v[524]);s.store_scalar(528, 1.0);}
        s.b[1479] = (s.v[528] == 0.0);s.store_scalar(1479, if s.b[1479] { 1.0 } else { 0.0 });
        if ((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) {s.store_mul_sqrt_mixed_ia(1427, 238, A::div(s.ad_value(521), s.ad_value(536)));s.store_scalar(1409, ((1.0 - -1.0) / 2.0));s.store_scalar(1410, ((1.0 + -1.0) / 2.0));s.store_add_scaled_products_mixed_iiia(1420, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);s.store_add_scaled_products_indices(1421, 461, 157, 1.0, 462, 157, -1.0);s.store_add_scaled_products_mixed_iiia(1422, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_add_scaled_products_mixed_iiia(1423, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_sub(1424, 1421, 1420);s.store_neg(1425, 1420);s.store_primal_add_scaled_products_indices(1411, 1409, 461, 1.0, 1410, 462, 1.0);s.store_primal_add_scaled_products_indices(1412, 1409, 462, 1.0, 1410, 461, 1.0);s.store_add_scaled_products_indices(1426, 1411, 1422, 1.0, 1412, 1423, 1.0);s.store_offset_ad(1418, A::add_scaled_products(s.ad_value(1411), s.ad_value(1425), 1.0, s.ad_value(1412), s.ad_value(1424), 1.0), (10.0 * 2.220446049250313e-16));s.store_neg(1398, 1418);}
        s.b[1480] = (s.v[1398] > s.v[141]);s.store_scalar(1480, if s.b[1480] { 1.0 } else { 0.0 });
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && s.b[1480]) {s.store_sub(1399, 1398, 141);s.store_sub(1400, 140, 141);s.store_div(44, 1399, 1400);s.store_square(45, 44);s.store_mul(46, 45, 44);s.store_square(47, 45);s.store_div_from_scalar_ad(1406, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));s.store_mul_scale_offset_indices(1406, 1400, 1406, -1.0, 1.0);s.store_add(1403, 141, 1406);}
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1480])) {s.copy_ad(1403, 1398);}
        if ((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) {s.store_offset_scaled(1419, 1403, -1.0, (-1e-12));s.store_mul(1428, 1427, 1408);s.store_square(1429, 1428);s.store_sub(1430, 1426, 523);s.store_div(1398, 521, 230);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1431, 2.0, 225, A::ln(s.ad_value(1398)));s.store_neg(1432, 1419);}
        s.b[1481] = (s.v[1430] < s.v[1432]);s.store_scalar(1481, if s.b[1481] { 1.0 } else { 0.0 });
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && s.b[1481]) {s.store_div_scalar_by_product_indices(1399, 1.0, 225, 1427, 1.0);s.store_mul(1406, 1399, 1407);s.store_offset_scaled(1433, 1406, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(1434, 1433, 1433, 8.0, 0.0, 1433);s.store_sub(1435, 237, 1431);s.store_mul_add_rhs(1405, 225, 1430, 1419);s.store_sub_from_scalar_scaled_mul_mixed_ia(1436, (7.0 * 1.414213562373095), 1406, A::offset(s.ad_value(1405), (-2.0)), 9.0);s.store_square(1437, 1436);}
        s.b[1482] = (s.v[1434] < (s.v[1437] * 1e-8));s.store_scalar(1482, if s.b[1482] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && s.b[1481]) && s.b[1482]) {s.store_add_scaled_inputs_product_mixed_aaia(1439, A::offset(s.ad_value(1436), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1434), 0.5, s.ad_value(1436), 1.0), 1.0, 1406, A::offset(s.ad_value(1405), (-2.0)), 9.0);}
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && s.b[1481]) && (!s.b[1482])) {s.store_sqrt_add(1438, 1434, 1437);s.store_add_scaled_offset_product_rhs_mixed_aii(1439, A::offset(s.ad_value(1438), ((-7.0) * 1.414213562373095)), 1.0, 1406, 1405, (-2.0), 9.0);}
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && s.b[1481]) {s.store_powf(1440, 1439, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(1441, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1406), 12.0)), 1.0, 1440, 2.0, 1440, 1440, 1.414213562373095);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_55(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && s.b[1481]) {s.store_div(1442, 1441, 1440);s.store_add_scaled_product_indices(1443, 1419, (-1.0), 1442, 227, 1.0);s.store_add(1399, 1443, 1419);s.store_div(1400, 1399, 1435);s.store_sqrt_square_offset(1401, 1400, 1.0);s.store_sub_div_lhs_indices(1444, 1399, 1401, 1419);s.store_sub(1400, 1430, 1444);s.store_mul(459, 1407, 1400);s.copy_ad(458, 459);}
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) {s.store_scalar(1442, 3.0);s.store_sub_div_lhs_indices(1445, 1442, 225, 1419);s.store_exp_neg_input(1406, 1442);s.store_offset_div_scaled_inputs2_mixed_aia(1405, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1430), s.ad_value(1419))), (-1.0)), 4.0, 1406, 4.0, A::mul(s.ad_value(1429), s.ad_value(226)), 1.0, 1.0);}
        s.b[1483] = (s.v[1405] < (10.0 * 2.220446049250313e-16));s.store_scalar(1483, if s.b[1483] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1483]) {s.store_scalar(1405, (10.0 * 2.220446049250313e-16));}
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) {s.store_add_product3_rhs_mixed_iia(1445, 1430, 1429, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1405))), 1.0 / (2.0));s.store_mul_add_rhs(1442, 225, 1445, 1419);s.store_exp_neg_input(1406, 1442);s.store_offset_div_scaled_inputs2_mixed_aia(1405, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1430), s.ad_value(1419))), (-1.0)), 4.0, 1406, 4.0, A::mul(s.ad_value(1429), s.ad_value(226)), 1.0, 1.0);}
        s.b[1484] = (s.v[1405] < (10.0 * 2.220446049250313e-16));s.store_scalar(1484, if s.b[1484] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1484]) {s.store_scalar(1405, (10.0 * 2.220446049250313e-16));}
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) {s.store_add_product3_rhs_mixed_iia(1445, 1430, 1429, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1405))), 1.0 / (2.0));s.store_mul_add_rhs(1442, 225, 1445, 1419);}
        s.b[1485] = (s.v[1442] < 3.0);s.store_scalar(1485, if s.b[1485] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1485]) {s.store_scalar(1446, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));s.store_scalar(1447, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));s.store_offset_div_from_scalar_ad(1448, 1.0, A::mul(s.ad_value(225), s.ad_value(1428)), (1.0 / 1.414213562373095));s.store_div_scaled_inputs2_indices(1449, 1430, -1.0, 1419, -1.0, 1428, 1.0);s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(1450, A::div_scaled_product(A::square(s.ad_value(1447)), s.ad_value(1447), 1.0, A::mul3_scaled_output(s.ad_value(1446), s.ad_value(1446), s.ad_value(1446), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1447), s.ad_value(1448), 1.0, s.ad_value(1446), s.ad_value(1446), 6.0), (-1.0), 1449, 1.0, 1446, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(1451, A::add_scaled_square_product(s.ad_value(1447), (-1.0), s.ad_value(1446), s.ad_value(1448), 3.0), 1.0, 1446, 1446, 9.0);s.store_sqrt_add_scaled_square_cube_product(1402, 1450, 1.0, 1451, 1.0);s.store_powf_ad(1452, A::sub(s.ad_value(1402), s.ad_value(1450)), 0.3333333333333333);s.store_neg_powf_add_input(1453, 1450, 1402, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(1405, 1452, 1.0, 1453, 1.0, 1447, 1.0, 1446, 3.0, -1.0);s.store_add_scaled_product_indices(1445, 1419, (-1.0), 1405, 227, 1.0);s.store_mul_add_rhs(1442, 225, 1445, 1419);}
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) {s.store_offset_add(1454, 1430, 1419, 0.1);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_56(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) {s.store_offset_exp_ad(1461, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1419), -1.0), 1e-50);s.store_div(1398, 230, 521);s.store_square(1455, 1398);s.store_mul(1456, 1455, 1461);s.store_mul(1398, 226, 1429);s.store_mul(1457, 225, 1454);s.store_add_scaled_inputs_product_mixed_aaii(1458, A::ln(A::add_scaled_square_product(s.ad_value(1457), 1.0, s.ad_value(1456), s.ad_value(1398), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1455), s.ad_value(1398))), (-1.0), 225, 1419, 1.0);s.store_offset_sub(44, 1457, 1458, (-1.0));s.store_scale(45, 1457, 4.0);}
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1399, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1400, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1458, 1457, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub(1457, 1457, 1458);s.store_add_scaled_inputs(1457, 1457, 1.0, 225, 0.1);s.store_add_scaled_inputs_product_mixed_aaii(1459, A::ln(A::add_scaled_square_product(s.ad_value(1457), 1.0, s.ad_value(1456), s.ad_value(1398), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1455), s.ad_value(1398))), (-1.0), 225, 1419, 1.0);s.copy_ad(1460, 1442);s.store_offset_sub(44, 1459, 1460, (-(0.0008 * 75.0)));s.store_scale(45, 1459, (4.0 * (0.0008 * 75.0)));}
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1399, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1400, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1442, 1459, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub_div_lhs_indices(1444, 1442, 225, 1419);s.store_add_offset_lhs_mixed_ia(1399, 1442, (-1.0), A::exp_scaled_input(s.ad_value(1442), -1.0));}
        s.b[1486] = (s.v[1399] < (10.0 * 2.220446049250313e-16));s.store_scalar(1486, if s.b[1486] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1486]) {s.store_scalar(1399, (10.0 * 2.220446049250313e-16));}
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) {s.store_sqrt(1400, 1399);s.store_mul(458, 1427, 1400);s.store_mul_sub_rhs(459, 1407, 1430, 1444);}
        s.b[1487] = (p[42] == 1.0);s.store_scalar(1487, if s.b[1487] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) {s.store_exp_ad(1461, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1419), -1.0));s.store_div(1398, 230, 521);s.store_square(1455, 1398);s.store_mul(1470, 1455, 1461);s.store_scalar(1415, 0.0);s.store_scalar(167, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_57(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t5: usize = 0;
        while {
            let t2: f64 = (2.0 * 20.0);let t3: f64 = (t2 + 1.0);let t4: f64 = if (((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (s.v[167] <= t3)) { 1.0 } else { 0.0 };
            t4 != 0.0
        } {
            t5 += 1;
            if t5 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t5, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) {s.store_scalar(1466, 0.0);s.store_mul_add_rhs(1442, 225, 1444, 1419);}
            s.b[1488] = (s.v[1442] < 5.0);s.store_scalar(1488, if s.b[1488] { 1.0 } else { 0.0 });
            if (((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && s.b[1488]) {s.store_mul3_ad_middle(1462, A::square(s.ad_value(1442)), 1442, A::offset(A::mul(s.ad_value(1442), A::scale_offset(s.ad_value(1442), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(1463, A::square(s.ad_value(1442)), A::mul(s.ad_value(1442), A::scale_offset(s.ad_value(1442), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul3_lhs(1464, 1470, 1462, 1462);s.store_mul_product3_indices(1465, 1463, 1470, 225, 1462, 2.0);s.store_mul_scale_offset_mixed_ia(1466, 1442, A::mul_offset_rhs(s.ad_value(1442), A::mul_offset_rhs(s.ad_value(1442), A::mul(s.ad_value(1442), A::scale_offset(s.ad_value(1442), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(1467, 1442, A::mul_offset_rhs(s.ad_value(1442), A::mul(s.ad_value(1442), A::scale_offset(s.ad_value(1442), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_offset_ad(1468, A::add(A::square(s.ad_value(1466)), s.ad_value(1464)), 1e-50);s.store_div_scaled_inputs2_mixed_aii(1469, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1467), s.ad_value(1466), 2.0), 1.0, 1465, 1.0, 1468, 2.0);}
            s.b[1489] = (s.v[1442] < 80.0);s.store_scalar(1489, if s.b[1489] { 1.0 } else { 0.0 });
            if ((((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1488])) && s.b[1489]) {s.store_exp(243, 1442);s.store_mul_scale_offset_indices(1464, 1470, 243, 1.0, (-1.0));s.store_mul3_lhs(1465, 1470, 225, 243);}
            if ((((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1488])) && (!s.b[1489])) {s.store_exp_mul(1471, 225, 1444);s.store_mul_sub_rhs(1464, 1455, 1471, 1461);s.store_mul3_lhs(1465, 1455, 225, 1471);}
            if (((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1488])) {s.store_sqrt_add_ad(1468, A::offset(s.ad_value(1442), (-1.0)), s.ad_value(1464));s.store_scale_ad(1469, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1465), 1.0, s.ad_value(1468), 1.0), 0.5);}
            if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) {s.store_add_scaled_inputs_product_indices(1472, 1430, 1.0, 1444, (-1.0), 1428, 1468, (-1.0));s.store_sub_from_scalar_scaled_mul(1473, (-1.0), 1428, 1469, 1.0);}
            s.b[1490] = (s.v[1415] == 1.0);s.store_scalar(1490, if s.b[1490] { 1.0 } else { 0.0 });
            if (((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && s.b[1490]) {s.store_scalar(167, ((2.0 * 20.0) + 1.0));}
            if (((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1490])) {s.store_div_scaled_inputs_indices(494, 1472, -1.0, 1473, 1.0);}
            if (((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1490])) {
                s.store_scaled_offset_ad(1474, {
                    if (1.0 >= ((s.v[1444]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1444))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1491] = (((s.v[494]) as f64).abs() > s.v[1474]);s.store_scalar(1491, if s.b[1491] { 1.0 } else { 0.0 });
            if ((((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1490])) && s.b[1491]) {s.store_scale(494, 1474, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1490])) {s.store_add(1444, 1444, 494);}
            s.b[1492] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1472]) as f64).abs() <= 1e-8));s.store_scalar(1492, if s.b[1492] { 1.0 } else { 0.0 });
            if ((((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1490])) && s.b[1492]) {s.store_scalar(1415, 1.0);}
            if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) {s.store_primal_offset(167, 167, 1.0);}
        }
        s.b[1494] = (s.v[1442] < 5.0);s.store_scalar(1494, if s.b[1494] { 1.0 } else { 0.0 });
        if (((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && s.b[1494]) {s.store_offset_square(1475, 1466, (10.0 * 2.220446049250313e-16));s.store_offset(1476, 1466, (10.0 * 2.220446049250313e-16));}
        if (((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1494])) {s.store_offset(1475, 1442, (-1.0));s.store_sqrt(1476, 1475);}
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) {s.store_mul(458, 1427, 1476);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_58(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) {s.store_div_from_scalar_add_ad(1399, 1.0, s.ad_value(1468), s.ad_value(1476));s.store_mul3_lhs(460, 1427, 1464, 1399);s.store_add(459, 458, 460);}
        if ((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) {s.store_sub(460, 459, 458);}
        s.b[1496] = (1.0 == 1.0);s.store_scalar(1496, if s.b[1496] { 1.0 } else { 0.0 });s.b[1497] = (1.0 == 2.0);s.store_scalar(1497, if s.b[1497] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && s.b[1496]) && (s.v[1409] != 0.0)) {s.store_mul_scale_offset_indices(463, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(465, 460, 522, -1.0, 0.0);}
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && s.b[1496]) && (s.v[1410] != 0.0)) {s.store_mul_scale_offset_indices(464, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(466, 460, 522, -1.0, 0.0);}
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (s.b[1497] && (!s.b[1496]))) && (s.v[1409] != 0.0)) {s.store_mul_scale_offset_indices(467, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(469, 460, 522, -1.0, 0.0);}
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (s.b[1497] && (!s.b[1496]))) && (s.v[1410] != 0.0)) {s.store_mul_scale_offset_indices(468, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(470, 460, 522, -1.0, 0.0);}
        if ((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) {s.store_scalar(1409, ((1.0 - 1.0) / 2.0));s.store_scalar(1410, ((1.0 + 1.0) / 2.0));s.store_add_scaled_products_mixed_iiia(1420, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);s.store_add_scaled_products_indices(1421, 461, 157, 1.0, 462, 157, -1.0);s.store_add_scaled_products_mixed_iiia(1422, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_add_scaled_products_mixed_iiia(1423, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_sub(1424, 1421, 1420);s.store_neg(1425, 1420);s.store_primal_add_scaled_products_indices(1411, 1409, 461, 1.0, 1410, 462, 1.0);s.store_primal_add_scaled_products_indices(1412, 1409, 462, 1.0, 1410, 461, 1.0);s.store_add_scaled_products_indices(1426, 1411, 1422, 1.0, 1412, 1423, 1.0);s.store_offset_ad(1418, A::add_scaled_products(s.ad_value(1411), s.ad_value(1425), 1.0, s.ad_value(1412), s.ad_value(1424), 1.0), (10.0 * 2.220446049250313e-16));s.store_neg(1398, 1418);}
        s.b[1498] = (s.v[1398] > s.v[141]);s.store_scalar(1498, if s.b[1498] { 1.0 } else { 0.0 });
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && s.b[1498]) {s.store_sub(1399, 1398, 141);s.store_sub(1400, 140, 141);s.store_div(44, 1399, 1400);s.store_square(45, 44);s.store_mul(46, 45, 44);s.store_square(47, 45);s.store_div_from_scalar_ad(1406, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));s.store_mul_scale_offset_indices(1406, 1400, 1406, -1.0, 1.0);s.store_add(1403, 141, 1406);}
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1498])) {s.copy_ad(1403, 1398);}
        if ((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) {s.store_offset_scaled(1419, 1403, -1.0, (-1e-12));s.store_mul(1428, 1427, 1408);s.store_square(1429, 1428);s.store_sub(1430, 1426, 523);s.store_div(1398, 521, 230);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1431, 2.0, 225, A::ln(s.ad_value(1398)));s.store_neg(1432, 1419);}
        s.b[1499] = (s.v[1430] < s.v[1432]);s.store_scalar(1499, if s.b[1499] { 1.0 } else { 0.0 });
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && s.b[1499]) {s.store_div_scalar_by_product_indices(1399, 1.0, 225, 1427, 1.0);s.store_mul(1406, 1399, 1407);s.store_offset_scaled(1433, 1406, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(1434, 1433, 1433, 8.0, 0.0, 1433);s.store_sub(1435, 237, 1431);s.store_mul_add_rhs(1405, 225, 1430, 1419);s.store_sub_from_scalar_scaled_mul_mixed_ia(1436, (7.0 * 1.414213562373095), 1406, A::offset(s.ad_value(1405), (-2.0)), 9.0);s.store_square(1437, 1436);}
        s.b[1500] = (s.v[1434] < (s.v[1437] * 1e-8));s.store_scalar(1500, if s.b[1500] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && s.b[1499]) && s.b[1500]) {s.store_add_scaled_inputs_product_mixed_aaia(1439, A::offset(s.ad_value(1436), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1434), 0.5, s.ad_value(1436), 1.0), 1.0, 1406, A::offset(s.ad_value(1405), (-2.0)), 9.0);}
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && s.b[1499]) && (!s.b[1500])) {s.store_sqrt_add(1438, 1434, 1437);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_59(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && s.b[1499]) && (!s.b[1500])) {s.store_add_scaled_offset_product_rhs_mixed_aii(1439, A::offset(s.ad_value(1438), ((-7.0) * 1.414213562373095)), 1.0, 1406, 1405, (-2.0), 9.0);}
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && s.b[1499]) {s.store_powf(1440, 1439, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(1441, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1406), 12.0)), 1.0, 1440, 2.0, 1440, 1440, 1.414213562373095);s.store_div(1442, 1441, 1440);s.store_add_scaled_product_indices(1443, 1419, (-1.0), 1442, 227, 1.0);s.store_add(1399, 1443, 1419);s.store_div(1400, 1399, 1435);s.store_sqrt_square_offset(1401, 1400, 1.0);s.store_sub_div_lhs_indices(1444, 1399, 1401, 1419);s.store_sub(1400, 1430, 1444);s.store_mul(459, 1407, 1400);s.copy_ad(458, 459);}
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) {s.store_scalar(1442, 3.0);s.store_sub_div_lhs_indices(1445, 1442, 225, 1419);s.store_exp_neg_input(1406, 1442);s.store_offset_div_scaled_inputs2_mixed_aia(1405, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1430), s.ad_value(1419))), (-1.0)), 4.0, 1406, 4.0, A::mul(s.ad_value(1429), s.ad_value(226)), 1.0, 1.0);}
        s.b[1501] = (s.v[1405] < (10.0 * 2.220446049250313e-16));s.store_scalar(1501, if s.b[1501] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1501]) {s.store_scalar(1405, (10.0 * 2.220446049250313e-16));}
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) {s.store_add_product3_rhs_mixed_iia(1445, 1430, 1429, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1405))), 1.0 / (2.0));s.store_mul_add_rhs(1442, 225, 1445, 1419);s.store_exp_neg_input(1406, 1442);s.store_offset_div_scaled_inputs2_mixed_aia(1405, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1430), s.ad_value(1419))), (-1.0)), 4.0, 1406, 4.0, A::mul(s.ad_value(1429), s.ad_value(226)), 1.0, 1.0);}
        s.b[1502] = (s.v[1405] < (10.0 * 2.220446049250313e-16));s.store_scalar(1502, if s.b[1502] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1502]) {s.store_scalar(1405, (10.0 * 2.220446049250313e-16));}
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) {s.store_add_product3_rhs_mixed_iia(1445, 1430, 1429, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1405))), 1.0 / (2.0));s.store_mul_add_rhs(1442, 225, 1445, 1419);}
        s.b[1503] = (s.v[1442] < 3.0);s.store_scalar(1503, if s.b[1503] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1503]) {s.store_scalar(1446, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));s.store_scalar(1447, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));s.store_offset_div_from_scalar_ad(1448, 1.0, A::mul(s.ad_value(225), s.ad_value(1428)), (1.0 / 1.414213562373095));s.store_div_scaled_inputs2_indices(1449, 1430, -1.0, 1419, -1.0, 1428, 1.0);s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(1450, A::div_scaled_product(A::square(s.ad_value(1447)), s.ad_value(1447), 1.0, A::mul3_scaled_output(s.ad_value(1446), s.ad_value(1446), s.ad_value(1446), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1447), s.ad_value(1448), 1.0, s.ad_value(1446), s.ad_value(1446), 6.0), (-1.0), 1449, 1.0, 1446, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(1451, A::add_scaled_square_product(s.ad_value(1447), (-1.0), s.ad_value(1446), s.ad_value(1448), 3.0), 1.0, 1446, 1446, 9.0);s.store_sqrt_add_scaled_square_cube_product(1402, 1450, 1.0, 1451, 1.0);s.store_powf_ad(1452, A::sub(s.ad_value(1402), s.ad_value(1450)), 0.3333333333333333);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_60(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1503]) {s.store_neg_powf_add_input(1453, 1450, 1402, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(1405, 1452, 1.0, 1453, 1.0, 1447, 1.0, 1446, 3.0, -1.0);s.store_add_scaled_product_indices(1445, 1419, (-1.0), 1405, 227, 1.0);s.store_mul_add_rhs(1442, 225, 1445, 1419);}
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) {s.store_offset_add(1454, 1430, 1419, 0.1);s.store_offset_exp_ad(1461, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1419), -1.0), 1e-50);s.store_div(1398, 230, 521);s.store_square(1455, 1398);s.store_mul(1456, 1455, 1461);s.store_mul(1398, 226, 1429);s.store_mul(1457, 225, 1454);s.store_add_scaled_inputs_product_mixed_aaii(1458, A::ln(A::add_scaled_square_product(s.ad_value(1457), 1.0, s.ad_value(1456), s.ad_value(1398), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1455), s.ad_value(1398))), (-1.0), 225, 1419, 1.0);s.store_offset_sub(44, 1457, 1458, (-1.0));s.store_scale(45, 1457, 4.0);}
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1399, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1400, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1458, 1457, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub(1457, 1457, 1458);s.store_add_scaled_inputs(1457, 1457, 1.0, 225, 0.1);s.store_add_scaled_inputs_product_mixed_aaii(1459, A::ln(A::add_scaled_square_product(s.ad_value(1457), 1.0, s.ad_value(1456), s.ad_value(1398), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1455), s.ad_value(1398))), (-1.0), 225, 1419, 1.0);s.copy_ad(1460, 1442);s.store_offset_sub(44, 1459, 1460, (-(0.0008 * 75.0)));s.store_scale(45, 1459, (4.0 * (0.0008 * 75.0)));}
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1399, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1400, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1442, 1459, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub_div_lhs_indices(1444, 1442, 225, 1419);s.store_add_offset_lhs_mixed_ia(1399, 1442, (-1.0), A::exp_scaled_input(s.ad_value(1442), -1.0));}
        s.b[1504] = (s.v[1399] < (10.0 * 2.220446049250313e-16));s.store_scalar(1504, if s.b[1504] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1504]) {s.store_scalar(1399, (10.0 * 2.220446049250313e-16));}
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) {s.store_sqrt(1400, 1399);s.store_mul(458, 1427, 1400);s.store_mul_sub_rhs(459, 1407, 1430, 1444);}
        s.b[1505] = (p[42] == 1.0);s.store_scalar(1505, if s.b[1505] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) {s.store_exp_ad(1461, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1419), -1.0));s.store_div(1398, 230, 521);s.store_square(1455, 1398);s.store_mul(1470, 1455, 1461);s.store_scalar(1415, 0.0);s.store_scalar(167, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_61(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t9: usize = 0;
        while {
            let t6: f64 = (2.0 * 20.0);let t7: f64 = (t6 + 1.0);let t8: f64 = if (((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (s.v[167] <= t7)) { 1.0 } else { 0.0 };
            t8 != 0.0
        } {
            t9 += 1;
            if t9 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t9, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) {s.store_scalar(1466, 0.0);s.store_mul_add_rhs(1442, 225, 1444, 1419);}
            s.b[1506] = (s.v[1442] < 5.0);s.store_scalar(1506, if s.b[1506] { 1.0 } else { 0.0 });
            if (((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && s.b[1506]) {s.store_mul3_ad_middle(1462, A::square(s.ad_value(1442)), 1442, A::offset(A::mul(s.ad_value(1442), A::scale_offset(s.ad_value(1442), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(1463, A::square(s.ad_value(1442)), A::mul(s.ad_value(1442), A::scale_offset(s.ad_value(1442), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul3_lhs(1464, 1470, 1462, 1462);s.store_mul_product3_indices(1465, 1463, 1470, 225, 1462, 2.0);s.store_mul_scale_offset_mixed_ia(1466, 1442, A::mul_offset_rhs(s.ad_value(1442), A::mul_offset_rhs(s.ad_value(1442), A::mul(s.ad_value(1442), A::scale_offset(s.ad_value(1442), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(1467, 1442, A::mul_offset_rhs(s.ad_value(1442), A::mul(s.ad_value(1442), A::scale_offset(s.ad_value(1442), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_offset_ad(1468, A::add(A::square(s.ad_value(1466)), s.ad_value(1464)), 1e-50);s.store_div_scaled_inputs2_mixed_aii(1469, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1467), s.ad_value(1466), 2.0), 1.0, 1465, 1.0, 1468, 2.0);}
            s.b[1507] = (s.v[1442] < 80.0);s.store_scalar(1507, if s.b[1507] { 1.0 } else { 0.0 });
            if ((((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1506])) && s.b[1507]) {s.store_exp(243, 1442);s.store_mul_scale_offset_indices(1464, 1470, 243, 1.0, (-1.0));s.store_mul3_lhs(1465, 1470, 225, 243);}
            if ((((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1506])) && (!s.b[1507])) {s.store_exp_mul(1471, 225, 1444);s.store_mul_sub_rhs(1464, 1455, 1471, 1461);s.store_mul3_lhs(1465, 1455, 225, 1471);}
            if (((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1506])) {s.store_sqrt_add_ad(1468, A::offset(s.ad_value(1442), (-1.0)), s.ad_value(1464));s.store_scale_ad(1469, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1465), 1.0, s.ad_value(1468), 1.0), 0.5);}
            if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) {s.store_add_scaled_inputs_product_indices(1472, 1430, 1.0, 1444, (-1.0), 1428, 1468, (-1.0));s.store_sub_from_scalar_scaled_mul(1473, (-1.0), 1428, 1469, 1.0);}
            s.b[1508] = (s.v[1415] == 1.0);s.store_scalar(1508, if s.b[1508] { 1.0 } else { 0.0 });
            if (((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && s.b[1508]) {s.store_scalar(167, ((2.0 * 20.0) + 1.0));}
            if (((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1508])) {s.store_div_scaled_inputs_indices(494, 1472, -1.0, 1473, 1.0);}
            if (((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1508])) {
                s.store_scaled_offset_ad(1474, {
                    if (1.0 >= ((s.v[1444]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1444))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1509] = (((s.v[494]) as f64).abs() > s.v[1474]);s.store_scalar(1509, if s.b[1509] { 1.0 } else { 0.0 });
            if ((((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1508])) && s.b[1509]) {s.store_scale(494, 1474, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1508])) {s.store_add(1444, 1444, 494);}
            s.b[1510] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1472]) as f64).abs() <= 1e-8));s.store_scalar(1510, if s.b[1510] { 1.0 } else { 0.0 });
            if ((((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1508])) && s.b[1510]) {s.store_scalar(1415, 1.0);}
            if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) {s.store_primal_offset(167, 167, 1.0);}
        }
        s.b[1512] = (s.v[1442] < 5.0);s.store_scalar(1512, if s.b[1512] { 1.0 } else { 0.0 });
        if (((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && s.b[1512]) {s.store_offset_square(1475, 1466, (10.0 * 2.220446049250313e-16));s.store_offset(1476, 1466, (10.0 * 2.220446049250313e-16));}
        if (((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1512])) {s.store_offset(1475, 1442, (-1.0));s.store_sqrt(1476, 1475);}
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) {s.store_mul(458, 1427, 1476);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_62(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) {s.store_div_from_scalar_add_ad(1399, 1.0, s.ad_value(1468), s.ad_value(1476));s.store_mul3_lhs(460, 1427, 1464, 1399);s.store_add(459, 458, 460);}
        if ((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) {s.store_sub(460, 459, 458);}
        s.b[1514] = (1.0 == 1.0);s.store_scalar(1514, if s.b[1514] { 1.0 } else { 0.0 });s.b[1515] = (1.0 == 2.0);s.store_scalar(1515, if s.b[1515] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && s.b[1514]) && (s.v[1409] != 0.0)) {s.store_mul_scale_offset_indices(463, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(465, 460, 522, -1.0, 0.0);}
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && s.b[1514]) && (s.v[1410] != 0.0)) {s.store_mul_scale_offset_indices(464, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(466, 460, 522, -1.0, 0.0);}
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (s.b[1515] && (!s.b[1514]))) && (s.v[1409] != 0.0)) {s.store_mul_scale_offset_indices(467, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(469, 460, 522, -1.0, 0.0);}
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (s.b[1515] && (!s.b[1514]))) && (s.v[1410] != 0.0)) {s.store_mul_scale_offset_indices(468, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(470, 460, 522, -1.0, 0.0);}
        s.store_scalar(317, p[189]);s.b[1518] = (s.v[145] != 0.0);s.store_scalar(1518, if s.b[1518] { 1.0 } else { 0.0 });
        if s.b[1518] {s.store_add(1517, 157, 161);s.store_add_scaled_inputs(314, 1517, s.v[317], 162, (1.0 - s.v[317]));}
        s.b[1519] = (p[64] != 0.0);s.store_scalar(1519, if s.b[1519] { 1.0 } else { 0.0 });
        if (s.b[1518] && s.b[1519]) {s.store_scalar(315, 0.0);}
        s.b[1520] = (s.v[314] > ((s.v[161] + s.v[157]) - (10.0 * 2.220446049250313e-16)));s.store_scalar(1520, if s.b[1520] { 1.0 } else { 0.0 });
        if (s.b[1518] && s.b[1520]) {s.store_offset_add(314, 161, 157, (-(10.0 * 2.220446049250313e-16)));}
        s.b[1521] = (p[64] != 0.0);s.store_scalar(1521, if s.b[1521] { 1.0 } else { 0.0 });s.b[1522] = (s.v[246] < 1e-15);s.store_scalar(1522, if s.b[1522] { 1.0 } else { 0.0 });
        if (((!s.b[1518]) && s.b[1521]) && s.b[1522]) {s.store_scalar(315, 0.0);}
        if (((!s.b[1518]) && s.b[1521]) && (!s.b[1522])) {s.store_scale(1516, 227, 1.0 / (s.v[97]));s.store_div_from_scalar(1517, 1.0, 244);s.store_mul3_lhs(315, 246, 1516, 1517);}
        s.store_scalar(1534, s.v[91]);s.store_scalar(1535, (1.0 / s.v[1534]));s.store_scalar(1555, 0.0);s.store_scalar(1595, 0.0);s.store_scalar(1593, 0.0);s.store_scalar(1597, 0.0);s.b[1606] = ((p[29] >= 1.0) && (p[188] > 0.0));s.store_scalar(1606, if s.b[1606] { 1.0 } else { 0.0 });
        if ((p[24] != 0.0) && s.b[1606]) {s.store_scalar(1537, p[171]);s.store_scalar(1538, p[172]);s.copy_ad(1539, 158);s.store_scalar(1536, p[188]);}
        s.b[1607] = ((s.v[69] == 0.0) && (p[188] > 0.0));s.store_scalar(1607, if s.b[1607] { 1.0 } else { 0.0 });
        if (((p[24] != 0.0) && s.b[1606]) && s.b[1607]) {
            if (p[43] == 1.0) {
                s.store_scale(1524, 287, s.v[1534]);
            } else {
                s.store_scale(1524, 108, s.v[1534]);
            }
        }
        if (((p[24] != 0.0) && s.b[1606]) && s.b[1607]) {s.store_mul_ad_product_rhs_mixed_ia(1527, 1537, 1524, A::add(s.ad_value(1538), s.ad_value(1539)));s.store_mul(1528, 1536, 1524);s.copy_ad(1532, 161);s.store_sub_from_scalar(1529, 1.2, 1532);s.store_add_scaled_products_indices(267, 158, 1528, 1.0, 1529, 1527, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(1527, 1537, 1524, A::add_scaled_inputs3(s.ad_value(1538), 1.0, s.ad_value(1539), 1.0, s.ad_value(157), -1.0));s.store_sub(1532, 162, 157);s.store_sub_from_scalar(1529, 1.2, 1532);s.store_add_scaled_products_mixed_aiii(268, A::sub(s.ad_value(158), s.ad_value(157)), 1528, 1.0, 1527, 1529, (-1.0));}
        if (((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) {s.store_mul_sqrt_mixed_ia(1556, 238, A::div_from_scalar(s.v[69], s.ad_value(536)));s.store_scalar(1540, ((1.0 - -1.0) / 2.0));s.store_scalar(1541, ((1.0 + -1.0) / 2.0));}
        s.b[1608] = (p[43] == 1.0);s.store_scalar(1608, if s.b[1608] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1608]) {s.store_add_scaled_products_mixed_iiia(1550, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);s.store_add_scaled_products_indices(1551, 461, 157, 1.0, 462, 157, -1.0);s.store_add_scaled_products_mixed_iiia(1552, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_sub(1553, 1551, 1550);s.store_sub(1555, 1552, 1550);s.store_neg(1554, 1550);s.store_primal_add_scaled_products_indices(1542, 1540, 461, 1.0, 1541, 462, 1.0);s.store_primal_add_scaled_products_indices(1543, 1540, 462, 1.0, 1541, 461, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_63(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1608]) {s.store_offset_ad(1548, A::add_scaled_products(s.ad_value(1542), s.ad_value(1554), 1.0, s.ad_value(1543), s.ad_value(1553), 1.0), (10.0 * 2.220446049250313e-16));}
        if ((((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1608])) {s.store_primal_add_scaled_products_indices(1542, 1540, 461, 1.0, 1541, 462, 1.0);s.store_primal_add_scaled_products_indices(1543, 1540, 462, 1.0, 1541, 461, 1.0);}
        if (((((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1608])) && (s.v[1540] != 0.0)) {s.store_add_scaled_products_mixed_iiia(1555, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);}
        if (((((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1608])) && (s.v[1541] != 0.0)) {s.store_add_scaled_products_mixed_iiia(1555, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);}
        if ((((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1608])) {s.store_scalar(1548, 0.0);}
        if (((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) {s.store_neg(1523, 1548);}
        s.b[1609] = (s.v[1523] > s.v[141]);s.store_scalar(1609, if s.b[1609] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1609]) {s.store_sub(1524, 1523, 141);s.store_sub(1525, 140, 141);s.store_div(44, 1524, 1525);s.store_square(45, 44);s.store_mul(46, 45, 44);s.store_square(47, 45);s.store_div_from_scalar_ad(1533, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));s.store_mul_scale_offset_indices(1533, 1525, 1533, -1.0, 1.0);s.store_add(1530, 141, 1533);}
        if ((((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1609])) {s.copy_ad(1530, 1523);}
        if (((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) {s.store_offset_scaled(1549, 1530, -1.0, (-1e-12));s.store_scale(1557, 1556, s.v[1535]);s.store_square(1558, 1557);s.store_sub_from_scalar(1559, s.v[82], 1555);s.store_div_from_scalar(1523, s.v[69], 230);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1560, 2.0, 225, A::ln(s.ad_value(1523)));s.store_neg(1561, 1549);}
        s.b[1610] = (s.v[1559] < s.v[1561]);s.store_scalar(1610, if s.b[1610] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1610]) {s.store_div_scalar_by_product_indices(1524, 1.0, 225, 1556, 1.0);s.store_scale(1533, 1524, s.v[1534]);s.store_offset_scaled(1562, 1533, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(1563, 1562, 1562, 8.0, 0.0, 1562);s.store_sub(1564, 237, 1560);s.store_mul_add_rhs(1532, 225, 1559, 1549);s.store_sub_from_scalar_scaled_mul_mixed_ia(1565, (7.0 * 1.414213562373095), 1533, A::offset(s.ad_value(1532), (-2.0)), 9.0);s.store_square(1566, 1565);}
        s.b[1611] = (s.v[1563] < (s.v[1566] * 1e-8));s.store_scalar(1611, if s.b[1611] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1610]) && s.b[1611]) {s.store_add_scaled_inputs_product_mixed_aaia(1568, A::offset(s.ad_value(1565), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1563), 0.5, s.ad_value(1565), 1.0), 1.0, 1533, A::offset(s.ad_value(1532), (-2.0)), 9.0);}
        if (((((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1610]) && (!s.b[1611])) {s.store_sqrt_add(1567, 1563, 1566);s.store_add_scaled_offset_product_rhs_mixed_aii(1568, A::offset(s.ad_value(1567), ((-7.0) * 1.414213562373095)), 1.0, 1533, 1532, (-2.0), 9.0);}
        if ((((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1610]) {s.store_powf(1569, 1568, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(1570, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1533), 12.0)), 1.0, 1569, 2.0, 1569, 1569, 1.414213562373095);s.store_div(1571, 1570, 1569);s.store_add_scaled_product_indices(1572, 1549, (-1.0), 1571, 227, 1.0);s.store_add(1524, 1572, 1549);s.store_div(1525, 1524, 1564);s.store_sqrt_square_offset(1526, 1525, 1.0);s.store_sub_div_lhs_indices(1573, 1524, 1526, 1549);s.store_sub(1525, 1559, 1573);s.store_scale(459, 1525, s.v[1534]);s.copy_ad(458, 459);}
        if ((((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) {s.store_scalar(1571, 3.0);s.store_sub_div_lhs_indices(1574, 1571, 225, 1549);s.store_exp_neg_input(1533, 1571);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_64(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) {s.store_offset_div_scaled_inputs2_mixed_aia(1532, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1559), s.ad_value(1549))), (-1.0)), 4.0, 1533, 4.0, A::mul(s.ad_value(1558), s.ad_value(226)), 1.0, 1.0);}
        s.b[1612] = (s.v[1532] < (10.0 * 2.220446049250313e-16));s.store_scalar(1612, if s.b[1612] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1612]) {s.store_scalar(1532, (10.0 * 2.220446049250313e-16));}
        if ((((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) {s.store_add_product3_rhs_mixed_iia(1574, 1559, 1558, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1532))), 1.0 / (2.0));s.store_mul_add_rhs(1571, 225, 1574, 1549);s.store_exp_neg_input(1533, 1571);s.store_offset_div_scaled_inputs2_mixed_aia(1532, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1559), s.ad_value(1549))), (-1.0)), 4.0, 1533, 4.0, A::mul(s.ad_value(1558), s.ad_value(226)), 1.0, 1.0);}
        s.b[1613] = (s.v[1532] < (10.0 * 2.220446049250313e-16));s.store_scalar(1613, if s.b[1613] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1613]) {s.store_scalar(1532, (10.0 * 2.220446049250313e-16));}
        if ((((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) {s.store_add_product3_rhs_mixed_iia(1574, 1559, 1558, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1532))), 1.0 / (2.0));s.store_mul_add_rhs(1571, 225, 1574, 1549);}
        s.b[1614] = (s.v[1571] < 3.0);s.store_scalar(1614, if s.b[1614] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1614]) {s.store_scalar(1575, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));s.store_scalar(1576, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));s.store_offset_div_from_scalar_ad(1577, 1.0, A::mul(s.ad_value(225), s.ad_value(1557)), (1.0 / 1.414213562373095));s.store_div_scaled_inputs2_indices(1578, 1559, -1.0, 1549, -1.0, 1557, 1.0);s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(1579, A::div_scaled_product(A::square(s.ad_value(1576)), s.ad_value(1576), 1.0, A::mul3_scaled_output(s.ad_value(1575), s.ad_value(1575), s.ad_value(1575), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1576), s.ad_value(1577), 1.0, s.ad_value(1575), s.ad_value(1575), 6.0), (-1.0), 1578, 1.0, 1575, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(1580, A::add_scaled_square_product(s.ad_value(1576), (-1.0), s.ad_value(1575), s.ad_value(1577), 3.0), 1.0, 1575, 1575, 9.0);s.store_sqrt_add_scaled_square_cube_product(1528, 1579, 1.0, 1580, 1.0);s.store_powf_ad(1581, A::sub(s.ad_value(1528), s.ad_value(1579)), 0.3333333333333333);s.store_neg_powf_add_input(1582, 1579, 1528, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(1532, 1581, 1.0, 1582, 1.0, 1576, 1.0, 1575, 3.0, -1.0);s.store_add_scaled_product_indices(1574, 1549, (-1.0), 1532, 227, 1.0);s.store_mul_add_rhs(1571, 225, 1574, 1549);}
        s.b[1615] = (p[41] > 0.0);s.store_scalar(1615, if s.b[1615] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1615]) {s.store_offset_add(1583, 1559, 1549, 0.1);s.store_offset_exp_ad(1590, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1549), -1.0), 1e-50);s.store_scale(1523, 230, 1.0 / (s.v[69]));s.store_square(1584, 1523);s.store_mul(1585, 1584, 1590);s.store_mul(1523, 226, 1558);s.store_mul(1586, 225, 1583);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_65(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1615]) {s.store_add_scaled_inputs_product_mixed_aaii(1587, A::ln(A::add_scaled_square_product(s.ad_value(1586), 1.0, s.ad_value(1585), s.ad_value(1523), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1584), s.ad_value(1523))), (-1.0), 225, 1549, 1.0);s.store_offset_sub(44, 1586, 1587, (-1.0));s.store_scale(45, 1586, 4.0);}
        if (((((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1615]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1615]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1524, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1525, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1587, 1586, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub(1586, 1586, 1587);s.store_add_scaled_inputs(1586, 1586, 1.0, 225, 0.1);s.store_add_scaled_inputs_product_mixed_aaii(1588, A::ln(A::add_scaled_square_product(s.ad_value(1586), 1.0, s.ad_value(1585), s.ad_value(1523), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1584), s.ad_value(1523))), (-1.0), 225, 1549, 1.0);s.copy_ad(1589, 1571);s.store_offset_sub(44, 1588, 1589, (-(0.0008 * 75.0)));s.store_scale(45, 1588, (4.0 * (0.0008 * 75.0)));}
        if (((((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1615]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1615]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1524, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1525, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1571, 1588, 1.0, 44, (-0.5), 45, (-0.5));}
        if ((((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) {s.store_sub_div_lhs_indices(1573, 1571, 225, 1549);s.store_add_offset_lhs_mixed_ia(1524, 1571, (-1.0), A::exp_scaled_input(s.ad_value(1571), -1.0));}
        s.b[1616] = (s.v[1524] < (10.0 * 2.220446049250313e-16));s.store_scalar(1616, if s.b[1616] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1616]) {s.store_scalar(1524, (10.0 * 2.220446049250313e-16));}
        if ((((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) {s.store_sqrt(1525, 1524);s.store_mul(458, 1556, 1525);s.store_scaled_sub(459, 1559, 1573, s.v[1534]);}
        s.b[1617] = (p[41] == 1.0);s.store_scalar(1617, if s.b[1617] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) {s.store_exp_ad(1590, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1549), -1.0));s.store_scale(1523, 230, 1.0 / (s.v[69]));s.store_square(1584, 1523);s.store_mul(1599, 1584, 1590);s.store_scalar(1546, 0.0);s.store_scalar(1593, 0.0);s.store_scalar(1597, 0.0);s.store_scalar(167, 1.0);}
    }
}
