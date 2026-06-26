#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
    ) {
        let mut assign25290_loop_guard: usize = 0;
        while {
            let assign25290_cond_e21731: f64 = (150.0 + 1.0);
            let assign25290_cond_e21733: f64 = if ((s.b[1443] && s.b[1444]) && (s.v[97] <= assign25290_cond_e21731)) { 1.0 } else { 0.0 };
            assign25290_cond_e21733 != 0.0
        } {
            assign25290_loop_guard += 1;
            assert!(assign25290_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1443] && s.b[1444]) {
                s.store_mul_sub_ad_rhs(1464, 1535, A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1483), 1.0), s.ad_value(1463));
                s.store_mul(1533, 1535, 1536);
                s.store_sub(335, 1483, 1464);
            }
            s.b[1594] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1594] = if s.b[1594] { 1.0 } else { 0.0 };
            if ((s.b[1443] && s.b[1444]) && s.b[1594]) {
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
            s.b[1595] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1595] = if s.b[1595] { 1.0 } else { 0.0 };
            s.b[1596] = (2.0 == 1.0);
            s.v[1596] = if s.b[1596] { 1.0 } else { 0.0 };
            if ((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && s.b[1596]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1597] = (2.0 == 2.0);
            s.v[1597] = if s.b[1597] { 1.0 } else { 0.0 };
            if (((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && s.b[1597]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1598] = (2.0 == 4.0);
            s.v[1598] = if s.b[1598] { 1.0 } else { 0.0 };
            if ((((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && (!s.b[1597])) && s.b[1598]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1599] = (2.0 == 8.0);
            s.v[1599] = if s.b[1599] { 1.0 } else { 0.0 };
            if (((((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && (!s.b[1597])) && (!s.b[1598])) && s.b[1599]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign25290_body29_loop_guard: usize = 0;
            while {
                let assign25290_body29_cond_e22024: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign25290_body29_cond_e22024 != 0.0
            } {
                assign25290_body29_loop_guard += 1;
                assert!(assign25290_body29_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1594]) && (!s.b[1595])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((s.b[1443] && s.b[1444]) && s.b[1594]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if ((s.b[1443] && s.b[1444]) && s.b[1594]) {
            }
            if ((s.b[1443] && s.b[1444]) && (!s.b[1594])) {
                s.copy_ad(336, 335);
                s.store_scalar(334, 1.0);
            }
            if (s.b[1443] && s.b[1444]) {
                s.store_sqrt_mul(1447, 1547, 336);
            }
            s.b[1600] = ((s.v[1447] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.v[1600] = if s.b[1600] { 1.0 } else { 0.0 };
            if ((s.b[1443] && s.b[1444]) && s.b[1600]) {
                s.store_offset_sub(781, 1447, 965, 1e-8);
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
            s.b[1601] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1601] = if s.b[1601] { 1.0 } else { 0.0 };
            s.b[1602] = (2.0 == 1.0);
            s.v[1602] = if s.b[1602] { 1.0 } else { 0.0 };
            if ((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && s.b[1602]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1603] = (2.0 == 2.0);
            s.v[1603] = if s.b[1603] { 1.0 } else { 0.0 };
            if (((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && (!s.b[1602])) && s.b[1603]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1604] = (2.0 == 4.0);
            s.v[1604] = if s.b[1604] { 1.0 } else { 0.0 };
            if ((((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && (!s.b[1602])) && (!s.b[1603])) && s.b[1604]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1605] = (2.0 == 8.0);
            s.v[1605] = if s.b[1605] { 1.0 } else { 0.0 };
            if (((((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && (!s.b[1602])) && (!s.b[1603])) && (!s.b[1604])) && s.b[1605]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign25290_body65_loop_guard: usize = 0;
            while {
                let assign25290_body65_cond_e22413: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign25290_body65_cond_e22413 != 0.0
            } {
                assign25290_body65_loop_guard += 1;
                assert!(assign25290_body65_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1600]) && (!s.b[1601])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((s.b[1443] && s.b[1444]) && s.b[1600]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
                s.store_add_ad_lhs(1447, A::offset(s.ad_value(965), (-1e-8)), 780);
            }
            if ((s.b[1443] && s.b[1444]) && s.b[1600]) {
            }
            if ((s.b[1443] && s.b[1444]) && (!s.b[1600])) {
            }
            if ((s.b[1443] && s.b[1444]) && (!s.b[1600])) {
                s.store_scalar(337, 1.0);
            }
            if (s.b[1443] && s.b[1444]) {
                s.store_sqrt_mul_ad(1451, s.ad_value(1550), A::add_scaled_inputs3(s.ad_value(1464), 1.0, s.ad_value(1435), (-1.0), s.ad_value(1463), 1.0));
                s.store_mul(1497, 1447, 1546);
                s.store_mul_ad_product_lhs(1527, A::div_from_scalar(1.034943e-10, s.ad_value(1447)), s.ad_value(334), 337);
                s.store_mul_ad_product_lhs(1529, A::div_from_scalar((-1.034943e-10), s.ad_value(1447)), s.ad_value(334), 337);
                s.store_mul_neg_lhs(1498, 1451, 1544);
                s.store_div_from_scalar(1531, (-1.034943e-10), 1451);
                s.store_scaled_mul(335, 1502, 1543, 8.0);
            }
            if (s.b[1443] && s.b[1444]) {
                let assign25290_body81_ad_e22648: A = A::add(A::add_scaled_inputs4(A::mul3_scaled_output(s.ad_value(1464), s.ad_value(1464), s.ad_value(1542), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1464), s.ad_value(1542), s.ad_value(1461), 8.0), (-1.0), A::mul3_scaled_output(s.ad_value(1542), s.ad_value(1461), s.ad_value(1461), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1464), s.ad_value(1502), s.ad_value(1543), 4.0), 1.0), A::mul3_scaled_output(s.ad_value(1461), s.ad_value(1502), s.ad_value(1543), 4.0));
                s.store_div_scaled_add_product(1520, assign25290_body81_ad_e22648, 1.0, A::mul3(s.ad_value(1545), s.ad_value(1541), s.ad_value(1543)), s.ad_value(1543), 1.0, s.ad_value(335), 1.0);
            }
            if (s.b[1443] && s.b[1444]) {
                s.store_div_ad_lhs(1521, A::add_scaled_products3(s.ad_value(1464), s.ad_value(1542), (-8.0), s.ad_value(1542), s.ad_value(1461), (4.0 * 2.0), s.ad_value(1502), s.ad_value(1543), 4.0), 335);
                s.store_div_ad_lhs(1522, A::add_scaled_products3(s.ad_value(1464), s.ad_value(1542), (4.0 * 2.0), s.ad_value(1542), s.ad_value(1461), (-8.0), s.ad_value(1502), s.ad_value(1543), 4.0), 335);
                s.store_mul_sub_rhs(335, 154, 1461, 1483);
                s.store_exp(336, 335);
            }
            s.b[1606] = (s.v[1461] >= s.v[1483]);
            s.v[1606] = if s.b[1606] { 1.0 } else { 0.0 };
            if ((s.b[1443] && s.b[1444]) && s.b[1606]) {
                s.store_mul_scaled_ad_rhs(1475, 209, -1.0, A::sqrt(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15)));
                s.store_mul_ad(1523, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(1475), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
                s.store_neg(1525, 1523);
            }
            if ((s.b[1443] && s.b[1444]) && (!s.b[1606])) {
                s.store_exp_ad(337, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1461), s.ad_value(1435))));
                s.store_exp_ad(338, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1483), s.ad_value(1435))));
                s.store_mul_sqrt_ad_rhs(1475, 209, A::add_scaled_product(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15), 1.0, s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1475, 1.0);
                s.store_mul_add_ad_rhs(1523, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
                s.store_mul_add_ad_rhs(1525, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));
            }
            s.b[1607] = ((s.v[1520] > (s.v[1511] - s.v[1519])) && (s.v[1519] >= 0.0));
            s.v[1607] = if s.b[1607] { 1.0 } else { 0.0 };
            if ((s.b[1443] && s.b[1444]) && s.b[1607]) {
                s.store_add_scaled_inputs3(781, s.ad_value(1520), 1.0, s.ad_value(1511), (-1.0), s.ad_value(1519), 1.0);
                s.store_square(722, 781);
                s.store_square(723, 1519);
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
            s.b[1608] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.v[1608] = if s.b[1608] { 1.0 } else { 0.0 };
            s.b[1609] = (4.0 == 1.0);
            s.v[1609] = if s.b[1609] { 1.0 } else { 0.0 };
            if ((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && s.b[1609]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1610] = (4.0 == 2.0);
            s.v[1610] = if s.b[1610] { 1.0 } else { 0.0 };
            if (((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && (!s.b[1609])) && s.b[1610]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1611] = (4.0 == 4.0);
            s.v[1611] = if s.b[1611] { 1.0 } else { 0.0 };
            if ((((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && (!s.b[1609])) && (!s.b[1610])) && s.b[1611]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1612] = (4.0 == 8.0);
            s.v[1612] = if s.b[1612] { 1.0 } else { 0.0 };
            if (((((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && (!s.b[1609])) && (!s.b[1610])) && (!s.b[1611])) && s.b[1612]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign25290_body126_loop_guard: usize = 0;
            while {
                let assign25290_body126_cond_e23194: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign25290_body126_cond_e23194 != 0.0
            } {
                assign25290_body126_loop_guard += 1;
                assert!(assign25290_body126_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1607]) && (!s.b[1608])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                    }
                });
            }
            if ((s.b[1443] && s.b[1444]) && s.b[1607]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_mul3_lhs(780, 781, 1519, 726);
                s.store_div_scaled_product3(334, s.ad_value(1519), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0);
                s.store_add_scaled_inputs3(335, s.ad_value(1511), 1.0, s.ad_value(1519), (-1.0), s.ad_value(780), 1.0);
            }
            if ((s.b[1443] && s.b[1444]) && s.b[1607]) {
            }
            if ((s.b[1443] && s.b[1444]) && (!s.b[1607])) {
                s.copy_ad(335, 1520);
                s.store_scalar(334, 1.0);
            }
            if (s.b[1443] && s.b[1444]) {
                s.store_sub(1485, 1483, 335);
                s.store_mul_neg_lhs(1487, 1521, 334);
                s.store_sub_from_scalar_ad(1488, 1.0, A::mul3(s.ad_value(1522), s.ad_value(1533), s.ad_value(334)));
                s.store_add_scaled_inputs3(1486, A::add_scaled_product(s.ad_value(1475), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1461)), 1.0), 1.0, s.ad_value(1497), 1.0, s.ad_value(1498), 1.0);
                s.store_sub(1489, 1523, 185);
                s.store_ad_value(1490, A::add_scaled_inputs_products(s.ad_value(1525), 1.0, s.ad_value(1527), 1.0, s.ad_value(1529), s.ad_value(1533), 1.0, s.ad_value(1531), s.ad_value(1533), 1.0));
                s.store_add_scaled_products_indices(1491, 1487, 1490, 1.0, 1489, 1488, (-1.0));
                s.store_div(1492, 1490, 1491);
                s.store_div_scaled_inputs(1493, s.ad_value(1488), -1.0, s.ad_value(1491), 1.0);
                s.store_div_scaled_inputs(1494, s.ad_value(1489), -1.0, s.ad_value(1491), 1.0);
                s.store_div(1495, 1487, 1491);
            }
            s.b[1613] = (((((s.v[1494] * s.v[1485]) + (s.v[1495] * s.v[1486]))) as f64).abs() > 0.5);
            s.v[1613] = if s.b[1613] { 1.0 } else { 0.0 };
            if ((s.b[1443] && s.b[1444]) && s.b[1613]) {
                s.store_offset(1461, 1461, (-(0.5 * (if (((s.v[1492] * s.v[1485]) + (s.v[1493] * s.v[1486])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if ((s.b[1443] && s.b[1444]) && s.b[1613]) {
                s.store_offset(1483, 1483, (-(0.5 * (if (((s.v[1494] * s.v[1485]) + (s.v[1495] * s.v[1486])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if ((s.b[1443] && s.b[1444]) && (!s.b[1613])) {
                s.store_sub_ad_rhs(1461, 1461, A::add_scaled_products(s.ad_value(1492), s.ad_value(1485), 1.0, s.ad_value(1493), s.ad_value(1486), 1.0));
                s.store_sub_ad_rhs(1483, 1483, A::add_scaled_products(s.ad_value(1494), s.ad_value(1485), 1.0, s.ad_value(1495), s.ad_value(1486), 1.0));
            }
            s.b[1614] = (((((s.v[1461] - s.v[1469])) as f64).abs() <= 1e-12) && ((((s.v[1483] - s.v[1470])) as f64).abs() <= 1e-12));
            s.v[1614] = if s.b[1614] { 1.0 } else { 0.0 };
            if ((s.b[1443] && s.b[1444]) && s.b[1614]) {
                s.store_scalar(97, (150.0 + 1.0));
                s.store_scalar(79, 1.0);
            }
            if (s.b[1443] && s.b[1444]) {
                s.copy_ad(1469, 1461);
                s.copy_ad(1470, 1483);
                s.store_offset(97, 97, 1.0);
            }
        }

        s.b[1616] = ((s.v[1453] > s.v[965]) && (s.v[1479] != 2.0));
        s.v[1616] = if s.b[1616] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
    ) {
        s.b[1617] = ((s.v[1483] > (s.v[1461] - 0.02)) && (0.02 >= 0.0));
        s.v[1617] = if s.b[1617] { 1.0 } else { 0.0 };

        if (((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) {
            s.store_offset_sub(781, 1483, 1461, 0.02);
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

        s.b[1618] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1618] = if s.b[1618] { 1.0 } else { 0.0 };

        s.b[1619] = (2.0 == 1.0);
        s.v[1619] = if s.b[1619] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && s.b[1619]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1620] = (2.0 == 2.0);
        s.v[1620] = if s.b[1620] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1620]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1621] = (2.0 == 4.0);
        s.v[1621] = if s.b[1621] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && (!s.b[1620])) && s.b[1621]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1622] = (2.0 == 8.0);
        s.v[1622] = if s.b[1622] { 1.0 } else { 0.0 };

        if ((((((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && (!s.b[1620])) && (!s.b[1621])) && s.b[1622]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign25580_loop_guard: usize = 0;
        while {
            let assign25580_cond_e23885: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign25580_cond_e23885 != 0.0
        } {
            assign25580_loop_guard += 1;
            assert!(assign25580_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && (!s.b[1618])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.02);
            s.store_div_scaled_product_indices(335, 725, 726, 0.02, 770, 1.0);
            s.store_add_ad_lhs(1483, A::offset(s.ad_value(1461), (-0.02)), 780);
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) {
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1616]) && (!s.b[1617])) {
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1616]) && (!s.b[1617])) {
            s.store_scalar(335, 1.0);
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_mul_sub_ad_rhs(1464, 1535, A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1483), 1.0), s.ad_value(1463));
            s.store_mul_sub_rhs(335, 154, 1461, 1483);
            s.store_exp(336, 335);
        }

        s.b[1623] = (s.v[1461] >= s.v[1483]);
        s.v[1623] = if s.b[1623] { 1.0 } else { 0.0 };

        if ((s.b[1443] && s.b[1444]) && s.b[1623]) {
            s.store_mul_scaled_ad_rhs(1475, 209, -1.0, A::sqrt(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15)));
            s.copy_ad(1538, 1475);
            s.store_scalar(1517, 0.0);
            s.store_scalar(1477, 0.0);
            s.store_sqrt_mul_ad(1447, s.ad_value(1547), A::sub(s.ad_value(1483), s.ad_value(1464)));
        }

        s.b[1624] = ((s.v[1447] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
        s.v[1624] = if s.b[1624] { 1.0 } else { 0.0 };

        if (((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) {
            s.store_offset_sub(781, 1447, 965, 1e-8);
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

        s.b[1625] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1625] = if s.b[1625] { 1.0 } else { 0.0 };

        s.b[1626] = (2.0 == 1.0);
        s.v[1626] = if s.b[1626] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && s.b[1626]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1627] = (2.0 == 2.0);
        s.v[1627] = if s.b[1627] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && (!s.b[1626])) && s.b[1627]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1628] = (2.0 == 4.0);
        s.v[1628] = if s.b[1628] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && (!s.b[1626])) && (!s.b[1627])) && s.b[1628]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1629] = (2.0 == 8.0);
        s.v[1629] = if s.b[1629] { 1.0 } else { 0.0 };

        if ((((((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && (!s.b[1626])) && (!s.b[1627])) && (!s.b[1628])) && s.b[1629]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign26020_loop_guard: usize = 0;
        while {
            let assign26020_cond_e24416: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign26020_cond_e24416 != 0.0
        } {
            assign26020_loop_guard += 1;
            assert!(assign26020_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && (!s.b[1625])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
            s.store_add_ad_lhs(1447, A::offset(s.ad_value(965), (-1e-8)), 780);
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) {
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1623]) && (!s.b[1624])) {
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1623]) && (!s.b[1624])) {
            s.store_scalar(337, 1.0);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1623]) {
            s.store_sqrt_mul_ad(1451, s.ad_value(1550), A::add_scaled_inputs3(s.ad_value(1464), 1.0, s.ad_value(1435), (-1.0), s.ad_value(1463), 1.0));
            s.store_mul(1497, 1447, 1546);
            s.store_mul_neg_lhs(1498, 1451, 1544);
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1623])) {
            s.store_exp_ad(337, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1461), s.ad_value(1435))));
            s.store_exp_ad(338, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1483), s.ad_value(1435))));
            s.store_mul_sqrt_ad_rhs(1475, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
        }

        s.b[1630] = ((s.v[1453] > s.v[965]) && (s.v[1479] != 2.0));
        s.v[1630] = if s.b[1630] { 1.0 } else { 0.0 };

        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1630]) {
            s.store_scalar(1477, 0.0);
            s.store_scalar(1517, 0.0);
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && (!s.b[1630])) {
            s.store_mul_sqrt_ad_rhs(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1461), s.ad_value(1435)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1483), s.ad_value(1435)))))), s.ad_value(335)));
            s.store_add_scaled_product(1477, s.ad_value(337), 1.0, s.ad_value(209), A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));
            s.store_mul_sqrt_ad_rhs(1517, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1623])) {
            s.store_scalar(1538, 0.0);
            s.store_sub(335, 1483, 1464);
        }

        s.b[1631] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
        s.v[1631] = if s.b[1631] { 1.0 } else { 0.0 };

        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) {
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

        s.b[1632] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1632] = if s.b[1632] { 1.0 } else { 0.0 };

        s.b[1633] = (2.0 == 1.0);
        s.v[1633] = if s.b[1633] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && s.b[1633]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1634] = (2.0 == 2.0);
        s.v[1634] = if s.b[1634] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && (!s.b[1633])) && s.b[1634]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1635] = (2.0 == 4.0);
        s.v[1635] = if s.b[1635] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && (!s.b[1633])) && (!s.b[1634])) && s.b[1635]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1636] = (2.0 == 8.0);
        s.v[1636] = if s.b[1636] { 1.0 } else { 0.0 };

        if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && (!s.b[1633])) && (!s.b[1634])) && (!s.b[1635])) && s.b[1636]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign26510_loop_guard: usize = 0;
        while {
            let assign26510_cond_e25091: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign26510_cond_e25091 != 0.0
        } {
            assign26510_loop_guard += 1;
            assert!(assign26510_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && (!s.b[1632])) {
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
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_sub_from_scalar(336, 0.1, 780);
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) {
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && (!s.b[1631])) {
            s.copy_ad(336, 335);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1623])) {
            s.store_sqrt_mul(1447, 1547, 336);
        }

        s.b[1637] = ((s.v[1447] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
        s.v[1637] = if s.b[1637] { 1.0 } else { 0.0 };

        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) {
            s.store_offset_sub(781, 1447, 965, 1e-8);
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

        s.b[1638] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1638] = if s.b[1638] { 1.0 } else { 0.0 };

        s.b[1639] = (2.0 == 1.0);
        s.v[1639] = if s.b[1639] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && s.b[1639]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1640] = (2.0 == 2.0);
        s.v[1640] = if s.b[1640] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && (!s.b[1639])) && s.b[1640]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1641] = (2.0 == 4.0);
        s.v[1641] = if s.b[1641] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && (!s.b[1639])) && (!s.b[1640])) && s.b[1641]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1642] = (2.0 == 8.0);
        s.v[1642] = if s.b[1642] { 1.0 } else { 0.0 };

        if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && (!s.b[1639])) && (!s.b[1640])) && (!s.b[1641])) && s.b[1642]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign26870_loop_guard: usize = 0;
        while {
            let assign26870_cond_e25576: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign26870_cond_e25576 != 0.0
        } {
            assign26870_loop_guard += 1;
            assert!(assign26870_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && (!s.b[1638])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
            s.store_add_ad_lhs(1447, A::offset(s.ad_value(965), (-1e-8)), 780);
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) {
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && (!s.b[1637])) {
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && (!s.b[1637])) {
            s.store_scalar(337, 1.0);
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1623])) {
            s.store_sqrt_mul_ad(1451, s.ad_value(1550), A::add_scaled_inputs3(s.ad_value(1464), 1.0, s.ad_value(1435), (-1.0), s.ad_value(1463), 1.0));
            s.store_mul(1497, 1447, 1546);
            s.store_mul_neg_lhs(1498, 1451, 1544);
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_sub(335, 1483, 1464);
        }

        s.b[1643] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
        s.v[1643] = if s.b[1643] { 1.0 } else { 0.0 };

        if ((s.b[1443] && s.b[1444]) && s.b[1643]) {
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

        s.b[1644] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1644] = if s.b[1644] { 1.0 } else { 0.0 };

        s.b[1645] = (2.0 == 1.0);
        s.v[1645] = if s.b[1645] { 1.0 } else { 0.0 };

        if ((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && s.b[1645]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1646] = (2.0 == 2.0);
        s.v[1646] = if s.b[1646] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && (!s.b[1645])) && s.b[1646]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1647] = (2.0 == 4.0);
        s.v[1647] = if s.b[1647] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && (!s.b[1645])) && (!s.b[1646])) && s.b[1647]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1648] = (2.0 == 8.0);
        s.v[1648] = if s.b[1648] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && (!s.b[1645])) && (!s.b[1646])) && (!s.b[1647])) && s.b[1648]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign27260_loop_guard: usize = 0;
        while {
            let assign27260_cond_e26033: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign27260_cond_e26033 != 0.0
        } {
            assign27260_loop_guard += 1;
            assert!(assign27260_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1643]) && (!s.b[1644])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1643]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_sub_from_scalar(336, 0.1, 780);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1643]) {
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1643])) {
            s.copy_ad(336, 335);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_sqrt_mul(1447, 1547, 336);
        }

        s.b[1649] = ((s.v[1447] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
        s.v[1649] = if s.b[1649] { 1.0 } else { 0.0 };

        if ((s.b[1443] && s.b[1444]) && s.b[1649]) {
            s.store_offset_sub(781, 1447, 965, 1e-8);
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

        s.b[1650] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1650] = if s.b[1650] { 1.0 } else { 0.0 };

        s.b[1651] = (2.0 == 1.0);
        s.v[1651] = if s.b[1651] { 1.0 } else { 0.0 };

        if ((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && s.b[1651]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1652] = (2.0 == 2.0);
        s.v[1652] = if s.b[1652] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && (!s.b[1651])) && s.b[1652]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1653] = (2.0 == 4.0);
        s.v[1653] = if s.b[1653] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && (!s.b[1651])) && (!s.b[1652])) && s.b[1653]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1654] = (2.0 == 8.0);
        s.v[1654] = if s.b[1654] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && (!s.b[1651])) && (!s.b[1652])) && (!s.b[1653])) && s.b[1654]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign27620_loop_guard: usize = 0;
        while {
            let assign27620_cond_e26422: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign27620_cond_e26422 != 0.0
        } {
            assign27620_loop_guard += 1;
            assert!(assign27620_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1649]) && (!s.b[1650])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1649]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
            s.store_add_ad_lhs(1447, A::offset(s.ad_value(965), (-1e-8)), 780);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1649]) {
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1649])) {
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1649])) {
            s.store_scalar(337, 1.0);
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_sub(335, 1483, 1461);
        }

        s.b[1655] = ((s.v[335] < 0.05) && (0.05 >= 0.0));
        s.v[1655] = if s.b[1655] { 1.0 } else { 0.0 };

        if ((s.b[1443] && s.b[1444]) && s.b[1655]) {
            s.store_sub_from_scalar(781, 0.05, 335);
        }

    }

    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
    ) {
        if ((s.b[1443] && s.b[1444]) && s.b[1655]) {
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

        s.b[1656] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1656] = if s.b[1656] { 1.0 } else { 0.0 };

        s.b[1657] = (2.0 == 1.0);
        s.v[1657] = if s.b[1657] { 1.0 } else { 0.0 };

        if ((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && s.b[1657]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1658] = (2.0 == 2.0);
        s.v[1658] = if s.b[1658] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && (!s.b[1657])) && s.b[1658]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1659] = (2.0 == 4.0);
        s.v[1659] = if s.b[1659] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && (!s.b[1657])) && (!s.b[1658])) && s.b[1659]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1660] = (2.0 == 8.0);
        s.v[1660] = if s.b[1660] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && (!s.b[1657])) && (!s.b[1658])) && (!s.b[1659])) && s.b[1660]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign27980_loop_guard: usize = 0;
        while {
            let assign27980_cond_e26810: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign27980_cond_e26810 != 0.0
        } {
            assign27980_loop_guard += 1;
            assert!(assign27980_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1655]) && (!s.b[1656])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1655]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.05);
            s.store_div_scaled_product_indices(334, 725, 726, 0.05, 770, 1.0);
            s.store_sub_from_scalar(336, 0.05, 780);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1655]) {
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1655])) {
            s.copy_ad(336, 335);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_sqrt_mul(1449, 1547, 336);
            s.store_add_scaled_inputs3(335, s.ad_value(965), 1.0, s.ad_value(1447), (-1.0), s.ad_value(1449), -1.0);
        }

        s.b[1661] = ((s.v[335] < (1e-25 + 1e-18)) && (1e-18 >= 0.0));
        s.v[1661] = if s.b[1661] { 1.0 } else { 0.0 };

        if ((s.b[1443] && s.b[1444]) && s.b[1661]) {
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

        s.b[1662] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1662] = if s.b[1662] { 1.0 } else { 0.0 };

        s.b[1663] = (2.0 == 1.0);
        s.v[1663] = if s.b[1663] { 1.0 } else { 0.0 };

        if ((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && s.b[1663]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1664] = (2.0 == 2.0);
        s.v[1664] = if s.b[1664] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1664]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1665] = (2.0 == 4.0);
        s.v[1665] = if s.b[1665] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && (!s.b[1664])) && s.b[1665]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1666] = (2.0 == 8.0);
        s.v[1666] = if s.b[1666] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && (!s.b[1664])) && (!s.b[1665])) && s.b[1666]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign28350_loop_guard: usize = 0;
        while {
            let assign28350_cond_e27209: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign28350_cond_e27209 != 0.0
        } {
            assign28350_loop_guard += 1;
            assert!(assign28350_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1661]) && (!s.b[1662])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1661]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-18);
            s.store_div_scaled_product_indices(334, 725, 726, 1e-18, 770, 1.0);
            s.store_sub_from_scalar(1501, (1e-25 + 1e-18), 780);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1661]) {
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1661])) {
            s.copy_ad(1501, 335);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_mul_neg_lhs(1496, 1501, 1546);
        }

        s.b[1667] = ((s.v[1453] > s.v[965]) && (s.v[1479] != 2.0));
        s.v[1667] = if s.b[1667] { 1.0 } else { 0.0 };

        s.b[1668] = ((s.v[1461] > (s.v[1511] - 0.8)) && (0.8 >= 0.0));
        s.v[1668] = if s.b[1668] { 1.0 } else { 0.0 };

        if (((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) {
            s.store_offset_sub(781, 1461, 1511, 0.8);
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

        s.b[1669] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1669] = if s.b[1669] { 1.0 } else { 0.0 };

        s.b[1670] = (2.0 == 1.0);
        s.v[1670] = if s.b[1670] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && s.b[1670]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1671] = (2.0 == 2.0);
        s.v[1671] = if s.b[1671] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && (!s.b[1670])) && s.b[1671]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1672] = (2.0 == 4.0);
        s.v[1672] = if s.b[1672] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && (!s.b[1670])) && (!s.b[1671])) && s.b[1672]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1673] = (2.0 == 8.0);
        s.v[1673] = if s.b[1673] { 1.0 } else { 0.0 };

        if ((((((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && (!s.b[1670])) && (!s.b[1671])) && (!s.b[1672])) && s.b[1673]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign28720_loop_guard: usize = 0;
        while {
            let assign28720_cond_e27647: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign28720_cond_e27647 != 0.0
        } {
            assign28720_loop_guard += 1;
            assert!(assign28720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && (!s.b[1669])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.8);
            s.store_div_scaled_product_indices(335, 725, 726, 0.8, 770, 1.0);
            s.store_add_ad_lhs(336, A::offset(s.ad_value(1511), (-0.8)), 780);
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) {
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1667]) && (!s.b[1668])) {
            s.copy_ad(336, 1461);
            s.store_scalar(335, 1.0);
        }

        s.b[1674] = ((s.v[1520] > (s.v[1511] - 0.8)) && (0.8 >= 0.0));
        s.v[1674] = if s.b[1674] { 1.0 } else { 0.0 };

        if (((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) {
            s.store_offset_sub(781, 1520, 1511, 0.8);
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
        if (((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) {
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1675] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1675] = if s.b[1675] { 1.0 } else { 0.0 };

        s.b[1676] = (2.0 == 1.0);
        s.v[1676] = if s.b[1676] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) && s.b[1676]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1677] = (2.0 == 2.0);
        s.v[1677] = if s.b[1677] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) && (!s.b[1676])) && s.b[1677]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1678] = (2.0 == 4.0);
        s.v[1678] = if s.b[1678] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) && (!s.b[1676])) && (!s.b[1677])) && s.b[1678]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1679] = (2.0 == 8.0);
        s.v[1679] = if s.b[1679] { 1.0 } else { 0.0 };

        if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) && (!s.b[1676])) && (!s.b[1677])) && (!s.b[1678])) && s.b[1679]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign29070_loop_guard: usize = 0;
        while {
            let assign29070_cond_e28110: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign29070_cond_e28110 != 0.0
        } {
            assign29070_loop_guard += 1;
            assert!(assign29070_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && (!s.b[1675])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.8);
            s.store_div_scaled_product_indices(334, 725, 726, 0.8, 770, 1.0);
            s.store_add_ad_lhs(336, A::offset(s.ad_value(1511), (-0.8)), 780);
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) {
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1667])) && (!s.b[1674])) {
            s.copy_ad(336, 1520);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_mul_ad_affine_product_lhs(1505, s.ad_value(964), A::exp(A::mul(s.ad_value(154), A::sub(s.ad_value(336), s.ad_value(1511)))), (-1.6021918e-19), 0.0, 1447);
        }

        s.b[1680] = (((s.v[1461] - s.v[1511]) < 0.06) && (0.06 >= 0.0));
        s.v[1680] = if s.b[1680] { 1.0 } else { 0.0 };

        if ((s.b[1443] && s.b[1444]) && s.b[1680]) {
            s.store_sub_from_scalar_ad(781, 0.06, A::sub(s.ad_value(1461), s.ad_value(1511)));
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

        s.b[1681] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1681] = if s.b[1681] { 1.0 } else { 0.0 };

        s.b[1682] = (2.0 == 1.0);
        s.v[1682] = if s.b[1682] { 1.0 } else { 0.0 };

        if ((((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) && s.b[1682]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1683] = (2.0 == 2.0);
        s.v[1683] = if s.b[1683] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) && (!s.b[1682])) && s.b[1683]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1684] = (2.0 == 4.0);
        s.v[1684] = if s.b[1684] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) && (!s.b[1682])) && (!s.b[1683])) && s.b[1684]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1685] = (2.0 == 8.0);
        s.v[1685] = if s.b[1685] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) && (!s.b[1682])) && (!s.b[1683])) && (!s.b[1684])) && s.b[1685]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign29430_loop_guard: usize = 0;
        while {
            let assign29430_cond_e28542: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign29430_cond_e28542 != 0.0
        } {
            assign29430_loop_guard += 1;
            assert!(assign29430_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && s.b[1680]) && s.b[1681]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1680]) && (!s.b[1681])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1680]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.06);
            s.store_div_scaled_product_indices(334, 725, 726, 0.06, 770, 1.0);
            s.store_sub_from_scalar(336, 0.06, 780);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1680]) {
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1680])) {
            s.store_sub(336, 1461, 1511);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_offset_ad(338, A::add_scaled_product(A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0)), (10.0 * 2.220446049250313e-16));
            s.store_mul_scaled_ad_rhs(1515, 209, -1.0, A::sqrt(s.ad_value(338)));
            s.store_sub_scaled_ad_lhs(338, A::offset(A::exp_scaled_input(s.ad_value(154), 0.1), (-1.0)), 154, 0.1);
            s.store_mul_sqrt_rhs(1540, 209, 338);
            s.copy_ad(349, 790);
        }

        s.b[1686] = (s.v[790] > 1e-6);
        s.v[1686] = if s.b[1686] { 1.0 } else { 0.0 };

        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {
            s.store_div_ad_rhs(336, 1502, A::square(s.ad_value(185)));
            s.store_add_scaled_inputs3_offset(334, s.ad_value(85), 1.0, s.ad_value(155), (-1.0), s.ad_value(1438), -1.0, 2.0);
            s.store_offset_mul_ad(338, A::div_from_scalar(2.0, s.ad_value(336)), s.ad_value(334), 1.0);
        }

        s.b[1687] = ((s.v[338] < 2.0) && (2.0 >= 0.0));
        s.v[1687] = if s.b[1687] { 1.0 } else { 0.0 };

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) {
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

        s.b[1688] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1688] = if s.b[1688] { 1.0 } else { 0.0 };

        s.b[1689] = (2.0 == 1.0);
        s.v[1689] = if s.b[1689] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) && s.b[1689]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1690] = (2.0 == 2.0);
        s.v[1690] = if s.b[1690] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) && (!s.b[1689])) && s.b[1690]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1691] = (2.0 == 4.0);
        s.v[1691] = if s.b[1691] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) && (!s.b[1689])) && (!s.b[1690])) && s.b[1691]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1692] = (2.0 == 8.0);
        s.v[1692] = if s.b[1692] { 1.0 } else { 0.0 };

        if ((((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) && (!s.b[1689])) && (!s.b[1690])) && (!s.b[1691])) && s.b[1692]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign29870_loop_guard: usize = 0;
        while {
            let assign29870_cond_e29068: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign29870_cond_e29068 != 0.0
        } {
            assign29870_loop_guard += 1;
            assert!(assign29870_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && s.b[1688]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) && (!s.b[1688])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 2.0);
            s.store_div_scaled_product_indices(334, 725, 726, 2.0, 770, 1.0);
            s.store_sub_from_scalar(343, 2.0, 780);
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1687]) {
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && (!s.b[1687])) {
            s.copy_ad(343, 338);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {
            s.store_offset(343, 343, 1e-25);
            s.store_sqrt(337, 343);
            s.store_mul_sub_from_scalar_rhs(338, 336, 1.0, 337);
            s.store_add_ad_lhs(344, A::offset(s.ad_value(85), 2.0), 338);
        }

        s.b[1693] = ((s.v[344] < (0.3 + 0.2)) && (0.2 >= 0.0));
        s.v[1693] = if s.b[1693] { 1.0 } else { 0.0 };

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) {
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
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) {
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1694] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1694] = if s.b[1694] { 1.0 } else { 0.0 };

        s.b[1695] = (4.0 == 1.0);
        s.v[1695] = if s.b[1695] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) && s.b[1695]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1696] = (4.0 == 2.0);
        s.v[1696] = if s.b[1696] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) && (!s.b[1695])) && s.b[1696]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1697] = (4.0 == 4.0);
        s.v[1697] = if s.b[1697] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) && (!s.b[1695])) && (!s.b[1696])) && s.b[1697]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1698] = (4.0 == 8.0);
        s.v[1698] = if s.b[1698] { 1.0 } else { 0.0 };

        if ((((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) && (!s.b[1695])) && (!s.b[1696])) && (!s.b[1697])) && s.b[1698]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign30300_loop_guard: usize = 0;
        while {
            let assign30300_cond_e29601: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign30300_cond_e29601 != 0.0
        } {
            assign30300_loop_guard += 1;
            assert!(assign30300_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && (!s.b[1694])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                }
            });
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.2);
            s.store_div_scaled_product_indices(334, 725, 726, 0.2, 770, 1.0);
            s.store_sub_from_scalar(344, (0.3 + 0.2), 780);
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) {
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && (!s.b[1693])) {
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && (!s.b[1693])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {
            s.store_offset(344, 344, (10.0 * 2.220446049250313e-16));
            s.store_div(335, 790, 344);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {
            s.store_ad_value(336, {
                if (s.v[335] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(335), A::offset(s.ad_value(658), (-1.0)))
                }
            });
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {
            s.store_mul(341, 336, 335);
            s.store_offset(337, 341, 1.0);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {
            s.store_ad_value(338, {
                if (s.v[337] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)))
                }
            });
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {
            s.store_mul(340, 338, 337);
            s.store_div(348, 790, 340);
        }

        s.b[1699] = ((s.v[85] < 0.5) && (0.5 >= 0.0));
        s.v[1699] = if s.b[1699] { 1.0 } else { 0.0 };

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) {
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

        s.b[1700] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1700] = if s.b[1700] { 1.0 } else { 0.0 };

        s.b[1701] = (2.0 == 1.0);
        s.v[1701] = if s.b[1701] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) && s.b[1701]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1702] = (2.0 == 2.0);
        s.v[1702] = if s.b[1702] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) && (!s.b[1701])) && s.b[1702]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1703] = (2.0 == 4.0);
        s.v[1703] = if s.b[1703] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) && (!s.b[1701])) && (!s.b[1702])) && s.b[1703]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1704] = (2.0 == 8.0);
        s.v[1704] = if s.b[1704] { 1.0 } else { 0.0 };

        if ((((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) && (!s.b[1701])) && (!s.b[1702])) && (!s.b[1703])) && s.b[1704]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign30730_loop_guard: usize = 0;
        while {
            let assign30730_cond_e30141: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign30730_cond_e30141 != 0.0
        } {
            assign30730_loop_guard += 1;
            assert!(assign30730_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && (!s.b[1700])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.5);
            s.store_div_scaled_product_indices(334, 725, 726, 0.5, 770, 1.0);
            s.store_sub_from_scalar(1537, 0.5, 780);
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) {
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && (!s.b[1699])) {
            s.copy_ad(1537, 85);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {
            s.store_scale(335, 1537, 0.8);
        }

        s.b[1705] = ((s.v[348] > (s.v[1537] - s.v[335])) && (s.v[335] >= 0.0));
        s.v[1705] = if s.b[1705] { 1.0 } else { 0.0 };

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) {
            s.store_add_scaled_inputs3(781, s.ad_value(348), 1.0, s.ad_value(1537), (-1.0), s.ad_value(335), 1.0);
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

        s.b[1706] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1706] = if s.b[1706] { 1.0 } else { 0.0 };

        s.b[1707] = (2.0 == 1.0);
        s.v[1707] = if s.b[1707] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) && s.b[1707]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1708] = (2.0 == 2.0);
        s.v[1708] = if s.b[1708] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) && (!s.b[1707])) && s.b[1708]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1709] = (2.0 == 4.0);
        s.v[1709] = if s.b[1709] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) && (!s.b[1707])) && (!s.b[1708])) && s.b[1709]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1710] = (2.0 == 8.0);
        s.v[1710] = if s.b[1710] { 1.0 } else { 0.0 };

        if ((((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) && (!s.b[1707])) && (!s.b[1708])) && (!s.b[1709])) && s.b[1710]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign31090_loop_guard: usize = 0;
        while {
            let assign31090_cond_e30593: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign31090_cond_e30593 != 0.0
        } {
            assign31090_loop_guard += 1;
            assert!(assign31090_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && (!s.b[1706])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_div_scaled_product3(334, s.ad_value(335), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0);
            s.store_add_scaled_inputs3(790, s.ad_value(1537), 1.0, s.ad_value(335), (-1.0), s.ad_value(780), 1.0);
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) {
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && (!s.b[1705])) {
            s.copy_ad(790, 348);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1686])) {
            s.copy_ad(348, 790);
        }

        s.b[1711] = (s.v[790] <= 0.0);
        s.v[1711] = if s.b[1711] { 1.0 } else { 0.0 };

        if ((s.b[1443] && s.b[1444]) && s.b[1711]) {
            s.copy_ad(1462, 1461);
            s.copy_ad(1484, 1483);
            s.copy_ad(1465, 1464);
            s.copy_ad(1478, 1477);
            s.copy_ad(1539, 1538);
            s.copy_ad(1499, 1497);
            s.copy_ad(1500, 1498);
            s.copy_ad(1518, 1517);
            s.copy_ad(1516, 1515);
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
            s.store_sqrt_mul_ad(1454, A::div_scaled_product(s.ad_value(1547), s.ad_value(622), 1.0, A::add(s.ad_value(622), s.ad_value(964)), 1.0), A::add_scaled_inputs3(s.ad_value(790), 1.0, s.ad_value(1435), (-1.0), s.ad_value(1463), 1.0));
        }

        s.b[1712] = (s.v[1454] > s.v[965]);
        s.v[1712] = if s.b[1712] { 1.0 } else { 0.0 };

        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
            s.copy_ad(1466, 790);
        }

    }

    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
    ) {
        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
            s.copy_ad(1448, 965);
            s.copy_ad(1484, 790);
            s.copy_ad(1512, 790);
            s.store_sub_ad_rhs(1465, 1484, A::mul3(s.ad_value(1548), s.ad_value(1448), s.ad_value(1448)));
            s.copy_ad(1510, 1466);
            s.copy_ad(1473, 1465);
            s.store_mul(1499, 1448, 1546);
            s.store_scalar(97, 1.0);
        }

        let mut assign31400_loop_guard: usize = 0;
        while {
            let assign31400_cond_e30960: f64 = (150.0 + 1.0);
            let assign31400_cond_e30962: f64 = if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (s.v[97] <= assign31400_cond_e30960)) { 1.0 } else { 0.0 };
            assign31400_cond_e30962 != 0.0
        } {
            assign31400_loop_guard += 1;
            assert!(assign31400_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
                s.store_sqrt_mul_ad(1448, s.ad_value(1547), A::sub(s.ad_value(1484), s.ad_value(1465)));
            }
            s.b[1713] = ((s.v[1448] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.v[1713] = if s.b[1713] { 1.0 } else { 0.0 };
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) {
                s.store_offset_sub(781, 1448, 965, 1e-8);
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
            s.b[1714] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1714] = if s.b[1714] { 1.0 } else { 0.0 };
            s.b[1715] = (2.0 == 1.0);
            s.v[1715] = if s.b[1715] { 1.0 } else { 0.0 };
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) && s.b[1715]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1716] = (2.0 == 2.0);
            s.v[1716] = if s.b[1716] { 1.0 } else { 0.0 };
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) && (!s.b[1715])) && s.b[1716]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1717] = (2.0 == 4.0);
            s.v[1717] = if s.b[1717] { 1.0 } else { 0.0 };
            if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) && (!s.b[1715])) && (!s.b[1716])) && s.b[1717]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1718] = (2.0 == 8.0);
            s.v[1718] = if s.b[1718] { 1.0 } else { 0.0 };
            if (((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) && (!s.b[1715])) && (!s.b[1716])) && (!s.b[1717])) && s.b[1718]) {
                s.store_scalar(720, 4.0);
            }
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign31400_body27_loop_guard: usize = 0;
            while {
                let assign31400_body27_cond_e31344: f64 = if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign31400_body27_cond_e31344 != 0.0
            } {
                assign31400_body27_loop_guard += 1;
                assert!(assign31400_body27_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && (!s.b[1714])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(334, 725, 726, 1e-8, 770, 1.0);
                s.store_add_ad_lhs(1448, A::offset(s.ad_value(965), (-1e-8)), 780);
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) {
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (!s.b[1713])) {
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (!s.b[1713])) {
                s.store_scalar(334, 1.0);
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
                s.store_add_scaled_inputs3(335, s.ad_value(1465), 1.0, s.ad_value(1435), (-1.0), s.ad_value(1463), 1.0);
            }
            s.b[1719] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1719] = if s.b[1719] { 1.0 } else { 0.0 };
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) {
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
            s.b[1720] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1720] = if s.b[1720] { 1.0 } else { 0.0 };
            s.b[1721] = (2.0 == 1.0);
            s.v[1721] = if s.b[1721] { 1.0 } else { 0.0 };
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) && s.b[1721]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1722] = (2.0 == 2.0);
            s.v[1722] = if s.b[1722] { 1.0 } else { 0.0 };
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) && (!s.b[1721])) && s.b[1722]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1723] = (2.0 == 4.0);
            s.v[1723] = if s.b[1723] { 1.0 } else { 0.0 };
            if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) && (!s.b[1721])) && (!s.b[1722])) && s.b[1723]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1724] = (2.0 == 8.0);
            s.v[1724] = if s.b[1724] { 1.0 } else { 0.0 };
            if (((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) && (!s.b[1721])) && (!s.b[1722])) && (!s.b[1723])) && s.b[1724]) {
                s.store_scalar(720, 4.0);
            }
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign31400_body63_loop_guard: usize = 0;
            while {
                let assign31400_body63_cond_e31894: f64 = if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign31400_body63_cond_e31894 != 0.0
            } {
                assign31400_body63_loop_guard += 1;
                assert!(assign31400_body63_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && (!s.b[1720])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(341, 725, 726, 0.1, 770, 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) {
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (!s.b[1719])) {
                s.copy_ad(336, 335);
                s.store_scalar(341, 1.0);
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
                s.store_sqrt_mul(1452, 1550, 336);
                s.store_mul(1499, 1448, 1546);
                s.store_mul_div_from_scalar_lhs(1530, (-1.034943e-10), 1448, 334);
                s.store_mul_neg_lhs(1500, 1452, 1544);
                s.store_mul_div_from_scalar_lhs(1532, (-1.034943e-10), 1452, 341);
                s.store_add_ad_lhs(1485, A::add_scaled_product(s.ad_value(1499), 1.0, s.ad_value(185), A::sub(s.ad_value(1466), s.ad_value(1484)), 1.0), 1500);
                s.copy_ad(1487, 185);
                s.store_add(1488, 1530, 1532);
                s.store_add_scaled_product(1486, s.ad_value(1465), 1.0, s.ad_value(1535), A::sub(A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1484), 1.0), s.ad_value(1463)), (-1.0));
                s.store_scalar(1489, 0.0);
                s.store_scalar(1490, 1.0);
                s.store_add_scaled_products_indices(1491, 1487, 1490, 1.0, 1489, 1488, (-1.0));
                s.store_div(1492, 1490, 1491);
                s.store_div_scaled_inputs(1493, s.ad_value(1488), -1.0, s.ad_value(1491), 1.0);
                s.store_div_scaled_inputs(1494, s.ad_value(1489), -1.0, s.ad_value(1491), 1.0);
                s.store_div(1495, 1487, 1491);
            }
            s.b[1725] = (((((s.v[1492] * s.v[1485]) + (s.v[1493] * s.v[1486]))) as f64).abs() > 0.5);
            s.v[1725] = if s.b[1725] { 1.0 } else { 0.0 };
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1725]) {
                s.store_offset(1466, 1466, (-(0.5 * (if (((s.v[1492] * s.v[1485]) + (s.v[1493] * s.v[1486])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1725]) {
                s.store_offset(1465, 1465, (-(0.5 * (if (((s.v[1494] * s.v[1485]) + (s.v[1495] * s.v[1486])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (!s.b[1725])) {
                s.store_sub_ad_rhs(1466, 1466, A::add_scaled_products(s.ad_value(1492), s.ad_value(1485), 1.0, s.ad_value(1493), s.ad_value(1486), 1.0));
                s.store_sub_ad_rhs(1465, 1465, A::add_scaled_products(s.ad_value(1494), s.ad_value(1485), 1.0, s.ad_value(1495), s.ad_value(1486), 1.0));
            }
            s.b[1726] = (((((s.v[1466] - s.v[1510])) as f64).abs() <= 1e-12) && ((((s.v[1465] - s.v[1473])) as f64).abs() <= 1e-12));
            s.v[1726] = if s.b[1726] { 1.0 } else { 0.0 };
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1726]) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
                s.copy_ad(1510, 1466);
                s.copy_ad(1473, 1465);
                s.store_offset(97, 97, 1.0);
            }
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
            s.copy_ad(1514, 1465);
            s.store_mul(1452, 965, 1536);
            s.store_add_scaled_inputs3(1465, A::mul3(s.ad_value(1551), s.ad_value(1452), s.ad_value(1452)), 1.0, s.ad_value(1435), 1.0, s.ad_value(1463), -1.0);
            s.store_add_scaled_product_indices(1484, 1465, 1.0, 1548, 1543, 1.0);
            s.copy_ad(1462, 1484);
            s.copy_ad(1467, 1484);
            s.copy_ad(1509, 1484);
        }

        s.b[1727] = (s.v[85] > s.v[1466]);
        s.v[1727] = if s.b[1727] { 1.0 } else { 0.0 };

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1727]) {
            s.store_scalar(1479, 1.0);
        }

        s.b[1728] = (s.v[85] > s.v[1509]);
        s.v[1728] = if s.b[1728] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (!s.b[1727])) && s.b[1728]) {
            s.store_scalar(1479, 3.0);
        }

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (!s.b[1727])) && (!s.b[1728])) {
            s.store_scalar(1479, 2.0);
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1712])) {
            s.copy_ad(1466, 790);
            s.copy_ad(1509, 1466);
            s.copy_ad(1467, 1466);
            s.copy_ad(1512, 1466);
            s.copy_ad(1448, 1454);
            s.store_mul(1452, 1448, 1536);
            s.store_add_scaled_inputs3(1465, A::mul3(s.ad_value(1551), s.ad_value(1452), s.ad_value(1452)), 1.0, s.ad_value(1435), 1.0, s.ad_value(1463), -1.0);
            s.store_add_ad_lhs(1484, A::mul3(s.ad_value(1548), s.ad_value(1448), s.ad_value(1448)), 1465);
            s.copy_ad(1514, 1465);
        }

        s.b[1729] = (s.v[85] > s.v[1466]);
        s.v[1729] = if s.b[1729] { 1.0 } else { 0.0 };

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1712])) && s.b[1729]) {
            s.store_scalar(1479, 1.0);
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1712])) && (!s.b[1729])) {
            s.store_scalar(1479, 2.0);
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
            s.store_mul_ad_rhs(335, 1549, A::add_scaled_inputs3(s.ad_value(1467), 1.0, s.ad_value(1435), -1.0, s.ad_value(961), 1.0));
        }

        s.b[1730] = (s.v[335] > 0.0);
        s.v[1730] = if s.b[1730] { 1.0 } else { 0.0 };

        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1730]) {
            s.store_add_scaled_inputs3(1455, s.ad_value(1435), 1.0, s.ad_value(961), (-1.0), A::div(A::sqrt(s.ad_value(335)), s.ad_value(185)), -1.0);
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1730])) {
            s.store_sub(1455, 1435, 961);
        }

        s.b[1731] = (s.v[85] > s.v[1466]);
        s.v[1731] = if s.b[1731] { 1.0 } else { 0.0 };

        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1731]) {
            s.copy_ad(1465, 1514);
            s.copy_ad(1484, 790);
            s.store_add_ad_lhs(1481, A::div(A::ln(A::mul3(s.ad_value(1456), s.ad_value(85), s.ad_value(85))), A::add(s.ad_value(154), A::div_from_scalar(2.0, s.ad_value(85)))), 790);
        }

        s.b[1732] = (s.v[1481] < (s.v[1512] + s.v[1553]));
        s.v[1732] = if s.b[1732] { 1.0 } else { 0.0 };

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1731]) && s.b[1732]) {
            s.store_add(1481, 1512, 1553);
        }

        s.b[1733] = (s.v[85] > s.v[1509]);
        s.v[1733] = if s.b[1733] { 1.0 } else { 0.0 };

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && s.b[1733]) {
            s.copy_ad(1481, 1462);
        }

        s.b[1734] = (s.v[85] > s.v[1455]);
        s.v[1734] = if s.b[1734] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) {
            s.store_add_scaled_product_indices(1457, 154, 1.0, 1456, 85, (-2.0));
            s.store_add_scaled_product(1458, A::mul3(s.ad_value(1456), s.ad_value(85), s.ad_value(85)), 1.0, s.ad_value(154), s.ad_value(1484), (-1.0));
            s.copy_ad(1471, 1484);
            s.store_div_scaled_inputs2(1481, A::sqrt(A::add_scaled_square_product(s.ad_value(1457), 1.0, s.ad_value(1456), s.ad_value(1458), (-4.0))), 0.5, s.ad_value(1457), (-0.5), s.ad_value(1456), 1.0);
        }

        s.b[1735] = (s.v[1481] > (s.v[1467] - s.v[1553]));
        s.v[1735] = if s.b[1735] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
    ) {
        if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1735]) {
            s.store_sub(1481, 1467, 1553);
        }

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) {
            s.store_sqrt_mul_ad(1450, s.ad_value(1547), A::sub(s.ad_value(1484), s.ad_value(1481)));
            s.store_sqrt_mul_ad(1448, s.ad_value(1547), A::sub(s.ad_value(1484), s.ad_value(1465)));
        }

        s.b[1736] = ((s.v[1450] + s.v[1448]) > s.v[965]);
        s.v[1736] = if s.b[1736] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) {
            s.store_scalar(97, 1.0);
        }

        let mut assign31880_loop_guard: usize = 0;
        while {
            let assign31880_cond_e33122: f64 = (150.0 + 1.0);
            let assign31880_cond_e33124: f64 = if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && (s.v[97] <= assign31880_cond_e33122)) { 1.0 } else { 0.0 };
            assign31880_cond_e33124 != 0.0
        } {
            assign31880_loop_guard += 1;
            assert!(assign31880_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) {
                s.store_add_scaled_inputs3(1468, s.ad_value(1450), 1.0, s.ad_value(1448), 1.0, s.ad_value(965), -1.0);
                s.store_add_ad(1508, A::div_scalar_by_product(1.034943e-10, s.ad_value(1546), s.ad_value(1450), 1.0), A::div(A::mul_sub_from_scalar_rhs(A::div_from_scalar(1.034943e-10, s.ad_value(1546)), 1.0, A::div_scaled_value_offset_denominator(s.ad_value(1536), 1.0, s.ad_value(1536), 1.0, 1.0)), s.ad_value(1448)));
            }
            s.b[1737] = ((((s.v[1468] / s.v[1508])) as f64).abs() > 0.5);
            s.v[1737] = if s.b[1737] { 1.0 } else { 0.0 };
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1737]) {
                s.store_offset(1484, 1484, (-(0.5 * (if ((s.v[1468] / s.v[1508]) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && (!s.b[1737])) {
                s.store_sub_ad_rhs(1484, 1484, A::div(s.ad_value(1468), s.ad_value(1508)));
            }
            s.b[1738] = (((s.v[1484] - s.v[1435]) + s.v[1463]) < (10.0 * 2.220446049250313e-16));
            s.v[1738] = if s.b[1738] { 1.0 } else { 0.0 };
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1738]) {
                s.store_offset_sub(1484, 1435, 1463, (10.0 * 2.220446049250313e-16));
            }
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) {
                s.store_add_scaled_product(1458, A::mul3(s.ad_value(1456), s.ad_value(85), s.ad_value(85)), 1.0, s.ad_value(154), s.ad_value(1484), (-1.0));
                s.store_add_scaled_square_product(335, s.ad_value(1457), 1.0, s.ad_value(1456), s.ad_value(1458), (-4.0));
            }
            s.b[1739] = (s.v[335] > 0.0);
            s.v[1739] = if s.b[1739] { 1.0 } else { 0.0 };
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1739]) {
                s.store_div_scaled_inputs2(1481, A::sqrt(s.ad_value(335)), 0.5, s.ad_value(1457), (-0.5), s.ad_value(1456), 1.0);
            }
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && (!s.b[1739])) {
                s.store_div_scaled_inputs(1481, s.ad_value(1457), (-0.5), s.ad_value(1456), 1.0);
            }
            s.b[1740] = (s.v[1481] > s.v[1467]);
            s.v[1740] = if s.b[1740] { 1.0 } else { 0.0 };
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1740]) {
                s.copy_ad(1481, 1467);
            }
            s.b[1741] = (s.v[1481] > s.v[1484]);
            s.v[1741] = if s.b[1741] { 1.0 } else { 0.0 };
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1741]) {
                s.store_sub(1481, 1484, 1553);
                s.store_scalar(97, (150.0 + 1.0));
            }
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) {
                s.store_sqrt_mul_ad(1450, s.ad_value(1547), A::sub(s.ad_value(1484), s.ad_value(1481)));
                s.store_div_scaled_inputs2(1465, A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1484), 1.0), 1.0, s.ad_value(1463), (-1.0), A::offset(s.ad_value(1536), 1.0), 1.0);
                s.store_sqrt_mul_ad(1448, s.ad_value(1547), A::sub(s.ad_value(1484), s.ad_value(1465)));
            }
            s.b[1742] = ((((s.v[1484] - s.v[1471])) as f64).abs() <= 1e-8);
            s.v[1742] = if s.b[1742] { 1.0 } else { 0.0 };
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1742]) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) {
                s.copy_ad(1471, 1484);
                s.store_offset(97, 97, 1.0);
            }
        }

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && (!s.b[1734])) {
            s.copy_ad(1484, 1483);
            s.copy_ad(1465, 1464);
            s.copy_ad(1481, 1461);
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
            s.copy_ad(1482, 1484);
            s.store_scalar(79, 0.0);
            s.copy_ad(1462, 1481);
            s.copy_ad(1484, 1482);
            s.copy_ad(1474, 1462);
            s.copy_ad(1471, 1484);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
    ) {
        let mut assign31990_loop_guard: usize = 0;
        while {
            let assign31990_cond_e33744: f64 = (150.0 + 1.0);
            let assign31990_cond_e33746: f64 = if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (s.v[97] <= assign31990_cond_e33744)) { 1.0 } else { 0.0 };
            assign31990_cond_e33746 != 0.0
        } {
            assign31990_loop_guard += 1;
            assert!(assign31990_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
                s.store_mul_sub_ad_rhs(1465, 1535, A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1484), 1.0), s.ad_value(1463));
                s.store_mul(1534, 1535, 1536);
                s.store_sub(335, 1484, 1465);
            }
            s.b[1743] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1743] = if s.b[1743] { 1.0 } else { 0.0 };
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) {
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
            s.b[1744] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1744] = if s.b[1744] { 1.0 } else { 0.0 };
            s.b[1745] = (2.0 == 1.0);
            s.v[1745] = if s.b[1745] { 1.0 } else { 0.0 };
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) && s.b[1745]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1746] = (2.0 == 2.0);
            s.v[1746] = if s.b[1746] { 1.0 } else { 0.0 };
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) && (!s.b[1745])) && s.b[1746]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1747] = (2.0 == 4.0);
            s.v[1747] = if s.b[1747] { 1.0 } else { 0.0 };
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) && (!s.b[1745])) && (!s.b[1746])) && s.b[1747]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1748] = (2.0 == 8.0);
            s.v[1748] = if s.b[1748] { 1.0 } else { 0.0 };
            if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) && (!s.b[1745])) && (!s.b[1746])) && (!s.b[1747])) && s.b[1748]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign31990_body29_loop_guard: usize = 0;
            while {
                let assign31990_body29_cond_e34109: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign31990_body29_cond_e34109 != 0.0
            } {
                assign31990_body29_loop_guard += 1;
                assert!(assign31990_body29_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && (!s.b[1744])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) {
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1743])) {
                s.copy_ad(336, 335);
                s.store_scalar(334, 1.0);
            }
            if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
                s.store_sqrt_mul(1448, 1547, 336);
            }
            s.b[1749] = ((s.v[1448] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.v[1749] = if s.b[1749] { 1.0 } else { 0.0 };
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) {
                s.store_offset_sub(781, 1448, 965, 1e-8);
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
            s.b[1750] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1750] = if s.b[1750] { 1.0 } else { 0.0 };
            s.b[1751] = (2.0 == 1.0);
            s.v[1751] = if s.b[1751] { 1.0 } else { 0.0 };
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) && s.b[1751]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1752] = (2.0 == 2.0);
            s.v[1752] = if s.b[1752] { 1.0 } else { 0.0 };
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) && (!s.b[1751])) && s.b[1752]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1753] = (2.0 == 4.0);
            s.v[1753] = if s.b[1753] { 1.0 } else { 0.0 };
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) && (!s.b[1751])) && (!s.b[1752])) && s.b[1753]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1754] = (2.0 == 8.0);
            s.v[1754] = if s.b[1754] { 1.0 } else { 0.0 };
            if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) && (!s.b[1751])) && (!s.b[1752])) && (!s.b[1753])) && s.b[1754]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign31990_body65_loop_guard: usize = 0;
            while {
                let assign31990_body65_cond_e34594: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign31990_body65_cond_e34594 != 0.0
            } {
                assign31990_body65_loop_guard += 1;
                assert!(assign31990_body65_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && (!s.b[1750])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
                s.store_add_ad_lhs(1448, A::offset(s.ad_value(965), (-1e-8)), 780);
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) {
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1749])) {
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1749])) {
                s.store_scalar(337, 1.0);
            }
            if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
                s.store_sqrt_mul_ad(1452, s.ad_value(1550), A::add_scaled_inputs3(s.ad_value(1465), 1.0, s.ad_value(1435), (-1.0), s.ad_value(1463), 1.0));
                s.store_mul(1499, 1448, 1546);
                s.store_mul_ad_product_lhs(1528, A::div_from_scalar(1.034943e-10, s.ad_value(1448)), s.ad_value(334), 337);
                s.store_mul_ad_product_lhs(1530, A::div_from_scalar((-1.034943e-10), s.ad_value(1448)), s.ad_value(334), 337);
                s.store_mul_neg_lhs(1500, 1452, 1544);
                s.store_div_from_scalar(1532, (-1.034943e-10), 1452);
                s.store_scaled_mul(335, 1502, 1543, 8.0);
            }
            if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
                let assign31990_body81_ad_e34883: A = A::add(A::add_scaled_inputs4(A::mul3_scaled_output(s.ad_value(1465), s.ad_value(1465), s.ad_value(1542), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1465), s.ad_value(1542), s.ad_value(1462), 8.0), (-1.0), A::mul3_scaled_output(s.ad_value(1542), s.ad_value(1462), s.ad_value(1462), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1465), s.ad_value(1502), s.ad_value(1543), 4.0), 1.0), A::mul3_scaled_output(s.ad_value(1462), s.ad_value(1502), s.ad_value(1543), 4.0));
                s.store_div_scaled_add_product(1520, assign31990_body81_ad_e34883, 1.0, A::mul3(s.ad_value(1545), s.ad_value(1541), s.ad_value(1543)), s.ad_value(1543), 1.0, s.ad_value(335), 1.0);
            }
            if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
                s.store_div_ad_lhs(1521, A::add_scaled_products3(s.ad_value(1465), s.ad_value(1542), (-8.0), s.ad_value(1542), s.ad_value(1462), (4.0 * 2.0), s.ad_value(1502), s.ad_value(1543), 4.0), 335);
                s.store_div_ad_lhs(1522, A::add_scaled_products3(s.ad_value(1465), s.ad_value(1542), (4.0 * 2.0), s.ad_value(1542), s.ad_value(1462), (-8.0), s.ad_value(1502), s.ad_value(1543), 4.0), 335);
                s.store_mul_sub_rhs(335, 154, 1462, 1484);
                s.store_exp(336, 335);
            }
            s.b[1755] = (s.v[1462] >= s.v[1484]);
            s.v[1755] = if s.b[1755] { 1.0 } else { 0.0 };
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1755]) {
                s.store_mul_scaled_ad_rhs(1476, 209, -1.0, A::sqrt(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15)));
                s.store_mul_ad(1524, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(1476), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
                s.store_neg(1526, 1524);
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1755])) {
                s.store_exp_ad(337, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1462), s.ad_value(1435))));
                s.store_exp_ad(338, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1484), s.ad_value(1435))));
                s.store_mul_sqrt_ad_rhs(1476, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1476, 1.0);
                s.store_mul_add_ad_rhs(1524, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
                s.store_mul_add_ad_rhs(1526, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));
            }
            s.b[1756] = ((s.v[1520] > (s.v[1512] - s.v[1519])) && (s.v[1519] >= 0.0));
            s.v[1756] = if s.b[1756] { 1.0 } else { 0.0 };
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) {
                s.store_add_scaled_inputs3(781, s.ad_value(1520), 1.0, s.ad_value(1512), (-1.0), s.ad_value(1519), 1.0);
                s.store_square(722, 781);
                s.store_square(723, 1519);
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
            s.b[1757] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.v[1757] = if s.b[1757] { 1.0 } else { 0.0 };
            s.b[1758] = (4.0 == 1.0);
            s.v[1758] = if s.b[1758] { 1.0 } else { 0.0 };
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) && s.b[1758]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1759] = (4.0 == 2.0);
            s.v[1759] = if s.b[1759] { 1.0 } else { 0.0 };
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) && (!s.b[1758])) && s.b[1759]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1760] = (4.0 == 4.0);
            s.v[1760] = if s.b[1760] { 1.0 } else { 0.0 };
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) && (!s.b[1758])) && (!s.b[1759])) && s.b[1760]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1761] = (4.0 == 8.0);
            s.v[1761] = if s.b[1761] { 1.0 } else { 0.0 };
            if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) && (!s.b[1758])) && (!s.b[1759])) && (!s.b[1760])) && s.b[1761]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign31990_body126_loop_guard: usize = 0;
            while {
                let assign31990_body126_cond_e35543: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign31990_body126_cond_e35543 != 0.0
            } {
                assign31990_body126_loop_guard += 1;
                assert!(assign31990_body126_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && (!s.b[1757])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                    }
                });
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_mul3_lhs(780, 781, 1519, 726);
                s.store_div_scaled_product3(334, s.ad_value(1519), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0);
                s.store_add_scaled_inputs3(335, s.ad_value(1512), 1.0, s.ad_value(1519), (-1.0), s.ad_value(780), 1.0);
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) {
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1756])) {
                s.copy_ad(335, 1520);
                s.store_scalar(334, 1.0);
            }
            if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
                s.store_sub(1485, 1484, 335);
                s.store_mul_neg_lhs(1487, 1521, 334);
                s.store_sub_from_scalar_ad(1488, 1.0, A::mul3(s.ad_value(1522), s.ad_value(1534), s.ad_value(334)));
                s.store_add_scaled_inputs3(1486, A::add_scaled_product(s.ad_value(1476), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1462)), 1.0), 1.0, s.ad_value(1499), 1.0, s.ad_value(1500), 1.0);
                s.store_sub(1489, 1524, 185);
                s.store_ad_value(1490, A::add_scaled_inputs_products(s.ad_value(1526), 1.0, s.ad_value(1528), 1.0, s.ad_value(1530), s.ad_value(1534), 1.0, s.ad_value(1532), s.ad_value(1534), 1.0));
                s.store_add_scaled_products_indices(1491, 1487, 1490, 1.0, 1489, 1488, (-1.0));
                s.store_div(1492, 1490, 1491);
                s.store_div_scaled_inputs(1493, s.ad_value(1488), -1.0, s.ad_value(1491), 1.0);
                s.store_div_scaled_inputs(1494, s.ad_value(1489), -1.0, s.ad_value(1491), 1.0);
                s.store_div(1495, 1487, 1491);
            }
            s.b[1762] = (((((s.v[1494] * s.v[1485]) + (s.v[1495] * s.v[1486]))) as f64).abs() > 0.5);
            s.v[1762] = if s.b[1762] { 1.0 } else { 0.0 };
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1762]) {
                s.store_offset(1462, 1462, (-(0.5 * (if (((s.v[1492] * s.v[1485]) + (s.v[1493] * s.v[1486])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1762]) {
                s.store_offset(1484, 1484, (-(0.5 * (if (((s.v[1494] * s.v[1485]) + (s.v[1495] * s.v[1486])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1762])) {
                s.store_sub_ad_rhs(1462, 1462, A::add_scaled_products(s.ad_value(1492), s.ad_value(1485), 1.0, s.ad_value(1493), s.ad_value(1486), 1.0));
                s.store_sub_ad_rhs(1484, 1484, A::add_scaled_products(s.ad_value(1494), s.ad_value(1485), 1.0, s.ad_value(1495), s.ad_value(1486), 1.0));
            }
            s.b[1763] = (((((s.v[1462] - s.v[1474])) as f64).abs() <= 1e-12) && ((((s.v[1484] - s.v[1471])) as f64).abs() <= 1e-12));
            s.v[1763] = if s.b[1763] { 1.0 } else { 0.0 };
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1763]) {
                s.store_scalar(97, (150.0 + 1.0));
                s.store_scalar(79, 1.0);
            }
            if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
                s.copy_ad(1474, 1462);
                s.copy_ad(1471, 1484);
                s.store_offset(97, 97, 1.0);
            }
        }

        s.b[1765] = ((s.v[1454] > s.v[965]) && (s.v[1479] != 2.0));
        s.v[1765] = if s.b[1765] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
    ) {
        s.b[1766] = ((s.v[1484] > (s.v[1462] - 0.02)) && (0.02 >= 0.0));
        s.v[1766] = if s.b[1766] { 1.0 } else { 0.0 };

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) {
            s.store_offset_sub(781, 1484, 1462, 0.02);
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

        s.b[1767] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1767] = if s.b[1767] { 1.0 } else { 0.0 };

        s.b[1768] = (2.0 == 1.0);
        s.v[1768] = if s.b[1768] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) && s.b[1767]) && s.b[1768]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1769] = (2.0 == 2.0);
        s.v[1769] = if s.b[1769] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) && s.b[1767]) && (!s.b[1768])) && s.b[1769]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1770] = (2.0 == 4.0);
        s.v[1770] = if s.b[1770] { 1.0 } else { 0.0 };

        if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) && s.b[1767]) && (!s.b[1768])) && (!s.b[1769])) && s.b[1770]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1771] = (2.0 == 8.0);
        s.v[1771] = if s.b[1771] { 1.0 } else { 0.0 };

        if (((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) && s.b[1767]) && (!s.b[1768])) && (!s.b[1769])) && (!s.b[1770])) && s.b[1771]) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) && s.b[1767]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign32280_loop_guard: usize = 0;
        while {
            let assign32280_cond_e36387: f64 = if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) && s.b[1767]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign32280_cond_e36387 != 0.0
        } {
            assign32280_loop_guard += 1;
            assert!(assign32280_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) && s.b[1767]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) && (!s.b[1767])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.02);
            s.store_div_scaled_product_indices(335, 725, 726, 0.02, 770, 1.0);
            s.store_add_ad_lhs(1484, A::offset(s.ad_value(1462), (-0.02)), 780);
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) {
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && (!s.b[1766])) {
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && (!s.b[1766])) {
            s.store_scalar(335, 1.0);
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
            s.store_mul_sub_ad_rhs(1465, 1535, A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1484), 1.0), s.ad_value(1463));
            s.store_mul_sub_rhs(335, 154, 1462, 1484);
            s.store_exp(336, 335);
        }

        s.b[1772] = (s.v[1462] >= s.v[1484]);
        s.v[1772] = if s.b[1772] { 1.0 } else { 0.0 };

        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) {
            s.store_mul_scaled_ad_rhs(1476, 209, -1.0, A::sqrt(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15)));
            s.copy_ad(1539, 1476);
            s.store_scalar(1518, 0.0);
            s.store_scalar(1478, 0.0);
            s.store_sqrt_mul_ad(1448, s.ad_value(1547), A::sub(s.ad_value(1484), s.ad_value(1465)));
        }

        s.b[1773] = ((s.v[1448] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
        s.v[1773] = if s.b[1773] { 1.0 } else { 0.0 };

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) {
            s.store_offset_sub(781, 1448, 965, 1e-8);
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

        s.b[1774] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1774] = if s.b[1774] { 1.0 } else { 0.0 };

        s.b[1775] = (2.0 == 1.0);
        s.v[1775] = if s.b[1775] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) && s.b[1774]) && s.b[1775]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1776] = (2.0 == 2.0);
        s.v[1776] = if s.b[1776] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) && s.b[1774]) && (!s.b[1775])) && s.b[1776]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1777] = (2.0 == 4.0);
        s.v[1777] = if s.b[1777] { 1.0 } else { 0.0 };

        if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) && s.b[1774]) && (!s.b[1775])) && (!s.b[1776])) && s.b[1777]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1778] = (2.0 == 8.0);
        s.v[1778] = if s.b[1778] { 1.0 } else { 0.0 };

        if (((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) && s.b[1774]) && (!s.b[1775])) && (!s.b[1776])) && (!s.b[1777])) && s.b[1778]) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) && s.b[1774]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign32720_loop_guard: usize = 0;
        while {
            let assign32720_cond_e37035: f64 = if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) && s.b[1774]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign32720_cond_e37035 != 0.0
        } {
            assign32720_loop_guard += 1;
            assert!(assign32720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) && s.b[1774]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) && (!s.b[1774])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
            s.store_add_ad_lhs(1448, A::offset(s.ad_value(965), (-1e-8)), 780);
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) {
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && (!s.b[1773])) {
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && (!s.b[1773])) {
            s.store_scalar(337, 1.0);
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) {
            s.store_sqrt_mul_ad(1452, s.ad_value(1550), A::add_scaled_inputs3(s.ad_value(1465), 1.0, s.ad_value(1435), (-1.0), s.ad_value(1463), 1.0));
            s.store_mul(1499, 1448, 1546);
            s.store_mul_neg_lhs(1500, 1452, 1544);
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) {
            s.store_exp_ad(337, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1462), s.ad_value(1435))));
            s.store_exp_ad(338, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1484), s.ad_value(1435))));
            s.store_mul_sqrt_ad_rhs(1476, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
        }

        s.b[1779] = ((s.v[1454] > s.v[965]) && (s.v[1479] != 2.0));
        s.v[1779] = if s.b[1779] { 1.0 } else { 0.0 };

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1779]) {
            s.store_scalar(1478, 0.0);
            s.store_scalar(1518, 0.0);
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && (!s.b[1779])) {
            s.store_mul_sqrt_ad_rhs(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1462), s.ad_value(1435)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1484), s.ad_value(1435)))))), s.ad_value(335)));
            s.store_add_scaled_product(1478, s.ad_value(337), 1.0, s.ad_value(209), A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));
            s.store_mul_sqrt_ad_rhs(1518, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) {
            s.store_scalar(1539, 0.0);
            s.store_sub(335, 1484, 1465);
        }

        s.b[1780] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
        s.v[1780] = if s.b[1780] { 1.0 } else { 0.0 };

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) {
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

        s.b[1781] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1781] = if s.b[1781] { 1.0 } else { 0.0 };

        s.b[1782] = (2.0 == 1.0);
        s.v[1782] = if s.b[1782] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) && s.b[1781]) && s.b[1782]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1783] = (2.0 == 2.0);
        s.v[1783] = if s.b[1783] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) && s.b[1781]) && (!s.b[1782])) && s.b[1783]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1784] = (2.0 == 4.0);
        s.v[1784] = if s.b[1784] { 1.0 } else { 0.0 };

        if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) && s.b[1781]) && (!s.b[1782])) && (!s.b[1783])) && s.b[1784]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1785] = (2.0 == 8.0);
        s.v[1785] = if s.b[1785] { 1.0 } else { 0.0 };

        if (((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) && s.b[1781]) && (!s.b[1782])) && (!s.b[1783])) && (!s.b[1784])) && s.b[1785]) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) && s.b[1781]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign33210_loop_guard: usize = 0;
        while {
            let assign33210_cond_e37842: f64 = if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) && s.b[1781]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign33210_cond_e37842 != 0.0
        } {
            assign33210_loop_guard += 1;
            assert!(assign33210_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) && s.b[1781]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) && (!s.b[1781])) {
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
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_sub_from_scalar(336, 0.1, 780);
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) {
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && (!s.b[1780])) {
            s.copy_ad(336, 335);
            s.store_scalar(334, 1.0);
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) {
            s.store_sqrt_mul(1448, 1547, 336);
        }

        s.b[1786] = ((s.v[1448] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
        s.v[1786] = if s.b[1786] { 1.0 } else { 0.0 };

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) {
            s.store_offset_sub(781, 1448, 965, 1e-8);
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

        s.b[1787] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1787] = if s.b[1787] { 1.0 } else { 0.0 };

        s.b[1788] = (2.0 == 1.0);
        s.v[1788] = if s.b[1788] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) && s.b[1787]) && s.b[1788]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1789] = (2.0 == 2.0);
        s.v[1789] = if s.b[1789] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) && s.b[1787]) && (!s.b[1788])) && s.b[1789]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1790] = (2.0 == 4.0);
        s.v[1790] = if s.b[1790] { 1.0 } else { 0.0 };

        if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) && s.b[1787]) && (!s.b[1788])) && (!s.b[1789])) && s.b[1790]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1791] = (2.0 == 8.0);
        s.v[1791] = if s.b[1791] { 1.0 } else { 0.0 };

        if (((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) && s.b[1787]) && (!s.b[1788])) && (!s.b[1789])) && (!s.b[1790])) && s.b[1791]) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) && s.b[1787]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign33570_loop_guard: usize = 0;
        while {
            let assign33570_cond_e38423: f64 = if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) && s.b[1787]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign33570_cond_e38423 != 0.0
        } {
            assign33570_loop_guard += 1;
            assert!(assign33570_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) && s.b[1787]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) && (!s.b[1787])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-8);
            s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
            s.store_add_ad_lhs(1448, A::offset(s.ad_value(965), (-1e-8)), 780);
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) {
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && (!s.b[1786])) {
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && (!s.b[1786])) {
            s.store_scalar(337, 1.0);
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) {
            s.store_sqrt_mul_ad(1452, s.ad_value(1550), A::add_scaled_inputs3(s.ad_value(1465), 1.0, s.ad_value(1435), (-1.0), s.ad_value(1463), 1.0));
            s.store_mul(1499, 1448, 1546);
            s.store_mul_neg_lhs(1500, 1452, 1544);
        }

        s.b[1792] = (((s.v[1462] - s.v[1512]) < 0.06) && (0.06 >= 0.0));
        s.v[1792] = if s.b[1792] { 1.0 } else { 0.0 };

        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) {
            s.store_sub_from_scalar_ad(781, 0.06, A::sub(s.ad_value(1462), s.ad_value(1512)));
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

        s.b[1793] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1793] = if s.b[1793] { 1.0 } else { 0.0 };

        s.b[1794] = (2.0 == 1.0);
        s.v[1794] = if s.b[1794] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) && s.b[1793]) && s.b[1794]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1795] = (2.0 == 2.0);
        s.v[1795] = if s.b[1795] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) && s.b[1793]) && (!s.b[1794])) && s.b[1795]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1796] = (2.0 == 4.0);
        s.v[1796] = if s.b[1796] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) && s.b[1793]) && (!s.b[1794])) && (!s.b[1795])) && s.b[1796]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1797] = (2.0 == 8.0);
        s.v[1797] = if s.b[1797] { 1.0 } else { 0.0 };

        if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) && s.b[1793]) && (!s.b[1794])) && (!s.b[1795])) && (!s.b[1796])) && s.b[1797]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) && s.b[1793]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign33950_loop_guard: usize = 0;
        while {
            let assign33950_cond_e38978: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) && s.b[1793]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign33950_cond_e38978 != 0.0
        } {
            assign33950_loop_guard += 1;
            assert!(assign33950_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) && s.b[1793]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) && (!s.b[1793])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.06);
            s.store_div_scaled_product_indices(334, 725, 726, 0.06, 770, 1.0);
            s.store_sub_from_scalar(336, 0.06, 780);
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) {
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1792])) {
            s.store_sub(336, 1462, 1512);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
            s.store_offset_ad(338, A::add_scaled_product(A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0)), (10.0 * 2.220446049250313e-16));
            s.store_mul_scaled_ad_rhs(1516, 209, -1.0, A::sqrt(s.ad_value(338)));
        }

        if (s.b[1443] && s.b[1444]) {
            s.copy_ad(87, 1461);
            s.copy_ad(91, 1462);
            s.store_sub(94, 1462, 1461);
            s.store_neg_ad(335, A::add(s.ad_value(1475), s.ad_value(1476)));
        }

        s.b[1798] = ((s.v[335] < s.v[1540]) && (s.v[1540] >= 0.0));
        s.v[1798] = if s.b[1798] { 1.0 } else { 0.0 };

        if ((s.b[1443] && s.b[1444]) && s.b[1798]) {
            s.store_sub(781, 1540, 335);
            s.store_square(722, 781);
            s.store_square(723, 1540);
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

        s.b[1799] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1799] = if s.b[1799] { 1.0 } else { 0.0 };

        s.b[1800] = (2.0 == 1.0);
        s.v[1800] = if s.b[1800] { 1.0 } else { 0.0 };

        if ((((s.b[1443] && s.b[1444]) && s.b[1798]) && s.b[1799]) && s.b[1800]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1801] = (2.0 == 2.0);
        s.v[1801] = if s.b[1801] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && s.b[1444]) && s.b[1798]) && s.b[1799]) && (!s.b[1800])) && s.b[1801]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1802] = (2.0 == 4.0);
        s.v[1802] = if s.b[1802] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && s.b[1444]) && s.b[1798]) && s.b[1799]) && (!s.b[1800])) && (!s.b[1801])) && s.b[1802]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1803] = (2.0 == 8.0);
        s.v[1803] = if s.b[1803] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && s.b[1444]) && s.b[1798]) && s.b[1799]) && (!s.b[1800])) && (!s.b[1801])) && (!s.b[1802])) && s.b[1803]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1798]) && s.b[1799]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign34360_loop_guard: usize = 0;
        while {
            let assign34360_cond_e39454: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1798]) && s.b[1799]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign34360_cond_e39454 != 0.0
        } {
            assign34360_loop_guard += 1;
            assert!(assign34360_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && s.b[1798]) && s.b[1799]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1798]) && (!s.b[1799])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1798]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1540, 726);
            s.store_div_scaled_product3(334, s.ad_value(1540), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0);
            s.store_sub(1556, 1540, 780);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1798]) {
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1798])) {
            s.copy_ad(1556, 335);
        }

    }

    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1443] && s.b[1444]) && (!s.b[1798])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_mul3_affine_lhs(1503, 154, 1556, 1.0 / (2.0), 0.0, 94);
            s.store_neg_ad(1504, A::sub(s.ad_value(1515), s.ad_value(1516)));
            s.store_add(248, 1503, 1504);
            s.store_neg(133, 1515);
            s.copy_ad(170, 162);
            s.store_scalar(336, (s.v[626] / 100.0));
            s.store_offset_mul_ad(338, A::sub(s.ad_value(1462), s.ad_value(1461)), s.ad_value(682), 1.0);
            s.store_mul(339, 336, 133);
            s.store_div(337, 339, 338);
            s.copy_ad(251, 337);
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_ad_value(339, {
                if (s.v[251] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(251), (p.p160 - 1.0))
                }
            });
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_mul(342, 339, 251);
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_ad_value(341, {
                if (s.v[251] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(251), A::offset(s.ad_value(624), (-1.0)))
                }
            });
        }

        if (s.b[1443] && s.b[1444]) {
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
            s.copy_ad(1558, 255);
        }

        s.b[1804] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1804] = if s.b[1804] { 1.0 } else { 0.0 };

        if ((s.b[1443] && s.b[1444]) && s.b[1804]) {
            s.store_scalar(337, 1.0);
        }

        s.b[1805] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1805] = if s.b[1805] { 1.0 } else { 0.0 };

        if (((s.b[1443] && s.b[1444]) && (!s.b[1804])) && s.b[1805]) {
            s.copy_ad(337, 335);
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1804])) && (!s.b[1805])) {
            s.store_ad_value(337, {
                if (s.v[335] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(335), (p.p178 - 1.0))
                }
            });
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_mul(336, 335, 337);
            s.store_offset(338, 336, 1.0);
        }

        s.b[1806] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1806] = if s.b[1806] { 1.0 } else { 0.0 };

        if ((s.b[1443] && s.b[1444]) && s.b[1806]) {
            s.store_div_from_scalar(339, 1.0, 338);
        }

        s.b[1807] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1807] = if s.b[1807] { 1.0 } else { 0.0 };

        if (((s.b[1443] && s.b[1444]) && (!s.b[1806])) && s.b[1807]) {
            s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1806])) && (!s.b[1807])) {
            s.store_ad_value(340, {
                if (s.v[338] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(338), (((-1.0) / p.p178) - 1.0))
                }
            });
        }

        if (((s.b[1443] && s.b[1444]) && (!s.b[1806])) && (!s.b[1807])) {
            s.store_mul(339, 338, 340);
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_mul(253, 254, 339);
        }

        s.b[1808] = (s.v[349] > 1e-6);
        s.v[1808] = if s.b[1808] { 1.0 } else { 0.0 };

        if ((s.b[1443] && s.b[1444]) && s.b[1808]) {
            s.store_div_ad_rhs(336, 1502, A::square(s.ad_value(185)));
            s.store_add_scaled_inputs4(334, s.ad_value(85), 1.0, s.ad_value(974), 1.0, s.ad_value(155), -1.0, s.ad_value(1438), -1.0);
            s.store_offset_mul_ad(338, A::div_from_scalar(2.0, s.ad_value(336)), s.ad_value(334), 1.0);
        }

        s.b[1809] = ((s.v[338] < 2.0) && (2.0 >= 0.0));
        s.v[1809] = if s.b[1809] { 1.0 } else { 0.0 };

        if (((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) {
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

        s.b[1810] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1810] = if s.b[1810] { 1.0 } else { 0.0 };

        s.b[1811] = (2.0 == 1.0);
        s.v[1811] = if s.b[1811] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) && s.b[1810]) && s.b[1811]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1812] = (2.0 == 2.0);
        s.v[1812] = if s.b[1812] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) && s.b[1810]) && (!s.b[1811])) && s.b[1812]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1813] = (2.0 == 4.0);
        s.v[1813] = if s.b[1813] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) && s.b[1810]) && (!s.b[1811])) && (!s.b[1812])) && s.b[1813]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1814] = (2.0 == 8.0);
        s.v[1814] = if s.b[1814] { 1.0 } else { 0.0 };

        if ((((((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) && s.b[1810]) && (!s.b[1811])) && (!s.b[1812])) && (!s.b[1813])) && s.b[1814]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) && s.b[1810]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign35170_loop_guard: usize = 0;
        while {
            let assign35170_cond_e40373: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) && s.b[1810]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign35170_cond_e40373 != 0.0
        } {
            assign35170_loop_guard += 1;
            assert!(assign35170_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) && s.b[1810]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) && (!s.b[1810])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 2.0);
            s.store_div_scaled_product_indices(334, 725, 726, 2.0, 770, 1.0);
            s.store_sub_from_scalar(343, 2.0, 780);
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) {
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1808]) && (!s.b[1809])) {
            s.copy_ad(343, 338);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1808]) {
            s.store_offset(343, 343, 1e-25);
            s.store_sqrt(337, 343);
            s.store_mul_sub_from_scalar_rhs(338, 336, 1.0, 337);
            s.store_add_scaled_inputs3(344, s.ad_value(85), 1.0, s.ad_value(974), 1.0, s.ad_value(338), 1.0);
            s.store_mul(344, 344, 975);
        }

        s.b[1815] = ((s.v[344] < (s.v[972] + 4.0)) && (4.0 >= 0.0));
        s.v[1815] = if s.b[1815] { 1.0 } else { 0.0 };

        if (((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) {
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

        s.b[1816] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1816] = if s.b[1816] { 1.0 } else { 0.0 };

        s.b[1817] = (4.0 == 1.0);
        s.v[1817] = if s.b[1817] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) && s.b[1816]) && s.b[1817]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1818] = (4.0 == 2.0);
        s.v[1818] = if s.b[1818] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) && s.b[1816]) && (!s.b[1817])) && s.b[1818]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1819] = (4.0 == 4.0);
        s.v[1819] = if s.b[1819] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) && s.b[1816]) && (!s.b[1817])) && (!s.b[1818])) && s.b[1819]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1820] = (4.0 == 8.0);
        s.v[1820] = if s.b[1820] { 1.0 } else { 0.0 };

        if ((((((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) && s.b[1816]) && (!s.b[1817])) && (!s.b[1818])) && (!s.b[1819])) && s.b[1820]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) && s.b[1816]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign35610_loop_guard: usize = 0;
        while {
            let assign35610_cond_e40916: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) && s.b[1816]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign35610_cond_e40916 != 0.0
        } {
            assign35610_loop_guard += 1;
            assert!(assign35610_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) && s.b[1816]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

    }

    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) && (!s.b[1816])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                }
            });
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 4.0);
            s.store_div_scaled_product_indices(334, 725, 726, 4.0, 770, 1.0);
            s.store_sub_ad_lhs(344, A::offset(s.ad_value(972), 4.0), 780);
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) {
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1808]) && (!s.b[1815])) {
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1808]) && (!s.b[1815])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1808]) {
            s.store_div(335, 349, 344);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1808]) {
            s.store_ad_value(336, {
                if (s.v[335] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(335), A::offset(s.ad_value(658), (-1.0)))
                }
            });
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1808]) {
            s.store_mul(341, 336, 335);
            s.store_offset(337, 341, 1.0);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1808]) {
            s.store_ad_value(338, {
                if (s.v[337] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)))
                }
            });
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1808]) {
            s.store_mul(340, 338, 337);
            s.store_div(1557, 349, 340);
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1808])) {
            s.copy_ad(1557, 349);
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_offset_mul_ad(338, A::sub(s.ad_value(1462), s.ad_value(1461)), s.ad_value(682), 1.0);
            s.store_neg(133, 1496);
            s.copy_ad(339, 133);
            s.store_div(337, 339, 338);
            s.copy_ad(251, 337);
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_ad_value(339, {
                if (s.v[251] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(251), (p.p376 - 1.0))
                }
            });
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_mul(342, 339, 251);
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 133, 343);
            s.store_add_scaled_product(335, A::div_scalar_offset_denominator(1.0, A::add_scaled_product(s.ad_value(966), 1.0, s.ad_value(968), s.ad_value(252), 1e-11), 1e-25, 1.0), 1.0, s.ad_value(977), s.ad_value(342), 1.0);
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_div(1555, 1557, 170);
            s.store_div_scaled_product_indices(335, 254, 1555, 1.0, 973, 1.0);
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_ad_value(336, {
                if (s.v[335] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(335), p.p378)
                }
            });
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_offset(337, 336, 1.0);
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_ad_value(338, {
                if (s.v[337] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(337), (1.0 / p.p378))
                }
            });
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_div(1506, 254, 338);
            s.store_mul3_affine_lhs(987, 1496, 1506, (-s.v[632]), 0.0, 1555);
            s.store_offset_mul_ad(338, A::sub(s.ad_value(1462), s.ad_value(1461)), s.ad_value(682), 1.0);
            s.store_neg(133, 1505);
            s.copy_ad(339, 133);
            s.store_div(337, 339, 338);
            s.copy_ad(251, 337);
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_ad_value(339, {
                if (s.v[251] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(251), (p.p376 - 1.0))
                }
            });
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_mul(342, 339, 251);
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 133, 343);
            s.store_add_scaled_product(335, A::div_scalar_offset_denominator(1.0, A::add_scaled_product(s.ad_value(967), 1.0, s.ad_value(969), s.ad_value(252), 1e-11), 1e-25, 1.0), 1.0, s.ad_value(977), s.ad_value(342), 1.0);
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_div(1555, 1557, 170);
            s.store_div_scaled_product_indices(335, 254, 1555, 1.0, 973, 1.0);
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_ad_value(336, {
                if (s.v[335] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(335), p.p378)
                }
            });
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_offset(337, 336, 1.0);
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_ad_value(338, {
                if (s.v[337] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(337), (1.0 / p.p378))
                }
            });
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_div(1507, 254, 338);
            s.store_mul3_affine_lhs(1554, 1505, 1507, (-s.v[632]), 0.0, 1555);
            s.store_div_scaled_inputs(115, s.ad_value(155), s.v[632], s.ad_value(170), 1.0);
            s.store_add_scaled_inputs3(135, A::mul3(s.ad_value(115), s.ad_value(248), s.ad_value(253)), 1.0, s.ad_value(987), 1.0, s.ad_value(1554), 1.0);
            s.store_mul3_lhs(986, 115, 248, 253);
            s.copy_ad(984, 253);
            s.copy_ad(790, 349);
        }

        s.b[1821] = (p.p283 != 0.0);
        s.v[1821] = if s.b[1821] { 1.0 } else { 0.0 };

        if ((s.b[1443] && s.b[1444]) && s.b[1821]) {
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 100.0));
            s.store_offset_ad(782, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0)), 1.0);
            s.store_offset_ad(783, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0)), (1.0 / 2.0));
            s.store_div_from_scalar(340, 0.01, 782);
            s.store_div_scaled_inputs(336, s.ad_value(783), (-2.0), A::square(s.ad_value(782)), 1.0);
            s.store_sub_from_scalar_ad(335, 1.1, A::add(s.ad_value(1461), s.ad_value(340)));
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.05) * 0.05));
            s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);
            s.store_scaled_add(336, 335, 782, 0.5);
        }

        s.b[1822] = (s.v[336] < 0.0);
        s.v[1822] = if s.b[1822] { 1.0 } else { 0.0 };

        if (((s.b[1443] && s.b[1444]) && s.b[1821]) && s.b[1822]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1821]) {
            s.store_offset(336, 336, 1e-25);
            s.store_scale(334, 154, s.v[672]);
            s.store_mul(337, 185, 334);
            s.store_powf(334, 336, p.p284);
            s.store_mul(343, 337, 334);
            s.store_offset_scaled(338, 1439, p.p285, 1.0);
            s.store_scalar(334, s.v[673]);
            s.store_add_scaled_inputs3(339, s.ad_value(1461), 1.0, s.ad_value(340), 1.0, s.ad_value(1438), -1.0);
            s.store_add_ad_rhs(338, 338, A::mul3(s.ad_value(1439), s.ad_value(334), s.ad_value(339)));
            s.store_mul(340, 343, 338);
            s.copy_ad(343, 340);
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1821])) {
            s.store_scalar(343, 0.0);
        }

        s.b[1823] = (p.p287 != 0.0);
        s.v[1823] = if s.b[1823] { 1.0 } else { 0.0 };

        if ((s.b[1443] && s.b[1444]) && s.b[1823]) {
            s.store_scale(335, 154, s.v[674]);
            s.store_mul(336, 185, 335);
            s.store_mul(342, 336, 1439);
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1823])) {
            s.store_scalar(342, 0.0);
        }

        s.b[1824] = ((s.v[343] + s.v[342]) > 0.0);
        s.v[1824] = if s.b[1824] { 1.0 } else { 0.0 };

        if ((s.b[1443] && s.b[1444]) && s.b[1824]) {
            s.store_mul_add_rhs(249, 94, 343, 342);
            s.store_add_ad_rhs(135, 135, A::mul3(s.ad_value(115), s.ad_value(249), s.ad_value(253)));
        }

        s.b[1825] = ((s.v[74] == 2.0) || (s.v[74] == 3.0));
        s.v[1825] = if s.b[1825] { 1.0 } else { 0.0 };

        s.b[1826] = (p.p296 > 0.0);
        s.v[1826] = if s.b[1826] { 1.0 } else { 0.0 };

        if (((s.b[1443] && s.b[1444]) && s.b[1825]) && s.b[1826]) {
            s.copy_ad(338, 647);
            s.store_scaled_offset(335, 796, (-p.p300), s.v[533]);
            s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));
            s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1825]) && s.b[1826]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1825]) && s.b[1826]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3(336, s.ad_value(338), 1.0, s.ad_value(781), 0.5, s.ad_value(782), 0.5);
            s.store_scale(337, 338, (p.p296 + 1.0));
            s.store_offset_sub(781, 337, 336, (-(0.01 * 0.01)));
            s.store_scale(782, 337, (4.0 * (0.01 * 0.01)));
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1825]) && s.b[1826]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1825]) && s.b[1826]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3(341, s.ad_value(337), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5));
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1825]) && (!s.b[1826])) {
            s.copy_ad(341, 647);
        }

        s.b[1827] = (s.v[793] >= 0.0);
        s.v[1827] = if s.b[1827] { 1.0 } else { 0.0 };

        if (((s.b[1443] && s.b[1444]) && s.b[1825]) && s.b[1827]) {
            s.copy_ad(369, 793);
        }

    }

    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1443] && s.b[1444]) && s.b[1825]) && (!s.b[1827])) {
            s.store_scalar(369, 0.0);
        }

        s.b[1828] = (s.v[369] < (20.0 * 1e-12));
        s.v[1828] = if s.b[1828] { 1.0 } else { 0.0 };

        if (((s.b[1443] && s.b[1444]) && s.b[1825]) && s.b[1828]) {
            s.store_scalar(378, (((((20.0 + 1.0)) as f64).powf((p.p297 - 1.0)) * ((20.0 + 1.0) - ((0.5 * p.p297) * 20.0))) * ((1e-12) as f64).powf(p.p297)));
            s.store_scalar(379, ((((0.5 * p.p297) * (((20.0 + 1.0)) as f64).powf((p.p297 - 1.0))) / 20.0) * ((1e-12) as f64).powf((p.p297 - 2.0))));
            s.store_add_ad_rhs(335, 378, A::mul3(s.ad_value(379), s.ad_value(369), s.ad_value(369)));
        }

        if (((s.b[1443] && s.b[1444]) && s.b[1825]) && (!s.b[1828])) {
            s.store_powf_ad(335, A::offset(s.ad_value(369), 1e-12), p.p297);
        }

        if ((s.b[1443] && s.b[1444]) && s.b[1825]) {
            s.store_powf_ad(343, A::offset(s.ad_value(369), 1e-12), p.p299);
            s.store_add_scaled_products_indices(368, 341, 335, 1.0 / (s.v[632]), 797, 343, (s.v[531] * 1.0 / (s.v[632])));
            s.store_mul(334, 368, 135);
            s.store_offset(335, 790, 1e-12);
            s.store_div_from_scalar(336, 1.0, 335);
            s.store_offset_mul(337, 334, 336, 1.0);
            s.store_div_from_scalar(338, 1.0, 337);
            s.store_mul(134, 135, 338);
        }

        if ((s.b[1443] && s.b[1444]) && (!s.b[1825])) {
            s.copy_ad(134, 135);
            s.store_scalar(368, 0.0);
        }

        if (s.b[1443] && s.b[1444]) {
            s.store_add_scaled_inputs4(131, s.ad_value(1477), (-0.5), s.ad_value(1478), (-0.5), s.ad_value(1498), (-0.5), s.ad_value(1500), (-0.5));
            s.store_scaled_add_ad_lhs(133, A::add(A::add_scaled_inputs4(s.ad_value(1538), 1.0, s.ad_value(1539), 1.0, s.ad_value(1517), 1.0, s.ad_value(1518), 1.0), s.ad_value(1497)), 1499, (-0.5));
            s.store_scalar(247, 0.5);
            s.store_scaled_add(978, 1538, 1539, (-0.5));
            s.store_neg(238, 1538);
            s.copy_ad(255, 1558);
        }

        s.b[1829] = ((s.v[238] < 1e-25) || (s.v[133] < 1e-25));
        s.v[1829] = if s.b[1829] { 1.0 } else { 0.0 };

        if ((s.b[1443] && s.b[1444]) && s.b[1829]) {
            s.store_scalar(78, 1.0);
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.copy_ad(1855, 960);
            s.store_scale(1905, 964, 1.6021918e-19);
            s.store_scale(1884, 964, (1.6021918e-19 * 1.034943e-10));
            s.store_scale(1904, 622, 1.6021918e-19);
            s.store_square(1903, 965);
            s.store_div_from_scalar(1908, (2.0 * 1.034943e-10), 1905);
            s.store_div_from_scalar(1909, (2.0 * 1.034943e-10), 1904);
            s.store_div(1902, 964, 622);
            s.store_div_from_scalar_offset_input(1901, 1.0, 1902, 1.0);
            s.store_div_ad_rhs(1906, 1884, A::square(s.ad_value(185)));
            s.store_div_from_scalar(1907, 2.0, 1906);
            s.store_scalar(1910, 4.0);
            s.store_scalar(1911, 0.1);
            s.store_scalar(1912, 0.1);
            s.store_offset(1913, 961, p.p407);
            s.store_scalar(1914, 3.0);
            s.store_scalar(1853, 0.0);
            s.store_scalar(1854, 0.0);
            s.store_scalar(1862, 0.0);
            s.store_scalar(1863, 0.0);
            s.store_scalar(1895, 0.0);
            s.store_scalar(1896, 0.0);
            s.store_scalar(1866, 0.0);
            s.store_scalar(1868, 0.0);
            s.store_scalar(1867, 0.0);
            s.store_scalar(1869, 0.0);
            s.store_scalar(1839, 0.0);
            s.store_scalar(1834, 0.0);
            s.copy_ad(1887, 1435);
            s.store_offset(85, 85, ((10.0 * 2.220446049250313e-16) * 100000000.0));
            s.store_div_scaled_product(962, s.ad_value(1908), s.ad_value(622), 1.0, A::add(s.ad_value(964), s.ad_value(622)), 1.0);
            s.store_sub(335, 1855, 1438);
        }

        s.b[1917] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
        s.v[1917] = if s.b[1917] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) {
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

        s.b[1918] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1918] = if s.b[1918] { 1.0 } else { 0.0 };

        s.b[1919] = (4.0 == 1.0);
        s.v[1919] = if s.b[1919] { 1.0 } else { 0.0 };

        if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) && s.b[1918]) && s.b[1919]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1920] = (4.0 == 2.0);
        s.v[1920] = if s.b[1920] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) && s.b[1918]) && (!s.b[1919])) && s.b[1920]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1921] = (4.0 == 4.0);
        s.v[1921] = if s.b[1921] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) && s.b[1918]) && (!s.b[1919])) && (!s.b[1920])) && s.b[1921]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1922] = (4.0 == 8.0);
        s.v[1922] = if s.b[1922] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) && s.b[1918]) && (!s.b[1919])) && (!s.b[1920])) && (!s.b[1921])) && s.b[1922]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) && s.b[1918]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign37610_loop_guard: usize = 0;
        while {
            let assign37610_cond_e43250: f64 = if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) && s.b[1918]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign37610_cond_e43250 != 0.0
        } {
            assign37610_loop_guard += 1;
            assert!(assign37610_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) && s.b[1918]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) && (!s.b[1918])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                }
            });
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_sub_from_scalar(336, 0.1, 780);
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) {
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1917])) {
            s.copy_ad(336, 335);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_mul(1838, 962, 336);
            s.store_sqrt(1836, 1838);
        }

        s.b[1923] = (p.p345 != 0.0);
        s.v[1923] = if s.b[1923] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1923]) {
            s.store_mul_sub_from_scalar_ad_rhs(335, 965, 1.0, A::scale(s.ad_value(790), p.p345));
            s.store_scale(336, 965, 0.001);
            s.store_add_scaled_inputs3(781, s.ad_value(335), 1.0, s.ad_value(965), (-0.1), s.ad_value(336), -1.0);
            s.store_scaled_mul(782, 965, 336, (0.1 * 4.0));
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1923]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1923]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3(335, s.ad_value(965), 0.1, s.ad_value(781), 0.5, s.ad_value(782), 0.5);
            s.store_add_scaled_inputs3(781, s.ad_value(965), 2.0, s.ad_value(335), (-1.0), s.ad_value(336), -1.0);
            s.store_scaled_mul(782, 965, 336, (2.0 * 4.0));
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1923]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1923]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3(965, s.ad_value(965), 2.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5));
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_scalar(1851, 0.0);
        }

        s.b[1924] = (s.v[1836] > s.v[965]);
        s.v[1924] = if s.b[1924] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1924]) {
            s.copy_ad(1835, 965);
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1924])) {
            s.copy_ad(1835, 1836);
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_offset_sub_from_scalar_ad(781, 0.3, s.ad_value(85), (-0.01));
            s.store_scalar(782, ((4.0 * 0.3) * 0.01));
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_sub_from_scalar_ad(1860, 0.3, A::add_scaled_inputs(s.ad_value(781), 0.5, s.ad_value(782), 0.5));
        }

    }

    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
    ) {
        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_add_scaled_inputs3_offset(781, s.ad_value(1860), 1.0, s.ad_value(1887), -1.0, s.ad_value(1855), 1.0, (-0.01));
            s.store_scaled_sub(782, 1887, 1855, (4.0 * 0.01));
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4(1860, s.ad_value(1887), 1.0, s.ad_value(1855), (-1.0), s.ad_value(781), 0.5, s.ad_value(782), 0.5);
            s.store_div_scaled_product(1888, s.ad_value(1855), s.ad_value(622), -1.0, A::add(s.ad_value(622), s.ad_value(964)), 1.0);
            s.store_offset_sub(1834, 965, 1835, 1e-15);
            s.store_scalar(79, 0.0);
            s.store_scalar(1850, 0.2);
            s.copy_ad(1853, 1860);
            s.copy_ad(1856, 1851);
            s.copy_ad(1858, 1888);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign38110_loop_guard: usize = 0;
        while {
            let assign38110_cond_e43963: f64 = if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign38110_cond_e43963 != 0.0
        } {
            assign38110_loop_guard += 1;
            assert!(assign38110_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
                s.store_mul_sub_ad_rhs(1858, 1901, A::add_scaled_product(s.ad_value(1887), 1.0, s.ad_value(1902), s.ad_value(1856), 1.0), s.ad_value(1855));
                s.store_mul(1842, 1901, 1902);
                s.store_sub(335, 1856, 1858);
            }
            s.b[1925] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1925] = if s.b[1925] { 1.0 } else { 0.0 };
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) {
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
            s.b[1926] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1926] = if s.b[1926] { 1.0 } else { 0.0 };
            s.b[1927] = (2.0 == 1.0);
            s.v[1927] = if s.b[1927] { 1.0 } else { 0.0 };
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) && s.b[1926]) && s.b[1927]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1928] = (2.0 == 2.0);
            s.v[1928] = if s.b[1928] { 1.0 } else { 0.0 };
            if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) && s.b[1926]) && (!s.b[1927])) && s.b[1928]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1929] = (2.0 == 4.0);
            s.v[1929] = if s.b[1929] { 1.0 } else { 0.0 };
            if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) && s.b[1926]) && (!s.b[1927])) && (!s.b[1928])) && s.b[1929]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1930] = (2.0 == 8.0);
            s.v[1930] = if s.b[1930] { 1.0 } else { 0.0 };
            if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) && s.b[1926]) && (!s.b[1927])) && (!s.b[1928])) && (!s.b[1929])) && s.b[1930]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) && s.b[1926]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign38110_body29_loop_guard: usize = 0;
            while {
                let assign38110_body29_cond_e44326: f64 = if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) && s.b[1926]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38110_body29_cond_e44326 != 0.0
            } {
                assign38110_body29_loop_guard += 1;
                assert!(assign38110_body29_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) && s.b[1926]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) && (!s.b[1926])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) {
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1925])) {
                s.copy_ad(336, 335);
                s.store_scalar(334, 1.0);
            }
            if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
                s.store_sqrt_mul(1830, 1908, 336);
            }
            s.b[1931] = ((s.v[1830] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.v[1931] = if s.b[1931] { 1.0 } else { 0.0 };
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) {
                s.store_offset_sub(781, 1830, 965, 1e-8);
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
            s.b[1932] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1932] = if s.b[1932] { 1.0 } else { 0.0 };
            s.b[1933] = (2.0 == 1.0);
            s.v[1933] = if s.b[1933] { 1.0 } else { 0.0 };
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) && s.b[1932]) && s.b[1933]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1934] = (2.0 == 2.0);
            s.v[1934] = if s.b[1934] { 1.0 } else { 0.0 };
            if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) && s.b[1932]) && (!s.b[1933])) && s.b[1934]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1935] = (2.0 == 4.0);
            s.v[1935] = if s.b[1935] { 1.0 } else { 0.0 };
            if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) && s.b[1932]) && (!s.b[1933])) && (!s.b[1934])) && s.b[1935]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1936] = (2.0 == 8.0);
            s.v[1936] = if s.b[1936] { 1.0 } else { 0.0 };
            if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) && s.b[1932]) && (!s.b[1933])) && (!s.b[1934])) && (!s.b[1935])) && s.b[1936]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) && s.b[1932]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign38110_body65_loop_guard: usize = 0;
            while {
                let assign38110_body65_cond_e44811: f64 = if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) && s.b[1932]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38110_body65_cond_e44811 != 0.0
            } {
                assign38110_body65_loop_guard += 1;
                assert!(assign38110_body65_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) && s.b[1932]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) && (!s.b[1932])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
                s.store_add_ad_lhs(1830, A::offset(s.ad_value(965), (-1e-8)), 780);
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) {
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1931])) {
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1931])) {
                s.store_scalar(337, 1.0);
            }
            if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
                s.store_mul(1866, 1830, 1905);
                s.store_mul_ad_product_lhs(1844, A::div_from_scalar(1.034943e-10, s.ad_value(1830)), s.ad_value(334), 337);
                s.store_mul_ad_product_lhs(1846, A::div_from_scalar((-1.034943e-10), s.ad_value(1830)), s.ad_value(334), 337);
            }
            s.b[1937] = (p.p49 == 0.0);
            s.v[1937] = if s.b[1937] { 1.0 } else { 0.0 };
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1937]) {
                s.store_add_ad_lhs(1839, A::div_scaled_inputs_product(s.ad_value(1903), 1.0, s.ad_value(1838), 1.0, s.ad_value(965), s.ad_value(1835), (-2.0), s.ad_value(1908), 1.0), 1853);
                s.store_scalar(1840, 1.0);
                s.store_scalar(1841, 0.0);
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1937])) {
                s.store_add_ad_rhs(1839, 1853, A::div_scaled_add_product(s.ad_value(1903), 1.0, s.ad_value(1830), A::sub_scaled_inputs(s.ad_value(1830), 1.0, s.ad_value(965), 2.0), 1.0, s.ad_value(1908), 1.0));
                s.store_scalar(1840, 1.0);
                s.store_ad_value(1841, A::mul_sub_from_scalar_rhs(A::add_scaled_product(s.ad_value(334), 1.0, A::div(s.ad_value(965), s.ad_value(1830)), s.ad_value(334), (-1.0)), 1.0, s.ad_value(1842)));
            }
            s.b[1938] = ((s.v[1839] > (s.v[1851] - s.v[1850])) && (s.v[1850] >= 0.0));
            s.v[1938] = if s.b[1938] { 1.0 } else { 0.0 };
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) {
                s.store_add_scaled_inputs3(781, s.ad_value(1839), 1.0, s.ad_value(1851), (-1.0), s.ad_value(1850), 1.0);
                s.store_square(722, 781);
                s.store_square(723, 1850);
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
            s.b[1939] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.v[1939] = if s.b[1939] { 1.0 } else { 0.0 };
            s.b[1940] = (4.0 == 1.0);
            s.v[1940] = if s.b[1940] { 1.0 } else { 0.0 };
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) && s.b[1939]) && s.b[1940]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1941] = (4.0 == 2.0);
            s.v[1941] = if s.b[1941] { 1.0 } else { 0.0 };
            if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) && s.b[1939]) && (!s.b[1940])) && s.b[1941]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1942] = (4.0 == 4.0);
            s.v[1942] = if s.b[1942] { 1.0 } else { 0.0 };
            if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) && s.b[1939]) && (!s.b[1940])) && (!s.b[1941])) && s.b[1942]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1943] = (4.0 == 8.0);
            s.v[1943] = if s.b[1943] { 1.0 } else { 0.0 };
            if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) && s.b[1939]) && (!s.b[1940])) && (!s.b[1941])) && (!s.b[1942])) && s.b[1943]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) && s.b[1939]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign38110_body114_loop_guard: usize = 0;
            while {
                let assign38110_body114_cond_e45484: f64 = if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) && s.b[1939]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38110_body114_cond_e45484 != 0.0
            } {
                assign38110_body114_loop_guard += 1;
                assert!(assign38110_body114_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) && s.b[1939]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) && (!s.b[1939])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                    }
                });
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_mul3_lhs(780, 781, 1850, 726);
                s.store_div_scaled_product3(334, s.ad_value(1850), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0);
                s.store_add_scaled_inputs3(1839, s.ad_value(1851), 1.0, s.ad_value(1850), (-1.0), s.ad_value(780), 1.0);
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) {
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1938])) {
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1938])) {
                s.store_scalar(334, 1.0);
            }
            if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
                s.store_mul(1840, 1840, 334);
                s.store_mul(1841, 1841, 334);
                s.store_add_scaled_inputs3(335, s.ad_value(1858), 1.0, s.ad_value(1887), (-1.0), s.ad_value(1855), 1.0);
            }
            s.b[1944] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1944] = if s.b[1944] { 1.0 } else { 0.0 };
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) {
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
            s.b[1945] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1945] = if s.b[1945] { 1.0 } else { 0.0 };
            s.b[1946] = (2.0 == 1.0);
            s.v[1946] = if s.b[1946] { 1.0 } else { 0.0 };
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) && s.b[1945]) && s.b[1946]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1947] = (2.0 == 2.0);
            s.v[1947] = if s.b[1947] { 1.0 } else { 0.0 };
            if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) && s.b[1945]) && (!s.b[1946])) && s.b[1947]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1948] = (2.0 == 4.0);
            s.v[1948] = if s.b[1948] { 1.0 } else { 0.0 };
            if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) && s.b[1945]) && (!s.b[1946])) && (!s.b[1947])) && s.b[1948]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1949] = (2.0 == 8.0);
            s.v[1949] = if s.b[1949] { 1.0 } else { 0.0 };
            if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) && s.b[1945]) && (!s.b[1946])) && (!s.b[1947])) && (!s.b[1948])) && s.b[1949]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) && s.b[1945]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign38110_body152_loop_guard: usize = 0;
            while {
                let assign38110_body152_cond_e45992: f64 = if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) && s.b[1945]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38110_body152_cond_e45992 != 0.0
            } {
                assign38110_body152_loop_guard += 1;
                assert!(assign38110_body152_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) && s.b[1945]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) && (!s.b[1945])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(337, 725, 726, 0.1, 770, 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) {
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1944])) {
                s.copy_ad(336, 335);
                s.store_scalar(337, 1.0);
            }
            if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
                s.store_sqrt_mul(1832, 1909, 336);
                s.store_mul_neg_lhs(1867, 1832, 1904);
                s.store_mul_div_from_scalar_lhs(1848, (-1.034943e-10), 1832, 337);
                s.store_mul_sub_rhs(335, 154, 1853, 1856);
                s.store_exp(336, 335);
            }
            s.b[1950] = (s.v[1853] >= s.v[1856]);
            s.v[1950] = if s.b[1950] { 1.0 } else { 0.0 };
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1950]) {
                s.store_mul_scaled_ad_rhs(1862, 209, -1.0, A::sqrt(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15)));
                s.store_mul_ad(1897, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(1862), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
                s.store_neg(1899, 1897);
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1950])) {
                s.store_exp_ad(337, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1853), s.ad_value(1887))));
                s.store_exp_ad(338, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1856), s.ad_value(1887))));
                s.store_mul_sqrt_ad_rhs(1862, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1862, 1.0);
                s.store_mul_add_ad_rhs(1897, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
                s.store_mul_add_ad_rhs(1899, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (s.v[79] != 0.0)) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (s.v[79] == 0.0)) {
                s.store_add_scaled_inputs3(1870, A::add_scaled_product(s.ad_value(1862), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1853)), 1.0), 1.0, s.ad_value(1866), 1.0, s.ad_value(1867), 1.0);
                s.store_sub(1871, 1897, 185);
                s.store_add_ad_rhs(1872, 1899, A::add_scaled_value_products(s.ad_value(1844), 1.0, s.ad_value(1846), s.ad_value(1842), 1.0, s.ad_value(1848), s.ad_value(1842), 1.0));
                s.store_sub(1873, 1856, 1839);
                s.store_neg(1874, 1840);
                s.store_sub_from_scalar(1875, 1.0, 1841);
                s.store_add_scaled_products_indices(1876, 1871, 1875, 1.0, 1872, 1874, (-1.0));
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (s.v[79] == 0.0)) {
                s.store_ad_value(1877, {
                    if (s.v[1876] > 0.0) {
                        A::div_scalar_offset_denominator(1.0, s.ad_value(1876), 1e-25, 1.0)
                    } else {
                        A::div_scalar_offset_denominator(1.0, s.ad_value(1876), (-1e-25), 1.0)
                    }
                });
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (s.v[79] == 0.0)) {
                s.copy_ad(1878, 1875);
                s.store_neg(1879, 1872);
                s.store_neg(1880, 1874);
                s.copy_ad(1881, 1871);
                s.store_mul_scaled_ad_rhs(1882, 1877, -1.0, A::add_scaled_products(s.ad_value(1878), s.ad_value(1870), 1.0, s.ad_value(1879), s.ad_value(1873), 1.0));
                s.store_mul_scaled_ad_rhs(1883, 1877, -1.0, A::add_scaled_products(s.ad_value(1880), s.ad_value(1870), 1.0, s.ad_value(1881), s.ad_value(1873), 1.0));
                s.store_abs(335, 1882);
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (s.v[79] == 0.0)) {
                s.store_ad_value(335, {
                    if (s.v[335] < ((s.v[1883]) as f64).abs()) {
                        A::abs(s.ad_value(1883))
                    } else {
                        s.ad_value(335)
                    }
                });
            }
            s.b[1951] = (s.v[335] > 0.1);
            s.v[1951] = if s.b[1951] { 1.0 } else { 0.0 };
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (s.v[79] == 0.0)) && s.b[1951]) {
                s.store_mul_div_from_scalar_rhs(1882, 1882, 0.1, 335);
                s.store_mul_div_from_scalar_rhs(1883, 1883, 0.1, 335);
            }
            s.b[1952] = (s.v[335] < 1e-12);
            s.v[1952] = if s.b[1952] { 1.0 } else { 0.0 };
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (s.v[79] == 0.0)) && s.b[1952]) {
                s.store_scalar(79, 1.0);
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (s.v[79] == 0.0)) {
                s.store_add(1853, 1853, 1882);
                s.store_add(1856, 1856, 1883);
            }
            if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
                s.store_offset(97, 97, 1.0);
            }
        }

    }
}
