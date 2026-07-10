#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_61(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t3: usize = 0;
        while {
            let t0: f64 = (2.0 * 20.0);let t1: f64 = (t0 + 1.0);let t2: f64 = if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (s.v[167] <= t1)) { 1.0 } else { 0.0 };
            t2 != 0.0
        } {
            t3 += 1;assert!(t3 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {s.store_scalar(1464, 0.0);s.store_mul_add_rhs(1440, 225, 1442, 1417);}
            s.b[1504] = (s.v[1440] < 5.0);s.store_scalar(1504, if s.b[1504] { 1.0 } else { 0.0 });
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && s.b[1504]) {s.store_mul3_ad_middle(1460, A::square(s.ad_value(1440)), 1440, A::offset(A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(1461, A::square(s.ad_value(1440)), A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul3_lhs(1462, 1468, 1460, 1460);s.store_mul_product3_indices(1463, 1461, 1468, 225, 1460, 2.0);s.store_mul_scale_offset_mixed_ia(1464, 1440, A::mul_offset_rhs(s.ad_value(1440), A::mul_offset_rhs(s.ad_value(1440), A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(1465, 1440, A::mul_offset_rhs(s.ad_value(1440), A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_offset_ad(1466, A::add(A::square(s.ad_value(1464)), s.ad_value(1462)), 1e-50);s.store_div_scaled_inputs2_mixed_aii(1467, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1465), s.ad_value(1464), 2.0), 1.0, 1463, 1.0, 1466, 2.0);}
            s.b[1505] = (s.v[1440] < 80.0);s.store_scalar(1505, if s.b[1505] { 1.0 } else { 0.0 });
            if ((((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1504])) && s.b[1505]) {s.store_exp(243, 1440);s.store_mul_scale_offset_indices(1462, 1468, 243, 1.0, (-1.0));s.store_mul3_lhs(1463, 1468, 225, 243);}
            if ((((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1504])) && (!s.b[1505])) {s.store_exp_mul(1469, 225, 1442);s.store_mul_sub_rhs(1462, 1453, 1469, 1459);s.store_mul3_lhs(1463, 1453, 225, 1469);}
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1504])) {s.store_sqrt_add_ad(1466, A::offset(s.ad_value(1440), (-1.0)), s.ad_value(1462));s.store_scale_ad(1467, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1463), 1.0, s.ad_value(1466), 1.0), 0.5);}
            if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {s.store_add_scaled_inputs_product_indices(1470, 1428, 1.0, 1442, (-1.0), 1426, 1466, (-1.0));s.store_sub_from_scalar_scaled_mul(1471, (-1.0), 1426, 1467, 1.0);}
            s.b[1506] = (s.v[1413] == 1.0);s.store_scalar(1506, if s.b[1506] { 1.0 } else { 0.0 });
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && s.b[1506]) {s.store_scalar(167, ((2.0 * 20.0) + 1.0));}
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1506])) {s.store_div_scaled_inputs_indices(494, 1470, -1.0, 1471, 1.0);}
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1506])) {
                s.store_scaled_offset_ad(1472, {
                    if (1.0 >= ((s.v[1442]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1442))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1507] = (((s.v[494]) as f64).abs() > s.v[1472]);s.store_scalar(1507, if s.b[1507] { 1.0 } else { 0.0 });
            if ((((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1506])) && s.b[1507]) {s.store_scale(494, 1472, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1506])) {s.store_add(1442, 1442, 494);}
            s.b[1508] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1470]) as f64).abs() <= 1e-8));s.store_scalar(1508, if s.b[1508] { 1.0 } else { 0.0 });
            if ((((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1506])) && s.b[1508]) {s.store_scalar(1413, 1.0);}
            if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {s.store_primal_offset(167, 167, 1.0);}
        }
        s.b[1510] = (s.v[1440] < 5.0);s.store_scalar(1510, if s.b[1510] { 1.0 } else { 0.0 });
        if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && s.b[1510]) {s.store_offset_square(1473, 1464, (10.0 * 2.220446049250313e-16));s.store_offset(1474, 1464, (10.0 * 2.220446049250313e-16));}
        if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1510])) {s.store_offset(1473, 1440, (-1.0));s.store_sqrt(1474, 1473);}
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {s.store_mul(458, 1425, 1474);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_62(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {s.store_div_from_scalar_add_ad(1397, 1.0, s.ad_value(1466), s.ad_value(1474));s.store_mul3_lhs(460, 1425, 1462, 1397);s.store_add(459, 458, 460);}
        if ((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) {s.store_sub(460, 459, 458);}
        s.b[1512] = (1.0 == 1.0);s.store_scalar(1512, if s.b[1512] { 1.0 } else { 0.0 });s.b[1513] = (1.0 == 2.0);s.store_scalar(1513, if s.b[1513] { 1.0 } else { 0.0 });
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1512]) && (s.v[1407] != 0.0)) {s.store_mul_scale_offset_indices(463, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(465, 460, 522, -1.0, 0.0);}
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1512]) && (s.v[1408] != 0.0)) {s.store_mul_scale_offset_indices(464, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(466, 460, 522, -1.0, 0.0);}
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (s.b[1513] && (!s.b[1512]))) && (s.v[1407] != 0.0)) {s.store_mul_scale_offset_indices(467, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(469, 460, 522, -1.0, 0.0);}
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (s.b[1513] && (!s.b[1512]))) && (s.v[1408] != 0.0)) {s.store_mul_scale_offset_indices(468, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(470, 460, 522, -1.0, 0.0);}
        s.store_scalar(317, p.p189);s.b[1516] = (s.v[145] != 0.0);s.store_scalar(1516, if s.b[1516] { 1.0 } else { 0.0 });
        if s.b[1516] {s.store_add(1515, 157, 161);s.store_add_scaled_inputs(314, 1515, s.v[317], 162, (1.0 - s.v[317]));}
        s.b[1517] = (p.p64 != 0.0);s.store_scalar(1517, if s.b[1517] { 1.0 } else { 0.0 });
        if (s.b[1516] && s.b[1517]) {s.store_scalar(315, 0.0);}
        s.b[1518] = (s.v[314] > ((s.v[161] + s.v[157]) - (10.0 * 2.220446049250313e-16)));s.store_scalar(1518, if s.b[1518] { 1.0 } else { 0.0 });
        if (s.b[1516] && s.b[1518]) {s.store_offset_add(314, 161, 157, (-(10.0 * 2.220446049250313e-16)));}
        s.b[1519] = (p.p64 != 0.0);s.store_scalar(1519, if s.b[1519] { 1.0 } else { 0.0 });s.b[1520] = (s.v[246] < 1e-15);s.store_scalar(1520, if s.b[1520] { 1.0 } else { 0.0 });
        if (((!s.b[1516]) && s.b[1519]) && s.b[1520]) {s.store_scalar(315, 0.0);}
        if (((!s.b[1516]) && s.b[1519]) && (!s.b[1520])) {s.store_scale(1514, 227, 1.0 / (s.v[97]));s.store_div_from_scalar(1515, 1.0, 244);s.store_mul3_lhs(315, 246, 1514, 1515);}
        s.store_scalar(1532, s.v[91]);s.store_scalar(1533, (1.0 / s.v[1532]));s.store_scalar(1553, 0.0);s.store_scalar(1593, 0.0);s.store_scalar(1591, 0.0);s.store_scalar(1595, 0.0);s.b[1604] = ((p.p29 >= 1.0) && (p.p188 > 0.0));s.store_scalar(1604, if s.b[1604] { 1.0 } else { 0.0 });
        if ((p.p24 != 0.0) && s.b[1604]) {s.store_scalar(1535, p.p171);s.store_scalar(1536, p.p172);s.copy_ad(1537, 158);s.store_scalar(1534, p.p188);}
        s.b[1605] = ((s.v[69] == 0.0) && (p.p188 > 0.0));s.store_scalar(1605, if s.b[1605] { 1.0 } else { 0.0 });
        if (((p.p24 != 0.0) && s.b[1604]) && s.b[1605]) {
            if (p.p43 == 1.0) {
                s.store_scale(1522, 287, s.v[1532]);
            } else {
                s.store_scale(1522, 108, s.v[1532]);
            }
        }
        if (((p.p24 != 0.0) && s.b[1604]) && s.b[1605]) {s.store_mul_ad_product_rhs_mixed_ia(1525, 1535, 1522, A::add(s.ad_value(1536), s.ad_value(1537)));s.store_mul(1526, 1534, 1522);s.copy_ad(1530, 161);s.store_sub_from_scalar(1527, 1.2, 1530);s.store_add_scaled_products_indices(267, 158, 1526, 1.0, 1527, 1525, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(1525, 1535, 1522, A::add_scaled_inputs3(s.ad_value(1536), 1.0, s.ad_value(1537), 1.0, s.ad_value(157), -1.0));s.store_sub(1530, 162, 157);s.store_sub_from_scalar(1527, 1.2, 1530);s.store_add_scaled_products_mixed_aiii(268, A::sub(s.ad_value(158), s.ad_value(157)), 1526, 1.0, 1525, 1527, (-1.0));}
        if (((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) {s.store_mul_sqrt_mixed_ia(1554, 238, A::div_from_scalar(s.v[69], s.ad_value(536)));s.store_scalar(1538, ((1.0 - -1.0) / 2.0));s.store_scalar(1539, ((1.0 + -1.0) / 2.0));}
        s.b[1606] = (p.p43 == 1.0);s.store_scalar(1606, if s.b[1606] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1606]) {s.store_add_scaled_products_mixed_iiia(1548, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);s.store_add_scaled_products_indices(1549, 461, 157, 1.0, 462, 157, -1.0);s.store_add_scaled_products_mixed_iiia(1550, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_sub(1551, 1549, 1548);s.store_sub(1553, 1550, 1548);s.store_neg(1552, 1548);s.store_primal_add_scaled_products_indices(1540, 1538, 461, 1.0, 1539, 462, 1.0);s.store_primal_add_scaled_products_indices(1541, 1538, 462, 1.0, 1539, 461, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_63(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1606]) {s.store_offset_ad(1546, A::add_scaled_products(s.ad_value(1540), s.ad_value(1552), 1.0, s.ad_value(1541), s.ad_value(1551), 1.0), (10.0 * 2.220446049250313e-16));}
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1606])) {s.store_primal_add_scaled_products_indices(1540, 1538, 461, 1.0, 1539, 462, 1.0);s.store_primal_add_scaled_products_indices(1541, 1538, 462, 1.0, 1539, 461, 1.0);}
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1606])) && (s.v[1538] != 0.0)) {s.store_add_scaled_products_mixed_iiia(1553, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);}
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1606])) && (s.v[1539] != 0.0)) {s.store_add_scaled_products_mixed_iiia(1553, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);}
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1606])) {s.store_scalar(1546, 0.0);}
        if (((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) {s.store_neg(1521, 1546);}
        s.b[1607] = (s.v[1521] > s.v[141]);s.store_scalar(1607, if s.b[1607] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1607]) {s.store_sub(1522, 1521, 141);s.store_sub(1523, 140, 141);s.store_div(44, 1522, 1523);s.store_square(45, 44);s.store_mul(46, 45, 44);s.store_square(47, 45);s.store_div_from_scalar_ad(1531, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));s.store_mul_scale_offset_indices(1531, 1523, 1531, -1.0, 1.0);s.store_add(1528, 141, 1531);}
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1607])) {s.copy_ad(1528, 1521);}
        if (((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) {s.store_offset_scaled(1547, 1528, -1.0, (-1e-12));s.store_scale(1555, 1554, s.v[1533]);s.store_square(1556, 1555);s.store_sub_from_scalar(1557, s.v[82], 1553);s.store_div_from_scalar(1521, s.v[69], 230);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1558, 2.0, 225, A::ln(s.ad_value(1521)));s.store_neg(1559, 1547);}
        s.b[1608] = (s.v[1557] < s.v[1559]);s.store_scalar(1608, if s.b[1608] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1608]) {s.store_div_scalar_by_product_indices(1522, 1.0, 225, 1554, 1.0);s.store_scale(1531, 1522, s.v[1532]);s.store_offset_scaled(1560, 1531, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(1561, 1560, 1560, 8.0, 0.0, 1560);s.store_sub(1562, 237, 1558);s.store_mul_add_rhs(1530, 225, 1557, 1547);s.store_sub_from_scalar_scaled_mul_mixed_ia(1563, (7.0 * 1.414213562373095), 1531, A::offset(s.ad_value(1530), (-2.0)), 9.0);s.store_square(1564, 1563);}
        s.b[1609] = (s.v[1561] < (s.v[1564] * 1e-8));s.store_scalar(1609, if s.b[1609] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1608]) && s.b[1609]) {s.store_add_scaled_inputs_product_mixed_aaia(1566, A::offset(s.ad_value(1563), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1561), 0.5, s.ad_value(1563), 1.0), 1.0, 1531, A::offset(s.ad_value(1530), (-2.0)), 9.0);}
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1608]) && (!s.b[1609])) {s.store_sqrt_add(1565, 1561, 1564);s.store_add_scaled_offset_product_rhs_mixed_aii(1566, A::offset(s.ad_value(1565), ((-7.0) * 1.414213562373095)), 1.0, 1531, 1530, (-2.0), 9.0);}
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1608]) {s.store_powf(1567, 1566, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(1568, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1531), 12.0)), 1.0, 1567, 2.0, 1567, 1567, 1.414213562373095);s.store_div(1569, 1568, 1567);s.store_add_scaled_product_indices(1570, 1547, (-1.0), 1569, 227, 1.0);s.store_add(1522, 1570, 1547);s.store_div(1523, 1522, 1562);s.store_sqrt_square_offset(1524, 1523, 1.0);s.store_sub_div_lhs_indices(1571, 1522, 1524, 1547);s.store_sub(1523, 1557, 1571);s.store_scale(459, 1523, s.v[1532]);s.copy_ad(458, 459);}
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) {s.store_scalar(1569, 3.0);s.store_sub_div_lhs_indices(1572, 1569, 225, 1547);s.store_exp_neg_input(1531, 1569);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_64(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) {s.store_offset_div_scaled_inputs2_mixed_aia(1530, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1557), s.ad_value(1547))), (-1.0)), 4.0, 1531, 4.0, A::mul(s.ad_value(1556), s.ad_value(226)), 1.0, 1.0);}
        s.b[1610] = (s.v[1530] < (10.0 * 2.220446049250313e-16));s.store_scalar(1610, if s.b[1610] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1610]) {s.store_scalar(1530, (10.0 * 2.220446049250313e-16));}
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) {s.store_add_product3_rhs_mixed_iia(1572, 1557, 1556, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1530))), 1.0 / (2.0));s.store_mul_add_rhs(1569, 225, 1572, 1547);s.store_exp_neg_input(1531, 1569);s.store_offset_div_scaled_inputs2_mixed_aia(1530, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1557), s.ad_value(1547))), (-1.0)), 4.0, 1531, 4.0, A::mul(s.ad_value(1556), s.ad_value(226)), 1.0, 1.0);}
        s.b[1611] = (s.v[1530] < (10.0 * 2.220446049250313e-16));s.store_scalar(1611, if s.b[1611] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1611]) {s.store_scalar(1530, (10.0 * 2.220446049250313e-16));}
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) {s.store_add_product3_rhs_mixed_iia(1572, 1557, 1556, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1530))), 1.0 / (2.0));s.store_mul_add_rhs(1569, 225, 1572, 1547);}
        s.b[1612] = (s.v[1569] < 3.0);s.store_scalar(1612, if s.b[1612] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1612]) {s.store_scalar(1573, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));s.store_scalar(1574, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));s.store_offset_div_from_scalar_ad(1575, 1.0, A::mul(s.ad_value(225), s.ad_value(1555)), (1.0 / 1.414213562373095));s.store_div_scaled_inputs2_indices(1576, 1557, -1.0, 1547, -1.0, 1555, 1.0);s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(1577, A::div_scaled_product(A::square(s.ad_value(1574)), s.ad_value(1574), 1.0, A::mul3_scaled_output(s.ad_value(1573), s.ad_value(1573), s.ad_value(1573), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1574), s.ad_value(1575), 1.0, s.ad_value(1573), s.ad_value(1573), 6.0), (-1.0), 1576, 1.0, 1573, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(1578, A::add_scaled_square_product(s.ad_value(1574), (-1.0), s.ad_value(1573), s.ad_value(1575), 3.0), 1.0, 1573, 1573, 9.0);s.store_sqrt_add_scaled_square_cube_product(1526, 1577, 1.0, 1578, 1.0);s.store_powf_ad(1579, A::sub(s.ad_value(1526), s.ad_value(1577)), 0.3333333333333333);s.store_neg_powf_add_input(1580, 1577, 1526, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(1530, 1579, 1.0, 1580, 1.0, 1574, 1.0, 1573, 3.0, -1.0);s.store_add_scaled_product_indices(1572, 1547, (-1.0), 1530, 227, 1.0);s.store_mul_add_rhs(1569, 225, 1572, 1547);}
        s.b[1613] = (p.p41 > 0.0);s.store_scalar(1613, if s.b[1613] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1613]) {s.store_offset_add(1581, 1557, 1547, 0.1);s.store_offset_exp_ad(1588, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1547), -1.0), 1e-50);s.store_scale(1521, 230, 1.0 / (s.v[69]));s.store_square(1582, 1521);s.store_mul(1583, 1582, 1588);s.store_mul(1521, 226, 1556);s.store_mul(1584, 225, 1581);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_65(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1613]) {s.store_add_scaled_inputs_product_mixed_aaii(1585, A::ln(A::add_scaled_square_product(s.ad_value(1584), 1.0, s.ad_value(1583), s.ad_value(1521), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1582), s.ad_value(1521))), (-1.0), 225, 1547, 1.0);s.store_offset_sub(44, 1584, 1585, (-1.0));s.store_scale(45, 1584, 4.0);}
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1613]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1613]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1522, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1523, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1585, 1584, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub(1584, 1584, 1585);s.store_add_scaled_inputs(1584, 1584, 1.0, 225, 0.1);s.store_add_scaled_inputs_product_mixed_aaii(1586, A::ln(A::add_scaled_square_product(s.ad_value(1584), 1.0, s.ad_value(1583), s.ad_value(1521), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1582), s.ad_value(1521))), (-1.0), 225, 1547, 1.0);s.copy_ad(1587, 1569);s.store_offset_sub(44, 1586, 1587, (-(0.0008 * 75.0)));s.store_scale(45, 1586, (4.0 * (0.0008 * 75.0)));}
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1613]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1613]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1522, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1523, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1569, 1586, 1.0, 44, (-0.5), 45, (-0.5));}
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) {s.store_sub_div_lhs_indices(1571, 1569, 225, 1547);s.store_add_offset_lhs_mixed_ia(1522, 1569, (-1.0), A::exp_scaled_input(s.ad_value(1569), -1.0));}
        s.b[1614] = (s.v[1522] < (10.0 * 2.220446049250313e-16));s.store_scalar(1614, if s.b[1614] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1614]) {s.store_scalar(1522, (10.0 * 2.220446049250313e-16));}
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) {s.store_sqrt(1523, 1522);s.store_mul(458, 1554, 1523);s.store_scaled_sub(459, 1557, 1571, s.v[1532]);}
        s.b[1615] = (p.p41 == 1.0);s.store_scalar(1615, if s.b[1615] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) {s.store_exp_ad(1588, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1547), -1.0));s.store_scale(1521, 230, 1.0 / (s.v[69]));s.store_square(1582, 1521);s.store_mul(1597, 1582, 1588);s.store_scalar(1544, 0.0);s.store_scalar(1591, 0.0);s.store_scalar(1595, 0.0);s.store_scalar(167, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_66(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t7: usize = 0;
        while {
            let t4: f64 = (2.0 * 20.0);let t5: f64 = (t4 + 1.0);let t6: f64 = if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (s.v[167] <= t5)) { 1.0 } else { 0.0 };
            t6 != 0.0
        } {
            t7 += 1;assert!(t7 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) {s.store_scalar(1593, 0.0);s.store_mul_add_rhs(1569, 225, 1571, 1547);}
            s.b[1616] = (s.v[1569] < 5.0);s.store_scalar(1616, if s.b[1616] { 1.0 } else { 0.0 });
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && s.b[1616]) {s.store_mul3_ad_middle(1589, A::square(s.ad_value(1569)), 1569, A::offset(A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(1590, A::square(s.ad_value(1569)), A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul3_lhs(1591, 1597, 1589, 1589);s.store_mul_product3_indices(1592, 1590, 1597, 225, 1589, 2.0);s.store_mul_scale_offset_mixed_ia(1593, 1569, A::mul_offset_rhs(s.ad_value(1569), A::mul_offset_rhs(s.ad_value(1569), A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(1594, 1569, A::mul_offset_rhs(s.ad_value(1569), A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_offset_ad(1595, A::add(A::square(s.ad_value(1593)), s.ad_value(1591)), 1e-50);s.store_div_scaled_inputs2_mixed_aii(1596, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1594), s.ad_value(1593), 2.0), 1.0, 1592, 1.0, 1595, 2.0);}
            s.b[1617] = (s.v[1569] < 80.0);s.store_scalar(1617, if s.b[1617] { 1.0 } else { 0.0 });
            if (((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1616])) && s.b[1617]) {s.store_exp(243, 1569);s.store_mul_scale_offset_indices(1591, 1597, 243, 1.0, (-1.0));s.store_mul3_lhs(1592, 1597, 225, 243);}
            if (((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1616])) && (!s.b[1617])) {s.store_exp_mul(1598, 225, 1571);s.store_mul_sub_rhs(1591, 1582, 1598, 1588);s.store_mul3_lhs(1592, 1582, 225, 1598);}
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1616])) {s.store_sqrt_add_ad(1595, A::offset(s.ad_value(1569), (-1.0)), s.ad_value(1591));s.store_scale_ad(1596, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1592), 1.0, s.ad_value(1595), 1.0), 0.5);}
            if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) {s.store_add_scaled_inputs_product_indices(1599, 1557, 1.0, 1571, (-1.0), 1555, 1595, (-1.0));s.store_sub_from_scalar_scaled_mul(1600, (-1.0), 1555, 1596, 1.0);}
            s.b[1618] = (s.v[1544] == 1.0);s.store_scalar(1618, if s.b[1618] { 1.0 } else { 0.0 });
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && s.b[1618]) {s.store_scalar(167, ((2.0 * 20.0) + 1.0));}
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1618])) {s.store_div_scaled_inputs_indices(494, 1599, -1.0, 1600, 1.0);}
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1618])) {
                s.store_scaled_offset_ad(1601, {
                    if (1.0 >= ((s.v[1571]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1571))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1619] = (((s.v[494]) as f64).abs() > s.v[1601]);s.store_scalar(1619, if s.b[1619] { 1.0 } else { 0.0 });
            if (((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1618])) && s.b[1619]) {s.store_scale(494, 1601, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1618])) {s.store_add(1571, 1571, 494);}
            s.b[1620] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1599]) as f64).abs() <= 1e-8));s.store_scalar(1620, if s.b[1620] { 1.0 } else { 0.0 });
            if (((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1618])) && s.b[1620]) {s.store_scalar(1544, 1.0);}
            if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) {s.store_primal_offset(167, 167, 1.0);}
        }
        s.b[1622] = (s.v[1569] < 5.0);s.store_scalar(1622, if s.b[1622] { 1.0 } else { 0.0 });
        if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && s.b[1622]) {s.store_offset_square(1602, 1593, (10.0 * 2.220446049250313e-16));s.store_offset(1603, 1593, (10.0 * 2.220446049250313e-16));}
        if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1622])) {s.store_offset(1602, 1569, (-1.0));s.store_sqrt(1603, 1602);}
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) {s.store_mul(458, 1554, 1603);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_67(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) {s.store_div_from_scalar_add_ad(1522, 1.0, s.ad_value(1595), s.ad_value(1603));s.store_mul3_lhs(460, 1554, 1591, 1522);s.store_add(459, 458, 460);}
        if (((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) {s.store_sub(460, 459, 458);}
        if (((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) {
            if (p.p43 == 1.0) {
                s.store_mul(1525, 287, 1534);
            } else {
                s.store_mul(1525, 108, 1534);
            }
        }
        s.b[1624] = (((s.v[1540] != 0.0) && (p.p43 == 0.0)) || ((s.v[1538] != 0.0) && (p.p43 == 1.0)));s.store_scalar(1624, if s.b[1624] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1624]) {s.store_mul(455, 1525, 459);s.store_mul(457, 1525, 458);}
        s.b[1625] = (((s.v[1541] != 0.0) && (p.p43 == 0.0)) || ((s.v[1539] != 0.0) && (p.p43 == 1.0)));s.store_scalar(1625, if s.b[1625] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1625]) {s.store_mul(454, 1525, 459);s.store_mul(456, 1525, 458);}
        if (((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) {s.store_scalar(1538, ((1.0 - 1.0) / 2.0));s.store_scalar(1539, ((1.0 + 1.0) / 2.0));}
        s.b[1626] = (p.p43 == 1.0);s.store_scalar(1626, if s.b[1626] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1626]) {s.store_add_scaled_products_mixed_iiia(1548, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);s.store_add_scaled_products_indices(1549, 461, 157, 1.0, 462, 157, -1.0);s.store_add_scaled_products_mixed_iiia(1550, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_sub(1551, 1549, 1548);s.store_sub(1553, 1550, 1548);s.store_neg(1552, 1548);s.store_primal_add_scaled_products_indices(1540, 1538, 461, 1.0, 1539, 462, 1.0);s.store_primal_add_scaled_products_indices(1541, 1538, 462, 1.0, 1539, 461, 1.0);s.store_offset_ad(1546, A::add_scaled_products(s.ad_value(1540), s.ad_value(1552), 1.0, s.ad_value(1541), s.ad_value(1551), 1.0), (10.0 * 2.220446049250313e-16));}
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1626])) {s.store_primal_add_scaled_products_indices(1540, 1538, 461, 1.0, 1539, 462, 1.0);s.store_primal_add_scaled_products_indices(1541, 1538, 462, 1.0, 1539, 461, 1.0);}
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1626])) && (s.v[1538] != 0.0)) {s.store_add_scaled_products_mixed_iiia(1553, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);}
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1626])) && (s.v[1539] != 0.0)) {s.store_add_scaled_products_mixed_iiia(1553, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);}
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1626])) {s.store_scalar(1546, 0.0);}
        if (((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) {s.store_neg(1521, 1546);}
        s.b[1627] = (s.v[1521] > s.v[141]);s.store_scalar(1627, if s.b[1627] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1627]) {s.store_sub(1522, 1521, 141);s.store_sub(1523, 140, 141);s.store_div(44, 1522, 1523);s.store_square(45, 44);s.store_mul(46, 45, 44);s.store_square(47, 45);s.store_div_from_scalar_ad(1531, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));s.store_mul_scale_offset_indices(1531, 1523, 1531, -1.0, 1.0);s.store_add(1528, 141, 1531);}
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1627])) {s.copy_ad(1528, 1521);}
        if (((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) {s.store_offset_scaled(1547, 1528, -1.0, (-1e-12));s.store_scale(1555, 1554, s.v[1533]);s.store_square(1556, 1555);s.store_sub_from_scalar(1557, s.v[82], 1553);s.store_div_from_scalar(1521, s.v[69], 230);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1558, 2.0, 225, A::ln(s.ad_value(1521)));s.store_neg(1559, 1547);}
        s.b[1628] = (s.v[1557] < s.v[1559]);s.store_scalar(1628, if s.b[1628] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1628]) {s.store_div_scalar_by_product_indices(1522, 1.0, 225, 1554, 1.0);s.store_scale(1531, 1522, s.v[1532]);s.store_offset_scaled(1560, 1531, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(1561, 1560, 1560, 8.0, 0.0, 1560);s.store_sub(1562, 237, 1558);s.store_mul_add_rhs(1530, 225, 1557, 1547);s.store_sub_from_scalar_scaled_mul_mixed_ia(1563, (7.0 * 1.414213562373095), 1531, A::offset(s.ad_value(1530), (-2.0)), 9.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_68(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1628]) {s.store_square(1564, 1563);}
        s.b[1629] = (s.v[1561] < (s.v[1564] * 1e-8));s.store_scalar(1629, if s.b[1629] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1628]) && s.b[1629]) {s.store_add_scaled_inputs_product_mixed_aaia(1566, A::offset(s.ad_value(1563), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1561), 0.5, s.ad_value(1563), 1.0), 1.0, 1531, A::offset(s.ad_value(1530), (-2.0)), 9.0);}
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1628]) && (!s.b[1629])) {s.store_sqrt_add(1565, 1561, 1564);s.store_add_scaled_offset_product_rhs_mixed_aii(1566, A::offset(s.ad_value(1565), ((-7.0) * 1.414213562373095)), 1.0, 1531, 1530, (-2.0), 9.0);}
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1628]) {s.store_powf(1567, 1566, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(1568, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1531), 12.0)), 1.0, 1567, 2.0, 1567, 1567, 1.414213562373095);s.store_div(1569, 1568, 1567);s.store_add_scaled_product_indices(1570, 1547, (-1.0), 1569, 227, 1.0);s.store_add(1522, 1570, 1547);s.store_div(1523, 1522, 1562);s.store_sqrt_square_offset(1524, 1523, 1.0);s.store_sub_div_lhs_indices(1571, 1522, 1524, 1547);s.store_sub(1523, 1557, 1571);s.store_scale(459, 1523, s.v[1532]);s.copy_ad(458, 459);}
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) {s.store_scalar(1569, 3.0);s.store_sub_div_lhs_indices(1572, 1569, 225, 1547);s.store_exp_neg_input(1531, 1569);s.store_offset_div_scaled_inputs2_mixed_aia(1530, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1557), s.ad_value(1547))), (-1.0)), 4.0, 1531, 4.0, A::mul(s.ad_value(1556), s.ad_value(226)), 1.0, 1.0);}
        s.b[1630] = (s.v[1530] < (10.0 * 2.220446049250313e-16));s.store_scalar(1630, if s.b[1630] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1630]) {s.store_scalar(1530, (10.0 * 2.220446049250313e-16));}
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) {s.store_add_product3_rhs_mixed_iia(1572, 1557, 1556, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1530))), 1.0 / (2.0));s.store_mul_add_rhs(1569, 225, 1572, 1547);s.store_exp_neg_input(1531, 1569);s.store_offset_div_scaled_inputs2_mixed_aia(1530, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1557), s.ad_value(1547))), (-1.0)), 4.0, 1531, 4.0, A::mul(s.ad_value(1556), s.ad_value(226)), 1.0, 1.0);}
        s.b[1631] = (s.v[1530] < (10.0 * 2.220446049250313e-16));s.store_scalar(1631, if s.b[1631] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1631]) {s.store_scalar(1530, (10.0 * 2.220446049250313e-16));}
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) {s.store_add_product3_rhs_mixed_iia(1572, 1557, 1556, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1530))), 1.0 / (2.0));s.store_mul_add_rhs(1569, 225, 1572, 1547);}
        s.b[1632] = (s.v[1569] < 3.0);s.store_scalar(1632, if s.b[1632] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1632]) {s.store_scalar(1573, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));s.store_scalar(1574, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));s.store_offset_div_from_scalar_ad(1575, 1.0, A::mul(s.ad_value(225), s.ad_value(1555)), (1.0 / 1.414213562373095));s.store_div_scaled_inputs2_indices(1576, 1557, -1.0, 1547, -1.0, 1555, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_69(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1632]) {s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(1577, A::div_scaled_product(A::square(s.ad_value(1574)), s.ad_value(1574), 1.0, A::mul3_scaled_output(s.ad_value(1573), s.ad_value(1573), s.ad_value(1573), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1574), s.ad_value(1575), 1.0, s.ad_value(1573), s.ad_value(1573), 6.0), (-1.0), 1576, 1.0, 1573, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(1578, A::add_scaled_square_product(s.ad_value(1574), (-1.0), s.ad_value(1573), s.ad_value(1575), 3.0), 1.0, 1573, 1573, 9.0);s.store_sqrt_add_scaled_square_cube_product(1526, 1577, 1.0, 1578, 1.0);s.store_powf_ad(1579, A::sub(s.ad_value(1526), s.ad_value(1577)), 0.3333333333333333);s.store_neg_powf_add_input(1580, 1577, 1526, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(1530, 1579, 1.0, 1580, 1.0, 1574, 1.0, 1573, 3.0, -1.0);s.store_add_scaled_product_indices(1572, 1547, (-1.0), 1530, 227, 1.0);s.store_mul_add_rhs(1569, 225, 1572, 1547);}
        s.b[1633] = (p.p41 > 0.0);s.store_scalar(1633, if s.b[1633] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1633]) {s.store_offset_add(1581, 1557, 1547, 0.1);s.store_offset_exp_ad(1588, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1547), -1.0), 1e-50);s.store_scale(1521, 230, 1.0 / (s.v[69]));s.store_square(1582, 1521);s.store_mul(1583, 1582, 1588);s.store_mul(1521, 226, 1556);s.store_mul(1584, 225, 1581);s.store_add_scaled_inputs_product_mixed_aaii(1585, A::ln(A::add_scaled_square_product(s.ad_value(1584), 1.0, s.ad_value(1583), s.ad_value(1521), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1582), s.ad_value(1521))), (-1.0), 225, 1547, 1.0);s.store_offset_sub(44, 1584, 1585, (-1.0));s.store_scale(45, 1584, 4.0);}
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1633]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1633]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1522, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1523, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1585, 1584, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub(1584, 1584, 1585);s.store_add_scaled_inputs(1584, 1584, 1.0, 225, 0.1);s.store_add_scaled_inputs_product_mixed_aaii(1586, A::ln(A::add_scaled_square_product(s.ad_value(1584), 1.0, s.ad_value(1583), s.ad_value(1521), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1582), s.ad_value(1521))), (-1.0), 225, 1547, 1.0);s.copy_ad(1587, 1569);s.store_offset_sub(44, 1586, 1587, (-(0.0008 * 75.0)));s.store_scale(45, 1586, (4.0 * (0.0008 * 75.0)));}
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1633]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1633]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1522, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1523, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1569, 1586, 1.0, 44, (-0.5), 45, (-0.5));}
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) {s.store_sub_div_lhs_indices(1571, 1569, 225, 1547);s.store_add_offset_lhs_mixed_ia(1522, 1569, (-1.0), A::exp_scaled_input(s.ad_value(1569), -1.0));}
        s.b[1634] = (s.v[1522] < (10.0 * 2.220446049250313e-16));s.store_scalar(1634, if s.b[1634] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1634]) {s.store_scalar(1522, (10.0 * 2.220446049250313e-16));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_70(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) {s.store_sqrt(1523, 1522);s.store_mul(458, 1554, 1523);s.store_scaled_sub(459, 1557, 1571, s.v[1532]);}
        s.b[1635] = (p.p41 == 1.0);s.store_scalar(1635, if s.b[1635] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) {s.store_exp_ad(1588, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1547), -1.0));s.store_scale(1521, 230, 1.0 / (s.v[69]));s.store_square(1582, 1521);s.store_mul(1597, 1582, 1588);s.store_scalar(1544, 0.0);s.store_scalar(1591, 0.0);s.store_scalar(1595, 0.0);s.store_scalar(167, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_71(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut tb: usize = 0;
        while {
            let t8: f64 = (2.0 * 20.0);let t9: f64 = (t8 + 1.0);let ta: f64 = if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (s.v[167] <= t9)) { 1.0 } else { 0.0 };
            ta != 0.0
        } {
            tb += 1;assert!(tb <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) {s.store_scalar(1593, 0.0);s.store_mul_add_rhs(1569, 225, 1571, 1547);}
            s.b[1636] = (s.v[1569] < 5.0);s.store_scalar(1636, if s.b[1636] { 1.0 } else { 0.0 });
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && s.b[1636]) {s.store_mul3_ad_middle(1589, A::square(s.ad_value(1569)), 1569, A::offset(A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(1590, A::square(s.ad_value(1569)), A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul3_lhs(1591, 1597, 1589, 1589);s.store_mul_product3_indices(1592, 1590, 1597, 225, 1589, 2.0);s.store_mul_scale_offset_mixed_ia(1593, 1569, A::mul_offset_rhs(s.ad_value(1569), A::mul_offset_rhs(s.ad_value(1569), A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(1594, 1569, A::mul_offset_rhs(s.ad_value(1569), A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_offset_ad(1595, A::add(A::square(s.ad_value(1593)), s.ad_value(1591)), 1e-50);s.store_div_scaled_inputs2_mixed_aii(1596, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1594), s.ad_value(1593), 2.0), 1.0, 1592, 1.0, 1595, 2.0);}
            s.b[1637] = (s.v[1569] < 80.0);s.store_scalar(1637, if s.b[1637] { 1.0 } else { 0.0 });
            if (((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1636])) && s.b[1637]) {s.store_exp(243, 1569);s.store_mul_scale_offset_indices(1591, 1597, 243, 1.0, (-1.0));s.store_mul3_lhs(1592, 1597, 225, 243);}
            if (((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1636])) && (!s.b[1637])) {s.store_exp_mul(1598, 225, 1571);s.store_mul_sub_rhs(1591, 1582, 1598, 1588);s.store_mul3_lhs(1592, 1582, 225, 1598);}
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1636])) {s.store_sqrt_add_ad(1595, A::offset(s.ad_value(1569), (-1.0)), s.ad_value(1591));s.store_scale_ad(1596, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1592), 1.0, s.ad_value(1595), 1.0), 0.5);}
            if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) {s.store_add_scaled_inputs_product_indices(1599, 1557, 1.0, 1571, (-1.0), 1555, 1595, (-1.0));s.store_sub_from_scalar_scaled_mul(1600, (-1.0), 1555, 1596, 1.0);}
            s.b[1638] = (s.v[1544] == 1.0);s.store_scalar(1638, if s.b[1638] { 1.0 } else { 0.0 });
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && s.b[1638]) {s.store_scalar(167, ((2.0 * 20.0) + 1.0));}
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1638])) {s.store_div_scaled_inputs_indices(494, 1599, -1.0, 1600, 1.0);}
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1638])) {
                s.store_scaled_offset_ad(1601, {
                    if (1.0 >= ((s.v[1571]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1571))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1639] = (((s.v[494]) as f64).abs() > s.v[1601]);s.store_scalar(1639, if s.b[1639] { 1.0 } else { 0.0 });
            if (((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1638])) && s.b[1639]) {s.store_scale(494, 1601, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1638])) {s.store_add(1571, 1571, 494);}
            s.b[1640] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1599]) as f64).abs() <= 1e-8));s.store_scalar(1640, if s.b[1640] { 1.0 } else { 0.0 });
            if (((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1638])) && s.b[1640]) {s.store_scalar(1544, 1.0);}
            if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) {s.store_primal_offset(167, 167, 1.0);}
        }
        s.b[1642] = (s.v[1569] < 5.0);s.store_scalar(1642, if s.b[1642] { 1.0 } else { 0.0 });
        if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && s.b[1642]) {s.store_offset_square(1602, 1593, (10.0 * 2.220446049250313e-16));s.store_offset(1603, 1593, (10.0 * 2.220446049250313e-16));}
        if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1642])) {s.store_offset(1602, 1569, (-1.0));s.store_sqrt(1603, 1602);}
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) {s.store_mul(458, 1554, 1603);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_72(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) {s.store_div_from_scalar_add_ad(1522, 1.0, s.ad_value(1595), s.ad_value(1603));s.store_mul3_lhs(460, 1554, 1591, 1522);s.store_add(459, 458, 460);}
        if (((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) {s.store_sub(460, 459, 458);}
        if (((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) {
            if (p.p43 == 1.0) {
                s.store_mul(1525, 287, 1534);
            } else {
                s.store_mul(1525, 108, 1534);
            }
        }
        s.b[1644] = (((s.v[1540] != 0.0) && (p.p43 == 0.0)) || ((s.v[1538] != 0.0) && (p.p43 == 1.0)));s.store_scalar(1644, if s.b[1644] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1644]) {s.store_mul(455, 1525, 459);s.store_mul(457, 1525, 458);}
        s.b[1645] = (((s.v[1541] != 0.0) && (p.p43 == 0.0)) || ((s.v[1539] != 0.0) && (p.p43 == 1.0)));s.store_scalar(1645, if s.b[1645] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1645]) {s.store_mul(454, 1525, 459);s.store_mul(456, 1525, 458);}
        if ((p.p24 != 0.0) && s.b[1604]) {s.store_primal_add_scaled_inputs(266, 462, s.v[566], 461, s.v[565]);}
        if (((p.p24 != 0.0) && s.b[1604]) && (s.v[266] != 0.0)) {s.store_add_scaled_inputs(269, 462, p.p170, 461, p.p169);}
        s.b[1646] = (p.p43 == 1.0);s.store_scalar(1646, if s.b[1646] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1604]) && (s.v[266] != 0.0)) && s.b[1646]) {s.store_add_scaled_products_indices(1522, 462, 287, 1.0, 461, 288, 1.0);s.store_mul_scale_offset_indices(269, 269, 1522, -1.0, 0.0);}
        if ((((p.p24 != 0.0) && s.b[1604]) && (s.v[266] != 0.0)) && (!s.b[1646])) {s.store_mul_scale_offset_indices(269, 269, 108, -1.0, 0.0);}
        if (((p.p24 != 0.0) && s.b[1604]) && (s.v[266] != 0.0)) {s.store_add_scaled_product_right_sub(268, 268, 1.0, 269, 158, 157, -1.0);}
        if ((p.p24 != 0.0) && s.b[1604]) {s.store_primal_add_scaled_inputs(266, 461, s.v[566], 462, s.v[565]);}
        if (((p.p24 != 0.0) && s.b[1604]) && (s.v[266] != 0.0)) {s.store_add_scaled_inputs(270, 461, p.p170, 462, p.p169);}
        s.b[1647] = (p.p43 == 1.0);s.store_scalar(1647, if s.b[1647] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1604]) && (s.v[266] != 0.0)) && s.b[1647]) {s.store_add_scaled_products_indices(1522, 461, 287, 1.0, 462, 288, 1.0);s.store_mul_scale_offset_indices(270, 270, 1522, -1.0, 0.0);}
        if ((((p.p24 != 0.0) && s.b[1604]) && (s.v[266] != 0.0)) && (!s.b[1647])) {s.store_mul_scale_offset_indices(270, 270, 108, -1.0, 0.0);}
        if (((p.p24 != 0.0) && s.b[1604]) && (s.v[266] != 0.0)) {s.store_add_scaled_product_indices(267, 267, 1.0, 270, 158, -1.0);}
        s.b[1648] = (((s.v[613] == 1.0) && (!s.b[565])) || ((s.v[613] != 1.0) && (!s.b[566])));s.store_scalar(1648, if s.b[1648] { 1.0 } else { 0.0 });s.b[1649] = (p.p43 == 1.0);s.store_scalar(1649, if s.b[1649] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && (!s.b[1604])) && s.b[1648]) && s.b[1649]) {s.store_scale(269, 288, ((-s.v[1532]) * p.p188));}
        if ((((p.p24 != 0.0) && (!s.b[1604])) && s.b[1648]) && (!s.b[1649])) {s.store_scale(269, 108, ((-s.v[1532]) * p.p188));}
        if (((p.p24 != 0.0) && (!s.b[1604])) && (!s.b[1648])) {s.store_add_scaled_inputs(269, 462, p.p170, 461, p.p169);}
        s.b[1650] = (p.p43 == 1.0);s.store_scalar(1650, if s.b[1650] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && (!s.b[1604])) && (!s.b[1648])) && s.b[1650]) {s.store_add_scaled_products_indices(1522, 462, 287, 1.0, 461, 288, 1.0);s.store_mul_scale_offset_indices(269, 269, 1522, -1.0, 0.0);}
        if ((((p.p24 != 0.0) && (!s.b[1604])) && (!s.b[1648])) && (!s.b[1650])) {s.store_mul_scale_offset_indices(269, 269, 108, -1.0, 0.0);}
        if ((p.p24 != 0.0) && (!s.b[1604])) {s.store_mul_sub_scaled_inputs_rhs_indices(268, 269, 158, -1.0, 157, -1.0);}
        s.b[1651] = (((s.v[613] == 1.0) && (!s.b[566])) || ((s.v[613] != 1.0) && (!s.b[565])));s.store_scalar(1651, if s.b[1651] { 1.0 } else { 0.0 });s.b[1652] = (p.p43 == 1.0);s.store_scalar(1652, if s.b[1652] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && (!s.b[1604])) && s.b[1651]) && s.b[1652]) {s.store_scale(270, 287, ((-s.v[1532]) * p.p188));}
        if ((((p.p24 != 0.0) && (!s.b[1604])) && s.b[1651]) && (!s.b[1652])) {s.store_scale(270, 108, ((-s.v[1532]) * p.p188));}
        if (((p.p24 != 0.0) && (!s.b[1604])) && (!s.b[1651])) {s.store_add_scaled_inputs(270, 461, p.p170, 462, p.p169);}
        s.b[1653] = (p.p43 == 1.0);s.store_scalar(1653, if s.b[1653] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && (!s.b[1604])) && (!s.b[1651])) && s.b[1653]) {s.store_add_scaled_products_indices(1522, 461, 287, 1.0, 462, 288, 1.0);s.store_mul_scale_offset_indices(270, 270, 1522, -1.0, 0.0);}
        if ((((p.p24 != 0.0) && (!s.b[1604])) && (!s.b[1651])) && (!s.b[1653])) {s.store_mul_scale_offset_indices(270, 270, 108, -1.0, 0.0);}
        if ((p.p24 != 0.0) && (!s.b[1604])) {s.store_mul_scale_offset_indices(267, 158, 270, -1.0, 0.0);}
        s.b[1654] = (p.p43 == 1.0);s.store_scalar(1654, if s.b[1654] { 1.0 } else { 0.0 });
        if s.b[1654] {s.copy_ad(1670, 590);s.copy_ad(1671, 591);s.store_scale_ad(1672, A::exp_scaled_input(A::add_scaled_inputs(A::sub_from_scalar((s.v[87] * s.v[114]), A::mul(s.ad_value(237), s.ad_value(225))), 1.0, A::ln_scaled_input(s.ad_value(429), 1.0 / (s.v[81])), p.p175), 1.0 / (p.p174)), p.p173);s.store_scale_ad(1673, A::exp_scaled_input(A::add_scaled_inputs(A::sub_from_scalar((s.v[87] * s.v[114]), A::mul(s.ad_value(237), s.ad_value(225))), 1.0, A::ln_scaled_input(s.ad_value(429), 1.0 / (s.v[81])), p.p176), 1.0 / (p.p174)), p.p173);s.store_scaled_mul(1677, 286, 1672, p.p237);s.store_scaled_mul(1679, 286, 1673, p.p237);s.store_scaled_mul(1678, 285, 1672, p.p237);s.store_scaled_mul(1680, 285, 1673, p.p237);s.store_scale(1656, 429, 1.0 / (s.v[81]));s.store_square(1655, 1656);s.store_offset(1657, 1677, 1e-50);s.store_offset(1658, 1678, 1e-50);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_73(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1654] {s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1675, p.p174, 225, A::ln_offset_div_scaled_inputs(s.ad_value(1655), p.p177, s.ad_value(1657), 1.0, 1.0));s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1676, p.p174, 225, A::ln_offset_div_scaled_inputs(s.ad_value(1655), p.p177, s.ad_value(1658), 1.0, 1.0));s.store_scale(1674, 227, p.p174);}
        s.b[1683] = (s.v[1670] < s.v[1675]);s.store_scalar(1683, if s.b[1683] { 1.0 } else { 0.0 });
        if (s.b[1654] && s.b[1683]) {s.store_exp_div(1656, 1670, 1674);s.store_mul_scale_offset_indices(282, 1677, 1656, 1.0, (-1.0));}
        if (s.b[1654] && (!s.b[1683])) {s.store_exp_div(1656, 1675, 1674);s.store_add_scaled_offset_product_rhs_mixed_aii(282, A::mul3(A::div(s.ad_value(1677), s.ad_value(1674)), s.ad_value(1656), A::sub(s.ad_value(1670), s.ad_value(1675))), 1.0, 1677, 1656, (-1.0), 1.0);}
        if s.b[1654] {s.store_add_scaled_product_indices(282, 282, 1.0, 1670, 1679, p.p178);}
        s.b[1684] = (s.v[1671] < s.v[1676]);s.store_scalar(1684, if s.b[1684] { 1.0 } else { 0.0 });
        if (s.b[1654] && s.b[1684]) {s.store_exp_div(1656, 1671, 1674);s.store_mul_scale_offset_indices(281, 1678, 1656, 1.0, (-1.0));}
        if (s.b[1654] && (!s.b[1684])) {s.store_exp_div(1656, 1676, 1674);s.store_add_scaled_offset_product_rhs_mixed_aii(281, A::mul3(A::div(s.ad_value(1678), s.ad_value(1674)), s.ad_value(1656), A::sub(s.ad_value(1671), s.ad_value(1676))), 1.0, 1678, 1656, (-1.0), 1.0);}
        if s.b[1654] {s.store_add_scaled_product_indices(281, 281, 1.0, 1671, 1680, p.p178);s.store_add_scaled_inputs(282, 282, 1.0, 1670, s.v[142]);s.store_add_scaled_inputs(281, 281, 1.0, 1671, s.v[142]);s.store_scalar(1664, (p.p179 * p.p2));s.store_scalar(1665, (p.p179 * p.p3));s.store_scalar(1663, (p.p237 - p.p238));}
        s.b[1685] = (s.v[1663] <= 0.0);s.store_scalar(1685, if s.b[1685] { 1.0 } else { 0.0 });
        if (s.b[1654] && s.b[1685]) {s.store_scalar(1664, 0.0);s.store_scalar(1665, 0.0);}
        s.b[1686] = (p.p5 > s.v[287]);s.store_scalar(1686, if s.b[1686] { 1.0 } else { 0.0 });
        if (s.b[1654] && s.b[1686]) {s.store_primal_offset_scaled(1667, 287, (-p.p180), ((p.p5) * (p.p180)));s.store_primal_scale(1669, 287, p.p181);}
        s.b[1687] = (s.v[1671] < 0.0);s.store_scalar(1687, if s.b[1687] { 1.0 } else { 0.0 });s.b[1688] = (s.v[1665] > 0.0);s.store_scalar(1688, if s.b[1688] { 1.0 } else { 0.0 });
        if (((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1688]) {s.store_sub_from_scalar_scaled_input(1681, 1.0, 1671, 1.0 / (p.p185));}
        s.b[1689] = (p.p182 == 0.5);s.store_scalar(1689, if s.b[1689] { 1.0 } else { 0.0 });
        if ((((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1688]) && s.b[1689]) {s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));}
        if ((((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1688]) && (!s.b[1689])) {s.store_powf(1682, 1681, (-p.p182));}
        if (((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1688]) {s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(283, 1665, 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), 1.0, (p.p185 * 1.0 / ((1.0 - p.p182))));}
        if (((s.b[1654] && s.b[1686]) && s.b[1687]) && (!s.b[1688])) {s.store_scalar(283, 0.0);}
        s.b[1690] = (s.v[1667] > 0.0);s.store_scalar(1690, if s.b[1690] { 1.0 } else { 0.0 });
        if (((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1690]) {s.store_sub_from_scalar_scaled_input(1681, 1.0, 1671, 1.0 / (p.p186));}
        s.b[1691] = (p.p183 == 0.5);s.store_scalar(1691, if s.b[1691] { 1.0 } else { 0.0 });
        if ((((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1690]) && s.b[1691]) {s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));}
        if ((((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1690]) && (!s.b[1691])) {s.store_powf(1682, 1681, (-p.p183));}
        if (((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1690]) {s.store_add_mixed_ia(283, 283, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1667), 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), (p.p186 * 1.0 / ((1.0 - p.p183)))));}
        s.b[1692] = (s.v[1669] > 0.0);s.store_scalar(1692, if s.b[1692] { 1.0 } else { 0.0 });
        if (((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1692]) {s.store_sub_from_scalar_scaled_input(1681, 1.0, 1671, 1.0 / (p.p187));}
        s.b[1693] = (p.p184 == 0.5);s.store_scalar(1693, if s.b[1693] { 1.0 } else { 0.0 });
        if ((((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1692]) && s.b[1693]) {s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));}
        if ((((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1692]) && (!s.b[1693])) {s.store_powf(1682, 1681, (-p.p184));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_74(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1692]) {s.store_add_mixed_ia(283, 283, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1669), 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), (p.p187 * 1.0 / ((1.0 - p.p184)))));}
        if ((s.b[1654] && s.b[1686]) && (!s.b[1687])) {s.store_add_scaled_inputs3_indices(1656, 1665, 1.0, 1667, 1.0, 1669, 1.0);s.store_add_scaled_inputs3_indices(1657, 1665, (p.p182 * 1.0 / (p.p185)), 1667, (p.p183 * 1.0 / (p.p186)), 1669, (p.p184 * 1.0 / (p.p187)));s.store_mul_add_scaled_product_rhs_indices(283, 1671, 1656, 1.0, 1671, 1657, 0.5);}
        if (s.b[1654] && (!s.b[1686])) {s.store_scalar(1669, (p.p181 * p.p5));}
        s.b[1694] = (s.v[1671] < 0.0);s.store_scalar(1694, if s.b[1694] { 1.0 } else { 0.0 });s.b[1695] = (s.v[1665] > 0.0);s.store_scalar(1695, if s.b[1695] { 1.0 } else { 0.0 });
        if (((s.b[1654] && (!s.b[1686])) && s.b[1694]) && s.b[1695]) {s.store_sub_from_scalar_scaled_input(1681, 1.0, 1671, 1.0 / (p.p185));}
        s.b[1696] = (p.p182 == 0.5);s.store_scalar(1696, if s.b[1696] { 1.0 } else { 0.0 });
        if ((((s.b[1654] && (!s.b[1686])) && s.b[1694]) && s.b[1695]) && s.b[1696]) {s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));}
        if ((((s.b[1654] && (!s.b[1686])) && s.b[1694]) && s.b[1695]) && (!s.b[1696])) {s.store_powf(1682, 1681, (-p.p182));}
        if (((s.b[1654] && (!s.b[1686])) && s.b[1694]) && s.b[1695]) {s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(283, 1665, 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), 1.0, (p.p185 * 1.0 / ((1.0 - p.p182))));}
        if (((s.b[1654] && (!s.b[1686])) && s.b[1694]) && (!s.b[1695])) {s.store_scalar(283, 0.0);}
        s.b[1697] = (s.v[1669] > 0.0);s.store_scalar(1697, if s.b[1697] { 1.0 } else { 0.0 });
        if (((s.b[1654] && (!s.b[1686])) && s.b[1694]) && s.b[1697]) {s.store_sub_from_scalar_scaled_input(1681, 1.0, 1671, 1.0 / (p.p187));}
        s.b[1698] = (p.p184 == 0.5);s.store_scalar(1698, if s.b[1698] { 1.0 } else { 0.0 });
        if ((((s.b[1654] && (!s.b[1686])) && s.b[1694]) && s.b[1697]) && s.b[1698]) {s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));}
        if ((((s.b[1654] && (!s.b[1686])) && s.b[1694]) && s.b[1697]) && (!s.b[1698])) {s.store_powf(1682, 1681, (-p.p184));}
        if (((s.b[1654] && (!s.b[1686])) && s.b[1694]) && s.b[1697]) {s.store_add_mixed_ia(283, 283, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1669), 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), (p.p187 * 1.0 / ((1.0 - p.p184)))));}
        if ((s.b[1654] && (!s.b[1686])) && (!s.b[1694])) {s.store_add(1656, 1665, 1669);s.store_add_scaled_inputs(1657, 1665, (p.p182 * 1.0 / (p.p185)), 1669, (p.p184 * 1.0 / (p.p187)));s.store_mul_add_scaled_product_rhs_indices(283, 1671, 1656, 1.0, 1671, 1657, 0.5);}
        s.b[1699] = (p.p4 > s.v[288]);s.store_scalar(1699, if s.b[1699] { 1.0 } else { 0.0 });
        if (s.b[1654] && s.b[1699]) {s.store_primal_offset_scaled(1666, 288, (-p.p180), ((p.p4) * (p.p180)));s.store_primal_scale(1668, 288, p.p181);}
        s.b[1700] = (s.v[1670] < 0.0);s.store_scalar(1700, if s.b[1700] { 1.0 } else { 0.0 });s.b[1701] = (s.v[1664] > 0.0);s.store_scalar(1701, if s.b[1701] { 1.0 } else { 0.0 });
        if (((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1701]) {s.store_sub_from_scalar_scaled_input(1681, 1.0, 1670, 1.0 / (p.p185));}
        s.b[1702] = (p.p182 == 0.5);s.store_scalar(1702, if s.b[1702] { 1.0 } else { 0.0 });
        if ((((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1701]) && s.b[1702]) {s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));}
        if ((((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1701]) && (!s.b[1702])) {s.store_powf(1682, 1681, (-p.p182));}
        if (((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1701]) {s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(284, 1664, 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), 1.0, (p.p185 * 1.0 / ((1.0 - p.p182))));}
        if (((s.b[1654] && s.b[1699]) && s.b[1700]) && (!s.b[1701])) {s.store_scalar(284, 0.0);}
        s.b[1703] = (s.v[1666] > 0.0);s.store_scalar(1703, if s.b[1703] { 1.0 } else { 0.0 });
        if (((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1703]) {s.store_sub_from_scalar_scaled_input(1681, 1.0, 1670, 1.0 / (p.p186));}
        s.b[1704] = (p.p183 == 0.5);s.store_scalar(1704, if s.b[1704] { 1.0 } else { 0.0 });
        if ((((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1703]) && s.b[1704]) {s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));}
        if ((((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1703]) && (!s.b[1704])) {s.store_powf(1682, 1681, (-p.p183));}
        if (((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1703]) {s.store_add_mixed_ia(284, 284, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1666), 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), (p.p186 * 1.0 / ((1.0 - p.p183)))));}
        s.b[1705] = (s.v[1668] > 0.0);s.store_scalar(1705, if s.b[1705] { 1.0 } else { 0.0 });
        if (((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1705]) {s.store_sub_from_scalar_scaled_input(1681, 1.0, 1670, 1.0 / (p.p187));}
        s.b[1706] = (p.p184 == 0.5);s.store_scalar(1706, if s.b[1706] { 1.0 } else { 0.0 });
        if ((((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1705]) && s.b[1706]) {s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));}
        if ((((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1705]) && (!s.b[1706])) {s.store_powf(1682, 1681, (-p.p184));}
        if (((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1705]) {s.store_add_mixed_ia(284, 284, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1668), 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), (p.p187 * 1.0 / ((1.0 - p.p184)))));}
        if ((s.b[1654] && s.b[1699]) && (!s.b[1700])) {s.store_add_scaled_inputs3_indices(1656, 1664, 1.0, 1666, 1.0, 1668, 1.0);s.store_add_scaled_inputs3_indices(1657, 1664, (p.p182 * 1.0 / (p.p185)), 1666, (p.p183 * 1.0 / (p.p186)), 1668, (p.p184 * 1.0 / (p.p187)));s.store_mul_add_scaled_product_rhs_indices(284, 1670, 1656, 1.0, 1670, 1657, 0.5);}
        if (s.b[1654] && (!s.b[1699])) {s.store_scalar(1668, (p.p181 * p.p4));}
        s.b[1707] = (s.v[1670] < 0.0);s.store_scalar(1707, if s.b[1707] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_75(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1708] = (s.v[1664] > 0.0);s.store_scalar(1708, if s.b[1708] { 1.0 } else { 0.0 });
        if (((s.b[1654] && (!s.b[1699])) && s.b[1707]) && s.b[1708]) {s.store_sub_from_scalar_scaled_input(1681, 1.0, 1670, 1.0 / (p.p185));}
        s.b[1709] = (p.p182 == 0.5);s.store_scalar(1709, if s.b[1709] { 1.0 } else { 0.0 });
        if ((((s.b[1654] && (!s.b[1699])) && s.b[1707]) && s.b[1708]) && s.b[1709]) {s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));}
        if ((((s.b[1654] && (!s.b[1699])) && s.b[1707]) && s.b[1708]) && (!s.b[1709])) {s.store_powf(1682, 1681, (-p.p182));}
        if (((s.b[1654] && (!s.b[1699])) && s.b[1707]) && s.b[1708]) {s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(284, 1664, 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), 1.0, (p.p185 * 1.0 / ((1.0 - p.p182))));}
        if (((s.b[1654] && (!s.b[1699])) && s.b[1707]) && (!s.b[1708])) {s.store_scalar(284, 0.0);}
        s.b[1710] = (s.v[1668] > 0.0);s.store_scalar(1710, if s.b[1710] { 1.0 } else { 0.0 });
        if (((s.b[1654] && (!s.b[1699])) && s.b[1707]) && s.b[1710]) {s.store_sub_from_scalar_scaled_input(1681, 1.0, 1670, 1.0 / (p.p187));}
        s.b[1711] = (p.p184 == 0.5);s.store_scalar(1711, if s.b[1711] { 1.0 } else { 0.0 });
        if ((((s.b[1654] && (!s.b[1699])) && s.b[1707]) && s.b[1710]) && s.b[1711]) {s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));}
        if ((((s.b[1654] && (!s.b[1699])) && s.b[1707]) && s.b[1710]) && (!s.b[1711])) {s.store_powf(1682, 1681, (-p.p184));}
        if (((s.b[1654] && (!s.b[1699])) && s.b[1707]) && s.b[1710]) {s.store_add_mixed_ia(284, 284, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1668), 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), (p.p187 * 1.0 / ((1.0 - p.p184)))));}
        if ((s.b[1654] && (!s.b[1699])) && (!s.b[1707])) {s.store_add(1656, 1664, 1668);s.store_add_scaled_inputs(1657, 1664, (p.p182 * 1.0 / (p.p185)), 1668, (p.p184 * 1.0 / (p.p187)));s.store_mul_add_scaled_product_rhs_indices(284, 1670, 1656, 1.0, 1670, 1657, 0.5);}
        s.b[1712] = (s.v[1665] > 0.0);s.store_scalar(1712, if s.b[1712] { 1.0 } else { 0.0 });
        if (s.b[1654] && s.b[1712]) {s.store_scaled_mul(1659, 544, 1663, ((-1.6021918e-19) * p.p3));s.store_scale(1661, 1659, (-0.001));s.store_add_scaled_inputs3_indices(44, 1659, -1.0, 283, 1.0, 1661, -1.0);s.store_scaled_mul(45, 1659, 1661, (-4.0));}
        if (s.b[1654] && s.b[1712]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (s.b[1654] && s.b[1712]) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(283, 1659, -1.0, 44, (-0.5), 45, (-0.5));s.store_scale(283, 283, (-1.0));}
        s.b[1713] = (s.v[1664] > 0.0);s.store_scalar(1713, if s.b[1713] { 1.0 } else { 0.0 });
        if (s.b[1654] && s.b[1713]) {s.store_scaled_mul(1660, 544, 1663, ((-1.6021918e-19) * p.p2));s.store_scale(1662, 1660, (-0.001));s.store_add_scaled_inputs3_indices(44, 1660, -1.0, 284, 1.0, 1662, -1.0);s.store_scaled_mul(45, 1660, 1662, (-4.0));}
        if (s.b[1654] && s.b[1713]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (s.b[1654] && s.b[1713]) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(284, 1660, -1.0, 44, (-0.5), 45, (-0.5));s.store_scale(284, 284, (-1.0));}
        s.b[1746] = ((p.p32 != 0.0) && (s.v[145] == 0.0));s.store_scalar(1746, if s.b[1746] { 1.0 } else { 0.0 });
        if s.b[1746] {s.store_div_scaled_inputs2_indices(1729, 314, 1.0, 161, (-1.0), 441, 1.0);s.store_scaled_mul(1730, 251, 1729, 1e-5);}
        s.b[1747] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1747, if s.b[1747] { 1.0 } else { 0.0 });
        if (s.b[1746] && s.b[1747]) {s.store_scalar(1731, 1.0);}
        s.b[1748] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1748, if s.b[1748] { 1.0 } else { 0.0 });
        if ((s.b[1746] && (!s.b[1747])) && s.b[1748]) {s.copy_ad(1731, 1730);}
        if ((s.b[1746] && (!s.b[1747])) && (!s.b[1748])) {s.store_powf(1731, 1730, (p.p113 - 1.0));}
        if s.b[1746] {s.store_mul(1732, 1730, 1731);s.store_offset(1733, 1732, 1.0);s.store_powf(1734, 1733, (((-1.0) / p.p113) - 1.0));s.store_mul(1735, 1733, 1734);s.store_mul(293, 251, 1735);s.store_scaled_add(1737, 250, 293, 0.5);s.store_square(1736, 190);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_76(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1746] {s.store_div_scaled_product3_by_product_mixed_aiaai(292, A::mul3(s.ad_value(107), s.ad_value(323), s.ad_value(192)), 250, A::add_scaled_inputs3(A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(190), 3.0, 1.0), 1.0, s.ad_value(1736), 6.0), s.ad_value(293), s.ad_value(293)), 1.0, A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(190), 4.0, 3.0), 1.0, s.ad_value(1736), 3.0), s.ad_value(293), s.ad_value(250)), 1.0, A::mul3(A::add(A::scale_offset(s.ad_value(190), 3.0, 6.0), s.ad_value(1736)), s.ad_value(250), s.ad_value(250)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(441), A::offset(s.ad_value(190), 1.0), s.ad_value(1737), 15.0), 1737, 1.0);}
        if (!s.b[1746]) {s.store_scalar(292, 0.0);}
        s.b[1749] = ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (s.v[146] == 1.0)) && (s.v[145] == 0.0));s.store_scalar(1749, if s.b[1749] { 1.0 } else { 0.0 });
        if s.b[1749] {s.store_sqrt(298, 296);s.store_add(1738, 192, 298);s.store_square(1739, 294);s.store_square(1740, 296);s.store_scaled_mul(1741, 294, 296, 42.0);s.store_add_scaled_inputs3_indices(1741, 1741, 1.0, 1739, 4.0, 1740, 4.0);s.store_add_product3_rhs_mixed_iia(1741, 1741, 298, 192, A::add(s.ad_value(294), s.ad_value(296)), 20.0);s.store_square(1742, 1738);s.store_square(1734, 1742);s.store_div_scaled_value_by_product_indices(299, 1741, 1.0, 1734, 1738, 1.0);s.store_mul_ad_product_lhs_mixed_ai(300, A::div(s.ad_value(107), s.ad_value(441)), 250, 323);}
        s.store_add(199, 199, 265);s.b[1750] = (p.p43 == 1.0);s.store_scalar(1750, if s.b[1750] { 1.0 } else { 0.0 });
        if s.b[1750] {s.store_primal_add(271, 531, 532);}
        if (s.b[1750] && s.b[564]) {s.store_primal_offset(271, 271, (-(p.p168 * s.v[99])));}
        if s.b[1750] {s.store_mul_sub_scaled_inputs_rhs_indices(272, 271, 158, -1.0, 513, -1.0);s.store_scalar(276, ((3.453133e-11 / (3.141592653589793 / 2.0)) * (((1.0 + (p.p167 / s.v[88]))) as f64).ln()));s.store_primal_mul_scaled_offset_rhs(274, 276, p.p9, 518, s.v[101]);s.store_primal_mul_scaled_offset_rhs(275, 276, p.p9, 519, s.v[101]);s.store_mul_sub_rhs(277, 274, 158, 157);s.store_mul(278, 275, 158);s.store_mul_sub_scaled_inputs_rhs_indices(279, 276, 158, (p.p19 * p.p9), 513, (p.p19 * p.p9));s.store_add(268, 268, 277);s.store_add(267, 267, 278);s.store_add(272, 272, 279);}
        if ((!s.b[1750]) && s.b[564]) {s.store_scalar(271, ((-p.p168) * s.v[99]));s.store_mul_sub_scaled_inputs_rhs_indices(272, 271, 158, -1.0, 513, -1.0);}
        if ((!s.b[1750]) && (!s.b[564])) {s.store_scalar(271, 0.0);s.store_scalar(272, 0.0);}
        if (!s.b[1750]) {s.store_scalar(273, ((((3.453133e-11 / (3.141592653589793 / 2.0)) * s.v[101]) * p.p9) * (((1.0 + (p.p167 / s.v[88]))) as f64).ln()));s.copy_ad(274, 273);s.copy_ad(275, 273);s.store_mul_sub_rhs(277, 274, 158, 157);s.store_mul(278, 275, 158);s.store_add(268, 268, 277);s.store_add(267, 267, 278);}
        s.store_scale(9, 199, s.v[451]);
        if (s.v[85] != 0.0) {s.store_scalar(24, 0.0);s.store_scalar(23, 0.0);}
        s.b[1751] = (p.p43 == 1.0);s.store_scalar(1751, if s.b[1751] { 1.0 } else { 0.0 });
        if ((s.v[85] != 0.0) && s.b[1751]) {s.store_scalar(25, 0.0);s.copy_ad(556, 438);}
        if ((s.v[85] != 0.0) && (!s.b[1751])) {s.store_scalar(554, 0.0);}
        s.b[1752] = (p.p43 == 1.0);s.store_scalar(1752, if s.b[1752] { 1.0 } else { 0.0 });
        if ((s.v[85] == 0.0) && s.b[1752]) {s.store_sub_scaled_inputs(23, 196, (-s.v[451]), 197, s.v[451]);s.store_scale(24, 198, s.v[451]);s.store_scaled_sub(25, 197, 198, s.v[451]);}
        if ((s.v[85] == 0.0) && (!s.b[1752])) {s.store_add_scaled_inputs4_indices(23, 392, (-s.v[451]), 197, ((-1.0) * s.v[451]), 476, (-s.v[451]), 477, (-s.v[451]));s.store_scaled_add(24, 198, 477, s.v[451]);}
    }
}
