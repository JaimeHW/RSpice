#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_80(
        s: &mut Scratch,
    ) {
        if ((s.b[1608] && (!s.b[1631])) && s.b[1632]) {s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);s.store_mul_div_lhs(1363, 1354, 1353, 1343);s.store_div_mixed_ai(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);}
        if ((s.b[1608] && (!s.b[1631])) && (!s.b[1632])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1344, 1353, 1.0, 1353, 1.0, 1353, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(1357, 1353, 1344, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1342, 1353, 1.0, 1353, 1.0, 1353, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(1358, 1354, 1342);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1343, 1353, 1.0, 1353, 1.0, 1353, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(1360, 1355, 1342, 1.0, A::square(s.ad_value(1354)), 1343, (-1.0));s.store_scaled_mul(1363, 1354, 1344, (-0.5));s.store_add_scaled_product_mixed_aii(1364, A::mul3_scaled_output(s.ad_value(1354), s.ad_value(1354), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 2.0, A::scale(s.ad_value(1353), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1355, 1344, (-0.5));}
        s.b[1633] = (s.v[1353] > 0.005);s.store_scalar(1633, if s.b[1633] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1633]) {s.store_div_scaled_inputs_mixed_ia(1343, 1353, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0);s.store_mul(1361, 1343, 1359);s.store_sub_ln_lhs(1362, 1343, 1356);}
        s.b[1634] = (s.v[1353] < (-0.005));s.store_scalar(1634, if s.b[1634] { 1.0 } else { 0.0 });
        if ((s.b[1608] && (!s.b[1633])) && s.b[1634]) {s.store_sin_scaled_input(1343, 1356, 0.5);s.store_div_scaled_inputs_square_rhs(1361, 1353, -1.0, 1343, 1.0);s.store_ln(1362, 1361);}
        if ((s.b[1608] && (!s.b[1633])) && (!s.b[1634])) {s.store_sub_from_scalar_ad(1361, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(1362, 1361);}
        s.b[1635] = (((1.01 * s.v[1351]) + s.v[1357]) > 0.0);s.store_scalar(1635, if s.b[1635] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1635]) {s.store_add(1365, 1351, 1357);s.store_add(1366, 1460, 1358);s.copy_ad(1367, 1360);}
        if (s.b[1608] && (!s.b[1635])) {s.store_div_from_scalar_sub_ad(1343, 1.0, s.ad_value(1351), s.ad_value(1357));s.store_sub(1344, 1358, 1460);s.store_mul_sub_lhs(1365, 1352, 1361, 1343);s.store_mul_mixed_ai(1366, A::add_scaled_value_products(s.ad_value(1352), (-1.0), s.ad_value(1344), s.ad_value(1365), 1.0, s.ad_value(1363), s.ad_value(1361), (-1.0)), 1343);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_81(
        s: &mut Scratch,
    ) {
        if (s.b[1608] && (!s.b[1635])) {s.store_mul_mixed_ai(1367, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1360), s.ad_value(1365), 1.0, s.ad_value(1344), s.ad_value(1366), 2.0), 1.0, s.ad_value(1352), 1.0, A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361), (-1.0)), 1343);}
        s.b[1636] = (s.v[1365] > 0.0);s.store_scalar(1636, if s.b[1636] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1636]) {s.store_ln(1368, 1365);s.store_div_from_scalar(1342, 1.0, 1365);s.store_mul(1369, 1366, 1342);s.store_add_scaled_square_product_indices(1370, 1369, (-1.0), 1367, 1342, 1.0);}
        if (s.b[1608] && (!s.b[1636])) {s.store_add_offset_lhs_mixed_ia(1368, 1351, 0.6931471805599, A::ln_scaled_input(s.ad_value(1351), -1.0));s.store_div_from_scalar(1342, 1.0, 1465);s.store_add(1369, 1460, 1342);s.store_mul_scale_offset_indices(1370, 1342, 1342, -1.0, 0.0);}
        if s.b[1608] {s.store_sub_add_scaled_inputs4_lhs_indices(1371, 1459, 1.0, 1458, (-1.0), 1465, 1.0, 1368, 2.0, 1362);s.store_sub_mixed_ai(1372, A::scale_offset(s.ad_value(1369), 2.0, 1.0), 1363);s.store_sub_scaled_inputs(1373, 1370, 2.0, 1364, 1.0);s.store_add_scaled_product_indices(1374, 1351, 1.0, 1461, 1371, 1.0);s.store_add_scaled_product_indices(1375, 1460, 1.0, 1461, 1372, 1.0);s.store_mul(1376, 1461, 1373);s.store_add_scaled_product_indices(1377, 1352, (-1.0), 1374, 1365, 1.0);s.store_add_mixed_ai(1378, A::add_scaled_products(s.ad_value(1375), s.ad_value(1365), 1.0, s.ad_value(1374), s.ad_value(1366), 1.0), 1352);s.store_sub_mixed_ai(1379, A::add_scaled_products3(s.ad_value(1376), s.ad_value(1365), 1.0, s.ad_value(1375), s.ad_value(1366), 2.0, s.ad_value(1374), s.ad_value(1367), 1.0), 1352);s.store_add_scaled_square_product_indices(1388, 1378, 1.0, 1377, 1379, (-0.5));s.store_div_scaled_product3_mixed_iiia(1380, 1377, 1378, 1388, -1.0, A::offset(A::square(s.ad_value(1388)), 1e-200), 1.0);s.store_add(1465, 1465, 1380);s.store_mul(1351, 1460, 1465);s.store_mul(1381, 1461, 1469);s.store_add(1374, 1351, 1381);s.store_offset_scaled(1382, 1374, 0.065345483024, 1.0);s.store_add_scaled_product_mixed_aii(1383, A::scale_offset(s.ad_value(1374), 8.5797362674, 39.478417604), 1.0, 1351, 1381, 1.0);s.store_add_scaled_product_indices(1384, 1374, (2.0 * 39.478417604), 1351, 1381, 39.478417604);s.store_sqrt_add_scaled_square_product(1385, 1383, 1.0, 1382, 1384, (-4.0));s.store_div_scaled_inputs2_indices(1353, 1385, 1.0, 1383, (-1.0), 1382, 2.0);s.store_sub_square_lhs(1386, 1351, 1353);}
        s.b[1637] = (s.v[1386] > 0.0);s.store_scalar(1637, if s.b[1637] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1637]) {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_aii(1377, 1386, A::ln(A::div(s.ad_value(1386), s.ad_value(1433))), 1.0, 1458, (-1.0), 1465, 1.0, 0.0);s.store_add_scaled_product_indices(1378, 1386, 1.0, 1460, 1351, 2.0);}
        let (t2,) = {
    if (s.b[1608] && s.b[1637]) {
        let t0: f64 = (s.v[1458] - s.v[1465]);let t1: f64 = (t0 - s.v[1345]);
        (t1,)
    } else {
        (s.v[1387],)
    }
};
        s.store_scalar(1387, t2);s.b[1638] = ((((s.v[1377] < 0.0) && (s.v[1378] > 0.0)) && (((s.v[1387] + 2.3025850929941) + ((s.v[1460]) as f64).ln()) > 0.0)) || (s.v[1387] > 1.0));s.store_scalar(1638, if s.b[1638] { 1.0 } else { 0.0 });
        if ((s.b[1608] && s.b[1637]) && s.b[1638]) {s.store_sub_div_rhs_indices(1465, 1465, 1377, 1378);}
        if s.b[1608] {s.store_mul(1351, 1460, 1465);s.store_mul(1381, 1461, 1469);s.store_add(1374, 1351, 1381);s.store_offset_scaled(1382, 1374, 0.065345483024, 1.0);s.store_add_scaled_product_mixed_aii(1383, A::scale_offset(s.ad_value(1374), 8.5797362674, 39.478417604), 1.0, 1351, 1381, 1.0);s.store_add_scaled_product_indices(1384, 1374, (2.0 * 39.478417604), 1351, 1381, 39.478417604);s.store_sqrt_add_scaled_square_product(1385, 1383, 1.0, 1382, 1384, (-4.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_82(
        s: &mut Scratch,
    ) {
        if s.b[1608] {s.store_div_scaled_inputs2_indices(1353, 1385, 1.0, 1383, (-1.0), 1382, 2.0);}
        s.b[1639] = (s.v[1353] < (-0.005));s.store_scalar(1639, if s.b[1639] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1639]) {s.store_sqrt_abs_ad(1356, s.ad_value(1353));s.store_div_mixed_ia(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));s.store_div_scaled_inputs2_mixed_iai(1358, 1353, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 0.25, 1353, 1.0);}
        s.b[1640] = (s.v[1353] > 0.005);s.store_scalar(1640, if s.b[1640] { 1.0 } else { 0.0 });
        if ((s.b[1608] && (!s.b[1639])) && s.b[1640]) {s.store_sqrt_abs_ad(1356, s.ad_value(1353));s.store_exp_neg_input(1359, 1356);s.store_div_scaled_product_offset_rhs_mixed_iia(1357, 1356, 1359, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0);s.store_div_scaled_inputs2_mixed_iai(1358, 1353, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 0.25, 1353, 1.0);}
        if ((s.b[1608] && (!s.b[1639])) && (!s.b[1640])) {s.store_offset_ad(1357, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1358, 1353, 1.0, 1353, 1.0, 1353, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);}
        if s.b[1608] {s.store_sub_mixed_ia(1353, 1353, A::div_scaled_inputs2(A::add_scaled_products(s.ad_value(1374), s.ad_value(1357), 1.0, s.ad_value(1351), s.ad_value(1381), 1.0), 1.0, s.ad_value(1353), 1.0, A::offset(A::mul(s.ad_value(1374), s.ad_value(1358)), 1.0), 1.0));s.store_sub_square_lhs(1386, 1351, 1353);}
        s.b[1641] = (s.v[1386] > 0.0);s.store_scalar(1641, if s.b[1641] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1641]) {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_aii(1377, 1386, A::ln(A::div(s.ad_value(1386), s.ad_value(1433))), 1.0, 1458, (-1.0), 1465, 1.0, 0.0);s.store_add_scaled_product_indices(1378, 1386, 1.0, 1460, 1351, 2.0);}
        let (t5,) = {
    if (s.b[1608] && s.b[1641]) {
        let t3: f64 = (s.v[1458] - s.v[1465]);let t4: f64 = (t3 - s.v[1345]);
        (t4,)
    } else {
        (s.v[1387],)
    }
};
        s.store_scalar(1387, t5);s.b[1642] = ((((s.v[1377] < 0.0) && (s.v[1378] > 0.0)) && (((s.v[1387] + 2.3025850929941) + ((s.v[1460]) as f64).ln()) > 0.0)) || (s.v[1387] > 1.0));s.store_scalar(1642, if s.b[1642] { 1.0 } else { 0.0 });
        if ((s.b[1608] && s.b[1641]) && s.b[1642]) {s.store_sub_div_rhs_indices(1465, 1465, 1377, 1378);}
        if s.b[1608] {s.store_mul(1351, 1460, 1465);}
        s.b[1643] = ((s.v[1458] - s.v[1465]) < 80.0);s.store_scalar(1643, if s.b[1643] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1643]) {s.store_exp_sub(1342, 1458, 1465);}
        if (s.b[1608] && (!s.b[1643])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1342, A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1608] {s.store_mul(1352, 1433, 1342);s.store_sub_square_lhs(1353, 1351, 1352);s.store_add_scaled_product_indices(1354, 1352, 1.0, 1460, 1351, 2.0);s.store_add_scaled_product_indices(1355, 1352, (-1.0), 1460, 1460, 2.0);}
        s.b[1644] = (s.v[1353] < (-0.005));s.store_scalar(1644, if s.b[1644] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1644]) {s.store_sqrt_abs_ad(1356, s.ad_value(1353));s.store_div_mixed_ia(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);s.store_mul_add_mixed_iia(1358, 1342, 1353, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_83(
        s: &mut Scratch,
    ) {
        if (s.b[1608] && s.b[1644]) {s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);s.store_mul_div_lhs(1363, 1354, 1353, 1343);s.store_div_mixed_ai(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);}
        s.b[1645] = (s.v[1353] > 0.005);s.store_scalar(1645, if s.b[1645] { 1.0 } else { 0.0 });
        if ((s.b[1608] && (!s.b[1644])) && s.b[1645]) {s.store_sqrt_abs_ad(1356, s.ad_value(1353));s.store_exp_neg_input(1359, 1356);s.store_div_scaled_product_offset_rhs_mixed_iia(1357, 1356, 1359, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0);s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);s.store_mul_add_mixed_iia(1358, 1342, 1353, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)));s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);s.store_mul_div_lhs(1363, 1354, 1353, 1343);s.store_div_mixed_ai(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);}
        if ((s.b[1608] && (!s.b[1644])) && (!s.b[1645])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1344, 1353, 1.0, 1353, 1.0, 1353, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(1357, 1353, 1344, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1342, 1353, 1.0, 1353, 1.0, 1353, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(1358, 1354, 1342);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1343, 1353, 1.0, 1353, 1.0, 1353, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(1360, 1355, 1342, 1.0, A::square(s.ad_value(1354)), 1343, (-1.0));s.store_scaled_mul(1363, 1354, 1344, (-0.5));s.store_add_scaled_product_mixed_aii(1364, A::mul3_scaled_output(s.ad_value(1354), s.ad_value(1354), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 2.0, A::scale(s.ad_value(1353), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1355, 1344, (-0.5));}
        s.b[1646] = (s.v[1353] > 0.005);s.store_scalar(1646, if s.b[1646] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1646]) {s.store_div_scaled_inputs_mixed_ia(1343, 1353, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0);s.store_mul(1361, 1343, 1359);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_84(
        s: &mut Scratch,
    ) {
        if (s.b[1608] && s.b[1646]) {s.store_sub_ln_lhs(1362, 1343, 1356);}
        s.b[1647] = (s.v[1353] < (-0.005));s.store_scalar(1647, if s.b[1647] { 1.0 } else { 0.0 });
        if ((s.b[1608] && (!s.b[1646])) && s.b[1647]) {s.store_sin_scaled_input(1343, 1356, 0.5);s.store_div_scaled_inputs_square_rhs(1361, 1353, -1.0, 1343, 1.0);s.store_ln(1362, 1361);}
        if ((s.b[1608] && (!s.b[1646])) && (!s.b[1647])) {s.store_sub_from_scalar_ad(1361, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(1362, 1361);}
        s.b[1648] = (((1.01 * s.v[1351]) + s.v[1357]) > 0.0);s.store_scalar(1648, if s.b[1648] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1648]) {s.store_add(1365, 1351, 1357);s.store_add(1366, 1460, 1358);s.copy_ad(1367, 1360);}
        if (s.b[1608] && (!s.b[1648])) {s.store_div_from_scalar_sub_ad(1343, 1.0, s.ad_value(1351), s.ad_value(1357));s.store_sub(1344, 1358, 1460);s.store_mul_sub_lhs(1365, 1352, 1361, 1343);s.store_mul_mixed_ai(1366, A::add_scaled_value_products(s.ad_value(1352), (-1.0), s.ad_value(1344), s.ad_value(1365), 1.0, s.ad_value(1363), s.ad_value(1361), (-1.0)), 1343);s.store_mul_mixed_ai(1367, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1360), s.ad_value(1365), 1.0, s.ad_value(1344), s.ad_value(1366), 2.0), 1.0, s.ad_value(1352), 1.0, A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361), (-1.0)), 1343);}
        s.b[1649] = (s.v[1365] > 0.0);s.store_scalar(1649, if s.b[1649] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1649]) {s.store_ln(1368, 1365);s.store_div_from_scalar(1342, 1.0, 1365);s.store_mul(1369, 1366, 1342);s.store_add_scaled_square_product_indices(1370, 1369, (-1.0), 1367, 1342, 1.0);}
        if (s.b[1608] && (!s.b[1649])) {s.store_add_offset_lhs_mixed_ia(1368, 1351, 0.6931471805599, A::ln_scaled_input(s.ad_value(1351), -1.0));s.store_div_from_scalar(1342, 1.0, 1465);s.store_add(1369, 1460, 1342);s.store_mul_scale_offset_indices(1370, 1342, 1342, -1.0, 0.0);}
        if s.b[1608] {s.store_sub_add_scaled_inputs4_lhs_indices(1371, 1459, 1.0, 1458, (-1.0), 1465, 1.0, 1368, 2.0, 1362);s.store_sub_mixed_ai(1372, A::scale_offset(s.ad_value(1369), 2.0, 1.0), 1363);s.store_sub_scaled_inputs(1373, 1370, 2.0, 1364, 1.0);s.store_add_scaled_product_indices(1374, 1351, 1.0, 1461, 1371, 1.0);s.store_add_scaled_product_indices(1375, 1460, 1.0, 1461, 1372, 1.0);s.store_mul(1376, 1461, 1373);s.store_add_scaled_product_indices(1377, 1352, (-1.0), 1374, 1365, 1.0);s.store_add_mixed_ai(1378, A::add_scaled_products(s.ad_value(1375), s.ad_value(1365), 1.0, s.ad_value(1374), s.ad_value(1366), 1.0), 1352);s.store_sub_mixed_ai(1379, A::add_scaled_products3(s.ad_value(1376), s.ad_value(1365), 1.0, s.ad_value(1375), s.ad_value(1366), 2.0, s.ad_value(1374), s.ad_value(1367), 1.0), 1352);s.store_add_scaled_square_product_indices(1388, 1378, 1.0, 1377, 1379, (-0.5));s.store_div_scaled_product3_mixed_iiia(1380, 1377, 1378, 1388, -1.0, A::offset(A::square(s.ad_value(1388)), 1e-200), 1.0);s.store_add(1465, 1465, 1380);s.store_mul(1351, 1460, 1465);}
        s.b[1650] = ((s.v[1458] - s.v[1465]) < 80.0);s.store_scalar(1650, if s.b[1650] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1650]) {s.store_exp_sub(1342, 1458, 1465);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_85(
        s: &mut Scratch,
    ) {
        if (s.b[1608] && (!s.b[1650])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1342, A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1608] {s.store_mul(1352, 1433, 1342);s.store_sub_square_lhs(1353, 1351, 1352);s.store_add_scaled_product_indices(1354, 1352, 1.0, 1460, 1351, 2.0);s.store_add_scaled_product_indices(1355, 1352, (-1.0), 1460, 1460, 2.0);}
        s.b[1651] = (s.v[1353] < (-0.005));s.store_scalar(1651, if s.b[1651] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1651]) {s.store_sqrt_abs_ad(1356, s.ad_value(1353));s.store_div_mixed_ia(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);s.store_mul_add_mixed_iia(1358, 1342, 1353, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)));s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);s.store_mul_div_lhs(1363, 1354, 1353, 1343);s.store_div_mixed_ai(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);}
        s.b[1652] = (s.v[1353] > 0.005);s.store_scalar(1652, if s.b[1652] { 1.0 } else { 0.0 });
        if ((s.b[1608] && (!s.b[1651])) && s.b[1652]) {s.store_sqrt_abs_ad(1356, s.ad_value(1353));s.store_exp_neg_input(1359, 1356);s.store_div_scaled_product_offset_rhs_mixed_iia(1357, 1356, 1359, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0);s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);s.store_mul_add_mixed_iia(1358, 1342, 1353, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)));s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);s.store_mul_div_lhs(1363, 1354, 1353, 1343);s.store_div_mixed_ai(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);}
        if ((s.b[1608] && (!s.b[1651])) && (!s.b[1652])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1344, 1353, 1.0, 1353, 1.0, 1353, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(1357, 1353, 1344, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1342, 1353, 1.0, 1353, 1.0, 1353, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(1358, 1354, 1342);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_86(
        s: &mut Scratch,
    ) {
        if ((s.b[1608] && (!s.b[1651])) && (!s.b[1652])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1343, 1353, 1.0, 1353, 1.0, 1353, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(1360, 1355, 1342, 1.0, A::square(s.ad_value(1354)), 1343, (-1.0));s.store_scaled_mul(1363, 1354, 1344, (-0.5));s.store_add_scaled_product_mixed_aii(1364, A::mul3_scaled_output(s.ad_value(1354), s.ad_value(1354), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 2.0, A::scale(s.ad_value(1353), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1355, 1344, (-0.5));}
        s.b[1653] = (s.v[1353] > 0.005);s.store_scalar(1653, if s.b[1653] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1653]) {s.store_div_scaled_inputs_mixed_ia(1343, 1353, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0);s.store_mul(1361, 1343, 1359);s.store_sub_ln_lhs(1362, 1343, 1356);}
        s.b[1654] = (s.v[1353] < (-0.005));s.store_scalar(1654, if s.b[1654] { 1.0 } else { 0.0 });
        if ((s.b[1608] && (!s.b[1653])) && s.b[1654]) {s.store_sin_scaled_input(1343, 1356, 0.5);s.store_div_scaled_inputs_square_rhs(1361, 1353, -1.0, 1343, 1.0);s.store_ln(1362, 1361);}
        if ((s.b[1608] && (!s.b[1653])) && (!s.b[1654])) {s.store_sub_from_scalar_ad(1361, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(1362, 1361);}
        s.b[1655] = (((1.01 * s.v[1351]) + s.v[1357]) > 0.0);s.store_scalar(1655, if s.b[1655] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1655]) {s.store_add(1365, 1351, 1357);s.store_add(1366, 1460, 1358);s.copy_ad(1367, 1360);}
        if (s.b[1608] && (!s.b[1655])) {s.store_div_from_scalar_sub_ad(1343, 1.0, s.ad_value(1351), s.ad_value(1357));s.store_sub(1344, 1358, 1460);s.store_mul_sub_lhs(1365, 1352, 1361, 1343);s.store_mul_mixed_ai(1366, A::add_scaled_value_products(s.ad_value(1352), (-1.0), s.ad_value(1344), s.ad_value(1365), 1.0, s.ad_value(1363), s.ad_value(1361), (-1.0)), 1343);s.store_mul_mixed_ai(1367, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1360), s.ad_value(1365), 1.0, s.ad_value(1344), s.ad_value(1366), 2.0), 1.0, s.ad_value(1352), 1.0, A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361), (-1.0)), 1343);}
        s.b[1656] = (s.v[1365] > 0.0);s.store_scalar(1656, if s.b[1656] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1656]) {s.store_ln(1368, 1365);s.store_div_from_scalar(1342, 1.0, 1365);s.store_mul(1369, 1366, 1342);s.store_add_scaled_square_product_indices(1370, 1369, (-1.0), 1367, 1342, 1.0);}
        if (s.b[1608] && (!s.b[1656])) {s.store_add_offset_lhs_mixed_ia(1368, 1351, 0.6931471805599, A::ln_scaled_input(s.ad_value(1351), -1.0));s.store_div_from_scalar(1342, 1.0, 1465);s.store_add(1369, 1460, 1342);s.store_mul_scale_offset_indices(1370, 1342, 1342, -1.0, 0.0);}
        if s.b[1608] {s.store_sub_add_scaled_inputs4_lhs_indices(1371, 1459, 1.0, 1458, (-1.0), 1465, 1.0, 1368, 2.0, 1362);s.store_sub_mixed_ai(1372, A::scale_offset(s.ad_value(1369), 2.0, 1.0), 1363);s.store_sub_scaled_inputs(1373, 1370, 2.0, 1364, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_87(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1608] {s.store_add_scaled_product_indices(1374, 1351, 1.0, 1461, 1371, 1.0);s.store_add_scaled_product_indices(1375, 1460, 1.0, 1461, 1372, 1.0);s.store_mul(1376, 1461, 1373);s.store_add_scaled_product_indices(1377, 1352, (-1.0), 1374, 1365, 1.0);s.store_add_mixed_ai(1378, A::add_scaled_products(s.ad_value(1375), s.ad_value(1365), 1.0, s.ad_value(1374), s.ad_value(1366), 1.0), 1352);s.store_sub_mixed_ai(1379, A::add_scaled_products3(s.ad_value(1376), s.ad_value(1365), 1.0, s.ad_value(1375), s.ad_value(1366), 2.0, s.ad_value(1374), s.ad_value(1367), 1.0), 1352);s.store_add_scaled_square_product_indices(1388, 1378, 1.0, 1377, 1379, (-0.5));s.store_div_scaled_product3_mixed_iiia(1380, 1377, 1378, 1388, -1.0, A::offset(A::square(s.ad_value(1388)), 1e-200), 1.0);s.store_add(1465, 1465, 1380);}
        s.b[1657] = (p.p10 == 1.0);s.store_scalar(1657, if s.b[1657] { 1.0 } else { 0.0 });s.b[1658] = (((s.v[1380]) as f64).abs() > 0.01);s.store_scalar(1658, if s.b[1658] { 1.0 } else { 0.0 });
        if ((s.b[1608] && s.b[1657]) && s.b[1658]) {s.store_mul(1351, 1460, 1465);}
        s.b[1659] = ((s.v[1458] - s.v[1465]) < 80.0);s.store_scalar(1659, if s.b[1659] { 1.0 } else { 0.0 });
        if (((s.b[1608] && s.b[1657]) && s.b[1658]) && s.b[1659]) {s.store_exp_sub(1342, 1458, 1465);}
        if (((s.b[1608] && s.b[1657]) && s.b[1658]) && (!s.b[1659])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1342, A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1608] && s.b[1657]) && s.b[1658]) {s.store_mul(1352, 1433, 1342);s.store_sub_square_lhs(1353, 1351, 1352);s.store_add_scaled_product_indices(1354, 1352, 1.0, 1460, 1351, 2.0);s.store_add_scaled_product_indices(1355, 1352, (-1.0), 1460, 1460, 2.0);}
        s.b[1660] = (s.v[1353] < (-0.005));s.store_scalar(1660, if s.b[1660] { 1.0 } else { 0.0 });
        if (((s.b[1608] && s.b[1657]) && s.b[1658]) && s.b[1660]) {s.store_sqrt_abs_ad(1356, s.ad_value(1353));s.store_div_mixed_ia(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);s.store_mul_add_mixed_iia(1358, 1342, 1353, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)));s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);s.store_mul_div_lhs(1363, 1354, 1353, 1343);s.store_div_mixed_ai(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);}
        s.b[1661] = (s.v[1353] > 0.005);s.store_scalar(1661, if s.b[1661] { 1.0 } else { 0.0 });
        if ((((s.b[1608] && s.b[1657]) && s.b[1658]) && (!s.b[1660])) && s.b[1661]) {s.store_sqrt_abs_ad(1356, s.ad_value(1353));s.store_exp_neg_input(1359, 1356);s.store_div_scaled_product_offset_rhs_mixed_iia(1357, 1356, 1359, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0);s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);s.store_mul_add_mixed_iia(1358, 1342, 1353, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)));s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_88(
        s: &mut Scratch,
    ) {
        if ((((s.b[1608] && s.b[1657]) && s.b[1658]) && (!s.b[1660])) && s.b[1661]) {s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);s.store_mul_div_lhs(1363, 1354, 1353, 1343);s.store_div_mixed_ai(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);}
        if ((((s.b[1608] && s.b[1657]) && s.b[1658]) && (!s.b[1660])) && (!s.b[1661])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1344, 1353, 1.0, 1353, 1.0, 1353, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(1357, 1353, 1344, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1342, 1353, 1.0, 1353, 1.0, 1353, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(1358, 1354, 1342);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1343, 1353, 1.0, 1353, 1.0, 1353, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(1360, 1355, 1342, 1.0, A::square(s.ad_value(1354)), 1343, (-1.0));s.store_scaled_mul(1363, 1354, 1344, (-0.5));s.store_add_scaled_product_mixed_aii(1364, A::mul3_scaled_output(s.ad_value(1354), s.ad_value(1354), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 2.0, A::scale(s.ad_value(1353), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1355, 1344, (-0.5));}
        s.b[1662] = (s.v[1353] > 0.005);s.store_scalar(1662, if s.b[1662] { 1.0 } else { 0.0 });
        if (((s.b[1608] && s.b[1657]) && s.b[1658]) && s.b[1662]) {s.store_div_scaled_inputs_mixed_ia(1343, 1353, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0);s.store_mul(1361, 1343, 1359);s.store_sub_ln_lhs(1362, 1343, 1356);}
        s.b[1663] = (s.v[1353] < (-0.005));s.store_scalar(1663, if s.b[1663] { 1.0 } else { 0.0 });
        if ((((s.b[1608] && s.b[1657]) && s.b[1658]) && (!s.b[1662])) && s.b[1663]) {s.store_sin_scaled_input(1343, 1356, 0.5);s.store_div_scaled_inputs_square_rhs(1361, 1353, -1.0, 1343, 1.0);s.store_ln(1362, 1361);}
        if ((((s.b[1608] && s.b[1657]) && s.b[1658]) && (!s.b[1662])) && (!s.b[1663])) {s.store_sub_from_scalar_ad(1361, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(1362, 1361);}
        s.b[1664] = (((1.01 * s.v[1351]) + s.v[1357]) > 0.0);s.store_scalar(1664, if s.b[1664] { 1.0 } else { 0.0 });
        if (((s.b[1608] && s.b[1657]) && s.b[1658]) && s.b[1664]) {s.store_add(1365, 1351, 1357);s.store_add(1366, 1460, 1358);s.copy_ad(1367, 1360);}
        if (((s.b[1608] && s.b[1657]) && s.b[1658]) && (!s.b[1664])) {s.store_div_from_scalar_sub_ad(1343, 1.0, s.ad_value(1351), s.ad_value(1357));s.store_sub(1344, 1358, 1460);s.store_mul_sub_lhs(1365, 1352, 1361, 1343);s.store_mul_mixed_ai(1366, A::add_scaled_value_products(s.ad_value(1352), (-1.0), s.ad_value(1344), s.ad_value(1365), 1.0, s.ad_value(1363), s.ad_value(1361), (-1.0)), 1343);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_89(
        s: &mut Scratch,
    ) {
        if (((s.b[1608] && s.b[1657]) && s.b[1658]) && (!s.b[1664])) {s.store_mul_mixed_ai(1367, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1360), s.ad_value(1365), 1.0, s.ad_value(1344), s.ad_value(1366), 2.0), 1.0, s.ad_value(1352), 1.0, A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361), (-1.0)), 1343);}
        s.b[1665] = (s.v[1365] > 0.0);s.store_scalar(1665, if s.b[1665] { 1.0 } else { 0.0 });
        if (((s.b[1608] && s.b[1657]) && s.b[1658]) && s.b[1665]) {s.store_ln(1368, 1365);s.store_div_from_scalar(1342, 1.0, 1365);s.store_mul(1369, 1366, 1342);s.store_add_scaled_square_product_indices(1370, 1369, (-1.0), 1367, 1342, 1.0);}
        if (((s.b[1608] && s.b[1657]) && s.b[1658]) && (!s.b[1665])) {s.store_add_offset_lhs_mixed_ia(1368, 1351, 0.6931471805599, A::ln_scaled_input(s.ad_value(1351), -1.0));s.store_div_from_scalar(1342, 1.0, 1465);s.store_add(1369, 1460, 1342);s.store_mul_scale_offset_indices(1370, 1342, 1342, -1.0, 0.0);}
        if ((s.b[1608] && s.b[1657]) && s.b[1658]) {s.store_sub_add_scaled_inputs4_lhs_indices(1371, 1459, 1.0, 1458, (-1.0), 1465, 1.0, 1368, 2.0, 1362);s.store_sub_mixed_ai(1372, A::scale_offset(s.ad_value(1369), 2.0, 1.0), 1363);s.store_sub_scaled_inputs(1373, 1370, 2.0, 1364, 1.0);s.store_add_scaled_product_indices(1374, 1351, 1.0, 1461, 1371, 1.0);s.store_add_scaled_product_indices(1375, 1460, 1.0, 1461, 1372, 1.0);s.store_mul(1376, 1461, 1373);s.store_add_scaled_product_indices(1377, 1352, (-1.0), 1374, 1365, 1.0);s.store_add_mixed_ai(1378, A::add_scaled_products(s.ad_value(1375), s.ad_value(1365), 1.0, s.ad_value(1374), s.ad_value(1366), 1.0), 1352);s.store_sub_mixed_ai(1379, A::add_scaled_products3(s.ad_value(1376), s.ad_value(1365), 1.0, s.ad_value(1375), s.ad_value(1366), 2.0, s.ad_value(1374), s.ad_value(1367), 1.0), 1352);s.store_add_scaled_square_product_indices(1388, 1378, 1.0, 1377, 1379, (-0.5));s.store_div_scaled_product3_mixed_iiia(1380, 1377, 1378, 1388, -1.0, A::offset(A::square(s.ad_value(1388)), 1e-200), 1.0);s.store_add(1465, 1465, 1380);}
        if s.b[1608] {s.store_mul(1467, 1460, 1465);}
        s.b[1666] = ((s.v[1458] - s.v[1465]) < 80.0);s.store_scalar(1666, if s.b[1666] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1666]) {s.store_exp_sub(1342, 1458, 1465);}
        if (s.b[1608] && (!s.b[1666])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1342, A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1608] {s.store_mul(1471, 1433, 1342);s.store_sub_square_lhs(1470, 1467, 1471);}
        s.b[1667] = (s.v[1471] <= 0.0);s.store_scalar(1667, if s.b[1667] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1667]) {s.store_scalar(1466, 1e-80);s.store_sub(1468, 1466, 1467);s.store_div(1469, 1468, 1461);}
        s.b[1668] = (s.v[1470] < (-0.005));s.store_scalar(1668, if s.b[1668] { 1.0 } else { 0.0 });
        if ((s.b[1608] && (!s.b[1667])) && s.b[1668]) {s.store_sqrt_abs_ad(1356, s.ad_value(1470));s.store_div_mixed_ia(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));}
        s.b[1669] = (s.v[1470] > 0.005);s.store_scalar(1669, if s.b[1669] { 1.0 } else { 0.0 });
        if (((s.b[1608] && (!s.b[1667])) && (!s.b[1668])) && s.b[1669]) {s.store_sqrt_abs_ad(1356, s.ad_value(1470));s.store_exp_neg_input(1359, 1356);s.store_div_scaled_product_offset_rhs_mixed_iia(1357, 1356, 1359, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_90(
        s: &mut Scratch,
    ) {
        if (((s.b[1608] && (!s.b[1667])) && (!s.b[1668])) && (!s.b[1669])) {s.store_offset_ad(1357, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1470), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1470), 1.0, A::scale(s.ad_value(1470), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);}
        s.b[1670] = (((1.01 * s.v[1467]) + s.v[1357]) > 0.0);s.store_scalar(1670, if s.b[1670] { 1.0 } else { 0.0 });
        if ((s.b[1608] && (!s.b[1667])) && s.b[1670]) {s.store_add(1342, 1467, 1357);}
        s.b[1671] = ((s.v[1471] * s.v[1467]) < (((0.9 * s.v[1467]) * s.v[1467]) * s.v[1342]));s.store_scalar(1671, if s.b[1671] { 1.0 } else { 0.0 });
        if (((s.b[1608] && (!s.b[1667])) && s.b[1670]) && s.b[1671]) {s.store_offset_div(1466, 1471, 1342, 1e-80);s.store_sub(1468, 1466, 1467);s.store_div(1469, 1468, 1461);}
        s.b[1672] = (s.v[1470] > 0.005);s.store_scalar(1672, if s.b[1672] { 1.0 } else { 0.0 });
        if ((((s.b[1608] && (!s.b[1667])) && s.b[1670]) && (!s.b[1671])) && s.b[1672]) {s.store_sub_mixed_ai(1343, A::ln(A::div_scaled_inputs(s.ad_value(1470), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0)), 1356);}
        s.b[1673] = (s.v[1470] < (-0.005));s.store_scalar(1673, if s.b[1673] { 1.0 } else { 0.0 });
        if (((((s.b[1608] && (!s.b[1667])) && s.b[1670]) && (!s.b[1671])) && (!s.b[1672])) && s.b[1673]) {s.store_sin_scaled_input(1344, 1356, 0.5);s.store_ln_div_scaled_input_square_denominator(1343, 1470, -1.0, 1344, 1.0);}
        if (((((s.b[1608] && (!s.b[1667])) && s.b[1670]) && (!s.b[1671])) && (!s.b[1672])) && (!s.b[1673])) {s.store_ln_ad(1343, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1470), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1470), 1.0, A::scale(s.ad_value(1470), 0.0396825396825397), 0.05), 0.3333333333333)));}
        if (((s.b[1608] && (!s.b[1667])) && s.b[1670]) && (!s.b[1671])) {s.store_sub_add_scaled_inputs4_lhs_mixed_iiia(1469, 1459, 1.0, 1458, (-1.0), 1465, 1.0, A::ln(s.ad_value(1342)), 2.0, 1343);s.store_mul(1468, 1461, 1469);s.store_add(1466, 1467, 1468);}
        s.b[1674] = (s.v[1470] > 0.005);s.store_scalar(1674, if s.b[1674] { 1.0 } else { 0.0 });s.b[1675] = (((s.v[1465] - s.v[1458]) - s.v[1356]) < 80.0);s.store_scalar(1675, if s.b[1675] { 1.0 } else { 0.0 });
        if ((((s.b[1608] && (!s.b[1667])) && (!s.b[1670])) && s.b[1674]) && s.b[1675]) {s.store_exp_ad(1344, A::add_scaled_inputs3(s.ad_value(1465), 1.0, s.ad_value(1458), (-1.0), s.ad_value(1356), -1.0));}
        if ((((s.b[1608] && (!s.b[1667])) && (!s.b[1670])) && s.b[1674]) && (!s.b[1675])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1344, A::add_scaled_inputs3(s.ad_value(1465), 1.0, s.ad_value(1458), (-1.0), s.ad_value(1356), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (((s.b[1608] && (!s.b[1667])) && (!s.b[1670])) && s.b[1674]) {s.store_div(1343, 1344, 1433);s.store_div_scaled_product_mixed_iia(1342, 1470, 1343, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0);}
        s.b[1676] = (s.v[1470] < (-0.005));s.store_scalar(1676, if s.b[1676] { 1.0 } else { 0.0 });
        if ((((s.b[1608] && (!s.b[1667])) && (!s.b[1670])) && (!s.b[1674])) && s.b[1676]) {s.store_sin_scaled_input(1343, 1356, 0.5);s.store_div_scaled_value_by_product_mixed_iai(1342, 1470, -1.0, A::square(s.ad_value(1343)), 1471, 1.0);}
        if ((((s.b[1608] && (!s.b[1667])) && (!s.b[1670])) && (!s.b[1674])) && (!s.b[1676])) {s.store_div_mixed_ai(1342, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1470), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1470), 1.0, A::scale(s.ad_value(1470), 0.0396825396825397), 0.05), 0.3333333333333)), 1471);}
        if ((s.b[1608] && (!s.b[1667])) && (!s.b[1670])) {s.store_offset_div_scaled_inputs2_mixed_iia(1466, 1467, 1.0, 1357, (-1.0), A::sub_from_scalar(1.0, s.ad_value(1342)), 1.0, 1e-80);s.store_sub(1468, 1466, 1467);s.store_div(1469, 1468, 1461);}
        s.b[1677] = ((s.v[1459] - s.v[1469]) < 80.0);s.store_scalar(1677, if s.b[1677] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1677]) {s.store_exp_sub(1342, 1459, 1469);}
        if (s.b[1608] && (!s.b[1677])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1342, A::sub(s.ad_value(1459), s.ad_value(1469)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1608] {s.store_mul(1472, 1433, 1342);s.store_scalar(1475, 0.0);s.store_scalar(1476, 0.0);s.store_scalar(1473, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_91(
        s: &mut Scratch,
    ) {
        if s.b[1608] {s.store_scalar(1474, 0.0);s.store_scalar(1477, 0.0);s.store_scalar(1478, 0.0);}
        s.b[1678] = (s.v[1466] > 1e-6);s.store_scalar(1678, if s.b[1678] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1678]) {s.store_mul(1473, 1471, 1434);s.store_mul(1474, 1472, 1435);s.store_add_scaled_inputs(1475, 1473, 1.0, 1467, 2.0);s.store_add_scaled_inputs(1476, 1474, 1.0, 1468, 2.0);s.store_add_scaled_inputs3_indices(1477, 1466, 2.0, 1473, 1.0, 1474, 1.0);}
        s.b[1679] = (((s.v[1470]) as f64).abs() > 0.005);s.store_scalar(1679, if s.b[1679] { 1.0 } else { 0.0 });
        if ((s.b[1608] && s.b[1678]) && s.b[1679]) {s.store_add_scaled_products3_mixed_iiaiai(2, 1475, 1476, 1.0, A::offset(s.ad_value(1465), 2.0), 1476, 2.0, A::offset(s.ad_value(1469), 2.0), 1475, 2.0);s.store_div_scaled_product_by_product_indices(1478, 1470, 1477, (-4.0), 1466, 2, 1.0);}
        if ((s.b[1608] && s.b[1678]) && (!s.b[1679])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2, 1470, 1.0, 1470, 1.0, 1470, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_add_scaled_products3_mixed_iiiiaa(3, 1475, 1471, 1.0, 1476, 1472, 1.0, A::mul3(s.ad_value(1475), s.ad_value(1476), s.ad_value(1466)), A::offset(A::mul(s.ad_value(1466), s.ad_value(2)), 1.0), 1.0);s.store_div_scaled_product3_by_product_indices(1478, 1471, 1472, 1477, 1.0, 1466, 3, 1.0);}
        if s.b[1608] {s.store_ln(1479, 1466);}
        s.b[1680] = ((s.v[1467] / 2.0) < 80.0);s.store_scalar(1680, if s.b[1680] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1680]) {s.store_ln_one_plus_exp_scaled_input(2, 1467, 0.5);}
        if (s.b[1608] && (!s.b[1680])) {s.store_scale(2, 1467, 0.5);}
        if s.b[1608] {s.store_scale(1480, 2, 2.0);}
        s.b[1681] = ((s.v[1468] / 2.0) < 80.0);s.store_scalar(1681, if s.b[1681] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1681]) {s.store_ln_one_plus_exp_scaled_input(3, 1468, 0.5);}
        if (s.b[1608] && (!s.b[1681])) {s.store_scale(3, 1468, 0.5);}
        if s.b[1608] {s.store_scale(1481, 3, 2.0);s.store_sub(1482, 1481, 1468);s.store_sub(1483, 1480, 1467);s.store_add_scaled_products_indices(1484, 270, 1480, 1.0, 271, 1482, 1.0);s.store_add_scaled_products_indices(1485, 270, 1481, 1.0, 271, 1483, 1.0);s.store_div_add_scaled_inputs_rhs_indices(0, 1466, 1480, 1.0, 1481, 1.0);s.store_mul(1486, 1480, 0);s.store_mul(1487, 1481, 0);s.store_mul_ad_product_rhs_mixed_ia(1488, 1480, 191, A::exp(A::mul(s.ad_value(40), s.ad_value(295))));s.store_mul_ad_product_rhs_mixed_ia(1489, 1481, 192, A::exp(A::mul(s.ad_value(40), s.ad_value(295))));s.store_mul_add_scaled_product_rhs_indices(2, 50, 1482, 1.0, 51, 1483, 1.0);s.store_scaled_add_sqrt_square_offset_ad(3, A::offset(s.ad_value(2), 1.0), 0.01, 0.5);s.store_scaled_add_sqrt_square_offset_ad(4, A::scale_offset(s.ad_value(2), 0.2, 1.0), 0.01, 0.5);s.store_div(1490, 3, 4);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_92(
        s: &mut Scratch,
    ) {
        if s.b[1608] {s.store_mul_ad_product_rhs(1491, 33, A::add_scaled_product(A::offset(A::mul(s.ad_value(41), s.ad_value(1482)), 1.0), 1.0, s.ad_value(42), s.ad_value(1483), 1.0), A::exp(A::mul_scaled_lhs(s.ad_value(44), -1.0, A::ln(A::add_scaled_product(A::offset(A::mul(s.ad_value(1486), s.ad_value(268)), 1.0), 1.0, s.ad_value(1487), s.ad_value(269), 1.0)))));}
        s.b[1682] = (s.v[56] == 0.0);s.store_scalar(1682, if s.b[1682] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1682]) {s.store_scalar(4, 1.0);}
        s.b[1683] = (s.v[56] < 0.0);s.store_scalar(1683, if s.b[1683] { 1.0 } else { 0.0 });
        if ((s.b[1608] && (!s.b[1682])) && s.b[1683]) {s.store_mul_exp_mixed_ia(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(1466), 1e-12))));s.store_sub_from_scalar(4, 1.0, 2);}
        if ((s.b[1608] && (!s.b[1682])) && (!s.b[1683])) {s.store_mul_exp_mixed_ia(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(1466), 1e-12))));s.store_div_from_scalar_offset_input(4, 1.0, 2, 1.0);}
        if s.b[1608] {s.store_mul_ad_affine_product_rhs(1492, 272, s.ad_value(1447), A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(1428))), A::sqrt_square_offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(1428))), 0.01)), 0.5, 0.0);s.store_mul_add_scaled_product_rhs_indices(1493, 1492, 54, 1.0, 1466, 4, 1.0);s.store_add_scaled_inputs_product_mixed_aiii(1494, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(1484)), 1e-6)))), 1.0), 1.0, 1491, 1.0, 38, 1493, 1.0);s.store_add_scaled_inputs_product_mixed_aiii(1495, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(1485)), 1e-6)))), 1.0), 1.0, 1491, 1.0, 39, 1493, 1.0);s.store_div_scaled_product_mixed_iaa(1496, 1490, A::add(s.ad_value(1488), s.ad_value(1489)), 1.0, A::add(A::div(s.ad_value(1488), s.ad_value(1494)), A::div(s.ad_value(1489), s.ad_value(1495))), 1.0);}
        s.b[1684] = (((s.v[1463]) as f64).abs() > 0.007);s.store_scalar(1684, if s.b[1684] { 1.0 } else { 0.0 });s.b[1685] = (s.v[1463] > 0.0);s.store_scalar(1685, if s.b[1685] { 1.0 } else { 0.0 });
        if ((s.b[1608] && s.b[1684]) && s.b[1685]) {s.store_exp_neg_input(0, 1463);s.store_div_mixed_ia(1497, 1463, A::sub_from_scalar(1.0, s.ad_value(0)));s.store_mul(1498, 0, 1497);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_93(
        s: &mut Scratch,
    ) {
        if ((s.b[1608] && s.b[1684]) && s.b[1685]) {s.store_add_offset_lhs_mixed_ai(1499, A::ln(A::div(s.ad_value(1433), A::mul(s.ad_value(1466), s.ad_value(1497)))), (-0.6931471805599), 1436);}
        if ((s.b[1608] && s.b[1684]) && (!s.b[1685])) {s.store_exp(0, 1463);s.store_div_scaled_value_offset_denominator(1498, s.ad_value(1463), 1.0, s.ad_value(0), (-1.0), 1.0);s.store_mul(1497, 0, 1498);s.store_add_offset_lhs_mixed_ai(1499, A::ln(A::div(s.ad_value(1433), A::mul(s.ad_value(1466), s.ad_value(1498)))), (-0.6931471805599), 1437);}
        if (s.b[1608] && s.b[1684]) {s.store_div_scaled_inputs_mixed_ia(1500, 1463, -1.0, A::mul(s.ad_value(1462), A::add_scaled_sub_value_product(1.0, s.ad_value(1497), 1.0, s.ad_value(1463), s.ad_value(1435), (-1.0))), 1.0);s.store_div_scaled_value_by_product_mixed_iia(1501, 1463, 1.0, 1462, A::add_scaled_sub_value_product(1.0, s.ad_value(1498), 1.0, s.ad_value(1463), s.ad_value(1434), 1.0), 1.0);s.store_div_add_scaled_inputs_rhs_ad(1502, 1463, A::div_scaled_offset_numerator(A::mul(s.ad_value(1498), s.ad_value(1435)), 1.0, 0.5, s.ad_value(1501), 1.0), 1.0, A::div_scaled_offset_numerator(A::mul(s.ad_value(1497), s.ad_value(1434)), 1.0, 0.5, s.ad_value(1500), 1.0), -1.0);}
        if (s.b[1608] && (!s.b[1684])) {s.store_scale(0, 1464, (0.5 * 0.1666666666667));s.store_scale(2, 1463, 0.5);s.store_add_offset_lhs(1497, 2, 1.0, 0);s.store_add_mixed_ai(1498, A::sub_from_scalar(1.0, s.ad_value(2)), 0);s.store_scale(3, 2, 0.1666666666667);s.store_div_scalar_by_product_mixed_ia(1500, 1.0, 1462, A::add(A::offset(s.ad_value(1435), 0.5), s.ad_value(3)), 1.0);s.store_div_scalar_by_product_mixed_ia(1501, 1.0, 1462, A::sub(A::offset(s.ad_value(1434), 0.5), s.ad_value(3)), 1.0);s.store_add_scaled_inputs3_offset_mixed_aii(1499, A::ln(A::div(s.ad_value(1433), A::mul_sub_from_scalar_rhs(s.ad_value(1466), 1.0, A::scale(s.ad_value(0), 0.5)))), 1.0, 1436, 0.5, 1437, 0.5, (-0.6931471805599));s.store_div_from_scalar_ad(1502, (-12.0), A::add_scaled_inputs4_offset(s.ad_value(1462), ((-1.0) * 3.0), A::div_scaled_inputs(s.ad_value(1462), 12.0, A::mul(s.ad_value(1460), s.ad_value(1461)), 1.0), 1.0, A::mul3(s.ad_value(1462), A::sub(s.ad_value(1434), s.ad_value(1435)), s.ad_value(1463)), 1.0, A::mul_sub_from_scalar_lhs_scaled_output(0.2, A::scale(s.ad_value(1462), 0.25), s.ad_value(1464), 0.3333333333333), 1.0, 4.0));}
        if s.b[1608] {s.store_div_from_scalar(1503, 1.0, 1502);}
        s.b[1686] = (s.v[1466] > 1e-6);s.store_scalar(1686, if s.b[1686] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1686]) {s.store_div_scaled_value_offset_denominator(1504, s.ad_value(1480), 100.0, s.ad_value(1480), 100.0, 1.0);}
        s.b[1687] = (s.v[61] < 0.0);s.store_scalar(1687, if s.b[1687] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_94(
        s: &mut Scratch,
    ) {
        if ((s.b[1608] && s.b[1686]) && s.b[1687]) {s.store_div_from_scalar_sub_from_scalar_ad(1505, 1.0, 1.0, A::mul(s.ad_value(61), s.ad_value(1504)));}
        if ((s.b[1608] && s.b[1686]) && (!s.b[1687])) {s.store_offset_mul(1505, 61, 1504, 1.0);}
        if (s.b[1608] && s.b[1686]) {s.store_div_scaled_value_offset_denominator(1506, s.ad_value(1481), 100.0, s.ad_value(1481), 100.0, 1.0);}
        s.b[1688] = (s.v[62] < 0.0);s.store_scalar(1688, if s.b[1688] { 1.0 } else { 0.0 });
        if ((s.b[1608] && s.b[1686]) && s.b[1688]) {s.store_div_from_scalar_sub_from_scalar_ad(1507, 1.0, 1.0, A::mul(s.ad_value(62), s.ad_value(1506)));}
        if ((s.b[1608] && s.b[1686]) && (!s.b[1688])) {s.store_offset_mul(1507, 62, 1506, 1.0);}
        if (s.b[1608] && s.b[1686]) {s.store_sub_ad(1508, A::div_scaled_product_by_product(s.ad_value(1478), s.ad_value(1477), 1.0, s.ad_value(1475), s.ad_value(1476), 1.0), A::div_scaled_inputs2(A::div(s.ad_value(1471), s.ad_value(1475)), 1.0, A::div(s.ad_value(1472), s.ad_value(1476)), 1.0, s.ad_value(1466), 1.0));s.store_div_scaled_product_offset_denominator_indices(1509, 1508, 1466, 1.0, 1508, 1.0, 1.0);s.store_sub(2, 1502, 1509);s.store_div_scaled_add_product_indices(1510, 1466, 1.0, 1502, 1499, 1.0, 2, 1.0);s.store_scaled_add_mixed_ia(1510, 1510, A::sqrt_square_offset(s.ad_value(1510), 1e-6), 0.5);s.store_scaled_mul_ad(1511, A::div(s.ad_value(1424), s.ad_value(1496)), A::add(s.ad_value(1505), s.ad_value(1507)), 0.5);s.store_sub_from_scalar_div_indices(1512, 1.0, 1466, 1509);s.store_offset(1513, 1499, 1.0);s.store_mul_sub_mixed_iai(1514, 1510, A::offset(A::mul(A::sub_scaled_inputs(s.ad_value(1509), 2.0, s.ad_value(1466), 1.0), s.ad_value(1503)), (-2.0)), 1499);}
        s.b[1689] = (s.v[1511] > 1e-14);s.store_scalar(1689, if s.b[1689] { 1.0 } else { 0.0 });
        if ((s.b[1608] && s.b[1686]) && s.b[1689]) {s.store_div_from_scalar_square_ad(1515, 2.0, s.ad_value(1511));s.store_mul(1516, 1515, 1512);s.store_add(1517, 1515, 1514);s.store_mul(1518, 1515, 1513);s.store_sqrt_offset_ad(1519, A::add(A::square(s.ad_value(1516)), A::mul3_scaled_output(s.ad_value(1515), s.ad_value(1515), s.ad_value(1515), 0.148148148148)), 1e-20);s.store_sqrt_offset_ad(1520, A::add(A::square(s.ad_value(1518)), A::mul3_scaled_output(s.ad_value(1517), s.ad_value(1517), s.ad_value(1517), 0.148148148148)), 1e-20);s.store_sub_ad(1521, A::exp_scaled_input(A::ln_scaled_input(A::add(s.ad_value(1519), s.ad_value(1516)), 0.5), 0.3333333333333), A::exp_scaled_input(A::ln_scaled_input(A::sub(s.ad_value(1519), s.ad_value(1516)), 0.5), 0.3333333333333));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_95(
        s: &mut Scratch,
    ) {
        if ((s.b[1608] && s.b[1686]) && s.b[1689]) {s.store_sub_ad(1522, A::exp_scaled_input(A::ln_scaled_input(A::add(s.ad_value(1520), s.ad_value(1518)), 0.5), 0.3333333333333), A::exp_scaled_input(A::ln_scaled_input(A::sub(s.ad_value(1520), s.ad_value(1518)), 0.5), 0.3333333333333));}
        if ((s.b[1608] && s.b[1686]) && (!s.b[1689])) {s.copy_ad(1521, 1512);s.copy_ad(1522, 1513);}
        if (s.b[1608] && s.b[1686]) {s.store_square(4, 2);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1523, 1521, (0.94 * 0.5), 1522, (0.94 * 0.5), A::add_scaled_inputs(A::square(A::sub(s.ad_value(1521), s.ad_value(1522))), 1.0, s.ad_value(4), 10.0), (0.94 * 0.5));s.store_add_scaled_product_indices(1524, 1466, 1.0, 1509, 1523, 1.0);s.store_mul_sub_rhs(1525, 1502, 1523, 1499);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1526, 1524, 0.5, 1525, 0.5, A::add_scaled_inputs(A::square(A::sub(s.ad_value(1524), s.ad_value(1525))), 1.0, s.ad_value(4), 36.0), 0.5);}
        if (s.b[1608] && (!s.b[1686])) {s.copy_ad(1509, 1502);s.store_scaled_offset(1523, 1499, 1.0, 0.94);s.store_add_scaled_product_mixed_iia(1526, 1466, 0.5, 1502, A::sub_scaled_inputs(s.ad_value(1523), 1.0, s.ad_value(1499), 0.5), 1.0);}
        s.b[1690] = ((s.v[1526] - 0.5) < 80.0);s.store_scalar(1690, if s.b[1690] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1690]) {s.store_ln_one_plus_exp_ad(2, A::offset(s.ad_value(1526), (-0.5)));}
        if (s.b[1608] && (!s.b[1690])) {s.store_offset(2, 1526, (-0.5));}
        if s.b[1608] {s.store_offset(3, 2, 0.5);s.store_add_mixed_ia(4, 1523, A::ln(A::div(s.ad_value(1466), s.ad_value(3))));}
        s.b[1691] = ((s.v[4] - 6.0) < 80.0);s.store_scalar(1691, if s.b[1691] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1691]) {s.store_ln_one_plus_exp_ad(2, A::offset(s.ad_value(4), (-6.0)));}
        if (s.b[1608] && (!s.b[1691])) {s.store_offset(2, 4, (-6.0));}
        if s.b[1608] {s.store_offset(4, 2, 6.0);}
        s.b[1692] = ((s.v[225] - s.v[4]) < 80.0);s.store_scalar(1692, if s.b[1692] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1692]) {s.store_ln_one_plus_exp_ad(2, A::sub(s.ad_value(225), s.ad_value(4)));}
        if (s.b[1608] && (!s.b[1692])) {s.store_sub(2, 225, 4);}
        if s.b[1608] {s.store_sub(1527, 225, 2);s.store_div(2, 339, 1527);s.store_square(3, 2);s.store_square(4, 3);s.store_square(5, 4);s.store_exp_scaled_input_ad(0, A::ln(A::offset(A::mul(s.ad_value(1425), s.ad_value(4)), 1.0)), 2.666666666667);s.store_mul_mixed_ia(1528, 339, A::exp_scaled_input(A::ln(A::add(s.ad_value(0), A::square(s.ad_value(5)))), (-0.0625)));s.store_div_from_scalar_offset_input(1342, 1.0, 1460, 1.0);s.store_div_from_scalar_offset_input(1343, 1.0, 1461, 1.0);s.store_offset_add_ad(1345, A::ln(A::div_scaled_product(A::add_scaled_product(s.ad_value(1460), 1.0, s.ad_value(1461), s.ad_value(1343), 1.0), s.ad_value(1432), 1.0, s.ad_value(1433), 1.0)), s.ad_value(1528), 3.0);}
    }
}
