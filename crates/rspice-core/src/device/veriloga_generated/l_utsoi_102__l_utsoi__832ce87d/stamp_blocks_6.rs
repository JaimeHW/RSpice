#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_96(
        s: &mut Scratch,
    ) {
        if s.b[1604] {s.store_offset_add_ad(1342, A::ln(A::div_scaled_product(A::add_scaled_product(s.ad_value(1457), 1.0, s.ad_value(1456), s.ad_value(1338), 1.0), s.ad_value(1428), 1.0, s.ad_value(1429), 1.0)), s.ad_value(1524), 3.0);}
        s.b[1689] = (((s.v[1341] - s.v[1432]) * 0.3333333333333) < 80.0);s.store_scalar(1689, if s.b[1689] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1689]) {s.store_ln_one_plus_exp_ad(1340, A::sub_scaled_inputs(s.ad_value(1341), 0.3333333333333, s.ad_value(1432), 0.3333333333333));}
        if (s.b[1604] && (!s.b[1689])) {s.store_scaled_sub(1340, 1341, 1432, 0.3333333333333);}
        if s.b[1604] {s.store_sub_scaled_inputs(1345, 1341, 1.0, 1340, 3.0);}
        s.b[1690] = (((s.v[1342] - s.v[1433]) * 0.3333333333333) < 80.0);s.store_scalar(1690, if s.b[1690] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1690]) {s.store_ln_one_plus_exp_ad(1340, A::sub_scaled_inputs(s.ad_value(1342), 0.3333333333333, s.ad_value(1433), 0.3333333333333));}
        if (s.b[1604] && (!s.b[1690])) {s.store_scaled_sub(1340, 1342, 1433, 0.3333333333333);}
        if s.b[1604] {s.store_sub_scaled_inputs(1346, 1342, 1.0, 1340, 3.0);s.store_mul_add_scaled_product_rhs_indices(1343, 1338, 1346, 1.0, 1456, 1454, 1.0);s.store_mul_add_scaled_product_rhs_indices(1344, 1339, 1345, 1.0, 1457, 1455, 1.0);}
        s.b[1691] = (((s.v[1341] - s.v[1343]) * 0.3333333333333) < 80.0);s.store_scalar(1691, if s.b[1691] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1691]) {s.store_ln_one_plus_exp_ad(1340, A::sub_scaled_inputs(s.ad_value(1341), 0.3333333333333, s.ad_value(1343), 0.3333333333333));}
        if (s.b[1604] && (!s.b[1691])) {s.store_scaled_sub(1340, 1341, 1343, 0.3333333333333);}
        if s.b[1604] {s.store_sub_scaled_inputs(1345, 1341, 1.0, 1340, 3.0);}
        s.b[1692] = (((s.v[1342] - s.v[1344]) * 0.3333333333333) < 80.0);s.store_scalar(1692, if s.b[1692] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1692]) {s.store_ln_one_plus_exp_ad(1340, A::sub_scaled_inputs(s.ad_value(1342), 0.3333333333333, s.ad_value(1344), 0.3333333333333));}
        if (s.b[1604] && (!s.b[1692])) {s.store_scaled_sub(1340, 1342, 1344, 0.3333333333333);}
        if s.b[1604] {s.store_sub_scaled_inputs(1346, 1342, 1.0, 1340, 3.0);s.store_sub(1525, 1454, 1345);s.store_sub(1526, 1455, 1346);s.store_scalar(1352, 0.0);s.store_scalar(1355, 0.0);s.store_mul(1347, 1456, 1525);}
        s.b[1693] = (((s.v[1454] - s.v[1525]) - s.v[1524]) < 80.0);s.store_scalar(1693, if s.b[1693] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1693]) {s.store_exp_ad(1338, A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0));}
        if (s.b[1604] && (!s.b[1693])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1338, A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1604] {s.store_mul(1348, 1429, 1338);s.store_sub_square_lhs(1349, 1347, 1348);s.store_add_scaled_product_indices(1350, 1348, 1.0, 1456, 1347, 2.0);s.store_add_scaled_product_indices(1351, 1348, (-1.0), 1456, 1456, 2.0);}
        s.b[1694] = (s.v[1349] < (-0.005));s.store_scalar(1694, if s.b[1694] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1694]) {s.store_sqrt_abs_ad(1352, s.ad_value(1349));s.store_div_mixed_ia(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);s.store_mul_add_mixed_iia(1354, 1338, 1349, A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)));s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);s.store_mul_div_lhs(1359, 1350, 1349, 1339);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_97(
        s: &mut Scratch,
    ) {
        if (s.b[1604] && s.b[1694]) {s.store_div_mixed_ai(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);}
        s.b[1695] = (s.v[1349] > 0.005);s.store_scalar(1695, if s.b[1695] { 1.0 } else { 0.0 });
        if ((s.b[1604] && (!s.b[1694])) && s.b[1695]) {s.store_sqrt_abs_ad(1352, s.ad_value(1349));s.store_exp_neg_input(1355, 1352);s.store_div_scaled_product_offset_rhs_mixed_iia(1353, 1352, 1355, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1355)), 1.0);s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);s.store_mul_add_mixed_iia(1354, 1338, 1349, A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)));s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);s.store_mul_div_lhs(1359, 1350, 1349, 1339);s.store_div_mixed_ai(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);}
        if ((s.b[1604] && (!s.b[1694])) && (!s.b[1695])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1340, 1349, 1.0, 1349, 1.0, 1349, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(1353, 1349, 1340, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1338, 1349, 1.0, 1349, 1.0, 1349, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(1354, 1350, 1338);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1339, 1349, 1.0, 1349, 1.0, 1349, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(1356, 1351, 1338, 1.0, A::square(s.ad_value(1350)), 1339, (-1.0));s.store_scaled_mul(1359, 1350, 1340, (-0.5));s.store_add_scaled_product_mixed_aii(1360, A::mul3_scaled_output(s.ad_value(1350), s.ad_value(1350), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 2.0, A::scale(s.ad_value(1349), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1351, 1340, (-0.5));}
        s.b[1696] = (s.v[1349] > 0.005);s.store_scalar(1696, if s.b[1696] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1696]) {s.store_div_scaled_inputs_mixed_ia(1339, 1349, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1355), 2.0, s.ad_value(1355))), 1.0);s.store_mul(1357, 1339, 1355);s.store_sub_ln_lhs(1358, 1339, 1352);}
        s.b[1697] = (s.v[1349] < (-0.005));s.store_scalar(1697, if s.b[1697] { 1.0 } else { 0.0 });
        if ((s.b[1604] && (!s.b[1696])) && s.b[1697]) {s.store_sin_scaled_input(1339, 1352, 0.5);s.store_div_scaled_inputs_square_rhs(1357, 1349, -1.0, 1339, 1.0);s.store_ln(1358, 1357);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_98(
        s: &mut Scratch,
    ) {
        if ((s.b[1604] && (!s.b[1696])) && (!s.b[1697])) {s.store_sub_from_scalar_ad(1357, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(1358, 1357);}
        s.b[1698] = (((1.01 * s.v[1347]) + s.v[1353]) > 0.0);s.store_scalar(1698, if s.b[1698] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1698]) {s.store_add(1361, 1347, 1353);s.store_add(1362, 1456, 1354);s.copy_ad(1363, 1356);}
        if (s.b[1604] && (!s.b[1698])) {s.store_div_from_scalar_sub_ad(1339, 1.0, s.ad_value(1347), s.ad_value(1353));s.store_sub(1340, 1354, 1456);s.store_mul_sub_lhs(1361, 1348, 1357, 1339);s.store_mul_mixed_ai(1362, A::add_scaled_value_products(s.ad_value(1348), (-1.0), s.ad_value(1340), s.ad_value(1361), 1.0, s.ad_value(1359), s.ad_value(1357), (-1.0)), 1339);s.store_mul_mixed_ai(1363, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1356), s.ad_value(1361), 1.0, s.ad_value(1340), s.ad_value(1362), 2.0), 1.0, s.ad_value(1348), 1.0, A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357), (-1.0)), 1339);}
        s.b[1699] = (s.v[1361] > 0.0);s.store_scalar(1699, if s.b[1699] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1699]) {s.store_ln(1364, 1361);s.store_div_from_scalar(1338, 1.0, 1361);s.store_mul(1365, 1362, 1338);s.store_add_scaled_square_product_indices(1366, 1365, (-1.0), 1363, 1338, 1.0);}
        if (s.b[1604] && (!s.b[1699])) {s.store_add_offset_lhs_mixed_ia(1364, 1347, 0.6931471805599, A::ln_scaled_input(s.ad_value(1347), -1.0));s.store_div_from_scalar(1338, 1.0, 1525);s.store_add(1365, 1456, 1338);s.store_mul_scale_offset_indices(1366, 1338, 1338, -1.0, 0.0);}
        if s.b[1604] {s.store_sub_add_scaled_inputs4_lhs_indices(1367, 1455, 1.0, 1454, (-1.0), 1525, 1.0, 1364, 2.0, 1358);s.store_sub_mixed_ai(1368, A::scale_offset(s.ad_value(1365), 2.0, 1.0), 1359);s.store_sub_scaled_inputs(1369, 1366, 2.0, 1360, 1.0);s.store_add_scaled_product_indices(1370, 1347, 1.0, 1457, 1367, 1.0);s.store_add_scaled_product_indices(1371, 1456, 1.0, 1457, 1368, 1.0);s.store_mul(1372, 1457, 1369);s.store_add_scaled_product_indices(1373, 1348, (-1.0), 1370, 1361, 1.0);s.store_add_mixed_ai(1374, A::add_scaled_products(s.ad_value(1371), s.ad_value(1361), 1.0, s.ad_value(1370), s.ad_value(1362), 1.0), 1348);s.store_sub_mixed_ai(1375, A::add_scaled_products3(s.ad_value(1372), s.ad_value(1361), 1.0, s.ad_value(1371), s.ad_value(1362), 2.0, s.ad_value(1370), s.ad_value(1363), 1.0), 1348);s.store_add_scaled_square_product_indices(1384, 1374, 1.0, 1373, 1375, (-0.5));s.store_div_scaled_product3_mixed_iiia(1376, 1373, 1374, 1384, -1.0, A::offset(A::square(s.ad_value(1384)), 1e-200), 1.0);s.store_add(1525, 1525, 1376);s.store_mul(1347, 1456, 1525);s.store_mul(1377, 1457, 1526);s.store_add(1370, 1347, 1377);s.store_offset_scaled(1378, 1370, 0.065345483024, 1.0);s.store_add_scaled_product_mixed_aii(1379, A::scale_offset(s.ad_value(1370), 8.5797362674, 39.478417604), 1.0, 1347, 1377, 1.0);s.store_add_scaled_product_indices(1380, 1370, (2.0 * 39.478417604), 1347, 1377, 39.478417604);s.store_sqrt_add_scaled_square_product(1381, 1379, 1.0, 1378, 1380, (-4.0));s.store_div_scaled_inputs2_indices(1349, 1381, 1.0, 1379, (-1.0), 1378, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_99(
        s: &mut Scratch,
    ) {
        if s.b[1604] {s.store_sub_square_lhs(1382, 1347, 1349);}
        s.b[1700] = (s.v[1382] > 0.0);s.store_scalar(1700, if s.b[1700] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1700]) {s.store_mul_add_scaled_inputs4_rhs_mixed_aiii(1373, 1382, A::ln(A::div(s.ad_value(1382), s.ad_value(1429))), 1.0, 1524, 1.0, 1454, -1.0, 1525, 1.0);s.store_add_scaled_product_indices(1374, 1382, 1.0, 1456, 1347, 2.0);}
        let (t2,) = {
    if (s.b[1604] && s.b[1700]) {
        let t0: f64 = (s.v[1454] - s.v[1525]);let t1: f64 = (t0 - s.v[1341]);
        (t1,)
    } else {
        (s.v[1383],)
    }
};
        s.store_scalar(1383, t2);s.b[1701] = ((((s.v[1373] < 0.0) && (s.v[1374] > 0.0)) && (((s.v[1383] + 2.3025850929941) + ((s.v[1456]) as f64).ln()) > 0.0)) || (s.v[1383] > 1.0));s.store_scalar(1701, if s.b[1701] { 1.0 } else { 0.0 });
        if ((s.b[1604] && s.b[1700]) && s.b[1701]) {s.store_sub_div_rhs_indices(1525, 1525, 1373, 1374);}
        if s.b[1604] {s.store_mul(1347, 1456, 1525);s.store_mul(1377, 1457, 1526);s.store_add(1370, 1347, 1377);s.store_offset_scaled(1378, 1370, 0.065345483024, 1.0);s.store_add_scaled_product_mixed_aii(1379, A::scale_offset(s.ad_value(1370), 8.5797362674, 39.478417604), 1.0, 1347, 1377, 1.0);s.store_add_scaled_product_indices(1380, 1370, (2.0 * 39.478417604), 1347, 1377, 39.478417604);s.store_sqrt_add_scaled_square_product(1381, 1379, 1.0, 1378, 1380, (-4.0));s.store_div_scaled_inputs2_indices(1349, 1381, 1.0, 1379, (-1.0), 1378, 2.0);}
        s.b[1702] = (s.v[1349] < (-0.005));s.store_scalar(1702, if s.b[1702] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1702]) {s.store_sqrt_abs_ad(1352, s.ad_value(1349));s.store_div_mixed_ia(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));s.store_div_scaled_inputs2_mixed_iai(1354, 1349, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)), 0.25, 1349, 1.0);}
        s.b[1703] = (s.v[1349] > 0.005);s.store_scalar(1703, if s.b[1703] { 1.0 } else { 0.0 });
        if ((s.b[1604] && (!s.b[1702])) && s.b[1703]) {s.store_sqrt_abs_ad(1352, s.ad_value(1349));s.store_exp_neg_input(1355, 1352);s.store_div_scaled_product_offset_rhs_mixed_iia(1353, 1352, 1355, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1355)), 1.0);s.store_div_scaled_inputs2_mixed_iai(1354, 1349, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)), 0.25, 1349, 1.0);}
        if ((s.b[1604] && (!s.b[1702])) && (!s.b[1703])) {s.store_offset_ad(1353, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1354, 1349, 1.0, 1349, 1.0, 1349, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);}
        if s.b[1604] {s.store_sub_mixed_ia(1349, 1349, A::div_scaled_inputs2(A::add_scaled_products(s.ad_value(1370), s.ad_value(1353), 1.0, s.ad_value(1347), s.ad_value(1377), 1.0), 1.0, s.ad_value(1349), 1.0, A::offset(A::mul(s.ad_value(1370), s.ad_value(1354)), 1.0), 1.0));s.store_sub_square_lhs(1382, 1347, 1349);}
        s.b[1704] = (s.v[1382] > 0.0);s.store_scalar(1704, if s.b[1704] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1704]) {s.store_mul_add_scaled_inputs4_rhs_mixed_aiii(1373, 1382, A::ln(A::div(s.ad_value(1382), s.ad_value(1429))), 1.0, 1524, 1.0, 1454, -1.0, 1525, 1.0);s.store_add_scaled_product_indices(1374, 1382, 1.0, 1456, 1347, 2.0);}
        let (t5,) = {
    if (s.b[1604] && s.b[1704]) {
        let t3: f64 = (s.v[1454] - s.v[1525]);let t4: f64 = (t3 - s.v[1341]);
        (t4,)
    } else {
        (s.v[1383],)
    }
};
        s.store_scalar(1383, t5);s.b[1705] = ((((s.v[1373] < 0.0) && (s.v[1374] > 0.0)) && (((s.v[1383] + 2.3025850929941) + ((s.v[1456]) as f64).ln()) > 0.0)) || (s.v[1383] > 1.0));s.store_scalar(1705, if s.b[1705] { 1.0 } else { 0.0 });
        if ((s.b[1604] && s.b[1704]) && s.b[1705]) {s.store_sub_div_rhs_indices(1525, 1525, 1373, 1374);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_100(
        s: &mut Scratch,
    ) {
        if s.b[1604] {s.store_mul(1347, 1456, 1525);}
        s.b[1706] = (((s.v[1454] - s.v[1525]) - s.v[1524]) < 80.0);s.store_scalar(1706, if s.b[1706] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1706]) {s.store_exp_ad(1338, A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0));}
        if (s.b[1604] && (!s.b[1706])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1338, A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1604] {s.store_mul(1348, 1429, 1338);s.store_sub_square_lhs(1349, 1347, 1348);s.store_add_scaled_product_indices(1350, 1348, 1.0, 1456, 1347, 2.0);s.store_add_scaled_product_indices(1351, 1348, (-1.0), 1456, 1456, 2.0);}
        s.b[1707] = (s.v[1349] < (-0.005));s.store_scalar(1707, if s.b[1707] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1707]) {s.store_sqrt_abs_ad(1352, s.ad_value(1349));s.store_div_mixed_ia(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);s.store_mul_add_mixed_iia(1354, 1338, 1349, A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)));s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);s.store_mul_div_lhs(1359, 1350, 1349, 1339);s.store_div_mixed_ai(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);}
        s.b[1708] = (s.v[1349] > 0.005);s.store_scalar(1708, if s.b[1708] { 1.0 } else { 0.0 });
        if ((s.b[1604] && (!s.b[1707])) && s.b[1708]) {s.store_sqrt_abs_ad(1352, s.ad_value(1349));s.store_exp_neg_input(1355, 1352);s.store_div_scaled_product_offset_rhs_mixed_iia(1353, 1352, 1355, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1355)), 1.0);s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);s.store_mul_add_mixed_iia(1354, 1338, 1349, A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)));s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);s.store_mul_div_lhs(1359, 1350, 1349, 1339);s.store_div_mixed_ai(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);}
        if ((s.b[1604] && (!s.b[1707])) && (!s.b[1708])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1340, 1349, 1.0, 1349, 1.0, 1349, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(1353, 1349, 1340, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_101(
        s: &mut Scratch,
    ) {
        if ((s.b[1604] && (!s.b[1707])) && (!s.b[1708])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1338, 1349, 1.0, 1349, 1.0, 1349, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(1354, 1350, 1338);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1339, 1349, 1.0, 1349, 1.0, 1349, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(1356, 1351, 1338, 1.0, A::square(s.ad_value(1350)), 1339, (-1.0));s.store_scaled_mul(1359, 1350, 1340, (-0.5));s.store_add_scaled_product_mixed_aii(1360, A::mul3_scaled_output(s.ad_value(1350), s.ad_value(1350), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 2.0, A::scale(s.ad_value(1349), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1351, 1340, (-0.5));}
        s.b[1709] = (s.v[1349] > 0.005);s.store_scalar(1709, if s.b[1709] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1709]) {s.store_div_scaled_inputs_mixed_ia(1339, 1349, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1355), 2.0, s.ad_value(1355))), 1.0);s.store_mul(1357, 1339, 1355);s.store_sub_ln_lhs(1358, 1339, 1352);}
        s.b[1710] = (s.v[1349] < (-0.005));s.store_scalar(1710, if s.b[1710] { 1.0 } else { 0.0 });
        if ((s.b[1604] && (!s.b[1709])) && s.b[1710]) {s.store_sin_scaled_input(1339, 1352, 0.5);s.store_div_scaled_inputs_square_rhs(1357, 1349, -1.0, 1339, 1.0);s.store_ln(1358, 1357);}
        if ((s.b[1604] && (!s.b[1709])) && (!s.b[1710])) {s.store_sub_from_scalar_ad(1357, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(1358, 1357);}
        s.b[1711] = (((1.01 * s.v[1347]) + s.v[1353]) > 0.0);s.store_scalar(1711, if s.b[1711] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1711]) {s.store_add(1361, 1347, 1353);s.store_add(1362, 1456, 1354);s.copy_ad(1363, 1356);}
        if (s.b[1604] && (!s.b[1711])) {s.store_div_from_scalar_sub_ad(1339, 1.0, s.ad_value(1347), s.ad_value(1353));s.store_sub(1340, 1354, 1456);s.store_mul_sub_lhs(1361, 1348, 1357, 1339);s.store_mul_mixed_ai(1362, A::add_scaled_value_products(s.ad_value(1348), (-1.0), s.ad_value(1340), s.ad_value(1361), 1.0, s.ad_value(1359), s.ad_value(1357), (-1.0)), 1339);s.store_mul_mixed_ai(1363, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1356), s.ad_value(1361), 1.0, s.ad_value(1340), s.ad_value(1362), 2.0), 1.0, s.ad_value(1348), 1.0, A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357), (-1.0)), 1339);}
        s.b[1712] = (s.v[1361] > 0.0);s.store_scalar(1712, if s.b[1712] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1712]) {s.store_ln(1364, 1361);s.store_div_from_scalar(1338, 1.0, 1361);s.store_mul(1365, 1362, 1338);s.store_add_scaled_square_product_indices(1366, 1365, (-1.0), 1363, 1338, 1.0);}
        if (s.b[1604] && (!s.b[1712])) {s.store_add_offset_lhs_mixed_ia(1364, 1347, 0.6931471805599, A::ln_scaled_input(s.ad_value(1347), -1.0));s.store_div_from_scalar(1338, 1.0, 1525);s.store_add(1365, 1456, 1338);s.store_mul_scale_offset_indices(1366, 1338, 1338, -1.0, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_102(
        s: &mut Scratch,
    ) {
        if s.b[1604] {s.store_sub_add_scaled_inputs4_lhs_indices(1367, 1455, 1.0, 1454, (-1.0), 1525, 1.0, 1364, 2.0, 1358);s.store_sub_mixed_ai(1368, A::scale_offset(s.ad_value(1365), 2.0, 1.0), 1359);s.store_sub_scaled_inputs(1369, 1366, 2.0, 1360, 1.0);s.store_add_scaled_product_indices(1370, 1347, 1.0, 1457, 1367, 1.0);s.store_add_scaled_product_indices(1371, 1456, 1.0, 1457, 1368, 1.0);s.store_mul(1372, 1457, 1369);s.store_add_scaled_product_indices(1373, 1348, (-1.0), 1370, 1361, 1.0);s.store_add_mixed_ai(1374, A::add_scaled_products(s.ad_value(1371), s.ad_value(1361), 1.0, s.ad_value(1370), s.ad_value(1362), 1.0), 1348);s.store_sub_mixed_ai(1375, A::add_scaled_products3(s.ad_value(1372), s.ad_value(1361), 1.0, s.ad_value(1371), s.ad_value(1362), 2.0, s.ad_value(1370), s.ad_value(1363), 1.0), 1348);s.store_add_scaled_square_product_indices(1384, 1374, 1.0, 1373, 1375, (-0.5));s.store_div_scaled_product3_mixed_iiia(1376, 1373, 1374, 1384, -1.0, A::offset(A::square(s.ad_value(1384)), 1e-200), 1.0);s.store_add(1525, 1525, 1376);s.store_mul(1347, 1456, 1525);}
        s.b[1713] = (((s.v[1454] - s.v[1525]) - s.v[1524]) < 80.0);s.store_scalar(1713, if s.b[1713] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1713]) {s.store_exp_ad(1338, A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0));}
        if (s.b[1604] && (!s.b[1713])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1338, A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1604] {s.store_mul(1348, 1429, 1338);s.store_sub_square_lhs(1349, 1347, 1348);s.store_add_scaled_product_indices(1350, 1348, 1.0, 1456, 1347, 2.0);s.store_add_scaled_product_indices(1351, 1348, (-1.0), 1456, 1456, 2.0);}
        s.b[1714] = (s.v[1349] < (-0.005));s.store_scalar(1714, if s.b[1714] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1714]) {s.store_sqrt_abs_ad(1352, s.ad_value(1349));s.store_div_mixed_ia(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);s.store_mul_add_mixed_iia(1354, 1338, 1349, A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)));s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);s.store_mul_div_lhs(1359, 1350, 1349, 1339);s.store_div_mixed_ai(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);}
        s.b[1715] = (s.v[1349] > 0.005);s.store_scalar(1715, if s.b[1715] { 1.0 } else { 0.0 });
        if ((s.b[1604] && (!s.b[1714])) && s.b[1715]) {s.store_sqrt_abs_ad(1352, s.ad_value(1349));s.store_exp_neg_input(1355, 1352);s.store_div_scaled_product_offset_rhs_mixed_iia(1353, 1352, 1355, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1355)), 1.0);s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_103(
        s: &mut Scratch,
    ) {
        if ((s.b[1604] && (!s.b[1714])) && s.b[1715]) {s.store_mul_add_mixed_iia(1354, 1338, 1349, A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)));s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);s.store_mul_div_lhs(1359, 1350, 1349, 1339);s.store_div_mixed_ai(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);}
        if ((s.b[1604] && (!s.b[1714])) && (!s.b[1715])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1340, 1349, 1.0, 1349, 1.0, 1349, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(1353, 1349, 1340, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1338, 1349, 1.0, 1349, 1.0, 1349, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(1354, 1350, 1338);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1339, 1349, 1.0, 1349, 1.0, 1349, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(1356, 1351, 1338, 1.0, A::square(s.ad_value(1350)), 1339, (-1.0));s.store_scaled_mul(1359, 1350, 1340, (-0.5));s.store_add_scaled_product_mixed_aii(1360, A::mul3_scaled_output(s.ad_value(1350), s.ad_value(1350), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 2.0, A::scale(s.ad_value(1349), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1351, 1340, (-0.5));}
        s.b[1716] = (s.v[1349] > 0.005);s.store_scalar(1716, if s.b[1716] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1716]) {s.store_div_scaled_inputs_mixed_ia(1339, 1349, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1355), 2.0, s.ad_value(1355))), 1.0);s.store_mul(1357, 1339, 1355);s.store_sub_ln_lhs(1358, 1339, 1352);}
        s.b[1717] = (s.v[1349] < (-0.005));s.store_scalar(1717, if s.b[1717] { 1.0 } else { 0.0 });
        if ((s.b[1604] && (!s.b[1716])) && s.b[1717]) {s.store_sin_scaled_input(1339, 1352, 0.5);s.store_div_scaled_inputs_square_rhs(1357, 1349, -1.0, 1339, 1.0);s.store_ln(1358, 1357);}
        if ((s.b[1604] && (!s.b[1716])) && (!s.b[1717])) {s.store_sub_from_scalar_ad(1357, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(1358, 1357);}
        s.b[1718] = (((1.01 * s.v[1347]) + s.v[1353]) > 0.0);s.store_scalar(1718, if s.b[1718] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1718]) {s.store_add(1361, 1347, 1353);s.store_add(1362, 1456, 1354);s.copy_ad(1363, 1356);}
        if (s.b[1604] && (!s.b[1718])) {s.store_div_from_scalar_sub_ad(1339, 1.0, s.ad_value(1347), s.ad_value(1353));s.store_sub(1340, 1354, 1456);s.store_mul_sub_lhs(1361, 1348, 1357, 1339);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_104(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1604] && (!s.b[1718])) {s.store_mul_mixed_ai(1362, A::add_scaled_value_products(s.ad_value(1348), (-1.0), s.ad_value(1340), s.ad_value(1361), 1.0, s.ad_value(1359), s.ad_value(1357), (-1.0)), 1339);s.store_mul_mixed_ai(1363, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1356), s.ad_value(1361), 1.0, s.ad_value(1340), s.ad_value(1362), 2.0), 1.0, s.ad_value(1348), 1.0, A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357), (-1.0)), 1339);}
        s.b[1719] = (s.v[1361] > 0.0);s.store_scalar(1719, if s.b[1719] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1719]) {s.store_ln(1364, 1361);s.store_div_from_scalar(1338, 1.0, 1361);s.store_mul(1365, 1362, 1338);s.store_add_scaled_square_product_indices(1366, 1365, (-1.0), 1363, 1338, 1.0);}
        if (s.b[1604] && (!s.b[1719])) {s.store_add_offset_lhs_mixed_ia(1364, 1347, 0.6931471805599, A::ln_scaled_input(s.ad_value(1347), -1.0));s.store_div_from_scalar(1338, 1.0, 1525);s.store_add(1365, 1456, 1338);s.store_mul_scale_offset_indices(1366, 1338, 1338, -1.0, 0.0);}
        if s.b[1604] {s.store_sub_add_scaled_inputs4_lhs_indices(1367, 1455, 1.0, 1454, (-1.0), 1525, 1.0, 1364, 2.0, 1358);s.store_sub_mixed_ai(1368, A::scale_offset(s.ad_value(1365), 2.0, 1.0), 1359);s.store_sub_scaled_inputs(1369, 1366, 2.0, 1360, 1.0);s.store_add_scaled_product_indices(1370, 1347, 1.0, 1457, 1367, 1.0);s.store_add_scaled_product_indices(1371, 1456, 1.0, 1457, 1368, 1.0);s.store_mul(1372, 1457, 1369);s.store_add_scaled_product_indices(1373, 1348, (-1.0), 1370, 1361, 1.0);s.store_add_mixed_ai(1374, A::add_scaled_products(s.ad_value(1371), s.ad_value(1361), 1.0, s.ad_value(1370), s.ad_value(1362), 1.0), 1348);s.store_sub_mixed_ai(1375, A::add_scaled_products3(s.ad_value(1372), s.ad_value(1361), 1.0, s.ad_value(1371), s.ad_value(1362), 2.0, s.ad_value(1370), s.ad_value(1363), 1.0), 1348);s.store_add_scaled_square_product_indices(1384, 1374, 1.0, 1373, 1375, (-0.5));s.store_div_scaled_product3_mixed_iiia(1376, 1373, 1374, 1384, -1.0, A::offset(A::square(s.ad_value(1384)), 1e-200), 1.0);s.store_add(1525, 1525, 1376);}
        s.b[1720] = (p.p10 == 1.0);s.store_scalar(1720, if s.b[1720] { 1.0 } else { 0.0 });s.b[1721] = (((s.v[1376]) as f64).abs() > 0.01);s.store_scalar(1721, if s.b[1721] { 1.0 } else { 0.0 });
        if ((s.b[1604] && s.b[1720]) && s.b[1721]) {s.store_mul(1347, 1456, 1525);}
        s.b[1722] = (((s.v[1454] - s.v[1525]) - s.v[1524]) < 80.0);s.store_scalar(1722, if s.b[1722] { 1.0 } else { 0.0 });
        if (((s.b[1604] && s.b[1720]) && s.b[1721]) && s.b[1722]) {s.store_exp_ad(1338, A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0));}
        if (((s.b[1604] && s.b[1720]) && s.b[1721]) && (!s.b[1722])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1338, A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1604] && s.b[1720]) && s.b[1721]) {s.store_mul(1348, 1429, 1338);s.store_sub_square_lhs(1349, 1347, 1348);s.store_add_scaled_product_indices(1350, 1348, 1.0, 1456, 1347, 2.0);s.store_add_scaled_product_indices(1351, 1348, (-1.0), 1456, 1456, 2.0);}
        s.b[1723] = (s.v[1349] < (-0.005));s.store_scalar(1723, if s.b[1723] { 1.0 } else { 0.0 });
        if (((s.b[1604] && s.b[1720]) && s.b[1721]) && s.b[1723]) {s.store_sqrt_abs_ad(1352, s.ad_value(1349));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_105(
        s: &mut Scratch,
    ) {
        if (((s.b[1604] && s.b[1720]) && s.b[1721]) && s.b[1723]) {s.store_div_mixed_ia(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);s.store_mul_add_mixed_iia(1354, 1338, 1349, A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)));s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);s.store_mul_div_lhs(1359, 1350, 1349, 1339);s.store_div_mixed_ai(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);}
        s.b[1724] = (s.v[1349] > 0.005);s.store_scalar(1724, if s.b[1724] { 1.0 } else { 0.0 });
        if ((((s.b[1604] && s.b[1720]) && s.b[1721]) && (!s.b[1723])) && s.b[1724]) {s.store_sqrt_abs_ad(1352, s.ad_value(1349));s.store_exp_neg_input(1355, 1352);s.store_div_scaled_product_offset_rhs_mixed_iia(1353, 1352, 1355, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1355)), 1.0);s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);s.store_mul_add_mixed_iia(1354, 1338, 1349, A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)));s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);s.store_mul_div_lhs(1359, 1350, 1349, 1339);s.store_div_mixed_ai(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);}
        if ((((s.b[1604] && s.b[1720]) && s.b[1721]) && (!s.b[1723])) && (!s.b[1724])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1340, 1349, 1.0, 1349, 1.0, 1349, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(1353, 1349, 1340, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1338, 1349, 1.0, 1349, 1.0, 1349, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(1354, 1350, 1338);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1339, 1349, 1.0, 1349, 1.0, 1349, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(1356, 1351, 1338, 1.0, A::square(s.ad_value(1350)), 1339, (-1.0));s.store_scaled_mul(1359, 1350, 1340, (-0.5));s.store_add_scaled_product_mixed_aii(1360, A::mul3_scaled_output(s.ad_value(1350), s.ad_value(1350), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 2.0, A::scale(s.ad_value(1349), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1351, 1340, (-0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_106(
        s: &mut Scratch,
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
    pub(super) fn stamp_transient_block_107(
        s: &mut Scratch,
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
    pub(super) fn stamp_transient_block_108(
        s: &mut Scratch,
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
    pub(super) fn stamp_transient_block_109(
        s: &mut Scratch,
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
    pub(super) fn stamp_transient_block_110(
        s: &mut Scratch,
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
    pub(super) fn stamp_transient_block_111(
        s: &mut Scratch,
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
}
