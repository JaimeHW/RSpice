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
            let t0: f64 = (2.0 * 20.0);let t1: f64 = (t0 + 1.0);let t2: f64 = if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (s.v[167] <= t1)) { 1.0 } else { 0.0 };
            t2 != 0.0
        } {
            t3 += 1;assert!(t3 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) {s.store_scalar(1462, 0.0);s.store_mul_add_rhs(1438, 225, 1440, 1415);}
            s.b[1502] = (s.v[1438] < 5.0);s.store_scalar(1502, if s.b[1502] { 1.0 } else { 0.0 });
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && s.b[1502]) {s.store_mul3_ad_middle(1458, A::square(s.ad_value(1438)), 1438, A::offset(A::mul(s.ad_value(1438), A::scale_offset(s.ad_value(1438), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(1459, A::square(s.ad_value(1438)), A::mul(s.ad_value(1438), A::scale_offset(s.ad_value(1438), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul3_lhs(1460, 1466, 1458, 1458);s.store_mul_product3_indices(1461, 1459, 1466, 225, 1458, 2.0);s.store_mul_scale_offset_mixed_ia(1462, 1438, A::mul_offset_rhs(s.ad_value(1438), A::mul_offset_rhs(s.ad_value(1438), A::mul(s.ad_value(1438), A::scale_offset(s.ad_value(1438), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(1463, 1438, A::mul_offset_rhs(s.ad_value(1438), A::mul(s.ad_value(1438), A::scale_offset(s.ad_value(1438), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_offset_ad(1464, A::add(A::square(s.ad_value(1462)), s.ad_value(1460)), 1e-50);s.store_div_scaled_inputs2_mixed_aii(1465, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1463), s.ad_value(1462), 2.0), 1.0, 1461, 1.0, 1464, 2.0);}
            s.b[1503] = (s.v[1438] < 80.0);s.store_scalar(1503, if s.b[1503] { 1.0 } else { 0.0 });
            if ((((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1502])) && s.b[1503]) {s.store_exp(243, 1438);s.store_mul_scale_offset_indices(1460, 1466, 243, 1.0, (-1.0));s.store_mul3_lhs(1461, 1466, 225, 243);}
            if ((((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1502])) && (!s.b[1503])) {s.store_exp_mul(1467, 225, 1440);s.store_mul_sub_rhs(1460, 1451, 1467, 1457);s.store_mul3_lhs(1461, 1451, 225, 1467);}
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1502])) {s.store_sqrt_add_ad(1464, A::offset(s.ad_value(1438), (-1.0)), s.ad_value(1460));s.store_scale_ad(1465, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1461), 1.0, s.ad_value(1464), 1.0), 0.5);}
            if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) {s.store_add_scaled_inputs_product_indices(1468, 1426, 1.0, 1440, (-1.0), 1424, 1464, (-1.0));s.store_sub_from_scalar_scaled_mul(1469, (-1.0), 1424, 1465, 1.0);}
            s.b[1504] = (s.v[1411] == 1.0);s.store_scalar(1504, if s.b[1504] { 1.0 } else { 0.0 });
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && s.b[1504]) {s.store_scalar(167, ((2.0 * 20.0) + 1.0));}
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1504])) {s.store_div_scaled_inputs_indices(494, 1468, -1.0, 1469, 1.0);}
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1504])) {
                s.store_scaled_offset_ad(1470, {
                    if (1.0 >= ((s.v[1440]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1440))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1505] = (((s.v[494]) as f64).abs() > s.v[1470]);s.store_scalar(1505, if s.b[1505] { 1.0 } else { 0.0 });
            if ((((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1504])) && s.b[1505]) {s.store_scale(494, 1470, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1504])) {s.store_add(1440, 1440, 494);}
            s.b[1506] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1468]) as f64).abs() <= 1e-8));s.store_scalar(1506, if s.b[1506] { 1.0 } else { 0.0 });
            if ((((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1504])) && s.b[1506]) {s.store_scalar(1411, 1.0);}
            if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) {s.store_primal_offset(167, 167, 1.0);}
        }
        s.b[1508] = (s.v[1438] < 5.0);s.store_scalar(1508, if s.b[1508] { 1.0 } else { 0.0 });
        if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && s.b[1508]) {s.store_offset_square(1471, 1462, (10.0 * 2.220446049250313e-16));s.store_offset(1472, 1462, (10.0 * 2.220446049250313e-16));}
        if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1508])) {s.store_offset(1471, 1438, (-1.0));s.store_sqrt(1472, 1471);}
        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) {s.store_mul(458, 1423, 1472);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_62(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) {s.store_div_from_scalar_add_ad(1395, 1.0, s.ad_value(1464), s.ad_value(1472));s.store_mul3_lhs(460, 1423, 1460, 1395);s.store_add(459, 458, 460);}
        if ((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) {s.store_sub(460, 459, 458);}
        s.b[1510] = (1.0 == 1.0);s.store_scalar(1510, if s.b[1510] { 1.0 } else { 0.0 });s.b[1511] = (1.0 == 2.0);s.store_scalar(1511, if s.b[1511] { 1.0 } else { 0.0 });
        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1510]) && (s.v[1405] != 0.0)) {s.store_mul_scale_offset_indices(463, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(465, 460, 522, -1.0, 0.0);}
        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1510]) && (s.v[1406] != 0.0)) {s.store_mul_scale_offset_indices(464, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(466, 460, 522, -1.0, 0.0);}
        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (s.b[1511] && (!s.b[1510]))) && (s.v[1405] != 0.0)) {s.store_mul_scale_offset_indices(467, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(469, 460, 522, -1.0, 0.0);}
        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (s.b[1511] && (!s.b[1510]))) && (s.v[1406] != 0.0)) {s.store_mul_scale_offset_indices(468, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(470, 460, 522, -1.0, 0.0);}
        s.store_scalar(317, p.p189);s.b[1514] = (s.v[145] != 0.0);s.store_scalar(1514, if s.b[1514] { 1.0 } else { 0.0 });
        if s.b[1514] {s.store_add(1513, 157, 161);s.store_add_scaled_inputs(314, 1513, s.v[317], 162, (1.0 - s.v[317]));}
        s.b[1515] = (p.p64 != 0.0);s.store_scalar(1515, if s.b[1515] { 1.0 } else { 0.0 });
        if (s.b[1514] && s.b[1515]) {s.store_scalar(315, 0.0);}
        s.b[1516] = (s.v[314] > ((s.v[161] + s.v[157]) - (10.0 * 2.220446049250313e-16)));s.store_scalar(1516, if s.b[1516] { 1.0 } else { 0.0 });
        if (s.b[1514] && s.b[1516]) {s.store_offset_add(314, 161, 157, (-(10.0 * 2.220446049250313e-16)));}
        s.b[1517] = (p.p64 != 0.0);s.store_scalar(1517, if s.b[1517] { 1.0 } else { 0.0 });s.b[1518] = (s.v[246] < 1e-15);s.store_scalar(1518, if s.b[1518] { 1.0 } else { 0.0 });
        if (((!s.b[1514]) && s.b[1517]) && s.b[1518]) {s.store_scalar(315, 0.0);}
        if (((!s.b[1514]) && s.b[1517]) && (!s.b[1518])) {s.store_scale(1512, 227, 1.0 / (s.v[97]));s.store_div_from_scalar(1513, 1.0, 244);s.store_mul3_lhs(315, 246, 1512, 1513);}
        s.store_scalar(1530, s.v[91]);s.store_scalar(1531, (1.0 / s.v[1530]));s.store_scalar(1551, 0.0);s.store_scalar(1591, 0.0);s.store_scalar(1589, 0.0);s.store_scalar(1593, 0.0);s.b[1602] = ((p.p29 >= 1.0) && (p.p188 > 0.0));s.store_scalar(1602, if s.b[1602] { 1.0 } else { 0.0 });
        if ((p.p24 != 0.0) && s.b[1602]) {s.store_scalar(1533, p.p171);s.store_scalar(1534, p.p172);s.copy_ad(1535, 158);s.store_scalar(1532, p.p188);}
        s.b[1603] = ((s.v[69] == 0.0) && (p.p188 > 0.0));s.store_scalar(1603, if s.b[1603] { 1.0 } else { 0.0 });
        if (((p.p24 != 0.0) && s.b[1602]) && s.b[1603]) {
            if (p.p43 == 1.0) {
                s.store_scale(1520, 287, s.v[1530]);
            } else {
                s.store_scale(1520, 108, s.v[1530]);
            }
        }
        if (((p.p24 != 0.0) && s.b[1602]) && s.b[1603]) {s.store_mul_ad_product_rhs_mixed_ia(1523, 1533, 1520, A::add(s.ad_value(1534), s.ad_value(1535)));s.store_mul(1524, 1532, 1520);s.copy_ad(1528, 161);s.store_sub_from_scalar(1525, 1.2, 1528);s.store_add_scaled_products_indices(267, 158, 1524, 1.0, 1525, 1523, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(1523, 1533, 1520, A::add_scaled_inputs3(s.ad_value(1534), 1.0, s.ad_value(1535), 1.0, s.ad_value(157), -1.0));s.store_sub(1528, 162, 157);s.store_sub_from_scalar(1525, 1.2, 1528);s.store_add_scaled_products_mixed_aiii(268, A::sub(s.ad_value(158), s.ad_value(157)), 1524, 1.0, 1523, 1525, (-1.0));}
        if (((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) {s.store_mul_sqrt_mixed_ia(1552, 238, A::div_from_scalar(s.v[69], s.ad_value(536)));s.store_scalar(1536, ((1.0 - -1.0) / 2.0));s.store_scalar(1537, ((1.0 + -1.0) / 2.0));}
        s.b[1604] = (p.p43 == 1.0);s.store_scalar(1604, if s.b[1604] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1604]) {s.store_add_scaled_products_mixed_iiia(1546, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);s.store_add_scaled_products_indices(1547, 461, 157, 1.0, 462, 157, -1.0);s.store_add_scaled_products_mixed_iiia(1548, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_sub(1549, 1547, 1546);s.store_sub(1551, 1548, 1546);s.store_neg(1550, 1546);s.store_primal_add_scaled_products_indices(1538, 1536, 461, 1.0, 1537, 462, 1.0);s.store_primal_add_scaled_products_indices(1539, 1536, 462, 1.0, 1537, 461, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_63(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1604]) {s.store_offset_ad(1544, A::add_scaled_products(s.ad_value(1538), s.ad_value(1550), 1.0, s.ad_value(1539), s.ad_value(1549), 1.0), (10.0 * 2.220446049250313e-16));}
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1604])) {s.store_primal_add_scaled_products_indices(1538, 1536, 461, 1.0, 1537, 462, 1.0);s.store_primal_add_scaled_products_indices(1539, 1536, 462, 1.0, 1537, 461, 1.0);}
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1604])) && (s.v[1536] != 0.0)) {s.store_add_scaled_products_mixed_iiia(1551, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);}
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1604])) && (s.v[1537] != 0.0)) {s.store_add_scaled_products_mixed_iiia(1551, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);}
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1604])) {s.store_scalar(1544, 0.0);}
        if (((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) {s.store_neg(1519, 1544);}
        s.b[1605] = (s.v[1519] > s.v[141]);s.store_scalar(1605, if s.b[1605] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1605]) {s.store_sub(1520, 1519, 141);s.store_sub(1521, 140, 141);s.store_div(44, 1520, 1521);s.store_square(45, 44);s.store_mul(46, 45, 44);s.store_square(47, 45);s.store_div_from_scalar_ad(1529, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));s.store_mul_scale_offset_indices(1529, 1521, 1529, -1.0, 1.0);s.store_add(1526, 141, 1529);}
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1605])) {s.copy_ad(1526, 1519);}
        if (((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) {s.store_offset_scaled(1545, 1526, -1.0, (-1e-12));s.store_scale(1553, 1552, s.v[1531]);s.store_square(1554, 1553);s.store_sub_from_scalar(1555, s.v[82], 1551);s.store_div_from_scalar(1519, s.v[69], 230);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1556, 2.0, 225, A::ln(s.ad_value(1519)));s.store_neg(1557, 1545);}
        s.b[1606] = (s.v[1555] < s.v[1557]);s.store_scalar(1606, if s.b[1606] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1606]) {s.store_div_scalar_by_product_indices(1520, 1.0, 225, 1552, 1.0);s.store_scale(1529, 1520, s.v[1530]);s.store_offset_scaled(1558, 1529, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(1559, 1558, 1558, 8.0, 0.0, 1558);s.store_sub(1560, 237, 1556);s.store_mul_add_rhs(1528, 225, 1555, 1545);s.store_sub_from_scalar_scaled_mul_mixed_ia(1561, (7.0 * 1.414213562373095), 1529, A::offset(s.ad_value(1528), (-2.0)), 9.0);s.store_square(1562, 1561);}
        s.b[1607] = (s.v[1559] < (s.v[1562] * 1e-8));s.store_scalar(1607, if s.b[1607] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1606]) && s.b[1607]) {s.store_add_scaled_inputs_product_mixed_aaia(1564, A::offset(s.ad_value(1561), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1559), 0.5, s.ad_value(1561), 1.0), 1.0, 1529, A::offset(s.ad_value(1528), (-2.0)), 9.0);}
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1606]) && (!s.b[1607])) {s.store_sqrt_add(1563, 1559, 1562);s.store_add_scaled_offset_product_rhs_mixed_aii(1564, A::offset(s.ad_value(1563), ((-7.0) * 1.414213562373095)), 1.0, 1529, 1528, (-2.0), 9.0);}
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1606]) {s.store_powf(1565, 1564, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(1566, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1529), 12.0)), 1.0, 1565, 2.0, 1565, 1565, 1.414213562373095);s.store_div(1567, 1566, 1565);s.store_add_scaled_product_indices(1568, 1545, (-1.0), 1567, 227, 1.0);s.store_add(1520, 1568, 1545);s.store_div(1521, 1520, 1560);s.store_sqrt_square_offset(1522, 1521, 1.0);s.store_sub_div_lhs_indices(1569, 1520, 1522, 1545);s.store_sub(1521, 1555, 1569);s.store_scale(459, 1521, s.v[1530]);s.copy_ad(458, 459);}
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) {s.store_scalar(1567, 3.0);s.store_sub_div_lhs_indices(1570, 1567, 225, 1545);s.store_exp_neg_input(1529, 1567);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_64(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) {s.store_offset_div_scaled_inputs2_mixed_aia(1528, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1555), s.ad_value(1545))), (-1.0)), 4.0, 1529, 4.0, A::mul(s.ad_value(1554), s.ad_value(226)), 1.0, 1.0);}
        s.b[1608] = (s.v[1528] < (10.0 * 2.220446049250313e-16));s.store_scalar(1608, if s.b[1608] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1608]) {s.store_scalar(1528, (10.0 * 2.220446049250313e-16));}
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) {s.store_add_product3_rhs_mixed_iia(1570, 1555, 1554, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1528))), 1.0 / (2.0));s.store_mul_add_rhs(1567, 225, 1570, 1545);s.store_exp_neg_input(1529, 1567);s.store_offset_div_scaled_inputs2_mixed_aia(1528, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1555), s.ad_value(1545))), (-1.0)), 4.0, 1529, 4.0, A::mul(s.ad_value(1554), s.ad_value(226)), 1.0, 1.0);}
        s.b[1609] = (s.v[1528] < (10.0 * 2.220446049250313e-16));s.store_scalar(1609, if s.b[1609] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1609]) {s.store_scalar(1528, (10.0 * 2.220446049250313e-16));}
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) {s.store_add_product3_rhs_mixed_iia(1570, 1555, 1554, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1528))), 1.0 / (2.0));s.store_mul_add_rhs(1567, 225, 1570, 1545);}
        s.b[1610] = (s.v[1567] < 3.0);s.store_scalar(1610, if s.b[1610] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1610]) {s.store_scalar(1571, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));s.store_scalar(1572, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));s.store_offset_div_from_scalar_ad(1573, 1.0, A::mul(s.ad_value(225), s.ad_value(1553)), (1.0 / 1.414213562373095));s.store_div_scaled_inputs2_indices(1574, 1555, -1.0, 1545, -1.0, 1553, 1.0);s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(1575, A::div_scaled_product(A::square(s.ad_value(1572)), s.ad_value(1572), 1.0, A::mul3_scaled_output(s.ad_value(1571), s.ad_value(1571), s.ad_value(1571), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1572), s.ad_value(1573), 1.0, s.ad_value(1571), s.ad_value(1571), 6.0), (-1.0), 1574, 1.0, 1571, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(1576, A::add_scaled_square_product(s.ad_value(1572), (-1.0), s.ad_value(1571), s.ad_value(1573), 3.0), 1.0, 1571, 1571, 9.0);s.store_sqrt_add_scaled_square_cube_product(1524, 1575, 1.0, 1576, 1.0);s.store_powf_ad(1577, A::sub(s.ad_value(1524), s.ad_value(1575)), 0.3333333333333333);s.store_neg_powf_add_input(1578, 1575, 1524, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(1528, 1577, 1.0, 1578, 1.0, 1572, 1.0, 1571, 3.0, -1.0);s.store_add_scaled_product_indices(1570, 1545, (-1.0), 1528, 227, 1.0);s.store_mul_add_rhs(1567, 225, 1570, 1545);}
        s.b[1611] = (p.p41 > 0.0);s.store_scalar(1611, if s.b[1611] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1611]) {s.store_offset_add(1579, 1555, 1545, 0.1);s.store_offset_exp_ad(1586, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1545), -1.0), 1e-50);s.store_scale(1519, 230, 1.0 / (s.v[69]));s.store_square(1580, 1519);s.store_mul(1581, 1580, 1586);s.store_mul(1519, 226, 1554);s.store_mul(1582, 225, 1579);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_65(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1611]) {s.store_add_scaled_inputs_product_mixed_aaii(1583, A::ln(A::add_scaled_square_product(s.ad_value(1582), 1.0, s.ad_value(1581), s.ad_value(1519), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1580), s.ad_value(1519))), (-1.0), 225, 1545, 1.0);s.store_offset_sub(44, 1582, 1583, (-1.0));s.store_scale(45, 1582, 4.0);}
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1611]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1611]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1520, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1521, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1583, 1582, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub(1582, 1582, 1583);s.store_add_scaled_inputs(1582, 1582, 1.0, 225, 0.1);s.store_add_scaled_inputs_product_mixed_aaii(1584, A::ln(A::add_scaled_square_product(s.ad_value(1582), 1.0, s.ad_value(1581), s.ad_value(1519), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1580), s.ad_value(1519))), (-1.0), 225, 1545, 1.0);s.copy_ad(1585, 1567);s.store_offset_sub(44, 1584, 1585, (-(0.0008 * 75.0)));s.store_scale(45, 1584, (4.0 * (0.0008 * 75.0)));}
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1611]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1611]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1520, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1521, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1567, 1584, 1.0, 44, (-0.5), 45, (-0.5));}
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) {s.store_sub_div_lhs_indices(1569, 1567, 225, 1545);s.store_add_offset_lhs_mixed_ia(1520, 1567, (-1.0), A::exp_scaled_input(s.ad_value(1567), -1.0));}
        s.b[1612] = (s.v[1520] < (10.0 * 2.220446049250313e-16));s.store_scalar(1612, if s.b[1612] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1612]) {s.store_scalar(1520, (10.0 * 2.220446049250313e-16));}
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) {s.store_sqrt(1521, 1520);s.store_mul(458, 1552, 1521);s.store_scaled_sub(459, 1555, 1569, s.v[1530]);}
        s.b[1613] = (p.p41 == 1.0);s.store_scalar(1613, if s.b[1613] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) {s.store_exp_ad(1586, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1545), -1.0));s.store_scale(1519, 230, 1.0 / (s.v[69]));s.store_square(1580, 1519);s.store_mul(1595, 1580, 1586);s.store_scalar(1542, 0.0);s.store_scalar(1589, 0.0);s.store_scalar(1593, 0.0);s.store_scalar(167, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_66(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t7: usize = 0;
        while {
            let t4: f64 = (2.0 * 20.0);let t5: f64 = (t4 + 1.0);let t6: f64 = if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (s.v[167] <= t5)) { 1.0 } else { 0.0 };
            t6 != 0.0
        } {
            t7 += 1;assert!(t7 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) {s.store_scalar(1591, 0.0);s.store_mul_add_rhs(1567, 225, 1569, 1545);}
            s.b[1614] = (s.v[1567] < 5.0);s.store_scalar(1614, if s.b[1614] { 1.0 } else { 0.0 });
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && s.b[1614]) {s.store_mul3_ad_middle(1587, A::square(s.ad_value(1567)), 1567, A::offset(A::mul(s.ad_value(1567), A::scale_offset(s.ad_value(1567), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(1588, A::square(s.ad_value(1567)), A::mul(s.ad_value(1567), A::scale_offset(s.ad_value(1567), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul3_lhs(1589, 1595, 1587, 1587);s.store_mul_product3_indices(1590, 1588, 1595, 225, 1587, 2.0);s.store_mul_scale_offset_mixed_ia(1591, 1567, A::mul_offset_rhs(s.ad_value(1567), A::mul_offset_rhs(s.ad_value(1567), A::mul(s.ad_value(1567), A::scale_offset(s.ad_value(1567), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(1592, 1567, A::mul_offset_rhs(s.ad_value(1567), A::mul(s.ad_value(1567), A::scale_offset(s.ad_value(1567), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_offset_ad(1593, A::add(A::square(s.ad_value(1591)), s.ad_value(1589)), 1e-50);s.store_div_scaled_inputs2_mixed_aii(1594, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1592), s.ad_value(1591), 2.0), 1.0, 1590, 1.0, 1593, 2.0);}
            s.b[1615] = (s.v[1567] < 80.0);s.store_scalar(1615, if s.b[1615] { 1.0 } else { 0.0 });
            if (((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1614])) && s.b[1615]) {s.store_exp(243, 1567);s.store_mul_scale_offset_indices(1589, 1595, 243, 1.0, (-1.0));s.store_mul3_lhs(1590, 1595, 225, 243);}
            if (((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1614])) && (!s.b[1615])) {s.store_exp_mul(1596, 225, 1569);s.store_mul_sub_rhs(1589, 1580, 1596, 1586);s.store_mul3_lhs(1590, 1580, 225, 1596);}
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1614])) {s.store_sqrt_add_ad(1593, A::offset(s.ad_value(1567), (-1.0)), s.ad_value(1589));s.store_scale_ad(1594, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1590), 1.0, s.ad_value(1593), 1.0), 0.5);}
            if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) {s.store_add_scaled_inputs_product_indices(1597, 1555, 1.0, 1569, (-1.0), 1553, 1593, (-1.0));s.store_sub_from_scalar_scaled_mul(1598, (-1.0), 1553, 1594, 1.0);}
            s.b[1616] = (s.v[1542] == 1.0);s.store_scalar(1616, if s.b[1616] { 1.0 } else { 0.0 });
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && s.b[1616]) {s.store_scalar(167, ((2.0 * 20.0) + 1.0));}
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1616])) {s.store_div_scaled_inputs_indices(494, 1597, -1.0, 1598, 1.0);}
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1616])) {
                s.store_scaled_offset_ad(1599, {
                    if (1.0 >= ((s.v[1569]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1569))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1617] = (((s.v[494]) as f64).abs() > s.v[1599]);s.store_scalar(1617, if s.b[1617] { 1.0 } else { 0.0 });
            if (((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1616])) && s.b[1617]) {s.store_scale(494, 1599, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1616])) {s.store_add(1569, 1569, 494);}
            s.b[1618] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1597]) as f64).abs() <= 1e-8));s.store_scalar(1618, if s.b[1618] { 1.0 } else { 0.0 });
            if (((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1616])) && s.b[1618]) {s.store_scalar(1542, 1.0);}
            if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) {s.store_primal_offset(167, 167, 1.0);}
        }
        s.b[1620] = (s.v[1567] < 5.0);s.store_scalar(1620, if s.b[1620] { 1.0 } else { 0.0 });
        if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && s.b[1620]) {s.store_offset_square(1600, 1591, (10.0 * 2.220446049250313e-16));s.store_offset(1601, 1591, (10.0 * 2.220446049250313e-16));}
        if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1620])) {s.store_offset(1600, 1567, (-1.0));s.store_sqrt(1601, 1600);}
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) {s.store_mul(458, 1552, 1601);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_67(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) {s.store_div_from_scalar_add_ad(1520, 1.0, s.ad_value(1593), s.ad_value(1601));s.store_mul3_lhs(460, 1552, 1589, 1520);s.store_add(459, 458, 460);}
        if (((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) {s.store_sub(460, 459, 458);}
        if (((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) {
            if (p.p43 == 1.0) {
                s.store_mul(1523, 287, 1532);
            } else {
                s.store_mul(1523, 108, 1532);
            }
        }
        s.b[1622] = (((s.v[1538] != 0.0) && (p.p43 == 0.0)) || ((s.v[1536] != 0.0) && (p.p43 == 1.0)));s.store_scalar(1622, if s.b[1622] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1622]) {s.store_mul(455, 1523, 459);s.store_mul(457, 1523, 458);}
        s.b[1623] = (((s.v[1539] != 0.0) && (p.p43 == 0.0)) || ((s.v[1537] != 0.0) && (p.p43 == 1.0)));s.store_scalar(1623, if s.b[1623] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1623]) {s.store_mul(454, 1523, 459);s.store_mul(456, 1523, 458);}
        if (((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) {s.store_scalar(1536, ((1.0 - 1.0) / 2.0));s.store_scalar(1537, ((1.0 + 1.0) / 2.0));}
        s.b[1624] = (p.p43 == 1.0);s.store_scalar(1624, if s.b[1624] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1624]) {s.store_add_scaled_products_mixed_iiia(1546, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);s.store_add_scaled_products_indices(1547, 461, 157, 1.0, 462, 157, -1.0);s.store_add_scaled_products_mixed_iiia(1548, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_sub(1549, 1547, 1546);s.store_sub(1551, 1548, 1546);s.store_neg(1550, 1546);s.store_primal_add_scaled_products_indices(1538, 1536, 461, 1.0, 1537, 462, 1.0);s.store_primal_add_scaled_products_indices(1539, 1536, 462, 1.0, 1537, 461, 1.0);s.store_offset_ad(1544, A::add_scaled_products(s.ad_value(1538), s.ad_value(1550), 1.0, s.ad_value(1539), s.ad_value(1549), 1.0), (10.0 * 2.220446049250313e-16));}
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1624])) {s.store_primal_add_scaled_products_indices(1538, 1536, 461, 1.0, 1537, 462, 1.0);s.store_primal_add_scaled_products_indices(1539, 1536, 462, 1.0, 1537, 461, 1.0);}
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1624])) && (s.v[1536] != 0.0)) {s.store_add_scaled_products_mixed_iiia(1551, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);}
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1624])) && (s.v[1537] != 0.0)) {s.store_add_scaled_products_mixed_iiia(1551, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);}
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1624])) {s.store_scalar(1544, 0.0);}
        if (((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) {s.store_neg(1519, 1544);}
        s.b[1625] = (s.v[1519] > s.v[141]);s.store_scalar(1625, if s.b[1625] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1625]) {s.store_sub(1520, 1519, 141);s.store_sub(1521, 140, 141);s.store_div(44, 1520, 1521);s.store_square(45, 44);s.store_mul(46, 45, 44);s.store_square(47, 45);s.store_div_from_scalar_ad(1529, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));s.store_mul_scale_offset_indices(1529, 1521, 1529, -1.0, 1.0);s.store_add(1526, 141, 1529);}
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1625])) {s.copy_ad(1526, 1519);}
        if (((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) {s.store_offset_scaled(1545, 1526, -1.0, (-1e-12));s.store_scale(1553, 1552, s.v[1531]);s.store_square(1554, 1553);s.store_sub_from_scalar(1555, s.v[82], 1551);s.store_div_from_scalar(1519, s.v[69], 230);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1556, 2.0, 225, A::ln(s.ad_value(1519)));s.store_neg(1557, 1545);}
        s.b[1626] = (s.v[1555] < s.v[1557]);s.store_scalar(1626, if s.b[1626] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1626]) {s.store_div_scalar_by_product_indices(1520, 1.0, 225, 1552, 1.0);s.store_scale(1529, 1520, s.v[1530]);s.store_offset_scaled(1558, 1529, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(1559, 1558, 1558, 8.0, 0.0, 1558);s.store_sub(1560, 237, 1556);s.store_mul_add_rhs(1528, 225, 1555, 1545);s.store_sub_from_scalar_scaled_mul_mixed_ia(1561, (7.0 * 1.414213562373095), 1529, A::offset(s.ad_value(1528), (-2.0)), 9.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_68(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1626]) {s.store_square(1562, 1561);}
        s.b[1627] = (s.v[1559] < (s.v[1562] * 1e-8));s.store_scalar(1627, if s.b[1627] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1626]) && s.b[1627]) {s.store_add_scaled_inputs_product_mixed_aaia(1564, A::offset(s.ad_value(1561), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1559), 0.5, s.ad_value(1561), 1.0), 1.0, 1529, A::offset(s.ad_value(1528), (-2.0)), 9.0);}
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1626]) && (!s.b[1627])) {s.store_sqrt_add(1563, 1559, 1562);s.store_add_scaled_offset_product_rhs_mixed_aii(1564, A::offset(s.ad_value(1563), ((-7.0) * 1.414213562373095)), 1.0, 1529, 1528, (-2.0), 9.0);}
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1626]) {s.store_powf(1565, 1564, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(1566, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1529), 12.0)), 1.0, 1565, 2.0, 1565, 1565, 1.414213562373095);s.store_div(1567, 1566, 1565);s.store_add_scaled_product_indices(1568, 1545, (-1.0), 1567, 227, 1.0);s.store_add(1520, 1568, 1545);s.store_div(1521, 1520, 1560);s.store_sqrt_square_offset(1522, 1521, 1.0);s.store_sub_div_lhs_indices(1569, 1520, 1522, 1545);s.store_sub(1521, 1555, 1569);s.store_scale(459, 1521, s.v[1530]);s.copy_ad(458, 459);}
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) {s.store_scalar(1567, 3.0);s.store_sub_div_lhs_indices(1570, 1567, 225, 1545);s.store_exp_neg_input(1529, 1567);s.store_offset_div_scaled_inputs2_mixed_aia(1528, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1555), s.ad_value(1545))), (-1.0)), 4.0, 1529, 4.0, A::mul(s.ad_value(1554), s.ad_value(226)), 1.0, 1.0);}
        s.b[1628] = (s.v[1528] < (10.0 * 2.220446049250313e-16));s.store_scalar(1628, if s.b[1628] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1628]) {s.store_scalar(1528, (10.0 * 2.220446049250313e-16));}
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) {s.store_add_product3_rhs_mixed_iia(1570, 1555, 1554, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1528))), 1.0 / (2.0));s.store_mul_add_rhs(1567, 225, 1570, 1545);s.store_exp_neg_input(1529, 1567);s.store_offset_div_scaled_inputs2_mixed_aia(1528, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1555), s.ad_value(1545))), (-1.0)), 4.0, 1529, 4.0, A::mul(s.ad_value(1554), s.ad_value(226)), 1.0, 1.0);}
        s.b[1629] = (s.v[1528] < (10.0 * 2.220446049250313e-16));s.store_scalar(1629, if s.b[1629] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1629]) {s.store_scalar(1528, (10.0 * 2.220446049250313e-16));}
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) {s.store_add_product3_rhs_mixed_iia(1570, 1555, 1554, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1528))), 1.0 / (2.0));s.store_mul_add_rhs(1567, 225, 1570, 1545);}
        s.b[1630] = (s.v[1567] < 3.0);s.store_scalar(1630, if s.b[1630] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1630]) {s.store_scalar(1571, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));s.store_scalar(1572, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));s.store_offset_div_from_scalar_ad(1573, 1.0, A::mul(s.ad_value(225), s.ad_value(1553)), (1.0 / 1.414213562373095));s.store_div_scaled_inputs2_indices(1574, 1555, -1.0, 1545, -1.0, 1553, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_69(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1630]) {s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(1575, A::div_scaled_product(A::square(s.ad_value(1572)), s.ad_value(1572), 1.0, A::mul3_scaled_output(s.ad_value(1571), s.ad_value(1571), s.ad_value(1571), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1572), s.ad_value(1573), 1.0, s.ad_value(1571), s.ad_value(1571), 6.0), (-1.0), 1574, 1.0, 1571, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(1576, A::add_scaled_square_product(s.ad_value(1572), (-1.0), s.ad_value(1571), s.ad_value(1573), 3.0), 1.0, 1571, 1571, 9.0);s.store_sqrt_add_scaled_square_cube_product(1524, 1575, 1.0, 1576, 1.0);s.store_powf_ad(1577, A::sub(s.ad_value(1524), s.ad_value(1575)), 0.3333333333333333);s.store_neg_powf_add_input(1578, 1575, 1524, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(1528, 1577, 1.0, 1578, 1.0, 1572, 1.0, 1571, 3.0, -1.0);s.store_add_scaled_product_indices(1570, 1545, (-1.0), 1528, 227, 1.0);s.store_mul_add_rhs(1567, 225, 1570, 1545);}
        s.b[1631] = (p.p41 > 0.0);s.store_scalar(1631, if s.b[1631] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1631]) {s.store_offset_add(1579, 1555, 1545, 0.1);s.store_offset_exp_ad(1586, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1545), -1.0), 1e-50);s.store_scale(1519, 230, 1.0 / (s.v[69]));s.store_square(1580, 1519);s.store_mul(1581, 1580, 1586);s.store_mul(1519, 226, 1554);s.store_mul(1582, 225, 1579);s.store_add_scaled_inputs_product_mixed_aaii(1583, A::ln(A::add_scaled_square_product(s.ad_value(1582), 1.0, s.ad_value(1581), s.ad_value(1519), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1580), s.ad_value(1519))), (-1.0), 225, 1545, 1.0);s.store_offset_sub(44, 1582, 1583, (-1.0));s.store_scale(45, 1582, 4.0);}
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1631]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1631]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1520, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1521, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1583, 1582, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub(1582, 1582, 1583);s.store_add_scaled_inputs(1582, 1582, 1.0, 225, 0.1);s.store_add_scaled_inputs_product_mixed_aaii(1584, A::ln(A::add_scaled_square_product(s.ad_value(1582), 1.0, s.ad_value(1581), s.ad_value(1519), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1580), s.ad_value(1519))), (-1.0), 225, 1545, 1.0);s.copy_ad(1585, 1567);s.store_offset_sub(44, 1584, 1585, (-(0.0008 * 75.0)));s.store_scale(45, 1584, (4.0 * (0.0008 * 75.0)));}
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1631]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1631]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1520, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1521, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1567, 1584, 1.0, 44, (-0.5), 45, (-0.5));}
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) {s.store_sub_div_lhs_indices(1569, 1567, 225, 1545);s.store_add_offset_lhs_mixed_ia(1520, 1567, (-1.0), A::exp_scaled_input(s.ad_value(1567), -1.0));}
        s.b[1632] = (s.v[1520] < (10.0 * 2.220446049250313e-16));s.store_scalar(1632, if s.b[1632] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1632]) {s.store_scalar(1520, (10.0 * 2.220446049250313e-16));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_70(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) {s.store_sqrt(1521, 1520);s.store_mul(458, 1552, 1521);s.store_scaled_sub(459, 1555, 1569, s.v[1530]);}
        s.b[1633] = (p.p41 == 1.0);s.store_scalar(1633, if s.b[1633] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) {s.store_exp_ad(1586, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1545), -1.0));s.store_scale(1519, 230, 1.0 / (s.v[69]));s.store_square(1580, 1519);s.store_mul(1595, 1580, 1586);s.store_scalar(1542, 0.0);s.store_scalar(1589, 0.0);s.store_scalar(1593, 0.0);s.store_scalar(167, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_71(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut tb: usize = 0;
        while {
            let t8: f64 = (2.0 * 20.0);let t9: f64 = (t8 + 1.0);let ta: f64 = if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (s.v[167] <= t9)) { 1.0 } else { 0.0 };
            ta != 0.0
        } {
            tb += 1;assert!(tb <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) {s.store_scalar(1591, 0.0);s.store_mul_add_rhs(1567, 225, 1569, 1545);}
            s.b[1634] = (s.v[1567] < 5.0);s.store_scalar(1634, if s.b[1634] { 1.0 } else { 0.0 });
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && s.b[1634]) {s.store_mul3_ad_middle(1587, A::square(s.ad_value(1567)), 1567, A::offset(A::mul(s.ad_value(1567), A::scale_offset(s.ad_value(1567), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(1588, A::square(s.ad_value(1567)), A::mul(s.ad_value(1567), A::scale_offset(s.ad_value(1567), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul3_lhs(1589, 1595, 1587, 1587);s.store_mul_product3_indices(1590, 1588, 1595, 225, 1587, 2.0);s.store_mul_scale_offset_mixed_ia(1591, 1567, A::mul_offset_rhs(s.ad_value(1567), A::mul_offset_rhs(s.ad_value(1567), A::mul(s.ad_value(1567), A::scale_offset(s.ad_value(1567), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(1592, 1567, A::mul_offset_rhs(s.ad_value(1567), A::mul(s.ad_value(1567), A::scale_offset(s.ad_value(1567), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_offset_ad(1593, A::add(A::square(s.ad_value(1591)), s.ad_value(1589)), 1e-50);s.store_div_scaled_inputs2_mixed_aii(1594, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1592), s.ad_value(1591), 2.0), 1.0, 1590, 1.0, 1593, 2.0);}
            s.b[1635] = (s.v[1567] < 80.0);s.store_scalar(1635, if s.b[1635] { 1.0 } else { 0.0 });
            if (((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (!s.b[1634])) && s.b[1635]) {s.store_exp(243, 1567);s.store_mul_scale_offset_indices(1589, 1595, 243, 1.0, (-1.0));s.store_mul3_lhs(1590, 1595, 225, 243);}
            if (((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (!s.b[1634])) && (!s.b[1635])) {s.store_exp_mul(1596, 225, 1569);s.store_mul_sub_rhs(1589, 1580, 1596, 1586);s.store_mul3_lhs(1590, 1580, 225, 1596);}
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (!s.b[1634])) {s.store_sqrt_add_ad(1593, A::offset(s.ad_value(1567), (-1.0)), s.ad_value(1589));s.store_scale_ad(1594, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1590), 1.0, s.ad_value(1593), 1.0), 0.5);}
            if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) {s.store_add_scaled_inputs_product_indices(1597, 1555, 1.0, 1569, (-1.0), 1553, 1593, (-1.0));s.store_sub_from_scalar_scaled_mul(1598, (-1.0), 1553, 1594, 1.0);}
            s.b[1636] = (s.v[1542] == 1.0);s.store_scalar(1636, if s.b[1636] { 1.0 } else { 0.0 });
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && s.b[1636]) {s.store_scalar(167, ((2.0 * 20.0) + 1.0));}
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (!s.b[1636])) {s.store_div_scaled_inputs_indices(494, 1597, -1.0, 1598, 1.0);}
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (!s.b[1636])) {
                s.store_scaled_offset_ad(1599, {
                    if (1.0 >= ((s.v[1569]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1569))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1637] = (((s.v[494]) as f64).abs() > s.v[1599]);s.store_scalar(1637, if s.b[1637] { 1.0 } else { 0.0 });
            if (((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (!s.b[1636])) && s.b[1637]) {s.store_scale(494, 1599, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (!s.b[1636])) {s.store_add(1569, 1569, 494);}
            s.b[1638] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1597]) as f64).abs() <= 1e-8));s.store_scalar(1638, if s.b[1638] { 1.0 } else { 0.0 });
            if (((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (!s.b[1636])) && s.b[1638]) {s.store_scalar(1542, 1.0);}
            if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) {s.store_primal_offset(167, 167, 1.0);}
        }
        s.b[1640] = (s.v[1567] < 5.0);s.store_scalar(1640, if s.b[1640] { 1.0 } else { 0.0 });
        if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && s.b[1640]) {s.store_offset_square(1600, 1591, (10.0 * 2.220446049250313e-16));s.store_offset(1601, 1591, (10.0 * 2.220446049250313e-16));}
        if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (!s.b[1640])) {s.store_offset(1600, 1567, (-1.0));s.store_sqrt(1601, 1600);}
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) {s.store_mul(458, 1552, 1601);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_72(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) {s.store_div_from_scalar_add_ad(1520, 1.0, s.ad_value(1593), s.ad_value(1601));s.store_mul3_lhs(460, 1552, 1589, 1520);s.store_add(459, 458, 460);}
        if (((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) {s.store_sub(460, 459, 458);}
        if (((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) {
            if (p.p43 == 1.0) {
                s.store_mul(1523, 287, 1532);
            } else {
                s.store_mul(1523, 108, 1532);
            }
        }
        s.b[1642] = (((s.v[1538] != 0.0) && (p.p43 == 0.0)) || ((s.v[1536] != 0.0) && (p.p43 == 1.0)));s.store_scalar(1642, if s.b[1642] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1642]) {s.store_mul(455, 1523, 459);s.store_mul(457, 1523, 458);}
        s.b[1643] = (((s.v[1539] != 0.0) && (p.p43 == 0.0)) || ((s.v[1537] != 0.0) && (p.p43 == 1.0)));s.store_scalar(1643, if s.b[1643] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1643]) {s.store_mul(454, 1523, 459);s.store_mul(456, 1523, 458);}
        if ((p.p24 != 0.0) && s.b[1602]) {s.store_primal_add_scaled_inputs(266, 462, s.v[566], 461, s.v[565]);}
        if (((p.p24 != 0.0) && s.b[1602]) && (s.v[266] != 0.0)) {s.store_add_scaled_inputs(269, 462, p.p170, 461, p.p169);}
        s.b[1644] = (p.p43 == 1.0);s.store_scalar(1644, if s.b[1644] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1602]) && (s.v[266] != 0.0)) && s.b[1644]) {s.store_add_scaled_products_indices(1520, 462, 287, 1.0, 461, 288, 1.0);s.store_mul_scale_offset_indices(269, 269, 1520, -1.0, 0.0);}
        if ((((p.p24 != 0.0) && s.b[1602]) && (s.v[266] != 0.0)) && (!s.b[1644])) {s.store_mul_scale_offset_indices(269, 269, 108, -1.0, 0.0);}
        if (((p.p24 != 0.0) && s.b[1602]) && (s.v[266] != 0.0)) {s.store_add_scaled_product_right_sub(268, 268, 1.0, 269, 158, 157, -1.0);}
        if ((p.p24 != 0.0) && s.b[1602]) {s.store_primal_add_scaled_inputs(266, 461, s.v[566], 462, s.v[565]);}
        if (((p.p24 != 0.0) && s.b[1602]) && (s.v[266] != 0.0)) {s.store_add_scaled_inputs(270, 461, p.p170, 462, p.p169);}
        s.b[1645] = (p.p43 == 1.0);s.store_scalar(1645, if s.b[1645] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1602]) && (s.v[266] != 0.0)) && s.b[1645]) {s.store_add_scaled_products_indices(1520, 461, 287, 1.0, 462, 288, 1.0);s.store_mul_scale_offset_indices(270, 270, 1520, -1.0, 0.0);}
        if ((((p.p24 != 0.0) && s.b[1602]) && (s.v[266] != 0.0)) && (!s.b[1645])) {s.store_mul_scale_offset_indices(270, 270, 108, -1.0, 0.0);}
        if (((p.p24 != 0.0) && s.b[1602]) && (s.v[266] != 0.0)) {s.store_add_scaled_product_indices(267, 267, 1.0, 270, 158, -1.0);}
        s.b[1646] = (((s.v[613] == 1.0) && (!s.b[565])) || ((s.v[613] != 1.0) && (!s.b[566])));s.store_scalar(1646, if s.b[1646] { 1.0 } else { 0.0 });s.b[1647] = (p.p43 == 1.0);s.store_scalar(1647, if s.b[1647] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && (!s.b[1602])) && s.b[1646]) && s.b[1647]) {s.store_scale(269, 288, ((-s.v[1530]) * p.p188));}
        if ((((p.p24 != 0.0) && (!s.b[1602])) && s.b[1646]) && (!s.b[1647])) {s.store_scale(269, 108, ((-s.v[1530]) * p.p188));}
        if (((p.p24 != 0.0) && (!s.b[1602])) && (!s.b[1646])) {s.store_add_scaled_inputs(269, 462, p.p170, 461, p.p169);}
        s.b[1648] = (p.p43 == 1.0);s.store_scalar(1648, if s.b[1648] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && (!s.b[1602])) && (!s.b[1646])) && s.b[1648]) {s.store_add_scaled_products_indices(1520, 462, 287, 1.0, 461, 288, 1.0);s.store_mul_scale_offset_indices(269, 269, 1520, -1.0, 0.0);}
        if ((((p.p24 != 0.0) && (!s.b[1602])) && (!s.b[1646])) && (!s.b[1648])) {s.store_mul_scale_offset_indices(269, 269, 108, -1.0, 0.0);}
        if ((p.p24 != 0.0) && (!s.b[1602])) {s.store_mul_sub_scaled_inputs_rhs_indices(268, 269, 158, -1.0, 157, -1.0);}
        s.b[1649] = (((s.v[613] == 1.0) && (!s.b[566])) || ((s.v[613] != 1.0) && (!s.b[565])));s.store_scalar(1649, if s.b[1649] { 1.0 } else { 0.0 });s.b[1650] = (p.p43 == 1.0);s.store_scalar(1650, if s.b[1650] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && (!s.b[1602])) && s.b[1649]) && s.b[1650]) {s.store_scale(270, 287, ((-s.v[1530]) * p.p188));}
        if ((((p.p24 != 0.0) && (!s.b[1602])) && s.b[1649]) && (!s.b[1650])) {s.store_scale(270, 108, ((-s.v[1530]) * p.p188));}
        if (((p.p24 != 0.0) && (!s.b[1602])) && (!s.b[1649])) {s.store_add_scaled_inputs(270, 461, p.p170, 462, p.p169);}
        s.b[1651] = (p.p43 == 1.0);s.store_scalar(1651, if s.b[1651] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && (!s.b[1602])) && (!s.b[1649])) && s.b[1651]) {s.store_add_scaled_products_indices(1520, 461, 287, 1.0, 462, 288, 1.0);s.store_mul_scale_offset_indices(270, 270, 1520, -1.0, 0.0);}
        if ((((p.p24 != 0.0) && (!s.b[1602])) && (!s.b[1649])) && (!s.b[1651])) {s.store_mul_scale_offset_indices(270, 270, 108, -1.0, 0.0);}
        if ((p.p24 != 0.0) && (!s.b[1602])) {s.store_mul_scale_offset_indices(267, 158, 270, -1.0, 0.0);}
        s.b[1652] = (p.p43 == 1.0);s.store_scalar(1652, if s.b[1652] { 1.0 } else { 0.0 });
        if s.b[1652] {s.copy_ad(1668, 590);s.copy_ad(1669, 591);s.store_scale_ad(1670, A::exp_scaled_input(A::add_scaled_inputs(A::sub_from_scalar((s.v[87] * s.v[114]), A::mul(s.ad_value(237), s.ad_value(225))), 1.0, A::ln_scaled_input(s.ad_value(429), 1.0 / (s.v[81])), p.p175), 1.0 / (p.p174)), p.p173);s.store_scale_ad(1671, A::exp_scaled_input(A::add_scaled_inputs(A::sub_from_scalar((s.v[87] * s.v[114]), A::mul(s.ad_value(237), s.ad_value(225))), 1.0, A::ln_scaled_input(s.ad_value(429), 1.0 / (s.v[81])), p.p176), 1.0 / (p.p174)), p.p173);s.store_scaled_mul(1675, 286, 1670, p.p237);s.store_scaled_mul(1677, 286, 1671, p.p237);s.store_scaled_mul(1676, 285, 1670, p.p237);s.store_scaled_mul(1678, 285, 1671, p.p237);s.store_scale(1654, 429, 1.0 / (s.v[81]));s.store_square(1653, 1654);s.store_offset(1655, 1675, 1e-50);s.store_offset(1656, 1676, 1e-50);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_73(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1652] {s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1673, p.p174, 225, A::ln_offset_div_scaled_inputs(s.ad_value(1653), p.p177, s.ad_value(1655), 1.0, 1.0));s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1674, p.p174, 225, A::ln_offset_div_scaled_inputs(s.ad_value(1653), p.p177, s.ad_value(1656), 1.0, 1.0));s.store_scale(1672, 227, p.p174);}
        s.b[1681] = (s.v[1668] < s.v[1673]);s.store_scalar(1681, if s.b[1681] { 1.0 } else { 0.0 });
        if (s.b[1652] && s.b[1681]) {s.store_exp_div(1654, 1668, 1672);s.store_mul_scale_offset_indices(282, 1675, 1654, 1.0, (-1.0));}
        if (s.b[1652] && (!s.b[1681])) {s.store_exp_div(1654, 1673, 1672);s.store_add_scaled_offset_product_rhs_mixed_aii(282, A::mul3(A::div(s.ad_value(1675), s.ad_value(1672)), s.ad_value(1654), A::sub(s.ad_value(1668), s.ad_value(1673))), 1.0, 1675, 1654, (-1.0), 1.0);}
        if s.b[1652] {s.store_add_scaled_product_indices(282, 282, 1.0, 1668, 1677, p.p178);}
        s.b[1682] = (s.v[1669] < s.v[1674]);s.store_scalar(1682, if s.b[1682] { 1.0 } else { 0.0 });
        if (s.b[1652] && s.b[1682]) {s.store_exp_div(1654, 1669, 1672);s.store_mul_scale_offset_indices(281, 1676, 1654, 1.0, (-1.0));}
        if (s.b[1652] && (!s.b[1682])) {s.store_exp_div(1654, 1674, 1672);s.store_add_scaled_offset_product_rhs_mixed_aii(281, A::mul3(A::div(s.ad_value(1676), s.ad_value(1672)), s.ad_value(1654), A::sub(s.ad_value(1669), s.ad_value(1674))), 1.0, 1676, 1654, (-1.0), 1.0);}
        if s.b[1652] {s.store_add_scaled_product_indices(281, 281, 1.0, 1669, 1678, p.p178);s.store_add_scaled_inputs(282, 282, 1.0, 1668, s.v[142]);s.store_add_scaled_inputs(281, 281, 1.0, 1669, s.v[142]);s.store_scalar(1662, (p.p179 * p.p2));s.store_scalar(1663, (p.p179 * p.p3));s.store_scalar(1661, (p.p237 - p.p238));}
        s.b[1683] = (s.v[1661] <= 0.0);s.store_scalar(1683, if s.b[1683] { 1.0 } else { 0.0 });
        if (s.b[1652] && s.b[1683]) {s.store_scalar(1662, 0.0);s.store_scalar(1663, 0.0);}
        s.b[1684] = (p.p5 > s.v[287]);s.store_scalar(1684, if s.b[1684] { 1.0 } else { 0.0 });
        if (s.b[1652] && s.b[1684]) {s.store_primal_offset_scaled(1665, 287, (-p.p180), ((p.p5) * (p.p180)));s.store_primal_scale(1667, 287, p.p181);}
        s.b[1685] = (s.v[1669] < 0.0);s.store_scalar(1685, if s.b[1685] { 1.0 } else { 0.0 });s.b[1686] = (s.v[1663] > 0.0);s.store_scalar(1686, if s.b[1686] { 1.0 } else { 0.0 });
        if (((s.b[1652] && s.b[1684]) && s.b[1685]) && s.b[1686]) {s.store_sub_from_scalar_scaled_input(1679, 1.0, 1669, 1.0 / (p.p185));}
        s.b[1687] = (p.p182 == 0.5);s.store_scalar(1687, if s.b[1687] { 1.0 } else { 0.0 });
        if ((((s.b[1652] && s.b[1684]) && s.b[1685]) && s.b[1686]) && s.b[1687]) {s.store_div_from_scalar_sqrt_ad(1680, 1.0, s.ad_value(1679));}
        if ((((s.b[1652] && s.b[1684]) && s.b[1685]) && s.b[1686]) && (!s.b[1687])) {s.store_powf(1680, 1679, (-p.p182));}
        if (((s.b[1652] && s.b[1684]) && s.b[1685]) && s.b[1686]) {s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(283, 1663, 1.0, A::mul(s.ad_value(1679), s.ad_value(1680)), 1.0, (p.p185 * 1.0 / ((1.0 - p.p182))));}
        if (((s.b[1652] && s.b[1684]) && s.b[1685]) && (!s.b[1686])) {s.store_scalar(283, 0.0);}
        s.b[1688] = (s.v[1665] > 0.0);s.store_scalar(1688, if s.b[1688] { 1.0 } else { 0.0 });
        if (((s.b[1652] && s.b[1684]) && s.b[1685]) && s.b[1688]) {s.store_sub_from_scalar_scaled_input(1679, 1.0, 1669, 1.0 / (p.p186));}
        s.b[1689] = (p.p183 == 0.5);s.store_scalar(1689, if s.b[1689] { 1.0 } else { 0.0 });
        if ((((s.b[1652] && s.b[1684]) && s.b[1685]) && s.b[1688]) && s.b[1689]) {s.store_div_from_scalar_sqrt_ad(1680, 1.0, s.ad_value(1679));}
        if ((((s.b[1652] && s.b[1684]) && s.b[1685]) && s.b[1688]) && (!s.b[1689])) {s.store_powf(1680, 1679, (-p.p183));}
        if (((s.b[1652] && s.b[1684]) && s.b[1685]) && s.b[1688]) {s.store_add_mixed_ia(283, 283, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1665), 1.0, A::mul(s.ad_value(1679), s.ad_value(1680)), (p.p186 * 1.0 / ((1.0 - p.p183)))));}
        s.b[1690] = (s.v[1667] > 0.0);s.store_scalar(1690, if s.b[1690] { 1.0 } else { 0.0 });
        if (((s.b[1652] && s.b[1684]) && s.b[1685]) && s.b[1690]) {s.store_sub_from_scalar_scaled_input(1679, 1.0, 1669, 1.0 / (p.p187));}
        s.b[1691] = (p.p184 == 0.5);s.store_scalar(1691, if s.b[1691] { 1.0 } else { 0.0 });
        if ((((s.b[1652] && s.b[1684]) && s.b[1685]) && s.b[1690]) && s.b[1691]) {s.store_div_from_scalar_sqrt_ad(1680, 1.0, s.ad_value(1679));}
        if ((((s.b[1652] && s.b[1684]) && s.b[1685]) && s.b[1690]) && (!s.b[1691])) {s.store_powf(1680, 1679, (-p.p184));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_74(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1652] && s.b[1684]) && s.b[1685]) && s.b[1690]) {s.store_add_mixed_ia(283, 283, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1667), 1.0, A::mul(s.ad_value(1679), s.ad_value(1680)), (p.p187 * 1.0 / ((1.0 - p.p184)))));}
        if ((s.b[1652] && s.b[1684]) && (!s.b[1685])) {s.store_add_scaled_inputs3_indices(1654, 1663, 1.0, 1665, 1.0, 1667, 1.0);s.store_add_scaled_inputs3_indices(1655, 1663, (p.p182 * 1.0 / (p.p185)), 1665, (p.p183 * 1.0 / (p.p186)), 1667, (p.p184 * 1.0 / (p.p187)));s.store_mul_add_scaled_product_rhs_indices(283, 1669, 1654, 1.0, 1669, 1655, 0.5);}
        if (s.b[1652] && (!s.b[1684])) {s.store_scalar(1667, (p.p181 * p.p5));}
        s.b[1692] = (s.v[1669] < 0.0);s.store_scalar(1692, if s.b[1692] { 1.0 } else { 0.0 });s.b[1693] = (s.v[1663] > 0.0);s.store_scalar(1693, if s.b[1693] { 1.0 } else { 0.0 });
        if (((s.b[1652] && (!s.b[1684])) && s.b[1692]) && s.b[1693]) {s.store_sub_from_scalar_scaled_input(1679, 1.0, 1669, 1.0 / (p.p185));}
        s.b[1694] = (p.p182 == 0.5);s.store_scalar(1694, if s.b[1694] { 1.0 } else { 0.0 });
        if ((((s.b[1652] && (!s.b[1684])) && s.b[1692]) && s.b[1693]) && s.b[1694]) {s.store_div_from_scalar_sqrt_ad(1680, 1.0, s.ad_value(1679));}
        if ((((s.b[1652] && (!s.b[1684])) && s.b[1692]) && s.b[1693]) && (!s.b[1694])) {s.store_powf(1680, 1679, (-p.p182));}
        if (((s.b[1652] && (!s.b[1684])) && s.b[1692]) && s.b[1693]) {s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(283, 1663, 1.0, A::mul(s.ad_value(1679), s.ad_value(1680)), 1.0, (p.p185 * 1.0 / ((1.0 - p.p182))));}
        if (((s.b[1652] && (!s.b[1684])) && s.b[1692]) && (!s.b[1693])) {s.store_scalar(283, 0.0);}
        s.b[1695] = (s.v[1667] > 0.0);s.store_scalar(1695, if s.b[1695] { 1.0 } else { 0.0 });
        if (((s.b[1652] && (!s.b[1684])) && s.b[1692]) && s.b[1695]) {s.store_sub_from_scalar_scaled_input(1679, 1.0, 1669, 1.0 / (p.p187));}
        s.b[1696] = (p.p184 == 0.5);s.store_scalar(1696, if s.b[1696] { 1.0 } else { 0.0 });
        if ((((s.b[1652] && (!s.b[1684])) && s.b[1692]) && s.b[1695]) && s.b[1696]) {s.store_div_from_scalar_sqrt_ad(1680, 1.0, s.ad_value(1679));}
        if ((((s.b[1652] && (!s.b[1684])) && s.b[1692]) && s.b[1695]) && (!s.b[1696])) {s.store_powf(1680, 1679, (-p.p184));}
        if (((s.b[1652] && (!s.b[1684])) && s.b[1692]) && s.b[1695]) {s.store_add_mixed_ia(283, 283, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1667), 1.0, A::mul(s.ad_value(1679), s.ad_value(1680)), (p.p187 * 1.0 / ((1.0 - p.p184)))));}
        if ((s.b[1652] && (!s.b[1684])) && (!s.b[1692])) {s.store_add(1654, 1663, 1667);s.store_add_scaled_inputs(1655, 1663, (p.p182 * 1.0 / (p.p185)), 1667, (p.p184 * 1.0 / (p.p187)));s.store_mul_add_scaled_product_rhs_indices(283, 1669, 1654, 1.0, 1669, 1655, 0.5);}
        s.b[1697] = (p.p4 > s.v[288]);s.store_scalar(1697, if s.b[1697] { 1.0 } else { 0.0 });
        if (s.b[1652] && s.b[1697]) {s.store_primal_offset_scaled(1664, 288, (-p.p180), ((p.p4) * (p.p180)));s.store_primal_scale(1666, 288, p.p181);}
        s.b[1698] = (s.v[1668] < 0.0);s.store_scalar(1698, if s.b[1698] { 1.0 } else { 0.0 });s.b[1699] = (s.v[1662] > 0.0);s.store_scalar(1699, if s.b[1699] { 1.0 } else { 0.0 });
        if (((s.b[1652] && s.b[1697]) && s.b[1698]) && s.b[1699]) {s.store_sub_from_scalar_scaled_input(1679, 1.0, 1668, 1.0 / (p.p185));}
        s.b[1700] = (p.p182 == 0.5);s.store_scalar(1700, if s.b[1700] { 1.0 } else { 0.0 });
        if ((((s.b[1652] && s.b[1697]) && s.b[1698]) && s.b[1699]) && s.b[1700]) {s.store_div_from_scalar_sqrt_ad(1680, 1.0, s.ad_value(1679));}
        if ((((s.b[1652] && s.b[1697]) && s.b[1698]) && s.b[1699]) && (!s.b[1700])) {s.store_powf(1680, 1679, (-p.p182));}
        if (((s.b[1652] && s.b[1697]) && s.b[1698]) && s.b[1699]) {s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(284, 1662, 1.0, A::mul(s.ad_value(1679), s.ad_value(1680)), 1.0, (p.p185 * 1.0 / ((1.0 - p.p182))));}
        if (((s.b[1652] && s.b[1697]) && s.b[1698]) && (!s.b[1699])) {s.store_scalar(284, 0.0);}
        s.b[1701] = (s.v[1664] > 0.0);s.store_scalar(1701, if s.b[1701] { 1.0 } else { 0.0 });
        if (((s.b[1652] && s.b[1697]) && s.b[1698]) && s.b[1701]) {s.store_sub_from_scalar_scaled_input(1679, 1.0, 1668, 1.0 / (p.p186));}
        s.b[1702] = (p.p183 == 0.5);s.store_scalar(1702, if s.b[1702] { 1.0 } else { 0.0 });
        if ((((s.b[1652] && s.b[1697]) && s.b[1698]) && s.b[1701]) && s.b[1702]) {s.store_div_from_scalar_sqrt_ad(1680, 1.0, s.ad_value(1679));}
        if ((((s.b[1652] && s.b[1697]) && s.b[1698]) && s.b[1701]) && (!s.b[1702])) {s.store_powf(1680, 1679, (-p.p183));}
        if (((s.b[1652] && s.b[1697]) && s.b[1698]) && s.b[1701]) {s.store_add_mixed_ia(284, 284, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1664), 1.0, A::mul(s.ad_value(1679), s.ad_value(1680)), (p.p186 * 1.0 / ((1.0 - p.p183)))));}
        s.b[1703] = (s.v[1666] > 0.0);s.store_scalar(1703, if s.b[1703] { 1.0 } else { 0.0 });
        if (((s.b[1652] && s.b[1697]) && s.b[1698]) && s.b[1703]) {s.store_sub_from_scalar_scaled_input(1679, 1.0, 1668, 1.0 / (p.p187));}
        s.b[1704] = (p.p184 == 0.5);s.store_scalar(1704, if s.b[1704] { 1.0 } else { 0.0 });
        if ((((s.b[1652] && s.b[1697]) && s.b[1698]) && s.b[1703]) && s.b[1704]) {s.store_div_from_scalar_sqrt_ad(1680, 1.0, s.ad_value(1679));}
        if ((((s.b[1652] && s.b[1697]) && s.b[1698]) && s.b[1703]) && (!s.b[1704])) {s.store_powf(1680, 1679, (-p.p184));}
        if (((s.b[1652] && s.b[1697]) && s.b[1698]) && s.b[1703]) {s.store_add_mixed_ia(284, 284, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1666), 1.0, A::mul(s.ad_value(1679), s.ad_value(1680)), (p.p187 * 1.0 / ((1.0 - p.p184)))));}
        if ((s.b[1652] && s.b[1697]) && (!s.b[1698])) {s.store_add_scaled_inputs3_indices(1654, 1662, 1.0, 1664, 1.0, 1666, 1.0);s.store_add_scaled_inputs3_indices(1655, 1662, (p.p182 * 1.0 / (p.p185)), 1664, (p.p183 * 1.0 / (p.p186)), 1666, (p.p184 * 1.0 / (p.p187)));s.store_mul_add_scaled_product_rhs_indices(284, 1668, 1654, 1.0, 1668, 1655, 0.5);}
        if (s.b[1652] && (!s.b[1697])) {s.store_scalar(1666, (p.p181 * p.p4));}
        s.b[1705] = (s.v[1668] < 0.0);s.store_scalar(1705, if s.b[1705] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_75(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1706] = (s.v[1662] > 0.0);s.store_scalar(1706, if s.b[1706] { 1.0 } else { 0.0 });
        if (((s.b[1652] && (!s.b[1697])) && s.b[1705]) && s.b[1706]) {s.store_sub_from_scalar_scaled_input(1679, 1.0, 1668, 1.0 / (p.p185));}
        s.b[1707] = (p.p182 == 0.5);s.store_scalar(1707, if s.b[1707] { 1.0 } else { 0.0 });
        if ((((s.b[1652] && (!s.b[1697])) && s.b[1705]) && s.b[1706]) && s.b[1707]) {s.store_div_from_scalar_sqrt_ad(1680, 1.0, s.ad_value(1679));}
        if ((((s.b[1652] && (!s.b[1697])) && s.b[1705]) && s.b[1706]) && (!s.b[1707])) {s.store_powf(1680, 1679, (-p.p182));}
        if (((s.b[1652] && (!s.b[1697])) && s.b[1705]) && s.b[1706]) {s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(284, 1662, 1.0, A::mul(s.ad_value(1679), s.ad_value(1680)), 1.0, (p.p185 * 1.0 / ((1.0 - p.p182))));}
        if (((s.b[1652] && (!s.b[1697])) && s.b[1705]) && (!s.b[1706])) {s.store_scalar(284, 0.0);}
        s.b[1708] = (s.v[1666] > 0.0);s.store_scalar(1708, if s.b[1708] { 1.0 } else { 0.0 });
        if (((s.b[1652] && (!s.b[1697])) && s.b[1705]) && s.b[1708]) {s.store_sub_from_scalar_scaled_input(1679, 1.0, 1668, 1.0 / (p.p187));}
        s.b[1709] = (p.p184 == 0.5);s.store_scalar(1709, if s.b[1709] { 1.0 } else { 0.0 });
        if ((((s.b[1652] && (!s.b[1697])) && s.b[1705]) && s.b[1708]) && s.b[1709]) {s.store_div_from_scalar_sqrt_ad(1680, 1.0, s.ad_value(1679));}
        if ((((s.b[1652] && (!s.b[1697])) && s.b[1705]) && s.b[1708]) && (!s.b[1709])) {s.store_powf(1680, 1679, (-p.p184));}
        if (((s.b[1652] && (!s.b[1697])) && s.b[1705]) && s.b[1708]) {s.store_add_mixed_ia(284, 284, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1666), 1.0, A::mul(s.ad_value(1679), s.ad_value(1680)), (p.p187 * 1.0 / ((1.0 - p.p184)))));}
        if ((s.b[1652] && (!s.b[1697])) && (!s.b[1705])) {s.store_add(1654, 1662, 1666);s.store_add_scaled_inputs(1655, 1662, (p.p182 * 1.0 / (p.p185)), 1666, (p.p184 * 1.0 / (p.p187)));s.store_mul_add_scaled_product_rhs_indices(284, 1668, 1654, 1.0, 1668, 1655, 0.5);}
        s.b[1710] = (s.v[1663] > 0.0);s.store_scalar(1710, if s.b[1710] { 1.0 } else { 0.0 });
        if (s.b[1652] && s.b[1710]) {s.store_scaled_mul(1657, 544, 1661, ((-1.6021918e-19) * p.p3));s.store_scale(1659, 1657, (-0.001));s.store_add_scaled_inputs3_indices(44, 1657, -1.0, 283, 1.0, 1659, -1.0);s.store_scaled_mul(45, 1657, 1659, (-4.0));}
        if (s.b[1652] && s.b[1710]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (s.b[1652] && s.b[1710]) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(283, 1657, -1.0, 44, (-0.5), 45, (-0.5));s.store_scale(283, 283, (-1.0));}
        s.b[1711] = (s.v[1662] > 0.0);s.store_scalar(1711, if s.b[1711] { 1.0 } else { 0.0 });
        if (s.b[1652] && s.b[1711]) {s.store_scaled_mul(1658, 544, 1661, ((-1.6021918e-19) * p.p2));s.store_scale(1660, 1658, (-0.001));s.store_add_scaled_inputs3_indices(44, 1658, -1.0, 284, 1.0, 1660, -1.0);s.store_scaled_mul(45, 1658, 1660, (-4.0));}
        if (s.b[1652] && s.b[1711]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (s.b[1652] && s.b[1711]) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(284, 1658, -1.0, 44, (-0.5), 45, (-0.5));s.store_scale(284, 284, (-1.0));}
        s.b[1744] = ((p.p32 != 0.0) && (s.v[145] == 0.0));s.store_scalar(1744, if s.b[1744] { 1.0 } else { 0.0 });
        if s.b[1744] {s.store_div_scaled_inputs2_indices(1727, 314, 1.0, 161, (-1.0), 441, 1.0);s.store_scaled_mul(1728, 251, 1727, 1e-5);}
        s.b[1745] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1745, if s.b[1745] { 1.0 } else { 0.0 });
        if (s.b[1744] && s.b[1745]) {s.store_scalar(1729, 1.0);}
        s.b[1746] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1746, if s.b[1746] { 1.0 } else { 0.0 });
        if ((s.b[1744] && (!s.b[1745])) && s.b[1746]) {s.copy_ad(1729, 1728);}
        if ((s.b[1744] && (!s.b[1745])) && (!s.b[1746])) {s.store_powf(1729, 1728, (p.p113 - 1.0));}
        if s.b[1744] {s.store_mul(1730, 1728, 1729);s.store_offset(1731, 1730, 1.0);s.store_powf(1732, 1731, (((-1.0) / p.p113) - 1.0));s.store_mul(1733, 1731, 1732);s.store_mul(293, 251, 1733);s.store_scaled_add(1735, 250, 293, 0.5);s.store_square(1734, 190);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_76(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1744] {s.store_div_scaled_product3_by_product_mixed_aiaai(292, A::mul3(s.ad_value(107), s.ad_value(323), s.ad_value(192)), 250, A::add_scaled_inputs3(A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(190), 3.0, 1.0), 1.0, s.ad_value(1734), 6.0), s.ad_value(293), s.ad_value(293)), 1.0, A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(190), 4.0, 3.0), 1.0, s.ad_value(1734), 3.0), s.ad_value(293), s.ad_value(250)), 1.0, A::mul3(A::add(A::scale_offset(s.ad_value(190), 3.0, 6.0), s.ad_value(1734)), s.ad_value(250), s.ad_value(250)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(441), A::offset(s.ad_value(190), 1.0), s.ad_value(1735), 15.0), 1735, 1.0);}
        if (!s.b[1744]) {s.store_scalar(292, 0.0);}
        s.b[1747] = ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (s.v[146] == 1.0)) && (s.v[145] == 0.0));s.store_scalar(1747, if s.b[1747] { 1.0 } else { 0.0 });
        if s.b[1747] {s.store_sqrt(298, 296);s.store_add(1736, 192, 298);s.store_square(1737, 294);s.store_square(1738, 296);s.store_scaled_mul(1739, 294, 296, 42.0);s.store_add_scaled_inputs3_indices(1739, 1739, 1.0, 1737, 4.0, 1738, 4.0);s.store_add_product3_rhs_mixed_iia(1739, 1739, 298, 192, A::add(s.ad_value(294), s.ad_value(296)), 20.0);s.store_square(1740, 1736);s.store_square(1732, 1740);s.store_div_scaled_value_by_product_indices(299, 1739, 1.0, 1732, 1736, 1.0);s.store_mul_ad_product_lhs_mixed_ai(300, A::div(s.ad_value(107), s.ad_value(441)), 250, 323);}
        s.store_add(199, 199, 265);s.b[1748] = (p.p43 == 1.0);s.store_scalar(1748, if s.b[1748] { 1.0 } else { 0.0 });
        if s.b[1748] {s.store_primal_add(271, 531, 532);}
        if (s.b[1748] && s.b[564]) {s.store_primal_offset(271, 271, (-(p.p168 * s.v[99])));}
        if s.b[1748] {s.store_mul_sub_scaled_inputs_rhs_indices(272, 271, 158, -1.0, 513, -1.0);s.store_scalar(276, ((3.453133e-11 / (3.141592653589793 / 2.0)) * (((1.0 + (p.p167 / s.v[88]))) as f64).ln()));s.store_primal_mul_scaled_offset_rhs(274, 276, p.p9, 518, s.v[101]);s.store_primal_mul_scaled_offset_rhs(275, 276, p.p9, 519, s.v[101]);s.store_mul_sub_rhs(277, 274, 158, 157);s.store_mul(278, 275, 158);s.store_mul_sub_scaled_inputs_rhs_indices(279, 276, 158, (p.p19 * p.p9), 513, (p.p19 * p.p9));s.store_add(268, 268, 277);s.store_add(267, 267, 278);s.store_add(272, 272, 279);}
        if ((!s.b[1748]) && s.b[564]) {s.store_scalar(271, ((-p.p168) * s.v[99]));s.store_mul_sub_scaled_inputs_rhs_indices(272, 271, 158, -1.0, 513, -1.0);}
        if ((!s.b[1748]) && (!s.b[564])) {s.store_scalar(271, 0.0);s.store_scalar(272, 0.0);}
        if (!s.b[1748]) {s.store_scalar(273, ((((3.453133e-11 / (3.141592653589793 / 2.0)) * s.v[101]) * p.p9) * (((1.0 + (p.p167 / s.v[88]))) as f64).ln()));s.copy_ad(274, 273);s.copy_ad(275, 273);s.store_mul_sub_rhs(277, 274, 158, 157);s.store_mul(278, 275, 158);s.store_add(268, 268, 277);s.store_add(267, 267, 278);}
        s.store_scale(9, 199, s.v[451]);
        if (s.v[85] != 0.0) {s.store_scalar(24, 0.0);s.store_scalar(23, 0.0);}
        s.b[1749] = (p.p43 == 1.0);s.store_scalar(1749, if s.b[1749] { 1.0 } else { 0.0 });
        if ((s.v[85] != 0.0) && s.b[1749]) {s.store_scalar(25, 0.0);s.copy_ad(556, 438);}
        if ((s.v[85] != 0.0) && (!s.b[1749])) {s.store_scalar(554, 0.0);}
        s.b[1750] = (p.p43 == 1.0);s.store_scalar(1750, if s.b[1750] { 1.0 } else { 0.0 });
        if ((s.v[85] == 0.0) && s.b[1750]) {s.store_sub_scaled_inputs(23, 196, (-s.v[451]), 197, s.v[451]);s.store_scale(24, 198, s.v[451]);s.store_scaled_sub(25, 197, 198, s.v[451]);}
        if ((s.v[85] == 0.0) && (!s.b[1750])) {s.store_add_scaled_inputs4_indices(23, 392, (-s.v[451]), 197, ((-1.0) * s.v[451]), 476, (-s.v[451]), 477, (-s.v[451]));s.store_scaled_add(24, 198, 477, s.v[451]);}
    }
}
