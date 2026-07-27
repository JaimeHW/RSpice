#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_48(
        s: &mut Scratch,
    ) {
        if (s.b[1441] && s.b[1442]) {s.store_scale(1544, 964, 1.6021918e-19);s.store_square(1543, 964);s.store_scale(1500, 964, (1.6021918e-19 * 1.034943e-10));s.store_scale(1542, 622, 1.6021918e-19);s.store_scalar(1539, (1.6021918e-19 * 1.6021918e-19));s.store_scalar(1540, (1.034943e-10 * 1.034943e-10));s.store_square(1541, 965);s.store_div_from_scalar(1545, (2.0 * 1.034943e-10), 1544);s.store_scale(1546, 1544, 1.0 / ((2.0 * 1.034943e-10)));s.store_scale(1547, 1544, (2.0 * 1.034943e-10));s.store_div_from_scalar(1548, (2.0 * 1.034943e-10), 1542);s.store_scale(1549, 1542, 1.0 / ((2.0 * 1.034943e-10)));s.store_div(1534, 964, 622);s.store_div_from_scalar_offset_input(1533, 1.0, 1534, 1.0);s.store_scalar(1550, (1e-12 * 1000.0));s.store_scalar(1551, (1e-10 * 1000.0));s.store_scalar(1459, 0.0);s.store_scalar(1460, 0.0);s.store_scalar(1473, 0.0);s.store_scalar(1474, 0.0);s.store_scalar(1515, 0.0);s.store_scalar(1516, 0.0);s.store_scalar(1495, 0.0);s.store_scalar(1497, 0.0);s.store_scalar(1496, 0.0);s.store_scalar(1498, 0.0);s.store_scalar(1518, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_49(
        s: &mut Scratch,
    ) {
        if (s.b[1441] && s.b[1442]) {s.store_offset(85, 85, ((10.0 * 2.220446049250313e-16) * 10000000.0));s.store_div_scaled_product_by_product_indices(1454, 185, 185, 1.0, 209, 209, 1.0);s.store_mul_mixed_ai(1457, A::div_scaled_value_by_product(s.ad_value(1454), 1.0, s.ad_value(394), s.ad_value(394), 1.0), 1543);s.store_sqrt_mul_ad(1451, A::div_scaled_product(s.ad_value(1545), s.ad_value(622), 1.0, A::add(s.ad_value(622), s.ad_value(964)), 1.0), A::sub(s.ad_value(1461), s.ad_value(1433)));}
        s.b[1557] = (s.v[1451] > s.v[965]);s.store_scalar(1557, if s.b[1557] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1557]) {s.store_scalar(1464, 0.0);s.copy_ad(1445, 965);s.store_scalar(1481, 0.0);s.store_sub_mixed_ia(1462, 1481, A::mul3(s.ad_value(1546), s.ad_value(1445), s.ad_value(1445)));s.store_scalar(1509, 0.0);}
        let (t0,) = {
    if ((s.b[1441] && s.b[1442]) && s.b[1557]) {
        (s.v[1464],)
    } else {
        (s.v[1508],)
    }
};
        s.store_scalar(1508, t0);
        let (t1,) = {
    if ((s.b[1441] && s.b[1442]) && s.b[1557]) {
        (s.v[1462],)
    } else {
        (s.v[1470],)
    }
};
        s.store_scalar(1470, t1);
        let (t2,) = {
    if ((s.b[1441] && s.b[1442]) && s.b[1557]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, t2);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_50(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        let mut t21: usize = 0;
        while {
            let t1f: f64 = (150.0 + 1.0);let t20: f64 = if (((s.b[1441] && s.b[1442]) && s.b[1557]) && (s.v[97] <= t1f)) { 1.0 } else { 0.0 };
            t20 != 0.0
        } {
            t21 += 1;
            if t21 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t21, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1441] && s.b[1442]) && s.b[1557]) {s.store_sqrt_mul_sub_rhs(1445, 1545, 1481, 1462);}
            s.b[1558] = ((s.v[1445] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1558, if s.b[1558] { 1.0 } else { 0.0 });
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) {s.store_offset_sub(781, 1445, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (t17,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t17);
            let (t18,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t18);
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1559] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1559, if s.b[1559] { 1.0 } else { 0.0 });s.b[1560] = (2.0 == 1.0);s.store_scalar(1560, if s.b[1560] { 1.0 } else { 0.0 });
            let (t3,) = {
    if (((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && s.b[1559]) && s.b[1560]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t3);s.b[1561] = (2.0 == 2.0);s.store_scalar(1561, if s.b[1561] { 1.0 } else { 0.0 });
            let (t4,) = {
    if ((((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && s.b[1559]) && (!s.b[1560])) && s.b[1561]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t4);s.b[1562] = (2.0 == 4.0);s.store_scalar(1562, if s.b[1562] { 1.0 } else { 0.0 });
            let (t5,) = {
    if (((((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && s.b[1559]) && (!s.b[1560])) && (!s.b[1561])) && s.b[1562]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t5);s.b[1563] = (2.0 == 8.0);s.store_scalar(1563, if s.b[1563] { 1.0 } else { 0.0 });
            let (t6,) = {
    if ((((((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && s.b[1559]) && (!s.b[1560])) && (!s.b[1561])) && (!s.b[1562])) && s.b[1563]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t6);
            let (t7,) = {
    if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && s.b[1559]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t7);let mut tb: usize = 0;
            while {
                let ta: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && s.b[1559]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                ta != 0.0
            } {
                tb += 1;
                if tb > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", tb, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && s.b[1559]) {s.store_sqrt(726, 726);}
                let (t9,) = {
    if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && s.b[1559]) {
        let t8: f64 = (s.v[719] + 1.0);
        (t8,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t9);
            }
            if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && (!s.b[1559])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(334, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1445, 965, (-1e-8), 780);}
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) {
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && (!s.b[1558])) {
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && (!s.b[1558])) {s.store_scalar(334, 1.0);}
            if ((s.b[1441] && s.b[1442]) && s.b[1557]) {s.store_add_scaled_inputs3_indices(335, 1462, 1.0, 1433, (-1.0), 1461, 1.0);}
            s.b[1564] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1564, if s.b[1564] { 1.0 } else { 0.0 });
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (tc,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tc);
            let (td,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, td);
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1565] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1565, if s.b[1565] { 1.0 } else { 0.0 });s.b[1566] = (2.0 == 1.0);s.store_scalar(1566, if s.b[1566] { 1.0 } else { 0.0 });
            let (te,) = {
    if (((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && s.b[1565]) && s.b[1566]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, te);s.b[1567] = (2.0 == 2.0);s.store_scalar(1567, if s.b[1567] { 1.0 } else { 0.0 });
            let (tf,) = {
    if ((((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && s.b[1565]) && (!s.b[1566])) && s.b[1567]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, tf);s.b[1568] = (2.0 == 4.0);s.store_scalar(1568, if s.b[1568] { 1.0 } else { 0.0 });
            let (t10,) = {
    if (((((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && s.b[1565]) && (!s.b[1566])) && (!s.b[1567])) && s.b[1568]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t10);s.b[1569] = (2.0 == 8.0);s.store_scalar(1569, if s.b[1569] { 1.0 } else { 0.0 });
            let (t11,) = {
    if ((((((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && s.b[1565]) && (!s.b[1566])) && (!s.b[1567])) && (!s.b[1568])) && s.b[1569]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t11);
            let (t12,) = {
    if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && s.b[1565]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t12);let mut t16: usize = 0;
            while {
                let t15: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && s.b[1565]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t15 != 0.0
            } {
                t16 += 1;
                if t16 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t16, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && s.b[1565]) {s.store_sqrt(726, 726);}
                let (t14,) = {
    if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && s.b[1565]) {
        let t13: f64 = (s.v[719] + 1.0);
        (t13,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t14);
            }
            if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && (!s.b[1565])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(341, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) {
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && (!s.b[1564])) {s.copy_ad(336, 335);s.store_scalar(341, 1.0);}
            if ((s.b[1441] && s.b[1442]) && s.b[1557]) {s.store_sqrt_mul(1449, 1548, 336);s.store_mul(1495, 1445, 1544);s.store_mul_div_from_scalar_lhs_ad_indices(1527, (-1.034943e-10), 1445, 334);s.store_mul_scale_offset_indices(1496, 1542, 1449, -1.0, 0.0);s.store_mul_div_from_scalar_lhs_ad_indices(1529, (-1.034943e-10), 1449, 341);s.store_add_mixed_ai(1483, A::add_scaled_product(s.ad_value(1495), 1.0, s.ad_value(185), A::sub(s.ad_value(1464), s.ad_value(1481)), 1.0), 1496);s.copy_ad(1485, 185);s.store_add(1486, 1527, 1529);s.store_add_scaled_product_mixed_iia(1484, 1462, 1.0, 1533, A::sub(A::add_scaled_product(s.ad_value(1433), 1.0, s.ad_value(1534), s.ad_value(1481), 1.0), s.ad_value(1461)), (-1.0));s.store_scalar(1487, 0.0);s.store_scalar(1488, 1.0);s.store_add_scaled_products_indices(1489, 1485, 1488, 1.0, 1487, 1486, (-1.0));s.store_div(1490, 1488, 1489);s.store_div_scaled_inputs_indices(1491, 1486, -1.0, 1489, 1.0);s.store_div_scaled_inputs_indices(1492, 1487, -1.0, 1489, 1.0);s.store_div(1493, 1485, 1489);}
            s.b[1570] = (((((s.v[1490] * s.v[1483]) + (s.v[1491] * s.v[1484]))) as f64).abs() > 0.5);s.store_scalar(1570, if s.b[1570] { 1.0 } else { 0.0 });
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1570]) {s.store_offset(1464, 1464, (-(0.5 * (if (((s.v[1490] * s.v[1483]) + (s.v[1491] * s.v[1484])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1570]) {s.store_offset(1462, 1462, (-(0.5 * (if (((s.v[1492] * s.v[1483]) + (s.v[1493] * s.v[1484])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && (!s.b[1570])) {s.store_sub_mixed_ia(1464, 1464, A::add_scaled_products(s.ad_value(1490), s.ad_value(1483), 1.0, s.ad_value(1491), s.ad_value(1484), 1.0));s.store_sub_mixed_ia(1462, 1462, A::add_scaled_products(s.ad_value(1492), s.ad_value(1483), 1.0, s.ad_value(1493), s.ad_value(1484), 1.0));}
            s.b[1571] = (((((s.v[1464] - s.v[1508])) as f64).abs() <= 1e-12) && ((((s.v[1462] - s.v[1470])) as f64).abs() <= 1e-12));s.store_scalar(1571, if s.b[1571] { 1.0 } else { 0.0 });
            let (t1a,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1571]) {
        let t19: f64 = (150.0 + 1.0);
        (t19,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t1a);
            let (t1b,) = {
    if ((s.b[1441] && s.b[1442]) && s.b[1557]) {
        (s.v[1464],)
    } else {
        (s.v[1508],)
    }
};
            s.store_scalar(1508, t1b);
            let (t1c,) = {
    if ((s.b[1441] && s.b[1442]) && s.b[1557]) {
        (s.v[1462],)
    } else {
        (s.v[1470],)
    }
};
            s.store_scalar(1470, t1c);
            let (t1e,) = {
    if ((s.b[1441] && s.b[1442]) && s.b[1557]) {
        let t1d: f64 = (s.v[97] + 1.0);
        (t1d,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t1e);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_51(
        s: &mut Scratch,
    ) {
        if ((s.b[1441] && s.b[1442]) && s.b[1557]) {s.copy_ad(1511, 1462);s.store_mul(1449, 965, 1534);s.store_add_scaled_inputs3_mixed_aii(1462, A::mul3(s.ad_value(1549), s.ad_value(1449), s.ad_value(1449)), 1.0, 1433, 1.0, 1461, -1.0);s.store_add_scaled_product_indices(1481, 1462, 1.0, 1546, 1541, 1.0);s.copy_ad(1459, 1481);s.copy_ad(1465, 1481);}
        let (t22,) = {
    if ((s.b[1441] && s.b[1442]) && s.b[1557]) {
        (s.v[1481],)
    } else {
        (s.v[1507],)
    }
};
        s.store_scalar(1507, t22);s.b[1572] = (s.v[85] > s.v[1464]);s.store_scalar(1572, if s.b[1572] { 1.0 } else { 0.0 });
        let (t23,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1572]) {
        (1.0,)
    } else {
        (s.v[1477],)
    }
};
        s.store_scalar(1477, t23);s.b[1573] = (s.v[85] > s.v[1507]);s.store_scalar(1573, if s.b[1573] { 1.0 } else { 0.0 });
        let (t24,) = {
    if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && (!s.b[1572])) && s.b[1573]) {
        (3.0,)
    } else {
        (s.v[1477],)
    }
};
        s.store_scalar(1477, t24);
        let (t25,) = {
    if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && (!s.b[1572])) && (!s.b[1573])) {
        (2.0,)
    } else {
        (s.v[1477],)
    }
};
        s.store_scalar(1477, t25);
        if ((s.b[1441] && s.b[1442]) && (!s.b[1557])) {s.store_scalar(1464, 0.0);}
        let (t26,) = {
    if ((s.b[1441] && s.b[1442]) && (!s.b[1557])) {
        (s.v[1464],)
    } else {
        (s.v[1507],)
    }
};
        s.store_scalar(1507, t26);
        if ((s.b[1441] && s.b[1442]) && (!s.b[1557])) {s.store_scalar(1465, 0.0);s.copy_ad(1509, 1464);s.copy_ad(1445, 1451);s.store_mul(1449, 1445, 1534);s.store_add_scaled_inputs3_mixed_aii(1462, A::mul3(s.ad_value(1549), s.ad_value(1449), s.ad_value(1449)), 1.0, 1433, 1.0, 1461, -1.0);s.store_add_mixed_ai(1481, A::mul3(s.ad_value(1546), s.ad_value(1445), s.ad_value(1445)), 1462);s.copy_ad(1511, 1462);}
        s.b[1574] = (s.v[85] > s.v[1464]);s.store_scalar(1574, if s.b[1574] { 1.0 } else { 0.0 });
        let (t27,) = {
    if (((s.b[1441] && s.b[1442]) && (!s.b[1557])) && s.b[1574]) {
        (1.0,)
    } else {
        (s.v[1477],)
    }
};
        s.store_scalar(1477, t27);
        let (t28,) = {
    if (((s.b[1441] && s.b[1442]) && (!s.b[1557])) && (!s.b[1574])) {
        (2.0,)
    } else {
        (s.v[1477],)
    }
};
        s.store_scalar(1477, t28);
        if (s.b[1441] && s.b[1442]) {s.store_mul_add_scaled_inputs3_offset_rhs_indices(335, 1547, 1465, 1.0, 1433, -1.0, 961, 1.0, 0.0);}
        s.b[1575] = (s.v[335] > 0.0);s.store_scalar(1575, if s.b[1575] { 1.0 } else { 0.0 });
        let (t2e,) = {
    if ((s.b[1441] && s.b[1442]) && s.b[1575]) {
        let t29: f64 = (-s.v[961]);let t2a: f64 = (t29 + s.v[1433]);let t2b: f64 = (s.v[335]).sqrt();let t2c: f64 = (t2b / s.v[185]);let t2d: f64 = (t2a - t2c);
        (t2d,)
    } else {
        (s.v[1453],)
    }
};
        s.store_scalar(1453, t2e);
        let (t31,) = {
    if ((s.b[1441] && s.b[1442]) && (!s.b[1575])) {
        let t2f: f64 = (-s.v[961]);let t30: f64 = (t2f + s.v[1433]);
        (t30,)
    } else {
        (s.v[1453],)
    }
};
        s.store_scalar(1453, t31);s.b[1576] = (s.v[85] > s.v[1464]);s.store_scalar(1576, if s.b[1576] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1576]) {s.copy_ad(1462, 1511);s.store_scalar(1481, 0.0);s.store_add_div_lhs(1478, A::ln(A::mul3(s.ad_value(1454), s.ad_value(85), s.ad_value(85))), A::add(s.ad_value(154), A::div_from_scalar(2.0, s.ad_value(85))), 1481);}
        s.b[1577] = (s.v[1478] < (s.v[1509] + s.v[1551]));s.store_scalar(1577, if s.b[1577] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && s.b[1576]) && s.b[1577]) {s.store_add(1478, 1509, 1551);}
        s.b[1578] = (s.v[85] > s.v[1507]);s.store_scalar(1578, if s.b[1578] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && (!s.b[1576])) && s.b[1578]) {s.copy_ad(1478, 1459);}
        s.b[1579] = (s.v[85] > s.v[1453]);s.store_scalar(1579, if s.b[1579] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) {s.store_add_scaled_product_indices(1455, 154, 1.0, 1454, 85, (-2.0));s.store_add_scaled_product_mixed_aii(1456, A::mul3(s.ad_value(1454), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1481, (-1.0));}
        let (t32,) = {
    if ((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) {
        (s.v[1481],)
    } else {
        (s.v[1468],)
    }
};
        s.store_scalar(1468, t32);
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) {s.store_div_scaled_inputs2_mixed_aii(1478, A::sqrt(A::add_scaled_square_product(s.ad_value(1455), 1.0, s.ad_value(1454), s.ad_value(1456), (-4.0))), 0.5, 1455, (-0.5), 1454, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_52(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        s.b[1580] = (s.v[1478] > (s.v[1465] - s.v[1550]));s.store_scalar(1580, if s.b[1580] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1580]) {s.store_sub(1478, 1465, 1550);}
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) {s.store_sqrt_mul_sub_rhs(1447, 1545, 1481, 1478);s.store_sqrt_mul_sub_rhs(1445, 1545, 1481, 1462);}
        s.b[1581] = ((s.v[1447] + s.v[1445]) > s.v[965]);s.store_scalar(1581, if s.b[1581] { 1.0 } else { 0.0 });
        let (t33,) = {
    if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, t33);let mut t3d: usize = 0;
        while {
            let t3b: f64 = (150.0 + 1.0);let t3c: f64 = if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) && (s.v[97] <= t3b)) { 1.0 } else { 0.0 };
            t3c != 0.0
        } {
            t3d += 1;
            if t3d > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t3d, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) {s.store_add_scaled_inputs3_indices(1466, 1447, 1.0, 1445, 1.0, 965, -1.0);s.store_add_ad(1506, A::div_scalar_by_product(1.034943e-10, s.ad_value(1544), s.ad_value(1447), 1.0), A::div(A::mul_sub_from_scalar_rhs(A::div_from_scalar(1.034943e-10, s.ad_value(1544)), 1.0, A::div_scaled_value_offset_denominator(s.ad_value(1534), 1.0, s.ad_value(1534), 1.0, 1.0)), s.ad_value(1445)));}
            s.b[1582] = ((((s.v[1466] / s.v[1506])) as f64).abs() > 0.5);s.store_scalar(1582, if s.b[1582] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) && s.b[1582]) {s.store_offset(1481, 1481, (-(0.5 * (if ((s.v[1466] / s.v[1506]) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) && (!s.b[1582])) {s.store_sub_div_rhs_indices(1481, 1481, 1466, 1506);}
            s.b[1583] = (((s.v[1481] - s.v[1433]) + s.v[1461]) < (10.0 * 2.220446049250313e-16));s.store_scalar(1583, if s.b[1583] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) && s.b[1583]) {s.store_offset_sub(1481, 1433, 1461, (10.0 * 2.220446049250313e-16));}
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) {s.store_add_scaled_product_mixed_aii(1456, A::mul3(s.ad_value(1454), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1481, (-1.0));s.store_add_scaled_square_product_indices(335, 1455, 1.0, 1454, 1456, (-4.0));}
            s.b[1584] = (s.v[335] > 0.0);s.store_scalar(1584, if s.b[1584] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) && s.b[1584]) {s.store_div_scaled_inputs2_sqrt_first(1478, 335, 0.5, 1455, (-0.5), 1454, 1.0);}
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) && (!s.b[1584])) {s.store_div_scaled_inputs_indices(1478, 1455, (-0.5), 1454, 1.0);}
            s.b[1585] = (s.v[1478] > s.v[1465]);s.store_scalar(1585, if s.b[1585] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) && s.b[1585]) {s.copy_ad(1478, 1465);}
            s.b[1586] = (s.v[1478] > s.v[1481]);s.store_scalar(1586, if s.b[1586] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) && s.b[1586]) {s.store_sub(1478, 1481, 1551);}
            let (t35,) = {
    if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) && s.b[1586]) {
        let t34: f64 = (150.0 + 1.0);
        (t34,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t35);
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) {s.store_sqrt_mul_sub_rhs(1447, 1545, 1481, 1478);s.store_div_scaled_inputs2_mixed_aia(1462, A::add_scaled_product(s.ad_value(1433), 1.0, s.ad_value(1534), s.ad_value(1481), 1.0), 1.0, 1461, (-1.0), A::offset(s.ad_value(1534), 1.0), 1.0);s.store_sqrt_mul_sub_rhs(1445, 1545, 1481, 1462);}
            s.b[1587] = ((((s.v[1481] - s.v[1468])) as f64).abs() <= 1e-8);s.store_scalar(1587, if s.b[1587] { 1.0 } else { 0.0 });
            let (t37,) = {
    if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) && s.b[1587]) {
        let t36: f64 = (150.0 + 1.0);
        (t36,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t37);
            let (t38,) = {
    if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) {
        (s.v[1481],)
    } else {
        (s.v[1468],)
    }
};
            s.store_scalar(1468, t38);
            let (t3a,) = {
    if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) {
        let t39: f64 = (s.v[97] + 1.0);
        (t39,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t3a);
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) {s.store_div_mixed_ia(1458, 1457, A::exp(A::mul(s.ad_value(154), s.ad_value(1433))));}
        let (t3e,) = {
    if ((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) {
        (s.v[1481],)
    } else {
        (s.v[1468],)
    }
};
        s.store_scalar(1468, t3e);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_53(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) {s.store_div_ad(1478, A::ln(A::mul3(s.ad_value(1458), s.ad_value(85), s.ad_value(85))), A::sub(A::div_from_scalar(2.0, s.ad_value(85)), s.ad_value(154)));s.store_sqrt_mul_sub_rhs(1447, 1545, 1481, 1478);s.store_sqrt_mul_sub_rhs(1445, 1545, 1481, 1462);}
        s.b[1588] = ((s.v[1447] + s.v[1445]) > s.v[965]);s.store_scalar(1588, if s.b[1588] { 1.0 } else { 0.0 });
        let (t3f,) = {
    if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) && s.b[1588]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, t3f);let mut t47: usize = 0;
        while {
            let t45: f64 = (s.v[421] + 1.0);let t46: f64 = if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) && s.b[1588]) && (s.v[97] <= t45)) { 1.0 } else { 0.0 };
            t46 != 0.0
        } {
            t47 += 1;
            if t47 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t47, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) && s.b[1588]) {s.store_add_scaled_inputs3_indices(1466, 1447, 1.0, 1445, 1.0, 965, -1.0);s.store_add_ad(1506, A::div_scalar_by_product(1.034943e-10, s.ad_value(1544), s.ad_value(1447), 1.0), A::div(A::mul_sub_from_scalar_rhs(A::div_from_scalar(1.034943e-10, s.ad_value(1544)), 1.0, A::div_scaled_value_offset_denominator(s.ad_value(1534), 1.0, s.ad_value(1534), 1.0, 1.0)), s.ad_value(1445)));}
            s.b[1589] = ((((s.v[1466] / s.v[1506])) as f64).abs() > 0.5);s.store_scalar(1589, if s.b[1589] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) && s.b[1588]) && s.b[1589]) {s.store_offset(1481, 1481, (-(0.5 * (if ((s.v[1466] / s.v[1506]) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) && s.b[1588]) && (!s.b[1589])) {s.store_sub_div_rhs_indices(1481, 1481, 1466, 1506);}
            s.b[1590] = (((s.v[1481] - s.v[1433]) + s.v[1461]) < (10.0 * 2.220446049250313e-16));s.store_scalar(1590, if s.b[1590] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) && s.b[1588]) && s.b[1590]) {s.store_offset_sub(1481, 1433, 1461, (10.0 * 2.220446049250313e-16));}
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) && s.b[1588]) {s.store_sqrt_mul_sub_rhs(1447, 1545, 1481, 1478);s.store_div_scaled_inputs2_mixed_aia(1462, A::add_scaled_product(s.ad_value(1433), 1.0, s.ad_value(1534), s.ad_value(1481), 1.0), 1.0, 1461, (-1.0), A::offset(s.ad_value(1534), 1.0), 1.0);s.store_sqrt_mul_sub_rhs(1445, 1545, 1481, 1462);}
            s.b[1591] = ((((s.v[1481] - s.v[1468])) as f64).abs() <= 1e-5);s.store_scalar(1591, if s.b[1591] { 1.0 } else { 0.0 });
            let (t41,) = {
    if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) && s.b[1588]) && s.b[1591]) {
        let t40: f64 = (s.v[421] + 1.0);
        (t40,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t41);
            let (t42,) = {
    if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) && s.b[1588]) {
        (s.v[1481],)
    } else {
        (s.v[1468],)
    }
};
            s.store_scalar(1468, t42);
            let (t44,) = {
    if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) && s.b[1588]) {
        let t43: f64 = (s.v[97] + 1.0);
        (t43,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t44);
        }
        if (s.b[1441] && s.b[1442]) {s.copy_ad(1480, 1481);s.store_scalar(1517, 0.12);}
        let (t48,) = {
    if (s.b[1441] && s.b[1442]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, t48);
        if (s.b[1441] && s.b[1442]) {s.copy_ad(1459, 1478);s.copy_ad(1481, 1480);}
        let (t49,) = {
    if (s.b[1441] && s.b[1442]) {
        (s.v[1459],)
    } else {
        (s.v[1467],)
    }
};
        s.store_scalar(1467, t49);
        let (t4a,) = {
    if (s.b[1441] && s.b[1442]) {
        (s.v[1481],)
    } else {
        (s.v[1468],)
    }
};
        s.store_scalar(1468, t4a);
        let (t4b,) = {
    if (s.b[1441] && s.b[1442]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, t4b);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_54(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        let mut t76: usize = 0;
        while {
            let t74: f64 = (150.0 + 1.0);let t75: f64 = if ((s.b[1441] && s.b[1442]) && (s.v[97] <= t74)) { 1.0 } else { 0.0 };
            t75 != 0.0
        } {
            t76 += 1;
            if t76 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t76, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (s.b[1441] && s.b[1442]) {s.store_mul_sub_mixed_iai(1462, 1533, A::add_scaled_product(s.ad_value(1433), 1.0, s.ad_value(1534), s.ad_value(1481), 1.0), 1461);s.store_mul(1531, 1533, 1534);s.store_sub(335, 1481, 1462);}
            s.b[1592] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1592, if s.b[1592] { 1.0 } else { 0.0 });
            if ((s.b[1441] && s.b[1442]) && s.b[1592]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (t73,) = {
    if ((s.b[1441] && s.b[1442]) && s.b[1592]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t73);
            let (t4e,) = {
    if ((s.b[1441] && s.b[1442]) && s.b[1592]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t4e);
            if ((s.b[1441] && s.b[1442]) && s.b[1592]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1593] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1593, if s.b[1593] { 1.0 } else { 0.0 });s.b[1594] = (2.0 == 1.0);s.store_scalar(1594, if s.b[1594] { 1.0 } else { 0.0 });
            let (t5f,) = {
    if ((((s.b[1441] && s.b[1442]) && s.b[1592]) && s.b[1593]) && s.b[1594]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t5f);s.b[1595] = (2.0 == 2.0);s.store_scalar(1595, if s.b[1595] { 1.0 } else { 0.0 });
            let (t60,) = {
    if (((((s.b[1441] && s.b[1442]) && s.b[1592]) && s.b[1593]) && (!s.b[1594])) && s.b[1595]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t60);s.b[1596] = (2.0 == 4.0);s.store_scalar(1596, if s.b[1596] { 1.0 } else { 0.0 });
            let (t61,) = {
    if ((((((s.b[1441] && s.b[1442]) && s.b[1592]) && s.b[1593]) && (!s.b[1594])) && (!s.b[1595])) && s.b[1596]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t61);s.b[1597] = (2.0 == 8.0);s.store_scalar(1597, if s.b[1597] { 1.0 } else { 0.0 });
            let (t62,) = {
    if (((((((s.b[1441] && s.b[1442]) && s.b[1592]) && s.b[1593]) && (!s.b[1594])) && (!s.b[1595])) && (!s.b[1596])) && s.b[1597]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t62);
            let (t63,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1592]) && s.b[1593]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t63);let mut t67: usize = 0;
            while {
                let t66: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1592]) && s.b[1593]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t66 != 0.0
            } {
                t67 += 1;
                if t67 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t67, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if (((s.b[1441] && s.b[1442]) && s.b[1592]) && s.b[1593]) {s.store_sqrt(726, 726);}
                let (t65,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1592]) && s.b[1593]) {
        let t64: f64 = (s.v[719] + 1.0);
        (t64,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t65);
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1592]) && (!s.b[1593])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1441] && s.b[1442]) && s.b[1592]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if ((s.b[1441] && s.b[1442]) && s.b[1592]) {
            }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1592])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
            if (s.b[1441] && s.b[1442]) {s.store_sqrt_mul(1445, 1545, 336);}
            s.b[1598] = ((s.v[1445] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1598, if s.b[1598] { 1.0 } else { 0.0 });
            if ((s.b[1441] && s.b[1442]) && s.b[1598]) {s.store_offset_sub(781, 1445, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (t68,) = {
    if ((s.b[1441] && s.b[1442]) && s.b[1598]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t68);
            let (t69,) = {
    if ((s.b[1441] && s.b[1442]) && s.b[1598]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t69);
            if ((s.b[1441] && s.b[1442]) && s.b[1598]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1599] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1599, if s.b[1599] { 1.0 } else { 0.0 });s.b[1600] = (2.0 == 1.0);s.store_scalar(1600, if s.b[1600] { 1.0 } else { 0.0 });
            let (t6a,) = {
    if ((((s.b[1441] && s.b[1442]) && s.b[1598]) && s.b[1599]) && s.b[1600]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t6a);s.b[1601] = (2.0 == 2.0);s.store_scalar(1601, if s.b[1601] { 1.0 } else { 0.0 });
            let (t6b,) = {
    if (((((s.b[1441] && s.b[1442]) && s.b[1598]) && s.b[1599]) && (!s.b[1600])) && s.b[1601]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t6b);s.b[1602] = (2.0 == 4.0);s.store_scalar(1602, if s.b[1602] { 1.0 } else { 0.0 });
            let (t6c,) = {
    if ((((((s.b[1441] && s.b[1442]) && s.b[1598]) && s.b[1599]) && (!s.b[1600])) && (!s.b[1601])) && s.b[1602]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t6c);s.b[1603] = (2.0 == 8.0);s.store_scalar(1603, if s.b[1603] { 1.0 } else { 0.0 });
            let (t6d,) = {
    if (((((((s.b[1441] && s.b[1442]) && s.b[1598]) && s.b[1599]) && (!s.b[1600])) && (!s.b[1601])) && (!s.b[1602])) && s.b[1603]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t6d);
            let (t6e,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1598]) && s.b[1599]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t6e);let mut t72: usize = 0;
            while {
                let t71: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1598]) && s.b[1599]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t71 != 0.0
            } {
                t72 += 1;
                if t72 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t72, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if (((s.b[1441] && s.b[1442]) && s.b[1598]) && s.b[1599]) {s.store_sqrt(726, 726);}
                let (t70,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1598]) && s.b[1599]) {
        let t6f: f64 = (s.v[719] + 1.0);
        (t6f,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t70);
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1598]) && (!s.b[1599])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1441] && s.b[1442]) && s.b[1598]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1445, 965, (-1e-8), 780);}
            if ((s.b[1441] && s.b[1442]) && s.b[1598]) {
            }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1598])) {
            }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1598])) {s.store_scalar(337, 1.0);}
            if (s.b[1441] && s.b[1442]) {s.store_sqrt_mul_ad(1449, s.ad_value(1548), A::add_scaled_inputs3(s.ad_value(1462), 1.0, s.ad_value(1433), (-1.0), s.ad_value(1461), 1.0));s.store_mul(1495, 1445, 1544);s.store_mul_ad_product_lhs_mixed_ai(1525, A::div_from_scalar(1.034943e-10, s.ad_value(1445)), 334, 337);s.store_mul_ad_product_lhs_mixed_ai(1527, A::div_from_scalar((-1.034943e-10), s.ad_value(1445)), 334, 337);s.store_mul_scale_offset_indices(1496, 1542, 1449, -1.0, 0.0);s.store_div_from_scalar(1529, (-1.034943e-10), 1449);s.store_scaled_mul(335, 1500, 1541, 8.0);s.store_div_scaled_inputs_product_mixed_aaaii(1518, A::add_scaled_inputs4(A::mul3_scaled_output(s.ad_value(1462), s.ad_value(1462), s.ad_value(1540), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1462), s.ad_value(1540), s.ad_value(1459), 8.0), (-1.0), A::mul3_scaled_output(s.ad_value(1540), s.ad_value(1459), s.ad_value(1459), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1462), s.ad_value(1500), s.ad_value(1541), 4.0), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(1459), s.ad_value(1500), s.ad_value(1541), 4.0), 1.0, A::mul3(s.ad_value(1543), s.ad_value(1539), s.ad_value(1541)), 1541, 1.0, 335, 1.0);s.store_div_mixed_ai(1519, A::add_scaled_products3(s.ad_value(1462), s.ad_value(1540), (-8.0), s.ad_value(1540), s.ad_value(1459), (4.0 * 2.0), s.ad_value(1500), s.ad_value(1541), 4.0), 335);s.store_div_mixed_ai(1520, A::add_scaled_products3(s.ad_value(1462), s.ad_value(1540), (4.0 * 2.0), s.ad_value(1540), s.ad_value(1459), (-8.0), s.ad_value(1500), s.ad_value(1541), 4.0), 335);s.store_mul_sub_rhs(335, 154, 1459, 1481);s.store_exp(336, 335);}
            s.b[1604] = (s.v[1459] >= s.v[1481]);s.store_scalar(1604, if s.b[1604] { 1.0 } else { 0.0 });
            if ((s.b[1441] && s.b[1442]) && s.b[1604]) {s.store_mul_scaled_sqrt_ad_rhs(1473, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.store_mul_add_scaled_product_div_scaled_product(1521, 154, (-1.0), 154, 336, 1.0, 209, 209, 0.5, 1473, 1.0);s.store_neg(1523, 1521);}
            if ((s.b[1441] && s.b[1442]) && (!s.b[1604])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1459), s.ad_value(1433)));s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1481), s.ad_value(1433)));s.store_mul_sqrt_mixed_ia(1473, 209, A::add_scaled_product(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15), 1.0, s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0));s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1473, 1.0);s.store_mul_add_mixed_iaa(1521, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));s.store_mul_add_mixed_iaa(1523, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));}
            s.b[1605] = ((s.v[1518] > (s.v[1509] - s.v[1517])) && (s.v[1517] >= 0.0));s.store_scalar(1605, if s.b[1605] { 1.0 } else { 0.0 });
            if ((s.b[1441] && s.b[1442]) && s.b[1605]) {s.store_add_scaled_inputs3_indices(781, 1518, 1.0, 1509, (-1.0), 1517, 1.0);s.store_square(722, 781);s.store_square(723, 1517);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (t4c,) = {
    if ((s.b[1441] && s.b[1442]) && s.b[1605]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t4c);
            let (t4d,) = {
    if ((s.b[1441] && s.b[1442]) && s.b[1605]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t4d);
            if ((s.b[1441] && s.b[1442]) && s.b[1605]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1606] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1606, if s.b[1606] { 1.0 } else { 0.0 });s.b[1607] = (4.0 == 1.0);s.store_scalar(1607, if s.b[1607] { 1.0 } else { 0.0 });
            let (t4f,) = {
    if ((((s.b[1441] && s.b[1442]) && s.b[1605]) && s.b[1606]) && s.b[1607]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t4f);s.b[1608] = (4.0 == 2.0);s.store_scalar(1608, if s.b[1608] { 1.0 } else { 0.0 });
            let (t50,) = {
    if (((((s.b[1441] && s.b[1442]) && s.b[1605]) && s.b[1606]) && (!s.b[1607])) && s.b[1608]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t50);s.b[1609] = (4.0 == 4.0);s.store_scalar(1609, if s.b[1609] { 1.0 } else { 0.0 });
            let (t51,) = {
    if ((((((s.b[1441] && s.b[1442]) && s.b[1605]) && s.b[1606]) && (!s.b[1607])) && (!s.b[1608])) && s.b[1609]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t51);s.b[1610] = (4.0 == 8.0);s.store_scalar(1610, if s.b[1610] { 1.0 } else { 0.0 });
            let (t52,) = {
    if (((((((s.b[1441] && s.b[1442]) && s.b[1605]) && s.b[1606]) && (!s.b[1607])) && (!s.b[1608])) && (!s.b[1609])) && s.b[1610]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t52);
            let (t53,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1605]) && s.b[1606]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t53);let mut t57: usize = 0;
            while {
                let t56: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1605]) && s.b[1606]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t56 != 0.0
            } {
                t57 += 1;
                if t57 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t57, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if (((s.b[1441] && s.b[1442]) && s.b[1605]) && s.b[1606]) {s.store_sqrt(726, 726);}
                let (t55,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1605]) && s.b[1606]) {
        let t54: f64 = (s.v[719] + 1.0);
        (t54,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t55);
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1605]) && (!s.b[1606])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if ((s.b[1441] && s.b[1442]) && s.b[1605]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1517, 726);s.store_div_scaled_product3_indices(334, 1517, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs3_indices(335, 1509, 1.0, 1517, (-1.0), 780, 1.0);}
            if ((s.b[1441] && s.b[1442]) && s.b[1605]) {
            }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1605])) {s.copy_ad(335, 1518);s.store_scalar(334, 1.0);}
            if (s.b[1441] && s.b[1442]) {s.store_sub(1483, 1481, 335);s.store_mul_scale_offset_indices(1485, 334, 1519, -1.0, 0.0);s.store_sub_from_scalar_ad(1486, 1.0, A::mul3(s.ad_value(1520), s.ad_value(1531), s.ad_value(334)));s.store_add_scaled_inputs3_mixed_aii(1484, A::add_scaled_product(s.ad_value(1473), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1459)), 1.0), 1.0, 1495, 1.0, 1496, 1.0);s.store_sub(1487, 1521, 185);s.store_add_scaled_inputs_products_indices(1488, 1523, 1.0, 1525, 1.0, 1527, 1531, 1.0, 1529, 1531, 1.0);s.store_add_scaled_products_indices(1489, 1485, 1488, 1.0, 1487, 1486, (-1.0));s.store_div(1490, 1488, 1489);s.store_div_scaled_inputs_indices(1491, 1486, -1.0, 1489, 1.0);s.store_div_scaled_inputs_indices(1492, 1487, -1.0, 1489, 1.0);s.store_div(1493, 1485, 1489);}
            s.b[1611] = (((((s.v[1492] * s.v[1483]) + (s.v[1493] * s.v[1484]))) as f64).abs() > 0.5);s.store_scalar(1611, if s.b[1611] { 1.0 } else { 0.0 });
            if ((s.b[1441] && s.b[1442]) && s.b[1611]) {s.store_offset(1459, 1459, (-(0.5 * (if (((s.v[1490] * s.v[1483]) + (s.v[1491] * s.v[1484])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if ((s.b[1441] && s.b[1442]) && s.b[1611]) {s.store_offset(1481, 1481, (-(0.5 * (if (((s.v[1492] * s.v[1483]) + (s.v[1493] * s.v[1484])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if ((s.b[1441] && s.b[1442]) && (!s.b[1611])) {s.store_sub_mixed_ia(1459, 1459, A::add_scaled_products(s.ad_value(1490), s.ad_value(1483), 1.0, s.ad_value(1491), s.ad_value(1484), 1.0));s.store_sub_mixed_ia(1481, 1481, A::add_scaled_products(s.ad_value(1492), s.ad_value(1483), 1.0, s.ad_value(1493), s.ad_value(1484), 1.0));}
            s.b[1612] = (((((s.v[1459] - s.v[1467])) as f64).abs() <= 1e-12) && ((((s.v[1481] - s.v[1468])) as f64).abs() <= 1e-12));s.store_scalar(1612, if s.b[1612] { 1.0 } else { 0.0 });
            let (t59,) = {
    if ((s.b[1441] && s.b[1442]) && s.b[1612]) {
        let t58: f64 = (150.0 + 1.0);
        (t58,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t59);
            let (t5a,) = {
    if ((s.b[1441] && s.b[1442]) && s.b[1612]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, t5a);
            let (t5b,) = {
    if (s.b[1441] && s.b[1442]) {
        (s.v[1459],)
    } else {
        (s.v[1467],)
    }
};
            s.store_scalar(1467, t5b);
            let (t5c,) = {
    if (s.b[1441] && s.b[1442]) {
        (s.v[1481],)
    } else {
        (s.v[1468],)
    }
};
            s.store_scalar(1468, t5c);
            let (t5e,) = {
    if (s.b[1441] && s.b[1442]) {
        let t5d: f64 = (s.v[97] + 1.0);
        (t5d,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t5e);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_55(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        s.b[1614] = ((s.v[1451] > s.v[965]) && (s.v[1477] != 2.0));s.store_scalar(1614, if s.b[1614] { 1.0 } else { 0.0 });s.b[1615] = ((s.v[1481] > (s.v[1459] - 0.02)) && (0.02 >= 0.0));s.store_scalar(1615, if s.b[1615] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) {s.store_offset_sub(781, 1481, 1459, 0.02);s.store_square(722, 781);s.store_scalar(723, (0.02 * 0.02));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t77,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t77);
        let (t78,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t78);
        if (((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1616] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1616, if s.b[1616] { 1.0 } else { 0.0 });s.b[1617] = (2.0 == 1.0);s.store_scalar(1617, if s.b[1617] { 1.0 } else { 0.0 });
        let (t79,) = {
    if (((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && s.b[1616]) && s.b[1617]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t79);s.b[1618] = (2.0 == 2.0);s.store_scalar(1618, if s.b[1618] { 1.0 } else { 0.0 });
        let (t7a,) = {
    if ((((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && s.b[1616]) && (!s.b[1617])) && s.b[1618]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t7a);s.b[1619] = (2.0 == 4.0);s.store_scalar(1619, if s.b[1619] { 1.0 } else { 0.0 });
        let (t7b,) = {
    if (((((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && s.b[1616]) && (!s.b[1617])) && (!s.b[1618])) && s.b[1619]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t7b);s.b[1620] = (2.0 == 8.0);s.store_scalar(1620, if s.b[1620] { 1.0 } else { 0.0 });
        let (t7c,) = {
    if ((((((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && s.b[1616]) && (!s.b[1617])) && (!s.b[1618])) && (!s.b[1619])) && s.b[1620]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t7c);
        let (t7d,) = {
    if ((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && s.b[1616]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t7d);let mut t81: usize = 0;
        while {
            let t80: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && s.b[1616]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t80 != 0.0
        } {
            t81 += 1;
            if t81 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t81, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && s.b[1616]) {s.store_sqrt(726, 726);}
            let (t7f,) = {
    if ((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && s.b[1616]) {
        let t7e: f64 = (s.v[719] + 1.0);
        (t7e,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t7f);
        }
        if ((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && (!s.b[1616])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.02);s.store_div_scaled_product_indices(335, 725, 726, 0.02, 770, 1.0);s.store_add_offset_lhs(1481, 1459, (-0.02), 780);}
        if (((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) {
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1614]) && (!s.b[1615])) {
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1614]) && (!s.b[1615])) {s.store_scalar(335, 1.0);}
        if (s.b[1441] && s.b[1442]) {s.store_mul_sub_mixed_iai(1462, 1533, A::add_scaled_product(s.ad_value(1433), 1.0, s.ad_value(1534), s.ad_value(1481), 1.0), 1461);s.store_mul_sub_rhs(335, 154, 1459, 1481);s.store_exp(336, 335);}
        s.b[1621] = (s.v[1459] >= s.v[1481]);s.store_scalar(1621, if s.b[1621] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1621]) {s.store_mul_scaled_sqrt_ad_rhs(1473, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.copy_ad(1536, 1473);s.store_scalar(1515, 0.0);s.store_scalar(1475, 0.0);s.store_sqrt_mul_sub_rhs(1445, 1545, 1481, 1462);}
        s.b[1622] = ((s.v[1445] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1622, if s.b[1622] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) {s.store_offset_sub(781, 1445, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t82,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t82);
        let (t83,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t83);
        if (((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_56(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        if (((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) {s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1623] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1623, if s.b[1623] { 1.0 } else { 0.0 });s.b[1624] = (2.0 == 1.0);s.store_scalar(1624, if s.b[1624] { 1.0 } else { 0.0 });
        let (t84,) = {
    if (((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && s.b[1623]) && s.b[1624]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t84);s.b[1625] = (2.0 == 2.0);s.store_scalar(1625, if s.b[1625] { 1.0 } else { 0.0 });
        let (t85,) = {
    if ((((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && s.b[1623]) && (!s.b[1624])) && s.b[1625]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t85);s.b[1626] = (2.0 == 4.0);s.store_scalar(1626, if s.b[1626] { 1.0 } else { 0.0 });
        let (t86,) = {
    if (((((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && s.b[1623]) && (!s.b[1624])) && (!s.b[1625])) && s.b[1626]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t86);s.b[1627] = (2.0 == 8.0);s.store_scalar(1627, if s.b[1627] { 1.0 } else { 0.0 });
        let (t87,) = {
    if ((((((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && s.b[1623]) && (!s.b[1624])) && (!s.b[1625])) && (!s.b[1626])) && s.b[1627]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t87);
        let (t88,) = {
    if ((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && s.b[1623]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t88);let mut t8c: usize = 0;
        while {
            let t8b: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && s.b[1623]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t8b != 0.0
        } {
            t8c += 1;
            if t8c > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t8c, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && s.b[1623]) {s.store_sqrt(726, 726);}
            let (t8a,) = {
    if ((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && s.b[1623]) {
        let t89: f64 = (s.v[719] + 1.0);
        (t89,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t8a);
        }
        if ((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && (!s.b[1623])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1445, 965, (-1e-8), 780);}
        if (((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) {
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1621]) && (!s.b[1622])) {
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1621]) && (!s.b[1622])) {s.store_scalar(337, 1.0);}
        if ((s.b[1441] && s.b[1442]) && s.b[1621]) {s.store_sqrt_mul_ad(1449, s.ad_value(1548), A::add_scaled_inputs3(s.ad_value(1462), 1.0, s.ad_value(1433), (-1.0), s.ad_value(1461), 1.0));s.store_mul(1495, 1445, 1544);s.store_mul_scale_offset_indices(1496, 1542, 1449, -1.0, 0.0);}
        if ((s.b[1441] && s.b[1442]) && (!s.b[1621])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1459), s.ad_value(1433)));s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1481), s.ad_value(1433)));s.store_mul_sqrt_mixed_ia(1473, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));}
        s.b[1628] = ((s.v[1451] > s.v[965]) && (s.v[1477] != 2.0));s.store_scalar(1628, if s.b[1628] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1628]) {s.store_scalar(1475, 0.0);s.store_scalar(1515, 0.0);}
        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && (!s.b[1628])) {s.store_mul_sqrt_mixed_ia(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1459), s.ad_value(1433)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1481), s.ad_value(1433)))))), s.ad_value(335)));s.store_add_scaled_product_mixed_iia(1475, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_57(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && (!s.b[1628])) {s.store_mul_sqrt_mixed_ia(1515, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));}
        if ((s.b[1441] && s.b[1442]) && (!s.b[1621])) {s.store_scalar(1536, 0.0);s.store_sub(335, 1481, 1462);}
        s.b[1629] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1629, if s.b[1629] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t8d,) = {
    if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t8d);
        let (t8e,) = {
    if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t8e);
        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1630] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1630, if s.b[1630] { 1.0 } else { 0.0 });s.b[1631] = (2.0 == 1.0);s.store_scalar(1631, if s.b[1631] { 1.0 } else { 0.0 });
        let (t8f,) = {
    if (((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && s.b[1630]) && s.b[1631]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t8f);s.b[1632] = (2.0 == 2.0);s.store_scalar(1632, if s.b[1632] { 1.0 } else { 0.0 });
        let (t90,) = {
    if ((((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && s.b[1630]) && (!s.b[1631])) && s.b[1632]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t90);s.b[1633] = (2.0 == 4.0);s.store_scalar(1633, if s.b[1633] { 1.0 } else { 0.0 });
        let (t91,) = {
    if (((((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && s.b[1630]) && (!s.b[1631])) && (!s.b[1632])) && s.b[1633]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t91);s.b[1634] = (2.0 == 8.0);s.store_scalar(1634, if s.b[1634] { 1.0 } else { 0.0 });
        let (t92,) = {
    if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && s.b[1630]) && (!s.b[1631])) && (!s.b[1632])) && (!s.b[1633])) && s.b[1634]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t92);
        let (t93,) = {
    if ((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && s.b[1630]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t93);let mut t97: usize = 0;
        while {
            let t96: f64 = if (((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && s.b[1630]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t96 != 0.0
        } {
            t97 += 1;
            if t97 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t97, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && s.b[1630]) {s.store_sqrt(726, 726);}
            let (t95,) = {
    if ((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && s.b[1630]) {
        let t94: f64 = (s.v[719] + 1.0);
        (t94,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t95);
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && (!s.b[1630])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) {
        }
        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && (!s.b[1629])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
        if ((s.b[1441] && s.b[1442]) && (!s.b[1621])) {s.store_sqrt_mul(1445, 1545, 336);}
        s.b[1635] = ((s.v[1445] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1635, if s.b[1635] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) {s.store_offset_sub(781, 1445, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t98,) = {
    if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t98);
        let (t99,) = {
    if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t99);
        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1636] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1636, if s.b[1636] { 1.0 } else { 0.0 });s.b[1637] = (2.0 == 1.0);s.store_scalar(1637, if s.b[1637] { 1.0 } else { 0.0 });
        let (t9a,) = {
    if (((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && s.b[1636]) && s.b[1637]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t9a);s.b[1638] = (2.0 == 2.0);s.store_scalar(1638, if s.b[1638] { 1.0 } else { 0.0 });
        let (t9b,) = {
    if ((((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && s.b[1636]) && (!s.b[1637])) && s.b[1638]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t9b);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_58(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        s.b[1639] = (2.0 == 4.0);s.store_scalar(1639, if s.b[1639] { 1.0 } else { 0.0 });
        let (t9c,) = {
    if (((((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && s.b[1636]) && (!s.b[1637])) && (!s.b[1638])) && s.b[1639]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t9c);s.b[1640] = (2.0 == 8.0);s.store_scalar(1640, if s.b[1640] { 1.0 } else { 0.0 });
        let (t9d,) = {
    if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && s.b[1636]) && (!s.b[1637])) && (!s.b[1638])) && (!s.b[1639])) && s.b[1640]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t9d);
        let (t9e,) = {
    if ((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && s.b[1636]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t9e);let mut ta2: usize = 0;
        while {
            let ta1: f64 = if (((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && s.b[1636]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            ta1 != 0.0
        } {
            ta2 += 1;
            if ta2 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", ta2, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && s.b[1636]) {s.store_sqrt(726, 726);}
            let (ta0,) = {
    if ((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && s.b[1636]) {
        let t9f: f64 = (s.v[719] + 1.0);
        (t9f,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, ta0);
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && (!s.b[1636])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1445, 965, (-1e-8), 780);}
        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) {
        }
        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && (!s.b[1635])) {
        }
        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && (!s.b[1635])) {s.store_scalar(337, 1.0);}
        if ((s.b[1441] && s.b[1442]) && (!s.b[1621])) {s.store_sqrt_mul_ad(1449, s.ad_value(1548), A::add_scaled_inputs3(s.ad_value(1462), 1.0, s.ad_value(1433), (-1.0), s.ad_value(1461), 1.0));s.store_mul(1495, 1445, 1544);s.store_mul_scale_offset_indices(1496, 1542, 1449, -1.0, 0.0);}
        if (s.b[1441] && s.b[1442]) {s.store_sub(335, 1481, 1462);}
        s.b[1641] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1641, if s.b[1641] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1641]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (ta3,) = {
    if ((s.b[1441] && s.b[1442]) && s.b[1641]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, ta3);
        let (ta4,) = {
    if ((s.b[1441] && s.b[1442]) && s.b[1641]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, ta4);
        if ((s.b[1441] && s.b[1442]) && s.b[1641]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1642] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1642, if s.b[1642] { 1.0 } else { 0.0 });s.b[1643] = (2.0 == 1.0);s.store_scalar(1643, if s.b[1643] { 1.0 } else { 0.0 });
        let (ta5,) = {
    if ((((s.b[1441] && s.b[1442]) && s.b[1641]) && s.b[1642]) && s.b[1643]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, ta5);s.b[1644] = (2.0 == 2.0);s.store_scalar(1644, if s.b[1644] { 1.0 } else { 0.0 });
        let (ta6,) = {
    if (((((s.b[1441] && s.b[1442]) && s.b[1641]) && s.b[1642]) && (!s.b[1643])) && s.b[1644]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, ta6);s.b[1645] = (2.0 == 4.0);s.store_scalar(1645, if s.b[1645] { 1.0 } else { 0.0 });
        let (ta7,) = {
    if ((((((s.b[1441] && s.b[1442]) && s.b[1641]) && s.b[1642]) && (!s.b[1643])) && (!s.b[1644])) && s.b[1645]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, ta7);s.b[1646] = (2.0 == 8.0);s.store_scalar(1646, if s.b[1646] { 1.0 } else { 0.0 });
        let (ta8,) = {
    if (((((((s.b[1441] && s.b[1442]) && s.b[1641]) && s.b[1642]) && (!s.b[1643])) && (!s.b[1644])) && (!s.b[1645])) && s.b[1646]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, ta8);
        let (ta9,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1641]) && s.b[1642]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, ta9);let mut tad: usize = 0;
        while {
            let tac: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1641]) && s.b[1642]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tac != 0.0
        } {
            tad += 1;
            if tad > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", tad, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && s.b[1442]) && s.b[1641]) && s.b[1642]) {s.store_sqrt(726, 726);}
            let (tab,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1641]) && s.b[1642]) {
        let taa: f64 = (s.v[719] + 1.0);
        (taa,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tab);
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1641]) && (!s.b[1642])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1441] && s.b[1442]) && s.b[1641]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
        if ((s.b[1441] && s.b[1442]) && s.b[1641]) {
        }
        if ((s.b[1441] && s.b[1442]) && (!s.b[1641])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
        if (s.b[1441] && s.b[1442]) {s.store_sqrt_mul(1445, 1545, 336);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_59(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        s.b[1647] = ((s.v[1445] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1647, if s.b[1647] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1647]) {s.store_offset_sub(781, 1445, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tae,) = {
    if ((s.b[1441] && s.b[1442]) && s.b[1647]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tae);
        let (taf,) = {
    if ((s.b[1441] && s.b[1442]) && s.b[1647]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, taf);
        if ((s.b[1441] && s.b[1442]) && s.b[1647]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1648] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1648, if s.b[1648] { 1.0 } else { 0.0 });s.b[1649] = (2.0 == 1.0);s.store_scalar(1649, if s.b[1649] { 1.0 } else { 0.0 });
        let (tb0,) = {
    if ((((s.b[1441] && s.b[1442]) && s.b[1647]) && s.b[1648]) && s.b[1649]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tb0);s.b[1650] = (2.0 == 2.0);s.store_scalar(1650, if s.b[1650] { 1.0 } else { 0.0 });
        let (tb1,) = {
    if (((((s.b[1441] && s.b[1442]) && s.b[1647]) && s.b[1648]) && (!s.b[1649])) && s.b[1650]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tb1);s.b[1651] = (2.0 == 4.0);s.store_scalar(1651, if s.b[1651] { 1.0 } else { 0.0 });
        let (tb2,) = {
    if ((((((s.b[1441] && s.b[1442]) && s.b[1647]) && s.b[1648]) && (!s.b[1649])) && (!s.b[1650])) && s.b[1651]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tb2);s.b[1652] = (2.0 == 8.0);s.store_scalar(1652, if s.b[1652] { 1.0 } else { 0.0 });
        let (tb3,) = {
    if (((((((s.b[1441] && s.b[1442]) && s.b[1647]) && s.b[1648]) && (!s.b[1649])) && (!s.b[1650])) && (!s.b[1651])) && s.b[1652]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tb3);
        let (tb4,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1647]) && s.b[1648]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tb4);let mut tb8: usize = 0;
        while {
            let tb7: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1647]) && s.b[1648]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tb7 != 0.0
        } {
            tb8 += 1;
            if tb8 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", tb8, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && s.b[1442]) && s.b[1647]) && s.b[1648]) {s.store_sqrt(726, 726);}
            let (tb6,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1647]) && s.b[1648]) {
        let tb5: f64 = (s.v[719] + 1.0);
        (tb5,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tb6);
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1647]) && (!s.b[1648])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1441] && s.b[1442]) && s.b[1647]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1445, 965, (-1e-8), 780);}
        if ((s.b[1441] && s.b[1442]) && s.b[1647]) {
        }
        if ((s.b[1441] && s.b[1442]) && (!s.b[1647])) {
        }
        if ((s.b[1441] && s.b[1442]) && (!s.b[1647])) {s.store_scalar(337, 1.0);}
        if (s.b[1441] && s.b[1442]) {s.store_sub(335, 1481, 1459);}
        s.b[1653] = ((s.v[335] < 0.05) && (0.05 >= 0.0));s.store_scalar(1653, if s.b[1653] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1653]) {s.store_sub_from_scalar(781, 0.05, 335);s.store_square(722, 781);s.store_scalar(723, (0.05 * 0.05));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tb9,) = {
    if ((s.b[1441] && s.b[1442]) && s.b[1653]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tb9);
        let (tba,) = {
    if ((s.b[1441] && s.b[1442]) && s.b[1653]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tba);
        if ((s.b[1441] && s.b[1442]) && s.b[1653]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1654] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1654, if s.b[1654] { 1.0 } else { 0.0 });s.b[1655] = (2.0 == 1.0);s.store_scalar(1655, if s.b[1655] { 1.0 } else { 0.0 });
        let (tbb,) = {
    if ((((s.b[1441] && s.b[1442]) && s.b[1653]) && s.b[1654]) && s.b[1655]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tbb);s.b[1656] = (2.0 == 2.0);s.store_scalar(1656, if s.b[1656] { 1.0 } else { 0.0 });
        let (tbc,) = {
    if (((((s.b[1441] && s.b[1442]) && s.b[1653]) && s.b[1654]) && (!s.b[1655])) && s.b[1656]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tbc);s.b[1657] = (2.0 == 4.0);s.store_scalar(1657, if s.b[1657] { 1.0 } else { 0.0 });
        let (tbd,) = {
    if ((((((s.b[1441] && s.b[1442]) && s.b[1653]) && s.b[1654]) && (!s.b[1655])) && (!s.b[1656])) && s.b[1657]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tbd);s.b[1658] = (2.0 == 8.0);s.store_scalar(1658, if s.b[1658] { 1.0 } else { 0.0 });
        let (tbe,) = {
    if (((((((s.b[1441] && s.b[1442]) && s.b[1653]) && s.b[1654]) && (!s.b[1655])) && (!s.b[1656])) && (!s.b[1657])) && s.b[1658]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tbe);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_60(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        let (tbf,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1653]) && s.b[1654]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tbf);let mut tc3: usize = 0;
        while {
            let tc2: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1653]) && s.b[1654]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tc2 != 0.0
        } {
            tc3 += 1;
            if tc3 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", tc3, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && s.b[1442]) && s.b[1653]) && s.b[1654]) {s.store_sqrt(726, 726);}
            let (tc1,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1653]) && s.b[1654]) {
        let tc0: f64 = (s.v[719] + 1.0);
        (tc0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tc1);
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1653]) && (!s.b[1654])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1441] && s.b[1442]) && s.b[1653]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.05);s.store_div_scaled_product_indices(334, 725, 726, 0.05, 770, 1.0);s.store_sub_from_scalar(336, 0.05, 780);}
        if ((s.b[1441] && s.b[1442]) && s.b[1653]) {
        }
        if ((s.b[1441] && s.b[1442]) && (!s.b[1653])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
        if (s.b[1441] && s.b[1442]) {s.store_sqrt_mul(1447, 1545, 336);s.store_add_scaled_inputs3_indices(335, 965, 1.0, 1445, (-1.0), 1447, -1.0);}
        s.b[1659] = ((s.v[335] < (1e-25 + 1e-18)) && (1e-18 >= 0.0));s.store_scalar(1659, if s.b[1659] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1659]) {s.store_sub_from_scalar(781, (1e-25 + 1e-18), 335);s.store_square(722, 781);s.store_scalar(723, (1e-18 * 1e-18));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tc4,) = {
    if ((s.b[1441] && s.b[1442]) && s.b[1659]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tc4);
        let (tc5,) = {
    if ((s.b[1441] && s.b[1442]) && s.b[1659]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc5);
        if ((s.b[1441] && s.b[1442]) && s.b[1659]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1660] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1660, if s.b[1660] { 1.0 } else { 0.0 });s.b[1661] = (2.0 == 1.0);s.store_scalar(1661, if s.b[1661] { 1.0 } else { 0.0 });
        let (tc6,) = {
    if ((((s.b[1441] && s.b[1442]) && s.b[1659]) && s.b[1660]) && s.b[1661]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc6);s.b[1662] = (2.0 == 2.0);s.store_scalar(1662, if s.b[1662] { 1.0 } else { 0.0 });
        let (tc7,) = {
    if (((((s.b[1441] && s.b[1442]) && s.b[1659]) && s.b[1660]) && (!s.b[1661])) && s.b[1662]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc7);s.b[1663] = (2.0 == 4.0);s.store_scalar(1663, if s.b[1663] { 1.0 } else { 0.0 });
        let (tc8,) = {
    if ((((((s.b[1441] && s.b[1442]) && s.b[1659]) && s.b[1660]) && (!s.b[1661])) && (!s.b[1662])) && s.b[1663]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc8);s.b[1664] = (2.0 == 8.0);s.store_scalar(1664, if s.b[1664] { 1.0 } else { 0.0 });
        let (tc9,) = {
    if (((((((s.b[1441] && s.b[1442]) && s.b[1659]) && s.b[1660]) && (!s.b[1661])) && (!s.b[1662])) && (!s.b[1663])) && s.b[1664]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc9);
        let (tca,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1659]) && s.b[1660]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tca);let mut tce: usize = 0;
        while {
            let tcd: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1659]) && s.b[1660]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tcd != 0.0
        } {
            tce += 1;
            if tce > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", tce, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && s.b[1442]) && s.b[1659]) && s.b[1660]) {s.store_sqrt(726, 726);}
            let (tcc,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1659]) && s.b[1660]) {
        let tcb: f64 = (s.v[719] + 1.0);
        (tcb,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tcc);
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1659]) && (!s.b[1660])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1441] && s.b[1442]) && s.b[1659]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-18);s.store_div_scaled_product_indices(334, 725, 726, 1e-18, 770, 1.0);s.store_sub_from_scalar(1499, (1e-25 + 1e-18), 780);}
        if ((s.b[1441] && s.b[1442]) && s.b[1659]) {
        }
        if ((s.b[1441] && s.b[1442]) && (!s.b[1659])) {s.copy_ad(1499, 335);s.store_scalar(334, 1.0);}
        if (s.b[1441] && s.b[1442]) {s.store_mul_scale_offset_indices(1494, 1544, 1499, -1.0, 0.0);}
        s.b[1665] = ((s.v[1451] > s.v[965]) && (s.v[1477] != 2.0));s.store_scalar(1665, if s.b[1665] { 1.0 } else { 0.0 });s.b[1666] = ((s.v[1459] > (s.v[1509] - 0.8)) && (0.8 >= 0.0));s.store_scalar(1666, if s.b[1666] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) {s.store_offset_sub(781, 1459, 1509, 0.8);s.store_square(722, 781);s.store_scalar(723, (0.8 * 0.8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tcf,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tcf);
        let (td0,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, td0);
        if (((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) {s.store_scalar(770, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_61(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        if (((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) {s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1667] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1667, if s.b[1667] { 1.0 } else { 0.0 });s.b[1668] = (2.0 == 1.0);s.store_scalar(1668, if s.b[1668] { 1.0 } else { 0.0 });
        let (td1,) = {
    if (((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && s.b[1667]) && s.b[1668]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, td1);s.b[1669] = (2.0 == 2.0);s.store_scalar(1669, if s.b[1669] { 1.0 } else { 0.0 });
        let (td2,) = {
    if ((((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && s.b[1667]) && (!s.b[1668])) && s.b[1669]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, td2);s.b[1670] = (2.0 == 4.0);s.store_scalar(1670, if s.b[1670] { 1.0 } else { 0.0 });
        let (td3,) = {
    if (((((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && s.b[1667]) && (!s.b[1668])) && (!s.b[1669])) && s.b[1670]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, td3);s.b[1671] = (2.0 == 8.0);s.store_scalar(1671, if s.b[1671] { 1.0 } else { 0.0 });
        let (td4,) = {
    if ((((((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && s.b[1667]) && (!s.b[1668])) && (!s.b[1669])) && (!s.b[1670])) && s.b[1671]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, td4);
        let (td5,) = {
    if ((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && s.b[1667]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, td5);let mut td9: usize = 0;
        while {
            let td8: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && s.b[1667]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            td8 != 0.0
        } {
            td9 += 1;
            if td9 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", td9, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && s.b[1667]) {s.store_sqrt(726, 726);}
            let (td7,) = {
    if ((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && s.b[1667]) {
        let td6: f64 = (s.v[719] + 1.0);
        (td6,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, td7);
        }
        if ((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && (!s.b[1667])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.8);s.store_div_scaled_product_indices(335, 725, 726, 0.8, 770, 1.0);s.store_add_offset_lhs(336, 1509, (-0.8), 780);}
        if (((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) {
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1665]) && (!s.b[1666])) {s.copy_ad(336, 1459);s.store_scalar(335, 1.0);}
        s.b[1672] = ((s.v[1518] > (s.v[1509] - 0.8)) && (0.8 >= 0.0));s.store_scalar(1672, if s.b[1672] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) {s.store_offset_sub(781, 1518, 1509, 0.8);s.store_square(722, 781);s.store_scalar(723, (0.8 * 0.8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tda,) = {
    if (((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tda);
        let (tdb,) = {
    if (((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tdb);
        if (((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1673] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1673, if s.b[1673] { 1.0 } else { 0.0 });s.b[1674] = (2.0 == 1.0);s.store_scalar(1674, if s.b[1674] { 1.0 } else { 0.0 });
        let (tdc,) = {
    if (((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && s.b[1673]) && s.b[1674]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tdc);s.b[1675] = (2.0 == 2.0);s.store_scalar(1675, if s.b[1675] { 1.0 } else { 0.0 });
        let (tdd,) = {
    if ((((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && s.b[1673]) && (!s.b[1674])) && s.b[1675]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tdd);s.b[1676] = (2.0 == 4.0);s.store_scalar(1676, if s.b[1676] { 1.0 } else { 0.0 });
        let (tde,) = {
    if (((((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && s.b[1673]) && (!s.b[1674])) && (!s.b[1675])) && s.b[1676]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tde);s.b[1677] = (2.0 == 8.0);s.store_scalar(1677, if s.b[1677] { 1.0 } else { 0.0 });
        let (tdf,) = {
    if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && s.b[1673]) && (!s.b[1674])) && (!s.b[1675])) && (!s.b[1676])) && s.b[1677]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tdf);
        let (te0,) = {
    if ((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && s.b[1673]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, te0);let mut te4: usize = 0;
        while {
            let te3: f64 = if (((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && s.b[1673]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            te3 != 0.0
        } {
            te4 += 1;
            if te4 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", te4, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && s.b[1673]) {s.store_sqrt(726, 726);}
            let (te2,) = {
    if ((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && s.b[1673]) {
        let te1: f64 = (s.v[719] + 1.0);
        (te1,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, te2);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_62(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && (!s.b[1673])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.8);s.store_div_scaled_product_indices(334, 725, 726, 0.8, 770, 1.0);s.store_add_offset_lhs(336, 1509, (-0.8), 780);}
        if (((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) {
        }
        if (((s.b[1441] && s.b[1442]) && (!s.b[1665])) && (!s.b[1672])) {s.copy_ad(336, 1518);s.store_scalar(334, 1.0);}
        if (s.b[1441] && s.b[1442]) {s.store_mul_ad_affine_product_lhs(1503, s.ad_value(964), A::exp(A::mul(s.ad_value(154), A::sub(s.ad_value(336), s.ad_value(1509)))), (-1.6021918e-19), 0.0, 1445);}
        s.b[1678] = (((s.v[1459] - s.v[1509]) < 0.06) && (0.06 >= 0.0));s.store_scalar(1678, if s.b[1678] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1678]) {s.store_sub_from_scalar_ad(781, 0.06, A::sub(s.ad_value(1459), s.ad_value(1509)));s.store_square(722, 781);s.store_scalar(723, (0.06 * 0.06));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (te5,) = {
    if ((s.b[1441] && s.b[1442]) && s.b[1678]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, te5);
        let (te6,) = {
    if ((s.b[1441] && s.b[1442]) && s.b[1678]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, te6);
        if ((s.b[1441] && s.b[1442]) && s.b[1678]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1679] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1679, if s.b[1679] { 1.0 } else { 0.0 });s.b[1680] = (2.0 == 1.0);s.store_scalar(1680, if s.b[1680] { 1.0 } else { 0.0 });
        let (te7,) = {
    if ((((s.b[1441] && s.b[1442]) && s.b[1678]) && s.b[1679]) && s.b[1680]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, te7);s.b[1681] = (2.0 == 2.0);s.store_scalar(1681, if s.b[1681] { 1.0 } else { 0.0 });
        let (te8,) = {
    if (((((s.b[1441] && s.b[1442]) && s.b[1678]) && s.b[1679]) && (!s.b[1680])) && s.b[1681]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, te8);s.b[1682] = (2.0 == 4.0);s.store_scalar(1682, if s.b[1682] { 1.0 } else { 0.0 });
        let (te9,) = {
    if ((((((s.b[1441] && s.b[1442]) && s.b[1678]) && s.b[1679]) && (!s.b[1680])) && (!s.b[1681])) && s.b[1682]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, te9);s.b[1683] = (2.0 == 8.0);s.store_scalar(1683, if s.b[1683] { 1.0 } else { 0.0 });
        let (tea,) = {
    if (((((((s.b[1441] && s.b[1442]) && s.b[1678]) && s.b[1679]) && (!s.b[1680])) && (!s.b[1681])) && (!s.b[1682])) && s.b[1683]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tea);
        let (teb,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1678]) && s.b[1679]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, teb);let mut tef: usize = 0;
        while {
            let tee: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1678]) && s.b[1679]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tee != 0.0
        } {
            tef += 1;
            if tef > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", tef, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && s.b[1442]) && s.b[1678]) && s.b[1679]) {s.store_sqrt(726, 726);}
            let (ted,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1678]) && s.b[1679]) {
        let tec: f64 = (s.v[719] + 1.0);
        (tec,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, ted);
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1678]) && (!s.b[1679])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1441] && s.b[1442]) && s.b[1678]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.06);s.store_div_scaled_product_indices(334, 725, 726, 0.06, 770, 1.0);s.store_sub_from_scalar(336, 0.06, 780);}
        if ((s.b[1441] && s.b[1442]) && s.b[1678]) {
        }
        if ((s.b[1441] && s.b[1442]) && (!s.b[1678])) {s.store_sub(336, 1459, 1509);s.store_scalar(334, 1.0);}
        if (s.b[1441] && s.b[1442]) {s.store_offset_add_scaled_product_mixed_aii(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, 154, 336, (-1.0), (10.0 * 2.220446049250313e-16));s.store_mul_scaled_sqrt_rhs(1513, 209, -1.0, 338);s.store_sub_scaled_inputs_mixed_ai(338, A::offset(A::exp_scaled_input(s.ad_value(154), 0.1), (-1.0)), 1.0, 154, 0.1);s.store_mul_sqrt_rhs(1538, 209, 338);s.copy_ad(349, 790);}
        s.b[1684] = (s.v[790] > 1e-6);s.store_scalar(1684, if s.b[1684] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1684]) {s.store_div_square_rhs(336, 1500, 185);s.store_add_scaled_inputs3_offset_indices(334, 85, 1.0, 155, (-1.0), 1436, -1.0, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_63(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        if ((s.b[1441] && s.b[1442]) && s.b[1684]) {s.store_offset_mul_ad(338, A::div_from_scalar(2.0, s.ad_value(336)), s.ad_value(334), 1.0);}
        s.b[1685] = ((s.v[338] < 2.0) && (2.0 >= 0.0));s.store_scalar(1685, if s.b[1685] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) {s.store_sub_from_scalar(781, 2.0, 338);s.store_square(722, 781);s.store_scalar(723, (2.0 * 2.0));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tf0,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tf0);
        let (tf1,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tf1);
        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1686] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1686, if s.b[1686] { 1.0 } else { 0.0 });s.b[1687] = (2.0 == 1.0);s.store_scalar(1687, if s.b[1687] { 1.0 } else { 0.0 });
        let (tf2,) = {
    if (((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && s.b[1686]) && s.b[1687]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tf2);s.b[1688] = (2.0 == 2.0);s.store_scalar(1688, if s.b[1688] { 1.0 } else { 0.0 });
        let (tf3,) = {
    if ((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && s.b[1686]) && (!s.b[1687])) && s.b[1688]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tf3);s.b[1689] = (2.0 == 4.0);s.store_scalar(1689, if s.b[1689] { 1.0 } else { 0.0 });
        let (tf4,) = {
    if (((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && s.b[1686]) && (!s.b[1687])) && (!s.b[1688])) && s.b[1689]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tf4);s.b[1690] = (2.0 == 8.0);s.store_scalar(1690, if s.b[1690] { 1.0 } else { 0.0 });
        let (tf5,) = {
    if ((((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && s.b[1686]) && (!s.b[1687])) && (!s.b[1688])) && (!s.b[1689])) && s.b[1690]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tf5);
        let (tf6,) = {
    if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && s.b[1686]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tf6);let mut tfa: usize = 0;
        while {
            let tf9: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && s.b[1686]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tf9 != 0.0
        } {
            tfa += 1;
            if tfa > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", tfa, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && s.b[1686]) {s.store_sqrt(726, 726);}
            let (tf8,) = {
    if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && s.b[1686]) {
        let tf7: f64 = (s.v[719] + 1.0);
        (tf7,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tf8);
        }
        if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && (!s.b[1686])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 2.0);s.store_div_scaled_product_indices(334, 725, 726, 2.0, 770, 1.0);s.store_sub_from_scalar(343, 2.0, 780);}
        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) {
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && (!s.b[1685])) {s.copy_ad(343, 338);s.store_scalar(334, 1.0);}
        if ((s.b[1441] && s.b[1442]) && s.b[1684]) {s.store_offset(343, 343, 1e-25);s.store_sqrt(337, 343);s.store_mul_scale_offset_indices(338, 336, 337, -1.0, 1.0);s.store_add_offset_lhs(344, 85, 2.0, 338);}
        s.b[1691] = ((s.v[344] < (0.3 + 0.2)) && (0.2 >= 0.0));s.store_scalar(1691, if s.b[1691] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) {s.store_sub_from_scalar(781, (0.3 + 0.2), 344);s.store_square(722, 781);s.store_scalar(723, (0.2 * 0.2));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tfb,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tfb);
        let (tfc,) = {
    if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tfc);
        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1692] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1692, if s.b[1692] { 1.0 } else { 0.0 });s.b[1693] = (4.0 == 1.0);s.store_scalar(1693, if s.b[1693] { 1.0 } else { 0.0 });
        let (tfd,) = {
    if (((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) && s.b[1692]) && s.b[1693]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tfd);
    }
}
