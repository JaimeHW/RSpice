#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_66(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t3: usize = 0;
        while {
            let t0: f64 = (2.0 * 20.0);let t1: f64 = (t0 + 1.0);let t2: f64 = if ((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (s.v[167] <= t1)) { 1.0 } else { 0.0 };
            t2 != 0.0
        } {
            t3 += 1;
            if t3 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t3, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) {s.store_scalar(1593, 0.0);s.store_mul_add_rhs(1569, 225, 1571, 1547);}
            s.b[1616] = (s.v[1569] < 5.0);s.store_scalar(1616, if s.b[1616] { 1.0 } else { 0.0 });
            if ((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && s.b[1616]) {s.store_mul3_ad_middle(1589, A::square(s.ad_value(1569)), 1569, A::offset(A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(1590, A::square(s.ad_value(1569)), A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul3_lhs(1591, 1597, 1589, 1589);s.store_mul_product3_indices(1592, 1590, 1597, 225, 1589, 2.0);s.store_mul_scale_offset_mixed_ia(1593, 1569, A::mul_offset_rhs(s.ad_value(1569), A::mul_offset_rhs(s.ad_value(1569), A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(1594, 1569, A::mul_offset_rhs(s.ad_value(1569), A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_offset_ad(1595, A::add(A::square(s.ad_value(1593)), s.ad_value(1591)), 1e-50);s.store_div_scaled_inputs2_mixed_aii(1596, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1594), s.ad_value(1593), 2.0), 1.0, 1592, 1.0, 1595, 2.0);}
            s.b[1617] = (s.v[1569] < 80.0);s.store_scalar(1617, if s.b[1617] { 1.0 } else { 0.0 });
            if (((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1616])) && s.b[1617]) {s.store_exp(243, 1569);s.store_mul_scale_offset_indices(1591, 1597, 243, 1.0, (-1.0));s.store_mul3_lhs(1592, 1597, 225, 243);}
            if (((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1616])) && (!s.b[1617])) {s.store_exp_mul(1598, 225, 1571);s.store_mul_sub_rhs(1591, 1582, 1598, 1588);s.store_mul3_lhs(1592, 1582, 225, 1598);}
            if ((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1616])) {s.store_sqrt_add_ad(1595, A::offset(s.ad_value(1569), (-1.0)), s.ad_value(1591));s.store_scale_ad(1596, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1592), 1.0, s.ad_value(1595), 1.0), 0.5);}
            if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) {s.store_add_scaled_inputs_product_indices(1599, 1557, 1.0, 1571, (-1.0), 1555, 1595, (-1.0));s.store_sub_from_scalar_scaled_mul(1600, (-1.0), 1555, 1596, 1.0);}
            s.b[1618] = (s.v[1544] == 1.0);s.store_scalar(1618, if s.b[1618] { 1.0 } else { 0.0 });
            if ((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && s.b[1618]) {s.store_scalar(167, ((2.0 * 20.0) + 1.0));}
            if ((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1618])) {s.store_div_scaled_inputs_indices(494, 1599, -1.0, 1600, 1.0);}
            if ((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1618])) {
                s.store_scaled_offset_ad(1601, {
                    if (1.0 >= ((s.v[1571]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1571))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1619] = (((s.v[494]) as f64).abs() > s.v[1601]);s.store_scalar(1619, if s.b[1619] { 1.0 } else { 0.0 });
            if (((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1618])) && s.b[1619]) {s.store_scale(494, 1601, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1618])) {s.store_add(1571, 1571, 494);}
            s.b[1620] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1599]) as f64).abs() <= 1e-8));s.store_scalar(1620, if s.b[1620] { 1.0 } else { 0.0 });
            if (((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1618])) && s.b[1620]) {s.store_scalar(1544, 1.0);}
            if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) {s.store_primal_offset(167, 167, 1.0);}
        }
        s.b[1622] = (s.v[1569] < 5.0);s.store_scalar(1622, if s.b[1622] { 1.0 } else { 0.0 });
        if ((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && s.b[1622]) {s.store_offset_square(1602, 1593, (10.0 * 2.220446049250313e-16));s.store_offset(1603, 1593, (10.0 * 2.220446049250313e-16));}
        if ((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1622])) {s.store_offset(1602, 1569, (-1.0));s.store_sqrt(1603, 1602);}
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) {s.store_mul(458, 1554, 1603);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_67(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) {s.store_div_from_scalar_add_ad(1522, 1.0, s.ad_value(1595), s.ad_value(1603));s.store_mul3_lhs(460, 1554, 1591, 1522);s.store_add(459, 458, 460);}
        if (((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) {s.store_sub(460, 459, 458);}
        if (((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) {
            if (p[43] == 1.0) {
                s.store_mul(1525, 287, 1534);
            } else {
                s.store_mul(1525, 108, 1534);
            }
        }
        s.b[1624] = (((s.v[1540] != 0.0) && (p[43] == 0.0)) || ((s.v[1538] != 0.0) && (p[43] == 1.0)));s.store_scalar(1624, if s.b[1624] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1624]) {s.store_mul(455, 1525, 459);s.store_mul(457, 1525, 458);}
        s.b[1625] = (((s.v[1541] != 0.0) && (p[43] == 0.0)) || ((s.v[1539] != 0.0) && (p[43] == 1.0)));s.store_scalar(1625, if s.b[1625] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1625]) {s.store_mul(454, 1525, 459);s.store_mul(456, 1525, 458);}
        if (((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) {s.store_scalar(1538, ((1.0 - 1.0) / 2.0));s.store_scalar(1539, ((1.0 + 1.0) / 2.0));}
        s.b[1626] = (p[43] == 1.0);s.store_scalar(1626, if s.b[1626] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1626]) {s.store_add_scaled_products_mixed_iiia(1548, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);s.store_add_scaled_products_indices(1549, 461, 157, 1.0, 462, 157, -1.0);s.store_add_scaled_products_mixed_iiia(1550, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_sub(1551, 1549, 1548);s.store_sub(1553, 1550, 1548);s.store_neg(1552, 1548);s.store_primal_add_scaled_products_indices(1540, 1538, 461, 1.0, 1539, 462, 1.0);s.store_primal_add_scaled_products_indices(1541, 1538, 462, 1.0, 1539, 461, 1.0);s.store_offset_ad(1546, A::add_scaled_products(s.ad_value(1540), s.ad_value(1552), 1.0, s.ad_value(1541), s.ad_value(1551), 1.0), (10.0 * 2.220446049250313e-16));}
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1626])) {s.store_primal_add_scaled_products_indices(1540, 1538, 461, 1.0, 1539, 462, 1.0);s.store_primal_add_scaled_products_indices(1541, 1538, 462, 1.0, 1539, 461, 1.0);}
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1626])) && (s.v[1538] != 0.0)) {s.store_add_scaled_products_mixed_iiia(1553, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);}
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1626])) && (s.v[1539] != 0.0)) {s.store_add_scaled_products_mixed_iiia(1553, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);}
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1626])) {s.store_scalar(1546, 0.0);}
        if (((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) {s.store_neg(1521, 1546);}
        s.b[1627] = (s.v[1521] > s.v[141]);s.store_scalar(1627, if s.b[1627] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1627]) {s.store_sub(1522, 1521, 141);s.store_sub(1523, 140, 141);s.store_div(44, 1522, 1523);s.store_square(45, 44);s.store_mul(46, 45, 44);s.store_square(47, 45);s.store_div_from_scalar_ad(1531, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));s.store_mul_scale_offset_indices(1531, 1523, 1531, -1.0, 1.0);s.store_add(1528, 141, 1531);}
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1627])) {s.copy_ad(1528, 1521);}
        if (((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) {s.store_offset_scaled(1547, 1528, -1.0, (-1e-12));s.store_scale(1555, 1554, s.v[1533]);s.store_square(1556, 1555);s.store_sub_from_scalar(1557, s.v[82], 1553);s.store_div_from_scalar(1521, s.v[69], 230);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1558, 2.0, 225, A::ln(s.ad_value(1521)));s.store_neg(1559, 1547);}
        s.b[1628] = (s.v[1557] < s.v[1559]);s.store_scalar(1628, if s.b[1628] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1628]) {s.store_div_scalar_by_product_indices(1522, 1.0, 225, 1554, 1.0);s.store_scale(1531, 1522, s.v[1532]);s.store_offset_scaled(1560, 1531, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(1561, 1560, 1560, 8.0, 0.0, 1560);s.store_sub(1562, 237, 1558);s.store_mul_add_rhs(1530, 225, 1557, 1547);s.store_sub_from_scalar_scaled_mul_mixed_ia(1563, (7.0 * 1.414213562373095), 1531, A::offset(s.ad_value(1530), (-2.0)), 9.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_68(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1628]) {s.store_square(1564, 1563);}
        s.b[1629] = (s.v[1561] < (s.v[1564] * 1e-8));s.store_scalar(1629, if s.b[1629] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1628]) && s.b[1629]) {s.store_add_scaled_inputs_product_mixed_aaia(1566, A::offset(s.ad_value(1563), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1561), 0.5, s.ad_value(1563), 1.0), 1.0, 1531, A::offset(s.ad_value(1530), (-2.0)), 9.0);}
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1628]) && (!s.b[1629])) {s.store_sqrt_add(1565, 1561, 1564);s.store_add_scaled_offset_product_rhs_mixed_aii(1566, A::offset(s.ad_value(1565), ((-7.0) * 1.414213562373095)), 1.0, 1531, 1530, (-2.0), 9.0);}
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1628]) {s.store_powf(1567, 1566, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(1568, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1531), 12.0)), 1.0, 1567, 2.0, 1567, 1567, 1.414213562373095);s.store_div(1569, 1568, 1567);s.store_add_scaled_product_indices(1570, 1547, (-1.0), 1569, 227, 1.0);s.store_add(1522, 1570, 1547);s.store_div(1523, 1522, 1562);s.store_sqrt_square_offset(1524, 1523, 1.0);s.store_sub_div_lhs_indices(1571, 1522, 1524, 1547);s.store_sub(1523, 1557, 1571);s.store_scale(459, 1523, s.v[1532]);s.copy_ad(458, 459);}
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) {s.store_scalar(1569, 3.0);s.store_sub_div_lhs_indices(1572, 1569, 225, 1547);s.store_exp_neg_input(1531, 1569);s.store_offset_div_scaled_inputs2_mixed_aia(1530, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1557), s.ad_value(1547))), (-1.0)), 4.0, 1531, 4.0, A::mul(s.ad_value(1556), s.ad_value(226)), 1.0, 1.0);}
        s.b[1630] = (s.v[1530] < (10.0 * 2.220446049250313e-16));s.store_scalar(1630, if s.b[1630] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1630]) {s.store_scalar(1530, (10.0 * 2.220446049250313e-16));}
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) {s.store_add_product3_rhs_mixed_iia(1572, 1557, 1556, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1530))), 1.0 / (2.0));s.store_mul_add_rhs(1569, 225, 1572, 1547);s.store_exp_neg_input(1531, 1569);s.store_offset_div_scaled_inputs2_mixed_aia(1530, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1557), s.ad_value(1547))), (-1.0)), 4.0, 1531, 4.0, A::mul(s.ad_value(1556), s.ad_value(226)), 1.0, 1.0);}
        s.b[1631] = (s.v[1530] < (10.0 * 2.220446049250313e-16));s.store_scalar(1631, if s.b[1631] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1631]) {s.store_scalar(1530, (10.0 * 2.220446049250313e-16));}
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) {s.store_add_product3_rhs_mixed_iia(1572, 1557, 1556, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1530))), 1.0 / (2.0));s.store_mul_add_rhs(1569, 225, 1572, 1547);}
        s.b[1632] = (s.v[1569] < 3.0);s.store_scalar(1632, if s.b[1632] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1632]) {s.store_scalar(1573, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));s.store_scalar(1574, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));s.store_offset_div_from_scalar_ad(1575, 1.0, A::mul(s.ad_value(225), s.ad_value(1555)), (1.0 / 1.414213562373095));s.store_div_scaled_inputs2_indices(1576, 1557, -1.0, 1547, -1.0, 1555, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_69(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1632]) {s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(1577, A::div_scaled_product(A::square(s.ad_value(1574)), s.ad_value(1574), 1.0, A::mul3_scaled_output(s.ad_value(1573), s.ad_value(1573), s.ad_value(1573), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1574), s.ad_value(1575), 1.0, s.ad_value(1573), s.ad_value(1573), 6.0), (-1.0), 1576, 1.0, 1573, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(1578, A::add_scaled_square_product(s.ad_value(1574), (-1.0), s.ad_value(1573), s.ad_value(1575), 3.0), 1.0, 1573, 1573, 9.0);s.store_sqrt_add_scaled_square_cube_product(1526, 1577, 1.0, 1578, 1.0);s.store_powf_ad(1579, A::sub(s.ad_value(1526), s.ad_value(1577)), 0.3333333333333333);s.store_neg_powf_add_input(1580, 1577, 1526, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(1530, 1579, 1.0, 1580, 1.0, 1574, 1.0, 1573, 3.0, -1.0);s.store_add_scaled_product_indices(1572, 1547, (-1.0), 1530, 227, 1.0);s.store_mul_add_rhs(1569, 225, 1572, 1547);}
        s.b[1633] = (p[41] > 0.0);s.store_scalar(1633, if s.b[1633] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1633]) {s.store_offset_add(1581, 1557, 1547, 0.1);s.store_offset_exp_ad(1588, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1547), -1.0), 1e-50);s.store_scale(1521, 230, 1.0 / (s.v[69]));s.store_square(1582, 1521);s.store_mul(1583, 1582, 1588);s.store_mul(1521, 226, 1556);s.store_mul(1584, 225, 1581);s.store_add_scaled_inputs_product_mixed_aaii(1585, A::ln(A::add_scaled_square_product(s.ad_value(1584), 1.0, s.ad_value(1583), s.ad_value(1521), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1582), s.ad_value(1521))), (-1.0), 225, 1547, 1.0);s.store_offset_sub(44, 1584, 1585, (-1.0));s.store_scale(45, 1584, 4.0);}
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1633]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1633]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1522, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1523, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1585, 1584, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub(1584, 1584, 1585);s.store_add_scaled_inputs(1584, 1584, 1.0, 225, 0.1);s.store_add_scaled_inputs_product_mixed_aaii(1586, A::ln(A::add_scaled_square_product(s.ad_value(1584), 1.0, s.ad_value(1583), s.ad_value(1521), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1582), s.ad_value(1521))), (-1.0), 225, 1547, 1.0);s.copy_ad(1587, 1569);s.store_offset_sub(44, 1586, 1587, (-(0.0008 * 75.0)));s.store_scale(45, 1586, (4.0 * (0.0008 * 75.0)));}
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1633]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1633]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1522, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1523, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1569, 1586, 1.0, 44, (-0.5), 45, (-0.5));}
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) {s.store_sub_div_lhs_indices(1571, 1569, 225, 1547);s.store_add_offset_lhs_mixed_ia(1522, 1569, (-1.0), A::exp_scaled_input(s.ad_value(1569), -1.0));}
        s.b[1634] = (s.v[1522] < (10.0 * 2.220446049250313e-16));s.store_scalar(1634, if s.b[1634] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1634]) {s.store_scalar(1522, (10.0 * 2.220446049250313e-16));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_70(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) {s.store_sqrt(1523, 1522);s.store_mul(458, 1554, 1523);s.store_scaled_sub(459, 1557, 1571, s.v[1532]);}
        s.b[1635] = (p[41] == 1.0);s.store_scalar(1635, if s.b[1635] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) {s.store_exp_ad(1588, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1547), -1.0));s.store_scale(1521, 230, 1.0 / (s.v[69]));s.store_square(1582, 1521);s.store_mul(1597, 1582, 1588);s.store_scalar(1544, 0.0);s.store_scalar(1591, 0.0);s.store_scalar(1595, 0.0);s.store_scalar(167, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_71(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t7: usize = 0;
        while {
            let t4: f64 = (2.0 * 20.0);let t5: f64 = (t4 + 1.0);let t6: f64 = if ((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (s.v[167] <= t5)) { 1.0 } else { 0.0 };
            t6 != 0.0
        } {
            t7 += 1;
            if t7 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t7, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) {s.store_scalar(1593, 0.0);s.store_mul_add_rhs(1569, 225, 1571, 1547);}
            s.b[1636] = (s.v[1569] < 5.0);s.store_scalar(1636, if s.b[1636] { 1.0 } else { 0.0 });
            if ((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && s.b[1636]) {s.store_mul3_ad_middle(1589, A::square(s.ad_value(1569)), 1569, A::offset(A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(1590, A::square(s.ad_value(1569)), A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul3_lhs(1591, 1597, 1589, 1589);s.store_mul_product3_indices(1592, 1590, 1597, 225, 1589, 2.0);s.store_mul_scale_offset_mixed_ia(1593, 1569, A::mul_offset_rhs(s.ad_value(1569), A::mul_offset_rhs(s.ad_value(1569), A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(1594, 1569, A::mul_offset_rhs(s.ad_value(1569), A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_offset_ad(1595, A::add(A::square(s.ad_value(1593)), s.ad_value(1591)), 1e-50);s.store_div_scaled_inputs2_mixed_aii(1596, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1594), s.ad_value(1593), 2.0), 1.0, 1592, 1.0, 1595, 2.0);}
            s.b[1637] = (s.v[1569] < 80.0);s.store_scalar(1637, if s.b[1637] { 1.0 } else { 0.0 });
            if (((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1636])) && s.b[1637]) {s.store_exp(243, 1569);s.store_mul_scale_offset_indices(1591, 1597, 243, 1.0, (-1.0));s.store_mul3_lhs(1592, 1597, 225, 243);}
            if (((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1636])) && (!s.b[1637])) {s.store_exp_mul(1598, 225, 1571);s.store_mul_sub_rhs(1591, 1582, 1598, 1588);s.store_mul3_lhs(1592, 1582, 225, 1598);}
            if ((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1636])) {s.store_sqrt_add_ad(1595, A::offset(s.ad_value(1569), (-1.0)), s.ad_value(1591));s.store_scale_ad(1596, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1592), 1.0, s.ad_value(1595), 1.0), 0.5);}
            if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) {s.store_add_scaled_inputs_product_indices(1599, 1557, 1.0, 1571, (-1.0), 1555, 1595, (-1.0));s.store_sub_from_scalar_scaled_mul(1600, (-1.0), 1555, 1596, 1.0);}
            s.b[1638] = (s.v[1544] == 1.0);s.store_scalar(1638, if s.b[1638] { 1.0 } else { 0.0 });
            if ((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && s.b[1638]) {s.store_scalar(167, ((2.0 * 20.0) + 1.0));}
            if ((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1638])) {s.store_div_scaled_inputs_indices(494, 1599, -1.0, 1600, 1.0);}
            if ((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1638])) {
                s.store_scaled_offset_ad(1601, {
                    if (1.0 >= ((s.v[1571]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1571))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1639] = (((s.v[494]) as f64).abs() > s.v[1601]);s.store_scalar(1639, if s.b[1639] { 1.0 } else { 0.0 });
            if (((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1638])) && s.b[1639]) {s.store_scale(494, 1601, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1638])) {s.store_add(1571, 1571, 494);}
            s.b[1640] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1599]) as f64).abs() <= 1e-8));s.store_scalar(1640, if s.b[1640] { 1.0 } else { 0.0 });
            if (((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1638])) && s.b[1640]) {s.store_scalar(1544, 1.0);}
            if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) {s.store_primal_offset(167, 167, 1.0);}
        }
        s.b[1642] = (s.v[1569] < 5.0);s.store_scalar(1642, if s.b[1642] { 1.0 } else { 0.0 });
        if ((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && s.b[1642]) {s.store_offset_square(1602, 1593, (10.0 * 2.220446049250313e-16));s.store_offset(1603, 1593, (10.0 * 2.220446049250313e-16));}
        if ((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1642])) {s.store_offset(1602, 1569, (-1.0));s.store_sqrt(1603, 1602);}
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) {s.store_mul(458, 1554, 1603);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_72(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) {s.store_div_from_scalar_add_ad(1522, 1.0, s.ad_value(1595), s.ad_value(1603));s.store_mul3_lhs(460, 1554, 1591, 1522);s.store_add(459, 458, 460);}
        if (((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) {s.store_sub(460, 459, 458);}
        if (((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) {
            if (p[43] == 1.0) {
                s.store_mul(1525, 287, 1534);
            } else {
                s.store_mul(1525, 108, 1534);
            }
        }
        s.b[1644] = (((s.v[1540] != 0.0) && (p[43] == 0.0)) || ((s.v[1538] != 0.0) && (p[43] == 1.0)));s.store_scalar(1644, if s.b[1644] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1644]) {s.store_mul(455, 1525, 459);s.store_mul(457, 1525, 458);}
        s.b[1645] = (((s.v[1541] != 0.0) && (p[43] == 0.0)) || ((s.v[1539] != 0.0) && (p[43] == 1.0)));s.store_scalar(1645, if s.b[1645] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1645]) {s.store_mul(454, 1525, 459);s.store_mul(456, 1525, 458);}
        if ((p[24] != 0.0) && s.b[1604]) {s.store_primal_add_scaled_inputs(266, 462, s.v[566], 461, s.v[565]);}
        if (((p[24] != 0.0) && s.b[1604]) && (s.v[266] != 0.0)) {s.store_add_scaled_inputs(269, 462, p[170], 461, p[169]);}
        s.b[1646] = (p[43] == 1.0);s.store_scalar(1646, if s.b[1646] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && s.b[1604]) && (s.v[266] != 0.0)) && s.b[1646]) {s.store_add_scaled_products_indices(1522, 462, 287, 1.0, 461, 288, 1.0);s.store_mul_scale_offset_indices(269, 269, 1522, -1.0, 0.0);}
        if ((((p[24] != 0.0) && s.b[1604]) && (s.v[266] != 0.0)) && (!s.b[1646])) {s.store_mul_scale_offset_indices(269, 269, 108, -1.0, 0.0);}
        if (((p[24] != 0.0) && s.b[1604]) && (s.v[266] != 0.0)) {s.store_add_scaled_product_right_sub(268, 268, 1.0, 269, 158, 157, -1.0);}
        if ((p[24] != 0.0) && s.b[1604]) {s.store_primal_add_scaled_inputs(266, 461, s.v[566], 462, s.v[565]);}
        if (((p[24] != 0.0) && s.b[1604]) && (s.v[266] != 0.0)) {s.store_add_scaled_inputs(270, 461, p[170], 462, p[169]);}
        s.b[1647] = (p[43] == 1.0);s.store_scalar(1647, if s.b[1647] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && s.b[1604]) && (s.v[266] != 0.0)) && s.b[1647]) {s.store_add_scaled_products_indices(1522, 461, 287, 1.0, 462, 288, 1.0);s.store_mul_scale_offset_indices(270, 270, 1522, -1.0, 0.0);}
        if ((((p[24] != 0.0) && s.b[1604]) && (s.v[266] != 0.0)) && (!s.b[1647])) {s.store_mul_scale_offset_indices(270, 270, 108, -1.0, 0.0);}
        if (((p[24] != 0.0) && s.b[1604]) && (s.v[266] != 0.0)) {s.store_add_scaled_product_indices(267, 267, 1.0, 270, 158, -1.0);}
        s.b[1648] = (((s.v[613] == 1.0) && (!s.b[565])) || ((s.v[613] != 1.0) && (!s.b[566])));s.store_scalar(1648, if s.b[1648] { 1.0 } else { 0.0 });s.b[1649] = (p[43] == 1.0);s.store_scalar(1649, if s.b[1649] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && (!s.b[1604])) && s.b[1648]) && s.b[1649]) {s.store_scale(269, 288, ((-s.v[1532]) * p[188]));}
        if ((((p[24] != 0.0) && (!s.b[1604])) && s.b[1648]) && (!s.b[1649])) {s.store_scale(269, 108, ((-s.v[1532]) * p[188]));}
        if (((p[24] != 0.0) && (!s.b[1604])) && (!s.b[1648])) {s.store_add_scaled_inputs(269, 462, p[170], 461, p[169]);}
        s.b[1650] = (p[43] == 1.0);s.store_scalar(1650, if s.b[1650] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && (!s.b[1604])) && (!s.b[1648])) && s.b[1650]) {s.store_add_scaled_products_indices(1522, 462, 287, 1.0, 461, 288, 1.0);s.store_mul_scale_offset_indices(269, 269, 1522, -1.0, 0.0);}
        if ((((p[24] != 0.0) && (!s.b[1604])) && (!s.b[1648])) && (!s.b[1650])) {s.store_mul_scale_offset_indices(269, 269, 108, -1.0, 0.0);}
        if ((p[24] != 0.0) && (!s.b[1604])) {s.store_mul_sub_scaled_inputs_rhs_indices(268, 269, 158, -1.0, 157, -1.0);}
        s.b[1651] = (((s.v[613] == 1.0) && (!s.b[566])) || ((s.v[613] != 1.0) && (!s.b[565])));s.store_scalar(1651, if s.b[1651] { 1.0 } else { 0.0 });s.b[1652] = (p[43] == 1.0);s.store_scalar(1652, if s.b[1652] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && (!s.b[1604])) && s.b[1651]) && s.b[1652]) {s.store_scale(270, 287, ((-s.v[1532]) * p[188]));}
        if ((((p[24] != 0.0) && (!s.b[1604])) && s.b[1651]) && (!s.b[1652])) {s.store_scale(270, 108, ((-s.v[1532]) * p[188]));}
        if (((p[24] != 0.0) && (!s.b[1604])) && (!s.b[1651])) {s.store_add_scaled_inputs(270, 461, p[170], 462, p[169]);}
        s.b[1653] = (p[43] == 1.0);s.store_scalar(1653, if s.b[1653] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && (!s.b[1604])) && (!s.b[1651])) && s.b[1653]) {s.store_add_scaled_products_indices(1522, 461, 287, 1.0, 462, 288, 1.0);s.store_mul_scale_offset_indices(270, 270, 1522, -1.0, 0.0);}
        if ((((p[24] != 0.0) && (!s.b[1604])) && (!s.b[1651])) && (!s.b[1653])) {s.store_mul_scale_offset_indices(270, 270, 108, -1.0, 0.0);}
        if ((p[24] != 0.0) && (!s.b[1604])) {s.store_mul_scale_offset_indices(267, 158, 270, -1.0, 0.0);}
        s.b[1654] = (p[43] == 1.0);s.store_scalar(1654, if s.b[1654] { 1.0 } else { 0.0 });
        if s.b[1654] {s.copy_ad(1670, 590);s.copy_ad(1671, 591);s.store_scale_ad(1672, A::exp_scaled_input(A::add_scaled_inputs(A::sub_from_scalar((s.v[87] * s.v[114]), A::mul(s.ad_value(237), s.ad_value(225))), 1.0, A::ln_scaled_input(s.ad_value(429), 1.0 / (s.v[81])), p[175]), 1.0 / (p[174])), p[173]);s.store_scale_ad(1673, A::exp_scaled_input(A::add_scaled_inputs(A::sub_from_scalar((s.v[87] * s.v[114]), A::mul(s.ad_value(237), s.ad_value(225))), 1.0, A::ln_scaled_input(s.ad_value(429), 1.0 / (s.v[81])), p[176]), 1.0 / (p[174])), p[173]);s.store_scaled_mul(1677, 286, 1672, p[237]);s.store_scaled_mul(1679, 286, 1673, p[237]);s.store_scaled_mul(1678, 285, 1672, p[237]);s.store_scaled_mul(1680, 285, 1673, p[237]);s.store_scale(1656, 429, 1.0 / (s.v[81]));s.store_square(1655, 1656);s.store_offset(1657, 1677, 1e-50);s.store_offset(1658, 1678, 1e-50);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_73(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1654] {s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1675, p[174], 225, A::ln_offset_div_scaled_inputs(s.ad_value(1655), p[177], s.ad_value(1657), 1.0, 1.0));s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1676, p[174], 225, A::ln_offset_div_scaled_inputs(s.ad_value(1655), p[177], s.ad_value(1658), 1.0, 1.0));s.store_scale(1674, 227, p[174]);}
        s.b[1683] = (s.v[1670] < s.v[1675]);s.store_scalar(1683, if s.b[1683] { 1.0 } else { 0.0 });
        if (s.b[1654] && s.b[1683]) {s.store_exp_div(1656, 1670, 1674);s.store_mul_scale_offset_indices(282, 1677, 1656, 1.0, (-1.0));}
        if (s.b[1654] && (!s.b[1683])) {s.store_exp_div(1656, 1675, 1674);s.store_add_scaled_offset_product_rhs_mixed_aii(282, A::mul3(A::div(s.ad_value(1677), s.ad_value(1674)), s.ad_value(1656), A::sub(s.ad_value(1670), s.ad_value(1675))), 1.0, 1677, 1656, (-1.0), 1.0);}
        if s.b[1654] {s.store_add_scaled_product_indices(282, 282, 1.0, 1670, 1679, p[178]);}
        s.b[1684] = (s.v[1671] < s.v[1676]);s.store_scalar(1684, if s.b[1684] { 1.0 } else { 0.0 });
        if (s.b[1654] && s.b[1684]) {s.store_exp_div(1656, 1671, 1674);s.store_mul_scale_offset_indices(281, 1678, 1656, 1.0, (-1.0));}
        if (s.b[1654] && (!s.b[1684])) {s.store_exp_div(1656, 1676, 1674);s.store_add_scaled_offset_product_rhs_mixed_aii(281, A::mul3(A::div(s.ad_value(1678), s.ad_value(1674)), s.ad_value(1656), A::sub(s.ad_value(1671), s.ad_value(1676))), 1.0, 1678, 1656, (-1.0), 1.0);}
        if s.b[1654] {s.store_add_scaled_product_indices(281, 281, 1.0, 1671, 1680, p[178]);s.store_add_scaled_inputs(282, 282, 1.0, 1670, s.v[142]);s.store_add_scaled_inputs(281, 281, 1.0, 1671, s.v[142]);s.store_scalar(1664, (p[179] * p[2]));s.store_scalar(1665, (p[179] * p[3]));s.store_scalar(1663, (p[237] - p[238]));}
        s.b[1685] = (s.v[1663] <= 0.0);s.store_scalar(1685, if s.b[1685] { 1.0 } else { 0.0 });
        if (s.b[1654] && s.b[1685]) {s.store_scalar(1664, 0.0);s.store_scalar(1665, 0.0);}
        s.b[1686] = (p[5] > s.v[287]);s.store_scalar(1686, if s.b[1686] { 1.0 } else { 0.0 });
        if (s.b[1654] && s.b[1686]) {s.store_primal_offset_scaled(1667, 287, (-p[180]), ((p[5]) * (p[180])));s.store_primal_scale(1669, 287, p[181]);}
        s.b[1687] = (s.v[1671] < 0.0);s.store_scalar(1687, if s.b[1687] { 1.0 } else { 0.0 });s.b[1688] = (s.v[1665] > 0.0);s.store_scalar(1688, if s.b[1688] { 1.0 } else { 0.0 });
        if (((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1688]) {s.store_sub_from_scalar_scaled_input(1681, 1.0, 1671, 1.0 / (p[185]));}
        s.b[1689] = (p[182] == 0.5);s.store_scalar(1689, if s.b[1689] { 1.0 } else { 0.0 });
        if ((((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1688]) && s.b[1689]) {s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));}
        if ((((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1688]) && (!s.b[1689])) {s.store_powf(1682, 1681, (-p[182]));}
        if (((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1688]) {s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(283, 1665, 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), 1.0, (p[185] * 1.0 / ((1.0 - p[182]))));}
        if (((s.b[1654] && s.b[1686]) && s.b[1687]) && (!s.b[1688])) {s.store_scalar(283, 0.0);}
        s.b[1690] = (s.v[1667] > 0.0);s.store_scalar(1690, if s.b[1690] { 1.0 } else { 0.0 });
        if (((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1690]) {s.store_sub_from_scalar_scaled_input(1681, 1.0, 1671, 1.0 / (p[186]));}
        s.b[1691] = (p[183] == 0.5);s.store_scalar(1691, if s.b[1691] { 1.0 } else { 0.0 });
        if ((((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1690]) && s.b[1691]) {s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));}
        if ((((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1690]) && (!s.b[1691])) {s.store_powf(1682, 1681, (-p[183]));}
        if (((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1690]) {s.store_add_mixed_ia(283, 283, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1667), 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), (p[186] * 1.0 / ((1.0 - p[183])))));}
        s.b[1692] = (s.v[1669] > 0.0);s.store_scalar(1692, if s.b[1692] { 1.0 } else { 0.0 });
        if (((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1692]) {s.store_sub_from_scalar_scaled_input(1681, 1.0, 1671, 1.0 / (p[187]));}
        s.b[1693] = (p[184] == 0.5);s.store_scalar(1693, if s.b[1693] { 1.0 } else { 0.0 });
        if ((((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1692]) && s.b[1693]) {s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));}
        if ((((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1692]) && (!s.b[1693])) {s.store_powf(1682, 1681, (-p[184]));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_74(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1654] && s.b[1686]) && s.b[1687]) && s.b[1692]) {s.store_add_mixed_ia(283, 283, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1669), 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), (p[187] * 1.0 / ((1.0 - p[184])))));}
        if ((s.b[1654] && s.b[1686]) && (!s.b[1687])) {s.store_add_scaled_inputs3_indices(1656, 1665, 1.0, 1667, 1.0, 1669, 1.0);s.store_add_scaled_inputs3_indices(1657, 1665, (p[182] * 1.0 / (p[185])), 1667, (p[183] * 1.0 / (p[186])), 1669, (p[184] * 1.0 / (p[187])));s.store_mul_add_scaled_product_rhs_indices(283, 1671, 1656, 1.0, 1671, 1657, 0.5);}
        if (s.b[1654] && (!s.b[1686])) {s.store_scalar(1669, (p[181] * p[5]));}
        s.b[1694] = (s.v[1671] < 0.0);s.store_scalar(1694, if s.b[1694] { 1.0 } else { 0.0 });s.b[1695] = (s.v[1665] > 0.0);s.store_scalar(1695, if s.b[1695] { 1.0 } else { 0.0 });
        if (((s.b[1654] && (!s.b[1686])) && s.b[1694]) && s.b[1695]) {s.store_sub_from_scalar_scaled_input(1681, 1.0, 1671, 1.0 / (p[185]));}
        s.b[1696] = (p[182] == 0.5);s.store_scalar(1696, if s.b[1696] { 1.0 } else { 0.0 });
        if ((((s.b[1654] && (!s.b[1686])) && s.b[1694]) && s.b[1695]) && s.b[1696]) {s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));}
        if ((((s.b[1654] && (!s.b[1686])) && s.b[1694]) && s.b[1695]) && (!s.b[1696])) {s.store_powf(1682, 1681, (-p[182]));}
        if (((s.b[1654] && (!s.b[1686])) && s.b[1694]) && s.b[1695]) {s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(283, 1665, 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), 1.0, (p[185] * 1.0 / ((1.0 - p[182]))));}
        if (((s.b[1654] && (!s.b[1686])) && s.b[1694]) && (!s.b[1695])) {s.store_scalar(283, 0.0);}
        s.b[1697] = (s.v[1669] > 0.0);s.store_scalar(1697, if s.b[1697] { 1.0 } else { 0.0 });
        if (((s.b[1654] && (!s.b[1686])) && s.b[1694]) && s.b[1697]) {s.store_sub_from_scalar_scaled_input(1681, 1.0, 1671, 1.0 / (p[187]));}
        s.b[1698] = (p[184] == 0.5);s.store_scalar(1698, if s.b[1698] { 1.0 } else { 0.0 });
        if ((((s.b[1654] && (!s.b[1686])) && s.b[1694]) && s.b[1697]) && s.b[1698]) {s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));}
        if ((((s.b[1654] && (!s.b[1686])) && s.b[1694]) && s.b[1697]) && (!s.b[1698])) {s.store_powf(1682, 1681, (-p[184]));}
        if (((s.b[1654] && (!s.b[1686])) && s.b[1694]) && s.b[1697]) {s.store_add_mixed_ia(283, 283, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1669), 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), (p[187] * 1.0 / ((1.0 - p[184])))));}
        if ((s.b[1654] && (!s.b[1686])) && (!s.b[1694])) {s.store_add(1656, 1665, 1669);s.store_add_scaled_inputs(1657, 1665, (p[182] * 1.0 / (p[185])), 1669, (p[184] * 1.0 / (p[187])));s.store_mul_add_scaled_product_rhs_indices(283, 1671, 1656, 1.0, 1671, 1657, 0.5);}
        s.b[1699] = (p[4] > s.v[288]);s.store_scalar(1699, if s.b[1699] { 1.0 } else { 0.0 });
        if (s.b[1654] && s.b[1699]) {s.store_primal_offset_scaled(1666, 288, (-p[180]), ((p[4]) * (p[180])));s.store_primal_scale(1668, 288, p[181]);}
        s.b[1700] = (s.v[1670] < 0.0);s.store_scalar(1700, if s.b[1700] { 1.0 } else { 0.0 });s.b[1701] = (s.v[1664] > 0.0);s.store_scalar(1701, if s.b[1701] { 1.0 } else { 0.0 });
        if (((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1701]) {s.store_sub_from_scalar_scaled_input(1681, 1.0, 1670, 1.0 / (p[185]));}
        s.b[1702] = (p[182] == 0.5);s.store_scalar(1702, if s.b[1702] { 1.0 } else { 0.0 });
        if ((((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1701]) && s.b[1702]) {s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));}
        if ((((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1701]) && (!s.b[1702])) {s.store_powf(1682, 1681, (-p[182]));}
        if (((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1701]) {s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(284, 1664, 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), 1.0, (p[185] * 1.0 / ((1.0 - p[182]))));}
        if (((s.b[1654] && s.b[1699]) && s.b[1700]) && (!s.b[1701])) {s.store_scalar(284, 0.0);}
        s.b[1703] = (s.v[1666] > 0.0);s.store_scalar(1703, if s.b[1703] { 1.0 } else { 0.0 });
        if (((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1703]) {s.store_sub_from_scalar_scaled_input(1681, 1.0, 1670, 1.0 / (p[186]));}
        s.b[1704] = (p[183] == 0.5);s.store_scalar(1704, if s.b[1704] { 1.0 } else { 0.0 });
        if ((((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1703]) && s.b[1704]) {s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));}
        if ((((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1703]) && (!s.b[1704])) {s.store_powf(1682, 1681, (-p[183]));}
        if (((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1703]) {s.store_add_mixed_ia(284, 284, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1666), 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), (p[186] * 1.0 / ((1.0 - p[183])))));}
        s.b[1705] = (s.v[1668] > 0.0);s.store_scalar(1705, if s.b[1705] { 1.0 } else { 0.0 });
        if (((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1705]) {s.store_sub_from_scalar_scaled_input(1681, 1.0, 1670, 1.0 / (p[187]));}
        s.b[1706] = (p[184] == 0.5);s.store_scalar(1706, if s.b[1706] { 1.0 } else { 0.0 });
        if ((((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1705]) && s.b[1706]) {s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));}
        if ((((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1705]) && (!s.b[1706])) {s.store_powf(1682, 1681, (-p[184]));}
        if (((s.b[1654] && s.b[1699]) && s.b[1700]) && s.b[1705]) {s.store_add_mixed_ia(284, 284, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1668), 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), (p[187] * 1.0 / ((1.0 - p[184])))));}
        if ((s.b[1654] && s.b[1699]) && (!s.b[1700])) {s.store_add_scaled_inputs3_indices(1656, 1664, 1.0, 1666, 1.0, 1668, 1.0);s.store_add_scaled_inputs3_indices(1657, 1664, (p[182] * 1.0 / (p[185])), 1666, (p[183] * 1.0 / (p[186])), 1668, (p[184] * 1.0 / (p[187])));s.store_mul_add_scaled_product_rhs_indices(284, 1670, 1656, 1.0, 1670, 1657, 0.5);}
        if (s.b[1654] && (!s.b[1699])) {s.store_scalar(1668, (p[181] * p[4]));}
        s.b[1707] = (s.v[1670] < 0.0);s.store_scalar(1707, if s.b[1707] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_75(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1708] = (s.v[1664] > 0.0);s.store_scalar(1708, if s.b[1708] { 1.0 } else { 0.0 });
        if (((s.b[1654] && (!s.b[1699])) && s.b[1707]) && s.b[1708]) {s.store_sub_from_scalar_scaled_input(1681, 1.0, 1670, 1.0 / (p[185]));}
        s.b[1709] = (p[182] == 0.5);s.store_scalar(1709, if s.b[1709] { 1.0 } else { 0.0 });
        if ((((s.b[1654] && (!s.b[1699])) && s.b[1707]) && s.b[1708]) && s.b[1709]) {s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));}
        if ((((s.b[1654] && (!s.b[1699])) && s.b[1707]) && s.b[1708]) && (!s.b[1709])) {s.store_powf(1682, 1681, (-p[182]));}
        if (((s.b[1654] && (!s.b[1699])) && s.b[1707]) && s.b[1708]) {s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(284, 1664, 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), 1.0, (p[185] * 1.0 / ((1.0 - p[182]))));}
        if (((s.b[1654] && (!s.b[1699])) && s.b[1707]) && (!s.b[1708])) {s.store_scalar(284, 0.0);}
        s.b[1710] = (s.v[1668] > 0.0);s.store_scalar(1710, if s.b[1710] { 1.0 } else { 0.0 });
        if (((s.b[1654] && (!s.b[1699])) && s.b[1707]) && s.b[1710]) {s.store_sub_from_scalar_scaled_input(1681, 1.0, 1670, 1.0 / (p[187]));}
        s.b[1711] = (p[184] == 0.5);s.store_scalar(1711, if s.b[1711] { 1.0 } else { 0.0 });
        if ((((s.b[1654] && (!s.b[1699])) && s.b[1707]) && s.b[1710]) && s.b[1711]) {s.store_div_from_scalar_sqrt_ad(1682, 1.0, s.ad_value(1681));}
        if ((((s.b[1654] && (!s.b[1699])) && s.b[1707]) && s.b[1710]) && (!s.b[1711])) {s.store_powf(1682, 1681, (-p[184]));}
        if (((s.b[1654] && (!s.b[1699])) && s.b[1707]) && s.b[1710]) {s.store_add_mixed_ia(284, 284, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1668), 1.0, A::mul(s.ad_value(1681), s.ad_value(1682)), (p[187] * 1.0 / ((1.0 - p[184])))));}
        if ((s.b[1654] && (!s.b[1699])) && (!s.b[1707])) {s.store_add(1656, 1664, 1668);s.store_add_scaled_inputs(1657, 1664, (p[182] * 1.0 / (p[185])), 1668, (p[184] * 1.0 / (p[187])));s.store_mul_add_scaled_product_rhs_indices(284, 1670, 1656, 1.0, 1670, 1657, 0.5);}
        s.b[1712] = (s.v[1665] > 0.0);s.store_scalar(1712, if s.b[1712] { 1.0 } else { 0.0 });
        if (s.b[1654] && s.b[1712]) {s.store_scaled_mul(1659, 544, 1663, ((-1.6021918e-19) * p[3]));s.store_scale(1661, 1659, (-0.001));s.store_add_scaled_inputs3_indices(44, 1659, -1.0, 283, 1.0, 1661, -1.0);s.store_scaled_mul(45, 1659, 1661, (-4.0));}
        if (s.b[1654] && s.b[1712]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (s.b[1654] && s.b[1712]) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(283, 1659, -1.0, 44, (-0.5), 45, (-0.5));s.store_scale(283, 283, (-1.0));}
        s.b[1713] = (s.v[1664] > 0.0);s.store_scalar(1713, if s.b[1713] { 1.0 } else { 0.0 });
        if (s.b[1654] && s.b[1713]) {s.store_scaled_mul(1660, 544, 1663, ((-1.6021918e-19) * p[2]));s.store_scale(1662, 1660, (-0.001));s.store_add_scaled_inputs3_indices(44, 1660, -1.0, 284, 1.0, 1662, -1.0);s.store_scaled_mul(45, 1660, 1662, (-4.0));}
        if (s.b[1654] && s.b[1713]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (s.b[1654] && s.b[1713]) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(284, 1660, -1.0, 44, (-0.5), 45, (-0.5));s.store_scale(284, 284, (-1.0));}
        s.b[1746] = ((p[32] != 0.0) && (s.v[145] == 0.0));s.store_scalar(1746, if s.b[1746] { 1.0 } else { 0.0 });
        if s.b[1746] {s.store_div_scaled_inputs2_indices(1729, 314, 1.0, 161, (-1.0), 441, 1.0);s.store_scaled_mul(1730, 251, 1729, 1e-5);}
        s.b[1747] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p[113]) && (p[113] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1747, if s.b[1747] { 1.0 } else { 0.0 });
        if (s.b[1746] && s.b[1747]) {s.store_scalar(1731, 1.0);}
        s.b[1748] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p[113]) && (p[113] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1748, if s.b[1748] { 1.0 } else { 0.0 });
        if ((s.b[1746] && (!s.b[1747])) && s.b[1748]) {s.copy_ad(1731, 1730);}
        if ((s.b[1746] && (!s.b[1747])) && (!s.b[1748])) {s.store_powf(1731, 1730, (p[113] - 1.0));}
        if s.b[1746] {s.store_mul(1732, 1730, 1731);s.store_offset(1733, 1732, 1.0);s.store_powf(1734, 1733, (((-1.0) / p[113]) - 1.0));s.store_mul(1735, 1733, 1734);s.store_mul(293, 251, 1735);s.store_scaled_add(1737, 250, 293, 0.5);s.store_square(1736, 190);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_76(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1746] {s.store_div_scaled_product3_by_product_mixed_aiaai(292, A::mul3(s.ad_value(107), s.ad_value(323), s.ad_value(192)), 250, A::add_scaled_inputs3(A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(190), 3.0, 1.0), 1.0, s.ad_value(1736), 6.0), s.ad_value(293), s.ad_value(293)), 1.0, A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(190), 4.0, 3.0), 1.0, s.ad_value(1736), 3.0), s.ad_value(293), s.ad_value(250)), 1.0, A::mul3(A::add(A::scale_offset(s.ad_value(190), 3.0, 6.0), s.ad_value(1736)), s.ad_value(250), s.ad_value(250)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(441), A::offset(s.ad_value(190), 1.0), s.ad_value(1737), 15.0), 1737, 1.0);}
        if (!s.b[1746]) {s.store_scalar(292, 0.0);}
        s.b[1749] = ((((p[30] != 0.0) && (p[32] != 0.0)) && (s.v[146] == 1.0)) && (s.v[145] == 0.0));s.store_scalar(1749, if s.b[1749] { 1.0 } else { 0.0 });
        if s.b[1749] {s.store_sqrt(298, 296);s.store_add(1738, 192, 298);s.store_square(1739, 294);s.store_square(1740, 296);s.store_scaled_mul(1741, 294, 296, 42.0);s.store_add_scaled_inputs3_indices(1741, 1741, 1.0, 1739, 4.0, 1740, 4.0);s.store_add_product3_rhs_mixed_iia(1741, 1741, 298, 192, A::add(s.ad_value(294), s.ad_value(296)), 20.0);s.store_square(1742, 1738);s.store_square(1734, 1742);s.store_div_scaled_value_by_product_indices(299, 1741, 1.0, 1734, 1738, 1.0);s.store_mul_ad_product_lhs_mixed_ai(300, A::div(s.ad_value(107), s.ad_value(441)), 250, 323);}
        s.store_add(199, 199, 265);s.b[1750] = (p[43] == 1.0);s.store_scalar(1750, if s.b[1750] { 1.0 } else { 0.0 });
        if s.b[1750] {s.store_primal_add(271, 531, 532);}
        if (s.b[1750] && s.b[564]) {s.store_primal_offset(271, 271, (-(p[168] * s.v[99])));}
        if s.b[1750] {s.store_mul_sub_scaled_inputs_rhs_indices(272, 271, 158, -1.0, 513, -1.0);s.store_scalar(276, ((3.453133e-11 / (3.141592653589793 / 2.0)) * (((1.0 + (p[167] / s.v[88]))) as f64).ln()));s.store_primal_mul_scaled_offset_rhs(274, 276, p[9], 518, s.v[101]);s.store_primal_mul_scaled_offset_rhs(275, 276, p[9], 519, s.v[101]);s.store_mul_sub_rhs(277, 274, 158, 157);s.store_mul(278, 275, 158);s.store_mul_sub_scaled_inputs_rhs_indices(279, 276, 158, (p[19] * p[9]), 513, (p[19] * p[9]));s.store_add(268, 268, 277);s.store_add(267, 267, 278);s.store_add(272, 272, 279);}
        if ((!s.b[1750]) && s.b[564]) {s.store_scalar(271, ((-p[168]) * s.v[99]));s.store_mul_sub_scaled_inputs_rhs_indices(272, 271, 158, -1.0, 513, -1.0);}
        if ((!s.b[1750]) && (!s.b[564])) {s.store_scalar(271, 0.0);s.store_scalar(272, 0.0);}
        if (!s.b[1750]) {s.store_scalar(273, ((((3.453133e-11 / (3.141592653589793 / 2.0)) * s.v[101]) * p[9]) * (((1.0 + (p[167] / s.v[88]))) as f64).ln()));s.copy_ad(274, 273);s.copy_ad(275, 273);s.store_mul_sub_rhs(277, 274, 158, 157);s.store_mul(278, 275, 158);s.store_add(268, 268, 277);s.store_add(267, 267, 278);}
        s.store_scale(9, 199, s.v[451]);
        if (s.v[85] != 0.0) {s.store_scalar(24, 0.0);s.store_scalar(23, 0.0);}
        s.b[1751] = (p[43] == 1.0);s.store_scalar(1751, if s.b[1751] { 1.0 } else { 0.0 });
        if ((s.v[85] != 0.0) && s.b[1751]) {s.store_scalar(25, 0.0);s.copy_ad(556, 438);}
        if ((s.v[85] != 0.0) && (!s.b[1751])) {s.store_scalar(554, 0.0);}
        s.b[1752] = (p[43] == 1.0);s.store_scalar(1752, if s.b[1752] { 1.0 } else { 0.0 });
        if ((s.v[85] == 0.0) && s.b[1752]) {s.store_sub_scaled_inputs(23, 196, (-s.v[451]), 197, s.v[451]);s.store_scale(24, 198, s.v[451]);s.store_scaled_sub(25, 197, 198, s.v[451]);}
        if ((s.v[85] == 0.0) && (!s.b[1752])) {s.store_add_scaled_inputs4_indices(23, 392, (-s.v[451]), 197, ((-1.0) * s.v[451]), 476, (-s.v[451]), 477, (-s.v[451]));s.store_scaled_add(24, 198, 477, s.v[451]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_77(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.v[85] == 0.0) && (!s.b[1752])) {s.store_add_scaled_inputs3_indices(25, 197, s.v[451], 198, ((-1.0) * s.v[451]), 476, s.v[451]);}
        s.b[1758] = (p[64] == 0.0);s.store_scalar(1758, if s.b[1758] { 1.0 } else { 0.0 });
        if s.b[1758] {s.store_scalar(280, 0.0);}
        if (!s.b[1758]) {s.store_add_scaled_inputs(1753, 315, s.v[97], 161, 1.0);}
        s.b[1759] = (s.v[1753] > s.v[314]);s.store_scalar(1759, if s.b[1759] { 1.0 } else { 0.0 });
        if ((!s.b[1758]) && s.b[1759]) {s.copy_ad(1753, 314);}
        if (!s.b[1758]) {s.store_add_scaled_inputs3_indices(1754, 157, s.v[317], 161, s.v[317], 1753, (1.0 - s.v[317]));s.store_sqrt_div_from_scalar_ad(1755, (2.0 * 1.034943e-10), s.ad_value(229));s.store_scale(1756, 1755, 1.3);s.store_scaled_mul(1757, 108, 1756, 1.034943e-10);s.store_mul_add_scaled_inputs4_indices_rhs(280, 1757, 161, 1.0 / (p[64]), 157, 1.0 / (p[64]), 1754, (-1.0 / (p[64])), 315, -1.0);}
        s.b[1760] = (p[65] != 0.0);s.store_scalar(1760, if s.b[1760] { 1.0 } else { 0.0 });
        if s.b[1760] {s.store_add_scaled_product_indices(280, 280, 1.0, 135, 513, 1.0);}
        s.b[1761] = (p[24] == 1.0);s.store_scalar(1761, if s.b[1761] { 1.0 } else { 0.0 });s.b[1762] = (p[43] == 1.0);s.store_scalar(1762, if s.b[1762] { 1.0 } else { 0.0 });
        if (s.b[1761] && s.b[1762]) {s.store_add_scaled_inputs4_indices(471, 463, -1.0, 464, (-1.0), 467, -1.0, 468, -1.0);s.store_add(472, 466, 470);s.store_add(473, 465, 469);s.store_add_mixed_ia(23, 23, A::add_scaled_inputs(A::sub(A::sub(A::add_scaled_inputs4(s.ad_value(268), 1.0, s.ad_value(267), 1.0, s.ad_value(272), 1.0, s.ad_value(280), -1.0), s.ad_value(455)), s.ad_value(454)), s.v[451], s.ad_value(471), s.v[451]));s.store_add_mixed_ia(24, 24, A::add_scaled_inputs4(s.ad_value(280), s.v[451], s.ad_value(268), ((-1.0) * s.v[451]), s.ad_value(456), s.v[451], s.ad_value(472), s.v[451]));s.store_add_scaled_inputs4_indices(25, 25, 1.0, 457, s.v[451], 267, ((-1.0) * s.v[451]), 473, s.v[451]);}
        if (s.b[1761] && (!s.b[1762])) {s.store_add_mixed_ia(23, 23, A::sub_scaled_inputs(A::sub(A::add_scaled_inputs4(s.ad_value(268), 1.0, s.ad_value(267), 1.0, s.ad_value(272), 1.0, s.ad_value(280), -1.0), s.ad_value(455)), s.v[451], s.ad_value(454), s.v[451]));s.store_add_scaled_inputs4_indices(24, 24, 1.0, 280, s.v[451], 268, ((-1.0) * s.v[451]), 456, s.v[451]);s.store_add_scaled_inputs3_indices(25, 25, 1.0, 457, s.v[451], 267, (-s.v[451]));}
        s.b[1763] = (p[43] == 1.0);s.store_scalar(1763, if s.b[1763] { 1.0 } else { 0.0 });
        if s.b[1763] {s.store_scale(36, 281, s.v[451]);s.store_scale(35, 282, s.v[451]);s.store_scale(560, 284, s.v[451]);s.store_scale(561, 283, s.v[451]);}
        if (!s.b[1763]) {s.store_scalar(36, 0.0);s.store_scalar(35, 0.0);s.store_scalar(560, 0.0);s.store_scalar(561, 0.0);}
        s.b[1764] = (p[25] != 1.0);s.store_scalar(1764, if s.b[1764] { 1.0 } else { 0.0 });
        if s.b[1764] {s.store_scalar(557, 0.0);}
        if (!s.b[1764]) {s.store_scale(557, 263, s.v[451]);}
        s.store_scale(598, 292, s.v[451]);s.store_scalar(27, A::ddx_projection(&s.ad_value(23), Some(6), None));s.store_scale(27, 27, p[50]);s.store_scalar(28, A::ddx_projection(&s.ad_value(23), Some(7), None));s.store_scale(28, 28, p[50]);
        if (s.v[613] > 0.0) {
            s.copy_ad(555, 28);
        } else {
            s.copy_ad(555, 27);
        }
        s.b[1773] = ((((p[30] != 0.0) && (p[32] != 0.0)) && (s.v[146] == 1.0)) && (s.v[145] == 0.0));s.store_scalar(1773, if s.b[1773] { 1.0 } else { 0.0 });
        if s.b[1773] {s.store_scaled_mul(1767, 323, 108, (1e-6 * s.v[98]));s.store_scale(1768, 555, 1.0 / (s.v[451]));s.store_div_scaled_product3_indices(1769, 227, 1768, 1768, (0.1185185185185185 * 1.6021918e-19), 300, 1.0);}
        s.b[1774] = ((s.v[297] > (10.0 * 2.220446049250313e-16)) && (s.v[157] > (10.0 * 2.220446049250313e-16)));s.store_scalar(1774, if s.b[1774] { 1.0 } else { 0.0 });
        if (s.b[1773] && s.b[1774]) {s.store_div(1770, 251, 250);s.store_div_scaled_inputs2_mixed_aii(1771, A::div(s.ad_value(251), s.ad_value(293)), 1.0, 1770, (-1.0), 157, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_78(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[1773] && s.b[1774]) {s.store_add_mixed_ia(1772, 1770, A::div_scaled_product(s.ad_value(1771), A::add(A::add_scaled_product(s.ad_value(294), 1.0, s.ad_value(192), s.ad_value(298), 1.0), s.ad_value(296)), 0.6666666666666667, A::add(s.ad_value(192), s.ad_value(298)), 1.0));}
        if (s.b[1773] && (!s.b[1774])) {s.store_div(1772, 251, 293);}
        if s.b[1773] {s.store_mul3_affine_lhs(558, 1769, 299, s.v[451], 0.0, 1772);}
        if s.b[1773] {
            if (((-s.v[1768]) > s.v[1767]) && (s.v[558] > 0.0)) {
            } else {
                s.store_scalar(558, 0.0);
            }
        }
        if (!s.b[1773]) {s.store_scalar(558, 0.0);}
        s.b[1775] = (p[259] == 1.0);s.store_scalar(1775, if s.b[1775] { 1.0 } else { 0.0 });
        if s.b[1775] {s.store_scalar(3, 1.0);}
        s.b[1795] = (s.v[3] == 1.0);s.store_scalar(1795, if s.b[1795] { 1.0 } else { 0.0 });
        if (s.b[1775] && s.b[1795]) {s.store_scalar(1779, p[266]);s.store_scalar(1780, p[268]);s.store_scalar(1781, p[273]);s.store_scalar(1785, p[258]);s.store_scaled_voltage(1783, ctx, nodes, Some(7), Some(2), p[50]);}
        if (s.b[1775] && (!s.b[1795])) {s.store_scalar(1779, p[265]);s.store_scalar(1780, p[267]);s.store_scalar(1781, p[272]);s.store_scalar(1785, p[257]);s.store_scaled_voltage(1783, ctx, nodes, Some(0), Some(6), p[50]);}
        if s.b[1775] {s.store_primal_scale(1779, 1779, 0.0001);s.store_primal_scale(1780, 1780, 0.01);s.store_scale(1784, 429, 1.0 / (s.v[81]));s.store_powf(328, 1784, p[269]);s.store_div(1787, 1779, 328);s.store_sub_ad(327, A::add_scaled_product(A::scale_offset(s.ad_value(1784), 0.4, 1.8), 1.0, s.ad_value(1784), s.ad_value(1784), 0.1), A::scale_offset(s.ad_value(1784), (-p[270]), p[270]));s.store_div(1788, 1780, 327);s.store_add_mixed_ia(1781, 1781, A::scaled_offset(s.ad_value(429), (-s.v[81]), p[274]));s.store_scalar(1776, (1.0 + (p[279] / ((s.v[100]) as f64).powf(p[280]))));s.store_scalar(1778, (1.0 + (p[277] / ((s.v[100]) as f64).powf(p[278]))));s.store_scalar(1777, (1.0 + (p[275] / ((s.v[109]) as f64).powf(p[276]))));s.store_mul(1787, 1787, 1776);s.store_offset_product3(1788, s.ad_value(1788), s.ad_value(1777), s.ad_value(1778), 1.0, 1e-50);s.store_div(1789, 1783, 1785);s.store_mul(1790, 1787, 1789);}
        s.b[1796] = (s.v[1783] >= 0.0);s.store_scalar(1796, if s.b[1796] { 1.0 } else { 0.0 });
        if (s.b[1775] && s.b[1796]) {s.store_div(328, 1790, 1788);}
        if (s.b[1775] && (!s.b[1796])) {s.store_div_scaled_inputs_indices(328, 1790, -1.0, 1788, 1.0);}
        s.b[1797] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1781]) && (s.v[1781] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1797, if s.b[1797] { 1.0 } else { 0.0 });
        if (s.b[1775] && s.b[1797]) {s.store_scalar(330, 1.0);}
        s.b[1798] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1781]) && (s.v[1781] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1798, if s.b[1798] { 1.0 } else { 0.0 });
        if ((s.b[1775] && (!s.b[1797])) && s.b[1798]) {s.copy_ad(330, 328);}
        if ((s.b[1775] && (!s.b[1797])) && (!s.b[1798])) {s.store_pow_offset_rhs(330, 328, 1781, (-1.0));}
        if s.b[1775] {s.store_mul(329, 328, 330);s.store_offset(331, 329, 1.0);}
        s.b[1799] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1781]) && (s.v[1781] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1799, if s.b[1799] { 1.0 } else { 0.0 });
        if (s.b[1775] && s.b[1799]) {s.store_div_from_scalar(332, 1.0, 331);}
        s.b[1800] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1781]) && (s.v[1781] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1800, if s.b[1800] { 1.0 } else { 0.0 });
        if ((s.b[1775] && (!s.b[1799])) && s.b[1800]) {s.store_div_from_scalar_sqrt_ad(332, 1.0, s.ad_value(331));}
        if ((s.b[1775] && (!s.b[1799])) && (!s.b[1800])) {s.store_pow_ad(333, s.ad_value(331), A::offset(A::div_from_scalar((-1.0), s.ad_value(1781)), (-1.0)));s.store_mul(332, 331, 333);}
        if s.b[1775] {s.store_div_from_scalar(328, 1.6021918e-19, 1785);}
        s.b[1803] = (p[260] == 1.0);s.store_scalar(1803, if s.b[1803] { 1.0 } else { 0.0 });
        if s.b[1803] {s.store_scalar(3, 2.0);}
        s.b[1823] = (s.v[3] == 1.0);s.store_scalar(1823, if s.b[1823] { 1.0 } else { 0.0 });
        if (s.b[1803] && s.b[1823]) {s.store_scalar(1807, p[266]);s.store_scalar(1808, p[268]);s.store_scalar(1809, p[273]);s.store_scalar(1813, p[258]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_79(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[1803] && s.b[1823]) {s.store_scaled_voltage(1811, ctx, nodes, Some(7), Some(2), p[50]);}
        if (s.b[1803] && (!s.b[1823])) {s.store_scalar(1807, p[265]);s.store_scalar(1808, p[267]);s.store_scalar(1809, p[272]);s.store_scalar(1813, p[257]);s.store_scaled_voltage(1811, ctx, nodes, Some(0), Some(6), p[50]);}
        if s.b[1803] {s.store_primal_scale(1807, 1807, 0.0001);s.store_primal_scale(1808, 1808, 0.01);s.store_scale(1812, 429, 1.0 / (s.v[81]));s.store_powf(328, 1812, p[269]);s.store_div(1815, 1807, 328);s.store_sub_ad(327, A::add_scaled_product(A::scale_offset(s.ad_value(1812), 0.4, 1.8), 1.0, s.ad_value(1812), s.ad_value(1812), 0.1), A::scale_offset(s.ad_value(1812), (-p[270]), p[270]));s.store_div(1816, 1808, 327);s.store_add_mixed_ia(1809, 1809, A::scaled_offset(s.ad_value(429), (-s.v[81]), p[274]));s.store_scalar(1804, (1.0 + (p[279] / ((s.v[100]) as f64).powf(p[280]))));s.store_scalar(1806, (1.0 + (p[277] / ((s.v[100]) as f64).powf(p[278]))));s.store_scalar(1805, (1.0 + (p[275] / ((s.v[109]) as f64).powf(p[276]))));s.store_mul(1815, 1815, 1804);s.store_offset_product3(1816, s.ad_value(1816), s.ad_value(1805), s.ad_value(1806), 1.0, 1e-50);s.store_div(1817, 1811, 1813);s.store_mul(1818, 1815, 1817);}
        s.b[1824] = (s.v[1811] >= 0.0);s.store_scalar(1824, if s.b[1824] { 1.0 } else { 0.0 });
        if (s.b[1803] && s.b[1824]) {s.store_div(328, 1818, 1816);}
        if (s.b[1803] && (!s.b[1824])) {s.store_div_scaled_inputs_indices(328, 1818, -1.0, 1816, 1.0);}
        s.b[1825] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1809]) && (s.v[1809] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1825, if s.b[1825] { 1.0 } else { 0.0 });
        if (s.b[1803] && s.b[1825]) {s.store_scalar(330, 1.0);}
        s.b[1826] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1809]) && (s.v[1809] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1826, if s.b[1826] { 1.0 } else { 0.0 });
        if ((s.b[1803] && (!s.b[1825])) && s.b[1826]) {s.copy_ad(330, 328);}
        if ((s.b[1803] && (!s.b[1825])) && (!s.b[1826])) {s.store_pow_offset_rhs(330, 328, 1809, (-1.0));}
        if s.b[1803] {s.store_mul(329, 328, 330);s.store_offset(331, 329, 1.0);}
        s.b[1827] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1809]) && (s.v[1809] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1827, if s.b[1827] { 1.0 } else { 0.0 });
        if (s.b[1803] && s.b[1827]) {s.store_div_from_scalar(332, 1.0, 331);}
        s.b[1828] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1809]) && (s.v[1809] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1828, if s.b[1828] { 1.0 } else { 0.0 });
        if ((s.b[1803] && (!s.b[1827])) && s.b[1828]) {s.store_div_from_scalar_sqrt_ad(332, 1.0, s.ad_value(331));}
        if ((s.b[1803] && (!s.b[1827])) && (!s.b[1828])) {s.store_pow_ad(333, s.ad_value(331), A::offset(A::div_from_scalar((-1.0), s.ad_value(1809)), (-1.0)));s.store_mul(332, 331, 333);}
        if s.b[1803] {s.store_div_from_scalar(328, 1.6021918e-19, 1813);}
        s.b[1831] = (p[43] == 1.0);s.store_scalar(1831, if s.b[1831] { 1.0 } else { 0.0 });
        if (s.b[1831] && (s.v[85] != 0.0)) {
            if (s.v[613] == 1.0) {
                s.copy_ad(438, 556);
            } else {
                s.store_sub_from_scalar(438, 1.0, 556);
            }
        }
        if (s.b[1831] && (s.v[85] != 0.0)) {s.store_add_scaled_product_indices(584, 473, 1.0, 580, 438, 1.0);s.store_add_mixed_ai(585, A::mul_sub_from_scalar_rhs(s.ad_value(580), 1.0, s.ad_value(438)), 473);s.store_add_scaled_inputs3_indices(586, 580, -1.0, 581, (-1.0), 471, 1.0);}
        if (s.b[1831] && (s.v[85] == 0.0)) {s.store_scalar(584, 0.0);s.store_scalar(585, 0.0);s.store_scalar(586, 0.0);s.store_scalar(581, 0.0);}
        if ((!s.b[1831]) && (s.v[85] != 0.0)) {s.store_add_scaled_inputs3_indices(586, 584, -1.0, 585, (-1.0), 581, -1.0);}
        if ((!s.b[1831]) && (s.v[85] == 0.0)) {s.store_scalar(584, 0.0);s.store_scalar(585, 0.0);s.store_scalar(586, 0.0);s.store_scalar(581, 0.0);}
        s.b[1836] = (s.v[613] == 1.0);s.store_scalar(1836, if s.b[1836] { 1.0 } else { 0.0 });
        if s.b[1836] {s.copy_ad(199, 9);s.copy_ad(263, 557);s.store_add(594, 23, 586);s.store_add(198, 24, 584);s.store_add_scaled_inputs3_indices(554, 23, (-1.0), 24, (-1.0), 25, (-1.0));s.store_add(196, 554, 581);}
        if (!s.b[1836]) {s.store_neg(199, 9);s.store_scalar(263, 0.0);s.store_add(594, 23, 586);s.store_add(198, 25, 585);s.store_add_scaled_inputs3_indices(554, 23, (-1.0), 24, (-1.0), 25, (-1.0));s.store_add(196, 554, 581);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_80(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1837] = (p[43] == 1.0);s.store_scalar(1837, if s.b[1837] { 1.0 } else { 0.0 });
        if s.b[1837] {s.copy_ad(282, 35);s.copy_ad(284, 560);s.copy_ad(281, 36);s.copy_ad(283, 561);}
        s.b[1838] = ((p[38] == 1.0) && (s.v[67] > 0.0));s.store_scalar(1838, if s.b[1838] { 1.0 } else { 0.0 });
        if s.b[1838] {s.copy_ad(563, 542);}
        if (!s.b[1838]) {s.store_scalar(563, 0.0);}
        s.copy_ad(9, 199);s.store_scalar(27, A::ddx_projection(&s.ad_value(594), Some(6), None));s.store_scale(27, 27, p[50]);s.store_scalar(28, A::ddx_projection(&s.ad_value(594), Some(7), None));s.store_scale(28, 28, p[50]);s.b[1840] = (p[43] == 1.0);s.store_scalar(1840, if s.b[1840] { 1.0 } else { 0.0 });
        if s.b[1840] {s.store_scale(35, 282, p[50]);s.store_scale(36, 281, p[50]);}
        s.store_scale(610, 429, (4.0 * 1.3806226e-23));s.copy_ad(438, 439);s.store_mul(615, 610, 598);
        if ((s.v[615] > 0.0) && (s.v[558] > 0.0)) {
            s.store_sqrt_div(616, 558, 615);
        } else {
            s.store_scalar(616, 0.0);
        }
        if (s.v[613] > 0.0) {
            s.store_mul_scale_offset_indices(617, 616, 438, -1.0, 1.0);
        } else {
            s.store_mul(617, 616, 438);
        }
        if (s.v[613] > 0.0) {
            s.store_mul(618, 616, 438);
        } else {
            s.store_mul_scale_offset_indices(618, 616, 438, -1.0, 1.0);
        }
        s.b[1848] = ((p[38] > 0.0) && (p[242] > 0.0));s.store_scalar(1848, if s.b[1848] { 1.0 } else { 0.0 });s.b[1849] = (p[43] == 1.0);s.store_scalar(1849, if s.b[1849] { 1.0 } else { 0.0 });s.b[1850] = ((p[37] != 0.0) || ((p[25] == 1.0) && (p[26] == 2.0)));s.store_scalar(1850, if s.b[1850] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);let nv7 = ctx.node_voltage(nodes[7]);let eq0_value: f64 = 0.0;
        stamper.stamp_potential_const_local(
            0,
            eq0_value,
        );
        let (eq1_e316,) = {
    if s.b[627] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq1_value: f64 = eq1_e316;
        stamper.stamp_potential_const_local(
            1,
            eq1_value,
        );let eq2_e319: f64 = (p[50] * s.v[199]);let eq2_value: f64 = eq2_e319;
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq2_value),
            &s.dn[199],
            &s.db[199],
            (multiplicity) * (p[50]),
        );
        let (eq3_e325, eq3_e325_d_n0, eq3_e325_d_n1, eq3_e325_d_n2, eq3_e325_d_n3, eq3_e325_d_n4, eq3_e325_d_n5, eq3_e325_d_n6, eq3_e325_d_n7, eq3_e325_d_n8, eq3_e325_d_n9, eq3_e325_d_n10, eq3_e325_d_n11, eq3_e325_d_n12, eq3_e325_d_n13, eq3_e325_d_n14, eq3_e325_d_n15, eq3_e325_d_n16, eq3_e325_d_n17, eq3_e325_d_n18, eq3_e325_d_b0, eq3_e325_d_b1, eq3_e325_d_b2, eq3_e325_d_b3, eq3_e325_d_b4, eq3_e325_d_b5, eq3_e325_d_b6, eq3_e325_d_b7, eq3_e325_d_b8, eq3_e325_d_b9, eq3_e325_d_b10, eq3_e325_d_b11, eq3_e325_d_b12, eq3_e325_d_b13, eq3_e325_d_b14,) = {
    if s.b[1846] {
        let eq3_e323: f64 = (p[50] * s.v[306]);
        (eq3_e323, (p[50] * s.dn[306][0]), (p[50] * s.dn[306][1]), (p[50] * s.dn[306][2]), (p[50] * s.dn[306][3]), (p[50] * s.dn[306][4]), (p[50] * s.dn[306][5]), (p[50] * s.dn[306][6]), (p[50] * s.dn[306][7]), (p[50] * s.dn[306][8]), (p[50] * s.dn[306][9]), (p[50] * s.dn[306][10]), (p[50] * s.dn[306][11]), (p[50] * s.dn[306][12]), (p[50] * s.dn[306][13]), (p[50] * s.dn[306][14]), (p[50] * s.dn[306][15]), (p[50] * s.dn[306][16]), (p[50] * s.dn[306][17]), (p[50] * s.dn[306][18]), (p[50] * s.db[306][0]), (p[50] * s.db[306][1]), (p[50] * s.db[306][2]), (p[50] * s.db[306][3]), (p[50] * s.db[306][4]), (p[50] * s.db[306][5]), (p[50] * s.db[306][6]), (p[50] * s.db[306][7]), (p[50] * s.db[306][8]), (p[50] * s.db[306][9]), (p[50] * s.db[306][10]), (p[50] * s.db[306][11]), (p[50] * s.db[306][12]), (p[50] * s.db[306][13]), (p[50] * s.db[306][14]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e325;let eq3_node_derivatives: [f64; 19] = [eq3_e325_d_n0, eq3_e325_d_n1, eq3_e325_d_n2, eq3_e325_d_n3, eq3_e325_d_n4, eq3_e325_d_n5, eq3_e325_d_n6, eq3_e325_d_n7, eq3_e325_d_n8, eq3_e325_d_n9, eq3_e325_d_n10, eq3_e325_d_n11, eq3_e325_d_n12, eq3_e325_d_n13, eq3_e325_d_n14, eq3_e325_d_n15, eq3_e325_d_n16, eq3_e325_d_n17, eq3_e325_d_n18];let eq3_branch_derivatives: [f64; 15] = [eq3_e325_d_b0, eq3_e325_d_b1, eq3_e325_d_b2, eq3_e325_d_b3, eq3_e325_d_b4, eq3_e325_d_b5, eq3_e325_d_b6, eq3_e325_d_b7, eq3_e325_d_b8, eq3_e325_d_b9, eq3_e325_d_b10, eq3_e325_d_b11, eq3_e325_d_b12, eq3_e325_d_b13, eq3_e325_d_b14];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let (eq4_e331, eq4_e331_d_n0, eq4_e331_d_n1, eq4_e331_d_n2, eq4_e331_d_n3, eq4_e331_d_n4, eq4_e331_d_n5, eq4_e331_d_n6, eq4_e331_d_n7, eq4_e331_d_n8, eq4_e331_d_n9, eq4_e331_d_n10, eq4_e331_d_n11, eq4_e331_d_n12, eq4_e331_d_n13, eq4_e331_d_n14, eq4_e331_d_n15, eq4_e331_d_n16, eq4_e331_d_n17, eq4_e331_d_n18, eq4_e331_d_b0, eq4_e331_d_b1, eq4_e331_d_b2, eq4_e331_d_b3, eq4_e331_d_b4, eq4_e331_d_b5, eq4_e331_d_b6, eq4_e331_d_b7, eq4_e331_d_b8, eq4_e331_d_b9, eq4_e331_d_b10, eq4_e331_d_b11, eq4_e331_d_b12, eq4_e331_d_b13, eq4_e331_d_b14,) = {
    if s.b[1846] {
        let eq4_e329: f64 = (p[50] * s.v[307]);
        (eq4_e329, (p[50] * s.dn[307][0]), (p[50] * s.dn[307][1]), (p[50] * s.dn[307][2]), (p[50] * s.dn[307][3]), (p[50] * s.dn[307][4]), (p[50] * s.dn[307][5]), (p[50] * s.dn[307][6]), (p[50] * s.dn[307][7]), (p[50] * s.dn[307][8]), (p[50] * s.dn[307][9]), (p[50] * s.dn[307][10]), (p[50] * s.dn[307][11]), (p[50] * s.dn[307][12]), (p[50] * s.dn[307][13]), (p[50] * s.dn[307][14]), (p[50] * s.dn[307][15]), (p[50] * s.dn[307][16]), (p[50] * s.dn[307][17]), (p[50] * s.dn[307][18]), (p[50] * s.db[307][0]), (p[50] * s.db[307][1]), (p[50] * s.db[307][2]), (p[50] * s.db[307][3]), (p[50] * s.db[307][4]), (p[50] * s.db[307][5]), (p[50] * s.db[307][6]), (p[50] * s.db[307][7]), (p[50] * s.db[307][8]), (p[50] * s.db[307][9]), (p[50] * s.db[307][10]), (p[50] * s.db[307][11]), (p[50] * s.db[307][12]), (p[50] * s.db[307][13]), (p[50] * s.db[307][14]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e331;let eq4_node_derivatives: [f64; 19] = [eq4_e331_d_n0, eq4_e331_d_n1, eq4_e331_d_n2, eq4_e331_d_n3, eq4_e331_d_n4, eq4_e331_d_n5, eq4_e331_d_n6, eq4_e331_d_n7, eq4_e331_d_n8, eq4_e331_d_n9, eq4_e331_d_n10, eq4_e331_d_n11, eq4_e331_d_n12, eq4_e331_d_n13, eq4_e331_d_n14, eq4_e331_d_n15, eq4_e331_d_n16, eq4_e331_d_n17, eq4_e331_d_n18];let eq4_branch_derivatives: [f64; 15] = [eq4_e331_d_b0, eq4_e331_d_b1, eq4_e331_d_b2, eq4_e331_d_b3, eq4_e331_d_b4, eq4_e331_d_b5, eq4_e331_d_b6, eq4_e331_d_b7, eq4_e331_d_b8, eq4_e331_d_b9, eq4_e331_d_b10, eq4_e331_d_b11, eq4_e331_d_b12, eq4_e331_d_b13, eq4_e331_d_b14];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e337, eq5_e337_d_n0, eq5_e337_d_n1, eq5_e337_d_n2, eq5_e337_d_n3, eq5_e337_d_n4, eq5_e337_d_n5, eq5_e337_d_n6, eq5_e337_d_n7, eq5_e337_d_n8, eq5_e337_d_n9, eq5_e337_d_n10, eq5_e337_d_n11, eq5_e337_d_n12, eq5_e337_d_n13, eq5_e337_d_n14, eq5_e337_d_n15, eq5_e337_d_n16, eq5_e337_d_n17, eq5_e337_d_n18, eq5_e337_d_b0, eq5_e337_d_b1, eq5_e337_d_b2, eq5_e337_d_b3, eq5_e337_d_b4, eq5_e337_d_b5, eq5_e337_d_b6, eq5_e337_d_b7, eq5_e337_d_b8, eq5_e337_d_b9, eq5_e337_d_b10, eq5_e337_d_b11, eq5_e337_d_b12, eq5_e337_d_b13, eq5_e337_d_b14,) = {
    if s.b[1846] {
        let eq5_e335: f64 = (p[50] * s.v[308]);
        (eq5_e335, (p[50] * s.dn[308][0]), (p[50] * s.dn[308][1]), (p[50] * s.dn[308][2]), (p[50] * s.dn[308][3]), (p[50] * s.dn[308][4]), (p[50] * s.dn[308][5]), (p[50] * s.dn[308][6]), (p[50] * s.dn[308][7]), (p[50] * s.dn[308][8]), (p[50] * s.dn[308][9]), (p[50] * s.dn[308][10]), (p[50] * s.dn[308][11]), (p[50] * s.dn[308][12]), (p[50] * s.dn[308][13]), (p[50] * s.dn[308][14]), (p[50] * s.dn[308][15]), (p[50] * s.dn[308][16]), (p[50] * s.dn[308][17]), (p[50] * s.dn[308][18]), (p[50] * s.db[308][0]), (p[50] * s.db[308][1]), (p[50] * s.db[308][2]), (p[50] * s.db[308][3]), (p[50] * s.db[308][4]), (p[50] * s.db[308][5]), (p[50] * s.db[308][6]), (p[50] * s.db[308][7]), (p[50] * s.db[308][8]), (p[50] * s.db[308][9]), (p[50] * s.db[308][10]), (p[50] * s.db[308][11]), (p[50] * s.db[308][12]), (p[50] * s.db[308][13]), (p[50] * s.db[308][14]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e337;let eq5_node_derivatives: [f64; 19] = [eq5_e337_d_n0, eq5_e337_d_n1, eq5_e337_d_n2, eq5_e337_d_n3, eq5_e337_d_n4, eq5_e337_d_n5, eq5_e337_d_n6, eq5_e337_d_n7, eq5_e337_d_n8, eq5_e337_d_n9, eq5_e337_d_n10, eq5_e337_d_n11, eq5_e337_d_n12, eq5_e337_d_n13, eq5_e337_d_n14, eq5_e337_d_n15, eq5_e337_d_n16, eq5_e337_d_n17, eq5_e337_d_n18];let eq5_branch_derivatives: [f64; 15] = [eq5_e337_d_b0, eq5_e337_d_b1, eq5_e337_d_b2, eq5_e337_d_b3, eq5_e337_d_b4, eq5_e337_d_b5, eq5_e337_d_b6, eq5_e337_d_b7, eq5_e337_d_b8, eq5_e337_d_b9, eq5_e337_d_b10, eq5_e337_d_b11, eq5_e337_d_b12, eq5_e337_d_b13, eq5_e337_d_b14];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(12),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let (eq6_e343, eq6_e343_d_n0, eq6_e343_d_n1, eq6_e343_d_n2, eq6_e343_d_n3, eq6_e343_d_n4, eq6_e343_d_n5, eq6_e343_d_n6, eq6_e343_d_n7, eq6_e343_d_n8, eq6_e343_d_n9, eq6_e343_d_n10, eq6_e343_d_n11, eq6_e343_d_n12, eq6_e343_d_n13, eq6_e343_d_n14, eq6_e343_d_n15, eq6_e343_d_n16, eq6_e343_d_n17, eq6_e343_d_n18, eq6_e343_d_b0, eq6_e343_d_b1, eq6_e343_d_b2, eq6_e343_d_b3, eq6_e343_d_b4, eq6_e343_d_b5, eq6_e343_d_b6, eq6_e343_d_b7, eq6_e343_d_b8, eq6_e343_d_b9, eq6_e343_d_b10, eq6_e343_d_b11, eq6_e343_d_b12, eq6_e343_d_b13, eq6_e343_d_b14,) = {
    if (p[259] != 0.0) {
        let eq6_e341: f64 = ((nv7 - nv2) / s.v[1]);let eq6_e341_d_n0: f64 = (-(((nv7 - nv2) * s.dn[1][0]) / (s.v[1] * s.v[1])));let eq6_e341_d_n1: f64 = (-(((nv7 - nv2) * s.dn[1][1]) / (s.v[1] * s.v[1])));let eq6_e341_d_n2: f64 = (((-s.v[1]) - ((nv7 - nv2) * s.dn[1][2])) / (s.v[1] * s.v[1]));let eq6_e341_d_n3: f64 = (-(((nv7 - nv2) * s.dn[1][3]) / (s.v[1] * s.v[1])));let eq6_e341_d_n4: f64 = (-(((nv7 - nv2) * s.dn[1][4]) / (s.v[1] * s.v[1])));let eq6_e341_d_n5: f64 = (-(((nv7 - nv2) * s.dn[1][5]) / (s.v[1] * s.v[1])));let eq6_e341_d_n6: f64 = (-(((nv7 - nv2) * s.dn[1][6]) / (s.v[1] * s.v[1])));let eq6_e341_d_n7: f64 = ((s.v[1] - ((nv7 - nv2) * s.dn[1][7])) / (s.v[1] * s.v[1]));let eq6_e341_d_n8: f64 = (-(((nv7 - nv2) * s.dn[1][8]) / (s.v[1] * s.v[1])));let eq6_e341_d_n9: f64 = (-(((nv7 - nv2) * s.dn[1][9]) / (s.v[1] * s.v[1])));let eq6_e341_d_n10: f64 = (-(((nv7 - nv2) * s.dn[1][10]) / (s.v[1] * s.v[1])));let eq6_e341_d_n11: f64 = (-(((nv7 - nv2) * s.dn[1][11]) / (s.v[1] * s.v[1])));let eq6_e341_d_n12: f64 = (-(((nv7 - nv2) * s.dn[1][12]) / (s.v[1] * s.v[1])));let eq6_e341_d_n13: f64 = (-(((nv7 - nv2) * s.dn[1][13]) / (s.v[1] * s.v[1])));let eq6_e341_d_n14: f64 = (-(((nv7 - nv2) * s.dn[1][14]) / (s.v[1] * s.v[1])));let eq6_e341_d_n15: f64 = (-(((nv7 - nv2) * s.dn[1][15]) / (s.v[1] * s.v[1])));let eq6_e341_d_n16: f64 = (-(((nv7 - nv2) * s.dn[1][16]) / (s.v[1] * s.v[1])));let eq6_e341_d_n17: f64 = (-(((nv7 - nv2) * s.dn[1][17]) / (s.v[1] * s.v[1])));let eq6_e341_d_n18: f64 = (-(((nv7 - nv2) * s.dn[1][18]) / (s.v[1] * s.v[1])));let eq6_e341_d_b0: f64 = (-(((nv7 - nv2) * s.db[1][0]) / (s.v[1] * s.v[1])));let eq6_e341_d_b1: f64 = (-(((nv7 - nv2) * s.db[1][1]) / (s.v[1] * s.v[1])));let eq6_e341_d_b2: f64 = (-(((nv7 - nv2) * s.db[1][2]) / (s.v[1] * s.v[1])));let eq6_e341_d_b3: f64 = (-(((nv7 - nv2) * s.db[1][3]) / (s.v[1] * s.v[1])));let eq6_e341_d_b4: f64 = (-(((nv7 - nv2) * s.db[1][4]) / (s.v[1] * s.v[1])));let eq6_e341_d_b5: f64 = (-(((nv7 - nv2) * s.db[1][5]) / (s.v[1] * s.v[1])));let eq6_e341_d_b6: f64 = (-(((nv7 - nv2) * s.db[1][6]) / (s.v[1] * s.v[1])));let eq6_e341_d_b7: f64 = (-(((nv7 - nv2) * s.db[1][7]) / (s.v[1] * s.v[1])));let eq6_e341_d_b8: f64 = (-(((nv7 - nv2) * s.db[1][8]) / (s.v[1] * s.v[1])));let eq6_e341_d_b9: f64 = (-(((nv7 - nv2) * s.db[1][9]) / (s.v[1] * s.v[1])));let eq6_e341_d_b10: f64 = (-(((nv7 - nv2) * s.db[1][10]) / (s.v[1] * s.v[1])));let eq6_e341_d_b11: f64 = (-(((nv7 - nv2) * s.db[1][11]) / (s.v[1] * s.v[1])));let eq6_e341_d_b12: f64 = (-(((nv7 - nv2) * s.db[1][12]) / (s.v[1] * s.v[1])));let eq6_e341_d_b13: f64 = (-(((nv7 - nv2) * s.db[1][13]) / (s.v[1] * s.v[1])));let eq6_e341_d_b14: f64 = (-(((nv7 - nv2) * s.db[1][14]) / (s.v[1] * s.v[1])));
        (eq6_e341, eq6_e341_d_n0, eq6_e341_d_n1, eq6_e341_d_n2, eq6_e341_d_n3, eq6_e341_d_n4, eq6_e341_d_n5, eq6_e341_d_n6, eq6_e341_d_n7, eq6_e341_d_n8, eq6_e341_d_n9, eq6_e341_d_n10, eq6_e341_d_n11, eq6_e341_d_n12, eq6_e341_d_n13, eq6_e341_d_n14, eq6_e341_d_n15, eq6_e341_d_n16, eq6_e341_d_n17, eq6_e341_d_n18, eq6_e341_d_b0, eq6_e341_d_b1, eq6_e341_d_b2, eq6_e341_d_b3, eq6_e341_d_b4, eq6_e341_d_b5, eq6_e341_d_b6, eq6_e341_d_b7, eq6_e341_d_b8, eq6_e341_d_b9, eq6_e341_d_b10, eq6_e341_d_b11, eq6_e341_d_b12, eq6_e341_d_b13, eq6_e341_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e343;let eq6_node_derivatives: [f64; 19] = [eq6_e343_d_n0, eq6_e343_d_n1, eq6_e343_d_n2, eq6_e343_d_n3, eq6_e343_d_n4, eq6_e343_d_n5, eq6_e343_d_n6, eq6_e343_d_n7, eq6_e343_d_n8, eq6_e343_d_n9, eq6_e343_d_n10, eq6_e343_d_n11, eq6_e343_d_n12, eq6_e343_d_n13, eq6_e343_d_n14, eq6_e343_d_n15, eq6_e343_d_n16, eq6_e343_d_n17, eq6_e343_d_n18];let eq6_branch_derivatives: [f64; 15] = [eq6_e343_d_b0, eq6_e343_d_b1, eq6_e343_d_b2, eq6_e343_d_b3, eq6_e343_d_b4, eq6_e343_d_b5, eq6_e343_d_b6, eq6_e343_d_b7, eq6_e343_d_b8, eq6_e343_d_b9, eq6_e343_d_b10, eq6_e343_d_b11, eq6_e343_d_b12, eq6_e343_d_b13, eq6_e343_d_b14];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(2),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let (eq7_e348,) = {
    if (p[259] == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq7_value: f64 = eq7_e348;
        stamper.stamp_potential_const_local(
            2,
            eq7_value,
        );
    }
}
