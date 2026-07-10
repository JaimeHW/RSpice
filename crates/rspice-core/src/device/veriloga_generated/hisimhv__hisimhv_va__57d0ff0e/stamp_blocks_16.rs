#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_37(
        s: &mut ReactiveScratch,
    ) {
        let mut t2: usize = 0;
        while {
            let t0: f64 = (s.v[421] + 1.0);let t1: f64 = if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) && (s.v[97] <= t0)) { 1.0 } else { 0.0 };
            t1 != 0.0
        } {
            t2 += 1;assert!(t2 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) {s.store_add_scaled_inputs3_indices(1468, 1449, 1.0, 1447, 1.0, 965, -1.0);s.store_add_ad(1508, A::div_scalar_by_product(1.034943e-10, s.ad_value(1546), s.ad_value(1449), 1.0), A::div(A::mul_sub_from_scalar_rhs(A::div_from_scalar(1.034943e-10, s.ad_value(1546)), 1.0, A::div_scaled_value_offset_denominator(s.ad_value(1536), 1.0, s.ad_value(1536), 1.0, 1.0)), s.ad_value(1447)));}
            s.b[1591] = ((((s.v[1468] / s.v[1508])) as f64).abs() > 0.5);s.store_scalar(1591, if s.b[1591] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) && s.b[1591]) {s.store_offset(1483, 1483, (-(0.5 * (if ((s.v[1468] / s.v[1508]) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) && (!s.b[1591])) {s.store_sub_div_rhs_indices(1483, 1483, 1468, 1508);}
            s.b[1592] = (((s.v[1483] - s.v[1435]) + s.v[1463]) < (10.0 * 2.220446049250313e-16));s.store_scalar(1592, if s.b[1592] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) && s.b[1592]) {s.store_offset_sub(1483, 1435, 1463, (10.0 * 2.220446049250313e-16));}
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) {s.store_sqrt_mul_sub_rhs(1449, 1547, 1483, 1480);s.store_div_scaled_inputs2_mixed_aia(1464, A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1483), 1.0), 1.0, 1463, (-1.0), A::offset(s.ad_value(1536), 1.0), 1.0);s.store_sqrt_mul_sub_rhs(1447, 1547, 1483, 1464);}
            s.b[1593] = ((((s.v[1483] - s.v[1470])) as f64).abs() <= 1e-5);s.store_scalar(1593, if s.b[1593] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) && s.b[1593]) {s.store_scalar(97, (s.v[421] + 1.0));}
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) {s.copy_ad(1470, 1483);s.store_primal_offset(97, 97, 1.0);}
        }
        if (s.b[1443] && s.b[1444]) {s.copy_ad(1482, 1483);s.store_scalar(1519, 0.12);s.store_scalar(79, 0.0);s.copy_ad(1461, 1480);s.copy_ad(1483, 1482);s.copy_ad(1469, 1461);s.copy_ad(1470, 1483);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_38(
        s: &mut ReactiveScratch,
    ) {
        let mut tb: usize = 0;
        while {
            let t9: f64 = (150.0 + 1.0);let ta: f64 = if ((s.b[1443] && s.b[1444]) && (s.v[97] <= t9)) { 1.0 } else { 0.0 };
            ta != 0.0
        } {
            tb += 1;assert!(tb <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1443] && s.b[1444]) {s.store_mul_sub_mixed_iai(1464, 1535, A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1483), 1.0), 1463);s.store_mul(1533, 1535, 1536);s.store_sub(335, 1483, 1464);}
            s.b[1594] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1594, if s.b[1594] { 1.0 } else { 0.0 });
            if ((s.b[1443] && s.b[1444]) && s.b[1594]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1595] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1595, if s.b[1595] { 1.0 } else { 0.0 });s.b[1596] = (2.0 == 1.0);s.store_scalar(1596, if s.b[1596] { 1.0 } else { 0.0 });
            if ((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && s.b[1596]) {s.store_scalar(720, 1.0);}
            s.b[1597] = (2.0 == 2.0);s.store_scalar(1597, if s.b[1597] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && s.b[1597]) {s.store_scalar(720, 2.0);}
            s.b[1598] = (2.0 == 4.0);s.store_scalar(1598, if s.b[1598] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && (!s.b[1597])) && s.b[1598]) {s.store_scalar(720, 3.0);}
            s.b[1599] = (2.0 == 8.0);s.store_scalar(1599, if s.b[1599] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && (!s.b[1597])) && (!s.b[1598])) && s.b[1599]) {s.store_scalar(720, 4.0);}
            if (((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) {s.store_scalar(719, 0.0);}
            let mut t6: usize = 0;
            while {
                let t5: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t5 != 0.0
            } {
                t6 += 1;assert!(t6 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
            if ((s.b[1443] && s.b[1444]) && s.b[1600]) {s.store_offset_sub(781, 1447, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1601] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1601, if s.b[1601] { 1.0 } else { 0.0 });s.b[1602] = (2.0 == 1.0);s.store_scalar(1602, if s.b[1602] { 1.0 } else { 0.0 });
            if ((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && s.b[1602]) {s.store_scalar(720, 1.0);}
            s.b[1603] = (2.0 == 2.0);s.store_scalar(1603, if s.b[1603] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && (!s.b[1602])) && s.b[1603]) {s.store_scalar(720, 2.0);}
            s.b[1604] = (2.0 == 4.0);s.store_scalar(1604, if s.b[1604] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && (!s.b[1602])) && (!s.b[1603])) && s.b[1604]) {s.store_scalar(720, 3.0);}
            s.b[1605] = (2.0 == 8.0);s.store_scalar(1605, if s.b[1605] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && (!s.b[1602])) && (!s.b[1603])) && (!s.b[1604])) && s.b[1605]) {s.store_scalar(720, 4.0);}
            if (((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) {s.store_scalar(719, 0.0);}
            let mut t8: usize = 0;
            while {
                let t7: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t7 != 0.0
            } {
                t8 += 1;assert!(t8 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
            if ((s.b[1443] && s.b[1444]) && s.b[1607]) {s.store_add_scaled_inputs3_indices(781, 1520, 1.0, 1511, (-1.0), 1519, 1.0);s.store_square(722, 781);s.store_square(723, 1519);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1608] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1608, if s.b[1608] { 1.0 } else { 0.0 });s.b[1609] = (4.0 == 1.0);s.store_scalar(1609, if s.b[1609] { 1.0 } else { 0.0 });
            if ((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && s.b[1609]) {s.store_scalar(720, 1.0);}
            s.b[1610] = (4.0 == 2.0);s.store_scalar(1610, if s.b[1610] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && (!s.b[1609])) && s.b[1610]) {s.store_scalar(720, 2.0);}
            s.b[1611] = (4.0 == 4.0);s.store_scalar(1611, if s.b[1611] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && (!s.b[1609])) && (!s.b[1610])) && s.b[1611]) {s.store_scalar(720, 3.0);}
            s.b[1612] = (4.0 == 8.0);s.store_scalar(1612, if s.b[1612] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && (!s.b[1609])) && (!s.b[1610])) && (!s.b[1611])) && s.b[1612]) {s.store_scalar(720, 4.0);}
            if (((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) {s.store_scalar(719, 0.0);}
            let mut t4: usize = 0;
            while {
                let t3: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t3 != 0.0
            } {
                t4 += 1;assert!(t4 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
            if ((s.b[1443] && s.b[1444]) && s.b[1614]) {s.store_scalar(97, (150.0 + 1.0));s.store_scalar(79, 1.0);}
            if (s.b[1443] && s.b[1444]) {s.copy_ad(1469, 1461);s.copy_ad(1470, 1483);s.store_primal_offset(97, 97, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_39(
        s: &mut ReactiveScratch,
    ) {
        s.b[1616] = ((s.v[1453] > s.v[965]) && (s.v[1479] != 2.0));s.store_scalar(1616, if s.b[1616] { 1.0 } else { 0.0 });s.b[1617] = ((s.v[1483] > (s.v[1461] - 0.02)) && (0.02 >= 0.0));s.store_scalar(1617, if s.b[1617] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) {s.store_offset_sub(781, 1483, 1461, 0.02);s.store_square(722, 781);s.store_scalar(723, (0.02 * 0.02));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1618] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1618, if s.b[1618] { 1.0 } else { 0.0 });s.b[1619] = (2.0 == 1.0);s.store_scalar(1619, if s.b[1619] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && s.b[1619]) {s.store_scalar(720, 1.0);}
        s.b[1620] = (2.0 == 2.0);s.store_scalar(1620, if s.b[1620] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1620]) {s.store_scalar(720, 2.0);}
        s.b[1621] = (2.0 == 4.0);s.store_scalar(1621, if s.b[1621] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && (!s.b[1620])) && s.b[1621]) {s.store_scalar(720, 3.0);}
        s.b[1622] = (2.0 == 8.0);s.store_scalar(1622, if s.b[1622] { 1.0 } else { 0.0 });
        if ((((((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && (!s.b[1620])) && (!s.b[1621])) && s.b[1622]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) {s.store_scalar(719, 0.0);}
        let mut td: usize = 0;
        while {
            let tc: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tc != 0.0
        } {
            td += 1;assert!(td <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if (((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) {s.store_offset_sub(781, 1447, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1625] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1625, if s.b[1625] { 1.0 } else { 0.0 });s.b[1626] = (2.0 == 1.0);s.store_scalar(1626, if s.b[1626] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && s.b[1626]) {s.store_scalar(720, 1.0);}
        s.b[1627] = (2.0 == 2.0);s.store_scalar(1627, if s.b[1627] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_40(
        s: &mut ReactiveScratch,
    ) {
        if ((((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && (!s.b[1626])) && s.b[1627]) {s.store_scalar(720, 2.0);}
        s.b[1628] = (2.0 == 4.0);s.store_scalar(1628, if s.b[1628] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && (!s.b[1626])) && (!s.b[1627])) && s.b[1628]) {s.store_scalar(720, 3.0);}
        s.b[1629] = (2.0 == 8.0);s.store_scalar(1629, if s.b[1629] { 1.0 } else { 0.0 });
        if ((((((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && (!s.b[1626])) && (!s.b[1627])) && (!s.b[1628])) && s.b[1629]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) {s.store_scalar(719, 0.0);}
        let mut tf: usize = 0;
        while {
            let te: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            te != 0.0
        } {
            tf += 1;assert!(tf <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && (!s.b[1630])) {s.store_mul_sqrt_mixed_ia(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1461), s.ad_value(1435)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1483), s.ad_value(1435)))))), s.ad_value(335)));s.store_add_scaled_product_mixed_iia(1477, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));s.store_mul_sqrt_mixed_ia(1517, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));}
        if ((s.b[1443] && s.b[1444]) && (!s.b[1623])) {s.store_scalar(1538, 0.0);s.store_sub(335, 1483, 1464);}
        s.b[1631] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1631, if s.b[1631] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_41(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) {s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1632] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1632, if s.b[1632] { 1.0 } else { 0.0 });s.b[1633] = (2.0 == 1.0);s.store_scalar(1633, if s.b[1633] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && s.b[1633]) {s.store_scalar(720, 1.0);}
        s.b[1634] = (2.0 == 2.0);s.store_scalar(1634, if s.b[1634] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && (!s.b[1633])) && s.b[1634]) {s.store_scalar(720, 2.0);}
        s.b[1635] = (2.0 == 4.0);s.store_scalar(1635, if s.b[1635] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && (!s.b[1633])) && (!s.b[1634])) && s.b[1635]) {s.store_scalar(720, 3.0);}
        s.b[1636] = (2.0 == 8.0);s.store_scalar(1636, if s.b[1636] { 1.0 } else { 0.0 });
        if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && (!s.b[1633])) && (!s.b[1634])) && (!s.b[1635])) && s.b[1636]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) {s.store_scalar(719, 0.0);}
        let mut t11: usize = 0;
        while {
            let t10: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t10 != 0.0
        } {
            t11 += 1;assert!(t11 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) {s.store_offset_sub(781, 1447, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1638] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1638, if s.b[1638] { 1.0 } else { 0.0 });s.b[1639] = (2.0 == 1.0);s.store_scalar(1639, if s.b[1639] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && s.b[1639]) {s.store_scalar(720, 1.0);}
        s.b[1640] = (2.0 == 2.0);s.store_scalar(1640, if s.b[1640] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && (!s.b[1639])) && s.b[1640]) {s.store_scalar(720, 2.0);}
        s.b[1641] = (2.0 == 4.0);s.store_scalar(1641, if s.b[1641] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && (!s.b[1639])) && (!s.b[1640])) && s.b[1641]) {s.store_scalar(720, 3.0);}
        s.b[1642] = (2.0 == 8.0);s.store_scalar(1642, if s.b[1642] { 1.0 } else { 0.0 });
        if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && (!s.b[1639])) && (!s.b[1640])) && (!s.b[1641])) && s.b[1642]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) {s.store_scalar(719, 0.0);}
        let mut t13: usize = 0;
        while {
            let t12: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t12 != 0.0
        } {
            t13 += 1;assert!(t13 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if ((s.b[1443] && s.b[1444]) && s.b[1643]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_42(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1443] && s.b[1444]) && s.b[1643]) {s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1644] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1644, if s.b[1644] { 1.0 } else { 0.0 });s.b[1645] = (2.0 == 1.0);s.store_scalar(1645, if s.b[1645] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && s.b[1645]) {s.store_scalar(720, 1.0);}
        s.b[1646] = (2.0 == 2.0);s.store_scalar(1646, if s.b[1646] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && (!s.b[1645])) && s.b[1646]) {s.store_scalar(720, 2.0);}
        s.b[1647] = (2.0 == 4.0);s.store_scalar(1647, if s.b[1647] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && (!s.b[1645])) && (!s.b[1646])) && s.b[1647]) {s.store_scalar(720, 3.0);}
        s.b[1648] = (2.0 == 8.0);s.store_scalar(1648, if s.b[1648] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && (!s.b[1645])) && (!s.b[1646])) && (!s.b[1647])) && s.b[1648]) {s.store_scalar(720, 4.0);}
        if (((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) {s.store_scalar(719, 0.0);}
        let mut t15: usize = 0;
        while {
            let t14: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t14 != 0.0
        } {
            t15 += 1;assert!(t15 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if ((s.b[1443] && s.b[1444]) && s.b[1649]) {s.store_offset_sub(781, 1447, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1650] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1650, if s.b[1650] { 1.0 } else { 0.0 });s.b[1651] = (2.0 == 1.0);s.store_scalar(1651, if s.b[1651] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && s.b[1651]) {s.store_scalar(720, 1.0);}
        s.b[1652] = (2.0 == 2.0);s.store_scalar(1652, if s.b[1652] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && (!s.b[1651])) && s.b[1652]) {s.store_scalar(720, 2.0);}
        s.b[1653] = (2.0 == 4.0);s.store_scalar(1653, if s.b[1653] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && (!s.b[1651])) && (!s.b[1652])) && s.b[1653]) {s.store_scalar(720, 3.0);}
        s.b[1654] = (2.0 == 8.0);s.store_scalar(1654, if s.b[1654] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && (!s.b[1651])) && (!s.b[1652])) && (!s.b[1653])) && s.b[1654]) {s.store_scalar(720, 4.0);}
        if (((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) {s.store_scalar(719, 0.0);}
        let mut t17: usize = 0;
        while {
            let t16: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t16 != 0.0
        } {
            t17 += 1;assert!(t17 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if ((s.b[1443] && s.b[1444]) && s.b[1655]) {s.store_sub_from_scalar(781, 0.05, 335);s.store_square(722, 781);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_43(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1443] && s.b[1444]) && s.b[1655]) {s.store_scalar(723, (0.05 * 0.05));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1656] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1656, if s.b[1656] { 1.0 } else { 0.0 });s.b[1657] = (2.0 == 1.0);s.store_scalar(1657, if s.b[1657] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && s.b[1657]) {s.store_scalar(720, 1.0);}
        s.b[1658] = (2.0 == 2.0);s.store_scalar(1658, if s.b[1658] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && (!s.b[1657])) && s.b[1658]) {s.store_scalar(720, 2.0);}
        s.b[1659] = (2.0 == 4.0);s.store_scalar(1659, if s.b[1659] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && (!s.b[1657])) && (!s.b[1658])) && s.b[1659]) {s.store_scalar(720, 3.0);}
        s.b[1660] = (2.0 == 8.0);s.store_scalar(1660, if s.b[1660] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && (!s.b[1657])) && (!s.b[1658])) && (!s.b[1659])) && s.b[1660]) {s.store_scalar(720, 4.0);}
        if (((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) {s.store_scalar(719, 0.0);}
        let mut t19: usize = 0;
        while {
            let t18: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t18 != 0.0
        } {
            t19 += 1;assert!(t19 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if ((s.b[1443] && s.b[1444]) && s.b[1661]) {s.store_sub_from_scalar(781, (1e-25 + 1e-18), 335);s.store_square(722, 781);s.store_scalar(723, (1e-18 * 1e-18));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1662] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1662, if s.b[1662] { 1.0 } else { 0.0 });s.b[1663] = (2.0 == 1.0);s.store_scalar(1663, if s.b[1663] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && s.b[1663]) {s.store_scalar(720, 1.0);}
        s.b[1664] = (2.0 == 2.0);s.store_scalar(1664, if s.b[1664] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1664]) {s.store_scalar(720, 2.0);}
        s.b[1665] = (2.0 == 4.0);s.store_scalar(1665, if s.b[1665] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && (!s.b[1664])) && s.b[1665]) {s.store_scalar(720, 3.0);}
        s.b[1666] = (2.0 == 8.0);s.store_scalar(1666, if s.b[1666] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && (!s.b[1664])) && (!s.b[1665])) && s.b[1666]) {s.store_scalar(720, 4.0);}
        if (((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) {s.store_scalar(719, 0.0);}
        let mut t1b: usize = 0;
        while {
            let t1a: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t1a != 0.0
        } {
            t1b += 1;assert!(t1b <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_44(
        s: &mut ReactiveScratch,
    ) {
        s.b[1667] = ((s.v[1453] > s.v[965]) && (s.v[1479] != 2.0));s.store_scalar(1667, if s.b[1667] { 1.0 } else { 0.0 });s.b[1668] = ((s.v[1461] > (s.v[1511] - 0.8)) && (0.8 >= 0.0));s.store_scalar(1668, if s.b[1668] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) {s.store_offset_sub(781, 1461, 1511, 0.8);s.store_square(722, 781);s.store_scalar(723, (0.8 * 0.8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1669] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1669, if s.b[1669] { 1.0 } else { 0.0 });s.b[1670] = (2.0 == 1.0);s.store_scalar(1670, if s.b[1670] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && s.b[1670]) {s.store_scalar(720, 1.0);}
        s.b[1671] = (2.0 == 2.0);s.store_scalar(1671, if s.b[1671] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && (!s.b[1670])) && s.b[1671]) {s.store_scalar(720, 2.0);}
        s.b[1672] = (2.0 == 4.0);s.store_scalar(1672, if s.b[1672] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && (!s.b[1670])) && (!s.b[1671])) && s.b[1672]) {s.store_scalar(720, 3.0);}
        s.b[1673] = (2.0 == 8.0);s.store_scalar(1673, if s.b[1673] { 1.0 } else { 0.0 });
        if ((((((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && (!s.b[1670])) && (!s.b[1671])) && (!s.b[1672])) && s.b[1673]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) {s.store_scalar(719, 0.0);}
        let mut t1d: usize = 0;
        while {
            let t1c: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t1c != 0.0
        } {
            t1d += 1;assert!(t1d <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if (((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) {s.store_offset_sub(781, 1520, 1511, 0.8);s.store_square(722, 781);s.store_scalar(723, (0.8 * 0.8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1675] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1675, if s.b[1675] { 1.0 } else { 0.0 });s.b[1676] = (2.0 == 1.0);s.store_scalar(1676, if s.b[1676] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) && s.b[1676]) {s.store_scalar(720, 1.0);}
        s.b[1677] = (2.0 == 2.0);s.store_scalar(1677, if s.b[1677] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) && (!s.b[1676])) && s.b[1677]) {s.store_scalar(720, 2.0);}
        s.b[1678] = (2.0 == 4.0);s.store_scalar(1678, if s.b[1678] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) && (!s.b[1676])) && (!s.b[1677])) && s.b[1678]) {s.store_scalar(720, 3.0);}
        s.b[1679] = (2.0 == 8.0);s.store_scalar(1679, if s.b[1679] { 1.0 } else { 0.0 });
        if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) && (!s.b[1676])) && (!s.b[1677])) && (!s.b[1678])) && s.b[1679]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) {s.store_scalar(719, 0.0);}
        let mut t1f: usize = 0;
        while {
            let t1e: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t1e != 0.0
        } {
            t1f += 1;assert!(t1f <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
    pub(super) fn stamp_reactive_block_45(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[1443] && s.b[1444]) && (!s.b[1667])) && (!s.b[1674])) {s.copy_ad(336, 1520);s.store_scalar(334, 1.0);}
        if (s.b[1443] && s.b[1444]) {s.store_mul_ad_affine_product_lhs(1505, s.ad_value(964), A::exp(A::mul(s.ad_value(154), A::sub(s.ad_value(336), s.ad_value(1511)))), (-1.6021918e-19), 0.0, 1447);}
        s.b[1680] = (((s.v[1461] - s.v[1511]) < 0.06) && (0.06 >= 0.0));s.store_scalar(1680, if s.b[1680] { 1.0 } else { 0.0 });
        if ((s.b[1443] && s.b[1444]) && s.b[1680]) {s.store_sub_from_scalar_ad(781, 0.06, A::sub(s.ad_value(1461), s.ad_value(1511)));s.store_square(722, 781);s.store_scalar(723, (0.06 * 0.06));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1681] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1681, if s.b[1681] { 1.0 } else { 0.0 });s.b[1682] = (2.0 == 1.0);s.store_scalar(1682, if s.b[1682] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) && s.b[1682]) {s.store_scalar(720, 1.0);}
        s.b[1683] = (2.0 == 2.0);s.store_scalar(1683, if s.b[1683] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) && (!s.b[1682])) && s.b[1683]) {s.store_scalar(720, 2.0);}
        s.b[1684] = (2.0 == 4.0);s.store_scalar(1684, if s.b[1684] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) && (!s.b[1682])) && (!s.b[1683])) && s.b[1684]) {s.store_scalar(720, 3.0);}
        s.b[1685] = (2.0 == 8.0);s.store_scalar(1685, if s.b[1685] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) && (!s.b[1682])) && (!s.b[1683])) && (!s.b[1684])) && s.b[1685]) {s.store_scalar(720, 4.0);}
        if (((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) {s.store_scalar(719, 0.0);}
        let mut t21: usize = 0;
        while {
            let t20: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t20 != 0.0
        } {
            t21 += 1;assert!(t21 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) {s.store_sub_from_scalar(781, 2.0, 338);s.store_square(722, 781);s.store_scalar(723, (2.0 * 2.0));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_46(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) {s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1688] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1688, if s.b[1688] { 1.0 } else { 0.0 });s.b[1689] = (2.0 == 1.0);s.store_scalar(1689, if s.b[1689] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) && s.b[1689]) {s.store_scalar(720, 1.0);}
        s.b[1690] = (2.0 == 2.0);s.store_scalar(1690, if s.b[1690] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) && (!s.b[1689])) && s.b[1690]) {s.store_scalar(720, 2.0);}
        s.b[1691] = (2.0 == 4.0);s.store_scalar(1691, if s.b[1691] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) && (!s.b[1689])) && (!s.b[1690])) && s.b[1691]) {s.store_scalar(720, 3.0);}
        s.b[1692] = (2.0 == 8.0);s.store_scalar(1692, if s.b[1692] { 1.0 } else { 0.0 });
        if ((((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) && (!s.b[1689])) && (!s.b[1690])) && (!s.b[1691])) && s.b[1692]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) {s.store_scalar(719, 0.0);}
        let mut t23: usize = 0;
        while {
            let t22: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t22 != 0.0
        } {
            t23 += 1;assert!(t23 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) {s.store_sub_from_scalar(781, (0.3 + 0.2), 344);s.store_square(722, 781);s.store_scalar(723, (0.2 * 0.2));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1694] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1694, if s.b[1694] { 1.0 } else { 0.0 });s.b[1695] = (4.0 == 1.0);s.store_scalar(1695, if s.b[1695] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) && s.b[1695]) {s.store_scalar(720, 1.0);}
        s.b[1696] = (4.0 == 2.0);s.store_scalar(1696, if s.b[1696] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) && (!s.b[1695])) && s.b[1696]) {s.store_scalar(720, 2.0);}
        s.b[1697] = (4.0 == 4.0);s.store_scalar(1697, if s.b[1697] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) && (!s.b[1695])) && (!s.b[1696])) && s.b[1697]) {s.store_scalar(720, 3.0);}
        s.b[1698] = (4.0 == 8.0);s.store_scalar(1698, if s.b[1698] { 1.0 } else { 0.0 });
        if ((((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) && (!s.b[1695])) && (!s.b[1696])) && (!s.b[1697])) && s.b[1698]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) {s.store_scalar(719, 0.0);}
        let mut t25: usize = 0;
        while {
            let t24: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t24 != 0.0
        } {
            t25 += 1;assert!(t25 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && (!s.b[1694])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.2);s.store_div_scaled_product_indices(334, 725, 726, 0.2, 770, 1.0);s.store_sub_from_scalar(344, (0.3 + 0.2), 780);}
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) {
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && (!s.b[1693])) {
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && (!s.b[1693])) {s.store_scalar(334, 1.0);}
        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {s.store_offset(344, 344, (10.0 * 2.220446049250313e-16));s.store_div(335, 790, 344);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_47(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_offset_rhs(336, 335, 658, (-1.0));
            }
        }
        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {s.store_mul(341, 336, 335);s.store_offset(337, 341, 1.0);}
        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
        }
        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {s.store_mul(340, 338, 337);s.store_div(348, 790, 340);}
        s.b[1699] = ((s.v[85] < 0.5) && (0.5 >= 0.0));s.store_scalar(1699, if s.b[1699] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) {s.store_sub_from_scalar(781, 0.5, 85);s.store_square(722, 781);s.store_scalar(723, (0.5 * 0.5));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1700] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1700, if s.b[1700] { 1.0 } else { 0.0 });s.b[1701] = (2.0 == 1.0);s.store_scalar(1701, if s.b[1701] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) && s.b[1701]) {s.store_scalar(720, 1.0);}
        s.b[1702] = (2.0 == 2.0);s.store_scalar(1702, if s.b[1702] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) && (!s.b[1701])) && s.b[1702]) {s.store_scalar(720, 2.0);}
        s.b[1703] = (2.0 == 4.0);s.store_scalar(1703, if s.b[1703] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) && (!s.b[1701])) && (!s.b[1702])) && s.b[1703]) {s.store_scalar(720, 3.0);}
        s.b[1704] = (2.0 == 8.0);s.store_scalar(1704, if s.b[1704] { 1.0 } else { 0.0 });
        if ((((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) && (!s.b[1701])) && (!s.b[1702])) && (!s.b[1703])) && s.b[1704]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) {s.store_scalar(719, 0.0);}
        let mut t27: usize = 0;
        while {
            let t26: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t26 != 0.0
        } {
            t27 += 1;assert!(t27 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && (!s.b[1700])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.5);s.store_div_scaled_product_indices(334, 725, 726, 0.5, 770, 1.0);s.store_sub_from_scalar(1537, 0.5, 780);}
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) {
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && (!s.b[1699])) {s.copy_ad(1537, 85);s.store_scalar(334, 1.0);}
        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {s.store_scale(335, 1537, 0.8);}
        s.b[1705] = ((s.v[348] > (s.v[1537] - s.v[335])) && (s.v[335] >= 0.0));s.store_scalar(1705, if s.b[1705] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) {s.store_add_scaled_inputs3_indices(781, 348, 1.0, 1537, (-1.0), 335, 1.0);s.store_square(722, 781);s.store_square(723, 335);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1706] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1706, if s.b[1706] { 1.0 } else { 0.0 });s.b[1707] = (2.0 == 1.0);s.store_scalar(1707, if s.b[1707] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) && s.b[1707]) {s.store_scalar(720, 1.0);}
        s.b[1708] = (2.0 == 2.0);s.store_scalar(1708, if s.b[1708] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) && (!s.b[1707])) && s.b[1708]) {s.store_scalar(720, 2.0);}
        s.b[1709] = (2.0 == 4.0);s.store_scalar(1709, if s.b[1709] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) && (!s.b[1707])) && (!s.b[1708])) && s.b[1709]) {s.store_scalar(720, 3.0);}
        s.b[1710] = (2.0 == 8.0);s.store_scalar(1710, if s.b[1710] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_48(
        s: &mut ReactiveScratch,
    ) {
        if ((((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) && (!s.b[1707])) && (!s.b[1708])) && (!s.b[1709])) && s.b[1710]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) {s.store_scalar(719, 0.0);}
        let mut t29: usize = 0;
        while {
            let t28: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t28 != 0.0
        } {
            t29 += 1;assert!(t29 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && (!s.b[1706])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 335, 726);s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs3_indices(790, 1537, 1.0, 335, (-1.0), 780, 1.0);}
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) {
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && (!s.b[1705])) {s.copy_ad(790, 348);s.store_scalar(334, 1.0);}
        if ((s.b[1443] && s.b[1444]) && (!s.b[1686])) {s.copy_ad(348, 790);}
        s.b[1711] = (s.v[790] <= 0.0);s.store_scalar(1711, if s.b[1711] { 1.0 } else { 0.0 });
        if ((s.b[1443] && s.b[1444]) && s.b[1711]) {s.copy_ad(1462, 1461);s.copy_ad(1484, 1483);s.copy_ad(1465, 1464);s.copy_ad(1478, 1477);s.copy_ad(1539, 1538);s.copy_ad(1499, 1497);s.copy_ad(1500, 1498);s.copy_ad(1518, 1517);s.copy_ad(1516, 1515);}
        if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {s.store_sqrt_mul_ad(1454, A::div_scaled_product(s.ad_value(1547), s.ad_value(622), 1.0, A::add(s.ad_value(622), s.ad_value(964)), 1.0), A::add_scaled_inputs3(s.ad_value(790), 1.0, s.ad_value(1435), (-1.0), s.ad_value(1463), 1.0));}
        s.b[1712] = (s.v[1454] > s.v[965]);s.store_scalar(1712, if s.b[1712] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {s.copy_ad(1466, 790);s.copy_ad(1448, 965);s.copy_ad(1484, 790);s.copy_ad(1512, 790);s.store_sub_mixed_ia(1465, 1484, A::mul3(s.ad_value(1548), s.ad_value(1448), s.ad_value(1448)));s.copy_ad(1510, 1466);s.copy_ad(1473, 1465);s.store_mul(1499, 1448, 1546);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_49(
        s: &mut ReactiveScratch,
    ) {
        let mut t30: usize = 0;
        while {
            let t2e: f64 = (150.0 + 1.0);let t2f: f64 = if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (s.v[97] <= t2e)) { 1.0 } else { 0.0 };
            t2f != 0.0
        } {
            t30 += 1;assert!(t30 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {s.store_sqrt_mul_sub_rhs(1448, 1547, 1484, 1465);}
            s.b[1713] = ((s.v[1448] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1713, if s.b[1713] { 1.0 } else { 0.0 });
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) {s.store_offset_sub(781, 1448, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1714] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1714, if s.b[1714] { 1.0 } else { 0.0 });s.b[1715] = (2.0 == 1.0);s.store_scalar(1715, if s.b[1715] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) && s.b[1715]) {s.store_scalar(720, 1.0);}
            s.b[1716] = (2.0 == 2.0);s.store_scalar(1716, if s.b[1716] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) && (!s.b[1715])) && s.b[1716]) {s.store_scalar(720, 2.0);}
            s.b[1717] = (2.0 == 4.0);s.store_scalar(1717, if s.b[1717] { 1.0 } else { 0.0 });
            if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) && (!s.b[1715])) && (!s.b[1716])) && s.b[1717]) {s.store_scalar(720, 3.0);}
            s.b[1718] = (2.0 == 8.0);s.store_scalar(1718, if s.b[1718] { 1.0 } else { 0.0 });
            if (((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) && (!s.b[1715])) && (!s.b[1716])) && (!s.b[1717])) && s.b[1718]) {s.store_scalar(720, 4.0);}
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) {s.store_scalar(719, 0.0);}
            let mut t2b: usize = 0;
            while {
                let t2a: f64 = if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t2a != 0.0
            } {
                t2b += 1;assert!(t2b <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && (!s.b[1714])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(334, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1448, 965, (-1e-8), 780);}
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) {
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (!s.b[1713])) {
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (!s.b[1713])) {s.store_scalar(334, 1.0);}
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {s.store_add_scaled_inputs3_indices(335, 1465, 1.0, 1435, (-1.0), 1463, 1.0);}
            s.b[1719] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1719, if s.b[1719] { 1.0 } else { 0.0 });
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1720] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1720, if s.b[1720] { 1.0 } else { 0.0 });s.b[1721] = (2.0 == 1.0);s.store_scalar(1721, if s.b[1721] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) && s.b[1721]) {s.store_scalar(720, 1.0);}
            s.b[1722] = (2.0 == 2.0);s.store_scalar(1722, if s.b[1722] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) && (!s.b[1721])) && s.b[1722]) {s.store_scalar(720, 2.0);}
            s.b[1723] = (2.0 == 4.0);s.store_scalar(1723, if s.b[1723] { 1.0 } else { 0.0 });
            if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) && (!s.b[1721])) && (!s.b[1722])) && s.b[1723]) {s.store_scalar(720, 3.0);}
            s.b[1724] = (2.0 == 8.0);s.store_scalar(1724, if s.b[1724] { 1.0 } else { 0.0 });
            if (((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) && (!s.b[1721])) && (!s.b[1722])) && (!s.b[1723])) && s.b[1724]) {s.store_scalar(720, 4.0);}
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) {s.store_scalar(719, 0.0);}
            let mut t2d: usize = 0;
            while {
                let t2c: f64 = if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t2c != 0.0
            } {
                t2d += 1;assert!(t2d <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && (!s.b[1720])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(341, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) {
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (!s.b[1719])) {s.copy_ad(336, 335);s.store_scalar(341, 1.0);}
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {s.store_sqrt_mul(1452, 1550, 336);s.store_mul(1499, 1448, 1546);s.store_mul_div_from_scalar_lhs_ad_indices(1530, (-1.034943e-10), 1448, 334);s.store_mul_scale_offset_indices(1500, 1544, 1452, -1.0, 0.0);s.store_mul_div_from_scalar_lhs_ad_indices(1532, (-1.034943e-10), 1452, 341);s.store_add_mixed_ai(1485, A::add_scaled_product(s.ad_value(1499), 1.0, s.ad_value(185), A::sub(s.ad_value(1466), s.ad_value(1484)), 1.0), 1500);s.copy_ad(1487, 185);s.store_add(1488, 1530, 1532);s.store_add_scaled_product_mixed_iia(1486, 1465, 1.0, 1535, A::sub(A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1484), 1.0), s.ad_value(1463)), (-1.0));s.store_scalar(1489, 0.0);s.store_scalar(1490, 1.0);s.store_add_scaled_products_indices(1491, 1487, 1490, 1.0, 1489, 1488, (-1.0));s.store_div(1492, 1490, 1491);s.store_div_scaled_inputs_indices(1493, 1488, -1.0, 1491, 1.0);s.store_div_scaled_inputs_indices(1494, 1489, -1.0, 1491, 1.0);s.store_div(1495, 1487, 1491);}
            s.b[1725] = (((((s.v[1492] * s.v[1485]) + (s.v[1493] * s.v[1486]))) as f64).abs() > 0.5);s.store_scalar(1725, if s.b[1725] { 1.0 } else { 0.0 });
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1725]) {s.store_offset(1466, 1466, (-(0.5 * (if (((s.v[1492] * s.v[1485]) + (s.v[1493] * s.v[1486])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1725]) {s.store_offset(1465, 1465, (-(0.5 * (if (((s.v[1494] * s.v[1485]) + (s.v[1495] * s.v[1486])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (!s.b[1725])) {s.store_sub_mixed_ia(1466, 1466, A::add_scaled_products(s.ad_value(1492), s.ad_value(1485), 1.0, s.ad_value(1493), s.ad_value(1486), 1.0));s.store_sub_mixed_ia(1465, 1465, A::add_scaled_products(s.ad_value(1494), s.ad_value(1485), 1.0, s.ad_value(1495), s.ad_value(1486), 1.0));}
            s.b[1726] = (((((s.v[1466] - s.v[1510])) as f64).abs() <= 1e-12) && ((((s.v[1465] - s.v[1473])) as f64).abs() <= 1e-12));s.store_scalar(1726, if s.b[1726] { 1.0 } else { 0.0 });
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1726]) {s.store_scalar(97, (150.0 + 1.0));}
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {s.copy_ad(1510, 1466);s.copy_ad(1473, 1465);s.store_primal_offset(97, 97, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_50(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {s.copy_ad(1514, 1465);s.store_mul(1452, 965, 1536);s.store_add_scaled_inputs3_mixed_aii(1465, A::mul3(s.ad_value(1551), s.ad_value(1452), s.ad_value(1452)), 1.0, 1435, 1.0, 1463, -1.0);s.store_add_scaled_product_indices(1484, 1465, 1.0, 1548, 1543, 1.0);s.copy_ad(1462, 1484);s.copy_ad(1467, 1484);s.copy_ad(1509, 1484);}
        s.b[1727] = (s.v[85] > s.v[1466]);s.store_scalar(1727, if s.b[1727] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1727]) {s.store_scalar(1479, 1.0);}
        s.b[1728] = (s.v[85] > s.v[1509]);s.store_scalar(1728, if s.b[1728] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (!s.b[1727])) && s.b[1728]) {s.store_scalar(1479, 3.0);}
        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (!s.b[1727])) && (!s.b[1728])) {s.store_scalar(1479, 2.0);}
        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1712])) {s.copy_ad(1466, 790);s.copy_ad(1509, 1466);s.copy_ad(1467, 1466);s.copy_ad(1512, 1466);s.copy_ad(1448, 1454);s.store_mul(1452, 1448, 1536);s.store_add_scaled_inputs3_mixed_aii(1465, A::mul3(s.ad_value(1551), s.ad_value(1452), s.ad_value(1452)), 1.0, 1435, 1.0, 1463, -1.0);s.store_add_mixed_ai(1484, A::mul3(s.ad_value(1548), s.ad_value(1448), s.ad_value(1448)), 1465);s.copy_ad(1514, 1465);}
        s.b[1729] = (s.v[85] > s.v[1466]);s.store_scalar(1729, if s.b[1729] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1712])) && s.b[1729]) {s.store_scalar(1479, 1.0);}
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1712])) && (!s.b[1729])) {s.store_scalar(1479, 2.0);}
        if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {s.store_mul_add_scaled_inputs3_offset_rhs_indices(335, 1549, 1467, 1.0, 1435, -1.0, 961, 1.0, 0.0);}
        s.b[1730] = (s.v[335] > 0.0);s.store_scalar(1730, if s.b[1730] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1730]) {s.store_add_scaled_inputs3_mixed_iia(1455, 1435, 1.0, 961, (-1.0), A::div(A::sqrt(s.ad_value(335)), s.ad_value(185)), -1.0);}
        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1730])) {s.store_sub(1455, 1435, 961);}
        s.b[1731] = (s.v[85] > s.v[1466]);s.store_scalar(1731, if s.b[1731] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1731]) {s.copy_ad(1465, 1514);s.copy_ad(1484, 790);s.store_add_div_lhs(1481, A::ln(A::mul3(s.ad_value(1456), s.ad_value(85), s.ad_value(85))), A::add(s.ad_value(154), A::div_from_scalar(2.0, s.ad_value(85))), 790);}
        s.b[1732] = (s.v[1481] < (s.v[1512] + s.v[1553]));s.store_scalar(1732, if s.b[1732] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1731]) && s.b[1732]) {s.store_add(1481, 1512, 1553);}
        s.b[1733] = (s.v[85] > s.v[1509]);s.store_scalar(1733, if s.b[1733] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && s.b[1733]) {s.copy_ad(1481, 1462);}
        s.b[1734] = (s.v[85] > s.v[1455]);s.store_scalar(1734, if s.b[1734] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) {s.store_add_scaled_product_indices(1457, 154, 1.0, 1456, 85, (-2.0));s.store_add_scaled_product_mixed_aii(1458, A::mul3(s.ad_value(1456), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1484, (-1.0));s.copy_ad(1471, 1484);s.store_div_scaled_inputs2_mixed_aii(1481, A::sqrt(A::add_scaled_square_product(s.ad_value(1457), 1.0, s.ad_value(1456), s.ad_value(1458), (-4.0))), 0.5, 1457, (-0.5), 1456, 1.0);}
        s.b[1735] = (s.v[1481] > (s.v[1467] - s.v[1553]));s.store_scalar(1735, if s.b[1735] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1735]) {s.store_sub(1481, 1467, 1553);}
        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) {s.store_sqrt_mul_sub_rhs(1450, 1547, 1484, 1481);s.store_sqrt_mul_sub_rhs(1448, 1547, 1484, 1465);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_51(
        s: &mut ReactiveScratch,
    ) {
        s.b[1736] = ((s.v[1450] + s.v[1448]) > s.v[965]);s.store_scalar(1736, if s.b[1736] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) {s.store_scalar(97, 1.0);}
        let mut t33: usize = 0;
        while {
            let t31: f64 = (150.0 + 1.0);let t32: f64 = if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && (s.v[97] <= t31)) { 1.0 } else { 0.0 };
            t32 != 0.0
        } {
            t33 += 1;assert!(t33 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) {s.store_add_scaled_inputs3_indices(1468, 1450, 1.0, 1448, 1.0, 965, -1.0);s.store_add_ad(1508, A::div_scalar_by_product(1.034943e-10, s.ad_value(1546), s.ad_value(1450), 1.0), A::div(A::mul_sub_from_scalar_rhs(A::div_from_scalar(1.034943e-10, s.ad_value(1546)), 1.0, A::div_scaled_value_offset_denominator(s.ad_value(1536), 1.0, s.ad_value(1536), 1.0, 1.0)), s.ad_value(1448)));}
            s.b[1737] = ((((s.v[1468] / s.v[1508])) as f64).abs() > 0.5);s.store_scalar(1737, if s.b[1737] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1737]) {s.store_offset(1484, 1484, (-(0.5 * (if ((s.v[1468] / s.v[1508]) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && (!s.b[1737])) {s.store_sub_div_rhs_indices(1484, 1484, 1468, 1508);}
            s.b[1738] = (((s.v[1484] - s.v[1435]) + s.v[1463]) < (10.0 * 2.220446049250313e-16));s.store_scalar(1738, if s.b[1738] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1738]) {s.store_offset_sub(1484, 1435, 1463, (10.0 * 2.220446049250313e-16));}
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) {s.store_add_scaled_product_mixed_aii(1458, A::mul3(s.ad_value(1456), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1484, (-1.0));s.store_add_scaled_square_product_indices(335, 1457, 1.0, 1456, 1458, (-4.0));}
            s.b[1739] = (s.v[335] > 0.0);s.store_scalar(1739, if s.b[1739] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1739]) {s.store_div_scaled_inputs2_sqrt_first(1481, 335, 0.5, 1457, (-0.5), 1456, 1.0);}
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && (!s.b[1739])) {s.store_div_scaled_inputs_indices(1481, 1457, (-0.5), 1456, 1.0);}
            s.b[1740] = (s.v[1481] > s.v[1467]);s.store_scalar(1740, if s.b[1740] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1740]) {s.copy_ad(1481, 1467);}
            s.b[1741] = (s.v[1481] > s.v[1484]);s.store_scalar(1741, if s.b[1741] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1741]) {s.store_sub(1481, 1484, 1553);s.store_scalar(97, (150.0 + 1.0));}
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) {s.store_sqrt_mul_sub_rhs(1450, 1547, 1484, 1481);s.store_div_scaled_inputs2_mixed_aia(1465, A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1484), 1.0), 1.0, 1463, (-1.0), A::offset(s.ad_value(1536), 1.0), 1.0);s.store_sqrt_mul_sub_rhs(1448, 1547, 1484, 1465);}
            s.b[1742] = ((((s.v[1484] - s.v[1471])) as f64).abs() <= 1e-8);s.store_scalar(1742, if s.b[1742] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1742]) {s.store_scalar(97, (150.0 + 1.0));}
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) {s.copy_ad(1471, 1484);s.store_primal_offset(97, 97, 1.0);}
        }
        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && (!s.b[1734])) {s.copy_ad(1484, 1483);s.copy_ad(1465, 1464);s.copy_ad(1481, 1461);}
        if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {s.copy_ad(1482, 1484);s.store_scalar(79, 0.0);s.copy_ad(1462, 1481);s.copy_ad(1484, 1482);s.copy_ad(1474, 1462);s.copy_ad(1471, 1484);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_52(
        s: &mut ReactiveScratch,
    ) {
        let mut t3c: usize = 0;
        while {
            let t3a: f64 = (150.0 + 1.0);let t3b: f64 = if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (s.v[97] <= t3a)) { 1.0 } else { 0.0 };
            t3b != 0.0
        } {
            t3c += 1;assert!(t3c <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {s.store_mul_sub_mixed_iai(1465, 1535, A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1484), 1.0), 1463);s.store_mul(1534, 1535, 1536);s.store_sub(335, 1484, 1465);}
            s.b[1743] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1743, if s.b[1743] { 1.0 } else { 0.0 });
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1744] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1744, if s.b[1744] { 1.0 } else { 0.0 });s.b[1745] = (2.0 == 1.0);s.store_scalar(1745, if s.b[1745] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) && s.b[1745]) {s.store_scalar(720, 1.0);}
            s.b[1746] = (2.0 == 2.0);s.store_scalar(1746, if s.b[1746] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) && (!s.b[1745])) && s.b[1746]) {s.store_scalar(720, 2.0);}
            s.b[1747] = (2.0 == 4.0);s.store_scalar(1747, if s.b[1747] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) && (!s.b[1745])) && (!s.b[1746])) && s.b[1747]) {s.store_scalar(720, 3.0);}
            s.b[1748] = (2.0 == 8.0);s.store_scalar(1748, if s.b[1748] { 1.0 } else { 0.0 });
            if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) && (!s.b[1745])) && (!s.b[1746])) && (!s.b[1747])) && s.b[1748]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) {s.store_scalar(719, 0.0);}
            let mut t37: usize = 0;
            while {
                let t36: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t36 != 0.0
            } {
                t37 += 1;assert!(t37 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && (!s.b[1744])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) {
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1743])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
            if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {s.store_sqrt_mul(1448, 1547, 336);}
            s.b[1749] = ((s.v[1448] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1749, if s.b[1749] { 1.0 } else { 0.0 });
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) {s.store_offset_sub(781, 1448, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1750] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1750, if s.b[1750] { 1.0 } else { 0.0 });s.b[1751] = (2.0 == 1.0);s.store_scalar(1751, if s.b[1751] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) && s.b[1751]) {s.store_scalar(720, 1.0);}
            s.b[1752] = (2.0 == 2.0);s.store_scalar(1752, if s.b[1752] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) && (!s.b[1751])) && s.b[1752]) {s.store_scalar(720, 2.0);}
            s.b[1753] = (2.0 == 4.0);s.store_scalar(1753, if s.b[1753] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) && (!s.b[1751])) && (!s.b[1752])) && s.b[1753]) {s.store_scalar(720, 3.0);}
            s.b[1754] = (2.0 == 8.0);s.store_scalar(1754, if s.b[1754] { 1.0 } else { 0.0 });
            if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) && (!s.b[1751])) && (!s.b[1752])) && (!s.b[1753])) && s.b[1754]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) {s.store_scalar(719, 0.0);}
            let mut t39: usize = 0;
            while {
                let t38: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t38 != 0.0
            } {
                t39 += 1;assert!(t39 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && (!s.b[1750])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1448, 965, (-1e-8), 780);}
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) {
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1749])) {
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1749])) {s.store_scalar(337, 1.0);}
            if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {s.store_sqrt_mul_ad(1452, s.ad_value(1550), A::add_scaled_inputs3(s.ad_value(1465), 1.0, s.ad_value(1435), (-1.0), s.ad_value(1463), 1.0));s.store_mul(1499, 1448, 1546);s.store_mul_ad_product_lhs_mixed_ai(1528, A::div_from_scalar(1.034943e-10, s.ad_value(1448)), 334, 337);s.store_mul_ad_product_lhs_mixed_ai(1530, A::div_from_scalar((-1.034943e-10), s.ad_value(1448)), 334, 337);s.store_mul_scale_offset_indices(1500, 1544, 1452, -1.0, 0.0);s.store_div_from_scalar(1532, (-1.034943e-10), 1452);s.store_scaled_mul(335, 1502, 1543, 8.0);s.store_div_scaled_inputs_product_mixed_aaaii(1520, A::add_scaled_inputs4(A::mul3_scaled_output(s.ad_value(1465), s.ad_value(1465), s.ad_value(1542), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1465), s.ad_value(1542), s.ad_value(1462), 8.0), (-1.0), A::mul3_scaled_output(s.ad_value(1542), s.ad_value(1462), s.ad_value(1462), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1465), s.ad_value(1502), s.ad_value(1543), 4.0), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(1462), s.ad_value(1502), s.ad_value(1543), 4.0), 1.0, A::mul3(s.ad_value(1545), s.ad_value(1541), s.ad_value(1543)), 1543, 1.0, 335, 1.0);s.store_div_mixed_ai(1521, A::add_scaled_products3(s.ad_value(1465), s.ad_value(1542), (-8.0), s.ad_value(1542), s.ad_value(1462), (4.0 * 2.0), s.ad_value(1502), s.ad_value(1543), 4.0), 335);s.store_div_mixed_ai(1522, A::add_scaled_products3(s.ad_value(1465), s.ad_value(1542), (4.0 * 2.0), s.ad_value(1542), s.ad_value(1462), (-8.0), s.ad_value(1502), s.ad_value(1543), 4.0), 335);s.store_mul_sub_rhs(335, 154, 1462, 1484);s.store_exp(336, 335);}
            s.b[1755] = (s.v[1462] >= s.v[1484]);s.store_scalar(1755, if s.b[1755] { 1.0 } else { 0.0 });
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1755]) {s.store_mul_scaled_sqrt_ad_rhs(1476, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.store_mul_add_scaled_product_div_scaled_product(1524, 154, (-1.0), 154, 336, 1.0, 209, 209, 0.5, 1476, 1.0);s.store_neg(1526, 1524);}
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1755])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1462), s.ad_value(1435)));s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1484), s.ad_value(1435)));s.store_mul_sqrt_mixed_ia(1476, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1476, 1.0);s.store_mul_add_mixed_iaa(1524, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));s.store_mul_add_mixed_iaa(1526, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));}
            s.b[1756] = ((s.v[1520] > (s.v[1512] - s.v[1519])) && (s.v[1519] >= 0.0));s.store_scalar(1756, if s.b[1756] { 1.0 } else { 0.0 });
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) {s.store_add_scaled_inputs3_indices(781, 1520, 1.0, 1512, (-1.0), 1519, 1.0);s.store_square(722, 781);s.store_square(723, 1519);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1757] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1757, if s.b[1757] { 1.0 } else { 0.0 });s.b[1758] = (4.0 == 1.0);s.store_scalar(1758, if s.b[1758] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) && s.b[1758]) {s.store_scalar(720, 1.0);}
            s.b[1759] = (4.0 == 2.0);s.store_scalar(1759, if s.b[1759] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) && (!s.b[1758])) && s.b[1759]) {s.store_scalar(720, 2.0);}
            s.b[1760] = (4.0 == 4.0);s.store_scalar(1760, if s.b[1760] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) && (!s.b[1758])) && (!s.b[1759])) && s.b[1760]) {s.store_scalar(720, 3.0);}
            s.b[1761] = (4.0 == 8.0);s.store_scalar(1761, if s.b[1761] { 1.0 } else { 0.0 });
            if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) && (!s.b[1758])) && (!s.b[1759])) && (!s.b[1760])) && s.b[1761]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) {s.store_scalar(719, 0.0);}
            let mut t35: usize = 0;
            while {
                let t34: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t34 != 0.0
            } {
                t35 += 1;assert!(t35 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && (!s.b[1757])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1519, 726);s.store_div_scaled_product3_indices(334, 1519, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs3_indices(335, 1512, 1.0, 1519, (-1.0), 780, 1.0);}
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) {
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1756])) {s.copy_ad(335, 1520);s.store_scalar(334, 1.0);}
            if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {s.store_sub(1485, 1484, 335);s.store_mul_scale_offset_indices(1487, 334, 1521, -1.0, 0.0);s.store_sub_from_scalar_ad(1488, 1.0, A::mul3(s.ad_value(1522), s.ad_value(1534), s.ad_value(334)));s.store_add_scaled_inputs3_mixed_aii(1486, A::add_scaled_product(s.ad_value(1476), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1462)), 1.0), 1.0, 1499, 1.0, 1500, 1.0);s.store_sub(1489, 1524, 185);s.store_add_scaled_inputs_products_indices(1490, 1526, 1.0, 1528, 1.0, 1530, 1534, 1.0, 1532, 1534, 1.0);s.store_add_scaled_products_indices(1491, 1487, 1490, 1.0, 1489, 1488, (-1.0));s.store_div(1492, 1490, 1491);s.store_div_scaled_inputs_indices(1493, 1488, -1.0, 1491, 1.0);s.store_div_scaled_inputs_indices(1494, 1489, -1.0, 1491, 1.0);s.store_div(1495, 1487, 1491);}
            s.b[1762] = (((((s.v[1494] * s.v[1485]) + (s.v[1495] * s.v[1486]))) as f64).abs() > 0.5);s.store_scalar(1762, if s.b[1762] { 1.0 } else { 0.0 });
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1762]) {s.store_offset(1462, 1462, (-(0.5 * (if (((s.v[1492] * s.v[1485]) + (s.v[1493] * s.v[1486])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1762]) {s.store_offset(1484, 1484, (-(0.5 * (if (((s.v[1494] * s.v[1485]) + (s.v[1495] * s.v[1486])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1762])) {s.store_sub_mixed_ia(1462, 1462, A::add_scaled_products(s.ad_value(1492), s.ad_value(1485), 1.0, s.ad_value(1493), s.ad_value(1486), 1.0));s.store_sub_mixed_ia(1484, 1484, A::add_scaled_products(s.ad_value(1494), s.ad_value(1485), 1.0, s.ad_value(1495), s.ad_value(1486), 1.0));}
            s.b[1763] = (((((s.v[1462] - s.v[1474])) as f64).abs() <= 1e-12) && ((((s.v[1484] - s.v[1471])) as f64).abs() <= 1e-12));s.store_scalar(1763, if s.b[1763] { 1.0 } else { 0.0 });
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1763]) {s.store_scalar(97, (150.0 + 1.0));s.store_scalar(79, 1.0);}
            if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {s.copy_ad(1474, 1462);s.copy_ad(1471, 1484);s.store_primal_offset(97, 97, 1.0);}
        }
    }
}
