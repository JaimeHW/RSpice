#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
    ) {
        let mut assign25270_loop_guard: usize = 0;
        while {
            let assign25270_cond_e21724: f64 = (150.0 + 1.0);
            let assign25270_cond_e21726: f64 = if ((s.b[1441] && s.b[1442]) && (s.v[97] <= assign25270_cond_e21724)) { 1.0 } else { 0.0 };
            assign25270_cond_e21726 != 0.0
        } {
            assign25270_loop_guard += 1;
            assert!(assign25270_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1441] && s.b[1442]) {
                s.store_mul_sub_ad_rhs(1462, 1533, A::add_scaled_product(s.ad_value(1433), 1.0, s.ad_value(1534), s.ad_value(1481), 1.0), s.ad_value(1461));
                s.store_mul(1531, 1533, 1534);
                s.store_sub(335, 1481, 1462);
            }
            s.b[1592] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1592] = if s.b[1592] { 1.0 } else { 0.0 };
            if ((s.b[1441] && s.b[1442]) && s.b[1592]) {
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
            s.b[1593] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1593] = if s.b[1593] { 1.0 } else { 0.0 };
            s.b[1594] = (2.0 == 1.0);
            s.v[1594] = if s.b[1594] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && s.b[1442]) && s.b[1592]) && s.b[1593]) && s.b[1594]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1595] = (2.0 == 2.0);
            s.v[1595] = if s.b[1595] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && s.b[1442]) && s.b[1592]) && s.b[1593]) && (!s.b[1594])) && s.b[1595]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1596] = (2.0 == 4.0);
            s.v[1596] = if s.b[1596] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && s.b[1442]) && s.b[1592]) && s.b[1593]) && (!s.b[1594])) && (!s.b[1595])) && s.b[1596]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1597] = (2.0 == 8.0);
            s.v[1597] = if s.b[1597] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && s.b[1442]) && s.b[1592]) && s.b[1593]) && (!s.b[1594])) && (!s.b[1595])) && (!s.b[1596])) && s.b[1597]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1592]) && s.b[1593]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign25270_body29_loop_guard: usize = 0;
            while {
                let assign25270_body29_cond_e22017: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1592]) && s.b[1593]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign25270_body29_cond_e22017 != 0.0
            } {
                assign25270_body29_loop_guard += 1;
                assert!(assign25270_body29_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && s.b[1442]) && s.b[1592]) && s.b[1593]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1592]) && (!s.b[1593])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((s.b[1441] && s.b[1442]) && s.b[1592]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_ad_lhs(334, A::mul_scaled_lhs(s.ad_value(725), 0.1, s.ad_value(726)), 770);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if ((s.b[1441] && s.b[1442]) && s.b[1592]) {
            }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1592])) {
                s.copy_ad(336, 335);
                s.store_scalar(334, 1.0);
            }
            if (s.b[1441] && s.b[1442]) {
                s.store_sqrt_mul(1445, 1545, 336);
            }
            s.b[1598] = ((s.v[1445] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.v[1598] = if s.b[1598] { 1.0 } else { 0.0 };
            if ((s.b[1441] && s.b[1442]) && s.b[1598]) {
                s.store_offset_sub(781, 1445, 965, 1e-8);
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
            s.b[1599] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1599] = if s.b[1599] { 1.0 } else { 0.0 };
            s.b[1600] = (2.0 == 1.0);
            s.v[1600] = if s.b[1600] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && s.b[1442]) && s.b[1598]) && s.b[1599]) && s.b[1600]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1601] = (2.0 == 2.0);
            s.v[1601] = if s.b[1601] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && s.b[1442]) && s.b[1598]) && s.b[1599]) && (!s.b[1600])) && s.b[1601]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1602] = (2.0 == 4.0);
            s.v[1602] = if s.b[1602] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && s.b[1442]) && s.b[1598]) && s.b[1599]) && (!s.b[1600])) && (!s.b[1601])) && s.b[1602]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1603] = (2.0 == 8.0);
            s.v[1603] = if s.b[1603] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && s.b[1442]) && s.b[1598]) && s.b[1599]) && (!s.b[1600])) && (!s.b[1601])) && (!s.b[1602])) && s.b[1603]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1598]) && s.b[1599]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign25270_body65_loop_guard: usize = 0;
            while {
                let assign25270_body65_cond_e22406: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1598]) && s.b[1599]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign25270_body65_cond_e22406 != 0.0
            } {
                assign25270_body65_loop_guard += 1;
                assert!(assign25270_body65_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && s.b[1442]) && s.b[1598]) && s.b[1599]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1598]) && (!s.b[1599])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((s.b[1441] && s.b[1442]) && s.b[1598]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_ad_lhs(337, A::mul_scaled_lhs(s.ad_value(725), 1e-8, s.ad_value(726)), 770);
                s.store_add_ad_lhs(1445, A::offset(s.ad_value(965), (-1e-8)), 780);
            }
            if ((s.b[1441] && s.b[1442]) && s.b[1598]) {
            }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1598])) {
            }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1598])) {
                s.store_scalar(337, 1.0);
            }
            if (s.b[1441] && s.b[1442]) {
                s.store_sqrt_mul_ad(1449, s.ad_value(1548), A::add(A::sub(s.ad_value(1462), s.ad_value(1433)), s.ad_value(1461)));
                s.store_mul(1495, 1445, 1544);
                s.store_mul_ad_product_lhs(1525, A::div_from_scalar(1.034943e-10, s.ad_value(1445)), s.ad_value(334), 337);
                s.store_mul_ad_product_lhs(1527, A::div_from_scalar((-1.034943e-10), s.ad_value(1445)), s.ad_value(334), 337);
                s.store_mul_neg_lhs(1496, 1449, 1542);
                s.store_div_from_scalar(1529, (-1.034943e-10), 1449);
                s.store_scaled_mul(335, 1500, 1541, 8.0);
            }
            if (s.b[1441] && s.b[1442]) {
                let assign25270_body81_ad_e22641: A = A::add(A::add(A::add(A::sub(A::mul3_scaled_output(s.ad_value(1462), s.ad_value(1462), s.ad_value(1540), 4.0), A::mul3_scaled_output(s.ad_value(1462), s.ad_value(1540), s.ad_value(1459), 8.0)), A::mul3_scaled_output(s.ad_value(1540), s.ad_value(1459), s.ad_value(1459), 4.0)), A::mul3_scaled_output(s.ad_value(1462), s.ad_value(1500), s.ad_value(1541), 4.0)), A::mul3_scaled_output(s.ad_value(1459), s.ad_value(1500), s.ad_value(1541), 4.0));
                s.store_div_ad_lhs(1518, A::add_scaled_product(assign25270_body81_ad_e22641, 1.0, A::mul3(s.ad_value(1543), s.ad_value(1539), s.ad_value(1541)), s.ad_value(1541), 1.0), 335);
            }
            if (s.b[1441] && s.b[1442]) {
                s.store_div_ad_lhs(1519, A::add_scaled_product(A::add_scaled_products(s.ad_value(1462), s.ad_value(1540), (-8.0), s.ad_value(1540), s.ad_value(1459), (4.0 * 2.0)), 1.0, s.ad_value(1500), s.ad_value(1541), 4.0), 335);
                s.store_div_ad_lhs(1520, A::add_scaled_product(A::add_scaled_products(s.ad_value(1462), s.ad_value(1540), (4.0 * 2.0), s.ad_value(1540), s.ad_value(1459), (-8.0)), 1.0, s.ad_value(1500), s.ad_value(1541), 4.0), 335);
                s.store_mul_sub_rhs(335, 154, 1459, 1481);
                s.store_exp(336, 335);
            }
            s.b[1604] = (s.v[1459] >= s.v[1481]);
            s.v[1604] = if s.b[1604] { 1.0 } else { 0.0 };
            if ((s.b[1441] && s.b[1442]) && s.b[1604]) {
                s.store_mul_scaled_ad_rhs(1473, 209, -1.0, A::sqrt(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15)));
                s.store_mul_ad(1521, A::div(A::mul_scaled_lhs(s.ad_value(209), 0.5, s.ad_value(209)), s.ad_value(1473)), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
                s.store_neg(1523, 1521);
            }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1604])) {
                s.store_exp_ad(337, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1459), s.ad_value(1433))));
                s.store_exp_ad(338, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1481), s.ad_value(1433))));
                s.store_mul_sqrt_ad_rhs(1473, 209, A::add_scaled_product(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15), 1.0, s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0));
                s.store_div_ad_lhs(339, A::mul_scaled_lhs(s.ad_value(209), 0.5, s.ad_value(209)), 1473);
                s.store_mul_add_ad_rhs(1521, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
                s.store_mul_add_ad_rhs(1523, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));
            }
            s.b[1605] = ((s.v[1518] > (s.v[1509] - s.v[1517])) && (s.v[1517] >= 0.0));
            s.v[1605] = if s.b[1605] { 1.0 } else { 0.0 };
            if ((s.b[1441] && s.b[1442]) && s.b[1605]) {
                s.store_add_ad_lhs(781, A::sub(s.ad_value(1518), s.ad_value(1509)), 1517);
                s.store_square(722, 781);
                s.store_square(723, 1517);
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
            s.b[1606] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.v[1606] = if s.b[1606] { 1.0 } else { 0.0 };
            s.b[1607] = (4.0 == 1.0);
            s.v[1607] = if s.b[1607] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && s.b[1442]) && s.b[1605]) && s.b[1606]) && s.b[1607]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1608] = (4.0 == 2.0);
            s.v[1608] = if s.b[1608] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && s.b[1442]) && s.b[1605]) && s.b[1606]) && (!s.b[1607])) && s.b[1608]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1609] = (4.0 == 4.0);
            s.v[1609] = if s.b[1609] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && s.b[1442]) && s.b[1605]) && s.b[1606]) && (!s.b[1607])) && (!s.b[1608])) && s.b[1609]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1610] = (4.0 == 8.0);
            s.v[1610] = if s.b[1610] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && s.b[1442]) && s.b[1605]) && s.b[1606]) && (!s.b[1607])) && (!s.b[1608])) && (!s.b[1609])) && s.b[1610]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1605]) && s.b[1606]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign25270_body126_loop_guard: usize = 0;
            while {
                let assign25270_body126_cond_e23187: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1605]) && s.b[1606]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign25270_body126_cond_e23187 != 0.0
            } {
                assign25270_body126_loop_guard += 1;
                assert!(assign25270_body126_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && s.b[1442]) && s.b[1605]) && s.b[1606]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1605]) && (!s.b[1606])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                    }
                });
            }
            if ((s.b[1441] && s.b[1442]) && s.b[1605]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_mul3_lhs(780, 781, 1517, 726);
                s.store_div_ad_lhs(334, A::mul3(s.ad_value(1517), s.ad_value(725), s.ad_value(726)), 770);
                s.store_add_ad_lhs(335, A::sub(s.ad_value(1509), s.ad_value(1517)), 780);
            }
            if ((s.b[1441] && s.b[1442]) && s.b[1605]) {
            }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1605])) {
                s.copy_ad(335, 1518);
                s.store_scalar(334, 1.0);
            }
            if (s.b[1441] && s.b[1442]) {
                s.store_sub(1483, 1481, 335);
                s.store_mul_neg_lhs(1485, 1519, 334);
                s.store_sub_from_scalar_ad(1486, 1.0, A::mul3(s.ad_value(1520), s.ad_value(1531), s.ad_value(334)));
                s.store_add_ad_lhs(1484, A::add(A::add_scaled_product(s.ad_value(1473), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1459)), 1.0), s.ad_value(1495)), 1496);
                s.store_sub(1487, 1521, 185);
                s.store_ad_value(1488, A::add_scaled_product(A::add_scaled_product(A::add(s.ad_value(1523), s.ad_value(1525)), 1.0, s.ad_value(1527), s.ad_value(1531), 1.0), 1.0, s.ad_value(1529), s.ad_value(1531), 1.0));
                s.store_ad_value(1489, A::add_scaled_products(s.ad_value(1485), s.ad_value(1488), 1.0, s.ad_value(1487), s.ad_value(1486), (-1.0)));
                s.store_div(1490, 1488, 1489);
                s.store_div_ad_lhs(1491, A::neg(s.ad_value(1486)), 1489);
                s.store_div_ad_lhs(1492, A::neg(s.ad_value(1487)), 1489);
                s.store_div(1493, 1485, 1489);
            }
            s.b[1611] = (((((s.v[1492] * s.v[1483]) + (s.v[1493] * s.v[1484]))) as f64).abs() > 0.5);
            s.v[1611] = if s.b[1611] { 1.0 } else { 0.0 };
            if ((s.b[1441] && s.b[1442]) && s.b[1611]) {
                s.store_offset(1459, 1459, (-(0.5 * (if (((s.v[1490] * s.v[1483]) + (s.v[1491] * s.v[1484])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if ((s.b[1441] && s.b[1442]) && s.b[1611]) {
                s.store_offset(1481, 1481, (-(0.5 * (if (((s.v[1492] * s.v[1483]) + (s.v[1493] * s.v[1484])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1611])) {
                s.store_sub_ad_rhs(1459, 1459, A::add_scaled_products(s.ad_value(1490), s.ad_value(1483), 1.0, s.ad_value(1491), s.ad_value(1484), 1.0));
                s.store_sub_ad_rhs(1481, 1481, A::add_scaled_products(s.ad_value(1492), s.ad_value(1483), 1.0, s.ad_value(1493), s.ad_value(1484), 1.0));
            }
            s.b[1612] = (((((s.v[1459] - s.v[1467])) as f64).abs() <= 1e-12) && ((((s.v[1481] - s.v[1468])) as f64).abs() <= 1e-12));
            s.v[1612] = if s.b[1612] { 1.0 } else { 0.0 };
            if ((s.b[1441] && s.b[1442]) && s.b[1612]) {
                s.store_scalar(97, (150.0 + 1.0));
                s.store_scalar(79, 1.0);
            }
            if (s.b[1441] && s.b[1442]) {
                s.copy_ad(1467, 1459);
                s.copy_ad(1468, 1481);
                s.store_offset(97, 97, 1.0);
            }
        }

        s.b[1614] = ((s.v[1451] > s.v[965]) && (s.v[1477] != 2.0));
        s.v[1614] = if s.b[1614] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
    ) {
        s.b[1615] = ((s.v[1481] > (s.v[1459] - 0.02)) && (0.02 >= 0.0));
        s.v[1615] = if s.b[1615] { 1.0 } else { 0.0 };

        if (((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) {
            s.store_offset_sub(781, 1481, 1459, 0.02);
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

        s.b[1616] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1616] = if s.b[1616] { 1.0 } else { 0.0 };

        s.b[1617] = (2.0 == 1.0);
        s.v[1617] = if s.b[1617] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && s.b[1616]) && s.b[1617]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1618] = (2.0 == 2.0);
        s.v[1618] = if s.b[1618] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && s.b[1616]) && (!s.b[1617])) && s.b[1618]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1619] = (2.0 == 4.0);
        s.v[1619] = if s.b[1619] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && s.b[1616]) && (!s.b[1617])) && (!s.b[1618])) && s.b[1619]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1620] = (2.0 == 8.0);
        s.v[1620] = if s.b[1620] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && s.b[1616]) && (!s.b[1617])) && (!s.b[1618])) && (!s.b[1619])) && s.b[1620]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && s.b[1616]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign25560_loop_guard: usize = 0;
        while {
            let assign25560_cond_e23878: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && s.b[1616]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign25560_cond_e23878 != 0.0
        } {
            assign25560_loop_guard += 1;
            assert!(assign25560_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && s.b[1616]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && (!s.b[1616])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.02);
            s.store_div_ad_lhs(335, A::mul_scaled_lhs(s.ad_value(725), 0.02, s.ad_value(726)), 770);
            s.store_add_ad_lhs(1481, A::offset(s.ad_value(1459), (-0.02)), 780);
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) {
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1614]) && (!s.b[1615])) {
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1614]) && (!s.b[1615])) {
            s.store_scalar(335, 1.0);
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_mul_sub_ad_rhs(1462, 1533, A::add_scaled_product(s.ad_value(1433), 1.0, s.ad_value(1534), s.ad_value(1481), 1.0), s.ad_value(1461));
            s.store_mul_sub_rhs(335, 154, 1459, 1481);
            s.store_exp(336, 335);
        }

        s.b[1621] = (s.v[1459] >= s.v[1481]);
        s.v[1621] = if s.b[1621] { 1.0 } else { 0.0 };

        if ((s.b[1441] && s.b[1442]) && s.b[1621]) {
            s.store_mul_scaled_ad_rhs(1473, 209, -1.0, A::sqrt(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15)));
            s.copy_ad(1536, 1473);
            s.store_scalar(1515, 0.0);
            s.store_scalar(1475, 0.0);
            s.store_sqrt_mul_ad(1445, s.ad_value(1545), A::sub(s.ad_value(1481), s.ad_value(1462)));
        }

        s.b[1622] = ((s.v[1445] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
        s.v[1622] = if s.b[1622] { 1.0 } else { 0.0 };

        if (((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) {
            s.store_offset_sub(781, 1445, 965, 1e-8);
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

        s.b[1623] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1623] = if s.b[1623] { 1.0 } else { 0.0 };

        s.b[1624] = (2.0 == 1.0);
        s.v[1624] = if s.b[1624] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && s.b[1623]) && s.b[1624]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1625] = (2.0 == 2.0);
        s.v[1625] = if s.b[1625] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && s.b[1623]) && (!s.b[1624])) && s.b[1625]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1626] = (2.0 == 4.0);
        s.v[1626] = if s.b[1626] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && s.b[1623]) && (!s.b[1624])) && (!s.b[1625])) && s.b[1626]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1627] = (2.0 == 8.0);
        s.v[1627] = if s.b[1627] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && s.b[1623]) && (!s.b[1624])) && (!s.b[1625])) && (!s.b[1626])) && s.b[1627]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && s.b[1623]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign26000_loop_guard: usize = 0;
        while {
            let assign26000_cond_e24409: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && s.b[1623]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign26000_cond_e24409 != 0.0
        } {
            assign26000_loop_guard += 1;
            assert!(assign26000_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && s.b[1623]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && (!s.b[1623])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_ad_lhs(337, A::mul_scaled_lhs(s.ad_value(725), 1e-8, s.ad_value(726)), 770);
            s.store_add_ad_lhs(1445, A::offset(s.ad_value(965), (-1e-8)), 780);
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) {
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1621]) && (!s.b[1622])) {
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1621]) && (!s.b[1622])) {
            s.store_scalar(337, 1.0);
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1621]) {
            s.store_sqrt_mul_ad(1449, s.ad_value(1548), A::add(A::sub(s.ad_value(1462), s.ad_value(1433)), s.ad_value(1461)));
            s.store_mul(1495, 1445, 1544);
            s.store_mul_neg_lhs(1496, 1449, 1542);
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1621])) {
            s.store_exp_ad(337, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1459), s.ad_value(1433))));
            s.store_exp_ad(338, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1481), s.ad_value(1433))));
            s.store_mul_sqrt_ad_rhs(1473, 209, A::offset(A::add_scaled_product(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1.0, s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
        }

        s.b[1628] = ((s.v[1451] > s.v[965]) && (s.v[1477] != 2.0));
        s.v[1628] = if s.b[1628] { 1.0 } else { 0.0 };

        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1628]) {
            s.store_scalar(1475, 0.0);
            s.store_scalar(1515, 0.0);
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && (!s.b[1628])) {
            s.store_mul_sqrt_ad_rhs(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1459), s.ad_value(1433)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1481), s.ad_value(1433)))))), s.ad_value(335)));
            s.store_ad_value(1475, A::add_scaled_product(s.ad_value(337), 1.0, s.ad_value(209), A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0)));
            s.store_mul_sqrt_ad_rhs(1515, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1621])) {
            s.store_scalar(1536, 0.0);
            s.store_sub(335, 1481, 1462);
        }

        s.b[1629] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
        s.v[1629] = if s.b[1629] { 1.0 } else { 0.0 };

        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) {
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

        s.b[1630] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1630] = if s.b[1630] { 1.0 } else { 0.0 };

        s.b[1631] = (2.0 == 1.0);
        s.v[1631] = if s.b[1631] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && s.b[1630]) && s.b[1631]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1632] = (2.0 == 2.0);
        s.v[1632] = if s.b[1632] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && s.b[1630]) && (!s.b[1631])) && s.b[1632]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1633] = (2.0 == 4.0);
        s.v[1633] = if s.b[1633] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && s.b[1630]) && (!s.b[1631])) && (!s.b[1632])) && s.b[1633]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1634] = (2.0 == 8.0);
        s.v[1634] = if s.b[1634] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && s.b[1630]) && (!s.b[1631])) && (!s.b[1632])) && (!s.b[1633])) && s.b[1634]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && s.b[1630]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign26490_loop_guard: usize = 0;
        while {
            let assign26490_cond_e25084: f64 = if (((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && s.b[1630]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign26490_cond_e25084 != 0.0
        } {
            assign26490_loop_guard += 1;
            assert!(assign26490_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && s.b[1630]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && (!s.b[1630])) {
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
        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_ad_lhs(334, A::mul_scaled_lhs(s.ad_value(725), 0.1, s.ad_value(726)), 770);
            s.store_sub_from_scalar(336, 0.1, 780);
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) {
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && (!s.b[1629])) {
            s.copy_ad(336, 335);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1621])) {
            s.store_sqrt_mul(1445, 1545, 336);
        }

        s.b[1635] = ((s.v[1445] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
        s.v[1635] = if s.b[1635] { 1.0 } else { 0.0 };

        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) {
            s.store_offset_sub(781, 1445, 965, 1e-8);
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

        s.b[1636] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1636] = if s.b[1636] { 1.0 } else { 0.0 };

        s.b[1637] = (2.0 == 1.0);
        s.v[1637] = if s.b[1637] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && s.b[1636]) && s.b[1637]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1638] = (2.0 == 2.0);
        s.v[1638] = if s.b[1638] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && s.b[1636]) && (!s.b[1637])) && s.b[1638]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1639] = (2.0 == 4.0);
        s.v[1639] = if s.b[1639] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && s.b[1636]) && (!s.b[1637])) && (!s.b[1638])) && s.b[1639]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1640] = (2.0 == 8.0);
        s.v[1640] = if s.b[1640] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && s.b[1636]) && (!s.b[1637])) && (!s.b[1638])) && (!s.b[1639])) && s.b[1640]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && s.b[1636]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign26850_loop_guard: usize = 0;
        while {
            let assign26850_cond_e25569: f64 = if (((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && s.b[1636]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign26850_cond_e25569 != 0.0
        } {
            assign26850_loop_guard += 1;
            assert!(assign26850_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && s.b[1636]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && (!s.b[1636])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_ad_lhs(337, A::mul_scaled_lhs(s.ad_value(725), 1e-8, s.ad_value(726)), 770);
            s.store_add_ad_lhs(1445, A::offset(s.ad_value(965), (-1e-8)), 780);
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) {
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && (!s.b[1635])) {
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && (!s.b[1635])) {
            s.store_scalar(337, 1.0);
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1621])) {
            s.store_sqrt_mul_ad(1449, s.ad_value(1548), A::add(A::sub(s.ad_value(1462), s.ad_value(1433)), s.ad_value(1461)));
            s.store_mul(1495, 1445, 1544);
            s.store_mul_neg_lhs(1496, 1449, 1542);
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_sub(335, 1481, 1462);
        }

        s.b[1641] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
        s.v[1641] = if s.b[1641] { 1.0 } else { 0.0 };

        if ((s.b[1441] && s.b[1442]) && s.b[1641]) {
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

        s.b[1642] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1642] = if s.b[1642] { 1.0 } else { 0.0 };

        s.b[1643] = (2.0 == 1.0);
        s.v[1643] = if s.b[1643] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && s.b[1442]) && s.b[1641]) && s.b[1642]) && s.b[1643]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1644] = (2.0 == 2.0);
        s.v[1644] = if s.b[1644] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && s.b[1442]) && s.b[1641]) && s.b[1642]) && (!s.b[1643])) && s.b[1644]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1645] = (2.0 == 4.0);
        s.v[1645] = if s.b[1645] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && s.b[1442]) && s.b[1641]) && s.b[1642]) && (!s.b[1643])) && (!s.b[1644])) && s.b[1645]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1646] = (2.0 == 8.0);
        s.v[1646] = if s.b[1646] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && s.b[1442]) && s.b[1641]) && s.b[1642]) && (!s.b[1643])) && (!s.b[1644])) && (!s.b[1645])) && s.b[1646]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1641]) && s.b[1642]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign27240_loop_guard: usize = 0;
        while {
            let assign27240_cond_e26026: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1641]) && s.b[1642]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign27240_cond_e26026 != 0.0
        } {
            assign27240_loop_guard += 1;
            assert!(assign27240_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && s.b[1442]) && s.b[1641]) && s.b[1642]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1641]) && (!s.b[1642])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1641]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_ad_lhs(334, A::mul_scaled_lhs(s.ad_value(725), 0.1, s.ad_value(726)), 770);
            s.store_sub_from_scalar(336, 0.1, 780);
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1641]) {
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1641])) {
            s.copy_ad(336, 335);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_sqrt_mul(1445, 1545, 336);
        }

        s.b[1647] = ((s.v[1445] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
        s.v[1647] = if s.b[1647] { 1.0 } else { 0.0 };

        if ((s.b[1441] && s.b[1442]) && s.b[1647]) {
            s.store_offset_sub(781, 1445, 965, 1e-8);
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

        s.b[1648] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1648] = if s.b[1648] { 1.0 } else { 0.0 };

        s.b[1649] = (2.0 == 1.0);
        s.v[1649] = if s.b[1649] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && s.b[1442]) && s.b[1647]) && s.b[1648]) && s.b[1649]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1650] = (2.0 == 2.0);
        s.v[1650] = if s.b[1650] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && s.b[1442]) && s.b[1647]) && s.b[1648]) && (!s.b[1649])) && s.b[1650]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1651] = (2.0 == 4.0);
        s.v[1651] = if s.b[1651] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && s.b[1442]) && s.b[1647]) && s.b[1648]) && (!s.b[1649])) && (!s.b[1650])) && s.b[1651]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1652] = (2.0 == 8.0);
        s.v[1652] = if s.b[1652] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && s.b[1442]) && s.b[1647]) && s.b[1648]) && (!s.b[1649])) && (!s.b[1650])) && (!s.b[1651])) && s.b[1652]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1647]) && s.b[1648]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign27600_loop_guard: usize = 0;
        while {
            let assign27600_cond_e26415: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1647]) && s.b[1648]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign27600_cond_e26415 != 0.0
        } {
            assign27600_loop_guard += 1;
            assert!(assign27600_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && s.b[1442]) && s.b[1647]) && s.b[1648]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1647]) && (!s.b[1648])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1647]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_ad_lhs(337, A::mul_scaled_lhs(s.ad_value(725), 1e-8, s.ad_value(726)), 770);
            s.store_add_ad_lhs(1445, A::offset(s.ad_value(965), (-1e-8)), 780);
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1647]) {
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1647])) {
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1647])) {
            s.store_scalar(337, 1.0);
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_sub(335, 1481, 1459);
        }

        s.b[1653] = ((s.v[335] < 0.05) && (0.05 >= 0.0));
        s.v[1653] = if s.b[1653] { 1.0 } else { 0.0 };

        if ((s.b[1441] && s.b[1442]) && s.b[1653]) {
            s.store_sub_from_scalar(781, 0.05, 335);
        }

    }

    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
    ) {
        if ((s.b[1441] && s.b[1442]) && s.b[1653]) {
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

        s.b[1654] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1654] = if s.b[1654] { 1.0 } else { 0.0 };

        s.b[1655] = (2.0 == 1.0);
        s.v[1655] = if s.b[1655] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && s.b[1442]) && s.b[1653]) && s.b[1654]) && s.b[1655]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1656] = (2.0 == 2.0);
        s.v[1656] = if s.b[1656] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && s.b[1442]) && s.b[1653]) && s.b[1654]) && (!s.b[1655])) && s.b[1656]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1657] = (2.0 == 4.0);
        s.v[1657] = if s.b[1657] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && s.b[1442]) && s.b[1653]) && s.b[1654]) && (!s.b[1655])) && (!s.b[1656])) && s.b[1657]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1658] = (2.0 == 8.0);
        s.v[1658] = if s.b[1658] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && s.b[1442]) && s.b[1653]) && s.b[1654]) && (!s.b[1655])) && (!s.b[1656])) && (!s.b[1657])) && s.b[1658]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1653]) && s.b[1654]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign27960_loop_guard: usize = 0;
        while {
            let assign27960_cond_e26803: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1653]) && s.b[1654]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign27960_cond_e26803 != 0.0
        } {
            assign27960_loop_guard += 1;
            assert!(assign27960_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && s.b[1442]) && s.b[1653]) && s.b[1654]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1653]) && (!s.b[1654])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1653]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.05);
            s.store_div_ad_lhs(334, A::mul_scaled_lhs(s.ad_value(725), 0.05, s.ad_value(726)), 770);
            s.store_sub_from_scalar(336, 0.05, 780);
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1653]) {
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1653])) {
            s.copy_ad(336, 335);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_sqrt_mul(1447, 1545, 336);
            s.store_sub_ad_lhs(335, A::sub(s.ad_value(965), s.ad_value(1445)), 1447);
        }

        s.b[1659] = ((s.v[335] < (1e-25 + 1e-18)) && (1e-18 >= 0.0));
        s.v[1659] = if s.b[1659] { 1.0 } else { 0.0 };

        if ((s.b[1441] && s.b[1442]) && s.b[1659]) {
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

        s.b[1660] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1660] = if s.b[1660] { 1.0 } else { 0.0 };

        s.b[1661] = (2.0 == 1.0);
        s.v[1661] = if s.b[1661] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && s.b[1442]) && s.b[1659]) && s.b[1660]) && s.b[1661]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1662] = (2.0 == 2.0);
        s.v[1662] = if s.b[1662] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && s.b[1442]) && s.b[1659]) && s.b[1660]) && (!s.b[1661])) && s.b[1662]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1663] = (2.0 == 4.0);
        s.v[1663] = if s.b[1663] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && s.b[1442]) && s.b[1659]) && s.b[1660]) && (!s.b[1661])) && (!s.b[1662])) && s.b[1663]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1664] = (2.0 == 8.0);
        s.v[1664] = if s.b[1664] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && s.b[1442]) && s.b[1659]) && s.b[1660]) && (!s.b[1661])) && (!s.b[1662])) && (!s.b[1663])) && s.b[1664]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1659]) && s.b[1660]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign28330_loop_guard: usize = 0;
        while {
            let assign28330_cond_e27202: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1659]) && s.b[1660]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign28330_cond_e27202 != 0.0
        } {
            assign28330_loop_guard += 1;
            assert!(assign28330_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && s.b[1442]) && s.b[1659]) && s.b[1660]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1659]) && (!s.b[1660])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1659]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-18);
            s.store_div_ad_lhs(334, A::mul_scaled_lhs(s.ad_value(725), 1e-18, s.ad_value(726)), 770);
            s.store_sub_from_scalar(1499, (1e-25 + 1e-18), 780);
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1659]) {
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1659])) {
            s.copy_ad(1499, 335);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_mul_neg_lhs(1494, 1499, 1544);
        }

        s.b[1665] = ((s.v[1451] > s.v[965]) && (s.v[1477] != 2.0));
        s.v[1665] = if s.b[1665] { 1.0 } else { 0.0 };

        s.b[1666] = ((s.v[1459] > (s.v[1509] - 0.8)) && (0.8 >= 0.0));
        s.v[1666] = if s.b[1666] { 1.0 } else { 0.0 };

        if (((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) {
            s.store_offset_sub(781, 1459, 1509, 0.8);
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

        s.b[1667] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1667] = if s.b[1667] { 1.0 } else { 0.0 };

        s.b[1668] = (2.0 == 1.0);
        s.v[1668] = if s.b[1668] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && s.b[1667]) && s.b[1668]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1669] = (2.0 == 2.0);
        s.v[1669] = if s.b[1669] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && s.b[1667]) && (!s.b[1668])) && s.b[1669]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1670] = (2.0 == 4.0);
        s.v[1670] = if s.b[1670] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && s.b[1667]) && (!s.b[1668])) && (!s.b[1669])) && s.b[1670]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1671] = (2.0 == 8.0);
        s.v[1671] = if s.b[1671] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && s.b[1667]) && (!s.b[1668])) && (!s.b[1669])) && (!s.b[1670])) && s.b[1671]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && s.b[1667]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign28700_loop_guard: usize = 0;
        while {
            let assign28700_cond_e27640: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && s.b[1667]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign28700_cond_e27640 != 0.0
        } {
            assign28700_loop_guard += 1;
            assert!(assign28700_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && s.b[1667]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && (!s.b[1667])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.8);
            s.store_div_ad_lhs(335, A::mul_scaled_lhs(s.ad_value(725), 0.8, s.ad_value(726)), 770);
            s.store_add_ad_lhs(336, A::offset(s.ad_value(1509), (-0.8)), 780);
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) {
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1665]) && (!s.b[1666])) {
            s.copy_ad(336, 1459);
            s.store_scalar(335, 1.0);
        }

        s.b[1672] = ((s.v[1518] > (s.v[1509] - 0.8)) && (0.8 >= 0.0));
        s.v[1672] = if s.b[1672] { 1.0 } else { 0.0 };

        if (((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) {
            s.store_offset_sub(781, 1518, 1509, 0.8);
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
        if (((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) {
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1673] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1673] = if s.b[1673] { 1.0 } else { 0.0 };

        s.b[1674] = (2.0 == 1.0);
        s.v[1674] = if s.b[1674] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && s.b[1673]) && s.b[1674]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1675] = (2.0 == 2.0);
        s.v[1675] = if s.b[1675] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && s.b[1673]) && (!s.b[1674])) && s.b[1675]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1676] = (2.0 == 4.0);
        s.v[1676] = if s.b[1676] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && s.b[1673]) && (!s.b[1674])) && (!s.b[1675])) && s.b[1676]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1677] = (2.0 == 8.0);
        s.v[1677] = if s.b[1677] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && s.b[1673]) && (!s.b[1674])) && (!s.b[1675])) && (!s.b[1676])) && s.b[1677]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && s.b[1673]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign29050_loop_guard: usize = 0;
        while {
            let assign29050_cond_e28103: f64 = if (((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && s.b[1673]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign29050_cond_e28103 != 0.0
        } {
            assign29050_loop_guard += 1;
            assert!(assign29050_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && s.b[1673]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && (!s.b[1673])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.8);
            s.store_div_ad_lhs(334, A::mul_scaled_lhs(s.ad_value(725), 0.8, s.ad_value(726)), 770);
            s.store_add_ad_lhs(336, A::offset(s.ad_value(1509), (-0.8)), 780);
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) {
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1665])) && (!s.b[1672])) {
            s.copy_ad(336, 1518);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_mul_ad_affine_product_lhs(1503, s.ad_value(964), A::exp(A::mul(s.ad_value(154), A::sub(s.ad_value(336), s.ad_value(1509)))), (-1.6021918e-19), 0.0, 1445);
        }

        s.b[1678] = (((s.v[1459] - s.v[1509]) < 0.06) && (0.06 >= 0.0));
        s.v[1678] = if s.b[1678] { 1.0 } else { 0.0 };

        if ((s.b[1441] && s.b[1442]) && s.b[1678]) {
            s.store_sub_from_scalar_ad(781, 0.06, A::sub(s.ad_value(1459), s.ad_value(1509)));
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

        s.b[1679] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1679] = if s.b[1679] { 1.0 } else { 0.0 };

        s.b[1680] = (2.0 == 1.0);
        s.v[1680] = if s.b[1680] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && s.b[1442]) && s.b[1678]) && s.b[1679]) && s.b[1680]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1681] = (2.0 == 2.0);
        s.v[1681] = if s.b[1681] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && s.b[1442]) && s.b[1678]) && s.b[1679]) && (!s.b[1680])) && s.b[1681]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1682] = (2.0 == 4.0);
        s.v[1682] = if s.b[1682] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && s.b[1442]) && s.b[1678]) && s.b[1679]) && (!s.b[1680])) && (!s.b[1681])) && s.b[1682]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1683] = (2.0 == 8.0);
        s.v[1683] = if s.b[1683] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && s.b[1442]) && s.b[1678]) && s.b[1679]) && (!s.b[1680])) && (!s.b[1681])) && (!s.b[1682])) && s.b[1683]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1678]) && s.b[1679]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign29410_loop_guard: usize = 0;
        while {
            let assign29410_cond_e28535: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1678]) && s.b[1679]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign29410_cond_e28535 != 0.0
        } {
            assign29410_loop_guard += 1;
            assert!(assign29410_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && s.b[1442]) && s.b[1678]) && s.b[1679]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1678]) && (!s.b[1679])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1678]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.06);
            s.store_div_ad_lhs(334, A::mul_scaled_lhs(s.ad_value(725), 0.06, s.ad_value(726)), 770);
            s.store_sub_from_scalar(336, 0.06, 780);
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1678]) {
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1678])) {
            s.store_sub(336, 1459, 1509);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_offset_ad(338, A::add_scaled_product(A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0)), (10.0 * 2.220446049250313e-16));
            s.store_mul_scaled_ad_rhs(1513, 209, -1.0, A::sqrt(s.ad_value(338)));
            s.store_sub_scaled_ad_lhs(338, A::offset(A::exp_scaled_input(s.ad_value(154), 0.1), (-1.0)), 154, 0.1);
            s.store_mul_sqrt_rhs(1538, 209, 338);
            s.copy_ad(349, 790);
        }

        s.b[1684] = (s.v[790] > 1e-6);
        s.v[1684] = if s.b[1684] { 1.0 } else { 0.0 };

        if ((s.b[1441] && s.b[1442]) && s.b[1684]) {
            s.store_div_ad_rhs(336, 1500, A::square(s.ad_value(185)));
            s.store_sub_ad_lhs(334, A::sub(A::offset(s.ad_value(85), 2.0), s.ad_value(155)), 1436);
            s.store_offset_mul_ad(338, A::div_from_scalar(2.0, s.ad_value(336)), s.ad_value(334), 1.0);
        }

        s.b[1685] = ((s.v[338] < 2.0) && (2.0 >= 0.0));
        s.v[1685] = if s.b[1685] { 1.0 } else { 0.0 };

        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) {
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

        s.b[1686] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1686] = if s.b[1686] { 1.0 } else { 0.0 };

        s.b[1687] = (2.0 == 1.0);
        s.v[1687] = if s.b[1687] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && s.b[1686]) && s.b[1687]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1688] = (2.0 == 2.0);
        s.v[1688] = if s.b[1688] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && s.b[1686]) && (!s.b[1687])) && s.b[1688]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1689] = (2.0 == 4.0);
        s.v[1689] = if s.b[1689] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && s.b[1686]) && (!s.b[1687])) && (!s.b[1688])) && s.b[1689]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1690] = (2.0 == 8.0);
        s.v[1690] = if s.b[1690] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && s.b[1686]) && (!s.b[1687])) && (!s.b[1688])) && (!s.b[1689])) && s.b[1690]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && s.b[1686]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign29850_loop_guard: usize = 0;
        while {
            let assign29850_cond_e29061: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && s.b[1686]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign29850_cond_e29061 != 0.0
        } {
            assign29850_loop_guard += 1;
            assert!(assign29850_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && s.b[1686]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && (!s.b[1686])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 2.0);
            s.store_div_ad_lhs(334, A::mul_scaled_lhs(s.ad_value(725), 2.0, s.ad_value(726)), 770);
            s.store_sub_from_scalar(343, 2.0, 780);
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) {
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && (!s.b[1685])) {
            s.copy_ad(343, 338);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1684]) {
            s.store_offset(343, 343, 1e-25);
            s.store_sqrt(337, 343);
            s.store_mul_sub_from_scalar_rhs(338, 336, 1.0, 337);
            s.store_add_ad_lhs(344, A::offset(s.ad_value(85), 2.0), 338);
        }

        s.b[1691] = ((s.v[344] < (0.3 + 0.2)) && (0.2 >= 0.0));
        s.v[1691] = if s.b[1691] { 1.0 } else { 0.0 };

        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) {
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
        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) {
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1692] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1692] = if s.b[1692] { 1.0 } else { 0.0 };

        s.b[1693] = (4.0 == 1.0);
        s.v[1693] = if s.b[1693] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) && s.b[1692]) && s.b[1693]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1694] = (4.0 == 2.0);
        s.v[1694] = if s.b[1694] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) && s.b[1692]) && (!s.b[1693])) && s.b[1694]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1695] = (4.0 == 4.0);
        s.v[1695] = if s.b[1695] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) && s.b[1692]) && (!s.b[1693])) && (!s.b[1694])) && s.b[1695]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1696] = (4.0 == 8.0);
        s.v[1696] = if s.b[1696] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) && s.b[1692]) && (!s.b[1693])) && (!s.b[1694])) && (!s.b[1695])) && s.b[1696]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) && s.b[1692]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign30280_loop_guard: usize = 0;
        while {
            let assign30280_cond_e29594: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) && s.b[1692]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign30280_cond_e29594 != 0.0
        } {
            assign30280_loop_guard += 1;
            assert!(assign30280_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) && s.b[1692]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) && (!s.b[1692])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                }
            });
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.2);
            s.store_div_ad_lhs(334, A::mul_scaled_lhs(s.ad_value(725), 0.2, s.ad_value(726)), 770);
            s.store_sub_from_scalar(344, (0.3 + 0.2), 780);
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) {
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && (!s.b[1691])) {
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && (!s.b[1691])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1684]) {
            s.store_offset(344, 344, (10.0 * 2.220446049250313e-16));
            s.store_div(335, 790, 344);
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1684]) {
            s.store_ad_value(336, {
                if (s.v[335] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(335), A::offset(s.ad_value(658), (-1.0)))
                }
            });
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1684]) {
            s.store_mul(341, 336, 335);
            s.store_offset(337, 341, 1.0);
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1684]) {
            s.store_ad_value(338, {
                if (s.v[337] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)))
                }
            });
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1684]) {
            s.store_mul(340, 338, 337);
            s.store_div(348, 790, 340);
        }

        s.b[1697] = ((s.v[85] < 0.5) && (0.5 >= 0.0));
        s.v[1697] = if s.b[1697] { 1.0 } else { 0.0 };

        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1697]) {
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

        s.b[1698] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1698] = if s.b[1698] { 1.0 } else { 0.0 };

        s.b[1699] = (2.0 == 1.0);
        s.v[1699] = if s.b[1699] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1697]) && s.b[1698]) && s.b[1699]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1700] = (2.0 == 2.0);
        s.v[1700] = if s.b[1700] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1697]) && s.b[1698]) && (!s.b[1699])) && s.b[1700]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1701] = (2.0 == 4.0);
        s.v[1701] = if s.b[1701] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1697]) && s.b[1698]) && (!s.b[1699])) && (!s.b[1700])) && s.b[1701]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1702] = (2.0 == 8.0);
        s.v[1702] = if s.b[1702] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1697]) && s.b[1698]) && (!s.b[1699])) && (!s.b[1700])) && (!s.b[1701])) && s.b[1702]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1697]) && s.b[1698]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign30710_loop_guard: usize = 0;
        while {
            let assign30710_cond_e30134: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1697]) && s.b[1698]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign30710_cond_e30134 != 0.0
        } {
            assign30710_loop_guard += 1;
            assert!(assign30710_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1697]) && s.b[1698]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1697]) && (!s.b[1698])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1697]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.5);
            s.store_div_ad_lhs(334, A::mul_scaled_lhs(s.ad_value(725), 0.5, s.ad_value(726)), 770);
            s.store_sub_from_scalar(1535, 0.5, 780);
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1697]) {
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && (!s.b[1697])) {
            s.copy_ad(1535, 85);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1684]) {
            s.store_scale(335, 1535, 0.8);
        }

        s.b[1703] = ((s.v[348] > (s.v[1535] - s.v[335])) && (s.v[335] >= 0.0));
        s.v[1703] = if s.b[1703] { 1.0 } else { 0.0 };

        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1703]) {
            s.store_add_ad_lhs(781, A::sub(s.ad_value(348), s.ad_value(1535)), 335);
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

        s.b[1704] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1704] = if s.b[1704] { 1.0 } else { 0.0 };

        s.b[1705] = (2.0 == 1.0);
        s.v[1705] = if s.b[1705] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1703]) && s.b[1704]) && s.b[1705]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1706] = (2.0 == 2.0);
        s.v[1706] = if s.b[1706] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1703]) && s.b[1704]) && (!s.b[1705])) && s.b[1706]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1707] = (2.0 == 4.0);
        s.v[1707] = if s.b[1707] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1703]) && s.b[1704]) && (!s.b[1705])) && (!s.b[1706])) && s.b[1707]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1708] = (2.0 == 8.0);
        s.v[1708] = if s.b[1708] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1703]) && s.b[1704]) && (!s.b[1705])) && (!s.b[1706])) && (!s.b[1707])) && s.b[1708]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1703]) && s.b[1704]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign31070_loop_guard: usize = 0;
        while {
            let assign31070_cond_e30586: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1703]) && s.b[1704]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign31070_cond_e30586 != 0.0
        } {
            assign31070_loop_guard += 1;
            assert!(assign31070_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1703]) && s.b[1704]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1703]) && (!s.b[1704])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1703]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_div_ad_lhs(334, A::mul3(s.ad_value(335), s.ad_value(725), s.ad_value(726)), 770);
            s.store_add_ad_lhs(790, A::sub(s.ad_value(1535), s.ad_value(335)), 780);
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1703]) {
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && (!s.b[1703])) {
            s.copy_ad(790, 348);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1684])) {
            s.copy_ad(348, 790);
        }

        s.b[1709] = (s.v[790] <= 0.0);
        s.v[1709] = if s.b[1709] { 1.0 } else { 0.0 };

        if ((s.b[1441] && s.b[1442]) && s.b[1709]) {
            s.copy_ad(1460, 1459);
            s.copy_ad(1482, 1481);
            s.copy_ad(1463, 1462);
            s.copy_ad(1476, 1475);
            s.copy_ad(1537, 1536);
            s.copy_ad(1497, 1495);
            s.copy_ad(1498, 1496);
            s.copy_ad(1516, 1515);
            s.copy_ad(1514, 1513);
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1709])) {
            s.store_sqrt_mul_ad(1452, A::div(A::mul(s.ad_value(1545), s.ad_value(622)), A::add(s.ad_value(622), s.ad_value(964))), A::add(A::sub(s.ad_value(790), s.ad_value(1433)), s.ad_value(1461)));
        }

        s.b[1710] = (s.v[1452] > s.v[965]);
        s.v[1710] = if s.b[1710] { 1.0 } else { 0.0 };

        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) {
            s.copy_ad(1464, 790);
        }

    }

    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
    ) {
        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) {
            s.copy_ad(1446, 965);
            s.copy_ad(1482, 790);
            s.copy_ad(1510, 790);
            s.store_sub_ad_rhs(1463, 1482, A::mul3(s.ad_value(1546), s.ad_value(1446), s.ad_value(1446)));
            s.copy_ad(1508, 1464);
            s.copy_ad(1471, 1463);
            s.store_mul(1497, 1446, 1544);
            s.store_scalar(97, 1.0);
        }

        let mut assign31380_loop_guard: usize = 0;
        while {
            let assign31380_cond_e30953: f64 = (150.0 + 1.0);
            let assign31380_cond_e30955: f64 = if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && (s.v[97] <= assign31380_cond_e30953)) { 1.0 } else { 0.0 };
            assign31380_cond_e30955 != 0.0
        } {
            assign31380_loop_guard += 1;
            assert!(assign31380_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) {
                s.store_sqrt_mul_ad(1446, s.ad_value(1545), A::sub(s.ad_value(1482), s.ad_value(1463)));
            }
            s.b[1711] = ((s.v[1446] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.v[1711] = if s.b[1711] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1711]) {
                s.store_offset_sub(781, 1446, 965, 1e-8);
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
            s.b[1712] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1712] = if s.b[1712] { 1.0 } else { 0.0 };
            s.b[1713] = (2.0 == 1.0);
            s.v[1713] = if s.b[1713] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1711]) && s.b[1712]) && s.b[1713]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1714] = (2.0 == 2.0);
            s.v[1714] = if s.b[1714] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1711]) && s.b[1712]) && (!s.b[1713])) && s.b[1714]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1715] = (2.0 == 4.0);
            s.v[1715] = if s.b[1715] { 1.0 } else { 0.0 };
            if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1711]) && s.b[1712]) && (!s.b[1713])) && (!s.b[1714])) && s.b[1715]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1716] = (2.0 == 8.0);
            s.v[1716] = if s.b[1716] { 1.0 } else { 0.0 };
            if (((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1711]) && s.b[1712]) && (!s.b[1713])) && (!s.b[1714])) && (!s.b[1715])) && s.b[1716]) {
                s.store_scalar(720, 4.0);
            }
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1711]) && s.b[1712]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign31380_body27_loop_guard: usize = 0;
            while {
                let assign31380_body27_cond_e31337: f64 = if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1711]) && s.b[1712]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign31380_body27_cond_e31337 != 0.0
            } {
                assign31380_body27_loop_guard += 1;
                assert!(assign31380_body27_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1711]) && s.b[1712]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1711]) && (!s.b[1712])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1711]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_ad_lhs(334, A::mul_scaled_lhs(s.ad_value(725), 1e-8, s.ad_value(726)), 770);
                s.store_add_ad_lhs(1446, A::offset(s.ad_value(965), (-1e-8)), 780);
            }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1711]) {
            }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && (!s.b[1711])) {
            }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && (!s.b[1711])) {
                s.store_scalar(334, 1.0);
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) {
                s.store_add_ad_lhs(335, A::sub(s.ad_value(1463), s.ad_value(1433)), 1461);
            }
            s.b[1717] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1717] = if s.b[1717] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1717]) {
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
            s.b[1718] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1718] = if s.b[1718] { 1.0 } else { 0.0 };
            s.b[1719] = (2.0 == 1.0);
            s.v[1719] = if s.b[1719] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1717]) && s.b[1718]) && s.b[1719]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1720] = (2.0 == 2.0);
            s.v[1720] = if s.b[1720] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1717]) && s.b[1718]) && (!s.b[1719])) && s.b[1720]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1721] = (2.0 == 4.0);
            s.v[1721] = if s.b[1721] { 1.0 } else { 0.0 };
            if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1717]) && s.b[1718]) && (!s.b[1719])) && (!s.b[1720])) && s.b[1721]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1722] = (2.0 == 8.0);
            s.v[1722] = if s.b[1722] { 1.0 } else { 0.0 };
            if (((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1717]) && s.b[1718]) && (!s.b[1719])) && (!s.b[1720])) && (!s.b[1721])) && s.b[1722]) {
                s.store_scalar(720, 4.0);
            }
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1717]) && s.b[1718]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign31380_body63_loop_guard: usize = 0;
            while {
                let assign31380_body63_cond_e31887: f64 = if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1717]) && s.b[1718]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign31380_body63_cond_e31887 != 0.0
            } {
                assign31380_body63_loop_guard += 1;
                assert!(assign31380_body63_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1717]) && s.b[1718]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1717]) && (!s.b[1718])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1717]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_ad_lhs(341, A::mul_scaled_lhs(s.ad_value(725), 0.1, s.ad_value(726)), 770);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1717]) {
            }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && (!s.b[1717])) {
                s.copy_ad(336, 335);
                s.store_scalar(341, 1.0);
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) {
                s.store_sqrt_mul(1450, 1548, 336);
                s.store_mul(1497, 1446, 1544);
                s.store_mul_div_from_scalar_lhs(1528, (-1.034943e-10), 1446, 334);
                s.store_mul_neg_lhs(1498, 1450, 1542);
                s.store_mul_div_from_scalar_lhs(1530, (-1.034943e-10), 1450, 341);
                s.store_add_ad_lhs(1483, A::add_scaled_product(s.ad_value(1497), 1.0, s.ad_value(185), A::sub(s.ad_value(1464), s.ad_value(1482)), 1.0), 1498);
                s.copy_ad(1485, 185);
                s.store_add(1486, 1528, 1530);
                s.store_ad_value(1484, A::add_scaled_product(s.ad_value(1463), 1.0, s.ad_value(1533), A::sub(A::add_scaled_product(s.ad_value(1433), 1.0, s.ad_value(1534), s.ad_value(1482), 1.0), s.ad_value(1461)), (-1.0)));
                s.store_scalar(1487, 0.0);
                s.store_scalar(1488, 1.0);
                s.store_ad_value(1489, A::add_scaled_products(s.ad_value(1485), s.ad_value(1488), 1.0, s.ad_value(1487), s.ad_value(1486), (-1.0)));
                s.store_div(1490, 1488, 1489);
                s.store_div_ad_lhs(1491, A::neg(s.ad_value(1486)), 1489);
                s.store_div_ad_lhs(1492, A::neg(s.ad_value(1487)), 1489);
                s.store_div(1493, 1485, 1489);
            }
            s.b[1723] = (((((s.v[1490] * s.v[1483]) + (s.v[1491] * s.v[1484]))) as f64).abs() > 0.5);
            s.v[1723] = if s.b[1723] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1723]) {
                s.store_offset(1464, 1464, (-(0.5 * (if (((s.v[1490] * s.v[1483]) + (s.v[1491] * s.v[1484])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1723]) {
                s.store_offset(1463, 1463, (-(0.5 * (if (((s.v[1492] * s.v[1483]) + (s.v[1493] * s.v[1484])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && (!s.b[1723])) {
                s.store_sub_ad_rhs(1464, 1464, A::add_scaled_products(s.ad_value(1490), s.ad_value(1483), 1.0, s.ad_value(1491), s.ad_value(1484), 1.0));
                s.store_sub_ad_rhs(1463, 1463, A::add_scaled_products(s.ad_value(1492), s.ad_value(1483), 1.0, s.ad_value(1493), s.ad_value(1484), 1.0));
            }
            s.b[1724] = (((((s.v[1464] - s.v[1508])) as f64).abs() <= 1e-12) && ((((s.v[1463] - s.v[1471])) as f64).abs() <= 1e-12));
            s.v[1724] = if s.b[1724] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1724]) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) {
                s.copy_ad(1508, 1464);
                s.copy_ad(1471, 1463);
                s.store_offset(97, 97, 1.0);
            }
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) {
            s.copy_ad(1512, 1463);
            s.store_mul(1450, 965, 1534);
            s.store_sub_ad_lhs(1463, A::add(A::mul3(s.ad_value(1549), s.ad_value(1450), s.ad_value(1450)), s.ad_value(1433)), 1461);
            s.store_ad_value(1482, A::add_scaled_product(s.ad_value(1463), 1.0, s.ad_value(1546), s.ad_value(1541), 1.0));
            s.copy_ad(1460, 1482);
            s.copy_ad(1465, 1482);
            s.copy_ad(1507, 1482);
        }

        s.b[1725] = (s.v[85] > s.v[1464]);
        s.v[1725] = if s.b[1725] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1725]) {
            s.store_scalar(1477, 1.0);
        }

        s.b[1726] = (s.v[85] > s.v[1507]);
        s.v[1726] = if s.b[1726] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && (!s.b[1725])) && s.b[1726]) {
            s.store_scalar(1477, 3.0);
        }

        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && (!s.b[1725])) && (!s.b[1726])) {
            s.store_scalar(1477, 2.0);
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1710])) {
            s.copy_ad(1464, 790);
            s.copy_ad(1507, 1464);
            s.copy_ad(1465, 1464);
            s.copy_ad(1510, 1464);
            s.copy_ad(1446, 1452);
            s.store_mul(1450, 1446, 1534);
            s.store_sub_ad_lhs(1463, A::add(A::mul3(s.ad_value(1549), s.ad_value(1450), s.ad_value(1450)), s.ad_value(1433)), 1461);
            s.store_add_ad_lhs(1482, A::mul3(s.ad_value(1546), s.ad_value(1446), s.ad_value(1446)), 1463);
            s.copy_ad(1512, 1463);
        }

        s.b[1727] = (s.v[85] > s.v[1464]);
        s.v[1727] = if s.b[1727] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1710])) && s.b[1727]) {
            s.store_scalar(1477, 1.0);
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1710])) && (!s.b[1727])) {
            s.store_scalar(1477, 2.0);
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1709])) {
            s.store_mul_sub_ad_rhs(335, 1547, s.ad_value(1465), A::sub(s.ad_value(1433), s.ad_value(961)));
        }

        s.b[1728] = (s.v[335] > 0.0);
        s.v[1728] = if s.b[1728] { 1.0 } else { 0.0 };

        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1728]) {
            s.store_sub_ad(1453, A::sub(s.ad_value(1433), s.ad_value(961)), A::div(A::sqrt(s.ad_value(335)), s.ad_value(185)));
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1728])) {
            s.store_sub(1453, 1433, 961);
        }

        s.b[1729] = (s.v[85] > s.v[1464]);
        s.v[1729] = if s.b[1729] { 1.0 } else { 0.0 };

        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1729]) {
            s.copy_ad(1463, 1512);
            s.copy_ad(1482, 790);
            s.store_add_ad_lhs(1479, A::div(A::ln(A::mul3(s.ad_value(1454), s.ad_value(85), s.ad_value(85))), A::add(s.ad_value(154), A::div_from_scalar(2.0, s.ad_value(85)))), 790);
        }

        s.b[1730] = (s.v[1479] < (s.v[1510] + s.v[1551]));
        s.v[1730] = if s.b[1730] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1729]) && s.b[1730]) {
            s.store_add(1479, 1510, 1551);
        }

        s.b[1731] = (s.v[85] > s.v[1507]);
        s.v[1731] = if s.b[1731] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && s.b[1731]) {
            s.copy_ad(1479, 1460);
        }

        s.b[1732] = (s.v[85] > s.v[1453]);
        s.v[1732] = if s.b[1732] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) {
            s.store_ad_value(1455, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(1454), s.ad_value(85), (-2.0)));
            s.store_ad_value(1456, A::add_scaled_product(A::mul3(s.ad_value(1454), s.ad_value(85), s.ad_value(85)), 1.0, s.ad_value(154), s.ad_value(1482), (-1.0)));
            s.copy_ad(1469, 1482);
            s.store_div_ad_lhs(1479, A::sub_scaled_inputs(A::sqrt(A::add_scaled_product(A::square(s.ad_value(1455)), 1.0, s.ad_value(1454), s.ad_value(1456), (-4.0))), 0.5, s.ad_value(1455), 0.5), 1454);
        }

        s.b[1733] = (s.v[1479] > (s.v[1465] - s.v[1551]));
        s.v[1733] = if s.b[1733] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
    ) {
        if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1733]) {
            s.store_sub(1479, 1465, 1551);
        }

        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) {
            s.store_sqrt_mul_ad(1448, s.ad_value(1545), A::sub(s.ad_value(1482), s.ad_value(1479)));
            s.store_sqrt_mul_ad(1446, s.ad_value(1545), A::sub(s.ad_value(1482), s.ad_value(1463)));
        }

        s.b[1734] = ((s.v[1448] + s.v[1446]) > s.v[965]);
        s.v[1734] = if s.b[1734] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) {
            s.store_scalar(97, 1.0);
        }

        let mut assign31860_loop_guard: usize = 0;
        while {
            let assign31860_cond_e33115: f64 = (150.0 + 1.0);
            let assign31860_cond_e33117: f64 = if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) && (s.v[97] <= assign31860_cond_e33115)) { 1.0 } else { 0.0 };
            assign31860_cond_e33117 != 0.0
        } {
            assign31860_loop_guard += 1;
            assert!(assign31860_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) {
                s.store_sub_ad_lhs(1466, A::add(s.ad_value(1448), s.ad_value(1446)), 965);
                s.store_add_ad(1506, A::div(A::div_from_scalar(1.034943e-10, s.ad_value(1544)), s.ad_value(1448)), A::div(A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(1544)), A::sub_from_scalar(1.0, A::div(s.ad_value(1534), A::offset(s.ad_value(1534), 1.0)))), s.ad_value(1446)));
            }
            s.b[1735] = ((((s.v[1466] / s.v[1506])) as f64).abs() > 0.5);
            s.v[1735] = if s.b[1735] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) && s.b[1735]) {
                s.store_offset(1482, 1482, (-(0.5 * (if ((s.v[1466] / s.v[1506]) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) && (!s.b[1735])) {
                s.store_sub_ad_rhs(1482, 1482, A::div(s.ad_value(1466), s.ad_value(1506)));
            }
            s.b[1736] = (((s.v[1482] - s.v[1433]) + s.v[1461]) < (10.0 * 2.220446049250313e-16));
            s.v[1736] = if s.b[1736] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) && s.b[1736]) {
                s.store_offset_sub(1482, 1433, 1461, (10.0 * 2.220446049250313e-16));
            }
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) {
                s.store_ad_value(1456, A::add_scaled_product(A::mul3(s.ad_value(1454), s.ad_value(85), s.ad_value(85)), 1.0, s.ad_value(154), s.ad_value(1482), (-1.0)));
                s.store_ad_value(335, A::add_scaled_product(A::square(s.ad_value(1455)), 1.0, s.ad_value(1454), s.ad_value(1456), (-4.0)));
            }
            s.b[1737] = (s.v[335] > 0.0);
            s.v[1737] = if s.b[1737] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) && s.b[1737]) {
                s.store_div_ad_lhs(1479, A::sub_scaled_inputs(A::sqrt(s.ad_value(335)), 0.5, s.ad_value(1455), 0.5), 1454);
            }
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) && (!s.b[1737])) {
                s.store_scaled_div(1479, 1455, 1454, (-0.5));
            }
            s.b[1738] = (s.v[1479] > s.v[1465]);
            s.v[1738] = if s.b[1738] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) && s.b[1738]) {
                s.copy_ad(1479, 1465);
            }
            s.b[1739] = (s.v[1479] > s.v[1482]);
            s.v[1739] = if s.b[1739] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) && s.b[1739]) {
                s.store_sub(1479, 1482, 1551);
                s.store_scalar(97, (150.0 + 1.0));
            }
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) {
                s.store_sqrt_mul_ad(1448, s.ad_value(1545), A::sub(s.ad_value(1482), s.ad_value(1479)));
                s.store_div_ad(1463, A::sub(A::add_scaled_product(s.ad_value(1433), 1.0, s.ad_value(1534), s.ad_value(1482), 1.0), s.ad_value(1461)), A::offset(s.ad_value(1534), 1.0));
                s.store_sqrt_mul_ad(1446, s.ad_value(1545), A::sub(s.ad_value(1482), s.ad_value(1463)));
            }
            s.b[1740] = ((((s.v[1482] - s.v[1469])) as f64).abs() <= 1e-8);
            s.v[1740] = if s.b[1740] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) && s.b[1740]) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) {
                s.copy_ad(1469, 1482);
                s.store_offset(97, 97, 1.0);
            }
        }

        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && (!s.b[1732])) {
            s.copy_ad(1482, 1481);
            s.copy_ad(1463, 1462);
            s.copy_ad(1479, 1459);
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1709])) {
            s.copy_ad(1480, 1482);
            s.store_scalar(79, 0.0);
            s.copy_ad(1460, 1479);
            s.copy_ad(1482, 1480);
            s.copy_ad(1472, 1460);
            s.copy_ad(1469, 1482);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
    ) {
        let mut assign31970_loop_guard: usize = 0;
        while {
            let assign31970_cond_e33737: f64 = (150.0 + 1.0);
            let assign31970_cond_e33739: f64 = if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (s.v[97] <= assign31970_cond_e33737)) { 1.0 } else { 0.0 };
            assign31970_cond_e33739 != 0.0
        } {
            assign31970_loop_guard += 1;
            assert!(assign31970_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1441] && s.b[1442]) && (!s.b[1709])) {
                s.store_mul_sub_ad_rhs(1463, 1533, A::add_scaled_product(s.ad_value(1433), 1.0, s.ad_value(1534), s.ad_value(1482), 1.0), s.ad_value(1461));
                s.store_mul(1532, 1533, 1534);
                s.store_sub(335, 1482, 1463);
            }
            s.b[1741] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1741] = if s.b[1741] { 1.0 } else { 0.0 };
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1741]) {
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
            s.b[1742] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1742] = if s.b[1742] { 1.0 } else { 0.0 };
            s.b[1743] = (2.0 == 1.0);
            s.v[1743] = if s.b[1743] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1741]) && s.b[1742]) && s.b[1743]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1744] = (2.0 == 2.0);
            s.v[1744] = if s.b[1744] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1741]) && s.b[1742]) && (!s.b[1743])) && s.b[1744]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1745] = (2.0 == 4.0);
            s.v[1745] = if s.b[1745] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1741]) && s.b[1742]) && (!s.b[1743])) && (!s.b[1744])) && s.b[1745]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1746] = (2.0 == 8.0);
            s.v[1746] = if s.b[1746] { 1.0 } else { 0.0 };
            if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1741]) && s.b[1742]) && (!s.b[1743])) && (!s.b[1744])) && (!s.b[1745])) && s.b[1746]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1741]) && s.b[1742]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign31970_body29_loop_guard: usize = 0;
            while {
                let assign31970_body29_cond_e34102: f64 = if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1741]) && s.b[1742]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign31970_body29_cond_e34102 != 0.0
            } {
                assign31970_body29_loop_guard += 1;
                assert!(assign31970_body29_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1741]) && s.b[1742]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1741]) && (!s.b[1742])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1741]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_ad_lhs(334, A::mul_scaled_lhs(s.ad_value(725), 0.1, s.ad_value(726)), 770);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1741]) {
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1741])) {
                s.copy_ad(336, 335);
                s.store_scalar(334, 1.0);
            }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1709])) {
                s.store_sqrt_mul(1446, 1545, 336);
            }
            s.b[1747] = ((s.v[1446] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.v[1747] = if s.b[1747] { 1.0 } else { 0.0 };
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1747]) {
                s.store_offset_sub(781, 1446, 965, 1e-8);
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
            s.b[1748] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1748] = if s.b[1748] { 1.0 } else { 0.0 };
            s.b[1749] = (2.0 == 1.0);
            s.v[1749] = if s.b[1749] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1747]) && s.b[1748]) && s.b[1749]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1750] = (2.0 == 2.0);
            s.v[1750] = if s.b[1750] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1747]) && s.b[1748]) && (!s.b[1749])) && s.b[1750]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1751] = (2.0 == 4.0);
            s.v[1751] = if s.b[1751] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1747]) && s.b[1748]) && (!s.b[1749])) && (!s.b[1750])) && s.b[1751]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1752] = (2.0 == 8.0);
            s.v[1752] = if s.b[1752] { 1.0 } else { 0.0 };
            if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1747]) && s.b[1748]) && (!s.b[1749])) && (!s.b[1750])) && (!s.b[1751])) && s.b[1752]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1747]) && s.b[1748]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign31970_body65_loop_guard: usize = 0;
            while {
                let assign31970_body65_cond_e34587: f64 = if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1747]) && s.b[1748]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign31970_body65_cond_e34587 != 0.0
            } {
                assign31970_body65_loop_guard += 1;
                assert!(assign31970_body65_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1747]) && s.b[1748]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1747]) && (!s.b[1748])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1747]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_ad_lhs(337, A::mul_scaled_lhs(s.ad_value(725), 1e-8, s.ad_value(726)), 770);
                s.store_add_ad_lhs(1446, A::offset(s.ad_value(965), (-1e-8)), 780);
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1747]) {
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1747])) {
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1747])) {
                s.store_scalar(337, 1.0);
            }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1709])) {
                s.store_sqrt_mul_ad(1450, s.ad_value(1548), A::add(A::sub(s.ad_value(1463), s.ad_value(1433)), s.ad_value(1461)));
                s.store_mul(1497, 1446, 1544);
                s.store_mul_ad_product_lhs(1526, A::div_from_scalar(1.034943e-10, s.ad_value(1446)), s.ad_value(334), 337);
                s.store_mul_ad_product_lhs(1528, A::div_from_scalar((-1.034943e-10), s.ad_value(1446)), s.ad_value(334), 337);
                s.store_mul_neg_lhs(1498, 1450, 1542);
                s.store_div_from_scalar(1530, (-1.034943e-10), 1450);
                s.store_scaled_mul(335, 1500, 1541, 8.0);
            }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1709])) {
                let assign31970_body81_ad_e34876: A = A::add(A::add(A::add(A::sub(A::mul3_scaled_output(s.ad_value(1463), s.ad_value(1463), s.ad_value(1540), 4.0), A::mul3_scaled_output(s.ad_value(1463), s.ad_value(1540), s.ad_value(1460), 8.0)), A::mul3_scaled_output(s.ad_value(1540), s.ad_value(1460), s.ad_value(1460), 4.0)), A::mul3_scaled_output(s.ad_value(1463), s.ad_value(1500), s.ad_value(1541), 4.0)), A::mul3_scaled_output(s.ad_value(1460), s.ad_value(1500), s.ad_value(1541), 4.0));
                s.store_div_ad_lhs(1518, A::add_scaled_product(assign31970_body81_ad_e34876, 1.0, A::mul3(s.ad_value(1543), s.ad_value(1539), s.ad_value(1541)), s.ad_value(1541), 1.0), 335);
            }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1709])) {
                s.store_div_ad_lhs(1519, A::add_scaled_product(A::add_scaled_products(s.ad_value(1463), s.ad_value(1540), (-8.0), s.ad_value(1540), s.ad_value(1460), (4.0 * 2.0)), 1.0, s.ad_value(1500), s.ad_value(1541), 4.0), 335);
                s.store_div_ad_lhs(1520, A::add_scaled_product(A::add_scaled_products(s.ad_value(1463), s.ad_value(1540), (4.0 * 2.0), s.ad_value(1540), s.ad_value(1460), (-8.0)), 1.0, s.ad_value(1500), s.ad_value(1541), 4.0), 335);
                s.store_mul_sub_rhs(335, 154, 1460, 1482);
                s.store_exp(336, 335);
            }
            s.b[1753] = (s.v[1460] >= s.v[1482]);
            s.v[1753] = if s.b[1753] { 1.0 } else { 0.0 };
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1753]) {
                s.store_mul_scaled_ad_rhs(1474, 209, -1.0, A::sqrt(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15)));
                s.store_mul_ad(1522, A::div(A::mul_scaled_lhs(s.ad_value(209), 0.5, s.ad_value(209)), s.ad_value(1474)), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
                s.store_neg(1524, 1522);
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1753])) {
                s.store_exp_ad(337, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1460), s.ad_value(1433))));
                s.store_exp_ad(338, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1482), s.ad_value(1433))));
                s.store_mul_sqrt_ad_rhs(1474, 209, A::offset(A::add_scaled_product(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1.0, s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_ad_lhs(339, A::mul_scaled_lhs(s.ad_value(209), 0.5, s.ad_value(209)), 1474);
                s.store_mul_add_ad_rhs(1522, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
                s.store_mul_add_ad_rhs(1524, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));
            }
            s.b[1754] = ((s.v[1518] > (s.v[1510] - s.v[1517])) && (s.v[1517] >= 0.0));
            s.v[1754] = if s.b[1754] { 1.0 } else { 0.0 };
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1754]) {
                s.store_add_ad_lhs(781, A::sub(s.ad_value(1518), s.ad_value(1510)), 1517);
                s.store_square(722, 781);
                s.store_square(723, 1517);
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
            s.b[1755] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.v[1755] = if s.b[1755] { 1.0 } else { 0.0 };
            s.b[1756] = (4.0 == 1.0);
            s.v[1756] = if s.b[1756] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1754]) && s.b[1755]) && s.b[1756]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1757] = (4.0 == 2.0);
            s.v[1757] = if s.b[1757] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1754]) && s.b[1755]) && (!s.b[1756])) && s.b[1757]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1758] = (4.0 == 4.0);
            s.v[1758] = if s.b[1758] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1754]) && s.b[1755]) && (!s.b[1756])) && (!s.b[1757])) && s.b[1758]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1759] = (4.0 == 8.0);
            s.v[1759] = if s.b[1759] { 1.0 } else { 0.0 };
            if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1754]) && s.b[1755]) && (!s.b[1756])) && (!s.b[1757])) && (!s.b[1758])) && s.b[1759]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1754]) && s.b[1755]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign31970_body126_loop_guard: usize = 0;
            while {
                let assign31970_body126_cond_e35536: f64 = if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1754]) && s.b[1755]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign31970_body126_cond_e35536 != 0.0
            } {
                assign31970_body126_loop_guard += 1;
                assert!(assign31970_body126_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1754]) && s.b[1755]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1754]) && (!s.b[1755])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                    }
                });
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1754]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_mul3_lhs(780, 781, 1517, 726);
                s.store_div_ad_lhs(334, A::mul3(s.ad_value(1517), s.ad_value(725), s.ad_value(726)), 770);
                s.store_add_ad_lhs(335, A::sub(s.ad_value(1510), s.ad_value(1517)), 780);
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1754]) {
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1754])) {
                s.copy_ad(335, 1518);
                s.store_scalar(334, 1.0);
            }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1709])) {
                s.store_sub(1483, 1482, 335);
                s.store_mul_neg_lhs(1485, 1519, 334);
                s.store_sub_from_scalar_ad(1486, 1.0, A::mul3(s.ad_value(1520), s.ad_value(1532), s.ad_value(334)));
                s.store_add_ad_lhs(1484, A::add(A::add_scaled_product(s.ad_value(1474), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1460)), 1.0), s.ad_value(1497)), 1498);
                s.store_sub(1487, 1522, 185);
                s.store_ad_value(1488, A::add_scaled_product(A::add_scaled_product(A::add(s.ad_value(1524), s.ad_value(1526)), 1.0, s.ad_value(1528), s.ad_value(1532), 1.0), 1.0, s.ad_value(1530), s.ad_value(1532), 1.0));
                s.store_ad_value(1489, A::add_scaled_products(s.ad_value(1485), s.ad_value(1488), 1.0, s.ad_value(1487), s.ad_value(1486), (-1.0)));
                s.store_div(1490, 1488, 1489);
                s.store_div_ad_lhs(1491, A::neg(s.ad_value(1486)), 1489);
                s.store_div_ad_lhs(1492, A::neg(s.ad_value(1487)), 1489);
                s.store_div(1493, 1485, 1489);
            }
            s.b[1760] = (((((s.v[1492] * s.v[1483]) + (s.v[1493] * s.v[1484]))) as f64).abs() > 0.5);
            s.v[1760] = if s.b[1760] { 1.0 } else { 0.0 };
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1760]) {
                s.store_offset(1460, 1460, (-(0.5 * (if (((s.v[1490] * s.v[1483]) + (s.v[1491] * s.v[1484])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1760]) {
                s.store_offset(1482, 1482, (-(0.5 * (if (((s.v[1492] * s.v[1483]) + (s.v[1493] * s.v[1484])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1760])) {
                s.store_sub_ad_rhs(1460, 1460, A::add_scaled_products(s.ad_value(1490), s.ad_value(1483), 1.0, s.ad_value(1491), s.ad_value(1484), 1.0));
                s.store_sub_ad_rhs(1482, 1482, A::add_scaled_products(s.ad_value(1492), s.ad_value(1483), 1.0, s.ad_value(1493), s.ad_value(1484), 1.0));
            }
            s.b[1761] = (((((s.v[1460] - s.v[1472])) as f64).abs() <= 1e-12) && ((((s.v[1482] - s.v[1469])) as f64).abs() <= 1e-12));
            s.v[1761] = if s.b[1761] { 1.0 } else { 0.0 };
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1761]) {
                s.store_scalar(97, (150.0 + 1.0));
                s.store_scalar(79, 1.0);
            }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1709])) {
                s.copy_ad(1472, 1460);
                s.copy_ad(1469, 1482);
                s.store_offset(97, 97, 1.0);
            }
        }

        s.b[1763] = ((s.v[1452] > s.v[965]) && (s.v[1477] != 2.0));
        s.v[1763] = if s.b[1763] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
    ) {
        s.b[1764] = ((s.v[1482] > (s.v[1460] - 0.02)) && (0.02 >= 0.0));
        s.v[1764] = if s.b[1764] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) {
            s.store_offset_sub(781, 1482, 1460, 0.02);
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

        s.b[1765] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1765] = if s.b[1765] { 1.0 } else { 0.0 };

        s.b[1766] = (2.0 == 1.0);
        s.v[1766] = if s.b[1766] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) && s.b[1765]) && s.b[1766]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1767] = (2.0 == 2.0);
        s.v[1767] = if s.b[1767] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) && s.b[1765]) && (!s.b[1766])) && s.b[1767]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1768] = (2.0 == 4.0);
        s.v[1768] = if s.b[1768] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) && s.b[1765]) && (!s.b[1766])) && (!s.b[1767])) && s.b[1768]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1769] = (2.0 == 8.0);
        s.v[1769] = if s.b[1769] { 1.0 } else { 0.0 };

        if (((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) && s.b[1765]) && (!s.b[1766])) && (!s.b[1767])) && (!s.b[1768])) && s.b[1769]) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) && s.b[1765]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign32260_loop_guard: usize = 0;
        while {
            let assign32260_cond_e36380: f64 = if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) && s.b[1765]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign32260_cond_e36380 != 0.0
        } {
            assign32260_loop_guard += 1;
            assert!(assign32260_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) && s.b[1765]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) && (!s.b[1765])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.02);
            s.store_div_ad_lhs(335, A::mul_scaled_lhs(s.ad_value(725), 0.02, s.ad_value(726)), 770);
            s.store_add_ad_lhs(1482, A::offset(s.ad_value(1460), (-0.02)), 780);
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) {
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && (!s.b[1764])) {
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && (!s.b[1764])) {
            s.store_scalar(335, 1.0);
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1709])) {
            s.store_mul_sub_ad_rhs(1463, 1533, A::add_scaled_product(s.ad_value(1433), 1.0, s.ad_value(1534), s.ad_value(1482), 1.0), s.ad_value(1461));
            s.store_mul_sub_rhs(335, 154, 1460, 1482);
            s.store_exp(336, 335);
        }

        s.b[1770] = (s.v[1460] >= s.v[1482]);
        s.v[1770] = if s.b[1770] { 1.0 } else { 0.0 };

        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) {
            s.store_mul_scaled_ad_rhs(1474, 209, -1.0, A::sqrt(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15)));
            s.copy_ad(1537, 1474);
            s.store_scalar(1516, 0.0);
            s.store_scalar(1476, 0.0);
            s.store_sqrt_mul_ad(1446, s.ad_value(1545), A::sub(s.ad_value(1482), s.ad_value(1463)));
        }

        s.b[1771] = ((s.v[1446] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
        s.v[1771] = if s.b[1771] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) {
            s.store_offset_sub(781, 1446, 965, 1e-8);
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

        s.b[1772] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1772] = if s.b[1772] { 1.0 } else { 0.0 };

        s.b[1773] = (2.0 == 1.0);
        s.v[1773] = if s.b[1773] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) && s.b[1772]) && s.b[1773]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1774] = (2.0 == 2.0);
        s.v[1774] = if s.b[1774] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) && s.b[1772]) && (!s.b[1773])) && s.b[1774]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1775] = (2.0 == 4.0);
        s.v[1775] = if s.b[1775] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) && s.b[1772]) && (!s.b[1773])) && (!s.b[1774])) && s.b[1775]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1776] = (2.0 == 8.0);
        s.v[1776] = if s.b[1776] { 1.0 } else { 0.0 };

        if (((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) && s.b[1772]) && (!s.b[1773])) && (!s.b[1774])) && (!s.b[1775])) && s.b[1776]) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) && s.b[1772]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign32700_loop_guard: usize = 0;
        while {
            let assign32700_cond_e37028: f64 = if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) && s.b[1772]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign32700_cond_e37028 != 0.0
        } {
            assign32700_loop_guard += 1;
            assert!(assign32700_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) && s.b[1772]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) && (!s.b[1772])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_ad_lhs(337, A::mul_scaled_lhs(s.ad_value(725), 1e-8, s.ad_value(726)), 770);
            s.store_add_ad_lhs(1446, A::offset(s.ad_value(965), (-1e-8)), 780);
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) {
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && (!s.b[1771])) {
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && (!s.b[1771])) {
            s.store_scalar(337, 1.0);
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) {
            s.store_sqrt_mul_ad(1450, s.ad_value(1548), A::add(A::sub(s.ad_value(1463), s.ad_value(1433)), s.ad_value(1461)));
            s.store_mul(1497, 1446, 1544);
            s.store_mul_neg_lhs(1498, 1450, 1542);
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) {
            s.store_exp_ad(337, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1460), s.ad_value(1433))));
            s.store_exp_ad(338, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1482), s.ad_value(1433))));
            s.store_mul_sqrt_ad_rhs(1474, 209, A::offset(A::add_scaled_product(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1.0, s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
        }

        s.b[1777] = ((s.v[1452] > s.v[965]) && (s.v[1477] != 2.0));
        s.v[1777] = if s.b[1777] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1777]) {
            s.store_scalar(1476, 0.0);
            s.store_scalar(1516, 0.0);
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && (!s.b[1777])) {
            s.store_mul_sqrt_ad_rhs(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1460), s.ad_value(1433)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1482), s.ad_value(1433)))))), s.ad_value(335)));
            s.store_ad_value(1476, A::add_scaled_product(s.ad_value(337), 1.0, s.ad_value(209), A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0)));
            s.store_mul_sqrt_ad_rhs(1516, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) {
            s.store_scalar(1537, 0.0);
            s.store_sub(335, 1482, 1463);
        }

        s.b[1778] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
        s.v[1778] = if s.b[1778] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) {
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

        s.b[1779] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1779] = if s.b[1779] { 1.0 } else { 0.0 };

        s.b[1780] = (2.0 == 1.0);
        s.v[1780] = if s.b[1780] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) && s.b[1779]) && s.b[1780]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1781] = (2.0 == 2.0);
        s.v[1781] = if s.b[1781] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) && s.b[1779]) && (!s.b[1780])) && s.b[1781]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1782] = (2.0 == 4.0);
        s.v[1782] = if s.b[1782] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) && s.b[1779]) && (!s.b[1780])) && (!s.b[1781])) && s.b[1782]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1783] = (2.0 == 8.0);
        s.v[1783] = if s.b[1783] { 1.0 } else { 0.0 };

        if (((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) && s.b[1779]) && (!s.b[1780])) && (!s.b[1781])) && (!s.b[1782])) && s.b[1783]) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) && s.b[1779]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign33190_loop_guard: usize = 0;
        while {
            let assign33190_cond_e37835: f64 = if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) && s.b[1779]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign33190_cond_e37835 != 0.0
        } {
            assign33190_loop_guard += 1;
            assert!(assign33190_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) && s.b[1779]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) && (!s.b[1779])) {
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
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_ad_lhs(334, A::mul_scaled_lhs(s.ad_value(725), 0.1, s.ad_value(726)), 770);
            s.store_sub_from_scalar(336, 0.1, 780);
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) {
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && (!s.b[1778])) {
            s.copy_ad(336, 335);
            s.store_scalar(334, 1.0);
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) {
            s.store_sqrt_mul(1446, 1545, 336);
        }

        s.b[1784] = ((s.v[1446] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
        s.v[1784] = if s.b[1784] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) {
            s.store_offset_sub(781, 1446, 965, 1e-8);
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

        s.b[1785] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1785] = if s.b[1785] { 1.0 } else { 0.0 };

        s.b[1786] = (2.0 == 1.0);
        s.v[1786] = if s.b[1786] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) && s.b[1785]) && s.b[1786]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1787] = (2.0 == 2.0);
        s.v[1787] = if s.b[1787] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) && s.b[1785]) && (!s.b[1786])) && s.b[1787]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1788] = (2.0 == 4.0);
        s.v[1788] = if s.b[1788] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) && s.b[1785]) && (!s.b[1786])) && (!s.b[1787])) && s.b[1788]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1789] = (2.0 == 8.0);
        s.v[1789] = if s.b[1789] { 1.0 } else { 0.0 };

        if (((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) && s.b[1785]) && (!s.b[1786])) && (!s.b[1787])) && (!s.b[1788])) && s.b[1789]) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) && s.b[1785]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign33550_loop_guard: usize = 0;
        while {
            let assign33550_cond_e38416: f64 = if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) && s.b[1785]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign33550_cond_e38416 != 0.0
        } {
            assign33550_loop_guard += 1;
            assert!(assign33550_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) && s.b[1785]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) && (!s.b[1785])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_ad_lhs(337, A::mul_scaled_lhs(s.ad_value(725), 1e-8, s.ad_value(726)), 770);
            s.store_add_ad_lhs(1446, A::offset(s.ad_value(965), (-1e-8)), 780);
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) {
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && (!s.b[1784])) {
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && (!s.b[1784])) {
            s.store_scalar(337, 1.0);
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) {
            s.store_sqrt_mul_ad(1450, s.ad_value(1548), A::add(A::sub(s.ad_value(1463), s.ad_value(1433)), s.ad_value(1461)));
            s.store_mul(1497, 1446, 1544);
            s.store_mul_neg_lhs(1498, 1450, 1542);
        }

        s.b[1790] = (((s.v[1460] - s.v[1510]) < 0.06) && (0.06 >= 0.0));
        s.v[1790] = if s.b[1790] { 1.0 } else { 0.0 };

        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) {
            s.store_sub_from_scalar_ad(781, 0.06, A::sub(s.ad_value(1460), s.ad_value(1510)));
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

        s.b[1791] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1791] = if s.b[1791] { 1.0 } else { 0.0 };

        s.b[1792] = (2.0 == 1.0);
        s.v[1792] = if s.b[1792] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) && s.b[1791]) && s.b[1792]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1793] = (2.0 == 2.0);
        s.v[1793] = if s.b[1793] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) && s.b[1791]) && (!s.b[1792])) && s.b[1793]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1794] = (2.0 == 4.0);
        s.v[1794] = if s.b[1794] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) && s.b[1791]) && (!s.b[1792])) && (!s.b[1793])) && s.b[1794]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1795] = (2.0 == 8.0);
        s.v[1795] = if s.b[1795] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) && s.b[1791]) && (!s.b[1792])) && (!s.b[1793])) && (!s.b[1794])) && s.b[1795]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) && s.b[1791]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign33930_loop_guard: usize = 0;
        while {
            let assign33930_cond_e38971: f64 = if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) && s.b[1791]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign33930_cond_e38971 != 0.0
        } {
            assign33930_loop_guard += 1;
            assert!(assign33930_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) && s.b[1791]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) && (!s.b[1791])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.06);
            s.store_div_ad_lhs(334, A::mul_scaled_lhs(s.ad_value(725), 0.06, s.ad_value(726)), 770);
            s.store_sub_from_scalar(336, 0.06, 780);
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) {
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1790])) {
            s.store_sub(336, 1460, 1510);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1709])) {
            s.store_offset_ad(338, A::add_scaled_product(A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0)), (10.0 * 2.220446049250313e-16));
            s.store_mul_scaled_ad_rhs(1514, 209, -1.0, A::sqrt(s.ad_value(338)));
        }

        if (s.b[1441] && s.b[1442]) {
            s.copy_ad(87, 1459);
            s.copy_ad(91, 1460);
            s.store_sub(94, 1460, 1459);
            s.store_neg_ad(335, A::add(s.ad_value(1473), s.ad_value(1474)));
        }

        s.b[1796] = ((s.v[335] < s.v[1538]) && (s.v[1538] >= 0.0));
        s.v[1796] = if s.b[1796] { 1.0 } else { 0.0 };

        if ((s.b[1441] && s.b[1442]) && s.b[1796]) {
            s.store_sub(781, 1538, 335);
            s.store_square(722, 781);
            s.store_square(723, 1538);
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

        s.b[1797] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1797] = if s.b[1797] { 1.0 } else { 0.0 };

        s.b[1798] = (2.0 == 1.0);
        s.v[1798] = if s.b[1798] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && s.b[1442]) && s.b[1796]) && s.b[1797]) && s.b[1798]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1799] = (2.0 == 2.0);
        s.v[1799] = if s.b[1799] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && s.b[1442]) && s.b[1796]) && s.b[1797]) && (!s.b[1798])) && s.b[1799]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1800] = (2.0 == 4.0);
        s.v[1800] = if s.b[1800] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && s.b[1442]) && s.b[1796]) && s.b[1797]) && (!s.b[1798])) && (!s.b[1799])) && s.b[1800]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1801] = (2.0 == 8.0);
        s.v[1801] = if s.b[1801] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && s.b[1442]) && s.b[1796]) && s.b[1797]) && (!s.b[1798])) && (!s.b[1799])) && (!s.b[1800])) && s.b[1801]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1796]) && s.b[1797]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign34340_loop_guard: usize = 0;
        while {
            let assign34340_cond_e39447: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1796]) && s.b[1797]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign34340_cond_e39447 != 0.0
        } {
            assign34340_loop_guard += 1;
            assert!(assign34340_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && s.b[1442]) && s.b[1796]) && s.b[1797]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1796]) && (!s.b[1797])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1796]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1538, 726);
            s.store_div_ad_lhs(334, A::mul3(s.ad_value(1538), s.ad_value(725), s.ad_value(726)), 770);
            s.store_sub(1554, 1538, 780);
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1796]) {
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1796])) {
            s.copy_ad(1554, 335);
        }

    }

    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1441] && s.b[1442]) && (!s.b[1796])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_mul3_affine_lhs(1501, 154, 1554, 1.0 / (2.0), 0.0, 94);
            s.store_neg_ad(1502, A::sub(s.ad_value(1513), s.ad_value(1514)));
            s.store_add(248, 1501, 1502);
            s.store_neg(133, 1513);
            s.copy_ad(170, 162);
            s.store_scalar(336, (s.v[626] / 100.0));
            s.store_offset_mul_ad(338, A::sub(s.ad_value(1460), s.ad_value(1459)), s.ad_value(682), 1.0);
            s.store_mul(339, 336, 133);
            s.store_div(337, 339, 338);
            s.copy_ad(251, 337);
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_ad_value(339, {
                if (s.v[251] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(251), (p.p160 - 1.0))
                }
            });
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_mul(342, 339, 251);
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_ad_value(341, {
                if (s.v[251] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(251), A::offset(s.ad_value(624), (-1.0)))
                }
            });
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_mul(340, 341, 251);
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 133, 343);
            s.store_add_scaled_ad_lhs(335, A::add_scaled_product(A::div_from_scalar(1.0, A::scale_offset(s.ad_value(252), (s.v[475] * 1e-11), ((s.v[474]) + (1e-25)))), 1.0, s.ad_value(679), s.ad_value(342), 1.0), 340, 1.0 / (s.v[479]));
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_mul_ad_product_lhs(336, s.ad_value(154), A::offset(s.ad_value(133), 1e-25), 170);
            s.store_div_from_scalar(335, 1.0, 336);
            s.store_mul(333, 248, 335);
            s.store_scaled_div(336, 257, 254, 0.2);
            s.store_sqrt_square_sum(255, 333, 336);
            s.store_div_from_scalar(338, 1.0, 255);
            s.store_mul(256, 254, 255);
            s.store_div(335, 256, 257);
            s.copy_ad(1556, 255);
        }

        s.b[1802] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1802] = if s.b[1802] { 1.0 } else { 0.0 };

        if ((s.b[1441] && s.b[1442]) && s.b[1802]) {
            s.store_scalar(337, 1.0);
        }

        s.b[1803] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1803] = if s.b[1803] { 1.0 } else { 0.0 };

        if (((s.b[1441] && s.b[1442]) && (!s.b[1802])) && s.b[1803]) {
            s.copy_ad(337, 335);
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1802])) && (!s.b[1803])) {
            s.store_ad_value(337, {
                if (s.v[335] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(335), (p.p178 - 1.0))
                }
            });
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_mul(336, 335, 337);
            s.store_offset(338, 336, 1.0);
        }

        s.b[1804] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1804] = if s.b[1804] { 1.0 } else { 0.0 };

        if ((s.b[1441] && s.b[1442]) && s.b[1804]) {
            s.store_div_from_scalar(339, 1.0, 338);
        }

        s.b[1805] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1805] = if s.b[1805] { 1.0 } else { 0.0 };

        if (((s.b[1441] && s.b[1442]) && (!s.b[1804])) && s.b[1805]) {
            s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1804])) && (!s.b[1805])) {
            s.store_ad_value(340, {
                if (s.v[338] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(338), (((-1.0) / p.p178) - 1.0))
                }
            });
        }

        if (((s.b[1441] && s.b[1442]) && (!s.b[1804])) && (!s.b[1805])) {
            s.store_mul(339, 338, 340);
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_mul(253, 254, 339);
        }

        s.b[1806] = (s.v[349] > 1e-6);
        s.v[1806] = if s.b[1806] { 1.0 } else { 0.0 };

        if ((s.b[1441] && s.b[1442]) && s.b[1806]) {
            s.store_div_ad_rhs(336, 1500, A::square(s.ad_value(185)));
            s.store_sub_ad_lhs(334, A::sub(A::add(s.ad_value(85), s.ad_value(974)), s.ad_value(155)), 1436);
            s.store_offset_mul_ad(338, A::div_from_scalar(2.0, s.ad_value(336)), s.ad_value(334), 1.0);
        }

        s.b[1807] = ((s.v[338] < 2.0) && (2.0 >= 0.0));
        s.v[1807] = if s.b[1807] { 1.0 } else { 0.0 };

        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) {
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

        s.b[1808] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1808] = if s.b[1808] { 1.0 } else { 0.0 };

        s.b[1809] = (2.0 == 1.0);
        s.v[1809] = if s.b[1809] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) && s.b[1808]) && s.b[1809]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1810] = (2.0 == 2.0);
        s.v[1810] = if s.b[1810] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) && s.b[1808]) && (!s.b[1809])) && s.b[1810]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1811] = (2.0 == 4.0);
        s.v[1811] = if s.b[1811] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) && s.b[1808]) && (!s.b[1809])) && (!s.b[1810])) && s.b[1811]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1812] = (2.0 == 8.0);
        s.v[1812] = if s.b[1812] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) && s.b[1808]) && (!s.b[1809])) && (!s.b[1810])) && (!s.b[1811])) && s.b[1812]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) && s.b[1808]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign35150_loop_guard: usize = 0;
        while {
            let assign35150_cond_e40366: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) && s.b[1808]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign35150_cond_e40366 != 0.0
        } {
            assign35150_loop_guard += 1;
            assert!(assign35150_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) && s.b[1808]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) && (!s.b[1808])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 2.0);
            s.store_div_ad_lhs(334, A::mul_scaled_lhs(s.ad_value(725), 2.0, s.ad_value(726)), 770);
            s.store_sub_from_scalar(343, 2.0, 780);
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) {
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && (!s.b[1807])) {
            s.copy_ad(343, 338);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1806]) {
            s.store_offset(343, 343, 1e-25);
            s.store_sqrt(337, 343);
            s.store_mul_sub_from_scalar_rhs(338, 336, 1.0, 337);
            s.store_add_ad_lhs(344, A::add(s.ad_value(85), s.ad_value(974)), 338);
            s.store_mul(344, 344, 975);
        }

        s.b[1813] = ((s.v[344] < (s.v[972] + 4.0)) && (4.0 >= 0.0));
        s.v[1813] = if s.b[1813] { 1.0 } else { 0.0 };

        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) {
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

        s.b[1814] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1814] = if s.b[1814] { 1.0 } else { 0.0 };

        s.b[1815] = (4.0 == 1.0);
        s.v[1815] = if s.b[1815] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) && s.b[1814]) && s.b[1815]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1816] = (4.0 == 2.0);
        s.v[1816] = if s.b[1816] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) && s.b[1814]) && (!s.b[1815])) && s.b[1816]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1817] = (4.0 == 4.0);
        s.v[1817] = if s.b[1817] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) && s.b[1814]) && (!s.b[1815])) && (!s.b[1816])) && s.b[1817]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1818] = (4.0 == 8.0);
        s.v[1818] = if s.b[1818] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) && s.b[1814]) && (!s.b[1815])) && (!s.b[1816])) && (!s.b[1817])) && s.b[1818]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) && s.b[1814]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign35590_loop_guard: usize = 0;
        while {
            let assign35590_cond_e40909: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) && s.b[1814]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign35590_cond_e40909 != 0.0
        } {
            assign35590_loop_guard += 1;
            assert!(assign35590_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) && s.b[1814]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

    }

    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) && (!s.b[1814])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                }
            });
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 4.0);
            s.store_div_ad_lhs(334, A::mul_scaled_lhs(s.ad_value(725), 4.0, s.ad_value(726)), 770);
            s.store_sub_ad_lhs(344, A::offset(s.ad_value(972), 4.0), 780);
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) {
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && (!s.b[1813])) {
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && (!s.b[1813])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1806]) {
            s.store_div(335, 349, 344);
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1806]) {
            s.store_ad_value(336, {
                if (s.v[335] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(335), A::offset(s.ad_value(658), (-1.0)))
                }
            });
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1806]) {
            s.store_mul(341, 336, 335);
            s.store_offset(337, 341, 1.0);
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1806]) {
            s.store_ad_value(338, {
                if (s.v[337] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)))
                }
            });
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1806]) {
            s.store_mul(340, 338, 337);
            s.store_div(1555, 349, 340);
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1806])) {
            s.copy_ad(1555, 349);
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_offset_mul_ad(338, A::sub(s.ad_value(1460), s.ad_value(1459)), s.ad_value(682), 1.0);
            s.store_neg(133, 1494);
            s.copy_ad(339, 133);
            s.store_div(337, 339, 338);
            s.copy_ad(251, 337);
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_ad_value(339, {
                if (s.v[251] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(251), (p.p376 - 1.0))
                }
            });
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_mul(342, 339, 251);
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 133, 343);
            s.store_ad_value(335, A::add_scaled_product(A::div_from_scalar(1.0, A::offset(A::add_scaled_product(s.ad_value(966), 1.0, s.ad_value(968), s.ad_value(252), 1e-11), 1e-25)), 1.0, s.ad_value(977), s.ad_value(342), 1.0));
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_div(1553, 1555, 170);
            s.store_div_ad_lhs(335, A::mul(s.ad_value(254), s.ad_value(1553)), 973);
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_ad_value(336, {
                if (s.v[335] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(335), p.p378)
                }
            });
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_offset(337, 336, 1.0);
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_ad_value(338, {
                if (s.v[337] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(337), (1.0 / p.p378))
                }
            });
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_div(1504, 254, 338);
            s.store_mul3_affine_lhs(987, 1494, 1504, (-s.v[632]), 0.0, 1553);
            s.store_offset_mul_ad(338, A::sub(s.ad_value(1460), s.ad_value(1459)), s.ad_value(682), 1.0);
            s.store_neg(133, 1503);
            s.copy_ad(339, 133);
            s.store_div(337, 339, 338);
            s.copy_ad(251, 337);
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_ad_value(339, {
                if (s.v[251] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(251), (p.p376 - 1.0))
                }
            });
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_mul(342, 339, 251);
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 133, 343);
            s.store_ad_value(335, A::add_scaled_product(A::div_from_scalar(1.0, A::offset(A::add_scaled_product(s.ad_value(967), 1.0, s.ad_value(969), s.ad_value(252), 1e-11), 1e-25)), 1.0, s.ad_value(977), s.ad_value(342), 1.0));
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_div(1553, 1555, 170);
            s.store_div_ad_lhs(335, A::mul(s.ad_value(254), s.ad_value(1553)), 973);
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_ad_value(336, {
                if (s.v[335] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(335), p.p378)
                }
            });
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_offset(337, 336, 1.0);
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_ad_value(338, {
                if (s.v[337] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(337), (1.0 / p.p378))
                }
            });
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_div(1505, 254, 338);
            s.store_mul3_affine_lhs(1552, 1503, 1505, (-s.v[632]), 0.0, 1553);
            s.store_scaled_div(115, 155, 170, s.v[632]);
            s.store_add_ad_lhs(135, A::add(A::mul3(s.ad_value(115), s.ad_value(248), s.ad_value(253)), s.ad_value(987)), 1552);
            s.store_mul3_lhs(986, 115, 248, 253);
            s.copy_ad(984, 253);
            s.copy_ad(790, 349);
        }

        s.b[1819] = (p.p283 != 0.0);
        s.v[1819] = if s.b[1819] { 1.0 } else { 0.0 };

        if ((s.b[1441] && s.b[1442]) && s.b[1819]) {
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 100.0));
            s.store_offset_mul_ad(782, s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0))), (1.0 / 24.0))), (1.0 / 6.0))), (1.0 / 2.0)), 1.0);
            s.store_offset_mul_ad(783, s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0))), (1.0 / 8.0))), (1.0 / 3.0)), (1.0 / 2.0));
            s.store_div_from_scalar(340, 0.01, 782);
            s.store_div_ad(336, A::scale(s.ad_value(783), (-2.0)), A::square(s.ad_value(782)));
            s.store_sub_from_scalar_ad(335, 1.1, A::add(s.ad_value(1459), s.ad_value(340)));
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.05) * 0.05));
            s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);
            s.store_scaled_add(336, 335, 782, 0.5);
        }

        s.b[1820] = (s.v[336] < 0.0);
        s.v[1820] = if s.b[1820] { 1.0 } else { 0.0 };

        if (((s.b[1441] && s.b[1442]) && s.b[1819]) && s.b[1820]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1819]) {
            s.store_offset(336, 336, 1e-25);
            s.store_scale(334, 154, s.v[672]);
            s.store_mul(337, 185, 334);
            s.store_powf(334, 336, p.p284);
            s.store_mul(343, 337, 334);
            s.store_offset_scaled(338, 1437, p.p285, 1.0);
            s.store_scalar(334, s.v[673]);
            s.store_sub_ad_lhs(339, A::add(s.ad_value(1459), s.ad_value(340)), 1436);
            s.store_add_ad_rhs(338, 338, A::mul3(s.ad_value(1437), s.ad_value(334), s.ad_value(339)));
            s.store_mul(340, 343, 338);
            s.copy_ad(343, 340);
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1819])) {
            s.store_scalar(343, 0.0);
        }

        s.b[1821] = (p.p287 != 0.0);
        s.v[1821] = if s.b[1821] { 1.0 } else { 0.0 };

        if ((s.b[1441] && s.b[1442]) && s.b[1821]) {
            s.store_scale(335, 154, s.v[674]);
            s.store_mul(336, 185, 335);
            s.store_mul(342, 336, 1437);
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1821])) {
            s.store_scalar(342, 0.0);
        }

        s.b[1822] = ((s.v[343] + s.v[342]) > 0.0);
        s.v[1822] = if s.b[1822] { 1.0 } else { 0.0 };

        if ((s.b[1441] && s.b[1442]) && s.b[1822]) {
            s.store_mul_add_rhs(249, 94, 343, 342);
            s.store_add_ad_rhs(135, 135, A::mul3(s.ad_value(115), s.ad_value(249), s.ad_value(253)));
        }

        s.b[1823] = ((s.v[74] == 2.0) || (s.v[74] == 3.0));
        s.v[1823] = if s.b[1823] { 1.0 } else { 0.0 };

        s.b[1824] = (p.p296 > 0.0);
        s.v[1824] = if s.b[1824] { 1.0 } else { 0.0 };

        if (((s.b[1441] && s.b[1442]) && s.b[1823]) && s.b[1824]) {
            s.copy_ad(338, 647);
            s.store_scaled_offset(335, 796, (-p.p300), s.v[533]);
            s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));
            s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1823]) && s.b[1824]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1823]) && s.b[1824]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_ad_rhs(336, 338, A::add_scaled_inputs(s.ad_value(781), 0.5, s.ad_value(782), 0.5));
            s.store_scale(337, 338, (p.p296 + 1.0));
            s.store_offset_sub(781, 337, 336, (-(0.01 * 0.01)));
            s.store_scale(782, 337, (4.0 * (0.01 * 0.01)));
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1823]) && s.b[1824]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1823]) && s.b[1824]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_sub_ad_rhs(341, 337, A::add_scaled_inputs(s.ad_value(781), 0.5, s.ad_value(782), 0.5));
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1823]) && (!s.b[1824])) {
            s.copy_ad(341, 647);
        }

        s.b[1825] = (s.v[793] >= 0.0);
        s.v[1825] = if s.b[1825] { 1.0 } else { 0.0 };

        if (((s.b[1441] && s.b[1442]) && s.b[1823]) && s.b[1825]) {
            s.copy_ad(369, 793);
        }

    }

    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1441] && s.b[1442]) && s.b[1823]) && (!s.b[1825])) {
            s.store_scalar(369, 0.0);
        }

        s.b[1826] = (s.v[369] < (20.0 * 1e-12));
        s.v[1826] = if s.b[1826] { 1.0 } else { 0.0 };

        if (((s.b[1441] && s.b[1442]) && s.b[1823]) && s.b[1826]) {
            s.store_scalar(378, (((((20.0 + 1.0)) as f64).powf((p.p297 - 1.0)) * ((20.0 + 1.0) - ((0.5 * p.p297) * 20.0))) * ((1e-12) as f64).powf(p.p297)));
            s.store_scalar(379, ((((0.5 * p.p297) * (((20.0 + 1.0)) as f64).powf((p.p297 - 1.0))) / 20.0) * ((1e-12) as f64).powf((p.p297 - 2.0))));
            s.store_add_ad_rhs(335, 378, A::mul3(s.ad_value(379), s.ad_value(369), s.ad_value(369)));
        }

        if (((s.b[1441] && s.b[1442]) && s.b[1823]) && (!s.b[1826])) {
            s.store_powf_ad(335, A::offset(s.ad_value(369), 1e-12), p.p297);
        }

        if ((s.b[1441] && s.b[1442]) && s.b[1823]) {
            s.store_powf_ad(343, A::offset(s.ad_value(369), 1e-12), p.p299);
            s.store_ad_value(368, A::add_scaled_products(s.ad_value(341), s.ad_value(335), 1.0 / (s.v[632]), s.ad_value(797), s.ad_value(343), (s.v[531] * 1.0 / (s.v[632]))));
            s.store_mul(334, 368, 135);
            s.store_offset(335, 790, 1e-12);
            s.store_div_from_scalar(336, 1.0, 335);
            s.store_offset_mul(337, 334, 336, 1.0);
            s.store_div_from_scalar(338, 1.0, 337);
            s.store_mul(134, 135, 338);
        }

        if ((s.b[1441] && s.b[1442]) && (!s.b[1823])) {
            s.copy_ad(134, 135);
            s.store_scalar(368, 0.0);
        }

        if (s.b[1441] && s.b[1442]) {
            s.store_scaled_add_ad_lhs(131, A::add(A::add(s.ad_value(1475), s.ad_value(1476)), s.ad_value(1496)), 1498, (-0.5));
            s.store_scaled_add_ad_lhs(133, A::add(A::add(A::add(A::add(s.ad_value(1536), s.ad_value(1537)), s.ad_value(1515)), s.ad_value(1516)), s.ad_value(1495)), 1497, (-0.5));
            s.store_scalar(247, 0.5);
            s.store_scaled_add(978, 1536, 1537, (-0.5));
            s.store_neg(238, 1536);
            s.copy_ad(255, 1556);
        }

        s.b[1827] = ((s.v[238] < 1e-25) || (s.v[133] < 1e-25));
        s.v[1827] = if s.b[1827] { 1.0 } else { 0.0 };

        if ((s.b[1441] && s.b[1442]) && s.b[1827]) {
            s.store_scalar(78, 1.0);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.copy_ad(1853, 960);
            s.store_scale(1903, 964, 1.6021918e-19);
            s.store_scale(1882, 964, (1.6021918e-19 * 1.034943e-10));
            s.store_scale(1902, 622, 1.6021918e-19);
            s.store_square(1901, 965);
            s.store_div_from_scalar(1906, (2.0 * 1.034943e-10), 1903);
            s.store_div_from_scalar(1907, (2.0 * 1.034943e-10), 1902);
            s.store_div(1900, 964, 622);
            s.store_div_from_scalar_offset_input(1899, 1.0, 1900, 1.0);
            s.store_div_ad_rhs(1904, 1882, A::square(s.ad_value(185)));
            s.store_div_from_scalar(1905, 2.0, 1904);
            s.store_scalar(1908, 4.0);
            s.store_scalar(1909, 0.1);
            s.store_scalar(1910, 0.1);
            s.store_offset(1911, 961, p.p407);
            s.store_scalar(1912, 3.0);
            s.store_scalar(1851, 0.0);
            s.store_scalar(1852, 0.0);
            s.store_scalar(1860, 0.0);
            s.store_scalar(1861, 0.0);
            s.store_scalar(1893, 0.0);
            s.store_scalar(1894, 0.0);
            s.store_scalar(1864, 0.0);
            s.store_scalar(1866, 0.0);
            s.store_scalar(1865, 0.0);
            s.store_scalar(1867, 0.0);
            s.store_scalar(1837, 0.0);
            s.store_scalar(1832, 0.0);
            s.copy_ad(1885, 1433);
            s.store_offset(85, 85, ((10.0 * 2.220446049250313e-16) * 100000000.0));
            s.store_div_ad(962, A::mul(s.ad_value(1906), s.ad_value(622)), A::add(s.ad_value(964), s.ad_value(622)));
            s.store_sub(335, 1853, 1436);
        }

        s.b[1915] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
        s.v[1915] = if s.b[1915] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) {
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

        s.b[1916] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1916] = if s.b[1916] { 1.0 } else { 0.0 };

        s.b[1917] = (4.0 == 1.0);
        s.v[1917] = if s.b[1917] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) && s.b[1916]) && s.b[1917]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1918] = (4.0 == 2.0);
        s.v[1918] = if s.b[1918] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) && s.b[1916]) && (!s.b[1917])) && s.b[1918]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1919] = (4.0 == 4.0);
        s.v[1919] = if s.b[1919] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) && s.b[1916]) && (!s.b[1917])) && (!s.b[1918])) && s.b[1919]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1920] = (4.0 == 8.0);
        s.v[1920] = if s.b[1920] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) && s.b[1916]) && (!s.b[1917])) && (!s.b[1918])) && (!s.b[1919])) && s.b[1920]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) && s.b[1916]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign37590_loop_guard: usize = 0;
        while {
            let assign37590_cond_e43243: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) && s.b[1916]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign37590_cond_e43243 != 0.0
        } {
            assign37590_loop_guard += 1;
            assert!(assign37590_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) && s.b[1916]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) && (!s.b[1916])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                }
            });
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_ad_lhs(334, A::mul_scaled_lhs(s.ad_value(725), 0.1, s.ad_value(726)), 770);
            s.store_sub_from_scalar(336, 0.1, 780);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) {
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1915])) {
            s.copy_ad(336, 335);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_mul(1836, 962, 336);
            s.store_sqrt(1834, 1836);
        }

        s.b[1921] = (p.p345 != 0.0);
        s.v[1921] = if s.b[1921] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1921]) {
            s.store_mul_sub_from_scalar_ad_rhs(335, 965, 1.0, A::scale(s.ad_value(790), p.p345));
            s.store_scale(336, 965, 0.001);
            s.store_sub_ad_lhs(781, A::sub_scaled_inputs(s.ad_value(335), 1.0, s.ad_value(965), 0.1), 336);
            s.store_scaled_mul(782, 965, 336, (0.1 * 4.0));
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1921]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1921]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_ad_rhs(335, 965, 0.1, A::add_scaled_inputs(s.ad_value(781), 0.5, s.ad_value(782), 0.5));
            s.store_sub_ad_lhs(781, A::sub_scaled_inputs(s.ad_value(965), 2.0, s.ad_value(335), 1.0), 336);
            s.store_scaled_mul(782, 965, 336, (2.0 * 4.0));
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1921]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1921]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_sub_scaled_ad_rhs(965, 965, 2.0, A::add_scaled_inputs(s.ad_value(781), 0.5, s.ad_value(782), 0.5));
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_scalar(1849, 0.0);
        }

        s.b[1922] = (s.v[1834] > s.v[965]);
        s.v[1922] = if s.b[1922] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1922]) {
            s.copy_ad(1833, 965);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1922])) {
            s.copy_ad(1833, 1834);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_offset_sub_from_scalar_ad(781, 0.3, s.ad_value(85), (-0.01));
            s.store_scalar(782, ((4.0 * 0.3) * 0.01));
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_sub_from_scalar_ad(1858, 0.3, A::add_scaled_inputs(s.ad_value(781), 0.5, s.ad_value(782), 0.5));
        }

    }

    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
    ) {
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_offset_sub_ad(781, s.ad_value(1858), A::sub(s.ad_value(1885), s.ad_value(1853)), (-0.01));
            s.store_scaled_sub(782, 1885, 1853, (4.0 * 0.01));
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_ad(1858, A::sub(s.ad_value(1885), s.ad_value(1853)), A::add_scaled_inputs(s.ad_value(781), 0.5, s.ad_value(782), 0.5));
            s.store_div_ad(1886, A::mul_scaled_lhs(s.ad_value(1853), -1.0, s.ad_value(622)), A::add(s.ad_value(622), s.ad_value(964)));
            s.store_offset_sub(1832, 965, 1833, 1e-15);
            s.store_scalar(79, 0.0);
            s.store_scalar(1848, 0.2);
            s.copy_ad(1851, 1858);
            s.copy_ad(1854, 1849);
            s.copy_ad(1856, 1886);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign38090_loop_guard: usize = 0;
        while {
            let assign38090_cond_e43956: f64 = if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign38090_cond_e43956 != 0.0
        } {
            assign38090_loop_guard += 1;
            assert!(assign38090_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
                s.store_mul_sub_ad_rhs(1856, 1899, A::add_scaled_product(s.ad_value(1885), 1.0, s.ad_value(1900), s.ad_value(1854), 1.0), s.ad_value(1853));
                s.store_mul(1840, 1899, 1900);
                s.store_sub(335, 1854, 1856);
            }
            s.b[1923] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1923] = if s.b[1923] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) {
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
            s.b[1924] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1924] = if s.b[1924] { 1.0 } else { 0.0 };
            s.b[1925] = (2.0 == 1.0);
            s.v[1925] = if s.b[1925] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && s.b[1924]) && s.b[1925]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1926] = (2.0 == 2.0);
            s.v[1926] = if s.b[1926] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && s.b[1924]) && (!s.b[1925])) && s.b[1926]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1927] = (2.0 == 4.0);
            s.v[1927] = if s.b[1927] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && s.b[1924]) && (!s.b[1925])) && (!s.b[1926])) && s.b[1927]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1928] = (2.0 == 8.0);
            s.v[1928] = if s.b[1928] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && s.b[1924]) && (!s.b[1925])) && (!s.b[1926])) && (!s.b[1927])) && s.b[1928]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && s.b[1924]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign38090_body29_loop_guard: usize = 0;
            while {
                let assign38090_body29_cond_e44319: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && s.b[1924]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38090_body29_cond_e44319 != 0.0
            } {
                assign38090_body29_loop_guard += 1;
                assert!(assign38090_body29_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && s.b[1924]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && (!s.b[1924])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_ad_lhs(334, A::mul_scaled_lhs(s.ad_value(725), 0.1, s.ad_value(726)), 770);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) {
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1923])) {
                s.copy_ad(336, 335);
                s.store_scalar(334, 1.0);
            }
            if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
                s.store_sqrt_mul(1828, 1906, 336);
            }
            s.b[1929] = ((s.v[1828] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.v[1929] = if s.b[1929] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) {
                s.store_offset_sub(781, 1828, 965, 1e-8);
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
            s.b[1930] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1930] = if s.b[1930] { 1.0 } else { 0.0 };
            s.b[1931] = (2.0 == 1.0);
            s.v[1931] = if s.b[1931] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && s.b[1930]) && s.b[1931]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1932] = (2.0 == 2.0);
            s.v[1932] = if s.b[1932] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && s.b[1930]) && (!s.b[1931])) && s.b[1932]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1933] = (2.0 == 4.0);
            s.v[1933] = if s.b[1933] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && s.b[1930]) && (!s.b[1931])) && (!s.b[1932])) && s.b[1933]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1934] = (2.0 == 8.0);
            s.v[1934] = if s.b[1934] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && s.b[1930]) && (!s.b[1931])) && (!s.b[1932])) && (!s.b[1933])) && s.b[1934]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && s.b[1930]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign38090_body65_loop_guard: usize = 0;
            while {
                let assign38090_body65_cond_e44804: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && s.b[1930]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38090_body65_cond_e44804 != 0.0
            } {
                assign38090_body65_loop_guard += 1;
                assert!(assign38090_body65_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && s.b[1930]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && (!s.b[1930])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_ad_lhs(337, A::mul_scaled_lhs(s.ad_value(725), 1e-8, s.ad_value(726)), 770);
                s.store_add_ad_lhs(1828, A::offset(s.ad_value(965), (-1e-8)), 780);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) {
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1929])) {
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1929])) {
                s.store_scalar(337, 1.0);
            }
            if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
                s.store_mul(1864, 1828, 1903);
                s.store_mul_ad_product_lhs(1842, A::div_from_scalar(1.034943e-10, s.ad_value(1828)), s.ad_value(334), 337);
                s.store_mul_ad_product_lhs(1844, A::div_from_scalar((-1.034943e-10), s.ad_value(1828)), s.ad_value(334), 337);
            }
            s.b[1935] = (p.p49 == 0.0);
            s.v[1935] = if s.b[1935] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1935]) {
                s.store_add_ad_lhs(1837, A::div(A::add_scaled_product(A::add(s.ad_value(1901), s.ad_value(1836)), 1.0, s.ad_value(965), s.ad_value(1833), (-2.0)), s.ad_value(1906)), 1851);
                s.store_scalar(1838, 1.0);
                s.store_scalar(1839, 0.0);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1935])) {
                s.store_add_ad_rhs(1837, 1851, A::div(A::add_scaled_product(s.ad_value(1901), 1.0, s.ad_value(1828), A::sub_scaled_inputs(s.ad_value(1828), 1.0, s.ad_value(965), 2.0), 1.0), s.ad_value(1906)));
                s.store_scalar(1838, 1.0);
                s.store_mul_ad(1839, A::add_scaled_product(s.ad_value(334), 1.0, A::div(s.ad_value(965), s.ad_value(1828)), s.ad_value(334), (-1.0)), A::sub_from_scalar(1.0, s.ad_value(1840)));
            }
            s.b[1936] = ((s.v[1837] > (s.v[1849] - s.v[1848])) && (s.v[1848] >= 0.0));
            s.v[1936] = if s.b[1936] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) {
                s.store_add_ad_lhs(781, A::sub(s.ad_value(1837), s.ad_value(1849)), 1848);
                s.store_square(722, 781);
                s.store_square(723, 1848);
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
            s.b[1937] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.v[1937] = if s.b[1937] { 1.0 } else { 0.0 };
            s.b[1938] = (4.0 == 1.0);
            s.v[1938] = if s.b[1938] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && s.b[1937]) && s.b[1938]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1939] = (4.0 == 2.0);
            s.v[1939] = if s.b[1939] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && s.b[1937]) && (!s.b[1938])) && s.b[1939]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1940] = (4.0 == 4.0);
            s.v[1940] = if s.b[1940] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && s.b[1937]) && (!s.b[1938])) && (!s.b[1939])) && s.b[1940]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1941] = (4.0 == 8.0);
            s.v[1941] = if s.b[1941] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && s.b[1937]) && (!s.b[1938])) && (!s.b[1939])) && (!s.b[1940])) && s.b[1941]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && s.b[1937]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign38090_body114_loop_guard: usize = 0;
            while {
                let assign38090_body114_cond_e45477: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && s.b[1937]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38090_body114_cond_e45477 != 0.0
            } {
                assign38090_body114_loop_guard += 1;
                assert!(assign38090_body114_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && s.b[1937]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && (!s.b[1937])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                    }
                });
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_mul3_lhs(780, 781, 1848, 726);
                s.store_div_ad_lhs(334, A::mul3(s.ad_value(1848), s.ad_value(725), s.ad_value(726)), 770);
                s.store_add_ad_lhs(1837, A::sub(s.ad_value(1849), s.ad_value(1848)), 780);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) {
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1936])) {
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1936])) {
                s.store_scalar(334, 1.0);
            }
            if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
                s.store_mul(1838, 1838, 334);
                s.store_mul(1839, 1839, 334);
                s.store_add_ad_lhs(335, A::sub(s.ad_value(1856), s.ad_value(1885)), 1853);
            }
            s.b[1942] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1942] = if s.b[1942] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) {
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
            s.b[1943] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1943] = if s.b[1943] { 1.0 } else { 0.0 };
            s.b[1944] = (2.0 == 1.0);
            s.v[1944] = if s.b[1944] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && s.b[1943]) && s.b[1944]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1945] = (2.0 == 2.0);
            s.v[1945] = if s.b[1945] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && s.b[1943]) && (!s.b[1944])) && s.b[1945]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1946] = (2.0 == 4.0);
            s.v[1946] = if s.b[1946] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && s.b[1943]) && (!s.b[1944])) && (!s.b[1945])) && s.b[1946]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1947] = (2.0 == 8.0);
            s.v[1947] = if s.b[1947] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && s.b[1943]) && (!s.b[1944])) && (!s.b[1945])) && (!s.b[1946])) && s.b[1947]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && s.b[1943]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign38090_body152_loop_guard: usize = 0;
            while {
                let assign38090_body152_cond_e45985: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && s.b[1943]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38090_body152_cond_e45985 != 0.0
            } {
                assign38090_body152_loop_guard += 1;
                assert!(assign38090_body152_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && s.b[1943]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && (!s.b[1943])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_ad_lhs(337, A::mul_scaled_lhs(s.ad_value(725), 0.1, s.ad_value(726)), 770);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) {
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1942])) {
                s.copy_ad(336, 335);
                s.store_scalar(337, 1.0);
            }
            if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
                s.store_sqrt_mul(1830, 1907, 336);
                s.store_mul_neg_lhs(1865, 1830, 1902);
                s.store_mul_div_from_scalar_lhs(1846, (-1.034943e-10), 1830, 337);
                s.store_mul_sub_rhs(335, 154, 1851, 1854);
                s.store_exp(336, 335);
            }
            s.b[1948] = (s.v[1851] >= s.v[1854]);
            s.v[1948] = if s.b[1948] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1948]) {
                s.store_mul_scaled_ad_rhs(1860, 209, -1.0, A::sqrt(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15)));
                s.store_mul_ad(1895, A::div(A::mul_scaled_lhs(s.ad_value(209), 0.5, s.ad_value(209)), s.ad_value(1860)), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
                s.store_neg(1897, 1895);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1948])) {
                s.store_exp_ad(337, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1851), s.ad_value(1885))));
                s.store_exp_ad(338, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1854), s.ad_value(1885))));
                s.store_mul_sqrt_ad_rhs(1860, 209, A::offset(A::add_scaled_product(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1.0, s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_ad_lhs(339, A::mul_scaled_lhs(s.ad_value(209), 0.5, s.ad_value(209)), 1860);
                s.store_mul_add_ad_rhs(1895, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
                s.store_mul_add_ad_rhs(1897, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] != 0.0)) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) {
                s.store_add_ad(1868, A::add_scaled_product(s.ad_value(1860), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1851)), 1.0), A::add(s.ad_value(1864), s.ad_value(1865)));
                s.store_sub(1869, 1895, 185);
                s.store_add_ad_rhs(1870, 1897, A::add_scaled_product(A::add_scaled_product(s.ad_value(1842), 1.0, s.ad_value(1844), s.ad_value(1840), 1.0), 1.0, s.ad_value(1846), s.ad_value(1840), 1.0));
                s.store_sub(1871, 1854, 1837);
                s.store_neg(1872, 1838);
                s.store_sub_from_scalar(1873, 1.0, 1839);
                s.store_ad_value(1874, A::add_scaled_products(s.ad_value(1869), s.ad_value(1873), 1.0, s.ad_value(1870), s.ad_value(1872), (-1.0)));
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) {
                s.store_ad_value(1875, {
                    if (s.v[1874] > 0.0) {
                        A::div_from_scalar(1.0, A::offset(s.ad_value(1874), 1e-25))
                    } else {
                        A::div_from_scalar(1.0, A::offset(s.ad_value(1874), (-1e-25)))
                    }
                });
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) {
                s.copy_ad(1876, 1873);
                s.store_neg(1877, 1870);
                s.store_neg(1878, 1872);
                s.copy_ad(1879, 1869);
                s.store_mul_scaled_ad_rhs(1880, 1875, -1.0, A::add_scaled_products(s.ad_value(1876), s.ad_value(1868), 1.0, s.ad_value(1877), s.ad_value(1871), 1.0));
                s.store_mul_scaled_ad_rhs(1881, 1875, -1.0, A::add_scaled_products(s.ad_value(1878), s.ad_value(1868), 1.0, s.ad_value(1879), s.ad_value(1871), 1.0));
                s.store_abs(335, 1880);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) {
                s.store_ad_value(335, {
                    if (s.v[335] < ((s.v[1881]) as f64).abs()) {
                        A::abs(s.ad_value(1881))
                    } else {
                        s.ad_value(335)
                    }
                });
            }
            s.b[1949] = (s.v[335] > 0.1);
            s.v[1949] = if s.b[1949] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) && s.b[1949]) {
                s.store_mul_div_from_scalar_rhs(1880, 1880, 0.1, 335);
                s.store_mul_div_from_scalar_rhs(1881, 1881, 0.1, 335);
            }
            s.b[1950] = (s.v[335] < 1e-12);
            s.v[1950] = if s.b[1950] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) && s.b[1950]) {
                s.store_scalar(79, 1.0);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) {
                s.store_add(1851, 1851, 1880);
                s.store_add(1854, 1854, 1881);
            }
            if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
                s.store_offset(97, 97, 1.0);
            }
        }

    }
}
