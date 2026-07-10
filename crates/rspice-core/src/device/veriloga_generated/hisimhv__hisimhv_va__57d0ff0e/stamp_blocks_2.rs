#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_32(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1434] = (s.v[182] < 0.0);s.store_scalar(1434, if s.b[1434] { 1.0 } else { 0.0 });
        if (s.b[1431] && s.b[1434]) {s.store_scalar(182, 0.0);s.store_scalar(340, 0.0);}
        if s.b[1431] {s.store_mul(182, 182, 334);s.store_offset_sub_from_scalar_ad(781, 1.0, s.ad_value(182), (-0.05));s.store_scalar(782, (4.0 * 0.05));}
        if s.b[1431] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if s.b[1431] {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(343, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(182, 781, (-0.5), 782, (-0.5), 1.0);}
        s.b[1441] = (s.v[792] > s.v[73]);s.store_scalar(1441, if s.b[1441] { 1.0 } else { 0.0 });
        if ((p.p37 != 0.0) && s.b[1441]) {s.store_sub(335, 792, 73);s.store_sub(336, 72, 73);s.store_div(781, 335, 336);s.store_square(782, 781);s.store_mul(783, 782, 781);s.store_square(784, 782);s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));s.store_mul_ad_affine_product_lhs(1436, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);s.store_mul_scale_offset_indices(333, 336, 780, -1.0, 1.0);s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 1436, 1.0);s.store_neg(1436, 1436);s.store_add(1435, 73, 333);s.store_div_from_scalar(337, 1.0, 336);s.store_mul(338, 335, 337);s.store_square(339, 338);s.store_add_scaled_product_mixed_aia(341, A::offset(s.ad_value(338), 1.0), 1.0, 339, A::add(A::offset(s.ad_value(338), 1.0), s.ad_value(339)), 1.0);s.store_div_scaled_inputs_product_mixed_aiiia(1436, A::scale_offset(s.ad_value(338), 2.0, 1.0), 1.0, 339, 3.0, 338, 339, 4.0, A::square(s.ad_value(341)), 1.0);}
        if ((p.p37 != 0.0) && (!s.b[1441])) {s.copy_ad(1435, 792);s.store_scalar(1436, 1.0);}
        if (p.p37 == 0.0) {s.copy_ad(1435, 792);s.store_scalar(1436, 1.0);}
        s.store_scaled_mul(335, 1436, 790, 0.5);s.store_scale(781, 335, (2.0 * 1.0 / (p.p262)));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(1437, p.p262, 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);s.b[1442] = (s.v[1437] < 1e-12);s.store_scalar(1442, if s.b[1442] { 1.0 } else { 0.0 });
        if s.b[1442] {s.store_scalar(1437, 1e-12);}
        s.store_add(1438, 1435, 1437);s.store_add_scaled_inputs(1439, 790, 1.0, 1437, 2.0);s.store_add(1440, 791, 1437);s.store_add_scaled_inputs3_offset_indices(85, 791, 1.0, 120, 1.0, 182, -1.0, (-s.v[160]));let t0: f64 = (s.v[160] - s.v[120]);let t1: f64 = (t0 + s.v[182]);let t2: f64 = (t1 + s.v[1435]);s.store_scalar(86, t2);s.b[1443] = (s.v[963] != 0.0);s.store_scalar(1443, if s.b[1443] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_33(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1444] = (p.p42 == 1.0);s.store_scalar(1444, if s.b[1444] { 1.0 } else { 0.0 });s.b[1445] = (p.p42 == 2.0);s.store_scalar(1445, if s.b[1445] { 1.0 } else { 0.0 });s.b[1446] = (p.p42 == 3.0);s.store_scalar(1446, if s.b[1446] { 1.0 } else { 0.0 });
        if (s.b[1443] && s.b[1444]) {s.copy_ad(1463, 960);s.store_scale(1546, 964, 1.6021918e-19);s.store_square(1545, 964);s.store_scale(1502, 964, (1.6021918e-19 * 1.034943e-10));s.store_scale(1544, 622, 1.6021918e-19);s.store_scalar(1541, (1.6021918e-19 * 1.6021918e-19));s.store_scalar(1542, (1.034943e-10 * 1.034943e-10));s.store_square(1543, 965);s.store_div_from_scalar(1547, (2.0 * 1.034943e-10), 1546);s.store_scale(1548, 1546, 1.0 / ((2.0 * 1.034943e-10)));s.store_scale(1549, 1546, (2.0 * 1.034943e-10));s.store_div_from_scalar(1550, (2.0 * 1.034943e-10), 1544);s.store_scale(1551, 1544, 1.0 / ((2.0 * 1.034943e-10)));s.store_div(1536, 964, 622);s.store_div_from_scalar_offset_input(1535, 1.0, 1536, 1.0);s.store_scalar(1552, (1e-12 * 1000.0));s.store_scalar(1553, (1e-10 * 1000.0));s.store_scalar(1461, 0.0);s.store_scalar(1462, 0.0);s.store_scalar(1475, 0.0);s.store_scalar(1476, 0.0);s.store_scalar(1517, 0.0);s.store_scalar(1518, 0.0);s.store_scalar(1497, 0.0);s.store_scalar(1499, 0.0);s.store_scalar(1498, 0.0);s.store_scalar(1500, 0.0);s.store_scalar(1520, 0.0);s.store_offset(85, 85, ((10.0 * 2.220446049250313e-16) * 10000000.0));s.store_div_scaled_product_by_product_indices(1456, 185, 185, 1.0, 209, 209, 1.0);s.store_mul_mixed_ai(1459, A::div_scaled_value_by_product(s.ad_value(1456), 1.0, s.ad_value(394), s.ad_value(394), 1.0), 1545);s.store_sqrt_mul_ad(1453, A::div_scaled_product(s.ad_value(1547), s.ad_value(622), 1.0, A::add(s.ad_value(622), s.ad_value(964)), 1.0), A::sub(s.ad_value(1463), s.ad_value(1435)));}
        s.b[1559] = (s.v[1453] > s.v[965]);s.store_scalar(1559, if s.b[1559] { 1.0 } else { 0.0 });
        if ((s.b[1443] && s.b[1444]) && s.b[1559]) {s.store_scalar(1466, 0.0);s.copy_ad(1447, 965);s.store_scalar(1483, 0.0);s.store_sub_mixed_ia(1464, 1483, A::mul3(s.ad_value(1548), s.ad_value(1447), s.ad_value(1447)));s.store_scalar(1511, 0.0);}
        let (t3,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1559]) {
        (s.v[1466],)
    } else {
        (s.v[1510],)
    }
};
        s.store_scalar(1510, t3);
        let (t4,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1559]) {
        (s.v[1464],)
    } else {
        (s.v[1472],)
    }
};
        s.store_scalar(1472, t4);
        let (t5,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1559]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, t5);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_34(
        s: &mut Scratch,
    ) {
        let mut t24: usize = 0;
        while {
            let t22: f64 = (150.0 + 1.0);let t23: f64 = if (((s.b[1443] && s.b[1444]) && s.b[1559]) && (s.v[97] <= t22)) { 1.0 } else { 0.0 };
            t23 != 0.0
        } {
            t24 += 1;assert!(t24 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1443] && s.b[1444]) && s.b[1559]) {s.store_sqrt_mul_sub_rhs(1447, 1547, 1483, 1464);}
            s.b[1560] = ((s.v[1447] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1560, if s.b[1560] { 1.0 } else { 0.0 });
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) {s.store_offset_sub(781, 1447, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (t1a,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t1a);
            let (t1b,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t1b);
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1561] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1561, if s.b[1561] { 1.0 } else { 0.0 });s.b[1562] = (2.0 == 1.0);s.store_scalar(1562, if s.b[1562] { 1.0 } else { 0.0 });
            let (t6,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) && s.b[1561]) && s.b[1562]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t6);s.b[1563] = (2.0 == 2.0);s.store_scalar(1563, if s.b[1563] { 1.0 } else { 0.0 });
            let (t7,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) && s.b[1561]) && (!s.b[1562])) && s.b[1563]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t7);s.b[1564] = (2.0 == 4.0);s.store_scalar(1564, if s.b[1564] { 1.0 } else { 0.0 });
            let (t8,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) && s.b[1561]) && (!s.b[1562])) && (!s.b[1563])) && s.b[1564]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t8);s.b[1565] = (2.0 == 8.0);s.store_scalar(1565, if s.b[1565] { 1.0 } else { 0.0 });
            let (t9,) = {
    if ((((((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) && s.b[1561]) && (!s.b[1562])) && (!s.b[1563])) && (!s.b[1564])) && s.b[1565]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t9);
            let (ta,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) && s.b[1561]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, ta);let mut te: usize = 0;
            while {
                let td: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) && s.b[1561]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                td != 0.0
            } {
                te += 1;assert!(te <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) && s.b[1561]) {s.store_sqrt(726, 726);}
                let (tc,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) && s.b[1561]) {
        let tb: f64 = (s.v[719] + 1.0);
        (tb,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, tc);
            }
            if ((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) && (!s.b[1561])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(334, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1447, 965, (-1e-8), 780);}
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) {
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && (!s.b[1560])) {
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && (!s.b[1560])) {s.store_scalar(334, 1.0);}
            if ((s.b[1443] && s.b[1444]) && s.b[1559]) {s.store_add_scaled_inputs3_indices(335, 1464, 1.0, 1435, (-1.0), 1463, 1.0);}
            s.b[1566] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1566, if s.b[1566] { 1.0 } else { 0.0 });
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (tf,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tf);
            let (t10,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t10);
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1567] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1567, if s.b[1567] { 1.0 } else { 0.0 });s.b[1568] = (2.0 == 1.0);s.store_scalar(1568, if s.b[1568] { 1.0 } else { 0.0 });
            let (t11,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) && s.b[1567]) && s.b[1568]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t11);s.b[1569] = (2.0 == 2.0);s.store_scalar(1569, if s.b[1569] { 1.0 } else { 0.0 });
            let (t12,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) && s.b[1567]) && (!s.b[1568])) && s.b[1569]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t12);s.b[1570] = (2.0 == 4.0);s.store_scalar(1570, if s.b[1570] { 1.0 } else { 0.0 });
            let (t13,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) && s.b[1567]) && (!s.b[1568])) && (!s.b[1569])) && s.b[1570]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t13);s.b[1571] = (2.0 == 8.0);s.store_scalar(1571, if s.b[1571] { 1.0 } else { 0.0 });
            let (t14,) = {
    if ((((((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) && s.b[1567]) && (!s.b[1568])) && (!s.b[1569])) && (!s.b[1570])) && s.b[1571]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t14);
            let (t15,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) && s.b[1567]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t15);let mut t19: usize = 0;
            while {
                let t18: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) && s.b[1567]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t18 != 0.0
            } {
                t19 += 1;assert!(t19 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) && s.b[1567]) {s.store_sqrt(726, 726);}
                let (t17,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) && s.b[1567]) {
        let t16: f64 = (s.v[719] + 1.0);
        (t16,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t17);
            }
            if ((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) && (!s.b[1567])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(341, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) {
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && (!s.b[1566])) {s.copy_ad(336, 335);s.store_scalar(341, 1.0);}
            if ((s.b[1443] && s.b[1444]) && s.b[1559]) {s.store_sqrt_mul(1451, 1550, 336);s.store_mul(1497, 1447, 1546);s.store_mul_div_from_scalar_lhs_ad_indices(1529, (-1.034943e-10), 1447, 334);s.store_mul_scale_offset_indices(1498, 1544, 1451, -1.0, 0.0);s.store_mul_div_from_scalar_lhs_ad_indices(1531, (-1.034943e-10), 1451, 341);s.store_add_mixed_ai(1485, A::add_scaled_product(s.ad_value(1497), 1.0, s.ad_value(185), A::sub(s.ad_value(1466), s.ad_value(1483)), 1.0), 1498);s.copy_ad(1487, 185);s.store_add(1488, 1529, 1531);s.store_add_scaled_product_mixed_iia(1486, 1464, 1.0, 1535, A::sub(A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1483), 1.0), s.ad_value(1463)), (-1.0));s.store_scalar(1489, 0.0);s.store_scalar(1490, 1.0);s.store_add_scaled_products_indices(1491, 1487, 1490, 1.0, 1489, 1488, (-1.0));s.store_div(1492, 1490, 1491);s.store_div_scaled_inputs_indices(1493, 1488, -1.0, 1491, 1.0);s.store_div_scaled_inputs_indices(1494, 1489, -1.0, 1491, 1.0);s.store_div(1495, 1487, 1491);}
            s.b[1572] = (((((s.v[1492] * s.v[1485]) + (s.v[1493] * s.v[1486]))) as f64).abs() > 0.5);s.store_scalar(1572, if s.b[1572] { 1.0 } else { 0.0 });
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1572]) {s.store_offset(1466, 1466, (-(0.5 * (if (((s.v[1492] * s.v[1485]) + (s.v[1493] * s.v[1486])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1572]) {s.store_offset(1464, 1464, (-(0.5 * (if (((s.v[1494] * s.v[1485]) + (s.v[1495] * s.v[1486])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && (!s.b[1572])) {s.store_sub_mixed_ia(1466, 1466, A::add_scaled_products(s.ad_value(1492), s.ad_value(1485), 1.0, s.ad_value(1493), s.ad_value(1486), 1.0));s.store_sub_mixed_ia(1464, 1464, A::add_scaled_products(s.ad_value(1494), s.ad_value(1485), 1.0, s.ad_value(1495), s.ad_value(1486), 1.0));}
            s.b[1573] = (((((s.v[1466] - s.v[1510])) as f64).abs() <= 1e-12) && ((((s.v[1464] - s.v[1472])) as f64).abs() <= 1e-12));s.store_scalar(1573, if s.b[1573] { 1.0 } else { 0.0 });
            let (t1d,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1573]) {
        let t1c: f64 = (150.0 + 1.0);
        (t1c,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t1d);
            let (t1e,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1559]) {
        (s.v[1466],)
    } else {
        (s.v[1510],)
    }
};
            s.store_scalar(1510, t1e);
            let (t1f,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1559]) {
        (s.v[1464],)
    } else {
        (s.v[1472],)
    }
};
            s.store_scalar(1472, t1f);
            let (t21,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1559]) {
        let t20: f64 = (s.v[97] + 1.0);
        (t20,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t21);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_35(
        s: &mut Scratch,
    ) {
        if ((s.b[1443] && s.b[1444]) && s.b[1559]) {s.copy_ad(1513, 1464);s.store_mul(1451, 965, 1536);s.store_add_scaled_inputs3_mixed_aii(1464, A::mul3(s.ad_value(1551), s.ad_value(1451), s.ad_value(1451)), 1.0, 1435, 1.0, 1463, -1.0);s.store_add_scaled_product_indices(1483, 1464, 1.0, 1548, 1543, 1.0);s.copy_ad(1461, 1483);s.copy_ad(1467, 1483);}
        let (t25,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1559]) {
        (s.v[1483],)
    } else {
        (s.v[1509],)
    }
};
        s.store_scalar(1509, t25);s.b[1574] = (s.v[85] > s.v[1466]);s.store_scalar(1574, if s.b[1574] { 1.0 } else { 0.0 });
        let (t26,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1574]) {
        (1.0,)
    } else {
        (s.v[1479],)
    }
};
        s.store_scalar(1479, t26);s.b[1575] = (s.v[85] > s.v[1509]);s.store_scalar(1575, if s.b[1575] { 1.0 } else { 0.0 });
        let (t27,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1559]) && (!s.b[1574])) && s.b[1575]) {
        (3.0,)
    } else {
        (s.v[1479],)
    }
};
        s.store_scalar(1479, t27);
        let (t28,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1559]) && (!s.b[1574])) && (!s.b[1575])) {
        (2.0,)
    } else {
        (s.v[1479],)
    }
};
        s.store_scalar(1479, t28);
        if ((s.b[1443] && s.b[1444]) && (!s.b[1559])) {s.store_scalar(1466, 0.0);}
        let (t29,) = {
    if ((s.b[1443] && s.b[1444]) && (!s.b[1559])) {
        (s.v[1466],)
    } else {
        (s.v[1509],)
    }
};
        s.store_scalar(1509, t29);
        if ((s.b[1443] && s.b[1444]) && (!s.b[1559])) {s.store_scalar(1467, 0.0);s.copy_ad(1511, 1466);s.copy_ad(1447, 1453);s.store_mul(1451, 1447, 1536);s.store_add_scaled_inputs3_mixed_aii(1464, A::mul3(s.ad_value(1551), s.ad_value(1451), s.ad_value(1451)), 1.0, 1435, 1.0, 1463, -1.0);s.store_add_mixed_ai(1483, A::mul3(s.ad_value(1548), s.ad_value(1447), s.ad_value(1447)), 1464);s.copy_ad(1513, 1464);}
        s.b[1576] = (s.v[85] > s.v[1466]);s.store_scalar(1576, if s.b[1576] { 1.0 } else { 0.0 });
        let (t2a,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1559])) && s.b[1576]) {
        (1.0,)
    } else {
        (s.v[1479],)
    }
};
        s.store_scalar(1479, t2a);
        let (t2b,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1559])) && (!s.b[1576])) {
        (2.0,)
    } else {
        (s.v[1479],)
    }
};
        s.store_scalar(1479, t2b);
        if (s.b[1443] && s.b[1444]) {s.store_mul_add_scaled_inputs3_offset_rhs_indices(335, 1549, 1467, 1.0, 1435, -1.0, 961, 1.0, 0.0);}
        s.b[1577] = (s.v[335] > 0.0);s.store_scalar(1577, if s.b[1577] { 1.0 } else { 0.0 });
        let (t31,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1577]) {
        let t2c: f64 = (-s.v[961]);let t2d: f64 = (t2c + s.v[1435]);let t2e: f64 = (s.v[335]).sqrt();let t2f: f64 = (t2e / s.v[185]);let t30: f64 = (t2d - t2f);
        (t30,)
    } else {
        (s.v[1455],)
    }
};
        s.store_scalar(1455, t31);
        let (t34,) = {
    if ((s.b[1443] && s.b[1444]) && (!s.b[1577])) {
        let t32: f64 = (-s.v[961]);let t33: f64 = (t32 + s.v[1435]);
        (t33,)
    } else {
        (s.v[1455],)
    }
};
        s.store_scalar(1455, t34);s.b[1578] = (s.v[85] > s.v[1466]);s.store_scalar(1578, if s.b[1578] { 1.0 } else { 0.0 });
        if ((s.b[1443] && s.b[1444]) && s.b[1578]) {s.copy_ad(1464, 1513);s.store_scalar(1483, 0.0);s.store_add_div_lhs(1480, A::ln(A::mul3(s.ad_value(1456), s.ad_value(85), s.ad_value(85))), A::add(s.ad_value(154), A::div_from_scalar(2.0, s.ad_value(85))), 1483);}
        s.b[1579] = (s.v[1480] < (s.v[1511] + s.v[1553]));s.store_scalar(1579, if s.b[1579] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && s.b[1578]) && s.b[1579]) {s.store_add(1480, 1511, 1553);}
        s.b[1580] = (s.v[85] > s.v[1509]);s.store_scalar(1580, if s.b[1580] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && (!s.b[1578])) && s.b[1580]) {s.copy_ad(1480, 1461);}
        s.b[1581] = (s.v[85] > s.v[1455]);s.store_scalar(1581, if s.b[1581] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) {s.store_add_scaled_product_indices(1457, 154, 1.0, 1456, 85, (-2.0));s.store_add_scaled_product_mixed_aii(1458, A::mul3(s.ad_value(1456), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1483, (-1.0));}
        let (t35,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) {
        (s.v[1483],)
    } else {
        (s.v[1470],)
    }
};
        s.store_scalar(1470, t35);
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) {s.store_div_scaled_inputs2_mixed_aii(1480, A::sqrt(A::add_scaled_square_product(s.ad_value(1457), 1.0, s.ad_value(1456), s.ad_value(1458), (-4.0))), 0.5, 1457, (-0.5), 1456, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_36(
        s: &mut Scratch,
    ) {
        s.b[1582] = (s.v[1480] > (s.v[1467] - s.v[1552]));s.store_scalar(1582, if s.b[1582] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1582]) {s.store_sub(1480, 1467, 1552);}
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) {s.store_sqrt_mul_sub_rhs(1449, 1547, 1483, 1480);s.store_sqrt_mul_sub_rhs(1447, 1547, 1483, 1464);}
        s.b[1583] = ((s.v[1449] + s.v[1447]) > s.v[965]);s.store_scalar(1583, if s.b[1583] { 1.0 } else { 0.0 });
        let (t36,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, t36);let mut t40: usize = 0;
        while {
            let t3e: f64 = (150.0 + 1.0);let t3f: f64 = if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) && (s.v[97] <= t3e)) { 1.0 } else { 0.0 };
            t3f != 0.0
        } {
            t40 += 1;assert!(t40 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) {s.store_add_scaled_inputs3_indices(1468, 1449, 1.0, 1447, 1.0, 965, -1.0);s.store_add_ad(1508, A::div_scalar_by_product(1.034943e-10, s.ad_value(1546), s.ad_value(1449), 1.0), A::div(A::mul_sub_from_scalar_rhs(A::div_from_scalar(1.034943e-10, s.ad_value(1546)), 1.0, A::div_scaled_value_offset_denominator(s.ad_value(1536), 1.0, s.ad_value(1536), 1.0, 1.0)), s.ad_value(1447)));}
            s.b[1584] = ((((s.v[1468] / s.v[1508])) as f64).abs() > 0.5);s.store_scalar(1584, if s.b[1584] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) && s.b[1584]) {s.store_offset(1483, 1483, (-(0.5 * (if ((s.v[1468] / s.v[1508]) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) && (!s.b[1584])) {s.store_sub_div_rhs_indices(1483, 1483, 1468, 1508);}
            s.b[1585] = (((s.v[1483] - s.v[1435]) + s.v[1463]) < (10.0 * 2.220446049250313e-16));s.store_scalar(1585, if s.b[1585] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) && s.b[1585]) {s.store_offset_sub(1483, 1435, 1463, (10.0 * 2.220446049250313e-16));}
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) {s.store_add_scaled_product_mixed_aii(1458, A::mul3(s.ad_value(1456), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1483, (-1.0));s.store_add_scaled_square_product_indices(335, 1457, 1.0, 1456, 1458, (-4.0));}
            s.b[1586] = (s.v[335] > 0.0);s.store_scalar(1586, if s.b[1586] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) && s.b[1586]) {s.store_div_scaled_inputs2_sqrt_first(1480, 335, 0.5, 1457, (-0.5), 1456, 1.0);}
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) && (!s.b[1586])) {s.store_div_scaled_inputs_indices(1480, 1457, (-0.5), 1456, 1.0);}
            s.b[1587] = (s.v[1480] > s.v[1467]);s.store_scalar(1587, if s.b[1587] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) && s.b[1587]) {s.copy_ad(1480, 1467);}
            s.b[1588] = (s.v[1480] > s.v[1483]);s.store_scalar(1588, if s.b[1588] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) && s.b[1588]) {s.store_sub(1480, 1483, 1553);}
            let (t38,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) && s.b[1588]) {
        let t37: f64 = (150.0 + 1.0);
        (t37,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t38);
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) {s.store_sqrt_mul_sub_rhs(1449, 1547, 1483, 1480);s.store_div_scaled_inputs2_mixed_aia(1464, A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1483), 1.0), 1.0, 1463, (-1.0), A::offset(s.ad_value(1536), 1.0), 1.0);s.store_sqrt_mul_sub_rhs(1447, 1547, 1483, 1464);}
            s.b[1589] = ((((s.v[1483] - s.v[1470])) as f64).abs() <= 1e-8);s.store_scalar(1589, if s.b[1589] { 1.0 } else { 0.0 });
            let (t3a,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) && s.b[1589]) {
        let t39: f64 = (150.0 + 1.0);
        (t39,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t3a);
            let (t3b,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) {
        (s.v[1483],)
    } else {
        (s.v[1470],)
    }
};
            s.store_scalar(1470, t3b);
            let (t3d,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) {
        let t3c: f64 = (s.v[97] + 1.0);
        (t3c,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t3d);
        }
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) {s.store_div_mixed_ia(1460, 1459, A::exp(A::mul(s.ad_value(154), s.ad_value(1435))));}
        let (t41,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) {
        (s.v[1483],)
    } else {
        (s.v[1470],)
    }
};
        s.store_scalar(1470, t41);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_37(
        s: &mut Scratch,
    ) {
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) {s.store_div_ad(1480, A::ln(A::mul3(s.ad_value(1460), s.ad_value(85), s.ad_value(85))), A::sub(A::div_from_scalar(2.0, s.ad_value(85)), s.ad_value(154)));s.store_sqrt_mul_sub_rhs(1449, 1547, 1483, 1480);s.store_sqrt_mul_sub_rhs(1447, 1547, 1483, 1464);}
        s.b[1590] = ((s.v[1449] + s.v[1447]) > s.v[965]);s.store_scalar(1590, if s.b[1590] { 1.0 } else { 0.0 });
        let (t42,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, t42);let mut t4a: usize = 0;
        while {
            let t48: f64 = (s.v[421] + 1.0);let t49: f64 = if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) && (s.v[97] <= t48)) { 1.0 } else { 0.0 };
            t49 != 0.0
        } {
            t4a += 1;assert!(t4a <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) {s.store_add_scaled_inputs3_indices(1468, 1449, 1.0, 1447, 1.0, 965, -1.0);s.store_add_ad(1508, A::div_scalar_by_product(1.034943e-10, s.ad_value(1546), s.ad_value(1449), 1.0), A::div(A::mul_sub_from_scalar_rhs(A::div_from_scalar(1.034943e-10, s.ad_value(1546)), 1.0, A::div_scaled_value_offset_denominator(s.ad_value(1536), 1.0, s.ad_value(1536), 1.0, 1.0)), s.ad_value(1447)));}
            s.b[1591] = ((((s.v[1468] / s.v[1508])) as f64).abs() > 0.5);s.store_scalar(1591, if s.b[1591] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) && s.b[1591]) {s.store_offset(1483, 1483, (-(0.5 * (if ((s.v[1468] / s.v[1508]) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) && (!s.b[1591])) {s.store_sub_div_rhs_indices(1483, 1483, 1468, 1508);}
            s.b[1592] = (((s.v[1483] - s.v[1435]) + s.v[1463]) < (10.0 * 2.220446049250313e-16));s.store_scalar(1592, if s.b[1592] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) && s.b[1592]) {s.store_offset_sub(1483, 1435, 1463, (10.0 * 2.220446049250313e-16));}
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) {s.store_sqrt_mul_sub_rhs(1449, 1547, 1483, 1480);s.store_div_scaled_inputs2_mixed_aia(1464, A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1483), 1.0), 1.0, 1463, (-1.0), A::offset(s.ad_value(1536), 1.0), 1.0);s.store_sqrt_mul_sub_rhs(1447, 1547, 1483, 1464);}
            s.b[1593] = ((((s.v[1483] - s.v[1470])) as f64).abs() <= 1e-5);s.store_scalar(1593, if s.b[1593] { 1.0 } else { 0.0 });
            let (t44,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) && s.b[1593]) {
        let t43: f64 = (s.v[421] + 1.0);
        (t43,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t44);
            let (t45,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) {
        (s.v[1483],)
    } else {
        (s.v[1470],)
    }
};
            s.store_scalar(1470, t45);
            let (t47,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) {
        let t46: f64 = (s.v[97] + 1.0);
        (t46,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t47);
        }
        if (s.b[1443] && s.b[1444]) {s.copy_ad(1482, 1483);s.store_scalar(1519, 0.12);}
        let (t4b,) = {
    if (s.b[1443] && s.b[1444]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, t4b);
        if (s.b[1443] && s.b[1444]) {s.copy_ad(1461, 1480);s.copy_ad(1483, 1482);}
        let (t4c,) = {
    if (s.b[1443] && s.b[1444]) {
        (s.v[1461],)
    } else {
        (s.v[1469],)
    }
};
        s.store_scalar(1469, t4c);
        let (t4d,) = {
    if (s.b[1443] && s.b[1444]) {
        (s.v[1483],)
    } else {
        (s.v[1470],)
    }
};
        s.store_scalar(1470, t4d);
        let (t4e,) = {
    if (s.b[1443] && s.b[1444]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, t4e);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_38(
        s: &mut Scratch,
    ) {
        let mut t79: usize = 0;
        while {
            let t77: f64 = (150.0 + 1.0);let t78: f64 = if ((s.b[1443] && s.b[1444]) && (s.v[97] <= t77)) { 1.0 } else { 0.0 };
            t78 != 0.0
        } {
            t79 += 1;assert!(t79 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1443] && s.b[1444]) {s.store_mul_sub_mixed_iai(1464, 1535, A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1483), 1.0), 1463);s.store_mul(1533, 1535, 1536);s.store_sub(335, 1483, 1464);}
            s.b[1594] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1594, if s.b[1594] { 1.0 } else { 0.0 });
            if ((s.b[1443] && s.b[1444]) && s.b[1594]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (t76,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1594]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t76);
            let (t51,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1594]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t51);
            if ((s.b[1443] && s.b[1444]) && s.b[1594]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1595] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1595, if s.b[1595] { 1.0 } else { 0.0 });s.b[1596] = (2.0 == 1.0);s.store_scalar(1596, if s.b[1596] { 1.0 } else { 0.0 });
            let (t62,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && s.b[1596]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t62);s.b[1597] = (2.0 == 2.0);s.store_scalar(1597, if s.b[1597] { 1.0 } else { 0.0 });
            let (t63,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && s.b[1597]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t63);s.b[1598] = (2.0 == 4.0);s.store_scalar(1598, if s.b[1598] { 1.0 } else { 0.0 });
            let (t64,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && (!s.b[1597])) && s.b[1598]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t64);s.b[1599] = (2.0 == 8.0);s.store_scalar(1599, if s.b[1599] { 1.0 } else { 0.0 });
            let (t65,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && (!s.b[1597])) && (!s.b[1598])) && s.b[1599]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t65);
            let (t66,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t66);let mut t6a: usize = 0;
            while {
                let t69: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t69 != 0.0
            } {
                t6a += 1;assert!(t6a <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) {s.store_sqrt(726, 726);}
                let (t68,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) {
        let t67: f64 = (s.v[719] + 1.0);
        (t67,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t68);
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1594]) && (!s.b[1595])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1443] && s.b[1444]) && s.b[1594]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if ((s.b[1443] && s.b[1444]) && s.b[1594]) {
            }
            if ((s.b[1443] && s.b[1444]) && (!s.b[1594])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
            if (s.b[1443] && s.b[1444]) {s.store_sqrt_mul(1447, 1547, 336);}
            s.b[1600] = ((s.v[1447] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1600, if s.b[1600] { 1.0 } else { 0.0 });
            if ((s.b[1443] && s.b[1444]) && s.b[1600]) {s.store_offset_sub(781, 1447, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (t6b,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1600]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t6b);
            let (t6c,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1600]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t6c);
            if ((s.b[1443] && s.b[1444]) && s.b[1600]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1601] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1601, if s.b[1601] { 1.0 } else { 0.0 });s.b[1602] = (2.0 == 1.0);s.store_scalar(1602, if s.b[1602] { 1.0 } else { 0.0 });
            let (t6d,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && s.b[1602]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t6d);s.b[1603] = (2.0 == 2.0);s.store_scalar(1603, if s.b[1603] { 1.0 } else { 0.0 });
            let (t6e,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && (!s.b[1602])) && s.b[1603]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t6e);s.b[1604] = (2.0 == 4.0);s.store_scalar(1604, if s.b[1604] { 1.0 } else { 0.0 });
            let (t6f,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && (!s.b[1602])) && (!s.b[1603])) && s.b[1604]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t6f);s.b[1605] = (2.0 == 8.0);s.store_scalar(1605, if s.b[1605] { 1.0 } else { 0.0 });
            let (t70,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && (!s.b[1602])) && (!s.b[1603])) && (!s.b[1604])) && s.b[1605]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t70);
            let (t71,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t71);let mut t75: usize = 0;
            while {
                let t74: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t74 != 0.0
            } {
                t75 += 1;assert!(t75 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) {s.store_sqrt(726, 726);}
                let (t73,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) {
        let t72: f64 = (s.v[719] + 1.0);
        (t72,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t73);
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1600]) && (!s.b[1601])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1443] && s.b[1444]) && s.b[1600]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1447, 965, (-1e-8), 780);}
            if ((s.b[1443] && s.b[1444]) && s.b[1600]) {
            }
            if ((s.b[1443] && s.b[1444]) && (!s.b[1600])) {
            }
            if ((s.b[1443] && s.b[1444]) && (!s.b[1600])) {s.store_scalar(337, 1.0);}
            if (s.b[1443] && s.b[1444]) {s.store_sqrt_mul_ad(1451, s.ad_value(1550), A::add_scaled_inputs3(s.ad_value(1464), 1.0, s.ad_value(1435), (-1.0), s.ad_value(1463), 1.0));s.store_mul(1497, 1447, 1546);s.store_mul_ad_product_lhs_mixed_ai(1527, A::div_from_scalar(1.034943e-10, s.ad_value(1447)), 334, 337);s.store_mul_ad_product_lhs_mixed_ai(1529, A::div_from_scalar((-1.034943e-10), s.ad_value(1447)), 334, 337);s.store_mul_scale_offset_indices(1498, 1544, 1451, -1.0, 0.0);s.store_div_from_scalar(1531, (-1.034943e-10), 1451);s.store_scaled_mul(335, 1502, 1543, 8.0);s.store_div_scaled_inputs_product_mixed_aaaii(1520, A::add_scaled_inputs4(A::mul3_scaled_output(s.ad_value(1464), s.ad_value(1464), s.ad_value(1542), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1464), s.ad_value(1542), s.ad_value(1461), 8.0), (-1.0), A::mul3_scaled_output(s.ad_value(1542), s.ad_value(1461), s.ad_value(1461), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1464), s.ad_value(1502), s.ad_value(1543), 4.0), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(1461), s.ad_value(1502), s.ad_value(1543), 4.0), 1.0, A::mul3(s.ad_value(1545), s.ad_value(1541), s.ad_value(1543)), 1543, 1.0, 335, 1.0);s.store_div_mixed_ai(1521, A::add_scaled_products3(s.ad_value(1464), s.ad_value(1542), (-8.0), s.ad_value(1542), s.ad_value(1461), (4.0 * 2.0), s.ad_value(1502), s.ad_value(1543), 4.0), 335);s.store_div_mixed_ai(1522, A::add_scaled_products3(s.ad_value(1464), s.ad_value(1542), (4.0 * 2.0), s.ad_value(1542), s.ad_value(1461), (-8.0), s.ad_value(1502), s.ad_value(1543), 4.0), 335);s.store_mul_sub_rhs(335, 154, 1461, 1483);s.store_exp(336, 335);}
            s.b[1606] = (s.v[1461] >= s.v[1483]);s.store_scalar(1606, if s.b[1606] { 1.0 } else { 0.0 });
            if ((s.b[1443] && s.b[1444]) && s.b[1606]) {s.store_mul_scaled_sqrt_ad_rhs(1475, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.store_mul_add_scaled_product_div_scaled_product(1523, 154, (-1.0), 154, 336, 1.0, 209, 209, 0.5, 1475, 1.0);s.store_neg(1525, 1523);}
            if ((s.b[1443] && s.b[1444]) && (!s.b[1606])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1461), s.ad_value(1435)));s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1483), s.ad_value(1435)));s.store_mul_sqrt_mixed_ia(1475, 209, A::add_scaled_product(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15), 1.0, s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0));s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1475, 1.0);s.store_mul_add_mixed_iaa(1523, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));s.store_mul_add_mixed_iaa(1525, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));}
            s.b[1607] = ((s.v[1520] > (s.v[1511] - s.v[1519])) && (s.v[1519] >= 0.0));s.store_scalar(1607, if s.b[1607] { 1.0 } else { 0.0 });
            if ((s.b[1443] && s.b[1444]) && s.b[1607]) {s.store_add_scaled_inputs3_indices(781, 1520, 1.0, 1511, (-1.0), 1519, 1.0);s.store_square(722, 781);s.store_square(723, 1519);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (t4f,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1607]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t4f);
            let (t50,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1607]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t50);
            if ((s.b[1443] && s.b[1444]) && s.b[1607]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1608] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1608, if s.b[1608] { 1.0 } else { 0.0 });s.b[1609] = (4.0 == 1.0);s.store_scalar(1609, if s.b[1609] { 1.0 } else { 0.0 });
            let (t52,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && s.b[1609]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t52);s.b[1610] = (4.0 == 2.0);s.store_scalar(1610, if s.b[1610] { 1.0 } else { 0.0 });
            let (t53,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && (!s.b[1609])) && s.b[1610]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t53);s.b[1611] = (4.0 == 4.0);s.store_scalar(1611, if s.b[1611] { 1.0 } else { 0.0 });
            let (t54,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && (!s.b[1609])) && (!s.b[1610])) && s.b[1611]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t54);s.b[1612] = (4.0 == 8.0);s.store_scalar(1612, if s.b[1612] { 1.0 } else { 0.0 });
            let (t55,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && (!s.b[1609])) && (!s.b[1610])) && (!s.b[1611])) && s.b[1612]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t55);
            let (t56,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t56);let mut t5a: usize = 0;
            while {
                let t59: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t59 != 0.0
            } {
                t5a += 1;assert!(t5a <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) {s.store_sqrt(726, 726);}
                let (t58,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) {
        let t57: f64 = (s.v[719] + 1.0);
        (t57,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t58);
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1607]) && (!s.b[1608])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if ((s.b[1443] && s.b[1444]) && s.b[1607]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1519, 726);s.store_div_scaled_product3_indices(334, 1519, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs3_indices(335, 1511, 1.0, 1519, (-1.0), 780, 1.0);}
            if ((s.b[1443] && s.b[1444]) && s.b[1607]) {
            }
            if ((s.b[1443] && s.b[1444]) && (!s.b[1607])) {s.copy_ad(335, 1520);s.store_scalar(334, 1.0);}
            if (s.b[1443] && s.b[1444]) {s.store_sub(1485, 1483, 335);s.store_mul_scale_offset_indices(1487, 334, 1521, -1.0, 0.0);s.store_sub_from_scalar_ad(1488, 1.0, A::mul3(s.ad_value(1522), s.ad_value(1533), s.ad_value(334)));s.store_add_scaled_inputs3_mixed_aii(1486, A::add_scaled_product(s.ad_value(1475), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1461)), 1.0), 1.0, 1497, 1.0, 1498, 1.0);s.store_sub(1489, 1523, 185);s.store_add_scaled_inputs_products_indices(1490, 1525, 1.0, 1527, 1.0, 1529, 1533, 1.0, 1531, 1533, 1.0);s.store_add_scaled_products_indices(1491, 1487, 1490, 1.0, 1489, 1488, (-1.0));s.store_div(1492, 1490, 1491);s.store_div_scaled_inputs_indices(1493, 1488, -1.0, 1491, 1.0);s.store_div_scaled_inputs_indices(1494, 1489, -1.0, 1491, 1.0);s.store_div(1495, 1487, 1491);}
            s.b[1613] = (((((s.v[1494] * s.v[1485]) + (s.v[1495] * s.v[1486]))) as f64).abs() > 0.5);s.store_scalar(1613, if s.b[1613] { 1.0 } else { 0.0 });
            if ((s.b[1443] && s.b[1444]) && s.b[1613]) {s.store_offset(1461, 1461, (-(0.5 * (if (((s.v[1492] * s.v[1485]) + (s.v[1493] * s.v[1486])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if ((s.b[1443] && s.b[1444]) && s.b[1613]) {s.store_offset(1483, 1483, (-(0.5 * (if (((s.v[1494] * s.v[1485]) + (s.v[1495] * s.v[1486])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if ((s.b[1443] && s.b[1444]) && (!s.b[1613])) {s.store_sub_mixed_ia(1461, 1461, A::add_scaled_products(s.ad_value(1492), s.ad_value(1485), 1.0, s.ad_value(1493), s.ad_value(1486), 1.0));s.store_sub_mixed_ia(1483, 1483, A::add_scaled_products(s.ad_value(1494), s.ad_value(1485), 1.0, s.ad_value(1495), s.ad_value(1486), 1.0));}
            s.b[1614] = (((((s.v[1461] - s.v[1469])) as f64).abs() <= 1e-12) && ((((s.v[1483] - s.v[1470])) as f64).abs() <= 1e-12));s.store_scalar(1614, if s.b[1614] { 1.0 } else { 0.0 });
            let (t5c,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1614]) {
        let t5b: f64 = (150.0 + 1.0);
        (t5b,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t5c);
            let (t5d,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1614]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, t5d);
            let (t5e,) = {
    if (s.b[1443] && s.b[1444]) {
        (s.v[1461],)
    } else {
        (s.v[1469],)
    }
};
            s.store_scalar(1469, t5e);
            let (t5f,) = {
    if (s.b[1443] && s.b[1444]) {
        (s.v[1483],)
    } else {
        (s.v[1470],)
    }
};
            s.store_scalar(1470, t5f);
            let (t61,) = {
    if (s.b[1443] && s.b[1444]) {
        let t60: f64 = (s.v[97] + 1.0);
        (t60,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t61);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_39(
        s: &mut Scratch,
    ) {
        s.b[1616] = ((s.v[1453] > s.v[965]) && (s.v[1479] != 2.0));s.store_scalar(1616, if s.b[1616] { 1.0 } else { 0.0 });s.b[1617] = ((s.v[1483] > (s.v[1461] - 0.02)) && (0.02 >= 0.0));s.store_scalar(1617, if s.b[1617] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) {s.store_offset_sub(781, 1483, 1461, 0.02);s.store_square(722, 781);s.store_scalar(723, (0.02 * 0.02));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t7a,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t7a);
        let (t7b,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t7b);
        if (((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1618] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1618, if s.b[1618] { 1.0 } else { 0.0 });s.b[1619] = (2.0 == 1.0);s.store_scalar(1619, if s.b[1619] { 1.0 } else { 0.0 });
        let (t7c,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && s.b[1619]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t7c);s.b[1620] = (2.0 == 2.0);s.store_scalar(1620, if s.b[1620] { 1.0 } else { 0.0 });
        let (t7d,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1620]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t7d);s.b[1621] = (2.0 == 4.0);s.store_scalar(1621, if s.b[1621] { 1.0 } else { 0.0 });
        let (t7e,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && (!s.b[1620])) && s.b[1621]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t7e);s.b[1622] = (2.0 == 8.0);s.store_scalar(1622, if s.b[1622] { 1.0 } else { 0.0 });
        let (t7f,) = {
    if ((((((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && (!s.b[1620])) && (!s.b[1621])) && s.b[1622]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t7f);
        let (t80,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t80);let mut t84: usize = 0;
        while {
            let t83: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t83 != 0.0
        } {
            t84 += 1;assert!(t84 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) {s.store_sqrt(726, 726);}
            let (t82,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) {
        let t81: f64 = (s.v[719] + 1.0);
        (t81,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t82);
        }
        if ((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && (!s.b[1618])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.02);s.store_div_scaled_product_indices(335, 725, 726, 0.02, 770, 1.0);s.store_add_offset_lhs(1483, 1461, (-0.02), 780);}
        if (((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) {
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1616]) && (!s.b[1617])) {
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1616]) && (!s.b[1617])) {s.store_scalar(335, 1.0);}
        if (s.b[1443] && s.b[1444]) {s.store_mul_sub_mixed_iai(1464, 1535, A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1483), 1.0), 1463);s.store_mul_sub_rhs(335, 154, 1461, 1483);s.store_exp(336, 335);}
        s.b[1623] = (s.v[1461] >= s.v[1483]);s.store_scalar(1623, if s.b[1623] { 1.0 } else { 0.0 });
        if ((s.b[1443] && s.b[1444]) && s.b[1623]) {s.store_mul_scaled_sqrt_ad_rhs(1475, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.copy_ad(1538, 1475);s.store_scalar(1517, 0.0);s.store_scalar(1477, 0.0);s.store_sqrt_mul_sub_rhs(1447, 1547, 1483, 1464);}
        s.b[1624] = ((s.v[1447] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1624, if s.b[1624] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) {s.store_offset_sub(781, 1447, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t85,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t85);
        let (t86,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t86);
        if (((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_40(
        s: &mut Scratch,
    ) {
        if (((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1625] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1625, if s.b[1625] { 1.0 } else { 0.0 });s.b[1626] = (2.0 == 1.0);s.store_scalar(1626, if s.b[1626] { 1.0 } else { 0.0 });
        let (t87,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && s.b[1626]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t87);s.b[1627] = (2.0 == 2.0);s.store_scalar(1627, if s.b[1627] { 1.0 } else { 0.0 });
        let (t88,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && (!s.b[1626])) && s.b[1627]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t88);s.b[1628] = (2.0 == 4.0);s.store_scalar(1628, if s.b[1628] { 1.0 } else { 0.0 });
        let (t89,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && (!s.b[1626])) && (!s.b[1627])) && s.b[1628]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t89);s.b[1629] = (2.0 == 8.0);s.store_scalar(1629, if s.b[1629] { 1.0 } else { 0.0 });
        let (t8a,) = {
    if ((((((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && (!s.b[1626])) && (!s.b[1627])) && (!s.b[1628])) && s.b[1629]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t8a);
        let (t8b,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t8b);let mut t8f: usize = 0;
        while {
            let t8e: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t8e != 0.0
        } {
            t8f += 1;assert!(t8f <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) {s.store_sqrt(726, 726);}
            let (t8d,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) {
        let t8c: f64 = (s.v[719] + 1.0);
        (t8c,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t8d);
        }
        if ((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && (!s.b[1625])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1447, 965, (-1e-8), 780);}
        if (((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) {
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1623]) && (!s.b[1624])) {
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1623]) && (!s.b[1624])) {s.store_scalar(337, 1.0);}
        if ((s.b[1443] && s.b[1444]) && s.b[1623]) {s.store_sqrt_mul_ad(1451, s.ad_value(1550), A::add_scaled_inputs3(s.ad_value(1464), 1.0, s.ad_value(1435), (-1.0), s.ad_value(1463), 1.0));s.store_mul(1497, 1447, 1546);s.store_mul_scale_offset_indices(1498, 1544, 1451, -1.0, 0.0);}
        if ((s.b[1443] && s.b[1444]) && (!s.b[1623])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1461), s.ad_value(1435)));s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1483), s.ad_value(1435)));s.store_mul_sqrt_mixed_ia(1475, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));}
        s.b[1630] = ((s.v[1453] > s.v[965]) && (s.v[1479] != 2.0));s.store_scalar(1630, if s.b[1630] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1630]) {s.store_scalar(1477, 0.0);s.store_scalar(1517, 0.0);}
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && (!s.b[1630])) {s.store_mul_sqrt_mixed_ia(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1461), s.ad_value(1435)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1483), s.ad_value(1435)))))), s.ad_value(335)));s.store_add_scaled_product_mixed_iia(1477, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_41(
        s: &mut Scratch,
    ) {
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && (!s.b[1630])) {s.store_mul_sqrt_mixed_ia(1517, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));}
        if ((s.b[1443] && s.b[1444]) && (!s.b[1623])) {s.store_scalar(1538, 0.0);s.store_sub(335, 1483, 1464);}
        s.b[1631] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1631, if s.b[1631] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t90,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t90);
        let (t91,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t91);
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1632] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1632, if s.b[1632] { 1.0 } else { 0.0 });s.b[1633] = (2.0 == 1.0);s.store_scalar(1633, if s.b[1633] { 1.0 } else { 0.0 });
        let (t92,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && s.b[1633]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t92);s.b[1634] = (2.0 == 2.0);s.store_scalar(1634, if s.b[1634] { 1.0 } else { 0.0 });
        let (t93,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && (!s.b[1633])) && s.b[1634]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t93);s.b[1635] = (2.0 == 4.0);s.store_scalar(1635, if s.b[1635] { 1.0 } else { 0.0 });
        let (t94,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && (!s.b[1633])) && (!s.b[1634])) && s.b[1635]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t94);s.b[1636] = (2.0 == 8.0);s.store_scalar(1636, if s.b[1636] { 1.0 } else { 0.0 });
        let (t95,) = {
    if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && (!s.b[1633])) && (!s.b[1634])) && (!s.b[1635])) && s.b[1636]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t95);
        let (t96,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t96);let mut t9a: usize = 0;
        while {
            let t99: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t99 != 0.0
        } {
            t9a += 1;assert!(t9a <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) {s.store_sqrt(726, 726);}
            let (t98,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) {
        let t97: f64 = (s.v[719] + 1.0);
        (t97,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t98);
        }
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && (!s.b[1632])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) {
        }
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && (!s.b[1631])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
        if ((s.b[1443] && s.b[1444]) && (!s.b[1623])) {s.store_sqrt_mul(1447, 1547, 336);}
        s.b[1637] = ((s.v[1447] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1637, if s.b[1637] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) {s.store_offset_sub(781, 1447, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t9b,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t9b);
        let (t9c,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t9c);
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1638] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1638, if s.b[1638] { 1.0 } else { 0.0 });s.b[1639] = (2.0 == 1.0);s.store_scalar(1639, if s.b[1639] { 1.0 } else { 0.0 });
        let (t9d,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && s.b[1639]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t9d);s.b[1640] = (2.0 == 2.0);s.store_scalar(1640, if s.b[1640] { 1.0 } else { 0.0 });
        let (t9e,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && (!s.b[1639])) && s.b[1640]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t9e);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_42(
        s: &mut Scratch,
    ) {
        s.b[1641] = (2.0 == 4.0);s.store_scalar(1641, if s.b[1641] { 1.0 } else { 0.0 });
        let (t9f,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && (!s.b[1639])) && (!s.b[1640])) && s.b[1641]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t9f);s.b[1642] = (2.0 == 8.0);s.store_scalar(1642, if s.b[1642] { 1.0 } else { 0.0 });
        let (ta0,) = {
    if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && (!s.b[1639])) && (!s.b[1640])) && (!s.b[1641])) && s.b[1642]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, ta0);
        let (ta1,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, ta1);let mut ta5: usize = 0;
        while {
            let ta4: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            ta4 != 0.0
        } {
            ta5 += 1;assert!(ta5 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) {s.store_sqrt(726, 726);}
            let (ta3,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) {
        let ta2: f64 = (s.v[719] + 1.0);
        (ta2,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, ta3);
        }
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && (!s.b[1638])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1447, 965, (-1e-8), 780);}
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) {
        }
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && (!s.b[1637])) {
        }
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && (!s.b[1637])) {s.store_scalar(337, 1.0);}
        if ((s.b[1443] && s.b[1444]) && (!s.b[1623])) {s.store_sqrt_mul_ad(1451, s.ad_value(1550), A::add_scaled_inputs3(s.ad_value(1464), 1.0, s.ad_value(1435), (-1.0), s.ad_value(1463), 1.0));s.store_mul(1497, 1447, 1546);s.store_mul_scale_offset_indices(1498, 1544, 1451, -1.0, 0.0);}
        if (s.b[1443] && s.b[1444]) {s.store_sub(335, 1483, 1464);}
        s.b[1643] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1643, if s.b[1643] { 1.0 } else { 0.0 });
        if ((s.b[1443] && s.b[1444]) && s.b[1643]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (ta6,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1643]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, ta6);
        let (ta7,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1643]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, ta7);
        if ((s.b[1443] && s.b[1444]) && s.b[1643]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1644] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1644, if s.b[1644] { 1.0 } else { 0.0 });s.b[1645] = (2.0 == 1.0);s.store_scalar(1645, if s.b[1645] { 1.0 } else { 0.0 });
        let (ta8,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && s.b[1645]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, ta8);s.b[1646] = (2.0 == 2.0);s.store_scalar(1646, if s.b[1646] { 1.0 } else { 0.0 });
        let (ta9,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && (!s.b[1645])) && s.b[1646]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, ta9);s.b[1647] = (2.0 == 4.0);s.store_scalar(1647, if s.b[1647] { 1.0 } else { 0.0 });
        let (taa,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && (!s.b[1645])) && (!s.b[1646])) && s.b[1647]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, taa);s.b[1648] = (2.0 == 8.0);s.store_scalar(1648, if s.b[1648] { 1.0 } else { 0.0 });
        let (tab,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && (!s.b[1645])) && (!s.b[1646])) && (!s.b[1647])) && s.b[1648]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tab);
        let (tac,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tac);let mut tb0: usize = 0;
        while {
            let taf: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            taf != 0.0
        } {
            tb0 += 1;assert!(tb0 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) {s.store_sqrt(726, 726);}
            let (tae,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) {
        let tad: f64 = (s.v[719] + 1.0);
        (tad,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tae);
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1643]) && (!s.b[1644])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1443] && s.b[1444]) && s.b[1643]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
        if ((s.b[1443] && s.b[1444]) && s.b[1643]) {
        }
        if ((s.b[1443] && s.b[1444]) && (!s.b[1643])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
        if (s.b[1443] && s.b[1444]) {s.store_sqrt_mul(1447, 1547, 336);}
        s.b[1649] = ((s.v[1447] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1649, if s.b[1649] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_43(
        s: &mut Scratch,
    ) {
        if ((s.b[1443] && s.b[1444]) && s.b[1649]) {s.store_offset_sub(781, 1447, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tb1,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1649]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tb1);
        let (tb2,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1649]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tb2);
        if ((s.b[1443] && s.b[1444]) && s.b[1649]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1650] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1650, if s.b[1650] { 1.0 } else { 0.0 });s.b[1651] = (2.0 == 1.0);s.store_scalar(1651, if s.b[1651] { 1.0 } else { 0.0 });
        let (tb3,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && s.b[1651]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tb3);s.b[1652] = (2.0 == 2.0);s.store_scalar(1652, if s.b[1652] { 1.0 } else { 0.0 });
        let (tb4,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && (!s.b[1651])) && s.b[1652]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tb4);s.b[1653] = (2.0 == 4.0);s.store_scalar(1653, if s.b[1653] { 1.0 } else { 0.0 });
        let (tb5,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && (!s.b[1651])) && (!s.b[1652])) && s.b[1653]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tb5);s.b[1654] = (2.0 == 8.0);s.store_scalar(1654, if s.b[1654] { 1.0 } else { 0.0 });
        let (tb6,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && (!s.b[1651])) && (!s.b[1652])) && (!s.b[1653])) && s.b[1654]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tb6);
        let (tb7,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tb7);let mut tbb: usize = 0;
        while {
            let tba: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tba != 0.0
        } {
            tbb += 1;assert!(tbb <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) {s.store_sqrt(726, 726);}
            let (tb9,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) {
        let tb8: f64 = (s.v[719] + 1.0);
        (tb8,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tb9);
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1649]) && (!s.b[1650])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1443] && s.b[1444]) && s.b[1649]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1447, 965, (-1e-8), 780);}
        if ((s.b[1443] && s.b[1444]) && s.b[1649]) {
        }
        if ((s.b[1443] && s.b[1444]) && (!s.b[1649])) {
        }
        if ((s.b[1443] && s.b[1444]) && (!s.b[1649])) {s.store_scalar(337, 1.0);}
        if (s.b[1443] && s.b[1444]) {s.store_sub(335, 1483, 1461);}
        s.b[1655] = ((s.v[335] < 0.05) && (0.05 >= 0.0));s.store_scalar(1655, if s.b[1655] { 1.0 } else { 0.0 });
        if ((s.b[1443] && s.b[1444]) && s.b[1655]) {s.store_sub_from_scalar(781, 0.05, 335);s.store_square(722, 781);s.store_scalar(723, (0.05 * 0.05));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tbc,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1655]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tbc);
        let (tbd,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1655]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tbd);
        if ((s.b[1443] && s.b[1444]) && s.b[1655]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1656] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1656, if s.b[1656] { 1.0 } else { 0.0 });s.b[1657] = (2.0 == 1.0);s.store_scalar(1657, if s.b[1657] { 1.0 } else { 0.0 });
        let (tbe,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && s.b[1657]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tbe);s.b[1658] = (2.0 == 2.0);s.store_scalar(1658, if s.b[1658] { 1.0 } else { 0.0 });
        let (tbf,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && (!s.b[1657])) && s.b[1658]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tbf);s.b[1659] = (2.0 == 4.0);s.store_scalar(1659, if s.b[1659] { 1.0 } else { 0.0 });
        let (tc0,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && (!s.b[1657])) && (!s.b[1658])) && s.b[1659]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc0);s.b[1660] = (2.0 == 8.0);s.store_scalar(1660, if s.b[1660] { 1.0 } else { 0.0 });
        let (tc1,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && (!s.b[1657])) && (!s.b[1658])) && (!s.b[1659])) && s.b[1660]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc1);
        let (tc2,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tc2);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_44(
        s: &mut Scratch,
    ) {
        let mut tc6: usize = 0;
        while {
            let tc5: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tc5 != 0.0
        } {
            tc6 += 1;assert!(tc6 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) {s.store_sqrt(726, 726);}
            let (tc4,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) {
        let tc3: f64 = (s.v[719] + 1.0);
        (tc3,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tc4);
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1655]) && (!s.b[1656])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1443] && s.b[1444]) && s.b[1655]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.05);s.store_div_scaled_product_indices(334, 725, 726, 0.05, 770, 1.0);s.store_sub_from_scalar(336, 0.05, 780);}
        if ((s.b[1443] && s.b[1444]) && s.b[1655]) {
        }
        if ((s.b[1443] && s.b[1444]) && (!s.b[1655])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
        if (s.b[1443] && s.b[1444]) {s.store_sqrt_mul(1449, 1547, 336);s.store_add_scaled_inputs3_indices(335, 965, 1.0, 1447, (-1.0), 1449, -1.0);}
        s.b[1661] = ((s.v[335] < (1e-25 + 1e-18)) && (1e-18 >= 0.0));s.store_scalar(1661, if s.b[1661] { 1.0 } else { 0.0 });
        if ((s.b[1443] && s.b[1444]) && s.b[1661]) {s.store_sub_from_scalar(781, (1e-25 + 1e-18), 335);s.store_square(722, 781);s.store_scalar(723, (1e-18 * 1e-18));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tc7,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1661]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tc7);
        let (tc8,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1661]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc8);
        if ((s.b[1443] && s.b[1444]) && s.b[1661]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1662] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1662, if s.b[1662] { 1.0 } else { 0.0 });s.b[1663] = (2.0 == 1.0);s.store_scalar(1663, if s.b[1663] { 1.0 } else { 0.0 });
        let (tc9,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && s.b[1663]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc9);s.b[1664] = (2.0 == 2.0);s.store_scalar(1664, if s.b[1664] { 1.0 } else { 0.0 });
        let (tca,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1664]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tca);s.b[1665] = (2.0 == 4.0);s.store_scalar(1665, if s.b[1665] { 1.0 } else { 0.0 });
        let (tcb,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && (!s.b[1664])) && s.b[1665]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tcb);s.b[1666] = (2.0 == 8.0);s.store_scalar(1666, if s.b[1666] { 1.0 } else { 0.0 });
        let (tcc,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && (!s.b[1664])) && (!s.b[1665])) && s.b[1666]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tcc);
        let (tcd,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tcd);let mut td1: usize = 0;
        while {
            let td0: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            td0 != 0.0
        } {
            td1 += 1;assert!(td1 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) {s.store_sqrt(726, 726);}
            let (tcf,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) {
        let tce: f64 = (s.v[719] + 1.0);
        (tce,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tcf);
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1661]) && (!s.b[1662])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1443] && s.b[1444]) && s.b[1661]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-18);s.store_div_scaled_product_indices(334, 725, 726, 1e-18, 770, 1.0);s.store_sub_from_scalar(1501, (1e-25 + 1e-18), 780);}
        if ((s.b[1443] && s.b[1444]) && s.b[1661]) {
        }
        if ((s.b[1443] && s.b[1444]) && (!s.b[1661])) {s.copy_ad(1501, 335);s.store_scalar(334, 1.0);}
        if (s.b[1443] && s.b[1444]) {s.store_mul_scale_offset_indices(1496, 1546, 1501, -1.0, 0.0);}
        s.b[1667] = ((s.v[1453] > s.v[965]) && (s.v[1479] != 2.0));s.store_scalar(1667, if s.b[1667] { 1.0 } else { 0.0 });s.b[1668] = ((s.v[1461] > (s.v[1511] - 0.8)) && (0.8 >= 0.0));s.store_scalar(1668, if s.b[1668] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) {s.store_offset_sub(781, 1461, 1511, 0.8);s.store_square(722, 781);s.store_scalar(723, (0.8 * 0.8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (td2,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, td2);
        let (td3,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, td3);
        if (((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_45(
        s: &mut Scratch,
    ) {
        if (((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) {s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1669] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1669, if s.b[1669] { 1.0 } else { 0.0 });s.b[1670] = (2.0 == 1.0);s.store_scalar(1670, if s.b[1670] { 1.0 } else { 0.0 });
        let (td4,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && s.b[1670]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, td4);s.b[1671] = (2.0 == 2.0);s.store_scalar(1671, if s.b[1671] { 1.0 } else { 0.0 });
        let (td5,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && (!s.b[1670])) && s.b[1671]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, td5);s.b[1672] = (2.0 == 4.0);s.store_scalar(1672, if s.b[1672] { 1.0 } else { 0.0 });
        let (td6,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && (!s.b[1670])) && (!s.b[1671])) && s.b[1672]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, td6);s.b[1673] = (2.0 == 8.0);s.store_scalar(1673, if s.b[1673] { 1.0 } else { 0.0 });
        let (td7,) = {
    if ((((((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && (!s.b[1670])) && (!s.b[1671])) && (!s.b[1672])) && s.b[1673]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, td7);
        let (td8,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, td8);let mut tdc: usize = 0;
        while {
            let tdb: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tdb != 0.0
        } {
            tdc += 1;assert!(tdc <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) {s.store_sqrt(726, 726);}
            let (tda,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) {
        let td9: f64 = (s.v[719] + 1.0);
        (td9,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tda);
        }
        if ((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && (!s.b[1669])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.8);s.store_div_scaled_product_indices(335, 725, 726, 0.8, 770, 1.0);s.store_add_offset_lhs(336, 1511, (-0.8), 780);}
        if (((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) {
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1667]) && (!s.b[1668])) {s.copy_ad(336, 1461);s.store_scalar(335, 1.0);}
        s.b[1674] = ((s.v[1520] > (s.v[1511] - 0.8)) && (0.8 >= 0.0));s.store_scalar(1674, if s.b[1674] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) {s.store_offset_sub(781, 1520, 1511, 0.8);s.store_square(722, 781);s.store_scalar(723, (0.8 * 0.8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tdd,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tdd);
        let (tde,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tde);
        if (((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1675] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1675, if s.b[1675] { 1.0 } else { 0.0 });s.b[1676] = (2.0 == 1.0);s.store_scalar(1676, if s.b[1676] { 1.0 } else { 0.0 });
        let (tdf,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) && s.b[1676]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tdf);s.b[1677] = (2.0 == 2.0);s.store_scalar(1677, if s.b[1677] { 1.0 } else { 0.0 });
        let (te0,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) && (!s.b[1676])) && s.b[1677]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, te0);s.b[1678] = (2.0 == 4.0);s.store_scalar(1678, if s.b[1678] { 1.0 } else { 0.0 });
        let (te1,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) && (!s.b[1676])) && (!s.b[1677])) && s.b[1678]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, te1);s.b[1679] = (2.0 == 8.0);s.store_scalar(1679, if s.b[1679] { 1.0 } else { 0.0 });
        let (te2,) = {
    if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) && (!s.b[1676])) && (!s.b[1677])) && (!s.b[1678])) && s.b[1679]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, te2);
        let (te3,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, te3);let mut te7: usize = 0;
        while {
            let te6: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            te6 != 0.0
        } {
            te7 += 1;assert!(te7 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) {s.store_sqrt(726, 726);}
            let (te5,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) {
        let te4: f64 = (s.v[719] + 1.0);
        (te4,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, te5);
        }
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && (!s.b[1675])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.8);s.store_div_scaled_product_indices(334, 725, 726, 0.8, 770, 1.0);s.store_add_offset_lhs(336, 1511, (-0.8), 780);}
        if (((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) {
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_46(
        s: &mut Scratch,
    ) {
        if (((s.b[1443] && s.b[1444]) && (!s.b[1667])) && (!s.b[1674])) {s.copy_ad(336, 1520);s.store_scalar(334, 1.0);}
        if (s.b[1443] && s.b[1444]) {s.store_mul_ad_affine_product_lhs(1505, s.ad_value(964), A::exp(A::mul(s.ad_value(154), A::sub(s.ad_value(336), s.ad_value(1511)))), (-1.6021918e-19), 0.0, 1447);}
        s.b[1680] = (((s.v[1461] - s.v[1511]) < 0.06) && (0.06 >= 0.0));s.store_scalar(1680, if s.b[1680] { 1.0 } else { 0.0 });
        if ((s.b[1443] && s.b[1444]) && s.b[1680]) {s.store_sub_from_scalar_ad(781, 0.06, A::sub(s.ad_value(1461), s.ad_value(1511)));s.store_square(722, 781);s.store_scalar(723, (0.06 * 0.06));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (te8,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1680]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, te8);
        let (te9,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1680]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, te9);
        if ((s.b[1443] && s.b[1444]) && s.b[1680]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1681] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1681, if s.b[1681] { 1.0 } else { 0.0 });s.b[1682] = (2.0 == 1.0);s.store_scalar(1682, if s.b[1682] { 1.0 } else { 0.0 });
        let (tea,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) && s.b[1682]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tea);s.b[1683] = (2.0 == 2.0);s.store_scalar(1683, if s.b[1683] { 1.0 } else { 0.0 });
        let (teb,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) && (!s.b[1682])) && s.b[1683]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, teb);s.b[1684] = (2.0 == 4.0);s.store_scalar(1684, if s.b[1684] { 1.0 } else { 0.0 });
        let (tec,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) && (!s.b[1682])) && (!s.b[1683])) && s.b[1684]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tec);s.b[1685] = (2.0 == 8.0);s.store_scalar(1685, if s.b[1685] { 1.0 } else { 0.0 });
        let (ted,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) && (!s.b[1682])) && (!s.b[1683])) && (!s.b[1684])) && s.b[1685]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, ted);
        let (tee,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tee);let mut tf2: usize = 0;
        while {
            let tf1: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tf1 != 0.0
        } {
            tf2 += 1;assert!(tf2 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) {s.store_sqrt(726, 726);}
            let (tf0,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) {
        let tef: f64 = (s.v[719] + 1.0);
        (tef,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tf0);
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1680]) && (!s.b[1681])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1443] && s.b[1444]) && s.b[1680]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.06);s.store_div_scaled_product_indices(334, 725, 726, 0.06, 770, 1.0);s.store_sub_from_scalar(336, 0.06, 780);}
        if ((s.b[1443] && s.b[1444]) && s.b[1680]) {
        }
        if ((s.b[1443] && s.b[1444]) && (!s.b[1680])) {s.store_sub(336, 1461, 1511);s.store_scalar(334, 1.0);}
        if (s.b[1443] && s.b[1444]) {s.store_offset_add_scaled_product_mixed_aii(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, 154, 336, (-1.0), (10.0 * 2.220446049250313e-16));s.store_mul_scaled_sqrt_rhs(1515, 209, -1.0, 338);s.store_sub_scaled_inputs_mixed_ai(338, A::offset(A::exp_scaled_input(s.ad_value(154), 0.1), (-1.0)), 1.0, 154, 0.1);s.store_mul_sqrt_rhs(1540, 209, 338);s.copy_ad(349, 790);}
        s.b[1686] = (s.v[790] > 1e-6);s.store_scalar(1686, if s.b[1686] { 1.0 } else { 0.0 });
        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {s.store_div_square_rhs(336, 1502, 185);s.store_add_scaled_inputs3_offset_indices(334, 85, 1.0, 155, (-1.0), 1438, -1.0, 2.0);s.store_offset_mul_ad(338, A::div_from_scalar(2.0, s.ad_value(336)), s.ad_value(334), 1.0);}
        s.b[1687] = ((s.v[338] < 2.0) && (2.0 >= 0.0));s.store_scalar(1687, if s.b[1687] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) {s.store_sub_from_scalar(781, 2.0, 338);s.store_square(722, 781);s.store_scalar(723, (2.0 * 2.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_47(
        s: &mut Scratch,
    ) {
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) {s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tf3,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tf3);
        let (tf4,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tf4);
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1688] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1688, if s.b[1688] { 1.0 } else { 0.0 });s.b[1689] = (2.0 == 1.0);s.store_scalar(1689, if s.b[1689] { 1.0 } else { 0.0 });
        let (tf5,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) && s.b[1689]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tf5);s.b[1690] = (2.0 == 2.0);s.store_scalar(1690, if s.b[1690] { 1.0 } else { 0.0 });
        let (tf6,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) && (!s.b[1689])) && s.b[1690]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tf6);s.b[1691] = (2.0 == 4.0);s.store_scalar(1691, if s.b[1691] { 1.0 } else { 0.0 });
        let (tf7,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) && (!s.b[1689])) && (!s.b[1690])) && s.b[1691]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tf7);s.b[1692] = (2.0 == 8.0);s.store_scalar(1692, if s.b[1692] { 1.0 } else { 0.0 });
        let (tf8,) = {
    if ((((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) && (!s.b[1689])) && (!s.b[1690])) && (!s.b[1691])) && s.b[1692]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tf8);
        let (tf9,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tf9);let mut tfd: usize = 0;
        while {
            let tfc: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tfc != 0.0
        } {
            tfd += 1;assert!(tfd <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) {s.store_sqrt(726, 726);}
            let (tfb,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) {
        let tfa: f64 = (s.v[719] + 1.0);
        (tfa,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tfb);
        }
        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && (!s.b[1688])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 2.0);s.store_div_scaled_product_indices(334, 725, 726, 2.0, 770, 1.0);s.store_sub_from_scalar(343, 2.0, 780);}
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) {
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && (!s.b[1687])) {s.copy_ad(343, 338);s.store_scalar(334, 1.0);}
        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {s.store_offset(343, 343, 1e-25);s.store_sqrt(337, 343);s.store_mul_scale_offset_indices(338, 336, 337, -1.0, 1.0);s.store_add_offset_lhs(344, 85, 2.0, 338);}
        s.b[1693] = ((s.v[344] < (0.3 + 0.2)) && (0.2 >= 0.0));s.store_scalar(1693, if s.b[1693] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) {s.store_sub_from_scalar(781, (0.3 + 0.2), 344);s.store_square(722, 781);s.store_scalar(723, (0.2 * 0.2));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tfe,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tfe);
        let (tff,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tff);
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1694] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1694, if s.b[1694] { 1.0 } else { 0.0 });s.b[1695] = (4.0 == 1.0);s.store_scalar(1695, if s.b[1695] { 1.0 } else { 0.0 });
        let (t100,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) && s.b[1695]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t100);s.b[1696] = (4.0 == 2.0);s.store_scalar(1696, if s.b[1696] { 1.0 } else { 0.0 });
        let (t101,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) && (!s.b[1695])) && s.b[1696]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t101);s.b[1697] = (4.0 == 4.0);s.store_scalar(1697, if s.b[1697] { 1.0 } else { 0.0 });
        let (t102,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) && (!s.b[1695])) && (!s.b[1696])) && s.b[1697]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t102);s.b[1698] = (4.0 == 8.0);s.store_scalar(1698, if s.b[1698] { 1.0 } else { 0.0 });
    }
}
