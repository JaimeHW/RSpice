#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_102(
        s: &mut ReactiveScratch,
    ) {
        s.b[1725] = (s.v[1349] > 0.005);s.store_scalar(1725, if s.b[1725] { 1.0 } else { 0.0 });
        if (((s.b[1604] && s.b[1720]) && s.b[1721]) && s.b[1725]) {s.store_div_scaled_inputs_mixed_ia(1339, 1349, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1355), 2.0, s.ad_value(1355))), 1.0);s.store_mul(1357, 1339, 1355);s.store_sub_ln_lhs(1358, 1339, 1352);}
        s.b[1726] = (s.v[1349] < (-0.005));s.store_scalar(1726, if s.b[1726] { 1.0 } else { 0.0 });
        if ((((s.b[1604] && s.b[1720]) && s.b[1721]) && (!s.b[1725])) && s.b[1726]) {s.store_sin_scaled_input(1339, 1352, 0.5);s.store_div_scaled_inputs_square_rhs(1357, 1349, -1.0, 1339, 1.0);s.store_ln(1358, 1357);}
        if ((((s.b[1604] && s.b[1720]) && s.b[1721]) && (!s.b[1725])) && (!s.b[1726])) {s.store_sub_from_scalar_ad(1357, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(1358, 1357);}
        s.b[1727] = (((1.01 * s.v[1347]) + s.v[1353]) > 0.0);s.store_scalar(1727, if s.b[1727] { 1.0 } else { 0.0 });
        if (((s.b[1604] && s.b[1720]) && s.b[1721]) && s.b[1727]) {s.store_add(1361, 1347, 1353);s.store_add(1362, 1456, 1354);s.copy_ad(1363, 1356);}
        if (((s.b[1604] && s.b[1720]) && s.b[1721]) && (!s.b[1727])) {s.store_div_from_scalar_sub_ad(1339, 1.0, s.ad_value(1347), s.ad_value(1353));s.store_sub(1340, 1354, 1456);s.store_mul_sub_lhs(1361, 1348, 1357, 1339);s.store_mul_mixed_ai(1362, A::add_scaled_value_products(s.ad_value(1348), (-1.0), s.ad_value(1340), s.ad_value(1361), 1.0, s.ad_value(1359), s.ad_value(1357), (-1.0)), 1339);s.store_mul_mixed_ai(1363, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1356), s.ad_value(1361), 1.0, s.ad_value(1340), s.ad_value(1362), 2.0), 1.0, s.ad_value(1348), 1.0, A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357), (-1.0)), 1339);}
        s.b[1728] = (s.v[1361] > 0.0);s.store_scalar(1728, if s.b[1728] { 1.0 } else { 0.0 });
        if (((s.b[1604] && s.b[1720]) && s.b[1721]) && s.b[1728]) {s.store_ln(1364, 1361);s.store_div_from_scalar(1338, 1.0, 1361);s.store_mul(1365, 1362, 1338);s.store_add_scaled_square_product_indices(1366, 1365, (-1.0), 1363, 1338, 1.0);}
        if (((s.b[1604] && s.b[1720]) && s.b[1721]) && (!s.b[1728])) {s.store_add_offset_lhs_mixed_ia(1364, 1347, 0.6931471805599, A::ln_scaled_input(s.ad_value(1347), -1.0));s.store_div_from_scalar(1338, 1.0, 1525);s.store_add(1365, 1456, 1338);s.store_mul_scale_offset_indices(1366, 1338, 1338, -1.0, 0.0);}
        if ((s.b[1604] && s.b[1720]) && s.b[1721]) {s.store_sub_add_scaled_inputs4_lhs_indices(1367, 1455, 1.0, 1454, (-1.0), 1525, 1.0, 1364, 2.0, 1358);s.store_sub_mixed_ai(1368, A::scale_offset(s.ad_value(1365), 2.0, 1.0), 1359);s.store_sub_scaled_inputs(1369, 1366, 2.0, 1360, 1.0);s.store_add_scaled_product_indices(1370, 1347, 1.0, 1457, 1367, 1.0);s.store_add_scaled_product_indices(1371, 1456, 1.0, 1457, 1368, 1.0);s.store_mul(1372, 1457, 1369);s.store_add_scaled_product_indices(1373, 1348, (-1.0), 1370, 1361, 1.0);s.store_add_mixed_ai(1374, A::add_scaled_products(s.ad_value(1371), s.ad_value(1361), 1.0, s.ad_value(1370), s.ad_value(1362), 1.0), 1348);s.store_sub_mixed_ai(1375, A::add_scaled_products3(s.ad_value(1372), s.ad_value(1361), 1.0, s.ad_value(1371), s.ad_value(1362), 2.0, s.ad_value(1370), s.ad_value(1363), 1.0), 1348);s.store_add_scaled_square_product_indices(1384, 1374, 1.0, 1373, 1375, (-0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_103(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1604] && s.b[1720]) && s.b[1721]) {s.store_div_scaled_product3_mixed_iiia(1376, 1373, 1374, 1384, -1.0, A::offset(A::square(s.ad_value(1384)), 1e-200), 1.0);s.store_add(1525, 1525, 1376);}
        if s.b[1604] {s.store_mul(1528, 1456, 1525);}
        s.b[1729] = (((s.v[1454] - s.v[1525]) - s.v[1524]) < 80.0);s.store_scalar(1729, if s.b[1729] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1729]) {s.store_exp_ad(1338, A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0));}
        if (s.b[1604] && (!s.b[1729])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1338, A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1604] {s.store_mul(1531, 1429, 1338);s.store_sub_square_lhs(1530, 1528, 1531);}
        s.b[1730] = (s.v[1531] <= 0.0);s.store_scalar(1730, if s.b[1730] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1730]) {s.store_scalar(1527, 1e-80);s.store_sub(1529, 1527, 1528);s.store_div(1526, 1529, 1457);}
        s.b[1731] = (s.v[1530] < (-0.005));s.store_scalar(1731, if s.b[1731] { 1.0 } else { 0.0 });
        if ((s.b[1604] && (!s.b[1730])) && s.b[1731]) {s.store_sqrt_abs_ad(1352, s.ad_value(1530));s.store_div_mixed_ia(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));}
        s.b[1732] = (s.v[1530] > 0.005);s.store_scalar(1732, if s.b[1732] { 1.0 } else { 0.0 });
        if (((s.b[1604] && (!s.b[1730])) && (!s.b[1731])) && s.b[1732]) {s.store_sqrt_abs_ad(1352, s.ad_value(1530));s.store_exp_neg_input(1355, 1352);s.store_div_scaled_product_offset_rhs_mixed_iia(1353, 1352, 1355, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1355)), 1.0);}
        if (((s.b[1604] && (!s.b[1730])) && (!s.b[1731])) && (!s.b[1732])) {s.store_offset_ad(1353, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1530), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1530), 1.0, A::scale(s.ad_value(1530), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);}
        s.b[1733] = (((1.01 * s.v[1528]) + s.v[1353]) > 0.0);s.store_scalar(1733, if s.b[1733] { 1.0 } else { 0.0 });
        if ((s.b[1604] && (!s.b[1730])) && s.b[1733]) {s.store_add(1338, 1528, 1353);}
        s.b[1734] = ((s.v[1531] * s.v[1528]) < (((0.9 * s.v[1528]) * s.v[1528]) * s.v[1338]));s.store_scalar(1734, if s.b[1734] { 1.0 } else { 0.0 });
        if (((s.b[1604] && (!s.b[1730])) && s.b[1733]) && s.b[1734]) {s.store_offset_div(1527, 1531, 1338, 1e-80);s.store_sub(1529, 1527, 1528);s.store_div(1526, 1529, 1457);}
        s.b[1735] = (s.v[1530] > 0.005);s.store_scalar(1735, if s.b[1735] { 1.0 } else { 0.0 });
        if ((((s.b[1604] && (!s.b[1730])) && s.b[1733]) && (!s.b[1734])) && s.b[1735]) {s.store_sub_mixed_ai(1339, A::ln(A::div_scaled_inputs(s.ad_value(1530), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1355), 2.0, s.ad_value(1355))), 1.0)), 1352);}
        s.b[1736] = (s.v[1530] < (-0.005));s.store_scalar(1736, if s.b[1736] { 1.0 } else { 0.0 });
        if (((((s.b[1604] && (!s.b[1730])) && s.b[1733]) && (!s.b[1734])) && (!s.b[1735])) && s.b[1736]) {s.store_sin_scaled_input(1340, 1352, 0.5);s.store_ln_div_scaled_input_square_denominator(1339, 1530, -1.0, 1340, 1.0);}
        if (((((s.b[1604] && (!s.b[1730])) && s.b[1733]) && (!s.b[1734])) && (!s.b[1735])) && (!s.b[1736])) {s.store_ln_ad(1339, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1530), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1530), 1.0, A::scale(s.ad_value(1530), 0.0396825396825397), 0.05), 0.3333333333333)));}
        if (((s.b[1604] && (!s.b[1730])) && s.b[1733]) && (!s.b[1734])) {s.store_sub_add_scaled_inputs4_lhs_mixed_iiia(1526, 1455, 1.0, 1454, (-1.0), 1525, 1.0, A::ln(s.ad_value(1338)), 2.0, 1339);s.store_mul(1529, 1457, 1526);s.store_add(1527, 1528, 1529);}
        s.b[1737] = (s.v[1530] > 0.005);s.store_scalar(1737, if s.b[1737] { 1.0 } else { 0.0 });s.b[1738] = ((((s.v[1525] + s.v[1524]) - s.v[1454]) - s.v[1352]) < 80.0);s.store_scalar(1738, if s.b[1738] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_104(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1604] && (!s.b[1730])) && (!s.b[1733])) && s.b[1737]) && s.b[1738]) {s.store_exp_ad(1340, A::add_scaled_inputs4(s.ad_value(1525), 1.0, s.ad_value(1524), 1.0, s.ad_value(1454), -1.0, s.ad_value(1352), -1.0));}
        if ((((s.b[1604] && (!s.b[1730])) && (!s.b[1733])) && s.b[1737]) && (!s.b[1738])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1340, A::add_scaled_inputs4(s.ad_value(1525), 1.0, s.ad_value(1524), 1.0, s.ad_value(1454), -1.0, s.ad_value(1352), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (((s.b[1604] && (!s.b[1730])) && (!s.b[1733])) && s.b[1737]) {s.store_div(1339, 1340, 1429);s.store_div_scaled_product_mixed_iia(1338, 1530, 1339, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1355), 2.0, s.ad_value(1355))), 1.0);}
        s.b[1739] = (s.v[1530] < (-0.005));s.store_scalar(1739, if s.b[1739] { 1.0 } else { 0.0 });
        if ((((s.b[1604] && (!s.b[1730])) && (!s.b[1733])) && (!s.b[1737])) && s.b[1739]) {s.store_sin_scaled_input(1339, 1352, 0.5);s.store_div_scaled_value_by_product_mixed_iai(1338, 1530, -1.0, A::square(s.ad_value(1339)), 1531, 1.0);}
        if ((((s.b[1604] && (!s.b[1730])) && (!s.b[1733])) && (!s.b[1737])) && (!s.b[1739])) {s.store_div_mixed_ai(1338, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1530), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1530), 1.0, A::scale(s.ad_value(1530), 0.0396825396825397), 0.05), 0.3333333333333)), 1531);}
        if ((s.b[1604] && (!s.b[1730])) && (!s.b[1733])) {s.store_offset_div_scaled_inputs2_mixed_iia(1527, 1528, 1.0, 1353, (-1.0), A::sub_from_scalar(1.0, s.ad_value(1338)), 1.0, 1e-80);s.store_sub(1529, 1527, 1528);s.store_div(1526, 1529, 1457);}
        s.b[1740] = (((s.v[1455] - s.v[1526]) - s.v[1524]) < 80.0);s.store_scalar(1740, if s.b[1740] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1740]) {s.store_exp_ad(1338, A::add_scaled_inputs3(s.ad_value(1455), 1.0, s.ad_value(1526), (-1.0), s.ad_value(1524), -1.0));}
        if (s.b[1604] && (!s.b[1740])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1338, A::add_scaled_inputs3(s.ad_value(1455), 1.0, s.ad_value(1526), (-1.0), s.ad_value(1524), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1604] {s.store_mul(1532, 1429, 1338);s.store_scalar(1535, 0.0);s.store_scalar(1536, 0.0);s.store_scalar(1533, 0.0);s.store_scalar(1534, 0.0);s.store_scalar(1537, 0.0);s.store_scalar(1538, 0.0);}
        s.b[1741] = (s.v[1462] > 1e-6);s.store_scalar(1741, if s.b[1741] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1741]) {s.store_mul(1533, 1531, 1430);s.store_mul(1534, 1532, 1431);s.store_add_scaled_inputs(1535, 1533, 1.0, 1528, 2.0);s.store_add_scaled_inputs(1536, 1534, 1.0, 1529, 2.0);s.store_add_scaled_inputs3_indices(1537, 1527, 2.0, 1533, 1.0, 1534, 1.0);}
        s.b[1742] = (((s.v[1530]) as f64).abs() > 0.005);s.store_scalar(1742, if s.b[1742] { 1.0 } else { 0.0 });
        if ((s.b[1604] && s.b[1741]) && s.b[1742]) {s.store_add_scaled_products3_mixed_iiaiai(2, 1535, 1536, 1.0, A::offset(s.ad_value(1525), 2.0), 1536, 2.0, A::offset(s.ad_value(1526), 2.0), 1535, 2.0);s.store_div_scaled_product_by_product_indices(1538, 1530, 1537, (-4.0), 1527, 2, 1.0);}
        if ((s.b[1604] && s.b[1741]) && (!s.b[1742])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2, 1530, 1.0, 1530, 1.0, 1530, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_add_scaled_products3_mixed_iiiiaa(3, 1535, 1531, 1.0, 1536, 1532, 1.0, A::mul3(s.ad_value(1535), s.ad_value(1536), s.ad_value(1527)), A::offset(A::mul(s.ad_value(1527), s.ad_value(2)), 1.0), 1.0);s.store_div_scaled_product3_by_product_indices(1538, 1531, 1532, 1537, 1.0, 1527, 3, 1.0);}
        if s.b[1604] {s.store_add_mixed_ia(1539, 1524, A::ln(s.ad_value(1527)));s.store_scaled_add(1540, 1462, 1527, 0.5);s.store_sub(1541, 1539, 1475);s.store_scalar(1544, 1.0);}
        s.b[1743] = (p.p9 > 0.0);s.store_scalar(1743, if s.b[1743] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_105(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1604] && s.b[1743]) {s.store_div_scaled_inputs2_indices(1542, 1463, 0.5, 1528, 0.5, 1456, 1.0);s.store_scaled_add_offset_sqrt_square_offset(1542, 1542, 1e-5, (-1e-5), 1.0, 0.5);s.store_sub_scaled_inputs_mixed_ai(1, A::sqrt(A::add_scaled_product(A::div(s.ad_value(1542), s.ad_value(223)), 1.0, s.ad_value(246), s.ad_value(246), 0.25)), 1.0, 246, 0.5);s.store_mul_square_lhs(1543, 1, 223);s.store_sub_from_scalar_div_indices(1544, 1.0, 1543, 1542);}
        s.b[1744] = ((s.v[1528] / 2.0) < 80.0);s.store_scalar(1744, if s.b[1744] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1744]) {s.store_ln_one_plus_exp_scaled_input(2, 1528, 0.5);}
        if (s.b[1604] && (!s.b[1744])) {s.store_scale(2, 1528, 0.5);}
        if s.b[1604] {s.store_scale(1545, 2, 2.0);}
        s.b[1745] = ((s.v[1529] / 2.0) < 80.0);s.store_scalar(1745, if s.b[1745] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1745]) {s.store_ln_one_plus_exp_scaled_input(3, 1529, 0.5);}
        if (s.b[1604] && (!s.b[1745])) {s.store_scale(3, 1529, 0.5);}
        if s.b[1604] {s.store_scale(1546, 3, 2.0);s.store_sub(1547, 1546, 1529);s.store_sub(1548, 1545, 1528);s.store_add_scaled_products_indices(1549, 266, 1545, 1.0, 267, 1547, 1.0);s.store_add_scaled_products_indices(1550, 266, 1546, 1.0, 267, 1548, 1.0);s.store_scaled_add(1551, 1476, 1545, 0.5);s.store_scaled_add(1552, 1477, 1546, 0.5);s.store_div_from_scalar_add_ad(0, 1.0, s.ad_value(1551), s.ad_value(1552));s.store_mul3_lhs(1553, 1540, 1551, 0);s.store_mul3_lhs(1554, 1540, 1552, 0);s.store_scaled_add(1555, 1478, 1547, 0.5);s.store_scaled_add(1556, 1479, 1548, 0.5);s.store_scaled_add(1557, 1480, 1549, 0.5);s.store_scaled_add(1558, 1481, 1550, 0.5);s.store_mul_product3_mixed_iiia(1559, 1544, 1551, 187, A::exp(A::mul(s.ad_value(40), s.ad_value(291))), 1.0);s.store_mul_ad_product_rhs_mixed_ia(1560, 1552, 188, A::exp(A::mul(s.ad_value(40), s.ad_value(291))));s.store_add(1561, 1559, 1560);s.store_mul_add_scaled_product_rhs_indices(2, 50, 1555, 1.0, 51, 1556, 1.0);s.store_scaled_add_sqrt_square_offset_ad(3, A::offset(s.ad_value(2), 1.0), 0.01, 0.5);s.store_scaled_add_sqrt_square_offset_ad(4, A::scale_offset(s.ad_value(2), 0.2, 1.0), 0.01, 0.5);s.store_div(1562, 3, 4);s.store_mul_ad_product_rhs(1563, 33, A::add_scaled_product(A::offset(A::mul(s.ad_value(41), s.ad_value(1555)), 1.0), 1.0, s.ad_value(42), s.ad_value(1556), 1.0), A::exp(A::mul_scaled_lhs(s.ad_value(44), -1.0, A::ln(A::add_scaled_product(A::offset(A::mul(s.ad_value(1553), s.ad_value(264)), 1.0), 1.0, s.ad_value(1554), s.ad_value(265), 1.0)))));}
        s.b[1746] = (s.v[56] == 0.0);s.store_scalar(1746, if s.b[1746] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1746]) {s.store_scalar(4, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_106(
        s: &mut ReactiveScratch,
    ) {
        s.b[1747] = (s.v[56] < 0.0);s.store_scalar(1747, if s.b[1747] { 1.0 } else { 0.0 });
        if ((s.b[1604] && (!s.b[1746])) && s.b[1747]) {s.store_mul_exp_mixed_ia(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(1540), 1e-12))));s.store_sub_from_scalar(4, 1.0, 2);}
        if ((s.b[1604] && (!s.b[1746])) && (!s.b[1747])) {s.store_mul_exp_mixed_ia(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(1540), 1e-12))));s.store_div_from_scalar_offset_input(4, 1.0, 2, 1.0);}
        if s.b[1604] {s.store_mul_add_scaled_product_rhs_indices(1564, 1488, 54, 1.0, 1540, 4, 1.0);s.store_add_scaled_inputs_product_mixed_aiii(1565, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(263), s.ad_value(1557)), 1e-6)))), 1.0), 1.0, 1563, 1.0, 38, 1564, 1.0);s.store_add_scaled_inputs_product_mixed_aiii(1566, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(263), s.ad_value(1558)), 1e-6)))), 1.0), 1.0, 1563, 1.0, 39, 1564, 1.0);s.store_div_scaled_product_add_scaled_denominator(1567, 1562, 1561, 1.0, A::div(s.ad_value(1559), s.ad_value(1565)), 1.0, A::div(s.ad_value(1560), s.ad_value(1566)), 1.0, 1.0);s.store_div_from_scalar_offset_input(1568, 1.0, 1540, 4.0);}
        s.b[1748] = (s.v[65] > 0.0);s.store_scalar(1748, if s.b[1748] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1748]) {s.store_div_from_scalar_offset_product(0, 1.0, 65, 1554, 1.0);}
        if (s.b[1604] && (!s.b[1748])) {s.store_sub_from_scalar_scaled_mul(0, 1.0, 65, 1554, 1.0);}
        if s.b[1604] {s.store_mul3_lhs(1569, 1540, 1568, 0);s.store_mul_ln_mixed_ia(1570, 1569, A::offset(A::div_scaled_inputs2(s.ad_value(335), 1.0, s.ad_value(1524), (-1.0), A::add_scaled_product(A::mul3(s.ad_value(67), s.ad_value(1540), s.ad_value(1540)), 1.0, s.ad_value(66), s.ad_value(223), 1.0), 1.0), 1.0));s.store_mul(1571, 1422, 1570);s.store_div_from_scalar_offset_ad(1572, 1.0, A::mul_offset_rhs(s.ad_value(1571), s.ad_value(1571), 1.0), 1.0);s.store_div_scaled_value_offset_denominator(1500, s.ad_value(1551), 100.0, s.ad_value(1551), 100.0, 1.0);}
        s.b[1749] = (s.v[61] < 0.0);s.store_scalar(1749, if s.b[1749] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1749]) {s.store_div_from_scalar_sub_from_scalar_ad(1501, 1.0, 1.0, A::mul(s.ad_value(61), s.ad_value(1500)));}
        if (s.b[1604] && (!s.b[1749])) {s.store_offset_mul(1501, 61, 1500, 1.0);}
        if s.b[1604] {s.store_div_scaled_value_offset_denominator(1502, s.ad_value(1552), 100.0, s.ad_value(1552), 100.0, 1.0);}
        s.b[1750] = (s.v[62] < 0.0);s.store_scalar(1750, if s.b[1750] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1750]) {s.store_div_from_scalar_sub_from_scalar_ad(1503, 1.0, 1.0, A::mul(s.ad_value(62), s.ad_value(1502)));}
        if (s.b[1604] && (!s.b[1750])) {s.store_offset_mul(1503, 62, 1502, 1.0);}
        if s.b[1604] {s.store_mul_ad_affine_product_rhs(1573, 1420, s.ad_value(1541), A::add(s.ad_value(1501), s.ad_value(1503)), 0.5, 0.0);s.store_div_scaled_value_by_product_indices(1574, 1573, 1.0, 1567, 1572, 1.0);s.store_square(1575, 1574);s.store_sqrt_offset_input(1576, 1575, 1.0);s.store_div_scaled_offset_numerator_indices(1577, 1575, 1.5, 1.0, 1576, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_107(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1751] = (p.p13 > 0.0);s.store_scalar(1751, if s.b[1751] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1751]) {s.store_mul_scaled_exp_ln_offset_square_rhs(2, 254, 0.6, 1551, 60.0, (-0.1666666666667));s.store_mul_scaled_exp_ln_offset_square_rhs(3, 254, 0.6, 1552, 60.0, (-0.1666666666667));s.store_div_scaled_offset_numerator_mixed_ai(1578, A::mul(s.ad_value(1456), s.ad_value(2)), 1.0, 1.0, 1437, 1.0);s.store_div_scaled_offset_numerator_mixed_ai(1579, A::mul(s.ad_value(1457), s.ad_value(3)), 1.0, 1.0, 1438, 1.0);}
        if (s.b[1604] && (!s.b[1751])) {s.store_scalar(1578, 1.0);s.store_scalar(1579, 1.0);}
        s.b[1752] = (s.v[1462] > 1e-6);s.store_scalar(1752, if s.b[1752] { 1.0 } else { 0.0 });s.b[1753] = (s.v[1527] > 1e-6);s.store_scalar(1753, if s.b[1753] { 1.0 } else { 0.0 });s.b[1754] = (((s.v[1536]) as f64).abs() < 0.01);s.store_scalar(1754, if s.b[1754] { 1.0 } else { 0.0 });
        if (((s.b[1604] && s.b[1752]) && s.b[1753]) && s.b[1754]) {s.store_div_scaled_inputs2_by_product_mixed_aiai(0, A::offset(s.ad_value(1525), 2.0), 1.0, 1535, 0.5, A::offset(s.ad_value(1526), 2.0), 1535, 1.0);s.store_mul(2, 0, 1536);s.store_square(3, 2);s.store_add_mixed_ai(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);s.store_add_scaled_product_indices(5, 4, 1.0, 2, 3, (-1.0));s.store_div_scaled_inputs2_mixed_iaa(2, 1529, 1.0, A::mul3_scaled_output(s.ad_value(1530), A::sub(s.ad_value(0), A::div_from_scalar(1.0, s.ad_value(1535))), s.ad_value(5), 2.0), (-1.0), A::offset(s.ad_value(1526), 2.0), 1.0);s.store_div_scaled_inputs2_mixed_aii(1580, A::div_scaled_add_product(s.ad_value(1531), (-1.0), s.ad_value(1538), s.ad_value(1527), 1.0, s.ad_value(1535), 1.0), 1.0, 2, (-1.0), 1527, 1.0);s.store_div_scaled_product_offset_denominator_indices(1581, 1580, 1527, 1.0, 1580, 1.0, 1.0);}
        if (((s.b[1604] && s.b[1752]) && s.b[1753]) && (!s.b[1754])) {s.store_sub_ad(1580, A::div_scaled_product_by_product(s.ad_value(1538), s.ad_value(1537), 1.0, s.ad_value(1535), s.ad_value(1536), 1.0), A::div_scaled_inputs2(A::div(s.ad_value(1531), s.ad_value(1535)), 1.0, A::div(s.ad_value(1532), s.ad_value(1536)), 1.0, s.ad_value(1527), 1.0));s.store_div_scaled_product_offset_denominator_indices(1581, 1580, 1527, 1.0, 1580, 1.0, 1.0);}
        if ((s.b[1604] && s.b[1752]) && (!s.b[1753])) {s.copy_ad(1581, 1498);}
        if (s.b[1604] && s.b[1752]) {s.store_sub(2, 1581, 1505);s.store_offset_scaled_mul(3, 2, 2, 36.0, 1.0);}
        s.b[1755] = (((s.v[2]) as f64).abs() > 0.001);s.store_scalar(1755, if s.b[1755] { 1.0 } else { 0.0 });
        if ((s.b[1604] && s.b[1752]) && s.b[1755]) {s.store_sub(4, 1527, 1462);s.store_add_scaled_product_indices(1582, 4, 1.0, 1581, 1541, (-1.0));s.store_add_scaled_product_indices(1583, 4, 1.0, 1505, 1541, (-1.0));s.store_sqrt_square_add(1584, 1582, 3);s.store_sqrt_square_add(1585, 1583, 3);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1586, 0.25, 2, A::add_scaled_products3(s.ad_value(1585), s.ad_value(1582), 1.0, s.ad_value(1584), s.ad_value(1583), (-1.0), s.ad_value(3), A::ln(A::div_scaled_inputs2(s.ad_value(1583), 1.0, s.ad_value(1585), 1.0, A::add(s.ad_value(1582), s.ad_value(1584)), 1.0)), 1.0));}
        if ((s.b[1604] && s.b[1752]) && (!s.b[1755])) {s.store_mul(4, 1541, 2);s.store_div_scaled_product3_mixed_iiia(1586, 1541, 4, 4, ((-0.25) * 0.1666666666667), A::sqrt(s.ad_value(3)), 1.0);}
        if (s.b[1604] && (!s.b[1752])) {s.copy_ad(1581, 1498);s.store_scalar(1586, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_108(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1604] {s.store_add_scaled_inputs3_mixed_aii(1587, A::add_scaled_product(s.ad_value(1586), 1.0, s.ad_value(1540), s.ad_value(1541), 1.0), 1.0, 1462, 1.0, 1527, -1.0);}
        s.b[1756] = (s.v[1462] > 1e-6);s.store_scalar(1756, if s.b[1756] { 1.0 } else { 0.0 });s.b[1757] = (s.v[1587] > 1e-30);s.store_scalar(1757, if s.b[1757] { 1.0 } else { 0.0 });
        if ((s.b[1604] && s.b[1756]) && s.b[1757]) {s.store_div_add_scaled_inputs_rhs_mixed_ai(1588, 1471, A::div(s.ad_value(1467), s.ad_value(1462)), 1.0, 1474, -1.0);s.store_div_add_scaled_inputs_rhs_mixed_ai(1589, 1535, A::div(s.ad_value(1531), s.ad_value(1527)), 1.0, 1538, -1.0);s.store_div_scaled_inputs2_indices(1590, 1588, 1.0, 1589, (-1.0), 1587, 1.0);s.store_div_add_scaled_inputs_rhs_mixed_ai(1591, 1472, A::div(s.ad_value(1468), s.ad_value(1462)), 1.0, 1474, -1.0);s.store_div_add_scaled_inputs_rhs_mixed_ai(1592, 1536, A::div(s.ad_value(1532), s.ad_value(1527)), 1.0, 1538, -1.0);s.store_div_scaled_inputs2_indices(1593, 1591, 1.0, 1592, (-1.0), 1587, 1.0);}
        if ((s.b[1604] && s.b[1756]) && (!s.b[1757])) {s.store_scalar(1590, 0.0);s.store_scalar(1593, 0.0);}
        if (s.b[1604] && (!s.b[1756])) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(1594, 1493, A::div(s.ad_value(1430), s.ad_value(1496)), (-2.0), 1499, (-2.0));s.store_mul_add_scaled_inputs_rhs_mixed_ai(1595, 1494, A::div(s.ad_value(1431), s.ad_value(1497)), (-2.0), 1499, (-2.0));s.store_mul_sub_lhs(0, 1595, 1594, 1499);s.store_mul(2, 1594, 1430);s.store_mul(3, 1595, 1431);s.store_add(4, 2, 3);s.store_offset_ad(5, A::add_scaled_products(s.ad_value(1493), s.ad_value(1430), 2.0, s.ad_value(1494), s.ad_value(1431), 2.0), 3.0);s.store_div_scaled_inputs3_mixed_iiai(1596, 3, 1.0, 0, 1.0, A::div(s.ad_value(4), s.ad_value(1496)), -1.0, 5, 1.0);s.store_div_scaled_inputs3_mixed_iiai(1597, 2, 1.0, 0, (-1.0), A::div(s.ad_value(4), s.ad_value(1497)), -1.0, 5, 1.0);s.store_mul_add_scaled_product_rhs_indices(1590, 1496, 1499, -1.0, 1596, 1496, -1.0);s.store_mul_add_scaled_product_rhs_indices(1593, 1497, 1499, -1.0, 1597, 1497, -1.0);}
        if s.b[1604] {s.store_mul(1598, 1590, 1577);s.store_mul(1599, 1593, 1577);s.store_scaled_sub(1600, 1528, 1463, 0.5);s.store_scaled_sub(1601, 1529, 1464, 0.5);s.store_mul(1602, 1600, 1598);s.store_mul(1603, 1601, 1599);s.copy_ad(436, 1424);s.copy_ad(437, 1428);s.copy_ad(438, 1429);s.copy_ad(439, 1430);s.copy_ad(440, 1431);s.copy_ad(441, 1458);s.copy_ad(442, 1459);s.copy_ad(443, 1443);s.copy_ad(444, 1442);s.copy_ad(445, 1446);s.copy_ad(446, 1447);s.copy_ad(447, 1448);s.copy_ad(448, 1449);s.copy_ad(449, 1450);s.copy_ad(450, 1453);s.copy_ad(451, 1455);s.copy_ad(452, 1456);s.copy_ad(453, 1457);s.copy_ad(454, 1463);s.copy_ad(455, 1464);s.copy_ad(456, 1475);s.copy_ad(457, 1528);s.copy_ad(458, 1529);s.copy_ad(459, 1539);s.copy_ad(460, 1540);s.copy_ad(461, 1544);s.copy_ad(462, 1553);s.copy_ad(463, 1554);s.copy_ad(464, 1575);s.copy_ad(465, 1578);s.copy_ad(466, 1579);s.copy_ad(467, 1600);s.copy_ad(468, 1601);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_109(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1604] {s.copy_ad(469, 1602);s.copy_ad(470, 1603);}
        if (!s.b[1604]) {s.copy_ad(436, 379);s.copy_ad(437, 380);s.copy_ad(438, 381);s.copy_ad(439, 382);s.copy_ad(440, 383);s.copy_ad(441, 384);s.copy_ad(442, 385);s.copy_ad(443, 386);s.copy_ad(444, 387);s.copy_ad(445, 389);s.copy_ad(446, 390);s.copy_ad(447, 391);s.copy_ad(448, 392);s.copy_ad(449, 393);s.copy_ad(450, 394);s.copy_ad(451, 395);s.copy_ad(452, 397);s.copy_ad(453, 398);s.copy_ad(454, 400);s.copy_ad(455, 401);s.copy_ad(456, 402);s.copy_ad(457, 404);s.copy_ad(458, 405);s.copy_ad(459, 410);s.copy_ad(460, 411);s.copy_ad(461, 412);s.copy_ad(462, 415);s.copy_ad(463, 416);s.copy_ad(464, 424);s.copy_ad(465, 426);s.copy_ad(466, 427);s.copy_ad(467, 432);s.copy_ad(468, 433);s.copy_ad(469, 434);s.copy_ad(470, 435);}
        s.store_div_scaled_product_mixed_iaa(0, 120, A::sub(s.ad_value(444), s.ad_value(442)), 1.0, A::scale_offset(s.ad_value(460), 0.25, 1.0), 1.0);s.store_add_scaled_inputs3_indices(1320, 454, 0.5, 457, 0.5, 0, 1.0);s.store_add_scaled_inputs3_indices(1321, 455, 0.5, 458, 0.5, 0, -1.0);s.b[1758] = (p.p13 > 0.0);s.store_scalar(1758, if s.b[1758] { 1.0 } else { 0.0 });
        if s.b[1758] {s.store_add_scaled_inputs3_mixed_iai(1322, 1320, 1.0, A::div(s.ad_value(462), s.ad_value(465)), 1.0, 462, -1.0);s.store_add_scaled_inputs3_mixed_iai(1323, 1321, 1.0, A::div(s.ad_value(463), s.ad_value(466)), 1.0, 463, -1.0);}
        if (!s.b[1758]) {s.copy_ad(1322, 1320);s.copy_ad(1323, 1321);}
        s.store_scaled_mul(2, 467, 469, 0.3333333333333);s.store_mul_scaled_offset_ad_rhs(3, 467, 0.1666666666667, A::mul_sub_from_scalar_rhs(s.ad_value(469), 1.0, A::scale(s.ad_value(469), 0.2)), 1.0);s.store_add_scaled_product_indices(1324, 3, 1.0, 1322, 461, 0.5);s.store_add_scaled_product_indices(1322, 2, 1.0, 1322, 461, 1.0);s.store_scaled_mul(2, 468, 470, 0.3333333333333);s.store_mul_scaled_offset_ad_rhs(3, 468, 0.1666666666667, A::mul_sub_from_scalar_rhs(s.ad_value(470), 1.0, A::scale(s.ad_value(470), 0.2)), 1.0);s.store_add_scaled_inputs(1325, 1323, 0.5, 3, 1.0);s.store_add(1323, 1323, 2);s.store_mul(0, 443, 283);s.store_mul(357, 0, 1322);s.store_mul(358, 0, 1323);s.store_mul_add_scaled_inputs_rhs_indices(359, 0, 1324, -1.0, 1325, -1.0);s.b[1759] = (s.v[119] > 0.0);s.store_scalar(1759, if s.b[1759] { 1.0 } else { 0.0 });
        if s.b[1759] {s.store_offset(0, 250, (2.0 * 0.6931471805599));s.store_add(1326, 456, 0);s.store_add(1327, 459, 0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1328, 1326, 0.5, 250, 0.5, 1326, 250, 9.0, (-0.5));s.store_add_scaled_inputs4_mixed_iiia(1329, 1327, 0.5, 250, 0.5, 335, 0.5, A::sqrt_square_offset(A::add_scaled_inputs3(s.ad_value(1327), 1.0, s.ad_value(250), -1.0, s.ad_value(335), -1.0), 9.0), (-0.5));s.store_mul_sqrt_mixed_ia(1330, 290, A::mul_offset_rhs(s.ad_value(441), s.ad_value(440), 0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_110(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1759] {s.store_mul_sqrt_mixed_ia(1331, 290, A::mul_offset_rhs(A::mul3(s.ad_value(441), s.ad_value(452), s.ad_value(440)), s.ad_value(439), 0.5));s.store_mul_square_lhs(1332, 1330, 287);s.store_mul_square_lhs(1333, 1331, 287);s.store_sub(2, 288, 1328);s.store_add_scaled_inputs3_indices(3, 288, 1.0, 335, 1.0, 1329, -1.0);s.store_scale(0, 1332, 2.0);s.store_add_scaled_offset_product_rhs_mixed_iia(1334, 1328, 1.0, 0, A::sqrt(A::offset(A::div(s.ad_value(2), s.ad_value(1332)), 1.0)), (-1.0), 1.0);s.store_add_scaled_offset_product_rhs_mixed_iia(1335, 1329, 1.0, 0, A::sqrt(A::offset(A::div(s.ad_value(3), s.ad_value(1332)), 1.0)), (-1.0), 1.0);s.store_scale(0, 1333, 2.0);s.store_add_scaled_offset_product_rhs_mixed_iia(1336, 1328, 1.0, 0, A::sqrt(A::offset(A::div(s.ad_value(2), s.ad_value(1333)), 1.0)), (-1.0), 1.0);s.store_add_scaled_offset_product_rhs_mixed_iia(1337, 1329, 1.0, 0, A::sqrt(A::offset(A::div(s.ad_value(3), s.ad_value(1333)), 1.0)), (-1.0), 1.0);s.store_mul(0, 289, 443);s.store_mul_product3_indices(2, 447, 0, 1330, 452, -1.0);s.store_mul_product3_indices(3, 448, 0, 1331, 453, -1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(0, 1334, 0.5, 1326, ((-1.0) * 0.5), 1334, 1326, 1.0, 0.5);s.store_div_scaled_product3_mixed_iiia(375, 2, 0, 0, 1.0, A::sub(s.ad_value(1334), s.ad_value(1328)), 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(0, 1335, 0.5, 1327, ((-1.0) * 0.5), 1335, 1327, 1.0, 0.5);s.store_div_scaled_product3_mixed_iiia(376, 2, 0, 0, 1.0, A::sub(s.ad_value(1335), s.ad_value(1329)), 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(0, 1336, 0.5, 1326, ((-1.0) * 0.5), 1336, 1326, 1.0, 0.5);s.store_div_scaled_product3_mixed_iiia(377, 3, 0, 0, 1.0, A::sub(s.ad_value(1336), s.ad_value(1328)), 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(0, 1337, 0.5, 1327, ((-1.0) * 0.5), 1337, 1327, 1.0, 0.5);s.store_div_scaled_product3_mixed_iiia(378, 3, 0, 0, 1.0, A::sub(s.ad_value(1337), s.ad_value(1329)), 1.0);}
        if (!s.b[1759]) {s.store_scalar(375, 0.0);s.store_scalar(376, 0.0);s.store_scalar(377, 0.0);s.store_scalar(378, 0.0);}
        s.store_mul(366, 164, 326);s.store_mul(367, 165, 328);s.store_scaled_add_sqrt_square_offset_ad(0, A::sub_from_scalar(1.0, A::mul3(s.ad_value(161), s.ad_value(445), A::sub_from_scalar(1.0, A::mul(s.ad_value(162), s.ad_value(436))))), 0.2, 0.5);s.store_mul3_lhs(368, 159, 345, 0);s.store_mul3_lhs(369, 160, 346, 0);s.store_mul(370, 117, 334);s.store_mul(371, 166, 332);s.store_mul_scale_offset_mixed_ia(373, 327, A::add_scaled_products(s.ad_value(236), s.ad_value(9), 1.0, s.ad_value(167), s.ad_value(11), 1.0), -1.0, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_111(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_mul_scale_offset_mixed_ia(372, 329, A::add_scaled_products(s.ad_value(236), s.ad_value(10), 1.0, s.ad_value(167), s.ad_value(12), 1.0), -1.0, 0.0);s.b[1760] = (s.v[6] > 0.0);s.store_scalar(1760, if s.b[1760] { 1.0 } else { 0.0 });
        if s.b[1760] {s.store_mul(374, 170, 215);}
        if (!s.b[1760]) {s.store_scalar(374, 0.0);}
        s.store_scaled_mul(357, 13, 357, p.p32);s.store_scaled_mul(358, 13, 358, p.p32);s.store_scaled_mul(359, 13, 359, p.p32);s.store_add_scaled_inputs3_indices(360, 357, (-1.0), 358, (-1.0), 359, (-1.0));s.store_scaled_mul(375, 13, 375, p.p32);s.store_scaled_mul(376, 13, 376, p.p32);s.store_scaled_mul(377, 13, 377, p.p32);s.store_scaled_mul(378, 13, 378, p.p32);s.store_scaled_mul(366, 13, 366, p.p32);s.store_scaled_mul(367, 13, 367, p.p32);s.store_scaled_mul(368, 13, 368, p.p32);s.store_scaled_mul(369, 13, 369, p.p32);s.store_scaled_mul(370, 13, 370, p.p32);s.store_scaled_mul(373, 13, 373, p.p32);s.store_scaled_mul(372, 13, 372, p.p32);s.store_scaled_mul(371, 13, 371, p.p32);s.store_mul(374, 13, 374);s.b[1769] = (s.v[330] < 0.0);s.store_scalar(1769, if s.b[1769] { 1.0 } else { 0.0 });
        if s.b[1769] {s.copy_ad(1768, 359);s.copy_ad(359, 360);s.copy_ad(360, 1768);s.store_neg(371, 371);s.copy_ad(1768, 376);s.copy_ad(376, 375);s.copy_ad(375, 1768);s.copy_ad(1768, 378);s.copy_ad(378, 377);s.copy_ad(377, 1768);}
        s.store_scaled_mul(1770, 386, 222, 1.0 / (1.602176565e-19));s.store_scaled_add(1771, 403, 428, (-0.5));s.store_add(1772, 411, 1771);s.store_div(0, 411, 1772);s.store_scaled_add_mixed_ia(1777, 0, A::sqrt_square_offset(s.ad_value(0), 1e-20), 0.5);s.store_scaled_mul(1778, 432, 431, (-0.1666666666667));s.store_square(1779, 1778);s.store_offset(1780, 425, (-1.0));s.store_scale(1784, 1779, 12.0);s.store_add_scaled_inputs3_mixed_iia(2, 1777, 1.0, 1784, 1.0, A::mul3_scaled_output(A::offset(s.ad_value(1777), 1.0), s.ad_value(1784), s.ad_value(1780), 2.0), -1.0);s.store_max_with_scalar(3, 2, 1e-40);s.store_div_scaled_product3_indices(1789, 452, 443, 116, 1.0, 465, 1.0);s.store_mul_scale_offset_indices(1790, 1789, 464, 1.0, 1.0);s.store_mul_scale_offset_mixed_ia(1792, 1790, A::mul_scaled_lhs(s.ad_value(330), 0.25, s.ad_value(1778)), -1.0, 0.5);s.store_sub(1791, 1790, 1792);s.b[1803] = (p.p6 > 0.0);s.store_scalar(1803, if s.b[1803] { 1.0 } else { 0.0 });
        if s.b[1803] {s.store_sub_ad(2, A::add_scaled_product(s.ad_value(1777), 0.08333333333333333, s.ad_value(1779), A::sub(A::offset(s.ad_value(1777), 0.2), s.ad_value(1784)), (-1.0)), A::mul3_scaled_output(s.ad_value(1779), A::sub(A::offset(s.ad_value(1777), 1.0), s.ad_value(1784)), s.ad_value(1780), 1.6));s.store_max_with_scalar(3, 2, 1e-40);}
        s.copy_ad(1773, 1770);s.store_mul_scale_offset_indices(1774, 1770, 411, 1.0, 1.0);s.store_mul_sub_rhs(1775, 1770, 399, 409);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_112(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_mul_add(2, A::ln(A::div_scaled_inputs2(s.ad_value(1774), 1.0, s.ad_value(1775), 0.5, A::sub_scaled_inputs(s.ad_value(1774), 1.0, s.ad_value(1775), 0.5), 1.0)), A::add_scaled_product(s.ad_value(173), 1.0, s.ad_value(174), s.ad_value(1773), (-1.0)), A::mul3(s.ad_value(175), s.ad_value(1773), s.ad_value(1773)));s.store_add_scaled_product_mixed_iai(3, 2, 1.0, A::add_scaled_product(s.ad_value(174), 1.0, s.ad_value(175), A::sub_scaled_inputs(s.ad_value(1774), 1.0, s.ad_value(1773), 2.0), 1.0), 1775, 1.0);s.store_offset_div_ad(0, A::add_scaled_products(s.ad_value(176), s.ad_value(413), 1.0, s.ad_value(177), s.ad_value(414), 1.0), A::offset(s.ad_value(411), 1.0), 1.0);s.store_scaled_add_offset_sqrt_square_offset(4, 0, 0.01, (-0.01), 0.0001, 0.5);s.store_mul_div_scaled_product_mixed_iaii(0, 4, A::div_scaled_product(s.ad_value(343), s.ad_value(344), 1.602176565e-19, s.ad_value(341), 1.0), 3, 1.0, 1773, 1.0);s.store_div_from_scalar_scaled_input(1813, 1.0, 8, 8.617332384961e-5);s.store_sub_from_scalar_ad(1814, 1.17, A::div_scaled_product_offset_denominator(s.ad_value(8), s.ad_value(8), 0.000473, s.ad_value(8), 636.0, 1.0));s.store_sub_from_scalar_ad(1815, 0.744, A::div_scaled_product_offset_denominator(s.ad_value(8), s.ad_value(8), 0.0004774, s.ad_value(8), 235.0, 1.0));s.store_mul_add_scaled_inputs3_offset_rhs_indices(1816, 15, 1815, 1.0, 1814, (-1.0), 224, (-0.4), 0.0);s.store_add(1817, 1814, 1816);s.store_scaled_mul(1818, 1817, 1813, 0.5);s.store_sub_scaled_inputs(1819, 15, 0.05, 1816, 0.5);s.store_sqrt_scaled_input(0, 8, 0.0033333333333);s.store_mul3_affine_lhs(2, 0, 0, 4.05e25, 0.0, 0);s.store_mul(1820, 2, 234);s.store_div_scaled_value_offset_denominator(1821, s.ad_value(1813), 1.0, A::div_scaled_inputs(s.ad_value(17), s.v[7], s.ad_value(8), 1.0), 1.0, 1.0);s.store_mul3_affine_lhs(1823, 1820, 225, (2.0 * 1.602176565e-19), 0.0, 1821);s.store_add_offset_lhs_mixed_ai(1824, A::ln(A::div_scaled_product(s.ad_value(241), s.ad_value(241), 1.0, s.ad_value(1823), 1.0)), (-0.6931471805599), 1818);s.store_mul_div_scaled_product_mixed_iiia(1825, 1821, 29, 14, (0.5 * 1.602176565e-19), A::add(s.ad_value(237), s.ad_value(238)), 1.0);s.store_mul(1828, 35, 1821);s.store_scalar(1829, 0.0);s.store_scalar(1822, 0.0);s.b[1874] = (p.p9 > 0.0);s.store_scalar(1874, if s.b[1874] { 1.0 } else { 0.0 });
        if s.b[1874] {s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1822, 1.0, 1813, A::ln(A::div(s.ad_value(24), s.ad_value(247))));}
        s.b[1875] = (p.p13 > 0.0);s.store_scalar(1875, if s.b[1875] { 1.0 } else { 0.0 });s.b[1876] = (p.p14 == 1.0);s.store_scalar(1876, if s.b[1876] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_113(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1875] && s.b[1876]) {s.store_scale_ad(1829, A::exp_scaled_input(A::ln(A::div(s.ad_value(255), s.ad_value(1821))), (-0.3333333333333)), ((0.4 * p.p13) * 1.27520989));}
        if (s.b[1875] && (!s.b[1876])) {s.store_scale_ad(1829, A::exp_scaled_input(A::ln(A::div(s.ad_value(255), s.ad_value(1821))), (-0.3333333333333)), ((0.4 * p.p13) * 1.5412087));}
        s.store_mul(1832, 332, 1821);s.store_mul_scale_offset_mixed_ia(1833, 1821, A::sqrt_square_offset(s.ad_value(332), 0.01), 1.0, (-0.1));s.store_scaled_sub(1834, 1832, 1833, 0.5);s.store_div_scaled_value_by_product_mixed_iia(1805, 398, 1.0, 397, A::offset(s.ad_value(398), 1.0), 1.0);s.store_div_scaled_value_by_product_mixed_iia(1806, 397, 1.0, 398, A::offset(s.ad_value(397), 1.0), 1.0);s.store_offset_ln_ad(1807, A::div_scaled_product3(s.ad_value(397), A::offset(s.ad_value(1805), 1.0), s.ad_value(380), 1.0, s.ad_value(381), 1.0), 2.0);s.store_offset_ln_ad(1808, A::div_scaled_product3(s.ad_value(398), A::offset(s.ad_value(1806), 1.0), s.ad_value(380), 1.0, s.ad_value(381), 1.0), 2.0);s.store_add_scaled_products_mixed_aiii(1809, A::offset(s.ad_value(1805), 1.0), 1807, 1.0, 395, 1805, (-1.0));s.store_add_scaled_offset_product_lhs_mixed_aai(1810, A::div(s.ad_value(395), s.ad_value(1806)), (-1.0), A::div_from_scalar(1.0, s.ad_value(1806)), 1.0, 1808, 1.0);s.store_add_mixed_ai(1811, A::div_scaled_inputs4(s.ad_value(1809), 0.5, s.ad_value(1810), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1809), s.ad_value(1810)), 38.0), (-0.5), s.ad_value(394), -1.0, s.ad_value(25), 1.0), 394);s.store_add_scaled_product_mixed_iia(1812, 21, 1.0, 222, A::add_scaled_inputs3(A::div_scaled_inputs2(s.ad_value(1811), 1.0, s.ad_value(390), (-1.0), s.ad_value(391), 1.0), 1.0, s.ad_value(393), (-1.0), s.ad_value(390), 1.0), 1.0);s.store_mul_scale_offset_indices(0, 34, 8, 1.0, (-s.v[7]));s.store_add_scaled_offset_product_rhs(0, 252, 1.0, 23, 8, (-s.v[7]), p.p14);s.store_sub_offset_lhs_mixed_ai(1830, A::add_scaled_inputs4(s.ad_value(179), p.p14, s.ad_value(1819), p.p14, s.ad_value(239), p.p14, s.ad_value(0), 1.0), p.p34, 1822);s.store_add_scaled_inputs4_indices(1831, 180, p.p14, 1819, p.p14, 240, p.p14, 0, 1.0);s.store_add_scaled_product_mixed_iai(1835, 1834, (-1.0), A::sub(s.ad_value(1812), s.ad_value(1830)), 1821, 1.0);s.store_add_scaled_product_mixed_iai(1836, 1834, (-1.0), A::sub_scaled_inputs(s.ad_value(333), -1.0, s.ad_value(1831), 1.0), 1821, 1.0);s.b[1877] = (p.p2 > 0.0);s.store_scalar(1877, if s.b[1877] { 1.0 } else { 0.0 });
        if s.b[1877] {s.store_div_scaled_product_mixed_iai(0, 16, A::sub(s.ad_value(1835), s.ad_value(1836)), p.p14, 256, 1.0);}
        s.b[1878] = (s.v[0] < 0.0);s.store_scalar(1878, if s.b[1878] { 1.0 } else { 0.0 });
        if (s.b[1877] && s.b[1878]) {s.store_scaled_ln_ad(2, A::sub_from_scalar(1.0, s.ad_value(0)), (-2.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_114(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1877] && (!s.b[1878])) {s.store_div_scaled_product_offset_denominator_mixed_iia(2, 0, 0, 1.0, A::div_scaled_inputs(s.ad_value(0), 2.0, s.ad_value(256), 1.0), 1.0, 1.0);}
        if s.b[1877] {s.store_add_scaled_product_indices(1837, 1836, 1.0, 16, 2, p.p14);}
        if (!s.b[1877]) {s.copy_ad(1837, 1836);}
        s.store_mul_sub_rhs(0, 244, 1835, 1837);s.b[1879] = (p.p13 > 0.0);s.store_scalar(1879, if s.b[1879] { 1.0 } else { 0.0 });
        if s.b[1879] {s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1838, 0, 0.5, 253, 0.5, A::add(A::square(A::sub(s.ad_value(0), s.ad_value(253))), A::square(s.ad_value(253))), 0.5);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1839, 253, 0.5, 0, ((-1.0) * 0.5), A::add(A::square(A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(253), 1.0)), A::square(s.ad_value(253))), 0.5);s.store_mul_mixed_ia(2, 1829, A::exp_scaled_input(A::ln(s.ad_value(1838)), (-0.3333333333333)));s.store_mul_mixed_ia(3, 1829, A::exp_scaled_input(A::ln(s.ad_value(1839)), (-0.3333333333333)));s.store_sub_mixed_ai(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);s.store_div_scaled_product_offset_denominator_mixed_iia(1841, 242, 4, 1.0, A::mul(s.ad_value(242), s.ad_value(2)), 1.0, 1.0);s.store_div_scaled_product_offset_denominator_mixed_iia(1842, 243, 4, 1.0, A::mul(s.ad_value(243), s.ad_value(3)), 1.0, 1.0);s.store_div_from_scalar_add_ad(1843, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(1841)), 1.0), A::div_from_scalar(1.0, s.ad_value(1842)));}
        if (!s.b[1879]) {s.copy_ad(1841, 242);s.copy_ad(1842, 243);s.copy_ad(1843, 244);}
        s.store_mul_sub_rhs(1844, 1843, 1835, 1837);s.b[1880] = (s.v[1844] > 0.0);s.store_scalar(1880, if s.b[1880] { 1.0 } else { 0.0 });s.b[1881] = ((-s.v[1844]) < 80.0);s.store_scalar(1881, if s.b[1881] { 1.0 } else { 0.0 });
        if (s.b[1880] && s.b[1881]) {s.store_ln_one_plus_exp_neg_input(0, 1844);}
        if (s.b[1880] && (!s.b[1881])) {s.store_neg(0, 1844);}
        if s.b[1880] {s.store_add_scaled_inputs3_offset_mixed_iai(1845, 1835, 1.0, A::div(s.ad_value(1844), s.ad_value(1841)), (-1.0), 0, 1.0, (-0.6931471805599));}
        s.b[1882] = (s.v[1844] < 80.0);s.store_scalar(1882, if s.b[1882] { 1.0 } else { 0.0 });
        if ((!s.b[1880]) && s.b[1882]) {s.store_ln_one_plus_exp(0, 1844);}
        if ((!s.b[1880]) && (!s.b[1882])) {s.copy_ad(0, 1844);}
        if (!s.b[1880]) {s.store_add_scaled_inputs3_offset_mixed_iai(1845, 1837, 1.0, A::div(s.ad_value(1844), s.ad_value(1842)), 1.0, 0, 1.0, (-0.6931471805599));}
        s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1846, 1845, 0.5, 1824, 0.5, 1845, 1824, 4.0, (-0.5));s.store_offset_sqrt_ad(1847, A::offset(A::div_scaled_inputs2(s.ad_value(1824), 2.0, s.ad_value(1846), (-2.0), s.ad_value(1825), 1.0), 1.0), (-1.0));s.store_scaled_add_offset_sqrt_square_offset_ad(0, A::mul(s.ad_value(30), s.ad_value(1836)), ((1.0) + (0.5)), ((1.0) + ((-0.5))), 0.01, 0.5);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_115(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_mul_scale_offset(0, A::mul3_scaled_output(s.ad_value(1828), A::offset(A::sqrt(A::offset(A::div(s.ad_value(1833), s.ad_value(1828)), 1.0)), (-1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(1847)), 1.0), 2.0), A::mul(s.ad_value(37), s.ad_value(1836)), 1.0, 1.0);s.b[1884] = (p.p11 > 0.0);s.store_scalar(1884, if s.b[1884] { 1.0 } else { 0.0 });
        if s.b[1884] {s.store_div_scaled_value_by_product_mixed_iia(1805, 453, 1.0, 452, A::offset(s.ad_value(453), 1.0), 1.0);s.store_div_scaled_value_by_product_mixed_iia(1806, 452, 1.0, 453, A::offset(s.ad_value(452), 1.0), 1.0);s.store_offset_ln_ad(1807, A::div_scaled_product3(s.ad_value(452), A::offset(s.ad_value(1805), 1.0), s.ad_value(437), 1.0, s.ad_value(438), 1.0), 2.0);s.store_offset_ln_ad(1808, A::div_scaled_product3(s.ad_value(453), A::offset(s.ad_value(1806), 1.0), s.ad_value(437), 1.0, s.ad_value(438), 1.0), 2.0);s.store_add_scaled_products_mixed_aiii(1809, A::offset(s.ad_value(1805), 1.0), 1807, 1.0, 451, 1805, (-1.0));s.store_add_scaled_offset_product_lhs_mixed_aai(1810, A::div(s.ad_value(451), s.ad_value(1806)), (-1.0), A::div_from_scalar(1.0, s.ad_value(1806)), 1.0, 1808, 1.0);s.store_add_mixed_ai(1811, A::div_scaled_inputs4(s.ad_value(1809), 0.5, s.ad_value(1810), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1809), s.ad_value(1810)), 38.0), (-0.5), s.ad_value(450), -1.0, s.ad_value(25), 1.0), 450);s.store_add_scaled_product_mixed_iia(1812, 130, 1.0, 222, A::add_scaled_inputs3(A::div_scaled_inputs2(s.ad_value(1811), 1.0, s.ad_value(446), (-1.0), s.ad_value(447), 1.0), 1.0, s.ad_value(449), (-1.0), s.ad_value(446), 1.0), 1.0);s.store_mul_scale_offset_indices(0, 34, 8, 1.0, (-s.v[7]));s.store_add_scaled_offset_product_rhs(0, 252, 1.0, 23, 8, (-s.v[7]), p.p14);s.store_sub_offset_lhs_mixed_ai(1830, A::add_scaled_inputs4(s.ad_value(181), p.p14, s.ad_value(1819), p.p14, s.ad_value(239), p.p14, s.ad_value(0), 1.0), p.p34, 1822);s.store_add_scaled_inputs4_indices(1831, 182, p.p14, 1819, p.p14, 240, p.p14, 0, 1.0);s.store_add_scaled_product_mixed_iai(1835, 1834, (-1.0), A::sub(s.ad_value(1812), s.ad_value(1830)), 1821, 1.0);s.store_add_scaled_product_mixed_iai(1836, 1834, (-1.0), A::sub_scaled_inputs(s.ad_value(333), -1.0, s.ad_value(1831), 1.0), 1821, 1.0);}
        s.b[1885] = (p.p2 > 0.0);s.store_scalar(1885, if s.b[1885] { 1.0 } else { 0.0 });
        if (s.b[1884] && s.b[1885]) {s.store_div_scaled_product_mixed_iai(0, 16, A::sub(s.ad_value(1835), s.ad_value(1836)), p.p14, 256, 1.0);}
        s.b[1886] = (s.v[0] < 0.0);s.store_scalar(1886, if s.b[1886] { 1.0 } else { 0.0 });
        if ((s.b[1884] && s.b[1885]) && s.b[1886]) {s.store_scaled_ln_ad(2, A::sub_from_scalar(1.0, s.ad_value(0)), (-2.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_116(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1884] && s.b[1885]) && (!s.b[1886])) {s.store_div_scaled_product_offset_denominator_mixed_iia(2, 0, 0, 1.0, A::div_scaled_inputs(s.ad_value(0), 2.0, s.ad_value(256), 1.0), 1.0, 1.0);}
        if (s.b[1884] && s.b[1885]) {s.store_add_scaled_product_indices(1837, 1836, 1.0, 16, 2, p.p14);}
        if (s.b[1884] && (!s.b[1885])) {s.copy_ad(1837, 1836);}
        if s.b[1884] {s.store_mul_sub_rhs(0, 244, 1835, 1837);}
        s.b[1887] = (p.p13 > 0.0);s.store_scalar(1887, if s.b[1887] { 1.0 } else { 0.0 });
        if (s.b[1884] && s.b[1887]) {s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1838, 0, 0.5, 253, 0.5, A::add(A::square(A::sub(s.ad_value(0), s.ad_value(253))), A::square(s.ad_value(253))), 0.5);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1839, 253, 0.5, 0, ((-1.0) * 0.5), A::add(A::square(A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(253), 1.0)), A::square(s.ad_value(253))), 0.5);s.store_mul_mixed_ia(2, 1829, A::exp_scaled_input(A::ln(s.ad_value(1838)), (-0.3333333333333)));s.store_mul_mixed_ia(3, 1829, A::exp_scaled_input(A::ln(s.ad_value(1839)), (-0.3333333333333)));s.store_sub_mixed_ai(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);s.store_div_scaled_product_offset_denominator_mixed_iia(1841, 242, 4, 1.0, A::mul(s.ad_value(242), s.ad_value(2)), 1.0, 1.0);s.store_div_scaled_product_offset_denominator_mixed_iia(1842, 243, 4, 1.0, A::mul(s.ad_value(243), s.ad_value(3)), 1.0, 1.0);s.store_div_from_scalar_add_ad(1843, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(1841)), 1.0), A::div_from_scalar(1.0, s.ad_value(1842)));}
        if (s.b[1884] && (!s.b[1887])) {s.copy_ad(1841, 242);s.copy_ad(1842, 243);s.copy_ad(1843, 244);}
        if s.b[1884] {s.store_mul_sub_rhs(1844, 1843, 1835, 1837);}
        s.b[1888] = (s.v[1844] > 0.0);s.store_scalar(1888, if s.b[1888] { 1.0 } else { 0.0 });s.b[1889] = ((-s.v[1844]) < 80.0);s.store_scalar(1889, if s.b[1889] { 1.0 } else { 0.0 });
        if ((s.b[1884] && s.b[1888]) && s.b[1889]) {s.store_ln_one_plus_exp_neg_input(0, 1844);}
        if ((s.b[1884] && s.b[1888]) && (!s.b[1889])) {s.store_neg(0, 1844);}
        if (s.b[1884] && s.b[1888]) {s.store_add_scaled_inputs3_offset_mixed_iai(1845, 1835, 1.0, A::div(s.ad_value(1844), s.ad_value(1841)), (-1.0), 0, 1.0, (-0.6931471805599));}
        s.b[1890] = (s.v[1844] < 80.0);s.store_scalar(1890, if s.b[1890] { 1.0 } else { 0.0 });
        if ((s.b[1884] && (!s.b[1888])) && s.b[1890]) {s.store_ln_one_plus_exp(0, 1844);}
        if ((s.b[1884] && (!s.b[1888])) && (!s.b[1890])) {s.copy_ad(0, 1844);}
        if (s.b[1884] && (!s.b[1888])) {s.store_add_scaled_inputs3_offset_mixed_iai(1845, 1837, 1.0, A::div(s.ad_value(1844), s.ad_value(1842)), 1.0, 0, 1.0, (-0.6931471805599));}
        if s.b[1884] {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1846, 1845, 0.5, 1824, 0.5, 1845, 1824, 4.0, (-0.5));s.store_offset_sqrt_ad(1847, A::offset(A::div_scaled_inputs2(s.ad_value(1824), 2.0, s.ad_value(1846), (-2.0), s.ad_value(1825), 1.0), 1.0), (-1.0));s.store_scaled_add_offset_sqrt_square_offset_ad(0, A::mul(s.ad_value(30), s.ad_value(1836)), ((1.0) + (0.5)), ((1.0) + ((-0.5))), 0.01, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_117(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1884] {s.store_mul_scale_offset(0, A::mul3_scaled_output(s.ad_value(1828), A::offset(A::sqrt(A::offset(A::div(s.ad_value(1833), s.ad_value(1828)), 1.0)), (-1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(1847)), 1.0), 2.0), A::mul(s.ad_value(37), s.ad_value(1836)), 1.0, 1.0);}
    }
}
