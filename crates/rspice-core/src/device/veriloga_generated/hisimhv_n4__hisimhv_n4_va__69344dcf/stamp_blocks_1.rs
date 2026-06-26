#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
    ) {
        let mut assign25250_loop_guard: usize = 0;
        while {
            let assign25250_cond_e21711: f64 = (150.0 + 1.0);
            let assign25250_cond_e21713: f64 = if ((s.b[1439] && s.b[1440]) && (s.v[97] <= assign25250_cond_e21711)) { 1.0 } else { 0.0 };
            assign25250_cond_e21713 != 0.0
        } {
            assign25250_loop_guard += 1;
            assert!(assign25250_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1439] && s.b[1440]) {
                s.store_mul_sub_ad_rhs(1460, 1531, A::add_scaled_product(s.ad_value(1431), 1.0, s.ad_value(1532), s.ad_value(1479), 1.0), s.ad_value(1459));
                s.store_mul(1529, 1531, 1532);
                s.store_sub(335, 1479, 1460);
            }
            s.b[1590] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1590] = if s.b[1590] { 1.0 } else { 0.0 };
            if ((s.b[1439] && s.b[1440]) && s.b[1590]) {
                s.store_sub_from_scalar(781, 0.1, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.1 * 0.1));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1591] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1591] = if s.b[1591] { 1.0 } else { 0.0 };
            s.b[1592] = (2.0 == 1.0);
            s.v[1592] = if s.b[1592] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && s.b[1440]) && s.b[1590]) && s.b[1591]) && s.b[1592]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1593] = (2.0 == 2.0);
            s.v[1593] = if s.b[1593] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && s.b[1440]) && s.b[1590]) && s.b[1591]) && (!s.b[1592])) && s.b[1593]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1594] = (2.0 == 4.0);
            s.v[1594] = if s.b[1594] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && s.b[1440]) && s.b[1590]) && s.b[1591]) && (!s.b[1592])) && (!s.b[1593])) && s.b[1594]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1595] = (2.0 == 8.0);
            s.v[1595] = if s.b[1595] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && s.b[1440]) && s.b[1590]) && s.b[1591]) && (!s.b[1592])) && (!s.b[1593])) && (!s.b[1594])) && s.b[1595]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1590]) && s.b[1591]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign25250_body29_loop_guard: usize = 0;
            while {
                let assign25250_body29_cond_e22004: f64 = if ((((s.b[1439] && s.b[1440]) && s.b[1590]) && s.b[1591]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign25250_body29_cond_e22004 != 0.0
            } {
                assign25250_body29_loop_guard += 1;
                assert!(assign25250_body29_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && s.b[1440]) && s.b[1590]) && s.b[1591]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1590]) && (!s.b[1591])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((s.b[1439] && s.b[1440]) && s.b[1590]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product(334, s.ad_value(725), s.ad_value(726), 0.1, s.ad_value(770), 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if ((s.b[1439] && s.b[1440]) && s.b[1590]) {
            }
            if ((s.b[1439] && s.b[1440]) && (!s.b[1590])) {
                s.copy_ad(336, 335);
                s.store_scalar(334, 1.0);
            }
            if (s.b[1439] && s.b[1440]) {
                s.store_sqrt_mul(1443, 1543, 336);
            }
            s.b[1596] = ((s.v[1443] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.v[1596] = if s.b[1596] { 1.0 } else { 0.0 };
            if ((s.b[1439] && s.b[1440]) && s.b[1596]) {
                s.store_offset_sub(781, 1443, 965, 1e-8);
                s.store_square(722, 781);
                s.store_scalar(723, (1e-8 * 1e-8));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1597] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1597] = if s.b[1597] { 1.0 } else { 0.0 };
            s.b[1598] = (2.0 == 1.0);
            s.v[1598] = if s.b[1598] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && s.b[1440]) && s.b[1596]) && s.b[1597]) && s.b[1598]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1599] = (2.0 == 2.0);
            s.v[1599] = if s.b[1599] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && s.b[1440]) && s.b[1596]) && s.b[1597]) && (!s.b[1598])) && s.b[1599]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1600] = (2.0 == 4.0);
            s.v[1600] = if s.b[1600] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && s.b[1440]) && s.b[1596]) && s.b[1597]) && (!s.b[1598])) && (!s.b[1599])) && s.b[1600]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1601] = (2.0 == 8.0);
            s.v[1601] = if s.b[1601] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && s.b[1440]) && s.b[1596]) && s.b[1597]) && (!s.b[1598])) && (!s.b[1599])) && (!s.b[1600])) && s.b[1601]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1596]) && s.b[1597]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign25250_body65_loop_guard: usize = 0;
            while {
                let assign25250_body65_cond_e22393: f64 = if ((((s.b[1439] && s.b[1440]) && s.b[1596]) && s.b[1597]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign25250_body65_cond_e22393 != 0.0
            } {
                assign25250_body65_loop_guard += 1;
                assert!(assign25250_body65_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && s.b[1440]) && s.b[1596]) && s.b[1597]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1596]) && (!s.b[1597])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((s.b[1439] && s.b[1440]) && s.b[1596]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product(337, s.ad_value(725), s.ad_value(726), 1e-8, s.ad_value(770), 1.0);
                s.store_add_ad_lhs(1443, A::offset(s.ad_value(965), (-1e-8)), 780);
            }
            if ((s.b[1439] && s.b[1440]) && s.b[1596]) {
            }
            if ((s.b[1439] && s.b[1440]) && (!s.b[1596])) {
            }
            if ((s.b[1439] && s.b[1440]) && (!s.b[1596])) {
                s.store_scalar(337, 1.0);
            }
            if (s.b[1439] && s.b[1440]) {
                s.store_sqrt_mul_ad(1447, s.ad_value(1546), A::add_scaled_inputs3(s.ad_value(1460), 1.0, s.ad_value(1431), (-1.0), s.ad_value(1459), 1.0));
                s.store_mul(1493, 1443, 1542);
                s.store_mul_ad_product_lhs(1523, A::div_from_scalar(1.034943e-10, s.ad_value(1443)), s.ad_value(334), 337);
                s.store_mul_ad_product_lhs(1525, A::div_from_scalar((-1.034943e-10), s.ad_value(1443)), s.ad_value(334), 337);
                s.store_mul_neg_lhs(1494, 1447, 1540);
                s.store_div_from_scalar(1527, (-1.034943e-10), 1447);
                s.store_scaled_mul(335, 1498, 1539, 8.0);
            }
            if (s.b[1439] && s.b[1440]) {
                let assign25250_body81_ad_e22628: A = A::add(A::add_scaled_inputs4(A::mul3_scaled_output(s.ad_value(1460), s.ad_value(1460), s.ad_value(1538), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1460), s.ad_value(1538), s.ad_value(1457), 8.0), (-1.0), A::mul3_scaled_output(s.ad_value(1538), s.ad_value(1457), s.ad_value(1457), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1460), s.ad_value(1498), s.ad_value(1539), 4.0), 1.0), A::mul3_scaled_output(s.ad_value(1457), s.ad_value(1498), s.ad_value(1539), 4.0));
                s.store_ad_value(1516, A::div_scaled_add_product(assign25250_body81_ad_e22628, 1.0, A::mul3(s.ad_value(1541), s.ad_value(1537), s.ad_value(1539)), s.ad_value(1539), 1.0, s.ad_value(335), 1.0));
            }
            if (s.b[1439] && s.b[1440]) {
                s.store_div_ad_lhs(1517, A::add_scaled_products3(s.ad_value(1460), s.ad_value(1538), (-8.0), s.ad_value(1538), s.ad_value(1457), (4.0 * 2.0), s.ad_value(1498), s.ad_value(1539), 4.0), 335);
                s.store_div_ad_lhs(1518, A::add_scaled_products3(s.ad_value(1460), s.ad_value(1538), (4.0 * 2.0), s.ad_value(1538), s.ad_value(1457), (-8.0), s.ad_value(1498), s.ad_value(1539), 4.0), 335);
                s.store_mul_sub_rhs(335, 154, 1457, 1479);
                s.store_exp(336, 335);
            }
            s.b[1602] = (s.v[1457] >= s.v[1479]);
            s.v[1602] = if s.b[1602] { 1.0 } else { 0.0 };
            if ((s.b[1439] && s.b[1440]) && s.b[1602]) {
                s.store_mul_scaled_ad_rhs(1471, 209, -1.0, A::sqrt(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15)));
                s.store_mul_ad(1519, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(1471), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
                s.store_neg(1521, 1519);
            }
            if ((s.b[1439] && s.b[1440]) && (!s.b[1602])) {
                s.store_exp_ad(337, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1457), s.ad_value(1431))));
                s.store_exp_ad(338, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1479), s.ad_value(1431))));
                s.store_mul_sqrt_ad_rhs(1471, 209, A::add_scaled_product(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15), 1.0, s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0));
                s.store_div_scaled_product(339, s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(1471), 1.0);
                s.store_mul_add_ad_rhs(1519, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
                s.store_mul_add_ad_rhs(1521, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));
            }
            s.b[1603] = ((s.v[1516] > (s.v[1507] - s.v[1515])) && (s.v[1515] >= 0.0));
            s.v[1603] = if s.b[1603] { 1.0 } else { 0.0 };
            if ((s.b[1439] && s.b[1440]) && s.b[1603]) {
                s.store_add_scaled_inputs3(781, s.ad_value(1516), 1.0, s.ad_value(1507), (-1.0), s.ad_value(1515), 1.0);
                s.store_square(722, 781);
                s.store_square(723, 1515);
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1604] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.v[1604] = if s.b[1604] { 1.0 } else { 0.0 };
            s.b[1605] = (4.0 == 1.0);
            s.v[1605] = if s.b[1605] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && s.b[1440]) && s.b[1603]) && s.b[1604]) && s.b[1605]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1606] = (4.0 == 2.0);
            s.v[1606] = if s.b[1606] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && s.b[1440]) && s.b[1603]) && s.b[1604]) && (!s.b[1605])) && s.b[1606]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1607] = (4.0 == 4.0);
            s.v[1607] = if s.b[1607] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && s.b[1440]) && s.b[1603]) && s.b[1604]) && (!s.b[1605])) && (!s.b[1606])) && s.b[1607]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1608] = (4.0 == 8.0);
            s.v[1608] = if s.b[1608] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && s.b[1440]) && s.b[1603]) && s.b[1604]) && (!s.b[1605])) && (!s.b[1606])) && (!s.b[1607])) && s.b[1608]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1603]) && s.b[1604]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign25250_body126_loop_guard: usize = 0;
            while {
                let assign25250_body126_cond_e23174: f64 = if ((((s.b[1439] && s.b[1440]) && s.b[1603]) && s.b[1604]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign25250_body126_cond_e23174 != 0.0
            } {
                assign25250_body126_loop_guard += 1;
                assert!(assign25250_body126_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && s.b[1440]) && s.b[1603]) && s.b[1604]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1603]) && (!s.b[1604])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                    }
                });
            }
            if ((s.b[1439] && s.b[1440]) && s.b[1603]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_mul3_lhs(780, 781, 1515, 726);
                s.store_ad_value(334, A::div_scaled_product3(s.ad_value(1515), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0));
                s.store_add_scaled_inputs3(335, s.ad_value(1507), 1.0, s.ad_value(1515), (-1.0), s.ad_value(780), 1.0);
            }
            if ((s.b[1439] && s.b[1440]) && s.b[1603]) {
            }
            if ((s.b[1439] && s.b[1440]) && (!s.b[1603])) {
                s.copy_ad(335, 1516);
                s.store_scalar(334, 1.0);
            }
            if (s.b[1439] && s.b[1440]) {
                s.store_sub(1481, 1479, 335);
                s.store_mul_neg_lhs(1483, 1517, 334);
                s.store_sub_from_scalar_ad(1484, 1.0, A::mul3(s.ad_value(1518), s.ad_value(1529), s.ad_value(334)));
                s.store_add_scaled_inputs3(1482, A::add_scaled_product(s.ad_value(1471), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1457)), 1.0), 1.0, s.ad_value(1493), 1.0, s.ad_value(1494), 1.0);
                s.store_sub(1485, 1519, 185);
                s.store_ad_value(1486, A::add_scaled_inputs_products(s.ad_value(1521), 1.0, s.ad_value(1523), 1.0, s.ad_value(1525), s.ad_value(1529), 1.0, s.ad_value(1527), s.ad_value(1529), 1.0));
                s.store_add_scaled_products(1487, s.ad_value(1483), s.ad_value(1486), 1.0, s.ad_value(1485), s.ad_value(1484), (-1.0));
                s.store_div(1488, 1486, 1487);
                s.store_div_scaled_inputs(1489, s.ad_value(1484), -1.0, s.ad_value(1487), 1.0);
                s.store_div_scaled_inputs(1490, s.ad_value(1485), -1.0, s.ad_value(1487), 1.0);
                s.store_div(1491, 1483, 1487);
            }
            s.b[1609] = (((((s.v[1490] * s.v[1481]) + (s.v[1491] * s.v[1482]))) as f64).abs() > 0.5);
            s.v[1609] = if s.b[1609] { 1.0 } else { 0.0 };
            if ((s.b[1439] && s.b[1440]) && s.b[1609]) {
                s.store_offset(1457, 1457, (-(0.5 * (if (((s.v[1488] * s.v[1481]) + (s.v[1489] * s.v[1482])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if ((s.b[1439] && s.b[1440]) && s.b[1609]) {
                s.store_offset(1479, 1479, (-(0.5 * (if (((s.v[1490] * s.v[1481]) + (s.v[1491] * s.v[1482])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if ((s.b[1439] && s.b[1440]) && (!s.b[1609])) {
                s.store_sub_ad_rhs(1457, 1457, A::add_scaled_products(s.ad_value(1488), s.ad_value(1481), 1.0, s.ad_value(1489), s.ad_value(1482), 1.0));
                s.store_sub_ad_rhs(1479, 1479, A::add_scaled_products(s.ad_value(1490), s.ad_value(1481), 1.0, s.ad_value(1491), s.ad_value(1482), 1.0));
            }
            s.b[1610] = (((((s.v[1457] - s.v[1465])) as f64).abs() <= 1e-12) && ((((s.v[1479] - s.v[1466])) as f64).abs() <= 1e-12));
            s.v[1610] = if s.b[1610] { 1.0 } else { 0.0 };
            if ((s.b[1439] && s.b[1440]) && s.b[1610]) {
                s.store_scalar(97, (150.0 + 1.0));
                s.store_scalar(79, 1.0);
            }
            if (s.b[1439] && s.b[1440]) {
                s.copy_ad(1465, 1457);
                s.copy_ad(1466, 1479);
                s.store_offset(97, 97, 1.0);
            }
        }

        s.b[1612] = ((s.v[1449] > s.v[965]) && (s.v[1475] != 2.0));
        s.v[1612] = if s.b[1612] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
    ) {
        s.b[1613] = ((s.v[1479] > (s.v[1457] - 0.02)) && (0.02 >= 0.0));
        s.v[1613] = if s.b[1613] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) {
            s.store_offset_sub(781, 1479, 1457, 0.02);
            s.store_square(722, 781);
            s.store_scalar(723, (0.02 * 0.02));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1614] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1614] = if s.b[1614] { 1.0 } else { 0.0 };

        s.b[1615] = (2.0 == 1.0);
        s.v[1615] = if s.b[1615] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) && s.b[1614]) && s.b[1615]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1616] = (2.0 == 2.0);
        s.v[1616] = if s.b[1616] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) && s.b[1614]) && (!s.b[1615])) && s.b[1616]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1617] = (2.0 == 4.0);
        s.v[1617] = if s.b[1617] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) && s.b[1614]) && (!s.b[1615])) && (!s.b[1616])) && s.b[1617]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1618] = (2.0 == 8.0);
        s.v[1618] = if s.b[1618] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) && s.b[1614]) && (!s.b[1615])) && (!s.b[1616])) && (!s.b[1617])) && s.b[1618]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) && s.b[1614]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign25540_loop_guard: usize = 0;
        while {
            let assign25540_cond_e23865: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) && s.b[1614]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign25540_cond_e23865 != 0.0
        } {
            assign25540_loop_guard += 1;
            assert!(assign25540_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) && s.b[1614]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) && (!s.b[1614])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.02);
            s.store_div_scaled_product(335, s.ad_value(725), s.ad_value(726), 0.02, s.ad_value(770), 1.0);
            s.store_add_ad_lhs(1479, A::offset(s.ad_value(1457), (-0.02)), 780);
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1612]) && s.b[1613]) {
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1612]) && (!s.b[1613])) {
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1612]) && (!s.b[1613])) {
            s.store_scalar(335, 1.0);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_mul_sub_ad_rhs(1460, 1531, A::add_scaled_product(s.ad_value(1431), 1.0, s.ad_value(1532), s.ad_value(1479), 1.0), s.ad_value(1459));
            s.store_mul_sub_rhs(335, 154, 1457, 1479);
            s.store_exp(336, 335);
        }

        s.b[1619] = (s.v[1457] >= s.v[1479]);
        s.v[1619] = if s.b[1619] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1619]) {
            s.store_mul_scaled_ad_rhs(1471, 209, -1.0, A::sqrt(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15)));
            s.copy_ad(1534, 1471);
            s.store_scalar(1513, 0.0);
            s.store_scalar(1473, 0.0);
            s.store_sqrt_mul_ad(1443, s.ad_value(1543), A::sub(s.ad_value(1479), s.ad_value(1460)));
        }

        s.b[1620] = ((s.v[1443] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
        s.v[1620] = if s.b[1620] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) {
            s.store_offset_sub(781, 1443, 965, 1e-8);
            s.store_square(722, 781);
            s.store_scalar(723, (1e-8 * 1e-8));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1621] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1621] = if s.b[1621] { 1.0 } else { 0.0 };

        s.b[1622] = (2.0 == 1.0);
        s.v[1622] = if s.b[1622] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) && s.b[1621]) && s.b[1622]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1623] = (2.0 == 2.0);
        s.v[1623] = if s.b[1623] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) && s.b[1621]) && (!s.b[1622])) && s.b[1623]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1624] = (2.0 == 4.0);
        s.v[1624] = if s.b[1624] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) && s.b[1621]) && (!s.b[1622])) && (!s.b[1623])) && s.b[1624]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1625] = (2.0 == 8.0);
        s.v[1625] = if s.b[1625] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) && s.b[1621]) && (!s.b[1622])) && (!s.b[1623])) && (!s.b[1624])) && s.b[1625]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) && s.b[1621]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign25980_loop_guard: usize = 0;
        while {
            let assign25980_cond_e24396: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) && s.b[1621]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign25980_cond_e24396 != 0.0
        } {
            assign25980_loop_guard += 1;
            assert!(assign25980_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) && s.b[1621]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) && (!s.b[1621])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product(337, s.ad_value(725), s.ad_value(726), 1e-8, s.ad_value(770), 1.0);
            s.store_add_ad_lhs(1443, A::offset(s.ad_value(965), (-1e-8)), 780);
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1619]) && s.b[1620]) {
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1619]) && (!s.b[1620])) {
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1619]) && (!s.b[1620])) {
            s.store_scalar(337, 1.0);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1619]) {
            s.store_sqrt_mul_ad(1447, s.ad_value(1546), A::add_scaled_inputs3(s.ad_value(1460), 1.0, s.ad_value(1431), (-1.0), s.ad_value(1459), 1.0));
            s.store_mul(1493, 1443, 1542);
            s.store_mul_neg_lhs(1494, 1447, 1540);
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1619])) {
            s.store_exp_ad(337, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1457), s.ad_value(1431))));
            s.store_exp_ad(338, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1479), s.ad_value(1431))));
            s.store_mul_sqrt_ad_rhs(1471, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
        }

        s.b[1626] = ((s.v[1449] > s.v[965]) && (s.v[1475] != 2.0));
        s.v[1626] = if s.b[1626] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1626]) {
            s.store_scalar(1473, 0.0);
            s.store_scalar(1513, 0.0);
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && (!s.b[1626])) {
            s.store_mul_sqrt_ad_rhs(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1457), s.ad_value(1431)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1479), s.ad_value(1431)))))), s.ad_value(335)));
            s.store_add_scaled_product(1473, s.ad_value(337), 1.0, s.ad_value(209), A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));
            s.store_mul_sqrt_ad_rhs(1513, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1619])) {
            s.store_scalar(1534, 0.0);
            s.store_sub(335, 1479, 1460);
        }

        s.b[1627] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
        s.v[1627] = if s.b[1627] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) {
            s.store_sub_from_scalar(781, 0.1, 335);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1628] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1628] = if s.b[1628] { 1.0 } else { 0.0 };

        s.b[1629] = (2.0 == 1.0);
        s.v[1629] = if s.b[1629] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) && s.b[1628]) && s.b[1629]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1630] = (2.0 == 2.0);
        s.v[1630] = if s.b[1630] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) && s.b[1628]) && (!s.b[1629])) && s.b[1630]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1631] = (2.0 == 4.0);
        s.v[1631] = if s.b[1631] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) && s.b[1628]) && (!s.b[1629])) && (!s.b[1630])) && s.b[1631]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1632] = (2.0 == 8.0);
        s.v[1632] = if s.b[1632] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) && s.b[1628]) && (!s.b[1629])) && (!s.b[1630])) && (!s.b[1631])) && s.b[1632]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) && s.b[1628]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign26470_loop_guard: usize = 0;
        while {
            let assign26470_cond_e25071: f64 = if (((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) && s.b[1628]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign26470_cond_e25071 != 0.0
        } {
            assign26470_loop_guard += 1;
            assert!(assign26470_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) && s.b[1628]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) && (!s.b[1628])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

    }

    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
    ) {
        if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product(334, s.ad_value(725), s.ad_value(726), 0.1, s.ad_value(770), 1.0);
            s.store_sub_from_scalar(336, 0.1, 780);
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1627]) {
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && (!s.b[1627])) {
            s.copy_ad(336, 335);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1619])) {
            s.store_sqrt_mul(1443, 1543, 336);
        }

        s.b[1633] = ((s.v[1443] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
        s.v[1633] = if s.b[1633] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) {
            s.store_offset_sub(781, 1443, 965, 1e-8);
            s.store_square(722, 781);
            s.store_scalar(723, (1e-8 * 1e-8));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1634] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1634] = if s.b[1634] { 1.0 } else { 0.0 };

        s.b[1635] = (2.0 == 1.0);
        s.v[1635] = if s.b[1635] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) && s.b[1634]) && s.b[1635]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1636] = (2.0 == 2.0);
        s.v[1636] = if s.b[1636] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) && s.b[1634]) && (!s.b[1635])) && s.b[1636]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1637] = (2.0 == 4.0);
        s.v[1637] = if s.b[1637] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) && s.b[1634]) && (!s.b[1635])) && (!s.b[1636])) && s.b[1637]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1638] = (2.0 == 8.0);
        s.v[1638] = if s.b[1638] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) && s.b[1634]) && (!s.b[1635])) && (!s.b[1636])) && (!s.b[1637])) && s.b[1638]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) && s.b[1634]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign26830_loop_guard: usize = 0;
        while {
            let assign26830_cond_e25556: f64 = if (((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) && s.b[1634]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign26830_cond_e25556 != 0.0
        } {
            assign26830_loop_guard += 1;
            assert!(assign26830_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) && s.b[1634]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) && (!s.b[1634])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product(337, s.ad_value(725), s.ad_value(726), 1e-8, s.ad_value(770), 1.0);
            s.store_add_ad_lhs(1443, A::offset(s.ad_value(965), (-1e-8)), 780);
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && s.b[1633]) {
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && (!s.b[1633])) {
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1619])) && (!s.b[1633])) {
            s.store_scalar(337, 1.0);
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1619])) {
            s.store_sqrt_mul_ad(1447, s.ad_value(1546), A::add_scaled_inputs3(s.ad_value(1460), 1.0, s.ad_value(1431), (-1.0), s.ad_value(1459), 1.0));
            s.store_mul(1493, 1443, 1542);
            s.store_mul_neg_lhs(1494, 1447, 1540);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_sub(335, 1479, 1460);
        }

        s.b[1639] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
        s.v[1639] = if s.b[1639] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1639]) {
            s.store_sub_from_scalar(781, 0.1, 335);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1640] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1640] = if s.b[1640] { 1.0 } else { 0.0 };

        s.b[1641] = (2.0 == 1.0);
        s.v[1641] = if s.b[1641] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && s.b[1440]) && s.b[1639]) && s.b[1640]) && s.b[1641]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1642] = (2.0 == 2.0);
        s.v[1642] = if s.b[1642] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1642]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1643] = (2.0 == 4.0);
        s.v[1643] = if s.b[1643] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && (!s.b[1642])) && s.b[1643]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1644] = (2.0 == 8.0);
        s.v[1644] = if s.b[1644] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && (!s.b[1642])) && (!s.b[1643])) && s.b[1644]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1639]) && s.b[1640]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign27220_loop_guard: usize = 0;
        while {
            let assign27220_cond_e26013: f64 = if ((((s.b[1439] && s.b[1440]) && s.b[1639]) && s.b[1640]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign27220_cond_e26013 != 0.0
        } {
            assign27220_loop_guard += 1;
            assert!(assign27220_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && s.b[1440]) && s.b[1639]) && s.b[1640]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1639]) && (!s.b[1640])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1639]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product(334, s.ad_value(725), s.ad_value(726), 0.1, s.ad_value(770), 1.0);
            s.store_sub_from_scalar(336, 0.1, 780);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1639]) {
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1639])) {
            s.copy_ad(336, 335);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_sqrt_mul(1443, 1543, 336);
        }

        s.b[1645] = ((s.v[1443] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
        s.v[1645] = if s.b[1645] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1645]) {
            s.store_offset_sub(781, 1443, 965, 1e-8);
            s.store_square(722, 781);
            s.store_scalar(723, (1e-8 * 1e-8));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1646] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1646] = if s.b[1646] { 1.0 } else { 0.0 };

        s.b[1647] = (2.0 == 1.0);
        s.v[1647] = if s.b[1647] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && s.b[1440]) && s.b[1645]) && s.b[1646]) && s.b[1647]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1648] = (2.0 == 2.0);
        s.v[1648] = if s.b[1648] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && s.b[1645]) && s.b[1646]) && (!s.b[1647])) && s.b[1648]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1649] = (2.0 == 4.0);
        s.v[1649] = if s.b[1649] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && s.b[1645]) && s.b[1646]) && (!s.b[1647])) && (!s.b[1648])) && s.b[1649]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1650] = (2.0 == 8.0);
        s.v[1650] = if s.b[1650] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && s.b[1645]) && s.b[1646]) && (!s.b[1647])) && (!s.b[1648])) && (!s.b[1649])) && s.b[1650]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1645]) && s.b[1646]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign27580_loop_guard: usize = 0;
        while {
            let assign27580_cond_e26402: f64 = if ((((s.b[1439] && s.b[1440]) && s.b[1645]) && s.b[1646]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign27580_cond_e26402 != 0.0
        } {
            assign27580_loop_guard += 1;
            assert!(assign27580_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && s.b[1440]) && s.b[1645]) && s.b[1646]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1645]) && (!s.b[1646])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1645]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product(337, s.ad_value(725), s.ad_value(726), 1e-8, s.ad_value(770), 1.0);
            s.store_add_ad_lhs(1443, A::offset(s.ad_value(965), (-1e-8)), 780);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1645]) {
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1645])) {
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1645])) {
            s.store_scalar(337, 1.0);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_sub(335, 1479, 1457);
        }

        s.b[1651] = ((s.v[335] < 0.05) && (0.05 >= 0.0));
        s.v[1651] = if s.b[1651] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1651]) {
            s.store_sub_from_scalar(781, 0.05, 335);
        }

    }

    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
    ) {
        if ((s.b[1439] && s.b[1440]) && s.b[1651]) {
            s.store_square(722, 781);
            s.store_scalar(723, (0.05 * 0.05));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1652] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1652] = if s.b[1652] { 1.0 } else { 0.0 };

        s.b[1653] = (2.0 == 1.0);
        s.v[1653] = if s.b[1653] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && s.b[1440]) && s.b[1651]) && s.b[1652]) && s.b[1653]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1654] = (2.0 == 2.0);
        s.v[1654] = if s.b[1654] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && s.b[1651]) && s.b[1652]) && (!s.b[1653])) && s.b[1654]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1655] = (2.0 == 4.0);
        s.v[1655] = if s.b[1655] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && s.b[1651]) && s.b[1652]) && (!s.b[1653])) && (!s.b[1654])) && s.b[1655]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1656] = (2.0 == 8.0);
        s.v[1656] = if s.b[1656] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && s.b[1651]) && s.b[1652]) && (!s.b[1653])) && (!s.b[1654])) && (!s.b[1655])) && s.b[1656]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1651]) && s.b[1652]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign27940_loop_guard: usize = 0;
        while {
            let assign27940_cond_e26790: f64 = if ((((s.b[1439] && s.b[1440]) && s.b[1651]) && s.b[1652]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign27940_cond_e26790 != 0.0
        } {
            assign27940_loop_guard += 1;
            assert!(assign27940_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && s.b[1440]) && s.b[1651]) && s.b[1652]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1651]) && (!s.b[1652])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1651]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.05);
            s.store_div_scaled_product(334, s.ad_value(725), s.ad_value(726), 0.05, s.ad_value(770), 1.0);
            s.store_sub_from_scalar(336, 0.05, 780);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1651]) {
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1651])) {
            s.copy_ad(336, 335);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_sqrt_mul(1445, 1543, 336);
            s.store_add_scaled_inputs3(335, s.ad_value(965), 1.0, s.ad_value(1443), (-1.0), s.ad_value(1445), -1.0);
        }

        s.b[1657] = ((s.v[335] < (1e-25 + 1e-18)) && (1e-18 >= 0.0));
        s.v[1657] = if s.b[1657] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1657]) {
            s.store_sub_from_scalar(781, (1e-25 + 1e-18), 335);
            s.store_square(722, 781);
            s.store_scalar(723, (1e-18 * 1e-18));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1658] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1658] = if s.b[1658] { 1.0 } else { 0.0 };

        s.b[1659] = (2.0 == 1.0);
        s.v[1659] = if s.b[1659] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && s.b[1440]) && s.b[1657]) && s.b[1658]) && s.b[1659]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1660] = (2.0 == 2.0);
        s.v[1660] = if s.b[1660] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && s.b[1657]) && s.b[1658]) && (!s.b[1659])) && s.b[1660]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1661] = (2.0 == 4.0);
        s.v[1661] = if s.b[1661] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && s.b[1657]) && s.b[1658]) && (!s.b[1659])) && (!s.b[1660])) && s.b[1661]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1662] = (2.0 == 8.0);
        s.v[1662] = if s.b[1662] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && s.b[1657]) && s.b[1658]) && (!s.b[1659])) && (!s.b[1660])) && (!s.b[1661])) && s.b[1662]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1657]) && s.b[1658]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign28310_loop_guard: usize = 0;
        while {
            let assign28310_cond_e27189: f64 = if ((((s.b[1439] && s.b[1440]) && s.b[1657]) && s.b[1658]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign28310_cond_e27189 != 0.0
        } {
            assign28310_loop_guard += 1;
            assert!(assign28310_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && s.b[1440]) && s.b[1657]) && s.b[1658]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1657]) && (!s.b[1658])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1657]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-18);
            s.store_div_scaled_product(334, s.ad_value(725), s.ad_value(726), 1e-18, s.ad_value(770), 1.0);
            s.store_sub_from_scalar(1497, (1e-25 + 1e-18), 780);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1657]) {
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1657])) {
            s.copy_ad(1497, 335);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_mul_neg_lhs(1492, 1497, 1542);
        }

        s.b[1663] = ((s.v[1449] > s.v[965]) && (s.v[1475] != 2.0));
        s.v[1663] = if s.b[1663] { 1.0 } else { 0.0 };

        s.b[1664] = ((s.v[1457] > (s.v[1507] - 0.8)) && (0.8 >= 0.0));
        s.v[1664] = if s.b[1664] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) {
            s.store_offset_sub(781, 1457, 1507, 0.8);
            s.store_square(722, 781);
            s.store_scalar(723, (0.8 * 0.8));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1665] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1665] = if s.b[1665] { 1.0 } else { 0.0 };

        s.b[1666] = (2.0 == 1.0);
        s.v[1666] = if s.b[1666] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) && s.b[1665]) && s.b[1666]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1667] = (2.0 == 2.0);
        s.v[1667] = if s.b[1667] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) && s.b[1665]) && (!s.b[1666])) && s.b[1667]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1668] = (2.0 == 4.0);
        s.v[1668] = if s.b[1668] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) && s.b[1665]) && (!s.b[1666])) && (!s.b[1667])) && s.b[1668]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1669] = (2.0 == 8.0);
        s.v[1669] = if s.b[1669] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) && s.b[1665]) && (!s.b[1666])) && (!s.b[1667])) && (!s.b[1668])) && s.b[1669]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) && s.b[1665]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign28680_loop_guard: usize = 0;
        while {
            let assign28680_cond_e27627: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) && s.b[1665]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign28680_cond_e27627 != 0.0
        } {
            assign28680_loop_guard += 1;
            assert!(assign28680_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) && s.b[1665]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) && (!s.b[1665])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.8);
            s.store_div_scaled_product(335, s.ad_value(725), s.ad_value(726), 0.8, s.ad_value(770), 1.0);
            s.store_add_ad_lhs(336, A::offset(s.ad_value(1507), (-0.8)), 780);
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1663]) && s.b[1664]) {
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1663]) && (!s.b[1664])) {
            s.copy_ad(336, 1457);
            s.store_scalar(335, 1.0);
        }

        s.b[1670] = ((s.v[1516] > (s.v[1507] - 0.8)) && (0.8 >= 0.0));
        s.v[1670] = if s.b[1670] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) {
            s.store_offset_sub(781, 1516, 1507, 0.8);
            s.store_square(722, 781);
            s.store_scalar(723, (0.8 * 0.8));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
        }

    }

    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
    ) {
        if (((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) {
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1671] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1671] = if s.b[1671] { 1.0 } else { 0.0 };

        s.b[1672] = (2.0 == 1.0);
        s.v[1672] = if s.b[1672] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) && s.b[1671]) && s.b[1672]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1673] = (2.0 == 2.0);
        s.v[1673] = if s.b[1673] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) && s.b[1671]) && (!s.b[1672])) && s.b[1673]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1674] = (2.0 == 4.0);
        s.v[1674] = if s.b[1674] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) && s.b[1671]) && (!s.b[1672])) && (!s.b[1673])) && s.b[1674]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1675] = (2.0 == 8.0);
        s.v[1675] = if s.b[1675] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) && s.b[1671]) && (!s.b[1672])) && (!s.b[1673])) && (!s.b[1674])) && s.b[1675]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) && s.b[1671]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign29030_loop_guard: usize = 0;
        while {
            let assign29030_cond_e28090: f64 = if (((((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) && s.b[1671]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign29030_cond_e28090 != 0.0
        } {
            assign29030_loop_guard += 1;
            assert!(assign29030_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) && s.b[1671]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) && (!s.b[1671])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.8);
            s.store_div_scaled_product(334, s.ad_value(725), s.ad_value(726), 0.8, s.ad_value(770), 1.0);
            s.store_add_ad_lhs(336, A::offset(s.ad_value(1507), (-0.8)), 780);
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1663])) && s.b[1670]) {
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1663])) && (!s.b[1670])) {
            s.copy_ad(336, 1516);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_mul_ad_affine_product_lhs(1501, s.ad_value(964), A::exp(A::mul(s.ad_value(154), A::sub(s.ad_value(336), s.ad_value(1507)))), (-1.6021918e-19), 0.0, 1443);
        }

        s.b[1676] = (((s.v[1457] - s.v[1507]) < 0.06) && (0.06 >= 0.0));
        s.v[1676] = if s.b[1676] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1676]) {
            s.store_sub_from_scalar_ad(781, 0.06, A::sub(s.ad_value(1457), s.ad_value(1507)));
            s.store_square(722, 781);
            s.store_scalar(723, (0.06 * 0.06));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1677] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1677] = if s.b[1677] { 1.0 } else { 0.0 };

        s.b[1678] = (2.0 == 1.0);
        s.v[1678] = if s.b[1678] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && s.b[1440]) && s.b[1676]) && s.b[1677]) && s.b[1678]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1679] = (2.0 == 2.0);
        s.v[1679] = if s.b[1679] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && s.b[1676]) && s.b[1677]) && (!s.b[1678])) && s.b[1679]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1680] = (2.0 == 4.0);
        s.v[1680] = if s.b[1680] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && s.b[1676]) && s.b[1677]) && (!s.b[1678])) && (!s.b[1679])) && s.b[1680]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1681] = (2.0 == 8.0);
        s.v[1681] = if s.b[1681] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && s.b[1676]) && s.b[1677]) && (!s.b[1678])) && (!s.b[1679])) && (!s.b[1680])) && s.b[1681]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1676]) && s.b[1677]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign29390_loop_guard: usize = 0;
        while {
            let assign29390_cond_e28522: f64 = if ((((s.b[1439] && s.b[1440]) && s.b[1676]) && s.b[1677]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign29390_cond_e28522 != 0.0
        } {
            assign29390_loop_guard += 1;
            assert!(assign29390_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && s.b[1440]) && s.b[1676]) && s.b[1677]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1676]) && (!s.b[1677])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1676]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.06);
            s.store_div_scaled_product(334, s.ad_value(725), s.ad_value(726), 0.06, s.ad_value(770), 1.0);
            s.store_sub_from_scalar(336, 0.06, 780);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1676]) {
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1676])) {
            s.store_sub(336, 1457, 1507);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_offset_ad(338, A::add_scaled_product(A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0)), (10.0 * 2.220446049250313e-16));
            s.store_mul_scaled_ad_rhs(1511, 209, -1.0, A::sqrt(s.ad_value(338)));
            s.store_sub_scaled_ad_lhs(338, A::offset(A::exp_scaled_input(s.ad_value(154), 0.1), (-1.0)), 154, 0.1);
            s.store_mul_sqrt_rhs(1536, 209, 338);
            s.copy_ad(349, 790);
        }

        s.b[1682] = (s.v[790] > 1e-6);
        s.v[1682] = if s.b[1682] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1682]) {
            s.store_div_ad_rhs(336, 1498, A::square(s.ad_value(185)));
            s.store_add_scaled_inputs3_offset(334, s.ad_value(85), 1.0, s.ad_value(155), (-1.0), s.ad_value(1434), -1.0, 2.0);
            s.store_offset_mul_ad(338, A::div_from_scalar(2.0, s.ad_value(336)), s.ad_value(334), 1.0);
        }

        s.b[1683] = ((s.v[338] < 2.0) && (2.0 >= 0.0));
        s.v[1683] = if s.b[1683] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) {
            s.store_sub_from_scalar(781, 2.0, 338);
            s.store_square(722, 781);
            s.store_scalar(723, (2.0 * 2.0));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1684] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1684] = if s.b[1684] { 1.0 } else { 0.0 };

        s.b[1685] = (2.0 == 1.0);
        s.v[1685] = if s.b[1685] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) && s.b[1684]) && s.b[1685]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1686] = (2.0 == 2.0);
        s.v[1686] = if s.b[1686] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) && s.b[1684]) && (!s.b[1685])) && s.b[1686]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1687] = (2.0 == 4.0);
        s.v[1687] = if s.b[1687] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) && s.b[1684]) && (!s.b[1685])) && (!s.b[1686])) && s.b[1687]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1688] = (2.0 == 8.0);
        s.v[1688] = if s.b[1688] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) && s.b[1684]) && (!s.b[1685])) && (!s.b[1686])) && (!s.b[1687])) && s.b[1688]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) && s.b[1684]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign29830_loop_guard: usize = 0;
        while {
            let assign29830_cond_e29048: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) && s.b[1684]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign29830_cond_e29048 != 0.0
        } {
            assign29830_loop_guard += 1;
            assert!(assign29830_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) && s.b[1684]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) && (!s.b[1684])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 2.0);
            s.store_div_scaled_product(334, s.ad_value(725), s.ad_value(726), 2.0, s.ad_value(770), 1.0);
            s.store_sub_from_scalar(343, 2.0, 780);
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1683]) {
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && (!s.b[1683])) {
            s.copy_ad(343, 338);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1682]) {
            s.store_offset(343, 343, 1e-25);
            s.store_sqrt(337, 343);
            s.store_mul_sub_from_scalar_rhs(338, 336, 1.0, 337);
            s.store_add_ad_lhs(344, A::offset(s.ad_value(85), 2.0), 338);
        }

        s.b[1689] = ((s.v[344] < (0.3 + 0.2)) && (0.2 >= 0.0));
        s.v[1689] = if s.b[1689] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1689]) {
            s.store_sub_from_scalar(781, (0.3 + 0.2), 344);
            s.store_square(722, 781);
            s.store_scalar(723, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
        }

    }

    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
    ) {
        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1689]) {
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1690] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1690] = if s.b[1690] { 1.0 } else { 0.0 };

        s.b[1691] = (4.0 == 1.0);
        s.v[1691] = if s.b[1691] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1689]) && s.b[1690]) && s.b[1691]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1692] = (4.0 == 2.0);
        s.v[1692] = if s.b[1692] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1689]) && s.b[1690]) && (!s.b[1691])) && s.b[1692]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1693] = (4.0 == 4.0);
        s.v[1693] = if s.b[1693] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1689]) && s.b[1690]) && (!s.b[1691])) && (!s.b[1692])) && s.b[1693]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1694] = (4.0 == 8.0);
        s.v[1694] = if s.b[1694] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1689]) && s.b[1690]) && (!s.b[1691])) && (!s.b[1692])) && (!s.b[1693])) && s.b[1694]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1689]) && s.b[1690]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign30260_loop_guard: usize = 0;
        while {
            let assign30260_cond_e29581: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1689]) && s.b[1690]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign30260_cond_e29581 != 0.0
        } {
            assign30260_loop_guard += 1;
            assert!(assign30260_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1689]) && s.b[1690]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1689]) && (!s.b[1690])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                }
            });
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1689]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.2);
            s.store_div_scaled_product(334, s.ad_value(725), s.ad_value(726), 0.2, s.ad_value(770), 1.0);
            s.store_sub_from_scalar(344, (0.3 + 0.2), 780);
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1689]) {
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && (!s.b[1689])) {
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && (!s.b[1689])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1682]) {
            s.store_offset(344, 344, (10.0 * 2.220446049250313e-16));
            s.store_div(335, 790, 344);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1682]) {
            s.store_ad_value(336, {
                if (s.v[335] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(335), A::offset(s.ad_value(658), (-1.0)))
                }
            });
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1682]) {
            s.store_mul(341, 336, 335);
            s.store_offset(337, 341, 1.0);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1682]) {
            s.store_ad_value(338, {
                if (s.v[337] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)))
                }
            });
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1682]) {
            s.store_mul(340, 338, 337);
            s.store_div(348, 790, 340);
        }

        s.b[1695] = ((s.v[85] < 0.5) && (0.5 >= 0.0));
        s.v[1695] = if s.b[1695] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1695]) {
            s.store_sub_from_scalar(781, 0.5, 85);
            s.store_square(722, 781);
            s.store_scalar(723, (0.5 * 0.5));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1696] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1696] = if s.b[1696] { 1.0 } else { 0.0 };

        s.b[1697] = (2.0 == 1.0);
        s.v[1697] = if s.b[1697] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1695]) && s.b[1696]) && s.b[1697]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1698] = (2.0 == 2.0);
        s.v[1698] = if s.b[1698] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1695]) && s.b[1696]) && (!s.b[1697])) && s.b[1698]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1699] = (2.0 == 4.0);
        s.v[1699] = if s.b[1699] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1695]) && s.b[1696]) && (!s.b[1697])) && (!s.b[1698])) && s.b[1699]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1700] = (2.0 == 8.0);
        s.v[1700] = if s.b[1700] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1695]) && s.b[1696]) && (!s.b[1697])) && (!s.b[1698])) && (!s.b[1699])) && s.b[1700]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1695]) && s.b[1696]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign30690_loop_guard: usize = 0;
        while {
            let assign30690_cond_e30121: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1695]) && s.b[1696]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign30690_cond_e30121 != 0.0
        } {
            assign30690_loop_guard += 1;
            assert!(assign30690_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1695]) && s.b[1696]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1695]) && (!s.b[1696])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1695]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.5);
            s.store_div_scaled_product(334, s.ad_value(725), s.ad_value(726), 0.5, s.ad_value(770), 1.0);
            s.store_sub_from_scalar(1533, 0.5, 780);
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1695]) {
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && (!s.b[1695])) {
            s.copy_ad(1533, 85);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1682]) {
            s.store_scale(335, 1533, 0.8);
        }

        s.b[1701] = ((s.v[348] > (s.v[1533] - s.v[335])) && (s.v[335] >= 0.0));
        s.v[1701] = if s.b[1701] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1701]) {
            s.store_add_scaled_inputs3(781, s.ad_value(348), 1.0, s.ad_value(1533), (-1.0), s.ad_value(335), 1.0);
            s.store_square(722, 781);
            s.store_square(723, 335);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1702] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1702] = if s.b[1702] { 1.0 } else { 0.0 };

        s.b[1703] = (2.0 == 1.0);
        s.v[1703] = if s.b[1703] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1701]) && s.b[1702]) && s.b[1703]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1704] = (2.0 == 2.0);
        s.v[1704] = if s.b[1704] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1701]) && s.b[1702]) && (!s.b[1703])) && s.b[1704]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1705] = (2.0 == 4.0);
        s.v[1705] = if s.b[1705] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1701]) && s.b[1702]) && (!s.b[1703])) && (!s.b[1704])) && s.b[1705]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1706] = (2.0 == 8.0);
        s.v[1706] = if s.b[1706] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1701]) && s.b[1702]) && (!s.b[1703])) && (!s.b[1704])) && (!s.b[1705])) && s.b[1706]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1701]) && s.b[1702]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign31050_loop_guard: usize = 0;
        while {
            let assign31050_cond_e30573: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1701]) && s.b[1702]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign31050_cond_e30573 != 0.0
        } {
            assign31050_loop_guard += 1;
            assert!(assign31050_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1701]) && s.b[1702]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1701]) && (!s.b[1702])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1701]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_ad_value(334, A::div_scaled_product3(s.ad_value(335), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0));
            s.store_add_scaled_inputs3(790, s.ad_value(1533), 1.0, s.ad_value(335), (-1.0), s.ad_value(780), 1.0);
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && s.b[1701]) {
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1682]) && (!s.b[1701])) {
            s.copy_ad(790, 348);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1682])) {
            s.copy_ad(348, 790);
        }

        s.b[1707] = (s.v[790] <= 0.0);
        s.v[1707] = if s.b[1707] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1707]) {
            s.copy_ad(1458, 1457);
            s.copy_ad(1480, 1479);
            s.copy_ad(1461, 1460);
            s.copy_ad(1474, 1473);
            s.copy_ad(1535, 1534);
            s.copy_ad(1495, 1493);
            s.copy_ad(1496, 1494);
            s.copy_ad(1514, 1513);
            s.copy_ad(1512, 1511);
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {
            s.store_sqrt_mul_ad(1450, A::div_scaled_product(s.ad_value(1543), s.ad_value(622), 1.0, A::add(s.ad_value(622), s.ad_value(964)), 1.0), A::add_scaled_inputs3(s.ad_value(790), 1.0, s.ad_value(1431), (-1.0), s.ad_value(1459), 1.0));
        }

        s.b[1708] = (s.v[1450] > s.v[965]);
        s.v[1708] = if s.b[1708] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) {
            s.copy_ad(1462, 790);
        }

    }

    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
    ) {
        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) {
            s.copy_ad(1444, 965);
            s.copy_ad(1480, 790);
            s.copy_ad(1508, 790);
            s.store_sub_ad_rhs(1461, 1480, A::mul3(s.ad_value(1544), s.ad_value(1444), s.ad_value(1444)));
            s.copy_ad(1506, 1462);
            s.copy_ad(1469, 1461);
            s.store_mul(1495, 1444, 1542);
            s.store_scalar(97, 1.0);
        }

        let mut assign31360_loop_guard: usize = 0;
        while {
            let assign31360_cond_e30940: f64 = (150.0 + 1.0);
            let assign31360_cond_e30942: f64 = if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && (s.v[97] <= assign31360_cond_e30940)) { 1.0 } else { 0.0 };
            assign31360_cond_e30942 != 0.0
        } {
            assign31360_loop_guard += 1;
            assert!(assign31360_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) {
                s.store_sqrt_mul_ad(1444, s.ad_value(1543), A::sub(s.ad_value(1480), s.ad_value(1461)));
            }
            s.b[1709] = ((s.v[1444] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.v[1709] = if s.b[1709] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1709]) {
                s.store_offset_sub(781, 1444, 965, 1e-8);
                s.store_square(722, 781);
                s.store_scalar(723, (1e-8 * 1e-8));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1710] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1710] = if s.b[1710] { 1.0 } else { 0.0 };
            s.b[1711] = (2.0 == 1.0);
            s.v[1711] = if s.b[1711] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1709]) && s.b[1710]) && s.b[1711]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1712] = (2.0 == 2.0);
            s.v[1712] = if s.b[1712] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1709]) && s.b[1710]) && (!s.b[1711])) && s.b[1712]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1713] = (2.0 == 4.0);
            s.v[1713] = if s.b[1713] { 1.0 } else { 0.0 };
            if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1709]) && s.b[1710]) && (!s.b[1711])) && (!s.b[1712])) && s.b[1713]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1714] = (2.0 == 8.0);
            s.v[1714] = if s.b[1714] { 1.0 } else { 0.0 };
            if (((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1709]) && s.b[1710]) && (!s.b[1711])) && (!s.b[1712])) && (!s.b[1713])) && s.b[1714]) {
                s.store_scalar(720, 4.0);
            }
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1709]) && s.b[1710]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign31360_body27_loop_guard: usize = 0;
            while {
                let assign31360_body27_cond_e31324: f64 = if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1709]) && s.b[1710]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign31360_body27_cond_e31324 != 0.0
            } {
                assign31360_body27_loop_guard += 1;
                assert!(assign31360_body27_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1709]) && s.b[1710]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1709]) && (!s.b[1710])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1709]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product(334, s.ad_value(725), s.ad_value(726), 1e-8, s.ad_value(770), 1.0);
                s.store_add_ad_lhs(1444, A::offset(s.ad_value(965), (-1e-8)), 780);
            }
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1709]) {
            }
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && (!s.b[1709])) {
            }
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && (!s.b[1709])) {
                s.store_scalar(334, 1.0);
            }
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) {
                s.store_add_scaled_inputs3(335, s.ad_value(1461), 1.0, s.ad_value(1431), (-1.0), s.ad_value(1459), 1.0);
            }
            s.b[1715] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1715] = if s.b[1715] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1715]) {
                s.store_sub_from_scalar(781, 0.1, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.1 * 0.1));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1716] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1716] = if s.b[1716] { 1.0 } else { 0.0 };
            s.b[1717] = (2.0 == 1.0);
            s.v[1717] = if s.b[1717] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1715]) && s.b[1716]) && s.b[1717]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1718] = (2.0 == 2.0);
            s.v[1718] = if s.b[1718] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1715]) && s.b[1716]) && (!s.b[1717])) && s.b[1718]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1719] = (2.0 == 4.0);
            s.v[1719] = if s.b[1719] { 1.0 } else { 0.0 };
            if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1715]) && s.b[1716]) && (!s.b[1717])) && (!s.b[1718])) && s.b[1719]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1720] = (2.0 == 8.0);
            s.v[1720] = if s.b[1720] { 1.0 } else { 0.0 };
            if (((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1715]) && s.b[1716]) && (!s.b[1717])) && (!s.b[1718])) && (!s.b[1719])) && s.b[1720]) {
                s.store_scalar(720, 4.0);
            }
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1715]) && s.b[1716]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign31360_body63_loop_guard: usize = 0;
            while {
                let assign31360_body63_cond_e31874: f64 = if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1715]) && s.b[1716]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign31360_body63_cond_e31874 != 0.0
            } {
                assign31360_body63_loop_guard += 1;
                assert!(assign31360_body63_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1715]) && s.b[1716]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1715]) && (!s.b[1716])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1715]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product(341, s.ad_value(725), s.ad_value(726), 0.1, s.ad_value(770), 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1715]) {
            }
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && (!s.b[1715])) {
                s.copy_ad(336, 335);
                s.store_scalar(341, 1.0);
            }
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) {
                s.store_sqrt_mul(1448, 1546, 336);
                s.store_mul(1495, 1444, 1542);
                s.store_mul_div_from_scalar_lhs(1526, (-1.034943e-10), 1444, 334);
                s.store_mul_neg_lhs(1496, 1448, 1540);
                s.store_mul_div_from_scalar_lhs(1528, (-1.034943e-10), 1448, 341);
                s.store_add_ad_lhs(1481, A::add_scaled_product(s.ad_value(1495), 1.0, s.ad_value(185), A::sub(s.ad_value(1462), s.ad_value(1480)), 1.0), 1496);
                s.copy_ad(1483, 185);
                s.store_add(1484, 1526, 1528);
                s.store_add_scaled_product(1482, s.ad_value(1461), 1.0, s.ad_value(1531), A::sub(A::add_scaled_product(s.ad_value(1431), 1.0, s.ad_value(1532), s.ad_value(1480), 1.0), s.ad_value(1459)), (-1.0));
                s.store_scalar(1485, 0.0);
                s.store_scalar(1486, 1.0);
                s.store_add_scaled_products(1487, s.ad_value(1483), s.ad_value(1486), 1.0, s.ad_value(1485), s.ad_value(1484), (-1.0));
                s.store_div(1488, 1486, 1487);
                s.store_div_scaled_inputs(1489, s.ad_value(1484), -1.0, s.ad_value(1487), 1.0);
                s.store_div_scaled_inputs(1490, s.ad_value(1485), -1.0, s.ad_value(1487), 1.0);
                s.store_div(1491, 1483, 1487);
            }
            s.b[1721] = (((((s.v[1488] * s.v[1481]) + (s.v[1489] * s.v[1482]))) as f64).abs() > 0.5);
            s.v[1721] = if s.b[1721] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1721]) {
                s.store_offset(1462, 1462, (-(0.5 * (if (((s.v[1488] * s.v[1481]) + (s.v[1489] * s.v[1482])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1721]) {
                s.store_offset(1461, 1461, (-(0.5 * (if (((s.v[1490] * s.v[1481]) + (s.v[1491] * s.v[1482])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && (!s.b[1721])) {
                s.store_sub_ad_rhs(1462, 1462, A::add_scaled_products(s.ad_value(1488), s.ad_value(1481), 1.0, s.ad_value(1489), s.ad_value(1482), 1.0));
                s.store_sub_ad_rhs(1461, 1461, A::add_scaled_products(s.ad_value(1490), s.ad_value(1481), 1.0, s.ad_value(1491), s.ad_value(1482), 1.0));
            }
            s.b[1722] = (((((s.v[1462] - s.v[1506])) as f64).abs() <= 1e-12) && ((((s.v[1461] - s.v[1469])) as f64).abs() <= 1e-12));
            s.v[1722] = if s.b[1722] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1722]) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) {
                s.copy_ad(1506, 1462);
                s.copy_ad(1469, 1461);
                s.store_offset(97, 97, 1.0);
            }
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) {
            s.copy_ad(1510, 1461);
            s.store_mul(1448, 965, 1532);
            s.store_add_scaled_inputs3(1461, A::mul3(s.ad_value(1547), s.ad_value(1448), s.ad_value(1448)), 1.0, s.ad_value(1431), 1.0, s.ad_value(1459), -1.0);
            s.store_add_scaled_product(1480, s.ad_value(1461), 1.0, s.ad_value(1544), s.ad_value(1539), 1.0);
            s.copy_ad(1458, 1480);
            s.copy_ad(1463, 1480);
            s.copy_ad(1505, 1480);
        }

        s.b[1723] = (s.v[85] > s.v[1462]);
        s.v[1723] = if s.b[1723] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && s.b[1723]) {
            s.store_scalar(1475, 1.0);
        }

        s.b[1724] = (s.v[85] > s.v[1505]);
        s.v[1724] = if s.b[1724] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && (!s.b[1723])) && s.b[1724]) {
            s.store_scalar(1475, 3.0);
        }

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1708]) && (!s.b[1723])) && (!s.b[1724])) {
            s.store_scalar(1475, 2.0);
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1708])) {
            s.copy_ad(1462, 790);
            s.copy_ad(1505, 1462);
            s.copy_ad(1463, 1462);
            s.copy_ad(1508, 1462);
            s.copy_ad(1444, 1450);
            s.store_mul(1448, 1444, 1532);
            s.store_add_scaled_inputs3(1461, A::mul3(s.ad_value(1547), s.ad_value(1448), s.ad_value(1448)), 1.0, s.ad_value(1431), 1.0, s.ad_value(1459), -1.0);
            s.store_add_ad_lhs(1480, A::mul3(s.ad_value(1544), s.ad_value(1444), s.ad_value(1444)), 1461);
            s.copy_ad(1510, 1461);
        }

        s.b[1725] = (s.v[85] > s.v[1462]);
        s.v[1725] = if s.b[1725] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1708])) && s.b[1725]) {
            s.store_scalar(1475, 1.0);
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1708])) && (!s.b[1725])) {
            s.store_scalar(1475, 2.0);
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {
            s.store_mul_ad_rhs(335, 1545, A::add_scaled_inputs3(s.ad_value(1463), 1.0, s.ad_value(1431), -1.0, s.ad_value(961), 1.0));
        }

        s.b[1726] = (s.v[335] > 0.0);
        s.v[1726] = if s.b[1726] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1726]) {
            s.store_add_scaled_inputs3(1451, s.ad_value(1431), 1.0, s.ad_value(961), (-1.0), A::div(A::sqrt(s.ad_value(335)), s.ad_value(185)), -1.0);
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1726])) {
            s.store_sub(1451, 1431, 961);
        }

        s.b[1727] = (s.v[85] > s.v[1462]);
        s.v[1727] = if s.b[1727] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1727]) {
            s.copy_ad(1461, 1510);
            s.copy_ad(1480, 790);
            s.store_add_ad_lhs(1477, A::div(A::ln(A::mul3(s.ad_value(1452), s.ad_value(85), s.ad_value(85))), A::add(s.ad_value(154), A::div_from_scalar(2.0, s.ad_value(85)))), 790);
        }

        s.b[1728] = (s.v[1477] < (s.v[1508] + s.v[1549]));
        s.v[1728] = if s.b[1728] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1727]) && s.b[1728]) {
            s.store_add(1477, 1508, 1549);
        }

        s.b[1729] = (s.v[85] > s.v[1505]);
        s.v[1729] = if s.b[1729] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && s.b[1729]) {
            s.copy_ad(1477, 1458);
        }

        s.b[1730] = (s.v[85] > s.v[1451]);
        s.v[1730] = if s.b[1730] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) {
            s.store_add_scaled_product(1453, s.ad_value(154), 1.0, s.ad_value(1452), s.ad_value(85), (-2.0));
            s.store_add_scaled_product(1454, A::mul3(s.ad_value(1452), s.ad_value(85), s.ad_value(85)), 1.0, s.ad_value(154), s.ad_value(1480), (-1.0));
            s.copy_ad(1467, 1480);
            s.store_div_scaled_inputs2(1477, A::sqrt(A::add_scaled_square_product(s.ad_value(1453), 1.0, s.ad_value(1452), s.ad_value(1454), (-4.0))), 0.5, s.ad_value(1453), (-0.5), s.ad_value(1452), 1.0);
        }

        s.b[1731] = (s.v[1477] > (s.v[1463] - s.v[1549]));
        s.v[1731] = if s.b[1731] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
    ) {
        if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1731]) {
            s.store_sub(1477, 1463, 1549);
        }

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) {
            s.store_sqrt_mul_ad(1446, s.ad_value(1543), A::sub(s.ad_value(1480), s.ad_value(1477)));
            s.store_sqrt_mul_ad(1444, s.ad_value(1543), A::sub(s.ad_value(1480), s.ad_value(1461)));
        }

        s.b[1732] = ((s.v[1446] + s.v[1444]) > s.v[965]);
        s.v[1732] = if s.b[1732] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) {
            s.store_scalar(97, 1.0);
        }

        let mut assign31840_loop_guard: usize = 0;
        while {
            let assign31840_cond_e33102: f64 = (150.0 + 1.0);
            let assign31840_cond_e33104: f64 = if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) && (s.v[97] <= assign31840_cond_e33102)) { 1.0 } else { 0.0 };
            assign31840_cond_e33104 != 0.0
        } {
            assign31840_loop_guard += 1;
            assert!(assign31840_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) {
                s.store_add_scaled_inputs3(1464, s.ad_value(1446), 1.0, s.ad_value(1444), 1.0, s.ad_value(965), -1.0);
                s.store_add_ad(1504, A::div_scalar_by_product(1.034943e-10, s.ad_value(1542), s.ad_value(1446), 1.0), A::div(A::mul_sub_from_scalar_rhs(A::div_from_scalar(1.034943e-10, s.ad_value(1542)), 1.0, A::div_scaled_value_offset_denominator(s.ad_value(1532), 1.0, s.ad_value(1532), 1.0, 1.0)), s.ad_value(1444)));
            }
            s.b[1733] = ((((s.v[1464] / s.v[1504])) as f64).abs() > 0.5);
            s.v[1733] = if s.b[1733] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) && s.b[1733]) {
                s.store_offset(1480, 1480, (-(0.5 * (if ((s.v[1464] / s.v[1504]) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) && (!s.b[1733])) {
                s.store_sub_ad_rhs(1480, 1480, A::div(s.ad_value(1464), s.ad_value(1504)));
            }
            s.b[1734] = (((s.v[1480] - s.v[1431]) + s.v[1459]) < (10.0 * 2.220446049250313e-16));
            s.v[1734] = if s.b[1734] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) && s.b[1734]) {
                s.store_offset_sub(1480, 1431, 1459, (10.0 * 2.220446049250313e-16));
            }
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) {
                s.store_add_scaled_product(1454, A::mul3(s.ad_value(1452), s.ad_value(85), s.ad_value(85)), 1.0, s.ad_value(154), s.ad_value(1480), (-1.0));
                s.store_add_scaled_square_product(335, s.ad_value(1453), 1.0, s.ad_value(1452), s.ad_value(1454), (-4.0));
            }
            s.b[1735] = (s.v[335] > 0.0);
            s.v[1735] = if s.b[1735] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) && s.b[1735]) {
                s.store_div_scaled_inputs2(1477, A::sqrt(s.ad_value(335)), 0.5, s.ad_value(1453), (-0.5), s.ad_value(1452), 1.0);
            }
            if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) && (!s.b[1735])) {
                s.store_div_scaled_inputs(1477, s.ad_value(1453), (-0.5), s.ad_value(1452), 1.0);
            }
            s.b[1736] = (s.v[1477] > s.v[1463]);
            s.v[1736] = if s.b[1736] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) && s.b[1736]) {
                s.copy_ad(1477, 1463);
            }
            s.b[1737] = (s.v[1477] > s.v[1480]);
            s.v[1737] = if s.b[1737] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) && s.b[1737]) {
                s.store_sub(1477, 1480, 1549);
                s.store_scalar(97, (150.0 + 1.0));
            }
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) {
                s.store_sqrt_mul_ad(1446, s.ad_value(1543), A::sub(s.ad_value(1480), s.ad_value(1477)));
                s.store_div_scaled_inputs2(1461, A::add_scaled_product(s.ad_value(1431), 1.0, s.ad_value(1532), s.ad_value(1480), 1.0), 1.0, s.ad_value(1459), (-1.0), A::offset(s.ad_value(1532), 1.0), 1.0);
                s.store_sqrt_mul_ad(1444, s.ad_value(1543), A::sub(s.ad_value(1480), s.ad_value(1461)));
            }
            s.b[1738] = ((((s.v[1480] - s.v[1467])) as f64).abs() <= 1e-8);
            s.v[1738] = if s.b[1738] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) && s.b[1738]) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) && s.b[1732]) {
                s.copy_ad(1467, 1480);
                s.store_offset(97, 97, 1.0);
            }
        }

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1727])) && (!s.b[1729])) && (!s.b[1730])) {
            s.copy_ad(1480, 1479);
            s.copy_ad(1461, 1460);
            s.copy_ad(1477, 1457);
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {
            s.copy_ad(1478, 1480);
            s.store_scalar(79, 0.0);
            s.copy_ad(1458, 1477);
            s.copy_ad(1480, 1478);
            s.copy_ad(1470, 1458);
            s.copy_ad(1467, 1480);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
    ) {
        let mut assign31950_loop_guard: usize = 0;
        while {
            let assign31950_cond_e33724: f64 = (150.0 + 1.0);
            let assign31950_cond_e33726: f64 = if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (s.v[97] <= assign31950_cond_e33724)) { 1.0 } else { 0.0 };
            assign31950_cond_e33726 != 0.0
        } {
            assign31950_loop_guard += 1;
            assert!(assign31950_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {
                s.store_mul_sub_ad_rhs(1461, 1531, A::add_scaled_product(s.ad_value(1431), 1.0, s.ad_value(1532), s.ad_value(1480), 1.0), s.ad_value(1459));
                s.store_mul(1530, 1531, 1532);
                s.store_sub(335, 1480, 1461);
            }
            s.b[1739] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1739] = if s.b[1739] { 1.0 } else { 0.0 };
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) {
                s.store_sub_from_scalar(781, 0.1, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.1 * 0.1));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1740] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1740] = if s.b[1740] { 1.0 } else { 0.0 };
            s.b[1741] = (2.0 == 1.0);
            s.v[1741] = if s.b[1741] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) && s.b[1740]) && s.b[1741]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1742] = (2.0 == 2.0);
            s.v[1742] = if s.b[1742] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) && s.b[1740]) && (!s.b[1741])) && s.b[1742]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1743] = (2.0 == 4.0);
            s.v[1743] = if s.b[1743] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) && s.b[1740]) && (!s.b[1741])) && (!s.b[1742])) && s.b[1743]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1744] = (2.0 == 8.0);
            s.v[1744] = if s.b[1744] { 1.0 } else { 0.0 };
            if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) && s.b[1740]) && (!s.b[1741])) && (!s.b[1742])) && (!s.b[1743])) && s.b[1744]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) && s.b[1740]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign31950_body29_loop_guard: usize = 0;
            while {
                let assign31950_body29_cond_e34089: f64 = if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) && s.b[1740]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign31950_body29_cond_e34089 != 0.0
            } {
                assign31950_body29_loop_guard += 1;
                assert!(assign31950_body29_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) && s.b[1740]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) && (!s.b[1740])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product(334, s.ad_value(725), s.ad_value(726), 0.1, s.ad_value(770), 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1739]) {
            }
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1739])) {
                s.copy_ad(336, 335);
                s.store_scalar(334, 1.0);
            }
            if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {
                s.store_sqrt_mul(1444, 1543, 336);
            }
            s.b[1745] = ((s.v[1444] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.v[1745] = if s.b[1745] { 1.0 } else { 0.0 };
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) {
                s.store_offset_sub(781, 1444, 965, 1e-8);
                s.store_square(722, 781);
                s.store_scalar(723, (1e-8 * 1e-8));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1746] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1746] = if s.b[1746] { 1.0 } else { 0.0 };
            s.b[1747] = (2.0 == 1.0);
            s.v[1747] = if s.b[1747] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) && s.b[1746]) && s.b[1747]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1748] = (2.0 == 2.0);
            s.v[1748] = if s.b[1748] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) && s.b[1746]) && (!s.b[1747])) && s.b[1748]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1749] = (2.0 == 4.0);
            s.v[1749] = if s.b[1749] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) && s.b[1746]) && (!s.b[1747])) && (!s.b[1748])) && s.b[1749]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1750] = (2.0 == 8.0);
            s.v[1750] = if s.b[1750] { 1.0 } else { 0.0 };
            if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) && s.b[1746]) && (!s.b[1747])) && (!s.b[1748])) && (!s.b[1749])) && s.b[1750]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) && s.b[1746]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign31950_body65_loop_guard: usize = 0;
            while {
                let assign31950_body65_cond_e34574: f64 = if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) && s.b[1746]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign31950_body65_cond_e34574 != 0.0
            } {
                assign31950_body65_loop_guard += 1;
                assert!(assign31950_body65_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) && s.b[1746]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) && (!s.b[1746])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product(337, s.ad_value(725), s.ad_value(726), 1e-8, s.ad_value(770), 1.0);
                s.store_add_ad_lhs(1444, A::offset(s.ad_value(965), (-1e-8)), 780);
            }
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1745]) {
            }
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1745])) {
            }
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1745])) {
                s.store_scalar(337, 1.0);
            }
            if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {
                s.store_sqrt_mul_ad(1448, s.ad_value(1546), A::add_scaled_inputs3(s.ad_value(1461), 1.0, s.ad_value(1431), (-1.0), s.ad_value(1459), 1.0));
                s.store_mul(1495, 1444, 1542);
                s.store_mul_ad_product_lhs(1524, A::div_from_scalar(1.034943e-10, s.ad_value(1444)), s.ad_value(334), 337);
                s.store_mul_ad_product_lhs(1526, A::div_from_scalar((-1.034943e-10), s.ad_value(1444)), s.ad_value(334), 337);
                s.store_mul_neg_lhs(1496, 1448, 1540);
                s.store_div_from_scalar(1528, (-1.034943e-10), 1448);
                s.store_scaled_mul(335, 1498, 1539, 8.0);
            }
            if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {
                let assign31950_body81_ad_e34863: A = A::add(A::add_scaled_inputs4(A::mul3_scaled_output(s.ad_value(1461), s.ad_value(1461), s.ad_value(1538), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1461), s.ad_value(1538), s.ad_value(1458), 8.0), (-1.0), A::mul3_scaled_output(s.ad_value(1538), s.ad_value(1458), s.ad_value(1458), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1461), s.ad_value(1498), s.ad_value(1539), 4.0), 1.0), A::mul3_scaled_output(s.ad_value(1458), s.ad_value(1498), s.ad_value(1539), 4.0));
                s.store_ad_value(1516, A::div_scaled_add_product(assign31950_body81_ad_e34863, 1.0, A::mul3(s.ad_value(1541), s.ad_value(1537), s.ad_value(1539)), s.ad_value(1539), 1.0, s.ad_value(335), 1.0));
            }
            if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {
                s.store_div_ad_lhs(1517, A::add_scaled_products3(s.ad_value(1461), s.ad_value(1538), (-8.0), s.ad_value(1538), s.ad_value(1458), (4.0 * 2.0), s.ad_value(1498), s.ad_value(1539), 4.0), 335);
                s.store_div_ad_lhs(1518, A::add_scaled_products3(s.ad_value(1461), s.ad_value(1538), (4.0 * 2.0), s.ad_value(1538), s.ad_value(1458), (-8.0), s.ad_value(1498), s.ad_value(1539), 4.0), 335);
                s.store_mul_sub_rhs(335, 154, 1458, 1480);
                s.store_exp(336, 335);
            }
            s.b[1751] = (s.v[1458] >= s.v[1480]);
            s.v[1751] = if s.b[1751] { 1.0 } else { 0.0 };
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1751]) {
                s.store_mul_scaled_ad_rhs(1472, 209, -1.0, A::sqrt(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15)));
                s.store_mul_ad(1520, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(1472), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
                s.store_neg(1522, 1520);
            }
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1751])) {
                s.store_exp_ad(337, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1458), s.ad_value(1431))));
                s.store_exp_ad(338, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1480), s.ad_value(1431))));
                s.store_mul_sqrt_ad_rhs(1472, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product(339, s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(1472), 1.0);
                s.store_mul_add_ad_rhs(1520, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
                s.store_mul_add_ad_rhs(1522, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));
            }
            s.b[1752] = ((s.v[1516] > (s.v[1508] - s.v[1515])) && (s.v[1515] >= 0.0));
            s.v[1752] = if s.b[1752] { 1.0 } else { 0.0 };
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) {
                s.store_add_scaled_inputs3(781, s.ad_value(1516), 1.0, s.ad_value(1508), (-1.0), s.ad_value(1515), 1.0);
                s.store_square(722, 781);
                s.store_square(723, 1515);
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1753] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.v[1753] = if s.b[1753] { 1.0 } else { 0.0 };
            s.b[1754] = (4.0 == 1.0);
            s.v[1754] = if s.b[1754] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) && s.b[1753]) && s.b[1754]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1755] = (4.0 == 2.0);
            s.v[1755] = if s.b[1755] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) && s.b[1753]) && (!s.b[1754])) && s.b[1755]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1756] = (4.0 == 4.0);
            s.v[1756] = if s.b[1756] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) && s.b[1753]) && (!s.b[1754])) && (!s.b[1755])) && s.b[1756]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1757] = (4.0 == 8.0);
            s.v[1757] = if s.b[1757] { 1.0 } else { 0.0 };
            if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) && s.b[1753]) && (!s.b[1754])) && (!s.b[1755])) && (!s.b[1756])) && s.b[1757]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) && s.b[1753]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign31950_body126_loop_guard: usize = 0;
            while {
                let assign31950_body126_cond_e35523: f64 = if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) && s.b[1753]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign31950_body126_cond_e35523 != 0.0
            } {
                assign31950_body126_loop_guard += 1;
                assert!(assign31950_body126_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) && s.b[1753]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) && (!s.b[1753])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                    }
                });
            }
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_mul3_lhs(780, 781, 1515, 726);
                s.store_ad_value(334, A::div_scaled_product3(s.ad_value(1515), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0));
                s.store_add_scaled_inputs3(335, s.ad_value(1508), 1.0, s.ad_value(1515), (-1.0), s.ad_value(780), 1.0);
            }
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1752]) {
            }
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1752])) {
                s.copy_ad(335, 1516);
                s.store_scalar(334, 1.0);
            }
            if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {
                s.store_sub(1481, 1480, 335);
                s.store_mul_neg_lhs(1483, 1517, 334);
                s.store_sub_from_scalar_ad(1484, 1.0, A::mul3(s.ad_value(1518), s.ad_value(1530), s.ad_value(334)));
                s.store_add_scaled_inputs3(1482, A::add_scaled_product(s.ad_value(1472), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1458)), 1.0), 1.0, s.ad_value(1495), 1.0, s.ad_value(1496), 1.0);
                s.store_sub(1485, 1520, 185);
                s.store_ad_value(1486, A::add_scaled_inputs_products(s.ad_value(1522), 1.0, s.ad_value(1524), 1.0, s.ad_value(1526), s.ad_value(1530), 1.0, s.ad_value(1528), s.ad_value(1530), 1.0));
                s.store_add_scaled_products(1487, s.ad_value(1483), s.ad_value(1486), 1.0, s.ad_value(1485), s.ad_value(1484), (-1.0));
                s.store_div(1488, 1486, 1487);
                s.store_div_scaled_inputs(1489, s.ad_value(1484), -1.0, s.ad_value(1487), 1.0);
                s.store_div_scaled_inputs(1490, s.ad_value(1485), -1.0, s.ad_value(1487), 1.0);
                s.store_div(1491, 1483, 1487);
            }
            s.b[1758] = (((((s.v[1490] * s.v[1481]) + (s.v[1491] * s.v[1482]))) as f64).abs() > 0.5);
            s.v[1758] = if s.b[1758] { 1.0 } else { 0.0 };
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1758]) {
                s.store_offset(1458, 1458, (-(0.5 * (if (((s.v[1488] * s.v[1481]) + (s.v[1489] * s.v[1482])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1758]) {
                s.store_offset(1480, 1480, (-(0.5 * (if (((s.v[1490] * s.v[1481]) + (s.v[1491] * s.v[1482])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1758])) {
                s.store_sub_ad_rhs(1458, 1458, A::add_scaled_products(s.ad_value(1488), s.ad_value(1481), 1.0, s.ad_value(1489), s.ad_value(1482), 1.0));
                s.store_sub_ad_rhs(1480, 1480, A::add_scaled_products(s.ad_value(1490), s.ad_value(1481), 1.0, s.ad_value(1491), s.ad_value(1482), 1.0));
            }
            s.b[1759] = (((((s.v[1458] - s.v[1470])) as f64).abs() <= 1e-12) && ((((s.v[1480] - s.v[1467])) as f64).abs() <= 1e-12));
            s.v[1759] = if s.b[1759] { 1.0 } else { 0.0 };
            if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1759]) {
                s.store_scalar(97, (150.0 + 1.0));
                s.store_scalar(79, 1.0);
            }
            if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {
                s.copy_ad(1470, 1458);
                s.copy_ad(1467, 1480);
                s.store_offset(97, 97, 1.0);
            }
        }

        s.b[1761] = ((s.v[1450] > s.v[965]) && (s.v[1475] != 2.0));
        s.v[1761] = if s.b[1761] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
    ) {
        s.b[1762] = ((s.v[1480] > (s.v[1458] - 0.02)) && (0.02 >= 0.0));
        s.v[1762] = if s.b[1762] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) {
            s.store_offset_sub(781, 1480, 1458, 0.02);
            s.store_square(722, 781);
            s.store_scalar(723, (0.02 * 0.02));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1763] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1763] = if s.b[1763] { 1.0 } else { 0.0 };

        s.b[1764] = (2.0 == 1.0);
        s.v[1764] = if s.b[1764] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) && s.b[1763]) && s.b[1764]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1765] = (2.0 == 2.0);
        s.v[1765] = if s.b[1765] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) && s.b[1763]) && (!s.b[1764])) && s.b[1765]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1766] = (2.0 == 4.0);
        s.v[1766] = if s.b[1766] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) && s.b[1763]) && (!s.b[1764])) && (!s.b[1765])) && s.b[1766]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1767] = (2.0 == 8.0);
        s.v[1767] = if s.b[1767] { 1.0 } else { 0.0 };

        if (((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) && s.b[1763]) && (!s.b[1764])) && (!s.b[1765])) && (!s.b[1766])) && s.b[1767]) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) && s.b[1763]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign32240_loop_guard: usize = 0;
        while {
            let assign32240_cond_e36367: f64 = if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) && s.b[1763]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign32240_cond_e36367 != 0.0
        } {
            assign32240_loop_guard += 1;
            assert!(assign32240_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) && s.b[1763]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) && (!s.b[1763])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.02);
            s.store_div_scaled_product(335, s.ad_value(725), s.ad_value(726), 0.02, s.ad_value(770), 1.0);
            s.store_add_ad_lhs(1480, A::offset(s.ad_value(1458), (-0.02)), 780);
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && s.b[1762]) {
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && (!s.b[1762])) {
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1761]) && (!s.b[1762])) {
            s.store_scalar(335, 1.0);
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {
            s.store_mul_sub_ad_rhs(1461, 1531, A::add_scaled_product(s.ad_value(1431), 1.0, s.ad_value(1532), s.ad_value(1480), 1.0), s.ad_value(1459));
            s.store_mul_sub_rhs(335, 154, 1458, 1480);
            s.store_exp(336, 335);
        }

        s.b[1768] = (s.v[1458] >= s.v[1480]);
        s.v[1768] = if s.b[1768] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) {
            s.store_mul_scaled_ad_rhs(1472, 209, -1.0, A::sqrt(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15)));
            s.copy_ad(1535, 1472);
            s.store_scalar(1514, 0.0);
            s.store_scalar(1474, 0.0);
            s.store_sqrt_mul_ad(1444, s.ad_value(1543), A::sub(s.ad_value(1480), s.ad_value(1461)));
        }

        s.b[1769] = ((s.v[1444] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
        s.v[1769] = if s.b[1769] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) {
            s.store_offset_sub(781, 1444, 965, 1e-8);
            s.store_square(722, 781);
            s.store_scalar(723, (1e-8 * 1e-8));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1770] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1770] = if s.b[1770] { 1.0 } else { 0.0 };

        s.b[1771] = (2.0 == 1.0);
        s.v[1771] = if s.b[1771] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) && s.b[1770]) && s.b[1771]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1772] = (2.0 == 2.0);
        s.v[1772] = if s.b[1772] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) && s.b[1770]) && (!s.b[1771])) && s.b[1772]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1773] = (2.0 == 4.0);
        s.v[1773] = if s.b[1773] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) && s.b[1770]) && (!s.b[1771])) && (!s.b[1772])) && s.b[1773]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1774] = (2.0 == 8.0);
        s.v[1774] = if s.b[1774] { 1.0 } else { 0.0 };

        if (((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) && s.b[1770]) && (!s.b[1771])) && (!s.b[1772])) && (!s.b[1773])) && s.b[1774]) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) && s.b[1770]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign32680_loop_guard: usize = 0;
        while {
            let assign32680_cond_e37015: f64 = if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) && s.b[1770]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign32680_cond_e37015 != 0.0
        } {
            assign32680_loop_guard += 1;
            assert!(assign32680_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) && s.b[1770]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) && (!s.b[1770])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product(337, s.ad_value(725), s.ad_value(726), 1e-8, s.ad_value(770), 1.0);
            s.store_add_ad_lhs(1444, A::offset(s.ad_value(965), (-1e-8)), 780);
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && s.b[1769]) {
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && (!s.b[1769])) {
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) && (!s.b[1769])) {
            s.store_scalar(337, 1.0);
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1768]) {
            s.store_sqrt_mul_ad(1448, s.ad_value(1546), A::add_scaled_inputs3(s.ad_value(1461), 1.0, s.ad_value(1431), (-1.0), s.ad_value(1459), 1.0));
            s.store_mul(1495, 1444, 1542);
            s.store_mul_neg_lhs(1496, 1448, 1540);
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) {
            s.store_exp_ad(337, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1458), s.ad_value(1431))));
            s.store_exp_ad(338, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1480), s.ad_value(1431))));
            s.store_mul_sqrt_ad_rhs(1472, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
        }

        s.b[1775] = ((s.v[1450] > s.v[965]) && (s.v[1475] != 2.0));
        s.v[1775] = if s.b[1775] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1775]) {
            s.store_scalar(1474, 0.0);
            s.store_scalar(1514, 0.0);
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && (!s.b[1775])) {
            s.store_mul_sqrt_ad_rhs(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1458), s.ad_value(1431)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1480), s.ad_value(1431)))))), s.ad_value(335)));
            s.store_add_scaled_product(1474, s.ad_value(337), 1.0, s.ad_value(209), A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));
            s.store_mul_sqrt_ad_rhs(1514, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) {
            s.store_scalar(1535, 0.0);
            s.store_sub(335, 1480, 1461);
        }

        s.b[1776] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
        s.v[1776] = if s.b[1776] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) {
            s.store_sub_from_scalar(781, 0.1, 335);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1777] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1777] = if s.b[1777] { 1.0 } else { 0.0 };

        s.b[1778] = (2.0 == 1.0);
        s.v[1778] = if s.b[1778] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) && s.b[1777]) && s.b[1778]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1779] = (2.0 == 2.0);
        s.v[1779] = if s.b[1779] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) && s.b[1777]) && (!s.b[1778])) && s.b[1779]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1780] = (2.0 == 4.0);
        s.v[1780] = if s.b[1780] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) && s.b[1777]) && (!s.b[1778])) && (!s.b[1779])) && s.b[1780]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1781] = (2.0 == 8.0);
        s.v[1781] = if s.b[1781] { 1.0 } else { 0.0 };

        if (((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) && s.b[1777]) && (!s.b[1778])) && (!s.b[1779])) && (!s.b[1780])) && s.b[1781]) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) && s.b[1777]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign33170_loop_guard: usize = 0;
        while {
            let assign33170_cond_e37822: f64 = if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) && s.b[1777]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign33170_cond_e37822 != 0.0
        } {
            assign33170_loop_guard += 1;
            assert!(assign33170_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) && s.b[1777]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) && (!s.b[1777])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

    }

    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
    ) {
        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product(334, s.ad_value(725), s.ad_value(726), 0.1, s.ad_value(770), 1.0);
            s.store_sub_from_scalar(336, 0.1, 780);
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1776]) {
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && (!s.b[1776])) {
            s.copy_ad(336, 335);
            s.store_scalar(334, 1.0);
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) {
            s.store_sqrt_mul(1444, 1543, 336);
        }

        s.b[1782] = ((s.v[1444] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
        s.v[1782] = if s.b[1782] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) {
            s.store_offset_sub(781, 1444, 965, 1e-8);
            s.store_square(722, 781);
            s.store_scalar(723, (1e-8 * 1e-8));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1783] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1783] = if s.b[1783] { 1.0 } else { 0.0 };

        s.b[1784] = (2.0 == 1.0);
        s.v[1784] = if s.b[1784] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) && s.b[1783]) && s.b[1784]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1785] = (2.0 == 2.0);
        s.v[1785] = if s.b[1785] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) && s.b[1783]) && (!s.b[1784])) && s.b[1785]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1786] = (2.0 == 4.0);
        s.v[1786] = if s.b[1786] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) && s.b[1783]) && (!s.b[1784])) && (!s.b[1785])) && s.b[1786]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1787] = (2.0 == 8.0);
        s.v[1787] = if s.b[1787] { 1.0 } else { 0.0 };

        if (((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) && s.b[1783]) && (!s.b[1784])) && (!s.b[1785])) && (!s.b[1786])) && s.b[1787]) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) && s.b[1783]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign33530_loop_guard: usize = 0;
        while {
            let assign33530_cond_e38403: f64 = if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) && s.b[1783]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign33530_cond_e38403 != 0.0
        } {
            assign33530_loop_guard += 1;
            assert!(assign33530_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) && s.b[1783]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) && (!s.b[1783])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product(337, s.ad_value(725), s.ad_value(726), 1e-8, s.ad_value(770), 1.0);
            s.store_add_ad_lhs(1444, A::offset(s.ad_value(965), (-1e-8)), 780);
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && s.b[1782]) {
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && (!s.b[1782])) {
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) && (!s.b[1782])) {
            s.store_scalar(337, 1.0);
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1768])) {
            s.store_sqrt_mul_ad(1448, s.ad_value(1546), A::add_scaled_inputs3(s.ad_value(1461), 1.0, s.ad_value(1431), (-1.0), s.ad_value(1459), 1.0));
            s.store_mul(1495, 1444, 1542);
            s.store_mul_neg_lhs(1496, 1448, 1540);
        }

        s.b[1788] = (((s.v[1458] - s.v[1508]) < 0.06) && (0.06 >= 0.0));
        s.v[1788] = if s.b[1788] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) {
            s.store_sub_from_scalar_ad(781, 0.06, A::sub(s.ad_value(1458), s.ad_value(1508)));
            s.store_square(722, 781);
            s.store_scalar(723, (0.06 * 0.06));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1789] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1789] = if s.b[1789] { 1.0 } else { 0.0 };

        s.b[1790] = (2.0 == 1.0);
        s.v[1790] = if s.b[1790] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) && s.b[1789]) && s.b[1790]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1791] = (2.0 == 2.0);
        s.v[1791] = if s.b[1791] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) && s.b[1789]) && (!s.b[1790])) && s.b[1791]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1792] = (2.0 == 4.0);
        s.v[1792] = if s.b[1792] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) && s.b[1789]) && (!s.b[1790])) && (!s.b[1791])) && s.b[1792]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1793] = (2.0 == 8.0);
        s.v[1793] = if s.b[1793] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) && s.b[1789]) && (!s.b[1790])) && (!s.b[1791])) && (!s.b[1792])) && s.b[1793]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) && s.b[1789]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign33910_loop_guard: usize = 0;
        while {
            let assign33910_cond_e38958: f64 = if (((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) && s.b[1789]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign33910_cond_e38958 != 0.0
        } {
            assign33910_loop_guard += 1;
            assert!(assign33910_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) && s.b[1789]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) && (!s.b[1789])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.06);
            s.store_div_scaled_product(334, s.ad_value(725), s.ad_value(726), 0.06, s.ad_value(770), 1.0);
            s.store_sub_from_scalar(336, 0.06, 780);
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && s.b[1788]) {
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1707])) && (!s.b[1788])) {
            s.store_sub(336, 1458, 1508);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1707])) {
            s.store_offset_ad(338, A::add_scaled_product(A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0)), (10.0 * 2.220446049250313e-16));
            s.store_mul_scaled_ad_rhs(1512, 209, -1.0, A::sqrt(s.ad_value(338)));
        }

        if (s.b[1439] && s.b[1440]) {
            s.copy_ad(87, 1457);
            s.copy_ad(91, 1458);
            s.store_sub(94, 1458, 1457);
            s.store_neg_ad(335, A::add(s.ad_value(1471), s.ad_value(1472)));
        }

        s.b[1794] = ((s.v[335] < s.v[1536]) && (s.v[1536] >= 0.0));
        s.v[1794] = if s.b[1794] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1794]) {
            s.store_sub(781, 1536, 335);
            s.store_square(722, 781);
            s.store_square(723, 1536);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1795] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1795] = if s.b[1795] { 1.0 } else { 0.0 };

        s.b[1796] = (2.0 == 1.0);
        s.v[1796] = if s.b[1796] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && s.b[1440]) && s.b[1794]) && s.b[1795]) && s.b[1796]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1797] = (2.0 == 2.0);
        s.v[1797] = if s.b[1797] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && s.b[1794]) && s.b[1795]) && (!s.b[1796])) && s.b[1797]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1798] = (2.0 == 4.0);
        s.v[1798] = if s.b[1798] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && s.b[1794]) && s.b[1795]) && (!s.b[1796])) && (!s.b[1797])) && s.b[1798]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1799] = (2.0 == 8.0);
        s.v[1799] = if s.b[1799] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && s.b[1794]) && s.b[1795]) && (!s.b[1796])) && (!s.b[1797])) && (!s.b[1798])) && s.b[1799]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1794]) && s.b[1795]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign34320_loop_guard: usize = 0;
        while {
            let assign34320_cond_e39434: f64 = if ((((s.b[1439] && s.b[1440]) && s.b[1794]) && s.b[1795]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign34320_cond_e39434 != 0.0
        } {
            assign34320_loop_guard += 1;
            assert!(assign34320_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && s.b[1440]) && s.b[1794]) && s.b[1795]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1794]) && (!s.b[1795])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1794]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1536, 726);
            s.store_ad_value(334, A::div_scaled_product3(s.ad_value(1536), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0));
            s.store_sub(1552, 1536, 780);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1794]) {
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1794])) {
            s.copy_ad(1552, 335);
        }

    }

    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1439] && s.b[1440]) && (!s.b[1794])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_mul3_affine_lhs(1499, 154, 1552, 1.0 / (2.0), 0.0, 94);
            s.store_neg_ad(1500, A::sub(s.ad_value(1511), s.ad_value(1512)));
            s.store_add(248, 1499, 1500);
            s.store_neg(133, 1511);
            s.copy_ad(170, 162);
            s.store_scalar(336, (s.v[626] / 100.0));
            s.store_offset_mul_ad(338, A::sub(s.ad_value(1458), s.ad_value(1457)), s.ad_value(682), 1.0);
            s.store_mul(339, 336, 133);
            s.store_div(337, 339, 338);
            s.copy_ad(251, 337);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_ad_value(339, {
                if (s.v[251] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(251), (p.p160 - 1.0))
                }
            });
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_mul(342, 339, 251);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_ad_value(341, {
                if (s.v[251] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(251), A::offset(s.ad_value(624), (-1.0)))
                }
            });
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_mul(340, 341, 251);
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 133, 343);
            s.store_add_scaled_ad_lhs(335, A::add_scaled_product(A::div_from_scalar(1.0, A::scale_offset(s.ad_value(252), (s.v[475] * 1e-11), ((s.v[474]) + (1e-25)))), 1.0, s.ad_value(679), s.ad_value(342), 1.0), 340, 1.0 / (s.v[479]));
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_mul_ad_product_lhs(336, s.ad_value(154), A::offset(s.ad_value(133), 1e-25), 170);
            s.store_div_from_scalar(335, 1.0, 336);
            s.store_mul(333, 248, 335);
            s.store_div_scaled_inputs(336, s.ad_value(257), 0.2, s.ad_value(254), 1.0);
            s.store_sqrt_square_sum(255, 333, 336);
            s.store_div_from_scalar(338, 1.0, 255);
            s.store_mul(256, 254, 255);
            s.store_div(335, 256, 257);
            s.copy_ad(1554, 255);
        }

        s.b[1800] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1800] = if s.b[1800] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1800]) {
            s.store_scalar(337, 1.0);
        }

        s.b[1801] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1801] = if s.b[1801] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && (!s.b[1800])) && s.b[1801]) {
            s.copy_ad(337, 335);
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1800])) && (!s.b[1801])) {
            s.store_ad_value(337, {
                if (s.v[335] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(335), (p.p178 - 1.0))
                }
            });
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_mul(336, 335, 337);
            s.store_offset(338, 336, 1.0);
        }

        s.b[1802] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1802] = if s.b[1802] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1802]) {
            s.store_div_from_scalar(339, 1.0, 338);
        }

        s.b[1803] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1803] = if s.b[1803] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && (!s.b[1802])) && s.b[1803]) {
            s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1802])) && (!s.b[1803])) {
            s.store_ad_value(340, {
                if (s.v[338] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(338), (((-1.0) / p.p178) - 1.0))
                }
            });
        }

        if (((s.b[1439] && s.b[1440]) && (!s.b[1802])) && (!s.b[1803])) {
            s.store_mul(339, 338, 340);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_mul(253, 254, 339);
        }

        s.b[1804] = (s.v[349] > 1e-6);
        s.v[1804] = if s.b[1804] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1804]) {
            s.store_div_ad_rhs(336, 1498, A::square(s.ad_value(185)));
            s.store_add_scaled_inputs4(334, s.ad_value(85), 1.0, s.ad_value(974), 1.0, s.ad_value(155), -1.0, s.ad_value(1434), -1.0);
            s.store_offset_mul_ad(338, A::div_from_scalar(2.0, s.ad_value(336)), s.ad_value(334), 1.0);
        }

        s.b[1805] = ((s.v[338] < 2.0) && (2.0 >= 0.0));
        s.v[1805] = if s.b[1805] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) {
            s.store_sub_from_scalar(781, 2.0, 338);
            s.store_square(722, 781);
            s.store_scalar(723, (2.0 * 2.0));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1806] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1806] = if s.b[1806] { 1.0 } else { 0.0 };

        s.b[1807] = (2.0 == 1.0);
        s.v[1807] = if s.b[1807] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) && s.b[1806]) && s.b[1807]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1808] = (2.0 == 2.0);
        s.v[1808] = if s.b[1808] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) && s.b[1806]) && (!s.b[1807])) && s.b[1808]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1809] = (2.0 == 4.0);
        s.v[1809] = if s.b[1809] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) && s.b[1806]) && (!s.b[1807])) && (!s.b[1808])) && s.b[1809]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1810] = (2.0 == 8.0);
        s.v[1810] = if s.b[1810] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) && s.b[1806]) && (!s.b[1807])) && (!s.b[1808])) && (!s.b[1809])) && s.b[1810]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) && s.b[1806]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign35130_loop_guard: usize = 0;
        while {
            let assign35130_cond_e40353: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) && s.b[1806]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign35130_cond_e40353 != 0.0
        } {
            assign35130_loop_guard += 1;
            assert!(assign35130_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) && s.b[1806]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) && (!s.b[1806])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 2.0);
            s.store_div_scaled_product(334, s.ad_value(725), s.ad_value(726), 2.0, s.ad_value(770), 1.0);
            s.store_sub_from_scalar(343, 2.0, 780);
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1805]) {
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1804]) && (!s.b[1805])) {
            s.copy_ad(343, 338);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1804]) {
            s.store_offset(343, 343, 1e-25);
            s.store_sqrt(337, 343);
            s.store_mul_sub_from_scalar_rhs(338, 336, 1.0, 337);
            s.store_add_scaled_inputs3(344, s.ad_value(85), 1.0, s.ad_value(974), 1.0, s.ad_value(338), 1.0);
            s.store_mul(344, 344, 975);
        }

        s.b[1811] = ((s.v[344] < (s.v[972] + 4.0)) && (4.0 >= 0.0));
        s.v[1811] = if s.b[1811] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) {
            s.store_sub_ad_lhs(781, A::offset(s.ad_value(972), 4.0), 344);
            s.store_square(722, 781);
            s.store_scalar(723, (4.0 * 4.0));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1812] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1812] = if s.b[1812] { 1.0 } else { 0.0 };

        s.b[1813] = (4.0 == 1.0);
        s.v[1813] = if s.b[1813] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) && s.b[1812]) && s.b[1813]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1814] = (4.0 == 2.0);
        s.v[1814] = if s.b[1814] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) && s.b[1812]) && (!s.b[1813])) && s.b[1814]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1815] = (4.0 == 4.0);
        s.v[1815] = if s.b[1815] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) && s.b[1812]) && (!s.b[1813])) && (!s.b[1814])) && s.b[1815]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1816] = (4.0 == 8.0);
        s.v[1816] = if s.b[1816] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) && s.b[1812]) && (!s.b[1813])) && (!s.b[1814])) && (!s.b[1815])) && s.b[1816]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) && s.b[1812]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign35570_loop_guard: usize = 0;
        while {
            let assign35570_cond_e40896: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) && s.b[1812]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign35570_cond_e40896 != 0.0
        } {
            assign35570_loop_guard += 1;
            assert!(assign35570_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) && s.b[1812]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

    }

    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) && (!s.b[1812])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                }
            });
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 4.0);
            s.store_div_scaled_product(334, s.ad_value(725), s.ad_value(726), 4.0, s.ad_value(770), 1.0);
            s.store_sub_ad_lhs(344, A::offset(s.ad_value(972), 4.0), 780);
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1804]) && s.b[1811]) {
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1804]) && (!s.b[1811])) {
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1804]) && (!s.b[1811])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1804]) {
            s.store_div(335, 349, 344);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1804]) {
            s.store_ad_value(336, {
                if (s.v[335] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(335), A::offset(s.ad_value(658), (-1.0)))
                }
            });
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1804]) {
            s.store_mul(341, 336, 335);
            s.store_offset(337, 341, 1.0);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1804]) {
            s.store_ad_value(338, {
                if (s.v[337] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)))
                }
            });
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1804]) {
            s.store_mul(340, 338, 337);
            s.store_div(1553, 349, 340);
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1804])) {
            s.copy_ad(1553, 349);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_offset_mul_ad(338, A::sub(s.ad_value(1458), s.ad_value(1457)), s.ad_value(682), 1.0);
            s.store_neg(133, 1492);
            s.copy_ad(339, 133);
            s.store_div(337, 339, 338);
            s.copy_ad(251, 337);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_ad_value(339, {
                if (s.v[251] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(251), (p.p376 - 1.0))
                }
            });
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_mul(342, 339, 251);
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 133, 343);
            s.store_add_scaled_product(335, A::div_scalar_offset_denominator(1.0, A::add_scaled_product(s.ad_value(966), 1.0, s.ad_value(968), s.ad_value(252), 1e-11), 1e-25, 1.0), 1.0, s.ad_value(977), s.ad_value(342), 1.0);
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_div(1551, 1553, 170);
            s.store_div_scaled_product(335, s.ad_value(254), s.ad_value(1551), 1.0, s.ad_value(973), 1.0);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_ad_value(336, {
                if (s.v[335] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(335), p.p378)
                }
            });
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_offset(337, 336, 1.0);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_ad_value(338, {
                if (s.v[337] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(337), (1.0 / p.p378))
                }
            });
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_div(1502, 254, 338);
            s.store_mul3_affine_lhs(987, 1492, 1502, (-s.v[632]), 0.0, 1551);
            s.store_offset_mul_ad(338, A::sub(s.ad_value(1458), s.ad_value(1457)), s.ad_value(682), 1.0);
            s.store_neg(133, 1501);
            s.copy_ad(339, 133);
            s.store_div(337, 339, 338);
            s.copy_ad(251, 337);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_ad_value(339, {
                if (s.v[251] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(251), (p.p376 - 1.0))
                }
            });
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_mul(342, 339, 251);
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 133, 343);
            s.store_add_scaled_product(335, A::div_scalar_offset_denominator(1.0, A::add_scaled_product(s.ad_value(967), 1.0, s.ad_value(969), s.ad_value(252), 1e-11), 1e-25, 1.0), 1.0, s.ad_value(977), s.ad_value(342), 1.0);
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_div(1551, 1553, 170);
            s.store_div_scaled_product(335, s.ad_value(254), s.ad_value(1551), 1.0, s.ad_value(973), 1.0);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_ad_value(336, {
                if (s.v[335] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(335), p.p378)
                }
            });
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_offset(337, 336, 1.0);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_ad_value(338, {
                if (s.v[337] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(337), (1.0 / p.p378))
                }
            });
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_div(1503, 254, 338);
            s.store_mul3_affine_lhs(1550, 1501, 1503, (-s.v[632]), 0.0, 1551);
            s.store_div_scaled_inputs(115, s.ad_value(155), s.v[632], s.ad_value(170), 1.0);
            s.store_add_scaled_inputs3(135, A::mul3(s.ad_value(115), s.ad_value(248), s.ad_value(253)), 1.0, s.ad_value(987), 1.0, s.ad_value(1550), 1.0);
            s.store_mul3_lhs(986, 115, 248, 253);
            s.copy_ad(984, 253);
            s.copy_ad(790, 349);
        }

        s.b[1817] = (p.p283 != 0.0);
        s.v[1817] = if s.b[1817] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1817]) {
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 100.0));
            s.store_offset_ad(782, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0)), 1.0);
            s.store_offset_ad(783, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0)), (1.0 / 2.0));
            s.store_div_from_scalar(340, 0.01, 782);
            s.store_div_scaled_inputs(336, s.ad_value(783), (-2.0), A::square(s.ad_value(782)), 1.0);
            s.store_sub_from_scalar_ad(335, 1.1, A::add(s.ad_value(1457), s.ad_value(340)));
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.05) * 0.05));
            s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);
            s.store_scaled_add(336, 335, 782, 0.5);
        }

        s.b[1818] = (s.v[336] < 0.0);
        s.v[1818] = if s.b[1818] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && s.b[1817]) && s.b[1818]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1817]) {
            s.store_offset(336, 336, 1e-25);
            s.store_scale(334, 154, s.v[672]);
            s.store_mul(337, 185, 334);
            s.store_powf(334, 336, p.p284);
            s.store_mul(343, 337, 334);
            s.store_offset_scaled(338, 1435, p.p285, 1.0);
            s.store_scalar(334, s.v[673]);
            s.store_add_scaled_inputs3(339, s.ad_value(1457), 1.0, s.ad_value(340), 1.0, s.ad_value(1434), -1.0);
            s.store_add_ad_rhs(338, 338, A::mul3(s.ad_value(1435), s.ad_value(334), s.ad_value(339)));
            s.store_mul(340, 343, 338);
            s.copy_ad(343, 340);
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1817])) {
            s.store_scalar(343, 0.0);
        }

        s.b[1819] = (p.p287 != 0.0);
        s.v[1819] = if s.b[1819] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1819]) {
            s.store_scale(335, 154, s.v[674]);
            s.store_mul(336, 185, 335);
            s.store_mul(342, 336, 1435);
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1819])) {
            s.store_scalar(342, 0.0);
        }

        s.b[1820] = ((s.v[343] + s.v[342]) > 0.0);
        s.v[1820] = if s.b[1820] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1820]) {
            s.store_mul_add_rhs(249, 94, 343, 342);
            s.store_add_ad_rhs(135, 135, A::mul3(s.ad_value(115), s.ad_value(249), s.ad_value(253)));
        }

        s.b[1821] = ((s.v[74] == 2.0) || (s.v[74] == 3.0));
        s.v[1821] = if s.b[1821] { 1.0 } else { 0.0 };

        s.b[1822] = (p.p296 > 0.0);
        s.v[1822] = if s.b[1822] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && s.b[1822]) {
            s.copy_ad(338, 647);
            s.store_scaled_offset(335, 796, (-p.p300), s.v[533]);
            s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));
            s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && s.b[1822]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && s.b[1822]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3(336, s.ad_value(338), 1.0, s.ad_value(781), 0.5, s.ad_value(782), 0.5);
            s.store_scale(337, 338, (p.p296 + 1.0));
            s.store_offset_sub(781, 337, 336, (-(0.01 * 0.01)));
            s.store_scale(782, 337, (4.0 * (0.01 * 0.01)));
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && s.b[1822]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && s.b[1822]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3(341, s.ad_value(337), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5));
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && (!s.b[1822])) {
            s.copy_ad(341, 647);
        }

        s.b[1823] = (s.v[793] >= 0.0);
        s.v[1823] = if s.b[1823] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && s.b[1823]) {
            s.copy_ad(369, 793);
        }

    }

    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && (!s.b[1823])) {
            s.store_scalar(369, 0.0);
        }

        s.b[1824] = (s.v[369] < (20.0 * 1e-12));
        s.v[1824] = if s.b[1824] { 1.0 } else { 0.0 };

        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && s.b[1824]) {
            s.store_scalar(378, (((((20.0 + 1.0)) as f64).powf((p.p297 - 1.0)) * ((20.0 + 1.0) - ((0.5 * p.p297) * 20.0))) * ((1e-12) as f64).powf(p.p297)));
            s.store_scalar(379, ((((0.5 * p.p297) * (((20.0 + 1.0)) as f64).powf((p.p297 - 1.0))) / 20.0) * ((1e-12) as f64).powf((p.p297 - 2.0))));
            s.store_add_ad_rhs(335, 378, A::mul3(s.ad_value(379), s.ad_value(369), s.ad_value(369)));
        }

        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && (!s.b[1824])) {
            s.store_powf_ad(335, A::offset(s.ad_value(369), 1e-12), p.p297);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1821]) {
            s.store_powf_ad(343, A::offset(s.ad_value(369), 1e-12), p.p299);
            s.store_add_scaled_products(368, s.ad_value(341), s.ad_value(335), 1.0 / (s.v[632]), s.ad_value(797), s.ad_value(343), (s.v[531] * 1.0 / (s.v[632])));
            s.store_mul(334, 368, 135);
            s.store_offset(335, 790, 1e-12);
            s.store_div_from_scalar(336, 1.0, 335);
            s.store_offset_mul(337, 334, 336, 1.0);
            s.store_div_from_scalar(338, 1.0, 337);
            s.store_mul(134, 135, 338);
        }

        if ((s.b[1439] && s.b[1440]) && (!s.b[1821])) {
            s.copy_ad(134, 135);
            s.store_scalar(368, 0.0);
        }

        if (s.b[1439] && s.b[1440]) {
            s.store_add_scaled_inputs4(131, s.ad_value(1473), (-0.5), s.ad_value(1474), (-0.5), s.ad_value(1494), (-0.5), s.ad_value(1496), (-0.5));
            s.store_scaled_add_ad_lhs(133, A::add(A::add_scaled_inputs4(s.ad_value(1534), 1.0, s.ad_value(1535), 1.0, s.ad_value(1513), 1.0, s.ad_value(1514), 1.0), s.ad_value(1493)), 1495, (-0.5));
            s.store_scalar(247, 0.5);
            s.store_scaled_add(978, 1534, 1535, (-0.5));
            s.store_neg(238, 1534);
            s.copy_ad(255, 1554);
        }

        s.b[1825] = ((s.v[238] < 1e-25) || (s.v[133] < 1e-25));
        s.v[1825] = if s.b[1825] { 1.0 } else { 0.0 };

        if ((s.b[1439] && s.b[1440]) && s.b[1825]) {
            s.store_scalar(78, 1.0);
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.copy_ad(1851, 960);
            s.store_scale(1901, 964, 1.6021918e-19);
            s.store_scale(1880, 964, (1.6021918e-19 * 1.034943e-10));
            s.store_scale(1900, 622, 1.6021918e-19);
            s.store_square(1899, 965);
            s.store_div_from_scalar(1904, (2.0 * 1.034943e-10), 1901);
            s.store_div_from_scalar(1905, (2.0 * 1.034943e-10), 1900);
            s.store_div(1898, 964, 622);
            s.store_div_from_scalar_offset_input(1897, 1.0, 1898, 1.0);
            s.store_div_ad_rhs(1902, 1880, A::square(s.ad_value(185)));
            s.store_div_from_scalar(1903, 2.0, 1902);
            s.store_scalar(1906, 4.0);
            s.store_scalar(1907, 0.1);
            s.store_scalar(1908, 0.1);
            s.store_offset(1909, 961, p.p407);
            s.store_scalar(1910, 3.0);
            s.store_scalar(1849, 0.0);
            s.store_scalar(1850, 0.0);
            s.store_scalar(1858, 0.0);
            s.store_scalar(1859, 0.0);
            s.store_scalar(1891, 0.0);
            s.store_scalar(1892, 0.0);
            s.store_scalar(1862, 0.0);
            s.store_scalar(1864, 0.0);
            s.store_scalar(1863, 0.0);
            s.store_scalar(1865, 0.0);
            s.store_scalar(1835, 0.0);
            s.store_scalar(1830, 0.0);
            s.copy_ad(1883, 1431);
            s.store_offset(85, 85, ((10.0 * 2.220446049250313e-16) * 100000000.0));
            s.store_div_scaled_product(962, s.ad_value(1904), s.ad_value(622), 1.0, A::add(s.ad_value(964), s.ad_value(622)), 1.0);
            s.store_sub(335, 1851, 1434);
        }

        s.b[1913] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
        s.v[1913] = if s.b[1913] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) {
            s.store_sub_from_scalar(781, 0.1, 335);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1914] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1914] = if s.b[1914] { 1.0 } else { 0.0 };

        s.b[1915] = (4.0 == 1.0);
        s.v[1915] = if s.b[1915] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) && s.b[1915]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1916] = (4.0 == 2.0);
        s.v[1916] = if s.b[1916] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) && (!s.b[1915])) && s.b[1916]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1917] = (4.0 == 4.0);
        s.v[1917] = if s.b[1917] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) && (!s.b[1915])) && (!s.b[1916])) && s.b[1917]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1918] = (4.0 == 8.0);
        s.v[1918] = if s.b[1918] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) && (!s.b[1915])) && (!s.b[1916])) && (!s.b[1917])) && s.b[1918]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign37570_loop_guard: usize = 0;
        while {
            let assign37570_cond_e43230: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign37570_cond_e43230 != 0.0
        } {
            assign37570_loop_guard += 1;
            assert!(assign37570_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && (!s.b[1914])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                }
            });
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product(334, s.ad_value(725), s.ad_value(726), 0.1, s.ad_value(770), 1.0);
            s.store_sub_from_scalar(336, 0.1, 780);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) {
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1913])) {
            s.copy_ad(336, 335);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_mul(1834, 962, 336);
            s.store_sqrt(1832, 1834);
        }

        s.b[1919] = (p.p345 != 0.0);
        s.v[1919] = if s.b[1919] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1919]) {
            s.store_mul_sub_from_scalar_ad_rhs(335, 965, 1.0, A::scale(s.ad_value(790), p.p345));
            s.store_scale(336, 965, 0.001);
            s.store_add_scaled_inputs3(781, s.ad_value(335), 1.0, s.ad_value(965), (-0.1), s.ad_value(336), -1.0);
            s.store_scaled_mul(782, 965, 336, (0.1 * 4.0));
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1919]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1919]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3(335, s.ad_value(965), 0.1, s.ad_value(781), 0.5, s.ad_value(782), 0.5);
            s.store_add_scaled_inputs3(781, s.ad_value(965), 2.0, s.ad_value(335), (-1.0), s.ad_value(336), -1.0);
            s.store_scaled_mul(782, 965, 336, (2.0 * 4.0));
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1919]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1919]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3(965, s.ad_value(965), 2.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5));
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_scalar(1847, 0.0);
        }

        s.b[1920] = (s.v[1832] > s.v[965]);
        s.v[1920] = if s.b[1920] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1920]) {
            s.copy_ad(1831, 965);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1920])) {
            s.copy_ad(1831, 1832);
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_offset_sub_from_scalar_ad(781, 0.3, s.ad_value(85), (-0.01));
            s.store_scalar(782, ((4.0 * 0.3) * 0.01));
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_sub_from_scalar_ad(1856, 0.3, A::add_scaled_inputs(s.ad_value(781), 0.5, s.ad_value(782), 0.5));
        }

    }

    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
    ) {
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_add_scaled_inputs3_offset(781, s.ad_value(1856), 1.0, s.ad_value(1883), -1.0, s.ad_value(1851), 1.0, (-0.01));
            s.store_scaled_sub(782, 1883, 1851, (4.0 * 0.01));
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4(1856, s.ad_value(1883), 1.0, s.ad_value(1851), (-1.0), s.ad_value(781), 0.5, s.ad_value(782), 0.5);
            s.store_div_scaled_product(1884, s.ad_value(1851), s.ad_value(622), -1.0, A::add(s.ad_value(622), s.ad_value(964)), 1.0);
            s.store_offset_sub(1830, 965, 1831, 1e-15);
            s.store_scalar(79, 0.0);
            s.store_scalar(1846, 0.2);
            s.copy_ad(1849, 1856);
            s.copy_ad(1852, 1847);
            s.copy_ad(1854, 1884);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign38070_loop_guard: usize = 0;
        while {
            let assign38070_cond_e43943: f64 = if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign38070_cond_e43943 != 0.0
        } {
            assign38070_loop_guard += 1;
            assert!(assign38070_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
                s.store_mul_sub_ad_rhs(1854, 1897, A::add_scaled_product(s.ad_value(1883), 1.0, s.ad_value(1898), s.ad_value(1852), 1.0), s.ad_value(1851));
                s.store_mul(1838, 1897, 1898);
                s.store_sub(335, 1852, 1854);
            }
            s.b[1921] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1921] = if s.b[1921] { 1.0 } else { 0.0 };
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) {
                s.store_sub_from_scalar(781, 0.1, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.1 * 0.1));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1922] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1922] = if s.b[1922] { 1.0 } else { 0.0 };
            s.b[1923] = (2.0 == 1.0);
            s.v[1923] = if s.b[1923] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) && s.b[1923]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1924] = (2.0 == 2.0);
            s.v[1924] = if s.b[1924] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) && (!s.b[1923])) && s.b[1924]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1925] = (2.0 == 4.0);
            s.v[1925] = if s.b[1925] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) && (!s.b[1923])) && (!s.b[1924])) && s.b[1925]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1926] = (2.0 == 8.0);
            s.v[1926] = if s.b[1926] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) && (!s.b[1923])) && (!s.b[1924])) && (!s.b[1925])) && s.b[1926]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign38070_body29_loop_guard: usize = 0;
            while {
                let assign38070_body29_cond_e44306: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38070_body29_cond_e44306 != 0.0
            } {
                assign38070_body29_loop_guard += 1;
                assert!(assign38070_body29_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && (!s.b[1922])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product(334, s.ad_value(725), s.ad_value(726), 0.1, s.ad_value(770), 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) {
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1921])) {
                s.copy_ad(336, 335);
                s.store_scalar(334, 1.0);
            }
            if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
                s.store_sqrt_mul(1826, 1904, 336);
            }
            s.b[1927] = ((s.v[1826] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.v[1927] = if s.b[1927] { 1.0 } else { 0.0 };
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) {
                s.store_offset_sub(781, 1826, 965, 1e-8);
                s.store_square(722, 781);
                s.store_scalar(723, (1e-8 * 1e-8));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1928] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1928] = if s.b[1928] { 1.0 } else { 0.0 };
            s.b[1929] = (2.0 == 1.0);
            s.v[1929] = if s.b[1929] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) && s.b[1929]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1930] = (2.0 == 2.0);
            s.v[1930] = if s.b[1930] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) && (!s.b[1929])) && s.b[1930]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1931] = (2.0 == 4.0);
            s.v[1931] = if s.b[1931] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) && (!s.b[1929])) && (!s.b[1930])) && s.b[1931]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1932] = (2.0 == 8.0);
            s.v[1932] = if s.b[1932] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) && (!s.b[1929])) && (!s.b[1930])) && (!s.b[1931])) && s.b[1932]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign38070_body65_loop_guard: usize = 0;
            while {
                let assign38070_body65_cond_e44791: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38070_body65_cond_e44791 != 0.0
            } {
                assign38070_body65_loop_guard += 1;
                assert!(assign38070_body65_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && (!s.b[1928])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product(337, s.ad_value(725), s.ad_value(726), 1e-8, s.ad_value(770), 1.0);
                s.store_add_ad_lhs(1826, A::offset(s.ad_value(965), (-1e-8)), 780);
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) {
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1927])) {
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1927])) {
                s.store_scalar(337, 1.0);
            }
            if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
                s.store_mul(1862, 1826, 1901);
                s.store_mul_ad_product_lhs(1840, A::div_from_scalar(1.034943e-10, s.ad_value(1826)), s.ad_value(334), 337);
                s.store_mul_ad_product_lhs(1842, A::div_from_scalar((-1.034943e-10), s.ad_value(1826)), s.ad_value(334), 337);
            }
            s.b[1933] = (p.p49 == 0.0);
            s.v[1933] = if s.b[1933] { 1.0 } else { 0.0 };
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1933]) {
                s.store_add_ad_lhs(1835, A::div_scaled_inputs_product(s.ad_value(1899), 1.0, s.ad_value(1834), 1.0, s.ad_value(965), s.ad_value(1831), (-2.0), s.ad_value(1904), 1.0), 1849);
                s.store_scalar(1836, 1.0);
                s.store_scalar(1837, 0.0);
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1933])) {
                s.store_add_ad_rhs(1835, 1849, A::div_scaled_add_product(s.ad_value(1899), 1.0, s.ad_value(1826), A::sub_scaled_inputs(s.ad_value(1826), 1.0, s.ad_value(965), 2.0), 1.0, s.ad_value(1904), 1.0));
                s.store_scalar(1836, 1.0);
                s.store_ad_value(1837, A::mul_sub_from_scalar_rhs(A::add_scaled_product(s.ad_value(334), 1.0, A::div(s.ad_value(965), s.ad_value(1826)), s.ad_value(334), (-1.0)), 1.0, s.ad_value(1838)));
            }
            s.b[1934] = ((s.v[1835] > (s.v[1847] - s.v[1846])) && (s.v[1846] >= 0.0));
            s.v[1934] = if s.b[1934] { 1.0 } else { 0.0 };
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) {
                s.store_add_scaled_inputs3(781, s.ad_value(1835), 1.0, s.ad_value(1847), (-1.0), s.ad_value(1846), 1.0);
                s.store_square(722, 781);
                s.store_square(723, 1846);
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1935] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.v[1935] = if s.b[1935] { 1.0 } else { 0.0 };
            s.b[1936] = (4.0 == 1.0);
            s.v[1936] = if s.b[1936] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) && s.b[1936]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1937] = (4.0 == 2.0);
            s.v[1937] = if s.b[1937] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) && (!s.b[1936])) && s.b[1937]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1938] = (4.0 == 4.0);
            s.v[1938] = if s.b[1938] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) && (!s.b[1936])) && (!s.b[1937])) && s.b[1938]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1939] = (4.0 == 8.0);
            s.v[1939] = if s.b[1939] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) && (!s.b[1936])) && (!s.b[1937])) && (!s.b[1938])) && s.b[1939]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign38070_body114_loop_guard: usize = 0;
            while {
                let assign38070_body114_cond_e45464: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38070_body114_cond_e45464 != 0.0
            } {
                assign38070_body114_loop_guard += 1;
                assert!(assign38070_body114_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && (!s.b[1935])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                    }
                });
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_mul3_lhs(780, 781, 1846, 726);
                s.store_ad_value(334, A::div_scaled_product3(s.ad_value(1846), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0));
                s.store_add_scaled_inputs3(1835, s.ad_value(1847), 1.0, s.ad_value(1846), (-1.0), s.ad_value(780), 1.0);
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) {
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1934])) {
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1934])) {
                s.store_scalar(334, 1.0);
            }
            if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
                s.store_mul(1836, 1836, 334);
                s.store_mul(1837, 1837, 334);
                s.store_add_scaled_inputs3(335, s.ad_value(1854), 1.0, s.ad_value(1883), (-1.0), s.ad_value(1851), 1.0);
            }
            s.b[1940] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1940] = if s.b[1940] { 1.0 } else { 0.0 };
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) {
                s.store_sub_from_scalar(781, 0.1, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.1 * 0.1));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1941] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1941] = if s.b[1941] { 1.0 } else { 0.0 };
            s.b[1942] = (2.0 == 1.0);
            s.v[1942] = if s.b[1942] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) && s.b[1942]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1943] = (2.0 == 2.0);
            s.v[1943] = if s.b[1943] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) && (!s.b[1942])) && s.b[1943]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1944] = (2.0 == 4.0);
            s.v[1944] = if s.b[1944] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) && (!s.b[1942])) && (!s.b[1943])) && s.b[1944]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1945] = (2.0 == 8.0);
            s.v[1945] = if s.b[1945] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) && (!s.b[1942])) && (!s.b[1943])) && (!s.b[1944])) && s.b[1945]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign38070_body152_loop_guard: usize = 0;
            while {
                let assign38070_body152_cond_e45972: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38070_body152_cond_e45972 != 0.0
            } {
                assign38070_body152_loop_guard += 1;
                assert!(assign38070_body152_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && (!s.b[1941])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product(337, s.ad_value(725), s.ad_value(726), 0.1, s.ad_value(770), 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) {
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1940])) {
                s.copy_ad(336, 335);
                s.store_scalar(337, 1.0);
            }
            if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
                s.store_sqrt_mul(1828, 1905, 336);
                s.store_mul_neg_lhs(1863, 1828, 1900);
                s.store_mul_div_from_scalar_lhs(1844, (-1.034943e-10), 1828, 337);
                s.store_mul_sub_rhs(335, 154, 1849, 1852);
                s.store_exp(336, 335);
            }
            s.b[1946] = (s.v[1849] >= s.v[1852]);
            s.v[1946] = if s.b[1946] { 1.0 } else { 0.0 };
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1946]) {
                s.store_mul_scaled_ad_rhs(1858, 209, -1.0, A::sqrt(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15)));
                s.store_mul_ad(1893, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(1858), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
                s.store_neg(1895, 1893);
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1946])) {
                s.store_exp_ad(337, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1849), s.ad_value(1883))));
                s.store_exp_ad(338, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1852), s.ad_value(1883))));
                s.store_mul_sqrt_ad_rhs(1858, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product(339, s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(1858), 1.0);
                s.store_mul_add_ad_rhs(1893, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
                s.store_mul_add_ad_rhs(1895, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] != 0.0)) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) {
                s.store_add_scaled_inputs3(1866, A::add_scaled_product(s.ad_value(1858), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1849)), 1.0), 1.0, s.ad_value(1862), 1.0, s.ad_value(1863), 1.0);
                s.store_sub(1867, 1893, 185);
                s.store_add_ad_rhs(1868, 1895, A::add_scaled_value_products(s.ad_value(1840), 1.0, s.ad_value(1842), s.ad_value(1838), 1.0, s.ad_value(1844), s.ad_value(1838), 1.0));
                s.store_sub(1869, 1852, 1835);
                s.store_neg(1870, 1836);
                s.store_sub_from_scalar(1871, 1.0, 1837);
                s.store_add_scaled_products(1872, s.ad_value(1867), s.ad_value(1871), 1.0, s.ad_value(1868), s.ad_value(1870), (-1.0));
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) {
                s.store_ad_value(1873, {
                    if (s.v[1872] > 0.0) {
                        A::div_scalar_offset_denominator(1.0, s.ad_value(1872), 1e-25, 1.0)
                    } else {
                        A::div_scalar_offset_denominator(1.0, s.ad_value(1872), (-1e-25), 1.0)
                    }
                });
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) {
                s.copy_ad(1874, 1871);
                s.store_neg(1875, 1868);
                s.store_neg(1876, 1870);
                s.copy_ad(1877, 1867);
                s.store_mul_scaled_ad_rhs(1878, 1873, -1.0, A::add_scaled_products(s.ad_value(1874), s.ad_value(1866), 1.0, s.ad_value(1875), s.ad_value(1869), 1.0));
                s.store_mul_scaled_ad_rhs(1879, 1873, -1.0, A::add_scaled_products(s.ad_value(1876), s.ad_value(1866), 1.0, s.ad_value(1877), s.ad_value(1869), 1.0));
                s.store_abs(335, 1878);
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) {
                s.store_ad_value(335, {
                    if (s.v[335] < ((s.v[1879]) as f64).abs()) {
                        A::abs(s.ad_value(1879))
                    } else {
                        s.ad_value(335)
                    }
                });
            }
            s.b[1947] = (s.v[335] > 0.1);
            s.v[1947] = if s.b[1947] { 1.0 } else { 0.0 };
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) && s.b[1947]) {
                s.store_mul_div_from_scalar_rhs(1878, 1878, 0.1, 335);
                s.store_mul_div_from_scalar_rhs(1879, 1879, 0.1, 335);
            }
            s.b[1948] = (s.v[335] < 1e-12);
            s.v[1948] = if s.b[1948] { 1.0 } else { 0.0 };
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) && s.b[1948]) {
                s.store_scalar(79, 1.0);
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) {
                s.store_add(1849, 1849, 1878);
                s.store_add(1852, 1852, 1879);
            }
            if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
                s.store_offset(97, 97, 1.0);
            }
        }

    }
}
