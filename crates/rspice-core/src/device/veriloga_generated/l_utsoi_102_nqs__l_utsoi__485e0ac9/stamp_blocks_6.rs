#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_96(
        s: &mut Scratch,
    ) {
        if s.b[1608] {s.store_offset_add_ad(1346, A::ln(A::div_scaled_product(A::add_scaled_product(s.ad_value(1461), 1.0, s.ad_value(1460), s.ad_value(1342), 1.0), s.ad_value(1432), 1.0, s.ad_value(1433), 1.0)), s.ad_value(1528), 3.0);}
        s.b[1693] = (((s.v[1345] - s.v[1436]) * 0.3333333333333) < 80.0);s.store_scalar(1693, if s.b[1693] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1693]) {s.store_ln_one_plus_exp_ad(1344, A::sub_scaled_inputs(s.ad_value(1345), 0.3333333333333, s.ad_value(1436), 0.3333333333333));}
        if (s.b[1608] && (!s.b[1693])) {s.store_scaled_sub(1344, 1345, 1436, 0.3333333333333);}
        if s.b[1608] {s.store_sub_scaled_inputs(1349, 1345, 1.0, 1344, 3.0);}
        s.b[1694] = (((s.v[1346] - s.v[1437]) * 0.3333333333333) < 80.0);s.store_scalar(1694, if s.b[1694] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1694]) {s.store_ln_one_plus_exp_ad(1344, A::sub_scaled_inputs(s.ad_value(1346), 0.3333333333333, s.ad_value(1437), 0.3333333333333));}
        if (s.b[1608] && (!s.b[1694])) {s.store_scaled_sub(1344, 1346, 1437, 0.3333333333333);}
        if s.b[1608] {s.store_sub_scaled_inputs(1350, 1346, 1.0, 1344, 3.0);s.store_mul_add_scaled_product_rhs_indices(1347, 1342, 1350, 1.0, 1460, 1458, 1.0);s.store_mul_add_scaled_product_rhs_indices(1348, 1343, 1349, 1.0, 1461, 1459, 1.0);}
        s.b[1695] = (((s.v[1345] - s.v[1347]) * 0.3333333333333) < 80.0);s.store_scalar(1695, if s.b[1695] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1695]) {s.store_ln_one_plus_exp_ad(1344, A::sub_scaled_inputs(s.ad_value(1345), 0.3333333333333, s.ad_value(1347), 0.3333333333333));}
        if (s.b[1608] && (!s.b[1695])) {s.store_scaled_sub(1344, 1345, 1347, 0.3333333333333);}
        if s.b[1608] {s.store_sub_scaled_inputs(1349, 1345, 1.0, 1344, 3.0);}
        s.b[1696] = (((s.v[1346] - s.v[1348]) * 0.3333333333333) < 80.0);s.store_scalar(1696, if s.b[1696] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1696]) {s.store_ln_one_plus_exp_ad(1344, A::sub_scaled_inputs(s.ad_value(1346), 0.3333333333333, s.ad_value(1348), 0.3333333333333));}
        if (s.b[1608] && (!s.b[1696])) {s.store_scaled_sub(1344, 1346, 1348, 0.3333333333333);}
        if s.b[1608] {s.store_sub_scaled_inputs(1350, 1346, 1.0, 1344, 3.0);s.store_sub(1529, 1458, 1349);s.store_sub(1530, 1459, 1350);s.store_scalar(1356, 0.0);s.store_scalar(1359, 0.0);s.store_mul(1351, 1460, 1529);}
        s.b[1697] = (((s.v[1458] - s.v[1529]) - s.v[1528]) < 80.0);s.store_scalar(1697, if s.b[1697] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1697]) {s.store_exp_ad(1342, A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0));}
        if (s.b[1608] && (!s.b[1697])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1342, A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1608] {s.store_mul(1352, 1433, 1342);s.store_sub_square_lhs(1353, 1351, 1352);s.store_add_scaled_product_indices(1354, 1352, 1.0, 1460, 1351, 2.0);s.store_add_scaled_product_indices(1355, 1352, (-1.0), 1460, 1460, 2.0);}
        s.b[1698] = (s.v[1353] < (-0.005));s.store_scalar(1698, if s.b[1698] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1698]) {s.store_sqrt_abs_ad(1356, s.ad_value(1353));s.store_div_mixed_ia(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);s.store_mul_add_mixed_iia(1358, 1342, 1353, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)));s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);s.store_mul_div_lhs(1363, 1354, 1353, 1343);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_97(
        s: &mut Scratch,
    ) {
        if (s.b[1608] && s.b[1698]) {s.store_div_mixed_ai(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);}
        s.b[1699] = (s.v[1353] > 0.005);s.store_scalar(1699, if s.b[1699] { 1.0 } else { 0.0 });
        if ((s.b[1608] && (!s.b[1698])) && s.b[1699]) {s.store_sqrt_abs_ad(1356, s.ad_value(1353));s.store_exp_neg_input(1359, 1356);s.store_div_scaled_product_offset_rhs_mixed_iia(1357, 1356, 1359, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0);s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);s.store_mul_add_mixed_iia(1358, 1342, 1353, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)));s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);s.store_mul_div_lhs(1363, 1354, 1353, 1343);s.store_div_mixed_ai(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);}
        if ((s.b[1608] && (!s.b[1698])) && (!s.b[1699])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1344, 1353, 1.0, 1353, 1.0, 1353, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(1357, 1353, 1344, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1342, 1353, 1.0, 1353, 1.0, 1353, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(1358, 1354, 1342);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1343, 1353, 1.0, 1353, 1.0, 1353, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(1360, 1355, 1342, 1.0, A::square(s.ad_value(1354)), 1343, (-1.0));s.store_scaled_mul(1363, 1354, 1344, (-0.5));s.store_add_scaled_product_mixed_aii(1364, A::mul3_scaled_output(s.ad_value(1354), s.ad_value(1354), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 2.0, A::scale(s.ad_value(1353), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1355, 1344, (-0.5));}
        s.b[1700] = (s.v[1353] > 0.005);s.store_scalar(1700, if s.b[1700] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1700]) {s.store_div_scaled_inputs_mixed_ia(1343, 1353, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0);s.store_mul(1361, 1343, 1359);s.store_sub_ln_lhs(1362, 1343, 1356);}
        s.b[1701] = (s.v[1353] < (-0.005));s.store_scalar(1701, if s.b[1701] { 1.0 } else { 0.0 });
        if ((s.b[1608] && (!s.b[1700])) && s.b[1701]) {s.store_sin_scaled_input(1343, 1356, 0.5);s.store_div_scaled_inputs_square_rhs(1361, 1353, -1.0, 1343, 1.0);s.store_ln(1362, 1361);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_98(
        s: &mut Scratch,
    ) {
        if ((s.b[1608] && (!s.b[1700])) && (!s.b[1701])) {s.store_sub_from_scalar_ad(1361, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(1362, 1361);}
        s.b[1702] = (((1.01 * s.v[1351]) + s.v[1357]) > 0.0);s.store_scalar(1702, if s.b[1702] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1702]) {s.store_add(1365, 1351, 1357);s.store_add(1366, 1460, 1358);s.copy_ad(1367, 1360);}
        if (s.b[1608] && (!s.b[1702])) {s.store_div_from_scalar_sub_ad(1343, 1.0, s.ad_value(1351), s.ad_value(1357));s.store_sub(1344, 1358, 1460);s.store_mul_sub_lhs(1365, 1352, 1361, 1343);s.store_mul_mixed_ai(1366, A::add_scaled_value_products(s.ad_value(1352), (-1.0), s.ad_value(1344), s.ad_value(1365), 1.0, s.ad_value(1363), s.ad_value(1361), (-1.0)), 1343);s.store_mul_mixed_ai(1367, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1360), s.ad_value(1365), 1.0, s.ad_value(1344), s.ad_value(1366), 2.0), 1.0, s.ad_value(1352), 1.0, A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361), (-1.0)), 1343);}
        s.b[1703] = (s.v[1365] > 0.0);s.store_scalar(1703, if s.b[1703] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1703]) {s.store_ln(1368, 1365);s.store_div_from_scalar(1342, 1.0, 1365);s.store_mul(1369, 1366, 1342);s.store_add_scaled_square_product_indices(1370, 1369, (-1.0), 1367, 1342, 1.0);}
        if (s.b[1608] && (!s.b[1703])) {s.store_add_offset_lhs_mixed_ia(1368, 1351, 0.6931471805599, A::ln_scaled_input(s.ad_value(1351), -1.0));s.store_div_from_scalar(1342, 1.0, 1529);s.store_add(1369, 1460, 1342);s.store_mul_scale_offset_indices(1370, 1342, 1342, -1.0, 0.0);}
        if s.b[1608] {s.store_sub_add_scaled_inputs4_lhs_indices(1371, 1459, 1.0, 1458, (-1.0), 1529, 1.0, 1368, 2.0, 1362);s.store_sub_mixed_ai(1372, A::scale_offset(s.ad_value(1369), 2.0, 1.0), 1363);s.store_sub_scaled_inputs(1373, 1370, 2.0, 1364, 1.0);s.store_add_scaled_product_indices(1374, 1351, 1.0, 1461, 1371, 1.0);s.store_add_scaled_product_indices(1375, 1460, 1.0, 1461, 1372, 1.0);s.store_mul(1376, 1461, 1373);s.store_add_scaled_product_indices(1377, 1352, (-1.0), 1374, 1365, 1.0);s.store_add_mixed_ai(1378, A::add_scaled_products(s.ad_value(1375), s.ad_value(1365), 1.0, s.ad_value(1374), s.ad_value(1366), 1.0), 1352);s.store_sub_mixed_ai(1379, A::add_scaled_products3(s.ad_value(1376), s.ad_value(1365), 1.0, s.ad_value(1375), s.ad_value(1366), 2.0, s.ad_value(1374), s.ad_value(1367), 1.0), 1352);s.store_add_scaled_square_product_indices(1388, 1378, 1.0, 1377, 1379, (-0.5));s.store_div_scaled_product3_mixed_iiia(1380, 1377, 1378, 1388, -1.0, A::offset(A::square(s.ad_value(1388)), 1e-200), 1.0);s.store_add(1529, 1529, 1380);s.store_mul(1351, 1460, 1529);s.store_mul(1381, 1461, 1530);s.store_add(1374, 1351, 1381);s.store_offset_scaled(1382, 1374, 0.065345483024, 1.0);s.store_add_scaled_product_mixed_aii(1383, A::scale_offset(s.ad_value(1374), 8.5797362674, 39.478417604), 1.0, 1351, 1381, 1.0);s.store_add_scaled_product_indices(1384, 1374, (2.0 * 39.478417604), 1351, 1381, 39.478417604);s.store_sqrt_add_scaled_square_product(1385, 1383, 1.0, 1382, 1384, (-4.0));s.store_div_scaled_inputs2_indices(1353, 1385, 1.0, 1383, (-1.0), 1382, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_99(
        s: &mut Scratch,
    ) {
        if s.b[1608] {s.store_sub_square_lhs(1386, 1351, 1353);}
        s.b[1704] = (s.v[1386] > 0.0);s.store_scalar(1704, if s.b[1704] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1704]) {s.store_mul_add_scaled_inputs4_rhs_mixed_aiii(1377, 1386, A::ln(A::div(s.ad_value(1386), s.ad_value(1433))), 1.0, 1528, 1.0, 1458, -1.0, 1529, 1.0);s.store_add_scaled_product_indices(1378, 1386, 1.0, 1460, 1351, 2.0);}
        let (t2,) = {
    if (s.b[1608] && s.b[1704]) {
        let t0: f64 = (s.v[1458] - s.v[1529]);let t1: f64 = (t0 - s.v[1345]);
        (t1,)
    } else {
        (s.v[1387],)
    }
};
        s.store_scalar(1387, t2);s.b[1705] = ((((s.v[1377] < 0.0) && (s.v[1378] > 0.0)) && (((s.v[1387] + 2.3025850929941) + ((s.v[1460]) as f64).ln()) > 0.0)) || (s.v[1387] > 1.0));s.store_scalar(1705, if s.b[1705] { 1.0 } else { 0.0 });
        if ((s.b[1608] && s.b[1704]) && s.b[1705]) {s.store_sub_div_rhs_indices(1529, 1529, 1377, 1378);}
        if s.b[1608] {s.store_mul(1351, 1460, 1529);s.store_mul(1381, 1461, 1530);s.store_add(1374, 1351, 1381);s.store_offset_scaled(1382, 1374, 0.065345483024, 1.0);s.store_add_scaled_product_mixed_aii(1383, A::scale_offset(s.ad_value(1374), 8.5797362674, 39.478417604), 1.0, 1351, 1381, 1.0);s.store_add_scaled_product_indices(1384, 1374, (2.0 * 39.478417604), 1351, 1381, 39.478417604);s.store_sqrt_add_scaled_square_product(1385, 1383, 1.0, 1382, 1384, (-4.0));s.store_div_scaled_inputs2_indices(1353, 1385, 1.0, 1383, (-1.0), 1382, 2.0);}
        s.b[1706] = (s.v[1353] < (-0.005));s.store_scalar(1706, if s.b[1706] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1706]) {s.store_sqrt_abs_ad(1356, s.ad_value(1353));s.store_div_mixed_ia(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));s.store_div_scaled_inputs2_mixed_iai(1358, 1353, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 0.25, 1353, 1.0);}
        s.b[1707] = (s.v[1353] > 0.005);s.store_scalar(1707, if s.b[1707] { 1.0 } else { 0.0 });
        if ((s.b[1608] && (!s.b[1706])) && s.b[1707]) {s.store_sqrt_abs_ad(1356, s.ad_value(1353));s.store_exp_neg_input(1359, 1356);s.store_div_scaled_product_offset_rhs_mixed_iia(1357, 1356, 1359, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0);s.store_div_scaled_inputs2_mixed_iai(1358, 1353, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 0.25, 1353, 1.0);}
        if ((s.b[1608] && (!s.b[1706])) && (!s.b[1707])) {s.store_offset_ad(1357, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1358, 1353, 1.0, 1353, 1.0, 1353, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);}
        if s.b[1608] {s.store_sub_mixed_ia(1353, 1353, A::div_scaled_inputs2(A::add_scaled_products(s.ad_value(1374), s.ad_value(1357), 1.0, s.ad_value(1351), s.ad_value(1381), 1.0), 1.0, s.ad_value(1353), 1.0, A::offset(A::mul(s.ad_value(1374), s.ad_value(1358)), 1.0), 1.0));s.store_sub_square_lhs(1386, 1351, 1353);}
        s.b[1708] = (s.v[1386] > 0.0);s.store_scalar(1708, if s.b[1708] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1708]) {s.store_mul_add_scaled_inputs4_rhs_mixed_aiii(1377, 1386, A::ln(A::div(s.ad_value(1386), s.ad_value(1433))), 1.0, 1528, 1.0, 1458, -1.0, 1529, 1.0);s.store_add_scaled_product_indices(1378, 1386, 1.0, 1460, 1351, 2.0);}
        let (t5,) = {
    if (s.b[1608] && s.b[1708]) {
        let t3: f64 = (s.v[1458] - s.v[1529]);let t4: f64 = (t3 - s.v[1345]);
        (t4,)
    } else {
        (s.v[1387],)
    }
};
        s.store_scalar(1387, t5);s.b[1709] = ((((s.v[1377] < 0.0) && (s.v[1378] > 0.0)) && (((s.v[1387] + 2.3025850929941) + ((s.v[1460]) as f64).ln()) > 0.0)) || (s.v[1387] > 1.0));s.store_scalar(1709, if s.b[1709] { 1.0 } else { 0.0 });
        if ((s.b[1608] && s.b[1708]) && s.b[1709]) {s.store_sub_div_rhs_indices(1529, 1529, 1377, 1378);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_100(
        s: &mut Scratch,
    ) {
        if s.b[1608] {s.store_mul(1351, 1460, 1529);}
        s.b[1710] = (((s.v[1458] - s.v[1529]) - s.v[1528]) < 80.0);s.store_scalar(1710, if s.b[1710] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1710]) {s.store_exp_ad(1342, A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0));}
        if (s.b[1608] && (!s.b[1710])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1342, A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1608] {s.store_mul(1352, 1433, 1342);s.store_sub_square_lhs(1353, 1351, 1352);s.store_add_scaled_product_indices(1354, 1352, 1.0, 1460, 1351, 2.0);s.store_add_scaled_product_indices(1355, 1352, (-1.0), 1460, 1460, 2.0);}
        s.b[1711] = (s.v[1353] < (-0.005));s.store_scalar(1711, if s.b[1711] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1711]) {s.store_sqrt_abs_ad(1356, s.ad_value(1353));s.store_div_mixed_ia(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);s.store_mul_add_mixed_iia(1358, 1342, 1353, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)));s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);s.store_mul_div_lhs(1363, 1354, 1353, 1343);s.store_div_mixed_ai(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);}
        s.b[1712] = (s.v[1353] > 0.005);s.store_scalar(1712, if s.b[1712] { 1.0 } else { 0.0 });
        if ((s.b[1608] && (!s.b[1711])) && s.b[1712]) {s.store_sqrt_abs_ad(1356, s.ad_value(1353));s.store_exp_neg_input(1359, 1356);s.store_div_scaled_product_offset_rhs_mixed_iia(1357, 1356, 1359, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0);s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);s.store_mul_add_mixed_iia(1358, 1342, 1353, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)));s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);s.store_mul_div_lhs(1363, 1354, 1353, 1343);s.store_div_mixed_ai(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);}
        if ((s.b[1608] && (!s.b[1711])) && (!s.b[1712])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1344, 1353, 1.0, 1353, 1.0, 1353, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(1357, 1353, 1344, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_101(
        s: &mut Scratch,
    ) {
        if ((s.b[1608] && (!s.b[1711])) && (!s.b[1712])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1342, 1353, 1.0, 1353, 1.0, 1353, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(1358, 1354, 1342);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1343, 1353, 1.0, 1353, 1.0, 1353, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(1360, 1355, 1342, 1.0, A::square(s.ad_value(1354)), 1343, (-1.0));s.store_scaled_mul(1363, 1354, 1344, (-0.5));s.store_add_scaled_product_mixed_aii(1364, A::mul3_scaled_output(s.ad_value(1354), s.ad_value(1354), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 2.0, A::scale(s.ad_value(1353), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1355, 1344, (-0.5));}
        s.b[1713] = (s.v[1353] > 0.005);s.store_scalar(1713, if s.b[1713] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1713]) {s.store_div_scaled_inputs_mixed_ia(1343, 1353, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0);s.store_mul(1361, 1343, 1359);s.store_sub_ln_lhs(1362, 1343, 1356);}
        s.b[1714] = (s.v[1353] < (-0.005));s.store_scalar(1714, if s.b[1714] { 1.0 } else { 0.0 });
        if ((s.b[1608] && (!s.b[1713])) && s.b[1714]) {s.store_sin_scaled_input(1343, 1356, 0.5);s.store_div_scaled_inputs_square_rhs(1361, 1353, -1.0, 1343, 1.0);s.store_ln(1362, 1361);}
        if ((s.b[1608] && (!s.b[1713])) && (!s.b[1714])) {s.store_sub_from_scalar_ad(1361, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(1362, 1361);}
        s.b[1715] = (((1.01 * s.v[1351]) + s.v[1357]) > 0.0);s.store_scalar(1715, if s.b[1715] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1715]) {s.store_add(1365, 1351, 1357);s.store_add(1366, 1460, 1358);s.copy_ad(1367, 1360);}
        if (s.b[1608] && (!s.b[1715])) {s.store_div_from_scalar_sub_ad(1343, 1.0, s.ad_value(1351), s.ad_value(1357));s.store_sub(1344, 1358, 1460);s.store_mul_sub_lhs(1365, 1352, 1361, 1343);s.store_mul_mixed_ai(1366, A::add_scaled_value_products(s.ad_value(1352), (-1.0), s.ad_value(1344), s.ad_value(1365), 1.0, s.ad_value(1363), s.ad_value(1361), (-1.0)), 1343);s.store_mul_mixed_ai(1367, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1360), s.ad_value(1365), 1.0, s.ad_value(1344), s.ad_value(1366), 2.0), 1.0, s.ad_value(1352), 1.0, A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361), (-1.0)), 1343);}
        s.b[1716] = (s.v[1365] > 0.0);s.store_scalar(1716, if s.b[1716] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1716]) {s.store_ln(1368, 1365);s.store_div_from_scalar(1342, 1.0, 1365);s.store_mul(1369, 1366, 1342);s.store_add_scaled_square_product_indices(1370, 1369, (-1.0), 1367, 1342, 1.0);}
        if (s.b[1608] && (!s.b[1716])) {s.store_add_offset_lhs_mixed_ia(1368, 1351, 0.6931471805599, A::ln_scaled_input(s.ad_value(1351), -1.0));s.store_div_from_scalar(1342, 1.0, 1529);s.store_add(1369, 1460, 1342);s.store_mul_scale_offset_indices(1370, 1342, 1342, -1.0, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_102(
        s: &mut Scratch,
    ) {
        if s.b[1608] {s.store_sub_add_scaled_inputs4_lhs_indices(1371, 1459, 1.0, 1458, (-1.0), 1529, 1.0, 1368, 2.0, 1362);s.store_sub_mixed_ai(1372, A::scale_offset(s.ad_value(1369), 2.0, 1.0), 1363);s.store_sub_scaled_inputs(1373, 1370, 2.0, 1364, 1.0);s.store_add_scaled_product_indices(1374, 1351, 1.0, 1461, 1371, 1.0);s.store_add_scaled_product_indices(1375, 1460, 1.0, 1461, 1372, 1.0);s.store_mul(1376, 1461, 1373);s.store_add_scaled_product_indices(1377, 1352, (-1.0), 1374, 1365, 1.0);s.store_add_mixed_ai(1378, A::add_scaled_products(s.ad_value(1375), s.ad_value(1365), 1.0, s.ad_value(1374), s.ad_value(1366), 1.0), 1352);s.store_sub_mixed_ai(1379, A::add_scaled_products3(s.ad_value(1376), s.ad_value(1365), 1.0, s.ad_value(1375), s.ad_value(1366), 2.0, s.ad_value(1374), s.ad_value(1367), 1.0), 1352);s.store_add_scaled_square_product_indices(1388, 1378, 1.0, 1377, 1379, (-0.5));s.store_div_scaled_product3_mixed_iiia(1380, 1377, 1378, 1388, -1.0, A::offset(A::square(s.ad_value(1388)), 1e-200), 1.0);s.store_add(1529, 1529, 1380);s.store_mul(1351, 1460, 1529);}
        s.b[1717] = (((s.v[1458] - s.v[1529]) - s.v[1528]) < 80.0);s.store_scalar(1717, if s.b[1717] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1717]) {s.store_exp_ad(1342, A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0));}
        if (s.b[1608] && (!s.b[1717])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1342, A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1608] {s.store_mul(1352, 1433, 1342);s.store_sub_square_lhs(1353, 1351, 1352);s.store_add_scaled_product_indices(1354, 1352, 1.0, 1460, 1351, 2.0);s.store_add_scaled_product_indices(1355, 1352, (-1.0), 1460, 1460, 2.0);}
        s.b[1718] = (s.v[1353] < (-0.005));s.store_scalar(1718, if s.b[1718] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1718]) {s.store_sqrt_abs_ad(1356, s.ad_value(1353));s.store_div_mixed_ia(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);s.store_mul_add_mixed_iia(1358, 1342, 1353, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)));s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);s.store_mul_div_lhs(1363, 1354, 1353, 1343);s.store_div_mixed_ai(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);}
        s.b[1719] = (s.v[1353] > 0.005);s.store_scalar(1719, if s.b[1719] { 1.0 } else { 0.0 });
        if ((s.b[1608] && (!s.b[1718])) && s.b[1719]) {s.store_sqrt_abs_ad(1356, s.ad_value(1353));s.store_exp_neg_input(1359, 1356);s.store_div_scaled_product_offset_rhs_mixed_iia(1357, 1356, 1359, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0);s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_103(
        s: &mut Scratch,
    ) {
        if ((s.b[1608] && (!s.b[1718])) && s.b[1719]) {s.store_mul_add_mixed_iia(1358, 1342, 1353, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)));s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);s.store_mul_div_lhs(1363, 1354, 1353, 1343);s.store_div_mixed_ai(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);}
        if ((s.b[1608] && (!s.b[1718])) && (!s.b[1719])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1344, 1353, 1.0, 1353, 1.0, 1353, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(1357, 1353, 1344, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1342, 1353, 1.0, 1353, 1.0, 1353, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(1358, 1354, 1342);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1343, 1353, 1.0, 1353, 1.0, 1353, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(1360, 1355, 1342, 1.0, A::square(s.ad_value(1354)), 1343, (-1.0));s.store_scaled_mul(1363, 1354, 1344, (-0.5));s.store_add_scaled_product_mixed_aii(1364, A::mul3_scaled_output(s.ad_value(1354), s.ad_value(1354), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 2.0, A::scale(s.ad_value(1353), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1355, 1344, (-0.5));}
        s.b[1720] = (s.v[1353] > 0.005);s.store_scalar(1720, if s.b[1720] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1720]) {s.store_div_scaled_inputs_mixed_ia(1343, 1353, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0);s.store_mul(1361, 1343, 1359);s.store_sub_ln_lhs(1362, 1343, 1356);}
        s.b[1721] = (s.v[1353] < (-0.005));s.store_scalar(1721, if s.b[1721] { 1.0 } else { 0.0 });
        if ((s.b[1608] && (!s.b[1720])) && s.b[1721]) {s.store_sin_scaled_input(1343, 1356, 0.5);s.store_div_scaled_inputs_square_rhs(1361, 1353, -1.0, 1343, 1.0);s.store_ln(1362, 1361);}
        if ((s.b[1608] && (!s.b[1720])) && (!s.b[1721])) {s.store_sub_from_scalar_ad(1361, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(1362, 1361);}
        s.b[1722] = (((1.01 * s.v[1351]) + s.v[1357]) > 0.0);s.store_scalar(1722, if s.b[1722] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1722]) {s.store_add(1365, 1351, 1357);s.store_add(1366, 1460, 1358);s.copy_ad(1367, 1360);}
        if (s.b[1608] && (!s.b[1722])) {s.store_div_from_scalar_sub_ad(1343, 1.0, s.ad_value(1351), s.ad_value(1357));s.store_sub(1344, 1358, 1460);s.store_mul_sub_lhs(1365, 1352, 1361, 1343);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_104(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1608] && (!s.b[1722])) {s.store_mul_mixed_ai(1366, A::add_scaled_value_products(s.ad_value(1352), (-1.0), s.ad_value(1344), s.ad_value(1365), 1.0, s.ad_value(1363), s.ad_value(1361), (-1.0)), 1343);s.store_mul_mixed_ai(1367, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1360), s.ad_value(1365), 1.0, s.ad_value(1344), s.ad_value(1366), 2.0), 1.0, s.ad_value(1352), 1.0, A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361), (-1.0)), 1343);}
        s.b[1723] = (s.v[1365] > 0.0);s.store_scalar(1723, if s.b[1723] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1723]) {s.store_ln(1368, 1365);s.store_div_from_scalar(1342, 1.0, 1365);s.store_mul(1369, 1366, 1342);s.store_add_scaled_square_product_indices(1370, 1369, (-1.0), 1367, 1342, 1.0);}
        if (s.b[1608] && (!s.b[1723])) {s.store_add_offset_lhs_mixed_ia(1368, 1351, 0.6931471805599, A::ln_scaled_input(s.ad_value(1351), -1.0));s.store_div_from_scalar(1342, 1.0, 1529);s.store_add(1369, 1460, 1342);s.store_mul_scale_offset_indices(1370, 1342, 1342, -1.0, 0.0);}
        if s.b[1608] {s.store_sub_add_scaled_inputs4_lhs_indices(1371, 1459, 1.0, 1458, (-1.0), 1529, 1.0, 1368, 2.0, 1362);s.store_sub_mixed_ai(1372, A::scale_offset(s.ad_value(1369), 2.0, 1.0), 1363);s.store_sub_scaled_inputs(1373, 1370, 2.0, 1364, 1.0);s.store_add_scaled_product_indices(1374, 1351, 1.0, 1461, 1371, 1.0);s.store_add_scaled_product_indices(1375, 1460, 1.0, 1461, 1372, 1.0);s.store_mul(1376, 1461, 1373);s.store_add_scaled_product_indices(1377, 1352, (-1.0), 1374, 1365, 1.0);s.store_add_mixed_ai(1378, A::add_scaled_products(s.ad_value(1375), s.ad_value(1365), 1.0, s.ad_value(1374), s.ad_value(1366), 1.0), 1352);s.store_sub_mixed_ai(1379, A::add_scaled_products3(s.ad_value(1376), s.ad_value(1365), 1.0, s.ad_value(1375), s.ad_value(1366), 2.0, s.ad_value(1374), s.ad_value(1367), 1.0), 1352);s.store_add_scaled_square_product_indices(1388, 1378, 1.0, 1377, 1379, (-0.5));s.store_div_scaled_product3_mixed_iiia(1380, 1377, 1378, 1388, -1.0, A::offset(A::square(s.ad_value(1388)), 1e-200), 1.0);s.store_add(1529, 1529, 1380);}
        s.b[1724] = (p.p10 == 1.0);s.store_scalar(1724, if s.b[1724] { 1.0 } else { 0.0 });s.b[1725] = (((s.v[1380]) as f64).abs() > 0.01);s.store_scalar(1725, if s.b[1725] { 1.0 } else { 0.0 });
        if ((s.b[1608] && s.b[1724]) && s.b[1725]) {s.store_mul(1351, 1460, 1529);}
        s.b[1726] = (((s.v[1458] - s.v[1529]) - s.v[1528]) < 80.0);s.store_scalar(1726, if s.b[1726] { 1.0 } else { 0.0 });
        if (((s.b[1608] && s.b[1724]) && s.b[1725]) && s.b[1726]) {s.store_exp_ad(1342, A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0));}
        if (((s.b[1608] && s.b[1724]) && s.b[1725]) && (!s.b[1726])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1342, A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1608] && s.b[1724]) && s.b[1725]) {s.store_mul(1352, 1433, 1342);s.store_sub_square_lhs(1353, 1351, 1352);s.store_add_scaled_product_indices(1354, 1352, 1.0, 1460, 1351, 2.0);s.store_add_scaled_product_indices(1355, 1352, (-1.0), 1460, 1460, 2.0);}
        s.b[1727] = (s.v[1353] < (-0.005));s.store_scalar(1727, if s.b[1727] { 1.0 } else { 0.0 });
        if (((s.b[1608] && s.b[1724]) && s.b[1725]) && s.b[1727]) {s.store_sqrt_abs_ad(1356, s.ad_value(1353));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_105(
        s: &mut Scratch,
    ) {
        if (((s.b[1608] && s.b[1724]) && s.b[1725]) && s.b[1727]) {s.store_div_mixed_ia(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);s.store_mul_add_mixed_iia(1358, 1342, 1353, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)));s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);s.store_mul_div_lhs(1363, 1354, 1353, 1343);s.store_div_mixed_ai(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);}
        s.b[1728] = (s.v[1353] > 0.005);s.store_scalar(1728, if s.b[1728] { 1.0 } else { 0.0 });
        if ((((s.b[1608] && s.b[1724]) && s.b[1725]) && (!s.b[1727])) && s.b[1728]) {s.store_sqrt_abs_ad(1356, s.ad_value(1353));s.store_exp_neg_input(1359, 1356);s.store_div_scaled_product_offset_rhs_mixed_iia(1357, 1356, 1359, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0);s.store_div_scaled_inputs_indices(1342, 1354, 0.25, 1353, 1.0);s.store_mul_add_mixed_iia(1358, 1342, 1353, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)));s.store_add_scaled_product_mixed_aai(1360, A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), 1342, 1.0);s.store_sub_from_scalar_scaled_input(1343, 1.0, 1357, 0.5);s.store_mul_div_lhs(1363, 1354, 1353, 1343);s.store_div_mixed_ai(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);}
        if ((((s.b[1608] && s.b[1724]) && s.b[1725]) && (!s.b[1727])) && (!s.b[1728])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1344, 1353, 1.0, 1353, 1.0, 1353, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(1357, 1353, 1344, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1342, 1353, 1.0, 1353, 1.0, 1353, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(1358, 1354, 1342);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1343, 1353, 1.0, 1353, 1.0, 1353, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(1360, 1355, 1342, 1.0, A::square(s.ad_value(1354)), 1343, (-1.0));s.store_scaled_mul(1363, 1354, 1344, (-0.5));s.store_add_scaled_product_mixed_aii(1364, A::mul3_scaled_output(s.ad_value(1354), s.ad_value(1354), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 2.0, A::scale(s.ad_value(1353), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1355, 1344, (-0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_106(
        s: &mut Scratch,
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
    pub(super) fn stamp_transient_block_107(
        s: &mut Scratch,
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
    pub(super) fn stamp_transient_block_108(
        s: &mut Scratch,
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
    pub(super) fn stamp_transient_block_109(
        s: &mut Scratch,
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
    pub(super) fn stamp_transient_block_110(
        s: &mut Scratch,
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
    pub(super) fn stamp_transient_block_111(
        s: &mut Scratch,
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
}
