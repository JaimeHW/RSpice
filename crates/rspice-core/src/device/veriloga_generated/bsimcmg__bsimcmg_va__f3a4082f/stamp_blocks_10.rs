#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_25(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1606] = (p.p1600 != 1.0);
        s.store_scalar(1606, if s.b[1606] { 1.0 } else { 0.0 });

        s.b[1607] = (p.p1600 == 0.5);
        s.store_scalar(1607, if s.b[1607] { 1.0 } else { 0.0 });

        if ((((s.b[1523] && s.b[1594]) && (!s.b[1595])) && s.b[1606]) && s.b[1607]) {
            s.store_scalar(1587, (1.0 / ((0.1) as f64).sqrt()));
        }

        if ((((s.b[1523] && s.b[1594]) && (!s.b[1595])) && s.b[1606]) && (!s.b[1607])) {
            s.store_scalar(1587, ((0.1) as f64).powf((-p.p1600)));
        }

        if (((s.b[1523] && s.b[1594]) && (!s.b[1595])) && s.b[1606]) {
            s.store_scalar(1588, (1.0 / (1.0 - p.p1600)));
            s.store_mul_sub_from_scalar_ad_rhs(1590, 1588, 1.0, A::scale(s.ad_value(1587), ((0.05 * p.p1600) * (1.0 + p.p1600))));
        }

        if (((s.b[1523] && s.b[1594]) && (!s.b[1595])) && (!s.b[1606])) {
            s.store_scalar(1587, 10.0);
            s.store_scalar(1590, (1.5 - ((0.1) as f64).ln()));
        }

        if ((s.b[1523] && s.b[1594]) && (!s.b[1595])) {
            s.store_mul_ad_product_rhs(1589, 1587, A::offset(s.ad_value(1586), (-1.0)), A::scale_offset(s.ad_value(1586), (5.0 * p.p1600), (((((-1.0)) * ((5.0 * p.p1600)))) + ((1.0 + p.p1600)))));
            s.store_mul_ad_product_rhs_mixed_ia(532, 271, 525, A::add(s.ad_value(1589), s.ad_value(1590)));
        }

        if (s.b[1523] && (!s.b[1594])) {
            s.store_scalar(532, 0.0);
        }

        if s.b[1523] {
            s.store_add_scaled_inputs3_indices(529, 530, 1.0, 531, 1.0, 532, 1.0);
        }

        s.b[1616] = (s.v[526] > 0.0);
        s.store_scalar(1616, if s.b[1616] { 1.0 } else { 0.0 });

        if (s.b[1523] && s.b[1616]) {
            s.store_div(1608, 522, 272);
        }

        s.b[1617] = (s.v[1608] < 0.9);
        s.store_scalar(1617, if s.b[1617] { 1.0 } else { 0.0 });

        s.b[1618] = (p.p1603 > 0.0);
        s.store_scalar(1618, if s.b[1618] { 1.0 } else { 0.0 });

        s.b[1619] = (s.v[522] > s.v[563]);
        s.store_scalar(1619, if s.b[1619] { 1.0 } else { 0.0 });

        if ((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && s.b[1619]) {
            s.store_sub_from_scalar(1613, 1.0, 1608);
        }

        s.b[1620] = (p.p1597 != 1.0);
        s.store_scalar(1620, if s.b[1620] { 1.0 } else { 0.0 });

        s.b[1621] = (p.p1597 == 0.5);
        s.store_scalar(1621, if s.b[1621] { 1.0 } else { 0.0 });

        if ((((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && s.b[1619]) && s.b[1620]) && s.b[1621]) {
            s.store_div_from_scalar_sqrt_ad(1614, 1.0, s.ad_value(1613));
        }

        if ((((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && s.b[1619]) && s.b[1620]) && (!s.b[1621])) {
            s.store_powf(1614, 1613, (-p.p1597));
        }

        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && s.b[1619]) && s.b[1620]) {
            s.store_mul_ad_affine_product_rhs(534, 272, s.ad_value(526), A::sub_from_scalar(1.0, A::mul(s.ad_value(1613), s.ad_value(1614))), 1.0 / ((1.0 - p.p1597)), 0.0);
        }

        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && s.b[1619]) && (!s.b[1620])) {
            s.store_mul_ad_affine_product_rhs(534, 272, s.ad_value(526), {
                if (!(s.v[1613] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1613] > 1e-38) {
                            A::ln(s.ad_value(1613))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }

        if ((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) {
            s.store_sub_from_scalar_div_indices(1613, 1.0, 563, 272);
        }

        s.b[1622] = (p.p1597 != 1.0);
        s.store_scalar(1622, if s.b[1622] { 1.0 } else { 0.0 });

        s.b[1623] = (p.p1597 == 0.5);
        s.store_scalar(1623, if s.b[1623] { 1.0 } else { 0.0 });

        if ((((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1622]) && s.b[1623]) {
            s.store_div_from_scalar_sqrt_ad(1614, 1.0, s.ad_value(1613));
        }

        if ((((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1622]) && (!s.b[1623])) {
            s.store_powf(1614, 1613, (-p.p1597));
        }

        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1622]) {
            s.store_mul_ad_affine_product_rhs(1615, 272, s.ad_value(526), A::sub_from_scalar(1.0, A::mul(s.ad_value(1613), s.ad_value(1614))), 1.0 / ((1.0 - p.p1597)), 0.0);
        }

        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && (!s.b[1622])) {
            s.store_mul_ad_affine_product_rhs(1615, 272, s.ad_value(526), {
                if (!(s.v[1613] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1613] > 1e-38) {
                            A::ln(s.ad_value(1613))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }

        if ((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) {
            s.store_sub_from_scalar_ad(1613, 1.0, A::div_scaled_inputs2(s.ad_value(522), 1.0, s.ad_value(563), (-1.0), s.ad_value(564), 1.0));
        }

        s.b[1624] = (p.p1609 != 1.0);
        s.store_scalar(1624, if s.b[1624] { 1.0 } else { 0.0 });

        s.b[1625] = (p.p1609 == 0.5);
        s.store_scalar(1625, if s.b[1625] { 1.0 } else { 0.0 });

        if ((((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1624]) && s.b[1625]) {
            s.store_div_from_scalar_sqrt_ad(1614, 1.0, s.ad_value(1613));
        }

        if ((((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1624]) && (!s.b[1625])) {
            s.store_powf(1614, 1613, (-p.p1609));
        }

        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1624]) {
            s.store_add_product3_rhs_mixed_iia(534, 1615, 564, 526, A::sub_from_scalar(1.0, A::mul(s.ad_value(1613), s.ad_value(1614))), (p.p1603 * 1.0 / ((1.0 - p.p1609))));
        }

        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && (!s.b[1624])) {
            s.store_sub_ad_rhs(534, 1615, A::mul3_scaled_output(s.ad_value(564), s.ad_value(526), {
                if (!(s.v[1613] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1613] > 1e-38) {
                            A::ln(s.ad_value(1613))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1603));
        }

        if (((s.b[1523] && s.b[1616]) && s.b[1617]) && (!s.b[1618])) {
            s.store_sub_from_scalar(1613, 1.0, 1608);
        }

        s.b[1626] = (p.p1597 != 1.0);
        s.store_scalar(1626, if s.b[1626] { 1.0 } else { 0.0 });

        s.b[1627] = (p.p1597 == 0.5);
        s.store_scalar(1627, if s.b[1627] { 1.0 } else { 0.0 });

        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && (!s.b[1618])) && s.b[1626]) && s.b[1627]) {
            s.store_div_from_scalar_sqrt_ad(1614, 1.0, s.ad_value(1613));
        }

        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && (!s.b[1618])) && s.b[1626]) && (!s.b[1627])) {
            s.store_powf(1614, 1613, (-p.p1597));
        }

        if ((((s.b[1523] && s.b[1616]) && s.b[1617]) && (!s.b[1618])) && s.b[1626]) {
            s.store_mul_ad_affine_product_rhs(534, 272, s.ad_value(526), A::sub_from_scalar(1.0, A::mul(s.ad_value(1613), s.ad_value(1614))), 1.0 / ((1.0 - p.p1597)), 0.0);
        }

        if ((((s.b[1523] && s.b[1616]) && s.b[1617]) && (!s.b[1618])) && (!s.b[1626])) {
            s.store_mul_ad_affine_product_rhs(534, 272, s.ad_value(526), {
                if (!(s.v[1613] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1613] > 1e-38) {
                            A::ln(s.ad_value(1613))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }

        s.b[1628] = (p.p1597 != 1.0);
        s.store_scalar(1628, if s.b[1628] { 1.0 } else { 0.0 });

        s.b[1629] = (p.p1597 == 0.5);
        s.store_scalar(1629, if s.b[1629] { 1.0 } else { 0.0 });

        if ((((s.b[1523] && s.b[1616]) && (!s.b[1617])) && s.b[1628]) && s.b[1629]) {
            s.store_scalar(1609, (1.0 / ((0.1) as f64).sqrt()));
        }

        if ((((s.b[1523] && s.b[1616]) && (!s.b[1617])) && s.b[1628]) && (!s.b[1629])) {
            s.store_scalar(1609, ((0.1) as f64).powf((-p.p1597)));
        }

        if (((s.b[1523] && s.b[1616]) && (!s.b[1617])) && s.b[1628]) {
            s.store_scalar(1610, (1.0 / (1.0 - p.p1597)));
            s.store_mul_sub_from_scalar_ad_rhs(1612, 1610, 1.0, A::scale(s.ad_value(1609), ((0.05 * p.p1597) * (1.0 + p.p1597))));
        }

        if (((s.b[1523] && s.b[1616]) && (!s.b[1617])) && (!s.b[1628])) {
            s.store_scalar(1609, 10.0);
            s.store_scalar(1612, (1.5 - ((0.1) as f64).ln()));
        }

        if ((s.b[1523] && s.b[1616]) && (!s.b[1617])) {
            s.store_mul_ad_product_rhs(1611, 1609, A::offset(s.ad_value(1608), (-1.0)), A::scale_offset(s.ad_value(1608), (5.0 * p.p1597), (((((-1.0)) * ((5.0 * p.p1597)))) + ((1.0 + p.p1597)))));
            s.store_mul_ad_product_rhs_mixed_ia(534, 272, 526, A::add(s.ad_value(1611), s.ad_value(1612)));
        }

        if (s.b[1523] && (!s.b[1616])) {
            s.store_scalar(534, 0.0);
        }

        s.b[1638] = (s.v[527] > 0.0);
        s.store_scalar(1638, if s.b[1638] { 1.0 } else { 0.0 });

        if (s.b[1523] && s.b[1638]) {
            s.store_div(1630, 522, 273);
        }

        s.b[1639] = (s.v[1630] < 0.9);
        s.store_scalar(1639, if s.b[1639] { 1.0 } else { 0.0 });

        s.b[1640] = (p.p1605 > 0.0);
        s.store_scalar(1640, if s.b[1640] { 1.0 } else { 0.0 });

        s.b[1641] = (s.v[522] > s.v[565]);
        s.store_scalar(1641, if s.b[1641] { 1.0 } else { 0.0 });

        if ((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && s.b[1641]) {
            s.store_sub_from_scalar(1635, 1.0, 1630);
        }

        s.b[1642] = (p.p1599 != 1.0);
        s.store_scalar(1642, if s.b[1642] { 1.0 } else { 0.0 });

        s.b[1643] = (p.p1599 == 0.5);
        s.store_scalar(1643, if s.b[1643] { 1.0 } else { 0.0 });

        if ((((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && s.b[1641]) && s.b[1642]) && s.b[1643]) {
            s.store_div_from_scalar_sqrt_ad(1636, 1.0, s.ad_value(1635));
        }

        if ((((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && s.b[1641]) && s.b[1642]) && (!s.b[1643])) {
            s.store_powf(1636, 1635, (-p.p1599));
        }

        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && s.b[1641]) && s.b[1642]) {
            s.store_mul_ad_affine_product_rhs(535, 273, s.ad_value(527), A::sub_from_scalar(1.0, A::mul(s.ad_value(1635), s.ad_value(1636))), 1.0 / ((1.0 - p.p1599)), 0.0);
        }

        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && s.b[1641]) && (!s.b[1642])) {
            s.store_mul_ad_affine_product_rhs(535, 273, s.ad_value(527), {
                if (!(s.v[1635] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1635] > 1e-38) {
                            A::ln(s.ad_value(1635))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }

        if ((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) {
            s.store_sub_from_scalar_div_indices(1635, 1.0, 565, 273);
        }

        s.b[1644] = (p.p1599 != 1.0);
        s.store_scalar(1644, if s.b[1644] { 1.0 } else { 0.0 });

        s.b[1645] = (p.p1599 == 0.5);
        s.store_scalar(1645, if s.b[1645] { 1.0 } else { 0.0 });

        if ((((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1644]) && s.b[1645]) {
            s.store_div_from_scalar_sqrt_ad(1636, 1.0, s.ad_value(1635));
        }

        if ((((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1644]) && (!s.b[1645])) {
            s.store_powf(1636, 1635, (-p.p1599));
        }

        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1644]) {
            s.store_mul_ad_affine_product_rhs(1637, 273, s.ad_value(527), A::sub_from_scalar(1.0, A::mul(s.ad_value(1635), s.ad_value(1636))), 1.0 / ((1.0 - p.p1599)), 0.0);
        }

        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && (!s.b[1644])) {
            s.store_mul_ad_affine_product_rhs(1637, 273, s.ad_value(527), {
                if (!(s.v[1635] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1635] > 1e-38) {
                            A::ln(s.ad_value(1635))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }

        if ((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) {
            s.store_sub_from_scalar_ad(1635, 1.0, A::div_scaled_inputs2(s.ad_value(522), 1.0, s.ad_value(565), (-1.0), s.ad_value(566), 1.0));
        }

        s.b[1646] = (p.p1611 != 1.0);
        s.store_scalar(1646, if s.b[1646] { 1.0 } else { 0.0 });

        s.b[1647] = (p.p1611 == 0.5);
        s.store_scalar(1647, if s.b[1647] { 1.0 } else { 0.0 });

        if ((((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1646]) && s.b[1647]) {
            s.store_div_from_scalar_sqrt_ad(1636, 1.0, s.ad_value(1635));
        }

        if ((((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1646]) && (!s.b[1647])) {
            s.store_powf(1636, 1635, (-p.p1611));
        }

        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1646]) {
            s.store_add_product3_rhs_mixed_iia(535, 1637, 566, 527, A::sub_from_scalar(1.0, A::mul(s.ad_value(1635), s.ad_value(1636))), (p.p1605 * 1.0 / ((1.0 - p.p1611))));
        }

        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && (!s.b[1646])) {
            s.store_sub_ad_rhs(535, 1637, A::mul3_scaled_output(s.ad_value(566), s.ad_value(527), {
                if (!(s.v[1635] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1635] > 1e-38) {
                            A::ln(s.ad_value(1635))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1605));
        }

        if (((s.b[1523] && s.b[1638]) && s.b[1639]) && (!s.b[1640])) {
            s.store_sub_from_scalar(1635, 1.0, 1630);
        }

        s.b[1648] = (p.p1599 != 1.0);
        s.store_scalar(1648, if s.b[1648] { 1.0 } else { 0.0 });

        s.b[1649] = (p.p1599 == 0.5);
        s.store_scalar(1649, if s.b[1649] { 1.0 } else { 0.0 });

        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && (!s.b[1640])) && s.b[1648]) && s.b[1649]) {
            s.store_div_from_scalar_sqrt_ad(1636, 1.0, s.ad_value(1635));
        }

        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && (!s.b[1640])) && s.b[1648]) && (!s.b[1649])) {
            s.store_powf(1636, 1635, (-p.p1599));
        }

        if ((((s.b[1523] && s.b[1638]) && s.b[1639]) && (!s.b[1640])) && s.b[1648]) {
            s.store_mul_ad_affine_product_rhs(535, 273, s.ad_value(527), A::sub_from_scalar(1.0, A::mul(s.ad_value(1635), s.ad_value(1636))), 1.0 / ((1.0 - p.p1599)), 0.0);
        }

        if ((((s.b[1523] && s.b[1638]) && s.b[1639]) && (!s.b[1640])) && (!s.b[1648])) {
            s.store_mul_ad_affine_product_rhs(535, 273, s.ad_value(527), {
                if (!(s.v[1635] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1635] > 1e-38) {
                            A::ln(s.ad_value(1635))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }

        s.b[1650] = (p.p1599 != 1.0);
        s.store_scalar(1650, if s.b[1650] { 1.0 } else { 0.0 });

        s.b[1651] = (p.p1599 == 0.5);
        s.store_scalar(1651, if s.b[1651] { 1.0 } else { 0.0 });

        if ((((s.b[1523] && s.b[1638]) && (!s.b[1639])) && s.b[1650]) && s.b[1651]) {
            s.store_scalar(1631, (1.0 / ((0.1) as f64).sqrt()));
        }

        if ((((s.b[1523] && s.b[1638]) && (!s.b[1639])) && s.b[1650]) && (!s.b[1651])) {
            s.store_scalar(1631, ((0.1) as f64).powf((-p.p1599)));
        }

        if (((s.b[1523] && s.b[1638]) && (!s.b[1639])) && s.b[1650]) {
            s.store_scalar(1632, (1.0 / (1.0 - p.p1599)));
            s.store_mul_sub_from_scalar_ad_rhs(1634, 1632, 1.0, A::scale(s.ad_value(1631), ((0.05 * p.p1599) * (1.0 + p.p1599))));
        }

        if (((s.b[1523] && s.b[1638]) && (!s.b[1639])) && (!s.b[1650])) {
            s.store_scalar(1631, 10.0);
            s.store_scalar(1634, (1.5 - ((0.1) as f64).ln()));
        }

        if ((s.b[1523] && s.b[1638]) && (!s.b[1639])) {
            s.store_mul_ad_product_rhs(1633, 1631, A::offset(s.ad_value(1630), (-1.0)), A::scale_offset(s.ad_value(1630), (5.0 * p.p1599), (((((-1.0)) * ((5.0 * p.p1599)))) + ((1.0 + p.p1599)))));
            s.store_mul_ad_product_rhs_mixed_ia(535, 273, 527, A::add(s.ad_value(1633), s.ad_value(1634)));
        }

        if (s.b[1523] && (!s.b[1638])) {
            s.store_scalar(535, 0.0);
        }

        s.b[1660] = (s.v[528] > 0.0);
        s.store_scalar(1660, if s.b[1660] { 1.0 } else { 0.0 });

        if (s.b[1523] && s.b[1660]) {
            s.store_div(1652, 522, 274);
        }

        s.b[1661] = (s.v[1652] < 0.9);
        s.store_scalar(1661, if s.b[1661] { 1.0 } else { 0.0 });

        s.b[1662] = (p.p1607 > 0.0);
        s.store_scalar(1662, if s.b[1662] { 1.0 } else { 0.0 });

        s.b[1663] = (s.v[522] > s.v[567]);
        s.store_scalar(1663, if s.b[1663] { 1.0 } else { 0.0 });

        if ((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && s.b[1663]) {
            s.store_sub_from_scalar(1657, 1.0, 1652);
        }

        s.b[1664] = (p.p1601 != 1.0);
        s.store_scalar(1664, if s.b[1664] { 1.0 } else { 0.0 });

        s.b[1665] = (p.p1601 == 0.5);
        s.store_scalar(1665, if s.b[1665] { 1.0 } else { 0.0 });

        if ((((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && s.b[1663]) && s.b[1664]) && s.b[1665]) {
            s.store_div_from_scalar_sqrt_ad(1658, 1.0, s.ad_value(1657));
        }

        if ((((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && s.b[1663]) && s.b[1664]) && (!s.b[1665])) {
            s.store_powf(1658, 1657, (-p.p1601));
        }

        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && s.b[1663]) && s.b[1664]) {
            s.store_mul_ad_affine_product_rhs(536, 274, s.ad_value(528), A::sub_from_scalar(1.0, A::mul(s.ad_value(1657), s.ad_value(1658))), 1.0 / ((1.0 - p.p1601)), 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_26(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && s.b[1663]) && (!s.b[1664])) {
            s.store_mul_ad_affine_product_rhs(536, 274, s.ad_value(528), {
                if (!(s.v[1657] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1657] > 1e-38) {
                            A::ln(s.ad_value(1657))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }

        if ((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) {
            s.store_sub_from_scalar_div_indices(1657, 1.0, 567, 274);
        }

        s.b[1666] = (p.p1601 != 1.0);
        s.store_scalar(1666, if s.b[1666] { 1.0 } else { 0.0 });

        s.b[1667] = (p.p1601 == 0.5);
        s.store_scalar(1667, if s.b[1667] { 1.0 } else { 0.0 });

        if ((((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1666]) && s.b[1667]) {
            s.store_div_from_scalar_sqrt_ad(1658, 1.0, s.ad_value(1657));
        }

        if ((((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1666]) && (!s.b[1667])) {
            s.store_powf(1658, 1657, (-p.p1601));
        }

        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1666]) {
            s.store_mul_ad_affine_product_rhs(1659, 274, s.ad_value(528), A::sub_from_scalar(1.0, A::mul(s.ad_value(1657), s.ad_value(1658))), 1.0 / ((1.0 - p.p1601)), 0.0);
        }

        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && (!s.b[1666])) {
            s.store_mul_ad_affine_product_rhs(1659, 274, s.ad_value(528), {
                if (!(s.v[1657] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1657] > 1e-38) {
                            A::ln(s.ad_value(1657))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }

        if ((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) {
            s.store_sub_from_scalar_ad(1657, 1.0, A::div_scaled_inputs2(s.ad_value(522), 1.0, s.ad_value(567), (-1.0), s.ad_value(568), 1.0));
        }

        s.b[1668] = (p.p1613 != 1.0);
        s.store_scalar(1668, if s.b[1668] { 1.0 } else { 0.0 });

        s.b[1669] = (p.p1613 == 0.5);
        s.store_scalar(1669, if s.b[1669] { 1.0 } else { 0.0 });

        if ((((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1668]) && s.b[1669]) {
            s.store_div_from_scalar_sqrt_ad(1658, 1.0, s.ad_value(1657));
        }

        if ((((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1668]) && (!s.b[1669])) {
            s.store_powf(1658, 1657, (-p.p1613));
        }

        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1668]) {
            s.store_add_product3_rhs_mixed_iia(536, 1659, 568, 528, A::sub_from_scalar(1.0, A::mul(s.ad_value(1657), s.ad_value(1658))), (p.p1607 * 1.0 / ((1.0 - p.p1613))));
        }

        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && (!s.b[1668])) {
            s.store_sub_ad_rhs(536, 1659, A::mul3_scaled_output(s.ad_value(568), s.ad_value(528), {
                if (!(s.v[1657] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1657] > 1e-38) {
                            A::ln(s.ad_value(1657))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1607));
        }

        if (((s.b[1523] && s.b[1660]) && s.b[1661]) && (!s.b[1662])) {
            s.store_sub_from_scalar(1657, 1.0, 1652);
        }

        s.b[1670] = (p.p1601 != 1.0);
        s.store_scalar(1670, if s.b[1670] { 1.0 } else { 0.0 });

        s.b[1671] = (p.p1601 == 0.5);
        s.store_scalar(1671, if s.b[1671] { 1.0 } else { 0.0 });

        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && (!s.b[1662])) && s.b[1670]) && s.b[1671]) {
            s.store_div_from_scalar_sqrt_ad(1658, 1.0, s.ad_value(1657));
        }

        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && (!s.b[1662])) && s.b[1670]) && (!s.b[1671])) {
            s.store_powf(1658, 1657, (-p.p1601));
        }

        if ((((s.b[1523] && s.b[1660]) && s.b[1661]) && (!s.b[1662])) && s.b[1670]) {
            s.store_mul_ad_affine_product_rhs(536, 274, s.ad_value(528), A::sub_from_scalar(1.0, A::mul(s.ad_value(1657), s.ad_value(1658))), 1.0 / ((1.0 - p.p1601)), 0.0);
        }

        if ((((s.b[1523] && s.b[1660]) && s.b[1661]) && (!s.b[1662])) && (!s.b[1670])) {
            s.store_mul_ad_affine_product_rhs(536, 274, s.ad_value(528), {
                if (!(s.v[1657] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1657] > 1e-38) {
                            A::ln(s.ad_value(1657))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }

        s.b[1672] = (p.p1601 != 1.0);
        s.store_scalar(1672, if s.b[1672] { 1.0 } else { 0.0 });

        s.b[1673] = (p.p1601 == 0.5);
        s.store_scalar(1673, if s.b[1673] { 1.0 } else { 0.0 });

        if ((((s.b[1523] && s.b[1660]) && (!s.b[1661])) && s.b[1672]) && s.b[1673]) {
            s.store_scalar(1653, (1.0 / ((0.1) as f64).sqrt()));
        }

        if ((((s.b[1523] && s.b[1660]) && (!s.b[1661])) && s.b[1672]) && (!s.b[1673])) {
            s.store_scalar(1653, ((0.1) as f64).powf((-p.p1601)));
        }

        if (((s.b[1523] && s.b[1660]) && (!s.b[1661])) && s.b[1672]) {
            s.store_scalar(1654, (1.0 / (1.0 - p.p1601)));
            s.store_mul_sub_from_scalar_ad_rhs(1656, 1654, 1.0, A::scale(s.ad_value(1653), ((0.05 * p.p1601) * (1.0 + p.p1601))));
        }

        if (((s.b[1523] && s.b[1660]) && (!s.b[1661])) && (!s.b[1672])) {
            s.store_scalar(1653, 10.0);
            s.store_scalar(1656, (1.5 - ((0.1) as f64).ln()));
        }

        if ((s.b[1523] && s.b[1660]) && (!s.b[1661])) {
            s.store_mul_ad_product_rhs(1655, 1653, A::offset(s.ad_value(1652), (-1.0)), A::scale_offset(s.ad_value(1652), (5.0 * p.p1601), (((((-1.0)) * ((5.0 * p.p1601)))) + ((1.0 + p.p1601)))));
            s.store_mul_ad_product_rhs_mixed_ia(536, 274, 528, A::add(s.ad_value(1655), s.ad_value(1656)));
        }

        if (s.b[1523] && (!s.b[1660])) {
            s.store_scalar(536, 0.0);
        }

        if s.b[1523] {
            s.store_add_scaled_inputs3_indices(533, 534, 1.0, 535, 1.0, 536, 1.0);
        }

        s.store_add_scaled_inputs(507, 529, 1.0, 521, s.v[515]);

        s.store_add_scaled_inputs(508, 533, 1.0, 522, s.v[516]);

        s.store_mul_ad_product_rhs_mixed_ia(509, 517, 114, A::voltage(ctx, nodes, Some(3), Some(10)));

        s.b[1674] = (p.p61 != 0.0);
        s.store_scalar(1674, if s.b[1674] { 1.0 } else { 0.0 });

        if s.b[1674] {
            s.store_mul_voltage_ad(170, s.ad_value(114), ctx, nodes, Some(10), Some(3));
            s.store_add_scaled_inputs4_offset_indices(171, 170, 1.0, 167, (-1.0), 146, 0.5, 166, 1.0, (-p.p1529));
            s.store_offset(168, 171, 0.02);
            s.store_scaled_add_sqrt_square_offset_rhs(512, 168, 168, (4.0 * 0.02), 0.5);
            s.store_sub_ad_rhs(509, 509, A::mul3_scaled_output(s.ad_value(156), s.ad_value(650), A::add_scaled_inputs_product(s.ad_value(171), 1.0, s.ad_value(512), (-1.0), s.ad_value(653), A::offset(A::sqrt(A::offset(A::div_scaled_inputs(s.ad_value(512), 4.0, s.ad_value(653), 1.0), 1.0)), (-1.0)), 0.5), s.v[115]));
        }

        s.store_mul_add_ad_rhs(169, 126, s.ad_value(865), A::mul3(s.ad_value(866), s.ad_value(126), s.ad_value(126)));

        s.store_div_scaled_product3_indices(168, 415, 372, 158, 1.0, 153, 1.0);

        s.store_div_scaled_inputs_indices(579, 428, 2.0, 415, 1.0);

        s.b[1678] = (((p.p1682 > 0.0) || (p.p1683 > 0.0)) || (p.p1684 > 0.0));
        s.store_scalar(1678, if s.b[1678] { 1.0 } else { 0.0 });

        if s.b[1678] {
            s.store_offset(580, 153, (-(2.0 * p.p1687)));
        }

        s.b[1679] = (s.v[580] <= 0.0);
        s.store_scalar(1679, if s.b[1679] { 1.0 } else { 0.0 });

        if (s.b[1678] && s.b[1679]) {
            s.copy_ad(580, 153);
        }

        s.b[1680] = ((p.p79 == 1.0) || (p.p79 == 0.0));
        s.store_scalar(1680, if s.b[1680] { 1.0 } else { 0.0 });

        if (s.b[1678] && s.b[1680]) {
            s.store_square(581, 580);
        }

        s.b[1681] = (p.p1681 > 0.0);
        s.store_scalar(1681, if s.b[1681] { 1.0 } else { 0.0 });

        if ((s.b[1678] && s.b[1680]) && s.b[1681]) {
            s.store_div_scaled_offset_numerator(168, s.ad_value(202), 1.0 / (s.v[578]), p.p1681, s.ad_value(579), 1.0);
        }

        if ((s.b[1678] && s.b[1680]) && s.b[1681]) {
            s.store_scale_ad(582, {
                if (!(s.v[168] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[168] > 1e-38) {
                            A::ln(s.ad_value(168))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.v[578]);
        }

        if ((s.b[1678] && s.b[1680]) && (!s.b[1681])) {
            s.store_scalar(582, 0.0);
        }

        s.b[1682] = (p.p79 == 1.0);
        s.store_scalar(1682, if s.b[1682] { 1.0 } else { 0.0 });

        if ((s.b[1678] && s.b[1680]) && s.b[1682]) {
            s.store_div(169, 400, 576);
            s.store_offset_pow_ad(170, s.ad_value(169), s.ad_value(575), 1.0);
            s.store_div(171, 574, 170);
            s.store_scale(172, 171, 1.0 / (p.p1682));
            s.store_scaled_add_offset_sqrt_square_offset(174, 172, 1.0, (-1.0), ((0.25 * p.p1688) * p.p1688), 0.5);
            s.store_scale(573, 174, p.p1682);
        }

        if ((s.b[1678] && s.b[1680]) && (!s.b[1682])) {
            s.store_scalar(573, p.p1682);
        }

        if (s.b[1678] && s.b[1680]) {
            s.store_mul_ad_affine_product_lhs(169, s.ad_value(179), A::abs(s.ad_value(124)), ((1.60219e-19 * 1.60219e-19) * 1.60219e-19), 0.0, 415);
            s.store_scaled_mul(170, 372, 581, 10000000000.0);
            s.store_scaled_mul(583, 372, 392, 6.241457005723417e18);
            s.store_scaled_mul(584, 372, 393, 6.241457005723417e18);
            s.store_mul_add_scaled_inputs_rhs(585, 179, s.ad_value(372), 1.0 / (1.60219e-19), s.ad_value(669), 1.0 / (1.60219e-19));
        }

        if (s.b[1678] && s.b[1680]) {
            s.store_mul_ad_rhs(171, 573, {
                if (!(((s.v[583] + s.v[585]) / (s.v[584] + s.v[585])) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((s.v[583] + s.v[585]) / (s.v[584] + s.v[585])) > 1e-38) {
                            A::ln(A::div_scaled_inputs2(s.ad_value(583), 1.0, s.ad_value(585), 1.0, A::add(s.ad_value(584), s.ad_value(585)), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.b[1678] && s.b[1680]) {
            s.store_scaled_sub(172, 583, 584, p.p1683);
            s.store_scaled_sub_ad(174, A::square(s.ad_value(583)), A::square(s.ad_value(584)), (0.5 * p.p1684));
            s.store_mul3_affine_lhs(175, 179, 124, 1.60219e-19, 0.0, 124);
            s.store_scaled_mul(176, 581, 158, (10000000000.0 * s.v[115]));
            s.store_add_scaled_inputs_product_indices(177, 573, 1.0, 584, p.p1683, 584, 584, p.p1684);
            s.store_square_ad(178, A::add(s.ad_value(584), s.ad_value(585)));
            s.store_add_scaled_product(586, A::div_scaled_product3_by_product(s.ad_value(175), s.ad_value(582), s.ad_value(177), 1.0, s.ad_value(176), s.ad_value(178), 1.0), 1.0, A::div(s.ad_value(169), s.ad_value(170)), A::add_scaled_inputs3(s.ad_value(171), 1.0, s.ad_value(172), 1.0, s.ad_value(174), 1.0), 1.0);
            s.store_scaled_mul(340, 573, 179, 1.60219e-19);
            s.store_mul_product3_indices(341, 585, 158, 580, 585, (s.v[115] * 10000000000.0));
            s.store_mul_ad_product_lhs_mixed_ai(587, A::div(s.ad_value(340), s.ad_value(341)), 124, 124);
            s.store_add(169, 587, 586);
        }

        s.b[1684] = (p.p79 == 2.0);
        s.store_scalar(1684, if s.b[1684] { 1.0 } else { 0.0 });

        if ((s.b[1678] && (!s.b[1680])) && s.b[1684]) {
            s.store_div(169, 400, 576);
            s.store_offset_pow_ad(170, s.ad_value(169), s.ad_value(575), 1.0);
            s.store_div(171, 574, 170);
            s.store_scale(172, 171, 1.0 / (p.p1682));
            s.store_scaled_add_offset_sqrt_square_offset(174, 172, 1.0, (-1.0), ((0.25 * p.p1688) * p.p1688), 0.5);
            s.store_scale(573, 174, p.p1682);
            s.store_div_scaled_inputs_indices(589, 179, 2.0, 217, 1.0);
            s.store_offset_mul(169, 589, 402, 1.0);
            s.store_offset_scaled(170, 402, p.p1685, 1.0);
        }

        s.b[1685] = ((s.v[169] > 0.0) && (s.v[170] > 0.0));
        s.store_scalar(1685, if s.b[1685] { 1.0 } else { 0.0 });

        if (((s.b[1678] && (!s.b[1680])) && s.b[1684]) && s.b[1685]) {
            s.store_mul_offset_rhs_ad(171, {
                if (!(((s.v[392] + 0.5) / (s.v[393] + 0.5)) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((s.v[392] + 0.5) / (s.v[393] + 0.5)) > 1e-38) {
                            A::ln(A::div_scaled_offset_numerator(s.ad_value(392), 1.0, 0.5, A::offset(s.ad_value(393), 0.5), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, A::add(s.ad_value(392), s.ad_value(393)), 1.0);
        }

        if (((s.b[1678] && (!s.b[1680])) && s.b[1684]) && s.b[1685]) {
            s.store_scaled_sub(172, 392, 393, 2.0);
        }

        s.b[1686] = (p.p72 == 0.0);
        s.store_scalar(1686, if s.b[1686] { 1.0 } else { 0.0 });

        s.b[1687] = (p.p72 == 1.0);
        s.store_scalar(1687, if s.b[1687] { 1.0 } else { 0.0 });

        if s.b[1686] {
            s.store_mul(168, 415, 592);
            s.store_add_scaled_square_product_indices(169, 153, 1.0, 168, 197, 1.0);
        }

        if (s.b[1687] && (!s.b[1686])) {
            s.store_div(168, 399, 217);
            s.store_square(168, 168);
            s.store_scaled_offset_ad(597, A::mul_scaled_lhs(s.ad_value(168), p.p1709, s.ad_value(153)), 1.0, p.p1708);
            s.store_scaled_offset_ad(598, A::mul_scaled_lhs(s.ad_value(168), p.p1711, s.ad_value(153)), 1.0, p.p1710);
            s.store_scaled_offset_ad(599, A::mul_scaled_lhs(s.ad_value(168), p.p1713, s.ad_value(153)), 1.0, p.p1712);
            s.store_scaled_offset_ad(600, A::mul_scaled_lhs(s.ad_value(168), p.p1715, s.ad_value(153)), 1.0, p.p1714);
            s.store_scaled_mul(169, 597, 597, 3.0);
            s.store_scaled_mul(170, 598, 598, 7.5);
            s.store_scale(171, 599, 2.5298);
            s.store_mul_sub_from_scalar_rhs_ad(601, A::div(s.ad_value(393), s.ad_value(392)), 1.0, A::div(s.ad_value(390), s.ad_value(210)));
            s.store_mul_square_lhs(604, 209, 209);
            s.store_div_add_scaled_inputs_rhs_indices(602, 339, 339, 1.0, 399, 1.0);
            s.store_div_ad_rhs(172, 236, A::add_scaled_product(s.ad_value(181), 2.0, A::max_from_scalar(0.0, s.ad_value(237)), s.ad_value(392), 1.0));
            s.store_limited_exp_neg_input(616, 172);
        }

        s.b[1688] = (p.p61 == 2.0);
        s.store_scalar(1688, if s.b[1688] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_27(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[1687] && (!s.b[1686])) && s.b[1688]) {
            if (!(s.v[293] < ((-10000.0) * 1e-6))) {
                s.store_scaled_add_sqrt_square_offset_rhs(172, 293, 293, ((4.0 * 1e-6) * 1e-6), 0.5);
            } else {
                if (s.v[293] < ((-10000.0) * 1e-6)) {
                    s.store_div_from_scalar(172, ((-1e-6) * 1e-6), 293);
                } else {
                    s.store_scalar(172, 0.0);
                }
            }
        }

        if ((s.b[1687] && (!s.b[1686])) && s.b[1688]) {
            s.store_div_ad_rhs(174, 172, A::add_scaled_product(s.ad_value(181), 2.0, A::max_from_scalar(0.0, s.ad_value(238)), s.ad_value(392), 1.0));
            s.store_sub_ad(175, A::sqrt(A::sub(s.ad_value(689), s.ad_value(370))), A::sqrt(s.ad_value(689)));
            s.store_limited_exp_ad(617, A::mul_scaled_lhs(s.ad_value(174), -1.0, s.ad_value(175)));
        }

        if ((s.b[1687] && (!s.b[1686])) && (!s.b[1688])) {
            s.store_scalar(617, 1.0);
        }

        if (s.b[1687] && (!s.b[1686])) {
            s.store_add_scaled_product_indices(615, 401, s.v[420], 407, 392, s.v[420]);
            s.store_pow_ad(172, A::scaled_offset(A::abs(A::div(s.ad_value(392), s.ad_value(406))), 1.0, 0.5), s.ad_value(317));
        }

        s.b[1689] = (p.p61 != 0.0);
        s.store_scalar(1689, if s.b[1689] { 1.0 } else { 0.0 });

        if ((s.b[1687] && (!s.b[1686])) && s.b[1689]) {
            s.store_add_scaled_product(174, A::div(s.ad_value(820), s.ad_value(172)), 1.0, A::add_scaled_product(s.ad_value(819), 1.0, s.ad_value(821), s.ad_value(370), 1.0), A::pow(A::abs(s.ad_value(615)), s.ad_value(822)), 1.0);
        }

        if ((s.b[1687] && (!s.b[1686])) && (!s.b[1689])) {
            s.store_add_scaled_product_mixed_aia(174, A::div(s.ad_value(820), s.ad_value(172)), 1.0, 819, A::pow(A::abs(s.ad_value(615)), s.ad_value(822)), 1.0);
        }

        if (s.b[1687] && (!s.b[1686])) {
            s.store_offset(618, 174, 1.0);
            s.store_scaled_add_offset_sqrt_square_offset(618, 618, 1.0, (-1.0), ((0.25 * p.p604) * p.p604), 0.5);
            s.store_scale(618, 618, 1.0 / (p.p24));
            s.store_scalar(619, (1.0 + (0.25 * p.p453)));
            s.store_div_add_scaled_inputs_rhs_indices(612, 339, 339, 1.0, 392, 1.0);
            s.store_mul_sub_from_scalar_lhs(172, 2.0, 612, 181);
            s.store_add(613, 392, 172);
        }

        s.b[1690] = (p.p64 == 0.0);
        s.store_scalar(1690, if s.b[1690] { 1.0 } else { 0.0 });

        s.b[1691] = (p.p64 == 1.0);
        s.store_scalar(1691, if s.b[1691] { 1.0 } else { 0.0 });

        s.b[1692] = (p.p64 == 2.0);
        s.store_scalar(1692, if s.b[1692] { 1.0 } else { 0.0 });

        if ((s.b[1687] && (!s.b[1686])) && s.b[1690]) {
            s.store_offset_mul(172, 711, 392, 1.0);
            s.store_div_from_scalar(174, 1.0, 172);
            s.store_scaled_add_sqrt_square_offset_rhs(175, 174, 174, 0.01, 0.5);
            s.store_mul_ad_product_lhs_mixed_ia(614, 194, A::offset(A::mul(s.ad_value(709), s.ad_value(175)), p.p908), 189);
            s.store_offset_mul_ad(620, A::div_scaled_product_by_product(s.ad_value(183), s.ad_value(613), s.v[115], s.ad_value(618), s.ad_value(619), 1.0), s.ad_value(614), 1.0);
        }

        if ((s.b[1687] && (!s.b[1686])) && (s.b[1691] && (!s.b[1690]))) {
            s.store_scalar(620, 1.0);
        }

        if ((s.b[1687] && (!s.b[1686])) && (s.b[1692] && (!(s.b[1690] || s.b[1691])))) {
            s.store_offset_mul(172, 711, 392, 1.0);
            s.store_div_from_scalar(174, 1.0, 172);
            s.store_scaled_add_sqrt_square_offset_rhs(175, 174, 174, 0.01, 0.5);
            s.store_mul_offset_ad_lhs(614, A::mul(s.ad_value(709), s.ad_value(175)), p.p908, 189);
            s.store_mul_add_scaled_inputs3_offset_rhs(614, 194, s.ad_value(190), 1.0, s.ad_value(191), 1.0, s.ad_value(614), 1.0, 0.0);
            s.store_offset_mul_ad(620, A::div_scaled_product_by_product(s.ad_value(183), s.ad_value(613), s.v[115], s.ad_value(618), s.ad_value(619), 1.0), s.ad_value(614), 1.0);
        }

        if (s.b[1687] && (!s.b[1686])) {
            s.store_div_scaled_product_mixed_aia(603, A::mul3_scaled_output(s.ad_value(183), s.ad_value(392), s.ad_value(616), s.v[115]), 617, 1.0, A::mul3(s.ad_value(618), s.ad_value(619), s.ad_value(620)), 1.0);
            s.store_offset(172, 601, 1.0);
            s.store_sub_from_scalar(174, 1.0, 601);
            s.store_mul_div_scaled_inputs_indices(175, 181, 602, 2.0, 392, 1.0);
            s.store_add(176, 172, 175);
            s.store_square(605, 174);
            s.store_mul(606, 605, 174);
            s.store_mul(607, 606, 174);
            s.store_square(608, 176);
            s.store_mul(609, 608, 176);
            s.store_mul(610, 609, 176);
            s.store_mul(611, 610, 176);
            s.store_scale(621, 172, 0.5);
            s.store_div_scaled_inputs_indices(622, 605, 1.0, 176, 6.0);
            s.store_mul_div_scaled_inputs_mixed_aii(623, A::add(s.ad_value(621), s.ad_value(622)), 205, 1.0, 209, 1.0);
            s.store_div(624, 172, 608);
            s.store_div_scaled_product_left_ad(625, A::add_scaled_inputs(s.ad_value(172), 6.0, s.ad_value(175), 1.0), 605, 1.0, 610, 15.0);
            s.store_div_scaled_inputs_indices(626, 607, 1.0, 611, 9.0);
            s.store_mul_ad_affine_product_rhs(627, 205, s.ad_value(604), A::add_scaled_inputs3(s.ad_value(624), 1.0, s.ad_value(625), (-1.0), s.ad_value(626), 1.0), 1.0 / (6.0), 0.0);
            s.store_offset_mul_ad(177, A::div_scaled_product_offset_denominator(s.ad_value(600), s.ad_value(600), 1.0, s.ad_value(399), p.p1716, 1.0), A::div(s.ad_value(390), s.ad_value(210)), 1.0);
            s.store_mul_div_scaled_inputs_mixed_aii(623, A::add_scaled_products(s.ad_value(177), s.ad_value(621), 1.0, s.ad_value(169), s.ad_value(622), 1.0), 205, 1.0, 209, 1.0);
            s.store_mul_product3_mixed_aiii(627, A::add_scaled_inputs3(s.ad_value(624), 1.0, s.ad_value(625), (-1.0), s.ad_value(626), 1.0), 205, 604, 170, 1.0 / (6.0));
            s.store_div_scaled_product_left_ad(632, A::mul3_scaled_output(A::sqrt(A::div(s.ad_value(627), s.ad_value(623))), s.ad_value(372), s.ad_value(159), s.v[115]), 156, 1.0, 603, 1.0);
        }

        s.b[1696] = (p.p73 == 2.0);
        s.store_scalar(1696, if s.b[1696] { 1.0 } else { 0.0 });

        s.b[1705] = (p.p76 != 2.0);
        s.store_scalar(1705, if s.b[1705] { 1.0 } else { 0.0 });

        s.b[1706] = (p.p65 == 1.0);
        s.store_scalar(1706, if s.b[1706] { 1.0 } else { 0.0 });

        s.b[1707] = (p.p78 == 1.0);
        s.store_scalar(1707, if s.b[1707] { 1.0 } else { 0.0 });

        s.b[1708] = (p.p65 == 1.0);
        s.store_scalar(1708, if s.b[1708] { 1.0 } else { 0.0 });

        s.b[1709] = (p.p78 == 1.0);
        s.store_scalar(1709, if s.b[1709] { 1.0 } else { 0.0 });

        s.b[1710] = (p.p61 != 0.0);
        s.store_scalar(1710, if s.b[1710] { 1.0 } else { 0.0 });

        s.b[1711] = (p.p64 == 1.0);
        s.store_scalar(1711, if s.b[1711] { 1.0 } else { 0.0 });

        s.b[1712] = (p.p1910 > 0.0);
        s.store_scalar(1712, if s.b[1712] { 1.0 } else { 0.0 });

        if (s.b[1711] && s.b[1712]) {
            if (!(((1.0 + (p.p1912 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                s.store_scaled_add_sqrt_square_offset_ad(1039, A::scale_offset(s.ad_value(232), p.p1912, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001), 0.5);
            } else {
                if (((1.0 + (p.p1912 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                    s.store_div_from_scalar_offset_scaled_input(1039, ((-0.001) * 0.001), 232, p.p1912, ((1.0) + ((-1e-6))));
                } else {
                    s.store_scalar(1039, 0.0);
                }
            }
        }

        s.b[1713] = (p.p75 != 0.0);
        s.store_scalar(1713, if s.b[1713] { 1.0 } else { 0.0 });

        if ((s.b[1711] && s.b[1712]) && s.b[1713]) {
            s.store_offset_add_scaled_inputs(1044, A::scale_offset(s.ad_value(232), (-p.p1913), (((-(-p.p1904))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), (-p.p1913), (((-(-p.p1904))) + ((-1e-6)))), (-((4.0 * (-p.p1904)) * 1e-6))), 0.5, (((-p.p1904)) + (p.p1904)));
        }

        if ((s.b[1711] && s.b[1712]) && (!s.b[1713])) {
            s.store_scale_ad(1044, {
                if (!(((1.0 + ((-p.p1913) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), (-p.p1913), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), (-p.p1913), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + ((-p.p1913) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), (-p.p1913), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1904);
        }

        if (s.b[1711] && s.b[1712]) {
            s.store_offset(168, 392, (-p.p1906));
            s.store_scaled_add_offset_sqrt_square_offset(168, 168, 0.1, (-0.1), ((0.25 * 2.0) * 2.0), 0.5);
            s.store_div_scaled_value_offset_denominator(169, s.ad_value(168), (10.0 * p.p1907), s.ad_value(168), (10.0 * p.p1907), 1.0);
            s.store_mul_scale_offset_rhs(1045, 1044, 169, p.p1905, 1.0);
        }

        if (s.b[1711] && s.b[1712]) {
            if (!(s.v[1045] < ((-10000.0) * 10.0))) {
                s.store_scaled_add_sqrt_square_offset_rhs(1045, 1045, 1045, ((4.0 * 10.0) * 10.0), 0.5);
            } else {
                if (s.v[1045] < ((-10000.0) * 10.0)) {
                    s.store_div_from_scalar(1045, ((-10.0) * 10.0), 1045);
                } else {
                    s.store_scalar(1045, 0.0);
                }
            }
        }

        if (s.b[1711] && s.b[1712]) {
            s.store_scaled_mul(170, 158, 1045, (s.v[115] * 1.60219e-19));
            s.store_abs_voltage(174, ctx, nodes, Some(9), Some(7));
        }

        s.b[1714] = (p.p1917 == 0.0);
        s.store_scalar(1714, if s.b[1714] { 1.0 } else { 0.0 });

        if ((s.b[1711] && s.b[1712]) && s.b[1714]) {
            s.store_scalar(171, 1.0);
        }

        if ((s.b[1711] && s.b[1712]) && (!s.b[1714])) {
            s.store_scaled_add_sqrt_square_offset_ad(171, A::offset(s.ad_value(174), (-p.p1916)), ((0.25 * 0.5) * 0.5), 0.5);
            s.store_offset_scaled(171, 171, p.p1917, 1.0);
        }

        if (s.b[1711] && s.b[1712]) {
            s.store_scaled_mul(1047, 170, 171, p.p1903);
            s.store_scaled_mul(172, 1039, 189, p.p1910);
            s.store_mul(1048, 1047, 172);
        }

        if (s.b[1711] && s.b[1712]) {
            let assign34510_ad_e57399: A = A::powf(s.ad_value(174), (4.0 - p.p1908));
            s.store_div_ad(1050, assign34510_ad_e57399, A::add_scaled_inputs(assign34510_ad_e57399, 1.0, A::powf(s.ad_value(1048), (4.0 - p.p1908)), p.p1914));
        }

        if (s.b[1711] && s.b[1712]) {
            s.store_div_scaled_product_left_ad(175, A::powf(s.ad_value(1050), (1.0 / p.p1908)), 174, 1.0, 1048, 1.0);
        }

        s.b[1715] = (p.p1911 > 0.0);
        s.store_scalar(1715, if s.b[1715] { 1.0 } else { 0.0 });

        s.b[1716] = (p.p1910 == 0.0);
        s.store_scalar(1716, if s.b[1716] { 1.0 } else { 0.0 });

        if ((s.b[1711] && s.b[1715]) && s.b[1716]) {
            if (!(((1.0 + (p.p1912 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                s.store_scaled_add_sqrt_square_offset_ad(1039, A::scale_offset(s.ad_value(232), p.p1912, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001), 0.5);
            } else {
                if (((1.0 + (p.p1912 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                    s.store_div_from_scalar_offset_scaled_input(1039, ((-0.001) * 0.001), 232, p.p1912, ((1.0) + ((-1e-6))));
                } else {
                    s.store_scalar(1039, 0.0);
                }
            }
        }

        s.b[1717] = (p.p75 != 0.0);
        s.store_scalar(1717, if s.b[1717] { 1.0 } else { 0.0 });

        if (((s.b[1711] && s.b[1715]) && s.b[1716]) && s.b[1717]) {
            s.store_offset_add_scaled_inputs(1044, A::scale_offset(s.ad_value(232), (-p.p1913), (((-(-p.p1904))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), (-p.p1913), (((-(-p.p1904))) + ((-1e-6)))), (-((4.0 * (-p.p1904)) * 1e-6))), 0.5, (((-p.p1904)) + (p.p1904)));
        }

        if (((s.b[1711] && s.b[1715]) && s.b[1716]) && (!s.b[1717])) {
            s.store_scale_ad(1044, {
                if (!(((1.0 + ((-p.p1913) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), (-p.p1913), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), (-p.p1913), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + ((-p.p1913) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), (-p.p1913), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1904);
        }

        if ((s.b[1711] && s.b[1715]) && s.b[1716]) {
            s.store_offset(168, 392, (-p.p1906));
            s.store_scaled_add_offset_sqrt_square_offset(168, 168, 0.1, (-0.1), ((0.25 * 2.0) * 2.0), 0.5);
            s.store_div_scaled_value_offset_denominator(169, s.ad_value(168), (10.0 * p.p1907), s.ad_value(168), (10.0 * p.p1907), 1.0);
            s.store_mul_scale_offset_rhs(1045, 1044, 169, p.p1905, 1.0);
        }

        if ((s.b[1711] && s.b[1715]) && s.b[1716]) {
            if (!(s.v[1045] < ((-10000.0) * 10.0))) {
                s.store_scaled_add_sqrt_square_offset_rhs(1045, 1045, 1045, ((4.0 * 10.0) * 10.0), 0.5);
            } else {
                if (s.v[1045] < ((-10000.0) * 10.0)) {
                    s.store_div_from_scalar(1045, ((-10.0) * 10.0), 1045);
                } else {
                    s.store_scalar(1045, 0.0);
                }
            }
        }

        if ((s.b[1711] && s.b[1715]) && s.b[1716]) {
            s.store_scaled_mul(170, 158, 1045, (s.v[115] * 1.60219e-19));
        }

        if (s.b[1711] && s.b[1715]) {
            s.store_scale(1046, 170, p.p1909);
            s.store_scaled_mul(172, 1039, 189, p.p1911);
            s.store_mul(1049, 1046, 172);
            s.store_abs_voltage(174, ctx, nodes, Some(6), Some(8));
        }

        if (s.b[1711] && s.b[1715]) {
            let assign34700_ad_e57843: A = A::powf(s.ad_value(174), (4.0 - p.p1908));
            s.store_div_ad(1051, assign34700_ad_e57843, A::add_scaled_inputs(assign34700_ad_e57843, 1.0, A::powf(s.ad_value(1049), (4.0 - p.p1908)), p.p1915));
        }

        if (s.b[1711] && s.b[1715]) {
            s.store_div_scaled_product_left_ad(175, A::powf(s.ad_value(1051), (1.0 / p.p1908)), 174, 1.0, 1049, 1.0);
        }

        s.b[1723] = (p.p73 == 2.0);
        s.store_scalar(1723, if s.b[1723] { 1.0 } else { 0.0 });

        s.b[1731] = (p.p72 == 0.0);
        s.store_scalar(1731, if s.b[1731] { 1.0 } else { 0.0 });

        s.b[1736] = ((p.p74 != 0.0) && (p.p1791 > 0.0));
        s.store_scalar(1736, if s.b[1736] { 1.0 } else { 0.0 });

        s.store_add_scaled_ad_lhs(339, A::div_scaled_inputs(s.ad_value(179), 10.0, s.ad_value(898), 1.0), 396, 2.0);

        s.store_mul_add_rhs(169, 179, 179, 339);

        s.store_mul_square_lhs(170, 163, 169);

        s.store_scaled_mul(171, 141, 179, ((2.0 * 1.60219e-19) * s.v[143]));

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        var_devsign: f64,
        var_gtau: f64,
        var_gtau_dn0: f64,
        var_gtau_dn10: f64,
        var_gtau_dn11: f64,
        var_gtau_dn13: f64,
        var_gtau_dn14: f64,
        var_gtau_dn2: f64,
        var_gtau_dn3: f64,
        var_gtau_dn4: f64,
        var_gtau_dn5: f64,
        var_gtau_dn6: f64,
        var_gtau_dn7: f64,
        var_gtau_dn8: f64,
        var_gtau_dn9: f64,
        var_guard641: f64,
        var_guard642: f64,
        var_guard644: f64,
        var_guard645: f64,
        var_guard646: f64,
        var_ids_v: f64,
        var_ids_v_dn0: f64,
        var_ids_v_dn10: f64,
        var_ids_v_dn11: f64,
        var_ids_v_dn13: f64,
        var_ids_v_dn14: f64,
        var_ids_v_dn2: f64,
        var_ids_v_dn3: f64,
        var_ids_v_dn4: f64,
        var_ids_v_dn5: f64,
        var_ids_v_dn6: f64,
        var_ids_v_dn7: f64,
        var_ids_v_dn8: f64,
        var_ids_v_dn9: f64,
        var_idsgen_v: f64,
        var_idsgen_v_dn0: f64,
        var_idsgen_v_dn10: f64,
        var_idsgen_v_dn11: f64,
        var_idsgen_v_dn13: f64,
        var_idsgen_v_dn14: f64,
        var_idsgen_v_dn2: f64,
        var_idsgen_v_dn3: f64,
        var_idsgen_v_dn4: f64,
        var_idsgen_v_dn5: f64,
        var_idsgen_v_dn6: f64,
        var_idsgen_v_dn7: f64,
        var_idsgen_v_dn8: f64,
        var_idsgen_v_dn9: f64,
        var_igcd_v: f64,
        var_igcd_v_dn0: f64,
        var_igcd_v_dn10: f64,
        var_igcd_v_dn11: f64,
        var_igcd_v_dn13: f64,
        var_igcd_v_dn14: f64,
        var_igcd_v_dn2: f64,
        var_igcd_v_dn3: f64,
        var_igcd_v_dn4: f64,
        var_igcd_v_dn5: f64,
        var_igcd_v_dn6: f64,
        var_igcd_v_dn7: f64,
        var_igcd_v_dn8: f64,
        var_igcd_v_dn9: f64,
        var_igcs_v: f64,
        var_igcs_v_dn0: f64,
        var_igcs_v_dn10: f64,
        var_igcs_v_dn11: f64,
        var_igcs_v_dn13: f64,
        var_igcs_v_dn14: f64,
        var_igcs_v_dn2: f64,
        var_igcs_v_dn3: f64,
        var_igcs_v_dn4: f64,
        var_igcs_v_dn5: f64,
        var_igcs_v_dn6: f64,
        var_igcs_v_dn7: f64,
        var_igcs_v_dn8: f64,
        var_igcs_v_dn9: f64,
        var_igd_v: f64,
        var_igd_v_dn0: f64,
        var_igd_v_dn10: f64,
        var_igd_v_dn11: f64,
        var_igd_v_dn13: f64,
        var_igd_v_dn14: f64,
        var_igd_v_dn2: f64,
        var_igd_v_dn3: f64,
        var_igd_v_dn4: f64,
        var_igd_v_dn5: f64,
        var_igd_v_dn6: f64,
        var_igd_v_dn7: f64,
        var_igd_v_dn8: f64,
        var_igd_v_dn9: f64,
        var_igidl_v: f64,
        var_igidl_v_dn0: f64,
        var_igidl_v_dn10: f64,
        var_igidl_v_dn11: f64,
        var_igidl_v_dn13: f64,
        var_igidl_v_dn14: f64,
        var_igidl_v_dn2: f64,
        var_igidl_v_dn3: f64,
        var_igidl_v_dn4: f64,
        var_igidl_v_dn5: f64,
        var_igidl_v_dn6: f64,
        var_igidl_v_dn7: f64,
        var_igidl_v_dn8: f64,
        var_igidl_v_dn9: f64,
        var_igisl_v: f64,
        var_igisl_v_dn0: f64,
        var_igisl_v_dn10: f64,
        var_igisl_v_dn11: f64,
        var_igisl_v_dn13: f64,
        var_igisl_v_dn14: f64,
        var_igisl_v_dn2: f64,
        var_igisl_v_dn3: f64,
        var_igisl_v_dn4: f64,
        var_igisl_v_dn5: f64,
        var_igisl_v_dn6: f64,
        var_igisl_v_dn7: f64,
        var_igisl_v_dn8: f64,
        var_igisl_v_dn9: f64,
        var_igs_v: f64,
        var_igs_v_dn0: f64,
        var_igs_v_dn10: f64,
        var_igs_v_dn11: f64,
        var_igs_v_dn13: f64,
        var_igs_v_dn14: f64,
        var_igs_v_dn2: f64,
        var_igs_v_dn3: f64,
        var_igs_v_dn4: f64,
        var_igs_v_dn5: f64,
        var_igs_v_dn6: f64,
        var_igs_v_dn7: f64,
        var_igs_v_dn8: f64,
        var_igs_v_dn9: f64,
        var_iii_1: f64,
        var_iii_1_dn0: f64,
        var_iii_1_dn10: f64,
        var_iii_1_dn11: f64,
        var_iii_1_dn13: f64,
        var_iii_1_dn14: f64,
        var_iii_1_dn2: f64,
        var_iii_1_dn3: f64,
        var_iii_1_dn4: f64,
        var_iii_1_dn5: f64,
        var_iii_1_dn6: f64,
        var_iii_1_dn7: f64,
        var_iii_1_dn8: f64,
        var_iii_1_dn9: f64,
        var_qd_v: f64,
        var_qd_v_dn0: f64,
        var_qd_v_dn10: f64,
        var_qd_v_dn11: f64,
        var_qd_v_dn13: f64,
        var_qd_v_dn14: f64,
        var_qd_v_dn2: f64,
        var_qd_v_dn3: f64,
        var_qd_v_dn4: f64,
        var_qd_v_dn5: f64,
        var_qd_v_dn6: f64,
        var_qd_v_dn7: f64,
        var_qd_v_dn8: f64,
        var_qd_v_dn9: f64,
        var_qg_v: f64,
        var_qg_v_dn0: f64,
        var_qg_v_dn10: f64,
        var_qg_v_dn11: f64,
        var_qg_v_dn13: f64,
        var_qg_v_dn14: f64,
        var_qg_v_dn2: f64,
        var_qg_v_dn3: f64,
        var_qg_v_dn4: f64,
        var_qg_v_dn5: f64,
        var_qg_v_dn6: f64,
        var_qg_v_dn7: f64,
        var_qg_v_dn8: f64,
        var_qg_v_dn9: f64,
        var_xdpart: f64,
        var_xdpart_dn0: f64,
        var_xdpart_dn10: f64,
        var_xdpart_dn11: f64,
        var_xdpart_dn13: f64,
        var_xdpart_dn14: f64,
        var_xdpart_dn2: f64,
        var_xdpart_dn3: f64,
        var_xdpart_dn4: f64,
        var_xdpart_dn5: f64,
        var_xdpart_dn6: f64,
        var_xdpart_dn7: f64,
        var_xdpart_dn8: f64,
        var_xdpart_dn9: f64,
    ) {
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq0_e1945, eq0_e1945_d_n0, eq0_e1945_d_n2, eq0_e1945_d_n3, eq0_e1945_d_n4, eq0_e1945_d_n5, eq0_e1945_d_n6, eq0_e1945_d_n7, eq0_e1945_d_n8, eq0_e1945_d_n9, eq0_e1945_d_n10, eq0_e1945_d_n11, eq0_e1945_d_n13, eq0_e1945_d_n14,) = {
    if (var_guard641 != 0.0) {
        let eq0_e1943: f64 = (var_devsign * var_ids_v);
        let eq0_e1943_d_n0: f64 = (var_devsign * var_ids_v_dn0);
        let eq0_e1943_d_n2: f64 = (var_devsign * var_ids_v_dn2);
        let eq0_e1943_d_n3: f64 = (var_devsign * var_ids_v_dn3);
        let eq0_e1943_d_n4: f64 = (var_devsign * var_ids_v_dn4);
        let eq0_e1943_d_n5: f64 = (var_devsign * var_ids_v_dn5);
        let eq0_e1943_d_n6: f64 = (var_devsign * var_ids_v_dn6);
        let eq0_e1943_d_n7: f64 = (var_devsign * var_ids_v_dn7);
        let eq0_e1943_d_n8: f64 = (var_devsign * var_ids_v_dn8);
        let eq0_e1943_d_n9: f64 = (var_devsign * var_ids_v_dn9);
        let eq0_e1943_d_n10: f64 = (var_devsign * var_ids_v_dn10);
        let eq0_e1943_d_n11: f64 = (var_devsign * var_ids_v_dn11);
        let eq0_e1943_d_n13: f64 = (var_devsign * var_ids_v_dn13);
        let eq0_e1943_d_n14: f64 = (var_devsign * var_ids_v_dn14);
        (eq0_e1943, eq0_e1943_d_n0, eq0_e1943_d_n2, eq0_e1943_d_n3, eq0_e1943_d_n4, eq0_e1943_d_n5, eq0_e1943_d_n6, eq0_e1943_d_n7, eq0_e1943_d_n8, eq0_e1943_d_n9, eq0_e1943_d_n10, eq0_e1943_d_n11, eq0_e1943_d_n13, eq0_e1943_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e1945;
        let eq0_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq0_node_derivatives: [f64; 13] = [eq0_e1945_d_n0, eq0_e1945_d_n2, eq0_e1945_d_n3, eq0_e1945_d_n4, eq0_e1945_d_n5, eq0_e1945_d_n6, eq0_e1945_d_n7, eq0_e1945_d_n8, eq0_e1945_d_n9, eq0_e1945_d_n10, eq0_e1945_d_n11, eq0_e1945_d_n13, eq0_e1945_d_n14];
        let eq0_branch_derivative_indices: [usize; 0] = [];
        let eq0_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq0_value),
            &eq0_node_derivative_indices,
            &eq0_node_derivatives,
            &eq0_branch_derivative_indices,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let (eq1_e1952, eq1_e1952_d_n0, eq1_e1952_d_n2, eq1_e1952_d_n3, eq1_e1952_d_n4, eq1_e1952_d_n5, eq1_e1952_d_n6, eq1_e1952_d_n7, eq1_e1952_d_n8, eq1_e1952_d_n9, eq1_e1952_d_n10, eq1_e1952_d_n11, eq1_e1952_d_n13, eq1_e1952_d_n14,) = {
    if (var_guard641 == 0.0) {
        let eq1_e1950: f64 = (var_devsign * var_ids_v);
        let eq1_e1950_d_n0: f64 = (var_devsign * var_ids_v_dn0);
        let eq1_e1950_d_n2: f64 = (var_devsign * var_ids_v_dn2);
        let eq1_e1950_d_n3: f64 = (var_devsign * var_ids_v_dn3);
        let eq1_e1950_d_n4: f64 = (var_devsign * var_ids_v_dn4);
        let eq1_e1950_d_n5: f64 = (var_devsign * var_ids_v_dn5);
        let eq1_e1950_d_n6: f64 = (var_devsign * var_ids_v_dn6);
        let eq1_e1950_d_n7: f64 = (var_devsign * var_ids_v_dn7);
        let eq1_e1950_d_n8: f64 = (var_devsign * var_ids_v_dn8);
        let eq1_e1950_d_n9: f64 = (var_devsign * var_ids_v_dn9);
        let eq1_e1950_d_n10: f64 = (var_devsign * var_ids_v_dn10);
        let eq1_e1950_d_n11: f64 = (var_devsign * var_ids_v_dn11);
        let eq1_e1950_d_n13: f64 = (var_devsign * var_ids_v_dn13);
        let eq1_e1950_d_n14: f64 = (var_devsign * var_ids_v_dn14);
        (eq1_e1950, eq1_e1950_d_n0, eq1_e1950_d_n2, eq1_e1950_d_n3, eq1_e1950_d_n4, eq1_e1950_d_n5, eq1_e1950_d_n6, eq1_e1950_d_n7, eq1_e1950_d_n8, eq1_e1950_d_n9, eq1_e1950_d_n10, eq1_e1950_d_n11, eq1_e1950_d_n13, eq1_e1950_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e1952;
        let eq1_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq1_node_derivatives: [f64; 13] = [eq1_e1952_d_n0, eq1_e1952_d_n2, eq1_e1952_d_n3, eq1_e1952_d_n4, eq1_e1952_d_n5, eq1_e1952_d_n6, eq1_e1952_d_n7, eq1_e1952_d_n8, eq1_e1952_d_n9, eq1_e1952_d_n10, eq1_e1952_d_n11, eq1_e1952_d_n13, eq1_e1952_d_n14];
        let eq1_branch_derivative_indices: [usize; 0] = [];
        let eq1_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq1_value),
            &eq1_node_derivative_indices,
            &eq1_node_derivatives,
            &eq1_branch_derivative_indices,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let (eq2_e1961, eq2_e1961_d_n0, eq2_e1961_d_n2, eq2_e1961_d_n3, eq2_e1961_d_n4, eq2_e1961_d_n5, eq2_e1961_d_n6, eq2_e1961_d_n7, eq2_e1961_d_n8, eq2_e1961_d_n9, eq2_e1961_d_n10, eq2_e1961_d_n11, eq2_e1961_d_n13, eq2_e1961_d_n14, eq2_e1961_d_n15,) = {
    if (var_guard642 != 0.0) {
        let eq2_e1956: f64 = (var_devsign * var_gtau);
        let eq2_e1956_d_n0: f64 = (var_devsign * var_gtau_dn0);
        let eq2_e1956_d_n2: f64 = (var_devsign * var_gtau_dn2);
        let eq2_e1956_d_n3: f64 = (var_devsign * var_gtau_dn3);
        let eq2_e1956_d_n4: f64 = (var_devsign * var_gtau_dn4);
        let eq2_e1956_d_n5: f64 = (var_devsign * var_gtau_dn5);
        let eq2_e1956_d_n6: f64 = (var_devsign * var_gtau_dn6);
        let eq2_e1956_d_n7: f64 = (var_devsign * var_gtau_dn7);
        let eq2_e1956_d_n8: f64 = (var_devsign * var_gtau_dn8);
        let eq2_e1956_d_n9: f64 = (var_devsign * var_gtau_dn9);
        let eq2_e1956_d_n10: f64 = (var_devsign * var_gtau_dn10);
        let eq2_e1956_d_n11: f64 = (var_devsign * var_gtau_dn11);
        let eq2_e1956_d_n13: f64 = (var_devsign * var_gtau_dn13);
        let eq2_e1956_d_n14: f64 = (var_devsign * var_gtau_dn14);
        let eq2_e1958: f64 = (-(nv15 - 0.0));
        let eq2_e1959: f64 = (eq2_e1956 * eq2_e1958);
        let eq2_e1959_d_n0: f64 = (eq2_e1956_d_n0 * eq2_e1958);
        let eq2_e1959_d_n2: f64 = (eq2_e1956_d_n2 * eq2_e1958);
        let eq2_e1959_d_n3: f64 = (eq2_e1956_d_n3 * eq2_e1958);
        let eq2_e1959_d_n4: f64 = (eq2_e1956_d_n4 * eq2_e1958);
        let eq2_e1959_d_n5: f64 = (eq2_e1956_d_n5 * eq2_e1958);
        let eq2_e1959_d_n6: f64 = (eq2_e1956_d_n6 * eq2_e1958);
        let eq2_e1959_d_n7: f64 = (eq2_e1956_d_n7 * eq2_e1958);
        let eq2_e1959_d_n8: f64 = (eq2_e1956_d_n8 * eq2_e1958);
        let eq2_e1959_d_n9: f64 = (eq2_e1956_d_n9 * eq2_e1958);
        let eq2_e1959_d_n10: f64 = (eq2_e1956_d_n10 * eq2_e1958);
        let eq2_e1959_d_n11: f64 = (eq2_e1956_d_n11 * eq2_e1958);
        let eq2_e1959_d_n13: f64 = (eq2_e1956_d_n13 * eq2_e1958);
        let eq2_e1959_d_n14: f64 = (eq2_e1956_d_n14 * eq2_e1958);
        let eq2_e1959_d_n15: f64 = (eq2_e1956 * (-1.0));
        (eq2_e1959, eq2_e1959_d_n0, eq2_e1959_d_n2, eq2_e1959_d_n3, eq2_e1959_d_n4, eq2_e1959_d_n5, eq2_e1959_d_n6, eq2_e1959_d_n7, eq2_e1959_d_n8, eq2_e1959_d_n9, eq2_e1959_d_n10, eq2_e1959_d_n11, eq2_e1959_d_n13, eq2_e1959_d_n14, eq2_e1959_d_n15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e1961;
        let eq2_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15];
        let eq2_node_derivatives: [f64; 14] = [eq2_e1961_d_n0, eq2_e1961_d_n2, eq2_e1961_d_n3, eq2_e1961_d_n4, eq2_e1961_d_n5, eq2_e1961_d_n6, eq2_e1961_d_n7, eq2_e1961_d_n8, eq2_e1961_d_n9, eq2_e1961_d_n10, eq2_e1961_d_n11, eq2_e1961_d_n13, eq2_e1961_d_n14, eq2_e1961_d_n15];
        let eq2_branch_derivative_indices: [usize; 0] = [];
        let eq2_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq2_value),
            &eq2_node_derivative_indices,
            &eq2_node_derivatives,
            &eq2_branch_derivative_indices,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let (eq3_e1971, eq3_e1971_d_n0, eq3_e1971_d_n2, eq3_e1971_d_n3, eq3_e1971_d_n4, eq3_e1971_d_n5, eq3_e1971_d_n6, eq3_e1971_d_n7, eq3_e1971_d_n8, eq3_e1971_d_n9, eq3_e1971_d_n10, eq3_e1971_d_n11, eq3_e1971_d_n13, eq3_e1971_d_n14, eq3_e1971_d_n15,) = {
    if (var_guard642 != 0.0) {
        let eq3_e1965: f64 = (var_devsign * var_xdpart);
        let eq3_e1965_d_n0: f64 = (var_devsign * var_xdpart_dn0);
        let eq3_e1965_d_n2: f64 = (var_devsign * var_xdpart_dn2);
        let eq3_e1965_d_n3: f64 = (var_devsign * var_xdpart_dn3);
        let eq3_e1965_d_n4: f64 = (var_devsign * var_xdpart_dn4);
        let eq3_e1965_d_n5: f64 = (var_devsign * var_xdpart_dn5);
        let eq3_e1965_d_n6: f64 = (var_devsign * var_xdpart_dn6);
        let eq3_e1965_d_n7: f64 = (var_devsign * var_xdpart_dn7);
        let eq3_e1965_d_n8: f64 = (var_devsign * var_xdpart_dn8);
        let eq3_e1965_d_n9: f64 = (var_devsign * var_xdpart_dn9);
        let eq3_e1965_d_n10: f64 = (var_devsign * var_xdpart_dn10);
        let eq3_e1965_d_n11: f64 = (var_devsign * var_xdpart_dn11);
        let eq3_e1965_d_n13: f64 = (var_devsign * var_xdpart_dn13);
        let eq3_e1965_d_n14: f64 = (var_devsign * var_xdpart_dn14);
        let eq3_e1967: f64 = (eq3_e1965 * var_gtau);
        let eq3_e1967_d_n0: f64 = ((eq3_e1965_d_n0 * var_gtau) + (eq3_e1965 * var_gtau_dn0));
        let eq3_e1967_d_n2: f64 = ((eq3_e1965_d_n2 * var_gtau) + (eq3_e1965 * var_gtau_dn2));
        let eq3_e1967_d_n3: f64 = ((eq3_e1965_d_n3 * var_gtau) + (eq3_e1965 * var_gtau_dn3));
        let eq3_e1967_d_n4: f64 = ((eq3_e1965_d_n4 * var_gtau) + (eq3_e1965 * var_gtau_dn4));
        let eq3_e1967_d_n5: f64 = ((eq3_e1965_d_n5 * var_gtau) + (eq3_e1965 * var_gtau_dn5));
        let eq3_e1967_d_n6: f64 = ((eq3_e1965_d_n6 * var_gtau) + (eq3_e1965 * var_gtau_dn6));
        let eq3_e1967_d_n7: f64 = ((eq3_e1965_d_n7 * var_gtau) + (eq3_e1965 * var_gtau_dn7));
        let eq3_e1967_d_n8: f64 = ((eq3_e1965_d_n8 * var_gtau) + (eq3_e1965 * var_gtau_dn8));
        let eq3_e1967_d_n9: f64 = ((eq3_e1965_d_n9 * var_gtau) + (eq3_e1965 * var_gtau_dn9));
        let eq3_e1967_d_n10: f64 = ((eq3_e1965_d_n10 * var_gtau) + (eq3_e1965 * var_gtau_dn10));
        let eq3_e1967_d_n11: f64 = ((eq3_e1965_d_n11 * var_gtau) + (eq3_e1965 * var_gtau_dn11));
        let eq3_e1967_d_n13: f64 = ((eq3_e1965_d_n13 * var_gtau) + (eq3_e1965 * var_gtau_dn13));
        let eq3_e1967_d_n14: f64 = ((eq3_e1965_d_n14 * var_gtau) + (eq3_e1965 * var_gtau_dn14));
        let eq3_e1969: f64 = (eq3_e1967 * (nv15 - 0.0));
        let eq3_e1969_d_n0: f64 = (eq3_e1967_d_n0 * (nv15 - 0.0));
        let eq3_e1969_d_n2: f64 = (eq3_e1967_d_n2 * (nv15 - 0.0));
        let eq3_e1969_d_n3: f64 = (eq3_e1967_d_n3 * (nv15 - 0.0));
        let eq3_e1969_d_n4: f64 = (eq3_e1967_d_n4 * (nv15 - 0.0));
        let eq3_e1969_d_n5: f64 = (eq3_e1967_d_n5 * (nv15 - 0.0));
        let eq3_e1969_d_n6: f64 = (eq3_e1967_d_n6 * (nv15 - 0.0));
        let eq3_e1969_d_n7: f64 = (eq3_e1967_d_n7 * (nv15 - 0.0));
        let eq3_e1969_d_n8: f64 = (eq3_e1967_d_n8 * (nv15 - 0.0));
        let eq3_e1969_d_n9: f64 = (eq3_e1967_d_n9 * (nv15 - 0.0));
        let eq3_e1969_d_n10: f64 = (eq3_e1967_d_n10 * (nv15 - 0.0));
        let eq3_e1969_d_n11: f64 = (eq3_e1967_d_n11 * (nv15 - 0.0));
        let eq3_e1969_d_n13: f64 = (eq3_e1967_d_n13 * (nv15 - 0.0));
        let eq3_e1969_d_n14: f64 = (eq3_e1967_d_n14 * (nv15 - 0.0));
        (eq3_e1969, eq3_e1969_d_n0, eq3_e1969_d_n2, eq3_e1969_d_n3, eq3_e1969_d_n4, eq3_e1969_d_n5, eq3_e1969_d_n6, eq3_e1969_d_n7, eq3_e1969_d_n8, eq3_e1969_d_n9, eq3_e1969_d_n10, eq3_e1969_d_n11, eq3_e1969_d_n13, eq3_e1969_d_n14, eq3_e1967,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e1971;
        let eq3_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15];
        let eq3_node_derivatives: [f64; 14] = [eq3_e1971_d_n0, eq3_e1971_d_n2, eq3_e1971_d_n3, eq3_e1971_d_n4, eq3_e1971_d_n5, eq3_e1971_d_n6, eq3_e1971_d_n7, eq3_e1971_d_n8, eq3_e1971_d_n9, eq3_e1971_d_n10, eq3_e1971_d_n11, eq3_e1971_d_n13, eq3_e1971_d_n14, eq3_e1971_d_n15];
        let eq3_branch_derivative_indices: [usize; 0] = [];
        let eq3_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq3_value),
            &eq3_node_derivative_indices,
            &eq3_node_derivatives,
            &eq3_branch_derivative_indices,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let (eq4_e1979, eq4_e1979_d_n0, eq4_e1979_d_n2, eq4_e1979_d_n3, eq4_e1979_d_n4, eq4_e1979_d_n5, eq4_e1979_d_n6, eq4_e1979_d_n7, eq4_e1979_d_n8, eq4_e1979_d_n9, eq4_e1979_d_n10, eq4_e1979_d_n11, eq4_e1979_d_n13, eq4_e1979_d_n14,) = {
    if (var_guard642 == 0.0) {
        let eq4_e1976: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, var_qd_v);
        let eq4_e1977: f64 = (var_devsign * eq4_e1976);
        let eq4_e1977_d_n0: f64 = (var_devsign * (var_qd_v_dn0 * ddt_scale));
        let eq4_e1977_d_n2: f64 = (var_devsign * (var_qd_v_dn2 * ddt_scale));
        let eq4_e1977_d_n3: f64 = (var_devsign * (var_qd_v_dn3 * ddt_scale));
        let eq4_e1977_d_n4: f64 = (var_devsign * (var_qd_v_dn4 * ddt_scale));
        let eq4_e1977_d_n5: f64 = (var_devsign * (var_qd_v_dn5 * ddt_scale));
        let eq4_e1977_d_n6: f64 = (var_devsign * (var_qd_v_dn6 * ddt_scale));
        let eq4_e1977_d_n7: f64 = (var_devsign * (var_qd_v_dn7 * ddt_scale));
        let eq4_e1977_d_n8: f64 = (var_devsign * (var_qd_v_dn8 * ddt_scale));
        let eq4_e1977_d_n9: f64 = (var_devsign * (var_qd_v_dn9 * ddt_scale));
        let eq4_e1977_d_n10: f64 = (var_devsign * (var_qd_v_dn10 * ddt_scale));
        let eq4_e1977_d_n11: f64 = (var_devsign * (var_qd_v_dn11 * ddt_scale));
        let eq4_e1977_d_n13: f64 = (var_devsign * (var_qd_v_dn13 * ddt_scale));
        let eq4_e1977_d_n14: f64 = (var_devsign * (var_qd_v_dn14 * ddt_scale));
        (eq4_e1977, eq4_e1977_d_n0, eq4_e1977_d_n2, eq4_e1977_d_n3, eq4_e1977_d_n4, eq4_e1977_d_n5, eq4_e1977_d_n6, eq4_e1977_d_n7, eq4_e1977_d_n8, eq4_e1977_d_n9, eq4_e1977_d_n10, eq4_e1977_d_n11, eq4_e1977_d_n13, eq4_e1977_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e1979;
        let eq4_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq4_node_derivatives: [f64; 13] = [eq4_e1979_d_n0, eq4_e1979_d_n2, eq4_e1979_d_n3, eq4_e1979_d_n4, eq4_e1979_d_n5, eq4_e1979_d_n6, eq4_e1979_d_n7, eq4_e1979_d_n8, eq4_e1979_d_n9, eq4_e1979_d_n10, eq4_e1979_d_n11, eq4_e1979_d_n13, eq4_e1979_d_n14];
        let eq4_branch_derivative_indices: [usize; 0] = [];
        let eq4_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq4_value),
            &eq4_node_derivative_indices,
            &eq4_node_derivatives,
            &eq4_branch_derivative_indices,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e1987, eq5_e1987_d_n0, eq5_e1987_d_n2, eq5_e1987_d_n3, eq5_e1987_d_n4, eq5_e1987_d_n5, eq5_e1987_d_n6, eq5_e1987_d_n7, eq5_e1987_d_n8, eq5_e1987_d_n9, eq5_e1987_d_n10, eq5_e1987_d_n11, eq5_e1987_d_n13, eq5_e1987_d_n14,) = {
    if (var_guard642 == 0.0) {
        let eq5_e1984: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, var_qg_v);
        let eq5_e1985: f64 = (var_devsign * eq5_e1984);
        let eq5_e1985_d_n0: f64 = (var_devsign * (var_qg_v_dn0 * ddt_scale));
        let eq5_e1985_d_n2: f64 = (var_devsign * (var_qg_v_dn2 * ddt_scale));
        let eq5_e1985_d_n3: f64 = (var_devsign * (var_qg_v_dn3 * ddt_scale));
        let eq5_e1985_d_n4: f64 = (var_devsign * (var_qg_v_dn4 * ddt_scale));
        let eq5_e1985_d_n5: f64 = (var_devsign * (var_qg_v_dn5 * ddt_scale));
        let eq5_e1985_d_n6: f64 = (var_devsign * (var_qg_v_dn6 * ddt_scale));
        let eq5_e1985_d_n7: f64 = (var_devsign * (var_qg_v_dn7 * ddt_scale));
        let eq5_e1985_d_n8: f64 = (var_devsign * (var_qg_v_dn8 * ddt_scale));
        let eq5_e1985_d_n9: f64 = (var_devsign * (var_qg_v_dn9 * ddt_scale));
        let eq5_e1985_d_n10: f64 = (var_devsign * (var_qg_v_dn10 * ddt_scale));
        let eq5_e1985_d_n11: f64 = (var_devsign * (var_qg_v_dn11 * ddt_scale));
        let eq5_e1985_d_n13: f64 = (var_devsign * (var_qg_v_dn13 * ddt_scale));
        let eq5_e1985_d_n14: f64 = (var_devsign * (var_qg_v_dn14 * ddt_scale));
        (eq5_e1985, eq5_e1985_d_n0, eq5_e1985_d_n2, eq5_e1985_d_n3, eq5_e1985_d_n4, eq5_e1985_d_n5, eq5_e1985_d_n6, eq5_e1985_d_n7, eq5_e1985_d_n8, eq5_e1985_d_n9, eq5_e1985_d_n10, eq5_e1985_d_n11, eq5_e1985_d_n13, eq5_e1985_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e1987;
        let eq5_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq5_node_derivatives: [f64; 13] = [eq5_e1987_d_n0, eq5_e1987_d_n2, eq5_e1987_d_n3, eq5_e1987_d_n4, eq5_e1987_d_n5, eq5_e1987_d_n6, eq5_e1987_d_n7, eq5_e1987_d_n8, eq5_e1987_d_n9, eq5_e1987_d_n10, eq5_e1987_d_n11, eq5_e1987_d_n13, eq5_e1987_d_n14];
        let eq5_branch_derivative_indices: [usize; 0] = [];
        let eq5_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq5_value),
            &eq5_node_derivative_indices,
            &eq5_node_derivatives,
            &eq5_branch_derivative_indices,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let (eq8_e2004, eq8_e2004_d_n0, eq8_e2004_d_n2, eq8_e2004_d_n3, eq8_e2004_d_n4, eq8_e2004_d_n5, eq8_e2004_d_n6, eq8_e2004_d_n7, eq8_e2004_d_n8, eq8_e2004_d_n9, eq8_e2004_d_n10, eq8_e2004_d_n11, eq8_e2004_d_n13, eq8_e2004_d_n14,) = {
    if (var_guard644 != 0.0) {
        let eq8_e2002: f64 = (var_devsign * var_idsgen_v);
        let eq8_e2002_d_n0: f64 = (var_devsign * var_idsgen_v_dn0);
        let eq8_e2002_d_n2: f64 = (var_devsign * var_idsgen_v_dn2);
        let eq8_e2002_d_n3: f64 = (var_devsign * var_idsgen_v_dn3);
        let eq8_e2002_d_n4: f64 = (var_devsign * var_idsgen_v_dn4);
        let eq8_e2002_d_n5: f64 = (var_devsign * var_idsgen_v_dn5);
        let eq8_e2002_d_n6: f64 = (var_devsign * var_idsgen_v_dn6);
        let eq8_e2002_d_n7: f64 = (var_devsign * var_idsgen_v_dn7);
        let eq8_e2002_d_n8: f64 = (var_devsign * var_idsgen_v_dn8);
        let eq8_e2002_d_n9: f64 = (var_devsign * var_idsgen_v_dn9);
        let eq8_e2002_d_n10: f64 = (var_devsign * var_idsgen_v_dn10);
        let eq8_e2002_d_n11: f64 = (var_devsign * var_idsgen_v_dn11);
        let eq8_e2002_d_n13: f64 = (var_devsign * var_idsgen_v_dn13);
        let eq8_e2002_d_n14: f64 = (var_devsign * var_idsgen_v_dn14);
        (eq8_e2002, eq8_e2002_d_n0, eq8_e2002_d_n2, eq8_e2002_d_n3, eq8_e2002_d_n4, eq8_e2002_d_n5, eq8_e2002_d_n6, eq8_e2002_d_n7, eq8_e2002_d_n8, eq8_e2002_d_n9, eq8_e2002_d_n10, eq8_e2002_d_n11, eq8_e2002_d_n13, eq8_e2002_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e2004;
        let eq8_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq8_node_derivatives: [f64; 13] = [eq8_e2004_d_n0, eq8_e2004_d_n2, eq8_e2004_d_n3, eq8_e2004_d_n4, eq8_e2004_d_n5, eq8_e2004_d_n6, eq8_e2004_d_n7, eq8_e2004_d_n8, eq8_e2004_d_n9, eq8_e2004_d_n10, eq8_e2004_d_n11, eq8_e2004_d_n13, eq8_e2004_d_n14];
        let eq8_branch_derivative_indices: [usize; 0] = [];
        let eq8_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq8_value),
            &eq8_node_derivative_indices,
            &eq8_node_derivatives,
            &eq8_branch_derivative_indices,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let (eq9_e2012, eq9_e2012_d_n0, eq9_e2012_d_n2, eq9_e2012_d_n3, eq9_e2012_d_n4, eq9_e2012_d_n5, eq9_e2012_d_n6, eq9_e2012_d_n7, eq9_e2012_d_n8, eq9_e2012_d_n9, eq9_e2012_d_n10, eq9_e2012_d_n11, eq9_e2012_d_n13, eq9_e2012_d_n14,) = {
    if (var_guard644 != 0.0) {
        let eq9_e2009: f64 = (var_igcs_v + var_igs_v);
        let eq9_e2009_d_n0: f64 = (var_igcs_v_dn0 + var_igs_v_dn0);
        let eq9_e2009_d_n2: f64 = (var_igcs_v_dn2 + var_igs_v_dn2);
        let eq9_e2009_d_n3: f64 = (var_igcs_v_dn3 + var_igs_v_dn3);
        let eq9_e2009_d_n4: f64 = (var_igcs_v_dn4 + var_igs_v_dn4);
        let eq9_e2009_d_n5: f64 = (var_igcs_v_dn5 + var_igs_v_dn5);
        let eq9_e2009_d_n6: f64 = (var_igcs_v_dn6 + var_igs_v_dn6);
        let eq9_e2009_d_n7: f64 = (var_igcs_v_dn7 + var_igs_v_dn7);
        let eq9_e2009_d_n8: f64 = (var_igcs_v_dn8 + var_igs_v_dn8);
        let eq9_e2009_d_n9: f64 = (var_igcs_v_dn9 + var_igs_v_dn9);
        let eq9_e2009_d_n10: f64 = (var_igcs_v_dn10 + var_igs_v_dn10);
        let eq9_e2009_d_n11: f64 = (var_igcs_v_dn11 + var_igs_v_dn11);
        let eq9_e2009_d_n13: f64 = (var_igcs_v_dn13 + var_igs_v_dn13);
        let eq9_e2009_d_n14: f64 = (var_igcs_v_dn14 + var_igs_v_dn14);
        let eq9_e2010: f64 = (var_devsign * eq9_e2009);
        let eq9_e2010_d_n0: f64 = (var_devsign * eq9_e2009_d_n0);
        let eq9_e2010_d_n2: f64 = (var_devsign * eq9_e2009_d_n2);
        let eq9_e2010_d_n3: f64 = (var_devsign * eq9_e2009_d_n3);
        let eq9_e2010_d_n4: f64 = (var_devsign * eq9_e2009_d_n4);
        let eq9_e2010_d_n5: f64 = (var_devsign * eq9_e2009_d_n5);
        let eq9_e2010_d_n6: f64 = (var_devsign * eq9_e2009_d_n6);
        let eq9_e2010_d_n7: f64 = (var_devsign * eq9_e2009_d_n7);
        let eq9_e2010_d_n8: f64 = (var_devsign * eq9_e2009_d_n8);
        let eq9_e2010_d_n9: f64 = (var_devsign * eq9_e2009_d_n9);
        let eq9_e2010_d_n10: f64 = (var_devsign * eq9_e2009_d_n10);
        let eq9_e2010_d_n11: f64 = (var_devsign * eq9_e2009_d_n11);
        let eq9_e2010_d_n13: f64 = (var_devsign * eq9_e2009_d_n13);
        let eq9_e2010_d_n14: f64 = (var_devsign * eq9_e2009_d_n14);
        (eq9_e2010, eq9_e2010_d_n0, eq9_e2010_d_n2, eq9_e2010_d_n3, eq9_e2010_d_n4, eq9_e2010_d_n5, eq9_e2010_d_n6, eq9_e2010_d_n7, eq9_e2010_d_n8, eq9_e2010_d_n9, eq9_e2010_d_n10, eq9_e2010_d_n11, eq9_e2010_d_n13, eq9_e2010_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e2012;
        let eq9_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq9_node_derivatives: [f64; 13] = [eq9_e2012_d_n0, eq9_e2012_d_n2, eq9_e2012_d_n3, eq9_e2012_d_n4, eq9_e2012_d_n5, eq9_e2012_d_n6, eq9_e2012_d_n7, eq9_e2012_d_n8, eq9_e2012_d_n9, eq9_e2012_d_n10, eq9_e2012_d_n11, eq9_e2012_d_n13, eq9_e2012_d_n14];
        let eq9_branch_derivative_indices: [usize; 0] = [];
        let eq9_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq9_value),
            &eq9_node_derivative_indices,
            &eq9_node_derivatives,
            &eq9_branch_derivative_indices,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let (eq10_e2020, eq10_e2020_d_n0, eq10_e2020_d_n2, eq10_e2020_d_n3, eq10_e2020_d_n4, eq10_e2020_d_n5, eq10_e2020_d_n6, eq10_e2020_d_n7, eq10_e2020_d_n8, eq10_e2020_d_n9, eq10_e2020_d_n10, eq10_e2020_d_n11, eq10_e2020_d_n13, eq10_e2020_d_n14,) = {
    if (var_guard644 != 0.0) {
        let eq10_e2017: f64 = (var_igcd_v + var_igd_v);
        let eq10_e2017_d_n0: f64 = (var_igcd_v_dn0 + var_igd_v_dn0);
        let eq10_e2017_d_n2: f64 = (var_igcd_v_dn2 + var_igd_v_dn2);
        let eq10_e2017_d_n3: f64 = (var_igcd_v_dn3 + var_igd_v_dn3);
        let eq10_e2017_d_n4: f64 = (var_igcd_v_dn4 + var_igd_v_dn4);
        let eq10_e2017_d_n5: f64 = (var_igcd_v_dn5 + var_igd_v_dn5);
        let eq10_e2017_d_n6: f64 = (var_igcd_v_dn6 + var_igd_v_dn6);
        let eq10_e2017_d_n7: f64 = (var_igcd_v_dn7 + var_igd_v_dn7);
        let eq10_e2017_d_n8: f64 = (var_igcd_v_dn8 + var_igd_v_dn8);
        let eq10_e2017_d_n9: f64 = (var_igcd_v_dn9 + var_igd_v_dn9);
        let eq10_e2017_d_n10: f64 = (var_igcd_v_dn10 + var_igd_v_dn10);
        let eq10_e2017_d_n11: f64 = (var_igcd_v_dn11 + var_igd_v_dn11);
        let eq10_e2017_d_n13: f64 = (var_igcd_v_dn13 + var_igd_v_dn13);
        let eq10_e2017_d_n14: f64 = (var_igcd_v_dn14 + var_igd_v_dn14);
        let eq10_e2018: f64 = (var_devsign * eq10_e2017);
        let eq10_e2018_d_n0: f64 = (var_devsign * eq10_e2017_d_n0);
        let eq10_e2018_d_n2: f64 = (var_devsign * eq10_e2017_d_n2);
        let eq10_e2018_d_n3: f64 = (var_devsign * eq10_e2017_d_n3);
        let eq10_e2018_d_n4: f64 = (var_devsign * eq10_e2017_d_n4);
        let eq10_e2018_d_n5: f64 = (var_devsign * eq10_e2017_d_n5);
        let eq10_e2018_d_n6: f64 = (var_devsign * eq10_e2017_d_n6);
        let eq10_e2018_d_n7: f64 = (var_devsign * eq10_e2017_d_n7);
        let eq10_e2018_d_n8: f64 = (var_devsign * eq10_e2017_d_n8);
        let eq10_e2018_d_n9: f64 = (var_devsign * eq10_e2017_d_n9);
        let eq10_e2018_d_n10: f64 = (var_devsign * eq10_e2017_d_n10);
        let eq10_e2018_d_n11: f64 = (var_devsign * eq10_e2017_d_n11);
        let eq10_e2018_d_n13: f64 = (var_devsign * eq10_e2017_d_n13);
        let eq10_e2018_d_n14: f64 = (var_devsign * eq10_e2017_d_n14);
        (eq10_e2018, eq10_e2018_d_n0, eq10_e2018_d_n2, eq10_e2018_d_n3, eq10_e2018_d_n4, eq10_e2018_d_n5, eq10_e2018_d_n6, eq10_e2018_d_n7, eq10_e2018_d_n8, eq10_e2018_d_n9, eq10_e2018_d_n10, eq10_e2018_d_n11, eq10_e2018_d_n13, eq10_e2018_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e2020;
        let eq10_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq10_node_derivatives: [f64; 13] = [eq10_e2020_d_n0, eq10_e2020_d_n2, eq10_e2020_d_n3, eq10_e2020_d_n4, eq10_e2020_d_n5, eq10_e2020_d_n6, eq10_e2020_d_n7, eq10_e2020_d_n8, eq10_e2020_d_n9, eq10_e2020_d_n10, eq10_e2020_d_n11, eq10_e2020_d_n13, eq10_e2020_d_n14];
        let eq10_branch_derivative_indices: [usize; 0] = [];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(5),
            multiplicity * (eq10_value),
            &eq10_node_derivative_indices,
            &eq10_node_derivatives,
            &eq10_branch_derivative_indices,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let (eq11_e2032, eq11_e2032_d_n0, eq11_e2032_d_n2, eq11_e2032_d_n3, eq11_e2032_d_n4, eq11_e2032_d_n5, eq11_e2032_d_n6, eq11_e2032_d_n7, eq11_e2032_d_n8, eq11_e2032_d_n9, eq11_e2032_d_n10, eq11_e2032_d_n11, eq11_e2032_d_n13, eq11_e2032_d_n14,) = {
    if (((var_guard644 != 0.0) && (var_guard645 != 0.0)) && (var_guard646 != 0.0)) {
        let eq11_e2029: f64 = (var_igidl_v + var_iii_1);
        let eq11_e2029_d_n0: f64 = (var_igidl_v_dn0 + var_iii_1_dn0);
        let eq11_e2029_d_n2: f64 = (var_igidl_v_dn2 + var_iii_1_dn2);
        let eq11_e2029_d_n3: f64 = (var_igidl_v_dn3 + var_iii_1_dn3);
        let eq11_e2029_d_n4: f64 = (var_igidl_v_dn4 + var_iii_1_dn4);
        let eq11_e2029_d_n5: f64 = (var_igidl_v_dn5 + var_iii_1_dn5);
        let eq11_e2029_d_n6: f64 = (var_igidl_v_dn6 + var_iii_1_dn6);
        let eq11_e2029_d_n7: f64 = (var_igidl_v_dn7 + var_iii_1_dn7);
        let eq11_e2029_d_n8: f64 = (var_igidl_v_dn8 + var_iii_1_dn8);
        let eq11_e2029_d_n9: f64 = (var_igidl_v_dn9 + var_iii_1_dn9);
        let eq11_e2029_d_n10: f64 = (var_igidl_v_dn10 + var_iii_1_dn10);
        let eq11_e2029_d_n11: f64 = (var_igidl_v_dn11 + var_iii_1_dn11);
        let eq11_e2029_d_n13: f64 = (var_igidl_v_dn13 + var_iii_1_dn13);
        let eq11_e2029_d_n14: f64 = (var_igidl_v_dn14 + var_iii_1_dn14);
        let eq11_e2030: f64 = (var_devsign * eq11_e2029);
        let eq11_e2030_d_n0: f64 = (var_devsign * eq11_e2029_d_n0);
        let eq11_e2030_d_n2: f64 = (var_devsign * eq11_e2029_d_n2);
        let eq11_e2030_d_n3: f64 = (var_devsign * eq11_e2029_d_n3);
        let eq11_e2030_d_n4: f64 = (var_devsign * eq11_e2029_d_n4);
        let eq11_e2030_d_n5: f64 = (var_devsign * eq11_e2029_d_n5);
        let eq11_e2030_d_n6: f64 = (var_devsign * eq11_e2029_d_n6);
        let eq11_e2030_d_n7: f64 = (var_devsign * eq11_e2029_d_n7);
        let eq11_e2030_d_n8: f64 = (var_devsign * eq11_e2029_d_n8);
        let eq11_e2030_d_n9: f64 = (var_devsign * eq11_e2029_d_n9);
        let eq11_e2030_d_n10: f64 = (var_devsign * eq11_e2029_d_n10);
        let eq11_e2030_d_n11: f64 = (var_devsign * eq11_e2029_d_n11);
        let eq11_e2030_d_n13: f64 = (var_devsign * eq11_e2029_d_n13);
        let eq11_e2030_d_n14: f64 = (var_devsign * eq11_e2029_d_n14);
        (eq11_e2030, eq11_e2030_d_n0, eq11_e2030_d_n2, eq11_e2030_d_n3, eq11_e2030_d_n4, eq11_e2030_d_n5, eq11_e2030_d_n6, eq11_e2030_d_n7, eq11_e2030_d_n8, eq11_e2030_d_n9, eq11_e2030_d_n10, eq11_e2030_d_n11, eq11_e2030_d_n13, eq11_e2030_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e2032;
        let eq11_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq11_node_derivatives: [f64; 13] = [eq11_e2032_d_n0, eq11_e2032_d_n2, eq11_e2032_d_n3, eq11_e2032_d_n4, eq11_e2032_d_n5, eq11_e2032_d_n6, eq11_e2032_d_n7, eq11_e2032_d_n8, eq11_e2032_d_n9, eq11_e2032_d_n10, eq11_e2032_d_n11, eq11_e2032_d_n13, eq11_e2032_d_n14];
        let eq11_branch_derivative_indices: [usize; 0] = [];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq11_value),
            &eq11_node_derivative_indices,
            &eq11_node_derivatives,
            &eq11_branch_derivative_indices,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let (eq12_e2042, eq12_e2042_d_n0, eq12_e2042_d_n2, eq12_e2042_d_n3, eq12_e2042_d_n4, eq12_e2042_d_n5, eq12_e2042_d_n6, eq12_e2042_d_n7, eq12_e2042_d_n8, eq12_e2042_d_n9, eq12_e2042_d_n10, eq12_e2042_d_n11, eq12_e2042_d_n13, eq12_e2042_d_n14,) = {
    if (((var_guard644 != 0.0) && (var_guard645 != 0.0)) && (var_guard646 != 0.0)) {
        let eq12_e2040: f64 = (var_devsign * var_igisl_v);
        let eq12_e2040_d_n0: f64 = (var_devsign * var_igisl_v_dn0);
        let eq12_e2040_d_n2: f64 = (var_devsign * var_igisl_v_dn2);
        let eq12_e2040_d_n3: f64 = (var_devsign * var_igisl_v_dn3);
        let eq12_e2040_d_n4: f64 = (var_devsign * var_igisl_v_dn4);
        let eq12_e2040_d_n5: f64 = (var_devsign * var_igisl_v_dn5);
        let eq12_e2040_d_n6: f64 = (var_devsign * var_igisl_v_dn6);
        let eq12_e2040_d_n7: f64 = (var_devsign * var_igisl_v_dn7);
        let eq12_e2040_d_n8: f64 = (var_devsign * var_igisl_v_dn8);
        let eq12_e2040_d_n9: f64 = (var_devsign * var_igisl_v_dn9);
        let eq12_e2040_d_n10: f64 = (var_devsign * var_igisl_v_dn10);
        let eq12_e2040_d_n11: f64 = (var_devsign * var_igisl_v_dn11);
        let eq12_e2040_d_n13: f64 = (var_devsign * var_igisl_v_dn13);
        let eq12_e2040_d_n14: f64 = (var_devsign * var_igisl_v_dn14);
        (eq12_e2040, eq12_e2040_d_n0, eq12_e2040_d_n2, eq12_e2040_d_n3, eq12_e2040_d_n4, eq12_e2040_d_n5, eq12_e2040_d_n6, eq12_e2040_d_n7, eq12_e2040_d_n8, eq12_e2040_d_n9, eq12_e2040_d_n10, eq12_e2040_d_n11, eq12_e2040_d_n13, eq12_e2040_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq12_value: f64 = eq12_e2042;
        let eq12_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq12_node_derivatives: [f64; 13] = [eq12_e2042_d_n0, eq12_e2042_d_n2, eq12_e2042_d_n3, eq12_e2042_d_n4, eq12_e2042_d_n5, eq12_e2042_d_n6, eq12_e2042_d_n7, eq12_e2042_d_n8, eq12_e2042_d_n9, eq12_e2042_d_n10, eq12_e2042_d_n11, eq12_e2042_d_n13, eq12_e2042_d_n14];
        let eq12_branch_derivative_indices: [usize; 0] = [];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq12_value),
            &eq12_node_derivative_indices,
            &eq12_node_derivatives,
            &eq12_branch_derivative_indices,
            &eq12_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        stamper: &mut GeneratedStamper<'_>,
        multiplicity: f64,
        var_devsign: f64,
        var_guard644: f64,
        var_guard645: f64,
        var_guard646: f64,
        var_guard647: f64,
        var_guard648: f64,
        var_idsgen_v: f64,
        var_idsgen_v_dn0: f64,
        var_idsgen_v_dn10: f64,
        var_idsgen_v_dn11: f64,
        var_idsgen_v_dn13: f64,
        var_idsgen_v_dn14: f64,
        var_idsgen_v_dn2: f64,
        var_idsgen_v_dn3: f64,
        var_idsgen_v_dn4: f64,
        var_idsgen_v_dn5: f64,
        var_idsgen_v_dn6: f64,
        var_idsgen_v_dn7: f64,
        var_idsgen_v_dn8: f64,
        var_idsgen_v_dn9: f64,
        var_igbacc_v: f64,
        var_igbacc_v_dn0: f64,
        var_igbacc_v_dn10: f64,
        var_igbacc_v_dn11: f64,
        var_igbacc_v_dn13: f64,
        var_igbacc_v_dn14: f64,
        var_igbacc_v_dn2: f64,
        var_igbacc_v_dn3: f64,
        var_igbacc_v_dn4: f64,
        var_igbacc_v_dn5: f64,
        var_igbacc_v_dn6: f64,
        var_igbacc_v_dn7: f64,
        var_igbacc_v_dn8: f64,
        var_igbacc_v_dn9: f64,
        var_igbinv_v: f64,
        var_igbinv_v_dn0: f64,
        var_igbinv_v_dn10: f64,
        var_igbinv_v_dn11: f64,
        var_igbinv_v_dn13: f64,
        var_igbinv_v_dn14: f64,
        var_igbinv_v_dn2: f64,
        var_igbinv_v_dn3: f64,
        var_igbinv_v_dn4: f64,
        var_igbinv_v_dn5: f64,
        var_igbinv_v_dn6: f64,
        var_igbinv_v_dn7: f64,
        var_igbinv_v_dn8: f64,
        var_igbinv_v_dn9: f64,
        var_igcd_v: f64,
        var_igcd_v_dn0: f64,
        var_igcd_v_dn10: f64,
        var_igcd_v_dn11: f64,
        var_igcd_v_dn13: f64,
        var_igcd_v_dn14: f64,
        var_igcd_v_dn2: f64,
        var_igcd_v_dn3: f64,
        var_igcd_v_dn4: f64,
        var_igcd_v_dn5: f64,
        var_igcd_v_dn6: f64,
        var_igcd_v_dn7: f64,
        var_igcd_v_dn8: f64,
        var_igcd_v_dn9: f64,
        var_igcs_v: f64,
        var_igcs_v_dn0: f64,
        var_igcs_v_dn10: f64,
        var_igcs_v_dn11: f64,
        var_igcs_v_dn13: f64,
        var_igcs_v_dn14: f64,
        var_igcs_v_dn2: f64,
        var_igcs_v_dn3: f64,
        var_igcs_v_dn4: f64,
        var_igcs_v_dn5: f64,
        var_igcs_v_dn6: f64,
        var_igcs_v_dn7: f64,
        var_igcs_v_dn8: f64,
        var_igcs_v_dn9: f64,
        var_igd_v: f64,
        var_igd_v_dn0: f64,
        var_igd_v_dn10: f64,
        var_igd_v_dn11: f64,
        var_igd_v_dn13: f64,
        var_igd_v_dn14: f64,
        var_igd_v_dn2: f64,
        var_igd_v_dn3: f64,
        var_igd_v_dn4: f64,
        var_igd_v_dn5: f64,
        var_igd_v_dn6: f64,
        var_igd_v_dn7: f64,
        var_igd_v_dn8: f64,
        var_igd_v_dn9: f64,
        var_igidl_v: f64,
        var_igidl_v_dn0: f64,
        var_igidl_v_dn10: f64,
        var_igidl_v_dn11: f64,
        var_igidl_v_dn13: f64,
        var_igidl_v_dn14: f64,
        var_igidl_v_dn2: f64,
        var_igidl_v_dn3: f64,
        var_igidl_v_dn4: f64,
        var_igidl_v_dn5: f64,
        var_igidl_v_dn6: f64,
        var_igidl_v_dn7: f64,
        var_igidl_v_dn8: f64,
        var_igidl_v_dn9: f64,
        var_igidlb: f64,
        var_igidlb_dn0: f64,
        var_igidlb_dn10: f64,
        var_igidlb_dn11: f64,
        var_igidlb_dn13: f64,
        var_igidlb_dn14: f64,
        var_igidlb_dn2: f64,
        var_igidlb_dn3: f64,
        var_igidlb_dn4: f64,
        var_igidlb_dn5: f64,
        var_igidlb_dn6: f64,
        var_igidlb_dn7: f64,
        var_igidlb_dn8: f64,
        var_igidlb_dn9: f64,
        var_igisl_v: f64,
        var_igisl_v_dn0: f64,
        var_igisl_v_dn10: f64,
        var_igisl_v_dn11: f64,
        var_igisl_v_dn13: f64,
        var_igisl_v_dn14: f64,
        var_igisl_v_dn2: f64,
        var_igisl_v_dn3: f64,
        var_igisl_v_dn4: f64,
        var_igisl_v_dn5: f64,
        var_igisl_v_dn6: f64,
        var_igisl_v_dn7: f64,
        var_igisl_v_dn8: f64,
        var_igisl_v_dn9: f64,
        var_igislb: f64,
        var_igislb_dn0: f64,
        var_igislb_dn10: f64,
        var_igislb_dn11: f64,
        var_igislb_dn13: f64,
        var_igislb_dn14: f64,
        var_igislb_dn2: f64,
        var_igislb_dn3: f64,
        var_igislb_dn4: f64,
        var_igislb_dn5: f64,
        var_igislb_dn6: f64,
        var_igislb_dn7: f64,
        var_igislb_dn8: f64,
        var_igislb_dn9: f64,
        var_igs_v: f64,
        var_igs_v_dn0: f64,
        var_igs_v_dn10: f64,
        var_igs_v_dn11: f64,
        var_igs_v_dn13: f64,
        var_igs_v_dn14: f64,
        var_igs_v_dn2: f64,
        var_igs_v_dn3: f64,
        var_igs_v_dn4: f64,
        var_igs_v_dn5: f64,
        var_igs_v_dn6: f64,
        var_igs_v_dn7: f64,
        var_igs_v_dn8: f64,
        var_igs_v_dn9: f64,
        var_iii_1: f64,
        var_iii_1_dn0: f64,
        var_iii_1_dn10: f64,
        var_iii_1_dn11: f64,
        var_iii_1_dn13: f64,
        var_iii_1_dn14: f64,
        var_iii_1_dn2: f64,
        var_iii_1_dn3: f64,
        var_iii_1_dn4: f64,
        var_iii_1_dn5: f64,
        var_iii_1_dn6: f64,
        var_iii_1_dn7: f64,
        var_iii_1_dn8: f64,
        var_iii_1_dn9: f64,
    ) {
        let (eq13_e2052, eq13_e2052_d_n0, eq13_e2052_d_n2, eq13_e2052_d_n3, eq13_e2052_d_n4, eq13_e2052_d_n5, eq13_e2052_d_n6, eq13_e2052_d_n7, eq13_e2052_d_n8, eq13_e2052_d_n9, eq13_e2052_d_n10, eq13_e2052_d_n11, eq13_e2052_d_n13, eq13_e2052_d_n14,) = {
    if (((var_guard644 != 0.0) && (var_guard645 != 0.0)) && (var_guard646 != 0.0)) {
        let eq13_e2050: f64 = (var_devsign * var_igidlb);
        let eq13_e2050_d_n0: f64 = (var_devsign * var_igidlb_dn0);
        let eq13_e2050_d_n2: f64 = (var_devsign * var_igidlb_dn2);
        let eq13_e2050_d_n3: f64 = (var_devsign * var_igidlb_dn3);
        let eq13_e2050_d_n4: f64 = (var_devsign * var_igidlb_dn4);
        let eq13_e2050_d_n5: f64 = (var_devsign * var_igidlb_dn5);
        let eq13_e2050_d_n6: f64 = (var_devsign * var_igidlb_dn6);
        let eq13_e2050_d_n7: f64 = (var_devsign * var_igidlb_dn7);
        let eq13_e2050_d_n8: f64 = (var_devsign * var_igidlb_dn8);
        let eq13_e2050_d_n9: f64 = (var_devsign * var_igidlb_dn9);
        let eq13_e2050_d_n10: f64 = (var_devsign * var_igidlb_dn10);
        let eq13_e2050_d_n11: f64 = (var_devsign * var_igidlb_dn11);
        let eq13_e2050_d_n13: f64 = (var_devsign * var_igidlb_dn13);
        let eq13_e2050_d_n14: f64 = (var_devsign * var_igidlb_dn14);
        (eq13_e2050, eq13_e2050_d_n0, eq13_e2050_d_n2, eq13_e2050_d_n3, eq13_e2050_d_n4, eq13_e2050_d_n5, eq13_e2050_d_n6, eq13_e2050_d_n7, eq13_e2050_d_n8, eq13_e2050_d_n9, eq13_e2050_d_n10, eq13_e2050_d_n11, eq13_e2050_d_n13, eq13_e2050_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq13_value: f64 = eq13_e2052;
        let eq13_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq13_node_derivatives: [f64; 13] = [eq13_e2052_d_n0, eq13_e2052_d_n2, eq13_e2052_d_n3, eq13_e2052_d_n4, eq13_e2052_d_n5, eq13_e2052_d_n6, eq13_e2052_d_n7, eq13_e2052_d_n8, eq13_e2052_d_n9, eq13_e2052_d_n10, eq13_e2052_d_n11, eq13_e2052_d_n13, eq13_e2052_d_n14];
        let eq13_branch_derivative_indices: [usize; 0] = [];
        let eq13_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(3),
            multiplicity * (eq13_value),
            &eq13_node_derivative_indices,
            &eq13_node_derivatives,
            &eq13_branch_derivative_indices,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let (eq14_e2062, eq14_e2062_d_n0, eq14_e2062_d_n2, eq14_e2062_d_n3, eq14_e2062_d_n4, eq14_e2062_d_n5, eq14_e2062_d_n6, eq14_e2062_d_n7, eq14_e2062_d_n8, eq14_e2062_d_n9, eq14_e2062_d_n10, eq14_e2062_d_n11, eq14_e2062_d_n13, eq14_e2062_d_n14,) = {
    if (((var_guard644 != 0.0) && (var_guard645 != 0.0)) && (var_guard646 != 0.0)) {
        let eq14_e2060: f64 = (var_devsign * var_igislb);
        let eq14_e2060_d_n0: f64 = (var_devsign * var_igislb_dn0);
        let eq14_e2060_d_n2: f64 = (var_devsign * var_igislb_dn2);
        let eq14_e2060_d_n3: f64 = (var_devsign * var_igislb_dn3);
        let eq14_e2060_d_n4: f64 = (var_devsign * var_igislb_dn4);
        let eq14_e2060_d_n5: f64 = (var_devsign * var_igislb_dn5);
        let eq14_e2060_d_n6: f64 = (var_devsign * var_igislb_dn6);
        let eq14_e2060_d_n7: f64 = (var_devsign * var_igislb_dn7);
        let eq14_e2060_d_n8: f64 = (var_devsign * var_igislb_dn8);
        let eq14_e2060_d_n9: f64 = (var_devsign * var_igislb_dn9);
        let eq14_e2060_d_n10: f64 = (var_devsign * var_igislb_dn10);
        let eq14_e2060_d_n11: f64 = (var_devsign * var_igislb_dn11);
        let eq14_e2060_d_n13: f64 = (var_devsign * var_igislb_dn13);
        let eq14_e2060_d_n14: f64 = (var_devsign * var_igislb_dn14);
        (eq14_e2060, eq14_e2060_d_n0, eq14_e2060_d_n2, eq14_e2060_d_n3, eq14_e2060_d_n4, eq14_e2060_d_n5, eq14_e2060_d_n6, eq14_e2060_d_n7, eq14_e2060_d_n8, eq14_e2060_d_n9, eq14_e2060_d_n10, eq14_e2060_d_n11, eq14_e2060_d_n13, eq14_e2060_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq14_value: f64 = eq14_e2062;
        let eq14_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq14_node_derivatives: [f64; 13] = [eq14_e2062_d_n0, eq14_e2062_d_n2, eq14_e2062_d_n3, eq14_e2062_d_n4, eq14_e2062_d_n5, eq14_e2062_d_n6, eq14_e2062_d_n7, eq14_e2062_d_n8, eq14_e2062_d_n9, eq14_e2062_d_n10, eq14_e2062_d_n11, eq14_e2062_d_n13, eq14_e2062_d_n14];
        let eq14_branch_derivative_indices: [usize; 0] = [];
        let eq14_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(3),
            multiplicity * (eq14_value),
            &eq14_node_derivative_indices,
            &eq14_node_derivatives,
            &eq14_branch_derivative_indices,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let (eq15_e2075, eq15_e2075_d_n0, eq15_e2075_d_n2, eq15_e2075_d_n3, eq15_e2075_d_n4, eq15_e2075_d_n5, eq15_e2075_d_n6, eq15_e2075_d_n7, eq15_e2075_d_n8, eq15_e2075_d_n9, eq15_e2075_d_n10, eq15_e2075_d_n11, eq15_e2075_d_n13, eq15_e2075_d_n14,) = {
    if (((var_guard644 != 0.0) && (var_guard645 != 0.0)) && (var_guard646 == 0.0)) {
        let eq15_e2072: f64 = (var_igidl_v + var_iii_1);
        let eq15_e2072_d_n0: f64 = (var_igidl_v_dn0 + var_iii_1_dn0);
        let eq15_e2072_d_n2: f64 = (var_igidl_v_dn2 + var_iii_1_dn2);
        let eq15_e2072_d_n3: f64 = (var_igidl_v_dn3 + var_iii_1_dn3);
        let eq15_e2072_d_n4: f64 = (var_igidl_v_dn4 + var_iii_1_dn4);
        let eq15_e2072_d_n5: f64 = (var_igidl_v_dn5 + var_iii_1_dn5);
        let eq15_e2072_d_n6: f64 = (var_igidl_v_dn6 + var_iii_1_dn6);
        let eq15_e2072_d_n7: f64 = (var_igidl_v_dn7 + var_iii_1_dn7);
        let eq15_e2072_d_n8: f64 = (var_igidl_v_dn8 + var_iii_1_dn8);
        let eq15_e2072_d_n9: f64 = (var_igidl_v_dn9 + var_iii_1_dn9);
        let eq15_e2072_d_n10: f64 = (var_igidl_v_dn10 + var_iii_1_dn10);
        let eq15_e2072_d_n11: f64 = (var_igidl_v_dn11 + var_iii_1_dn11);
        let eq15_e2072_d_n13: f64 = (var_igidl_v_dn13 + var_iii_1_dn13);
        let eq15_e2072_d_n14: f64 = (var_igidl_v_dn14 + var_iii_1_dn14);
        let eq15_e2073: f64 = (var_devsign * eq15_e2072);
        let eq15_e2073_d_n0: f64 = (var_devsign * eq15_e2072_d_n0);
        let eq15_e2073_d_n2: f64 = (var_devsign * eq15_e2072_d_n2);
        let eq15_e2073_d_n3: f64 = (var_devsign * eq15_e2072_d_n3);
        let eq15_e2073_d_n4: f64 = (var_devsign * eq15_e2072_d_n4);
        let eq15_e2073_d_n5: f64 = (var_devsign * eq15_e2072_d_n5);
        let eq15_e2073_d_n6: f64 = (var_devsign * eq15_e2072_d_n6);
        let eq15_e2073_d_n7: f64 = (var_devsign * eq15_e2072_d_n7);
        let eq15_e2073_d_n8: f64 = (var_devsign * eq15_e2072_d_n8);
        let eq15_e2073_d_n9: f64 = (var_devsign * eq15_e2072_d_n9);
        let eq15_e2073_d_n10: f64 = (var_devsign * eq15_e2072_d_n10);
        let eq15_e2073_d_n11: f64 = (var_devsign * eq15_e2072_d_n11);
        let eq15_e2073_d_n13: f64 = (var_devsign * eq15_e2072_d_n13);
        let eq15_e2073_d_n14: f64 = (var_devsign * eq15_e2072_d_n14);
        (eq15_e2073, eq15_e2073_d_n0, eq15_e2073_d_n2, eq15_e2073_d_n3, eq15_e2073_d_n4, eq15_e2073_d_n5, eq15_e2073_d_n6, eq15_e2073_d_n7, eq15_e2073_d_n8, eq15_e2073_d_n9, eq15_e2073_d_n10, eq15_e2073_d_n11, eq15_e2073_d_n13, eq15_e2073_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e2075;
        let eq15_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq15_node_derivatives: [f64; 13] = [eq15_e2075_d_n0, eq15_e2075_d_n2, eq15_e2075_d_n3, eq15_e2075_d_n4, eq15_e2075_d_n5, eq15_e2075_d_n6, eq15_e2075_d_n7, eq15_e2075_d_n8, eq15_e2075_d_n9, eq15_e2075_d_n10, eq15_e2075_d_n11, eq15_e2075_d_n13, eq15_e2075_d_n14];
        let eq15_branch_derivative_indices: [usize; 0] = [];
        let eq15_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(3),
            multiplicity * (eq15_value),
            &eq15_node_derivative_indices,
            &eq15_node_derivatives,
            &eq15_branch_derivative_indices,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let (eq16_e2086, eq16_e2086_d_n0, eq16_e2086_d_n2, eq16_e2086_d_n3, eq16_e2086_d_n4, eq16_e2086_d_n5, eq16_e2086_d_n6, eq16_e2086_d_n7, eq16_e2086_d_n8, eq16_e2086_d_n9, eq16_e2086_d_n10, eq16_e2086_d_n11, eq16_e2086_d_n13, eq16_e2086_d_n14,) = {
    if (((var_guard644 != 0.0) && (var_guard645 != 0.0)) && (var_guard646 == 0.0)) {
        let eq16_e2084: f64 = (var_devsign * var_igisl_v);
        let eq16_e2084_d_n0: f64 = (var_devsign * var_igisl_v_dn0);
        let eq16_e2084_d_n2: f64 = (var_devsign * var_igisl_v_dn2);
        let eq16_e2084_d_n3: f64 = (var_devsign * var_igisl_v_dn3);
        let eq16_e2084_d_n4: f64 = (var_devsign * var_igisl_v_dn4);
        let eq16_e2084_d_n5: f64 = (var_devsign * var_igisl_v_dn5);
        let eq16_e2084_d_n6: f64 = (var_devsign * var_igisl_v_dn6);
        let eq16_e2084_d_n7: f64 = (var_devsign * var_igisl_v_dn7);
        let eq16_e2084_d_n8: f64 = (var_devsign * var_igisl_v_dn8);
        let eq16_e2084_d_n9: f64 = (var_devsign * var_igisl_v_dn9);
        let eq16_e2084_d_n10: f64 = (var_devsign * var_igisl_v_dn10);
        let eq16_e2084_d_n11: f64 = (var_devsign * var_igisl_v_dn11);
        let eq16_e2084_d_n13: f64 = (var_devsign * var_igisl_v_dn13);
        let eq16_e2084_d_n14: f64 = (var_devsign * var_igisl_v_dn14);
        (eq16_e2084, eq16_e2084_d_n0, eq16_e2084_d_n2, eq16_e2084_d_n3, eq16_e2084_d_n4, eq16_e2084_d_n5, eq16_e2084_d_n6, eq16_e2084_d_n7, eq16_e2084_d_n8, eq16_e2084_d_n9, eq16_e2084_d_n10, eq16_e2084_d_n11, eq16_e2084_d_n13, eq16_e2084_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq16_value: f64 = eq16_e2086;
        let eq16_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq16_node_derivatives: [f64; 13] = [eq16_e2086_d_n0, eq16_e2086_d_n2, eq16_e2086_d_n3, eq16_e2086_d_n4, eq16_e2086_d_n5, eq16_e2086_d_n6, eq16_e2086_d_n7, eq16_e2086_d_n8, eq16_e2086_d_n9, eq16_e2086_d_n10, eq16_e2086_d_n11, eq16_e2086_d_n13, eq16_e2086_d_n14];
        let eq16_branch_derivative_indices: [usize; 0] = [];
        let eq16_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(3),
            multiplicity * (eq16_value),
            &eq16_node_derivative_indices,
            &eq16_node_derivatives,
            &eq16_branch_derivative_indices,
            &eq16_branch_derivatives,
            multiplicity,
        );
        let (eq17_e2096, eq17_e2096_d_n0, eq17_e2096_d_n2, eq17_e2096_d_n3, eq17_e2096_d_n4, eq17_e2096_d_n5, eq17_e2096_d_n6, eq17_e2096_d_n7, eq17_e2096_d_n8, eq17_e2096_d_n9, eq17_e2096_d_n10, eq17_e2096_d_n11, eq17_e2096_d_n13, eq17_e2096_d_n14,) = {
    if ((var_guard644 != 0.0) && (var_guard645 != 0.0)) {
        let eq17_e2093: f64 = (var_igbinv_v + var_igbacc_v);
        let eq17_e2093_d_n0: f64 = (var_igbinv_v_dn0 + var_igbacc_v_dn0);
        let eq17_e2093_d_n2: f64 = (var_igbinv_v_dn2 + var_igbacc_v_dn2);
        let eq17_e2093_d_n3: f64 = (var_igbinv_v_dn3 + var_igbacc_v_dn3);
        let eq17_e2093_d_n4: f64 = (var_igbinv_v_dn4 + var_igbacc_v_dn4);
        let eq17_e2093_d_n5: f64 = (var_igbinv_v_dn5 + var_igbacc_v_dn5);
        let eq17_e2093_d_n6: f64 = (var_igbinv_v_dn6 + var_igbacc_v_dn6);
        let eq17_e2093_d_n7: f64 = (var_igbinv_v_dn7 + var_igbacc_v_dn7);
        let eq17_e2093_d_n8: f64 = (var_igbinv_v_dn8 + var_igbacc_v_dn8);
        let eq17_e2093_d_n9: f64 = (var_igbinv_v_dn9 + var_igbacc_v_dn9);
        let eq17_e2093_d_n10: f64 = (var_igbinv_v_dn10 + var_igbacc_v_dn10);
        let eq17_e2093_d_n11: f64 = (var_igbinv_v_dn11 + var_igbacc_v_dn11);
        let eq17_e2093_d_n13: f64 = (var_igbinv_v_dn13 + var_igbacc_v_dn13);
        let eq17_e2093_d_n14: f64 = (var_igbinv_v_dn14 + var_igbacc_v_dn14);
        let eq17_e2094: f64 = (var_devsign * eq17_e2093);
        let eq17_e2094_d_n0: f64 = (var_devsign * eq17_e2093_d_n0);
        let eq17_e2094_d_n2: f64 = (var_devsign * eq17_e2093_d_n2);
        let eq17_e2094_d_n3: f64 = (var_devsign * eq17_e2093_d_n3);
        let eq17_e2094_d_n4: f64 = (var_devsign * eq17_e2093_d_n4);
        let eq17_e2094_d_n5: f64 = (var_devsign * eq17_e2093_d_n5);
        let eq17_e2094_d_n6: f64 = (var_devsign * eq17_e2093_d_n6);
        let eq17_e2094_d_n7: f64 = (var_devsign * eq17_e2093_d_n7);
        let eq17_e2094_d_n8: f64 = (var_devsign * eq17_e2093_d_n8);
        let eq17_e2094_d_n9: f64 = (var_devsign * eq17_e2093_d_n9);
        let eq17_e2094_d_n10: f64 = (var_devsign * eq17_e2093_d_n10);
        let eq17_e2094_d_n11: f64 = (var_devsign * eq17_e2093_d_n11);
        let eq17_e2094_d_n13: f64 = (var_devsign * eq17_e2093_d_n13);
        let eq17_e2094_d_n14: f64 = (var_devsign * eq17_e2093_d_n14);
        (eq17_e2094, eq17_e2094_d_n0, eq17_e2094_d_n2, eq17_e2094_d_n3, eq17_e2094_d_n4, eq17_e2094_d_n5, eq17_e2094_d_n6, eq17_e2094_d_n7, eq17_e2094_d_n8, eq17_e2094_d_n9, eq17_e2094_d_n10, eq17_e2094_d_n11, eq17_e2094_d_n13, eq17_e2094_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e2096;
        let eq17_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq17_node_derivatives: [f64; 13] = [eq17_e2096_d_n0, eq17_e2096_d_n2, eq17_e2096_d_n3, eq17_e2096_d_n4, eq17_e2096_d_n5, eq17_e2096_d_n6, eq17_e2096_d_n7, eq17_e2096_d_n8, eq17_e2096_d_n9, eq17_e2096_d_n10, eq17_e2096_d_n11, eq17_e2096_d_n13, eq17_e2096_d_n14];
        let eq17_branch_derivative_indices: [usize; 0] = [];
        let eq17_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(3),
            multiplicity * (eq17_value),
            &eq17_node_derivative_indices,
            &eq17_node_derivatives,
            &eq17_branch_derivative_indices,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let (eq18_e2107, eq18_e2107_d_n0, eq18_e2107_d_n2, eq18_e2107_d_n3, eq18_e2107_d_n4, eq18_e2107_d_n5, eq18_e2107_d_n6, eq18_e2107_d_n7, eq18_e2107_d_n8, eq18_e2107_d_n9, eq18_e2107_d_n10, eq18_e2107_d_n11, eq18_e2107_d_n13, eq18_e2107_d_n14,) = {
    if ((var_guard644 != 0.0) && (var_guard645 == 0.0)) {
        let eq18_e2104: f64 = (var_igidl_v + var_iii_1);
        let eq18_e2104_d_n0: f64 = (var_igidl_v_dn0 + var_iii_1_dn0);
        let eq18_e2104_d_n2: f64 = (var_igidl_v_dn2 + var_iii_1_dn2);
        let eq18_e2104_d_n3: f64 = (var_igidl_v_dn3 + var_iii_1_dn3);
        let eq18_e2104_d_n4: f64 = (var_igidl_v_dn4 + var_iii_1_dn4);
        let eq18_e2104_d_n5: f64 = (var_igidl_v_dn5 + var_iii_1_dn5);
        let eq18_e2104_d_n6: f64 = (var_igidl_v_dn6 + var_iii_1_dn6);
        let eq18_e2104_d_n7: f64 = (var_igidl_v_dn7 + var_iii_1_dn7);
        let eq18_e2104_d_n8: f64 = (var_igidl_v_dn8 + var_iii_1_dn8);
        let eq18_e2104_d_n9: f64 = (var_igidl_v_dn9 + var_iii_1_dn9);
        let eq18_e2104_d_n10: f64 = (var_igidl_v_dn10 + var_iii_1_dn10);
        let eq18_e2104_d_n11: f64 = (var_igidl_v_dn11 + var_iii_1_dn11);
        let eq18_e2104_d_n13: f64 = (var_igidl_v_dn13 + var_iii_1_dn13);
        let eq18_e2104_d_n14: f64 = (var_igidl_v_dn14 + var_iii_1_dn14);
        let eq18_e2105: f64 = (var_devsign * eq18_e2104);
        let eq18_e2105_d_n0: f64 = (var_devsign * eq18_e2104_d_n0);
        let eq18_e2105_d_n2: f64 = (var_devsign * eq18_e2104_d_n2);
        let eq18_e2105_d_n3: f64 = (var_devsign * eq18_e2104_d_n3);
        let eq18_e2105_d_n4: f64 = (var_devsign * eq18_e2104_d_n4);
        let eq18_e2105_d_n5: f64 = (var_devsign * eq18_e2104_d_n5);
        let eq18_e2105_d_n6: f64 = (var_devsign * eq18_e2104_d_n6);
        let eq18_e2105_d_n7: f64 = (var_devsign * eq18_e2104_d_n7);
        let eq18_e2105_d_n8: f64 = (var_devsign * eq18_e2104_d_n8);
        let eq18_e2105_d_n9: f64 = (var_devsign * eq18_e2104_d_n9);
        let eq18_e2105_d_n10: f64 = (var_devsign * eq18_e2104_d_n10);
        let eq18_e2105_d_n11: f64 = (var_devsign * eq18_e2104_d_n11);
        let eq18_e2105_d_n13: f64 = (var_devsign * eq18_e2104_d_n13);
        let eq18_e2105_d_n14: f64 = (var_devsign * eq18_e2104_d_n14);
        (eq18_e2105, eq18_e2105_d_n0, eq18_e2105_d_n2, eq18_e2105_d_n3, eq18_e2105_d_n4, eq18_e2105_d_n5, eq18_e2105_d_n6, eq18_e2105_d_n7, eq18_e2105_d_n8, eq18_e2105_d_n9, eq18_e2105_d_n10, eq18_e2105_d_n11, eq18_e2105_d_n13, eq18_e2105_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e2107;
        let eq18_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq18_node_derivatives: [f64; 13] = [eq18_e2107_d_n0, eq18_e2107_d_n2, eq18_e2107_d_n3, eq18_e2107_d_n4, eq18_e2107_d_n5, eq18_e2107_d_n6, eq18_e2107_d_n7, eq18_e2107_d_n8, eq18_e2107_d_n9, eq18_e2107_d_n10, eq18_e2107_d_n11, eq18_e2107_d_n13, eq18_e2107_d_n14];
        let eq18_branch_derivative_indices: [usize; 0] = [];
        let eq18_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq18_value),
            &eq18_node_derivative_indices,
            &eq18_node_derivatives,
            &eq18_branch_derivative_indices,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let (eq19_e2116, eq19_e2116_d_n0, eq19_e2116_d_n2, eq19_e2116_d_n3, eq19_e2116_d_n4, eq19_e2116_d_n5, eq19_e2116_d_n6, eq19_e2116_d_n7, eq19_e2116_d_n8, eq19_e2116_d_n9, eq19_e2116_d_n10, eq19_e2116_d_n11, eq19_e2116_d_n13, eq19_e2116_d_n14,) = {
    if ((var_guard644 != 0.0) && (var_guard645 == 0.0)) {
        let eq19_e2114: f64 = (var_devsign * var_igisl_v);
        let eq19_e2114_d_n0: f64 = (var_devsign * var_igisl_v_dn0);
        let eq19_e2114_d_n2: f64 = (var_devsign * var_igisl_v_dn2);
        let eq19_e2114_d_n3: f64 = (var_devsign * var_igisl_v_dn3);
        let eq19_e2114_d_n4: f64 = (var_devsign * var_igisl_v_dn4);
        let eq19_e2114_d_n5: f64 = (var_devsign * var_igisl_v_dn5);
        let eq19_e2114_d_n6: f64 = (var_devsign * var_igisl_v_dn6);
        let eq19_e2114_d_n7: f64 = (var_devsign * var_igisl_v_dn7);
        let eq19_e2114_d_n8: f64 = (var_devsign * var_igisl_v_dn8);
        let eq19_e2114_d_n9: f64 = (var_devsign * var_igisl_v_dn9);
        let eq19_e2114_d_n10: f64 = (var_devsign * var_igisl_v_dn10);
        let eq19_e2114_d_n11: f64 = (var_devsign * var_igisl_v_dn11);
        let eq19_e2114_d_n13: f64 = (var_devsign * var_igisl_v_dn13);
        let eq19_e2114_d_n14: f64 = (var_devsign * var_igisl_v_dn14);
        (eq19_e2114, eq19_e2114_d_n0, eq19_e2114_d_n2, eq19_e2114_d_n3, eq19_e2114_d_n4, eq19_e2114_d_n5, eq19_e2114_d_n6, eq19_e2114_d_n7, eq19_e2114_d_n8, eq19_e2114_d_n9, eq19_e2114_d_n10, eq19_e2114_d_n11, eq19_e2114_d_n13, eq19_e2114_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_value: f64 = eq19_e2116;
        let eq19_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq19_node_derivatives: [f64; 13] = [eq19_e2116_d_n0, eq19_e2116_d_n2, eq19_e2116_d_n3, eq19_e2116_d_n4, eq19_e2116_d_n5, eq19_e2116_d_n6, eq19_e2116_d_n7, eq19_e2116_d_n8, eq19_e2116_d_n9, eq19_e2116_d_n10, eq19_e2116_d_n11, eq19_e2116_d_n13, eq19_e2116_d_n14];
        let eq19_branch_derivative_indices: [usize; 0] = [];
        let eq19_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq19_value),
            &eq19_node_derivative_indices,
            &eq19_node_derivatives,
            &eq19_branch_derivative_indices,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let (eq20_e2123, eq20_e2123_d_n0, eq20_e2123_d_n2, eq20_e2123_d_n3, eq20_e2123_d_n4, eq20_e2123_d_n5, eq20_e2123_d_n6, eq20_e2123_d_n7, eq20_e2123_d_n8, eq20_e2123_d_n9, eq20_e2123_d_n10, eq20_e2123_d_n11, eq20_e2123_d_n13, eq20_e2123_d_n14,) = {
    if (var_guard644 == 0.0) {
        let eq20_e2121: f64 = (var_devsign * var_idsgen_v);
        let eq20_e2121_d_n0: f64 = (var_devsign * var_idsgen_v_dn0);
        let eq20_e2121_d_n2: f64 = (var_devsign * var_idsgen_v_dn2);
        let eq20_e2121_d_n3: f64 = (var_devsign * var_idsgen_v_dn3);
        let eq20_e2121_d_n4: f64 = (var_devsign * var_idsgen_v_dn4);
        let eq20_e2121_d_n5: f64 = (var_devsign * var_idsgen_v_dn5);
        let eq20_e2121_d_n6: f64 = (var_devsign * var_idsgen_v_dn6);
        let eq20_e2121_d_n7: f64 = (var_devsign * var_idsgen_v_dn7);
        let eq20_e2121_d_n8: f64 = (var_devsign * var_idsgen_v_dn8);
        let eq20_e2121_d_n9: f64 = (var_devsign * var_idsgen_v_dn9);
        let eq20_e2121_d_n10: f64 = (var_devsign * var_idsgen_v_dn10);
        let eq20_e2121_d_n11: f64 = (var_devsign * var_idsgen_v_dn11);
        let eq20_e2121_d_n13: f64 = (var_devsign * var_idsgen_v_dn13);
        let eq20_e2121_d_n14: f64 = (var_devsign * var_idsgen_v_dn14);
        (eq20_e2121, eq20_e2121_d_n0, eq20_e2121_d_n2, eq20_e2121_d_n3, eq20_e2121_d_n4, eq20_e2121_d_n5, eq20_e2121_d_n6, eq20_e2121_d_n7, eq20_e2121_d_n8, eq20_e2121_d_n9, eq20_e2121_d_n10, eq20_e2121_d_n11, eq20_e2121_d_n13, eq20_e2121_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq20_value: f64 = eq20_e2123;
        let eq20_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq20_node_derivatives: [f64; 13] = [eq20_e2123_d_n0, eq20_e2123_d_n2, eq20_e2123_d_n3, eq20_e2123_d_n4, eq20_e2123_d_n5, eq20_e2123_d_n6, eq20_e2123_d_n7, eq20_e2123_d_n8, eq20_e2123_d_n9, eq20_e2123_d_n10, eq20_e2123_d_n11, eq20_e2123_d_n13, eq20_e2123_d_n14];
        let eq20_branch_derivative_indices: [usize; 0] = [];
        let eq20_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq20_value),
            &eq20_node_derivative_indices,
            &eq20_node_derivatives,
            &eq20_branch_derivative_indices,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let (eq21_e2132, eq21_e2132_d_n0, eq21_e2132_d_n2, eq21_e2132_d_n3, eq21_e2132_d_n4, eq21_e2132_d_n5, eq21_e2132_d_n6, eq21_e2132_d_n7, eq21_e2132_d_n8, eq21_e2132_d_n9, eq21_e2132_d_n10, eq21_e2132_d_n11, eq21_e2132_d_n13, eq21_e2132_d_n14,) = {
    if (var_guard644 == 0.0) {
        let eq21_e2129: f64 = (var_igcs_v + var_igs_v);
        let eq21_e2129_d_n0: f64 = (var_igcs_v_dn0 + var_igs_v_dn0);
        let eq21_e2129_d_n2: f64 = (var_igcs_v_dn2 + var_igs_v_dn2);
        let eq21_e2129_d_n3: f64 = (var_igcs_v_dn3 + var_igs_v_dn3);
        let eq21_e2129_d_n4: f64 = (var_igcs_v_dn4 + var_igs_v_dn4);
        let eq21_e2129_d_n5: f64 = (var_igcs_v_dn5 + var_igs_v_dn5);
        let eq21_e2129_d_n6: f64 = (var_igcs_v_dn6 + var_igs_v_dn6);
        let eq21_e2129_d_n7: f64 = (var_igcs_v_dn7 + var_igs_v_dn7);
        let eq21_e2129_d_n8: f64 = (var_igcs_v_dn8 + var_igs_v_dn8);
        let eq21_e2129_d_n9: f64 = (var_igcs_v_dn9 + var_igs_v_dn9);
        let eq21_e2129_d_n10: f64 = (var_igcs_v_dn10 + var_igs_v_dn10);
        let eq21_e2129_d_n11: f64 = (var_igcs_v_dn11 + var_igs_v_dn11);
        let eq21_e2129_d_n13: f64 = (var_igcs_v_dn13 + var_igs_v_dn13);
        let eq21_e2129_d_n14: f64 = (var_igcs_v_dn14 + var_igs_v_dn14);
        let eq21_e2130: f64 = (var_devsign * eq21_e2129);
        let eq21_e2130_d_n0: f64 = (var_devsign * eq21_e2129_d_n0);
        let eq21_e2130_d_n2: f64 = (var_devsign * eq21_e2129_d_n2);
        let eq21_e2130_d_n3: f64 = (var_devsign * eq21_e2129_d_n3);
        let eq21_e2130_d_n4: f64 = (var_devsign * eq21_e2129_d_n4);
        let eq21_e2130_d_n5: f64 = (var_devsign * eq21_e2129_d_n5);
        let eq21_e2130_d_n6: f64 = (var_devsign * eq21_e2129_d_n6);
        let eq21_e2130_d_n7: f64 = (var_devsign * eq21_e2129_d_n7);
        let eq21_e2130_d_n8: f64 = (var_devsign * eq21_e2129_d_n8);
        let eq21_e2130_d_n9: f64 = (var_devsign * eq21_e2129_d_n9);
        let eq21_e2130_d_n10: f64 = (var_devsign * eq21_e2129_d_n10);
        let eq21_e2130_d_n11: f64 = (var_devsign * eq21_e2129_d_n11);
        let eq21_e2130_d_n13: f64 = (var_devsign * eq21_e2129_d_n13);
        let eq21_e2130_d_n14: f64 = (var_devsign * eq21_e2129_d_n14);
        (eq21_e2130, eq21_e2130_d_n0, eq21_e2130_d_n2, eq21_e2130_d_n3, eq21_e2130_d_n4, eq21_e2130_d_n5, eq21_e2130_d_n6, eq21_e2130_d_n7, eq21_e2130_d_n8, eq21_e2130_d_n9, eq21_e2130_d_n10, eq21_e2130_d_n11, eq21_e2130_d_n13, eq21_e2130_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e2132;
        let eq21_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq21_node_derivatives: [f64; 13] = [eq21_e2132_d_n0, eq21_e2132_d_n2, eq21_e2132_d_n3, eq21_e2132_d_n4, eq21_e2132_d_n5, eq21_e2132_d_n6, eq21_e2132_d_n7, eq21_e2132_d_n8, eq21_e2132_d_n9, eq21_e2132_d_n10, eq21_e2132_d_n11, eq21_e2132_d_n13, eq21_e2132_d_n14];
        let eq21_branch_derivative_indices: [usize; 0] = [];
        let eq21_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(5),
            multiplicity * (eq21_value),
            &eq21_node_derivative_indices,
            &eq21_node_derivatives,
            &eq21_branch_derivative_indices,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let (eq22_e2141, eq22_e2141_d_n0, eq22_e2141_d_n2, eq22_e2141_d_n3, eq22_e2141_d_n4, eq22_e2141_d_n5, eq22_e2141_d_n6, eq22_e2141_d_n7, eq22_e2141_d_n8, eq22_e2141_d_n9, eq22_e2141_d_n10, eq22_e2141_d_n11, eq22_e2141_d_n13, eq22_e2141_d_n14,) = {
    if (var_guard644 == 0.0) {
        let eq22_e2138: f64 = (var_igcd_v + var_igd_v);
        let eq22_e2138_d_n0: f64 = (var_igcd_v_dn0 + var_igd_v_dn0);
        let eq22_e2138_d_n2: f64 = (var_igcd_v_dn2 + var_igd_v_dn2);
        let eq22_e2138_d_n3: f64 = (var_igcd_v_dn3 + var_igd_v_dn3);
        let eq22_e2138_d_n4: f64 = (var_igcd_v_dn4 + var_igd_v_dn4);
        let eq22_e2138_d_n5: f64 = (var_igcd_v_dn5 + var_igd_v_dn5);
        let eq22_e2138_d_n6: f64 = (var_igcd_v_dn6 + var_igd_v_dn6);
        let eq22_e2138_d_n7: f64 = (var_igcd_v_dn7 + var_igd_v_dn7);
        let eq22_e2138_d_n8: f64 = (var_igcd_v_dn8 + var_igd_v_dn8);
        let eq22_e2138_d_n9: f64 = (var_igcd_v_dn9 + var_igd_v_dn9);
        let eq22_e2138_d_n10: f64 = (var_igcd_v_dn10 + var_igd_v_dn10);
        let eq22_e2138_d_n11: f64 = (var_igcd_v_dn11 + var_igd_v_dn11);
        let eq22_e2138_d_n13: f64 = (var_igcd_v_dn13 + var_igd_v_dn13);
        let eq22_e2138_d_n14: f64 = (var_igcd_v_dn14 + var_igd_v_dn14);
        let eq22_e2139: f64 = (var_devsign * eq22_e2138);
        let eq22_e2139_d_n0: f64 = (var_devsign * eq22_e2138_d_n0);
        let eq22_e2139_d_n2: f64 = (var_devsign * eq22_e2138_d_n2);
        let eq22_e2139_d_n3: f64 = (var_devsign * eq22_e2138_d_n3);
        let eq22_e2139_d_n4: f64 = (var_devsign * eq22_e2138_d_n4);
        let eq22_e2139_d_n5: f64 = (var_devsign * eq22_e2138_d_n5);
        let eq22_e2139_d_n6: f64 = (var_devsign * eq22_e2138_d_n6);
        let eq22_e2139_d_n7: f64 = (var_devsign * eq22_e2138_d_n7);
        let eq22_e2139_d_n8: f64 = (var_devsign * eq22_e2138_d_n8);
        let eq22_e2139_d_n9: f64 = (var_devsign * eq22_e2138_d_n9);
        let eq22_e2139_d_n10: f64 = (var_devsign * eq22_e2138_d_n10);
        let eq22_e2139_d_n11: f64 = (var_devsign * eq22_e2138_d_n11);
        let eq22_e2139_d_n13: f64 = (var_devsign * eq22_e2138_d_n13);
        let eq22_e2139_d_n14: f64 = (var_devsign * eq22_e2138_d_n14);
        (eq22_e2139, eq22_e2139_d_n0, eq22_e2139_d_n2, eq22_e2139_d_n3, eq22_e2139_d_n4, eq22_e2139_d_n5, eq22_e2139_d_n6, eq22_e2139_d_n7, eq22_e2139_d_n8, eq22_e2139_d_n9, eq22_e2139_d_n10, eq22_e2139_d_n11, eq22_e2139_d_n13, eq22_e2139_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq22_value: f64 = eq22_e2141;
        let eq22_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq22_node_derivatives: [f64; 13] = [eq22_e2141_d_n0, eq22_e2141_d_n2, eq22_e2141_d_n3, eq22_e2141_d_n4, eq22_e2141_d_n5, eq22_e2141_d_n6, eq22_e2141_d_n7, eq22_e2141_d_n8, eq22_e2141_d_n9, eq22_e2141_d_n10, eq22_e2141_d_n11, eq22_e2141_d_n13, eq22_e2141_d_n14];
        let eq22_branch_derivative_indices: [usize; 0] = [];
        let eq22_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq22_value),
            &eq22_node_derivative_indices,
            &eq22_node_derivatives,
            &eq22_branch_derivative_indices,
            &eq22_branch_derivatives,
            multiplicity,
        );
        let (eq23_e2154, eq23_e2154_d_n0, eq23_e2154_d_n2, eq23_e2154_d_n3, eq23_e2154_d_n4, eq23_e2154_d_n5, eq23_e2154_d_n6, eq23_e2154_d_n7, eq23_e2154_d_n8, eq23_e2154_d_n9, eq23_e2154_d_n10, eq23_e2154_d_n11, eq23_e2154_d_n13, eq23_e2154_d_n14,) = {
    if (((var_guard644 == 0.0) && (var_guard647 != 0.0)) && (var_guard648 != 0.0)) {
        let eq23_e2151: f64 = (var_igidl_v + var_iii_1);
        let eq23_e2151_d_n0: f64 = (var_igidl_v_dn0 + var_iii_1_dn0);
        let eq23_e2151_d_n2: f64 = (var_igidl_v_dn2 + var_iii_1_dn2);
        let eq23_e2151_d_n3: f64 = (var_igidl_v_dn3 + var_iii_1_dn3);
        let eq23_e2151_d_n4: f64 = (var_igidl_v_dn4 + var_iii_1_dn4);
        let eq23_e2151_d_n5: f64 = (var_igidl_v_dn5 + var_iii_1_dn5);
        let eq23_e2151_d_n6: f64 = (var_igidl_v_dn6 + var_iii_1_dn6);
        let eq23_e2151_d_n7: f64 = (var_igidl_v_dn7 + var_iii_1_dn7);
        let eq23_e2151_d_n8: f64 = (var_igidl_v_dn8 + var_iii_1_dn8);
        let eq23_e2151_d_n9: f64 = (var_igidl_v_dn9 + var_iii_1_dn9);
        let eq23_e2151_d_n10: f64 = (var_igidl_v_dn10 + var_iii_1_dn10);
        let eq23_e2151_d_n11: f64 = (var_igidl_v_dn11 + var_iii_1_dn11);
        let eq23_e2151_d_n13: f64 = (var_igidl_v_dn13 + var_iii_1_dn13);
        let eq23_e2151_d_n14: f64 = (var_igidl_v_dn14 + var_iii_1_dn14);
        let eq23_e2152: f64 = (var_devsign * eq23_e2151);
        let eq23_e2152_d_n0: f64 = (var_devsign * eq23_e2151_d_n0);
        let eq23_e2152_d_n2: f64 = (var_devsign * eq23_e2151_d_n2);
        let eq23_e2152_d_n3: f64 = (var_devsign * eq23_e2151_d_n3);
        let eq23_e2152_d_n4: f64 = (var_devsign * eq23_e2151_d_n4);
        let eq23_e2152_d_n5: f64 = (var_devsign * eq23_e2151_d_n5);
        let eq23_e2152_d_n6: f64 = (var_devsign * eq23_e2151_d_n6);
        let eq23_e2152_d_n7: f64 = (var_devsign * eq23_e2151_d_n7);
        let eq23_e2152_d_n8: f64 = (var_devsign * eq23_e2151_d_n8);
        let eq23_e2152_d_n9: f64 = (var_devsign * eq23_e2151_d_n9);
        let eq23_e2152_d_n10: f64 = (var_devsign * eq23_e2151_d_n10);
        let eq23_e2152_d_n11: f64 = (var_devsign * eq23_e2151_d_n11);
        let eq23_e2152_d_n13: f64 = (var_devsign * eq23_e2151_d_n13);
        let eq23_e2152_d_n14: f64 = (var_devsign * eq23_e2151_d_n14);
        (eq23_e2152, eq23_e2152_d_n0, eq23_e2152_d_n2, eq23_e2152_d_n3, eq23_e2152_d_n4, eq23_e2152_d_n5, eq23_e2152_d_n6, eq23_e2152_d_n7, eq23_e2152_d_n8, eq23_e2152_d_n9, eq23_e2152_d_n10, eq23_e2152_d_n11, eq23_e2152_d_n13, eq23_e2152_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e2154;
        let eq23_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq23_node_derivatives: [f64; 13] = [eq23_e2154_d_n0, eq23_e2154_d_n2, eq23_e2154_d_n3, eq23_e2154_d_n4, eq23_e2154_d_n5, eq23_e2154_d_n6, eq23_e2154_d_n7, eq23_e2154_d_n8, eq23_e2154_d_n9, eq23_e2154_d_n10, eq23_e2154_d_n11, eq23_e2154_d_n13, eq23_e2154_d_n14];
        let eq23_branch_derivative_indices: [usize; 0] = [];
        let eq23_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq23_value),
            &eq23_node_derivative_indices,
            &eq23_node_derivatives,
            &eq23_branch_derivative_indices,
            &eq23_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        var_devsign: f64,
        var_gmin: f64,
        var_guard644: f64,
        var_guard647: f64,
        var_guard648: f64,
        var_guard649: f64,
        var_guard650: f64,
        var_ied: f64,
        var_ied_dn0: f64,
        var_ied_dn10: f64,
        var_ied_dn11: f64,
        var_ied_dn13: f64,
        var_ied_dn14: f64,
        var_ied_dn2: f64,
        var_ied_dn3: f64,
        var_ied_dn4: f64,
        var_ied_dn5: f64,
        var_ied_dn6: f64,
        var_ied_dn7: f64,
        var_ied_dn8: f64,
        var_ied_dn9: f64,
        var_ies: f64,
        var_ies_dn0: f64,
        var_ies_dn10: f64,
        var_ies_dn11: f64,
        var_ies_dn13: f64,
        var_ies_dn14: f64,
        var_ies_dn2: f64,
        var_ies_dn3: f64,
        var_ies_dn4: f64,
        var_ies_dn5: f64,
        var_ies_dn6: f64,
        var_ies_dn7: f64,
        var_ies_dn8: f64,
        var_ies_dn9: f64,
        var_igbacc_v: f64,
        var_igbacc_v_dn0: f64,
        var_igbacc_v_dn10: f64,
        var_igbacc_v_dn11: f64,
        var_igbacc_v_dn13: f64,
        var_igbacc_v_dn14: f64,
        var_igbacc_v_dn2: f64,
        var_igbacc_v_dn3: f64,
        var_igbacc_v_dn4: f64,
        var_igbacc_v_dn5: f64,
        var_igbacc_v_dn6: f64,
        var_igbacc_v_dn7: f64,
        var_igbacc_v_dn8: f64,
        var_igbacc_v_dn9: f64,
        var_igbd_v: f64,
        var_igbd_v_dn0: f64,
        var_igbd_v_dn10: f64,
        var_igbd_v_dn11: f64,
        var_igbd_v_dn13: f64,
        var_igbd_v_dn14: f64,
        var_igbd_v_dn2: f64,
        var_igbd_v_dn3: f64,
        var_igbd_v_dn4: f64,
        var_igbd_v_dn5: f64,
        var_igbd_v_dn6: f64,
        var_igbd_v_dn7: f64,
        var_igbd_v_dn8: f64,
        var_igbd_v_dn9: f64,
        var_igbinv_v: f64,
        var_igbinv_v_dn0: f64,
        var_igbinv_v_dn10: f64,
        var_igbinv_v_dn11: f64,
        var_igbinv_v_dn13: f64,
        var_igbinv_v_dn14: f64,
        var_igbinv_v_dn2: f64,
        var_igbinv_v_dn3: f64,
        var_igbinv_v_dn4: f64,
        var_igbinv_v_dn5: f64,
        var_igbinv_v_dn6: f64,
        var_igbinv_v_dn7: f64,
        var_igbinv_v_dn8: f64,
        var_igbinv_v_dn9: f64,
        var_igbs_v: f64,
        var_igbs_v_dn0: f64,
        var_igbs_v_dn10: f64,
        var_igbs_v_dn11: f64,
        var_igbs_v_dn13: f64,
        var_igbs_v_dn14: f64,
        var_igbs_v_dn2: f64,
        var_igbs_v_dn3: f64,
        var_igbs_v_dn4: f64,
        var_igbs_v_dn5: f64,
        var_igbs_v_dn6: f64,
        var_igbs_v_dn7: f64,
        var_igbs_v_dn8: f64,
        var_igbs_v_dn9: f64,
        var_igidl_v: f64,
        var_igidl_v_dn0: f64,
        var_igidl_v_dn10: f64,
        var_igidl_v_dn11: f64,
        var_igidl_v_dn13: f64,
        var_igidl_v_dn14: f64,
        var_igidl_v_dn2: f64,
        var_igidl_v_dn3: f64,
        var_igidl_v_dn4: f64,
        var_igidl_v_dn5: f64,
        var_igidl_v_dn6: f64,
        var_igidl_v_dn7: f64,
        var_igidl_v_dn8: f64,
        var_igidl_v_dn9: f64,
        var_igidlb: f64,
        var_igidlb_dn0: f64,
        var_igidlb_dn10: f64,
        var_igidlb_dn11: f64,
        var_igidlb_dn13: f64,
        var_igidlb_dn14: f64,
        var_igidlb_dn2: f64,
        var_igidlb_dn3: f64,
        var_igidlb_dn4: f64,
        var_igidlb_dn5: f64,
        var_igidlb_dn6: f64,
        var_igidlb_dn7: f64,
        var_igidlb_dn8: f64,
        var_igidlb_dn9: f64,
        var_igisl_v: f64,
        var_igisl_v_dn0: f64,
        var_igisl_v_dn10: f64,
        var_igisl_v_dn11: f64,
        var_igisl_v_dn13: f64,
        var_igisl_v_dn14: f64,
        var_igisl_v_dn2: f64,
        var_igisl_v_dn3: f64,
        var_igisl_v_dn4: f64,
        var_igisl_v_dn5: f64,
        var_igisl_v_dn6: f64,
        var_igisl_v_dn7: f64,
        var_igisl_v_dn8: f64,
        var_igisl_v_dn9: f64,
        var_igislb: f64,
        var_igislb_dn0: f64,
        var_igislb_dn10: f64,
        var_igislb_dn11: f64,
        var_igislb_dn13: f64,
        var_igislb_dn14: f64,
        var_igislb_dn2: f64,
        var_igislb_dn3: f64,
        var_igislb_dn4: f64,
        var_igislb_dn5: f64,
        var_igislb_dn6: f64,
        var_igislb_dn7: f64,
        var_igislb_dn8: f64,
        var_igislb_dn9: f64,
        var_iii_1: f64,
        var_iii_1_dn0: f64,
        var_iii_1_dn10: f64,
        var_iii_1_dn11: f64,
        var_iii_1_dn13: f64,
        var_iii_1_dn14: f64,
        var_iii_1_dn2: f64,
        var_iii_1_dn3: f64,
        var_iii_1_dn4: f64,
        var_iii_1_dn5: f64,
        var_iii_1_dn6: f64,
        var_iii_1_dn7: f64,
        var_iii_1_dn8: f64,
        var_iii_1_dn9: f64,
        var_qes: f64,
        var_qes_dn0: f64,
        var_qes_dn10: f64,
        var_qes_dn11: f64,
        var_qes_dn13: f64,
        var_qes_dn14: f64,
        var_qes_dn2: f64,
        var_qes_dn3: f64,
        var_qes_dn4: f64,
        var_qes_dn5: f64,
        var_qes_dn6: f64,
        var_qes_dn7: f64,
        var_qes_dn8: f64,
        var_qes_dn9: f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq24_e2165, eq24_e2165_d_n0, eq24_e2165_d_n2, eq24_e2165_d_n3, eq24_e2165_d_n4, eq24_e2165_d_n5, eq24_e2165_d_n6, eq24_e2165_d_n7, eq24_e2165_d_n8, eq24_e2165_d_n9, eq24_e2165_d_n10, eq24_e2165_d_n11, eq24_e2165_d_n13, eq24_e2165_d_n14,) = {
    if (((var_guard644 == 0.0) && (var_guard647 != 0.0)) && (var_guard648 != 0.0)) {
        let eq24_e2163: f64 = (var_devsign * var_igisl_v);
        let eq24_e2163_d_n0: f64 = (var_devsign * var_igisl_v_dn0);
        let eq24_e2163_d_n2: f64 = (var_devsign * var_igisl_v_dn2);
        let eq24_e2163_d_n3: f64 = (var_devsign * var_igisl_v_dn3);
        let eq24_e2163_d_n4: f64 = (var_devsign * var_igisl_v_dn4);
        let eq24_e2163_d_n5: f64 = (var_devsign * var_igisl_v_dn5);
        let eq24_e2163_d_n6: f64 = (var_devsign * var_igisl_v_dn6);
        let eq24_e2163_d_n7: f64 = (var_devsign * var_igisl_v_dn7);
        let eq24_e2163_d_n8: f64 = (var_devsign * var_igisl_v_dn8);
        let eq24_e2163_d_n9: f64 = (var_devsign * var_igisl_v_dn9);
        let eq24_e2163_d_n10: f64 = (var_devsign * var_igisl_v_dn10);
        let eq24_e2163_d_n11: f64 = (var_devsign * var_igisl_v_dn11);
        let eq24_e2163_d_n13: f64 = (var_devsign * var_igisl_v_dn13);
        let eq24_e2163_d_n14: f64 = (var_devsign * var_igisl_v_dn14);
        (eq24_e2163, eq24_e2163_d_n0, eq24_e2163_d_n2, eq24_e2163_d_n3, eq24_e2163_d_n4, eq24_e2163_d_n5, eq24_e2163_d_n6, eq24_e2163_d_n7, eq24_e2163_d_n8, eq24_e2163_d_n9, eq24_e2163_d_n10, eq24_e2163_d_n11, eq24_e2163_d_n13, eq24_e2163_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e2165;
        let eq24_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq24_node_derivatives: [f64; 13] = [eq24_e2165_d_n0, eq24_e2165_d_n2, eq24_e2165_d_n3, eq24_e2165_d_n4, eq24_e2165_d_n5, eq24_e2165_d_n6, eq24_e2165_d_n7, eq24_e2165_d_n8, eq24_e2165_d_n9, eq24_e2165_d_n10, eq24_e2165_d_n11, eq24_e2165_d_n13, eq24_e2165_d_n14];
        let eq24_branch_derivative_indices: [usize; 0] = [];
        let eq24_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq24_value),
            &eq24_node_derivative_indices,
            &eq24_node_derivatives,
            &eq24_branch_derivative_indices,
            &eq24_branch_derivatives,
            multiplicity,
        );
        let (eq25_e2176, eq25_e2176_d_n0, eq25_e2176_d_n2, eq25_e2176_d_n3, eq25_e2176_d_n4, eq25_e2176_d_n5, eq25_e2176_d_n6, eq25_e2176_d_n7, eq25_e2176_d_n8, eq25_e2176_d_n9, eq25_e2176_d_n10, eq25_e2176_d_n11, eq25_e2176_d_n13, eq25_e2176_d_n14,) = {
    if (((var_guard644 == 0.0) && (var_guard647 != 0.0)) && (var_guard648 != 0.0)) {
        let eq25_e2174: f64 = (var_devsign * var_igidlb);
        let eq25_e2174_d_n0: f64 = (var_devsign * var_igidlb_dn0);
        let eq25_e2174_d_n2: f64 = (var_devsign * var_igidlb_dn2);
        let eq25_e2174_d_n3: f64 = (var_devsign * var_igidlb_dn3);
        let eq25_e2174_d_n4: f64 = (var_devsign * var_igidlb_dn4);
        let eq25_e2174_d_n5: f64 = (var_devsign * var_igidlb_dn5);
        let eq25_e2174_d_n6: f64 = (var_devsign * var_igidlb_dn6);
        let eq25_e2174_d_n7: f64 = (var_devsign * var_igidlb_dn7);
        let eq25_e2174_d_n8: f64 = (var_devsign * var_igidlb_dn8);
        let eq25_e2174_d_n9: f64 = (var_devsign * var_igidlb_dn9);
        let eq25_e2174_d_n10: f64 = (var_devsign * var_igidlb_dn10);
        let eq25_e2174_d_n11: f64 = (var_devsign * var_igidlb_dn11);
        let eq25_e2174_d_n13: f64 = (var_devsign * var_igidlb_dn13);
        let eq25_e2174_d_n14: f64 = (var_devsign * var_igidlb_dn14);
        (eq25_e2174, eq25_e2174_d_n0, eq25_e2174_d_n2, eq25_e2174_d_n3, eq25_e2174_d_n4, eq25_e2174_d_n5, eq25_e2174_d_n6, eq25_e2174_d_n7, eq25_e2174_d_n8, eq25_e2174_d_n9, eq25_e2174_d_n10, eq25_e2174_d_n11, eq25_e2174_d_n13, eq25_e2174_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e2176;
        let eq25_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq25_node_derivatives: [f64; 13] = [eq25_e2176_d_n0, eq25_e2176_d_n2, eq25_e2176_d_n3, eq25_e2176_d_n4, eq25_e2176_d_n5, eq25_e2176_d_n6, eq25_e2176_d_n7, eq25_e2176_d_n8, eq25_e2176_d_n9, eq25_e2176_d_n10, eq25_e2176_d_n11, eq25_e2176_d_n13, eq25_e2176_d_n14];
        let eq25_branch_derivative_indices: [usize; 0] = [];
        let eq25_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(3),
            multiplicity * (eq25_value),
            &eq25_node_derivative_indices,
            &eq25_node_derivatives,
            &eq25_branch_derivative_indices,
            &eq25_branch_derivatives,
            multiplicity,
        );
        let (eq26_e2187, eq26_e2187_d_n0, eq26_e2187_d_n2, eq26_e2187_d_n3, eq26_e2187_d_n4, eq26_e2187_d_n5, eq26_e2187_d_n6, eq26_e2187_d_n7, eq26_e2187_d_n8, eq26_e2187_d_n9, eq26_e2187_d_n10, eq26_e2187_d_n11, eq26_e2187_d_n13, eq26_e2187_d_n14,) = {
    if (((var_guard644 == 0.0) && (var_guard647 != 0.0)) && (var_guard648 != 0.0)) {
        let eq26_e2185: f64 = (var_devsign * var_igislb);
        let eq26_e2185_d_n0: f64 = (var_devsign * var_igislb_dn0);
        let eq26_e2185_d_n2: f64 = (var_devsign * var_igislb_dn2);
        let eq26_e2185_d_n3: f64 = (var_devsign * var_igislb_dn3);
        let eq26_e2185_d_n4: f64 = (var_devsign * var_igislb_dn4);
        let eq26_e2185_d_n5: f64 = (var_devsign * var_igislb_dn5);
        let eq26_e2185_d_n6: f64 = (var_devsign * var_igislb_dn6);
        let eq26_e2185_d_n7: f64 = (var_devsign * var_igislb_dn7);
        let eq26_e2185_d_n8: f64 = (var_devsign * var_igislb_dn8);
        let eq26_e2185_d_n9: f64 = (var_devsign * var_igislb_dn9);
        let eq26_e2185_d_n10: f64 = (var_devsign * var_igislb_dn10);
        let eq26_e2185_d_n11: f64 = (var_devsign * var_igislb_dn11);
        let eq26_e2185_d_n13: f64 = (var_devsign * var_igislb_dn13);
        let eq26_e2185_d_n14: f64 = (var_devsign * var_igislb_dn14);
        (eq26_e2185, eq26_e2185_d_n0, eq26_e2185_d_n2, eq26_e2185_d_n3, eq26_e2185_d_n4, eq26_e2185_d_n5, eq26_e2185_d_n6, eq26_e2185_d_n7, eq26_e2185_d_n8, eq26_e2185_d_n9, eq26_e2185_d_n10, eq26_e2185_d_n11, eq26_e2185_d_n13, eq26_e2185_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e2187;
        let eq26_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq26_node_derivatives: [f64; 13] = [eq26_e2187_d_n0, eq26_e2187_d_n2, eq26_e2187_d_n3, eq26_e2187_d_n4, eq26_e2187_d_n5, eq26_e2187_d_n6, eq26_e2187_d_n7, eq26_e2187_d_n8, eq26_e2187_d_n9, eq26_e2187_d_n10, eq26_e2187_d_n11, eq26_e2187_d_n13, eq26_e2187_d_n14];
        let eq26_branch_derivative_indices: [usize; 0] = [];
        let eq26_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(3),
            multiplicity * (eq26_value),
            &eq26_node_derivative_indices,
            &eq26_node_derivatives,
            &eq26_branch_derivative_indices,
            &eq26_branch_derivatives,
            multiplicity,
        );
        let (eq27_e2201, eq27_e2201_d_n0, eq27_e2201_d_n2, eq27_e2201_d_n3, eq27_e2201_d_n4, eq27_e2201_d_n5, eq27_e2201_d_n6, eq27_e2201_d_n7, eq27_e2201_d_n8, eq27_e2201_d_n9, eq27_e2201_d_n10, eq27_e2201_d_n11, eq27_e2201_d_n13, eq27_e2201_d_n14,) = {
    if (((var_guard644 == 0.0) && (var_guard647 != 0.0)) && (var_guard648 == 0.0)) {
        let eq27_e2198: f64 = (var_igidl_v + var_iii_1);
        let eq27_e2198_d_n0: f64 = (var_igidl_v_dn0 + var_iii_1_dn0);
        let eq27_e2198_d_n2: f64 = (var_igidl_v_dn2 + var_iii_1_dn2);
        let eq27_e2198_d_n3: f64 = (var_igidl_v_dn3 + var_iii_1_dn3);
        let eq27_e2198_d_n4: f64 = (var_igidl_v_dn4 + var_iii_1_dn4);
        let eq27_e2198_d_n5: f64 = (var_igidl_v_dn5 + var_iii_1_dn5);
        let eq27_e2198_d_n6: f64 = (var_igidl_v_dn6 + var_iii_1_dn6);
        let eq27_e2198_d_n7: f64 = (var_igidl_v_dn7 + var_iii_1_dn7);
        let eq27_e2198_d_n8: f64 = (var_igidl_v_dn8 + var_iii_1_dn8);
        let eq27_e2198_d_n9: f64 = (var_igidl_v_dn9 + var_iii_1_dn9);
        let eq27_e2198_d_n10: f64 = (var_igidl_v_dn10 + var_iii_1_dn10);
        let eq27_e2198_d_n11: f64 = (var_igidl_v_dn11 + var_iii_1_dn11);
        let eq27_e2198_d_n13: f64 = (var_igidl_v_dn13 + var_iii_1_dn13);
        let eq27_e2198_d_n14: f64 = (var_igidl_v_dn14 + var_iii_1_dn14);
        let eq27_e2199: f64 = (var_devsign * eq27_e2198);
        let eq27_e2199_d_n0: f64 = (var_devsign * eq27_e2198_d_n0);
        let eq27_e2199_d_n2: f64 = (var_devsign * eq27_e2198_d_n2);
        let eq27_e2199_d_n3: f64 = (var_devsign * eq27_e2198_d_n3);
        let eq27_e2199_d_n4: f64 = (var_devsign * eq27_e2198_d_n4);
        let eq27_e2199_d_n5: f64 = (var_devsign * eq27_e2198_d_n5);
        let eq27_e2199_d_n6: f64 = (var_devsign * eq27_e2198_d_n6);
        let eq27_e2199_d_n7: f64 = (var_devsign * eq27_e2198_d_n7);
        let eq27_e2199_d_n8: f64 = (var_devsign * eq27_e2198_d_n8);
        let eq27_e2199_d_n9: f64 = (var_devsign * eq27_e2198_d_n9);
        let eq27_e2199_d_n10: f64 = (var_devsign * eq27_e2198_d_n10);
        let eq27_e2199_d_n11: f64 = (var_devsign * eq27_e2198_d_n11);
        let eq27_e2199_d_n13: f64 = (var_devsign * eq27_e2198_d_n13);
        let eq27_e2199_d_n14: f64 = (var_devsign * eq27_e2198_d_n14);
        (eq27_e2199, eq27_e2199_d_n0, eq27_e2199_d_n2, eq27_e2199_d_n3, eq27_e2199_d_n4, eq27_e2199_d_n5, eq27_e2199_d_n6, eq27_e2199_d_n7, eq27_e2199_d_n8, eq27_e2199_d_n9, eq27_e2199_d_n10, eq27_e2199_d_n11, eq27_e2199_d_n13, eq27_e2199_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e2201;
        let eq27_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq27_node_derivatives: [f64; 13] = [eq27_e2201_d_n0, eq27_e2201_d_n2, eq27_e2201_d_n3, eq27_e2201_d_n4, eq27_e2201_d_n5, eq27_e2201_d_n6, eq27_e2201_d_n7, eq27_e2201_d_n8, eq27_e2201_d_n9, eq27_e2201_d_n10, eq27_e2201_d_n11, eq27_e2201_d_n13, eq27_e2201_d_n14];
        let eq27_branch_derivative_indices: [usize; 0] = [];
        let eq27_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(3),
            multiplicity * (eq27_value),
            &eq27_node_derivative_indices,
            &eq27_node_derivatives,
            &eq27_branch_derivative_indices,
            &eq27_branch_derivatives,
            multiplicity,
        );
        let (eq28_e2213, eq28_e2213_d_n0, eq28_e2213_d_n2, eq28_e2213_d_n3, eq28_e2213_d_n4, eq28_e2213_d_n5, eq28_e2213_d_n6, eq28_e2213_d_n7, eq28_e2213_d_n8, eq28_e2213_d_n9, eq28_e2213_d_n10, eq28_e2213_d_n11, eq28_e2213_d_n13, eq28_e2213_d_n14,) = {
    if (((var_guard644 == 0.0) && (var_guard647 != 0.0)) && (var_guard648 == 0.0)) {
        let eq28_e2211: f64 = (var_devsign * var_igisl_v);
        let eq28_e2211_d_n0: f64 = (var_devsign * var_igisl_v_dn0);
        let eq28_e2211_d_n2: f64 = (var_devsign * var_igisl_v_dn2);
        let eq28_e2211_d_n3: f64 = (var_devsign * var_igisl_v_dn3);
        let eq28_e2211_d_n4: f64 = (var_devsign * var_igisl_v_dn4);
        let eq28_e2211_d_n5: f64 = (var_devsign * var_igisl_v_dn5);
        let eq28_e2211_d_n6: f64 = (var_devsign * var_igisl_v_dn6);
        let eq28_e2211_d_n7: f64 = (var_devsign * var_igisl_v_dn7);
        let eq28_e2211_d_n8: f64 = (var_devsign * var_igisl_v_dn8);
        let eq28_e2211_d_n9: f64 = (var_devsign * var_igisl_v_dn9);
        let eq28_e2211_d_n10: f64 = (var_devsign * var_igisl_v_dn10);
        let eq28_e2211_d_n11: f64 = (var_devsign * var_igisl_v_dn11);
        let eq28_e2211_d_n13: f64 = (var_devsign * var_igisl_v_dn13);
        let eq28_e2211_d_n14: f64 = (var_devsign * var_igisl_v_dn14);
        (eq28_e2211, eq28_e2211_d_n0, eq28_e2211_d_n2, eq28_e2211_d_n3, eq28_e2211_d_n4, eq28_e2211_d_n5, eq28_e2211_d_n6, eq28_e2211_d_n7, eq28_e2211_d_n8, eq28_e2211_d_n9, eq28_e2211_d_n10, eq28_e2211_d_n11, eq28_e2211_d_n13, eq28_e2211_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e2213;
        let eq28_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq28_node_derivatives: [f64; 13] = [eq28_e2213_d_n0, eq28_e2213_d_n2, eq28_e2213_d_n3, eq28_e2213_d_n4, eq28_e2213_d_n5, eq28_e2213_d_n6, eq28_e2213_d_n7, eq28_e2213_d_n8, eq28_e2213_d_n9, eq28_e2213_d_n10, eq28_e2213_d_n11, eq28_e2213_d_n13, eq28_e2213_d_n14];
        let eq28_branch_derivative_indices: [usize; 0] = [];
        let eq28_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(3),
            multiplicity * (eq28_value),
            &eq28_node_derivative_indices,
            &eq28_node_derivatives,
            &eq28_branch_derivative_indices,
            &eq28_branch_derivatives,
            multiplicity,
        );
        let (eq29_e2224, eq29_e2224_d_n0, eq29_e2224_d_n2, eq29_e2224_d_n3, eq29_e2224_d_n4, eq29_e2224_d_n5, eq29_e2224_d_n6, eq29_e2224_d_n7, eq29_e2224_d_n8, eq29_e2224_d_n9, eq29_e2224_d_n10, eq29_e2224_d_n11, eq29_e2224_d_n13, eq29_e2224_d_n14,) = {
    if ((var_guard644 == 0.0) && (var_guard647 != 0.0)) {
        let eq29_e2221: f64 = (var_igbinv_v + var_igbacc_v);
        let eq29_e2221_d_n0: f64 = (var_igbinv_v_dn0 + var_igbacc_v_dn0);
        let eq29_e2221_d_n2: f64 = (var_igbinv_v_dn2 + var_igbacc_v_dn2);
        let eq29_e2221_d_n3: f64 = (var_igbinv_v_dn3 + var_igbacc_v_dn3);
        let eq29_e2221_d_n4: f64 = (var_igbinv_v_dn4 + var_igbacc_v_dn4);
        let eq29_e2221_d_n5: f64 = (var_igbinv_v_dn5 + var_igbacc_v_dn5);
        let eq29_e2221_d_n6: f64 = (var_igbinv_v_dn6 + var_igbacc_v_dn6);
        let eq29_e2221_d_n7: f64 = (var_igbinv_v_dn7 + var_igbacc_v_dn7);
        let eq29_e2221_d_n8: f64 = (var_igbinv_v_dn8 + var_igbacc_v_dn8);
        let eq29_e2221_d_n9: f64 = (var_igbinv_v_dn9 + var_igbacc_v_dn9);
        let eq29_e2221_d_n10: f64 = (var_igbinv_v_dn10 + var_igbacc_v_dn10);
        let eq29_e2221_d_n11: f64 = (var_igbinv_v_dn11 + var_igbacc_v_dn11);
        let eq29_e2221_d_n13: f64 = (var_igbinv_v_dn13 + var_igbacc_v_dn13);
        let eq29_e2221_d_n14: f64 = (var_igbinv_v_dn14 + var_igbacc_v_dn14);
        let eq29_e2222: f64 = (var_devsign * eq29_e2221);
        let eq29_e2222_d_n0: f64 = (var_devsign * eq29_e2221_d_n0);
        let eq29_e2222_d_n2: f64 = (var_devsign * eq29_e2221_d_n2);
        let eq29_e2222_d_n3: f64 = (var_devsign * eq29_e2221_d_n3);
        let eq29_e2222_d_n4: f64 = (var_devsign * eq29_e2221_d_n4);
        let eq29_e2222_d_n5: f64 = (var_devsign * eq29_e2221_d_n5);
        let eq29_e2222_d_n6: f64 = (var_devsign * eq29_e2221_d_n6);
        let eq29_e2222_d_n7: f64 = (var_devsign * eq29_e2221_d_n7);
        let eq29_e2222_d_n8: f64 = (var_devsign * eq29_e2221_d_n8);
        let eq29_e2222_d_n9: f64 = (var_devsign * eq29_e2221_d_n9);
        let eq29_e2222_d_n10: f64 = (var_devsign * eq29_e2221_d_n10);
        let eq29_e2222_d_n11: f64 = (var_devsign * eq29_e2221_d_n11);
        let eq29_e2222_d_n13: f64 = (var_devsign * eq29_e2221_d_n13);
        let eq29_e2222_d_n14: f64 = (var_devsign * eq29_e2221_d_n14);
        (eq29_e2222, eq29_e2222_d_n0, eq29_e2222_d_n2, eq29_e2222_d_n3, eq29_e2222_d_n4, eq29_e2222_d_n5, eq29_e2222_d_n6, eq29_e2222_d_n7, eq29_e2222_d_n8, eq29_e2222_d_n9, eq29_e2222_d_n10, eq29_e2222_d_n11, eq29_e2222_d_n13, eq29_e2222_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e2224;
        let eq29_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq29_node_derivatives: [f64; 13] = [eq29_e2224_d_n0, eq29_e2224_d_n2, eq29_e2224_d_n3, eq29_e2224_d_n4, eq29_e2224_d_n5, eq29_e2224_d_n6, eq29_e2224_d_n7, eq29_e2224_d_n8, eq29_e2224_d_n9, eq29_e2224_d_n10, eq29_e2224_d_n11, eq29_e2224_d_n13, eq29_e2224_d_n14];
        let eq29_branch_derivative_indices: [usize; 0] = [];
        let eq29_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(3),
            multiplicity * (eq29_value),
            &eq29_node_derivative_indices,
            &eq29_node_derivatives,
            &eq29_branch_derivative_indices,
            &eq29_branch_derivatives,
            multiplicity,
        );
        let (eq30_e2236, eq30_e2236_d_n0, eq30_e2236_d_n2, eq30_e2236_d_n3, eq30_e2236_d_n4, eq30_e2236_d_n5, eq30_e2236_d_n6, eq30_e2236_d_n7, eq30_e2236_d_n8, eq30_e2236_d_n9, eq30_e2236_d_n10, eq30_e2236_d_n11, eq30_e2236_d_n13, eq30_e2236_d_n14,) = {
    if ((var_guard644 == 0.0) && (var_guard647 == 0.0)) {
        let eq30_e2233: f64 = (var_igidl_v + var_iii_1);
        let eq30_e2233_d_n0: f64 = (var_igidl_v_dn0 + var_iii_1_dn0);
        let eq30_e2233_d_n2: f64 = (var_igidl_v_dn2 + var_iii_1_dn2);
        let eq30_e2233_d_n3: f64 = (var_igidl_v_dn3 + var_iii_1_dn3);
        let eq30_e2233_d_n4: f64 = (var_igidl_v_dn4 + var_iii_1_dn4);
        let eq30_e2233_d_n5: f64 = (var_igidl_v_dn5 + var_iii_1_dn5);
        let eq30_e2233_d_n6: f64 = (var_igidl_v_dn6 + var_iii_1_dn6);
        let eq30_e2233_d_n7: f64 = (var_igidl_v_dn7 + var_iii_1_dn7);
        let eq30_e2233_d_n8: f64 = (var_igidl_v_dn8 + var_iii_1_dn8);
        let eq30_e2233_d_n9: f64 = (var_igidl_v_dn9 + var_iii_1_dn9);
        let eq30_e2233_d_n10: f64 = (var_igidl_v_dn10 + var_iii_1_dn10);
        let eq30_e2233_d_n11: f64 = (var_igidl_v_dn11 + var_iii_1_dn11);
        let eq30_e2233_d_n13: f64 = (var_igidl_v_dn13 + var_iii_1_dn13);
        let eq30_e2233_d_n14: f64 = (var_igidl_v_dn14 + var_iii_1_dn14);
        let eq30_e2234: f64 = (var_devsign * eq30_e2233);
        let eq30_e2234_d_n0: f64 = (var_devsign * eq30_e2233_d_n0);
        let eq30_e2234_d_n2: f64 = (var_devsign * eq30_e2233_d_n2);
        let eq30_e2234_d_n3: f64 = (var_devsign * eq30_e2233_d_n3);
        let eq30_e2234_d_n4: f64 = (var_devsign * eq30_e2233_d_n4);
        let eq30_e2234_d_n5: f64 = (var_devsign * eq30_e2233_d_n5);
        let eq30_e2234_d_n6: f64 = (var_devsign * eq30_e2233_d_n6);
        let eq30_e2234_d_n7: f64 = (var_devsign * eq30_e2233_d_n7);
        let eq30_e2234_d_n8: f64 = (var_devsign * eq30_e2233_d_n8);
        let eq30_e2234_d_n9: f64 = (var_devsign * eq30_e2233_d_n9);
        let eq30_e2234_d_n10: f64 = (var_devsign * eq30_e2233_d_n10);
        let eq30_e2234_d_n11: f64 = (var_devsign * eq30_e2233_d_n11);
        let eq30_e2234_d_n13: f64 = (var_devsign * eq30_e2233_d_n13);
        let eq30_e2234_d_n14: f64 = (var_devsign * eq30_e2233_d_n14);
        (eq30_e2234, eq30_e2234_d_n0, eq30_e2234_d_n2, eq30_e2234_d_n3, eq30_e2234_d_n4, eq30_e2234_d_n5, eq30_e2234_d_n6, eq30_e2234_d_n7, eq30_e2234_d_n8, eq30_e2234_d_n9, eq30_e2234_d_n10, eq30_e2234_d_n11, eq30_e2234_d_n13, eq30_e2234_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e2236;
        let eq30_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq30_node_derivatives: [f64; 13] = [eq30_e2236_d_n0, eq30_e2236_d_n2, eq30_e2236_d_n3, eq30_e2236_d_n4, eq30_e2236_d_n5, eq30_e2236_d_n6, eq30_e2236_d_n7, eq30_e2236_d_n8, eq30_e2236_d_n9, eq30_e2236_d_n10, eq30_e2236_d_n11, eq30_e2236_d_n13, eq30_e2236_d_n14];
        let eq30_branch_derivative_indices: [usize; 0] = [];
        let eq30_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq30_value),
            &eq30_node_derivative_indices,
            &eq30_node_derivatives,
            &eq30_branch_derivative_indices,
            &eq30_branch_derivatives,
            multiplicity,
        );
        let (eq31_e2246, eq31_e2246_d_n0, eq31_e2246_d_n2, eq31_e2246_d_n3, eq31_e2246_d_n4, eq31_e2246_d_n5, eq31_e2246_d_n6, eq31_e2246_d_n7, eq31_e2246_d_n8, eq31_e2246_d_n9, eq31_e2246_d_n10, eq31_e2246_d_n11, eq31_e2246_d_n13, eq31_e2246_d_n14,) = {
    if ((var_guard644 == 0.0) && (var_guard647 == 0.0)) {
        let eq31_e2244: f64 = (var_devsign * var_igisl_v);
        let eq31_e2244_d_n0: f64 = (var_devsign * var_igisl_v_dn0);
        let eq31_e2244_d_n2: f64 = (var_devsign * var_igisl_v_dn2);
        let eq31_e2244_d_n3: f64 = (var_devsign * var_igisl_v_dn3);
        let eq31_e2244_d_n4: f64 = (var_devsign * var_igisl_v_dn4);
        let eq31_e2244_d_n5: f64 = (var_devsign * var_igisl_v_dn5);
        let eq31_e2244_d_n6: f64 = (var_devsign * var_igisl_v_dn6);
        let eq31_e2244_d_n7: f64 = (var_devsign * var_igisl_v_dn7);
        let eq31_e2244_d_n8: f64 = (var_devsign * var_igisl_v_dn8);
        let eq31_e2244_d_n9: f64 = (var_devsign * var_igisl_v_dn9);
        let eq31_e2244_d_n10: f64 = (var_devsign * var_igisl_v_dn10);
        let eq31_e2244_d_n11: f64 = (var_devsign * var_igisl_v_dn11);
        let eq31_e2244_d_n13: f64 = (var_devsign * var_igisl_v_dn13);
        let eq31_e2244_d_n14: f64 = (var_devsign * var_igisl_v_dn14);
        (eq31_e2244, eq31_e2244_d_n0, eq31_e2244_d_n2, eq31_e2244_d_n3, eq31_e2244_d_n4, eq31_e2244_d_n5, eq31_e2244_d_n6, eq31_e2244_d_n7, eq31_e2244_d_n8, eq31_e2244_d_n9, eq31_e2244_d_n10, eq31_e2244_d_n11, eq31_e2244_d_n13, eq31_e2244_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e2246;
        let eq31_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq31_node_derivatives: [f64; 13] = [eq31_e2246_d_n0, eq31_e2246_d_n2, eq31_e2246_d_n3, eq31_e2246_d_n4, eq31_e2246_d_n5, eq31_e2246_d_n6, eq31_e2246_d_n7, eq31_e2246_d_n8, eq31_e2246_d_n9, eq31_e2246_d_n10, eq31_e2246_d_n11, eq31_e2246_d_n13, eq31_e2246_d_n14];
        let eq31_branch_derivative_indices: [usize; 0] = [];
        let eq31_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq31_value),
            &eq31_node_derivative_indices,
            &eq31_node_derivatives,
            &eq31_branch_derivative_indices,
            &eq31_branch_derivatives,
            multiplicity,
        );
        let (eq32_e2252, eq32_e2252_d_n0, eq32_e2252_d_n2, eq32_e2252_d_n3, eq32_e2252_d_n4, eq32_e2252_d_n5, eq32_e2252_d_n6, eq32_e2252_d_n7, eq32_e2252_d_n8, eq32_e2252_d_n9, eq32_e2252_d_n10, eq32_e2252_d_n11, eq32_e2252_d_n13, eq32_e2252_d_n14,) = {
    if (var_guard649 != 0.0) {
        let eq32_e2250: f64 = (var_devsign * var_igbs_v);
        let eq32_e2250_d_n0: f64 = (var_devsign * var_igbs_v_dn0);
        let eq32_e2250_d_n2: f64 = (var_devsign * var_igbs_v_dn2);
        let eq32_e2250_d_n3: f64 = (var_devsign * var_igbs_v_dn3);
        let eq32_e2250_d_n4: f64 = (var_devsign * var_igbs_v_dn4);
        let eq32_e2250_d_n5: f64 = (var_devsign * var_igbs_v_dn5);
        let eq32_e2250_d_n6: f64 = (var_devsign * var_igbs_v_dn6);
        let eq32_e2250_d_n7: f64 = (var_devsign * var_igbs_v_dn7);
        let eq32_e2250_d_n8: f64 = (var_devsign * var_igbs_v_dn8);
        let eq32_e2250_d_n9: f64 = (var_devsign * var_igbs_v_dn9);
        let eq32_e2250_d_n10: f64 = (var_devsign * var_igbs_v_dn10);
        let eq32_e2250_d_n11: f64 = (var_devsign * var_igbs_v_dn11);
        let eq32_e2250_d_n13: f64 = (var_devsign * var_igbs_v_dn13);
        let eq32_e2250_d_n14: f64 = (var_devsign * var_igbs_v_dn14);
        (eq32_e2250, eq32_e2250_d_n0, eq32_e2250_d_n2, eq32_e2250_d_n3, eq32_e2250_d_n4, eq32_e2250_d_n5, eq32_e2250_d_n6, eq32_e2250_d_n7, eq32_e2250_d_n8, eq32_e2250_d_n9, eq32_e2250_d_n10, eq32_e2250_d_n11, eq32_e2250_d_n13, eq32_e2250_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e2252;
        let eq32_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq32_node_derivatives: [f64; 13] = [eq32_e2252_d_n0, eq32_e2252_d_n2, eq32_e2252_d_n3, eq32_e2252_d_n4, eq32_e2252_d_n5, eq32_e2252_d_n6, eq32_e2252_d_n7, eq32_e2252_d_n8, eq32_e2252_d_n9, eq32_e2252_d_n10, eq32_e2252_d_n11, eq32_e2252_d_n13, eq32_e2252_d_n14];
        let eq32_branch_derivative_indices: [usize; 0] = [];
        let eq32_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq32_value),
            &eq32_node_derivative_indices,
            &eq32_node_derivatives,
            &eq32_branch_derivative_indices,
            &eq32_branch_derivatives,
            multiplicity,
        );
        let (eq33_e2258, eq33_e2258_d_n0, eq33_e2258_d_n2, eq33_e2258_d_n3, eq33_e2258_d_n4, eq33_e2258_d_n5, eq33_e2258_d_n6, eq33_e2258_d_n7, eq33_e2258_d_n8, eq33_e2258_d_n9, eq33_e2258_d_n10, eq33_e2258_d_n11, eq33_e2258_d_n13, eq33_e2258_d_n14,) = {
    if (var_guard649 != 0.0) {
        let eq33_e2256: f64 = (var_devsign * var_igbd_v);
        let eq33_e2256_d_n0: f64 = (var_devsign * var_igbd_v_dn0);
        let eq33_e2256_d_n2: f64 = (var_devsign * var_igbd_v_dn2);
        let eq33_e2256_d_n3: f64 = (var_devsign * var_igbd_v_dn3);
        let eq33_e2256_d_n4: f64 = (var_devsign * var_igbd_v_dn4);
        let eq33_e2256_d_n5: f64 = (var_devsign * var_igbd_v_dn5);
        let eq33_e2256_d_n6: f64 = (var_devsign * var_igbd_v_dn6);
        let eq33_e2256_d_n7: f64 = (var_devsign * var_igbd_v_dn7);
        let eq33_e2256_d_n8: f64 = (var_devsign * var_igbd_v_dn8);
        let eq33_e2256_d_n9: f64 = (var_devsign * var_igbd_v_dn9);
        let eq33_e2256_d_n10: f64 = (var_devsign * var_igbd_v_dn10);
        let eq33_e2256_d_n11: f64 = (var_devsign * var_igbd_v_dn11);
        let eq33_e2256_d_n13: f64 = (var_devsign * var_igbd_v_dn13);
        let eq33_e2256_d_n14: f64 = (var_devsign * var_igbd_v_dn14);
        (eq33_e2256, eq33_e2256_d_n0, eq33_e2256_d_n2, eq33_e2256_d_n3, eq33_e2256_d_n4, eq33_e2256_d_n5, eq33_e2256_d_n6, eq33_e2256_d_n7, eq33_e2256_d_n8, eq33_e2256_d_n9, eq33_e2256_d_n10, eq33_e2256_d_n11, eq33_e2256_d_n13, eq33_e2256_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e2258;
        let eq33_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq33_node_derivatives: [f64; 13] = [eq33_e2258_d_n0, eq33_e2258_d_n2, eq33_e2258_d_n3, eq33_e2258_d_n4, eq33_e2258_d_n5, eq33_e2258_d_n6, eq33_e2258_d_n7, eq33_e2258_d_n8, eq33_e2258_d_n9, eq33_e2258_d_n10, eq33_e2258_d_n11, eq33_e2258_d_n13, eq33_e2258_d_n14];
        let eq33_branch_derivative_indices: [usize; 0] = [];
        let eq33_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(5),
            multiplicity * (eq33_value),
            &eq33_node_derivative_indices,
            &eq33_node_derivatives,
            &eq33_branch_derivative_indices,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let (eq34_e2268, eq34_e2268_d_n0, eq34_e2268_d_n2, eq34_e2268_d_n3, eq34_e2268_d_n4, eq34_e2268_d_n5, eq34_e2268_d_n6, eq34_e2268_d_n7, eq34_e2268_d_n8, eq34_e2268_d_n9, eq34_e2268_d_n10, eq34_e2268_d_n11, eq34_e2268_d_n13, eq34_e2268_d_n14,) = {
    if (var_guard650 != 0.0) {
        let eq34_e2262: f64 = (var_devsign * var_ies);
        let eq34_e2262_d_n0: f64 = (var_devsign * var_ies_dn0);
        let eq34_e2262_d_n2: f64 = (var_devsign * var_ies_dn2);
        let eq34_e2262_d_n3: f64 = (var_devsign * var_ies_dn3);
        let eq34_e2262_d_n4: f64 = (var_devsign * var_ies_dn4);
        let eq34_e2262_d_n5: f64 = (var_devsign * var_ies_dn5);
        let eq34_e2262_d_n6: f64 = (var_devsign * var_ies_dn6);
        let eq34_e2262_d_n7: f64 = (var_devsign * var_ies_dn7);
        let eq34_e2262_d_n8: f64 = (var_devsign * var_ies_dn8);
        let eq34_e2262_d_n9: f64 = (var_devsign * var_ies_dn9);
        let eq34_e2262_d_n10: f64 = (var_devsign * var_ies_dn10);
        let eq34_e2262_d_n11: f64 = (var_devsign * var_ies_dn11);
        let eq34_e2262_d_n13: f64 = (var_devsign * var_ies_dn13);
        let eq34_e2262_d_n14: f64 = (var_devsign * var_ies_dn14);
        let eq34_e2265: f64 = ((nv3 - nv6) * var_gmin);
        let eq34_e2266: f64 = (eq34_e2262 + eq34_e2265);
        let eq34_e2266_d_n3: f64 = (eq34_e2262_d_n3 + var_gmin);
        let eq34_e2266_d_n6: f64 = (eq34_e2262_d_n6 + (-var_gmin));
        (eq34_e2266, eq34_e2262_d_n0, eq34_e2262_d_n2, eq34_e2266_d_n3, eq34_e2262_d_n4, eq34_e2262_d_n5, eq34_e2266_d_n6, eq34_e2262_d_n7, eq34_e2262_d_n8, eq34_e2262_d_n9, eq34_e2262_d_n10, eq34_e2262_d_n11, eq34_e2262_d_n13, eq34_e2262_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e2268;
        let eq34_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq34_node_derivatives: [f64; 13] = [eq34_e2268_d_n0, eq34_e2268_d_n2, eq34_e2268_d_n3, eq34_e2268_d_n4, eq34_e2268_d_n5, eq34_e2268_d_n6, eq34_e2268_d_n7, eq34_e2268_d_n8, eq34_e2268_d_n9, eq34_e2268_d_n10, eq34_e2268_d_n11, eq34_e2268_d_n13, eq34_e2268_d_n14];
        let eq34_branch_derivative_indices: [usize; 0] = [];
        let eq34_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(6),
            multiplicity * (eq34_value),
            &eq34_node_derivative_indices,
            &eq34_node_derivatives,
            &eq34_branch_derivative_indices,
            &eq34_branch_derivatives,
            multiplicity,
        );
        let (eq35_e2278, eq35_e2278_d_n0, eq35_e2278_d_n2, eq35_e2278_d_n3, eq35_e2278_d_n4, eq35_e2278_d_n5, eq35_e2278_d_n6, eq35_e2278_d_n7, eq35_e2278_d_n8, eq35_e2278_d_n9, eq35_e2278_d_n10, eq35_e2278_d_n11, eq35_e2278_d_n13, eq35_e2278_d_n14,) = {
    if (var_guard650 != 0.0) {
        let eq35_e2272: f64 = (var_devsign * var_ied);
        let eq35_e2272_d_n0: f64 = (var_devsign * var_ied_dn0);
        let eq35_e2272_d_n2: f64 = (var_devsign * var_ied_dn2);
        let eq35_e2272_d_n3: f64 = (var_devsign * var_ied_dn3);
        let eq35_e2272_d_n4: f64 = (var_devsign * var_ied_dn4);
        let eq35_e2272_d_n5: f64 = (var_devsign * var_ied_dn5);
        let eq35_e2272_d_n6: f64 = (var_devsign * var_ied_dn6);
        let eq35_e2272_d_n7: f64 = (var_devsign * var_ied_dn7);
        let eq35_e2272_d_n8: f64 = (var_devsign * var_ied_dn8);
        let eq35_e2272_d_n9: f64 = (var_devsign * var_ied_dn9);
        let eq35_e2272_d_n10: f64 = (var_devsign * var_ied_dn10);
        let eq35_e2272_d_n11: f64 = (var_devsign * var_ied_dn11);
        let eq35_e2272_d_n13: f64 = (var_devsign * var_ied_dn13);
        let eq35_e2272_d_n14: f64 = (var_devsign * var_ied_dn14);
        let eq35_e2275: f64 = ((nv3 - nv5) * var_gmin);
        let eq35_e2276: f64 = (eq35_e2272 + eq35_e2275);
        let eq35_e2276_d_n3: f64 = (eq35_e2272_d_n3 + var_gmin);
        let eq35_e2276_d_n5: f64 = (eq35_e2272_d_n5 + (-var_gmin));
        (eq35_e2276, eq35_e2272_d_n0, eq35_e2272_d_n2, eq35_e2276_d_n3, eq35_e2272_d_n4, eq35_e2276_d_n5, eq35_e2272_d_n6, eq35_e2272_d_n7, eq35_e2272_d_n8, eq35_e2272_d_n9, eq35_e2272_d_n10, eq35_e2272_d_n11, eq35_e2272_d_n13, eq35_e2272_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e2278;
        let eq35_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq35_node_derivatives: [f64; 13] = [eq35_e2278_d_n0, eq35_e2278_d_n2, eq35_e2278_d_n3, eq35_e2278_d_n4, eq35_e2278_d_n5, eq35_e2278_d_n6, eq35_e2278_d_n7, eq35_e2278_d_n8, eq35_e2278_d_n9, eq35_e2278_d_n10, eq35_e2278_d_n11, eq35_e2278_d_n13, eq35_e2278_d_n14];
        let eq35_branch_derivative_indices: [usize; 0] = [];
        let eq35_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(5),
            multiplicity * (eq35_value),
            &eq35_node_derivative_indices,
            &eq35_node_derivatives,
            &eq35_branch_derivative_indices,
            &eq35_branch_derivatives,
            multiplicity,
        );
        let eq36_e2281: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, var_qes);
        let eq36_e2282: f64 = (var_devsign * eq36_e2281);
        let eq36_e2282_d_n0: f64 = (var_devsign * (var_qes_dn0 * ddt_scale));
        let eq36_e2282_d_n2: f64 = (var_devsign * (var_qes_dn2 * ddt_scale));
        let eq36_e2282_d_n3: f64 = (var_devsign * (var_qes_dn3 * ddt_scale));
        let eq36_e2282_d_n4: f64 = (var_devsign * (var_qes_dn4 * ddt_scale));
        let eq36_e2282_d_n5: f64 = (var_devsign * (var_qes_dn5 * ddt_scale));
        let eq36_e2282_d_n6: f64 = (var_devsign * (var_qes_dn6 * ddt_scale));
        let eq36_e2282_d_n7: f64 = (var_devsign * (var_qes_dn7 * ddt_scale));
        let eq36_e2282_d_n8: f64 = (var_devsign * (var_qes_dn8 * ddt_scale));
        let eq36_e2282_d_n9: f64 = (var_devsign * (var_qes_dn9 * ddt_scale));
        let eq36_e2282_d_n10: f64 = (var_devsign * (var_qes_dn10 * ddt_scale));
        let eq36_e2282_d_n11: f64 = (var_devsign * (var_qes_dn11 * ddt_scale));
        let eq36_e2282_d_n13: f64 = (var_devsign * (var_qes_dn13 * ddt_scale));
        let eq36_e2282_d_n14: f64 = (var_devsign * (var_qes_dn14 * ddt_scale));
        let eq36_value: f64 = eq36_e2282;
        let eq36_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq36_node_derivatives: [f64; 13] = [eq36_e2282_d_n0, eq36_e2282_d_n2, eq36_e2282_d_n3, eq36_e2282_d_n4, eq36_e2282_d_n5, eq36_e2282_d_n6, eq36_e2282_d_n7, eq36_e2282_d_n8, eq36_e2282_d_n9, eq36_e2282_d_n10, eq36_e2282_d_n11, eq36_e2282_d_n13, eq36_e2282_d_n14];
        let eq36_branch_derivative_indices: [usize; 0] = [];
        let eq36_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(6),
            multiplicity * (eq36_value),
            &eq36_node_derivative_indices,
            &eq36_node_derivatives,
            &eq36_branch_derivative_indices,
            &eq36_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
        stamper: &mut GeneratedStamper<'_>,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        var_devsign: f64,
        var_guard651: f64,
        var_guard652: f64,
        var_guard653: f64,
        var_guard654: f64,
        var_guard655: f64,
        var_guard656: f64,
        var_qbov: f64,
        var_qbov_dn0: f64,
        var_qbov_dn10: f64,
        var_qbov_dn11: f64,
        var_qbov_dn13: f64,
        var_qbov_dn14: f64,
        var_qbov_dn2: f64,
        var_qbov_dn3: f64,
        var_qbov_dn4: f64,
        var_qbov_dn5: f64,
        var_qbov_dn6: f64,
        var_qbov_dn7: f64,
        var_qbov_dn8: f64,
        var_qbov_dn9: f64,
        var_qbov_s: f64,
        var_qbov_s_dn0: f64,
        var_qbov_s_dn10: f64,
        var_qbov_s_dn11: f64,
        var_qbov_s_dn13: f64,
        var_qbov_s_dn14: f64,
        var_qbov_s_dn2: f64,
        var_qbov_s_dn3: f64,
        var_qbov_s_dn4: f64,
        var_qbov_s_dn5: f64,
        var_qbov_s_dn6: f64,
        var_qbov_s_dn7: f64,
        var_qbov_s_dn8: f64,
        var_qbov_s_dn9: f64,
        var_qds_fr: f64,
        var_qds_fr_dn0: f64,
        var_qds_fr_dn2: f64,
        var_qed: f64,
        var_qed_dn0: f64,
        var_qed_dn10: f64,
        var_qed_dn11: f64,
        var_qed_dn13: f64,
        var_qed_dn14: f64,
        var_qed_dn2: f64,
        var_qed_dn3: f64,
        var_qed_dn4: f64,
        var_qed_dn5: f64,
        var_qed_dn6: f64,
        var_qed_dn7: f64,
        var_qed_dn8: f64,
        var_qed_dn9: f64,
        var_qeg: f64,
        var_qeg_dn0: f64,
        var_qeg_dn10: f64,
        var_qeg_dn11: f64,
        var_qeg_dn13: f64,
        var_qeg_dn14: f64,
        var_qeg_dn2: f64,
        var_qeg_dn3: f64,
        var_qeg_dn4: f64,
        var_qeg_dn5: f64,
        var_qeg_dn6: f64,
        var_qeg_dn7: f64,
        var_qeg_dn8: f64,
        var_qeg_dn9: f64,
        var_qg_acc: f64,
        var_qg_acc_dn0: f64,
        var_qg_acc_dn10: f64,
        var_qg_acc_dn11: f64,
        var_qg_acc_dn13: f64,
        var_qg_acc_dn14: f64,
        var_qg_acc_dn2: f64,
        var_qg_acc_dn3: f64,
        var_qg_acc_dn4: f64,
        var_qg_acc_dn5: f64,
        var_qg_acc_dn6: f64,
        var_qg_acc_dn7: f64,
        var_qg_acc_dn8: f64,
        var_qg_acc_dn9: f64,
        var_qgd_fr: f64,
        var_qgd_fr_dn0: f64,
        var_qgd_fr_dn10: f64,
        var_qgd_fr_dn11: f64,
        var_qgd_fr_dn13: f64,
        var_qgd_fr_dn14: f64,
        var_qgd_fr_dn2: f64,
        var_qgd_fr_dn3: f64,
        var_qgd_fr_dn4: f64,
        var_qgd_fr_dn5: f64,
        var_qgd_fr_dn6: f64,
        var_qgd_fr_dn7: f64,
        var_qgd_fr_dn8: f64,
        var_qgd_fr_dn9: f64,
        var_qgd_parasitic: f64,
        var_qgd_parasitic_dn0: f64,
        var_qgd_parasitic_dn10: f64,
        var_qgd_parasitic_dn11: f64,
        var_qgd_parasitic_dn13: f64,
        var_qgd_parasitic_dn14: f64,
        var_qgd_parasitic_dn2: f64,
        var_qgd_parasitic_dn3: f64,
        var_qgd_parasitic_dn4: f64,
        var_qgd_parasitic_dn5: f64,
        var_qgd_parasitic_dn6: f64,
        var_qgd_parasitic_dn7: f64,
        var_qgd_parasitic_dn8: f64,
        var_qgd_parasitic_dn9: f64,
        var_qgs_fr: f64,
        var_qgs_fr_dn0: f64,
        var_qgs_fr_dn10: f64,
        var_qgs_fr_dn11: f64,
        var_qgs_fr_dn13: f64,
        var_qgs_fr_dn14: f64,
        var_qgs_fr_dn2: f64,
        var_qgs_fr_dn3: f64,
        var_qgs_fr_dn4: f64,
        var_qgs_fr_dn5: f64,
        var_qgs_fr_dn6: f64,
        var_qgs_fr_dn7: f64,
        var_qgs_fr_dn8: f64,
        var_qgs_fr_dn9: f64,
        var_qgs_parasitic: f64,
        var_qgs_parasitic_dn0: f64,
        var_qgs_parasitic_dn10: f64,
        var_qgs_parasitic_dn11: f64,
        var_qgs_parasitic_dn13: f64,
        var_qgs_parasitic_dn14: f64,
        var_qgs_parasitic_dn2: f64,
        var_qgs_parasitic_dn3: f64,
        var_qgs_parasitic_dn4: f64,
        var_qgs_parasitic_dn5: f64,
        var_qgs_parasitic_dn6: f64,
        var_qgs_parasitic_dn7: f64,
        var_qgs_parasitic_dn8: f64,
        var_qgs_parasitic_dn9: f64,
    ) {
        let eq37_e2285: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, var_qed);
        let eq37_e2286: f64 = (var_devsign * eq37_e2285);
        let eq37_e2286_d_n0: f64 = (var_devsign * (var_qed_dn0 * ddt_scale));
        let eq37_e2286_d_n2: f64 = (var_devsign * (var_qed_dn2 * ddt_scale));
        let eq37_e2286_d_n3: f64 = (var_devsign * (var_qed_dn3 * ddt_scale));
        let eq37_e2286_d_n4: f64 = (var_devsign * (var_qed_dn4 * ddt_scale));
        let eq37_e2286_d_n5: f64 = (var_devsign * (var_qed_dn5 * ddt_scale));
        let eq37_e2286_d_n6: f64 = (var_devsign * (var_qed_dn6 * ddt_scale));
        let eq37_e2286_d_n7: f64 = (var_devsign * (var_qed_dn7 * ddt_scale));
        let eq37_e2286_d_n8: f64 = (var_devsign * (var_qed_dn8 * ddt_scale));
        let eq37_e2286_d_n9: f64 = (var_devsign * (var_qed_dn9 * ddt_scale));
        let eq37_e2286_d_n10: f64 = (var_devsign * (var_qed_dn10 * ddt_scale));
        let eq37_e2286_d_n11: f64 = (var_devsign * (var_qed_dn11 * ddt_scale));
        let eq37_e2286_d_n13: f64 = (var_devsign * (var_qed_dn13 * ddt_scale));
        let eq37_e2286_d_n14: f64 = (var_devsign * (var_qed_dn14 * ddt_scale));
        let eq37_value: f64 = eq37_e2286;
        let eq37_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq37_node_derivatives: [f64; 13] = [eq37_e2286_d_n0, eq37_e2286_d_n2, eq37_e2286_d_n3, eq37_e2286_d_n4, eq37_e2286_d_n5, eq37_e2286_d_n6, eq37_e2286_d_n7, eq37_e2286_d_n8, eq37_e2286_d_n9, eq37_e2286_d_n10, eq37_e2286_d_n11, eq37_e2286_d_n13, eq37_e2286_d_n14];
        let eq37_branch_derivative_indices: [usize; 0] = [];
        let eq37_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(5),
            multiplicity * (eq37_value),
            &eq37_node_derivative_indices,
            &eq37_node_derivatives,
            &eq37_branch_derivative_indices,
            &eq37_branch_derivatives,
            multiplicity,
        );
        let eq38_e2289: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, var_qeg);
        let eq38_e2290: f64 = (var_devsign * eq38_e2289);
        let eq38_e2290_d_n0: f64 = (var_devsign * (var_qeg_dn0 * ddt_scale));
        let eq38_e2290_d_n2: f64 = (var_devsign * (var_qeg_dn2 * ddt_scale));
        let eq38_e2290_d_n3: f64 = (var_devsign * (var_qeg_dn3 * ddt_scale));
        let eq38_e2290_d_n4: f64 = (var_devsign * (var_qeg_dn4 * ddt_scale));
        let eq38_e2290_d_n5: f64 = (var_devsign * (var_qeg_dn5 * ddt_scale));
        let eq38_e2290_d_n6: f64 = (var_devsign * (var_qeg_dn6 * ddt_scale));
        let eq38_e2290_d_n7: f64 = (var_devsign * (var_qeg_dn7 * ddt_scale));
        let eq38_e2290_d_n8: f64 = (var_devsign * (var_qeg_dn8 * ddt_scale));
        let eq38_e2290_d_n9: f64 = (var_devsign * (var_qeg_dn9 * ddt_scale));
        let eq38_e2290_d_n10: f64 = (var_devsign * (var_qeg_dn10 * ddt_scale));
        let eq38_e2290_d_n11: f64 = (var_devsign * (var_qeg_dn11 * ddt_scale));
        let eq38_e2290_d_n13: f64 = (var_devsign * (var_qeg_dn13 * ddt_scale));
        let eq38_e2290_d_n14: f64 = (var_devsign * (var_qeg_dn14 * ddt_scale));
        let eq38_value: f64 = eq38_e2290;
        let eq38_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq38_node_derivatives: [f64; 13] = [eq38_e2290_d_n0, eq38_e2290_d_n2, eq38_e2290_d_n3, eq38_e2290_d_n4, eq38_e2290_d_n5, eq38_e2290_d_n6, eq38_e2290_d_n7, eq38_e2290_d_n8, eq38_e2290_d_n9, eq38_e2290_d_n10, eq38_e2290_d_n11, eq38_e2290_d_n13, eq38_e2290_d_n14];
        let eq38_branch_derivative_indices: [usize; 0] = [];
        let eq38_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(10),
            multiplicity * (eq38_value),
            &eq38_node_derivative_indices,
            &eq38_node_derivatives,
            &eq38_branch_derivative_indices,
            &eq38_branch_derivatives,
            multiplicity,
        );
        let (eq39_e2295, eq39_e2295_d_n0, eq39_e2295_d_n2, eq39_e2295_d_n3, eq39_e2295_d_n4, eq39_e2295_d_n5, eq39_e2295_d_n6, eq39_e2295_d_n7, eq39_e2295_d_n8, eq39_e2295_d_n9, eq39_e2295_d_n10, eq39_e2295_d_n11, eq39_e2295_d_n13, eq39_e2295_d_n14,) = {
    if (var_guard651 != 0.0) {
        let eq39_e2293: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, var_qgs_parasitic);
        (eq39_e2293, (var_qgs_parasitic_dn0 * ddt_scale), (var_qgs_parasitic_dn2 * ddt_scale), (var_qgs_parasitic_dn3 * ddt_scale), (var_qgs_parasitic_dn4 * ddt_scale), (var_qgs_parasitic_dn5 * ddt_scale), (var_qgs_parasitic_dn6 * ddt_scale), (var_qgs_parasitic_dn7 * ddt_scale), (var_qgs_parasitic_dn8 * ddt_scale), (var_qgs_parasitic_dn9 * ddt_scale), (var_qgs_parasitic_dn10 * ddt_scale), (var_qgs_parasitic_dn11 * ddt_scale), (var_qgs_parasitic_dn13 * ddt_scale), (var_qgs_parasitic_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e2295;
        let eq39_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq39_node_derivatives: [f64; 13] = [eq39_e2295_d_n0, eq39_e2295_d_n2, eq39_e2295_d_n3, eq39_e2295_d_n4, eq39_e2295_d_n5, eq39_e2295_d_n6, eq39_e2295_d_n7, eq39_e2295_d_n8, eq39_e2295_d_n9, eq39_e2295_d_n10, eq39_e2295_d_n11, eq39_e2295_d_n13, eq39_e2295_d_n14];
        let eq39_branch_derivative_indices: [usize; 0] = [];
        let eq39_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(6),
            multiplicity * (eq39_value),
            &eq39_node_derivative_indices,
            &eq39_node_derivatives,
            &eq39_branch_derivative_indices,
            &eq39_branch_derivatives,
            multiplicity,
        );
        let (eq40_e2302, eq40_e2302_d_n0, eq40_e2302_d_n2, eq40_e2302_d_n3, eq40_e2302_d_n4, eq40_e2302_d_n5, eq40_e2302_d_n6, eq40_e2302_d_n7, eq40_e2302_d_n8, eq40_e2302_d_n9, eq40_e2302_d_n10, eq40_e2302_d_n11, eq40_e2302_d_n13, eq40_e2302_d_n14,) = {
    if ((var_guard651 != 0.0) && (var_guard652 != 0.0)) {
        let eq40_e2300: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, var_qgd_parasitic);
        (eq40_e2300, (var_qgd_parasitic_dn0 * ddt_scale), (var_qgd_parasitic_dn2 * ddt_scale), (var_qgd_parasitic_dn3 * ddt_scale), (var_qgd_parasitic_dn4 * ddt_scale), (var_qgd_parasitic_dn5 * ddt_scale), (var_qgd_parasitic_dn6 * ddt_scale), (var_qgd_parasitic_dn7 * ddt_scale), (var_qgd_parasitic_dn8 * ddt_scale), (var_qgd_parasitic_dn9 * ddt_scale), (var_qgd_parasitic_dn10 * ddt_scale), (var_qgd_parasitic_dn11 * ddt_scale), (var_qgd_parasitic_dn13 * ddt_scale), (var_qgd_parasitic_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_value: f64 = eq40_e2302;
        let eq40_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq40_node_derivatives: [f64; 13] = [eq40_e2302_d_n0, eq40_e2302_d_n2, eq40_e2302_d_n3, eq40_e2302_d_n4, eq40_e2302_d_n5, eq40_e2302_d_n6, eq40_e2302_d_n7, eq40_e2302_d_n8, eq40_e2302_d_n9, eq40_e2302_d_n10, eq40_e2302_d_n11, eq40_e2302_d_n13, eq40_e2302_d_n14];
        let eq40_branch_derivative_indices: [usize; 0] = [];
        let eq40_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(7),
            multiplicity * (eq40_value),
            &eq40_node_derivative_indices,
            &eq40_node_derivatives,
            &eq40_branch_derivative_indices,
            &eq40_branch_derivatives,
            multiplicity,
        );
        let (eq41_e2311, eq41_e2311_d_n0, eq41_e2311_d_n2, eq41_e2311_d_n3, eq41_e2311_d_n4, eq41_e2311_d_n5, eq41_e2311_d_n6, eq41_e2311_d_n7, eq41_e2311_d_n8, eq41_e2311_d_n9, eq41_e2311_d_n10, eq41_e2311_d_n11, eq41_e2311_d_n13, eq41_e2311_d_n14,) = {
    if ((var_guard651 != 0.0) && (var_guard652 != 0.0)) {
        let eq41_e2308: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, var_qbov);
        let eq41_e2309: f64 = (var_devsign * eq41_e2308);
        let eq41_e2309_d_n0: f64 = (var_devsign * (var_qbov_dn0 * ddt_scale));
        let eq41_e2309_d_n2: f64 = (var_devsign * (var_qbov_dn2 * ddt_scale));
        let eq41_e2309_d_n3: f64 = (var_devsign * (var_qbov_dn3 * ddt_scale));
        let eq41_e2309_d_n4: f64 = (var_devsign * (var_qbov_dn4 * ddt_scale));
        let eq41_e2309_d_n5: f64 = (var_devsign * (var_qbov_dn5 * ddt_scale));
        let eq41_e2309_d_n6: f64 = (var_devsign * (var_qbov_dn6 * ddt_scale));
        let eq41_e2309_d_n7: f64 = (var_devsign * (var_qbov_dn7 * ddt_scale));
        let eq41_e2309_d_n8: f64 = (var_devsign * (var_qbov_dn8 * ddt_scale));
        let eq41_e2309_d_n9: f64 = (var_devsign * (var_qbov_dn9 * ddt_scale));
        let eq41_e2309_d_n10: f64 = (var_devsign * (var_qbov_dn10 * ddt_scale));
        let eq41_e2309_d_n11: f64 = (var_devsign * (var_qbov_dn11 * ddt_scale));
        let eq41_e2309_d_n13: f64 = (var_devsign * (var_qbov_dn13 * ddt_scale));
        let eq41_e2309_d_n14: f64 = (var_devsign * (var_qbov_dn14 * ddt_scale));
        (eq41_e2309, eq41_e2309_d_n0, eq41_e2309_d_n2, eq41_e2309_d_n3, eq41_e2309_d_n4, eq41_e2309_d_n5, eq41_e2309_d_n6, eq41_e2309_d_n7, eq41_e2309_d_n8, eq41_e2309_d_n9, eq41_e2309_d_n10, eq41_e2309_d_n11, eq41_e2309_d_n13, eq41_e2309_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq41_value: f64 = eq41_e2311;
        let eq41_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq41_node_derivatives: [f64; 13] = [eq41_e2311_d_n0, eq41_e2311_d_n2, eq41_e2311_d_n3, eq41_e2311_d_n4, eq41_e2311_d_n5, eq41_e2311_d_n6, eq41_e2311_d_n7, eq41_e2311_d_n8, eq41_e2311_d_n9, eq41_e2311_d_n10, eq41_e2311_d_n11, eq41_e2311_d_n13, eq41_e2311_d_n14];
        let eq41_branch_derivative_indices: [usize; 0] = [];
        let eq41_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(7),
            multiplicity * (eq41_value),
            &eq41_node_derivative_indices,
            &eq41_node_derivatives,
            &eq41_branch_derivative_indices,
            &eq41_branch_derivatives,
            multiplicity,
        );
        let (eq42_e2320, eq42_e2320_d_n0, eq42_e2320_d_n2, eq42_e2320_d_n3, eq42_e2320_d_n4, eq42_e2320_d_n5, eq42_e2320_d_n6, eq42_e2320_d_n7, eq42_e2320_d_n8, eq42_e2320_d_n9, eq42_e2320_d_n10, eq42_e2320_d_n11, eq42_e2320_d_n13, eq42_e2320_d_n14,) = {
    if ((var_guard651 != 0.0) && (var_guard652 != 0.0)) {
        let eq42_e2317: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, var_qbov_s);
        let eq42_e2318: f64 = (var_devsign * eq42_e2317);
        let eq42_e2318_d_n0: f64 = (var_devsign * (var_qbov_s_dn0 * ddt_scale));
        let eq42_e2318_d_n2: f64 = (var_devsign * (var_qbov_s_dn2 * ddt_scale));
        let eq42_e2318_d_n3: f64 = (var_devsign * (var_qbov_s_dn3 * ddt_scale));
        let eq42_e2318_d_n4: f64 = (var_devsign * (var_qbov_s_dn4 * ddt_scale));
        let eq42_e2318_d_n5: f64 = (var_devsign * (var_qbov_s_dn5 * ddt_scale));
        let eq42_e2318_d_n6: f64 = (var_devsign * (var_qbov_s_dn6 * ddt_scale));
        let eq42_e2318_d_n7: f64 = (var_devsign * (var_qbov_s_dn7 * ddt_scale));
        let eq42_e2318_d_n8: f64 = (var_devsign * (var_qbov_s_dn8 * ddt_scale));
        let eq42_e2318_d_n9: f64 = (var_devsign * (var_qbov_s_dn9 * ddt_scale));
        let eq42_e2318_d_n10: f64 = (var_devsign * (var_qbov_s_dn10 * ddt_scale));
        let eq42_e2318_d_n11: f64 = (var_devsign * (var_qbov_s_dn11 * ddt_scale));
        let eq42_e2318_d_n13: f64 = (var_devsign * (var_qbov_s_dn13 * ddt_scale));
        let eq42_e2318_d_n14: f64 = (var_devsign * (var_qbov_s_dn14 * ddt_scale));
        (eq42_e2318, eq42_e2318_d_n0, eq42_e2318_d_n2, eq42_e2318_d_n3, eq42_e2318_d_n4, eq42_e2318_d_n5, eq42_e2318_d_n6, eq42_e2318_d_n7, eq42_e2318_d_n8, eq42_e2318_d_n9, eq42_e2318_d_n10, eq42_e2318_d_n11, eq42_e2318_d_n13, eq42_e2318_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq42_value: f64 = eq42_e2320;
        let eq42_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq42_node_derivatives: [f64; 13] = [eq42_e2320_d_n0, eq42_e2320_d_n2, eq42_e2320_d_n3, eq42_e2320_d_n4, eq42_e2320_d_n5, eq42_e2320_d_n6, eq42_e2320_d_n7, eq42_e2320_d_n8, eq42_e2320_d_n9, eq42_e2320_d_n10, eq42_e2320_d_n11, eq42_e2320_d_n13, eq42_e2320_d_n14];
        let eq42_branch_derivative_indices: [usize; 0] = [];
        let eq42_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(5),
            multiplicity * (eq42_value),
            &eq42_node_derivative_indices,
            &eq42_node_derivatives,
            &eq42_branch_derivative_indices,
            &eq42_branch_derivatives,
            multiplicity,
        );
        let (eq43_e2328, eq43_e2328_d_n0, eq43_e2328_d_n2, eq43_e2328_d_n3, eq43_e2328_d_n4, eq43_e2328_d_n5, eq43_e2328_d_n6, eq43_e2328_d_n7, eq43_e2328_d_n8, eq43_e2328_d_n9, eq43_e2328_d_n10, eq43_e2328_d_n11, eq43_e2328_d_n13, eq43_e2328_d_n14,) = {
    if ((var_guard651 != 0.0) && (var_guard652 == 0.0)) {
        let eq43_e2326: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, var_qgd_parasitic);
        (eq43_e2326, (var_qgd_parasitic_dn0 * ddt_scale), (var_qgd_parasitic_dn2 * ddt_scale), (var_qgd_parasitic_dn3 * ddt_scale), (var_qgd_parasitic_dn4 * ddt_scale), (var_qgd_parasitic_dn5 * ddt_scale), (var_qgd_parasitic_dn6 * ddt_scale), (var_qgd_parasitic_dn7 * ddt_scale), (var_qgd_parasitic_dn8 * ddt_scale), (var_qgd_parasitic_dn9 * ddt_scale), (var_qgd_parasitic_dn10 * ddt_scale), (var_qgd_parasitic_dn11 * ddt_scale), (var_qgd_parasitic_dn13 * ddt_scale), (var_qgd_parasitic_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq43_value: f64 = eq43_e2328;
        let eq43_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq43_node_derivatives: [f64; 13] = [eq43_e2328_d_n0, eq43_e2328_d_n2, eq43_e2328_d_n3, eq43_e2328_d_n4, eq43_e2328_d_n5, eq43_e2328_d_n6, eq43_e2328_d_n7, eq43_e2328_d_n8, eq43_e2328_d_n9, eq43_e2328_d_n10, eq43_e2328_d_n11, eq43_e2328_d_n13, eq43_e2328_d_n14];
        let eq43_branch_derivative_indices: [usize; 0] = [];
        let eq43_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(5),
            multiplicity * (eq43_value),
            &eq43_node_derivative_indices,
            &eq43_node_derivatives,
            &eq43_branch_derivative_indices,
            &eq43_branch_derivatives,
            multiplicity,
        );
        let (eq44_e2333, eq44_e2333_d_n0, eq44_e2333_d_n2,) = {
    if (var_guard651 != 0.0) {
        let eq44_e2331: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, var_qds_fr);
        (eq44_e2331, (var_qds_fr_dn0 * ddt_scale), (var_qds_fr_dn2 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e2333;
        stamper.stamp_current_node2_local(
            Some(0),
            Some(2),
            multiplicity * (eq44_value),
            0,
            multiplicity * (eq44_e2333_d_n0),
            2,
            multiplicity * (eq44_e2333_d_n2),
        );
        let (eq45_e2340, eq45_e2340_d_n0, eq45_e2340_d_n2, eq45_e2340_d_n3, eq45_e2340_d_n4, eq45_e2340_d_n5, eq45_e2340_d_n6, eq45_e2340_d_n7, eq45_e2340_d_n8, eq45_e2340_d_n9, eq45_e2340_d_n10, eq45_e2340_d_n11, eq45_e2340_d_n13, eq45_e2340_d_n14,) = {
    if ((var_guard651 != 0.0) && (var_guard653 != 0.0)) {
        let eq45_e2338: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, var_qgs_fr);
        (eq45_e2338, (var_qgs_fr_dn0 * ddt_scale), (var_qgs_fr_dn2 * ddt_scale), (var_qgs_fr_dn3 * ddt_scale), (var_qgs_fr_dn4 * ddt_scale), (var_qgs_fr_dn5 * ddt_scale), (var_qgs_fr_dn6 * ddt_scale), (var_qgs_fr_dn7 * ddt_scale), (var_qgs_fr_dn8 * ddt_scale), (var_qgs_fr_dn9 * ddt_scale), (var_qgs_fr_dn10 * ddt_scale), (var_qgs_fr_dn11 * ddt_scale), (var_qgs_fr_dn13 * ddt_scale), (var_qgs_fr_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq45_value: f64 = eq45_e2340;
        let eq45_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq45_node_derivatives: [f64; 13] = [eq45_e2340_d_n0, eq45_e2340_d_n2, eq45_e2340_d_n3, eq45_e2340_d_n4, eq45_e2340_d_n5, eq45_e2340_d_n6, eq45_e2340_d_n7, eq45_e2340_d_n8, eq45_e2340_d_n9, eq45_e2340_d_n10, eq45_e2340_d_n11, eq45_e2340_d_n13, eq45_e2340_d_n14];
        let eq45_branch_derivative_indices: [usize; 0] = [];
        let eq45_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(2),
            multiplicity * (eq45_value),
            &eq45_node_derivative_indices,
            &eq45_node_derivatives,
            &eq45_branch_derivative_indices,
            &eq45_branch_derivatives,
            multiplicity,
        );
        let (eq46_e2347, eq46_e2347_d_n0, eq46_e2347_d_n2, eq46_e2347_d_n3, eq46_e2347_d_n4, eq46_e2347_d_n5, eq46_e2347_d_n6, eq46_e2347_d_n7, eq46_e2347_d_n8, eq46_e2347_d_n9, eq46_e2347_d_n10, eq46_e2347_d_n11, eq46_e2347_d_n13, eq46_e2347_d_n14,) = {
    if ((var_guard651 != 0.0) && (var_guard653 != 0.0)) {
        let eq46_e2345: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, var_qgd_fr);
        (eq46_e2345, (var_qgd_fr_dn0 * ddt_scale), (var_qgd_fr_dn2 * ddt_scale), (var_qgd_fr_dn3 * ddt_scale), (var_qgd_fr_dn4 * ddt_scale), (var_qgd_fr_dn5 * ddt_scale), (var_qgd_fr_dn6 * ddt_scale), (var_qgd_fr_dn7 * ddt_scale), (var_qgd_fr_dn8 * ddt_scale), (var_qgd_fr_dn9 * ddt_scale), (var_qgd_fr_dn10 * ddt_scale), (var_qgd_fr_dn11 * ddt_scale), (var_qgd_fr_dn13 * ddt_scale), (var_qgd_fr_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e2347;
        let eq46_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq46_node_derivatives: [f64; 13] = [eq46_e2347_d_n0, eq46_e2347_d_n2, eq46_e2347_d_n3, eq46_e2347_d_n4, eq46_e2347_d_n5, eq46_e2347_d_n6, eq46_e2347_d_n7, eq46_e2347_d_n8, eq46_e2347_d_n9, eq46_e2347_d_n10, eq46_e2347_d_n11, eq46_e2347_d_n13, eq46_e2347_d_n14];
        let eq46_branch_derivative_indices: [usize; 0] = [];
        let eq46_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(0),
            multiplicity * (eq46_value),
            &eq46_node_derivative_indices,
            &eq46_node_derivatives,
            &eq46_branch_derivative_indices,
            &eq46_branch_derivatives,
            multiplicity,
        );
        let (eq47_e2353, eq47_e2353_d_n0, eq47_e2353_d_n2, eq47_e2353_d_n3, eq47_e2353_d_n4, eq47_e2353_d_n5, eq47_e2353_d_n6, eq47_e2353_d_n7, eq47_e2353_d_n8, eq47_e2353_d_n9, eq47_e2353_d_n10, eq47_e2353_d_n11, eq47_e2353_d_n13, eq47_e2353_d_n14,) = {
    if (var_guard651 == 0.0) {
        let eq47_e2351: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, var_qgs_parasitic);
        (eq47_e2351, (var_qgs_parasitic_dn0 * ddt_scale), (var_qgs_parasitic_dn2 * ddt_scale), (var_qgs_parasitic_dn3 * ddt_scale), (var_qgs_parasitic_dn4 * ddt_scale), (var_qgs_parasitic_dn5 * ddt_scale), (var_qgs_parasitic_dn6 * ddt_scale), (var_qgs_parasitic_dn7 * ddt_scale), (var_qgs_parasitic_dn8 * ddt_scale), (var_qgs_parasitic_dn9 * ddt_scale), (var_qgs_parasitic_dn10 * ddt_scale), (var_qgs_parasitic_dn11 * ddt_scale), (var_qgs_parasitic_dn13 * ddt_scale), (var_qgs_parasitic_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq47_value: f64 = eq47_e2353;
        let eq47_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq47_node_derivatives: [f64; 13] = [eq47_e2353_d_n0, eq47_e2353_d_n2, eq47_e2353_d_n3, eq47_e2353_d_n4, eq47_e2353_d_n5, eq47_e2353_d_n6, eq47_e2353_d_n7, eq47_e2353_d_n8, eq47_e2353_d_n9, eq47_e2353_d_n10, eq47_e2353_d_n11, eq47_e2353_d_n13, eq47_e2353_d_n14];
        let eq47_branch_derivative_indices: [usize; 0] = [];
        let eq47_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            Some(6),
            multiplicity * (eq47_value),
            &eq47_node_derivative_indices,
            &eq47_node_derivatives,
            &eq47_branch_derivative_indices,
            &eq47_branch_derivatives,
            multiplicity,
        );
        let (eq48_e2361, eq48_e2361_d_n0, eq48_e2361_d_n2, eq48_e2361_d_n3, eq48_e2361_d_n4, eq48_e2361_d_n5, eq48_e2361_d_n6, eq48_e2361_d_n7, eq48_e2361_d_n8, eq48_e2361_d_n9, eq48_e2361_d_n10, eq48_e2361_d_n11, eq48_e2361_d_n13, eq48_e2361_d_n14,) = {
    if ((var_guard651 == 0.0) && (var_guard654 != 0.0)) {
        let eq48_e2359: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, var_qgd_parasitic);
        (eq48_e2359, (var_qgd_parasitic_dn0 * ddt_scale), (var_qgd_parasitic_dn2 * ddt_scale), (var_qgd_parasitic_dn3 * ddt_scale), (var_qgd_parasitic_dn4 * ddt_scale), (var_qgd_parasitic_dn5 * ddt_scale), (var_qgd_parasitic_dn6 * ddt_scale), (var_qgd_parasitic_dn7 * ddt_scale), (var_qgd_parasitic_dn8 * ddt_scale), (var_qgd_parasitic_dn9 * ddt_scale), (var_qgd_parasitic_dn10 * ddt_scale), (var_qgd_parasitic_dn11 * ddt_scale), (var_qgd_parasitic_dn13 * ddt_scale), (var_qgd_parasitic_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq48_value: f64 = eq48_e2361;
        let eq48_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq48_node_derivatives: [f64; 13] = [eq48_e2361_d_n0, eq48_e2361_d_n2, eq48_e2361_d_n3, eq48_e2361_d_n4, eq48_e2361_d_n5, eq48_e2361_d_n6, eq48_e2361_d_n7, eq48_e2361_d_n8, eq48_e2361_d_n9, eq48_e2361_d_n10, eq48_e2361_d_n11, eq48_e2361_d_n13, eq48_e2361_d_n14];
        let eq48_branch_derivative_indices: [usize; 0] = [];
        let eq48_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(14),
            Some(7),
            multiplicity * (eq48_value),
            &eq48_node_derivative_indices,
            &eq48_node_derivatives,
            &eq48_branch_derivative_indices,
            &eq48_branch_derivatives,
            multiplicity,
        );
        let (eq49_e2371, eq49_e2371_d_n0, eq49_e2371_d_n2, eq49_e2371_d_n3, eq49_e2371_d_n4, eq49_e2371_d_n5, eq49_e2371_d_n6, eq49_e2371_d_n7, eq49_e2371_d_n8, eq49_e2371_d_n9, eq49_e2371_d_n10, eq49_e2371_d_n11, eq49_e2371_d_n13, eq49_e2371_d_n14,) = {
    if ((var_guard651 == 0.0) && (var_guard654 != 0.0)) {
        let eq49_e2368: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, var_qbov);
        let eq49_e2369: f64 = (var_devsign * eq49_e2368);
        let eq49_e2369_d_n0: f64 = (var_devsign * (var_qbov_dn0 * ddt_scale));
        let eq49_e2369_d_n2: f64 = (var_devsign * (var_qbov_dn2 * ddt_scale));
        let eq49_e2369_d_n3: f64 = (var_devsign * (var_qbov_dn3 * ddt_scale));
        let eq49_e2369_d_n4: f64 = (var_devsign * (var_qbov_dn4 * ddt_scale));
        let eq49_e2369_d_n5: f64 = (var_devsign * (var_qbov_dn5 * ddt_scale));
        let eq49_e2369_d_n6: f64 = (var_devsign * (var_qbov_dn6 * ddt_scale));
        let eq49_e2369_d_n7: f64 = (var_devsign * (var_qbov_dn7 * ddt_scale));
        let eq49_e2369_d_n8: f64 = (var_devsign * (var_qbov_dn8 * ddt_scale));
        let eq49_e2369_d_n9: f64 = (var_devsign * (var_qbov_dn9 * ddt_scale));
        let eq49_e2369_d_n10: f64 = (var_devsign * (var_qbov_dn10 * ddt_scale));
        let eq49_e2369_d_n11: f64 = (var_devsign * (var_qbov_dn11 * ddt_scale));
        let eq49_e2369_d_n13: f64 = (var_devsign * (var_qbov_dn13 * ddt_scale));
        let eq49_e2369_d_n14: f64 = (var_devsign * (var_qbov_dn14 * ddt_scale));
        (eq49_e2369, eq49_e2369_d_n0, eq49_e2369_d_n2, eq49_e2369_d_n3, eq49_e2369_d_n4, eq49_e2369_d_n5, eq49_e2369_d_n6, eq49_e2369_d_n7, eq49_e2369_d_n8, eq49_e2369_d_n9, eq49_e2369_d_n10, eq49_e2369_d_n11, eq49_e2369_d_n13, eq49_e2369_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq49_value: f64 = eq49_e2371;
        let eq49_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq49_node_derivatives: [f64; 13] = [eq49_e2371_d_n0, eq49_e2371_d_n2, eq49_e2371_d_n3, eq49_e2371_d_n4, eq49_e2371_d_n5, eq49_e2371_d_n6, eq49_e2371_d_n7, eq49_e2371_d_n8, eq49_e2371_d_n9, eq49_e2371_d_n10, eq49_e2371_d_n11, eq49_e2371_d_n13, eq49_e2371_d_n14];
        let eq49_branch_derivative_indices: [usize; 0] = [];
        let eq49_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(14),
            Some(7),
            multiplicity * (eq49_value),
            &eq49_node_derivative_indices,
            &eq49_node_derivatives,
            &eq49_branch_derivative_indices,
            &eq49_branch_derivatives,
            multiplicity,
        );
        let (eq50_e2381, eq50_e2381_d_n0, eq50_e2381_d_n2, eq50_e2381_d_n3, eq50_e2381_d_n4, eq50_e2381_d_n5, eq50_e2381_d_n6, eq50_e2381_d_n7, eq50_e2381_d_n8, eq50_e2381_d_n9, eq50_e2381_d_n10, eq50_e2381_d_n11, eq50_e2381_d_n13, eq50_e2381_d_n14,) = {
    if ((var_guard651 == 0.0) && (var_guard654 != 0.0)) {
        let eq50_e2378: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, var_qbov_s);
        let eq50_e2379: f64 = (var_devsign * eq50_e2378);
        let eq50_e2379_d_n0: f64 = (var_devsign * (var_qbov_s_dn0 * ddt_scale));
        let eq50_e2379_d_n2: f64 = (var_devsign * (var_qbov_s_dn2 * ddt_scale));
        let eq50_e2379_d_n3: f64 = (var_devsign * (var_qbov_s_dn3 * ddt_scale));
        let eq50_e2379_d_n4: f64 = (var_devsign * (var_qbov_s_dn4 * ddt_scale));
        let eq50_e2379_d_n5: f64 = (var_devsign * (var_qbov_s_dn5 * ddt_scale));
        let eq50_e2379_d_n6: f64 = (var_devsign * (var_qbov_s_dn6 * ddt_scale));
        let eq50_e2379_d_n7: f64 = (var_devsign * (var_qbov_s_dn7 * ddt_scale));
        let eq50_e2379_d_n8: f64 = (var_devsign * (var_qbov_s_dn8 * ddt_scale));
        let eq50_e2379_d_n9: f64 = (var_devsign * (var_qbov_s_dn9 * ddt_scale));
        let eq50_e2379_d_n10: f64 = (var_devsign * (var_qbov_s_dn10 * ddt_scale));
        let eq50_e2379_d_n11: f64 = (var_devsign * (var_qbov_s_dn11 * ddt_scale));
        let eq50_e2379_d_n13: f64 = (var_devsign * (var_qbov_s_dn13 * ddt_scale));
        let eq50_e2379_d_n14: f64 = (var_devsign * (var_qbov_s_dn14 * ddt_scale));
        (eq50_e2379, eq50_e2379_d_n0, eq50_e2379_d_n2, eq50_e2379_d_n3, eq50_e2379_d_n4, eq50_e2379_d_n5, eq50_e2379_d_n6, eq50_e2379_d_n7, eq50_e2379_d_n8, eq50_e2379_d_n9, eq50_e2379_d_n10, eq50_e2379_d_n11, eq50_e2379_d_n13, eq50_e2379_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e2381;
        let eq50_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq50_node_derivatives: [f64; 13] = [eq50_e2381_d_n0, eq50_e2381_d_n2, eq50_e2381_d_n3, eq50_e2381_d_n4, eq50_e2381_d_n5, eq50_e2381_d_n6, eq50_e2381_d_n7, eq50_e2381_d_n8, eq50_e2381_d_n9, eq50_e2381_d_n10, eq50_e2381_d_n11, eq50_e2381_d_n13, eq50_e2381_d_n14];
        let eq50_branch_derivative_indices: [usize; 0] = [];
        let eq50_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(14),
            Some(5),
            multiplicity * (eq50_value),
            &eq50_node_derivative_indices,
            &eq50_node_derivatives,
            &eq50_branch_derivative_indices,
            &eq50_branch_derivatives,
            multiplicity,
        );
        let (eq51_e2390, eq51_e2390_d_n0, eq51_e2390_d_n2, eq51_e2390_d_n3, eq51_e2390_d_n4, eq51_e2390_d_n5, eq51_e2390_d_n6, eq51_e2390_d_n7, eq51_e2390_d_n8, eq51_e2390_d_n9, eq51_e2390_d_n10, eq51_e2390_d_n11, eq51_e2390_d_n13, eq51_e2390_d_n14,) = {
    if ((var_guard651 == 0.0) && (var_guard654 == 0.0)) {
        let eq51_e2388: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 17, var_qgd_parasitic);
        (eq51_e2388, (var_qgd_parasitic_dn0 * ddt_scale), (var_qgd_parasitic_dn2 * ddt_scale), (var_qgd_parasitic_dn3 * ddt_scale), (var_qgd_parasitic_dn4 * ddt_scale), (var_qgd_parasitic_dn5 * ddt_scale), (var_qgd_parasitic_dn6 * ddt_scale), (var_qgd_parasitic_dn7 * ddt_scale), (var_qgd_parasitic_dn8 * ddt_scale), (var_qgd_parasitic_dn9 * ddt_scale), (var_qgd_parasitic_dn10 * ddt_scale), (var_qgd_parasitic_dn11 * ddt_scale), (var_qgd_parasitic_dn13 * ddt_scale), (var_qgd_parasitic_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e2390;
        let eq51_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq51_node_derivatives: [f64; 13] = [eq51_e2390_d_n0, eq51_e2390_d_n2, eq51_e2390_d_n3, eq51_e2390_d_n4, eq51_e2390_d_n5, eq51_e2390_d_n6, eq51_e2390_d_n7, eq51_e2390_d_n8, eq51_e2390_d_n9, eq51_e2390_d_n10, eq51_e2390_d_n11, eq51_e2390_d_n13, eq51_e2390_d_n14];
        let eq51_branch_derivative_indices: [usize; 0] = [];
        let eq51_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(14),
            Some(5),
            multiplicity * (eq51_value),
            &eq51_node_derivative_indices,
            &eq51_node_derivatives,
            &eq51_branch_derivative_indices,
            &eq51_branch_derivatives,
            multiplicity,
        );
        let (eq52_e2396, eq52_e2396_d_n0, eq52_e2396_d_n2,) = {
    if (var_guard651 == 0.0) {
        let eq52_e2394: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, var_qds_fr);
        (eq52_e2394, (var_qds_fr_dn0 * ddt_scale), (var_qds_fr_dn2 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e2396;
        stamper.stamp_current_node2_local(
            Some(0),
            Some(2),
            multiplicity * (eq52_value),
            0,
            multiplicity * (eq52_e2396_d_n0),
            2,
            multiplicity * (eq52_e2396_d_n2),
        );
        let (eq53_e2404, eq53_e2404_d_n0, eq53_e2404_d_n2, eq53_e2404_d_n3, eq53_e2404_d_n4, eq53_e2404_d_n5, eq53_e2404_d_n6, eq53_e2404_d_n7, eq53_e2404_d_n8, eq53_e2404_d_n9, eq53_e2404_d_n10, eq53_e2404_d_n11, eq53_e2404_d_n13, eq53_e2404_d_n14,) = {
    if ((var_guard651 == 0.0) && (var_guard655 != 0.0)) {
        let eq53_e2402: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, var_qgs_fr);
        (eq53_e2402, (var_qgs_fr_dn0 * ddt_scale), (var_qgs_fr_dn2 * ddt_scale), (var_qgs_fr_dn3 * ddt_scale), (var_qgs_fr_dn4 * ddt_scale), (var_qgs_fr_dn5 * ddt_scale), (var_qgs_fr_dn6 * ddt_scale), (var_qgs_fr_dn7 * ddt_scale), (var_qgs_fr_dn8 * ddt_scale), (var_qgs_fr_dn9 * ddt_scale), (var_qgs_fr_dn10 * ddt_scale), (var_qgs_fr_dn11 * ddt_scale), (var_qgs_fr_dn13 * ddt_scale), (var_qgs_fr_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e2404;
        let eq53_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq53_node_derivatives: [f64; 13] = [eq53_e2404_d_n0, eq53_e2404_d_n2, eq53_e2404_d_n3, eq53_e2404_d_n4, eq53_e2404_d_n5, eq53_e2404_d_n6, eq53_e2404_d_n7, eq53_e2404_d_n8, eq53_e2404_d_n9, eq53_e2404_d_n10, eq53_e2404_d_n11, eq53_e2404_d_n13, eq53_e2404_d_n14];
        let eq53_branch_derivative_indices: [usize; 0] = [];
        let eq53_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            Some(2),
            multiplicity * (eq53_value),
            &eq53_node_derivative_indices,
            &eq53_node_derivatives,
            &eq53_branch_derivative_indices,
            &eq53_branch_derivatives,
            multiplicity,
        );
        let (eq54_e2412, eq54_e2412_d_n0, eq54_e2412_d_n2, eq54_e2412_d_n3, eq54_e2412_d_n4, eq54_e2412_d_n5, eq54_e2412_d_n6, eq54_e2412_d_n7, eq54_e2412_d_n8, eq54_e2412_d_n9, eq54_e2412_d_n10, eq54_e2412_d_n11, eq54_e2412_d_n13, eq54_e2412_d_n14,) = {
    if ((var_guard651 == 0.0) && (var_guard655 != 0.0)) {
        let eq54_e2410: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 20, var_qgd_fr);
        (eq54_e2410, (var_qgd_fr_dn0 * ddt_scale), (var_qgd_fr_dn2 * ddt_scale), (var_qgd_fr_dn3 * ddt_scale), (var_qgd_fr_dn4 * ddt_scale), (var_qgd_fr_dn5 * ddt_scale), (var_qgd_fr_dn6 * ddt_scale), (var_qgd_fr_dn7 * ddt_scale), (var_qgd_fr_dn8 * ddt_scale), (var_qgd_fr_dn9 * ddt_scale), (var_qgd_fr_dn10 * ddt_scale), (var_qgd_fr_dn11 * ddt_scale), (var_qgd_fr_dn13 * ddt_scale), (var_qgd_fr_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e2412;
        let eq54_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq54_node_derivatives: [f64; 13] = [eq54_e2412_d_n0, eq54_e2412_d_n2, eq54_e2412_d_n3, eq54_e2412_d_n4, eq54_e2412_d_n5, eq54_e2412_d_n6, eq54_e2412_d_n7, eq54_e2412_d_n8, eq54_e2412_d_n9, eq54_e2412_d_n10, eq54_e2412_d_n11, eq54_e2412_d_n13, eq54_e2412_d_n14];
        let eq54_branch_derivative_indices: [usize; 0] = [];
        let eq54_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(14),
            Some(0),
            multiplicity * (eq54_value),
            &eq54_node_derivative_indices,
            &eq54_node_derivatives,
            &eq54_branch_derivative_indices,
            &eq54_branch_derivatives,
            multiplicity,
        );
        let (eq55_e2419, eq55_e2419_d_n0, eq55_e2419_d_n2, eq55_e2419_d_n3, eq55_e2419_d_n4, eq55_e2419_d_n5, eq55_e2419_d_n6, eq55_e2419_d_n7, eq55_e2419_d_n8, eq55_e2419_d_n9, eq55_e2419_d_n10, eq55_e2419_d_n11, eq55_e2419_d_n13, eq55_e2419_d_n14,) = {
    if (var_guard656 != 0.0) {
        let eq55_e2416: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 21, var_qg_acc);
        let eq55_e2417: f64 = (var_devsign * eq55_e2416);
        let eq55_e2417_d_n0: f64 = (var_devsign * (var_qg_acc_dn0 * ddt_scale));
        let eq55_e2417_d_n2: f64 = (var_devsign * (var_qg_acc_dn2 * ddt_scale));
        let eq55_e2417_d_n3: f64 = (var_devsign * (var_qg_acc_dn3 * ddt_scale));
        let eq55_e2417_d_n4: f64 = (var_devsign * (var_qg_acc_dn4 * ddt_scale));
        let eq55_e2417_d_n5: f64 = (var_devsign * (var_qg_acc_dn5 * ddt_scale));
        let eq55_e2417_d_n6: f64 = (var_devsign * (var_qg_acc_dn6 * ddt_scale));
        let eq55_e2417_d_n7: f64 = (var_devsign * (var_qg_acc_dn7 * ddt_scale));
        let eq55_e2417_d_n8: f64 = (var_devsign * (var_qg_acc_dn8 * ddt_scale));
        let eq55_e2417_d_n9: f64 = (var_devsign * (var_qg_acc_dn9 * ddt_scale));
        let eq55_e2417_d_n10: f64 = (var_devsign * (var_qg_acc_dn10 * ddt_scale));
        let eq55_e2417_d_n11: f64 = (var_devsign * (var_qg_acc_dn11 * ddt_scale));
        let eq55_e2417_d_n13: f64 = (var_devsign * (var_qg_acc_dn13 * ddt_scale));
        let eq55_e2417_d_n14: f64 = (var_devsign * (var_qg_acc_dn14 * ddt_scale));
        (eq55_e2417, eq55_e2417_d_n0, eq55_e2417_d_n2, eq55_e2417_d_n3, eq55_e2417_d_n4, eq55_e2417_d_n5, eq55_e2417_d_n6, eq55_e2417_d_n7, eq55_e2417_d_n8, eq55_e2417_d_n9, eq55_e2417_d_n10, eq55_e2417_d_n11, eq55_e2417_d_n13, eq55_e2417_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e2419;
        let eq55_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq55_node_derivatives: [f64; 13] = [eq55_e2419_d_n0, eq55_e2419_d_n2, eq55_e2419_d_n3, eq55_e2419_d_n4, eq55_e2419_d_n5, eq55_e2419_d_n6, eq55_e2419_d_n7, eq55_e2419_d_n8, eq55_e2419_d_n9, eq55_e2419_d_n10, eq55_e2419_d_n11, eq55_e2419_d_n13, eq55_e2419_d_n14];
        let eq55_branch_derivative_indices: [usize; 0] = [];
        let eq55_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq55_value),
            &eq55_node_derivative_indices,
            &eq55_node_derivatives,
            &eq55_branch_derivative_indices,
            &eq55_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_4(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        var_ctnoi: f64,
        var_ctnoi_dn0: f64,
        var_ctnoi_dn10: f64,
        var_ctnoi_dn11: f64,
        var_ctnoi_dn13: f64,
        var_ctnoi_dn14: f64,
        var_ctnoi_dn2: f64,
        var_ctnoi_dn3: f64,
        var_ctnoi_dn4: f64,
        var_ctnoi_dn5: f64,
        var_ctnoi_dn6: f64,
        var_ctnoi_dn7: f64,
        var_ctnoi_dn8: f64,
        var_ctnoi_dn9: f64,
        var_devsign: f64,
        var_gcrg: f64,
        var_gcrg_dn0: f64,
        var_gcrg_dn10: f64,
        var_gcrg_dn11: f64,
        var_gcrg_dn13: f64,
        var_gcrg_dn14: f64,
        var_gcrg_dn2: f64,
        var_gcrg_dn3: f64,
        var_gcrg_dn4: f64,
        var_gcrg_dn5: f64,
        var_gcrg_dn6: f64,
        var_gcrg_dn7: f64,
        var_gcrg_dn8: f64,
        var_gcrg_dn9: f64,
        var_gdpr: f64,
        var_gdpr_dn0: f64,
        var_gdpr_dn10: f64,
        var_gdpr_dn11: f64,
        var_gdpr_dn13: f64,
        var_gdpr_dn14: f64,
        var_gdpr_dn2: f64,
        var_gdpr_dn3: f64,
        var_gdpr_dn4: f64,
        var_gdpr_dn5: f64,
        var_gdpr_dn6: f64,
        var_gdpr_dn7: f64,
        var_gdpr_dn8: f64,
        var_gdpr_dn9: f64,
        var_gspr: f64,
        var_gspr_dn0: f64,
        var_gspr_dn10: f64,
        var_gspr_dn11: f64,
        var_gspr_dn13: f64,
        var_gspr_dn14: f64,
        var_gspr_dn2: f64,
        var_gspr_dn3: f64,
        var_gspr_dn4: f64,
        var_gspr_dn5: f64,
        var_gspr_dn6: f64,
        var_gspr_dn7: f64,
        var_gspr_dn8: f64,
        var_gspr_dn9: f64,
        var_gtau: f64,
        var_gtau_dn0: f64,
        var_gtau_dn10: f64,
        var_gtau_dn11: f64,
        var_gtau_dn13: f64,
        var_gtau_dn14: f64,
        var_gtau_dn2: f64,
        var_gtau_dn3: f64,
        var_gtau_dn4: f64,
        var_gtau_dn5: f64,
        var_gtau_dn6: f64,
        var_gtau_dn7: f64,
        var_gtau_dn8: f64,
        var_gtau_dn9: f64,
        var_guard656: f64,
        var_guard664: f64,
        var_guard665: f64,
        var_guard666: f64,
        var_guard667: f64,
        var_guard668: f64,
        var_guard669: f64,
        var_guard677: f64,
        var_gvs_d: f64,
        var_gvs_d_dn0: f64,
        var_gvs_d_dn10: f64,
        var_gvs_d_dn11: f64,
        var_gvs_d_dn13: f64,
        var_gvs_d_dn14: f64,
        var_gvs_d_dn2: f64,
        var_gvs_d_dn3: f64,
        var_gvs_d_dn4: f64,
        var_gvs_d_dn5: f64,
        var_gvs_d_dn6: f64,
        var_gvs_d_dn7: f64,
        var_gvs_d_dn8: f64,
        var_gvs_d_dn9: f64,
        var_gvs_s: f64,
        var_gvs_s_dn0: f64,
        var_gvs_s_dn10: f64,
        var_gvs_s_dn11: f64,
        var_gvs_s_dn13: f64,
        var_gvs_s_dn14: f64,
        var_gvs_s_dn2: f64,
        var_gvs_s_dn3: f64,
        var_gvs_s_dn4: f64,
        var_gvs_s_dn5: f64,
        var_gvs_s_dn6: f64,
        var_gvs_s_dn7: f64,
        var_gvs_s_dn8: f64,
        var_gvs_s_dn9: f64,
        var_qb_acc: f64,
        var_qb_acc_dn0: f64,
        var_qb_acc_dn10: f64,
        var_qb_acc_dn11: f64,
        var_qb_acc_dn13: f64,
        var_qb_acc_dn14: f64,
        var_qb_acc_dn2: f64,
        var_qb_acc_dn3: f64,
        var_qb_acc_dn4: f64,
        var_qb_acc_dn5: f64,
        var_qb_acc_dn6: f64,
        var_qb_acc_dn7: f64,
        var_qb_acc_dn8: f64,
        var_qb_acc_dn9: f64,
        var_qb_v: f64,
        var_qb_v_dn0: f64,
        var_qb_v_dn10: f64,
        var_qb_v_dn11: f64,
        var_qb_v_dn13: f64,
        var_qb_v_dn14: f64,
        var_qb_v_dn2: f64,
        var_qb_v_dn3: f64,
        var_qb_v_dn4: f64,
        var_qb_v_dn5: f64,
        var_qb_v_dn6: f64,
        var_qb_v_dn7: f64,
        var_qb_v_dn8: f64,
        var_qb_v_dn9: f64,
        var_qg_v: f64,
        var_qg_v_dn0: f64,
        var_qg_v_dn10: f64,
        var_qg_v_dn11: f64,
        var_qg_v_dn13: f64,
        var_qg_v_dn14: f64,
        var_qg_v_dn2: f64,
        var_qg_v_dn3: f64,
        var_qg_v_dn4: f64,
        var_qg_v_dn5: f64,
        var_qg_v_dn6: f64,
        var_qg_v_dn7: f64,
        var_qg_v_dn8: f64,
        var_qg_v_dn9: f64,
        var_sigrat: f64,
        var_sigrat_dn0: f64,
        var_sigrat_dn10: f64,
        var_sigrat_dn11: f64,
        var_sigrat_dn13: f64,
        var_sigrat_dn14: f64,
        var_sigrat_dn2: f64,
        var_sigrat_dn3: f64,
        var_sigrat_dn4: f64,
        var_sigrat_dn5: f64,
        var_sigrat_dn6: f64,
        var_sigrat_dn7: f64,
        var_sigrat_dn8: f64,
        var_sigrat_dn9: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq56_e2426, eq56_e2426_d_n0, eq56_e2426_d_n2, eq56_e2426_d_n3, eq56_e2426_d_n4, eq56_e2426_d_n5, eq56_e2426_d_n6, eq56_e2426_d_n7, eq56_e2426_d_n8, eq56_e2426_d_n9, eq56_e2426_d_n10, eq56_e2426_d_n11, eq56_e2426_d_n13, eq56_e2426_d_n14,) = {
    if (var_guard656 != 0.0) {
        let eq56_e2423: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 22, var_qb_acc);
        let eq56_e2424: f64 = (var_devsign * eq56_e2423);
        let eq56_e2424_d_n0: f64 = (var_devsign * (var_qb_acc_dn0 * ddt_scale));
        let eq56_e2424_d_n2: f64 = (var_devsign * (var_qb_acc_dn2 * ddt_scale));
        let eq56_e2424_d_n3: f64 = (var_devsign * (var_qb_acc_dn3 * ddt_scale));
        let eq56_e2424_d_n4: f64 = (var_devsign * (var_qb_acc_dn4 * ddt_scale));
        let eq56_e2424_d_n5: f64 = (var_devsign * (var_qb_acc_dn5 * ddt_scale));
        let eq56_e2424_d_n6: f64 = (var_devsign * (var_qb_acc_dn6 * ddt_scale));
        let eq56_e2424_d_n7: f64 = (var_devsign * (var_qb_acc_dn7 * ddt_scale));
        let eq56_e2424_d_n8: f64 = (var_devsign * (var_qb_acc_dn8 * ddt_scale));
        let eq56_e2424_d_n9: f64 = (var_devsign * (var_qb_acc_dn9 * ddt_scale));
        let eq56_e2424_d_n10: f64 = (var_devsign * (var_qb_acc_dn10 * ddt_scale));
        let eq56_e2424_d_n11: f64 = (var_devsign * (var_qb_acc_dn11 * ddt_scale));
        let eq56_e2424_d_n13: f64 = (var_devsign * (var_qb_acc_dn13 * ddt_scale));
        let eq56_e2424_d_n14: f64 = (var_devsign * (var_qb_acc_dn14 * ddt_scale));
        (eq56_e2424, eq56_e2424_d_n0, eq56_e2424_d_n2, eq56_e2424_d_n3, eq56_e2424_d_n4, eq56_e2424_d_n5, eq56_e2424_d_n6, eq56_e2424_d_n7, eq56_e2424_d_n8, eq56_e2424_d_n9, eq56_e2424_d_n10, eq56_e2424_d_n11, eq56_e2424_d_n13, eq56_e2424_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq56_value: f64 = eq56_e2426;
        let eq56_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq56_node_derivatives: [f64; 13] = [eq56_e2426_d_n0, eq56_e2426_d_n2, eq56_e2426_d_n3, eq56_e2426_d_n4, eq56_e2426_d_n5, eq56_e2426_d_n6, eq56_e2426_d_n7, eq56_e2426_d_n8, eq56_e2426_d_n9, eq56_e2426_d_n10, eq56_e2426_d_n11, eq56_e2426_d_n13, eq56_e2426_d_n14];
        let eq56_branch_derivative_indices: [usize; 0] = [];
        let eq56_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(6),
            multiplicity * (eq56_value),
            &eq56_node_derivative_indices,
            &eq56_node_derivatives,
            &eq56_branch_derivative_indices,
            &eq56_branch_derivatives,
            multiplicity,
        );
        let (eq57_e2432, eq57_e2432_d_n0, eq57_e2432_d_n2, eq57_e2432_d_n3, eq57_e2432_d_n4, eq57_e2432_d_n5, eq57_e2432_d_n6, eq57_e2432_d_n7, eq57_e2432_d_n8, eq57_e2432_d_n9, eq57_e2432_d_n10, eq57_e2432_d_n11, eq57_e2432_d_n13, eq57_e2432_d_n14,) = {
    if (var_guard664 != 0.0) {
        let eq57_e2430: f64 = ((nv0 - nv9) * var_gdpr);
        let eq57_e2430_d_n0: f64 = (var_gdpr + ((nv0 - nv9) * var_gdpr_dn0));
        let eq57_e2430_d_n2: f64 = ((nv0 - nv9) * var_gdpr_dn2);
        let eq57_e2430_d_n3: f64 = ((nv0 - nv9) * var_gdpr_dn3);
        let eq57_e2430_d_n4: f64 = ((nv0 - nv9) * var_gdpr_dn4);
        let eq57_e2430_d_n5: f64 = ((nv0 - nv9) * var_gdpr_dn5);
        let eq57_e2430_d_n6: f64 = ((nv0 - nv9) * var_gdpr_dn6);
        let eq57_e2430_d_n7: f64 = ((nv0 - nv9) * var_gdpr_dn7);
        let eq57_e2430_d_n8: f64 = ((nv0 - nv9) * var_gdpr_dn8);
        let eq57_e2430_d_n9: f64 = ((-var_gdpr) + ((nv0 - nv9) * var_gdpr_dn9));
        let eq57_e2430_d_n10: f64 = ((nv0 - nv9) * var_gdpr_dn10);
        let eq57_e2430_d_n11: f64 = ((nv0 - nv9) * var_gdpr_dn11);
        let eq57_e2430_d_n13: f64 = ((nv0 - nv9) * var_gdpr_dn13);
        let eq57_e2430_d_n14: f64 = ((nv0 - nv9) * var_gdpr_dn14);
        (eq57_e2430, eq57_e2430_d_n0, eq57_e2430_d_n2, eq57_e2430_d_n3, eq57_e2430_d_n4, eq57_e2430_d_n5, eq57_e2430_d_n6, eq57_e2430_d_n7, eq57_e2430_d_n8, eq57_e2430_d_n9, eq57_e2430_d_n10, eq57_e2430_d_n11, eq57_e2430_d_n13, eq57_e2430_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e2432;
        let eq57_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq57_node_derivatives: [f64; 13] = [eq57_e2432_d_n0, eq57_e2432_d_n2, eq57_e2432_d_n3, eq57_e2432_d_n4, eq57_e2432_d_n5, eq57_e2432_d_n6, eq57_e2432_d_n7, eq57_e2432_d_n8, eq57_e2432_d_n9, eq57_e2432_d_n10, eq57_e2432_d_n11, eq57_e2432_d_n13, eq57_e2432_d_n14];
        let eq57_branch_derivative_indices: [usize; 0] = [];
        let eq57_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(0),
            Some(9),
            multiplicity * (eq57_value),
            &eq57_node_derivative_indices,
            &eq57_node_derivatives,
            &eq57_branch_derivative_indices,
            &eq57_branch_derivatives,
            multiplicity,
        );
        let (eq58_e2440, eq58_e2440_d_n0, eq58_e2440_d_n2, eq58_e2440_d_n3, eq58_e2440_d_n4, eq58_e2440_d_n5, eq58_e2440_d_n6, eq58_e2440_d_n7, eq58_e2440_d_n8, eq58_e2440_d_n9, eq58_e2440_d_n10, eq58_e2440_d_n11, eq58_e2440_d_n13, eq58_e2440_d_n14,) = {
    if ((var_guard664 != 0.0) && (var_guard665 != 0.0)) {
        let eq58_e2438: f64 = ((nv9 - nv7) * var_gvs_d);
        let eq58_e2438_d_n0: f64 = ((nv9 - nv7) * var_gvs_d_dn0);
        let eq58_e2438_d_n2: f64 = ((nv9 - nv7) * var_gvs_d_dn2);
        let eq58_e2438_d_n3: f64 = ((nv9 - nv7) * var_gvs_d_dn3);
        let eq58_e2438_d_n4: f64 = ((nv9 - nv7) * var_gvs_d_dn4);
        let eq58_e2438_d_n5: f64 = ((nv9 - nv7) * var_gvs_d_dn5);
        let eq58_e2438_d_n6: f64 = ((nv9 - nv7) * var_gvs_d_dn6);
        let eq58_e2438_d_n7: f64 = ((-var_gvs_d) + ((nv9 - nv7) * var_gvs_d_dn7));
        let eq58_e2438_d_n8: f64 = ((nv9 - nv7) * var_gvs_d_dn8);
        let eq58_e2438_d_n9: f64 = (var_gvs_d + ((nv9 - nv7) * var_gvs_d_dn9));
        let eq58_e2438_d_n10: f64 = ((nv9 - nv7) * var_gvs_d_dn10);
        let eq58_e2438_d_n11: f64 = ((nv9 - nv7) * var_gvs_d_dn11);
        let eq58_e2438_d_n13: f64 = ((nv9 - nv7) * var_gvs_d_dn13);
        let eq58_e2438_d_n14: f64 = ((nv9 - nv7) * var_gvs_d_dn14);
        (eq58_e2438, eq58_e2438_d_n0, eq58_e2438_d_n2, eq58_e2438_d_n3, eq58_e2438_d_n4, eq58_e2438_d_n5, eq58_e2438_d_n6, eq58_e2438_d_n7, eq58_e2438_d_n8, eq58_e2438_d_n9, eq58_e2438_d_n10, eq58_e2438_d_n11, eq58_e2438_d_n13, eq58_e2438_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq58_value: f64 = eq58_e2440;
        let eq58_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq58_node_derivatives: [f64; 13] = [eq58_e2440_d_n0, eq58_e2440_d_n2, eq58_e2440_d_n3, eq58_e2440_d_n4, eq58_e2440_d_n5, eq58_e2440_d_n6, eq58_e2440_d_n7, eq58_e2440_d_n8, eq58_e2440_d_n9, eq58_e2440_d_n10, eq58_e2440_d_n11, eq58_e2440_d_n13, eq58_e2440_d_n14];
        let eq58_branch_derivative_indices: [usize; 0] = [];
        let eq58_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq58_value),
            &eq58_node_derivative_indices,
            &eq58_node_derivatives,
            &eq58_branch_derivative_indices,
            &eq58_branch_derivatives,
            multiplicity,
        );
        let (eq59_e2447,) = {
    if ((var_guard664 != 0.0) && (var_guard665 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq59_value: f64 = eq59_e2447;
        stamper.stamp_potential_const_local(
            1,
            eq59_value,
        );
        let (eq60_e2452,) = {
    if (var_guard664 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq60_value: f64 = eq60_e2452;
        stamper.stamp_potential_const_local(
            2,
            eq60_value,
        );
        let (eq61_e2457,) = {
    if (var_guard664 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq61_value: f64 = eq61_e2457;
        stamper.stamp_potential_const_local(
            3,
            eq61_value,
        );
        let (eq62_e2463, eq62_e2463_d_n0, eq62_e2463_d_n2, eq62_e2463_d_n3, eq62_e2463_d_n4, eq62_e2463_d_n5, eq62_e2463_d_n6, eq62_e2463_d_n7, eq62_e2463_d_n8, eq62_e2463_d_n9, eq62_e2463_d_n10, eq62_e2463_d_n11, eq62_e2463_d_n13, eq62_e2463_d_n14,) = {
    if (var_guard666 != 0.0) {
        let eq62_e2461: f64 = ((nv2 - nv8) * var_gspr);
        let eq62_e2461_d_n0: f64 = ((nv2 - nv8) * var_gspr_dn0);
        let eq62_e2461_d_n2: f64 = (var_gspr + ((nv2 - nv8) * var_gspr_dn2));
        let eq62_e2461_d_n3: f64 = ((nv2 - nv8) * var_gspr_dn3);
        let eq62_e2461_d_n4: f64 = ((nv2 - nv8) * var_gspr_dn4);
        let eq62_e2461_d_n5: f64 = ((nv2 - nv8) * var_gspr_dn5);
        let eq62_e2461_d_n6: f64 = ((nv2 - nv8) * var_gspr_dn6);
        let eq62_e2461_d_n7: f64 = ((nv2 - nv8) * var_gspr_dn7);
        let eq62_e2461_d_n8: f64 = ((-var_gspr) + ((nv2 - nv8) * var_gspr_dn8));
        let eq62_e2461_d_n9: f64 = ((nv2 - nv8) * var_gspr_dn9);
        let eq62_e2461_d_n10: f64 = ((nv2 - nv8) * var_gspr_dn10);
        let eq62_e2461_d_n11: f64 = ((nv2 - nv8) * var_gspr_dn11);
        let eq62_e2461_d_n13: f64 = ((nv2 - nv8) * var_gspr_dn13);
        let eq62_e2461_d_n14: f64 = ((nv2 - nv8) * var_gspr_dn14);
        (eq62_e2461, eq62_e2461_d_n0, eq62_e2461_d_n2, eq62_e2461_d_n3, eq62_e2461_d_n4, eq62_e2461_d_n5, eq62_e2461_d_n6, eq62_e2461_d_n7, eq62_e2461_d_n8, eq62_e2461_d_n9, eq62_e2461_d_n10, eq62_e2461_d_n11, eq62_e2461_d_n13, eq62_e2461_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e2463;
        let eq62_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq62_node_derivatives: [f64; 13] = [eq62_e2463_d_n0, eq62_e2463_d_n2, eq62_e2463_d_n3, eq62_e2463_d_n4, eq62_e2463_d_n5, eq62_e2463_d_n6, eq62_e2463_d_n7, eq62_e2463_d_n8, eq62_e2463_d_n9, eq62_e2463_d_n10, eq62_e2463_d_n11, eq62_e2463_d_n13, eq62_e2463_d_n14];
        let eq62_branch_derivative_indices: [usize; 0] = [];
        let eq62_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq62_value),
            &eq62_node_derivative_indices,
            &eq62_node_derivatives,
            &eq62_branch_derivative_indices,
            &eq62_branch_derivatives,
            multiplicity,
        );
        let (eq63_e2471, eq63_e2471_d_n0, eq63_e2471_d_n2, eq63_e2471_d_n3, eq63_e2471_d_n4, eq63_e2471_d_n5, eq63_e2471_d_n6, eq63_e2471_d_n7, eq63_e2471_d_n8, eq63_e2471_d_n9, eq63_e2471_d_n10, eq63_e2471_d_n11, eq63_e2471_d_n13, eq63_e2471_d_n14,) = {
    if ((var_guard666 != 0.0) && (var_guard667 != 0.0)) {
        let eq63_e2469: f64 = ((nv8 - nv6) * var_gvs_s);
        let eq63_e2469_d_n0: f64 = ((nv8 - nv6) * var_gvs_s_dn0);
        let eq63_e2469_d_n2: f64 = ((nv8 - nv6) * var_gvs_s_dn2);
        let eq63_e2469_d_n3: f64 = ((nv8 - nv6) * var_gvs_s_dn3);
        let eq63_e2469_d_n4: f64 = ((nv8 - nv6) * var_gvs_s_dn4);
        let eq63_e2469_d_n5: f64 = ((nv8 - nv6) * var_gvs_s_dn5);
        let eq63_e2469_d_n6: f64 = ((-var_gvs_s) + ((nv8 - nv6) * var_gvs_s_dn6));
        let eq63_e2469_d_n7: f64 = ((nv8 - nv6) * var_gvs_s_dn7);
        let eq63_e2469_d_n8: f64 = (var_gvs_s + ((nv8 - nv6) * var_gvs_s_dn8));
        let eq63_e2469_d_n9: f64 = ((nv8 - nv6) * var_gvs_s_dn9);
        let eq63_e2469_d_n10: f64 = ((nv8 - nv6) * var_gvs_s_dn10);
        let eq63_e2469_d_n11: f64 = ((nv8 - nv6) * var_gvs_s_dn11);
        let eq63_e2469_d_n13: f64 = ((nv8 - nv6) * var_gvs_s_dn13);
        let eq63_e2469_d_n14: f64 = ((nv8 - nv6) * var_gvs_s_dn14);
        (eq63_e2469, eq63_e2469_d_n0, eq63_e2469_d_n2, eq63_e2469_d_n3, eq63_e2469_d_n4, eq63_e2469_d_n5, eq63_e2469_d_n6, eq63_e2469_d_n7, eq63_e2469_d_n8, eq63_e2469_d_n9, eq63_e2469_d_n10, eq63_e2469_d_n11, eq63_e2469_d_n13, eq63_e2469_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e2471;
        let eq63_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq63_node_derivatives: [f64; 13] = [eq63_e2471_d_n0, eq63_e2471_d_n2, eq63_e2471_d_n3, eq63_e2471_d_n4, eq63_e2471_d_n5, eq63_e2471_d_n6, eq63_e2471_d_n7, eq63_e2471_d_n8, eq63_e2471_d_n9, eq63_e2471_d_n10, eq63_e2471_d_n11, eq63_e2471_d_n13, eq63_e2471_d_n14];
        let eq63_branch_derivative_indices: [usize; 0] = [];
        let eq63_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq63_value),
            &eq63_node_derivative_indices,
            &eq63_node_derivatives,
            &eq63_branch_derivative_indices,
            &eq63_branch_derivatives,
            multiplicity,
        );
        let (eq64_e2478,) = {
    if ((var_guard666 != 0.0) && (var_guard667 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq64_value: f64 = eq64_e2478;
        stamper.stamp_potential_const_local(
            4,
            eq64_value,
        );
        let (eq65_e2483,) = {
    if (var_guard666 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq65_value: f64 = eq65_e2483;
        stamper.stamp_potential_const_local(
            5,
            eq65_value,
        );
        let (eq66_e2488,) = {
    if (var_guard666 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq66_value: f64 = eq66_e2488;
        stamper.stamp_potential_const_local(
            6,
            eq66_value,
        );
        let (eq67_e2494, eq67_e2494_d_n0, eq67_e2494_d_n2, eq67_e2494_d_n3, eq67_e2494_d_n4, eq67_e2494_d_n5, eq67_e2494_d_n6, eq67_e2494_d_n7, eq67_e2494_d_n8, eq67_e2494_d_n9, eq67_e2494_d_n10, eq67_e2494_d_n11, eq67_e2494_d_n12, eq67_e2494_d_n13, eq67_e2494_d_n14,) = {
    if (var_guard668 != 0.0) {
        let eq67_e2492: f64 = ((nv12 - nv11) * var_gcrg);
        let eq67_e2492_d_n0: f64 = ((nv12 - nv11) * var_gcrg_dn0);
        let eq67_e2492_d_n2: f64 = ((nv12 - nv11) * var_gcrg_dn2);
        let eq67_e2492_d_n3: f64 = ((nv12 - nv11) * var_gcrg_dn3);
        let eq67_e2492_d_n4: f64 = ((nv12 - nv11) * var_gcrg_dn4);
        let eq67_e2492_d_n5: f64 = ((nv12 - nv11) * var_gcrg_dn5);
        let eq67_e2492_d_n6: f64 = ((nv12 - nv11) * var_gcrg_dn6);
        let eq67_e2492_d_n7: f64 = ((nv12 - nv11) * var_gcrg_dn7);
        let eq67_e2492_d_n8: f64 = ((nv12 - nv11) * var_gcrg_dn8);
        let eq67_e2492_d_n9: f64 = ((nv12 - nv11) * var_gcrg_dn9);
        let eq67_e2492_d_n10: f64 = ((nv12 - nv11) * var_gcrg_dn10);
        let eq67_e2492_d_n11: f64 = ((-var_gcrg) + ((nv12 - nv11) * var_gcrg_dn11));
        let eq67_e2492_d_n13: f64 = ((nv12 - nv11) * var_gcrg_dn13);
        let eq67_e2492_d_n14: f64 = ((nv12 - nv11) * var_gcrg_dn14);
        (eq67_e2492, eq67_e2492_d_n0, eq67_e2492_d_n2, eq67_e2492_d_n3, eq67_e2492_d_n4, eq67_e2492_d_n5, eq67_e2492_d_n6, eq67_e2492_d_n7, eq67_e2492_d_n8, eq67_e2492_d_n9, eq67_e2492_d_n10, eq67_e2492_d_n11, var_gcrg, eq67_e2492_d_n13, eq67_e2492_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq67_value: f64 = eq67_e2494;
        let eq67_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq67_node_derivatives: [f64; 14] = [eq67_e2494_d_n0, eq67_e2494_d_n2, eq67_e2494_d_n3, eq67_e2494_d_n4, eq67_e2494_d_n5, eq67_e2494_d_n6, eq67_e2494_d_n7, eq67_e2494_d_n8, eq67_e2494_d_n9, eq67_e2494_d_n10, eq67_e2494_d_n11, eq67_e2494_d_n12, eq67_e2494_d_n13, eq67_e2494_d_n14];
        let eq67_branch_derivative_indices: [usize; 0] = [];
        let eq67_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(12),
            Some(11),
            multiplicity * (eq67_value),
            &eq67_node_derivative_indices,
            &eq67_node_derivatives,
            &eq67_branch_derivative_indices,
            &eq67_branch_derivatives,
            multiplicity,
        );
        let (eq69_e2506, eq69_e2506_d_n0, eq69_e2506_d_n2, eq69_e2506_d_n3, eq69_e2506_d_n4, eq69_e2506_d_n5, eq69_e2506_d_n6, eq69_e2506_d_n7, eq69_e2506_d_n8, eq69_e2506_d_n9, eq69_e2506_d_n10, eq69_e2506_d_n11, eq69_e2506_d_n13, eq69_e2506_d_n14,) = {
    if (var_guard669 != 0.0) {
        let eq69_e2503: f64 = (var_qg_v - var_qb_v);
        let eq69_e2503_d_n0: f64 = (var_qg_v_dn0 - var_qb_v_dn0);
        let eq69_e2503_d_n2: f64 = (var_qg_v_dn2 - var_qb_v_dn2);
        let eq69_e2503_d_n3: f64 = (var_qg_v_dn3 - var_qb_v_dn3);
        let eq69_e2503_d_n4: f64 = (var_qg_v_dn4 - var_qb_v_dn4);
        let eq69_e2503_d_n5: f64 = (var_qg_v_dn5 - var_qb_v_dn5);
        let eq69_e2503_d_n6: f64 = (var_qg_v_dn6 - var_qb_v_dn6);
        let eq69_e2503_d_n7: f64 = (var_qg_v_dn7 - var_qb_v_dn7);
        let eq69_e2503_d_n8: f64 = (var_qg_v_dn8 - var_qb_v_dn8);
        let eq69_e2503_d_n9: f64 = (var_qg_v_dn9 - var_qb_v_dn9);
        let eq69_e2503_d_n10: f64 = (var_qg_v_dn10 - var_qb_v_dn10);
        let eq69_e2503_d_n11: f64 = (var_qg_v_dn11 - var_qb_v_dn11);
        let eq69_e2503_d_n13: f64 = (var_qg_v_dn13 - var_qb_v_dn13);
        let eq69_e2503_d_n14: f64 = (var_qg_v_dn14 - var_qb_v_dn14);
        let eq69_e2504: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 23, eq69_e2503);
        (eq69_e2504, (eq69_e2503_d_n0 * ddt_scale), (eq69_e2503_d_n2 * ddt_scale), (eq69_e2503_d_n3 * ddt_scale), (eq69_e2503_d_n4 * ddt_scale), (eq69_e2503_d_n5 * ddt_scale), (eq69_e2503_d_n6 * ddt_scale), (eq69_e2503_d_n7 * ddt_scale), (eq69_e2503_d_n8 * ddt_scale), (eq69_e2503_d_n9 * ddt_scale), (eq69_e2503_d_n10 * ddt_scale), (eq69_e2503_d_n11 * ddt_scale), (eq69_e2503_d_n13 * ddt_scale), (eq69_e2503_d_n14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq69_value: f64 = eq69_e2506;
        let eq69_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq69_node_derivatives: [f64; 13] = [eq69_e2506_d_n0, eq69_e2506_d_n2, eq69_e2506_d_n3, eq69_e2506_d_n4, eq69_e2506_d_n5, eq69_e2506_d_n6, eq69_e2506_d_n7, eq69_e2506_d_n8, eq69_e2506_d_n9, eq69_e2506_d_n10, eq69_e2506_d_n11, eq69_e2506_d_n13, eq69_e2506_d_n14];
        let eq69_branch_derivative_indices: [usize; 0] = [];
        let eq69_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(15),
            None,
            multiplicity * (eq69_value),
            &eq69_node_derivative_indices,
            &eq69_node_derivatives,
            &eq69_branch_derivative_indices,
            &eq69_branch_derivatives,
            multiplicity,
        );
        let (eq70_e2512, eq70_e2512_d_n0, eq70_e2512_d_n2, eq70_e2512_d_n3, eq70_e2512_d_n4, eq70_e2512_d_n5, eq70_e2512_d_n6, eq70_e2512_d_n7, eq70_e2512_d_n8, eq70_e2512_d_n9, eq70_e2512_d_n10, eq70_e2512_d_n11, eq70_e2512_d_n13, eq70_e2512_d_n14, eq70_e2512_d_n15,) = {
    if (var_guard669 != 0.0) {
        let eq70_e2510: f64 = ((nv15 - 0.0) * var_gtau);
        let eq70_e2510_d_n0: f64 = ((nv15 - 0.0) * var_gtau_dn0);
        let eq70_e2510_d_n2: f64 = ((nv15 - 0.0) * var_gtau_dn2);
        let eq70_e2510_d_n3: f64 = ((nv15 - 0.0) * var_gtau_dn3);
        let eq70_e2510_d_n4: f64 = ((nv15 - 0.0) * var_gtau_dn4);
        let eq70_e2510_d_n5: f64 = ((nv15 - 0.0) * var_gtau_dn5);
        let eq70_e2510_d_n6: f64 = ((nv15 - 0.0) * var_gtau_dn6);
        let eq70_e2510_d_n7: f64 = ((nv15 - 0.0) * var_gtau_dn7);
        let eq70_e2510_d_n8: f64 = ((nv15 - 0.0) * var_gtau_dn8);
        let eq70_e2510_d_n9: f64 = ((nv15 - 0.0) * var_gtau_dn9);
        let eq70_e2510_d_n10: f64 = ((nv15 - 0.0) * var_gtau_dn10);
        let eq70_e2510_d_n11: f64 = ((nv15 - 0.0) * var_gtau_dn11);
        let eq70_e2510_d_n13: f64 = ((nv15 - 0.0) * var_gtau_dn13);
        let eq70_e2510_d_n14: f64 = ((nv15 - 0.0) * var_gtau_dn14);
        (eq70_e2510, eq70_e2510_d_n0, eq70_e2510_d_n2, eq70_e2510_d_n3, eq70_e2510_d_n4, eq70_e2510_d_n5, eq70_e2510_d_n6, eq70_e2510_d_n7, eq70_e2510_d_n8, eq70_e2510_d_n9, eq70_e2510_d_n10, eq70_e2510_d_n11, eq70_e2510_d_n13, eq70_e2510_d_n14, var_gtau,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq70_value: f64 = eq70_e2512;
        let eq70_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15];
        let eq70_node_derivatives: [f64; 14] = [eq70_e2512_d_n0, eq70_e2512_d_n2, eq70_e2512_d_n3, eq70_e2512_d_n4, eq70_e2512_d_n5, eq70_e2512_d_n6, eq70_e2512_d_n7, eq70_e2512_d_n8, eq70_e2512_d_n9, eq70_e2512_d_n10, eq70_e2512_d_n11, eq70_e2512_d_n13, eq70_e2512_d_n14, eq70_e2512_d_n15];
        let eq70_branch_derivative_indices: [usize; 0] = [];
        let eq70_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(15),
            None,
            multiplicity * (eq70_value),
            &eq70_node_derivative_indices,
            &eq70_node_derivatives,
            &eq70_branch_derivative_indices,
            &eq70_branch_derivatives,
            multiplicity,
        );
        let (eq71_e2519, eq71_e2519_d_n15,) = {
    if (var_guard669 != 0.0) {
        let eq71_e2516: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 24, (nv15 - 0.0));
        let eq71_e2517: f64 = (1e-9 * eq71_e2516);
        (eq71_e2517, (1e-9 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq71_value: f64 = eq71_e2519;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (eq71_value),
            15,
            multiplicity * (eq71_e2519_d_n15),
        );
        let (eq95_e2707, eq95_e2707_d_n0, eq95_e2707_d_n2, eq95_e2707_d_n3, eq95_e2707_d_n4, eq95_e2707_d_n5, eq95_e2707_d_n6, eq95_e2707_d_n7, eq95_e2707_d_n8, eq95_e2707_d_n9, eq95_e2707_d_n10, eq95_e2707_d_n11, eq95_e2707_d_n13, eq95_e2707_d_n14, eq95_e2707_d_n16,) = {
    if (var_guard677 == 0.0) {
        let eq95_e2705: f64 = (var_ctnoi * (nv16 - 0.0));
        let eq95_e2705_d_n0: f64 = (var_ctnoi_dn0 * (nv16 - 0.0));
        let eq95_e2705_d_n2: f64 = (var_ctnoi_dn2 * (nv16 - 0.0));
        let eq95_e2705_d_n3: f64 = (var_ctnoi_dn3 * (nv16 - 0.0));
        let eq95_e2705_d_n4: f64 = (var_ctnoi_dn4 * (nv16 - 0.0));
        let eq95_e2705_d_n5: f64 = (var_ctnoi_dn5 * (nv16 - 0.0));
        let eq95_e2705_d_n6: f64 = (var_ctnoi_dn6 * (nv16 - 0.0));
        let eq95_e2705_d_n7: f64 = (var_ctnoi_dn7 * (nv16 - 0.0));
        let eq95_e2705_d_n8: f64 = (var_ctnoi_dn8 * (nv16 - 0.0));
        let eq95_e2705_d_n9: f64 = (var_ctnoi_dn9 * (nv16 - 0.0));
        let eq95_e2705_d_n10: f64 = (var_ctnoi_dn10 * (nv16 - 0.0));
        let eq95_e2705_d_n11: f64 = (var_ctnoi_dn11 * (nv16 - 0.0));
        let eq95_e2705_d_n13: f64 = (var_ctnoi_dn13 * (nv16 - 0.0));
        let eq95_e2705_d_n14: f64 = (var_ctnoi_dn14 * (nv16 - 0.0));
        (eq95_e2705, eq95_e2705_d_n0, eq95_e2705_d_n2, eq95_e2705_d_n3, eq95_e2705_d_n4, eq95_e2705_d_n5, eq95_e2705_d_n6, eq95_e2705_d_n7, eq95_e2705_d_n8, eq95_e2705_d_n9, eq95_e2705_d_n10, eq95_e2705_d_n11, eq95_e2705_d_n13, eq95_e2705_d_n14, var_ctnoi,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq95_value: f64 = eq95_e2707;
        let eq95_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 16];
        let eq95_node_derivatives: [f64; 14] = [eq95_e2707_d_n0, eq95_e2707_d_n2, eq95_e2707_d_n3, eq95_e2707_d_n4, eq95_e2707_d_n5, eq95_e2707_d_n6, eq95_e2707_d_n7, eq95_e2707_d_n8, eq95_e2707_d_n9, eq95_e2707_d_n10, eq95_e2707_d_n11, eq95_e2707_d_n13, eq95_e2707_d_n14, eq95_e2707_d_n16];
        let eq95_branch_derivative_indices: [usize; 0] = [];
        let eq95_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq95_value),
            &eq95_node_derivative_indices,
            &eq95_node_derivatives,
            &eq95_branch_derivative_indices,
            &eq95_branch_derivatives,
            multiplicity,
        );
        let (eq96_e2717, eq96_e2717_d_n0, eq96_e2717_d_n2, eq96_e2717_d_n3, eq96_e2717_d_n4, eq96_e2717_d_n5, eq96_e2717_d_n6, eq96_e2717_d_n7, eq96_e2717_d_n8, eq96_e2717_d_n9, eq96_e2717_d_n10, eq96_e2717_d_n11, eq96_e2717_d_n13, eq96_e2717_d_n14, eq96_e2717_d_n16,) = {
    if (var_guard677 == 0.0) {
        let eq96_e2712: f64 = (0.7071 * var_sigrat);
        let eq96_e2712_d_n0: f64 = (0.7071 * var_sigrat_dn0);
        let eq96_e2712_d_n2: f64 = (0.7071 * var_sigrat_dn2);
        let eq96_e2712_d_n3: f64 = (0.7071 * var_sigrat_dn3);
        let eq96_e2712_d_n4: f64 = (0.7071 * var_sigrat_dn4);
        let eq96_e2712_d_n5: f64 = (0.7071 * var_sigrat_dn5);
        let eq96_e2712_d_n6: f64 = (0.7071 * var_sigrat_dn6);
        let eq96_e2712_d_n7: f64 = (0.7071 * var_sigrat_dn7);
        let eq96_e2712_d_n8: f64 = (0.7071 * var_sigrat_dn8);
        let eq96_e2712_d_n9: f64 = (0.7071 * var_sigrat_dn9);
        let eq96_e2712_d_n10: f64 = (0.7071 * var_sigrat_dn10);
        let eq96_e2712_d_n11: f64 = (0.7071 * var_sigrat_dn11);
        let eq96_e2712_d_n13: f64 = (0.7071 * var_sigrat_dn13);
        let eq96_e2712_d_n14: f64 = (0.7071 * var_sigrat_dn14);
        let eq96_e2714: f64 = (eq96_e2712 * (nv16 - 0.0));
        let eq96_e2714_d_n0: f64 = (eq96_e2712_d_n0 * (nv16 - 0.0));
        let eq96_e2714_d_n2: f64 = (eq96_e2712_d_n2 * (nv16 - 0.0));
        let eq96_e2714_d_n3: f64 = (eq96_e2712_d_n3 * (nv16 - 0.0));
        let eq96_e2714_d_n4: f64 = (eq96_e2712_d_n4 * (nv16 - 0.0));
        let eq96_e2714_d_n5: f64 = (eq96_e2712_d_n5 * (nv16 - 0.0));
        let eq96_e2714_d_n6: f64 = (eq96_e2712_d_n6 * (nv16 - 0.0));
        let eq96_e2714_d_n7: f64 = (eq96_e2712_d_n7 * (nv16 - 0.0));
        let eq96_e2714_d_n8: f64 = (eq96_e2712_d_n8 * (nv16 - 0.0));
        let eq96_e2714_d_n9: f64 = (eq96_e2712_d_n9 * (nv16 - 0.0));
        let eq96_e2714_d_n10: f64 = (eq96_e2712_d_n10 * (nv16 - 0.0));
        let eq96_e2714_d_n11: f64 = (eq96_e2712_d_n11 * (nv16 - 0.0));
        let eq96_e2714_d_n13: f64 = (eq96_e2712_d_n13 * (nv16 - 0.0));
        let eq96_e2714_d_n14: f64 = (eq96_e2712_d_n14 * (nv16 - 0.0));
        let eq96_e2715: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 25, eq96_e2714);
        (eq96_e2715, (eq96_e2714_d_n0 * ddt_scale), (eq96_e2714_d_n2 * ddt_scale), (eq96_e2714_d_n3 * ddt_scale), (eq96_e2714_d_n4 * ddt_scale), (eq96_e2714_d_n5 * ddt_scale), (eq96_e2714_d_n6 * ddt_scale), (eq96_e2714_d_n7 * ddt_scale), (eq96_e2714_d_n8 * ddt_scale), (eq96_e2714_d_n9 * ddt_scale), (eq96_e2714_d_n10 * ddt_scale), (eq96_e2714_d_n11 * ddt_scale), (eq96_e2714_d_n13 * ddt_scale), (eq96_e2714_d_n14 * ddt_scale), (eq96_e2712 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq96_value: f64 = eq96_e2717;
        let eq96_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 16];
        let eq96_node_derivatives: [f64; 14] = [eq96_e2717_d_n0, eq96_e2717_d_n2, eq96_e2717_d_n3, eq96_e2717_d_n4, eq96_e2717_d_n5, eq96_e2717_d_n6, eq96_e2717_d_n7, eq96_e2717_d_n8, eq96_e2717_d_n9, eq96_e2717_d_n10, eq96_e2717_d_n11, eq96_e2717_d_n13, eq96_e2717_d_n14, eq96_e2717_d_n16];
        let eq96_branch_derivative_indices: [usize; 0] = [];
        let eq96_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq96_value),
            &eq96_node_derivative_indices,
            &eq96_node_derivatives,
            &eq96_branch_derivative_indices,
            &eq96_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_5(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        var_cth: f64,
        var_cth_dn0: f64,
        var_cth_dn10: f64,
        var_cth_dn11: f64,
        var_cth_dn13: f64,
        var_cth_dn14: f64,
        var_cth_dn2: f64,
        var_cth_dn3: f64,
        var_cth_dn4: f64,
        var_cth_dn5: f64,
        var_cth_dn6: f64,
        var_cth_dn7: f64,
        var_cth_dn8: f64,
        var_cth_dn9: f64,
        var_devsign: f64,
        var_gdpr: f64,
        var_gdpr_dn0: f64,
        var_gdpr_dn10: f64,
        var_gdpr_dn11: f64,
        var_gdpr_dn13: f64,
        var_gdpr_dn14: f64,
        var_gdpr_dn2: f64,
        var_gdpr_dn3: f64,
        var_gdpr_dn4: f64,
        var_gdpr_dn5: f64,
        var_gdpr_dn6: f64,
        var_gdpr_dn7: f64,
        var_gdpr_dn8: f64,
        var_gdpr_dn9: f64,
        var_gspr: f64,
        var_gspr_dn0: f64,
        var_gspr_dn10: f64,
        var_gspr_dn11: f64,
        var_gspr_dn13: f64,
        var_gspr_dn14: f64,
        var_gspr_dn2: f64,
        var_gspr_dn3: f64,
        var_gspr_dn4: f64,
        var_gspr_dn5: f64,
        var_gspr_dn6: f64,
        var_gspr_dn7: f64,
        var_gspr_dn8: f64,
        var_gspr_dn9: f64,
        var_gth: f64,
        var_gth_dn0: f64,
        var_gth_dn10: f64,
        var_gth_dn11: f64,
        var_gth_dn13: f64,
        var_gth_dn14: f64,
        var_gth_dn2: f64,
        var_gth_dn3: f64,
        var_gth_dn4: f64,
        var_gth_dn5: f64,
        var_gth_dn6: f64,
        var_gth_dn7: f64,
        var_gth_dn8: f64,
        var_gth_dn9: f64,
        var_guard677: f64,
        var_guard682: f64,
        var_guard683: f64,
        var_guard684: f64,
        var_guard685: f64,
        var_guard686: f64,
        var_gvs_d: f64,
        var_gvs_d_dn0: f64,
        var_gvs_d_dn10: f64,
        var_gvs_d_dn11: f64,
        var_gvs_d_dn13: f64,
        var_gvs_d_dn14: f64,
        var_gvs_d_dn2: f64,
        var_gvs_d_dn3: f64,
        var_gvs_d_dn4: f64,
        var_gvs_d_dn5: f64,
        var_gvs_d_dn6: f64,
        var_gvs_d_dn7: f64,
        var_gvs_d_dn8: f64,
        var_gvs_d_dn9: f64,
        var_gvs_s: f64,
        var_gvs_s_dn0: f64,
        var_gvs_s_dn10: f64,
        var_gvs_s_dn11: f64,
        var_gvs_s_dn13: f64,
        var_gvs_s_dn14: f64,
        var_gvs_s_dn2: f64,
        var_gvs_s_dn3: f64,
        var_gvs_s_dn4: f64,
        var_gvs_s_dn5: f64,
        var_gvs_s_dn6: f64,
        var_gvs_s_dn7: f64,
        var_gvs_s_dn8: f64,
        var_gvs_s_dn9: f64,
        var_ids_v: f64,
        var_ids_v_dn0: f64,
        var_ids_v_dn10: f64,
        var_ids_v_dn11: f64,
        var_ids_v_dn13: f64,
        var_ids_v_dn14: f64,
        var_ids_v_dn2: f64,
        var_ids_v_dn3: f64,
        var_ids_v_dn4: f64,
        var_ids_v_dn5: f64,
        var_ids_v_dn6: f64,
        var_ids_v_dn7: f64,
        var_ids_v_dn8: f64,
        var_ids_v_dn9: f64,
        var_sigrat: f64,
        var_sigrat_dn0: f64,
        var_sigrat_dn10: f64,
        var_sigrat_dn11: f64,
        var_sigrat_dn13: f64,
        var_sigrat_dn14: f64,
        var_sigrat_dn2: f64,
        var_sigrat_dn3: f64,
        var_sigrat_dn4: f64,
        var_sigrat_dn5: f64,
        var_sigrat_dn6: f64,
        var_sigrat_dn7: f64,
        var_sigrat_dn8: f64,
        var_sigrat_dn9: f64,
        var_sigvds: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq97_e2727, eq97_e2727_d_n0, eq97_e2727_d_n2, eq97_e2727_d_n3, eq97_e2727_d_n4, eq97_e2727_d_n5, eq97_e2727_d_n6, eq97_e2727_d_n7, eq97_e2727_d_n8, eq97_e2727_d_n9, eq97_e2727_d_n10, eq97_e2727_d_n11, eq97_e2727_d_n13, eq97_e2727_d_n14, eq97_e2727_d_n16,) = {
    if (var_guard677 == 0.0) {
        let eq97_e2722: f64 = (0.7071 * var_sigrat);
        let eq97_e2722_d_n0: f64 = (0.7071 * var_sigrat_dn0);
        let eq97_e2722_d_n2: f64 = (0.7071 * var_sigrat_dn2);
        let eq97_e2722_d_n3: f64 = (0.7071 * var_sigrat_dn3);
        let eq97_e2722_d_n4: f64 = (0.7071 * var_sigrat_dn4);
        let eq97_e2722_d_n5: f64 = (0.7071 * var_sigrat_dn5);
        let eq97_e2722_d_n6: f64 = (0.7071 * var_sigrat_dn6);
        let eq97_e2722_d_n7: f64 = (0.7071 * var_sigrat_dn7);
        let eq97_e2722_d_n8: f64 = (0.7071 * var_sigrat_dn8);
        let eq97_e2722_d_n9: f64 = (0.7071 * var_sigrat_dn9);
        let eq97_e2722_d_n10: f64 = (0.7071 * var_sigrat_dn10);
        let eq97_e2722_d_n11: f64 = (0.7071 * var_sigrat_dn11);
        let eq97_e2722_d_n13: f64 = (0.7071 * var_sigrat_dn13);
        let eq97_e2722_d_n14: f64 = (0.7071 * var_sigrat_dn14);
        let eq97_e2724: f64 = (eq97_e2722 * (nv16 - 0.0));
        let eq97_e2724_d_n0: f64 = (eq97_e2722_d_n0 * (nv16 - 0.0));
        let eq97_e2724_d_n2: f64 = (eq97_e2722_d_n2 * (nv16 - 0.0));
        let eq97_e2724_d_n3: f64 = (eq97_e2722_d_n3 * (nv16 - 0.0));
        let eq97_e2724_d_n4: f64 = (eq97_e2722_d_n4 * (nv16 - 0.0));
        let eq97_e2724_d_n5: f64 = (eq97_e2722_d_n5 * (nv16 - 0.0));
        let eq97_e2724_d_n6: f64 = (eq97_e2722_d_n6 * (nv16 - 0.0));
        let eq97_e2724_d_n7: f64 = (eq97_e2722_d_n7 * (nv16 - 0.0));
        let eq97_e2724_d_n8: f64 = (eq97_e2722_d_n8 * (nv16 - 0.0));
        let eq97_e2724_d_n9: f64 = (eq97_e2722_d_n9 * (nv16 - 0.0));
        let eq97_e2724_d_n10: f64 = (eq97_e2722_d_n10 * (nv16 - 0.0));
        let eq97_e2724_d_n11: f64 = (eq97_e2722_d_n11 * (nv16 - 0.0));
        let eq97_e2724_d_n13: f64 = (eq97_e2722_d_n13 * (nv16 - 0.0));
        let eq97_e2724_d_n14: f64 = (eq97_e2722_d_n14 * (nv16 - 0.0));
        let eq97_e2725: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 26, eq97_e2724);
        (eq97_e2725, (eq97_e2724_d_n0 * ddt_scale), (eq97_e2724_d_n2 * ddt_scale), (eq97_e2724_d_n3 * ddt_scale), (eq97_e2724_d_n4 * ddt_scale), (eq97_e2724_d_n5 * ddt_scale), (eq97_e2724_d_n6 * ddt_scale), (eq97_e2724_d_n7 * ddt_scale), (eq97_e2724_d_n8 * ddt_scale), (eq97_e2724_d_n9 * ddt_scale), (eq97_e2724_d_n10 * ddt_scale), (eq97_e2724_d_n11 * ddt_scale), (eq97_e2724_d_n13 * ddt_scale), (eq97_e2724_d_n14 * ddt_scale), (eq97_e2722 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq97_value: f64 = eq97_e2727;
        let eq97_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 16];
        let eq97_node_derivatives: [f64; 14] = [eq97_e2727_d_n0, eq97_e2727_d_n2, eq97_e2727_d_n3, eq97_e2727_d_n4, eq97_e2727_d_n5, eq97_e2727_d_n6, eq97_e2727_d_n7, eq97_e2727_d_n8, eq97_e2727_d_n9, eq97_e2727_d_n10, eq97_e2727_d_n11, eq97_e2727_d_n13, eq97_e2727_d_n14, eq97_e2727_d_n16];
        let eq97_branch_derivative_indices: [usize; 0] = [];
        let eq97_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(5),
            multiplicity * (eq97_value),
            &eq97_node_derivative_indices,
            &eq97_node_derivatives,
            &eq97_branch_derivative_indices,
            &eq97_branch_derivatives,
            multiplicity,
        );
        let (eq105_e2843, eq105_e2843_d_n0, eq105_e2843_d_n2, eq105_e2843_d_n3, eq105_e2843_d_n4, eq105_e2843_d_n5, eq105_e2843_d_n6, eq105_e2843_d_n7, eq105_e2843_d_n8, eq105_e2843_d_n9, eq105_e2843_d_n10, eq105_e2843_d_n11, eq105_e2843_d_n13, eq105_e2843_d_n14,) = {
    if (var_guard682 != 0.0) {
        let eq105_e2836: f64 = (var_devsign * var_sigvds);
        let eq105_e2838: f64 = (eq105_e2836 * (nv5 - nv6));
        let eq105_e2840: f64 = (eq105_e2838 * var_ids_v);
        let eq105_e2840_d_n0: f64 = (eq105_e2838 * var_ids_v_dn0);
        let eq105_e2840_d_n2: f64 = (eq105_e2838 * var_ids_v_dn2);
        let eq105_e2840_d_n3: f64 = (eq105_e2838 * var_ids_v_dn3);
        let eq105_e2840_d_n4: f64 = (eq105_e2838 * var_ids_v_dn4);
        let eq105_e2840_d_n5: f64 = ((eq105_e2836 * var_ids_v) + (eq105_e2838 * var_ids_v_dn5));
        let eq105_e2840_d_n6: f64 = (((-eq105_e2836) * var_ids_v) + (eq105_e2838 * var_ids_v_dn6));
        let eq105_e2840_d_n7: f64 = (eq105_e2838 * var_ids_v_dn7);
        let eq105_e2840_d_n8: f64 = (eq105_e2838 * var_ids_v_dn8);
        let eq105_e2840_d_n9: f64 = (eq105_e2838 * var_ids_v_dn9);
        let eq105_e2840_d_n10: f64 = (eq105_e2838 * var_ids_v_dn10);
        let eq105_e2840_d_n11: f64 = (eq105_e2838 * var_ids_v_dn11);
        let eq105_e2840_d_n13: f64 = (eq105_e2838 * var_ids_v_dn13);
        let eq105_e2840_d_n14: f64 = (eq105_e2838 * var_ids_v_dn14);
        let eq105_e2841: f64 = (-eq105_e2840);
        (eq105_e2841, (-eq105_e2840_d_n0), (-eq105_e2840_d_n2), (-eq105_e2840_d_n3), (-eq105_e2840_d_n4), (-eq105_e2840_d_n5), (-eq105_e2840_d_n6), (-eq105_e2840_d_n7), (-eq105_e2840_d_n8), (-eq105_e2840_d_n9), (-eq105_e2840_d_n10), (-eq105_e2840_d_n11), (-eq105_e2840_d_n13), (-eq105_e2840_d_n14),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq105_value: f64 = eq105_e2843;
        let eq105_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq105_node_derivatives: [f64; 13] = [eq105_e2843_d_n0, eq105_e2843_d_n2, eq105_e2843_d_n3, eq105_e2843_d_n4, eq105_e2843_d_n5, eq105_e2843_d_n6, eq105_e2843_d_n7, eq105_e2843_d_n8, eq105_e2843_d_n9, eq105_e2843_d_n10, eq105_e2843_d_n11, eq105_e2843_d_n13, eq105_e2843_d_n14];
        let eq105_branch_derivative_indices: [usize; 0] = [];
        let eq105_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq105_value),
            &eq105_node_derivative_indices,
            &eq105_node_derivatives,
            &eq105_branch_derivative_indices,
            &eq105_branch_derivatives,
            multiplicity,
        );
        let (eq106_e2854, eq106_e2854_d_n0, eq106_e2854_d_n2, eq106_e2854_d_n3, eq106_e2854_d_n4, eq106_e2854_d_n5, eq106_e2854_d_n6, eq106_e2854_d_n7, eq106_e2854_d_n8, eq106_e2854_d_n9, eq106_e2854_d_n10, eq106_e2854_d_n11, eq106_e2854_d_n13, eq106_e2854_d_n14,) = {
    if ((var_guard682 != 0.0) && (var_guard683 != 0.0)) {
        let eq106_e2848: f64 = (-(nv0 - nv9));
        let eq106_e2850: f64 = (eq106_e2848 * (nv0 - nv9));
        let eq106_e2850_d_n0: f64 = (((-1.0) * (nv0 - nv9)) + eq106_e2848);
        let eq106_e2850_d_n9: f64 = ((nv0 - nv9) + (-eq106_e2848));
        let eq106_e2852: f64 = (eq106_e2850 * var_gdpr);
        let eq106_e2852_d_n0: f64 = ((eq106_e2850_d_n0 * var_gdpr) + (eq106_e2850 * var_gdpr_dn0));
        let eq106_e2852_d_n2: f64 = (eq106_e2850 * var_gdpr_dn2);
        let eq106_e2852_d_n3: f64 = (eq106_e2850 * var_gdpr_dn3);
        let eq106_e2852_d_n4: f64 = (eq106_e2850 * var_gdpr_dn4);
        let eq106_e2852_d_n5: f64 = (eq106_e2850 * var_gdpr_dn5);
        let eq106_e2852_d_n6: f64 = (eq106_e2850 * var_gdpr_dn6);
        let eq106_e2852_d_n7: f64 = (eq106_e2850 * var_gdpr_dn7);
        let eq106_e2852_d_n8: f64 = (eq106_e2850 * var_gdpr_dn8);
        let eq106_e2852_d_n9: f64 = ((eq106_e2850_d_n9 * var_gdpr) + (eq106_e2850 * var_gdpr_dn9));
        let eq106_e2852_d_n10: f64 = (eq106_e2850 * var_gdpr_dn10);
        let eq106_e2852_d_n11: f64 = (eq106_e2850 * var_gdpr_dn11);
        let eq106_e2852_d_n13: f64 = (eq106_e2850 * var_gdpr_dn13);
        let eq106_e2852_d_n14: f64 = (eq106_e2850 * var_gdpr_dn14);
        (eq106_e2852, eq106_e2852_d_n0, eq106_e2852_d_n2, eq106_e2852_d_n3, eq106_e2852_d_n4, eq106_e2852_d_n5, eq106_e2852_d_n6, eq106_e2852_d_n7, eq106_e2852_d_n8, eq106_e2852_d_n9, eq106_e2852_d_n10, eq106_e2852_d_n11, eq106_e2852_d_n13, eq106_e2852_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq106_value: f64 = eq106_e2854;
        let eq106_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq106_node_derivatives: [f64; 13] = [eq106_e2854_d_n0, eq106_e2854_d_n2, eq106_e2854_d_n3, eq106_e2854_d_n4, eq106_e2854_d_n5, eq106_e2854_d_n6, eq106_e2854_d_n7, eq106_e2854_d_n8, eq106_e2854_d_n9, eq106_e2854_d_n10, eq106_e2854_d_n11, eq106_e2854_d_n13, eq106_e2854_d_n14];
        let eq106_branch_derivative_indices: [usize; 0] = [];
        let eq106_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq106_value),
            &eq106_node_derivative_indices,
            &eq106_node_derivatives,
            &eq106_branch_derivative_indices,
            &eq106_branch_derivatives,
            multiplicity,
        );
        let (eq107_e2867, eq107_e2867_d_n0, eq107_e2867_d_n2, eq107_e2867_d_n3, eq107_e2867_d_n4, eq107_e2867_d_n5, eq107_e2867_d_n6, eq107_e2867_d_n7, eq107_e2867_d_n8, eq107_e2867_d_n9, eq107_e2867_d_n10, eq107_e2867_d_n11, eq107_e2867_d_n13, eq107_e2867_d_n14,) = {
    if (((var_guard682 != 0.0) && (var_guard683 != 0.0)) && (var_guard684 != 0.0)) {
        let eq107_e2861: f64 = (-(nv9 - nv7));
        let eq107_e2863: f64 = (eq107_e2861 * (nv9 - nv7));
        let eq107_e2863_d_n7: f64 = ((nv9 - nv7) + (-eq107_e2861));
        let eq107_e2863_d_n9: f64 = (((-1.0) * (nv9 - nv7)) + eq107_e2861);
        let eq107_e2865: f64 = (eq107_e2863 * var_gvs_d);
        let eq107_e2865_d_n0: f64 = (eq107_e2863 * var_gvs_d_dn0);
        let eq107_e2865_d_n2: f64 = (eq107_e2863 * var_gvs_d_dn2);
        let eq107_e2865_d_n3: f64 = (eq107_e2863 * var_gvs_d_dn3);
        let eq107_e2865_d_n4: f64 = (eq107_e2863 * var_gvs_d_dn4);
        let eq107_e2865_d_n5: f64 = (eq107_e2863 * var_gvs_d_dn5);
        let eq107_e2865_d_n6: f64 = (eq107_e2863 * var_gvs_d_dn6);
        let eq107_e2865_d_n7: f64 = ((eq107_e2863_d_n7 * var_gvs_d) + (eq107_e2863 * var_gvs_d_dn7));
        let eq107_e2865_d_n8: f64 = (eq107_e2863 * var_gvs_d_dn8);
        let eq107_e2865_d_n9: f64 = ((eq107_e2863_d_n9 * var_gvs_d) + (eq107_e2863 * var_gvs_d_dn9));
        let eq107_e2865_d_n10: f64 = (eq107_e2863 * var_gvs_d_dn10);
        let eq107_e2865_d_n11: f64 = (eq107_e2863 * var_gvs_d_dn11);
        let eq107_e2865_d_n13: f64 = (eq107_e2863 * var_gvs_d_dn13);
        let eq107_e2865_d_n14: f64 = (eq107_e2863 * var_gvs_d_dn14);
        (eq107_e2865, eq107_e2865_d_n0, eq107_e2865_d_n2, eq107_e2865_d_n3, eq107_e2865_d_n4, eq107_e2865_d_n5, eq107_e2865_d_n6, eq107_e2865_d_n7, eq107_e2865_d_n8, eq107_e2865_d_n9, eq107_e2865_d_n10, eq107_e2865_d_n11, eq107_e2865_d_n13, eq107_e2865_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq107_value: f64 = eq107_e2867;
        let eq107_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq107_node_derivatives: [f64; 13] = [eq107_e2867_d_n0, eq107_e2867_d_n2, eq107_e2867_d_n3, eq107_e2867_d_n4, eq107_e2867_d_n5, eq107_e2867_d_n6, eq107_e2867_d_n7, eq107_e2867_d_n8, eq107_e2867_d_n9, eq107_e2867_d_n10, eq107_e2867_d_n11, eq107_e2867_d_n13, eq107_e2867_d_n14];
        let eq107_branch_derivative_indices: [usize; 0] = [];
        let eq107_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq107_value),
            &eq107_node_derivative_indices,
            &eq107_node_derivatives,
            &eq107_branch_derivative_indices,
            &eq107_branch_derivatives,
            multiplicity,
        );
        let (eq108_e2878, eq108_e2878_d_n0, eq108_e2878_d_n2, eq108_e2878_d_n3, eq108_e2878_d_n4, eq108_e2878_d_n5, eq108_e2878_d_n6, eq108_e2878_d_n7, eq108_e2878_d_n8, eq108_e2878_d_n9, eq108_e2878_d_n10, eq108_e2878_d_n11, eq108_e2878_d_n13, eq108_e2878_d_n14,) = {
    if ((var_guard682 != 0.0) && (var_guard685 != 0.0)) {
        let eq108_e2872: f64 = (-(nv2 - nv8));
        let eq108_e2874: f64 = (eq108_e2872 * (nv2 - nv8));
        let eq108_e2874_d_n2: f64 = (((-1.0) * (nv2 - nv8)) + eq108_e2872);
        let eq108_e2874_d_n8: f64 = ((nv2 - nv8) + (-eq108_e2872));
        let eq108_e2876: f64 = (eq108_e2874 * var_gspr);
        let eq108_e2876_d_n0: f64 = (eq108_e2874 * var_gspr_dn0);
        let eq108_e2876_d_n2: f64 = ((eq108_e2874_d_n2 * var_gspr) + (eq108_e2874 * var_gspr_dn2));
        let eq108_e2876_d_n3: f64 = (eq108_e2874 * var_gspr_dn3);
        let eq108_e2876_d_n4: f64 = (eq108_e2874 * var_gspr_dn4);
        let eq108_e2876_d_n5: f64 = (eq108_e2874 * var_gspr_dn5);
        let eq108_e2876_d_n6: f64 = (eq108_e2874 * var_gspr_dn6);
        let eq108_e2876_d_n7: f64 = (eq108_e2874 * var_gspr_dn7);
        let eq108_e2876_d_n8: f64 = ((eq108_e2874_d_n8 * var_gspr) + (eq108_e2874 * var_gspr_dn8));
        let eq108_e2876_d_n9: f64 = (eq108_e2874 * var_gspr_dn9);
        let eq108_e2876_d_n10: f64 = (eq108_e2874 * var_gspr_dn10);
        let eq108_e2876_d_n11: f64 = (eq108_e2874 * var_gspr_dn11);
        let eq108_e2876_d_n13: f64 = (eq108_e2874 * var_gspr_dn13);
        let eq108_e2876_d_n14: f64 = (eq108_e2874 * var_gspr_dn14);
        (eq108_e2876, eq108_e2876_d_n0, eq108_e2876_d_n2, eq108_e2876_d_n3, eq108_e2876_d_n4, eq108_e2876_d_n5, eq108_e2876_d_n6, eq108_e2876_d_n7, eq108_e2876_d_n8, eq108_e2876_d_n9, eq108_e2876_d_n10, eq108_e2876_d_n11, eq108_e2876_d_n13, eq108_e2876_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq108_value: f64 = eq108_e2878;
        let eq108_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq108_node_derivatives: [f64; 13] = [eq108_e2878_d_n0, eq108_e2878_d_n2, eq108_e2878_d_n3, eq108_e2878_d_n4, eq108_e2878_d_n5, eq108_e2878_d_n6, eq108_e2878_d_n7, eq108_e2878_d_n8, eq108_e2878_d_n9, eq108_e2878_d_n10, eq108_e2878_d_n11, eq108_e2878_d_n13, eq108_e2878_d_n14];
        let eq108_branch_derivative_indices: [usize; 0] = [];
        let eq108_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq108_value),
            &eq108_node_derivative_indices,
            &eq108_node_derivatives,
            &eq108_branch_derivative_indices,
            &eq108_branch_derivatives,
            multiplicity,
        );
        let (eq109_e2891, eq109_e2891_d_n0, eq109_e2891_d_n2, eq109_e2891_d_n3, eq109_e2891_d_n4, eq109_e2891_d_n5, eq109_e2891_d_n6, eq109_e2891_d_n7, eq109_e2891_d_n8, eq109_e2891_d_n9, eq109_e2891_d_n10, eq109_e2891_d_n11, eq109_e2891_d_n13, eq109_e2891_d_n14,) = {
    if (((var_guard682 != 0.0) && (var_guard685 != 0.0)) && (var_guard686 != 0.0)) {
        let eq109_e2885: f64 = (-(nv8 - nv6));
        let eq109_e2887: f64 = (eq109_e2885 * (nv8 - nv6));
        let eq109_e2887_d_n6: f64 = ((nv8 - nv6) + (-eq109_e2885));
        let eq109_e2887_d_n8: f64 = (((-1.0) * (nv8 - nv6)) + eq109_e2885);
        let eq109_e2889: f64 = (eq109_e2887 * var_gvs_s);
        let eq109_e2889_d_n0: f64 = (eq109_e2887 * var_gvs_s_dn0);
        let eq109_e2889_d_n2: f64 = (eq109_e2887 * var_gvs_s_dn2);
        let eq109_e2889_d_n3: f64 = (eq109_e2887 * var_gvs_s_dn3);
        let eq109_e2889_d_n4: f64 = (eq109_e2887 * var_gvs_s_dn4);
        let eq109_e2889_d_n5: f64 = (eq109_e2887 * var_gvs_s_dn5);
        let eq109_e2889_d_n6: f64 = ((eq109_e2887_d_n6 * var_gvs_s) + (eq109_e2887 * var_gvs_s_dn6));
        let eq109_e2889_d_n7: f64 = (eq109_e2887 * var_gvs_s_dn7);
        let eq109_e2889_d_n8: f64 = ((eq109_e2887_d_n8 * var_gvs_s) + (eq109_e2887 * var_gvs_s_dn8));
        let eq109_e2889_d_n9: f64 = (eq109_e2887 * var_gvs_s_dn9);
        let eq109_e2889_d_n10: f64 = (eq109_e2887 * var_gvs_s_dn10);
        let eq109_e2889_d_n11: f64 = (eq109_e2887 * var_gvs_s_dn11);
        let eq109_e2889_d_n13: f64 = (eq109_e2887 * var_gvs_s_dn13);
        let eq109_e2889_d_n14: f64 = (eq109_e2887 * var_gvs_s_dn14);
        (eq109_e2889, eq109_e2889_d_n0, eq109_e2889_d_n2, eq109_e2889_d_n3, eq109_e2889_d_n4, eq109_e2889_d_n5, eq109_e2889_d_n6, eq109_e2889_d_n7, eq109_e2889_d_n8, eq109_e2889_d_n9, eq109_e2889_d_n10, eq109_e2889_d_n11, eq109_e2889_d_n13, eq109_e2889_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq109_value: f64 = eq109_e2891;
        let eq109_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq109_node_derivatives: [f64; 13] = [eq109_e2891_d_n0, eq109_e2891_d_n2, eq109_e2891_d_n3, eq109_e2891_d_n4, eq109_e2891_d_n5, eq109_e2891_d_n6, eq109_e2891_d_n7, eq109_e2891_d_n8, eq109_e2891_d_n9, eq109_e2891_d_n10, eq109_e2891_d_n11, eq109_e2891_d_n13, eq109_e2891_d_n14];
        let eq109_branch_derivative_indices: [usize; 0] = [];
        let eq109_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq109_value),
            &eq109_node_derivative_indices,
            &eq109_node_derivatives,
            &eq109_branch_derivative_indices,
            &eq109_branch_derivatives,
            multiplicity,
        );
        let (eq110_e2897, eq110_e2897_d_n0, eq110_e2897_d_n2, eq110_e2897_d_n3, eq110_e2897_d_n4, eq110_e2897_d_n5, eq110_e2897_d_n6, eq110_e2897_d_n7, eq110_e2897_d_n8, eq110_e2897_d_n9, eq110_e2897_d_n10, eq110_e2897_d_n11, eq110_e2897_d_n13, eq110_e2897_d_n14,) = {
    if (var_guard682 != 0.0) {
        let eq110_e2895: f64 = ((nv4 - 0.0) * var_gth);
        let eq110_e2895_d_n0: f64 = ((nv4 - 0.0) * var_gth_dn0);
        let eq110_e2895_d_n2: f64 = ((nv4 - 0.0) * var_gth_dn2);
        let eq110_e2895_d_n3: f64 = ((nv4 - 0.0) * var_gth_dn3);
        let eq110_e2895_d_n4: f64 = (var_gth + ((nv4 - 0.0) * var_gth_dn4));
        let eq110_e2895_d_n5: f64 = ((nv4 - 0.0) * var_gth_dn5);
        let eq110_e2895_d_n6: f64 = ((nv4 - 0.0) * var_gth_dn6);
        let eq110_e2895_d_n7: f64 = ((nv4 - 0.0) * var_gth_dn7);
        let eq110_e2895_d_n8: f64 = ((nv4 - 0.0) * var_gth_dn8);
        let eq110_e2895_d_n9: f64 = ((nv4 - 0.0) * var_gth_dn9);
        let eq110_e2895_d_n10: f64 = ((nv4 - 0.0) * var_gth_dn10);
        let eq110_e2895_d_n11: f64 = ((nv4 - 0.0) * var_gth_dn11);
        let eq110_e2895_d_n13: f64 = ((nv4 - 0.0) * var_gth_dn13);
        let eq110_e2895_d_n14: f64 = ((nv4 - 0.0) * var_gth_dn14);
        (eq110_e2895, eq110_e2895_d_n0, eq110_e2895_d_n2, eq110_e2895_d_n3, eq110_e2895_d_n4, eq110_e2895_d_n5, eq110_e2895_d_n6, eq110_e2895_d_n7, eq110_e2895_d_n8, eq110_e2895_d_n9, eq110_e2895_d_n10, eq110_e2895_d_n11, eq110_e2895_d_n13, eq110_e2895_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq110_value: f64 = eq110_e2897;
        let eq110_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq110_node_derivatives: [f64; 13] = [eq110_e2897_d_n0, eq110_e2897_d_n2, eq110_e2897_d_n3, eq110_e2897_d_n4, eq110_e2897_d_n5, eq110_e2897_d_n6, eq110_e2897_d_n7, eq110_e2897_d_n8, eq110_e2897_d_n9, eq110_e2897_d_n10, eq110_e2897_d_n11, eq110_e2897_d_n13, eq110_e2897_d_n14];
        let eq110_branch_derivative_indices: [usize; 0] = [];
        let eq110_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq110_value),
            &eq110_node_derivative_indices,
            &eq110_node_derivatives,
            &eq110_branch_derivative_indices,
            &eq110_branch_derivatives,
            multiplicity,
        );
        let (eq111_e2904, eq111_e2904_d_n0, eq111_e2904_d_n2, eq111_e2904_d_n3, eq111_e2904_d_n4, eq111_e2904_d_n5, eq111_e2904_d_n6, eq111_e2904_d_n7, eq111_e2904_d_n8, eq111_e2904_d_n9, eq111_e2904_d_n10, eq111_e2904_d_n11, eq111_e2904_d_n13, eq111_e2904_d_n14,) = {
    if (var_guard682 != 0.0) {
        let eq111_e2901: f64 = ((nv4 - 0.0) * var_cth);
        let eq111_e2901_d_n0: f64 = ((nv4 - 0.0) * var_cth_dn0);
        let eq111_e2901_d_n2: f64 = ((nv4 - 0.0) * var_cth_dn2);
        let eq111_e2901_d_n3: f64 = ((nv4 - 0.0) * var_cth_dn3);
        let eq111_e2901_d_n4: f64 = (var_cth + ((nv4 - 0.0) * var_cth_dn4));
        let eq111_e2901_d_n5: f64 = ((nv4 - 0.0) * var_cth_dn5);
        let eq111_e2901_d_n6: f64 = ((nv4 - 0.0) * var_cth_dn6);
        let eq111_e2901_d_n7: f64 = ((nv4 - 0.0) * var_cth_dn7);
        let eq111_e2901_d_n8: f64 = ((nv4 - 0.0) * var_cth_dn8);
        let eq111_e2901_d_n9: f64 = ((nv4 - 0.0) * var_cth_dn9);
        let eq111_e2901_d_n10: f64 = ((nv4 - 0.0) * var_cth_dn10);
        let eq111_e2901_d_n11: f64 = ((nv4 - 0.0) * var_cth_dn11);
        let eq111_e2901_d_n13: f64 = ((nv4 - 0.0) * var_cth_dn13);
        let eq111_e2901_d_n14: f64 = ((nv4 - 0.0) * var_cth_dn14);
        let eq111_e2902: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 27, eq111_e2901);
        (eq111_e2902, (eq111_e2901_d_n0 * ddt_scale), (eq111_e2901_d_n2 * ddt_scale), (eq111_e2901_d_n3 * ddt_scale), (eq111_e2901_d_n4 * ddt_scale), (eq111_e2901_d_n5 * ddt_scale), (eq111_e2901_d_n6 * ddt_scale), (eq111_e2901_d_n7 * ddt_scale), (eq111_e2901_d_n8 * ddt_scale), (eq111_e2901_d_n9 * ddt_scale), (eq111_e2901_d_n10 * ddt_scale), (eq111_e2901_d_n11 * ddt_scale), (eq111_e2901_d_n13 * ddt_scale), (eq111_e2901_d_n14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq111_value: f64 = eq111_e2904;
        let eq111_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq111_node_derivatives: [f64; 13] = [eq111_e2904_d_n0, eq111_e2904_d_n2, eq111_e2904_d_n3, eq111_e2904_d_n4, eq111_e2904_d_n5, eq111_e2904_d_n6, eq111_e2904_d_n7, eq111_e2904_d_n8, eq111_e2904_d_n9, eq111_e2904_d_n10, eq111_e2904_d_n11, eq111_e2904_d_n13, eq111_e2904_d_n14];
        let eq111_branch_derivative_indices: [usize; 0] = [];
        let eq111_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq111_value),
            &eq111_node_derivative_indices,
            &eq111_node_derivatives,
            &eq111_branch_derivative_indices,
            &eq111_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq4_e1979, eq4_e1979_d_n0, eq4_e1979_d_n1, eq4_e1979_d_n2, eq4_e1979_d_n3, eq4_e1979_d_n4, eq4_e1979_d_n5, eq4_e1979_d_n6, eq4_e1979_d_n7, eq4_e1979_d_n8, eq4_e1979_d_n9, eq4_e1979_d_n10, eq4_e1979_d_n11, eq4_e1979_d_n12, eq4_e1979_d_n13, eq4_e1979_d_n14, eq4_e1979_d_n15, eq4_e1979_d_n16, eq4_e1979_d_b0, eq4_e1979_d_b1, eq4_e1979_d_b2, eq4_e1979_d_b3, eq4_e1979_d_b4, eq4_e1979_d_b5, eq4_e1979_d_b6, eq4_e1979_d_b7, eq4_e1979_d_b8, eq4_e1979_d_b9, eq4_e1979_d_b10, eq4_e1979_d_b11, eq4_e1979_d_b12, eq4_e1979_d_b13, eq4_e1979_d_b14, eq4_e1979_d_b15, eq4_e1979_d_b16, eq4_e1979_d_b17, eq4_e1979_q, eq4_e1979_q_d_n0, eq4_e1979_q_d_n1, eq4_e1979_q_d_n2, eq4_e1979_q_d_n3, eq4_e1979_q_d_n4, eq4_e1979_q_d_n5, eq4_e1979_q_d_n6, eq4_e1979_q_d_n7, eq4_e1979_q_d_n8, eq4_e1979_q_d_n9, eq4_e1979_q_d_n10, eq4_e1979_q_d_n11, eq4_e1979_q_d_n12, eq4_e1979_q_d_n13, eq4_e1979_q_d_n14, eq4_e1979_q_d_n15, eq4_e1979_q_d_n16, eq4_e1979_q_d_b0, eq4_e1979_q_d_b1, eq4_e1979_q_d_b2, eq4_e1979_q_d_b3, eq4_e1979_q_d_b4, eq4_e1979_q_d_b5, eq4_e1979_q_d_b6, eq4_e1979_q_d_b7, eq4_e1979_q_d_b8, eq4_e1979_q_d_b9, eq4_e1979_q_d_b10, eq4_e1979_q_d_b11, eq4_e1979_q_d_b12, eq4_e1979_q_d_b13, eq4_e1979_q_d_b14, eq4_e1979_q_d_b15, eq4_e1979_q_d_b16, eq4_e1979_q_d_b17,) = {
    if (!s.b[1696]) {
        let eq4_e1976_q: f64 = s.v[137];
        let eq4_e1977: f64 = (s.v[114] * s.v[137]);
        let eq4_e1977_d_n0: f64 = ((s.dn[114][0] * s.v[137]) + (s.v[114] * s.dn[137][0]));
        let eq4_e1977_d_n1: f64 = ((s.dn[114][1] * s.v[137]) + (s.v[114] * s.dn[137][1]));
        let eq4_e1977_d_n2: f64 = ((s.dn[114][2] * s.v[137]) + (s.v[114] * s.dn[137][2]));
        let eq4_e1977_d_n3: f64 = ((s.dn[114][3] * s.v[137]) + (s.v[114] * s.dn[137][3]));
        let eq4_e1977_d_n4: f64 = ((s.dn[114][4] * s.v[137]) + (s.v[114] * s.dn[137][4]));
        let eq4_e1977_d_n5: f64 = ((s.dn[114][5] * s.v[137]) + (s.v[114] * s.dn[137][5]));
        let eq4_e1977_d_n6: f64 = ((s.dn[114][6] * s.v[137]) + (s.v[114] * s.dn[137][6]));
        let eq4_e1977_d_n7: f64 = ((s.dn[114][7] * s.v[137]) + (s.v[114] * s.dn[137][7]));
        let eq4_e1977_d_n8: f64 = ((s.dn[114][8] * s.v[137]) + (s.v[114] * s.dn[137][8]));
        let eq4_e1977_d_n9: f64 = ((s.dn[114][9] * s.v[137]) + (s.v[114] * s.dn[137][9]));
        let eq4_e1977_d_n10: f64 = ((s.dn[114][10] * s.v[137]) + (s.v[114] * s.dn[137][10]));
        let eq4_e1977_d_n11: f64 = ((s.dn[114][11] * s.v[137]) + (s.v[114] * s.dn[137][11]));
        let eq4_e1977_d_n12: f64 = ((s.dn[114][12] * s.v[137]) + (s.v[114] * s.dn[137][12]));
        let eq4_e1977_d_n13: f64 = ((s.dn[114][13] * s.v[137]) + (s.v[114] * s.dn[137][13]));
        let eq4_e1977_d_n14: f64 = ((s.dn[114][14] * s.v[137]) + (s.v[114] * s.dn[137][14]));
        let eq4_e1977_d_n15: f64 = ((s.dn[114][15] * s.v[137]) + (s.v[114] * s.dn[137][15]));
        let eq4_e1977_d_n16: f64 = ((s.dn[114][16] * s.v[137]) + (s.v[114] * s.dn[137][16]));
        let eq4_e1977_d_b0: f64 = ((s.db[114][0] * s.v[137]) + (s.v[114] * s.db[137][0]));
        let eq4_e1977_d_b1: f64 = ((s.db[114][1] * s.v[137]) + (s.v[114] * s.db[137][1]));
        let eq4_e1977_d_b2: f64 = ((s.db[114][2] * s.v[137]) + (s.v[114] * s.db[137][2]));
        let eq4_e1977_d_b3: f64 = ((s.db[114][3] * s.v[137]) + (s.v[114] * s.db[137][3]));
        let eq4_e1977_d_b4: f64 = ((s.db[114][4] * s.v[137]) + (s.v[114] * s.db[137][4]));
        let eq4_e1977_d_b5: f64 = ((s.db[114][5] * s.v[137]) + (s.v[114] * s.db[137][5]));
        let eq4_e1977_d_b6: f64 = ((s.db[114][6] * s.v[137]) + (s.v[114] * s.db[137][6]));
        let eq4_e1977_d_b7: f64 = ((s.db[114][7] * s.v[137]) + (s.v[114] * s.db[137][7]));
        let eq4_e1977_d_b8: f64 = ((s.db[114][8] * s.v[137]) + (s.v[114] * s.db[137][8]));
        let eq4_e1977_d_b9: f64 = ((s.db[114][9] * s.v[137]) + (s.v[114] * s.db[137][9]));
        let eq4_e1977_d_b10: f64 = ((s.db[114][10] * s.v[137]) + (s.v[114] * s.db[137][10]));
        let eq4_e1977_d_b11: f64 = ((s.db[114][11] * s.v[137]) + (s.v[114] * s.db[137][11]));
        let eq4_e1977_d_b12: f64 = ((s.db[114][12] * s.v[137]) + (s.v[114] * s.db[137][12]));
        let eq4_e1977_d_b13: f64 = ((s.db[114][13] * s.v[137]) + (s.v[114] * s.db[137][13]));
        let eq4_e1977_d_b14: f64 = ((s.db[114][14] * s.v[137]) + (s.v[114] * s.db[137][14]));
        let eq4_e1977_d_b15: f64 = ((s.db[114][15] * s.v[137]) + (s.v[114] * s.db[137][15]));
        let eq4_e1977_d_b16: f64 = ((s.db[114][16] * s.v[137]) + (s.v[114] * s.db[137][16]));
        let eq4_e1977_d_b17: f64 = ((s.db[114][17] * s.v[137]) + (s.v[114] * s.db[137][17]));
        let eq4_e1977_q: f64 = (s.v[114] * eq4_e1976_q);
        let eq4_e1977_q_d_n0: f64 = ((s.dn[114][0] * eq4_e1976_q) + (s.v[114] * s.dn[137][0]));
        let eq4_e1977_q_d_n1: f64 = ((s.dn[114][1] * eq4_e1976_q) + (s.v[114] * s.dn[137][1]));
        let eq4_e1977_q_d_n2: f64 = ((s.dn[114][2] * eq4_e1976_q) + (s.v[114] * s.dn[137][2]));
        let eq4_e1977_q_d_n3: f64 = ((s.dn[114][3] * eq4_e1976_q) + (s.v[114] * s.dn[137][3]));
        let eq4_e1977_q_d_n4: f64 = ((s.dn[114][4] * eq4_e1976_q) + (s.v[114] * s.dn[137][4]));
        let eq4_e1977_q_d_n5: f64 = ((s.dn[114][5] * eq4_e1976_q) + (s.v[114] * s.dn[137][5]));
        let eq4_e1977_q_d_n6: f64 = ((s.dn[114][6] * eq4_e1976_q) + (s.v[114] * s.dn[137][6]));
        let eq4_e1977_q_d_n7: f64 = ((s.dn[114][7] * eq4_e1976_q) + (s.v[114] * s.dn[137][7]));
        let eq4_e1977_q_d_n8: f64 = ((s.dn[114][8] * eq4_e1976_q) + (s.v[114] * s.dn[137][8]));
        let eq4_e1977_q_d_n9: f64 = ((s.dn[114][9] * eq4_e1976_q) + (s.v[114] * s.dn[137][9]));
        let eq4_e1977_q_d_n10: f64 = ((s.dn[114][10] * eq4_e1976_q) + (s.v[114] * s.dn[137][10]));
        let eq4_e1977_q_d_n11: f64 = ((s.dn[114][11] * eq4_e1976_q) + (s.v[114] * s.dn[137][11]));
        let eq4_e1977_q_d_n12: f64 = ((s.dn[114][12] * eq4_e1976_q) + (s.v[114] * s.dn[137][12]));
        let eq4_e1977_q_d_n13: f64 = ((s.dn[114][13] * eq4_e1976_q) + (s.v[114] * s.dn[137][13]));
        let eq4_e1977_q_d_n14: f64 = ((s.dn[114][14] * eq4_e1976_q) + (s.v[114] * s.dn[137][14]));
        let eq4_e1977_q_d_n15: f64 = ((s.dn[114][15] * eq4_e1976_q) + (s.v[114] * s.dn[137][15]));
        let eq4_e1977_q_d_n16: f64 = ((s.dn[114][16] * eq4_e1976_q) + (s.v[114] * s.dn[137][16]));
        let eq4_e1977_q_d_b0: f64 = ((s.db[114][0] * eq4_e1976_q) + (s.v[114] * s.db[137][0]));
        let eq4_e1977_q_d_b1: f64 = ((s.db[114][1] * eq4_e1976_q) + (s.v[114] * s.db[137][1]));
        let eq4_e1977_q_d_b2: f64 = ((s.db[114][2] * eq4_e1976_q) + (s.v[114] * s.db[137][2]));
        let eq4_e1977_q_d_b3: f64 = ((s.db[114][3] * eq4_e1976_q) + (s.v[114] * s.db[137][3]));
        let eq4_e1977_q_d_b4: f64 = ((s.db[114][4] * eq4_e1976_q) + (s.v[114] * s.db[137][4]));
        let eq4_e1977_q_d_b5: f64 = ((s.db[114][5] * eq4_e1976_q) + (s.v[114] * s.db[137][5]));
        let eq4_e1977_q_d_b6: f64 = ((s.db[114][6] * eq4_e1976_q) + (s.v[114] * s.db[137][6]));
        let eq4_e1977_q_d_b7: f64 = ((s.db[114][7] * eq4_e1976_q) + (s.v[114] * s.db[137][7]));
        let eq4_e1977_q_d_b8: f64 = ((s.db[114][8] * eq4_e1976_q) + (s.v[114] * s.db[137][8]));
        let eq4_e1977_q_d_b9: f64 = ((s.db[114][9] * eq4_e1976_q) + (s.v[114] * s.db[137][9]));
        let eq4_e1977_q_d_b10: f64 = ((s.db[114][10] * eq4_e1976_q) + (s.v[114] * s.db[137][10]));
        let eq4_e1977_q_d_b11: f64 = ((s.db[114][11] * eq4_e1976_q) + (s.v[114] * s.db[137][11]));
        let eq4_e1977_q_d_b12: f64 = ((s.db[114][12] * eq4_e1976_q) + (s.v[114] * s.db[137][12]));
        let eq4_e1977_q_d_b13: f64 = ((s.db[114][13] * eq4_e1976_q) + (s.v[114] * s.db[137][13]));
        let eq4_e1977_q_d_b14: f64 = ((s.db[114][14] * eq4_e1976_q) + (s.v[114] * s.db[137][14]));
        let eq4_e1977_q_d_b15: f64 = ((s.db[114][15] * eq4_e1976_q) + (s.v[114] * s.db[137][15]));
        let eq4_e1977_q_d_b16: f64 = ((s.db[114][16] * eq4_e1976_q) + (s.v[114] * s.db[137][16]));
        let eq4_e1977_q_d_b17: f64 = ((s.db[114][17] * eq4_e1976_q) + (s.v[114] * s.db[137][17]));
        (eq4_e1977, eq4_e1977_d_n0, eq4_e1977_d_n1, eq4_e1977_d_n2, eq4_e1977_d_n3, eq4_e1977_d_n4, eq4_e1977_d_n5, eq4_e1977_d_n6, eq4_e1977_d_n7, eq4_e1977_d_n8, eq4_e1977_d_n9, eq4_e1977_d_n10, eq4_e1977_d_n11, eq4_e1977_d_n12, eq4_e1977_d_n13, eq4_e1977_d_n14, eq4_e1977_d_n15, eq4_e1977_d_n16, eq4_e1977_d_b0, eq4_e1977_d_b1, eq4_e1977_d_b2, eq4_e1977_d_b3, eq4_e1977_d_b4, eq4_e1977_d_b5, eq4_e1977_d_b6, eq4_e1977_d_b7, eq4_e1977_d_b8, eq4_e1977_d_b9, eq4_e1977_d_b10, eq4_e1977_d_b11, eq4_e1977_d_b12, eq4_e1977_d_b13, eq4_e1977_d_b14, eq4_e1977_d_b15, eq4_e1977_d_b16, eq4_e1977_d_b17, eq4_e1977_q, eq4_e1977_q_d_n0, eq4_e1977_q_d_n1, eq4_e1977_q_d_n2, eq4_e1977_q_d_n3, eq4_e1977_q_d_n4, eq4_e1977_q_d_n5, eq4_e1977_q_d_n6, eq4_e1977_q_d_n7, eq4_e1977_q_d_n8, eq4_e1977_q_d_n9, eq4_e1977_q_d_n10, eq4_e1977_q_d_n11, eq4_e1977_q_d_n12, eq4_e1977_q_d_n13, eq4_e1977_q_d_n14, eq4_e1977_q_d_n15, eq4_e1977_q_d_n16, eq4_e1977_q_d_b0, eq4_e1977_q_d_b1, eq4_e1977_q_d_b2, eq4_e1977_q_d_b3, eq4_e1977_q_d_b4, eq4_e1977_q_d_b5, eq4_e1977_q_d_b6, eq4_e1977_q_d_b7, eq4_e1977_q_d_b8, eq4_e1977_q_d_b9, eq4_e1977_q_d_b10, eq4_e1977_q_d_b11, eq4_e1977_q_d_b12, eq4_e1977_q_d_b13, eq4_e1977_q_d_b14, eq4_e1977_q_d_b15, eq4_e1977_q_d_b16, eq4_e1977_q_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_reactive_node_derivatives: [f64; 17] = [eq4_e1979_q_d_n0, eq4_e1979_q_d_n1, eq4_e1979_q_d_n2, eq4_e1979_q_d_n3, eq4_e1979_q_d_n4, eq4_e1979_q_d_n5, eq4_e1979_q_d_n6, eq4_e1979_q_d_n7, eq4_e1979_q_d_n8, eq4_e1979_q_d_n9, eq4_e1979_q_d_n10, eq4_e1979_q_d_n11, eq4_e1979_q_d_n12, eq4_e1979_q_d_n13, eq4_e1979_q_d_n14, eq4_e1979_q_d_n15, eq4_e1979_q_d_n16];
        let eq4_reactive_branch_derivatives: [f64; 18] = [eq4_e1979_q_d_b0, eq4_e1979_q_d_b1, eq4_e1979_q_d_b2, eq4_e1979_q_d_b3, eq4_e1979_q_d_b4, eq4_e1979_q_d_b5, eq4_e1979_q_d_b6, eq4_e1979_q_d_b7, eq4_e1979_q_d_b8, eq4_e1979_q_d_b9, eq4_e1979_q_d_b10, eq4_e1979_q_d_b11, eq4_e1979_q_d_b12, eq4_e1979_q_d_b13, eq4_e1979_q_d_b14, eq4_e1979_q_d_b15, eq4_e1979_q_d_b16, eq4_e1979_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq4_reactive_node_derivatives,
            branches,
            &eq4_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq5_e1987, eq5_e1987_d_n0, eq5_e1987_d_n1, eq5_e1987_d_n2, eq5_e1987_d_n3, eq5_e1987_d_n4, eq5_e1987_d_n5, eq5_e1987_d_n6, eq5_e1987_d_n7, eq5_e1987_d_n8, eq5_e1987_d_n9, eq5_e1987_d_n10, eq5_e1987_d_n11, eq5_e1987_d_n12, eq5_e1987_d_n13, eq5_e1987_d_n14, eq5_e1987_d_n15, eq5_e1987_d_n16, eq5_e1987_d_b0, eq5_e1987_d_b1, eq5_e1987_d_b2, eq5_e1987_d_b3, eq5_e1987_d_b4, eq5_e1987_d_b5, eq5_e1987_d_b6, eq5_e1987_d_b7, eq5_e1987_d_b8, eq5_e1987_d_b9, eq5_e1987_d_b10, eq5_e1987_d_b11, eq5_e1987_d_b12, eq5_e1987_d_b13, eq5_e1987_d_b14, eq5_e1987_d_b15, eq5_e1987_d_b16, eq5_e1987_d_b17, eq5_e1987_q, eq5_e1987_q_d_n0, eq5_e1987_q_d_n1, eq5_e1987_q_d_n2, eq5_e1987_q_d_n3, eq5_e1987_q_d_n4, eq5_e1987_q_d_n5, eq5_e1987_q_d_n6, eq5_e1987_q_d_n7, eq5_e1987_q_d_n8, eq5_e1987_q_d_n9, eq5_e1987_q_d_n10, eq5_e1987_q_d_n11, eq5_e1987_q_d_n12, eq5_e1987_q_d_n13, eq5_e1987_q_d_n14, eq5_e1987_q_d_n15, eq5_e1987_q_d_n16, eq5_e1987_q_d_b0, eq5_e1987_q_d_b1, eq5_e1987_q_d_b2, eq5_e1987_q_d_b3, eq5_e1987_q_d_b4, eq5_e1987_q_d_b5, eq5_e1987_q_d_b6, eq5_e1987_q_d_b7, eq5_e1987_q_d_b8, eq5_e1987_q_d_b9, eq5_e1987_q_d_b10, eq5_e1987_q_d_b11, eq5_e1987_q_d_b12, eq5_e1987_q_d_b13, eq5_e1987_q_d_b14, eq5_e1987_q_d_b15, eq5_e1987_q_d_b16, eq5_e1987_q_d_b17,) = {
    if (!s.b[1696]) {
        let eq5_e1984_q: f64 = s.v[138];
        let eq5_e1985: f64 = (s.v[114] * s.v[138]);
        let eq5_e1985_d_n0: f64 = ((s.dn[114][0] * s.v[138]) + (s.v[114] * s.dn[138][0]));
        let eq5_e1985_d_n1: f64 = ((s.dn[114][1] * s.v[138]) + (s.v[114] * s.dn[138][1]));
        let eq5_e1985_d_n2: f64 = ((s.dn[114][2] * s.v[138]) + (s.v[114] * s.dn[138][2]));
        let eq5_e1985_d_n3: f64 = ((s.dn[114][3] * s.v[138]) + (s.v[114] * s.dn[138][3]));
        let eq5_e1985_d_n4: f64 = ((s.dn[114][4] * s.v[138]) + (s.v[114] * s.dn[138][4]));
        let eq5_e1985_d_n5: f64 = ((s.dn[114][5] * s.v[138]) + (s.v[114] * s.dn[138][5]));
        let eq5_e1985_d_n6: f64 = ((s.dn[114][6] * s.v[138]) + (s.v[114] * s.dn[138][6]));
        let eq5_e1985_d_n7: f64 = ((s.dn[114][7] * s.v[138]) + (s.v[114] * s.dn[138][7]));
        let eq5_e1985_d_n8: f64 = ((s.dn[114][8] * s.v[138]) + (s.v[114] * s.dn[138][8]));
        let eq5_e1985_d_n9: f64 = ((s.dn[114][9] * s.v[138]) + (s.v[114] * s.dn[138][9]));
        let eq5_e1985_d_n10: f64 = ((s.dn[114][10] * s.v[138]) + (s.v[114] * s.dn[138][10]));
        let eq5_e1985_d_n11: f64 = ((s.dn[114][11] * s.v[138]) + (s.v[114] * s.dn[138][11]));
        let eq5_e1985_d_n12: f64 = ((s.dn[114][12] * s.v[138]) + (s.v[114] * s.dn[138][12]));
        let eq5_e1985_d_n13: f64 = ((s.dn[114][13] * s.v[138]) + (s.v[114] * s.dn[138][13]));
        let eq5_e1985_d_n14: f64 = ((s.dn[114][14] * s.v[138]) + (s.v[114] * s.dn[138][14]));
        let eq5_e1985_d_n15: f64 = ((s.dn[114][15] * s.v[138]) + (s.v[114] * s.dn[138][15]));
        let eq5_e1985_d_n16: f64 = ((s.dn[114][16] * s.v[138]) + (s.v[114] * s.dn[138][16]));
        let eq5_e1985_d_b0: f64 = ((s.db[114][0] * s.v[138]) + (s.v[114] * s.db[138][0]));
        let eq5_e1985_d_b1: f64 = ((s.db[114][1] * s.v[138]) + (s.v[114] * s.db[138][1]));
        let eq5_e1985_d_b2: f64 = ((s.db[114][2] * s.v[138]) + (s.v[114] * s.db[138][2]));
        let eq5_e1985_d_b3: f64 = ((s.db[114][3] * s.v[138]) + (s.v[114] * s.db[138][3]));
        let eq5_e1985_d_b4: f64 = ((s.db[114][4] * s.v[138]) + (s.v[114] * s.db[138][4]));
        let eq5_e1985_d_b5: f64 = ((s.db[114][5] * s.v[138]) + (s.v[114] * s.db[138][5]));
        let eq5_e1985_d_b6: f64 = ((s.db[114][6] * s.v[138]) + (s.v[114] * s.db[138][6]));
        let eq5_e1985_d_b7: f64 = ((s.db[114][7] * s.v[138]) + (s.v[114] * s.db[138][7]));
        let eq5_e1985_d_b8: f64 = ((s.db[114][8] * s.v[138]) + (s.v[114] * s.db[138][8]));
        let eq5_e1985_d_b9: f64 = ((s.db[114][9] * s.v[138]) + (s.v[114] * s.db[138][9]));
        let eq5_e1985_d_b10: f64 = ((s.db[114][10] * s.v[138]) + (s.v[114] * s.db[138][10]));
        let eq5_e1985_d_b11: f64 = ((s.db[114][11] * s.v[138]) + (s.v[114] * s.db[138][11]));
        let eq5_e1985_d_b12: f64 = ((s.db[114][12] * s.v[138]) + (s.v[114] * s.db[138][12]));
        let eq5_e1985_d_b13: f64 = ((s.db[114][13] * s.v[138]) + (s.v[114] * s.db[138][13]));
        let eq5_e1985_d_b14: f64 = ((s.db[114][14] * s.v[138]) + (s.v[114] * s.db[138][14]));
        let eq5_e1985_d_b15: f64 = ((s.db[114][15] * s.v[138]) + (s.v[114] * s.db[138][15]));
        let eq5_e1985_d_b16: f64 = ((s.db[114][16] * s.v[138]) + (s.v[114] * s.db[138][16]));
        let eq5_e1985_d_b17: f64 = ((s.db[114][17] * s.v[138]) + (s.v[114] * s.db[138][17]));
        let eq5_e1985_q: f64 = (s.v[114] * eq5_e1984_q);
        let eq5_e1985_q_d_n0: f64 = ((s.dn[114][0] * eq5_e1984_q) + (s.v[114] * s.dn[138][0]));
        let eq5_e1985_q_d_n1: f64 = ((s.dn[114][1] * eq5_e1984_q) + (s.v[114] * s.dn[138][1]));
        let eq5_e1985_q_d_n2: f64 = ((s.dn[114][2] * eq5_e1984_q) + (s.v[114] * s.dn[138][2]));
        let eq5_e1985_q_d_n3: f64 = ((s.dn[114][3] * eq5_e1984_q) + (s.v[114] * s.dn[138][3]));
        let eq5_e1985_q_d_n4: f64 = ((s.dn[114][4] * eq5_e1984_q) + (s.v[114] * s.dn[138][4]));
        let eq5_e1985_q_d_n5: f64 = ((s.dn[114][5] * eq5_e1984_q) + (s.v[114] * s.dn[138][5]));
        let eq5_e1985_q_d_n6: f64 = ((s.dn[114][6] * eq5_e1984_q) + (s.v[114] * s.dn[138][6]));
        let eq5_e1985_q_d_n7: f64 = ((s.dn[114][7] * eq5_e1984_q) + (s.v[114] * s.dn[138][7]));
        let eq5_e1985_q_d_n8: f64 = ((s.dn[114][8] * eq5_e1984_q) + (s.v[114] * s.dn[138][8]));
        let eq5_e1985_q_d_n9: f64 = ((s.dn[114][9] * eq5_e1984_q) + (s.v[114] * s.dn[138][9]));
        let eq5_e1985_q_d_n10: f64 = ((s.dn[114][10] * eq5_e1984_q) + (s.v[114] * s.dn[138][10]));
        let eq5_e1985_q_d_n11: f64 = ((s.dn[114][11] * eq5_e1984_q) + (s.v[114] * s.dn[138][11]));
        let eq5_e1985_q_d_n12: f64 = ((s.dn[114][12] * eq5_e1984_q) + (s.v[114] * s.dn[138][12]));
        let eq5_e1985_q_d_n13: f64 = ((s.dn[114][13] * eq5_e1984_q) + (s.v[114] * s.dn[138][13]));
        let eq5_e1985_q_d_n14: f64 = ((s.dn[114][14] * eq5_e1984_q) + (s.v[114] * s.dn[138][14]));
        let eq5_e1985_q_d_n15: f64 = ((s.dn[114][15] * eq5_e1984_q) + (s.v[114] * s.dn[138][15]));
        let eq5_e1985_q_d_n16: f64 = ((s.dn[114][16] * eq5_e1984_q) + (s.v[114] * s.dn[138][16]));
        let eq5_e1985_q_d_b0: f64 = ((s.db[114][0] * eq5_e1984_q) + (s.v[114] * s.db[138][0]));
        let eq5_e1985_q_d_b1: f64 = ((s.db[114][1] * eq5_e1984_q) + (s.v[114] * s.db[138][1]));
        let eq5_e1985_q_d_b2: f64 = ((s.db[114][2] * eq5_e1984_q) + (s.v[114] * s.db[138][2]));
        let eq5_e1985_q_d_b3: f64 = ((s.db[114][3] * eq5_e1984_q) + (s.v[114] * s.db[138][3]));
        let eq5_e1985_q_d_b4: f64 = ((s.db[114][4] * eq5_e1984_q) + (s.v[114] * s.db[138][4]));
        let eq5_e1985_q_d_b5: f64 = ((s.db[114][5] * eq5_e1984_q) + (s.v[114] * s.db[138][5]));
        let eq5_e1985_q_d_b6: f64 = ((s.db[114][6] * eq5_e1984_q) + (s.v[114] * s.db[138][6]));
        let eq5_e1985_q_d_b7: f64 = ((s.db[114][7] * eq5_e1984_q) + (s.v[114] * s.db[138][7]));
        let eq5_e1985_q_d_b8: f64 = ((s.db[114][8] * eq5_e1984_q) + (s.v[114] * s.db[138][8]));
        let eq5_e1985_q_d_b9: f64 = ((s.db[114][9] * eq5_e1984_q) + (s.v[114] * s.db[138][9]));
        let eq5_e1985_q_d_b10: f64 = ((s.db[114][10] * eq5_e1984_q) + (s.v[114] * s.db[138][10]));
        let eq5_e1985_q_d_b11: f64 = ((s.db[114][11] * eq5_e1984_q) + (s.v[114] * s.db[138][11]));
        let eq5_e1985_q_d_b12: f64 = ((s.db[114][12] * eq5_e1984_q) + (s.v[114] * s.db[138][12]));
        let eq5_e1985_q_d_b13: f64 = ((s.db[114][13] * eq5_e1984_q) + (s.v[114] * s.db[138][13]));
        let eq5_e1985_q_d_b14: f64 = ((s.db[114][14] * eq5_e1984_q) + (s.v[114] * s.db[138][14]));
        let eq5_e1985_q_d_b15: f64 = ((s.db[114][15] * eq5_e1984_q) + (s.v[114] * s.db[138][15]));
        let eq5_e1985_q_d_b16: f64 = ((s.db[114][16] * eq5_e1984_q) + (s.v[114] * s.db[138][16]));
        let eq5_e1985_q_d_b17: f64 = ((s.db[114][17] * eq5_e1984_q) + (s.v[114] * s.db[138][17]));
        (eq5_e1985, eq5_e1985_d_n0, eq5_e1985_d_n1, eq5_e1985_d_n2, eq5_e1985_d_n3, eq5_e1985_d_n4, eq5_e1985_d_n5, eq5_e1985_d_n6, eq5_e1985_d_n7, eq5_e1985_d_n8, eq5_e1985_d_n9, eq5_e1985_d_n10, eq5_e1985_d_n11, eq5_e1985_d_n12, eq5_e1985_d_n13, eq5_e1985_d_n14, eq5_e1985_d_n15, eq5_e1985_d_n16, eq5_e1985_d_b0, eq5_e1985_d_b1, eq5_e1985_d_b2, eq5_e1985_d_b3, eq5_e1985_d_b4, eq5_e1985_d_b5, eq5_e1985_d_b6, eq5_e1985_d_b7, eq5_e1985_d_b8, eq5_e1985_d_b9, eq5_e1985_d_b10, eq5_e1985_d_b11, eq5_e1985_d_b12, eq5_e1985_d_b13, eq5_e1985_d_b14, eq5_e1985_d_b15, eq5_e1985_d_b16, eq5_e1985_d_b17, eq5_e1985_q, eq5_e1985_q_d_n0, eq5_e1985_q_d_n1, eq5_e1985_q_d_n2, eq5_e1985_q_d_n3, eq5_e1985_q_d_n4, eq5_e1985_q_d_n5, eq5_e1985_q_d_n6, eq5_e1985_q_d_n7, eq5_e1985_q_d_n8, eq5_e1985_q_d_n9, eq5_e1985_q_d_n10, eq5_e1985_q_d_n11, eq5_e1985_q_d_n12, eq5_e1985_q_d_n13, eq5_e1985_q_d_n14, eq5_e1985_q_d_n15, eq5_e1985_q_d_n16, eq5_e1985_q_d_b0, eq5_e1985_q_d_b1, eq5_e1985_q_d_b2, eq5_e1985_q_d_b3, eq5_e1985_q_d_b4, eq5_e1985_q_d_b5, eq5_e1985_q_d_b6, eq5_e1985_q_d_b7, eq5_e1985_q_d_b8, eq5_e1985_q_d_b9, eq5_e1985_q_d_b10, eq5_e1985_q_d_b11, eq5_e1985_q_d_b12, eq5_e1985_q_d_b13, eq5_e1985_q_d_b14, eq5_e1985_q_d_b15, eq5_e1985_q_d_b16, eq5_e1985_q_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_reactive_node_derivatives: [f64; 17] = [eq5_e1987_q_d_n0, eq5_e1987_q_d_n1, eq5_e1987_q_d_n2, eq5_e1987_q_d_n3, eq5_e1987_q_d_n4, eq5_e1987_q_d_n5, eq5_e1987_q_d_n6, eq5_e1987_q_d_n7, eq5_e1987_q_d_n8, eq5_e1987_q_d_n9, eq5_e1987_q_d_n10, eq5_e1987_q_d_n11, eq5_e1987_q_d_n12, eq5_e1987_q_d_n13, eq5_e1987_q_d_n14, eq5_e1987_q_d_n15, eq5_e1987_q_d_n16];
        let eq5_reactive_branch_derivatives: [f64; 18] = [eq5_e1987_q_d_b0, eq5_e1987_q_d_b1, eq5_e1987_q_d_b2, eq5_e1987_q_d_b3, eq5_e1987_q_d_b4, eq5_e1987_q_d_b5, eq5_e1987_q_d_b6, eq5_e1987_q_d_b7, eq5_e1987_q_d_b8, eq5_e1987_q_d_b9, eq5_e1987_q_d_b10, eq5_e1987_q_d_b11, eq5_e1987_q_d_b12, eq5_e1987_q_d_b13, eq5_e1987_q_d_b14, eq5_e1987_q_d_b15, eq5_e1987_q_d_b16, eq5_e1987_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            nodes,
            &eq5_reactive_node_derivatives,
            branches,
            &eq5_reactive_branch_derivatives,
            multiplicity,
        );
        let eq36_e2281_q: f64 = s.v[507];
        let eq36_e2282: f64 = (s.v[114] * s.v[507]);
        let eq36_e2282_d_n0: f64 = ((s.dn[114][0] * s.v[507]) + (s.v[114] * s.dn[507][0]));
        let eq36_e2282_d_n1: f64 = ((s.dn[114][1] * s.v[507]) + (s.v[114] * s.dn[507][1]));
        let eq36_e2282_d_n2: f64 = ((s.dn[114][2] * s.v[507]) + (s.v[114] * s.dn[507][2]));
        let eq36_e2282_d_n3: f64 = ((s.dn[114][3] * s.v[507]) + (s.v[114] * s.dn[507][3]));
        let eq36_e2282_d_n4: f64 = ((s.dn[114][4] * s.v[507]) + (s.v[114] * s.dn[507][4]));
        let eq36_e2282_d_n5: f64 = ((s.dn[114][5] * s.v[507]) + (s.v[114] * s.dn[507][5]));
        let eq36_e2282_d_n6: f64 = ((s.dn[114][6] * s.v[507]) + (s.v[114] * s.dn[507][6]));
        let eq36_e2282_d_n7: f64 = ((s.dn[114][7] * s.v[507]) + (s.v[114] * s.dn[507][7]));
        let eq36_e2282_d_n8: f64 = ((s.dn[114][8] * s.v[507]) + (s.v[114] * s.dn[507][8]));
        let eq36_e2282_d_n9: f64 = ((s.dn[114][9] * s.v[507]) + (s.v[114] * s.dn[507][9]));
        let eq36_e2282_d_n10: f64 = ((s.dn[114][10] * s.v[507]) + (s.v[114] * s.dn[507][10]));
        let eq36_e2282_d_n11: f64 = ((s.dn[114][11] * s.v[507]) + (s.v[114] * s.dn[507][11]));
        let eq36_e2282_d_n12: f64 = ((s.dn[114][12] * s.v[507]) + (s.v[114] * s.dn[507][12]));
        let eq36_e2282_d_n13: f64 = ((s.dn[114][13] * s.v[507]) + (s.v[114] * s.dn[507][13]));
        let eq36_e2282_d_n14: f64 = ((s.dn[114][14] * s.v[507]) + (s.v[114] * s.dn[507][14]));
        let eq36_e2282_d_n15: f64 = ((s.dn[114][15] * s.v[507]) + (s.v[114] * s.dn[507][15]));
        let eq36_e2282_d_n16: f64 = ((s.dn[114][16] * s.v[507]) + (s.v[114] * s.dn[507][16]));
        let eq36_e2282_d_b0: f64 = ((s.db[114][0] * s.v[507]) + (s.v[114] * s.db[507][0]));
        let eq36_e2282_d_b1: f64 = ((s.db[114][1] * s.v[507]) + (s.v[114] * s.db[507][1]));
        let eq36_e2282_d_b2: f64 = ((s.db[114][2] * s.v[507]) + (s.v[114] * s.db[507][2]));
        let eq36_e2282_d_b3: f64 = ((s.db[114][3] * s.v[507]) + (s.v[114] * s.db[507][3]));
        let eq36_e2282_d_b4: f64 = ((s.db[114][4] * s.v[507]) + (s.v[114] * s.db[507][4]));
        let eq36_e2282_d_b5: f64 = ((s.db[114][5] * s.v[507]) + (s.v[114] * s.db[507][5]));
        let eq36_e2282_d_b6: f64 = ((s.db[114][6] * s.v[507]) + (s.v[114] * s.db[507][6]));
        let eq36_e2282_d_b7: f64 = ((s.db[114][7] * s.v[507]) + (s.v[114] * s.db[507][7]));
        let eq36_e2282_d_b8: f64 = ((s.db[114][8] * s.v[507]) + (s.v[114] * s.db[507][8]));
        let eq36_e2282_d_b9: f64 = ((s.db[114][9] * s.v[507]) + (s.v[114] * s.db[507][9]));
        let eq36_e2282_d_b10: f64 = ((s.db[114][10] * s.v[507]) + (s.v[114] * s.db[507][10]));
        let eq36_e2282_d_b11: f64 = ((s.db[114][11] * s.v[507]) + (s.v[114] * s.db[507][11]));
        let eq36_e2282_d_b12: f64 = ((s.db[114][12] * s.v[507]) + (s.v[114] * s.db[507][12]));
        let eq36_e2282_d_b13: f64 = ((s.db[114][13] * s.v[507]) + (s.v[114] * s.db[507][13]));
        let eq36_e2282_d_b14: f64 = ((s.db[114][14] * s.v[507]) + (s.v[114] * s.db[507][14]));
        let eq36_e2282_d_b15: f64 = ((s.db[114][15] * s.v[507]) + (s.v[114] * s.db[507][15]));
        let eq36_e2282_d_b16: f64 = ((s.db[114][16] * s.v[507]) + (s.v[114] * s.db[507][16]));
        let eq36_e2282_d_b17: f64 = ((s.db[114][17] * s.v[507]) + (s.v[114] * s.db[507][17]));
        let eq36_e2282_q: f64 = (s.v[114] * eq36_e2281_q);
        let eq36_e2282_q_d_n0: f64 = ((s.dn[114][0] * eq36_e2281_q) + (s.v[114] * s.dn[507][0]));
        let eq36_e2282_q_d_n1: f64 = ((s.dn[114][1] * eq36_e2281_q) + (s.v[114] * s.dn[507][1]));
        let eq36_e2282_q_d_n2: f64 = ((s.dn[114][2] * eq36_e2281_q) + (s.v[114] * s.dn[507][2]));
        let eq36_e2282_q_d_n3: f64 = ((s.dn[114][3] * eq36_e2281_q) + (s.v[114] * s.dn[507][3]));
        let eq36_e2282_q_d_n4: f64 = ((s.dn[114][4] * eq36_e2281_q) + (s.v[114] * s.dn[507][4]));
        let eq36_e2282_q_d_n5: f64 = ((s.dn[114][5] * eq36_e2281_q) + (s.v[114] * s.dn[507][5]));
        let eq36_e2282_q_d_n6: f64 = ((s.dn[114][6] * eq36_e2281_q) + (s.v[114] * s.dn[507][6]));
        let eq36_e2282_q_d_n7: f64 = ((s.dn[114][7] * eq36_e2281_q) + (s.v[114] * s.dn[507][7]));
        let eq36_e2282_q_d_n8: f64 = ((s.dn[114][8] * eq36_e2281_q) + (s.v[114] * s.dn[507][8]));
        let eq36_e2282_q_d_n9: f64 = ((s.dn[114][9] * eq36_e2281_q) + (s.v[114] * s.dn[507][9]));
        let eq36_e2282_q_d_n10: f64 = ((s.dn[114][10] * eq36_e2281_q) + (s.v[114] * s.dn[507][10]));
        let eq36_e2282_q_d_n11: f64 = ((s.dn[114][11] * eq36_e2281_q) + (s.v[114] * s.dn[507][11]));
        let eq36_e2282_q_d_n12: f64 = ((s.dn[114][12] * eq36_e2281_q) + (s.v[114] * s.dn[507][12]));
        let eq36_e2282_q_d_n13: f64 = ((s.dn[114][13] * eq36_e2281_q) + (s.v[114] * s.dn[507][13]));
        let eq36_e2282_q_d_n14: f64 = ((s.dn[114][14] * eq36_e2281_q) + (s.v[114] * s.dn[507][14]));
        let eq36_e2282_q_d_n15: f64 = ((s.dn[114][15] * eq36_e2281_q) + (s.v[114] * s.dn[507][15]));
        let eq36_e2282_q_d_n16: f64 = ((s.dn[114][16] * eq36_e2281_q) + (s.v[114] * s.dn[507][16]));
        let eq36_e2282_q_d_b0: f64 = ((s.db[114][0] * eq36_e2281_q) + (s.v[114] * s.db[507][0]));
        let eq36_e2282_q_d_b1: f64 = ((s.db[114][1] * eq36_e2281_q) + (s.v[114] * s.db[507][1]));
        let eq36_e2282_q_d_b2: f64 = ((s.db[114][2] * eq36_e2281_q) + (s.v[114] * s.db[507][2]));
        let eq36_e2282_q_d_b3: f64 = ((s.db[114][3] * eq36_e2281_q) + (s.v[114] * s.db[507][3]));
        let eq36_e2282_q_d_b4: f64 = ((s.db[114][4] * eq36_e2281_q) + (s.v[114] * s.db[507][4]));
        let eq36_e2282_q_d_b5: f64 = ((s.db[114][5] * eq36_e2281_q) + (s.v[114] * s.db[507][5]));
        let eq36_e2282_q_d_b6: f64 = ((s.db[114][6] * eq36_e2281_q) + (s.v[114] * s.db[507][6]));
        let eq36_e2282_q_d_b7: f64 = ((s.db[114][7] * eq36_e2281_q) + (s.v[114] * s.db[507][7]));
        let eq36_e2282_q_d_b8: f64 = ((s.db[114][8] * eq36_e2281_q) + (s.v[114] * s.db[507][8]));
        let eq36_e2282_q_d_b9: f64 = ((s.db[114][9] * eq36_e2281_q) + (s.v[114] * s.db[507][9]));
        let eq36_e2282_q_d_b10: f64 = ((s.db[114][10] * eq36_e2281_q) + (s.v[114] * s.db[507][10]));
        let eq36_e2282_q_d_b11: f64 = ((s.db[114][11] * eq36_e2281_q) + (s.v[114] * s.db[507][11]));
        let eq36_e2282_q_d_b12: f64 = ((s.db[114][12] * eq36_e2281_q) + (s.v[114] * s.db[507][12]));
        let eq36_e2282_q_d_b13: f64 = ((s.db[114][13] * eq36_e2281_q) + (s.v[114] * s.db[507][13]));
        let eq36_e2282_q_d_b14: f64 = ((s.db[114][14] * eq36_e2281_q) + (s.v[114] * s.db[507][14]));
        let eq36_e2282_q_d_b15: f64 = ((s.db[114][15] * eq36_e2281_q) + (s.v[114] * s.db[507][15]));
        let eq36_e2282_q_d_b16: f64 = ((s.db[114][16] * eq36_e2281_q) + (s.v[114] * s.db[507][16]));
        let eq36_e2282_q_d_b17: f64 = ((s.db[114][17] * eq36_e2281_q) + (s.v[114] * s.db[507][17]));
        let eq36_reactive_node_derivatives: [f64; 17] = [eq36_e2282_q_d_n0, eq36_e2282_q_d_n1, eq36_e2282_q_d_n2, eq36_e2282_q_d_n3, eq36_e2282_q_d_n4, eq36_e2282_q_d_n5, eq36_e2282_q_d_n6, eq36_e2282_q_d_n7, eq36_e2282_q_d_n8, eq36_e2282_q_d_n9, eq36_e2282_q_d_n10, eq36_e2282_q_d_n11, eq36_e2282_q_d_n12, eq36_e2282_q_d_n13, eq36_e2282_q_d_n14, eq36_e2282_q_d_n15, eq36_e2282_q_d_n16];
        let eq36_reactive_branch_derivatives: [f64; 18] = [eq36_e2282_q_d_b0, eq36_e2282_q_d_b1, eq36_e2282_q_d_b2, eq36_e2282_q_d_b3, eq36_e2282_q_d_b4, eq36_e2282_q_d_b5, eq36_e2282_q_d_b6, eq36_e2282_q_d_b7, eq36_e2282_q_d_b8, eq36_e2282_q_d_b9, eq36_e2282_q_d_b10, eq36_e2282_q_d_b11, eq36_e2282_q_d_b12, eq36_e2282_q_d_b13, eq36_e2282_q_d_b14, eq36_e2282_q_d_b15, eq36_e2282_q_d_b16, eq36_e2282_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[6]),
            nodes,
            &eq36_reactive_node_derivatives,
            branches,
            &eq36_reactive_branch_derivatives,
            multiplicity,
        );
        let eq37_e2285_q: f64 = s.v[508];
        let eq37_e2286: f64 = (s.v[114] * s.v[508]);
        let eq37_e2286_d_n0: f64 = ((s.dn[114][0] * s.v[508]) + (s.v[114] * s.dn[508][0]));
        let eq37_e2286_d_n1: f64 = ((s.dn[114][1] * s.v[508]) + (s.v[114] * s.dn[508][1]));
        let eq37_e2286_d_n2: f64 = ((s.dn[114][2] * s.v[508]) + (s.v[114] * s.dn[508][2]));
        let eq37_e2286_d_n3: f64 = ((s.dn[114][3] * s.v[508]) + (s.v[114] * s.dn[508][3]));
        let eq37_e2286_d_n4: f64 = ((s.dn[114][4] * s.v[508]) + (s.v[114] * s.dn[508][4]));
        let eq37_e2286_d_n5: f64 = ((s.dn[114][5] * s.v[508]) + (s.v[114] * s.dn[508][5]));
        let eq37_e2286_d_n6: f64 = ((s.dn[114][6] * s.v[508]) + (s.v[114] * s.dn[508][6]));
        let eq37_e2286_d_n7: f64 = ((s.dn[114][7] * s.v[508]) + (s.v[114] * s.dn[508][7]));
        let eq37_e2286_d_n8: f64 = ((s.dn[114][8] * s.v[508]) + (s.v[114] * s.dn[508][8]));
        let eq37_e2286_d_n9: f64 = ((s.dn[114][9] * s.v[508]) + (s.v[114] * s.dn[508][9]));
        let eq37_e2286_d_n10: f64 = ((s.dn[114][10] * s.v[508]) + (s.v[114] * s.dn[508][10]));
        let eq37_e2286_d_n11: f64 = ((s.dn[114][11] * s.v[508]) + (s.v[114] * s.dn[508][11]));
        let eq37_e2286_d_n12: f64 = ((s.dn[114][12] * s.v[508]) + (s.v[114] * s.dn[508][12]));
        let eq37_e2286_d_n13: f64 = ((s.dn[114][13] * s.v[508]) + (s.v[114] * s.dn[508][13]));
        let eq37_e2286_d_n14: f64 = ((s.dn[114][14] * s.v[508]) + (s.v[114] * s.dn[508][14]));
        let eq37_e2286_d_n15: f64 = ((s.dn[114][15] * s.v[508]) + (s.v[114] * s.dn[508][15]));
        let eq37_e2286_d_n16: f64 = ((s.dn[114][16] * s.v[508]) + (s.v[114] * s.dn[508][16]));
        let eq37_e2286_d_b0: f64 = ((s.db[114][0] * s.v[508]) + (s.v[114] * s.db[508][0]));
        let eq37_e2286_d_b1: f64 = ((s.db[114][1] * s.v[508]) + (s.v[114] * s.db[508][1]));
        let eq37_e2286_d_b2: f64 = ((s.db[114][2] * s.v[508]) + (s.v[114] * s.db[508][2]));
        let eq37_e2286_d_b3: f64 = ((s.db[114][3] * s.v[508]) + (s.v[114] * s.db[508][3]));
        let eq37_e2286_d_b4: f64 = ((s.db[114][4] * s.v[508]) + (s.v[114] * s.db[508][4]));
        let eq37_e2286_d_b5: f64 = ((s.db[114][5] * s.v[508]) + (s.v[114] * s.db[508][5]));
        let eq37_e2286_d_b6: f64 = ((s.db[114][6] * s.v[508]) + (s.v[114] * s.db[508][6]));
        let eq37_e2286_d_b7: f64 = ((s.db[114][7] * s.v[508]) + (s.v[114] * s.db[508][7]));
        let eq37_e2286_d_b8: f64 = ((s.db[114][8] * s.v[508]) + (s.v[114] * s.db[508][8]));
        let eq37_e2286_d_b9: f64 = ((s.db[114][9] * s.v[508]) + (s.v[114] * s.db[508][9]));
        let eq37_e2286_d_b10: f64 = ((s.db[114][10] * s.v[508]) + (s.v[114] * s.db[508][10]));
        let eq37_e2286_d_b11: f64 = ((s.db[114][11] * s.v[508]) + (s.v[114] * s.db[508][11]));
        let eq37_e2286_d_b12: f64 = ((s.db[114][12] * s.v[508]) + (s.v[114] * s.db[508][12]));
        let eq37_e2286_d_b13: f64 = ((s.db[114][13] * s.v[508]) + (s.v[114] * s.db[508][13]));
        let eq37_e2286_d_b14: f64 = ((s.db[114][14] * s.v[508]) + (s.v[114] * s.db[508][14]));
        let eq37_e2286_d_b15: f64 = ((s.db[114][15] * s.v[508]) + (s.v[114] * s.db[508][15]));
        let eq37_e2286_d_b16: f64 = ((s.db[114][16] * s.v[508]) + (s.v[114] * s.db[508][16]));
        let eq37_e2286_d_b17: f64 = ((s.db[114][17] * s.v[508]) + (s.v[114] * s.db[508][17]));
        let eq37_e2286_q: f64 = (s.v[114] * eq37_e2285_q);
        let eq37_e2286_q_d_n0: f64 = ((s.dn[114][0] * eq37_e2285_q) + (s.v[114] * s.dn[508][0]));
        let eq37_e2286_q_d_n1: f64 = ((s.dn[114][1] * eq37_e2285_q) + (s.v[114] * s.dn[508][1]));
        let eq37_e2286_q_d_n2: f64 = ((s.dn[114][2] * eq37_e2285_q) + (s.v[114] * s.dn[508][2]));
        let eq37_e2286_q_d_n3: f64 = ((s.dn[114][3] * eq37_e2285_q) + (s.v[114] * s.dn[508][3]));
        let eq37_e2286_q_d_n4: f64 = ((s.dn[114][4] * eq37_e2285_q) + (s.v[114] * s.dn[508][4]));
        let eq37_e2286_q_d_n5: f64 = ((s.dn[114][5] * eq37_e2285_q) + (s.v[114] * s.dn[508][5]));
        let eq37_e2286_q_d_n6: f64 = ((s.dn[114][6] * eq37_e2285_q) + (s.v[114] * s.dn[508][6]));
        let eq37_e2286_q_d_n7: f64 = ((s.dn[114][7] * eq37_e2285_q) + (s.v[114] * s.dn[508][7]));
        let eq37_e2286_q_d_n8: f64 = ((s.dn[114][8] * eq37_e2285_q) + (s.v[114] * s.dn[508][8]));
        let eq37_e2286_q_d_n9: f64 = ((s.dn[114][9] * eq37_e2285_q) + (s.v[114] * s.dn[508][9]));
        let eq37_e2286_q_d_n10: f64 = ((s.dn[114][10] * eq37_e2285_q) + (s.v[114] * s.dn[508][10]));
        let eq37_e2286_q_d_n11: f64 = ((s.dn[114][11] * eq37_e2285_q) + (s.v[114] * s.dn[508][11]));
        let eq37_e2286_q_d_n12: f64 = ((s.dn[114][12] * eq37_e2285_q) + (s.v[114] * s.dn[508][12]));
        let eq37_e2286_q_d_n13: f64 = ((s.dn[114][13] * eq37_e2285_q) + (s.v[114] * s.dn[508][13]));
        let eq37_e2286_q_d_n14: f64 = ((s.dn[114][14] * eq37_e2285_q) + (s.v[114] * s.dn[508][14]));
        let eq37_e2286_q_d_n15: f64 = ((s.dn[114][15] * eq37_e2285_q) + (s.v[114] * s.dn[508][15]));
        let eq37_e2286_q_d_n16: f64 = ((s.dn[114][16] * eq37_e2285_q) + (s.v[114] * s.dn[508][16]));
        let eq37_e2286_q_d_b0: f64 = ((s.db[114][0] * eq37_e2285_q) + (s.v[114] * s.db[508][0]));
        let eq37_e2286_q_d_b1: f64 = ((s.db[114][1] * eq37_e2285_q) + (s.v[114] * s.db[508][1]));
        let eq37_e2286_q_d_b2: f64 = ((s.db[114][2] * eq37_e2285_q) + (s.v[114] * s.db[508][2]));
        let eq37_e2286_q_d_b3: f64 = ((s.db[114][3] * eq37_e2285_q) + (s.v[114] * s.db[508][3]));
        let eq37_e2286_q_d_b4: f64 = ((s.db[114][4] * eq37_e2285_q) + (s.v[114] * s.db[508][4]));
        let eq37_e2286_q_d_b5: f64 = ((s.db[114][5] * eq37_e2285_q) + (s.v[114] * s.db[508][5]));
        let eq37_e2286_q_d_b6: f64 = ((s.db[114][6] * eq37_e2285_q) + (s.v[114] * s.db[508][6]));
        let eq37_e2286_q_d_b7: f64 = ((s.db[114][7] * eq37_e2285_q) + (s.v[114] * s.db[508][7]));
        let eq37_e2286_q_d_b8: f64 = ((s.db[114][8] * eq37_e2285_q) + (s.v[114] * s.db[508][8]));
        let eq37_e2286_q_d_b9: f64 = ((s.db[114][9] * eq37_e2285_q) + (s.v[114] * s.db[508][9]));
        let eq37_e2286_q_d_b10: f64 = ((s.db[114][10] * eq37_e2285_q) + (s.v[114] * s.db[508][10]));
        let eq37_e2286_q_d_b11: f64 = ((s.db[114][11] * eq37_e2285_q) + (s.v[114] * s.db[508][11]));
        let eq37_e2286_q_d_b12: f64 = ((s.db[114][12] * eq37_e2285_q) + (s.v[114] * s.db[508][12]));
        let eq37_e2286_q_d_b13: f64 = ((s.db[114][13] * eq37_e2285_q) + (s.v[114] * s.db[508][13]));
        let eq37_e2286_q_d_b14: f64 = ((s.db[114][14] * eq37_e2285_q) + (s.v[114] * s.db[508][14]));
        let eq37_e2286_q_d_b15: f64 = ((s.db[114][15] * eq37_e2285_q) + (s.v[114] * s.db[508][15]));
        let eq37_e2286_q_d_b16: f64 = ((s.db[114][16] * eq37_e2285_q) + (s.v[114] * s.db[508][16]));
        let eq37_e2286_q_d_b17: f64 = ((s.db[114][17] * eq37_e2285_q) + (s.v[114] * s.db[508][17]));
        let eq37_reactive_node_derivatives: [f64; 17] = [eq37_e2286_q_d_n0, eq37_e2286_q_d_n1, eq37_e2286_q_d_n2, eq37_e2286_q_d_n3, eq37_e2286_q_d_n4, eq37_e2286_q_d_n5, eq37_e2286_q_d_n6, eq37_e2286_q_d_n7, eq37_e2286_q_d_n8, eq37_e2286_q_d_n9, eq37_e2286_q_d_n10, eq37_e2286_q_d_n11, eq37_e2286_q_d_n12, eq37_e2286_q_d_n13, eq37_e2286_q_d_n14, eq37_e2286_q_d_n15, eq37_e2286_q_d_n16];
        let eq37_reactive_branch_derivatives: [f64; 18] = [eq37_e2286_q_d_b0, eq37_e2286_q_d_b1, eq37_e2286_q_d_b2, eq37_e2286_q_d_b3, eq37_e2286_q_d_b4, eq37_e2286_q_d_b5, eq37_e2286_q_d_b6, eq37_e2286_q_d_b7, eq37_e2286_q_d_b8, eq37_e2286_q_d_b9, eq37_e2286_q_d_b10, eq37_e2286_q_d_b11, eq37_e2286_q_d_b12, eq37_e2286_q_d_b13, eq37_e2286_q_d_b14, eq37_e2286_q_d_b15, eq37_e2286_q_d_b16, eq37_e2286_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[5]),
            nodes,
            &eq37_reactive_node_derivatives,
            branches,
            &eq37_reactive_branch_derivatives,
            multiplicity,
        );
        let eq38_e2289_q: f64 = s.v[509];
        let eq38_e2290: f64 = (s.v[114] * s.v[509]);
        let eq38_e2290_d_n0: f64 = ((s.dn[114][0] * s.v[509]) + (s.v[114] * s.dn[509][0]));
        let eq38_e2290_d_n1: f64 = ((s.dn[114][1] * s.v[509]) + (s.v[114] * s.dn[509][1]));
        let eq38_e2290_d_n2: f64 = ((s.dn[114][2] * s.v[509]) + (s.v[114] * s.dn[509][2]));
        let eq38_e2290_d_n3: f64 = ((s.dn[114][3] * s.v[509]) + (s.v[114] * s.dn[509][3]));
        let eq38_e2290_d_n4: f64 = ((s.dn[114][4] * s.v[509]) + (s.v[114] * s.dn[509][4]));
        let eq38_e2290_d_n5: f64 = ((s.dn[114][5] * s.v[509]) + (s.v[114] * s.dn[509][5]));
        let eq38_e2290_d_n6: f64 = ((s.dn[114][6] * s.v[509]) + (s.v[114] * s.dn[509][6]));
        let eq38_e2290_d_n7: f64 = ((s.dn[114][7] * s.v[509]) + (s.v[114] * s.dn[509][7]));
        let eq38_e2290_d_n8: f64 = ((s.dn[114][8] * s.v[509]) + (s.v[114] * s.dn[509][8]));
        let eq38_e2290_d_n9: f64 = ((s.dn[114][9] * s.v[509]) + (s.v[114] * s.dn[509][9]));
        let eq38_e2290_d_n10: f64 = ((s.dn[114][10] * s.v[509]) + (s.v[114] * s.dn[509][10]));
        let eq38_e2290_d_n11: f64 = ((s.dn[114][11] * s.v[509]) + (s.v[114] * s.dn[509][11]));
        let eq38_e2290_d_n12: f64 = ((s.dn[114][12] * s.v[509]) + (s.v[114] * s.dn[509][12]));
        let eq38_e2290_d_n13: f64 = ((s.dn[114][13] * s.v[509]) + (s.v[114] * s.dn[509][13]));
        let eq38_e2290_d_n14: f64 = ((s.dn[114][14] * s.v[509]) + (s.v[114] * s.dn[509][14]));
        let eq38_e2290_d_n15: f64 = ((s.dn[114][15] * s.v[509]) + (s.v[114] * s.dn[509][15]));
        let eq38_e2290_d_n16: f64 = ((s.dn[114][16] * s.v[509]) + (s.v[114] * s.dn[509][16]));
        let eq38_e2290_d_b0: f64 = ((s.db[114][0] * s.v[509]) + (s.v[114] * s.db[509][0]));
        let eq38_e2290_d_b1: f64 = ((s.db[114][1] * s.v[509]) + (s.v[114] * s.db[509][1]));
        let eq38_e2290_d_b2: f64 = ((s.db[114][2] * s.v[509]) + (s.v[114] * s.db[509][2]));
        let eq38_e2290_d_b3: f64 = ((s.db[114][3] * s.v[509]) + (s.v[114] * s.db[509][3]));
        let eq38_e2290_d_b4: f64 = ((s.db[114][4] * s.v[509]) + (s.v[114] * s.db[509][4]));
        let eq38_e2290_d_b5: f64 = ((s.db[114][5] * s.v[509]) + (s.v[114] * s.db[509][5]));
        let eq38_e2290_d_b6: f64 = ((s.db[114][6] * s.v[509]) + (s.v[114] * s.db[509][6]));
        let eq38_e2290_d_b7: f64 = ((s.db[114][7] * s.v[509]) + (s.v[114] * s.db[509][7]));
        let eq38_e2290_d_b8: f64 = ((s.db[114][8] * s.v[509]) + (s.v[114] * s.db[509][8]));
        let eq38_e2290_d_b9: f64 = ((s.db[114][9] * s.v[509]) + (s.v[114] * s.db[509][9]));
        let eq38_e2290_d_b10: f64 = ((s.db[114][10] * s.v[509]) + (s.v[114] * s.db[509][10]));
        let eq38_e2290_d_b11: f64 = ((s.db[114][11] * s.v[509]) + (s.v[114] * s.db[509][11]));
        let eq38_e2290_d_b12: f64 = ((s.db[114][12] * s.v[509]) + (s.v[114] * s.db[509][12]));
        let eq38_e2290_d_b13: f64 = ((s.db[114][13] * s.v[509]) + (s.v[114] * s.db[509][13]));
        let eq38_e2290_d_b14: f64 = ((s.db[114][14] * s.v[509]) + (s.v[114] * s.db[509][14]));
        let eq38_e2290_d_b15: f64 = ((s.db[114][15] * s.v[509]) + (s.v[114] * s.db[509][15]));
        let eq38_e2290_d_b16: f64 = ((s.db[114][16] * s.v[509]) + (s.v[114] * s.db[509][16]));
        let eq38_e2290_d_b17: f64 = ((s.db[114][17] * s.v[509]) + (s.v[114] * s.db[509][17]));
        let eq38_e2290_q: f64 = (s.v[114] * eq38_e2289_q);
        let eq38_e2290_q_d_n0: f64 = ((s.dn[114][0] * eq38_e2289_q) + (s.v[114] * s.dn[509][0]));
        let eq38_e2290_q_d_n1: f64 = ((s.dn[114][1] * eq38_e2289_q) + (s.v[114] * s.dn[509][1]));
        let eq38_e2290_q_d_n2: f64 = ((s.dn[114][2] * eq38_e2289_q) + (s.v[114] * s.dn[509][2]));
        let eq38_e2290_q_d_n3: f64 = ((s.dn[114][3] * eq38_e2289_q) + (s.v[114] * s.dn[509][3]));
        let eq38_e2290_q_d_n4: f64 = ((s.dn[114][4] * eq38_e2289_q) + (s.v[114] * s.dn[509][4]));
        let eq38_e2290_q_d_n5: f64 = ((s.dn[114][5] * eq38_e2289_q) + (s.v[114] * s.dn[509][5]));
        let eq38_e2290_q_d_n6: f64 = ((s.dn[114][6] * eq38_e2289_q) + (s.v[114] * s.dn[509][6]));
        let eq38_e2290_q_d_n7: f64 = ((s.dn[114][7] * eq38_e2289_q) + (s.v[114] * s.dn[509][7]));
        let eq38_e2290_q_d_n8: f64 = ((s.dn[114][8] * eq38_e2289_q) + (s.v[114] * s.dn[509][8]));
        let eq38_e2290_q_d_n9: f64 = ((s.dn[114][9] * eq38_e2289_q) + (s.v[114] * s.dn[509][9]));
        let eq38_e2290_q_d_n10: f64 = ((s.dn[114][10] * eq38_e2289_q) + (s.v[114] * s.dn[509][10]));
        let eq38_e2290_q_d_n11: f64 = ((s.dn[114][11] * eq38_e2289_q) + (s.v[114] * s.dn[509][11]));
        let eq38_e2290_q_d_n12: f64 = ((s.dn[114][12] * eq38_e2289_q) + (s.v[114] * s.dn[509][12]));
        let eq38_e2290_q_d_n13: f64 = ((s.dn[114][13] * eq38_e2289_q) + (s.v[114] * s.dn[509][13]));
        let eq38_e2290_q_d_n14: f64 = ((s.dn[114][14] * eq38_e2289_q) + (s.v[114] * s.dn[509][14]));
        let eq38_e2290_q_d_n15: f64 = ((s.dn[114][15] * eq38_e2289_q) + (s.v[114] * s.dn[509][15]));
        let eq38_e2290_q_d_n16: f64 = ((s.dn[114][16] * eq38_e2289_q) + (s.v[114] * s.dn[509][16]));
        let eq38_e2290_q_d_b0: f64 = ((s.db[114][0] * eq38_e2289_q) + (s.v[114] * s.db[509][0]));
        let eq38_e2290_q_d_b1: f64 = ((s.db[114][1] * eq38_e2289_q) + (s.v[114] * s.db[509][1]));
        let eq38_e2290_q_d_b2: f64 = ((s.db[114][2] * eq38_e2289_q) + (s.v[114] * s.db[509][2]));
        let eq38_e2290_q_d_b3: f64 = ((s.db[114][3] * eq38_e2289_q) + (s.v[114] * s.db[509][3]));
        let eq38_e2290_q_d_b4: f64 = ((s.db[114][4] * eq38_e2289_q) + (s.v[114] * s.db[509][4]));
        let eq38_e2290_q_d_b5: f64 = ((s.db[114][5] * eq38_e2289_q) + (s.v[114] * s.db[509][5]));
        let eq38_e2290_q_d_b6: f64 = ((s.db[114][6] * eq38_e2289_q) + (s.v[114] * s.db[509][6]));
        let eq38_e2290_q_d_b7: f64 = ((s.db[114][7] * eq38_e2289_q) + (s.v[114] * s.db[509][7]));
        let eq38_e2290_q_d_b8: f64 = ((s.db[114][8] * eq38_e2289_q) + (s.v[114] * s.db[509][8]));
        let eq38_e2290_q_d_b9: f64 = ((s.db[114][9] * eq38_e2289_q) + (s.v[114] * s.db[509][9]));
        let eq38_e2290_q_d_b10: f64 = ((s.db[114][10] * eq38_e2289_q) + (s.v[114] * s.db[509][10]));
        let eq38_e2290_q_d_b11: f64 = ((s.db[114][11] * eq38_e2289_q) + (s.v[114] * s.db[509][11]));
        let eq38_e2290_q_d_b12: f64 = ((s.db[114][12] * eq38_e2289_q) + (s.v[114] * s.db[509][12]));
        let eq38_e2290_q_d_b13: f64 = ((s.db[114][13] * eq38_e2289_q) + (s.v[114] * s.db[509][13]));
        let eq38_e2290_q_d_b14: f64 = ((s.db[114][14] * eq38_e2289_q) + (s.v[114] * s.db[509][14]));
        let eq38_e2290_q_d_b15: f64 = ((s.db[114][15] * eq38_e2289_q) + (s.v[114] * s.db[509][15]));
        let eq38_e2290_q_d_b16: f64 = ((s.db[114][16] * eq38_e2289_q) + (s.v[114] * s.db[509][16]));
        let eq38_e2290_q_d_b17: f64 = ((s.db[114][17] * eq38_e2289_q) + (s.v[114] * s.db[509][17]));
        let eq38_reactive_node_derivatives: [f64; 17] = [eq38_e2290_q_d_n0, eq38_e2290_q_d_n1, eq38_e2290_q_d_n2, eq38_e2290_q_d_n3, eq38_e2290_q_d_n4, eq38_e2290_q_d_n5, eq38_e2290_q_d_n6, eq38_e2290_q_d_n7, eq38_e2290_q_d_n8, eq38_e2290_q_d_n9, eq38_e2290_q_d_n10, eq38_e2290_q_d_n11, eq38_e2290_q_d_n12, eq38_e2290_q_d_n13, eq38_e2290_q_d_n14, eq38_e2290_q_d_n15, eq38_e2290_q_d_n16];
        let eq38_reactive_branch_derivatives: [f64; 18] = [eq38_e2290_q_d_b0, eq38_e2290_q_d_b1, eq38_e2290_q_d_b2, eq38_e2290_q_d_b3, eq38_e2290_q_d_b4, eq38_e2290_q_d_b5, eq38_e2290_q_d_b6, eq38_e2290_q_d_b7, eq38_e2290_q_d_b8, eq38_e2290_q_d_b9, eq38_e2290_q_d_b10, eq38_e2290_q_d_b11, eq38_e2290_q_d_b12, eq38_e2290_q_d_b13, eq38_e2290_q_d_b14, eq38_e2290_q_d_b15, eq38_e2290_q_d_b16, eq38_e2290_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[10]),
            nodes,
            &eq38_reactive_node_derivatives,
            branches,
            &eq38_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq39_e2295, eq39_e2295_d_n0, eq39_e2295_d_n1, eq39_e2295_d_n2, eq39_e2295_d_n3, eq39_e2295_d_n4, eq39_e2295_d_n5, eq39_e2295_d_n6, eq39_e2295_d_n7, eq39_e2295_d_n8, eq39_e2295_d_n9, eq39_e2295_d_n10, eq39_e2295_d_n11, eq39_e2295_d_n12, eq39_e2295_d_n13, eq39_e2295_d_n14, eq39_e2295_d_n15, eq39_e2295_d_n16, eq39_e2295_d_b0, eq39_e2295_d_b1, eq39_e2295_d_b2, eq39_e2295_d_b3, eq39_e2295_d_b4, eq39_e2295_d_b5, eq39_e2295_d_b6, eq39_e2295_d_b7, eq39_e2295_d_b8, eq39_e2295_d_b9, eq39_e2295_d_b10, eq39_e2295_d_b11, eq39_e2295_d_b12, eq39_e2295_d_b13, eq39_e2295_d_b14, eq39_e2295_d_b15, eq39_e2295_d_b16, eq39_e2295_d_b17, eq39_e2295_q,) = {
    if s.b[1705] {
        let eq39_e2293_q: f64 = s.v[505];
        (s.v[505], s.dn[505][0], s.dn[505][1], s.dn[505][2], s.dn[505][3], s.dn[505][4], s.dn[505][5], s.dn[505][6], s.dn[505][7], s.dn[505][8], s.dn[505][9], s.dn[505][10], s.dn[505][11], s.dn[505][12], s.dn[505][13], s.dn[505][14], s.dn[505][15], s.dn[505][16], s.db[505][0], s.db[505][1], s.db[505][2], s.db[505][3], s.db[505][4], s.db[505][5], s.db[505][6], s.db[505][7], s.db[505][8], s.db[505][9], s.db[505][10], s.db[505][11], s.db[505][12], s.db[505][13], s.db[505][14], s.db[505][15], s.db[505][16], s.db[505][17], eq39_e2293_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_reactive_node_derivatives: [f64; 17] = [eq39_e2295_d_n0, eq39_e2295_d_n1, eq39_e2295_d_n2, eq39_e2295_d_n3, eq39_e2295_d_n4, eq39_e2295_d_n5, eq39_e2295_d_n6, eq39_e2295_d_n7, eq39_e2295_d_n8, eq39_e2295_d_n9, eq39_e2295_d_n10, eq39_e2295_d_n11, eq39_e2295_d_n12, eq39_e2295_d_n13, eq39_e2295_d_n14, eq39_e2295_d_n15, eq39_e2295_d_n16];
        let eq39_reactive_branch_derivatives: [f64; 18] = [eq39_e2295_d_b0, eq39_e2295_d_b1, eq39_e2295_d_b2, eq39_e2295_d_b3, eq39_e2295_d_b4, eq39_e2295_d_b5, eq39_e2295_d_b6, eq39_e2295_d_b7, eq39_e2295_d_b8, eq39_e2295_d_b9, eq39_e2295_d_b10, eq39_e2295_d_b11, eq39_e2295_d_b12, eq39_e2295_d_b13, eq39_e2295_d_b14, eq39_e2295_d_b15, eq39_e2295_d_b16, eq39_e2295_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[6]),
            nodes,
            &eq39_reactive_node_derivatives,
            branches,
            &eq39_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq40_e2302, eq40_e2302_d_n0, eq40_e2302_d_n1, eq40_e2302_d_n2, eq40_e2302_d_n3, eq40_e2302_d_n4, eq40_e2302_d_n5, eq40_e2302_d_n6, eq40_e2302_d_n7, eq40_e2302_d_n8, eq40_e2302_d_n9, eq40_e2302_d_n10, eq40_e2302_d_n11, eq40_e2302_d_n12, eq40_e2302_d_n13, eq40_e2302_d_n14, eq40_e2302_d_n15, eq40_e2302_d_n16, eq40_e2302_d_b0, eq40_e2302_d_b1, eq40_e2302_d_b2, eq40_e2302_d_b3, eq40_e2302_d_b4, eq40_e2302_d_b5, eq40_e2302_d_b6, eq40_e2302_d_b7, eq40_e2302_d_b8, eq40_e2302_d_b9, eq40_e2302_d_b10, eq40_e2302_d_b11, eq40_e2302_d_b12, eq40_e2302_d_b13, eq40_e2302_d_b14, eq40_e2302_d_b15, eq40_e2302_d_b16, eq40_e2302_d_b17, eq40_e2302_q,) = {
    if (s.b[1705] && s.b[1706]) {
        let eq40_e2300_q: f64 = s.v[506];
        (s.v[506], s.dn[506][0], s.dn[506][1], s.dn[506][2], s.dn[506][3], s.dn[506][4], s.dn[506][5], s.dn[506][6], s.dn[506][7], s.dn[506][8], s.dn[506][9], s.dn[506][10], s.dn[506][11], s.dn[506][12], s.dn[506][13], s.dn[506][14], s.dn[506][15], s.dn[506][16], s.db[506][0], s.db[506][1], s.db[506][2], s.db[506][3], s.db[506][4], s.db[506][5], s.db[506][6], s.db[506][7], s.db[506][8], s.db[506][9], s.db[506][10], s.db[506][11], s.db[506][12], s.db[506][13], s.db[506][14], s.db[506][15], s.db[506][16], s.db[506][17], eq40_e2300_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_reactive_node_derivatives: [f64; 17] = [eq40_e2302_d_n0, eq40_e2302_d_n1, eq40_e2302_d_n2, eq40_e2302_d_n3, eq40_e2302_d_n4, eq40_e2302_d_n5, eq40_e2302_d_n6, eq40_e2302_d_n7, eq40_e2302_d_n8, eq40_e2302_d_n9, eq40_e2302_d_n10, eq40_e2302_d_n11, eq40_e2302_d_n12, eq40_e2302_d_n13, eq40_e2302_d_n14, eq40_e2302_d_n15, eq40_e2302_d_n16];
        let eq40_reactive_branch_derivatives: [f64; 18] = [eq40_e2302_d_b0, eq40_e2302_d_b1, eq40_e2302_d_b2, eq40_e2302_d_b3, eq40_e2302_d_b4, eq40_e2302_d_b5, eq40_e2302_d_b6, eq40_e2302_d_b7, eq40_e2302_d_b8, eq40_e2302_d_b9, eq40_e2302_d_b10, eq40_e2302_d_b11, eq40_e2302_d_b12, eq40_e2302_d_b13, eq40_e2302_d_b14, eq40_e2302_d_b15, eq40_e2302_d_b16, eq40_e2302_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            nodes,
            &eq40_reactive_node_derivatives,
            branches,
            &eq40_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let __rspice_deriv_cse_0: f64 = ((s.dn[114][0] * s.v[503]) + (s.v[114] * s.dn[503][0]));
        let __rspice_deriv_cse_1: f64 = ((s.dn[114][1] * s.v[503]) + (s.v[114] * s.dn[503][1]));
        let __rspice_deriv_cse_2: f64 = ((s.dn[114][2] * s.v[503]) + (s.v[114] * s.dn[503][2]));
        let __rspice_deriv_cse_3: f64 = ((s.dn[114][3] * s.v[503]) + (s.v[114] * s.dn[503][3]));
        let __rspice_deriv_cse_4: f64 = ((s.dn[114][4] * s.v[503]) + (s.v[114] * s.dn[503][4]));
        let __rspice_deriv_cse_5: f64 = ((s.dn[114][5] * s.v[503]) + (s.v[114] * s.dn[503][5]));
        let __rspice_deriv_cse_6: f64 = ((s.dn[114][6] * s.v[503]) + (s.v[114] * s.dn[503][6]));
        let __rspice_deriv_cse_7: f64 = ((s.dn[114][7] * s.v[503]) + (s.v[114] * s.dn[503][7]));
        let __rspice_deriv_cse_8: f64 = ((s.dn[114][8] * s.v[503]) + (s.v[114] * s.dn[503][8]));
        let __rspice_deriv_cse_9: f64 = ((s.dn[114][9] * s.v[503]) + (s.v[114] * s.dn[503][9]));
        let __rspice_deriv_cse_10: f64 = ((s.dn[114][10] * s.v[503]) + (s.v[114] * s.dn[503][10]));
        let __rspice_deriv_cse_11: f64 = ((s.dn[114][11] * s.v[503]) + (s.v[114] * s.dn[503][11]));
        let __rspice_deriv_cse_12: f64 = ((s.dn[114][12] * s.v[503]) + (s.v[114] * s.dn[503][12]));
        let __rspice_deriv_cse_13: f64 = ((s.dn[114][13] * s.v[503]) + (s.v[114] * s.dn[503][13]));
        let __rspice_deriv_cse_14: f64 = ((s.dn[114][14] * s.v[503]) + (s.v[114] * s.dn[503][14]));
        let __rspice_deriv_cse_15: f64 = ((s.dn[114][15] * s.v[503]) + (s.v[114] * s.dn[503][15]));
        let __rspice_deriv_cse_16: f64 = ((s.dn[114][16] * s.v[503]) + (s.v[114] * s.dn[503][16]));
        let __rspice_deriv_cse_17: f64 = ((s.db[114][0] * s.v[503]) + (s.v[114] * s.db[503][0]));
        let __rspice_deriv_cse_18: f64 = ((s.db[114][1] * s.v[503]) + (s.v[114] * s.db[503][1]));
        let __rspice_deriv_cse_19: f64 = ((s.db[114][2] * s.v[503]) + (s.v[114] * s.db[503][2]));
        let __rspice_deriv_cse_20: f64 = ((s.db[114][3] * s.v[503]) + (s.v[114] * s.db[503][3]));
        let __rspice_deriv_cse_21: f64 = ((s.db[114][4] * s.v[503]) + (s.v[114] * s.db[503][4]));
        let __rspice_deriv_cse_22: f64 = ((s.db[114][5] * s.v[503]) + (s.v[114] * s.db[503][5]));
        let __rspice_deriv_cse_23: f64 = ((s.db[114][6] * s.v[503]) + (s.v[114] * s.db[503][6]));
        let __rspice_deriv_cse_24: f64 = ((s.db[114][7] * s.v[503]) + (s.v[114] * s.db[503][7]));
        let __rspice_deriv_cse_25: f64 = ((s.db[114][8] * s.v[503]) + (s.v[114] * s.db[503][8]));
        let __rspice_deriv_cse_26: f64 = ((s.db[114][9] * s.v[503]) + (s.v[114] * s.db[503][9]));
        let __rspice_deriv_cse_27: f64 = ((s.db[114][10] * s.v[503]) + (s.v[114] * s.db[503][10]));
        let __rspice_deriv_cse_28: f64 = ((s.db[114][11] * s.v[503]) + (s.v[114] * s.db[503][11]));
        let __rspice_deriv_cse_29: f64 = ((s.db[114][12] * s.v[503]) + (s.v[114] * s.db[503][12]));
        let __rspice_deriv_cse_30: f64 = ((s.db[114][13] * s.v[503]) + (s.v[114] * s.db[503][13]));
        let __rspice_deriv_cse_31: f64 = ((s.db[114][14] * s.v[503]) + (s.v[114] * s.db[503][14]));
        let __rspice_deriv_cse_32: f64 = ((s.db[114][15] * s.v[503]) + (s.v[114] * s.db[503][15]));
        let __rspice_deriv_cse_33: f64 = ((s.db[114][16] * s.v[503]) + (s.v[114] * s.db[503][16]));
        let __rspice_deriv_cse_34: f64 = ((s.db[114][17] * s.v[503]) + (s.v[114] * s.db[503][17]));
        let __rspice_deriv_cse_35: f64 = ((s.dn[114][0] * s.v[504]) + (s.v[114] * s.dn[504][0]));
        let __rspice_deriv_cse_36: f64 = ((s.dn[114][1] * s.v[504]) + (s.v[114] * s.dn[504][1]));
        let __rspice_deriv_cse_37: f64 = ((s.dn[114][2] * s.v[504]) + (s.v[114] * s.dn[504][2]));
        let __rspice_deriv_cse_38: f64 = ((s.dn[114][3] * s.v[504]) + (s.v[114] * s.dn[504][3]));
        let __rspice_deriv_cse_39: f64 = ((s.dn[114][4] * s.v[504]) + (s.v[114] * s.dn[504][4]));
        let __rspice_deriv_cse_40: f64 = ((s.dn[114][5] * s.v[504]) + (s.v[114] * s.dn[504][5]));
        let __rspice_deriv_cse_41: f64 = ((s.dn[114][6] * s.v[504]) + (s.v[114] * s.dn[504][6]));
        let __rspice_deriv_cse_42: f64 = ((s.dn[114][7] * s.v[504]) + (s.v[114] * s.dn[504][7]));
        let __rspice_deriv_cse_43: f64 = ((s.dn[114][8] * s.v[504]) + (s.v[114] * s.dn[504][8]));
        let __rspice_deriv_cse_44: f64 = ((s.dn[114][9] * s.v[504]) + (s.v[114] * s.dn[504][9]));
        let __rspice_deriv_cse_45: f64 = ((s.dn[114][10] * s.v[504]) + (s.v[114] * s.dn[504][10]));
        let __rspice_deriv_cse_46: f64 = ((s.dn[114][11] * s.v[504]) + (s.v[114] * s.dn[504][11]));
        let __rspice_deriv_cse_47: f64 = ((s.dn[114][12] * s.v[504]) + (s.v[114] * s.dn[504][12]));
        let __rspice_deriv_cse_48: f64 = ((s.dn[114][13] * s.v[504]) + (s.v[114] * s.dn[504][13]));
        let __rspice_deriv_cse_49: f64 = ((s.dn[114][14] * s.v[504]) + (s.v[114] * s.dn[504][14]));
        let __rspice_deriv_cse_50: f64 = ((s.dn[114][15] * s.v[504]) + (s.v[114] * s.dn[504][15]));
        let __rspice_deriv_cse_51: f64 = ((s.dn[114][16] * s.v[504]) + (s.v[114] * s.dn[504][16]));
        let __rspice_deriv_cse_52: f64 = ((s.db[114][0] * s.v[504]) + (s.v[114] * s.db[504][0]));
        let __rspice_deriv_cse_53: f64 = ((s.db[114][1] * s.v[504]) + (s.v[114] * s.db[504][1]));
        let __rspice_deriv_cse_54: f64 = ((s.db[114][2] * s.v[504]) + (s.v[114] * s.db[504][2]));
        let __rspice_deriv_cse_55: f64 = ((s.db[114][3] * s.v[504]) + (s.v[114] * s.db[504][3]));
        let __rspice_deriv_cse_56: f64 = ((s.db[114][4] * s.v[504]) + (s.v[114] * s.db[504][4]));
        let __rspice_deriv_cse_57: f64 = ((s.db[114][5] * s.v[504]) + (s.v[114] * s.db[504][5]));
        let __rspice_deriv_cse_58: f64 = ((s.db[114][6] * s.v[504]) + (s.v[114] * s.db[504][6]));
        let __rspice_deriv_cse_59: f64 = ((s.db[114][7] * s.v[504]) + (s.v[114] * s.db[504][7]));
        let __rspice_deriv_cse_60: f64 = ((s.db[114][8] * s.v[504]) + (s.v[114] * s.db[504][8]));
        let __rspice_deriv_cse_61: f64 = ((s.db[114][9] * s.v[504]) + (s.v[114] * s.db[504][9]));
        let __rspice_deriv_cse_62: f64 = ((s.db[114][10] * s.v[504]) + (s.v[114] * s.db[504][10]));
        let __rspice_deriv_cse_63: f64 = ((s.db[114][11] * s.v[504]) + (s.v[114] * s.db[504][11]));
        let __rspice_deriv_cse_64: f64 = ((s.db[114][12] * s.v[504]) + (s.v[114] * s.db[504][12]));
        let __rspice_deriv_cse_65: f64 = ((s.db[114][13] * s.v[504]) + (s.v[114] * s.db[504][13]));
        let __rspice_deriv_cse_66: f64 = ((s.db[114][14] * s.v[504]) + (s.v[114] * s.db[504][14]));
        let __rspice_deriv_cse_67: f64 = ((s.db[114][15] * s.v[504]) + (s.v[114] * s.db[504][15]));
        let __rspice_deriv_cse_68: f64 = ((s.db[114][16] * s.v[504]) + (s.v[114] * s.db[504][16]));
        let __rspice_deriv_cse_69: f64 = ((s.db[114][17] * s.v[504]) + (s.v[114] * s.db[504][17]));
        let (eq41_e2311, eq41_e2311_d_n0, eq41_e2311_d_n1, eq41_e2311_d_n2, eq41_e2311_d_n3, eq41_e2311_d_n4, eq41_e2311_d_n5, eq41_e2311_d_n6, eq41_e2311_d_n7, eq41_e2311_d_n8, eq41_e2311_d_n9, eq41_e2311_d_n10, eq41_e2311_d_n11, eq41_e2311_d_n12, eq41_e2311_d_n13, eq41_e2311_d_n14, eq41_e2311_d_n15, eq41_e2311_d_n16, eq41_e2311_d_b0, eq41_e2311_d_b1, eq41_e2311_d_b2, eq41_e2311_d_b3, eq41_e2311_d_b4, eq41_e2311_d_b5, eq41_e2311_d_b6, eq41_e2311_d_b7, eq41_e2311_d_b8, eq41_e2311_d_b9, eq41_e2311_d_b10, eq41_e2311_d_b11, eq41_e2311_d_b12, eq41_e2311_d_b13, eq41_e2311_d_b14, eq41_e2311_d_b15, eq41_e2311_d_b16, eq41_e2311_d_b17, eq41_e2311_q, eq41_e2311_q_d_n0, eq41_e2311_q_d_n1, eq41_e2311_q_d_n2, eq41_e2311_q_d_n3, eq41_e2311_q_d_n4, eq41_e2311_q_d_n5, eq41_e2311_q_d_n6, eq41_e2311_q_d_n7, eq41_e2311_q_d_n8, eq41_e2311_q_d_n9, eq41_e2311_q_d_n10, eq41_e2311_q_d_n11, eq41_e2311_q_d_n12, eq41_e2311_q_d_n13, eq41_e2311_q_d_n14, eq41_e2311_q_d_n15, eq41_e2311_q_d_n16, eq41_e2311_q_d_b0, eq41_e2311_q_d_b1, eq41_e2311_q_d_b2, eq41_e2311_q_d_b3, eq41_e2311_q_d_b4, eq41_e2311_q_d_b5, eq41_e2311_q_d_b6, eq41_e2311_q_d_b7, eq41_e2311_q_d_b8, eq41_e2311_q_d_b9, eq41_e2311_q_d_b10, eq41_e2311_q_d_b11, eq41_e2311_q_d_b12, eq41_e2311_q_d_b13, eq41_e2311_q_d_b14, eq41_e2311_q_d_b15, eq41_e2311_q_d_b16, eq41_e2311_q_d_b17,) = {
    if (s.b[1705] && s.b[1706]) {
        let eq41_e2308_q: f64 = s.v[503];
        let eq41_e2309: f64 = (s.v[114] * s.v[503]);
        let eq41_e2309_q: f64 = (s.v[114] * eq41_e2308_q);
        let eq41_e2309_q_d_n0: f64 = ((s.dn[114][0] * eq41_e2308_q) + (s.v[114] * s.dn[503][0]));
        let eq41_e2309_q_d_n1: f64 = ((s.dn[114][1] * eq41_e2308_q) + (s.v[114] * s.dn[503][1]));
        let eq41_e2309_q_d_n2: f64 = ((s.dn[114][2] * eq41_e2308_q) + (s.v[114] * s.dn[503][2]));
        let eq41_e2309_q_d_n3: f64 = ((s.dn[114][3] * eq41_e2308_q) + (s.v[114] * s.dn[503][3]));
        let eq41_e2309_q_d_n4: f64 = ((s.dn[114][4] * eq41_e2308_q) + (s.v[114] * s.dn[503][4]));
        let eq41_e2309_q_d_n5: f64 = ((s.dn[114][5] * eq41_e2308_q) + (s.v[114] * s.dn[503][5]));
        let eq41_e2309_q_d_n6: f64 = ((s.dn[114][6] * eq41_e2308_q) + (s.v[114] * s.dn[503][6]));
        let eq41_e2309_q_d_n7: f64 = ((s.dn[114][7] * eq41_e2308_q) + (s.v[114] * s.dn[503][7]));
        let eq41_e2309_q_d_n8: f64 = ((s.dn[114][8] * eq41_e2308_q) + (s.v[114] * s.dn[503][8]));
        let eq41_e2309_q_d_n9: f64 = ((s.dn[114][9] * eq41_e2308_q) + (s.v[114] * s.dn[503][9]));
        let eq41_e2309_q_d_n10: f64 = ((s.dn[114][10] * eq41_e2308_q) + (s.v[114] * s.dn[503][10]));
        let eq41_e2309_q_d_n11: f64 = ((s.dn[114][11] * eq41_e2308_q) + (s.v[114] * s.dn[503][11]));
        let eq41_e2309_q_d_n12: f64 = ((s.dn[114][12] * eq41_e2308_q) + (s.v[114] * s.dn[503][12]));
        let eq41_e2309_q_d_n13: f64 = ((s.dn[114][13] * eq41_e2308_q) + (s.v[114] * s.dn[503][13]));
        let eq41_e2309_q_d_n14: f64 = ((s.dn[114][14] * eq41_e2308_q) + (s.v[114] * s.dn[503][14]));
        let eq41_e2309_q_d_n15: f64 = ((s.dn[114][15] * eq41_e2308_q) + (s.v[114] * s.dn[503][15]));
        let eq41_e2309_q_d_n16: f64 = ((s.dn[114][16] * eq41_e2308_q) + (s.v[114] * s.dn[503][16]));
        let eq41_e2309_q_d_b0: f64 = ((s.db[114][0] * eq41_e2308_q) + (s.v[114] * s.db[503][0]));
        let eq41_e2309_q_d_b1: f64 = ((s.db[114][1] * eq41_e2308_q) + (s.v[114] * s.db[503][1]));
        let eq41_e2309_q_d_b2: f64 = ((s.db[114][2] * eq41_e2308_q) + (s.v[114] * s.db[503][2]));
        let eq41_e2309_q_d_b3: f64 = ((s.db[114][3] * eq41_e2308_q) + (s.v[114] * s.db[503][3]));
        let eq41_e2309_q_d_b4: f64 = ((s.db[114][4] * eq41_e2308_q) + (s.v[114] * s.db[503][4]));
        let eq41_e2309_q_d_b5: f64 = ((s.db[114][5] * eq41_e2308_q) + (s.v[114] * s.db[503][5]));
        let eq41_e2309_q_d_b6: f64 = ((s.db[114][6] * eq41_e2308_q) + (s.v[114] * s.db[503][6]));
        let eq41_e2309_q_d_b7: f64 = ((s.db[114][7] * eq41_e2308_q) + (s.v[114] * s.db[503][7]));
        let eq41_e2309_q_d_b8: f64 = ((s.db[114][8] * eq41_e2308_q) + (s.v[114] * s.db[503][8]));
        let eq41_e2309_q_d_b9: f64 = ((s.db[114][9] * eq41_e2308_q) + (s.v[114] * s.db[503][9]));
        let eq41_e2309_q_d_b10: f64 = ((s.db[114][10] * eq41_e2308_q) + (s.v[114] * s.db[503][10]));
        let eq41_e2309_q_d_b11: f64 = ((s.db[114][11] * eq41_e2308_q) + (s.v[114] * s.db[503][11]));
        let eq41_e2309_q_d_b12: f64 = ((s.db[114][12] * eq41_e2308_q) + (s.v[114] * s.db[503][12]));
        let eq41_e2309_q_d_b13: f64 = ((s.db[114][13] * eq41_e2308_q) + (s.v[114] * s.db[503][13]));
        let eq41_e2309_q_d_b14: f64 = ((s.db[114][14] * eq41_e2308_q) + (s.v[114] * s.db[503][14]));
        let eq41_e2309_q_d_b15: f64 = ((s.db[114][15] * eq41_e2308_q) + (s.v[114] * s.db[503][15]));
        let eq41_e2309_q_d_b16: f64 = ((s.db[114][16] * eq41_e2308_q) + (s.v[114] * s.db[503][16]));
        let eq41_e2309_q_d_b17: f64 = ((s.db[114][17] * eq41_e2308_q) + (s.v[114] * s.db[503][17]));
        (eq41_e2309, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, eq41_e2309_q, eq41_e2309_q_d_n0, eq41_e2309_q_d_n1, eq41_e2309_q_d_n2, eq41_e2309_q_d_n3, eq41_e2309_q_d_n4, eq41_e2309_q_d_n5, eq41_e2309_q_d_n6, eq41_e2309_q_d_n7, eq41_e2309_q_d_n8, eq41_e2309_q_d_n9, eq41_e2309_q_d_n10, eq41_e2309_q_d_n11, eq41_e2309_q_d_n12, eq41_e2309_q_d_n13, eq41_e2309_q_d_n14, eq41_e2309_q_d_n15, eq41_e2309_q_d_n16, eq41_e2309_q_d_b0, eq41_e2309_q_d_b1, eq41_e2309_q_d_b2, eq41_e2309_q_d_b3, eq41_e2309_q_d_b4, eq41_e2309_q_d_b5, eq41_e2309_q_d_b6, eq41_e2309_q_d_b7, eq41_e2309_q_d_b8, eq41_e2309_q_d_b9, eq41_e2309_q_d_b10, eq41_e2309_q_d_b11, eq41_e2309_q_d_b12, eq41_e2309_q_d_b13, eq41_e2309_q_d_b14, eq41_e2309_q_d_b15, eq41_e2309_q_d_b16, eq41_e2309_q_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq41_reactive_node_derivatives: [f64; 17] = [eq41_e2311_q_d_n0, eq41_e2311_q_d_n1, eq41_e2311_q_d_n2, eq41_e2311_q_d_n3, eq41_e2311_q_d_n4, eq41_e2311_q_d_n5, eq41_e2311_q_d_n6, eq41_e2311_q_d_n7, eq41_e2311_q_d_n8, eq41_e2311_q_d_n9, eq41_e2311_q_d_n10, eq41_e2311_q_d_n11, eq41_e2311_q_d_n12, eq41_e2311_q_d_n13, eq41_e2311_q_d_n14, eq41_e2311_q_d_n15, eq41_e2311_q_d_n16];
        let eq41_reactive_branch_derivatives: [f64; 18] = [eq41_e2311_q_d_b0, eq41_e2311_q_d_b1, eq41_e2311_q_d_b2, eq41_e2311_q_d_b3, eq41_e2311_q_d_b4, eq41_e2311_q_d_b5, eq41_e2311_q_d_b6, eq41_e2311_q_d_b7, eq41_e2311_q_d_b8, eq41_e2311_q_d_b9, eq41_e2311_q_d_b10, eq41_e2311_q_d_b11, eq41_e2311_q_d_b12, eq41_e2311_q_d_b13, eq41_e2311_q_d_b14, eq41_e2311_q_d_b15, eq41_e2311_q_d_b16, eq41_e2311_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq42_e2320, eq42_e2320_d_n0, eq42_e2320_d_n1, eq42_e2320_d_n2, eq42_e2320_d_n3, eq42_e2320_d_n4, eq42_e2320_d_n5, eq42_e2320_d_n6, eq42_e2320_d_n7, eq42_e2320_d_n8, eq42_e2320_d_n9, eq42_e2320_d_n10, eq42_e2320_d_n11, eq42_e2320_d_n12, eq42_e2320_d_n13, eq42_e2320_d_n14, eq42_e2320_d_n15, eq42_e2320_d_n16, eq42_e2320_d_b0, eq42_e2320_d_b1, eq42_e2320_d_b2, eq42_e2320_d_b3, eq42_e2320_d_b4, eq42_e2320_d_b5, eq42_e2320_d_b6, eq42_e2320_d_b7, eq42_e2320_d_b8, eq42_e2320_d_b9, eq42_e2320_d_b10, eq42_e2320_d_b11, eq42_e2320_d_b12, eq42_e2320_d_b13, eq42_e2320_d_b14, eq42_e2320_d_b15, eq42_e2320_d_b16, eq42_e2320_d_b17, eq42_e2320_q, eq42_e2320_q_d_n0, eq42_e2320_q_d_n1, eq42_e2320_q_d_n2, eq42_e2320_q_d_n3, eq42_e2320_q_d_n4, eq42_e2320_q_d_n5, eq42_e2320_q_d_n6, eq42_e2320_q_d_n7, eq42_e2320_q_d_n8, eq42_e2320_q_d_n9, eq42_e2320_q_d_n10, eq42_e2320_q_d_n11, eq42_e2320_q_d_n12, eq42_e2320_q_d_n13, eq42_e2320_q_d_n14, eq42_e2320_q_d_n15, eq42_e2320_q_d_n16, eq42_e2320_q_d_b0, eq42_e2320_q_d_b1, eq42_e2320_q_d_b2, eq42_e2320_q_d_b3, eq42_e2320_q_d_b4, eq42_e2320_q_d_b5, eq42_e2320_q_d_b6, eq42_e2320_q_d_b7, eq42_e2320_q_d_b8, eq42_e2320_q_d_b9, eq42_e2320_q_d_b10, eq42_e2320_q_d_b11, eq42_e2320_q_d_b12, eq42_e2320_q_d_b13, eq42_e2320_q_d_b14, eq42_e2320_q_d_b15, eq42_e2320_q_d_b16, eq42_e2320_q_d_b17,) = {
    if (s.b[1705] && s.b[1706]) {
        let eq42_e2317_q: f64 = s.v[504];
        let eq42_e2318: f64 = (s.v[114] * s.v[504]);
        let eq42_e2318_q: f64 = (s.v[114] * eq42_e2317_q);
        let eq42_e2318_q_d_n0: f64 = ((s.dn[114][0] * eq42_e2317_q) + (s.v[114] * s.dn[504][0]));
        let eq42_e2318_q_d_n1: f64 = ((s.dn[114][1] * eq42_e2317_q) + (s.v[114] * s.dn[504][1]));
        let eq42_e2318_q_d_n2: f64 = ((s.dn[114][2] * eq42_e2317_q) + (s.v[114] * s.dn[504][2]));
        let eq42_e2318_q_d_n3: f64 = ((s.dn[114][3] * eq42_e2317_q) + (s.v[114] * s.dn[504][3]));
        let eq42_e2318_q_d_n4: f64 = ((s.dn[114][4] * eq42_e2317_q) + (s.v[114] * s.dn[504][4]));
        let eq42_e2318_q_d_n5: f64 = ((s.dn[114][5] * eq42_e2317_q) + (s.v[114] * s.dn[504][5]));
        let eq42_e2318_q_d_n6: f64 = ((s.dn[114][6] * eq42_e2317_q) + (s.v[114] * s.dn[504][6]));
        let eq42_e2318_q_d_n7: f64 = ((s.dn[114][7] * eq42_e2317_q) + (s.v[114] * s.dn[504][7]));
        let eq42_e2318_q_d_n8: f64 = ((s.dn[114][8] * eq42_e2317_q) + (s.v[114] * s.dn[504][8]));
        let eq42_e2318_q_d_n9: f64 = ((s.dn[114][9] * eq42_e2317_q) + (s.v[114] * s.dn[504][9]));
        let eq42_e2318_q_d_n10: f64 = ((s.dn[114][10] * eq42_e2317_q) + (s.v[114] * s.dn[504][10]));
        let eq42_e2318_q_d_n11: f64 = ((s.dn[114][11] * eq42_e2317_q) + (s.v[114] * s.dn[504][11]));
        let eq42_e2318_q_d_n12: f64 = ((s.dn[114][12] * eq42_e2317_q) + (s.v[114] * s.dn[504][12]));
        let eq42_e2318_q_d_n13: f64 = ((s.dn[114][13] * eq42_e2317_q) + (s.v[114] * s.dn[504][13]));
        let eq42_e2318_q_d_n14: f64 = ((s.dn[114][14] * eq42_e2317_q) + (s.v[114] * s.dn[504][14]));
        let eq42_e2318_q_d_n15: f64 = ((s.dn[114][15] * eq42_e2317_q) + (s.v[114] * s.dn[504][15]));
        let eq42_e2318_q_d_n16: f64 = ((s.dn[114][16] * eq42_e2317_q) + (s.v[114] * s.dn[504][16]));
        let eq42_e2318_q_d_b0: f64 = ((s.db[114][0] * eq42_e2317_q) + (s.v[114] * s.db[504][0]));
        let eq42_e2318_q_d_b1: f64 = ((s.db[114][1] * eq42_e2317_q) + (s.v[114] * s.db[504][1]));
        let eq42_e2318_q_d_b2: f64 = ((s.db[114][2] * eq42_e2317_q) + (s.v[114] * s.db[504][2]));
        let eq42_e2318_q_d_b3: f64 = ((s.db[114][3] * eq42_e2317_q) + (s.v[114] * s.db[504][3]));
        let eq42_e2318_q_d_b4: f64 = ((s.db[114][4] * eq42_e2317_q) + (s.v[114] * s.db[504][4]));
        let eq42_e2318_q_d_b5: f64 = ((s.db[114][5] * eq42_e2317_q) + (s.v[114] * s.db[504][5]));
        let eq42_e2318_q_d_b6: f64 = ((s.db[114][6] * eq42_e2317_q) + (s.v[114] * s.db[504][6]));
        let eq42_e2318_q_d_b7: f64 = ((s.db[114][7] * eq42_e2317_q) + (s.v[114] * s.db[504][7]));
        let eq42_e2318_q_d_b8: f64 = ((s.db[114][8] * eq42_e2317_q) + (s.v[114] * s.db[504][8]));
        let eq42_e2318_q_d_b9: f64 = ((s.db[114][9] * eq42_e2317_q) + (s.v[114] * s.db[504][9]));
        let eq42_e2318_q_d_b10: f64 = ((s.db[114][10] * eq42_e2317_q) + (s.v[114] * s.db[504][10]));
        let eq42_e2318_q_d_b11: f64 = ((s.db[114][11] * eq42_e2317_q) + (s.v[114] * s.db[504][11]));
        let eq42_e2318_q_d_b12: f64 = ((s.db[114][12] * eq42_e2317_q) + (s.v[114] * s.db[504][12]));
        let eq42_e2318_q_d_b13: f64 = ((s.db[114][13] * eq42_e2317_q) + (s.v[114] * s.db[504][13]));
        let eq42_e2318_q_d_b14: f64 = ((s.db[114][14] * eq42_e2317_q) + (s.v[114] * s.db[504][14]));
        let eq42_e2318_q_d_b15: f64 = ((s.db[114][15] * eq42_e2317_q) + (s.v[114] * s.db[504][15]));
        let eq42_e2318_q_d_b16: f64 = ((s.db[114][16] * eq42_e2317_q) + (s.v[114] * s.db[504][16]));
        let eq42_e2318_q_d_b17: f64 = ((s.db[114][17] * eq42_e2317_q) + (s.v[114] * s.db[504][17]));
        (eq42_e2318, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, eq42_e2318_q, eq42_e2318_q_d_n0, eq42_e2318_q_d_n1, eq42_e2318_q_d_n2, eq42_e2318_q_d_n3, eq42_e2318_q_d_n4, eq42_e2318_q_d_n5, eq42_e2318_q_d_n6, eq42_e2318_q_d_n7, eq42_e2318_q_d_n8, eq42_e2318_q_d_n9, eq42_e2318_q_d_n10, eq42_e2318_q_d_n11, eq42_e2318_q_d_n12, eq42_e2318_q_d_n13, eq42_e2318_q_d_n14, eq42_e2318_q_d_n15, eq42_e2318_q_d_n16, eq42_e2318_q_d_b0, eq42_e2318_q_d_b1, eq42_e2318_q_d_b2, eq42_e2318_q_d_b3, eq42_e2318_q_d_b4, eq42_e2318_q_d_b5, eq42_e2318_q_d_b6, eq42_e2318_q_d_b7, eq42_e2318_q_d_b8, eq42_e2318_q_d_b9, eq42_e2318_q_d_b10, eq42_e2318_q_d_b11, eq42_e2318_q_d_b12, eq42_e2318_q_d_b13, eq42_e2318_q_d_b14, eq42_e2318_q_d_b15, eq42_e2318_q_d_b16, eq42_e2318_q_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq42_reactive_node_derivatives: [f64; 17] = [eq42_e2320_q_d_n0, eq42_e2320_q_d_n1, eq42_e2320_q_d_n2, eq42_e2320_q_d_n3, eq42_e2320_q_d_n4, eq42_e2320_q_d_n5, eq42_e2320_q_d_n6, eq42_e2320_q_d_n7, eq42_e2320_q_d_n8, eq42_e2320_q_d_n9, eq42_e2320_q_d_n10, eq42_e2320_q_d_n11, eq42_e2320_q_d_n12, eq42_e2320_q_d_n13, eq42_e2320_q_d_n14, eq42_e2320_q_d_n15, eq42_e2320_q_d_n16];
        let eq42_reactive_branch_derivatives: [f64; 18] = [eq42_e2320_q_d_b0, eq42_e2320_q_d_b1, eq42_e2320_q_d_b2, eq42_e2320_q_d_b3, eq42_e2320_q_d_b4, eq42_e2320_q_d_b5, eq42_e2320_q_d_b6, eq42_e2320_q_d_b7, eq42_e2320_q_d_b8, eq42_e2320_q_d_b9, eq42_e2320_q_d_b10, eq42_e2320_q_d_b11, eq42_e2320_q_d_b12, eq42_e2320_q_d_b13, eq42_e2320_q_d_b14, eq42_e2320_q_d_b15, eq42_e2320_q_d_b16, eq42_e2320_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq43_e2328, eq43_e2328_d_n0, eq43_e2328_d_n1, eq43_e2328_d_n2, eq43_e2328_d_n3, eq43_e2328_d_n4, eq43_e2328_d_n5, eq43_e2328_d_n6, eq43_e2328_d_n7, eq43_e2328_d_n8, eq43_e2328_d_n9, eq43_e2328_d_n10, eq43_e2328_d_n11, eq43_e2328_d_n12, eq43_e2328_d_n13, eq43_e2328_d_n14, eq43_e2328_d_n15, eq43_e2328_d_n16, eq43_e2328_d_b0, eq43_e2328_d_b1, eq43_e2328_d_b2, eq43_e2328_d_b3, eq43_e2328_d_b4, eq43_e2328_d_b5, eq43_e2328_d_b6, eq43_e2328_d_b7, eq43_e2328_d_b8, eq43_e2328_d_b9, eq43_e2328_d_b10, eq43_e2328_d_b11, eq43_e2328_d_b12, eq43_e2328_d_b13, eq43_e2328_d_b14, eq43_e2328_d_b15, eq43_e2328_d_b16, eq43_e2328_d_b17, eq43_e2328_q,) = {
    if (s.b[1705] && (!s.b[1706])) {
        let eq43_e2326_q: f64 = s.v[506];
        (s.v[506], s.dn[506][0], s.dn[506][1], s.dn[506][2], s.dn[506][3], s.dn[506][4], s.dn[506][5], s.dn[506][6], s.dn[506][7], s.dn[506][8], s.dn[506][9], s.dn[506][10], s.dn[506][11], s.dn[506][12], s.dn[506][13], s.dn[506][14], s.dn[506][15], s.dn[506][16], s.db[506][0], s.db[506][1], s.db[506][2], s.db[506][3], s.db[506][4], s.db[506][5], s.db[506][6], s.db[506][7], s.db[506][8], s.db[506][9], s.db[506][10], s.db[506][11], s.db[506][12], s.db[506][13], s.db[506][14], s.db[506][15], s.db[506][16], s.db[506][17], eq43_e2326_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq43_reactive_node_derivatives: [f64; 17] = [eq43_e2328_d_n0, eq43_e2328_d_n1, eq43_e2328_d_n2, eq43_e2328_d_n3, eq43_e2328_d_n4, eq43_e2328_d_n5, eq43_e2328_d_n6, eq43_e2328_d_n7, eq43_e2328_d_n8, eq43_e2328_d_n9, eq43_e2328_d_n10, eq43_e2328_d_n11, eq43_e2328_d_n12, eq43_e2328_d_n13, eq43_e2328_d_n14, eq43_e2328_d_n15, eq43_e2328_d_n16];
        let eq43_reactive_branch_derivatives: [f64; 18] = [eq43_e2328_d_b0, eq43_e2328_d_b1, eq43_e2328_d_b2, eq43_e2328_d_b3, eq43_e2328_d_b4, eq43_e2328_d_b5, eq43_e2328_d_b6, eq43_e2328_d_b7, eq43_e2328_d_b8, eq43_e2328_d_b9, eq43_e2328_d_b10, eq43_e2328_d_b11, eq43_e2328_d_b12, eq43_e2328_d_b13, eq43_e2328_d_b14, eq43_e2328_d_b15, eq43_e2328_d_b16, eq43_e2328_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            nodes,
            &eq43_reactive_node_derivatives,
            branches,
            &eq43_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq44_e2333, eq44_e2333_d_n0, eq44_e2333_d_n1, eq44_e2333_d_n2, eq44_e2333_d_n3, eq44_e2333_d_n4, eq44_e2333_d_n5, eq44_e2333_d_n6, eq44_e2333_d_n7, eq44_e2333_d_n8, eq44_e2333_d_n9, eq44_e2333_d_n10, eq44_e2333_d_n11, eq44_e2333_d_n12, eq44_e2333_d_n13, eq44_e2333_d_n14, eq44_e2333_d_n15, eq44_e2333_d_n16, eq44_e2333_d_b0, eq44_e2333_d_b1, eq44_e2333_d_b2, eq44_e2333_d_b3, eq44_e2333_d_b4, eq44_e2333_d_b5, eq44_e2333_d_b6, eq44_e2333_d_b7, eq44_e2333_d_b8, eq44_e2333_d_b9, eq44_e2333_d_b10, eq44_e2333_d_b11, eq44_e2333_d_b12, eq44_e2333_d_b13, eq44_e2333_d_b14, eq44_e2333_d_b15, eq44_e2333_d_b16, eq44_e2333_d_b17, eq44_e2333_q,) = {
    if s.b[1705] {
        let eq44_e2331_q: f64 = s.v[502];
        (s.v[502], s.dn[502][0], s.dn[502][1], s.dn[502][2], s.dn[502][3], s.dn[502][4], s.dn[502][5], s.dn[502][6], s.dn[502][7], s.dn[502][8], s.dn[502][9], s.dn[502][10], s.dn[502][11], s.dn[502][12], s.dn[502][13], s.dn[502][14], s.dn[502][15], s.dn[502][16], s.db[502][0], s.db[502][1], s.db[502][2], s.db[502][3], s.db[502][4], s.db[502][5], s.db[502][6], s.db[502][7], s.db[502][8], s.db[502][9], s.db[502][10], s.db[502][11], s.db[502][12], s.db[502][13], s.db[502][14], s.db[502][15], s.db[502][16], s.db[502][17], eq44_e2331_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq44_reactive_node_derivatives: [f64; 17] = [eq44_e2333_d_n0, eq44_e2333_d_n1, eq44_e2333_d_n2, eq44_e2333_d_n3, eq44_e2333_d_n4, eq44_e2333_d_n5, eq44_e2333_d_n6, eq44_e2333_d_n7, eq44_e2333_d_n8, eq44_e2333_d_n9, eq44_e2333_d_n10, eq44_e2333_d_n11, eq44_e2333_d_n12, eq44_e2333_d_n13, eq44_e2333_d_n14, eq44_e2333_d_n15, eq44_e2333_d_n16];
        let eq44_reactive_branch_derivatives: [f64; 18] = [eq44_e2333_d_b0, eq44_e2333_d_b1, eq44_e2333_d_b2, eq44_e2333_d_b3, eq44_e2333_d_b4, eq44_e2333_d_b5, eq44_e2333_d_b6, eq44_e2333_d_b7, eq44_e2333_d_b8, eq44_e2333_d_b9, eq44_e2333_d_b10, eq44_e2333_d_b11, eq44_e2333_d_b12, eq44_e2333_d_b13, eq44_e2333_d_b14, eq44_e2333_d_b15, eq44_e2333_d_b16, eq44_e2333_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            nodes,
            &eq44_reactive_node_derivatives,
            branches,
            &eq44_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq45_e2340, eq45_e2340_d_n0, eq45_e2340_d_n1, eq45_e2340_d_n2, eq45_e2340_d_n3, eq45_e2340_d_n4, eq45_e2340_d_n5, eq45_e2340_d_n6, eq45_e2340_d_n7, eq45_e2340_d_n8, eq45_e2340_d_n9, eq45_e2340_d_n10, eq45_e2340_d_n11, eq45_e2340_d_n12, eq45_e2340_d_n13, eq45_e2340_d_n14, eq45_e2340_d_n15, eq45_e2340_d_n16, eq45_e2340_d_b0, eq45_e2340_d_b1, eq45_e2340_d_b2, eq45_e2340_d_b3, eq45_e2340_d_b4, eq45_e2340_d_b5, eq45_e2340_d_b6, eq45_e2340_d_b7, eq45_e2340_d_b8, eq45_e2340_d_b9, eq45_e2340_d_b10, eq45_e2340_d_b11, eq45_e2340_d_b12, eq45_e2340_d_b13, eq45_e2340_d_b14, eq45_e2340_d_b15, eq45_e2340_d_b16, eq45_e2340_d_b17, eq45_e2340_q,) = {
    if (s.b[1705] && s.b[1707]) {
        let eq45_e2338_q: f64 = s.v[500];
        (s.v[500], s.dn[500][0], s.dn[500][1], s.dn[500][2], s.dn[500][3], s.dn[500][4], s.dn[500][5], s.dn[500][6], s.dn[500][7], s.dn[500][8], s.dn[500][9], s.dn[500][10], s.dn[500][11], s.dn[500][12], s.dn[500][13], s.dn[500][14], s.dn[500][15], s.dn[500][16], s.db[500][0], s.db[500][1], s.db[500][2], s.db[500][3], s.db[500][4], s.db[500][5], s.db[500][6], s.db[500][7], s.db[500][8], s.db[500][9], s.db[500][10], s.db[500][11], s.db[500][12], s.db[500][13], s.db[500][14], s.db[500][15], s.db[500][16], s.db[500][17], eq45_e2338_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq45_reactive_node_derivatives: [f64; 17] = [eq45_e2340_d_n0, eq45_e2340_d_n1, eq45_e2340_d_n2, eq45_e2340_d_n3, eq45_e2340_d_n4, eq45_e2340_d_n5, eq45_e2340_d_n6, eq45_e2340_d_n7, eq45_e2340_d_n8, eq45_e2340_d_n9, eq45_e2340_d_n10, eq45_e2340_d_n11, eq45_e2340_d_n12, eq45_e2340_d_n13, eq45_e2340_d_n14, eq45_e2340_d_n15, eq45_e2340_d_n16];
        let eq45_reactive_branch_derivatives: [f64; 18] = [eq45_e2340_d_b0, eq45_e2340_d_b1, eq45_e2340_d_b2, eq45_e2340_d_b3, eq45_e2340_d_b4, eq45_e2340_d_b5, eq45_e2340_d_b6, eq45_e2340_d_b7, eq45_e2340_d_b8, eq45_e2340_d_b9, eq45_e2340_d_b10, eq45_e2340_d_b11, eq45_e2340_d_b12, eq45_e2340_d_b13, eq45_e2340_d_b14, eq45_e2340_d_b15, eq45_e2340_d_b16, eq45_e2340_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[2]),
            nodes,
            &eq45_reactive_node_derivatives,
            branches,
            &eq45_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq46_e2347, eq46_e2347_d_n0, eq46_e2347_d_n1, eq46_e2347_d_n2, eq46_e2347_d_n3, eq46_e2347_d_n4, eq46_e2347_d_n5, eq46_e2347_d_n6, eq46_e2347_d_n7, eq46_e2347_d_n8, eq46_e2347_d_n9, eq46_e2347_d_n10, eq46_e2347_d_n11, eq46_e2347_d_n12, eq46_e2347_d_n13, eq46_e2347_d_n14, eq46_e2347_d_n15, eq46_e2347_d_n16, eq46_e2347_d_b0, eq46_e2347_d_b1, eq46_e2347_d_b2, eq46_e2347_d_b3, eq46_e2347_d_b4, eq46_e2347_d_b5, eq46_e2347_d_b6, eq46_e2347_d_b7, eq46_e2347_d_b8, eq46_e2347_d_b9, eq46_e2347_d_b10, eq46_e2347_d_b11, eq46_e2347_d_b12, eq46_e2347_d_b13, eq46_e2347_d_b14, eq46_e2347_d_b15, eq46_e2347_d_b16, eq46_e2347_d_b17, eq46_e2347_q,) = {
    if (s.b[1705] && s.b[1707]) {
        let eq46_e2345_q: f64 = s.v[501];
        (s.v[501], s.dn[501][0], s.dn[501][1], s.dn[501][2], s.dn[501][3], s.dn[501][4], s.dn[501][5], s.dn[501][6], s.dn[501][7], s.dn[501][8], s.dn[501][9], s.dn[501][10], s.dn[501][11], s.dn[501][12], s.dn[501][13], s.dn[501][14], s.dn[501][15], s.dn[501][16], s.db[501][0], s.db[501][1], s.db[501][2], s.db[501][3], s.db[501][4], s.db[501][5], s.db[501][6], s.db[501][7], s.db[501][8], s.db[501][9], s.db[501][10], s.db[501][11], s.db[501][12], s.db[501][13], s.db[501][14], s.db[501][15], s.db[501][16], s.db[501][17], eq46_e2345_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq46_reactive_node_derivatives: [f64; 17] = [eq46_e2347_d_n0, eq46_e2347_d_n1, eq46_e2347_d_n2, eq46_e2347_d_n3, eq46_e2347_d_n4, eq46_e2347_d_n5, eq46_e2347_d_n6, eq46_e2347_d_n7, eq46_e2347_d_n8, eq46_e2347_d_n9, eq46_e2347_d_n10, eq46_e2347_d_n11, eq46_e2347_d_n12, eq46_e2347_d_n13, eq46_e2347_d_n14, eq46_e2347_d_n15, eq46_e2347_d_n16];
        let eq46_reactive_branch_derivatives: [f64; 18] = [eq46_e2347_d_b0, eq46_e2347_d_b1, eq46_e2347_d_b2, eq46_e2347_d_b3, eq46_e2347_d_b4, eq46_e2347_d_b5, eq46_e2347_d_b6, eq46_e2347_d_b7, eq46_e2347_d_b8, eq46_e2347_d_b9, eq46_e2347_d_b10, eq46_e2347_d_b11, eq46_e2347_d_b12, eq46_e2347_d_b13, eq46_e2347_d_b14, eq46_e2347_d_b15, eq46_e2347_d_b16, eq46_e2347_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[0]),
            nodes,
            &eq46_reactive_node_derivatives,
            branches,
            &eq46_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq47_e2353, eq47_e2353_d_n0, eq47_e2353_d_n1, eq47_e2353_d_n2, eq47_e2353_d_n3, eq47_e2353_d_n4, eq47_e2353_d_n5, eq47_e2353_d_n6, eq47_e2353_d_n7, eq47_e2353_d_n8, eq47_e2353_d_n9, eq47_e2353_d_n10, eq47_e2353_d_n11, eq47_e2353_d_n12, eq47_e2353_d_n13, eq47_e2353_d_n14, eq47_e2353_d_n15, eq47_e2353_d_n16, eq47_e2353_d_b0, eq47_e2353_d_b1, eq47_e2353_d_b2, eq47_e2353_d_b3, eq47_e2353_d_b4, eq47_e2353_d_b5, eq47_e2353_d_b6, eq47_e2353_d_b7, eq47_e2353_d_b8, eq47_e2353_d_b9, eq47_e2353_d_b10, eq47_e2353_d_b11, eq47_e2353_d_b12, eq47_e2353_d_b13, eq47_e2353_d_b14, eq47_e2353_d_b15, eq47_e2353_d_b16, eq47_e2353_d_b17, eq47_e2353_q,) = {
    if (!s.b[1705]) {
        let eq47_e2351_q: f64 = s.v[505];
        (s.v[505], s.dn[505][0], s.dn[505][1], s.dn[505][2], s.dn[505][3], s.dn[505][4], s.dn[505][5], s.dn[505][6], s.dn[505][7], s.dn[505][8], s.dn[505][9], s.dn[505][10], s.dn[505][11], s.dn[505][12], s.dn[505][13], s.dn[505][14], s.dn[505][15], s.dn[505][16], s.db[505][0], s.db[505][1], s.db[505][2], s.db[505][3], s.db[505][4], s.db[505][5], s.db[505][6], s.db[505][7], s.db[505][8], s.db[505][9], s.db[505][10], s.db[505][11], s.db[505][12], s.db[505][13], s.db[505][14], s.db[505][15], s.db[505][16], s.db[505][17], eq47_e2351_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq47_reactive_node_derivatives: [f64; 17] = [eq47_e2353_d_n0, eq47_e2353_d_n1, eq47_e2353_d_n2, eq47_e2353_d_n3, eq47_e2353_d_n4, eq47_e2353_d_n5, eq47_e2353_d_n6, eq47_e2353_d_n7, eq47_e2353_d_n8, eq47_e2353_d_n9, eq47_e2353_d_n10, eq47_e2353_d_n11, eq47_e2353_d_n12, eq47_e2353_d_n13, eq47_e2353_d_n14, eq47_e2353_d_n15, eq47_e2353_d_n16];
        let eq47_reactive_branch_derivatives: [f64; 18] = [eq47_e2353_d_b0, eq47_e2353_d_b1, eq47_e2353_d_b2, eq47_e2353_d_b3, eq47_e2353_d_b4, eq47_e2353_d_b5, eq47_e2353_d_b6, eq47_e2353_d_b7, eq47_e2353_d_b8, eq47_e2353_d_b9, eq47_e2353_d_b10, eq47_e2353_d_b11, eq47_e2353_d_b12, eq47_e2353_d_b13, eq47_e2353_d_b14, eq47_e2353_d_b15, eq47_e2353_d_b16, eq47_e2353_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[13]),
            Some(nodes[6]),
            nodes,
            &eq47_reactive_node_derivatives,
            branches,
            &eq47_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq48_e2361, eq48_e2361_d_n0, eq48_e2361_d_n1, eq48_e2361_d_n2, eq48_e2361_d_n3, eq48_e2361_d_n4, eq48_e2361_d_n5, eq48_e2361_d_n6, eq48_e2361_d_n7, eq48_e2361_d_n8, eq48_e2361_d_n9, eq48_e2361_d_n10, eq48_e2361_d_n11, eq48_e2361_d_n12, eq48_e2361_d_n13, eq48_e2361_d_n14, eq48_e2361_d_n15, eq48_e2361_d_n16, eq48_e2361_d_b0, eq48_e2361_d_b1, eq48_e2361_d_b2, eq48_e2361_d_b3, eq48_e2361_d_b4, eq48_e2361_d_b5, eq48_e2361_d_b6, eq48_e2361_d_b7, eq48_e2361_d_b8, eq48_e2361_d_b9, eq48_e2361_d_b10, eq48_e2361_d_b11, eq48_e2361_d_b12, eq48_e2361_d_b13, eq48_e2361_d_b14, eq48_e2361_d_b15, eq48_e2361_d_b16, eq48_e2361_d_b17, eq48_e2361_q,) = {
    if ((!s.b[1705]) && s.b[1708]) {
        let eq48_e2359_q: f64 = s.v[506];
        (s.v[506], s.dn[506][0], s.dn[506][1], s.dn[506][2], s.dn[506][3], s.dn[506][4], s.dn[506][5], s.dn[506][6], s.dn[506][7], s.dn[506][8], s.dn[506][9], s.dn[506][10], s.dn[506][11], s.dn[506][12], s.dn[506][13], s.dn[506][14], s.dn[506][15], s.dn[506][16], s.db[506][0], s.db[506][1], s.db[506][2], s.db[506][3], s.db[506][4], s.db[506][5], s.db[506][6], s.db[506][7], s.db[506][8], s.db[506][9], s.db[506][10], s.db[506][11], s.db[506][12], s.db[506][13], s.db[506][14], s.db[506][15], s.db[506][16], s.db[506][17], eq48_e2359_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq48_reactive_node_derivatives: [f64; 17] = [eq48_e2361_d_n0, eq48_e2361_d_n1, eq48_e2361_d_n2, eq48_e2361_d_n3, eq48_e2361_d_n4, eq48_e2361_d_n5, eq48_e2361_d_n6, eq48_e2361_d_n7, eq48_e2361_d_n8, eq48_e2361_d_n9, eq48_e2361_d_n10, eq48_e2361_d_n11, eq48_e2361_d_n12, eq48_e2361_d_n13, eq48_e2361_d_n14, eq48_e2361_d_n15, eq48_e2361_d_n16];
        let eq48_reactive_branch_derivatives: [f64; 18] = [eq48_e2361_d_b0, eq48_e2361_d_b1, eq48_e2361_d_b2, eq48_e2361_d_b3, eq48_e2361_d_b4, eq48_e2361_d_b5, eq48_e2361_d_b6, eq48_e2361_d_b7, eq48_e2361_d_b8, eq48_e2361_d_b9, eq48_e2361_d_b10, eq48_e2361_d_b11, eq48_e2361_d_b12, eq48_e2361_d_b13, eq48_e2361_d_b14, eq48_e2361_d_b15, eq48_e2361_d_b16, eq48_e2361_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[14]),
            Some(nodes[7]),
            nodes,
            &eq48_reactive_node_derivatives,
            branches,
            &eq48_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq49_e2371, eq49_e2371_d_n0, eq49_e2371_d_n1, eq49_e2371_d_n2, eq49_e2371_d_n3, eq49_e2371_d_n4, eq49_e2371_d_n5, eq49_e2371_d_n6, eq49_e2371_d_n7, eq49_e2371_d_n8, eq49_e2371_d_n9, eq49_e2371_d_n10, eq49_e2371_d_n11, eq49_e2371_d_n12, eq49_e2371_d_n13, eq49_e2371_d_n14, eq49_e2371_d_n15, eq49_e2371_d_n16, eq49_e2371_d_b0, eq49_e2371_d_b1, eq49_e2371_d_b2, eq49_e2371_d_b3, eq49_e2371_d_b4, eq49_e2371_d_b5, eq49_e2371_d_b6, eq49_e2371_d_b7, eq49_e2371_d_b8, eq49_e2371_d_b9, eq49_e2371_d_b10, eq49_e2371_d_b11, eq49_e2371_d_b12, eq49_e2371_d_b13, eq49_e2371_d_b14, eq49_e2371_d_b15, eq49_e2371_d_b16, eq49_e2371_d_b17, eq49_e2371_q, eq49_e2371_q_d_n0, eq49_e2371_q_d_n1, eq49_e2371_q_d_n2, eq49_e2371_q_d_n3, eq49_e2371_q_d_n4, eq49_e2371_q_d_n5, eq49_e2371_q_d_n6, eq49_e2371_q_d_n7, eq49_e2371_q_d_n8, eq49_e2371_q_d_n9, eq49_e2371_q_d_n10, eq49_e2371_q_d_n11, eq49_e2371_q_d_n12, eq49_e2371_q_d_n13, eq49_e2371_q_d_n14, eq49_e2371_q_d_n15, eq49_e2371_q_d_n16, eq49_e2371_q_d_b0, eq49_e2371_q_d_b1, eq49_e2371_q_d_b2, eq49_e2371_q_d_b3, eq49_e2371_q_d_b4, eq49_e2371_q_d_b5, eq49_e2371_q_d_b6, eq49_e2371_q_d_b7, eq49_e2371_q_d_b8, eq49_e2371_q_d_b9, eq49_e2371_q_d_b10, eq49_e2371_q_d_b11, eq49_e2371_q_d_b12, eq49_e2371_q_d_b13, eq49_e2371_q_d_b14, eq49_e2371_q_d_b15, eq49_e2371_q_d_b16, eq49_e2371_q_d_b17,) = {
    if ((!s.b[1705]) && s.b[1708]) {
        let eq49_e2368_q: f64 = s.v[503];
        let eq49_e2369: f64 = (s.v[114] * s.v[503]);
        let eq49_e2369_q: f64 = (s.v[114] * eq49_e2368_q);
        let eq49_e2369_q_d_n0: f64 = ((s.dn[114][0] * eq49_e2368_q) + (s.v[114] * s.dn[503][0]));
        let eq49_e2369_q_d_n1: f64 = ((s.dn[114][1] * eq49_e2368_q) + (s.v[114] * s.dn[503][1]));
        let eq49_e2369_q_d_n2: f64 = ((s.dn[114][2] * eq49_e2368_q) + (s.v[114] * s.dn[503][2]));
        let eq49_e2369_q_d_n3: f64 = ((s.dn[114][3] * eq49_e2368_q) + (s.v[114] * s.dn[503][3]));
        let eq49_e2369_q_d_n4: f64 = ((s.dn[114][4] * eq49_e2368_q) + (s.v[114] * s.dn[503][4]));
        let eq49_e2369_q_d_n5: f64 = ((s.dn[114][5] * eq49_e2368_q) + (s.v[114] * s.dn[503][5]));
        let eq49_e2369_q_d_n6: f64 = ((s.dn[114][6] * eq49_e2368_q) + (s.v[114] * s.dn[503][6]));
        let eq49_e2369_q_d_n7: f64 = ((s.dn[114][7] * eq49_e2368_q) + (s.v[114] * s.dn[503][7]));
        let eq49_e2369_q_d_n8: f64 = ((s.dn[114][8] * eq49_e2368_q) + (s.v[114] * s.dn[503][8]));
        let eq49_e2369_q_d_n9: f64 = ((s.dn[114][9] * eq49_e2368_q) + (s.v[114] * s.dn[503][9]));
        let eq49_e2369_q_d_n10: f64 = ((s.dn[114][10] * eq49_e2368_q) + (s.v[114] * s.dn[503][10]));
        let eq49_e2369_q_d_n11: f64 = ((s.dn[114][11] * eq49_e2368_q) + (s.v[114] * s.dn[503][11]));
        let eq49_e2369_q_d_n12: f64 = ((s.dn[114][12] * eq49_e2368_q) + (s.v[114] * s.dn[503][12]));
        let eq49_e2369_q_d_n13: f64 = ((s.dn[114][13] * eq49_e2368_q) + (s.v[114] * s.dn[503][13]));
        let eq49_e2369_q_d_n14: f64 = ((s.dn[114][14] * eq49_e2368_q) + (s.v[114] * s.dn[503][14]));
        let eq49_e2369_q_d_n15: f64 = ((s.dn[114][15] * eq49_e2368_q) + (s.v[114] * s.dn[503][15]));
        let eq49_e2369_q_d_n16: f64 = ((s.dn[114][16] * eq49_e2368_q) + (s.v[114] * s.dn[503][16]));
        let eq49_e2369_q_d_b0: f64 = ((s.db[114][0] * eq49_e2368_q) + (s.v[114] * s.db[503][0]));
        let eq49_e2369_q_d_b1: f64 = ((s.db[114][1] * eq49_e2368_q) + (s.v[114] * s.db[503][1]));
        let eq49_e2369_q_d_b2: f64 = ((s.db[114][2] * eq49_e2368_q) + (s.v[114] * s.db[503][2]));
        let eq49_e2369_q_d_b3: f64 = ((s.db[114][3] * eq49_e2368_q) + (s.v[114] * s.db[503][3]));
        let eq49_e2369_q_d_b4: f64 = ((s.db[114][4] * eq49_e2368_q) + (s.v[114] * s.db[503][4]));
        let eq49_e2369_q_d_b5: f64 = ((s.db[114][5] * eq49_e2368_q) + (s.v[114] * s.db[503][5]));
        let eq49_e2369_q_d_b6: f64 = ((s.db[114][6] * eq49_e2368_q) + (s.v[114] * s.db[503][6]));
        let eq49_e2369_q_d_b7: f64 = ((s.db[114][7] * eq49_e2368_q) + (s.v[114] * s.db[503][7]));
        let eq49_e2369_q_d_b8: f64 = ((s.db[114][8] * eq49_e2368_q) + (s.v[114] * s.db[503][8]));
        let eq49_e2369_q_d_b9: f64 = ((s.db[114][9] * eq49_e2368_q) + (s.v[114] * s.db[503][9]));
        let eq49_e2369_q_d_b10: f64 = ((s.db[114][10] * eq49_e2368_q) + (s.v[114] * s.db[503][10]));
        let eq49_e2369_q_d_b11: f64 = ((s.db[114][11] * eq49_e2368_q) + (s.v[114] * s.db[503][11]));
        let eq49_e2369_q_d_b12: f64 = ((s.db[114][12] * eq49_e2368_q) + (s.v[114] * s.db[503][12]));
        let eq49_e2369_q_d_b13: f64 = ((s.db[114][13] * eq49_e2368_q) + (s.v[114] * s.db[503][13]));
        let eq49_e2369_q_d_b14: f64 = ((s.db[114][14] * eq49_e2368_q) + (s.v[114] * s.db[503][14]));
        let eq49_e2369_q_d_b15: f64 = ((s.db[114][15] * eq49_e2368_q) + (s.v[114] * s.db[503][15]));
        let eq49_e2369_q_d_b16: f64 = ((s.db[114][16] * eq49_e2368_q) + (s.v[114] * s.db[503][16]));
        let eq49_e2369_q_d_b17: f64 = ((s.db[114][17] * eq49_e2368_q) + (s.v[114] * s.db[503][17]));
        (eq49_e2369, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, eq49_e2369_q, eq49_e2369_q_d_n0, eq49_e2369_q_d_n1, eq49_e2369_q_d_n2, eq49_e2369_q_d_n3, eq49_e2369_q_d_n4, eq49_e2369_q_d_n5, eq49_e2369_q_d_n6, eq49_e2369_q_d_n7, eq49_e2369_q_d_n8, eq49_e2369_q_d_n9, eq49_e2369_q_d_n10, eq49_e2369_q_d_n11, eq49_e2369_q_d_n12, eq49_e2369_q_d_n13, eq49_e2369_q_d_n14, eq49_e2369_q_d_n15, eq49_e2369_q_d_n16, eq49_e2369_q_d_b0, eq49_e2369_q_d_b1, eq49_e2369_q_d_b2, eq49_e2369_q_d_b3, eq49_e2369_q_d_b4, eq49_e2369_q_d_b5, eq49_e2369_q_d_b6, eq49_e2369_q_d_b7, eq49_e2369_q_d_b8, eq49_e2369_q_d_b9, eq49_e2369_q_d_b10, eq49_e2369_q_d_b11, eq49_e2369_q_d_b12, eq49_e2369_q_d_b13, eq49_e2369_q_d_b14, eq49_e2369_q_d_b15, eq49_e2369_q_d_b16, eq49_e2369_q_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq49_reactive_node_derivatives: [f64; 17] = [eq49_e2371_q_d_n0, eq49_e2371_q_d_n1, eq49_e2371_q_d_n2, eq49_e2371_q_d_n3, eq49_e2371_q_d_n4, eq49_e2371_q_d_n5, eq49_e2371_q_d_n6, eq49_e2371_q_d_n7, eq49_e2371_q_d_n8, eq49_e2371_q_d_n9, eq49_e2371_q_d_n10, eq49_e2371_q_d_n11, eq49_e2371_q_d_n12, eq49_e2371_q_d_n13, eq49_e2371_q_d_n14, eq49_e2371_q_d_n15, eq49_e2371_q_d_n16];
        let eq49_reactive_branch_derivatives: [f64; 18] = [eq49_e2371_q_d_b0, eq49_e2371_q_d_b1, eq49_e2371_q_d_b2, eq49_e2371_q_d_b3, eq49_e2371_q_d_b4, eq49_e2371_q_d_b5, eq49_e2371_q_d_b6, eq49_e2371_q_d_b7, eq49_e2371_q_d_b8, eq49_e2371_q_d_b9, eq49_e2371_q_d_b10, eq49_e2371_q_d_b11, eq49_e2371_q_d_b12, eq49_e2371_q_d_b13, eq49_e2371_q_d_b14, eq49_e2371_q_d_b15, eq49_e2371_q_d_b16, eq49_e2371_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[14]),
            Some(nodes[7]),
            nodes,
            &eq49_reactive_node_derivatives,
            branches,
            &eq49_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq50_e2381, eq50_e2381_d_n0, eq50_e2381_d_n1, eq50_e2381_d_n2, eq50_e2381_d_n3, eq50_e2381_d_n4, eq50_e2381_d_n5, eq50_e2381_d_n6, eq50_e2381_d_n7, eq50_e2381_d_n8, eq50_e2381_d_n9, eq50_e2381_d_n10, eq50_e2381_d_n11, eq50_e2381_d_n12, eq50_e2381_d_n13, eq50_e2381_d_n14, eq50_e2381_d_n15, eq50_e2381_d_n16, eq50_e2381_d_b0, eq50_e2381_d_b1, eq50_e2381_d_b2, eq50_e2381_d_b3, eq50_e2381_d_b4, eq50_e2381_d_b5, eq50_e2381_d_b6, eq50_e2381_d_b7, eq50_e2381_d_b8, eq50_e2381_d_b9, eq50_e2381_d_b10, eq50_e2381_d_b11, eq50_e2381_d_b12, eq50_e2381_d_b13, eq50_e2381_d_b14, eq50_e2381_d_b15, eq50_e2381_d_b16, eq50_e2381_d_b17, eq50_e2381_q, eq50_e2381_q_d_n0, eq50_e2381_q_d_n1, eq50_e2381_q_d_n2, eq50_e2381_q_d_n3, eq50_e2381_q_d_n4, eq50_e2381_q_d_n5, eq50_e2381_q_d_n6, eq50_e2381_q_d_n7, eq50_e2381_q_d_n8, eq50_e2381_q_d_n9, eq50_e2381_q_d_n10, eq50_e2381_q_d_n11, eq50_e2381_q_d_n12, eq50_e2381_q_d_n13, eq50_e2381_q_d_n14, eq50_e2381_q_d_n15, eq50_e2381_q_d_n16, eq50_e2381_q_d_b0, eq50_e2381_q_d_b1, eq50_e2381_q_d_b2, eq50_e2381_q_d_b3, eq50_e2381_q_d_b4, eq50_e2381_q_d_b5, eq50_e2381_q_d_b6, eq50_e2381_q_d_b7, eq50_e2381_q_d_b8, eq50_e2381_q_d_b9, eq50_e2381_q_d_b10, eq50_e2381_q_d_b11, eq50_e2381_q_d_b12, eq50_e2381_q_d_b13, eq50_e2381_q_d_b14, eq50_e2381_q_d_b15, eq50_e2381_q_d_b16, eq50_e2381_q_d_b17,) = {
    if ((!s.b[1705]) && s.b[1708]) {
        let eq50_e2378_q: f64 = s.v[504];
        let eq50_e2379: f64 = (s.v[114] * s.v[504]);
        let eq50_e2379_q: f64 = (s.v[114] * eq50_e2378_q);
        let eq50_e2379_q_d_n0: f64 = ((s.dn[114][0] * eq50_e2378_q) + (s.v[114] * s.dn[504][0]));
        let eq50_e2379_q_d_n1: f64 = ((s.dn[114][1] * eq50_e2378_q) + (s.v[114] * s.dn[504][1]));
        let eq50_e2379_q_d_n2: f64 = ((s.dn[114][2] * eq50_e2378_q) + (s.v[114] * s.dn[504][2]));
        let eq50_e2379_q_d_n3: f64 = ((s.dn[114][3] * eq50_e2378_q) + (s.v[114] * s.dn[504][3]));
        let eq50_e2379_q_d_n4: f64 = ((s.dn[114][4] * eq50_e2378_q) + (s.v[114] * s.dn[504][4]));
        let eq50_e2379_q_d_n5: f64 = ((s.dn[114][5] * eq50_e2378_q) + (s.v[114] * s.dn[504][5]));
        let eq50_e2379_q_d_n6: f64 = ((s.dn[114][6] * eq50_e2378_q) + (s.v[114] * s.dn[504][6]));
        let eq50_e2379_q_d_n7: f64 = ((s.dn[114][7] * eq50_e2378_q) + (s.v[114] * s.dn[504][7]));
        let eq50_e2379_q_d_n8: f64 = ((s.dn[114][8] * eq50_e2378_q) + (s.v[114] * s.dn[504][8]));
        let eq50_e2379_q_d_n9: f64 = ((s.dn[114][9] * eq50_e2378_q) + (s.v[114] * s.dn[504][9]));
        let eq50_e2379_q_d_n10: f64 = ((s.dn[114][10] * eq50_e2378_q) + (s.v[114] * s.dn[504][10]));
        let eq50_e2379_q_d_n11: f64 = ((s.dn[114][11] * eq50_e2378_q) + (s.v[114] * s.dn[504][11]));
        let eq50_e2379_q_d_n12: f64 = ((s.dn[114][12] * eq50_e2378_q) + (s.v[114] * s.dn[504][12]));
        let eq50_e2379_q_d_n13: f64 = ((s.dn[114][13] * eq50_e2378_q) + (s.v[114] * s.dn[504][13]));
        let eq50_e2379_q_d_n14: f64 = ((s.dn[114][14] * eq50_e2378_q) + (s.v[114] * s.dn[504][14]));
        let eq50_e2379_q_d_n15: f64 = ((s.dn[114][15] * eq50_e2378_q) + (s.v[114] * s.dn[504][15]));
        let eq50_e2379_q_d_n16: f64 = ((s.dn[114][16] * eq50_e2378_q) + (s.v[114] * s.dn[504][16]));
        let eq50_e2379_q_d_b0: f64 = ((s.db[114][0] * eq50_e2378_q) + (s.v[114] * s.db[504][0]));
        let eq50_e2379_q_d_b1: f64 = ((s.db[114][1] * eq50_e2378_q) + (s.v[114] * s.db[504][1]));
        let eq50_e2379_q_d_b2: f64 = ((s.db[114][2] * eq50_e2378_q) + (s.v[114] * s.db[504][2]));
        let eq50_e2379_q_d_b3: f64 = ((s.db[114][3] * eq50_e2378_q) + (s.v[114] * s.db[504][3]));
        let eq50_e2379_q_d_b4: f64 = ((s.db[114][4] * eq50_e2378_q) + (s.v[114] * s.db[504][4]));
        let eq50_e2379_q_d_b5: f64 = ((s.db[114][5] * eq50_e2378_q) + (s.v[114] * s.db[504][5]));
        let eq50_e2379_q_d_b6: f64 = ((s.db[114][6] * eq50_e2378_q) + (s.v[114] * s.db[504][6]));
        let eq50_e2379_q_d_b7: f64 = ((s.db[114][7] * eq50_e2378_q) + (s.v[114] * s.db[504][7]));
        let eq50_e2379_q_d_b8: f64 = ((s.db[114][8] * eq50_e2378_q) + (s.v[114] * s.db[504][8]));
        let eq50_e2379_q_d_b9: f64 = ((s.db[114][9] * eq50_e2378_q) + (s.v[114] * s.db[504][9]));
        let eq50_e2379_q_d_b10: f64 = ((s.db[114][10] * eq50_e2378_q) + (s.v[114] * s.db[504][10]));
        let eq50_e2379_q_d_b11: f64 = ((s.db[114][11] * eq50_e2378_q) + (s.v[114] * s.db[504][11]));
        let eq50_e2379_q_d_b12: f64 = ((s.db[114][12] * eq50_e2378_q) + (s.v[114] * s.db[504][12]));
        let eq50_e2379_q_d_b13: f64 = ((s.db[114][13] * eq50_e2378_q) + (s.v[114] * s.db[504][13]));
        let eq50_e2379_q_d_b14: f64 = ((s.db[114][14] * eq50_e2378_q) + (s.v[114] * s.db[504][14]));
        let eq50_e2379_q_d_b15: f64 = ((s.db[114][15] * eq50_e2378_q) + (s.v[114] * s.db[504][15]));
        let eq50_e2379_q_d_b16: f64 = ((s.db[114][16] * eq50_e2378_q) + (s.v[114] * s.db[504][16]));
        let eq50_e2379_q_d_b17: f64 = ((s.db[114][17] * eq50_e2378_q) + (s.v[114] * s.db[504][17]));
        (eq50_e2379, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, eq50_e2379_q, eq50_e2379_q_d_n0, eq50_e2379_q_d_n1, eq50_e2379_q_d_n2, eq50_e2379_q_d_n3, eq50_e2379_q_d_n4, eq50_e2379_q_d_n5, eq50_e2379_q_d_n6, eq50_e2379_q_d_n7, eq50_e2379_q_d_n8, eq50_e2379_q_d_n9, eq50_e2379_q_d_n10, eq50_e2379_q_d_n11, eq50_e2379_q_d_n12, eq50_e2379_q_d_n13, eq50_e2379_q_d_n14, eq50_e2379_q_d_n15, eq50_e2379_q_d_n16, eq50_e2379_q_d_b0, eq50_e2379_q_d_b1, eq50_e2379_q_d_b2, eq50_e2379_q_d_b3, eq50_e2379_q_d_b4, eq50_e2379_q_d_b5, eq50_e2379_q_d_b6, eq50_e2379_q_d_b7, eq50_e2379_q_d_b8, eq50_e2379_q_d_b9, eq50_e2379_q_d_b10, eq50_e2379_q_d_b11, eq50_e2379_q_d_b12, eq50_e2379_q_d_b13, eq50_e2379_q_d_b14, eq50_e2379_q_d_b15, eq50_e2379_q_d_b16, eq50_e2379_q_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_reactive_node_derivatives: [f64; 17] = [eq50_e2381_q_d_n0, eq50_e2381_q_d_n1, eq50_e2381_q_d_n2, eq50_e2381_q_d_n3, eq50_e2381_q_d_n4, eq50_e2381_q_d_n5, eq50_e2381_q_d_n6, eq50_e2381_q_d_n7, eq50_e2381_q_d_n8, eq50_e2381_q_d_n9, eq50_e2381_q_d_n10, eq50_e2381_q_d_n11, eq50_e2381_q_d_n12, eq50_e2381_q_d_n13, eq50_e2381_q_d_n14, eq50_e2381_q_d_n15, eq50_e2381_q_d_n16];
        let eq50_reactive_branch_derivatives: [f64; 18] = [eq50_e2381_q_d_b0, eq50_e2381_q_d_b1, eq50_e2381_q_d_b2, eq50_e2381_q_d_b3, eq50_e2381_q_d_b4, eq50_e2381_q_d_b5, eq50_e2381_q_d_b6, eq50_e2381_q_d_b7, eq50_e2381_q_d_b8, eq50_e2381_q_d_b9, eq50_e2381_q_d_b10, eq50_e2381_q_d_b11, eq50_e2381_q_d_b12, eq50_e2381_q_d_b13, eq50_e2381_q_d_b14, eq50_e2381_q_d_b15, eq50_e2381_q_d_b16, eq50_e2381_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[14]),
            Some(nodes[5]),
            nodes,
            &eq50_reactive_node_derivatives,
            branches,
            &eq50_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq51_e2390, eq51_e2390_d_n0, eq51_e2390_d_n1, eq51_e2390_d_n2, eq51_e2390_d_n3, eq51_e2390_d_n4, eq51_e2390_d_n5, eq51_e2390_d_n6, eq51_e2390_d_n7, eq51_e2390_d_n8, eq51_e2390_d_n9, eq51_e2390_d_n10, eq51_e2390_d_n11, eq51_e2390_d_n12, eq51_e2390_d_n13, eq51_e2390_d_n14, eq51_e2390_d_n15, eq51_e2390_d_n16, eq51_e2390_d_b0, eq51_e2390_d_b1, eq51_e2390_d_b2, eq51_e2390_d_b3, eq51_e2390_d_b4, eq51_e2390_d_b5, eq51_e2390_d_b6, eq51_e2390_d_b7, eq51_e2390_d_b8, eq51_e2390_d_b9, eq51_e2390_d_b10, eq51_e2390_d_b11, eq51_e2390_d_b12, eq51_e2390_d_b13, eq51_e2390_d_b14, eq51_e2390_d_b15, eq51_e2390_d_b16, eq51_e2390_d_b17, eq51_e2390_q,) = {
    if ((!s.b[1705]) && (!s.b[1708])) {
        let eq51_e2388_q: f64 = s.v[506];
        (s.v[506], s.dn[506][0], s.dn[506][1], s.dn[506][2], s.dn[506][3], s.dn[506][4], s.dn[506][5], s.dn[506][6], s.dn[506][7], s.dn[506][8], s.dn[506][9], s.dn[506][10], s.dn[506][11], s.dn[506][12], s.dn[506][13], s.dn[506][14], s.dn[506][15], s.dn[506][16], s.db[506][0], s.db[506][1], s.db[506][2], s.db[506][3], s.db[506][4], s.db[506][5], s.db[506][6], s.db[506][7], s.db[506][8], s.db[506][9], s.db[506][10], s.db[506][11], s.db[506][12], s.db[506][13], s.db[506][14], s.db[506][15], s.db[506][16], s.db[506][17], eq51_e2388_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_reactive_node_derivatives: [f64; 17] = [eq51_e2390_d_n0, eq51_e2390_d_n1, eq51_e2390_d_n2, eq51_e2390_d_n3, eq51_e2390_d_n4, eq51_e2390_d_n5, eq51_e2390_d_n6, eq51_e2390_d_n7, eq51_e2390_d_n8, eq51_e2390_d_n9, eq51_e2390_d_n10, eq51_e2390_d_n11, eq51_e2390_d_n12, eq51_e2390_d_n13, eq51_e2390_d_n14, eq51_e2390_d_n15, eq51_e2390_d_n16];
        let eq51_reactive_branch_derivatives: [f64; 18] = [eq51_e2390_d_b0, eq51_e2390_d_b1, eq51_e2390_d_b2, eq51_e2390_d_b3, eq51_e2390_d_b4, eq51_e2390_d_b5, eq51_e2390_d_b6, eq51_e2390_d_b7, eq51_e2390_d_b8, eq51_e2390_d_b9, eq51_e2390_d_b10, eq51_e2390_d_b11, eq51_e2390_d_b12, eq51_e2390_d_b13, eq51_e2390_d_b14, eq51_e2390_d_b15, eq51_e2390_d_b16, eq51_e2390_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[14]),
            Some(nodes[5]),
            nodes,
            &eq51_reactive_node_derivatives,
            branches,
            &eq51_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let __rspice_deriv_cse_0: f64 = ((0.7071 * s.dn[632][0]) * (nv16 - 0.0));
        let __rspice_deriv_cse_1: f64 = ((0.7071 * s.dn[632][1]) * (nv16 - 0.0));
        let __rspice_deriv_cse_2: f64 = ((0.7071 * s.dn[632][2]) * (nv16 - 0.0));
        let __rspice_deriv_cse_3: f64 = ((0.7071 * s.dn[632][3]) * (nv16 - 0.0));
        let __rspice_deriv_cse_4: f64 = ((0.7071 * s.dn[632][4]) * (nv16 - 0.0));
        let __rspice_deriv_cse_5: f64 = ((0.7071 * s.dn[632][5]) * (nv16 - 0.0));
        let __rspice_deriv_cse_6: f64 = ((0.7071 * s.dn[632][6]) * (nv16 - 0.0));
        let __rspice_deriv_cse_7: f64 = ((0.7071 * s.dn[632][7]) * (nv16 - 0.0));
        let __rspice_deriv_cse_8: f64 = ((0.7071 * s.dn[632][8]) * (nv16 - 0.0));
        let __rspice_deriv_cse_9: f64 = ((0.7071 * s.dn[632][9]) * (nv16 - 0.0));
        let __rspice_deriv_cse_10: f64 = ((0.7071 * s.dn[632][10]) * (nv16 - 0.0));
        let __rspice_deriv_cse_11: f64 = ((0.7071 * s.dn[632][11]) * (nv16 - 0.0));
        let __rspice_deriv_cse_12: f64 = ((0.7071 * s.dn[632][12]) * (nv16 - 0.0));
        let __rspice_deriv_cse_13: f64 = ((0.7071 * s.dn[632][13]) * (nv16 - 0.0));
        let __rspice_deriv_cse_14: f64 = ((0.7071 * s.dn[632][14]) * (nv16 - 0.0));
        let __rspice_deriv_cse_15: f64 = ((0.7071 * s.dn[632][15]) * (nv16 - 0.0));
        let __rspice_deriv_cse_16: f64 = ((0.7071 * s.db[632][0]) * (nv16 - 0.0));
        let __rspice_deriv_cse_17: f64 = ((0.7071 * s.db[632][1]) * (nv16 - 0.0));
        let __rspice_deriv_cse_18: f64 = ((0.7071 * s.db[632][2]) * (nv16 - 0.0));
        let __rspice_deriv_cse_19: f64 = ((0.7071 * s.db[632][3]) * (nv16 - 0.0));
        let __rspice_deriv_cse_20: f64 = ((0.7071 * s.db[632][4]) * (nv16 - 0.0));
        let __rspice_deriv_cse_21: f64 = ((0.7071 * s.db[632][5]) * (nv16 - 0.0));
        let __rspice_deriv_cse_22: f64 = ((0.7071 * s.db[632][6]) * (nv16 - 0.0));
        let __rspice_deriv_cse_23: f64 = ((0.7071 * s.db[632][7]) * (nv16 - 0.0));
        let __rspice_deriv_cse_24: f64 = ((0.7071 * s.db[632][8]) * (nv16 - 0.0));
        let __rspice_deriv_cse_25: f64 = ((0.7071 * s.db[632][9]) * (nv16 - 0.0));
        let __rspice_deriv_cse_26: f64 = ((0.7071 * s.db[632][10]) * (nv16 - 0.0));
        let __rspice_deriv_cse_27: f64 = ((0.7071 * s.db[632][11]) * (nv16 - 0.0));
        let __rspice_deriv_cse_28: f64 = ((0.7071 * s.db[632][12]) * (nv16 - 0.0));
        let __rspice_deriv_cse_29: f64 = ((0.7071 * s.db[632][13]) * (nv16 - 0.0));
        let __rspice_deriv_cse_30: f64 = ((0.7071 * s.db[632][14]) * (nv16 - 0.0));
        let __rspice_deriv_cse_31: f64 = ((0.7071 * s.db[632][15]) * (nv16 - 0.0));
        let __rspice_deriv_cse_32: f64 = ((0.7071 * s.db[632][16]) * (nv16 - 0.0));
        let __rspice_deriv_cse_33: f64 = ((0.7071 * s.db[632][17]) * (nv16 - 0.0));
        let (eq52_e2396, eq52_e2396_d_n0, eq52_e2396_d_n1, eq52_e2396_d_n2, eq52_e2396_d_n3, eq52_e2396_d_n4, eq52_e2396_d_n5, eq52_e2396_d_n6, eq52_e2396_d_n7, eq52_e2396_d_n8, eq52_e2396_d_n9, eq52_e2396_d_n10, eq52_e2396_d_n11, eq52_e2396_d_n12, eq52_e2396_d_n13, eq52_e2396_d_n14, eq52_e2396_d_n15, eq52_e2396_d_n16, eq52_e2396_d_b0, eq52_e2396_d_b1, eq52_e2396_d_b2, eq52_e2396_d_b3, eq52_e2396_d_b4, eq52_e2396_d_b5, eq52_e2396_d_b6, eq52_e2396_d_b7, eq52_e2396_d_b8, eq52_e2396_d_b9, eq52_e2396_d_b10, eq52_e2396_d_b11, eq52_e2396_d_b12, eq52_e2396_d_b13, eq52_e2396_d_b14, eq52_e2396_d_b15, eq52_e2396_d_b16, eq52_e2396_d_b17, eq52_e2396_q,) = {
    if (!s.b[1705]) {
        let eq52_e2394_q: f64 = s.v[502];
        (s.v[502], s.dn[502][0], s.dn[502][1], s.dn[502][2], s.dn[502][3], s.dn[502][4], s.dn[502][5], s.dn[502][6], s.dn[502][7], s.dn[502][8], s.dn[502][9], s.dn[502][10], s.dn[502][11], s.dn[502][12], s.dn[502][13], s.dn[502][14], s.dn[502][15], s.dn[502][16], s.db[502][0], s.db[502][1], s.db[502][2], s.db[502][3], s.db[502][4], s.db[502][5], s.db[502][6], s.db[502][7], s.db[502][8], s.db[502][9], s.db[502][10], s.db[502][11], s.db[502][12], s.db[502][13], s.db[502][14], s.db[502][15], s.db[502][16], s.db[502][17], eq52_e2394_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_reactive_node_derivatives: [f64; 17] = [eq52_e2396_d_n0, eq52_e2396_d_n1, eq52_e2396_d_n2, eq52_e2396_d_n3, eq52_e2396_d_n4, eq52_e2396_d_n5, eq52_e2396_d_n6, eq52_e2396_d_n7, eq52_e2396_d_n8, eq52_e2396_d_n9, eq52_e2396_d_n10, eq52_e2396_d_n11, eq52_e2396_d_n12, eq52_e2396_d_n13, eq52_e2396_d_n14, eq52_e2396_d_n15, eq52_e2396_d_n16];
        let eq52_reactive_branch_derivatives: [f64; 18] = [eq52_e2396_d_b0, eq52_e2396_d_b1, eq52_e2396_d_b2, eq52_e2396_d_b3, eq52_e2396_d_b4, eq52_e2396_d_b5, eq52_e2396_d_b6, eq52_e2396_d_b7, eq52_e2396_d_b8, eq52_e2396_d_b9, eq52_e2396_d_b10, eq52_e2396_d_b11, eq52_e2396_d_b12, eq52_e2396_d_b13, eq52_e2396_d_b14, eq52_e2396_d_b15, eq52_e2396_d_b16, eq52_e2396_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            nodes,
            &eq52_reactive_node_derivatives,
            branches,
            &eq52_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq53_e2404, eq53_e2404_d_n0, eq53_e2404_d_n1, eq53_e2404_d_n2, eq53_e2404_d_n3, eq53_e2404_d_n4, eq53_e2404_d_n5, eq53_e2404_d_n6, eq53_e2404_d_n7, eq53_e2404_d_n8, eq53_e2404_d_n9, eq53_e2404_d_n10, eq53_e2404_d_n11, eq53_e2404_d_n12, eq53_e2404_d_n13, eq53_e2404_d_n14, eq53_e2404_d_n15, eq53_e2404_d_n16, eq53_e2404_d_b0, eq53_e2404_d_b1, eq53_e2404_d_b2, eq53_e2404_d_b3, eq53_e2404_d_b4, eq53_e2404_d_b5, eq53_e2404_d_b6, eq53_e2404_d_b7, eq53_e2404_d_b8, eq53_e2404_d_b9, eq53_e2404_d_b10, eq53_e2404_d_b11, eq53_e2404_d_b12, eq53_e2404_d_b13, eq53_e2404_d_b14, eq53_e2404_d_b15, eq53_e2404_d_b16, eq53_e2404_d_b17, eq53_e2404_q,) = {
    if ((!s.b[1705]) && s.b[1709]) {
        let eq53_e2402_q: f64 = s.v[500];
        (s.v[500], s.dn[500][0], s.dn[500][1], s.dn[500][2], s.dn[500][3], s.dn[500][4], s.dn[500][5], s.dn[500][6], s.dn[500][7], s.dn[500][8], s.dn[500][9], s.dn[500][10], s.dn[500][11], s.dn[500][12], s.dn[500][13], s.dn[500][14], s.dn[500][15], s.dn[500][16], s.db[500][0], s.db[500][1], s.db[500][2], s.db[500][3], s.db[500][4], s.db[500][5], s.db[500][6], s.db[500][7], s.db[500][8], s.db[500][9], s.db[500][10], s.db[500][11], s.db[500][12], s.db[500][13], s.db[500][14], s.db[500][15], s.db[500][16], s.db[500][17], eq53_e2402_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_reactive_node_derivatives: [f64; 17] = [eq53_e2404_d_n0, eq53_e2404_d_n1, eq53_e2404_d_n2, eq53_e2404_d_n3, eq53_e2404_d_n4, eq53_e2404_d_n5, eq53_e2404_d_n6, eq53_e2404_d_n7, eq53_e2404_d_n8, eq53_e2404_d_n9, eq53_e2404_d_n10, eq53_e2404_d_n11, eq53_e2404_d_n12, eq53_e2404_d_n13, eq53_e2404_d_n14, eq53_e2404_d_n15, eq53_e2404_d_n16];
        let eq53_reactive_branch_derivatives: [f64; 18] = [eq53_e2404_d_b0, eq53_e2404_d_b1, eq53_e2404_d_b2, eq53_e2404_d_b3, eq53_e2404_d_b4, eq53_e2404_d_b5, eq53_e2404_d_b6, eq53_e2404_d_b7, eq53_e2404_d_b8, eq53_e2404_d_b9, eq53_e2404_d_b10, eq53_e2404_d_b11, eq53_e2404_d_b12, eq53_e2404_d_b13, eq53_e2404_d_b14, eq53_e2404_d_b15, eq53_e2404_d_b16, eq53_e2404_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[13]),
            Some(nodes[2]),
            nodes,
            &eq53_reactive_node_derivatives,
            branches,
            &eq53_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq54_e2412, eq54_e2412_d_n0, eq54_e2412_d_n1, eq54_e2412_d_n2, eq54_e2412_d_n3, eq54_e2412_d_n4, eq54_e2412_d_n5, eq54_e2412_d_n6, eq54_e2412_d_n7, eq54_e2412_d_n8, eq54_e2412_d_n9, eq54_e2412_d_n10, eq54_e2412_d_n11, eq54_e2412_d_n12, eq54_e2412_d_n13, eq54_e2412_d_n14, eq54_e2412_d_n15, eq54_e2412_d_n16, eq54_e2412_d_b0, eq54_e2412_d_b1, eq54_e2412_d_b2, eq54_e2412_d_b3, eq54_e2412_d_b4, eq54_e2412_d_b5, eq54_e2412_d_b6, eq54_e2412_d_b7, eq54_e2412_d_b8, eq54_e2412_d_b9, eq54_e2412_d_b10, eq54_e2412_d_b11, eq54_e2412_d_b12, eq54_e2412_d_b13, eq54_e2412_d_b14, eq54_e2412_d_b15, eq54_e2412_d_b16, eq54_e2412_d_b17, eq54_e2412_q,) = {
    if ((!s.b[1705]) && s.b[1709]) {
        let eq54_e2410_q: f64 = s.v[501];
        (s.v[501], s.dn[501][0], s.dn[501][1], s.dn[501][2], s.dn[501][3], s.dn[501][4], s.dn[501][5], s.dn[501][6], s.dn[501][7], s.dn[501][8], s.dn[501][9], s.dn[501][10], s.dn[501][11], s.dn[501][12], s.dn[501][13], s.dn[501][14], s.dn[501][15], s.dn[501][16], s.db[501][0], s.db[501][1], s.db[501][2], s.db[501][3], s.db[501][4], s.db[501][5], s.db[501][6], s.db[501][7], s.db[501][8], s.db[501][9], s.db[501][10], s.db[501][11], s.db[501][12], s.db[501][13], s.db[501][14], s.db[501][15], s.db[501][16], s.db[501][17], eq54_e2410_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_reactive_node_derivatives: [f64; 17] = [eq54_e2412_d_n0, eq54_e2412_d_n1, eq54_e2412_d_n2, eq54_e2412_d_n3, eq54_e2412_d_n4, eq54_e2412_d_n5, eq54_e2412_d_n6, eq54_e2412_d_n7, eq54_e2412_d_n8, eq54_e2412_d_n9, eq54_e2412_d_n10, eq54_e2412_d_n11, eq54_e2412_d_n12, eq54_e2412_d_n13, eq54_e2412_d_n14, eq54_e2412_d_n15, eq54_e2412_d_n16];
        let eq54_reactive_branch_derivatives: [f64; 18] = [eq54_e2412_d_b0, eq54_e2412_d_b1, eq54_e2412_d_b2, eq54_e2412_d_b3, eq54_e2412_d_b4, eq54_e2412_d_b5, eq54_e2412_d_b6, eq54_e2412_d_b7, eq54_e2412_d_b8, eq54_e2412_d_b9, eq54_e2412_d_b10, eq54_e2412_d_b11, eq54_e2412_d_b12, eq54_e2412_d_b13, eq54_e2412_d_b14, eq54_e2412_d_b15, eq54_e2412_d_b16, eq54_e2412_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[14]),
            Some(nodes[0]),
            nodes,
            &eq54_reactive_node_derivatives,
            branches,
            &eq54_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq55_e2419, eq55_e2419_d_n0, eq55_e2419_d_n1, eq55_e2419_d_n2, eq55_e2419_d_n3, eq55_e2419_d_n4, eq55_e2419_d_n5, eq55_e2419_d_n6, eq55_e2419_d_n7, eq55_e2419_d_n8, eq55_e2419_d_n9, eq55_e2419_d_n10, eq55_e2419_d_n11, eq55_e2419_d_n12, eq55_e2419_d_n13, eq55_e2419_d_n14, eq55_e2419_d_n15, eq55_e2419_d_n16, eq55_e2419_d_b0, eq55_e2419_d_b1, eq55_e2419_d_b2, eq55_e2419_d_b3, eq55_e2419_d_b4, eq55_e2419_d_b5, eq55_e2419_d_b6, eq55_e2419_d_b7, eq55_e2419_d_b8, eq55_e2419_d_b9, eq55_e2419_d_b10, eq55_e2419_d_b11, eq55_e2419_d_b12, eq55_e2419_d_b13, eq55_e2419_d_b14, eq55_e2419_d_b15, eq55_e2419_d_b16, eq55_e2419_d_b17, eq55_e2419_q, eq55_e2419_q_d_n0, eq55_e2419_q_d_n1, eq55_e2419_q_d_n2, eq55_e2419_q_d_n3, eq55_e2419_q_d_n4, eq55_e2419_q_d_n5, eq55_e2419_q_d_n6, eq55_e2419_q_d_n7, eq55_e2419_q_d_n8, eq55_e2419_q_d_n9, eq55_e2419_q_d_n10, eq55_e2419_q_d_n11, eq55_e2419_q_d_n12, eq55_e2419_q_d_n13, eq55_e2419_q_d_n14, eq55_e2419_q_d_n15, eq55_e2419_q_d_n16, eq55_e2419_q_d_b0, eq55_e2419_q_d_b1, eq55_e2419_q_d_b2, eq55_e2419_q_d_b3, eq55_e2419_q_d_b4, eq55_e2419_q_d_b5, eq55_e2419_q_d_b6, eq55_e2419_q_d_b7, eq55_e2419_q_d_b8, eq55_e2419_q_d_b9, eq55_e2419_q_d_b10, eq55_e2419_q_d_b11, eq55_e2419_q_d_b12, eq55_e2419_q_d_b13, eq55_e2419_q_d_b14, eq55_e2419_q_d_b15, eq55_e2419_q_d_b16, eq55_e2419_q_d_b17,) = {
    if s.b[1710] {
        let eq55_e2416_q: f64 = s.v[495];
        let eq55_e2417: f64 = (s.v[114] * s.v[495]);
        let eq55_e2417_d_n0: f64 = ((s.dn[114][0] * s.v[495]) + (s.v[114] * s.dn[495][0]));
        let eq55_e2417_d_n1: f64 = ((s.dn[114][1] * s.v[495]) + (s.v[114] * s.dn[495][1]));
        let eq55_e2417_d_n2: f64 = ((s.dn[114][2] * s.v[495]) + (s.v[114] * s.dn[495][2]));
        let eq55_e2417_d_n3: f64 = ((s.dn[114][3] * s.v[495]) + (s.v[114] * s.dn[495][3]));
        let eq55_e2417_d_n4: f64 = ((s.dn[114][4] * s.v[495]) + (s.v[114] * s.dn[495][4]));
        let eq55_e2417_d_n5: f64 = ((s.dn[114][5] * s.v[495]) + (s.v[114] * s.dn[495][5]));
        let eq55_e2417_d_n6: f64 = ((s.dn[114][6] * s.v[495]) + (s.v[114] * s.dn[495][6]));
        let eq55_e2417_d_n7: f64 = ((s.dn[114][7] * s.v[495]) + (s.v[114] * s.dn[495][7]));
        let eq55_e2417_d_n8: f64 = ((s.dn[114][8] * s.v[495]) + (s.v[114] * s.dn[495][8]));
        let eq55_e2417_d_n9: f64 = ((s.dn[114][9] * s.v[495]) + (s.v[114] * s.dn[495][9]));
        let eq55_e2417_d_n10: f64 = ((s.dn[114][10] * s.v[495]) + (s.v[114] * s.dn[495][10]));
        let eq55_e2417_d_n11: f64 = ((s.dn[114][11] * s.v[495]) + (s.v[114] * s.dn[495][11]));
        let eq55_e2417_d_n12: f64 = ((s.dn[114][12] * s.v[495]) + (s.v[114] * s.dn[495][12]));
        let eq55_e2417_d_n13: f64 = ((s.dn[114][13] * s.v[495]) + (s.v[114] * s.dn[495][13]));
        let eq55_e2417_d_n14: f64 = ((s.dn[114][14] * s.v[495]) + (s.v[114] * s.dn[495][14]));
        let eq55_e2417_d_n15: f64 = ((s.dn[114][15] * s.v[495]) + (s.v[114] * s.dn[495][15]));
        let eq55_e2417_d_n16: f64 = ((s.dn[114][16] * s.v[495]) + (s.v[114] * s.dn[495][16]));
        let eq55_e2417_d_b0: f64 = ((s.db[114][0] * s.v[495]) + (s.v[114] * s.db[495][0]));
        let eq55_e2417_d_b1: f64 = ((s.db[114][1] * s.v[495]) + (s.v[114] * s.db[495][1]));
        let eq55_e2417_d_b2: f64 = ((s.db[114][2] * s.v[495]) + (s.v[114] * s.db[495][2]));
        let eq55_e2417_d_b3: f64 = ((s.db[114][3] * s.v[495]) + (s.v[114] * s.db[495][3]));
        let eq55_e2417_d_b4: f64 = ((s.db[114][4] * s.v[495]) + (s.v[114] * s.db[495][4]));
        let eq55_e2417_d_b5: f64 = ((s.db[114][5] * s.v[495]) + (s.v[114] * s.db[495][5]));
        let eq55_e2417_d_b6: f64 = ((s.db[114][6] * s.v[495]) + (s.v[114] * s.db[495][6]));
        let eq55_e2417_d_b7: f64 = ((s.db[114][7] * s.v[495]) + (s.v[114] * s.db[495][7]));
        let eq55_e2417_d_b8: f64 = ((s.db[114][8] * s.v[495]) + (s.v[114] * s.db[495][8]));
        let eq55_e2417_d_b9: f64 = ((s.db[114][9] * s.v[495]) + (s.v[114] * s.db[495][9]));
        let eq55_e2417_d_b10: f64 = ((s.db[114][10] * s.v[495]) + (s.v[114] * s.db[495][10]));
        let eq55_e2417_d_b11: f64 = ((s.db[114][11] * s.v[495]) + (s.v[114] * s.db[495][11]));
        let eq55_e2417_d_b12: f64 = ((s.db[114][12] * s.v[495]) + (s.v[114] * s.db[495][12]));
        let eq55_e2417_d_b13: f64 = ((s.db[114][13] * s.v[495]) + (s.v[114] * s.db[495][13]));
        let eq55_e2417_d_b14: f64 = ((s.db[114][14] * s.v[495]) + (s.v[114] * s.db[495][14]));
        let eq55_e2417_d_b15: f64 = ((s.db[114][15] * s.v[495]) + (s.v[114] * s.db[495][15]));
        let eq55_e2417_d_b16: f64 = ((s.db[114][16] * s.v[495]) + (s.v[114] * s.db[495][16]));
        let eq55_e2417_d_b17: f64 = ((s.db[114][17] * s.v[495]) + (s.v[114] * s.db[495][17]));
        let eq55_e2417_q: f64 = (s.v[114] * eq55_e2416_q);
        let eq55_e2417_q_d_n0: f64 = ((s.dn[114][0] * eq55_e2416_q) + (s.v[114] * s.dn[495][0]));
        let eq55_e2417_q_d_n1: f64 = ((s.dn[114][1] * eq55_e2416_q) + (s.v[114] * s.dn[495][1]));
        let eq55_e2417_q_d_n2: f64 = ((s.dn[114][2] * eq55_e2416_q) + (s.v[114] * s.dn[495][2]));
        let eq55_e2417_q_d_n3: f64 = ((s.dn[114][3] * eq55_e2416_q) + (s.v[114] * s.dn[495][3]));
        let eq55_e2417_q_d_n4: f64 = ((s.dn[114][4] * eq55_e2416_q) + (s.v[114] * s.dn[495][4]));
        let eq55_e2417_q_d_n5: f64 = ((s.dn[114][5] * eq55_e2416_q) + (s.v[114] * s.dn[495][5]));
        let eq55_e2417_q_d_n6: f64 = ((s.dn[114][6] * eq55_e2416_q) + (s.v[114] * s.dn[495][6]));
        let eq55_e2417_q_d_n7: f64 = ((s.dn[114][7] * eq55_e2416_q) + (s.v[114] * s.dn[495][7]));
        let eq55_e2417_q_d_n8: f64 = ((s.dn[114][8] * eq55_e2416_q) + (s.v[114] * s.dn[495][8]));
        let eq55_e2417_q_d_n9: f64 = ((s.dn[114][9] * eq55_e2416_q) + (s.v[114] * s.dn[495][9]));
        let eq55_e2417_q_d_n10: f64 = ((s.dn[114][10] * eq55_e2416_q) + (s.v[114] * s.dn[495][10]));
        let eq55_e2417_q_d_n11: f64 = ((s.dn[114][11] * eq55_e2416_q) + (s.v[114] * s.dn[495][11]));
        let eq55_e2417_q_d_n12: f64 = ((s.dn[114][12] * eq55_e2416_q) + (s.v[114] * s.dn[495][12]));
        let eq55_e2417_q_d_n13: f64 = ((s.dn[114][13] * eq55_e2416_q) + (s.v[114] * s.dn[495][13]));
        let eq55_e2417_q_d_n14: f64 = ((s.dn[114][14] * eq55_e2416_q) + (s.v[114] * s.dn[495][14]));
        let eq55_e2417_q_d_n15: f64 = ((s.dn[114][15] * eq55_e2416_q) + (s.v[114] * s.dn[495][15]));
        let eq55_e2417_q_d_n16: f64 = ((s.dn[114][16] * eq55_e2416_q) + (s.v[114] * s.dn[495][16]));
        let eq55_e2417_q_d_b0: f64 = ((s.db[114][0] * eq55_e2416_q) + (s.v[114] * s.db[495][0]));
        let eq55_e2417_q_d_b1: f64 = ((s.db[114][1] * eq55_e2416_q) + (s.v[114] * s.db[495][1]));
        let eq55_e2417_q_d_b2: f64 = ((s.db[114][2] * eq55_e2416_q) + (s.v[114] * s.db[495][2]));
        let eq55_e2417_q_d_b3: f64 = ((s.db[114][3] * eq55_e2416_q) + (s.v[114] * s.db[495][3]));
        let eq55_e2417_q_d_b4: f64 = ((s.db[114][4] * eq55_e2416_q) + (s.v[114] * s.db[495][4]));
        let eq55_e2417_q_d_b5: f64 = ((s.db[114][5] * eq55_e2416_q) + (s.v[114] * s.db[495][5]));
        let eq55_e2417_q_d_b6: f64 = ((s.db[114][6] * eq55_e2416_q) + (s.v[114] * s.db[495][6]));
        let eq55_e2417_q_d_b7: f64 = ((s.db[114][7] * eq55_e2416_q) + (s.v[114] * s.db[495][7]));
        let eq55_e2417_q_d_b8: f64 = ((s.db[114][8] * eq55_e2416_q) + (s.v[114] * s.db[495][8]));
        let eq55_e2417_q_d_b9: f64 = ((s.db[114][9] * eq55_e2416_q) + (s.v[114] * s.db[495][9]));
        let eq55_e2417_q_d_b10: f64 = ((s.db[114][10] * eq55_e2416_q) + (s.v[114] * s.db[495][10]));
        let eq55_e2417_q_d_b11: f64 = ((s.db[114][11] * eq55_e2416_q) + (s.v[114] * s.db[495][11]));
        let eq55_e2417_q_d_b12: f64 = ((s.db[114][12] * eq55_e2416_q) + (s.v[114] * s.db[495][12]));
        let eq55_e2417_q_d_b13: f64 = ((s.db[114][13] * eq55_e2416_q) + (s.v[114] * s.db[495][13]));
        let eq55_e2417_q_d_b14: f64 = ((s.db[114][14] * eq55_e2416_q) + (s.v[114] * s.db[495][14]));
        let eq55_e2417_q_d_b15: f64 = ((s.db[114][15] * eq55_e2416_q) + (s.v[114] * s.db[495][15]));
        let eq55_e2417_q_d_b16: f64 = ((s.db[114][16] * eq55_e2416_q) + (s.v[114] * s.db[495][16]));
        let eq55_e2417_q_d_b17: f64 = ((s.db[114][17] * eq55_e2416_q) + (s.v[114] * s.db[495][17]));
        (eq55_e2417, eq55_e2417_d_n0, eq55_e2417_d_n1, eq55_e2417_d_n2, eq55_e2417_d_n3, eq55_e2417_d_n4, eq55_e2417_d_n5, eq55_e2417_d_n6, eq55_e2417_d_n7, eq55_e2417_d_n8, eq55_e2417_d_n9, eq55_e2417_d_n10, eq55_e2417_d_n11, eq55_e2417_d_n12, eq55_e2417_d_n13, eq55_e2417_d_n14, eq55_e2417_d_n15, eq55_e2417_d_n16, eq55_e2417_d_b0, eq55_e2417_d_b1, eq55_e2417_d_b2, eq55_e2417_d_b3, eq55_e2417_d_b4, eq55_e2417_d_b5, eq55_e2417_d_b6, eq55_e2417_d_b7, eq55_e2417_d_b8, eq55_e2417_d_b9, eq55_e2417_d_b10, eq55_e2417_d_b11, eq55_e2417_d_b12, eq55_e2417_d_b13, eq55_e2417_d_b14, eq55_e2417_d_b15, eq55_e2417_d_b16, eq55_e2417_d_b17, eq55_e2417_q, eq55_e2417_q_d_n0, eq55_e2417_q_d_n1, eq55_e2417_q_d_n2, eq55_e2417_q_d_n3, eq55_e2417_q_d_n4, eq55_e2417_q_d_n5, eq55_e2417_q_d_n6, eq55_e2417_q_d_n7, eq55_e2417_q_d_n8, eq55_e2417_q_d_n9, eq55_e2417_q_d_n10, eq55_e2417_q_d_n11, eq55_e2417_q_d_n12, eq55_e2417_q_d_n13, eq55_e2417_q_d_n14, eq55_e2417_q_d_n15, eq55_e2417_q_d_n16, eq55_e2417_q_d_b0, eq55_e2417_q_d_b1, eq55_e2417_q_d_b2, eq55_e2417_q_d_b3, eq55_e2417_q_d_b4, eq55_e2417_q_d_b5, eq55_e2417_q_d_b6, eq55_e2417_q_d_b7, eq55_e2417_q_d_b8, eq55_e2417_q_d_b9, eq55_e2417_q_d_b10, eq55_e2417_q_d_b11, eq55_e2417_q_d_b12, eq55_e2417_q_d_b13, eq55_e2417_q_d_b14, eq55_e2417_q_d_b15, eq55_e2417_q_d_b16, eq55_e2417_q_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_reactive_node_derivatives: [f64; 17] = [eq55_e2419_q_d_n0, eq55_e2419_q_d_n1, eq55_e2419_q_d_n2, eq55_e2419_q_d_n3, eq55_e2419_q_d_n4, eq55_e2419_q_d_n5, eq55_e2419_q_d_n6, eq55_e2419_q_d_n7, eq55_e2419_q_d_n8, eq55_e2419_q_d_n9, eq55_e2419_q_d_n10, eq55_e2419_q_d_n11, eq55_e2419_q_d_n12, eq55_e2419_q_d_n13, eq55_e2419_q_d_n14, eq55_e2419_q_d_n15, eq55_e2419_q_d_n16];
        let eq55_reactive_branch_derivatives: [f64; 18] = [eq55_e2419_q_d_b0, eq55_e2419_q_d_b1, eq55_e2419_q_d_b2, eq55_e2419_q_d_b3, eq55_e2419_q_d_b4, eq55_e2419_q_d_b5, eq55_e2419_q_d_b6, eq55_e2419_q_d_b7, eq55_e2419_q_d_b8, eq55_e2419_q_d_b9, eq55_e2419_q_d_b10, eq55_e2419_q_d_b11, eq55_e2419_q_d_b12, eq55_e2419_q_d_b13, eq55_e2419_q_d_b14, eq55_e2419_q_d_b15, eq55_e2419_q_d_b16, eq55_e2419_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            nodes,
            &eq55_reactive_node_derivatives,
            branches,
            &eq55_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq56_e2426, eq56_e2426_d_n0, eq56_e2426_d_n1, eq56_e2426_d_n2, eq56_e2426_d_n3, eq56_e2426_d_n4, eq56_e2426_d_n5, eq56_e2426_d_n6, eq56_e2426_d_n7, eq56_e2426_d_n8, eq56_e2426_d_n9, eq56_e2426_d_n10, eq56_e2426_d_n11, eq56_e2426_d_n12, eq56_e2426_d_n13, eq56_e2426_d_n14, eq56_e2426_d_n15, eq56_e2426_d_n16, eq56_e2426_d_b0, eq56_e2426_d_b1, eq56_e2426_d_b2, eq56_e2426_d_b3, eq56_e2426_d_b4, eq56_e2426_d_b5, eq56_e2426_d_b6, eq56_e2426_d_b7, eq56_e2426_d_b8, eq56_e2426_d_b9, eq56_e2426_d_b10, eq56_e2426_d_b11, eq56_e2426_d_b12, eq56_e2426_d_b13, eq56_e2426_d_b14, eq56_e2426_d_b15, eq56_e2426_d_b16, eq56_e2426_d_b17, eq56_e2426_q, eq56_e2426_q_d_n0, eq56_e2426_q_d_n1, eq56_e2426_q_d_n2, eq56_e2426_q_d_n3, eq56_e2426_q_d_n4, eq56_e2426_q_d_n5, eq56_e2426_q_d_n6, eq56_e2426_q_d_n7, eq56_e2426_q_d_n8, eq56_e2426_q_d_n9, eq56_e2426_q_d_n10, eq56_e2426_q_d_n11, eq56_e2426_q_d_n12, eq56_e2426_q_d_n13, eq56_e2426_q_d_n14, eq56_e2426_q_d_n15, eq56_e2426_q_d_n16, eq56_e2426_q_d_b0, eq56_e2426_q_d_b1, eq56_e2426_q_d_b2, eq56_e2426_q_d_b3, eq56_e2426_q_d_b4, eq56_e2426_q_d_b5, eq56_e2426_q_d_b6, eq56_e2426_q_d_b7, eq56_e2426_q_d_b8, eq56_e2426_q_d_b9, eq56_e2426_q_d_b10, eq56_e2426_q_d_b11, eq56_e2426_q_d_b12, eq56_e2426_q_d_b13, eq56_e2426_q_d_b14, eq56_e2426_q_d_b15, eq56_e2426_q_d_b16, eq56_e2426_q_d_b17,) = {
    if s.b[1710] {
        let eq56_e2423_q: f64 = s.v[496];
        let eq56_e2424: f64 = (s.v[114] * s.v[496]);
        let eq56_e2424_d_n0: f64 = ((s.dn[114][0] * s.v[496]) + (s.v[114] * s.dn[496][0]));
        let eq56_e2424_d_n1: f64 = ((s.dn[114][1] * s.v[496]) + (s.v[114] * s.dn[496][1]));
        let eq56_e2424_d_n2: f64 = ((s.dn[114][2] * s.v[496]) + (s.v[114] * s.dn[496][2]));
        let eq56_e2424_d_n3: f64 = ((s.dn[114][3] * s.v[496]) + (s.v[114] * s.dn[496][3]));
        let eq56_e2424_d_n4: f64 = ((s.dn[114][4] * s.v[496]) + (s.v[114] * s.dn[496][4]));
        let eq56_e2424_d_n5: f64 = ((s.dn[114][5] * s.v[496]) + (s.v[114] * s.dn[496][5]));
        let eq56_e2424_d_n6: f64 = ((s.dn[114][6] * s.v[496]) + (s.v[114] * s.dn[496][6]));
        let eq56_e2424_d_n7: f64 = ((s.dn[114][7] * s.v[496]) + (s.v[114] * s.dn[496][7]));
        let eq56_e2424_d_n8: f64 = ((s.dn[114][8] * s.v[496]) + (s.v[114] * s.dn[496][8]));
        let eq56_e2424_d_n9: f64 = ((s.dn[114][9] * s.v[496]) + (s.v[114] * s.dn[496][9]));
        let eq56_e2424_d_n10: f64 = ((s.dn[114][10] * s.v[496]) + (s.v[114] * s.dn[496][10]));
        let eq56_e2424_d_n11: f64 = ((s.dn[114][11] * s.v[496]) + (s.v[114] * s.dn[496][11]));
        let eq56_e2424_d_n12: f64 = ((s.dn[114][12] * s.v[496]) + (s.v[114] * s.dn[496][12]));
        let eq56_e2424_d_n13: f64 = ((s.dn[114][13] * s.v[496]) + (s.v[114] * s.dn[496][13]));
        let eq56_e2424_d_n14: f64 = ((s.dn[114][14] * s.v[496]) + (s.v[114] * s.dn[496][14]));
        let eq56_e2424_d_n15: f64 = ((s.dn[114][15] * s.v[496]) + (s.v[114] * s.dn[496][15]));
        let eq56_e2424_d_n16: f64 = ((s.dn[114][16] * s.v[496]) + (s.v[114] * s.dn[496][16]));
        let eq56_e2424_d_b0: f64 = ((s.db[114][0] * s.v[496]) + (s.v[114] * s.db[496][0]));
        let eq56_e2424_d_b1: f64 = ((s.db[114][1] * s.v[496]) + (s.v[114] * s.db[496][1]));
        let eq56_e2424_d_b2: f64 = ((s.db[114][2] * s.v[496]) + (s.v[114] * s.db[496][2]));
        let eq56_e2424_d_b3: f64 = ((s.db[114][3] * s.v[496]) + (s.v[114] * s.db[496][3]));
        let eq56_e2424_d_b4: f64 = ((s.db[114][4] * s.v[496]) + (s.v[114] * s.db[496][4]));
        let eq56_e2424_d_b5: f64 = ((s.db[114][5] * s.v[496]) + (s.v[114] * s.db[496][5]));
        let eq56_e2424_d_b6: f64 = ((s.db[114][6] * s.v[496]) + (s.v[114] * s.db[496][6]));
        let eq56_e2424_d_b7: f64 = ((s.db[114][7] * s.v[496]) + (s.v[114] * s.db[496][7]));
        let eq56_e2424_d_b8: f64 = ((s.db[114][8] * s.v[496]) + (s.v[114] * s.db[496][8]));
        let eq56_e2424_d_b9: f64 = ((s.db[114][9] * s.v[496]) + (s.v[114] * s.db[496][9]));
        let eq56_e2424_d_b10: f64 = ((s.db[114][10] * s.v[496]) + (s.v[114] * s.db[496][10]));
        let eq56_e2424_d_b11: f64 = ((s.db[114][11] * s.v[496]) + (s.v[114] * s.db[496][11]));
        let eq56_e2424_d_b12: f64 = ((s.db[114][12] * s.v[496]) + (s.v[114] * s.db[496][12]));
        let eq56_e2424_d_b13: f64 = ((s.db[114][13] * s.v[496]) + (s.v[114] * s.db[496][13]));
        let eq56_e2424_d_b14: f64 = ((s.db[114][14] * s.v[496]) + (s.v[114] * s.db[496][14]));
        let eq56_e2424_d_b15: f64 = ((s.db[114][15] * s.v[496]) + (s.v[114] * s.db[496][15]));
        let eq56_e2424_d_b16: f64 = ((s.db[114][16] * s.v[496]) + (s.v[114] * s.db[496][16]));
        let eq56_e2424_d_b17: f64 = ((s.db[114][17] * s.v[496]) + (s.v[114] * s.db[496][17]));
        let eq56_e2424_q: f64 = (s.v[114] * eq56_e2423_q);
        let eq56_e2424_q_d_n0: f64 = ((s.dn[114][0] * eq56_e2423_q) + (s.v[114] * s.dn[496][0]));
        let eq56_e2424_q_d_n1: f64 = ((s.dn[114][1] * eq56_e2423_q) + (s.v[114] * s.dn[496][1]));
        let eq56_e2424_q_d_n2: f64 = ((s.dn[114][2] * eq56_e2423_q) + (s.v[114] * s.dn[496][2]));
        let eq56_e2424_q_d_n3: f64 = ((s.dn[114][3] * eq56_e2423_q) + (s.v[114] * s.dn[496][3]));
        let eq56_e2424_q_d_n4: f64 = ((s.dn[114][4] * eq56_e2423_q) + (s.v[114] * s.dn[496][4]));
        let eq56_e2424_q_d_n5: f64 = ((s.dn[114][5] * eq56_e2423_q) + (s.v[114] * s.dn[496][5]));
        let eq56_e2424_q_d_n6: f64 = ((s.dn[114][6] * eq56_e2423_q) + (s.v[114] * s.dn[496][6]));
        let eq56_e2424_q_d_n7: f64 = ((s.dn[114][7] * eq56_e2423_q) + (s.v[114] * s.dn[496][7]));
        let eq56_e2424_q_d_n8: f64 = ((s.dn[114][8] * eq56_e2423_q) + (s.v[114] * s.dn[496][8]));
        let eq56_e2424_q_d_n9: f64 = ((s.dn[114][9] * eq56_e2423_q) + (s.v[114] * s.dn[496][9]));
        let eq56_e2424_q_d_n10: f64 = ((s.dn[114][10] * eq56_e2423_q) + (s.v[114] * s.dn[496][10]));
        let eq56_e2424_q_d_n11: f64 = ((s.dn[114][11] * eq56_e2423_q) + (s.v[114] * s.dn[496][11]));
        let eq56_e2424_q_d_n12: f64 = ((s.dn[114][12] * eq56_e2423_q) + (s.v[114] * s.dn[496][12]));
        let eq56_e2424_q_d_n13: f64 = ((s.dn[114][13] * eq56_e2423_q) + (s.v[114] * s.dn[496][13]));
        let eq56_e2424_q_d_n14: f64 = ((s.dn[114][14] * eq56_e2423_q) + (s.v[114] * s.dn[496][14]));
        let eq56_e2424_q_d_n15: f64 = ((s.dn[114][15] * eq56_e2423_q) + (s.v[114] * s.dn[496][15]));
        let eq56_e2424_q_d_n16: f64 = ((s.dn[114][16] * eq56_e2423_q) + (s.v[114] * s.dn[496][16]));
        let eq56_e2424_q_d_b0: f64 = ((s.db[114][0] * eq56_e2423_q) + (s.v[114] * s.db[496][0]));
        let eq56_e2424_q_d_b1: f64 = ((s.db[114][1] * eq56_e2423_q) + (s.v[114] * s.db[496][1]));
        let eq56_e2424_q_d_b2: f64 = ((s.db[114][2] * eq56_e2423_q) + (s.v[114] * s.db[496][2]));
        let eq56_e2424_q_d_b3: f64 = ((s.db[114][3] * eq56_e2423_q) + (s.v[114] * s.db[496][3]));
        let eq56_e2424_q_d_b4: f64 = ((s.db[114][4] * eq56_e2423_q) + (s.v[114] * s.db[496][4]));
        let eq56_e2424_q_d_b5: f64 = ((s.db[114][5] * eq56_e2423_q) + (s.v[114] * s.db[496][5]));
        let eq56_e2424_q_d_b6: f64 = ((s.db[114][6] * eq56_e2423_q) + (s.v[114] * s.db[496][6]));
        let eq56_e2424_q_d_b7: f64 = ((s.db[114][7] * eq56_e2423_q) + (s.v[114] * s.db[496][7]));
        let eq56_e2424_q_d_b8: f64 = ((s.db[114][8] * eq56_e2423_q) + (s.v[114] * s.db[496][8]));
        let eq56_e2424_q_d_b9: f64 = ((s.db[114][9] * eq56_e2423_q) + (s.v[114] * s.db[496][9]));
        let eq56_e2424_q_d_b10: f64 = ((s.db[114][10] * eq56_e2423_q) + (s.v[114] * s.db[496][10]));
        let eq56_e2424_q_d_b11: f64 = ((s.db[114][11] * eq56_e2423_q) + (s.v[114] * s.db[496][11]));
        let eq56_e2424_q_d_b12: f64 = ((s.db[114][12] * eq56_e2423_q) + (s.v[114] * s.db[496][12]));
        let eq56_e2424_q_d_b13: f64 = ((s.db[114][13] * eq56_e2423_q) + (s.v[114] * s.db[496][13]));
        let eq56_e2424_q_d_b14: f64 = ((s.db[114][14] * eq56_e2423_q) + (s.v[114] * s.db[496][14]));
        let eq56_e2424_q_d_b15: f64 = ((s.db[114][15] * eq56_e2423_q) + (s.v[114] * s.db[496][15]));
        let eq56_e2424_q_d_b16: f64 = ((s.db[114][16] * eq56_e2423_q) + (s.v[114] * s.db[496][16]));
        let eq56_e2424_q_d_b17: f64 = ((s.db[114][17] * eq56_e2423_q) + (s.v[114] * s.db[496][17]));
        (eq56_e2424, eq56_e2424_d_n0, eq56_e2424_d_n1, eq56_e2424_d_n2, eq56_e2424_d_n3, eq56_e2424_d_n4, eq56_e2424_d_n5, eq56_e2424_d_n6, eq56_e2424_d_n7, eq56_e2424_d_n8, eq56_e2424_d_n9, eq56_e2424_d_n10, eq56_e2424_d_n11, eq56_e2424_d_n12, eq56_e2424_d_n13, eq56_e2424_d_n14, eq56_e2424_d_n15, eq56_e2424_d_n16, eq56_e2424_d_b0, eq56_e2424_d_b1, eq56_e2424_d_b2, eq56_e2424_d_b3, eq56_e2424_d_b4, eq56_e2424_d_b5, eq56_e2424_d_b6, eq56_e2424_d_b7, eq56_e2424_d_b8, eq56_e2424_d_b9, eq56_e2424_d_b10, eq56_e2424_d_b11, eq56_e2424_d_b12, eq56_e2424_d_b13, eq56_e2424_d_b14, eq56_e2424_d_b15, eq56_e2424_d_b16, eq56_e2424_d_b17, eq56_e2424_q, eq56_e2424_q_d_n0, eq56_e2424_q_d_n1, eq56_e2424_q_d_n2, eq56_e2424_q_d_n3, eq56_e2424_q_d_n4, eq56_e2424_q_d_n5, eq56_e2424_q_d_n6, eq56_e2424_q_d_n7, eq56_e2424_q_d_n8, eq56_e2424_q_d_n9, eq56_e2424_q_d_n10, eq56_e2424_q_d_n11, eq56_e2424_q_d_n12, eq56_e2424_q_d_n13, eq56_e2424_q_d_n14, eq56_e2424_q_d_n15, eq56_e2424_q_d_n16, eq56_e2424_q_d_b0, eq56_e2424_q_d_b1, eq56_e2424_q_d_b2, eq56_e2424_q_d_b3, eq56_e2424_q_d_b4, eq56_e2424_q_d_b5, eq56_e2424_q_d_b6, eq56_e2424_q_d_b7, eq56_e2424_q_d_b8, eq56_e2424_q_d_b9, eq56_e2424_q_d_b10, eq56_e2424_q_d_b11, eq56_e2424_q_d_b12, eq56_e2424_q_d_b13, eq56_e2424_q_d_b14, eq56_e2424_q_d_b15, eq56_e2424_q_d_b16, eq56_e2424_q_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq56_reactive_node_derivatives: [f64; 17] = [eq56_e2426_q_d_n0, eq56_e2426_q_d_n1, eq56_e2426_q_d_n2, eq56_e2426_q_d_n3, eq56_e2426_q_d_n4, eq56_e2426_q_d_n5, eq56_e2426_q_d_n6, eq56_e2426_q_d_n7, eq56_e2426_q_d_n8, eq56_e2426_q_d_n9, eq56_e2426_q_d_n10, eq56_e2426_q_d_n11, eq56_e2426_q_d_n12, eq56_e2426_q_d_n13, eq56_e2426_q_d_n14, eq56_e2426_q_d_n15, eq56_e2426_q_d_n16];
        let eq56_reactive_branch_derivatives: [f64; 18] = [eq56_e2426_q_d_b0, eq56_e2426_q_d_b1, eq56_e2426_q_d_b2, eq56_e2426_q_d_b3, eq56_e2426_q_d_b4, eq56_e2426_q_d_b5, eq56_e2426_q_d_b6, eq56_e2426_q_d_b7, eq56_e2426_q_d_b8, eq56_e2426_q_d_b9, eq56_e2426_q_d_b10, eq56_e2426_q_d_b11, eq56_e2426_q_d_b12, eq56_e2426_q_d_b13, eq56_e2426_q_d_b14, eq56_e2426_q_d_b15, eq56_e2426_q_d_b16, eq56_e2426_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[6]),
            nodes,
            &eq56_reactive_node_derivatives,
            branches,
            &eq56_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq69_e2506, eq69_e2506_d_n0, eq69_e2506_d_n1, eq69_e2506_d_n2, eq69_e2506_d_n3, eq69_e2506_d_n4, eq69_e2506_d_n5, eq69_e2506_d_n6, eq69_e2506_d_n7, eq69_e2506_d_n8, eq69_e2506_d_n9, eq69_e2506_d_n10, eq69_e2506_d_n11, eq69_e2506_d_n12, eq69_e2506_d_n13, eq69_e2506_d_n14, eq69_e2506_d_n15, eq69_e2506_d_n16, eq69_e2506_d_b0, eq69_e2506_d_b1, eq69_e2506_d_b2, eq69_e2506_d_b3, eq69_e2506_d_b4, eq69_e2506_d_b5, eq69_e2506_d_b6, eq69_e2506_d_b7, eq69_e2506_d_b8, eq69_e2506_d_b9, eq69_e2506_d_b10, eq69_e2506_d_b11, eq69_e2506_d_b12, eq69_e2506_d_b13, eq69_e2506_d_b14, eq69_e2506_d_b15, eq69_e2506_d_b16, eq69_e2506_d_b17, eq69_e2506_q,) = {
    if s.b[1723] {
        let eq69_e2503: f64 = (s.v[138] - s.v[140]);
        let eq69_e2503_d_n0: f64 = (s.dn[138][0] - s.dn[140][0]);
        let eq69_e2503_d_n1: f64 = (s.dn[138][1] - s.dn[140][1]);
        let eq69_e2503_d_n2: f64 = (s.dn[138][2] - s.dn[140][2]);
        let eq69_e2503_d_n3: f64 = (s.dn[138][3] - s.dn[140][3]);
        let eq69_e2503_d_n4: f64 = (s.dn[138][4] - s.dn[140][4]);
        let eq69_e2503_d_n5: f64 = (s.dn[138][5] - s.dn[140][5]);
        let eq69_e2503_d_n6: f64 = (s.dn[138][6] - s.dn[140][6]);
        let eq69_e2503_d_n7: f64 = (s.dn[138][7] - s.dn[140][7]);
        let eq69_e2503_d_n8: f64 = (s.dn[138][8] - s.dn[140][8]);
        let eq69_e2503_d_n9: f64 = (s.dn[138][9] - s.dn[140][9]);
        let eq69_e2503_d_n10: f64 = (s.dn[138][10] - s.dn[140][10]);
        let eq69_e2503_d_n11: f64 = (s.dn[138][11] - s.dn[140][11]);
        let eq69_e2503_d_n12: f64 = (s.dn[138][12] - s.dn[140][12]);
        let eq69_e2503_d_n13: f64 = (s.dn[138][13] - s.dn[140][13]);
        let eq69_e2503_d_n14: f64 = (s.dn[138][14] - s.dn[140][14]);
        let eq69_e2503_d_n15: f64 = (s.dn[138][15] - s.dn[140][15]);
        let eq69_e2503_d_n16: f64 = (s.dn[138][16] - s.dn[140][16]);
        let eq69_e2503_d_b0: f64 = (s.db[138][0] - s.db[140][0]);
        let eq69_e2503_d_b1: f64 = (s.db[138][1] - s.db[140][1]);
        let eq69_e2503_d_b2: f64 = (s.db[138][2] - s.db[140][2]);
        let eq69_e2503_d_b3: f64 = (s.db[138][3] - s.db[140][3]);
        let eq69_e2503_d_b4: f64 = (s.db[138][4] - s.db[140][4]);
        let eq69_e2503_d_b5: f64 = (s.db[138][5] - s.db[140][5]);
        let eq69_e2503_d_b6: f64 = (s.db[138][6] - s.db[140][6]);
        let eq69_e2503_d_b7: f64 = (s.db[138][7] - s.db[140][7]);
        let eq69_e2503_d_b8: f64 = (s.db[138][8] - s.db[140][8]);
        let eq69_e2503_d_b9: f64 = (s.db[138][9] - s.db[140][9]);
        let eq69_e2503_d_b10: f64 = (s.db[138][10] - s.db[140][10]);
        let eq69_e2503_d_b11: f64 = (s.db[138][11] - s.db[140][11]);
        let eq69_e2503_d_b12: f64 = (s.db[138][12] - s.db[140][12]);
        let eq69_e2503_d_b13: f64 = (s.db[138][13] - s.db[140][13]);
        let eq69_e2503_d_b14: f64 = (s.db[138][14] - s.db[140][14]);
        let eq69_e2503_d_b15: f64 = (s.db[138][15] - s.db[140][15]);
        let eq69_e2503_d_b16: f64 = (s.db[138][16] - s.db[140][16]);
        let eq69_e2503_d_b17: f64 = (s.db[138][17] - s.db[140][17]);
        let eq69_e2504_q: f64 = eq69_e2503;
        (eq69_e2503, eq69_e2503_d_n0, eq69_e2503_d_n1, eq69_e2503_d_n2, eq69_e2503_d_n3, eq69_e2503_d_n4, eq69_e2503_d_n5, eq69_e2503_d_n6, eq69_e2503_d_n7, eq69_e2503_d_n8, eq69_e2503_d_n9, eq69_e2503_d_n10, eq69_e2503_d_n11, eq69_e2503_d_n12, eq69_e2503_d_n13, eq69_e2503_d_n14, eq69_e2503_d_n15, eq69_e2503_d_n16, eq69_e2503_d_b0, eq69_e2503_d_b1, eq69_e2503_d_b2, eq69_e2503_d_b3, eq69_e2503_d_b4, eq69_e2503_d_b5, eq69_e2503_d_b6, eq69_e2503_d_b7, eq69_e2503_d_b8, eq69_e2503_d_b9, eq69_e2503_d_b10, eq69_e2503_d_b11, eq69_e2503_d_b12, eq69_e2503_d_b13, eq69_e2503_d_b14, eq69_e2503_d_b15, eq69_e2503_d_b16, eq69_e2503_d_b17, eq69_e2504_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq69_reactive_node_derivatives: [f64; 17] = [eq69_e2506_d_n0, eq69_e2506_d_n1, eq69_e2506_d_n2, eq69_e2506_d_n3, eq69_e2506_d_n4, eq69_e2506_d_n5, eq69_e2506_d_n6, eq69_e2506_d_n7, eq69_e2506_d_n8, eq69_e2506_d_n9, eq69_e2506_d_n10, eq69_e2506_d_n11, eq69_e2506_d_n12, eq69_e2506_d_n13, eq69_e2506_d_n14, eq69_e2506_d_n15, eq69_e2506_d_n16];
        let eq69_reactive_branch_derivatives: [f64; 18] = [eq69_e2506_d_b0, eq69_e2506_d_b1, eq69_e2506_d_b2, eq69_e2506_d_b3, eq69_e2506_d_b4, eq69_e2506_d_b5, eq69_e2506_d_b6, eq69_e2506_d_b7, eq69_e2506_d_b8, eq69_e2506_d_b9, eq69_e2506_d_b10, eq69_e2506_d_b11, eq69_e2506_d_b12, eq69_e2506_d_b13, eq69_e2506_d_b14, eq69_e2506_d_b15, eq69_e2506_d_b16, eq69_e2506_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[15]),
            None,
            nodes,
            &eq69_reactive_node_derivatives,
            branches,
            &eq69_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq71_e2519, eq71_e2519_d_n15, eq71_e2519_q,) = {
    if s.b[1723] {
        let eq71_e2516_q: f64 = (nv15 - 0.0);
        let eq71_e2517: f64 = (1e-9 * (nv15 - 0.0));
        let eq71_e2517_q: f64 = (1e-9 * eq71_e2516_q);
        (eq71_e2517, 1e-9, eq71_e2517_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[15]),
            None,
            nodes[15],
            multiplicity * (eq71_e2519_d_n15),
        );
        let (eq96_e2717, eq96_e2717_d_n0, eq96_e2717_d_n1, eq96_e2717_d_n2, eq96_e2717_d_n3, eq96_e2717_d_n4, eq96_e2717_d_n5, eq96_e2717_d_n6, eq96_e2717_d_n7, eq96_e2717_d_n8, eq96_e2717_d_n9, eq96_e2717_d_n10, eq96_e2717_d_n11, eq96_e2717_d_n12, eq96_e2717_d_n13, eq96_e2717_d_n14, eq96_e2717_d_n15, eq96_e2717_d_n16, eq96_e2717_d_b0, eq96_e2717_d_b1, eq96_e2717_d_b2, eq96_e2717_d_b3, eq96_e2717_d_b4, eq96_e2717_d_b5, eq96_e2717_d_b6, eq96_e2717_d_b7, eq96_e2717_d_b8, eq96_e2717_d_b9, eq96_e2717_d_b10, eq96_e2717_d_b11, eq96_e2717_d_b12, eq96_e2717_d_b13, eq96_e2717_d_b14, eq96_e2717_d_b15, eq96_e2717_d_b16, eq96_e2717_d_b17, eq96_e2717_q,) = {
    if (!s.b[1731]) {
        let eq96_e2712: f64 = (0.7071 * s.v[632]);
        let eq96_e2714: f64 = (eq96_e2712 * (nv16 - 0.0));
        let eq96_e2714_d_n16: f64 = (((0.7071 * s.dn[632][16]) * (nv16 - 0.0)) + eq96_e2712);
        let eq96_e2715_q: f64 = eq96_e2714;
        (eq96_e2714, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, eq96_e2714_d_n16, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, eq96_e2715_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq96_reactive_node_derivatives: [f64; 17] = [eq96_e2717_d_n0, eq96_e2717_d_n1, eq96_e2717_d_n2, eq96_e2717_d_n3, eq96_e2717_d_n4, eq96_e2717_d_n5, eq96_e2717_d_n6, eq96_e2717_d_n7, eq96_e2717_d_n8, eq96_e2717_d_n9, eq96_e2717_d_n10, eq96_e2717_d_n11, eq96_e2717_d_n12, eq96_e2717_d_n13, eq96_e2717_d_n14, eq96_e2717_d_n15, eq96_e2717_d_n16];
        let eq96_reactive_branch_derivatives: [f64; 18] = [eq96_e2717_d_b0, eq96_e2717_d_b1, eq96_e2717_d_b2, eq96_e2717_d_b3, eq96_e2717_d_b4, eq96_e2717_d_b5, eq96_e2717_d_b6, eq96_e2717_d_b7, eq96_e2717_d_b8, eq96_e2717_d_b9, eq96_e2717_d_b10, eq96_e2717_d_b11, eq96_e2717_d_b12, eq96_e2717_d_b13, eq96_e2717_d_b14, eq96_e2717_d_b15, eq96_e2717_d_b16, eq96_e2717_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            nodes,
            &eq96_reactive_node_derivatives,
            branches,
            &eq96_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq97_e2727, eq97_e2727_d_n0, eq97_e2727_d_n1, eq97_e2727_d_n2, eq97_e2727_d_n3, eq97_e2727_d_n4, eq97_e2727_d_n5, eq97_e2727_d_n6, eq97_e2727_d_n7, eq97_e2727_d_n8, eq97_e2727_d_n9, eq97_e2727_d_n10, eq97_e2727_d_n11, eq97_e2727_d_n12, eq97_e2727_d_n13, eq97_e2727_d_n14, eq97_e2727_d_n15, eq97_e2727_d_n16, eq97_e2727_d_b0, eq97_e2727_d_b1, eq97_e2727_d_b2, eq97_e2727_d_b3, eq97_e2727_d_b4, eq97_e2727_d_b5, eq97_e2727_d_b6, eq97_e2727_d_b7, eq97_e2727_d_b8, eq97_e2727_d_b9, eq97_e2727_d_b10, eq97_e2727_d_b11, eq97_e2727_d_b12, eq97_e2727_d_b13, eq97_e2727_d_b14, eq97_e2727_d_b15, eq97_e2727_d_b16, eq97_e2727_d_b17, eq97_e2727_q,) = {
    if (!s.b[1731]) {
        let eq97_e2722: f64 = (0.7071 * s.v[632]);
        let eq97_e2724: f64 = (eq97_e2722 * (nv16 - 0.0));
        let eq97_e2724_d_n16: f64 = (((0.7071 * s.dn[632][16]) * (nv16 - 0.0)) + eq97_e2722);
        let eq97_e2725_q: f64 = eq97_e2724;
        (eq97_e2724, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, eq97_e2724_d_n16, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, eq97_e2725_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq97_reactive_node_derivatives: [f64; 17] = [eq97_e2727_d_n0, eq97_e2727_d_n1, eq97_e2727_d_n2, eq97_e2727_d_n3, eq97_e2727_d_n4, eq97_e2727_d_n5, eq97_e2727_d_n6, eq97_e2727_d_n7, eq97_e2727_d_n8, eq97_e2727_d_n9, eq97_e2727_d_n10, eq97_e2727_d_n11, eq97_e2727_d_n12, eq97_e2727_d_n13, eq97_e2727_d_n14, eq97_e2727_d_n15, eq97_e2727_d_n16];
        let eq97_reactive_branch_derivatives: [f64; 18] = [eq97_e2727_d_b0, eq97_e2727_d_b1, eq97_e2727_d_b2, eq97_e2727_d_b3, eq97_e2727_d_b4, eq97_e2727_d_b5, eq97_e2727_d_b6, eq97_e2727_d_b7, eq97_e2727_d_b8, eq97_e2727_d_b9, eq97_e2727_d_b10, eq97_e2727_d_b11, eq97_e2727_d_b12, eq97_e2727_d_b13, eq97_e2727_d_b14, eq97_e2727_d_b15, eq97_e2727_d_b16, eq97_e2727_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[5]),
            nodes,
            &eq97_reactive_node_derivatives,
            branches,
            &eq97_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq111_e2904, eq111_e2904_d_n0, eq111_e2904_d_n1, eq111_e2904_d_n2, eq111_e2904_d_n3, eq111_e2904_d_n4, eq111_e2904_d_n5, eq111_e2904_d_n6, eq111_e2904_d_n7, eq111_e2904_d_n8, eq111_e2904_d_n9, eq111_e2904_d_n10, eq111_e2904_d_n11, eq111_e2904_d_n12, eq111_e2904_d_n13, eq111_e2904_d_n14, eq111_e2904_d_n15, eq111_e2904_d_n16, eq111_e2904_d_b0, eq111_e2904_d_b1, eq111_e2904_d_b2, eq111_e2904_d_b3, eq111_e2904_d_b4, eq111_e2904_d_b5, eq111_e2904_d_b6, eq111_e2904_d_b7, eq111_e2904_d_b8, eq111_e2904_d_b9, eq111_e2904_d_b10, eq111_e2904_d_b11, eq111_e2904_d_b12, eq111_e2904_d_b13, eq111_e2904_d_b14, eq111_e2904_d_b15, eq111_e2904_d_b16, eq111_e2904_d_b17, eq111_e2904_q,) = {
    if s.b[1736] {
        let eq111_e2901: f64 = ((nv4 - 0.0) * s.v[634]);
        let eq111_e2901_d_n0: f64 = ((nv4 - 0.0) * s.dn[634][0]);
        let eq111_e2901_d_n1: f64 = ((nv4 - 0.0) * s.dn[634][1]);
        let eq111_e2901_d_n2: f64 = ((nv4 - 0.0) * s.dn[634][2]);
        let eq111_e2901_d_n3: f64 = ((nv4 - 0.0) * s.dn[634][3]);
        let eq111_e2901_d_n4: f64 = (s.v[634] + ((nv4 - 0.0) * s.dn[634][4]));
        let eq111_e2901_d_n5: f64 = ((nv4 - 0.0) * s.dn[634][5]);
        let eq111_e2901_d_n6: f64 = ((nv4 - 0.0) * s.dn[634][6]);
        let eq111_e2901_d_n7: f64 = ((nv4 - 0.0) * s.dn[634][7]);
        let eq111_e2901_d_n8: f64 = ((nv4 - 0.0) * s.dn[634][8]);
        let eq111_e2901_d_n9: f64 = ((nv4 - 0.0) * s.dn[634][9]);
        let eq111_e2901_d_n10: f64 = ((nv4 - 0.0) * s.dn[634][10]);
        let eq111_e2901_d_n11: f64 = ((nv4 - 0.0) * s.dn[634][11]);
        let eq111_e2901_d_n12: f64 = ((nv4 - 0.0) * s.dn[634][12]);
        let eq111_e2901_d_n13: f64 = ((nv4 - 0.0) * s.dn[634][13]);
        let eq111_e2901_d_n14: f64 = ((nv4 - 0.0) * s.dn[634][14]);
        let eq111_e2901_d_n15: f64 = ((nv4 - 0.0) * s.dn[634][15]);
        let eq111_e2901_d_n16: f64 = ((nv4 - 0.0) * s.dn[634][16]);
        let eq111_e2901_d_b0: f64 = ((nv4 - 0.0) * s.db[634][0]);
        let eq111_e2901_d_b1: f64 = ((nv4 - 0.0) * s.db[634][1]);
        let eq111_e2901_d_b2: f64 = ((nv4 - 0.0) * s.db[634][2]);
        let eq111_e2901_d_b3: f64 = ((nv4 - 0.0) * s.db[634][3]);
        let eq111_e2901_d_b4: f64 = ((nv4 - 0.0) * s.db[634][4]);
        let eq111_e2901_d_b5: f64 = ((nv4 - 0.0) * s.db[634][5]);
        let eq111_e2901_d_b6: f64 = ((nv4 - 0.0) * s.db[634][6]);
        let eq111_e2901_d_b7: f64 = ((nv4 - 0.0) * s.db[634][7]);
        let eq111_e2901_d_b8: f64 = ((nv4 - 0.0) * s.db[634][8]);
        let eq111_e2901_d_b9: f64 = ((nv4 - 0.0) * s.db[634][9]);
        let eq111_e2901_d_b10: f64 = ((nv4 - 0.0) * s.db[634][10]);
        let eq111_e2901_d_b11: f64 = ((nv4 - 0.0) * s.db[634][11]);
        let eq111_e2901_d_b12: f64 = ((nv4 - 0.0) * s.db[634][12]);
        let eq111_e2901_d_b13: f64 = ((nv4 - 0.0) * s.db[634][13]);
        let eq111_e2901_d_b14: f64 = ((nv4 - 0.0) * s.db[634][14]);
        let eq111_e2901_d_b15: f64 = ((nv4 - 0.0) * s.db[634][15]);
        let eq111_e2901_d_b16: f64 = ((nv4 - 0.0) * s.db[634][16]);
        let eq111_e2901_d_b17: f64 = ((nv4 - 0.0) * s.db[634][17]);
        let eq111_e2902_q: f64 = eq111_e2901;
        (eq111_e2901, eq111_e2901_d_n0, eq111_e2901_d_n1, eq111_e2901_d_n2, eq111_e2901_d_n3, eq111_e2901_d_n4, eq111_e2901_d_n5, eq111_e2901_d_n6, eq111_e2901_d_n7, eq111_e2901_d_n8, eq111_e2901_d_n9, eq111_e2901_d_n10, eq111_e2901_d_n11, eq111_e2901_d_n12, eq111_e2901_d_n13, eq111_e2901_d_n14, eq111_e2901_d_n15, eq111_e2901_d_n16, eq111_e2901_d_b0, eq111_e2901_d_b1, eq111_e2901_d_b2, eq111_e2901_d_b3, eq111_e2901_d_b4, eq111_e2901_d_b5, eq111_e2901_d_b6, eq111_e2901_d_b7, eq111_e2901_d_b8, eq111_e2901_d_b9, eq111_e2901_d_b10, eq111_e2901_d_b11, eq111_e2901_d_b12, eq111_e2901_d_b13, eq111_e2901_d_b14, eq111_e2901_d_b15, eq111_e2901_d_b16, eq111_e2901_d_b17, eq111_e2902_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq111_reactive_node_derivatives: [f64; 17] = [eq111_e2904_d_n0, eq111_e2904_d_n1, eq111_e2904_d_n2, eq111_e2904_d_n3, eq111_e2904_d_n4, eq111_e2904_d_n5, eq111_e2904_d_n6, eq111_e2904_d_n7, eq111_e2904_d_n8, eq111_e2904_d_n9, eq111_e2904_d_n10, eq111_e2904_d_n11, eq111_e2904_d_n12, eq111_e2904_d_n13, eq111_e2904_d_n14, eq111_e2904_d_n15, eq111_e2904_d_n16];
        let eq111_reactive_branch_derivatives: [f64; 18] = [eq111_e2904_d_b0, eq111_e2904_d_b1, eq111_e2904_d_b2, eq111_e2904_d_b3, eq111_e2904_d_b4, eq111_e2904_d_b5, eq111_e2904_d_b6, eq111_e2904_d_b7, eq111_e2904_d_b8, eq111_e2904_d_b9, eq111_e2904_d_b10, eq111_e2904_d_b11, eq111_e2904_d_b12, eq111_e2904_d_b13, eq111_e2904_d_b14, eq111_e2904_d_b15, eq111_e2904_d_b16, eq111_e2904_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq111_reactive_node_derivatives,
            branches,
            &eq111_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
