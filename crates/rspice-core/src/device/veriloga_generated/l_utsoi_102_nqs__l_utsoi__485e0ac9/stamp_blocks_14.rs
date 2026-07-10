#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_101(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[1608] && s.b[1724]) && s.b[1725]) && s.b[1727]) {s.store_div_mixed_ia(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);s.store_mul_add_mixed_iia(1358, 1342, 1353, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)));s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);s.store_mul_div_lhs(1363, 1354, 1353, 1343);s.store_div_mixed_ai(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);}
        s.b[1728] = (s.v[1353] > 0.005);s.store_scalar(1728, if s.b[1728] { 1.0 } else { 0.0 });
        if ((((s.b[1608] && s.b[1724]) && s.b[1725]) && (!s.b[1727])) && s.b[1728]) {s.store_sqrt_abs_ad(1356, s.ad_value(1353));s.store_exp_neg_input(1359, 1356);s.store_div_scaled_product_offset_rhs_mixed_iia(1357, 1356, 1359, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0);s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);s.store_mul_add_mixed_iia(1358, 1342, 1353, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)));s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);s.store_mul_div_lhs(1363, 1354, 1353, 1343);s.store_div_mixed_ai(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);}
        if ((((s.b[1608] && s.b[1724]) && s.b[1725]) && (!s.b[1727])) && (!s.b[1728])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1344, 1353, 1.0, 1353, 1.0, 1353, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(1357, 1353, 1344, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1342, 1353, 1.0, 1353, 1.0, 1353, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(1358, 1354, 1342);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1343, 1353, 1.0, 1353, 1.0, 1353, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(1360, 1355, 1342, 1.0, A::square(s.ad_value(1354)), 1343, (-1.0));s.store_scaled_mul(1363, 1354, 1344, (-0.5));s.store_add_scaled_product_mixed_aii(1364, A::mul3_scaled_output(s.ad_value(1354), s.ad_value(1354), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 2.0, A::scale(s.ad_value(1353), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1355, 1344, (-0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_102(
        s: &mut ReactiveScratch,
    ) {
        s.b[1729] = (s.v[1353] > 0.005);s.store_scalar(1729, if s.b[1729] { 1.0 } else { 0.0 });
        if (((s.b[1608] && s.b[1724]) && s.b[1725]) && s.b[1729]) {s.store_div_scaled_inputs_mixed_ia(1343, 1353, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0);s.store_mul(1361, 1343, 1359);s.store_sub_ln_lhs(1362, 1343, 1356);}
        s.b[1730] = (s.v[1353] < (-0.005));s.store_scalar(1730, if s.b[1730] { 1.0 } else { 0.0 });
        if ((((s.b[1608] && s.b[1724]) && s.b[1725]) && (!s.b[1729])) && s.b[1730]) {s.store_sin_scaled_input(1343, 1356, 0.5);s.store_div_scaled_inputs_square_rhs(1361, 1353, -1.0, 1343, 1.0);s.store_ln(1362, 1361);}
        if ((((s.b[1608] && s.b[1724]) && s.b[1725]) && (!s.b[1729])) && (!s.b[1730])) {s.store_sub_from_scalar_ad(1361, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(1362, 1361);}
        s.b[1731] = (((1.01 * s.v[1351]) + s.v[1357]) > 0.0);s.store_scalar(1731, if s.b[1731] { 1.0 } else { 0.0 });
        if (((s.b[1608] && s.b[1724]) && s.b[1725]) && s.b[1731]) {s.store_add(1365, 1351, 1357);s.store_add(1366, 1460, 1358);s.copy_ad(1367, 1360);}
        if (((s.b[1608] && s.b[1724]) && s.b[1725]) && (!s.b[1731])) {s.store_div_from_scalar_sub_ad(1343, 1.0, s.ad_value(1351), s.ad_value(1357));s.store_sub(1344, 1358, 1460);s.store_mul_sub_lhs(1365, 1352, 1361, 1343);s.store_mul_mixed_ai(1366, A::add_scaled_value_products(s.ad_value(1352), (-1.0), s.ad_value(1344), s.ad_value(1365), 1.0, s.ad_value(1363), s.ad_value(1361), (-1.0)), 1343);s.store_mul_mixed_ai(1367, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1360), s.ad_value(1365), 1.0, s.ad_value(1344), s.ad_value(1366), 2.0), 1.0, s.ad_value(1352), 1.0, A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361), (-1.0)), 1343);}
        s.b[1732] = (s.v[1365] > 0.0);s.store_scalar(1732, if s.b[1732] { 1.0 } else { 0.0 });
        if (((s.b[1608] && s.b[1724]) && s.b[1725]) && s.b[1732]) {s.store_ln(1368, 1365);s.store_div_from_scalar(1342, 1.0, 1365);s.store_mul(1369, 1366, 1342);s.store_add_scaled_square_product_indices(1370, 1369, (-1.0), 1367, 1342, 1.0);}
        if (((s.b[1608] && s.b[1724]) && s.b[1725]) && (!s.b[1732])) {s.store_add_offset_lhs_mixed_ia(1368, 1351, 0.6931471805599, A::ln_scaled_input(s.ad_value(1351), -1.0));s.store_div_from_scalar(1342, 1.0, 1529);s.store_add(1369, 1460, 1342);s.store_mul_scale_offset_indices(1370, 1342, 1342, -1.0, 0.0);}
        if ((s.b[1608] && s.b[1724]) && s.b[1725]) {s.store_sub_add_scaled_inputs4_lhs_indices(1371, 1459, 1.0, 1458, (-1.0), 1529, 1.0, 1368, 2.0, 1362);s.store_sub_mixed_ai(1372, A::scale_offset(s.ad_value(1369), 2.0, 1.0), 1363);s.store_sub_scaled_inputs(1373, 1370, 2.0, 1364, 1.0);s.store_add_scaled_product_indices(1374, 1351, 1.0, 1461, 1371, 1.0);s.store_add_scaled_product_indices(1375, 1460, 1.0, 1461, 1372, 1.0);s.store_mul(1376, 1461, 1373);s.store_add_scaled_product_indices(1377, 1352, (-1.0), 1374, 1365, 1.0);s.store_add_mixed_ai(1378, A::add_scaled_products(s.ad_value(1375), s.ad_value(1365), 1.0, s.ad_value(1374), s.ad_value(1366), 1.0), 1352);s.store_sub_mixed_ai(1379, A::add_scaled_products3(s.ad_value(1376), s.ad_value(1365), 1.0, s.ad_value(1375), s.ad_value(1366), 2.0, s.ad_value(1374), s.ad_value(1367), 1.0), 1352);s.store_add_scaled_square_product_indices(1388, 1378, 1.0, 1377, 1379, (-0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_103(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1608] && s.b[1724]) && s.b[1725]) {s.store_div_scaled_product3_mixed_iiia(1380, 1377, 1378, 1388, -1.0, A::offset(A::square(s.ad_value(1388)), 1e-200), 1.0);s.store_add(1529, 1529, 1380);}
        if s.b[1608] {s.store_mul(1532, 1460, 1529);}
        s.b[1733] = (((s.v[1458] - s.v[1529]) - s.v[1528]) < 80.0);s.store_scalar(1733, if s.b[1733] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1733]) {s.store_exp_ad(1342, A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0));}
        if (s.b[1608] && (!s.b[1733])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1342, A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1608] {s.store_mul(1535, 1433, 1342);s.store_sub_square_lhs(1534, 1532, 1535);}
        s.b[1734] = (s.v[1535] <= 0.0);s.store_scalar(1734, if s.b[1734] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1734]) {s.store_scalar(1531, 1e-80);s.store_sub(1533, 1531, 1532);s.store_div(1530, 1533, 1461);}
        s.b[1735] = (s.v[1534] < (-0.005));s.store_scalar(1735, if s.b[1735] { 1.0 } else { 0.0 });
        if ((s.b[1608] && (!s.b[1734])) && s.b[1735]) {s.store_sqrt_abs_ad(1356, s.ad_value(1534));s.store_div_mixed_ia(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));}
        s.b[1736] = (s.v[1534] > 0.005);s.store_scalar(1736, if s.b[1736] { 1.0 } else { 0.0 });
        if (((s.b[1608] && (!s.b[1734])) && (!s.b[1735])) && s.b[1736]) {s.store_sqrt_abs_ad(1356, s.ad_value(1534));s.store_exp_neg_input(1359, 1356);s.store_div_scaled_product_offset_rhs_mixed_iia(1357, 1356, 1359, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0);}
        if (((s.b[1608] && (!s.b[1734])) && (!s.b[1735])) && (!s.b[1736])) {s.store_offset_ad(1357, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1534), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1534), 1.0, A::scale(s.ad_value(1534), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);}
        s.b[1737] = (((1.01 * s.v[1532]) + s.v[1357]) > 0.0);s.store_scalar(1737, if s.b[1737] { 1.0 } else { 0.0 });
        if ((s.b[1608] && (!s.b[1734])) && s.b[1737]) {s.store_add(1342, 1532, 1357);}
        s.b[1738] = ((s.v[1535] * s.v[1532]) < (((0.9 * s.v[1532]) * s.v[1532]) * s.v[1342]));s.store_scalar(1738, if s.b[1738] { 1.0 } else { 0.0 });
        if (((s.b[1608] && (!s.b[1734])) && s.b[1737]) && s.b[1738]) {s.store_offset_div(1531, 1535, 1342, 1e-80);s.store_sub(1533, 1531, 1532);s.store_div(1530, 1533, 1461);}
        s.b[1739] = (s.v[1534] > 0.005);s.store_scalar(1739, if s.b[1739] { 1.0 } else { 0.0 });
        if ((((s.b[1608] && (!s.b[1734])) && s.b[1737]) && (!s.b[1738])) && s.b[1739]) {s.store_sub_mixed_ai(1343, A::ln(A::div_scaled_inputs(s.ad_value(1534), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0)), 1356);}
        s.b[1740] = (s.v[1534] < (-0.005));s.store_scalar(1740, if s.b[1740] { 1.0 } else { 0.0 });
        if (((((s.b[1608] && (!s.b[1734])) && s.b[1737]) && (!s.b[1738])) && (!s.b[1739])) && s.b[1740]) {s.store_sin_scaled_input(1344, 1356, 0.5);s.store_ln_div_scaled_input_square_denominator(1343, 1534, -1.0, 1344, 1.0);}
        if (((((s.b[1608] && (!s.b[1734])) && s.b[1737]) && (!s.b[1738])) && (!s.b[1739])) && (!s.b[1740])) {s.store_ln_ad(1343, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1534), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1534), 1.0, A::scale(s.ad_value(1534), 0.0396825396825397), 0.05), 0.3333333333333)));}
        if (((s.b[1608] && (!s.b[1734])) && s.b[1737]) && (!s.b[1738])) {s.store_sub_add_scaled_inputs4_lhs_mixed_iiia(1530, 1459, 1.0, 1458, (-1.0), 1529, 1.0, A::ln(s.ad_value(1342)), 2.0, 1343);s.store_mul(1533, 1461, 1530);s.store_add(1531, 1532, 1533);}
        s.b[1741] = (s.v[1534] > 0.005);s.store_scalar(1741, if s.b[1741] { 1.0 } else { 0.0 });s.b[1742] = ((((s.v[1529] + s.v[1528]) - s.v[1458]) - s.v[1356]) < 80.0);s.store_scalar(1742, if s.b[1742] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_104(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1608] && (!s.b[1734])) && (!s.b[1737])) && s.b[1741]) && s.b[1742]) {s.store_exp_ad(1344, A::add_scaled_inputs4(s.ad_value(1529), 1.0, s.ad_value(1528), 1.0, s.ad_value(1458), -1.0, s.ad_value(1356), -1.0));}
        if ((((s.b[1608] && (!s.b[1734])) && (!s.b[1737])) && s.b[1741]) && (!s.b[1742])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1344, A::add_scaled_inputs4(s.ad_value(1529), 1.0, s.ad_value(1528), 1.0, s.ad_value(1458), -1.0, s.ad_value(1356), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (((s.b[1608] && (!s.b[1734])) && (!s.b[1737])) && s.b[1741]) {s.store_div(1343, 1344, 1433);s.store_div_scaled_product_mixed_iia(1342, 1534, 1343, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0);}
        s.b[1743] = (s.v[1534] < (-0.005));s.store_scalar(1743, if s.b[1743] { 1.0 } else { 0.0 });
        if ((((s.b[1608] && (!s.b[1734])) && (!s.b[1737])) && (!s.b[1741])) && s.b[1743]) {s.store_sin_scaled_input(1343, 1356, 0.5);s.store_div_scaled_value_by_product_mixed_iai(1342, 1534, -1.0, A::square(s.ad_value(1343)), 1535, 1.0);}
        if ((((s.b[1608] && (!s.b[1734])) && (!s.b[1737])) && (!s.b[1741])) && (!s.b[1743])) {s.store_div_mixed_ai(1342, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1534), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1534), 1.0, A::scale(s.ad_value(1534), 0.0396825396825397), 0.05), 0.3333333333333)), 1535);}
        if ((s.b[1608] && (!s.b[1734])) && (!s.b[1737])) {s.store_offset_div_scaled_inputs2_mixed_iia(1531, 1532, 1.0, 1357, (-1.0), A::sub_from_scalar(1.0, s.ad_value(1342)), 1.0, 1e-80);s.store_sub(1533, 1531, 1532);s.store_div(1530, 1533, 1461);}
        s.b[1744] = (((s.v[1459] - s.v[1530]) - s.v[1528]) < 80.0);s.store_scalar(1744, if s.b[1744] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1744]) {s.store_exp_ad(1342, A::add_scaled_inputs3(s.ad_value(1459), 1.0, s.ad_value(1530), (-1.0), s.ad_value(1528), -1.0));}
        if (s.b[1608] && (!s.b[1744])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1342, A::add_scaled_inputs3(s.ad_value(1459), 1.0, s.ad_value(1530), (-1.0), s.ad_value(1528), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1608] {s.store_mul(1536, 1433, 1342);s.store_scalar(1539, 0.0);s.store_scalar(1540, 0.0);s.store_scalar(1537, 0.0);s.store_scalar(1538, 0.0);s.store_scalar(1541, 0.0);s.store_scalar(1542, 0.0);}
        s.b[1745] = (s.v[1466] > 1e-6);s.store_scalar(1745, if s.b[1745] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1745]) {s.store_mul(1537, 1535, 1434);s.store_mul(1538, 1536, 1435);s.store_add_scaled_inputs(1539, 1537, 1.0, 1532, 2.0);s.store_add_scaled_inputs(1540, 1538, 1.0, 1533, 2.0);s.store_add_scaled_inputs3_indices(1541, 1531, 2.0, 1537, 1.0, 1538, 1.0);}
        s.b[1746] = (((s.v[1534]) as f64).abs() > 0.005);s.store_scalar(1746, if s.b[1746] { 1.0 } else { 0.0 });
        if ((s.b[1608] && s.b[1745]) && s.b[1746]) {s.store_add_scaled_products3_mixed_iiaiai(2, 1539, 1540, 1.0, A::offset(s.ad_value(1529), 2.0), 1540, 2.0, A::offset(s.ad_value(1530), 2.0), 1539, 2.0);s.store_div_scaled_product_by_product_indices(1542, 1534, 1541, (-4.0), 1531, 2, 1.0);}
        if ((s.b[1608] && s.b[1745]) && (!s.b[1746])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2, 1534, 1.0, 1534, 1.0, 1534, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_add_scaled_products3_mixed_iiiiaa(3, 1539, 1535, 1.0, 1540, 1536, 1.0, A::mul3(s.ad_value(1539), s.ad_value(1540), s.ad_value(1531)), A::offset(A::mul(s.ad_value(1531), s.ad_value(2)), 1.0), 1.0);s.store_div_scaled_product3_by_product_indices(1542, 1535, 1536, 1541, 1.0, 1531, 3, 1.0);}
        if s.b[1608] {s.store_add_mixed_ia(1543, 1528, A::ln(s.ad_value(1531)));s.store_scaled_add(1544, 1466, 1531, 0.5);s.store_sub(1545, 1543, 1479);s.store_scalar(1548, 1.0);}
        s.b[1747] = (p.p9 > 0.0);s.store_scalar(1747, if s.b[1747] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_105(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1608] && s.b[1747]) {s.store_div_scaled_inputs2_indices(1546, 1467, 0.5, 1532, 0.5, 1460, 1.0);s.store_scaled_add_offset_sqrt_square_offset(1546, 1546, 1e-5, (-1e-5), 1.0, 0.5);s.store_sub_scaled_inputs_mixed_ai(1, A::sqrt(A::add_scaled_product(A::div(s.ad_value(1546), s.ad_value(227)), 1.0, s.ad_value(250), s.ad_value(250), 0.25)), 1.0, 250, 0.5);s.store_mul_square_lhs(1547, 1, 227);s.store_sub_from_scalar_div_indices(1548, 1.0, 1547, 1546);}
        s.b[1748] = ((s.v[1532] / 2.0) < 80.0);s.store_scalar(1748, if s.b[1748] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1748]) {s.store_ln_one_plus_exp_scaled_input(2, 1532, 0.5);}
        if (s.b[1608] && (!s.b[1748])) {s.store_scale(2, 1532, 0.5);}
        if s.b[1608] {s.store_scale(1549, 2, 2.0);}
        s.b[1749] = ((s.v[1533] / 2.0) < 80.0);s.store_scalar(1749, if s.b[1749] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1749]) {s.store_ln_one_plus_exp_scaled_input(3, 1533, 0.5);}
        if (s.b[1608] && (!s.b[1749])) {s.store_scale(3, 1533, 0.5);}
        if s.b[1608] {s.store_scale(1550, 3, 2.0);s.store_sub(1551, 1550, 1533);s.store_sub(1552, 1549, 1532);s.store_add_scaled_products_indices(1553, 270, 1549, 1.0, 271, 1551, 1.0);s.store_add_scaled_products_indices(1554, 270, 1550, 1.0, 271, 1552, 1.0);s.store_scaled_add(1555, 1480, 1549, 0.5);s.store_scaled_add(1556, 1481, 1550, 0.5);s.store_div_from_scalar_add_ad(0, 1.0, s.ad_value(1555), s.ad_value(1556));s.store_mul3_lhs(1557, 1544, 1555, 0);s.store_mul3_lhs(1558, 1544, 1556, 0);s.store_scaled_add(1559, 1482, 1551, 0.5);s.store_scaled_add(1560, 1483, 1552, 0.5);s.store_scaled_add(1561, 1484, 1553, 0.5);s.store_scaled_add(1562, 1485, 1554, 0.5);s.store_mul_product3_mixed_iiia(1563, 1548, 1555, 191, A::exp(A::mul(s.ad_value(40), s.ad_value(295))), 1.0);s.store_mul_ad_product_rhs_mixed_ia(1564, 1556, 192, A::exp(A::mul(s.ad_value(40), s.ad_value(295))));s.store_add(1565, 1563, 1564);s.store_mul_add_scaled_product_rhs_indices(2, 50, 1559, 1.0, 51, 1560, 1.0);s.store_scaled_add_sqrt_square_offset_ad(3, A::offset(s.ad_value(2), 1.0), 0.01, 0.5);s.store_scaled_add_sqrt_square_offset_ad(4, A::scale_offset(s.ad_value(2), 0.2, 1.0), 0.01, 0.5);s.store_div(1566, 3, 4);s.store_mul_ad_product_rhs(1567, 33, A::add_scaled_product(A::offset(A::mul(s.ad_value(41), s.ad_value(1559)), 1.0), 1.0, s.ad_value(42), s.ad_value(1560), 1.0), A::exp(A::mul_scaled_lhs(s.ad_value(44), -1.0, A::ln(A::add_scaled_product(A::offset(A::mul(s.ad_value(1557), s.ad_value(268)), 1.0), 1.0, s.ad_value(1558), s.ad_value(269), 1.0)))));}
        s.b[1750] = (s.v[56] == 0.0);s.store_scalar(1750, if s.b[1750] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1750]) {s.store_scalar(4, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_106(
        s: &mut ReactiveScratch,
    ) {
        s.b[1751] = (s.v[56] < 0.0);s.store_scalar(1751, if s.b[1751] { 1.0 } else { 0.0 });
        if ((s.b[1608] && (!s.b[1750])) && s.b[1751]) {s.store_mul_exp_mixed_ia(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(1544), 1e-12))));s.store_sub_from_scalar(4, 1.0, 2);}
        if ((s.b[1608] && (!s.b[1750])) && (!s.b[1751])) {s.store_mul_exp_mixed_ia(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(1544), 1e-12))));s.store_div_from_scalar_offset_input(4, 1.0, 2, 1.0);}
        if s.b[1608] {s.store_mul_add_scaled_product_rhs_indices(1568, 1492, 54, 1.0, 1544, 4, 1.0);s.store_add_scaled_inputs_product_mixed_aiii(1569, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(1561)), 1e-6)))), 1.0), 1.0, 1567, 1.0, 38, 1568, 1.0);s.store_add_scaled_inputs_product_mixed_aiii(1570, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(1562)), 1e-6)))), 1.0), 1.0, 1567, 1.0, 39, 1568, 1.0);s.store_div_scaled_product_add_scaled_denominator(1571, 1566, 1565, 1.0, A::div(s.ad_value(1563), s.ad_value(1569)), 1.0, A::div(s.ad_value(1564), s.ad_value(1570)), 1.0, 1.0);s.store_div_from_scalar_offset_input(1572, 1.0, 1544, 4.0);}
        s.b[1752] = (s.v[65] > 0.0);s.store_scalar(1752, if s.b[1752] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1752]) {s.store_div_from_scalar_offset_product(0, 1.0, 65, 1558, 1.0);}
        if (s.b[1608] && (!s.b[1752])) {s.store_sub_from_scalar_scaled_mul(0, 1.0, 65, 1558, 1.0);}
        if s.b[1608] {s.store_mul3_lhs(1573, 1544, 1572, 0);s.store_mul_ln_mixed_ia(1574, 1573, A::offset(A::div_scaled_inputs2(s.ad_value(339), 1.0, s.ad_value(1528), (-1.0), A::add_scaled_product(A::mul3(s.ad_value(67), s.ad_value(1544), s.ad_value(1544)), 1.0, s.ad_value(66), s.ad_value(227), 1.0), 1.0), 1.0));s.store_mul(1575, 1426, 1574);s.store_div_from_scalar_offset_ad(1576, 1.0, A::mul_offset_rhs(s.ad_value(1575), s.ad_value(1575), 1.0), 1.0);s.store_div_scaled_value_offset_denominator(1504, s.ad_value(1555), 100.0, s.ad_value(1555), 100.0, 1.0);}
        s.b[1753] = (s.v[61] < 0.0);s.store_scalar(1753, if s.b[1753] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1753]) {s.store_div_from_scalar_sub_from_scalar_ad(1505, 1.0, 1.0, A::mul(s.ad_value(61), s.ad_value(1504)));}
        if (s.b[1608] && (!s.b[1753])) {s.store_offset_mul(1505, 61, 1504, 1.0);}
        if s.b[1608] {s.store_div_scaled_value_offset_denominator(1506, s.ad_value(1556), 100.0, s.ad_value(1556), 100.0, 1.0);}
        s.b[1754] = (s.v[62] < 0.0);s.store_scalar(1754, if s.b[1754] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1754]) {s.store_div_from_scalar_sub_from_scalar_ad(1507, 1.0, 1.0, A::mul(s.ad_value(62), s.ad_value(1506)));}
        if (s.b[1608] && (!s.b[1754])) {s.store_offset_mul(1507, 62, 1506, 1.0);}
        if s.b[1608] {s.store_mul_ad_affine_product_rhs(1577, 1424, s.ad_value(1545), A::add(s.ad_value(1505), s.ad_value(1507)), 0.5, 0.0);s.store_div_scaled_value_by_product_indices(1578, 1577, 1.0, 1571, 1576, 1.0);s.store_square(1579, 1578);s.store_sqrt_offset_input(1580, 1579, 1.0);s.store_div_scaled_offset_numerator_indices(1581, 1579, 1.5, 1.0, 1580, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_107(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1755] = (p.p13 > 0.0);s.store_scalar(1755, if s.b[1755] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1755]) {s.store_mul_scaled_exp_ln_offset_square_rhs(2, 258, 0.6, 1555, 60.0, (-0.1666666666667));s.store_mul_scaled_exp_ln_offset_square_rhs(3, 258, 0.6, 1556, 60.0, (-0.1666666666667));s.store_div_scaled_offset_numerator_mixed_ai(1582, A::mul(s.ad_value(1460), s.ad_value(2)), 1.0, 1.0, 1441, 1.0);s.store_div_scaled_offset_numerator_mixed_ai(1583, A::mul(s.ad_value(1461), s.ad_value(3)), 1.0, 1.0, 1442, 1.0);}
        if (s.b[1608] && (!s.b[1755])) {s.store_scalar(1582, 1.0);s.store_scalar(1583, 1.0);}
        s.b[1756] = (s.v[1466] > 1e-6);s.store_scalar(1756, if s.b[1756] { 1.0 } else { 0.0 });s.b[1757] = (s.v[1531] > 1e-6);s.store_scalar(1757, if s.b[1757] { 1.0 } else { 0.0 });s.b[1758] = (((s.v[1540]) as f64).abs() < 0.01);s.store_scalar(1758, if s.b[1758] { 1.0 } else { 0.0 });
        if (((s.b[1608] && s.b[1756]) && s.b[1757]) && s.b[1758]) {s.store_div_scaled_inputs2_by_product_mixed_aiai(0, A::offset(s.ad_value(1529), 2.0), 1.0, 1539, 0.5, A::offset(s.ad_value(1530), 2.0), 1539, 1.0);s.store_mul(2, 0, 1540);s.store_square(3, 2);s.store_add_mixed_ai(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);s.store_add_scaled_product_indices(5, 4, 1.0, 2, 3, (-1.0));s.store_div_scaled_inputs2_mixed_iaa(2, 1533, 1.0, A::mul3_scaled_output(s.ad_value(1534), A::sub(s.ad_value(0), A::div_from_scalar(1.0, s.ad_value(1539))), s.ad_value(5), 2.0), (-1.0), A::offset(s.ad_value(1530), 2.0), 1.0);s.store_div_scaled_inputs2_mixed_aii(1584, A::div_scaled_add_product(s.ad_value(1535), (-1.0), s.ad_value(1542), s.ad_value(1531), 1.0, s.ad_value(1539), 1.0), 1.0, 2, (-1.0), 1531, 1.0);s.store_div_scaled_product_offset_denominator_indices(1585, 1584, 1531, 1.0, 1584, 1.0, 1.0);}
        if (((s.b[1608] && s.b[1756]) && s.b[1757]) && (!s.b[1758])) {s.store_sub_ad(1584, A::div_scaled_product_by_product(s.ad_value(1542), s.ad_value(1541), 1.0, s.ad_value(1539), s.ad_value(1540), 1.0), A::div_scaled_inputs2(A::div(s.ad_value(1535), s.ad_value(1539)), 1.0, A::div(s.ad_value(1536), s.ad_value(1540)), 1.0, s.ad_value(1531), 1.0));s.store_div_scaled_product_offset_denominator_indices(1585, 1584, 1531, 1.0, 1584, 1.0, 1.0);}
        if ((s.b[1608] && s.b[1756]) && (!s.b[1757])) {s.copy_ad(1585, 1502);}
        if (s.b[1608] && s.b[1756]) {s.store_sub(2, 1585, 1509);s.store_offset_scaled_mul(3, 2, 2, 36.0, 1.0);}
        s.b[1759] = (((s.v[2]) as f64).abs() > 0.001);s.store_scalar(1759, if s.b[1759] { 1.0 } else { 0.0 });
        if ((s.b[1608] && s.b[1756]) && s.b[1759]) {s.store_sub(4, 1531, 1466);s.store_add_scaled_product_indices(1586, 4, 1.0, 1585, 1545, (-1.0));s.store_add_scaled_product_indices(1587, 4, 1.0, 1509, 1545, (-1.0));s.store_sqrt_square_add(1588, 1586, 3);s.store_sqrt_square_add(1589, 1587, 3);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1590, 0.25, 2, A::add_scaled_products3(s.ad_value(1589), s.ad_value(1586), 1.0, s.ad_value(1588), s.ad_value(1587), (-1.0), s.ad_value(3), A::ln(A::div_scaled_inputs2(s.ad_value(1587), 1.0, s.ad_value(1589), 1.0, A::add(s.ad_value(1586), s.ad_value(1588)), 1.0)), 1.0));}
        if ((s.b[1608] && s.b[1756]) && (!s.b[1759])) {s.store_mul(4, 1545, 2);s.store_div_scaled_product3_mixed_iiia(1590, 1545, 4, 4, ((-0.25) * 0.1666666666667), A::sqrt(s.ad_value(3)), 1.0);}
        if (s.b[1608] && (!s.b[1756])) {s.copy_ad(1585, 1502);s.store_scalar(1590, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_108(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1608] {s.store_add_scaled_inputs3_mixed_aii(1591, A::add_scaled_product(s.ad_value(1590), 1.0, s.ad_value(1544), s.ad_value(1545), 1.0), 1.0, 1466, 1.0, 1531, -1.0);}
        s.b[1760] = (s.v[1466] > 1e-6);s.store_scalar(1760, if s.b[1760] { 1.0 } else { 0.0 });s.b[1761] = (s.v[1591] > 1e-30);s.store_scalar(1761, if s.b[1761] { 1.0 } else { 0.0 });
        if ((s.b[1608] && s.b[1760]) && s.b[1761]) {s.store_div_add_scaled_inputs_rhs_mixed_ai(1592, 1475, A::div(s.ad_value(1471), s.ad_value(1466)), 1.0, 1478, -1.0);s.store_div_add_scaled_inputs_rhs_mixed_ai(1593, 1539, A::div(s.ad_value(1535), s.ad_value(1531)), 1.0, 1542, -1.0);s.store_div_scaled_inputs2_indices(1594, 1592, 1.0, 1593, (-1.0), 1591, 1.0);s.store_div_add_scaled_inputs_rhs_mixed_ai(1595, 1476, A::div(s.ad_value(1472), s.ad_value(1466)), 1.0, 1478, -1.0);s.store_div_add_scaled_inputs_rhs_mixed_ai(1596, 1540, A::div(s.ad_value(1536), s.ad_value(1531)), 1.0, 1542, -1.0);s.store_div_scaled_inputs2_indices(1597, 1595, 1.0, 1596, (-1.0), 1591, 1.0);}
        if ((s.b[1608] && s.b[1760]) && (!s.b[1761])) {s.store_scalar(1594, 0.0);s.store_scalar(1597, 0.0);}
        if (s.b[1608] && (!s.b[1760])) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(1598, 1497, A::div(s.ad_value(1434), s.ad_value(1500)), (-2.0), 1503, (-2.0));s.store_mul_add_scaled_inputs_rhs_mixed_ai(1599, 1498, A::div(s.ad_value(1435), s.ad_value(1501)), (-2.0), 1503, (-2.0));s.store_mul_sub_lhs(0, 1599, 1598, 1503);s.store_mul(2, 1598, 1434);s.store_mul(3, 1599, 1435);s.store_add(4, 2, 3);s.store_offset_ad(5, A::add_scaled_products(s.ad_value(1497), s.ad_value(1434), 2.0, s.ad_value(1498), s.ad_value(1435), 2.0), 3.0);s.store_div_scaled_inputs3_mixed_iiai(1600, 3, 1.0, 0, 1.0, A::div(s.ad_value(4), s.ad_value(1500)), -1.0, 5, 1.0);s.store_div_scaled_inputs3_mixed_iiai(1601, 2, 1.0, 0, (-1.0), A::div(s.ad_value(4), s.ad_value(1501)), -1.0, 5, 1.0);s.store_mul_add_scaled_product_rhs_indices(1594, 1500, 1503, -1.0, 1600, 1500, -1.0);s.store_mul_add_scaled_product_rhs_indices(1597, 1501, 1503, -1.0, 1601, 1501, -1.0);}
        if s.b[1608] {s.store_mul(1602, 1594, 1581);s.store_mul(1603, 1597, 1581);s.store_scaled_sub(1604, 1532, 1467, 0.5);s.store_scaled_sub(1605, 1533, 1468, 0.5);s.store_mul(1606, 1604, 1602);s.store_mul(1607, 1605, 1603);s.copy_ad(440, 1428);s.copy_ad(441, 1432);s.copy_ad(442, 1433);s.copy_ad(443, 1434);s.copy_ad(444, 1435);s.copy_ad(445, 1462);s.copy_ad(446, 1463);s.copy_ad(447, 1447);s.copy_ad(448, 1446);s.copy_ad(449, 1450);s.copy_ad(450, 1451);s.copy_ad(451, 1452);s.copy_ad(452, 1453);s.copy_ad(453, 1454);s.copy_ad(454, 1457);s.copy_ad(455, 1459);s.copy_ad(456, 1460);s.copy_ad(457, 1461);s.copy_ad(458, 1467);s.copy_ad(459, 1468);s.copy_ad(460, 1479);s.copy_ad(461, 1532);s.copy_ad(462, 1533);s.copy_ad(463, 1543);s.copy_ad(464, 1544);s.copy_ad(465, 1548);s.copy_ad(466, 1557);s.copy_ad(467, 1558);s.copy_ad(468, 1579);s.copy_ad(469, 1582);s.copy_ad(470, 1583);s.copy_ad(471, 1604);s.copy_ad(472, 1605);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_109(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1608] {s.copy_ad(473, 1606);s.copy_ad(474, 1607);}
        if (!s.b[1608]) {s.copy_ad(440, 383);s.copy_ad(441, 384);s.copy_ad(442, 385);s.copy_ad(443, 386);s.copy_ad(444, 387);s.copy_ad(445, 388);s.copy_ad(446, 389);s.copy_ad(447, 390);s.copy_ad(448, 391);s.copy_ad(449, 393);s.copy_ad(450, 394);s.copy_ad(451, 395);s.copy_ad(452, 396);s.copy_ad(453, 397);s.copy_ad(454, 398);s.copy_ad(455, 399);s.copy_ad(456, 401);s.copy_ad(457, 402);s.copy_ad(458, 404);s.copy_ad(459, 405);s.copy_ad(460, 406);s.copy_ad(461, 408);s.copy_ad(462, 409);s.copy_ad(463, 414);s.copy_ad(464, 415);s.copy_ad(465, 416);s.copy_ad(466, 419);s.copy_ad(467, 420);s.copy_ad(468, 428);s.copy_ad(469, 430);s.copy_ad(470, 431);s.copy_ad(471, 436);s.copy_ad(472, 437);s.copy_ad(473, 438);s.copy_ad(474, 439);}
        s.store_div_scaled_product_mixed_iaa(0, 120, A::sub(s.ad_value(448), s.ad_value(446)), 1.0, A::scale_offset(s.ad_value(464), 0.25, 1.0), 1.0);s.store_add_scaled_inputs3_indices(1324, 458, 0.5, 461, 0.5, 0, 1.0);s.store_add_scaled_inputs3_indices(1325, 459, 0.5, 462, 0.5, 0, -1.0);s.b[1762] = (p.p13 > 0.0);s.store_scalar(1762, if s.b[1762] { 1.0 } else { 0.0 });
        if s.b[1762] {s.store_add_scaled_inputs3_mixed_iai(1326, 1324, 1.0, A::div(s.ad_value(466), s.ad_value(469)), 1.0, 466, -1.0);s.store_add_scaled_inputs3_mixed_iai(1327, 1325, 1.0, A::div(s.ad_value(467), s.ad_value(470)), 1.0, 467, -1.0);}
        if (!s.b[1762]) {s.copy_ad(1326, 1324);s.copy_ad(1327, 1325);}
        s.store_scaled_mul(2, 471, 473, 0.3333333333333);s.store_mul_scaled_offset_ad_rhs(3, 471, 0.1666666666667, A::mul_sub_from_scalar_rhs(s.ad_value(473), 1.0, A::scale(s.ad_value(473), 0.2)), 1.0);s.store_add_scaled_product_indices(1328, 3, 1.0, 1326, 465, 0.5);s.store_add_scaled_product_indices(1326, 2, 1.0, 1326, 465, 1.0);s.store_scaled_mul(2, 472, 474, 0.3333333333333);s.store_mul_scaled_offset_ad_rhs(3, 472, 0.1666666666667, A::mul_sub_from_scalar_rhs(s.ad_value(474), 1.0, A::scale(s.ad_value(474), 0.2)), 1.0);s.store_add_scaled_inputs(1329, 1327, 0.5, 3, 1.0);s.store_add(1327, 1327, 2);s.store_mul(0, 447, 287);s.store_mul(361, 0, 1326);s.store_mul(362, 0, 1327);s.store_mul_add_scaled_inputs_rhs_indices(363, 0, 1328, -1.0, 1329, -1.0);s.b[1763] = (s.v[119] > 0.0);s.store_scalar(1763, if s.b[1763] { 1.0 } else { 0.0 });
        if s.b[1763] {s.store_offset(0, 254, (2.0 * 0.6931471805599));s.store_add(1330, 460, 0);s.store_add(1331, 463, 0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1332, 1330, 0.5, 254, 0.5, 1330, 254, 9.0, (-0.5));s.store_add_scaled_inputs4_mixed_iiia(1333, 1331, 0.5, 254, 0.5, 339, 0.5, A::sqrt_square_offset(A::add_scaled_inputs3(s.ad_value(1331), 1.0, s.ad_value(254), -1.0, s.ad_value(339), -1.0), 9.0), (-0.5));s.store_mul_sqrt_mixed_ia(1334, 294, A::mul_offset_rhs(s.ad_value(445), s.ad_value(444), 0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_110(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1763] {s.store_mul_sqrt_mixed_ia(1335, 294, A::mul_offset_rhs(A::mul3(s.ad_value(445), s.ad_value(456), s.ad_value(444)), s.ad_value(443), 0.5));s.store_mul_square_lhs(1336, 1334, 291);s.store_mul_square_lhs(1337, 1335, 291);s.store_sub(2, 292, 1332);s.store_add_scaled_inputs3_indices(3, 292, 1.0, 339, 1.0, 1333, -1.0);s.store_scale(0, 1336, 2.0);s.store_add_scaled_offset_product_rhs_mixed_iia(1338, 1332, 1.0, 0, A::sqrt(A::offset(A::div(s.ad_value(2), s.ad_value(1336)), 1.0)), (-1.0), 1.0);s.store_add_scaled_offset_product_rhs_mixed_iia(1339, 1333, 1.0, 0, A::sqrt(A::offset(A::div(s.ad_value(3), s.ad_value(1336)), 1.0)), (-1.0), 1.0);s.store_scale(0, 1337, 2.0);s.store_add_scaled_offset_product_rhs_mixed_iia(1340, 1332, 1.0, 0, A::sqrt(A::offset(A::div(s.ad_value(2), s.ad_value(1337)), 1.0)), (-1.0), 1.0);s.store_add_scaled_offset_product_rhs_mixed_iia(1341, 1333, 1.0, 0, A::sqrt(A::offset(A::div(s.ad_value(3), s.ad_value(1337)), 1.0)), (-1.0), 1.0);s.store_mul(0, 293, 447);s.store_mul_product3_indices(2, 451, 0, 1334, 456, -1.0);s.store_mul_product3_indices(3, 452, 0, 1335, 457, -1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(0, 1338, 0.5, 1330, ((-1.0) * 0.5), 1338, 1330, 1.0, 0.5);s.store_div_scaled_product3_mixed_iiia(379, 2, 0, 0, 1.0, A::sub(s.ad_value(1338), s.ad_value(1332)), 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(0, 1339, 0.5, 1331, ((-1.0) * 0.5), 1339, 1331, 1.0, 0.5);s.store_div_scaled_product3_mixed_iiia(380, 2, 0, 0, 1.0, A::sub(s.ad_value(1339), s.ad_value(1333)), 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(0, 1340, 0.5, 1330, ((-1.0) * 0.5), 1340, 1330, 1.0, 0.5);s.store_div_scaled_product3_mixed_iiia(381, 3, 0, 0, 1.0, A::sub(s.ad_value(1340), s.ad_value(1332)), 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(0, 1341, 0.5, 1331, ((-1.0) * 0.5), 1341, 1331, 1.0, 0.5);s.store_div_scaled_product3_mixed_iiia(382, 3, 0, 0, 1.0, A::sub(s.ad_value(1341), s.ad_value(1333)), 1.0);}
        if (!s.b[1763]) {s.store_scalar(379, 0.0);s.store_scalar(380, 0.0);s.store_scalar(381, 0.0);s.store_scalar(382, 0.0);}
        s.store_mul(370, 164, 330);s.store_mul(371, 165, 332);s.store_scaled_add_sqrt_square_offset_ad(0, A::sub_from_scalar(1.0, A::mul3(s.ad_value(161), s.ad_value(449), A::sub_from_scalar(1.0, A::mul(s.ad_value(162), s.ad_value(440))))), 0.2, 0.5);s.store_mul3_lhs(372, 159, 349, 0);s.store_mul3_lhs(373, 160, 350, 0);s.store_mul(374, 117, 338);s.store_mul(375, 166, 336);s.store_mul_scale_offset_mixed_ia(377, 331, A::add_scaled_products(s.ad_value(240), s.ad_value(9), 1.0, s.ad_value(167), s.ad_value(11), 1.0), -1.0, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_111(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_mul_scale_offset_mixed_ia(376, 333, A::add_scaled_products(s.ad_value(240), s.ad_value(10), 1.0, s.ad_value(167), s.ad_value(12), 1.0), -1.0, 0.0);s.b[1764] = (s.v[6] > 0.0);s.store_scalar(1764, if s.b[1764] { 1.0 } else { 0.0 });
        if s.b[1764] {s.store_mul(378, 170, 219);}
        if (!s.b[1764]) {s.store_scalar(378, 0.0);}
        s.copy_ad(1774, 361);s.copy_ad(1775, 362);s.copy_ad(1776, 363);s.store_add_scaled_inputs3_indices(364, 361, (-1.0), 362, (-1.0), 363, (-1.0));s.b[1777] = (s.v[334] < 0.0);s.store_scalar(1777, if s.b[1777] { 1.0 } else { 0.0 });
        if s.b[1777] {s.copy_ad(1776, 364);}
        s.store_scaled_mul(361, 13, 361, p.p32);s.store_scaled_mul(362, 13, 362, p.p32);s.store_scaled_mul(363, 13, 363, p.p32);s.store_add_scaled_inputs3_indices(364, 361, (-1.0), 362, (-1.0), 363, (-1.0));s.store_scaled_mul(379, 13, 379, p.p32);s.store_scaled_mul(380, 13, 380, p.p32);s.store_scaled_mul(381, 13, 381, p.p32);s.store_scaled_mul(382, 13, 382, p.p32);s.store_scaled_mul(370, 13, 370, p.p32);s.store_scaled_mul(371, 13, 371, p.p32);s.store_scaled_mul(372, 13, 372, p.p32);s.store_scaled_mul(373, 13, 373, p.p32);s.store_scaled_mul(374, 13, 374, p.p32);s.store_scaled_mul(377, 13, 377, p.p32);s.store_scaled_mul(376, 13, 376, p.p32);s.store_scaled_mul(375, 13, 375, p.p32);s.store_mul(378, 13, 378);s.b[1778] = (s.v[334] < 0.0);s.store_scalar(1778, if s.b[1778] { 1.0 } else { 0.0 });
        if s.b[1778] {s.copy_ad(1772, 363);s.copy_ad(363, 364);s.copy_ad(364, 1772);s.store_neg(375, 375);s.copy_ad(1772, 380);s.copy_ad(380, 379);s.copy_ad(379, 1772);s.copy_ad(1772, 382);s.copy_ad(382, 381);s.copy_ad(381, 1772);}
        s.b[1779] = (s.v[13] > 0.0);s.store_scalar(1779, if s.b[1779] { 1.0 } else { 0.0 });
        if s.b[1779] {s.store_mul_div_scaled_inputs_mixed_aia(1773, A::add_scaled_product(A::div_scaled_product_by_product(s.ad_value(179), A::add(s.ad_value(1774), s.ad_value(1775)), 1.0, s.ad_value(116), s.ad_value(239), 1.0), 1.0, s.ad_value(180), s.ad_value(226), 1.0), 342, 1e-9, A::mul(s.ad_value(345), s.ad_value(116)), 1.0);}
        if (!s.b[1779]) {s.store_scalar(1773, 0.0);}
        s.store_scaled_mul(1780, 390, 226, 1.0 / (1.602176565e-19));s.store_scaled_add(1781, 407, 432, (-0.5));s.store_add(1782, 415, 1781);s.store_div(0, 415, 1782);s.store_scaled_add_mixed_ia(1787, 0, A::sqrt_square_offset(s.ad_value(0), 1e-20), 0.5);s.store_scaled_mul(1788, 436, 435, (-0.1666666666667));s.store_square(1789, 1788);s.store_offset(1790, 429, (-1.0));s.store_scale(1794, 1789, 12.0);s.store_add_scaled_inputs3_mixed_iia(2, 1787, 1.0, 1794, 1.0, A::mul3_scaled_output(A::offset(s.ad_value(1787), 1.0), s.ad_value(1794), s.ad_value(1790), 2.0), -1.0);s.store_max_with_scalar(3, 2, 1e-40);s.store_div_scaled_product3_indices(1799, 456, 447, 116, 1.0, 469, 1.0);s.store_mul_scale_offset_indices(1800, 1799, 468, 1.0, 1.0);s.store_mul_scale_offset_mixed_ia(1802, 1800, A::mul_scaled_lhs(s.ad_value(334), 0.25, s.ad_value(1788)), -1.0, 0.5);s.store_sub(1801, 1800, 1802);s.b[1813] = (p.p6 > 0.0);s.store_scalar(1813, if s.b[1813] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_112(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1813] {s.store_sub_ad(2, A::add_scaled_product(s.ad_value(1787), 0.08333333333333333, s.ad_value(1789), A::sub(A::offset(s.ad_value(1787), 0.2), s.ad_value(1794)), (-1.0)), A::mul3_scaled_output(s.ad_value(1789), A::sub(A::offset(s.ad_value(1787), 1.0), s.ad_value(1794)), s.ad_value(1790), 1.6));s.store_max_with_scalar(3, 2, 1e-40);}
        s.copy_ad(1783, 1780);s.store_mul_scale_offset_indices(1784, 1780, 415, 1.0, 1.0);s.store_mul_sub_rhs(1785, 1780, 403, 413);s.store_mul_add(2, A::ln(A::div_scaled_inputs2(s.ad_value(1784), 1.0, s.ad_value(1785), 0.5, A::sub_scaled_inputs(s.ad_value(1784), 1.0, s.ad_value(1785), 0.5), 1.0)), A::add_scaled_product(s.ad_value(173), 1.0, s.ad_value(174), s.ad_value(1783), (-1.0)), A::mul3(s.ad_value(175), s.ad_value(1783), s.ad_value(1783)));s.store_add_scaled_product_mixed_iai(3, 2, 1.0, A::add_scaled_product(s.ad_value(174), 1.0, s.ad_value(175), A::sub_scaled_inputs(s.ad_value(1784), 1.0, s.ad_value(1783), 2.0), 1.0), 1785, 1.0);s.store_offset_div_ad(0, A::add_scaled_products(s.ad_value(176), s.ad_value(417), 1.0, s.ad_value(177), s.ad_value(418), 1.0), A::offset(s.ad_value(415), 1.0), 1.0);s.store_scaled_add_offset_sqrt_square_offset(4, 0, 0.01, (-0.01), 0.0001, 0.5);s.store_mul_div_scaled_product_mixed_iaii(0, 4, A::div_scaled_product(s.ad_value(347), s.ad_value(348), 1.602176565e-19, s.ad_value(345), 1.0), 3, 1.0, 1783, 1.0);s.store_div_from_scalar_scaled_input(1823, 1.0, 8, 8.617332384961e-5);s.store_sub_from_scalar_ad(1824, 1.17, A::div_scaled_product_offset_denominator(s.ad_value(8), s.ad_value(8), 0.000473, s.ad_value(8), 636.0, 1.0));s.store_sub_from_scalar_ad(1825, 0.744, A::div_scaled_product_offset_denominator(s.ad_value(8), s.ad_value(8), 0.0004774, s.ad_value(8), 235.0, 1.0));s.store_mul_add_scaled_inputs3_offset_rhs_indices(1826, 15, 1825, 1.0, 1824, (-1.0), 228, (-0.4), 0.0);s.store_add(1827, 1824, 1826);s.store_scaled_mul(1828, 1827, 1823, 0.5);s.store_sub_scaled_inputs(1829, 15, 0.05, 1826, 0.5);s.store_sqrt_scaled_input(0, 8, 0.0033333333333);s.store_mul3_affine_lhs(2, 0, 0, 4.05e25, 0.0, 0);s.store_mul(1830, 2, 238);s.store_div_scaled_value_offset_denominator(1831, s.ad_value(1823), 1.0, A::div_scaled_inputs(s.ad_value(17), s.v[7], s.ad_value(8), 1.0), 1.0, 1.0);s.store_mul3_affine_lhs(1833, 1830, 229, (2.0 * 1.602176565e-19), 0.0, 1831);s.store_add_offset_lhs_mixed_ai(1834, A::ln(A::div_scaled_product(s.ad_value(245), s.ad_value(245), 1.0, s.ad_value(1833), 1.0)), (-0.6931471805599), 1828);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_113(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_mul_div_scaled_product_mixed_iiia(1835, 1831, 29, 14, (0.5 * 1.602176565e-19), A::add(s.ad_value(241), s.ad_value(242)), 1.0);s.store_mul(1838, 35, 1831);s.store_scalar(1839, 0.0);s.store_scalar(1832, 0.0);s.b[1884] = (p.p9 > 0.0);s.store_scalar(1884, if s.b[1884] { 1.0 } else { 0.0 });
        if s.b[1884] {s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1832, 1.0, 1823, A::ln(A::div(s.ad_value(24), s.ad_value(251))));}
        s.b[1885] = (p.p13 > 0.0);s.store_scalar(1885, if s.b[1885] { 1.0 } else { 0.0 });s.b[1886] = (p.p14 == 1.0);s.store_scalar(1886, if s.b[1886] { 1.0 } else { 0.0 });
        if (s.b[1885] && s.b[1886]) {s.store_scale_ad(1839, A::exp_scaled_input(A::ln(A::div(s.ad_value(259), s.ad_value(1831))), (-0.3333333333333)), ((0.4 * p.p13) * 1.27520989));}
        if (s.b[1885] && (!s.b[1886])) {s.store_scale_ad(1839, A::exp_scaled_input(A::ln(A::div(s.ad_value(259), s.ad_value(1831))), (-0.3333333333333)), ((0.4 * p.p13) * 1.5412087));}
        s.store_mul(1842, 336, 1831);s.store_mul_scale_offset_mixed_ia(1843, 1831, A::sqrt_square_offset(s.ad_value(336), 0.01), 1.0, (-0.1));s.store_scaled_sub(1844, 1842, 1843, 0.5);s.store_div_scaled_value_by_product_mixed_iia(1815, 402, 1.0, 401, A::offset(s.ad_value(402), 1.0), 1.0);s.store_div_scaled_value_by_product_mixed_iia(1816, 401, 1.0, 402, A::offset(s.ad_value(401), 1.0), 1.0);s.store_offset_ln_ad(1817, A::div_scaled_product3(s.ad_value(401), A::offset(s.ad_value(1815), 1.0), s.ad_value(384), 1.0, s.ad_value(385), 1.0), 2.0);s.store_offset_ln_ad(1818, A::div_scaled_product3(s.ad_value(402), A::offset(s.ad_value(1816), 1.0), s.ad_value(384), 1.0, s.ad_value(385), 1.0), 2.0);s.store_add_scaled_products_mixed_aiii(1819, A::offset(s.ad_value(1815), 1.0), 1817, 1.0, 399, 1815, (-1.0));s.store_add_scaled_offset_product_lhs_mixed_aai(1820, A::div(s.ad_value(399), s.ad_value(1816)), (-1.0), A::div_from_scalar(1.0, s.ad_value(1816)), 1.0, 1818, 1.0);s.store_add_mixed_ai(1821, A::div_scaled_inputs4(s.ad_value(1819), 0.5, s.ad_value(1820), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1819), s.ad_value(1820)), 38.0), (-0.5), s.ad_value(398), -1.0, s.ad_value(25), 1.0), 398);s.store_add_scaled_product_mixed_iia(1822, 21, 1.0, 226, A::add_scaled_inputs3(A::div_scaled_inputs2(s.ad_value(1821), 1.0, s.ad_value(394), (-1.0), s.ad_value(395), 1.0), 1.0, s.ad_value(397), (-1.0), s.ad_value(394), 1.0), 1.0);s.store_mul_scale_offset_indices(0, 34, 8, 1.0, (-s.v[7]));s.store_add_scaled_offset_product_rhs(0, 256, 1.0, 23, 8, (-s.v[7]), p.p14);s.store_sub_offset_lhs_mixed_ai(1840, A::add_scaled_inputs4(s.ad_value(183), p.p14, s.ad_value(1829), p.p14, s.ad_value(243), p.p14, s.ad_value(0), 1.0), p.p34, 1832);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_114(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_add_scaled_inputs4_indices(1841, 184, p.p14, 1829, p.p14, 244, p.p14, 0, 1.0);s.store_add_scaled_product_mixed_iai(1845, 1844, (-1.0), A::sub(s.ad_value(1822), s.ad_value(1840)), 1831, 1.0);s.store_add_scaled_product_mixed_iai(1846, 1844, (-1.0), A::sub_scaled_inputs(s.ad_value(337), -1.0, s.ad_value(1841), 1.0), 1831, 1.0);s.b[1887] = (p.p2 > 0.0);s.store_scalar(1887, if s.b[1887] { 1.0 } else { 0.0 });
        if s.b[1887] {s.store_div_scaled_product_mixed_iai(0, 16, A::sub(s.ad_value(1845), s.ad_value(1846)), p.p14, 260, 1.0);}
        s.b[1888] = (s.v[0] < 0.0);s.store_scalar(1888, if s.b[1888] { 1.0 } else { 0.0 });
        if (s.b[1887] && s.b[1888]) {s.store_scaled_ln_ad(2, A::sub_from_scalar(1.0, s.ad_value(0)), (-2.0));}
        if (s.b[1887] && (!s.b[1888])) {s.store_div_scaled_product_offset_denominator_mixed_iia(2, 0, 0, 1.0, A::div_scaled_inputs(s.ad_value(0), 2.0, s.ad_value(260), 1.0), 1.0, 1.0);}
        if s.b[1887] {s.store_add_scaled_product_indices(1847, 1846, 1.0, 16, 2, p.p14);}
        if (!s.b[1887]) {s.copy_ad(1847, 1846);}
        s.store_mul_sub_rhs(0, 248, 1845, 1847);s.b[1889] = (p.p13 > 0.0);s.store_scalar(1889, if s.b[1889] { 1.0 } else { 0.0 });
        if s.b[1889] {s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1848, 0, 0.5, 257, 0.5, A::add(A::square(A::sub(s.ad_value(0), s.ad_value(257))), A::square(s.ad_value(257))), 0.5);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1849, 257, 0.5, 0, ((-1.0) * 0.5), A::add(A::square(A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(257), 1.0)), A::square(s.ad_value(257))), 0.5);s.store_mul_mixed_ia(2, 1839, A::exp_scaled_input(A::ln(s.ad_value(1848)), (-0.3333333333333)));s.store_mul_mixed_ia(3, 1839, A::exp_scaled_input(A::ln(s.ad_value(1849)), (-0.3333333333333)));s.store_sub_mixed_ai(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);s.store_div_scaled_product_offset_denominator_mixed_iia(1851, 246, 4, 1.0, A::mul(s.ad_value(246), s.ad_value(2)), 1.0, 1.0);s.store_div_scaled_product_offset_denominator_mixed_iia(1852, 247, 4, 1.0, A::mul(s.ad_value(247), s.ad_value(3)), 1.0, 1.0);s.store_div_from_scalar_add_ad(1853, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(1851)), 1.0), A::div_from_scalar(1.0, s.ad_value(1852)));}
        if (!s.b[1889]) {s.copy_ad(1851, 246);s.copy_ad(1852, 247);s.copy_ad(1853, 248);}
        s.store_mul_sub_rhs(1854, 1853, 1845, 1847);s.b[1890] = (s.v[1854] > 0.0);s.store_scalar(1890, if s.b[1890] { 1.0 } else { 0.0 });s.b[1891] = ((-s.v[1854]) < 80.0);s.store_scalar(1891, if s.b[1891] { 1.0 } else { 0.0 });
        if (s.b[1890] && s.b[1891]) {s.store_ln_one_plus_exp_neg_input(0, 1854);}
        if (s.b[1890] && (!s.b[1891])) {s.store_neg(0, 1854);}
        if s.b[1890] {s.store_add_scaled_inputs3_offset_mixed_iai(1855, 1845, 1.0, A::div(s.ad_value(1854), s.ad_value(1851)), (-1.0), 0, 1.0, (-0.6931471805599));}
        s.b[1892] = (s.v[1854] < 80.0);s.store_scalar(1892, if s.b[1892] { 1.0 } else { 0.0 });
        if ((!s.b[1890]) && s.b[1892]) {s.store_ln_one_plus_exp(0, 1854);}
        if ((!s.b[1890]) && (!s.b[1892])) {s.copy_ad(0, 1854);}
        if (!s.b[1890]) {s.store_add_scaled_inputs3_offset_mixed_iai(1855, 1847, 1.0, A::div(s.ad_value(1854), s.ad_value(1852)), 1.0, 0, 1.0, (-0.6931471805599));}
        s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1856, 1855, 0.5, 1834, 0.5, 1855, 1834, 4.0, (-0.5));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_115(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_offset_sqrt_ad(1857, A::offset(A::div_scaled_inputs2(s.ad_value(1834), 2.0, s.ad_value(1856), (-2.0), s.ad_value(1835), 1.0), 1.0), (-1.0));s.store_scaled_add_offset_sqrt_square_offset_ad(0, A::mul(s.ad_value(30), s.ad_value(1846)), ((1.0) + (0.5)), ((1.0) + ((-0.5))), 0.01, 0.5);s.store_mul_scale_offset(0, A::mul3_scaled_output(s.ad_value(1838), A::offset(A::sqrt(A::offset(A::div(s.ad_value(1843), s.ad_value(1838)), 1.0)), (-1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(1857)), 1.0), 2.0), A::mul(s.ad_value(37), s.ad_value(1846)), 1.0, 1.0);s.b[1894] = (p.p11 > 0.0);s.store_scalar(1894, if s.b[1894] { 1.0 } else { 0.0 });
        if s.b[1894] {s.store_div_scaled_value_by_product_mixed_iia(1815, 457, 1.0, 456, A::offset(s.ad_value(457), 1.0), 1.0);s.store_div_scaled_value_by_product_mixed_iia(1816, 456, 1.0, 457, A::offset(s.ad_value(456), 1.0), 1.0);s.store_offset_ln_ad(1817, A::div_scaled_product3(s.ad_value(456), A::offset(s.ad_value(1815), 1.0), s.ad_value(441), 1.0, s.ad_value(442), 1.0), 2.0);s.store_offset_ln_ad(1818, A::div_scaled_product3(s.ad_value(457), A::offset(s.ad_value(1816), 1.0), s.ad_value(441), 1.0, s.ad_value(442), 1.0), 2.0);s.store_add_scaled_products_mixed_aiii(1819, A::offset(s.ad_value(1815), 1.0), 1817, 1.0, 455, 1815, (-1.0));s.store_add_scaled_offset_product_lhs_mixed_aai(1820, A::div(s.ad_value(455), s.ad_value(1816)), (-1.0), A::div_from_scalar(1.0, s.ad_value(1816)), 1.0, 1818, 1.0);s.store_add_mixed_ai(1821, A::div_scaled_inputs4(s.ad_value(1819), 0.5, s.ad_value(1820), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1819), s.ad_value(1820)), 38.0), (-0.5), s.ad_value(454), -1.0, s.ad_value(25), 1.0), 454);s.store_add_scaled_product_mixed_iia(1822, 130, 1.0, 226, A::add_scaled_inputs3(A::div_scaled_inputs2(s.ad_value(1821), 1.0, s.ad_value(450), (-1.0), s.ad_value(451), 1.0), 1.0, s.ad_value(453), (-1.0), s.ad_value(450), 1.0), 1.0);s.store_mul_scale_offset_indices(0, 34, 8, 1.0, (-s.v[7]));s.store_add_scaled_offset_product_rhs(0, 256, 1.0, 23, 8, (-s.v[7]), p.p14);s.store_sub_offset_lhs_mixed_ai(1840, A::add_scaled_inputs4(s.ad_value(185), p.p14, s.ad_value(1829), p.p14, s.ad_value(243), p.p14, s.ad_value(0), 1.0), p.p34, 1832);s.store_add_scaled_inputs4_indices(1841, 186, p.p14, 1829, p.p14, 244, p.p14, 0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_116(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1894] {s.store_add_scaled_product_mixed_iai(1845, 1844, (-1.0), A::sub(s.ad_value(1822), s.ad_value(1840)), 1831, 1.0);s.store_add_scaled_product_mixed_iai(1846, 1844, (-1.0), A::sub_scaled_inputs(s.ad_value(337), -1.0, s.ad_value(1841), 1.0), 1831, 1.0);}
        s.b[1895] = (p.p2 > 0.0);s.store_scalar(1895, if s.b[1895] { 1.0 } else { 0.0 });
        if (s.b[1894] && s.b[1895]) {s.store_div_scaled_product_mixed_iai(0, 16, A::sub(s.ad_value(1845), s.ad_value(1846)), p.p14, 260, 1.0);}
        s.b[1896] = (s.v[0] < 0.0);s.store_scalar(1896, if s.b[1896] { 1.0 } else { 0.0 });
        if ((s.b[1894] && s.b[1895]) && s.b[1896]) {s.store_scaled_ln_ad(2, A::sub_from_scalar(1.0, s.ad_value(0)), (-2.0));}
        if ((s.b[1894] && s.b[1895]) && (!s.b[1896])) {s.store_div_scaled_product_offset_denominator_mixed_iia(2, 0, 0, 1.0, A::div_scaled_inputs(s.ad_value(0), 2.0, s.ad_value(260), 1.0), 1.0, 1.0);}
        if (s.b[1894] && s.b[1895]) {s.store_add_scaled_product_indices(1847, 1846, 1.0, 16, 2, p.p14);}
        if (s.b[1894] && (!s.b[1895])) {s.copy_ad(1847, 1846);}
        if s.b[1894] {s.store_mul_sub_rhs(0, 248, 1845, 1847);}
        s.b[1897] = (p.p13 > 0.0);s.store_scalar(1897, if s.b[1897] { 1.0 } else { 0.0 });
        if (s.b[1894] && s.b[1897]) {s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1848, 0, 0.5, 257, 0.5, A::add(A::square(A::sub(s.ad_value(0), s.ad_value(257))), A::square(s.ad_value(257))), 0.5);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1849, 257, 0.5, 0, ((-1.0) * 0.5), A::add(A::square(A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(257), 1.0)), A::square(s.ad_value(257))), 0.5);s.store_mul_mixed_ia(2, 1839, A::exp_scaled_input(A::ln(s.ad_value(1848)), (-0.3333333333333)));s.store_mul_mixed_ia(3, 1839, A::exp_scaled_input(A::ln(s.ad_value(1849)), (-0.3333333333333)));s.store_sub_mixed_ai(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);s.store_div_scaled_product_offset_denominator_mixed_iia(1851, 246, 4, 1.0, A::mul(s.ad_value(246), s.ad_value(2)), 1.0, 1.0);s.store_div_scaled_product_offset_denominator_mixed_iia(1852, 247, 4, 1.0, A::mul(s.ad_value(247), s.ad_value(3)), 1.0, 1.0);s.store_div_from_scalar_add_ad(1853, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(1851)), 1.0), A::div_from_scalar(1.0, s.ad_value(1852)));}
        if (s.b[1894] && (!s.b[1897])) {s.copy_ad(1851, 246);s.copy_ad(1852, 247);s.copy_ad(1853, 248);}
        if s.b[1894] {s.store_mul_sub_rhs(1854, 1853, 1845, 1847);}
        s.b[1898] = (s.v[1854] > 0.0);s.store_scalar(1898, if s.b[1898] { 1.0 } else { 0.0 });s.b[1899] = ((-s.v[1854]) < 80.0);s.store_scalar(1899, if s.b[1899] { 1.0 } else { 0.0 });
        if ((s.b[1894] && s.b[1898]) && s.b[1899]) {s.store_ln_one_plus_exp_neg_input(0, 1854);}
        if ((s.b[1894] && s.b[1898]) && (!s.b[1899])) {s.store_neg(0, 1854);}
        if (s.b[1894] && s.b[1898]) {s.store_add_scaled_inputs3_offset_mixed_iai(1855, 1845, 1.0, A::div(s.ad_value(1854), s.ad_value(1851)), (-1.0), 0, 1.0, (-0.6931471805599));}
        s.b[1900] = (s.v[1854] < 80.0);s.store_scalar(1900, if s.b[1900] { 1.0 } else { 0.0 });
        if ((s.b[1894] && (!s.b[1898])) && s.b[1900]) {s.store_ln_one_plus_exp(0, 1854);}
        if ((s.b[1894] && (!s.b[1898])) && (!s.b[1900])) {s.copy_ad(0, 1854);}
        if (s.b[1894] && (!s.b[1898])) {s.store_add_scaled_inputs3_offset_mixed_iai(1855, 1847, 1.0, A::div(s.ad_value(1854), s.ad_value(1852)), 1.0, 0, 1.0, (-0.6931471805599));}
        if s.b[1894] {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1856, 1855, 0.5, 1834, 0.5, 1855, 1834, 4.0, (-0.5));}
    }
}
