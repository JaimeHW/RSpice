#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1608] && s.b[1609]) && s.b[1615]) {
            s.store_scaled_square(1392, 264, (0.1666666666667 * 0.707106781186545));
            s.store_mul_ad_product_rhs_mixed_ia(4, 1394, 264, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(1394), 1.0, s.ad_value(1395)), s.ad_value(260), s.ad_value(1392)), 1.0));
        }

        s.b[1616] = (s.v[1394] < (-s.v[265]));
        s.store_scalar(1616, if s.b[1616] { 1.0 } else { 0.0 });

        if (((s.b[1608] && s.b[1609]) && (!s.b[1615])) && s.b[1616]) {
            s.store_neg(1396, 1394);
            s.store_scaled_mul(1397, 1396, 264, 1.25);
            s.store_scaled_sub_offset_sqrt_square_offset(1398, 1397, 10.0, (-6.0), 64.0, 0.5);
            s.store_sub(1391, 1396, 1398);
            s.store_add_scaled_square_product_mixed_iia(1399, 1391, 1.0, 261, A::offset(s.ad_value(1398), 1.0), 1.0);
            s.store_sub_scaled_inputs(1401, 1391, 2.0, 261, 1.0);
            s.store_sub_ln_mul_lhs(1402, 1399, 262, 1398);
            s.store_add(1389, 1399, 1401);
            s.store_add_scaled_square_product_mixed_iia(1390, 1389, 1.0, 1402, A::add_scaled_product(s.ad_value(1399), (-1.0), s.ad_value(1401), s.ad_value(1401), 0.5), 1.0);
            s.store_add_ad_rhs(1403, 1398, A::div_scaled_product3(s.ad_value(1399), s.ad_value(1389), s.ad_value(1402), 1.0, A::add(s.ad_value(1390), A::mul3(A::mul3(A::div(s.ad_value(1389), s.ad_value(1390)), s.ad_value(1402), s.ad_value(1402)), s.ad_value(1401), A::sub_scaled_inputs(A::square(s.ad_value(1401)), 0.3333333333333, s.ad_value(1399), 1.0))), 1.0));
        }

        s.b[1617] = (s.v[1403] < 80.0);
        s.store_scalar(1617, if s.b[1617] { 1.0 } else { 0.0 });

        if ((((s.b[1608] && s.b[1609]) && (!s.b[1615])) && s.b[1616]) && s.b[1617]) {
            s.store_exp(1404, 1403);
        }

        if ((((s.b[1608] && s.b[1609]) && (!s.b[1615])) && s.b[1616]) && (!s.b[1617])) {
            s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(1404, 1403, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);
        }

        if (((s.b[1608] && s.b[1609]) && (!s.b[1615])) && s.b[1616]) {
            s.store_div_from_scalar(1405, 1.0, 1404);
            s.store_div_from_scalar_offset_square(1391, 1.0, 1403, 2.0);
            s.store_mul_square_lhs(1406, 1403, 1391);
            s.store_mul3_affine_lhs(1407, 1403, 1391, 4.0, 0.0, 1391);
            s.store_mul_ad_product_lhs_mixed_ai(1408, A::sub_scaled_inputs(s.ad_value(1391), 8.0, s.ad_value(1406), 12.0), 1391, 1391);
            s.store_sub(1391, 1396, 1403);
            s.store_mul(1392, 1395, 1405);
            s.store_add_scaled_product_right_ad(1409, 1391, 2.0, 261, A::add_scaled_inputs3_offset(s.ad_value(1404), 1.0, s.ad_value(1392), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(1395), 1.0, s.ad_value(1407)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(1410, 1391, 1.0, 261, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(1404), 1.0, s.ad_value(1403), (-1.0), s.ad_value(1392), 1.0, (-1.0)), 1.0, s.ad_value(1395), A::sub(A::offset(s.ad_value(1403), (-1.0)), s.ad_value(1406)), 1.0), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(1391, 2.0, 261, A::add_scaled_inputs_product(s.ad_value(1404), 1.0, s.ad_value(1392), 1.0, s.ad_value(1395), s.ad_value(1408), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(1391, 1409, 1.0, 1410, 1391, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(4, 1403, -1.0, A::div(s.ad_value(1410), A::add(s.ad_value(1409), A::sqrt(s.ad_value(1391)))), 2.0);
        }

        if (((s.b[1608] && s.b[1609]) && (!s.b[1615])) && (!s.b[1616])) {
            s.store_div_from_scalar_offset_scaled_input(1411, 1.0, 260, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(1412, A::mul_scaled_lhs(s.ad_value(263), 1.25, s.ad_value(1411)), (-1.0), 1411);
            s.store_mul_ad_product_rhs_mixed_ia(1413, 1394, 264, A::offset(A::mul(s.ad_value(1412), s.ad_value(1394)), 1.0));
        }

        s.b[1618] = ((-s.v[1413]) > (-80.0));
        s.store_scalar(1618, if s.b[1618] { 1.0 } else { 0.0 });

        if ((((s.b[1608] && s.b[1609]) && (!s.b[1615])) && (!s.b[1616])) && s.b[1618]) {
            s.store_exp_neg_input(1391, 1413);
        }

        if ((((s.b[1608] && s.b[1609]) && (!s.b[1615])) && (!s.b[1616])) && (!s.b[1618])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(1391, 1.80485e-35, A::neg(A::neg(s.ad_value(1413))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if (((s.b[1608] && s.b[1609]) && (!s.b[1615])) && (!s.b[1616])) {
            s.store_sub_from_scalar(1414, 1.0, 1391);
            s.store_add_scaled_inputs_product_right_ad(1415, 1394, 1.0, 261, 0.5, 260, A::sqrt(A::add_scaled_inputs3(s.ad_value(1394), 1.0, s.ad_value(261), 0.25, s.ad_value(1414), -1.0)), (-1.0));
            s.store_offset(1416, 266, 3.0);
            s.store_sub_ad(1398, A::add_scaled_inputs3(s.ad_value(1415), 0.5, s.ad_value(1416), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1415), s.ad_value(1416)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(1416), 0.5, A::sqrt_square_offset(s.ad_value(1416), 5.0), 0.5));
            s.store_sub(1391, 1394, 1398);
            s.store_exp_neg_input(1392, 1398);
            s.store_div_from_scalar_offset_square(1393, 1.0, 1398, 2.0);
            s.store_mul_square_lhs(1406, 1398, 1393);
            s.store_mul3_affine_lhs(1407, 1398, 1393, 4.0, 0.0, 1393);
            s.store_mul_ad_product_lhs_mixed_ai(1408, A::sub_scaled_inputs(s.ad_value(1393), 8.0, s.ad_value(1406), 12.0), 1393, 1393);
            s.store_max_from_scalar_ad(1399, 1e-40, A::add_scaled_square_product(s.ad_value(1391), 1.0, s.ad_value(261), A::add_scaled_product(A::offset(A::add(s.ad_value(1392), s.ad_value(1398)), (-1.0)), 1.0, s.ad_value(1395), A::add(A::offset(s.ad_value(1398), 1.0), s.ad_value(1406)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(1400, 1.0, 261, A::add_scaled_product(s.ad_value(1392), 1.0, s.ad_value(1395), s.ad_value(1408), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(1401, 1391, 2.0, 261, A::add_scaled_sub_value_product(1.0, s.ad_value(1392), 1.0, s.ad_value(1395), A::offset(s.ad_value(1407), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(1402, 266, 1.0, 1398, (-1.0), A::ln(A::div(s.ad_value(1399), s.ad_value(261))), 1.0);
            s.store_add(1389, 1399, 1401);
            s.store_add_scaled_square_product_mixed_iia(1390, 1389, 1.0, 1402, A::add_scaled_products(s.ad_value(1401), s.ad_value(1401), 0.5, s.ad_value(1399), s.ad_value(1400), (-1.0)), 1.0);
            s.store_add_ad_rhs(1417, 1398, A::div_scaled_product3(s.ad_value(1399), s.ad_value(1389), s.ad_value(1402), 1.0, A::add(s.ad_value(1390), A::mul3(A::mul3(A::div(s.ad_value(1389), s.ad_value(1390)), s.ad_value(1402), s.ad_value(1402)), s.ad_value(1401), A::add_scaled_square_product(s.ad_value(1401), 0.3333333333333, s.ad_value(1399), s.ad_value(1400), (-1.0)))), 1.0));
        }

        s.b[1619] = (s.v[1417] < 80.0);
        s.store_scalar(1619, if s.b[1619] { 1.0 } else { 0.0 });

        if ((((s.b[1608] && s.b[1609]) && (!s.b[1615])) && (!s.b[1616])) && s.b[1619]) {
            s.store_exp(1404, 1417);
            s.store_div_from_scalar(1405, 1.0, 1404);
            s.store_mul(1404, 1395, 1404);
        }

        s.b[1620] = (s.v[1417] > (s.v[266] - 80.0));
        s.store_scalar(1620, if s.b[1620] { 1.0 } else { 0.0 });

        if (((((s.b[1608] && s.b[1609]) && (!s.b[1615])) && (!s.b[1616])) && (!s.b[1619])) && s.b[1620]) {
            s.store_exp_sub(1404, 1417, 266);
            s.store_div(1405, 1395, 1404);
        }

        if (((((s.b[1608] && s.b[1609]) && (!s.b[1615])) && (!s.b[1616])) && (!s.b[1619])) && (!s.b[1620])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(1404, 1.80485e-35, A::sub(s.ad_value(266), s.ad_value(1417)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_mixed_ia(1405, 1.80485e-35, 1417, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if (((s.b[1608] && s.b[1609]) && (!s.b[1615])) && (!s.b[1616])) {
            s.store_div_from_scalar_offset_square(1391, 1.0, 1417, 2.0);
            s.store_mul_square_lhs(1406, 1417, 1391);
            s.store_mul3_affine_lhs(1407, 1417, 1391, 4.0, 0.0, 1391);
            s.store_mul_ad_product_lhs_mixed_ai(1408, A::sub_scaled_inputs(s.ad_value(1391), 8.0, s.ad_value(1406), 12.0), 1391, 1391);
            s.store_sub(1391, 1394, 1417);
            s.store_add_scaled_product_right_ad(1409, 1391, 2.0, 261, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(1405)), 1.0, s.ad_value(1404), 1.0, s.ad_value(1395), A::offset(s.ad_value(1407), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(1410, 1391, 1.0, 261, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(1405), 1.0, s.ad_value(1417), 1.0, s.ad_value(1404), 1.0, (-1.0)), 1.0, s.ad_value(1395), A::add(A::offset(s.ad_value(1417), 1.0), s.ad_value(1406)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(1391, 2.0, 261, A::add_scaled_inputs_product(s.ad_value(1405), 1.0, s.ad_value(1404), 1.0, s.ad_value(1395), s.ad_value(1408), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(1391, 1409, 1.0, 1410, 1391, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(4, 1417, 1.0, A::div(s.ad_value(1410), A::add(s.ad_value(1409), A::sqrt(s.ad_value(1391)))), 2.0);
        }

        if (s.b[1608] && s.b[1609]) {
            s.store_mul_add_rhs(1438, 0, 4, 3);
        }

        if (s.b[1608] && (!s.b[1609])) {
            s.copy_ad(1438, 1429);
        }

        if s.b[1608] {
            s.store_mul_sub_rhs(0, 248, 1427, 1438);
        }

        s.b[1621] = (p.p13 > 0.0);
        s.store_scalar(1621, if s.b[1621] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1621]) {
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1439, 0, 0.5, 257, 0.5, A::add(A::square(A::sub(s.ad_value(0), s.ad_value(257))), A::square(s.ad_value(257))), 0.5);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1440, 257, 0.5, 0, ((-1.0) * 0.5), A::add(A::square(A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(257), 1.0)), A::square(s.ad_value(257))), 0.5);
            s.store_mul_ad_rhs(2, 258, A::exp_scaled_input(A::ln(s.ad_value(1439)), (-0.3333333333333)));
            s.store_mul_ad_rhs(3, 258, A::exp_scaled_input(A::ln(s.ad_value(1440)), (-0.3333333333333)));
            s.store_sub_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
            s.store_div(1447, 245, 4);
            s.store_offset_mul(1441, 246, 2, 1.0);
            s.store_offset_mul(1442, 247, 3, 1.0);
            s.store_div_scaled_product_indices(1443, 246, 4, 1.0, 1441, 1.0);
            s.store_div_scaled_product_indices(1444, 247, 4, 1.0, 1442, 1.0);
            s.store_div_from_scalar_add_ad(1445, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(1443)), 1.0), A::div_from_scalar(1.0, s.ad_value(1444)));
            s.store_offset_mul(1441, 1443, 2, 1.0);
            s.store_offset_mul(1442, 1444, 3, 1.0);
        }

        if (s.b[1608] && (!s.b[1621])) {
            s.copy_ad(1447, 245);
            s.copy_ad(1443, 246);
            s.copy_ad(1444, 247);
            s.copy_ad(1445, 248);
            s.store_scalar(1441, 1.0);
            s.store_scalar(1442, 1.0);
        }

        if s.b[1608] {
            s.store_mul_sub_rhs(1446, 1445, 1427, 1438);
        }

        s.b[1622] = (s.v[1446] > 0.0);
        s.store_scalar(1622, if s.b[1622] { 1.0 } else { 0.0 });

        s.b[1623] = ((-s.v[1446]) < 80.0);
        s.store_scalar(1623, if s.b[1623] { 1.0 } else { 0.0 });

        if ((s.b[1608] && s.b[1622]) && s.b[1623]) {
            s.store_ln_one_plus_exp_neg_input(0, 1446);
        }

        if ((s.b[1608] && s.b[1622]) && (!s.b[1623])) {
            s.store_neg(0, 1446);
        }

        if (s.b[1608] && s.b[1622]) {
            s.store_add_scaled_inputs3_offset_mixed_iai(1448, 1427, 1.0, A::div(s.ad_value(1446), s.ad_value(1443)), (-1.0), 0, 1.0, (-0.6931471805599));
        }

        s.b[1624] = (s.v[1446] < 80.0);
        s.store_scalar(1624, if s.b[1624] { 1.0 } else { 0.0 });

        if ((s.b[1608] && (!s.b[1622])) && s.b[1624]) {
            s.store_ln_one_plus_exp(0, 1446);
        }

        if ((s.b[1608] && (!s.b[1622])) && (!s.b[1624])) {
            s.copy_ad(0, 1446);
        }

        if (s.b[1608] && (!s.b[1622])) {
            s.store_add_scaled_inputs3_offset_mixed_iai(1448, 1438, 1.0, A::div(s.ad_value(1446), s.ad_value(1444)), 1.0, 0, 1.0, (-0.6931471805599));
        }

        if s.b[1608] {
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1449, 1448, 0.5, 254, 0.5, 1448, 254, 4.0, (-0.5));
            s.store_offset_sqrt_ad(1450, A::offset(A::div_scaled_inputs2(s.ad_value(254), 2.0, s.ad_value(1449), (-2.0), s.ad_value(255), 1.0), 1.0), (-1.0));
            s.store_add_scaled_product_indices(1451, 1449, 1.0, 255, 1450, 1.0);
            s.store_scaled_add_offset_sqrt_square_offset_ad(0, A::mul(s.ad_value(30), s.ad_value(1428)), ((1.0) + (0.5)), ((1.0) + ((-0.5))), 0.01, 0.5);
            s.store_div_from_scalar_offset_product(1452, 1.0, 1420, 0, 1.0);
            s.store_div_from_scalar_offset_product(1453, 1.0, 1421, 0, 1.0);
            s.store_mul_offset_rhs_ad(0, A::mul3_scaled_output(s.ad_value(329), A::offset(A::sqrt(A::offset(A::div(s.ad_value(340), s.ad_value(329)), 1.0)), (-1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(1450)), 1.0), 2.0), A::mul(s.ad_value(37), s.ad_value(1428)), 1.0);
            s.store_mul(1454, 1422, 0);
            s.store_mul(1455, 1423, 0);
            s.store_add_ad_lhs(1456, A::add_scaled_product(s.ad_value(1451), 1.0, A::add_scaled_inputs3(s.ad_value(1427), 1.0, s.ad_value(1451), (-1.0), s.ad_value(1454), 1.0), s.ad_value(1452), 1.0), 341);
            s.store_add_ad_lhs(1457, A::add_scaled_product(s.ad_value(1451), 1.0, A::add_scaled_inputs3(s.ad_value(1438), 1.0, s.ad_value(1451), (-1.0), s.ad_value(1455), 1.0), s.ad_value(1453), 1.0), 341);
            s.store_add_scaled_inputs3_sqrt_third_mixed_aia(1458, A::add_scaled_product(s.ad_value(1457), 1.0, s.ad_value(25), A::sub(s.ad_value(1456), s.ad_value(1457)), 1.0), 0.5, 225, 0.5, A::offset(A::square(A::sub(A::add_scaled_product(s.ad_value(1457), 1.0, s.ad_value(25), A::sub(s.ad_value(1456), s.ad_value(1457)), 1.0), s.ad_value(225))), 0.01), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_aia(1459, A::add_scaled_product(s.ad_value(1456), 1.0, s.ad_value(26), A::sub(s.ad_value(1457), s.ad_value(1456)), 1.0), 0.5, 225, 0.5, A::offset(A::square(A::sub(A::add_scaled_product(s.ad_value(1456), 1.0, s.ad_value(26), A::sub(s.ad_value(1457), s.ad_value(1456)), 1.0), s.ad_value(225))), 0.01), (-0.5));
            s.store_div(1460, 1443, 1452);
            s.store_div(1461, 1444, 1453);
            s.store_div_from_scalar(1434, 1.0, 1460);
            s.store_div_from_scalar(1435, 1.0, 1461);
            s.store_div_from_scalar_add_ad(1462, 1.0, A::offset(s.ad_value(1434), 1.0), s.ad_value(1435));
            s.store_div_square_rhs(1433, 253, 1447);
            s.store_div_scaled_offset_numerator(1430, s.ad_value(1460), 1.0, 1.0, A::offset(s.ad_value(1461), 1.0), 1.0);
            s.store_ln(1431, 1430);
        }

        s.b[1625] = (s.v[1431] > 1e-8);
        s.store_scalar(1625, if s.b[1625] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1625]) {
            s.store_div_scaled_product_offset_denominator(1432, s.ad_value(1431), A::offset(s.ad_value(1430), 1.0), 2.0, s.ad_value(1430), (-1.0), 1.0);
        }

        if (s.b[1608] && (!s.b[1625])) {
            s.store_scaled_offset(1432, 1431, 2.0, 2.0);
        }

        if s.b[1608] {
            s.store_mul_sub_rhs(1463, 1462, 1458, 1459);
            s.store_square(1464, 1463);
            s.store_add_scaled_product_indices(1436, 1458, 1.0, 1463, 1434, (-1.0));
        }

    }

    pub(super) fn stamp_reactive_block_20(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1608] {
            s.store_add_scaled_product_indices(1437, 1459, 1.0, 1463, 1435, 1.0);
            s.store_div_from_scalar_offset_input(1342, 1.0, 1460, 1.0);
            s.store_div_from_scalar_offset_input(1343, 1.0, 1461, 1.0);
            s.store_offset_ln_ad(1345, A::div_scaled_product(A::add_scaled_product(s.ad_value(1460), 1.0, s.ad_value(1461), s.ad_value(1343), 1.0), s.ad_value(1432), 1.0, s.ad_value(1433), 1.0), 3.0);
            s.store_offset_ln_ad(1346, A::div_scaled_product(A::add_scaled_product(s.ad_value(1461), 1.0, s.ad_value(1460), s.ad_value(1342), 1.0), s.ad_value(1432), 1.0, s.ad_value(1433), 1.0), 3.0);
        }

        s.b[1626] = (((s.v[1345] - s.v[1436]) * 0.3333333333333) < 80.0);
        s.store_scalar(1626, if s.b[1626] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1626]) {
            s.store_ln_one_plus_exp_ad(1344, A::sub_scaled_inputs(s.ad_value(1345), 0.3333333333333, s.ad_value(1436), 0.3333333333333));
        }

        if (s.b[1608] && (!s.b[1626])) {
            s.store_scaled_sub(1344, 1345, 1436, 0.3333333333333);
        }

        if s.b[1608] {
            s.store_sub_scaled_inputs(1349, 1345, 1.0, 1344, 3.0);
        }

        s.b[1627] = (((s.v[1346] - s.v[1437]) * 0.3333333333333) < 80.0);
        s.store_scalar(1627, if s.b[1627] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1627]) {
            s.store_ln_one_plus_exp_ad(1344, A::sub_scaled_inputs(s.ad_value(1346), 0.3333333333333, s.ad_value(1437), 0.3333333333333));
        }

        if (s.b[1608] && (!s.b[1627])) {
            s.store_scaled_sub(1344, 1346, 1437, 0.3333333333333);
        }

        if s.b[1608] {
            s.store_sub_scaled_inputs(1350, 1346, 1.0, 1344, 3.0);
            s.store_mul_add_scaled_product_rhs(1347, 1342, s.ad_value(1350), 1.0, s.ad_value(1460), s.ad_value(1458), 1.0);
            s.store_mul_add_scaled_product_rhs(1348, 1343, s.ad_value(1349), 1.0, s.ad_value(1461), s.ad_value(1459), 1.0);
        }

        s.b[1628] = (((s.v[1345] - s.v[1347]) * 0.3333333333333) < 80.0);
        s.store_scalar(1628, if s.b[1628] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1628]) {
            s.store_ln_one_plus_exp_ad(1344, A::sub_scaled_inputs(s.ad_value(1345), 0.3333333333333, s.ad_value(1347), 0.3333333333333));
        }

        if (s.b[1608] && (!s.b[1628])) {
            s.store_scaled_sub(1344, 1345, 1347, 0.3333333333333);
        }

        if s.b[1608] {
            s.store_sub_scaled_inputs(1349, 1345, 1.0, 1344, 3.0);
        }

        s.b[1629] = (((s.v[1346] - s.v[1348]) * 0.3333333333333) < 80.0);
        s.store_scalar(1629, if s.b[1629] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1629]) {
            s.store_ln_one_plus_exp_ad(1344, A::sub_scaled_inputs(s.ad_value(1346), 0.3333333333333, s.ad_value(1348), 0.3333333333333));
        }

        if (s.b[1608] && (!s.b[1629])) {
            s.store_scaled_sub(1344, 1346, 1348, 0.3333333333333);
        }

        if s.b[1608] {
            s.store_sub_scaled_inputs(1350, 1346, 1.0, 1344, 3.0);
            s.store_sub(1465, 1458, 1349);
            s.store_sub(1469, 1459, 1350);
            s.store_scalar(1356, 0.0);
            s.store_scalar(1359, 0.0);
            s.store_mul(1351, 1460, 1465);
        }

        s.b[1630] = ((s.v[1458] - s.v[1465]) < 80.0);
        s.store_scalar(1630, if s.b[1630] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1630]) {
            s.store_exp_sub(1342, 1458, 1465);
        }

        if (s.b[1608] && (!s.b[1630])) {
            s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1342, A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);
        }

        if s.b[1608] {
            s.store_mul(1352, 1433, 1342);
            s.store_sub_square_lhs(1353, 1351, 1352);
            s.store_add_scaled_product_indices(1354, 1352, 1.0, 1460, 1351, 2.0);
            s.store_add_scaled_product_indices(1355, 1352, (-1.0), 1460, 1460, 2.0);
        }

        s.b[1631] = (s.v[1353] < (-0.005));
        s.store_scalar(1631, if s.b[1631] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1631]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
            s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);
            s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        s.b[1632] = (s.v[1353] > 0.005);
        s.store_scalar(1632, if s.b[1632] { 1.0 } else { 0.0 });

        if ((s.b[1608] && (!s.b[1631])) && s.b[1632]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_exp_neg_input(1359, 1356);
            s.store_div_scaled_product_offset_rhs(1357, s.ad_value(1356), s.ad_value(1359), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0);
            s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);
            s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        if ((s.b[1608] && (!s.b[1631])) && (!s.b[1632])) {
            s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1344, 1353, 1.0, 1353, 1.0, 1353, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);
            s.store_offset_mul(1357, 1353, 1344, 2.0);
            s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1342, 1353, 1.0, 1353, 1.0, 1353, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);
            s.store_mul(1358, 1354, 1342);
            s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1343, 1353, 1.0, 1353, 1.0, 1353, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(1360, 1355, 1342, 1.0, A::square(s.ad_value(1354)), 1343, (-1.0));
            s.store_scaled_mul(1363, 1354, 1344, (-0.5));
            s.store_add_scaled_product_value_ad(1364, A::mul3_scaled_output(s.ad_value(1354), s.ad_value(1354), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 2.0, A::scale(s.ad_value(1353), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1355, 1344, (-0.5));
        }

        s.b[1633] = (s.v[1353] > 0.005);
        s.store_scalar(1633, if s.b[1633] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1633]) {
            s.store_div_scaled_inputs_mixed_ia(1343, 1353, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0);
            s.store_mul(1361, 1343, 1359);
            s.store_sub_ln_lhs(1362, 1343, 1356);
        }

        s.b[1634] = (s.v[1353] < (-0.005));
        s.store_scalar(1634, if s.b[1634] { 1.0 } else { 0.0 });

        if ((s.b[1608] && (!s.b[1633])) && s.b[1634]) {
            s.store_sin_scaled_input(1343, 1356, 0.5);
            s.store_div_scaled_inputs_square_rhs(1361, 1353, -1.0, 1343, 1.0);
            s.store_ln(1362, 1361);
        }

        if ((s.b[1608] && (!s.b[1633])) && (!s.b[1634])) {
            s.store_sub_from_scalar_ad(1361, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(1362, 1361);
        }

        s.b[1635] = (((1.01 * s.v[1351]) + s.v[1357]) > 0.0);
        s.store_scalar(1635, if s.b[1635] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1635]) {
            s.store_add(1365, 1351, 1357);
            s.store_add(1366, 1460, 1358);
            s.copy_ad(1367, 1360);
        }

        if (s.b[1608] && (!s.b[1635])) {
            s.store_div_from_scalar_sub_ad(1343, 1.0, s.ad_value(1351), s.ad_value(1357));
            s.store_sub(1344, 1358, 1460);
            s.store_mul_sub_lhs(1365, 1352, 1361, 1343);
            s.store_mul_ad_lhs(1366, A::add_scaled_value_products(s.ad_value(1352), (-1.0), s.ad_value(1344), s.ad_value(1365), 1.0, s.ad_value(1363), s.ad_value(1361), (-1.0)), 1343);
            s.store_mul_ad_lhs(1367, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1360), s.ad_value(1365), 1.0, s.ad_value(1344), s.ad_value(1366), 2.0), 1.0, s.ad_value(1352), 1.0, A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361), (-1.0)), 1343);
        }

        s.b[1636] = (s.v[1365] > 0.0);
        s.store_scalar(1636, if s.b[1636] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1636]) {
            s.store_ln(1368, 1365);
            s.store_div_from_scalar(1342, 1.0, 1365);
            s.store_mul(1369, 1366, 1342);
            s.store_add_scaled_square_product_indices(1370, 1369, (-1.0), 1367, 1342, 1.0);
        }

        if (s.b[1608] && (!s.b[1636])) {
            s.store_add_offset_lhs_ad_rhs(1368, 1351, 0.6931471805599, A::ln_scaled_input(s.ad_value(1351), -1.0));
            s.store_div_from_scalar(1342, 1.0, 1465);
            s.store_add(1369, 1460, 1342);
            s.store_mul_neg_lhs(1370, 1342, 1342);
        }

        if s.b[1608] {
            s.store_sub_add_scaled_inputs4_lhs_indices(1371, 1459, 1.0, 1458, (-1.0), 1465, 1.0, 1368, 2.0, 1362);
            s.store_sub_ad_lhs(1372, A::scale_offset(s.ad_value(1369), 2.0, 1.0), 1363);
            s.store_sub_scaled_inputs(1373, 1370, 2.0, 1364, 1.0);
            s.store_add_scaled_product_indices(1374, 1351, 1.0, 1461, 1371, 1.0);
            s.store_add_scaled_product_indices(1375, 1460, 1.0, 1461, 1372, 1.0);
            s.store_mul(1376, 1461, 1373);
            s.store_add_scaled_product_indices(1377, 1352, (-1.0), 1374, 1365, 1.0);
            s.store_add_ad_lhs(1378, A::add_scaled_products(s.ad_value(1375), s.ad_value(1365), 1.0, s.ad_value(1374), s.ad_value(1366), 1.0), 1352);
            s.store_sub_ad_lhs(1379, A::add_scaled_products3(s.ad_value(1376), s.ad_value(1365), 1.0, s.ad_value(1375), s.ad_value(1366), 2.0, s.ad_value(1374), s.ad_value(1367), 1.0), 1352);
            s.store_add_scaled_square_product_indices(1388, 1378, 1.0, 1377, 1379, (-0.5));
            s.store_div_scaled_product3_mixed_iiia(1380, 1377, 1378, 1388, -1.0, A::offset(A::square(s.ad_value(1388)), 1e-200), 1.0);
            s.store_add(1465, 1465, 1380);
            s.store_mul(1351, 1460, 1465);
            s.store_mul(1381, 1461, 1469);
            s.store_add(1374, 1351, 1381);
            s.store_offset_scaled(1382, 1374, 0.065345483024, 1.0);
            s.store_add_scaled_product_value_ad(1383, A::scale_offset(s.ad_value(1374), 8.5797362674, 39.478417604), 1.0, 1351, 1381, 1.0);
            s.store_add_scaled_product_indices(1384, 1374, (2.0 * 39.478417604), 1351, 1381, 39.478417604);
            s.store_sqrt_add_scaled_square_product(1385, 1383, 1.0, 1382, 1384, (-4.0));
            s.store_div_scaled_inputs2_indices(1353, 1385, 1.0, 1383, (-1.0), 1382, 2.0);
            s.store_sub_square_lhs(1386, 1351, 1353);
        }

        s.b[1637] = (s.v[1386] > 0.0);
        s.store_scalar(1637, if s.b[1637] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1637]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(1377, 1386, A::ln(A::div(s.ad_value(1386), s.ad_value(1433))), 1.0, s.ad_value(1458), (-1.0), s.ad_value(1465), 1.0, 0.0);
            s.store_add_scaled_product_indices(1378, 1386, 1.0, 1460, 1351, 2.0);
            s.store_add_scaled_inputs3_indices(1387, 1458, 1.0, 1465, (-1.0), 1345, -1.0);
        }

        s.b[1638] = ((((s.v[1377] < 0.0) && (s.v[1378] > 0.0)) && (((s.v[1387] + 2.3025850929941) + ((s.v[1460]) as f64).ln()) > 0.0)) || (s.v[1387] > 1.0));
        s.store_scalar(1638, if s.b[1638] { 1.0 } else { 0.0 });

        if ((s.b[1608] && s.b[1637]) && s.b[1638]) {
            s.store_sub_div_rhs_indices(1465, 1465, 1377, 1378);
        }

        if s.b[1608] {
            s.store_mul(1351, 1460, 1465);
            s.store_mul(1381, 1461, 1469);
            s.store_add(1374, 1351, 1381);
            s.store_offset_scaled(1382, 1374, 0.065345483024, 1.0);
            s.store_add_scaled_product_value_ad(1383, A::scale_offset(s.ad_value(1374), 8.5797362674, 39.478417604), 1.0, 1351, 1381, 1.0);
            s.store_add_scaled_product_indices(1384, 1374, (2.0 * 39.478417604), 1351, 1381, 39.478417604);
            s.store_sqrt_add_scaled_square_product(1385, 1383, 1.0, 1382, 1384, (-4.0));
            s.store_div_scaled_inputs2_indices(1353, 1385, 1.0, 1383, (-1.0), 1382, 2.0);
        }

        s.b[1639] = (s.v[1353] < (-0.005));
        s.store_scalar(1639, if s.b[1639] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1639]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
            s.store_div_scaled_inputs2_mixed_iai(1358, 1353, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 0.25, 1353, 1.0);
        }

        s.b[1640] = (s.v[1353] > 0.005);
        s.store_scalar(1640, if s.b[1640] { 1.0 } else { 0.0 });

        if ((s.b[1608] && (!s.b[1639])) && s.b[1640]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
        }

    }

    pub(super) fn stamp_reactive_block_21(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1608] && (!s.b[1639])) && s.b[1640]) {
            s.store_exp_neg_input(1359, 1356);
            s.store_div_scaled_product_offset_rhs(1357, s.ad_value(1356), s.ad_value(1359), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0);
            s.store_div_scaled_inputs2_mixed_iai(1358, 1353, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 0.25, 1353, 1.0);
        }

        if ((s.b[1608] && (!s.b[1639])) && (!s.b[1640])) {
            s.store_offset_ad(1357, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);
            s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1358, 1353, 1.0, 1353, 1.0, 1353, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);
        }

        if s.b[1608] {
            s.store_sub_ad_rhs(1353, 1353, A::div_scaled_inputs2(A::add_scaled_products(s.ad_value(1374), s.ad_value(1357), 1.0, s.ad_value(1351), s.ad_value(1381), 1.0), 1.0, s.ad_value(1353), 1.0, A::offset(A::mul(s.ad_value(1374), s.ad_value(1358)), 1.0), 1.0));
            s.store_sub_square_lhs(1386, 1351, 1353);
        }

        s.b[1641] = (s.v[1386] > 0.0);
        s.store_scalar(1641, if s.b[1641] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1641]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(1377, 1386, A::ln(A::div(s.ad_value(1386), s.ad_value(1433))), 1.0, s.ad_value(1458), (-1.0), s.ad_value(1465), 1.0, 0.0);
            s.store_add_scaled_product_indices(1378, 1386, 1.0, 1460, 1351, 2.0);
            s.store_add_scaled_inputs3_indices(1387, 1458, 1.0, 1465, (-1.0), 1345, -1.0);
        }

        s.b[1642] = ((((s.v[1377] < 0.0) && (s.v[1378] > 0.0)) && (((s.v[1387] + 2.3025850929941) + ((s.v[1460]) as f64).ln()) > 0.0)) || (s.v[1387] > 1.0));
        s.store_scalar(1642, if s.b[1642] { 1.0 } else { 0.0 });

        if ((s.b[1608] && s.b[1641]) && s.b[1642]) {
            s.store_sub_div_rhs_indices(1465, 1465, 1377, 1378);
        }

        if s.b[1608] {
            s.store_mul(1351, 1460, 1465);
        }

        s.b[1643] = ((s.v[1458] - s.v[1465]) < 80.0);
        s.store_scalar(1643, if s.b[1643] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1643]) {
            s.store_exp_sub(1342, 1458, 1465);
        }

        if (s.b[1608] && (!s.b[1643])) {
            s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1342, A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);
        }

        if s.b[1608] {
            s.store_mul(1352, 1433, 1342);
            s.store_sub_square_lhs(1353, 1351, 1352);
            s.store_add_scaled_product_indices(1354, 1352, 1.0, 1460, 1351, 2.0);
            s.store_add_scaled_product_indices(1355, 1352, (-1.0), 1460, 1460, 2.0);
        }

        s.b[1644] = (s.v[1353] < (-0.005));
        s.store_scalar(1644, if s.b[1644] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1644]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
            s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);
            s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        s.b[1645] = (s.v[1353] > 0.005);
        s.store_scalar(1645, if s.b[1645] { 1.0 } else { 0.0 });

        if ((s.b[1608] && (!s.b[1644])) && s.b[1645]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_exp_neg_input(1359, 1356);
            s.store_div_scaled_product_offset_rhs(1357, s.ad_value(1356), s.ad_value(1359), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0);
            s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);
            s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        if ((s.b[1608] && (!s.b[1644])) && (!s.b[1645])) {
            s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1344, 1353, 1.0, 1353, 1.0, 1353, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);
            s.store_offset_mul(1357, 1353, 1344, 2.0);
            s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1342, 1353, 1.0, 1353, 1.0, 1353, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);
            s.store_mul(1358, 1354, 1342);
            s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1343, 1353, 1.0, 1353, 1.0, 1353, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(1360, 1355, 1342, 1.0, A::square(s.ad_value(1354)), 1343, (-1.0));
            s.store_scaled_mul(1363, 1354, 1344, (-0.5));
            s.store_add_scaled_product_value_ad(1364, A::mul3_scaled_output(s.ad_value(1354), s.ad_value(1354), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 2.0, A::scale(s.ad_value(1353), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1355, 1344, (-0.5));
        }

        s.b[1646] = (s.v[1353] > 0.005);
        s.store_scalar(1646, if s.b[1646] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1646]) {
            s.store_div_scaled_inputs_mixed_ia(1343, 1353, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0);
            s.store_mul(1361, 1343, 1359);
            s.store_sub_ln_lhs(1362, 1343, 1356);
        }

        s.b[1647] = (s.v[1353] < (-0.005));
        s.store_scalar(1647, if s.b[1647] { 1.0 } else { 0.0 });

        if ((s.b[1608] && (!s.b[1646])) && s.b[1647]) {
            s.store_sin_scaled_input(1343, 1356, 0.5);
            s.store_div_scaled_inputs_square_rhs(1361, 1353, -1.0, 1343, 1.0);
            s.store_ln(1362, 1361);
        }

        if ((s.b[1608] && (!s.b[1646])) && (!s.b[1647])) {
            s.store_sub_from_scalar_ad(1361, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(1362, 1361);
        }

        s.b[1648] = (((1.01 * s.v[1351]) + s.v[1357]) > 0.0);
        s.store_scalar(1648, if s.b[1648] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1648]) {
            s.store_add(1365, 1351, 1357);
            s.store_add(1366, 1460, 1358);
            s.copy_ad(1367, 1360);
        }

        if (s.b[1608] && (!s.b[1648])) {
            s.store_div_from_scalar_sub_ad(1343, 1.0, s.ad_value(1351), s.ad_value(1357));
            s.store_sub(1344, 1358, 1460);
            s.store_mul_sub_lhs(1365, 1352, 1361, 1343);
            s.store_mul_ad_lhs(1366, A::add_scaled_value_products(s.ad_value(1352), (-1.0), s.ad_value(1344), s.ad_value(1365), 1.0, s.ad_value(1363), s.ad_value(1361), (-1.0)), 1343);
            s.store_mul_ad_lhs(1367, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1360), s.ad_value(1365), 1.0, s.ad_value(1344), s.ad_value(1366), 2.0), 1.0, s.ad_value(1352), 1.0, A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361), (-1.0)), 1343);
        }

        s.b[1649] = (s.v[1365] > 0.0);
        s.store_scalar(1649, if s.b[1649] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1649]) {
            s.store_ln(1368, 1365);
            s.store_div_from_scalar(1342, 1.0, 1365);
            s.store_mul(1369, 1366, 1342);
            s.store_add_scaled_square_product_indices(1370, 1369, (-1.0), 1367, 1342, 1.0);
        }

        if (s.b[1608] && (!s.b[1649])) {
            s.store_add_offset_lhs_ad_rhs(1368, 1351, 0.6931471805599, A::ln_scaled_input(s.ad_value(1351), -1.0));
            s.store_div_from_scalar(1342, 1.0, 1465);
            s.store_add(1369, 1460, 1342);
            s.store_mul_neg_lhs(1370, 1342, 1342);
        }

        if s.b[1608] {
            s.store_sub_add_scaled_inputs4_lhs_indices(1371, 1459, 1.0, 1458, (-1.0), 1465, 1.0, 1368, 2.0, 1362);
            s.store_sub_ad_lhs(1372, A::scale_offset(s.ad_value(1369), 2.0, 1.0), 1363);
            s.store_sub_scaled_inputs(1373, 1370, 2.0, 1364, 1.0);
            s.store_add_scaled_product_indices(1374, 1351, 1.0, 1461, 1371, 1.0);
            s.store_add_scaled_product_indices(1375, 1460, 1.0, 1461, 1372, 1.0);
            s.store_mul(1376, 1461, 1373);
            s.store_add_scaled_product_indices(1377, 1352, (-1.0), 1374, 1365, 1.0);
            s.store_add_ad_lhs(1378, A::add_scaled_products(s.ad_value(1375), s.ad_value(1365), 1.0, s.ad_value(1374), s.ad_value(1366), 1.0), 1352);
            s.store_sub_ad_lhs(1379, A::add_scaled_products3(s.ad_value(1376), s.ad_value(1365), 1.0, s.ad_value(1375), s.ad_value(1366), 2.0, s.ad_value(1374), s.ad_value(1367), 1.0), 1352);
            s.store_add_scaled_square_product_indices(1388, 1378, 1.0, 1377, 1379, (-0.5));
            s.store_div_scaled_product3_mixed_iiia(1380, 1377, 1378, 1388, -1.0, A::offset(A::square(s.ad_value(1388)), 1e-200), 1.0);
            s.store_add(1465, 1465, 1380);
            s.store_mul(1351, 1460, 1465);
        }

        s.b[1650] = ((s.v[1458] - s.v[1465]) < 80.0);
        s.store_scalar(1650, if s.b[1650] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1650]) {
            s.store_exp_sub(1342, 1458, 1465);
        }

        if (s.b[1608] && (!s.b[1650])) {
            s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1342, A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);
        }

        if s.b[1608] {
            s.store_mul(1352, 1433, 1342);
            s.store_sub_square_lhs(1353, 1351, 1352);
            s.store_add_scaled_product_indices(1354, 1352, 1.0, 1460, 1351, 2.0);
            s.store_add_scaled_product_indices(1355, 1352, (-1.0), 1460, 1460, 2.0);
        }

        s.b[1651] = (s.v[1353] < (-0.005));
        s.store_scalar(1651, if s.b[1651] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1651]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
            s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);
            s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        s.b[1652] = (s.v[1353] > 0.005);
        s.store_scalar(1652, if s.b[1652] { 1.0 } else { 0.0 });

        if ((s.b[1608] && (!s.b[1651])) && s.b[1652]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_exp_neg_input(1359, 1356);
            s.store_div_scaled_product_offset_rhs(1357, s.ad_value(1356), s.ad_value(1359), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0);
            s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);
            s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        if ((s.b[1608] && (!s.b[1651])) && (!s.b[1652])) {
            s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1344, 1353, 1.0, 1353, 1.0, 1353, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);
            s.store_offset_mul(1357, 1353, 1344, 2.0);
            s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1342, 1353, 1.0, 1353, 1.0, 1353, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);
            s.store_mul(1358, 1354, 1342);
            s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1343, 1353, 1.0, 1353, 1.0, 1353, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(1360, 1355, 1342, 1.0, A::square(s.ad_value(1354)), 1343, (-1.0));
            s.store_scaled_mul(1363, 1354, 1344, (-0.5));
            s.store_add_scaled_product_value_ad(1364, A::mul3_scaled_output(s.ad_value(1354), s.ad_value(1354), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 2.0, A::scale(s.ad_value(1353), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1355, 1344, (-0.5));
        }

        s.b[1653] = (s.v[1353] > 0.005);
        s.store_scalar(1653, if s.b[1653] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1653]) {
            s.store_div_scaled_inputs_mixed_ia(1343, 1353, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0);
            s.store_mul(1361, 1343, 1359);
            s.store_sub_ln_lhs(1362, 1343, 1356);
        }

        s.b[1654] = (s.v[1353] < (-0.005));
        s.store_scalar(1654, if s.b[1654] { 1.0 } else { 0.0 });

        if ((s.b[1608] && (!s.b[1653])) && s.b[1654]) {
            s.store_sin_scaled_input(1343, 1356, 0.5);
            s.store_div_scaled_inputs_square_rhs(1361, 1353, -1.0, 1343, 1.0);
            s.store_ln(1362, 1361);
        }

    }

    pub(super) fn stamp_reactive_block_22(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1608] && (!s.b[1653])) && (!s.b[1654])) {
            s.store_sub_from_scalar_ad(1361, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(1362, 1361);
        }

        s.b[1655] = (((1.01 * s.v[1351]) + s.v[1357]) > 0.0);
        s.store_scalar(1655, if s.b[1655] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1655]) {
            s.store_add(1365, 1351, 1357);
            s.store_add(1366, 1460, 1358);
            s.copy_ad(1367, 1360);
        }

        if (s.b[1608] && (!s.b[1655])) {
            s.store_div_from_scalar_sub_ad(1343, 1.0, s.ad_value(1351), s.ad_value(1357));
            s.store_sub(1344, 1358, 1460);
            s.store_mul_sub_lhs(1365, 1352, 1361, 1343);
            s.store_mul_ad_lhs(1366, A::add_scaled_value_products(s.ad_value(1352), (-1.0), s.ad_value(1344), s.ad_value(1365), 1.0, s.ad_value(1363), s.ad_value(1361), (-1.0)), 1343);
            s.store_mul_ad_lhs(1367, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1360), s.ad_value(1365), 1.0, s.ad_value(1344), s.ad_value(1366), 2.0), 1.0, s.ad_value(1352), 1.0, A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361), (-1.0)), 1343);
        }

        s.b[1656] = (s.v[1365] > 0.0);
        s.store_scalar(1656, if s.b[1656] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1656]) {
            s.store_ln(1368, 1365);
            s.store_div_from_scalar(1342, 1.0, 1365);
            s.store_mul(1369, 1366, 1342);
            s.store_add_scaled_square_product_indices(1370, 1369, (-1.0), 1367, 1342, 1.0);
        }

        if (s.b[1608] && (!s.b[1656])) {
            s.store_add_offset_lhs_ad_rhs(1368, 1351, 0.6931471805599, A::ln_scaled_input(s.ad_value(1351), -1.0));
            s.store_div_from_scalar(1342, 1.0, 1465);
            s.store_add(1369, 1460, 1342);
            s.store_mul_neg_lhs(1370, 1342, 1342);
        }

        if s.b[1608] {
            s.store_sub_add_scaled_inputs4_lhs_indices(1371, 1459, 1.0, 1458, (-1.0), 1465, 1.0, 1368, 2.0, 1362);
            s.store_sub_ad_lhs(1372, A::scale_offset(s.ad_value(1369), 2.0, 1.0), 1363);
            s.store_sub_scaled_inputs(1373, 1370, 2.0, 1364, 1.0);
            s.store_add_scaled_product_indices(1374, 1351, 1.0, 1461, 1371, 1.0);
            s.store_add_scaled_product_indices(1375, 1460, 1.0, 1461, 1372, 1.0);
            s.store_mul(1376, 1461, 1373);
            s.store_add_scaled_product_indices(1377, 1352, (-1.0), 1374, 1365, 1.0);
            s.store_add_ad_lhs(1378, A::add_scaled_products(s.ad_value(1375), s.ad_value(1365), 1.0, s.ad_value(1374), s.ad_value(1366), 1.0), 1352);
            s.store_sub_ad_lhs(1379, A::add_scaled_products3(s.ad_value(1376), s.ad_value(1365), 1.0, s.ad_value(1375), s.ad_value(1366), 2.0, s.ad_value(1374), s.ad_value(1367), 1.0), 1352);
            s.store_add_scaled_square_product_indices(1388, 1378, 1.0, 1377, 1379, (-0.5));
            s.store_div_scaled_product3_mixed_iiia(1380, 1377, 1378, 1388, -1.0, A::offset(A::square(s.ad_value(1388)), 1e-200), 1.0);
            s.store_add(1465, 1465, 1380);
        }

        s.b[1657] = (p.p10 == 1.0);
        s.store_scalar(1657, if s.b[1657] { 1.0 } else { 0.0 });

        s.b[1658] = (((s.v[1380]) as f64).abs() > 0.01);
        s.store_scalar(1658, if s.b[1658] { 1.0 } else { 0.0 });

        if ((s.b[1608] && s.b[1657]) && s.b[1658]) {
            s.store_mul(1351, 1460, 1465);
        }

        s.b[1659] = ((s.v[1458] - s.v[1465]) < 80.0);
        s.store_scalar(1659, if s.b[1659] { 1.0 } else { 0.0 });

        if (((s.b[1608] && s.b[1657]) && s.b[1658]) && s.b[1659]) {
            s.store_exp_sub(1342, 1458, 1465);
        }

        if (((s.b[1608] && s.b[1657]) && s.b[1658]) && (!s.b[1659])) {
            s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1342, A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);
        }

        if ((s.b[1608] && s.b[1657]) && s.b[1658]) {
            s.store_mul(1352, 1433, 1342);
            s.store_sub_square_lhs(1353, 1351, 1352);
            s.store_add_scaled_product_indices(1354, 1352, 1.0, 1460, 1351, 2.0);
            s.store_add_scaled_product_indices(1355, 1352, (-1.0), 1460, 1460, 2.0);
        }

        s.b[1660] = (s.v[1353] < (-0.005));
        s.store_scalar(1660, if s.b[1660] { 1.0 } else { 0.0 });

        if (((s.b[1608] && s.b[1657]) && s.b[1658]) && s.b[1660]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
            s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);
            s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        s.b[1661] = (s.v[1353] > 0.005);
        s.store_scalar(1661, if s.b[1661] { 1.0 } else { 0.0 });

        if ((((s.b[1608] && s.b[1657]) && s.b[1658]) && (!s.b[1660])) && s.b[1661]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_exp_neg_input(1359, 1356);
            s.store_div_scaled_product_offset_rhs(1357, s.ad_value(1356), s.ad_value(1359), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0);
            s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);
            s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        if ((((s.b[1608] && s.b[1657]) && s.b[1658]) && (!s.b[1660])) && (!s.b[1661])) {
            s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1344, 1353, 1.0, 1353, 1.0, 1353, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);
            s.store_offset_mul(1357, 1353, 1344, 2.0);
            s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1342, 1353, 1.0, 1353, 1.0, 1353, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);
            s.store_mul(1358, 1354, 1342);
            s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1343, 1353, 1.0, 1353, 1.0, 1353, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(1360, 1355, 1342, 1.0, A::square(s.ad_value(1354)), 1343, (-1.0));
            s.store_scaled_mul(1363, 1354, 1344, (-0.5));
            s.store_add_scaled_product_value_ad(1364, A::mul3_scaled_output(s.ad_value(1354), s.ad_value(1354), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 2.0, A::scale(s.ad_value(1353), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1355, 1344, (-0.5));
        }

        s.b[1662] = (s.v[1353] > 0.005);
        s.store_scalar(1662, if s.b[1662] { 1.0 } else { 0.0 });

        if (((s.b[1608] && s.b[1657]) && s.b[1658]) && s.b[1662]) {
            s.store_div_scaled_inputs_mixed_ia(1343, 1353, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0);
            s.store_mul(1361, 1343, 1359);
            s.store_sub_ln_lhs(1362, 1343, 1356);
        }

        s.b[1663] = (s.v[1353] < (-0.005));
        s.store_scalar(1663, if s.b[1663] { 1.0 } else { 0.0 });

        if ((((s.b[1608] && s.b[1657]) && s.b[1658]) && (!s.b[1662])) && s.b[1663]) {
            s.store_sin_scaled_input(1343, 1356, 0.5);
            s.store_div_scaled_inputs_square_rhs(1361, 1353, -1.0, 1343, 1.0);
            s.store_ln(1362, 1361);
        }

        if ((((s.b[1608] && s.b[1657]) && s.b[1658]) && (!s.b[1662])) && (!s.b[1663])) {
            s.store_sub_from_scalar_ad(1361, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(1362, 1361);
        }

        s.b[1664] = (((1.01 * s.v[1351]) + s.v[1357]) > 0.0);
        s.store_scalar(1664, if s.b[1664] { 1.0 } else { 0.0 });

        if (((s.b[1608] && s.b[1657]) && s.b[1658]) && s.b[1664]) {
            s.store_add(1365, 1351, 1357);
            s.store_add(1366, 1460, 1358);
            s.copy_ad(1367, 1360);
        }

        if (((s.b[1608] && s.b[1657]) && s.b[1658]) && (!s.b[1664])) {
            s.store_div_from_scalar_sub_ad(1343, 1.0, s.ad_value(1351), s.ad_value(1357));
            s.store_sub(1344, 1358, 1460);
            s.store_mul_sub_lhs(1365, 1352, 1361, 1343);
            s.store_mul_ad_lhs(1366, A::add_scaled_value_products(s.ad_value(1352), (-1.0), s.ad_value(1344), s.ad_value(1365), 1.0, s.ad_value(1363), s.ad_value(1361), (-1.0)), 1343);
            s.store_mul_ad_lhs(1367, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1360), s.ad_value(1365), 1.0, s.ad_value(1344), s.ad_value(1366), 2.0), 1.0, s.ad_value(1352), 1.0, A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361), (-1.0)), 1343);
        }

        s.b[1665] = (s.v[1365] > 0.0);
        s.store_scalar(1665, if s.b[1665] { 1.0 } else { 0.0 });

        if (((s.b[1608] && s.b[1657]) && s.b[1658]) && s.b[1665]) {
            s.store_ln(1368, 1365);
            s.store_div_from_scalar(1342, 1.0, 1365);
            s.store_mul(1369, 1366, 1342);
            s.store_add_scaled_square_product_indices(1370, 1369, (-1.0), 1367, 1342, 1.0);
        }

        if (((s.b[1608] && s.b[1657]) && s.b[1658]) && (!s.b[1665])) {
            s.store_add_offset_lhs_ad_rhs(1368, 1351, 0.6931471805599, A::ln_scaled_input(s.ad_value(1351), -1.0));
            s.store_div_from_scalar(1342, 1.0, 1465);
            s.store_add(1369, 1460, 1342);
            s.store_mul_neg_lhs(1370, 1342, 1342);
        }

        if ((s.b[1608] && s.b[1657]) && s.b[1658]) {
            s.store_sub_add_scaled_inputs4_lhs_indices(1371, 1459, 1.0, 1458, (-1.0), 1465, 1.0, 1368, 2.0, 1362);
            s.store_sub_ad_lhs(1372, A::scale_offset(s.ad_value(1369), 2.0, 1.0), 1363);
            s.store_sub_scaled_inputs(1373, 1370, 2.0, 1364, 1.0);
            s.store_add_scaled_product_indices(1374, 1351, 1.0, 1461, 1371, 1.0);
            s.store_add_scaled_product_indices(1375, 1460, 1.0, 1461, 1372, 1.0);
            s.store_mul(1376, 1461, 1373);
            s.store_add_scaled_product_indices(1377, 1352, (-1.0), 1374, 1365, 1.0);
            s.store_add_ad_lhs(1378, A::add_scaled_products(s.ad_value(1375), s.ad_value(1365), 1.0, s.ad_value(1374), s.ad_value(1366), 1.0), 1352);
            s.store_sub_ad_lhs(1379, A::add_scaled_products3(s.ad_value(1376), s.ad_value(1365), 1.0, s.ad_value(1375), s.ad_value(1366), 2.0, s.ad_value(1374), s.ad_value(1367), 1.0), 1352);
            s.store_add_scaled_square_product_indices(1388, 1378, 1.0, 1377, 1379, (-0.5));
            s.store_div_scaled_product3_mixed_iiia(1380, 1377, 1378, 1388, -1.0, A::offset(A::square(s.ad_value(1388)), 1e-200), 1.0);
            s.store_add(1465, 1465, 1380);
        }

        if s.b[1608] {
            s.store_mul(1467, 1460, 1465);
        }

        s.b[1666] = ((s.v[1458] - s.v[1465]) < 80.0);
        s.store_scalar(1666, if s.b[1666] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1666]) {
            s.store_exp_sub(1342, 1458, 1465);
        }

        if (s.b[1608] && (!s.b[1666])) {
            s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1342, A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);
        }

        if s.b[1608] {
            s.store_mul(1471, 1433, 1342);
            s.store_sub_square_lhs(1470, 1467, 1471);
        }

        s.b[1667] = (s.v[1471] <= 0.0);
        s.store_scalar(1667, if s.b[1667] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1667]) {
            s.store_scalar(1466, 1e-80);
            s.store_sub(1468, 1466, 1467);
            s.store_div(1469, 1468, 1461);
        }

        s.b[1668] = (s.v[1470] < (-0.005));
        s.store_scalar(1668, if s.b[1668] { 1.0 } else { 0.0 });

        if ((s.b[1608] && (!s.b[1667])) && s.b[1668]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1470));
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
        }

        s.b[1669] = (s.v[1470] > 0.005);
        s.store_scalar(1669, if s.b[1669] { 1.0 } else { 0.0 });

        if (((s.b[1608] && (!s.b[1667])) && (!s.b[1668])) && s.b[1669]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1470));
            s.store_exp_neg_input(1359, 1356);
            s.store_div_scaled_product_offset_rhs(1357, s.ad_value(1356), s.ad_value(1359), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0);
        }

        if (((s.b[1608] && (!s.b[1667])) && (!s.b[1668])) && (!s.b[1669])) {
            s.store_offset_ad(1357, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1470), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1470), 1.0, A::scale(s.ad_value(1470), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);
        }

        s.b[1670] = (((1.01 * s.v[1467]) + s.v[1357]) > 0.0);
        s.store_scalar(1670, if s.b[1670] { 1.0 } else { 0.0 });

        if ((s.b[1608] && (!s.b[1667])) && s.b[1670]) {
            s.store_add(1342, 1467, 1357);
        }

        s.b[1671] = ((s.v[1471] * s.v[1467]) < (((0.9 * s.v[1467]) * s.v[1467]) * s.v[1342]));
        s.store_scalar(1671, if s.b[1671] { 1.0 } else { 0.0 });

        if (((s.b[1608] && (!s.b[1667])) && s.b[1670]) && s.b[1671]) {
            s.store_offset_div(1466, 1471, 1342, 1e-80);
            s.store_sub(1468, 1466, 1467);
        }

    }

    pub(super) fn stamp_reactive_block_23(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[1608] && (!s.b[1667])) && s.b[1670]) && s.b[1671]) {
            s.store_div(1469, 1468, 1461);
        }

        s.b[1672] = (s.v[1470] > 0.005);
        s.store_scalar(1672, if s.b[1672] { 1.0 } else { 0.0 });

        if ((((s.b[1608] && (!s.b[1667])) && s.b[1670]) && (!s.b[1671])) && s.b[1672]) {
            s.store_sub_ad_lhs(1343, A::ln(A::div_scaled_inputs(s.ad_value(1470), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0)), 1356);
        }

        s.b[1673] = (s.v[1470] < (-0.005));
        s.store_scalar(1673, if s.b[1673] { 1.0 } else { 0.0 });

        if (((((s.b[1608] && (!s.b[1667])) && s.b[1670]) && (!s.b[1671])) && (!s.b[1672])) && s.b[1673]) {
            s.store_sin_scaled_input(1344, 1356, 0.5);
            s.store_ln_div_scaled_input_square_denominator(1343, 1470, -1.0, 1344, 1.0);
        }

        if (((((s.b[1608] && (!s.b[1667])) && s.b[1670]) && (!s.b[1671])) && (!s.b[1672])) && (!s.b[1673])) {
            s.store_ln_ad(1343, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1470), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1470), 1.0, A::scale(s.ad_value(1470), 0.0396825396825397), 0.05), 0.3333333333333)));
        }

        if (((s.b[1608] && (!s.b[1667])) && s.b[1670]) && (!s.b[1671])) {
            s.store_sub_add_scaled_inputs4_lhs_mixed_iiia(1469, 1459, 1.0, 1458, (-1.0), 1465, 1.0, A::ln(s.ad_value(1342)), 2.0, 1343);
            s.store_mul(1468, 1461, 1469);
            s.store_add(1466, 1467, 1468);
        }

        s.b[1674] = (s.v[1470] > 0.005);
        s.store_scalar(1674, if s.b[1674] { 1.0 } else { 0.0 });

        s.b[1675] = (((s.v[1465] - s.v[1458]) - s.v[1356]) < 80.0);
        s.store_scalar(1675, if s.b[1675] { 1.0 } else { 0.0 });

        if ((((s.b[1608] && (!s.b[1667])) && (!s.b[1670])) && s.b[1674]) && s.b[1675]) {
            s.store_exp_ad(1344, A::add_scaled_inputs3(s.ad_value(1465), 1.0, s.ad_value(1458), (-1.0), s.ad_value(1356), -1.0));
        }

        if ((((s.b[1608] && (!s.b[1667])) && (!s.b[1670])) && s.b[1674]) && (!s.b[1675])) {
            s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1344, A::add_scaled_inputs3(s.ad_value(1465), 1.0, s.ad_value(1458), (-1.0), s.ad_value(1356), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);
        }

        if (((s.b[1608] && (!s.b[1667])) && (!s.b[1670])) && s.b[1674]) {
            s.store_div(1343, 1344, 1433);
            s.store_div_scaled_product_denominator_ad(1342, 1470, 1343, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0);
        }

        s.b[1676] = (s.v[1470] < (-0.005));
        s.store_scalar(1676, if s.b[1676] { 1.0 } else { 0.0 });

        if ((((s.b[1608] && (!s.b[1667])) && (!s.b[1670])) && (!s.b[1674])) && s.b[1676]) {
            s.store_sin_scaled_input(1343, 1356, 0.5);
            s.store_div_scaled_value_by_product(1342, s.ad_value(1470), -1.0, A::square(s.ad_value(1343)), s.ad_value(1471), 1.0);
        }

        if ((((s.b[1608] && (!s.b[1667])) && (!s.b[1670])) && (!s.b[1674])) && (!s.b[1676])) {
            s.store_div_ad_lhs(1342, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1470), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1470), 1.0, A::scale(s.ad_value(1470), 0.0396825396825397), 0.05), 0.3333333333333)), 1471);
        }

        if ((s.b[1608] && (!s.b[1667])) && (!s.b[1670])) {
            s.store_offset_div_scaled_inputs2_mixed_iia(1466, 1467, 1.0, 1357, (-1.0), A::sub_from_scalar(1.0, s.ad_value(1342)), 1.0, 1e-80);
            s.store_sub(1468, 1466, 1467);
            s.store_div(1469, 1468, 1461);
        }

        s.b[1677] = ((s.v[1459] - s.v[1469]) < 80.0);
        s.store_scalar(1677, if s.b[1677] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1677]) {
            s.store_exp_sub(1342, 1459, 1469);
        }

        if (s.b[1608] && (!s.b[1677])) {
            s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1342, A::sub(s.ad_value(1459), s.ad_value(1469)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);
        }

        if s.b[1608] {
            s.store_mul(1472, 1433, 1342);
            s.store_scalar(1475, 0.0);
            s.store_scalar(1476, 0.0);
            s.store_scalar(1473, 0.0);
            s.store_scalar(1474, 0.0);
            s.store_scalar(1477, 0.0);
            s.store_scalar(1478, 0.0);
        }

        s.b[1678] = (s.v[1466] > 1e-6);
        s.store_scalar(1678, if s.b[1678] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1678]) {
            s.store_mul(1473, 1471, 1434);
            s.store_mul(1474, 1472, 1435);
            s.store_add_scaled_inputs(1475, 1473, 1.0, 1467, 2.0);
            s.store_add_scaled_inputs(1476, 1474, 1.0, 1468, 2.0);
            s.store_add_scaled_inputs3_indices(1477, 1466, 2.0, 1473, 1.0, 1474, 1.0);
        }

        s.b[1679] = (((s.v[1470]) as f64).abs() > 0.005);
        s.store_scalar(1679, if s.b[1679] { 1.0 } else { 0.0 });

        if ((s.b[1608] && s.b[1678]) && s.b[1679]) {
            s.store_add_scaled_products3(2, s.ad_value(1475), s.ad_value(1476), 1.0, A::offset(s.ad_value(1465), 2.0), s.ad_value(1476), 2.0, A::offset(s.ad_value(1469), 2.0), s.ad_value(1475), 2.0);
            s.store_div_scaled_product_by_product(1478, s.ad_value(1470), s.ad_value(1477), (-4.0), s.ad_value(1466), s.ad_value(2), 1.0);
        }

        if ((s.b[1608] && s.b[1678]) && (!s.b[1679])) {
            s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2, 1470, 1.0, 1470, 1.0, 1470, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);
            s.store_add_scaled_products3(3, s.ad_value(1475), s.ad_value(1471), 1.0, s.ad_value(1476), s.ad_value(1472), 1.0, A::mul3(s.ad_value(1475), s.ad_value(1476), s.ad_value(1466)), A::offset(A::mul(s.ad_value(1466), s.ad_value(2)), 1.0), 1.0);
            s.store_div_scaled_product3_by_product(1478, s.ad_value(1471), s.ad_value(1472), s.ad_value(1477), 1.0, s.ad_value(1466), s.ad_value(3), 1.0);
        }

        if s.b[1608] {
            s.store_ln(1479, 1466);
        }

        s.b[1680] = ((s.v[1467] / 2.0) < 80.0);
        s.store_scalar(1680, if s.b[1680] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1680]) {
            s.store_ln_one_plus_exp_scaled_input(2, 1467, 0.5);
        }

        if (s.b[1608] && (!s.b[1680])) {
            s.store_scale(2, 1467, 0.5);
        }

        if s.b[1608] {
            s.store_scale(1480, 2, 2.0);
        }

        s.b[1681] = ((s.v[1468] / 2.0) < 80.0);
        s.store_scalar(1681, if s.b[1681] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1681]) {
            s.store_ln_one_plus_exp_scaled_input(3, 1468, 0.5);
        }

        if (s.b[1608] && (!s.b[1681])) {
            s.store_scale(3, 1468, 0.5);
        }

        if s.b[1608] {
            s.store_scale(1481, 3, 2.0);
            s.store_sub(1482, 1481, 1468);
            s.store_sub(1483, 1480, 1467);
            s.store_add_scaled_products_indices(1484, 270, 1480, 1.0, 271, 1482, 1.0);
            s.store_add_scaled_products_indices(1485, 270, 1481, 1.0, 271, 1483, 1.0);
            s.store_div_add_scaled_inputs_rhs_indices(0, 1466, 1480, 1.0, 1481, 1.0);
            s.store_mul(1486, 1480, 0);
            s.store_mul(1487, 1481, 0);
            s.store_mul_ad_product_rhs_mixed_ia(1488, 1480, 191, A::exp(A::mul(s.ad_value(40), s.ad_value(295))));
            s.store_mul_ad_product_rhs_mixed_ia(1489, 1481, 192, A::exp(A::mul(s.ad_value(40), s.ad_value(295))));
            s.store_mul_add_scaled_product_rhs(2, 50, s.ad_value(1482), 1.0, s.ad_value(51), s.ad_value(1483), 1.0);
            s.store_scaled_add_sqrt_square_offset_ad(3, A::offset(s.ad_value(2), 1.0), 0.01, 0.5);
            s.store_scaled_add_sqrt_square_offset_ad(4, A::scale_offset(s.ad_value(2), 0.2, 1.0), 0.01, 0.5);
            s.store_div(1490, 3, 4);
            s.store_mul_ad_product_rhs(1491, 33, A::add_scaled_product(A::offset(A::mul(s.ad_value(41), s.ad_value(1482)), 1.0), 1.0, s.ad_value(42), s.ad_value(1483), 1.0), A::exp(A::mul_scaled_lhs(s.ad_value(44), -1.0, A::ln(A::add_scaled_product(A::offset(A::mul(s.ad_value(1486), s.ad_value(268)), 1.0), 1.0, s.ad_value(1487), s.ad_value(269), 1.0)))));
        }

        s.b[1682] = (s.v[56] == 0.0);
        s.store_scalar(1682, if s.b[1682] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1682]) {
            s.store_scalar(4, 1.0);
        }

        s.b[1683] = (s.v[56] < 0.0);
        s.store_scalar(1683, if s.b[1683] { 1.0 } else { 0.0 });

        if ((s.b[1608] && (!s.b[1682])) && s.b[1683]) {
            s.store_mul_exp_ad_rhs(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(1466), 1e-12))));
            s.store_sub_from_scalar(4, 1.0, 2);
        }

        if ((s.b[1608] && (!s.b[1682])) && (!s.b[1683])) {
            s.store_mul_exp_ad_rhs(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(1466), 1e-12))));
            s.store_div_from_scalar_offset_input(4, 1.0, 2, 1.0);
        }

        if s.b[1608] {
            s.store_mul_ad_affine_product_rhs(1492, 272, s.ad_value(1447), A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(1428))), A::sqrt_square_offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(1428))), 0.01)), 0.5, 0.0);
            s.store_mul_add_scaled_product_rhs(1493, 1492, s.ad_value(54), 1.0, s.ad_value(1466), s.ad_value(4), 1.0);
            s.store_add_scaled_inputs_product_first_ad(1494, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(1484)), 1e-6)))), 1.0), 1.0, 1491, 1.0, 38, 1493, 1.0);
            s.store_add_scaled_inputs_product_first_ad(1495, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(1485)), 1e-6)))), 1.0), 1.0, 1491, 1.0, 39, 1493, 1.0);
            s.store_div_scaled_product_mixed_iaa(1496, 1490, A::add(s.ad_value(1488), s.ad_value(1489)), 1.0, A::add(A::div(s.ad_value(1488), s.ad_value(1494)), A::div(s.ad_value(1489), s.ad_value(1495))), 1.0);
        }

        s.b[1684] = (((s.v[1463]) as f64).abs() > 0.007);
        s.store_scalar(1684, if s.b[1684] { 1.0 } else { 0.0 });

        s.b[1685] = (s.v[1463] > 0.0);
        s.store_scalar(1685, if s.b[1685] { 1.0 } else { 0.0 });

        if ((s.b[1608] && s.b[1684]) && s.b[1685]) {
            s.store_exp_neg_input(0, 1463);
            s.store_div_ad_rhs(1497, 1463, A::sub_from_scalar(1.0, s.ad_value(0)));
            s.store_mul(1498, 0, 1497);
            s.store_add_offset_ad_lhs(1499, A::ln(A::div(s.ad_value(1433), A::mul(s.ad_value(1466), s.ad_value(1497)))), (-0.6931471805599), 1436);
        }

        if ((s.b[1608] && s.b[1684]) && (!s.b[1685])) {
            s.store_exp(0, 1463);
            s.store_div_scaled_value_offset_denominator(1498, s.ad_value(1463), 1.0, s.ad_value(0), (-1.0), 1.0);
            s.store_mul(1497, 0, 1498);
            s.store_add_offset_ad_lhs(1499, A::ln(A::div(s.ad_value(1433), A::mul(s.ad_value(1466), s.ad_value(1498)))), (-0.6931471805599), 1437);
        }

        if (s.b[1608] && s.b[1684]) {
            s.store_div_scaled_inputs_mixed_ia(1500, 1463, -1.0, A::mul(s.ad_value(1462), A::add_scaled_sub_value_product(1.0, s.ad_value(1497), 1.0, s.ad_value(1463), s.ad_value(1435), (-1.0))), 1.0);
            s.store_div_ad_rhs(1501, 1463, A::mul(s.ad_value(1462), A::add_scaled_sub_value_product(1.0, s.ad_value(1498), 1.0, s.ad_value(1463), s.ad_value(1434), 1.0)));
            s.store_div_add_scaled_inputs_rhs_ad(1502, 1463, A::div_scaled_offset_numerator(A::mul(s.ad_value(1498), s.ad_value(1435)), 1.0, 0.5, s.ad_value(1501), 1.0), 1.0, A::div_scaled_offset_numerator(A::mul(s.ad_value(1497), s.ad_value(1434)), 1.0, 0.5, s.ad_value(1500), 1.0), -1.0);
        }

        if (s.b[1608] && (!s.b[1684])) {
            s.store_scale(0, 1464, (0.5 * 0.1666666666667));
            s.store_scale(2, 1463, 0.5);
            s.store_add_offset_lhs(1497, 2, 1.0, 0);
            s.store_add_ad_lhs(1498, A::sub_from_scalar(1.0, s.ad_value(2)), 0);
            s.store_scale(3, 2, 0.1666666666667);
            s.store_div_from_scalar_mul_ad(1500, 1.0, s.ad_value(1462), A::add(A::offset(s.ad_value(1435), 0.5), s.ad_value(3)));
            s.store_div_from_scalar_mul_ad(1501, 1.0, s.ad_value(1462), A::sub(A::offset(s.ad_value(1434), 0.5), s.ad_value(3)));
            s.store_add_scaled_inputs3_offset_mixed_aii(1499, A::ln(A::div(s.ad_value(1433), A::mul_sub_from_scalar_rhs(s.ad_value(1466), 1.0, A::scale(s.ad_value(0), 0.5)))), 1.0, 1436, 0.5, 1437, 0.5, (-0.6931471805599));
            s.store_div_from_scalar_ad(1502, (-12.0), A::add_scaled_inputs4_offset(s.ad_value(1462), ((-1.0) * 3.0), A::div_scaled_inputs(s.ad_value(1462), 12.0, A::mul(s.ad_value(1460), s.ad_value(1461)), 1.0), 1.0, A::mul3(s.ad_value(1462), A::sub(s.ad_value(1434), s.ad_value(1435)), s.ad_value(1463)), 1.0, A::mul_sub_from_scalar_lhs_scaled_output(0.2, A::scale(s.ad_value(1462), 0.25), s.ad_value(1464), 0.3333333333333), 1.0, 4.0));
        }

        if s.b[1608] {
            s.store_div_from_scalar(1503, 1.0, 1502);
        }

        s.b[1686] = (s.v[1466] > 1e-6);
        s.store_scalar(1686, if s.b[1686] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1686]) {
            s.store_div_scaled_value_offset_denominator(1504, s.ad_value(1480), 100.0, s.ad_value(1480), 100.0, 1.0);
        }

        s.b[1687] = (s.v[61] < 0.0);
        s.store_scalar(1687, if s.b[1687] { 1.0 } else { 0.0 });

        if ((s.b[1608] && s.b[1686]) && s.b[1687]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1505, 1.0, 1.0, A::mul(s.ad_value(61), s.ad_value(1504)));
        }

        if ((s.b[1608] && s.b[1686]) && (!s.b[1687])) {
            s.store_offset_mul(1505, 61, 1504, 1.0);
        }

        if (s.b[1608] && s.b[1686]) {
            s.store_div_scaled_value_offset_denominator(1506, s.ad_value(1481), 100.0, s.ad_value(1481), 100.0, 1.0);
        }

        s.b[1688] = (s.v[62] < 0.0);
        s.store_scalar(1688, if s.b[1688] { 1.0 } else { 0.0 });

        if ((s.b[1608] && s.b[1686]) && s.b[1688]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1507, 1.0, 1.0, A::mul(s.ad_value(62), s.ad_value(1506)));
        }

        if ((s.b[1608] && s.b[1686]) && (!s.b[1688])) {
            s.store_offset_mul(1507, 62, 1506, 1.0);
        }

        if (s.b[1608] && s.b[1686]) {
            s.store_sub_ad(1508, A::div_scaled_product_by_product(s.ad_value(1478), s.ad_value(1477), 1.0, s.ad_value(1475), s.ad_value(1476), 1.0), A::div_scaled_inputs2(A::div(s.ad_value(1471), s.ad_value(1475)), 1.0, A::div(s.ad_value(1472), s.ad_value(1476)), 1.0, s.ad_value(1466), 1.0));
            s.store_div_scaled_product_offset_denominator(1509, s.ad_value(1508), s.ad_value(1466), 1.0, s.ad_value(1508), 1.0, 1.0);
            s.store_sub(2, 1502, 1509);
            s.store_div_scaled_add_product(1510, s.ad_value(1466), 1.0, s.ad_value(1502), s.ad_value(1499), 1.0, s.ad_value(2), 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(1510, 1510, 1510, 1e-6, 0.5);
            s.store_scaled_mul_ad(1511, A::div(s.ad_value(1424), s.ad_value(1496)), A::add(s.ad_value(1505), s.ad_value(1507)), 0.5);
            s.store_sub_from_scalar_div_indices(1512, 1.0, 1466, 1509);
            s.store_offset(1513, 1499, 1.0);
            s.store_mul_sub_ad_lhs(1514, A::offset(A::mul(A::sub_scaled_inputs(s.ad_value(1509), 2.0, s.ad_value(1466), 1.0), s.ad_value(1503)), (-2.0)), s.ad_value(1499), 1510);
        }

        s.b[1689] = (s.v[1511] > 1e-14);
        s.store_scalar(1689, if s.b[1689] { 1.0 } else { 0.0 });

        if ((s.b[1608] && s.b[1686]) && s.b[1689]) {
            s.store_div_from_scalar_square_ad(1515, 2.0, s.ad_value(1511));
            s.store_mul(1516, 1515, 1512);
            s.store_add(1517, 1515, 1514);
            s.store_mul(1518, 1515, 1513);
            s.store_sqrt_offset_ad(1519, A::add(A::square(s.ad_value(1516)), A::mul3_scaled_output(s.ad_value(1515), s.ad_value(1515), s.ad_value(1515), 0.148148148148)), 1e-20);
            s.store_sqrt_offset_ad(1520, A::add(A::square(s.ad_value(1518)), A::mul3_scaled_output(s.ad_value(1517), s.ad_value(1517), s.ad_value(1517), 0.148148148148)), 1e-20);
            s.store_sub_ad(1521, A::exp_scaled_input(A::ln_scaled_input(A::add(s.ad_value(1519), s.ad_value(1516)), 0.5), 0.3333333333333), A::exp_scaled_input(A::ln_scaled_input(A::sub(s.ad_value(1519), s.ad_value(1516)), 0.5), 0.3333333333333));
            s.store_sub_ad(1522, A::exp_scaled_input(A::ln_scaled_input(A::add(s.ad_value(1520), s.ad_value(1518)), 0.5), 0.3333333333333), A::exp_scaled_input(A::ln_scaled_input(A::sub(s.ad_value(1520), s.ad_value(1518)), 0.5), 0.3333333333333));
        }

        if ((s.b[1608] && s.b[1686]) && (!s.b[1689])) {
            s.copy_ad(1521, 1512);
            s.copy_ad(1522, 1513);
        }

    }

    pub(super) fn stamp_reactive_block_24(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1608] && s.b[1686]) {
            s.store_square(4, 2);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1523, 1521, (0.94 * 0.5), 1522, (0.94 * 0.5), A::add_scaled_inputs(A::square(A::sub(s.ad_value(1521), s.ad_value(1522))), 1.0, s.ad_value(4), 10.0), (0.94 * 0.5));
            s.store_add_scaled_product_indices(1524, 1466, 1.0, 1509, 1523, 1.0);
            s.store_mul_sub_rhs(1525, 1502, 1523, 1499);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1526, 1524, 0.5, 1525, 0.5, A::add_scaled_inputs(A::square(A::sub(s.ad_value(1524), s.ad_value(1525))), 1.0, s.ad_value(4), 36.0), 0.5);
        }

        if (s.b[1608] && (!s.b[1686])) {
            s.copy_ad(1509, 1502);
            s.store_scaled_offset(1523, 1499, 1.0, 0.94);
            s.store_add_scaled_product_right_ad(1526, 1466, 0.5, 1502, A::sub_scaled_inputs(s.ad_value(1523), 1.0, s.ad_value(1499), 0.5), 1.0);
        }

        s.b[1690] = ((s.v[1526] - 0.5) < 80.0);
        s.store_scalar(1690, if s.b[1690] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1690]) {
            s.store_ln_one_plus_exp_ad(2, A::offset(s.ad_value(1526), (-0.5)));
        }

        if (s.b[1608] && (!s.b[1690])) {
            s.store_offset(2, 1526, (-0.5));
        }

        if s.b[1608] {
            s.store_offset(3, 2, 0.5);
            s.store_add_ad_rhs(4, 1523, A::ln(A::div(s.ad_value(1466), s.ad_value(3))));
        }

        s.b[1691] = ((s.v[4] - 6.0) < 80.0);
        s.store_scalar(1691, if s.b[1691] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1691]) {
            s.store_ln_one_plus_exp_ad(2, A::offset(s.ad_value(4), (-6.0)));
        }

        if (s.b[1608] && (!s.b[1691])) {
            s.store_offset(2, 4, (-6.0));
        }

        if s.b[1608] {
            s.store_offset(4, 2, 6.0);
        }

        s.b[1692] = ((s.v[225] - s.v[4]) < 80.0);
        s.store_scalar(1692, if s.b[1692] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1692]) {
            s.store_ln_one_plus_exp_ad(2, A::sub(s.ad_value(225), s.ad_value(4)));
        }

        if (s.b[1608] && (!s.b[1692])) {
            s.store_sub(2, 225, 4);
        }

        if s.b[1608] {
            s.store_sub(1527, 225, 2);
            s.store_div(2, 339, 1527);
            s.store_square(3, 2);
            s.store_square(4, 3);
            s.store_square(5, 4);
            s.store_exp_scaled_input_ad(0, A::ln(A::offset(A::mul(s.ad_value(1425), s.ad_value(4)), 1.0)), 2.666666666667);
            s.store_mul_ad_rhs(1528, 339, A::exp_scaled_input(A::ln(A::add(s.ad_value(0), A::square(s.ad_value(5)))), (-0.0625)));
            s.store_div_from_scalar_offset_input(1342, 1.0, 1460, 1.0);
            s.store_div_from_scalar_offset_input(1343, 1.0, 1461, 1.0);
            s.store_offset_add_ad(1345, A::ln(A::div_scaled_product(A::add_scaled_product(s.ad_value(1460), 1.0, s.ad_value(1461), s.ad_value(1343), 1.0), s.ad_value(1432), 1.0, s.ad_value(1433), 1.0)), s.ad_value(1528), 3.0);
            s.store_offset_add_ad(1346, A::ln(A::div_scaled_product(A::add_scaled_product(s.ad_value(1461), 1.0, s.ad_value(1460), s.ad_value(1342), 1.0), s.ad_value(1432), 1.0, s.ad_value(1433), 1.0)), s.ad_value(1528), 3.0);
        }

        s.b[1693] = (((s.v[1345] - s.v[1436]) * 0.3333333333333) < 80.0);
        s.store_scalar(1693, if s.b[1693] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1693]) {
            s.store_ln_one_plus_exp_ad(1344, A::sub_scaled_inputs(s.ad_value(1345), 0.3333333333333, s.ad_value(1436), 0.3333333333333));
        }

        if (s.b[1608] && (!s.b[1693])) {
            s.store_scaled_sub(1344, 1345, 1436, 0.3333333333333);
        }

        if s.b[1608] {
            s.store_sub_scaled_inputs(1349, 1345, 1.0, 1344, 3.0);
        }

        s.b[1694] = (((s.v[1346] - s.v[1437]) * 0.3333333333333) < 80.0);
        s.store_scalar(1694, if s.b[1694] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1694]) {
            s.store_ln_one_plus_exp_ad(1344, A::sub_scaled_inputs(s.ad_value(1346), 0.3333333333333, s.ad_value(1437), 0.3333333333333));
        }

        if (s.b[1608] && (!s.b[1694])) {
            s.store_scaled_sub(1344, 1346, 1437, 0.3333333333333);
        }

        if s.b[1608] {
            s.store_sub_scaled_inputs(1350, 1346, 1.0, 1344, 3.0);
            s.store_mul_add_scaled_product_rhs(1347, 1342, s.ad_value(1350), 1.0, s.ad_value(1460), s.ad_value(1458), 1.0);
            s.store_mul_add_scaled_product_rhs(1348, 1343, s.ad_value(1349), 1.0, s.ad_value(1461), s.ad_value(1459), 1.0);
        }

        s.b[1695] = (((s.v[1345] - s.v[1347]) * 0.3333333333333) < 80.0);
        s.store_scalar(1695, if s.b[1695] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1695]) {
            s.store_ln_one_plus_exp_ad(1344, A::sub_scaled_inputs(s.ad_value(1345), 0.3333333333333, s.ad_value(1347), 0.3333333333333));
        }

        if (s.b[1608] && (!s.b[1695])) {
            s.store_scaled_sub(1344, 1345, 1347, 0.3333333333333);
        }

        if s.b[1608] {
            s.store_sub_scaled_inputs(1349, 1345, 1.0, 1344, 3.0);
        }

        s.b[1696] = (((s.v[1346] - s.v[1348]) * 0.3333333333333) < 80.0);
        s.store_scalar(1696, if s.b[1696] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1696]) {
            s.store_ln_one_plus_exp_ad(1344, A::sub_scaled_inputs(s.ad_value(1346), 0.3333333333333, s.ad_value(1348), 0.3333333333333));
        }

        if (s.b[1608] && (!s.b[1696])) {
            s.store_scaled_sub(1344, 1346, 1348, 0.3333333333333);
        }

        if s.b[1608] {
            s.store_sub_scaled_inputs(1350, 1346, 1.0, 1344, 3.0);
            s.store_sub(1529, 1458, 1349);
            s.store_sub(1530, 1459, 1350);
            s.store_scalar(1356, 0.0);
            s.store_scalar(1359, 0.0);
            s.store_mul(1351, 1460, 1529);
        }

        s.b[1697] = (((s.v[1458] - s.v[1529]) - s.v[1528]) < 80.0);
        s.store_scalar(1697, if s.b[1697] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1697]) {
            s.store_exp_ad(1342, A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0));
        }

        if (s.b[1608] && (!s.b[1697])) {
            s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1342, A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);
        }

        if s.b[1608] {
            s.store_mul(1352, 1433, 1342);
            s.store_sub_square_lhs(1353, 1351, 1352);
            s.store_add_scaled_product_indices(1354, 1352, 1.0, 1460, 1351, 2.0);
            s.store_add_scaled_product_indices(1355, 1352, (-1.0), 1460, 1460, 2.0);
        }

        s.b[1698] = (s.v[1353] < (-0.005));
        s.store_scalar(1698, if s.b[1698] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1698]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
            s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);
            s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        s.b[1699] = (s.v[1353] > 0.005);
        s.store_scalar(1699, if s.b[1699] { 1.0 } else { 0.0 });

        if ((s.b[1608] && (!s.b[1698])) && s.b[1699]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_exp_neg_input(1359, 1356);
            s.store_div_scaled_product_offset_rhs(1357, s.ad_value(1356), s.ad_value(1359), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0);
            s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);
            s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        if ((s.b[1608] && (!s.b[1698])) && (!s.b[1699])) {
            s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1344, 1353, 1.0, 1353, 1.0, 1353, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);
            s.store_offset_mul(1357, 1353, 1344, 2.0);
            s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1342, 1353, 1.0, 1353, 1.0, 1353, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);
            s.store_mul(1358, 1354, 1342);
            s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1343, 1353, 1.0, 1353, 1.0, 1353, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(1360, 1355, 1342, 1.0, A::square(s.ad_value(1354)), 1343, (-1.0));
            s.store_scaled_mul(1363, 1354, 1344, (-0.5));
            s.store_add_scaled_product_value_ad(1364, A::mul3_scaled_output(s.ad_value(1354), s.ad_value(1354), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 2.0, A::scale(s.ad_value(1353), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1355, 1344, (-0.5));
        }

        s.b[1700] = (s.v[1353] > 0.005);
        s.store_scalar(1700, if s.b[1700] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1700]) {
            s.store_div_scaled_inputs_mixed_ia(1343, 1353, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0);
            s.store_mul(1361, 1343, 1359);
            s.store_sub_ln_lhs(1362, 1343, 1356);
        }

        s.b[1701] = (s.v[1353] < (-0.005));
        s.store_scalar(1701, if s.b[1701] { 1.0 } else { 0.0 });

        if ((s.b[1608] && (!s.b[1700])) && s.b[1701]) {
            s.store_sin_scaled_input(1343, 1356, 0.5);
            s.store_div_scaled_inputs_square_rhs(1361, 1353, -1.0, 1343, 1.0);
            s.store_ln(1362, 1361);
        }

        if ((s.b[1608] && (!s.b[1700])) && (!s.b[1701])) {
            s.store_sub_from_scalar_ad(1361, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(1362, 1361);
        }

        s.b[1702] = (((1.01 * s.v[1351]) + s.v[1357]) > 0.0);
        s.store_scalar(1702, if s.b[1702] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1702]) {
            s.store_add(1365, 1351, 1357);
            s.store_add(1366, 1460, 1358);
            s.copy_ad(1367, 1360);
        }

        if (s.b[1608] && (!s.b[1702])) {
            s.store_div_from_scalar_sub_ad(1343, 1.0, s.ad_value(1351), s.ad_value(1357));
            s.store_sub(1344, 1358, 1460);
            s.store_mul_sub_lhs(1365, 1352, 1361, 1343);
            s.store_mul_ad_lhs(1366, A::add_scaled_value_products(s.ad_value(1352), (-1.0), s.ad_value(1344), s.ad_value(1365), 1.0, s.ad_value(1363), s.ad_value(1361), (-1.0)), 1343);
            s.store_mul_ad_lhs(1367, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1360), s.ad_value(1365), 1.0, s.ad_value(1344), s.ad_value(1366), 2.0), 1.0, s.ad_value(1352), 1.0, A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361), (-1.0)), 1343);
        }

        s.b[1703] = (s.v[1365] > 0.0);
        s.store_scalar(1703, if s.b[1703] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1703]) {
            s.store_ln(1368, 1365);
            s.store_div_from_scalar(1342, 1.0, 1365);
            s.store_mul(1369, 1366, 1342);
            s.store_add_scaled_square_product_indices(1370, 1369, (-1.0), 1367, 1342, 1.0);
        }

        if (s.b[1608] && (!s.b[1703])) {
            s.store_add_offset_lhs_ad_rhs(1368, 1351, 0.6931471805599, A::ln_scaled_input(s.ad_value(1351), -1.0));
            s.store_div_from_scalar(1342, 1.0, 1529);
            s.store_add(1369, 1460, 1342);
            s.store_mul_neg_lhs(1370, 1342, 1342);
        }

        if s.b[1608] {
            s.store_sub_add_scaled_inputs4_lhs_indices(1371, 1459, 1.0, 1458, (-1.0), 1529, 1.0, 1368, 2.0, 1362);
            s.store_sub_ad_lhs(1372, A::scale_offset(s.ad_value(1369), 2.0, 1.0), 1363);
            s.store_sub_scaled_inputs(1373, 1370, 2.0, 1364, 1.0);
            s.store_add_scaled_product_indices(1374, 1351, 1.0, 1461, 1371, 1.0);
            s.store_add_scaled_product_indices(1375, 1460, 1.0, 1461, 1372, 1.0);
            s.store_mul(1376, 1461, 1373);
            s.store_add_scaled_product_indices(1377, 1352, (-1.0), 1374, 1365, 1.0);
            s.store_add_ad_lhs(1378, A::add_scaled_products(s.ad_value(1375), s.ad_value(1365), 1.0, s.ad_value(1374), s.ad_value(1366), 1.0), 1352);
            s.store_sub_ad_lhs(1379, A::add_scaled_products3(s.ad_value(1376), s.ad_value(1365), 1.0, s.ad_value(1375), s.ad_value(1366), 2.0, s.ad_value(1374), s.ad_value(1367), 1.0), 1352);
            s.store_add_scaled_square_product_indices(1388, 1378, 1.0, 1377, 1379, (-0.5));
            s.store_div_scaled_product3_mixed_iiia(1380, 1377, 1378, 1388, -1.0, A::offset(A::square(s.ad_value(1388)), 1e-200), 1.0);
            s.store_add(1529, 1529, 1380);
            s.store_mul(1351, 1460, 1529);
            s.store_mul(1381, 1461, 1530);
            s.store_add(1374, 1351, 1381);
        }

    }

    pub(super) fn stamp_reactive_block_25(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1608] {
            s.store_offset_scaled(1382, 1374, 0.065345483024, 1.0);
            s.store_add_scaled_product_value_ad(1383, A::scale_offset(s.ad_value(1374), 8.5797362674, 39.478417604), 1.0, 1351, 1381, 1.0);
            s.store_add_scaled_product_indices(1384, 1374, (2.0 * 39.478417604), 1351, 1381, 39.478417604);
            s.store_sqrt_add_scaled_square_product(1385, 1383, 1.0, 1382, 1384, (-4.0));
            s.store_div_scaled_inputs2_indices(1353, 1385, 1.0, 1383, (-1.0), 1382, 2.0);
            s.store_sub_square_lhs(1386, 1351, 1353);
        }

        s.b[1704] = (s.v[1386] > 0.0);
        s.store_scalar(1704, if s.b[1704] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1704]) {
            s.store_mul_ad_rhs(1377, 1386, A::add_scaled_inputs4(A::ln(A::div(s.ad_value(1386), s.ad_value(1433))), 1.0, s.ad_value(1528), 1.0, s.ad_value(1458), -1.0, s.ad_value(1529), 1.0));
            s.store_add_scaled_product_indices(1378, 1386, 1.0, 1460, 1351, 2.0);
            s.store_add_scaled_inputs3_indices(1387, 1458, 1.0, 1529, (-1.0), 1345, -1.0);
        }

        s.b[1705] = ((((s.v[1377] < 0.0) && (s.v[1378] > 0.0)) && (((s.v[1387] + 2.3025850929941) + ((s.v[1460]) as f64).ln()) > 0.0)) || (s.v[1387] > 1.0));
        s.store_scalar(1705, if s.b[1705] { 1.0 } else { 0.0 });

        if ((s.b[1608] && s.b[1704]) && s.b[1705]) {
            s.store_sub_div_rhs_indices(1529, 1529, 1377, 1378);
        }

        if s.b[1608] {
            s.store_mul(1351, 1460, 1529);
            s.store_mul(1381, 1461, 1530);
            s.store_add(1374, 1351, 1381);
            s.store_offset_scaled(1382, 1374, 0.065345483024, 1.0);
            s.store_add_scaled_product_value_ad(1383, A::scale_offset(s.ad_value(1374), 8.5797362674, 39.478417604), 1.0, 1351, 1381, 1.0);
            s.store_add_scaled_product_indices(1384, 1374, (2.0 * 39.478417604), 1351, 1381, 39.478417604);
            s.store_sqrt_add_scaled_square_product(1385, 1383, 1.0, 1382, 1384, (-4.0));
            s.store_div_scaled_inputs2_indices(1353, 1385, 1.0, 1383, (-1.0), 1382, 2.0);
        }

        s.b[1706] = (s.v[1353] < (-0.005));
        s.store_scalar(1706, if s.b[1706] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1706]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
            s.store_div_scaled_inputs2_mixed_iai(1358, 1353, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 0.25, 1353, 1.0);
        }

        s.b[1707] = (s.v[1353] > 0.005);
        s.store_scalar(1707, if s.b[1707] { 1.0 } else { 0.0 });

        if ((s.b[1608] && (!s.b[1706])) && s.b[1707]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_exp_neg_input(1359, 1356);
            s.store_div_scaled_product_offset_rhs(1357, s.ad_value(1356), s.ad_value(1359), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0);
            s.store_div_scaled_inputs2_mixed_iai(1358, 1353, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 0.25, 1353, 1.0);
        }

        if ((s.b[1608] && (!s.b[1706])) && (!s.b[1707])) {
            s.store_offset_ad(1357, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);
            s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1358, 1353, 1.0, 1353, 1.0, 1353, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);
        }

        if s.b[1608] {
            s.store_sub_ad_rhs(1353, 1353, A::div_scaled_inputs2(A::add_scaled_products(s.ad_value(1374), s.ad_value(1357), 1.0, s.ad_value(1351), s.ad_value(1381), 1.0), 1.0, s.ad_value(1353), 1.0, A::offset(A::mul(s.ad_value(1374), s.ad_value(1358)), 1.0), 1.0));
            s.store_sub_square_lhs(1386, 1351, 1353);
        }

        s.b[1708] = (s.v[1386] > 0.0);
        s.store_scalar(1708, if s.b[1708] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1708]) {
            s.store_mul_ad_rhs(1377, 1386, A::add_scaled_inputs4(A::ln(A::div(s.ad_value(1386), s.ad_value(1433))), 1.0, s.ad_value(1528), 1.0, s.ad_value(1458), -1.0, s.ad_value(1529), 1.0));
            s.store_add_scaled_product_indices(1378, 1386, 1.0, 1460, 1351, 2.0);
            s.store_add_scaled_inputs3_indices(1387, 1458, 1.0, 1529, (-1.0), 1345, -1.0);
        }

        s.b[1709] = ((((s.v[1377] < 0.0) && (s.v[1378] > 0.0)) && (((s.v[1387] + 2.3025850929941) + ((s.v[1460]) as f64).ln()) > 0.0)) || (s.v[1387] > 1.0));
        s.store_scalar(1709, if s.b[1709] { 1.0 } else { 0.0 });

        if ((s.b[1608] && s.b[1708]) && s.b[1709]) {
            s.store_sub_div_rhs_indices(1529, 1529, 1377, 1378);
        }

        if s.b[1608] {
            s.store_mul(1351, 1460, 1529);
        }

        s.b[1710] = (((s.v[1458] - s.v[1529]) - s.v[1528]) < 80.0);
        s.store_scalar(1710, if s.b[1710] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1710]) {
            s.store_exp_ad(1342, A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0));
        }

        if (s.b[1608] && (!s.b[1710])) {
            s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1342, A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);
        }

        if s.b[1608] {
            s.store_mul(1352, 1433, 1342);
            s.store_sub_square_lhs(1353, 1351, 1352);
            s.store_add_scaled_product_indices(1354, 1352, 1.0, 1460, 1351, 2.0);
            s.store_add_scaled_product_indices(1355, 1352, (-1.0), 1460, 1460, 2.0);
        }

        s.b[1711] = (s.v[1353] < (-0.005));
        s.store_scalar(1711, if s.b[1711] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1711]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
            s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);
            s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        s.b[1712] = (s.v[1353] > 0.005);
        s.store_scalar(1712, if s.b[1712] { 1.0 } else { 0.0 });

        if ((s.b[1608] && (!s.b[1711])) && s.b[1712]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_exp_neg_input(1359, 1356);
            s.store_div_scaled_product_offset_rhs(1357, s.ad_value(1356), s.ad_value(1359), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0);
            s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);
            s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        if ((s.b[1608] && (!s.b[1711])) && (!s.b[1712])) {
            s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1344, 1353, 1.0, 1353, 1.0, 1353, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);
            s.store_offset_mul(1357, 1353, 1344, 2.0);
            s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1342, 1353, 1.0, 1353, 1.0, 1353, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);
            s.store_mul(1358, 1354, 1342);
            s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1343, 1353, 1.0, 1353, 1.0, 1353, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(1360, 1355, 1342, 1.0, A::square(s.ad_value(1354)), 1343, (-1.0));
            s.store_scaled_mul(1363, 1354, 1344, (-0.5));
            s.store_add_scaled_product_value_ad(1364, A::mul3_scaled_output(s.ad_value(1354), s.ad_value(1354), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 2.0, A::scale(s.ad_value(1353), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1355, 1344, (-0.5));
        }

        s.b[1713] = (s.v[1353] > 0.005);
        s.store_scalar(1713, if s.b[1713] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1713]) {
            s.store_div_scaled_inputs_mixed_ia(1343, 1353, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0);
            s.store_mul(1361, 1343, 1359);
            s.store_sub_ln_lhs(1362, 1343, 1356);
        }

        s.b[1714] = (s.v[1353] < (-0.005));
        s.store_scalar(1714, if s.b[1714] { 1.0 } else { 0.0 });

        if ((s.b[1608] && (!s.b[1713])) && s.b[1714]) {
            s.store_sin_scaled_input(1343, 1356, 0.5);
            s.store_div_scaled_inputs_square_rhs(1361, 1353, -1.0, 1343, 1.0);
            s.store_ln(1362, 1361);
        }

        if ((s.b[1608] && (!s.b[1713])) && (!s.b[1714])) {
            s.store_sub_from_scalar_ad(1361, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(1362, 1361);
        }

        s.b[1715] = (((1.01 * s.v[1351]) + s.v[1357]) > 0.0);
        s.store_scalar(1715, if s.b[1715] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1715]) {
            s.store_add(1365, 1351, 1357);
            s.store_add(1366, 1460, 1358);
            s.copy_ad(1367, 1360);
        }

        if (s.b[1608] && (!s.b[1715])) {
            s.store_div_from_scalar_sub_ad(1343, 1.0, s.ad_value(1351), s.ad_value(1357));
            s.store_sub(1344, 1358, 1460);
            s.store_mul_sub_lhs(1365, 1352, 1361, 1343);
            s.store_mul_ad_lhs(1366, A::add_scaled_value_products(s.ad_value(1352), (-1.0), s.ad_value(1344), s.ad_value(1365), 1.0, s.ad_value(1363), s.ad_value(1361), (-1.0)), 1343);
            s.store_mul_ad_lhs(1367, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1360), s.ad_value(1365), 1.0, s.ad_value(1344), s.ad_value(1366), 2.0), 1.0, s.ad_value(1352), 1.0, A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361), (-1.0)), 1343);
        }

        s.b[1716] = (s.v[1365] > 0.0);
        s.store_scalar(1716, if s.b[1716] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1716]) {
            s.store_ln(1368, 1365);
            s.store_div_from_scalar(1342, 1.0, 1365);
            s.store_mul(1369, 1366, 1342);
            s.store_add_scaled_square_product_indices(1370, 1369, (-1.0), 1367, 1342, 1.0);
        }

        if (s.b[1608] && (!s.b[1716])) {
            s.store_add_offset_lhs_ad_rhs(1368, 1351, 0.6931471805599, A::ln_scaled_input(s.ad_value(1351), -1.0));
            s.store_div_from_scalar(1342, 1.0, 1529);
            s.store_add(1369, 1460, 1342);
            s.store_mul_neg_lhs(1370, 1342, 1342);
        }

        if s.b[1608] {
            s.store_sub_add_scaled_inputs4_lhs_indices(1371, 1459, 1.0, 1458, (-1.0), 1529, 1.0, 1368, 2.0, 1362);
            s.store_sub_ad_lhs(1372, A::scale_offset(s.ad_value(1369), 2.0, 1.0), 1363);
            s.store_sub_scaled_inputs(1373, 1370, 2.0, 1364, 1.0);
            s.store_add_scaled_product_indices(1374, 1351, 1.0, 1461, 1371, 1.0);
            s.store_add_scaled_product_indices(1375, 1460, 1.0, 1461, 1372, 1.0);
            s.store_mul(1376, 1461, 1373);
            s.store_add_scaled_product_indices(1377, 1352, (-1.0), 1374, 1365, 1.0);
            s.store_add_ad_lhs(1378, A::add_scaled_products(s.ad_value(1375), s.ad_value(1365), 1.0, s.ad_value(1374), s.ad_value(1366), 1.0), 1352);
            s.store_sub_ad_lhs(1379, A::add_scaled_products3(s.ad_value(1376), s.ad_value(1365), 1.0, s.ad_value(1375), s.ad_value(1366), 2.0, s.ad_value(1374), s.ad_value(1367), 1.0), 1352);
            s.store_add_scaled_square_product_indices(1388, 1378, 1.0, 1377, 1379, (-0.5));
            s.store_div_scaled_product3_mixed_iiia(1380, 1377, 1378, 1388, -1.0, A::offset(A::square(s.ad_value(1388)), 1e-200), 1.0);
            s.store_add(1529, 1529, 1380);
            s.store_mul(1351, 1460, 1529);
        }

        s.b[1717] = (((s.v[1458] - s.v[1529]) - s.v[1528]) < 80.0);
        s.store_scalar(1717, if s.b[1717] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1717]) {
            s.store_exp_ad(1342, A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0));
        }

        if (s.b[1608] && (!s.b[1717])) {
            s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1342, A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);
        }

        if s.b[1608] {
            s.store_mul(1352, 1433, 1342);
            s.store_sub_square_lhs(1353, 1351, 1352);
            s.store_add_scaled_product_indices(1354, 1352, 1.0, 1460, 1351, 2.0);
            s.store_add_scaled_product_indices(1355, 1352, (-1.0), 1460, 1460, 2.0);
        }

        s.b[1718] = (s.v[1353] < (-0.005));
        s.store_scalar(1718, if s.b[1718] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1718]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
            s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);
            s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        s.b[1719] = (s.v[1353] > 0.005);
        s.store_scalar(1719, if s.b[1719] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_26(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1608] && (!s.b[1718])) && s.b[1719]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_exp_neg_input(1359, 1356);
            s.store_div_scaled_product_offset_rhs(1357, s.ad_value(1356), s.ad_value(1359), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0);
            s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);
            s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        if ((s.b[1608] && (!s.b[1718])) && (!s.b[1719])) {
            s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1344, 1353, 1.0, 1353, 1.0, 1353, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);
            s.store_offset_mul(1357, 1353, 1344, 2.0);
            s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1342, 1353, 1.0, 1353, 1.0, 1353, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);
            s.store_mul(1358, 1354, 1342);
            s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1343, 1353, 1.0, 1353, 1.0, 1353, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(1360, 1355, 1342, 1.0, A::square(s.ad_value(1354)), 1343, (-1.0));
            s.store_scaled_mul(1363, 1354, 1344, (-0.5));
            s.store_add_scaled_product_value_ad(1364, A::mul3_scaled_output(s.ad_value(1354), s.ad_value(1354), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 2.0, A::scale(s.ad_value(1353), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1355, 1344, (-0.5));
        }

        s.b[1720] = (s.v[1353] > 0.005);
        s.store_scalar(1720, if s.b[1720] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1720]) {
            s.store_div_scaled_inputs_mixed_ia(1343, 1353, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0);
            s.store_mul(1361, 1343, 1359);
            s.store_sub_ln_lhs(1362, 1343, 1356);
        }

        s.b[1721] = (s.v[1353] < (-0.005));
        s.store_scalar(1721, if s.b[1721] { 1.0 } else { 0.0 });

        if ((s.b[1608] && (!s.b[1720])) && s.b[1721]) {
            s.store_sin_scaled_input(1343, 1356, 0.5);
            s.store_div_scaled_inputs_square_rhs(1361, 1353, -1.0, 1343, 1.0);
            s.store_ln(1362, 1361);
        }

        if ((s.b[1608] && (!s.b[1720])) && (!s.b[1721])) {
            s.store_sub_from_scalar_ad(1361, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(1362, 1361);
        }

        s.b[1722] = (((1.01 * s.v[1351]) + s.v[1357]) > 0.0);
        s.store_scalar(1722, if s.b[1722] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1722]) {
            s.store_add(1365, 1351, 1357);
            s.store_add(1366, 1460, 1358);
            s.copy_ad(1367, 1360);
        }

        if (s.b[1608] && (!s.b[1722])) {
            s.store_div_from_scalar_sub_ad(1343, 1.0, s.ad_value(1351), s.ad_value(1357));
            s.store_sub(1344, 1358, 1460);
            s.store_mul_sub_lhs(1365, 1352, 1361, 1343);
            s.store_mul_ad_lhs(1366, A::add_scaled_value_products(s.ad_value(1352), (-1.0), s.ad_value(1344), s.ad_value(1365), 1.0, s.ad_value(1363), s.ad_value(1361), (-1.0)), 1343);
            s.store_mul_ad_lhs(1367, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1360), s.ad_value(1365), 1.0, s.ad_value(1344), s.ad_value(1366), 2.0), 1.0, s.ad_value(1352), 1.0, A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361), (-1.0)), 1343);
        }

        s.b[1723] = (s.v[1365] > 0.0);
        s.store_scalar(1723, if s.b[1723] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1723]) {
            s.store_ln(1368, 1365);
            s.store_div_from_scalar(1342, 1.0, 1365);
            s.store_mul(1369, 1366, 1342);
            s.store_add_scaled_square_product_indices(1370, 1369, (-1.0), 1367, 1342, 1.0);
        }

        if (s.b[1608] && (!s.b[1723])) {
            s.store_add_offset_lhs_ad_rhs(1368, 1351, 0.6931471805599, A::ln_scaled_input(s.ad_value(1351), -1.0));
            s.store_div_from_scalar(1342, 1.0, 1529);
            s.store_add(1369, 1460, 1342);
            s.store_mul_neg_lhs(1370, 1342, 1342);
        }

        if s.b[1608] {
            s.store_sub_add_scaled_inputs4_lhs_indices(1371, 1459, 1.0, 1458, (-1.0), 1529, 1.0, 1368, 2.0, 1362);
            s.store_sub_ad_lhs(1372, A::scale_offset(s.ad_value(1369), 2.0, 1.0), 1363);
            s.store_sub_scaled_inputs(1373, 1370, 2.0, 1364, 1.0);
            s.store_add_scaled_product_indices(1374, 1351, 1.0, 1461, 1371, 1.0);
            s.store_add_scaled_product_indices(1375, 1460, 1.0, 1461, 1372, 1.0);
            s.store_mul(1376, 1461, 1373);
            s.store_add_scaled_product_indices(1377, 1352, (-1.0), 1374, 1365, 1.0);
            s.store_add_ad_lhs(1378, A::add_scaled_products(s.ad_value(1375), s.ad_value(1365), 1.0, s.ad_value(1374), s.ad_value(1366), 1.0), 1352);
            s.store_sub_ad_lhs(1379, A::add_scaled_products3(s.ad_value(1376), s.ad_value(1365), 1.0, s.ad_value(1375), s.ad_value(1366), 2.0, s.ad_value(1374), s.ad_value(1367), 1.0), 1352);
            s.store_add_scaled_square_product_indices(1388, 1378, 1.0, 1377, 1379, (-0.5));
            s.store_div_scaled_product3_mixed_iiia(1380, 1377, 1378, 1388, -1.0, A::offset(A::square(s.ad_value(1388)), 1e-200), 1.0);
            s.store_add(1529, 1529, 1380);
        }

        s.b[1724] = (p.p10 == 1.0);
        s.store_scalar(1724, if s.b[1724] { 1.0 } else { 0.0 });

        s.b[1725] = (((s.v[1380]) as f64).abs() > 0.01);
        s.store_scalar(1725, if s.b[1725] { 1.0 } else { 0.0 });

        if ((s.b[1608] && s.b[1724]) && s.b[1725]) {
            s.store_mul(1351, 1460, 1529);
        }

        s.b[1726] = (((s.v[1458] - s.v[1529]) - s.v[1528]) < 80.0);
        s.store_scalar(1726, if s.b[1726] { 1.0 } else { 0.0 });

        if (((s.b[1608] && s.b[1724]) && s.b[1725]) && s.b[1726]) {
            s.store_exp_ad(1342, A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0));
        }

        if (((s.b[1608] && s.b[1724]) && s.b[1725]) && (!s.b[1726])) {
            s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1342, A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);
        }

        if ((s.b[1608] && s.b[1724]) && s.b[1725]) {
            s.store_mul(1352, 1433, 1342);
            s.store_sub_square_lhs(1353, 1351, 1352);
            s.store_add_scaled_product_indices(1354, 1352, 1.0, 1460, 1351, 2.0);
            s.store_add_scaled_product_indices(1355, 1352, (-1.0), 1460, 1460, 2.0);
        }

        s.b[1727] = (s.v[1353] < (-0.005));
        s.store_scalar(1727, if s.b[1727] { 1.0 } else { 0.0 });

        if (((s.b[1608] && s.b[1724]) && s.b[1725]) && s.b[1727]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
            s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);
            s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        s.b[1728] = (s.v[1353] > 0.005);
        s.store_scalar(1728, if s.b[1728] { 1.0 } else { 0.0 });

        if ((((s.b[1608] && s.b[1724]) && s.b[1725]) && (!s.b[1727])) && s.b[1728]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_exp_neg_input(1359, 1356);
            s.store_div_scaled_product_offset_rhs(1357, s.ad_value(1356), s.ad_value(1359), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0);
            s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);
            s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        if ((((s.b[1608] && s.b[1724]) && s.b[1725]) && (!s.b[1727])) && (!s.b[1728])) {
            s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1344, 1353, 1.0, 1353, 1.0, 1353, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);
            s.store_offset_mul(1357, 1353, 1344, 2.0);
            s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1342, 1353, 1.0, 1353, 1.0, 1353, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);
            s.store_mul(1358, 1354, 1342);
            s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1343, 1353, 1.0, 1353, 1.0, 1353, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(1360, 1355, 1342, 1.0, A::square(s.ad_value(1354)), 1343, (-1.0));
            s.store_scaled_mul(1363, 1354, 1344, (-0.5));
            s.store_add_scaled_product_value_ad(1364, A::mul3_scaled_output(s.ad_value(1354), s.ad_value(1354), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 2.0, A::scale(s.ad_value(1353), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1355, 1344, (-0.5));
        }

        s.b[1729] = (s.v[1353] > 0.005);
        s.store_scalar(1729, if s.b[1729] { 1.0 } else { 0.0 });

        if (((s.b[1608] && s.b[1724]) && s.b[1725]) && s.b[1729]) {
            s.store_div_scaled_inputs_mixed_ia(1343, 1353, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0);
            s.store_mul(1361, 1343, 1359);
            s.store_sub_ln_lhs(1362, 1343, 1356);
        }

        s.b[1730] = (s.v[1353] < (-0.005));
        s.store_scalar(1730, if s.b[1730] { 1.0 } else { 0.0 });

        if ((((s.b[1608] && s.b[1724]) && s.b[1725]) && (!s.b[1729])) && s.b[1730]) {
            s.store_sin_scaled_input(1343, 1356, 0.5);
            s.store_div_scaled_inputs_square_rhs(1361, 1353, -1.0, 1343, 1.0);
            s.store_ln(1362, 1361);
        }

        if ((((s.b[1608] && s.b[1724]) && s.b[1725]) && (!s.b[1729])) && (!s.b[1730])) {
            s.store_sub_from_scalar_ad(1361, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(1362, 1361);
        }

        s.b[1731] = (((1.01 * s.v[1351]) + s.v[1357]) > 0.0);
        s.store_scalar(1731, if s.b[1731] { 1.0 } else { 0.0 });

        if (((s.b[1608] && s.b[1724]) && s.b[1725]) && s.b[1731]) {
            s.store_add(1365, 1351, 1357);
            s.store_add(1366, 1460, 1358);
            s.copy_ad(1367, 1360);
        }

        if (((s.b[1608] && s.b[1724]) && s.b[1725]) && (!s.b[1731])) {
            s.store_div_from_scalar_sub_ad(1343, 1.0, s.ad_value(1351), s.ad_value(1357));
            s.store_sub(1344, 1358, 1460);
            s.store_mul_sub_lhs(1365, 1352, 1361, 1343);
            s.store_mul_ad_lhs(1366, A::add_scaled_value_products(s.ad_value(1352), (-1.0), s.ad_value(1344), s.ad_value(1365), 1.0, s.ad_value(1363), s.ad_value(1361), (-1.0)), 1343);
            s.store_mul_ad_lhs(1367, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1360), s.ad_value(1365), 1.0, s.ad_value(1344), s.ad_value(1366), 2.0), 1.0, s.ad_value(1352), 1.0, A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361), (-1.0)), 1343);
        }

        s.b[1732] = (s.v[1365] > 0.0);
        s.store_scalar(1732, if s.b[1732] { 1.0 } else { 0.0 });

        if (((s.b[1608] && s.b[1724]) && s.b[1725]) && s.b[1732]) {
            s.store_ln(1368, 1365);
            s.store_div_from_scalar(1342, 1.0, 1365);
            s.store_mul(1369, 1366, 1342);
            s.store_add_scaled_square_product_indices(1370, 1369, (-1.0), 1367, 1342, 1.0);
        }

        if (((s.b[1608] && s.b[1724]) && s.b[1725]) && (!s.b[1732])) {
            s.store_add_offset_lhs_ad_rhs(1368, 1351, 0.6931471805599, A::ln_scaled_input(s.ad_value(1351), -1.0));
            s.store_div_from_scalar(1342, 1.0, 1529);
            s.store_add(1369, 1460, 1342);
            s.store_mul_neg_lhs(1370, 1342, 1342);
        }

        if ((s.b[1608] && s.b[1724]) && s.b[1725]) {
            s.store_sub_add_scaled_inputs4_lhs_indices(1371, 1459, 1.0, 1458, (-1.0), 1529, 1.0, 1368, 2.0, 1362);
            s.store_sub_ad_lhs(1372, A::scale_offset(s.ad_value(1369), 2.0, 1.0), 1363);
            s.store_sub_scaled_inputs(1373, 1370, 2.0, 1364, 1.0);
            s.store_add_scaled_product_indices(1374, 1351, 1.0, 1461, 1371, 1.0);
            s.store_add_scaled_product_indices(1375, 1460, 1.0, 1461, 1372, 1.0);
            s.store_mul(1376, 1461, 1373);
            s.store_add_scaled_product_indices(1377, 1352, (-1.0), 1374, 1365, 1.0);
            s.store_add_ad_lhs(1378, A::add_scaled_products(s.ad_value(1375), s.ad_value(1365), 1.0, s.ad_value(1374), s.ad_value(1366), 1.0), 1352);
            s.store_sub_ad_lhs(1379, A::add_scaled_products3(s.ad_value(1376), s.ad_value(1365), 1.0, s.ad_value(1375), s.ad_value(1366), 2.0, s.ad_value(1374), s.ad_value(1367), 1.0), 1352);
        }

    }

    pub(super) fn stamp_reactive_block_27(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1608] && s.b[1724]) && s.b[1725]) {
            s.store_add_scaled_square_product_indices(1388, 1378, 1.0, 1377, 1379, (-0.5));
            s.store_div_scaled_product3_mixed_iiia(1380, 1377, 1378, 1388, -1.0, A::offset(A::square(s.ad_value(1388)), 1e-200), 1.0);
            s.store_add(1529, 1529, 1380);
        }

        if s.b[1608] {
            s.store_mul(1532, 1460, 1529);
        }

        s.b[1733] = (((s.v[1458] - s.v[1529]) - s.v[1528]) < 80.0);
        s.store_scalar(1733, if s.b[1733] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1733]) {
            s.store_exp_ad(1342, A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0));
        }

        if (s.b[1608] && (!s.b[1733])) {
            s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1342, A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);
        }

        if s.b[1608] {
            s.store_mul(1535, 1433, 1342);
            s.store_sub_square_lhs(1534, 1532, 1535);
        }

        s.b[1734] = (s.v[1535] <= 0.0);
        s.store_scalar(1734, if s.b[1734] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1734]) {
            s.store_scalar(1531, 1e-80);
            s.store_sub(1533, 1531, 1532);
            s.store_div(1530, 1533, 1461);
        }

        s.b[1735] = (s.v[1534] < (-0.005));
        s.store_scalar(1735, if s.b[1735] { 1.0 } else { 0.0 });

        if ((s.b[1608] && (!s.b[1734])) && s.b[1735]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1534));
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
        }

        s.b[1736] = (s.v[1534] > 0.005);
        s.store_scalar(1736, if s.b[1736] { 1.0 } else { 0.0 });

        if (((s.b[1608] && (!s.b[1734])) && (!s.b[1735])) && s.b[1736]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1534));
            s.store_exp_neg_input(1359, 1356);
            s.store_div_scaled_product_offset_rhs(1357, s.ad_value(1356), s.ad_value(1359), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0);
        }

        if (((s.b[1608] && (!s.b[1734])) && (!s.b[1735])) && (!s.b[1736])) {
            s.store_offset_ad(1357, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1534), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1534), 1.0, A::scale(s.ad_value(1534), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);
        }

        s.b[1737] = (((1.01 * s.v[1532]) + s.v[1357]) > 0.0);
        s.store_scalar(1737, if s.b[1737] { 1.0 } else { 0.0 });

        if ((s.b[1608] && (!s.b[1734])) && s.b[1737]) {
            s.store_add(1342, 1532, 1357);
        }

        s.b[1738] = ((s.v[1535] * s.v[1532]) < (((0.9 * s.v[1532]) * s.v[1532]) * s.v[1342]));
        s.store_scalar(1738, if s.b[1738] { 1.0 } else { 0.0 });

        if (((s.b[1608] && (!s.b[1734])) && s.b[1737]) && s.b[1738]) {
            s.store_offset_div(1531, 1535, 1342, 1e-80);
            s.store_sub(1533, 1531, 1532);
            s.store_div(1530, 1533, 1461);
        }

        s.b[1739] = (s.v[1534] > 0.005);
        s.store_scalar(1739, if s.b[1739] { 1.0 } else { 0.0 });

        if ((((s.b[1608] && (!s.b[1734])) && s.b[1737]) && (!s.b[1738])) && s.b[1739]) {
            s.store_sub_ad_lhs(1343, A::ln(A::div_scaled_inputs(s.ad_value(1534), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0)), 1356);
        }

        s.b[1740] = (s.v[1534] < (-0.005));
        s.store_scalar(1740, if s.b[1740] { 1.0 } else { 0.0 });

        if (((((s.b[1608] && (!s.b[1734])) && s.b[1737]) && (!s.b[1738])) && (!s.b[1739])) && s.b[1740]) {
            s.store_sin_scaled_input(1344, 1356, 0.5);
            s.store_ln_div_scaled_input_square_denominator(1343, 1534, -1.0, 1344, 1.0);
        }

        if (((((s.b[1608] && (!s.b[1734])) && s.b[1737]) && (!s.b[1738])) && (!s.b[1739])) && (!s.b[1740])) {
            s.store_ln_ad(1343, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1534), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1534), 1.0, A::scale(s.ad_value(1534), 0.0396825396825397), 0.05), 0.3333333333333)));
        }

        if (((s.b[1608] && (!s.b[1734])) && s.b[1737]) && (!s.b[1738])) {
            s.store_sub_add_scaled_inputs4_lhs_mixed_iiia(1530, 1459, 1.0, 1458, (-1.0), 1529, 1.0, A::ln(s.ad_value(1342)), 2.0, 1343);
            s.store_mul(1533, 1461, 1530);
            s.store_add(1531, 1532, 1533);
        }

        s.b[1741] = (s.v[1534] > 0.005);
        s.store_scalar(1741, if s.b[1741] { 1.0 } else { 0.0 });

        s.b[1742] = ((((s.v[1529] + s.v[1528]) - s.v[1458]) - s.v[1356]) < 80.0);
        s.store_scalar(1742, if s.b[1742] { 1.0 } else { 0.0 });

        if ((((s.b[1608] && (!s.b[1734])) && (!s.b[1737])) && s.b[1741]) && s.b[1742]) {
            s.store_exp_ad(1344, A::add_scaled_inputs4(s.ad_value(1529), 1.0, s.ad_value(1528), 1.0, s.ad_value(1458), -1.0, s.ad_value(1356), -1.0));
        }

        if ((((s.b[1608] && (!s.b[1734])) && (!s.b[1737])) && s.b[1741]) && (!s.b[1742])) {
            s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1344, A::add_scaled_inputs4(s.ad_value(1529), 1.0, s.ad_value(1528), 1.0, s.ad_value(1458), -1.0, s.ad_value(1356), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);
        }

        if (((s.b[1608] && (!s.b[1734])) && (!s.b[1737])) && s.b[1741]) {
            s.store_div(1343, 1344, 1433);
            s.store_div_scaled_product_denominator_ad(1342, 1534, 1343, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0);
        }

        s.b[1743] = (s.v[1534] < (-0.005));
        s.store_scalar(1743, if s.b[1743] { 1.0 } else { 0.0 });

        if ((((s.b[1608] && (!s.b[1734])) && (!s.b[1737])) && (!s.b[1741])) && s.b[1743]) {
            s.store_sin_scaled_input(1343, 1356, 0.5);
            s.store_div_scaled_value_by_product(1342, s.ad_value(1534), -1.0, A::square(s.ad_value(1343)), s.ad_value(1535), 1.0);
        }

        if ((((s.b[1608] && (!s.b[1734])) && (!s.b[1737])) && (!s.b[1741])) && (!s.b[1743])) {
            s.store_div_ad_lhs(1342, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1534), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1534), 1.0, A::scale(s.ad_value(1534), 0.0396825396825397), 0.05), 0.3333333333333)), 1535);
        }

        if ((s.b[1608] && (!s.b[1734])) && (!s.b[1737])) {
            s.store_offset_div_scaled_inputs2_mixed_iia(1531, 1532, 1.0, 1357, (-1.0), A::sub_from_scalar(1.0, s.ad_value(1342)), 1.0, 1e-80);
            s.store_sub(1533, 1531, 1532);
            s.store_div(1530, 1533, 1461);
        }

        s.b[1744] = (((s.v[1459] - s.v[1530]) - s.v[1528]) < 80.0);
        s.store_scalar(1744, if s.b[1744] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1744]) {
            s.store_exp_ad(1342, A::add_scaled_inputs3(s.ad_value(1459), 1.0, s.ad_value(1530), (-1.0), s.ad_value(1528), -1.0));
        }

        if (s.b[1608] && (!s.b[1744])) {
            s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1342, A::add_scaled_inputs3(s.ad_value(1459), 1.0, s.ad_value(1530), (-1.0), s.ad_value(1528), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);
        }

        if s.b[1608] {
            s.store_mul(1536, 1433, 1342);
            s.store_scalar(1539, 0.0);
            s.store_scalar(1540, 0.0);
            s.store_scalar(1537, 0.0);
            s.store_scalar(1538, 0.0);
            s.store_scalar(1541, 0.0);
            s.store_scalar(1542, 0.0);
        }

        s.b[1745] = (s.v[1466] > 1e-6);
        s.store_scalar(1745, if s.b[1745] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1745]) {
            s.store_mul(1537, 1535, 1434);
            s.store_mul(1538, 1536, 1435);
            s.store_add_scaled_inputs(1539, 1537, 1.0, 1532, 2.0);
            s.store_add_scaled_inputs(1540, 1538, 1.0, 1533, 2.0);
            s.store_add_scaled_inputs3_indices(1541, 1531, 2.0, 1537, 1.0, 1538, 1.0);
        }

        s.b[1746] = (((s.v[1534]) as f64).abs() > 0.005);
        s.store_scalar(1746, if s.b[1746] { 1.0 } else { 0.0 });

        if ((s.b[1608] && s.b[1745]) && s.b[1746]) {
            s.store_add_scaled_products3(2, s.ad_value(1539), s.ad_value(1540), 1.0, A::offset(s.ad_value(1529), 2.0), s.ad_value(1540), 2.0, A::offset(s.ad_value(1530), 2.0), s.ad_value(1539), 2.0);
            s.store_div_scaled_product_by_product(1542, s.ad_value(1534), s.ad_value(1541), (-4.0), s.ad_value(1531), s.ad_value(2), 1.0);
        }

        if ((s.b[1608] && s.b[1745]) && (!s.b[1746])) {
            s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2, 1534, 1.0, 1534, 1.0, 1534, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);
            s.store_add_scaled_products3(3, s.ad_value(1539), s.ad_value(1535), 1.0, s.ad_value(1540), s.ad_value(1536), 1.0, A::mul3(s.ad_value(1539), s.ad_value(1540), s.ad_value(1531)), A::offset(A::mul(s.ad_value(1531), s.ad_value(2)), 1.0), 1.0);
            s.store_div_scaled_product3_by_product(1542, s.ad_value(1535), s.ad_value(1536), s.ad_value(1541), 1.0, s.ad_value(1531), s.ad_value(3), 1.0);
        }

        if s.b[1608] {
            s.store_add_ad_rhs(1543, 1528, A::ln(s.ad_value(1531)));
            s.store_scaled_add(1544, 1466, 1531, 0.5);
            s.store_sub(1545, 1543, 1479);
            s.store_scalar(1548, 1.0);
        }

        s.b[1747] = (p.p9 > 0.0);
        s.store_scalar(1747, if s.b[1747] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1747]) {
            s.store_div_scaled_inputs2_indices(1546, 1467, 0.5, 1532, 0.5, 1460, 1.0);
            s.store_scaled_add_offset_sqrt_square_offset(1546, 1546, 1e-5, (-1e-5), 1.0, 0.5);
            s.store_sub_scaled_ad_lhs(1, A::sqrt(A::add_scaled_product(A::div(s.ad_value(1546), s.ad_value(227)), 1.0, s.ad_value(250), s.ad_value(250), 0.25)), 250, 0.5);
            s.store_mul_square_lhs(1547, 1, 227);
            s.store_sub_from_scalar_div_indices(1548, 1.0, 1547, 1546);
        }

        s.b[1748] = ((s.v[1532] / 2.0) < 80.0);
        s.store_scalar(1748, if s.b[1748] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1748]) {
            s.store_ln_one_plus_exp_scaled_input(2, 1532, 0.5);
        }

        if (s.b[1608] && (!s.b[1748])) {
            s.store_scale(2, 1532, 0.5);
        }

        if s.b[1608] {
            s.store_scale(1549, 2, 2.0);
        }

        s.b[1749] = ((s.v[1533] / 2.0) < 80.0);
        s.store_scalar(1749, if s.b[1749] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1749]) {
            s.store_ln_one_plus_exp_scaled_input(3, 1533, 0.5);
        }

        if (s.b[1608] && (!s.b[1749])) {
            s.store_scale(3, 1533, 0.5);
        }

        if s.b[1608] {
            s.store_scale(1550, 3, 2.0);
            s.store_sub(1551, 1550, 1533);
            s.store_sub(1552, 1549, 1532);
            s.store_add_scaled_products_indices(1553, 270, 1549, 1.0, 271, 1551, 1.0);
            s.store_add_scaled_products_indices(1554, 270, 1550, 1.0, 271, 1552, 1.0);
            s.store_scaled_add(1555, 1480, 1549, 0.5);
            s.store_scaled_add(1556, 1481, 1550, 0.5);
            s.store_div_from_scalar_add_ad(0, 1.0, s.ad_value(1555), s.ad_value(1556));
            s.store_mul3_lhs(1557, 1544, 1555, 0);
            s.store_mul3_lhs(1558, 1544, 1556, 0);
            s.store_scaled_add(1559, 1482, 1551, 0.5);
            s.store_scaled_add(1560, 1483, 1552, 0.5);
            s.store_scaled_add(1561, 1484, 1553, 0.5);
            s.store_scaled_add(1562, 1485, 1554, 0.5);
            s.store_mul_product3_mixed_iiia(1563, 1548, 1555, 191, A::exp(A::mul(s.ad_value(40), s.ad_value(295))), 1.0);
            s.store_mul_ad_product_rhs_mixed_ia(1564, 1556, 192, A::exp(A::mul(s.ad_value(40), s.ad_value(295))));
            s.store_add(1565, 1563, 1564);
            s.store_mul_add_scaled_product_rhs(2, 50, s.ad_value(1559), 1.0, s.ad_value(51), s.ad_value(1560), 1.0);
            s.store_scaled_add_sqrt_square_offset_ad(3, A::offset(s.ad_value(2), 1.0), 0.01, 0.5);
            s.store_scaled_add_sqrt_square_offset_ad(4, A::scale_offset(s.ad_value(2), 0.2, 1.0), 0.01, 0.5);
            s.store_div(1566, 3, 4);
            s.store_mul_ad_product_rhs(1567, 33, A::add_scaled_product(A::offset(A::mul(s.ad_value(41), s.ad_value(1559)), 1.0), 1.0, s.ad_value(42), s.ad_value(1560), 1.0), A::exp(A::mul_scaled_lhs(s.ad_value(44), -1.0, A::ln(A::add_scaled_product(A::offset(A::mul(s.ad_value(1557), s.ad_value(268)), 1.0), 1.0, s.ad_value(1558), s.ad_value(269), 1.0)))));
        }

        s.b[1750] = (s.v[56] == 0.0);
        s.store_scalar(1750, if s.b[1750] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1750]) {
            s.store_scalar(4, 1.0);
        }

        s.b[1751] = (s.v[56] < 0.0);
        s.store_scalar(1751, if s.b[1751] { 1.0 } else { 0.0 });

        if ((s.b[1608] && (!s.b[1750])) && s.b[1751]) {
            s.store_mul_exp_ad_rhs(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(1544), 1e-12))));
            s.store_sub_from_scalar(4, 1.0, 2);
        }

        if ((s.b[1608] && (!s.b[1750])) && (!s.b[1751])) {
            s.store_mul_exp_ad_rhs(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(1544), 1e-12))));
            s.store_div_from_scalar_offset_input(4, 1.0, 2, 1.0);
        }

        if s.b[1608] {
            s.store_mul_add_scaled_product_rhs(1568, 1492, s.ad_value(54), 1.0, s.ad_value(1544), s.ad_value(4), 1.0);
            s.store_add_scaled_inputs_product_first_ad(1569, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(1561)), 1e-6)))), 1.0), 1.0, 1567, 1.0, 38, 1568, 1.0);
            s.store_add_scaled_inputs_product_first_ad(1570, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(1562)), 1e-6)))), 1.0), 1.0, 1567, 1.0, 39, 1568, 1.0);
            s.store_div_scaled_product_add_scaled_denominator(1571, 1566, 1565, 1.0, A::div(s.ad_value(1563), s.ad_value(1569)), 1.0, A::div(s.ad_value(1564), s.ad_value(1570)), 1.0, 1.0);
            s.store_div_from_scalar_offset_input(1572, 1.0, 1544, 4.0);
        }

        s.b[1752] = (s.v[65] > 0.0);
        s.store_scalar(1752, if s.b[1752] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1752]) {
            s.store_div_from_scalar_offset_product(0, 1.0, 65, 1558, 1.0);
        }

        if (s.b[1608] && (!s.b[1752])) {
            s.store_sub_from_scalar_scaled_mul(0, 1.0, 65, 1558, 1.0);
        }

        if s.b[1608] {
            s.store_mul3_lhs(1573, 1544, 1572, 0);
            s.store_mul_ln_ad_lhs(1574, A::offset(A::div_scaled_inputs2(s.ad_value(339), 1.0, s.ad_value(1528), (-1.0), A::add_scaled_product(A::mul3(s.ad_value(67), s.ad_value(1544), s.ad_value(1544)), 1.0, s.ad_value(66), s.ad_value(227), 1.0), 1.0), 1.0), 1573);
            s.store_mul(1575, 1426, 1574);
            s.store_div_from_scalar_offset_ad(1576, 1.0, A::mul_offset_rhs(s.ad_value(1575), s.ad_value(1575), 1.0), 1.0);
            s.store_div_scaled_value_offset_denominator(1504, s.ad_value(1555), 100.0, s.ad_value(1555), 100.0, 1.0);
        }

        s.b[1753] = (s.v[61] < 0.0);
        s.store_scalar(1753, if s.b[1753] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1753]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1505, 1.0, 1.0, A::mul(s.ad_value(61), s.ad_value(1504)));
        }

        if (s.b[1608] && (!s.b[1753])) {
            s.store_offset_mul(1505, 61, 1504, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_28(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1608] {
            s.store_div_scaled_value_offset_denominator(1506, s.ad_value(1556), 100.0, s.ad_value(1556), 100.0, 1.0);
        }

        s.b[1754] = (s.v[62] < 0.0);
        s.store_scalar(1754, if s.b[1754] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1754]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1507, 1.0, 1.0, A::mul(s.ad_value(62), s.ad_value(1506)));
        }

        if (s.b[1608] && (!s.b[1754])) {
            s.store_offset_mul(1507, 62, 1506, 1.0);
        }

        if s.b[1608] {
            s.store_mul_ad_affine_product_rhs(1577, 1424, s.ad_value(1545), A::add(s.ad_value(1505), s.ad_value(1507)), 0.5, 0.0);
            s.store_div_ad_rhs(1578, 1577, A::mul(s.ad_value(1571), s.ad_value(1576)));
            s.store_square(1579, 1578);
            s.store_sqrt_offset_input(1580, 1579, 1.0);
            s.store_div_scaled_offset_numerator(1581, s.ad_value(1579), 1.5, 1.0, s.ad_value(1580), 1.0);
        }

        s.b[1755] = (p.p13 > 0.0);
        s.store_scalar(1755, if s.b[1755] { 1.0 } else { 0.0 });

        if (s.b[1608] && s.b[1755]) {
            s.store_mul_scaled_exp_ln_input_rhs(2, 258, 0.6, A::offset(A::square(s.ad_value(1555)), 60.0), (-0.1666666666667));
            s.store_mul_scaled_exp_ln_input_rhs(3, 258, 0.6, A::offset(A::square(s.ad_value(1556)), 60.0), (-0.1666666666667));
            s.store_div_scaled_offset_numerator(1582, A::mul(s.ad_value(1460), s.ad_value(2)), 1.0, 1.0, s.ad_value(1441), 1.0);
            s.store_div_scaled_offset_numerator(1583, A::mul(s.ad_value(1461), s.ad_value(3)), 1.0, 1.0, s.ad_value(1442), 1.0);
        }

        if (s.b[1608] && (!s.b[1755])) {
            s.store_scalar(1582, 1.0);
            s.store_scalar(1583, 1.0);
        }

        s.b[1756] = (s.v[1466] > 1e-6);
        s.store_scalar(1756, if s.b[1756] { 1.0 } else { 0.0 });

        s.b[1757] = (s.v[1531] > 1e-6);
        s.store_scalar(1757, if s.b[1757] { 1.0 } else { 0.0 });

        s.b[1758] = (((s.v[1540]) as f64).abs() < 0.01);
        s.store_scalar(1758, if s.b[1758] { 1.0 } else { 0.0 });

        if (((s.b[1608] && s.b[1756]) && s.b[1757]) && s.b[1758]) {
            s.store_div_scaled_inputs2_mixed_aia(0, A::offset(s.ad_value(1529), 2.0), 1.0, 1539, 0.5, A::mul_offset_lhs(s.ad_value(1530), 2.0, s.ad_value(1539)), 1.0);
            s.store_mul(2, 0, 1540);
            s.store_square(3, 2);
            s.store_add_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
            s.store_add_scaled_product_indices(5, 4, 1.0, 2, 3, (-1.0));
            s.store_div_scaled_inputs2_mixed_iaa(2, 1533, 1.0, A::mul3_scaled_output(s.ad_value(1534), A::sub(s.ad_value(0), A::div_from_scalar(1.0, s.ad_value(1539))), s.ad_value(5), 2.0), (-1.0), A::offset(s.ad_value(1530), 2.0), 1.0);
            s.store_div_scaled_inputs2_mixed_aii(1584, A::div_scaled_add_product(s.ad_value(1535), (-1.0), s.ad_value(1542), s.ad_value(1531), 1.0, s.ad_value(1539), 1.0), 1.0, 2, (-1.0), 1531, 1.0);
            s.store_div_scaled_product_offset_denominator(1585, s.ad_value(1584), s.ad_value(1531), 1.0, s.ad_value(1584), 1.0, 1.0);
        }

        if (((s.b[1608] && s.b[1756]) && s.b[1757]) && (!s.b[1758])) {
            s.store_sub_ad(1584, A::div_scaled_product_by_product(s.ad_value(1542), s.ad_value(1541), 1.0, s.ad_value(1539), s.ad_value(1540), 1.0), A::div_scaled_inputs2(A::div(s.ad_value(1535), s.ad_value(1539)), 1.0, A::div(s.ad_value(1536), s.ad_value(1540)), 1.0, s.ad_value(1531), 1.0));
            s.store_div_scaled_product_offset_denominator(1585, s.ad_value(1584), s.ad_value(1531), 1.0, s.ad_value(1584), 1.0, 1.0);
        }

        if ((s.b[1608] && s.b[1756]) && (!s.b[1757])) {
            s.copy_ad(1585, 1502);
        }

        if (s.b[1608] && s.b[1756]) {
            s.store_sub(2, 1585, 1509);
            s.store_offset_scaled_mul(3, 2, 2, 36.0, 1.0);
        }

        s.b[1759] = (((s.v[2]) as f64).abs() > 0.001);
        s.store_scalar(1759, if s.b[1759] { 1.0 } else { 0.0 });

        if ((s.b[1608] && s.b[1756]) && s.b[1759]) {
            s.store_sub(4, 1531, 1466);
            s.store_add_scaled_product_indices(1586, 4, 1.0, 1585, 1545, (-1.0));
            s.store_add_scaled_product_indices(1587, 4, 1.0, 1509, 1545, (-1.0));
            s.store_sqrt_square_add(1588, 1586, 3);
            s.store_sqrt_square_add(1589, 1587, 3);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1590, 0.25, 2, A::add_scaled_products3(s.ad_value(1589), s.ad_value(1586), 1.0, s.ad_value(1588), s.ad_value(1587), (-1.0), s.ad_value(3), A::ln(A::div_scaled_inputs2(s.ad_value(1587), 1.0, s.ad_value(1589), 1.0, A::add(s.ad_value(1586), s.ad_value(1588)), 1.0)), 1.0));
        }

        if ((s.b[1608] && s.b[1756]) && (!s.b[1759])) {
            s.store_mul(4, 1545, 2);
            s.store_div_scaled_product3_mixed_iiia(1590, 1545, 4, 4, ((-0.25) * 0.1666666666667), A::sqrt(s.ad_value(3)), 1.0);
        }

        if (s.b[1608] && (!s.b[1756])) {
            s.copy_ad(1585, 1502);
            s.store_scalar(1590, 0.0);
        }

        if s.b[1608] {
            s.store_add_scaled_inputs3_mixed_aii(1591, A::add_scaled_product(s.ad_value(1590), 1.0, s.ad_value(1544), s.ad_value(1545), 1.0), 1.0, 1466, 1.0, 1531, -1.0);
        }

        s.b[1760] = (s.v[1466] > 1e-6);
        s.store_scalar(1760, if s.b[1760] { 1.0 } else { 0.0 });

        s.b[1761] = (s.v[1591] > 1e-30);
        s.store_scalar(1761, if s.b[1761] { 1.0 } else { 0.0 });

        if ((s.b[1608] && s.b[1760]) && s.b[1761]) {
            s.store_div_add_scaled_inputs_rhs_mixed_ai(1592, 1475, A::div(s.ad_value(1471), s.ad_value(1466)), 1.0, 1478, -1.0);
            s.store_div_add_scaled_inputs_rhs_mixed_ai(1593, 1539, A::div(s.ad_value(1535), s.ad_value(1531)), 1.0, 1542, -1.0);
            s.store_div_scaled_inputs2_indices(1594, 1592, 1.0, 1593, (-1.0), 1591, 1.0);
            s.store_div_add_scaled_inputs_rhs_mixed_ai(1595, 1476, A::div(s.ad_value(1472), s.ad_value(1466)), 1.0, 1478, -1.0);
            s.store_div_add_scaled_inputs_rhs_mixed_ai(1596, 1540, A::div(s.ad_value(1536), s.ad_value(1531)), 1.0, 1542, -1.0);
            s.store_div_scaled_inputs2_indices(1597, 1595, 1.0, 1596, (-1.0), 1591, 1.0);
        }

        if ((s.b[1608] && s.b[1760]) && (!s.b[1761])) {
            s.store_scalar(1594, 0.0);
            s.store_scalar(1597, 0.0);
        }

        if (s.b[1608] && (!s.b[1760])) {
            s.store_mul_add_scaled_inputs_rhs(1598, 1497, A::div(s.ad_value(1434), s.ad_value(1500)), (-2.0), s.ad_value(1503), (-2.0));
            s.store_mul_add_scaled_inputs_rhs(1599, 1498, A::div(s.ad_value(1435), s.ad_value(1501)), (-2.0), s.ad_value(1503), (-2.0));
            s.store_mul_sub_lhs(0, 1599, 1598, 1503);
            s.store_mul(2, 1598, 1434);
            s.store_mul(3, 1599, 1435);
            s.store_add(4, 2, 3);
            s.store_offset_ad(5, A::add_scaled_products(s.ad_value(1497), s.ad_value(1434), 2.0, s.ad_value(1498), s.ad_value(1435), 2.0), 3.0);
            s.store_div_scaled_inputs3_mixed_iiai(1600, 3, 1.0, 0, 1.0, A::div(s.ad_value(4), s.ad_value(1500)), -1.0, 5, 1.0);
            s.store_div_scaled_inputs3_mixed_iiai(1601, 2, 1.0, 0, (-1.0), A::div(s.ad_value(4), s.ad_value(1501)), -1.0, 5, 1.0);
            s.store_mul_add_scaled_product_rhs(1594, 1500, s.ad_value(1503), -1.0, s.ad_value(1600), s.ad_value(1500), -1.0);
            s.store_mul_add_scaled_product_rhs(1597, 1501, s.ad_value(1503), -1.0, s.ad_value(1601), s.ad_value(1501), -1.0);
        }

        if s.b[1608] {
            s.store_mul(1602, 1594, 1581);
            s.store_mul(1603, 1597, 1581);
            s.store_scaled_sub(1604, 1532, 1467, 0.5);
            s.store_scaled_sub(1605, 1533, 1468, 0.5);
            s.store_mul(1606, 1604, 1602);
            s.store_mul(1607, 1605, 1603);
            s.copy_ad(440, 1428);
            s.copy_ad(441, 1432);
            s.copy_ad(442, 1433);
            s.copy_ad(443, 1434);
            s.copy_ad(444, 1435);
            s.copy_ad(445, 1462);
            s.copy_ad(446, 1463);
            s.copy_ad(447, 1447);
            s.copy_ad(448, 1446);
            s.copy_ad(449, 1450);
            s.copy_ad(450, 1451);
            s.copy_ad(451, 1452);
            s.copy_ad(452, 1453);
            s.copy_ad(453, 1454);
            s.copy_ad(454, 1457);
            s.copy_ad(455, 1459);
            s.copy_ad(456, 1460);
            s.copy_ad(457, 1461);
            s.copy_ad(458, 1467);
            s.copy_ad(459, 1468);
            s.copy_ad(460, 1479);
            s.copy_ad(461, 1532);
            s.copy_ad(462, 1533);
            s.copy_ad(463, 1543);
            s.copy_ad(464, 1544);
            s.copy_ad(465, 1548);
            s.copy_ad(466, 1557);
            s.copy_ad(467, 1558);
            s.copy_ad(468, 1579);
            s.copy_ad(469, 1582);
            s.copy_ad(470, 1583);
            s.copy_ad(471, 1604);
            s.copy_ad(472, 1605);
            s.copy_ad(473, 1606);
            s.copy_ad(474, 1607);
        }

        if (!s.b[1608]) {
            s.copy_ad(440, 383);
            s.copy_ad(441, 384);
            s.copy_ad(442, 385);
            s.copy_ad(443, 386);
            s.copy_ad(444, 387);
            s.copy_ad(445, 388);
            s.copy_ad(446, 389);
            s.copy_ad(447, 390);
            s.copy_ad(448, 391);
            s.copy_ad(449, 393);
            s.copy_ad(450, 394);
            s.copy_ad(451, 395);
            s.copy_ad(452, 396);
            s.copy_ad(453, 397);
            s.copy_ad(454, 398);
            s.copy_ad(455, 399);
            s.copy_ad(456, 401);
            s.copy_ad(457, 402);
            s.copy_ad(458, 404);
            s.copy_ad(459, 405);
            s.copy_ad(460, 406);
            s.copy_ad(461, 408);
            s.copy_ad(462, 409);
            s.copy_ad(463, 414);
        }

    }

    pub(super) fn stamp_reactive_block_29(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1608]) {
            s.copy_ad(464, 415);
            s.copy_ad(465, 416);
            s.copy_ad(466, 419);
            s.copy_ad(467, 420);
            s.copy_ad(468, 428);
            s.copy_ad(469, 430);
            s.copy_ad(470, 431);
            s.copy_ad(471, 436);
            s.copy_ad(472, 437);
            s.copy_ad(473, 438);
            s.copy_ad(474, 439);
        }

        s.store_div_scaled_product_mixed_iaa(0, 120, A::sub(s.ad_value(448), s.ad_value(446)), 1.0, A::scale_offset(s.ad_value(464), 0.25, 1.0), 1.0);

        s.store_add_scaled_inputs3_indices(1324, 458, 0.5, 461, 0.5, 0, 1.0);

        s.store_add_scaled_inputs3_indices(1325, 459, 0.5, 462, 0.5, 0, -1.0);

        s.b[1762] = (p.p13 > 0.0);
        s.store_scalar(1762, if s.b[1762] { 1.0 } else { 0.0 });

        if s.b[1762] {
            s.store_add_scaled_inputs3_mixed_iai(1326, 1324, 1.0, A::div(s.ad_value(466), s.ad_value(469)), 1.0, 466, -1.0);
            s.store_add_scaled_inputs3_mixed_iai(1327, 1325, 1.0, A::div(s.ad_value(467), s.ad_value(470)), 1.0, 467, -1.0);
        }

        if (!s.b[1762]) {
            s.copy_ad(1326, 1324);
            s.copy_ad(1327, 1325);
        }

        s.store_scaled_mul(2, 471, 473, 0.3333333333333);

        s.store_mul_scaled_offset_ad_rhs(3, 471, 0.1666666666667, A::mul_sub_from_scalar_rhs(s.ad_value(473), 1.0, A::scale(s.ad_value(473), 0.2)), 1.0);

        s.store_add_scaled_product_indices(1328, 3, 1.0, 1326, 465, 0.5);

        s.store_add_scaled_product_indices(1326, 2, 1.0, 1326, 465, 1.0);

        s.store_scaled_mul(2, 472, 474, 0.3333333333333);

        s.store_mul_scaled_offset_ad_rhs(3, 472, 0.1666666666667, A::mul_sub_from_scalar_rhs(s.ad_value(474), 1.0, A::scale(s.ad_value(474), 0.2)), 1.0);

        s.store_add_scaled_inputs(1329, 1327, 0.5, 3, 1.0);

        s.store_add(1327, 1327, 2);

        s.store_mul(0, 447, 287);

        s.store_mul(361, 0, 1326);

        s.store_mul(362, 0, 1327);

        s.store_mul_add_scaled_inputs_rhs(363, 0, s.ad_value(1328), -1.0, s.ad_value(1329), -1.0);

        s.b[1763] = (s.v[119] > 0.0);
        s.store_scalar(1763, if s.b[1763] { 1.0 } else { 0.0 });

        if s.b[1763] {
            s.store_offset(0, 254, (2.0 * 0.6931471805599));
            s.store_add(1330, 460, 0);
            s.store_add(1331, 463, 0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1332, 1330, 0.5, 254, 0.5, 1330, 254, 9.0, (-0.5));
            s.store_add_scaled_inputs4_mixed_iiia(1333, 1331, 0.5, 254, 0.5, 339, 0.5, A::sqrt_square_offset(A::add_scaled_inputs3(s.ad_value(1331), 1.0, s.ad_value(254), -1.0, s.ad_value(339), -1.0), 9.0), (-0.5));
            s.store_mul_sqrt_ad_rhs(1334, 294, A::mul_offset_rhs(s.ad_value(445), s.ad_value(444), 0.5));
            s.store_mul_sqrt_ad_rhs(1335, 294, A::mul_offset_rhs(A::mul3(s.ad_value(445), s.ad_value(456), s.ad_value(444)), s.ad_value(443), 0.5));
            s.store_mul_square_lhs(1336, 1334, 291);
            s.store_mul_square_lhs(1337, 1335, 291);
            s.store_sub(2, 292, 1332);
            s.store_add_scaled_inputs3_indices(3, 292, 1.0, 339, 1.0, 1333, -1.0);
            s.store_scale(0, 1336, 2.0);
            s.store_add_scaled_offset_product_rhs_mixed_iia(1338, 1332, 1.0, 0, A::sqrt(A::offset(A::div(s.ad_value(2), s.ad_value(1336)), 1.0)), (-1.0), 1.0);
            s.store_add_scaled_offset_product_rhs_mixed_iia(1339, 1333, 1.0, 0, A::sqrt(A::offset(A::div(s.ad_value(3), s.ad_value(1336)), 1.0)), (-1.0), 1.0);
            s.store_scale(0, 1337, 2.0);
            s.store_add_scaled_offset_product_rhs_mixed_iia(1340, 1332, 1.0, 0, A::sqrt(A::offset(A::div(s.ad_value(2), s.ad_value(1337)), 1.0)), (-1.0), 1.0);
            s.store_add_scaled_offset_product_rhs_mixed_iia(1341, 1333, 1.0, 0, A::sqrt(A::offset(A::div(s.ad_value(3), s.ad_value(1337)), 1.0)), (-1.0), 1.0);
            s.store_mul(0, 293, 447);
            s.store_mul_product3_indices(2, 451, 0, 1334, 456, -1.0);
            s.store_mul_product3_indices(3, 452, 0, 1335, 457, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(0, 1338, 0.5, 1330, ((-1.0) * 0.5), 1338, 1330, 1.0, 0.5);
            s.store_div_scaled_product3_mixed_iiia(379, 2, 0, 0, 1.0, A::sub(s.ad_value(1338), s.ad_value(1332)), 1.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(0, 1339, 0.5, 1331, ((-1.0) * 0.5), 1339, 1331, 1.0, 0.5);
            s.store_div_scaled_product3_mixed_iiia(380, 2, 0, 0, 1.0, A::sub(s.ad_value(1339), s.ad_value(1333)), 1.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(0, 1340, 0.5, 1330, ((-1.0) * 0.5), 1340, 1330, 1.0, 0.5);
            s.store_div_scaled_product3_mixed_iiia(381, 3, 0, 0, 1.0, A::sub(s.ad_value(1340), s.ad_value(1332)), 1.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(0, 1341, 0.5, 1331, ((-1.0) * 0.5), 1341, 1331, 1.0, 0.5);
            s.store_div_scaled_product3_mixed_iiia(382, 3, 0, 0, 1.0, A::sub(s.ad_value(1341), s.ad_value(1333)), 1.0);
        }

        if (!s.b[1763]) {
            s.store_scalar(379, 0.0);
            s.store_scalar(380, 0.0);
            s.store_scalar(381, 0.0);
            s.store_scalar(382, 0.0);
        }

        s.store_mul(370, 164, 330);

        s.store_mul(371, 165, 332);

        s.store_scaled_add_sqrt_square_offset_ad(0, A::sub_from_scalar(1.0, A::mul3(s.ad_value(161), s.ad_value(449), A::sub_from_scalar(1.0, A::mul(s.ad_value(162), s.ad_value(440))))), 0.2, 0.5);

        s.store_mul3_lhs(372, 159, 349, 0);

        s.store_mul3_lhs(373, 160, 350, 0);

        s.store_mul(374, 117, 338);

        s.store_mul(375, 166, 336);

        s.store_mul_neg_ad_lhs(377, A::add_scaled_products(s.ad_value(240), s.ad_value(9), 1.0, s.ad_value(167), s.ad_value(11), 1.0), 331);

        s.store_mul_neg_ad_lhs(376, A::add_scaled_products(s.ad_value(240), s.ad_value(10), 1.0, s.ad_value(167), s.ad_value(12), 1.0), 333);

        s.b[1764] = (s.v[6] > 0.0);
        s.store_scalar(1764, if s.b[1764] { 1.0 } else { 0.0 });

        if s.b[1764] {
            s.store_mul(378, 170, 219);
        }

        if (!s.b[1764]) {
            s.store_scalar(378, 0.0);
        }

        s.copy_ad(1774, 361);

        s.copy_ad(1775, 362);

        s.copy_ad(1776, 363);

        s.store_add_scaled_inputs3_indices(364, 361, (-1.0), 362, (-1.0), 363, (-1.0));

        s.b[1777] = (s.v[334] < 0.0);
        s.store_scalar(1777, if s.b[1777] { 1.0 } else { 0.0 });

        if s.b[1777] {
            s.copy_ad(1776, 364);
        }

        s.store_scaled_mul(361, 13, 361, p.p32);

        s.store_scaled_mul(362, 13, 362, p.p32);

        s.store_scaled_mul(363, 13, 363, p.p32);

        s.store_add_scaled_inputs3_indices(364, 361, (-1.0), 362, (-1.0), 363, (-1.0));

        s.store_scaled_mul(379, 13, 379, p.p32);

        s.store_scaled_mul(380, 13, 380, p.p32);

        s.store_scaled_mul(381, 13, 381, p.p32);

        s.store_scaled_mul(382, 13, 382, p.p32);

        s.store_scaled_mul(370, 13, 370, p.p32);

        s.store_scaled_mul(371, 13, 371, p.p32);

        s.store_scaled_mul(372, 13, 372, p.p32);

        s.store_scaled_mul(373, 13, 373, p.p32);

        s.store_scaled_mul(374, 13, 374, p.p32);

        s.store_scaled_mul(377, 13, 377, p.p32);

        s.store_scaled_mul(376, 13, 376, p.p32);

        s.store_scaled_mul(375, 13, 375, p.p32);

        s.store_mul(378, 13, 378);

        s.b[1778] = (s.v[334] < 0.0);
        s.store_scalar(1778, if s.b[1778] { 1.0 } else { 0.0 });

        if s.b[1778] {
            s.copy_ad(1772, 363);
            s.copy_ad(363, 364);
            s.copy_ad(364, 1772);
            s.store_neg(375, 375);
            s.copy_ad(1772, 380);
            s.copy_ad(380, 379);
            s.copy_ad(379, 1772);
            s.copy_ad(1772, 382);
            s.copy_ad(382, 381);
            s.copy_ad(381, 1772);
        }

        s.b[1779] = (s.v[13] > 0.0);
        s.store_scalar(1779, if s.b[1779] { 1.0 } else { 0.0 });

        if s.b[1779] {
            s.store_mul_div_scaled_inputs_mixed_aia(1773, A::add_scaled_product(A::div_scaled_product_by_product(s.ad_value(179), A::add(s.ad_value(1774), s.ad_value(1775)), 1.0, s.ad_value(116), s.ad_value(239), 1.0), 1.0, s.ad_value(180), s.ad_value(226), 1.0), 342, 1e-9, A::mul(s.ad_value(345), s.ad_value(116)), 1.0);
        }

        if (!s.b[1779]) {
            s.store_scalar(1773, 0.0);
        }

        s.store_scaled_mul(1780, 390, 226, 1.0 / (1.602176565e-19));

        s.store_scaled_add(1781, 407, 432, (-0.5));

        s.store_add(1782, 415, 1781);

        s.store_div(0, 415, 1782);

        s.store_scaled_add_sqrt_square_offset_rhs(1787, 0, 0, 1e-20, 0.5);

        s.store_scaled_mul(1788, 436, 435, (-0.1666666666667));

        s.store_square(1789, 1788);

        s.store_offset(1790, 429, (-1.0));

        s.store_scale(1794, 1789, 12.0);

        s.store_add_scaled_inputs3_mixed_iia(2, 1787, 1.0, 1794, 1.0, A::mul3_scaled_output(A::offset(s.ad_value(1787), 1.0), s.ad_value(1794), s.ad_value(1790), 2.0), -1.0);

        s.store_max_with_scalar(3, 2, 1e-40);

        s.store_div_scaled_product3_indices(1799, 456, 447, 116, 1.0, 469, 1.0);

        s.store_mul_offset_lhs(1800, 468, 1.0, 1799);

        s.store_mul_sub_from_scalar_ad_rhs(1802, 1800, 0.5, A::mul_scaled_lhs(s.ad_value(334), 0.25, s.ad_value(1788)));

        s.store_sub(1801, 1800, 1802);

        s.b[1813] = (p.p6 > 0.0);
        s.store_scalar(1813, if s.b[1813] { 1.0 } else { 0.0 });

        if s.b[1813] {
            s.store_sub_ad(2, A::add_scaled_product(s.ad_value(1787), 0.08333333333333333, s.ad_value(1789), A::sub(A::offset(s.ad_value(1787), 0.2), s.ad_value(1794)), (-1.0)), A::mul3_scaled_output(s.ad_value(1789), A::sub(A::offset(s.ad_value(1787), 1.0), s.ad_value(1794)), s.ad_value(1790), 1.6));
            s.store_max_with_scalar(3, 2, 1e-40);
        }

        s.copy_ad(1783, 1780);

        s.store_mul_offset_rhs(1784, 1780, 415, 1.0);

        s.store_mul_sub_rhs(1785, 1780, 403, 413);

        s.store_mul_add(2, A::ln(A::div_scaled_inputs2(s.ad_value(1784), 1.0, s.ad_value(1785), 0.5, A::sub_scaled_inputs(s.ad_value(1784), 1.0, s.ad_value(1785), 0.5), 1.0)), A::add_scaled_product(s.ad_value(173), 1.0, s.ad_value(174), s.ad_value(1783), (-1.0)), A::mul3(s.ad_value(175), s.ad_value(1783), s.ad_value(1783)));

        s.store_add_scaled_product_left_ad(3, 2, 1.0, A::add_scaled_product(s.ad_value(174), 1.0, s.ad_value(175), A::sub_scaled_inputs(s.ad_value(1784), 1.0, s.ad_value(1783), 2.0), 1.0), 1785, 1.0);

        s.store_offset_div_ad(0, A::add_scaled_products(s.ad_value(176), s.ad_value(417), 1.0, s.ad_value(177), s.ad_value(418), 1.0), A::offset(s.ad_value(415), 1.0), 1.0);

        s.store_scaled_add_offset_sqrt_square_offset(4, 0, 0.01, (-0.01), 0.0001, 0.5);

        s.store_mul_div_scaled_product_mixed_iaii(0, 4, A::div_scaled_product(s.ad_value(347), s.ad_value(348), 1.602176565e-19, s.ad_value(345), 1.0), 3, 1.0, 1783, 1.0);

        s.store_div_from_scalar_scaled_input(1823, 1.0, 8, 8.617332384961e-5);

        s.store_sub_from_scalar_ad(1824, 1.17, A::div_scaled_product_offset_denominator(s.ad_value(8), s.ad_value(8), 0.000473, s.ad_value(8), 636.0, 1.0));

        s.store_sub_from_scalar_ad(1825, 0.744, A::div_scaled_product_offset_denominator(s.ad_value(8), s.ad_value(8), 0.0004774, s.ad_value(8), 235.0, 1.0));

        s.store_mul_add_scaled_inputs3_offset_rhs(1826, 15, s.ad_value(1825), 1.0, s.ad_value(1824), (-1.0), s.ad_value(228), (-0.4), 0.0);

        s.store_add(1827, 1824, 1826);

        s.store_scaled_mul(1828, 1827, 1823, 0.5);

        s.store_sub_scaled_inputs(1829, 15, 0.05, 1826, 0.5);

        s.store_sqrt_scaled_input(0, 8, 0.0033333333333);

        s.store_mul3_affine_lhs(2, 0, 0, 4.05e25, 0.0, 0);

        s.store_mul(1830, 2, 238);

        s.store_div_scaled_value_offset_denominator(1831, s.ad_value(1823), 1.0, A::div_scaled_inputs(s.ad_value(17), s.v[7], s.ad_value(8), 1.0), 1.0, 1.0);

        s.store_mul3_affine_lhs(1833, 1830, 229, (2.0 * 1.602176565e-19), 0.0, 1831);

        s.store_add_offset_ad_lhs(1834, A::ln(A::div_scaled_product(s.ad_value(245), s.ad_value(245), 1.0, s.ad_value(1833), 1.0)), (-0.6931471805599), 1828);

        s.store_mul_div_scaled_product_mixed_iiia(1835, 1831, 29, 14, (0.5 * 1.602176565e-19), A::add(s.ad_value(241), s.ad_value(242)), 1.0);

        s.store_mul(1838, 35, 1831);

        s.store_scalar(1839, 0.0);

        s.store_scalar(1832, 0.0);

        s.b[1884] = (p.p9 > 0.0);
        s.store_scalar(1884, if s.b[1884] { 1.0 } else { 0.0 });

        if s.b[1884] {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1832, 1.0, 1823, A::ln(A::div(s.ad_value(24), s.ad_value(251))));
        }

        s.b[1885] = (p.p13 > 0.0);
        s.store_scalar(1885, if s.b[1885] { 1.0 } else { 0.0 });

        s.b[1886] = (p.p14 == 1.0);
        s.store_scalar(1886, if s.b[1886] { 1.0 } else { 0.0 });

        if (s.b[1885] && s.b[1886]) {
            s.store_scale_ad(1839, A::exp_scaled_input(A::ln(A::div(s.ad_value(259), s.ad_value(1831))), (-0.3333333333333)), ((0.4 * p.p13) * 1.27520989));
        }

        if (s.b[1885] && (!s.b[1886])) {
            s.store_scale_ad(1839, A::exp_scaled_input(A::ln(A::div(s.ad_value(259), s.ad_value(1831))), (-0.3333333333333)), ((0.4 * p.p13) * 1.5412087));
        }

        s.store_mul(1842, 336, 1831);

        s.store_mul_offset_ad_lhs(1843, A::sqrt_square_offset(s.ad_value(336), 0.01), (-0.1), 1831);

        s.store_scaled_sub(1844, 1842, 1843, 0.5);

        s.store_div_scaled_value_by_product(1815, s.ad_value(402), 1.0, s.ad_value(401), A::offset(s.ad_value(402), 1.0), 1.0);

        s.store_div_scaled_value_by_product(1816, s.ad_value(401), 1.0, s.ad_value(402), A::offset(s.ad_value(401), 1.0), 1.0);

        s.store_offset_ln_ad(1817, A::div_scaled_product3(s.ad_value(401), A::offset(s.ad_value(1815), 1.0), s.ad_value(384), 1.0, s.ad_value(385), 1.0), 2.0);

        s.store_offset_ln_ad(1818, A::div_scaled_product3(s.ad_value(402), A::offset(s.ad_value(1816), 1.0), s.ad_value(384), 1.0, s.ad_value(385), 1.0), 2.0);

        s.store_add_scaled_products_left_left_ad(1819, A::offset(s.ad_value(1815), 1.0), 1817, 1.0, 399, 1815, (-1.0));

        s.store_add_scaled_offset_product_lhs_mixed_aai(1820, A::div(s.ad_value(399), s.ad_value(1816)), (-1.0), A::div_from_scalar(1.0, s.ad_value(1816)), 1.0, 1818, 1.0);

        s.store_add_ad_lhs(1821, A::div_scaled_inputs4(s.ad_value(1819), 0.5, s.ad_value(1820), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1819), s.ad_value(1820)), 38.0), (-0.5), s.ad_value(398), -1.0, s.ad_value(25), 1.0), 398);

        s.store_add_scaled_product_right_ad(1822, 21, 1.0, 226, A::add_scaled_inputs3(A::div_scaled_inputs2(s.ad_value(1821), 1.0, s.ad_value(394), (-1.0), s.ad_value(395), 1.0), 1.0, s.ad_value(397), (-1.0), s.ad_value(394), 1.0), 1.0);

        s.store_mul_offset_rhs(0, 34, 8, (-s.v[7]));

        s.store_add_scaled_offset_product_rhs(0, 256, 1.0, 23, 8, (-s.v[7]), p.p14);

        s.store_sub_offset_ad_lhs(1840, A::add_scaled_inputs4(s.ad_value(183), p.p14, s.ad_value(1829), p.p14, s.ad_value(243), p.p14, s.ad_value(0), 1.0), p.p34, 1832);

        s.store_add_scaled_inputs4_indices(1841, 184, p.p14, 1829, p.p14, 244, p.p14, 0, 1.0);

        s.store_add_scaled_product_left_ad(1845, 1844, (-1.0), A::sub(s.ad_value(1822), s.ad_value(1840)), 1831, 1.0);

        s.store_add_scaled_product_left_ad(1846, 1844, (-1.0), A::sub_scaled_inputs(s.ad_value(337), -1.0, s.ad_value(1841), 1.0), 1831, 1.0);

        s.b[1887] = (p.p2 > 0.0);
        s.store_scalar(1887, if s.b[1887] { 1.0 } else { 0.0 });

        if s.b[1887] {
            s.store_div_scaled_product_right_ad(0, 16, A::sub(s.ad_value(1845), s.ad_value(1846)), p.p14, 260, 1.0);
        }

        s.b[1888] = (s.v[0] < 0.0);
        s.store_scalar(1888, if s.b[1888] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_30(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1887] && s.b[1888]) {
            s.store_scaled_ln_ad(2, A::sub_from_scalar(1.0, s.ad_value(0)), (-2.0));
        }

        if (s.b[1887] && (!s.b[1888])) {
            s.store_div_scaled_product_offset_denominator(2, s.ad_value(0), s.ad_value(0), 1.0, A::div_scaled_inputs(s.ad_value(0), 2.0, s.ad_value(260), 1.0), 1.0, 1.0);
        }

        if s.b[1887] {
            s.store_add_scaled_product_indices(1847, 1846, 1.0, 16, 2, p.p14);
        }

        if (!s.b[1887]) {
            s.copy_ad(1847, 1846);
        }

        s.store_mul_sub_rhs(0, 248, 1845, 1847);

        s.b[1889] = (p.p13 > 0.0);
        s.store_scalar(1889, if s.b[1889] { 1.0 } else { 0.0 });

        if s.b[1889] {
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1848, 0, 0.5, 257, 0.5, A::add(A::square(A::sub(s.ad_value(0), s.ad_value(257))), A::square(s.ad_value(257))), 0.5);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1849, 257, 0.5, 0, ((-1.0) * 0.5), A::add(A::square(A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(257), 1.0)), A::square(s.ad_value(257))), 0.5);
            s.store_mul_ad_rhs(2, 1839, A::exp_scaled_input(A::ln(s.ad_value(1848)), (-0.3333333333333)));
            s.store_mul_ad_rhs(3, 1839, A::exp_scaled_input(A::ln(s.ad_value(1849)), (-0.3333333333333)));
            s.store_sub_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
            s.store_div_scaled_product_offset_denominator(1851, s.ad_value(246), s.ad_value(4), 1.0, A::mul(s.ad_value(246), s.ad_value(2)), 1.0, 1.0);
            s.store_div_scaled_product_offset_denominator(1852, s.ad_value(247), s.ad_value(4), 1.0, A::mul(s.ad_value(247), s.ad_value(3)), 1.0, 1.0);
            s.store_div_from_scalar_add_ad(1853, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(1851)), 1.0), A::div_from_scalar(1.0, s.ad_value(1852)));
        }

        if (!s.b[1889]) {
            s.copy_ad(1851, 246);
            s.copy_ad(1852, 247);
            s.copy_ad(1853, 248);
        }

        s.store_mul_sub_rhs(1854, 1853, 1845, 1847);

        s.b[1890] = (s.v[1854] > 0.0);
        s.store_scalar(1890, if s.b[1890] { 1.0 } else { 0.0 });

        s.b[1891] = ((-s.v[1854]) < 80.0);
        s.store_scalar(1891, if s.b[1891] { 1.0 } else { 0.0 });

        if (s.b[1890] && s.b[1891]) {
            s.store_ln_one_plus_exp_neg_input(0, 1854);
        }

        if (s.b[1890] && (!s.b[1891])) {
            s.store_neg(0, 1854);
        }

        if s.b[1890] {
            s.store_add_scaled_inputs3_offset_mixed_iai(1855, 1845, 1.0, A::div(s.ad_value(1854), s.ad_value(1851)), (-1.0), 0, 1.0, (-0.6931471805599));
        }

        s.b[1892] = (s.v[1854] < 80.0);
        s.store_scalar(1892, if s.b[1892] { 1.0 } else { 0.0 });

        if ((!s.b[1890]) && s.b[1892]) {
            s.store_ln_one_plus_exp(0, 1854);
        }

        if ((!s.b[1890]) && (!s.b[1892])) {
            s.copy_ad(0, 1854);
        }

        if (!s.b[1890]) {
            s.store_add_scaled_inputs3_offset_mixed_iai(1855, 1847, 1.0, A::div(s.ad_value(1854), s.ad_value(1852)), 1.0, 0, 1.0, (-0.6931471805599));
        }

        s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1856, 1855, 0.5, 1834, 0.5, 1855, 1834, 4.0, (-0.5));

        s.store_offset_sqrt_ad(1857, A::offset(A::div_scaled_inputs2(s.ad_value(1834), 2.0, s.ad_value(1856), (-2.0), s.ad_value(1835), 1.0), 1.0), (-1.0));

        s.store_scaled_add_offset_sqrt_square_offset_ad(0, A::mul(s.ad_value(30), s.ad_value(1846)), ((1.0) + (0.5)), ((1.0) + ((-0.5))), 0.01, 0.5);

        s.store_mul_offset_rhs_ad(0, A::mul3_scaled_output(s.ad_value(1838), A::offset(A::sqrt(A::offset(A::div(s.ad_value(1843), s.ad_value(1838)), 1.0)), (-1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(1857)), 1.0), 2.0), A::mul(s.ad_value(37), s.ad_value(1846)), 1.0);

        s.b[1894] = (p.p11 > 0.0);
        s.store_scalar(1894, if s.b[1894] { 1.0 } else { 0.0 });

        if s.b[1894] {
            s.store_div_scaled_value_by_product(1815, s.ad_value(457), 1.0, s.ad_value(456), A::offset(s.ad_value(457), 1.0), 1.0);
            s.store_div_scaled_value_by_product(1816, s.ad_value(456), 1.0, s.ad_value(457), A::offset(s.ad_value(456), 1.0), 1.0);
            s.store_offset_ln_ad(1817, A::div_scaled_product3(s.ad_value(456), A::offset(s.ad_value(1815), 1.0), s.ad_value(441), 1.0, s.ad_value(442), 1.0), 2.0);
            s.store_offset_ln_ad(1818, A::div_scaled_product3(s.ad_value(457), A::offset(s.ad_value(1816), 1.0), s.ad_value(441), 1.0, s.ad_value(442), 1.0), 2.0);
            s.store_add_scaled_products_left_left_ad(1819, A::offset(s.ad_value(1815), 1.0), 1817, 1.0, 455, 1815, (-1.0));
            s.store_add_scaled_offset_product_lhs_mixed_aai(1820, A::div(s.ad_value(455), s.ad_value(1816)), (-1.0), A::div_from_scalar(1.0, s.ad_value(1816)), 1.0, 1818, 1.0);
            s.store_add_ad_lhs(1821, A::div_scaled_inputs4(s.ad_value(1819), 0.5, s.ad_value(1820), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1819), s.ad_value(1820)), 38.0), (-0.5), s.ad_value(454), -1.0, s.ad_value(25), 1.0), 454);
            s.store_add_scaled_product_right_ad(1822, 130, 1.0, 226, A::add_scaled_inputs3(A::div_scaled_inputs2(s.ad_value(1821), 1.0, s.ad_value(450), (-1.0), s.ad_value(451), 1.0), 1.0, s.ad_value(453), (-1.0), s.ad_value(450), 1.0), 1.0);
            s.store_mul_offset_rhs(0, 34, 8, (-s.v[7]));
            s.store_add_scaled_offset_product_rhs(0, 256, 1.0, 23, 8, (-s.v[7]), p.p14);
            s.store_sub_offset_ad_lhs(1840, A::add_scaled_inputs4(s.ad_value(185), p.p14, s.ad_value(1829), p.p14, s.ad_value(243), p.p14, s.ad_value(0), 1.0), p.p34, 1832);
            s.store_add_scaled_inputs4_indices(1841, 186, p.p14, 1829, p.p14, 244, p.p14, 0, 1.0);
            s.store_add_scaled_product_left_ad(1845, 1844, (-1.0), A::sub(s.ad_value(1822), s.ad_value(1840)), 1831, 1.0);
            s.store_add_scaled_product_left_ad(1846, 1844, (-1.0), A::sub_scaled_inputs(s.ad_value(337), -1.0, s.ad_value(1841), 1.0), 1831, 1.0);
        }

        s.b[1895] = (p.p2 > 0.0);
        s.store_scalar(1895, if s.b[1895] { 1.0 } else { 0.0 });

        if (s.b[1894] && s.b[1895]) {
            s.store_div_scaled_product_right_ad(0, 16, A::sub(s.ad_value(1845), s.ad_value(1846)), p.p14, 260, 1.0);
        }

        s.b[1896] = (s.v[0] < 0.0);
        s.store_scalar(1896, if s.b[1896] { 1.0 } else { 0.0 });

        if ((s.b[1894] && s.b[1895]) && s.b[1896]) {
            s.store_scaled_ln_ad(2, A::sub_from_scalar(1.0, s.ad_value(0)), (-2.0));
        }

        if ((s.b[1894] && s.b[1895]) && (!s.b[1896])) {
            s.store_div_scaled_product_offset_denominator(2, s.ad_value(0), s.ad_value(0), 1.0, A::div_scaled_inputs(s.ad_value(0), 2.0, s.ad_value(260), 1.0), 1.0, 1.0);
        }

        if (s.b[1894] && s.b[1895]) {
            s.store_add_scaled_product_indices(1847, 1846, 1.0, 16, 2, p.p14);
        }

        if (s.b[1894] && (!s.b[1895])) {
            s.copy_ad(1847, 1846);
        }

        if s.b[1894] {
            s.store_mul_sub_rhs(0, 248, 1845, 1847);
        }

        s.b[1897] = (p.p13 > 0.0);
        s.store_scalar(1897, if s.b[1897] { 1.0 } else { 0.0 });

        if (s.b[1894] && s.b[1897]) {
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1848, 0, 0.5, 257, 0.5, A::add(A::square(A::sub(s.ad_value(0), s.ad_value(257))), A::square(s.ad_value(257))), 0.5);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1849, 257, 0.5, 0, ((-1.0) * 0.5), A::add(A::square(A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(257), 1.0)), A::square(s.ad_value(257))), 0.5);
            s.store_mul_ad_rhs(2, 1839, A::exp_scaled_input(A::ln(s.ad_value(1848)), (-0.3333333333333)));
            s.store_mul_ad_rhs(3, 1839, A::exp_scaled_input(A::ln(s.ad_value(1849)), (-0.3333333333333)));
            s.store_sub_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
            s.store_div_scaled_product_offset_denominator(1851, s.ad_value(246), s.ad_value(4), 1.0, A::mul(s.ad_value(246), s.ad_value(2)), 1.0, 1.0);
            s.store_div_scaled_product_offset_denominator(1852, s.ad_value(247), s.ad_value(4), 1.0, A::mul(s.ad_value(247), s.ad_value(3)), 1.0, 1.0);
            s.store_div_from_scalar_add_ad(1853, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(1851)), 1.0), A::div_from_scalar(1.0, s.ad_value(1852)));
        }

        if (s.b[1894] && (!s.b[1897])) {
            s.copy_ad(1851, 246);
            s.copy_ad(1852, 247);
            s.copy_ad(1853, 248);
        }

        if s.b[1894] {
            s.store_mul_sub_rhs(1854, 1853, 1845, 1847);
        }

        s.b[1898] = (s.v[1854] > 0.0);
        s.store_scalar(1898, if s.b[1898] { 1.0 } else { 0.0 });

        s.b[1899] = ((-s.v[1854]) < 80.0);
        s.store_scalar(1899, if s.b[1899] { 1.0 } else { 0.0 });

        if ((s.b[1894] && s.b[1898]) && s.b[1899]) {
            s.store_ln_one_plus_exp_neg_input(0, 1854);
        }

        if ((s.b[1894] && s.b[1898]) && (!s.b[1899])) {
            s.store_neg(0, 1854);
        }

        if (s.b[1894] && s.b[1898]) {
            s.store_add_scaled_inputs3_offset_mixed_iai(1855, 1845, 1.0, A::div(s.ad_value(1854), s.ad_value(1851)), (-1.0), 0, 1.0, (-0.6931471805599));
        }

        s.b[1900] = (s.v[1854] < 80.0);
        s.store_scalar(1900, if s.b[1900] { 1.0 } else { 0.0 });

        if ((s.b[1894] && (!s.b[1898])) && s.b[1900]) {
            s.store_ln_one_plus_exp(0, 1854);
        }

        if ((s.b[1894] && (!s.b[1898])) && (!s.b[1900])) {
            s.copy_ad(0, 1854);
        }

        if (s.b[1894] && (!s.b[1898])) {
            s.store_add_scaled_inputs3_offset_mixed_iai(1855, 1847, 1.0, A::div(s.ad_value(1854), s.ad_value(1852)), 1.0, 0, 1.0, (-0.6931471805599));
        }

        if s.b[1894] {
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1856, 1855, 0.5, 1834, 0.5, 1855, 1834, 4.0, (-0.5));
            s.store_offset_sqrt_ad(1857, A::offset(A::div_scaled_inputs2(s.ad_value(1834), 2.0, s.ad_value(1856), (-2.0), s.ad_value(1835), 1.0), 1.0), (-1.0));
            s.store_scaled_add_offset_sqrt_square_offset_ad(0, A::mul(s.ad_value(30), s.ad_value(1846)), ((1.0) + (0.5)), ((1.0) + ((-0.5))), 0.01, 0.5);
            s.store_mul_offset_rhs_ad(0, A::mul3_scaled_output(s.ad_value(1838), A::offset(A::sqrt(A::offset(A::div(s.ad_value(1843), s.ad_value(1838)), 1.0)), (-1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(1857)), 1.0), 2.0), A::mul(s.ad_value(37), s.ad_value(1846)), 1.0);
        }

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        var_fracinv_i: f64,
        var_guard1239: f64,
        var_idse: f64,
        var_idse_dn4: f64,
        var_idse_dn6: f64,
        var_idse_dn7: f64,
        var_idse_dn8: f64,
        var_idse_dn9: f64,
        var_igde: f64,
        var_igde_dn4: f64,
        var_igde_dn6: f64,
        var_igde_dn7: f64,
        var_igde_dn8: f64,
        var_igde_dn9: f64,
        var_igidle: f64,
        var_igidle_dn4: f64,
        var_igidle_dn6: f64,
        var_igidle_dn7: f64,
        var_igidle_dn8: f64,
        var_igidle_dn9: f64,
        var_igisle: f64,
        var_igisle_dn4: f64,
        var_igisle_dn6: f64,
        var_igisle_dn7: f64,
        var_igisle_dn8: f64,
        var_igisle_dn9: f64,
        var_igse: f64,
        var_igse_dn4: f64,
        var_igse_dn6: f64,
        var_igse_dn7: f64,
        var_igse_dn8: f64,
        var_igse_dn9: f64,
        var_itaueff: f64,
        var_itaueff_dn4: f64,
        var_itaueff_dn6: f64,
        var_itaueff_dn7: f64,
        var_itaueff_dn8: f64,
        var_itaueff_dn9: f64,
        var_ithpwre: f64,
        var_ithpwre_dn4: f64,
        var_ithpwre_dn6: f64,
        var_ithpwre_dn7: f64,
        var_ithpwre_dn8: f64,
        var_ithpwre_dn9: f64,
        var_ithrce: f64,
        var_ithrce_dn4: f64,
        var_ithrce_dn6: f64,
        var_ithrce_dn7: f64,
        var_ithrce_dn8: f64,
        var_ithrce_dn9: f64,
        var_kfracinv_i: f64,
        var_mult_i_int: f64,
        var_qb: f64,
        var_qb_dn4: f64,
        var_qb_dn6: f64,
        var_qb_dn7: f64,
        var_qb_dn8: f64,
        var_qb_dn9: f64,
        var_qb_wo_mult: f64,
        var_qb_wo_mult_dn4: f64,
        var_qb_wo_mult_dn6: f64,
        var_qb_wo_mult_dn7: f64,
        var_qb_wo_mult_dn8: f64,
        var_qb_wo_mult_dn9: f64,
        var_qbdif: f64,
        var_qbdif_dn4: f64,
        var_qbdif_dn6: f64,
        var_qbdif_dn7: f64,
        var_qbdif_dn8: f64,
        var_qbdif_dn9: f64,
        var_qbsif: f64,
        var_qbsif_dn4: f64,
        var_qbsif_dn6: f64,
        var_qbsif_dn7: f64,
        var_qbsif_dn8: f64,
        var_qbsif_dn9: f64,
        var_qd_wo_mult: f64,
        var_qd_wo_mult_dn4: f64,
        var_qd_wo_mult_dn6: f64,
        var_qd_wo_mult_dn7: f64,
        var_qd_wo_mult_dn8: f64,
        var_qd_wo_mult_dn9: f64,
        var_qdse: f64,
        var_qdse_dn6: f64,
        var_qdse_dn7: f64,
        var_qdsub: f64,
        var_qdsub_dn6: f64,
        var_qdsub_dn7: f64,
        var_qdsub_dn8: f64,
        var_qg_wo_mult: f64,
        var_qg_wo_mult_dn4: f64,
        var_qg_wo_mult_dn6: f64,
        var_qg_wo_mult_dn7: f64,
        var_qg_wo_mult_dn8: f64,
        var_qg_wo_mult_dn9: f64,
        var_qgbe: f64,
        var_qgbe_dn4: f64,
        var_qgbe_dn6: f64,
        var_qgbe_dn7: f64,
        var_qgbe_dn8: f64,
        var_qgbe_dn9: f64,
        var_qgde: f64,
        var_qgde_dn4: f64,
        var_qgde_dn6: f64,
        var_qgde_dn7: f64,
        var_qgde_dn8: f64,
        var_qgde_dn9: f64,
        var_qgdif: f64,
        var_qgdif_dn4: f64,
        var_qgdif_dn6: f64,
        var_qgdif_dn7: f64,
        var_qgdif_dn8: f64,
        var_qgdif_dn9: f64,
        var_qgse: f64,
        var_qgse_dn4: f64,
        var_qgse_dn6: f64,
        var_qgse_dn7: f64,
        var_qgse_dn8: f64,
        var_qgse_dn9: f64,
        var_qgsif: f64,
        var_qgsif_dn4: f64,
        var_qgsif_dn6: f64,
        var_qgsif_dn7: f64,
        var_qgsif_dn8: f64,
        var_qgsif_dn9: f64,
        var_qovd: f64,
        var_qovd_dn4: f64,
        var_qovd_dn6: f64,
        var_qovd_dn7: f64,
        var_qovd_dn8: f64,
        var_qovd_dn9: f64,
        var_qovs: f64,
        var_qovs_dn4: f64,
        var_qovs_dn6: f64,
        var_qovs_dn7: f64,
        var_qovs_dn8: f64,
        var_qovs_dn9: f64,
        var_qssub: f64,
        var_qssub_dn6: f64,
        var_qssub_dn8: f64,
        var_qth: f64,
        var_qth_dn4: f64,
        var_qth_dn6: f64,
        var_qth_dn7: f64,
        var_qth_dn8: f64,
        var_qth_dn9: f64,
    ) {
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq0_e510, eq0_e510_d_n4, eq0_e510_d_n6, eq0_e510_d_n7, eq0_e510_d_n8, eq0_e510_d_n9,) = {
    if (var_guard1239 != 0.0) {
        let eq0_e508: f64 = (p.p14 * var_idse);
        let eq0_e508_d_n4: f64 = (p.p14 * var_idse_dn4);
        let eq0_e508_d_n6: f64 = (p.p14 * var_idse_dn6);
        let eq0_e508_d_n7: f64 = (p.p14 * var_idse_dn7);
        let eq0_e508_d_n8: f64 = (p.p14 * var_idse_dn8);
        let eq0_e508_d_n9: f64 = (p.p14 * var_idse_dn9);
        (eq0_e508, eq0_e508_d_n4, eq0_e508_d_n6, eq0_e508_d_n7, eq0_e508_d_n8, eq0_e508_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e510;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq0_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq0_e510_d_n4), multiplicity * (eq0_e510_d_n6), multiplicity * (eq0_e510_d_n7), multiplicity * (eq0_e510_d_n8), multiplicity * (eq0_e510_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq1_e517, eq1_e517_d_n4, eq1_e517_d_n6, eq1_e517_d_n7, eq1_e517_d_n8, eq1_e517_d_n9,) = {
    if (var_guard1239 == 0.0) {
        let eq1_e515: f64 = (p.p14 * var_idse);
        let eq1_e515_d_n4: f64 = (p.p14 * var_idse_dn4);
        let eq1_e515_d_n6: f64 = (p.p14 * var_idse_dn6);
        let eq1_e515_d_n7: f64 = (p.p14 * var_idse_dn7);
        let eq1_e515_d_n8: f64 = (p.p14 * var_idse_dn8);
        let eq1_e515_d_n9: f64 = (p.p14 * var_idse_dn9);
        (eq1_e515, eq1_e515_d_n4, eq1_e515_d_n6, eq1_e515_d_n7, eq1_e515_d_n8, eq1_e515_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e517;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq1_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq1_e517_d_n4), multiplicity * (eq1_e517_d_n6), multiplicity * (eq1_e517_d_n7), multiplicity * (eq1_e517_d_n8), multiplicity * (eq1_e517_d_n9)],
            [],
            [],
            1.0,
        );
        let eq2_e521: f64 = (var_igidle - var_igisle);
        let eq2_e521_d_n4: f64 = (var_igidle_dn4 - var_igisle_dn4);
        let eq2_e521_d_n6: f64 = (var_igidle_dn6 - var_igisle_dn6);
        let eq2_e521_d_n7: f64 = (var_igidle_dn7 - var_igisle_dn7);
        let eq2_e521_d_n8: f64 = (var_igidle_dn8 - var_igisle_dn8);
        let eq2_e521_d_n9: f64 = (var_igidle_dn9 - var_igisle_dn9);
        let eq2_e522: f64 = (p.p14 * eq2_e521);
        let eq2_e522_d_n4: f64 = (p.p14 * eq2_e521_d_n4);
        let eq2_e522_d_n6: f64 = (p.p14 * eq2_e521_d_n6);
        let eq2_e522_d_n7: f64 = (p.p14 * eq2_e521_d_n7);
        let eq2_e522_d_n8: f64 = (p.p14 * eq2_e521_d_n8);
        let eq2_e522_d_n9: f64 = (p.p14 * eq2_e521_d_n9);
        let eq2_value: f64 = eq2_e522;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq2_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq2_e522_d_n4), multiplicity * (eq2_e522_d_n6), multiplicity * (eq2_e522_d_n7), multiplicity * (eq2_e522_d_n8), multiplicity * (eq2_e522_d_n9)],
            [],
            [],
            1.0,
        );
        let eq3_e525: f64 = (p.p14 * var_igse);
        let eq3_e525_d_n4: f64 = (p.p14 * var_igse_dn4);
        let eq3_e525_d_n6: f64 = (p.p14 * var_igse_dn6);
        let eq3_e525_d_n7: f64 = (p.p14 * var_igse_dn7);
        let eq3_e525_d_n8: f64 = (p.p14 * var_igse_dn8);
        let eq3_e525_d_n9: f64 = (p.p14 * var_igse_dn9);
        let eq3_value: f64 = eq3_e525;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(6),
            multiplicity * (eq3_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq3_e525_d_n4), multiplicity * (eq3_e525_d_n6), multiplicity * (eq3_e525_d_n7), multiplicity * (eq3_e525_d_n8), multiplicity * (eq3_e525_d_n9)],
            [],
            [],
            1.0,
        );
        let eq4_e528: f64 = (p.p14 * var_igde);
        let eq4_e528_d_n4: f64 = (p.p14 * var_igde_dn4);
        let eq4_e528_d_n6: f64 = (p.p14 * var_igde_dn6);
        let eq4_e528_d_n7: f64 = (p.p14 * var_igde_dn7);
        let eq4_e528_d_n8: f64 = (p.p14 * var_igde_dn8);
        let eq4_e528_d_n9: f64 = (p.p14 * var_igde_dn9);
        let eq4_value: f64 = eq4_e528;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq4_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq4_e528_d_n4), multiplicity * (eq4_e528_d_n6), multiplicity * (eq4_e528_d_n7), multiplicity * (eq4_e528_d_n8), multiplicity * (eq4_e528_d_n9)],
            [],
            [],
            1.0,
        );
        let eq9_value: f64 = var_ithpwre;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            None,
            multiplicity * (eq9_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (var_ithpwre_dn4), multiplicity * (var_ithpwre_dn6), multiplicity * (var_ithpwre_dn7), multiplicity * (var_ithpwre_dn8), multiplicity * (var_ithpwre_dn9)],
            [],
            [],
            1.0,
        );
        let eq10_value: f64 = var_ithrce;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            None,
            multiplicity * (eq10_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (var_ithrce_dn4), multiplicity * (var_ithrce_dn6), multiplicity * (var_ithrce_dn7), multiplicity * (var_ithrce_dn8), multiplicity * (var_ithrce_dn9)],
            [],
            [],
            1.0,
        );
        let eq23_e642: f64 = (var_qg_wo_mult + var_qb_wo_mult);
        let eq23_e642_d_n4: f64 = (var_qg_wo_mult_dn4 + var_qb_wo_mult_dn4);
        let eq23_e642_d_n6: f64 = (var_qg_wo_mult_dn6 + var_qb_wo_mult_dn6);
        let eq23_e642_d_n7: f64 = (var_qg_wo_mult_dn7 + var_qb_wo_mult_dn7);
        let eq23_e642_d_n8: f64 = (var_qg_wo_mult_dn8 + var_qb_wo_mult_dn8);
        let eq23_e642_d_n9: f64 = (var_qg_wo_mult_dn9 + var_qb_wo_mult_dn9);
        let eq23_e643: f64 = (var_fracinv_i * eq23_e642);
        let eq23_e643_d_n4: f64 = (var_fracinv_i * eq23_e642_d_n4);
        let eq23_e643_d_n6: f64 = (var_fracinv_i * eq23_e642_d_n6);
        let eq23_e643_d_n7: f64 = (var_fracinv_i * eq23_e642_d_n7);
        let eq23_e643_d_n8: f64 = (var_fracinv_i * eq23_e642_d_n8);
        let eq23_e643_d_n9: f64 = (var_fracinv_i * eq23_e642_d_n9);
        let eq23_e644: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq23_e643);
        let eq23_value: f64 = eq23_e644;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(10),
            Some(13),
            multiplicity * (eq23_value),
            [4, 6, 7, 8, 9],
            [multiplicity * ((eq23_e643_d_n4 * ddt_scale)), multiplicity * ((eq23_e643_d_n6 * ddt_scale)), multiplicity * ((eq23_e643_d_n7 * ddt_scale)), multiplicity * ((eq23_e643_d_n8 * ddt_scale)), multiplicity * ((eq23_e643_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq24_e647: f64 = (var_itaueff * (nv10 - nv13));
        let eq24_e647_d_n4: f64 = (var_itaueff_dn4 * (nv10 - nv13));
        let eq24_e647_d_n6: f64 = (var_itaueff_dn6 * (nv10 - nv13));
        let eq24_e647_d_n7: f64 = (var_itaueff_dn7 * (nv10 - nv13));
        let eq24_e647_d_n8: f64 = (var_itaueff_dn8 * (nv10 - nv13));
        let eq24_e647_d_n9: f64 = (var_itaueff_dn9 * (nv10 - nv13));
        let eq24_value: f64 = eq24_e647;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(10),
            Some(13),
            multiplicity * (eq24_value),
            [4, 6, 7, 8, 9, 10, 13],
            [multiplicity * (eq24_e647_d_n4), multiplicity * (eq24_e647_d_n6), multiplicity * (eq24_e647_d_n7), multiplicity * (eq24_e647_d_n8), multiplicity * (eq24_e647_d_n9), multiplicity * (var_itaueff), multiplicity * ((-var_itaueff))],
            [],
            [],
            1.0,
        );
        let eq26_e653: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, var_qd_wo_mult);
        let eq26_value: f64 = eq26_e653;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(12),
            Some(13),
            multiplicity * (eq26_value),
            [4, 6, 7, 8, 9],
            [multiplicity * ((var_qd_wo_mult_dn4 * ddt_scale)), multiplicity * ((var_qd_wo_mult_dn6 * ddt_scale)), multiplicity * ((var_qd_wo_mult_dn7 * ddt_scale)), multiplicity * ((var_qd_wo_mult_dn8 * ddt_scale)), multiplicity * ((var_qd_wo_mult_dn9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq27_e656: f64 = (var_itaueff * (nv12 - nv13));
        let eq27_e656_d_n4: f64 = (var_itaueff_dn4 * (nv12 - nv13));
        let eq27_e656_d_n6: f64 = (var_itaueff_dn6 * (nv12 - nv13));
        let eq27_e656_d_n7: f64 = (var_itaueff_dn7 * (nv12 - nv13));
        let eq27_e656_d_n8: f64 = (var_itaueff_dn8 * (nv12 - nv13));
        let eq27_e656_d_n9: f64 = (var_itaueff_dn9 * (nv12 - nv13));
        let eq27_value: f64 = eq27_e656;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(12),
            Some(13),
            multiplicity * (eq27_value),
            [4, 6, 7, 8, 9, 12, 13],
            [multiplicity * (eq27_e656_d_n4), multiplicity * (eq27_e656_d_n6), multiplicity * (eq27_e656_d_n7), multiplicity * (eq27_e656_d_n8), multiplicity * (eq27_e656_d_n9), multiplicity * (var_itaueff), multiplicity * ((-var_itaueff))],
            [],
            [],
            1.0,
        );
        let eq29_e662: f64 = (var_kfracinv_i).sqrt();
        let eq29_e665: f64 = (1.0 - var_fracinv_i);
        let eq29_e668: f64 = (var_qg_wo_mult + var_qb_wo_mult);
        let eq29_e669: f64 = (eq29_e665 * eq29_e668);
        let eq29_e669_d_n4: f64 = (eq29_e665 * eq23_e642_d_n4);
        let eq29_e669_d_n6: f64 = (eq29_e665 * eq23_e642_d_n6);
        let eq29_e669_d_n7: f64 = (eq29_e665 * eq23_e642_d_n7);
        let eq29_e669_d_n8: f64 = (eq29_e665 * eq23_e642_d_n8);
        let eq29_e669_d_n9: f64 = (eq29_e665 * eq23_e642_d_n9);
        let eq29_e670: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq29_e669);
        let eq29_e671: f64 = (eq29_e662 * eq29_e670);
        let eq29_e671_d_n4: f64 = (eq29_e662 * (eq29_e669_d_n4 * ddt_scale));
        let eq29_e671_d_n6: f64 = (eq29_e662 * (eq29_e669_d_n6 * ddt_scale));
        let eq29_e671_d_n7: f64 = (eq29_e662 * (eq29_e669_d_n7 * ddt_scale));
        let eq29_e671_d_n8: f64 = (eq29_e662 * (eq29_e669_d_n8 * ddt_scale));
        let eq29_e671_d_n9: f64 = (eq29_e662 * (eq29_e669_d_n9 * ddt_scale));
        let eq29_value: f64 = eq29_e671;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(11),
            Some(13),
            multiplicity * (eq29_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq29_e671_d_n4), multiplicity * (eq29_e671_d_n6), multiplicity * (eq29_e671_d_n7), multiplicity * (eq29_e671_d_n8), multiplicity * (eq29_e671_d_n9)],
            [],
            [],
            1.0,
        );
        let eq30_e674: f64 = (var_itaueff * (nv11 - nv13));
        let eq30_e674_d_n4: f64 = (var_itaueff_dn4 * (nv11 - nv13));
        let eq30_e674_d_n6: f64 = (var_itaueff_dn6 * (nv11 - nv13));
        let eq30_e674_d_n7: f64 = (var_itaueff_dn7 * (nv11 - nv13));
        let eq30_e674_d_n8: f64 = (var_itaueff_dn8 * (nv11 - nv13));
        let eq30_e674_d_n9: f64 = (var_itaueff_dn9 * (nv11 - nv13));
        let eq30_value: f64 = eq30_e674;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(11),
            Some(13),
            multiplicity * (eq30_value),
            [4, 6, 7, 8, 9, 11, 13],
            [multiplicity * (eq30_e674_d_n4), multiplicity * (eq30_e674_d_n6), multiplicity * (eq30_e674_d_n7), multiplicity * (eq30_e674_d_n8), multiplicity * (eq30_e674_d_n9), multiplicity * (var_itaueff), multiplicity * ((-var_itaueff))],
            [],
            [],
            1.0,
        );
        let eq31_e678: f64 = (1e-9 * (nv11 - nv13));
        let eq31_e679: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq31_e678);
        let eq31_e680: f64 = (var_kfracinv_i * eq31_e679);
        let eq31_e680_d_n11: f64 = (var_kfracinv_i * (1e-9 * ddt_scale));
        let eq31_e680_d_n13: f64 = (var_kfracinv_i * ((-1e-9) * ddt_scale));
        let eq31_value: f64 = eq31_e680;
        stamper.stamp_current_node2_local(
            Some(11),
            Some(13),
            multiplicity * (eq31_value),
            11,
            multiplicity * (eq31_e680_d_n11),
            13,
            multiplicity * (eq31_e680_d_n13),
        );
        let eq32_e683: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, var_qb);
        let eq32_e685: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, var_qssub);
        let eq32_e686: f64 = (eq32_e683 + eq32_e685);
        let eq32_e686_d_n6: f64 = ((var_qb_dn6 * ddt_scale) + (var_qssub_dn6 * ddt_scale));
        let eq32_e686_d_n8: f64 = ((var_qb_dn8 * ddt_scale) + (var_qssub_dn8 * ddt_scale));
        let eq32_e688: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, var_qbsif);
        let eq32_e689: f64 = (eq32_e686 + eq32_e688);
        let eq32_e689_d_n4: f64 = ((var_qb_dn4 * ddt_scale) + (var_qbsif_dn4 * ddt_scale));
        let eq32_e689_d_n6: f64 = (eq32_e686_d_n6 + (var_qbsif_dn6 * ddt_scale));
        let eq32_e689_d_n7: f64 = ((var_qb_dn7 * ddt_scale) + (var_qbsif_dn7 * ddt_scale));
        let eq32_e689_d_n8: f64 = (eq32_e686_d_n8 + (var_qbsif_dn8 * ddt_scale));
        let eq32_e689_d_n9: f64 = ((var_qb_dn9 * ddt_scale) + (var_qbsif_dn9 * ddt_scale));
        let eq32_e690: f64 = (p.p14 * eq32_e689);
        let eq32_e690_d_n4: f64 = (p.p14 * eq32_e689_d_n4);
        let eq32_e690_d_n6: f64 = (p.p14 * eq32_e689_d_n6);
        let eq32_e690_d_n7: f64 = (p.p14 * eq32_e689_d_n7);
        let eq32_e690_d_n8: f64 = (p.p14 * eq32_e689_d_n8);
        let eq32_e690_d_n9: f64 = (p.p14 * eq32_e689_d_n9);
        let eq32_value: f64 = eq32_e690;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq32_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq32_e690_d_n4), multiplicity * (eq32_e690_d_n6), multiplicity * (eq32_e690_d_n7), multiplicity * (eq32_e690_d_n8), multiplicity * (eq32_e690_d_n9)],
            [],
            [],
            1.0,
        );
        let eq33_e693: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, var_qgde);
        let eq33_e695: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, var_qovd);
        let eq33_e696: f64 = (eq33_e693 + eq33_e695);
        let eq33_e696_d_n4: f64 = ((var_qgde_dn4 * ddt_scale) + (var_qovd_dn4 * ddt_scale));
        let eq33_e696_d_n6: f64 = ((var_qgde_dn6 * ddt_scale) + (var_qovd_dn6 * ddt_scale));
        let eq33_e696_d_n7: f64 = ((var_qgde_dn7 * ddt_scale) + (var_qovd_dn7 * ddt_scale));
        let eq33_e696_d_n8: f64 = ((var_qgde_dn8 * ddt_scale) + (var_qovd_dn8 * ddt_scale));
        let eq33_e696_d_n9: f64 = ((var_qgde_dn9 * ddt_scale) + (var_qovd_dn9 * ddt_scale));
        let eq33_e698: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, var_qgdif);
        let eq33_e699: f64 = (eq33_e696 + eq33_e698);
        let eq33_e699_d_n4: f64 = (eq33_e696_d_n4 + (var_qgdif_dn4 * ddt_scale));
        let eq33_e699_d_n6: f64 = (eq33_e696_d_n6 + (var_qgdif_dn6 * ddt_scale));
        let eq33_e699_d_n7: f64 = (eq33_e696_d_n7 + (var_qgdif_dn7 * ddt_scale));
        let eq33_e699_d_n8: f64 = (eq33_e696_d_n8 + (var_qgdif_dn8 * ddt_scale));
        let eq33_e699_d_n9: f64 = (eq33_e696_d_n9 + (var_qgdif_dn9 * ddt_scale));
        let eq33_e700: f64 = (p.p14 * eq33_e699);
        let eq33_e700_d_n4: f64 = (p.p14 * eq33_e699_d_n4);
        let eq33_e700_d_n6: f64 = (p.p14 * eq33_e699_d_n6);
        let eq33_e700_d_n7: f64 = (p.p14 * eq33_e699_d_n7);
        let eq33_e700_d_n8: f64 = (p.p14 * eq33_e699_d_n8);
        let eq33_e700_d_n9: f64 = (p.p14 * eq33_e699_d_n9);
        let eq33_value: f64 = eq33_e700;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq33_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq33_e700_d_n4), multiplicity * (eq33_e700_d_n6), multiplicity * (eq33_e700_d_n7), multiplicity * (eq33_e700_d_n8), multiplicity * (eq33_e700_d_n9)],
            [],
            [],
            1.0,
        );
        let eq34_e703: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, var_qdsub);
        let eq34_e705: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, var_qbdif);
        let eq34_e706: f64 = (eq34_e703 + eq34_e705);
        let eq34_e706_d_n6: f64 = ((var_qdsub_dn6 * ddt_scale) + (var_qbdif_dn6 * ddt_scale));
        let eq34_e706_d_n7: f64 = ((var_qdsub_dn7 * ddt_scale) + (var_qbdif_dn7 * ddt_scale));
        let eq34_e706_d_n8: f64 = ((var_qdsub_dn8 * ddt_scale) + (var_qbdif_dn8 * ddt_scale));
        let eq34_e707: f64 = (p.p14 * eq34_e706);
        let eq34_e707_d_n4: f64 = (p.p14 * (var_qbdif_dn4 * ddt_scale));
        let eq34_e707_d_n6: f64 = (p.p14 * eq34_e706_d_n6);
        let eq34_e707_d_n7: f64 = (p.p14 * eq34_e706_d_n7);
        let eq34_e707_d_n8: f64 = (p.p14 * eq34_e706_d_n8);
        let eq34_e707_d_n9: f64 = (p.p14 * (var_qbdif_dn9 * ddt_scale));
        let eq34_value: f64 = eq34_e707;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(7),
            multiplicity * (eq34_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq34_e707_d_n4), multiplicity * (eq34_e707_d_n6), multiplicity * (eq34_e707_d_n7), multiplicity * (eq34_e707_d_n8), multiplicity * (eq34_e707_d_n9)],
            [],
            [],
            1.0,
        );
        let eq35_e710: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, var_qgbe);
        let eq35_e711: f64 = (p.p14 * eq35_e710);
        let eq35_e711_d_n4: f64 = (p.p14 * (var_qgbe_dn4 * ddt_scale));
        let eq35_e711_d_n6: f64 = (p.p14 * (var_qgbe_dn6 * ddt_scale));
        let eq35_e711_d_n7: f64 = (p.p14 * (var_qgbe_dn7 * ddt_scale));
        let eq35_e711_d_n8: f64 = (p.p14 * (var_qgbe_dn8 * ddt_scale));
        let eq35_e711_d_n9: f64 = (p.p14 * (var_qgbe_dn9 * ddt_scale));
        let eq35_value: f64 = eq35_e711;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(8),
            multiplicity * (eq35_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq35_e711_d_n4), multiplicity * (eq35_e711_d_n6), multiplicity * (eq35_e711_d_n7), multiplicity * (eq35_e711_d_n8), multiplicity * (eq35_e711_d_n9)],
            [],
            [],
            1.0,
        );
        let eq36_e714: f64 = (-var_itaueff);
        let eq36_e716: f64 = (eq36_e714 * p.p32);
        let eq36_e716_d_n4: f64 = ((-var_itaueff_dn4) * p.p32);
        let eq36_e716_d_n6: f64 = ((-var_itaueff_dn6) * p.p32);
        let eq36_e716_d_n7: f64 = ((-var_itaueff_dn7) * p.p32);
        let eq36_e716_d_n8: f64 = ((-var_itaueff_dn8) * p.p32);
        let eq36_e716_d_n9: f64 = ((-var_itaueff_dn9) * p.p32);
        let eq36_e718: f64 = (eq36_e716 * var_mult_i_int);
        let eq36_e718_d_n4: f64 = (eq36_e716_d_n4 * var_mult_i_int);
        let eq36_e718_d_n6: f64 = (eq36_e716_d_n6 * var_mult_i_int);
        let eq36_e718_d_n7: f64 = (eq36_e716_d_n7 * var_mult_i_int);
        let eq36_e718_d_n8: f64 = (eq36_e716_d_n8 * var_mult_i_int);
        let eq36_e718_d_n9: f64 = (eq36_e716_d_n9 * var_mult_i_int);
        let eq36_e722: f64 = (var_kfracinv_i).sqrt();
        let __rspice_inv_cse_0: f64 = 1.0 / eq36_e722;
        let eq36_e723: f64 = ((nv11 - nv13) * __rspice_inv_cse_0);
        let eq36_e723_d_n11: f64 = (1.0 * __rspice_inv_cse_0);
        let eq36_e723_d_n13: f64 = ((-1.0) * __rspice_inv_cse_0);
        let eq36_e724: f64 = ((nv10 - nv13) + eq36_e723);
        let eq36_e724_d_n13: f64 = (-1.0 + eq36_e723_d_n13);
        let eq36_e725: f64 = (eq36_e718 * eq36_e724);
        let eq36_e725_d_n4: f64 = (eq36_e718_d_n4 * eq36_e724);
        let eq36_e725_d_n6: f64 = (eq36_e718_d_n6 * eq36_e724);
        let eq36_e725_d_n7: f64 = (eq36_e718_d_n7 * eq36_e724);
        let eq36_e725_d_n8: f64 = (eq36_e718_d_n8 * eq36_e724);
        let eq36_e725_d_n9: f64 = (eq36_e718_d_n9 * eq36_e724);
        let eq36_e725_d_n11: f64 = (eq36_e718 * eq36_e723_d_n11);
        let eq36_e725_d_n13: f64 = (eq36_e718 * eq36_e724_d_n13);
        let eq36_e727: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, var_qb);
        let eq36_e728: f64 = (eq36_e725 - eq36_e727);
        let eq36_e728_d_n4: f64 = (eq36_e725_d_n4 - (var_qb_dn4 * ddt_scale));
        let eq36_e728_d_n6: f64 = (eq36_e725_d_n6 - (var_qb_dn6 * ddt_scale));
        let eq36_e728_d_n7: f64 = (eq36_e725_d_n7 - (var_qb_dn7 * ddt_scale));
        let eq36_e728_d_n8: f64 = (eq36_e725_d_n8 - (var_qb_dn8 * ddt_scale));
        let eq36_e728_d_n9: f64 = (eq36_e725_d_n9 - (var_qb_dn9 * ddt_scale));
        let eq36_e730: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, var_qgse);
        let eq36_e731: f64 = (eq36_e728 + eq36_e730);
        let eq36_e731_d_n4: f64 = (eq36_e728_d_n4 + (var_qgse_dn4 * ddt_scale));
        let eq36_e731_d_n6: f64 = (eq36_e728_d_n6 + (var_qgse_dn6 * ddt_scale));
        let eq36_e731_d_n7: f64 = (eq36_e728_d_n7 + (var_qgse_dn7 * ddt_scale));
        let eq36_e731_d_n8: f64 = (eq36_e728_d_n8 + (var_qgse_dn8 * ddt_scale));
        let eq36_e731_d_n9: f64 = (eq36_e728_d_n9 + (var_qgse_dn9 * ddt_scale));
        let eq36_e733: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 17, var_qovs);
        let eq36_e734: f64 = (eq36_e731 + eq36_e733);
        let eq36_e734_d_n4: f64 = (eq36_e731_d_n4 + (var_qovs_dn4 * ddt_scale));
        let eq36_e734_d_n6: f64 = (eq36_e731_d_n6 + (var_qovs_dn6 * ddt_scale));
        let eq36_e734_d_n7: f64 = (eq36_e731_d_n7 + (var_qovs_dn7 * ddt_scale));
        let eq36_e734_d_n8: f64 = (eq36_e731_d_n8 + (var_qovs_dn8 * ddt_scale));
        let eq36_e734_d_n9: f64 = (eq36_e731_d_n9 + (var_qovs_dn9 * ddt_scale));
        let eq36_e736: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, var_qgsif);
        let eq36_e737: f64 = (eq36_e734 + eq36_e736);
        let eq36_e737_d_n4: f64 = (eq36_e734_d_n4 + (var_qgsif_dn4 * ddt_scale));
        let eq36_e737_d_n6: f64 = (eq36_e734_d_n6 + (var_qgsif_dn6 * ddt_scale));
        let eq36_e737_d_n7: f64 = (eq36_e734_d_n7 + (var_qgsif_dn7 * ddt_scale));
        let eq36_e737_d_n8: f64 = (eq36_e734_d_n8 + (var_qgsif_dn8 * ddt_scale));
        let eq36_e737_d_n9: f64 = (eq36_e734_d_n9 + (var_qgsif_dn9 * ddt_scale));
        let eq36_e738: f64 = (p.p14 * eq36_e737);
        let eq36_e738_d_n4: f64 = (p.p14 * eq36_e737_d_n4);
        let eq36_e738_d_n6: f64 = (p.p14 * eq36_e737_d_n6);
        let eq36_e738_d_n7: f64 = (p.p14 * eq36_e737_d_n7);
        let eq36_e738_d_n8: f64 = (p.p14 * eq36_e737_d_n8);
        let eq36_e738_d_n9: f64 = (p.p14 * eq36_e737_d_n9);
        let eq36_e738_d_n10: f64 = (p.p14 * eq36_e718);
        let eq36_e738_d_n11: f64 = (p.p14 * eq36_e725_d_n11);
        let eq36_e738_d_n13: f64 = (p.p14 * eq36_e725_d_n13);
        let eq36_value: f64 = eq36_e738;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(9),
            Some(6),
            multiplicity * (eq36_value),
            [4, 6, 7, 8, 9, 10, 11, 13],
            [multiplicity * (eq36_e738_d_n4), multiplicity * (eq36_e738_d_n6), multiplicity * (eq36_e738_d_n7), multiplicity * (eq36_e738_d_n8), multiplicity * (eq36_e738_d_n9), multiplicity * (eq36_e738_d_n10), multiplicity * (eq36_e738_d_n11), multiplicity * (eq36_e738_d_n13)],
            [],
            [],
            1.0,
        );
        let eq37_e741: f64 = (-var_itaueff);
        let eq37_e743: f64 = (eq37_e741 * p.p31);
        let eq37_e743_d_n4: f64 = ((-var_itaueff_dn4) * p.p31);
        let eq37_e743_d_n6: f64 = ((-var_itaueff_dn6) * p.p31);
        let eq37_e743_d_n7: f64 = ((-var_itaueff_dn7) * p.p31);
        let eq37_e743_d_n8: f64 = ((-var_itaueff_dn8) * p.p31);
        let eq37_e743_d_n9: f64 = ((-var_itaueff_dn9) * p.p31);
        let eq37_e745: f64 = (eq37_e743 * var_mult_i_int);
        let eq37_e745_d_n4: f64 = (eq37_e743_d_n4 * var_mult_i_int);
        let eq37_e745_d_n6: f64 = (eq37_e743_d_n6 * var_mult_i_int);
        let eq37_e745_d_n7: f64 = (eq37_e743_d_n7 * var_mult_i_int);
        let eq37_e745_d_n8: f64 = (eq37_e743_d_n8 * var_mult_i_int);
        let eq37_e745_d_n9: f64 = (eq37_e743_d_n9 * var_mult_i_int);
        let eq37_e747: f64 = (eq37_e745 * (nv12 - nv13));
        let eq37_e747_d_n4: f64 = (eq37_e745_d_n4 * (nv12 - nv13));
        let eq37_e747_d_n6: f64 = (eq37_e745_d_n6 * (nv12 - nv13));
        let eq37_e747_d_n7: f64 = (eq37_e745_d_n7 * (nv12 - nv13));
        let eq37_e747_d_n8: f64 = (eq37_e745_d_n8 * (nv12 - nv13));
        let eq37_e747_d_n9: f64 = (eq37_e745_d_n9 * (nv12 - nv13));
        let eq37_e749: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, var_qdse);
        let eq37_e750: f64 = (eq37_e747 + eq37_e749);
        let eq37_e750_d_n6: f64 = (eq37_e747_d_n6 + (var_qdse_dn6 * ddt_scale));
        let eq37_e750_d_n7: f64 = (eq37_e747_d_n7 + (var_qdse_dn7 * ddt_scale));
        let eq37_e751: f64 = (p.p14 * eq37_e750);
        let eq37_e751_d_n4: f64 = (p.p14 * eq37_e747_d_n4);
        let eq37_e751_d_n6: f64 = (p.p14 * eq37_e750_d_n6);
        let eq37_e751_d_n7: f64 = (p.p14 * eq37_e750_d_n7);
        let eq37_e751_d_n8: f64 = (p.p14 * eq37_e747_d_n8);
        let eq37_e751_d_n9: f64 = (p.p14 * eq37_e747_d_n9);
        let eq37_e751_d_n12: f64 = (p.p14 * eq37_e745);
        let eq37_e751_d_n13: f64 = (p.p14 * (-eq37_e745));
        let eq37_value: f64 = eq37_e751;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq37_value),
            [4, 6, 7, 8, 9, 12, 13],
            [multiplicity * (eq37_e751_d_n4), multiplicity * (eq37_e751_d_n6), multiplicity * (eq37_e751_d_n7), multiplicity * (eq37_e751_d_n8), multiplicity * (eq37_e751_d_n9), multiplicity * (eq37_e751_d_n12), multiplicity * (eq37_e751_d_n13)],
            [],
            [],
            1.0,
        );
        let eq38_e753: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 20, var_qth);
        let eq38_value: f64 = eq38_e753;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            None,
            multiplicity * (eq38_value),
            [4, 6, 7, 8, 9],
            [multiplicity * ((var_qth_dn4 * ddt_scale)), multiplicity * ((var_qth_dn6 * ddt_scale)), multiplicity * ((var_qth_dn7 * ddt_scale)), multiplicity * ((var_qth_dn8 * ddt_scale)), multiplicity * ((var_qth_dn9 * ddt_scale))],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        var_cdgeff: f64,
        var_cdgeff_dn4: f64,
        var_cdgeff_dn6: f64,
        var_cdgeff_dn7: f64,
        var_cdgeff_dn8: f64,
        var_cdgeff_dn9: f64,
        var_cgeff: f64,
        var_cgeff_dn4: f64,
        var_cgeff_dn6: f64,
        var_cgeff_dn7: f64,
        var_cgeff_dn8: f64,
        var_cgeff_dn9: f64,
        var_csgeff: f64,
        var_csgeff_dn4: f64,
        var_csgeff_dn6: f64,
        var_csgeff_dn7: f64,
        var_csgeff_dn8: f64,
        var_csgeff_dn9: f64,
        var_gsig: f64,
        var_gsig_dn4: f64,
        var_gsig_dn6: f64,
        var_gsig_dn7: f64,
        var_gsig_dn8: f64,
        var_gsig_dn9: f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq40_e759: f64 = (var_gsig * (nv5 - 0.0));
        let eq40_e759_d_n4: f64 = (var_gsig_dn4 * (nv5 - 0.0));
        let eq40_e759_d_n6: f64 = (var_gsig_dn6 * (nv5 - 0.0));
        let eq40_e759_d_n7: f64 = (var_gsig_dn7 * (nv5 - 0.0));
        let eq40_e759_d_n8: f64 = (var_gsig_dn8 * (nv5 - 0.0));
        let eq40_e759_d_n9: f64 = (var_gsig_dn9 * (nv5 - 0.0));
        let eq40_value: f64 = eq40_e759;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            None,
            multiplicity * (eq40_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * (eq40_e759_d_n4), multiplicity * (var_gsig), multiplicity * (eq40_e759_d_n6), multiplicity * (eq40_e759_d_n7), multiplicity * (eq40_e759_d_n8), multiplicity * (eq40_e759_d_n9)],
            [],
            [],
            1.0,
        );
        let eq41_e762: f64 = (var_cgeff * (nv5 - 0.0));
        let eq41_e762_d_n4: f64 = (var_cgeff_dn4 * (nv5 - 0.0));
        let eq41_e762_d_n6: f64 = (var_cgeff_dn6 * (nv5 - 0.0));
        let eq41_e762_d_n7: f64 = (var_cgeff_dn7 * (nv5 - 0.0));
        let eq41_e762_d_n8: f64 = (var_cgeff_dn8 * (nv5 - 0.0));
        let eq41_e762_d_n9: f64 = (var_cgeff_dn9 * (nv5 - 0.0));
        let eq41_e763: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 21, eq41_e762);
        let eq41_value: f64 = eq41_e763;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            None,
            multiplicity * (eq41_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq41_e762_d_n4 * ddt_scale)), multiplicity * ((var_cgeff * ddt_scale)), multiplicity * ((eq41_e762_d_n6 * ddt_scale)), multiplicity * ((eq41_e762_d_n7 * ddt_scale)), multiplicity * ((eq41_e762_d_n8 * ddt_scale)), multiplicity * ((eq41_e762_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq42_e765: f64 = (-var_csgeff);
        let eq42_e767: f64 = (eq42_e765 * (nv5 - 0.0));
        let eq42_e767_d_n4: f64 = ((-var_csgeff_dn4) * (nv5 - 0.0));
        let eq42_e767_d_n6: f64 = ((-var_csgeff_dn6) * (nv5 - 0.0));
        let eq42_e767_d_n7: f64 = ((-var_csgeff_dn7) * (nv5 - 0.0));
        let eq42_e767_d_n8: f64 = ((-var_csgeff_dn8) * (nv5 - 0.0));
        let eq42_e767_d_n9: f64 = ((-var_csgeff_dn9) * (nv5 - 0.0));
        let eq42_e768: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 22, eq42_e767);
        let eq42_value: f64 = eq42_e768;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            Some(6),
            multiplicity * (eq42_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq42_e767_d_n4 * ddt_scale)), multiplicity * ((eq42_e765 * ddt_scale)), multiplicity * ((eq42_e767_d_n6 * ddt_scale)), multiplicity * ((eq42_e767_d_n7 * ddt_scale)), multiplicity * ((eq42_e767_d_n8 * ddt_scale)), multiplicity * ((eq42_e767_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq43_e770: f64 = (-var_cdgeff);
        let eq43_e772: f64 = (eq43_e770 * (nv5 - 0.0));
        let eq43_e772_d_n4: f64 = ((-var_cdgeff_dn4) * (nv5 - 0.0));
        let eq43_e772_d_n6: f64 = ((-var_cdgeff_dn6) * (nv5 - 0.0));
        let eq43_e772_d_n7: f64 = ((-var_cdgeff_dn7) * (nv5 - 0.0));
        let eq43_e772_d_n8: f64 = ((-var_cdgeff_dn8) * (nv5 - 0.0));
        let eq43_e772_d_n9: f64 = ((-var_cdgeff_dn9) * (nv5 - 0.0));
        let eq43_e773: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 23, eq43_e772);
        let eq43_value: f64 = eq43_e773;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq43_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq43_e772_d_n4 * ddt_scale)), multiplicity * ((eq43_e770 * ddt_scale)), multiplicity * ((eq43_e772_d_n6 * ddt_scale)), multiplicity * ((eq43_e772_d_n7 * ddt_scale)), multiplicity * ((eq43_e772_d_n8 * ddt_scale)), multiplicity * ((eq43_e772_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let __rspice_deriv_cse_0: f64 = (s.dn[1774][0] + s.dn[1775][0]);
        let __rspice_deriv_cse_1: f64 = (s.dn[1774][1] + s.dn[1775][1]);
        let __rspice_deriv_cse_2: f64 = (s.dn[1774][2] + s.dn[1775][2]);
        let __rspice_deriv_cse_3: f64 = (s.dn[1774][3] + s.dn[1775][3]);
        let __rspice_deriv_cse_4: f64 = (s.dn[1774][4] + s.dn[1775][4]);
        let __rspice_deriv_cse_5: f64 = (s.dn[1774][5] + s.dn[1775][5]);
        let __rspice_deriv_cse_6: f64 = (s.dn[1774][6] + s.dn[1775][6]);
        let __rspice_deriv_cse_7: f64 = (s.dn[1774][7] + s.dn[1775][7]);
        let __rspice_deriv_cse_8: f64 = (s.dn[1774][8] + s.dn[1775][8]);
        let __rspice_deriv_cse_9: f64 = (s.dn[1774][9] + s.dn[1775][9]);
        let __rspice_deriv_cse_10: f64 = (s.dn[1774][10] + s.dn[1775][10]);
        let __rspice_deriv_cse_11: f64 = (s.dn[1774][11] + s.dn[1775][11]);
        let __rspice_deriv_cse_12: f64 = (s.dn[1774][12] + s.dn[1775][12]);
        let __rspice_deriv_cse_13: f64 = (s.dn[1774][13] + s.dn[1775][13]);
        let __rspice_deriv_cse_14: f64 = (s.db[1774][0] + s.db[1775][0]);
        let __rspice_deriv_cse_15: f64 = (s.db[1774][1] + s.db[1775][1]);
        let __rspice_deriv_cse_16: f64 = (s.db[1774][2] + s.db[1775][2]);
        let __rspice_deriv_cse_17: f64 = (s.db[1774][3] + s.db[1775][3]);
        let eq23_e642: f64 = (s.v[1774] + s.v[1775]);
        let eq23_e643: f64 = (s.v[181] * eq23_e642);
        let eq23_e643_d_n0: f64 = ((s.dn[181][0] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_0));
        let eq23_e643_d_n1: f64 = ((s.dn[181][1] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_1));
        let eq23_e643_d_n2: f64 = ((s.dn[181][2] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_2));
        let eq23_e643_d_n3: f64 = ((s.dn[181][3] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_3));
        let eq23_e643_d_n4: f64 = ((s.dn[181][4] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_4));
        let eq23_e643_d_n5: f64 = ((s.dn[181][5] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_5));
        let eq23_e643_d_n6: f64 = ((s.dn[181][6] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_6));
        let eq23_e643_d_n7: f64 = ((s.dn[181][7] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_7));
        let eq23_e643_d_n8: f64 = ((s.dn[181][8] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_8));
        let eq23_e643_d_n9: f64 = ((s.dn[181][9] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_9));
        let eq23_e643_d_n10: f64 = ((s.dn[181][10] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_10));
        let eq23_e643_d_n11: f64 = ((s.dn[181][11] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_11));
        let eq23_e643_d_n12: f64 = ((s.dn[181][12] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_12));
        let eq23_e643_d_n13: f64 = ((s.dn[181][13] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_13));
        let eq23_e643_d_b0: f64 = ((s.db[181][0] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_14));
        let eq23_e643_d_b1: f64 = ((s.db[181][1] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_15));
        let eq23_e643_d_b2: f64 = ((s.db[181][2] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_16));
        let eq23_e643_d_b3: f64 = ((s.db[181][3] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_17));
        let eq23_e644_q: f64 = eq23_e643;
        let eq23_reactive_node_derivatives: [f64; 14] = [eq23_e643_d_n0, eq23_e643_d_n1, eq23_e643_d_n2, eq23_e643_d_n3, eq23_e643_d_n4, eq23_e643_d_n5, eq23_e643_d_n6, eq23_e643_d_n7, eq23_e643_d_n8, eq23_e643_d_n9, eq23_e643_d_n10, eq23_e643_d_n11, eq23_e643_d_n12, eq23_e643_d_n13];
        let eq23_reactive_branch_derivatives: [f64; 4] = [eq23_e643_d_b0, eq23_e643_d_b1, eq23_e643_d_b2, eq23_e643_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[13]),
            nodes,
            &eq23_reactive_node_derivatives,
            branches,
            &eq23_reactive_branch_derivatives,
            multiplicity,
        );
        let eq26_e653_q: f64 = s.v[1776];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[13]),
            nodes,
            &s.dn[1776],
            branches,
            &s.db[1776],
            multiplicity,
        );
        let eq29_e662: f64 = (s.v[182]).sqrt();
        let __rspice_inv_cse_0: f64 = 1.0 / (2.0 * eq29_e662);
        let eq29_e662_d_n0: f64 = (s.dn[182][0] * __rspice_inv_cse_0);
        let eq29_e662_d_n1: f64 = (s.dn[182][1] * __rspice_inv_cse_0);
        let eq29_e662_d_n2: f64 = (s.dn[182][2] * __rspice_inv_cse_0);
        let eq29_e662_d_n3: f64 = (s.dn[182][3] * __rspice_inv_cse_0);
        let eq29_e662_d_n4: f64 = (s.dn[182][4] * __rspice_inv_cse_0);
        let eq29_e662_d_n5: f64 = (s.dn[182][5] * __rspice_inv_cse_0);
        let eq29_e662_d_n6: f64 = (s.dn[182][6] * __rspice_inv_cse_0);
        let eq29_e662_d_n7: f64 = (s.dn[182][7] * __rspice_inv_cse_0);
        let eq29_e662_d_n8: f64 = (s.dn[182][8] * __rspice_inv_cse_0);
        let eq29_e662_d_n9: f64 = (s.dn[182][9] * __rspice_inv_cse_0);
        let eq29_e662_d_n10: f64 = (s.dn[182][10] * __rspice_inv_cse_0);
        let eq29_e662_d_n11: f64 = (s.dn[182][11] * __rspice_inv_cse_0);
        let eq29_e662_d_n12: f64 = (s.dn[182][12] * __rspice_inv_cse_0);
        let eq29_e662_d_n13: f64 = (s.dn[182][13] * __rspice_inv_cse_0);
        let eq29_e662_d_b0: f64 = (s.db[182][0] * __rspice_inv_cse_0);
        let eq29_e662_d_b1: f64 = (s.db[182][1] * __rspice_inv_cse_0);
        let eq29_e662_d_b2: f64 = (s.db[182][2] * __rspice_inv_cse_0);
        let eq29_e662_d_b3: f64 = (s.db[182][3] * __rspice_inv_cse_0);
        let eq29_e665: f64 = (1.0 - s.v[181]);
        let eq29_e668: f64 = (s.v[1774] + s.v[1775]);
        let eq29_e669: f64 = (eq29_e665 * eq29_e668);
        let eq29_e669_d_n0: f64 = (((-s.dn[181][0]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_0));
        let eq29_e669_d_n1: f64 = (((-s.dn[181][1]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_1));
        let eq29_e669_d_n2: f64 = (((-s.dn[181][2]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_2));
        let eq29_e669_d_n3: f64 = (((-s.dn[181][3]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_3));
        let eq29_e669_d_n4: f64 = (((-s.dn[181][4]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_4));
        let eq29_e669_d_n5: f64 = (((-s.dn[181][5]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_5));
        let eq29_e669_d_n6: f64 = (((-s.dn[181][6]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_6));
        let eq29_e669_d_n7: f64 = (((-s.dn[181][7]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_7));
        let eq29_e669_d_n8: f64 = (((-s.dn[181][8]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_8));
        let eq29_e669_d_n9: f64 = (((-s.dn[181][9]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_9));
        let eq29_e669_d_n10: f64 = (((-s.dn[181][10]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_10));
        let eq29_e669_d_n11: f64 = (((-s.dn[181][11]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_11));
        let eq29_e669_d_n12: f64 = (((-s.dn[181][12]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_12));
        let eq29_e669_d_n13: f64 = (((-s.dn[181][13]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_13));
        let eq29_e669_d_b0: f64 = (((-s.db[181][0]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_14));
        let eq29_e669_d_b1: f64 = (((-s.db[181][1]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_15));
        let eq29_e669_d_b2: f64 = (((-s.db[181][2]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_16));
        let eq29_e669_d_b3: f64 = (((-s.db[181][3]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_17));
        let eq29_e670_q: f64 = eq29_e669;
        let eq29_e671: f64 = (eq29_e662 * eq29_e669);
        let eq29_e671_d_n0: f64 = ((eq29_e662_d_n0 * eq29_e669) + (eq29_e662 * eq29_e669_d_n0));
        let eq29_e671_d_n1: f64 = ((eq29_e662_d_n1 * eq29_e669) + (eq29_e662 * eq29_e669_d_n1));
        let eq29_e671_d_n2: f64 = ((eq29_e662_d_n2 * eq29_e669) + (eq29_e662 * eq29_e669_d_n2));
        let eq29_e671_d_n3: f64 = ((eq29_e662_d_n3 * eq29_e669) + (eq29_e662 * eq29_e669_d_n3));
        let eq29_e671_d_n4: f64 = ((eq29_e662_d_n4 * eq29_e669) + (eq29_e662 * eq29_e669_d_n4));
        let eq29_e671_d_n5: f64 = ((eq29_e662_d_n5 * eq29_e669) + (eq29_e662 * eq29_e669_d_n5));
        let eq29_e671_d_n6: f64 = ((eq29_e662_d_n6 * eq29_e669) + (eq29_e662 * eq29_e669_d_n6));
        let eq29_e671_d_n7: f64 = ((eq29_e662_d_n7 * eq29_e669) + (eq29_e662 * eq29_e669_d_n7));
        let eq29_e671_d_n8: f64 = ((eq29_e662_d_n8 * eq29_e669) + (eq29_e662 * eq29_e669_d_n8));
        let eq29_e671_d_n9: f64 = ((eq29_e662_d_n9 * eq29_e669) + (eq29_e662 * eq29_e669_d_n9));
        let eq29_e671_d_n10: f64 = ((eq29_e662_d_n10 * eq29_e669) + (eq29_e662 * eq29_e669_d_n10));
        let eq29_e671_d_n11: f64 = ((eq29_e662_d_n11 * eq29_e669) + (eq29_e662 * eq29_e669_d_n11));
        let eq29_e671_d_n12: f64 = ((eq29_e662_d_n12 * eq29_e669) + (eq29_e662 * eq29_e669_d_n12));
        let eq29_e671_d_n13: f64 = ((eq29_e662_d_n13 * eq29_e669) + (eq29_e662 * eq29_e669_d_n13));
        let eq29_e671_d_b0: f64 = ((eq29_e662_d_b0 * eq29_e669) + (eq29_e662 * eq29_e669_d_b0));
        let eq29_e671_d_b1: f64 = ((eq29_e662_d_b1 * eq29_e669) + (eq29_e662 * eq29_e669_d_b1));
        let eq29_e671_d_b2: f64 = ((eq29_e662_d_b2 * eq29_e669) + (eq29_e662 * eq29_e669_d_b2));
        let eq29_e671_d_b3: f64 = ((eq29_e662_d_b3 * eq29_e669) + (eq29_e662 * eq29_e669_d_b3));
        let eq29_e671_q: f64 = (eq29_e662 * eq29_e670_q);
        let eq29_e671_q_d_n0: f64 = ((eq29_e662_d_n0 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_n0));
        let eq29_e671_q_d_n1: f64 = ((eq29_e662_d_n1 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_n1));
        let eq29_e671_q_d_n2: f64 = ((eq29_e662_d_n2 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_n2));
        let eq29_e671_q_d_n3: f64 = ((eq29_e662_d_n3 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_n3));
        let eq29_e671_q_d_n4: f64 = ((eq29_e662_d_n4 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_n4));
        let eq29_e671_q_d_n5: f64 = ((eq29_e662_d_n5 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_n5));
        let eq29_e671_q_d_n6: f64 = ((eq29_e662_d_n6 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_n6));
        let eq29_e671_q_d_n7: f64 = ((eq29_e662_d_n7 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_n7));
        let eq29_e671_q_d_n8: f64 = ((eq29_e662_d_n8 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_n8));
        let eq29_e671_q_d_n9: f64 = ((eq29_e662_d_n9 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_n9));
        let eq29_e671_q_d_n10: f64 = ((eq29_e662_d_n10 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_n10));
        let eq29_e671_q_d_n11: f64 = ((eq29_e662_d_n11 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_n11));
        let eq29_e671_q_d_n12: f64 = ((eq29_e662_d_n12 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_n12));
        let eq29_e671_q_d_n13: f64 = ((eq29_e662_d_n13 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_n13));
        let eq29_e671_q_d_b0: f64 = ((eq29_e662_d_b0 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_b0));
        let eq29_e671_q_d_b1: f64 = ((eq29_e662_d_b1 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_b1));
        let eq29_e671_q_d_b2: f64 = ((eq29_e662_d_b2 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_b2));
        let eq29_e671_q_d_b3: f64 = ((eq29_e662_d_b3 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_b3));
        let eq29_reactive_node_derivatives: [f64; 14] = [eq29_e671_q_d_n0, eq29_e671_q_d_n1, eq29_e671_q_d_n2, eq29_e671_q_d_n3, eq29_e671_q_d_n4, eq29_e671_q_d_n5, eq29_e671_q_d_n6, eq29_e671_q_d_n7, eq29_e671_q_d_n8, eq29_e671_q_d_n9, eq29_e671_q_d_n10, eq29_e671_q_d_n11, eq29_e671_q_d_n12, eq29_e671_q_d_n13];
        let eq29_reactive_branch_derivatives: [f64; 4] = [eq29_e671_q_d_b0, eq29_e671_q_d_b1, eq29_e671_q_d_b2, eq29_e671_q_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[13]),
            nodes,
            &eq29_reactive_node_derivatives,
            branches,
            &eq29_reactive_branch_derivatives,
            multiplicity,
        );
        let eq31_e678: f64 = (1e-9 * (nv11 - nv13));
        let eq31_e679_q: f64 = eq31_e678;
        let eq31_e680: f64 = (s.v[182] * eq31_e678);
        let eq31_e680_d_n11: f64 = ((s.dn[182][11] * eq31_e678) + (s.v[182] * 1e-9));
        let eq31_e680_d_n13: f64 = ((s.dn[182][13] * eq31_e678) + (s.v[182] * (-1e-9)));
        let eq31_e680_q: f64 = (s.v[182] * eq31_e679_q);
        let eq31_e680_q_d_n11: f64 = ((s.dn[182][11] * eq31_e679_q) + (s.v[182] * 1e-9));
        let eq31_e680_q_d_n13: f64 = ((s.dn[182][13] * eq31_e679_q) + (s.v[182] * (-1e-9)));
        let eq31_reactive_node_derivatives: [f64; 14] = [(s.dn[182][0] * eq31_e679_q), (s.dn[182][1] * eq31_e679_q), (s.dn[182][2] * eq31_e679_q), (s.dn[182][3] * eq31_e679_q), (s.dn[182][4] * eq31_e679_q), (s.dn[182][5] * eq31_e679_q), (s.dn[182][6] * eq31_e679_q), (s.dn[182][7] * eq31_e679_q), (s.dn[182][8] * eq31_e679_q), (s.dn[182][9] * eq31_e679_q), (s.dn[182][10] * eq31_e679_q), eq31_e680_q_d_n11, (s.dn[182][12] * eq31_e679_q), eq31_e680_q_d_n13];
        let eq31_reactive_branch_derivatives: [f64; 4] = [(s.db[182][0] * eq31_e679_q), (s.db[182][1] * eq31_e679_q), (s.db[182][2] * eq31_e679_q), (s.db[182][3] * eq31_e679_q)];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[13]),
            nodes,
            &eq31_reactive_node_derivatives,
            branches,
            &eq31_reactive_branch_derivatives,
            multiplicity,
        );
        let eq32_e683_q: f64 = s.v[362];
        let eq32_e685_q: f64 = s.v[377];
        let eq32_e686: f64 = (s.v[362] + s.v[377]);
        let eq32_e686_d_n0: f64 = (s.dn[362][0] + s.dn[377][0]);
        let eq32_e686_d_n1: f64 = (s.dn[362][1] + s.dn[377][1]);
        let eq32_e686_d_n2: f64 = (s.dn[362][2] + s.dn[377][2]);
        let eq32_e686_d_n3: f64 = (s.dn[362][3] + s.dn[377][3]);
        let eq32_e686_d_n4: f64 = (s.dn[362][4] + s.dn[377][4]);
        let eq32_e686_d_n5: f64 = (s.dn[362][5] + s.dn[377][5]);
        let eq32_e686_d_n6: f64 = (s.dn[362][6] + s.dn[377][6]);
        let eq32_e686_d_n7: f64 = (s.dn[362][7] + s.dn[377][7]);
        let eq32_e686_d_n8: f64 = (s.dn[362][8] + s.dn[377][8]);
        let eq32_e686_d_n9: f64 = (s.dn[362][9] + s.dn[377][9]);
        let eq32_e686_d_n10: f64 = (s.dn[362][10] + s.dn[377][10]);
        let eq32_e686_d_n11: f64 = (s.dn[362][11] + s.dn[377][11]);
        let eq32_e686_d_n12: f64 = (s.dn[362][12] + s.dn[377][12]);
        let eq32_e686_d_n13: f64 = (s.dn[362][13] + s.dn[377][13]);
        let eq32_e686_d_b0: f64 = (s.db[362][0] + s.db[377][0]);
        let eq32_e686_d_b1: f64 = (s.db[362][1] + s.db[377][1]);
        let eq32_e686_d_b2: f64 = (s.db[362][2] + s.db[377][2]);
        let eq32_e686_d_b3: f64 = (s.db[362][3] + s.db[377][3]);
        let eq32_e686_q: f64 = (eq32_e683_q + eq32_e685_q);
        let eq32_e688_q: f64 = s.v[381];
        let eq32_e689: f64 = (eq32_e686 + s.v[381]);
        let eq32_e689_d_n0: f64 = (eq32_e686_d_n0 + s.dn[381][0]);
        let eq32_e689_d_n1: f64 = (eq32_e686_d_n1 + s.dn[381][1]);
        let eq32_e689_d_n2: f64 = (eq32_e686_d_n2 + s.dn[381][2]);
        let eq32_e689_d_n3: f64 = (eq32_e686_d_n3 + s.dn[381][3]);
        let eq32_e689_d_n4: f64 = (eq32_e686_d_n4 + s.dn[381][4]);
        let eq32_e689_d_n5: f64 = (eq32_e686_d_n5 + s.dn[381][5]);
        let eq32_e689_d_n6: f64 = (eq32_e686_d_n6 + s.dn[381][6]);
        let eq32_e689_d_n7: f64 = (eq32_e686_d_n7 + s.dn[381][7]);
        let eq32_e689_d_n8: f64 = (eq32_e686_d_n8 + s.dn[381][8]);
        let eq32_e689_d_n9: f64 = (eq32_e686_d_n9 + s.dn[381][9]);
        let eq32_e689_d_n10: f64 = (eq32_e686_d_n10 + s.dn[381][10]);
        let eq32_e689_d_n11: f64 = (eq32_e686_d_n11 + s.dn[381][11]);
        let eq32_e689_d_n12: f64 = (eq32_e686_d_n12 + s.dn[381][12]);
        let eq32_e689_d_n13: f64 = (eq32_e686_d_n13 + s.dn[381][13]);
        let eq32_e689_d_b0: f64 = (eq32_e686_d_b0 + s.db[381][0]);
        let eq32_e689_d_b1: f64 = (eq32_e686_d_b1 + s.db[381][1]);
        let eq32_e689_d_b2: f64 = (eq32_e686_d_b2 + s.db[381][2]);
        let eq32_e689_d_b3: f64 = (eq32_e686_d_b3 + s.db[381][3]);
        let eq32_e689_q: f64 = (eq32_e686_q + eq32_e688_q);
        let eq32_e690: f64 = (p.p14 * eq32_e689);
        let eq32_e690_d_n0: f64 = (p.p14 * eq32_e689_d_n0);
        let eq32_e690_d_n1: f64 = (p.p14 * eq32_e689_d_n1);
        let eq32_e690_d_n2: f64 = (p.p14 * eq32_e689_d_n2);
        let eq32_e690_d_n3: f64 = (p.p14 * eq32_e689_d_n3);
        let eq32_e690_d_n4: f64 = (p.p14 * eq32_e689_d_n4);
        let eq32_e690_d_n5: f64 = (p.p14 * eq32_e689_d_n5);
        let eq32_e690_d_n6: f64 = (p.p14 * eq32_e689_d_n6);
        let eq32_e690_d_n7: f64 = (p.p14 * eq32_e689_d_n7);
        let eq32_e690_d_n8: f64 = (p.p14 * eq32_e689_d_n8);
        let eq32_e690_d_n9: f64 = (p.p14 * eq32_e689_d_n9);
        let eq32_e690_d_n10: f64 = (p.p14 * eq32_e689_d_n10);
        let eq32_e690_d_n11: f64 = (p.p14 * eq32_e689_d_n11);
        let eq32_e690_d_n12: f64 = (p.p14 * eq32_e689_d_n12);
        let eq32_e690_d_n13: f64 = (p.p14 * eq32_e689_d_n13);
        let eq32_e690_d_b0: f64 = (p.p14 * eq32_e689_d_b0);
        let eq32_e690_d_b1: f64 = (p.p14 * eq32_e689_d_b1);
        let eq32_e690_d_b2: f64 = (p.p14 * eq32_e689_d_b2);
        let eq32_e690_d_b3: f64 = (p.p14 * eq32_e689_d_b3);
        let eq32_e690_q: f64 = (p.p14 * eq32_e689_q);
        let eq32_reactive_node_derivatives: [f64; 14] = [eq32_e690_d_n0, eq32_e690_d_n1, eq32_e690_d_n2, eq32_e690_d_n3, eq32_e690_d_n4, eq32_e690_d_n5, eq32_e690_d_n6, eq32_e690_d_n7, eq32_e690_d_n8, eq32_e690_d_n9, eq32_e690_d_n10, eq32_e690_d_n11, eq32_e690_d_n12, eq32_e690_d_n13];
        let eq32_reactive_branch_derivatives: [f64; 4] = [eq32_e690_d_b0, eq32_e690_d_b1, eq32_e690_d_b2, eq32_e690_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq32_reactive_node_derivatives,
            branches,
            &eq32_reactive_branch_derivatives,
            multiplicity,
        );
        let eq33_e693_q: f64 = s.v[371];
        let eq33_e695_q: f64 = s.v[373];
        let eq33_e696: f64 = (s.v[371] + s.v[373]);
        let eq33_e696_d_n0: f64 = (s.dn[371][0] + s.dn[373][0]);
        let eq33_e696_d_n1: f64 = (s.dn[371][1] + s.dn[373][1]);
        let eq33_e696_d_n2: f64 = (s.dn[371][2] + s.dn[373][2]);
        let eq33_e696_d_n3: f64 = (s.dn[371][3] + s.dn[373][3]);
        let eq33_e696_d_n4: f64 = (s.dn[371][4] + s.dn[373][4]);
        let eq33_e696_d_n5: f64 = (s.dn[371][5] + s.dn[373][5]);
        let eq33_e696_d_n6: f64 = (s.dn[371][6] + s.dn[373][6]);
        let eq33_e696_d_n7: f64 = (s.dn[371][7] + s.dn[373][7]);
        let eq33_e696_d_n8: f64 = (s.dn[371][8] + s.dn[373][8]);
        let eq33_e696_d_n9: f64 = (s.dn[371][9] + s.dn[373][9]);
        let eq33_e696_d_n10: f64 = (s.dn[371][10] + s.dn[373][10]);
        let eq33_e696_d_n11: f64 = (s.dn[371][11] + s.dn[373][11]);
        let eq33_e696_d_n12: f64 = (s.dn[371][12] + s.dn[373][12]);
        let eq33_e696_d_n13: f64 = (s.dn[371][13] + s.dn[373][13]);
        let eq33_e696_d_b0: f64 = (s.db[371][0] + s.db[373][0]);
        let eq33_e696_d_b1: f64 = (s.db[371][1] + s.db[373][1]);
        let eq33_e696_d_b2: f64 = (s.db[371][2] + s.db[373][2]);
        let eq33_e696_d_b3: f64 = (s.db[371][3] + s.db[373][3]);
        let eq33_e696_q: f64 = (eq33_e693_q + eq33_e695_q);
        let eq33_e698_q: f64 = s.v[380];
        let eq33_e699: f64 = (eq33_e696 + s.v[380]);
        let eq33_e699_d_n0: f64 = (eq33_e696_d_n0 + s.dn[380][0]);
        let eq33_e699_d_n1: f64 = (eq33_e696_d_n1 + s.dn[380][1]);
        let eq33_e699_d_n2: f64 = (eq33_e696_d_n2 + s.dn[380][2]);
        let eq33_e699_d_n3: f64 = (eq33_e696_d_n3 + s.dn[380][3]);
        let eq33_e699_d_n4: f64 = (eq33_e696_d_n4 + s.dn[380][4]);
        let eq33_e699_d_n5: f64 = (eq33_e696_d_n5 + s.dn[380][5]);
        let eq33_e699_d_n6: f64 = (eq33_e696_d_n6 + s.dn[380][6]);
        let eq33_e699_d_n7: f64 = (eq33_e696_d_n7 + s.dn[380][7]);
        let eq33_e699_d_n8: f64 = (eq33_e696_d_n8 + s.dn[380][8]);
        let eq33_e699_d_n9: f64 = (eq33_e696_d_n9 + s.dn[380][9]);
        let eq33_e699_d_n10: f64 = (eq33_e696_d_n10 + s.dn[380][10]);
        let eq33_e699_d_n11: f64 = (eq33_e696_d_n11 + s.dn[380][11]);
        let eq33_e699_d_n12: f64 = (eq33_e696_d_n12 + s.dn[380][12]);
        let eq33_e699_d_n13: f64 = (eq33_e696_d_n13 + s.dn[380][13]);
        let eq33_e699_d_b0: f64 = (eq33_e696_d_b0 + s.db[380][0]);
        let eq33_e699_d_b1: f64 = (eq33_e696_d_b1 + s.db[380][1]);
        let eq33_e699_d_b2: f64 = (eq33_e696_d_b2 + s.db[380][2]);
        let eq33_e699_d_b3: f64 = (eq33_e696_d_b3 + s.db[380][3]);
        let eq33_e699_q: f64 = (eq33_e696_q + eq33_e698_q);
        let eq33_e700: f64 = (p.p14 * eq33_e699);
        let eq33_e700_d_n0: f64 = (p.p14 * eq33_e699_d_n0);
        let eq33_e700_d_n1: f64 = (p.p14 * eq33_e699_d_n1);
        let eq33_e700_d_n2: f64 = (p.p14 * eq33_e699_d_n2);
        let eq33_e700_d_n3: f64 = (p.p14 * eq33_e699_d_n3);
        let eq33_e700_d_n4: f64 = (p.p14 * eq33_e699_d_n4);
        let eq33_e700_d_n5: f64 = (p.p14 * eq33_e699_d_n5);
        let eq33_e700_d_n6: f64 = (p.p14 * eq33_e699_d_n6);
        let eq33_e700_d_n7: f64 = (p.p14 * eq33_e699_d_n7);
        let eq33_e700_d_n8: f64 = (p.p14 * eq33_e699_d_n8);
        let eq33_e700_d_n9: f64 = (p.p14 * eq33_e699_d_n9);
        let eq33_e700_d_n10: f64 = (p.p14 * eq33_e699_d_n10);
        let eq33_e700_d_n11: f64 = (p.p14 * eq33_e699_d_n11);
        let eq33_e700_d_n12: f64 = (p.p14 * eq33_e699_d_n12);
        let eq33_e700_d_n13: f64 = (p.p14 * eq33_e699_d_n13);
        let eq33_e700_d_b0: f64 = (p.p14 * eq33_e699_d_b0);
        let eq33_e700_d_b1: f64 = (p.p14 * eq33_e699_d_b1);
        let eq33_e700_d_b2: f64 = (p.p14 * eq33_e699_d_b2);
        let eq33_e700_d_b3: f64 = (p.p14 * eq33_e699_d_b3);
        let eq33_e700_q: f64 = (p.p14 * eq33_e699_q);
        let eq33_reactive_node_derivatives: [f64; 14] = [eq33_e700_d_n0, eq33_e700_d_n1, eq33_e700_d_n2, eq33_e700_d_n3, eq33_e700_d_n4, eq33_e700_d_n5, eq33_e700_d_n6, eq33_e700_d_n7, eq33_e700_d_n8, eq33_e700_d_n9, eq33_e700_d_n10, eq33_e700_d_n11, eq33_e700_d_n12, eq33_e700_d_n13];
        let eq33_reactive_branch_derivatives: [f64; 4] = [eq33_e700_d_b0, eq33_e700_d_b1, eq33_e700_d_b2, eq33_e700_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq33_reactive_node_derivatives,
            branches,
            &eq33_reactive_branch_derivatives,
            multiplicity,
        );
        let eq34_e703_q: f64 = s.v[376];
        let eq34_e705_q: f64 = s.v[382];
        let eq34_e706: f64 = (s.v[376] + s.v[382]);
        let eq34_e706_d_n0: f64 = (s.dn[376][0] + s.dn[382][0]);
        let eq34_e706_d_n1: f64 = (s.dn[376][1] + s.dn[382][1]);
        let eq34_e706_d_n2: f64 = (s.dn[376][2] + s.dn[382][2]);
        let eq34_e706_d_n3: f64 = (s.dn[376][3] + s.dn[382][3]);
        let eq34_e706_d_n4: f64 = (s.dn[376][4] + s.dn[382][4]);
        let eq34_e706_d_n5: f64 = (s.dn[376][5] + s.dn[382][5]);
        let eq34_e706_d_n6: f64 = (s.dn[376][6] + s.dn[382][6]);
        let eq34_e706_d_n7: f64 = (s.dn[376][7] + s.dn[382][7]);
        let eq34_e706_d_n8: f64 = (s.dn[376][8] + s.dn[382][8]);
        let eq34_e706_d_n9: f64 = (s.dn[376][9] + s.dn[382][9]);
        let eq34_e706_d_n10: f64 = (s.dn[376][10] + s.dn[382][10]);
        let eq34_e706_d_n11: f64 = (s.dn[376][11] + s.dn[382][11]);
        let eq34_e706_d_n12: f64 = (s.dn[376][12] + s.dn[382][12]);
        let eq34_e706_d_n13: f64 = (s.dn[376][13] + s.dn[382][13]);
        let eq34_e706_d_b0: f64 = (s.db[376][0] + s.db[382][0]);
        let eq34_e706_d_b1: f64 = (s.db[376][1] + s.db[382][1]);
        let eq34_e706_d_b2: f64 = (s.db[376][2] + s.db[382][2]);
        let eq34_e706_d_b3: f64 = (s.db[376][3] + s.db[382][3]);
        let eq34_e706_q: f64 = (eq34_e703_q + eq34_e705_q);
        let eq34_e707: f64 = (p.p14 * eq34_e706);
        let eq34_e707_d_n0: f64 = (p.p14 * eq34_e706_d_n0);
        let eq34_e707_d_n1: f64 = (p.p14 * eq34_e706_d_n1);
        let eq34_e707_d_n2: f64 = (p.p14 * eq34_e706_d_n2);
        let eq34_e707_d_n3: f64 = (p.p14 * eq34_e706_d_n3);
        let eq34_e707_d_n4: f64 = (p.p14 * eq34_e706_d_n4);
        let eq34_e707_d_n5: f64 = (p.p14 * eq34_e706_d_n5);
        let eq34_e707_d_n6: f64 = (p.p14 * eq34_e706_d_n6);
        let eq34_e707_d_n7: f64 = (p.p14 * eq34_e706_d_n7);
        let eq34_e707_d_n8: f64 = (p.p14 * eq34_e706_d_n8);
        let eq34_e707_d_n9: f64 = (p.p14 * eq34_e706_d_n9);
        let eq34_e707_d_n10: f64 = (p.p14 * eq34_e706_d_n10);
        let eq34_e707_d_n11: f64 = (p.p14 * eq34_e706_d_n11);
        let eq34_e707_d_n12: f64 = (p.p14 * eq34_e706_d_n12);
        let eq34_e707_d_n13: f64 = (p.p14 * eq34_e706_d_n13);
        let eq34_e707_d_b0: f64 = (p.p14 * eq34_e706_d_b0);
        let eq34_e707_d_b1: f64 = (p.p14 * eq34_e706_d_b1);
        let eq34_e707_d_b2: f64 = (p.p14 * eq34_e706_d_b2);
        let eq34_e707_d_b3: f64 = (p.p14 * eq34_e706_d_b3);
        let eq34_e707_q: f64 = (p.p14 * eq34_e706_q);
        let eq34_reactive_node_derivatives: [f64; 14] = [eq34_e707_d_n0, eq34_e707_d_n1, eq34_e707_d_n2, eq34_e707_d_n3, eq34_e707_d_n4, eq34_e707_d_n5, eq34_e707_d_n6, eq34_e707_d_n7, eq34_e707_d_n8, eq34_e707_d_n9, eq34_e707_d_n10, eq34_e707_d_n11, eq34_e707_d_n12, eq34_e707_d_n13];
        let eq34_reactive_branch_derivatives: [f64; 4] = [eq34_e707_d_b0, eq34_e707_d_b1, eq34_e707_d_b2, eq34_e707_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq34_reactive_node_derivatives,
            branches,
            &eq34_reactive_branch_derivatives,
            multiplicity,
        );
        let eq35_e710_q: f64 = s.v[374];
        let eq35_e711: f64 = (p.p14 * s.v[374]);
        let eq35_e711_q: f64 = (p.p14 * eq35_e710_q);
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &s.dn[374],
            branches,
            &s.db[374],
            (multiplicity) * (p.p14),
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let eq36_e714: f64 = (-s.v[1773]);
        let eq36_e716: f64 = (eq36_e714 * p.p32);
        let eq36_e716_d_n0: f64 = ((-s.dn[1773][0]) * p.p32);
        let eq36_e716_d_n1: f64 = ((-s.dn[1773][1]) * p.p32);
        let eq36_e716_d_n2: f64 = ((-s.dn[1773][2]) * p.p32);
        let eq36_e716_d_n3: f64 = ((-s.dn[1773][3]) * p.p32);
        let eq36_e716_d_n4: f64 = ((-s.dn[1773][4]) * p.p32);
        let eq36_e716_d_n5: f64 = ((-s.dn[1773][5]) * p.p32);
        let eq36_e716_d_n6: f64 = ((-s.dn[1773][6]) * p.p32);
        let eq36_e716_d_n7: f64 = ((-s.dn[1773][7]) * p.p32);
        let eq36_e716_d_n8: f64 = ((-s.dn[1773][8]) * p.p32);
        let eq36_e716_d_n9: f64 = ((-s.dn[1773][9]) * p.p32);
        let eq36_e716_d_n10: f64 = ((-s.dn[1773][10]) * p.p32);
        let eq36_e716_d_n11: f64 = ((-s.dn[1773][11]) * p.p32);
        let eq36_e716_d_n12: f64 = ((-s.dn[1773][12]) * p.p32);
        let eq36_e716_d_n13: f64 = ((-s.dn[1773][13]) * p.p32);
        let eq36_e716_d_b0: f64 = ((-s.db[1773][0]) * p.p32);
        let eq36_e716_d_b1: f64 = ((-s.db[1773][1]) * p.p32);
        let eq36_e716_d_b2: f64 = ((-s.db[1773][2]) * p.p32);
        let eq36_e716_d_b3: f64 = ((-s.db[1773][3]) * p.p32);
        let eq36_e718: f64 = (eq36_e716 * s.v[13]);
        let eq36_e718_d_n0: f64 = ((eq36_e716_d_n0 * s.v[13]) + (eq36_e716 * s.dn[13][0]));
        let eq36_e718_d_n1: f64 = ((eq36_e716_d_n1 * s.v[13]) + (eq36_e716 * s.dn[13][1]));
        let eq36_e718_d_n2: f64 = ((eq36_e716_d_n2 * s.v[13]) + (eq36_e716 * s.dn[13][2]));
        let eq36_e718_d_n3: f64 = ((eq36_e716_d_n3 * s.v[13]) + (eq36_e716 * s.dn[13][3]));
        let eq36_e718_d_n4: f64 = ((eq36_e716_d_n4 * s.v[13]) + (eq36_e716 * s.dn[13][4]));
        let eq36_e718_d_n5: f64 = ((eq36_e716_d_n5 * s.v[13]) + (eq36_e716 * s.dn[13][5]));
        let eq36_e718_d_n6: f64 = ((eq36_e716_d_n6 * s.v[13]) + (eq36_e716 * s.dn[13][6]));
        let eq36_e718_d_n7: f64 = ((eq36_e716_d_n7 * s.v[13]) + (eq36_e716 * s.dn[13][7]));
        let eq36_e718_d_n8: f64 = ((eq36_e716_d_n8 * s.v[13]) + (eq36_e716 * s.dn[13][8]));
        let eq36_e718_d_n9: f64 = ((eq36_e716_d_n9 * s.v[13]) + (eq36_e716 * s.dn[13][9]));
        let eq36_e718_d_n10: f64 = ((eq36_e716_d_n10 * s.v[13]) + (eq36_e716 * s.dn[13][10]));
        let eq36_e718_d_n11: f64 = ((eq36_e716_d_n11 * s.v[13]) + (eq36_e716 * s.dn[13][11]));
        let eq36_e718_d_n12: f64 = ((eq36_e716_d_n12 * s.v[13]) + (eq36_e716 * s.dn[13][12]));
        let eq36_e718_d_n13: f64 = ((eq36_e716_d_n13 * s.v[13]) + (eq36_e716 * s.dn[13][13]));
        let eq36_e718_d_b0: f64 = ((eq36_e716_d_b0 * s.v[13]) + (eq36_e716 * s.db[13][0]));
        let eq36_e718_d_b1: f64 = ((eq36_e716_d_b1 * s.v[13]) + (eq36_e716 * s.db[13][1]));
        let eq36_e718_d_b2: f64 = ((eq36_e716_d_b2 * s.v[13]) + (eq36_e716 * s.db[13][2]));
        let eq36_e718_d_b3: f64 = ((eq36_e716_d_b3 * s.v[13]) + (eq36_e716 * s.db[13][3]));
        let eq36_e722: f64 = (s.v[182]).sqrt();
        let __rspice_inv_cse_0: f64 = 1.0 / (2.0 * eq36_e722);
        let eq36_e722_d_n0: f64 = (s.dn[182][0] * __rspice_inv_cse_0);
        let eq36_e722_d_n1: f64 = (s.dn[182][1] * __rspice_inv_cse_0);
        let eq36_e722_d_n2: f64 = (s.dn[182][2] * __rspice_inv_cse_0);
        let eq36_e722_d_n3: f64 = (s.dn[182][3] * __rspice_inv_cse_0);
        let eq36_e722_d_n4: f64 = (s.dn[182][4] * __rspice_inv_cse_0);
        let eq36_e722_d_n5: f64 = (s.dn[182][5] * __rspice_inv_cse_0);
        let eq36_e722_d_n6: f64 = (s.dn[182][6] * __rspice_inv_cse_0);
        let eq36_e722_d_n7: f64 = (s.dn[182][7] * __rspice_inv_cse_0);
        let eq36_e722_d_n8: f64 = (s.dn[182][8] * __rspice_inv_cse_0);
        let eq36_e722_d_n9: f64 = (s.dn[182][9] * __rspice_inv_cse_0);
        let eq36_e722_d_n10: f64 = (s.dn[182][10] * __rspice_inv_cse_0);
        let eq36_e722_d_n11: f64 = (s.dn[182][11] * __rspice_inv_cse_0);
        let eq36_e722_d_n12: f64 = (s.dn[182][12] * __rspice_inv_cse_0);
        let eq36_e722_d_n13: f64 = (s.dn[182][13] * __rspice_inv_cse_0);
        let eq36_e722_d_b0: f64 = (s.db[182][0] * __rspice_inv_cse_0);
        let eq36_e722_d_b1: f64 = (s.db[182][1] * __rspice_inv_cse_0);
        let eq36_e722_d_b2: f64 = (s.db[182][2] * __rspice_inv_cse_0);
        let eq36_e722_d_b3: f64 = (s.db[182][3] * __rspice_inv_cse_0);
        let eq36_e723: f64 = ((nv11 - nv13) / eq36_e722);
        let eq36_e723_d_n0: f64 = (-(((nv11 - nv13) * eq36_e722_d_n0) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n1: f64 = (-(((nv11 - nv13) * eq36_e722_d_n1) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n2: f64 = (-(((nv11 - nv13) * eq36_e722_d_n2) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n3: f64 = (-(((nv11 - nv13) * eq36_e722_d_n3) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n4: f64 = (-(((nv11 - nv13) * eq36_e722_d_n4) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n5: f64 = (-(((nv11 - nv13) * eq36_e722_d_n5) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n6: f64 = (-(((nv11 - nv13) * eq36_e722_d_n6) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n7: f64 = (-(((nv11 - nv13) * eq36_e722_d_n7) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n8: f64 = (-(((nv11 - nv13) * eq36_e722_d_n8) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n9: f64 = (-(((nv11 - nv13) * eq36_e722_d_n9) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n10: f64 = (-(((nv11 - nv13) * eq36_e722_d_n10) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n11: f64 = ((eq36_e722 - ((nv11 - nv13) * eq36_e722_d_n11)) / (eq36_e722 * eq36_e722));
        let eq36_e723_d_n12: f64 = (-(((nv11 - nv13) * eq36_e722_d_n12) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n13: f64 = (((-eq36_e722) - ((nv11 - nv13) * eq36_e722_d_n13)) / (eq36_e722 * eq36_e722));
        let eq36_e723_d_b0: f64 = (-(((nv11 - nv13) * eq36_e722_d_b0) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_b1: f64 = (-(((nv11 - nv13) * eq36_e722_d_b1) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_b2: f64 = (-(((nv11 - nv13) * eq36_e722_d_b2) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_b3: f64 = (-(((nv11 - nv13) * eq36_e722_d_b3) / (eq36_e722 * eq36_e722)));
        let eq36_e724: f64 = ((nv10 - nv13) + eq36_e723);
        let eq36_e724_d_n10: f64 = (1.0 + eq36_e723_d_n10);
        let eq36_e724_d_n13: f64 = (-1.0 + eq36_e723_d_n13);
        let eq36_e725: f64 = (eq36_e718 * eq36_e724);
        let eq36_e725_d_n0: f64 = ((eq36_e718_d_n0 * eq36_e724) + (eq36_e718 * eq36_e723_d_n0));
        let eq36_e725_d_n1: f64 = ((eq36_e718_d_n1 * eq36_e724) + (eq36_e718 * eq36_e723_d_n1));
        let eq36_e725_d_n2: f64 = ((eq36_e718_d_n2 * eq36_e724) + (eq36_e718 * eq36_e723_d_n2));
        let eq36_e725_d_n3: f64 = ((eq36_e718_d_n3 * eq36_e724) + (eq36_e718 * eq36_e723_d_n3));
        let eq36_e725_d_n4: f64 = ((eq36_e718_d_n4 * eq36_e724) + (eq36_e718 * eq36_e723_d_n4));
        let eq36_e725_d_n5: f64 = ((eq36_e718_d_n5 * eq36_e724) + (eq36_e718 * eq36_e723_d_n5));
        let eq36_e725_d_n6: f64 = ((eq36_e718_d_n6 * eq36_e724) + (eq36_e718 * eq36_e723_d_n6));
        let eq36_e725_d_n7: f64 = ((eq36_e718_d_n7 * eq36_e724) + (eq36_e718 * eq36_e723_d_n7));
        let eq36_e725_d_n8: f64 = ((eq36_e718_d_n8 * eq36_e724) + (eq36_e718 * eq36_e723_d_n8));
        let eq36_e725_d_n9: f64 = ((eq36_e718_d_n9 * eq36_e724) + (eq36_e718 * eq36_e723_d_n9));
        let eq36_e725_d_n10: f64 = ((eq36_e718_d_n10 * eq36_e724) + (eq36_e718 * eq36_e724_d_n10));
        let eq36_e725_d_n11: f64 = ((eq36_e718_d_n11 * eq36_e724) + (eq36_e718 * eq36_e723_d_n11));
        let eq36_e725_d_n12: f64 = ((eq36_e718_d_n12 * eq36_e724) + (eq36_e718 * eq36_e723_d_n12));
        let eq36_e725_d_n13: f64 = ((eq36_e718_d_n13 * eq36_e724) + (eq36_e718 * eq36_e724_d_n13));
        let eq36_e725_d_b0: f64 = ((eq36_e718_d_b0 * eq36_e724) + (eq36_e718 * eq36_e723_d_b0));
        let eq36_e725_d_b1: f64 = ((eq36_e718_d_b1 * eq36_e724) + (eq36_e718 * eq36_e723_d_b1));
        let eq36_e725_d_b2: f64 = ((eq36_e718_d_b2 * eq36_e724) + (eq36_e718 * eq36_e723_d_b2));
        let eq36_e725_d_b3: f64 = ((eq36_e718_d_b3 * eq36_e724) + (eq36_e718 * eq36_e723_d_b3));
        let eq36_e727_q: f64 = s.v[362];
        let eq36_e728: f64 = (eq36_e725 - s.v[362]);
        let eq36_e728_d_n0: f64 = (eq36_e725_d_n0 - s.dn[362][0]);
        let eq36_e728_d_n1: f64 = (eq36_e725_d_n1 - s.dn[362][1]);
        let eq36_e728_d_n2: f64 = (eq36_e725_d_n2 - s.dn[362][2]);
        let eq36_e728_d_n3: f64 = (eq36_e725_d_n3 - s.dn[362][3]);
        let eq36_e728_d_n4: f64 = (eq36_e725_d_n4 - s.dn[362][4]);
        let eq36_e728_d_n5: f64 = (eq36_e725_d_n5 - s.dn[362][5]);
        let eq36_e728_d_n6: f64 = (eq36_e725_d_n6 - s.dn[362][6]);
        let eq36_e728_d_n7: f64 = (eq36_e725_d_n7 - s.dn[362][7]);
        let eq36_e728_d_n8: f64 = (eq36_e725_d_n8 - s.dn[362][8]);
        let eq36_e728_d_n9: f64 = (eq36_e725_d_n9 - s.dn[362][9]);
        let eq36_e728_d_n10: f64 = (eq36_e725_d_n10 - s.dn[362][10]);
        let eq36_e728_d_n11: f64 = (eq36_e725_d_n11 - s.dn[362][11]);
        let eq36_e728_d_n12: f64 = (eq36_e725_d_n12 - s.dn[362][12]);
        let eq36_e728_d_n13: f64 = (eq36_e725_d_n13 - s.dn[362][13]);
        let eq36_e728_d_b0: f64 = (eq36_e725_d_b0 - s.db[362][0]);
        let eq36_e728_d_b1: f64 = (eq36_e725_d_b1 - s.db[362][1]);
        let eq36_e728_d_b2: f64 = (eq36_e725_d_b2 - s.db[362][2]);
        let eq36_e728_d_b3: f64 = (eq36_e725_d_b3 - s.db[362][3]);
        let eq36_e728_q: f64 = (-eq36_e727_q);
        let eq36_e730_q: f64 = s.v[370];
        let eq36_e731: f64 = (eq36_e728 + s.v[370]);
        let eq36_e731_d_n0: f64 = (eq36_e728_d_n0 + s.dn[370][0]);
        let eq36_e731_d_n1: f64 = (eq36_e728_d_n1 + s.dn[370][1]);
        let eq36_e731_d_n2: f64 = (eq36_e728_d_n2 + s.dn[370][2]);
        let eq36_e731_d_n3: f64 = (eq36_e728_d_n3 + s.dn[370][3]);
        let eq36_e731_d_n4: f64 = (eq36_e728_d_n4 + s.dn[370][4]);
        let eq36_e731_d_n5: f64 = (eq36_e728_d_n5 + s.dn[370][5]);
        let eq36_e731_d_n6: f64 = (eq36_e728_d_n6 + s.dn[370][6]);
        let eq36_e731_d_n7: f64 = (eq36_e728_d_n7 + s.dn[370][7]);
        let eq36_e731_d_n8: f64 = (eq36_e728_d_n8 + s.dn[370][8]);
        let eq36_e731_d_n9: f64 = (eq36_e728_d_n9 + s.dn[370][9]);
        let eq36_e731_d_n10: f64 = (eq36_e728_d_n10 + s.dn[370][10]);
        let eq36_e731_d_n11: f64 = (eq36_e728_d_n11 + s.dn[370][11]);
        let eq36_e731_d_n12: f64 = (eq36_e728_d_n12 + s.dn[370][12]);
        let eq36_e731_d_n13: f64 = (eq36_e728_d_n13 + s.dn[370][13]);
        let eq36_e731_d_b0: f64 = (eq36_e728_d_b0 + s.db[370][0]);
        let eq36_e731_d_b1: f64 = (eq36_e728_d_b1 + s.db[370][1]);
        let eq36_e731_d_b2: f64 = (eq36_e728_d_b2 + s.db[370][2]);
        let eq36_e731_d_b3: f64 = (eq36_e728_d_b3 + s.db[370][3]);
        let eq36_e731_q: f64 = (eq36_e728_q + eq36_e730_q);
        let eq36_e731_q_d_n0: f64 = ((-s.dn[362][0]) + s.dn[370][0]);
        let eq36_e731_q_d_n1: f64 = ((-s.dn[362][1]) + s.dn[370][1]);
        let eq36_e731_q_d_n2: f64 = ((-s.dn[362][2]) + s.dn[370][2]);
        let eq36_e731_q_d_n3: f64 = ((-s.dn[362][3]) + s.dn[370][3]);
        let eq36_e731_q_d_n4: f64 = ((-s.dn[362][4]) + s.dn[370][4]);
        let eq36_e731_q_d_n5: f64 = ((-s.dn[362][5]) + s.dn[370][5]);
        let eq36_e731_q_d_n6: f64 = ((-s.dn[362][6]) + s.dn[370][6]);
        let eq36_e731_q_d_n7: f64 = ((-s.dn[362][7]) + s.dn[370][7]);
        let eq36_e731_q_d_n8: f64 = ((-s.dn[362][8]) + s.dn[370][8]);
        let eq36_e731_q_d_n9: f64 = ((-s.dn[362][9]) + s.dn[370][9]);
        let eq36_e731_q_d_n10: f64 = ((-s.dn[362][10]) + s.dn[370][10]);
        let eq36_e731_q_d_n11: f64 = ((-s.dn[362][11]) + s.dn[370][11]);
        let eq36_e731_q_d_n12: f64 = ((-s.dn[362][12]) + s.dn[370][12]);
        let eq36_e731_q_d_n13: f64 = ((-s.dn[362][13]) + s.dn[370][13]);
        let eq36_e731_q_d_b0: f64 = ((-s.db[362][0]) + s.db[370][0]);
        let eq36_e731_q_d_b1: f64 = ((-s.db[362][1]) + s.db[370][1]);
        let eq36_e731_q_d_b2: f64 = ((-s.db[362][2]) + s.db[370][2]);
        let eq36_e731_q_d_b3: f64 = ((-s.db[362][3]) + s.db[370][3]);
        let eq36_e733_q: f64 = s.v[372];
        let eq36_e734: f64 = (eq36_e731 + s.v[372]);
        let eq36_e734_d_n0: f64 = (eq36_e731_d_n0 + s.dn[372][0]);
        let eq36_e734_d_n1: f64 = (eq36_e731_d_n1 + s.dn[372][1]);
        let eq36_e734_d_n2: f64 = (eq36_e731_d_n2 + s.dn[372][2]);
        let eq36_e734_d_n3: f64 = (eq36_e731_d_n3 + s.dn[372][3]);
        let eq36_e734_d_n4: f64 = (eq36_e731_d_n4 + s.dn[372][4]);
        let eq36_e734_d_n5: f64 = (eq36_e731_d_n5 + s.dn[372][5]);
        let eq36_e734_d_n6: f64 = (eq36_e731_d_n6 + s.dn[372][6]);
        let eq36_e734_d_n7: f64 = (eq36_e731_d_n7 + s.dn[372][7]);
        let eq36_e734_d_n8: f64 = (eq36_e731_d_n8 + s.dn[372][8]);
        let eq36_e734_d_n9: f64 = (eq36_e731_d_n9 + s.dn[372][9]);
        let eq36_e734_d_n10: f64 = (eq36_e731_d_n10 + s.dn[372][10]);
        let eq36_e734_d_n11: f64 = (eq36_e731_d_n11 + s.dn[372][11]);
        let eq36_e734_d_n12: f64 = (eq36_e731_d_n12 + s.dn[372][12]);
        let eq36_e734_d_n13: f64 = (eq36_e731_d_n13 + s.dn[372][13]);
        let eq36_e734_d_b0: f64 = (eq36_e731_d_b0 + s.db[372][0]);
        let eq36_e734_d_b1: f64 = (eq36_e731_d_b1 + s.db[372][1]);
        let eq36_e734_d_b2: f64 = (eq36_e731_d_b2 + s.db[372][2]);
        let eq36_e734_d_b3: f64 = (eq36_e731_d_b3 + s.db[372][3]);
        let eq36_e734_q: f64 = (eq36_e731_q + eq36_e733_q);
        let eq36_e734_q_d_n0: f64 = (eq36_e731_q_d_n0 + s.dn[372][0]);
        let eq36_e734_q_d_n1: f64 = (eq36_e731_q_d_n1 + s.dn[372][1]);
        let eq36_e734_q_d_n2: f64 = (eq36_e731_q_d_n2 + s.dn[372][2]);
        let eq36_e734_q_d_n3: f64 = (eq36_e731_q_d_n3 + s.dn[372][3]);
        let eq36_e734_q_d_n4: f64 = (eq36_e731_q_d_n4 + s.dn[372][4]);
        let eq36_e734_q_d_n5: f64 = (eq36_e731_q_d_n5 + s.dn[372][5]);
        let eq36_e734_q_d_n6: f64 = (eq36_e731_q_d_n6 + s.dn[372][6]);
        let eq36_e734_q_d_n7: f64 = (eq36_e731_q_d_n7 + s.dn[372][7]);
        let eq36_e734_q_d_n8: f64 = (eq36_e731_q_d_n8 + s.dn[372][8]);
        let eq36_e734_q_d_n9: f64 = (eq36_e731_q_d_n9 + s.dn[372][9]);
        let eq36_e734_q_d_n10: f64 = (eq36_e731_q_d_n10 + s.dn[372][10]);
        let eq36_e734_q_d_n11: f64 = (eq36_e731_q_d_n11 + s.dn[372][11]);
        let eq36_e734_q_d_n12: f64 = (eq36_e731_q_d_n12 + s.dn[372][12]);
        let eq36_e734_q_d_n13: f64 = (eq36_e731_q_d_n13 + s.dn[372][13]);
        let eq36_e734_q_d_b0: f64 = (eq36_e731_q_d_b0 + s.db[372][0]);
        let eq36_e734_q_d_b1: f64 = (eq36_e731_q_d_b1 + s.db[372][1]);
        let eq36_e734_q_d_b2: f64 = (eq36_e731_q_d_b2 + s.db[372][2]);
        let eq36_e734_q_d_b3: f64 = (eq36_e731_q_d_b3 + s.db[372][3]);
        let eq36_e736_q: f64 = s.v[379];
        let eq36_e737: f64 = (eq36_e734 + s.v[379]);
        let eq36_e737_d_n0: f64 = (eq36_e734_d_n0 + s.dn[379][0]);
        let eq36_e737_d_n1: f64 = (eq36_e734_d_n1 + s.dn[379][1]);
        let eq36_e737_d_n2: f64 = (eq36_e734_d_n2 + s.dn[379][2]);
        let eq36_e737_d_n3: f64 = (eq36_e734_d_n3 + s.dn[379][3]);
        let eq36_e737_d_n4: f64 = (eq36_e734_d_n4 + s.dn[379][4]);
        let eq36_e737_d_n5: f64 = (eq36_e734_d_n5 + s.dn[379][5]);
        let eq36_e737_d_n6: f64 = (eq36_e734_d_n6 + s.dn[379][6]);
        let eq36_e737_d_n7: f64 = (eq36_e734_d_n7 + s.dn[379][7]);
        let eq36_e737_d_n8: f64 = (eq36_e734_d_n8 + s.dn[379][8]);
        let eq36_e737_d_n9: f64 = (eq36_e734_d_n9 + s.dn[379][9]);
        let eq36_e737_d_n10: f64 = (eq36_e734_d_n10 + s.dn[379][10]);
        let eq36_e737_d_n11: f64 = (eq36_e734_d_n11 + s.dn[379][11]);
        let eq36_e737_d_n12: f64 = (eq36_e734_d_n12 + s.dn[379][12]);
        let eq36_e737_d_n13: f64 = (eq36_e734_d_n13 + s.dn[379][13]);
        let eq36_e737_d_b0: f64 = (eq36_e734_d_b0 + s.db[379][0]);
        let eq36_e737_d_b1: f64 = (eq36_e734_d_b1 + s.db[379][1]);
        let eq36_e737_d_b2: f64 = (eq36_e734_d_b2 + s.db[379][2]);
        let eq36_e737_d_b3: f64 = (eq36_e734_d_b3 + s.db[379][3]);
        let eq36_e737_q: f64 = (eq36_e734_q + eq36_e736_q);
        let eq36_e737_q_d_n0: f64 = (eq36_e734_q_d_n0 + s.dn[379][0]);
        let eq36_e737_q_d_n1: f64 = (eq36_e734_q_d_n1 + s.dn[379][1]);
        let eq36_e737_q_d_n2: f64 = (eq36_e734_q_d_n2 + s.dn[379][2]);
        let eq36_e737_q_d_n3: f64 = (eq36_e734_q_d_n3 + s.dn[379][3]);
        let eq36_e737_q_d_n4: f64 = (eq36_e734_q_d_n4 + s.dn[379][4]);
        let eq36_e737_q_d_n5: f64 = (eq36_e734_q_d_n5 + s.dn[379][5]);
        let eq36_e737_q_d_n6: f64 = (eq36_e734_q_d_n6 + s.dn[379][6]);
        let eq36_e737_q_d_n7: f64 = (eq36_e734_q_d_n7 + s.dn[379][7]);
        let eq36_e737_q_d_n8: f64 = (eq36_e734_q_d_n8 + s.dn[379][8]);
        let eq36_e737_q_d_n9: f64 = (eq36_e734_q_d_n9 + s.dn[379][9]);
        let eq36_e737_q_d_n10: f64 = (eq36_e734_q_d_n10 + s.dn[379][10]);
        let eq36_e737_q_d_n11: f64 = (eq36_e734_q_d_n11 + s.dn[379][11]);
        let eq36_e737_q_d_n12: f64 = (eq36_e734_q_d_n12 + s.dn[379][12]);
        let eq36_e737_q_d_n13: f64 = (eq36_e734_q_d_n13 + s.dn[379][13]);
        let eq36_e737_q_d_b0: f64 = (eq36_e734_q_d_b0 + s.db[379][0]);
        let eq36_e737_q_d_b1: f64 = (eq36_e734_q_d_b1 + s.db[379][1]);
        let eq36_e737_q_d_b2: f64 = (eq36_e734_q_d_b2 + s.db[379][2]);
        let eq36_e737_q_d_b3: f64 = (eq36_e734_q_d_b3 + s.db[379][3]);
        let eq36_e738: f64 = (p.p14 * eq36_e737);
        let eq36_e738_d_n0: f64 = (p.p14 * eq36_e737_d_n0);
        let eq36_e738_d_n1: f64 = (p.p14 * eq36_e737_d_n1);
        let eq36_e738_d_n2: f64 = (p.p14 * eq36_e737_d_n2);
        let eq36_e738_d_n3: f64 = (p.p14 * eq36_e737_d_n3);
        let eq36_e738_d_n4: f64 = (p.p14 * eq36_e737_d_n4);
        let eq36_e738_d_n5: f64 = (p.p14 * eq36_e737_d_n5);
        let eq36_e738_d_n6: f64 = (p.p14 * eq36_e737_d_n6);
        let eq36_e738_d_n7: f64 = (p.p14 * eq36_e737_d_n7);
        let eq36_e738_d_n8: f64 = (p.p14 * eq36_e737_d_n8);
        let eq36_e738_d_n9: f64 = (p.p14 * eq36_e737_d_n9);
        let eq36_e738_d_n10: f64 = (p.p14 * eq36_e737_d_n10);
        let eq36_e738_d_n11: f64 = (p.p14 * eq36_e737_d_n11);
        let eq36_e738_d_n12: f64 = (p.p14 * eq36_e737_d_n12);
        let eq36_e738_d_n13: f64 = (p.p14 * eq36_e737_d_n13);
        let eq36_e738_d_b0: f64 = (p.p14 * eq36_e737_d_b0);
        let eq36_e738_d_b1: f64 = (p.p14 * eq36_e737_d_b1);
        let eq36_e738_d_b2: f64 = (p.p14 * eq36_e737_d_b2);
        let eq36_e738_d_b3: f64 = (p.p14 * eq36_e737_d_b3);
        let eq36_e738_q: f64 = (p.p14 * eq36_e737_q);
        let eq36_e738_q_d_n0: f64 = (p.p14 * eq36_e737_q_d_n0);
        let eq36_e738_q_d_n1: f64 = (p.p14 * eq36_e737_q_d_n1);
        let eq36_e738_q_d_n2: f64 = (p.p14 * eq36_e737_q_d_n2);
        let eq36_e738_q_d_n3: f64 = (p.p14 * eq36_e737_q_d_n3);
        let eq36_e738_q_d_n4: f64 = (p.p14 * eq36_e737_q_d_n4);
        let eq36_e738_q_d_n5: f64 = (p.p14 * eq36_e737_q_d_n5);
        let eq36_e738_q_d_n6: f64 = (p.p14 * eq36_e737_q_d_n6);
        let eq36_e738_q_d_n7: f64 = (p.p14 * eq36_e737_q_d_n7);
        let eq36_e738_q_d_n8: f64 = (p.p14 * eq36_e737_q_d_n8);
        let eq36_e738_q_d_n9: f64 = (p.p14 * eq36_e737_q_d_n9);
        let eq36_e738_q_d_n10: f64 = (p.p14 * eq36_e737_q_d_n10);
        let eq36_e738_q_d_n11: f64 = (p.p14 * eq36_e737_q_d_n11);
        let eq36_e738_q_d_n12: f64 = (p.p14 * eq36_e737_q_d_n12);
        let eq36_e738_q_d_n13: f64 = (p.p14 * eq36_e737_q_d_n13);
        let eq36_e738_q_d_b0: f64 = (p.p14 * eq36_e737_q_d_b0);
        let eq36_e738_q_d_b1: f64 = (p.p14 * eq36_e737_q_d_b1);
        let eq36_e738_q_d_b2: f64 = (p.p14 * eq36_e737_q_d_b2);
        let eq36_e738_q_d_b3: f64 = (p.p14 * eq36_e737_q_d_b3);
        let eq36_reactive_node_derivatives: [f64; 14] = [eq36_e738_q_d_n0, eq36_e738_q_d_n1, eq36_e738_q_d_n2, eq36_e738_q_d_n3, eq36_e738_q_d_n4, eq36_e738_q_d_n5, eq36_e738_q_d_n6, eq36_e738_q_d_n7, eq36_e738_q_d_n8, eq36_e738_q_d_n9, eq36_e738_q_d_n10, eq36_e738_q_d_n11, eq36_e738_q_d_n12, eq36_e738_q_d_n13];
        let eq36_reactive_branch_derivatives: [f64; 4] = [eq36_e738_q_d_b0, eq36_e738_q_d_b1, eq36_e738_q_d_b2, eq36_e738_q_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            nodes,
            &eq36_reactive_node_derivatives,
            branches,
            &eq36_reactive_branch_derivatives,
            multiplicity,
        );
        let eq37_e741: f64 = (-s.v[1773]);
        let eq37_e743: f64 = (eq37_e741 * p.p31);
        let eq37_e743_d_n0: f64 = ((-s.dn[1773][0]) * p.p31);
        let eq37_e743_d_n1: f64 = ((-s.dn[1773][1]) * p.p31);
        let eq37_e743_d_n2: f64 = ((-s.dn[1773][2]) * p.p31);
        let eq37_e743_d_n3: f64 = ((-s.dn[1773][3]) * p.p31);
        let eq37_e743_d_n4: f64 = ((-s.dn[1773][4]) * p.p31);
        let eq37_e743_d_n5: f64 = ((-s.dn[1773][5]) * p.p31);
        let eq37_e743_d_n6: f64 = ((-s.dn[1773][6]) * p.p31);
        let eq37_e743_d_n7: f64 = ((-s.dn[1773][7]) * p.p31);
        let eq37_e743_d_n8: f64 = ((-s.dn[1773][8]) * p.p31);
        let eq37_e743_d_n9: f64 = ((-s.dn[1773][9]) * p.p31);
        let eq37_e743_d_n10: f64 = ((-s.dn[1773][10]) * p.p31);
        let eq37_e743_d_n11: f64 = ((-s.dn[1773][11]) * p.p31);
        let eq37_e743_d_n12: f64 = ((-s.dn[1773][12]) * p.p31);
        let eq37_e743_d_n13: f64 = ((-s.dn[1773][13]) * p.p31);
        let eq37_e743_d_b0: f64 = ((-s.db[1773][0]) * p.p31);
        let eq37_e743_d_b1: f64 = ((-s.db[1773][1]) * p.p31);
        let eq37_e743_d_b2: f64 = ((-s.db[1773][2]) * p.p31);
        let eq37_e743_d_b3: f64 = ((-s.db[1773][3]) * p.p31);
        let eq37_e745: f64 = (eq37_e743 * s.v[13]);
        let eq37_e745_d_n0: f64 = ((eq37_e743_d_n0 * s.v[13]) + (eq37_e743 * s.dn[13][0]));
        let eq37_e745_d_n1: f64 = ((eq37_e743_d_n1 * s.v[13]) + (eq37_e743 * s.dn[13][1]));
        let eq37_e745_d_n2: f64 = ((eq37_e743_d_n2 * s.v[13]) + (eq37_e743 * s.dn[13][2]));
        let eq37_e745_d_n3: f64 = ((eq37_e743_d_n3 * s.v[13]) + (eq37_e743 * s.dn[13][3]));
        let eq37_e745_d_n4: f64 = ((eq37_e743_d_n4 * s.v[13]) + (eq37_e743 * s.dn[13][4]));
        let eq37_e745_d_n5: f64 = ((eq37_e743_d_n5 * s.v[13]) + (eq37_e743 * s.dn[13][5]));
        let eq37_e745_d_n6: f64 = ((eq37_e743_d_n6 * s.v[13]) + (eq37_e743 * s.dn[13][6]));
        let eq37_e745_d_n7: f64 = ((eq37_e743_d_n7 * s.v[13]) + (eq37_e743 * s.dn[13][7]));
        let eq37_e745_d_n8: f64 = ((eq37_e743_d_n8 * s.v[13]) + (eq37_e743 * s.dn[13][8]));
        let eq37_e745_d_n9: f64 = ((eq37_e743_d_n9 * s.v[13]) + (eq37_e743 * s.dn[13][9]));
        let eq37_e745_d_n10: f64 = ((eq37_e743_d_n10 * s.v[13]) + (eq37_e743 * s.dn[13][10]));
        let eq37_e745_d_n11: f64 = ((eq37_e743_d_n11 * s.v[13]) + (eq37_e743 * s.dn[13][11]));
        let eq37_e745_d_n12: f64 = ((eq37_e743_d_n12 * s.v[13]) + (eq37_e743 * s.dn[13][12]));
        let eq37_e745_d_n13: f64 = ((eq37_e743_d_n13 * s.v[13]) + (eq37_e743 * s.dn[13][13]));
        let eq37_e745_d_b0: f64 = ((eq37_e743_d_b0 * s.v[13]) + (eq37_e743 * s.db[13][0]));
        let eq37_e745_d_b1: f64 = ((eq37_e743_d_b1 * s.v[13]) + (eq37_e743 * s.db[13][1]));
        let eq37_e745_d_b2: f64 = ((eq37_e743_d_b2 * s.v[13]) + (eq37_e743 * s.db[13][2]));
        let eq37_e745_d_b3: f64 = ((eq37_e743_d_b3 * s.v[13]) + (eq37_e743 * s.db[13][3]));
        let eq37_e747: f64 = (eq37_e745 * (nv12 - nv13));
        let eq37_e747_d_n0: f64 = (eq37_e745_d_n0 * (nv12 - nv13));
        let eq37_e747_d_n1: f64 = (eq37_e745_d_n1 * (nv12 - nv13));
        let eq37_e747_d_n2: f64 = (eq37_e745_d_n2 * (nv12 - nv13));
        let eq37_e747_d_n3: f64 = (eq37_e745_d_n3 * (nv12 - nv13));
        let eq37_e747_d_n4: f64 = (eq37_e745_d_n4 * (nv12 - nv13));
        let eq37_e747_d_n5: f64 = (eq37_e745_d_n5 * (nv12 - nv13));
        let eq37_e747_d_n6: f64 = (eq37_e745_d_n6 * (nv12 - nv13));
        let eq37_e747_d_n7: f64 = (eq37_e745_d_n7 * (nv12 - nv13));
        let eq37_e747_d_n8: f64 = (eq37_e745_d_n8 * (nv12 - nv13));
        let eq37_e747_d_n9: f64 = (eq37_e745_d_n9 * (nv12 - nv13));
        let eq37_e747_d_n10: f64 = (eq37_e745_d_n10 * (nv12 - nv13));
        let eq37_e747_d_n11: f64 = (eq37_e745_d_n11 * (nv12 - nv13));
        let eq37_e747_d_n12: f64 = ((eq37_e745_d_n12 * (nv12 - nv13)) + eq37_e745);
        let eq37_e747_d_n13: f64 = ((eq37_e745_d_n13 * (nv12 - nv13)) + (-eq37_e745));
        let eq37_e747_d_b0: f64 = (eq37_e745_d_b0 * (nv12 - nv13));
        let eq37_e747_d_b1: f64 = (eq37_e745_d_b1 * (nv12 - nv13));
        let eq37_e747_d_b2: f64 = (eq37_e745_d_b2 * (nv12 - nv13));
        let eq37_e747_d_b3: f64 = (eq37_e745_d_b3 * (nv12 - nv13));
        let eq37_e749_q: f64 = s.v[375];
        let eq37_e750: f64 = (eq37_e747 + s.v[375]);
        let eq37_e750_d_n0: f64 = (eq37_e747_d_n0 + s.dn[375][0]);
        let eq37_e750_d_n1: f64 = (eq37_e747_d_n1 + s.dn[375][1]);
        let eq37_e750_d_n2: f64 = (eq37_e747_d_n2 + s.dn[375][2]);
        let eq37_e750_d_n3: f64 = (eq37_e747_d_n3 + s.dn[375][3]);
        let eq37_e750_d_n4: f64 = (eq37_e747_d_n4 + s.dn[375][4]);
        let eq37_e750_d_n5: f64 = (eq37_e747_d_n5 + s.dn[375][5]);
        let eq37_e750_d_n6: f64 = (eq37_e747_d_n6 + s.dn[375][6]);
        let eq37_e750_d_n7: f64 = (eq37_e747_d_n7 + s.dn[375][7]);
        let eq37_e750_d_n8: f64 = (eq37_e747_d_n8 + s.dn[375][8]);
        let eq37_e750_d_n9: f64 = (eq37_e747_d_n9 + s.dn[375][9]);
        let eq37_e750_d_n10: f64 = (eq37_e747_d_n10 + s.dn[375][10]);
        let eq37_e750_d_n11: f64 = (eq37_e747_d_n11 + s.dn[375][11]);
        let eq37_e750_d_n12: f64 = (eq37_e747_d_n12 + s.dn[375][12]);
        let eq37_e750_d_n13: f64 = (eq37_e747_d_n13 + s.dn[375][13]);
        let eq37_e750_d_b0: f64 = (eq37_e747_d_b0 + s.db[375][0]);
        let eq37_e750_d_b1: f64 = (eq37_e747_d_b1 + s.db[375][1]);
        let eq37_e750_d_b2: f64 = (eq37_e747_d_b2 + s.db[375][2]);
        let eq37_e750_d_b3: f64 = (eq37_e747_d_b3 + s.db[375][3]);
        let eq37_e750_q: f64 = eq37_e749_q;
        let eq37_e751: f64 = (p.p14 * eq37_e750);
        let eq37_e751_d_n0: f64 = (p.p14 * eq37_e750_d_n0);
        let eq37_e751_d_n1: f64 = (p.p14 * eq37_e750_d_n1);
        let eq37_e751_d_n2: f64 = (p.p14 * eq37_e750_d_n2);
        let eq37_e751_d_n3: f64 = (p.p14 * eq37_e750_d_n3);
        let eq37_e751_d_n4: f64 = (p.p14 * eq37_e750_d_n4);
        let eq37_e751_d_n5: f64 = (p.p14 * eq37_e750_d_n5);
        let eq37_e751_d_n6: f64 = (p.p14 * eq37_e750_d_n6);
        let eq37_e751_d_n7: f64 = (p.p14 * eq37_e750_d_n7);
        let eq37_e751_d_n8: f64 = (p.p14 * eq37_e750_d_n8);
        let eq37_e751_d_n9: f64 = (p.p14 * eq37_e750_d_n9);
        let eq37_e751_d_n10: f64 = (p.p14 * eq37_e750_d_n10);
        let eq37_e751_d_n11: f64 = (p.p14 * eq37_e750_d_n11);
        let eq37_e751_d_n12: f64 = (p.p14 * eq37_e750_d_n12);
        let eq37_e751_d_n13: f64 = (p.p14 * eq37_e750_d_n13);
        let eq37_e751_d_b0: f64 = (p.p14 * eq37_e750_d_b0);
        let eq37_e751_d_b1: f64 = (p.p14 * eq37_e750_d_b1);
        let eq37_e751_d_b2: f64 = (p.p14 * eq37_e750_d_b2);
        let eq37_e751_d_b3: f64 = (p.p14 * eq37_e750_d_b3);
        let eq37_e751_q: f64 = (p.p14 * eq37_e750_q);
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            nodes,
            &s.dn[375],
            branches,
            &s.db[375],
            (multiplicity) * (p.p14),
        );
        let eq38_e753_q: f64 = s.v[378];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &s.dn[378],
            branches,
            &s.db[378],
            multiplicity,
        );
        let eq41_e762: f64 = (s.v[1800] * (nv5 - 0.0));
        let eq41_e762_d_n0: f64 = (s.dn[1800][0] * (nv5 - 0.0));
        let eq41_e762_d_n1: f64 = (s.dn[1800][1] * (nv5 - 0.0));
        let eq41_e762_d_n2: f64 = (s.dn[1800][2] * (nv5 - 0.0));
        let eq41_e762_d_n3: f64 = (s.dn[1800][3] * (nv5 - 0.0));
        let eq41_e762_d_n4: f64 = (s.dn[1800][4] * (nv5 - 0.0));
        let eq41_e762_d_n5: f64 = ((s.dn[1800][5] * (nv5 - 0.0)) + s.v[1800]);
        let eq41_e762_d_n6: f64 = (s.dn[1800][6] * (nv5 - 0.0));
        let eq41_e762_d_n7: f64 = (s.dn[1800][7] * (nv5 - 0.0));
        let eq41_e762_d_n8: f64 = (s.dn[1800][8] * (nv5 - 0.0));
        let eq41_e762_d_n9: f64 = (s.dn[1800][9] * (nv5 - 0.0));
        let eq41_e762_d_n10: f64 = (s.dn[1800][10] * (nv5 - 0.0));
        let eq41_e762_d_n11: f64 = (s.dn[1800][11] * (nv5 - 0.0));
        let eq41_e762_d_n12: f64 = (s.dn[1800][12] * (nv5 - 0.0));
        let eq41_e762_d_n13: f64 = (s.dn[1800][13] * (nv5 - 0.0));
        let eq41_e762_d_b0: f64 = (s.db[1800][0] * (nv5 - 0.0));
        let eq41_e762_d_b1: f64 = (s.db[1800][1] * (nv5 - 0.0));
        let eq41_e762_d_b2: f64 = (s.db[1800][2] * (nv5 - 0.0));
        let eq41_e762_d_b3: f64 = (s.db[1800][3] * (nv5 - 0.0));
        let eq41_e763_q: f64 = eq41_e762;
        let eq41_reactive_node_derivatives: [f64; 14] = [eq41_e762_d_n0, eq41_e762_d_n1, eq41_e762_d_n2, eq41_e762_d_n3, eq41_e762_d_n4, eq41_e762_d_n5, eq41_e762_d_n6, eq41_e762_d_n7, eq41_e762_d_n8, eq41_e762_d_n9, eq41_e762_d_n10, eq41_e762_d_n11, eq41_e762_d_n12, eq41_e762_d_n13];
        let eq41_reactive_branch_derivatives: [f64; 4] = [eq41_e762_d_b0, eq41_e762_d_b1, eq41_e762_d_b2, eq41_e762_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let eq42_e765: f64 = (-s.v[1801]);
        let eq42_e767: f64 = (eq42_e765 * (nv5 - 0.0));
        let eq42_e767_d_n0: f64 = ((-s.dn[1801][0]) * (nv5 - 0.0));
        let eq42_e767_d_n1: f64 = ((-s.dn[1801][1]) * (nv5 - 0.0));
        let eq42_e767_d_n2: f64 = ((-s.dn[1801][2]) * (nv5 - 0.0));
        let eq42_e767_d_n3: f64 = ((-s.dn[1801][3]) * (nv5 - 0.0));
        let eq42_e767_d_n4: f64 = ((-s.dn[1801][4]) * (nv5 - 0.0));
        let eq42_e767_d_n5: f64 = (((-s.dn[1801][5]) * (nv5 - 0.0)) + eq42_e765);
        let eq42_e767_d_n6: f64 = ((-s.dn[1801][6]) * (nv5 - 0.0));
        let eq42_e767_d_n7: f64 = ((-s.dn[1801][7]) * (nv5 - 0.0));
        let eq42_e767_d_n8: f64 = ((-s.dn[1801][8]) * (nv5 - 0.0));
        let eq42_e767_d_n9: f64 = ((-s.dn[1801][9]) * (nv5 - 0.0));
        let eq42_e767_d_n10: f64 = ((-s.dn[1801][10]) * (nv5 - 0.0));
        let eq42_e767_d_n11: f64 = ((-s.dn[1801][11]) * (nv5 - 0.0));
        let eq42_e767_d_n12: f64 = ((-s.dn[1801][12]) * (nv5 - 0.0));
        let eq42_e767_d_n13: f64 = ((-s.dn[1801][13]) * (nv5 - 0.0));
        let eq42_e767_d_b0: f64 = ((-s.db[1801][0]) * (nv5 - 0.0));
        let eq42_e767_d_b1: f64 = ((-s.db[1801][1]) * (nv5 - 0.0));
        let eq42_e767_d_b2: f64 = ((-s.db[1801][2]) * (nv5 - 0.0));
        let eq42_e767_d_b3: f64 = ((-s.db[1801][3]) * (nv5 - 0.0));
        let eq42_e768_q: f64 = eq42_e767;
        let eq42_reactive_node_derivatives: [f64; 14] = [eq42_e767_d_n0, eq42_e767_d_n1, eq42_e767_d_n2, eq42_e767_d_n3, eq42_e767_d_n4, eq42_e767_d_n5, eq42_e767_d_n6, eq42_e767_d_n7, eq42_e767_d_n8, eq42_e767_d_n9, eq42_e767_d_n10, eq42_e767_d_n11, eq42_e767_d_n12, eq42_e767_d_n13];
        let eq42_reactive_branch_derivatives: [f64; 4] = [eq42_e767_d_b0, eq42_e767_d_b1, eq42_e767_d_b2, eq42_e767_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
        let eq43_e770: f64 = (-s.v[1802]);
        let eq43_e772: f64 = (eq43_e770 * (nv5 - 0.0));
        let eq43_e772_d_n0: f64 = ((-s.dn[1802][0]) * (nv5 - 0.0));
        let eq43_e772_d_n1: f64 = ((-s.dn[1802][1]) * (nv5 - 0.0));
        let eq43_e772_d_n2: f64 = ((-s.dn[1802][2]) * (nv5 - 0.0));
        let eq43_e772_d_n3: f64 = ((-s.dn[1802][3]) * (nv5 - 0.0));
        let eq43_e772_d_n4: f64 = ((-s.dn[1802][4]) * (nv5 - 0.0));
        let eq43_e772_d_n5: f64 = (((-s.dn[1802][5]) * (nv5 - 0.0)) + eq43_e770);
        let eq43_e772_d_n6: f64 = ((-s.dn[1802][6]) * (nv5 - 0.0));
        let eq43_e772_d_n7: f64 = ((-s.dn[1802][7]) * (nv5 - 0.0));
        let eq43_e772_d_n8: f64 = ((-s.dn[1802][8]) * (nv5 - 0.0));
        let eq43_e772_d_n9: f64 = ((-s.dn[1802][9]) * (nv5 - 0.0));
        let eq43_e772_d_n10: f64 = ((-s.dn[1802][10]) * (nv5 - 0.0));
        let eq43_e772_d_n11: f64 = ((-s.dn[1802][11]) * (nv5 - 0.0));
        let eq43_e772_d_n12: f64 = ((-s.dn[1802][12]) * (nv5 - 0.0));
        let eq43_e772_d_n13: f64 = ((-s.dn[1802][13]) * (nv5 - 0.0));
        let eq43_e772_d_b0: f64 = ((-s.db[1802][0]) * (nv5 - 0.0));
        let eq43_e772_d_b1: f64 = ((-s.db[1802][1]) * (nv5 - 0.0));
        let eq43_e772_d_b2: f64 = ((-s.db[1802][2]) * (nv5 - 0.0));
        let eq43_e772_d_b3: f64 = ((-s.db[1802][3]) * (nv5 - 0.0));
        let eq43_e773_q: f64 = eq43_e772;
        let eq43_reactive_node_derivatives: [f64; 14] = [eq43_e772_d_n0, eq43_e772_d_n1, eq43_e772_d_n2, eq43_e772_d_n3, eq43_e772_d_n4, eq43_e772_d_n5, eq43_e772_d_n6, eq43_e772_d_n7, eq43_e772_d_n8, eq43_e772_d_n9, eq43_e772_d_n10, eq43_e772_d_n11, eq43_e772_d_n12, eq43_e772_d_n13];
        let eq43_reactive_branch_derivatives: [f64; 4] = [eq43_e772_d_b0, eq43_e772_d_b1, eq43_e772_d_b2, eq43_e772_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq43_reactive_node_derivatives,
            branches,
            &eq43_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
