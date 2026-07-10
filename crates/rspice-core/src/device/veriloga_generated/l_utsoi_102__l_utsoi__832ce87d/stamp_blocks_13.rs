#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_86(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1604] && (!s.b[1662])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1338, A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1604] {s.store_mul(1467, 1429, 1338);s.store_sub_square_lhs(1466, 1463, 1467);}
        s.b[1663] = (s.v[1467] <= 0.0);s.store_scalar(1663, if s.b[1663] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1663]) {s.store_scalar(1462, 1e-80);s.store_sub(1464, 1462, 1463);s.store_div(1465, 1464, 1457);}
        s.b[1664] = (s.v[1466] < (-0.005));s.store_scalar(1664, if s.b[1664] { 1.0 } else { 0.0 });
        if ((s.b[1604] && (!s.b[1663])) && s.b[1664]) {s.store_sqrt_abs_ad(1352, s.ad_value(1466));s.store_div_mixed_ia(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));}
        s.b[1665] = (s.v[1466] > 0.005);s.store_scalar(1665, if s.b[1665] { 1.0 } else { 0.0 });
        if (((s.b[1604] && (!s.b[1663])) && (!s.b[1664])) && s.b[1665]) {s.store_sqrt_abs_ad(1352, s.ad_value(1466));s.store_exp_neg_input(1355, 1352);s.store_div_scaled_product_offset_rhs_mixed_iia(1353, 1352, 1355, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1355)), 1.0);}
        if (((s.b[1604] && (!s.b[1663])) && (!s.b[1664])) && (!s.b[1665])) {s.store_offset_ad(1353, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1466), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1466), 1.0, A::scale(s.ad_value(1466), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);}
        s.b[1666] = (((1.01 * s.v[1463]) + s.v[1353]) > 0.0);s.store_scalar(1666, if s.b[1666] { 1.0 } else { 0.0 });
        if ((s.b[1604] && (!s.b[1663])) && s.b[1666]) {s.store_add(1338, 1463, 1353);}
        s.b[1667] = ((s.v[1467] * s.v[1463]) < (((0.9 * s.v[1463]) * s.v[1463]) * s.v[1338]));s.store_scalar(1667, if s.b[1667] { 1.0 } else { 0.0 });
        if (((s.b[1604] && (!s.b[1663])) && s.b[1666]) && s.b[1667]) {s.store_offset_div(1462, 1467, 1338, 1e-80);s.store_sub(1464, 1462, 1463);s.store_div(1465, 1464, 1457);}
        s.b[1668] = (s.v[1466] > 0.005);s.store_scalar(1668, if s.b[1668] { 1.0 } else { 0.0 });
        if ((((s.b[1604] && (!s.b[1663])) && s.b[1666]) && (!s.b[1667])) && s.b[1668]) {s.store_sub_mixed_ai(1339, A::ln(A::div_scaled_inputs(s.ad_value(1466), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1355), 2.0, s.ad_value(1355))), 1.0)), 1352);}
        s.b[1669] = (s.v[1466] < (-0.005));s.store_scalar(1669, if s.b[1669] { 1.0 } else { 0.0 });
        if (((((s.b[1604] && (!s.b[1663])) && s.b[1666]) && (!s.b[1667])) && (!s.b[1668])) && s.b[1669]) {s.store_sin_scaled_input(1340, 1352, 0.5);s.store_ln_div_scaled_input_square_denominator(1339, 1466, -1.0, 1340, 1.0);}
        if (((((s.b[1604] && (!s.b[1663])) && s.b[1666]) && (!s.b[1667])) && (!s.b[1668])) && (!s.b[1669])) {s.store_ln_ad(1339, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1466), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1466), 1.0, A::scale(s.ad_value(1466), 0.0396825396825397), 0.05), 0.3333333333333)));}
        if (((s.b[1604] && (!s.b[1663])) && s.b[1666]) && (!s.b[1667])) {s.store_sub_add_scaled_inputs4_lhs_mixed_iiia(1465, 1455, 1.0, 1454, (-1.0), 1461, 1.0, A::ln(s.ad_value(1338)), 2.0, 1339);s.store_mul(1464, 1457, 1465);s.store_add(1462, 1463, 1464);}
        s.b[1670] = (s.v[1466] > 0.005);s.store_scalar(1670, if s.b[1670] { 1.0 } else { 0.0 });s.b[1671] = (((s.v[1461] - s.v[1454]) - s.v[1352]) < 80.0);s.store_scalar(1671, if s.b[1671] { 1.0 } else { 0.0 });
        if ((((s.b[1604] && (!s.b[1663])) && (!s.b[1666])) && s.b[1670]) && s.b[1671]) {s.store_exp_ad(1340, A::add_scaled_inputs3(s.ad_value(1461), 1.0, s.ad_value(1454), (-1.0), s.ad_value(1352), -1.0));}
        if ((((s.b[1604] && (!s.b[1663])) && (!s.b[1666])) && s.b[1670]) && (!s.b[1671])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1340, A::add_scaled_inputs3(s.ad_value(1461), 1.0, s.ad_value(1454), (-1.0), s.ad_value(1352), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if (((s.b[1604] && (!s.b[1663])) && (!s.b[1666])) && s.b[1670]) {s.store_div(1339, 1340, 1429);s.store_div_scaled_product_mixed_iia(1338, 1466, 1339, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1355), 2.0, s.ad_value(1355))), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_87(
        s: &mut ReactiveScratch,
    ) {
        s.b[1672] = (s.v[1466] < (-0.005));s.store_scalar(1672, if s.b[1672] { 1.0 } else { 0.0 });
        if ((((s.b[1604] && (!s.b[1663])) && (!s.b[1666])) && (!s.b[1670])) && s.b[1672]) {s.store_sin_scaled_input(1339, 1352, 0.5);s.store_div_scaled_value_by_product_mixed_iai(1338, 1466, -1.0, A::square(s.ad_value(1339)), 1467, 1.0);}
        if ((((s.b[1604] && (!s.b[1663])) && (!s.b[1666])) && (!s.b[1670])) && (!s.b[1672])) {s.store_div_mixed_ai(1338, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1466), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1466), 1.0, A::scale(s.ad_value(1466), 0.0396825396825397), 0.05), 0.3333333333333)), 1467);}
        if ((s.b[1604] && (!s.b[1663])) && (!s.b[1666])) {s.store_offset_div_scaled_inputs2_mixed_iia(1462, 1463, 1.0, 1353, (-1.0), A::sub_from_scalar(1.0, s.ad_value(1338)), 1.0, 1e-80);s.store_sub(1464, 1462, 1463);s.store_div(1465, 1464, 1457);}
        s.b[1673] = ((s.v[1455] - s.v[1465]) < 80.0);s.store_scalar(1673, if s.b[1673] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1673]) {s.store_exp_sub(1338, 1455, 1465);}
        if (s.b[1604] && (!s.b[1673])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1338, A::sub(s.ad_value(1455), s.ad_value(1465)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1604] {s.store_mul(1468, 1429, 1338);s.store_scalar(1471, 0.0);s.store_scalar(1472, 0.0);s.store_scalar(1469, 0.0);s.store_scalar(1470, 0.0);s.store_scalar(1473, 0.0);s.store_scalar(1474, 0.0);}
        s.b[1674] = (s.v[1462] > 1e-6);s.store_scalar(1674, if s.b[1674] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1674]) {s.store_mul(1469, 1467, 1430);s.store_mul(1470, 1468, 1431);s.store_add_scaled_inputs(1471, 1469, 1.0, 1463, 2.0);s.store_add_scaled_inputs(1472, 1470, 1.0, 1464, 2.0);s.store_add_scaled_inputs3_indices(1473, 1462, 2.0, 1469, 1.0, 1470, 1.0);}
        s.b[1675] = (((s.v[1466]) as f64).abs() > 0.005);s.store_scalar(1675, if s.b[1675] { 1.0 } else { 0.0 });
        if ((s.b[1604] && s.b[1674]) && s.b[1675]) {s.store_add_scaled_products3_mixed_iiaiai(2, 1471, 1472, 1.0, A::offset(s.ad_value(1461), 2.0), 1472, 2.0, A::offset(s.ad_value(1465), 2.0), 1471, 2.0);s.store_div_scaled_product_by_product_indices(1474, 1466, 1473, (-4.0), 1462, 2, 1.0);}
        if ((s.b[1604] && s.b[1674]) && (!s.b[1675])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2, 1466, 1.0, 1466, 1.0, 1466, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_add_scaled_products3_mixed_iiiiaa(3, 1471, 1467, 1.0, 1472, 1468, 1.0, A::mul3(s.ad_value(1471), s.ad_value(1472), s.ad_value(1462)), A::offset(A::mul(s.ad_value(1462), s.ad_value(2)), 1.0), 1.0);s.store_div_scaled_product3_by_product_indices(1474, 1467, 1468, 1473, 1.0, 1462, 3, 1.0);}
        if s.b[1604] {s.store_ln(1475, 1462);}
        s.b[1676] = ((s.v[1463] / 2.0) < 80.0);s.store_scalar(1676, if s.b[1676] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1676]) {s.store_ln_one_plus_exp_scaled_input(2, 1463, 0.5);}
        if (s.b[1604] && (!s.b[1676])) {s.store_scale(2, 1463, 0.5);}
        if s.b[1604] {s.store_scale(1476, 2, 2.0);}
        s.b[1677] = ((s.v[1464] / 2.0) < 80.0);s.store_scalar(1677, if s.b[1677] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1677]) {s.store_ln_one_plus_exp_scaled_input(3, 1464, 0.5);}
        if (s.b[1604] && (!s.b[1677])) {s.store_scale(3, 1464, 0.5);}
        if s.b[1604] {s.store_scale(1477, 3, 2.0);s.store_sub(1478, 1477, 1464);s.store_sub(1479, 1476, 1463);s.store_add_scaled_products_indices(1480, 266, 1476, 1.0, 267, 1478, 1.0);s.store_add_scaled_products_indices(1481, 266, 1477, 1.0, 267, 1479, 1.0);s.store_div_add_scaled_inputs_rhs_indices(0, 1462, 1476, 1.0, 1477, 1.0);s.store_mul(1482, 1476, 0);s.store_mul(1483, 1477, 0);s.store_mul_ad_product_rhs_mixed_ia(1484, 1476, 187, A::exp(A::mul(s.ad_value(40), s.ad_value(291))));s.store_mul_ad_product_rhs_mixed_ia(1485, 1477, 188, A::exp(A::mul(s.ad_value(40), s.ad_value(291))));s.store_mul_add_scaled_product_rhs_indices(2, 50, 1478, 1.0, 51, 1479, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_88(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1604] {s.store_scaled_add_sqrt_square_offset_ad(3, A::offset(s.ad_value(2), 1.0), 0.01, 0.5);s.store_scaled_add_sqrt_square_offset_ad(4, A::scale_offset(s.ad_value(2), 0.2, 1.0), 0.01, 0.5);s.store_div(1486, 3, 4);s.store_mul_ad_product_rhs(1487, 33, A::add_scaled_product(A::offset(A::mul(s.ad_value(41), s.ad_value(1478)), 1.0), 1.0, s.ad_value(42), s.ad_value(1479), 1.0), A::exp(A::mul_scaled_lhs(s.ad_value(44), -1.0, A::ln(A::add_scaled_product(A::offset(A::mul(s.ad_value(1482), s.ad_value(264)), 1.0), 1.0, s.ad_value(1483), s.ad_value(265), 1.0)))));}
        s.b[1678] = (s.v[56] == 0.0);s.store_scalar(1678, if s.b[1678] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1678]) {s.store_scalar(4, 1.0);}
        s.b[1679] = (s.v[56] < 0.0);s.store_scalar(1679, if s.b[1679] { 1.0 } else { 0.0 });
        if ((s.b[1604] && (!s.b[1678])) && s.b[1679]) {s.store_mul_exp_mixed_ia(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(1462), 1e-12))));s.store_sub_from_scalar(4, 1.0, 2);}
        if ((s.b[1604] && (!s.b[1678])) && (!s.b[1679])) {s.store_mul_exp_mixed_ia(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(1462), 1e-12))));s.store_div_from_scalar_offset_input(4, 1.0, 2, 1.0);}
        if s.b[1604] {s.store_mul_ad_affine_product_rhs(1488, 268, s.ad_value(1443), A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(1424))), A::sqrt_square_offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(1424))), 0.01)), 0.5, 0.0);s.store_mul_add_scaled_product_rhs_indices(1489, 1488, 54, 1.0, 1462, 4, 1.0);s.store_add_scaled_inputs_product_mixed_aiii(1490, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(263), s.ad_value(1480)), 1e-6)))), 1.0), 1.0, 1487, 1.0, 38, 1489, 1.0);s.store_add_scaled_inputs_product_mixed_aiii(1491, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(263), s.ad_value(1481)), 1e-6)))), 1.0), 1.0, 1487, 1.0, 39, 1489, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_89(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1604] {s.store_div_scaled_product_mixed_iaa(1492, 1486, A::add(s.ad_value(1484), s.ad_value(1485)), 1.0, A::add(A::div(s.ad_value(1484), s.ad_value(1490)), A::div(s.ad_value(1485), s.ad_value(1491))), 1.0);}
        s.b[1680] = (((s.v[1459]) as f64).abs() > 0.007);s.store_scalar(1680, if s.b[1680] { 1.0 } else { 0.0 });s.b[1681] = (s.v[1459] > 0.0);s.store_scalar(1681, if s.b[1681] { 1.0 } else { 0.0 });
        if ((s.b[1604] && s.b[1680]) && s.b[1681]) {s.store_exp_neg_input(0, 1459);s.store_div_mixed_ia(1493, 1459, A::sub_from_scalar(1.0, s.ad_value(0)));s.store_mul(1494, 0, 1493);s.store_add_offset_lhs_mixed_ai(1495, A::ln(A::div(s.ad_value(1429), A::mul(s.ad_value(1462), s.ad_value(1493)))), (-0.6931471805599), 1432);}
        if ((s.b[1604] && s.b[1680]) && (!s.b[1681])) {s.store_exp(0, 1459);s.store_div_scaled_value_offset_denominator(1494, s.ad_value(1459), 1.0, s.ad_value(0), (-1.0), 1.0);s.store_mul(1493, 0, 1494);s.store_add_offset_lhs_mixed_ai(1495, A::ln(A::div(s.ad_value(1429), A::mul(s.ad_value(1462), s.ad_value(1494)))), (-0.6931471805599), 1433);}
        if (s.b[1604] && s.b[1680]) {s.store_div_scaled_inputs_mixed_ia(1496, 1459, -1.0, A::mul(s.ad_value(1458), A::add_scaled_sub_value_product(1.0, s.ad_value(1493), 1.0, s.ad_value(1459), s.ad_value(1431), (-1.0))), 1.0);s.store_div_scaled_value_by_product_mixed_iia(1497, 1459, 1.0, 1458, A::add_scaled_sub_value_product(1.0, s.ad_value(1494), 1.0, s.ad_value(1459), s.ad_value(1430), 1.0), 1.0);s.store_div_add_scaled_inputs_rhs_ad(1498, 1459, A::div_scaled_offset_numerator(A::mul(s.ad_value(1494), s.ad_value(1431)), 1.0, 0.5, s.ad_value(1497), 1.0), 1.0, A::div_scaled_offset_numerator(A::mul(s.ad_value(1493), s.ad_value(1430)), 1.0, 0.5, s.ad_value(1496), 1.0), -1.0);}
        if (s.b[1604] && (!s.b[1680])) {s.store_scale(0, 1460, (0.5 * 0.1666666666667));s.store_scale(2, 1459, 0.5);s.store_add_offset_lhs(1493, 2, 1.0, 0);s.store_add_mixed_ai(1494, A::sub_from_scalar(1.0, s.ad_value(2)), 0);s.store_scale(3, 2, 0.1666666666667);s.store_div_scalar_by_product_mixed_ia(1496, 1.0, 1458, A::add(A::offset(s.ad_value(1431), 0.5), s.ad_value(3)), 1.0);s.store_div_scalar_by_product_mixed_ia(1497, 1.0, 1458, A::sub(A::offset(s.ad_value(1430), 0.5), s.ad_value(3)), 1.0);s.store_add_scaled_inputs3_offset_mixed_aii(1495, A::ln(A::div(s.ad_value(1429), A::mul_sub_from_scalar_rhs(s.ad_value(1462), 1.0, A::scale(s.ad_value(0), 0.5)))), 1.0, 1432, 0.5, 1433, 0.5, (-0.6931471805599));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_90(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1604] && (!s.b[1680])) {s.store_div_from_scalar_ad(1498, (-12.0), A::add_scaled_inputs4_offset(s.ad_value(1458), ((-1.0) * 3.0), A::div_scaled_inputs(s.ad_value(1458), 12.0, A::mul(s.ad_value(1456), s.ad_value(1457)), 1.0), 1.0, A::mul3(s.ad_value(1458), A::sub(s.ad_value(1430), s.ad_value(1431)), s.ad_value(1459)), 1.0, A::mul_sub_from_scalar_lhs_scaled_output(0.2, A::scale(s.ad_value(1458), 0.25), s.ad_value(1460), 0.3333333333333), 1.0, 4.0));}
        if s.b[1604] {s.store_div_from_scalar(1499, 1.0, 1498);}
        s.b[1682] = (s.v[1462] > 1e-6);s.store_scalar(1682, if s.b[1682] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1682]) {s.store_div_scaled_value_offset_denominator(1500, s.ad_value(1476), 100.0, s.ad_value(1476), 100.0, 1.0);}
        s.b[1683] = (s.v[61] < 0.0);s.store_scalar(1683, if s.b[1683] { 1.0 } else { 0.0 });
        if ((s.b[1604] && s.b[1682]) && s.b[1683]) {s.store_div_from_scalar_sub_from_scalar_ad(1501, 1.0, 1.0, A::mul(s.ad_value(61), s.ad_value(1500)));}
        if ((s.b[1604] && s.b[1682]) && (!s.b[1683])) {s.store_offset_mul(1501, 61, 1500, 1.0);}
        if (s.b[1604] && s.b[1682]) {s.store_div_scaled_value_offset_denominator(1502, s.ad_value(1477), 100.0, s.ad_value(1477), 100.0, 1.0);}
        s.b[1684] = (s.v[62] < 0.0);s.store_scalar(1684, if s.b[1684] { 1.0 } else { 0.0 });
        if ((s.b[1604] && s.b[1682]) && s.b[1684]) {s.store_div_from_scalar_sub_from_scalar_ad(1503, 1.0, 1.0, A::mul(s.ad_value(62), s.ad_value(1502)));}
        if ((s.b[1604] && s.b[1682]) && (!s.b[1684])) {s.store_offset_mul(1503, 62, 1502, 1.0);}
        if (s.b[1604] && s.b[1682]) {s.store_sub_ad(1504, A::div_scaled_product_by_product(s.ad_value(1474), s.ad_value(1473), 1.0, s.ad_value(1471), s.ad_value(1472), 1.0), A::div_scaled_inputs2(A::div(s.ad_value(1467), s.ad_value(1471)), 1.0, A::div(s.ad_value(1468), s.ad_value(1472)), 1.0, s.ad_value(1462), 1.0));s.store_div_scaled_product_offset_denominator_indices(1505, 1504, 1462, 1.0, 1504, 1.0, 1.0);s.store_sub(2, 1498, 1505);s.store_div_scaled_add_product_indices(1506, 1462, 1.0, 1498, 1495, 1.0, 2, 1.0);s.store_scaled_add_mixed_ia(1506, 1506, A::sqrt_square_offset(s.ad_value(1506), 1e-6), 0.5);s.store_scaled_mul_ad(1507, A::div(s.ad_value(1420), s.ad_value(1492)), A::add(s.ad_value(1501), s.ad_value(1503)), 0.5);s.store_sub_from_scalar_div_indices(1508, 1.0, 1462, 1505);s.store_offset(1509, 1495, 1.0);s.store_mul_sub_mixed_iai(1510, 1506, A::offset(A::mul(A::sub_scaled_inputs(s.ad_value(1505), 2.0, s.ad_value(1462), 1.0), s.ad_value(1499)), (-2.0)), 1495);}
        s.b[1685] = (s.v[1507] > 1e-14);s.store_scalar(1685, if s.b[1685] { 1.0 } else { 0.0 });
        if ((s.b[1604] && s.b[1682]) && s.b[1685]) {s.store_div_from_scalar_square_ad(1511, 2.0, s.ad_value(1507));s.store_mul(1512, 1511, 1508);s.store_add(1513, 1511, 1510);s.store_mul(1514, 1511, 1509);s.store_sqrt_offset_ad(1515, A::add(A::square(s.ad_value(1512)), A::mul3_scaled_output(s.ad_value(1511), s.ad_value(1511), s.ad_value(1511), 0.148148148148)), 1e-20);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_91(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1604] && s.b[1682]) && s.b[1685]) {s.store_sqrt_offset_ad(1516, A::add(A::square(s.ad_value(1514)), A::mul3_scaled_output(s.ad_value(1513), s.ad_value(1513), s.ad_value(1513), 0.148148148148)), 1e-20);s.store_sub_ad(1517, A::exp_scaled_input(A::ln_scaled_input(A::add(s.ad_value(1515), s.ad_value(1512)), 0.5), 0.3333333333333), A::exp_scaled_input(A::ln_scaled_input(A::sub(s.ad_value(1515), s.ad_value(1512)), 0.5), 0.3333333333333));s.store_sub_ad(1518, A::exp_scaled_input(A::ln_scaled_input(A::add(s.ad_value(1516), s.ad_value(1514)), 0.5), 0.3333333333333), A::exp_scaled_input(A::ln_scaled_input(A::sub(s.ad_value(1516), s.ad_value(1514)), 0.5), 0.3333333333333));}
        if ((s.b[1604] && s.b[1682]) && (!s.b[1685])) {s.copy_ad(1517, 1508);s.copy_ad(1518, 1509);}
        if (s.b[1604] && s.b[1682]) {s.store_square(4, 2);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1519, 1517, (0.94 * 0.5), 1518, (0.94 * 0.5), A::add_scaled_inputs(A::square(A::sub(s.ad_value(1517), s.ad_value(1518))), 1.0, s.ad_value(4), 10.0), (0.94 * 0.5));s.store_add_scaled_product_indices(1520, 1462, 1.0, 1505, 1519, 1.0);s.store_mul_sub_rhs(1521, 1498, 1519, 1495);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1522, 1520, 0.5, 1521, 0.5, A::add_scaled_inputs(A::square(A::sub(s.ad_value(1520), s.ad_value(1521))), 1.0, s.ad_value(4), 36.0), 0.5);}
        if (s.b[1604] && (!s.b[1682])) {s.copy_ad(1505, 1498);s.store_scaled_offset(1519, 1495, 1.0, 0.94);s.store_add_scaled_product_mixed_iia(1522, 1462, 0.5, 1498, A::sub_scaled_inputs(s.ad_value(1519), 1.0, s.ad_value(1495), 0.5), 1.0);}
        s.b[1686] = ((s.v[1522] - 0.5) < 80.0);s.store_scalar(1686, if s.b[1686] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1686]) {s.store_ln_one_plus_exp_ad(2, A::offset(s.ad_value(1522), (-0.5)));}
        if (s.b[1604] && (!s.b[1686])) {s.store_offset(2, 1522, (-0.5));}
        if s.b[1604] {s.store_offset(3, 2, 0.5);s.store_add_mixed_ia(4, 1519, A::ln(A::div(s.ad_value(1462), s.ad_value(3))));}
        s.b[1687] = ((s.v[4] - 6.0) < 80.0);s.store_scalar(1687, if s.b[1687] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1687]) {s.store_ln_one_plus_exp_ad(2, A::offset(s.ad_value(4), (-6.0)));}
        if (s.b[1604] && (!s.b[1687])) {s.store_offset(2, 4, (-6.0));}
        if s.b[1604] {s.store_offset(4, 2, 6.0);}
        s.b[1688] = ((s.v[221] - s.v[4]) < 80.0);s.store_scalar(1688, if s.b[1688] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1688]) {s.store_ln_one_plus_exp_ad(2, A::sub(s.ad_value(221), s.ad_value(4)));}
        if (s.b[1604] && (!s.b[1688])) {s.store_sub(2, 221, 4);}
        if s.b[1604] {s.store_sub(1523, 221, 2);s.store_div(2, 335, 1523);s.store_square(3, 2);s.store_square(4, 3);s.store_square(5, 4);s.store_exp_scaled_input_ad(0, A::ln(A::offset(A::mul(s.ad_value(1421), s.ad_value(4)), 1.0)), 2.666666666667);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_92(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1604] {s.store_mul_mixed_ia(1524, 335, A::exp_scaled_input(A::ln(A::add(s.ad_value(0), A::square(s.ad_value(5)))), (-0.0625)));s.store_div_from_scalar_offset_input(1338, 1.0, 1456, 1.0);s.store_div_from_scalar_offset_input(1339, 1.0, 1457, 1.0);s.store_offset_add_ad(1341, A::ln(A::div_scaled_product(A::add_scaled_product(s.ad_value(1456), 1.0, s.ad_value(1457), s.ad_value(1339), 1.0), s.ad_value(1428), 1.0, s.ad_value(1429), 1.0)), s.ad_value(1524), 3.0);s.store_offset_add_ad(1342, A::ln(A::div_scaled_product(A::add_scaled_product(s.ad_value(1457), 1.0, s.ad_value(1456), s.ad_value(1338), 1.0), s.ad_value(1428), 1.0, s.ad_value(1429), 1.0)), s.ad_value(1524), 3.0);}
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
        if (s.b[1604] && s.b[1694]) {s.store_sqrt_abs_ad(1352, s.ad_value(1349));s.store_div_mixed_ia(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_93(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1604] && s.b[1694]) {s.store_mul_add_mixed_iia(1354, 1338, 1349, A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)));s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);s.store_mul_div_lhs(1359, 1350, 1349, 1339);s.store_div_mixed_ai(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);}
        s.b[1695] = (s.v[1349] > 0.005);s.store_scalar(1695, if s.b[1695] { 1.0 } else { 0.0 });
        if ((s.b[1604] && (!s.b[1694])) && s.b[1695]) {s.store_sqrt_abs_ad(1352, s.ad_value(1349));s.store_exp_neg_input(1355, 1352);s.store_div_scaled_product_offset_rhs_mixed_iia(1353, 1352, 1355, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1355)), 1.0);s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);s.store_mul_add_mixed_iia(1354, 1338, 1349, A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)));s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);s.store_mul_div_lhs(1359, 1350, 1349, 1339);s.store_div_mixed_ai(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);}
        if ((s.b[1604] && (!s.b[1694])) && (!s.b[1695])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1340, 1349, 1.0, 1349, 1.0, 1349, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(1353, 1349, 1340, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1338, 1349, 1.0, 1349, 1.0, 1349, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(1354, 1350, 1338);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1339, 1349, 1.0, 1349, 1.0, 1349, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(1356, 1351, 1338, 1.0, A::square(s.ad_value(1350)), 1339, (-1.0));s.store_scaled_mul(1359, 1350, 1340, (-0.5));s.store_add_scaled_product_mixed_aii(1360, A::mul3_scaled_output(s.ad_value(1350), s.ad_value(1350), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 2.0, A::scale(s.ad_value(1349), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1351, 1340, (-0.5));}
        s.b[1696] = (s.v[1349] > 0.005);s.store_scalar(1696, if s.b[1696] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_94(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1604] && s.b[1696]) {s.store_div_scaled_inputs_mixed_ia(1339, 1349, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1355), 2.0, s.ad_value(1355))), 1.0);s.store_mul(1357, 1339, 1355);s.store_sub_ln_lhs(1358, 1339, 1352);}
        s.b[1697] = (s.v[1349] < (-0.005));s.store_scalar(1697, if s.b[1697] { 1.0 } else { 0.0 });
        if ((s.b[1604] && (!s.b[1696])) && s.b[1697]) {s.store_sin_scaled_input(1339, 1352, 0.5);s.store_div_scaled_inputs_square_rhs(1357, 1349, -1.0, 1339, 1.0);s.store_ln(1358, 1357);}
        if ((s.b[1604] && (!s.b[1696])) && (!s.b[1697])) {s.store_sub_from_scalar_ad(1357, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(1358, 1357);}
        s.b[1698] = (((1.01 * s.v[1347]) + s.v[1353]) > 0.0);s.store_scalar(1698, if s.b[1698] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1698]) {s.store_add(1361, 1347, 1353);s.store_add(1362, 1456, 1354);s.copy_ad(1363, 1356);}
        if (s.b[1604] && (!s.b[1698])) {s.store_div_from_scalar_sub_ad(1339, 1.0, s.ad_value(1347), s.ad_value(1353));s.store_sub(1340, 1354, 1456);s.store_mul_sub_lhs(1361, 1348, 1357, 1339);s.store_mul_mixed_ai(1362, A::add_scaled_value_products(s.ad_value(1348), (-1.0), s.ad_value(1340), s.ad_value(1361), 1.0, s.ad_value(1359), s.ad_value(1357), (-1.0)), 1339);s.store_mul_mixed_ai(1363, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1356), s.ad_value(1361), 1.0, s.ad_value(1340), s.ad_value(1362), 2.0), 1.0, s.ad_value(1348), 1.0, A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357), (-1.0)), 1339);}
        s.b[1699] = (s.v[1361] > 0.0);s.store_scalar(1699, if s.b[1699] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1699]) {s.store_ln(1364, 1361);s.store_div_from_scalar(1338, 1.0, 1361);s.store_mul(1365, 1362, 1338);s.store_add_scaled_square_product_indices(1366, 1365, (-1.0), 1363, 1338, 1.0);}
        if (s.b[1604] && (!s.b[1699])) {s.store_add_offset_lhs_mixed_ia(1364, 1347, 0.6931471805599, A::ln_scaled_input(s.ad_value(1347), -1.0));s.store_div_from_scalar(1338, 1.0, 1525);s.store_add(1365, 1456, 1338);s.store_mul_scale_offset_indices(1366, 1338, 1338, -1.0, 0.0);}
        if s.b[1604] {s.store_sub_add_scaled_inputs4_lhs_indices(1367, 1455, 1.0, 1454, (-1.0), 1525, 1.0, 1364, 2.0, 1358);s.store_sub_mixed_ai(1368, A::scale_offset(s.ad_value(1365), 2.0, 1.0), 1359);s.store_sub_scaled_inputs(1369, 1366, 2.0, 1360, 1.0);s.store_add_scaled_product_indices(1370, 1347, 1.0, 1457, 1367, 1.0);s.store_add_scaled_product_indices(1371, 1456, 1.0, 1457, 1368, 1.0);s.store_mul(1372, 1457, 1369);s.store_add_scaled_product_indices(1373, 1348, (-1.0), 1370, 1361, 1.0);s.store_add_mixed_ai(1374, A::add_scaled_products(s.ad_value(1371), s.ad_value(1361), 1.0, s.ad_value(1370), s.ad_value(1362), 1.0), 1348);s.store_sub_mixed_ai(1375, A::add_scaled_products3(s.ad_value(1372), s.ad_value(1361), 1.0, s.ad_value(1371), s.ad_value(1362), 2.0, s.ad_value(1370), s.ad_value(1363), 1.0), 1348);s.store_add_scaled_square_product_indices(1384, 1374, 1.0, 1373, 1375, (-0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_95(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1604] {s.store_div_scaled_product3_mixed_iiia(1376, 1373, 1374, 1384, -1.0, A::offset(A::square(s.ad_value(1384)), 1e-200), 1.0);s.store_add(1525, 1525, 1376);s.store_mul(1347, 1456, 1525);s.store_mul(1377, 1457, 1526);s.store_add(1370, 1347, 1377);s.store_offset_scaled(1378, 1370, 0.065345483024, 1.0);s.store_add_scaled_product_mixed_aii(1379, A::scale_offset(s.ad_value(1370), 8.5797362674, 39.478417604), 1.0, 1347, 1377, 1.0);s.store_add_scaled_product_indices(1380, 1370, (2.0 * 39.478417604), 1347, 1377, 39.478417604);s.store_sqrt_add_scaled_square_product(1381, 1379, 1.0, 1378, 1380, (-4.0));s.store_div_scaled_inputs2_indices(1349, 1381, 1.0, 1379, (-1.0), 1378, 2.0);s.store_sub_square_lhs(1382, 1347, 1349);}
        s.b[1700] = (s.v[1382] > 0.0);s.store_scalar(1700, if s.b[1700] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1700]) {s.store_mul_add_scaled_inputs4_rhs_mixed_aiii(1373, 1382, A::ln(A::div(s.ad_value(1382), s.ad_value(1429))), 1.0, 1524, 1.0, 1454, -1.0, 1525, 1.0);s.store_add_scaled_product_indices(1374, 1382, 1.0, 1456, 1347, 2.0);s.store_add_scaled_inputs3_indices(1383, 1454, 1.0, 1525, (-1.0), 1341, -1.0);}
        s.b[1701] = ((((s.v[1373] < 0.0) && (s.v[1374] > 0.0)) && (((s.v[1383] + 2.3025850929941) + ((s.v[1456]) as f64).ln()) > 0.0)) || (s.v[1383] > 1.0));s.store_scalar(1701, if s.b[1701] { 1.0 } else { 0.0 });
        if ((s.b[1604] && s.b[1700]) && s.b[1701]) {s.store_sub_div_rhs_indices(1525, 1525, 1373, 1374);}
        if s.b[1604] {s.store_mul(1347, 1456, 1525);s.store_mul(1377, 1457, 1526);s.store_add(1370, 1347, 1377);s.store_offset_scaled(1378, 1370, 0.065345483024, 1.0);s.store_add_scaled_product_mixed_aii(1379, A::scale_offset(s.ad_value(1370), 8.5797362674, 39.478417604), 1.0, 1347, 1377, 1.0);s.store_add_scaled_product_indices(1380, 1370, (2.0 * 39.478417604), 1347, 1377, 39.478417604);s.store_sqrt_add_scaled_square_product(1381, 1379, 1.0, 1378, 1380, (-4.0));s.store_div_scaled_inputs2_indices(1349, 1381, 1.0, 1379, (-1.0), 1378, 2.0);}
        s.b[1702] = (s.v[1349] < (-0.005));s.store_scalar(1702, if s.b[1702] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1702]) {s.store_sqrt_abs_ad(1352, s.ad_value(1349));s.store_div_mixed_ia(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));s.store_div_scaled_inputs2_mixed_iai(1354, 1349, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)), 0.25, 1349, 1.0);}
        s.b[1703] = (s.v[1349] > 0.005);s.store_scalar(1703, if s.b[1703] { 1.0 } else { 0.0 });
        if ((s.b[1604] && (!s.b[1702])) && s.b[1703]) {s.store_sqrt_abs_ad(1352, s.ad_value(1349));s.store_exp_neg_input(1355, 1352);s.store_div_scaled_product_offset_rhs_mixed_iia(1353, 1352, 1355, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1355)), 1.0);s.store_div_scaled_inputs2_mixed_iai(1354, 1349, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)), 0.25, 1349, 1.0);}
        if ((s.b[1604] && (!s.b[1702])) && (!s.b[1703])) {s.store_offset_ad(1353, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1354, 1349, 1.0, 1349, 1.0, 1349, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);}
        if s.b[1604] {s.store_sub_mixed_ia(1349, 1349, A::div_scaled_inputs2(A::add_scaled_products(s.ad_value(1370), s.ad_value(1353), 1.0, s.ad_value(1347), s.ad_value(1377), 1.0), 1.0, s.ad_value(1349), 1.0, A::offset(A::mul(s.ad_value(1370), s.ad_value(1354)), 1.0), 1.0));s.store_sub_square_lhs(1382, 1347, 1349);}
        s.b[1704] = (s.v[1382] > 0.0);s.store_scalar(1704, if s.b[1704] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_96(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1604] && s.b[1704]) {s.store_mul_add_scaled_inputs4_rhs_mixed_aiii(1373, 1382, A::ln(A::div(s.ad_value(1382), s.ad_value(1429))), 1.0, 1524, 1.0, 1454, -1.0, 1525, 1.0);s.store_add_scaled_product_indices(1374, 1382, 1.0, 1456, 1347, 2.0);s.store_add_scaled_inputs3_indices(1383, 1454, 1.0, 1525, (-1.0), 1341, -1.0);}
        s.b[1705] = ((((s.v[1373] < 0.0) && (s.v[1374] > 0.0)) && (((s.v[1383] + 2.3025850929941) + ((s.v[1456]) as f64).ln()) > 0.0)) || (s.v[1383] > 1.0));s.store_scalar(1705, if s.b[1705] { 1.0 } else { 0.0 });
        if ((s.b[1604] && s.b[1704]) && s.b[1705]) {s.store_sub_div_rhs_indices(1525, 1525, 1373, 1374);}
        if s.b[1604] {s.store_mul(1347, 1456, 1525);}
        s.b[1706] = (((s.v[1454] - s.v[1525]) - s.v[1524]) < 80.0);s.store_scalar(1706, if s.b[1706] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1706]) {s.store_exp_ad(1338, A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0));}
        if (s.b[1604] && (!s.b[1706])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1338, A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1604] {s.store_mul(1348, 1429, 1338);s.store_sub_square_lhs(1349, 1347, 1348);s.store_add_scaled_product_indices(1350, 1348, 1.0, 1456, 1347, 2.0);s.store_add_scaled_product_indices(1351, 1348, (-1.0), 1456, 1456, 2.0);}
        s.b[1707] = (s.v[1349] < (-0.005));s.store_scalar(1707, if s.b[1707] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1707]) {s.store_sqrt_abs_ad(1352, s.ad_value(1349));s.store_div_mixed_ia(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);s.store_mul_add_mixed_iia(1354, 1338, 1349, A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)));s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);s.store_mul_div_lhs(1359, 1350, 1349, 1339);s.store_div_mixed_ai(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);}
        s.b[1708] = (s.v[1349] > 0.005);s.store_scalar(1708, if s.b[1708] { 1.0 } else { 0.0 });
        if ((s.b[1604] && (!s.b[1707])) && s.b[1708]) {s.store_sqrt_abs_ad(1352, s.ad_value(1349));s.store_exp_neg_input(1355, 1352);s.store_div_scaled_product_offset_rhs_mixed_iia(1353, 1352, 1355, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1355)), 1.0);s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);s.store_mul_add_mixed_iia(1354, 1338, 1349, A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)));s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);s.store_mul_div_lhs(1359, 1350, 1349, 1339);s.store_div_mixed_ai(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_97(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1604] && (!s.b[1707])) && (!s.b[1708])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1340, 1349, 1.0, 1349, 1.0, 1349, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(1353, 1349, 1340, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1338, 1349, 1.0, 1349, 1.0, 1349, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(1354, 1350, 1338);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1339, 1349, 1.0, 1349, 1.0, 1349, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(1356, 1351, 1338, 1.0, A::square(s.ad_value(1350)), 1339, (-1.0));s.store_scaled_mul(1359, 1350, 1340, (-0.5));s.store_add_scaled_product_mixed_aii(1360, A::mul3_scaled_output(s.ad_value(1350), s.ad_value(1350), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 2.0, A::scale(s.ad_value(1349), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1351, 1340, (-0.5));}
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
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_98(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1604] && (!s.b[1712])) {s.store_add_offset_lhs_mixed_ia(1364, 1347, 0.6931471805599, A::ln_scaled_input(s.ad_value(1347), -1.0));s.store_div_from_scalar(1338, 1.0, 1525);s.store_add(1365, 1456, 1338);s.store_mul_scale_offset_indices(1366, 1338, 1338, -1.0, 0.0);}
        if s.b[1604] {s.store_sub_add_scaled_inputs4_lhs_indices(1367, 1455, 1.0, 1454, (-1.0), 1525, 1.0, 1364, 2.0, 1358);s.store_sub_mixed_ai(1368, A::scale_offset(s.ad_value(1365), 2.0, 1.0), 1359);s.store_sub_scaled_inputs(1369, 1366, 2.0, 1360, 1.0);s.store_add_scaled_product_indices(1370, 1347, 1.0, 1457, 1367, 1.0);s.store_add_scaled_product_indices(1371, 1456, 1.0, 1457, 1368, 1.0);s.store_mul(1372, 1457, 1369);s.store_add_scaled_product_indices(1373, 1348, (-1.0), 1370, 1361, 1.0);s.store_add_mixed_ai(1374, A::add_scaled_products(s.ad_value(1371), s.ad_value(1361), 1.0, s.ad_value(1370), s.ad_value(1362), 1.0), 1348);s.store_sub_mixed_ai(1375, A::add_scaled_products3(s.ad_value(1372), s.ad_value(1361), 1.0, s.ad_value(1371), s.ad_value(1362), 2.0, s.ad_value(1370), s.ad_value(1363), 1.0), 1348);s.store_add_scaled_square_product_indices(1384, 1374, 1.0, 1373, 1375, (-0.5));s.store_div_scaled_product3_mixed_iiia(1376, 1373, 1374, 1384, -1.0, A::offset(A::square(s.ad_value(1384)), 1e-200), 1.0);s.store_add(1525, 1525, 1376);s.store_mul(1347, 1456, 1525);}
        s.b[1713] = (((s.v[1454] - s.v[1525]) - s.v[1524]) < 80.0);s.store_scalar(1713, if s.b[1713] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1713]) {s.store_exp_ad(1338, A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0));}
        if (s.b[1604] && (!s.b[1713])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(1338, A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if s.b[1604] {s.store_mul(1348, 1429, 1338);s.store_sub_square_lhs(1349, 1347, 1348);s.store_add_scaled_product_indices(1350, 1348, 1.0, 1456, 1347, 2.0);s.store_add_scaled_product_indices(1351, 1348, (-1.0), 1456, 1456, 2.0);}
        s.b[1714] = (s.v[1349] < (-0.005));s.store_scalar(1714, if s.b[1714] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1714]) {s.store_sqrt_abs_ad(1352, s.ad_value(1349));s.store_div_mixed_ia(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);s.store_mul_add_mixed_iia(1354, 1338, 1349, A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)));s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);s.store_mul_div_lhs(1359, 1350, 1349, 1339);s.store_div_mixed_ai(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);}
        s.b[1715] = (s.v[1349] > 0.005);s.store_scalar(1715, if s.b[1715] { 1.0 } else { 0.0 });
        if ((s.b[1604] && (!s.b[1714])) && s.b[1715]) {s.store_sqrt_abs_ad(1352, s.ad_value(1349));s.store_exp_neg_input(1355, 1352);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_99(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1604] && (!s.b[1714])) && s.b[1715]) {s.store_div_scaled_product_offset_rhs_mixed_iia(1353, 1352, 1355, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1355)), 1.0);s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);s.store_mul_add_mixed_iia(1354, 1338, 1349, A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)));s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);s.store_mul_div_lhs(1359, 1350, 1349, 1339);s.store_div_mixed_ai(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);}
        if ((s.b[1604] && (!s.b[1714])) && (!s.b[1715])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1340, 1349, 1.0, 1349, 1.0, 1349, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(1353, 1349, 1340, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1338, 1349, 1.0, 1349, 1.0, 1349, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(1354, 1350, 1338);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1339, 1349, 1.0, 1349, 1.0, 1349, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(1356, 1351, 1338, 1.0, A::square(s.ad_value(1350)), 1339, (-1.0));s.store_scaled_mul(1359, 1350, 1340, (-0.5));s.store_add_scaled_product_mixed_aii(1360, A::mul3_scaled_output(s.ad_value(1350), s.ad_value(1350), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 2.0, A::scale(s.ad_value(1349), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1351, 1340, (-0.5));}
        s.b[1716] = (s.v[1349] > 0.005);s.store_scalar(1716, if s.b[1716] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1716]) {s.store_div_scaled_inputs_mixed_ia(1339, 1349, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1355), 2.0, s.ad_value(1355))), 1.0);s.store_mul(1357, 1339, 1355);s.store_sub_ln_lhs(1358, 1339, 1352);}
        s.b[1717] = (s.v[1349] < (-0.005));s.store_scalar(1717, if s.b[1717] { 1.0 } else { 0.0 });
        if ((s.b[1604] && (!s.b[1716])) && s.b[1717]) {s.store_sin_scaled_input(1339, 1352, 0.5);s.store_div_scaled_inputs_square_rhs(1357, 1349, -1.0, 1339, 1.0);s.store_ln(1358, 1357);}
        if ((s.b[1604] && (!s.b[1716])) && (!s.b[1717])) {s.store_sub_from_scalar_ad(1357, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0396825396825397), 0.05), 0.3333333333333));s.store_ln(1358, 1357);}
        s.b[1718] = (((1.01 * s.v[1347]) + s.v[1353]) > 0.0);s.store_scalar(1718, if s.b[1718] { 1.0 } else { 0.0 });
        if (s.b[1604] && s.b[1718]) {s.store_add(1361, 1347, 1353);s.store_add(1362, 1456, 1354);s.copy_ad(1363, 1356);}
        if (s.b[1604] && (!s.b[1718])) {s.store_div_from_scalar_sub_ad(1339, 1.0, s.ad_value(1347), s.ad_value(1353));s.store_sub(1340, 1354, 1456);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_100(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1604] && (!s.b[1718])) {s.store_mul_sub_lhs(1361, 1348, 1357, 1339);s.store_mul_mixed_ai(1362, A::add_scaled_value_products(s.ad_value(1348), (-1.0), s.ad_value(1340), s.ad_value(1361), 1.0, s.ad_value(1359), s.ad_value(1357), (-1.0)), 1339);s.store_mul_mixed_ai(1363, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1356), s.ad_value(1361), 1.0, s.ad_value(1340), s.ad_value(1362), 2.0), 1.0, s.ad_value(1348), 1.0, A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357), (-1.0)), 1339);}
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
    pub(super) fn stamp_reactive_block_101(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[1604] && s.b[1720]) && s.b[1721]) && s.b[1723]) {s.store_div_mixed_ia(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);s.store_mul_add_mixed_iia(1354, 1338, 1349, A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)));s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);s.store_mul_div_lhs(1359, 1350, 1349, 1339);s.store_div_mixed_ai(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);}
        s.b[1724] = (s.v[1349] > 0.005);s.store_scalar(1724, if s.b[1724] { 1.0 } else { 0.0 });
        if ((((s.b[1604] && s.b[1720]) && s.b[1721]) && (!s.b[1723])) && s.b[1724]) {s.store_sqrt_abs_ad(1352, s.ad_value(1349));s.store_exp_neg_input(1355, 1352);s.store_div_scaled_product_offset_rhs_mixed_iia(1353, 1352, 1355, 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1355)), 1.0);s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);s.store_mul_add_mixed_iia(1354, 1338, 1349, A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)));s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);s.store_mul_div_lhs(1359, 1350, 1349, 1339);s.store_div_mixed_ai(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);}
        if ((((s.b[1604] && s.b[1720]) && s.b[1721]) && (!s.b[1723])) && (!s.b[1724])) {s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1340, 1349, 1.0, 1349, 1.0, 1349, 0.025, 0.0238095238095, ((0.0166666666667) * ((-0.1666666666667))), 0.1666666666667);s.store_offset_mul(1353, 1349, 1340, 2.0);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1338, 1349, 1.0, 1349, 1.0, 1349, 0.0333333333333, 0.0357142857143, ((0.0333333333333) * ((-0.1666666666667))), 0.1666666666667);s.store_mul(1354, 1350, 1338);s.store_offset_scaled_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(1339, 1349, 1.0, 1349, 1.0, 1349, 0.0420875420875421, 0.05, ((0.0714285714286) * ((-0.0055555555556))), 0.0055555555556);s.store_add_scaled_products_mixed_iiai(1356, 1351, 1338, 1.0, A::square(s.ad_value(1350)), 1339, (-1.0));s.store_scaled_mul(1359, 1350, 1340, (-0.5));s.store_add_scaled_product_mixed_aii(1360, A::mul3_scaled_output(s.ad_value(1350), s.ad_value(1350), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 2.0, A::scale(s.ad_value(1349), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1351, 1340, (-0.5));}
    }
}
