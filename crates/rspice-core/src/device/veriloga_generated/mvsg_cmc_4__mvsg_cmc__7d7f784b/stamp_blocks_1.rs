#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1202] {
            s.store_scalar(1234, p.p135);
            s.store_scalar(1235, p.p136);
            s.store_scalar(1236, p.p142);
            s.store_scalar(1237, p.p141);
            s.store_scalar(1238, p.p140);
            s.store_scalar(1239, p.p39);
            s.store_scalar(1240, p.p47);
            s.store_scalar(1241, p.p45);
            s.store_scalar(1242, p.p42);
            s.store_scalar(1243, p.p2);
            s.store_scalar(1244, p.p6);
            s.store_scalar(1245, 1.0);
            s.store_scalar(1246, 0.0);
            s.store_scalar(1247, 0.0);
            s.store_scalar(1248, 0.0);
            s.store_scalar(1249, 0.0);
            s.store_scalar(1250, 0.0);
            s.store_scalar(1251, 0.0);
            s.store_scalar(1252, 0.0);
            s.store_scalar(1253, 0.0);
            s.store_scalar(1254, 0.0);
            s.store_scalar(1255, 0.0);
            s.store_scalar(1256, 0.0);
            s.store_scalar(1257, 0.0);
            s.store_scalar(1258, 0.0);
            s.store_scalar(1259, 0.0);
            s.store_scalar(1260, 0.0);
            s.store_scalar(1261, 0.0);
            s.store_scalar(1262, 0.0);
            s.store_scalar(1263, 0.0);
            s.store_scalar(1264, 0.0);
            s.store_scalar(1265, 0.0);
            s.store_scalar(1266, 0.0);
            s.store_scalar(1267, 0.0);
            s.store_scalar(1268, 0.0);
            s.store_scalar(1269, 0.0);
            s.store_scalar(1270, 0.0);
            s.store_scalar(1271, 0.0);
            s.store_scalar(1272, 0.0);
            s.store_scalar(1273, 0.0);
            s.store_scalar(1274, 0.0);
            s.store_scalar(1275, 0.0);
            s.store_scalar(1276, 0.0);
            s.store_scalar(1277, 0.0);
            s.store_scalar(1278, 0.0);
            s.store_scalar(1279, 0.0);
            s.store_scalar(1280, 0.0);
            s.store_scalar(1281, 0.0);
            s.store_scalar(1282, 0.0);
            s.store_scalar(1283, 0.0);
            s.store_scalar(1284, 0.0);
            s.store_scalar(1285, 0.0);
            s.store_scalar(1286, 0.0);
            s.store_scalar(1287, 0.0);
            s.store_scalar(1288, 0.0);
            s.store_scalar(1289, 0.0);
            s.store_scalar(1290, 0.0);
            s.store_scalar(1291, 0.0);
            s.store_scalar(1292, 0.0);
            s.store_scalar(1293, 0.0);
            s.store_scalar(1294, 0.0);
            s.store_scalar(1295, 0.0);
            s.store_scalar(1296, 0.0);
            s.store_scalar(1297, 0.0);
            s.store_scalar(1298, 0.0);
            s.store_scalar(1299, 0.0);
            s.store_scalar(1300, 0.0);
            s.store_scalar(1301, 0.0);
            s.store_scalar(1302, 0.0);
            s.store_scalar(1303, 0.0);
            s.store_scalar(1304, 0.0);
            s.store_scalar(1305, 0.0);
            s.store_scalar(1306, 0.0);
            s.store_scalar(1307, 0.0);
            s.store_scalar(1308, 0.0);
            s.store_scalar(1309, 0.0);
            s.store_scalar(1310, 0.0);
            s.store_scalar(1311, 0.0);
            s.store_scalar(1312, 0.0);
            s.store_scalar(1313, 0.0);
            s.store_scalar(1314, 0.0);
        }

        if s.b[1202] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(1311, 1213, A::tanh_scaled_input(s.ad_value(1213), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(1311, 1213, p.p53);
                } else {
                    s.store_scalar(1311, 0.0);
                }
            }
        }

        if s.b[1202] {
            s.store_sub(1312, 1212, 1213);
            s.store_mul(1246, 1232, 1220);
            s.store_add_scaled_product_value_ad(1248, A::div_scaled_inputs(s.ad_value(1228), 1.0, s.ad_value(1220), 2.302585092994046), 1.0, 1231, 1311, 1.0);
            s.store_add_scaled_product_right_ad(1249, 1227, 1.0, 1238, A::sub(s.ad_value(1218), s.ad_value(1219)), 1.0);
            s.store_pow_ad(1267, A::div(s.ad_value(1218), s.ad_value(1219)), s.ad_value(1240));
        }

        s.b[1315] = (s.v[1239] != 0.0);
        s.v[1315] = if s.b[1315] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1315]) {
            s.store_div_ad_rhs(1250, 1311, A::pow(A::offset(A::pow(A::div(s.ad_value(1311), s.ad_value(1239)), s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235))));
        }

        if (s.b[1202] && (!s.b[1315])) {
            s.store_scalar(1250, 0.0);
        }

        if s.b[1202] {
            s.store_mul_add_scaled_product_rhs(1247, 1311, s.ad_value(1229), 1.0, s.ad_value(1250), s.ad_value(1230), (-1.0));
            s.store_sub(1210, 1249, 1247);
            s.store_scaled_mul(1252, 1248, 1220, 2.0);
            s.store_mul(1253, 1223, 1252);
            s.store_sub_scaled_inputs(1310, 1210, 1.0, 1246, (p.p51 * 0.5));
        }

        if s.b[1202] {
            let assign20060_ad_e19059: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sub(s.ad_value(1212), s.ad_value(1312)), A::tanh_scaled_input(A::sub(s.ad_value(1212), s.ad_value(1312)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1212), s.ad_value(1312)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2_mixed_aii(1309, assign20060_ad_e19059, 1.0, 1310, (-1.0), 1246, 1.0);
        }

        s.b[1316] = (s.v[1309] > 50.0);
        s.v[1316] = if s.b[1316] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1316]) {
            s.store_scalar(1268, 0.0);
        }

        s.b[1317] = (s.v[1309] < (-50.0));
        s.v[1317] = if s.b[1317] { 1.0 } else { 0.0 };

        if ((s.b[1202] && (!s.b[1316])) && s.b[1317]) {
            s.store_scalar(1268, 1.0);
        }

        if ((s.b[1202] && (!s.b[1316])) && (!s.b[1317])) {
            s.store_div_from_scalar_offset_ad(1268, 1.0, A::exp(s.ad_value(1309)), 1.0);
        }

        if s.b[1202] {
            let assign20120_ad_e19147: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sub(s.ad_value(1212), s.ad_value(1312)), A::tanh_scaled_input(A::sub(s.ad_value(1212), s.ad_value(1312)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1212), s.ad_value(1312)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2_mixed_aai(1269, assign20120_ad_e19147, 1.0, A::add_scaled_product(s.ad_value(1210), 1.0, s.ad_value(1246), s.ad_value(1268), (-(p.p51 * 0.1))), (-1.0), 1252, 1.0);
        }

        s.b[1318] = (s.v[1269] > 50.0);
        s.v[1318] = if s.b[1318] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1318]) {
            s.store_mul(1270, 1253, 1269);
        }

        s.b[1319] = (s.v[1269] < (-50.0));
        s.v[1319] = if s.b[1319] { 1.0 } else { 0.0 };

        if ((s.b[1202] && (!s.b[1318])) && s.b[1319]) {
            s.store_mul_exp_rhs(1270, 1253, 1269);
        }

        if ((s.b[1202] && (!s.b[1318])) && (!s.b[1319])) {
            s.store_mul_ln_one_plus_exp_rhs(1270, 1253, 1269);
        }

        if s.b[1202] {
            s.store_div_ad_rhs(1256, 1234, A::mul_offset_rhs(s.ad_value(1267), A::div_scaled_product(s.ad_value(1236), s.ad_value(1270), 1.0, s.ad_value(1223), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(1257, 1233, A::div_scaled_offset_numerator(A::mul(s.ad_value(1241), s.ad_value(1219)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1241), s.ad_value(1218)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(1242), s.ad_value(1311), 1.0, s.ad_value(1222), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(1237), s.ad_value(1270), 1.0, s.ad_value(1223), 1.0), 1.0), 1.0);
            s.store_add_ad(1258, A::div_scaled_product3(s.ad_value(1268), s.ad_value(1220), s.ad_value(1256), 2.0, s.ad_value(1222), 1.0), A::mul_sub_from_scalar_lhs(1.0, s.ad_value(1268), s.ad_value(1257)));
            s.store_div_scaled_product_indices(1274, 1257, 1222, 1.0, 1256, 1.0);
            s.store_add_scaled_product_right_ad(1275, 1274, (-1.0), 1274, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1270), 2.0, s.ad_value(1223), s.ad_value(1274), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(1276, A::mul_sub_from_scalar_rhs(s.ad_value(1274), 1.0, s.ad_value(1268)), 1.0, 1252, 1268, 1.0);
            s.store_add_scaled_product_value_ad(1211, A::mul_sub_from_scalar_rhs(s.ad_value(1275), 1.0, s.ad_value(1268)), 1.0, 1252, 1268, 1.0);
        }

        if s.b[1202] {
            let assign20250_ad_e19376: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1213), s.ad_value(1211)), 0.5, A::div(s.ad_value(1213), s.ad_value(1211)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1213), s.ad_value(1211))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1213), s.ad_value(1211)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(1213), s.ad_value(1211)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(1277, 1.0, A::offset(A::pow(assign20250_ad_e19376, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235)));
        }

        if s.b[1202] {
            s.store_mul(1278, 1213, 1277);
        }

    }

    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1202] {
            let assign20270_ad_e19457: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0), 0.5, A::sqrt_square_offset(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(1279, 1.0, A::offset(A::pow(assign20270_ad_e19457, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235)));
        }

        if s.b[1202] {
            s.store_mul_neg_lhs(1280, 1213, 1279);
            s.store_div_scaled_inputs2_indices(1309, 1212, 1.0, 1310, (-1.0), 1246, 1.0);
        }

        s.b[1320] = (s.v[1309] > 50.0);
        s.v[1320] = if s.b[1320] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1320]) {
            s.store_scalar(1251, 0.0);
        }

        s.b[1321] = (s.v[1309] < (-50.0));
        s.v[1321] = if s.b[1321] { 1.0 } else { 0.0 };

        if ((s.b[1202] && (!s.b[1320])) && s.b[1321]) {
            s.store_scalar(1251, 1.0);
        }

        if ((s.b[1202] && (!s.b[1320])) && (!s.b[1321])) {
            s.store_div_from_scalar_offset_ad(1251, 1.0, A::exp(s.ad_value(1309)), 1.0);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs3_mixed_iiai(1254, 1312, 1.0, 1280, (-1.0), A::add_scaled_product(s.ad_value(1210), 1.0, s.ad_value(1246), s.ad_value(1251), (-(p.p51 * 0.1))), -1.0, 1252, 1.0);
        }

        s.b[1322] = (s.v[1254] > 50.0);
        s.v[1322] = if s.b[1322] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1322]) {
            s.store_mul(1255, 1253, 1254);
        }

        s.b[1323] = (s.v[1254] < (-50.0));
        s.v[1323] = if s.b[1323] { 1.0 } else { 0.0 };

        if ((s.b[1202] && (!s.b[1322])) && s.b[1323]) {
            s.store_mul_exp_rhs(1255, 1253, 1254);
        }

        if ((s.b[1202] && (!s.b[1322])) && (!s.b[1323])) {
            s.store_mul_ln_one_plus_exp_rhs(1255, 1253, 1254);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2_indices(1309, 1312, 1.0, 1310, (-1.0), 1246, 1.0);
        }

        s.b[1324] = (s.v[1309] > 50.0);
        s.v[1324] = if s.b[1324] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1324]) {
            s.store_scalar(1281, 0.0);
        }

        s.b[1325] = (s.v[1309] < (-50.0));
        s.v[1325] = if s.b[1325] { 1.0 } else { 0.0 };

        if ((s.b[1202] && (!s.b[1324])) && s.b[1325]) {
            s.store_scalar(1281, 1.0);
        }

        if ((s.b[1202] && (!s.b[1324])) && (!s.b[1325])) {
            s.store_div_from_scalar_offset_ad(1281, 1.0, A::exp(s.ad_value(1309)), 1.0);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs3_mixed_iiai(1282, 1212, 1.0, 1278, (-1.0), A::add_scaled_product(s.ad_value(1210), 1.0, s.ad_value(1246), s.ad_value(1281), (-(p.p51 * 0.1))), -1.0, 1252, 1.0);
        }

        s.b[1326] = (s.v[1282] > 50.0);
        s.v[1326] = if s.b[1326] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1326]) {
            s.store_mul(1283, 1253, 1282);
        }

        s.b[1327] = (s.v[1282] < (-50.0));
        s.v[1327] = if s.b[1327] { 1.0 } else { 0.0 };

        if ((s.b[1202] && (!s.b[1326])) && s.b[1327]) {
            s.store_mul_exp_rhs(1283, 1253, 1282);
        }

        if ((s.b[1202] && (!s.b[1326])) && (!s.b[1327])) {
            s.store_mul_ln_one_plus_exp_rhs(1283, 1253, 1282);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2_indices(1284, 1255, 1.0, 1283, (-1.0), 1223, 1.0);
            s.store_div(1310, 1284, 1276);
        }

        if s.b[1202] {
            let assign20550_ad_e19734: A = A::pow(A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::mul(s.ad_value(1310), A::tanh_scaled_input(s.ad_value(1310), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt_square_offset(s.ad_value(1310), p.p53)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235)));
            s.store_div_ad_rhs(1285, 1310, assign20550_ad_e19734);
        }

        if s.b[1202] {
            s.store_mul(1286, 1258, 1285);
            s.store_mul_product3_rhs(1204, 1245, A::mul3(s.ad_value(1244), s.ad_value(1221), s.ad_value(1243)), A::add(s.ad_value(1255), s.ad_value(1283)), s.ad_value(1286), 0.5);
            s.store_div_scaled_inputs_indices(1259, 1228, 1.0, 1220, 2.302585092994046);
            s.store_scaled_mul(1261, 1259, 1220, 2.0);
            s.store_mul(1262, 1223, 1261);
            s.store_sub_scaled_inputs(1314, 1249, 1.0, 1246, (p.p51 * 0.5));
        }

        if s.b[1202] {
            let assign20620_ad_e19838: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sub(s.ad_value(1212), s.ad_value(1312)), A::tanh_scaled_input(A::sub(s.ad_value(1212), s.ad_value(1312)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1212), s.ad_value(1312)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2_mixed_aii(1313, assign20620_ad_e19838, 1.0, 1314, (-1.0), 1246, 1.0);
        }

        s.b[1328] = (s.v[1313] > 50.0);
        s.v[1328] = if s.b[1328] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1328]) {
            s.store_scalar(1271, 0.0);
        }

        s.b[1329] = (s.v[1313] < (-50.0));
        s.v[1329] = if s.b[1329] { 1.0 } else { 0.0 };

        if ((s.b[1202] && (!s.b[1328])) && s.b[1329]) {
            s.store_scalar(1271, 1.0);
        }

        if ((s.b[1202] && (!s.b[1328])) && (!s.b[1329])) {
            s.store_div_from_scalar_offset_ad(1271, 1.0, A::exp(s.ad_value(1313)), 1.0);
        }

        if s.b[1202] {
            let assign20680_ad_e19926: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sub(s.ad_value(1212), s.ad_value(1312)), A::tanh_scaled_input(A::sub(s.ad_value(1212), s.ad_value(1312)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1212), s.ad_value(1312)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2_mixed_aai(1272, assign20680_ad_e19926, 1.0, A::add_scaled_product(s.ad_value(1249), 1.0, s.ad_value(1246), s.ad_value(1271), (-(p.p51 * 0.1))), (-1.0), 1261, 1.0);
        }

        s.b[1330] = (s.v[1272] > 50.0);
        s.v[1330] = if s.b[1330] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1330]) {
            s.store_mul(1273, 1262, 1272);
        }

        s.b[1331] = (s.v[1272] < (-50.0));
        s.v[1331] = if s.b[1331] { 1.0 } else { 0.0 };

        if ((s.b[1202] && (!s.b[1330])) && s.b[1331]) {
            s.store_mul_exp_rhs(1273, 1262, 1272);
        }

        if ((s.b[1202] && (!s.b[1330])) && (!s.b[1331])) {
            s.store_mul_ln_one_plus_exp_rhs(1273, 1262, 1272);
        }

        if s.b[1202] {
            s.store_div(1265, 1234, 1267);
            s.store_mul_div_scaled_offset_numerator_rhs(1266, 1233, A::mul(s.ad_value(1241), s.ad_value(1219)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1241), s.ad_value(1218)), 1.0), 1.0);
            s.store_div_scaled_product_indices(1287, 1266, 1222, 1.0, 1265, 1.0);
            s.store_add_scaled_product_right_ad(1288, 1287, (-1.0), 1287, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1273), 2.0, s.ad_value(1223), s.ad_value(1287), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(1289, A::mul_sub_from_scalar_rhs(s.ad_value(1288), 1.0, s.ad_value(1271)), 1.0, 1261, 1271, 1.0);
        }

        if s.b[1202] {
            let assign20790_ad_e20101: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1213), s.ad_value(1289)), 0.5, A::div(s.ad_value(1213), s.ad_value(1289)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1213), s.ad_value(1289))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1213), s.ad_value(1289)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(1213), s.ad_value(1289)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(1290, 1.0, A::offset(A::pow(assign20790_ad_e20101, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235)));
        }

        if s.b[1202] {
            s.store_mul(1291, 1213, 1290);
        }

        if s.b[1202] {
            let assign20810_ad_e20182: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0), 0.5, A::sqrt_square_offset(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(1292, 1.0, A::offset(A::pow(assign20810_ad_e20182, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235)));
        }

        if s.b[1202] {
            s.store_mul_neg_lhs(1293, 1213, 1292);
            s.store_div_scaled_inputs2_indices(1313, 1212, 1.0, 1314, (-1.0), 1246, 1.0);
        }

        s.b[1332] = (s.v[1313] > 50.0);
        s.v[1332] = if s.b[1332] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1332]) {
            s.store_scalar(1260, 0.0);
        }

        s.b[1333] = (s.v[1313] < (-50.0));
        s.v[1333] = if s.b[1333] { 1.0 } else { 0.0 };

        if ((s.b[1202] && (!s.b[1332])) && s.b[1333]) {
            s.store_scalar(1260, 1.0);
        }

        if ((s.b[1202] && (!s.b[1332])) && (!s.b[1333])) {
            s.store_div_from_scalar_offset_ad(1260, 1.0, A::exp(s.ad_value(1313)), 1.0);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs3_mixed_iiai(1263, 1312, 1.0, 1293, (-1.0), A::add_scaled_product(s.ad_value(1249), 1.0, s.ad_value(1246), s.ad_value(1260), (-(p.p51 * 0.1))), -1.0, 1261, 1.0);
        }

        s.b[1334] = (s.v[1263] > 50.0);
        s.v[1334] = if s.b[1334] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1334]) {
            s.store_mul(1264, 1262, 1263);
        }

        s.b[1335] = (s.v[1263] < (-50.0));
        s.v[1335] = if s.b[1335] { 1.0 } else { 0.0 };

        if ((s.b[1202] && (!s.b[1334])) && s.b[1335]) {
            s.store_mul_exp_rhs(1264, 1262, 1263);
        }

        if ((s.b[1202] && (!s.b[1334])) && (!s.b[1335])) {
            s.store_mul_ln_one_plus_exp_rhs(1264, 1262, 1263);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2_indices(1313, 1312, 1.0, 1314, (-1.0), 1246, 1.0);
        }

        s.b[1336] = (s.v[1313] > 50.0);
        s.v[1336] = if s.b[1336] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1336]) {
            s.store_scalar(1294, 0.0);
        }

        s.b[1337] = (s.v[1313] < (-50.0));
        s.v[1337] = if s.b[1337] { 1.0 } else { 0.0 };

        if ((s.b[1202] && (!s.b[1336])) && s.b[1337]) {
            s.store_scalar(1294, 1.0);
        }

        if ((s.b[1202] && (!s.b[1336])) && (!s.b[1337])) {
            s.store_div_from_scalar_offset_ad(1294, 1.0, A::exp(s.ad_value(1313)), 1.0);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs3_mixed_iiai(1295, 1212, 1.0, 1291, (-1.0), A::add_scaled_product(s.ad_value(1249), 1.0, s.ad_value(1246), s.ad_value(1294), (-(p.p51 * 0.1))), -1.0, 1261, 1.0);
        }

        s.b[1338] = (s.v[1295] > 50.0);
        s.v[1338] = if s.b[1338] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1338]) {
            s.store_mul(1296, 1262, 1295);
        }

        s.b[1339] = (s.v[1295] < (-50.0));
        s.v[1339] = if s.b[1339] { 1.0 } else { 0.0 };

        if ((s.b[1202] && (!s.b[1338])) && s.b[1339]) {
            s.store_mul_exp_rhs(1296, 1262, 1295);
        }

        if ((s.b[1202] && (!s.b[1338])) && (!s.b[1339])) {
            s.store_mul_ln_one_plus_exp_rhs(1296, 1262, 1295);
        }

        if s.b[1202] {
            s.store_offset_square(1297, 1264, 1e-38);
            s.store_offset_mul(1298, 1297, 1264, 1e-57);
            s.store_offset_square(1299, 1296, 1e-38);
            s.store_offset_mul(1300, 1299, 1296, 1e-57);
            s.store_offset_mul(1301, 1264, 1296, 1e-38);
            s.store_div_scaled_inputs3_mixed_iiia(1302, 1297, (2.0 / 3.0), 1299, (2.0 / 3.0), 1301, (2.0 / 3.0), A::offset(A::add(s.ad_value(1264), s.ad_value(1296)), 2e-19), 1.0);
            s.store_div_ad(1303, A::add_scaled_inputs_products(s.ad_value(1298), (2.0 * 2.0), s.ad_value(1300), (3.0 * 2.0), s.ad_value(1297), s.ad_value(1296), (4.0 * 2.0), s.ad_value(1299), s.ad_value(1264), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(1297), 15.0, s.ad_value(1299), 15.0, s.ad_value(1301), (2.0 * 15.0)));
            s.store_sub(1304, 1302, 1303);
            s.copy_ad(1305, 1303);
            s.store_mul_product3_rhs(1205, 1245, A::mul3(s.ad_value(1221), s.ad_value(1243), s.ad_value(1222)), s.ad_value(1244), s.ad_value(1304), 1.0);
            s.store_mul_product3_rhs(1206, 1245, A::mul3(s.ad_value(1221), s.ad_value(1243), s.ad_value(1222)), s.ad_value(1244), s.ad_value(1305), 1.0);
        }

        s.b[1340] = (s.v[1214] == 1.0);
        s.v[1340] = if s.b[1340] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1340]) {
            s.store_div_scaled_inputs3_indices(1306, 1215, 1.0, 1249, -1.0, 1246, (-(-(p.p51 * 0.5))), 1261, 1.0);
        }

        s.b[1341] = (s.v[1306] > 50.0);
        s.v[1341] = if s.b[1341] { 1.0 } else { 0.0 };

        if ((s.b[1202] && s.b[1340]) && s.b[1341]) {
            s.copy_ad(1309, 1306);
        }

        s.b[1342] = (s.v[1306] < (-50.0));
        s.v[1342] = if s.b[1342] { 1.0 } else { 0.0 };

        if (((s.b[1202] && s.b[1340]) && (!s.b[1341])) && s.b[1342]) {
            s.store_exp(1309, 1306);
        }

        if (((s.b[1202] && s.b[1340]) && (!s.b[1341])) && (!s.b[1342])) {
            s.store_ln_one_plus_exp(1309, 1306);
        }

        if (s.b[1202] && s.b[1340]) {
            s.store_mul_ad_product_lhs(1207, A::mul3(A::mul3(s.ad_value(1221), s.ad_value(1243), s.ad_value(1244)), s.ad_value(1225), s.ad_value(1261)), s.ad_value(1309), 1245);
            s.store_div_scaled_inputs3_indices(1307, 1216, 1.0, 1249, -1.0, 1246, (-(-(p.p51 * 0.5))), 1261, 1.0);
        }

        s.b[1343] = (s.v[1307] > 50.0);
        s.v[1343] = if s.b[1343] { 1.0 } else { 0.0 };

        if ((s.b[1202] && s.b[1340]) && s.b[1343]) {
            s.copy_ad(1309, 1307);
        }

        s.b[1344] = (s.v[1307] < (-50.0));
        s.v[1344] = if s.b[1344] { 1.0 } else { 0.0 };

        if (((s.b[1202] && s.b[1340]) && (!s.b[1343])) && s.b[1344]) {
            s.store_exp(1309, 1307);
        }

        if (((s.b[1202] && s.b[1340]) && (!s.b[1343])) && (!s.b[1344])) {
            s.store_ln_one_plus_exp(1309, 1307);
        }

        if (s.b[1202] && s.b[1340]) {
            s.store_mul_ad_product_lhs(1208, A::mul3(A::mul3(s.ad_value(1221), s.ad_value(1243), s.ad_value(1244)), s.ad_value(1226), s.ad_value(1261)), s.ad_value(1309), 1245);
        }

        if (s.b[1202] && (!s.b[1340])) {
            s.store_scalar(1207, 0.0);
            s.store_scalar(1208, 0.0);
        }

        s.b[1345] = (s.v[1217] == 1.0);
        s.v[1345] = if s.b[1345] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1345]) {
            s.store_div_scaled_inputs3_indices(1308, 1212, 1.0, 1249, -1.0, 1246, (-(-(p.p51 * 0.5))), 1261, 1.0);
        }

        s.b[1346] = (s.v[1308] > 50.0);
        s.v[1346] = if s.b[1346] { 1.0 } else { 0.0 };

        if ((s.b[1202] && s.b[1345]) && s.b[1346]) {
            s.copy_ad(1309, 1308);
        }

        s.b[1347] = (s.v[1308] < (-50.0));
        s.v[1347] = if s.b[1347] { 1.0 } else { 0.0 };

        if (((s.b[1202] && s.b[1345]) && (!s.b[1346])) && s.b[1347]) {
            s.store_exp(1309, 1308);
        }

        if (((s.b[1202] && s.b[1345]) && (!s.b[1346])) && (!s.b[1347])) {
            s.store_ln_one_plus_exp(1309, 1308);
        }

    }

    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1202] && s.b[1345]) {
            s.store_mul_ad_product_lhs(1209, A::mul3(A::mul3(s.ad_value(1221), s.ad_value(1243), s.ad_value(1244)), s.ad_value(1224), s.ad_value(1261)), s.ad_value(1309), 1245);
        }

        if (s.b[1202] && (!s.b[1345])) {
            s.store_scalar(1209, 0.0);
        }

        if s.b[1202] {
            s.copy_ad(1203, 1204);
            s.copy_ad(178, 1204);
            s.copy_ad(179, 1205);
            s.copy_ad(180, 1206);
            s.copy_ad(181, 1207);
            s.copy_ad(182, 1208);
            s.copy_ad(183, 1209);
            s.copy_ad(178, 1203);
        }

        s.b[1348] = (p.p122 == 1.0);
        s.v[1348] = if s.b[1348] { 1.0 } else { 0.0 };

        s.v[184] = 0.0;

        s.v[185] = 0.0;

        s.v[186] = 0.0;

        s.v[187] = 0.0;

        s.v[188] = 0.0;

        s.v[189] = 0.0;

        s.b[1349] = (p.p145 > p.p354);
        s.v[1349] = if s.b[1349] { 1.0 } else { 0.0 };

        if s.b[1349] {
            s.store_scalar(1350, 0.0);
            s.store_scalar(1351, 0.0);
            s.store_scalar(1352, 0.0);
            s.store_scalar(1353, 0.0);
            s.store_scalar(1354, 0.0);
            s.store_scalar(1355, 0.0);
            s.store_scalar(1356, 0.0);
            s.store_scalar(1357, 0.0);
            s.store_scalar(1358, 0.0);
            s.copy_ad(1359, 78);
            s.copy_ad(1360, 79);
        }

        let (assign21750_e20901,) = {
    if s.b[1349] {
        (p.p151,)
    } else {
        (s.v[1361],)
    }
};
        s.v[1361] = assign21750_e20901;

        if s.b[1349] {
            s.copy_ad(1362, 80);
            s.copy_ad(1363, 81);
        }

        let (assign21780_e20913,) = {
    if s.b[1349] {
        (p.p149,)
    } else {
        (s.v[1364],)
    }
};
        s.v[1364] = assign21780_e20913;

        if s.b[1349] {
            s.copy_ad(1365, 111);
            s.store_scalar(1366, s.v[109]);
            s.copy_ad(1367, 113);
            s.store_scalar(1368, p.p0);
            s.store_scalar(1369, p.p145);
            s.copy_ad(1370, 29);
            s.store_scalar(1371, p.p150);
            s.copy_ad(1372, 30);
            s.copy_ad(1373, 31);
            s.store_scalar(1374, p.p146);
            s.store_scalar(1375, p.p160);
            s.store_scalar(1376, p.p159);
            s.store_scalar(1377, 0.0);
            s.store_scalar(1378, p.p161);
            s.store_scalar(1379, p.p165);
            s.store_scalar(1380, p.p156);
            s.store_scalar(1381, p.p157);
            s.store_scalar(1382, p.p158);
            s.store_scalar(1383, p.p164);
            s.store_scalar(1384, p.p163);
            s.store_scalar(1385, p.p162);
            s.store_scalar(1386, p.p39);
            s.store_scalar(1387, p.p47);
            s.store_scalar(1388, p.p45);
            s.store_scalar(1389, p.p42);
            s.store_scalar(1390, p.p2);
            s.store_scalar(1391, p.p6);
            s.store_scalar(1392, 1.0);
            s.store_scalar(1393, 0.0);
            s.store_scalar(1394, 0.0);
            s.store_scalar(1395, 0.0);
            s.store_scalar(1396, 0.0);
            s.store_scalar(1397, 0.0);
            s.store_scalar(1398, 0.0);
            s.store_scalar(1399, 0.0);
            s.store_scalar(1400, 0.0);
            s.store_scalar(1401, 0.0);
            s.store_scalar(1402, 0.0);
            s.store_scalar(1403, 0.0);
            s.store_scalar(1404, 0.0);
            s.store_scalar(1405, 0.0);
            s.store_scalar(1406, 0.0);
            s.store_scalar(1407, 0.0);
            s.store_scalar(1408, 0.0);
            s.store_scalar(1409, 0.0);
            s.store_scalar(1410, 0.0);
            s.store_scalar(1411, 0.0);
            s.store_scalar(1412, 0.0);
            s.store_scalar(1413, 0.0);
            s.store_scalar(1414, 0.0);
            s.store_scalar(1415, 0.0);
            s.store_scalar(1416, 0.0);
            s.store_scalar(1417, 0.0);
            s.store_scalar(1418, 0.0);
            s.store_scalar(1419, 0.0);
            s.store_scalar(1420, 0.0);
            s.store_scalar(1421, 0.0);
            s.store_scalar(1422, 0.0);
            s.store_scalar(1423, 0.0);
            s.store_scalar(1424, 0.0);
            s.store_scalar(1425, 0.0);
            s.store_scalar(1426, 0.0);
            s.store_scalar(1427, 0.0);
            s.store_scalar(1428, 0.0);
            s.store_scalar(1429, 0.0);
            s.store_scalar(1430, 0.0);
            s.store_scalar(1431, 0.0);
            s.store_scalar(1432, 0.0);
            s.store_scalar(1433, 0.0);
            s.store_scalar(1434, 0.0);
            s.store_scalar(1435, 0.0);
            s.store_scalar(1436, 0.0);
            s.store_scalar(1437, 0.0);
            s.store_scalar(1438, 0.0);
            s.store_scalar(1439, 0.0);
            s.store_scalar(1440, 0.0);
            s.store_scalar(1441, 0.0);
            s.store_scalar(1442, 0.0);
            s.store_scalar(1443, 0.0);
            s.store_scalar(1444, 0.0);
            s.store_scalar(1445, 0.0);
            s.store_scalar(1446, 0.0);
            s.store_scalar(1447, 0.0);
            s.store_scalar(1448, 0.0);
            s.store_scalar(1449, 0.0);
            s.store_scalar(1450, 0.0);
            s.store_scalar(1451, 0.0);
            s.store_scalar(1452, 0.0);
            s.store_scalar(1453, 0.0);
            s.store_scalar(1454, 0.0);
            s.store_scalar(1455, 0.0);
            s.store_scalar(1456, 0.0);
            s.store_scalar(1457, 0.0);
            s.store_scalar(1458, 0.0);
            s.store_scalar(1459, 0.0);
            s.store_scalar(1460, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1349] {
            s.store_scalar(1461, 0.0);
        }

        if s.b[1349] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(1458, 1360, A::tanh_scaled_input(s.ad_value(1360), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(1458, 1360, p.p53);
                } else {
                    s.store_scalar(1458, 0.0);
                }
            }
        }

        if s.b[1349] {
            s.store_sub(1459, 1359, 1360);
            s.store_mul(1393, 1379, 1367);
            s.store_add_scaled_product_value_ad(1395, A::div_scaled_inputs(s.ad_value(1375), 1.0, s.ad_value(1367), 2.302585092994046), 1.0, 1378, 1458, 1.0);
            s.store_add_scaled_product_right_ad(1396, 1374, 1.0, 1385, A::sub(s.ad_value(1365), s.ad_value(1366)), 1.0);
            s.store_pow_ad(1414, A::div(s.ad_value(1365), s.ad_value(1366)), s.ad_value(1387));
        }

        s.b[1462] = (s.v[1386] != 0.0);
        s.v[1462] = if s.b[1462] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1462]) {
            s.store_div_ad_rhs(1397, 1458, A::pow(A::offset(A::pow(A::div(s.ad_value(1458), s.ad_value(1386)), s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382))));
        }

        if (s.b[1349] && (!s.b[1462])) {
            s.store_scalar(1397, 0.0);
        }

        if s.b[1349] {
            s.store_mul_add_scaled_product_rhs(1394, 1458, s.ad_value(1376), 1.0, s.ad_value(1397), s.ad_value(1377), (-1.0));
            s.store_sub(1357, 1396, 1394);
            s.store_scaled_mul(1399, 1395, 1367, 2.0);
            s.store_mul(1400, 1370, 1399);
            s.store_sub_scaled_inputs(1457, 1357, 1.0, 1393, (p.p51 * 0.5));
        }

        if s.b[1349] {
            let assign22900_ad_e21483: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sub(s.ad_value(1359), s.ad_value(1459)), A::tanh_scaled_input(A::sub(s.ad_value(1359), s.ad_value(1459)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1359), s.ad_value(1459)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2_mixed_aii(1456, assign22900_ad_e21483, 1.0, 1457, (-1.0), 1393, 1.0);
        }

        s.b[1463] = (s.v[1456] > 50.0);
        s.v[1463] = if s.b[1463] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1463]) {
            s.store_scalar(1415, 0.0);
        }

        s.b[1464] = (s.v[1456] < (-50.0));
        s.v[1464] = if s.b[1464] { 1.0 } else { 0.0 };

        if ((s.b[1349] && (!s.b[1463])) && s.b[1464]) {
            s.store_scalar(1415, 1.0);
        }

        if ((s.b[1349] && (!s.b[1463])) && (!s.b[1464])) {
            s.store_div_from_scalar_offset_ad(1415, 1.0, A::exp(s.ad_value(1456)), 1.0);
        }

        if s.b[1349] {
            let assign22960_ad_e21571: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sub(s.ad_value(1359), s.ad_value(1459)), A::tanh_scaled_input(A::sub(s.ad_value(1359), s.ad_value(1459)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1359), s.ad_value(1459)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2_mixed_aai(1416, assign22960_ad_e21571, 1.0, A::add_scaled_product(s.ad_value(1357), 1.0, s.ad_value(1393), s.ad_value(1415), (-(p.p51 * 0.1))), (-1.0), 1399, 1.0);
        }

        s.b[1465] = (s.v[1416] > 50.0);
        s.v[1465] = if s.b[1465] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1465]) {
            s.store_mul(1417, 1400, 1416);
        }

        s.b[1466] = (s.v[1416] < (-50.0));
        s.v[1466] = if s.b[1466] { 1.0 } else { 0.0 };

        if ((s.b[1349] && (!s.b[1465])) && s.b[1466]) {
            s.store_mul_exp_rhs(1417, 1400, 1416);
        }

        if ((s.b[1349] && (!s.b[1465])) && (!s.b[1466])) {
            s.store_mul_ln_one_plus_exp_rhs(1417, 1400, 1416);
        }

        if s.b[1349] {
            s.store_div_ad_rhs(1403, 1381, A::mul_offset_rhs(s.ad_value(1414), A::div_scaled_product(s.ad_value(1383), s.ad_value(1417), 1.0, s.ad_value(1370), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(1404, 1380, A::div_scaled_offset_numerator(A::mul(s.ad_value(1388), s.ad_value(1366)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1388), s.ad_value(1365)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(1389), s.ad_value(1458), 1.0, s.ad_value(1369), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(1384), s.ad_value(1417), 1.0, s.ad_value(1370), 1.0), 1.0), 1.0);
            s.store_add_ad(1405, A::div_scaled_product3(s.ad_value(1415), s.ad_value(1367), s.ad_value(1403), 2.0, s.ad_value(1369), 1.0), A::mul_sub_from_scalar_lhs(1.0, s.ad_value(1415), s.ad_value(1404)));
            s.store_div_scaled_product_indices(1421, 1404, 1369, 1.0, 1403, 1.0);
            s.store_add_scaled_product_right_ad(1422, 1421, (-1.0), 1421, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1417), 2.0, s.ad_value(1370), s.ad_value(1421), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(1423, A::mul_sub_from_scalar_rhs(s.ad_value(1421), 1.0, s.ad_value(1415)), 1.0, 1399, 1415, 1.0);
            s.store_add_scaled_product_value_ad(1358, A::mul_sub_from_scalar_rhs(s.ad_value(1422), 1.0, s.ad_value(1415)), 1.0, 1399, 1415, 1.0);
        }

        if s.b[1349] {
            let assign23090_ad_e21800: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1360), s.ad_value(1358)), 0.5, A::div(s.ad_value(1360), s.ad_value(1358)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1360), s.ad_value(1358))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1360), s.ad_value(1358)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(1360), s.ad_value(1358)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(1424, 1.0, A::offset(A::pow(assign23090_ad_e21800, s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382)));
        }

        if s.b[1349] {
            s.store_mul(1425, 1360, 1424);
        }

        if s.b[1349] {
            let assign23110_ad_e21881: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1358), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1358), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1358), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1358), 1.0), 0.5, A::sqrt_square_offset(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1358), 1.0), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(1426, 1.0, A::offset(A::pow(assign23110_ad_e21881, s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382)));
        }

        if s.b[1349] {
            s.store_mul_neg_lhs(1427, 1360, 1426);
            s.store_div_scaled_inputs2_indices(1456, 1359, 1.0, 1457, (-1.0), 1393, 1.0);
        }

        s.b[1467] = (s.v[1456] > 50.0);
        s.v[1467] = if s.b[1467] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1467]) {
            s.store_scalar(1398, 0.0);
        }

        s.b[1468] = (s.v[1456] < (-50.0));
        s.v[1468] = if s.b[1468] { 1.0 } else { 0.0 };

        if ((s.b[1349] && (!s.b[1467])) && s.b[1468]) {
            s.store_scalar(1398, 1.0);
        }

        if ((s.b[1349] && (!s.b[1467])) && (!s.b[1468])) {
            s.store_div_from_scalar_offset_ad(1398, 1.0, A::exp(s.ad_value(1456)), 1.0);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs3_mixed_iiai(1401, 1459, 1.0, 1427, (-1.0), A::add_scaled_product(s.ad_value(1357), 1.0, s.ad_value(1393), s.ad_value(1398), (-(p.p51 * 0.1))), -1.0, 1399, 1.0);
        }

        s.b[1469] = (s.v[1401] > 50.0);
        s.v[1469] = if s.b[1469] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1469]) {
            s.store_mul(1402, 1400, 1401);
        }

        s.b[1470] = (s.v[1401] < (-50.0));
        s.v[1470] = if s.b[1470] { 1.0 } else { 0.0 };

        if ((s.b[1349] && (!s.b[1469])) && s.b[1470]) {
            s.store_mul_exp_rhs(1402, 1400, 1401);
        }

        if ((s.b[1349] && (!s.b[1469])) && (!s.b[1470])) {
            s.store_mul_ln_one_plus_exp_rhs(1402, 1400, 1401);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs2_indices(1456, 1459, 1.0, 1457, (-1.0), 1393, 1.0);
        }

        s.b[1471] = (s.v[1456] > 50.0);
        s.v[1471] = if s.b[1471] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1471]) {
            s.store_scalar(1428, 0.0);
        }

        s.b[1472] = (s.v[1456] < (-50.0));
        s.v[1472] = if s.b[1472] { 1.0 } else { 0.0 };

        if ((s.b[1349] && (!s.b[1471])) && s.b[1472]) {
            s.store_scalar(1428, 1.0);
        }

        if ((s.b[1349] && (!s.b[1471])) && (!s.b[1472])) {
            s.store_div_from_scalar_offset_ad(1428, 1.0, A::exp(s.ad_value(1456)), 1.0);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs3_mixed_iiai(1429, 1359, 1.0, 1425, (-1.0), A::add_scaled_product(s.ad_value(1357), 1.0, s.ad_value(1393), s.ad_value(1428), (-(p.p51 * 0.1))), -1.0, 1399, 1.0);
        }

        s.b[1473] = (s.v[1429] > 50.0);
        s.v[1473] = if s.b[1473] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1473]) {
            s.store_mul(1430, 1400, 1429);
        }

        s.b[1474] = (s.v[1429] < (-50.0));
        s.v[1474] = if s.b[1474] { 1.0 } else { 0.0 };

        if ((s.b[1349] && (!s.b[1473])) && s.b[1474]) {
            s.store_mul_exp_rhs(1430, 1400, 1429);
        }

        if ((s.b[1349] && (!s.b[1473])) && (!s.b[1474])) {
            s.store_mul_ln_one_plus_exp_rhs(1430, 1400, 1429);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs2_indices(1431, 1402, 1.0, 1430, (-1.0), 1370, 1.0);
            s.store_div(1457, 1431, 1423);
        }

        if s.b[1349] {
            let assign23390_ad_e22158: A = A::pow(A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::mul(s.ad_value(1457), A::tanh_scaled_input(s.ad_value(1457), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt_square_offset(s.ad_value(1457), p.p53)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382)));
            s.store_div_ad_rhs(1432, 1457, assign23390_ad_e22158);
        }

        if s.b[1349] {
            s.store_mul(1433, 1405, 1432);
            s.store_mul_product3_rhs(1351, 1392, A::mul3(s.ad_value(1391), s.ad_value(1368), s.ad_value(1390)), A::add(s.ad_value(1402), s.ad_value(1430)), s.ad_value(1433), 0.5);
            s.store_div_scaled_inputs_indices(1406, 1375, 1.0, 1367, 2.302585092994046);
            s.store_scaled_mul(1408, 1406, 1367, 2.0);
            s.store_mul(1409, 1370, 1408);
            s.store_sub_scaled_inputs(1461, 1396, 1.0, 1393, (p.p51 * 0.5));
        }

        if s.b[1349] {
            let assign23460_ad_e22262: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sub(s.ad_value(1359), s.ad_value(1459)), A::tanh_scaled_input(A::sub(s.ad_value(1359), s.ad_value(1459)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1359), s.ad_value(1459)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2_mixed_aii(1460, assign23460_ad_e22262, 1.0, 1461, (-1.0), 1393, 1.0);
        }

        s.b[1475] = (s.v[1460] > 50.0);
        s.v[1475] = if s.b[1475] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1475]) {
            s.store_scalar(1418, 0.0);
        }

        s.b[1476] = (s.v[1460] < (-50.0));
        s.v[1476] = if s.b[1476] { 1.0 } else { 0.0 };

        if ((s.b[1349] && (!s.b[1475])) && s.b[1476]) {
            s.store_scalar(1418, 1.0);
        }

        if ((s.b[1349] && (!s.b[1475])) && (!s.b[1476])) {
            s.store_div_from_scalar_offset_ad(1418, 1.0, A::exp(s.ad_value(1460)), 1.0);
        }

        if s.b[1349] {
            let assign23520_ad_e22350: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sub(s.ad_value(1359), s.ad_value(1459)), A::tanh_scaled_input(A::sub(s.ad_value(1359), s.ad_value(1459)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1359), s.ad_value(1459)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2_mixed_aai(1419, assign23520_ad_e22350, 1.0, A::add_scaled_product(s.ad_value(1396), 1.0, s.ad_value(1393), s.ad_value(1418), (-(p.p51 * 0.1))), (-1.0), 1408, 1.0);
        }

        s.b[1477] = (s.v[1419] > 50.0);
        s.v[1477] = if s.b[1477] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1477]) {
            s.store_mul(1420, 1409, 1419);
        }

        s.b[1478] = (s.v[1419] < (-50.0));
        s.v[1478] = if s.b[1478] { 1.0 } else { 0.0 };

        if ((s.b[1349] && (!s.b[1477])) && s.b[1478]) {
            s.store_mul_exp_rhs(1420, 1409, 1419);
        }

        if ((s.b[1349] && (!s.b[1477])) && (!s.b[1478])) {
            s.store_mul_ln_one_plus_exp_rhs(1420, 1409, 1419);
        }

        if s.b[1349] {
            s.store_div(1412, 1381, 1414);
            s.store_mul_div_scaled_offset_numerator_rhs(1413, 1380, A::mul(s.ad_value(1388), s.ad_value(1366)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1388), s.ad_value(1365)), 1.0), 1.0);
            s.store_div_scaled_product_indices(1434, 1413, 1369, 1.0, 1412, 1.0);
            s.store_add_scaled_product_right_ad(1435, 1434, (-1.0), 1434, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1420), 2.0, s.ad_value(1370), s.ad_value(1434), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(1436, A::mul_sub_from_scalar_rhs(s.ad_value(1435), 1.0, s.ad_value(1418)), 1.0, 1408, 1418, 1.0);
        }

        if s.b[1349] {
            let assign23630_ad_e22525: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1360), s.ad_value(1436)), 0.5, A::div(s.ad_value(1360), s.ad_value(1436)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1360), s.ad_value(1436))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1360), s.ad_value(1436)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(1360), s.ad_value(1436)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(1437, 1.0, A::offset(A::pow(assign23630_ad_e22525, s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382)));
        }

        if s.b[1349] {
            s.store_mul(1438, 1360, 1437);
        }

        if s.b[1349] {
            let assign23650_ad_e22606: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1436), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1436), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1436), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1436), 1.0), 0.5, A::sqrt_square_offset(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1436), 1.0), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(1439, 1.0, A::offset(A::pow(assign23650_ad_e22606, s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382)));
        }

        if s.b[1349] {
            s.store_mul_neg_lhs(1440, 1360, 1439);
            s.store_div_scaled_inputs2_indices(1460, 1359, 1.0, 1461, (-1.0), 1393, 1.0);
        }

        s.b[1479] = (s.v[1460] > 50.0);
        s.v[1479] = if s.b[1479] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1479]) {
            s.store_scalar(1407, 0.0);
        }

        s.b[1480] = (s.v[1460] < (-50.0));
        s.v[1480] = if s.b[1480] { 1.0 } else { 0.0 };

        if ((s.b[1349] && (!s.b[1479])) && s.b[1480]) {
            s.store_scalar(1407, 1.0);
        }

        if ((s.b[1349] && (!s.b[1479])) && (!s.b[1480])) {
            s.store_div_from_scalar_offset_ad(1407, 1.0, A::exp(s.ad_value(1460)), 1.0);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs3_mixed_iiai(1410, 1459, 1.0, 1440, (-1.0), A::add_scaled_product(s.ad_value(1396), 1.0, s.ad_value(1393), s.ad_value(1407), (-(p.p51 * 0.1))), -1.0, 1408, 1.0);
        }

        s.b[1481] = (s.v[1410] > 50.0);
        s.v[1481] = if s.b[1481] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1349] && s.b[1481]) {
            s.store_mul(1411, 1409, 1410);
        }

        s.b[1482] = (s.v[1410] < (-50.0));
        s.v[1482] = if s.b[1482] { 1.0 } else { 0.0 };

        if ((s.b[1349] && (!s.b[1481])) && s.b[1482]) {
            s.store_mul_exp_rhs(1411, 1409, 1410);
        }

        if ((s.b[1349] && (!s.b[1481])) && (!s.b[1482])) {
            s.store_mul_ln_one_plus_exp_rhs(1411, 1409, 1410);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs2_indices(1460, 1459, 1.0, 1461, (-1.0), 1393, 1.0);
        }

        s.b[1483] = (s.v[1460] > 50.0);
        s.v[1483] = if s.b[1483] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1483]) {
            s.store_scalar(1441, 0.0);
        }

        s.b[1484] = (s.v[1460] < (-50.0));
        s.v[1484] = if s.b[1484] { 1.0 } else { 0.0 };

        if ((s.b[1349] && (!s.b[1483])) && s.b[1484]) {
            s.store_scalar(1441, 1.0);
        }

        if ((s.b[1349] && (!s.b[1483])) && (!s.b[1484])) {
            s.store_div_from_scalar_offset_ad(1441, 1.0, A::exp(s.ad_value(1460)), 1.0);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs3_mixed_iiai(1442, 1359, 1.0, 1438, (-1.0), A::add_scaled_product(s.ad_value(1396), 1.0, s.ad_value(1393), s.ad_value(1441), (-(p.p51 * 0.1))), -1.0, 1408, 1.0);
        }

        s.b[1485] = (s.v[1442] > 50.0);
        s.v[1485] = if s.b[1485] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1485]) {
            s.store_mul(1443, 1409, 1442);
        }

        s.b[1486] = (s.v[1442] < (-50.0));
        s.v[1486] = if s.b[1486] { 1.0 } else { 0.0 };

        if ((s.b[1349] && (!s.b[1485])) && s.b[1486]) {
            s.store_mul_exp_rhs(1443, 1409, 1442);
        }

        if ((s.b[1349] && (!s.b[1485])) && (!s.b[1486])) {
            s.store_mul_ln_one_plus_exp_rhs(1443, 1409, 1442);
        }

        if s.b[1349] {
            s.store_offset_square(1444, 1411, 1e-38);
            s.store_offset_mul(1445, 1444, 1411, 1e-57);
            s.store_offset_square(1446, 1443, 1e-38);
            s.store_offset_mul(1447, 1446, 1443, 1e-57);
            s.store_offset_mul(1448, 1411, 1443, 1e-38);
            s.store_div_scaled_inputs3_mixed_iiia(1449, 1444, (2.0 / 3.0), 1446, (2.0 / 3.0), 1448, (2.0 / 3.0), A::offset(A::add(s.ad_value(1411), s.ad_value(1443)), 2e-19), 1.0);
            s.store_div_ad(1450, A::add_scaled_inputs_products(s.ad_value(1445), (2.0 * 2.0), s.ad_value(1447), (3.0 * 2.0), s.ad_value(1444), s.ad_value(1443), (4.0 * 2.0), s.ad_value(1446), s.ad_value(1411), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(1444), 15.0, s.ad_value(1446), 15.0, s.ad_value(1448), (2.0 * 15.0)));
            s.store_sub(1451, 1449, 1450);
            s.copy_ad(1452, 1450);
            s.store_mul_product3_rhs(1352, 1392, A::mul3(s.ad_value(1368), s.ad_value(1390), s.ad_value(1369)), s.ad_value(1391), s.ad_value(1451), 1.0);
            s.store_mul_product3_rhs(1353, 1392, A::mul3(s.ad_value(1368), s.ad_value(1390), s.ad_value(1369)), s.ad_value(1391), s.ad_value(1452), 1.0);
        }

        s.b[1487] = (s.v[1361] == 1.0);
        s.v[1487] = if s.b[1487] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1487]) {
            s.store_div_scaled_inputs3_indices(1453, 1362, 1.0, 1396, -1.0, 1393, (-(-(p.p51 * 0.5))), 1408, 1.0);
        }

        s.b[1488] = (s.v[1453] > 50.0);
        s.v[1488] = if s.b[1488] { 1.0 } else { 0.0 };

        if ((s.b[1349] && s.b[1487]) && s.b[1488]) {
            s.copy_ad(1456, 1453);
        }

        s.b[1489] = (s.v[1453] < (-50.0));
        s.v[1489] = if s.b[1489] { 1.0 } else { 0.0 };

        if (((s.b[1349] && s.b[1487]) && (!s.b[1488])) && s.b[1489]) {
            s.store_exp(1456, 1453);
        }

        if (((s.b[1349] && s.b[1487]) && (!s.b[1488])) && (!s.b[1489])) {
            s.store_ln_one_plus_exp(1456, 1453);
        }

        if (s.b[1349] && s.b[1487]) {
            s.store_mul_ad_product_lhs(1354, A::mul3(A::mul3(s.ad_value(1368), s.ad_value(1390), s.ad_value(1391)), s.ad_value(1372), s.ad_value(1408)), s.ad_value(1456), 1392);
            s.store_div_scaled_inputs3_indices(1454, 1363, 1.0, 1396, -1.0, 1393, (-(-(p.p51 * 0.5))), 1408, 1.0);
        }

        s.b[1490] = (s.v[1454] > 50.0);
        s.v[1490] = if s.b[1490] { 1.0 } else { 0.0 };

        if ((s.b[1349] && s.b[1487]) && s.b[1490]) {
            s.copy_ad(1456, 1454);
        }

        s.b[1491] = (s.v[1454] < (-50.0));
        s.v[1491] = if s.b[1491] { 1.0 } else { 0.0 };

        if (((s.b[1349] && s.b[1487]) && (!s.b[1490])) && s.b[1491]) {
            s.store_exp(1456, 1454);
        }

        if (((s.b[1349] && s.b[1487]) && (!s.b[1490])) && (!s.b[1491])) {
            s.store_ln_one_plus_exp(1456, 1454);
        }

        if (s.b[1349] && s.b[1487]) {
            s.store_mul_ad_product_lhs(1355, A::mul3(A::mul3(s.ad_value(1368), s.ad_value(1390), s.ad_value(1391)), s.ad_value(1373), s.ad_value(1408)), s.ad_value(1456), 1392);
        }

        if (s.b[1349] && (!s.b[1487])) {
            s.store_scalar(1354, 0.0);
            s.store_scalar(1355, 0.0);
        }

        s.b[1492] = (s.v[1364] == 1.0);
        s.v[1492] = if s.b[1492] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1492]) {
            s.store_div_scaled_inputs3_indices(1455, 1359, 1.0, 1396, -1.0, 1393, (-(-(p.p51 * 0.5))), 1408, 1.0);
        }

        s.b[1493] = (s.v[1455] > 50.0);
        s.v[1493] = if s.b[1493] { 1.0 } else { 0.0 };

        if ((s.b[1349] && s.b[1492]) && s.b[1493]) {
            s.copy_ad(1456, 1455);
        }

        s.b[1494] = (s.v[1455] < (-50.0));
        s.v[1494] = if s.b[1494] { 1.0 } else { 0.0 };

        if (((s.b[1349] && s.b[1492]) && (!s.b[1493])) && s.b[1494]) {
            s.store_exp(1456, 1455);
        }

        if (((s.b[1349] && s.b[1492]) && (!s.b[1493])) && (!s.b[1494])) {
            s.store_ln_one_plus_exp(1456, 1455);
        }

        if (s.b[1349] && s.b[1492]) {
            s.store_mul_ad_product_lhs(1356, A::mul3(A::mul3(s.ad_value(1368), s.ad_value(1390), s.ad_value(1391)), s.ad_value(1371), s.ad_value(1408)), s.ad_value(1456), 1392);
        }

        if (s.b[1349] && (!s.b[1492])) {
            s.store_scalar(1356, 0.0);
        }

        if s.b[1349] {
            s.copy_ad(1350, 1351);
            s.copy_ad(184, 1351);
            s.copy_ad(185, 1352);
            s.copy_ad(186, 1353);
            s.copy_ad(187, 1354);
            s.copy_ad(188, 1355);
            s.copy_ad(189, 1356);
            s.copy_ad(184, 1350);
        }

        s.b[1495] = (p.p144 == 1.0);
        s.v[1495] = if s.b[1495] { 1.0 } else { 0.0 };

        s.v[154] = 0.0;

        s.b[1496] = ((p.p50 == 0.0) && (p.p54 > p.p354));
        s.v[1496] = if s.b[1496] { 1.0 } else { 0.0 };

        if s.b[1496] {
            s.store_scalar(1497, 0.0);
            s.store_scalar(1498, 0.0);
            s.store_scalar(1504, 0.0);
            s.store_scalar(1505, 0.0);
            s.copy_ad(1506, 52);
            s.copy_ad(1507, 53);
        }

        let (assign24590_e23329,) = {
    if s.b[1496] {
        (0.0,)
    } else {
        (s.v[1508],)
    }
};
        s.v[1508] = assign24590_e23329;

        if s.b[1496] {
            s.store_scalar(1509, 0.0);
            s.store_scalar(1510, 0.0);
        }

        let (assign24620_e23341,) = {
    if s.b[1496] {
        (0.0,)
    } else {
        (s.v[1511],)
    }
};
        s.v[1511] = assign24620_e23341;

        if s.b[1496] {
            s.copy_ad(1512, 111);
            s.store_scalar(1513, s.v[109]);
            s.copy_ad(1514, 113);
            s.store_scalar(1515, p.p0);
            s.store_scalar(1516, p.p54);
            s.store_scalar(1517, p.p56);
            s.store_scalar(1521, p.p55);
            s.store_scalar(1522, p.p61);
            s.store_scalar(1523, p.p60);
            s.store_scalar(1524, 0.0);
            s.store_scalar(1525, p.p62);
            s.store_scalar(1526, p.p65);
            s.store_scalar(1527, p.p57);
            s.store_scalar(1528, p.p58);
            s.store_scalar(1529, p.p59);
            s.store_scalar(1530, p.p64);
            s.store_scalar(1531, p.p63);
            s.store_scalar(1532, p.p46);
            s.store_scalar(1533, p.p39);
            s.store_scalar(1534, p.p47);
            s.store_scalar(1535, p.p45);
            s.store_scalar(1536, p.p42);
            s.store_scalar(1537, p.p2);
            s.store_scalar(1538, p.p6);
            s.store_scalar(1539, 1.0);
            s.store_scalar(1540, 0.0);
            s.store_scalar(1541, 0.0);
            s.store_scalar(1542, 0.0);
            s.store_scalar(1543, 0.0);
            s.store_scalar(1544, 0.0);
            s.store_scalar(1545, 0.0);
            s.store_scalar(1546, 0.0);
            s.store_scalar(1547, 0.0);
            s.store_scalar(1548, 0.0);
            s.store_scalar(1549, 0.0);
            s.store_scalar(1550, 0.0);
            s.store_scalar(1551, 0.0);
            s.store_scalar(1552, 0.0);
            s.store_scalar(1553, 0.0);
            s.store_scalar(1555, 0.0);
            s.store_scalar(1561, 0.0);
            s.store_scalar(1562, 0.0);
            s.store_scalar(1563, 0.0);
            s.store_scalar(1564, 0.0);
            s.store_scalar(1568, 0.0);
            s.store_scalar(1569, 0.0);
            s.store_scalar(1570, 0.0);
            s.store_scalar(1571, 0.0);
            s.store_scalar(1572, 0.0);
            s.store_scalar(1573, 0.0);
            s.store_scalar(1574, 0.0);
            s.store_scalar(1575, 0.0);
            s.store_scalar(1576, 0.0);
            s.store_scalar(1577, 0.0);
            s.store_scalar(1578, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1496] {
            s.store_scalar(1579, 0.0);
            s.store_scalar(1580, 0.0);
            s.store_scalar(1600, 0.0);
            s.store_scalar(1601, 0.0);
            s.store_scalar(1602, 0.0);
            s.store_scalar(1603, 0.0);
            s.store_scalar(1604, 0.0);
            s.store_scalar(1605, 0.0);
            s.store_scalar(1606, 0.0);
        }

        if s.b[1496] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(1605, 1507, A::tanh_scaled_input(s.ad_value(1507), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(1605, 1507, p.p53);
                } else {
                    s.store_scalar(1605, 0.0);
                }
            }
        }

        if s.b[1496] {
            s.store_sub(1606, 1506, 1507);
            s.store_mul(1540, 1526, 1514);
            s.store_add_scaled_product_value_ad(1542, A::div_scaled_inputs(s.ad_value(1522), 1.0, s.ad_value(1514), 2.302585092994046), 1.0, 1525, 1605, 1.0);
            s.store_add_scaled_product_right_ad(1543, 1521, 1.0, 1532, A::sub(s.ad_value(1512), s.ad_value(1513)), 1.0);
            s.store_pow_ad(1561, A::div(s.ad_value(1512), s.ad_value(1513)), s.ad_value(1534));
        }

        s.b[1609] = (s.v[1533] != 0.0);
        s.v[1609] = if s.b[1609] { 1.0 } else { 0.0 };

        if (s.b[1496] && s.b[1609]) {
            s.store_div_ad_rhs(1544, 1605, A::pow(A::offset(A::pow(A::div(s.ad_value(1605), s.ad_value(1533)), s.ad_value(1529)), 1.0), A::div_from_scalar(1.0, s.ad_value(1529))));
        }

        if (s.b[1496] && (!s.b[1609])) {
            s.store_scalar(1544, 0.0);
        }

        if s.b[1496] {
            s.store_mul_add_scaled_product_rhs(1541, 1605, s.ad_value(1523), 1.0, s.ad_value(1544), s.ad_value(1524), (-1.0));
            s.store_sub(1504, 1543, 1541);
            s.store_scaled_mul(1546, 1542, 1514, 2.0);
            s.store_mul(1547, 1517, 1546);
            s.store_sub_scaled_inputs(1604, 1504, 1.0, 1540, (p.p51 * 0.5));
        }

        if s.b[1496] {
            let assign25740_ad_e23911: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1506), 0.5, s.ad_value(1606), 0.5, A::sub(s.ad_value(1506), s.ad_value(1606)), A::tanh_scaled_input(A::sub(s.ad_value(1506), s.ad_value(1606)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1506), 0.5, s.ad_value(1606), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1506), s.ad_value(1606)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2_mixed_aii(1603, assign25740_ad_e23911, 1.0, 1604, (-1.0), 1540, 1.0);
        }

        s.b[1610] = (s.v[1603] > 50.0);
        s.v[1610] = if s.b[1610] { 1.0 } else { 0.0 };

        if (s.b[1496] && s.b[1610]) {
            s.store_scalar(1562, 0.0);
        }

        s.b[1611] = (s.v[1603] < (-50.0));
        s.v[1611] = if s.b[1611] { 1.0 } else { 0.0 };

        if ((s.b[1496] && (!s.b[1610])) && s.b[1611]) {
            s.store_scalar(1562, 1.0);
        }

        if ((s.b[1496] && (!s.b[1610])) && (!s.b[1611])) {
            s.store_div_from_scalar_offset_ad(1562, 1.0, A::exp(s.ad_value(1603)), 1.0);
        }

        if s.b[1496] {
            let assign25800_ad_e23999: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1506), 0.5, s.ad_value(1606), 0.5, A::sub(s.ad_value(1506), s.ad_value(1606)), A::tanh_scaled_input(A::sub(s.ad_value(1506), s.ad_value(1606)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1506), 0.5, s.ad_value(1606), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1506), s.ad_value(1606)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2_mixed_aai(1563, assign25800_ad_e23999, 1.0, A::add_scaled_product(s.ad_value(1504), 1.0, s.ad_value(1540), s.ad_value(1562), (-(p.p51 * 0.1))), (-1.0), 1546, 1.0);
        }

        s.b[1612] = (s.v[1563] > 50.0);
        s.v[1612] = if s.b[1612] { 1.0 } else { 0.0 };

        if (s.b[1496] && s.b[1612]) {
            s.store_mul(1564, 1547, 1563);
        }

        s.b[1613] = (s.v[1563] < (-50.0));
        s.v[1613] = if s.b[1613] { 1.0 } else { 0.0 };

        if ((s.b[1496] && (!s.b[1612])) && s.b[1613]) {
            s.store_mul_exp_rhs(1564, 1547, 1563);
        }

        if ((s.b[1496] && (!s.b[1612])) && (!s.b[1613])) {
            s.store_mul_ln_one_plus_exp_rhs(1564, 1547, 1563);
        }

        if s.b[1496] {
            s.store_div_ad_rhs(1550, 1528, A::mul_offset_rhs(s.ad_value(1561), A::div_scaled_product(s.ad_value(1530), s.ad_value(1564), 1.0, s.ad_value(1517), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(1551, 1527, A::div_scaled_offset_numerator(A::mul(s.ad_value(1535), s.ad_value(1513)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1535), s.ad_value(1512)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(1536), s.ad_value(1605), 1.0, s.ad_value(1516), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(1531), s.ad_value(1564), 1.0, s.ad_value(1517), 1.0), 1.0), 1.0);
            s.store_add_ad(1552, A::div_scaled_product3(s.ad_value(1562), s.ad_value(1514), s.ad_value(1550), 2.0, s.ad_value(1516), 1.0), A::mul_sub_from_scalar_lhs(1.0, s.ad_value(1562), s.ad_value(1551)));
            s.store_div_scaled_product_indices(1568, 1551, 1516, 1.0, 1550, 1.0);
            s.store_add_scaled_product_right_ad(1569, 1568, (-1.0), 1568, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1564), 2.0, s.ad_value(1517), s.ad_value(1568), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(1570, A::mul_sub_from_scalar_rhs(s.ad_value(1568), 1.0, s.ad_value(1562)), 1.0, 1546, 1562, 1.0);
            s.store_add_scaled_product_value_ad(1505, A::mul_sub_from_scalar_rhs(s.ad_value(1569), 1.0, s.ad_value(1562)), 1.0, 1546, 1562, 1.0);
        }

        if s.b[1496] {
            let assign25930_ad_e24228: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1507), s.ad_value(1505)), 0.5, A::div(s.ad_value(1507), s.ad_value(1505)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1507), s.ad_value(1505))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1507), s.ad_value(1505)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(1507), s.ad_value(1505)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(1571, 1.0, A::offset(A::pow(assign25930_ad_e24228, s.ad_value(1529)), 1.0), A::div_from_scalar(1.0, s.ad_value(1529)));
        }

        if s.b[1496] {
            s.store_mul(1572, 1507, 1571);
        }

        if s.b[1496] {
            let assign25950_ad_e24309: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1507), -1.0, s.ad_value(1505), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1507), -1.0, s.ad_value(1505), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1507), -1.0, s.ad_value(1505), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1507), -1.0, s.ad_value(1505), 1.0), 0.5, A::sqrt_square_offset(A::div_scaled_inputs(s.ad_value(1507), -1.0, s.ad_value(1505), 1.0), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(1573, 1.0, A::offset(A::pow(assign25950_ad_e24309, s.ad_value(1529)), 1.0), A::div_from_scalar(1.0, s.ad_value(1529)));
        }

        if s.b[1496] {
            s.store_mul_neg_lhs(1574, 1507, 1573);
            s.store_div_scaled_inputs2_indices(1603, 1506, 1.0, 1604, (-1.0), 1540, 1.0);
        }

        s.b[1614] = (s.v[1603] > 50.0);
        s.v[1614] = if s.b[1614] { 1.0 } else { 0.0 };

        if (s.b[1496] && s.b[1614]) {
            s.store_scalar(1545, 0.0);
        }

        s.b[1615] = (s.v[1603] < (-50.0));
        s.v[1615] = if s.b[1615] { 1.0 } else { 0.0 };

        if ((s.b[1496] && (!s.b[1614])) && s.b[1615]) {
            s.store_scalar(1545, 1.0);
        }

        if ((s.b[1496] && (!s.b[1614])) && (!s.b[1615])) {
            s.store_div_from_scalar_offset_ad(1545, 1.0, A::exp(s.ad_value(1603)), 1.0);
        }

        if s.b[1496] {
            s.store_div_scaled_inputs3_mixed_iiai(1548, 1606, 1.0, 1574, (-1.0), A::add_scaled_product(s.ad_value(1504), 1.0, s.ad_value(1540), s.ad_value(1545), (-(p.p51 * 0.1))), -1.0, 1546, 1.0);
        }

        s.b[1616] = (s.v[1548] > 50.0);
        s.v[1616] = if s.b[1616] { 1.0 } else { 0.0 };

        if (s.b[1496] && s.b[1616]) {
            s.store_mul(1549, 1547, 1548);
        }

        s.b[1617] = (s.v[1548] < (-50.0));
        s.v[1617] = if s.b[1617] { 1.0 } else { 0.0 };

        if ((s.b[1496] && (!s.b[1616])) && s.b[1617]) {
            s.store_mul_exp_rhs(1549, 1547, 1548);
        }

        if ((s.b[1496] && (!s.b[1616])) && (!s.b[1617])) {
            s.store_mul_ln_one_plus_exp_rhs(1549, 1547, 1548);
        }

        if s.b[1496] {
            s.store_div_scaled_inputs2_indices(1603, 1606, 1.0, 1604, (-1.0), 1540, 1.0);
        }

        s.b[1618] = (s.v[1603] > 50.0);
        s.v[1618] = if s.b[1618] { 1.0 } else { 0.0 };

        if (s.b[1496] && s.b[1618]) {
            s.store_scalar(1575, 0.0);
        }

        s.b[1619] = (s.v[1603] < (-50.0));
        s.v[1619] = if s.b[1619] { 1.0 } else { 0.0 };

        if ((s.b[1496] && (!s.b[1618])) && s.b[1619]) {
            s.store_scalar(1575, 1.0);
        }

        if ((s.b[1496] && (!s.b[1618])) && (!s.b[1619])) {
            s.store_div_from_scalar_offset_ad(1575, 1.0, A::exp(s.ad_value(1603)), 1.0);
        }

        if s.b[1496] {
            s.store_div_scaled_inputs3_mixed_iiai(1576, 1506, 1.0, 1572, (-1.0), A::add_scaled_product(s.ad_value(1504), 1.0, s.ad_value(1540), s.ad_value(1575), (-(p.p51 * 0.1))), -1.0, 1546, 1.0);
        }

        s.b[1620] = (s.v[1576] > 50.0);
        s.v[1620] = if s.b[1620] { 1.0 } else { 0.0 };

        if (s.b[1496] && s.b[1620]) {
            s.store_mul(1577, 1547, 1576);
        }

        s.b[1621] = (s.v[1576] < (-50.0));
        s.v[1621] = if s.b[1621] { 1.0 } else { 0.0 };

        if ((s.b[1496] && (!s.b[1620])) && s.b[1621]) {
            s.store_mul_exp_rhs(1577, 1547, 1576);
        }

        if ((s.b[1496] && (!s.b[1620])) && (!s.b[1621])) {
            s.store_mul_ln_one_plus_exp_rhs(1577, 1547, 1576);
        }

        if s.b[1496] {
            s.store_div_scaled_inputs2_indices(1578, 1549, 1.0, 1577, (-1.0), 1517, 1.0);
            s.store_div(1604, 1578, 1570);
        }

        if s.b[1496] {
            let assign26230_ad_e24586: A = A::pow(A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::mul(s.ad_value(1604), A::tanh_scaled_input(s.ad_value(1604), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt_square_offset(s.ad_value(1604), p.p53)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1529)), 1.0), A::div_from_scalar(1.0, s.ad_value(1529)));
            s.store_div_ad_rhs(1579, 1604, assign26230_ad_e24586);
        }

        if s.b[1496] {
            s.store_mul(1580, 1552, 1579);
            s.store_mul_product3_rhs(1498, 1539, A::mul3(s.ad_value(1538), s.ad_value(1515), s.ad_value(1537)), A::add(s.ad_value(1549), s.ad_value(1577)), s.ad_value(1580), 0.5);
            s.store_div_scaled_inputs_indices(1553, 1522, 1.0, 1514, 2.302585092994046);
            s.store_scaled_mul(1555, 1553, 1514, 2.0);
        }

        s.b[1634] = (s.v[1508] == 1.0);
        s.v[1634] = if s.b[1634] { 1.0 } else { 0.0 };

        if (s.b[1496] && s.b[1634]) {
            s.store_div_scaled_inputs3_indices(1600, 1509, 1.0, 1543, -1.0, 1540, (-(-(p.p51 * 0.5))), 1555, 1.0);
        }

        s.b[1635] = (s.v[1600] > 50.0);
        s.v[1635] = if s.b[1635] { 1.0 } else { 0.0 };

        if ((s.b[1496] && s.b[1634]) && s.b[1635]) {
            s.copy_ad(1603, 1600);
        }

        s.b[1636] = (s.v[1600] < (-50.0));
        s.v[1636] = if s.b[1636] { 1.0 } else { 0.0 };

        if (((s.b[1496] && s.b[1634]) && (!s.b[1635])) && s.b[1636]) {
            s.store_exp(1603, 1600);
        }

        if (((s.b[1496] && s.b[1634]) && (!s.b[1635])) && (!s.b[1636])) {
            s.store_ln_one_plus_exp(1603, 1600);
        }

        if (s.b[1496] && s.b[1634]) {
            s.store_div_scaled_inputs3_indices(1601, 1510, 1.0, 1543, -1.0, 1540, (-(-(p.p51 * 0.5))), 1555, 1.0);
        }

        s.b[1637] = (s.v[1601] > 50.0);
        s.v[1637] = if s.b[1637] { 1.0 } else { 0.0 };

        if ((s.b[1496] && s.b[1634]) && s.b[1637]) {
            s.copy_ad(1603, 1601);
        }

        s.b[1638] = (s.v[1601] < (-50.0));
        s.v[1638] = if s.b[1638] { 1.0 } else { 0.0 };

        if (((s.b[1496] && s.b[1634]) && (!s.b[1637])) && s.b[1638]) {
            s.store_exp(1603, 1601);
        }

        if (((s.b[1496] && s.b[1634]) && (!s.b[1637])) && (!s.b[1638])) {
            s.store_ln_one_plus_exp(1603, 1601);
        }

        s.b[1639] = (s.v[1511] == 1.0);
        s.v[1639] = if s.b[1639] { 1.0 } else { 0.0 };

        if (s.b[1496] && s.b[1639]) {
            s.store_div_scaled_inputs3_indices(1602, 1506, 1.0, 1543, -1.0, 1540, (-(-(p.p51 * 0.5))), 1555, 1.0);
        }

        s.b[1640] = (s.v[1602] > 50.0);
        s.v[1640] = if s.b[1640] { 1.0 } else { 0.0 };

        if ((s.b[1496] && s.b[1639]) && s.b[1640]) {
            s.copy_ad(1603, 1602);
        }

        s.b[1641] = (s.v[1602] < (-50.0));
        s.v[1641] = if s.b[1641] { 1.0 } else { 0.0 };

        if (((s.b[1496] && s.b[1639]) && (!s.b[1640])) && s.b[1641]) {
            s.store_exp(1603, 1602);
        }

        if (((s.b[1496] && s.b[1639]) && (!s.b[1640])) && (!s.b[1641])) {
            s.store_ln_one_plus_exp(1603, 1602);
        }

        if s.b[1496] {
            s.copy_ad(1497, 1498);
            s.copy_ad(154, 1498);
            s.copy_ad(154, 1497);
        }

        s.v[160] = 0.0;

        s.b[1642] = ((p.p50 == 0.0) && (p.p66 > p.p354));
        s.v[1642] = if s.b[1642] { 1.0 } else { 0.0 };

        if s.b[1642] {
            s.store_scalar(1643, 0.0);
            s.store_scalar(1644, 0.0);
            s.store_scalar(1650, 0.0);
            s.store_scalar(1651, 0.0);
            s.copy_ad(1652, 56);
            s.copy_ad(1653, 57);
        }

        let (assign27420_e25754,) = {
    if s.b[1642] {
        (0.0,)
    } else {
        (s.v[1654],)
    }
};
        s.v[1654] = assign27420_e25754;

        if s.b[1642] {
            s.store_scalar(1655, 0.0);
            s.store_scalar(1656, 0.0);
        }

        let (assign27450_e25766,) = {
    if s.b[1642] {
        (0.0,)
    } else {
        (s.v[1657],)
    }
};
        s.v[1657] = assign27450_e25766;

    }

    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1642] {
            s.copy_ad(1658, 111);
            s.store_scalar(1659, s.v[109]);
            s.copy_ad(1660, 113);
            s.store_scalar(1661, p.p0);
            s.store_scalar(1662, p.p66);
            s.store_scalar(1663, p.p68);
            s.store_scalar(1667, p.p67);
            s.store_scalar(1668, p.p73);
            s.store_scalar(1669, p.p72);
            s.store_scalar(1670, 0.0);
            s.store_scalar(1671, p.p74);
            s.store_scalar(1672, p.p77);
            s.store_scalar(1673, p.p69);
            s.store_scalar(1674, p.p70);
            s.store_scalar(1675, p.p71);
            s.store_scalar(1676, p.p76);
            s.store_scalar(1677, p.p75);
            s.store_scalar(1678, p.p46);
            s.store_scalar(1679, p.p39);
            s.store_scalar(1680, p.p47);
            s.store_scalar(1681, p.p45);
            s.store_scalar(1682, p.p42);
            s.store_scalar(1683, p.p2);
            s.store_scalar(1684, p.p6);
            s.store_scalar(1685, 1.0);
            s.store_scalar(1686, 0.0);
            s.store_scalar(1687, 0.0);
            s.store_scalar(1688, 0.0);
            s.store_scalar(1689, 0.0);
            s.store_scalar(1690, 0.0);
            s.store_scalar(1691, 0.0);
            s.store_scalar(1692, 0.0);
            s.store_scalar(1693, 0.0);
            s.store_scalar(1694, 0.0);
            s.store_scalar(1695, 0.0);
            s.store_scalar(1696, 0.0);
            s.store_scalar(1697, 0.0);
            s.store_scalar(1698, 0.0);
            s.store_scalar(1699, 0.0);
            s.store_scalar(1701, 0.0);
            s.store_scalar(1707, 0.0);
            s.store_scalar(1708, 0.0);
            s.store_scalar(1709, 0.0);
            s.store_scalar(1710, 0.0);
            s.store_scalar(1714, 0.0);
            s.store_scalar(1715, 0.0);
            s.store_scalar(1716, 0.0);
            s.store_scalar(1717, 0.0);
            s.store_scalar(1718, 0.0);
            s.store_scalar(1719, 0.0);
            s.store_scalar(1720, 0.0);
            s.store_scalar(1721, 0.0);
            s.store_scalar(1722, 0.0);
            s.store_scalar(1723, 0.0);
            s.store_scalar(1724, 0.0);
            s.store_scalar(1725, 0.0);
            s.store_scalar(1726, 0.0);
            s.store_scalar(1746, 0.0);
            s.store_scalar(1747, 0.0);
            s.store_scalar(1748, 0.0);
            s.store_scalar(1749, 0.0);
            s.store_scalar(1750, 0.0);
            s.store_scalar(1751, 0.0);
            s.store_scalar(1752, 0.0);
        }

        if s.b[1642] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(1751, 1653, A::tanh_scaled_input(s.ad_value(1653), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(1751, 1653, p.p53);
                } else {
                    s.store_scalar(1751, 0.0);
                }
            }
        }

        if s.b[1642] {
            s.store_sub(1752, 1652, 1653);
            s.store_mul(1686, 1672, 1660);
            s.store_add_scaled_product_value_ad(1688, A::div_scaled_inputs(s.ad_value(1668), 1.0, s.ad_value(1660), 2.302585092994046), 1.0, 1671, 1751, 1.0);
            s.store_add_scaled_product_right_ad(1689, 1667, 1.0, 1678, A::sub(s.ad_value(1658), s.ad_value(1659)), 1.0);
            s.store_pow_ad(1707, A::div(s.ad_value(1658), s.ad_value(1659)), s.ad_value(1680));
        }

        s.b[1755] = (s.v[1679] != 0.0);
        s.v[1755] = if s.b[1755] { 1.0 } else { 0.0 };

        if (s.b[1642] && s.b[1755]) {
            s.store_div_ad_rhs(1690, 1751, A::pow(A::offset(A::pow(A::div(s.ad_value(1751), s.ad_value(1679)), s.ad_value(1675)), 1.0), A::div_from_scalar(1.0, s.ad_value(1675))));
        }

        if (s.b[1642] && (!s.b[1755])) {
            s.store_scalar(1690, 0.0);
        }

        if s.b[1642] {
            s.store_mul_add_scaled_product_rhs(1687, 1751, s.ad_value(1669), 1.0, s.ad_value(1690), s.ad_value(1670), (-1.0));
            s.store_sub(1650, 1689, 1687);
            s.store_scaled_mul(1692, 1688, 1660, 2.0);
            s.store_mul(1693, 1663, 1692);
            s.store_sub_scaled_inputs(1750, 1650, 1.0, 1686, (p.p51 * 0.5));
        }

        if s.b[1642] {
            let assign28570_ad_e26336: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1652), 0.5, s.ad_value(1752), 0.5, A::sub(s.ad_value(1652), s.ad_value(1752)), A::tanh_scaled_input(A::sub(s.ad_value(1652), s.ad_value(1752)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1652), 0.5, s.ad_value(1752), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1652), s.ad_value(1752)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2_mixed_aii(1749, assign28570_ad_e26336, 1.0, 1750, (-1.0), 1686, 1.0);
        }

        s.b[1756] = (s.v[1749] > 50.0);
        s.v[1756] = if s.b[1756] { 1.0 } else { 0.0 };

        if (s.b[1642] && s.b[1756]) {
            s.store_scalar(1708, 0.0);
        }

        s.b[1757] = (s.v[1749] < (-50.0));
        s.v[1757] = if s.b[1757] { 1.0 } else { 0.0 };

        if ((s.b[1642] && (!s.b[1756])) && s.b[1757]) {
            s.store_scalar(1708, 1.0);
        }

        if ((s.b[1642] && (!s.b[1756])) && (!s.b[1757])) {
            s.store_div_from_scalar_offset_ad(1708, 1.0, A::exp(s.ad_value(1749)), 1.0);
        }

        if s.b[1642] {
            let assign28630_ad_e26424: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1652), 0.5, s.ad_value(1752), 0.5, A::sub(s.ad_value(1652), s.ad_value(1752)), A::tanh_scaled_input(A::sub(s.ad_value(1652), s.ad_value(1752)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1652), 0.5, s.ad_value(1752), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1652), s.ad_value(1752)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2_mixed_aai(1709, assign28630_ad_e26424, 1.0, A::add_scaled_product(s.ad_value(1650), 1.0, s.ad_value(1686), s.ad_value(1708), (-(p.p51 * 0.1))), (-1.0), 1692, 1.0);
        }

        s.b[1758] = (s.v[1709] > 50.0);
        s.v[1758] = if s.b[1758] { 1.0 } else { 0.0 };

        if (s.b[1642] && s.b[1758]) {
            s.store_mul(1710, 1693, 1709);
        }

        s.b[1759] = (s.v[1709] < (-50.0));
        s.v[1759] = if s.b[1759] { 1.0 } else { 0.0 };

        if ((s.b[1642] && (!s.b[1758])) && s.b[1759]) {
            s.store_mul_exp_rhs(1710, 1693, 1709);
        }

        if ((s.b[1642] && (!s.b[1758])) && (!s.b[1759])) {
            s.store_mul_ln_one_plus_exp_rhs(1710, 1693, 1709);
        }

        if s.b[1642] {
            s.store_div_ad_rhs(1696, 1674, A::mul_offset_rhs(s.ad_value(1707), A::div_scaled_product(s.ad_value(1676), s.ad_value(1710), 1.0, s.ad_value(1663), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(1697, 1673, A::div_scaled_offset_numerator(A::mul(s.ad_value(1681), s.ad_value(1659)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1681), s.ad_value(1658)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(1682), s.ad_value(1751), 1.0, s.ad_value(1662), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(1677), s.ad_value(1710), 1.0, s.ad_value(1663), 1.0), 1.0), 1.0);
            s.store_add_ad(1698, A::div_scaled_product3(s.ad_value(1708), s.ad_value(1660), s.ad_value(1696), 2.0, s.ad_value(1662), 1.0), A::mul_sub_from_scalar_lhs(1.0, s.ad_value(1708), s.ad_value(1697)));
            s.store_div_scaled_product_indices(1714, 1697, 1662, 1.0, 1696, 1.0);
            s.store_add_scaled_product_right_ad(1715, 1714, (-1.0), 1714, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1710), 2.0, s.ad_value(1663), s.ad_value(1714), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(1716, A::mul_sub_from_scalar_rhs(s.ad_value(1714), 1.0, s.ad_value(1708)), 1.0, 1692, 1708, 1.0);
            s.store_add_scaled_product_value_ad(1651, A::mul_sub_from_scalar_rhs(s.ad_value(1715), 1.0, s.ad_value(1708)), 1.0, 1692, 1708, 1.0);
        }

        if s.b[1642] {
            let assign28760_ad_e26653: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1653), s.ad_value(1651)), 0.5, A::div(s.ad_value(1653), s.ad_value(1651)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1653), s.ad_value(1651))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1653), s.ad_value(1651)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(1653), s.ad_value(1651)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(1717, 1.0, A::offset(A::pow(assign28760_ad_e26653, s.ad_value(1675)), 1.0), A::div_from_scalar(1.0, s.ad_value(1675)));
        }

        if s.b[1642] {
            s.store_mul(1718, 1653, 1717);
        }

        if s.b[1642] {
            let assign28780_ad_e26734: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1653), -1.0, s.ad_value(1651), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1653), -1.0, s.ad_value(1651), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1653), -1.0, s.ad_value(1651), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1653), -1.0, s.ad_value(1651), 1.0), 0.5, A::sqrt_square_offset(A::div_scaled_inputs(s.ad_value(1653), -1.0, s.ad_value(1651), 1.0), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(1719, 1.0, A::offset(A::pow(assign28780_ad_e26734, s.ad_value(1675)), 1.0), A::div_from_scalar(1.0, s.ad_value(1675)));
        }

        if s.b[1642] {
            s.store_mul_neg_lhs(1720, 1653, 1719);
            s.store_div_scaled_inputs2_indices(1749, 1652, 1.0, 1750, (-1.0), 1686, 1.0);
        }

        s.b[1760] = (s.v[1749] > 50.0);
        s.v[1760] = if s.b[1760] { 1.0 } else { 0.0 };

        if (s.b[1642] && s.b[1760]) {
            s.store_scalar(1691, 0.0);
        }

        s.b[1761] = (s.v[1749] < (-50.0));
        s.v[1761] = if s.b[1761] { 1.0 } else { 0.0 };

        if ((s.b[1642] && (!s.b[1760])) && s.b[1761]) {
            s.store_scalar(1691, 1.0);
        }

        if ((s.b[1642] && (!s.b[1760])) && (!s.b[1761])) {
            s.store_div_from_scalar_offset_ad(1691, 1.0, A::exp(s.ad_value(1749)), 1.0);
        }

        if s.b[1642] {
            s.store_div_scaled_inputs3_mixed_iiai(1694, 1752, 1.0, 1720, (-1.0), A::add_scaled_product(s.ad_value(1650), 1.0, s.ad_value(1686), s.ad_value(1691), (-(p.p51 * 0.1))), -1.0, 1692, 1.0);
        }

        s.b[1762] = (s.v[1694] > 50.0);
        s.v[1762] = if s.b[1762] { 1.0 } else { 0.0 };

        if (s.b[1642] && s.b[1762]) {
            s.store_mul(1695, 1693, 1694);
        }

        s.b[1763] = (s.v[1694] < (-50.0));
        s.v[1763] = if s.b[1763] { 1.0 } else { 0.0 };

        if ((s.b[1642] && (!s.b[1762])) && s.b[1763]) {
            s.store_mul_exp_rhs(1695, 1693, 1694);
        }

        if ((s.b[1642] && (!s.b[1762])) && (!s.b[1763])) {
            s.store_mul_ln_one_plus_exp_rhs(1695, 1693, 1694);
        }

        if s.b[1642] {
            s.store_div_scaled_inputs2_indices(1749, 1752, 1.0, 1750, (-1.0), 1686, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1764] = (s.v[1749] > 50.0);
        s.v[1764] = if s.b[1764] { 1.0 } else { 0.0 };

        if (s.b[1642] && s.b[1764]) {
            s.store_scalar(1721, 0.0);
        }

        s.b[1765] = (s.v[1749] < (-50.0));
        s.v[1765] = if s.b[1765] { 1.0 } else { 0.0 };

        if ((s.b[1642] && (!s.b[1764])) && s.b[1765]) {
            s.store_scalar(1721, 1.0);
        }

        if ((s.b[1642] && (!s.b[1764])) && (!s.b[1765])) {
            s.store_div_from_scalar_offset_ad(1721, 1.0, A::exp(s.ad_value(1749)), 1.0);
        }

        if s.b[1642] {
            s.store_div_scaled_inputs3_mixed_iiai(1722, 1652, 1.0, 1718, (-1.0), A::add_scaled_product(s.ad_value(1650), 1.0, s.ad_value(1686), s.ad_value(1721), (-(p.p51 * 0.1))), -1.0, 1692, 1.0);
        }

        s.b[1766] = (s.v[1722] > 50.0);
        s.v[1766] = if s.b[1766] { 1.0 } else { 0.0 };

        if (s.b[1642] && s.b[1766]) {
            s.store_mul(1723, 1693, 1722);
        }

        s.b[1767] = (s.v[1722] < (-50.0));
        s.v[1767] = if s.b[1767] { 1.0 } else { 0.0 };

        if ((s.b[1642] && (!s.b[1766])) && s.b[1767]) {
            s.store_mul_exp_rhs(1723, 1693, 1722);
        }

        if ((s.b[1642] && (!s.b[1766])) && (!s.b[1767])) {
            s.store_mul_ln_one_plus_exp_rhs(1723, 1693, 1722);
        }

        if s.b[1642] {
            s.store_div_scaled_inputs2_indices(1724, 1695, 1.0, 1723, (-1.0), 1663, 1.0);
            s.store_div(1750, 1724, 1716);
        }

        if s.b[1642] {
            let assign29060_ad_e27011: A = A::pow(A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::mul(s.ad_value(1750), A::tanh_scaled_input(s.ad_value(1750), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt_square_offset(s.ad_value(1750), p.p53)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1675)), 1.0), A::div_from_scalar(1.0, s.ad_value(1675)));
            s.store_div_ad_rhs(1725, 1750, assign29060_ad_e27011);
        }

        if s.b[1642] {
            s.store_mul(1726, 1698, 1725);
            s.store_mul_product3_rhs(1644, 1685, A::mul3(s.ad_value(1684), s.ad_value(1661), s.ad_value(1683)), A::add(s.ad_value(1695), s.ad_value(1723)), s.ad_value(1726), 0.5);
            s.store_div_scaled_inputs_indices(1699, 1668, 1.0, 1660, 2.302585092994046);
            s.store_scaled_mul(1701, 1699, 1660, 2.0);
        }

        s.b[1780] = (s.v[1654] == 1.0);
        s.v[1780] = if s.b[1780] { 1.0 } else { 0.0 };

        if (s.b[1642] && s.b[1780]) {
            s.store_div_scaled_inputs3_indices(1746, 1655, 1.0, 1689, -1.0, 1686, (-(-(p.p51 * 0.5))), 1701, 1.0);
        }

        s.b[1781] = (s.v[1746] > 50.0);
        s.v[1781] = if s.b[1781] { 1.0 } else { 0.0 };

        if ((s.b[1642] && s.b[1780]) && s.b[1781]) {
            s.copy_ad(1749, 1746);
        }

        s.b[1782] = (s.v[1746] < (-50.0));
        s.v[1782] = if s.b[1782] { 1.0 } else { 0.0 };

        if (((s.b[1642] && s.b[1780]) && (!s.b[1781])) && s.b[1782]) {
            s.store_exp(1749, 1746);
        }

        if (((s.b[1642] && s.b[1780]) && (!s.b[1781])) && (!s.b[1782])) {
            s.store_ln_one_plus_exp(1749, 1746);
        }

        if (s.b[1642] && s.b[1780]) {
            s.store_div_scaled_inputs3_indices(1747, 1656, 1.0, 1689, -1.0, 1686, (-(-(p.p51 * 0.5))), 1701, 1.0);
        }

        s.b[1783] = (s.v[1747] > 50.0);
        s.v[1783] = if s.b[1783] { 1.0 } else { 0.0 };

        if ((s.b[1642] && s.b[1780]) && s.b[1783]) {
            s.copy_ad(1749, 1747);
        }

        s.b[1784] = (s.v[1747] < (-50.0));
        s.v[1784] = if s.b[1784] { 1.0 } else { 0.0 };

        if (((s.b[1642] && s.b[1780]) && (!s.b[1783])) && s.b[1784]) {
            s.store_exp(1749, 1747);
        }

        if (((s.b[1642] && s.b[1780]) && (!s.b[1783])) && (!s.b[1784])) {
            s.store_ln_one_plus_exp(1749, 1747);
        }

        s.b[1785] = (s.v[1657] == 1.0);
        s.v[1785] = if s.b[1785] { 1.0 } else { 0.0 };

        if (s.b[1642] && s.b[1785]) {
            s.store_div_scaled_inputs3_indices(1748, 1652, 1.0, 1689, -1.0, 1686, (-(-(p.p51 * 0.5))), 1701, 1.0);
        }

        s.b[1786] = (s.v[1748] > 50.0);
        s.v[1786] = if s.b[1786] { 1.0 } else { 0.0 };

        if ((s.b[1642] && s.b[1785]) && s.b[1786]) {
            s.copy_ad(1749, 1748);
        }

        s.b[1787] = (s.v[1748] < (-50.0));
        s.v[1787] = if s.b[1787] { 1.0 } else { 0.0 };

        if (((s.b[1642] && s.b[1785]) && (!s.b[1786])) && s.b[1787]) {
            s.store_exp(1749, 1748);
        }

        if (((s.b[1642] && s.b[1785]) && (!s.b[1786])) && (!s.b[1787])) {
            s.store_ln_one_plus_exp(1749, 1748);
        }

        if s.b[1642] {
            s.copy_ad(1643, 1644);
            s.copy_ad(160, 1644);
            s.copy_ad(160, 1643);
        }

        s.v[1788] = 0.0;

        s.v[1789] = 0.0;

        s.v[1790] = 0.0;

        s.v[1791] = 0.0;

        s.v[1795] = 0.0;

        s.v[1796] = 0.0;

        s.copy_ad(1797, 45);

        s.copy_ad(1798, 44);

        s.v[1799] = 0.0;

        s.v[1800] = 0.0;

        s.v[1801] = 0.0;

        s.v[1802] = 0.0;

        s.copy_ad(1803, 111);

        s.v[1804] = s.v[109];

        s.copy_ad(1805, 113);

        s.v[1806] = p.p0;

        s.v[1807] = p.p1;

        s.copy_ad(1808, 19);

        s.v[1812] = p.p35;

        s.v[1813] = p.p36;

        s.v[1814] = p.p37;

        s.v[1815] = p.p38;

        s.v[1816] = p.p40;

        s.v[1817] = p.p41;

        s.v[1818] = p.p32;

        s.v[1819] = p.p33;

        s.v[1820] = p.p34;

        s.v[1821] = p.p44;

        s.v[1822] = p.p43;

        s.v[1823] = p.p46;

        s.v[1824] = p.p39;

        s.v[1825] = p.p47;

        s.v[1826] = p.p45;

        s.v[1827] = p.p42;

        s.v[1828] = p.p2;

        s.v[1829] = p.p6;

        s.copy_ad(1830, 230);

        s.v[1831] = 0.0;

        s.v[1832] = 0.0;

        s.v[1833] = 0.0;

        s.v[1834] = 0.0;

        s.v[1835] = 0.0;

        s.v[1836] = 0.0;

        s.v[1837] = 0.0;

        s.v[1838] = 0.0;

        s.v[1839] = 0.0;

        s.v[1840] = 0.0;

        s.v[1841] = 0.0;

        s.v[1842] = 0.0;

        s.v[1843] = 0.0;

        s.v[1844] = 0.0;

        s.v[1845] = 0.0;

        s.v[1846] = 0.0;

        s.v[1847] = 0.0;

        s.v[1848] = 0.0;

        s.v[1849] = 0.0;

        s.v[1850] = 0.0;

        s.v[1851] = 0.0;

        s.v[1852] = 0.0;

        s.v[1853] = 0.0;

        s.v[1854] = 0.0;

        s.v[1855] = 0.0;

        s.v[1856] = 0.0;

        s.v[1857] = 0.0;

        s.v[1858] = 0.0;

        s.v[1859] = 0.0;

        s.v[1860] = 0.0;

        s.v[1861] = 0.0;

        s.v[1862] = 0.0;

        s.v[1863] = 0.0;

        s.v[1864] = 0.0;

        s.v[1865] = 0.0;

        s.v[1866] = 0.0;

        s.v[1867] = 0.0;

        s.v[1868] = 0.0;

        s.v[1869] = 0.0;

        s.v[1870] = 0.0;

        s.v[1871] = 0.0;

        s.v[1872] = 0.0;

        s.v[1873] = 0.0;

        s.v[1874] = 0.0;

        s.v[1875] = 0.0;

        s.v[1876] = 0.0;

        s.v[1877] = 0.0;

        s.v[1878] = 0.0;

        s.v[1879] = 0.0;

        s.v[1880] = 0.0;

        s.v[1881] = 0.0;

        s.v[1882] = 0.0;

        s.v[1883] = 0.0;

        s.v[1884] = 0.0;

        s.v[1885] = 0.0;

        s.v[1886] = 0.0;

        s.v[1887] = 0.0;

        s.v[1888] = 0.0;

        s.v[1889] = 0.0;

        s.v[1890] = 0.0;

        s.v[1891] = 0.0;

        s.v[1892] = 0.0;

        s.v[1893] = 0.0;

        s.v[1894] = 0.0;

        s.v[1895] = 0.0;

        s.v[1896] = 0.0;

        s.v[1897] = 0.0;

        s.v[1898] = 0.0;

        s.v[1899] = 0.0;

        if (p.p52 != 0.0) {
            s.store_mul_ad_rhs(1896, 1798, A::tanh_scaled_input(s.ad_value(1798), (0.001 / p.p53)));
        } else {
            if (p.p52 == 0.0) {
                s.store_sqrt_square_offset(1896, 1798, p.p53);
            } else {
                s.store_scalar(1896, 0.0);
            }
        }

        s.store_sub(1897, 1797, 1798);

        s.store_scale(1831, 1805, s.v[1817]);

        s.store_add_scaled_ad_lhs(1833, A::div_from_scalar(s.v[1813], A::scale(s.ad_value(1805), 2.302585092994046)), 1896, s.v[1816]);

        s.store_offset_scaled(1834, 1803, s.v[1823], (((((-s.v[1804])) * (s.v[1823]))) + (s.v[1812])));

        s.store_powf_scaled_input(1852, 1803, 1.0 / (s.v[1804]), s.v[1825]);

        s.b[1900] = (s.v[1824] != 0.0);
        s.v[1900] = if s.b[1900] { 1.0 } else { 0.0 };

        if s.b[1900] {
            s.store_div_ad_rhs(1835, 1896, A::powf(A::offset(A::powf(A::scale(s.ad_value(1896), 1.0 / (s.v[1824])), s.v[1820]), 1.0), (1.0 / s.v[1820])));
        }

        if (!s.b[1900]) {
            s.store_scalar(1835, 0.0);
        }

        s.store_mul_sub_from_scalar_ad_lhs(1832, s.v[1814], A::scale(s.ad_value(1835), s.v[1815]), 1896);

        s.store_sub(1795, 1834, 1832);

        s.store_scaled_mul(1837, 1833, 1805, 2.0);

        s.store_mul(1838, 1808, 1837);

        s.store_sub_scaled_inputs(1895, 1795, 1.0, 1831, (p.p51 * 0.5));

        let assign31310_ad_e28372: A = {
    if (p.p52 != 0.0) {
        A::add_scaled_inputs_product(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sub(s.ad_value(1797), s.ad_value(1897)), A::tanh_scaled_input(A::sub(s.ad_value(1797), s.ad_value(1897)), (0.001 / p.p53)), 0.5)
    } else {
        {
            if (p.p52 == 0.0) {
                A::add_scaled_inputs3(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1797), s.ad_value(1897)), p.p53), 0.5)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_div_scaled_inputs2_mixed_aii(1894, assign31310_ad_e28372, 1.0, 1895, (-1.0), 1831, 1.0);

        s.b[1901] = (s.v[1894] > 50.0);
        s.v[1901] = if s.b[1901] { 1.0 } else { 0.0 };

        if s.b[1901] {
            s.store_scalar(1853, 0.0);
        }

        s.b[1902] = (s.v[1894] < (-50.0));
        s.v[1902] = if s.b[1902] { 1.0 } else { 0.0 };

        if ((!s.b[1901]) && s.b[1902]) {
            s.store_scalar(1853, 1.0);
        }

        if ((!s.b[1901]) && (!s.b[1902])) {
            s.store_div_from_scalar_offset_ad(1853, 1.0, A::exp(s.ad_value(1894)), 1.0);
        }

        let assign31370_ad_e28451: A = {
    if (p.p52 != 0.0) {
        A::add_scaled_inputs_product(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sub(s.ad_value(1797), s.ad_value(1897)), A::tanh_scaled_input(A::sub(s.ad_value(1797), s.ad_value(1897)), (0.001 / p.p53)), 0.5)
    } else {
        {
            if (p.p52 == 0.0) {
                A::add_scaled_inputs3(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1797), s.ad_value(1897)), p.p53), 0.5)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_div_scaled_inputs2_mixed_aai(1854, assign31370_ad_e28451, 1.0, A::add_scaled_product(s.ad_value(1795), 1.0, s.ad_value(1831), s.ad_value(1853), (-(p.p51 * 0.1))), (-1.0), 1837, 1.0);

        s.b[1903] = (s.v[1854] > 50.0);
        s.v[1903] = if s.b[1903] { 1.0 } else { 0.0 };

        if s.b[1903] {
            s.store_mul(1855, 1838, 1854);
        }

        s.b[1904] = (s.v[1854] < (-50.0));
        s.v[1904] = if s.b[1904] { 1.0 } else { 0.0 };

        if ((!s.b[1903]) && s.b[1904]) {
            s.store_mul_exp_rhs(1855, 1838, 1854);
        }

        if ((!s.b[1903]) && (!s.b[1904])) {
            s.store_mul_ln_one_plus_exp_rhs(1855, 1838, 1854);
        }

        s.store_div_from_scalar_ad(1841, s.v[1819], A::mul_offset_rhs(s.ad_value(1852), A::div_scaled_inputs(s.ad_value(1855), s.v[1821], s.ad_value(1808), 1.0), 1.0));

        s.store_div_scaled_value_by_product(1842, A::scale_offset(s.ad_value(1896), (s.v[1827] * 1.0 / (s.v[1807])), 1.0), (s.v[1818] * (1.0 + (s.v[1826] * s.v[1804]))), A::scale_offset(s.ad_value(1803), s.v[1826], 1.0), A::offset(A::div_scaled_inputs(s.ad_value(1855), s.v[1822], s.ad_value(1808), 1.0), 1.0), 1.0);

        s.store_add_ad(1843, A::mul3_scaled_output(s.ad_value(1853), s.ad_value(1805), s.ad_value(1841), (2.0 * 1.0 / (s.v[1807]))), A::mul_sub_from_scalar_lhs(1.0, s.ad_value(1853), s.ad_value(1842)));

        s.store_div_scaled_inputs_indices(1859, 1842, s.v[1807], 1841, 1.0);

        s.store_add_scaled_product_right_ad(1860, 1859, (-1.0), 1859, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1855), 2.0, s.ad_value(1808), s.ad_value(1859), 1.0), 1.0)), 1.0);

        s.store_add_scaled_product_value_ad(1861, A::mul_sub_from_scalar_rhs(s.ad_value(1859), 1.0, s.ad_value(1853)), 1.0, 1837, 1853, 1.0);

        s.store_add_scaled_product_value_ad(1796, A::mul_sub_from_scalar_rhs(s.ad_value(1860), 1.0, s.ad_value(1853)), 1.0, 1837, 1853, 1.0);

    }

    pub(super) fn stamp_transient_block_24(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let assign31500_ad_e28650: A = {
    if (p.p52 != 0.0) {
        A::add_scaled_product(A::div(s.ad_value(1798), s.ad_value(1796)), 0.5, A::div(s.ad_value(1798), s.ad_value(1796)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1798), s.ad_value(1796))), (0.001 / p.p53)), (-0.5))
    } else {
        {
            if (p.p52 == 0.0) {
                A::add_scaled_inputs(A::div(s.ad_value(1798), s.ad_value(1796)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(1798), s.ad_value(1796)), p.p53), 0.5)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_div_from_scalar_powf_ad(1862, 1.0, A::offset(A::powf(assign31500_ad_e28650, s.v[1820]), 1.0), (1.0 / s.v[1820]));

        s.store_mul(1863, 1798, 1862);

        let assign31520_ad_e28725: A = {
    if (p.p52 != 0.0) {
        A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1796), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1796), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1796), 1.0)), (0.001 / p.p53)), (-0.5))
    } else {
        {
            if (p.p52 == 0.0) {
                A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1796), 1.0), 0.5, A::sqrt_square_offset(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1796), 1.0), p.p53), 0.5)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_div_from_scalar_powf_ad(1864, 1.0, A::offset(A::powf(assign31520_ad_e28725, s.v[1820]), 1.0), (1.0 / s.v[1820]));

        s.store_mul_neg_lhs(1865, 1798, 1864);

        s.store_div_scaled_inputs2_indices(1894, 1797, 1.0, 1895, (-1.0), 1831, 1.0);

        s.b[1905] = (s.v[1894] > 50.0);
        s.v[1905] = if s.b[1905] { 1.0 } else { 0.0 };

        if s.b[1905] {
            s.store_scalar(1836, 0.0);
        }

        s.b[1906] = (s.v[1894] < (-50.0));
        s.v[1906] = if s.b[1906] { 1.0 } else { 0.0 };

        if ((!s.b[1905]) && s.b[1906]) {
            s.store_scalar(1836, 1.0);
        }

        if ((!s.b[1905]) && (!s.b[1906])) {
            s.store_div_from_scalar_offset_ad(1836, 1.0, A::exp(s.ad_value(1894)), 1.0);
        }

        s.store_div_scaled_inputs3_mixed_iiai(1839, 1897, 1.0, 1865, (-1.0), A::add_scaled_product(s.ad_value(1795), 1.0, s.ad_value(1831), s.ad_value(1836), (-(p.p51 * 0.1))), -1.0, 1837, 1.0);

        s.b[1907] = (s.v[1839] > 50.0);
        s.v[1907] = if s.b[1907] { 1.0 } else { 0.0 };

        if s.b[1907] {
            s.store_mul(1840, 1838, 1839);
        }

        s.b[1908] = (s.v[1839] < (-50.0));
        s.v[1908] = if s.b[1908] { 1.0 } else { 0.0 };

        if ((!s.b[1907]) && s.b[1908]) {
            s.store_mul_exp_rhs(1840, 1838, 1839);
        }

        if ((!s.b[1907]) && (!s.b[1908])) {
            s.store_mul_ln_one_plus_exp_rhs(1840, 1838, 1839);
        }

        s.store_div_scaled_inputs2_indices(1894, 1897, 1.0, 1895, (-1.0), 1831, 1.0);

        s.b[1909] = (s.v[1894] > 50.0);
        s.v[1909] = if s.b[1909] { 1.0 } else { 0.0 };

        if s.b[1909] {
            s.store_scalar(1866, 0.0);
        }

        s.b[1910] = (s.v[1894] < (-50.0));
        s.v[1910] = if s.b[1910] { 1.0 } else { 0.0 };

        if ((!s.b[1909]) && s.b[1910]) {
            s.store_scalar(1866, 1.0);
        }

        if ((!s.b[1909]) && (!s.b[1910])) {
            s.store_div_from_scalar_offset_ad(1866, 1.0, A::exp(s.ad_value(1894)), 1.0);
        }

        s.store_div_scaled_inputs3_mixed_iiai(1867, 1797, 1.0, 1863, (-1.0), A::add_scaled_product(s.ad_value(1795), 1.0, s.ad_value(1831), s.ad_value(1866), (-(p.p51 * 0.1))), -1.0, 1837, 1.0);

        s.b[1911] = (s.v[1867] > 50.0);
        s.v[1911] = if s.b[1911] { 1.0 } else { 0.0 };

        if s.b[1911] {
            s.store_mul(1868, 1838, 1867);
        }

        s.b[1912] = (s.v[1867] < (-50.0));
        s.v[1912] = if s.b[1912] { 1.0 } else { 0.0 };

        if ((!s.b[1911]) && s.b[1912]) {
            s.store_mul_exp_rhs(1868, 1838, 1867);
        }

        if ((!s.b[1911]) && (!s.b[1912])) {
            s.store_mul_ln_one_plus_exp_rhs(1868, 1838, 1867);
        }

        s.store_div_scaled_inputs2_indices(1869, 1840, 1.0, 1868, (-1.0), 1808, 1.0);

        s.store_div(1895, 1869, 1861);

        s.store_div_ad_rhs(1870, 1895, A::powf(A::offset(A::powf({
            if (p.p52 != 0.0) {
                A::mul(s.ad_value(1895), A::tanh_scaled_input(s.ad_value(1895), (0.001 / p.p53)))
            } else {
                {
                    if (p.p52 == 0.0) {
                        A::sqrt_square_offset(s.ad_value(1895), p.p53)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, s.v[1820]), 1.0), (1.0 / s.v[1820])));

        s.store_mul(1871, 1843, 1870);

        s.store_mul_ad_affine_product_lhs(1789, A::add(s.ad_value(1840), s.ad_value(1868)), s.ad_value(1871), (((s.v[1829] * s.v[1806]) * s.v[1828]) * 0.5), 0.0, 1830);

        s.store_div_from_scalar_scaled_input(1844, s.v[1813], 1805, 2.302585092994046);

        s.store_scaled_mul(1846, 1844, 1805, 2.0);

        s.store_mul(1847, 1808, 1846);

        s.store_sub_scaled_inputs(1899, 1834, 1.0, 1831, (p.p51 * 0.5));

        let assign31870_ad_e29037: A = {
    if (p.p52 != 0.0) {
        A::add_scaled_inputs_product(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sub(s.ad_value(1797), s.ad_value(1897)), A::tanh_scaled_input(A::sub(s.ad_value(1797), s.ad_value(1897)), (0.001 / p.p53)), 0.5)
    } else {
        {
            if (p.p52 == 0.0) {
                A::add_scaled_inputs3(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1797), s.ad_value(1897)), p.p53), 0.5)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_div_scaled_inputs2_mixed_aii(1898, assign31870_ad_e29037, 1.0, 1899, (-1.0), 1831, 1.0);

        s.b[1913] = (s.v[1898] > 50.0);
        s.v[1913] = if s.b[1913] { 1.0 } else { 0.0 };

        if s.b[1913] {
            s.store_scalar(1856, 0.0);
        }

        s.b[1914] = (s.v[1898] < (-50.0));
        s.v[1914] = if s.b[1914] { 1.0 } else { 0.0 };

        if ((!s.b[1913]) && s.b[1914]) {
            s.store_scalar(1856, 1.0);
        }

        if ((!s.b[1913]) && (!s.b[1914])) {
            s.store_div_from_scalar_offset_ad(1856, 1.0, A::exp(s.ad_value(1898)), 1.0);
        }

        let assign31930_ad_e29116: A = {
    if (p.p52 != 0.0) {
        A::add_scaled_inputs_product(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sub(s.ad_value(1797), s.ad_value(1897)), A::tanh_scaled_input(A::sub(s.ad_value(1797), s.ad_value(1897)), (0.001 / p.p53)), 0.5)
    } else {
        {
            if (p.p52 == 0.0) {
                A::add_scaled_inputs3(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1797), s.ad_value(1897)), p.p53), 0.5)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_div_scaled_inputs2_mixed_aai(1857, assign31930_ad_e29116, 1.0, A::add_scaled_product(s.ad_value(1834), 1.0, s.ad_value(1831), s.ad_value(1856), (-(p.p51 * 0.1))), (-1.0), 1846, 1.0);

        s.b[1915] = (s.v[1857] > 50.0);
        s.v[1915] = if s.b[1915] { 1.0 } else { 0.0 };

        if s.b[1915] {
            s.store_mul(1858, 1847, 1857);
        }

        s.b[1916] = (s.v[1857] < (-50.0));
        s.v[1916] = if s.b[1916] { 1.0 } else { 0.0 };

        if ((!s.b[1915]) && s.b[1916]) {
            s.store_mul_exp_rhs(1858, 1847, 1857);
        }

        if ((!s.b[1915]) && (!s.b[1916])) {
            s.store_mul_ln_one_plus_exp_rhs(1858, 1847, 1857);
        }

        s.store_div_from_scalar(1850, s.v[1819], 1852);

        s.store_scaled_div_from_scalar_ad(1851, (1.0 + (s.v[1826] * s.v[1804])), A::scale_offset(s.ad_value(1803), s.v[1826], 1.0), s.v[1818]);

        s.store_div_scaled_inputs_indices(1872, 1851, s.v[1807], 1850, 1.0);

        s.store_add_scaled_product_right_ad(1873, 1872, (-1.0), 1872, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1858), 2.0, s.ad_value(1808), s.ad_value(1872), 1.0), 1.0)), 1.0);

        s.store_add_scaled_product_value_ad(1874, A::mul_sub_from_scalar_rhs(s.ad_value(1873), 1.0, s.ad_value(1856)), 1.0, 1846, 1856, 1.0);

        let assign32040_ad_e29267: A = {
    if (p.p52 != 0.0) {
        A::add_scaled_product(A::div(s.ad_value(1798), s.ad_value(1874)), 0.5, A::div(s.ad_value(1798), s.ad_value(1874)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1798), s.ad_value(1874))), (0.001 / p.p53)), (-0.5))
    } else {
        {
            if (p.p52 == 0.0) {
                A::add_scaled_inputs(A::div(s.ad_value(1798), s.ad_value(1874)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(1798), s.ad_value(1874)), p.p53), 0.5)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_div_from_scalar_powf_ad(1875, 1.0, A::offset(A::powf(assign32040_ad_e29267, s.v[1820]), 1.0), (1.0 / s.v[1820]));

        s.store_mul(1876, 1798, 1875);

        let assign32060_ad_e29342: A = {
    if (p.p52 != 0.0) {
        A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1874), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1874), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1874), 1.0)), (0.001 / p.p53)), (-0.5))
    } else {
        {
            if (p.p52 == 0.0) {
                A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1874), 1.0), 0.5, A::sqrt_square_offset(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1874), 1.0), p.p53), 0.5)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_div_from_scalar_powf_ad(1877, 1.0, A::offset(A::powf(assign32060_ad_e29342, s.v[1820]), 1.0), (1.0 / s.v[1820]));

        s.store_mul_neg_lhs(1878, 1798, 1877);

        s.store_div_scaled_inputs2_indices(1898, 1797, 1.0, 1899, (-1.0), 1831, 1.0);

        s.b[1917] = (s.v[1898] > 50.0);
        s.v[1917] = if s.b[1917] { 1.0 } else { 0.0 };

        if s.b[1917] {
            s.store_scalar(1845, 0.0);
        }

        s.b[1918] = (s.v[1898] < (-50.0));
        s.v[1918] = if s.b[1918] { 1.0 } else { 0.0 };

        if ((!s.b[1917]) && s.b[1918]) {
            s.store_scalar(1845, 1.0);
        }

        if ((!s.b[1917]) && (!s.b[1918])) {
            s.store_div_from_scalar_offset_ad(1845, 1.0, A::exp(s.ad_value(1898)), 1.0);
        }

        s.store_div_scaled_inputs3_mixed_iiai(1848, 1897, 1.0, 1878, (-1.0), A::add_scaled_product(s.ad_value(1834), 1.0, s.ad_value(1831), s.ad_value(1845), (-(p.p51 * 0.1))), -1.0, 1846, 1.0);

        s.b[1919] = (s.v[1848] > 50.0);
        s.v[1919] = if s.b[1919] { 1.0 } else { 0.0 };

        if s.b[1919] {
            s.store_mul(1849, 1847, 1848);
        }

        s.b[1920] = (s.v[1848] < (-50.0));
        s.v[1920] = if s.b[1920] { 1.0 } else { 0.0 };

        if ((!s.b[1919]) && s.b[1920]) {
            s.store_mul_exp_rhs(1849, 1847, 1848);
        }

        if ((!s.b[1919]) && (!s.b[1920])) {
            s.store_mul_ln_one_plus_exp_rhs(1849, 1847, 1848);
        }

        s.store_div_scaled_inputs2_indices(1898, 1897, 1.0, 1899, (-1.0), 1831, 1.0);

        s.b[1921] = (s.v[1898] > 50.0);
        s.v[1921] = if s.b[1921] { 1.0 } else { 0.0 };

        if s.b[1921] {
            s.store_scalar(1879, 0.0);
        }

        s.b[1922] = (s.v[1898] < (-50.0));
        s.v[1922] = if s.b[1922] { 1.0 } else { 0.0 };

        if ((!s.b[1921]) && s.b[1922]) {
            s.store_scalar(1879, 1.0);
        }

        if ((!s.b[1921]) && (!s.b[1922])) {
            s.store_div_from_scalar_offset_ad(1879, 1.0, A::exp(s.ad_value(1898)), 1.0);
        }

        s.store_div_scaled_inputs3_mixed_iiai(1880, 1797, 1.0, 1876, (-1.0), A::add_scaled_product(s.ad_value(1834), 1.0, s.ad_value(1831), s.ad_value(1879), (-(p.p51 * 0.1))), -1.0, 1846, 1.0);

        s.b[1923] = (s.v[1880] > 50.0);
        s.v[1923] = if s.b[1923] { 1.0 } else { 0.0 };

        if s.b[1923] {
            s.store_mul(1881, 1847, 1880);
        }

        s.b[1924] = (s.v[1880] < (-50.0));
        s.v[1924] = if s.b[1924] { 1.0 } else { 0.0 };

        if ((!s.b[1923]) && s.b[1924]) {
            s.store_mul_exp_rhs(1881, 1847, 1880);
        }

        if ((!s.b[1923]) && (!s.b[1924])) {
            s.store_mul_ln_one_plus_exp_rhs(1881, 1847, 1880);
        }

        s.store_offset_square(1882, 1849, 1e-38);

        s.store_offset_mul(1883, 1882, 1849, 1e-57);

        s.store_offset_square(1884, 1881, 1e-38);

        s.store_offset_mul(1885, 1884, 1881, 1e-57);

        s.store_offset_mul(1886, 1849, 1881, 1e-38);

        s.store_div_scaled_inputs3_mixed_iiia(1887, 1882, (2.0 / 3.0), 1884, (2.0 / 3.0), 1886, (2.0 / 3.0), A::offset(A::add(s.ad_value(1849), s.ad_value(1881)), 2e-19), 1.0);

        s.store_div_ad(1888, A::add_scaled_inputs_products(s.ad_value(1883), (2.0 * 2.0), s.ad_value(1885), (3.0 * 2.0), s.ad_value(1882), s.ad_value(1881), (4.0 * 2.0), s.ad_value(1884), s.ad_value(1849), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(1882), 15.0, s.ad_value(1884), 15.0, s.ad_value(1886), (2.0 * 15.0)));

        s.store_sub(1889, 1887, 1888);

        s.copy_ad(1890, 1888);

        s.store_scaled_mul(1790, 1889, 1830, (((s.v[1806] * s.v[1828]) * s.v[1807]) * s.v[1829]));

        s.store_scaled_mul(1791, 1890, 1830, (((s.v[1806] * s.v[1828]) * s.v[1807]) * s.v[1829]));

        s.b[1925] = (s.v[1799] == 1.0);
        s.v[1925] = if s.b[1925] { 1.0 } else { 0.0 };

        if s.b[1925] {
            s.store_div_ad_lhs(1891, A::sub_from_scalar(s.v[1800], A::sub_scaled_inputs(s.ad_value(1834), 1.0, s.ad_value(1831), (p.p51 * 0.5))), 1846);
        }

        s.b[1926] = (s.v[1891] > 50.0);
        s.v[1926] = if s.b[1926] { 1.0 } else { 0.0 };

        if (s.b[1925] && s.b[1926]) {
            s.copy_ad(1894, 1891);
        }

        s.b[1927] = (s.v[1891] < (-50.0));
        s.v[1927] = if s.b[1927] { 1.0 } else { 0.0 };

        if ((s.b[1925] && (!s.b[1926])) && s.b[1927]) {
            s.store_exp(1894, 1891);
        }

        if ((s.b[1925] && (!s.b[1926])) && (!s.b[1927])) {
            s.store_ln_one_plus_exp(1894, 1891);
        }

        if s.b[1925] {
            s.store_div_ad_lhs(1892, A::sub_from_scalar(s.v[1801], A::sub_scaled_inputs(s.ad_value(1834), 1.0, s.ad_value(1831), (p.p51 * 0.5))), 1846);
        }

        s.b[1928] = (s.v[1892] > 50.0);
        s.v[1928] = if s.b[1928] { 1.0 } else { 0.0 };

        if (s.b[1925] && s.b[1928]) {
            s.copy_ad(1894, 1892);
        }

        s.b[1929] = (s.v[1892] < (-50.0));
        s.v[1929] = if s.b[1929] { 1.0 } else { 0.0 };

        if ((s.b[1925] && (!s.b[1928])) && s.b[1929]) {
            s.store_exp(1894, 1892);
        }

        if ((s.b[1925] && (!s.b[1928])) && (!s.b[1929])) {
            s.store_ln_one_plus_exp(1894, 1892);
        }

        s.b[1930] = (s.v[1802] == 1.0);
        s.v[1930] = if s.b[1930] { 1.0 } else { 0.0 };

        if s.b[1930] {
            s.store_div_scaled_inputs3_indices(1893, 1797, 1.0, 1834, -1.0, 1831, (-(-(p.p51 * 0.5))), 1846, 1.0);
        }

        s.b[1931] = (s.v[1893] > 50.0);
        s.v[1931] = if s.b[1931] { 1.0 } else { 0.0 };

        if (s.b[1930] && s.b[1931]) {
            s.copy_ad(1894, 1893);
        }

        s.b[1932] = (s.v[1893] < (-50.0));
        s.v[1932] = if s.b[1932] { 1.0 } else { 0.0 };

        if ((s.b[1930] && (!s.b[1931])) && s.b[1932]) {
            s.store_exp(1894, 1893);
        }

        if ((s.b[1930] && (!s.b[1931])) && (!s.b[1932])) {
            s.store_ln_one_plus_exp(1894, 1893);
        }

        s.copy_ad(1788, 1789);

        s.copy_ad(115, 1789);

        s.copy_ad(117, 1790);

        s.copy_ad(118, 1791);

        s.copy_ad(115, 1788);

        s.b[1933] = (p.p322 == 0.0);
        s.v[1933] = if s.b[1933] { 1.0 } else { 0.0 };

        s.v[122] = 0.0;

        s.v[123] = 0.0;

        s.v[134] = 0.0;

        s.v[135] = 0.0;

        s.v[128] = 0.0;

        s.v[129] = 0.0;

        s.v[140] = 0.0;

        s.v[141] = 0.0;

        s.b[1934] = (p.p254 == 1.0);
        s.v[1934] = if s.b[1934] { 1.0 } else { 0.0 };

        if s.b[1934] {
            s.store_scalar(1935, 0.0);
            s.store_scalar(1936, 0.0);
            s.store_scalar(1937, 0.0);
            s.store_scaled_voltage(1938, ctx, nodes, Some(8), Some(13), p.p6);
            s.copy_ad(1939, 113);
            s.store_scalar(1940, p.p260);
            s.store_scalar(1941, p.p262);
            s.store_scalar(1942, p.p261);
            s.store_scalar(1943, p.p258);
            s.store_scalar(1944, p.p278);
            s.store_scalar(1945, p.p277);
            s.copy_ad(1946, 112);
        }

    }

    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1934] {
            s.store_scalar(1947, p.p0);
            s.store_scalar(1948, p.p2);
            s.store_scalar(1949, ((1.0 - p.p255) * p.p259));
            s.store_scalar(1950, p.p276);
            s.store_scalar(1951, p.p270);
            s.store_scalar(1952, p.p271);
            s.store_scalar(1953, ((1.0 - p.p255) * p.p269));
            s.store_scalar(1954, p.p268);
            s.store_scalar(1955, p.p257);
            s.store_scalar(1956, p.p256);
            s.store_scalar(1957, p.p6);
            s.store_scalar(1958, 0.0);
            s.store_scalar(1959, 0.0);
            s.store_scalar(1960, 0.0);
            s.store_scalar(1961, 0.0);
            s.store_scalar(1962, 0.0);
            s.store_scalar(1963, 0.0);
            s.store_scalar(1964, 0.0);
            s.store_scalar(1965, 0.0);
            s.store_scalar(1966, 0.0);
            s.store_scalar(1967, 0.0);
            s.store_scalar(1968, 0.0);
            s.store_scalar(1969, 0.0);
            s.store_scalar(1970, 0.0);
            s.store_scalar(1971, 0.0);
            s.store_scalar(1972, 0.0);
            s.store_scalar(1973, 0.0);
            s.store_scalar(1974, 0.0);
            s.store_scalar(1975, 0.0);
            s.store_scalar(1976, 0.0);
            s.store_scalar(1977, 0.0);
            s.store_scalar(1978, 0.0);
            s.store_scalar(1979, 0.0);
            s.store_scalar(1980, 0.0);
            s.store_scalar(1981, 0.0);
            s.store_scalar(1982, 0.0);
            s.store_scalar(1983, 0.0);
            s.store_scalar(1984, 0.0);
            s.store_scalar(1985, 0.0);
            s.store_scalar(1986, 0.0);
            s.store_scalar(1987, 0.0);
            s.store_scalar(1988, 0.0);
            s.store_scalar(1989, 0.0);
            s.store_scalar(1990, 0.0);
            s.store_mul_scaled_ad_lhs(1970, A::div(s.ad_value(1955), s.ad_value(1939)), 1956, -1.0);
        }

        if s.b[1934] {
            if ((!(s.v[1970] > 50.0)) && (!(s.v[1970] < (-50.0)))) {
                s.store_exp(1960, 1970);
            } else {
                if ((!(s.v[1970] > 50.0)) && (s.v[1970] < (-50.0))) {
                    s.store_scalar(1960, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[1970] > 50.0) {
                        s.store_scaled_offset(1960, 1970, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(1960, 0.0);
                    }
                }
            }
        }

        if s.b[1934] {
            s.store_add_scaled_product_right_ad(1966, 1970, 1.0, 1944, A::sub_scaled_inputs(s.ad_value(1938), -1.0, s.ad_value(1945), 1.0), 1.0);
            s.store_add_scaled_product_indices(1967, 1970, 1.0, 1944, 1945, -1.0);
        }

        if s.b[1934] {
            if ((!(s.v[1966] > 50.0)) && (!(s.v[1966] < (-50.0)))) {
                s.store_exp(1968, 1966);
            } else {
                if ((!(s.v[1966] > 50.0)) && (s.v[1966] < (-50.0))) {
                    s.store_scalar(1968, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[1966] > 50.0) {
                        s.store_scaled_offset(1968, 1966, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(1968, 0.0);
                    }
                }
            }
        }

        if s.b[1934] {
            if ((!(s.v[1967] > 50.0)) && (!(s.v[1967] < (-50.0)))) {
                s.store_exp(1969, 1967);
            } else {
                if ((!(s.v[1967] > 50.0)) && (s.v[1967] < (-50.0))) {
                    s.store_scalar(1969, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[1967] > 50.0) {
                        s.store_scaled_offset(1969, 1967, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(1969, 0.0);
                    }
                }
            }
        }

        if s.b[1934] {
            s.store_sub(1962, 1968, 1969);
            s.store_mul_ad_product_lhs(1936, A::mul3(s.ad_value(1957), s.ad_value(1947), s.ad_value(1948)), s.ad_value(1949), 1946);
            s.store_add_scaled_product_left_ad(1972, 1970, 1.0, A::div(s.ad_value(1943), s.ad_value(1939)), 1938, 1.0);
        }

        if s.b[1934] {
            if ((!(s.v[1972] > 50.0)) && (!(s.v[1972] < (-50.0)))) {
                s.store_exp(1973, 1972);
            } else {
                if ((!(s.v[1972] > 50.0)) && (s.v[1972] < (-50.0))) {
                    s.store_scalar(1973, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[1972] > 50.0) {
                        s.store_scaled_offset(1973, 1972, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(1973, 0.0);
                    }
                }
            }
        }

        s.b[1991] = (s.v[1942] == 1.0);
        s.v[1991] = if s.b[1991] { 1.0 } else { 0.0 };

        if (s.b[1934] && s.b[1991]) {
            s.store_mul_sub_ad_rhs(1963, 1936, A::add_scaled_product(s.ad_value(1973), 1.0, s.ad_value(1950), s.ad_value(1962), (-1.0)), s.ad_value(1960));
        }

        if (s.b[1934] && (!s.b[1991])) {
            s.store_add_scaled_product_right_ad(1977, 1970, 1.0, 1944, A::sub_scaled_inputs(s.ad_value(1940), -1.0, s.ad_value(1945), 1.0), 1.0);
        }

        if (s.b[1934] && (!s.b[1991])) {
            if ((!(s.v[1977] > 50.0)) && (!(s.v[1977] < (-50.0)))) {
                s.store_exp(1978, 1977);
            } else {
                if ((!(s.v[1977] > 50.0)) && (s.v[1977] < (-50.0))) {
                    s.store_scalar(1978, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[1977] > 50.0) {
                        s.store_scaled_offset(1978, 1977, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(1978, 0.0);
                    }
                }
            }
        }

        if (s.b[1934] && (!s.b[1991])) {
            s.store_sub(1979, 1978, 1969);
            s.store_add_scaled_product_left_ad(1980, 1970, 1.0, A::div(s.ad_value(1943), s.ad_value(1939)), 1940, 1.0);
        }

        if (s.b[1934] && (!s.b[1991])) {
            if ((!(s.v[1980] > 50.0)) && (!(s.v[1980] < (-50.0)))) {
                s.store_exp(1981, 1980);
            } else {
                if ((!(s.v[1980] > 50.0)) && (s.v[1980] < (-50.0))) {
                    s.store_scalar(1981, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[1980] > 50.0) {
                        s.store_scaled_offset(1981, 1980, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(1981, 0.0);
                    }
                }
            }
        }

        if (s.b[1934] && (!s.b[1991])) {
            s.store_sub_ad_lhs(1982, A::add_scaled_product(s.ad_value(1981), 1.0, s.ad_value(1950), s.ad_value(1979), (-1.0)), 1960);
            s.store_mul_sub_ad_rhs(1983, 1936, A::add_scaled_product(s.ad_value(1973), 1.0, s.ad_value(1950), s.ad_value(1962), (-1.0)), s.ad_value(1960));
        }

        s.b[1992] = (s.v[1942] > 0.0);
        s.v[1992] = if s.b[1992] { 1.0 } else { 0.0 };

        if ((s.b[1934] && (!s.b[1991])) && s.b[1992]) {
            s.store_mul(1976, 1942, 1943);
            s.store_add_scaled_product_left_ad(1984, 1970, 1.0, A::div(s.ad_value(1976), s.ad_value(1939)), 1940, 1.0);
        }

        if ((s.b[1934] && (!s.b[1991])) && s.b[1992]) {
            if ((!(s.v[1984] > 50.0)) && (!(s.v[1984] < (-50.0)))) {
                s.store_exp(1985, 1984);
            } else {
                if ((!(s.v[1984] > 50.0)) && (s.v[1984] < (-50.0))) {
                    s.store_scalar(1985, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[1984] > 50.0) {
                        s.store_scaled_offset(1985, 1984, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(1985, 0.0);
                    }
                }
            }
        }

        if ((s.b[1934] && (!s.b[1991])) && s.b[1992]) {
            s.store_sub_ad_lhs(1986, A::add_scaled_product(s.ad_value(1985), 1.0, s.ad_value(1950), s.ad_value(1979), (-1.0)), 1960);
            s.store_add_scaled_product_left_ad(1987, 1970, 1.0, A::div(s.ad_value(1976), s.ad_value(1939)), 1938, 1.0);
        }

        if ((s.b[1934] && (!s.b[1991])) && s.b[1992]) {
            if ((!(s.v[1987] > 50.0)) && (!(s.v[1987] < (-50.0)))) {
                s.store_exp(1988, 1987);
            } else {
                if ((!(s.v[1987] > 50.0)) && (s.v[1987] < (-50.0))) {
                    s.store_scalar(1988, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[1987] > 50.0) {
                        s.store_scaled_offset(1988, 1987, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(1988, 0.0);
                    }
                }
            }
        }

        if ((s.b[1934] && (!s.b[1991])) && s.b[1992]) {
            s.store_div_scaled_product_indices(1989, 1936, 1982, 1.0, 1986, 1.0);
            s.store_mul_sub_ad_rhs(1990, 1989, A::add_scaled_product(s.ad_value(1988), 1.0, s.ad_value(1950), s.ad_value(1962), (-1.0)), s.ad_value(1960));
        }

        if ((s.b[1934] && (!s.b[1991])) && (!s.b[1992])) {
            s.store_mul(1990, 1936, 1982);
        }

        if (s.b[1934] && (!s.b[1991])) {
            s.store_mul_square_lhs(1959, 1941, 1939);
            s.store_div_scaled_inputs3_indices(1971, 1938, 1.0, 1940, -1.0, 1959, (-(-0.5)), 1959, 1.0);
        }

        s.b[1993] = (s.v[1971] > 50.0);
        s.v[1993] = if s.b[1993] { 1.0 } else { 0.0 };

        if ((s.b[1934] && (!s.b[1991])) && s.b[1993]) {
            s.store_scalar(1961, 0.0);
        }

        s.b[1994] = (s.v[1971] < (-50.0));
        s.v[1994] = if s.b[1994] { 1.0 } else { 0.0 };

        if (((s.b[1934] && (!s.b[1991])) && (!s.b[1993])) && s.b[1994]) {
            s.store_scalar(1961, 1.0);
        }

        if (((s.b[1934] && (!s.b[1991])) && (!s.b[1993])) && (!s.b[1994])) {
            s.store_div_from_scalar_offset_ad(1961, 1.0, A::exp(s.ad_value(1971)), 1.0);
        }

        if (s.b[1934] && (!s.b[1991])) {
            s.store_add_scaled_product_value_ad(1963, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(1961), s.ad_value(1990)), 1.0, 1961, 1983, 1.0);
        }

        if s.b[1934] {
            let assign33990_ad_e30838: A = {
                if (p.p52 != 0.0) {
                    A::mul(A::div(s.ad_value(1938), s.ad_value(1951)), A::tanh_scaled_input(A::div(s.ad_value(1938), s.ad_value(1951)), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt_square_offset(A::div(s.ad_value(1938), s.ad_value(1951)), p.p53)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs_mixed_ia(1964, 1938, -1.0, A::pow(A::offset(A::pow(assign33990_ad_e30838, s.ad_value(1952)), 1.0), A::div_from_scalar(1.0, s.ad_value(1952))), 1.0);
        }

        if s.b[1934] {
            s.store_mul_ad_product_lhs(1937, A::mul3_scaled_output(s.ad_value(1957), s.ad_value(1947), s.ad_value(1948), -1.0), s.ad_value(1953), 1946);
            s.store_mul_div_lhs(1974, 1954, 1939, 1964);
        }

    }

    pub(super) fn stamp_transient_block_26(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1934] {
            if ((!(s.v[1974] > 50.0)) && (!(s.v[1974] < (-50.0)))) {
                s.store_exp(1975, 1974);
            } else {
                if ((!(s.v[1974] > 50.0)) && (s.v[1974] < (-50.0))) {
                    s.store_scalar(1975, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[1974] > 50.0) {
                        s.store_scaled_offset(1975, 1974, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(1975, 0.0);
                    }
                }
            }
        }

        if s.b[1934] {
            s.store_mul_offset_rhs(1965, 1937, 1975, (-1.0));
            s.store_add(1958, 1963, 1965);
            s.copy_ad(1935, 1958);
            s.copy_ad(122, 1935);
            s.store_scalar(1995, 0.0);
            s.store_scalar(1996, 0.0);
            s.store_scalar(1997, 0.0);
            s.store_scaled_voltage(1998, ctx, nodes, Some(8), Some(17), p.p6);
            s.copy_ad(1999, 113);
            s.store_scalar(2000, p.p265);
            s.store_scalar(2001, p.p267);
            s.store_scalar(2002, p.p266);
            s.store_scalar(2003, p.p263);
            s.store_scalar(2004, p.p281);
            s.store_scalar(2005, p.p280);
            s.copy_ad(2006, 112);
            s.store_scalar(2007, p.p0);
            s.store_scalar(2008, p.p2);
            s.store_scalar(2009, ((1.0 - p.p255) * p.p264));
            s.store_scalar(2010, p.p279);
            s.store_scalar(2011, p.p274);
            s.store_scalar(2012, p.p275);
            s.store_scalar(2013, ((1.0 - p.p255) * p.p273));
            s.store_scalar(2014, p.p272);
            s.store_scalar(2015, p.p257);
            s.store_scalar(2016, p.p256);
            s.store_scalar(2017, p.p6);
            s.store_scalar(2018, 0.0);
            s.store_scalar(2019, 0.0);
            s.store_scalar(2020, 0.0);
            s.store_scalar(2021, 0.0);
            s.store_scalar(2022, 0.0);
            s.store_scalar(2023, 0.0);
            s.store_scalar(2024, 0.0);
            s.store_scalar(2025, 0.0);
            s.store_scalar(2026, 0.0);
            s.store_scalar(2027, 0.0);
            s.store_scalar(2028, 0.0);
            s.store_scalar(2029, 0.0);
            s.store_scalar(2030, 0.0);
            s.store_scalar(2031, 0.0);
            s.store_scalar(2032, 0.0);
            s.store_scalar(2033, 0.0);
            s.store_scalar(2034, 0.0);
            s.store_scalar(2035, 0.0);
            s.store_scalar(2036, 0.0);
            s.store_scalar(2037, 0.0);
            s.store_scalar(2038, 0.0);
            s.store_scalar(2039, 0.0);
            s.store_scalar(2040, 0.0);
            s.store_scalar(2041, 0.0);
            s.store_scalar(2042, 0.0);
            s.store_scalar(2043, 0.0);
            s.store_scalar(2044, 0.0);
            s.store_scalar(2045, 0.0);
            s.store_scalar(2046, 0.0);
            s.store_scalar(2047, 0.0);
            s.store_scalar(2048, 0.0);
            s.store_scalar(2049, 0.0);
            s.store_scalar(2050, 0.0);
            s.store_mul_scaled_ad_lhs(2030, A::div(s.ad_value(2015), s.ad_value(1999)), 2016, -1.0);
        }

        if s.b[1934] {
            if ((!(s.v[2030] > 50.0)) && (!(s.v[2030] < (-50.0)))) {
                s.store_exp(2020, 2030);
            } else {
                if ((!(s.v[2030] > 50.0)) && (s.v[2030] < (-50.0))) {
                    s.store_scalar(2020, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2030] > 50.0) {
                        s.store_scaled_offset(2020, 2030, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2020, 0.0);
                    }
                }
            }
        }

        if s.b[1934] {
            s.store_add_scaled_product_right_ad(2026, 2030, 1.0, 2004, A::sub_scaled_inputs(s.ad_value(1998), -1.0, s.ad_value(2005), 1.0), 1.0);
            s.store_add_scaled_product_indices(2027, 2030, 1.0, 2004, 2005, -1.0);
        }

        if s.b[1934] {
            if ((!(s.v[2026] > 50.0)) && (!(s.v[2026] < (-50.0)))) {
                s.store_exp(2028, 2026);
            } else {
                if ((!(s.v[2026] > 50.0)) && (s.v[2026] < (-50.0))) {
                    s.store_scalar(2028, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2026] > 50.0) {
                        s.store_scaled_offset(2028, 2026, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2028, 0.0);
                    }
                }
            }
        }

        if s.b[1934] {
            if ((!(s.v[2027] > 50.0)) && (!(s.v[2027] < (-50.0)))) {
                s.store_exp(2029, 2027);
            } else {
                if ((!(s.v[2027] > 50.0)) && (s.v[2027] < (-50.0))) {
                    s.store_scalar(2029, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2027] > 50.0) {
                        s.store_scaled_offset(2029, 2027, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2029, 0.0);
                    }
                }
            }
        }

        if s.b[1934] {
            s.store_sub(2022, 2028, 2029);
            s.store_mul_ad_product_lhs(1996, A::mul3(s.ad_value(2017), s.ad_value(2007), s.ad_value(2008)), s.ad_value(2009), 2006);
            s.store_add_scaled_product_left_ad(2032, 2030, 1.0, A::div(s.ad_value(2003), s.ad_value(1999)), 1998, 1.0);
        }

        if s.b[1934] {
            if ((!(s.v[2032] > 50.0)) && (!(s.v[2032] < (-50.0)))) {
                s.store_exp(2033, 2032);
            } else {
                if ((!(s.v[2032] > 50.0)) && (s.v[2032] < (-50.0))) {
                    s.store_scalar(2033, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2032] > 50.0) {
                        s.store_scaled_offset(2033, 2032, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2033, 0.0);
                    }
                }
            }
        }

        s.b[2051] = (s.v[2002] == 1.0);
        s.v[2051] = if s.b[2051] { 1.0 } else { 0.0 };

        if (s.b[1934] && s.b[2051]) {
            s.store_mul_sub_ad_rhs(2023, 1996, A::add_scaled_product(s.ad_value(2033), 1.0, s.ad_value(2010), s.ad_value(2022), (-1.0)), s.ad_value(2020));
        }

        if (s.b[1934] && (!s.b[2051])) {
            s.store_add_scaled_product_right_ad(2037, 2030, 1.0, 2004, A::sub_scaled_inputs(s.ad_value(2000), -1.0, s.ad_value(2005), 1.0), 1.0);
        }

        if (s.b[1934] && (!s.b[2051])) {
            if ((!(s.v[2037] > 50.0)) && (!(s.v[2037] < (-50.0)))) {
                s.store_exp(2038, 2037);
            } else {
                if ((!(s.v[2037] > 50.0)) && (s.v[2037] < (-50.0))) {
                    s.store_scalar(2038, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2037] > 50.0) {
                        s.store_scaled_offset(2038, 2037, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2038, 0.0);
                    }
                }
            }
        }

        if (s.b[1934] && (!s.b[2051])) {
            s.store_sub(2039, 2038, 2029);
            s.store_add_scaled_product_left_ad(2040, 2030, 1.0, A::div(s.ad_value(2003), s.ad_value(1999)), 2000, 1.0);
        }

        if (s.b[1934] && (!s.b[2051])) {
            if ((!(s.v[2040] > 50.0)) && (!(s.v[2040] < (-50.0)))) {
                s.store_exp(2041, 2040);
            } else {
                if ((!(s.v[2040] > 50.0)) && (s.v[2040] < (-50.0))) {
                    s.store_scalar(2041, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2040] > 50.0) {
                        s.store_scaled_offset(2041, 2040, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2041, 0.0);
                    }
                }
            }
        }

        if (s.b[1934] && (!s.b[2051])) {
            s.store_sub_ad_lhs(2042, A::add_scaled_product(s.ad_value(2041), 1.0, s.ad_value(2010), s.ad_value(2039), (-1.0)), 2020);
            s.store_mul_sub_ad_rhs(2043, 1996, A::add_scaled_product(s.ad_value(2033), 1.0, s.ad_value(2010), s.ad_value(2022), (-1.0)), s.ad_value(2020));
        }

        s.b[2052] = (s.v[2002] > 0.0);
        s.v[2052] = if s.b[2052] { 1.0 } else { 0.0 };

        if ((s.b[1934] && (!s.b[2051])) && s.b[2052]) {
            s.store_mul(2036, 2002, 2003);
            s.store_add_scaled_product_left_ad(2044, 2030, 1.0, A::div(s.ad_value(2036), s.ad_value(1999)), 2000, 1.0);
        }

        if ((s.b[1934] && (!s.b[2051])) && s.b[2052]) {
            if ((!(s.v[2044] > 50.0)) && (!(s.v[2044] < (-50.0)))) {
                s.store_exp(2045, 2044);
            } else {
                if ((!(s.v[2044] > 50.0)) && (s.v[2044] < (-50.0))) {
                    s.store_scalar(2045, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2044] > 50.0) {
                        s.store_scaled_offset(2045, 2044, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2045, 0.0);
                    }
                }
            }
        }

        if ((s.b[1934] && (!s.b[2051])) && s.b[2052]) {
            s.store_sub_ad_lhs(2046, A::add_scaled_product(s.ad_value(2045), 1.0, s.ad_value(2010), s.ad_value(2039), (-1.0)), 2020);
            s.store_add_scaled_product_left_ad(2047, 2030, 1.0, A::div(s.ad_value(2036), s.ad_value(1999)), 1998, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_27(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[1934] && (!s.b[2051])) && s.b[2052]) {
            if ((!(s.v[2047] > 50.0)) && (!(s.v[2047] < (-50.0)))) {
                s.store_exp(2048, 2047);
            } else {
                if ((!(s.v[2047] > 50.0)) && (s.v[2047] < (-50.0))) {
                    s.store_scalar(2048, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2047] > 50.0) {
                        s.store_scaled_offset(2048, 2047, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2048, 0.0);
                    }
                }
            }
        }

        if ((s.b[1934] && (!s.b[2051])) && s.b[2052]) {
            s.store_div_scaled_product_indices(2049, 1996, 2042, 1.0, 2046, 1.0);
            s.store_mul_sub_ad_rhs(2050, 2049, A::add_scaled_product(s.ad_value(2048), 1.0, s.ad_value(2010), s.ad_value(2022), (-1.0)), s.ad_value(2020));
        }

        if ((s.b[1934] && (!s.b[2051])) && (!s.b[2052])) {
            s.store_mul(2050, 1996, 2042);
        }

        if (s.b[1934] && (!s.b[2051])) {
            s.store_mul_square_lhs(2019, 2001, 1999);
            s.store_div_scaled_inputs3_indices(2031, 1998, 1.0, 2000, -1.0, 2019, (-(-0.5)), 2019, 1.0);
        }

        s.b[2053] = (s.v[2031] > 50.0);
        s.v[2053] = if s.b[2053] { 1.0 } else { 0.0 };

        if ((s.b[1934] && (!s.b[2051])) && s.b[2053]) {
            s.store_scalar(2021, 0.0);
        }

        s.b[2054] = (s.v[2031] < (-50.0));
        s.v[2054] = if s.b[2054] { 1.0 } else { 0.0 };

        if (((s.b[1934] && (!s.b[2051])) && (!s.b[2053])) && s.b[2054]) {
            s.store_scalar(2021, 1.0);
        }

        if (((s.b[1934] && (!s.b[2051])) && (!s.b[2053])) && (!s.b[2054])) {
            s.store_div_from_scalar_offset_ad(2021, 1.0, A::exp(s.ad_value(2031)), 1.0);
        }

        if (s.b[1934] && (!s.b[2051])) {
            s.store_add_scaled_product_value_ad(2023, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(2021), s.ad_value(2050)), 1.0, 2021, 2043, 1.0);
        }

        if s.b[1934] {
            let assign35020_ad_e31891: A = {
                if (p.p52 != 0.0) {
                    A::mul(A::div(s.ad_value(1998), s.ad_value(2011)), A::tanh_scaled_input(A::div(s.ad_value(1998), s.ad_value(2011)), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt_square_offset(A::div(s.ad_value(1998), s.ad_value(2011)), p.p53)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs_mixed_ia(2024, 1998, -1.0, A::pow(A::offset(A::pow(assign35020_ad_e31891, s.ad_value(2012)), 1.0), A::div_from_scalar(1.0, s.ad_value(2012))), 1.0);
        }

        if s.b[1934] {
            s.store_mul_ad_product_lhs(1997, A::mul3_scaled_output(s.ad_value(2017), s.ad_value(2007), s.ad_value(2008), -1.0), s.ad_value(2013), 2006);
            s.store_mul_div_lhs(2034, 2014, 1999, 2024);
        }

        if s.b[1934] {
            if ((!(s.v[2034] > 50.0)) && (!(s.v[2034] < (-50.0)))) {
                s.store_exp(2035, 2034);
            } else {
                if ((!(s.v[2034] > 50.0)) && (s.v[2034] < (-50.0))) {
                    s.store_scalar(2035, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2034] > 50.0) {
                        s.store_scaled_offset(2035, 2034, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2035, 0.0);
                    }
                }
            }
        }

        if s.b[1934] {
            s.store_mul_offset_rhs(2025, 1997, 2035, (-1.0));
            s.store_add(2018, 2023, 2025);
            s.copy_ad(1995, 2018);
            s.copy_ad(123, 1995);
        }

        s.b[2055] = (p.p282 == 1.0);
        s.v[2055] = if s.b[2055] { 1.0 } else { 0.0 };

        if (s.b[1934] && s.b[2055]) {
            s.store_scalar(2056, 0.0);
            s.store_scalar(2057, 0.0);
            s.store_scalar(2058, 0.0);
            s.store_scaled_voltage(2059, ctx, nodes, Some(8), Some(13), p.p6);
            s.copy_ad(2060, 113);
            s.store_scalar(2061, p.p260);
            s.store_scalar(2062, p.p262);
            s.store_scalar(2063, 1.0);
            s.store_scalar(2064, p.p258);
            s.store_scalar(2065, p.p278);
            s.store_scalar(2066, p.p277);
            s.copy_ad(2067, 112);
            s.store_scalar(2068, p.p0);
            s.store_scalar(2069, p.p2);
            s.store_scalar(2070, 0.0);
            s.store_scalar(2071, 0.0);
            s.store_scalar(2072, p.p285);
            s.store_scalar(2073, p.p286);
            s.store_scalar(2074, ((1.0 - p.p255) * p.p284));
            s.store_scalar(2075, p.p283);
            s.store_scalar(2076, p.p257);
            s.store_scalar(2077, p.p256);
            s.store_scalar(2078, p.p6);
            s.store_scalar(2079, 0.0);
            s.store_scalar(2080, 0.0);
            s.store_scalar(2081, 0.0);
            s.store_scalar(2082, 0.0);
            s.store_scalar(2083, 0.0);
            s.store_scalar(2084, 0.0);
            s.store_scalar(2085, 0.0);
            s.store_scalar(2086, 0.0);
            s.store_scalar(2087, 0.0);
            s.store_scalar(2088, 0.0);
            s.store_scalar(2089, 0.0);
            s.store_scalar(2090, 0.0);
            s.store_scalar(2091, 0.0);
            s.store_scalar(2092, 0.0);
            s.store_scalar(2093, 0.0);
            s.store_scalar(2094, 0.0);
            s.store_scalar(2095, 0.0);
            s.store_scalar(2096, 0.0);
            s.store_scalar(2097, 0.0);
            s.store_scalar(2098, 0.0);
            s.store_scalar(2099, 0.0);
            s.store_scalar(2100, 0.0);
            s.store_scalar(2101, 0.0);
            s.store_scalar(2102, 0.0);
            s.store_scalar(2103, 0.0);
            s.store_scalar(2104, 0.0);
            s.store_scalar(2105, 0.0);
            s.store_scalar(2106, 0.0);
            s.store_scalar(2107, 0.0);
            s.store_scalar(2108, 0.0);
            s.store_scalar(2109, 0.0);
            s.store_scalar(2110, 0.0);
            s.store_scalar(2111, 0.0);
            s.store_mul_scaled_ad_lhs(2091, A::div(s.ad_value(2076), s.ad_value(2060)), 2077, -1.0);
        }

        if (s.b[1934] && s.b[2055]) {
            if ((!(s.v[2091] > 50.0)) && (!(s.v[2091] < (-50.0)))) {
                s.store_exp(2081, 2091);
            } else {
                if ((!(s.v[2091] > 50.0)) && (s.v[2091] < (-50.0))) {
                    s.store_scalar(2081, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2091] > 50.0) {
                        s.store_scaled_offset(2081, 2091, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2081, 0.0);
                    }
                }
            }
        }

        if (s.b[1934] && s.b[2055]) {
            s.store_add_scaled_product_right_ad(2087, 2091, 1.0, 2065, A::sub_scaled_inputs(s.ad_value(2059), -1.0, s.ad_value(2066), 1.0), 1.0);
            s.store_add_scaled_product_indices(2088, 2091, 1.0, 2065, 2066, -1.0);
        }

        if (s.b[1934] && s.b[2055]) {
            if ((!(s.v[2087] > 50.0)) && (!(s.v[2087] < (-50.0)))) {
                s.store_exp(2089, 2087);
            } else {
                if ((!(s.v[2087] > 50.0)) && (s.v[2087] < (-50.0))) {
                    s.store_scalar(2089, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2087] > 50.0) {
                        s.store_scaled_offset(2089, 2087, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2089, 0.0);
                    }
                }
            }
        }

        if (s.b[1934] && s.b[2055]) {
            if ((!(s.v[2088] > 50.0)) && (!(s.v[2088] < (-50.0)))) {
                s.store_exp(2090, 2088);
            } else {
                if ((!(s.v[2088] > 50.0)) && (s.v[2088] < (-50.0))) {
                    s.store_scalar(2090, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2088] > 50.0) {
                        s.store_scaled_offset(2090, 2088, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2090, 0.0);
                    }
                }
            }
        }

        if (s.b[1934] && s.b[2055]) {
            s.store_sub(2083, 2089, 2090);
            s.store_mul_ad_product_lhs(2057, A::mul3(s.ad_value(2078), s.ad_value(2068), s.ad_value(2069)), s.ad_value(2070), 2067);
            s.store_add_scaled_product_left_ad(2093, 2091, 1.0, A::div(s.ad_value(2064), s.ad_value(2060)), 2059, 1.0);
        }

        if (s.b[1934] && s.b[2055]) {
            if ((!(s.v[2093] > 50.0)) && (!(s.v[2093] < (-50.0)))) {
                s.store_exp(2094, 2093);
            } else {
                if ((!(s.v[2093] > 50.0)) && (s.v[2093] < (-50.0))) {
                    s.store_scalar(2094, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2093] > 50.0) {
                        s.store_scaled_offset(2094, 2093, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2094, 0.0);
                    }
                }
            }
        }

        s.b[2112] = (s.v[2063] == 1.0);
        s.v[2112] = if s.b[2112] { 1.0 } else { 0.0 };

        if ((s.b[1934] && s.b[2055]) && s.b[2112]) {
            s.store_mul_sub_ad_rhs(2084, 2057, A::add_scaled_product(s.ad_value(2094), 1.0, s.ad_value(2071), s.ad_value(2083), (-1.0)), s.ad_value(2081));
        }

        if ((s.b[1934] && s.b[2055]) && (!s.b[2112])) {
            s.store_add_scaled_product_right_ad(2098, 2091, 1.0, 2065, A::sub_scaled_inputs(s.ad_value(2061), -1.0, s.ad_value(2066), 1.0), 1.0);
        }

        if ((s.b[1934] && s.b[2055]) && (!s.b[2112])) {
            if ((!(s.v[2098] > 50.0)) && (!(s.v[2098] < (-50.0)))) {
                s.store_exp(2099, 2098);
            } else {
                if ((!(s.v[2098] > 50.0)) && (s.v[2098] < (-50.0))) {
                    s.store_scalar(2099, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2098] > 50.0) {
                        s.store_scaled_offset(2099, 2098, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2099, 0.0);
                    }
                }
            }
        }

        if ((s.b[1934] && s.b[2055]) && (!s.b[2112])) {
            s.store_sub(2100, 2099, 2090);
        }

    }

    pub(super) fn stamp_transient_block_28(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[1934] && s.b[2055]) && (!s.b[2112])) {
            s.store_add_scaled_product_left_ad(2101, 2091, 1.0, A::div(s.ad_value(2064), s.ad_value(2060)), 2061, 1.0);
        }

        if ((s.b[1934] && s.b[2055]) && (!s.b[2112])) {
            if ((!(s.v[2101] > 50.0)) && (!(s.v[2101] < (-50.0)))) {
                s.store_exp(2102, 2101);
            } else {
                if ((!(s.v[2101] > 50.0)) && (s.v[2101] < (-50.0))) {
                    s.store_scalar(2102, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2101] > 50.0) {
                        s.store_scaled_offset(2102, 2101, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2102, 0.0);
                    }
                }
            }
        }

        if ((s.b[1934] && s.b[2055]) && (!s.b[2112])) {
            s.store_sub_ad_lhs(2103, A::add_scaled_product(s.ad_value(2102), 1.0, s.ad_value(2071), s.ad_value(2100), (-1.0)), 2081);
            s.store_mul_sub_ad_rhs(2104, 2057, A::add_scaled_product(s.ad_value(2094), 1.0, s.ad_value(2071), s.ad_value(2083), (-1.0)), s.ad_value(2081));
        }

        s.b[2113] = (s.v[2063] > 0.0);
        s.v[2113] = if s.b[2113] { 1.0 } else { 0.0 };

        if (((s.b[1934] && s.b[2055]) && (!s.b[2112])) && s.b[2113]) {
            s.store_mul(2097, 2063, 2064);
            s.store_add_scaled_product_left_ad(2105, 2091, 1.0, A::div(s.ad_value(2097), s.ad_value(2060)), 2061, 1.0);
        }

        if (((s.b[1934] && s.b[2055]) && (!s.b[2112])) && s.b[2113]) {
            if ((!(s.v[2105] > 50.0)) && (!(s.v[2105] < (-50.0)))) {
                s.store_exp(2106, 2105);
            } else {
                if ((!(s.v[2105] > 50.0)) && (s.v[2105] < (-50.0))) {
                    s.store_scalar(2106, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2105] > 50.0) {
                        s.store_scaled_offset(2106, 2105, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2106, 0.0);
                    }
                }
            }
        }

        if (((s.b[1934] && s.b[2055]) && (!s.b[2112])) && s.b[2113]) {
            s.store_sub_ad_lhs(2107, A::add_scaled_product(s.ad_value(2106), 1.0, s.ad_value(2071), s.ad_value(2100), (-1.0)), 2081);
            s.store_add_scaled_product_left_ad(2108, 2091, 1.0, A::div(s.ad_value(2097), s.ad_value(2060)), 2059, 1.0);
        }

        if (((s.b[1934] && s.b[2055]) && (!s.b[2112])) && s.b[2113]) {
            if ((!(s.v[2108] > 50.0)) && (!(s.v[2108] < (-50.0)))) {
                s.store_exp(2109, 2108);
            } else {
                if ((!(s.v[2108] > 50.0)) && (s.v[2108] < (-50.0))) {
                    s.store_scalar(2109, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2108] > 50.0) {
                        s.store_scaled_offset(2109, 2108, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2109, 0.0);
                    }
                }
            }
        }

        if (((s.b[1934] && s.b[2055]) && (!s.b[2112])) && s.b[2113]) {
            s.store_div_scaled_product_indices(2110, 2057, 2103, 1.0, 2107, 1.0);
            s.store_mul_sub_ad_rhs(2111, 2110, A::add_scaled_product(s.ad_value(2109), 1.0, s.ad_value(2071), s.ad_value(2083), (-1.0)), s.ad_value(2081));
        }

        if (((s.b[1934] && s.b[2055]) && (!s.b[2112])) && (!s.b[2113])) {
            s.store_mul(2111, 2057, 2103);
        }

        if ((s.b[1934] && s.b[2055]) && (!s.b[2112])) {
            s.store_mul_square_lhs(2080, 2062, 2060);
            s.store_div_scaled_inputs3_indices(2092, 2059, 1.0, 2061, -1.0, 2080, (-(-0.5)), 2080, 1.0);
        }

        s.b[2114] = (s.v[2092] > 50.0);
        s.v[2114] = if s.b[2114] { 1.0 } else { 0.0 };

        if (((s.b[1934] && s.b[2055]) && (!s.b[2112])) && s.b[2114]) {
            s.store_scalar(2082, 0.0);
        }

        s.b[2115] = (s.v[2092] < (-50.0));
        s.v[2115] = if s.b[2115] { 1.0 } else { 0.0 };

        if ((((s.b[1934] && s.b[2055]) && (!s.b[2112])) && (!s.b[2114])) && s.b[2115]) {
            s.store_scalar(2082, 1.0);
        }

        if ((((s.b[1934] && s.b[2055]) && (!s.b[2112])) && (!s.b[2114])) && (!s.b[2115])) {
            s.store_div_from_scalar_offset_ad(2082, 1.0, A::exp(s.ad_value(2092)), 1.0);
        }

        if ((s.b[1934] && s.b[2055]) && (!s.b[2112])) {
            s.store_add_scaled_product_value_ad(2084, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(2082), s.ad_value(2111)), 1.0, 2082, 2104, 1.0);
        }

        if (s.b[1934] && s.b[2055]) {
            let assign36060_ad_e33123: A = {
                if (p.p52 != 0.0) {
                    A::mul(A::div(s.ad_value(2059), s.ad_value(2072)), A::tanh_scaled_input(A::div(s.ad_value(2059), s.ad_value(2072)), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt_square_offset(A::div(s.ad_value(2059), s.ad_value(2072)), p.p53)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs_mixed_ia(2085, 2059, -1.0, A::pow(A::offset(A::pow(assign36060_ad_e33123, s.ad_value(2073)), 1.0), A::div_from_scalar(1.0, s.ad_value(2073))), 1.0);
        }

        if (s.b[1934] && s.b[2055]) {
            s.store_mul_ad_product_lhs(2058, A::mul3_scaled_output(s.ad_value(2078), s.ad_value(2068), s.ad_value(2069), -1.0), s.ad_value(2074), 2067);
            s.store_mul_div_lhs(2095, 2075, 2060, 2085);
        }

        if (s.b[1934] && s.b[2055]) {
            if ((!(s.v[2095] > 50.0)) && (!(s.v[2095] < (-50.0)))) {
                s.store_exp(2096, 2095);
            } else {
                if ((!(s.v[2095] > 50.0)) && (s.v[2095] < (-50.0))) {
                    s.store_scalar(2096, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2095] > 50.0) {
                        s.store_scaled_offset(2096, 2095, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2096, 0.0);
                    }
                }
            }
        }

        if (s.b[1934] && s.b[2055]) {
            s.store_mul_offset_rhs(2086, 2058, 2096, (-1.0));
            s.store_add(2079, 2084, 2086);
            s.copy_ad(2056, 2079);
            s.copy_ad(134, 2056);
            s.store_scalar(2116, 0.0);
            s.store_scalar(2117, 0.0);
            s.store_scalar(2118, 0.0);
            s.store_scaled_voltage(2119, ctx, nodes, Some(8), Some(17), p.p6);
            s.copy_ad(2120, 113);
            s.store_scalar(2121, p.p265);
            s.store_scalar(2122, p.p267);
            s.store_scalar(2123, 1.0);
            s.store_scalar(2124, p.p263);
            s.store_scalar(2125, p.p281);
            s.store_scalar(2126, p.p280);
            s.copy_ad(2127, 112);
            s.store_scalar(2128, p.p0);
            s.store_scalar(2129, p.p2);
            s.store_scalar(2130, 0.0);
            s.store_scalar(2131, 0.0);
            s.store_scalar(2132, p.p289);
            s.store_scalar(2133, p.p290);
            s.store_scalar(2134, ((1.0 - p.p255) * p.p288));
            s.store_scalar(2135, p.p287);
            s.store_scalar(2136, p.p257);
            s.store_scalar(2137, p.p256);
            s.store_scalar(2138, p.p6);
            s.store_scalar(2139, 0.0);
            s.store_scalar(2140, 0.0);
            s.store_scalar(2141, 0.0);
            s.store_scalar(2142, 0.0);
            s.store_scalar(2143, 0.0);
            s.store_scalar(2144, 0.0);
            s.store_scalar(2145, 0.0);
            s.store_scalar(2146, 0.0);
            s.store_scalar(2147, 0.0);
            s.store_scalar(2148, 0.0);
            s.store_scalar(2149, 0.0);
            s.store_scalar(2150, 0.0);
            s.store_scalar(2151, 0.0);
            s.store_scalar(2152, 0.0);
            s.store_scalar(2153, 0.0);
            s.store_scalar(2154, 0.0);
            s.store_scalar(2155, 0.0);
            s.store_scalar(2156, 0.0);
            s.store_scalar(2157, 0.0);
            s.store_scalar(2158, 0.0);
            s.store_scalar(2159, 0.0);
            s.store_scalar(2160, 0.0);
            s.store_scalar(2161, 0.0);
            s.store_scalar(2162, 0.0);
            s.store_scalar(2163, 0.0);
            s.store_scalar(2164, 0.0);
            s.store_scalar(2165, 0.0);
            s.store_scalar(2166, 0.0);
            s.store_scalar(2167, 0.0);
            s.store_scalar(2168, 0.0);
            s.store_scalar(2169, 0.0);
            s.store_scalar(2170, 0.0);
            s.store_scalar(2171, 0.0);
            s.store_mul_scaled_ad_lhs(2151, A::div(s.ad_value(2136), s.ad_value(2120)), 2137, -1.0);
        }

        if (s.b[1934] && s.b[2055]) {
            if ((!(s.v[2151] > 50.0)) && (!(s.v[2151] < (-50.0)))) {
                s.store_exp(2141, 2151);
            } else {
                if ((!(s.v[2151] > 50.0)) && (s.v[2151] < (-50.0))) {
                    s.store_scalar(2141, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2151] > 50.0) {
                        s.store_scaled_offset(2141, 2151, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2141, 0.0);
                    }
                }
            }
        }

        if (s.b[1934] && s.b[2055]) {
            s.store_add_scaled_product_right_ad(2147, 2151, 1.0, 2125, A::sub_scaled_inputs(s.ad_value(2119), -1.0, s.ad_value(2126), 1.0), 1.0);
            s.store_add_scaled_product_indices(2148, 2151, 1.0, 2125, 2126, -1.0);
        }

        if (s.b[1934] && s.b[2055]) {
            if ((!(s.v[2147] > 50.0)) && (!(s.v[2147] < (-50.0)))) {
                s.store_exp(2149, 2147);
            } else {
                if ((!(s.v[2147] > 50.0)) && (s.v[2147] < (-50.0))) {
                    s.store_scalar(2149, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2147] > 50.0) {
                        s.store_scaled_offset(2149, 2147, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2149, 0.0);
                    }
                }
            }
        }

        if (s.b[1934] && s.b[2055]) {
            if ((!(s.v[2148] > 50.0)) && (!(s.v[2148] < (-50.0)))) {
                s.store_exp(2150, 2148);
            } else {
                if ((!(s.v[2148] > 50.0)) && (s.v[2148] < (-50.0))) {
                    s.store_scalar(2150, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2148] > 50.0) {
                        s.store_scaled_offset(2150, 2148, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2150, 0.0);
                    }
                }
            }
        }

    }

    pub(super) fn stamp_transient_block_29(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[1934] && s.b[2055]) {
            s.store_sub(2143, 2149, 2150);
            s.store_mul_ad_product_lhs(2117, A::mul3(s.ad_value(2138), s.ad_value(2128), s.ad_value(2129)), s.ad_value(2130), 2127);
            s.store_add_scaled_product_left_ad(2153, 2151, 1.0, A::div(s.ad_value(2124), s.ad_value(2120)), 2119, 1.0);
        }

        if (s.b[1934] && s.b[2055]) {
            if ((!(s.v[2153] > 50.0)) && (!(s.v[2153] < (-50.0)))) {
                s.store_exp(2154, 2153);
            } else {
                if ((!(s.v[2153] > 50.0)) && (s.v[2153] < (-50.0))) {
                    s.store_scalar(2154, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2153] > 50.0) {
                        s.store_scaled_offset(2154, 2153, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2154, 0.0);
                    }
                }
            }
        }

        s.b[2172] = (s.v[2123] == 1.0);
        s.v[2172] = if s.b[2172] { 1.0 } else { 0.0 };

        if ((s.b[1934] && s.b[2055]) && s.b[2172]) {
            s.store_mul_sub_ad_rhs(2144, 2117, A::add_scaled_product(s.ad_value(2154), 1.0, s.ad_value(2131), s.ad_value(2143), (-1.0)), s.ad_value(2141));
        }

        if ((s.b[1934] && s.b[2055]) && (!s.b[2172])) {
            s.store_add_scaled_product_right_ad(2158, 2151, 1.0, 2125, A::sub_scaled_inputs(s.ad_value(2121), -1.0, s.ad_value(2126), 1.0), 1.0);
        }

        if ((s.b[1934] && s.b[2055]) && (!s.b[2172])) {
            if ((!(s.v[2158] > 50.0)) && (!(s.v[2158] < (-50.0)))) {
                s.store_exp(2159, 2158);
            } else {
                if ((!(s.v[2158] > 50.0)) && (s.v[2158] < (-50.0))) {
                    s.store_scalar(2159, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2158] > 50.0) {
                        s.store_scaled_offset(2159, 2158, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2159, 0.0);
                    }
                }
            }
        }

        if ((s.b[1934] && s.b[2055]) && (!s.b[2172])) {
            s.store_sub(2160, 2159, 2150);
            s.store_add_scaled_product_left_ad(2161, 2151, 1.0, A::div(s.ad_value(2124), s.ad_value(2120)), 2121, 1.0);
        }

        if ((s.b[1934] && s.b[2055]) && (!s.b[2172])) {
            if ((!(s.v[2161] > 50.0)) && (!(s.v[2161] < (-50.0)))) {
                s.store_exp(2162, 2161);
            } else {
                if ((!(s.v[2161] > 50.0)) && (s.v[2161] < (-50.0))) {
                    s.store_scalar(2162, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2161] > 50.0) {
                        s.store_scaled_offset(2162, 2161, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2162, 0.0);
                    }
                }
            }
        }

        if ((s.b[1934] && s.b[2055]) && (!s.b[2172])) {
            s.store_sub_ad_lhs(2163, A::add_scaled_product(s.ad_value(2162), 1.0, s.ad_value(2131), s.ad_value(2160), (-1.0)), 2141);
            s.store_mul_sub_ad_rhs(2164, 2117, A::add_scaled_product(s.ad_value(2154), 1.0, s.ad_value(2131), s.ad_value(2143), (-1.0)), s.ad_value(2141));
        }

        s.b[2173] = (s.v[2123] > 0.0);
        s.v[2173] = if s.b[2173] { 1.0 } else { 0.0 };

        if (((s.b[1934] && s.b[2055]) && (!s.b[2172])) && s.b[2173]) {
            s.store_mul(2157, 2123, 2124);
            s.store_add_scaled_product_left_ad(2165, 2151, 1.0, A::div(s.ad_value(2157), s.ad_value(2120)), 2121, 1.0);
        }

        if (((s.b[1934] && s.b[2055]) && (!s.b[2172])) && s.b[2173]) {
            if ((!(s.v[2165] > 50.0)) && (!(s.v[2165] < (-50.0)))) {
                s.store_exp(2166, 2165);
            } else {
                if ((!(s.v[2165] > 50.0)) && (s.v[2165] < (-50.0))) {
                    s.store_scalar(2166, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2165] > 50.0) {
                        s.store_scaled_offset(2166, 2165, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2166, 0.0);
                    }
                }
            }
        }

        if (((s.b[1934] && s.b[2055]) && (!s.b[2172])) && s.b[2173]) {
            s.store_sub_ad_lhs(2167, A::add_scaled_product(s.ad_value(2166), 1.0, s.ad_value(2131), s.ad_value(2160), (-1.0)), 2141);
            s.store_add_scaled_product_left_ad(2168, 2151, 1.0, A::div(s.ad_value(2157), s.ad_value(2120)), 2119, 1.0);
        }

        if (((s.b[1934] && s.b[2055]) && (!s.b[2172])) && s.b[2173]) {
            if ((!(s.v[2168] > 50.0)) && (!(s.v[2168] < (-50.0)))) {
                s.store_exp(2169, 2168);
            } else {
                if ((!(s.v[2168] > 50.0)) && (s.v[2168] < (-50.0))) {
                    s.store_scalar(2169, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2168] > 50.0) {
                        s.store_scaled_offset(2169, 2168, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2169, 0.0);
                    }
                }
            }
        }

        if (((s.b[1934] && s.b[2055]) && (!s.b[2172])) && s.b[2173]) {
            s.store_div_scaled_product_indices(2170, 2117, 2163, 1.0, 2167, 1.0);
            s.store_mul_sub_ad_rhs(2171, 2170, A::add_scaled_product(s.ad_value(2169), 1.0, s.ad_value(2131), s.ad_value(2143), (-1.0)), s.ad_value(2141));
        }

        if (((s.b[1934] && s.b[2055]) && (!s.b[2172])) && (!s.b[2173])) {
            s.store_mul(2171, 2117, 2163);
        }

        if ((s.b[1934] && s.b[2055]) && (!s.b[2172])) {
            s.store_mul_square_lhs(2140, 2122, 2120);
            s.store_div_scaled_inputs3_indices(2152, 2119, 1.0, 2121, -1.0, 2140, (-(-0.5)), 2140, 1.0);
        }

        s.b[2174] = (s.v[2152] > 50.0);
        s.v[2174] = if s.b[2174] { 1.0 } else { 0.0 };

        if (((s.b[1934] && s.b[2055]) && (!s.b[2172])) && s.b[2174]) {
            s.store_scalar(2142, 0.0);
        }

        s.b[2175] = (s.v[2152] < (-50.0));
        s.v[2175] = if s.b[2175] { 1.0 } else { 0.0 };

        if ((((s.b[1934] && s.b[2055]) && (!s.b[2172])) && (!s.b[2174])) && s.b[2175]) {
            s.store_scalar(2142, 1.0);
        }

        if ((((s.b[1934] && s.b[2055]) && (!s.b[2172])) && (!s.b[2174])) && (!s.b[2175])) {
            s.store_div_from_scalar_offset_ad(2142, 1.0, A::exp(s.ad_value(2152)), 1.0);
        }

        if ((s.b[1934] && s.b[2055]) && (!s.b[2172])) {
            s.store_add_scaled_product_value_ad(2144, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(2142), s.ad_value(2171)), 1.0, 2142, 2164, 1.0);
        }

        if (s.b[1934] && s.b[2055]) {
            let assign37090_ad_e34370: A = {
                if (p.p52 != 0.0) {
                    A::mul(A::div(s.ad_value(2119), s.ad_value(2132)), A::tanh_scaled_input(A::div(s.ad_value(2119), s.ad_value(2132)), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt_square_offset(A::div(s.ad_value(2119), s.ad_value(2132)), p.p53)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs_mixed_ia(2145, 2119, -1.0, A::pow(A::offset(A::pow(assign37090_ad_e34370, s.ad_value(2133)), 1.0), A::div_from_scalar(1.0, s.ad_value(2133))), 1.0);
        }

        if (s.b[1934] && s.b[2055]) {
            s.store_mul_ad_product_lhs(2118, A::mul3_scaled_output(s.ad_value(2138), s.ad_value(2128), s.ad_value(2129), -1.0), s.ad_value(2134), 2127);
            s.store_mul_div_lhs(2155, 2135, 2120, 2145);
        }

        if (s.b[1934] && s.b[2055]) {
            if ((!(s.v[2155] > 50.0)) && (!(s.v[2155] < (-50.0)))) {
                s.store_exp(2156, 2155);
            } else {
                if ((!(s.v[2155] > 50.0)) && (s.v[2155] < (-50.0))) {
                    s.store_scalar(2156, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2155] > 50.0) {
                        s.store_scaled_offset(2156, 2155, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2156, 0.0);
                    }
                }
            }
        }

        if (s.b[1934] && s.b[2055]) {
            s.store_mul_offset_rhs(2146, 2118, 2156, (-1.0));
            s.store_add(2139, 2144, 2146);
            s.copy_ad(2116, 2139);
            s.copy_ad(135, 2116);
        }

        s.b[2176] = (p.p255 != 0.0);
        s.v[2176] = if s.b[2176] { 1.0 } else { 0.0 };

        if (s.b[1934] && s.b[2176]) {
            s.store_scalar(2177, 0.0);
            s.store_scalar(2178, 0.0);
            s.store_scalar(2179, 0.0);
            s.store_scaled_voltage(2180, ctx, nodes, Some(8), Some(9), p.p6);
            s.copy_ad(2181, 113);
            s.store_scalar(2182, p.p260);
            s.store_scalar(2183, p.p262);
            s.store_scalar(2184, p.p261);
            s.store_scalar(2185, p.p258);
            s.store_scalar(2186, p.p278);
            s.store_scalar(2187, p.p277);
            s.copy_ad(2188, 112);
            s.store_scalar(2189, p.p0);
            s.store_scalar(2190, p.p2);
            s.store_scalar(2191, (p.p255 * p.p259));
            s.store_scalar(2192, p.p276);
            s.store_scalar(2193, p.p270);
            s.store_scalar(2194, p.p271);
            s.store_scalar(2195, (p.p255 * p.p269));
            s.store_scalar(2196, p.p268);
            s.store_scalar(2197, p.p257);
            s.store_scalar(2198, p.p256);
            s.store_scalar(2199, p.p6);
            s.store_scalar(2200, 0.0);
            s.store_scalar(2201, 0.0);
            s.store_scalar(2202, 0.0);
            s.store_scalar(2203, 0.0);
            s.store_scalar(2204, 0.0);
            s.store_scalar(2205, 0.0);
            s.store_scalar(2206, 0.0);
            s.store_scalar(2207, 0.0);
            s.store_scalar(2208, 0.0);
            s.store_scalar(2209, 0.0);
            s.store_scalar(2210, 0.0);
            s.store_scalar(2211, 0.0);
            s.store_scalar(2212, 0.0);
            s.store_scalar(2213, 0.0);
            s.store_scalar(2214, 0.0);
            s.store_scalar(2215, 0.0);
            s.store_scalar(2216, 0.0);
            s.store_scalar(2217, 0.0);
            s.store_scalar(2218, 0.0);
            s.store_scalar(2219, 0.0);
            s.store_scalar(2220, 0.0);
            s.store_scalar(2221, 0.0);
            s.store_scalar(2222, 0.0);
            s.store_scalar(2223, 0.0);
            s.store_scalar(2224, 0.0);
            s.store_scalar(2225, 0.0);
            s.store_scalar(2226, 0.0);
            s.store_scalar(2227, 0.0);
            s.store_scalar(2228, 0.0);
            s.store_scalar(2229, 0.0);
            s.store_scalar(2230, 0.0);
            s.store_scalar(2231, 0.0);
            s.store_scalar(2232, 0.0);
            s.store_mul_scaled_ad_lhs(2212, A::div(s.ad_value(2197), s.ad_value(2181)), 2198, -1.0);
        }

    }

    pub(super) fn stamp_transient_block_30(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[1934] && s.b[2176]) {
            if ((!(s.v[2212] > 50.0)) && (!(s.v[2212] < (-50.0)))) {
                s.store_exp(2202, 2212);
            } else {
                if ((!(s.v[2212] > 50.0)) && (s.v[2212] < (-50.0))) {
                    s.store_scalar(2202, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2212] > 50.0) {
                        s.store_scaled_offset(2202, 2212, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2202, 0.0);
                    }
                }
            }
        }

        if (s.b[1934] && s.b[2176]) {
            s.store_add_scaled_product_right_ad(2208, 2212, 1.0, 2186, A::sub_scaled_inputs(s.ad_value(2180), -1.0, s.ad_value(2187), 1.0), 1.0);
            s.store_add_scaled_product_indices(2209, 2212, 1.0, 2186, 2187, -1.0);
        }

        if (s.b[1934] && s.b[2176]) {
            if ((!(s.v[2208] > 50.0)) && (!(s.v[2208] < (-50.0)))) {
                s.store_exp(2210, 2208);
            } else {
                if ((!(s.v[2208] > 50.0)) && (s.v[2208] < (-50.0))) {
                    s.store_scalar(2210, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2208] > 50.0) {
                        s.store_scaled_offset(2210, 2208, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2210, 0.0);
                    }
                }
            }
        }

        if (s.b[1934] && s.b[2176]) {
            if ((!(s.v[2209] > 50.0)) && (!(s.v[2209] < (-50.0)))) {
                s.store_exp(2211, 2209);
            } else {
                if ((!(s.v[2209] > 50.0)) && (s.v[2209] < (-50.0))) {
                    s.store_scalar(2211, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2209] > 50.0) {
                        s.store_scaled_offset(2211, 2209, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2211, 0.0);
                    }
                }
            }
        }

        if (s.b[1934] && s.b[2176]) {
            s.store_sub(2204, 2210, 2211);
            s.store_mul_ad_product_lhs(2178, A::mul3(s.ad_value(2199), s.ad_value(2189), s.ad_value(2190)), s.ad_value(2191), 2188);
            s.store_add_scaled_product_left_ad(2214, 2212, 1.0, A::div(s.ad_value(2185), s.ad_value(2181)), 2180, 1.0);
        }

        if (s.b[1934] && s.b[2176]) {
            if ((!(s.v[2214] > 50.0)) && (!(s.v[2214] < (-50.0)))) {
                s.store_exp(2215, 2214);
            } else {
                if ((!(s.v[2214] > 50.0)) && (s.v[2214] < (-50.0))) {
                    s.store_scalar(2215, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2214] > 50.0) {
                        s.store_scaled_offset(2215, 2214, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2215, 0.0);
                    }
                }
            }
        }

        s.b[2233] = (s.v[2184] == 1.0);
        s.v[2233] = if s.b[2233] { 1.0 } else { 0.0 };

        if ((s.b[1934] && s.b[2176]) && s.b[2233]) {
            s.store_mul_sub_ad_rhs(2205, 2178, A::add_scaled_product(s.ad_value(2215), 1.0, s.ad_value(2192), s.ad_value(2204), (-1.0)), s.ad_value(2202));
        }

        if ((s.b[1934] && s.b[2176]) && (!s.b[2233])) {
            s.store_add_scaled_product_right_ad(2219, 2212, 1.0, 2186, A::sub_scaled_inputs(s.ad_value(2182), -1.0, s.ad_value(2187), 1.0), 1.0);
        }

        if ((s.b[1934] && s.b[2176]) && (!s.b[2233])) {
            if ((!(s.v[2219] > 50.0)) && (!(s.v[2219] < (-50.0)))) {
                s.store_exp(2220, 2219);
            } else {
                if ((!(s.v[2219] > 50.0)) && (s.v[2219] < (-50.0))) {
                    s.store_scalar(2220, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2219] > 50.0) {
                        s.store_scaled_offset(2220, 2219, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2220, 0.0);
                    }
                }
            }
        }

        if ((s.b[1934] && s.b[2176]) && (!s.b[2233])) {
            s.store_sub(2221, 2220, 2211);
            s.store_add_scaled_product_left_ad(2222, 2212, 1.0, A::div(s.ad_value(2185), s.ad_value(2181)), 2182, 1.0);
        }

        if ((s.b[1934] && s.b[2176]) && (!s.b[2233])) {
            if ((!(s.v[2222] > 50.0)) && (!(s.v[2222] < (-50.0)))) {
                s.store_exp(2223, 2222);
            } else {
                if ((!(s.v[2222] > 50.0)) && (s.v[2222] < (-50.0))) {
                    s.store_scalar(2223, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2222] > 50.0) {
                        s.store_scaled_offset(2223, 2222, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2223, 0.0);
                    }
                }
            }
        }

        if ((s.b[1934] && s.b[2176]) && (!s.b[2233])) {
            s.store_sub_ad_lhs(2224, A::add_scaled_product(s.ad_value(2223), 1.0, s.ad_value(2192), s.ad_value(2221), (-1.0)), 2202);
            s.store_mul_sub_ad_rhs(2225, 2178, A::add_scaled_product(s.ad_value(2215), 1.0, s.ad_value(2192), s.ad_value(2204), (-1.0)), s.ad_value(2202));
        }

        s.b[2234] = (s.v[2184] > 0.0);
        s.v[2234] = if s.b[2234] { 1.0 } else { 0.0 };

        if (((s.b[1934] && s.b[2176]) && (!s.b[2233])) && s.b[2234]) {
            s.store_mul(2218, 2184, 2185);
            s.store_add_scaled_product_left_ad(2226, 2212, 1.0, A::div(s.ad_value(2218), s.ad_value(2181)), 2182, 1.0);
        }

        if (((s.b[1934] && s.b[2176]) && (!s.b[2233])) && s.b[2234]) {
            if ((!(s.v[2226] > 50.0)) && (!(s.v[2226] < (-50.0)))) {
                s.store_exp(2227, 2226);
            } else {
                if ((!(s.v[2226] > 50.0)) && (s.v[2226] < (-50.0))) {
                    s.store_scalar(2227, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2226] > 50.0) {
                        s.store_scaled_offset(2227, 2226, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2227, 0.0);
                    }
                }
            }
        }

        if (((s.b[1934] && s.b[2176]) && (!s.b[2233])) && s.b[2234]) {
            s.store_sub_ad_lhs(2228, A::add_scaled_product(s.ad_value(2227), 1.0, s.ad_value(2192), s.ad_value(2221), (-1.0)), 2202);
            s.store_add_scaled_product_left_ad(2229, 2212, 1.0, A::div(s.ad_value(2218), s.ad_value(2181)), 2180, 1.0);
        }

        if (((s.b[1934] && s.b[2176]) && (!s.b[2233])) && s.b[2234]) {
            if ((!(s.v[2229] > 50.0)) && (!(s.v[2229] < (-50.0)))) {
                s.store_exp(2230, 2229);
            } else {
                if ((!(s.v[2229] > 50.0)) && (s.v[2229] < (-50.0))) {
                    s.store_scalar(2230, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2229] > 50.0) {
                        s.store_scaled_offset(2230, 2229, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2230, 0.0);
                    }
                }
            }
        }

        if (((s.b[1934] && s.b[2176]) && (!s.b[2233])) && s.b[2234]) {
            s.store_div_scaled_product_indices(2231, 2178, 2224, 1.0, 2228, 1.0);
            s.store_mul_sub_ad_rhs(2232, 2231, A::add_scaled_product(s.ad_value(2230), 1.0, s.ad_value(2192), s.ad_value(2204), (-1.0)), s.ad_value(2202));
        }

        if (((s.b[1934] && s.b[2176]) && (!s.b[2233])) && (!s.b[2234])) {
            s.store_mul(2232, 2178, 2224);
        }

        if ((s.b[1934] && s.b[2176]) && (!s.b[2233])) {
            s.store_mul_square_lhs(2201, 2183, 2181);
            s.store_div_scaled_inputs3_indices(2213, 2180, 1.0, 2182, -1.0, 2201, (-(-0.5)), 2201, 1.0);
        }

        s.b[2235] = (s.v[2213] > 50.0);
        s.v[2235] = if s.b[2235] { 1.0 } else { 0.0 };

        if (((s.b[1934] && s.b[2176]) && (!s.b[2233])) && s.b[2235]) {
            s.store_scalar(2203, 0.0);
        }

        s.b[2236] = (s.v[2213] < (-50.0));
        s.v[2236] = if s.b[2236] { 1.0 } else { 0.0 };

        if ((((s.b[1934] && s.b[2176]) && (!s.b[2233])) && (!s.b[2235])) && s.b[2236]) {
            s.store_scalar(2203, 1.0);
        }

        if ((((s.b[1934] && s.b[2176]) && (!s.b[2233])) && (!s.b[2235])) && (!s.b[2236])) {
            s.store_div_from_scalar_offset_ad(2203, 1.0, A::exp(s.ad_value(2213)), 1.0);
        }

        if ((s.b[1934] && s.b[2176]) && (!s.b[2233])) {
            s.store_add_scaled_product_value_ad(2205, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(2203), s.ad_value(2232)), 1.0, 2203, 2225, 1.0);
        }

        if (s.b[1934] && s.b[2176]) {
            let assign38130_ad_e35620: A = {
                if (p.p52 != 0.0) {
                    A::mul(A::div(s.ad_value(2180), s.ad_value(2193)), A::tanh_scaled_input(A::div(s.ad_value(2180), s.ad_value(2193)), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt_square_offset(A::div(s.ad_value(2180), s.ad_value(2193)), p.p53)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs_mixed_ia(2206, 2180, -1.0, A::pow(A::offset(A::pow(assign38130_ad_e35620, s.ad_value(2194)), 1.0), A::div_from_scalar(1.0, s.ad_value(2194))), 1.0);
        }

        if (s.b[1934] && s.b[2176]) {
            s.store_mul_ad_product_lhs(2179, A::mul3_scaled_output(s.ad_value(2199), s.ad_value(2189), s.ad_value(2190), -1.0), s.ad_value(2195), 2188);
            s.store_mul_div_lhs(2216, 2196, 2181, 2206);
        }

        if (s.b[1934] && s.b[2176]) {
            if ((!(s.v[2216] > 50.0)) && (!(s.v[2216] < (-50.0)))) {
                s.store_exp(2217, 2216);
            } else {
                if ((!(s.v[2216] > 50.0)) && (s.v[2216] < (-50.0))) {
                    s.store_scalar(2217, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2216] > 50.0) {
                        s.store_scaled_offset(2217, 2216, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2217, 0.0);
                    }
                }
            }
        }

        if (s.b[1934] && s.b[2176]) {
            s.store_mul_offset_rhs(2207, 2179, 2217, (-1.0));
            s.store_add(2200, 2205, 2207);
            s.copy_ad(2177, 2200);
            s.copy_ad(128, 2177);
            s.store_scalar(2237, 0.0);
            s.store_scalar(2238, 0.0);
            s.store_scalar(2239, 0.0);
            s.store_scaled_voltage(2240, ctx, nodes, Some(8), Some(5), p.p6);
            s.copy_ad(2241, 113);
            s.store_scalar(2242, p.p265);
            s.store_scalar(2243, p.p267);
            s.store_scalar(2244, p.p266);
            s.store_scalar(2245, p.p263);
            s.store_scalar(2246, p.p281);
            s.store_scalar(2247, p.p280);
            s.copy_ad(2248, 112);
            s.store_scalar(2249, p.p0);
            s.store_scalar(2250, p.p2);
            s.store_scalar(2251, (p.p255 * p.p264));
            s.store_scalar(2252, p.p279);
            s.store_scalar(2253, p.p274);
            s.store_scalar(2254, p.p275);
            s.store_scalar(2255, (p.p255 * p.p273));
            s.store_scalar(2256, p.p272);
            s.store_scalar(2257, p.p257);
            s.store_scalar(2258, p.p256);
            s.store_scalar(2259, p.p6);
            s.store_scalar(2260, 0.0);
            s.store_scalar(2261, 0.0);
            s.store_scalar(2262, 0.0);
            s.store_scalar(2263, 0.0);
            s.store_scalar(2264, 0.0);
            s.store_scalar(2265, 0.0);
            s.store_scalar(2266, 0.0);
            s.store_scalar(2267, 0.0);
            s.store_scalar(2268, 0.0);
            s.store_scalar(2269, 0.0);
            s.store_scalar(2270, 0.0);
            s.store_scalar(2271, 0.0);
            s.store_scalar(2272, 0.0);
            s.store_scalar(2273, 0.0);
            s.store_scalar(2274, 0.0);
            s.store_scalar(2275, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_31(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[1934] && s.b[2176]) {
            s.store_scalar(2276, 0.0);
            s.store_scalar(2277, 0.0);
            s.store_scalar(2278, 0.0);
            s.store_scalar(2279, 0.0);
            s.store_scalar(2280, 0.0);
            s.store_scalar(2281, 0.0);
            s.store_scalar(2282, 0.0);
            s.store_scalar(2283, 0.0);
            s.store_scalar(2284, 0.0);
            s.store_scalar(2285, 0.0);
            s.store_scalar(2286, 0.0);
            s.store_scalar(2287, 0.0);
            s.store_scalar(2288, 0.0);
            s.store_scalar(2289, 0.0);
            s.store_scalar(2290, 0.0);
            s.store_scalar(2291, 0.0);
            s.store_scalar(2292, 0.0);
            s.store_mul_scaled_ad_lhs(2272, A::div(s.ad_value(2257), s.ad_value(2241)), 2258, -1.0);
        }

        if (s.b[1934] && s.b[2176]) {
            if ((!(s.v[2272] > 50.0)) && (!(s.v[2272] < (-50.0)))) {
                s.store_exp(2262, 2272);
            } else {
                if ((!(s.v[2272] > 50.0)) && (s.v[2272] < (-50.0))) {
                    s.store_scalar(2262, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2272] > 50.0) {
                        s.store_scaled_offset(2262, 2272, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2262, 0.0);
                    }
                }
            }
        }

        if (s.b[1934] && s.b[2176]) {
            s.store_add_scaled_product_right_ad(2268, 2272, 1.0, 2246, A::sub_scaled_inputs(s.ad_value(2240), -1.0, s.ad_value(2247), 1.0), 1.0);
            s.store_add_scaled_product_indices(2269, 2272, 1.0, 2246, 2247, -1.0);
        }

        if (s.b[1934] && s.b[2176]) {
            if ((!(s.v[2268] > 50.0)) && (!(s.v[2268] < (-50.0)))) {
                s.store_exp(2270, 2268);
            } else {
                if ((!(s.v[2268] > 50.0)) && (s.v[2268] < (-50.0))) {
                    s.store_scalar(2270, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2268] > 50.0) {
                        s.store_scaled_offset(2270, 2268, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2270, 0.0);
                    }
                }
            }
        }

        if (s.b[1934] && s.b[2176]) {
            if ((!(s.v[2269] > 50.0)) && (!(s.v[2269] < (-50.0)))) {
                s.store_exp(2271, 2269);
            } else {
                if ((!(s.v[2269] > 50.0)) && (s.v[2269] < (-50.0))) {
                    s.store_scalar(2271, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2269] > 50.0) {
                        s.store_scaled_offset(2271, 2269, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2271, 0.0);
                    }
                }
            }
        }

        if (s.b[1934] && s.b[2176]) {
            s.store_sub(2264, 2270, 2271);
            s.store_mul_ad_product_lhs(2238, A::mul3(s.ad_value(2259), s.ad_value(2249), s.ad_value(2250)), s.ad_value(2251), 2248);
            s.store_add_scaled_product_left_ad(2274, 2272, 1.0, A::div(s.ad_value(2245), s.ad_value(2241)), 2240, 1.0);
        }

        if (s.b[1934] && s.b[2176]) {
            if ((!(s.v[2274] > 50.0)) && (!(s.v[2274] < (-50.0)))) {
                s.store_exp(2275, 2274);
            } else {
                if ((!(s.v[2274] > 50.0)) && (s.v[2274] < (-50.0))) {
                    s.store_scalar(2275, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2274] > 50.0) {
                        s.store_scaled_offset(2275, 2274, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2275, 0.0);
                    }
                }
            }
        }

        s.b[2293] = (s.v[2244] == 1.0);
        s.v[2293] = if s.b[2293] { 1.0 } else { 0.0 };

        if ((s.b[1934] && s.b[2176]) && s.b[2293]) {
            s.store_mul_sub_ad_rhs(2265, 2238, A::add_scaled_product(s.ad_value(2275), 1.0, s.ad_value(2252), s.ad_value(2264), (-1.0)), s.ad_value(2262));
        }

        if ((s.b[1934] && s.b[2176]) && (!s.b[2293])) {
            s.store_add_scaled_product_right_ad(2279, 2272, 1.0, 2246, A::sub_scaled_inputs(s.ad_value(2242), -1.0, s.ad_value(2247), 1.0), 1.0);
        }

        if ((s.b[1934] && s.b[2176]) && (!s.b[2293])) {
            if ((!(s.v[2279] > 50.0)) && (!(s.v[2279] < (-50.0)))) {
                s.store_exp(2280, 2279);
            } else {
                if ((!(s.v[2279] > 50.0)) && (s.v[2279] < (-50.0))) {
                    s.store_scalar(2280, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2279] > 50.0) {
                        s.store_scaled_offset(2280, 2279, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2280, 0.0);
                    }
                }
            }
        }

        if ((s.b[1934] && s.b[2176]) && (!s.b[2293])) {
            s.store_sub(2281, 2280, 2271);
            s.store_add_scaled_product_left_ad(2282, 2272, 1.0, A::div(s.ad_value(2245), s.ad_value(2241)), 2242, 1.0);
        }

        if ((s.b[1934] && s.b[2176]) && (!s.b[2293])) {
            if ((!(s.v[2282] > 50.0)) && (!(s.v[2282] < (-50.0)))) {
                s.store_exp(2283, 2282);
            } else {
                if ((!(s.v[2282] > 50.0)) && (s.v[2282] < (-50.0))) {
                    s.store_scalar(2283, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2282] > 50.0) {
                        s.store_scaled_offset(2283, 2282, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2283, 0.0);
                    }
                }
            }
        }

        if ((s.b[1934] && s.b[2176]) && (!s.b[2293])) {
            s.store_sub_ad_lhs(2284, A::add_scaled_product(s.ad_value(2283), 1.0, s.ad_value(2252), s.ad_value(2281), (-1.0)), 2262);
            s.store_mul_sub_ad_rhs(2285, 2238, A::add_scaled_product(s.ad_value(2275), 1.0, s.ad_value(2252), s.ad_value(2264), (-1.0)), s.ad_value(2262));
        }

        s.b[2294] = (s.v[2244] > 0.0);
        s.v[2294] = if s.b[2294] { 1.0 } else { 0.0 };

        if (((s.b[1934] && s.b[2176]) && (!s.b[2293])) && s.b[2294]) {
            s.store_mul(2278, 2244, 2245);
            s.store_add_scaled_product_left_ad(2286, 2272, 1.0, A::div(s.ad_value(2278), s.ad_value(2241)), 2242, 1.0);
        }

        if (((s.b[1934] && s.b[2176]) && (!s.b[2293])) && s.b[2294]) {
            if ((!(s.v[2286] > 50.0)) && (!(s.v[2286] < (-50.0)))) {
                s.store_exp(2287, 2286);
            } else {
                if ((!(s.v[2286] > 50.0)) && (s.v[2286] < (-50.0))) {
                    s.store_scalar(2287, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2286] > 50.0) {
                        s.store_scaled_offset(2287, 2286, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2287, 0.0);
                    }
                }
            }
        }

        if (((s.b[1934] && s.b[2176]) && (!s.b[2293])) && s.b[2294]) {
            s.store_sub_ad_lhs(2288, A::add_scaled_product(s.ad_value(2287), 1.0, s.ad_value(2252), s.ad_value(2281), (-1.0)), 2262);
            s.store_add_scaled_product_left_ad(2289, 2272, 1.0, A::div(s.ad_value(2278), s.ad_value(2241)), 2240, 1.0);
        }

        if (((s.b[1934] && s.b[2176]) && (!s.b[2293])) && s.b[2294]) {
            if ((!(s.v[2289] > 50.0)) && (!(s.v[2289] < (-50.0)))) {
                s.store_exp(2290, 2289);
            } else {
                if ((!(s.v[2289] > 50.0)) && (s.v[2289] < (-50.0))) {
                    s.store_scalar(2290, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2289] > 50.0) {
                        s.store_scaled_offset(2290, 2289, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2290, 0.0);
                    }
                }
            }
        }

        if (((s.b[1934] && s.b[2176]) && (!s.b[2293])) && s.b[2294]) {
            s.store_div_scaled_product_indices(2291, 2238, 2284, 1.0, 2288, 1.0);
            s.store_mul_sub_ad_rhs(2292, 2291, A::add_scaled_product(s.ad_value(2290), 1.0, s.ad_value(2252), s.ad_value(2264), (-1.0)), s.ad_value(2262));
        }

        if (((s.b[1934] && s.b[2176]) && (!s.b[2293])) && (!s.b[2294])) {
            s.store_mul(2292, 2238, 2284);
        }

        if ((s.b[1934] && s.b[2176]) && (!s.b[2293])) {
            s.store_mul_square_lhs(2261, 2243, 2241);
            s.store_div_scaled_inputs3_indices(2273, 2240, 1.0, 2242, -1.0, 2261, (-(-0.5)), 2261, 1.0);
        }

        s.b[2295] = (s.v[2273] > 50.0);
        s.v[2295] = if s.b[2295] { 1.0 } else { 0.0 };

        if (((s.b[1934] && s.b[2176]) && (!s.b[2293])) && s.b[2295]) {
            s.store_scalar(2263, 0.0);
        }

        s.b[2296] = (s.v[2273] < (-50.0));
        s.v[2296] = if s.b[2296] { 1.0 } else { 0.0 };

        if ((((s.b[1934] && s.b[2176]) && (!s.b[2293])) && (!s.b[2295])) && s.b[2296]) {
            s.store_scalar(2263, 1.0);
        }

        if ((((s.b[1934] && s.b[2176]) && (!s.b[2293])) && (!s.b[2295])) && (!s.b[2296])) {
            s.store_div_from_scalar_offset_ad(2263, 1.0, A::exp(s.ad_value(2273)), 1.0);
        }

        if ((s.b[1934] && s.b[2176]) && (!s.b[2293])) {
            s.store_add_scaled_product_value_ad(2265, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(2263), s.ad_value(2292)), 1.0, 2263, 2285, 1.0);
        }

        if (s.b[1934] && s.b[2176]) {
            let assign39160_ad_e36867: A = {
                if (p.p52 != 0.0) {
                    A::mul(A::div(s.ad_value(2240), s.ad_value(2253)), A::tanh_scaled_input(A::div(s.ad_value(2240), s.ad_value(2253)), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt_square_offset(A::div(s.ad_value(2240), s.ad_value(2253)), p.p53)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs_mixed_ia(2266, 2240, -1.0, A::pow(A::offset(A::pow(assign39160_ad_e36867, s.ad_value(2254)), 1.0), A::div_from_scalar(1.0, s.ad_value(2254))), 1.0);
        }

        if (s.b[1934] && s.b[2176]) {
            s.store_mul_ad_product_lhs(2239, A::mul3_scaled_output(s.ad_value(2259), s.ad_value(2249), s.ad_value(2250), -1.0), s.ad_value(2255), 2248);
            s.store_mul_div_lhs(2276, 2256, 2241, 2266);
        }

        if (s.b[1934] && s.b[2176]) {
            if ((!(s.v[2276] > 50.0)) && (!(s.v[2276] < (-50.0)))) {
                s.store_exp(2277, 2276);
            } else {
                if ((!(s.v[2276] > 50.0)) && (s.v[2276] < (-50.0))) {
                    s.store_scalar(2277, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2276] > 50.0) {
                        s.store_scaled_offset(2277, 2276, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2277, 0.0);
                    }
                }
            }
        }

        if (s.b[1934] && s.b[2176]) {
            s.store_mul_offset_rhs(2267, 2239, 2277, (-1.0));
            s.store_add(2260, 2265, 2267);
            s.copy_ad(2237, 2260);
            s.copy_ad(129, 2237);
        }

        s.b[2297] = (p.p282 == 1.0);
        s.v[2297] = if s.b[2297] { 1.0 } else { 0.0 };

        if ((s.b[1934] && s.b[2176]) && s.b[2297]) {
            s.store_scalar(2298, 0.0);
            s.store_scalar(2299, 0.0);
            s.store_scalar(2300, 0.0);
            s.store_scaled_voltage(2301, ctx, nodes, Some(8), Some(9), p.p6);
            s.copy_ad(2302, 113);
            s.store_scalar(2303, p.p260);
            s.store_scalar(2304, p.p262);
            s.store_scalar(2305, 1.0);
            s.store_scalar(2306, p.p258);
            s.store_scalar(2307, p.p278);
            s.store_scalar(2308, p.p277);
            s.copy_ad(2309, 112);
            s.store_scalar(2310, p.p0);
            s.store_scalar(2311, p.p2);
            s.store_scalar(2312, 0.0);
            s.store_scalar(2313, 0.0);
            s.store_scalar(2314, p.p285);
            s.store_scalar(2315, p.p286);
            s.store_scalar(2316, (p.p255 * p.p284));
            s.store_scalar(2317, p.p283);
        }

    }
}
