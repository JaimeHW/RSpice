#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_64(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1608]) {s.store_offset_ad(1548, A::add_scaled_products(s.ad_value(1542), s.ad_value(1554), 1.0, s.ad_value(1543), s.ad_value(1553), 1.0), (10.0 * 2.220446049250313e-16));}
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1608])) {s.store_primal_add_scaled_products_indices(1542, 1540, 461, 1.0, 1541, 462, 1.0);s.store_primal_add_scaled_products_indices(1543, 1540, 462, 1.0, 1541, 461, 1.0);}
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1608])) && (s.v[1540] != 0.0)) {s.store_add_scaled_products_mixed_iiia(1555, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);}
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1608])) && (s.v[1541] != 0.0)) {s.store_add_scaled_products_mixed_iiia(1555, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);}
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1608])) {s.store_scalar(1548, 0.0);}
        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {s.store_neg(1523, 1548);}
        s.b[1609] = (s.v[1523] > s.v[141]);s.store_scalar(1609, if s.b[1609] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1609]) {s.store_sub(1524, 1523, 141);s.store_sub(1525, 140, 141);s.store_div(44, 1524, 1525);s.store_square(45, 44);s.store_mul(46, 45, 44);s.store_square(47, 45);s.store_div_from_scalar_ad(1533, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));s.store_mul_scale_offset_indices(1533, 1525, 1533, -1.0, 1.0);s.store_add(1530, 141, 1533);}
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1609])) {s.copy_ad(1530, 1523);}
        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {s.store_offset_scaled(1549, 1530, -1.0, (-1e-12));s.store_scale(1557, 1556, s.v[1535]);s.store_square(1558, 1557);s.store_sub_from_scalar(1559, s.v[82], 1555);s.store_div_from_scalar(1523, s.v[69], 230);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1560, 2.0, 225, A::ln(s.ad_value(1523)));}
        let (t1,) = {
    if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {
        let t0: f64 = (-s.v[1549]);
        (t0,)
    } else {
        (s.v[1561],)
    }
};
        s.store_scalar(1561, t1);s.b[1610] = (s.v[1559] < s.v[1561]);s.store_scalar(1610, if s.b[1610] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1610]) {s.store_div_scalar_by_product_indices(1524, 1.0, 225, 1556, 1.0);s.store_scale(1533, 1524, s.v[1534]);s.store_offset_scaled(1562, 1533, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(1563, 1562, 1562, 8.0, 0.0, 1562);s.store_sub(1564, 237, 1560);s.store_mul_add_rhs(1532, 225, 1559, 1549);s.store_sub_from_scalar_scaled_mul_mixed_ia(1565, (7.0 * 1.414213562373095), 1533, A::offset(s.ad_value(1532), (-2.0)), 9.0);s.store_square(1566, 1565);}
        s.b[1611] = (s.v[1563] < (s.v[1566] * 1e-8));s.store_scalar(1611, if s.b[1611] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1610]) && s.b[1611]) {s.store_add_scaled_inputs_product_mixed_aaia(1568, A::offset(s.ad_value(1565), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1563), 0.5, s.ad_value(1565), 1.0), 1.0, 1533, A::offset(s.ad_value(1532), (-2.0)), 9.0);}
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1610]) && (!s.b[1611])) {s.store_sqrt_add(1567, 1563, 1566);s.store_add_scaled_offset_product_rhs_mixed_aii(1568, A::offset(s.ad_value(1567), ((-7.0) * 1.414213562373095)), 1.0, 1533, 1532, (-2.0), 9.0);}
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1610]) {s.store_powf(1569, 1568, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(1570, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1533), 12.0)), 1.0, 1569, 2.0, 1569, 1569, 1.414213562373095);s.store_div(1571, 1570, 1569);s.store_add_scaled_product_indices(1572, 1549, (-1.0), 1571, 227, 1.0);s.store_add(1524, 1572, 1549);s.store_div(1525, 1524, 1564);s.store_sqrt_square_offset(1526, 1525, 1.0);s.store_sub_div_lhs_indices(1573, 1524, 1526, 1549);s.store_sub(1525, 1559, 1573);s.store_scale(459, 1525, s.v[1534]);s.copy_ad(458, 459);}
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) {s.store_scalar(1571, 3.0);s.store_sub_div_lhs_indices(1574, 1571, 225, 1549);s.store_exp_neg_input(1533, 1571);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_65(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) {s.store_offset_div_scaled_inputs2_mixed_aia(1532, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1559), s.ad_value(1549))), (-1.0)), 4.0, 1533, 4.0, A::mul(s.ad_value(1558), s.ad_value(226)), 1.0, 1.0);}
        s.b[1612] = (s.v[1532] < (10.0 * 2.220446049250313e-16));s.store_scalar(1612, if s.b[1612] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1612]) {s.store_scalar(1532, (10.0 * 2.220446049250313e-16));}
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) {s.store_add_product3_rhs_mixed_iia(1574, 1559, 1558, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1532))), 1.0 / (2.0));s.store_mul_add_rhs(1571, 225, 1574, 1549);s.store_exp_neg_input(1533, 1571);s.store_offset_div_scaled_inputs2_mixed_aia(1532, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1559), s.ad_value(1549))), (-1.0)), 4.0, 1533, 4.0, A::mul(s.ad_value(1558), s.ad_value(226)), 1.0, 1.0);}
        s.b[1613] = (s.v[1532] < (10.0 * 2.220446049250313e-16));s.store_scalar(1613, if s.b[1613] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1613]) {s.store_scalar(1532, (10.0 * 2.220446049250313e-16));}
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) {s.store_add_product3_rhs_mixed_iia(1574, 1559, 1558, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1532))), 1.0 / (2.0));s.store_mul_add_rhs(1571, 225, 1574, 1549);}
        s.b[1614] = (s.v[1571] < 3.0);s.store_scalar(1614, if s.b[1614] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1614]) {s.store_scalar(1575, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));s.store_scalar(1576, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));s.store_offset_div_from_scalar_ad(1577, 1.0, A::mul(s.ad_value(225), s.ad_value(1557)), (1.0 / 1.414213562373095));s.store_div_scaled_inputs2_indices(1578, 1559, -1.0, 1549, -1.0, 1557, 1.0);s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(1579, A::div_scaled_product(A::square(s.ad_value(1576)), s.ad_value(1576), 1.0, A::mul3_scaled_output(s.ad_value(1575), s.ad_value(1575), s.ad_value(1575), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1576), s.ad_value(1577), 1.0, s.ad_value(1575), s.ad_value(1575), 6.0), (-1.0), 1578, 1.0, 1575, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(1580, A::add_scaled_square_product(s.ad_value(1576), (-1.0), s.ad_value(1575), s.ad_value(1577), 3.0), 1.0, 1575, 1575, 9.0);s.store_sqrt_add_scaled_square_cube_product(1528, 1579, 1.0, 1580, 1.0);s.store_powf_ad(1581, A::sub(s.ad_value(1528), s.ad_value(1579)), 0.3333333333333333);s.store_neg_powf_add_input(1582, 1579, 1528, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(1532, 1581, 1.0, 1582, 1.0, 1576, 1.0, 1575, 3.0, -1.0);s.store_add_scaled_product_indices(1574, 1549, (-1.0), 1532, 227, 1.0);s.store_mul_add_rhs(1571, 225, 1574, 1549);}
        s.b[1615] = (p.p41 > 0.0);s.store_scalar(1615, if s.b[1615] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1615]) {s.store_offset_add(1583, 1559, 1549, 0.1);s.store_offset_exp_ad(1590, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1549), -1.0), 1e-50);s.store_scale(1523, 230, 1.0 / (s.v[69]));s.store_square(1584, 1523);s.store_mul(1585, 1584, 1590);s.store_mul(1523, 226, 1558);s.store_mul(1586, 225, 1583);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_66(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1615]) {s.store_add_scaled_inputs_product_mixed_aaii(1587, A::ln(A::add_scaled_square_product(s.ad_value(1586), 1.0, s.ad_value(1585), s.ad_value(1523), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1584), s.ad_value(1523))), (-1.0), 225, 1549, 1.0);s.store_offset_sub(44, 1586, 1587, (-1.0));s.store_scale(45, 1586, 4.0);}
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1615]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1615]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1524, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1525, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1587, 1586, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub(1586, 1586, 1587);s.store_add_scaled_inputs(1586, 1586, 1.0, 225, 0.1);s.store_add_scaled_inputs_product_mixed_aaii(1588, A::ln(A::add_scaled_square_product(s.ad_value(1586), 1.0, s.ad_value(1585), s.ad_value(1523), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1584), s.ad_value(1523))), (-1.0), 225, 1549, 1.0);s.copy_ad(1589, 1571);s.store_offset_sub(44, 1588, 1589, (-(0.0008 * 75.0)));s.store_scale(45, 1588, (4.0 * (0.0008 * 75.0)));}
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1615]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1615]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1524, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1525, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1571, 1588, 1.0, 44, (-0.5), 45, (-0.5));}
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) {s.store_sub_div_lhs_indices(1573, 1571, 225, 1549);s.store_add_offset_lhs_mixed_ia(1524, 1571, (-1.0), A::exp_scaled_input(s.ad_value(1571), -1.0));}
        s.b[1616] = (s.v[1524] < (10.0 * 2.220446049250313e-16));s.store_scalar(1616, if s.b[1616] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1616]) {s.store_scalar(1524, (10.0 * 2.220446049250313e-16));}
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) {s.store_sqrt(1525, 1524);s.store_mul(458, 1556, 1525);s.store_scaled_sub(459, 1559, 1573, s.v[1534]);}
        s.b[1617] = (p.p41 == 1.0);s.store_scalar(1617, if s.b[1617] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) {s.store_exp_ad(1590, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1549), -1.0));s.store_scale(1523, 230, 1.0 / (s.v[69]));s.store_square(1584, 1523);s.store_mul(1599, 1584, 1590);}
        let (t2,) = {
    if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) {
        (0.0,)
    } else {
        (s.v[1546],)
    }
};
        s.store_scalar(1546, t2);
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) {s.store_scalar(1593, 0.0);s.store_scalar(1597, 0.0);s.store_scalar(167, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_67(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t7: usize = 0;
        while {
            let t4: f64 = (2.0 * 20.0);let t5: f64 = (t4 + 1.0);let t6: f64 = if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (s.v[167] <= t5)) { 1.0 } else { 0.0 };
            t6 != 0.0
        } {
            t7 += 1;assert!(t7 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) {s.store_scalar(1595, 0.0);s.store_mul_add_rhs(1571, 225, 1573, 1549);}
            s.b[1618] = (s.v[1571] < 5.0);s.store_scalar(1618, if s.b[1618] { 1.0 } else { 0.0 });
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && s.b[1618]) {s.store_mul3_ad_middle(1591, A::square(s.ad_value(1571)), 1571, A::offset(A::mul(s.ad_value(1571), A::scale_offset(s.ad_value(1571), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(1592, A::square(s.ad_value(1571)), A::mul(s.ad_value(1571), A::scale_offset(s.ad_value(1571), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul3_lhs(1593, 1599, 1591, 1591);s.store_mul_product3_indices(1594, 1592, 1599, 225, 1591, 2.0);s.store_mul_scale_offset_mixed_ia(1595, 1571, A::mul_offset_rhs(s.ad_value(1571), A::mul_offset_rhs(s.ad_value(1571), A::mul(s.ad_value(1571), A::scale_offset(s.ad_value(1571), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(1596, 1571, A::mul_offset_rhs(s.ad_value(1571), A::mul(s.ad_value(1571), A::scale_offset(s.ad_value(1571), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_offset_ad(1597, A::add(A::square(s.ad_value(1595)), s.ad_value(1593)), 1e-50);s.store_div_scaled_inputs2_mixed_aii(1598, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1596), s.ad_value(1595), 2.0), 1.0, 1594, 1.0, 1597, 2.0);}
            s.b[1619] = (s.v[1571] < 80.0);s.store_scalar(1619, if s.b[1619] { 1.0 } else { 0.0 });
            if (((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (!s.b[1618])) && s.b[1619]) {s.store_exp(243, 1571);s.store_mul_scale_offset_indices(1593, 1599, 243, 1.0, (-1.0));s.store_mul3_lhs(1594, 1599, 225, 243);}
            if (((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (!s.b[1618])) && (!s.b[1619])) {s.store_exp_mul(1600, 225, 1573);s.store_mul_sub_rhs(1593, 1584, 1600, 1590);s.store_mul3_lhs(1594, 1584, 225, 1600);}
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (!s.b[1618])) {s.store_sqrt_add_ad(1597, A::offset(s.ad_value(1571), (-1.0)), s.ad_value(1593));s.store_scale_ad(1598, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1594), 1.0, s.ad_value(1597), 1.0), 0.5);}
            if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) {s.store_add_scaled_inputs_product_indices(1601, 1559, 1.0, 1573, (-1.0), 1557, 1597, (-1.0));s.store_sub_from_scalar_scaled_mul(1602, (-1.0), 1557, 1598, 1.0);}
            s.b[1620] = (s.v[1546] == 1.0);s.store_scalar(1620, if s.b[1620] { 1.0 } else { 0.0 });
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && s.b[1620]) {s.store_scalar(167, ((2.0 * 20.0) + 1.0));}
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (!s.b[1620])) {s.store_div_scaled_inputs_indices(494, 1601, -1.0, 1602, 1.0);}
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (!s.b[1620])) {
                s.store_scaled_offset_ad(1603, {
                    if (1.0 >= ((s.v[1573]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1573))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1621] = (((s.v[494]) as f64).abs() > s.v[1603]);s.store_scalar(1621, if s.b[1621] { 1.0 } else { 0.0 });
            if (((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (!s.b[1620])) && s.b[1621]) {s.store_scale(494, 1603, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (!s.b[1620])) {s.store_add(1573, 1573, 494);}
            s.b[1622] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1601]) as f64).abs() <= 1e-8));s.store_scalar(1622, if s.b[1622] { 1.0 } else { 0.0 });
            let (t3,) = {
    if (((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (!s.b[1620])) && s.b[1622]) {
        (1.0,)
    } else {
        (s.v[1546],)
    }
};
            s.store_scalar(1546, t3);
            if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) {s.store_primal_offset(167, 167, 1.0);}
        }
        s.b[1624] = (s.v[1571] < 5.0);s.store_scalar(1624, if s.b[1624] { 1.0 } else { 0.0 });
        if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && s.b[1624]) {s.store_offset_square(1604, 1595, (10.0 * 2.220446049250313e-16));s.store_offset(1605, 1595, (10.0 * 2.220446049250313e-16));}
        if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (!s.b[1624])) {s.store_offset(1604, 1571, (-1.0));s.store_sqrt(1605, 1604);}
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) {s.store_mul(458, 1556, 1605);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_68(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) {s.store_div_from_scalar_add_ad(1524, 1.0, s.ad_value(1597), s.ad_value(1605));s.store_mul3_lhs(460, 1556, 1593, 1524);s.store_add(459, 458, 460);}
        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {s.store_sub(460, 459, 458);}
        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {
            if (p.p43 == 1.0) {
                s.store_mul(1527, 287, 1536);
            } else {
                s.store_mul(1527, 108, 1536);
            }
        }
        s.b[1626] = (((s.v[1542] != 0.0) && (p.p43 == 0.0)) || ((s.v[1540] != 0.0) && (p.p43 == 1.0)));s.store_scalar(1626, if s.b[1626] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1626]) {s.store_mul(455, 1527, 459);s.store_mul(457, 1527, 458);}
        s.b[1627] = (((s.v[1543] != 0.0) && (p.p43 == 0.0)) || ((s.v[1541] != 0.0) && (p.p43 == 1.0)));s.store_scalar(1627, if s.b[1627] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1627]) {s.store_mul(454, 1527, 459);s.store_mul(456, 1527, 458);}
        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {s.store_scalar(1540, ((1.0 - 1.0) / 2.0));s.store_scalar(1541, ((1.0 + 1.0) / 2.0));}
        s.b[1628] = (p.p43 == 1.0);s.store_scalar(1628, if s.b[1628] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1628]) {s.store_add_scaled_products_mixed_iiia(1550, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);s.store_add_scaled_products_indices(1551, 461, 157, 1.0, 462, 157, -1.0);s.store_add_scaled_products_mixed_iiia(1552, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_sub(1553, 1551, 1550);s.store_sub(1555, 1552, 1550);s.store_neg(1554, 1550);s.store_primal_add_scaled_products_indices(1542, 1540, 461, 1.0, 1541, 462, 1.0);s.store_primal_add_scaled_products_indices(1543, 1540, 462, 1.0, 1541, 461, 1.0);s.store_offset_ad(1548, A::add_scaled_products(s.ad_value(1542), s.ad_value(1554), 1.0, s.ad_value(1543), s.ad_value(1553), 1.0), (10.0 * 2.220446049250313e-16));}
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1628])) {s.store_primal_add_scaled_products_indices(1542, 1540, 461, 1.0, 1541, 462, 1.0);s.store_primal_add_scaled_products_indices(1543, 1540, 462, 1.0, 1541, 461, 1.0);}
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1628])) && (s.v[1540] != 0.0)) {s.store_add_scaled_products_mixed_iiia(1555, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);}
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1628])) && (s.v[1541] != 0.0)) {s.store_add_scaled_products_mixed_iiia(1555, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);}
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1628])) {s.store_scalar(1548, 0.0);}
        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {s.store_neg(1523, 1548);}
        s.b[1629] = (s.v[1523] > s.v[141]);s.store_scalar(1629, if s.b[1629] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1629]) {s.store_sub(1524, 1523, 141);s.store_sub(1525, 140, 141);s.store_div(44, 1524, 1525);s.store_square(45, 44);s.store_mul(46, 45, 44);s.store_square(47, 45);s.store_div_from_scalar_ad(1533, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));s.store_mul_scale_offset_indices(1533, 1525, 1533, -1.0, 1.0);s.store_add(1530, 141, 1533);}
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1629])) {s.copy_ad(1530, 1523);}
        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {s.store_offset_scaled(1549, 1530, -1.0, (-1e-12));s.store_scale(1557, 1556, s.v[1535]);s.store_square(1558, 1557);s.store_sub_from_scalar(1559, s.v[82], 1555);s.store_div_from_scalar(1523, s.v[69], 230);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1560, 2.0, 225, A::ln(s.ad_value(1523)));}
        let (t9,) = {
    if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {
        let t8: f64 = (-s.v[1549]);
        (t8,)
    } else {
        (s.v[1561],)
    }
};
        s.store_scalar(1561, t9);s.b[1630] = (s.v[1559] < s.v[1561]);s.store_scalar(1630, if s.b[1630] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1630]) {s.store_div_scalar_by_product_indices(1524, 1.0, 225, 1556, 1.0);s.store_scale(1533, 1524, s.v[1534]);s.store_offset_scaled(1562, 1533, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(1563, 1562, 1562, 8.0, 0.0, 1562);s.store_sub(1564, 237, 1560);s.store_mul_add_rhs(1532, 225, 1559, 1549);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_69(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1630]) {s.store_sub_from_scalar_scaled_mul_mixed_ia(1565, (7.0 * 1.414213562373095), 1533, A::offset(s.ad_value(1532), (-2.0)), 9.0);s.store_square(1566, 1565);}
        s.b[1631] = (s.v[1563] < (s.v[1566] * 1e-8));s.store_scalar(1631, if s.b[1631] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1630]) && s.b[1631]) {s.store_add_scaled_inputs_product_mixed_aaia(1568, A::offset(s.ad_value(1565), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1563), 0.5, s.ad_value(1565), 1.0), 1.0, 1533, A::offset(s.ad_value(1532), (-2.0)), 9.0);}
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1630]) && (!s.b[1631])) {s.store_sqrt_add(1567, 1563, 1566);s.store_add_scaled_offset_product_rhs_mixed_aii(1568, A::offset(s.ad_value(1567), ((-7.0) * 1.414213562373095)), 1.0, 1533, 1532, (-2.0), 9.0);}
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1630]) {s.store_powf(1569, 1568, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(1570, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1533), 12.0)), 1.0, 1569, 2.0, 1569, 1569, 1.414213562373095);s.store_div(1571, 1570, 1569);s.store_add_scaled_product_indices(1572, 1549, (-1.0), 1571, 227, 1.0);s.store_add(1524, 1572, 1549);s.store_div(1525, 1524, 1564);s.store_sqrt_square_offset(1526, 1525, 1.0);s.store_sub_div_lhs_indices(1573, 1524, 1526, 1549);s.store_sub(1525, 1559, 1573);s.store_scale(459, 1525, s.v[1534]);s.copy_ad(458, 459);}
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) {s.store_scalar(1571, 3.0);s.store_sub_div_lhs_indices(1574, 1571, 225, 1549);s.store_exp_neg_input(1533, 1571);s.store_offset_div_scaled_inputs2_mixed_aia(1532, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1559), s.ad_value(1549))), (-1.0)), 4.0, 1533, 4.0, A::mul(s.ad_value(1558), s.ad_value(226)), 1.0, 1.0);}
        s.b[1632] = (s.v[1532] < (10.0 * 2.220446049250313e-16));s.store_scalar(1632, if s.b[1632] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1632]) {s.store_scalar(1532, (10.0 * 2.220446049250313e-16));}
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) {s.store_add_product3_rhs_mixed_iia(1574, 1559, 1558, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1532))), 1.0 / (2.0));s.store_mul_add_rhs(1571, 225, 1574, 1549);s.store_exp_neg_input(1533, 1571);s.store_offset_div_scaled_inputs2_mixed_aia(1532, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1559), s.ad_value(1549))), (-1.0)), 4.0, 1533, 4.0, A::mul(s.ad_value(1558), s.ad_value(226)), 1.0, 1.0);}
        s.b[1633] = (s.v[1532] < (10.0 * 2.220446049250313e-16));s.store_scalar(1633, if s.b[1633] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1633]) {s.store_scalar(1532, (10.0 * 2.220446049250313e-16));}
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) {s.store_add_product3_rhs_mixed_iia(1574, 1559, 1558, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1532))), 1.0 / (2.0));s.store_mul_add_rhs(1571, 225, 1574, 1549);}
        s.b[1634] = (s.v[1571] < 3.0);s.store_scalar(1634, if s.b[1634] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1634]) {s.store_scalar(1575, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));s.store_scalar(1576, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));s.store_offset_div_from_scalar_ad(1577, 1.0, A::mul(s.ad_value(225), s.ad_value(1557)), (1.0 / 1.414213562373095));s.store_div_scaled_inputs2_indices(1578, 1559, -1.0, 1549, -1.0, 1557, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_70(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1634]) {s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(1579, A::div_scaled_product(A::square(s.ad_value(1576)), s.ad_value(1576), 1.0, A::mul3_scaled_output(s.ad_value(1575), s.ad_value(1575), s.ad_value(1575), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1576), s.ad_value(1577), 1.0, s.ad_value(1575), s.ad_value(1575), 6.0), (-1.0), 1578, 1.0, 1575, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(1580, A::add_scaled_square_product(s.ad_value(1576), (-1.0), s.ad_value(1575), s.ad_value(1577), 3.0), 1.0, 1575, 1575, 9.0);s.store_sqrt_add_scaled_square_cube_product(1528, 1579, 1.0, 1580, 1.0);s.store_powf_ad(1581, A::sub(s.ad_value(1528), s.ad_value(1579)), 0.3333333333333333);s.store_neg_powf_add_input(1582, 1579, 1528, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(1532, 1581, 1.0, 1582, 1.0, 1576, 1.0, 1575, 3.0, -1.0);s.store_add_scaled_product_indices(1574, 1549, (-1.0), 1532, 227, 1.0);s.store_mul_add_rhs(1571, 225, 1574, 1549);}
        s.b[1635] = (p.p41 > 0.0);s.store_scalar(1635, if s.b[1635] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1635]) {s.store_offset_add(1583, 1559, 1549, 0.1);s.store_offset_exp_ad(1590, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1549), -1.0), 1e-50);s.store_scale(1523, 230, 1.0 / (s.v[69]));s.store_square(1584, 1523);s.store_mul(1585, 1584, 1590);s.store_mul(1523, 226, 1558);s.store_mul(1586, 225, 1583);s.store_add_scaled_inputs_product_mixed_aaii(1587, A::ln(A::add_scaled_square_product(s.ad_value(1586), 1.0, s.ad_value(1585), s.ad_value(1523), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1584), s.ad_value(1523))), (-1.0), 225, 1549, 1.0);s.store_offset_sub(44, 1586, 1587, (-1.0));s.store_scale(45, 1586, 4.0);}
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1635]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1635]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1524, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1525, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1587, 1586, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub(1586, 1586, 1587);s.store_add_scaled_inputs(1586, 1586, 1.0, 225, 0.1);s.store_add_scaled_inputs_product_mixed_aaii(1588, A::ln(A::add_scaled_square_product(s.ad_value(1586), 1.0, s.ad_value(1585), s.ad_value(1523), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1584), s.ad_value(1523))), (-1.0), 225, 1549, 1.0);s.copy_ad(1589, 1571);s.store_offset_sub(44, 1588, 1589, (-(0.0008 * 75.0)));s.store_scale(45, 1588, (4.0 * (0.0008 * 75.0)));}
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1635]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1635]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1524, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1525, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1571, 1588, 1.0, 44, (-0.5), 45, (-0.5));}
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) {s.store_sub_div_lhs_indices(1573, 1571, 225, 1549);s.store_add_offset_lhs_mixed_ia(1524, 1571, (-1.0), A::exp_scaled_input(s.ad_value(1571), -1.0));}
        s.b[1636] = (s.v[1524] < (10.0 * 2.220446049250313e-16));s.store_scalar(1636, if s.b[1636] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1636]) {s.store_scalar(1524, (10.0 * 2.220446049250313e-16));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_71(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) {s.store_sqrt(1525, 1524);s.store_mul(458, 1556, 1525);s.store_scaled_sub(459, 1559, 1573, s.v[1534]);}
        s.b[1637] = (p.p41 == 1.0);s.store_scalar(1637, if s.b[1637] { 1.0 } else { 0.0 });
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) {s.store_exp_ad(1590, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1549), -1.0));s.store_scale(1523, 230, 1.0 / (s.v[69]));s.store_square(1584, 1523);s.store_mul(1599, 1584, 1590);}
        let (ta,) = {
    if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) {
        (0.0,)
    } else {
        (s.v[1546],)
    }
};
        s.store_scalar(1546, ta);
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) {s.store_scalar(1593, 0.0);s.store_scalar(1597, 0.0);s.store_scalar(167, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_72(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut tf: usize = 0;
        while {
            let tc: f64 = (2.0 * 20.0);let td: f64 = (tc + 1.0);let te: f64 = if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (s.v[167] <= td)) { 1.0 } else { 0.0 };
            te != 0.0
        } {
            tf += 1;assert!(tf <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) {s.store_scalar(1595, 0.0);s.store_mul_add_rhs(1571, 225, 1573, 1549);}
            s.b[1638] = (s.v[1571] < 5.0);s.store_scalar(1638, if s.b[1638] { 1.0 } else { 0.0 });
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && s.b[1638]) {s.store_mul3_ad_middle(1591, A::square(s.ad_value(1571)), 1571, A::offset(A::mul(s.ad_value(1571), A::scale_offset(s.ad_value(1571), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(1592, A::square(s.ad_value(1571)), A::mul(s.ad_value(1571), A::scale_offset(s.ad_value(1571), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul3_lhs(1593, 1599, 1591, 1591);s.store_mul_product3_indices(1594, 1592, 1599, 225, 1591, 2.0);s.store_mul_scale_offset_mixed_ia(1595, 1571, A::mul_offset_rhs(s.ad_value(1571), A::mul_offset_rhs(s.ad_value(1571), A::mul(s.ad_value(1571), A::scale_offset(s.ad_value(1571), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(1596, 1571, A::mul_offset_rhs(s.ad_value(1571), A::mul(s.ad_value(1571), A::scale_offset(s.ad_value(1571), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_offset_ad(1597, A::add(A::square(s.ad_value(1595)), s.ad_value(1593)), 1e-50);s.store_div_scaled_inputs2_mixed_aii(1598, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1596), s.ad_value(1595), 2.0), 1.0, 1594, 1.0, 1597, 2.0);}
            s.b[1639] = (s.v[1571] < 80.0);s.store_scalar(1639, if s.b[1639] { 1.0 } else { 0.0 });
            if (((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (!s.b[1638])) && s.b[1639]) {s.store_exp(243, 1571);s.store_mul_scale_offset_indices(1593, 1599, 243, 1.0, (-1.0));s.store_mul3_lhs(1594, 1599, 225, 243);}
            if (((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (!s.b[1638])) && (!s.b[1639])) {s.store_exp_mul(1600, 225, 1573);s.store_mul_sub_rhs(1593, 1584, 1600, 1590);s.store_mul3_lhs(1594, 1584, 225, 1600);}
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (!s.b[1638])) {s.store_sqrt_add_ad(1597, A::offset(s.ad_value(1571), (-1.0)), s.ad_value(1593));s.store_scale_ad(1598, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1594), 1.0, s.ad_value(1597), 1.0), 0.5);}
            if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) {s.store_add_scaled_inputs_product_indices(1601, 1559, 1.0, 1573, (-1.0), 1557, 1597, (-1.0));s.store_sub_from_scalar_scaled_mul(1602, (-1.0), 1557, 1598, 1.0);}
            s.b[1640] = (s.v[1546] == 1.0);s.store_scalar(1640, if s.b[1640] { 1.0 } else { 0.0 });
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && s.b[1640]) {s.store_scalar(167, ((2.0 * 20.0) + 1.0));}
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (!s.b[1640])) {s.store_div_scaled_inputs_indices(494, 1601, -1.0, 1602, 1.0);}
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (!s.b[1640])) {
                s.store_scaled_offset_ad(1603, {
                    if (1.0 >= ((s.v[1573]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1573))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1641] = (((s.v[494]) as f64).abs() > s.v[1603]);s.store_scalar(1641, if s.b[1641] { 1.0 } else { 0.0 });
            if (((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (!s.b[1640])) && s.b[1641]) {s.store_scale(494, 1603, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (!s.b[1640])) {s.store_add(1573, 1573, 494);}
            s.b[1642] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1601]) as f64).abs() <= 1e-8));s.store_scalar(1642, if s.b[1642] { 1.0 } else { 0.0 });
            let (tb,) = {
    if (((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (!s.b[1640])) && s.b[1642]) {
        (1.0,)
    } else {
        (s.v[1546],)
    }
};
            s.store_scalar(1546, tb);
            if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) {s.store_primal_offset(167, 167, 1.0);}
        }
        s.b[1644] = (s.v[1571] < 5.0);s.store_scalar(1644, if s.b[1644] { 1.0 } else { 0.0 });
        if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && s.b[1644]) {s.store_offset_square(1604, 1595, (10.0 * 2.220446049250313e-16));s.store_offset(1605, 1595, (10.0 * 2.220446049250313e-16));}
        if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (!s.b[1644])) {s.store_offset(1604, 1571, (-1.0));s.store_sqrt(1605, 1604);}
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) {s.store_mul(458, 1556, 1605);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_73(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) {s.store_div_from_scalar_add_ad(1524, 1.0, s.ad_value(1597), s.ad_value(1605));s.store_mul3_lhs(460, 1556, 1593, 1524);s.store_add(459, 458, 460);}
        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {s.store_sub(460, 459, 458);}
        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {
            if (p.p43 == 1.0) {
                s.store_mul(1527, 287, 1536);
            } else {
                s.store_mul(1527, 108, 1536);
            }
        }
        s.b[1646] = (((s.v[1542] != 0.0) && (p.p43 == 0.0)) || ((s.v[1540] != 0.0) && (p.p43 == 1.0)));s.store_scalar(1646, if s.b[1646] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1646]) {s.store_mul(455, 1527, 459);s.store_mul(457, 1527, 458);}
        s.b[1647] = (((s.v[1543] != 0.0) && (p.p43 == 0.0)) || ((s.v[1541] != 0.0) && (p.p43 == 1.0)));s.store_scalar(1647, if s.b[1647] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1647]) {s.store_mul(454, 1527, 459);s.store_mul(456, 1527, 458);}
        let (t13,) = {
    if ((p.p24 != 0.0) && s.b[1606]) {
        let t10: f64 = (s.v[462] * s.v[566]);let t11: f64 = (s.v[461] * s.v[565]);let t12: f64 = (t10 + t11);
        (t12,)
    } else {
        (s.v[266],)
    }
};
        s.store_scalar(266, t13);
        if (((p.p24 != 0.0) && s.b[1606]) && (s.v[266] != 0.0)) {s.store_add_scaled_inputs(269, 462, p.p170, 461, p.p169);}
        s.b[1648] = (p.p43 == 1.0);s.store_scalar(1648, if s.b[1648] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1606]) && (s.v[266] != 0.0)) && s.b[1648]) {s.store_add_scaled_products_indices(1524, 462, 287, 1.0, 461, 288, 1.0);s.store_mul_scale_offset_indices(269, 269, 1524, -1.0, 0.0);}
        if ((((p.p24 != 0.0) && s.b[1606]) && (s.v[266] != 0.0)) && (!s.b[1648])) {s.store_mul_scale_offset_indices(269, 269, 108, -1.0, 0.0);}
        if (((p.p24 != 0.0) && s.b[1606]) && (s.v[266] != 0.0)) {s.store_add_scaled_product_right_sub(268, 268, 1.0, 269, 158, 157, -1.0);}
        let (t17,) = {
    if ((p.p24 != 0.0) && s.b[1606]) {
        let t14: f64 = (s.v[461] * s.v[566]);let t15: f64 = (s.v[462] * s.v[565]);let t16: f64 = (t14 + t15);
        (t16,)
    } else {
        (s.v[266],)
    }
};
        s.store_scalar(266, t17);
        if (((p.p24 != 0.0) && s.b[1606]) && (s.v[266] != 0.0)) {s.store_add_scaled_inputs(270, 461, p.p170, 462, p.p169);}
        s.b[1649] = (p.p43 == 1.0);s.store_scalar(1649, if s.b[1649] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1606]) && (s.v[266] != 0.0)) && s.b[1649]) {s.store_add_scaled_products_indices(1524, 461, 287, 1.0, 462, 288, 1.0);s.store_mul_scale_offset_indices(270, 270, 1524, -1.0, 0.0);}
        if ((((p.p24 != 0.0) && s.b[1606]) && (s.v[266] != 0.0)) && (!s.b[1649])) {s.store_mul_scale_offset_indices(270, 270, 108, -1.0, 0.0);}
        if (((p.p24 != 0.0) && s.b[1606]) && (s.v[266] != 0.0)) {s.store_add_scaled_product_indices(267, 267, 1.0, 270, 158, -1.0);}
        s.b[1650] = (((s.v[613] == 1.0) && (!s.b[565])) || ((s.v[613] != 1.0) && (!s.b[566])));s.store_scalar(1650, if s.b[1650] { 1.0 } else { 0.0 });s.b[1651] = (p.p43 == 1.0);s.store_scalar(1651, if s.b[1651] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && (!s.b[1606])) && s.b[1650]) && s.b[1651]) {s.store_scale(269, 288, ((-s.v[1534]) * p.p188));}
        if ((((p.p24 != 0.0) && (!s.b[1606])) && s.b[1650]) && (!s.b[1651])) {s.store_scale(269, 108, ((-s.v[1534]) * p.p188));}
        if (((p.p24 != 0.0) && (!s.b[1606])) && (!s.b[1650])) {s.store_add_scaled_inputs(269, 462, p.p170, 461, p.p169);}
        s.b[1652] = (p.p43 == 1.0);s.store_scalar(1652, if s.b[1652] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && (!s.b[1606])) && (!s.b[1650])) && s.b[1652]) {s.store_add_scaled_products_indices(1524, 462, 287, 1.0, 461, 288, 1.0);s.store_mul_scale_offset_indices(269, 269, 1524, -1.0, 0.0);}
        if ((((p.p24 != 0.0) && (!s.b[1606])) && (!s.b[1650])) && (!s.b[1652])) {s.store_mul_scale_offset_indices(269, 269, 108, -1.0, 0.0);}
        if ((p.p24 != 0.0) && (!s.b[1606])) {s.store_mul_sub_scaled_inputs_rhs_indices(268, 269, 158, -1.0, 157, -1.0);}
        s.b[1653] = (((s.v[613] == 1.0) && (!s.b[566])) || ((s.v[613] != 1.0) && (!s.b[565])));s.store_scalar(1653, if s.b[1653] { 1.0 } else { 0.0 });s.b[1654] = (p.p43 == 1.0);s.store_scalar(1654, if s.b[1654] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && (!s.b[1606])) && s.b[1653]) && s.b[1654]) {s.store_scale(270, 287, ((-s.v[1534]) * p.p188));}
        if ((((p.p24 != 0.0) && (!s.b[1606])) && s.b[1653]) && (!s.b[1654])) {s.store_scale(270, 108, ((-s.v[1534]) * p.p188));}
        if (((p.p24 != 0.0) && (!s.b[1606])) && (!s.b[1653])) {s.store_add_scaled_inputs(270, 461, p.p170, 462, p.p169);}
        s.b[1655] = (p.p43 == 1.0);s.store_scalar(1655, if s.b[1655] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && (!s.b[1606])) && (!s.b[1653])) && s.b[1655]) {s.store_add_scaled_products_indices(1524, 461, 287, 1.0, 462, 288, 1.0);s.store_mul_scale_offset_indices(270, 270, 1524, -1.0, 0.0);}
        if ((((p.p24 != 0.0) && (!s.b[1606])) && (!s.b[1653])) && (!s.b[1655])) {s.store_mul_scale_offset_indices(270, 270, 108, -1.0, 0.0);}
        if ((p.p24 != 0.0) && (!s.b[1606])) {s.store_mul_scale_offset_indices(267, 158, 270, -1.0, 0.0);}
        s.b[1656] = (p.p43 == 1.0);s.store_scalar(1656, if s.b[1656] { 1.0 } else { 0.0 });
        if s.b[1656] {s.copy_ad(1672, 590);s.copy_ad(1673, 591);s.store_scale_ad(1674, A::exp_scaled_input(A::add_scaled_inputs(A::sub_from_scalar((s.v[87] * s.v[114]), A::mul(s.ad_value(237), s.ad_value(225))), 1.0, A::ln_scaled_input(s.ad_value(429), 1.0 / (s.v[81])), p.p175), 1.0 / (p.p174)), p.p173);s.store_scale_ad(1675, A::exp_scaled_input(A::add_scaled_inputs(A::sub_from_scalar((s.v[87] * s.v[114]), A::mul(s.ad_value(237), s.ad_value(225))), 1.0, A::ln_scaled_input(s.ad_value(429), 1.0 / (s.v[81])), p.p176), 1.0 / (p.p174)), p.p173);s.store_scaled_mul(1679, 286, 1674, p.p237);s.store_scaled_mul(1681, 286, 1675, p.p237);s.store_scaled_mul(1680, 285, 1674, p.p237);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_74(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1656] {s.store_scaled_mul(1682, 285, 1675, p.p237);s.store_scale(1658, 429, 1.0 / (s.v[81]));s.store_square(1657, 1658);s.store_offset(1659, 1679, 1e-50);s.store_offset(1660, 1680, 1e-50);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1677, p.p174, 225, A::ln_offset_div_scaled_inputs(s.ad_value(1657), p.p177, s.ad_value(1659), 1.0, 1.0));s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1678, p.p174, 225, A::ln_offset_div_scaled_inputs(s.ad_value(1657), p.p177, s.ad_value(1660), 1.0, 1.0));s.store_scale(1676, 227, p.p174);}
        s.b[1685] = (s.v[1672] < s.v[1677]);s.store_scalar(1685, if s.b[1685] { 1.0 } else { 0.0 });
        if (s.b[1656] && s.b[1685]) {s.store_exp_div(1658, 1672, 1676);s.store_mul_scale_offset_indices(282, 1679, 1658, 1.0, (-1.0));}
        if (s.b[1656] && (!s.b[1685])) {s.store_exp_div(1658, 1677, 1676);s.store_add_scaled_offset_product_rhs_mixed_aii(282, A::mul3(A::div(s.ad_value(1679), s.ad_value(1676)), s.ad_value(1658), A::sub(s.ad_value(1672), s.ad_value(1677))), 1.0, 1679, 1658, (-1.0), 1.0);}
        if s.b[1656] {s.store_add_scaled_product_indices(282, 282, 1.0, 1672, 1681, p.p178);}
        s.b[1686] = (s.v[1673] < s.v[1678]);s.store_scalar(1686, if s.b[1686] { 1.0 } else { 0.0 });
        if (s.b[1656] && s.b[1686]) {s.store_exp_div(1658, 1673, 1676);s.store_mul_scale_offset_indices(281, 1680, 1658, 1.0, (-1.0));}
        if (s.b[1656] && (!s.b[1686])) {s.store_exp_div(1658, 1678, 1676);s.store_add_scaled_offset_product_rhs_mixed_aii(281, A::mul3(A::div(s.ad_value(1680), s.ad_value(1676)), s.ad_value(1658), A::sub(s.ad_value(1673), s.ad_value(1678))), 1.0, 1680, 1658, (-1.0), 1.0);}
        if s.b[1656] {s.store_add_scaled_product_indices(281, 281, 1.0, 1673, 1682, p.p178);s.store_add_scaled_inputs(282, 282, 1.0, 1672, s.v[142]);s.store_add_scaled_inputs(281, 281, 1.0, 1673, s.v[142]);s.store_scalar(1666, (p.p179 * p.p2));s.store_scalar(1667, (p.p179 * p.p3));s.store_scalar(1665, (p.p237 - p.p238));}
        s.b[1687] = (s.v[1665] <= 0.0);s.store_scalar(1687, if s.b[1687] { 1.0 } else { 0.0 });
        if (s.b[1656] && s.b[1687]) {s.store_scalar(1666, 0.0);s.store_scalar(1667, 0.0);}
        s.b[1688] = (p.p5 > s.v[287]);s.store_scalar(1688, if s.b[1688] { 1.0 } else { 0.0 });
        if (s.b[1656] && s.b[1688]) {s.store_primal_offset_scaled(1669, 287, (-p.p180), ((p.p5) * (p.p180)));s.store_primal_scale(1671, 287, p.p181);}
        s.b[1689] = (s.v[1673] < 0.0);s.store_scalar(1689, if s.b[1689] { 1.0 } else { 0.0 });s.b[1690] = (s.v[1667] > 0.0);s.store_scalar(1690, if s.b[1690] { 1.0 } else { 0.0 });
        if (((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1690]) {s.store_sub_from_scalar_scaled_input(1683, 1.0, 1673, 1.0 / (p.p185));}
        s.b[1691] = (p.p182 == 0.5);s.store_scalar(1691, if s.b[1691] { 1.0 } else { 0.0 });
        if ((((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1690]) && s.b[1691]) {s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));}
        if ((((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1690]) && (!s.b[1691])) {s.store_powf(1684, 1683, (-p.p182));}
        if (((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1690]) {s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(283, 1667, 1.0, A::mul(s.ad_value(1683), s.ad_value(1684)), 1.0, (p.p185 * 1.0 / ((1.0 - p.p182))));}
        if (((s.b[1656] && s.b[1688]) && s.b[1689]) && (!s.b[1690])) {s.store_scalar(283, 0.0);}
        s.b[1692] = (s.v[1669] > 0.0);s.store_scalar(1692, if s.b[1692] { 1.0 } else { 0.0 });
        if (((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1692]) {s.store_sub_from_scalar_scaled_input(1683, 1.0, 1673, 1.0 / (p.p186));}
        s.b[1693] = (p.p183 == 0.5);s.store_scalar(1693, if s.b[1693] { 1.0 } else { 0.0 });
        if ((((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1692]) && s.b[1693]) {s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));}
        if ((((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1692]) && (!s.b[1693])) {s.store_powf(1684, 1683, (-p.p183));}
        if (((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1692]) {s.store_add_mixed_ia(283, 283, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1669), 1.0, A::mul(s.ad_value(1683), s.ad_value(1684)), (p.p186 * 1.0 / ((1.0 - p.p183)))));}
        s.b[1694] = (s.v[1671] > 0.0);s.store_scalar(1694, if s.b[1694] { 1.0 } else { 0.0 });
        if (((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1694]) {s.store_sub_from_scalar_scaled_input(1683, 1.0, 1673, 1.0 / (p.p187));}
        s.b[1695] = (p.p184 == 0.5);s.store_scalar(1695, if s.b[1695] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_75(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1694]) && s.b[1695]) {s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));}
        if ((((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1694]) && (!s.b[1695])) {s.store_powf(1684, 1683, (-p.p184));}
        if (((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1694]) {s.store_add_mixed_ia(283, 283, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1671), 1.0, A::mul(s.ad_value(1683), s.ad_value(1684)), (p.p187 * 1.0 / ((1.0 - p.p184)))));}
        if ((s.b[1656] && s.b[1688]) && (!s.b[1689])) {s.store_add_scaled_inputs3_indices(1658, 1667, 1.0, 1669, 1.0, 1671, 1.0);s.store_add_scaled_inputs3_indices(1659, 1667, (p.p182 * 1.0 / (p.p185)), 1669, (p.p183 * 1.0 / (p.p186)), 1671, (p.p184 * 1.0 / (p.p187)));s.store_mul_add_scaled_product_rhs_indices(283, 1673, 1658, 1.0, 1673, 1659, 0.5);}
        if (s.b[1656] && (!s.b[1688])) {s.store_scalar(1671, (p.p181 * p.p5));}
        s.b[1696] = (s.v[1673] < 0.0);s.store_scalar(1696, if s.b[1696] { 1.0 } else { 0.0 });s.b[1697] = (s.v[1667] > 0.0);s.store_scalar(1697, if s.b[1697] { 1.0 } else { 0.0 });
        if (((s.b[1656] && (!s.b[1688])) && s.b[1696]) && s.b[1697]) {s.store_sub_from_scalar_scaled_input(1683, 1.0, 1673, 1.0 / (p.p185));}
        s.b[1698] = (p.p182 == 0.5);s.store_scalar(1698, if s.b[1698] { 1.0 } else { 0.0 });
        if ((((s.b[1656] && (!s.b[1688])) && s.b[1696]) && s.b[1697]) && s.b[1698]) {s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));}
        if ((((s.b[1656] && (!s.b[1688])) && s.b[1696]) && s.b[1697]) && (!s.b[1698])) {s.store_powf(1684, 1683, (-p.p182));}
        if (((s.b[1656] && (!s.b[1688])) && s.b[1696]) && s.b[1697]) {s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(283, 1667, 1.0, A::mul(s.ad_value(1683), s.ad_value(1684)), 1.0, (p.p185 * 1.0 / ((1.0 - p.p182))));}
        if (((s.b[1656] && (!s.b[1688])) && s.b[1696]) && (!s.b[1697])) {s.store_scalar(283, 0.0);}
        s.b[1699] = (s.v[1671] > 0.0);s.store_scalar(1699, if s.b[1699] { 1.0 } else { 0.0 });
        if (((s.b[1656] && (!s.b[1688])) && s.b[1696]) && s.b[1699]) {s.store_sub_from_scalar_scaled_input(1683, 1.0, 1673, 1.0 / (p.p187));}
        s.b[1700] = (p.p184 == 0.5);s.store_scalar(1700, if s.b[1700] { 1.0 } else { 0.0 });
        if ((((s.b[1656] && (!s.b[1688])) && s.b[1696]) && s.b[1699]) && s.b[1700]) {s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));}
        if ((((s.b[1656] && (!s.b[1688])) && s.b[1696]) && s.b[1699]) && (!s.b[1700])) {s.store_powf(1684, 1683, (-p.p184));}
        if (((s.b[1656] && (!s.b[1688])) && s.b[1696]) && s.b[1699]) {s.store_add_mixed_ia(283, 283, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1671), 1.0, A::mul(s.ad_value(1683), s.ad_value(1684)), (p.p187 * 1.0 / ((1.0 - p.p184)))));}
        if ((s.b[1656] && (!s.b[1688])) && (!s.b[1696])) {s.store_add(1658, 1667, 1671);s.store_add_scaled_inputs(1659, 1667, (p.p182 * 1.0 / (p.p185)), 1671, (p.p184 * 1.0 / (p.p187)));s.store_mul_add_scaled_product_rhs_indices(283, 1673, 1658, 1.0, 1673, 1659, 0.5);}
        s.b[1701] = (p.p4 > s.v[288]);s.store_scalar(1701, if s.b[1701] { 1.0 } else { 0.0 });
        if (s.b[1656] && s.b[1701]) {s.store_primal_offset_scaled(1668, 288, (-p.p180), ((p.p4) * (p.p180)));s.store_primal_scale(1670, 288, p.p181);}
        s.b[1702] = (s.v[1672] < 0.0);s.store_scalar(1702, if s.b[1702] { 1.0 } else { 0.0 });s.b[1703] = (s.v[1666] > 0.0);s.store_scalar(1703, if s.b[1703] { 1.0 } else { 0.0 });
        if (((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1703]) {s.store_sub_from_scalar_scaled_input(1683, 1.0, 1672, 1.0 / (p.p185));}
        s.b[1704] = (p.p182 == 0.5);s.store_scalar(1704, if s.b[1704] { 1.0 } else { 0.0 });
        if ((((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1703]) && s.b[1704]) {s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));}
        if ((((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1703]) && (!s.b[1704])) {s.store_powf(1684, 1683, (-p.p182));}
        if (((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1703]) {s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(284, 1666, 1.0, A::mul(s.ad_value(1683), s.ad_value(1684)), 1.0, (p.p185 * 1.0 / ((1.0 - p.p182))));}
        if (((s.b[1656] && s.b[1701]) && s.b[1702]) && (!s.b[1703])) {s.store_scalar(284, 0.0);}
        s.b[1705] = (s.v[1668] > 0.0);s.store_scalar(1705, if s.b[1705] { 1.0 } else { 0.0 });
        if (((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1705]) {s.store_sub_from_scalar_scaled_input(1683, 1.0, 1672, 1.0 / (p.p186));}
        s.b[1706] = (p.p183 == 0.5);s.store_scalar(1706, if s.b[1706] { 1.0 } else { 0.0 });
        if ((((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1705]) && s.b[1706]) {s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));}
        if ((((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1705]) && (!s.b[1706])) {s.store_powf(1684, 1683, (-p.p183));}
        if (((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1705]) {s.store_add_mixed_ia(284, 284, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1668), 1.0, A::mul(s.ad_value(1683), s.ad_value(1684)), (p.p186 * 1.0 / ((1.0 - p.p183)))));}
        s.b[1707] = (s.v[1670] > 0.0);s.store_scalar(1707, if s.b[1707] { 1.0 } else { 0.0 });
        if (((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1707]) {s.store_sub_from_scalar_scaled_input(1683, 1.0, 1672, 1.0 / (p.p187));}
        s.b[1708] = (p.p184 == 0.5);s.store_scalar(1708, if s.b[1708] { 1.0 } else { 0.0 });
        if ((((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1707]) && s.b[1708]) {s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));}
        if ((((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1707]) && (!s.b[1708])) {s.store_powf(1684, 1683, (-p.p184));}
        if (((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1707]) {s.store_add_mixed_ia(284, 284, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1670), 1.0, A::mul(s.ad_value(1683), s.ad_value(1684)), (p.p187 * 1.0 / ((1.0 - p.p184)))));}
        if ((s.b[1656] && s.b[1701]) && (!s.b[1702])) {s.store_add_scaled_inputs3_indices(1658, 1666, 1.0, 1668, 1.0, 1670, 1.0);s.store_add_scaled_inputs3_indices(1659, 1666, (p.p182 * 1.0 / (p.p185)), 1668, (p.p183 * 1.0 / (p.p186)), 1670, (p.p184 * 1.0 / (p.p187)));s.store_mul_add_scaled_product_rhs_indices(284, 1672, 1658, 1.0, 1672, 1659, 0.5);}
        if (s.b[1656] && (!s.b[1701])) {s.store_scalar(1670, (p.p181 * p.p4));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_76(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1709] = (s.v[1672] < 0.0);s.store_scalar(1709, if s.b[1709] { 1.0 } else { 0.0 });s.b[1710] = (s.v[1666] > 0.0);s.store_scalar(1710, if s.b[1710] { 1.0 } else { 0.0 });
        if (((s.b[1656] && (!s.b[1701])) && s.b[1709]) && s.b[1710]) {s.store_sub_from_scalar_scaled_input(1683, 1.0, 1672, 1.0 / (p.p185));}
        s.b[1711] = (p.p182 == 0.5);s.store_scalar(1711, if s.b[1711] { 1.0 } else { 0.0 });
        if ((((s.b[1656] && (!s.b[1701])) && s.b[1709]) && s.b[1710]) && s.b[1711]) {s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));}
        if ((((s.b[1656] && (!s.b[1701])) && s.b[1709]) && s.b[1710]) && (!s.b[1711])) {s.store_powf(1684, 1683, (-p.p182));}
        if (((s.b[1656] && (!s.b[1701])) && s.b[1709]) && s.b[1710]) {s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(284, 1666, 1.0, A::mul(s.ad_value(1683), s.ad_value(1684)), 1.0, (p.p185 * 1.0 / ((1.0 - p.p182))));}
        if (((s.b[1656] && (!s.b[1701])) && s.b[1709]) && (!s.b[1710])) {s.store_scalar(284, 0.0);}
        s.b[1712] = (s.v[1670] > 0.0);s.store_scalar(1712, if s.b[1712] { 1.0 } else { 0.0 });
        if (((s.b[1656] && (!s.b[1701])) && s.b[1709]) && s.b[1712]) {s.store_sub_from_scalar_scaled_input(1683, 1.0, 1672, 1.0 / (p.p187));}
        s.b[1713] = (p.p184 == 0.5);s.store_scalar(1713, if s.b[1713] { 1.0 } else { 0.0 });
        if ((((s.b[1656] && (!s.b[1701])) && s.b[1709]) && s.b[1712]) && s.b[1713]) {s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));}
        if ((((s.b[1656] && (!s.b[1701])) && s.b[1709]) && s.b[1712]) && (!s.b[1713])) {s.store_powf(1684, 1683, (-p.p184));}
        if (((s.b[1656] && (!s.b[1701])) && s.b[1709]) && s.b[1712]) {s.store_add_mixed_ia(284, 284, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1670), 1.0, A::mul(s.ad_value(1683), s.ad_value(1684)), (p.p187 * 1.0 / ((1.0 - p.p184)))));}
        if ((s.b[1656] && (!s.b[1701])) && (!s.b[1709])) {s.store_add(1658, 1666, 1670);s.store_add_scaled_inputs(1659, 1666, (p.p182 * 1.0 / (p.p185)), 1670, (p.p184 * 1.0 / (p.p187)));s.store_mul_add_scaled_product_rhs_indices(284, 1672, 1658, 1.0, 1672, 1659, 0.5);}
        s.b[1714] = (s.v[1667] > 0.0);s.store_scalar(1714, if s.b[1714] { 1.0 } else { 0.0 });
        if (s.b[1656] && s.b[1714]) {s.store_scaled_mul(1661, 544, 1665, ((-1.6021918e-19) * p.p3));s.store_scale(1663, 1661, (-0.001));s.store_add_scaled_inputs3_indices(44, 1661, -1.0, 283, 1.0, 1663, -1.0);s.store_scaled_mul(45, 1661, 1663, (-4.0));}
        if (s.b[1656] && s.b[1714]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (s.b[1656] && s.b[1714]) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(283, 1661, -1.0, 44, (-0.5), 45, (-0.5));s.store_scale(283, 283, (-1.0));}
        s.b[1715] = (s.v[1666] > 0.0);s.store_scalar(1715, if s.b[1715] { 1.0 } else { 0.0 });
        if (s.b[1656] && s.b[1715]) {s.store_scaled_mul(1662, 544, 1665, ((-1.6021918e-19) * p.p2));s.store_scale(1664, 1662, (-0.001));s.store_add_scaled_inputs3_indices(44, 1662, -1.0, 284, 1.0, 1664, -1.0);s.store_scaled_mul(45, 1662, 1664, (-4.0));}
        if (s.b[1656] && s.b[1715]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (s.b[1656] && s.b[1715]) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(284, 1662, -1.0, 44, (-0.5), 45, (-0.5));s.store_scale(284, 284, (-1.0));}
        s.b[1721] = (s.v[145] == 0.0);s.store_scalar(1721, if s.b[1721] { 1.0 } else { 0.0 });
        if ((s.v[85] != 0.0) && s.b[1721]) {s.store_scalar(1716, p.p233);s.store_scalar(1717, p.p234);s.copy_ad(1718, 441);s.store_mul_product3_indices(1719, 1718, 1716, 1717, 1718, 1.0);s.store_offset_add_ad(1720, A::mul3(s.ad_value(250), s.ad_value(192), s.ad_value(1716)), A::mul3(s.ad_value(1717), s.ad_value(1718), s.ad_value(1718)), 1e-50);s.store_div(289, 1719, 1720);}
        if ((s.v[85] != 0.0) && (!s.b[1721])) {s.store_scalar(289, (p.p233 + 1e-50));}
        if (s.v[85] != 0.0) {s.store_scalar(1719, p.p235);s.store_mul(290, 1719, 323);}
        s.b[1748] = ((p.p32 != 0.0) && (s.v[145] == 0.0));s.store_scalar(1748, if s.b[1748] { 1.0 } else { 0.0 });
        if s.b[1748] {s.store_div_scaled_inputs2_indices(1731, 314, 1.0, 161, (-1.0), 441, 1.0);s.store_scaled_mul(1732, 251, 1731, 1e-5);}
        s.b[1749] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1749, if s.b[1749] { 1.0 } else { 0.0 });
        if (s.b[1748] && s.b[1749]) {s.store_scalar(1733, 1.0);}
        s.b[1750] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1750, if s.b[1750] { 1.0 } else { 0.0 });
        if ((s.b[1748] && (!s.b[1749])) && s.b[1750]) {s.copy_ad(1733, 1732);}
        if ((s.b[1748] && (!s.b[1749])) && (!s.b[1750])) {s.store_powf(1733, 1732, (p.p113 - 1.0));}
        if s.b[1748] {s.store_mul(1734, 1732, 1733);s.store_offset(1735, 1734, 1.0);s.store_powf(1736, 1735, (((-1.0) / p.p113) - 1.0));s.store_mul(1737, 1735, 1736);s.store_mul(293, 251, 1737);s.store_scaled_add(1739, 250, 293, 0.5);s.store_square(1738, 190);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_77(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1748] {s.store_div_scaled_product3_by_product_mixed_aiaai(292, A::mul3(s.ad_value(107), s.ad_value(323), s.ad_value(192)), 250, A::add_scaled_inputs3(A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(190), 3.0, 1.0), 1.0, s.ad_value(1738), 6.0), s.ad_value(293), s.ad_value(293)), 1.0, A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(190), 4.0, 3.0), 1.0, s.ad_value(1738), 3.0), s.ad_value(293), s.ad_value(250)), 1.0, A::mul3(A::add(A::scale_offset(s.ad_value(190), 3.0, 6.0), s.ad_value(1738)), s.ad_value(250), s.ad_value(250)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(441), A::offset(s.ad_value(190), 1.0), s.ad_value(1739), 15.0), 1739, 1.0);}
        if (!s.b[1748]) {s.store_scalar(292, 0.0);}
        s.b[1751] = ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (s.v[146] == 1.0)) && (s.v[145] == 0.0));s.store_scalar(1751, if s.b[1751] { 1.0 } else { 0.0 });
        if s.b[1751] {s.store_sqrt(298, 296);s.store_add(1740, 192, 298);s.store_square(1741, 294);s.store_square(1742, 296);s.store_scaled_mul(1743, 294, 296, 42.0);s.store_add_scaled_inputs3_indices(1743, 1743, 1.0, 1741, 4.0, 1742, 4.0);s.store_add_product3_rhs_mixed_iia(1743, 1743, 298, 192, A::add(s.ad_value(294), s.ad_value(296)), 20.0);s.store_square(1744, 1740);s.store_square(1736, 1744);s.store_div_scaled_value_by_product_indices(299, 1743, 1.0, 1736, 1740, 1.0);s.store_mul_ad_product_lhs_mixed_ai(300, A::div(s.ad_value(107), s.ad_value(441)), 250, 323);s.store_mul(1746, 300, 192);s.store_div(1747, 292, 1746);s.store_add_mixed_ai(1745, A::add_scaled_product(s.ad_value(294), 1.0, s.ad_value(192), s.ad_value(298), 4.0), 296);s.store_div_scaled_product_by_product_mixed_iiia(301, 297, 1745, 3.872983346207417, 1740, A::sqrt(A::mul(A::mul3(s.ad_value(1747), s.ad_value(1740), s.ad_value(192)), s.ad_value(1743))), 6.0);}
        s.store_add(199, 199, 265);s.b[1752] = (p.p43 == 1.0);s.store_scalar(1752, if s.b[1752] { 1.0 } else { 0.0 });
        if s.b[1752] {s.store_primal_add(271, 531, 532);}
        if (s.b[1752] && s.b[564]) {s.store_primal_offset(271, 271, (-(p.p168 * s.v[99])));}
        if s.b[1752] {s.store_mul_sub_scaled_inputs_rhs_indices(272, 271, 158, -1.0, 513, -1.0);s.store_scalar(276, ((3.453133e-11 / (3.141592653589793 / 2.0)) * (((1.0 + (p.p167 / s.v[88]))) as f64).ln()));s.store_primal_mul_scaled_offset_rhs(274, 276, p.p9, 518, s.v[101]);s.store_primal_mul_scaled_offset_rhs(275, 276, p.p9, 519, s.v[101]);s.store_mul_sub_rhs(277, 274, 158, 157);s.store_mul(278, 275, 158);s.store_mul_sub_scaled_inputs_rhs_indices(279, 276, 158, (p.p19 * p.p9), 513, (p.p19 * p.p9));s.store_add(268, 268, 277);s.store_add(267, 267, 278);s.store_add(272, 272, 279);}
        if ((!s.b[1752]) && s.b[564]) {s.store_scalar(271, ((-p.p168) * s.v[99]));s.store_mul_sub_scaled_inputs_rhs_indices(272, 271, 158, -1.0, 513, -1.0);}
        if ((!s.b[1752]) && (!s.b[564])) {s.store_scalar(271, 0.0);s.store_scalar(272, 0.0);}
        if (!s.b[1752]) {s.store_scalar(273, ((((3.453133e-11 / (3.141592653589793 / 2.0)) * s.v[101]) * p.p9) * (((1.0 + (p.p167 / s.v[88]))) as f64).ln()));s.copy_ad(274, 273);s.copy_ad(275, 273);s.store_mul_sub_rhs(277, 274, 158, 157);s.store_mul(278, 275, 158);s.store_add(268, 268, 277);s.store_add(267, 267, 278);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_78(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scale(9, 199, s.v[451]);
        if (s.v[85] != 0.0) {s.store_scalar(24, 0.0);s.store_scalar(23, 0.0);}
        s.b[1753] = (p.p43 == 1.0);s.store_scalar(1753, if s.b[1753] { 1.0 } else { 0.0 });
        if ((s.v[85] != 0.0) && s.b[1753]) {s.store_scalar(25, 0.0);s.copy_ad(556, 438);s.store_scale(588, 196, s.v[451]);s.store_scale(587, 197, s.v[451]);}
        if ((s.v[85] != 0.0) && (!s.b[1753])) {s.store_scalar(554, 0.0);s.store_scale(588, 392, s.v[451]);s.store_scaled_add(576, 198, 477, s.v[451]);s.store_add_scaled_inputs3_indices(577, 197, s.v[451], 198, ((-1.0) * s.v[451]), 476, s.v[451]);}
        s.b[1754] = (p.p43 == 1.0);s.store_scalar(1754, if s.b[1754] { 1.0 } else { 0.0 });
        if ((s.v[85] == 0.0) && s.b[1754]) {s.store_sub_scaled_inputs(23, 196, (-s.v[451]), 197, s.v[451]);s.store_scale(24, 198, s.v[451]);s.store_scaled_sub(25, 197, 198, s.v[451]);}
        if ((s.v[85] == 0.0) && (!s.b[1754])) {s.store_add_scaled_inputs4_indices(23, 392, (-s.v[451]), 197, ((-1.0) * s.v[451]), 476, (-s.v[451]), 477, (-s.v[451]));s.store_scaled_add(24, 198, 477, s.v[451]);s.store_add_scaled_inputs3_indices(25, 197, s.v[451], 198, ((-1.0) * s.v[451]), 476, s.v[451]);}
        s.b[1760] = (p.p64 == 0.0);s.store_scalar(1760, if s.b[1760] { 1.0 } else { 0.0 });
        if s.b[1760] {s.store_scalar(280, 0.0);}
        if (!s.b[1760]) {s.store_add_scaled_inputs(1755, 315, s.v[97], 161, 1.0);}
        s.b[1761] = (s.v[1755] > s.v[314]);s.store_scalar(1761, if s.b[1761] { 1.0 } else { 0.0 });
        if ((!s.b[1760]) && s.b[1761]) {s.copy_ad(1755, 314);}
        if (!s.b[1760]) {s.store_add_scaled_inputs3_indices(1756, 157, s.v[317], 161, s.v[317], 1755, (1.0 - s.v[317]));s.store_sqrt_div_from_scalar_ad(1757, (2.0 * 1.034943e-10), s.ad_value(229));s.store_scale(1758, 1757, 1.3);s.store_scaled_mul(1759, 108, 1758, 1.034943e-10);s.store_mul_add_scaled_inputs4_indices_rhs(280, 1759, 161, 1.0 / (p.p64), 157, 1.0 / (p.p64), 1756, (-1.0 / (p.p64)), 315, -1.0);}
        s.b[1762] = (p.p65 != 0.0);s.store_scalar(1762, if s.b[1762] { 1.0 } else { 0.0 });
        if s.b[1762] {s.store_add_scaled_product_indices(280, 280, 1.0, 135, 513, 1.0);}
        s.b[1763] = (p.p24 == 1.0);s.store_scalar(1763, if s.b[1763] { 1.0 } else { 0.0 });s.b[1764] = (p.p43 == 1.0);s.store_scalar(1764, if s.b[1764] { 1.0 } else { 0.0 });
        if (s.b[1763] && s.b[1764]) {s.store_add_scaled_inputs4_indices(471, 463, -1.0, 464, (-1.0), 467, -1.0, 468, -1.0);s.store_add(472, 466, 470);s.store_add(473, 465, 469);s.store_add_mixed_ia(23, 23, A::add_scaled_inputs(A::sub(A::sub(A::add_scaled_inputs4(s.ad_value(268), 1.0, s.ad_value(267), 1.0, s.ad_value(272), 1.0, s.ad_value(280), -1.0), s.ad_value(455)), s.ad_value(454)), s.v[451], s.ad_value(471), s.v[451]));s.store_add_mixed_ia(24, 24, A::add_scaled_inputs4(s.ad_value(280), s.v[451], s.ad_value(268), ((-1.0) * s.v[451]), s.ad_value(456), s.v[451], s.ad_value(472), s.v[451]));s.store_add_scaled_inputs4_indices(25, 25, 1.0, 457, s.v[451], 267, ((-1.0) * s.v[451]), 473, s.v[451]);}
        if (s.b[1763] && (!s.b[1764])) {s.store_add_mixed_ia(23, 23, A::sub_scaled_inputs(A::sub(A::add_scaled_inputs4(s.ad_value(268), 1.0, s.ad_value(267), 1.0, s.ad_value(272), 1.0, s.ad_value(280), -1.0), s.ad_value(455)), s.v[451], s.ad_value(454), s.v[451]));s.store_add_scaled_inputs4_indices(24, 24, 1.0, 280, s.v[451], 268, ((-1.0) * s.v[451]), 456, s.v[451]);s.store_add_scaled_inputs3_indices(25, 25, 1.0, 457, s.v[451], 267, (-s.v[451]));}
        s.b[1765] = (p.p43 == 1.0);s.store_scalar(1765, if s.b[1765] { 1.0 } else { 0.0 });
        if s.b[1765] {s.store_scale(36, 281, s.v[451]);s.store_scale(35, 282, s.v[451]);s.store_scale(560, 284, s.v[451]);s.store_scale(561, 283, s.v[451]);}
        if (!s.b[1765]) {s.store_scalar(36, 0.0);s.store_scalar(35, 0.0);s.store_scalar(560, 0.0);s.store_scalar(561, 0.0);}
        s.b[1766] = (p.p25 != 1.0);s.store_scalar(1766, if s.b[1766] { 1.0 } else { 0.0 });
        if s.b[1766] {s.store_scalar(557, 0.0);}
        if (!s.b[1766]) {s.store_scale(557, 263, s.v[451]);}
        s.store_scale(15, 308, (-s.v[451]));s.b[1767] = (s.v[613] == 1.0);s.store_scalar(1767, if s.b[1767] { 1.0 } else { 0.0 });
        if s.b[1767] {s.store_add_scaled_product_indices(13, 307, ((-1.0) * s.v[451]), 310, 309, s.v[451]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_79(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (!s.b[1767]) {s.store_scaled_sub_mixed_ai(13, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(310), s.ad_value(309)), 306, s.v[451]);}
        s.b[1768] = (s.v[613] == 1.0);s.store_scalar(1768, if s.b[1768] { 1.0 } else { 0.0 });
        if s.b[1768] {s.store_scaled_sub_mixed_ai(14, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(310), s.ad_value(309)), 306, s.v[451]);}
        if (!s.b[1768]) {s.store_add_scaled_product_indices(14, 307, ((-1.0) * s.v[451]), 310, 309, s.v[451]);}
        if (s.v[613] == 1.0) {
            s.store_scale(11, 311, s.v[451]);
        } else {
            s.store_scale(11, 312, s.v[451]);
        }
        if (s.v[613] == 1.0) {
            s.store_scale(12, 312, s.v[451]);
        } else {
            s.store_scale(12, 311, s.v[451]);
        }
        s.store_scale(598, 292, s.v[451]);s.store_scalar(27, A::ddx_projection(&s.ad_value(23), Some(6), None));s.store_scale(27, 27, p.p50);s.store_scalar(28, A::ddx_projection(&s.ad_value(23), Some(7), None));s.store_scale(28, 28, p.p50);
        if (s.v[613] > 0.0) {
            s.copy_ad(555, 28);
        } else {
            s.copy_ad(555, 27);
        }
        s.b[1775] = ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (s.v[146] == 1.0)) && (s.v[145] == 0.0));s.store_scalar(1775, if s.b[1775] { 1.0 } else { 0.0 });
        let (t1b,) = {
    if s.b[1775] {
        let t18: f64 = (1e-6 * s.v[323]);let t19: f64 = (t18 * s.v[108]);let t1a: f64 = (t19 * s.v[98]);
        (t1a,)
    } else {
        (s.v[1769],)
    }
};
        s.store_scalar(1769, t1b);
        if s.b[1775] {s.store_scale(1770, 555, 1.0 / (s.v[451]));s.store_div_scaled_product3_indices(1771, 227, 1770, 1770, (0.1185185185185185 * 1.6021918e-19), 300, 1.0);}
        s.b[1776] = ((s.v[297] > (10.0 * 2.220446049250313e-16)) && (s.v[157] > (10.0 * 2.220446049250313e-16)));s.store_scalar(1776, if s.b[1776] { 1.0 } else { 0.0 });
        if (s.b[1775] && s.b[1776]) {s.store_div(1772, 251, 250);s.store_div_scaled_inputs2_mixed_aii(1773, A::div(s.ad_value(251), s.ad_value(293)), 1.0, 1772, (-1.0), 157, 1.0);s.store_add_mixed_ia(1774, 1772, A::div_scaled_product(s.ad_value(1773), A::add(A::add_scaled_product(s.ad_value(294), 1.0, s.ad_value(192), s.ad_value(298), 1.0), s.ad_value(296)), 0.6666666666666667, A::add(s.ad_value(192), s.ad_value(298)), 1.0));}
        if (s.b[1775] && (!s.b[1776])) {s.store_div(1774, 251, 293);}
        if s.b[1775] {s.store_mul3_affine_lhs(558, 1771, 299, s.v[451], 0.0, 1774);s.copy_ad(559, 301);}
        if s.b[1775] {
            if (((-s.v[1770]) > s.v[1769]) && (s.v[558] > 0.0)) {
            } else {
                s.store_scalar(558, 0.0);
            }
        }
        if s.b[1775] {
            if ((-s.v[1770]) > s.v[1769]) {
            } else {
                s.store_scalar(559, 0.0);
            }
        }
        if (!s.b[1775]) {s.store_scalar(558, 0.0);s.store_scalar(559, 0.0);}
        s.store_scalar(4, 0.0);s.store_scalar(5, 0.0);s.b[1777] = (p.p259 == 1.0);s.store_scalar(1777, if s.b[1777] { 1.0 } else { 0.0 });
        let (t1c,) = {
    if s.b[1777] {
        (1.0,)
    } else {
        (s.v[3],)
    }
};
        s.store_scalar(3, t1c);s.b[1797] = (s.v[3] == 1.0);s.store_scalar(1797, if s.b[1797] { 1.0 } else { 0.0 });
        if (s.b[1777] && s.b[1797]) {s.store_scalar(1788, (p.p264 / 1e-6));s.store_scalar(1781, p.p266);s.store_scalar(1782, p.p268);s.store_scalar(1783, p.p273);}
        if (s.b[1777] && s.b[1797]) {s.store_scalar(1784, (if (p.p263 > 0.0) { (p.p263 * p.p255) } else { 0.0 }));}
        if (s.b[1777] && s.b[1797]) {s.store_scalar(1787, p.p258);s.store_scaled_voltage(1785, ctx, nodes, Some(7), Some(2), p.p50);}
        if (s.b[1777] && (!s.b[1797])) {s.store_scalar(1788, (p.p59 / 1e-6));s.store_scalar(1781, p.p265);s.store_scalar(1782, p.p267);s.store_scalar(1783, p.p272);}
        if (s.b[1777] && (!s.b[1797])) {s.store_scalar(1784, (if (p.p263 > 0.0) { (p.p263 * p.p256) } else { 0.0 }));}
        if (s.b[1777] && (!s.b[1797])) {s.store_scalar(1787, p.p257);s.store_scaled_voltage(1785, ctx, nodes, Some(0), Some(6), p.p50);}
        if s.b[1777] {s.store_scalar(1794, ((((p.p271 * p.p271) + (p.p56 * p.p56))) as f64).sqrt());s.store_primal_scale(1796, 105, p.p9);s.store_primal_scale(1781, 1781, 0.0001);s.store_primal_scale(1782, 1782, 0.01);s.store_scale(1786, 429, 1.0 / (s.v[81]));s.store_powf(328, 1786, p.p269);s.store_div(1789, 1781, 328);}
    }
}
