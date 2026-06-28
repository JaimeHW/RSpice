#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_mul(1179, 502, 1181);

        s.store_mul(1239, 1179, 1170);

        s.store_sqrt_offset_scaled_input(1179, 498, 1.0 / (s.v[1227]), 1.0);

        s.store_add_scaled_inputs_product_indices(1180, 491, 1.0, 492, 1.0 / (s.v[1227]), 493, 1177, 1.0);

        s.store_add_scaled_product_value_ad(1238, A::mul3(s.ad_value(737), A::offset(s.ad_value(1179), (-1.0)), s.ad_value(1278)), 1.0, 1180, 772, 1.0);

        s.store_div_scaled_product_offset_denominator(1205, s.ad_value(776), s.ad_value(1277), 1.0, s.ad_value(497), s.v[689], 1.0);

        s.store_add_scaled_product_indices(1182, 761, 1.0, 557, 1177, 1.0);

        s.b[1570] = (s.v[1182] < 0.0001);
        s.v[1570] = if s.b[1570] { 1.0 } else { 0.0 };

        if s.b[1570] {
            s.store_div_from_scalar_sub_from_scalar_ad(1188, 1.0, 3.0, A::scale(s.ad_value(1182), 20000.0));
            s.store_mul_sub_from_scalar_lhs(1182, 0.0002, 1182, 1188);
        }

        s.store_mul3_lhs(1208, 1182, 1474, 1158);

        s.store_sqrt_offset_scaled_input(1423, 738, 1.0 / (s.v[1227]), 1.0);

        s.store_div_from_scalar(1188, 2.2361, 1278);

        s.store_add_scaled_product_right_sub(1298, 1164, 1.0, 1188, 1297, 1177, (-1.0));

        s.store_exp_mul_scaled_lhs_indices(1179, 743, 2.0, 1158);

        s.store_div_scaled_product_offset_denominator(1425, s.ad_value(752), A::offset(s.ad_value(1179), (-1.0)), 1.0, s.ad_value(1179), 1.0, 1.0);

        s.store_sub_ad_lhs(1165, A::add_scaled_inputs4(A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_value_products(s.ad_value(768), s.v[36], A::add_scaled_products(s.ad_value(737), s.ad_value(1298), 1.0, s.ad_value(707), s.ad_value(1278), (-1.0)), s.ad_value(1423), 1.0, s.ad_value(764), s.ad_value(1177), (-1.0)), 1.0, s.ad_value(1202), (-1.0), s.ad_value(1239), -1.0), 1.0, A::add_scaled_product(s.ad_value(495), 1.0, s.ad_value(496), s.ad_value(1177), 1.0), s.ad_value(1205), 1.0), 1.0, s.ad_value(1238), 1.0, s.ad_value(1208), -1.0, s.ad_value(1424), -1.0), 1425);

        s.store_sub(1387, 1277, 1378);

        s.store_sqrt(1388, 1387);

        s.store_div_scaled_product_indices(1389, 1279, 1388, 1.0, 1278, 1.0);

        s.store_sqrt(1182, 1389);

        s.store_mul(1179, 501, 1378);

        s.b[1571] = (s.v[1179] >= (-0.5));
        s.v[1571] = if s.b[1571] { 1.0 } else { 0.0 };

        if s.b[1571] {
            s.store_offset(1180, 1179, 1.0);
        }

        if (!s.b[1571]) {
            s.store_div_from_scalar_offset_scaled_input(1183, 1.0, 1179, 8.0, 3.0);
            s.store_mul_scale_offset_rhs(1180, 1183, 1179, 3.0, 1.0);
        }

        s.store_mul3_lhs(1390, 758, 1182, 1180);

        s.store_mul(1179, 504, 1378);

        s.b[1572] = (s.v[1179] >= (-0.5));
        s.v[1572] = if s.b[1572] { 1.0 } else { 0.0 };

        if s.b[1572] {
            s.store_offset(1180, 1179, 1.0);
        }

        if (!s.b[1572]) {
            s.store_div_from_scalar_offset_scaled_input(1183, 1.0, 1179, 8.0, 3.0);
            s.store_mul_scale_offset_rhs(1180, 1183, 1179, 3.0, 1.0);
        }

        s.store_mul3_lhs(1391, 758, 1182, 1180);

        s.store_div_scaled_inputs_indices(1179, 500, ((-0.5) * s.v[1227]), 1390, 1.0);

        s.b[1573] = (s.v[1179] > (-100.0));
        s.v[1573] = if s.b[1573] { 1.0 } else { 0.0 };

        if s.b[1573] {
            s.store_exp(1180, 1179);
            s.store_mul_scale_offset_rhs(1392, 1180, 1180, 2.0, 1.0);
        }

        if (!s.b[1573]) {
            s.store_scalar(1180, 3.720075976e-44);
            s.store_mul_scale_offset_rhs(1392, 1180, 1180, 2.0, 1.0);
        }

        s.store_div_scaled_product_indices(1181, 470, 778, 1.0, 1389, 1.0);

        s.store_add_scaled_value_products(1182, s.ad_value(466), 1.0, s.ad_value(467), s.ad_value(1378), 1.0, s.ad_value(468), s.ad_value(1158), 1.0);

        s.store_div_scaled_inputs2_mixed_aii(1183, A::add_scaled_product(s.ad_value(1181), 1.0, s.ad_value(1182), s.ad_value(1392), 1.0), 1.0, 469, 1.0, 757, 1.0);

        s.b[1574] = (s.v[1183] >= (-0.5));
        s.v[1574] = if s.b[1574] { 1.0 } else { 0.0 };

        if s.b[1574] {
            s.store_offset(1393, 1183, 1.0);
        }

        if (!s.b[1574]) {
            s.store_div_from_scalar_offset_scaled_input(1179, 1.0, 1183, 8.0, 3.0);
            s.store_mul_scale_offset_rhs(1393, 1179, 1183, 3.0, 1.0);
        }

        s.b[1575] = (s.v[739] > 0.0);
        s.v[1575] = if s.b[1575] { 1.0 } else { 0.0 };

        if s.b[1575] {
            s.store_mul_neg_lhs(1179, 740, 1158);
        }

        s.b[1576] = (s.v[1179] < (-100.0));
        s.v[1576] = if s.b[1576] { 1.0 } else { 0.0 };

        if (s.b[1575] && s.b[1576]) {
            s.store_scalar(1181, 3.720075976e-44);
        }

        if (s.b[1575] && (!s.b[1576])) {
            s.store_exp(1181, 1179);
        }

        if s.b[1575] {
            s.store_offset_mul_offset_rhs(1182, 739, 1181, 1.0, s.v[1227]);
        }

        if s.b[1575] {
            s.store_mul_ad_rhs(1183, 1168, {
                if ((s.v[1227] / s.v[1182]) > 1e-38) {
                    A::ln(A::div_from_scalar(s.v[1227], s.ad_value(1182)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if s.b[1575] {
            s.store_mul(1405, 1393, 1183);
        }

        if (!s.b[1575]) {
            s.store_scalar(1405, 0.0);
        }

        s.store_mul(411, 499, 1392);

        s.store_mul(1401, 411, 1170);

        s.store_div_scaled_inputs_indices(1179, 503, ((-0.5) * (s.v[689] * s.v[1227])), 1391, 1.0);

        s.b[1577] = (s.v[1179] > (-100.0));
        s.v[1577] = if s.b[1577] { 1.0 } else { 0.0 };

        if s.b[1577] {
            s.store_exp(1180, 1179);
            s.store_mul_scale_offset_rhs(1181, 1180, 1180, 2.0, 1.0);
        }

        if (!s.b[1577]) {
            s.store_scalar(1180, 3.720075976e-44);
            s.store_mul_scale_offset_rhs(1181, 1180, 1180, 2.0, 1.0);
        }

        s.store_mul(1179, 502, 1181);

        s.store_mul(1402, 1179, 1170);

        s.store_sqrt_offset_scaled_input(1179, 498, 1.0 / (s.v[1227]), 1.0);

        s.store_add_scaled_inputs_product_indices(1180, 491, 1.0, 492, 1.0 / (s.v[1227]), 493, 1378, 1.0);

        s.store_add_scaled_product_value_ad(1403, A::mul3(s.ad_value(737), A::offset(s.ad_value(1179), (-1.0)), s.ad_value(1278)), 1.0, 1180, 772, 1.0);

        s.store_div_scaled_product_offset_denominator(1400, s.ad_value(776), s.ad_value(1277), 1.0, s.ad_value(497), s.v[689], 1.0);

        s.store_add_scaled_product_indices(1182, 762, 1.0, 559, 1378, 1.0);

        s.b[1578] = (s.v[1182] < 0.0001);
        s.v[1578] = if s.b[1578] { 1.0 } else { 0.0 };

        if s.b[1578] {
            s.store_div_from_scalar_sub_from_scalar_ad(1188, 1.0, 3.0, A::scale(s.ad_value(1182), 20000.0));
            s.store_mul_sub_from_scalar_lhs(1182, 0.0002, 1182, 1188);
        }

        s.store_mul3_lhs(1404, 1182, 1474, 1158);

        s.store_sqrt_offset_scaled_input(1423, 738, 1.0 / (s.v[1227]), 1.0);

        s.store_div_from_scalar(1188, 2.2361, 1278);

        s.store_add_scaled_product_right_sub(1406, 1388, 1.0, 1188, 1379, 1378, (-1.0));

        s.store_exp_mul_scaled_lhs_indices(1179, 743, 2.0, 1158);

        s.store_div_scaled_product_offset_denominator(1425, s.ad_value(752), A::offset(s.ad_value(1179), (-1.0)), 1.0, s.ad_value(1179), 1.0, 1.0);

        s.store_sub_ad_lhs(1407, A::add_scaled_inputs4(A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_value_products(s.ad_value(768), s.v[36], A::add_scaled_products(s.ad_value(737), s.ad_value(1406), 1.0, s.ad_value(707), s.ad_value(1278), (-1.0)), s.ad_value(1423), 1.0, s.ad_value(764), s.ad_value(1378), (-1.0)), 1.0, s.ad_value(1401), (-1.0), s.ad_value(1402), -1.0), 1.0, A::add_scaled_product(s.ad_value(495), 1.0, s.ad_value(496), s.ad_value(1378), 1.0), s.ad_value(1400), 1.0), 1.0, s.ad_value(1403), 1.0, s.ad_value(1404), -1.0, s.ad_value(1405), -1.0), 1425);

        s.b[1579] = (((s.v[88] == 3.0) && (p.p33 == 1.0)) && (p.p16 != 0.0));
        s.v[1579] = if s.b[1579] { 1.0 } else { 0.0 };

        if s.b[1579] {
            s.store_sqrt(1342, 1279);
            s.store_mul(1343, 758, 1342);
            s.store_mul(1344, 758, 1342);
            s.store_div_scaled_inputs_indices(1179, 500, ((-0.5) * s.v[1227]), 1343, 1.0);
        }

        s.b[1580] = (s.v[1179] > (-100.0));
        s.v[1580] = if s.b[1580] { 1.0 } else { 0.0 };

        if (s.b[1579] && s.b[1580]) {
            s.store_exp(1180, 1179);
            s.store_mul_scale_offset_rhs(1345, 1180, 1180, 2.0, 1.0);
        }

        if (s.b[1579] && (!s.b[1580])) {
            s.store_scalar(1180, 3.720075976e-44);
            s.store_mul_scale_offset_rhs(1345, 1180, 1180, 2.0, 1.0);
        }

        if s.b[1579] {
            s.store_mul3_lhs(1346, 499, 1345, 1170);
            s.store_div_scaled_inputs_indices(1179, 503, ((-0.5) * (s.v[689] * s.v[1227])), 1344, 1.0);
        }

        s.b[1581] = (s.v[1179] > (-100.0));
        s.v[1581] = if s.b[1581] { 1.0 } else { 0.0 };

        if (s.b[1579] && s.b[1581]) {
            s.store_exp(1180, 1179);
            s.store_mul_scale_offset_rhs(1181, 1180, 1180, 2.0, 1.0);
        }

        if (s.b[1579] && (!s.b[1581])) {
            s.store_scalar(1180, 3.720075976e-44);
            s.store_mul_scale_offset_rhs(1181, 1180, 1180, 2.0, 1.0);
        }

        if s.b[1579] {
            s.store_mul(1179, 502, 1181);
            s.store_mul(1347, 1179, 1170);
            s.store_sqrt_offset_scaled_input(1179, 498, 1.0 / (s.v[1227]), 1.0);
            s.store_add_scaled_inputs(1180, 491, 1.0, 492, 1.0 / (s.v[1227]));
            s.store_add_scaled_product_value_ad(1348, A::mul3(s.ad_value(737), A::offset(s.ad_value(1179), (-1.0)), s.ad_value(1278)), 1.0, 1180, 772, 1.0);
            s.store_add_ad_lhs(1349, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(768), s.v[36], s.ad_value(1346), (-1.0), s.ad_value(1347), -1.0), 1.0, s.ad_value(495), s.ad_value(1400), 1.0), 1348);
        }

        if (!s.b[1579]) {
            s.store_scalar(1349, 0.0);
        }

        s.store_sub(1166, 1161, 1165);

        s.store_mul(1189, 1167, 1168);

        s.store_div_scaled_product_indices(1145, 745, 1166, 1.0, 1189, 1.0);

        s.store_div_scaled_inputs2_mixed_iai(1169, 521, 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(745), s.ad_value(1166)), (-1.0), 1189, 1.0);

        s.b[1582] = (s.v[1145] > 100.0);
        s.v[1582] = if s.b[1582] { 1.0 } else { 0.0 };

        if s.b[1582] {
            s.copy_ad(1210, 1166);
            s.store_scalar(1146, 0.0);
        }

        s.b[1583] = (s.v[1169] > 100.0);
        s.v[1583] = if s.b[1583] { 1.0 } else { 0.0 };

        if ((!s.b[1582]) && s.b[1583]) {
            s.store_div_scaled_inputs2_mixed_iia(1179, 1166, 1.0, 521, (-1.0), A::mul(s.ad_value(1167), s.ad_value(1168)), 1.0);
            s.store_exp(1146, 1179);
            s.store_mul_div_scaled_product_indices(1210, 1146, 1168, 1473, 1.0, 757, 1.0);
        }

        if ((!s.b[1582]) && (!s.b[1583])) {
            s.store_exp(1146, 1145);
            s.store_mul_ln_ad_rhs(1180, 1189, A::offset(s.ad_value(1146), 1.0));
            s.store_mul3_ad(1192, A::div_scaled_inputs(s.ad_value(757), -1.0, A::mul(s.ad_value(1168), s.ad_value(1473)), 1.0), A::exp(s.ad_value(1169)), A::sub_from_scalar(1.0, s.ad_value(745)));
            s.store_sub_ad_rhs(1181, 745, A::div_scaled_product(s.ad_value(1189), s.ad_value(1192), 1.0, A::sub_from_scalar(1.0, s.ad_value(745)), 1.0));
            s.store_div(1210, 1180, 1181);
        }

        s.store_add_scaled_inputs(1225, 1210, 1.0, 1168, 2.0);

        s.copy_ad(451, 1210);

        s.b[1584] = (s.v[746] <= 0.0);
        s.v[1584] = if s.b[1584] { 1.0 } else { 0.0 };

        if s.b[1584] {
            s.store_scalar(1426, 1.0);
        }

        if (!s.b[1584]) {
            s.store_div_scaled_inputs_indices(1188, 746, ((s.v[1227]) as f64).sqrt(), 1225, 1.0);
            s.store_div_from_scalar_offset_input(1426, 1.0, 1188, 1.0);
        }

        s.store_sub(1188, 1164, 1278);

        s.store_sub_from_scalar_ad(1228, s.v[689], A::add_scaled_products(s.ad_value(566), s.ad_value(1210), (2.0 - s.v[58]), s.ad_value(567), s.ad_value(1188), (2.0 - s.v[58])));

        s.b[1585] = (s.v[1228] < 2e-8);
        s.v[1585] = if s.b[1585] { 1.0 } else { 0.0 };

        if s.b[1585] {
            s.store_div_from_scalar_sub_from_scalar_ad(1179, 1.0, 6e-8, A::scale(s.ad_value(1228), 2.0));
            s.store_mul_sub_from_scalar_lhs_scaled_output(1228, 4e-8, 1228, 1179, 2e-8);
        }

        s.b[1586] = (s.v[403] == 1.0);
        s.v[1586] = if s.b[1586] { 1.0 } else { 0.0 };

        if s.b[1586] {
            s.store_scalar(1222, 0.0);
        }

        if (!s.b[1586]) {
            s.store_add_scaled_products_indices(1179, 553, 1210, 1.0, 554, 1188, 1.0);
        }

        s.b[1587] = (s.v[1179] >= (-0.9));
        s.v[1587] = if s.b[1587] { 1.0 } else { 0.0 };

        if ((!s.b[1586]) && s.b[1587]) {
            s.store_mul_offset_rhs(1222, 1290, 1179, 1.0);
        }

        if ((!s.b[1586]) && (!s.b[1587])) {
            s.store_div_from_scalar_offset_scaled_input(1180, 1.0, 1179, 20.0, 17.0);
            s.store_mul_ad_product_lhs(1222, s.ad_value(1290), A::offset(s.ad_value(1179), 0.8), 1180);
        }

        s.b[1588] = (s.v[403] == 2.0);
        s.v[1588] = if s.b[1588] { 1.0 } else { 0.0 };

        if s.b[1588] {
            s.store_add_scaled_inputs3_indices(1222, 423, 1.0, 1222, 1.0, 422, 1.0);
        }

        s.b[1589] = (s.v[473] == 0.0);
        s.v[1589] = if s.b[1589] { 1.0 } else { 0.0 };

        if s.b[1589] {
            s.store_scalar(1195, 1.0);
            s.store_scalar(1196, 1.0);
        }

        if (!s.b[1589]) {
            s.store_mul(1189, 477, 1297);
        }

        s.b[1590] = (s.v[1189] >= (-0.5));
        s.v[1590] = if s.b[1590] { 1.0 } else { 0.0 };

        if ((!s.b[1589]) && s.b[1590]) {
            s.store_div_from_scalar_offset_input(1190, 1.0, 1189, 1.0);
        }

        if ((!s.b[1589]) && (!s.b[1590])) {
            s.store_scalar(1191, ((-1.0) / ((1.0 - 0.5) * (1.0 - 0.5))));
            s.store_offset_scaled(1299, 1191, 0.5, (1.0 / (1.0 - 0.5)));
            s.store_add_scaled_product_indices(1190, 1299, 1.0, 1191, 1189, 1.0);
        }

        if (!s.b[1589]) {
            s.store_add(1189, 1277, 629);
            s.store_div_scaled_product_indices(1299, 1297, 1190, 1.0, 1189, 1.0);
        }

        s.b[1591] = (s.v[1299] < 0.5);
        s.v[1591] = if s.b[1591] { 1.0 } else { 0.0 };

        if ((!s.b[1589]) && s.b[1591]) {
            s.store_div_from_scalar_sqrt_ad(1300, 1.0, A::sub_from_scalar(1.0, s.ad_value(1299)));
        }

        if ((!s.b[1589]) && (!s.b[1591])) {
            s.store_scalar(1190, (1.0 / ((2.0 * (1.0 - 0.5)) * (((1.0 - 0.5)) as f64).sqrt())));
            s.store_sub_from_scalar_scaled_input(1191, (1.0 / (((1.0 - 0.5)) as f64).sqrt()), 1190, 0.5);
            s.store_add_scaled_product_indices(1300, 1191, 1.0, 1190, 1299, 1.0);
        }

        if (!s.b[1589]) {
            s.store_div_scaled_product_denominator_ad(1189, 737, 1423, 0.5, A::sqrt(A::add(s.ad_value(1277), s.ad_value(629))), 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
    ) {
        if (!s.b[1589]) {
            s.store_mul(1180, 1189, 1300);
            s.store_sqrt_mul(1188, 608, 1199);
            s.store_offset_scaled(1204, 1188, 2.0, s.v[1227]);
            s.store_div_from_scalar(1184, s.v[1227], 1204);
            s.store_mul(1205, 473, 1184);
            s.store_offset(1206, 569, s.v[689]);
            s.store_div(1207, 568, 1206);
            s.store_add(1181, 1205, 1207);
            s.store_square(1185, 1184);
            s.store_mul(1186, 1184, 1185);
            s.store_offset_mul(1196, 1180, 1181, 1.0);
            s.store_mul3_lhs(1187, 474, 473, 1186);
            s.store_mul_neg_lhs(1214, 1180, 1187);
            s.store_add_scaled_product_indices(1195, 1196, 1.0, 1214, 1210, 1.0);
        }

        s.b[1592] = (s.v[1196] < 0.01);
        s.v[1592] = if s.b[1592] { 1.0 } else { 0.0 };

        if s.b[1592] {
            s.store_div_from_scalar_sub_from_scalar_ad(1188, 1.0, 3.0, A::scale(s.ad_value(1196), 200.0));
            s.store_mul_sub_from_scalar_lhs(1196, 0.02, 1196, 1188);
        }

        s.b[1593] = (s.v[1195] < 0.01);
        s.v[1593] = if s.b[1593] { 1.0 } else { 0.0 };

        if s.b[1593] {
            s.store_div_from_scalar_sub_from_scalar_ad(1188, 1.0, 3.0, A::scale(s.ad_value(1195), 200.0));
            s.store_mul_sub_from_scalar_lhs(1195, 0.02, 1195, 1188);
        }

        s.b[1594] = (s.v[473] == 0.0);
        s.v[1594] = if s.b[1594] { 1.0 } else { 0.0 };

        if s.b[1594] {
            s.store_scalar(1408, 1.0);
        }

        if (!s.b[1594]) {
            s.store_mul(1189, 477, 1379);
        }

        s.b[1595] = (s.v[1189] >= (-0.5));
        s.v[1595] = if s.b[1595] { 1.0 } else { 0.0 };

        if ((!s.b[1594]) && s.b[1595]) {
            s.store_div_from_scalar_offset_input(1190, 1.0, 1189, 1.0);
        }

        if ((!s.b[1594]) && (!s.b[1595])) {
            s.store_scalar(1191, ((-1.0) / ((1.0 - 0.5) * (1.0 - 0.5))));
            s.store_offset_scaled(1299, 1191, 0.5, (1.0 / (1.0 - 0.5)));
            s.store_add_scaled_product_indices(1190, 1299, 1.0, 1191, 1189, 1.0);
        }

        if (!s.b[1594]) {
            s.store_add(1189, 1277, 629);
            s.store_div_scaled_product_indices(1299, 1379, 1190, 1.0, 1189, 1.0);
        }

        s.b[1596] = (s.v[1299] < 0.5);
        s.v[1596] = if s.b[1596] { 1.0 } else { 0.0 };

        if ((!s.b[1594]) && s.b[1596]) {
            s.store_div_from_scalar_sqrt_ad(1300, 1.0, A::sub_from_scalar(1.0, s.ad_value(1299)));
        }

        if ((!s.b[1594]) && (!s.b[1596])) {
            s.store_scalar(1190, (1.0 / ((2.0 * (1.0 - 0.5)) * (((1.0 - 0.5)) as f64).sqrt())));
            s.store_sub_from_scalar_scaled_input(1191, (1.0 / (((1.0 - 0.5)) as f64).sqrt()), 1190, 0.5);
            s.store_add_scaled_product_indices(1300, 1191, 1.0, 1190, 1299, 1.0);
        }

        if (!s.b[1594]) {
            s.store_div_scaled_product_denominator_ad(1189, 737, 1423, 0.5, A::sqrt(A::add(s.ad_value(1277), s.ad_value(629))), 1.0);
            s.store_mul(1180, 1189, 1300);
            s.store_sqrt_mul(1188, 608, 1389);
            s.store_offset_scaled(1204, 1188, 2.0, s.v[1227]);
            s.store_div_from_scalar(1184, s.v[1227], 1204);
            s.store_mul(1205, 473, 1184);
            s.store_offset(1206, 569, s.v[689]);
            s.store_div(1207, 568, 1206);
            s.store_add(1181, 1205, 1207);
            s.store_square(1185, 1184);
            s.store_mul(1186, 1184, 1185);
            s.store_offset_mul(1408, 1180, 1181, 1.0);
        }

        s.b[1597] = (s.v[1408] < 0.01);
        s.v[1597] = if s.b[1597] { 1.0 } else { 0.0 };

        if s.b[1597] {
            s.store_div_from_scalar_sub_from_scalar_ad(1188, 1.0, 3.0, A::scale(s.ad_value(1408), 200.0));
            s.store_mul_sub_from_scalar_lhs(1408, 0.02, 1408, 1188);
        }

        if (s.v[68] != 0.0) {
            s.store_scaled_offset_ad(1300, A::sub_from_scalar((s.v[79] - s.v[80]), A::scale(s.ad_value(1247), 0.5)), 0.45, (2.0 * s.v[36]));
            s.store_scalar(1442, ((s.v[72] * s.v[74]) / 3.9));
        }

        if (s.v[68] == 0.0) {
            s.store_scalar(1300, 0.0);
            s.store_scalar(1442, s.v[91]);
        }

        s.b[1598] = (s.v[89] == 1.0);
        s.v[1598] = if s.b[1598] { 1.0 } else { 0.0 };

        if s.b[1598] {
            s.store_add_scaled_inputs4_indices(1179, 1210, 1.0, 1165, 1.0, 1165, 1.0, 1300, -1.0);
            s.store_add_scaled_product_indices(1181, 1291, 1.0, 1293, 1177, 1.0);
            s.store_div(1182, 1179, 1442);
            s.store_mul_add_scaled_product_rhs(1184, 1182, s.ad_value(1181), 1.0, s.ad_value(1292), s.ad_value(1182), 1.0);
        }

        s.b[1599] = (s.v[89] == 2.0);
        s.v[1599] = if s.b[1599] { 1.0 } else { 0.0 };

        if ((!s.b[1598]) && s.b[1599]) {
            s.store_mul_add(1184, A::div_scaled_inputs2(s.ad_value(1210), 1.0, s.ad_value(1300), (-1.0), s.ad_value(776), 1.0), A::add_scaled_product(s.ad_value(1291), 1.0, s.ad_value(1293), s.ad_value(1177), 1.0), A::div_scaled_product(s.ad_value(1292), A::sub(s.ad_value(1210), s.ad_value(1300)), 1.0, s.ad_value(776), 1.0));
        }

        s.b[1600] = (s.v[89] == 3.0);
        s.v[1600] = if s.b[1600] { 1.0 } else { 0.0 };

        if (((!s.b[1598]) && (!s.b[1599])) && s.b[1600]) {
            s.store_add_scaled_inputs4_indices(1179, 1210, 1.0, 1165, 1.0, 1165, 1.0, 1300, -1.0);
            s.store_offset_mul(1181, 1293, 1177, 1.0);
            s.store_div(1182, 1179, 1442);
            s.store_mul_add_scaled_product_rhs(1183, 1182, s.ad_value(1291), 1.0, s.ad_value(1292), s.ad_value(1182), 1.0);
            s.store_mul(1184, 1183, 1181);
        }

        if (((!s.b[1598]) && (!s.b[1599])) && (!s.b[1600])) {
            s.store_scale_ad(1179, A::div_scaled_inputs2(s.ad_value(1210), 1e-8, s.ad_value(425), 1e-8, s.ad_value(776), 1.0), 0.16666666666666666);
        }

        if (((!s.b[1598]) && (!s.b[1599])) && (!s.b[1600])) {
            s.store_exp_ad(1180, A::mul(s.ad_value(518), {
                if (s.v[1179] > 1e-38) {
                    A::ln(s.ad_value(1179))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }));
        }

        if (((!s.b[1598]) && (!s.b[1599])) && (!s.b[1600])) {
            s.store_add_scaled_product_indices(1181, 1291, 1.0, 1293, 1177, 1.0);
            s.store_mul_pow_ad_rhs(1490, 519, s.ad_value(771), s.ad_value(520));
            s.store_mul_pow_ad_rhs(1491, 516, s.ad_value(771), s.ad_value(517));
            s.copy_ad(1441, 426);
        }

        if (((!s.b[1598]) && (!s.b[1599])) && (!s.b[1600])) {
            s.store_exp_ad(1189, A::mul(s.ad_value(1490), {
                if ((1.0 + (s.v[1210] / s.v[1441])) > 1e-38) {
                    A::ln(A::offset(A::div(s.ad_value(1210), s.ad_value(1441)), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }));
        }

        if (((!s.b[1598]) && (!s.b[1599])) && (!s.b[1600])) {
            s.store_div(1190, 1491, 1189);
            s.store_add_scaled_product_indices(1184, 1190, 1.0, 1180, 1181, 1.0);
        }

        s.b[1601] = (s.v[1184] >= (-0.8));
        s.v[1601] = if s.b[1601] { 1.0 } else { 0.0 };

        if s.b[1601] {
            s.store_offset(1271, 1184, 1.0);
        }

        if (!s.b[1601]) {
            s.store_div_from_scalar_offset_scaled_input(1188, 1.0, 1184, 10.0, 7.0);
            s.store_mul_offset_lhs(1271, 1184, 0.6, 1188);
        }

        s.store_div(1171, 1280, 1271);

        s.copy_ad(410, 1171);

        s.store_mul3_lhs(1223, 1228, 1281, 757);

        s.store_mul(1224, 1223, 1222);

        s.store_div_scaled_inputs_indices(1172, 1281, 2.0, 1171, 1.0);

        s.store_scale(1174, 1172, s.v[1227]);

        s.b[1602] = (s.v[475] == 0.0);
        s.v[1602] = if s.b[1602] { 1.0 } else { 0.0 };

        if s.b[1602] {
            s.copy_ad(1209, 476);
        }

        s.b[1603] = (s.v[475] > 0.0);
        s.v[1603] = if s.b[1603] { 1.0 } else { 0.0 };

        if ((!s.b[1602]) && s.b[1603]) {
            s.store_sub_from_scalar(1179, 1.0, 476);
            s.store_offset_add_scaled_product(1180, s.ad_value(1179), 1.0, s.ad_value(475), s.ad_value(1210), (-1.0), (-0.0001));
            s.store_sqrt_add_scaled_square_input(1181, 1180, 1.0, 1179, 0.0004);
            s.store_add_scaled_inputs4_indices(1209, 476, 1.0, 1179, 1.0, 1180, (-0.5), 1181, (-0.5));
        }

        if ((!s.b[1602]) && (!s.b[1603])) {
            s.store_offset_add_scaled_product(1180, s.ad_value(476), 1.0, s.ad_value(475), s.ad_value(1210), 1.0, (-0.0001));
            s.store_sqrt_add_scaled_square_input(1181, 1180, 1.0, 476, 0.0004);
            s.store_scaled_add(1209, 1180, 1181, 0.5);
        }

        s.b[1604] = ((s.v[1222] == 0.0) && (s.v[1209] == 1.0));
        s.v[1604] = if s.b[1604] { 1.0 } else { 0.0 };

        if s.b[1604] {
            s.store_div_from_scalar_ad(1179, 1.0, A::add_scaled_product(s.ad_value(1225), 1.0, s.ad_value(1195), s.ad_value(1174), 1.0));
            s.store_mul(1182, 1174, 1225);
            s.store_mul(1173, 1182, 1179);
        }

        if (!s.b[1604]) {
            s.store_mul(1188, 1195, 1224);
            s.store_mul(1186, 1225, 1188);
            s.store_mul(1185, 1225, 1224);
            s.store_mul_add_scaled_inputs_rhs(1179, 1195, A::offset(s.ad_value(1188), (-1.0)), 2.0, A::div_from_scalar(1.0, s.ad_value(1209)), 2.0);
            s.store_add_scaled_ad_lhs(1180, A::add_scaled_products(s.ad_value(1225), A::offset(A::div_from_scalar(2.0, s.ad_value(1209)), (-1.0)), 1.0, s.ad_value(1195), s.ad_value(1174), 1.0), 1186, 3.0);
            s.store_mul_add_scaled_inputs_rhs(1181, 1225, s.ad_value(1174), 1.0, s.ad_value(1185), 2.0);
            s.store_sqrt_add_scaled_square_product(1182, 1180, 1.0, 1179, 1181, (-2.0));
            s.store_div_scaled_inputs2_indices(1173, 1180, 1.0, 1182, (-1.0), 1179, 1.0);
        }

        s.store_add_scaled_inputs3_indices(1180, 1173, 1.0, 1158, (-1.0), 550, -1.0);

        s.store_sqrt_add_scaled_square_product(1181, 1180, 1.0, 550, 1173, 4.0);

        s.store_add_scaled_inputs3_indices(1211, 1173, 1.0, 1180, (-0.5), 1181, (-0.5));

        s.b[1605] = (s.v[1211] > s.v[1158]);
        s.v[1605] = if s.b[1605] { 1.0 } else { 0.0 };

        if s.b[1605] {
            s.copy_ad(1211, 1158);
        }

        s.store_sub(1213, 1158, 1211);

        s.store_sub_from_scalar_ad(1207, 1.0, A::div_scaled_product(s.ad_value(1195), s.ad_value(1173), 0.5, s.ad_value(1225), 1.0));

        s.store_mul(1188, 1224, 1210);

        s.store_add_scaled_inputs_product_indices(1179, 1174, 1.0, 1173, 1.0, 1188, 1207, 2.0);

        s.store_mul(1188, 1224, 1195);

        s.store_add_offset_ad_lhs(1180, A::div_from_scalar(2.0, s.ad_value(1209)), (-1.0), 1188);

        s.store_div(1176, 1179, 1180);

        s.b[1606] = ((s.v[560] > 0.0) && (s.v[1213] > 1e-10));
        s.v[1606] = if s.b[1606] { 1.0 } else { 0.0 };

        if s.b[1606] {
            s.store_div_from_scalar_ad(1179, 1.0, A::mul3(s.ad_value(560), s.ad_value(1195), s.ad_value(489)));
            s.store_div(1181, 1210, 1174);
            s.store_scaled_add(1180, 1195, 1181, s.v[1227]);
            s.store_mul(1188, 1179, 1180);
            s.store_mul(1197, 1188, 1213);
        }

        if (!s.b[1606]) {
            s.store_scalar(1197, 2.688117142e43);
        }

        s.b[1607] = (s.v[1475] > 0.0);
        s.v[1607] = if s.b[1607] { 1.0 } else { 0.0 };

        if s.b[1607] {
            s.store_mul(1187, 1195, 1173);
            s.store_mul(1179, 1225, 1187);
            s.store_add(1180, 1225, 1187);
            s.copy_ad(1181, 1475);
            s.store_div_scaled_inputs2_mixed_iai(1198, 1225, 1.0, A::div(s.ad_value(1179), s.ad_value(1180)), (-1.0), 1181, 1.0);
            s.store_mul(1186, 563, 1177);
        }

        s.b[1608] = (s.v[1186] >= (-0.9));
        s.v[1608] = if s.b[1608] { 1.0 } else { 0.0 };

        if (s.b[1607] && s.b[1608]) {
            s.store_div_from_scalar_offset_input(1182, 1.0, 1186, 1.0);
            s.store_mul(1198, 1198, 1182);
        }

    }

    pub(super) fn stamp_reactive_block_13(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1607] && (!s.b[1608])) {
            s.store_div_from_scalar_offset_input(1183, 1.0, 1186, 0.8);
            s.store_mul_scale_offset_rhs(1182, 1183, 1186, 20.0, 17.0);
            s.store_mul(1198, 1198, 1182);
        }

        if (!s.b[1607]) {
            s.store_scalar(1198, 2.688117142e43);
        }

        s.store_mul(1179, 748, 1158);

        s.b[1609] = (s.v[1179] > 100.0);
        s.v[1609] = if s.b[1609] { 1.0 } else { 0.0 };

        if s.b[1609] {
            s.store_scalar(1180, 2.688117142e43);
        }

        if (!s.b[1609]) {
            s.store_exp(1180, 1179);
        }

        s.b[1610] = (s.v[747] > 3.720075976e-44);
        s.v[1610] = if s.b[1610] { 1.0 } else { 0.0 };

        if s.b[1610] {
            s.store_scalar(1181, (1.0 + (s.v[273] * s.v[1227])));
            s.store_div_scaled_offset_numerator(1427, A::mul(s.ad_value(1181), s.ad_value(1180)), 1.0, 1.0, s.ad_value(747), 1.0);
            s.store_mul(1427, 1427, 1426);
        }

        if (!s.b[1610]) {
            s.store_scalar(1427, 2.688117142e43);
        }

        s.store_div(1187, 564, 1174);

        s.store_mul(1188, 1187, 1210);

        s.b[1611] = (s.v[1188] > (-0.9));
        s.v[1611] = if s.b[1611] { 1.0 } else { 0.0 };

        if s.b[1611] {
            s.store_offset(1179, 1188, 1.0);
        }

        if (!s.b[1611]) {
            s.store_div_from_scalar_offset_scaled_input(1180, 1.0, 1188, 20.0, 17.0);
            s.store_mul_offset_lhs(1179, 1188, 0.8, 1180);
        }

        s.store_add(1206, 1197, 1198);

        s.store_div_scaled_product_indices(1180, 1197, 1198, 1.0, 1206, 1.0);

        s.store_add(1206, 1180, 1427);

        s.store_div_scaled_product_indices(1181, 1180, 1427, 1.0, 1206, 1.0);

        s.store_add_scaled_product_indices(1175, 1176, 1.0, 1179, 1181, 1.0);

        s.store_scaled_mul(1221, 757, 1228, 1.0 / (s.v[1227]));

        s.store_mul(1215, 1171, 1221);

        s.store_sub_from_scalar_ad(1179, 1.0, A::div_scaled_product(s.ad_value(1195), s.ad_value(1211), 0.5, s.ad_value(1225), 1.0));

        s.store_mul(1217, 1210, 1179);

        s.store_div(1188, 1211, 1174);

        s.store_offset(1218, 1188, 1.0);

        s.store_div_scaled_product_indices(1216, 1215, 1217, 1.0, 1218, 1.0);

        s.store_offset_mul(1179, 1216, 1222, 1.0);

        s.store_div(1188, 1211, 1179);

        s.store_mul(1219, 1216, 1188);

        s.store_div(1419, 1216, 1179);

        s.store_div(1188, 1213, 1175);

        s.store_offset(1179, 1188, 1.0);

        s.store_scaled_mul(1220, 1219, 1179, 1.0 / (s.v[59]));

        s.store_scaled_mul(454, 1419, 1179, 1.0 / (s.v[59]));

        s.b[1612] = (s.v[454] < 1e-9);
        s.v[1612] = if s.b[1612] { 1.0 } else { 0.0 };

        if s.b[1612] {
            s.store_scalar(454, 1e-9);
        }

        s.store_scaled_mul(1420, 1419, 1179, 1.0 / (s.v[59]));

        s.b[1613] = (s.v[57] != 2.0);
        s.v[1613] = if s.b[1613] { 1.0 } else { 0.0 };

        s.b[1614] = (s.v[68] == 0.0);
        s.v[1614] = if s.b[1614] { 1.0 } else { 0.0 };

        if (s.b[1613] && s.b[1614]) {
            s.store_mul_div_from_scalar_lhs(1179, (3.0 * 3.9), 777, 776);
        }

        if (s.b[1613] && (!s.b[1614])) {
            s.store_div_scaled_inputs_indices(1179, 776, s.v[74], 777, 1.0);
        }

        s.b[1615] = (s.v[70] == 0.0);
        s.v[1615] = if s.b[1615] { 1.0 } else { 0.0 };

        s.b[1616] = (s.v[68] == 0.0);
        s.v[1616] = if s.b[1616] { 1.0 } else { 0.0 };

        if ((s.b[1613] && s.b[1615]) && s.b[1616]) {
            s.store_div_scaled_inputs3_indices(1180, 1158, -1.0, 1444, (-1.0), 1486, -1.0, 1179, 1.0);
        }

        if ((s.b[1613] && s.b[1615]) && (!s.b[1616])) {
            s.store_div_scaled_inputs4_indices(1180, 1158, -1.0, 1444, (-1.0), 1486, -1.0, 736, 1.0, 1179, 1.0);
        }

        s.b[1617] = (((s.v[1483] <= 0.0) || (s.v[1484] <= 0.0)) || (s.v[1485] < 0.0));
        s.v[1617] = if s.b[1617] { 1.0 } else { 0.0 };

        if ((s.b[1613] && s.b[1615]) && (!s.b[1617])) {
            s.store_scaled_add_sqrt_square_offset_rhs(1180, 1180, 1180, ((4.0 * 0.01) * 0.01), 0.5);
            s.store_div_scaled_value_offset_denominator(1181, s.ad_value(1484), 1.0, s.ad_value(1180), 0.001, 1.0);
            s.store_square(1183, 1160);
            s.store_mul_neg_lhs(1184, 1160, 1183);
            s.store_offset_add_ad(1185, s.ad_value(1485), A::abs(s.ad_value(1184)), 1e-9);
            s.store_offset_add_scaled_inputs(1186, A::div(s.ad_value(1184), s.ad_value(1185)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(1184), s.ad_value(1185)), ((4.0 * 1e-6) * 1e-6)), 0.5, (-1e-6));
        }

        s.b[1618] = (s.v[68] == 0.0);
        s.v[1618] = if s.b[1618] { 1.0 } else { 0.0 };

        if ((s.b[1613] && s.b[1615]) && s.b[1618]) {
            s.store_div_scaled_inputs3_indices(1180, 1158, 1.0, 1161, (-1.0), 1479, -1.0, 1179, 1.0);
        }

        if ((s.b[1613] && s.b[1615]) && (!s.b[1618])) {
            s.store_div_scaled_inputs4_indices(1180, 1158, 1.0, 1161, (-1.0), 1479, -1.0, 736, 1.0, 1179, 1.0);
        }

        s.b[1619] = (((s.v[1476] <= 0.0) || (s.v[1477] <= 0.0)) || (s.v[1478] < 0.0));
        s.v[1619] = if s.b[1619] { 1.0 } else { 0.0 };

        if ((s.b[1613] && s.b[1615]) && (!s.b[1619])) {
            s.store_scaled_add_sqrt_square_offset_rhs(1180, 1180, 1180, ((4.0 * 0.01) * 0.01), 0.5);
            s.store_div_scaled_value_offset_denominator(1181, s.ad_value(1477), 1.0, s.ad_value(1180), 0.001, 1.0);
            s.store_square(1183, 1235);
            s.store_mul_neg_lhs(1184, 1235, 1183);
            s.store_offset_add_ad(1185, s.ad_value(1478), A::abs(s.ad_value(1184)), 1e-9);
            s.store_offset_add_scaled_inputs(1186, A::div(s.ad_value(1184), s.ad_value(1185)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(1184), s.ad_value(1185)), ((4.0 * 1e-6) * 1e-6)), 0.5, (-1e-6));
        }

        s.b[1620] = (s.v[68] == 0.0);
        s.v[1620] = if s.b[1620] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1615])) && s.b[1620]) {
            s.store_div_scaled_inputs2_mixed_aii(1180, A::add_scaled_product(s.ad_value(1158), -1.0, s.ad_value(1487), s.ad_value(1444), (-1.0)), 1.0, 1486, (-1.0), 1179, 1.0);
        }

        if ((s.b[1613] && (!s.b[1615])) && (!s.b[1620])) {
            s.store_div_scaled_inputs3_mixed_aiii(1180, A::add_scaled_product(s.ad_value(1158), -1.0, s.ad_value(1487), s.ad_value(1444), (-1.0)), 1.0, 1486, (-1.0), 736, 1.0, 1179, 1.0);
        }

        s.b[1621] = (((s.v[1483] <= 0.0) || (s.v[1484] <= 0.0)) || (s.v[1485] < 0.0));
        s.v[1621] = if s.b[1621] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1615])) && (!s.b[1621])) {
            s.store_scaled_add_sqrt_square_offset_rhs(1180, 1180, 1180, ((4.0 * 0.01) * 0.01), 0.5);
            s.store_div_scaled_value_offset_denominator(1181, s.ad_value(1484), 1.0, s.ad_value(1180), 0.001, 1.0);
            s.store_sub(1183, 1160, 1489);
        }

        s.b[1622] = (s.v[1183] >= ((-1.0) / 100.0));
        s.v[1622] = if s.b[1622] { 1.0 } else { 0.0 };

        if (((s.b[1613] && (!s.b[1615])) && (!s.b[1621])) && s.b[1622]) {
            s.store_scale(1184, 1488, (-100.0));
        }

        if (((s.b[1613] && (!s.b[1615])) && (!s.b[1621])) && (!s.b[1622])) {
            s.store_div(1184, 1488, 1183);
        }

        if ((s.b[1613] && (!s.b[1615])) && (!s.b[1621])) {
            s.store_exp(1185, 1184);
        }

        s.b[1623] = (s.v[68] == 0.0);
        s.v[1623] = if s.b[1623] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1615])) && s.b[1623]) {
            s.store_div_scaled_inputs2_mixed_aii(1180, A::add_scaled_product(s.ad_value(1158), 1.0, s.ad_value(1480), s.ad_value(1161), (-1.0)), 1.0, 1479, (-1.0), 1179, 1.0);
        }

        if ((s.b[1613] && (!s.b[1615])) && (!s.b[1623])) {
            s.store_div_scaled_inputs3_mixed_aiii(1180, A::add_scaled_product(s.ad_value(1158), 1.0, s.ad_value(1480), s.ad_value(1161), (-1.0)), 1.0, 1479, (-1.0), 736, 1.0, 1179, 1.0);
        }

        s.b[1624] = (((s.v[1476] <= 0.0) || (s.v[1477] <= 0.0)) || (s.v[1478] < 0.0));
        s.v[1624] = if s.b[1624] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1615])) && (!s.b[1624])) {
            s.store_scaled_add_sqrt_square_offset_rhs(1180, 1180, 1180, ((4.0 * 0.01) * 0.01), 0.5);
            s.store_div_scaled_value_offset_denominator(1181, s.ad_value(1477), 1.0, s.ad_value(1180), 0.001, 1.0);
            s.store_sub(1183, 1235, 1482);
        }

        s.b[1625] = (s.v[1183] >= ((-1.0) / 100.0));
        s.v[1625] = if s.b[1625] { 1.0 } else { 0.0 };

        if (((s.b[1613] && (!s.b[1615])) && (!s.b[1624])) && s.b[1625]) {
            s.store_scale(1184, 1481, (-100.0));
        }

        if (((s.b[1613] && (!s.b[1615])) && (!s.b[1624])) && (!s.b[1625])) {
            s.store_div(1184, 1481, 1183);
        }

        if ((s.b[1613] && (!s.b[1615])) && (!s.b[1624])) {
            s.store_exp(1185, 1184);
        }

        if s.b[1613] {
            s.store_scalar(1309, (s.v[708] * s.v[174]));
            s.store_scalar(1310, (s.v[709] * s.v[174]));
            s.store_mul(1266, 1168, 661);
            s.store_div(1179, 1421, 1266);
        }

        s.b[1626] = (s.v[1179] > 100.0);
        s.v[1626] = if s.b[1626] { 1.0 } else { 0.0 };

        if (s.b[1613] && s.b[1626]) {
            s.store_scaled_offset(1318, 1179, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1627] = (s.v[1179] < (-100.0));
        s.v[1627] = if s.b[1627] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1626])) && s.b[1627]) {
            s.store_scalar(1318, 3.720075976e-44);
        }

        if ((s.b[1613] && (!s.b[1626])) && (!s.b[1627])) {
            s.store_exp(1318, 1179);
        }

        if s.b[1613] {
            s.store_mul(1266, 1168, 662);
            s.store_div(1179, 1422, 1266);
        }

        s.b[1628] = (s.v[1179] > 100.0);
        s.v[1628] = if s.b[1628] { 1.0 } else { 0.0 };

        if (s.b[1613] && s.b[1628]) {
            s.store_scaled_offset(1319, 1179, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1629] = (s.v[1179] < (-100.0));
        s.v[1629] = if s.b[1629] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1628])) && s.b[1629]) {
            s.store_scalar(1319, 3.720075976e-44);
        }

        if ((s.b[1613] && (!s.b[1628])) && (!s.b[1629])) {
            s.store_exp(1319, 1179);
        }

        s.b[1630] = (s.v[1282] == 0.0);
        s.v[1630] = if s.b[1630] { 1.0 } else { 0.0 };

        if (s.b[1613] && (!s.b[1630])) {
            s.store_mul(1179, 1309, 1282);
        }

        s.b[1631] = (s.v[1283] == 0.0);
        s.v[1631] = if s.b[1631] { 1.0 } else { 0.0 };

        if (s.b[1613] && (!s.b[1631])) {
            s.store_mul(1179, 1310, 1283);
        }

        s.b[1632] = (s.v[1286] == 0.0);
        s.v[1632] = if s.b[1632] { 1.0 } else { 0.0 };

        if (s.b[1613] && (!s.b[1632])) {
            s.store_mul_scaled_offset_ad_rhs(1305, 663, s.v[783], A::mul_offset_rhs(s.ad_value(617), s.ad_value(771), (-1.0)), 1.0);
            s.store_mul_scaled_offset_ad_rhs(1306, 665, s.v[783], A::mul_offset_rhs(s.ad_value(618), s.ad_value(771), (-1.0)), 1.0);
            s.store_div(1179, 1421, 1305);
        }

        s.b[1633] = (s.v[1179] > 100.0);
        s.v[1633] = if s.b[1633] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1632])) && s.b[1633]) {
            s.store_scaled_offset(1189, 1179, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1634] = (s.v[1179] < (-100.0));
        s.v[1634] = if s.b[1634] { 1.0 } else { 0.0 };

        if (((s.b[1613] && (!s.b[1632])) && (!s.b[1633])) && s.b[1634]) {
            s.store_scalar(1189, 3.720075976e-44);
        }

        if (((s.b[1613] && (!s.b[1632])) && (!s.b[1633])) && (!s.b[1634])) {
            s.store_exp(1189, 1179);
        }

        s.b[1635] = ((s.v[675] - s.v[1421]) < 0.001);
        s.v[1635] = if s.b[1635] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1632])) && s.b[1635]) {
            s.store_scalar(1180, 1000.0);
            s.store_mul_div_scaled_inputs_product_lhs(1179, 1421, -1.0, 1306, 1.0, 675, 1180);
        }

        s.b[1636] = (s.v[1179] > 100.0);
        s.v[1636] = if s.b[1636] { 1.0 } else { 0.0 };

        if (((s.b[1613] && (!s.b[1632])) && s.b[1635]) && s.b[1636]) {
            s.store_scaled_offset(1190, 1179, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1637] = (s.v[1179] < (-100.0));
        s.v[1637] = if s.b[1637] { 1.0 } else { 0.0 };

        if ((((s.b[1613] && (!s.b[1632])) && s.b[1635]) && (!s.b[1636])) && s.b[1637]) {
            s.store_scalar(1190, 3.720075976e-44);
        }

        if ((((s.b[1613] && (!s.b[1632])) && s.b[1635]) && (!s.b[1636])) && (!s.b[1637])) {
            s.store_exp(1190, 1179);
        }

        if ((s.b[1613] && (!s.b[1632])) && s.b[1635]) {
            s.store_neg(1190, 1190);
        }

        if ((s.b[1613] && (!s.b[1632])) && (!s.b[1635])) {
            s.store_div_from_scalar_sub_ad(1180, 1.0, s.ad_value(675), s.ad_value(1421));
            s.store_mul_div_scaled_inputs_product_lhs(1179, 1421, -1.0, 1306, 1.0, 675, 1180);
        }

        s.b[1638] = (s.v[1179] > 100.0);
        s.v[1638] = if s.b[1638] { 1.0 } else { 0.0 };

        if (((s.b[1613] && (!s.b[1632])) && (!s.b[1635])) && s.b[1638]) {
            s.store_scaled_offset(1190, 1179, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1639] = (s.v[1179] < (-100.0));
        s.v[1639] = if s.b[1639] { 1.0 } else { 0.0 };

        if ((((s.b[1613] && (!s.b[1632])) && (!s.b[1635])) && (!s.b[1638])) && s.b[1639]) {
            s.store_scalar(1190, 3.720075976e-44);
        }

        if ((((s.b[1613] && (!s.b[1632])) && (!s.b[1635])) && (!s.b[1638])) && (!s.b[1639])) {
            s.store_exp(1190, 1179);
        }

        if ((s.b[1613] && (!s.b[1632])) && (!s.b[1635])) {
            s.store_neg(1190, 1190);
        }

        if (s.b[1613] && (!s.b[1632])) {
            s.store_mul(1182, 1309, 1286);
        }

        s.b[1640] = (s.v[1287] == 0.0);
        s.v[1640] = if s.b[1640] { 1.0 } else { 0.0 };

        if (s.b[1613] && (!s.b[1640])) {
            s.store_mul_scaled_offset_ad_rhs(1305, 664, s.v[783], A::mul_offset_rhs(s.ad_value(617), s.ad_value(771), (-1.0)), 1.0);
            s.store_mul_scaled_offset_ad_rhs(1306, 666, s.v[783], A::mul_offset_rhs(s.ad_value(618), s.ad_value(771), (-1.0)), 1.0);
            s.store_div(1179, 1422, 1305);
        }

        s.b[1641] = (s.v[1179] > 100.0);
        s.v[1641] = if s.b[1641] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1640])) && s.b[1641]) {
            s.store_scaled_offset(1189, 1179, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1642] = (s.v[1179] < (-100.0));
        s.v[1642] = if s.b[1642] { 1.0 } else { 0.0 };

        if (((s.b[1613] && (!s.b[1640])) && (!s.b[1641])) && s.b[1642]) {
            s.store_scalar(1189, 3.720075976e-44);
        }

        if (((s.b[1613] && (!s.b[1640])) && (!s.b[1641])) && (!s.b[1642])) {
            s.store_exp(1189, 1179);
        }

        s.b[1643] = ((s.v[676] - s.v[1422]) < 0.001);
        s.v[1643] = if s.b[1643] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1640])) && s.b[1643]) {
            s.store_scalar(1180, 1000.0);
            s.store_mul_div_scaled_inputs_product_lhs(1179, 1422, -1.0, 1306, 1.0, 676, 1180);
        }

        s.b[1644] = (s.v[1179] > 100.0);
        s.v[1644] = if s.b[1644] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[1613] && (!s.b[1640])) && s.b[1643]) && s.b[1644]) {
            s.store_scaled_offset(1190, 1179, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1645] = (s.v[1179] < (-100.0));
        s.v[1645] = if s.b[1645] { 1.0 } else { 0.0 };

        if ((((s.b[1613] && (!s.b[1640])) && s.b[1643]) && (!s.b[1644])) && s.b[1645]) {
            s.store_scalar(1190, 3.720075976e-44);
        }

        if ((((s.b[1613] && (!s.b[1640])) && s.b[1643]) && (!s.b[1644])) && (!s.b[1645])) {
            s.store_exp(1190, 1179);
        }

        if ((s.b[1613] && (!s.b[1640])) && s.b[1643]) {
            s.store_neg(1190, 1190);
        }

        if ((s.b[1613] && (!s.b[1640])) && (!s.b[1643])) {
            s.store_div_from_scalar_sub_ad(1180, 1.0, s.ad_value(676), s.ad_value(1422));
            s.store_mul_div_scaled_inputs_product_lhs(1179, 1422, -1.0, 1306, 1.0, 676, 1180);
        }

        s.b[1646] = (s.v[1179] > 100.0);
        s.v[1646] = if s.b[1646] { 1.0 } else { 0.0 };

        if (((s.b[1613] && (!s.b[1640])) && (!s.b[1643])) && s.b[1646]) {
            s.store_scaled_offset(1190, 1179, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1647] = (s.v[1179] < (-100.0));
        s.v[1647] = if s.b[1647] { 1.0 } else { 0.0 };

        if ((((s.b[1613] && (!s.b[1640])) && (!s.b[1643])) && (!s.b[1646])) && s.b[1647]) {
            s.store_scalar(1190, 3.720075976e-44);
        }

        if ((((s.b[1613] && (!s.b[1640])) && (!s.b[1643])) && (!s.b[1646])) && (!s.b[1647])) {
            s.store_exp(1190, 1179);
        }

        if ((s.b[1613] && (!s.b[1640])) && (!s.b[1643])) {
            s.store_neg(1190, 1190);
        }

        if (s.b[1613] && (!s.b[1640])) {
            s.store_mul(1182, 1310, 1287);
        }

        if s.b[1613] {
            s.store_scalar(1265, ((s.v[689] / s.v[59]) * s.v[174]));
        }

        s.b[1648] = ((s.v[1284] == 0.0) && (s.v[1285] == 0.0));
        s.v[1648] = if s.b[1648] { 1.0 } else { 0.0 };

        if (s.b[1613] && s.b[1648]) {
            s.store_scalar(1322, 0.0);
            s.store_scalar(1323, 0.0);
            s.store_scalar(1268, 0.0);
        }

        if (s.b[1613] && (!s.b[1648])) {
            s.store_mul_offset_rhs(1324, 1307, 1318, (-1.0));
        }

        s.b[1649] = (s.v[1324] < 1e-5);
        s.v[1649] = if s.b[1649] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1648])) && s.b[1649]) {
            s.store_scalar(1324, 0.0);
            s.store_scalar(1326, 1.0);
        }

        if ((s.b[1613] && (!s.b[1648])) && (!s.b[1649])) {
            s.store_div_from_scalar_sqrt_ad(1326, 1.0, A::offset(s.ad_value(1324), 1.0));
        }

        if (s.b[1613] && (!s.b[1648])) {
            s.store_mul_offset_rhs(1325, 1308, 1319, (-1.0));
        }

        s.b[1650] = (s.v[1325] < 1e-5);
        s.v[1650] = if s.b[1650] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1648])) && s.b[1650]) {
            s.store_scalar(1325, 0.0);
            s.store_scalar(1327, 1.0);
        }

        if ((s.b[1613] && (!s.b[1648])) && (!s.b[1650])) {
            s.store_div_from_scalar_sqrt_ad(1327, 1.0, A::offset(s.ad_value(1325), 1.0));
        }

        if (s.b[1613] && (!s.b[1648])) {
            s.store_sub_from_scalar(1179, 1.0, 712);
            s.store_mul3_lhs(1320, 1265, 1284, 713);
            s.store_mul(1180, 1179, 1320);
            s.store_mul3_lhs(1320, 1265, 1285, 713);
            s.store_mul(1180, 1179, 1320);
            s.store_mul3_lhs(1321, 1265, 1284, 714);
            s.store_mul_ad_product_lhs(1322, s.ad_value(1321), A::offset(s.ad_value(1318), (-1.0)), 1326);
            s.store_mul3_lhs(1321, 1265, 1285, 714);
            s.store_mul_ad_product_lhs(1323, s.ad_value(1321), A::offset(s.ad_value(1319), (-1.0)), 1327);
        }

        s.b[1651] = (s.v[49] == 1.0);
        s.v[1651] = if s.b[1651] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1648])) && s.b[1651]) {
            s.store_scalar(1268, 0.0);
        }

        if ((s.b[1613] && (!s.b[1648])) && (!s.b[1651])) {
            s.store_offset_div_scaled_inputs2_indices(1179, 1421, 1.0, 1422, 1.0, 715, 1.0, 1.0);
            s.store_add(1180, 1324, 1325);
            s.store_sqrt_add_scaled_square_input(1182, 1179, 1.0, 1180, 4.0);
            s.store_scaled_add(1181, 1179, 1182, 0.5);
        }

        s.b[1652] = (s.v[1181] < 0.1);
        s.v[1652] = if s.b[1652] { 1.0 } else { 0.0 };

        if (((s.b[1613] && (!s.b[1648])) && (!s.b[1651])) && s.b[1652]) {
            s.store_scalar(1328, 10.0);
        }

        if (((s.b[1613] && (!s.b[1648])) && (!s.b[1651])) && (!s.b[1652])) {
            s.store_div_from_scalar(1328, 1.0, 1181);
        }

        if ((s.b[1613] && (!s.b[1648])) && (!s.b[1651])) {
            s.store_mul(1179, 712, 1320);
            s.store_mul_ad_product_lhs(1268, s.ad_value(1179), A::sub(s.ad_value(1318), s.ad_value(1319)), 1328);
        }

        s.b[1653] = ((s.v[1288] == 0.0) && (s.v[1289] == 0.0));
        s.v[1653] = if s.b[1653] { 1.0 } else { 0.0 };

        if (s.b[1613] && (!s.b[1653])) {
            s.store_scale(1267, 659, s.v[783]);
        }

        s.b[1654] = ((s.v[677] - s.v[1421]) < 0.001);
        s.v[1654] = if s.b[1654] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1653])) && s.b[1654]) {
            s.store_scalar(1180, 1000.0);
            s.store_mul_div_scaled_inputs_product_lhs(1179, 1421, -1.0, 1267, 1.0, 677, 1180);
        }

        s.b[1655] = (s.v[1179] > 100.0);
        s.v[1655] = if s.b[1655] { 1.0 } else { 0.0 };

        if (((s.b[1613] && (!s.b[1653])) && s.b[1654]) && s.b[1655]) {
            s.store_scaled_offset(1180, 1179, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1656] = (s.v[1179] < (-100.0));
        s.v[1656] = if s.b[1656] { 1.0 } else { 0.0 };

        if ((((s.b[1613] && (!s.b[1653])) && s.b[1654]) && (!s.b[1655])) && s.b[1656]) {
            s.store_scalar(1180, 3.720075976e-44);
        }

        if ((((s.b[1613] && (!s.b[1653])) && s.b[1654]) && (!s.b[1655])) && (!s.b[1656])) {
            s.store_exp(1180, 1179);
        }

        if ((s.b[1613] && (!s.b[1653])) && s.b[1654]) {
            s.store_mul(1182, 1309, 1288);
        }

        if ((s.b[1613] && (!s.b[1653])) && (!s.b[1654])) {
            s.store_div_from_scalar_sub_ad(1180, 1.0, s.ad_value(677), s.ad_value(1421));
            s.store_mul_div_scaled_inputs_product_lhs(1179, 1421, -1.0, 1267, 1.0, 677, 1180);
        }

        s.b[1657] = (s.v[1179] > 100.0);
        s.v[1657] = if s.b[1657] { 1.0 } else { 0.0 };

        if (((s.b[1613] && (!s.b[1653])) && (!s.b[1654])) && s.b[1657]) {
            s.store_scaled_offset(1180, 1179, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1658] = (s.v[1179] < (-100.0));
        s.v[1658] = if s.b[1658] { 1.0 } else { 0.0 };

        if ((((s.b[1613] && (!s.b[1653])) && (!s.b[1654])) && (!s.b[1657])) && s.b[1658]) {
            s.store_scalar(1180, 3.720075976e-44);
        }

        if ((((s.b[1613] && (!s.b[1653])) && (!s.b[1654])) && (!s.b[1657])) && (!s.b[1658])) {
            s.store_exp(1180, 1179);
        }

        if ((s.b[1613] && (!s.b[1653])) && (!s.b[1654])) {
            s.store_mul(1182, 1309, 1288);
        }

        if (s.b[1613] && (!s.b[1653])) {
            s.store_scale(1267, 660, s.v[783]);
        }

        s.b[1659] = ((s.v[678] - s.v[1422]) < 0.001);
        s.v[1659] = if s.b[1659] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1653])) && s.b[1659]) {
            s.store_scalar(1180, 1000.0);
            s.store_mul_div_scaled_inputs_product_lhs(1179, 1422, -1.0, 1267, 1.0, 678, 1180);
        }

        s.b[1660] = (s.v[1179] > 100.0);
        s.v[1660] = if s.b[1660] { 1.0 } else { 0.0 };

        if (((s.b[1613] && (!s.b[1653])) && s.b[1659]) && s.b[1660]) {
            s.store_scaled_offset(1180, 1179, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1661] = (s.v[1179] < (-100.0));
        s.v[1661] = if s.b[1661] { 1.0 } else { 0.0 };

        if ((((s.b[1613] && (!s.b[1653])) && s.b[1659]) && (!s.b[1660])) && s.b[1661]) {
            s.store_scalar(1180, 3.720075976e-44);
        }

        if ((((s.b[1613] && (!s.b[1653])) && s.b[1659]) && (!s.b[1660])) && (!s.b[1661])) {
            s.store_exp(1180, 1179);
        }

        if ((s.b[1613] && (!s.b[1653])) && s.b[1659]) {
            s.store_mul(1182, 1310, 1289);
        }

        if ((s.b[1613] && (!s.b[1653])) && (!s.b[1659])) {
            s.store_div_from_scalar_sub_ad(1180, 1.0, s.ad_value(678), s.ad_value(1422));
            s.store_mul_div_scaled_inputs_product_lhs(1179, 1422, -1.0, 1267, 1.0, 678, 1180);
        }

        s.b[1662] = (s.v[1179] > 100.0);
        s.v[1662] = if s.b[1662] { 1.0 } else { 0.0 };

        if (((s.b[1613] && (!s.b[1653])) && (!s.b[1659])) && s.b[1662]) {
            s.store_scaled_offset(1180, 1179, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1663] = (s.v[1179] < (-100.0));
        s.v[1663] = if s.b[1663] { 1.0 } else { 0.0 };

        if ((((s.b[1613] && (!s.b[1653])) && (!s.b[1659])) && (!s.b[1662])) && s.b[1663]) {
            s.store_scalar(1180, 3.720075976e-44);
        }

        if ((((s.b[1613] && (!s.b[1653])) && (!s.b[1659])) && (!s.b[1662])) && (!s.b[1663])) {
            s.store_exp(1180, 1179);
        }

        if ((s.b[1613] && (!s.b[1653])) && (!s.b[1659])) {
            s.store_mul(1182, 1310, 1289);
        }

        if (!s.b[1613]) {
            s.store_scalar(1322, 0.0);
            s.store_scalar(1323, 0.0);
            s.store_scalar(1268, 0.0);
        }

        s.b[1664] = ((s.v[355] != 0.0) || (s.v[356] != 0.0));
        s.v[1664] = if s.b[1664] { 1.0 } else { 0.0 };

        if s.b[1664] {
            s.store_sub(1409, 1161, 1160);
            s.store_add_scaled_inputs_product_indices(1162, 768, s.v[36], 1277, (-1.0), 707, 1278, (-1.0));
            s.store_add_scaled_inputs3_offset_indices(1182, 1162, 1.0, 1161, (-1.0), 1160, 1.0, (-0.02));
        }

        s.b[1665] = (s.v[1162] <= 0.0);
        s.v[1665] = if s.b[1665] { 1.0 } else { 0.0 };

        if (s.b[1664] && s.b[1665]) {
            s.store_sqrt_add_scaled_square_input(1179, 1182, 1.0, 1162, (-(4.0 * 0.02)));
        }

        if (s.b[1664] && (!s.b[1665])) {
            s.store_sqrt_add_scaled_square_input(1179, 1182, 1.0, 1162, (4.0 * 0.02));
        }

        if s.b[1664] {
            s.store_add_scaled_inputs3_indices(1148, 1162, 1.0, 1182, (-0.5), 1179, (-0.5));
            s.store_sub(1415, 1162, 1148);
        }

        s.b[1666] = (s.v[1415] < 0.0);
        s.v[1666] = if s.b[1666] { 1.0 } else { 0.0 };

        if (s.b[1664] && s.b[1666]) {
            s.store_scalar(1415, 0.0);
        }

        s.b[1667] = (s.v[737] == 0.0);
        s.v[1667] = if s.b[1667] { 1.0 } else { 0.0 };

        if (s.b[1664] && s.b[1667]) {
            s.store_scalar(1416, 0.0);
        }

        if (s.b[1664] && (!s.b[1667])) {
            s.store_add_scaled_inputs4_indices(1179, 1161, 1.0, 1210, (-1.0), 1148, -1.0, 1177, -1.0);
        }

        s.b[1668] = (s.v[1179] < 0.0);
        s.v[1668] = if s.b[1668] { 1.0 } else { 0.0 };

        if ((s.b[1664] && (!s.b[1667])) && s.b[1668]) {
            s.store_div(1180, 1179, 737);
        }

        if ((s.b[1664] && (!s.b[1667])) && (!s.b[1668])) {
            s.store_mul_scaled_offset_ad_rhs(1180, 737, 1.0 / (2.0), A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1179), 4.0, s.ad_value(737), s.ad_value(737), 1.0), 1.0)), (-1.0));
        }

        if (s.b[1664] && (!s.b[1667])) {
            s.store_add_scaled_inputs4_mixed_iaii(1416, 1161, 1.0, A::square(s.ad_value(1180)), -1.0, 1160, -1.0, 1162, -1.0);
        }

        if (!s.b[1664]) {
            s.store_scalar(1162, 0.0);
            s.store_scalar(1409, 0.0);
            s.store_scalar(1415, 0.0);
            s.store_scalar(1416, 0.0);
        }

        if (s.v[356] != 0.0) {
            s.store_mul(1179, 1168, 578);
            s.store_div_scaled_inputs2_indices(1362, 1161, 1.0, 768, (-s.v[36]), 1179, 1.0);
        }

        s.b[1669] = (s.v[1362] > 100.0);
        s.v[1669] = if s.b[1669] { 1.0 } else { 0.0 };

        if ((s.v[356] != 0.0) && s.b[1669]) {
            s.store_sub_scaled_inputs(1412, 1161, 1.0, 768, s.v[36]);
        }

        s.b[1670] = (s.v[1362] < (-100.0));
        s.v[1670] = if s.b[1670] { 1.0 } else { 0.0 };

        if (((s.v[356] != 0.0) && (!s.b[1669])) && s.b[1670]) {
            s.store_scale(1412, 1179, (((1.0 + 3.720075976e-44)) as f64).ln());
        }

        if (((s.v[356] != 0.0) && (!s.b[1669])) && (!s.b[1670])) {
            s.store_exp(1363, 1362);
            s.store_mul_ln_ad_rhs(1412, 1179, A::offset(s.ad_value(1363), 1.0));
        }

        if (s.v[356] != 0.0) {
            s.store_mul(1181, 1161, 1412);
            s.copy_ad(1190, 730);
            s.copy_ad(1191, 731);
            s.store_add_scaled_product_indices(1182, 573, (-1.0), 572, 574, 1.0);
            s.store_mul(1183, 573, 574);
            s.store_mul_sub_ad_rhs(1184, 1191, A::add_scaled_product(s.ad_value(572), 1.0, s.ad_value(1182), s.ad_value(1416), 1.0), A::mul3(s.ad_value(1183), s.ad_value(1416), s.ad_value(1416)));
        }

        s.b[1671] = (s.v[1184] > 100.0);
        s.v[1671] = if s.b[1671] { 1.0 } else { 0.0 };

        if ((s.v[356] != 0.0) && s.b[1671]) {
            s.store_scalar(1185, 2.688117142e43);
        }

        s.b[1672] = (s.v[1184] < (-100.0));
        s.v[1672] = if s.b[1672] { 1.0 } else { 0.0 };

        if (((s.v[356] != 0.0) && (!s.b[1671])) && s.b[1672]) {
            s.store_scalar(1185, 3.720075976e-44);
        }

        if (((s.v[356] != 0.0) && (!s.b[1671])) && (!s.b[1672])) {
            s.store_exp(1185, 1184);
        }

        if (s.v[356] != 0.0) {
            s.store_mul_neg_lhs(1186, 579, 1158);
            s.store_offset_square(1187, 1186, 0.0002);
        }

        s.b[1673] = (s.v[1186] > 100.0);
        s.v[1673] = if s.b[1673] { 1.0 } else { 0.0 };

        if ((s.v[356] != 0.0) && s.b[1673]) {
            s.store_scalar(1188, 2.688117142e43);
        }

        s.b[1674] = (s.v[1186] < (-100.0));
        s.v[1674] = if s.b[1674] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
    ) {
        if (((s.v[356] != 0.0) && (!s.b[1673])) && s.b[1674]) {
            s.store_scalar(1188, 3.720075976e-44);
        }

        if (((s.v[356] != 0.0) && (!s.b[1673])) && (!s.b[1674])) {
            s.store_exp(1188, 1186);
        }

        if (s.v[356] != 0.0) {
            s.store_offset(1180, 1188, (((-1.0)) + (0.0001)));
            s.store_div_scaled_inputs2_indices(1189, 1180, 1.0, 1186, (-1.0), 1187, 1.0);
            s.store_offset(1180, 1188, (((-1.0)) + ((-0.0001))));
            s.store_div_scaled_add_product(1189, s.ad_value(1180), (-1.0), s.ad_value(1186), s.ad_value(1188), 1.0, s.ad_value(1187), 1.0);
            s.store_sub(1179, 1157, 736);
            s.store_sqrt_square_offset(1360, 1179, 0.0001);
            s.store_mul(1181, 1157, 1360);
            s.copy_ad(1299, 733);
            s.copy_ad(1300, 734);
            s.copy_ad(1191, 735);
            s.store_add_scaled_product_indices(1182, 576, (-1.0), 575, 577, 1.0);
            s.store_mul(1183, 576, 577);
            s.store_mul_sub_ad_rhs(1184, 1191, A::add_scaled_product(s.ad_value(575), 1.0, s.ad_value(1182), s.ad_value(1360), 1.0), A::mul3(s.ad_value(1183), s.ad_value(1360), s.ad_value(1360)));
        }

        s.b[1675] = (s.v[1184] > 100.0);
        s.v[1675] = if s.b[1675] { 1.0 } else { 0.0 };

        if ((s.v[356] != 0.0) && s.b[1675]) {
            s.store_scalar(1185, 2.688117142e43);
        }

        s.b[1676] = (s.v[1184] < (-100.0));
        s.v[1676] = if s.b[1676] { 1.0 } else { 0.0 };

        if (((s.v[356] != 0.0) && (!s.b[1675])) && s.b[1676]) {
            s.store_scalar(1185, 3.720075976e-44);
        }

        if (((s.v[356] != 0.0) && (!s.b[1675])) && (!s.b[1676])) {
            s.store_exp(1185, 1184);
        }

        if (s.v[356] != 0.0) {
            s.store_sub(1179, 1156, 736);
            s.store_sqrt_square_offset(1361, 1179, 0.0001);
            s.store_mul(1181, 1156, 1361);
            s.store_mul_sub_ad_rhs(1184, 1191, A::add_scaled_product(s.ad_value(575), 1.0, s.ad_value(1182), s.ad_value(1361), 1.0), A::mul3(s.ad_value(1183), s.ad_value(1361), s.ad_value(1361)));
        }

        s.b[1677] = (s.v[1184] > 100.0);
        s.v[1677] = if s.b[1677] { 1.0 } else { 0.0 };

        if ((s.v[356] != 0.0) && s.b[1677]) {
            s.store_scalar(1185, 2.688117142e43);
        }

        s.b[1678] = (s.v[1184] < (-100.0));
        s.v[1678] = if s.b[1678] { 1.0 } else { 0.0 };

        if (((s.v[356] != 0.0) && (!s.b[1677])) && s.b[1678]) {
            s.store_scalar(1185, 3.720075976e-44);
        }

        if (((s.v[356] != 0.0) && (!s.b[1677])) && (!s.b[1678])) {
            s.store_exp(1185, 1184);
        }

        s.b[1679] = ((s.v[355] != 0.0) && (s.v[57] != 2.0));
        s.v[1679] = if s.b[1679] { 1.0 } else { 0.0 };

        if s.b[1679] {
            s.store_scalar(1411, s.v[706]);
            s.copy_ad(1410, 1416);
            s.store_scalar(1179, s.v[374]);
            s.store_offset_sub(1180, 1179, 1410, (-s.v[375]));
            s.store_sqrt_add_scaled_square_input(1182, 1180, 1.0, 1179, (4.0 * s.v[375]));
            s.store_add_scaled_inputs3_indices(1414, 1179, 1.0, 1180, (-0.5), 1182, (-0.5));
            s.copy_ad(1410, 1414);
            s.store_scaled_offset(1179, 1410, (-s.v[362]), 1.0 / (s.v[363]));
        }

        s.b[1680] = (s.v[1179] > 100.0);
        s.v[1680] = if s.b[1680] { 1.0 } else { 0.0 };

        if (s.b[1679] && s.b[1680]) {
            s.store_scaled_offset(1180, 1179, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1681] = (s.v[1179] < (-100.0));
        s.v[1681] = if s.b[1681] { 1.0 } else { 0.0 };

        if ((s.b[1679] && (!s.b[1680])) && s.b[1681]) {
            s.store_scalar(1180, 3.720075976e-44);
        }

        if ((s.b[1679] && (!s.b[1680])) && (!s.b[1681])) {
            s.store_exp(1180, 1179);
        }

        if s.b[1679] {
            s.store_scaled_ln_ad(1412, A::offset(s.ad_value(1180), 1.0), s.v[363]);
        }

        s.b[1682] = (s.v[366] != 0.0);
        s.v[1682] = if s.b[1682] { 1.0 } else { 0.0 };

        if (s.b[1679] && s.b[1682]) {
            s.store_sub_from_scalar_scaled_input(1179, 1.0, 1410, 1.0 / (s.v[366]));
        }

        if (s.b[1679] && (!s.b[1682])) {
            s.store_scalar(1179, 1.0);
        }

        s.b[1683] = (s.v[1179] < 0.01);
        s.v[1683] = if s.b[1683] { 1.0 } else { 0.0 };

        if (s.b[1679] && s.b[1683]) {
            s.store_scalar(1179, 0.01);
        }

        if s.b[1679] {
            s.store_mul_ad_product_lhs(1180, A::scale_offset(s.ad_value(1228), (s.v[1227] * 1.0 / (s.v[59])), (s.v[64] / s.v[39])), s.ad_value(784), 1411);
            s.store_scale(1181, 785, s.v[357]);
            s.copy_ad(1182, 609);
            s.copy_ad(1183, 610);
            s.store_div_scaled_product_right_ad(1185, 1181, A::add_scaled_product(s.ad_value(1182), 1.0, s.ad_value(1183), s.ad_value(1410), (-1.0)), 1.0, 1179, 1.0);
        }

        s.b[1684] = (s.v[1185] > 100.0);
        s.v[1684] = if s.b[1684] { 1.0 } else { 0.0 };

        if (s.b[1679] && s.b[1684]) {
            s.store_scaled_offset(1184, 1185, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1685] = (s.v[1185] < (-100.0));
        s.v[1685] = if s.b[1685] { 1.0 } else { 0.0 };

        if ((s.b[1679] && (!s.b[1684])) && s.b[1685]) {
            s.store_scalar(1184, 3.720075976e-44);
        }

        if ((s.b[1679] && (!s.b[1684])) && (!s.b[1685])) {
            s.store_exp(1184, 1185);
        }

        if s.b[1679] {
            s.copy_ad(1410, 1415);
            s.store_scalar(1179, s.v[374]);
            s.store_offset_sub(1180, 1179, 1410, (-s.v[375]));
            s.store_sqrt_add_scaled_square_input(1182, 1180, 1.0, 1179, (4.0 * s.v[375]));
            s.store_add_scaled_inputs3_indices(1414, 1179, 1.0, 1180, (-0.5), 1182, (-0.5));
            s.copy_ad(1410, 1414);
            s.store_scaled_sub(1179, 1162, 1409, 1.0 / (s.v[367]));
        }

        s.b[1686] = (s.v[1179] > 100.0);
        s.v[1686] = if s.b[1686] { 1.0 } else { 0.0 };

        if (s.b[1679] && s.b[1686]) {
            s.store_scaled_offset(1180, 1179, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1687] = (s.v[1179] < (-100.0));
        s.v[1687] = if s.b[1687] { 1.0 } else { 0.0 };

        if ((s.b[1679] && (!s.b[1686])) && s.b[1687]) {
            s.store_scalar(1180, 3.720075976e-44);
        }

        if ((s.b[1679] && (!s.b[1686])) && (!s.b[1687])) {
            s.store_exp(1180, 1179);
        }

        if s.b[1679] {
            s.store_scaled_ln_ad(1412, A::offset(s.ad_value(1180), 1.0), s.v[367]);
        }

        s.b[1688] = (s.v[370] != 0.0);
        s.v[1688] = if s.b[1688] { 1.0 } else { 0.0 };

        if (s.b[1679] && s.b[1688]) {
            s.store_sub_from_scalar_scaled_input(1179, 1.0, 1410, 1.0 / (s.v[370]));
        }

        if (s.b[1679] && (!s.b[1688])) {
            s.store_scalar(1179, 1.0);
        }

        s.b[1689] = (s.v[1179] < 0.01);
        s.v[1689] = if s.b[1689] { 1.0 } else { 0.0 };

        if (s.b[1679] && s.b[1689]) {
            s.store_scalar(1179, 0.01);
        }

        if s.b[1679] {
            s.store_mul_ad_product_lhs(1180, A::scale_offset(s.ad_value(1228), (s.v[1227] * 1.0 / (s.v[59])), (s.v[64] / s.v[39])), s.ad_value(786), 1411);
            s.store_scale(1181, 787, s.v[357]);
            s.copy_ad(1182, 611);
            s.copy_ad(1183, 612);
            s.store_div_scaled_product_right_ad(1185, 1181, A::add_scaled_product(s.ad_value(1182), 1.0, s.ad_value(1183), s.ad_value(1410), (-1.0)), 1.0, 1179, 1.0);
        }

        s.b[1690] = (s.v[1185] > 100.0);
        s.v[1690] = if s.b[1690] { 1.0 } else { 0.0 };

        if (s.b[1679] && s.b[1690]) {
            s.store_scaled_offset(1184, 1185, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1691] = (s.v[1185] < (-100.0));
        s.v[1691] = if s.b[1691] { 1.0 } else { 0.0 };

        if ((s.b[1679] && (!s.b[1690])) && s.b[1691]) {
            s.store_scalar(1184, 3.720075976e-44);
        }

        if ((s.b[1679] && (!s.b[1690])) && (!s.b[1691])) {
            s.store_exp(1184, 1185);
        }

        if s.b[1679] {
            s.store_add(1460, 1162, 781);
        }

        s.b[1693] = (((((s.v[355] != 0.0) && (s.v[57] != 2.0)) && (s.v[760] != 0.0)) && (s.v[63] > 0.0)) && (s.v[1447] < s.v[1460]));
        s.v[1693] = if s.b[1693] { 1.0 } else { 0.0 };

        if s.b[1693] {
            s.store_sub(1179, 1447, 1460);
            s.store_sqrt_square_offset(1180, 1179, 0.0001);
            s.store_offset_scaled_sub(1446, 1180, 1179, 0.5, (((-0.01)) * (0.5)));
        }

        if s.b[1693] {
            if (s.v[36] == 1.0) {
                s.copy_ad(1190, 788);
            } else {
                s.copy_ad(1190, 789);
            }
        }

        if s.b[1693] {
            if (s.v[36] == 1.0) {
                s.copy_ad(1191, 790);
            } else {
                s.copy_ad(1191, 791);
            }
        }

        if s.b[1693] {
            s.store_mul(1181, 1447, 1446);
            s.store_add_scaled_product_indices(1182, 614, (-1.0), 613, 615, 1.0);
            s.store_mul(1183, 614, 615);
            s.store_mul_sub_scaled_inputs_rhs(1184, 1191, A::add_scaled_product(s.ad_value(613), 1.0, s.ad_value(1182), s.ad_value(1446), 1.0), (-s.v[357]), A::mul3(s.ad_value(1183), s.ad_value(1446), s.ad_value(1446)), (-s.v[357]));
        }

        s.b[1694] = (s.v[1184] > 100.0);
        s.v[1694] = if s.b[1694] { 1.0 } else { 0.0 };

        if (s.b[1693] && s.b[1694]) {
            s.store_scalar(1185, 2.688117142e43);
        }

        s.b[1695] = (s.v[1184] < (-100.0));
        s.v[1695] = if s.b[1695] { 1.0 } else { 0.0 };

        if ((s.b[1693] && (!s.b[1694])) && s.b[1695]) {
            s.store_scalar(1185, 3.720075976e-44);
        }

        if ((s.b[1693] && (!s.b[1694])) && (!s.b[1695])) {
            s.store_exp(1185, 1184);
        }

        if s.b[1693] {
            s.store_scale(1190, 1190, (s.v[63] * s.v[706]));
        }

        s.b[1696] = (s.v[57] != 2.0);
        s.v[1696] = if s.b[1696] { 1.0 } else { 0.0 };

        s.b[1697] = (s.v[71] == 0.0);
        s.v[1697] = if s.b[1697] { 1.0 } else { 0.0 };

        s.b[1698] = (s.v[570] <= 0.0);
        s.v[1698] = if s.b[1698] { 1.0 } else { 0.0 };

        if ((s.b[1696] && s.b[1697]) && (!s.b[1698])) {
            s.store_add_scaled_product_right_ad(1301, 639, (-1.0 / (s.v[1227])), 638, A::scale_offset(s.ad_value(771), s.v[289], (((((-1.0)) * (s.v[289]))) + (1.0))), 1.0);
            s.store_scale(1179, 640, s.v[1227]);
            s.store_div_scaled_product_offset_denominator(1180, s.ad_value(641), s.ad_value(1179), 1.0, s.ad_value(1179), 1.0, 1.0);
            s.store_div_from_scalar_offset_product(1179, 1.0, 642, 1210, 1.0);
            s.store_add(1182, 1179, 643);
            s.store_mul(1181, 1166, 1182);
            s.store_div_from_scalar_offset_product(1182, 1.0, 644, 1158, 1.0);
            s.store_mul3_lhs(1302, 1180, 1181, 1182);
            s.store_add(1256, 1301, 1302);
            s.store_sub(1304, 1158, 1256);
            s.store_add_ad(1179, A::add_scaled_product(s.ad_value(637), 1.0, s.ad_value(636), s.ad_value(1304), 1.0), A::mul3(s.ad_value(571), s.ad_value(1304), s.ad_value(1304)));
        }

        s.b[1699] = (s.v[1179] < 1e-5);
        s.v[1699] = if s.b[1699] { 1.0 } else { 0.0 };

        if (((s.b[1696] && s.b[1697]) && (!s.b[1698])) && s.b[1699]) {
            s.store_scalar(1179, 1e-5);
        }

        if ((s.b[1696] && s.b[1697]) && (!s.b[1698])) {
            s.store_add_ad_rhs(1179, 1220, A::mul3(s.ad_value(630), s.ad_value(759), s.ad_value(1268)));
        }

        s.b[1703] = (s.v[570] <= 0.0);
        s.v[1703] = if s.b[1703] { 1.0 } else { 0.0 };

        if ((s.b[1696] && (!s.b[1697])) && (!s.b[1703])) {
            s.store_add_scaled_product_right_ad(1301, 639, (-1.0 / (s.v[1227])), 638, A::scale_offset(s.ad_value(771), s.v[289], (((((-1.0)) * (s.v[289]))) + (1.0))), 1.0);
            s.store_scale(1179, 640, s.v[1227]);
            s.store_div_scaled_product_offset_denominator(1180, s.ad_value(641), s.ad_value(1179), 1.0, s.ad_value(1179), 1.0, 1.0);
            s.store_div_from_scalar_offset_product(1179, 1.0, 642, 1210, 1.0);
            s.store_add(1182, 1179, 643);
            s.store_mul(1181, 1166, 1182);
            s.store_div_from_scalar_offset_product(1182, 1.0, 644, 1158, 1.0);
            s.store_mul3_lhs(1302, 1180, 1181, 1182);
            s.store_add(1256, 1301, 1302);
        }

    }

    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1696] && (!s.b[1697])) && (!s.b[1703])) {
            s.store_sub(1304, 1158, 1256);
            s.store_add_ad(1179, A::add_scaled_product(s.ad_value(637), 1.0, s.ad_value(636), s.ad_value(1304), 1.0), A::mul3(s.ad_value(571), s.ad_value(1304), s.ad_value(1304)));
        }

        s.b[1704] = (s.v[1179] < 1e-5);
        s.v[1704] = if s.b[1704] { 1.0 } else { 0.0 };

        if (((s.b[1696] && (!s.b[1697])) && (!s.b[1703])) && s.b[1704]) {
            s.store_scalar(1179, 1e-5);
        }

        if ((s.b[1696] && (!s.b[1697])) && (!s.b[1703])) {
            s.copy_ad(1179, 1220);
        }

        if (s.b[1696] && (!s.b[1697])) {
            s.store_add_scaled_inputs(1179, 632, 1.0 / (s.v[1227]), 631, (s.v[1227] * 1.0 / (s.v[1227])));
            s.store_mul_scale_offset_rhs(1438, 633, 771, s.v[301], (((((-1.0)) * (s.v[301]))) + (1.0)));
        }

        s.b[1708] = (s.v[759] > 0.0);
        s.v[1708] = if s.b[1708] { 1.0 } else { 0.0 };

        if ((s.b[1696] && (!s.b[1697])) && s.b[1708]) {
            s.store_sub(1180, 1438, 1422);
        }

        if ((s.b[1696] && (!s.b[1697])) && (!s.b[1708])) {
            s.store_sub(1180, 1438, 1421);
        }

        if (s.b[1696] && (!s.b[1697])) {
            s.store_offset(1181, 635, (-1.0));
        }

        s.b[1709] = (s.v[1180] <= 0.0);
        s.v[1709] = if s.b[1709] { 1.0 } else { 0.0 };

        if ((s.b[1696] && (!s.b[1697])) && s.b[1709]) {
            s.store_scalar(1182, 0.0);
        }

        if ((s.b[1696] && (!s.b[1697])) && (!s.b[1709])) {
            s.store_mul_scaled_pow_ad_rhs(1182, 634, -1.0, s.ad_value(1180), s.ad_value(1181));
        }

        s.b[1710] = (s.v[1182] > 100.0);
        s.v[1710] = if s.b[1710] { 1.0 } else { 0.0 };

        if ((s.b[1696] && (!s.b[1697])) && s.b[1710]) {
            s.store_scalar(1183, 2.688117142e43);
        }

        s.b[1711] = (s.v[1182] < (-100.0));
        s.v[1711] = if s.b[1711] { 1.0 } else { 0.0 };

        if (((s.b[1696] && (!s.b[1697])) && (!s.b[1710])) && s.b[1711]) {
            s.store_scalar(1183, 3.720075976e-44);
        }

        if (((s.b[1696] && (!s.b[1697])) && (!s.b[1710])) && (!s.b[1711])) {
            s.store_exp(1183, 1182);
        }

        s.b[1712] = ((s.v[760] == 0.0) || (s.v[760] == 2.0));
        s.v[1712] = if s.b[1712] { 1.0 } else { 0.0 };

        s.b[1713] = (s.v[526] < 0.001);
        s.v[1713] = if s.b[1713] { 1.0 } else { 0.0 };

        s.b[1714] = (s.v[427] <= 0.001);
        s.v[1714] = if s.b[1714] { 1.0 } else { 0.0 };

        if (((s.b[1696] && (!s.b[1712])) && s.b[1713]) && s.b[1714]) {
            s.store_scalar(1179, (1.0 / 0.001));
        }

        if (((s.b[1696] && (!s.b[1712])) && s.b[1713]) && (!s.b[1714])) {
            s.store_scalar(1179, (1.0 / s.v[427]));
        }

        s.b[1715] = (s.v[66] > 1.0);
        s.v[1715] = if s.b[1715] { 1.0 } else { 0.0 };

        if s.b[1715] {
            s.store_mul(1188, 596, 409);
            s.store_mul(1179, 1188, 1215);
            s.store_mul_add_rhs(413, 595, 1179, 1420);
        }

        s.b[1716] = (s.v[39] != 1.0);
        s.v[1716] = if s.b[1716] { 1.0 } else { 0.0 };

        if (s.b[1715] && s.b[1716]) {
            s.store_scale(413, 413, s.v[39]);
        }

        s.b[1717] = (s.v[66] == 2.0);
        s.v[1717] = if s.b[1717] { 1.0 } else { 0.0 };

        if (s.b[1715] && s.b[1717]) {
            s.store_add(1190, 421, 413);
            s.store_div_scaled_product_indices(413, 421, 413, 1.0, 1190, 1.0);
        }

        if (!s.b[1715]) {
            s.store_scalar(413, 0.0);
        }

        s.b[1718] = (s.v[403] == 1.0);
        s.v[1718] = if s.b[1718] { 1.0 } else { 0.0 };

        if s.b[1718] {
            s.store_scalar(1222, 0.0);
            s.store_sub(1179, 1157, 736);
            s.store_sqrt_square_offset(1180, 1179, 0.0001);
            s.store_scaled_add(1360, 1179, 1180, 0.5);
            s.store_offset_mul(1179, 553, 1360, 1.0);
            s.store_mul_neg_lhs(1180, 554, 1154);
            s.store_add_ad_lhs(1181, A::div_from_scalar(1.0, s.ad_value(1179)), 1180);
            s.store_add_ad_rhs(1182, 1181, A::sqrt_square_offset(s.ad_value(1181), 0.01));
            s.store_scale(1183, 1430, 0.5);
            s.store_sub(1179, 1156, 736);
            s.store_sqrt_square_offset(1180, 1179, 0.0001);
            s.store_scaled_add(1361, 1179, 1180, 0.5);
            s.store_offset_mul(1179, 553, 1361, 1.0);
            s.store_mul_neg_lhs(1180, 554, 1153);
            s.store_add_ad_lhs(1181, A::div_from_scalar(1.0, s.ad_value(1179)), 1180);
            s.store_add_ad_rhs(1182, 1181, A::sqrt_square_offset(s.ad_value(1181), 0.01));
            s.store_scale(1183, 1429, 0.5);
        }

        s.store_mul_sub_from_scalar_ad_rhs(1180, 1210, 1.0, A::div_scaled_product(s.ad_value(1195), s.ad_value(1211), 0.5, s.ad_value(1225), 1.0));

        s.b[1720] = (s.v[39] != 1.0);
        s.v[1720] = if s.b[1720] { 1.0 } else { 0.0 };

        if s.b[1720] {
            s.store_scale(1220, 1220, s.v[39]);
            s.store_scale(1268, 1268, s.v[39]);
            s.store_scale(454, 454, s.v[39]);
        }

        s.store_scalar(439, (A::ddx_projection(&s.ad_value(1220), Some(9), None) * s.v[36]));

        s.b[1721] = (s.v[759] > 0.0);
        s.v[1721] = if s.b[1721] { 1.0 } else { 0.0 };

        if s.b[1721] {
            s.store_scalar(440, (A::ddx_projection(&s.ad_value(1220), Some(7), None) * s.v[36]));
        }

        if (!s.b[1721]) {
            s.store_scalar(440, (A::ddx_projection(&s.ad_value(1220), Some(8), None) * s.v[36]));
        }

        s.store_scalar(441, (A::ddx_projection(&s.ad_value(1220), Some(5), None) * s.v[36]));

        s.store_scale(1178, 757, ((((s.v[693] / s.v[59]) * s.v[39]) * s.v[692]) + s.v[62]));

        s.store_scale(1316, 757, (s.v[342] * ((((s.v[693] / s.v[59]) * s.v[39]) * s.v[726]) + s.v[62])));

        s.store_scale(1448, 757, s.v[63]);

        s.store_scale(1449, 757, (s.v[342] * s.v[63]));

        s.store_sub(1166, 1161, 1407);

        s.store_mul(1189, 1393, 1168);

        s.store_div_scaled_product_indices(1145, 745, 1166, 1.0, 1189, 1.0);

        s.store_mul3_lhs(1351, 1393, 724, 1168);

        s.store_mul3_lhs(1352, 1393, 725, 1168);

        s.b[1722] = (s.v[69] == 0.0);
        s.v[1722] = if s.b[1722] { 1.0 } else { 0.0 };

        s.b[1723] = ((s.v[1145] > (-100.0)) && (s.v[1145] < 100.0));
        s.v[1723] = if s.b[1723] { 1.0 } else { 0.0 };

        if (s.b[1722] && s.b[1723]) {
            let assign29060_ad_e22527: A = A::exp(s.ad_value(1145));
            s.store_square_ad(1146, assign29060_ad_e22527);
        }

        if (s.b[1722] && s.b[1723]) {
            s.store_mul_ad_rhs(1146, 1146, A::exp_scaled_input(A::div(s.ad_value(685), s.ad_value(1351)), -1.0));
        }

        if (s.b[1722] && s.b[1723]) {
            s.store_mul_ad_rhs(1210, 1351, {
                if ((1.0 + s.v[1146]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(1146), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        s.b[1724] = (s.v[63] > 0.0);
        s.v[1724] = if s.b[1724] { 1.0 } else { 0.0 };

        if ((s.b[1722] && s.b[1723]) && s.b[1724]) {
            s.store_mul_exp_ad_rhs(1450, 1146, A::div_scaled_value_by_product(s.ad_value(781), -1.0, s.ad_value(1352), A::square(s.ad_value(1168)), 1.0));
        }

        if ((s.b[1722] && s.b[1723]) && s.b[1724]) {
            s.store_mul_ad_rhs(1451, 1352, {
                if ((1.0 + s.v[1450]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(1450), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        s.b[1725] = (s.v[69] == 1.0);
        s.v[1725] = if s.b[1725] { 1.0 } else { 0.0 };

        s.b[1726] = ((s.v[1145] > (-100.0)) && (s.v[1145] < 100.0));
        s.v[1726] = if s.b[1726] { 1.0 } else { 0.0 };

        if (((!s.b[1722]) && s.b[1725]) && s.b[1726]) {
            s.store_exp_ad(1146, A::div(s.ad_value(1145), A::mul(s.ad_value(745), s.ad_value(724))));
            s.store_mul_ad_rhs(1146, 1146, A::exp_scaled_input(A::div(s.ad_value(685), s.ad_value(1351)), -1.0));
        }

        if (((!s.b[1722]) && s.b[1725]) && s.b[1726]) {
            s.store_mul_ad_rhs(1210, 1351, {
                if ((1.0 + s.v[1146]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(1146), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        s.b[1727] = (s.v[63] > 0.0);
        s.v[1727] = if s.b[1727] { 1.0 } else { 0.0 };

        if ((((!s.b[1722]) && s.b[1725]) && s.b[1726]) && s.b[1727]) {
            s.store_mul_exp_ad_rhs(1450, 1146, A::div_scaled_value_by_product(s.ad_value(781), -1.0, s.ad_value(1352), A::square(s.ad_value(1168)), 1.0));
        }

        if ((((!s.b[1722]) && s.b[1725]) && s.b[1726]) && s.b[1727]) {
            s.store_mul_ad_rhs(1451, 1352, {
                if ((1.0 + s.v[1450]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(1450), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((!s.b[1722]) && (!s.b[1725])) {
            s.store_div_scaled_product_right_ad(1145, 749, A::sub(s.ad_value(1166), s.ad_value(685)), 1.0, 1351, 1.0);
            s.store_div_scaled_inputs2_mixed_iai(1169, 751, 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(749), A::sub(s.ad_value(1166), s.ad_value(685))), (-1.0), 1351, 1.0);
        }

        s.b[1728] = (s.v[1145] > 100.0);
        s.v[1728] = if s.b[1728] { 1.0 } else { 0.0 };

        if (((!s.b[1722]) && (!s.b[1725])) && s.b[1728]) {
            s.store_sub(1210, 1166, 685);
        }

        s.b[1729] = (s.v[1169] > 100.0);
        s.v[1729] = if s.b[1729] { 1.0 } else { 0.0 };

        if ((((!s.b[1722]) && (!s.b[1725])) && (!s.b[1728])) && s.b[1729]) {
            s.store_div_scaled_inputs3_indices(1179, 1166, 1.0, 685, (-1.0), 751, -1.0, 1351, 1.0);
            s.store_exp(1146, 1179);
            s.store_mul_div_scaled_product_indices(1210, 1146, 1168, 1473, 1.0, 757, 1.0);
        }

        if ((((!s.b[1722]) && (!s.b[1725])) && (!s.b[1728])) && (!s.b[1729])) {
            s.store_exp(1146, 1145);
        }

        if ((((!s.b[1722]) && (!s.b[1725])) && (!s.b[1728])) && (!s.b[1729])) {
            s.store_mul_ad_rhs(1180, 1351, {
                if ((1.0 + s.v[1146]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(1146), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((((!s.b[1722]) && (!s.b[1725])) && (!s.b[1728])) && (!s.b[1729])) {
            s.store_mul3_ad(1192, A::div_scaled_inputs(s.ad_value(757), -1.0, A::mul(s.ad_value(1168), s.ad_value(1473)), 1.0), A::exp(s.ad_value(1169)), A::sub_from_scalar(1.0, s.ad_value(749)));
            s.store_sub_ad_rhs(1181, 749, A::div_scaled_product(s.ad_value(1351), s.ad_value(1192), 1.0, A::sub_from_scalar(1.0, s.ad_value(749)), 1.0));
            s.store_div(1210, 1180, 1181);
        }

        s.b[1730] = (s.v[63] > 0.0);
        s.v[1730] = if s.b[1730] { 1.0 } else { 0.0 };

        if (((!s.b[1722]) && (!s.b[1725])) && s.b[1730]) {
            s.store_div_scaled_product_right_ad(1452, 749, A::add_scaled_inputs3(s.ad_value(1166), 1.0, s.ad_value(685), (-1.0), s.ad_value(781), -1.0), 1.0, 1352, 1.0);
            s.store_div_scaled_inputs2_mixed_iai(1453, 751, 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(749), A::add_scaled_inputs3(s.ad_value(1166), 1.0, s.ad_value(685), (-1.0), s.ad_value(781), -1.0)), (-1.0), 1352, 1.0);
        }

        s.b[1731] = (s.v[1452] > 100.0);
        s.v[1731] = if s.b[1731] { 1.0 } else { 0.0 };

        if ((((!s.b[1722]) && (!s.b[1725])) && s.b[1730]) && s.b[1731]) {
            s.store_add_scaled_inputs3_indices(1451, 1166, 1.0, 685, (-1.0), 781, -1.0);
        }

        s.b[1732] = (s.v[1453] > 100.0);
        s.v[1732] = if s.b[1732] { 1.0 } else { 0.0 };

        if (((((!s.b[1722]) && (!s.b[1725])) && s.b[1730]) && (!s.b[1731])) && s.b[1732]) {
            s.store_div_scaled_inputs4_indices(1179, 1166, 1.0, 685, (-1.0), 751, -1.0, 781, -1.0, 1352, 1.0);
            s.store_exp(1450, 1179);
            s.store_mul_div_scaled_product_indices(1451, 1450, 1168, 1473, 1.0, 757, 1.0);
        }

        if (((((!s.b[1722]) && (!s.b[1725])) && s.b[1730]) && (!s.b[1731])) && (!s.b[1732])) {
            s.store_exp(1450, 1452);
        }

        if (((((!s.b[1722]) && (!s.b[1725])) && s.b[1730]) && (!s.b[1731])) && (!s.b[1732])) {
            s.store_mul_ad_rhs(1180, 1352, {
                if ((1.0 + s.v[1450]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(1450), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (((((!s.b[1722]) && (!s.b[1725])) && s.b[1730]) && (!s.b[1731])) && (!s.b[1732])) {
            s.store_mul3_ad(1192, A::div_scaled_inputs(s.ad_value(757), -1.0, A::mul(s.ad_value(1168), s.ad_value(1473)), 1.0), A::exp(s.ad_value(1453)), A::sub_from_scalar(1.0, s.ad_value(749)));
            s.store_sub_ad_rhs(1181, 749, A::div_scaled_product(s.ad_value(1352), s.ad_value(1192), 1.0, A::sub_from_scalar(1.0, s.ad_value(749)), 1.0));
            s.store_div(1451, 1180, 1181);
        }

        s.copy_ad(1165, 1407);

        s.copy_ad(1164, 1388);

        s.copy_ad(1177, 1378);

        s.b[1733] = (s.v[88] == 2.0);
        s.v[1733] = if s.b[1733] { 1.0 } else { 0.0 };

        s.b[1734] = (s.v[57] == 2.0);
        s.v[1734] = if s.b[1734] { 1.0 } else { 0.0 };

        if (s.b[1733] && s.b[1734]) {
            s.store_scalar(1273, 0.0);
            s.store_scalar(1272, 0.0);
        }

        if (s.b[1733] && (!s.b[1734])) {
            s.store_add_ad_lhs(1162, A::add_scaled_inputs_product(s.ad_value(1165), 1.0, s.ad_value(1277), (-1.0), s.ad_value(707), s.ad_value(1164), (-1.0)), 685);
            s.store_add_scaled_inputs3_offset_indices(1149, 1162, 1.0, 1161, (-1.0), 1177, 1.0, (-0.08));
        }

        s.b[1735] = (s.v[1162] <= 0.0);
        s.v[1735] = if s.b[1735] { 1.0 } else { 0.0 };

        if ((s.b[1733] && (!s.b[1734])) && s.b[1735]) {
            s.store_sqrt_add_scaled_square_input(1179, 1149, 1.0, 1162, (-(4.0 * 0.08)));
        }

        if ((s.b[1733] && (!s.b[1734])) && (!s.b[1735])) {
            s.store_sqrt_add_scaled_square_input(1179, 1149, 1.0, 1162, (4.0 * 0.08));
        }

        if (s.b[1733] && (!s.b[1734])) {
            s.store_add_scaled_inputs3_indices(1148, 1162, 1.0, 1149, (-0.5), 1179, (-0.5));
            s.store_mul_sub_rhs(1273, 1316, 1148, 1162);
        }

        s.b[1736] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));
        s.v[1736] = if s.b[1736] { 1.0 } else { 0.0 };

        if ((s.b[1733] && (!s.b[1734])) && s.b[1736]) {
            s.store_add(1460, 1162, 781);
            s.store_scalar(1472, 0.08);
            s.store_add_scaled_inputs4_indices(1149, 1460, 1.0, 1458, (-1.0), 1177, 1.0, 1472, -1.0);
        }

        s.b[1737] = (s.v[1460] <= 0.0);
        s.v[1737] = if s.b[1737] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1733] && (!s.b[1734])) && s.b[1736]) && s.b[1737]) {
            s.store_sqrt_add_scaled_square_product(1179, 1149, 1.0, 1472, 1460, (-100.0));
        }

        if (((s.b[1733] && (!s.b[1734])) && s.b[1736]) && (!s.b[1737])) {
            s.store_sqrt_add_scaled_square_product(1179, 1149, 1.0, 1472, 1460, 100.0);
        }

        if ((s.b[1733] && (!s.b[1734])) && s.b[1736]) {
            s.store_add_scaled_inputs3_indices(1461, 1460, 1.0, 1149, (-0.5), 1179, (-0.5));
            s.store_add_scaled_product_right_sub(1273, 1273, 1.0, 1449, 1461, 1460, 1.0);
        }

        if (s.b[1733] && (!s.b[1734])) {
            s.store_scale(1179, 737, 0.5);
            s.store_add_scaled_inputs4_indices(1182, 1161, 1.0, 1148, (-1.0), 1177, -1.0, 1210, -1.0);
        }

        s.b[1738] = (s.v[737] == 0.0);
        s.v[1738] = if s.b[1738] { 1.0 } else { 0.0 };

        if ((s.b[1733] && (!s.b[1734])) && s.b[1738]) {
            s.store_scalar(1180, 0.0);
        }

        s.b[1739] = (s.v[1182] < 0.0);
        s.v[1739] = if s.b[1739] { 1.0 } else { 0.0 };

        if (((s.b[1733] && (!s.b[1734])) && (!s.b[1738])) && s.b[1739]) {
            s.store_add_div_rhs_indices(1180, 1179, 1182, 737);
        }

        if (((s.b[1733] && (!s.b[1734])) && (!s.b[1738])) && (!s.b[1739])) {
            s.store_sqrt_square_add(1180, 1179, 1182);
        }

        if (s.b[1733] && (!s.b[1734])) {
            s.store_mul_ad_product_rhs(1272, 1316, s.ad_value(737), A::sub(s.ad_value(1180), s.ad_value(1179)));
        }

        s.b[1740] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));
        s.v[1740] = if s.b[1740] { 1.0 } else { 0.0 };

        if ((s.b[1733] && (!s.b[1734])) && s.b[1740]) {
            s.store_add_scaled_inputs4_indices(1182, 1458, 1.0, 1461, (-1.0), 1177, -1.0, 1451, -1.0);
        }

        s.b[1741] = (s.v[1182] < 0.0);
        s.v[1741] = if s.b[1741] { 1.0 } else { 0.0 };

        if (((s.b[1733] && (!s.b[1734])) && s.b[1740]) && s.b[1741]) {
            s.store_add_div_rhs_indices(1180, 1179, 1182, 737);
        }

        if (((s.b[1733] && (!s.b[1734])) && s.b[1740]) && (!s.b[1741])) {
            s.store_sqrt_square_add(1180, 1179, 1182);
        }

        if ((s.b[1733] && (!s.b[1734])) && s.b[1740]) {
            s.store_add_ad_rhs(1272, 1272, A::mul3(s.ad_value(1449), s.ad_value(737), A::sub(s.ad_value(1180), s.ad_value(1179))));
        }

        if s.b[1733] {
            s.store_scale(1229, 1196, s.v[694]);
            s.store_div(1226, 1210, 1229);
            s.store_offset_sub(1150, 1226, 1158, (-0.02));
            s.store_sqrt_add_scaled_square_input(1179, 1150, 1.0, 1226, (4.0 * 0.02));
            s.store_add_scaled_inputs3_indices(1212, 1226, 1.0, 1150, (-0.5), 1179, (-0.5));
        }

        s.b[1742] = (s.v[63] > 0.0);
        s.v[1742] = if s.b[1742] { 1.0 } else { 0.0 };

        if (s.b[1733] && s.b[1742]) {
            s.store_div(1462, 1451, 1229);
            s.store_offset_sub(1150, 1462, 1158, (-0.02));
            s.store_sqrt_add_scaled_square_input(1179, 1150, 1.0, 1462, (4.0 * 0.02));
            s.store_add_scaled_inputs3_indices(1463, 1462, 1.0, 1150, (-0.5), 1179, (-0.5));
        }

        s.b[1743] = (s.v[57] == 2.0);
        s.v[1743] = if s.b[1743] { 1.0 } else { 0.0 };

        if (s.b[1733] && s.b[1743]) {
            s.store_scalar(1341, 0.0);
        }

        if (s.b[1733] && (!s.b[1743])) {
            s.store_mul(1179, 1229, 1212);
            s.store_scaled_offset_ad(1180, A::sub_scaled_inputs(s.ad_value(1210), 1.0, s.ad_value(1179), 0.5), 1e-20, 12.0);
            s.store_div(1181, 1212, 1180);
            s.store_mul(1182, 1179, 1181);
            s.store_sub_from_scalar(1186, 1.0, 1229);
            s.store_mul_ad_product_rhs(1341, 1316, s.ad_value(1186), A::sub_scaled_inputs(s.ad_value(1212), 0.5, s.ad_value(1182), 1.0));
        }

        s.b[1744] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));
        s.v[1744] = if s.b[1744] { 1.0 } else { 0.0 };

        if ((s.b[1733] && (!s.b[1743])) && s.b[1744]) {
            s.store_mul(1179, 1229, 1463);
            s.store_scaled_offset_ad(1180, A::sub_scaled_inputs(s.ad_value(1451), 1.0, s.ad_value(1179), 0.5), 1e-20, 12.0);
            s.store_div(1181, 1463, 1180);
            s.store_mul(1182, 1179, 1181);
            s.store_sub_from_scalar(1186, 1.0, 1229);
            s.store_add_ad_rhs(1341, 1341, A::mul3(s.ad_value(1449), s.ad_value(1186), A::sub_scaled_inputs(s.ad_value(1463), 0.5, s.ad_value(1182), 1.0)));
        }

        if s.b[1733] {
            s.store_mul(1179, 1229, 1212);
            s.store_scaled_offset_ad(1180, A::sub_scaled_inputs(s.ad_value(1210), 1.0, s.ad_value(1179), 0.5), 1e-20, 12.0);
            s.store_div(1181, 1179, 1180);
            s.store_mul(1182, 1179, 1181);
            s.store_mul_add_scaled_inputs3_offset_rhs(1250, 1178, s.ad_value(1210), 1.0, s.ad_value(1179), (-0.5), s.ad_value(1182), 1.0, 0.0);
        }

        s.b[1745] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));
        s.v[1745] = if s.b[1745] { 1.0 } else { 0.0 };

        if (s.b[1733] && s.b[1745]) {
            s.store_mul(1454, 1229, 1463);
            s.store_scaled_offset_ad(1191, A::sub_scaled_inputs(s.ad_value(1451), 1.0, s.ad_value(1454), 0.5), 1e-20, 12.0);
            s.store_div(1181, 1454, 1191);
            s.store_mul(1182, 1454, 1181);
            s.store_add_scaled_product_right_ad(1250, 1250, 1.0, 1448, A::add_scaled_inputs3(s.ad_value(1451), 1.0, s.ad_value(1454), (-0.5), s.ad_value(1182), 1.0), 1.0);
        }

        s.b[1746] = (s.v[153] > 0.5);
        s.v[1746] = if s.b[1746] { 1.0 } else { 0.0 };

        if (s.b[1733] && s.b[1746]) {
            s.store_scale(1180, 1180, 2.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(1254, 1178, s.ad_value(1210), ((0.5) * (-1.0)), s.ad_value(1179), ((0.25) * (-1.0)), A::div_scaled_product(s.ad_value(1179), s.ad_value(1179), 1.0, s.ad_value(1180), 1.0), ((-1.0) * (-1.0)), 0.0);
        }

        s.b[1747] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));
        s.v[1747] = if s.b[1747] { 1.0 } else { 0.0 };

        if ((s.b[1733] && s.b[1746]) && s.b[1747]) {
            s.store_scale(1191, 1191, 2.0);
            s.store_add_scaled_product_right_ad(1254, 1254, 1.0, 1448, A::add_scaled_inputs3(s.ad_value(1451), 0.5, s.ad_value(1454), 0.25, A::div_scaled_product(s.ad_value(1454), s.ad_value(1454), 1.0, s.ad_value(1191), 1.0), -1.0), (-1.0));
        }

        s.b[1748] = (s.v[153] < 0.5);
        s.v[1748] = if s.b[1748] { 1.0 } else { 0.0 };

        if ((s.b[1733] && (!s.b[1746])) && s.b[1748]) {
            s.store_scale(1180, 1180, 0.08333333333333333);
            s.store_div_scaled_inputs_square_rhs(1181, 1178, 0.5, 1180, 1.0);
            s.store_add_scaled_product_mixed_aia(1182, A::mul3_scaled_output(s.ad_value(1179), s.ad_value(1179), s.ad_value(1179), (2.0 * 0.06666666666666667)), (-1.0), 1210, A::add_scaled_products(s.ad_value(1179), s.ad_value(1179), (2.0 * 0.3333333333333333), s.ad_value(1210), A::sub_scaled_inputs(s.ad_value(1210), 1.0, s.ad_value(1179), (4.0 * 0.3333333333333333)), 1.0), 1.0);
            s.store_mul_neg_lhs(1254, 1181, 1182);
        }

        s.b[1749] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));
        s.v[1749] = if s.b[1749] { 1.0 } else { 0.0 };

        if (((s.b[1733] && (!s.b[1746])) && s.b[1748]) && s.b[1749]) {
            s.store_scale(1191, 1191, 0.08333333333333333);
            s.store_div_scaled_inputs_square_rhs(1181, 1448, 0.5, 1191, 1.0);
            s.store_add_scaled_product_mixed_aia(1182, A::mul3_scaled_output(s.ad_value(1454), s.ad_value(1454), s.ad_value(1454), (2.0 * 0.06666666666666667)), (-1.0), 1451, A::add_scaled_products(s.ad_value(1454), s.ad_value(1454), (2.0 * 0.3333333333333333), s.ad_value(1451), A::sub_scaled_inputs(s.ad_value(1451), 1.0, s.ad_value(1454), (4.0 * 0.3333333333333333)), 1.0), 1.0);
            s.store_mul_neg_lhs(1470, 1181, 1182);
            s.store_add(1254, 1254, 1470);
        }

        if ((s.b[1733] && (!s.b[1746])) && (!s.b[1748])) {
            s.store_scaled_add(1254, 1250, 1341, (-0.5));
        }

        s.b[1750] = (s.v[57] == 2.0);
        s.v[1750] = if s.b[1750] { 1.0 } else { 0.0 };

        if (s.b[1733] && s.b[1750]) {
            s.store_scalar(1274, 0.0);
        }

        if (s.b[1733] && (!s.b[1750])) {
            s.store_scale(1249, 626, (s.v[342] * (s.v[1248] * ((((s.v[693] / s.v[59]) * s.v[39]) * s.v[727]) + s.v[65]))));
            s.store_mul_sub_rhs(1274, 1249, 1237, 1160);
        }

        if s.b[1733] {
            s.store_add_scaled_inputs3_indices(1251, 1250, 1.0, 1273, 1.0, 1272, 1.0);
            s.store_add_scaled_inputs4_indices(1252, 1341, 1.0, 1273, (-1.0), 1272, -1.0, 1274, -1.0);
            s.copy_ad(1255, 1274);
            s.store_add_scaled_inputs4_indices(1253, 1251, (-1.0), 1254, (-1.0), 1252, (-1.0), 1255, (-1.0));
        }

        s.b[1751] = (s.v[88] == 3.0);
        s.v[1751] = if s.b[1751] { 1.0 } else { 0.0 };

        s.b[1752] = (s.v[68] == 0.0);
        s.v[1752] = if s.b[1752] { 1.0 } else { 0.0 };

        if (((!s.b[1733]) && s.b[1751]) && s.b[1752]) {
            s.store_div_from_scalar(1332, 3.453133e-11, 92);
        }

        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1752])) {
            s.store_div_scaled_inputs_indices(1332, 777, 8.85418e-12, 92, 1.0);
        }

        if ((!s.b[1733]) && s.b[1751]) {
            s.store_div_scaled_product_indices(1178, 1178, 776, 1.0, 92, 1.0);
            s.store_div_scaled_inputs_indices(1316, 1316, s.v[91], 92, 1.0);
            s.store_scale(1333, 92, 100000000.0);
        }

        s.b[1753] = (s.v[63] > 0.0);
        s.v[1753] = if s.b[1753] { 1.0 } else { 0.0 };

        if (((!s.b[1733]) && s.b[1751]) && s.b[1753]) {
            s.store_div_scaled_inputs_indices(1448, 1448, s.v[91], 92, 1.0);
            s.store_div_scaled_inputs_indices(1449, 1449, s.v[91], 92, 1.0);
        }

        s.b[1754] = (s.v[57] == 2.0);
        s.v[1754] = if s.b[1754] { 1.0 } else { 0.0 };

        if (((!s.b[1733]) && s.b[1751]) && s.b[1754]) {
            s.store_scalar(1273, 0.0);
            s.store_scalar(1272, 0.0);
            s.store_scalar(1350, 0.0);
        }

        s.b[1755] = ((p.p33 == 1.0) && (p.p16 != 0.0));
        s.v[1755] = if s.b[1755] { 1.0 } else { 0.0 };

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1755]) {
            s.store_add_ad_lhs(1350, A::add_scaled_inputs_product(s.ad_value(1349), 1.0, s.ad_value(1277), (-1.0), s.ad_value(707), s.ad_value(1278), (-1.0)), 685);
        }

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && (!s.b[1755])) {
            s.store_add(1350, 424, 685);
        }

        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {
            s.store_add_scaled_inputs3_offset_indices(1149, 1350, 1.0, 1161, (-1.0), 1177, 1.0, (-0.02));
        }

        s.b[1756] = (s.v[1350] <= 0.0);
        s.v[1756] = if s.b[1756] { 1.0 } else { 0.0 };

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1756]) {
            s.store_sqrt_add_scaled_square_input(1179, 1149, 1.0, 1350, (-(4.0 * 0.02)));
        }

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && (!s.b[1756])) {
            s.store_sqrt_add_scaled_square_input(1179, 1149, 1.0, 1350, (4.0 * 0.02));
        }

        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {
            s.store_add_scaled_inputs3_indices(1148, 1350, 1.0, 1149, (-0.5), 1179, (-0.5));
        }

        s.b[1757] = (s.v[63] > 0.0);
        s.v[1757] = if s.b[1757] { 1.0 } else { 0.0 };

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1757]) {
            s.store_add(1459, 1350, 781);
            s.store_add_scaled_inputs3_offset_indices(1149, 1459, 1.0, 1458, (-1.0), 1177, 1.0, (-0.02));
        }

        s.b[1758] = (s.v[1459] <= 0.0);
        s.v[1758] = if s.b[1758] { 1.0 } else { 0.0 };

        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1757]) && s.b[1758]) {
            s.store_sqrt_add_scaled_square_input(1179, 1149, 1.0, 1459, (-(100.0 * 0.02)));
        }

        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1757]) && (!s.b[1758])) {
            s.store_sqrt_add_scaled_square_input(1179, 1149, 1.0, 1459, (100.0 * 0.02));
        }

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1757]) {
            s.store_add_scaled_inputs3_indices(1461, 1459, 1.0, 1149, (-0.5), 1179, (-0.5));
        }

        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {
            s.store_div_scaled_inputs3_indices(1179, 1161, 1.0, 1177, (-1.0), 1350, -1.0, 1333, 1.0);
            s.store_mul(1194, 1179, 722);
        }

        s.b[1759] = (((-100.0) < s.v[1194]) && (s.v[1194] < 100.0));
        s.v[1759] = if s.b[1759] { 1.0 } else { 0.0 };

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1759]) {
            s.store_mul_exp_rhs(1334, 721, 1194);
        }

        s.b[1760] = (s.v[1194] <= (-100.0));
        s.v[1760] = if s.b[1760] { 1.0 } else { 0.0 };

        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && (!s.b[1759])) && s.b[1760]) {
            s.store_scale(1334, 721, 3.720075976e-44);
        }

        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && (!s.b[1759])) && (!s.b[1760])) {
            s.store_scale(1334, 721, 2.688117142e43);
        }

        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {
            s.store_scale(1335, 92, 0.001);
            s.store_add_scaled_inputs3_indices(1149, 721, 1.0, 1334, (-1.0), 1335, -1.0);
            s.store_sqrt_add_scaled_square_product(1150, 1149, 1.0, 1335, 721, 4.0);
            s.store_add_scaled_inputs3_indices(1334, 721, 1.0, 1149, (-0.5), 1150, (-0.5));
        }

        s.b[1761] = (s.v[1334] < 1e-15);
        s.v[1761] = if s.b[1761] { 1.0 } else { 0.0 };

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1761]) {
            s.store_scalar(1334, 1e-15);
        }

        s.b[1762] = (s.v[63] > 0.0);
        s.v[1762] = if s.b[1762] { 1.0 } else { 0.0 };

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1762]) {
            s.store_div_scaled_inputs3_indices(1179, 1458, 1.0, 1177, (-1.0), 1459, -1.0, 1333, 1.0);
            s.store_mul(1194, 1179, 722);
        }

        s.b[1763] = (((-100.0) < s.v[1194]) && (s.v[1194] < 100.0));
        s.v[1763] = if s.b[1763] { 1.0 } else { 0.0 };

        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1762]) && s.b[1763]) {
            s.store_mul_exp_rhs(1464, 721, 1194);
        }

        s.b[1764] = (s.v[1194] <= (-100.0));
        s.v[1764] = if s.b[1764] { 1.0 } else { 0.0 };

        if ((((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1762]) && (!s.b[1763])) && s.b[1764]) {
            s.store_scale(1464, 721, 3.720075976e-44);
        }

        if ((((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1762]) && (!s.b[1763])) && (!s.b[1764])) {
            s.store_scale(1464, 721, 2.688117142e43);
        }

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1762]) {
            s.store_add_scaled_inputs3_indices(1149, 721, 1.0, 1464, (-1.0), 1335, -1.0);
            s.store_sqrt_add_scaled_square_product(1150, 1149, 1.0, 1335, 721, 4.0);
            s.store_add_scaled_inputs3_indices(1464, 721, 1.0, 1149, (-0.5), 1150, (-0.5));
        }

        s.b[1765] = (s.v[1464] < 1e-15);
        s.v[1765] = if s.b[1765] { 1.0 } else { 0.0 };

        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1762]) && s.b[1765]) {
            s.store_scalar(1464, 1e-15);
        }

    }

    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
    ) {
        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {
            s.store_div(1336, 778, 1334);
            s.store_div_add_scaled_inputs_rhs_indices(1181, 1332, 1332, 1.0, 1336, 1.0);
            s.store_mul(1337, 1181, 1336);
        }

        s.b[1766] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));
        s.v[1766] = if s.b[1766] { 1.0 } else { 0.0 };

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1766]) {
            s.store_div(1465, 778, 1464);
            s.store_div_add_scaled_inputs_rhs_indices(1181, 1332, 1332, 1.0, 1465, 1.0);
            s.store_mul(1466, 1181, 1465);
        }

        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {
            s.store_div_scaled_product_indices(1317, 1316, 1337, 1.0, 1332, 1.0);
        }

        s.b[1767] = (s.v[63] > 0.0);
        s.v[1767] = if s.b[1767] { 1.0 } else { 0.0 };

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1767]) {
            s.store_div_scaled_product_indices(1468, 1449, 1466, 1.0, 1332, 1.0);
        }

        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {
            s.store_mul_sub_rhs(1273, 1317, 1148, 1350);
        }

        s.b[1768] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));
        s.v[1768] = if s.b[1768] { 1.0 } else { 0.0 };

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1768]) {
            s.store_mul_sub_rhs(1456, 1468, 1461, 1459);
            s.store_add(1273, 1273, 1456);
        }

        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {
            s.store_scale(1179, 737, 0.5);
            s.store_add_scaled_inputs4_indices(1182, 1161, 1.0, 1148, (-1.0), 1177, -1.0, 1210, -1.0);
        }

        s.b[1769] = (s.v[737] == 0.0);
        s.v[1769] = if s.b[1769] { 1.0 } else { 0.0 };

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1769]) {
            s.store_scalar(1180, 0.0);
        }

        s.b[1770] = (s.v[1182] < 0.0);
        s.v[1770] = if s.b[1770] { 1.0 } else { 0.0 };

        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && (!s.b[1769])) && s.b[1770]) {
            s.store_add_div_rhs_indices(1180, 1179, 1182, 737);
        }

        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && (!s.b[1769])) && (!s.b[1770])) {
            s.store_sqrt_square_add(1180, 1179, 1182);
        }

        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {
            s.store_mul_ad_product_rhs(1272, 1317, s.ad_value(737), A::sub(s.ad_value(1180), s.ad_value(1179)));
        }

        s.b[1771] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));
        s.v[1771] = if s.b[1771] { 1.0 } else { 0.0 };

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1771]) {
            s.store_add_scaled_inputs4_indices(1182, 1458, 1.0, 1461, (-1.0), 1177, -1.0, 1451, -1.0);
        }

        s.b[1772] = (s.v[737] == 0.0);
        s.v[1772] = if s.b[1772] { 1.0 } else { 0.0 };

        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1771]) && s.b[1772]) {
            s.store_scalar(1180, 0.0);
        }

        s.b[1773] = (s.v[1182] < 0.0);
        s.v[1773] = if s.b[1773] { 1.0 } else { 0.0 };

        if ((((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1771]) && (!s.b[1772])) && s.b[1773]) {
            s.store_add_div_rhs_indices(1180, 1179, 1182, 737);
        }

        if ((((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1771]) && (!s.b[1772])) && (!s.b[1773])) {
            s.store_sqrt_square_add(1180, 1179, 1182);
        }

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1771]) {
            s.store_mul_ad_product_rhs(1457, 1468, s.ad_value(737), A::sub(s.ad_value(1180), s.ad_value(1179)));
            s.store_add(1272, 1272, 1457);
        }

        s.b[1774] = (s.v[737] <= 0.0);
        s.v[1774] = if s.b[1774] { 1.0 } else { 0.0 };

        if (((!s.b[1733]) && s.b[1751]) && s.b[1774]) {
            s.store_scaled_mul(1271, 723, 1168, 0.25);
            s.store_scale(1179, 700, 0.5);
        }

        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1774])) {
            s.store_mul_product3_rhs(1271, 737, s.ad_value(723), s.ad_value(1168), s.ad_value(737), 1.0);
            s.store_mul(1179, 737, 700);
        }

        if ((!s.b[1733]) && s.b[1751]) {
            s.store_add_scaled_inputs(1180, 1179, 2.0, 1210, 1.0);
        }

        if ((!s.b[1733]) && s.b[1751]) {
            s.store_mul_ad_rhs(1339, 1168, {
                if ((1.0 + ((s.v[1180] * s.v[1210]) / s.v[1271])) > 1e-38) {
                    A::ln(A::offset(A::div_scaled_product(s.ad_value(1180), s.ad_value(1210), 1.0, s.ad_value(1271), 1.0), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        s.b[1775] = (s.v[63] > 0.0);
        s.v[1775] = if s.b[1775] { 1.0 } else { 0.0 };

        if (((!s.b[1733]) && s.b[1751]) && s.b[1775]) {
            s.store_add_scaled_inputs(1180, 1179, 2.0, 1451, 1.0);
        }

        if (((!s.b[1733]) && s.b[1751]) && s.b[1775]) {
            s.store_mul_ad_rhs(1469, 1168, {
                if ((1.0 + ((s.v[1180] * s.v[1451]) / s.v[1271])) > 1e-38) {
                    A::ln(A::offset(A::div_scaled_product(s.ad_value(1180), s.ad_value(1451), 1.0, s.ad_value(1271), 1.0), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((!s.b[1733]) && s.b[1751]) {
            s.store_add_scaled_inputs3_indices(1182, 1165, 4.0, 1350, ((-1.0) * 4.0), 1277, (-4.0));
            s.store_sqrt_square_offset(1181, 1182, 0.0001);
            s.store_scaled_add(1183, 1182, 1181, 0.5);
            s.store_scale(1333, 1333, 2.0);
            s.store_div_scaled_inputs2_indices(1179, 1210, 1.0, 1183, 1.0, 1333, 1.0);
        }

        if ((!s.b[1733]) && s.b[1751]) {
            s.store_exp_scaled_input_ad(1194, {
                if (s.v[1179] > 1e-38) {
                    A::ln(s.ad_value(1179))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (s.v[86] * 0.7));
        }

        if ((!s.b[1733]) && s.b[1751]) {
            s.store_offset(1180, 1194, 1.0);
            s.store_div_from_scalar(1334, (s.v[85] * 1.9e-9), 1180);
            s.store_div(1336, 778, 1334);
            s.store_div_add_scaled_inputs_rhs_indices(1179, 1332, 1332, 1.0, 1336, 1.0);
            s.store_mul(1337, 1179, 1336);
            s.store_div_scaled_product_indices(1338, 1178, 1337, 1.0, 1332, 1.0);
            s.store_div_scaled_product_indices(1317, 1316, 1337, 1.0, 1332, 1.0);
        }

        s.b[1776] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));
        s.v[1776] = if s.b[1776] { 1.0 } else { 0.0 };

        if (((!s.b[1733]) && s.b[1751]) && s.b[1776]) {
            s.store_add_scaled_inputs4_indices(1182, 1165, 4.0, 781, 4.0, 1459, (-4.0), 1277, (-4.0));
            s.store_sqrt_square_offset(1181, 1182, 0.0001);
            s.store_scaled_add(1183, 1182, 1181, 0.5);
            s.store_div_scaled_inputs2_indices(1179, 1451, 1.0, 1183, 1.0, 1333, 1.0);
        }

        if (((!s.b[1733]) && s.b[1751]) && s.b[1776]) {
            s.store_exp_scaled_input_ad(1194, {
                if (s.v[1179] > 1e-38) {
                    A::ln(s.ad_value(1179))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (s.v[86] * 0.7));
        }

        if (((!s.b[1733]) && s.b[1751]) && s.b[1776]) {
            s.store_offset(1180, 1194, 1.0);
            s.store_div_from_scalar(1464, (s.v[85] * 1.9e-9), 1180);
            s.store_div(1465, 778, 1464);
            s.store_div_add_scaled_inputs_rhs_indices(1179, 1332, 1332, 1.0, 1465, 1.0);
            s.store_mul(1466, 1179, 1465);
            s.store_div_scaled_product_indices(1467, 1448, 1466, 1.0, 1332, 1.0);
            s.store_div_scaled_product_indices(1468, 1449, 1466, 1.0, 1332, 1.0);
        }

        if ((!s.b[1733]) && s.b[1751]) {
            s.store_sub(1180, 1210, 1339);
            s.store_scale(1229, 1196, s.v[694]);
            s.store_div(1226, 1180, 1229);
            s.store_offset_sub(1150, 1226, 1158, (-0.02));
            s.store_sqrt_add_scaled_square_input(1179, 1150, 1.0, 1226, (4.0 * 0.02));
            s.store_add_scaled_inputs3_indices(1212, 1226, 1.0, 1150, (-0.5), 1179, (-0.5));
            s.store_mul(1179, 1229, 1212);
            s.store_scaled_offset_ad(1181, A::sub_scaled_inputs(s.ad_value(1180), 1.0, s.ad_value(1179), 0.5), 1e-20, 12.0);
            s.store_div(1182, 1179, 1181);
            s.store_mul_sub_ad_rhs(1250, 1338, s.ad_value(1180), A::mul_sub_from_scalar_rhs(s.ad_value(1179), 0.5, s.ad_value(1182)));
            s.copy_ad(1251, 1250);
        }

        s.b[1777] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));
        s.v[1777] = if s.b[1777] { 1.0 } else { 0.0 };

        if (((!s.b[1733]) && s.b[1751]) && s.b[1777]) {
            s.store_sub(1191, 1451, 1469);
            s.store_div(1462, 1191, 1229);
            s.store_offset_sub(1150, 1462, 1158, (-0.02));
            s.store_sqrt_add_scaled_square_input(1454, 1150, 1.0, 1462, (4.0 * 0.02));
            s.store_add_scaled_inputs3_indices(1463, 1462, 1.0, 1150, (-0.5), 1454, (-0.5));
            s.store_mul(1454, 1229, 1463);
            s.store_scaled_offset_ad(1455, A::sub_scaled_inputs(s.ad_value(1191), 1.0, s.ad_value(1454), 0.5), 1e-20, 12.0);
            s.store_div(1182, 1454, 1455);
            s.store_mul_sub_ad_rhs(1186, 1467, s.ad_value(1191), A::mul_sub_from_scalar_rhs(s.ad_value(1454), 0.5, s.ad_value(1182)));
            s.store_add(1250, 1250, 1186);
            s.copy_ad(1251, 1250);
        }

        s.b[1778] = (s.v[57] == 2.0);
        s.v[1778] = if s.b[1778] { 1.0 } else { 0.0 };

        if (((!s.b[1733]) && s.b[1751]) && s.b[1778]) {
            s.store_scalar(1341, 0.0);
        }

        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1778])) {
            s.store_sub_from_scalar(1186, 1.0, 1229);
            s.store_mul_ad_product_rhs(1341, 1317, s.ad_value(1186), A::sub_scaled_inputs(s.ad_value(1212), 0.5, A::div_scaled_product(s.ad_value(1179), s.ad_value(1212), 1.0, s.ad_value(1181), 1.0), 1.0));
        }

        s.b[1779] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));
        s.v[1779] = if s.b[1779] { 1.0 } else { 0.0 };

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1778])) && s.b[1779]) {
            s.store_mul_ad_product_rhs(1471, 1468, s.ad_value(1186), A::sub_scaled_inputs(s.ad_value(1463), 0.5, A::div_scaled_product(s.ad_value(1454), s.ad_value(1463), 1.0, s.ad_value(1455), 1.0), 1.0));
            s.store_add(1341, 1341, 1471);
        }

        s.b[1780] = (s.v[153] > 0.5);
        s.v[1780] = if s.b[1780] { 1.0 } else { 0.0 };

        if (((!s.b[1733]) && s.b[1751]) && s.b[1780]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(1254, 1338, s.ad_value(1180), ((0.5) * (-1.0)), s.ad_value(1179), ((0.25) * (-1.0)), A::div_scaled_product(s.ad_value(1179), s.ad_value(1179), 0.5, s.ad_value(1181), 1.0), ((-1.0) * (-1.0)), 0.0);
        }

        s.b[1781] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));
        s.v[1781] = if s.b[1781] { 1.0 } else { 0.0 };

        if ((((!s.b[1733]) && s.b[1751]) && s.b[1780]) && s.b[1781]) {
            s.store_mul_add_scaled_inputs4_rhs(1470, 1467, s.ad_value(1451), ((0.5) * (-1.0)), s.ad_value(1469), (((-0.5)) * (-1.0)), s.ad_value(1454), ((0.25) * (-1.0)), A::div_scaled_product(s.ad_value(1454), s.ad_value(1454), 0.5, s.ad_value(1455), 1.0), ((-1.0) * (-1.0)));
            s.store_add(1254, 1254, 1470);
        }

        s.b[1782] = (s.v[153] < 0.5);
        s.v[1782] = if s.b[1782] { 1.0 } else { 0.0 };

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1780])) && s.b[1782]) {
            s.store_scale(1181, 1181, 0.08333333333333333);
            s.store_div_scaled_inputs_square_rhs(1182, 1338, 0.5, 1181, 1.0);
            s.store_add_scaled_product_mixed_aia(1183, A::mul3_scaled_output(s.ad_value(1179), s.ad_value(1179), s.ad_value(1179), (2.0 * 0.06666666666666667)), (-1.0), 1180, A::add_scaled_products(s.ad_value(1179), s.ad_value(1179), (2.0 * 0.3333333333333333), s.ad_value(1180), A::sub_scaled_inputs(s.ad_value(1180), 1.0, s.ad_value(1179), (4.0 * 0.3333333333333333)), 1.0), 1.0);
            s.store_mul_neg_lhs(1254, 1182, 1183);
        }

        s.b[1783] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));
        s.v[1783] = if s.b[1783] { 1.0 } else { 0.0 };

        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1780])) && s.b[1782]) && s.b[1783]) {
            s.store_scale(1455, 1455, 0.08333333333333333);
            s.store_div_scaled_inputs_square_rhs(1182, 1467, 0.5, 1455, 1.0);
            s.store_add_scaled_product_mixed_aia(1183, A::mul3_scaled_output(s.ad_value(1454), s.ad_value(1454), s.ad_value(1454), (2.0 * 0.06666666666666667)), (-1.0), 1191, A::add_scaled_products(s.ad_value(1454), s.ad_value(1454), (2.0 * 0.3333333333333333), s.ad_value(1191), A::sub_scaled_inputs(s.ad_value(1191), 1.0, s.ad_value(1454), (4.0 * 0.3333333333333333)), 1.0), 1.0);
            s.store_mul_neg_lhs(1470, 1182, 1183);
            s.store_add(1254, 1254, 1470);
        }

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1780])) && (!s.b[1782])) {
            s.store_scale(1254, 1251, (-0.5));
        }

        s.b[1784] = (s.v[57] == 2.0);
        s.v[1784] = if s.b[1784] { 1.0 } else { 0.0 };

        if (((!s.b[1733]) && s.b[1751]) && s.b[1784]) {
            s.store_scalar(1274, 0.0);
        }

        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1784])) {
            s.store_scale(1249, 626, (s.v[342] * (s.v[1248] * ((((s.v[693] / s.v[59]) * s.v[39]) * s.v[727]) + s.v[65]))));
            s.store_mul_sub_rhs(1274, 1249, 1237, 1160);
        }

        if ((!s.b[1733]) && s.b[1751]) {
            s.store_add_scaled_inputs4_indices(1251, 1251, 1.0, 1273, 1.0, 1272, 1.0, 1341, -1.0);
            s.store_add_scaled_inputs4_indices(1252, 1341, 1.0, 1273, (-1.0), 1272, -1.0, 1274, -1.0);
            s.copy_ad(1255, 1274);
            s.store_add_scaled_inputs4_indices(1253, 1251, (-1.0), 1252, (-1.0), 1255, (-1.0), 1254, (-1.0));
        }

        if ((!s.b[1733]) && (!s.b[1751])) {
            s.store_scalar(1273, 0.0);
            s.store_scalar(1272, 0.0);
            s.store_scalar(1255, 0.0);
            s.store_scalar(1252, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[1733]) && (!s.b[1751])) {
            s.store_scalar(1254, 0.0);
            s.store_scalar(1253, 0.0);
            s.store_scalar(1251, 0.0);
        }

        s.b[1785] = (s.v[57] == 2.0);
        s.v[1785] = if s.b[1785] { 1.0 } else { 0.0 };

        if s.b[1785] {
            s.store_scalar(1244, 0.0);
            s.store_scalar(1245, 0.0);
        }

        if (!s.b[1785]) {
            s.copy_ad(1151, 200);
            s.store_scalar(1315, (-s.v[344]));
            s.store_add_scaled_offset_product_rhs(1151, 1151, 1.0, 1315, 769, (-s.v[150]), 1.0);
            s.copy_ad(1152, 202);
            s.store_scalar(1311, ((((s.v[204] * s.v[711]) * s.v[174]) * s.v[39]) / 1e-7));
            s.store_scale(1314, 1311, s.v[343]);
            s.store_add_scaled_offset_product_rhs(1311, 1311, 1.0, 1314, 769, (-s.v[150]), 1.0);
            s.store_scalar(1312, ((((s.v[205] * s.v[710]) * s.v[174]) * s.v[39]) / 1e-7));
            s.store_scale(1313, 1312, s.v[345]);
            s.store_add_scaled_offset_product_rhs(1312, 1312, 1.0, 1313, 769, (-s.v[150]), 1.0);
            s.store_scale(1329, 1151, 0.9);
        }

        if (!s.b[1785]) {
            s.store_sub_from_scalar_div_mixed_ai(1147, 1.0, {
                if (s.v[1421] > s.v[1329]) {
                    s.ad_value(1329)
                } else {
                    s.ad_value(1421)
                }
            }, 1151);
        }

        s.b[1786] = (p.p173 == 0.5);
        s.v[1786] = if s.b[1786] { 1.0 } else { 0.0 };

        if ((!s.b[1785]) && s.b[1786]) {
            s.store_div_from_scalar_sqrt_ad(1193, 1.0, s.ad_value(1147));
        }

        if ((!s.b[1785]) && (!s.b[1786])) {
            s.store_exp_scaled_input_ad(1193, {
                if (s.v[1147] > 1e-38) {
                    A::ln(s.ad_value(1147))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (-p.p173));
        }

        if (!s.b[1785]) {
            s.store_mul_sub_from_scalar_lhs_scaled_ad_lhs(1182, 1.0, A::mul(s.ad_value(1147), s.ad_value(1193)), 1151, 1.0 / ((1.0 - p.p173)));
        }

        s.b[1787] = (s.v[1421] > s.v[1329]);
        s.v[1787] = if s.b[1787] { 1.0 } else { 0.0 };

        if ((!s.b[1785]) && s.b[1787]) {
            s.store_add_scaled_product_right_sub(1182, 1182, 1.0, 1193, 1421, 1329, 1.0);
        }

        if (!s.b[1785]) {
            s.store_add_scaled_product_indices(1245, 1322, (s.v[332] * s.v[39]), 1311, 1182, 1.0);
            s.copy_ad(1151, 201);
            s.store_scalar(1315, (-s.v[346]));
            s.store_add_scaled_offset_product_rhs(1151, 1151, 1.0, 1315, 769, (-s.v[150]), 1.0);
            s.store_scalar(1152, s.v[203]);
            s.store_scale(1329, 1151, 0.9);
        }

        if (!s.b[1785]) {
            s.store_sub_from_scalar_div_mixed_ai(1147, 1.0, {
                if (s.v[1422] > s.v[1329]) {
                    s.ad_value(1329)
                } else {
                    s.ad_value(1422)
                }
            }, 1151);
        }

        s.b[1788] = (p.p173 == 0.5);
        s.v[1788] = if s.b[1788] { 1.0 } else { 0.0 };

        if ((!s.b[1785]) && s.b[1788]) {
            s.store_div_from_scalar_sqrt_ad(1193, 1.0, s.ad_value(1147));
        }

        if ((!s.b[1785]) && (!s.b[1788])) {
            s.store_exp_scaled_input_ad(1193, {
                if (s.v[1147] > 1e-38) {
                    A::ln(s.ad_value(1147))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (-p.p173));
        }

        if (!s.b[1785]) {
            s.store_mul_sub_from_scalar_lhs_scaled_ad_lhs(1182, 1.0, A::mul(s.ad_value(1147), s.ad_value(1193)), 1151, 1.0 / ((1.0 - p.p173)));
        }

        s.b[1789] = (s.v[1422] > s.v[1329]);
        s.v[1789] = if s.b[1789] { 1.0 } else { 0.0 };

        if ((!s.b[1785]) && s.b[1789]) {
            s.store_add_scaled_product_right_sub(1182, 1182, 1.0, 1193, 1422, 1329, 1.0);
        }

        if (!s.b[1785]) {
            s.store_add_scaled_product_indices(1244, 1323, (s.v[332] * s.v[39]), 1312, 1182, 1.0);
        }

        s.store_scale(1189, 1232, (-s.v[36]));

        s.store_scaled_sub(1190, 1155, 1232, s.v[36]);

        s.b[1790] = (s.v[336] != 0.0);
        s.v[1790] = if s.b[1790] { 1.0 } else { 0.0 };

        s.b[1791] = (((s.v[479] > 0.0) && (s.v[36] > 0.0)) || ((s.v[479] < 0.0) && (s.v[36] < 0.0)));
        s.v[1791] = if s.b[1791] { 1.0 } else { 0.0 };

        s.b[1792] = (s.v[1189] < s.v[683]);
        s.v[1792] = if s.b[1792] { 1.0 } else { 0.0 };

        if ((s.b[1790] && s.b[1791]) && s.b[1792]) {
            s.store_scaled_sub(448, 1189, 683, s.v[430]);
        }

        s.b[1793] = (s.v[1189] < s.v[545]);
        s.v[1793] = if s.b[1793] { 1.0 } else { 0.0 };

        if (((s.b[1790] && s.b[1791]) && (!s.b[1792])) && s.b[1793]) {
            s.store_sub(1179, 1189, 683);
            s.store_square(1180, 1179);
            s.store_mul_sub_from_scalar_ad_rhs(448, 1179, s.v[430], A::mul_scaled_lhs(s.ad_value(546), 1.0 / (3.0), s.ad_value(1180)));
        }

        s.b[1794] = (s.v[1189] < s.v[684]);
        s.v[1794] = if s.b[1794] { 1.0 } else { 0.0 };

        if ((((s.b[1790] && s.b[1791]) && (!s.b[1792])) && (!s.b[1793])) && s.b[1794]) {
            s.store_sub(1179, 1189, 684);
            s.store_square(1180, 1179);
            s.store_add_ad(448, A::add_scaled_product(s.ad_value(434), 1.0, s.ad_value(432), s.ad_value(1189), 1.0), A::mul3_scaled_output(s.ad_value(547), s.ad_value(1179), s.ad_value(1180), 1.0 / (3.0)));
        }

        if ((((s.b[1790] && s.b[1791]) && (!s.b[1792])) && (!s.b[1793])) && (!s.b[1794])) {
            s.store_add_scaled_product_indices(448, 434, 1.0, 432, 1189, 1.0);
        }

        s.b[1795] = (s.v[1189] < s.v[684]);
        s.v[1795] = if s.b[1795] { 1.0 } else { 0.0 };

        if ((s.b[1790] && (!s.b[1791])) && s.b[1795]) {
            s.store_mul_sub_rhs(448, 432, 1189, 684);
        }

        s.b[1796] = (s.v[1189] < s.v[545]);
        s.v[1796] = if s.b[1796] { 1.0 } else { 0.0 };

        if (((s.b[1790] && (!s.b[1791])) && (!s.b[1795])) && s.b[1796]) {
            s.store_sub(1179, 1189, 684);
            s.store_square(1180, 1179);
            s.store_mul_add_scaled_product_rhs(448, 1179, s.ad_value(432), 1.0, s.ad_value(546), s.ad_value(1180), (-1.0 / (3.0)));
        }

        s.b[1797] = (s.v[1189] < s.v[683]);
        s.v[1797] = if s.b[1797] { 1.0 } else { 0.0 };

        if ((((s.b[1790] && (!s.b[1791])) && (!s.b[1795])) && (!s.b[1796])) && s.b[1797]) {
            s.store_sub(1179, 1189, 683);
            s.store_square(1180, 1179);
            s.store_add_scaled_inputs3_mixed_iia(448, 1189, s.v[430], 434, 1.0, A::mul3_scaled_output(s.ad_value(547), s.ad_value(1179), s.ad_value(1180), 1.0 / (3.0)), 1.0);
        }

        if ((((s.b[1790] && (!s.b[1791])) && (!s.b[1795])) && (!s.b[1796])) && (!s.b[1797])) {
            s.store_add_scaled_inputs(448, 1189, s.v[430], 434, 1.0);
        }

        s.b[1798] = (((s.v[479] > 0.0) && (s.v[36] > 0.0)) || ((s.v[479] < 0.0) && (s.v[36] < 0.0)));
        s.v[1798] = if s.b[1798] { 1.0 } else { 0.0 };

        s.b[1799] = (s.v[1190] < s.v[683]);
        s.v[1799] = if s.b[1799] { 1.0 } else { 0.0 };

        if ((s.b[1790] && s.b[1798]) && s.b[1799]) {
            s.store_scaled_sub(449, 1190, 683, s.v[431]);
        }

        s.b[1800] = (s.v[1190] < s.v[545]);
        s.v[1800] = if s.b[1800] { 1.0 } else { 0.0 };

        if (((s.b[1790] && s.b[1798]) && (!s.b[1799])) && s.b[1800]) {
            s.store_sub(1179, 1190, 683);
            s.store_square(1180, 1179);
            s.store_mul_sub_from_scalar_ad_rhs(449, 1179, s.v[431], A::mul_scaled_lhs(s.ad_value(548), 1.0 / (3.0), s.ad_value(1180)));
        }

        s.b[1801] = (s.v[1190] < s.v[684]);
        s.v[1801] = if s.b[1801] { 1.0 } else { 0.0 };

        if ((((s.b[1790] && s.b[1798]) && (!s.b[1799])) && (!s.b[1800])) && s.b[1801]) {
            s.store_sub(1179, 1190, 684);
            s.store_square(1180, 1179);
            s.store_add_ad(449, A::add_scaled_product(s.ad_value(435), 1.0, s.ad_value(433), s.ad_value(1190), 1.0), A::mul3_scaled_output(s.ad_value(549), s.ad_value(1179), s.ad_value(1180), 1.0 / (3.0)));
        }

        if ((((s.b[1790] && s.b[1798]) && (!s.b[1799])) && (!s.b[1800])) && (!s.b[1801])) {
            s.store_add_scaled_product_indices(449, 435, 1.0, 433, 1190, 1.0);
        }

        s.b[1802] = (s.v[1190] < s.v[684]);
        s.v[1802] = if s.b[1802] { 1.0 } else { 0.0 };

        if ((s.b[1790] && (!s.b[1798])) && s.b[1802]) {
            s.store_mul_sub_rhs(449, 433, 1190, 684);
        }

        s.b[1803] = (s.v[1190] < s.v[545]);
        s.v[1803] = if s.b[1803] { 1.0 } else { 0.0 };

        if (((s.b[1790] && (!s.b[1798])) && (!s.b[1802])) && s.b[1803]) {
            s.store_sub(1179, 1190, 684);
            s.store_square(1180, 1179);
            s.store_mul_add_scaled_product_rhs(449, 1179, s.ad_value(433), 1.0, s.ad_value(548), s.ad_value(1180), (-1.0 / (3.0)));
        }

        s.b[1804] = (s.v[1190] < s.v[683]);
        s.v[1804] = if s.b[1804] { 1.0 } else { 0.0 };

        if ((((s.b[1790] && (!s.b[1798])) && (!s.b[1802])) && (!s.b[1803])) && s.b[1804]) {
            s.store_sub(1179, 1190, 683);
            s.store_square(1180, 1179);
            s.store_add_scaled_inputs3_mixed_iia(449, 1190, s.v[431], 435, 1.0, A::mul3_scaled_output(s.ad_value(549), s.ad_value(1179), s.ad_value(1180), 1.0 / (3.0)), 1.0);
        }

        if ((((s.b[1790] && (!s.b[1798])) && (!s.b[1802])) && (!s.b[1803])) && (!s.b[1804])) {
            s.store_add_scaled_inputs(449, 1190, s.v[431], 435, 1.0);
        }

        if (!s.b[1790]) {
            s.store_scale(448, 1189, s.v[430]);
            s.store_scale(449, 1190, s.v[431]);
        }

        s.store_add_scaled_product_indices(448, 448, 1.0, 428, 1189, 1.0);

        s.store_add_scaled_product_indices(449, 449, 1.0, 429, 1190, 1.0);

        s.b[1805] = (s.v[66] == 3.0);
        s.v[1805] = if s.b[1805] { 1.0 } else { 0.0 };

        if s.b[1805] {
            s.store_offset(1179, 1354, 0.02);
        }

        if (!s.b[1805]) {
            s.store_offset(1179, 1156, 0.02);
        }

        s.store_sqrt_square_offset(1180, 1179, (4.0 * 0.02));

        s.store_scaled_sub(1181, 1179, 1180, 0.5);

        s.store_scale(1182, 603, s.v[710]);

        s.store_sqrt_sub_from_scalar_ad(1183, 1.0, A::div_scaled_inputs(s.ad_value(1181), 4.0, s.ad_value(604), 1.0));

        s.b[1806] = (s.v[66] == 3.0);
        s.v[1806] = if s.b[1806] { 1.0 } else { 0.0 };

        if s.b[1806] {
            s.store_add_scaled_products_mixed_aiia(1230, A::add(s.ad_value(696), s.ad_value(1182)), 1354, 1.0, 1182, A::add_scaled_offset_product_rhs(s.ad_value(1181), 1.0, s.ad_value(604), s.ad_value(1183), (-1.0), 0.5), (-1.0));
        }

        if (!s.b[1806]) {
            s.store_add_scaled_products_mixed_aiia(1230, A::add(s.ad_value(696), s.ad_value(1182)), 1156, 1.0, 1182, A::add_scaled_offset_product_rhs(s.ad_value(1181), 1.0, s.ad_value(604), s.ad_value(1183), (-1.0), 0.5), (-1.0));
        }

        s.b[1807] = (s.v[66] == 3.0);
        s.v[1807] = if s.b[1807] { 1.0 } else { 0.0 };

        if s.b[1807] {
            s.store_offset(1179, 1353, 0.02);
        }

        if (!s.b[1807]) {
            s.store_offset(1179, 1157, 0.02);
        }

        s.store_sqrt_square_offset(1180, 1179, (4.0 * 0.02));

        s.store_scaled_sub(1181, 1179, 1180, 0.5);

        s.store_scale(1182, 602, s.v[711]);

        s.store_sqrt_sub_from_scalar_ad(1183, 1.0, A::div_scaled_inputs(s.ad_value(1181), 4.0, s.ad_value(604), 1.0));

        s.b[1808] = (s.v[66] == 3.0);
        s.v[1808] = if s.b[1808] { 1.0 } else { 0.0 };

        if s.b[1808] {
            s.store_add_scaled_products_mixed_aiia(1231, A::add(s.ad_value(695), s.ad_value(1182)), 1353, 1.0, 1182, A::add_scaled_offset_product_rhs(s.ad_value(1181), 1.0, s.ad_value(604), s.ad_value(1183), (-1.0), 0.5), (-1.0));
        }

        if (!s.b[1808]) {
            s.store_add_scaled_products_mixed_aiia(1231, A::add(s.ad_value(695), s.ad_value(1182)), 1157, 1.0, 1182, A::add_scaled_offset_product_rhs(s.ad_value(1181), 1.0, s.ad_value(604), s.ad_value(1183), (-1.0), 0.5), (-1.0));
        }

        s.b[1809] = (s.v[39] != 1.0);
        s.v[1809] = if s.b[1809] { 1.0 } else { 0.0 };

        if s.b[1809] {
            s.store_scale(1230, 1230, s.v[39]);
            s.store_scale(1231, 1231, s.v[39]);
        }

        s.copy_ad(798, 1251);

        s.store_add(797, 1231, 1230);

        s.store_add(1251, 798, 797);

        s.b[1823] = (p.p213 == 0.0);
        s.v[1823] = if s.b[1823] { 1.0 } else { 0.0 };

        s.b[1824] = (p.p213 == 1.0);
        s.v[1824] = if s.b[1824] { 1.0 } else { 0.0 };

        if (s.b[1824] && (!s.b[1823])) {
            s.store_add_scaled_inputs3_indices(1179, 439, 1.0, 440, 1.0, 441, 1.0);
            s.store_square(1179, 1179);
            s.store_div_scaled_inputs_indices(1817, 1281, 2.0, 410, 1.0);
            s.store_div_scaled_inputs_indices(1184, 451, 1.0, 1817, s.v[688]);
            s.store_square(1184, 1184);
            s.store_offset_scaled(1818, 1184, (((s.v[241] * s.v[688])) * (s.v[243])), s.v[243]);
            s.store_add_scaled_product_right_ad(1180, 440, 1.0, 1818, A::add(s.ad_value(439), s.ad_value(441)), 1.0);
            s.store_div_scaled_product_indices(1181, 1180, 1180, 1.0, 454, 1.0);
        }

        s.b[1861] = (s.v[759] > 0.0);
        s.v[1861] = if s.b[1861] { 1.0 } else { 0.0 };

        if s.b[1861] {
            s.store_scale(446, 1253, s.v[36]);
            s.store_scale(447, 1254, s.v[36]);
        }

        if (!s.b[1861]) {
            s.store_scale(447, 1253, s.v[36]);
            s.store_scale(446, 1254, s.v[36]);
        }

        s.b[1863] = (p.p37 == 3.0);
        s.v[1863] = if s.b[1863] { 1.0 } else { 0.0 };

        s.b[1869] = ((p.p33 == 1.0) && (p.p16 != 0.0));
        s.v[1869] = if s.b[1869] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let __rspice_deriv_cse_0: f64 = (s.v[36] * s.dn[1243][0]);
        let __rspice_deriv_cse_1: f64 = (s.v[36] * s.dn[1243][1]);
        let __rspice_deriv_cse_2: f64 = (s.v[36] * s.dn[1243][2]);
        let __rspice_deriv_cse_3: f64 = (s.v[36] * s.dn[1243][3]);
        let __rspice_deriv_cse_4: f64 = (s.v[36] * s.dn[1243][4]);
        let __rspice_deriv_cse_5: f64 = (s.v[36] * s.dn[1243][5]);
        let __rspice_deriv_cse_6: f64 = (s.v[36] * s.dn[1243][6]);
        let __rspice_deriv_cse_7: f64 = (s.v[36] * s.dn[1243][7]);
        let __rspice_deriv_cse_8: f64 = (s.v[36] * s.dn[1243][8]);
        let __rspice_deriv_cse_9: f64 = (s.v[36] * s.dn[1243][9]);
        let __rspice_deriv_cse_10: f64 = (s.v[36] * s.dn[1243][10]);
        let __rspice_deriv_cse_11: f64 = (s.v[36] * s.dn[1243][11]);
        let __rspice_deriv_cse_12: f64 = (s.v[36] * s.dn[1243][12]);
        let __rspice_deriv_cse_13: f64 = (s.v[36] * s.db[1243][0]);
        let __rspice_deriv_cse_14: f64 = (s.v[36] * s.db[1243][1]);
        let __rspice_deriv_cse_15: f64 = (s.v[36] * s.db[1243][2]);
        let __rspice_deriv_cse_16: f64 = (s.v[36] * s.db[1243][3]);
        let __rspice_deriv_cse_17: f64 = (s.v[36] * s.db[1243][4]);
        let __rspice_deriv_cse_18: f64 = (s.v[36] * s.db[1243][5]);
        let __rspice_deriv_cse_19: f64 = (s.v[36] * s.db[1243][6]);
        let __rspice_deriv_cse_20: f64 = (s.v[36] * s.db[1243][7]);
        let __rspice_deriv_cse_21: f64 = (s.v[36] * s.db[1243][8]);
        let (eq4_e1143, eq4_e1143_d_n0, eq4_e1143_d_n1, eq4_e1143_d_n2, eq4_e1143_d_n3, eq4_e1143_d_n4, eq4_e1143_d_n5, eq4_e1143_d_n6, eq4_e1143_d_n7, eq4_e1143_d_n8, eq4_e1143_d_n9, eq4_e1143_d_n10, eq4_e1143_d_n11, eq4_e1143_d_n12, eq4_e1143_d_b0, eq4_e1143_d_b1, eq4_e1143_d_b2, eq4_e1143_d_b3, eq4_e1143_d_b4, eq4_e1143_d_b5, eq4_e1143_d_b6, eq4_e1143_d_b7, eq4_e1143_d_b8,) = {
    if s.b[1860] {
        let eq4_e1141: f64 = ((nv0 - nv7) / s.v[1433]);
        let eq4_e1141_d_n0: f64 = ((s.v[1433] - ((nv0 - nv7) * s.dn[1433][0])) / (s.v[1433] * s.v[1433]));
        let eq4_e1141_d_n1: f64 = (-(((nv0 - nv7) * s.dn[1433][1]) / (s.v[1433] * s.v[1433])));
        let eq4_e1141_d_n2: f64 = (-(((nv0 - nv7) * s.dn[1433][2]) / (s.v[1433] * s.v[1433])));
        let eq4_e1141_d_n3: f64 = (-(((nv0 - nv7) * s.dn[1433][3]) / (s.v[1433] * s.v[1433])));
        let eq4_e1141_d_n4: f64 = (-(((nv0 - nv7) * s.dn[1433][4]) / (s.v[1433] * s.v[1433])));
        let eq4_e1141_d_n5: f64 = (-(((nv0 - nv7) * s.dn[1433][5]) / (s.v[1433] * s.v[1433])));
        let eq4_e1141_d_n6: f64 = (-(((nv0 - nv7) * s.dn[1433][6]) / (s.v[1433] * s.v[1433])));
        let eq4_e1141_d_n7: f64 = (((-s.v[1433]) - ((nv0 - nv7) * s.dn[1433][7])) / (s.v[1433] * s.v[1433]));
        let eq4_e1141_d_n8: f64 = (-(((nv0 - nv7) * s.dn[1433][8]) / (s.v[1433] * s.v[1433])));
        let eq4_e1141_d_n9: f64 = (-(((nv0 - nv7) * s.dn[1433][9]) / (s.v[1433] * s.v[1433])));
        let eq4_e1141_d_n10: f64 = (-(((nv0 - nv7) * s.dn[1433][10]) / (s.v[1433] * s.v[1433])));
        let eq4_e1141_d_n11: f64 = (-(((nv0 - nv7) * s.dn[1433][11]) / (s.v[1433] * s.v[1433])));
        let eq4_e1141_d_n12: f64 = (-(((nv0 - nv7) * s.dn[1433][12]) / (s.v[1433] * s.v[1433])));
        let eq4_e1141_d_b0: f64 = (-(((nv0 - nv7) * s.db[1433][0]) / (s.v[1433] * s.v[1433])));
        let eq4_e1141_d_b1: f64 = (-(((nv0 - nv7) * s.db[1433][1]) / (s.v[1433] * s.v[1433])));
        let eq4_e1141_d_b2: f64 = (-(((nv0 - nv7) * s.db[1433][2]) / (s.v[1433] * s.v[1433])));
        let eq4_e1141_d_b3: f64 = (-(((nv0 - nv7) * s.db[1433][3]) / (s.v[1433] * s.v[1433])));
        let eq4_e1141_d_b4: f64 = (-(((nv0 - nv7) * s.db[1433][4]) / (s.v[1433] * s.v[1433])));
        let eq4_e1141_d_b5: f64 = (-(((nv0 - nv7) * s.db[1433][5]) / (s.v[1433] * s.v[1433])));
        let eq4_e1141_d_b6: f64 = (-(((nv0 - nv7) * s.db[1433][6]) / (s.v[1433] * s.v[1433])));
        let eq4_e1141_d_b7: f64 = (-(((nv0 - nv7) * s.db[1433][7]) / (s.v[1433] * s.v[1433])));
        let eq4_e1141_d_b8: f64 = (-(((nv0 - nv7) * s.db[1433][8]) / (s.v[1433] * s.v[1433])));
        (eq4_e1141, eq4_e1141_d_n0, eq4_e1141_d_n1, eq4_e1141_d_n2, eq4_e1141_d_n3, eq4_e1141_d_n4, eq4_e1141_d_n5, eq4_e1141_d_n6, eq4_e1141_d_n7, eq4_e1141_d_n8, eq4_e1141_d_n9, eq4_e1141_d_n10, eq4_e1141_d_n11, eq4_e1141_d_n12, eq4_e1141_d_b0, eq4_e1141_d_b1, eq4_e1141_d_b2, eq4_e1141_d_b3, eq4_e1141_d_b4, eq4_e1141_d_b5, eq4_e1141_d_b6, eq4_e1141_d_b7, eq4_e1141_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e1143;
        let eq4_node_derivatives: [f64; 13] = [eq4_e1143_d_n0, eq4_e1143_d_n1, eq4_e1143_d_n2, eq4_e1143_d_n3, eq4_e1143_d_n4, eq4_e1143_d_n5, eq4_e1143_d_n6, eq4_e1143_d_n7, eq4_e1143_d_n8, eq4_e1143_d_n9, eq4_e1143_d_n10, eq4_e1143_d_n11, eq4_e1143_d_n12];
        let eq4_branch_derivatives: [f64; 9] = [eq4_e1143_d_b0, eq4_e1143_d_b1, eq4_e1143_d_b2, eq4_e1143_d_b3, eq4_e1143_d_b4, eq4_e1143_d_b5, eq4_e1143_d_b6, eq4_e1143_d_b7, eq4_e1143_d_b8];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(7),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq6_e1158, eq6_e1158_d_n0, eq6_e1158_d_n1, eq6_e1158_d_n2, eq6_e1158_d_n3, eq6_e1158_d_n4, eq6_e1158_d_n5, eq6_e1158_d_n6, eq6_e1158_d_n7, eq6_e1158_d_n8, eq6_e1158_d_n9, eq6_e1158_d_n10, eq6_e1158_d_n11, eq6_e1158_d_n12, eq6_e1158_d_b0, eq6_e1158_d_b1, eq6_e1158_d_b2, eq6_e1158_d_b3, eq6_e1158_d_b4, eq6_e1158_d_b5, eq6_e1158_d_b6, eq6_e1158_d_b7, eq6_e1158_d_b8,) = {
    if s.b[1860] {
        let eq6_e1156: f64 = ((nv2 - nv8) / s.v[1434]);
        let eq6_e1156_d_n0: f64 = (-(((nv2 - nv8) * s.dn[1434][0]) / (s.v[1434] * s.v[1434])));
        let eq6_e1156_d_n1: f64 = (-(((nv2 - nv8) * s.dn[1434][1]) / (s.v[1434] * s.v[1434])));
        let eq6_e1156_d_n2: f64 = ((s.v[1434] - ((nv2 - nv8) * s.dn[1434][2])) / (s.v[1434] * s.v[1434]));
        let eq6_e1156_d_n3: f64 = (-(((nv2 - nv8) * s.dn[1434][3]) / (s.v[1434] * s.v[1434])));
        let eq6_e1156_d_n4: f64 = (-(((nv2 - nv8) * s.dn[1434][4]) / (s.v[1434] * s.v[1434])));
        let eq6_e1156_d_n5: f64 = (-(((nv2 - nv8) * s.dn[1434][5]) / (s.v[1434] * s.v[1434])));
        let eq6_e1156_d_n6: f64 = (-(((nv2 - nv8) * s.dn[1434][6]) / (s.v[1434] * s.v[1434])));
        let eq6_e1156_d_n7: f64 = (-(((nv2 - nv8) * s.dn[1434][7]) / (s.v[1434] * s.v[1434])));
        let eq6_e1156_d_n8: f64 = (((-s.v[1434]) - ((nv2 - nv8) * s.dn[1434][8])) / (s.v[1434] * s.v[1434]));
        let eq6_e1156_d_n9: f64 = (-(((nv2 - nv8) * s.dn[1434][9]) / (s.v[1434] * s.v[1434])));
        let eq6_e1156_d_n10: f64 = (-(((nv2 - nv8) * s.dn[1434][10]) / (s.v[1434] * s.v[1434])));
        let eq6_e1156_d_n11: f64 = (-(((nv2 - nv8) * s.dn[1434][11]) / (s.v[1434] * s.v[1434])));
        let eq6_e1156_d_n12: f64 = (-(((nv2 - nv8) * s.dn[1434][12]) / (s.v[1434] * s.v[1434])));
        let eq6_e1156_d_b0: f64 = (-(((nv2 - nv8) * s.db[1434][0]) / (s.v[1434] * s.v[1434])));
        let eq6_e1156_d_b1: f64 = (-(((nv2 - nv8) * s.db[1434][1]) / (s.v[1434] * s.v[1434])));
        let eq6_e1156_d_b2: f64 = (-(((nv2 - nv8) * s.db[1434][2]) / (s.v[1434] * s.v[1434])));
        let eq6_e1156_d_b3: f64 = (-(((nv2 - nv8) * s.db[1434][3]) / (s.v[1434] * s.v[1434])));
        let eq6_e1156_d_b4: f64 = (-(((nv2 - nv8) * s.db[1434][4]) / (s.v[1434] * s.v[1434])));
        let eq6_e1156_d_b5: f64 = (-(((nv2 - nv8) * s.db[1434][5]) / (s.v[1434] * s.v[1434])));
        let eq6_e1156_d_b6: f64 = (-(((nv2 - nv8) * s.db[1434][6]) / (s.v[1434] * s.v[1434])));
        let eq6_e1156_d_b7: f64 = (-(((nv2 - nv8) * s.db[1434][7]) / (s.v[1434] * s.v[1434])));
        let eq6_e1156_d_b8: f64 = (-(((nv2 - nv8) * s.db[1434][8]) / (s.v[1434] * s.v[1434])));
        (eq6_e1156, eq6_e1156_d_n0, eq6_e1156_d_n1, eq6_e1156_d_n2, eq6_e1156_d_n3, eq6_e1156_d_n4, eq6_e1156_d_n5, eq6_e1156_d_n6, eq6_e1156_d_n7, eq6_e1156_d_n8, eq6_e1156_d_n9, eq6_e1156_d_n10, eq6_e1156_d_n11, eq6_e1156_d_n12, eq6_e1156_d_b0, eq6_e1156_d_b1, eq6_e1156_d_b2, eq6_e1156_d_b3, eq6_e1156_d_b4, eq6_e1156_d_b5, eq6_e1156_d_b6, eq6_e1156_d_b7, eq6_e1156_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e1158;
        let eq6_node_derivatives: [f64; 13] = [eq6_e1158_d_n0, eq6_e1158_d_n1, eq6_e1158_d_n2, eq6_e1158_d_n3, eq6_e1158_d_n4, eq6_e1158_d_n5, eq6_e1158_d_n6, eq6_e1158_d_n7, eq6_e1158_d_n8, eq6_e1158_d_n9, eq6_e1158_d_n10, eq6_e1158_d_n11, eq6_e1158_d_n12];
        let eq6_branch_derivatives: [f64; 9] = [eq6_e1158_d_b0, eq6_e1158_d_b1, eq6_e1158_d_b2, eq6_e1158_d_b3, eq6_e1158_d_b4, eq6_e1158_d_b5, eq6_e1158_d_b6, eq6_e1158_d_b7, eq6_e1158_d_b8];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let (eq10_e1185, eq10_e1185_d_n0, eq10_e1185_d_n1, eq10_e1185_d_n2, eq10_e1185_d_n3, eq10_e1185_d_n4, eq10_e1185_d_n5, eq10_e1185_d_n6, eq10_e1185_d_n7, eq10_e1185_d_n8, eq10_e1185_d_n9, eq10_e1185_d_n10, eq10_e1185_d_n11, eq10_e1185_d_n12, eq10_e1185_d_b0, eq10_e1185_d_b1, eq10_e1185_d_b2, eq10_e1185_d_b3, eq10_e1185_d_b4, eq10_e1185_d_b5, eq10_e1185_d_b6, eq10_e1185_d_b7, eq10_e1185_d_b8,) = {
    if s.b[1861] {
        let eq10_e1182: f64 = (s.v[1220] + s.v[1268]);
        let eq10_e1182_d_n0: f64 = (s.dn[1220][0] + s.dn[1268][0]);
        let eq10_e1182_d_n1: f64 = (s.dn[1220][1] + s.dn[1268][1]);
        let eq10_e1182_d_n2: f64 = (s.dn[1220][2] + s.dn[1268][2]);
        let eq10_e1182_d_n3: f64 = (s.dn[1220][3] + s.dn[1268][3]);
        let eq10_e1182_d_n4: f64 = (s.dn[1220][4] + s.dn[1268][4]);
        let eq10_e1182_d_n5: f64 = (s.dn[1220][5] + s.dn[1268][5]);
        let eq10_e1182_d_n6: f64 = (s.dn[1220][6] + s.dn[1268][6]);
        let eq10_e1182_d_n7: f64 = (s.dn[1220][7] + s.dn[1268][7]);
        let eq10_e1182_d_n8: f64 = (s.dn[1220][8] + s.dn[1268][8]);
        let eq10_e1182_d_n9: f64 = (s.dn[1220][9] + s.dn[1268][9]);
        let eq10_e1182_d_n10: f64 = (s.dn[1220][10] + s.dn[1268][10]);
        let eq10_e1182_d_n11: f64 = (s.dn[1220][11] + s.dn[1268][11]);
        let eq10_e1182_d_n12: f64 = (s.dn[1220][12] + s.dn[1268][12]);
        let eq10_e1182_d_b0: f64 = (s.db[1220][0] + s.db[1268][0]);
        let eq10_e1182_d_b1: f64 = (s.db[1220][1] + s.db[1268][1]);
        let eq10_e1182_d_b2: f64 = (s.db[1220][2] + s.db[1268][2]);
        let eq10_e1182_d_b3: f64 = (s.db[1220][3] + s.db[1268][3]);
        let eq10_e1182_d_b4: f64 = (s.db[1220][4] + s.db[1268][4]);
        let eq10_e1182_d_b5: f64 = (s.db[1220][5] + s.db[1268][5]);
        let eq10_e1182_d_b6: f64 = (s.db[1220][6] + s.db[1268][6]);
        let eq10_e1182_d_b7: f64 = (s.db[1220][7] + s.db[1268][7]);
        let eq10_e1182_d_b8: f64 = (s.db[1220][8] + s.db[1268][8]);
        let eq10_e1183: f64 = (s.v[36] * eq10_e1182);
        let eq10_e1183_d_n0: f64 = (s.v[36] * eq10_e1182_d_n0);
        let eq10_e1183_d_n1: f64 = (s.v[36] * eq10_e1182_d_n1);
        let eq10_e1183_d_n2: f64 = (s.v[36] * eq10_e1182_d_n2);
        let eq10_e1183_d_n3: f64 = (s.v[36] * eq10_e1182_d_n3);
        let eq10_e1183_d_n4: f64 = (s.v[36] * eq10_e1182_d_n4);
        let eq10_e1183_d_n5: f64 = (s.v[36] * eq10_e1182_d_n5);
        let eq10_e1183_d_n6: f64 = (s.v[36] * eq10_e1182_d_n6);
        let eq10_e1183_d_n7: f64 = (s.v[36] * eq10_e1182_d_n7);
        let eq10_e1183_d_n8: f64 = (s.v[36] * eq10_e1182_d_n8);
        let eq10_e1183_d_n9: f64 = (s.v[36] * eq10_e1182_d_n9);
        let eq10_e1183_d_n10: f64 = (s.v[36] * eq10_e1182_d_n10);
        let eq10_e1183_d_n11: f64 = (s.v[36] * eq10_e1182_d_n11);
        let eq10_e1183_d_n12: f64 = (s.v[36] * eq10_e1182_d_n12);
        let eq10_e1183_d_b0: f64 = (s.v[36] * eq10_e1182_d_b0);
        let eq10_e1183_d_b1: f64 = (s.v[36] * eq10_e1182_d_b1);
        let eq10_e1183_d_b2: f64 = (s.v[36] * eq10_e1182_d_b2);
        let eq10_e1183_d_b3: f64 = (s.v[36] * eq10_e1182_d_b3);
        let eq10_e1183_d_b4: f64 = (s.v[36] * eq10_e1182_d_b4);
        let eq10_e1183_d_b5: f64 = (s.v[36] * eq10_e1182_d_b5);
        let eq10_e1183_d_b6: f64 = (s.v[36] * eq10_e1182_d_b6);
        let eq10_e1183_d_b7: f64 = (s.v[36] * eq10_e1182_d_b7);
        let eq10_e1183_d_b8: f64 = (s.v[36] * eq10_e1182_d_b8);
        (eq10_e1183, eq10_e1183_d_n0, eq10_e1183_d_n1, eq10_e1183_d_n2, eq10_e1183_d_n3, eq10_e1183_d_n4, eq10_e1183_d_n5, eq10_e1183_d_n6, eq10_e1183_d_n7, eq10_e1183_d_n8, eq10_e1183_d_n9, eq10_e1183_d_n10, eq10_e1183_d_n11, eq10_e1183_d_n12, eq10_e1183_d_b0, eq10_e1183_d_b1, eq10_e1183_d_b2, eq10_e1183_d_b3, eq10_e1183_d_b4, eq10_e1183_d_b5, eq10_e1183_d_b6, eq10_e1183_d_b7, eq10_e1183_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e1185;
        let eq10_node_derivatives: [f64; 13] = [eq10_e1185_d_n0, eq10_e1185_d_n1, eq10_e1185_d_n2, eq10_e1185_d_n3, eq10_e1185_d_n4, eq10_e1185_d_n5, eq10_e1185_d_n6, eq10_e1185_d_n7, eq10_e1185_d_n8, eq10_e1185_d_n9, eq10_e1185_d_n10, eq10_e1185_d_n11, eq10_e1185_d_n12];
        let eq10_branch_derivatives: [f64; 9] = [eq10_e1185_d_b0, eq10_e1185_d_b1, eq10_e1185_d_b2, eq10_e1185_d_b3, eq10_e1185_d_b4, eq10_e1185_d_b5, eq10_e1185_d_b6, eq10_e1185_d_b7, eq10_e1185_d_b8];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let (eq11_e1191, eq11_e1191_d_n0, eq11_e1191_d_n1, eq11_e1191_d_n2, eq11_e1191_d_n3, eq11_e1191_d_n4, eq11_e1191_d_n5, eq11_e1191_d_n6, eq11_e1191_d_n7, eq11_e1191_d_n8, eq11_e1191_d_n9, eq11_e1191_d_n10, eq11_e1191_d_n11, eq11_e1191_d_n12, eq11_e1191_d_b0, eq11_e1191_d_b1, eq11_e1191_d_b2, eq11_e1191_d_b3, eq11_e1191_d_b4, eq11_e1191_d_b5, eq11_e1191_d_b6, eq11_e1191_d_b7, eq11_e1191_d_b8,) = {
    if s.b[1861] {
        let eq11_e1189: f64 = (s.v[36] * s.v[1243]);
        (eq11_e1189, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e1191;
        let eq11_node_derivatives: [f64; 13] = [eq11_e1191_d_n0, eq11_e1191_d_n1, eq11_e1191_d_n2, eq11_e1191_d_n3, eq11_e1191_d_n4, eq11_e1191_d_n5, eq11_e1191_d_n6, eq11_e1191_d_n7, eq11_e1191_d_n8, eq11_e1191_d_n9, eq11_e1191_d_n10, eq11_e1191_d_n11, eq11_e1191_d_n12];
        let eq11_branch_derivatives: [f64; 9] = [eq11_e1191_d_b0, eq11_e1191_d_b1, eq11_e1191_d_b2, eq11_e1191_d_b3, eq11_e1191_d_b4, eq11_e1191_d_b5, eq11_e1191_d_b6, eq11_e1191_d_b7, eq11_e1191_d_b8];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let (eq12_e1200, eq12_e1200_d_n0, eq12_e1200_d_n1, eq12_e1200_d_n2, eq12_e1200_d_n3, eq12_e1200_d_n4, eq12_e1200_d_n5, eq12_e1200_d_n6, eq12_e1200_d_n7, eq12_e1200_d_n8, eq12_e1200_d_n9, eq12_e1200_d_n10, eq12_e1200_d_n11, eq12_e1200_d_n12, eq12_e1200_d_b0, eq12_e1200_d_b1, eq12_e1200_d_b2, eq12_e1200_d_b3, eq12_e1200_d_b4, eq12_e1200_d_b5, eq12_e1200_d_b6, eq12_e1200_d_b7, eq12_e1200_d_b8,) = {
    if (!s.b[1861]) {
        let eq12_e1197: f64 = (s.v[1220] - s.v[1268]);
        let eq12_e1197_d_n0: f64 = (s.dn[1220][0] - s.dn[1268][0]);
        let eq12_e1197_d_n1: f64 = (s.dn[1220][1] - s.dn[1268][1]);
        let eq12_e1197_d_n2: f64 = (s.dn[1220][2] - s.dn[1268][2]);
        let eq12_e1197_d_n3: f64 = (s.dn[1220][3] - s.dn[1268][3]);
        let eq12_e1197_d_n4: f64 = (s.dn[1220][4] - s.dn[1268][4]);
        let eq12_e1197_d_n5: f64 = (s.dn[1220][5] - s.dn[1268][5]);
        let eq12_e1197_d_n6: f64 = (s.dn[1220][6] - s.dn[1268][6]);
        let eq12_e1197_d_n7: f64 = (s.dn[1220][7] - s.dn[1268][7]);
        let eq12_e1197_d_n8: f64 = (s.dn[1220][8] - s.dn[1268][8]);
        let eq12_e1197_d_n9: f64 = (s.dn[1220][9] - s.dn[1268][9]);
        let eq12_e1197_d_n10: f64 = (s.dn[1220][10] - s.dn[1268][10]);
        let eq12_e1197_d_n11: f64 = (s.dn[1220][11] - s.dn[1268][11]);
        let eq12_e1197_d_n12: f64 = (s.dn[1220][12] - s.dn[1268][12]);
        let eq12_e1197_d_b0: f64 = (s.db[1220][0] - s.db[1268][0]);
        let eq12_e1197_d_b1: f64 = (s.db[1220][1] - s.db[1268][1]);
        let eq12_e1197_d_b2: f64 = (s.db[1220][2] - s.db[1268][2]);
        let eq12_e1197_d_b3: f64 = (s.db[1220][3] - s.db[1268][3]);
        let eq12_e1197_d_b4: f64 = (s.db[1220][4] - s.db[1268][4]);
        let eq12_e1197_d_b5: f64 = (s.db[1220][5] - s.db[1268][5]);
        let eq12_e1197_d_b6: f64 = (s.db[1220][6] - s.db[1268][6]);
        let eq12_e1197_d_b7: f64 = (s.db[1220][7] - s.db[1268][7]);
        let eq12_e1197_d_b8: f64 = (s.db[1220][8] - s.db[1268][8]);
        let eq12_e1198: f64 = (s.v[36] * eq12_e1197);
        let eq12_e1198_d_n0: f64 = (s.v[36] * eq12_e1197_d_n0);
        let eq12_e1198_d_n1: f64 = (s.v[36] * eq12_e1197_d_n1);
        let eq12_e1198_d_n2: f64 = (s.v[36] * eq12_e1197_d_n2);
        let eq12_e1198_d_n3: f64 = (s.v[36] * eq12_e1197_d_n3);
        let eq12_e1198_d_n4: f64 = (s.v[36] * eq12_e1197_d_n4);
        let eq12_e1198_d_n5: f64 = (s.v[36] * eq12_e1197_d_n5);
        let eq12_e1198_d_n6: f64 = (s.v[36] * eq12_e1197_d_n6);
        let eq12_e1198_d_n7: f64 = (s.v[36] * eq12_e1197_d_n7);
        let eq12_e1198_d_n8: f64 = (s.v[36] * eq12_e1197_d_n8);
        let eq12_e1198_d_n9: f64 = (s.v[36] * eq12_e1197_d_n9);
        let eq12_e1198_d_n10: f64 = (s.v[36] * eq12_e1197_d_n10);
        let eq12_e1198_d_n11: f64 = (s.v[36] * eq12_e1197_d_n11);
        let eq12_e1198_d_n12: f64 = (s.v[36] * eq12_e1197_d_n12);
        let eq12_e1198_d_b0: f64 = (s.v[36] * eq12_e1197_d_b0);
        let eq12_e1198_d_b1: f64 = (s.v[36] * eq12_e1197_d_b1);
        let eq12_e1198_d_b2: f64 = (s.v[36] * eq12_e1197_d_b2);
        let eq12_e1198_d_b3: f64 = (s.v[36] * eq12_e1197_d_b3);
        let eq12_e1198_d_b4: f64 = (s.v[36] * eq12_e1197_d_b4);
        let eq12_e1198_d_b5: f64 = (s.v[36] * eq12_e1197_d_b5);
        let eq12_e1198_d_b6: f64 = (s.v[36] * eq12_e1197_d_b6);
        let eq12_e1198_d_b7: f64 = (s.v[36] * eq12_e1197_d_b7);
        let eq12_e1198_d_b8: f64 = (s.v[36] * eq12_e1197_d_b8);
        (eq12_e1198, eq12_e1198_d_n0, eq12_e1198_d_n1, eq12_e1198_d_n2, eq12_e1198_d_n3, eq12_e1198_d_n4, eq12_e1198_d_n5, eq12_e1198_d_n6, eq12_e1198_d_n7, eq12_e1198_d_n8, eq12_e1198_d_n9, eq12_e1198_d_n10, eq12_e1198_d_n11, eq12_e1198_d_n12, eq12_e1198_d_b0, eq12_e1198_d_b1, eq12_e1198_d_b2, eq12_e1198_d_b3, eq12_e1198_d_b4, eq12_e1198_d_b5, eq12_e1198_d_b6, eq12_e1198_d_b7, eq12_e1198_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq12_value: f64 = eq12_e1200;
        let eq12_node_derivatives: [f64; 13] = [eq12_e1200_d_n0, eq12_e1200_d_n1, eq12_e1200_d_n2, eq12_e1200_d_n3, eq12_e1200_d_n4, eq12_e1200_d_n5, eq12_e1200_d_n6, eq12_e1200_d_n7, eq12_e1200_d_n8, eq12_e1200_d_n9, eq12_e1200_d_n10, eq12_e1200_d_n11, eq12_e1200_d_n12];
        let eq12_branch_derivatives: [f64; 9] = [eq12_e1200_d_b0, eq12_e1200_d_b1, eq12_e1200_d_b2, eq12_e1200_d_b3, eq12_e1200_d_b4, eq12_e1200_d_b5, eq12_e1200_d_b6, eq12_e1200_d_b7, eq12_e1200_d_b8];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let (eq13_e1207, eq13_e1207_d_n0, eq13_e1207_d_n1, eq13_e1207_d_n2, eq13_e1207_d_n3, eq13_e1207_d_n4, eq13_e1207_d_n5, eq13_e1207_d_n6, eq13_e1207_d_n7, eq13_e1207_d_n8, eq13_e1207_d_n9, eq13_e1207_d_n10, eq13_e1207_d_n11, eq13_e1207_d_n12, eq13_e1207_d_b0, eq13_e1207_d_b1, eq13_e1207_d_b2, eq13_e1207_d_b3, eq13_e1207_d_b4, eq13_e1207_d_b5, eq13_e1207_d_b6, eq13_e1207_d_b7, eq13_e1207_d_b8,) = {
    if (!s.b[1861]) {
        let eq13_e1205: f64 = (s.v[36] * s.v[1243]);
        (eq13_e1205, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq13_value: f64 = eq13_e1207;
        let eq13_node_derivatives: [f64; 13] = [eq13_e1207_d_n0, eq13_e1207_d_n1, eq13_e1207_d_n2, eq13_e1207_d_n3, eq13_e1207_d_n4, eq13_e1207_d_n5, eq13_e1207_d_n6, eq13_e1207_d_n7, eq13_e1207_d_n8, eq13_e1207_d_n9, eq13_e1207_d_n10, eq13_e1207_d_n11, eq13_e1207_d_n12];
        let eq13_branch_derivatives: [f64; 9] = [eq13_e1207_d_b0, eq13_e1207_d_b1, eq13_e1207_d_b2, eq13_e1207_d_b3, eq13_e1207_d_b4, eq13_e1207_d_b5, eq13_e1207_d_b6, eq13_e1207_d_b7, eq13_e1207_d_b8];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq14_value: f64 = s.v[419];
        let eq14_node_derivatives: [f64; 13] = [s.dn[419][0], s.dn[419][1], s.dn[419][2], s.dn[419][3], s.dn[419][4], s.dn[419][5], s.dn[419][6], s.dn[419][7], s.dn[419][8], s.dn[419][9], s.dn[419][10], s.dn[419][11], s.dn[419][12]];
        let eq14_branch_derivatives: [f64; 9] = [s.db[419][0], s.db[419][1], s.db[419][2], s.db[419][3], s.db[419][4], s.db[419][5], s.db[419][6], s.db[419][7], s.db[419][8]];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let eq15_value: f64 = s.v[420];
        let eq15_node_derivatives: [f64; 13] = [s.dn[420][0], s.dn[420][1], s.dn[420][2], s.dn[420][3], s.dn[420][4], s.dn[420][5], s.dn[420][6], s.dn[420][7], s.dn[420][8], s.dn[420][9], s.dn[420][10], s.dn[420][11], s.dn[420][12]];
        let eq15_branch_derivatives: [f64; 9] = [s.db[420][0], s.db[420][1], s.db[420][2], s.db[420][3], s.db[420][4], s.db[420][5], s.db[420][6], s.db[420][7], s.db[420][8]];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let eq16_e1212: f64 = (s.v[36] * s.v[1270]);
        let eq16_e1212_d_n0: f64 = (s.v[36] * s.dn[1270][0]);
        let eq16_e1212_d_n1: f64 = (s.v[36] * s.dn[1270][1]);
        let eq16_e1212_d_n2: f64 = (s.v[36] * s.dn[1270][2]);
        let eq16_e1212_d_n3: f64 = (s.v[36] * s.dn[1270][3]);
        let eq16_e1212_d_n4: f64 = (s.v[36] * s.dn[1270][4]);
        let eq16_e1212_d_n5: f64 = (s.v[36] * s.dn[1270][5]);
        let eq16_e1212_d_n6: f64 = (s.v[36] * s.dn[1270][6]);
        let eq16_e1212_d_n7: f64 = (s.v[36] * s.dn[1270][7]);
        let eq16_e1212_d_n8: f64 = (s.v[36] * s.dn[1270][8]);
        let eq16_e1212_d_n9: f64 = (s.v[36] * s.dn[1270][9]);
        let eq16_e1212_d_n10: f64 = (s.v[36] * s.dn[1270][10]);
        let eq16_e1212_d_n11: f64 = (s.v[36] * s.dn[1270][11]);
        let eq16_e1212_d_n12: f64 = (s.v[36] * s.dn[1270][12]);
        let eq16_e1212_d_b0: f64 = (s.v[36] * s.db[1270][0]);
        let eq16_e1212_d_b1: f64 = (s.v[36] * s.db[1270][1]);
        let eq16_e1212_d_b2: f64 = (s.v[36] * s.db[1270][2]);
        let eq16_e1212_d_b3: f64 = (s.v[36] * s.db[1270][3]);
        let eq16_e1212_d_b4: f64 = (s.v[36] * s.db[1270][4]);
        let eq16_e1212_d_b5: f64 = (s.v[36] * s.db[1270][5]);
        let eq16_e1212_d_b6: f64 = (s.v[36] * s.db[1270][6]);
        let eq16_e1212_d_b7: f64 = (s.v[36] * s.db[1270][7]);
        let eq16_e1212_d_b8: f64 = (s.v[36] * s.db[1270][8]);
        let eq16_value: f64 = eq16_e1212;
        let eq16_node_derivatives: [f64; 13] = [eq16_e1212_d_n0, eq16_e1212_d_n1, eq16_e1212_d_n2, eq16_e1212_d_n3, eq16_e1212_d_n4, eq16_e1212_d_n5, eq16_e1212_d_n6, eq16_e1212_d_n7, eq16_e1212_d_n8, eq16_e1212_d_n9, eq16_e1212_d_n10, eq16_e1212_d_n11, eq16_e1212_d_n12];
        let eq16_branch_derivatives: [f64; 9] = [eq16_e1212_d_b0, eq16_e1212_d_b1, eq16_e1212_d_b2, eq16_e1212_d_b3, eq16_e1212_d_b4, eq16_e1212_d_b5, eq16_e1212_d_b6, eq16_e1212_d_b7, eq16_e1212_d_b8];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(7),
            multiplicity * (eq16_value),
            &eq16_node_derivatives,
            &eq16_branch_derivatives,
            multiplicity,
        );
        let eq17_e1215: f64 = (s.v[36] * s.v[1269]);
        let eq17_e1215_d_n0: f64 = (s.v[36] * s.dn[1269][0]);
        let eq17_e1215_d_n1: f64 = (s.v[36] * s.dn[1269][1]);
        let eq17_e1215_d_n2: f64 = (s.v[36] * s.dn[1269][2]);
        let eq17_e1215_d_n3: f64 = (s.v[36] * s.dn[1269][3]);
        let eq17_e1215_d_n4: f64 = (s.v[36] * s.dn[1269][4]);
        let eq17_e1215_d_n5: f64 = (s.v[36] * s.dn[1269][5]);
        let eq17_e1215_d_n6: f64 = (s.v[36] * s.dn[1269][6]);
        let eq17_e1215_d_n7: f64 = (s.v[36] * s.dn[1269][7]);
        let eq17_e1215_d_n8: f64 = (s.v[36] * s.dn[1269][8]);
        let eq17_e1215_d_n9: f64 = (s.v[36] * s.dn[1269][9]);
        let eq17_e1215_d_n10: f64 = (s.v[36] * s.dn[1269][10]);
        let eq17_e1215_d_n11: f64 = (s.v[36] * s.dn[1269][11]);
        let eq17_e1215_d_n12: f64 = (s.v[36] * s.dn[1269][12]);
        let eq17_e1215_d_b0: f64 = (s.v[36] * s.db[1269][0]);
        let eq17_e1215_d_b1: f64 = (s.v[36] * s.db[1269][1]);
        let eq17_e1215_d_b2: f64 = (s.v[36] * s.db[1269][2]);
        let eq17_e1215_d_b3: f64 = (s.v[36] * s.db[1269][3]);
        let eq17_e1215_d_b4: f64 = (s.v[36] * s.db[1269][4]);
        let eq17_e1215_d_b5: f64 = (s.v[36] * s.db[1269][5]);
        let eq17_e1215_d_b6: f64 = (s.v[36] * s.db[1269][6]);
        let eq17_e1215_d_b7: f64 = (s.v[36] * s.db[1269][7]);
        let eq17_e1215_d_b8: f64 = (s.v[36] * s.db[1269][8]);
        let eq17_value: f64 = eq17_e1215;
        let eq17_node_derivatives: [f64; 13] = [eq17_e1215_d_n0, eq17_e1215_d_n1, eq17_e1215_d_n2, eq17_e1215_d_n3, eq17_e1215_d_n4, eq17_e1215_d_n5, eq17_e1215_d_n6, eq17_e1215_d_n7, eq17_e1215_d_n8, eq17_e1215_d_n9, eq17_e1215_d_n10, eq17_e1215_d_n11, eq17_e1215_d_n12];
        let eq17_branch_derivatives: [f64; 9] = [eq17_e1215_d_b0, eq17_e1215_d_b1, eq17_e1215_d_b2, eq17_e1215_d_b3, eq17_e1215_d_b4, eq17_e1215_d_b5, eq17_e1215_d_b6, eq17_e1215_d_b7, eq17_e1215_d_b8];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(8),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let eq18_e1218: f64 = (s.v[445] + s.v[443]);
        let eq18_e1218_d_n0: f64 = (s.dn[445][0] + s.dn[443][0]);
        let eq18_e1218_d_n1: f64 = (s.dn[445][1] + s.dn[443][1]);
        let eq18_e1218_d_n2: f64 = (s.dn[445][2] + s.dn[443][2]);
        let eq18_e1218_d_n3: f64 = (s.dn[445][3] + s.dn[443][3]);
        let eq18_e1218_d_n4: f64 = (s.dn[445][4] + s.dn[443][4]);
        let eq18_e1218_d_n5: f64 = (s.dn[445][5] + s.dn[443][5]);
        let eq18_e1218_d_n6: f64 = (s.dn[445][6] + s.dn[443][6]);
        let eq18_e1218_d_n7: f64 = (s.dn[445][7] + s.dn[443][7]);
        let eq18_e1218_d_n8: f64 = (s.dn[445][8] + s.dn[443][8]);
        let eq18_e1218_d_n9: f64 = (s.dn[445][9] + s.dn[443][9]);
        let eq18_e1218_d_n10: f64 = (s.dn[445][10] + s.dn[443][10]);
        let eq18_e1218_d_n11: f64 = (s.dn[445][11] + s.dn[443][11]);
        let eq18_e1218_d_n12: f64 = (s.dn[445][12] + s.dn[443][12]);
        let eq18_e1218_d_b0: f64 = (s.db[445][0] + s.db[443][0]);
        let eq18_e1218_d_b1: f64 = (s.db[445][1] + s.db[443][1]);
        let eq18_e1218_d_b2: f64 = (s.db[445][2] + s.db[443][2]);
        let eq18_e1218_d_b3: f64 = (s.db[445][3] + s.db[443][3]);
        let eq18_e1218_d_b4: f64 = (s.db[445][4] + s.db[443][4]);
        let eq18_e1218_d_b5: f64 = (s.db[445][5] + s.db[443][5]);
        let eq18_e1218_d_b6: f64 = (s.db[445][6] + s.db[443][6]);
        let eq18_e1218_d_b7: f64 = (s.db[445][7] + s.db[443][7]);
        let eq18_e1218_d_b8: f64 = (s.db[445][8] + s.db[443][8]);
        let eq18_value: f64 = eq18_e1218;
        let eq18_node_derivatives: [f64; 13] = [eq18_e1218_d_n0, eq18_e1218_d_n1, eq18_e1218_d_n2, eq18_e1218_d_n3, eq18_e1218_d_n4, eq18_e1218_d_n5, eq18_e1218_d_n6, eq18_e1218_d_n7, eq18_e1218_d_n8, eq18_e1218_d_n9, eq18_e1218_d_n10, eq18_e1218_d_n11, eq18_e1218_d_n12];
        let eq18_branch_derivatives: [f64; 9] = [eq18_e1218_d_b0, eq18_e1218_d_b1, eq18_e1218_d_b2, eq18_e1218_d_b3, eq18_e1218_d_b4, eq18_e1218_d_b5, eq18_e1218_d_b6, eq18_e1218_d_b7, eq18_e1218_d_b8];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let eq19_e1221: f64 = (s.v[444] + s.v[442]);
        let eq19_e1221_d_n0: f64 = (s.dn[444][0] + s.dn[442][0]);
        let eq19_e1221_d_n1: f64 = (s.dn[444][1] + s.dn[442][1]);
        let eq19_e1221_d_n2: f64 = (s.dn[444][2] + s.dn[442][2]);
        let eq19_e1221_d_n3: f64 = (s.dn[444][3] + s.dn[442][3]);
        let eq19_e1221_d_n4: f64 = (s.dn[444][4] + s.dn[442][4]);
        let eq19_e1221_d_n5: f64 = (s.dn[444][5] + s.dn[442][5]);
        let eq19_e1221_d_n6: f64 = (s.dn[444][6] + s.dn[442][6]);
        let eq19_e1221_d_n7: f64 = (s.dn[444][7] + s.dn[442][7]);
        let eq19_e1221_d_n8: f64 = (s.dn[444][8] + s.dn[442][8]);
        let eq19_e1221_d_n9: f64 = (s.dn[444][9] + s.dn[442][9]);
        let eq19_e1221_d_n10: f64 = (s.dn[444][10] + s.dn[442][10]);
        let eq19_e1221_d_n11: f64 = (s.dn[444][11] + s.dn[442][11]);
        let eq19_e1221_d_n12: f64 = (s.dn[444][12] + s.dn[442][12]);
        let eq19_e1221_d_b0: f64 = (s.db[444][0] + s.db[442][0]);
        let eq19_e1221_d_b1: f64 = (s.db[444][1] + s.db[442][1]);
        let eq19_e1221_d_b2: f64 = (s.db[444][2] + s.db[442][2]);
        let eq19_e1221_d_b3: f64 = (s.db[444][3] + s.db[442][3]);
        let eq19_e1221_d_b4: f64 = (s.db[444][4] + s.db[442][4]);
        let eq19_e1221_d_b5: f64 = (s.db[444][5] + s.db[442][5]);
        let eq19_e1221_d_b6: f64 = (s.db[444][6] + s.db[442][6]);
        let eq19_e1221_d_b7: f64 = (s.db[444][7] + s.db[442][7]);
        let eq19_e1221_d_b8: f64 = (s.db[444][8] + s.db[442][8]);
        let eq19_value: f64 = eq19_e1221;
        let eq19_node_derivatives: [f64; 13] = [eq19_e1221_d_n0, eq19_e1221_d_n1, eq19_e1221_d_n2, eq19_e1221_d_n3, eq19_e1221_d_n4, eq19_e1221_d_n5, eq19_e1221_d_n6, eq19_e1221_d_n7, eq19_e1221_d_n8, eq19_e1221_d_n9, eq19_e1221_d_n10, eq19_e1221_d_n11, eq19_e1221_d_n12];
        let eq19_branch_derivatives: [f64; 9] = [eq19_e1221_d_b0, eq19_e1221_d_b1, eq19_e1221_d_b2, eq19_e1221_d_b3, eq19_e1221_d_b4, eq19_e1221_d_b5, eq19_e1221_d_b6, eq19_e1221_d_b7, eq19_e1221_d_b8];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let eq20_value: f64 = s.v[412];
        let eq20_node_derivatives: [f64; 13] = [s.dn[412][0], s.dn[412][1], s.dn[412][2], s.dn[412][3], s.dn[412][4], s.dn[412][5], s.dn[412][6], s.dn[412][7], s.dn[412][8], s.dn[412][9], s.dn[412][10], s.dn[412][11], s.dn[412][12]];
        let eq20_branch_derivatives: [f64; 9] = [s.db[412][0], s.db[412][1], s.db[412][2], s.db[412][3], s.db[412][4], s.db[412][5], s.db[412][6], s.db[412][7], s.db[412][8]];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(5),
            multiplicity * (eq20_value),
            &eq20_node_derivatives,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let eq21_value: f64 = s.v[417];
        let eq21_node_derivatives: [f64; 13] = [s.dn[417][0], s.dn[417][1], s.dn[417][2], s.dn[417][3], s.dn[417][4], s.dn[417][5], s.dn[417][6], s.dn[417][7], s.dn[417][8], s.dn[417][9], s.dn[417][10], s.dn[417][11], s.dn[417][12]];
        let eq21_branch_derivatives: [f64; 9] = [s.db[417][0], s.db[417][1], s.db[417][2], s.db[417][3], s.db[417][4], s.db[417][5], s.db[417][6], s.db[417][7], s.db[417][8]];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(4),
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let __rspice_deriv_cse_0: f64 = (s.v[36] * (s.dn[1230][0] * ddt_scale));
        let __rspice_deriv_cse_1: f64 = (s.v[36] * (s.dn[1230][1] * ddt_scale));
        let __rspice_deriv_cse_2: f64 = (s.v[36] * (s.dn[1230][2] * ddt_scale));
        let __rspice_deriv_cse_3: f64 = (s.v[36] * (s.dn[1230][3] * ddt_scale));
        let __rspice_deriv_cse_4: f64 = (s.v[36] * (s.dn[1230][4] * ddt_scale));
        let __rspice_deriv_cse_5: f64 = (s.v[36] * (s.dn[1230][5] * ddt_scale));
        let __rspice_deriv_cse_6: f64 = (s.v[36] * (s.dn[1230][6] * ddt_scale));
        let __rspice_deriv_cse_7: f64 = (s.v[36] * (s.dn[1230][7] * ddt_scale));
        let __rspice_deriv_cse_8: f64 = (s.v[36] * (s.dn[1230][8] * ddt_scale));
        let __rspice_deriv_cse_9: f64 = (s.v[36] * (s.dn[1230][9] * ddt_scale));
        let __rspice_deriv_cse_10: f64 = (s.v[36] * (s.dn[1230][10] * ddt_scale));
        let __rspice_deriv_cse_11: f64 = (s.v[36] * (s.dn[1230][11] * ddt_scale));
        let __rspice_deriv_cse_12: f64 = (s.v[36] * (s.dn[1230][12] * ddt_scale));
        let __rspice_deriv_cse_13: f64 = (s.v[36] * (s.db[1230][0] * ddt_scale));
        let __rspice_deriv_cse_14: f64 = (s.v[36] * (s.db[1230][1] * ddt_scale));
        let __rspice_deriv_cse_15: f64 = (s.v[36] * (s.db[1230][2] * ddt_scale));
        let __rspice_deriv_cse_16: f64 = (s.v[36] * (s.db[1230][3] * ddt_scale));
        let __rspice_deriv_cse_17: f64 = (s.v[36] * (s.db[1230][4] * ddt_scale));
        let __rspice_deriv_cse_18: f64 = (s.v[36] * (s.db[1230][5] * ddt_scale));
        let __rspice_deriv_cse_19: f64 = (s.v[36] * (s.db[1230][6] * ddt_scale));
        let __rspice_deriv_cse_20: f64 = (s.v[36] * (s.db[1230][7] * ddt_scale));
        let __rspice_deriv_cse_21: f64 = (s.v[36] * (s.db[1230][8] * ddt_scale));
        let __rspice_deriv_cse_22: f64 = (s.v[36] * (s.dn[1231][0] * ddt_scale));
        let __rspice_deriv_cse_23: f64 = (s.v[36] * (s.dn[1231][1] * ddt_scale));
        let __rspice_deriv_cse_24: f64 = (s.v[36] * (s.dn[1231][2] * ddt_scale));
        let __rspice_deriv_cse_25: f64 = (s.v[36] * (s.dn[1231][3] * ddt_scale));
        let __rspice_deriv_cse_26: f64 = (s.v[36] * (s.dn[1231][4] * ddt_scale));
        let __rspice_deriv_cse_27: f64 = (s.v[36] * (s.dn[1231][5] * ddt_scale));
        let __rspice_deriv_cse_28: f64 = (s.v[36] * (s.dn[1231][6] * ddt_scale));
        let __rspice_deriv_cse_29: f64 = (s.v[36] * (s.dn[1231][7] * ddt_scale));
        let __rspice_deriv_cse_30: f64 = (s.v[36] * (s.dn[1231][8] * ddt_scale));
        let __rspice_deriv_cse_31: f64 = (s.v[36] * (s.dn[1231][9] * ddt_scale));
        let __rspice_deriv_cse_32: f64 = (s.v[36] * (s.dn[1231][10] * ddt_scale));
        let __rspice_deriv_cse_33: f64 = (s.v[36] * (s.dn[1231][11] * ddt_scale));
        let __rspice_deriv_cse_34: f64 = (s.v[36] * (s.dn[1231][12] * ddt_scale));
        let __rspice_deriv_cse_35: f64 = (s.v[36] * (s.db[1231][0] * ddt_scale));
        let __rspice_deriv_cse_36: f64 = (s.v[36] * (s.db[1231][1] * ddt_scale));
        let __rspice_deriv_cse_37: f64 = (s.v[36] * (s.db[1231][2] * ddt_scale));
        let __rspice_deriv_cse_38: f64 = (s.v[36] * (s.db[1231][3] * ddt_scale));
        let __rspice_deriv_cse_39: f64 = (s.v[36] * (s.db[1231][4] * ddt_scale));
        let __rspice_deriv_cse_40: f64 = (s.v[36] * (s.db[1231][5] * ddt_scale));
        let __rspice_deriv_cse_41: f64 = (s.v[36] * (s.db[1231][6] * ddt_scale));
        let __rspice_deriv_cse_42: f64 = (s.v[36] * (s.db[1231][7] * ddt_scale));
        let __rspice_deriv_cse_43: f64 = (s.v[36] * (s.db[1231][8] * ddt_scale));
        let (eq23_e1234, eq23_e1234_d_n0, eq23_e1234_d_n1, eq23_e1234_d_n2, eq23_e1234_d_n3, eq23_e1234_d_n4, eq23_e1234_d_n5, eq23_e1234_d_n6, eq23_e1234_d_n7, eq23_e1234_d_n8, eq23_e1234_d_n9, eq23_e1234_d_n10, eq23_e1234_d_n11, eq23_e1234_d_n12, eq23_e1234_d_b0, eq23_e1234_d_b1, eq23_e1234_d_b2, eq23_e1234_d_b3, eq23_e1234_d_b4, eq23_e1234_d_b5, eq23_e1234_d_b6, eq23_e1234_d_b7, eq23_e1234_d_b8,) = {
    if (!s.b[1862]) {
        let eq23_e1232: f64 = (s.v[36] * s.v[1242]);
        let eq23_e1232_d_n0: f64 = (s.v[36] * s.dn[1242][0]);
        let eq23_e1232_d_n1: f64 = (s.v[36] * s.dn[1242][1]);
        let eq23_e1232_d_n2: f64 = (s.v[36] * s.dn[1242][2]);
        let eq23_e1232_d_n3: f64 = (s.v[36] * s.dn[1242][3]);
        let eq23_e1232_d_n4: f64 = (s.v[36] * s.dn[1242][4]);
        let eq23_e1232_d_n5: f64 = (s.v[36] * s.dn[1242][5]);
        let eq23_e1232_d_n6: f64 = (s.v[36] * s.dn[1242][6]);
        let eq23_e1232_d_n7: f64 = (s.v[36] * s.dn[1242][7]);
        let eq23_e1232_d_n8: f64 = (s.v[36] * s.dn[1242][8]);
        let eq23_e1232_d_n9: f64 = (s.v[36] * s.dn[1242][9]);
        let eq23_e1232_d_n10: f64 = (s.v[36] * s.dn[1242][10]);
        let eq23_e1232_d_n11: f64 = (s.v[36] * s.dn[1242][11]);
        let eq23_e1232_d_n12: f64 = (s.v[36] * s.dn[1242][12]);
        let eq23_e1232_d_b0: f64 = (s.v[36] * s.db[1242][0]);
        let eq23_e1232_d_b1: f64 = (s.v[36] * s.db[1242][1]);
        let eq23_e1232_d_b2: f64 = (s.v[36] * s.db[1242][2]);
        let eq23_e1232_d_b3: f64 = (s.v[36] * s.db[1242][3]);
        let eq23_e1232_d_b4: f64 = (s.v[36] * s.db[1242][4]);
        let eq23_e1232_d_b5: f64 = (s.v[36] * s.db[1242][5]);
        let eq23_e1232_d_b6: f64 = (s.v[36] * s.db[1242][6]);
        let eq23_e1232_d_b7: f64 = (s.v[36] * s.db[1242][7]);
        let eq23_e1232_d_b8: f64 = (s.v[36] * s.db[1242][8]);
        (eq23_e1232, eq23_e1232_d_n0, eq23_e1232_d_n1, eq23_e1232_d_n2, eq23_e1232_d_n3, eq23_e1232_d_n4, eq23_e1232_d_n5, eq23_e1232_d_n6, eq23_e1232_d_n7, eq23_e1232_d_n8, eq23_e1232_d_n9, eq23_e1232_d_n10, eq23_e1232_d_n11, eq23_e1232_d_n12, eq23_e1232_d_b0, eq23_e1232_d_b1, eq23_e1232_d_b2, eq23_e1232_d_b3, eq23_e1232_d_b4, eq23_e1232_d_b5, eq23_e1232_d_b6, eq23_e1232_d_b7, eq23_e1232_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e1234;
        let eq23_node_derivatives: [f64; 13] = [eq23_e1234_d_n0, eq23_e1234_d_n1, eq23_e1234_d_n2, eq23_e1234_d_n3, eq23_e1234_d_n4, eq23_e1234_d_n5, eq23_e1234_d_n6, eq23_e1234_d_n7, eq23_e1234_d_n8, eq23_e1234_d_n9, eq23_e1234_d_n10, eq23_e1234_d_n11, eq23_e1234_d_n12];
        let eq23_branch_derivatives: [f64; 9] = [eq23_e1234_d_b0, eq23_e1234_d_b1, eq23_e1234_d_b2, eq23_e1234_d_b3, eq23_e1234_d_b4, eq23_e1234_d_b5, eq23_e1234_d_b6, eq23_e1234_d_b7, eq23_e1234_d_b8];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(4),
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
            multiplicity,
        );
        let eq30_e1299: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, s.v[446]);
        let eq30_value: f64 = eq30_e1299;
        let eq30_node_derivatives: [f64; 13] = [(s.dn[446][0] * ddt_scale), (s.dn[446][1] * ddt_scale), (s.dn[446][2] * ddt_scale), (s.dn[446][3] * ddt_scale), (s.dn[446][4] * ddt_scale), (s.dn[446][5] * ddt_scale), (s.dn[446][6] * ddt_scale), (s.dn[446][7] * ddt_scale), (s.dn[446][8] * ddt_scale), (s.dn[446][9] * ddt_scale), (s.dn[446][10] * ddt_scale), (s.dn[446][11] * ddt_scale), (s.dn[446][12] * ddt_scale)];
        let eq30_branch_derivatives: [f64; 9] = [(s.db[446][0] * ddt_scale), (s.db[446][1] * ddt_scale), (s.db[446][2] * ddt_scale), (s.db[446][3] * ddt_scale), (s.db[446][4] * ddt_scale), (s.db[446][5] * ddt_scale), (s.db[446][6] * ddt_scale), (s.db[446][7] * ddt_scale), (s.db[446][8] * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq30_value),
            &eq30_node_derivatives,
            &eq30_branch_derivatives,
            multiplicity,
        );
        let eq31_e1301: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, s.v[447]);
        let eq31_value: f64 = eq31_e1301;
        let eq31_node_derivatives: [f64; 13] = [(s.dn[447][0] * ddt_scale), (s.dn[447][1] * ddt_scale), (s.dn[447][2] * ddt_scale), (s.dn[447][3] * ddt_scale), (s.dn[447][4] * ddt_scale), (s.dn[447][5] * ddt_scale), (s.dn[447][6] * ddt_scale), (s.dn[447][7] * ddt_scale), (s.dn[447][8] * ddt_scale), (s.dn[447][9] * ddt_scale), (s.dn[447][10] * ddt_scale), (s.dn[447][11] * ddt_scale), (s.dn[447][12] * ddt_scale)];
        let eq31_branch_derivatives: [f64; 9] = [(s.db[447][0] * ddt_scale), (s.db[447][1] * ddt_scale), (s.db[447][2] * ddt_scale), (s.db[447][3] * ddt_scale), (s.db[447][4] * ddt_scale), (s.db[447][5] * ddt_scale), (s.db[447][6] * ddt_scale), (s.db[447][7] * ddt_scale), (s.db[447][8] * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq31_value),
            &eq31_node_derivatives,
            &eq31_branch_derivatives,
            multiplicity,
        );
        let eq32_e1304: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, s.v[1251]);
        let eq32_e1305: f64 = (s.v[36] * eq32_e1304);
        let eq32_e1305_d_n0: f64 = (s.v[36] * (s.dn[1251][0] * ddt_scale));
        let eq32_e1305_d_n1: f64 = (s.v[36] * (s.dn[1251][1] * ddt_scale));
        let eq32_e1305_d_n2: f64 = (s.v[36] * (s.dn[1251][2] * ddt_scale));
        let eq32_e1305_d_n3: f64 = (s.v[36] * (s.dn[1251][3] * ddt_scale));
        let eq32_e1305_d_n4: f64 = (s.v[36] * (s.dn[1251][4] * ddt_scale));
        let eq32_e1305_d_n5: f64 = (s.v[36] * (s.dn[1251][5] * ddt_scale));
        let eq32_e1305_d_n6: f64 = (s.v[36] * (s.dn[1251][6] * ddt_scale));
        let eq32_e1305_d_n7: f64 = (s.v[36] * (s.dn[1251][7] * ddt_scale));
        let eq32_e1305_d_n8: f64 = (s.v[36] * (s.dn[1251][8] * ddt_scale));
        let eq32_e1305_d_n9: f64 = (s.v[36] * (s.dn[1251][9] * ddt_scale));
        let eq32_e1305_d_n10: f64 = (s.v[36] * (s.dn[1251][10] * ddt_scale));
        let eq32_e1305_d_n11: f64 = (s.v[36] * (s.dn[1251][11] * ddt_scale));
        let eq32_e1305_d_n12: f64 = (s.v[36] * (s.dn[1251][12] * ddt_scale));
        let eq32_e1305_d_b0: f64 = (s.v[36] * (s.db[1251][0] * ddt_scale));
        let eq32_e1305_d_b1: f64 = (s.v[36] * (s.db[1251][1] * ddt_scale));
        let eq32_e1305_d_b2: f64 = (s.v[36] * (s.db[1251][2] * ddt_scale));
        let eq32_e1305_d_b3: f64 = (s.v[36] * (s.db[1251][3] * ddt_scale));
        let eq32_e1305_d_b4: f64 = (s.v[36] * (s.db[1251][4] * ddt_scale));
        let eq32_e1305_d_b5: f64 = (s.v[36] * (s.db[1251][5] * ddt_scale));
        let eq32_e1305_d_b6: f64 = (s.v[36] * (s.db[1251][6] * ddt_scale));
        let eq32_e1305_d_b7: f64 = (s.v[36] * (s.db[1251][7] * ddt_scale));
        let eq32_e1305_d_b8: f64 = (s.v[36] * (s.db[1251][8] * ddt_scale));
        let eq32_value: f64 = eq32_e1305;
        let eq32_node_derivatives: [f64; 13] = [eq32_e1305_d_n0, eq32_e1305_d_n1, eq32_e1305_d_n2, eq32_e1305_d_n3, eq32_e1305_d_n4, eq32_e1305_d_n5, eq32_e1305_d_n6, eq32_e1305_d_n7, eq32_e1305_d_n8, eq32_e1305_d_n9, eq32_e1305_d_n10, eq32_e1305_d_n11, eq32_e1305_d_n12];
        let eq32_branch_derivatives: [f64; 9] = [eq32_e1305_d_b0, eq32_e1305_d_b1, eq32_e1305_d_b2, eq32_e1305_d_b3, eq32_e1305_d_b4, eq32_e1305_d_b5, eq32_e1305_d_b6, eq32_e1305_d_b7, eq32_e1305_d_b8];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(5),
            multiplicity * (eq32_value),
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
            multiplicity,
        );
        let eq33_e1308: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, s.v[1255]);
        let eq33_e1309: f64 = (s.v[36] * eq33_e1308);
        let eq33_e1309_d_n0: f64 = (s.v[36] * (s.dn[1255][0] * ddt_scale));
        let eq33_e1309_d_n1: f64 = (s.v[36] * (s.dn[1255][1] * ddt_scale));
        let eq33_e1309_d_n2: f64 = (s.v[36] * (s.dn[1255][2] * ddt_scale));
        let eq33_e1309_d_n3: f64 = (s.v[36] * (s.dn[1255][3] * ddt_scale));
        let eq33_e1309_d_n4: f64 = (s.v[36] * (s.dn[1255][4] * ddt_scale));
        let eq33_e1309_d_n5: f64 = (s.v[36] * (s.dn[1255][5] * ddt_scale));
        let eq33_e1309_d_n6: f64 = (s.v[36] * (s.dn[1255][6] * ddt_scale));
        let eq33_e1309_d_n7: f64 = (s.v[36] * (s.dn[1255][7] * ddt_scale));
        let eq33_e1309_d_n8: f64 = (s.v[36] * (s.dn[1255][8] * ddt_scale));
        let eq33_e1309_d_n9: f64 = (s.v[36] * (s.dn[1255][9] * ddt_scale));
        let eq33_e1309_d_n10: f64 = (s.v[36] * (s.dn[1255][10] * ddt_scale));
        let eq33_e1309_d_n11: f64 = (s.v[36] * (s.dn[1255][11] * ddt_scale));
        let eq33_e1309_d_n12: f64 = (s.v[36] * (s.dn[1255][12] * ddt_scale));
        let eq33_e1309_d_b0: f64 = (s.v[36] * (s.db[1255][0] * ddt_scale));
        let eq33_e1309_d_b1: f64 = (s.v[36] * (s.db[1255][1] * ddt_scale));
        let eq33_e1309_d_b2: f64 = (s.v[36] * (s.db[1255][2] * ddt_scale));
        let eq33_e1309_d_b3: f64 = (s.v[36] * (s.db[1255][3] * ddt_scale));
        let eq33_e1309_d_b4: f64 = (s.v[36] * (s.db[1255][4] * ddt_scale));
        let eq33_e1309_d_b5: f64 = (s.v[36] * (s.db[1255][5] * ddt_scale));
        let eq33_e1309_d_b6: f64 = (s.v[36] * (s.db[1255][6] * ddt_scale));
        let eq33_e1309_d_b7: f64 = (s.v[36] * (s.db[1255][7] * ddt_scale));
        let eq33_e1309_d_b8: f64 = (s.v[36] * (s.db[1255][8] * ddt_scale));
        let eq33_value: f64 = eq33_e1309;
        let eq33_node_derivatives: [f64; 13] = [eq33_e1309_d_n0, eq33_e1309_d_n1, eq33_e1309_d_n2, eq33_e1309_d_n3, eq33_e1309_d_n4, eq33_e1309_d_n5, eq33_e1309_d_n6, eq33_e1309_d_n7, eq33_e1309_d_n8, eq33_e1309_d_n9, eq33_e1309_d_n10, eq33_e1309_d_n11, eq33_e1309_d_n12];
        let eq33_branch_derivatives: [f64; 9] = [eq33_e1309_d_b0, eq33_e1309_d_b1, eq33_e1309_d_b2, eq33_e1309_d_b3, eq33_e1309_d_b4, eq33_e1309_d_b5, eq33_e1309_d_b6, eq33_e1309_d_b7, eq33_e1309_d_b8];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(5),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let eq34_e1312: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, s.v[1244]);
        let eq34_e1313: f64 = (s.v[36] * eq34_e1312);
        let eq34_e1313_d_n0: f64 = (s.v[36] * (s.dn[1244][0] * ddt_scale));
        let eq34_e1313_d_n1: f64 = (s.v[36] * (s.dn[1244][1] * ddt_scale));
        let eq34_e1313_d_n2: f64 = (s.v[36] * (s.dn[1244][2] * ddt_scale));
        let eq34_e1313_d_n3: f64 = (s.v[36] * (s.dn[1244][3] * ddt_scale));
        let eq34_e1313_d_n4: f64 = (s.v[36] * (s.dn[1244][4] * ddt_scale));
        let eq34_e1313_d_n5: f64 = (s.v[36] * (s.dn[1244][5] * ddt_scale));
        let eq34_e1313_d_n6: f64 = (s.v[36] * (s.dn[1244][6] * ddt_scale));
        let eq34_e1313_d_n7: f64 = (s.v[36] * (s.dn[1244][7] * ddt_scale));
        let eq34_e1313_d_n8: f64 = (s.v[36] * (s.dn[1244][8] * ddt_scale));
        let eq34_e1313_d_n9: f64 = (s.v[36] * (s.dn[1244][9] * ddt_scale));
        let eq34_e1313_d_n10: f64 = (s.v[36] * (s.dn[1244][10] * ddt_scale));
        let eq34_e1313_d_n11: f64 = (s.v[36] * (s.dn[1244][11] * ddt_scale));
        let eq34_e1313_d_n12: f64 = (s.v[36] * (s.dn[1244][12] * ddt_scale));
        let eq34_e1313_d_b0: f64 = (s.v[36] * (s.db[1244][0] * ddt_scale));
        let eq34_e1313_d_b1: f64 = (s.v[36] * (s.db[1244][1] * ddt_scale));
        let eq34_e1313_d_b2: f64 = (s.v[36] * (s.db[1244][2] * ddt_scale));
        let eq34_e1313_d_b3: f64 = (s.v[36] * (s.db[1244][3] * ddt_scale));
        let eq34_e1313_d_b4: f64 = (s.v[36] * (s.db[1244][4] * ddt_scale));
        let eq34_e1313_d_b5: f64 = (s.v[36] * (s.db[1244][5] * ddt_scale));
        let eq34_e1313_d_b6: f64 = (s.v[36] * (s.db[1244][6] * ddt_scale));
        let eq34_e1313_d_b7: f64 = (s.v[36] * (s.db[1244][7] * ddt_scale));
        let eq34_e1313_d_b8: f64 = (s.v[36] * (s.db[1244][8] * ddt_scale));
        let eq34_value: f64 = eq34_e1313;
        let eq34_node_derivatives: [f64; 13] = [eq34_e1313_d_n0, eq34_e1313_d_n1, eq34_e1313_d_n2, eq34_e1313_d_n3, eq34_e1313_d_n4, eq34_e1313_d_n5, eq34_e1313_d_n6, eq34_e1313_d_n7, eq34_e1313_d_n8, eq34_e1313_d_n9, eq34_e1313_d_n10, eq34_e1313_d_n11, eq34_e1313_d_n12];
        let eq34_branch_derivatives: [f64; 9] = [eq34_e1313_d_b0, eq34_e1313_d_b1, eq34_e1313_d_b2, eq34_e1313_d_b3, eq34_e1313_d_b4, eq34_e1313_d_b5, eq34_e1313_d_b6, eq34_e1313_d_b7, eq34_e1313_d_b8];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(7),
            multiplicity * (eq34_value),
            &eq34_node_derivatives,
            &eq34_branch_derivatives,
            multiplicity,
        );
        let eq35_e1316: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, s.v[1245]);
        let eq35_e1317: f64 = (s.v[36] * eq35_e1316);
        let eq35_e1317_d_n0: f64 = (s.v[36] * (s.dn[1245][0] * ddt_scale));
        let eq35_e1317_d_n1: f64 = (s.v[36] * (s.dn[1245][1] * ddt_scale));
        let eq35_e1317_d_n2: f64 = (s.v[36] * (s.dn[1245][2] * ddt_scale));
        let eq35_e1317_d_n3: f64 = (s.v[36] * (s.dn[1245][3] * ddt_scale));
        let eq35_e1317_d_n4: f64 = (s.v[36] * (s.dn[1245][4] * ddt_scale));
        let eq35_e1317_d_n5: f64 = (s.v[36] * (s.dn[1245][5] * ddt_scale));
        let eq35_e1317_d_n6: f64 = (s.v[36] * (s.dn[1245][6] * ddt_scale));
        let eq35_e1317_d_n7: f64 = (s.v[36] * (s.dn[1245][7] * ddt_scale));
        let eq35_e1317_d_n8: f64 = (s.v[36] * (s.dn[1245][8] * ddt_scale));
        let eq35_e1317_d_n9: f64 = (s.v[36] * (s.dn[1245][9] * ddt_scale));
        let eq35_e1317_d_n10: f64 = (s.v[36] * (s.dn[1245][10] * ddt_scale));
        let eq35_e1317_d_n11: f64 = (s.v[36] * (s.dn[1245][11] * ddt_scale));
        let eq35_e1317_d_n12: f64 = (s.v[36] * (s.dn[1245][12] * ddt_scale));
        let eq35_e1317_d_b0: f64 = (s.v[36] * (s.db[1245][0] * ddt_scale));
        let eq35_e1317_d_b1: f64 = (s.v[36] * (s.db[1245][1] * ddt_scale));
        let eq35_e1317_d_b2: f64 = (s.v[36] * (s.db[1245][2] * ddt_scale));
        let eq35_e1317_d_b3: f64 = (s.v[36] * (s.db[1245][3] * ddt_scale));
        let eq35_e1317_d_b4: f64 = (s.v[36] * (s.db[1245][4] * ddt_scale));
        let eq35_e1317_d_b5: f64 = (s.v[36] * (s.db[1245][5] * ddt_scale));
        let eq35_e1317_d_b6: f64 = (s.v[36] * (s.db[1245][6] * ddt_scale));
        let eq35_e1317_d_b7: f64 = (s.v[36] * (s.db[1245][7] * ddt_scale));
        let eq35_e1317_d_b8: f64 = (s.v[36] * (s.db[1245][8] * ddt_scale));
        let eq35_value: f64 = eq35_e1317;
        let eq35_node_derivatives: [f64; 13] = [eq35_e1317_d_n0, eq35_e1317_d_n1, eq35_e1317_d_n2, eq35_e1317_d_n3, eq35_e1317_d_n4, eq35_e1317_d_n5, eq35_e1317_d_n6, eq35_e1317_d_n7, eq35_e1317_d_n8, eq35_e1317_d_n9, eq35_e1317_d_n10, eq35_e1317_d_n11, eq35_e1317_d_n12];
        let eq35_branch_derivatives: [f64; 9] = [eq35_e1317_d_b0, eq35_e1317_d_b1, eq35_e1317_d_b2, eq35_e1317_d_b3, eq35_e1317_d_b4, eq35_e1317_d_b5, eq35_e1317_d_b6, eq35_e1317_d_b7, eq35_e1317_d_b8];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(8),
            multiplicity * (eq35_value),
            &eq35_node_derivatives,
            &eq35_branch_derivatives,
            multiplicity,
        );
        let (eq36_e1324, eq36_e1324_d_n0, eq36_e1324_d_n1, eq36_e1324_d_n2, eq36_e1324_d_n3, eq36_e1324_d_n4, eq36_e1324_d_n5, eq36_e1324_d_n6, eq36_e1324_d_n7, eq36_e1324_d_n8, eq36_e1324_d_n9, eq36_e1324_d_n10, eq36_e1324_d_n11, eq36_e1324_d_n12, eq36_e1324_d_b0, eq36_e1324_d_b1, eq36_e1324_d_b2, eq36_e1324_d_b3, eq36_e1324_d_b4, eq36_e1324_d_b5, eq36_e1324_d_b6, eq36_e1324_d_b7, eq36_e1324_d_b8,) = {
    if s.b[1863] {
        let eq36_e1321: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, s.v[1230]);
        let eq36_e1322: f64 = (s.v[36] * eq36_e1321);
        (eq36_e1322, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e1324;
        let eq36_node_derivatives: [f64; 13] = [eq36_e1324_d_n0, eq36_e1324_d_n1, eq36_e1324_d_n2, eq36_e1324_d_n3, eq36_e1324_d_n4, eq36_e1324_d_n5, eq36_e1324_d_n6, eq36_e1324_d_n7, eq36_e1324_d_n8, eq36_e1324_d_n9, eq36_e1324_d_n10, eq36_e1324_d_n11, eq36_e1324_d_n12];
        let eq36_branch_derivatives: [f64; 9] = [eq36_e1324_d_b0, eq36_e1324_d_b1, eq36_e1324_d_b2, eq36_e1324_d_b3, eq36_e1324_d_b4, eq36_e1324_d_b5, eq36_e1324_d_b6, eq36_e1324_d_b7, eq36_e1324_d_b8];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(7),
            multiplicity * (eq36_value),
            &eq36_node_derivatives,
            &eq36_branch_derivatives,
            multiplicity,
        );
        let (eq37_e1331, eq37_e1331_d_n0, eq37_e1331_d_n1, eq37_e1331_d_n2, eq37_e1331_d_n3, eq37_e1331_d_n4, eq37_e1331_d_n5, eq37_e1331_d_n6, eq37_e1331_d_n7, eq37_e1331_d_n8, eq37_e1331_d_n9, eq37_e1331_d_n10, eq37_e1331_d_n11, eq37_e1331_d_n12, eq37_e1331_d_b0, eq37_e1331_d_b1, eq37_e1331_d_b2, eq37_e1331_d_b3, eq37_e1331_d_b4, eq37_e1331_d_b5, eq37_e1331_d_b6, eq37_e1331_d_b7, eq37_e1331_d_b8,) = {
    if s.b[1863] {
        let eq37_e1328: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, s.v[1231]);
        let eq37_e1329: f64 = (s.v[36] * eq37_e1328);
        (eq37_e1329, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq37_value: f64 = eq37_e1331;
        let eq37_node_derivatives: [f64; 13] = [eq37_e1331_d_n0, eq37_e1331_d_n1, eq37_e1331_d_n2, eq37_e1331_d_n3, eq37_e1331_d_n4, eq37_e1331_d_n5, eq37_e1331_d_n6, eq37_e1331_d_n7, eq37_e1331_d_n8, eq37_e1331_d_n9, eq37_e1331_d_n10, eq37_e1331_d_n11, eq37_e1331_d_n12];
        let eq37_branch_derivatives: [f64; 9] = [eq37_e1331_d_b0, eq37_e1331_d_b1, eq37_e1331_d_b2, eq37_e1331_d_b3, eq37_e1331_d_b4, eq37_e1331_d_b5, eq37_e1331_d_b6, eq37_e1331_d_b7, eq37_e1331_d_b8];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(8),
            multiplicity * (eq37_value),
            &eq37_node_derivatives,
            &eq37_branch_derivatives,
            multiplicity,
        );
        let (eq38_e1338, eq38_e1338_d_n0, eq38_e1338_d_n1, eq38_e1338_d_n2, eq38_e1338_d_n3, eq38_e1338_d_n4, eq38_e1338_d_n5, eq38_e1338_d_n6, eq38_e1338_d_n7, eq38_e1338_d_n8, eq38_e1338_d_n9, eq38_e1338_d_n10, eq38_e1338_d_n11, eq38_e1338_d_n12, eq38_e1338_d_b0, eq38_e1338_d_b1, eq38_e1338_d_b2, eq38_e1338_d_b3, eq38_e1338_d_b4, eq38_e1338_d_b5, eq38_e1338_d_b6, eq38_e1338_d_b7, eq38_e1338_d_b8,) = {
    if s.b[1863] {
        let eq38_e1335: f64 = ((nv10 - nv3) * s.v[697]);
        let eq38_e1335_d_n0: f64 = ((nv10 - nv3) * s.dn[697][0]);
        let eq38_e1335_d_n1: f64 = ((nv10 - nv3) * s.dn[697][1]);
        let eq38_e1335_d_n2: f64 = ((nv10 - nv3) * s.dn[697][2]);
        let eq38_e1335_d_n3: f64 = ((-s.v[697]) + ((nv10 - nv3) * s.dn[697][3]));
        let eq38_e1335_d_n4: f64 = ((nv10 - nv3) * s.dn[697][4]);
        let eq38_e1335_d_n5: f64 = ((nv10 - nv3) * s.dn[697][5]);
        let eq38_e1335_d_n6: f64 = ((nv10 - nv3) * s.dn[697][6]);
        let eq38_e1335_d_n7: f64 = ((nv10 - nv3) * s.dn[697][7]);
        let eq38_e1335_d_n8: f64 = ((nv10 - nv3) * s.dn[697][8]);
        let eq38_e1335_d_n9: f64 = ((nv10 - nv3) * s.dn[697][9]);
        let eq38_e1335_d_n10: f64 = (s.v[697] + ((nv10 - nv3) * s.dn[697][10]));
        let eq38_e1335_d_n11: f64 = ((nv10 - nv3) * s.dn[697][11]);
        let eq38_e1335_d_n12: f64 = ((nv10 - nv3) * s.dn[697][12]);
        let eq38_e1335_d_b0: f64 = ((nv10 - nv3) * s.db[697][0]);
        let eq38_e1335_d_b1: f64 = ((nv10 - nv3) * s.db[697][1]);
        let eq38_e1335_d_b2: f64 = ((nv10 - nv3) * s.db[697][2]);
        let eq38_e1335_d_b3: f64 = ((nv10 - nv3) * s.db[697][3]);
        let eq38_e1335_d_b4: f64 = ((nv10 - nv3) * s.db[697][4]);
        let eq38_e1335_d_b5: f64 = ((nv10 - nv3) * s.db[697][5]);
        let eq38_e1335_d_b6: f64 = ((nv10 - nv3) * s.db[697][6]);
        let eq38_e1335_d_b7: f64 = ((nv10 - nv3) * s.db[697][7]);
        let eq38_e1335_d_b8: f64 = ((nv10 - nv3) * s.db[697][8]);
        let eq38_e1336: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, eq38_e1335);
        (eq38_e1336, (eq38_e1335_d_n0 * ddt_scale), (eq38_e1335_d_n1 * ddt_scale), (eq38_e1335_d_n2 * ddt_scale), (eq38_e1335_d_n3 * ddt_scale), (eq38_e1335_d_n4 * ddt_scale), (eq38_e1335_d_n5 * ddt_scale), (eq38_e1335_d_n6 * ddt_scale), (eq38_e1335_d_n7 * ddt_scale), (eq38_e1335_d_n8 * ddt_scale), (eq38_e1335_d_n9 * ddt_scale), (eq38_e1335_d_n10 * ddt_scale), (eq38_e1335_d_n11 * ddt_scale), (eq38_e1335_d_n12 * ddt_scale), (eq38_e1335_d_b0 * ddt_scale), (eq38_e1335_d_b1 * ddt_scale), (eq38_e1335_d_b2 * ddt_scale), (eq38_e1335_d_b3 * ddt_scale), (eq38_e1335_d_b4 * ddt_scale), (eq38_e1335_d_b5 * ddt_scale), (eq38_e1335_d_b6 * ddt_scale), (eq38_e1335_d_b7 * ddt_scale), (eq38_e1335_d_b8 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_value: f64 = eq38_e1338;
        let eq38_node_derivatives: [f64; 13] = [eq38_e1338_d_n0, eq38_e1338_d_n1, eq38_e1338_d_n2, eq38_e1338_d_n3, eq38_e1338_d_n4, eq38_e1338_d_n5, eq38_e1338_d_n6, eq38_e1338_d_n7, eq38_e1338_d_n8, eq38_e1338_d_n9, eq38_e1338_d_n10, eq38_e1338_d_n11, eq38_e1338_d_n12];
        let eq38_branch_derivatives: [f64; 9] = [eq38_e1338_d_b0, eq38_e1338_d_b1, eq38_e1338_d_b2, eq38_e1338_d_b3, eq38_e1338_d_b4, eq38_e1338_d_b5, eq38_e1338_d_b6, eq38_e1338_d_b7, eq38_e1338_d_b8];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(3),
            multiplicity * (eq38_value),
            &eq38_node_derivatives,
            &eq38_branch_derivatives,
            multiplicity,
        );
        let (eq39_e1346, eq39_e1346_d_n0, eq39_e1346_d_n1, eq39_e1346_d_n2, eq39_e1346_d_n3, eq39_e1346_d_n4, eq39_e1346_d_n5, eq39_e1346_d_n6, eq39_e1346_d_n7, eq39_e1346_d_n8, eq39_e1346_d_n9, eq39_e1346_d_n10, eq39_e1346_d_n11, eq39_e1346_d_n12, eq39_e1346_d_b0, eq39_e1346_d_b1, eq39_e1346_d_b2, eq39_e1346_d_b3, eq39_e1346_d_b4, eq39_e1346_d_b5, eq39_e1346_d_b6, eq39_e1346_d_b7, eq39_e1346_d_b8,) = {
    if (!s.b[1863]) {
        let eq39_e1343: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 9, s.v[1230]);
        let eq39_e1344: f64 = (s.v[36] * eq39_e1343);
        (eq39_e1344, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e1346;
        let eq39_node_derivatives: [f64; 13] = [eq39_e1346_d_n0, eq39_e1346_d_n1, eq39_e1346_d_n2, eq39_e1346_d_n3, eq39_e1346_d_n4, eq39_e1346_d_n5, eq39_e1346_d_n6, eq39_e1346_d_n7, eq39_e1346_d_n8, eq39_e1346_d_n9, eq39_e1346_d_n10, eq39_e1346_d_n11, eq39_e1346_d_n12];
        let eq39_branch_derivatives: [f64; 9] = [eq39_e1346_d_b0, eq39_e1346_d_b1, eq39_e1346_d_b2, eq39_e1346_d_b3, eq39_e1346_d_b4, eq39_e1346_d_b5, eq39_e1346_d_b6, eq39_e1346_d_b7, eq39_e1346_d_b8];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq39_value),
            &eq39_node_derivatives,
            &eq39_branch_derivatives,
            multiplicity,
        );
        let (eq40_e1354, eq40_e1354_d_n0, eq40_e1354_d_n1, eq40_e1354_d_n2, eq40_e1354_d_n3, eq40_e1354_d_n4, eq40_e1354_d_n5, eq40_e1354_d_n6, eq40_e1354_d_n7, eq40_e1354_d_n8, eq40_e1354_d_n9, eq40_e1354_d_n10, eq40_e1354_d_n11, eq40_e1354_d_n12, eq40_e1354_d_b0, eq40_e1354_d_b1, eq40_e1354_d_b2, eq40_e1354_d_b3, eq40_e1354_d_b4, eq40_e1354_d_b5, eq40_e1354_d_b6, eq40_e1354_d_b7, eq40_e1354_d_b8,) = {
    if (!s.b[1863]) {
        let eq40_e1351: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 10, s.v[1231]);
        let eq40_e1352: f64 = (s.v[36] * eq40_e1351);
        (eq40_e1352, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_value: f64 = eq40_e1354;
        let eq40_node_derivatives: [f64; 13] = [eq40_e1354_d_n0, eq40_e1354_d_n1, eq40_e1354_d_n2, eq40_e1354_d_n3, eq40_e1354_d_n4, eq40_e1354_d_n5, eq40_e1354_d_n6, eq40_e1354_d_n7, eq40_e1354_d_n8, eq40_e1354_d_n9, eq40_e1354_d_n10, eq40_e1354_d_n11, eq40_e1354_d_n12];
        let eq40_branch_derivatives: [f64; 9] = [eq40_e1354_d_b0, eq40_e1354_d_b1, eq40_e1354_d_b2, eq40_e1354_d_b3, eq40_e1354_d_b4, eq40_e1354_d_b5, eq40_e1354_d_b6, eq40_e1354_d_b7, eq40_e1354_d_b8];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq40_value),
            &eq40_node_derivatives,
            &eq40_branch_derivatives,
            multiplicity,
        );
        let (eq41_e1362, eq41_e1362_d_n0, eq41_e1362_d_n1, eq41_e1362_d_n2, eq41_e1362_d_n3, eq41_e1362_d_n4, eq41_e1362_d_n5, eq41_e1362_d_n6, eq41_e1362_d_n7, eq41_e1362_d_n8, eq41_e1362_d_n9, eq41_e1362_d_n10, eq41_e1362_d_n11, eq41_e1362_d_n12, eq41_e1362_d_b0, eq41_e1362_d_b1, eq41_e1362_d_b2, eq41_e1362_d_b3, eq41_e1362_d_b4, eq41_e1362_d_b5, eq41_e1362_d_b6, eq41_e1362_d_b7, eq41_e1362_d_b8,) = {
    if (!s.b[1863]) {
        let eq41_e1359: f64 = ((nv9 - nv3) * s.v[697]);
        let eq41_e1359_d_n0: f64 = ((nv9 - nv3) * s.dn[697][0]);
        let eq41_e1359_d_n1: f64 = ((nv9 - nv3) * s.dn[697][1]);
        let eq41_e1359_d_n2: f64 = ((nv9 - nv3) * s.dn[697][2]);
        let eq41_e1359_d_n3: f64 = ((-s.v[697]) + ((nv9 - nv3) * s.dn[697][3]));
        let eq41_e1359_d_n4: f64 = ((nv9 - nv3) * s.dn[697][4]);
        let eq41_e1359_d_n5: f64 = ((nv9 - nv3) * s.dn[697][5]);
        let eq41_e1359_d_n6: f64 = ((nv9 - nv3) * s.dn[697][6]);
        let eq41_e1359_d_n7: f64 = ((nv9 - nv3) * s.dn[697][7]);
        let eq41_e1359_d_n8: f64 = ((nv9 - nv3) * s.dn[697][8]);
        let eq41_e1359_d_n9: f64 = (s.v[697] + ((nv9 - nv3) * s.dn[697][9]));
        let eq41_e1359_d_n10: f64 = ((nv9 - nv3) * s.dn[697][10]);
        let eq41_e1359_d_n11: f64 = ((nv9 - nv3) * s.dn[697][11]);
        let eq41_e1359_d_n12: f64 = ((nv9 - nv3) * s.dn[697][12]);
        let eq41_e1359_d_b0: f64 = ((nv9 - nv3) * s.db[697][0]);
        let eq41_e1359_d_b1: f64 = ((nv9 - nv3) * s.db[697][1]);
        let eq41_e1359_d_b2: f64 = ((nv9 - nv3) * s.db[697][2]);
        let eq41_e1359_d_b3: f64 = ((nv9 - nv3) * s.db[697][3]);
        let eq41_e1359_d_b4: f64 = ((nv9 - nv3) * s.db[697][4]);
        let eq41_e1359_d_b5: f64 = ((nv9 - nv3) * s.db[697][5]);
        let eq41_e1359_d_b6: f64 = ((nv9 - nv3) * s.db[697][6]);
        let eq41_e1359_d_b7: f64 = ((nv9 - nv3) * s.db[697][7]);
        let eq41_e1359_d_b8: f64 = ((nv9 - nv3) * s.db[697][8]);
        let eq41_e1360: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 11, eq41_e1359);
        (eq41_e1360, (eq41_e1359_d_n0 * ddt_scale), (eq41_e1359_d_n1 * ddt_scale), (eq41_e1359_d_n2 * ddt_scale), (eq41_e1359_d_n3 * ddt_scale), (eq41_e1359_d_n4 * ddt_scale), (eq41_e1359_d_n5 * ddt_scale), (eq41_e1359_d_n6 * ddt_scale), (eq41_e1359_d_n7 * ddt_scale), (eq41_e1359_d_n8 * ddt_scale), (eq41_e1359_d_n9 * ddt_scale), (eq41_e1359_d_n10 * ddt_scale), (eq41_e1359_d_n11 * ddt_scale), (eq41_e1359_d_n12 * ddt_scale), (eq41_e1359_d_b0 * ddt_scale), (eq41_e1359_d_b1 * ddt_scale), (eq41_e1359_d_b2 * ddt_scale), (eq41_e1359_d_b3 * ddt_scale), (eq41_e1359_d_b4 * ddt_scale), (eq41_e1359_d_b5 * ddt_scale), (eq41_e1359_d_b6 * ddt_scale), (eq41_e1359_d_b7 * ddt_scale), (eq41_e1359_d_b8 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq41_value: f64 = eq41_e1362;
        let eq41_node_derivatives: [f64; 13] = [eq41_e1362_d_n0, eq41_e1362_d_n1, eq41_e1362_d_n2, eq41_e1362_d_n3, eq41_e1362_d_n4, eq41_e1362_d_n5, eq41_e1362_d_n6, eq41_e1362_d_n7, eq41_e1362_d_n8, eq41_e1362_d_n9, eq41_e1362_d_n10, eq41_e1362_d_n11, eq41_e1362_d_n12];
        let eq41_branch_derivatives: [f64; 9] = [eq41_e1362_d_b0, eq41_e1362_d_b1, eq41_e1362_d_b2, eq41_e1362_d_b3, eq41_e1362_d_b4, eq41_e1362_d_b5, eq41_e1362_d_b6, eq41_e1362_d_b7, eq41_e1362_d_b8];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(3),
            multiplicity * (eq41_value),
            &eq41_node_derivatives,
            &eq41_branch_derivatives,
            multiplicity,
        );
        let eq42_e1364: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 12, s.v[449]);
        let eq42_value: f64 = eq42_e1364;
        let eq42_node_derivatives: [f64; 13] = [(s.dn[449][0] * ddt_scale), (s.dn[449][1] * ddt_scale), (s.dn[449][2] * ddt_scale), (s.dn[449][3] * ddt_scale), (s.dn[449][4] * ddt_scale), (s.dn[449][5] * ddt_scale), (s.dn[449][6] * ddt_scale), (s.dn[449][7] * ddt_scale), (s.dn[449][8] * ddt_scale), (s.dn[449][9] * ddt_scale), (s.dn[449][10] * ddt_scale), (s.dn[449][11] * ddt_scale), (s.dn[449][12] * ddt_scale)];
        let eq42_branch_derivatives: [f64; 9] = [(s.db[449][0] * ddt_scale), (s.db[449][1] * ddt_scale), (s.db[449][2] * ddt_scale), (s.db[449][3] * ddt_scale), (s.db[449][4] * ddt_scale), (s.db[449][5] * ddt_scale), (s.db[449][6] * ddt_scale), (s.db[449][7] * ddt_scale), (s.db[449][8] * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(3),
            multiplicity * (eq42_value),
            &eq42_node_derivatives,
            &eq42_branch_derivatives,
            multiplicity,
        );
        let eq43_e1366: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 13, s.v[448]);
        let eq43_value: f64 = eq43_e1366;
        let eq43_node_derivatives: [f64; 13] = [(s.dn[448][0] * ddt_scale), (s.dn[448][1] * ddt_scale), (s.dn[448][2] * ddt_scale), (s.dn[448][3] * ddt_scale), (s.dn[448][4] * ddt_scale), (s.dn[448][5] * ddt_scale), (s.dn[448][6] * ddt_scale), (s.dn[448][7] * ddt_scale), (s.dn[448][8] * ddt_scale), (s.dn[448][9] * ddt_scale), (s.dn[448][10] * ddt_scale), (s.dn[448][11] * ddt_scale), (s.dn[448][12] * ddt_scale)];
        let eq43_branch_derivatives: [f64; 9] = [(s.db[448][0] * ddt_scale), (s.db[448][1] * ddt_scale), (s.db[448][2] * ddt_scale), (s.db[448][3] * ddt_scale), (s.db[448][4] * ddt_scale), (s.db[448][5] * ddt_scale), (s.db[448][6] * ddt_scale), (s.db[448][7] * ddt_scale), (s.db[448][8] * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(3),
            multiplicity * (eq43_value),
            &eq43_node_derivatives,
            &eq43_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq48_e1398, eq48_e1398_d_n0, eq48_e1398_d_n1, eq48_e1398_d_n2, eq48_e1398_d_n3, eq48_e1398_d_n4, eq48_e1398_d_n5, eq48_e1398_d_n6, eq48_e1398_d_n7, eq48_e1398_d_n8, eq48_e1398_d_n9, eq48_e1398_d_n10, eq48_e1398_d_n11, eq48_e1398_d_n12, eq48_e1398_d_b0, eq48_e1398_d_b1, eq48_e1398_d_b2, eq48_e1398_d_b3, eq48_e1398_d_b4, eq48_e1398_d_b5, eq48_e1398_d_b6, eq48_e1398_d_b7, eq48_e1398_d_b8,) = {
    if (!s.b[1865]) {
        let eq48_e1396: f64 = ((nv10 - nv9) * s.v[413]);
        let eq48_e1396_d_n0: f64 = ((nv10 - nv9) * s.dn[413][0]);
        let eq48_e1396_d_n1: f64 = ((nv10 - nv9) * s.dn[413][1]);
        let eq48_e1396_d_n2: f64 = ((nv10 - nv9) * s.dn[413][2]);
        let eq48_e1396_d_n3: f64 = ((nv10 - nv9) * s.dn[413][3]);
        let eq48_e1396_d_n4: f64 = ((nv10 - nv9) * s.dn[413][4]);
        let eq48_e1396_d_n5: f64 = ((nv10 - nv9) * s.dn[413][5]);
        let eq48_e1396_d_n6: f64 = ((nv10 - nv9) * s.dn[413][6]);
        let eq48_e1396_d_n7: f64 = ((nv10 - nv9) * s.dn[413][7]);
        let eq48_e1396_d_n8: f64 = ((nv10 - nv9) * s.dn[413][8]);
        let eq48_e1396_d_n9: f64 = ((-s.v[413]) + ((nv10 - nv9) * s.dn[413][9]));
        let eq48_e1396_d_n10: f64 = (s.v[413] + ((nv10 - nv9) * s.dn[413][10]));
        let eq48_e1396_d_n11: f64 = ((nv10 - nv9) * s.dn[413][11]);
        let eq48_e1396_d_n12: f64 = ((nv10 - nv9) * s.dn[413][12]);
        let eq48_e1396_d_b0: f64 = ((nv10 - nv9) * s.db[413][0]);
        let eq48_e1396_d_b1: f64 = ((nv10 - nv9) * s.db[413][1]);
        let eq48_e1396_d_b2: f64 = ((nv10 - nv9) * s.db[413][2]);
        let eq48_e1396_d_b3: f64 = ((nv10 - nv9) * s.db[413][3]);
        let eq48_e1396_d_b4: f64 = ((nv10 - nv9) * s.db[413][4]);
        let eq48_e1396_d_b5: f64 = ((nv10 - nv9) * s.db[413][5]);
        let eq48_e1396_d_b6: f64 = ((nv10 - nv9) * s.db[413][6]);
        let eq48_e1396_d_b7: f64 = ((nv10 - nv9) * s.db[413][7]);
        let eq48_e1396_d_b8: f64 = ((nv10 - nv9) * s.db[413][8]);
        (eq48_e1396, eq48_e1396_d_n0, eq48_e1396_d_n1, eq48_e1396_d_n2, eq48_e1396_d_n3, eq48_e1396_d_n4, eq48_e1396_d_n5, eq48_e1396_d_n6, eq48_e1396_d_n7, eq48_e1396_d_n8, eq48_e1396_d_n9, eq48_e1396_d_n10, eq48_e1396_d_n11, eq48_e1396_d_n12, eq48_e1396_d_b0, eq48_e1396_d_b1, eq48_e1396_d_b2, eq48_e1396_d_b3, eq48_e1396_d_b4, eq48_e1396_d_b5, eq48_e1396_d_b6, eq48_e1396_d_b7, eq48_e1396_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq48_value: f64 = eq48_e1398;
        let eq48_node_derivatives: [f64; 13] = [eq48_e1398_d_n0, eq48_e1398_d_n1, eq48_e1398_d_n2, eq48_e1398_d_n3, eq48_e1398_d_n4, eq48_e1398_d_n5, eq48_e1398_d_n6, eq48_e1398_d_n7, eq48_e1398_d_n8, eq48_e1398_d_n9, eq48_e1398_d_n10, eq48_e1398_d_n11, eq48_e1398_d_n12];
        let eq48_branch_derivatives: [f64; 9] = [eq48_e1398_d_b0, eq48_e1398_d_b1, eq48_e1398_d_b2, eq48_e1398_d_b3, eq48_e1398_d_b4, eq48_e1398_d_b5, eq48_e1398_d_b6, eq48_e1398_d_b7, eq48_e1398_d_b8];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(9),
            multiplicity * (eq48_value),
            &eq48_node_derivatives,
            &eq48_branch_derivatives,
            multiplicity,
        );
        let (eq57_e1469, eq57_e1469_d_n0, eq57_e1469_d_n1, eq57_e1469_d_n2, eq57_e1469_d_n3, eq57_e1469_d_n4, eq57_e1469_d_n5, eq57_e1469_d_n6, eq57_e1469_d_n7, eq57_e1469_d_n8, eq57_e1469_d_n9, eq57_e1469_d_n10, eq57_e1469_d_n11, eq57_e1469_d_n12, eq57_e1469_d_b0, eq57_e1469_d_b1, eq57_e1469_d_b2, eq57_e1469_d_b3, eq57_e1469_d_b4, eq57_e1469_d_b5, eq57_e1469_d_b6, eq57_e1469_d_b7, eq57_e1469_d_b8,) = {
    if s.b[1869] {
        let eq57_e1461: f64 = (-s.v[1220]);
        let eq57_e1463: f64 = (eq57_e1461 * s.v[1158]);
        let eq57_e1463_d_n0: f64 = (((-s.dn[1220][0]) * s.v[1158]) + (eq57_e1461 * s.dn[1158][0]));
        let eq57_e1463_d_n1: f64 = (((-s.dn[1220][1]) * s.v[1158]) + (eq57_e1461 * s.dn[1158][1]));
        let eq57_e1463_d_n2: f64 = (((-s.dn[1220][2]) * s.v[1158]) + (eq57_e1461 * s.dn[1158][2]));
        let eq57_e1463_d_n3: f64 = (((-s.dn[1220][3]) * s.v[1158]) + (eq57_e1461 * s.dn[1158][3]));
        let eq57_e1463_d_n4: f64 = (((-s.dn[1220][4]) * s.v[1158]) + (eq57_e1461 * s.dn[1158][4]));
        let eq57_e1463_d_n5: f64 = (((-s.dn[1220][5]) * s.v[1158]) + (eq57_e1461 * s.dn[1158][5]));
        let eq57_e1463_d_n6: f64 = (((-s.dn[1220][6]) * s.v[1158]) + (eq57_e1461 * s.dn[1158][6]));
        let eq57_e1463_d_n7: f64 = (((-s.dn[1220][7]) * s.v[1158]) + (eq57_e1461 * s.dn[1158][7]));
        let eq57_e1463_d_n8: f64 = (((-s.dn[1220][8]) * s.v[1158]) + (eq57_e1461 * s.dn[1158][8]));
        let eq57_e1463_d_n9: f64 = (((-s.dn[1220][9]) * s.v[1158]) + (eq57_e1461 * s.dn[1158][9]));
        let eq57_e1463_d_n10: f64 = (((-s.dn[1220][10]) * s.v[1158]) + (eq57_e1461 * s.dn[1158][10]));
        let eq57_e1463_d_n11: f64 = (((-s.dn[1220][11]) * s.v[1158]) + (eq57_e1461 * s.dn[1158][11]));
        let eq57_e1463_d_n12: f64 = (((-s.dn[1220][12]) * s.v[1158]) + (eq57_e1461 * s.dn[1158][12]));
        let eq57_e1463_d_b0: f64 = (((-s.db[1220][0]) * s.v[1158]) + (eq57_e1461 * s.db[1158][0]));
        let eq57_e1463_d_b1: f64 = (((-s.db[1220][1]) * s.v[1158]) + (eq57_e1461 * s.db[1158][1]));
        let eq57_e1463_d_b2: f64 = (((-s.db[1220][2]) * s.v[1158]) + (eq57_e1461 * s.db[1158][2]));
        let eq57_e1463_d_b3: f64 = (((-s.db[1220][3]) * s.v[1158]) + (eq57_e1461 * s.db[1158][3]));
        let eq57_e1463_d_b4: f64 = (((-s.db[1220][4]) * s.v[1158]) + (eq57_e1461 * s.db[1158][4]));
        let eq57_e1463_d_b5: f64 = (((-s.db[1220][5]) * s.v[1158]) + (eq57_e1461 * s.db[1158][5]));
        let eq57_e1463_d_b6: f64 = (((-s.db[1220][6]) * s.v[1158]) + (eq57_e1461 * s.db[1158][6]));
        let eq57_e1463_d_b7: f64 = (((-s.db[1220][7]) * s.v[1158]) + (eq57_e1461 * s.db[1158][7]));
        let eq57_e1463_d_b8: f64 = (((-s.db[1220][8]) * s.v[1158]) + (eq57_e1461 * s.db[1158][8]));
        let __rspice_inv_cse_0: f64 = 1.0 / s.v[527];
        let eq57_e1466: f64 = (s.v[770] * __rspice_inv_cse_0);
        let eq57_e1466_d_n0: f64 = (s.dn[770][0] * __rspice_inv_cse_0);
        let eq57_e1466_d_n1: f64 = (s.dn[770][1] * __rspice_inv_cse_0);
        let eq57_e1466_d_n2: f64 = (s.dn[770][2] * __rspice_inv_cse_0);
        let eq57_e1466_d_n3: f64 = (s.dn[770][3] * __rspice_inv_cse_0);
        let eq57_e1466_d_n4: f64 = (s.dn[770][4] * __rspice_inv_cse_0);
        let eq57_e1466_d_n5: f64 = (s.dn[770][5] * __rspice_inv_cse_0);
        let eq57_e1466_d_n6: f64 = (s.dn[770][6] * __rspice_inv_cse_0);
        let eq57_e1466_d_n7: f64 = (s.dn[770][7] * __rspice_inv_cse_0);
        let eq57_e1466_d_n8: f64 = (s.dn[770][8] * __rspice_inv_cse_0);
        let eq57_e1466_d_n9: f64 = (s.dn[770][9] * __rspice_inv_cse_0);
        let eq57_e1466_d_n10: f64 = (s.dn[770][10] * __rspice_inv_cse_0);
        let eq57_e1466_d_n11: f64 = (s.dn[770][11] * __rspice_inv_cse_0);
        let eq57_e1466_d_n12: f64 = (s.dn[770][12] * __rspice_inv_cse_0);
        let eq57_e1466_d_b0: f64 = (s.db[770][0] * __rspice_inv_cse_0);
        let eq57_e1466_d_b1: f64 = (s.db[770][1] * __rspice_inv_cse_0);
        let eq57_e1466_d_b2: f64 = (s.db[770][2] * __rspice_inv_cse_0);
        let eq57_e1466_d_b3: f64 = (s.db[770][3] * __rspice_inv_cse_0);
        let eq57_e1466_d_b4: f64 = (s.db[770][4] * __rspice_inv_cse_0);
        let eq57_e1466_d_b5: f64 = (s.db[770][5] * __rspice_inv_cse_0);
        let eq57_e1466_d_b6: f64 = (s.db[770][6] * __rspice_inv_cse_0);
        let eq57_e1466_d_b7: f64 = (s.db[770][7] * __rspice_inv_cse_0);
        let eq57_e1466_d_b8: f64 = (s.db[770][8] * __rspice_inv_cse_0);
        let eq57_e1467: f64 = (eq57_e1463 + eq57_e1466);
        let eq57_e1467_d_n0: f64 = (eq57_e1463_d_n0 + eq57_e1466_d_n0);
        let eq57_e1467_d_n1: f64 = (eq57_e1463_d_n1 + eq57_e1466_d_n1);
        let eq57_e1467_d_n2: f64 = (eq57_e1463_d_n2 + eq57_e1466_d_n2);
        let eq57_e1467_d_n3: f64 = (eq57_e1463_d_n3 + eq57_e1466_d_n3);
        let eq57_e1467_d_n4: f64 = (eq57_e1463_d_n4 + eq57_e1466_d_n4);
        let eq57_e1467_d_n5: f64 = (eq57_e1463_d_n5 + eq57_e1466_d_n5);
        let eq57_e1467_d_n6: f64 = (eq57_e1463_d_n6 + eq57_e1466_d_n6);
        let eq57_e1467_d_n7: f64 = (eq57_e1463_d_n7 + eq57_e1466_d_n7);
        let eq57_e1467_d_n8: f64 = (eq57_e1463_d_n8 + eq57_e1466_d_n8);
        let eq57_e1467_d_n9: f64 = (eq57_e1463_d_n9 + eq57_e1466_d_n9);
        let eq57_e1467_d_n10: f64 = (eq57_e1463_d_n10 + eq57_e1466_d_n10);
        let eq57_e1467_d_n11: f64 = (eq57_e1463_d_n11 + eq57_e1466_d_n11);
        let eq57_e1467_d_n12: f64 = (eq57_e1463_d_n12 + eq57_e1466_d_n12);
        let eq57_e1467_d_b0: f64 = (eq57_e1463_d_b0 + eq57_e1466_d_b0);
        let eq57_e1467_d_b1: f64 = (eq57_e1463_d_b1 + eq57_e1466_d_b1);
        let eq57_e1467_d_b2: f64 = (eq57_e1463_d_b2 + eq57_e1466_d_b2);
        let eq57_e1467_d_b3: f64 = (eq57_e1463_d_b3 + eq57_e1466_d_b3);
        let eq57_e1467_d_b4: f64 = (eq57_e1463_d_b4 + eq57_e1466_d_b4);
        let eq57_e1467_d_b5: f64 = (eq57_e1463_d_b5 + eq57_e1466_d_b5);
        let eq57_e1467_d_b6: f64 = (eq57_e1463_d_b6 + eq57_e1466_d_b6);
        let eq57_e1467_d_b7: f64 = (eq57_e1463_d_b7 + eq57_e1466_d_b7);
        let eq57_e1467_d_b8: f64 = (eq57_e1463_d_b8 + eq57_e1466_d_b8);
        (eq57_e1467, eq57_e1467_d_n0, eq57_e1467_d_n1, eq57_e1467_d_n2, eq57_e1467_d_n3, eq57_e1467_d_n4, eq57_e1467_d_n5, eq57_e1467_d_n6, eq57_e1467_d_n7, eq57_e1467_d_n8, eq57_e1467_d_n9, eq57_e1467_d_n10, eq57_e1467_d_n11, eq57_e1467_d_n12, eq57_e1467_d_b0, eq57_e1467_d_b1, eq57_e1467_d_b2, eq57_e1467_d_b3, eq57_e1467_d_b4, eq57_e1467_d_b5, eq57_e1467_d_b6, eq57_e1467_d_b7, eq57_e1467_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e1469;
        let eq57_node_derivatives: [f64; 13] = [eq57_e1469_d_n0, eq57_e1469_d_n1, eq57_e1469_d_n2, eq57_e1469_d_n3, eq57_e1469_d_n4, eq57_e1469_d_n5, eq57_e1469_d_n6, eq57_e1469_d_n7, eq57_e1469_d_n8, eq57_e1469_d_n9, eq57_e1469_d_n10, eq57_e1469_d_n11, eq57_e1469_d_n12];
        let eq57_branch_derivatives: [f64; 9] = [eq57_e1469_d_b0, eq57_e1469_d_b1, eq57_e1469_d_b2, eq57_e1469_d_b3, eq57_e1469_d_b4, eq57_e1469_d_b5, eq57_e1469_d_b6, eq57_e1469_d_b7, eq57_e1469_d_b8];
        stamper.stamp_current_dense_local(
            Some(6),
            None,
            multiplicity * (eq57_value),
            &eq57_node_derivatives,
            &eq57_branch_derivatives,
            multiplicity,
        );
        let (eq58_e1476, eq58_e1476_d_n0, eq58_e1476_d_n1, eq58_e1476_d_n2, eq58_e1476_d_n3, eq58_e1476_d_n4, eq58_e1476_d_n5, eq58_e1476_d_n6, eq58_e1476_d_n7, eq58_e1476_d_n8, eq58_e1476_d_n9, eq58_e1476_d_n10, eq58_e1476_d_n11, eq58_e1476_d_n12, eq58_e1476_d_b0, eq58_e1476_d_b1, eq58_e1476_d_b2, eq58_e1476_d_b3, eq58_e1476_d_b4, eq58_e1476_d_b5, eq58_e1476_d_b6, eq58_e1476_d_b7, eq58_e1476_d_b8,) = {
    if s.b[1869] {
        let eq58_e1473: f64 = (s.v[770] * s.v[528]);
        let eq58_e1473_d_n0: f64 = (s.dn[770][0] * s.v[528]);
        let eq58_e1473_d_n1: f64 = (s.dn[770][1] * s.v[528]);
        let eq58_e1473_d_n2: f64 = (s.dn[770][2] * s.v[528]);
        let eq58_e1473_d_n3: f64 = (s.dn[770][3] * s.v[528]);
        let eq58_e1473_d_n4: f64 = (s.dn[770][4] * s.v[528]);
        let eq58_e1473_d_n5: f64 = (s.dn[770][5] * s.v[528]);
        let eq58_e1473_d_n6: f64 = (s.dn[770][6] * s.v[528]);
        let eq58_e1473_d_n7: f64 = (s.dn[770][7] * s.v[528]);
        let eq58_e1473_d_n8: f64 = (s.dn[770][8] * s.v[528]);
        let eq58_e1473_d_n9: f64 = (s.dn[770][9] * s.v[528]);
        let eq58_e1473_d_n10: f64 = (s.dn[770][10] * s.v[528]);
        let eq58_e1473_d_n11: f64 = (s.dn[770][11] * s.v[528]);
        let eq58_e1473_d_n12: f64 = (s.dn[770][12] * s.v[528]);
        let eq58_e1473_d_b0: f64 = (s.db[770][0] * s.v[528]);
        let eq58_e1473_d_b1: f64 = (s.db[770][1] * s.v[528]);
        let eq58_e1473_d_b2: f64 = (s.db[770][2] * s.v[528]);
        let eq58_e1473_d_b3: f64 = (s.db[770][3] * s.v[528]);
        let eq58_e1473_d_b4: f64 = (s.db[770][4] * s.v[528]);
        let eq58_e1473_d_b5: f64 = (s.db[770][5] * s.v[528]);
        let eq58_e1473_d_b6: f64 = (s.db[770][6] * s.v[528]);
        let eq58_e1473_d_b7: f64 = (s.db[770][7] * s.v[528]);
        let eq58_e1473_d_b8: f64 = (s.db[770][8] * s.v[528]);
        let eq58_e1474: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 14, eq58_e1473);
        (eq58_e1474, (eq58_e1473_d_n0 * ddt_scale), (eq58_e1473_d_n1 * ddt_scale), (eq58_e1473_d_n2 * ddt_scale), (eq58_e1473_d_n3 * ddt_scale), (eq58_e1473_d_n4 * ddt_scale), (eq58_e1473_d_n5 * ddt_scale), (eq58_e1473_d_n6 * ddt_scale), (eq58_e1473_d_n7 * ddt_scale), (eq58_e1473_d_n8 * ddt_scale), (eq58_e1473_d_n9 * ddt_scale), (eq58_e1473_d_n10 * ddt_scale), (eq58_e1473_d_n11 * ddt_scale), (eq58_e1473_d_n12 * ddt_scale), (eq58_e1473_d_b0 * ddt_scale), (eq58_e1473_d_b1 * ddt_scale), (eq58_e1473_d_b2 * ddt_scale), (eq58_e1473_d_b3 * ddt_scale), (eq58_e1473_d_b4 * ddt_scale), (eq58_e1473_d_b5 * ddt_scale), (eq58_e1473_d_b6 * ddt_scale), (eq58_e1473_d_b7 * ddt_scale), (eq58_e1473_d_b8 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq58_value: f64 = eq58_e1476;
        let eq58_node_derivatives: [f64; 13] = [eq58_e1476_d_n0, eq58_e1476_d_n1, eq58_e1476_d_n2, eq58_e1476_d_n3, eq58_e1476_d_n4, eq58_e1476_d_n5, eq58_e1476_d_n6, eq58_e1476_d_n7, eq58_e1476_d_n8, eq58_e1476_d_n9, eq58_e1476_d_n10, eq58_e1476_d_n11, eq58_e1476_d_n12];
        let eq58_branch_derivatives: [f64; 9] = [eq58_e1476_d_b0, eq58_e1476_d_b1, eq58_e1476_d_b2, eq58_e1476_d_b3, eq58_e1476_d_b4, eq58_e1476_d_b5, eq58_e1476_d_b6, eq58_e1476_d_b7, eq58_e1476_d_b8];
        stamper.stamp_current_dense_local(
            Some(6),
            None,
            multiplicity * (eq58_value),
            &eq58_node_derivatives,
            &eq58_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let __rspice_deriv_cse_0: f64 = (s.v[36] * s.dn[1230][0]);
        let __rspice_deriv_cse_1: f64 = (s.v[36] * s.dn[1230][1]);
        let __rspice_deriv_cse_2: f64 = (s.v[36] * s.dn[1230][2]);
        let __rspice_deriv_cse_3: f64 = (s.v[36] * s.dn[1230][3]);
        let __rspice_deriv_cse_4: f64 = (s.v[36] * s.dn[1230][4]);
        let __rspice_deriv_cse_5: f64 = (s.v[36] * s.dn[1230][5]);
        let __rspice_deriv_cse_6: f64 = (s.v[36] * s.dn[1230][6]);
        let __rspice_deriv_cse_7: f64 = (s.v[36] * s.dn[1230][7]);
        let __rspice_deriv_cse_8: f64 = (s.v[36] * s.dn[1230][8]);
        let __rspice_deriv_cse_9: f64 = (s.v[36] * s.dn[1230][9]);
        let __rspice_deriv_cse_10: f64 = (s.v[36] * s.dn[1230][10]);
        let __rspice_deriv_cse_11: f64 = (s.v[36] * s.dn[1230][11]);
        let __rspice_deriv_cse_12: f64 = (s.v[36] * s.dn[1230][12]);
        let __rspice_deriv_cse_13: f64 = (s.v[36] * s.db[1230][0]);
        let __rspice_deriv_cse_14: f64 = (s.v[36] * s.db[1230][1]);
        let __rspice_deriv_cse_15: f64 = (s.v[36] * s.db[1230][2]);
        let __rspice_deriv_cse_16: f64 = (s.v[36] * s.db[1230][3]);
        let __rspice_deriv_cse_17: f64 = (s.v[36] * s.db[1230][4]);
        let __rspice_deriv_cse_18: f64 = (s.v[36] * s.db[1230][5]);
        let __rspice_deriv_cse_19: f64 = (s.v[36] * s.db[1230][6]);
        let __rspice_deriv_cse_20: f64 = (s.v[36] * s.db[1230][7]);
        let __rspice_deriv_cse_21: f64 = (s.v[36] * s.db[1230][8]);
        let __rspice_deriv_cse_22: f64 = (s.v[36] * s.dn[1231][0]);
        let __rspice_deriv_cse_23: f64 = (s.v[36] * s.dn[1231][1]);
        let __rspice_deriv_cse_24: f64 = (s.v[36] * s.dn[1231][2]);
        let __rspice_deriv_cse_25: f64 = (s.v[36] * s.dn[1231][3]);
        let __rspice_deriv_cse_26: f64 = (s.v[36] * s.dn[1231][4]);
        let __rspice_deriv_cse_27: f64 = (s.v[36] * s.dn[1231][5]);
        let __rspice_deriv_cse_28: f64 = (s.v[36] * s.dn[1231][6]);
        let __rspice_deriv_cse_29: f64 = (s.v[36] * s.dn[1231][7]);
        let __rspice_deriv_cse_30: f64 = (s.v[36] * s.dn[1231][8]);
        let __rspice_deriv_cse_31: f64 = (s.v[36] * s.dn[1231][9]);
        let __rspice_deriv_cse_32: f64 = (s.v[36] * s.dn[1231][10]);
        let __rspice_deriv_cse_33: f64 = (s.v[36] * s.dn[1231][11]);
        let __rspice_deriv_cse_34: f64 = (s.v[36] * s.dn[1231][12]);
        let __rspice_deriv_cse_35: f64 = (s.v[36] * s.db[1231][0]);
        let __rspice_deriv_cse_36: f64 = (s.v[36] * s.db[1231][1]);
        let __rspice_deriv_cse_37: f64 = (s.v[36] * s.db[1231][2]);
        let __rspice_deriv_cse_38: f64 = (s.v[36] * s.db[1231][3]);
        let __rspice_deriv_cse_39: f64 = (s.v[36] * s.db[1231][4]);
        let __rspice_deriv_cse_40: f64 = (s.v[36] * s.db[1231][5]);
        let __rspice_deriv_cse_41: f64 = (s.v[36] * s.db[1231][6]);
        let __rspice_deriv_cse_42: f64 = (s.v[36] * s.db[1231][7]);
        let __rspice_deriv_cse_43: f64 = (s.v[36] * s.db[1231][8]);
        let eq30_e1299_q: f64 = s.v[446];
        let eq30_reactive_node_derivatives: [f64; 13] = [s.dn[446][0], s.dn[446][1], s.dn[446][2], s.dn[446][3], s.dn[446][4], s.dn[446][5], s.dn[446][6], s.dn[446][7], s.dn[446][8], s.dn[446][9], s.dn[446][10], s.dn[446][11], s.dn[446][12]];
        let eq30_reactive_branch_derivatives: [f64; 9] = [s.db[446][0], s.db[446][1], s.db[446][2], s.db[446][3], s.db[446][4], s.db[446][5], s.db[446][6], s.db[446][7], s.db[446][8]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes,
            &eq30_reactive_node_derivatives,
            branches,
            &eq30_reactive_branch_derivatives,
            multiplicity,
        );
        let eq31_e1301_q: f64 = s.v[447];
        let eq31_reactive_node_derivatives: [f64; 13] = [s.dn[447][0], s.dn[447][1], s.dn[447][2], s.dn[447][3], s.dn[447][4], s.dn[447][5], s.dn[447][6], s.dn[447][7], s.dn[447][8], s.dn[447][9], s.dn[447][10], s.dn[447][11], s.dn[447][12]];
        let eq31_reactive_branch_derivatives: [f64; 9] = [s.db[447][0], s.db[447][1], s.db[447][2], s.db[447][3], s.db[447][4], s.db[447][5], s.db[447][6], s.db[447][7], s.db[447][8]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes,
            &eq31_reactive_node_derivatives,
            branches,
            &eq31_reactive_branch_derivatives,
            multiplicity,
        );
        let eq32_e1304_q: f64 = s.v[1251];
        let eq32_e1305: f64 = (s.v[36] * s.v[1251]);
        let eq32_e1305_d_n0: f64 = (s.v[36] * s.dn[1251][0]);
        let eq32_e1305_d_n1: f64 = (s.v[36] * s.dn[1251][1]);
        let eq32_e1305_d_n2: f64 = (s.v[36] * s.dn[1251][2]);
        let eq32_e1305_d_n3: f64 = (s.v[36] * s.dn[1251][3]);
        let eq32_e1305_d_n4: f64 = (s.v[36] * s.dn[1251][4]);
        let eq32_e1305_d_n5: f64 = (s.v[36] * s.dn[1251][5]);
        let eq32_e1305_d_n6: f64 = (s.v[36] * s.dn[1251][6]);
        let eq32_e1305_d_n7: f64 = (s.v[36] * s.dn[1251][7]);
        let eq32_e1305_d_n8: f64 = (s.v[36] * s.dn[1251][8]);
        let eq32_e1305_d_n9: f64 = (s.v[36] * s.dn[1251][9]);
        let eq32_e1305_d_n10: f64 = (s.v[36] * s.dn[1251][10]);
        let eq32_e1305_d_n11: f64 = (s.v[36] * s.dn[1251][11]);
        let eq32_e1305_d_n12: f64 = (s.v[36] * s.dn[1251][12]);
        let eq32_e1305_d_b0: f64 = (s.v[36] * s.db[1251][0]);
        let eq32_e1305_d_b1: f64 = (s.v[36] * s.db[1251][1]);
        let eq32_e1305_d_b2: f64 = (s.v[36] * s.db[1251][2]);
        let eq32_e1305_d_b3: f64 = (s.v[36] * s.db[1251][3]);
        let eq32_e1305_d_b4: f64 = (s.v[36] * s.db[1251][4]);
        let eq32_e1305_d_b5: f64 = (s.v[36] * s.db[1251][5]);
        let eq32_e1305_d_b6: f64 = (s.v[36] * s.db[1251][6]);
        let eq32_e1305_d_b7: f64 = (s.v[36] * s.db[1251][7]);
        let eq32_e1305_d_b8: f64 = (s.v[36] * s.db[1251][8]);
        let eq32_e1305_q: f64 = (s.v[36] * eq32_e1304_q);
        let eq32_reactive_node_derivatives: [f64; 13] = [eq32_e1305_d_n0, eq32_e1305_d_n1, eq32_e1305_d_n2, eq32_e1305_d_n3, eq32_e1305_d_n4, eq32_e1305_d_n5, eq32_e1305_d_n6, eq32_e1305_d_n7, eq32_e1305_d_n8, eq32_e1305_d_n9, eq32_e1305_d_n10, eq32_e1305_d_n11, eq32_e1305_d_n12];
        let eq32_reactive_branch_derivatives: [f64; 9] = [eq32_e1305_d_b0, eq32_e1305_d_b1, eq32_e1305_d_b2, eq32_e1305_d_b3, eq32_e1305_d_b4, eq32_e1305_d_b5, eq32_e1305_d_b6, eq32_e1305_d_b7, eq32_e1305_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[5]),
            nodes,
            &eq32_reactive_node_derivatives,
            branches,
            &eq32_reactive_branch_derivatives,
            multiplicity,
        );
        let eq33_e1308_q: f64 = s.v[1255];
        let eq33_e1309: f64 = (s.v[36] * s.v[1255]);
        let eq33_e1309_d_n0: f64 = (s.v[36] * s.dn[1255][0]);
        let eq33_e1309_d_n1: f64 = (s.v[36] * s.dn[1255][1]);
        let eq33_e1309_d_n2: f64 = (s.v[36] * s.dn[1255][2]);
        let eq33_e1309_d_n3: f64 = (s.v[36] * s.dn[1255][3]);
        let eq33_e1309_d_n4: f64 = (s.v[36] * s.dn[1255][4]);
        let eq33_e1309_d_n5: f64 = (s.v[36] * s.dn[1255][5]);
        let eq33_e1309_d_n6: f64 = (s.v[36] * s.dn[1255][6]);
        let eq33_e1309_d_n7: f64 = (s.v[36] * s.dn[1255][7]);
        let eq33_e1309_d_n8: f64 = (s.v[36] * s.dn[1255][8]);
        let eq33_e1309_d_n9: f64 = (s.v[36] * s.dn[1255][9]);
        let eq33_e1309_d_n10: f64 = (s.v[36] * s.dn[1255][10]);
        let eq33_e1309_d_n11: f64 = (s.v[36] * s.dn[1255][11]);
        let eq33_e1309_d_n12: f64 = (s.v[36] * s.dn[1255][12]);
        let eq33_e1309_d_b0: f64 = (s.v[36] * s.db[1255][0]);
        let eq33_e1309_d_b1: f64 = (s.v[36] * s.db[1255][1]);
        let eq33_e1309_d_b2: f64 = (s.v[36] * s.db[1255][2]);
        let eq33_e1309_d_b3: f64 = (s.v[36] * s.db[1255][3]);
        let eq33_e1309_d_b4: f64 = (s.v[36] * s.db[1255][4]);
        let eq33_e1309_d_b5: f64 = (s.v[36] * s.db[1255][5]);
        let eq33_e1309_d_b6: f64 = (s.v[36] * s.db[1255][6]);
        let eq33_e1309_d_b7: f64 = (s.v[36] * s.db[1255][7]);
        let eq33_e1309_d_b8: f64 = (s.v[36] * s.db[1255][8]);
        let eq33_e1309_q: f64 = (s.v[36] * eq33_e1308_q);
        let eq33_reactive_node_derivatives: [f64; 13] = [eq33_e1309_d_n0, eq33_e1309_d_n1, eq33_e1309_d_n2, eq33_e1309_d_n3, eq33_e1309_d_n4, eq33_e1309_d_n5, eq33_e1309_d_n6, eq33_e1309_d_n7, eq33_e1309_d_n8, eq33_e1309_d_n9, eq33_e1309_d_n10, eq33_e1309_d_n11, eq33_e1309_d_n12];
        let eq33_reactive_branch_derivatives: [f64; 9] = [eq33_e1309_d_b0, eq33_e1309_d_b1, eq33_e1309_d_b2, eq33_e1309_d_b3, eq33_e1309_d_b4, eq33_e1309_d_b5, eq33_e1309_d_b6, eq33_e1309_d_b7, eq33_e1309_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[5]),
            nodes,
            &eq33_reactive_node_derivatives,
            branches,
            &eq33_reactive_branch_derivatives,
            multiplicity,
        );
        let eq34_e1312_q: f64 = s.v[1244];
        let eq34_e1313: f64 = (s.v[36] * s.v[1244]);
        let eq34_e1313_d_n0: f64 = (s.v[36] * s.dn[1244][0]);
        let eq34_e1313_d_n1: f64 = (s.v[36] * s.dn[1244][1]);
        let eq34_e1313_d_n2: f64 = (s.v[36] * s.dn[1244][2]);
        let eq34_e1313_d_n3: f64 = (s.v[36] * s.dn[1244][3]);
        let eq34_e1313_d_n4: f64 = (s.v[36] * s.dn[1244][4]);
        let eq34_e1313_d_n5: f64 = (s.v[36] * s.dn[1244][5]);
        let eq34_e1313_d_n6: f64 = (s.v[36] * s.dn[1244][6]);
        let eq34_e1313_d_n7: f64 = (s.v[36] * s.dn[1244][7]);
        let eq34_e1313_d_n8: f64 = (s.v[36] * s.dn[1244][8]);
        let eq34_e1313_d_n9: f64 = (s.v[36] * s.dn[1244][9]);
        let eq34_e1313_d_n10: f64 = (s.v[36] * s.dn[1244][10]);
        let eq34_e1313_d_n11: f64 = (s.v[36] * s.dn[1244][11]);
        let eq34_e1313_d_n12: f64 = (s.v[36] * s.dn[1244][12]);
        let eq34_e1313_d_b0: f64 = (s.v[36] * s.db[1244][0]);
        let eq34_e1313_d_b1: f64 = (s.v[36] * s.db[1244][1]);
        let eq34_e1313_d_b2: f64 = (s.v[36] * s.db[1244][2]);
        let eq34_e1313_d_b3: f64 = (s.v[36] * s.db[1244][3]);
        let eq34_e1313_d_b4: f64 = (s.v[36] * s.db[1244][4]);
        let eq34_e1313_d_b5: f64 = (s.v[36] * s.db[1244][5]);
        let eq34_e1313_d_b6: f64 = (s.v[36] * s.db[1244][6]);
        let eq34_e1313_d_b7: f64 = (s.v[36] * s.db[1244][7]);
        let eq34_e1313_d_b8: f64 = (s.v[36] * s.db[1244][8]);
        let eq34_e1313_q: f64 = (s.v[36] * eq34_e1312_q);
        let eq34_reactive_node_derivatives: [f64; 13] = [eq34_e1313_d_n0, eq34_e1313_d_n1, eq34_e1313_d_n2, eq34_e1313_d_n3, eq34_e1313_d_n4, eq34_e1313_d_n5, eq34_e1313_d_n6, eq34_e1313_d_n7, eq34_e1313_d_n8, eq34_e1313_d_n9, eq34_e1313_d_n10, eq34_e1313_d_n11, eq34_e1313_d_n12];
        let eq34_reactive_branch_derivatives: [f64; 9] = [eq34_e1313_d_b0, eq34_e1313_d_b1, eq34_e1313_d_b2, eq34_e1313_d_b3, eq34_e1313_d_b4, eq34_e1313_d_b5, eq34_e1313_d_b6, eq34_e1313_d_b7, eq34_e1313_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            nodes,
            &eq34_reactive_node_derivatives,
            branches,
            &eq34_reactive_branch_derivatives,
            multiplicity,
        );
        let eq35_e1316_q: f64 = s.v[1245];
        let eq35_e1317: f64 = (s.v[36] * s.v[1245]);
        let eq35_e1317_d_n0: f64 = (s.v[36] * s.dn[1245][0]);
        let eq35_e1317_d_n1: f64 = (s.v[36] * s.dn[1245][1]);
        let eq35_e1317_d_n2: f64 = (s.v[36] * s.dn[1245][2]);
        let eq35_e1317_d_n3: f64 = (s.v[36] * s.dn[1245][3]);
        let eq35_e1317_d_n4: f64 = (s.v[36] * s.dn[1245][4]);
        let eq35_e1317_d_n5: f64 = (s.v[36] * s.dn[1245][5]);
        let eq35_e1317_d_n6: f64 = (s.v[36] * s.dn[1245][6]);
        let eq35_e1317_d_n7: f64 = (s.v[36] * s.dn[1245][7]);
        let eq35_e1317_d_n8: f64 = (s.v[36] * s.dn[1245][8]);
        let eq35_e1317_d_n9: f64 = (s.v[36] * s.dn[1245][9]);
        let eq35_e1317_d_n10: f64 = (s.v[36] * s.dn[1245][10]);
        let eq35_e1317_d_n11: f64 = (s.v[36] * s.dn[1245][11]);
        let eq35_e1317_d_n12: f64 = (s.v[36] * s.dn[1245][12]);
        let eq35_e1317_d_b0: f64 = (s.v[36] * s.db[1245][0]);
        let eq35_e1317_d_b1: f64 = (s.v[36] * s.db[1245][1]);
        let eq35_e1317_d_b2: f64 = (s.v[36] * s.db[1245][2]);
        let eq35_e1317_d_b3: f64 = (s.v[36] * s.db[1245][3]);
        let eq35_e1317_d_b4: f64 = (s.v[36] * s.db[1245][4]);
        let eq35_e1317_d_b5: f64 = (s.v[36] * s.db[1245][5]);
        let eq35_e1317_d_b6: f64 = (s.v[36] * s.db[1245][6]);
        let eq35_e1317_d_b7: f64 = (s.v[36] * s.db[1245][7]);
        let eq35_e1317_d_b8: f64 = (s.v[36] * s.db[1245][8]);
        let eq35_e1317_q: f64 = (s.v[36] * eq35_e1316_q);
        let eq35_reactive_node_derivatives: [f64; 13] = [eq35_e1317_d_n0, eq35_e1317_d_n1, eq35_e1317_d_n2, eq35_e1317_d_n3, eq35_e1317_d_n4, eq35_e1317_d_n5, eq35_e1317_d_n6, eq35_e1317_d_n7, eq35_e1317_d_n8, eq35_e1317_d_n9, eq35_e1317_d_n10, eq35_e1317_d_n11, eq35_e1317_d_n12];
        let eq35_reactive_branch_derivatives: [f64; 9] = [eq35_e1317_d_b0, eq35_e1317_d_b1, eq35_e1317_d_b2, eq35_e1317_d_b3, eq35_e1317_d_b4, eq35_e1317_d_b5, eq35_e1317_d_b6, eq35_e1317_d_b7, eq35_e1317_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[8]),
            nodes,
            &eq35_reactive_node_derivatives,
            branches,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq36_e1324, eq36_e1324_d_n0, eq36_e1324_d_n1, eq36_e1324_d_n2, eq36_e1324_d_n3, eq36_e1324_d_n4, eq36_e1324_d_n5, eq36_e1324_d_n6, eq36_e1324_d_n7, eq36_e1324_d_n8, eq36_e1324_d_n9, eq36_e1324_d_n10, eq36_e1324_d_n11, eq36_e1324_d_n12, eq36_e1324_d_b0, eq36_e1324_d_b1, eq36_e1324_d_b2, eq36_e1324_d_b3, eq36_e1324_d_b4, eq36_e1324_d_b5, eq36_e1324_d_b6, eq36_e1324_d_b7, eq36_e1324_d_b8, eq36_e1324_q,) = {
    if s.b[1863] {
        let eq36_e1321_q: f64 = s.v[1230];
        let eq36_e1322: f64 = (s.v[36] * s.v[1230]);
        let eq36_e1322_q: f64 = (s.v[36] * eq36_e1321_q);
        (eq36_e1322, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, eq36_e1322_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_reactive_node_derivatives: [f64; 13] = [eq36_e1324_d_n0, eq36_e1324_d_n1, eq36_e1324_d_n2, eq36_e1324_d_n3, eq36_e1324_d_n4, eq36_e1324_d_n5, eq36_e1324_d_n6, eq36_e1324_d_n7, eq36_e1324_d_n8, eq36_e1324_d_n9, eq36_e1324_d_n10, eq36_e1324_d_n11, eq36_e1324_d_n12];
        let eq36_reactive_branch_derivatives: [f64; 9] = [eq36_e1324_d_b0, eq36_e1324_d_b1, eq36_e1324_d_b2, eq36_e1324_d_b3, eq36_e1324_d_b4, eq36_e1324_d_b5, eq36_e1324_d_b6, eq36_e1324_d_b7, eq36_e1324_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            nodes,
            &eq36_reactive_node_derivatives,
            branches,
            &eq36_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq37_e1331, eq37_e1331_d_n0, eq37_e1331_d_n1, eq37_e1331_d_n2, eq37_e1331_d_n3, eq37_e1331_d_n4, eq37_e1331_d_n5, eq37_e1331_d_n6, eq37_e1331_d_n7, eq37_e1331_d_n8, eq37_e1331_d_n9, eq37_e1331_d_n10, eq37_e1331_d_n11, eq37_e1331_d_n12, eq37_e1331_d_b0, eq37_e1331_d_b1, eq37_e1331_d_b2, eq37_e1331_d_b3, eq37_e1331_d_b4, eq37_e1331_d_b5, eq37_e1331_d_b6, eq37_e1331_d_b7, eq37_e1331_d_b8, eq37_e1331_q,) = {
    if s.b[1863] {
        let eq37_e1328_q: f64 = s.v[1231];
        let eq37_e1329: f64 = (s.v[36] * s.v[1231]);
        let eq37_e1329_q: f64 = (s.v[36] * eq37_e1328_q);
        (eq37_e1329, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, eq37_e1329_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq37_reactive_node_derivatives: [f64; 13] = [eq37_e1331_d_n0, eq37_e1331_d_n1, eq37_e1331_d_n2, eq37_e1331_d_n3, eq37_e1331_d_n4, eq37_e1331_d_n5, eq37_e1331_d_n6, eq37_e1331_d_n7, eq37_e1331_d_n8, eq37_e1331_d_n9, eq37_e1331_d_n10, eq37_e1331_d_n11, eq37_e1331_d_n12];
        let eq37_reactive_branch_derivatives: [f64; 9] = [eq37_e1331_d_b0, eq37_e1331_d_b1, eq37_e1331_d_b2, eq37_e1331_d_b3, eq37_e1331_d_b4, eq37_e1331_d_b5, eq37_e1331_d_b6, eq37_e1331_d_b7, eq37_e1331_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[8]),
            nodes,
            &eq37_reactive_node_derivatives,
            branches,
            &eq37_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq38_e1338, eq38_e1338_d_n0, eq38_e1338_d_n1, eq38_e1338_d_n2, eq38_e1338_d_n3, eq38_e1338_d_n4, eq38_e1338_d_n5, eq38_e1338_d_n6, eq38_e1338_d_n7, eq38_e1338_d_n8, eq38_e1338_d_n9, eq38_e1338_d_n10, eq38_e1338_d_n11, eq38_e1338_d_n12, eq38_e1338_d_b0, eq38_e1338_d_b1, eq38_e1338_d_b2, eq38_e1338_d_b3, eq38_e1338_d_b4, eq38_e1338_d_b5, eq38_e1338_d_b6, eq38_e1338_d_b7, eq38_e1338_d_b8, eq38_e1338_q,) = {
    if s.b[1863] {
        let eq38_e1335: f64 = ((nv10 - nv3) * s.v[697]);
        let eq38_e1335_d_n0: f64 = ((nv10 - nv3) * s.dn[697][0]);
        let eq38_e1335_d_n1: f64 = ((nv10 - nv3) * s.dn[697][1]);
        let eq38_e1335_d_n2: f64 = ((nv10 - nv3) * s.dn[697][2]);
        let eq38_e1335_d_n3: f64 = ((-s.v[697]) + ((nv10 - nv3) * s.dn[697][3]));
        let eq38_e1335_d_n4: f64 = ((nv10 - nv3) * s.dn[697][4]);
        let eq38_e1335_d_n5: f64 = ((nv10 - nv3) * s.dn[697][5]);
        let eq38_e1335_d_n6: f64 = ((nv10 - nv3) * s.dn[697][6]);
        let eq38_e1335_d_n7: f64 = ((nv10 - nv3) * s.dn[697][7]);
        let eq38_e1335_d_n8: f64 = ((nv10 - nv3) * s.dn[697][8]);
        let eq38_e1335_d_n9: f64 = ((nv10 - nv3) * s.dn[697][9]);
        let eq38_e1335_d_n10: f64 = (s.v[697] + ((nv10 - nv3) * s.dn[697][10]));
        let eq38_e1335_d_n11: f64 = ((nv10 - nv3) * s.dn[697][11]);
        let eq38_e1335_d_n12: f64 = ((nv10 - nv3) * s.dn[697][12]);
        let eq38_e1335_d_b0: f64 = ((nv10 - nv3) * s.db[697][0]);
        let eq38_e1335_d_b1: f64 = ((nv10 - nv3) * s.db[697][1]);
        let eq38_e1335_d_b2: f64 = ((nv10 - nv3) * s.db[697][2]);
        let eq38_e1335_d_b3: f64 = ((nv10 - nv3) * s.db[697][3]);
        let eq38_e1335_d_b4: f64 = ((nv10 - nv3) * s.db[697][4]);
        let eq38_e1335_d_b5: f64 = ((nv10 - nv3) * s.db[697][5]);
        let eq38_e1335_d_b6: f64 = ((nv10 - nv3) * s.db[697][6]);
        let eq38_e1335_d_b7: f64 = ((nv10 - nv3) * s.db[697][7]);
        let eq38_e1335_d_b8: f64 = ((nv10 - nv3) * s.db[697][8]);
        let eq38_e1336_q: f64 = eq38_e1335;
        (eq38_e1335, eq38_e1335_d_n0, eq38_e1335_d_n1, eq38_e1335_d_n2, eq38_e1335_d_n3, eq38_e1335_d_n4, eq38_e1335_d_n5, eq38_e1335_d_n6, eq38_e1335_d_n7, eq38_e1335_d_n8, eq38_e1335_d_n9, eq38_e1335_d_n10, eq38_e1335_d_n11, eq38_e1335_d_n12, eq38_e1335_d_b0, eq38_e1335_d_b1, eq38_e1335_d_b2, eq38_e1335_d_b3, eq38_e1335_d_b4, eq38_e1335_d_b5, eq38_e1335_d_b6, eq38_e1335_d_b7, eq38_e1335_d_b8, eq38_e1336_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_reactive_node_derivatives: [f64; 13] = [eq38_e1338_d_n0, eq38_e1338_d_n1, eq38_e1338_d_n2, eq38_e1338_d_n3, eq38_e1338_d_n4, eq38_e1338_d_n5, eq38_e1338_d_n6, eq38_e1338_d_n7, eq38_e1338_d_n8, eq38_e1338_d_n9, eq38_e1338_d_n10, eq38_e1338_d_n11, eq38_e1338_d_n12];
        let eq38_reactive_branch_derivatives: [f64; 9] = [eq38_e1338_d_b0, eq38_e1338_d_b1, eq38_e1338_d_b2, eq38_e1338_d_b3, eq38_e1338_d_b4, eq38_e1338_d_b5, eq38_e1338_d_b6, eq38_e1338_d_b7, eq38_e1338_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[3]),
            nodes,
            &eq38_reactive_node_derivatives,
            branches,
            &eq38_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq39_e1346, eq39_e1346_d_n0, eq39_e1346_d_n1, eq39_e1346_d_n2, eq39_e1346_d_n3, eq39_e1346_d_n4, eq39_e1346_d_n5, eq39_e1346_d_n6, eq39_e1346_d_n7, eq39_e1346_d_n8, eq39_e1346_d_n9, eq39_e1346_d_n10, eq39_e1346_d_n11, eq39_e1346_d_n12, eq39_e1346_d_b0, eq39_e1346_d_b1, eq39_e1346_d_b2, eq39_e1346_d_b3, eq39_e1346_d_b4, eq39_e1346_d_b5, eq39_e1346_d_b6, eq39_e1346_d_b7, eq39_e1346_d_b8, eq39_e1346_q,) = {
    if (!s.b[1863]) {
        let eq39_e1343_q: f64 = s.v[1230];
        let eq39_e1344: f64 = (s.v[36] * s.v[1230]);
        let eq39_e1344_q: f64 = (s.v[36] * eq39_e1343_q);
        (eq39_e1344, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, eq39_e1344_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_reactive_node_derivatives: [f64; 13] = [eq39_e1346_d_n0, eq39_e1346_d_n1, eq39_e1346_d_n2, eq39_e1346_d_n3, eq39_e1346_d_n4, eq39_e1346_d_n5, eq39_e1346_d_n6, eq39_e1346_d_n7, eq39_e1346_d_n8, eq39_e1346_d_n9, eq39_e1346_d_n10, eq39_e1346_d_n11, eq39_e1346_d_n12];
        let eq39_reactive_branch_derivatives: [f64; 9] = [eq39_e1346_d_b0, eq39_e1346_d_b1, eq39_e1346_d_b2, eq39_e1346_d_b3, eq39_e1346_d_b4, eq39_e1346_d_b5, eq39_e1346_d_b6, eq39_e1346_d_b7, eq39_e1346_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq39_reactive_node_derivatives,
            branches,
            &eq39_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq40_e1354, eq40_e1354_d_n0, eq40_e1354_d_n1, eq40_e1354_d_n2, eq40_e1354_d_n3, eq40_e1354_d_n4, eq40_e1354_d_n5, eq40_e1354_d_n6, eq40_e1354_d_n7, eq40_e1354_d_n8, eq40_e1354_d_n9, eq40_e1354_d_n10, eq40_e1354_d_n11, eq40_e1354_d_n12, eq40_e1354_d_b0, eq40_e1354_d_b1, eq40_e1354_d_b2, eq40_e1354_d_b3, eq40_e1354_d_b4, eq40_e1354_d_b5, eq40_e1354_d_b6, eq40_e1354_d_b7, eq40_e1354_d_b8, eq40_e1354_q,) = {
    if (!s.b[1863]) {
        let eq40_e1351_q: f64 = s.v[1231];
        let eq40_e1352: f64 = (s.v[36] * s.v[1231]);
        let eq40_e1352_q: f64 = (s.v[36] * eq40_e1351_q);
        (eq40_e1352, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, eq40_e1352_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_reactive_node_derivatives: [f64; 13] = [eq40_e1354_d_n0, eq40_e1354_d_n1, eq40_e1354_d_n2, eq40_e1354_d_n3, eq40_e1354_d_n4, eq40_e1354_d_n5, eq40_e1354_d_n6, eq40_e1354_d_n7, eq40_e1354_d_n8, eq40_e1354_d_n9, eq40_e1354_d_n10, eq40_e1354_d_n11, eq40_e1354_d_n12];
        let eq40_reactive_branch_derivatives: [f64; 9] = [eq40_e1354_d_b0, eq40_e1354_d_b1, eq40_e1354_d_b2, eq40_e1354_d_b3, eq40_e1354_d_b4, eq40_e1354_d_b5, eq40_e1354_d_b6, eq40_e1354_d_b7, eq40_e1354_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq40_reactive_node_derivatives,
            branches,
            &eq40_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq41_e1362, eq41_e1362_d_n0, eq41_e1362_d_n1, eq41_e1362_d_n2, eq41_e1362_d_n3, eq41_e1362_d_n4, eq41_e1362_d_n5, eq41_e1362_d_n6, eq41_e1362_d_n7, eq41_e1362_d_n8, eq41_e1362_d_n9, eq41_e1362_d_n10, eq41_e1362_d_n11, eq41_e1362_d_n12, eq41_e1362_d_b0, eq41_e1362_d_b1, eq41_e1362_d_b2, eq41_e1362_d_b3, eq41_e1362_d_b4, eq41_e1362_d_b5, eq41_e1362_d_b6, eq41_e1362_d_b7, eq41_e1362_d_b8, eq41_e1362_q,) = {
    if (!s.b[1863]) {
        let eq41_e1359: f64 = ((nv9 - nv3) * s.v[697]);
        let eq41_e1359_d_n0: f64 = ((nv9 - nv3) * s.dn[697][0]);
        let eq41_e1359_d_n1: f64 = ((nv9 - nv3) * s.dn[697][1]);
        let eq41_e1359_d_n2: f64 = ((nv9 - nv3) * s.dn[697][2]);
        let eq41_e1359_d_n3: f64 = ((-s.v[697]) + ((nv9 - nv3) * s.dn[697][3]));
        let eq41_e1359_d_n4: f64 = ((nv9 - nv3) * s.dn[697][4]);
        let eq41_e1359_d_n5: f64 = ((nv9 - nv3) * s.dn[697][5]);
        let eq41_e1359_d_n6: f64 = ((nv9 - nv3) * s.dn[697][6]);
        let eq41_e1359_d_n7: f64 = ((nv9 - nv3) * s.dn[697][7]);
        let eq41_e1359_d_n8: f64 = ((nv9 - nv3) * s.dn[697][8]);
        let eq41_e1359_d_n9: f64 = (s.v[697] + ((nv9 - nv3) * s.dn[697][9]));
        let eq41_e1359_d_n10: f64 = ((nv9 - nv3) * s.dn[697][10]);
        let eq41_e1359_d_n11: f64 = ((nv9 - nv3) * s.dn[697][11]);
        let eq41_e1359_d_n12: f64 = ((nv9 - nv3) * s.dn[697][12]);
        let eq41_e1359_d_b0: f64 = ((nv9 - nv3) * s.db[697][0]);
        let eq41_e1359_d_b1: f64 = ((nv9 - nv3) * s.db[697][1]);
        let eq41_e1359_d_b2: f64 = ((nv9 - nv3) * s.db[697][2]);
        let eq41_e1359_d_b3: f64 = ((nv9 - nv3) * s.db[697][3]);
        let eq41_e1359_d_b4: f64 = ((nv9 - nv3) * s.db[697][4]);
        let eq41_e1359_d_b5: f64 = ((nv9 - nv3) * s.db[697][5]);
        let eq41_e1359_d_b6: f64 = ((nv9 - nv3) * s.db[697][6]);
        let eq41_e1359_d_b7: f64 = ((nv9 - nv3) * s.db[697][7]);
        let eq41_e1359_d_b8: f64 = ((nv9 - nv3) * s.db[697][8]);
        let eq41_e1360_q: f64 = eq41_e1359;
        (eq41_e1359, eq41_e1359_d_n0, eq41_e1359_d_n1, eq41_e1359_d_n2, eq41_e1359_d_n3, eq41_e1359_d_n4, eq41_e1359_d_n5, eq41_e1359_d_n6, eq41_e1359_d_n7, eq41_e1359_d_n8, eq41_e1359_d_n9, eq41_e1359_d_n10, eq41_e1359_d_n11, eq41_e1359_d_n12, eq41_e1359_d_b0, eq41_e1359_d_b1, eq41_e1359_d_b2, eq41_e1359_d_b3, eq41_e1359_d_b4, eq41_e1359_d_b5, eq41_e1359_d_b6, eq41_e1359_d_b7, eq41_e1359_d_b8, eq41_e1360_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq41_reactive_node_derivatives: [f64; 13] = [eq41_e1362_d_n0, eq41_e1362_d_n1, eq41_e1362_d_n2, eq41_e1362_d_n3, eq41_e1362_d_n4, eq41_e1362_d_n5, eq41_e1362_d_n6, eq41_e1362_d_n7, eq41_e1362_d_n8, eq41_e1362_d_n9, eq41_e1362_d_n10, eq41_e1362_d_n11, eq41_e1362_d_n12];
        let eq41_reactive_branch_derivatives: [f64; 9] = [eq41_e1362_d_b0, eq41_e1362_d_b1, eq41_e1362_d_b2, eq41_e1362_d_b3, eq41_e1362_d_b4, eq41_e1362_d_b5, eq41_e1362_d_b6, eq41_e1362_d_b7, eq41_e1362_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[3]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let eq42_e1364_q: f64 = s.v[449];
        let eq42_reactive_node_derivatives: [f64; 13] = [s.dn[449][0], s.dn[449][1], s.dn[449][2], s.dn[449][3], s.dn[449][4], s.dn[449][5], s.dn[449][6], s.dn[449][7], s.dn[449][8], s.dn[449][9], s.dn[449][10], s.dn[449][11], s.dn[449][12]];
        let eq42_reactive_branch_derivatives: [f64; 9] = [s.db[449][0], s.db[449][1], s.db[449][2], s.db[449][3], s.db[449][4], s.db[449][5], s.db[449][6], s.db[449][7], s.db[449][8]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[3]),
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
        let eq43_e1366_q: f64 = s.v[448];
        let eq43_reactive_node_derivatives: [f64; 13] = [s.dn[448][0], s.dn[448][1], s.dn[448][2], s.dn[448][3], s.dn[448][4], s.dn[448][5], s.dn[448][6], s.dn[448][7], s.dn[448][8], s.dn[448][9], s.dn[448][10], s.dn[448][11], s.dn[448][12]];
        let eq43_reactive_branch_derivatives: [f64; 9] = [s.db[448][0], s.db[448][1], s.db[448][2], s.db[448][3], s.db[448][4], s.db[448][5], s.db[448][6], s.db[448][7], s.db[448][8]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[3]),
            nodes,
            &eq43_reactive_node_derivatives,
            branches,
            &eq43_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq58_e1476, eq58_e1476_d_n0, eq58_e1476_d_n1, eq58_e1476_d_n2, eq58_e1476_d_n3, eq58_e1476_d_n4, eq58_e1476_d_n5, eq58_e1476_d_n6, eq58_e1476_d_n7, eq58_e1476_d_n8, eq58_e1476_d_n9, eq58_e1476_d_n10, eq58_e1476_d_n11, eq58_e1476_d_n12, eq58_e1476_d_b0, eq58_e1476_d_b1, eq58_e1476_d_b2, eq58_e1476_d_b3, eq58_e1476_d_b4, eq58_e1476_d_b5, eq58_e1476_d_b6, eq58_e1476_d_b7, eq58_e1476_d_b8, eq58_e1476_q,) = {
    if s.b[1869] {
        let eq58_e1473: f64 = (s.v[770] * s.v[528]);
        let eq58_e1473_d_n0: f64 = (s.dn[770][0] * s.v[528]);
        let eq58_e1473_d_n1: f64 = (s.dn[770][1] * s.v[528]);
        let eq58_e1473_d_n2: f64 = (s.dn[770][2] * s.v[528]);
        let eq58_e1473_d_n3: f64 = (s.dn[770][3] * s.v[528]);
        let eq58_e1473_d_n4: f64 = (s.dn[770][4] * s.v[528]);
        let eq58_e1473_d_n5: f64 = (s.dn[770][5] * s.v[528]);
        let eq58_e1473_d_n6: f64 = (s.dn[770][6] * s.v[528]);
        let eq58_e1473_d_n7: f64 = (s.dn[770][7] * s.v[528]);
        let eq58_e1473_d_n8: f64 = (s.dn[770][8] * s.v[528]);
        let eq58_e1473_d_n9: f64 = (s.dn[770][9] * s.v[528]);
        let eq58_e1473_d_n10: f64 = (s.dn[770][10] * s.v[528]);
        let eq58_e1473_d_n11: f64 = (s.dn[770][11] * s.v[528]);
        let eq58_e1473_d_n12: f64 = (s.dn[770][12] * s.v[528]);
        let eq58_e1473_d_b0: f64 = (s.db[770][0] * s.v[528]);
        let eq58_e1473_d_b1: f64 = (s.db[770][1] * s.v[528]);
        let eq58_e1473_d_b2: f64 = (s.db[770][2] * s.v[528]);
        let eq58_e1473_d_b3: f64 = (s.db[770][3] * s.v[528]);
        let eq58_e1473_d_b4: f64 = (s.db[770][4] * s.v[528]);
        let eq58_e1473_d_b5: f64 = (s.db[770][5] * s.v[528]);
        let eq58_e1473_d_b6: f64 = (s.db[770][6] * s.v[528]);
        let eq58_e1473_d_b7: f64 = (s.db[770][7] * s.v[528]);
        let eq58_e1473_d_b8: f64 = (s.db[770][8] * s.v[528]);
        let eq58_e1474_q: f64 = eq58_e1473;
        (eq58_e1473, eq58_e1473_d_n0, eq58_e1473_d_n1, eq58_e1473_d_n2, eq58_e1473_d_n3, eq58_e1473_d_n4, eq58_e1473_d_n5, eq58_e1473_d_n6, eq58_e1473_d_n7, eq58_e1473_d_n8, eq58_e1473_d_n9, eq58_e1473_d_n10, eq58_e1473_d_n11, eq58_e1473_d_n12, eq58_e1473_d_b0, eq58_e1473_d_b1, eq58_e1473_d_b2, eq58_e1473_d_b3, eq58_e1473_d_b4, eq58_e1473_d_b5, eq58_e1473_d_b6, eq58_e1473_d_b7, eq58_e1473_d_b8, eq58_e1474_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq58_reactive_node_derivatives: [f64; 13] = [eq58_e1476_d_n0, eq58_e1476_d_n1, eq58_e1476_d_n2, eq58_e1476_d_n3, eq58_e1476_d_n4, eq58_e1476_d_n5, eq58_e1476_d_n6, eq58_e1476_d_n7, eq58_e1476_d_n8, eq58_e1476_d_n9, eq58_e1476_d_n10, eq58_e1476_d_n11, eq58_e1476_d_n12];
        let eq58_reactive_branch_derivatives: [f64; 9] = [eq58_e1476_d_b0, eq58_e1476_d_b1, eq58_e1476_d_b2, eq58_e1476_d_b3, eq58_e1476_d_b4, eq58_e1476_d_b5, eq58_e1476_d_b6, eq58_e1476_d_b7, eq58_e1476_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            None,
            nodes,
            &eq58_reactive_node_derivatives,
            branches,
            &eq58_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
